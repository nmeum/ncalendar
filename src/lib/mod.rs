extern crate nom;
extern crate time;

mod cpp;
mod error;
mod format;
mod util;

use std::convert;
use std::path;

use crate::error::Error;
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

impl Entry {
    pub fn is_reoccuring(&self) -> bool {
        match self.day {
            Reminder::Weekday(_) => true,
            _ => false,
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
