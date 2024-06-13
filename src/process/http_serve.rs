use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tracing::info;

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}
pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("http serve: {:?} {:?}", path, port);
    let state = HttpServeState { path };
    // axum router
    let router = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("path: {:?}", p);
    if !p.exists() {
        (StatusCode::NOT_FOUND, format!("{} not found", p.display()))
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => (StatusCode::OK, content),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("read file error: {}", e),
            ),
        }
    }
}
