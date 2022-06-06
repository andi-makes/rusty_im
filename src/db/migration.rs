use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/res"]
struct ConfigurationFolder;
struct Mig {
    up: String,
    down: String,
}

impl Mig {
    fn from(up: String, down: String) -> Mig {
        Mig { up, down }
    }
}

fn get_content(path: &str) -> String {
    let file =
        ConfigurationFolder::get(path).expect("Could not get SQL configuration file. Aborting.");
    std::str::from_utf8(file.data.as_ref())
        .expect("Could not read the contents of the SQL configuration file. Aborting.\nError: ")
        .to_string()
}

fn get_migrations() -> Vec<Mig> {
    let up_paths: Vec<_> = ConfigurationFolder::iter()
        .filter(|x| x.to_string().ends_with("up.sql"))
        .collect();
    let down_paths: Vec<_> = ConfigurationFolder::iter()
        .filter(|x| x.to_string().ends_with("down.sql"))
        .collect();

    assert_eq!(
        up_paths.len(),
        down_paths.len(),
        "There must be an equal amount of up- and down-Sql-files"
    );
    let mut migs: Vec<Mig> = Vec::new();
    for i in 0..up_paths.len() {
        // The following 2 unwraps are safe. We already checked the size of the vector, we cannot go out of bounds
        migs.push(Mig::from(
            get_content(up_paths.get(i).unwrap()),
            get_content(down_paths.get(i).unwrap()),
        ));
    }
    migs
}

pub fn print_fs() {
    for file_name in ConfigurationFolder::iter() {
        // Unwrapping here is safe, we are providing filenames that are guaranteed to exist
        let file = ConfigurationFolder::get(file_name.as_ref()).unwrap();
        println!("{}:", file_name.as_ref());
        println!(
            "{}",
            std::str::from_utf8(file.data.as_ref())
                .expect("Could not read the contents of the file. Aborting.\nError: ")
        );
    }
}

pub fn run(connection: &dyn diesel::connection::SimpleConnection) {
    let migs = get_migrations();

    for mig in migs {
        connection
            .batch_execute(&mig.up)
            .expect("Migration failed!");
    }
}

pub fn revert(connection: &dyn diesel::connection::SimpleConnection) {
    let migs = get_migrations();

    for mig in migs {
        connection
            .batch_execute(&mig.down)
            .expect("Migration failed!");
    }
}

pub fn redo(connection: &dyn diesel::connection::SimpleConnection) {
    revert(connection);
    run(connection);
}
