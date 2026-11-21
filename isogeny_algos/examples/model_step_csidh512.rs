//! Same-parameter Montgomery and Edwards successors on the CSIDH-512 prime.
//! Run: cargo run --release --example model_step_csidh512
use isogeny_algos::field::{Field, Rng};
use isogeny_algos::fpm::FpM;
use isogeny_algos::kernel::models::{
    edwards_from_montgomery, edwards_isogeny, mont_to_edwards_point,
};
use isogeny_algos::kernel::montgomery::{self, XZ};
use isogeny_algos::path::csidh::Csidh;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::Instant;

fn rhs<F: Field>(f: &F, a: F::E, x: F::E) -> F::E {
    f.mul(x, f.add(f.add(f.mul(x, x), f.mul(a, x)), f.one()))
}

fn sample_kernel<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, rng: &mut Rng) -> XZ<F::E> {
    let f = &cs.fp;
    let cof = cs.p1.divrem_small(ell).0;
    loop {
        let x = f.random(rng);
        let r = rhs(f, a, x);
        if f.is_zero(r) || f.sqrt(r).is_none() {
            continue;
        }
        let k = montgomery::ladder_xz(f, montgomery::a24(f, a), (x, f.one()), &cof);
        if !f.is_zero(k.1) {
            return k;
        }
    }
}

fn mont_successor<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, k: XZ<F::E>) -> F::E {
    let f = &cs.fp;
    let ms = montgomery::multiples(f, montgomery::a24(f, a), k, ((ell - 1) / 2) as usize);
    montgomery::velu_codomain(f, a, &ms, ell)
}

fn edwards_successor<F: Field>(cs: &Csidh<F>, a: F::E, ell: u64, k: XZ<F::E>) -> F::E {
    let f = &cs.fp;
    let x = f.div(k.0, k.1);
    let y = f
        .sqrt(rhs(f, a, x))
        .expect("positive-direction kernel is rational");
    let ed = edwards_from_montgomery(f, a);
    let ke = mont_to_edwards_point(f, x, y);
    assert!(ed.on_curve(f, ke));
    let iso = edwards_isogeny(f, &ed, ke, ell);
    let two = f.from_u64(2);
    f.div(
        f.mul(two, f.add(iso.cod.a, iso.cod.d)),
        f.sub(iso.cod.a, iso.cod.d),
    )
}

fn step<F: Field>(cs: &Csidh<F>, a: F::E, idx: usize, rng: &mut Rng, edwards: bool) -> F::E {
    let ell = cs.primes[idx];
    let k = sample_kernel(cs, a, ell, rng);
    if edwards {
        edwards_successor(cs, a, ell, k)
    } else {
        mont_successor(cs, a, ell, k)
    }
}

fn sparse_action<F: Field>(cs: &Csidh<F>, indices: &[usize], seed: u64, edwards: bool) -> F::E {
    let mut rng = Rng::new(seed);
    let mut a = cs.fp.zero();
    for &idx in indices {
        a = step(cs, a, idx, &mut rng, edwards);
    }
    a
}

fn frontier<F: Field>(cs: &Csidh<F>, a: F::E, rng: &mut Rng, edwards: bool) -> [F::E; 4] {
    let a0 = step(cs, a, 0, rng, edwards);
    let a1 = step(cs, a, 1, rng, edwards);
    let a01 = step(cs, a0, 1, rng, edwards);
    [a, a0, a1, a01]
}

fn bounded_path_search<F: Field>(
    cs: &Csidh<F>,
    target: F::E,
    seed: u64,
    edwards: bool,
) -> Option<(usize, usize)> {
    let mut rng = Rng::new(seed);
    let left = frontier(cs, cs.fp.zero(), &mut rng, edwards);
    let mut index = HashMap::new();
    for (i, &a) in left.iter().enumerate() {
        index.entry(a).or_insert(i);
    }
    let right = frontier(cs, target, &mut rng, edwards);
    for (j, a) in right.iter().enumerate() {
        if let Some(&i) = index.get(a) {
            return Some((i, j));
        }
    }
    None
}

