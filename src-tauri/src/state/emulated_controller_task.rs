use tauri::async_runtime::JoinHandle;

pub enum EmulatedControllerTask {
    SingleController(JoinHandle<()>),
    DualJoyCon {
        left: JoinHandle<()>,
        right: JoinHandle<()>,
        output: JoinHandle<()>,
    },
}

impl EmulatedControllerTask {
    pub fn new_single_controller(task: JoinHandle<()>) -> Self {
        Self::SingleController(task)
    }

    pub fn new_dual_joy_con(
        left: JoinHandle<()>,
        right: JoinHandle<()>,
        output: JoinHandle<()>,
    ) -> Self {
        Self::DualJoyCon {
            left,
            right,
            output,
        }
    }
}

impl Drop for EmulatedControllerTask {
    fn drop(&mut self) {
        match self {
            EmulatedControllerTask::SingleController(controller) => controller.abort(),

            EmulatedControllerTask::DualJoyCon {
                left,
                right,
                output,
            } => {
                left.abort();
                right.abort();
                output.abort();
            }
        }
    }
}
