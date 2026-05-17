use crate::data::output::Output;
use crate::data::profile_kind::ProfileKind;
use crate::profiles::condition::Condition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub profile_name: String,
    pub profile_kind: ProfileKind,
    pub outputs: HashMap<Output, Condition>,
}

impl Profile {
    pub fn new(
        profile_name: String,
        kind: ProfileKind,
        outputs: HashMap<Output, Condition>,
    ) -> Self {
        Self {
            profile_name,
            profile_kind: kind,
            outputs,
        }
    }
}
