use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of},
    combinator::{map_res, recognize},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::delimited,
    IResult,
};

// Bind the given parser to the given value (map_res short).
pub fn bind<'a, F: 'a, T: Copy, O, E: ParseError<&'a str> + FromExternalError<&'a str, ()>>(
    inner: F,
    val: T,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    map_res(inner, move |_| -> Result<T, ()> { Ok(val) })
}

// Parse on of the given strings and return the given value.
pub fn str<'a, T: Copy, E: ParseError<&'a str> + FromExternalError<&'a str, ()> + 'a>(
    name: &'a str,
    other: &'a str,
    val: T,
) -> impl FnMut(&'a str) -> IResult<&'a str, T, E> {
    bind(alt((tag(name), tag(other))), val)
}

pub fn digits(input: &str) -> IResult<&str, u32> {
    map_res(recognize(many1(one_of("0123456789"))), |input: &str| {
        u32::from_str_radix(input, 10)
    })(input)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(many0(char(' ')), inner, many0(char(' ')))
}

pub fn empty_lines<'a, F: 'a, O, E: ParseError<&'a str> + 'a>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(many0(ws(line_ending)), inner, many0(ws(line_ending)))
}
