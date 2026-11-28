//! `smith_walk_dlp_p7`: re-run of the round-1 push-forward test (P7) with the library module
//! `plane_quartic_pic` copied to research/isogeny-walk-quartic_20261121/src/pqp.rs and its
//! completion bound raised from 7 to 12, so that no test pair panics. Local development check,
//! all figures `[pcc:unverified]`. The test code is the round-1 `pushfwd` (unchanged apart from
//! the import of the copied module and a retry with another divisor on a panic).
//!
//! `smith_walk_dlp_p7 --p P --seed S --want N`

#[allow(dead_code)]
#[path = "../../research/isogeny-walk-quartic_20261121/src/pqp.rs"]
mod pqp;

#[allow(dead_code)]
#[path = "../../research/isogeny-walk-quartic_20261115/src/planefit.rs"]
mod planefit;

use crypto_lib::cryptanalysis::smith_prop24::{analyze_curve, peval, SplitMix64};
use crypto_lib::cryptanalysis::smith_trigonal::{Fp2, Trigonal, F2};
use pqp::{mons_of_degree, PicClass, PlaneQuartic, Pt};
use serde_json::json;
use std::collections::BTreeMap;

fn arg(a: &[String], k: &str) -> Option<String> {
    a.iter()
        .position(|x| x == k)
        .and_then(|i| a.get(i + 1))
        .cloned()
}

/// Random Smith-positive degree-7 curves (generator copied from smith_trigonal tests).
fn positives(p: u64, want: usize, seed: u64) -> Vec<(Vec<u64>, Trigonal)> {
    positives_lc(p, want, seed, false)
}

/// As `positives`; with `monic` the leading coefficient is 1 (needed by the Cantor arithmetic).
fn positives_lc(p: u64, want: usize, seed: u64, monic: bool) -> Vec<(Vec<u64>, Trigonal)> {
    let mut rng = SplitMix64(seed);
    let mut out = Vec::new();
    let mut tries = 0;
    while out.len() < want && tries < 50_000 {
        tries += 1;
        let mut f: Vec<u64> = (0..7).map(|_| rng.below(p)).collect();
        f.push(if monic { 1 } else { 1 + rng.below(p - 1) });
        let Some(co) = analyze_curve(&f, p, &mut rng) else {
            continue;
        };
        let found = co
            .subgroups
            .iter()
            .filter(|s| s.trig_rational() && s.iso_rational(false))
            .flat_map(|s| s.maps.iter())
            .filter(|m| m.valid() && m.alpha_chi == 1)
            .find_map(|m| m.map.smith_coeffs());
        if let Some((n1, n0, d1, d0)) = found {
            if let Ok(t) = Trigonal::new(&f, p, n1, n0, d1, d0) {
                out.push((f, t));
            }
        }
    }
    out
}

/// Multiply a polynomial of degree < 3 by `x` modulo the monic cubic `x^3 + g2 x^2 + g1 x + g0`.
type Cub = [Fp2; 3];

/// Multiply by x modulo the monic cubic.
fn mulx(k: &F2, a: Cub, g: [u64; 3]) -> Cub {
    let top = a[2];
    [
        k.neg(k.scale(top, g[0])),
        k.sub(a[0], k.scale(top, g[1])),
        k.sub(a[1], k.scale(top, g[2])),
    ]
}

/// `a * b` modulo `G`.
fn mulmod(k: &F2, a: Cub, b: Cub, g: [u64; 3]) -> Cub {
    let mut r = [Fp2::default(); 3];
    let mut xb = b;
    for ai in a {
        for (rj, bj) in r.iter_mut().zip(xb.iter()) {
            *rj = k.add(*rj, k.mul(ai, *bj));
        }
        xb = mulx(k, xb, g);
    }
    r
}

/// Solve the 3x3 system `m z = rhs` over F_{p^2}; `None` if singular.
fn solve3(k: &F2, mut m: [[Fp2; 4]; 3]) -> Option<[Fp2; 3]> {
    for c in 0..3 {
        let piv = (c..3).find(|&i| !k.is_zero(m[i][c]))?;
        m.swap(c, piv);
        let iv = k.inv(m[c][c]);
        for j in 0..4 {
            m[c][j] = k.mul(m[c][j], iv);
        }
        for i in 0..3 {
            if i != c && !k.is_zero(m[i][c]) {
                let f = m[i][c];
                for j in 0..4 {
                    m[i][j] = k.sub(m[i][j], k.mul(f, m[c][j]));
                }
            }
        }
    }
    Some([m[0][3], m[1][3], m[2][3]])
}

