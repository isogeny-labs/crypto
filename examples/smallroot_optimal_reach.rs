//! Strategy-independent bound on the Coppersmith reach of a windowed
//! Semaev block (lead LEAD-ICP-coppersmith-smallroot-semaev, item 6 of the
//! 2026-10-26 triage).
//!
//! Setting.  `f` is a polynomial over `F_p` in `n` small unknowns
//! (`|x_i| < p^{δ_i}`), here a summation polynomial `S_3` with three
//! unknowns (or two, with the third known).  Every Coppersmith lattice built
//! from shift polynomials of `f` modulo `p^t`, whatever the strategy (basic or
//! extended Jochemsz–May, Ryan's provably optimal Gröbner shifts), is a
//! sublattice of
//!
//! ```text
//!   L(M) = { g in Z[x] : supp g ⊆ M,  g ∈ (p, f)^t + p^t Z[x] },
//! ```
//!
//! for its monomial set `M`.  Write `E(M) = log_p [L(M) : p^t Z^M]`.  The
//! Howgrave-Graham condition `det L < p^{t·dim L}` then reads
//! `Σ_{μ∈M} <δ, μ> < E(M)` (dimension factors dropped, which favours the
//! attack), and the reach is `ε = Σ_i δ_i`.
//!
//! Two facts make `E(M)` computable without choosing a strategy:
//!
//! 1. *Layers.* `p, f` is a regular sequence in `Z_p[x]`, so
//!    `((p,f)^t : p^i) = (p,f)^{t−i}` and layer `i` of `L(M)` reduces mod `p`
//!    into `f̄^{t−i}·F_p[x] ∩ span(M)`.  Hence
//!    `E(M) ≤ Σ_{s=1}^{t} dim( f̄^s F_p[x] ∩ span M )`.
//! 2. *Ostrowski.* `Newt(h·f^s) = Newt(h) + s·Newt(f)`, so for `M` the lattice
//!    points of a convex region `R`, `supp(h f^s) ⊆ M` forces
//!    `supp(h) ⊆ R ⊖ s·N`.  Hence `dim(...) ≤ #(R ⊖ sN) ∩ Z^n`, the
//!    *translate count*.
//!
//! So `ε` is bounded by a geometric optimisation over regions `R`, and by
//! Brunn–Minkowski plus the bathtub principle, for equal bounds,
//! `ε ≤ (n!·vol N)^{−1/n}`.
//!
//! This example (i) computes `E(M)` exactly, by elimination over the local
//! ring `Z/p^t`, on small boxes, and checks it against the translate count;
//! (ii) maximises the translate-count reach over polytope regions and bound
//! directions; (iii) prints the Brunn–Minkowski bound.  Calibrations: linear
//! `f` in two unknowns (reach 1, simplex region), univariate degree `d`
//! (reach `1/d`), the Jochemsz–May full box `2/(d(n+1))`.
//!
//! `cargo run --release --example smallroot_optimal_reach -- [exact|optimize|bm|all]`
//!
//! Output: JSON lines on stdout.  Nothing here is a measurement of an attack;
//! it is an exact computation and a numerical optimisation of a bound.

use std::collections::HashMap;

type Exp = [i32; 3];

// ---------------------------------------------------------------- RNG ----

struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn normal(&mut self) -> f64 {
        let (u, v) = (self.unit().max(1e-300), self.unit());
        (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
    }
}

// ------------------------------------------------------- polynomials ----

#[derive(Clone)]
struct Poly {
    n: usize,
    terms: HashMap<Exp, u128>,
}

fn mulmod(a: u128, b: u128, m: u128) -> u128 {
    // m < 2^62, so a, b < 2^62 and the product fits in u128.
    (a * b) % m
}

impl Poly {
    fn one(n: usize) -> Self {
        let mut terms = HashMap::new();
        terms.insert([0; 3], 1);
        Poly { n, terms }
    }
    fn mul(&self, o: &Poly, m: u128) -> Poly {
        let mut terms: HashMap<Exp, u128> = HashMap::new();
        for (ea, &ca) in &self.terms {
            for (eb, &cb) in &o.terms {
                let e = [ea[0] + eb[0], ea[1] + eb[1], ea[2] + eb[2]];
                let c = terms.entry(e).or_insert(0);
                *c = (*c + mulmod(ca, cb, m)) % m;
            }
        }
        terms.retain(|_, c| *c != 0);
        Poly { n: self.n, terms }
    }
    fn support(&self) -> Vec<Exp> {
        let mut s: Vec<Exp> = self.terms.keys().copied().collect();
        s.sort();
        s
    }
}

