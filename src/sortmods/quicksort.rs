pub fn sort<T: Ord>(numlist: &mut [T], lq: usize, rq: usize) -> () {
    //実体ではなくmut参照で受け取り変更を加えるので返り値は()
    let numslength = numlist.len();
    let endnum: usize = if rq > 0 { rq } else { numslength - 1 };
    let startnum: usize = if lq > 0 { lq } else { 0 };
    let arraylength = endnum - startnum + 1;
    if arraylength <= 1 {
        // 何もしない
        return ();
    } else if arraylength == 2 {
        if numlist[startnum] > numlist[endnum] {
            numlist.swap(startnum, endnum);
            return ();
        } else {
            return ();
        }
    }
    let pivotposi = endnum;
    let mut rstart = endnum;
    // 左マーカを右に動かしてpivot以上の数にたどり着いたらstop
    for lmarker in startnum..endnum {
        if numlist[lmarker] >= numlist[pivotposi] {
            // 左マーカがpivot以上の時左マーカはいったん止まる
            'rturn: for rmarker in (startnum..rstart + 1).rev() {
                // 右マーカを左に移動開始
                if rmarker == lmarker {
                    // 右と左のマーカがぶつかった時,pivotとマーカの値を入れ替え
                    numlist.swap(rmarker, endnum);
                    // pivotの以前を処理(pivotより小さい数列)
                    sort(numlist, startnum, rmarker);
                    // pivot以降を処理(pivotより大きい数列)
                    sort(numlist, rmarker + 1, endnum);
                    return ();
                // let rnum = numlist[rmarker];
                } else if numlist[rmarker] < numlist[pivotposi] {
                    // 右マーカがpivotより小さい時
                    rstart = rmarker;
                    numlist.swap(rmarker, lmarker);
                    break 'rturn;
                }
            }
        }
    }
    sort(numlist, startnum, endnum - 1);
    return ();
}
