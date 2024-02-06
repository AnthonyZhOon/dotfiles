use crate::custom_error::AocError;
use std::cmp::min;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let lines = _input.trim().split("\n");
    let mut result = 0;
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
        .sum::<usize>()
        .to_string())
}

fn discrete_deriv(sequence: Vec<usize>) -> usize {
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
            }
            next[i - 1] = curr[i] - curr[i - 1];
        }
        matrix.push(next);
    }
    println!("{:?}", matrix.clone());
    matrix
        .iter()
        .enumerate()
        .map(|(row, val)| val.last().unwrap()*(min(row, 1)))
        .inspect(|x| println!("{x}"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {

        let input = "0 3 6 9 12 15";
        assert_eq!("18", process(input)?);
        Ok(())
    }
    #[test]
    fn test_all() -> miette::Result<()> {
        let input = "1 3 6 10 15 21";
        assert_eq!("28", process(input)?);
        Ok(())
    }
}