fn signed(c: i128, p: u128) -> u128 {
    c.rem_euclid(p as i128) as u128
}

/// `S_3(x1, x2, x3)` for `y² = x³ + a x + b`, coefficients reduced mod `p`.
/// With `n = 2` the third abscissa is fixed to a random value.
fn semaev_s3(n: usize, p: u128, rng: &mut SplitMix) -> Poly {
    let a = rng.below(p as u64) as i128;
    let b = rng.below(p as u64) as i128;
    let x3 = rng.below(p as u64) as i128;
    let t: [([i32; 3], i128); 13] = [
        ([2, 0, 2], 1),
        ([1, 1, 2], -2),
        ([0, 2, 2], 1),
        ([2, 1, 1], -2),
        ([1, 2, 1], -2),
        ([1, 0, 1], -2 * a),
        ([0, 1, 1], -2 * a),
        ([0, 0, 1], -4 * b),
        ([2, 2, 0], 1),
        ([1, 1, 0], -2 * a),
        ([0, 0, 0], a * a),
        ([1, 0, 0], -4 * b),
        ([0, 1, 0], -4 * b),
    ];
    let mut terms: HashMap<Exp, u128> = HashMap::new();
    for (e, c) in t {
        let (key, coef) = if n == 3 {
            (e, signed(c, p))
        } else {
            let mut x = 1u128;
            for _ in 0..e[2] {
                x = mulmod(x, signed(x3, p), p);
            }
            ([e[0], e[1], 0], mulmod(signed(c, p), x, p))
        };
        let v = terms.entry(key).or_insert(0);
        *v = (*v + coef) % p;
    }
    terms.retain(|_, c| *c != 0);
    Poly { n, terms }
}

fn linear(n: usize, p: u128, rng: &mut SplitMix) -> Poly {
    let mut terms = HashMap::new();
    terms.insert([0; 3], 1 + rng.below(p as u64 - 1) as u128);
    for i in 0..n {
        let mut e = [0; 3];
        e[i] = 1;
        terms.insert(e, 1 + rng.below(p as u64 - 1) as u128);
    }
    Poly { n, terms }
}

fn univariate(d: i32, p: u128, rng: &mut SplitMix) -> Poly {
    let mut terms = HashMap::new();
    for k in 0..=d {
        terms.insert([k, 0, 0], 1 + rng.below(p as u64 - 1) as u128);
    }
    Poly { n: 1, terms }
}

fn boxed(n: usize, side: i32) -> Vec<Exp> {
    let mut out = Vec::new();
    let r = |i: usize| if i < n { side } else { 0 };
    for a in 0..=r(0) {
        for b in 0..=r(1) {
            for c in 0..=r(2) {
                out.push([a, b, c]);
            }
        }
    }
    out
}

// ------------------------------------------- exact gain over Z/p^t ------

