use tauri::async_runtime::JoinHandle;

pub enum EmulatedControllerTask {
    SingleController {
        unique_input_listener: JoinHandle<()>,
        common_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    },
    DualJoyCon {
        left_unique_input_listener: JoinHandle<()>,
        left_common_input_listener: JoinHandle<()>,
        right_unique_input_listener: JoinHandle<()>,
        right_common_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    },
}

impl EmulatedControllerTask {
    pub fn new_single_controller(
        unique_input_listener: JoinHandle<()>,
        common_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    ) -> Self {
        Self::SingleController {
            unique_input_listener,
            common_input_listener,
            output_emulator,
        }
    }

    pub fn new_dual_joy_con(
        left_unique_input_listener: JoinHandle<()>,
        left_common_input_listener: JoinHandle<()>,
        right_unique_input_listener: JoinHandle<()>,
        right_common_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
    ) -> Self {
        Self::DualJoyCon {
            left_unique_input_listener,
            left_common_input_listener,
            right_unique_input_listener,
            right_common_input_listener,
            output_emulator,
        }
    }
}

impl Drop for EmulatedControllerTask {
    fn drop(&mut self) {
        match self {
            EmulatedControllerTask::SingleController {
                unique_input_listener,
                common_input_listener,
                output_emulator,
            } => {
                output_emulator.abort();
                common_input_listener.abort();
                unique_input_listener.abort();
            }

            EmulatedControllerTask::DualJoyCon {
                left_unique_input_listener,
                left_common_input_listener,
                right_unique_input_listener,
                right_common_input_listener,
                output_emulator,
            } => {
                output_emulator.abort();
                left_unique_input_listener.abort();
                left_common_input_listener.abort();
                right_unique_input_listener.abort();
                right_common_input_listener.abort();
            }
        }
    }
}
