mod handlers;
mod router;
mod storage;

use async_nats::jetstream::{
    self,
    consumer::{Consumer, pull},
};
use dotenvy::dotenv;
use futures::StreamExt;
use std::env;
use std::sync::Arc;
use storage::postgres::PgRepo;

pub struct AppContext {
    pub db: PgRepo,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let nats = async_nats::connect("nats://nats:4222").await?;
    let js = jetstream::new(nats);

    let db = PgRepo::new(
        env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set.")
            .as_str(),
    )
    .await?;

    let ctx = Arc::new(AppContext { db });

    let consumer: Consumer<pull::Config> = js
        .get_stream("SOLANA_EVENTS")
        .await?
        .get_consumer("interpreter_worker")
        .await?;

    tracing::info!("Interpreter started. Waiting for events...");

    let mut messages = consumer.messages().await?;
    while let Some(Ok(msg)) = messages.next().await {
        let payload: serde_json::Value = match serde_json::from_slice(&msg.payload) {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Malformed JSON: {}", e);
                msg.ack().await?;
                continue;
            }
        };

        if let Err(e) = router::route_event(ctx.clone(), payload).await {
            tracing::error!("Processing failed: {}", e);
        } else {
            msg.ack().await?;
        }
    }

    Ok(())
}
