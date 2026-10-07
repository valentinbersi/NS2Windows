use crate::cemuhook::{CemuHookRegistration, MotionPublisher};
use tauri::async_runtime::JoinHandle;
use uuid::Uuid;

/// Also a startup guard: spawned tasks are owned immediately, so a later
/// error or cancelled start cannot detach listeners or reserve a DSU slot.
pub struct EmulatedControllerTask {
    tasks: Vec<JoinHandle<()>>,
    physical_ids: Vec<Uuid>,
    motion: Option<CemuHookRegistration>,
}

impl EmulatedControllerTask {
    pub fn new(physical_ids: Vec<Uuid>, motion: Option<CemuHookRegistration>) -> Self {
        Self {
            tasks: Vec::new(),
            physical_ids,
            motion,
        }
    }

    pub fn push(&mut self, task: JoinHandle<()>) {
        self.tasks.push(task);
    }

    pub fn publisher(&self) -> Option<MotionPublisher> {
        self.motion.as_ref().map(CemuHookRegistration::publisher)
    }

    pub fn uses(&self, id: &Uuid) -> bool {
        self.physical_ids.contains(id)
    }

    pub fn physical_ids(&self) -> &[Uuid] {
        &self.physical_ids
    }
}

impl Drop for EmulatedControllerTask {
    fn drop(&mut self) {
        for task in self.tasks.iter().rev() {
            task.abort();
        }
        // Registration drops synchronously after aborting the producers.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cemuhook::CemuHookServer;
    use std::net::UdpSocket;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn failed_or_cancelled_start_aborts_tasks_and_releases_reservation() {
        let probe = UdpSocket::bind("127.0.0.1:0").unwrap();
        let endpoint = probe.local_addr().unwrap();
        drop(probe);
        let server = CemuHookServer::new(endpoint);
        let physical = Uuid::new_v4();
        let registration = server.reserve(&[physical]).unwrap();
        let mut task = EmulatedControllerTask::new(vec![physical], Some(registration));
        assert!(task.uses(&physical));
        assert!(!task.uses(&Uuid::nil()));
        let dropped = Arc::new(AtomicBool::new(false));
        struct DropSignal(Arc<AtomicBool>);
        impl Drop for DropSignal {
            fn drop(&mut self) {
                self.0.store(true, Ordering::SeqCst);
            }
        }
        let signal = DropSignal(dropped.clone());
        let (started, ready) = tokio::sync::oneshot::channel();
        task.push(tauri::async_runtime::spawn(async move {
            let _signal = signal;
            let _ = started.send(());
            std::future::pending::<()>().await;
        }));
        ready.await.unwrap();
        drop(task); // Same cleanup path used by stop/disconnect/start errors.
        assert!(UdpSocket::bind(endpoint).is_ok());
        tokio::time::timeout(std::time::Duration::from_secs(2), async {
            while !dropped.load(Ordering::SeqCst) {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        assert!(server.reserve(&[physical]).is_ok());
    }
}
