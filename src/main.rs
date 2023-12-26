mod algo;
mod test;

fn runner_linear_search() {
    let arr = [1,5,3,6,8,10];
    println!("{}", algo::lenear_search(&arr, 10));
    println!("{}", algo::lenear_search(&arr, 11));
}

fn main() {
    runner_linear_search();
}
