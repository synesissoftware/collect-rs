// benches/unicode-point-map.rs : benchmarking of `UnicodePointMap`

#![allow(non_snake_case)]

use collect_rs::containers::UnicodePointMap;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion,
};

use std::{
    fmt as std_fmt,
    ops as std_ops,
};


mod constants {}


mod implementation {
    use criterion::BenchmarkId;

    /*
    pub(super) fn make_id(
        benchmarked_function_name : &'static str,
    ) -> BenchmarkId {
        let parameter = "";

        BenchmarkId::new(
            format!("`{benchmarked_function_name}()`"),
            parameter,
        )
    }
     */
}


fn BENCHMARK_UnicodePointMap_default(c : &mut Criterion) {
    let id = "`UnicodePointMap::default()`";

    c.bench_function(id, |b| {
        b.iter(|| {
            let upm = black_box(UnicodePointMap::default());

            let _ = black_box(upm);
        })
    });
}

fn BENCHMARK_UnicodePointMap_new_WITH_(
    c : &mut Criterion,
    default_contiguous_ceiling : char,
) {
    let id = format!("`UnicodePointMap::new({:})`", default_contiguous_ceiling as u64);

    c.bench_function(&id, |b| {
        b.iter(|| {
            let upm = black_box(UnicodePointMap::new(default_contiguous_ceiling));

            let _ = black_box(upm);
        })
    });
}

fn BENCHMARK_UnicodePointMap_new_0(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, 0 as char);
}
fn BENCHMARK_UnicodePointMap_new_1(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, 1 as char);
}
fn BENCHMARK_UnicodePointMap_new_10(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, 10 as char);
}
fn BENCHMARK_UnicodePointMap_new_100(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, 100 as char);
}
fn BENCHMARK_UnicodePointMap_new_1000(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, '\u{3E8}');
}
fn BENCHMARK_UnicodePointMap_new_10000(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, '\u{2710}');
}
fn BENCHMARK_UnicodePointMap_new_100000(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, '\u{186A0}');
}
fn BENCHMARK_UnicodePointMap_new_1000000(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_new_WITH_(c, '\u{F4240}');
}

fn BENCHMARK_UnicodePointMap_FromIterator_WITH_(
    c : &mut Criterion,
    r : std_ops::RangeInclusive<char>,
)
// where
//     R : Clone + std_fmt::Debug + std_ops::RangeBounds<char>,
{
    let id = format!("`UnicodePointMap::from_iter({:?})`", r);

    c.bench_function(&id, |b| {
        b.iter(|| {
            let upm = black_box(UnicodePointMap::from_iter(r.clone().into_iter()));

            let _ = black_box(upm);
        })
    });
}

fn BENCHMARK_UnicodePointMap_FromIterator_a_TO_d(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, 'a'..='d');
}
fn BENCHMARK_UnicodePointMap_FromIterator_a_TO_z(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, 'a'..='z');
}
fn BENCHMARK_UnicodePointMap_FromIterator_u1_TO_u100(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, '\u{1}'..='\u{300}');
}
fn BENCHMARK_UnicodePointMap_FromIterator_u101_TO_u200(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, '\u{101}'..='\u{200}');
}
fn BENCHMARK_UnicodePointMap_FromIterator_u201_TO_u300(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, '\u{201}'..='\u{300}');
}
fn BENCHMARK_UnicodePointMap_FromIterator_u100_TO_u300(c : &mut Criterion) {
    BENCHMARK_UnicodePointMap_FromIterator_WITH_(c, '\u{100}'..='\u{300}');
}


criterion_group!(
    benches,
    // empty construction
    BENCHMARK_UnicodePointMap_default,
    BENCHMARK_UnicodePointMap_new_0,
    BENCHMARK_UnicodePointMap_new_1,
    BENCHMARK_UnicodePointMap_new_10,
    BENCHMARK_UnicodePointMap_new_100,
    BENCHMARK_UnicodePointMap_new_1000,
    BENCHMARK_UnicodePointMap_new_10000,
    BENCHMARK_UnicodePointMap_new_100000,
    BENCHMARK_UnicodePointMap_new_1000000,
    // FromIterator construction
    BENCHMARK_UnicodePointMap_FromIterator_a_TO_d,
    BENCHMARK_UnicodePointMap_FromIterator_a_TO_z,
    BENCHMARK_UnicodePointMap_FromIterator_u1_TO_u100,
    BENCHMARK_UnicodePointMap_FromIterator_u101_TO_u200,
    BENCHMARK_UnicodePointMap_FromIterator_u201_TO_u300,
    BENCHMARK_UnicodePointMap_FromIterator_u100_TO_u300,
);
criterion_main!(benches);


/* ///////////////////////////// end of file //////////////////////////// */
