use crate::dtos::ns_connected_controller::NsConnectedController;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EmulatedController {
    pub profile_name: String,
    pub connected_controller: NsConnectedController,
}
