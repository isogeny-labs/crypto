//! Release-build check of `cryptanalysis::binary_factored_wide` and
//! `Gf2W::sqr`, through the public API only.
//!
//! The same checks are `#[test]`s in the library. This example exists because
//! the crate's lib test binary does not compile within a 15 GiB container
//! (rustc is OOM-killed); the release library does. Exit status 0 = all pass.
//!
//! `cargo run --release --example factored_wide_check`

use crypto_lib::binary_ecc::f2m::IrreduciblePoly;
use crypto_lib::cryptanalysis::binary_factored_wide::{
    ArtinSchreierAny, FactorField, FactoredWide, PairCase, RowScratch,
};
use crypto_lib::cryptanalysis::certicom131_symlp::gf2w::{self as gw, Gf2W, W3};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2_128;
use rand::{rngs::StdRng, Rng, SeedableRng};

/// Brute force over V of the quartic, sorted.
fn truth<F: FactorField>(f: &F, l: u32, x1: F::E, x2: F::E, xr: F::E) -> Vec<F::E>
where
    F::E: Ord + Eq + Copy + std::fmt::Debug,
{
    let q = FactoredWide::<F::E>::quartic(f, x1, x2, xr);
    let mut r: Vec<F::E> = (0..(1u64 << l))
        .map(|v| f.elem(v))
        .filter(|&x| {
            let mut acc = q[4];
            for k in (0..4).rev() {
                acc = f.add(f.mul(acc, x), q[k]);
            }
            f.is_zero(acc)
        })
        .collect();
    r.sort();
    r
}

/// A target with a planted decomposition through x₃ ∈ V: u = root of
/// S₃(x₁, x₂, ·), x_R = root of S₃(u, x₃, ·), when both split.
fn plant<F: FactorField>(
    f: &F,
    fw: &FactoredWide<F::E>,
    x1: F::E,
    x2: F::E,
    x3: F::E,
) -> Option<F::E>
where
    F::E: Ord + Eq + Copy + std::fmt::Debug,
{
    let [u, _] = fw.s3_roots(f, x1, x2)?;
    let [xr, _] = fw.s3_roots(f, u, x3)?;
    Some(xr)
}

fn check_field<F: FactorField>(f: &F, l: u32, seed: u64, trials: usize)
where
    F::E: Ord + Eq + Copy + std::fmt::Debug,
{
    let fw = FactoredWide::new(f, l);
    let mut rng = StdRng::seed_from_u64(seed);
    let span = 1u64 << l;
    let n = f.degree();
    let mut planted_hits = 0;
    let mut cases = [0usize; 3];
    for t in 0..trials {
        let x1 = f.elem(rng.gen_range(0..span));
        let x2 = f.elem(rng.gen_range(0..span));
        let xr = if t % 2 == 0 {
            let x3 = f.elem(rng.gen_range(0..span));
            match plant(f, &fw, x1, x2, x3) {
                Some(xr) => {
                    let q = FactoredWide::<F::E>::quartic(f, x1, x2, xr);
                    let mut acc = q[4];
                    for k in (0..4).rev() {
                        acc = f.add(f.mul(acc, x3), q[k]);
                    }
                    assert!(
                        f.is_zero(acc),
                        "planted x₃ is a root of the quartic (n={n})"
                    );
                    planted_hits += 1;
                    xr
                }
                None => f.elem(rng.gen()),
            }
        } else {
            // a random element of the field
            let mut x = f.zero();
            for i in 0..n {
                if rng.gen::<bool>() {
                    x = f.add(x, f.mono(i));
                }
            }
            x
        };
        let inv_xr = f.inv(xr);
        let mut got = Vec::new();
        let case = fw.pair_roots(f, x1, x2, xr, inv_xr, &mut got);
        cases[case as usize] += 1;
        got.sort();
        assert_eq!(
            got,
            truth(f, l, x1, x2, xr),
            "pair_roots n={n} l={l} trial={t} case={case:?}"
        );
    }
    assert!(
        planted_hits > trials / 20,
        "planting works (n={n}): {planted_hits}"
    );
    assert!(
        cases[PairCase::Factored as usize] > 0 && cases[PairCase::Rejected as usize] > 0,
        "{cases:?}"
    );
}

fn check_rows<F: FactorField>(f: &F, l: u32, seed: u64, targets: usize)
where
    F::E: Ord + Eq + Copy + std::fmt::Debug,
{
    let fw = FactoredWide::new(f, l);
    let mut rng = StdRng::seed_from_u64(seed);
    let span = 1u64 << l;
    let v: Vec<F::E> = (0..span).map(|i| f.elem(i)).collect();
    let mut sc = RowScratch::default();
    for _ in 0..targets {
        let (a, b, c) = (
            rng.gen_range(0..span),
            rng.gen_range(0..span),
            rng.gen_range(0..span),
        );
        let xr = plant(f, &fw, v[a as usize], v[b as usize], v[c as usize])
            .unwrap_or_else(|| f.elem(rng.gen()));
        let inv_xr = f.inv(xr);
        for x1 in 0..span {
            let x1e = v[x1 as usize];
            let mut got: Vec<(F::E, F::E)> = Vec::new();
            fw.row(f, x1e, &v[x1 as usize..], xr, inv_xr, &mut sc, |x2, x3| {
                got.push((x2, x3))
            });
            got.sort();
            let mut want: Vec<(F::E, F::E)> = Vec::new();
            for &x2 in &v[x1 as usize..] {
                for x3 in truth(f, l, x1e, x2, xr) {
                    want.push((x2, x3));
                }
            }
            want.sort();
            assert_eq!(got, want, "row n={} x1={x1}", f.degree());
        }
    }
}

