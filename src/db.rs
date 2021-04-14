pub mod migration;
pub mod model;
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

pub mod manufacturer {
    use super::schema::*;
    use super::*;

    pub fn insert(con: &PgConnection, name: String) {
        let manufacturer = model::NewManufacturer { name };
        diesel::insert_into(manufacturers::table)
            .values(&manufacturer)
            .execute(con)
            .unwrap();
    }

    pub fn update(conn: &PgConnection, selected_id: i32, new_name: String) {
        use schema::manufacturers::dsl::*;

        diesel::update(manufacturers.filter(id.eq(selected_id)))
            .set(name.eq(new_name))
            .execute(conn)
            .unwrap();
    }

    pub fn delete(conn: &PgConnection, selected_id: i32) {
        use schema::manufacturers::dsl::*;

        diesel::delete(manufacturers.filter(id.eq(selected_id)))
            .execute(conn)
            .unwrap();
    }

    pub fn get(con: &PgConnection) -> Vec<model::Manufacturer> {
        use schema::manufacturers::dsl::*;
        manufacturers.load(con).unwrap()
    }
}
