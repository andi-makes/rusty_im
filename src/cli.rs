use crate::db;
use structopt::StructOpt;

mod user;

mod dev;
use dev::*;

/// Defines an unified Interface for calling cli-subcommands and provides access to the database
pub trait CommandHandler {
    /// Provides database access for subcommands through `connection` variable
    fn handle(&self, connection: &diesel::SqliteConnection);
}

#[derive(StructOpt)]
enum DevCommands {
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

#[derive(StructOpt)]
#[structopt(
    name = "Rusty Inventory Manager",
    about = "A simple inventory manager written in rust.",
    global_settings(&[structopt::clap::AppSettings::DisableHelpSubcommand])
)]
enum Commands {
    /// Low level API for rim, intended for development purposes
    Low(DevCommands),
    /// Lists stored Data
    List(user::list::Action),
    /// Adds data to database
    Add(user::add::Action),
}

pub fn parse(path: &str) {
    let args = Commands::from_args();
    match args {
        Commands::Low(l) => match l {
            DevCommands::Migration(m) => {
                let connection = db::connect(path).unwrap();
                m.handle(&connection);
            }
            DevCommands::Manufacturer(m) => {
                let connection = db::connect(path).unwrap();
                m.handle(&connection);
            }
            DevCommands::Part(p) => {
                let connection = db::connect(path).unwrap();
                p.handle(&connection);
            }
            DevCommands::Tagname(t) => {
                let connection = db::connect(path).unwrap();
                t.handle(&connection);
            }
            DevCommands::Tag(t) => {
                let connection = db::connect(path).unwrap();
                t.handle(&connection);
            }
            DevCommands::List => {
                println!("low level list is deprecated, use normal list instead")
                // let connection = db::connect(path).unwrap();
                // for entry in db::list(&connection) {
                //     println!("{:?}", entry);
                // }
            }
        },
        Commands::List(l) => {
            let connection = db::connect(path).unwrap();
            l.handle(&connection);
        }
        Commands::Add(a) => {
            let connection = db::connect(path).unwrap();
            a.handle(&connection);
        }
    }
}
