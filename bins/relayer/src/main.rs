use std::sync::Arc;
use config::config::AppConfig;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let configuration = config::config::Configuration::load();

    let db = db::db::DbState::default(&configuration.database_url, configuration.max_db_connection as u32).await.expect("Couldn't create DB");
    let app_arc_config: AppConfig = Arc::new(configuration);
    routes::routes::start_app(app_arc_config, db).await.expect("Couldn't start app");
}
