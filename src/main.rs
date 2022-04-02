use clap::Parser;
use sled::Result;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
enum Args {
    Insert { key: String, val: i64 },
    List,
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
                let mut buf = [0u8; 8];
                buf.copy_from_slice(&val[0..8]);
                let val = u64::from_be_bytes(buf);
                println!("{key}: {val}");
            }

            Ok(())
        }
    }
}
