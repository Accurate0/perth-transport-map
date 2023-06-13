use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerHealthStatus {
    pub worker_output_healthy: bool,
    pub task_manager_healthy: bool,
}
