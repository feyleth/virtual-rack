use std::{thread::sleep, time::Duration};

use tokio::{runtime::Builder, sync::broadcast, task};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::{info, trace};
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use virtual_rack::pipewire::{create_pipewire_runner, state::State};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building th Runtime");
    let format = fmt::layer().pretty().with_thread_names(true);
    tracing_subscriber::registry()
        .with(format)
        .with(EnvFilter::from_default_env())
        .init();
    trace!("start");

    let state = State::new();
    let state_clone = state.clone();
    runtime.spawn(async move {
        sleep(Duration::from_secs(2));
        let (state, mut events) = state_clone.subscribe();
        info!("start state {:#?}", state);
        loop {
            let change = events.recv().await;
            match change {
                Ok(node_event) => match node_event {
                    virtual_rack::pipewire::state::StateChangeEvent::AddNode(node) => {
                        let (node, events) = node.subcribe();
                        let mut events = BroadcastStream::new(events);
                        task::spawn(async move {
                            info!("new node {:#?}", node);
                            let id = node.id;
                            while let Some(event) = events.next().await {
                                info!("node {} event {:#?}", id, event);
                            }
                        });
                    }
                    virtual_rack::pipewire::state::StateChangeEvent::AddLink(link) => {
                        let (link, events) = link.subcribe();
                        let mut events = BroadcastStream::new(events);
                        task::spawn(async move {
                            info!("new link {:#?}", link);
                            let id = link.id;
                            while let Some(event) = events.next().await {
                                info!("link {} event {:#?}", id, event);
                            }
                        });
                    }
                },
                Err(e) => match e {
                    broadcast::error::RecvError::Closed => break,
                    broadcast::error::RecvError::Lagged(_) => (),
                },
            }
        }
    });
    create_pipewire_runner(state)
        .join()
        .expect("not to pannic")?;

    Ok(())
}
