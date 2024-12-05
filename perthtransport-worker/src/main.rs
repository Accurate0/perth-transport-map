use crate::{routes::health_check, types::AppState};
use axum::{body::Body, routing::get, Router};
use futures_util::StreamExt;
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE, USER_AGENT},
    Request,
};
use perthtransport::{
    constants::PUBSUB_CHANNEL_GENERAL_IN, log::TimeTrace, queue::MessageBus,
    types::message::WorkerMessage,
};
use reqwest::header::HeaderMap;
use reqwest_tracing::TracingMiddleware;
use std::{future::IntoFuture, net::SocketAddr, sync::Arc};
use task_manager::TaskManager;
use tokio::{net::TcpListener, sync::RwLock};
use tower::limit::GlobalConcurrencyLimitLayer;
use tower_http::{
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{Instrument, Level};

mod auth;
mod routes;
mod task_manager;
mod tasks;
mod types;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), anyhow::Error> {
    perthtransport::log::init_logger();
    let config = Arc::new(perthtransport::config::get_application_config()?);
    let redis = redis::Client::open(config.redis_connection_string.clone())?;

    let redis_multiplexed = Arc::new(RwLock::new(redis.get_multiplexed_async_connection().await?));
    let message_bus = MessageBus::new(redis.clone()).await?;
    let mut pubsub = message_bus.subscribe(&[PUBSUB_CHANNEL_GENERAL_IN]).await?;

    let mut default_headers = HeaderMap::new();
    default_headers.append(ACCEPT_ENCODING, "gzip".parse()?);
    default_headers.append(CONTENT_TYPE, "application/json".parse()?);
    default_headers.append(ACCEPT, "application/json".parse()?);
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
    let task_manager_handle = Arc::new(tokio::spawn(async move {
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
    let worker_tx_cloned = worker_tx.clone();

    let active_trains_handle = Arc::new(tokio::spawn(async move {
        if let Err(e) = tasks::handle_active_trains(
            active_trains_task_manager,
            active_trains_config,
            active_trains_http_client,
            active_trains_redis_connection,
            worker_tx_cloned,
        )
        .instrument(span)
        .await
        {
            tracing::error!("error handling active trains task: {}", e)
        }
    }));

    let worker_state = AppState {
        worker_out_handle,
        task_manager_handle,
        task_manager: Arc::clone(&task_manager),
        active_trains_handle,
    };

    let routes = Router::new()
        .route("/status/health", get(health_check))
        .with_state(worker_state);

    let app = Router::new()
        .nest("/v1", routes)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::info_span!("worker", uri = request.uri().to_string())
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Millis),
                ),
        )
        .layer(GlobalConcurrencyLimitLayer::new(2048));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    tracing::info!("server starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("server starting on {}", addr);
    tokio::spawn(
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .into_future(),
    );

    while let Some(message) = pubsub.on_message().next().await {
        let http_client = Arc::clone(&http_client);
        let redis_multiplexed = Arc::clone(&redis_multiplexed);
        let task_manager = Arc::clone(&task_manager);
        let worker_tx = worker_tx.clone();
        let config = Arc::clone(&config);

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

    Ok(())
}
