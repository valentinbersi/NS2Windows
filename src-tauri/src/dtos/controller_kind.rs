use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum ControllerKind {
    LeftJoyCon,
    RightJoyCon,
    DualJoyCons,
    ProNsoGcController,
}
