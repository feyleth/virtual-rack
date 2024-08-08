use std::{convert::Infallible, error::Error, time::Duration};

use async_stream::stream;
use axum::{
    extract,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use futures::{
    future::select,
    stream::{self, Stream},
    TryStreamExt,
};
use tokio_stream::{wrappers::errors::BroadcastStreamRecvError, StreamExt as _};
use tower_http::trace::TraceLayer;

use crate::pipewire::state::State;

pub fn app(state: State) -> Router {
    Router::new()
        .route("/state", get(sse_handler))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn sse_handler(
    extract::State(state): extract::State<State>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let (state, events) = state.subscribe();
    let mut stream = tokio_stream::wrappers::BroadcastStream::new(events)
        .map(|event| event.map(|event| Event::default().id("event").data(format!("{:?}", event))));

    let stream = stream! {
        yield Ok(Event::default().id("init").data(format!("{:?}", state)));

        while let Some(value) = stream.next().await{
            yield value;
        }
    };
    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new())
}
