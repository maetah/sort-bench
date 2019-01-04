#![feature(test)]

extern crate test;

#[macro_use]
mod macros;
mod sort;

mod i8_32 {
    use super::*;

    target!(i8, 32);
    tests!(bubble_sort, heapsort, insertion_sort, quicksort, selection_sort);
    benchmarks!(bubble_sort, heapsort, insertion_sort, quicksort, selection_sort);
}

mod i32_1024 {
    use super::*;

    target!(i32, 1024);
    tests![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, heapsort, insertion_sort, quicksort, selection_sort];
}
