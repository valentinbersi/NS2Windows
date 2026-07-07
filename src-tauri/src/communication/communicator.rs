use crate::connection::connected_controller::ConnectedController;
use crate::data::ns_controller_kind::NsControllerKind;
use bitflags::bitflags;
use btleplug::api::WriteType;
use std::time::Duration;
use tokio::time::sleep;

const FEATURE_COMMAND_DELAY: Duration = Duration::from_millis(100);

bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct LedPatten: u8 {
        const Led1 = 0x1;
        const Led2 = 0x2;
        const Led3 = 0x4;
        const Led4 = 0x8;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BluetoothCommunicator;

impl BluetoothCommunicator {
    fn combined_command_prefix_size(kind: NsControllerKind) -> usize {
        match kind {
            NsControllerKind::LeftJoyCon | NsControllerKind::RightJoyCon => 17,
            NsControllerKind::ProController => 33,
            NsControllerKind::NsoGcController => 0,
        }
    }

    fn combined_command(kind: NsControllerKind, command: &[u8]) -> Vec<u8> {
        let mut buffer = vec![0; Self::combined_command_prefix_size(kind)];
        buffer.extend_from_slice(command);
        buffer
    }

    fn feature_mask(kind: NsControllerKind) -> u8 {
        match kind {
            NsControllerKind::LeftJoyCon | NsControllerKind::RightJoyCon => 0x37,
            NsControllerKind::ProController => 0x2f,
            NsControllerKind::NsoGcController => 0x27,
        }
    }

    pub async fn set_feature_mask(&self, device: &ConnectedController) -> btleplug::Result<()> {
        let data = [Self::feature_mask(device.kind()), 0x00, 0x00, 0x00];
        self.send_feature_command(device, 0x02, &data).await
    }

    pub async fn configure_motion(&self, device: &ConnectedController) -> btleplug::Result<()> {
        if device.kind() == NsControllerKind::NsoGcController {
            return Ok(());
        }

        let data = [0x04, 0x00, 0x00, 0x00, 0x02, 0x02, 0x01, 0x00, 0x8a, 0x00];
        self.send_feature_command(device, 0x06, &data).await
    }

    pub async fn enable_features(&self, device: &ConnectedController) -> btleplug::Result<()> {
        let data = [Self::feature_mask(device.kind()), 0x00, 0x00, 0x00];
        self.send_feature_command(device, 0x04, &data).await
    }

    async fn send_feature_command(
        &self,
        device: &ConnectedController,
        sub_cmd_id: u8,
        data: &[u8],
    ) -> btleplug::Result<()> {
        self.send_generic_command(device, 0x0c, sub_cmd_id, data)
            .await?;

        sleep(FEATURE_COMMAND_DELAY).await;

        Ok(())
    }

    pub async fn send_generic_command(
        &self,
        device: &ConnectedController,
        cmd_id: u8,
        sub_cmd_id: u8,
        data: &[u8],
    ) -> btleplug::Result<()> {
        let mut buffer = vec![
            cmd_id,
            0x91,
            0x01,
            sub_cmd_id,
            0x00,
            data.len() as u8,
            0x00,
            0x00,
        ];

        buffer.extend_from_slice(data);

        device.write(&buffer, WriteType::WithoutResponse).await?;

        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    pub async fn initialize_rumble(&self, device: &ConnectedController) -> btleplug::Result<()> {
        let data = match device.kind() {
            NsControllerKind::LeftJoyCon | NsControllerKind::RightJoyCon => [
                0x01, 0x59, 0x09, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0x35, 0x00, 0x46, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],

            NsControllerKind::ProController => [
                0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x35, 0x00, 0x46, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ],

            NsControllerKind::NsoGcController => return Ok(()),
        };

        let mut command = vec![0x0a, 0x91, 0x01, 0x08, 0x00, data.len() as u8, 0x00, 0x00];
        command.extend_from_slice(&data);

        let packet = Self::combined_command(device.kind(), &command);
        device.write_combined_output(&packet).await?;
        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    pub async fn emit_sound(&self, device: &ConnectedController) -> btleplug::Result<()> {
        if device.kind() == NsControllerKind::NsoGcController {
            return Ok(());
        }

        let mut data = [0x00_u8; 8];
        data[0] = 0x04;
        self.send_generic_command(device, 0x0A, 0x02, &data).await
    }

    pub async fn set_device_led(
        &self,
        device: &ConnectedController,
        pattern: LedPatten,
    ) -> btleplug::Result<()> {
        let mut data = [0x00_u8; 8];
        data[0] = pattern.bits();
        self.send_generic_command(device, 0x09, 0x07, &data).await
    }
}
