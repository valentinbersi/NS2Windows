use crate::data::output::Output;
use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::ops::Index;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputData {
    outputs: HashMap<Output, f32>,
}

impl OutputData {
    pub fn new(outputs: HashMap<Output, f32>) -> Self {
        Self { outputs }
    }
}

impl IntoIterator for OutputData {
    type Item = (Output, f32);
    type IntoIter = IntoIter<Output, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.outputs.into_iter()
    }
}

impl Index<Output> for OutputData {
    type Output = f32;

    fn index(&self, index: Output) -> &Self::Output {
        &self.outputs[&index]
    }
}

impl OutputData {
    pub fn get(&self, output: Output) -> Option<f32> {
        self.outputs.get(&output).cloned()
    }

    pub fn get_mut(&mut self, output: Output) -> Option<&mut f32> {
        self.outputs.get_mut(&output)
    }
}
