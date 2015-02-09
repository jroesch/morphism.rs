#![feature(box_syntax)]
#![feature(core)]
#![feature(unboxed_closures)]
#![feature(test)]

extern crate morphism;
extern crate test;

use morphism::Morphism;

#[bench]
fn new_closure_unboxed(b: &mut test::Bencher) {
    b.iter(|&:| {
        (|&:| {});
    });
}

#[bench]
fn new_closure_boxed(b: &mut test::Bencher) {
    b.iter(|&:| {
        (box |&:| {});
    });
}

#[bench]
fn new_morphism(b: &mut test::Bencher) {
    b.iter(|&:| {
        Morphism::new::<()>();
    });
}

#[bench]
fn push_front(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        f.push_front(|&: _| {});
    });
}

#[bench]
fn push_front_10(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 10u64) {
            f.push_front(|&: _| {});
        };
    });
}

#[bench]
fn push_front_100(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 100u64) {
            f.push_front(|&: _| {});
        };
    });
}

#[bench]
fn push_front_1000(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 1000u64) {
            f.push_front(|&: _| {});
        };
    });
}

#[bench]
fn push_back(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        f.push_back(|&: _| {});
    });
}

#[bench]
fn push_back_10(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 10u64) {
            f.push_back(|&: _| {});
        };
    });
}

#[bench]
fn push_back_100(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 100u64) {
            f.push_back(|&: _| {});
        };
    });
}

#[bench]
fn push_back_1000(b: &mut test::Bencher) {
    let f = &mut Morphism::new::<()>();
    b.iter(|| {
        for _ in range(0u64, 1000u64) {
            f.push_back(|&: _| {})
        };
    });
}

#[bench]
fn run_closure_boxed(b: &mut test::Bencher) {
    let f = box |&:| {};
    b.iter(|&:| {
        f();
    });
}

#[bench]
fn run_closure_unboxed(b: &mut test::Bencher) {
    let f = &(|&:| {});
    b.iter(|&:| {
        (*f)();
    });
}

#[bench]
fn run_morphism(b: &mut test::Bencher) {
    let f = Morphism::new::<()>().tail(|&: _| {});
    b.iter(|&:| {
        f(());
    });
}

#[bench]
fn run_morphism_10(b: &mut test::Bencher) {
    let mut f = Morphism::new::<()>();
    for _ in range(0u64, 10u64) {
        (&mut f).push_back(|&: _| {})
    };
    b.iter(|&:| {
        f(());
    });
}

#[bench]
fn run_morphism_100(b: &mut test::Bencher) {
    let mut f = Morphism::new::<()>();
    for _ in range(0u64, 100u64) {
        (&mut f).push_back(|&: _| {})
    };
    b.iter(|&:| {
        f(());
    });
}

#[bench]
fn run_morphism_1000(b: &mut test::Bencher) {
    let mut f = Morphism::new::<()>();
    for _ in range(0u64, 1000u64) {
        (&mut f).push_back(|&: _| {})
    };
    b.iter(|&:| {
        f(());
    });
}

