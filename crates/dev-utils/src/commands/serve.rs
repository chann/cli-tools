use anyhow::Result;
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn run(path: String, port: u16) -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    let app = Router::new().nest_service("/", ServeDir::new(&path));

    info!("Serving {} on http://{}", path, addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
