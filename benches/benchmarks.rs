#![feature(test)]

extern crate test;
extern crate iter_or;

use iter_or::IterOr;
use std::iter;

#[bench]
fn vector_and_count_empty(b: &mut test::Bencher) {
    b.iter(|| {
        let result: Vec<i32> = iter::empty().collect();
        let default = 27;

        if result.is_empty() {
            test::black_box(default);
        } else {
            for i in result {
                test::black_box(i);
            }
        }
    });
}

#[bench]
fn vector_and_count_10(b: &mut test::Bencher) {
    b.iter(|| {
        let result: Vec<i32> = (0..10).collect();
        let default = 27;

        if result.is_empty() {
            test::black_box(default);
        } else {
            for i in result {
                test::black_box(i);
            }
        }
    });
}

#[bench]
fn vector_and_count_1000(b: &mut test::Bencher) {
    b.iter(|| {
        let result: Vec<i32> = (0..1000).collect();
        let default = 27;

        if result.is_empty() {
            test::black_box(default);
        } else {
            for i in result {
                test::black_box(i);
            }
        }
    });
}

#[bench]
fn vector_and_count_100000(b: &mut test::Bencher) {
    b.iter(|| {
        let result: Vec<i32> = (0..100000).collect();
        let default = 27;

        if result.is_empty() {
            test::black_box(default);
        } else {
            for i in result {
                test::black_box(i);
            }
        }
    });
}

#[bench]
fn iterator_empty(b: &mut test::Bencher) {
    b.iter(|| {
        for i in iter::empty().or(27) {
            test::black_box(i);
        }
    })
}

#[bench]
fn iterator_10(b: &mut test::Bencher) {
    b.iter(|| {
        for i in (0..10).or(27) {
            test::black_box(i);
        }
    })
}

#[bench]
fn iterator_1000(b: &mut test::Bencher) {
    b.iter(|| {
        for i in (0..1000).or(27) {
            test::black_box(i);
        }
    })
}

#[bench]
fn iterator_100000(b: &mut test::Bencher) {
    b.iter(|| {
        for i in (0..100000).or(27) {
            test::black_box(i);
        }
    })
}
