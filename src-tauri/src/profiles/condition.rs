use crate::data::input_data::InputData;
use crate::data::ns_input::NsInput;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Condition {
    Value(NsInput),
}

impl Condition {
    pub fn evaluate(&self, data: &InputData) -> f32 {
        match self {
            Condition::Value(input) => data.get(*input).unwrap_or(0_f32),
        }
    }
}
