use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    Add { name: String },
    Update { id: i32, new: String },
    Delete { id: i32 },
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::PgConnection) {
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
