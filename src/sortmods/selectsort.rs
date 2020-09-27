fn minimum_index<T: Ord>(numlist: &[T]) -> usize {
    let mut min_num = &numlist[0];
    let mut min_index = 0;
    for (i, n) in numlist.iter().enumerate() {
        if min_num > n {
            min_num = n;
            min_index = i;
        }
    }
    min_index
}

pub fn sort<T: Ord>(numlist: &mut [T]) -> () {
    // sort numlist with select sort
    let n = numlist.len();

    // 選択ソート
    // リストの中で最小の値を選択肢左端に持っていく。
    // 本来は線形探索で最小値を見つけるがここでは組み込み関数を使う
    if n == 0{
        ()
    } else {
        let min_index = minimum_index(numlist);
        numlist.swap(0, min_index);
        let (_head,tail) = numlist.split_first_mut().unwrap();
        sort(tail);
        ()
    }
}
