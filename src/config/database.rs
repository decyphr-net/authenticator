use crate::config::entities::DatabaseConfig;

impl DatabaseConfig {
    pub fn init() -> DatabaseConfig {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let max_connections = std::env::var("MAX_CONNECTIONS").expect("MAX_CONNECTIONS must be set");

        DatabaseConfig {
            database_url,
            max_connections: max_connections.parse::<u32>().unwrap(),
        }
    }
}