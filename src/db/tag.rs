use super::schema::tags;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub tagname_id: i32,
    pub value: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
struct NewTag {
    tagname_id: i32,
    value: String,
}

pub fn new(conn: &PgConnection, tagname_id: i32, value: String) {
    let data = NewTag { tagname_id, value };
    diesel::insert_into(tags::table)
        .values(&data)
        .execute(conn)
        .unwrap();
}

pub fn get(conn: &PgConnection) -> Vec<Tag> {
    tags::dsl::tags.load(conn).unwrap()
}

pub fn update(conn: &PgConnection, id: i32, new_value: String) {
    use tags::dsl as col;
    diesel::update(col::tags.filter(col::id.eq(id)))
        .set(col::value.eq(new_value))
        .execute(conn)
        .unwrap();
}

pub fn delete(conn: &PgConnection, id: i32) {
    use tags::dsl as col;
    diesel::delete(col::tags.filter(col::id.eq(id)))
        .execute(conn)
        .unwrap();
}
