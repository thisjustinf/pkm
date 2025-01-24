use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub mod schema;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct DatabasePool {
    pub pool: DbPool,
}

impl DatabasePool {
    /// Initialize the connection pool
    pub fn create_pool() -> Self {
        dotenv().ok();
        let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager: ConnectionManager<SqliteConnection> =
            ConnectionManager::<SqliteConnection>::new(database_url);
        let pool: DbPool = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");

        Self { pool }
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> Result<DbConnection, PoolError> {
        self.pool.get()
    }
}
