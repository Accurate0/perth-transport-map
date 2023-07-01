use std::sync::Arc;
use tokio::task::JoinHandle;

use crate::task_manager::TaskManager;

#[derive(Clone)]
pub struct AppState {
    pub worker_out_handle: Arc<JoinHandle<()>>,
    pub task_manager_handle: Arc<JoinHandle<()>>,
    pub active_trains_handle: Arc<JoinHandle<()>>,
    pub task_manager: Arc<TaskManager>,
}
