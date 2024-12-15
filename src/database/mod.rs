use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use nject::injectable;
use std::env;

pub mod schema;


    // Provide the Database connection
    injector.provide::<Database>().with_factory(|_| {
        let manager: ConnectionManager<SqliteConnection> =
            ConnectionManager::new("db/pkm.sqlite");
        let pool: Pool<ConnectionManager<SqliteConnection>> = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");
        Database::new(pool.get().expect("Failed to get DB connection"))
    });

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("Error connecting to database")
}

pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[injectable]
#[inject(Pool::builder().build(establish_connection()))]
pub struct Database {
    pub connection: DbConnection,
}

impl Database {
    pub fn new(connection: DbConnection) -> Self {
        Self { connection }
    }
}
