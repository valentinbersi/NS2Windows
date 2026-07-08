use crate::data::input_data::InputData;
use crate::profiles::input::input::Input;
use crate::profiles::input::unary_operator::UnaryOperator;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct UnaryInput {
    input: Input,
    operator: UnaryOperator,
}

impl UnaryInput {
    pub fn new(input: Input, operator: UnaryOperator) -> Self {
        Self { input, operator }
    }

    pub fn evaluate(&self, input: &InputData) -> f32 {
        let input = self.input.evaluate(input);

        match self.operator {
            UnaryOperator::Not => 1_f32 - input,
        }
    }
}
