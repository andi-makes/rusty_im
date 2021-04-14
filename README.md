# The `rusty inventory manager`

Simple inventory manager written in `rust`.

## TODO:
 - [ ] First use setup wizard
 - [ ] Parts database and interface 
 - [ ] Tags database and interface
 - [ ] .config files instead of .env

## Building
### Prerequisits
You need a PostgreSQL Database.
Create a `.env` file with the following contents:
```
DATABASE_URL=postgres://{username}:{password}@{url}/{database_name}
```
Make sure the user and the database exist.

### Building the program
```
cargo build --release
```
The resulting binary will be located in `target/release/rusty_im`.

### Configuring the database
Execute the following command:
```
rusty_im migration run
```

Now you can start using this program.