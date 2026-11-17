//! **Joint Coppersmith lattice for the three-equation `(m, w) = (4, 2)`
//! system** (follow-up to `research/smallroot_optimal_reach_20261026/`).
//!
//! The system: `f1 = S₃(x1, x2, u)`, `f2 = S₃(x3, x4, v)`,
//! `f3 = S₃(u, v, x_R)` in six unknowns, with bounds `p^β` on the leaves and
//! `p^{γ_u}`, `p^{γ_v}` on the kept internal nodes.  The index-calculus
//! demand (CLOSURE.md §1, unchanged by CORR-20261024-b2a799) is
//! `β < 1/4` (linear algebra) and `3β + γ_u + γ_v > 5/2` (relation yield).
//!
//! A joint lattice uses all of `(p, f1, f2, f3)^t` on a monomial set `M`.
//! As in the single-block theorem, its gain obeys
//! `E(M) ≤ Σ_s dim(Ī^s ∩ span M)`, and under a weight order refining `δ`
//! every monomial counted at layer `s` has `δ`-weight at least
//! `ν_s = min { deg_δ g : 0 ≠ g ∈ Ī^s }`.  The Howgrave-Graham condition
//! `Σ_{μ∈M} <δ,μ> < E(M)` therefore needs `ν_s < s` for some `s`.  For one
//! block `ν_s = s·h_N(δ)` exactly; here the initial forms of `f1` and `f3`
//! share `u²` (and `f2`, `f3` share `v²`), so `ν_s` can fall below
//! `s·min_k deg_δ f_k`, and only a computation says by how much.
//!
//! This example computes `ν_s(T)`, the least `δ`-weight in the row space of
//! the Macaulay matrix `{x^a f^α : |α| = s, deg ≤ T}` over `F_p` (columns in
//! descending `δ`-weight, row echelon; the least pivot weight).  Truncation
//! only removes ideal elements, so `ν_s(T)` is an **upper bound** on `ν_s`,
//! non-increasing in `T`: a value below `s` would show a joint lattice can
//! meet the demand's determinant condition; values above `s` are evidence,
//! not proof.
//!
//! `cargo run --release --example smallroot_joint_lattice`
//!
//! Output: JSON lines.  Analysis computation, no measurement, no claim.

use std::collections::HashMap;

const NV: usize = 6;
type Mono = [u8; NV];

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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

type Pol = HashMap<Mono, u64>;

fn add(f: &Pol, g: &Pol, p: u64) -> Pol {
    let mut h = f.clone();
    for (e, &c) in g {
        let v = h.entry(*e).or_insert(0);
        *v = (*v + c) % p;
    }
    h.retain(|_, c| *c != 0);
    h
}
fn mul(f: &Pol, g: &Pol, p: u64) -> Pol {
    let mut h: Pol = HashMap::new();
    for (a, &ca) in f {
        for (b, &cb) in g {
            let mut e = [0u8; NV];
            for i in 0..NV {
                e[i] = a[i] + b[i];
            }
            let v = h.entry(e).or_insert(0);
            *v = (*v + mulm(ca, cb, p)) % p;
        }
    }
    h.retain(|_, c| *c != 0);
    h
}
fn konst(c: u64, p: u64) -> Pol {
    let mut h = HashMap::new();
    if !c.is_multiple_of(p) {
        h.insert([0; NV], c % p);
    }
    h
}
fn var(i: usize) -> Pol {
    let mut e = [0u8; NV];
    e[i] = 1;
    let mut h = HashMap::new();
    h.insert(e, 1);
    h
}
fn scale(f: &Pol, k: u64, p: u64) -> Pol {
    f.iter()
        .map(|(e, &c)| (*e, mulm(c, k % p, p)))
        .filter(|(_, c)| *c != 0)
        .collect()
}

/// `S₃(a, b, c)` for `y² = x³ + A x + B`.
fn s3(a: &Pol, b: &Pol, c: &Pol, ca: u64, cb: u64, p: u64) -> Pol {
    let neg = |f: &Pol| scale(f, p - 1, p);
    let d = add(a, &neg(b), p);
    let t1 = mul(&mul(&mul(&d, &d, p), c, p), c, p);
    let s = add(a, b, p);
    let pr = mul(a, b, p);
    let inner = add(
        &mul(&s, &add(&pr, &konst(ca, p), p), p),
        &konst(mulm(2, cb, p), p),
        p,
    );
    let t2 = scale(&mul(&inner, c, p), p - 2, p);
    let q = add(&pr, &konst(p - ca % p, p), p);
    let t3 = add(&mul(&q, &q, p), &neg(&scale(&s, mulm(4, cb, p), p)), p);
    add(&add(&t1, &t2, p), &t3, p)
}

fn monomials(t: usize) -> Vec<Mono> {
    let mut out = Vec::new();
    let mut e = [0u8; NV];
    fn rec(i: usize, left: usize, e: &mut Mono, out: &mut Vec<Mono>) {
        if i == NV {
            out.push(*e);
            return;
        }
        for k in 0..=left {
            e[i] = k as u8;
            rec(i + 1, left - k, e, out);
        }
        e[i] = 0;
    }
    rec(0, t, &mut e, &mut out);
    out
}

fn weight(e: &Mono, w: &[f64; NV]) -> f64 {
    (0..NV).map(|i| e[i] as f64 * w[i]).sum()
}

