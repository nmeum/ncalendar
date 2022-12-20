use nom::{
    IResult,
    branch::alt,
    multi::{many0, many1},
    bytes::complete::{tag},
    sequence::{delimited},
    error::{ParseError, FromExternalError},
    character::complete::{one_of, char},
    combinator::{map_res, recognize},
};

pub fn bind<'a, T: Copy, E: ParseError<&'a str> + FromExternalError<&'a str, ()>>(name: &'a str, other: &'a str, val: T)
        -> impl FnMut(&'a str) -> IResult<&'a str, T, E> {
    map_res(alt((tag(name), tag(other))), move |_| -> Result<T, ()> { Ok(val) })
}

pub fn digits(input: &str) -> IResult<&str, u32> {
  map_res(
    recognize(many1(one_of("0123456789"))),
    |input: &str| u32::from_str_radix(input, 10),
  )(input)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    many0(char(' ')),
    inner,
    many0(char(' ')),
  )
}
