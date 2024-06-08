mod sortmods;
use std::fmt::Debug;

use sortmods::heapsort;
fn mysort<T: Ord + Debug + Clone>(x: &mut [T]) -> Vec<T> {
    heapsort::sort(x)
}

fn main() {
    let mut numlist2: Vec<u32> = vec![99, 541, 4, 784, 5, 3, 239, 34, 0, 3432, 45, 2, 67, 699];
    let mut numlist3: Vec<char> = "test is success".chars().collect();
    let mynumlist2 = mysort(&mut numlist2);
    let mynumlist3 = mysort(&mut numlist3);
    numlist2.sort();
    numlist3.sort();
    assert_eq!(numlist2, mynumlist2);
    assert_eq!(numlist3, mynumlist3);
}
