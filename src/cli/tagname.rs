use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    Add { tagname: String },
    Update { id: i32, new_name: String },
    Delete { id: i32 },
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::PgConnection) {
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
