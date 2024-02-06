use nom::InputIter;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    Ok(String::from(_input.split_whitespace().map(|s| {
        [s.chars().take_while(|x| !x.is_ascii_digit()).last().map_or("".to_string(), |s| s.to_string()), 
        s.chars().rev().take_while(|x| !x.is_ascii_digit()).last().map_or("".to_string(), |s| s.to_string())].join("")
    })
    .map(|x| x.parse::<usize>().unwrap()).sum::<usize>().to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2\n";
        assert_eq!("12", process(input)?);
        Ok(())
    }
}
