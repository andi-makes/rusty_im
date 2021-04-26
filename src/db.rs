pub mod manufacturer;
pub mod part;
pub mod tag;
pub mod tagname;

pub mod migration;
pub mod schema;

pub use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn connect() -> PgConnection {
    PgConnection::establish(crate::config::get_database_connection_url().as_str()).unwrap()
}

pub fn list(
    connection: &PgConnection,
) -> Vec<(i32, String, i32, String, Option<String>, Option<String>)> {
    use diesel::*;
    use schema::*;

    // parts LEFT JOIN manufacturers ON parts.manufacturer_id = manufacturers.id
    let join = parts::table.left_join(manufacturers::table);
    // "above" LEFT JOIN part_tag on parts.id = part_tag.part_id
    let join = join.left_join(part_tag::table.on(part_tag::dsl::part_id.eq(parts::dsl::id)));
    // "above" LEFT JOIN tags ON part_tag.tag_id = tags.id
    let join = join.left_join(tags::table.on(part_tag::dsl::tag_id.eq(tags::dsl::id)));
    // "above" LEFT JOIN tagnames ON tags.tagname_id = tagnames.id
    let join = join.left_join(tagnames::table.on(tags::dsl::tagname_id.eq(tagnames::dsl::id)));

    join.select((
        parts::id,
        parts::name,
        parts::amount,
        manufacturers::name,
        tagnames::name.nullable(),
        tags::value.nullable(),
    ))
    .load::<(i32, String, i32, String, Option<String>, Option<String>)>(connection)
    .unwrap()
}
