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
    /// Update data
    Update(user::update::Action),
    /// Adds tags to parts
    Tag(user::tag::Action),
}

pub fn parse(path: &str) {
    let args = Commands::from_args();
    let connection = db::connect(path);
    match args {
        Commands::Low(l) => match l {
            DevCommands::Migration(m) => {
                m.handle(&connection);
            }
            DevCommands::Manufacturer(m) => {
                m.handle(&connection);
            }
            DevCommands::Part(p) => {
                p.handle(&connection);
            }
            DevCommands::Tagname(t) => {
                t.handle(&connection);
            }
            DevCommands::Tag(t) => {
                t.handle(&connection);
            }
            DevCommands::List => {
                println!("low level list is deprecated, use normal list instead");
            }
        },
        Commands::List(l) => {
            l.handle(&connection);
        }
        Commands::Add(a) => {
            a.handle(&connection);
        }
        Commands::Update(u) => {
            u.handle(&connection);
        }
        Commands::Tag(t) => {
            t.handle(&connection);
        }
    }
}
