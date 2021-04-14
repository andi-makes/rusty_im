pub mod model;
pub mod schema;

pub use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use schema::*;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap();
    PgConnection::establish(&database_url).unwrap()
}

pub fn insert_manufacturer(con: &PgConnection, name: String) {
    let manufacturer = model::NewManufacturer { name };
    diesel::insert_into(manufacturers::table)
        .values(&manufacturer)
        .execute(con)
        .unwrap();
}

pub fn get_manufacturers(con: &PgConnection) -> Vec<model::Manufacturer> {
    use schema::manufacturers::dsl::*;
    manufacturers.load(con).unwrap()
}
