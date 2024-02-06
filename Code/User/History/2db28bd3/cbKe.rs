use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut copies = [1; 204];
    for (card_num, line) in _input.trim().split('\n').enumerate() {
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
            for i in 1..=successes {
                copies[i] += copies[card_num]
            }
        }
        
    }
    Ok(copies.iter().sum::<u32>().to_string())
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