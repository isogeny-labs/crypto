//! F4 on Petit–Kosters–Messeng Kummer towers of **any** degree mix
//! (`research/pkm_odd_tower_f4_20261114/PROTOCOL.md`, EXP-PKM-ODD-001).
//!
//! The tower engine `f4_fp_tower` stores tower monomials as bit masks, so
//! it can only express degree-2 rules (`research/curve25519_leads_wave4_
//! 20261104/l11_odd_kummer_design`). This example writes the same systems
//! as plain polynomials and runs the generic degree-bounded F4 of
//! `f4_fp` (grevlex), which accepts any degree:
//!
//! * unknowns `x_{i,1..n'}` per point `i = 1..m`, plus `u` at `m = 3`;
//! * tower: `x_{i,j}^{q_j} − x_{i,j+1}` (`j < n'`), `x_{i,n'}^{q_{n'}} − c`,
//!   so `x_{i,1} ∈ V = g·μ_S`, `S = ∏ q_j`, from `pkm_smooth_tower::kummer_tower`;
//! * summation: `S_3(x_{1,1}, x_{2,1}, X)` at `m = 2`;
//!   `S_3(x_{1,1}, x_{2,1}, u)` and `S_3(u, x_{3,1}, X)` at `m = 3`;
//! * control `null`: every summation polynomial replaced by one with the
//!   same monomial support and random nonzero coefficients.
//!
//! Targets: `refute` draws random curve points `R` until no tuple of `V`
//! satisfies the summation equations (checked exhaustively), so the
//! system has no solution and F4 must reach `1`; `planted` takes
//! `R = P_1 + … + P_m` with `x(P_i) ∈ V`, so the system has a solution and
//! F4 must not reach `1`. Every row records the exhaustive verdict next to
//! F4's (`agree`), and a planted row also evaluates every equation at the
//! planted point (`planted_ok`).
//!
//! ```text
//! pkm_odd_tower_f4 --p P --degrees 2,3,2 [--m 2|3] [--control tower|null]
//!     [--kind refute|planted] [--draws K] [--seed S] [--budget SECS]
//!     [--dmax D] [--cell NAME] [--out FILE.jsonl]
//! ```
//! One JSON row per system on stdout (and appended to `--out`). Counted
//! metrics (`field_ops`, degrees, widths) are the study's; `ms` is recorded
//! and not used.

use crypto_lib::cryptanalysis::f4_fp::{self, F4Options, Ordering, Poly};
use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    addm, kummer_tower, mulm, powm, s3, sqrtm, subm, Curve, Rng,
};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, Instant};

/// `S_3(x_a, x_b, x_c)` as terms over `n` variables, where each slot is a
/// variable index or a constant.
#[derive(Clone, Copy)]
enum Slot {
    Var(usize),
    Const(u64),
}

/// Monomials of `S_3` in `(x1, x2, x3)`: exponent triple and coefficient.
fn s3_terms(e: &Curve) -> Vec<([u32; 3], u64)> {
    let p = e.p;
    let (a, b) = (e.a, e.b);
    let m2 = p - 2;
    vec![
        ([2, 0, 2], 1),
        ([0, 2, 2], 1),
        ([1, 1, 2], m2),
        ([2, 1, 1], m2),
        ([1, 2, 1], m2),
        ([1, 0, 1], mulm(m2, a, p)),
        ([0, 1, 1], mulm(m2, a, p)),
        ([0, 0, 1], mulm(p - 4, b, p)),
        ([2, 2, 0], 1),
        ([1, 1, 0], mulm(m2, a, p)),
        ([0, 0, 0], mulm(a, a, p)),
        ([1, 0, 0], mulm(p - 4, b, p)),
        ([0, 1, 0], mulm(p - 4, b, p)),
    ]
}

/// A summation polynomial in three slots: monomials `(exps, coeff)`.
#[derive(Clone)]
struct Sum3 {
    terms: Vec<([u32; 3], u64)>,
}

