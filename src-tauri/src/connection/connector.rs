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
use std::time::Duration;
use tokio::time::error::Elapsed;
use tokio::time::timeout;
use uuid::{Uuid, uuid};

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

struct Characteristics {
    input: Characteristic,
    output: Characteristic,
    rumble_output: Characteristic,
    combined_output: Characteristic,
}

impl BluetoothConnector {
    pub fn new(adapter: Adapter) -> Self {
        Self { adapter }
    }

    pub fn adapter(&self) -> Adapter {
        self.adapter.clone()
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

    const OUTPUT_UUID: Uuid = uuid!("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005");

    const COMMON_INPUT_UUID: Uuid = uuid!("ab7de9be-89fe-49ad-828f-118f09df7fd2");
    const LEFT_JOY_CON_INPUT_UUID: Uuid = uuid!("cc1bbbb5-7354-4d32-a716-a81cb241a32a");
    const RIGHT_JOY_CON_INPUT_UUID: Uuid = uuid!("d5a9e01e-2ffc-4cca-b20c-8b67142bf442");
    const PRO_CONTROLLER_INPUT_UUID: Uuid = uuid!("7492866c-ec3e-4619-8258-32755ffcc0f8");
    const NSO_GC_CONTROLLER_INPUT_UUID: Uuid = uuid!("8261cba1-9435-420c-84d6-f0c75a2c8e4d");

    const LEFT_JOY_CON_RUMBLE_OUTPUT_UUID: Uuid = uuid!("289326cb-a471-485d-a8f4-240c14f18241");
    const RIGHT_JOY_CON_RUMBLE_OUTPUT_UUID: Uuid = uuid!("fa19b0fb-cd1f-46a7-84a1-bbb09e00c149");
    const PRO_CONTROLLER_RUMBLE_OUTPUT_UUID: Uuid = uuid!("cc483f51-9258-427d-a939-630c31f72b05");
    const NSO_GC_CONTROLLER_RUMBLE_OUTPUT_UUID: Uuid =
        uuid!("3f8fb670-ab25-45bf-b540-38c72834d064");

    const LEFT_JOY_CON_COMBINED_OUTPUT_UUID: Uuid = uuid!("ce49a830-dced-48ae-931e-c8cf88aadbea");
    const RIGHT_JOY_CON_COMBINED_OUTPUT_UUID: Uuid = uuid!("65a724b3-f1e7-4a61-8078-a342376b27ff");
    const PRO_CONTROLLER_COMBINED_OUTPUT_UUID: Uuid = uuid!("3dacbc7e-6955-40b5-8eaf-6f9809e8b379");
    const NSO_GC_CONTROLLER_COMBINED_OUTPUT_UUID: Uuid =
        uuid!("af95885e-44b3-4a24-9cf0-483cc129469a");

    fn discover_characteristics(
        &self,
        controller: &Peripheral,
    ) -> Result<(NsControllerKind, Characteristics), ConnectorError> {
        let mut input = None;
        let mut kind = None;
        let mut output = None;
        let mut rumble_output = None;
        let mut combined_output = None;

        for characteristic in controller.characteristics() {
            if input.is_some()
                && kind.is_some()
                && output.is_some()
                && rumble_output.is_some()
                && combined_output.is_some()
            {
                break;
            }

            match characteristic.uuid {
                Self::OUTPUT_UUID => output = Some(characteristic),

                Self::COMMON_INPUT_UUID => input = Some(characteristic),

                Self::LEFT_JOY_CON_INPUT_UUID => kind = Some(NsControllerKind::LeftJoyCon),
                Self::RIGHT_JOY_CON_INPUT_UUID => kind = Some(NsControllerKind::RightJoyCon),
                Self::PRO_CONTROLLER_INPUT_UUID => kind = Some(NsControllerKind::ProController),
                Self::NSO_GC_CONTROLLER_INPUT_UUID => {
                    kind = Some(NsControllerKind::NsoGcController)
                }

                Self::LEFT_JOY_CON_RUMBLE_OUTPUT_UUID
                | Self::RIGHT_JOY_CON_RUMBLE_OUTPUT_UUID
                | Self::PRO_CONTROLLER_RUMBLE_OUTPUT_UUID
                | Self::NSO_GC_CONTROLLER_RUMBLE_OUTPUT_UUID => {
                    rumble_output = Some(characteristic)
                }

                Self::LEFT_JOY_CON_COMBINED_OUTPUT_UUID
                | Self::RIGHT_JOY_CON_COMBINED_OUTPUT_UUID
                | Self::PRO_CONTROLLER_COMBINED_OUTPUT_UUID
                | Self::NSO_GC_CONTROLLER_COMBINED_OUTPUT_UUID => {
                    combined_output = Some(characteristic)
                }

                _ => {}
            }
        }

        let input = input.ok_or_else(|| ConnectorError::ConnectionError)?;
        let controller_kind = kind.ok_or_else(|| ConnectorError::ConnectionError)?;
        let output = output.ok_or_else(|| ConnectorError::ConnectionError)?;
        let rumble_output = rumble_output.ok_or_else(|| ConnectorError::ConnectionError)?;
        let combined_output = combined_output.ok_or_else(|| ConnectorError::ConnectionError)?;

        Ok((
            controller_kind,
            Characteristics {
                input,
                output,
                rumble_output,
                combined_output,
            },
        ))
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
            characteristics.rumble_output,
            characteristics.combined_output,
            controller_kind,
        );

        Ok(connected_controller)
    }
}
