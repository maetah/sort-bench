#![feature(test)]

extern crate test;

#[macro_use]
mod macros;
mod sort;

mod i8_32 {
    use super::*;

    target!(i8, 32);
    tests![bubble, insertion];
    benchmarks![bubble, insertion];
}

mod i8_1024 {
    use super::*;

    target!(i8, 1024);
    tests![bubble, insertion];
    benchmarks![bubble, insertion];
}
