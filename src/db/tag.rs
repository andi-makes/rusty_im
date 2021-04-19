use super::schema::{tagnames, tags};
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Queryable)]
pub struct Tagname {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "tagnames"]
struct NewTagname {
    name: String,
}

#[derive(Queryable)]
pub struct Tag {
    id: i32,
    tagname_id: i32,
    value: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
struct NewTag {
    tagname_id: i32,
    value: String,
}

pub fn new_tagname(conn: &PgConnection, name: String) {
    let data = NewTagname { name };
    diesel::insert_into(tagnames::table)
        .values(&data)
        .execute(conn)
        .unwrap();
}

pub fn get_tagnames(conn: &PgConnection) -> Vec<Tagname> {
    tagnames::dsl::tagnames.load(conn).unwrap()
}

pub fn update_tagname(conn: &PgConnection, id: i32, new_name: String) {
    use self::tagnames::dsl as col;
    diesel::update(col::tagnames.filter(col::id.eq(id)))
        .set(col::name.eq(new_name))
        .execute(conn)
        .unwrap();
}

pub fn delete_tagname(conn: &PgConnection, id: i32) {
    use self::tagnames::dsl as col;
    diesel::delete(col::tagnames.filter(col::id.eq(id)))
        .execute(conn)
        .unwrap();
}

pub fn new_tag(conn: &PgConnection, tagname_id: i32, value: String) {
    let data = NewTag { tagname_id, value };
    diesel::insert_into(tags::table)
        .values(&data)
        .execute(conn)
        .unwrap();
}

pub fn get_tags(conn: &PgConnection) -> Vec<Tag> {
    tags::dsl::tags.load(conn).unwrap()
}

pub fn update_tag(conn: &PgConnection, id: i32, new_value: String) {
    use tags::dsl as col;
    diesel::update(col::tags.filter(col::id.eq(id)))
        .set(col::value.eq(new_value))
        .execute(conn)
        .unwrap();
}

pub fn delete_tag(conn: &PgConnection, id: i32) {
    use tags::dsl as col;
    diesel::delete(col::tags.filter(col::id.eq(id)))
        .execute(conn)
        .unwrap();
}
