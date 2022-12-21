extern crate ncalendar;
extern crate structopt;

use ncalendar::{Entry, Reminder};
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
    #[structopt(short = "a", default_value = "1")]
    num: u32,
}

#[derive(PartialEq)]
enum Direction {
    Future,
    Past,
}

/// Represents a time span between two dates.
struct TimeSpan {
    start: time::Date,
    end: time::Date,
}

impl TimeSpan {
    pub fn new(day: time::Date, dur: time::Duration, dir: Direction) -> Option<Self> {
        let (start, end) = if dir == Direction::Future {
            (day, day.checked_add(dur)?)
        } else {
            (day.checked_sub(dur)?, day)
        };

        Some(TimeSpan { start, end })
    }

    pub fn contains(&self, d: time::Date) -> bool {
        d >= self.start && d <= self.end
    }
}

////////////////////////////////////////////////////////////////////////

fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

/// Check if entry matches for the given date (assume today for now).
fn matches(t: &TimeSpan, e: &Entry) -> bool {
    match e.day {
        Reminder::Weekday(_wday) => {
            // TODO: Requires reasoning about WeekDay + 1
            // but time doesn't implement Ord or PartialOrd.
            false
        }
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

    let today = time::OffsetDateTime::now_local().unwrap().date();
    let duration = time::Duration::days(opt.num.into());
    let span = TimeSpan::new(today, duration, Direction::Future).unwrap();

    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    let filtered = entries.iter().filter(|entry| matches(&span, entry));

    // TODO: Filter entries using the matches method below and then print them.
    for entry in filtered {
        println!("Entry: {:?} - {:?}", entry.day, entry.desc);
    }
}
