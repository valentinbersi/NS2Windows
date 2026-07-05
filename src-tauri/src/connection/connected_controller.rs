use crate::data::ns_controller_kind::NsControllerKind;
use btleplug::api::{Characteristic, Peripheral as PeripheralApi, WriteType};
use btleplug::platform::Peripheral;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ConnectedController {
    controller: Peripheral,
    input: Characteristic,
    output: Characteristic,
    kind: NsControllerKind,
}

impl ConnectedController {
    pub fn new(
        controller: Peripheral,
        input: Characteristic,
        output: Characteristic,
        kind: NsControllerKind,
    ) -> Self {
        Self {
            controller,
            input,
            output,
            kind,
        }
    }

    pub async fn suscribe_input(&self) -> btleplug::Result<()> {
        self.controller.subscribe(&self.input).await
    }

    pub async fn write(&self, cmd: &[u8], write_type: WriteType) -> btleplug::Result<()> {
        self.controller.write(&self.output, cmd, write_type).await
    }

    pub fn controller(&self) -> Peripheral {
        self.controller.clone()
    }

    pub fn input_uuid(&self) -> Uuid {
        self.input.uuid
    }

    pub fn kind(&self) -> NsControllerKind {
        self.kind
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.controller.disconnect().await
    }
}
