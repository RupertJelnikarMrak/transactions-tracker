use crate::{AppContext, Error};
use serde_json::Value;
use std::sync::Arc;

pub async fn handle(ctx: Arc<AppContext>, event: Value) -> Result<(), Error> {
    Ok(())
}
