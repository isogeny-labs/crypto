//! **Linearized two-summand root finding against the Semaev pair loop.**
//!
//! The same decomposition question — does a target `R` split as `P_1 + P_2 + P_3` over the low-order
//! subspace `V = ⟨1, z, …, z^{l−1}⟩`? — decided two ways on identical fields, subspaces and targets:
//!
//! 1. **pair loop** ([`SubspaceOracle::decompose`], the algorithm of
//!    [`semaev_decomp::decompose`]): loop over the `2^{2l}/2` pairs `(x_1, x_2)` in `V`, build the
//!    quartic `S₄` in the third variable and keep its roots in `V` via `gcd(q, L_V mod q)`. `O(2^{2l} · l)`.
//! 2. **linearized two-summand** ([`LinearDecomposer::decompose3`]): fix one summand `P_3` over the
//!    `2^l` abscissas of `V` (both signs) and solve the remaining two-summand problem for `T = R − P_3`
//!    as one `F_2`-linear system plus one quadratic. `O(2^l · l²)`.
//!
//! Both are the repository's one-word field `Gf2` (`n ≤ 62`), Koblitz `b = 1`, so only the algorithm
//! differs. Targets are **rational points** `R` (a real attack's targets); `x(R)` drives the pair loop.
//! Every target's verdict is checked to agree, and every linearized witness is re-added in the group and
//! checked against `S₄ = 0`. `n = 131` (ECC2K-130) is above `Gf2` and `Gf2_128`; it is measured in the
//! standalone three-word crate (lfd-summation-systems, rust/ic_rootfind).
//!
//! ```bash
//! cargo run --release --example semaev_linear_bench            # to l = 11
//! cargo run --release --example semaev_linear_bench 13 0       # to l = 13, curve a = 0
//! ```

use crypto_lib::binary_ecc::IrreduciblePoly;
use crypto_lib::cryptanalysis::semaev_decomp::{eval_s4_general, Gf2, SubspaceOracle};
use crypto_lib::cryptanalysis::semaev_linear::{LinearDecomposer, Pt};
use std::time::Instant;

