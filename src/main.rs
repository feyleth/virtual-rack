use std::net::SocketAddr;

use axum_server::tls_rustls::RustlsConfig;
use rcgen::{generate_simple_self_signed, CertifiedKey};
use tracing::trace;
use tracing_subscriber::{
    fmt::{self},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use virtual_rack::{
    pipewire::{create_pipewire_runner, state::State},
    server::{app, redirect_http_to_https, Ports},
};

async fn rustls_config() -> RustlsConfig {
    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];

    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();
    RustlsConfig::from_pem(
        cert.pem().into_bytes(),
        key_pair.serialize_pem().into_bytes(),
    )
    .await
    .expect("create certificate")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let format = fmt::layer().pretty().with_thread_names(true);
    tracing_subscriber::registry()
        .with(format)
        .with(EnvFilter::from_default_env())
        .init();
    trace!("start");

    let ports = Ports {
        http: 3000,
        https: 3001,
    };
    tokio::spawn(redirect_http_to_https(ports));
    let state = State::new();

    let pipewire = create_pipewire_runner(state.clone());
    let addr = SocketAddr::from(([127, 0, 0, 1], ports.https));
    tracing::debug!("listening on {}", addr);
    axum_server::bind_rustls(addr, rustls_config().await)
        .serve(app(state).into_make_service())
        .await
        .unwrap();
    // TODO: handle crash
    pipewire.join().expect("not to pannic")?;

    Ok(())
}
