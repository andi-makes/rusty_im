use crate::db;
use structopt::StructOpt;

mod types;

pub fn parse(connection: &db::PgConnection) {
    let args = types::Cli::from_args();

    match args.table {
        types::Table::Manufacturer { action } => match action {
            types::Action::Add { name } => {
                db::insert_manufacturer(&connection, name);
            }
            types::Action::Update { id } => {}
            types::Action::Delete { id } => {}
            types::Action::List => {
                for manufacturer in db::get_manufacturers(&connection) {
                    println!("{}", manufacturer.to_string());
                }
            }
        },
    }
}
