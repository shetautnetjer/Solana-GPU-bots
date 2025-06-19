// wss.rs
use crossbeam::channel::Sender;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::time::SystemTime;
tokio_tungstenite::connect_async;

pub async fn subscribe_to_account(account: String, tx: Sender<String>) {
    let url = "wss://api.mainnet-beta.solana.com";
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let sub_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "accountSubscribe",
        "params": [
            account,
            {"encoding": "jsonParsed", "commitment": "confirmed"}
        ]
    });

    write.send(tokio_tungstenite::tungstenite::Message::Text(sub_msg.to_string()))
        .await
        .unwrap();

    while let Some(msg) = read.next().await {
        if let Ok(tokio_tungstenite::tungstenite::Message::Text(text)) = msg {
            let timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let log_line = format!("{},{},{}", timestamp, account, text);
            let _ = tx.send(log_line);
        }
    }
}