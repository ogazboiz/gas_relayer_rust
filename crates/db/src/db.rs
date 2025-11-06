use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// use config;

static MIGRANT: Migrator = sqlx::migrate!("./migrations");

#[derive(Clone, Debug)]
pub struct DbState {
    pub pool: Pool<Postgres>,
}

impl DbState {
    pub async fn default(db_url: &str, max_connection: u32) -> anyhow::Result<Self> {
        // let db_url = config::config::load_env_var("DATABASE_URL"); // loads the database url from the environment variable

        let connection_pool = PgPoolOptions::new()
            .max_connections(max_connection)
            .connect(db_url)
            .await?;

        MIGRANT.run(&connection_pool).await?;

        Ok(Self {
            pool: connection_pool,
        })
    }

    pub async fn ping_db(pool: &Pool<Postgres>) -> anyhow::Result<()> {
        sqlx::query("SELECT 1").execute(pool).await?;
        Ok(())
    }
}