fn median_ns(mut f: impl FnMut(), repeats: usize) -> u128 {
    let mut times = Vec::with_capacity(repeats);
    for _ in 0..repeats {
        let t = Instant::now();
        f();
        times.push(t.elapsed().as_nanos());
    }
    times.sort_unstable();
    times[repeats / 2]
}

fn main() {
    let cs = Csidh::<FpM<8>>::csidh512();
    let f = &cs.fp;
    println!("field,task,degree_or_indices,arm,median_ns,verified");
    for &ell in &[3u64, 5, 7, 31, 587] {
        let idx = cs.primes.iter().position(|&v| v == ell).unwrap();
        let k = sample_kernel(&cs, f.zero(), ell, &mut Rng::new(10_000 + ell));
        let ma = mont_successor(&cs, f.zero(), ell, k);
        let ea = edwards_successor(&cs, f.zero(), ell, k);
        assert_eq!(ma, ea, "Montgomery successor differs at degree {ell}");
        assert_eq!(
            montgomery::j_invariant(f, ma),
            montgomery::j_invariant(f, ea)
        );
        let reference = cs.step(f.zero(), idx, true, &mut Rng::new(20_000 + ell));
        let ours = step(&cs, f.zero(), idx, &mut Rng::new(20_000 + ell), false);
        let ours_edwards = step(&cs, f.zero(), idx, &mut Rng::new(20_000 + ell), true);
        assert_eq!(reference, ours);
        assert_eq!(reference, ours_edwards);
        for (arm, edwards) in [
            ("montgomery_x_only", false),
            ("edwards_with_conversion", true),
        ] {
            let known = median_ns(
                || {
                    let a = if edwards {
                        edwards_successor(&cs, f.zero(), ell, black_box(k))
                    } else {
                        mont_successor(&cs, f.zero(), ell, black_box(k))
                    };
                    black_box(a);
                },
                11,
            );
            println!("CSIDH-512,known_kernel,{ell},{arm},{known},true");
            let full = median_ns(
                || {
                    black_box(step(
                        &cs,
                        f.zero(),
                        idx,
                        &mut Rng::new(30_000 + ell),
                        edwards,
                    ));
                },
                5,
            );
            println!("CSIDH-512,full_positive_step,{ell},{arm},{full},true");
        }
    }
    let indices = [0usize, 1, 2];
    let start_seed = 40_000;
    let ma = sparse_action(&cs, &indices, start_seed, false);
    let ea = sparse_action(&cs, &indices, start_seed, true);
    assert_eq!(ma, ea);
    for (arm, edwards) in [
        ("montgomery_x_only", false),
        ("edwards_with_conversion", true),
    ] {
        let ns = median_ns(
            || {
                black_box(sparse_action(&cs, &indices, start_seed, edwards));
            },
            5,
        );
        println!("CSIDH-512,sparse_action,3_5_7,{arm},{ns},true");
    }
    let target = sparse_action(&cs, &[0, 1], 50_000, false);
    assert_eq!(target, sparse_action(&cs, &[0, 1], 50_000, true));
    let mut mont_rng = Rng::new(60_000);
    let mut edwards_rng = Rng::new(60_000);
    assert_eq!(
        frontier(&cs, f.zero(), &mut mont_rng, false),
        frontier(&cs, f.zero(), &mut edwards_rng, true)
    );
    assert_eq!(
        frontier(&cs, target, &mut mont_rng, false),
        frontier(&cs, target, &mut edwards_rng, true)
    );
    for (arm, edwards) in [
        ("montgomery_x_only", false),
        ("edwards_with_conversion", true),
    ] {
        let found = bounded_path_search(&cs, target, 60_000, edwards);
        assert!(found.is_some(), "bounded path search missed the target");
        let ns = median_ns(
            || {
                black_box(bounded_path_search(&cs, target, 60_000, edwards));
            },
            3,
        );
        println!("CSIDH-512,bounded_path_search,3_5,{arm},{ns},true");
    }
}
