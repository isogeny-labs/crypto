//! `smith_walk_dlp`: local development checks for the Smith walk to a plane quartic.
//! Study: research/isogeny-walk-quartic_20261115/ (PLAN.md frozen before any run).
//! All figures `[pcc:unverified]`.
//!
//! ```text
//! smith_walk_dlp onestep  --p P --seed S --n N          # Smith full_success rate
//! smith_walk_dlp coverage --p P                         # exhaustive class-level ceilings
//! smith_walk_dlp modeltest --p P --seed S --want N      # does X project to a plane quartic?
//! smith_walk_dlp canon     --p P --seed S --want N      # canonical map via trace of H differentials
//! ```

#[path = "../../research/isogeny-walk-quartic_20261115/src/planefit.rs"]
mod planefit;

use crypto_lib::cryptanalysis::plane_quartic_pic::{mons_of_degree, PicClass, PlaneQuartic, Pt};
use crypto_lib::cryptanalysis::smith_kernel_coverage::{chebotarev_report, prime_report};
use crypto_lib::cryptanalysis::smith_prop24::{analyze_curve, peval, random_row, SplitMix64};
use crypto_lib::cryptanalysis::smith_trigonal::{Fp2, Trigonal, F2};
use crypto_lib::prime_hyperelliptic::ops as hops;
use crypto_lib::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};
use num_bigint::BigUint;
use serde_json::json;
use std::collections::BTreeMap;

fn arg(a: &[String], k: &str) -> Option<String> {
    a.iter()
        .position(|x| x == k)
        .and_then(|i| a.get(i + 1))
        .cloned()
}

fn wilson(k: u64, n: u64) -> (f64, f64) {
    let z = 1.959_963_984_540_054;
    let (kf, nf) = (k as f64, n as f64);
    let ph = kf / nf;
    let den = 1.0 + z * z / nf;
    let c = (ph + z * z / (2.0 * nf)) / den;
    let h = z * (ph * (1.0 - ph) / nf + z * z / (4.0 * nf * nf)).sqrt() / den;
    (c - h, c + h)
}

fn onestep(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let n: u64 = arg(a, "--n").unwrap().parse().unwrap();
    let mut ok = 0u64;
    let mut tractable = 0u64;
    let mut trig = 0u64;
    let mut by_ct: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for i in 0..n {
        let r = random_row(p, seed, i);
        ok += r.full_success as u64;
        tractable += (r.n_tractable > 0) as u64;
        trig += (r.n_trig > 0) as u64;
        let e = by_ct.entry(r.cycle_type.clone()).or_default();
        e.0 += 1;
        e.1 += r.full_success as u64;
    }
    let (lo, hi) = wilson(ok, n);
    let ct: BTreeMap<_, _> = by_ct
        .into_iter()
        .map(|(k, v)| (k, json!({"n": v.0, "full_success": v.1})))
        .collect();
    println!(
        "{}",
        json!({"cell": "onestep", "p": p, "seed": seed, "n": n, "full_success": ok, "rate": ok as f64 / n as f64,
               "wilson95": [lo, hi], "with_tractable_subgroup": tractable, "prop2_pass": trig, "by_cycle_type": ct})
    );
}

