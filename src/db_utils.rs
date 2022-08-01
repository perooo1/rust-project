use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use std::env;
use std::sync::Arc;


/* 
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct StaticData {
    pub db: DbPool,
}

#[derive(Clone)]
pub struct AppState {
    pub static_data: Arc<StaticData>,
}

impl AppState {
    pub fn get_connection(&self) -> DbConnection {
        self.static_data
            .db
            .get()
            .expect("Error retrieving DB connection from pool")
    }
}

pub fn initialize() -> AppState {
    let pool = establish_pool_connection();
    AppState {
        static_data: Arc::new(StaticData { db: pool }),
    }
}
*/
///////////////////////////////////////////////old implementation down below ////////////////////////////

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
