pub mod events;

use std::{env, net::SocketAddr};

use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
    BoxError, Router,
};
use events::sse_hanler;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

use crate::pipewire::state::State;

pub fn app(state: State) -> Router {
    let static_dir = env::var("STATIC_FILES").unwrap_or_else(|_| "./static".to_owned());
    info!("static file directory {}", static_dir);
    let serve_dir = ServeDir::new(static_dir.clone())
        .not_found_service(ServeFile::new(format!("{}/index.html", static_dir)));
    Router::new()
        .nest(
            "/api",
            Router::new()
                .route("/state", get(sse_hanler))
                .with_state(state)
                .layer(TraceLayer::new_for_http()),
        )
        .fallback_service(serve_dir)
}

#[derive(Clone, Copy)]
pub struct Ports {
    pub http: u16,
    pub https: u16,
}
pub async fn redirect_http_to_https(ports: Ports) {
    fn make_https(host: String, uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.http.to_string(), &ports.https.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], ports.http));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
