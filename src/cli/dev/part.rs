use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Adds a new part
    Add {
        /// id of the manufacturer of the new part
        manufacturer_id: i32,
        /// name of the new part
        name: String,
        /// Initial stock of the new part
        amount: i32,
    },
    /// Updates an existing parts stock based by its id
    Update {
        /// id of the manufacturer to be updated
        id: i32,
        /// New stock of the existing part
        new_amount: i32,
    },
    /// Deletes an exisitng part based by its id
    Delete {
        /// id of the part to be deleted
        id: i32,
    },
    /// Lists all parts
    List,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Action::Add {
                manufacturer_id,
                name,
                amount,
            } => db::part::insert(&connection, *manufacturer_id, name.to_string(), *amount),
            Action::Delete { id } => db::part::delete(&connection, *id),
            Action::List => {
                let parts = db::part::get_detailed(&connection);
                for p in parts {
                    println!(
                        "id: {}, We have {} of {} from {}",
                        p.0,
                        p.3,
                        p.1,
                        match p.2 {
                            Some(val) => val,
                            None => "NULL".to_string(),
                        }
                    );
                }
            }
            Action::Update { id, new_amount } => db::part::update(&connection, *id, *new_amount),
        }
    }
}
