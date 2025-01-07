use std::env;

use diesel::{r2d2::ConnectionManager, SqliteConnection};
use dotenvy::dotenv;
use r2d2::{Pool, PooledConnection};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn create_db_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool")
}
