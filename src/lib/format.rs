use crate::util::*;
use crate::*;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, not_line_ending},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{terminated, tuple},
    IResult,
};
use std::num::TryFromIntError;

////////////////////////////////////////////////////////////////////////

fn parse_weekday(input: &str) -> IResult<&str, time::Weekday> {
    alt((
        bind("Monday", "Mon", time::Weekday::Monday),
        bind("Tuesday", "Tue", time::Weekday::Tuesday),
        bind("Wednesday", "Wed", time::Weekday::Wednesday),
        bind("Thursday", "Thu", time::Weekday::Thursday),
        bind("Friday", "Fri", time::Weekday::Friday),
        bind("Saturday", "Sat", time::Weekday::Saturday),
        bind("Sunday", "Sun", time::Weekday::Sunday),
    ))(input)
}

fn parse_month(input: &str) -> IResult<&str, time::Month> {
    alt((
        bind("January", "Jan", time::Month::January),
        bind("February", "Feb", time::Month::February),
        bind("March", "Mar", time::Month::March),
        bind("April", "Apr", time::Month::April),
        bind("May", "May", time::Month::May),
        bind("June", "Jun", time::Month::June),
        bind("July", "Jul", time::Month::July),
        bind("August", "Aug", time::Month::August),
        bind("September", "Sep", time::Month::September),
        bind("October", "Oct", time::Month::October),
        bind("November", "Nov", time::Month::November),
        bind("December", "Dec", time::Month::December),
    ))(input)
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(digits, |n| -> Result<Day, TryFromIntError> { n.try_into() })(input)
}

fn parse_year(input: &str) -> IResult<&str, Year> {
    map_res(digits, |n| -> Result<Year, TryFromIntError> {
        n.try_into()
    })(input)
}

fn parse_date(input: &str) -> IResult<&str, time::Date> {
    // TODO: error handling
    let cur = time::OffsetDateTime::now_local().unwrap();
    let year = cur.year();

    alt((
        map_res(
            tuple((parse_day, ws(parse_month), opt(parse_year))),
            move |(day, mon, y)| -> Result<time::Date, time::error::ComponentRange> {
                time::Date::from_calendar_date(y.unwrap_or(year), mon, day)
            },
        ),
        map_res(
            tuple((parse_month, opt(ws(parse_year)))),
            move |(mon, y)| -> Result<time::Date, time::error::ComponentRange> {
                time::Date::from_calendar_date(y.unwrap_or(year), mon, 01)
            },
        ),
    ))(input)
}

fn parse_reminder(input: &str) -> IResult<&str, Reminder> {
    alt((
        map_res(parse_weekday, |wday| -> Result<Reminder, ()> {
            Ok(Reminder::Weekday(wday))
        }),
        map_res(parse_date, |date| -> Result<Reminder, ()> {
            Ok(Reminder::Date(date))
        }),
    ))(input)
}

fn parse_desc(input: &str) -> IResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, (day, _, desc)) = tuple((parse_reminder, char('\t'), parse_desc))(input)?;

    Ok((
        input,
        Entry {
            day,
            desc: desc.to_string(),
        },
    ))
}

pub fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    many0(parse_entry)(input)
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn weekday() {
        assert_eq!(parse_weekday("Monday"), Ok(("", time::Weekday::Monday)));
        assert_eq!(parse_weekday("Mon"), Ok(("", time::Weekday::Monday)));
        assert_eq!(parse_weekday("Tuesday"), Ok(("", time::Weekday::Tuesday)));
    }

    #[test]
    fn date() {
        let year = time::OffsetDateTime::now_local().unwrap().year();
        assert_eq!(
            parse_date("25 Feb"),
            Ok((
                "",
                time::Date::from_calendar_date(year, time::Month::February, 25).unwrap()
            ))
        );
        assert_eq!(parse_date("12 Dec 1950"), Ok(("", date!(1950 - 12 - 12))));
        assert_eq!(
            parse_date("Dec"),
            Ok((
                "",
                time::Date::from_calendar_date(year, time::Month::December, 01).unwrap()
            ))
        );
        assert_eq!(parse_date("Jan 1990"), Ok(("", date!(1990 - 01 - 01))));
    }

    #[test]
    fn reminder() {
        assert_eq!(
            parse_reminder("Fri"),
            Ok(("", Reminder::Weekday(time::Weekday::Friday)))
        );
        assert_eq!(
            parse_reminder("06 July 2020"),
            Ok(("", Reminder::Date(date!(2020 - 07 - 06))))
        );
    }

    #[test]
    fn desc() {
        assert_eq!(parse_desc("foo bar\n"), Ok(("", "foo bar")));
    }

    #[test]
    fn event() {
        assert_eq!(
            parse_entry("12 Mar 2015\tDo some stuff\n"),
            Ok((
                "",
                Entry {
                    day: Reminder::Date(date!(2015 - 03 - 12)),
                    desc: "Do some stuff".to_string(),
                }
            ))
        );
    }
}
