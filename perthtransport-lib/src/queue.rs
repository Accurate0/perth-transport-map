use std::sync::Arc;

use crate::constants::PUBSUB_CHANNEL_OUT_PREFIX;
use redis::AsyncCommands;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct MessageBus {
    redis_client: redis::Client,
    redis_connection: Arc<Mutex<redis::aio::ConnectionManager>>,
}

impl MessageBus {
    pub async fn new(client: redis::Client) -> Result<Self, anyhow::Error> {
        Ok(Self {
            redis_client: client.clone(),
            redis_connection: Arc::new(client.get_tokio_connection_manager().await?.into()),
        })
    }

    pub async fn publish_socket<T: serde::Serialize>(
        &self,
        socket_id: &str,
        message: T,
    ) -> Result<(), anyhow::Error> {
        let mut redis_connection = self.redis_connection.lock().await;
        let channel = format!("{}_{}", PUBSUB_CHANNEL_OUT_PREFIX, socket_id);
        redis_connection
            .publish(channel, serde_json::to_string(&message)?)
            .await?;

        Ok(())
    }

    pub async fn publish<T: serde::Serialize>(
        &self,
        channel: &str,
        message: T,
    ) -> Result<(), anyhow::Error> {
        let mut redis_connection = self.redis_connection.lock().await;
        redis_connection
            .publish(channel, serde_json::to_string(&message)?)
            .await?;

        Ok(())
    }

    pub async fn subscribe(&self, channels: &[&str]) -> Result<redis::aio::PubSub, anyhow::Error> {
        let mut pubsub = self.redis_client.get_async_pubsub().await?;

        for channel in channels {
            pubsub.subscribe(channel).await?
        }

        Ok(pubsub)
    }
}
