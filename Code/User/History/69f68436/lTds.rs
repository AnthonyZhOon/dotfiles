use itertools::Itertools;

use crate::custom_error::AocError;
struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}


const MAX: RGB = RGB{red: 12, green: 13, blue: 14};
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    _input.split('\n').map(|line| {
        parse_round(line)
    })

    Ok(String::from("shutup"))
}


fn parse_round(s: &str) -> usize {
    let (mut red,mut green,mut blue): (usize, usize, usize) = (0,0,0);
    let (game, rounds) = s.split_once(':').unwrap();
    let game_id = game.split_once(' ').unwrap().1.parse::<usize>();
    for round in s.split(';') {
        for colour in round.trim().split(',') {
            match colour.split_ascii_whitespace().take(2).collect_tuple().unwrap() {
                (x, "red") => red += x.parse::<usize>().unwrap(),
                (x, "blue") => blue += x.parse::<usize>().unwrap(),
                (x, "green") => green += x.parse::<usize>().unwrap(),
                _ => (),
            }
        }
    } 
    if 
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
