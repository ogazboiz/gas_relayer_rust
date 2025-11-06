use config::config::{AppConfig, Configuration};
use db::db::DbState;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let configuration: Configuration = Configuration::load();
    let db: DbState = DbState::default(
        &configuration.database_url,
        configuration.max_db_connection as u32,
    )
    .await
    .expect("Couldn't create DB");

    let app_arc_config: AppConfig = Arc::new(configuration);
    routes::routes::start_app(app_arc_config, db)
        .await
        .expect("Couldn't start app");
}
