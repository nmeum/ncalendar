extern crate nom;
extern crate time;

mod format;
mod util;

use std::convert;
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

/// Represents a single appointment from the calendar file.
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub day: Reminder,
    pub desc: String,
    //pub time: time::Time,
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
