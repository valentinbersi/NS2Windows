use crate::data::input_data::InputData;
use crate::profiles::input::input::Input;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GroupingInput {
    input: Input,
}

impl GroupingInput {
    pub fn new(input: Input) -> Self {
        Self { input }
    }

    pub fn evaluate(&self, input: &InputData) -> f32 {
        self.input.evaluate(input)
    }
}
