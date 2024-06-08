mod sortmods;
use std::fmt::Debug;

use sortmods::heapsort;
fn mysort<T: Ord + Debug + Clone>(x: &mut [T]) -> Vec<T> {
    heapsort::sort(x)
}

fn main() {
    let mut numlist2: Vec<u32> = vec![99, 541, 4, 784, 5, 3, 239, 34, 0, 3432, 45, 2, 67, 699];
    let mut numlist3: Vec<char> = "test is success".chars().collect();
    numlist2 = mysort(&mut numlist2);
    numlist3 = mysort(&mut numlist3);
    println!("{:?}", numlist2);
    println!("{:?}", numlist3);
}