fn coverage(a: &[String]) {
    let p: u32 = arg(a, "--p").unwrap().parse().unwrap();
    let cheb = chebotarev_report();
    let r = prime_report(p, &cheb);
    let c = r.curves as f64;
    println!(
        "{}",
        json!({"cell": "coverage", "p": p, "curves": r.curves, "classes": r.classes,
               "direct_pair_kernel": r.direct_pair_kernel_curves, "direct_any_lagrangian": r.direct_any_lagrangian_curves,
               "class_pair_kernel": r.class_pair_kernel_curves, "class_any_lagrangian": r.class_any_lagrangian_curves,
               "necessary_condition": r.necessary_condition_curves,
               "frac": {"direct_pair": r.direct_pair_kernel_curves as f64 / c, "direct_any": r.direct_any_lagrangian_curves as f64 / c,
                        "class_pair": r.class_pair_kernel_curves as f64 / c, "class_any": r.class_any_lagrangian_curves as f64 / c},
               "charpoly_mismatches": r.charpoly_mismatches, "necessary_condition_violations": r.necessary_condition_violations})
    );
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

fn modeltest(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: usize = arg(a, "--want").unwrap().parse().unwrap();
    let k = F2::new(p);
    for (idx, (f, t)) in positives(p, want, seed).into_iter().enumerate() {
        let mut pts = Vec::new();
        let (mut seen, mut errs, mut rat_pairs, mut pairs) =
            (std::collections::HashSet::new(), 0u64, 0u64, 0u64);
        for (x, y) in t.points_over_u() {
            match t.image_points(x, y) {
                Ok(ip) => {
                    pairs += 1;
                    rat_pairs += ip.rational as u64;
                    for q in ip.points {
                        if seen.insert(q.mumford) {
                            let ib2 = {
                                // 1 / b2 in F_{p^2}
                                k.inv(q.mumford[2])
                            };
                            pts.push((k.mul(q.mumford[0], ib2), k.mul(q.mumford[1], ib2)));
                        }
                    }
                }
                Err(_) => errs += 1,
            }
        }
        let dims: Vec<usize> = (2..=12).map(|d| planefit::kernel_dim(&pts, d, p)).collect();
        let min_d = (2..=12)
            .zip(dims.iter())
            .find(|(_, &z)| z > 0)
            .map(|(d, _)| d);
        println!(
            "{}",
            json!({"cell": "modeltest", "p": p, "seed": seed, "instance": idx, "f": f, "n_points": pts.len(),
                   "h_points_over_u": pairs + errs, "image_errors": errs, "rational_pairs": rat_pairs, "pairs": pairs,
                   "kernel_dim_d2_to_d12": dims, "min_degree": min_d,
                   "monomials_d4": planefit::monomials(4).len()})
        );
    }
}

type Cub = [Fp2; 3];

/// Multiply a polynomial of degree < 3 by `x` modulo the monic cubic `x^3 + g2 x^2 + g1 x + g0`.
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

fn canon(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: usize = arg(a, "--want").unwrap().parse().unwrap();
    let k = F2::new(p);
    for (idx, (f, t)) in positives(p, want, seed).into_iter().enumerate() {
        let (pts, errs, skipped) = canon_points(&t, &k);
        let dims: Vec<usize> = (2..=6).map(|d| planefit::kernel_dim(&pts, d, p)).collect();
        println!(
            "{}",
            json!({"cell": "canon", "p": p, "seed": seed, "instance": idx, "f": f, "n_points": pts.len(),
                   "image_errors": errs, "skipped": skipped, "kernel_dim_d2_to_d6": dims,
                   "monomials_d4": planefit::monomials(4).len()})
        );
    }
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

fn pushfwd(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: usize = arg(a, "--want").unwrap().parse().unwrap();
    let k = F2::new(p);
    for (idx, (f, t)) in positives(p, want, seed).into_iter().enumerate() {
        let c = match build_quartic(&t, &k) {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "{}",
                    json!({"cell": "pushfwd", "p": p, "seed": seed, "instance": idx, "f": f, "quartic": e})
                );
                continue;
            }
        };
        let ratpts = c.rational_points_bruteforce();
        let ctx = Pushfwd { c: &c, k, ratpts };
        // principal-divisor test: Phi(P) + Phi(iota P) is independent of the x-coordinate
        let mut by_x: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
        for (x, y) in t.points_over_u() {
            by_x.entry(x).or_default().push((x, y));
        }
        let (mut sums, mut fails, mut tested, mut panics) = (Vec::new(), 0u64, 0u64, 0u64);
        let mut phis: Vec<(u64, u64, PicClass)> = Vec::new();
        for (x, v) in by_x {
            if v.len() != 2 {
                continue;
            }
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let (pa, pb) = (ctx.phi(&t, v[0].0, v[0].1), ctx.phi(&t, v[1].0, v[1].1));
                match (pa, pb) {
                    (Some(pa), Some(pb)) => Some((c.add(&pa, &pb), pa, pb)),
                    _ => None,
                }
            }));
            match r {
                Ok(Some((s, pa, pb))) => {
                    tested += 1;
                    if !sums.contains(&s) {
                        sums.push(s);
                    }
                    phis.push((x, v[0].1, pa));
                    phis.push((x, v[1].1, pb));
                }
                Ok(None) => fails += 1,
                Err(_) => panics += 1,
            }
        }
        // negative control: Phi(P) + Phi(P') for P, P' with different x (not related by iota)
        let mut neg_sums: Vec<PicClass> = Vec::new();
        for w in phis.windows(3) {
            if w[0].0 != w[2].0 {
                let s = c.add(&w[0].2, &w[2].2);
                if !neg_sums.contains(&s) {
                    neg_sums.push(s);
                }
            }
        }
        println!(
            "{}",
            json!({"cell": "pushfwd", "p": p, "seed": seed, "instance": idx, "f": f,
                   "quartic_f": c.coefficients().to_vec(), "n_rational_points_C": ctx.ratpts.len(),
                   "pairs_tested": tested, "phi_failures": fails, "cantor_panics": panics, "distinct_phiP_plus_phiiotaP": sums.len(), "negctl_distinct_sums": neg_sums.len(), "negctl_pairs": phis.len().saturating_sub(2)})
        );
    }
}

