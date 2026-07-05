use crate::connection::connected_controller::ConnectedController;
use crate::data::ns_controller_kind::NsControllerKind;
use btleplug::api::{
    Central, CentralEvent, Characteristic, ConnectionParameterPreset, Peripheral as PeripheralApi,
    ScanFilter,
};
use btleplug::platform::{Adapter, Peripheral};
use futures::StreamExt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use uuid::Uuid;

const JOY_CON_MANUFACTURER_ID: u16 = 1363; // Nintendo
const JOY_CON_MANUFACTURER_PREFIX: [u8; 4] = [0x01, 0x00, 0x03, 0x7E];

#[derive(Clone, Debug)]
pub struct BluetoothConnector {
    adapter: Adapter,
}

#[derive(Debug)]
pub enum ConnectorError {
    BTLEPlugError(btleplug::Error),
    TimeoutError,
    ConnectionError,
}

impl Display for ConnectorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectorError::BTLEPlugError(error) => write!(f, "{error}"),
            ConnectorError::TimeoutError => write!(f, "Timed out"),
            ConnectorError::ConnectionError => write!(f, "Connection Error"),
        }
    }
}

impl Error for ConnectorError {}

impl From<btleplug::Error> for ConnectorError {
    fn from(value: btleplug::Error) -> Self {
        Self::BTLEPlugError(value)
    }
}

impl From<Elapsed> for ConnectorError {
    fn from(_: Elapsed) -> Self {
        Self::TimeoutError
    }
}

const COMMON_INPUT_UUID: &str = "ab7de9be-89fe-49ad-828f-118f09df7fd2";
const LEFT_JOY_CON_INPUT_UUID: &str = "cc1bbbb5-7354-4d32-a716-a81cb241a32a";
const RIGHT_JOY_CON_INPUT_UUID: &str = "d5a9e01e-2ffc-4cca-b20c-8b67142bf442";
const PRO_CONTROLLER_INPUT_UUID: &str = "7492866c-ec3e-4619-8258-32755ffcc0f8";
const NSO_GC_CONTROLLER_INPUT_UUID: &str = "8261cba1-9435-420c-84d6-f0c75a2c8e4d";

struct Characteristics {
    input: Characteristic,
    output: Characteristic,
}

impl BluetoothConnector {
    pub fn new(adapter: Adapter) -> Self {
        Self { adapter }
    }

    async fn scan_devices(&self) -> Result<Peripheral, ConnectorError> {
        let mut events = self.adapter.events().await?;

        timeout(Duration::from_secs(30), async {
            for peripheral in self.adapter.peripherals().await? {
                if peripheral.is_connected().await.unwrap_or(false) {
                    continue;
                }

                if let Some(props) = peripheral.properties().await?
                    && let Some(data) = props.manufacturer_data.get(&JOY_CON_MANUFACTURER_ID)
                    && data.starts_with(&JOY_CON_MANUFACTURER_PREFIX)
                {
                    self.adapter.stop_scan().await?;
                    return Ok::<_, btleplug::Error>(peripheral);
                }
            }

            while let Some(event) = events.next().await {
                let id = match event {
                    CentralEvent::DeviceDiscovered(id) => id,
                    CentralEvent::DeviceUpdated(id) => id,
                    CentralEvent::ManufacturerDataAdvertisement { id, .. } => id,
                    _ => continue,
                };

                let peripheral = self.adapter.peripheral(&id).await?;

                if peripheral.is_connected().await.unwrap_or(false) {
                    continue;
                }

                if let Some(props) = peripheral.properties().await?
                    && let Some(data) = props.manufacturer_data.get(&JOY_CON_MANUFACTURER_ID)
                    && data.starts_with(&JOY_CON_MANUFACTURER_PREFIX)
                {
                    self.adapter.stop_scan().await?;
                    return Ok(peripheral);
                }
            }

            Err(btleplug::Error::Other(
                "Event stream ended unexpectedly".into(),
            ))
        })
        .await
        .map_err(Into::into)
        .and_then(|value| value.map_err(Into::into))
    }

    fn discover_characteristics(
        &self,
        controller: &Peripheral,
    ) -> Result<(NsControllerKind, Characteristics), ConnectorError> {
        let mut input = None;
        let mut controller_kind = None;
        let mut output = None;

        let common_input_uuid = Uuid::from_str(COMMON_INPUT_UUID).unwrap();
        let left_joy_con_input_uuid = Uuid::from_str(LEFT_JOY_CON_INPUT_UUID).unwrap();
        let right_joy_con_input_uuid = Uuid::from_str(RIGHT_JOY_CON_INPUT_UUID).unwrap();
        let pro_controller_input_uuid = Uuid::from_str(PRO_CONTROLLER_INPUT_UUID).unwrap();
        let nso_gc_controller_input_uuid = Uuid::from_str(NSO_GC_CONTROLLER_INPUT_UUID).unwrap();
        let output_uuid = Uuid::from_str("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005").unwrap();

        for characteristic in controller.characteristics() {
            if input.is_some() && controller_kind.is_some() && output.is_some() {
                break;
            }

            let uuid = characteristic.uuid;

            // Get the common input characteristic
            if uuid == common_input_uuid {
                input = Some(characteristic);
                continue;
            }

            // Get the output characteristic
            if uuid == output_uuid {
                output = Some(characteristic);
                continue;
            }

            // Get the left joy con unique characteristic
            if uuid == left_joy_con_input_uuid {
                controller_kind = Some(NsControllerKind::LeftJoyCon);
                continue;
            }

            // Get the right joy con unique characteristic
            if uuid == right_joy_con_input_uuid {
                controller_kind = Some(NsControllerKind::RightJoyCon);
                continue;
            }

            // Get the pro controller unique characteristic
            if uuid == pro_controller_input_uuid {
                controller_kind = Some(NsControllerKind::ProController);
                continue;
            }

            // Get the nso gc controller unique characteristic
            if uuid == nso_gc_controller_input_uuid {
                controller_kind = Some(NsControllerKind::NsoGcController);
            }
        }

        let input = input.ok_or_else(|| ConnectorError::ConnectionError)?;
        let controller_kind = controller_kind.ok_or_else(|| ConnectorError::ConnectionError)?;
        let output = output.ok_or_else(|| ConnectorError::ConnectionError)?;

        Ok((controller_kind, Characteristics { input, output }))
    }

    pub async fn wait_for_controller(&self) -> Result<ConnectedController, ConnectorError> {
        self.adapter.start_scan(ScanFilter::default()).await?;

        let scan_result = self.scan_devices().await;

        self.adapter.stop_scan().await?;
        tokio::time::sleep(Duration::from_millis(200)).await;

        let controller = scan_result?;

        controller.connect().await?;
        controller.discover_services().await?;

        let (controller_kind, characteristics) = self.discover_characteristics(&controller)?;

        controller
            .request_connection_parameters(ConnectionParameterPreset::ThroughputOptimized)
            .await?;

        let connected_controller = ConnectedController::new(
            controller,
            characteristics.input,
            characteristics.output,
            controller_kind,
        );

        Ok(connected_controller)
    }
}
