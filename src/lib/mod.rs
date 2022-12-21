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

impl Entry {
    pub fn is_reoccuring(&self) -> bool {
        match self.day {
            Reminder::Weekday(_) => true,
            _ => false,
        }
    }
}

////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub enum Error {
    IncompleteParse,
    ParsingError(String, nom::error::ErrorKind),
    IoError(io::Error),
}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(e: nom::Err<nom::error::Error<&str>>) -> Self {
        match e {
            nom::Err::Incomplete(_) => Error::IncompleteParse,
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                Error::ParsingError(e.input.to_string(), e.code)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IoError(e)
    }
}

pub fn parse_file<'a, P: convert::AsRef<path::Path>>(fp: P) -> Result<Vec<Entry>, Error> {
    let mut f = File::open(fp)?;

    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let (input, entries) = parse_entries(&buf)?;
    if input != "" {
        Err(Error::IncompleteParse)
    } else {
        Ok(entries)
    }
}
