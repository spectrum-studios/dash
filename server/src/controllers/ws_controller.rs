use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{ Message, Utf8Bytes, WebSocket, WebSocketUpgrade };
use axum::response::IntoResponse;
use axum::routing::get;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::sync::broadcast;

use crate::strategies::auth_strategy::{ AuthRequestClaims, JWTClaims };
use crate::strategies::user_strategy::get_db_user_by_uuid;

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<Utf8Bytes>,
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    while let Some(Ok(auth)) = receiver.next().await {
        if let Message::Text(text) = auth {
            if let Ok(claim) = AuthRequestClaims::from_string(&text) {
                username = get_db_user_by_uuid(claim.sub.clone()).await.unwrap().username;
                break;
            } else {
                sender.close().await.unwrap();
                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();
    let msg = format!("{} joined", username).into();
    let _ = state.tx.send(msg);
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg == "" {
                break;
            }
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if text == "" {
                break;
            }
            let _ = tx.send(format!("{}: {}", name, text).into());
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    let msg = format!("{} left", username).into();
    let _ = state.tx.send(msg);

    state.user_set.lock().unwrap().remove(&username);
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

pub fn routes() -> Router {
    let user_set = Mutex::new(HashSet::new());
    let (tx, _) = broadcast::channel(100);
    let app_state = Arc::new(AppState { user_set, tx });
    Router::new().route("/", get(ws_handler)).with_state(app_state)
}
