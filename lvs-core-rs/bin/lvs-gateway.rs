use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::stream::StreamExt;
use futures::SinkExt; 
use tokio::{net::TcpListener, sync::broadcast};

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    // Канал для широковещательной рассылки сообщений между всеми подключёнными нодами
    let (tx, _rx) = broadcast::channel::<String>(1024);
    let state = AppState { tx };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:9001")
        .await
        .expect("Failed to bind port");

    println!("LVS Gateway running on ws://127.0.0.1:9001/ws");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C handler");
    println!("Shutting down LVS Gateway...");
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("[Gateway] Incoming WS upgrade");
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Один WebSocket-клиент (нода).
/// Читаем от него сообщения и ретранслируем их всем остальным через broadcast-канал.
async fn handle_socket(socket: WebSocket, state: AppState) {
    println!("[Gateway] Node websocket opened");

    let (mut sender, mut receiver) = socket.split();

    // Подписываемся на общий канал, чтобы получать сообщения от других нод
    let mut rx = state.tx.subscribe();

    // Задача-писатель: всё, что приходит по broadcast, отправляем в этот сокет
    let writer = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
        println!("[Gateway] Writer task finished");
    });

    let tx = state.tx.clone();

    // Основной цикл чтения из этого сокета
    while let Some(Ok(msg)) = receiver.next().await {
        match msg {
            Message::Text(txt) => {
                // На входе уже есть поле "node": "node-1" / "node-2" и т.д.
                println!("[<<] {txt}");

                // Оборачиваем сообщение, помечая как relay-пакет
                let wrapped = format!(r#"{{"relay":true,"payload":{txt}}}"#);

                // Шлём всем подписчикам (включая отправителя)
                let _ = tx.send(wrapped);
            }
            Message::Close(_) => {
                println!("[Gateway] Node disconnected");
                break;
            }
            _ => {}
        }
    }

    writer.abort();
    println!("[Gateway] Node handler finished");
}
