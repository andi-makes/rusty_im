pub mod manufacturer;
pub mod part;
pub mod tag;
pub mod tagname;

pub mod migration;
pub mod schema;

use diesel::prelude::*;

pub fn connect(url: &str) -> SqliteConnection {
    let try_connection = SqliteConnection::establish(url);

    match try_connection {
        Ok(connection) => connection,
        Err(err) => {
            println!("Couldn't connect to database. Aborting.\nError: {}", err);
            std::process::exit(-1);
        }
    }
}

use cli_table::Table;

pub fn cli_table_print_option_string(value: &Option<String>) -> impl std::fmt::Display {
    value.clone().unwrap_or_else(|| String::from("?"))
}

#[derive(Table, Queryable)]
pub struct ListTable {
    #[table(title = "ID")]
    id: i32,
    #[table(title = "Name")]
    name: String,
    #[table(title = "Description", display_fn = "cli_table_print_option_string")]
    description: Option<String>,
    #[table(title = "Amount")]
    amount: i32,
    #[table(title = "Manufacturer")]
    manufacturer: String,
    #[table(title = "Tag", display_fn = "cli_table_print_option_string")]
    tag: Option<String>,
    #[table(title = "Tag-Value", display_fn = "cli_table_print_option_string")]
    tag_value: Option<String>,
}

// fn <func_name>(value: &<type>) -> impl Display

pub fn list(connection: &SqliteConnection) -> Vec<ListTable> {
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

    match join
        .select((
            parts::id,
            parts::name,
            parts::description,
            parts::amount,
            manufacturers::name,
            tagnames::name.nullable(),
            tags::value.nullable(),
        ))
        .load::<ListTable>(connection)
    {
        Ok(table) => table,
        Err(err) => {
            eprintln!(
                "Error occured while listing the table! Aborting.\nError: {}",
                err
            );
            std::process::exit(-1);
        }
    }
}
