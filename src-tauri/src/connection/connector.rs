use crate::connection::connected_controller::ConnectedDevice;
use btleplug::api::{Central, CentralEvent, ConnectionParameterPreset, Peripheral, ScanFilter};
use btleplug::platform::Adapter;
use futures::StreamExt;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::timeout;
use uuid::Uuid;

const JOYCON_MANUFACTURER_ID: u16 = 1363; // Nintendo
const JOYCON_MANUFACTURER_PREFIX: [u8; 4] = [0x01, 0x00, 0x03, 0x7E];

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

impl BluetoothConnector {
    pub fn new(adapter: Adapter) -> Self {
        Self { adapter }
    }

    pub async fn wait_for_controller(&self) -> Result<ConnectedDevice, ConnectorError> {
        // Start the scanner
        self.adapter.start_scan(ScanFilter::default()).await?;

        // Get the live stream of Bluetooth events from the OS
        let mut events = self.adapter.events().await?;

        let scan_result = timeout(Duration::from_secs(30), async {
            // 1. Quick Check: Is it already cached from a previous session?
            for peripheral in self.adapter.peripherals().await? {
                if peripheral.is_connected().await.unwrap_or(false) {
                    continue;
                }

                if let Some(props) = peripheral.properties().await?
                    && let Some(data) = props.manufacturer_data.get(&JOYCON_MANUFACTURER_ID)
                    && data.starts_with(&JOYCON_MANUFACTURER_PREFIX)
                {
                    self.adapter.stop_scan().await?;
                    return Ok::<_, btleplug::Error>(peripheral);
                }
            }

            // 2. Event Loop: Wait for new beacons to arrive in real-time
            while let Some(event) = events.next().await {
                // We only care about events related to device discovery or data updates
                let id = match event {
                    CentralEvent::DeviceDiscovered(id) => id,
                    CentralEvent::DeviceUpdated(id) => id,
                    CentralEvent::ManufacturerDataAdvertisement { id, .. } => id,
                    _ => continue, // Ignore disconnects, service data updates, etc.
                };

                // Fetch the specific peripheral that triggered the event
                let peripheral = self.adapter.peripheral(&id).await?;

                if peripheral.is_connected().await.unwrap_or(false) {
                    continue;
                }

                if let Some(props) = peripheral.properties().await?
                    && let Some(data) = props.manufacturer_data.get(&JOYCON_MANUFACTURER_ID)
                    && data.starts_with(&JOYCON_MANUFACTURER_PREFIX)
                {
                    self.adapter.stop_scan().await?;
                    return Ok(peripheral);
                }
            }

            // This is just a fallback in case the stream dies
            Err(btleplug::Error::Other(
                "Event stream ended unexpectedly".into(),
            ))
        })
        .await;

        self.adapter.stop_scan().await?;
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Match the result exactly as before
        let device = match scan_result {
            Ok(Ok(peripheral)) => peripheral,
            _ => return Err(ConnectorError::TimeoutError),
        };

        device.connect().await?;
        device.discover_services().await?;

        let mut input_char = None;
        let mut write_char = None;

        let input_report = Uuid::from_str("ab7de9be-89fe-49ad-828f-118f09df7fd2").unwrap();
        let write_command = Uuid::from_str("649d4ac9-8eb7-4e6c-af44-1ea54fe5f005").unwrap();

        for characteristic in device.characteristics() {
            if characteristic.uuid == input_report {
                input_char = Some(characteristic);
                continue;
            }

            if characteristic.uuid == write_command {
                write_char = Some(characteristic)
            }
        }

        let input_char = input_char.ok_or_else(|| ConnectorError::ConnectionError)?;
        let write_char = write_char.ok_or_else(|| ConnectorError::ConnectionError)?;

        device
            .request_connection_parameters(ConnectionParameterPreset::ThroughputOptimized)
            .await?;

        Ok(ConnectedDevice {
            peripheral: device,
            input_char,
            write_char,
        })
    }
}
