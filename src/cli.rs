use crate::db;
use structopt::StructOpt;

mod types;

pub fn parse(connection: &db::PgConnection) {
    let args = types::Cli::from_args();

    match args.table {
        types::Table::Manufacturer { action } => match action {
            types::Action::Add { name } => {
                db::manufacturer::insert(&connection, name);
            }
            types::Action::Update { id, new } => db::manufacturer::update(&connection, id, new),
            types::Action::Delete { id } => db::manufacturer::delete(&connection, id),
            types::Action::List => {
                for manufacturer in db::manufacturer::get(&connection) {
                    println!("{}", manufacturer.to_string());
                }
            }
        },
    }
}
