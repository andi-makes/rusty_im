use crate::cli::CommandHandler;
use crate::db;
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
}

use crate::db::schema::*;
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
                use crate::diesel::ExpressionMethods;
                use crate::diesel::QueryDsl;

                if name.as_ref() == None
                    && manufacturer.as_ref() == None
                    && amount.as_ref() == None
                    && description.as_ref() == None
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
                    .unwrap();
                println!("Successfully updated part `{}`.", id);
            }
        };
    }
}
