pub fn sort<T: Ord>(numlist: &mut [T]) -> &[T] {
    let n = numlist.len();
    if n == 1 {
        return numlist;
    } else {
        // sort by insertsorting
        // 挿入ソート:左端を処理済みとして次の要素を左と比較
        // 自分が左より小さかったら入れ替え。さらに左と比較して入れ替えが発生しなくなるまで続ける
        // 上記が終わると処理済みとして処理の要素を右に１つずらして最後まで繰り返して処理
        for i in 1..n {
            let mut j = i;
            loop {
                if j == 0 {
                    break;
                } else if numlist[j] < numlist[j - 1] {
                    numlist.swap(j, j - 1);
                    j -= 1;
                } else {
                    break;
                }
            }
        }
    }
    numlist
}
