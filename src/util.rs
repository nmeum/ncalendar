use std::env;
use std::path::{self, Path};
use time::macros::format_description;

pub fn calendar_file() -> Result<path::PathBuf, env::VarError> {
    let home = env::var("HOME")?;
    let path = Path::new(&home);

    Ok(path.join(".ncalendar").join("calendar"))
}

pub fn parse_today(input: &str) -> Result<time::Date, time::error::Parse> {
    let fmt = format_description!("[day][month][year]");
    time::Date::parse(input, &fmt)
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
