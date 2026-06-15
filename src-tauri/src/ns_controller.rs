use crate::connection::connected_device::ConnectedDevice;
use crate::data::ns_controller_kind::NsControllerKind;
use tauri::async_runtime::JoinHandle;

#[derive(Debug)]
pub struct NsController {
    kind: NsControllerKind,
    device: ConnectedDevice,
    input_informer: JoinHandle<()>,
}

impl NsController {
    pub fn new(
        kind: NsControllerKind,
        device: ConnectedDevice,
        input_informer: JoinHandle<()>,
    ) -> Self {
        Self {
            kind,
            device,
            input_informer,
        }
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.device.disconnect().await
    }
}
