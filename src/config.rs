use directories::ProjectDirs;
use std::env;
use std::fs::{self, File};
use std::io::prelude::*;

pub fn get_database_connection_url() -> String {
    let config = match ProjectDirs::from("at", "andi-makes", "rusty_im") {
        Some(a) => a,
        None => panic!("Cannot get project dir"),
    };

    let env_config = config.config_dir().join("rusty.env");

    // Create the config file if it doesn't exist
    if !env_config.exists() {
        fs::create_dir_all(config.config_dir()).unwrap();
        let mut f = match File::create(env_config.clone()) {
            Ok(a) => a,
            Err(e) => {
                panic!("Error: {}", e);
            }
        };
        f.write_all(b"configured=false").unwrap();
    }

    dotenv::from_path(env_config).unwrap();
    match env::var("configured") {
        Ok(val) => {
            if val.to_lowercase().trim().eq("false") {
                todo!("Setup Wizard");
            }
        }
        Err(_) => {
            panic!("Corrupted configuration file")
        }
    }
    env::var("DATABASE_URL").unwrap()
}
