use std::error::Error;

use crate::custom_error::AocError;
use nom::{IResult, Err, multi};
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::{opt, map_res};
use nom::character::is_digit;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    Ok(String::from(_input.split_whitespace().map(first_last)
    .sum::<usize>().to_string()))
}

fn first_last(s: &str) -> usize {
    let first = num(s);
    let mut last = "";

    for i in (0..s.len()).rev() {
        match num(&s[i..]) {
            x if !x.is_empty() => {last = x.; break},
            _ => (),
        }
    }

    first + last
}
    
fn num(s: &str) -> &str {
    match opt(alt((is_str_digit, first_digit)))(s) {
        Ok((_, Some(s))) => s,
        Ok((_, None)) => "",
        Err(x) => panic!("{x}")
    }
}
fn first_digit(s: &str) -> IResult<&str, &str> {
    if s.len() == 0 {
        Err(Err::Incomplete(nom::Needed::new(1)))
    } else if s.chars().take(1).last().unwrap().is_ascii_digit() {
        Ok((&s[1..], &s[0..0]))
    } else {
        Ok((&s[..], &""))
    }
}

fn is_str_digit(s: &str) -> IResult<&str, &str> {
    map_res(parse_str_digit, str_to_digit)(s)
}

fn parse_str_digit(s: &str) -> IResult<&str, &str> {
    alt((tag("one"), tag("two"), tag("three"), tag("four"), tag("five"), tag("six"), tag("seven"), tag("eight"), tag("nine"), tag("zero")))(s)
}

fn str_to_digit(s: &str) -> Result<&str, ()> {
    match s {
        "zero" => Ok("0"),
        "one" => Ok("1"),
        "two" => Ok("2"),
        "three" => Ok("3"),
        "four" => Ok("4"),
        "five" => Ok("5"),
        "six" => Ok("6"),
        "seven" => Ok("7"),
        "eight" => Ok("8"),
        "nine" => Ok("9"),
    _ => Err(()),        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "abcone2threexyz";
        assert_eq!("11", process(input)?);
        Ok(())
    }
}