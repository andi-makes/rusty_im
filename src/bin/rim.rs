use rusty_im::cli;
use rusty_im::config;

fn main() {
    let path = config::check();
    cli::parse(&path);
}
