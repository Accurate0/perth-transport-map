use futures_util::StreamExt;
use http::header::{ACCEPT_ENCODING, CONTENT_TYPE, HOST, USER_AGENT};
use pta::constants::{PUBSUB_CHANNEL_GENERAL_IN, PUBSUB_CHANNEL_WORKER_HEALTH_IN};
use reqwest::header::HeaderMap;
use reqwest_tracing::TracingMiddleware;
use std::sync::Arc;
use task_manager::TaskManager;
use tokio::sync::RwLock;
use tracing::{Instrument, Level};

use crate::tasks::handle_healthcheck;

mod auth;
mod task_manager;
mod tasks;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pta::log::init_logger();
    let config = Arc::new(pta::config::get_application_config()?);
    let redis = redis::Client::open(config.redis_connection_string.clone())?;

    let redis_multiplexed = Arc::new(RwLock::new(redis.get_multiplexed_async_connection().await?));
    let mut redis_pubsub = redis.get_async_connection().await?.into_pubsub();
    redis_pubsub.subscribe(PUBSUB_CHANNEL_GENERAL_IN).await?;
    redis_pubsub
        .subscribe(PUBSUB_CHANNEL_WORKER_HEALTH_IN)
        .await?;

    let mut default_headers = HeaderMap::new();
    default_headers.append(ACCEPT_ENCODING, "gzip".parse()?);
    default_headers.append(CONTENT_TYPE, "application/json".parse()?);
    default_headers.append(HOST, "realtime.transperth.info".parse()?);
    default_headers.append(USER_AGENT, "okhttp/4.9.2".parse()?);

    let http_client = Arc::new(
        reqwest_middleware::ClientBuilder::new(
            reqwest::ClientBuilder::new()
                .default_headers(default_headers)
                .build()?,
        )
        .with(TracingMiddleware::default())
        .build(),
    );

    let task_manager = Arc::new(TaskManager::new());

    let worker_redis_connection = redis.get_multiplexed_async_connection().await?;
    let worker_pubsub_connection = redis.get_async_connection().await?;
    let worker_task_manager = Arc::clone(&task_manager);

    tracing::info!("starting worker");
    let span = tracing::span!(Level::INFO, "trip_out");
    let worker_out_handle = Arc::new(tokio::spawn(async move {
        if let Err(e) = tasks::handle_worker_out(
            worker_redis_connection,
            worker_pubsub_connection,
            worker_task_manager,
        )
        .instrument(span)
        .await
        {
            tracing::error!("error handling worker out task: {}", e)
        }
    }));

    loop {
        let http_client = Arc::clone(&http_client);
        let redis_multiplexed = Arc::clone(&redis_multiplexed);
        let task_manager = Arc::clone(&task_manager);
        let worker_out_handle = Arc::clone(&worker_out_handle);
        let config = Arc::clone(&config);
        let message = redis_pubsub.on_message().next().await;

        if let Some(message) = message {
            let channel_name = message.get_channel_name();
            // health check, write back on out channel
            let span = tracing::info_span!("health_check");
            if channel_name == PUBSUB_CHANNEL_WORKER_HEALTH_IN {
                tokio::spawn(
                    handle_healthcheck(worker_out_handle, redis_multiplexed, task_manager)
                        .instrument(span),
                );
            } else {
                tokio::spawn(async {
                    if let Err(e) = tasks::handle_message(
                        http_client,
                        redis_multiplexed,
                        task_manager,
                        message,
                        config,
                    )
                    .await
                    {
                        tracing::error!("error handling message task: {}", e)
                    }
                });
            }
        }
    }
}
