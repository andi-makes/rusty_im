use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    Add {
        /// The id of the part which should recieve the tag
        part_id: i32,
        /// The id of the tag which should be given to the part
        tag_id: i32,
    },
    Remove {
        /// The id of the part which should get the tag removed
        part_id: i32,
        /// The id of the tag which should be removed from the part
        tag_id: i32,
    },
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match self {
            Action::Add { part_id, tag_id } => db::part::add_tag(connection, *part_id, *tag_id),
            Action::Remove { part_id, tag_id } => {
                db::part::remove_tag(connection, *part_id, *tag_id)
            }
        }
    }
}
