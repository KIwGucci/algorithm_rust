mod bubblesort;
fn main() {
    let mut numlist = [2, 13, 6, 4, 3, 1, 5, 7, 9, 8, 10, 2, 3, 800, 30, 40, 50];
    let mut numlist2:Vec<u32> = vec![99,541,4,784,5,3];
    let mut numlist3:Vec<char> = "test is success".chars().collect();
    bubblesort::sort(&mut numlist);
    bubblesort::sort(&mut numlist2);
    bubblesort::sort(&mut numlist3);
    println!("{:?}",numlist);
    println!("{:?}",numlist2);
    println!("{:?}",numlist3);
}
