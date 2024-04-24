use axum::Router;
use protocol::tokio::net::TcpListener;
use std::net::SocketAddr;

pub async fn run_prometheus_server(prometheus_listening_address: SocketAddr) {
    let router = Router::new().route("/metrics", axum::routing::get(get_metrics));
    axum::serve(
        TcpListener::bind(prometheus_listening_address)
            .await
            .unwrap(),
        router,
    )
    .await
    .unwrap();
}

async fn get_metrics() -> String {
    let metrics_data = match crate::metrics::all_metrics() {
        Ok(data) => data,
        Err(e) => e.to_string().into_bytes(),
    };

    String::from_utf8(metrics_data).unwrap()
}
