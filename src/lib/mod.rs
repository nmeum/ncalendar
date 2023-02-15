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

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum WeekOffsetAmount {
    First = 1,
    Second,
    Third,
    Fourth,
    Fifth,
}

impl TryFrom<usize> for WeekOffsetAmount {
    type Error = ();

    fn try_from(v: usize) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(WeekOffsetAmount::First),
            2 => Ok(WeekOffsetAmount::Second),
            3 => Ok(WeekOffsetAmount::Third),
            4 => Ok(WeekOffsetAmount::Fourth),
            5 => Ok(WeekOffsetAmount::Fifth),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct WeekOffset {
    // Whether the offset is relative to the start or the end of the month.
    from_start: bool,

    // Offset in weeks.
    amount: WeekOffsetAmount,
}

impl TryFrom<i8> for WeekOffset {
    type Error = ();

    fn try_from(v: i8) -> Result<Self, Self::Error> {
        let amount: usize = v.abs() as usize;
        let amount: WeekOffsetAmount = amount.try_into()?;

        Ok(WeekOffset {
            from_start: v > 0,
            amount: amount,
        })
    }
}

impl WeekOffset {
    pub fn get(&self, days: Vec<time::Date>) -> Option<time::Date> {
        let off = self.amount as usize;
        let idx = if self.from_start {
            off - 1
        } else {
            if off > days.len() {
                return None;
            }
            days.len() - off
        };

        if idx >= days.len() {
            None
        } else {
            Some(days[idx])
        }
    }
}

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
            Reminder::SemiWeekly(wday, off) => weekday::filter(date.year(), date.month(), *wday)
                .map(|wdays| -> bool {
                    if date.weekday() != *wday {
                        false
                    } else {
                        off.get(wdays) == Some(date)
                    }
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
            Reminder::SemiWeekly(_, _) | Reminder::Weekly(_) | Reminder::Monthly(_, _) => false,
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
        let rem0 = Reminder::SemiWeekly(time::Weekday::Friday, 1.try_into().unwrap());
        assert!(rem0.matches(date!(2023 - 12 - 01)));
        assert!(rem0.matches(date!(2023 - 07 - 07)));

        let rem1 = Reminder::SemiWeekly(time::Weekday::Monday, 2.try_into().unwrap());
        assert!(rem1.matches(date!(2023 - 02 - 13)));
        assert!(!rem1.matches(date!(2023 - 02 - 06)));

        let rem2 = Reminder::SemiWeekly(time::Weekday::Sunday, 4.try_into().unwrap());
        assert!(rem2.matches(date!(2023 - 02 - 26)));
        assert!(!rem2.matches(date!(2023 - 02 - 05)));

        // Maximum negative
        let rem3 = Reminder::SemiWeekly(time::Weekday::Tuesday, (-5).try_into().unwrap());
        assert!(rem3.matches(date!(2023 - 05 - 02)));
        assert!(rem3.matches(date!(2023 - 08 - 01)));

        // Maximum positive
        let rem4 = Reminder::SemiWeekly(time::Weekday::Tuesday, 5.try_into().unwrap());
        assert!(rem4.matches(date!(2023 - 05 - 30)));
        assert!(rem4.matches(date!(2023 - 08 - 29)));
    }
}
