use crate::connection::connected_device::ConnectedDevice;
use crate::data::ns_controller_kind::NsControllerKind;
use btleplug::api::{Peripheral as PeripheralApi, ValueNotification};
use futures::Stream;
use std::pin::Pin;
use tauri::async_runtime::JoinHandle;
use uuid::Uuid;

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

    pub async fn notifications(
        &self,
    ) -> btleplug::Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>> {
        self.device.peripheral.notifications().await
    }

    pub fn input_uuid(&self) -> Uuid {
        self.device.input_char.uuid
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.input_informer.abort();
        self.input_poller.abort();
        self.device.disconnect().await
    }
}
