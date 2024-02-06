use core::panic;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let sections = _input.split("\n\n").collect::<Vec<_>>();
    let mut seeds: Vec<u32> = sections
        .first()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    // Map the rest into closures that map nums.
    let conversions: Vec<Vec<[u32; 3]>> = sections
        .iter()
        .skip(1)
        .map(|&data| {
            data.trim()
                .split("\n")
                .skip(1)
                .map(|x| {
                    match x
                        .trim()
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_vec()
                    {
                        x if x.len() == 3 => [x[0], x[1], x[2]],
                        _ => panic!("Wrong length"),
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    seeds = seeds.chunks_exact(2).flat_map(|chunk| {
        match chunk {
            &[start, length] => {
                let mut nums = vec![start, start+length];
                
                for &[dest, source, len] in conversions.first().unwrap() {
                    let end = start + length;
                    if source > start {
                        nums.push(source);
                        if source+len <= end {
                            nums.push(source+len-1);
                        }
                    }
                }
                nums
            },
            _ => panic!("Improper chunk")
        }
    } ).collect_vec();
    for rules in conversions {
        seeds = seeds.iter().map(|&x| process_map(x, &rules)).collect();
    }
    Ok(seeds.iter().min().unwrap().to_string())
}

fn process_map(x: u32, mappings: &Vec<[u32; 3]>) -> u32 {
    for [dest, source, length] in mappings {
            if x >= *source && x < source + length {
                return x + dest - source;
            }
        }
    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}
