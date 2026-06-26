use tauri::async_runtime::JoinHandle;

pub enum EmulatedControllerTask {
    SingleController {
        input_pooler: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    },
    DualJoyCon {
        left: JoinHandle<()>,
        right: JoinHandle<()>,
        output: JoinHandle<()>,
    },
}

impl EmulatedControllerTask {
    pub fn new_single_controller(
        input_pooler: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    ) -> Self {
        Self::SingleController {
            input_pooler,
            output_emulator,
        }
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
            EmulatedControllerTask::SingleController {
                input_pooler,
                output_emulator,
            } => {
                output_emulator.abort();
                input_pooler.abort();
            }

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
