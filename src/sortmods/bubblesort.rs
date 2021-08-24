pub fn sort<T: Ord>(numlist: &mut [T]) {
    let n = numlist.len();
    if n != 1 {
        // bubblesort
        // 右から順に隣り合った数を比較して小さいほうが左になるように入れ替える。
        for i in (1..n).rev() {
            if numlist[i] < numlist[i - 1] {
                numlist.swap(i, i - 1);
            }
        }
        // 上記をn-1回繰り返し、最も左を最小値として確定し残りのそれ以外の数列で
        // 同様の処理を行う
        let (_head, tail) = numlist.split_first_mut().unwrap();
        // ここでは再帰的に処理する。上記処理は変更可能参照ポインタで処理しているので
        // 元の数列の長さは変化しない。
        // 比較対象を参照ポインタを移動しながら比較し入れ替え処理を行っている。
        sort(tail)
    }else{

    }
}
