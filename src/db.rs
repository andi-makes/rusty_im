pub mod manufacturer;
pub mod part;
pub mod tag;

pub mod migration;
pub mod schema;

pub use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).unwrap()
}
