use super::schema::manufacturers;
use super::*;
use cli_table::Table;

#[derive(Queryable, Table)]
pub struct Manufacturer {
    #[table(title = "ID")]
    pub id: i32,
    #[table(title = "Name")]
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

pub fn insert(connection: &diesel::SqliteConnection, name: String) {
    let manufacturer = NewManufacturer { name };
    diesel::insert_into(manufacturers::table)
        .values(&manufacturer)
        .execute(connection)
        .unwrap();
}

pub fn update(connection: &diesel::SqliteConnection, selected_id: i32, new_name: String) {
    use schema::manufacturers::dsl::*;

    diesel::update(manufacturers.filter(id.eq(selected_id)))
        .set(name.eq(new_name))
        .execute(connection)
        .unwrap();
}

pub fn delete(connection: &diesel::SqliteConnection, selected_id: i32) {
    use schema::manufacturers::dsl::*;

    diesel::delete(manufacturers.filter(id.eq(selected_id)))
        .execute(connection)
        .unwrap();
}

pub fn get(connection: &diesel::SqliteConnection) -> Vec<Manufacturer> {
    use schema::manufacturers::dsl::*;
    manufacturers.load(connection).unwrap()
}
