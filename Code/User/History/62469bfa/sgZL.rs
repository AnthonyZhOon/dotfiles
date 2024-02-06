use std::collections::BTreeSet;

fn main() {
    // println!("Hello");
    let x: Vec<_> = (0..20).map(| n| fun_name(n)).collect();
    println!("{x:?}")
    // println!("edges: {edges:?}");
    // Every vertex up to the nth one will add at least 1 new edge
}

fn fun_name(n: usize) -> usize {
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
    intersections
}
