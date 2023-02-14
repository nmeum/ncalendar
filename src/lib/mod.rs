extern crate nom;
extern crate time;

mod cpp;
pub mod error;
mod format;
mod util;
mod weekday;

use std::convert;
use std::path;

use crate::error::Error;
use crate::format::*;

////////////////////////////////////////////////////////////////////////

///
pub type Day = u8; // Day of the month
pub type Year = i32;
pub type WeekOffset = i8; // -4...+5

///
#[derive(Debug, PartialEq)]
pub enum Reminder {
    Weekly(time::Weekday),
    SemiWeekly(time::Weekday, WeekOffset),
    Monthly(Day, Option<Year>),
    Yearly(Day, time::Month),
    Date(time::Date),
}

impl Reminder {
    pub fn matches(&self, date: time::Date) -> bool {
        match self {
            Reminder::Weekly(wday) => date.weekday() == *wday,
            Reminder::SemiWeekly(wday, xoff) => weekday::filter(date.year(), date.month(), *wday)
                .map(|wdays: Vec<time::Date>| -> bool {
                    let off = xoff - 1;
                    let idx: usize = if off < 0 {
                        wdays.len() - (off as usize)
                    } else {
                        off as usize
                    };

                    if idx >= wdays.len() {
                        return false;
                    }
                    wdays[idx] == date
                })
                .unwrap_or(false),
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

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn match_semiweekly() {
        let rem1 = Reminder::SemiWeekly(time::Weekday::Monday, 2);
        assert!(rem1.matches(date!(2023 - 02 - 13)));
        assert!(!rem1.matches(date!(2023 - 02 - 06)));

        let rem2 = Reminder::SemiWeekly(time::Weekday::Sunday, 4);
        assert!(rem2.matches(date!(2023 - 02 - 26)));
        assert!(!rem2.matches(date!(2023 - 02 - 05)));
    }
}
