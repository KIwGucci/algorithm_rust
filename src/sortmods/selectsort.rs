fn minimum_index<T:Ord>(numlist:&[T])->usize{
    let mut min_num = numlist.first().unwrap();
    let mut min_index = 0;
    for (i,n) in numlist.iter().enumerate(){
        if min_num > n{
            min_num = n;
            min_index = i;
        }
    }
    min_index
    }

pub fn sort<T: Ord>(numlist: &mut [T],mut startnum:usize) -> &[T] {
    // sort numlist with select sort
    let n = numlist.len();

    // 選択ソート
    // リストの中で最小の値を選択肢左端に持っていく。
    // 本来は線形探索で最小値を見つけるがここでは組み込み関数を使う
    if n == startnum{
        numlist
    }else{
        let min_index = minimum_index(numlist);
        numlist.swap(startnum, min_index);
        startnum += 1;
        sort(numlist,startnum);
        numlist
    }
}