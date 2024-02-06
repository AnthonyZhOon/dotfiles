use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let lines = _input.trim().split("\n");
    // Discrete derivatives, diff = (x_(n+1) - x_n)
    // Can be calc'd with 1d convolve and maybe fft
    Ok(1.to_string())
}

fn discrete_deriv(sequence: Vec<usize>) -> Vec<Vec<usize>> {
    let mut matrix = vec![sequence];
    let mut non_zero = true;
    while non_zero {
        non_zero = false;
        let curr = matrix.last().unwrap();
        let mut next = Vec::with_capacity(curr.len());
        for i in 1..curr.len() {
            let diff = curr[i] - curr[i-1];
            if diff !=
            next[i-1] = curr[i] - curr[i-1];
        }
        matrix.push(next);
    }
    matrix
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
