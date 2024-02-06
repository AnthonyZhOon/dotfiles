use crate::custom_error::AocError;

const RGB: (u8, u8, u8) = (12, 13, 14);
#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    _input.split('\n').map(|line| {
        line.split(':').last().unwrap()
    })

    Ok(String::from("shutup"))
}

struct RGB {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse_round(s: &str) -> RGB {
    match s.trim().split(',')
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
