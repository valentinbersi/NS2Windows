use crate::connection::connected_device::ConnectedDevice;
use crate::data::ns_controller_kind::NsControllerKind;
use btleplug::api::Characteristic;
use btleplug::platform::Peripheral;
use std::sync::Arc;
use tauri::async_runtime::JoinHandle;

#[derive(Debug)]
pub struct NsController {
    kind: NsControllerKind,
    device: ConnectedDevice,
    input_poller: JoinHandle<()>,
    input_informer: JoinHandle<()>,
}

impl NsController {
    pub fn new(
        kind: NsControllerKind,
        device: ConnectedDevice,
        input_poller: JoinHandle<()>,
        input_informer: JoinHandle<()>,
    ) -> Self {
        Self {
            kind,
            device,
            input_poller,
            input_informer,
        }
    }

    pub fn kind(&self) -> NsControllerKind {
        self.kind
    }

    pub fn peripheral(&self) -> Peripheral {
        self.device.peripheral.clone()
    }

    pub fn input(&self) -> Arc<Characteristic> {
        self.device.input_char.clone()
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.input_informer.abort();
        self.input_poller.abort();
        self.device.disconnect().await
    }
}