fn check_sqr(n: u32) {
    let irr = gw::low_weight_irreducible(n);
    let mask = {
        let mut m: W3 = [0; 3];
        for i in 0..n {
            m[(i / 64) as usize] |= 1 << (i % 64);
        }
        m
    };
    let fields = [
        Gf2W::new(&irr),
        Gf2W::with_kernel(&irr, false),
        Gf2W::with_kernel(&irr, true),
    ];
    let mut rng = StdRng::seed_from_u64(0x5a5a ^ n as u64);
    let mut xs: Vec<W3> = vec![[0; 3], [1, 0, 0], mask];
    for _ in 0..2000 {
        let a: W3 = [rng.gen(), rng.gen(), rng.gen()];
        xs.push([a[0] & mask[0], a[1] & mask[1], a[2] & mask[2]]);
    }
    for (k, f) in fields.iter().enumerate() {
        for a in &xs {
            assert_eq!(f.sqr(a), f.mul(a, a), "sqr n={n} kernel#{k} a={a:?}");
            assert_eq!(
                f.sqr(a),
                fields[0].mul(a, a),
                "sqr n={n} kernel#{k} vs default"
            );
        }
    }
}

fn check_as<F: FactorField>(f: &F, rng: &mut StdRng)
where
    F::E: Ord + Eq + Copy + std::fmt::Debug,
{
    let a = ArtinSchreierAny::new(f);
    let n = f.degree();
    let (mut solved, mut refused) = (0, 0);
    for _ in 0..500 {
        let mut c = f.zero();
        for i in 0..n {
            if rng.gen::<bool>() {
                c = f.add(c, f.mono(i));
            }
        }
        match a.solve(f, c) {
            Some(z) => {
                assert_eq!(f.add(f.sqr(z), z), c, "AS n={n}");
                solved += 1;
            }
            None => refused += 1,
        }
    }
    assert!(
        solved > 100 && refused > 100,
        "AS n={n}: {solved}/{refused}"
    );
}

/// Latency chain: x <- op(x), `reps` times; ns per op, median of 5 runs.
fn chain_ns(reps: u64, mut step: impl FnMut()) -> f64 {
    let mut v: Vec<f64> = (0..5)
        .map(|_| {
            let t = std::time::Instant::now();
            for _ in 0..reps {
                step();
            }
            t.elapsed().as_nanos() as f64 / reps as f64
        })
        .collect();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[2]
}

fn time_sqr() {
    let reps = 4_000_000;
    for n in [131u32, 163] {
        let irr = gw::low_weight_irreducible(n);
        for (name, f) in [
            ("default", Gf2W::new(&irr)),
            ("no-clmul", Gf2W::with_kernel(&irr, false)),
        ] {
            let mut x: W3 = [0x1234_5678_9abc_def1, 0x0fed_cba9_8765_4321, 0x3];
            let mul = chain_ns(reps, || x = std::hint::black_box(f.mul(&x, &x)));
            let mut y: W3 = [0x1234_5678_9abc_def1, 0x0fed_cba9_8765_4321, 0x3];
            let sqr = chain_ns(reps, || y = std::hint::black_box(f.sqr(&y)));
            assert_eq!(x, y);
            println!(
                "time n={n} kernel={name}: mul(a,a) {mul:.2} ns, sqr {sqr:.2} ns, ratio {:.3}",
                sqr / mul
            );
        }
    }
}

fn main() {
    if std::env::args().any(|a| a == "--time") {
        time_sqr();
        return;
    }
    for n in [61u32, 64, 113, 127, 128, 131, 163, 191] {
        check_sqr(n);
    }
    println!("ok  Gf2W::sqr == mul(a,a) on every kernel, n in 61..191");

    let mut rng = StdRng::seed_from_u64(0xA5);
    for n in [113u32, 126, 127] {
        check_as(&Gf2_128::new(&gw::low_weight_irreducible(n)), &mut rng);
    }
    for n in [130u32, 131, 163] {
        check_as(&Gf2W::new(&gw::low_weight_irreducible(n)), &mut rng);
    }
    println!("ok  Artin-Schreier solver exact, odd and even n");

    for irr in [IrreduciblePoly::deg_113(), IrreduciblePoly::deg_127()] {
        let f = Gf2_128::new(&irr);
        for l in [4u32, 6, 8] {
            check_field(&f, l, 0x113 ^ irr.degree as u64 ^ (l as u64) << 8, 400);
        }
    }
    println!("ok  pair_roots == brute force, Gf2_128 n=113,127, l=4,6,8");
    for n in [131u32, 130, 163] {
        let f = Gf2W::new(&gw::low_weight_irreducible(n));
        for l in [4u32, 6, 8] {
            check_field(&f, l, 0x131 ^ n as u64 ^ (l as u64) << 8, 400);
        }
    }
    println!("ok  pair_roots == brute force, Gf2W n=131,130,163, l=4,6,8");
    let f = Gf2_128::new(&gw::low_weight_irreducible(126));
    for l in [4u32, 6] {
        check_field(&f, l, 0x126 ^ (l as u64) << 8, 400);
    }
    println!("ok  pair_roots == brute force, Gf2_128 n=126 (even), l=4,6");
    check_rows(&Gf2_128::new(&IrreduciblePoly::deg_127()), 4, 1, 6);
    check_rows(&Gf2W::new(&gw::low_weight_irreducible(131)), 4, 2, 6);
    check_rows(&Gf2W::new(&gw::low_weight_irreducible(130)), 4, 3, 6);
    check_rows(&Gf2W::new(&gw::low_weight_irreducible(163)), 6, 4, 3);
    println!("ok  batched row == brute force, n=127,131,130,163");
    println!("ALL PASS");
}
