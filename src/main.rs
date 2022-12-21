extern crate ncalendar;
extern crate structopt;

mod timespan;

use crate::timespan::TimeSpan;
use ncalendar::{Entry, Reminder};
use std::env;
use std::path::{self, Path};
use structopt::StructOpt;
use time::macros::format_description;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Use the given file as the default calendar file.
    #[structopt(short = "f", parse(from_os_str))]
    file: Option<path::PathBuf>,

    /// Amount of next days to consider.
    #[structopt(short = "A", default_value = "1")]
    forward: u32,

    /// Amonut of past days to consider.
    #[structopt(short = "B", default_value = "0")]
    back: u32,

    /// Act like the specified value is today.
    #[structopt(short = "t", parse(try_from_str = parse_today))]
    today: Option<time::Date>,
}

////////////////////////////////////////////////////////////////////////

fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

fn parse_today(input: &str) -> Result<time::Date, time::error::Parse> {
    let fmt = format_description!("[day][month][year]");
    time::Date::parse(input, &fmt)
}

fn matches(t: &TimeSpan, e: &Entry) -> bool {
    match e.day {
        Reminder::Weekday(wday) => t.contains_weekday(wday),
        Reminder::Date(date) => t.contains(date),
    }
}

fn main() {
    let opt = Opt::from_args();
    let fp = if let Some(p) = opt.file {
        p
    } else {
        calendar_file().unwrap()
    };

    let today = if let Some(t) = opt.today {
        t
    } else {
        time::OffsetDateTime::now_local().unwrap().date()
    };
    let backward = time::Duration::days(opt.back.into());
    let forward = time::Duration::days(opt.forward.into());
    let span = TimeSpan::new(today, backward, forward).unwrap();

    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    let filtered = entries.iter().filter(|entry| matches(&span, entry));

    // TODO: Filter entries using the matches method below and then print them.
    for entry in filtered {
        println!("{}", entry);
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn today_parser() {
        assert_eq!(parse_today("02012022"), Ok(date!(2022 - 01 - 02)));
        assert_eq!(parse_today("12122000"), Ok(date!(2000 - 12 - 12)));
    }
}
