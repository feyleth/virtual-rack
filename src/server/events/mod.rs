use axum::{
    extract,
    response::{sse::Event, Sse},
};
use futures::{Stream, StreamExt};
use state::StateValue;
use tokio::sync::{self, mpsc::Sender};
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

use crate::pipewire::state::State;

pub mod node;
pub mod state;

async fn send_update(prod: Sender<Result<Event, BroadcastStreamRecvError>>, state_handler: State) {
    let _ = prod
        .send(Ok(Event::default().event("change state").data(
            serde_json::to_string::<StateValue>(&state_handler.get().into()).unwrap(),
        )))
        .await;
}

// TODO: maybe desync, future implement other method
pub async fn sse_hanler(
    extract::State(state_handler): extract::State<State>,
) -> Sse<impl Stream<Item = Result<Event, BroadcastStreamRecvError>>> {
    let (prod, recv) = sync::mpsc::channel(50);

    let (state, events) = state_handler.subscribe();

    let clone_state = state.clone();
    let _ = prod
        .send(Ok(Event::default().event("init state").data(
            serde_json::to_string::<StateValue>(&clone_state.into()).unwrap(),
        )))
        .await;

    let clone_state_handler = state_handler.clone();
    let clone_prod = prod.clone();
    state.nodes.iter().for_each(move |node| {
        let (_, events) = node.1.subcribe();
        let state_handler = clone_state_handler.clone();
        let prod = clone_prod.clone();
        tokio::task::spawn(async move {
            tokio_stream::wrappers::BroadcastStream::new(events)
                .for_each(|_| send_update(prod.clone(), state_handler.clone()))
                .await;
        });
    });

    let clone_state_handler = state_handler.clone();
    let clone_prod = prod.clone();
    state.links.iter().for_each(move |link| {
        let (_, events) = link.1.subcribe();
        let state_handler = clone_state_handler.clone();
        let prod = clone_prod.clone();
        tokio::task::spawn(async move {
            tokio_stream::wrappers::BroadcastStream::new(events)
                .for_each(|_| send_update(prod.clone(), state_handler.clone()))
                .await;
        });
    });

    tokio::task::spawn(async move {
        let _ = tokio_stream::wrappers::BroadcastStream::new(events)
            .for_each(move |event| {
                let clone_state_handler = state_handler.clone();
                let clone_prod = prod.clone();
                match event {
                    Ok(event) => match event {
                        crate::pipewire::state::StateChangeEvent::AddNode(node) => {
                            let (_, events) = node.subcribe();
                            let state_handler = clone_state_handler.clone();
                            let prod = clone_prod.clone();
                            tokio::task::spawn(async move {
                                tokio_stream::wrappers::BroadcastStream::new(events)
                                    .for_each(|_| send_update(prod.clone(), state_handler.clone()))
                                    .await;
                            });
                        }
                        crate::pipewire::state::StateChangeEvent::AddLink(link) => {
                            let (_, events) = link.subcribe();
                            let state_handler = clone_state_handler.clone();
                            let prod = clone_prod.clone();
                            tokio::task::spawn(async move {
                                tokio_stream::wrappers::BroadcastStream::new(events)
                                    .for_each(|_| send_update(prod.clone(), state_handler.clone()))
                                    .await;
                            });
                        }
                    },
                    Err(_) => (),
                }
                send_update(prod.clone(), state_handler.clone())
            })
            .await;
    });

    Sse::new(tokio_stream::wrappers::ReceiverStream::new(recv))
        .keep_alive(axum::response::sse::KeepAlive::new())
}
