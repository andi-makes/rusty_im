use super::schema::tags;
use diesel::prelude::*;

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

pub fn new(connection: &diesel::SqliteConnection, tagname_id: i32, value: String) {
    let data = NewTag { tagname_id, value };
    diesel::insert_into(tags::table)
        .values(&data)
        .execute(connection)
        .expect("Could not insert value into tags table. Aborting.\nError: ");
}

pub fn get(connection: &diesel::SqliteConnection) -> Vec<Tag> {
    tags::dsl::tags
        .load(connection)
        .expect("Could not load the tags table. Aborting.\nError: ")
}

pub fn update(connection: &diesel::SqliteConnection, id: i32, new_value: String) {
    use tags::dsl as col;
    diesel::update(col::tags.filter(col::id.eq(id)))
        .set(col::value.eq(new_value))
        .execute(connection)
        .expect("Could not update the tags table. Aborting.\nError: ");
}

pub fn delete(connection: &diesel::SqliteConnection, id: i32) {
    use tags::dsl as col;
    diesel::delete(col::tags.filter(col::id.eq(id)))
        .execute(connection)
        .expect("Could not delete a value from the tags table. Aborting.\nError: ");
}
