use crate::util::*;
use crate::*;

use nom::{
    branch::alt,
    character::complete::{char, line_ending, not_line_ending, one_of},
    combinator::{map_res, opt},
    multi::many0,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::num::TryFromIntError;

////////////////////////////////////////////////////////////////////////

fn parse_weekday(input: &str) -> IResult<&str, time::Weekday> {
    alt((
        str("Monday", "Mon", time::Weekday::Monday),
        str("Tuesday", "Tue", time::Weekday::Tuesday),
        str("Wednesday", "Wed", time::Weekday::Wednesday),
        str("Thursday", "Thu", time::Weekday::Thursday),
        str("Friday", "Fri", time::Weekday::Friday),
        str("Saturday", "Sat", time::Weekday::Saturday),
        str("Sunday", "Sun", time::Weekday::Sunday),
    ))(input)
}

fn parse_offset(input: &str) -> IResult<&str, WeekOffset> {
    let (input, prefix) = one_of("+-")(input)?;
    let (input, amount) = alt((
        bind(char('1'), WeekOffsetAmount::First),
        bind(char('2'), WeekOffsetAmount::Second),
        bind(char('3'), WeekOffsetAmount::Third),
        bind(char('4'), WeekOffsetAmount::Fourth),
        bind(char('5'), WeekOffsetAmount::Fifth),
    ))(input)?;

    Ok((
        input,
        WeekOffset {
            from_start: prefix == '+',
            amount: amount,
        },
    ))
}

fn parse_month_str(input: &str) -> IResult<&str, time::Month> {
    alt((
        str("January", "Jan", time::Month::January),
        str("February", "Feb", time::Month::February),
        str("March", "Mar", time::Month::March),
        str("April", "Apr", time::Month::April),
        str("May", "May", time::Month::May),
        str("June", "Jun", time::Month::June),
        str("July", "Jul", time::Month::July),
        str("August", "Aug", time::Month::August),
        str("September", "Sep", time::Month::September),
        str("October", "Oct", time::Month::October),
        str("November", "Nov", time::Month::November),
        str("December", "Dec", time::Month::December),
    ))(input)
}

fn parse_month_num(input: &str) -> IResult<&str, time::Month> {
    map_res(
        digits,
        |n| -> Result<time::Month, time::error::ComponentRange> {
            // XXX: If there is a u32 ??? u8 conversion error then use 0xff
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
        map_res(
            tuple((parse_weekday, parse_offset)),
            |(wday, off)| -> Result<Reminder, ()> { Ok(Reminder::SemiWeekly(wday, off)) },
        ),
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
            parse_reminder("Fri+2"),
            Ok((
                "",
                Reminder::SemiWeekly(time::Weekday::Friday, 2i8.try_into().unwrap())
            )),
        );
        assert_eq!(
            parse_reminder("Mon-4"),
            Ok((
                "",
                Reminder::SemiWeekly(time::Weekday::Monday, (-4i8).try_into().unwrap())
            )),
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
