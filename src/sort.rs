type Array<T> = [T];

pub fn bubble<T : Ord>(xs : &mut Array<T>) {
    for i in 0 .. xs.len() {
        for j in 0 .. xs.len() {
            if xs[i] < xs[j] {
                xs.swap(i, j);
            }
        }
    }
}

pub fn insertion<T : Ord>(xs : &mut Array<T>) {
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