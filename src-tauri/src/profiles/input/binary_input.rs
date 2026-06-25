use crate::data::input_data::InputData;
use crate::profiles::input::binary_operator::BinaryOperator;
use crate::profiles::input::input::Input;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct BinaryInput {
    left: Input,
    right: Input,
    operator: BinaryOperator,
}

impl BinaryInput {
    pub fn new(left: Input, right: Input, operator: BinaryOperator) -> Self {
        Self {
            left,
            right,
            operator,
        }
    }

    pub fn evaluate(&self, input: &InputData) -> f32 {
        let left = self.left.evaluate(input);
        let right = self.right.evaluate(input);

        match self.operator {
            BinaryOperator::And => left.min(right),
            BinaryOperator::Or => left.max(right),
        }
    }
}
