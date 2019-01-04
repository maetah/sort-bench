pub fn bubble<T : Ord>(xs : &mut [T]) {
    for i in 0 .. xs.len() {
        for j in 0 .. xs.len() {
            if xs[i] < xs[j] {
                xs.swap(i, j);
            }
        }
    }
}

pub fn insertion<T : Ord>(xs : &mut [T]) {
    for i in 0 .. xs.len() - 1 {
        for j in (0 .. i + 1).rev() {
            if xs[j] > xs[j + 1] {
                xs.swap(j, j + 1);
            } else {
                break;
            }
        }
    }
}

pub fn quick<T : Ord>(xs : &mut [T]) {
    fn partition<T : Ord>(ys : &mut [T]) -> usize {
        let len = ys.len();
        let last = len - 1;

        ys.swap(len / 2, last);

        let mut i = 0;
        for j in 0 .. last {
            if ys[j] < ys[last] {
                ys.swap(i, j);
                i += 1;
            }
        }

        ys.swap(i, last);
        i
    }

    let len = xs.len();
    if len >= 2 {
        let p = partition(xs);
        quick(&mut xs[0 .. p]);
        quick(&mut xs[p + 1 .. len]);
    }
}

pub fn selection<T : Ord>(xs : &mut [T]) {
    let len = xs.len();
    for i in 0 .. len {
        let mut j = i;
        for k in i + 1 .. len {
            if xs[j] > xs[k] {
                j = k;
            }
        }
        xs.swap(i, j);
    }
}
