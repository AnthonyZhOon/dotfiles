use nom::InputIter;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    Ok(_input.split_whitespace().map(|s| {
        [s.chars().take_while(|x| !x.is_digit(10)).last(), 
        s.ch]
    }))

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
