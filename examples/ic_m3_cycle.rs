//! Three-summand (m = 3) algebraic index calculus on Koblitz curves over GF(2^n), n <= 63, with a
//! single-target dependency stop, beside the strong-rho reference on the same target.
//!
//! The decomposer, factor base, anchors and strong rho are those of `ic_m2_cycle.rs`, unchanged.
//! Relations with three summands come from a "2 + 1" split: for an anchor point p and every base
//! point P (both signs), decompose R' = p - eP over the base with the linear two-summand solve
//! (Semaev's S3 over V, linear after Weil descent). Each success gives
//! `p = eP + e2 P2 + e3 P3`, a relation with up to three terms (merged by column).
//!
//! Anchors are `A = [a]G + [b]T`, so every relation's right side is affine in t: `alpha + beta t`.
//! With three terms per relation the relation graph is a hypergraph, so the union-find of the m = 2
//! producer does not apply. Relations are instead reduced incrementally into a row echelon form
//! modulo the prime r. The first relation that reduces to the zero row leaves `0 = alpha' + beta' t`;
//! with `beta' != 0` that gives t, which is checked as `[t]G = T`. This is the same stop as the
//! m = 2 cycle stop (a closed equation in t), generalized from graph cycles to linear dependencies.
//!
//! Requires r < 2^32 (row arithmetic in u64). Usage:
//! `ic_m3_cycle <a> <n> <d|0> <targets> <seed> <out.json>`; targets use the same derivation as
//! `ic_m2_cycle` (same seed, same secrets). The "trials" counter counts decomposer calls, the same
//! unit as the m = 2 producers; group additions are counted separately.

use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::koblitz_strong_rho::{
    RawPoint, StrongRho, StrongRhoCharges, StrongRhoParams,
};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_traits::ToPrimitive;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;
use std::time::Instant;

type Pt = Option<(u64, u64)>;

struct Curve {
    f: Gf2,
    n: u32,
    a: u64,
    b: u64,
    irr: u64,
}

#[derive(Default, Clone, Copy)]
struct Counts {
    group_additions: u64,
    trials: u64,
    consistent: u64,
    split_attempts: u64,
    relations: u64,
    gauss_xors: u64,
    field_muls: u64,
}

impl Curve {
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }

    fn add(&self, p: Pt, q: Pt, c: &mut Counts) -> Pt {
        c.group_additions += 1;
        let Some((x1, y1)) = p else { return q };
        let Some((x2, y2)) = q else { return p };
        let f = &self.f;
        if x1 == x2 {
            if y1 ^ y2 == x2 || x1 == 0 {
                return None;
            }
            let l = x1 ^ f.mul(y1, f.inv(x1));
            let x3 = f.sqr(l) ^ l ^ self.a;
            let y3 = f.sqr(x1) ^ f.mul(l ^ 1, x3);
            return Some((x3, y3));
        }
        let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sqr(l) ^ l ^ x1 ^ x2 ^ self.a;
        let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }

    fn mul(&self, p: Pt, k: u64, c: &mut Counts) -> Pt {
        let mut acc = None;
        for bit in (0..64 - k.leading_zeros()).rev() {
            acc = self.add(acc, acc, c);
            if (k >> bit) & 1 == 1 {
                acc = self.add(acc, p, c);
            }
        }
        acc
    }

    fn frob(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (self.f.sqr(x), self.f.sqr(y)))
    }

    /// A point with this x, if any: `y = x z`, `z² + z = x + a + b/x²`.
    fn lift_x(&self, x: u64) -> Pt {
        if x == 0 {
            return None;
        }
        let f = &self.f;
        let c = x ^ self.a ^ f.mul(self.b, f.sqr(f.inv(x)));
        let (mut t, mut tr) = (c, c);
        for _ in 1..self.n {
            t = f.sqr(t);
            tr ^= t;
        }
        if tr != 0 {
            return None;
        }
        let (mut t, mut z) = (c, c);
        for _ in 0..(self.n - 1) / 2 {
            t = f.sqr(f.sqr(t));
            z ^= t;
        }
        Some((x, f.mul(x, z)))
    }

    /// `v · z` in the field.
    #[inline(always)]
    fn times_z(&self, v: u64) -> u64 {
        let w = v << 1;
        if (w >> self.n) & 1 == 1 {
            w ^ self.irr
        } else {
            w
        }
    }
}

fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn invmod(a: u64, r: u64) -> u64 {
    let (mut e, mut base, mut acc) = (r - 2, a % r, 1u64);
    while e != 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, base, r);
        }
        base = mulmod(base, base, r);
        e >>= 1;
    }
    acc
}

