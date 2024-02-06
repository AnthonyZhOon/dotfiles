use crate::custom_error::AocError;
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
    todo!("day 01 - part 1");
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