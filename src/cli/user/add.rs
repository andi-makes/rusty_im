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
    Tag {
        /// The tagname of the tag.
        name: String,
        /// The value of the tag.
        value: String,
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
                println!("Added part {} from {} to database.", name, manufacturer);
            }
            Action::Tag { name, value } => {
                let tagname_id = match db::tagname::get_id(connection, name.to_string()) {
                    Some(id) => id,
                    None => {
                        print!("There is no tagname called `{}` in the database yet.\nDo you want to add it to the database? [y|N]: ", name);
                        use std::io::Write;
                        std::io::stdout().flush().unwrap();
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        match input.chars().nth(0) {
                            Some(c) => match c.to_ascii_lowercase() {
                                'y' => {
                                    db::tagname::new(connection, name.to_string());
                                    db::tagname::get_id(connection, name.to_string()).unwrap()
                                }
                                _ => {
                                    println!("Aborting.");
                                    std::process::exit(-1);
                                }
                            },
                            None => {
                                println!("Aborting.");
                                std::process::exit(-1);
                            }
                        }
                    }
                };
                db::tag::new(connection, tagname_id, value.to_string());
                println!("Added tag `{}: {}` to database.", name, value);
            }
        }
    }
}
