use super::schema::tagnames;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Tagname {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "tagnames"]
struct NewTagname {
    name: String,
}

pub fn new(connection: &diesel::SqliteConnection, name: String) {
    let data = NewTagname { name };
    diesel::insert_into(tagnames::table)
        .values(&data)
        .execute(connection)
        .unwrap();
}

pub fn get(connection: &diesel::SqliteConnection) -> Vec<Tagname> {
    tagnames::dsl::tagnames.load(connection).unwrap()
}

pub fn update(connection: &diesel::SqliteConnection, id: i32, new_name: String) {
    use self::tagnames::dsl as col;
    diesel::update(col::tagnames.filter(col::id.eq(id)))
        .set(col::name.eq(new_name))
        .execute(connection)
        .unwrap();
}

pub fn delete(connection: &diesel::SqliteConnection, id: i32) {
    use self::tagnames::dsl as col;
    diesel::delete(col::tagnames.filter(col::id.eq(id)))
        .execute(connection)
        .unwrap();
}

pub fn get_id(connection: &diesel::SqliteConnection, selected_name: String) -> Option<i32> {
    use crate::db::schema::tagnames::dsl::*;
    match tagnames
        .filter(name.eq(selected_name))
        .select(id)
        .first::<i32>(connection)
    {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}
