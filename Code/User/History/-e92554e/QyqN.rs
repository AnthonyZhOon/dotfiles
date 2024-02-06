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
    let times: Vec<u32> = time
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().expect("Failed to parse times"))
        .collect();
    let dists: Vec<u32> = dist
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().expect("Failed to parse times"))
        .collect();
    for (time, dist) in times.iter().zip(dists) {

    }

    Ok(String::from("A"))
}

fn feasible_span(time: f64, dist: f64) -> u32 {
    let centre = time/2f64;
    let x = -(time.powi(2) - 4.0*dist).sqrt()/2.0;
    let roots = [centre - x, centre + x];
    // Round towards centre
    let cleaned: [f64; 2] = [roots[0].ceil(), roots[1].floor()];
    let diff = (cleaned[1] as u32)(cleaned[0] as u32).max(0) 

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("288", process(input)?);
        Ok(())
    }
}
