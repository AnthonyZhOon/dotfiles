fn main() {
    // println!("Hello");
    let n = 5;
    let degrees = vec![0; n];
    let exterior_degree = n;
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
