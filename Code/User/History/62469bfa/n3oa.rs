use std::collections::BTreeSet;

fn main() {
    // println!("Hello");
    let n = 50;
    let mut edges = BTreeSet::<(usize, usize)>::new();
    // Generate every pair, count how many start and end on opposite sides of each edge?? divide by 2 to fix over counts
    let mut intersections = 0usize;

    for i in 0..n {
        for j in i + 1..n {
            for &(x, y) in edges.iter() {
                if y > i && y < j  && x < i {
                    intersections += 1;
                }
            }
            edges.insert((i, j));
        }
    }
    println!("n = {n}, intersections = {intersections}");
    println!("edges: {edges:?}");
    // Every vertex up to the nth one will add at least 1 new edge
}
