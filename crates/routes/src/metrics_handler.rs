use crate::states::AppState;
use axum::{
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    Json,
};
use metrics::HealthChecker;

pub async fn metrics_handler(State(app_state): State<AppState>) -> impl IntoResponse {
    match app_state.metrics.export_metrics() {
        Ok(metrics_output) => {
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/plain; version=0.0.4; charset=utf-8")
                .body(metrics_output)
                .unwrap()
        }
        Err(e) => {
            tracing::error!("Failed to export metrics: {}", e);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to export metrics".to_string())
                .unwrap()
        }
    }
}

pub async fn health_handler(State(_app_state): State<AppState>) -> impl IntoResponse {
    let health_check = HealthChecker::new();
    let system_health = health_check.check_system_health().await;
    
    let status_code = match system_health.overall_status {
        metrics::HealthStatus::Healthy => StatusCode::OK,
        metrics::HealthStatus::Degraded => StatusCode::OK,
        metrics::HealthStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
    };
    
    (status_code, Json(system_health)).into_response()
}

pub async fn readiness_handler(State(app_state): State<AppState>) -> impl IntoResponse {
    match db::db::DbState::ping_db(&app_state.db.pool).await {
        Ok(_) => {
            Json(serde_json::json!({
                "status": "ready",
                "timestamp": chrono::Utc::now()
            })).into_response()
        }
        Err(e) => {
            tracing::error!("Readiness check failed: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({
                    "status": "not_ready",
                    "error": "Database connection failed",
                    "timestamp": chrono::Utc::now()
                }))
            ).into_response()
        }
    }
}

pub async fn liveness_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "alive",
        "timestamp": chrono::Utc::now()
    })).into_response()
}