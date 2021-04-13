use rusty_im::*;
use std::fmt::{self, Display};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Action {
    Add { name: String },
    Update { id: i32 },
    Delete { id: i32 },
    List,
}

impl Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Action::Add { name: _ } => "add",
                Action::Update { id: _ } => "update",
                Action::Delete { id: _ } => "delete",
                Action::List => "list",
            }
        )
    }
}

#[derive(StructOpt, Debug)]
enum Table {
    /// Modify the manufacturer table
    Manufacturer {
        #[structopt(subcommand)]
        action: Action,
    },
}

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Table::Manufacturer { action: _ } => "manufacturer",
            }
        )
    }
}

#[derive(StructOpt, Debug)]
struct Cli {
    /// The name of the table to modify
    #[structopt(subcommand)]
    table: Table,
}

fn main() {
    let args = Cli::from_args();
    let connection = database::connect();
    println!("{:?}", args);

    match args.table {
        Table::Manufacturer { action } => match action {
            Action::Add { name } => {
                database::insert_manufacturer(&connection, name);
            }
            Action::Update { id } => {}
            Action::Delete { id } => {}
            Action::List => {
                for manufacturer in database::get_manufacturers(&connection) {
                    println!("{}", manufacturer.to_string());
                }
            }
        },
    }
}
