use itertools::Itertools;

use crate::custom_error::AocError;
use std::collections::{HashMap, VecDeque};
use std::str::from_utf8;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    // Step 1: Create the HashSet and then traverse it
    let (instruct, edges) = _input.split_once("\n\n").expect("Empty input?");

    let mut graph: HashMap<&[u8], [&[u8]; 2]> = HashMap::new();
    let mut frontier = VecDeque::new();
    edges
        .split('\n')
        .map(|line| {
            line.split_once(" = ")
                .map(|(from, to)| {
                    (
                        from.as_bytes(),
                        to.trim_start_matches("(")
                            .trim_end_matches(")")
                            .split_once(", ")
                            .map(|(l, r)| [l.as_bytes(), r.as_bytes()])
                            .expect("failed to split {to} tuple"),
                    )
                })
                .unwrap()
        })
        .for_each(|(k, v)| {
            graph.insert(k, v.into());
            if k.ends_with(&[b'A']) {
                frontier.push_back(k);
            }
        });
    let choices: Vec<usize> = instruct
        .trim()
        .chars()
        .map(|x| match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected char in instructions"),
        })
        .collect();
    let mut i = 0;
    let mut targets = 0;
    let sources = frontier.len();
    let mut cycles: Vec<Vec<i128>> = vec![vec![]; 6];
    loop {

        for (idx, place) in frontier.iter_mut().enumerate() {
            let &next = graph.get(place).unwrap();
            *place = next[choices[i % choices.len()]].as_ref();
            // println!("{curr}");
            
            if place.ends_with(&[b'Z']) {
                cycles[idx].push(i as i128);
                if cycles[idx].len() == 2 {
                    targets += 1
                }
            }
        }
        i += 1;
        // frontier.clone().iter().inspect(|&&x| println!("{}", from_utf8(x).unwrap())).count();
        // a + kX = b + kY = c + kZ
        
        if targets == sources {
            break
        } 
    }
    // Generate the simultaneous linear modular expression
    let moduli = cycles.iter().map(|inner| {
        match inner.into_iter().take(2).collect_tuple() {
            Some((&a, &b)) => [a, b-a],
            None => {println!("Skipping empty"); [1, 1]},
        }
        }).collect::<Vec<_>>();
    // let moduli = vec![[1, 2], [2, 3]];
    let [steps, modulo] = moduli.into_iter().reduce(|[a, n1], [b, n2]| {
        let (gcd, m1, m2) = extended_euclidean_algorithm(n1, n2);
        let (n1, n2) = (n1/gcd, n2/gcd);
        let (gcd, m1, m2) = extended_euclidean_algorithm(n1, n2);
        let prod = n1*n2;
        let x =  (a*m2*n2)%(prod) + (b*m1*n1)%(prod);
        [x % (prod), prod]
        }).unwrap();
    println!("{:?}", steps);
    Ok(i.to_string())
}

fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    // This is the base case: when b = 0,
    //     gcd(a, 0) = a
    // Hence the Euclidean equation becomes:
    //     a(1) + b(0) = a
    if b == 0 {
        return (a, 1, 0);
    }

    // Recursively call the extended Euclidean Algorithm
    let (gcd, x1, y1) = extended_euclidean_algorithm(b, a % b);

    // Compute x and y by working backwards the Euclidean Algorithm
    let x = y1;
    let y = x1 - (a / b) * y1;

    // Return the tuple
    return (gcd, x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
