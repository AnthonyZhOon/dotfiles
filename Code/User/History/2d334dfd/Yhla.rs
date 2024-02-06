#![allow(unused)]

use std::{io::stdin, process::exit};

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let res = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    res
}

fn take_str() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

fn take_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
fn to_chars(x: String) -> Vec<char> {
    x.chars().collect()
}

fn valid(a:usize, b:usize, c:usize) -> String {
    let remain = b.abs_diff(c);
    if remain % 2 == 0 && a > remain / 2 {
        return String::from("1")
    }
    String::from("0")
}

fn main() {
    let tests = take_int();
    for _ in 0..tests {
        let counts = take_vector();
        println!("{} {} {}", valid(counts[0], counts[1], counts[2]), valid(counts[1], counts[0], counts[2]), valid(counts[2], counts[1], counts[0]))
    }
}
