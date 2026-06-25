use crate::data::input_data::InputData;
use crate::profiles::input::binary_input::BinaryInput;
use crate::profiles::input::grouping_input::GroupingInput;
use crate::profiles::input::value_input::ValueInput;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Input {
    Value(ValueInput),
    Grouping(Box<GroupingInput>),
    Binary(Box<BinaryInput>),
}

impl Input {
    pub fn evaluate(&self, data: &InputData) -> f32 {
        match self {
            Input::Value(input) => input.evaluate(data),
            Input::Grouping(input) => input.evaluate(data),
            Input::Binary(input) => input.evaluate(data),
        }
    }
}
