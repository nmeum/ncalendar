extern crate ncalendar;
extern crate structopt;

use std::env;
use std::path::{self, Path};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Use the given file as the default calendar file.
    #[structopt(short = "f", parse(from_os_str))]
    file: Option<path::PathBuf>,

    /// Amount of next days to consider.
    #[structopt(short = "a", default_value = "0")]
    num: u32,
}

fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

fn main() {
    let opt = Opt::from_args();
    let fp = if let Some(p) = opt.file {
        p
    } else {
        calendar_file().unwrap()
    };

    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    for entry in entries.iter() {
        println!("Entry: {:?} - {:?}", entry.day, entry.desc);
    }
}
