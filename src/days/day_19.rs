use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::broadcast::Sender;
use tokio::sync::{broadcast, Mutex};

#[derive(Default, Debug)]
struct Day19 {
    broadcaster: Mutex<HashMap<u16, Sender<Arc<Tweet>>>>,
    tweet_views: Mutex<usize>,
}

type Day19State = State<Arc<Day19>>;

pub(super) fn route() -> Router {
    Router::new()
        .route("/ws/ping", get(ping))
        .route("/ws/room/:room_number/user/:username", get(rooms))
        .route("/reset", post(reset))
        .route("/views", get(views))
        .with_state(Arc::new(Day19::default()))
}

#[tracing::instrument(ret)]
async fn ping(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |mut socket| async move {
        let mut started = false;
        while let Some(Ok(message)) = socket.recv().await {
            tracing::debug!("Received message: {:?}", message);
            match message {
                Message::Text(text) => {
                    if started && text == "ping" {
                        tracing::debug!("Sending pong back to client as game started");
                        socket
                            .send(Message::Text("pong".to_string()))
                            .await
                            .unwrap();
                    }
                    if text == "serve" {
                        started = true;
                    }
                }
                Message::Binary(bin) => {
                    tracing::debug!("Received binary: {:?}", bin);
                }
                Message::Ping(_) => {
                    tracing::debug!("Received ping");
                }
                Message::Pong(_) => {
                    tracing::debug!("Received pong");
                }
                Message::Close(_) => break,
            }
        }
    })
}

#[tracing::instrument(ret)]
async fn views(State(chat): Day19State) -> String {
    chat.tweet_views.lock().await.to_string()
}

#[tracing::instrument(ret)]
async fn reset(State(chat): Day19State) {
    *chat.tweet_views.lock().await = 0;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Tweet {
    #[serde(default)]
    user: String,
    message: String,
}

#[tracing::instrument(skip(chat))]
async fn rooms(
    State(chat): Day19State,
    Path((room_number, username)): Path<(u16, String)>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| async move {
        tracing::info!(
            message = "Connection established",
            room_number = room_number,
            username = username
        );
        let tx = chat
            .broadcaster
            .lock()
            .await
            .entry(room_number)
            .or_insert_with(|| broadcast::channel(1024).0)
            .clone();
        let mut rv = tx.subscribe();
        let (mut sender, mut receiver) = socket.split();
        let mut send_task = tokio::spawn(async move {
            while let Ok(tweet) = rv.recv().await {
                if let Err(e) = sender
                    .send(Message::Text(serde_json::to_string(&*tweet).unwrap()))
                    .await
                {
                    tracing::error!("Error sending tweet: {e:?}");
                }
                *chat.tweet_views.lock().await += 1;
            }
        });
        let current_user = username.clone();
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                tracing::debug!("Received message: {:?}", message);
                match message {
                    Message::Text(text) => match serde_json::from_str::<Tweet>(&text) {
                        Ok(mut tweet) => {
                            tweet.user = current_user.clone();
                            let tweet = Arc::new(tweet);
                            if tweet.message.len() <= 128 && tx.send(tweet.clone()).is_err() {
                                tracing::warn!("tweet {tweet:?} could not be sent");
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error parsing tweet: {:?}", e);
                        }
                    },
                    Message::Binary(bin) => {
                        tracing::debug!("Received binary: {:?}", bin);
                    }
                    Message::Ping(_) => {
                        tracing::debug!("Received ping");
                    }
                    Message::Pong(_) => {
                        tracing::debug!("Received pong");
                    }
                    Message::Close(_) => break,
                }
            }
        });

        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        }
        tracing::info!(
            message = "Connection Dropped",
            room_number = room_number,
            username = username,
        );
    })
}
