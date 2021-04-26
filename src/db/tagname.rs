use super::schema::tagnames;
use diesel::prelude::*;
use diesel::PgConnection;

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

pub fn new(connection: &PgConnection, name: String) {
    let data = NewTagname { name };
    diesel::insert_into(tagnames::table)
        .values(&data)
        .execute(connection)
        .unwrap();
}

pub fn get(connection: &PgConnection) -> Vec<Tagname> {
    tagnames::dsl::tagnames.load(connection).unwrap()
}

pub fn update(connection: &PgConnection, id: i32, new_name: String) {
    use self::tagnames::dsl as col;
    diesel::update(col::tagnames.filter(col::id.eq(id)))
        .set(col::name.eq(new_name))
        .execute(connection)
        .unwrap();
}

pub fn delete(connection: &PgConnection, id: i32) {
    use self::tagnames::dsl as col;
    diesel::delete(col::tagnames.filter(col::id.eq(id)))
        .execute(connection)
        .unwrap();
}