/// Least `δ`-weight in the span of `{x^a f^α : |α| = s, deg ≤ t_max}`.
fn nu(gens: &[Pol; 3], s: usize, t_max: usize, w: &[f64; NV], p: u64) -> (f64, usize, usize) {
    let mut cols = monomials(t_max);
    cols.sort_by(|a, b| {
        weight(b, w)
            .partial_cmp(&weight(a, w))
            .unwrap()
            .then(b.cmp(a))
    });
    let idx: HashMap<Mono, usize> = cols.iter().enumerate().map(|(i, e)| (*e, i)).collect();
    // products f^α, |α| = s
    let mut prods: Vec<Pol> = Vec::new();
    for a1 in 0..=s {
        for a2 in 0..=s - a1 {
            let a3 = s - a1 - a2;
            let mut g = konst(1, p);
            for _ in 0..a1 {
                g = mul(&g, &gens[0], p);
            }
            for _ in 0..a2 {
                g = mul(&g, &gens[1], p);
            }
            for _ in 0..a3 {
                g = mul(&g, &gens[2], p);
            }
            prods.push(g);
        }
    }
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for g in &prods {
        let dg = g
            .keys()
            .map(|e| e.iter().map(|&x| x as usize).sum::<usize>())
            .max()
            .unwrap();
        if dg > t_max {
            continue;
        }
        for a in monomials(t_max - dg) {
            let mut row = vec![0u64; cols.len()];
            for (e, &c) in g {
                let mut m = [0u8; NV];
                for i in 0..NV {
                    m[i] = e[i] + a[i];
                }
                row[idx[&m]] = c;
            }
            rows.push(row);
        }
    }
    let n_rows = rows.len();
    let mut best = f64::INFINITY;
    let mut start = 0;
    for c in 0..cols.len() {
        if start == rows.len() {
            break;
        }
        let Some(r) = (start..rows.len()).find(|&r| rows[r][c] != 0) else {
            continue;
        };
        rows.swap(start, r);
        let inv = powm(rows[start][c], p - 2, p);
        let piv: Vec<u64> = rows[start].iter().map(|&x| mulm(x, inv, p)).collect();
        rows[start] = piv.clone();
        for row in rows.iter_mut().skip(start + 1) {
            let x = row[c];
            if x == 0 {
                continue;
            }
            for j in c..row.len() {
                if piv[j] != 0 {
                    row[j] = (row[j] + p - mulm(x, piv[j], p)) % p;
                }
            }
        }
        best = best.min(weight(&cols[c], w));
        start += 1;
    }
    (best, n_rows, cols.len())
}

fn main() {
    let p: u64 = 1_000_003;
    let mut rng = Rng(0x4A01_2026);
    let (ca, cb, xr) = (rng.below(p), 1 + rng.below(p - 1), rng.below(p));
    let v = |i: usize| var(i);
    let gens = [
        s3(&v(0), &v(1), &v(4), ca, cb, p),
        s3(&v(2), &v(3), &v(5), ca, cb, p),
        s3(&v(4), &v(5), &konst(xr, p), ca, cb, p),
    ];
    // Positive control: {f1, f2, f1 + x1} contains x1, so ν_1 must equal β.
    if std::env::args().any(|a| a == "--control") {
        let ctrl = [gens[0].clone(), gens[1].clone(), add(&gens[0], &var(0), p)];
        let w = [0.2, 0.2, 0.2, 0.2, 0.95, 0.95];
        for t in [6usize, 8] {
            let (nu_s, rows, cols) = nu(&ctrl, 1, t, &w, p);
            println!("{{\"control\":\"f1, f2, f1+x1\",\"expected_nu_1\":0.2,\"T\":{t},\"rows\":{rows},\"cols\":{cols},\"nu_1\":{nu_s:.4}}}");
        }
        return;
    }
    // Demand-shaped bounds: β in (1/6, 1/4), γu + γv just above 5/2 − 3β.
    let cells: Vec<(f64, f64, f64)> = vec![
        (0.17, 0.99, 0.99),
        (0.20, 0.95, 0.95),
        (0.24, 0.90, 0.90),
        (0.20, 1.00, 0.90),
        (0.24, 1.00, 0.80),
        // control: the per-block optimum shape (one heavy variable)
        (0.20, 0.50, 0.50),
    ];
    let truncs: [(usize, &[usize]); 3] = [(1, &[6, 8, 10]), (2, &[8, 10, 12]), (3, &[12, 13])];
    for (beta, gu, gv) in cells {
        let w = [beta, beta, beta, beta, gu, gv];
        let demand_ok = beta < 0.25 && 3.0 * beta + gu + gv > 2.5;
        let hdeg: Vec<f64> = gens
            .iter()
            .map(|g| g.keys().map(|e| weight(e, &w)).fold(0.0, f64::max))
            .collect();
        for (s, ts) in truncs.iter() {
            for &t in ts.iter() {
                let (nu_s, rows, cols) = nu(&gens, *s, t, &w, p);
                println!(
                    "{{\"beta\":{beta},\"gamma_u\":{gu},\"gamma_v\":{gv},\"in_demand_region\":{demand_ok},\
                     \"deg_delta_f\":[{:.3},{:.3},{:.3}],\"s\":{s},\"T\":{t},\"rows\":{rows},\"cols\":{cols},\
                     \"nu_s_upper\":{nu_s:.4},\"nu_s_over_s\":{:.4}}}",
                    hdeg[0],
                    hdeg[1],
                    hdeg[2],
                    nu_s / *s as f64
                );
            }
        }
    }
}
