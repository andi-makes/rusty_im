use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    /// Adds a new tag
    Add {
        /// id of the tagname of the new tag
        tagname_id: i32,
        /// value of the new tag
        value: String,
    },
    /// Updates an existing tags value based by its id
    Update {
        /// id of the tag to be updated
        id: i32,
        /// New value of the tag
        new_value: String,
    },
    /// Deletes an exisitng tag based by its id
    Delete {
        /// id of the tag to be deleted
        id: i32,
    },
    /// Lists all tags
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Action::Add { tagname_id, value } => {
                db::tag::new(&connection, *tagname_id, value.to_string())
            }
            Action::Update { id, new_value } => {
                db::tag::update(&connection, *id, new_value.to_string())
            }
            Action::Delete { id } => db::tag::delete(&connection, *id),
            Action::List => {
                for element in db::tag::get(&connection) {
                    println!(
                        "{}: {} with {}",
                        element.id, element.tagname_id, element.value
                    );
                }
            }
        }
    }
}
