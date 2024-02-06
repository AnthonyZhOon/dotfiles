#![allow(unused)]

use std::io::stdin;

fn take_int() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn take_vector() -> Vec<usize> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
}

fn take_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
fn to_chars(x: String) -> Vec<char> {
    x.chars().collect()
}

fn main() {
    println!("Hello, world!");
}
