use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Lists all parts
    Parts,
    /// Lists all manufacturers
    Manufacturers,
    /// Lists all Tags
    Tags,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match self {
            Action::Parts => {
                use cli_table::WithTitle;
                cli_table::print_stdout(db::list(&connection).with_title()).unwrap();
            }
            Action::Manufacturers => {}
            Action::Tags => {}
        }
    }
}
