use php5rand::{Php5MtRandom, Php5Random};

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_rand_new() {
    divan::black_box(Php5Random::new(divan::black_box(1)));
}

#[divan::bench]
fn bench_rand_srand() {
    let mut r = Php5Random::new(1);
    r.srand(divan::black_box(42));
    divan::black_box(r);
}

#[divan::bench]
fn bench_rand_rand() {
    let mut r = Php5Random::new(1);
    divan::black_box(r.rand());
}

#[divan::bench]
fn bench_rand_rand_range() {
    let mut r = Php5Random::new(1);
    divan::black_box(r.rand_range(divan::black_box(0), divan::black_box(100)));
}

#[divan::bench]
fn bench_rand_1000_calls() {
    let mut r = Php5Random::new(1);
    for _ in 0..1000 {
        divan::black_box(r.rand());
    }
}

#[divan::bench]
fn bench_mt_new() {
    divan::black_box(Php5MtRandom::new(divan::black_box(1)));
}

#[divan::bench]
fn bench_mt_srand() {
    let mut r = Php5MtRandom::new(1);
    r.srand(divan::black_box(42));
    divan::black_box(r);
}

#[divan::bench]
fn bench_mt_rand() {
    let mut r = Php5MtRandom::new(1);
    divan::black_box(r.rand());
}

#[divan::bench]
fn bench_mt_rand_range() {
    let mut r = Php5MtRandom::new(1);
    divan::black_box(r.rand_range(divan::black_box(0), divan::black_box(100)));
}

#[divan::bench]
fn bench_mt_1000_calls() {
    let mut r = Php5MtRandom::new(1);
    for _ in 0..1000 {
        divan::black_box(r.rand());
    }
}
