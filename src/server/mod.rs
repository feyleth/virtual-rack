pub mod events;

use axum::{
    extract,
    response::{sse::Event, Sse},
    routing::get,
    Router,
};
use events::{
    node::{LinkEvent, LinkValue, NodeEvent, NodeValue},
    state::StateValue,
};
use futures::stream::Stream;
use tokio::sync;
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
    let (prod, recv) = sync::mpsc::channel(50);

    let (state, events) = state.subscribe();

    let _ = prod
        .send(Ok(Event::default().id("init state").data(
            serde_json::to_string::<StateValue>(&state.into()).unwrap(),
        )))
        .await;

    tokio::task::spawn(async move {
        let mut stream = tokio_stream::wrappers::BroadcastStream::new(events).map(|event| {
            event.map(|event| {
                let (id, value) = match event {
                    crate::pipewire::state::StateChangeEvent::AddNode(node) => {
                        let (init, events) = node.subcribe();
                        let prod_clone = prod.clone();
                        tokio::task::spawn(async move {
                            let mut stream = tokio_stream::wrappers::BroadcastStream::new(events)
                                .map(|event| {
                                    event.map(|event| {
                                        Event::default().id("node event").data(
                                            serde_json::to_string(&NodeEvent {
                                                id: init.id,
                                                event: event.into(),
                                            })
                                            .unwrap(),
                                        )
                                    })
                                });
                            while let Some(value) = stream.next().await {
                                let _ = prod_clone.send(value).await;
                            }
                        });
                        ("add node", serde_json::to_string::<NodeValue>(&init.into()))
                    }
                    crate::pipewire::state::StateChangeEvent::AddLink(link) => {
                        let (init, events) = link.subcribe();
                        let prod_clone = prod.clone();
                        tokio::task::spawn(async move {
                            let mut stream = tokio_stream::wrappers::BroadcastStream::new(events)
                                .map(|event| {
                                    event.map(|event| {
                                        Event::default().id("link event").data(
                                            serde_json::to_string(&LinkEvent {
                                                id: init.id,
                                                event: event.into(),
                                            })
                                            .unwrap(),
                                        )
                                    })
                                });
                            while let Some(value) = stream.next().await {
                                let _ = prod_clone.send(value).await;
                            }
                        });
                        ("add link", serde_json::to_string::<LinkValue>(&init.into()))
                    }
                };
                Event::default().id(id).data(value.unwrap())
            })
        });
        while let Some(value) = stream.next().await {
            let _ = prod.send(value).await;
        }
    });

    Sse::new(tokio_stream::wrappers::ReceiverStream::new(recv))
        .keep_alive(axum::response::sse::KeepAlive::new())
}
