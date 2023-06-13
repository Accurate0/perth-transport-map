use crate::types::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use pta::{
    constants::{PUBSUB_CHANNEL_GENERAL_IN, PUBSUB_CHANNEL_OUT_PREFIX},
    types::message::{PubSubAction, PubSubMessage, WebSocketMessage},
};
use redis::{aio::MultiplexedConnection, AsyncCommands};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tracing::{Instrument, Level};

pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    tracing::info!("client: {} has connected", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, axum::extract::State(state)))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr, state: State<AppState>) {
    let socket_id = uuid::Uuid::new_v4().as_hyphenated().to_string();
    let (sender, receiver) = socket.split();

    let incoming_span = tracing::span!(Level::INFO, "ws_incoming", socket_id = socket_id);
    let mut incoming_handle = tokio::spawn(
        handle_incoming(receiver, who, state.clone(), socket_id.clone()).instrument(incoming_span),
    );
    let outgoing_span = tracing::span!(Level::INFO, "ws_outgoing", socket_id = socket_id);
    let mut outgoing_handle = tokio::spawn(
        handle_outgoing(sender, who, state, socket_id.clone()).instrument(outgoing_span),
    );

    tokio::select! {
        rv_a = (&mut incoming_handle) => {
            match rv_a {
                Ok(_) => tracing::info!("client: {} incoming handle ended, aborting", who),
                Err(a) => tracing::error!("Error sending messages {:?}", a)
            }
            outgoing_handle.abort();
        },
        rv_b = (&mut outgoing_handle) => {
            match rv_b {
                Ok(_) => tracing::info!("client: {} outgoing handle ended, aborting", who),
                Err(a) => tracing::error!("Error sending messages {:?}", a)
            }
            incoming_handle.abort();
        }
    }
}

async fn handle_outgoing(
    mut sender: SplitSink<WebSocket, Message>,
    _who: SocketAddr,
    state: State<AppState>,
    socket_id: String,
) -> Result<(), anyhow::Error> {
    let mut redis = state.redis.get_async_connection().await?.into_pubsub();
    let channel = format!("{}_{}", PUBSUB_CHANNEL_OUT_PREFIX, socket_id);
    redis.subscribe(channel.clone()).await?;

    tracing::info!("subscribed: {}", channel);

    loop {
        let msg = redis.on_message().next().await;
        if let Some(msg) = msg {
            tracing::info!("message recieved");
            let _ = sender.send(Message::Text(msg.get_payload().unwrap())).await;
        }
    }
}

async fn handle_valid_message(
    socket_id: String,
    who: SocketAddr,
    redis: Arc<RwLock<MultiplexedConnection>>,
    m: String,
) -> Result<(), anyhow::Error> {
    let mut redis = redis.write().await;
    let message_result = serde_json::from_str::<WebSocketMessage>(&m);
    if let Ok(websocket_message) = message_result {
        redis
            .publish(
                PUBSUB_CHANNEL_GENERAL_IN,
                serde_json::to_string(&PubSubMessage {
                    // TODO: other message types
                    action: PubSubAction::TripAdd,
                    socket_id: socket_id.clone(),
                    trip_id: Some(websocket_message.trip_id),
                })?,
            )
            .await?;
        tracing::info!("PubSubAction::TripAdd: message sent");
    } else {
        tracing::info!(
            "client: {} sent invalid payload - {} - {:?}",
            who,
            m,
            message_result.err()
        );
    }

    Ok(())
}

async fn handle_incoming(
    mut receiver: SplitStream<WebSocket>,
    who: SocketAddr,
    state: State<AppState>,
    socket_id: String,
) -> Result<(), anyhow::Error> {
    let redis = Arc::new(RwLock::new(
        state.redis.get_multiplexed_async_connection().await?,
    ));
    {
        let mut redis = redis.write().await;
        redis
            .publish(
                PUBSUB_CHANNEL_GENERAL_IN,
                serde_json::to_string(&PubSubMessage {
                    action: PubSubAction::Hello,
                    socket_id: socket_id.clone(),
                    trip_id: None,
                })?,
            )
            .await?;
        tracing::info!("PubSubAction::Hello: message sent");
    }

    loop {
        if let Some(message) = receiver.next().await {
            match message {
                Ok(m) => match m {
                    Message::Text(m) => {
                        handle_valid_message(socket_id.clone(), who, Arc::clone(&redis), m).await?;
                    }
                    Message::Close(_) => {
                        let mut redis = redis.write().await;
                        redis
                            .publish(
                                PUBSUB_CHANNEL_GENERAL_IN,
                                serde_json::to_string(&PubSubMessage {
                                    action: PubSubAction::Bye,
                                    socket_id: socket_id.clone(),
                                    trip_id: None,
                                })?,
                            )
                            .await?;
                    }
                    // https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers#pings_and_pongs_the_heartbeat_of_websockets
                    // You might also get a pong without ever sending a ping; ignore this if it happens.
                    e => tracing::info!("client: {} sent unhandled event {:#?}", who, e),
                },
                Err(e) => tracing::error!("client: {} has error {}", who, e),
            }
        } else {
            tracing::info!("client: {} has disconnected", who);
            break Ok(());
        }
    }
}
