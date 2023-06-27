use crate::tasks::handle_healthcheck;
use futures_util::StreamExt;
use http::header::{ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, HOST, USER_AGENT};
use perthtransport::{
    constants::{PUBSUB_CHANNEL_GENERAL_IN, PUBSUB_CHANNEL_WORKER_HEALTH_IN},
    log::TimeTrace,
    queue::MessageBus,
    types::message::WorkerMessage,
};
use reqwest::header::HeaderMap;
use reqwest_tracing::TracingMiddleware;
use std::sync::Arc;
use task_manager::TaskManager;
use tokio::sync::RwLock;
use tracing::{Instrument, Level};

mod auth;
mod task_manager;
mod tasks;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), anyhow::Error> {
    perthtransport::log::init_logger();
    let config = Arc::new(perthtransport::config::get_application_config()?);
    let redis = redis::Client::open(config.redis_connection_string.clone())?;

    let redis_multiplexed = Arc::new(RwLock::new(redis.get_multiplexed_async_connection().await?));
    let message_bus = MessageBus::new(redis.clone()).await?;
    let mut pubsub = message_bus
        .subscribe(&[PUBSUB_CHANNEL_GENERAL_IN, PUBSUB_CHANNEL_WORKER_HEALTH_IN])
        .await?;

    let mut default_headers = HeaderMap::new();
    default_headers.append(ACCEPT_ENCODING, "gzip".parse()?);
    default_headers.append(CONTENT_TYPE, "application/json".parse()?);
    default_headers.append(ACCEPT, "application/json".parse()?);
    default_headers.append(HOST, "realtime.transperth.info".parse()?);
    default_headers.append(USER_AGENT, "okhttp/4.9.2".parse()?);

    let http_client = Arc::new(
        reqwest_middleware::ClientBuilder::new(
            reqwest::ClientBuilder::new()
                .default_headers(default_headers)
                .build()?,
        )
        .with(TracingMiddleware::<TimeTrace>::new())
        .build(),
    );

    let task_manager = Arc::new(TaskManager::new());

    let worker_redis_connection = redis.get_multiplexed_async_connection().await?;
    let worker_task_manager = Arc::clone(&task_manager);

    tracing::info!("starting worker");
    let span = tracing::span!(Level::INFO, "trip_out");
    let (worker_tx, worker_rx) = flume::unbounded::<WorkerMessage>();

    let worker_out_handle = Arc::new(tokio::spawn(async move {
        if let Err(e) =
            tasks::handle_worker_out(worker_redis_connection, worker_rx, worker_task_manager)
                .instrument(span)
                .await
        {
            tracing::error!("error handling worker out task: {}", e)
        }
    }));

    let background_task_manager = Arc::clone(&task_manager);
    let span = tracing::span!(Level::INFO, "task_manager");
    let task_manager_out_handle = Arc::new(tokio::spawn(async move {
        if let Err(e) = tasks::handle_task_manager(background_task_manager)
            .instrument(span)
            .await
        {
            tracing::error!("error handling worker out task: {}", e)
        }
    }));

    let active_trains_task_manager = Arc::clone(&task_manager);
    let active_trains_redis_connection = redis.get_multiplexed_async_connection().await?;
    let span = tracing::span!(Level::INFO, "active_trains");
    let active_trains_http_client = Arc::clone(&http_client);
    let active_trains_config = Arc::clone(&config);

    let active_trains_handle = Arc::new(tokio::spawn(async move {
        if let Err(e) = tasks::handle_active_trains(
            active_trains_task_manager,
            active_trains_config,
            active_trains_http_client,
            active_trains_redis_connection,
        )
        .instrument(span)
        .await
        {
            tracing::error!("error handling active trains task: {}", e)
        }
    }));

    while let Some(message) = pubsub.on_message().next().await {
        let http_client = Arc::clone(&http_client);
        let redis_multiplexed = Arc::clone(&redis_multiplexed);
        let task_manager = Arc::clone(&task_manager);
        let worker_out_handle = Arc::clone(&worker_out_handle);
        let task_manager_out_handle = Arc::clone(&task_manager_out_handle);
        let active_trains_handle = Arc::clone(&active_trains_handle);
        let worker_tx = worker_tx.clone();
        let config = Arc::clone(&config);

        let channel_name = message.get_channel_name();
        let span = tracing::info_span!("health_check");
        if channel_name == PUBSUB_CHANNEL_WORKER_HEALTH_IN {
            tokio::spawn(
                handle_healthcheck(
                    worker_out_handle,
                    task_manager_out_handle,
                    active_trains_handle,
                    redis_multiplexed,
                    task_manager,
                )
                .instrument(span),
            );
        } else {
            tokio::spawn(async {
                if let Err(e) = tasks::handle_message(
                    http_client,
                    worker_tx,
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

    Ok(())
}
