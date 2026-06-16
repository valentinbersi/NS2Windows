use btleplug::api::{Characteristic, Peripheral as PeripheralApi};
use btleplug::platform::Peripheral;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ConnectedDevice {
    pub peripheral: Peripheral,
    pub input_char: Arc<Characteristic>,
    pub write_char: Arc<Characteristic>,
}

impl ConnectedDevice {
    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.peripheral.disconnect().await
    }
}
