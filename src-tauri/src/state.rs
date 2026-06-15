use crate::communication::communicator::BluetoothCommunicator;
use crate::connection::connected_controller::NsController;
use crate::connection::connector::BluetoothConnector;
use crate::repositories::profile_repository::ProfileRepository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use vigem_rust::Client;

pub struct AppState {
    pub profile_repository: ProfileRepository,
    pub connector: BluetoothConnector,
    pub communicator: BluetoothCommunicator,
    pub connected_controllers: RwLock<HashMap<Uuid, Arc<NsController>>>,
    pub vigem_client: Client,
}
