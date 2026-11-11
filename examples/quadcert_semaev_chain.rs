//! Development check: one planted three-point Semaev S3-chain decomposition
//! over a small prime field, solved by `f4_fp::solve` in three encodings.
//!
//! ```text
//! cargo run --release --example quadcert_semaev_chain -- [p] [k] [seed] [budget_secs]
//! ```
//!
//! The factor base is `{ t² + c : h(t) = 0 }` for a split `h` of degree `k`,
//! and a target `R = P₁ + P₂ + P₃` is planted from three of its points. The
//! system is `S₃(x₁, x₂, u) = 0`, `S₃(u, x₃, x(R)) = 0`, `h(tᵢ) = 0`,
//! projected onto `(t₁, t₂, t₃)`:
//!
//! - `direct`: `xᵢ = tᵢ² + c` substituted by hand (degree 8 in the chain,
//!   `k` in the factor-base equations), 4 variables;
//! - `quadcert-noelim`: the gate program quadratised by
//!   [`crypto_lib::cryptanalysis::quadcert`], factor-base equations by the
//!   square split, no eliminations;
//! - `quadcert`: the same with certified eliminations.
//!
//! Both quadcert systems are checked by `quadcert::check` before they are
//! solved, and the planted assignment is checked against all three. The
//! solution sets projected onto `(t₁, t₂, t₃)` must agree; the run fails
//! loudly if they do not.
//!
//! This is a development check, not a measurement: one host, one seed, no
//! protocol, no matched rho. Any cost comparison between encodings needs a
//! frozen experiment (`controlled-experiment`, `ecbench-measure`).

