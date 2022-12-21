extern crate nom;
extern crate time;

mod format;
mod util;

use std::convert;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::path;

use crate::format::*;

////////////////////////////////////////////////////////////////////////

///
pub type Day = u8;
pub type Year = i32;

///
#[derive(Debug, PartialEq)]
pub enum Reminder {
    Weekday(time::Weekday),
    Date(time::Date),
}

impl fmt::Display for Reminder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reminder::Weekday(wday) => write!(f, "{}", wday),
            Reminder::Date(date) => write!(f, "{}", date),
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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.day, self.desc)
    }
}

////////////////////////////////////////////////////////////////////////

pub fn parse_file<'a, P: convert::AsRef<path::Path>>(fp: P) -> io::Result<Vec<Entry>> {
    let mut f = File::open(fp)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    // TODO: error handling
    let (input, entries) = parse_entries(&buf).unwrap();
    if input != "" {
        // TODO: handle incomplete parse
    }

    Ok(entries)
}
