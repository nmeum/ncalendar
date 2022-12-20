extern crate ncalendar;

use std::env;
use std::path::{self, Path};

fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path
        .join(".ncalendar")
        .join("calendar"))
}

fn main() {
    let fp = calendar_file().unwrap();

    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    for entry in entries.iter() {
        println!("Entry: {:?} - {:?}", entry.day, entry.desc);
    }
}
