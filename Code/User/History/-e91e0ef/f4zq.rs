use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    // Dist = (t-x)*x > y // Binary search?
    // x*t - x^2 - y >= 0
    // Solve the polynomial, roots are between
    // -x^2 + xt -y >= 0
    // x = (-t +- sqrt(t^2 - 4y)/-2
    // min with 0 somewhere, consider squaring both sides?
    let (time, dist) = _input.split_once('\n').expect("Weird input cannot split");
    let times: f64 = time
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();
        
    let dists: f64 = dist
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();

    let ways = feasible_span(times, dists);
    Ok(ways.to_string())
}

fn feasible_span(time: f64, dist: f64) -> u32 {
    let centre = time / 2.0;
    let x = {
            let span = time.powi(2) - 4.0 * ( dist + 1.0);
            if span.is_sign_negative() {
                return 0
            }else {
                span.sqrt()/2.0
            }};
    let cleaned: [f64; 2] = [(centre - x).ceil(), (centre + x).floor()];
    let diff =  (cleaned[1] as u32) - (cleaned[0].max(0.0) as u32).max(0) + 1;
    diff as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}
