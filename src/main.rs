mod algo;
mod test;

fn runner_linear_search() {
    let arr = [1, 5, 3, 6, 8, 10];
    println!("{}", algo::linear_search(&arr, 10));
    println!("{}", algo::linear_search(&arr, 11));
}

fn main() {
    // runner_linear_search();

    let value = f64::sqrt(5.0);
    let v2 = value as u64;
    let v3 = v2;
    println!("{}", algo::two_crystal_balls(&[false, false, false, true]));
}