impl Sum3 {
    fn eval(&self, x: [u64; 3], p: u64) -> u64 {
        self.terms.iter().fold(0, |acc, &(ex, c)| {
            let mut t = c;
            for k in 0..3 {
                t = mulm(t, powm(x[k], ex[k] as u64, p), p);
            }
            addm(acc, t, p)
        })
    }
    /// Substitute the slots and emit terms over `n` variables.
    fn emit(&self, slots: [Slot; 3], n: usize, p: u64) -> Vec<(Vec<u32>, u64)> {
        let mut acc: HashMap<Vec<u32>, u64> = HashMap::new();
        for &(ex, c) in &self.terms {
            let mut mono = vec![0u32; n];
            let mut coef = c;
            for k in 0..3 {
                match slots[k] {
                    Slot::Var(i) => mono[i] += ex[k],
                    Slot::Const(v) => coef = mulm(coef, powm(v, ex[k] as u64, p), p),
                }
            }
            let e = acc.entry(mono).or_insert(0);
            *e = addm(*e, coef, p);
        }
        acc.into_iter().filter(|&(_, c)| c != 0).collect()
    }
    /// Coefficients `[c0, c1, c2]` of the quadratic in slot `k` with the
    /// other two slots fixed (`S_3` has degree 2 in every slot).
    fn quad_in(&self, k: usize, fixed: [u64; 3], p: u64) -> [u64; 3] {
        let at = |v: u64| {
            let mut x = fixed;
            x[k] = v;
            self.eval(x, p)
        };
        let (f0, f1, fm) = (at(0), at(1), at(p - 1));
        let inv2 = (p + 1) / 2;
        let c2 = mulm(subm(addm(f1, fm, p), mulm(2, f0, p), p), inv2, p);
        let c1 = mulm(subm(f1, fm, p), inv2, p);
        [f0, c1, c2]
    }
}

/// Resultant of two quadratics `a2 u² + a1 u + a0` and `b2 u² + b1 u + b0`.
fn res_quad(a: [u64; 3], b: [u64; 3], p: u64) -> u64 {
    let d = |x: u64, y: u64, z: u64, w: u64| subm(mulm(x, y, p), mulm(z, w, p), p);
    let t1 = d(a[2], b[0], a[0], b[2]);
    let t2 = d(a[2], b[1], a[1], b[2]);
    let t3 = d(a[1], b[0], a[0], b[1]);
    subm(mulm(t1, t1, p), mulm(t2, t3, p), p)
}

/// Does some tuple of `V` satisfy the summation equations (over the
/// algebraic closure for the helper `u`)? Exhaustive.
fn decomposable(sums: &[Sum3], v: &[u64], xr: u64, m: usize, p: u64) -> bool {
    if m == 2 {
        for &a in v {
            for &b in v {
                if sums[0].eval([a, b, xr], p) == 0 {
                    return true;
                }
            }
        }
        return false;
    }
    // m = 3: S(x1, x2, u) and S'(u, x3, X) share a root u.
    let second: Vec<[u64; 3]> = v
        .iter()
        .map(|&c| sums[1].quad_in(0, [0, c, xr], p))
        .collect();
    for &a in v {
        for &b in v {
            let q1 = sums[0].quad_in(2, [a, b, 0], p);
            if q1 == [0, 0, 0] {
                return true;
            }
            for q2 in &second {
                // Resultant zero ⇒ common root (leading coefficients nonzero
                // generically; a degenerate pair is also reported as a hit,
                // which only makes `agree` stricter for refutation rows).
                if res_quad(q1, *q2, p) == 0 {
                    return true;
                }
            }
        }
    }
    false
}

