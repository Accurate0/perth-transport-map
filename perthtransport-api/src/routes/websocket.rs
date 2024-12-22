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
use perthtransport::{
    constants::{PUBSUB_CHANNEL_GENERAL_IN, PUBSUB_CHANNEL_OUT_PREFIX},
    queue::MessageBus,
    types::message::{PubSubAction, PubSubMessage, WebSocketMessage},
};
use std::net::SocketAddr;
use tracing::{Instrument, Level};

pub async fn handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<AppState>,
) -> impl IntoResponse {
    tracing::info!("client: {} has connected", addr);
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
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
    let channel = format!("{}_{}", PUBSUB_CHANNEL_OUT_PREFIX, socket_id);
    let mut pubsub = state.message_bus.subscribe(&[&channel]).await?;
    tracing::info!("we're subscribed, sending hello");
    state
        .message_bus
        .publish(
            PUBSUB_CHANNEL_GENERAL_IN,
            PubSubMessage {
                action: PubSubAction::Hello,
                socket_id: socket_id.clone(),
                trip_id: None,
            },
        )
        .await?;

    tracing::info!("subscribed: {}", channel);

    while let Some(msg) = pubsub.on_message().next().await {
        let payload = msg.get_payload()?;
        let _ = sender.send(Message::Text(payload)).await;
    }

    Ok(())
}

async fn handle_valid_message(
    _socket_id: String,
    who: SocketAddr,
    _message_bus: MessageBus,
    m: String,
) -> Result<(), anyhow::Error> {
    let message_result = serde_json::from_str::<WebSocketMessage>(&m);
    match message_result {
        Ok(websocket_message) => match websocket_message {},
        Err(_) => {
            tracing::info!(
                "client: {} sent invalid payload - {} - {:?}",
                who,
                m,
                message_result.err()
            );
        }
    }

    Ok(())
}

async fn handle_incoming(
    mut receiver: SplitStream<WebSocket>,
    who: SocketAddr,
    state: State<AppState>,
    socket_id: String,
) -> Result<(), anyhow::Error> {
    while let Some(message) = receiver.next().await {
        match message {
            Ok(m) => match m {
                Message::Text(m) => {
                    handle_valid_message(socket_id.clone(), who, state.message_bus.clone(), m)
                        .await?;
                }
                Message::Close(_) => {
                    state
                        .message_bus
                        .publish(
                            PUBSUB_CHANNEL_GENERAL_IN,
                            PubSubMessage {
                                action: PubSubAction::Bye,
                                socket_id: socket_id.clone(),
                                trip_id: None,
                            },
                        )
                        .await?;
                }
                // https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers#pings_and_pongs_the_heartbeat_of_websockets
                // You might also get a pong without ever sending a ping; ignore this if it happens.
                e => tracing::info!("client: {} sent unhandled event {:#?}", who, e),
            },
            Err(e) => tracing::error!("client: {} has error {}", who, e),
        }
    }

    tracing::info!("client: {} has disconnected", who);
    Ok(())
}
