use crate::handlers::{swaps, transfers};
use crate::{AppContext, Error};
use serde_json::Value;
use std::sync::Arc;

pub async fn route_event(ctx: Arc<AppContext>, event: Value) -> Result<(), Error> {
    let event_type = event
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("UNKNOWN");

    match event_type {
        "TRANSFER" | "NFT_SALE" => transfers::handle(ctx, event).await,
        "SWAP" => swaps::handle(ctx, event).await,
        "COMPRESSED_NFT_MINT" => {
            println!("Ignoring compressed mint");
            Ok(())
        }
        _ => {
            println!("Unknown event type: {}", event_type);
            Ok(())
        }
    }
}
