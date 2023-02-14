use crate::util::*;
use crate::*;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, not_line_ending},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{preceded, terminated, tuple},
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

fn parse_month_str(input: &str) -> IResult<&str, time::Month> {
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

fn parse_month_num(input: &str) -> IResult<&str, time::Month> {
    map_res(
        digits,
        |n| -> Result<time::Month, time::error::ComponentRange> {
            // XXX: If there is a u32 → u8 conversion error then use 0xff
            // as the month value which will result in a ComponentRange error.
            // Unfourtunately, can't create a ComponentRange directly and use .map_err().
            let m: u8 = n.try_into().unwrap_or(0xff);
            time::Month::try_from(m)
        },
    )(input)
}

fn parse_month(input: &str) -> IResult<&str, time::Month> {
    alt((parse_month_str, parse_month_num))(input)
}

fn parse_day(input: &str) -> IResult<&str, Day> {
    map_res(digits, |n| -> Result<Day, TryFromIntError> { n.try_into() })(input)
}

fn parse_year(input: &str) -> IResult<&str, Year> {
    map_res(digits, |n| -> Result<Year, TryFromIntError> {
        n.try_into()
    })(input)
}

fn parse_reminder(input: &str) -> IResult<&str, Reminder> {
    alt((
        map_res(parse_weekday, |wday| -> Result<Reminder, ()> {
            Ok(Reminder::Weekly(wday))
        }),
        map_res(
            tuple((parse_day, ws(char('*')), opt(parse_year))),
            |(day, _, year)| -> Result<Reminder, ()> { Ok(Reminder::Monthly(day, year)) },
        ),
        map_res(
            tuple((opt(parse_day), ws(parse_month), opt(parse_year))),
            move |(day, mon, year)| -> Result<Reminder, time::error::ComponentRange> {
                let day = day.unwrap_or(1);
                Ok(match year {
                    Some(y) => Reminder::Date(time::Date::from_calendar_date(y, mon, day)?),
                    None => Reminder::Yearly(day, mon),
                })
            },
        ),
    ))(input)
}

fn parse_desc(input: &str) -> IResult<&str, String> {
    let (input, (desc, ext)) = tuple((
        terminated(not_line_ending, line_ending),
        many0(terminated(
            preceded(char('\t'), not_line_ending),
            line_ending,
        )),
    ))(input)?;

    if ext.is_empty() {
        Ok((input, desc.to_string()))
    } else {
        Ok((input, desc.to_owned() + "\n\t" + &ext.join("\n\t")))
    }
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    let (input, (day, _, desc)) = tuple((parse_reminder, char('\t'), parse_desc))(input)?;

    Ok((input, Entry { day, desc: desc }))
}

pub fn parse_entries(input: &str) -> IResult<&str, Vec<Entry>> {
    many0(empty_lines(parse_entry))(input)
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::Error;
    use nom::error::ErrorKind;
    use nom::Err;
    use time::macros::date;

    #[test]
    fn weekday() {
        assert_eq!(parse_weekday("Monday"), Ok(("", time::Weekday::Monday)));
        assert_eq!(parse_weekday("Mon"), Ok(("", time::Weekday::Monday)));
        assert_eq!(parse_weekday("Tuesday"), Ok(("", time::Weekday::Tuesday)));
    }

    #[test]
    fn month() {
        assert_eq!(parse_month("April"), Ok(("", time::Month::April)));
        assert_eq!(parse_month("04"), Ok(("", time::Month::April)));
        assert_eq!(parse_month("4"), Ok(("", time::Month::April)));
        assert_eq!(
            parse_month("13"),
            Err(Err::Error(Error::new("13", ErrorKind::MapRes)))
        );
        assert_eq!(
            parse_month("2342"),
            Err(Err::Error(Error::new("2342", ErrorKind::MapRes)))
        );
    }

    #[test]
    fn reminder() {
        assert_eq!(
            parse_reminder("25 Feb"),
            Ok(("", Reminder::Yearly(25, time::Month::February)))
        );
        assert_eq!(
            parse_reminder("Fri"),
            Ok(("", Reminder::Weekly(time::Weekday::Friday)))
        );
        assert_eq!(
            parse_reminder("Jan 1990"),
            Ok(("", Reminder::Date(date!(1990 - 01 - 01))))
        );
        assert_eq!(
            parse_reminder("06 July 2020"),
            Ok(("", Reminder::Date(date!(2020 - 07 - 06))))
        );
        assert_eq!(
            parse_reminder("12 Dec 1950"),
            Ok(("", Reminder::Date(date!(1950 - 12 - 12))))
        );
        assert_eq!(
            parse_reminder("10 *"),
            Ok(("", Reminder::Monthly(10, None)))
        );
        assert_eq!(
            parse_reminder("10 * 1989"),
            Ok(("", Reminder::Monthly(10, Some(1989))))
        );
    }

    #[test]
    fn desc() {
        assert_eq!(parse_desc("foo bar\n"), Ok(("", "foo bar".to_string())));
        assert_eq!(
            parse_desc("foo\n\tbar\n"),
            Ok(("", "foo\n\tbar".to_string()))
        );
        assert_eq!(
            parse_desc("foo\n\tbar\n\tbaz\n"),
            Ok(("", "foo\n\tbar\n\tbaz".to_string()))
        );
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

        assert_eq!(
            parse_entry("Mon\tMonday\n"),
            Ok((
                "",
                Entry {
                    day: Reminder::Weekly(time::Weekday::Monday),
                    desc: "Monday".to_string(),
                }
            ))
        );
    }
}
