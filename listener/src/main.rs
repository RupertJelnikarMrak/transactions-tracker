use async_nats::jetstream::{self, stream};
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::post,
};
use serde_json::Value;
use std::sync::Arc;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

struct AppState {
    js: jetstream::Context,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let client = async_nats::connect("nats://127.0.0.1:4222").await?;
    let js = jetstream::new(client);

    let _stream = js
        .get_or_create_stream(stream::Config {
            name: "SOLANA_EVENTS".to_string(),
            subjects: vec!["solana.webhooks".to_string()],
            max_age: std::time::Duration::from_secs(60 * 60 * 24 * 7), // 7 days
            storage: stream::StorageType::File,
            ..Default::default()
        })
        .await?;

    println!("JetStream 'SOLANA_EVENTS' is ready.");

    let state = Arc::new(AppState { js });

    let app = Router::new()
        .route("/webhooks", post(handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listener active on port 3000");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(State(state): State<Arc<AppState>>, Json(payload): Json<Value>) -> StatusCode {
    // TODO: Implement an authentication header check
    let payload_bytes = serde_json::to_vec(&payload).unwrap_or_default();

    println!("A webhook has been recieved!");

    let result = state
        .js
        .publish("solana.webhooks", payload_bytes.clone().into())
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            eprintln!(
                "Failed to publish to NATS! Writing to fallback file. Error: {}",
                e
            );
            let file_result = OpenOptions::new()
                .create(true)
                .append(true)
                .open("failed_transaction.jsonl")
                .await;

            if let Ok(mut file) = file_result {
                if let Err(io_err) = file.write_all(&payload_bytes).await {
                    eprintln!("CRITICAL: Failed to write to fallback file: {}", io_err);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
                let _ = file.write_all(b"\n").await;

                return StatusCode::OK;
            }

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
