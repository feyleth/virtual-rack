use tracing::{info, trace};
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use virtual_rack::{
    pipewire::{create_pipewire_runner, state::State},
    server::app,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let format = fmt::layer().pretty().with_thread_names(true);
    tracing_subscriber::registry()
        .with(format)
        .with(EnvFilter::from_default_env())
        .init();
    trace!("start");

    let state = State::new();

    let pipewire = create_pipewire_runner(state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(state)).await.unwrap();
    // TODO: handle crash
    pipewire.join().expect("not to pannic")?;

    Ok(())
}
