use config::config::AppConfig;
use db::db::DbState;

#[derive(Clone)]
pub struct AppState {
    pub db: DbState,
    pub configuration: AppConfig,
}

impl AppState {
    pub fn new(db: DbState, configuration: AppConfig) -> Self {
        Self { db, configuration }
    }
}
