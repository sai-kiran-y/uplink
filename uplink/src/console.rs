use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Router};
use log::info;
use uplink::base::bridge::BridgeTx;

use crate::ReloadHandle;

#[derive(Debug, Clone)]
struct StateHandle {
    reload_handle: ReloadHandle,
    bridge_handle: BridgeTx,
}

#[tokio::main]
pub async fn start(port: u16, reload_handle: ReloadHandle, bridge_handle: BridgeTx) {
    let address = format!("0.0.0.0:{port}");
    info!("Starting uplink console server: {address}");
    let state = StateHandle { reload_handle, bridge_handle };
    let app = Router::new()
        .route("/logs", post(reload_loglevel))
        .route("/shutdown", post(shutdown))
        .with_state(state);

    axum::Server::bind(&address.parse().unwrap()).serve(app.into_make_service()).await.unwrap();
}

async fn reload_loglevel(State(state): State<StateHandle>, filter: String) -> impl IntoResponse {
    info!("Reloading tracing filter: {filter}");
    if state.reload_handle.reload(&filter).is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn shutdown(State(state): State<StateHandle>) -> impl IntoResponse {
    info!("Shutting down uplink");
    state.bridge_handle.trigger_shutdown().await;

    StatusCode::OK
}