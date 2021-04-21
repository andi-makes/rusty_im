use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    Add { tagname_id: i32, value: String },
    Update { id: i32, new_value: String },
    Delete { id: i32 },
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::PgConnection) {
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
