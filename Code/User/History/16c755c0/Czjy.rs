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



fn main() {
    let tests = take_int();
    'core: for _ in 0..tests {
        let size = take_int();
        let row = take_str();
        let mut moves: usize = 0;
        // First pass tries to create an infinite resource
        for i in 2..row.len() {
            if row[i-2..=i] == "...".to_string() {
                println!("2");
                continue 'core;
            }
        }
        for char in row.chars() {
            if char == '.' {
                moves += 1
            }
        }
        println!("{moves}")
    }
}