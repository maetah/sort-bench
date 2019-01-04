#![feature(test)]

extern crate test;

#[macro_use]
mod macros;
mod sort;

mod _00008 {
    use super::*;

    target!(i32, 8);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00016 {
    use super::*;

    target!(i32, 16);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00032 {
    use super::*;

    target!(i32, 32);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00064 {
    use super::*;

    target!(i32, 64);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00128 {
    use super::*;

    target!(i32, 128);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00256 {
    use super::*;

    target!(i32, 256);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _00512 {
    use super::*;

    target!(i32, 512);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _01024 {
    use super::*;

    target!(i32, 1024);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _02048 {
    use super::*;

    target!(i32, 2048);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _04096 {
    use super::*;

    target!(i32, 4096);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _08192 {
    use super::*;

    target!(i32, 8192);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}

mod _16384 {
    use super::*;

    target!(i32, 16384);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}