fn bsgs_order(c: &PlaneQuartic, g: &PicClass, b: u64) -> Option<(u64, u64)> {
    // returns (order, hash probes) for an element of order < b*b
    let mut table: std::collections::HashMap<
        crypto_lib::cryptanalysis::plane_quartic_pic::PicKey,
        u64,
    > = std::collections::HashMap::new();
    let mut cur = c.zero();
    let mut probes = 0u64;
    for j in 0..b {
        if j > 0 && c.is_zero(&cur) {
            return Some((j, probes));
        }
        table.insert(cur.key(), j);
        probes += 1;
        cur = c.add(&cur, g);
    }
    // cur = b*G
    let step = cur;
    let mut y = step.clone();
    for i in 1..=b {
        probes += 1;
        if let Some(&j) = table.get(&y.key()) {
            return Some((i * b - j, probes));
        }
        y = c.add(&y, &step);
    }
    None
}

/// Solve m G = Q by BSGS given the order of G. Returns (m, probes).
fn bsgs_dlog(c: &PlaneQuartic, g: &PicClass, q: &PicClass, ord: u64) -> Option<(u64, u64)> {
    let b = (ord as f64).sqrt().ceil() as u64 + 1;
    let mut table: std::collections::HashMap<
        crypto_lib::cryptanalysis::plane_quartic_pic::PicKey,
        u64,
    > = std::collections::HashMap::new();
    let mut cur = c.zero();
    let mut probes = 0u64;
    for j in 0..b {
        table.entry(cur.key()).or_insert(j);
        probes += 1;
        cur = c.add(&cur, g);
    }
    let giant = c.neg(&cur);
    let mut y = q.clone();
    for i in 0..=b {
        probes += 1;
        if let Some(&j) = table.get(&y.key()) {
            return Some(((i * b + j) % ord, probes));
        }
        y = c.add(&y, &giant);
    }
    None
}

