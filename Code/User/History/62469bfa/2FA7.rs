fn main() {
    // println!("Hello");
    let n = 5;
    let degrees = vec![0u32; n];
    let exterior_degree = n;
    // Generate every pair, count how many start and end on opposite sides of each edge?? divide by 2 to fix over counts
    
    // Every vertex up to the nth one will add at least 1 new edge
    for i in 0..(n-1) {
        

    }
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
