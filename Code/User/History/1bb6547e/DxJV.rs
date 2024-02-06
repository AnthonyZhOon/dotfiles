#![allow(unused)]

use std::{io::stdin, process::exit, array};

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

fn take_string() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}
fn to_chars(x: String) -> Vec<char> {
    x.chars().collect()
}
fn min_moves(root: usize, tree: &String, children: &Vec<[usize; 2]>) -> usize {
    let mut moves: usize = 0;
    let mut curr = root;
    loop {
        match tree.chars().into_iter().nth(curr-1).unwrap() {
            'L' => {
                match children[curr] {
                    [x,y] if *x == 0 => {
                        curr = *y;
                        moves += 1
                    },
                    [x, y] if *x == 0 && *y == 0 => {
                        return moves;
                    }
                    _ => (),
                }
            },
            'R' => {
                match children[curr] {
                    [x,y] if *y == 0 => {
                        curr = *x;
                        moves += 1
                    },
                    [x, y] if *x == 0 && *y == 0 => {
                        return moves;
                    }
                    _ => (),
                }
            },
            'U' => {
                match children[curr] {
                    [x, y] if *x == 0 && *y == 0 => {
                        moves += 1;
                        return moves;
                    }
                    [x, y] => {
                        moves += min_moves(*x, tree, children).min(min_moves(*y, tree, children));
                        return moves;
                    },
                    _ => ()
                }
            },
            _ => (),

        } 
    }
}

fn main() {
    let tests = take_int();
    for _ in 0..tests {
        let nodes = take_int();
        let tree = take_string();
        let mut children  = Vec::new();
        for _ in 0..nodes {
            if let [a, b] = take_vector().as_slice() {
                children.push([a.clone(), b.clone()]);
            }
            
        }
        println!("{}", min_moves(1, &tree, &children))
    }
}