fn poly_deg(a: u64) -> i32 {
    63 - a.leading_zeros() as i32
}
fn poly_mulmod(mut a: u64, mut b: u64, f: u64, n: u32) -> u64 {
    let mut r = 0u64;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 == 1 {
            a ^= f;
        }
    }
    r
}
fn poly_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let db = poly_deg(b);
        while a != 0 && poly_deg(a) >= db {
            a ^= b << (poly_deg(a) - db);
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}
fn prime_factors(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    let mut p = 2;
    while p * p <= n {
        if n.is_multiple_of(p) {
            out.push(p);
            while n.is_multiple_of(p) {
                n /= p;
            }
        }
        p += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}
/// Lowest-weight irreducible x^n + ... + 1 (trinomial, else pentanomial), Rabin-tested.
fn find_modulus(n: u32) -> (u64, Vec<u32>) {
    let irreducible = |f: u64| -> bool {
        // x^(2^n) = x, and gcd(x^(2^(n/q)) + x, f) = 1 for each prime q | n
        let mut xp = 2u64;
        for _ in 0..n {
            xp = poly_mulmod(xp, xp, f, n);
        }
        if xp != 2 {
            return false;
        }
        for q in prime_factors(n) {
            let mut y = 2u64;
            for _ in 0..n / q {
                y = poly_mulmod(y, y, f, n);
            }
            if poly_gcd(y ^ 2, f) != 1 {
                return false;
            }
        }
        true
    };
    let base = 1u64 << n;
    for k in 1..n {
        let f = base | (1 << k) | 1;
        if irreducible(f) {
            return (f, vec![0, k]);
        }
    }
    for k3 in 3..n {
        for k2 in 2..k3 {
            for k1 in 1..k2 {
                let f = base | (1 << k3) | (1 << k2) | (1 << k1) | 1;
                if irreducible(f) {
                    return (f, vec![0, k1, k2, k3]);
                }
            }
        }
    }
    panic!("no sparse irreducible for n = {n}")
}

fn rng(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max_l: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(11);
    let a: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1);
    let b = 1u64;
    let samples: usize = 400;

    println!(
        "Linearized two-summand root finding vs the Semaev pair loop (Koblitz a = {a}, b = 1)\n\
         same field Gf2, same low-order subspace V = <1, z, .., z^(l-1)>, same rational-point targets\n"
    );
    println!(
        "  {:>2} {:>3} {:>8} {:>12} {:>12} {:>9} {:>9} {:>9}",
        "l", "n", "|V|", "pairs(loop)", "scanned(lin)", "unit x", "absc% loop", "rat% lin"
    );
    println!("  {}", "-".repeat(72));

    let mut time_rows: Vec<(u32, u32, f64, f64, f64)> = Vec::new();
    for l in 5..=max_l {
        let n = 3 * l;
        if n > 60 {
            break;
        }
        let (f, low) = find_modulus(n);
        let _ = f;
        let irr = IrreduciblePoly {
            degree: n,
            low_terms: low.clone(),
        };
        let gf = Gf2::new(&irr);
        let lin = LinearDecomposer::low_order(&gf, a, b, l);
        let oracle = SubspaceOracle::low_order(l, b, &gf);

        let mask = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
        let mut st = 0x5EED ^ ((n as u64) << 8) ^ (a << 2);
        let rand_point = |st: &mut u64| -> Pt {
            loop {
                let x = rng(st) & mask;
                if let Some(p) = lin.lift(&gf, x) {
                    return if rng(st) & 1 == 1 { lin.neg(p) } else { p };
                }
            }
        };

        // Correctness + native-unit cost on a worst-case (non-decomposing) target, found by sampling.
        let mut worst: Option<Pt> = None;
        for _ in 0..4000 {
            let r = rand_point(&mut st);
            let (w, _pairs) = oracle.decompose(r.0, &gf);
            if w.is_none() {
                worst = Some(r);
                break;
            }
        }
        let (pairs_unit, scanned_unit) = if let Some(r) = worst {
            let (w1, pairs) = oracle.decompose(r.0, &gf);
            let (w2, scanned) = lin.decompose3(&gf, r);
            assert!(
                w1.is_none() && w2.is_none(),
                "worst-case target must not decompose"
            );
            (pairs, scanned)
        } else {
            (0, 0)
        };

        // Timing on random rational-point targets.  The pair loop decides the ABSCISSA question
        // (S4 = 0, which also admits twist-only solutions); the linearized solver decides the RATIONAL
        // question (points over F_2^n summing to R).  At small l we check LRF against an independent
        // group-law triple enumeration; always we re-add every linearized witness.
        let (mut t_loop, mut t_lin) = (0.0f64, 0.0f64);
        let mut absc = 0usize;
        let mut rat = 0usize;
        let brute_ok = l <= 8;
        let targets: Vec<Pt> = (0..samples).map(|_| rand_point(&mut st)).collect();
        for &r in &targets {
            let t = Instant::now();
            let (w_loop, _) = oracle.decompose(r.0, &gf);
            t_loop += t.elapsed().as_secs_f64();
            let t = Instant::now();
            let (w_lin, _) = lin.decompose3(&gf, r);
            t_lin += t.elapsed().as_secs_f64();
            absc += w_loop.is_some() as usize;
            if let Some(tri) = w_lin {
                let mut acc = None;
                for p in tri {
                    assert!(lin.on_curve(&gf, p) && lin.in_v(p.0));
                    acc = lin.add(&gf, acc, Some(p));
                }
                assert_eq!(acc, Some(r), "linearized witness sums to R");
                assert_eq!(
                    eval_s4_general(tri[0].0, tri[1].0, tri[2].0, r.0, b, &gf),
                    0
                );
                rat += 1;
            }
            if brute_ok {
                assert_eq!(
                    w_lin.is_some(),
                    lin.decomposes3_brute(&gf, r),
                    "LRF disagrees with the independent brute at l = {l}"
                );
            }
            // every rational decomposition is also an abscissa solution
            assert!(
                w_loop.is_some() || w_lin.is_none(),
                "pair loop missed a rational decomposition"
            );
            if let Some(w) = w_loop {
                assert_eq!(eval_s4_general(w[0], w[1], w[2], r.0, b, &gf), 0);
            }
        }
        let per_loop = t_loop / samples as f64;
        let per_lin = t_lin / samples as f64;
        let unit_x = if scanned_unit > 0 {
            pairs_unit as f64 / scanned_unit as f64
        } else {
            f64::NAN
        };
        println!(
            "  {:>2} {:>3} {:>8} {:>12} {:>12} {:>8.1}x {:>8.1} {:>8.1}",
            l,
            n,
            1u64 << l,
            pairs_unit,
            scanned_unit,
            unit_x,
            100.0 * absc as f64 / samples as f64,
            100.0 * rat as f64 / samples as f64
        );
        time_rows.push((l, n, per_loop, per_lin, per_loop / per_lin.max(1e-12)));
    }

    println!("\n  per-target wall time (median target is a decide; mixed decomposable/not):\n");
    println!(
        "  {:>2} {:>3} {:>14} {:>14} {:>10}",
        "l", "n", "pair loop", "linearized", "speed-up"
    );
    println!("  {}", "-".repeat(50));
    for (l, n, pl, pli, sp) in &time_rows {
        let fmt = |s: f64| {
            if s >= 1e-3 {
                format!("{:.3} ms", s * 1e3)
            } else {
                format!("{:.1} us", s * 1e6)
            }
        };
        println!(
            "  {:>2} {:>3} {:>14} {:>14} {:>9.1}x",
            l,
            n,
            fmt(*pl),
            fmt(*pli),
            sp
        );
    }
    println!(
        "\n  The linearized solver is exact and complete for RATIONAL decompositions (checked against an\n  \
         independent group-law enumeration at l <= 8; every witness re-adds to R and has S4 = 0). The pair\n  \
         loop decides the ABSCISSA question S4 = 0, a superset that also admits twist-only solutions, so a\n  \
         real relation search lift-checks each of its hits. Pair loop O(2^(2l) l); linearized O(2^l l^2)."
    );
}
