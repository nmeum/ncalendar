use std::env;
use std::num::ParseIntError;
use std::path::{self, Path};
use time::macros::format_description;

pub fn weekday_short(date: time::Date) -> String {
    let w = date.weekday();

    // Every weekday is at least three characters long.
    // Hence, the .get() invocation should never panic.
    w.to_string().get(0..3).unwrap().to_string()
}

pub fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

pub fn parse_file(input: &str) -> Result<path::PathBuf, env::VarError> {
    if input.is_empty() {
        calendar_file()
    } else {
        Ok(input.into())
    }
}

pub fn parse_today(input: &str) -> Result<time::Date, time::error::Parse> {
    if input == "today" {
        Ok(time::OffsetDateTime::now_local().unwrap().date())
    } else {
        let fmt = format_description!("[day][month][year]");
        time::Date::parse(input, &fmt)
    }
}

pub fn parse_days(days: &str) -> Result<time::Duration, ParseIntError> {
    let days = days.parse::<u32>()?;
    Ok(time::Duration::days(days.into()))
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn today_parser() {
        assert_eq!(parse_today("02012022"), Ok(date!(2022 - 01 - 02)));
        assert_eq!(parse_today("12122000"), Ok(date!(2000 - 12 - 12)));
    }
}
