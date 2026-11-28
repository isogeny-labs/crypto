use hec_native_ic::{
    counted,
    instance::pick_instance,
    jac::{Curve, Div},
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
const L: u64 = 239753;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/finish.rs"]
mod finish;
#[path = "../hec_residual_span_20261116/native-pencil-images/images.rs"]
mod images;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/linear.rs"]
mod linear;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/pencil.rs"]
mod pencil;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/prepared.rs"]
mod prepared;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/public.rs"]
mod public;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/small.rs"]
mod small;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/table_oracle.rs"]
mod table_oracle;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/tail.rs"]
mod tail;
#[path = "../hec_residual_span_20261116/native-deferred-membership/validate.rs"]
mod validate;
fn baseline(c: &Curve, d: &Div, t: &Div, case: usize) -> Option<u64> {
    assert!(validate::eligible(c, d, t, L));
    let prep = prepared::prepare(c, false);
    pencil::solve(
        c,
        d,
        t,
        L,
        512,
        false,
        &prep,
        pencil::Options {
            normalize: true,
            finish: false,
            seed: 2026112200 ^ 31 ^ (4u64 << 32) ^ case as u64,
        },
    )
    .k
}
fn endpoints(c: &Curve, d: &Div, t: &Div, source_count: usize, target_count: usize) -> (Div, Div) {
    let mut s = c.zero();
    for _ in 0..source_count {
        s = c.add(&s, d);
    }
    let mut a = *t;
    for _ in 1..target_count {
        a = c.add(&a, d);
    }
    (s, a)
}
fn main() {
    let inst = pick_instance(31, 4).unwrap();
    assert_eq!((inst.c, inst.ell), (16, L));
    let c = &inst.curve;
    let mut rng = Rng::new(2026111800 ^ 31 ^ (4u64 << 32));
    let raw = include_str!("../hec_g4_original_descent_20261118/CONTROL.txt");
    let val = |s: &str, k: &str| {
        s.split_whitespace()
            .find_map(|w| w.strip_prefix(k))
            .unwrap()
            .parse::<usize>()
            .unwrap()
    };
    let counts: Vec<_> = raw
        .lines()
        .filter(|s| s.starts_with("VERIFY "))
        .map(|s| (val(s, "source_anchors="), val(s, "target_anchors=")))
        .collect();
    assert_eq!(counts, [(10, 39), (48, 153)]);
    for case in 0..2 {
        counted::disable();
        let d = c.mul(&inst.d1, if case == 0 { 1 } else { 17 });
        let k = 1 + rng.below(L - 1);
        let t = c.mul(&d, k);
        let (sc, tc) = counts[case];
        let expected = (c.mul(&d, sc as u64), c.add(&t, &c.mul(&d, tc as u64 - 1)));
        let mut walls = [Vec::new(), Vec::new()];
        for rep in 0..=41 {
            for arm in if (case + rep) % 2 == 0 {
                [0, 1]
            } else {
                [1, 0]
            } {
                let start = Instant::now();
                let elapsed = if arm == 0 {
                    let r = black_box(baseline(c, black_box(&d), black_box(&t), case));
                    let n = start.elapsed().as_nanos();
                    assert_eq!(r, Some(k));
                    n
                } else {
                    let r = black_box(endpoints(c, black_box(&d), black_box(&t), sc, tc));
                    let n = start.elapsed().as_nanos();
                    assert_eq!(r, expected);
                    n
                };
                if rep != 0 {
                    walls[arm].push(elapsed);
                    println!("WALL case={case} rep={rep} arm={arm} ns={elapsed}");
                }
            }
        }
        let mut vecs = vec![];
        for arm in 0..2 {
            counted::reset();
            if arm == 0 {
                assert_eq!(baseline(c, &d, &t, case), Some(k));
            } else {
                assert_eq!(endpoints(c, &d, &t, sc, tc), expected);
            }
            let counts = counted::snapshot();
            counted::disable();
            let mut red = [0; 2];
            for (p, v) in counts {
                let i = if p == 31 {
                    0
                } else {
                    assert_eq!(p, L);
                    1
                };
                red[i] = v[8];
                println!("COUNT case={case} arm={arm} modulus={p} vector={v:?}");
            }
            vecs.push(red);
        }
        for v in &mut walls {
            v.sort();
        }
        println!(
            "SUMMARY case={case} calls={} baseline_ns={} anchors_ns={} base_subgroup_redc={vecs:?}",
            sc + tc - 1,
            walls[0][20],
            walls[1][20]
        );
    }
}
