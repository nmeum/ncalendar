use ncalendar::Reminder;
use time::util::days_in_year_month;

/// Represents a time span between two dates.
#[derive(Debug, PartialEq)]
pub struct TimeSpan {
    start: time::Date,
    end: time::Date,
}

/// Iterates over a time span by included days.
pub struct DayIterator<'a> {
    cur: &'a TimeSpan,
    off: i64, // offset in days
}

impl<'a> Iterator for DayIterator<'a> {
    type Item = time::Date;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = &self.cur;
        match cur.start.checked_add(time::Duration::days(self.off)) {
            Some(ndate) => {
                if ndate > cur.end {
                    return None;
                }
                self.off += 1;
                return Some(ndate);
            }
            None => return None,
        }
    }
}

/// Iterates over a time span by included months.
/// In each iteration a time span for the month is returned.
pub struct MonthIterator<'a> {
    cur: &'a TimeSpan,
    off: time::Duration, // offset
}

impl<'a> Iterator for MonthIterator<'a> {
    type Item = TimeSpan;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = &self.cur;
        match cur.start.checked_add(self.off) {
            Some(ndate) => {
                if ndate > cur.end {
                    return None
                }

                if ndate.month() == cur.end.month() && ndate.year() == cur.end.year() {
                    let diff = cur.end - ndate;
                    // Increment off beyond cur.end to return None on next iteration.
                    self.off += diff + time::Duration::days(1);

                    return Some(TimeSpan{
                        start: ndate,
                        end: self.cur.end,
                    })
                }

                // Duration to reach end of month from new date.
                let days_to_end = days_in_year_month(ndate.year(), ndate.month()) - ndate.day();
                let to_end = time::Duration::days(days_to_end.into());

                self.off += to_end + time::Duration::days(1);
                ndate.checked_add(to_end)
                    .map(|d| TimeSpan{ start: ndate, end: d })
            }
            None => return None,
        }
    }
}

impl TimeSpan {
    // TODO: Make back and forward days, not durations.
    pub fn new(day: time::Date, back: time::Duration, forward: time::Duration) -> Option<Self> {
        let start = day.checked_sub(back)?;
        let end = day.checked_add(forward)?;

        Some(TimeSpan { start, end })
    }

    pub fn from_dates(start: time::Date, end: time::Date) -> TimeSpan {
        TimeSpan{start, end}
    }

    pub fn contains_date(&self, d: time::Date) -> bool {
        d >= self.start && d <= self.end
    }

    /// Check if the time span contains the given weekday and if so
    /// returns the first date for this weekday in the time span.
    pub fn find_weekday(&self, w: time::Weekday) -> Option<time::Date> {
        for (days, date) in self.iter().enumerate() {
            // Assume weekdays repeat every seven days.
            if days >= 7 {
                return None;
            } else if date.weekday() == w {
                return Some(date);
            }
        }

        None
    }

    /// Check if the given reminder is matched by the time span and
    /// if so return the first date in the time span that matches it.
    pub fn match_reminder(&self, r: Reminder) -> Option<time::Date> {
        match r {
            Reminder::Yearly(d, m) => {
                // TODO: fast path, i.e. check if months is even in time span?
                // Also: Don't iterate over multiple years, stop after 1 year.
                // Ideally iterate over months and then days in the month.
                //
                // See: days_in_year_month from time.rs
                for date in self.iter() {
                    if date.day() == d && date.month() == m {
                        return Some(date)
                    }
                }

                None
            }
            Reminder::Weekly(w) => {
                if let Some(d) = self.find_weekday(w) {
                    Some(d)
                } else {
                    None
                }
            }
            Reminder::Date(d) => {
                if !self.contains_date(d) {
                    None
                } else {
                    Some(d)
                }
            }
        }
    }

    pub fn months(&self) -> MonthIterator {
        return MonthIterator { cur: self, off: time::Duration::days(0) };
    }

    /// Iterate over all days in the given time span.
    pub fn iter(&self) -> DayIterator {
        return DayIterator { cur: self, off: 0 };
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;
    use time::Duration;

    #[test]
    fn contains_date() {
        let t = TimeSpan::new(date!(2022 - 12 - 20), Duration::days(5), Duration::days(4)).unwrap();

        // Lower bound: 15th of December 2022
        assert!(t.contains_date(date!(2022 - 12 - 15)));
        assert!(!t.contains_date(date!(2022 - 12 - 14)));

        // Upper bound: 24th of December 2022
        assert!(t.contains_date(date!(2022 - 12 - 24)));
        assert!(!t.contains_date(date!(2022 - 12 - 25)));
    }

    #[test]
    fn find_weekday() {
        let d = date!(2000 - 06 - 13);
        println!("Weekday: {:?}", d.weekday());

        let t = TimeSpan::new(d, Duration::days(0), Duration::days(3)).unwrap();
        assert_eq!(t.find_weekday(time::Weekday::Monday), None);
        assert_eq!(
            t.find_weekday(time::Weekday::Tuesday),
            Some(date!(2000 - 06 - 13))
        );
        assert_eq!(
            t.find_weekday(time::Weekday::Wednesday),
            Some(date!(2000 - 06 - 14))
        );
        assert_eq!(
            t.find_weekday(time::Weekday::Thursday),
            Some(date!(2000 - 06 - 15))
        );
        assert_eq!(
            t.find_weekday(time::Weekday::Friday),
            Some(date!(2000 - 06 - 16))
        );
        assert_eq!(t.find_weekday(time::Weekday::Saturday), None);
        assert_eq!(t.find_weekday(time::Weekday::Sunday), None);
    }

    #[test]
    fn day_iterator() {
        let d = date!(1980 - 03 - 20);
        let t = TimeSpan::new(d, time::Duration::days(0), time::Duration::days(1)).unwrap();

        let mut it: DayIterator = t.iter();
        assert_eq!(it.next(), Some(date!(1980 - 03 - 20)));
        assert_eq!(it.next(), Some(date!(1980 - 03 - 21)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn month_iterator() {
        let d = date!(2022 - 12 - 01);
        let t = TimeSpan::new(d, time::Duration::days(0), time::Duration::days(100)).unwrap();
        println!("t: {:?}", t);

        let months: Vec<TimeSpan> = t.months().collect();
        assert_eq!(
            vec![
                TimeSpan::from_dates(date!(2022 - 12 - 01), date!(2022 - 12 - 31)),
                TimeSpan::from_dates(date!(2023 - 01 - 01), date!(2023 - 01 - 31)),
                TimeSpan::from_dates(date!(2023 - 02 - 01), date!(2023 - 02 - 28)),
                TimeSpan::from_dates(date!(2023 - 03 - 01), date!(2023 - 03 - 11)),
            ],
            months
        );
    }
}
