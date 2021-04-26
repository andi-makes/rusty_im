use crate::db;
use structopt::StructOpt;

mod manufacturer;
mod migration;
mod part;
mod tag;
mod tagname;

/// Defines an unified Interface for calling cli-subcommands and provides access to the database
pub trait CommandHandler {
    /// Provides database access for subcommands through `connection` variable
    fn handle(&self, connection: &db::PgConnection);
}

#[derive(StructOpt)]
#[structopt(
    name = "Rusty Inventory Manager",
    about = "A simple inventory manager written in rust."
)]
enum Commands {
    /// Low level handling of the database
    Migration(migration::Migration),
    /// Low level managing of manufacturers
    Manufacturer(manufacturer::Action),
    /// Low level managing of parts
    Part(part::Action),
    /// Low level managing of tagnames
    Tagname(tagname::Action),
    /// Low level managing of tags
    Tag(tag::Action),
    /// Prints a view of the entire database
    List,
}

pub fn parse(connection: &db::PgConnection) {
    let args = Commands::from_args();

    match args {
        Commands::Migration(m) => m.handle(&connection),
        Commands::Manufacturer(m) => m.handle(&connection),
        Commands::Part(p) => p.handle(&connection),
        Commands::Tagname(t) => t.handle(&connection),
        Commands::Tag(t) => t.handle(&connection),
        Commands::List => {
            for entry in db::list(&connection) {
                println!("{:?}", entry);
            }
        }
    }
}
