use nom::InputIter;

use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // Step 1: Create the HashSet and then traverse it
    let (instruct, edges) = _input.split_once("\n\n").expect("Empty input?");

    let mut graph: HashMap<&str, [&str; 2]> = HashMap::new();
    edges.split('\n').map(|line| {
        line.split_once(" = ").map(|(from, to)| {
            (from,
            to.trim_start_matches("(").trim_end_matches(")").split_once(", ").expect("failed to split {to} tuple"))
            
        }).unwrap()
    }).for_each(|(k, v)| {graph.insert(k, v.into());});
    let (start, goal) = ("AAA", "ZZZ");
    let choices: Vec<u8> = instruct.trim().chars().map(|x| 
        match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected char in instructions")
        }
    ).collect();
    let i: usize = 0;
    let curr = "AAA";
    while let Some(curr) = graph.get(curr) {
        
    }
    
    Ok(1.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
