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
