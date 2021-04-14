use super::StructOpt;
use std::fmt::{self, Display};
#[derive(StructOpt, Debug)]
pub enum Action {
    Add { name: String },
    Update { id: i32, new: String },
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
                Action::Update { id: _, new: _ } => "update",
                Action::Delete { id: _ } => "delete",
                Action::List => "list",
            }
        )
    }
}

#[derive(StructOpt, Debug)]
pub enum Table {
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
pub struct Cli {
    /// The name of the table to modify
    #[structopt(subcommand)]
    pub table: Table,
}
