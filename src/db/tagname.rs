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

pub fn new(conn: &PgConnection, name: String) {
    let data = NewTagname { name };
    diesel::insert_into(tagnames::table)
        .values(&data)
        .execute(conn)
        .unwrap();
}

pub fn get(conn: &PgConnection) -> Vec<Tagname> {
    tagnames::dsl::tagnames.load(conn).unwrap()
}

pub fn update(conn: &PgConnection, id: i32, new_name: String) {
    use self::tagnames::dsl as col;
    diesel::update(col::tagnames.filter(col::id.eq(id)))
        .set(col::name.eq(new_name))
        .execute(conn)
        .unwrap();
}

pub fn delete(conn: &PgConnection, id: i32) {
    use self::tagnames::dsl as col;
    diesel::delete(col::tagnames.filter(col::id.eq(id)))
        .execute(conn)
        .unwrap();
}
