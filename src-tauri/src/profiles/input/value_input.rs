use crate::data::input_data::InputData;
use crate::data::ns_input::NsInput;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ValueInput {
    input: NsInput,
}

impl ValueInput {
    pub fn new(input: NsInput) -> Self {
        Self { input }
    }

    pub fn evaluate(&self, input: &InputData) -> f32 {
        input.get(self.input).unwrap_or(0_f32)
    }
}
