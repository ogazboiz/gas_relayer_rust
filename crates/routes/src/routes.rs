use crate::db_health_handler::db_health_handler;
use crate::metrics_handler::{metrics_handler, health_handler, readiness_handler, liveness_handler};
use crate::states::AppState;
use axum::{Router, middleware};
use axum::routing::get;
use config::config::Configuration;
use db::db::DbState;
use metrics::{MetricsCollector, metrics_middleware};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn api_router(app_state: AppState) -> Router {
    Router::new()
        .route("/db-health", get(db_health_handler))
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_handler))
        .route("/ready", get(readiness_handler))
        .route("/alive", get(liveness_handler))
        // Add metrics middleware to all routes
        .layer(middleware::from_fn_with_state(
            app_state.metrics.clone(),
            metrics_middleware,
        ))
        .with_state(app_state)
}

pub async fn start_app(config: Arc<Configuration>, db: DbState) -> anyhow::Result<()> {
    // Initialize tracing
    init_tracing()?;
    
    // Initialize metrics collector
    let metrics = MetricsCollector::new()?;
    
    let listening_addr = config.listening_addr;
    let app_state = AppState::new(db, config, metrics);
    let api_router = api_router(app_state);
    let listener = TcpListener::bind(listening_addr).await?;
    
    tracing::info!("Starting gas relayer server on {}", listening_addr);
    
    axum::serve(listener, api_router)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    
    Ok(())
}

fn init_tracing() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "gas_relayer=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();
    
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