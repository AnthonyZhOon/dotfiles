use itertools::Itertools;

use crate::custom_error::AocError;

const RGB: (u8, u8, u8) = (12, 13, 14);
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    _input.split('\n').map(|line| {
        parse_round(line)
    })

    Ok(String::from("shutup"))
}

struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_round(s: &str) -> RGB {
    let (mut red,mut green,mut blue): (usize, usize, usize) = (0,0,0);
    let ()
    
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
    RGB{red, green, blue}
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