/// Canonical map of Recillas' curve X at the point with parameter `t` and Mumford `b`:
/// `h_i = sum over the three H-points (x_k, b(x_k)) of x_k^i / (y_k g'(x_k))`, i.e. the
/// coefficients of the trace of the H differentials `x^i dx / y` along the correspondence
/// R, written against `dt`. With `g' = G_x / D` this is `h_i = [x^2] (x^i D b^{-1} mod G)`
/// (Euler: `sum A(x_k)/G'(x_k) = [x^2] A`). Returns `None` when `b` is not invertible mod `G`.
fn canon_map(t: &Trigonal, k: &F2, tt: u64, b: [Fp2; 3]) -> Option<[Fp2; 3]> {
    let p = t.p;
    let g = [
        peval(&t.g[0], tt, p),
        peval(&t.g[1], tt, p),
        peval(&t.g[2], tt, p),
    ];
    let d = [k.c(t.d0), k.c(t.d1), k.c(1)];
    // column j of M_b is b * x^j mod G; solve M_b^T r = e2 so that h_i = r . (x^i D mod G).
    let mut cols = [[Fp2::default(); 3]; 3];
    let mut xj = [k.c(1), Fp2::default(), Fp2::default()];
    for col in cols.iter_mut() {
        *col = mulmod(k, b, xj, g);
        xj = mulx(k, xj, g);
    }
    // M_b^T r = e2: row j of M_b^T is column j of M_b.
    let mut m = [[Fp2::default(); 4]; 3];
    for j in 0..3 {
        m[j][..3].copy_from_slice(&cols[j]);
        m[j][3] = if j == 2 { k.c(1) } else { Fp2::default() };
    }
    let r = solve3(k, m)?;
    let mut v = d;
    let mut h = [Fp2::default(); 3];
    for hi in h.iter_mut() {
        let mut s = Fp2::default();
        for (rj, vj) in r.iter().zip(v.iter()) {
            s = k.add(s, k.mul(*rj, *vj));
        }
        *hi = s;
        v = mulx(k, v, g);
    }
    Some(h)
}

/// Affine chart (h0/h2, h1/h2) of the canonical image of every X point over U.
fn canon_points(t: &Trigonal, k: &F2) -> (Vec<(Fp2, Fp2)>, u64, u64) {
    let mut pts = Vec::new();
    let mut seen = std::collections::HashSet::new();
    let (mut errs, mut skipped) = (0u64, 0u64);
    for (x, y) in t.points_over_u() {
        match t.image_points(x, y) {
            Ok(ip) => {
                for q in ip.points {
                    if !seen.insert(q.mumford) {
                        continue;
                    }
                    match canon_map(t, k, q.t, q.mumford) {
                        Some(h) if !k.is_zero(h[2]) => {
                            let i2 = k.inv(h[2]);
                            pts.push((k.mul(h[0], i2), k.mul(h[1], i2)));
                        }
                        _ => skipped += 1,
                    }
                }
            }
            Err(_) => errs += 1,
        }
    }
    (pts, errs, skipped)
}

/// An X point in the chart (u, v) = (h0/h2, h1/h2) of the canonical plane.
type Chart = (Fp2, Fp2);

fn canon_chart(
    t: &Trigonal,
    k: &F2,
    q: &crypto_lib::cryptanalysis::smith_trigonal::XPoint,
) -> Option<Chart> {
    let h = canon_map(t, k, q.t, q.mumford)?;
    if k.is_zero(h[2]) {
        return None;
    }
    let i2 = k.inv(h[2]);
    Some((k.mul(h[0], i2), k.mul(h[1], i2)))
}

/// Build the canonical quartic of X from its points, with a rational base point.
fn build_quartic(t: &Trigonal, k: &F2) -> Result<PlaneQuartic, String> {
    let p = t.p;
    let (pts, _, _) = canon_points(t, k);
    let basis = planefit::kernel_basis(&pts, 4, p);
    if basis.len() != 1 {
        return Err(format!("kernel dim {} at d = 4", basis.len()));
    }
    let mons = planefit::monomials(4);
    let order = mons_of_degree(4);
    let mut f = [0u32; 15];
    for (idx, &(a, b, c)) in order.iter().enumerate() {
        // X^a Y^b Z^c with u^i v^j, i = a, j = b
        let pos = mons
            .iter()
            .position(|&(i, j)| i == a as usize && j == b as usize && (i + j + c as usize) == 4)
            .unwrap();
        f[idx] = basis[0][pos] as u32;
    }
    // base point: first rational point of the quartic by brute force
    // base point: first affine rational point of the quartic by brute force
    let mut p0 = None;
    'o: for x in 0..p as u32 {
        for y in 0..p as u32 {
            // evaluate F(x, y, 1)
            let mut v = 0u64;
            for (idx, &(a, b, _c)) in order.iter().enumerate() {
                v = (v + f[idx] as u64 * pw_mod(x as u64, a as u64, p) % p
                    * pw_mod(y as u64, b as u64, p))
                    % p;
            }
            if v == 0 {
                p0 = Some([x, y, 1]);
                break 'o;
            }
        }
    }
    let p0 = p0.ok_or("no affine rational point")?;
    PlaneQuartic::new(p as u32, f, p0).map_err(|e| format!("{e:?}"))
}

