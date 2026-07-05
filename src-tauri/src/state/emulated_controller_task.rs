use tauri::async_runtime::JoinHandle;

pub enum EmulatedControllerTask {
    SingleController {
        input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
        rumble_output: JoinHandle<()>,
    },
    DualJoyCon {
        left_input_listener: JoinHandle<()>,
        right_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
        rumble_output: JoinHandle<()>,
    },
}

impl EmulatedControllerTask {
    pub fn new_single_controller(
        input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
        rumble_output: JoinHandle<()>,
    ) -> Self {
        Self::SingleController {
            input_listener,
            output_emulator,
            rumble_output,
        }
    }

    pub fn new_dual_joy_con(
        left_input_listener: JoinHandle<()>,
        right_input_listener: JoinHandle<()>,
        output_emulator: JoinHandle<()>,
        rumble_output: JoinHandle<()>,
    ) -> Self {
        Self::DualJoyCon {
            left_input_listener,
            right_input_listener,
            output_emulator,
            rumble_output,
        }
    }
}

impl Drop for EmulatedControllerTask {
    fn drop(&mut self) {
        match self {
            EmulatedControllerTask::SingleController {
                input_listener,
                output_emulator,
                rumble_output,
            } => {
                rumble_output.abort();
                output_emulator.abort();
                input_listener.abort();
            }

            EmulatedControllerTask::DualJoyCon {
                left_input_listener,
                right_input_listener,
                output_emulator,
                rumble_output,
            } => {
                rumble_output.abort();
                output_emulator.abort();
                left_input_listener.abort();
                right_input_listener.abort();
            }
        }
    }
}
