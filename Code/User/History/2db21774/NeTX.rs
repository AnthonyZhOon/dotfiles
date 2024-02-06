use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut total = 0usize;
    for line in _input.split('\n') {
        let (winning_str, given) = line
            .trim()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(" | ")
            .unwrap();
        let winning_nums = winning_str
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect_vec();
        let successes = given
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .filter(|x| winning_nums.contains(x)).count();
        total += 0 << successes;

    }
    Ok(format!("{total}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
