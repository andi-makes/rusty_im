use crate::cli::CommandHandler;
use crate::db;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum PartUpdateFields {
    /// New stock of the existing part
    Amount { value: i32 },
    /// New description of the existing part
    Description { value: String },
}

#[derive(StructOpt)]
pub enum Action {
    /// Adds a new part
    Add {
        /// id of the manufacturer of the new part
        manufacturer_id: i32,
        /// name of the new part
        name: String,
        /// Initial stock of the new part, defaults to zero
        #[structopt(short = "a", long = "amount")]
        amount: Option<i32>,
        /// description of the new part
        #[structopt(short = "d", long = "description")]
        description: Option<String>,
    },
    /// Updates an existing parts stock based by its id
    Update {
        /// id of the manufacturer to be updated
        id: i32,
        #[structopt(subcommand)]
        field: PartUpdateFields,
    },
    /// Deletes an exisitng part based by its id
    Delete {
        /// id of the part to be deleted
        id: i32,
    },
    /// Lists all parts
    List,
}

impl CommandHandler for Action {
    fn handle(&self, connection: &diesel::SqliteConnection) {
        match &self {
            Action::Add {
                manufacturer_id,
                name,
                description,
                amount,
            } => db::part::insert(
                &connection,
                *manufacturer_id,
                name,
                description,
                amount.unwrap_or(0),
            ),
            Action::Delete { id } => db::part::delete(&connection, *id),
            Action::List => {
                println!("Deprecated, use list instead");
            }
            Action::Update { id, field } => match field {
                PartUpdateFields::Amount { value } => {
                    db::part::update_amount(&connection, *id, *value)
                }
                PartUpdateFields::Description { value } => {
                    db::part::update_description(&connection, *id, value.to_string())
                }
            },
        }
    }
}
