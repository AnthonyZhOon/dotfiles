use crate::custom_error::AocError;
use std::cmp::min;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let lines = _input.trim().split("\n");
    // Discrete derivatives, diff = (x_(n+1) - x_n)
    // Can be calc'd with 1d convolve and maybe fft

    Ok(lines
        .map(|line| {
            discrete_deriv(
                line.split_ascii_whitespace()
                    .map(|x| x.parse().expect("Failed to parse int"))
                    .collect(),
            )
        })
        // .inspect(|x| println!("{x}"))
        .sum::<isize>()
        .to_string())
}

fn discrete_deriv(sequence: Vec<isize>) -> isize {
    let mut matrix = vec![sequence];
    let mut non_zero = true;
    while non_zero {
        non_zero = false;
        let curr = matrix.last().unwrap();
        let mut next = vec![0; curr.len() - 1];
        for i in 1..curr.len() {
            let diff = curr[i] - curr[i - 1];
            if diff != 0 {
                next[i - 1] = diff;
                non_zero = true;
            }
            next[i - 1] = curr[i] - curr[i - 1];
        }
        matrix.push(next);
    }

    matrix
        .iter()
        // .inspect(|row| println!("{row:?}"))
        .rev()
        .fold(0, |acc, row| row.first().unwrap() - acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {

        let input = "0 3 6 9 12 15";
        assert_eq!("-3", process(input)?);
        Ok(())
    }
    #[test]
    fn test_all() -> miette::Result<()> {
        let input = "1 3 6 10 15 21";
        assert_eq!("0", process(input)?);
        Ok(())
    }
    

    #[test]
    fn test_3() -> miette::Result<()> {
        let input = "10 13 16 21 30 45";
        assert_eq!("5", process(input)?);
        Ok(())
    }
}
