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
    Ok(String::from(_input.split_whitespace().map(|s| {
        [s.chars().find(|x| x.is_ascii_digit()).map_or("".to_string(), |s| s.to_string()), 
        s.chars().rev().find(|x| x.is_ascii_digit()).map_or("".to_string(), |s| s.to_string())].join("")
    })
    .map(|x| x.parse::<usize>().unwrap_or(0)).sum::<usize>().to_string()))
}

fn first_last(s: &str) -> usize {
    let first = num(s).parse().unwrap();
    let mut i = s.len();
    let last: usize = while let x = num(&s[i..]) {
        if !x.is_empty(){
            return x.parse().unwrap();
        }
        i -= 1;
    };
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
        todo!("haven't built test yet");
        let input = "";
        assert_eq!("", process(input)?);
        Ok(())
    }
}