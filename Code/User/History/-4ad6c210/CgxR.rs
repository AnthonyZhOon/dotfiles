use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // Step 1: Create the HashSet and then traverse it
    let (instruct, edges) = _input.split_once("\n\n").expect("Empty input?");

    let mut graph: HashMap<&[u8], [&[u8];2]> = HashMap::new();
    edges.split('\n').map(|line| {
        line.split_once(" = ").map(|(from, to)| {
            (from.as_bytes(),
            to.trim_start_matches("(").trim_end_matches(")").split_once(", ").map(|(l, r)| [l.as_bytes(), r.as_bytes()]).expect("failed to split {to} tuple"))
            
        }).unwrap()
    }).for_each(|(k, v)| {graph.insert(k, v.into());});
    let (start, goal) = ("AAA".as_bytes(), "ZZZ".as_bytes());
    let choices: Vec<usize> = instruct.trim().chars().map(|x| 
        match x {
            'L' => 0,
            'R' => 1,
            _ => panic!("Unexpected char in instructions")
        }
    ).collect();
    let mut i = 0usize;
    let mut curr = start;

    while let Some(&next) = graph.get(curr) {
        curr = next[choices[i % choices.len()]];
        // println!("{curr}");
        i += 1;
        if curr.eq(goal) {
            break
        }
        
    }
    
    Ok(i.to_string())
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
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
