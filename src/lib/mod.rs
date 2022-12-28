extern crate nom;
extern crate time;

mod cpp;
pub mod error;
mod format;
mod util;

use std::convert;
use std::path;

use crate::error::Error;
use crate::format::*;

////////////////////////////////////////////////////////////////////////

///
pub type Day = u8; // Day of the month
pub type Year = i32;

///
#[derive(Debug, PartialEq)]
pub enum Reminder {
    Weekly(time::Weekday),
    Monthly(Day, Option<Year>),
    Yearly(Day, time::Month),
    Date(time::Date),
}

impl Reminder {
    pub fn matches(&self, date: time::Date) -> bool {
        match self {
            Reminder::Weekly(wday) => date.weekday() == *wday,
            Reminder::Monthly(day, year) => {
                date.day() == *day && year.map(|y| date.year() == y).unwrap_or(true)
            }
            Reminder::Yearly(day, mon) => date.month() == *mon && date.day() == *day,
            Reminder::Date(d) => date == *d,
        }
    }
}

/// Represents a single appointment from the calendar file.
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub day: Reminder,
    pub desc: String,
    //pub time: time::Time,
}

impl Entry {
    pub fn is_fixed(&self) -> bool {
        match self.day {
            Reminder::Weekly(_) | Reminder::Monthly(_, _) => false,
            _ => true,
        }
    }
}

////////////////////////////////////////////////////////////////////////

pub fn parse_file<'a, P: convert::AsRef<path::Path>>(fp: P) -> Result<Vec<Entry>, Error> {
    let out = cpp::preprocess(fp)?;
    let (input, entries) = parse_entries(&out)?;
    if input != "" {
        Err(Error::IncompleteParse)
    } else {
        Ok(entries)
    }
}
