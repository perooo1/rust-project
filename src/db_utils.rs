use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

//pool

pub fn establish_pool_connection() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .expect("Error creating database connection pool")
}
