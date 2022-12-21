extern crate ncalendar;
extern crate structopt;

mod timespan;
mod util;

use crate::timespan::TimeSpan;
use crate::util::*;

use std::path;
use structopt::StructOpt;
use time::format_description;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Use the given file as the default calendar file.
    #[structopt(short = "f", parse(from_os_str))]
    file: Option<path::PathBuf>,

    /// Amount of next days to consider.
    #[structopt(short = "A", default_value = "1", parse(try_from_str = parse_days))]
    forward: time::Duration,

    /// Amount of past days to consider.
    #[structopt(short = "B", default_value = "0", parse(try_from_str = parse_days))]
    back: time::Duration,

    /// Act like the specified value is today.
    #[structopt(short = "t", parse(try_from_str = parse_today))]
    today: Option<time::Date>,

    /// Print day of the week name in front of each event.
    #[structopt(short = "w")]
    week: bool,
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
    let span = TimeSpan::new(today, opt.back, opt.forward).unwrap();

    let out_fmt = format_description::parse("[month repr:short] [day]").unwrap();
    let entries = ncalendar::parse_file(fp.as_path()).unwrap();
    for entry in entries {
        let postfix = if entry.is_reoccuring() { '*' } else { ' ' };

        if let Some(date) = span.match_reminder(entry.day) {
            if opt.week {
                print!("{} ", weekday_short(date));
            }
            println!(
                "{}{}\t{}",
                date.format(&out_fmt).unwrap(),
                postfix,
                entry.desc
            );
        }
    }
}
