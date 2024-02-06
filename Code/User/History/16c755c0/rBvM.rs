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

fn take_vector_str() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let res = input
        .trim()
        .split_whitespace()
        .collect();
    res
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
    let tests = take_int();
    for _ in ..tests {
        let size = take_int();
        let row = take_vector_str();
        let mut moves: usize = 0;
        // First pass tries to create an infinite resource
        for i in 2..row.len() {
            if row[0..i] == "..." {
                println!("{2}");
                exit(0)
            }
        for char in row {
            if char == "." {
                moves += 1
            }
        }
        println!("{moves}")
        }
    }
}
