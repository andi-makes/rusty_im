use rusty_im::cli;
use rusty_im::db;

fn main() {
    let connection = db::connect();
    cli::parse(&connection);
}
