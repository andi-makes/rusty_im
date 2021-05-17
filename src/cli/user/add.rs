use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    Manufacturer {
        /// Name of the manufacturer to be added.
        name: String,
    },
    Part {
        /// Name of the part to be added.
        name: String,
        /// The name or id of the manufacturer of the part.
        manufacturer: String,
        /// Initial stock of the new part. Defaults to zero.
        #[structopt(short = "a", long = "amount")]
        amount: Option<i32>,
        /// Description of the new part.
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match self {
            Action::Manufacturer { name } => {
                db::manufacturer::insert(connection, name.to_string());
                println!("Added manufacturer {} to database.", name);
            }
            Action::Part {
                name,
                manufacturer,
                amount,
                description,
            } => {
                let manu_id = match db::manufacturer::get_id(connection, manufacturer.to_string()) {
                    Some(id) => id,
                    None => {
                        println!(
                            "There was no manufacturer with the name `{}`. Aborting.",
                            manufacturer
                        );
                        std::process::exit(-1);
                    }
                };
                db::part::insert(connection, manu_id, name, description, amount.unwrap_or(0));
            }
        }
    }
}