use std::collections::BTreeSet;
use std::time::Duration;

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Verdict};
use crypto_lib::cryptanalysis::quadcert::{
    self as qc, add, check, compile, constant, mul, scale, sub, var, Builder, CompileOptions, Poly,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

fn sqrt_exists(v: u64, p: u64) -> bool {
    v == 0 || powm(v, (p - 1) / 2, p) == 1
}

fn sqrt_mod(v: u64, p: u64) -> u64 {
    (0..p).find(|&y| mulm(y, y, p) == v).expect("square")
}

type Pt = Option<(u64, u64)>;

fn ec_add(a: Pt, b: Pt, ca: u64, p: u64) -> Pt {
    let (Some((x1, y1)), Some((x2, y2))) = (a, b) else {
        return a.or(b);
    };
    let lam = if x1 == x2 {
        if (y1 + y2) % p == 0 {
            return None;
        }
        mulm((3 * mulm(x1, x1, p) + ca) % p, qc::invm(2 * y1 % p, p), p)
    } else {
        mulm((y2 + p - y1) % p, qc::invm((x2 + p - x1) % p, p), p)
    };
    let x3 = (mulm(lam, lam, p) + 2 * p - x1 - x2) % p;
    let y3 = (mulm(lam, (x1 + p - x3) % p, p) + p - y1) % p;
    Some((x3, y3))
}

/// Prime-field `S₃(x₁, x₂, X)` for `y² = x³ + a·x + b`
/// (as in `semaev_s3_cost`).
fn s3(x1: &Poly, x2: &Poly, xx: &Poly, a: u64, b: u64, p: u64) -> Poly {
    let d = sub(x1, x2, p);
    let s = add(x1, x2, p);
    let pr = mul(x1, x2, p);
    let t0 = mul(&mul(&d, &d, p), &mul(xx, xx, p), p);
    let inner = add(
        &mul(&s, &add(&pr, &constant(a, p), p), p),
        &constant(2 * b % p, p),
        p,
    );
    let t1 = scale(&mul(&inner, xx, p), p - 2, p);
    let pa = sub(&pr, &constant(a, p), p);
    let t2 = sub(&mul(&pa, &pa, p), &scale(&s, 4 * b % p, p), p);
    add(&add(&t0, &t1, p), &t2, p)
}

fn horner(coeffs: &[u64], w: &Poly, p: u64) -> Poly {
    coeffs.iter().rev().fold(Poly::new(), |acc, &c| {
        add(&mul(&acc, w, p), &constant(c, p), p)
    })
}

struct Outcome {
    name: &'static str,
    n_vars: usize,
    n_polys: usize,
    max_deg: usize,
    monomials: usize,
    solving_degree: u32,
    max_cols: usize,
    ms: f64,
    timed_out: bool,
    projected: Option<BTreeSet<Vec<u64>>>,
}

fn run(
    name: &'static str,
    polys: &[Poly],
    n: usize,
    proj: &[usize],
    p: u64,
    budget: Duration,
) -> Outcome {
    let input: Vec<f4_fp::Poly> = polys
        .iter()
        .map(|r| f4_fp::normalise(&qc::to_dense_terms(r, n), p, Ordering::Grevlex))
        .collect();
    let opts = F4Options::new(Ordering::Grevlex, 24).with_budget(budget);
    let rep = f4_fp::solve(&input, n, p, &opts);
    let projected = match &rep.verdict {
        Verdict::Solutions(s) => Some(
            s.iter()
                .map(|x| proj.iter().map(|&i| x[i]).collect())
                .collect(),
        ),
        Verdict::Inconsistent => Some(BTreeSet::new()),
        Verdict::Undetermined => None,
    };
    Outcome {
        name,
        n_vars: n,
        n_polys: polys.len(),
        max_deg: polys.iter().map(qc::degree).max().unwrap_or(0),
        monomials: polys.iter().map(|r| r.len()).sum(),
        solving_degree: rep.solving_degree,
        max_cols: rep.max_cols,
        ms: rep.ms,
        timed_out: rep.timed_out,
        projected,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p: u64 = args.get(1).map_or(1009, |s| s.parse().expect("p"));
    let k: usize = args.get(2).map_or(4, |s| s.parse().expect("k"));
    let seed: u64 = args.get(3).map_or(1, |s| s.parse().expect("seed"));
    let budget = Duration::from_secs(args.get(4).map_or(60, |s| s.parse().expect("budget")));
    let mut rng = StdRng::seed_from_u64(seed);

    // Curve, shift c, and k factor-base roots t whose x = t² + c is on it.
    let (ca, cb) = (rng.gen_range(1..p), rng.gen_range(1..p));
    let c = rng.gen_range(1..p);
    let on_curve = |x: u64| (mulm(mulm(x, x, p), x, p) + mulm(ca, x, p) + cb) % p;
    let mut roots: Vec<u64> = vec![];
    while roots.len() < k {
        let t = rng.gen_range(1..p);
        let x = (mulm(t, t, p) + c) % p;
        let squares: BTreeSet<u64> = roots.iter().map(|r| mulm(*r, *r, p)).collect();
        if sqrt_exists(on_curve(x), p) && !squares.contains(&mulm(t, t, p)) {
            roots.push(t);
        }
    }
    // h(t) = ∏ (t − r), coefficients low → high.
    let mut h = vec![1u64];
    for &r in &roots {
        let mut next = vec![0u64; h.len() + 1];
        for (i, &hi) in h.iter().enumerate() {
            next[i + 1] = (next[i + 1] + hi) % p;
            next[i] = (next[i] + mulm(hi, p - r, p)) % p;
        }
        h = next;
    }
    // Plant R = P1 + P2 + P3 from three factor-base points.
    let (planted_t, planted_u, xr) = loop {
        let ts: Vec<u64> = (0..3).map(|_| roots[rng.gen_range(0..k)]).collect();
        let pts: Vec<Pt> = ts
            .iter()
            .map(|&t| {
                let x = (mulm(t, t, p) + c) % p;
                Some((x, sqrt_mod(on_curve(x), p)))
            })
            .collect();
        let s12 = ec_add(pts[0], pts[1], ca, p);
        let r = ec_add(s12, pts[2], ca, p);
        if let (Some((u, _)), Some((xr, _))) = (s12, r) {
            break (ts, u, xr);
        }
    };

    // Encoding A: direct substitution. Variables t1, t2, t3, u.
    let tx = |i: u32| add(&mul(&var(i), &var(i), p), &constant(c, p), p);
    let direct = {
        let mut sys = vec![
            s3(&tx(0), &tx(1), &var(3), ca, cb, p),
            s3(&var(3), &tx(2), &constant(xr, p), ca, cb, p),
        ];
        for i in 0..3 {
            sys.push(horner(&h, &var(i), p));
        }
        sys
    };
    let mut planted_direct = planted_t.clone();
    planted_direct.push(planted_u);
    assert!(
        direct.iter().all(|r| qc::eval(r, &planted_direct, p) == 0),
        "planted solution fails the direct system"
    );

    // Encodings B/C: gate program.
    let mut b = Builder::new(p);
    let t: Vec<u32> = (0..3).map(|_| b.input(true)).collect();
    let u = b.input(false);
    let x: Vec<u32> = t.iter().map(|&ti| b.assign(&tx(ti))).collect();
    b.constrain(&s3(&var(x[0]), &var(x[1]), &var(u), ca, cb, p));
    b.constrain(&s3(&var(u), &var(x[2]), &constant(xr, p), ca, cb, p));
    for &ti in &t {
        let e = b.univariate_square_split(&h, ti);
        b.constrain(&e);
    }

    let mut outcomes = vec![run("direct", &direct, 4, &[0, 1, 2], p, budget)];
    for (name, eliminate) in [("quadcert-noelim", false), ("quadcert", true)] {
        let cert = compile(
            &b,
            CompileOptions {
                eliminate,
                ..CompileOptions::default()
            },
        );
        let report = check(&cert).unwrap_or_else(|e| panic!("{name}: certificate rejected: {e}"));
        let (polys, order) = cert.compacted();
        assert_eq!(&order[..3], &t[..]);
        eprintln!("{name}: certificate ok {report:?}");
        outcomes.push(run(name, &polys, order.len(), &[0, 1, 2], p, budget));
    }

    let mut reference: Option<&BTreeSet<Vec<u64>>> = None;
    let mut agree = true;
    for o in &outcomes {
        if let Some(s) = &o.projected {
            if !s.contains(&planted_t) {
                agree = false;
                eprintln!(
                    "{}: planted (t1, t2, t3) missing from the solutions",
                    o.name
                );
            }
            match reference {
                None => reference = Some(s),
                Some(r) if r != s => {
                    agree = false;
                    eprintln!("{}: projected solution set differs", o.name);
                }
                _ => {}
            }
        }
        println!(
            "{{\"encoding\":\"{}\",\"p\":{p},\"k\":{k},\"seed\":{seed},\"vars\":{},\"polys\":{},\"max_deg\":{},\"monomials\":{},\"solving_degree\":{},\"max_cols\":{},\"ms\":{:.1},\"timed_out\":{},\"projected_solutions\":{}}}",
            o.name,
            o.n_vars,
            o.n_polys,
            o.max_deg,
            o.monomials,
            o.solving_degree,
            o.max_cols,
            o.ms,
            o.timed_out,
            o.projected.as_ref().map_or("null".to_string(), |s| s.len().to_string()),
        );
    }
    if !agree {
        eprintln!("FAIL: encodings disagree on the projected solution set");
        std::process::exit(1);
    }
}
