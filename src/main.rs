#![feature(test)]

extern crate test;

mod sort;

macro_rules! xs {
    () => {
        [9, 1, 0, 5, 6, 8, 4, 2, 7, 3]
    }
}

macro_rules! ys {
    () => {
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}

macro_rules! test {
    ($f : ident, $xs : expr, $ys : expr) => {
        #[test]
        fn $f() {
            let mut xs = $xs;
            super::sort::$f(&mut xs);
            assert_eq!(xs, $ys);
        }
    }
}

macro_rules! benchmark {
    ($f : ident, $xs : expr) => {
        #[bench]
        fn $f(b : &mut super::test::Bencher) {
            let mut xs = $xs;
            b.iter(|| super::sort::insertion(&mut xs));
        }
    }
}

macro_rules! tests {
    ($( $f : ident ),*) => {
        #[cfg(test)]
        mod tests {
            $( test!($f, xs!(), ys!()); )*
        }
    }
}

macro_rules! benchmarks {
    ($( $f : ident ),*) => {
        #[cfg(test)]
        mod benchmarks {
            $( benchmark!($f, xs!()); )*
        }
    }
}

tests![bubble, insertion];
benchmarks![bubble, insertion];