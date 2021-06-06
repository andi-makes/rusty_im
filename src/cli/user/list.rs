use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Action {
    /// Lists all parts
    Parts,
    /// Lists all manufacturers
    Manufacturers,
    /// Lists all Tags
    Tags,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        use cli_table::WithTitle;
        match self {
            Action::Parts => {
                cli_table::print_stdout(db::list(&connection).with_title())
                    .expect("Could not print to stdout! Aborting.\nError: ");
            }
            Action::Manufacturers => {
                cli_table::print_stdout(db::manufacturer::get(&connection).with_title())
                    .expect("Could not print to stdout! Aborting.\nError: ");
            }
            Action::Tags => {
                use cli_table::Table;
                use db::cli_table_print_option_string;
                use db::schema::*;
                use diesel::*;

                #[derive(Table, Queryable)]
                struct TagTable {
                    #[table(title = "ID")]
                    id: i32,
                    #[table(title = "Name", display_fn = "cli_table_print_option_string")]
                    name: Option<String>,
                    #[table(title = "Value", display_fn = "cli_table_print_option_string")]
                    value: Option<String>,
                }

                let table = tags::table
                    .left_join(tagnames::table)
                    .select((tags::id, tagnames::name.nullable(), tags::value.nullable()))
                    .load::<TagTable>(connection)
                    .expect("Could not get the joined tag table. Aborting.\nError: ");

                cli_table::print_stdout(table.with_title())
                    .expect("Could not print to stdout! Aborting.\nError: ");

                let possible_tagnames = db::tagname::get(&connection);
                print!("Possible Tagnames are: ");
                let mut first = true;
                for tagname in possible_tagnames {
                    if !first {
                        print!(", {}", tagname.name);
                    } else {
                        print!("{}", tagname.name);
                        first = false;
                    }
                }
                println!();
            }
        }
    }
}
