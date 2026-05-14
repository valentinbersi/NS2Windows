use crate::dtos::controller_kind::ControllerKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Connection {
    pub id: Uuid,
    pub controller_kind: ControllerKind,
}
