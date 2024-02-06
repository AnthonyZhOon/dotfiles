use std::collections::BTreeSet;

fn main() {
    // println!("Hello");
    let n = 6;
    let degrees = vec![0usize; n];
    let exterior_degree = n;
    let mut edges = BTreeSet::<(usize, usize)>::new();
    // Generate every pair, count how many start and end on opposite sides of each edge?? divide by 2 to fix over counts
    let mut intersections = 0usize;

    for i in 0..n {
        for j in i + 1..n {
            for &(x, y) in edges.iter() {
                match y > i && y < j {
                    true => {
                        intersections += 1;
                    }
                    false => (),
                }
            }
            edges.insert((i, j));
        }
    }
    println!("n = {n}, intersections = {intersections}");
    println!("edges: {edges:?}");
    // Every vertex up to the nth one will add at least 1 new edge
}

const fn total_degree(n: usize) -> usize {
    n * (n - 1) / 2
}

const fn basic_degree(n: usize) -> usize {
    2
}

const fn max_degree(n: usize) -> usize {
    n - 1
}

fn edges_added(n: usize, curr_degree: usize) -> usize {
    max_degree(n) - curr_degree
}