fn inv_mod(a: u128, m: u128) -> u128 {
    let (mut r0, mut r1) = (m as i128, a as i128);
    let (mut s0, mut s1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    assert_eq!(r0, 1, "unit expected");
    s0.rem_euclid(m as i128) as u128
}

fn val(mut x: u128, p: u128, t: u32) -> u32 {
    if x == 0 {
        return t;
    }
    let mut v = 0;
    while x.is_multiple_of(p) {
        x /= p;
        v += 1;
    }
    v
}

/// `E(M)` for `L(M) = ((p,f)^t + p^t) ∩ span M`, using shifts inside the
/// superset `big` (outside-`M` columns are eliminated first).
fn exact_gain(f: &Poly, t: u32, m: &[Exp], big: &[Exp], p: u128) -> u32 {
    let modulus = p.pow(t);
    let in_m: std::collections::HashSet<Exp> = m.iter().copied().collect();
    let mut order: Vec<Exp> = big.iter().filter(|e| !in_m.contains(*e)).copied().collect();
    let n_out = order.len();
    order.extend(big.iter().filter(|e| in_m.contains(*e)).copied());
    let col: HashMap<Exp, usize> = order.iter().enumerate().map(|(i, e)| (*e, i)).collect();
    let mut powers = vec![Poly::one(f.n)];
    for k in 0..t {
        let next = powers[k as usize].mul(f, modulus);
        powers.push(next);
    }
    let mut rows: Vec<Vec<u128>> = Vec::new();
    for k in 0..=t {
        let scale = p.pow(t - k) % modulus;
        for a in big {
            let mut row = vec![0u128; order.len()];
            let mut ok = true;
            for (e, &c) in &powers[k as usize].terms {
                let s = [a[0] + e[0], a[1] + e[1], a[2] + e[2]];
                match col.get(&s) {
                    Some(&j) => row[j] = mulmod(c, scale, modulus),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if ok && row.iter().any(|&x| x != 0) {
                rows.push(row);
            }
        }
    }
    let mut gain = 0;
    let mut start = 0;
    for c in 0..order.len() {
        let mut best: Option<(usize, u32)> = None;
        for (r, row) in rows.iter().enumerate().skip(start) {
            let v = val(row[c], p, t);
            if v < t && best.is_none_or(|(_, bv)| v < bv) {
                best = Some((r, v));
            }
        }
        let Some((r, v)) = best else { continue };
        rows.swap(start, r);
        let piv = rows[start][c];
        let unit = inv_mod(piv / p.pow(v), modulus);
        let pivot_row = rows[start].clone();
        for row in rows.iter_mut().skip(start + 1) {
            let x = row[c];
            if x == 0 {
                continue;
            }
            let q = mulmod(x / p.pow(v), unit, modulus);
            for j in c..row.len() {
                if pivot_row[j] != 0 {
                    row[j] = (row[j] + modulus - mulmod(q, pivot_row[j], modulus)) % modulus;
                }
            }
        }
        if c >= n_out {
            gain += t - v;
        }
        start += 1;
    }
    gain
}

/// `Σ_{s=1}^{t} #{a ∈ M : a + s·supp f ⊆ M}` for a convex `M`.
fn translate_count(f: &Poly, t: u32, m: &[Exp]) -> u32 {
    let set: std::collections::HashSet<Exp> = m.iter().copied().collect();
    let sup = f.support();
    let mut total = 0;
    for s in 1..=t as i32 {
        for a in m {
            if sup
                .iter()
                .all(|e| set.contains(&[a[0] + s * e[0], a[1] + s * e[1], a[2] + s * e[2]]))
            {
                total += 1;
            }
        }
    }
    total
}

fn run_exact() {
    let p: u128 = 1_000_003;
    let mut rng = SplitMix(20261026);
    let cases: &[(&str, usize, u32, i32, i32)] = &[
        ("linear", 2, 2, 4, 7),
        ("linear", 2, 3, 6, 9),
        ("univariate-d2", 1, 3, 6, 12),
        ("s3", 2, 1, 4, 8),
        ("s3", 2, 2, 4, 8),
        ("s3", 2, 2, 6, 10),
        ("s3", 2, 3, 6, 10),
        ("s3", 2, 3, 8, 12),
        ("s3", 3, 1, 3, 5),
        ("s3", 3, 2, 4, 6),
        ("s3", 3, 2, 5, 7),
    ];
    for &(name, n, t, side, sup) in cases {
        assert!((p.pow(t)) < (1u128 << 62));
        let f = match name {
            "linear" => linear(n, p, &mut rng),
            "univariate-d2" => univariate(2, p, &mut rng),
            _ => semaev_s3(n, p, &mut rng),
        };
        let m = boxed(n, side);
        let big = boxed(n, sup);
        let e = exact_gain(&f, t, &m, &big, p);
        let tc = translate_count(&f, t, &m);
        let den: i64 = m.iter().map(|e| (e[0] + e[1] + e[2]) as i64).sum();
        println!(
            "{{\"mode\":\"exact\",\"f\":\"{name}\",\"n\":{n},\"t\":{t},\"box\":{side},\"superset\":{sup},\"p\":{p},\
             \"E_exact\":{e},\"E_translate\":{tc},\"bound_holds\":{},\"reach_exact_equal_bounds\":{:.6}}}",
            e <= tc,
            n as f64 * e as f64 / den as f64
        );
    }
}

// --------------------------------------- region optimisation (bound) ----

/// Support of the polynomial whose Newton polytope `N` is used.
fn support_of(name: &str, n: usize) -> Vec<[f64; 3]> {
    let mut rng = SplitMix(1);
    let f = match name {
        "linear" => linear(n, 1_000_003, &mut rng),
        "univariate-d2" => univariate(2, 1_000_003, &mut rng),
        _ => semaev_s3(n, 1_000_003, &mut rng),
    };
    f.support()
        .iter()
        .map(|e| [e[0] as f64, e[1] as f64, e[2] as f64])
        .collect()
}

/// Reach of region `R = { x ≥ 0 : a_j·x ≤ 1 }` (scaled by `scale`, lattice
/// points) with bound direction `w` (`Σ w = 1`), via the translate count.
fn region_reach(n: usize, sup: &[[f64; 3]], a: &[[f64; 3]], w: &[f64; 3], scale: f64) -> f64 {
    let dot = |u: &[f64; 3], v: &[f64; 3]| u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
    let h: Vec<f64> = a
        .iter()
        .map(|r| sup.iter().map(|v| dot(r, v)).fold(0.0, f64::max))
        .collect();
    // `scale` is a lattice-point budget: the region is dilated so that its
    // bounding box holds about that many points (the ratio is scale-free).
    let mut mx = [1.0f64; 3];
    for (i, m) in mx.iter_mut().enumerate().take(n) {
        *m = a.iter().map(|r| r[i]).fold(0.0, f64::max).max(1e-12);
    }
    let budget = scale;
    let boxsize = |s: f64| -> f64 { (0..n).map(|i| (s / mx[i]).floor() + 1.0).product() };
    let (mut lo, mut hi) = (0.0f64, 1.0f64);
    while boxsize(hi) <= budget {
        hi *= 2.0;
    }
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if boxsize(mid) <= budget {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let scale = lo;
    let mut ub = [0i64; 3];
    for i in 0..n {
        ub[i] = (scale / mx[i]).floor() as i64;
    }
    let (mut e, mut den) = (0.0f64, 0.0f64);
    for x0 in 0..=ub[0] {
        for x1 in 0..=ub[1] {
            for x2 in 0..=ub[2] {
                let x = [x0 as f64, x1 as f64, x2 as f64];
                let mut phi = f64::INFINITY;
                let mut inside = true;
                for (r, &hr) in a.iter().zip(&h) {
                    let slack = scale - dot(r, &x);
                    if slack < 0.0 {
                        inside = false;
                        break;
                    }
                    if hr > 0.0 {
                        phi = phi.min((slack / hr).floor());
                    }
                }
                if inside {
                    e += phi;
                    den += dot(w, &x);
                }
            }
        }
    }
    if den <= 0.0 {
        return 0.0;
    }
    e / den
}

fn unpack(n: usize, k: usize, th: &[f64]) -> (Vec<[f64; 3]>, [f64; 3]) {
    let mut a = Vec::with_capacity(k);
    for j in 0..k {
        let mut r = [0.0; 3];
        for i in 0..n {
            r[i] = th[j * n + i].exp();
        }
        a.push(r);
    }
    let mut w = [0.0; 3];
    let mut s = 0.0;
    for i in 0..n {
        w[i] = if i == 0 { 1.0 } else { th[k * n + i - 1].exp() };
        s += w[i];
    }
    for x in w.iter_mut().take(n) {
        *x /= s;
    }
    (a, w)
}

fn run_optimize(
    name: &str,
    n: usize,
    k: usize,
    seed: u64,
    restarts: usize,
    iters: usize,
    scale: f64,
) {
    let sup = support_of(name, n);
    let mut rng = SplitMix(seed);
    let dim = k * n + n - 1;
    let score = |th: &[f64]| {
        let (a, w) = unpack(n, k, th);
        region_reach(n, &sup, &a, &w, scale)
    };
    let mut best_th = vec![0.0; dim];
    let mut best = f64::MIN;
    for _ in 0..restarts {
        let mut th: Vec<f64> = (0..dim).map(|_| rng.normal()).collect();
        let mut cur = score(&th);
        let mut step = 1.0;
        for it in 0..iters {
            let mut cand = th.clone();
            for x in cand.iter_mut() {
                *x += step * rng.normal();
            }
            let s = score(&cand);
            if s > cur {
                th = cand;
                cur = s;
            }
            if it % 200 == 199 {
                step = (step * 0.7).max(0.02);
            }
        }
        if cur > best {
            best = cur;
            best_th = th;
        }
    }
    let (a, w) = unpack(n, k, &best_th);
    let check = region_reach(n, &sup, &a, &w, scale * 8.0);
    let fmt3 = |v: &[f64; 3]| format!("[{:.4},{:.4},{:.4}]", v[0], v[1], v[2]);
    let facets: Vec<String> = a.iter().map(fmt3).collect();
    println!(
        "{{\"mode\":\"optimize\",\"f\":\"{name}\",\"n\":{n},\"facets\":{k},\"seed\":{seed},\"scale\":{scale},\
         \"reach\":{best:.6},\"reach_at_8x_points\":{check:.6},\"bound_direction\":{},\"normals\":[{}]}}",
        fmt3(&w),
        facets.join(",")
    );
}

// --------------------------------------------- Brunn–Minkowski bound ----

/// Volume of the convex hull of `pts` (n = 2 or 3) by facet enumeration and
/// lattice counting of `k·conv(pts)`; returns `(volume, eps_bm)`.
fn bm_bound(name: &str, n: usize) -> (f64, f64) {
    let pts = support_of(name, n);
    let mut planes: Vec<([f64; 3], f64)> = Vec::new();
    let sub = |u: &[f64; 3], v: &[f64; 3]| [u[0] - v[0], u[1] - v[1], u[2] - v[2]];
    let dot = |u: &[f64; 3], v: &[f64; 3]| u[0] * v[0] + u[1] * v[1] + u[2] * v[2];
    if n == 3 {
        for i in 0..pts.len() {
            for j in i + 1..pts.len() {
                for l in j + 1..pts.len() {
                    let (u, v) = (sub(&pts[j], &pts[i]), sub(&pts[l], &pts[i]));
                    let nrm = [
                        u[1] * v[2] - u[2] * v[1],
                        u[2] * v[0] - u[0] * v[2],
                        u[0] * v[1] - u[1] * v[0],
                    ];
                    if dot(&nrm, &nrm) < 1e-12 {
                        continue;
                    }
                    let c = dot(&nrm, &pts[i]);
                    let side: Vec<f64> = pts.iter().map(|q| dot(&nrm, q) - c).collect();
                    if side.iter().all(|&s| s <= 1e-9) {
                        planes.push((nrm, c));
                    } else if side.iter().all(|&s| s >= -1e-9) {
                        planes.push(([-nrm[0], -nrm[1], -nrm[2]], -c));
                    }
                }
            }
        }
    } else {
        for i in 0..pts.len() {
            for j in i + 1..pts.len() {
                let u = sub(&pts[j], &pts[i]);
                let nrm = [u[1], -u[0], 0.0];
                if dot(&nrm, &nrm) < 1e-12 {
                    continue;
                }
                let c = dot(&nrm, &pts[i]);
                let side: Vec<f64> = pts.iter().map(|q| dot(&nrm, q) - c).collect();
                if side.iter().all(|&s| s <= 1e-9) {
                    planes.push((nrm, c));
                } else if side.iter().all(|&s| s >= -1e-9) {
                    planes.push(([-nrm[0], -nrm[1], 0.0], -c));
                }
            }
        }
    }
    // `N` is a lattice polytope, so `L(k) = #(kN ∩ Z^n)` is its Ehrhart
    // polynomial with constant term 1, and the leading coefficient (the
    // volume) is exact from finite differences: `6V = Δ³L`, `2V = Δ²L`.
    let maxc = pts
        .iter()
        .map(|p| p[0].max(p[1]).max(p[2]))
        .fold(0.0, f64::max);
    let count = |k: i64| -> i64 {
        let lim = k * maxc as i64;
        let z_lim = if n == 3 { lim } else { 0 };
        let mut c = 0;
        for x in 0..=lim {
            for y in 0..=lim {
                for z in 0..=z_lim {
                    let q = [x as f64, y as f64, z as f64];
                    if planes
                        .iter()
                        .all(|(nr, cc)| dot(nr, &q) <= k as f64 * cc + 1e-9)
                    {
                        c += 1;
                    }
                }
            }
        }
        c
    };
    let vol = if n == 3 {
        (count(3) - 3 * count(2) + 3 * count(1) - 1) as f64 / 6.0
    } else {
        (count(2) - 2 * count(1) + 1) as f64 / 2.0
    };
    let fact: f64 = (1..=n).map(|i| i as f64).product();
    (vol, (fact * vol).powf(-1.0 / n as f64))
}

fn run_bm() {
    for (name, n) in [("linear", 2), ("s3", 2), ("s3", 3)] {
        let (vol, eps) = bm_bound(name, n);
        println!("{{\"mode\":\"bm\",\"f\":\"{name}\",\"n\":{n},\"vol_newton\":{vol:.4},\"eps_bm_equal_bounds\":{eps:.6}}}");
    }
}

/// The weight-order theorem, checked exactly.  For any monomial set `M`
/// (convex or not) and any weight `w ≥ 0`, every lattice from `(p, f)^t`
/// on `M` has `E(M) ≤ Σ_{μ∈M} min(t, ⌊<w,μ>/h_N(w)⌋)`, so the
/// Howgrave-Graham condition needs `h_N(δ) < 1`.  This mode draws random
/// non-convex `M` (random subsets of a box), computes `E(M)` exactly, and
/// checks the inequality for random weights; it also reports the exact
/// scaling `λ* = E / Σ<δ̂,μ>` against `1/h_N(δ̂)` for a random direction `δ̂`.
fn run_theorem() {
    let p: u128 = 1_000_003;
    let mut rng = SplitMix(0x7EE0_2026);
    let cases: &[(&str, usize, i32, u32)] = &[
        ("linear", 2, 6, 3),
        ("univariate-d2", 1, 8, 3),
        ("s3", 2, 6, 3),
        ("s3", 3, 4, 2),
    ];
    for &(name, n, side, tmax) in cases {
        let f = match name {
            "linear" => linear(n, p, &mut rng),
            "univariate-d2" => univariate(2, p, &mut rng),
            _ => semaev_s3(n, p, &mut rng),
        };
        let sup = f.support();
        let hn = |w: &[f64; 3]| {
            sup.iter()
                .map(|e| w[0] * e[0] as f64 + w[1] * e[1] as f64 + w[2] * e[2] as f64)
                .fold(0.0, f64::max)
        };
        let (mut trials, mut violations, mut worst_ratio, mut worst_lambda) =
            (0, 0, 0.0f64, 0.0f64);
        let mut nonzero = 0;
        for _ in 0..200 {
            let t = 1 + (rng.below(tmax as u64) as u32);
            let keep = 0.45 + 0.4 * rng.unit();
            let m: Vec<Exp> = boxed(n, side)
                .into_iter()
                .filter(|_| rng.unit() < keep)
                .collect();
            if m.is_empty() {
                continue;
            }
            let big = boxed(n, side + 4);
            let e = exact_gain(&f, t, &m, &big, p);
            trials += 1;
            if e > 0 {
                nonzero += 1;
            }
            for _ in 0..12 {
                let mut w = [0.0; 3];
                for wi in w.iter_mut().take(n) {
                    *wi = rng.unit() + 1e-3;
                }
                let h = hn(&w);
                let bound: u32 = m
                    .iter()
                    .map(|mu| {
                        let x =
                            (w[0] * mu[0] as f64 + w[1] * mu[1] as f64 + w[2] * mu[2] as f64) / h;
                        ((x + 1e-9).floor() as u32).min(t)
                    })
                    .sum();
                if e > bound {
                    violations += 1;
                }
                if e > 0 {
                    worst_ratio = worst_ratio.max(e as f64 / bound.max(1) as f64);
                    let den: f64 = m
                        .iter()
                        .map(|mu| w[0] * mu[0] as f64 + w[1] * mu[1] as f64 + w[2] * mu[2] as f64)
                        .sum();
                    // λ* h_N(w) must stay ≤ 1 for the theorem to hold.
                    worst_lambda = worst_lambda.max(e as f64 / den * h);
                }
            }
        }
        println!(
            "{{\"mode\":\"theorem\",\"f\":\"{name}\",\"n\":{n},\"box\":{side},\"trials\":{trials},\"nonzero_gain\":{nonzero},\
             \"weights_per_trial\":12,\"violations\":{violations},\"max_E_over_bound\":{worst_ratio:.4},\
             \"max_lambda_times_h\":{worst_lambda:.4}}}"
        );
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    if mode == "exact" || mode == "all" {
        run_exact();
    }
    if mode == "bm" || mode == "all" {
        run_bm();
    }
    if mode == "theorem" {
        run_theorem();
    }
    if mode == "optimize" || mode == "all" {
        // calibrations: linear n = 2 (expect 1), univariate degree 2 (expect 1/2)
        run_optimize("linear", 2, 3, 11, 4, 1500, 40000.0);
        run_optimize("univariate-d2", 1, 1, 12, 2, 300, 2000.0);
        for (k, seed) in [(3, 21), (6, 22), (10, 23), (6, 24), (10, 25)] {
            run_optimize("s3", 3, k, seed, 6, 2500, 60000.0);
        }
        for (k, seed) in [(4, 31), (8, 32)] {
            run_optimize("s3", 2, k, seed, 6, 2500, 40000.0);
        }
    }
}
