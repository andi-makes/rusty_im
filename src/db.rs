pub mod manufacturer;
pub mod part;
pub mod tag;
pub mod tagname;

pub mod migration;
pub mod schema;

use diesel::prelude::*;

#[derive(Debug)]
pub enum ConnectionError {
    DatabaseDoesNotExist { dbname: String },
    UserAuthFailed { username: String },
    UnknownAddress { address: String },
}

pub fn connect(url: &str) -> Result<SqliteConnection, ConnectionError> {
    let try_connection = SqliteConnection::establish(url);

    match try_connection {
        Ok(connection) => Ok(connection),
        Err(err) => match err {
            diesel::ConnectionError::InvalidCString(nulerror) => {
                eprintln!(
                    "Unexpected Nullbyte found in input! (How did you even manage to do this?)"
                );
                eprintln!("{}, Nullbyte @ index {}", url, nulerror.nul_position());
                std::process::exit(-1);
            }
            diesel::ConnectionError::BadConnection(err) => {
                eprintln!("Database returned the following error:");
                eprintln!("{}", err);
                if err.contains("database") && err.contains("does not exist") {
                    let dbname = err.split('"').nth(1).unwrap().to_string();
                    return Err(ConnectionError::DatabaseDoesNotExist { dbname: dbname });
                } else if err.contains("password authentication failed for user") {
                    let username = err.split('"').nth(1).unwrap().to_string();
                    return Err(ConnectionError::UserAuthFailed { username: username });
                } else if err.contains("could not translate host name") {
                    let address = err.split('"').nth(1).unwrap().to_string();
                    return Err(ConnectionError::UnknownAddress { address: address });
                }
                std::process::exit(-1);
            }
            diesel::ConnectionError::InvalidConnectionUrl(err) => {
                eprintln!(
                    "Database URL ({}) is invalid! (How did you even manage to do this?)",
                    url
                );
                eprintln!("{}", err);
                std::process::exit(-1);
            }
            diesel::ConnectionError::CouldntSetupConfiguration(err) => match err {
                diesel::result::Error::InvalidCString(nulerror) => {
                    eprintln!(
                        "Unexpected Nullbyte found in input! (How did you even manage to do this?)"
                    );
                    eprintln!("{}, Nullbyte @ index {}", url, nulerror.nul_position());
                    std::process::exit(-1);
                }
                diesel::result::Error::DatabaseError(kind, _) => match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => {
                        eprintln!("A unique constraint was violated.");
                        std::process::exit(-1);
                    }
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        eprintln!("A foreign key constraint was violated.");
                        std::process::exit(-1);
                    }
                    diesel::result::DatabaseErrorKind::UnableToSendCommand => {
                        eprintln!(
                            "The query could not be sent to the database due to a protocol violation.
                            An example of a case where this would occur is if you attempted to send
                            a query with more than 65000 bind parameters using PostgreSQL."
                        );
                        std::process::exit(-1);
                    }
                    diesel::result::DatabaseErrorKind::SerializationFailure => {
                        eprintln!(
                            "A serializable transaction failed to commit due to a read/write
                            dependency on a concurrent transaction.
                            Corresponds to SQLSTATE code 40001
                            This error is only detected for PostgreSQL, as we do not yet support
                            transaction isolation levels for other backends."
                        );
                        std::process::exit(-1);
                    }
                    _ => {
                        panic!("Unknown Configuration error!");
                    }
                },
                diesel::result::Error::NotFound => {
                    eprintln!(
                        "No rows were returned by a query expected to return at least one row
                        This variant is only returned by [`get_result`] and [`first`]. [`load`]
                        does not treat 0 rows as an error. If you would like to allow either 0
                        or 1 rows, call [`optional`] on the result.
                        [`get_result`]: ../query_dsl/trait.RunQueryDsl.html#method.get_result
                        [`first`]: ../query_dsl/trait.RunQueryDsl.html#method.first
                        [`load`]: ../query_dsl/trait.RunQueryDsl.html#method.load
                        [`optional`]: trait.OptionalExtension.html#tymethod.optional"
                    );
                    std::process::exit(-1);
                }
                diesel::result::Error::QueryBuilderError(_) => {
                    panic!("The query could not be constructed.");
                }
                diesel::result::Error::DeserializationError(_) => {
                    panic!("An error occurred deserializing the data being sent to the database.");
                }
                diesel::result::Error::SerializationError(_) => {
                    panic!("An error occurred serializing the data being sent to the database.");
                }
                _ => {
                    panic!("Rollbacktransaction, Already in transaction, unknown stuff");
                }
            },
            _ => {
                panic!("Unknown Database Connection Error!")
            }
        },
    }
}

use cli_table::Table;

pub fn cli_table_print_option_string(value: &Option<String>) -> impl std::fmt::Display {
    value.clone().unwrap_or(String::from("?"))
}

#[derive(Table, Queryable)]
pub struct ListTable {
    #[table(title = "ID")]
    id: i32,
    #[table(title = "Name")]
    name: String,
    #[table(title = "Amount")]
    amount: i32,
    #[table(title = "Manufacturer")]
    manufacturer: String,
    #[table(title = "Tag", display_fn = "cli_table_print_option_string")]
    tag: Option<String>,
    #[table(title = "Tag-Value", display_fn = "cli_table_print_option_string")]
    tag_value: Option<String>,
}

// fn <func_name>(value: &<type>) -> impl Display

pub fn list(connection: &SqliteConnection) -> Vec<ListTable> {
    use diesel::*;
    use schema::*;

    // parts LEFT JOIN manufacturers ON parts.manufacturer_id = manufacturers.id
    let join = parts::table.left_join(manufacturers::table);
    // "above" LEFT JOIN part_tag on parts.id = part_tag.part_id
    let join = join.left_join(part_tag::table.on(part_tag::dsl::part_id.eq(parts::dsl::id)));
    // "above" LEFT JOIN tags ON part_tag.tag_id = tags.id
    let join = join.left_join(tags::table.on(part_tag::dsl::tag_id.eq(tags::dsl::id)));
    // "above" LEFT JOIN tagnames ON tags.tagname_id = tagnames.id
    let join = join.left_join(tagnames::table.on(tags::dsl::tagname_id.eq(tagnames::dsl::id)));

    join.select((
        parts::id,
        parts::name,
        parts::amount,
        manufacturers::name,
        tagnames::name.nullable(),
        tags::value.nullable(),
    ))
    .load::<ListTable>(connection)
    .unwrap()
}