fn pw_mod(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}

/// Class of an effective degree-3 divisor made of rational points and at most one conjugate pair,
/// via the conics through it; `None` when it is collinear.
fn class_of_e3(c: &PlaneQuartic, k: &F2, pts: &[Chart]) -> Option<PicClass> {
    let p = c.q() as u64;
    let mons = c.std_monomials(2);
    let mut rows: Vec<Vec<u64>> = Vec::new();
    for &(u, v) in pts {
        let one = k.c(1);
        let vals: Vec<Fp2> = mons
            .iter()
            .map(|&(a, b, cc)| {
                let mut r = k.c(1);
                for _ in 0..a {
                    r = k.mul(r, u);
                }
                for _ in 0..b {
                    r = k.mul(r, v);
                }
                for _ in 0..cc {
                    r = k.mul(r, one);
                }
                r
            })
            .collect();
        rows.push(vals.iter().map(|z| z.a).collect());
        if vals.iter().any(|z| z.b != 0) {
            rows.push(vals.iter().map(|z| z.b).collect());
        }
    }
    let ns = planefit::nullspace(&mut rows, mons.len(), p);
    if ns.len() != 3 {
        return None;
    }
    Some(
        c.class_from_conics(
            ns.into_iter()
                .map(|r| r.into_iter().map(|x| x as u32).collect())
                .collect(),
        ),
    )
}

struct Pushfwd<'a> {
    c: &'a PlaneQuartic,
    k: F2,
    ratpts: Vec<Pt>,
}

impl Pushfwd<'_> {
    fn chart_of(&self, pt: Pt) -> Chart {
        // chart (X/Z, Y/Z); only used for points with Z != 0
        let zi = pw_mod(pt[2] as u64, self.k.p - 2, self.k.p);
        (
            self.k.c(pt[0] as u64 * zi % self.k.p),
            self.k.c(pt[1] as u64 * zi % self.k.p),
        )
    }

    /// Class of the degree-0 divisor `q + qbar - 2 P0` for a conjugate pair.
    fn conj_pair_class(&self, q: Chart) -> Option<PicClass> {
        let c = self.c;
        let k = &self.k;
        let p0 = c.base_point();
        let rp: Vec<Pt> = self
            .ratpts
            .iter()
            .copied()
            .filter(|&r| r != p0 && r[2] != 0)
            .collect();
        let qb = (k.conj(q.0), k.conj(q.1));
        // y = class(q+qb+C) - class(A+B+C) + [A-P0] + [B-P0]
        for i in 0..rp.len() {
            for j in (i + 1)..rp.len() {
                for l in 0..rp.len() {
                    if l == i || l == j {
                        continue;
                    }
                    let (a, b, cc) = (
                        self.chart_of(rp[i]),
                        self.chart_of(rp[j]),
                        self.chart_of(rp[l]),
                    );
                    let e1 = class_of_e3(c, k, &[q, qb, cc]);
                    let e2 = class_of_e3(c, k, &[a, b, cc]);
                    if let (Some(e1), Some(e2)) = (e1, e2) {
                        let d = c.sub(&e1, &e2);
                        let d = c.add(&d, &c.point_class(rp[i]));
                        return Some(c.add(&d, &c.point_class(rp[j])));
                    }
                }
            }
        }
        None
    }

    /// Phi(P) = [Q1 + Q2 - 2 P0] for the R-image pair of P.
    fn phi(&self, t: &Trigonal, x: u64, y: u64) -> Option<PicClass> {
        let k = &self.k;
        let ip = t.image_points(x, y).ok()?;
        let ch: Vec<Chart> = ip
            .points
            .iter()
            .map(|q| canon_chart(t, k, q))
            .collect::<Option<Vec<_>>>()?;
        if ip.rational {
            let pt = |c: Chart| -> Pt { self.c.normalize_point([c.0.a as u32, c.1.a as u32, 1]) };
            let a = self.c.point_class(pt(ch[0]));
            let b = self.c.point_class(pt(ch[1]));
            Some(self.c.add(&a, &b))
        } else {
            self.conj_pair_class(ch[0])
        }
    }
}

