use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Adds a new manufacturer
    Add {
        /// Name of the new manufacturer
        name: String,
    },
    /// Updates an existing manufacturers name based by its id
    Update {
        /// id of the manufacturer to be updated
        id: i32,
        /// New manufacturer name
        new: String,
    },
    /// Deletes an existing manufacturer based by its id
    Delete {
        // id of the manufacturer to be deleted
        id: i32,
    },
    /// Lists all manufacturers
    List,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Action::Add { name } => {
                db::manufacturer::insert(&connection, name.trim().to_string());
            }
            Action::Update { id, new } => {
                db::manufacturer::update(&connection, *id, new.trim().to_string())
            }
            Action::Delete { id } => db::manufacturer::delete(&connection, *id),
            Action::List => {
                for manufacturer in db::manufacturer::get(&connection) {
                    println!("{}", manufacturer.to_string());
                }
            }
        }
    }
}
