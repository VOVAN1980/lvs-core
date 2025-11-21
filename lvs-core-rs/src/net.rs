use anyhow::Result;
use futures::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    WebSocketStream,
    MaybeTlsStream,
};
use tokio::net::TcpStream;

// --------------- ВАЖНО ---------------------
// Тип должен соответствовать реальному WebSocketStream:
//   WebSocketStream<MaybeTlsStream<TcpStream>>
// --------------------------------------------

lazy_static! {
    static ref WS_WRITER: Mutex<Option<
        futures::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            Message
        >
    >> = Mutex::new(None);
}

pub async fn connect_node(url: &str) -> Result<()> {
    let (ws_stream, _) = connect_async(url).await?;
    println!("[WS] Connected to {}", url);

    let (write, mut read) = ws_stream.split();

    {
        let mut w = WS_WRITER.lock().await;
        *w = Some(write);
    }

    // приём сообщений
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            if let Message::Text(txt) = msg {
                println!("[WS <<] {}", txt);
            }
        }
    });

    Ok(())
}

pub async fn send(msg: String) -> Result<()> {
    let mut writer = WS_WRITER.lock().await;

    if let Some(ref mut w) = *writer {
        w.send(Message::Text(msg)).await?;
    }

    Ok(())
}