/// Test for one base point: `Phi(P) + Phi(iota P)` must be one class for all x. Returns
/// (x of the pairs tested, x of the pairs that panicked, x without a Phi, distinct sums,
/// distinct negative-control sums, negative-control pairs).
type BaseRun = (Vec<u64>, Vec<u64>, Vec<u64>, usize, usize, usize);

fn run_base(t: &Trigonal, k: F2, c: &PlaneQuartic) -> BaseRun {
    let ratpts = c.rational_points_bruteforce();
    let ctx = Pushfwd { c, k, ratpts };
    let mut by_x: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
    for (x, y) in t.points_over_u() {
        by_x.entry(x).or_default().push((x, y));
    }
    let (mut tested, mut panics, mut fails) = (Vec::new(), Vec::new(), Vec::new());
    let mut sums: Vec<PicClass> = Vec::new();
    let mut phis: Vec<(u64, PicClass)> = Vec::new();
    for (x, v) in by_x {
        if v.len() != 2 {
            continue;
        }
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let (pa, pb) = (ctx.phi(t, v[0].0, v[0].1), ctx.phi(t, v[1].0, v[1].1));
            match (pa, pb) {
                (Some(pa), Some(pb)) => Some((c.add(&pa, &pb), pa, pb)),
                _ => None,
            }
        }));
        match r {
            Ok(Some((s, pa, pb))) => {
                tested.push(x);
                if !sums.contains(&s) {
                    sums.push(s);
                }
                phis.push((x, pa));
                phis.push((x, pb));
            }
            Ok(None) => fails.push(x),
            Err(_) => panics.push(x),
        }
    }
    let mut neg_sums: Vec<PicClass> = Vec::new();
    for w in phis.windows(3) {
        if w[0].0 != w[2].0 {
            let s = c.add(&w[0].1, &w[2].1);
            if !neg_sums.contains(&s) {
                neg_sums.push(s);
            }
        }
    }
    (
        tested,
        panics,
        fails,
        sums.len(),
        neg_sums.len(),
        phis.len().saturating_sub(2),
    )
}

fn pushfwd(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: usize = arg(a, "--want").unwrap().parse().unwrap();
    let nbase: usize = arg(a, "--bases").map_or(6, |v| v.parse().unwrap());
    let k = F2::new(p);
    for (idx, (f, t)) in positives(p, want, seed).into_iter().enumerate() {
        let c0 = match build_quartic(&t, &k) {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "{}",
                    json!({"cell": "pushfwd2", "p": p, "seed": seed, "instance": idx, "f": f, "quartic": e})
                );
                continue;
            }
        };
        let coef: [u32; 15] = *c0.coefficients();
        let bases: Vec<Pt> = c0
            .rational_points_bruteforce()
            .into_iter()
            .filter(|b| b[2] == 1)
            .take(nbase)
            .collect();
        let mut per_base = Vec::new();
        let mut union_tested: std::collections::BTreeSet<u64> = Default::default();
        let mut universe: std::collections::BTreeSet<u64> = Default::default();
        let mut base_ok = true;
        for b in &bases {
            let Ok(cb) = PlaneQuartic::new(p as u32, coef, *b) else {
                continue;
            };
            let (tested, panics, fails, nsums, nneg, negpairs) = run_base(&t, k, &cb);
            universe.extend(tested.iter().chain(panics.iter()).chain(fails.iter()));
            union_tested.extend(tested.iter());
            if nsums > 1 {
                base_ok = false;
            }
            per_base.push(json!({"base": b, "tested": tested.len(), "panics": panics.len(), "no_phi": fails.len(),
                                  "distinct_sums": nsums, "negctl_distinct_sums": nneg, "negctl_pairs": negpairs}));
        }
        let untested_all: Vec<u64> = universe.difference(&union_tested).copied().collect();
        println!(
            "{}",
            json!({"cell": "pushfwd2", "p": p, "seed": seed, "instance": idx, "f": f, "quartic_f": coef.to_vec(),
                   "bases": per_base, "pairs_total": universe.len(), "pairs_tested_under_some_base": union_tested.len(),
                   "pairs_untested_under_all_bases": untested_all.len(), "single_valued_under_every_base": base_ok})
        );
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    pushfwd(&a);
}
