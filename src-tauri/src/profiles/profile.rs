use crate::data::output::Output;
use crate::data::profile_kind::ProfileKind;
use crate::profiles::input::Input;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub kind: ProfileKind,
    pub outputs: HashMap<Output, Input>,
}

impl Profile {
    pub fn new(name: String, kind: ProfileKind, outputs: HashMap<Output, Input>) -> Self {
        Self {
            name,
            kind,
            outputs,
        }
    }
}
