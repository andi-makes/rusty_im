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
    fn handle(&self, connection: &diesel::SqliteConnection);
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

pub fn parse(path: &str) {
    let args = Commands::from_args();

    match args {
        Commands::Migration(m) => {
            let connection = db::connect(path).unwrap();
            m.handle(&connection);
        }
        Commands::Manufacturer(m) => {
            let connection = db::connect(path).unwrap();
            m.handle(&connection);
        }
        Commands::Part(p) => {
            let connection = db::connect(path).unwrap();
            p.handle(&connection);
        }
        Commands::Tagname(t) => {
            let connection = db::connect(path).unwrap();
            t.handle(&connection);
        }
        Commands::Tag(t) => {
            let connection = db::connect(path).unwrap();
            t.handle(&connection);
        }
        Commands::List => {
            let connection = db::connect(path).unwrap();
            for entry in db::list(&connection) {
                println!("{:?}", entry);
            }
        }
    }
}
