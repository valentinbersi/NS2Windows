use tokio::sync::{Mutex, Notify};
use windows::Devices::Bluetooth::BluetoothLEDevice;

#[derive(Debug, Default)]
pub struct ConnectionState {
    pub mtx: Mutex<Option<windows::core::Result<BluetoothLEDevice>>>,
    pub cv: Notify,
}
