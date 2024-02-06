use itertools::Itertools;
use nom::InputIter;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    Ok(String::from(
        _input
            .split_whitespace()
            .map(|s| {
                [
                    s.chars()
                        .find(|x| x.is_ascii_digit())
                        .map_or("".to_string(), |s| s.to_string()),
                    s.chars()
                        .rev()
                        .find(|x| x.is_ascii_digit())
                        .map_or("".to_string(), |s| s.to_string()),
                ]
                .join("")
            })
            .map(|x| x.parse::<usize>().unwrap_or(0))
            .sum::<usize>()
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2\n
        pqr3stu8vwx\n
        a1b2c3d4e5f\n
        treb7uchet\n";
        assert_eq!("142", process(input)?);
        Ok(())
    }
}
