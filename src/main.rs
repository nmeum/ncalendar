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
    forward: u32,

    /// Amonut of past days to consider.
    #[structopt(short = "b", default_value = "0")]
    back: u32,
}

/// Represents a time span between two dates.
struct TimeSpan {
    start: time::Date,
    end: time::Date,
}

impl TimeSpan {
    pub fn new(day: time::Date, back: time::Duration, forward: time::Duration) -> Option<Self> {
        let start = day.checked_sub(back)?;
        let end = day.checked_add(forward)?;

        Some(TimeSpan { start, end })
    }

    pub fn contains(&self, d: time::Date) -> bool {
        d >= self.start && d <= self.end
    }

    pub fn contains_week(&self, w: time::Weekday) -> bool {
        let mut date = self.start;

        // Assume weekdays repeat every seven days.
        let mut days = 0;
        while days < 7 && date <= self.end {
            match date.checked_add(time::Duration::days(days)) {
                Some(ndate) => {
                    date = ndate;
                    if date.weekday() == w {
                        return true;
                    }
                }
                None => return false,
            }

            days += 1
        }

        false
    }
}

////////////////////////////////////////////////////////////////////////

fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

fn matches(t: &TimeSpan, e: &Entry) -> bool {
    match e.day {
        Reminder::Weekday(wday) => t.contains_week(wday),
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
    let backward = time::Duration::days(opt.back.into());
    let forward = time::Duration::days(opt.forward.into());
    let span = TimeSpan::new(today, backward, forward).unwrap();

    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    let filtered = entries.iter().filter(|entry| matches(&span, entry));

    // TODO: Filter entries using the matches method below and then print them.
    for entry in filtered {
        println!("Entry: {:?} - {:?}", entry.day, entry.desc);
    }
}
