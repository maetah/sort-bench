pub fn bubble_sort<T : Ord>(xs : &mut [T]) {
    for i in 0 .. xs.len() {
        for j in 0 .. xs.len() {
            if xs[i] < xs[j] {
                xs.swap(i, j);
            }
        }
    }
}

pub fn heapsort<T : Ord>(xs : &mut [T]) {
    fn shift_down<T : Ord>(xs : &mut [T], start : usize, end : usize) {
        let mut root = start;

        loop {
            let mut child = root * 2 + 1;

            if child > end {
                break;
            }

            if child + 1 <= end && xs[child] < xs[child + 1] {
                child += 1;
            }

            if xs[root] < xs[child] {
                xs.swap(root, child);
                root = child;
            } else {
                break;
            }
        }
    }

    let len = xs.len();
    let last = len - 1;

    for start in (0 .. len / 2).rev() {
        shift_down(xs, start, last);
    }

    for end in (0 .. last).rev() {
        xs.swap(0, end + 1);
        shift_down(xs, 0, end);
    }
}

pub fn insertion_sort<T : Ord>(xs : &mut [T]) {
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

pub fn merge_sort<T : Clone + Copy + Ord>(xs : &mut [T]) {
    fn merge<T : Copy + Ord>(xs : &mut [T], ys : &[T], zs : &[T]) {
        let ylen = ys.len();
        let zlen = zs.len();

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        while j < ylen && k < zlen {
            if ys[j] < zs[k] {
                xs[i] = ys[j];
                j += 1;
            } else {
                xs[i] = zs[k];
                k += 1;
            }
            i += 1;
        }

        if j < ylen {
            xs[i ..].copy_from_slice(&ys[j ..]);
        }

        if k < zlen {
            xs[i ..].copy_from_slice(&zs[k ..]);
        }
    }

    let len = xs.len();

    let mut ys = xs.to_vec();
    let mut n = 1;
    while n < len {
        let mut i = 0;
        while i < len {
            let j = len.min(i + n);
            let k = len.min(i + 2 * n);
            merge(&mut ys[i .. k], &xs[i .. j], &xs[j .. k]);
            xs[i .. k].copy_from_slice(&ys[i .. k]);
            i += 2 * n;
        }
        n *= 2;
    }
}

pub fn quicksort<T : Ord>(xs : &mut [T]) {
    fn partition<T : Ord>(xs : &mut [T]) -> usize {
        let len = xs.len();
        let last = len - 1;

        xs.swap(len / 2, last);

        let mut i = 0;
        for j in 0 .. last {
            if xs[j] < xs[last] {
                xs.swap(i, j);
                i += 1;
            }
        }

        xs.swap(i, last);
        i
    }

    let len = xs.len();
    if len >= 2 {
        let p = partition(xs);
        quicksort(&mut xs[0 .. p]);
        quicksort(&mut xs[p + 1 .. len]);
    }
}

pub fn selection_sort<T : Ord>(xs : &mut [T]) {
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
