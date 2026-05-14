use crate::data::ns_input::NsInput;
use std::collections::hash_map::IntoIter;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputData {
    inputs: HashMap<NsInput, f32>,
}

impl InputData {
    pub fn new(inputs: HashMap<NsInput, f32>) -> Self {
        Self { inputs }
    }
}

impl IntoIterator for InputData {
    type Item = (NsInput, f32);
    type IntoIter = IntoIter<NsInput, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.inputs.into_iter()
    }
}

impl Index<NsInput> for InputData {
    type Output = f32;

    fn index(&self, index: NsInput) -> &Self::Output {
        &self.inputs[&index]
    }
}

impl InputData {
    pub fn get(&self, input: NsInput) -> Option<f32> {
        self.inputs.get(&input).cloned()
    }

    pub fn get_mut(&mut self, input: NsInput) -> Option<&mut f32> {
        self.inputs.get_mut(&input)
    }
}