fn arg<'a>(o: &'a HashMap<String, String>, k: &str, d: &'a str) -> &'a str {
    o.get(k).map(|s| s.as_str()).unwrap_or(d)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut o: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    while i + 1 < args.len() {
        o.insert(
            args[i].trim_start_matches("--").to_string(),
            args[i + 1].clone(),
        );
        i += 2;
    }
    let p: u64 = arg(&o, "p", "1399680001").parse().expect("--p");
    let degrees: Vec<u64> = arg(&o, "degrees", "2,3")
        .split(',')
        .map(|s| s.parse().expect("--degrees"))
        .collect();
    let m: usize = arg(&o, "m", "2").parse().expect("--m");
    let control = arg(&o, "control", "tower").to_string();
    let kind = arg(&o, "kind", "refute").to_string();
    let draws: u64 = arg(&o, "draws", "3").parse().expect("--draws");
    let seed: u64 = arg(&o, "seed", "20261114").parse().expect("--seed");
    let budget: u64 = arg(&o, "budget", "900").parse().expect("--budget");
    let dmax: u32 = arg(&o, "dmax", "64").parse().expect("--dmax");
    let cell = arg(&o, "cell", "adhoc").to_string();
    assert!(m == 2 || m == 3, "--m 2 or 3");
    assert!(p < 1 << 32, "f4_fp needs p < 2^32");
    assert!(control == "tower" || control == "null");
    assert!(kind == "refute" || kind == "planted");

    // Same V for both controls and every kind: it depends on (p, degrees, seed) only.
    let tower = kummer_tower(p, &degrees, None, seed).expect("kummer tower");
    tower.verify().expect("tower verify");
    let s = tower.size();
    let n_tower = degrees.len();
    let n_vars = m * n_tower + if m == 3 { 1 } else { 0 };
    let xv = |i: usize, j: usize| i * n_tower + j;
    let uvar = m * n_tower;

    // The curve and the summation polynomials (null: same support, random coefficients).
    let mut crng = Rng(seed ^ 0xe11);
    let e = loop {
        let c = Curve {
            p,
            a: crng.below(p),
            b: crng.below(p),
        };
        if !c.is_singular() {
            break c;
        }
    };
    let base = s3_terms(&e);
    {
        // The expansion must be the library's S_3 (checked at random points).
        let chk = Sum3 {
            terms: base.clone(),
        };
        let mut r = Rng(seed ^ 0xc4ec);
        for _ in 0..64 {
            let x = [r.below(p), r.below(p), r.below(p)];
            assert_eq!(
                chk.eval(x, p),
                s3(&e, x[0], x[1], x[2]),
                "S_3 expansion mismatch"
            );
        }
    }
    let n_sums = m - 1;
    let sums: Vec<Sum3> = (0..n_sums)
        .map(|k| {
            if control == "tower" {
                Sum3 {
                    terms: base.clone(),
                }
            } else {
                let mut r = Rng(seed ^ 0x5011 ^ (k as u64 * 0x9e37));
                Sum3 {
                    terms: base
                        .iter()
                        .map(|&(ex, _)| (ex, 1 + r.below(p - 1)))
                        .collect(),
                }
            }
        })
        .collect();
    let lift: Vec<(u64, u64)> = tower
        .v
        .iter()
        .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
        .collect();

    let mut out = o.get("out").map(|f| {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(f)
            .expect("--out")
    });
    let threads = std::env::var("RAYON_NUM_THREADS").unwrap_or_default();
    let mut trng = Rng(seed ^ 0x7a6e7 ^ (m as u64) << 8);
    for draw in 0..draws {
        // Target.
        let (xr, planted, tries): (u64, Option<Vec<u64>>, u64) = if kind == "planted" {
            assert!(
                control == "tower",
                "planted targets are for the tower control"
            );
            assert!(lift.len() >= m, "fewer than m lifting abscissae in V");
            let pts: Vec<(u64, u64)> = (0..m)
                .map(|_| lift[trng.below(lift.len() as u64) as usize])
                .collect();
            let r = pts.iter().fold(None, |acc, &q| e.add(acc, Some(q)));
            let Some((xr, _)) = r else { continue };
            (xr, Some(pts.iter().map(|q| q.0).collect()), 1)
        } else {
            let mut tries = 0;
            loop {
                tries += 1;
                let r = e.random_point(&mut trng).unwrap();
                if !decomposable(&sums, &tower.v, r.0, m, p) {
                    break (r.0, None, tries);
                }
            }
        };
        let decomp = decomposable(&sums, &tower.v, xr, m, p);

        // The system.
        let mut raw: Vec<Vec<(Vec<u32>, u64)>> = vec![];
        for i in 0..m {
            for j in 0..n_tower {
                let mut a = vec![0u32; n_vars];
                a[xv(i, j)] = degrees[j] as u32;
                if j + 1 < n_tower {
                    let mut b = vec![0u32; n_vars];
                    b[xv(i, j + 1)] = 1;
                    raw.push(vec![(a, 1), (b, p - 1)]);
                } else {
                    raw.push(vec![(a, 1), (vec![0u32; n_vars], p - tower.c % p)]);
                }
            }
        }
        if m == 2 {
            raw.push(sums[0].emit(
                [Slot::Var(xv(0, 0)), Slot::Var(xv(1, 0)), Slot::Const(xr)],
                n_vars,
                p,
            ));
        } else {
            raw.push(sums[0].emit(
                [Slot::Var(xv(0, 0)), Slot::Var(xv(1, 0)), Slot::Var(uvar)],
                n_vars,
                p,
            ));
            raw.push(sums[1].emit(
                [Slot::Var(uvar), Slot::Var(xv(2, 0)), Slot::Const(xr)],
                n_vars,
                p,
            ));
        }
        let input: Vec<Poly> = raw
            .iter()
            .map(|t| f4_fp::normalise(t, p, Ordering::Grevlex))
            .collect();

        // Planted check: every equation vanishes at the planted point (u from the chain).
        let planted_ok = planted.as_ref().map(|xs| {
            let mut pt = vec![0u64; n_vars];
            for i in 0..m {
                let mut y = xs[i];
                for j in 0..n_tower {
                    pt[xv(i, j)] = y;
                    y = powm(y, degrees[j], p);
                }
            }
            if m == 3 {
                let q = sums[0].quad_in(2, [xs[0], xs[1], 0], p);
                // the u that also satisfies the second equation
                let disc = subm(mulm(q[1], q[1], p), mulm(4, mulm(q[2], q[0], p), p), p);
                let sd = sqrtm(disc, p).unwrap_or(0);
                let i2 = mulm(2, q[2], p);
                let inv = powm(i2, p - 2, p);
                let cand = [
                    mulm(subm(sd, q[1], p), inv, p),
                    mulm(subm(p - sd, q[1], p), inv, p),
                ];
                pt[uvar] = *cand
                    .iter()
                    .find(|&&u| sums[1].eval([u, xs[2], xr], p) == 0)
                    .unwrap_or(&cand[0]);
            }
            raw.iter().all(|t| {
                t.iter().fold(0u64, |acc, (ex, c)| {
                    let mut v = *c;
                    for (k, &d) in ex.iter().enumerate() {
                        v = mulm(v, powm(pt[k], d as u64, p), p);
                    }
                    addm(acc, v, p)
                }) == 0
            })
        });

        let mut opts = F4Options::new(Ordering::Grevlex, dmax);
        opts.deadline = Some(Instant::now() + Duration::from_secs(budget));
        let rep = f4_fp::f4(&input, n_vars, p, &opts);
        let agree = !rep.timed_out && rep.inconsistent == !decomp;
        let row = json!({
            "schema": "pkm-odd-tower-f4.row/v1", "cell": cell, "p": p, "m": m,
            "degrees": degrees, "arm": degrees.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(""),
            "S": s, "log2S": (s as f64).log2(), "n_tower_vars": m * n_tower, "n_vars": n_vars,
            "control": control, "kind": kind, "draw": draw, "seed": seed, "target_x": xr,
            "target_tries": tries, "curve": [e.a, e.b], "g": match tower.kind {
                crypto_lib::cryptanalysis::pkm_smooth_tower::TowerKind::Kummer { g } => g, _ => 0 },
            "exhaustive_decomposable": decomp, "inconsistent": rep.inconsistent,
            "agree": agree, "planted_ok": planted_ok, "timed_out": rep.timed_out,
            "censored": rep.timed_out || rep.pairs_above_bound > 0,
            "field_ops": rep.field_ops, "solving_degree": rep.solving_degree,
            "solving_degree_max": rep.solving_degree_max, "degree_reached": rep.degree_reached,
            "max_cols": rep.max_cols, "max_rows": rep.max_rows,
            "max_cols_to_solution": rep.max_cols_to_solution, "steps": rep.steps,
            "steps_to_solution": rep.steps_to_solution, "pairs_above_bound": rep.pairs_above_bound,
            "dmax": dmax, "budget_s": budget, "threads": threads, "ms": rep.ms,
        });
        let line = row.to_string();
        println!("{line}");
        if let Some(f) = out.as_mut() {
            writeln!(f, "{line}").unwrap();
        }
    }
}
