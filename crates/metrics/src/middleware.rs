use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::MetricsCollector;

pub async fn metrics_middleware(
    State(metrics): State<MetricsCollector>,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    
    let response = next.run(request).await;
    
    let duration = start.elapsed().as_secs_f64();
    let status = response.status();
    
    // Record HTTP metrics
    record_http_request_metrics(&metrics, &method.to_string(), &path, status, duration);
    
    response
}

fn record_http_request_metrics(
    metrics: &MetricsCollector,
    method: &str,
    path: &str,
    status: StatusCode,
    duration: f64,
) {
    // For now, we'll use the existing RPC metrics as a placeholder
    // In a full implementation, you'd want dedicated HTTP request metrics
    metrics.rpc_requests_total.inc();
    metrics.rpc_latency.observe(duration);
    
    if status.is_server_error() || status.is_client_error() {
        metrics.rpc_errors_total.inc();
    }
    
    // Log the request for debugging
    tracing::info!(
        method = method,
        path = path,
        status = status.as_u16(),
        duration_ms = duration * 1000.0,
        "HTTP request processed"
    );
}