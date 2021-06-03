use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Adds a new tagname
    Add {
        /// The new tagname
        tagname: String,
    },
    /// Updates an existing tagname based on its id
    Update {
        /// id of the tagname to be updated
        id: i32,
        /// New tagname
        new_name: String,
    },
    /// Deletes an existing tagname based on its id
    Delete {
        /// id of the tagname to be deleted
        id: i32,
    },
    /// Lists all tagnames
    List,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Action::Add { tagname } => db::tagname::new(&connection, tagname.to_string()),
            Action::Update { id, new_name } => {
                db::tagname::update(&connection, *id, new_name.to_string())
            }
            Action::Delete { id } => db::tagname::delete(&connection, *id),
            Action::List => {
                for element in db::tagname::get(&connection) {
                    println!("{}: {}", element.id, element.name);
                }
            }
        }
    }
}
