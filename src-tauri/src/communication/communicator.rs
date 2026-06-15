use crate::connection::connected_device::ConnectedDevice;
use bitflags::bitflags;
use btleplug::api::{Peripheral as PeripheralTrait, WriteType};
use std::time::Duration;
use tokio::time::sleep;

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
    pub async fn send_custom_command(&self, device: &ConnectedDevice) -> btleplug::Result<()> {
        let commands = [
            [
                0x0c_u8, 0x91, 0x01, 0x02, 0x00, 0x04, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
            ],
            [
                0x0c, 0x91, 0x01, 0x04, 0x00, 0x04, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00,
            ],
        ];

        for cmd in commands {
            device
                .peripheral
                .write(&device.write_char, &cmd, WriteType::WithoutResponse)
                .await?;

            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }

    pub async fn send_generic_command(
        &self,
        device: &ConnectedDevice,
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

        device
            .peripheral
            .write(&device.write_char, &buffer, WriteType::WithoutResponse)
            .await?;

        sleep(Duration::from_millis(50)).await;

        Ok(())
    }

    pub async fn emit_sound(&self, device: &ConnectedDevice) -> btleplug::Result<()> {
        let mut data = [0x00_u8; 8];
        data[0] = 0x04;
        self.send_generic_command(device, 0x0A, 0x02, &data).await
    }

    pub async fn set_device_led(
        &self,
        device: &ConnectedDevice,
        pattern: LedPatten,
    ) -> btleplug::Result<()> {
        let mut data = [0x00_u8; 8];
        data[0] = pattern.bits();
        self.send_generic_command(device, 0x09, 0x07, &data).await
    }
}
