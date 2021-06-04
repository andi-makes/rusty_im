pub fn check() -> String {
    let dirs = match directories::ProjectDirs::from("dev.andi-makes.rim", "andi-makes", "rim") {
        Some(d) => d,
        None => {
            println!("Couldn't find a suited directory for storing the database file. Aborting.");
            std::process::exit(-1);
        }
    };

    let data_dir_slice = match dirs.data_dir().to_str() {
        Some(d) => d,
        None => {
            eprintln!("Path to data directory isn't valid unicode.");
            std::process::exit(-1);
        }
    };

    let db_file = format!("{}/rusty.db", data_dir_slice);
    if !dirs.data_dir().exists() {
        println!(
            "First time setup, creating project data directory @ {}...",
            data_dir_slice
        );
        match std::fs::create_dir_all(dirs.data_dir()) {
            Ok(_) => {
                println!("Created data directory @ {}.", data_dir_slice);
            }
            Err(_) => {
                eprintln!(
                    "Could not create the projects data directory @ {}.\nMake sure you have the required permissions.",
                    data_dir_slice
                );
                std::process::exit(-1);
            }
        }
        println!("Setting up the database...");
        let connection = super::db::connect(&db_file);
        super::db::migration::run(&connection);
        println!("Successfully setup the database!");
    }

    db_file
}
