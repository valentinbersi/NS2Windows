use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum NsControllerKind {
    LeftJoyCon,
    RightJoyCon,
    ProController,
    NsoGcController,
}
