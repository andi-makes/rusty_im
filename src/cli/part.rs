use super::StructOpt;
use crate::db;

#[derive(StructOpt)]
pub enum Action {
    Add {
        manufacturer_id: i32,
        name: String,
        amount: i32,
    },
    Delete {
        id: i32,
    },
    List,
}

impl super::CommandHandler for Action {
    fn handle(&self, connection: &diesel::PgConnection) {
        match &self {
            Action::Add {
                manufacturer_id,
                name,
                amount,
            } => db::part::insert(&connection, *manufacturer_id, name.to_string(), *amount),
            Action::Delete { id } => db::part::delete(&connection, *id),
            Action::List => {
                let parts = db::part::get_detailed(&connection);
                for p in parts {
                    println!(
                        "id: {}, We have {} of {} from {}",
                        p.0,
                        p.3,
                        p.1,
                        match p.2 {
                            Some(val) => val,
                            None => "NULL".to_string(),
                        }
                    );
                }
            }
        }
    }
}
