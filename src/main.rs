use std::net::SocketAddr;

mod bootstrap;
mod controllers;
mod router;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
     .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
     .init();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    let app = bootstrap::create_app().await;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}