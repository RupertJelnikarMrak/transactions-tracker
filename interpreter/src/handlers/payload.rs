use crate::{AppContext, Error};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle(ctx: Arc<AppContext>, event: Value) -> Result<(), Error> {
    if let Some(transfers) = event.get("tokenTransfers").and_then(|t| t.as_array()) {
        for t in transfers {
            let mint = t["mint"].as_str().unwrap_or("");

            if mint != "7GdpaeSzvkx1a78rRkU11KstM1x8naMmMmmpWQnQSEAS" {
                continue;
            }
        }
    }

    Ok(())
}
