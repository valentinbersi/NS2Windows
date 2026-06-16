use crate::communication::communicator::BluetoothCommunicator;
use crate::connection::connector::BluetoothConnector;
use crate::repositories::profile_repository::ProfileRepository;
use crate::state::emulated_controller_task::EmulatedControllerTask;
use crate::state::ns_controller::NsController;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use vigem_rust::Client;

pub struct AppState {
    pub profile_repository: ProfileRepository,

    pub connector: BluetoothConnector,
    pub communicator: BluetoothCommunicator,

    pub vigem_client: Client,

    connected_controllers: RwLock<HashMap<Uuid, Arc<NsController>>>,
    emulated_controllers: RwLock<HashMap<Uuid, EmulatedControllerTask>>,
}

impl AppState {
    pub fn new(
        profile_repository: ProfileRepository,
        connector: BluetoothConnector,
        communicator: BluetoothCommunicator,
        vigem: Client,
    ) -> Self {
        Self {
            profile_repository,
            connector,
            communicator,
            vigem_client: vigem,
            connected_controllers: Default::default(),
            emulated_controllers: Default::default(),
        }
    }

    pub async fn insert_ns_controller(&self, id: Uuid, controller: NsController) {
        self.connected_controllers
            .write()
            .await
            .entry(id)
            .insert_entry(Arc::new(controller));
    }

    pub async fn get_ns_controller(&self, id: &Uuid) -> Option<Arc<NsController>> {
        self.connected_controllers.read().await.get(id).cloned()
    }

    pub async fn get_dual_ns_controllers(
        &self,
        left: &Uuid,
        right: &Uuid,
    ) -> Result<(Arc<NsController>, Arc<NsController>), Uuid> {
        let connected_controllers = self.connected_controllers.read().await;

        let left = connected_controllers.get(left).cloned().ok_or(*left)?;

        let right = connected_controllers.get(right).cloned().ok_or(*right)?;

        Ok((left, right))
    }

    pub async fn remove_connected_controller(&self, id: &Uuid) -> btleplug::Result<Option<()>> {
        let controller = self.connected_controllers.write().await.remove(id);

        match controller {
            None => Ok(None),
            Some(controller) => {
                controller.disconnect().await?;
                Ok(Some(()))
            }
        }
    }

    pub async fn insert_emulated_controller(&self, id: Uuid, controller: EmulatedControllerTask) {
        self.emulated_controllers
            .write()
            .await
            .entry(id)
            .insert_entry(controller);
    }

    pub async fn remove_emulated_controller(&self, id: &Uuid) -> Option<()> {
        self.emulated_controllers
            .write()
            .await
            .remove(id)
            .map(|_| ())
    }

    pub async fn cleanup(&self) -> Result<(), String> {
        for controller in self.connected_controllers.read().await.values() {
            controller
                .disconnect()
                .await
                .map_err(|err| err.to_string())?;
        }

        Ok(())
    }
}
