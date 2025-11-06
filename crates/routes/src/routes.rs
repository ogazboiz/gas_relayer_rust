use crate::db_health_handler::db_health_handler;
use crate::states::AppState;
use axum::Router;
use axum::routing::get;
use config::config::Configuration;
use db::db::DbState;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .route("/db-health", get(db_health_handler))
        // .merge("")
        .with_state(app_state)
}

pub async fn start_app(config: Arc<Configuration>, db: DbState) -> anyhow::Result<()> {
    let listening_addr = config.listening_addr;
    let app_state = AppState::new(db, config);
    let api_router = api_router(app_state);
    let listener = TcpListener::bind(listening_addr).await?;
    // axum::serve(listener, api_router).with_graceful_shutdown(shutdown_signal()).await?;
    axum::serve(listener, api_router).await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to configure ctrl+c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to configure SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {_ = ctrl_c => {}, _ = terminate => {},}
}