fn dlp(a: &[String]) {
    let p: u64 = arg(a, "--p").unwrap().parse().unwrap();
    let seed: u64 = arg(a, "--seed").unwrap().parse().unwrap();
    let want: usize = arg(a, "--want").unwrap().parse().unwrap();
    let ntarget: usize = arg(a, "--targets").map(|x| x.parse().unwrap()).unwrap_or(6);
    let k = F2::new(p);
    let pb = BigUint::from(p);
    for (idx, (f, t)) in positives_lc(p, want, seed, true).into_iter().enumerate() {
        let Ok(c) = build_quartic(&t, &k) else {
            println!(
                "{}",
                json!({"cell":"dlp","p":p,"seed":seed,"instance":idx,"f":f,"status":"no canonical quartic"})
            );
            continue;
        };
        let ratpts = c.rational_points_bruteforce();
        let ctx = Pushfwd { c: &c, k, ratpts };
        let fp = FpPoly::from_coeffs(f.iter().map(|&x| BigUint::from(x)).collect(), pb.clone());
        let hc = HyperellipticCurveP::new(pb.clone(), fp, 3);
        let mut pts: Vec<(u64, u64, PicClass)> = Vec::new();
        for (x, y) in t.points_over_u() {
            if pts.len() >= ntarget * 2 + 4 {
                break;
            }
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ctx.phi(&t, x, y)));
            if let Ok(Some(ph)) = r {
                pts.push((x, y, ph));
            }
        }
        if pts.len() < 6 {
            println!(
                "{}",
                json!({"cell":"dlp","p":p,"seed":seed,"instance":idx,"f":f,"status":"too few pushable points"})
            );
            continue;
        }
        let hpt = |i: usize| {
            MumfordDivisorP::from_point(&hc, &BigUint::from(pts[i].0), &BigUint::from(pts[i].1))
                .unwrap()
        };
        let hdiff = |i: usize, j: usize| hpt(i).add_fast(&hpt(j).neg_fast(&hc), &hc);
        let cdiff = |i: usize, j: usize| c.sub(&pts[i].2, &pts[j].2);
        // Cantor cost per op on H
        let h0 = hops::snapshot().total();
        let g_h = hdiff(0, 1);
        let mut acc = g_h.clone();
        for _ in 0..200 {
            acc = acc.add_fast(&g_h, &hc);
        }
        let hcost = hops::snapshot().total().since(&h0);
        let cantor_field_ops_per_add = hcost.field_ops() as f64 / 201.0;
        let g_c = cdiff(0, 1);
        let bound = ((p as f64).sqrt() + 1.0).powi(6);
        let b = bound.sqrt().ceil() as u64 + 1;
        let c0 = c.counter();
        let Some((ord_c, probes_o)) = bsgs_order(&c, &g_c, b) else {
            println!(
                "{}",
                json!({"cell":"dlp","p":p,"seed":seed,"instance":idx,"f":f,"status":"order not found"})
            );
            continue;
        };
        let order_cost = c.counter().since(&c0);
        // order of G_H: smallest e in 1..=8 with ord_c * e * G_H = 0
        let mut ord_h = 0u64;
        for e in 1..=8u64 {
            if g_h
                .scalar_mul_fast(&BigUint::from(ord_c * e), &hc)
                .is_identity()
            {
                ord_h = ord_c * e;
                break;
            }
        }
        let mut diff_in_8tors = 0u64;
        let (mut in_c, mut verified, mut not_in_h, mut cost_mul, mut cost_inv, mut probes_t) =
            (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        let mut tried = 0u64;
        let mut i = 2;
        while i + 1 < pts.len() && tried < ntarget as u64 {
            let q_c = cdiff(i, i + 1);
            let q_h = hdiff(i, i + 1);
            i += 2;
            tried += 1;
            let c1 = c.counter();
            let r = bsgs_dlog(&c, &g_c, &q_c, ord_c);
            let d = c.counter().since(&c1);
            cost_mul += d.mul;
            cost_inv += d.inv;
            if let Some((m, pr)) = r {
                probes_t += pr;
                in_c += 1;
                let ok = (0..8u64)
                    .any(|j| g_h.scalar_mul_fast(&BigUint::from(m + j * ord_c), &hc) == q_h);
                if ok {
                    verified += 1;
                } else {
                    not_in_h += 1;
                    // Q_H - m G_H must lie in ker(phi), which has order dividing 8
                    let diff = q_h.add_fast(
                        &g_h.scalar_mul_fast(&BigUint::from(m), &hc).neg_fast(&hc),
                        &hc,
                    );
                    if diff
                        .scalar_mul_fast(&BigUint::from(8u64), &hc)
                        .is_identity()
                    {
                        diff_in_8tors += 1;
                    }
                }
            }
        }
        println!(
            "{}",
            json!({"cell":"dlp","p":p,"seed":seed,"instance":idx,"f":f,"status":"ok","ord_GC":ord_c,"ord_GH":ord_h,
                   "targets":tried,"logs_found_on_C":in_c,"pullback_verified_on_H":verified,"found_on_C_but_not_verified_on_H":not_in_h,"unverified_with_difference_in_8_torsion":diff_in_8tors,
                   "order_phase_fq":{"mul":order_cost.mul,"inv":order_cost.inv,"probes":probes_o},
                   "dlog_phase_fq_total":{"mul":cost_mul,"inv":cost_inv,"probes":probes_t},
                   "cantor_field_ops_per_add_H":cantor_field_ops_per_add,
                   "hasse_weil_bound":bound,"baby_giant_B":b})
        );
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("onestep") => onestep(&a),
        Some("coverage") => coverage(&a),
        Some("modeltest") => modeltest(&a),
        Some("canon") => canon(&a),
        Some("pushfwd") => pushfwd(&a),
        Some("dlp") => dlp(&a),
        _ => {
            eprintln!("usage: smith_walk_dlp onestep|coverage|modeltest ...");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mulmod_matches_x_cubed_reduction() {
        let p = 101u64;
        let k = F2::new(p);
        let g = [5u64, 7, 11];
        let x = [Fp2::default(), k.c(1), Fp2::default()];
        let x2 = mulmod(&k, x, x, g);
        let x3 = mulmod(&k, x2, x, g);
        // x^3 = -(g2 x^2 + g1 x + g0)
        assert_eq!(x3, [k.c(p - 5), k.c(p - 7), k.c(p - 11)]);
        assert_eq!(x3, mulx(&k, x2, g));
    }

    #[test]
    fn canon_image_lies_on_a_unique_plane_quartic() {
        let p = 101u64;
        let k = F2::new(p);
        let (_, t) = positives(p, 1, 1).remove(0);
        let (pts, _, _) = canon_points(&t, &k);
        assert!(pts.len() > 40);
        assert_eq!(planefit::kernel_dim(&pts, 3, p), 0);
        assert_eq!(planefit::kernel_dim(&pts, 4, p), 1);
    }
}
