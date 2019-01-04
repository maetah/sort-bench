#![feature(test)]

extern crate test;

#[macro_use]
mod macros;
mod sort;

mod i8_32 {
    use super::*;

    target!(i8, 32);
    tests![bubble, insertion, quick, selection];
    benchmarks![bubble, insertion, quick, selection];
}

mod i64_1024 {
    use super::*;

    target!(i64, 1024);
    tests![bubble, insertion, quick, selection];
    benchmarks![bubble, insertion, quick, selection];
}
