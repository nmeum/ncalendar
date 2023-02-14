use time::Date;

pub struct WeekdayIterator {
    start: time::Date,
    off: i8, // offset in weeks
}

impl Iterator for WeekdayIterator {
    type Item = time::Date;

    fn next(&mut self) -> Option<Self::Item> {
        if self.off == 0 {
            self.off += 1;
            return Some(self.start);
        }

        let dur = time::Duration::weeks(self.off as i64);
        let day = self.start.checked_add(dur)?;
        if day.month() != self.start.month() {
            None
        } else {
            self.off += 1;
            Some(day)
        }
    }
}

fn iterator(year: i32, month: time::Month, wday: time::Weekday) -> Option<WeekdayIterator> {
    let mut day = Date::from_calendar_date(year, month, 1).ok()?;

    // Find first matching weekday in given month.
    while day.weekday() != wday {
        if day.month() != month {
            return None;
        }

        day = day.next_day()?;
    }

    Some(WeekdayIterator { start: day, off: 0 })
}

pub fn filter(year: i32, month: time::Month, wday: time::Weekday) -> Option<Vec<time::Date>> {
    iterator(year, month, wday).map(|it| it.collect())
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn iterate_five() {
        let it = iterator(2023, time::Month::January, time::Weekday::Monday).unwrap();
        let mondays: Vec<time::Date> = it.collect();
        assert_eq!(
            mondays,
            vec![
                date!(2023 - 01 - 02),
                date!(2023 - 01 - 09),
                date!(2023 - 01 - 16),
                date!(2023 - 01 - 23),
                date!(2023 - 01 - 30),
            ]
        );
    }

    #[test]
    fn iterate_four() {
        let it = iterator(2023, time::Month::February, time::Weekday::Monday).unwrap();
        let mondays: Vec<time::Date> = it.collect();
        assert_eq!(
            mondays,
            vec![
                date!(2023 - 02 - 06),
                date!(2023 - 02 - 13),
                date!(2023 - 02 - 20),
                date!(2023 - 02 - 27),
            ]
        );
    }
}
