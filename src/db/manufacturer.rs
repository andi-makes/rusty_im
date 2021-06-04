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
        .expect("Could not insert value in manufacturer table. Aborting.\nError: ");
}

pub fn update(connection: &diesel::SqliteConnection, selected_id: i32, new_name: String) {
    use schema::manufacturers::dsl::*;

    diesel::update(manufacturers.filter(id.eq(selected_id)))
        .set(name.eq(new_name))
        .execute(connection)
        .expect("Could not update value in manufacturer table. Aborting.\nError: ");
}

pub fn delete(connection: &diesel::SqliteConnection, selected_id: i32) {
    use schema::manufacturers::dsl::*;

    diesel::delete(manufacturers.filter(id.eq(selected_id)))
        .execute(connection)
        .expect("Could not delete value from manufacturer table. Aborting.\nError: ");
}

pub fn get_id(connection: &diesel::SqliteConnection, selected_name: String) -> Option<i32> {
    use schema::manufacturers::dsl::*;
    manufacturers
        .filter(name.eq(selected_name))
        .select(id)
        .first::<i32>(connection)
        .ok()
}

pub fn get(connection: &diesel::SqliteConnection) -> Vec<Manufacturer> {
    use schema::manufacturers::dsl::*;
    manufacturers
        .load(connection)
        .expect("Could not load the manufacturer table. Aborting.\nError: ")
}
