use config::config::Configuration;
use db::db::DbState;
use metrics::MetricsCollector;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub config: Arc<Configuration>,
    pub metrics: MetricsCollector,
}

impl AppState {
    pub fn new(db: DbState, config: Arc<Configuration>, metrics: MetricsCollector) -> Self {
        Self { db, config, metrics }
    }
}