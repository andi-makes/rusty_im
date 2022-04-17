use clap::Parser;
use sled::{Result, IVec};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
enum Args {
    Insert { key: String, val: i64 },
    List,
    #[clap(allow_hyphen_values=true)]
    Modify { key: String, val: i64 },
    Search { prefix: String },
}

fn amount_from_ivec(v: IVec) -> i64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&v[0..8]);
    i64::from_be_bytes(buf)
}

fn main() -> Result<()> {
    // clap command line parsing magic
    let args = Args::parse();

    // Get the database
    let db = sled::open("rimv1")?;

    match args {
        Args::Insert { key, val } => {
            // Insert the key:val pair
            db.insert(key.as_bytes(), &val.to_be_bytes())?;
            Ok(())
        }
        Args::List => {
            // List every available key:val pair
            for (key, val) in db.iter().flatten() {
                let key = String::from_utf8(key.to_vec()).unwrap();
                let val = amount_from_ivec(val);
                println!("{key}: {val}");
            }

            Ok(())
        }
        Args::Modify { key, val } => {
            if let Some(data) = db.get(key.as_bytes()).expect("DB Error") {
                let amount = amount_from_ivec(data);
                let new_amount = amount + val;

                db.insert(key.as_bytes(), &new_amount.to_be_bytes())?;
            } else {
                eprintln!("No Item named `{key}`, nothing changed.");
            }
            Ok(())
        }
        Args::Search { prefix } => {
            for (key, val) in db.scan_prefix(&prefix).flatten() {
                let key = String::from_utf8(key.to_vec()).unwrap();
                let val = amount_from_ivec(val);
                println!("{}: {}", key, val);
            }
            Ok(())
        }
    }
}
