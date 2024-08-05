use tracing::{error, trace};
use tracing_subscriber::{
    fmt::{self, format},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use virtual_rack::pipewire::create_pipewire_runner;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let format = fmt::layer().pretty().with_thread_names(true);
    tracing_subscriber::registry()
        .with(format)
        .with(EnvFilter::from_default_env())
        .init();
    trace!("start");
    create_pipewire_runner().join().expect("not to pannic")?;

    Ok(())
}
