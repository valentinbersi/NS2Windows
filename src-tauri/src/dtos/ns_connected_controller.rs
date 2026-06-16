use crate::dtos::motion_source::MotionSource;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SingleController {
    pub id: Uuid,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DualJoyCon {
    pub left_id: Uuid,
    pub right_id: Uuid,
    pub motion_source: MotionSource,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NsConnectedController {
    SingleController(SingleController),
    DualJoyCon(DualJoyCon),
}
