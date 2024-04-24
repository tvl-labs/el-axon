mod types;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

use core_consensus::{SyncStatus, SYNC_STATUS};
use protocol::tokio::net::TcpListener;

use crate::types::{NodeVersion, ServiceStatus};

pub async fn run_el_node_api(url: &str) {
    let app = Router::new()
        .route("/eigen/node", get(node_version))
        .route("/eigen/node/health", get(health))
        .route("/eigen/nodes/services", get(all_service))
        .route("/eigen/node/:{service_ID}/service", get(all_service));

    axum::serve(TcpListener::bind(url).await.unwrap(), app)
        .await
        .unwrap();
}

async fn node_version() -> impl IntoResponse {
    Json(NodeVersion::default())
}

async fn health() -> StatusCode {
    let sync_state = SYNC_STATUS.read().clone();

    return match sync_state {
        SyncStatus::False => StatusCode::OK,
        _ => StatusCode::PARTIAL_CONTENT,
    };
}

async fn all_service() -> (StatusCode, Json<ServiceStatus>) {
    (StatusCode::OK, Json(ServiceStatus::default()))
}
