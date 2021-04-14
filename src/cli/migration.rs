use super::{CommandHandler, StructOpt};
use crate::db;
use structopt::clap::arg_enum;

arg_enum! {
    #[derive(StructOpt)]
    pub enum Migration {
        Run,
        Redo,
        Revert,
        List,
    }
}

impl CommandHandler for Migration {
    fn handle(&self, connection: &db::PgConnection) {
        match &self {
            Migration::Run => db::migration::run(connection),
            Migration::Redo => db::migration::redo(connection),
            Migration::Revert => db::migration::revert(connection),
            Migration::List => db::migration::print_fs(),
        }
    }
}
