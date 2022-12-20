extern crate ncalendar;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error: Supply exactly one argument");
    }

    let entries = ncalendar::parse_file(&args[1]).unwrap();
    for entry in entries.iter() {
        println!("Entry: {:?} - {:?}", entry.day, entry.desc);
    }
}
