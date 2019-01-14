#[macro_export]
macro_rules! target {
    ($t : ty, $l : expr) => {
        use lazy_static::lazy_static;

        lazy_static! {
            static ref XS : Vec<$t> = {
                use rand::Rng;

                let mut rng = rand::prelude::thread_rng();
                std::iter::repeat_with(|| rng.gen()).take($l).collect()
            };
            static ref YS : Vec<$t> = {
                let mut ys = XS.clone();
                ys.sort();
                ys
            };
        }
    };
}

#[macro_export]
macro_rules! test {
    ($f : ident, $xs : expr, $ys : expr) => {
        #[test]
        fn $f() {
            let mut xs = $xs.clone();
            super::sort::$f(&mut xs);
            assert_eq!(xs, $ys);
        }
    };
}

#[macro_export]
macro_rules! benchmark {
    ($f : ident, $xs : expr) => {
        #[bench]
        fn $f(b : &mut super::test::Bencher) {
            b.iter(|| {
                let mut xs = $xs.clone();
                super::sort::$f(&mut xs)
            });
        }
    };
}

#[macro_export]
macro_rules! tests {
    ($( $f : ident ),*) => {
        #[cfg(test)]
        mod tests {
            $( test!($f, *super::XS, *super::YS); )*
        }
    };
}

#[macro_export]
macro_rules! benchmarks {
    ($( $f : ident ),*) => {
        #[cfg(test)]
        mod benchmarks {
            $( benchmark!($f, *super::XS); )*
        }
    };
}
