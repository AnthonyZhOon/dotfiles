use crate::custom_error::AocError;
use itertools::Itertools;
struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
use std::cmp::Ordering::{Less, Equal, Greater};
impl PartialOrd for RGB {
    
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (r, g, b) = (self.red.cmp(&other.red),
                                                        self.green.cmp(&other.green),
                                                        self.blue.cmp(&other.blue));
        match (r, g, b) {
            (Equal, Equal, Equal) => Some(Equal),
            (Less | Equal, Less | Equal, Less | Equal) => Some(Less),
            _ => Some(Greater)
        }
    }
}
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    Ok(format!("{}", _input.split('\n').map(|line| {
        parse_round(line)
    }).sum::<usize>()))
}

fn parse_round(s: &str) -> usize {
    let (game, rounds) = s.split_once(':').unwrap_or_else(|| {println!("{s}"); ("help", "bad") });
    let game_id = game.trim().split_once(' ').unwrap().1.parse::<usize>().unwrap();

    for round in rounds.split(';') {
        let (mut red,mut green,mut blue): (usize, usize, usize) = (0,0,0);
        for colour in round.trim().split(',') {
            match colour.split_ascii_whitespace().take(2).collect_tuple().unwrap() {
                (x, "red") => red += x.parse::<usize>().unwrap(),
                (x, "blue") => blue += x.parse::<usize>().unwrap(),
                (x, "green") => green += x.parse::<usize>().unwrap(),
                _ => (),
            }
        }
        if (RGB{red, green, blue}) > MAX {return 0}    
    } 
    game_id
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