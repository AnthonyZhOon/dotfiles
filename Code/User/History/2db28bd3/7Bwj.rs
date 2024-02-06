use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut copies = [1; 205]; // Optimise by pre-allocating entire array magic num because lazy
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
            let amt_to_add = copies[card_num];
            for i in 1..=successes {
                copies[card_num + i] += amt_to_add
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!("30", process(input)?);
        Ok(())
    }
}