/// Solve `Σ u_k cols[k] = rhs` over F2 (vectors and combinations as u64
/// bit sets); returns a particular solution and a kernel basis, or None.
fn solve_f2(cols: &[u64], rhs: u64, xors: &mut u64) -> Option<(u64, Vec<u64>)> {
    let mut piv: [(u64, u64); 64] = [(0, 0); 64];
    let mut kernel = Vec::new();
    for (k, &col) in cols.iter().enumerate() {
        let (mut v, mut comb) = (col, 1u64 << k);
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            if piv[top].0 == 0 {
                piv[top] = (v, comb);
                break;
            }
            v ^= piv[top].0;
            comb ^= piv[top].1;
            *xors += 1;
        }
        if v == 0 {
            kernel.push(comb);
        }
    }
    let (mut v, mut comb) = (rhs, 0u64);
    while v != 0 {
        let top = 63 - v.leading_zeros() as usize;
        if piv[top].0 == 0 {
            return None;
        }
        v ^= piv[top].0;
        comb ^= piv[top].1;
        *xors += 1;
    }
    Some((comb, kernel))
}

/// Carry-less product of two polynomials over F2 (degrees < 32 each).
fn clmul(a: u64, b: u64) -> u64 {
    let mut acc = 0;
    let mut a = a;
    let mut i = 0;
    while a != 0 {
        if a & 1 == 1 {
            acc ^= b << i;
        }
        a >>= 1;
        i += 1;
    }
    acc
}

struct Decomposer {
    d: usize,
    /// `z^{2j}` in the field, j < 2d − 1.
    z2: Vec<u64>,
    /// Base column of each x in V (u32::MAX when x is not rational).
    col_of_x: Vec<u32>,
    points: Vec<(u64, u64)>,
}

/// A verified two-term relation `R = Σ coef_i P_{col_i}`.
#[derive(Clone, Debug)]
struct Rel {
    terms: Vec<(u32, i64)>,
}

impl Decomposer {
    /// All decompositions `R = ε1 P1 + ε2 P2` (or `±P1`, `±2P1`) of R with
    /// P1, P2 in the base, each verified in the group.
    fn decompose(&self, cv: &Curve, r: (u64, u64), c: &mut Counts) -> Vec<Rel> {
        let mut out = Vec::new();
        let d = self.d;
        let (xr, _) = r;
        c.trials += 1;
        // Direct hit: x(R) in V.
        if xr < (1u64 << d) && self.col_of_x[xr as usize] != u32::MAX {
            let k = self.col_of_x[xr as usize];
            let p = Some(self.points[k as usize]);
            let e = if p == Some(r) { 1 } else { -1 };
            out.push(Rel {
                terms: vec![(k, e)],
            });
        }
        // Columns: s-part xR² z^{2i} (i < d), p-part z^{2j} + xR z^j (j < 2d−1).
        let f = &cv.f;
        let xr2 = f.sqr(xr);
        c.field_muls += 1;
        let mut cols = Vec::with_capacity(3 * d - 1);
        let mut t = xr2;
        for _ in 0..d {
            cols.push(t);
            t = cv.times_z(cv.times_z(t));
        }
        let mut t = xr;
        for j in 0..2 * d - 1 {
            cols.push(self.z2[j] ^ t);
            t = cv.times_z(t);
        }
        let Some((part, kernel)) = solve_f2(&cols, cv.b, &mut c.gauss_xors) else {
            return out;
        };
        c.consistent += 1;
        let dim = kernel.len().min(12);
        let dmask = (1u64 << d) - 1;
        for sel in 0u64..(1u64 << dim) {
            let mut u = part;
            for (i, &kv) in kernel.iter().take(dim).enumerate() {
                if (sel >> i) & 1 == 1 {
                    u ^= kv;
                }
            }
            let s = u & dmask;
            let p = u >> d;
            c.split_attempts += 1;
            // x1² + s x1 = p in F2[z]_{<d}.
            let mut scol = Vec::with_capacity(d);
            for i in 0..d {
                scol.push((1u64 << (2 * i)) ^ clmul(s, 1u64 << i));
            }
            let Some((x1, _)) = solve_f2(&scol, p, &mut c.gauss_xors) else {
                continue;
            };
            let x2 = x1 ^ s;
            if x1 == 0 || x2 == 0 || x1 > dmask || x2 > dmask {
                continue;
            }
            let (k1, k2) = (self.col_of_x[x1 as usize], self.col_of_x[x2 as usize]);
            if k1 == u32::MAX || k2 == u32::MAX {
                continue;
            }
            let (p1, p2) = (
                Some(self.points[k1 as usize]),
                Some(self.points[k2 as usize]),
            );
            if k1 == k2 {
                let dbl = cv.add(p1, p1, c);
                if dbl == Some(r) {
                    out.push(Rel {
                        terms: vec![(k1, 2)],
                    });
                } else if cv.neg(dbl) == Some(r) {
                    out.push(Rel {
                        terms: vec![(k1, -2)],
                    });
                }
                continue;
            }
            let sum = cv.add(p1, p2, c);
            let rel = if sum == Some(r) {
                Some((1, 1))
            } else if cv.neg(sum) == Some(r) {
                Some((-1, -1))
            } else {
                let diff = cv.add(p1, cv.neg(p2), c);
                if diff == Some(r) {
                    Some((1, -1))
                } else if cv.neg(diff) == Some(r) {
                    Some((-1, 1))
                } else {
                    None
                }
            };
            if let Some((e1, e2)) = rel {
                out.push(Rel {
                    terms: vec![(k1, e1), (k2, e2)],
                });
            }
        }
        out
    }
}

