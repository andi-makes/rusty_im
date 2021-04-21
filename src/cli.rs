use crate::db;
use structopt::StructOpt;

mod manufacturer;
mod migration;
mod part;
mod tag;
mod tagname;

pub trait CommandHandler {
    fn handle(&self, connection: &db::PgConnection);
}

#[derive(StructOpt)]
enum Commands {
    Migration(migration::Migration),
    Manufacturer(manufacturer::Action),
    Part(part::Action),
    Tagname(tagname::Action),
    Tag(tag::Action),
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(subcommand)]
    subcommand: Commands,
}

pub fn parse(connection: &db::PgConnection) {
    let args = Cli::from_args();

    match args.subcommand {
        Commands::Migration(m) => m.handle(&connection),
        Commands::Manufacturer(m) => m.handle(&connection),
        Commands::Part(p) => p.handle(&connection),
        Commands::Tagname(t) => t.handle(&connection),
        Commands::Tag(t) => t.handle(&connection),
    }
}
