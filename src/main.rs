pub mod sortmods;

fn mysort<T: Ord>(x: &mut [T]) -> () {
    sortmods::quicksort::sort(x, 0, 0)
}

fn main() {
    let mut numlist = [2, 13, 6, 4, 3, 1, 5, 7, 9, 8, 10, 2, 3, 800, 30, 40, 50];
    let mut numlist2: Vec<u32> = vec![99, 541, 4, 784, 5, 3,34,0,3432,45,67,699];
    let mut numlist3: Vec<char> = "test is success".chars().collect();

    mysort(&mut numlist);
    mysort(&mut numlist2);
    mysort(&mut numlist3);
    println!("{:?}", numlist);
    println!("{:?}", numlist2);
    println!("{:?}", numlist3);
}
