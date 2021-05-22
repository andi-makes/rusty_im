use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    Manufacturer { old_name: String, new_name: String },
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match self {
            Action::Manufacturer { old_name, new_name } => {
                let id = match db::manufacturer::get_id(connection, old_name.to_string()) {
                    Some(id) => id,
                    None => {
                        println!("There is no manufacturer called `{}`. Aborting.", old_name);
                        std::process::exit(-1);
                    }
                };
                db::manufacturer::update(connection, id, new_name.to_string());
                println!("Updated Manufacturer `{}` to `{}`", old_name, new_name);
            }
        };
    }
}
