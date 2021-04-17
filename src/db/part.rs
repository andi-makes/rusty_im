use super::schema::*;

use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable)]
pub struct Part {
    pub id: i32,
    pub manufacturer_id: i32,
    pub name: String,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, manufacturer_id: {}, name: {}",
            self.id, self.manufacturer_id, self.name
        )
    }
}

#[derive(Insertable)]
#[table_name = "parts"]
pub struct NewPart {
    pub manufacturer_id: i32,
    pub name: String,
}

pub fn insert(con: &PgConnection, manufacturer_id: i32, name: String) {
    let part = NewPart {
        manufacturer_id,
        name,
    };
    diesel::insert_into(parts::table)
        .values(&part)
        .execute(con)
        .unwrap();
}

pub fn get(con: &PgConnection) -> Vec<Part> {
    use super::schema::parts::dsl::*;
    parts.load(con).unwrap()
}

pub fn get_detailed(con: &PgConnection) -> Vec<(i32, String, Option<String>)> {
    let source = parts::table.left_join(manufacturers::table).select((
        parts::id,
        parts::name,
        manufacturers::name.nullable(),
    ));
    source.load::<(i32, String, Option<String>)>(con).unwrap()
}

pub fn delete(conn: &PgConnection, selected_id: i32) {
    use super::schema::parts::dsl::*;

    diesel::delete(parts.filter(id.eq(selected_id)))
        .execute(conn)
        .unwrap();
}
