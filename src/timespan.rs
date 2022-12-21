/// Represents a time span between two dates.
pub struct TimeSpan {
    start: time::Date,
    end: time::Date,
}

impl TimeSpan {
    pub fn new(day: time::Date, back: time::Duration, forward: time::Duration) -> Option<Self> {
        let start = day.checked_sub(back)?;
        let end = day.checked_add(forward)?;

        Some(TimeSpan { start, end })
    }

    pub fn contains(&self, d: time::Date) -> bool {
        d >= self.start && d <= self.end
    }

    /// Check if the time span contains the given weekday and if so
    /// returns the first date for this weekday in the time span.
    pub fn find_weekday(&self, w: time::Weekday) -> Option<time::Date> {
        let date = self.start;

        // Assume weekdays repeat every seven days.
        let mut days = 0;
        while days < 7 {
            match date.checked_add(time::Duration::days(days)) {
                Some(ndate) => {
                    if ndate > self.end {
                        return None;
                    } else if ndate.weekday() == w {
                        return Some(ndate);
                    }
                }
                None => return None,
            }

            days += 1
        }

        None
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;
    use time::Duration;

    #[test]
    fn contains() {
        let t = TimeSpan::new(date!(2022 - 12 - 20), Duration::days(5), Duration::days(4)).unwrap();

        // Lower bound: 15th of December 2022
        assert!(t.contains(date!(2022 - 12 - 15)));
        assert!(!t.contains(date!(2022 - 12 - 14)));

        // Upper bound: 24th of December 2022
        assert!(t.contains(date!(2022 - 12 - 24)));
        assert!(!t.contains(date!(2022 - 12 - 25)));
    }

    #[test]
    fn find_weekday() {
        let d = date!(2000 - 06 - 13);
        println!("Weekday: {:?}", d.weekday());

        let t = TimeSpan::new(d, Duration::days(0), Duration::days(3)).unwrap();
        assert_eq!(t.find_weekday(time::Weekday::Monday), None);
        assert_eq!(t.find_weekday(time::Weekday::Tuesday), Some(date!(2000 - 06 - 13)));
        assert_eq!(t.find_weekday(time::Weekday::Wednesday), Some(date!(2000 - 06 - 14)));
        assert_eq!(t.find_weekday(time::Weekday::Thursday), Some(date!(2000 - 06 - 15)));
        assert_eq!(t.find_weekday(time::Weekday::Friday), Some(date!(2000 - 06 - 16)));
        assert_eq!(t.find_weekday(time::Weekday::Saturday), None);
        assert_eq!(t.find_weekday(time::Weekday::Sunday), None);
    }
}
