fn main() {
    // println!("Hello");
    let n = 6;
    let degrees = vec![0u32; n];
    let exterior_degree = n;
    let mut edges = Vec::<(usize, usize)>::new();
    // Generate every pair, count how many start and end on opposite sides of each edge?? divide by 2 to fix over counts
    let mut intersections = 0usize;
    for i in 0..(n-1) {
        'x: for x in 0..n {
            'y: for y in 0..n {
                if x == i {
                    continue 'x;
                } else if y == i {
                    continue 'y;
                }
                if x < i &&  i < y {
                    intersections += 1;
                    edges.push((x, y));
                }
            }
        } 
    }
    println!("n = {n}, intersections = {intersections}");
    // Every vertex up to the nth one will add at least 1 new edge
}



const fn total_degree(n: usize) -> usize {
    n * (n - 1) / 2
}

const fn basic_degree(n: usize) -> usize {
    2
}

const fn max_degree(n: usize) -> usize {
    n-1
}

fn edges_added(n: usize, curr_degree: usize) -> usize {
    max_degree(n) - curr_degree
}
