use ncalendar::Reminder;

/// Represents a time span between two dates.
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

impl TimeSpan {
    pub fn new(day: time::Date, back: time::Duration, forward: time::Duration) -> Option<Self> {
        let start = day.checked_sub(back)?;
        let end = day.checked_add(forward)?;

        Some(TimeSpan { start, end })
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
    fn iterator() {
        let d = date!(1980 - 03 - 20);
        let t = TimeSpan::new(d, time::Duration::days(0), time::Duration::days(1)).unwrap();

        let mut it: DayIterator = t.iter();
        assert_eq!(it.next(), Some(date!(1980 - 03 - 20)));
        assert_eq!(it.next(), Some(date!(1980 - 03 - 21)));
        assert_eq!(it.next(), None);
    }
}
