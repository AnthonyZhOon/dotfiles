use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut total = 0usize;
    for line in _input.trim().split('\n') {
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
        if successes > 0 {
            total += 1 << (successes - 1) ;
        }
        
    }
    Ok(total.to_string())
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