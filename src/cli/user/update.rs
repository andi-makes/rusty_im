use crate::cli::CommandHandler;
use crate::db;
use crate::db::schema;
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use diesel::RunQueryDsl;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    Manufacturer {
        /// Old name of the manufacturer.
        old_name: String,
        /// New name of the manufacturer.
        new_name: String,
    },
    Part {
        /// Id of the part to be updated.
        id: i32,
        /// New name of the part.
        #[structopt(short = "n", long = "name")]
        name: Option<String>,
        /// New manufacturer of the part.
        #[structopt(short = "m", long = "manufacturer")]
        manufacturer: Option<String>,
        /// New amount of stock of the part.
        #[structopt(short = "a", long = "amount")]
        amount: Option<i32>,
        /// New description of the part.
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    Tag {
        /// Id of the tag to be updated.
        id: i32,
        /// New value of the tag.
        #[structopt(short = "v", long = "value")]
        new_value: Option<String>,
        /// New name of the tag.
        #[structopt(short = "n", long = "name")]
        new_name: Option<String>,
    },
}

use schema::parts;
#[derive(AsChangeset)]
#[table_name = "parts"]
struct UpdatePart {
    manufacturer_id: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    amount: Option<i32>,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match self {
            Action::Manufacturer { old_name, new_name } => {
                let id = match db::manufacturer::get_id(connection, old_name.to_string()) {
                    Some(id) => id,
                    None => {
                        println!("There is no manufacturer called `{}`. Aborting.", old_name);
                        std::process::exit(-1);
                    }
                };
                db::manufacturer::update(connection, id, new_name.to_string());
                println!("Updated Manufacturer `{}` to `{}`", old_name, new_name);
            }
            Action::Part {
                id,
                name,
                manufacturer,
                amount,
                description,
            } => {
                if name.is_none()
                    && manufacturer.is_none()
                    && amount.is_none()
                    && description.is_none()
                {
                    println!("Nothing to update, aborting.");
                    std::process::exit(-1);
                }

                diesel::update(parts::table.filter(parts::id.eq(*id)))
                    .set(&UpdatePart {
                        manufacturer_id: match manufacturer {
                            Some(m) => db::manufacturer::get_id(connection, m.to_owned()),
                            None => None,
                        },
                        name: name.to_owned(),
                        description: description.to_owned(),
                        amount: *amount,
                    })
                    .execute(connection)
                    .expect("Could not update the manufacturer field of the part table. Aborting.\nError: ");
                println!("Successfully updated part `{}`.", id);
            }
            Action::Tag {
                id: tag_id,
                new_value,
                new_name,
            } => {
                if new_value.is_none() && new_name.is_none() {
                    println!("Nothing to update, aborting.");
                    std::process::exit(-1);
                }
                let old_tagname_id = {
                    use schema::tags::dsl::*;
                    tags.filter(id.eq(tag_id))
                        .select(tagname_id)
                        .first::<i32>(connection)
                        .ok()
                };

                if let (Some(new_name_u), Some(old_tag_id_u)) = (new_name, old_tagname_id) {
                    print!("Do you want to change all occurrences of the tagname `{}` to `{}`? [y|N]: ", db::tagname::get_name(connection, old_tag_id_u), new_name_u);
                    use std::io::Write;
                    std::io::stdout().flush().unwrap();
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    match input.chars().next() {
                        Some(c) => match c.to_ascii_lowercase() {
                            'y' => {
                                // Simple, just change the tagname table, BUT
                                // There would be a problem if the tagname already exists.
                                // So, let's check if it already exists:
                                let already_exists = {
                                    use schema::tagnames::dsl::*;
                                    tagnames
                                        .filter(name.eq(new_name_u))
                                        .select(id)
                                        .first::<i32>(connection)
                                        .ok()
                                        .is_some()
                                };

                                // If it doesn't exist, we create the new Tagname!
                                if !already_exists {
                                    use schema::tagnames::dsl::*;
                                    diesel::update(tagnames.filter(id.eq(old_tag_id_u)))
                                        .set(name.eq(new_name_u))
                                        .execute(connection)
                                        .expect(
                                            "Could not create a new tagname. Aborting.\nError: ",
                                        );
                                }

                                let new_tagname_id = {
                                    use schema::tagnames::dsl::*;

                                    tagnames
                                        .filter(name.eq(new_name_u))
                                        .select(id)
                                        .first::<i32>(connection)
                                        .expect(
                                            "Could not get the id of the tagname. Aborting.\nError: ",
                                        )
                                };

                                // Now, we need to change all the primary keys from the old tagname to the new tagname
                                use schema::tags::dsl::*;
                                diesel::update(tags.filter(tagname_id.eq(old_tag_id_u)))
                                    .set(tagname_id.eq(new_tagname_id))
                                    .execute(connection)
                                    .expect("Could not update the tagname. Aborting.\nError: ");
                            }
                            _ => {
                                // Check if the new tagname already exists
                                match {
                                    use schema::tagnames::dsl::*;
                                    tagnames
                                        .filter(name.eq(new_name_u))
                                        .select(id)
                                        .first::<i32>(connection)
                                        .ok()
                                } {
                                    Some(existing_id) => {
                                        // The tagname already exists, just change the tagname_id of the tags table
                                        use schema::tags::dsl::*;
                                        diesel::update(tags.filter(id.eq(tag_id)))
                                            .set(tagname_id.eq(existing_id))
                                            .execute(connection)
                                            .expect(
                                                "Could not update the tagname. Aborting.\nError: ",
                                            );
                                    }
                                    None => {
                                        // The tagname does not exist, add a new Tagname and change the tagname_id of the tags table to the new tagname
                                        db::tagname::new(connection, new_name_u.to_string());
                                        let existing_id = db::tagname::get_id(
                                            connection,
                                            new_name_u.to_string(),
                                        ).expect("Could not get the id of the tagname. Aborting.\n Error: ");

                                        use schema::tags::dsl::*;
                                        diesel::update(tags.filter(id.eq(tag_id)))
                                            .set(tagname_id.eq(existing_id))
                                            .execute(connection)
                                            .expect(
                                                "Could not update the tagname. Aborting.\nError: ",
                                            );
                                    }
                                }
                            }
                        },
                        None => {
                            println!("Aborting.");
                            std::process::exit(-1);
                        }
                    }
                }

                if new_value.is_some() {
                    use schema::tags::dsl::*;
                    diesel::update(tags.filter(id.eq(tag_id)))
                        .set(value.eq(new_value.as_ref().unwrap()))
                        .execute(connection)
                        .expect("Could not update the value of the tag. Aborting.\nError: ");
                }
            }
        };
    }
}
