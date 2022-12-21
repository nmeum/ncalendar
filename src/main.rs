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
    #[structopt(short = "A", parse(try_from_str = parse_days))]
    forward: Option<time::Duration>,

    /// Amount of past days to consider.
    #[structopt(short = "B", parse(try_from_str = parse_days))]
    back: Option<time::Duration>,

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

    // For Fridays (if neither -A nor -B was provided) look
    // three days into the future by default (next monday).
    let forward;
    if today.weekday() == time::Weekday::Friday && opt.forward.is_none() && opt.back.is_none() {
        forward = time::Duration::days(3);
    } else {
        forward = time::Duration::days(1);
    }

    let span = TimeSpan::new(
        today,
        opt.back.unwrap_or(time::Duration::days(0)),
        opt.forward.unwrap_or(forward),
    )
    .unwrap();

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
