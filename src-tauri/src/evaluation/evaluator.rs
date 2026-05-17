use crate::data::input_data::InputData;
use crate::data::output::Output;
use crate::data::output_data::OutputData;
use crate::profiles::profile::Profile;
use maplit::hashmap;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Evaluator;

impl Evaluator {
    fn evaluate_condition(&self, profile: &Profile, input_data: &InputData, output: Output) -> f32 {
        profile
            .outputs
            .get(&output)
            .map(|condition| condition.evaluate(input_data))
            .unwrap_or(0_f32)
    }

    pub fn evaluate_profile(&self, profile: &Profile, input_data: &InputData) -> OutputData {
        OutputData::new(hashmap! {
            Output::CrossA => self.evaluate_condition(profile, input_data, Output::CrossA),
            Output::CircleB => self.evaluate_condition(profile, input_data, Output::CircleB),
            Output::SquareX => self.evaluate_condition(profile, input_data, Output::SquareX),
            Output::TriangleY => self.evaluate_condition(profile, input_data, Output::TriangleY),

            Output::PsGuide => self.evaluate_condition(profile, input_data, Output::PsGuide),
            Output::Share => self.evaluate_condition(profile, input_data, Output::Share),

            Output::R1Rb => self.evaluate_condition(profile, input_data, Output::R1Rb),
            Output::R2Rt => self.evaluate_condition(profile, input_data, Output::R2Rt),
            Output::R3Rs => self.evaluate_condition(profile, input_data, Output::R3Rs),

            Output::L1Lb => self.evaluate_condition(profile, input_data, Output::L1Lb),
            Output::L2Lt => self.evaluate_condition(profile, input_data, Output::L2Lt),
            Output::L3Ls => self.evaluate_condition(profile, input_data, Output::L3Ls),

            Output::OptionsStart => self.evaluate_condition(profile, input_data, Output::OptionsStart),
            Output::TouchpadBack => self.evaluate_condition(profile, input_data, Output::TouchpadBack),

            Output::Down => self.evaluate_condition(profile, input_data, Output::Down),
            Output::Left => self.evaluate_condition(profile, input_data, Output::Left),
            Output::Right => self.evaluate_condition(profile, input_data, Output::Right),
            Output::Up => self.evaluate_condition(profile, input_data, Output::Up),

            Output::LeftXMinus => self.evaluate_condition(profile, input_data, Output::LeftXMinus),
            Output::LeftXPlus => self.evaluate_condition(profile, input_data, Output::LeftXPlus),
            Output::LeftYMinus => self.evaluate_condition(profile, input_data, Output::LeftYMinus),
            Output::LeftYPlus => self.evaluate_condition(profile, input_data, Output::LeftYPlus),

            Output::RightXMinus => self.evaluate_condition(profile, input_data, Output::RightXMinus),
            Output::RightXPlus => self.evaluate_condition(profile, input_data, Output::RightXPlus),
            Output::RightYMinus => self.evaluate_condition(profile, input_data, Output::RightYMinus),
            Output::RightYPlus => self.evaluate_condition(profile, input_data, Output::RightYPlus),

            Output::AccelUp => self.evaluate_condition(profile, input_data, Output::AccelUp),
            Output::AccelDown => self.evaluate_condition(profile, input_data, Output::AccelDown),
            Output::AccelLeft => self.evaluate_condition(profile, input_data, Output::AccelLeft),
            Output::AccelRight => self.evaluate_condition(profile, input_data, Output::AccelRight),
            Output::AccelForward => self.evaluate_condition(profile, input_data, Output::AccelForward),
            Output::AccelBackward => self.evaluate_condition(profile, input_data, Output::AccelBackward),

            Output::GyroPitchUp => self.evaluate_condition(profile, input_data, Output::GyroPitchUp),
            Output::GyroPitchDown => self.evaluate_condition(profile, input_data, Output::GyroPitchDown),
            Output::GyroRollLeft => self.evaluate_condition(profile, input_data, Output::GyroRollLeft),
            Output::GyroRollRight => self.evaluate_condition(profile, input_data, Output::GyroRollRight),
            Output::GyroYawLeft => self.evaluate_condition(profile, input_data, Output::GyroYawLeft),
            Output::GyroYawRight => self.evaluate_condition(profile, input_data, Output::GyroYawRight),
        })
    }
}
