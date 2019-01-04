#![feature(test)]

extern crate test;

#[macro_use]
mod macros;
mod sort;

mod i8_32 {
    use super::*;

    target!(i8, 32);
    tests!(bubble_sort, insertion_sort, quicksort, selection_sort);
    benchmarks!(bubble_sort, insertion_sort, quicksort, selection_sort);
}

mod i64_1024 {
    use super::*;

    target!(i64, 1024);
    tests![bubble_sort, insertion_sort, quicksort, selection_sort];
    benchmarks![bubble_sort, insertion_sort, quicksort, selection_sort];
}
