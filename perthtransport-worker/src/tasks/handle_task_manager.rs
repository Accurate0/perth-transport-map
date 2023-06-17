use std::{sync::Arc, time::Duration};

use crate::task_manager::TaskManager;

pub async fn handle_task_manager(task_manager: Arc<TaskManager>) -> Result<(), anyhow::Error> {
    loop {
        task_manager.clean_up_dead_tasks().await?;
        tokio::time::sleep(Duration::from_secs(15)).await;
    }
}
