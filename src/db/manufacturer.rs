use super::schema::manufacturers;
use super::*;

#[derive(Queryable)]
pub struct Manufacturer {
    pub id: i32,
    pub name: String,
}
impl std::fmt::Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.name)
    }
}
#[derive(Insertable)]
#[table_name = "manufacturers"]
pub struct NewManufacturer {
    pub name: String,
}

pub fn insert(con: &PgConnection, name: String) {
    let manufacturer = NewManufacturer { name };
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

pub fn get(con: &PgConnection) -> Vec<Manufacturer> {
    use schema::manufacturers::dsl::*;
    manufacturers.load(con).unwrap()
}