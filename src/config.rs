use directories::ProjectDirs;
use std::fs::{self, File};
use std::io::prelude::*;
use std::{env, path::PathBuf};

fn ask_input(promt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", promt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    String::from(buffer.trim())
}

fn get_config_file() -> PathBuf {
    let config = match ProjectDirs::from("dev", "andi-makes", "rusty_im") {
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
        f.write_all(b"export RIM_CONFIGURED=false").unwrap();
    }
    let mut path = config.config_dir().to_path_buf();
    path.push("rusty.env");
    path
}

pub fn wizard() {
    println!("Currently off-topic");
    return;

    println!("Welcome to the Rusty Inventory Manager Setup Wizard.\nFirst of all, we need the Address to a PostgreSQL server.");
    let db_ip = ask_input("[Database Address]: ");
    println!("Please choose an Username that will be used to access the Database.");
    let db_username = ask_input("[Username]: ");
    println!("Please enter the Password for that user.");
    let db_password = ask_input("[Password]: ");
    println!("Please enter the name of the Database.");
    let db_name = ask_input("[Database Name]: ");

    let db_url = format!(
        "postgres://{}:{}@{}/{}",
        db_username, db_password, db_ip, db_name
    );
    println!("Crafted DB URL: {}", db_url);

    match super::db::connect(&db_url) {
        std::result::Result::Ok(_) => {}
        std::result::Result::Err(err) => match err {
            crate::db::ConnectionError::DatabaseDoesNotExist { dbname } => {
                println!("Do you want to create a database named  `{}`?", dbname);
                let result = ask_input("Yes, no or rename? [Y/n/r] ");
                match result.to_lowercase().as_str() {
                    "n" => {
                        eprintln!("please create a database named `{}` yourself and re-execute the wizard", dbname);
                        std::process::exit(-1);
                    }
                    "r" => {
                        todo!("rename");
                    }
                    "" | "y" => {
                        todo!("Create db");
                    }
                    _ => {
                        panic!("Non valid character");
                    }
                }
                println!("{}", result);
            }
            crate::db::ConnectionError::UserAuthFailed { username } => {
                todo!("Create user {} or change credentials", username)
            }
            crate::db::ConnectionError::UnknownAddress { address: _ } => {
                todo!("Wrong Address!")
            }
        },
    }

    let mut config_file = File::create(get_config_file()).unwrap();
    config_file
        .write_fmt(format_args!(
        "export RIM_CONFIGURED=true\nexport RIM_IP={}\nexport RIM_USERNAME={}\nexport RIM_PASSWORD={}\nexport RIM_DB_NAME={}",
        db_ip, db_username, db_password, db_name
    ))
        .unwrap();
}

fn get_db_url() -> String {
    // dotenv::from_path(get_config_file()).unwrap();
    // let ip = env::var("RIM_IP").unwrap();
    // let username = env::var("RIM_USERNAME").unwrap();
    // let password = env::var("RIM_PASSWORD").unwrap();
    // let name = env::var("RIM_DB_NAME").unwrap();
    // format!("postgres://{}:{}@{}/{}", username, password, ip, name)
    // TODO: no hardcoding here!
    String::from("rusty.db")
}

pub fn get_database_connection_url() -> String {
    dotenv::from_path(get_config_file()).unwrap();
    match env::var("RIM_CONFIGURED") {
        Ok(val) => {
            if val.to_lowercase().trim().eq("false") {
                wizard();
            }
        }
        Err(_) => {
            eprintln!("Corrupted Config File, redirecting to wizard...");
            wizard();
        }
    }
    get_db_url()
}
