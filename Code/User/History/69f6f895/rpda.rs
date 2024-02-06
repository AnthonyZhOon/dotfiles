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
    let (_, rounds) = s.split_once(':').unwrap_or_else(|| {println!("{s}"); ("help", "bad") });
    let (mut red,mut green,mut blue): (usize, usize, usize) = (0,0,0);
    for round in rounds.split(';') {
        for colour in round.trim().split(',') {
            match colour.split_ascii_whitespace().take(2).collect_tuple().unwrap() {
                (x, "red") =>  red = red.max(x.parse::<usize>().unwrap()),
                (x, "blue") => blue = blue.max(x.parse::<usize>().unwrap()),
                (x, "green") => green = green.max(x.parse::<usize>().unwrap()),
                _ => (),
            }
        }  
    } 
    red * green * blue
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}