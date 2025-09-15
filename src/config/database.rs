use std::str::FromStr;

use sqlx::{ MySqlPool, Pool, MySql };

pub type DbPool = Pool<MySql>;

pub async fn create_pool(database_url: &str) -> DbPool {
    MySqlPool::connect_with(
        sqlx::mysql::MySqlConnectOptions
            ::from_str(database_url)
            .expect("Failed to parse DATABASE_URL")
    ).await.expect("Failed to connect to MySQL")
}
