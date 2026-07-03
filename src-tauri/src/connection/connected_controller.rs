use crate::data::ns_controller_kind::NsControllerKind;
use btleplug::api::{Characteristic, Peripheral as PeripheralApi, WriteType};
use btleplug::platform::Peripheral;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ConnectedController {
    controller: Peripheral,
    common_input: Characteristic,
    unique_input: Characteristic,
    output: Characteristic,
    kind: NsControllerKind,
}

impl ConnectedController {
    pub fn new(
        controller: Peripheral,
        common_input: Characteristic,
        unique_input: Characteristic,
        output: Characteristic,
        kind: NsControllerKind,
    ) -> Self {
        Self {
            controller,
            common_input,
            unique_input,
            output,
            kind,
        }
    }

    pub async fn suscribe_inputs(&self) -> btleplug::Result<()> {
        self.controller.subscribe(&self.common_input).await?;
        self.controller.subscribe(&self.unique_input).await
    }

    pub async fn write(&self, cmd: &[u8], write_type: WriteType) -> btleplug::Result<()> {
        self.controller.write(&self.output, cmd, write_type).await
    }

    pub fn controller(&self) -> Peripheral {
        self.controller.clone()
    }

    pub fn common_input_uuid(&self) -> Uuid {
        self.common_input.uuid
    }

    pub fn unique_input_uuid(&self) -> Uuid {
        self.unique_input.uuid
    }

    pub fn kind(&self) -> NsControllerKind {
        self.kind
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.controller.disconnect().await
    }
}
