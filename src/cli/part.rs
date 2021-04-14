use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    Add { manufacturer_id: i32, name: String },
    Delete { id: i32 },
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::PgConnection) {
        match &self {
            Action::Add {
                manufacturer_id,
                name,
            } => db::part::insert(&connection, *manufacturer_id, name.to_string()),
            Action::Delete { id } => todo!("Implement deleting parts!"),
            Action::List => {
                let parts = db::part::get(&connection);
                for p in parts {
                    println!("{}", p);
                }
            }
        }
    }
}
