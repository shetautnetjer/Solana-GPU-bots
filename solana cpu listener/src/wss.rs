// -------- src/wss.rs ------------------------------------------
use crate::types::LogRow;
use crossbeam::channel::Sender;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::time::SystemTime;
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Subscribe to a single token account and stream LogRow structs.
pub async fn subscribe_to_account(account: String, tx: Sender<LogRow>) {
    let url = "wss://api.mainnet-beta.solana.com";
    let (ws_stream, _) = match connect_async(url).await {
        Ok(v) => v,
        Err(e) => {
            eprintln!("WS connect error for {account}: {e}");
            return;
        }
    };
    let (mut write, mut read) = ws_stream.split();

    // Build subscription request
    let sub_msg = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "accountSubscribe",
        "params": [
            account,
            {
                "encoding": "jsonParsed",
                "commitment": "confirmed"
            }
        ]
    });
    let _ = write
        .send(Message::Text(sub_msg.to_string()))
        .await
        .map_err(|e| eprintln!("WS send error: {e}"));

    // Track previous uiAmount for delta calc
    let mut last_ui_amount: Option<f64> = None;

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(txt)) => {
                if let Ok(val) = serde_json::from_str::<serde_json::Value>(&txt) {
                    // Navigate JSON structure: result.value.data.parsed.info.tokenAmount.uiAmount
                    let ui_amount = val["result"]["value"]["data"]["parsed"]["info"]["tokenAmount"]["uiAmount"].as_f64();
                    let mint = val["result"]["value"]["data"]["parsed"]["info"]["mint"].as_str();
                    let owner = val["result"]["value"]["owner"].as_str();
                    let slot = val["context"]["slot"].as_u64();

                    if let (Some(amount), Some(mint_str), Some(owner_str)) = (ui_amount, mint, owner) {
                        let delta = match last_ui_amount {
                            Some(prev) => amount - prev,
                            None => 0.0,
                        };
                        last_ui_amount = Some(amount);

                        let ts = SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let row = LogRow {
                            timestamp: ts,
                            slot,
                            account: account.clone(),
                            mint: mint_str.to_string(),
                            owner: owner_str.to_string(),
                            ui_amount: amount,
                            delta,
                        };
                        let _ = tx.send(row);
                    }
                }
            }
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) => {}
            Ok(_) => {}
            Err(e) => {
                eprintln!("WS error on {account}: {e}");
                break;
            }
        }
    }
}