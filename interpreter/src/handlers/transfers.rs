use crate::{AppContext, Error};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle(ctx: Arc<AppContext>, event: Value) -> Result<(), Error> {
    let signature = event
        .get("signature")
        .and_then(|s| s.as_str())
        .unwrap_or_default();
    let slot = event.get("slot").and_then(|s| s.as_u64()).unwrap_or(0);

    if let Some(transfers) = event.get("tokenTransfers").and_then(|t| t.as_array()) {
        for transfer in transfers {
            let amount = transfer
                .get("tokenAmount")
                .and_then(|a| a.as_f64())
                .unwrap_or(0.0);
            let mint = transfer.get("mint").and_then(|s| s.as_str()).unwrap_or("");
            let user = transfer
                .get("toUserAccount")
                .and_then(|s| s.as_str())
                .unwrap_or("");

            ctx.db
                .insert_transaction(signature, slot, user, mint, amount)
                .await?;
        }
    }

    Ok(())
}
