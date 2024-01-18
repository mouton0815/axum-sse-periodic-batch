use axum::BoxError;
use crate::http_server::{SharedState, start_http_server};

mod http_server;
mod scheduled_stream;
mod random_fetcher;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    env_logger::init();
    let state = SharedState::new(10, 10, 3);
    start_http_server("localhost:3000", state).await
}
