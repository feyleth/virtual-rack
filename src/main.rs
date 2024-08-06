use tokio::{runtime::Builder, sync::broadcast, task};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::{info, trace};
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use virtual_rack::pipewire::create_pipewire_runner;

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
    let (state_broadcast, mut changes) = broadcast::channel(50);

    runtime.spawn(async move {
        loop {
            let change = changes.recv().await;
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
                },
                Err(e) => match e {
                    broadcast::error::RecvError::Closed => break,
                    broadcast::error::RecvError::Lagged(_) => (),
                },
            }
        }
    });
    create_pipewire_runner(state_broadcast)
        .join()
        .expect("not to pannic")?;

    Ok(())
}