/// `α + β t (mod r)`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Aff(u64, u64);

fn red(k: i64, r: u64) -> u64 {
    let ri = r as i128;
    (((k as i128) % ri + ri) % ri) as u64
}

impl Aff {
    fn add(self, o: Aff, r: u64) -> Aff {
        Aff((self.0 + o.0) % r, (self.1 + o.1) % r)
    }
    fn mulu(self, k: u64, r: u64) -> Aff {
        Aff(mulmod(self.0, k, r), mulmod(self.1, k, r))
    }
}

/// One row of the echelon form: `sum_j c[j] l_j = alpha + beta t (mod r)`; `c[j] = 0` before the lead.
struct Row {
    c: Vec<u32>,
    a: u64,
    b: u64,
}

enum Outcome {
    /// The relation was independent; it became a new row.
    Independent,
    /// The relation reduced to zero: `0 = alpha + beta t`.
    Dependent(u64, u64),
}

/// Incremental row echelon form over the K base columns, modulo the prime r.
struct Echelon {
    r: u64,
    k: usize,
    piv: Vec<Option<usize>>,
    rows: Vec<Row>,
}

impl Echelon {
    fn new(k: usize, r: u64) -> Self {
        Self {
            r,
            k,
            piv: vec![None; k],
            rows: Vec::new(),
        }
    }

    /// Add `sum e_i l_{col_i} = rhs` (rhs affine in t).
    fn add(&mut self, terms: &[(u32, i64)], rhs: Aff) -> Outcome {
        let r = self.r;
        let mut v = vec![0u64; self.k];
        for &(col, e) in terms {
            let x = &mut v[col as usize];
            *x = (*x + red(e, r)) % r;
        }
        let (mut a, mut b) = (rhs.0 % r, rhs.1 % r);
        let mut c = 0usize;
        loop {
            while c < self.k && v[c] == 0 {
                c += 1;
            }
            if c == self.k {
                return Outcome::Dependent(a, b);
            }
            match self.piv[c] {
                Some(ri) => {
                    let m = v[c];
                    let row = &self.rows[ri];
                    for j in c..self.k {
                        let t = (m * row.c[j] as u64) % r;
                        v[j] = (v[j] + r - t) % r;
                    }
                    a = (a + r - (m * row.a) % r) % r;
                    b = (b + r - (m * row.b) % r) % r;
                    c += 1;
                }
                None => {
                    let inv = invmod(v[c], r);
                    let mut cv = vec![0u32; self.k];
                    for j in c..self.k {
                        cv[j] = ((v[j] * inv) % r) as u32;
                    }
                    self.piv[c] = Some(self.rows.len());
                    self.rows.push(Row {
                        c: cv,
                        a: (a * inv) % r,
                        b: (b * inv) % r,
                    });
                    return Outcome::Independent;
                }
            }
        }
    }
}

