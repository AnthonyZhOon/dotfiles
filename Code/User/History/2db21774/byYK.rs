use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut total = 0usize;
    for line in _input.split('\n') {
        let (winning_str, given) = _input
            .trim()
            .split_once(": 0")
            .unwrap()
            .1
            .split_once(" | ")
            .unwrap();
        let winning_nums = winning_str
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let success = given
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .filter(|x| winning_nums.contains(x));

    }
    Ok(String::from("Shutup"))
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