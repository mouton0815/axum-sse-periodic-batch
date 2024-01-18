use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use axum::extract::State;
use axum::response::Sse;
use axum::response::sse::Event;
use axum::{BoxError, Router};
use axum::routing::get;
use axum_macros::debug_handler;
use futures::Stream;
use futures_util::StreamExt;
use log::info;
use tokio::net::TcpListener;
use crate::random_fetcher::RandomFetcher;
use crate::scheduled_stream::ScheduledStream;

pub struct SharedState {
    max_batch: u16,
    max_number: u16,
    repeat_secs: u64
}

impl SharedState {
    pub fn new(max_batch: u16, max_number: u16, repeat_secs: u64) -> Self {
        Self { max_batch, max_number, repeat_secs }
    }
}

#[debug_handler]
async fn sse_handler(State(state): State<Arc<SharedState>>) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("Connected");
    let fetcher = Box::new(RandomFetcher::new(state.max_batch, state.max_number));
    let stream = ScheduledStream::new(Duration::from_secs(state.repeat_secs), fetcher);
    let stream = stream.map(move |item| {
        Ok::<Event, Infallible>(Event::default().data(item))
    });
    Sse::new(stream)
}

pub async fn start_http_server(address: &str, state: SharedState) -> Result<(), BoxError>{
    info!("Start HTTP server");

    let router = Router::new()
        .route("/sse", get(sse_handler))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