/// Merge duplicate columns of a relation's term list and drop zero coefficients.
fn merge_terms(terms: &mut Vec<(u32, i64)>) {
    terms.sort_by_key(|t| t.0);
    let mut out: Vec<(u32, i64)> = Vec::with_capacity(terms.len());
    for &(k, e) in terms.iter() {
        match out.last_mut() {
            Some(last) if last.0 == k => last.1 += e,
            _ => out.push((k, e)),
        }
    }
    out.retain(|t| t.1 != 0);
    *terms = out;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        eprintln!("usage: ic_m3_cycle <a> <n> <d|0> <targets> <seed> <out.json>");
        std::process::exit(2);
    }
    let a: u8 = args[1].parse().unwrap();
    let n: u32 = args[2].parse().unwrap();
    let mut d: usize = args[3].parse().unwrap();
    let targets: usize = args[4].parse().unwrap();
    let seed: u64 = args[5].parse().unwrap();
    if d == 0 {
        d = (n as usize + 1) / 3;
    }
    assert!(
        2 * d - 1 <= n as usize && 3 * d - 1 <= 63,
        "need 2d - 1 <= n and 3d - 1 <= 63"
    );

    let kc = KoblitzCurve::new(a, n).expect("Koblitz curve with a usable subgroup");
    let r = kc.subgroup_order.to_u64().expect("r fits u64");
    assert!(r < (1u64 << 32), "row arithmetic needs r < 2^32");
    let h = kc.cofactor.to_u64().expect("cofactor fits u64");
    let f = Gf2::new(&kc.curve.irreducible);
    let irr = f.irr;
    let cv = Curve {
        f,
        n,
        a: a as u64,
        b: 1,
        irr,
    };
    let g: (u64, u64) = match RawPoint::from_binary(kc.generator()) {
        RawPoint::Affine { x, y } => (x, y),
        RawPoint::Infinity => panic!("generator at infinity"),
    };
    let lambda = kc.lambda.to_u64().unwrap_or(0);
    let mut scratch = Counts::default();
    assert_eq!(cv.mul(Some(g), r, &mut scratch), None, "[r]G = O");
    assert_eq!(
        cv.frob(Some(g)),
        cv.mul(Some(g), lambda, &mut scratch),
        "phi(G) = [lambda]G"
    );
    let hr = h % r;

    let rho = StrongRho::new(&kc);
    let mut rows_json = Vec::new();
    for ti in 0..targets {
        let mut st = seed ^ (ti as u64).wrapping_mul(0x9e3779b97f4a7c15);
        let mut next = || {
            st = st.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = st;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        };
        let t_secret = next() % (r - 1) + 1;
        let target = cv.mul(Some(g), t_secret, &mut scratch).unwrap();

        // ---------------- IC (cold, one target) ----------------
        let mut c = Counts::default();
        let started = Instant::now();
        let mut col_of_x = vec![u32::MAX; 1usize << d];
        let mut points = Vec::new();
        for x in 1u64..(1u64 << d) {
            if let Some(p) = cv.lift_x(x) {
                col_of_x[x as usize] = points.len() as u32;
                points.push(p);
            }
        }
        let k = points.len();
        let mut z2 = Vec::with_capacity(2 * d);
        for j in 0..2 * d - 1 {
            let zj = if j < n as usize { 1u64 << j } else { 0 };
            z2.push(cv.f.sqr(zj));
        }
        let dec = Decomposer {
            d,
            z2,
            col_of_x,
            points,
        };
        let mut ech = Echelon::new(k, r);
        let (mut useless, mut inconsistent, mut bad_t) = (0u64, 0u64, 0u64);
        // Columns whose projection [h]P is O have logarithm 0.
        for (i, &p) in dec.points.iter().enumerate() {
            if cv.mul(Some(p), h, &mut c).is_none() {
                ech.add(&[(i as u32, 1)], Aff(0, 0));
            }
        }
        let setup_s = started.elapsed().as_secs_f64();
        let mut steps: Vec<(Pt, Aff)> = Vec::with_capacity(16);
        for _ in 0..16 {
            let (u, v) = (next() % (r - 1) + 1, next() % (r - 1) + 1);
            let s = cv.add(
                cv.mul(Some(g), u, &mut c),
                cv.mul(Some(target), v, &mut c),
                &mut c,
            );
            steps.push((s, Aff(u, v)));
        }
        let fresh = |c: &mut Counts, next: &mut dyn FnMut() -> u64| -> (Pt, Aff) {
            let (u, v) = (next() % (r - 1) + 1, next() % (r - 1) + 1);
            let s = cv.add(cv.mul(Some(g), u, c), cv.mul(Some(target), v, c), c);
            (s, Aff(u, v))
        };
        let (mut anchor, mut alog) = fresh(&mut c, &mut next);
        let mut anchors_used = 0u64;
        let recovered: u64;
        'outer: loop {
            if let Some(ap) = anchor {
                anchors_used += 1;
                let mut p = Some(ap);
                let mut lg = alog;
                for _ in 0..n {
                    let pp = p.unwrap();
                    for col in 0..k {
                        let pk = dec.points[col];
                        for eps in [1i64, -1i64] {
                            // R' = p - eps * P_col
                            let q = if eps == 1 {
                                cv.add(Some(pp), cv.neg(Some(pk)), &mut c)
                            } else {
                                cv.add(Some(pp), Some(pk), &mut c)
                            };
                            let mut batch: Vec<Vec<(u32, i64)>> = Vec::new();
                            match q {
                                None => batch.push(vec![(col as u32, eps)]),
                                Some(qq) => {
                                    for rel in dec.decompose(&cv, qq, &mut c) {
                                        let mut terms = vec![(col as u32, eps)];
                                        terms.extend(rel.terms.iter().map(|&(kk, e)| (kk, e)));
                                        merge_terms(&mut terms);
                                        batch.push(terms);
                                    }
                                }
                            }
                            for terms in batch {
                                c.relations += 1;
                                // [h]p = sum terms * [h]P  =>  sum coef l = h * (alpha + beta t)
                                if let Outcome::Dependent(da, db) = ech.add(&terms, lg.mulu(hr, r))
                                {
                                    if db != 0 {
                                        let t = mulmod((r - da) % r, invmod(db, r), r);
                                        if cv.mul(Some(g), t, &mut scratch) == Some(target) {
                                            recovered = t;
                                            break 'outer;
                                        }
                                        bad_t += 1;
                                    } else if da != 0 {
                                        inconsistent += 1;
                                    } else {
                                        useless += 1;
                                    }
                                }
                            }
                        }
                    }
                    p = cv.frob(p);
                    lg = lg.mulu(lambda, r);
                }
            }
            let (w, wl) = steps[(next() % 16) as usize];
            match cv.add(anchor, w, &mut c) {
                Some(q) => {
                    anchor = Some(q);
                    alog = alog.add(wl, r);
                }
                None => {
                    let (fa, fl) = fresh(&mut c, &mut next);
                    anchor = fa;
                    alog = fl;
                }
            }
        }
        let ic_s = started.elapsed().as_secs_f64();
        let verified = cv.mul(Some(g), recovered, &mut scratch) == Some(target);

        // ---------------- strong rho, same target ----------------
        let rho_started = Instant::now();
        let mut charges = StrongRhoCharges::default();
        let jumps = rho.jumps(seed ^ 0x6a75 ^ ti as u64, &mut charges);
        let mut rng = StdRng::seed_from_u64(seed ^ 0x7268 ^ ti as u64);
        let params = StrongRhoParams::default();
        let q = RawPoint::Affine {
            x: target.0,
            y: target.1,
        };
        let out = rho.solve(q, &jumps, &mut rng, &params, charges);
        let rho_s = rho_started.elapsed().as_secs_f64();
        let (rho_scalar, rho_steps, rho_adds) = match &out {
            Some(o) => (Some(o.scalar), o.walk_steps, o.charges.group_additions),
            None => (None, 0, 0),
        };
        rows_json.push(json!({
            "target": ti, "secret": t_secret.to_string(), "ic_recovered": recovered.to_string(),
            "ic_verified": verified && recovered == t_secret % r,
            "ic_s": ic_s, "ic_setup_s": setup_s,
            "ic_trials": c.trials, "ic_consistent": c.consistent, "ic_split_attempts": c.split_attempts,
            "ic_relations": c.relations, "ic_group_additions": c.group_additions,
            "ic_gauss_xors": c.gauss_xors, "ic_columns": k,
            "ic_rank": ech.rows.len(), "ic_anchors_used": anchors_used,
            "ic_useless_dependencies": useless, "ic_inconsistent": inconsistent, "ic_bad_t": bad_t,
            "rho_s": rho_s, "rho_recovered": rho_scalar.map(|v| v.to_string()),
            "rho_verified": rho_scalar == Some(t_secret % r),
            "rho_walk_steps": rho_steps, "rho_group_additions": rho_adds,
        }));
        eprintln!(
            "n={n} d={d} t={ti}: ic {:.3}s ({} trials, {} rels, rank {}/{k}, {} anchors, bad {bad_t}, incons {inconsistent}) verified={} | rho {:.4}s",
            ic_s, c.trials, c.relations, ech.rows.len(), anchors_used, verified, rho_s
        );
    }
    let out = json!({
        "schema": "ic-m3-cycle/v1", "a": a, "n": n, "d": d, "seed": seed, "summands": 3,
        "curve_id": format!("{:?}", kc.curve_id()), "r": r.to_string(), "cofactor": h.to_string(),
        "log2_r": (r as f64).log2(),
        "factor_base": format!("x in span{{1..z^{}}} (polynomial basis), both signs per x", d - 1),
        "stop": "first linear dependency leaving an equation in t (anchors [a]G + [b]T), 2+1 split",
        "rows": rows_json,
    });
    std::fs::write(&args[6], serde_json::to_string_pretty(&out).unwrap()).unwrap();
}
