# The `rusty inventory manager`

Simple inventory manager written in `rust`.

## Building
### Prerequisits
You need a PostgreSQL Database.
Create a `.env` file with the following contents:
```
DATABASE_URL=postgres://{username}:{password}@{url}/{database_name}
```
This program uses the `diesel` crate. Currently, you need to manage the diesel
migrations yourself. You need to install `diesel-cli` with at least the 
`postgres` feature. Click 
[here](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) for further 
information about installing `diesel-cli`.

### Configuring the database
Execute the following commands:
```
diesel setup
diesel migration run
```

### Building the program
```
cargo build --release
```
The resulting binary will be located in `target/release/rusty_im`.