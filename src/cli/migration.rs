use super::{CommandHandler, StructOpt};
use crate::db;

#[derive(StructOpt)]
pub enum Migration {
    /// Initialises the database
    Init,
    /// Deletes the database and re-initialises it
    Redo,
    /// Deletes the database
    Revert,
    /// Lists SQL statements for initialising and deleting the database
    List,
}

impl CommandHandler for Migration {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Migration::Init => db::migration::run(connection),
            Migration::Redo => db::migration::redo(connection),
            Migration::Revert => db::migration::revert(connection),
            Migration::List => db::migration::print_fs(),
        }
    }
}
