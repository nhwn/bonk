use bonk::{bonk, Bonk};
use itertools::iproduct;

use criterion::{criterion_group, criterion_main, Criterion};

fn iproduct_macro() {
    static A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    static D: &[u8] = b"0123456789";
    let mut buf = [b'S', b'K', b'Y', b'-', 0, 0, 0, 0, b'-', 0, 0, 0, 0];
    for (a0, a1, a2, a3, d0, d1, d2, d3) in iproduct!(
        A.iter().copied(),
        A.iter().copied(),
        A.iter().copied(),
        A.iter().copied(),
        D.iter().copied(),
        D.iter().copied(),
        D.iter().copied(),
        D.iter().copied()
    ) {
        buf[4] = a0;
        buf[5] = a1;
        buf[6] = a2;
        buf[7] = a3;
        buf[9] = d0;
        buf[10] = d1;
        buf[11] = d2;
        buf[12] = d3;
        if &buf == b"SKY-ZZZZ-9999" {
            println!("{}", unsafe { std::str::from_utf8_unchecked(&buf) });
            return;
        }
    }
}

struct S;

impl Bonk for S {
    fn new(_id: usize) -> Self {
        Self
    }
    fn check(&mut self, buf: &[u8]) -> bool {
        if &buf == b"SKY-ZZZZ-9999" {
            println!("{}", unsafe { std::str::from_utf8_unchecked(&buf) });
            true
        } else {
            false
        }
    }
}

fn bonk_macro() {
    bonk!(r"SKY-\A{4}-\d{4}", S);
}

fn bench_macros(c: &mut Criterion) {
    let mut group = c.benchmark_group("macros");
    group.bench_function("iproduct", |b| b.iter(|| iproduct_macro()));
    group.bench_function("bonk", |b| b.iter(|| bonk_macro()));
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_macros
);

criterion_main!(benches);
