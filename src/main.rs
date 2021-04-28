use rusty_im::cli;
use rusty_im::config;
use rusty_im::db;

fn main() {
    let connection = db::connect(config::get_database_connection_url().as_str()).unwrap();
    cli::parse(&connection);
}
