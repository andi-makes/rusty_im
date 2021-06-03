use super::schema::*;

use diesel::prelude::*;

#[derive(Queryable)]
pub struct Part {
    pub id: i32,
    pub manufacturer_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub amount: i32,
}

impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, manufacturer_id: {}, name: {}, amount: {}",
            self.id, self.manufacturer_id, self.name, self.amount
        )
    }
}

#[derive(Insertable)]
#[table_name = "parts"]
pub struct NewPart {
    pub manufacturer_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub amount: i32,
}

pub fn insert(
    connection: &diesel::SqliteConnection,
    manufacturer_id: i32,
    name: &String,
    description: &Option<String>,
    amount: i32,
) {
    let part = NewPart {
        manufacturer_id,
        name: name.to_string(),
        description: description.clone(),
        amount,
    };
    diesel::insert_into(parts::table)
        .values(&part)
        .execute(connection)
        .unwrap();
}

pub fn update_amount(connection: &diesel::SqliteConnection, part_id: i32, new_amount: i32) {
    use super::schema::parts::dsl::*;
    diesel::update(parts.filter(id.eq(part_id)))
        .set(amount.eq(new_amount))
        .execute(connection)
        .unwrap();
}

pub fn update_description(
    connection: &diesel::SqliteConnection,
    part_id: i32,
    new_description: String,
) {
    use super::schema::parts::dsl::*;
    diesel::update(parts.filter(id.eq(part_id)))
        .set(description.eq(new_description))
        .execute(connection)
        .unwrap();
}

pub fn get(connection: &diesel::SqliteConnection) -> Vec<Part> {
    use super::schema::parts::dsl::*;
    parts.load(connection).unwrap()
}

pub fn get_detailed(
    connection: &diesel::SqliteConnection,
) -> Vec<(i32, String, Option<String>, i32)> {
    let source = parts::table.left_join(manufacturers::table).select((
        parts::id,
        parts::name,
        manufacturers::name.nullable(),
        parts::amount,
    ));
    source
        .load::<(i32, String, Option<String>, i32)>(connection)
        .unwrap()
}

pub fn delete(connection: &diesel::SqliteConnection, selected_id: i32) {
    use super::schema::parts::dsl::*;

    diesel::delete(parts.filter(id.eq(selected_id)))
        .execute(connection)
        .unwrap();
}

pub fn add_tag(connection: &diesel::SqliteConnection, p_id: i32, t_id: i32) {
    use crate::db::schema::part_tag::dsl::*;

    diesel::insert_into(part_tag)
        .values((part_id.eq(p_id), tag_id.eq(t_id)))
        .execute(connection)
        .unwrap();
}

pub fn remove_tag(connection: &diesel::SqliteConnection, p_id: i32, t_id: i32) {
    use crate::db::schema::part_tag::dsl::*;

    diesel::delete(part_tag.filter(part_id.eq(p_id).and(tag_id.eq(t_id))))
        .execute(connection)
        .unwrap();
}
