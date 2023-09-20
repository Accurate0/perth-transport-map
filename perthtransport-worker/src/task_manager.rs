use anyhow::Ok;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::{sync::RwLock, task::JoinHandle};

pub struct TaskManager {
    // socket id -> list of tasks subscribed
    websocket_subscriptions: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    // trip -> list of subscribed sockets
    task_subscribers: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    // all tasks running
    // TODO: check periodically to see if all handles still alive :)
    existing_tasks: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            websocket_subscriptions: Default::default(),
            existing_tasks: Default::default(),
            task_subscribers: Default::default(),
        }
    }

    pub async fn create_websocket_session(&self, socket_id: String) -> Result<(), anyhow::Error> {
        let mut websocket_subscriptions = self.websocket_subscriptions.write().await;
        // TODO: handle
        if websocket_subscriptions.contains_key(&socket_id) {
            tracing::warn!("[{}] this socket has already said hello", socket_id);
            return Ok(());
        }

        websocket_subscriptions.insert(socket_id, Default::default());

        Ok(())
    }

    // TODO: handle removal of all related websocket sessions
    // heartbeat based or otherwise
    pub async fn destroy_websocket_session(&self, socket_id: String) -> Result<(), anyhow::Error> {
        let mut websocket_subscriptions = self.websocket_subscriptions.write().await;
        let mut task_subscribers = self.task_subscribers.write().await;

        websocket_subscriptions.remove(&socket_id);
        let containing_keys = task_subscribers
            .iter()
            .filter(|(_, v)| v.contains(&socket_id))
            .map(|(k, _)| k.clone())
            .collect::<Vec<String>>();

        for key in containing_keys {
            task_subscribers.entry(key.to_string()).and_modify(|l| {
                l.remove(&socket_id);
            });
        }

        Ok(())
    }

    async fn task_exists(&self, task_id: &String) -> Result<bool, anyhow::Error> {
        let existing_tasks = self.existing_tasks.read().await;
        Ok(existing_tasks.contains_key(task_id))
    }

    pub async fn abort_task(&self, task_id: &String) {
        match self.existing_tasks.write().await.remove(task_id) {
            Some(j) => {
                j.abort();
                tracing::info!("[{}] task aborted", task_id);
            }
            None => tracing::warn!("[{}] attempting to remove task that isn't running", task_id),
        }
    }

    pub async fn add_task_to_websocket_session(
        &self,
        socket_id: String,
        task_id: String,
        start_task: impl Fn() -> JoinHandle<()>,
    ) -> Result<bool, anyhow::Error> {
        let mut websocket_subscriptions = self.websocket_subscriptions.write().await;
        websocket_subscriptions
            .entry(socket_id.clone())
            .and_modify(|list| {
                list.insert(task_id.clone());
            });

        let mut task_subscribers = self.task_subscribers.write().await;
        task_subscribers
            .entry(task_id.clone())
            .and_modify(|socket_list| {
                socket_list.insert(socket_id.clone());
            })
            .or_insert(HashSet::from_iter([socket_id]));

        if self.task_exists(&task_id).await? {
            // Do nothing
            tracing::info!("[{}] task already exists", task_id);

            Ok(false)
        } else {
            let mut existing_tasks = self.existing_tasks.write().await;
            existing_tasks.insert(task_id, start_task());

            Ok(true)
        }
    }

    pub async fn get_all_task_subscribers(&self, task_id: String) -> Option<HashSet<String>> {
        let task_subscribers = self.task_subscribers.read().await;
        task_subscribers.get(&task_id).cloned()
    }

    pub async fn is_healthy(&self) -> bool {
        // TODO: determine what is health for this
        true
    }

    pub async fn get_all_socket_ids(&self) -> Vec<String> {
        self.websocket_subscriptions
            .read()
            .await
            .keys()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
    }

    pub async fn clean_up_dead_tasks(&self) -> Result<(), anyhow::Error> {
        tracing::debug!("checking status of all known tasks");
        let mut dead_tasks = vec![];

        let existing_tasks = self.existing_tasks.read().await;
        for (key, value) in existing_tasks.iter() {
            let running = !value.is_finished();
            if !running {
                dead_tasks.push(key.clone());
            }
        }

        tracing::debug!(
            "total tasks: {}, dead tasks: {}",
            existing_tasks.keys().len(),
            dead_tasks.len()
        );

        if dead_tasks.is_empty() {
            return Ok(());
        }

        // explicit unlock... remember what happened last time...
        std::mem::drop(existing_tasks);

        let mut existing_tasks = self.existing_tasks.write().await;
        for dead_task in dead_tasks {
            existing_tasks.remove(&dead_task);
        }

        tracing::debug!("removed all dead tasks");
        Ok(())
    }
}
