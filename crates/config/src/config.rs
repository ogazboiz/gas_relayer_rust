use serde::Deserialize;
use std::net::{Ipv6Addr, SocketAddr};
use std::sync::Arc;

#[derive(Deserialize, Clone, Debug)]
pub enum Environment {
    Local,
    Production,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Configuration {
    pub environment: Environment,
    pub database_url: String,
    pub app_port: u16,
    pub max_db_connection: u8,
    pub listening_addr: SocketAddr,
}

impl Configuration {
    pub fn load() -> Self {
        let environment = load_env_var("APP_ENVIRONMENT")
            .try_into()
            .expect("APP_ENVIRONMENT is not a valid environment");
        let port: u16 = load_env_var("APP_PORT")
            .parse::<u16>()
            .expect("APP_PORT is not a valid port");
        let database_url = load_env_var("DATABASE_URL")
            .try_into()
            .expect("DATABASE_URL is not set");
        let max_db_connection: u8 = load_env_var("MAX_DB_CONNECTION")
            .parse::<u8>()
            .expect("MAX_DB_CONNECTION is not a valid number");

        let listening_addr: SocketAddr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

        Self {
            environment,
            database_url,
            app_port: port,
            max_db_connection,
            listening_addr,
        }
    }
}

impl Environment {
    pub fn environment_as_string(&self) -> &'static str {
        match self {
            Environment::Local => "Local",
            Environment::Production => "Production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Local" => Ok(Environment::Local),
            "Production" => Ok(Environment::Production),
            _ => Err("Invalid environment"),
        }
    }
}

pub type AppConfig = Arc<Configuration>;
pub fn load_env_var(name: &str) -> String {
    std::env::var(name)
        .map_err(|e| format!("{name}: {e}"))
        .expect("Missing environment variable")
}
