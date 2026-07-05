use crate::connection::connected_controller::ConnectedController;
use tauri::async_runtime::JoinHandle;

#[derive(Debug)]
pub struct NsController {
    device: ConnectedController,
    input_listener: JoinHandle<()>,
    input_reporter: JoinHandle<()>,
}

impl NsController {
    pub fn new(
        device: ConnectedController,
        input_listener: JoinHandle<()>,
        input_reporter: JoinHandle<()>,
    ) -> Self {
        Self {
            device,
            input_listener,
            input_reporter,
        }
    }

    pub fn device(&self) -> &ConnectedController {
        &self.device
    }

    pub async fn disconnect(&self) -> btleplug::Result<()> {
        self.input_reporter.abort();
        self.input_listener.abort();
        self.device.disconnect().await
    }
}
