use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/res"]
struct Files;
struct Mig {
    up: Option<String>,
    down: Option<String>,
}

impl Mig {
    fn from(up: String, down: String) -> Mig {
        Mig {
            up: Option::from(up),
            down: Option::from(down),
        }
    }
}

fn get_content(path: &str) -> String {
    let file = Files::get(path).unwrap();
    std::str::from_utf8(file.as_ref()).unwrap().to_string()
}

fn get_migrations() -> Vec<Mig> {
    let up_paths: Vec<_> = Files::iter()
        .filter(|x| x.to_string().ends_with("up.sql"))
        .collect();
    let down_paths: Vec<_> = Files::iter()
        .filter(|x| x.to_string().ends_with("down.sql"))
        .collect();

    assert_eq!(
        up_paths.len(),
        down_paths.len(),
        "There must be an equal amount of up- and down-Sql-files"
    );
    let mut migs: Vec<Mig> = Vec::new();
    for i in 0..up_paths.len() {
        migs.push(Mig::from(
            get_content(up_paths.get(i).unwrap()),
            get_content(down_paths.get(i).unwrap()),
        ));
    }
    migs
}

pub fn print_fs() {
    for file_name in Files::iter() {
        let file = Files::get(file_name.as_ref()).unwrap();
        println!("{}:", file_name.as_ref());
        println!("{}", std::str::from_utf8(file.as_ref()).unwrap());
    }
}

pub fn run(connection: &dyn diesel::connection::SimpleConnection) {
    let migs = get_migrations();

    for mig in migs {
        connection
            .batch_execute(&mig.up.unwrap())
            .expect("Migration failed!");
    }
}

pub fn revert(connection: &dyn diesel::connection::SimpleConnection) {
    let migs = get_migrations();

    for mig in migs {
        connection
            .batch_execute(&mig.down.unwrap())
            .expect("Migration failed!");
    }
}

pub fn redo(connection: &dyn diesel::connection::SimpleConnection) {
    revert(connection);
    run(connection);
}
