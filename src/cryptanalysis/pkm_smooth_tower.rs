//! Petit–Kosters–Messeng factor bases with **mixed smooth degrees**
//! (PKC 2016, "Algebraic approaches for the ECDLP over prime fields",
//! slides 12–17), for a prime `p` that is already fixed.
//!
//! `ic_framework::prime_tower` builds the two cases the repository had so
//! far, both with every map of degree 2: the Kummer tower `x ↦ x²` and the
//! Vélu 2-isogeny tower. The paper only asks for low-degree maps `L_j`
//! whose composition `L = L_{n'} ∘ … ∘ L_1` has `#{x ∈ F_p : L(x) = 0} ≈
//! ∏ deg L_j ≈ p^{1/m}`. This module builds that for any ordered list of
//! small primes `q_1, …, q_{n'}` (repetitions allowed), `S = ∏ q_j`:
//!
//! * **Kummer form** (slide 14, "`p − 1` smooth"): when `S | p − 1`,
//!   `L_j(x) = x^{q_j}` and the last step `x^{q_{n'}} − g^S`, so
//!   `V = g·μ_S`, `|V| = S`.
//! * **Isogeny form** (slide 15): an auxiliary curve `E'/F_p` with a
//!   cyclic subgroup `G = ⟨Q⟩` of order `S`; `φ_j` the Vélu isogeny of
//!   degree `q_j` with kernel `⟨[S_j/q_j]Q_j⟩` (`Q_1 = Q`,
//!   `Q_{j+1} = φ_j(Q_j)`, `S_j = q_j ⋯ q_{n'}`), `L_j` its x-map
//!   `N_j/D_j` of degree `q_j`, and the last step taken "slightly
//!   differently": `L(x) − c` with `c = x(Φ(R'))`, so `V = x(R' + G)`.
//!   `|V| = S` when `2R' ∉ G`.
//! * **Method 1** (slide 16): pick random curves until `#E'(F_p)` has a
//!   `B`-smooth part reaching the target size with a cyclic subgroup of
//!   that order. The number of curves tried is recorded so the slide's
//!   "≈ |F| trials" can be measured instead of assumed.
//! * **Relation system** (slide 17): the polynomial system
//!   `S_{m+1}(x_{1,1}, …, x_{m,1}, X) = 0`, `x_{i,j+1} D_j(x_{i,j}) =
//!   N_j(x_{i,j})`, `0 = L_{n'}(x_{i,n'})`, exported in msolve's input
//!   format. `S_{m+1}` is written as the equivalent chain of `S_3`'s with
//!   `m − 2` auxiliary unknowns (same solutions in the `x_{i,1}`), and
//!   an exact `m = 2` relation finder over `V` checks it on small cases.
//!
//! * **Method 2** (slide 16, complex multiplication) for the given `p`:
//!   for each of the 13 class-number-1 orders in which `p` splits,
//!   Cornacchia gives `4p = t² + |D|v²`, hence every CM trace (six twists
//!   for `D = −3`, four for `D = −4`), and the curves are built from
//!   `j(D)` with each order certified on random points. A handful of
//!   candidate orders per `p`, against Method 1's random search.
//!
//! The slide's other use of Method 2, choosing `p` itself so that a CM
//! order is smooth ("kind of trapdoor"), is out of scope: every routine
//! takes `p` as given.
//!
//! Single-word arithmetic: `p < 2^62`. Order computation is a baby-step
//! giant-step over the Hasse interval and is practical to about 48 bits.
//! Nothing here is a measured result; it is construction and checking
//! code.

use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------- field

#[inline]
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
#[inline]
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
#[inline]
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
pub fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
pub fn invm(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero");
    powm(a, p - 2, p)
}
pub fn is_square(a: u64, p: u64) -> bool {
    a % p == 0 || powm(a, (p - 1) / 2, p) == 1
}
/// Tonelli–Shanks; `None` for a non-residue.
pub fn sqrtm(a: u64, p: u64) -> Option<u64> {
    let a = a % p;
    if a == 0 {
        return Some(0);
    }
    if p == 2 {
        return Some(a);
    }
    if !is_square(a, p) {
        return None;
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while is_square(z, p) {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (s, powm(z, q, p), powm(a, q, p), powm(a, q.div_ceil(2), p));
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = mulm(tt, tt, p);
            i += 1;
        }
        let b = powm(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mulm(b, b, p);
        t = mulm(t, c, p);
        r = mulm(r, b, p);
    }
    Some(r)
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powm(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulm(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// Trial division of `n` by primes `≤ bound`: `(smooth factors with
/// multiplicity, cofactor)`.
pub fn smooth_split(mut n: u64, bound: u64) -> (Vec<u64>, u64) {
    let mut f = vec![];
    let mut q = 2;
    while q <= bound && q <= n {
        while n % q == 0 {
            f.push(q);
            n /= q;
        }
        q += if q == 2 { 1 } else { 2 };
    }
    (f, n)
}

/// SplitMix64, so every search is a function of its seed.
#[derive(Clone, Debug)]
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

// ---------------------------------------------------------------- curves

/// `y² = x³ + ax + b` over `F_p`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    pub p: u64,
    pub a: u64,
    pub b: u64,
}

/// Affine point or the point at infinity (`None`).
pub type Pt = Option<(u64, u64)>;

impl Curve {
    pub fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        addm(
            addm(mulm(mulm(x, x, p), x, p), mulm(self.a, x, p), p),
            self.b,
            p,
        )
    }
    pub fn is_singular(&self) -> bool {
        let p = self.p;
        let d = addm(
            mulm(4, powm(self.a, 3, p), p),
            mulm(27, mulm(self.b, self.b, p), p),
            p,
        );
        d == 0
    }
    pub fn on_curve(&self, pt: Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => mulm(y, y, self.p) == self.rhs(x),
        }
    }
    pub fn neg(&self, pt: Pt) -> Pt {
        pt.map(|(x, y)| (x, subm(0, y, self.p)))
    }
    pub fn add(&self, u: Pt, v: Pt) -> Pt {
        let p = self.p;
        let ((x1, y1), (x2, y2)) = match (u, v) {
            (None, _) => return v,
            (_, None) => return u,
            (Some(a), Some(b)) => (a, b),
        };
        let l = if x1 == x2 {
            if addm(y1, y2, p) == 0 {
                return None;
            }
            mulm(
                addm(mulm(3, mulm(x1, x1, p), p), self.a, p),
                invm(mulm(2, y1, p), p),
                p,
            )
        } else {
            mulm(subm(y2, y1, p), invm(subm(x2, x1, p), p), p)
        };
        let x3 = subm(subm(mulm(l, l, p), x1, p), x2, p);
        let y3 = subm(mulm(l, subm(x1, x3, p), p), y1, p);
        Some((x3, y3))
    }
    pub fn mul(&self, mut k: u64, pt: Pt) -> Pt {
        let (mut r, mut b) = (None, pt);
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(r, b);
            }
            b = self.add(b, b);
            k >>= 1;
        }
        r
    }
    pub fn random_point(&self, rng: &mut Rng) -> Pt {
        loop {
            let x = rng.below(self.p);
            if let Some(y) = sqrtm(self.rhs(x), self.p) {
                let y = if rng.next() & 1 == 1 {
                    subm(0, y, self.p)
                } else {
                    y
                };
                return Some((x, y));
            }
        }
    }
    /// Every `k ∈ [lo, hi]` with `[k]P = O`, by baby-step giant-step.
    fn annihilators_in(&self, pt: Pt, lo: u64, hi: u64) -> Vec<u64> {
        let w = hi - lo + 1;
        let m = ((w as f64).sqrt() as u64).max(1) + 1;
        let mut baby: HashMap<Pt, Vec<u64>> = HashMap::new();
        let mut cur = None;
        for j in 0..m {
            baby.entry(cur).or_default().push(j);
            cur = self.add(cur, pt);
        }
        // [lo + i m + j] P = O  ⇔  [j]P = −[lo + i m]P
        let step = self.mul(m, pt);
        let mut g = self.mul(lo, pt);
        let mut out = vec![];
        let mut i = 0;
        while lo + i * m <= hi {
            if let Some(js) = baby.get(&self.neg(g)) {
                for &j in js {
                    let k = lo + i * m + j;
                    if k <= hi {
                        out.push(k);
                    }
                }
            }
            g = self.add(g, step);
            i += 1;
        }
        out.sort_unstable();
        out.dedup();
        out
    }
    /// `#E(F_p)`: exhaustive below 2^16, otherwise the Hasse-interval
    /// candidates annihilating several random points, until one is left.
    pub fn order(&self, rng: &mut Rng) -> Option<u64> {
        let p = self.p;
        if p < 1 << 16 {
            let mut n = 1;
            for x in 0..p {
                let r = self.rhs(x);
                n += if r == 0 {
                    1
                } else if is_square(r, p) {
                    2
                } else {
                    0
                };
            }
            return Some(n);
        }
        let s = 2 * (p as f64).sqrt().ceil() as u64 + 2;
        let (lo, hi) = (p + 1 - s, p + 1 + s);
        let mut cand: Option<Vec<u64>> = None;
        for _ in 0..24 {
            let pt = self.random_point(rng);
            let hits = match &cand {
                None => self.annihilators_in(pt, lo, hi),
                Some(c) => c
                    .iter()
                    .copied()
                    .filter(|&k| self.mul(k, pt).is_none())
                    .collect(),
            };
            if hits.len() == 1 {
                return Some(hits[0]);
            }
            cand = Some(hits);
        }
        None
    }
}

// ---------------------------------------------------------- polynomials

/// Univariate polynomial over `F_p`, low degree first.
pub type Poly = Vec<u64>;

pub fn poly_trim(mut a: Poly) -> Poly {
    while a.len() > 1 && *a.last().unwrap() == 0 {
        a.pop();
    }
    a
}
pub fn poly_mul(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut r = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = addm(r[i + j], mulm(x, y, p), p);
        }
    }
    poly_trim(r)
}
pub fn poly_add(a: &Poly, b: &Poly, p: u64) -> Poly {
    let mut r = vec![0; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        r[i] = x;
    }
    for (i, &y) in b.iter().enumerate() {
        r[i] = addm(r[i], y, p);
    }
    poly_trim(r)
}
pub fn poly_scale(a: &Poly, c: u64, p: u64) -> Poly {
    poly_trim(a.iter().map(|&x| mulm(x, c, p)).collect())
}
pub fn poly_eval(a: &Poly, x: u64, p: u64) -> u64 {
    a.iter()
        .rev()
        .fold(0, |acc, &c| addm(mulm(acc, x, p), c, p))
}

// ---------------------------------------------------------------- maps

/// One step `L_j = N_j / D_j` of degree `q_j`.
#[derive(Clone, Debug)]
pub struct MapStep {
    pub degree: u64,
    pub num: Poly,
    pub den: Poly,
}

impl MapStep {
    pub fn eval(&self, x: u64, p: u64) -> Option<u64> {
        let d = poly_eval(&self.den, x, p);
        (d != 0).then(|| mulm(poly_eval(&self.num, x, p), invm(d, p), p))
    }
}

/// Which construction a tower came from.
#[derive(Clone, Debug)]
pub enum TowerKind {
    /// `V = g·μ_S`, `S | p − 1`.
    Kummer { g: u64 },
    /// `V = x(R' + ⟨Q⟩)` on the auxiliary curve.
    Isogeny {
        aux: Curve,
        aux_order: u64,
        q: (u64, u64),
        r: (u64, u64),
        /// Codomains `E'_2, …, E'_{n'+1}`.
        codomains: Vec<Curve>,
        curves_tried: u64,
    },
}

/// `V = {x ∈ F_p : L(x) = c}` with `L = L_{n'} ∘ … ∘ L_1`.
#[derive(Clone, Debug)]
pub struct SmoothTower {
    pub p: u64,
    pub degrees: Vec<u64>,
    pub steps: Vec<MapStep>,
    pub c: u64,
    pub v: Vec<u64>,
    pub kind: TowerKind,
}

impl SmoothTower {
    pub fn size(&self) -> u64 {
        self.degrees.iter().product()
    }
    /// `L(x)`, or `None` at a pole of some step.
    pub fn eval(&self, x: u64) -> Option<u64> {
        self.steps.iter().try_fold(x, |y, s| s.eval(y, self.p))
    }
    /// The checks every tower must pass before it is used: `|V| = S`
    /// distinct values and `L(v) = c` on every `v ∈ V`; below 2^22, also
    /// that `L(x) = c` has no other root in `F_p`.
    pub fn verify(&self) -> Result<(), String> {
        let s = self.size();
        let set: HashSet<u64> = self.v.iter().copied().collect();
        if self.v.len() as u64 != s || set.len() as u64 != s {
            return Err(format!(
                "|V| = {} distinct of {}, expected S = {s}",
                set.len(),
                self.v.len()
            ));
        }
        for (k, s) in self.steps.iter().enumerate() {
            let deg = (s.num.len().max(s.den.len()) - 1) as u64;
            if deg != s.degree {
                return Err(format!(
                    "step {k}: rational degree {deg}, expected {}",
                    s.degree
                ));
            }
        }
        for &x in &self.v {
            if self.eval(x) != Some(self.c) {
                return Err(format!("L({x}) ≠ c"));
            }
        }
        if self.p < 1 << 22 {
            let n = (0..self.p)
                .filter(|&x| self.eval(x) == Some(self.c))
                .count() as u64;
            if n != s {
                return Err(format!("L(x) = c has {n} roots in F_p, expected {s}"));
            }
        }
        Ok(())
    }
}

/// Kummer form: `L_j(x) = x^{q_j}`, `c = g^S`. Needs `S | p − 1`; `g`
/// is drawn from the seed when not given.
pub fn kummer_tower(
    p: u64,
    degrees: &[u64],
    g: Option<u64>,
    seed: u64,
) -> Result<SmoothTower, String> {
    check_degrees(p, degrees)?;
    let s: u64 = degrees.iter().product();
    if (p - 1) % s != 0 {
        return Err(format!("S = {s} does not divide p − 1"));
    }
    let mut rng = Rng(seed);
    let g = g.unwrap_or_else(|| 1 + rng.below(p - 1));
    if g % p == 0 {
        return Err("g = 0".into());
    }
    // primitive S-th root of unity
    let mut primes: Vec<u64> = degrees.to_vec();
    primes.sort_unstable();
    primes.dedup();
    let zeta = loop {
        let h = 2 + rng.below(p - 3);
        let z = powm(h, (p - 1) / s, p);
        if primes.iter().all(|&q| powm(z, s / q, p) != 1) {
            break z;
        }
    };
    let mut v = Vec::with_capacity(s as usize);
    let mut w = g % p;
    for _ in 0..s {
        v.push(w);
        w = mulm(w, zeta, p);
    }
    let steps = degrees
        .iter()
        .map(|&q| {
            let mut num = vec![0; q as usize + 1];
            num[q as usize] = 1;
            MapStep {
                degree: q,
                num,
                den: vec![1],
            }
        })
        .collect();
    Ok(SmoothTower {
        p,
        degrees: degrees.to_vec(),
        steps,
        c: powm(g, s, p),
        v,
        kind: TowerKind::Kummer { g },
    })
}

fn check_degrees(p: u64, degrees: &[u64]) -> Result<(), String> {
    if !is_prime(p) || p < 5 || p >= 1 << 62 {
        return Err(format!("p = {p}: need a prime 5 ≤ p < 2^62"));
    }
    if degrees.is_empty() {
        return Err("empty degree list".into());
    }
    if let Some(&q) = degrees.iter().find(|&&q| !is_prime(q) || q == p) {
        return Err(format!("degree {q} is not a prime different from p"));
    }
    let mut s: u64 = 1;
    for &q in degrees {
        s = s.checked_mul(q).ok_or("S overflows")?;
    }
    Ok(())
}

/// Vélu's isogeny with kernel `K` (all its points, `O` excluded),
/// returned as the x-map `N/D`, the codomain, and the point map
/// `P ↦ (x(P) + Σ (x(P+T) − x(T)), y(P) + Σ (y(P+T) − y(T)))`.
pub struct Velu {
    pub step: MapStep,
    pub codomain: Curve,
    kernel: Vec<(u64, u64)>,
    dom: Curve,
}

impl Velu {
    pub fn new(e: &Curve, kernel: &[(u64, u64)]) -> Velu {
        let p = e.p;
        // One representative per ±-pair.
        let mut reps: Vec<(u64, u64)> = vec![];
        for &(x, y) in kernel {
            if !reps.iter().any(|&(rx, _)| rx == x) {
                reps.push((x, y));
            }
        }
        let (mut vsum, mut wsum) = (0u64, 0u64);
        // X = x + Σ v_T/(x − x_T) + u_T/(x − x_T)²
        let mut num: Poly = vec![0, 1];
        let mut den: Poly = vec![1];
        for &(xt, yt) in &reps {
            let gx = addm(mulm(3, mulm(xt, xt, p), p), e.a, p);
            let (vt, ut) = if yt == 0 {
                (gx, 0)
            } else {
                (mulm(2, gx, p), mulm(4, mulm(yt, yt, p), p))
            };
            vsum = addm(vsum, vt, p);
            wsum = addm(wsum, addm(ut, mulm(xt, vt, p), p), p);
            let lin: Poly = vec![subm(0, xt, p), 1];
            if yt == 0 {
                // num/den + v/(x − x_T)
                num = poly_add(&poly_mul(&num, &lin, p), &poly_scale(&den, vt, p), p);
                den = poly_mul(&den, &lin, p);
            } else {
                let sq = poly_mul(&lin, &lin, p);
                let extra = poly_add(&poly_scale(&lin, vt, p), &vec![ut], p);
                num = poly_add(&poly_mul(&num, &sq, p), &poly_mul(&den, &extra, p), p);
                den = poly_mul(&den, &sq, p);
            }
        }
        let codomain = Curve {
            p,
            a: subm(e.a, mulm(5, vsum, p), p),
            b: subm(e.b, mulm(7, wsum, p), p),
        };
        let degree = kernel.len() as u64 + 1;
        Velu {
            step: MapStep { degree, num, den },
            codomain,
            kernel: kernel.to_vec(),
            dom: *e,
        }
    }
    pub fn map(&self, pt: Pt) -> Pt {
        let (x, y) = pt?;
        if self.kernel.iter().any(|&(kx, ky)| kx == x && ky == y) {
            return None;
        }
        let p = self.dom.p;
        let (mut xx, mut yy) = (x, y);
        for &t in &self.kernel {
            let (sx, sy) = self.dom.add(pt, Some(t)).expect("P + T = O with P ∉ K");
            xx = addm(xx, subm(sx, t.0, p), p);
            yy = addm(yy, subm(sy, t.1, p), p);
        }
        Some((xx, yy))
    }
}

/// Isogeny form on a given auxiliary curve `aux` with `#aux = aux_order`
/// and a point `q` of order exactly `S = ∏ degrees` (degree order is the
/// order of the chain).
pub fn isogeny_tower_on(
    aux: Curve,
    aux_order: u64,
    q: (u64, u64),
    degrees: &[u64],
    seed: u64,
    curves_tried: u64,
) -> Result<SmoothTower, String> {
    let p = aux.p;
    check_degrees(p, degrees)?;
    let s: u64 = degrees.iter().product();
    if !aux.on_curve(Some(q)) || aux.mul(s, Some(q)).is_some() {
        return Err("Q is not a point of order dividing S".into());
    }
    let mut primes = degrees.to_vec();
    primes.sort_unstable();
    primes.dedup();
    if primes.iter().any(|&l| aux.mul(s / l, Some(q)).is_none()) {
        return Err("Q does not have order exactly S".into());
    }
    let mut rng = Rng(seed ^ 0x5eed);
    // G = ⟨Q⟩ and R' with 2R' ∉ G.
    let mut g: Vec<Pt> = Vec::with_capacity(s as usize);
    let mut cur = None;
    for _ in 0..s {
        g.push(cur);
        cur = aux.add(cur, Some(q));
    }
    let gset: HashSet<Pt> = g.iter().copied().collect();
    let r = (0..256)
        .map(|_| aux.random_point(&mut rng))
        .find(|&r| !gset.contains(&aux.add(r, r)))
        .ok_or("no R' with 2R' ∉ G")?;
    let v: Vec<u64> = g
        .iter()
        .map(|&t| aux.add(r, t).expect("R' ∈ G").0)
        .collect();
    // The chain.
    let (mut e, mut qj, mut rj, mut rest) = (aux, Some(q), r, s);
    let (mut steps, mut codomains) = (vec![], vec![]);
    for &l in degrees {
        let gen = e.mul(rest / l, qj);
        let mut kernel = vec![];
        let mut k = gen;
        for _ in 1..l {
            kernel.push(k.expect("kernel generator has order l"));
            k = e.add(k, gen);
        }
        if k.is_some() {
            return Err(format!(
                "kernel generator of step of degree {l} has wrong order"
            ));
        }
        let iso = Velu::new(&e, &kernel);
        qj = iso.map(qj);
        rj = iso.map(rj);
        if !iso.codomain.on_curve(qj) || !iso.codomain.on_curve(rj) {
            return Err("Vélu image off the codomain".into());
        }
        e = iso.codomain;
        codomains.push(e);
        steps.push(iso.step);
        rest /= l;
    }
    let c = rj.ok_or("Φ(R') = O")?.0;
    let t = SmoothTower {
        p,
        degrees: degrees.to_vec(),
        steps,
        c,
        v,
        kind: TowerKind::Isogeny {
            aux,
            aux_order,
            q,
            r: r.unwrap(),
            codomains,
            curves_tried,
        },
    };
    Ok(t)
}

/// Outcome of a Method-1 search.
#[derive(Clone, Debug)]
pub struct AuxFound {
    pub aux: Curve,
    pub order: u64,
    /// Chosen degrees (ascending), `S = ∏`.
    pub degrees: Vec<u64>,
    pub q: (u64, u64),
    pub curves_tried: u64,
    pub orders_failed: u64,
}

/// Method 1: random `E'/F_p` until `#E'` has a `bound`-smooth cyclic
/// part of size at least `min_size`. The smallest prefix (ascending
/// primes, largest first dropped) reaching `min_size` is used, so `S` is
/// as close to the target as the order allows.
pub fn search_aux(
    p: u64,
    bound: u64,
    min_size: u64,
    seed: u64,
    max_curves: u64,
) -> Result<AuxFound, String> {
    if !is_prime(p) || p < 5 || p >= 1 << 62 {
        return Err(format!("p = {p}: need a prime 5 ≤ p < 2^62"));
    }
    let mut rng = Rng(seed);
    let mut failed = 0;
    for tried in 1..=max_curves {
        let e = Curve {
            p,
            a: rng.below(p),
            b: rng.below(p),
        };
        if e.is_singular() {
            continue;
        }
        let Some(n) = e.order(&mut rng) else {
            failed += 1;
            continue;
        };
        let (mut f, _) = smooth_split(n, bound.min(p - 1));
        f.retain(|&q| q != p);
        // Largest smooth cyclic part: Q = [n/S]P with order exactly S,
        // shrinking S by its largest factors until it fits.
        let mut sel: Vec<u64> = vec![];
        let mut s = 1u64;
        for &q in &f {
            if s.saturating_mul(q) > min_size.saturating_mul(bound) {
                break;
            }
            sel.push(q);
            s *= q;
            if s >= min_size {
                break;
            }
        }
        if s < min_size {
            continue;
        }
        let mut primes = sel.clone();
        primes.dedup();
        for _ in 0..16 {
            let pt = e.random_point(&mut rng);
            let qpt = e.mul(n / s, pt);
            if primes.iter().all(|&l| e.mul(s / l, qpt).is_some()) {
                return Ok(AuxFound {
                    aux: e,
                    order: n,
                    degrees: sel,
                    q: qpt.unwrap(),
                    curves_tried: tried,
                    orders_failed: failed,
                });
            }
        }
    }
    Err(format!("no auxiliary curve in {max_curves} draws"))
}

/// Method 1 for a **chosen** `S`: random `E'/F_p` until one has a point of
/// order exactly `S` (so a cyclic subgroup of that order). `degrees` is
/// returned empty; the caller orders the chain. Curves whose order is not
/// divisible by `S` are rejected after the order computation.
pub fn search_aux_exact(p: u64, s: u64, seed: u64, max_curves: u64) -> Result<AuxFound, String> {
    if !is_prime(p) || p < 5 || p >= 1 << 62 {
        return Err(format!("p = {p}: need a prime 5 ≤ p < 2^62"));
    }
    let (mut primes, _) = smooth_split(s, s);
    primes.dedup();
    let mut rng = Rng(seed);
    let mut failed = 0;
    for tried in 1..=max_curves {
        let e = Curve {
            p,
            a: rng.below(p),
            b: rng.below(p),
        };
        if e.is_singular() {
            continue;
        }
        let Some(n) = e.order(&mut rng) else {
            failed += 1;
            continue;
        };
        if n % s != 0 {
            continue;
        }
        for _ in 0..16 {
            let q = e.mul(n / s, e.random_point(&mut rng));
            if primes.iter().all(|&l| e.mul(s / l, q).is_some()) {
                return Ok(AuxFound {
                    aux: e,
                    order: n,
                    degrees: vec![],
                    q: q.unwrap(),
                    curves_tried: tried,
                    orders_failed: failed,
                });
            }
        }
    }
    Err(format!(
        "no auxiliary curve with a point of order {s} in {max_curves} draws"
    ))
}

// ------------------------------------------------- relation system (m=2..)

/// Semaev's `S_3(x_1, x_2, x_3)` on `y² = x³ + ax + b`.
pub fn s3(e: &Curve, x1: u64, x2: u64, x3: u64) -> u64 {
    let p = e.p;
    let d = subm(x1, x2, p);
    let s = addm(x1, x2, p);
    let pr = mulm(x1, x2, p);
    let t0 = mulm(mulm(d, d, p), mulm(x3, x3, p), p);
    let t1 = mulm(
        2,
        mulm(
            addm(mulm(s, addm(pr, e.a, p), p), mulm(2, e.b, p), p),
            x3,
            p,
        ),
        p,
    );
    let pa = subm(pr, e.a, p);
    let t2 = subm(mulm(pa, pa, p), mulm(4, mulm(e.b, s, p), p), p);
    addm(subm(t0, t1, p), t2, p)
}

/// One relation `±P_1 ± P_2 = R` with `x(P_i) ∈ V`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    pub p1: (u64, u64),
    pub p2: (u64, u64),
}

/// Every `m = 2` decomposition of `r` over the factor base
/// `{P ∈ E(F_p) : x(P) ∈ V}`: for each `x_1 ∈ V`, the roots `x_2` of the
/// quadratic `S_3(x_1, ·, x(R))`, kept when in `V`, then lifted and
/// checked by point addition (so `S_3` is never trusted on its own).
pub fn decompose_m2(e: &Curve, v: &[u64], r: (u64, u64)) -> Vec<Relation> {
    let p = e.p;
    let set: HashSet<u64> = v.iter().copied().collect();
    let mut out = vec![];
    let xr = r.0;
    let lift = |x: u64| sqrtm(e.rhs(x), p).map(|y| (x, y));
    for &x1 in v {
        // S_3(x1, x2, xr) as a quadratic in x2: evaluate at 0, 1, −1.
        let f0 = s3(e, x1, 0, xr);
        let f1 = s3(e, x1, 1, xr);
        let fm = s3(e, x1, p - 1, xr);
        let inv2 = invm(2, p);
        let a2 = mulm(subm(addm(f1, fm, p), mulm(2, f0, p), p), inv2, p);
        let a1 = mulm(subm(f1, fm, p), inv2, p);
        let roots: Vec<u64> = if a2 == 0 {
            if a1 == 0 {
                vec![]
            } else {
                vec![mulm(subm(0, f0, p), invm(a1, p), p)]
            }
        } else {
            let disc = subm(mulm(a1, a1, p), mulm(4, mulm(a2, f0, p), p), p);
            match sqrtm(disc, p) {
                None => vec![],
                Some(sd) => {
                    let i = invm(mulm(2, a2, p), p);
                    vec![
                        mulm(subm(sd, a1, p), i, p),
                        mulm(subm(subm(0, sd, p), a1, p), i, p),
                    ]
                }
            }
        };
        for x2 in roots {
            if x2 < x1 || !set.contains(&x2) {
                continue;
            }
            let (Some(a), Some(b)) = (lift(x1), lift(x2)) else {
                continue;
            };
            for pa in [Some(a), e.neg(Some(a))] {
                for pb in [Some(b), e.neg(Some(b))] {
                    if e.add(pa, pb) == Some(r) {
                        let rel = Relation {
                            p1: pa.unwrap(),
                            p2: pb.unwrap(),
                        };
                        if !out.contains(&rel) {
                            out.push(rel);
                        }
                    }
                }
            }
        }
    }
    out
}

fn poly_str(c: &Poly, var: &str) -> String {
    let mut terms = vec![];
    for (i, &k) in c.iter().enumerate().rev() {
        if k == 0 {
            continue;
        }
        terms.push(match i {
            0 => format!("{k}"),
            1 => format!("{k}*{var}"),
            _ => format!("{k}*{var}^{i}"),
        });
    }
    if terms.is_empty() {
        "0".into()
    } else {
        terms.join("+")
    }
}

fn s3_str(e: &Curve, x1: &str, x2: &str, x3: &str) -> String {
    let p = e.p;
    let (a, b) = (e.a, e.b);
    let two_b = mulm(2, b, p);
    let four_b = mulm(4, b, p);
    format!(
        "({x1}-{x2})^2*{x3}^2-2*(({x1}+{x2})*({x1}*{x2}+{a})+{two_b})*{x3}+({x1}*{x2}-{a})^2-{four_b}*({x1}+{x2})"
    )
}

/// Slide 17's system for `m` factor-base points and target abscissa
/// `xr`, in msolve's format (variables line, characteristic line, then
/// the polynomials, comma-separated). `S_{m+1}` is the chain
/// `S_3(x_{1,1}, x_{2,1}, u_1)`, `S_3(u_{k−1}, x_{k+1,1}, u_k)`, …,
/// `S_3(u_{m−2}, x_{m,1}, X)`.
pub fn relation_system_msolve(e: &Curve, tower: &SmoothTower, m: usize, xr: u64) -> String {
    assert!(m >= 2);
    let n = tower.steps.len();
    let x = |i: usize, j: usize| format!("x{i}_{j}");
    let mut vars = vec![];
    for i in 1..=m {
        for j in 1..=n {
            vars.push(x(i, j));
        }
    }
    for k in 1..m - 1 {
        vars.push(format!("u{k}"));
    }
    let mut eqs = vec![];
    // summation chain
    let xrs = xr.to_string();
    let mut left = x(1, 1);
    for k in 2..=m {
        let right = if k == m {
            xrs.clone()
        } else {
            format!("u{}", k - 1)
        };
        eqs.push(s3_str(e, &left, &x(k, 1), &right));
        left = right;
    }
    // tower equations
    for i in 1..=m {
        for j in 1..=n {
            let st = &tower.steps[j - 1];
            let xv = x(i, j);
            if j < n {
                eqs.push(format!(
                    "{}*({})-({})",
                    x(i, j + 1),
                    poly_str(&st.den, &xv),
                    poly_str(&st.num, &xv)
                ));
            } else {
                eqs.push(format!(
                    "({})-{}*({})",
                    poly_str(&st.num, &xv),
                    tower.c,
                    poly_str(&st.den, &xv)
                ));
            }
        }
    }
    format!("{}\n{}\n{}\n", vars.join(","), e.p, eqs.join(",\n"))
}

// ------------------------------------------------- Method 2: CM, fixed p

/// The 13 imaginary quadratic orders of class number 1, with `j`.
pub const CLASS_NUMBER_ONE: [(i64, i128); 13] = [
    (-3, 0),
    (-4, 1728),
    (-7, -3375),
    (-8, 8000),
    (-11, -32768),
    (-12, 54000),
    (-16, 287496),
    (-19, -884736),
    (-27, -12288000),
    (-28, 16581375),
    (-43, -884736000),
    (-67, -147197952000),
    (-163, -262537412640768000),
];

/// Cornacchia for `4p = t² + |D| v²` (`D < 0`, `D ≡ 0, 1 mod 4`): `(t, v)`
/// with `t, v ≥ 0`, or `None` when `p` is not a norm from the order.
pub fn cornacchia_4p(p: u64, d: i64) -> Option<(u64, u64)> {
    let ad = d.unsigned_abs();
    let dm = ((d % p as i64) + p as i64) as u64 % p;
    let mut x0 = sqrtm(dm, p)?;
    if (x0 & 1) != (ad & 1) {
        x0 = p - x0;
    }
    let (mut a, mut b) = (2 * p as u128, x0 as u128);
    let l = (2.0 * (p as f64).sqrt()).floor() as u128;
    while b > l {
        let r = a % b;
        a = b;
        b = r;
    }
    let rem = 4 * p as u128 - b * b;
    if rem % ad as u128 != 0 {
        return None;
    }
    let c = rem / ad as u128;
    let v = (c as f64).sqrt().round() as u128;
    for w in [v.saturating_sub(1), v, v + 1] {
        if w * w == c {
            return Some((b as u64, w as u64));
        }
    }
    None
}

/// Every trace `t'` of a curve with CM by the order of discriminant `d`
/// over `F_p`, given `4p = t² + |d|v²` (twists included: six for `d = −3`,
/// four for `d = −4`, two otherwise).
pub fn cm_traces(d: i64, t: u64, v: u64) -> Vec<i64> {
    let (t, v) = (t as i64, v as i64);
    let mut out = match d {
        -3 => vec![t, (t + 3 * v) / 2, (t - 3 * v) / 2],
        -4 => vec![t, 2 * v],
        _ => vec![t],
    };
    out.extend(out.clone().iter().map(|x| -x));
    out.sort_unstable();
    out.dedup();
    out
}

/// One CM curve over the given `p` with its order certified on random
/// points, and that order's smooth split.
#[derive(Clone, Debug)]
pub struct CmCurve {
    pub d: i64,
    /// Class number `h(D)`.
    pub h: usize,
    pub j: u64,
    pub curve: Curve,
    pub order: u64,
    pub trace: i64,
    pub smooth: Vec<u64>,
    pub cofactor: u64,
}

fn j_mod(j: i128, p: u64) -> u64 {
    (j.rem_euclid(p as i128)) as u64
}

/// The order of `e` among `candidates`, identified by annihilating random
/// points (exhaustive count below 2^16).
fn identify_order(e: &Curve, candidates: &[u64], rng: &mut Rng) -> Option<u64> {
    if e.p < 1 << 16 {
        let n = e.order(rng)?;
        return candidates.contains(&n).then_some(n);
    }
    let mut left: Vec<u64> = candidates.to_vec();
    for _ in 0..32 {
        let pt = e.random_point(rng);
        left.retain(|&n| e.mul(n, pt).is_none());
        if left.len() <= 1 {
            break;
        }
    }
    (left.len() == 1).then(|| left[0])
}

/// The CM curves with j-invariant `jm` over `F_p` whose order is one of
/// `p + 1 − t'` for `t'` in `traces`: twists drawn until every trace is
/// covered or the draws run out. A curve is kept only when its order is
/// identified among the candidates on random points, so a wrong `j` (a
/// mis-rounded class polynomial, say) yields nothing rather than a wrong
/// curve.
fn curves_with_j(
    p: u64,
    d: i64,
    h: usize,
    jm: u64,
    traces: &[i64],
    bound: u64,
    rng: &mut Rng,
) -> Vec<CmCurve> {
    let cands: Vec<u64> = traces
        .iter()
        .map(|&tr| (p as i64 + 1 - tr) as u64)
        .collect();
    let mut seen: HashSet<u64> = HashSet::new();
    let mut out = vec![];
    let j1728 = 1728 % p;
    for _ in 0..64 * cands.len() {
        if seen.len() == cands.len() {
            break;
        }
        let u = 1 + rng.below(p - 1);
        let e = if jm == 0 {
            Curve { p, a: 0, b: u }
        } else if jm == j1728 {
            Curve { p, a: u, b: 0 }
        } else {
            let k = mulm(jm, invm(subm(j1728, jm, p), p), p);
            let (u2, u3) = (mulm(u, u, p), mulm(mulm(u, u, p), u, p));
            Curve {
                p,
                a: mulm(mulm(3, k, p), u2, p),
                b: mulm(mulm(2, k, p), u3, p),
            }
        };
        if e.is_singular() {
            continue;
        }
        let Some(n) = identify_order(&e, &cands, rng) else {
            continue;
        };
        if !seen.insert(n) {
            continue;
        }
        let (smooth, cofactor) = smooth_split(n, bound);
        out.push(CmCurve {
            d,
            h,
            j: jm,
            curve: e,
            order: n,
            trace: p as i64 + 1 - n as i64,
            smooth,
            cofactor,
        });
    }
    out
}

/// Method 2 for a **given** prime `p`: for each class-number-1
/// discriminant in which `p` splits, every CM curve over `F_p` (one per
/// trace), its order (checked on random points), and the `bound`-smooth
/// part of that order. Nothing here chooses or changes `p`.
pub fn cm_curves(p: u64, bound: u64, seed: u64) -> Result<Vec<CmCurve>, String> {
    if !is_prime(p) || p < 5 || p >= 1 << 62 {
        return Err(format!("p = {p}: need a prime 5 ≤ p < 2^62"));
    }
    let mut rng = Rng(seed);
    let mut out = vec![];
    for &(d, j) in CLASS_NUMBER_ONE.iter() {
        if (d.unsigned_abs() as u64) % p == 0 {
            continue;
        }
        let Some((t, v)) = cornacchia_4p(p, d) else {
            continue;
        };
        let jm = j_mod(j, p);
        // j(d) ≡ 0 or 1728 (mod p) for another order: those curves are the
        // j = 0 / 1728 ones, handled by d = −3 / −4.
        if (jm == 0 && j != 0) || (jm == 1728 % p && j != 1728) {
            continue;
        }
        out.extend(curves_with_j(
            p,
            d,
            1,
            jm,
            &cm_traces(d, t, v),
            bound,
            &mut rng,
        ));
    }
    Ok(out)
}

// ---------------------------------------- polynomials mod p: root finding

fn poly_deg(a: &Poly) -> usize {
    a.len() - 1
}
fn poly_is_zero(a: &Poly) -> bool {
    a.len() == 1 && a[0] == 0
}
/// `(q, r)` with `a = q·b + r`, `deg r < deg b`; `b ≠ 0`.
pub fn poly_divrem(a: &Poly, b: &Poly, p: u64) -> (Poly, Poly) {
    let b = poly_trim(b.clone());
    assert!(!poly_is_zero(&b), "division by zero polynomial");
    let mut r = poly_trim(a.clone());
    let db = poly_deg(&b);
    if r.len() < b.len() {
        return (vec![0], r);
    }
    let inv = invm(b[db], p);
    let mut q = vec![0; r.len() - db];
    for k in (0..q.len()).rev() {
        let c = mulm(r[k + db], inv, p);
        q[k] = c;
        if c != 0 {
            for (i, &bi) in b.iter().enumerate() {
                r[i + k] = subm(r[i + k], mulm(c, bi, p), p);
            }
        }
    }
    r.truncate(db.max(1));
    if db == 0 {
        r = vec![0];
    }
    (poly_trim(q), poly_trim(r))
}
fn poly_monic(a: &Poly, p: u64) -> Poly {
    let a = poly_trim(a.clone());
    let c = invm(*a.last().unwrap(), p);
    poly_scale(&a, c, p)
}
/// Monic gcd.
pub fn poly_gcd(a: &Poly, b: &Poly, p: u64) -> Poly {
    let (mut a, mut b) = (poly_trim(a.clone()), poly_trim(b.clone()));
    while !poly_is_zero(&b) {
        let r = poly_divrem(&a, &b, p).1;
        a = b;
        b = r;
    }
    if poly_is_zero(&a) {
        a
    } else {
        poly_monic(&a, p)
    }
}
fn poly_powmod(base: &Poly, mut e: u64, f: &Poly, p: u64) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = poly_divrem(base, f, p).1;
    while e > 0 {
        if e & 1 == 1 {
            r = poly_divrem(&poly_mul(&r, &b, p), f, p).1;
        }
        b = poly_divrem(&poly_mul(&b, &b, p), f, p).1;
        e >>= 1;
    }
    r
}
fn split_roots(g: &Poly, p: u64, rng: &mut Rng, out: &mut Vec<u64>) {
    match poly_deg(g) {
        0 => {}
        1 => out.push(mulm(subm(0, g[0], p), invm(g[1], p), p)),
        _ => loop {
            // Cantor–Zassenhaus: gcd((x + a)^((p−1)/2) − 1, g).
            let a = rng.below(p);
            let w = poly_powmod(&vec![a, 1], (p - 1) / 2, g, p);
            let w = poly_add(&w, &vec![p - 1], p);
            let h = poly_gcd(&w, g, p);
            let dh = if poly_is_zero(&h) { 0 } else { poly_deg(&h) };
            if dh > 0 && dh < poly_deg(g) {
                split_roots(&h, p, rng, out);
                split_roots(&poly_divrem(g, &h, p).0, p, rng, out);
                return;
            }
        },
    }
}
/// The distinct roots of `f` in `F_p` (`p` odd), sorted.
pub fn poly_roots(f: &Poly, p: u64, rng: &mut Rng) -> Vec<u64> {
    let f = poly_trim(f.clone());
    if poly_deg(&f) == 0 {
        return vec![];
    }
    let f = poly_monic(&f, p);
    // gcd(x^p − x, f): the product of the distinct linear factors.
    let xp = poly_powmod(&vec![0, 1], p, &f, p);
    let g = poly_gcd(&poly_add(&xp, &vec![0, p - 1], p), &f, p);
    let mut out = vec![];
    if !poly_is_zero(&g) {
        split_roots(&g, p, rng, &mut out);
    }
    out.sort_unstable();
    out.dedup();
    out
}

// ------------------------------- Method 2 with class number > 1 (Hilbert)

/// What a Hilbert-polynomial CM scan did, per stage.
#[derive(Clone, Debug, Default)]
pub struct CmScanStats {
    /// Discriminants with `h(D) ≤ max_h` examined.
    pub discriminants: u64,
    /// Of which `4p = t² + |D|v²` has a solution (p splits in the order).
    pub split: u64,
    /// `H_D` computations that failed their rounding test.
    pub poly_failed: u64,
    /// Roots of `H_D mod p` found, and how many split discriminants had
    /// fewer than `h(D)` roots in `F_p` (expected: none).
    pub roots: u64,
    pub incomplete_splits: u64,
    /// Roots for which no curve's order matched a CM trace (expected:
    /// none; a hit means the polynomial or the root is wrong).
    pub roots_rejected: u64,
}

/// Method 2 for a **given** prime `p` with class number above 1: for every
/// discriminant `D` with `|D| ≤ max_disc` and `h(D) ≤ max_h` in whose order
/// `p` splits (Cornacchia), the roots of the class polynomial `H_D mod p`
/// (`isogeny_algos::cm::hilbert_class_poly`, primitive forms, so ring class
/// polynomials for non-maximal orders), and for every root every twist
/// whose order is certified as `p + 1 ∓ t` on random points. Class number
/// 1 is included (`H_D` then has degree 1). Nothing here chooses `p`.
pub fn cm_curves_hilbert(
    p: u64,
    bound: u64,
    max_disc: u64,
    max_h: usize,
    seed: u64,
) -> Result<(Vec<CmCurve>, CmScanStats), String> {
    use isogeny_algos::cm::{hilbert_class_poly, reduced_forms};
    if !is_prime(p) || p < 5 || p >= 1 << 62 {
        return Err(format!("p = {p}: need a prime 5 ≤ p < 2^62"));
    }
    let mut rng = Rng(seed);
    let mut st = CmScanStats::default();
    let mut out = vec![];
    for ad in 3..=max_disc {
        if !(ad % 4 == 0 || ad % 4 == 3) || ad % p == 0 {
            continue;
        }
        let d = -(ad as i64);
        let h = reduced_forms(d).len();
        if h > max_h {
            continue;
        }
        st.discriminants += 1;
        let Some((t, v)) = cornacchia_4p(p, d) else {
            continue;
        };
        st.split += 1;
        let Ok(hp) = hilbert_class_poly(d) else {
            st.poly_failed += 1;
            continue;
        };
        let f: Poly = poly_trim(hp.coeffs.iter().map(|c| c.mod_u64(p)).collect());
        let roots = poly_roots(&f, p, &mut rng);
        st.roots += roots.len() as u64;
        if roots.len() < h {
            st.incomplete_splits += 1;
        }
        let traces = cm_traces(d, t, v);
        for jm in roots {
            // j ≡ 0 or 1728 belongs to d = −3 / −4 (different twists).
            if (jm == 0 && d != -3) || (jm == 1728 % p && d != -4) {
                continue;
            }
            let cs = curves_with_j(p, d, h, jm, &traces, bound, &mut rng);
            if cs.is_empty() {
                st.roots_rejected += 1;
            }
            out.extend(cs);
        }
    }
    Ok((out, st))
}

/// The first curve of `curves` whose order has a cyclic `bound`-smooth
/// part of at least `min_size`, as an [`AuxFound`], with its `D`.
pub fn cm_aux_from(curves: &[CmCurve], min_size: u64, seed: u64) -> Option<(AuxFound, i64)> {
    let mut rng = Rng(seed ^ 0xc0ffee);
    for c in curves {
        let (e, n, p) = (c.curve, c.order, c.curve.p);
        let mut sel = vec![];
        let mut s = 1u64;
        for &q in &c.smooth {
            if q == p {
                continue;
            }
            sel.push(q);
            s *= q;
            if s >= min_size {
                break;
            }
        }
        if s < min_size {
            continue;
        }
        let mut primes = sel.clone();
        primes.dedup();
        for _ in 0..32 {
            let qpt = e.mul(n / s, e.random_point(&mut rng));
            if primes.iter().all(|&l| e.mul(s / l, qpt).is_some()) {
                return Some((
                    AuxFound {
                        aux: e,
                        order: n,
                        degrees: sel,
                        q: qpt.unwrap(),
                        curves_tried: 0,
                        orders_failed: 0,
                    },
                    c.d,
                ));
            }
        }
    }
    None
}

/// [`cm_aux_from`] over the class-number-1 curves of [`cm_curves`].
pub fn cm_aux(p: u64, bound: u64, min_size: u64, seed: u64) -> Result<(AuxFound, i64), String> {
    cm_aux_from(&cm_curves(p, bound, seed)?, min_size, seed).ok_or_else(|| {
        format!(
            "no class-number-1 CM curve over p = {p} has a cyclic {bound}-smooth part ≥ {min_size}"
        )
    })
}

// ------------------------------------- algebraic decomposition oracle (slide 17)

/// Slide 17's system as polynomials over `F_p` for [`crate::cryptanalysis::f4_fp`]:
/// unknowns `x_{i,j}` (index `i·n' + j`) then the `m − 2` chain helpers
/// `u_k`; equations `x_{i,j+1}·D_j(x_{i,j}) − N_j(x_{i,j})`,
/// `N_{n'}(x_{i,n'}) − c·D_{n'}(x_{i,n'})`, and the `S_3` chain
/// `S_3(x_{1,1}, x_{2,1}, u_1), …, S_3(u_{m−2}, x_{m,1}, X)` (just
/// `S_3(x_{1,1}, x_{2,1}, X)` at `m = 2`). Returns `(terms, n_vars)`.
pub fn relation_system_polys(
    e: &Curve,
    tower: &SmoothTower,
    m: usize,
    xr: u64,
) -> (Vec<Vec<(Vec<u32>, u64)>>, usize) {
    assert!(m >= 2);
    let p = e.p;
    let n = tower.steps.len();
    let n_vars = m * n + (m - 2);
    let xv = |i: usize, j: usize| i * n + j;
    let mut out = vec![];
    for i in 0..m {
        for j in 0..n {
            let st = &tower.steps[j];
            let mut eq: HashMap<Vec<u32>, u64> = HashMap::new();
            let mut put = |mono: Vec<u32>, c: u64| {
                let e = eq.entry(mono).or_insert(0);
                *e = addm(*e, c % p, p);
            };
            for (k, &c) in st.num.iter().enumerate() {
                let mut mono = vec![0u32; n_vars];
                mono[xv(i, j)] = k as u32;
                put(mono, p - c % p);
            }
            for (k, &c) in st.den.iter().enumerate() {
                let mut mono = vec![0u32; n_vars];
                mono[xv(i, j)] = k as u32;
                if j + 1 < n {
                    mono[xv(i, j + 1)] += 1;
                    put(mono, c);
                } else {
                    put(mono, mulm(c, tower.c, p));
                }
            }
            out.push(eq.into_iter().filter(|&(_, c)| c != 0).collect());
        }
    }
    // S_3 chain. Slot: Some(var) or None for the constant X.
    let s3_poly = |a: Option<usize>, b: usize, last: Option<usize>| -> Vec<(Vec<u32>, u64)> {
        let m2 = p - 2;
        let terms: [([u32; 3], u64); 13] = [
            ([2, 0, 2], 1),
            ([0, 2, 2], 1),
            ([1, 1, 2], m2),
            ([2, 1, 1], m2),
            ([1, 2, 1], m2),
            ([1, 0, 1], mulm(m2, e.a, p)),
            ([0, 1, 1], mulm(m2, e.a, p)),
            ([0, 0, 1], mulm(p - 4, e.b, p)),
            ([2, 2, 0], 1),
            ([1, 1, 0], mulm(m2, e.a, p)),
            ([0, 0, 0], mulm(e.a, e.a, p)),
            ([1, 0, 0], mulm(p - 4, e.b, p)),
            ([0, 1, 0], mulm(p - 4, e.b, p)),
        ];
        let mut acc: HashMap<Vec<u32>, u64> = HashMap::new();
        for (ex, c) in terms {
            let mut mono = vec![0u32; n_vars];
            let mut coef = c;
            for (k, slot) in [a, Some(b), last].iter().enumerate() {
                match slot {
                    Some(v) => mono[*v] += ex[k],
                    None => coef = mulm(coef, powm(xr, ex[k] as u64, p), p),
                }
            }
            let t = acc.entry(mono).or_insert(0);
            *t = addm(*t, coef, p);
        }
        acc.into_iter().filter(|&(_, c)| c != 0).collect()
    };
    let u = |k: usize| m * n + k; // u_1 … u_{m−2} at indices m·n + 0 …
    if m == 2 {
        out.push(s3_poly(Some(xv(0, 0)), xv(1, 0), None));
    } else {
        out.push(s3_poly(Some(xv(0, 0)), xv(1, 0), Some(u(0))));
        for k in 2..m {
            let right = if k == m - 1 { None } else { Some(u(k - 1)) };
            out.push(s3_poly(Some(u(k - 2)), xv(k, 0), right));
        }
    }
    (out, n_vars)
}

/// What the algebraic oracle did on one target.
#[derive(Clone, Debug)]
pub struct AlgebraicDecomposition {
    /// Relations `P_1 + … + P_m = R` with every `x(P_i) ∈ V`, each checked by
    /// point addition (sorted, as point lists).
    pub relations: Vec<Vec<(u64, u64)>>,
    /// Solutions of the system in `F_p` (all unknowns), before lifting.
    pub solutions: usize,
    /// Solutions whose abscissae do not all lift to `E(F_p)` (points over
    /// `F_{p²}`): not relations, dropped.
    pub unlifted: usize,
    /// `F_p` multiplications of F4's row reductions over the substitution
    /// tree (root finding of univariates is not included).
    pub field_ops: u64,
    pub solving_degree: u32,
    pub f4_runs: usize,
    pub max_cols: usize,
    pub ms: f64,
    /// The solver stopped (deadline or degree bound) before deciding.
    pub undetermined: bool,
}

/// Every sign choice of `pts` that sums to `r`, as sorted point lists.
fn signed_relations(e: &Curve, pts: &[(u64, u64)], r: (u64, u64)) -> Vec<Vec<(u64, u64)>> {
    let m = pts.len();
    let mut out = vec![];
    for mask in 0u32..(1 << m) {
        let chosen: Vec<(u64, u64)> = pts
            .iter()
            .enumerate()
            .map(|(i, &q)| {
                if mask >> i & 1 == 1 {
                    e.neg(Some(q)).unwrap()
                } else {
                    q
                }
            })
            .collect();
        let s = chosen.iter().fold(None, |acc, &q| e.add(acc, Some(q)));
        if s == Some(r) {
            let mut c = chosen;
            c.sort_unstable();
            if !out.contains(&c) {
                out.push(c);
            }
        }
    }
    out
}

/// The slides' relation search: solve the low-degree system of
/// [`relation_system_polys`] by F4 and substitution
/// ([`crate::cryptanalysis::f4_fp::solve`], grevlex, degree bound `dmax`),
/// then lift every solution's `x_{i,1}` to points and keep the sign choices
/// whose sum is `r`. Nothing is enumerated over `V`.
pub fn algebraic_decompose(
    e: &Curve,
    tower: &SmoothTower,
    m: usize,
    r: (u64, u64),
    dmax: u32,
    budget: std::time::Duration,
) -> AlgebraicDecomposition {
    use crate::cryptanalysis::f4_fp::{self, F4Options, Ordering, Verdict};
    let p = e.p;
    assert!(p < 1 << 32, "f4_fp needs p < 2^32");
    let (terms, n_vars) = relation_system_polys(e, tower, m, r.0);
    let input: Vec<f4_fp::Poly> = terms
        .iter()
        .map(|t| f4_fp::normalise(t, p, Ordering::Grevlex))
        .collect();
    let mut opts = F4Options::new(Ordering::Grevlex, dmax);
    opts.deadline = Some(std::time::Instant::now() + budget);
    let rep = f4_fp::solve(&input, n_vars, p, &opts);
    let n = tower.steps.len();
    let mut relations: Vec<Vec<(u64, u64)>> = vec![];
    let (mut solutions, mut unlifted) = (0, 0);
    let undetermined = matches!(rep.verdict, Verdict::Undetermined) || rep.timed_out;
    if let Verdict::Solutions(sols) = &rep.verdict {
        solutions = sols.len();
        for sol in sols {
            let xs: Vec<u64> = (0..m).map(|i| sol[i * n]).collect();
            let lifted: Option<Vec<(u64, u64)>> = xs
                .iter()
                .map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
                .collect();
            match lifted {
                None => unlifted += 1,
                Some(pts) => {
                    for rel in signed_relations(e, &pts, r) {
                        if !relations.contains(&rel) {
                            relations.push(rel);
                        }
                    }
                }
            }
        }
    }
    relations.sort();
    AlgebraicDecomposition {
        relations,
        solutions,
        unlifted,
        field_ops: rep.field_ops,
        solving_degree: rep.solving_degree,
        f4_runs: rep.f4_runs,
        max_cols: rep.max_cols,
        ms: rep.ms,
        undetermined,
    }
}

/// The baseline the oracle must beat: enumeration over the factor base, with
/// every `F_p` multiplication counted (a square root by Tonelli–Shanks
/// counted as its exponentiations). `m = 2`: for each `P_1`, the quadratic
/// `S_3(x_1, ·, x(R))`, roots looked up in `V`. `m = 3`: for each pair
/// `(P_1, P_2)`, the abscissae `u` of `P_1 ± P_2` (quadratic `S_3(x_1, x_2, ·)`),
/// then the quadratic `S_3(u, ·, x(R))`. Returns relations (as in
/// [`algebraic_decompose`]) and the multiplication count.
pub fn enumerate_decompositions(
    e: &Curve,
    v: &[u64],
    m: usize,
    r: (u64, u64),
) -> (Vec<Vec<(u64, u64)>>, u64) {
    assert!(m == 2 || m == 3, "enumeration baseline for m = 2, 3");
    let p = e.p;
    let mut ops = 0u64;
    let logp = 64 - p.leading_zeros() as u64;
    // Roots in F_p of c2 X² + c1 X + c0, with its multiplications counted.
    let quad_roots = |c: [u64; 3], ops: &mut u64| -> Vec<u64> {
        if c[2] == 0 {
            if c[1] == 0 {
                return vec![];
            }
            *ops += 2 * logp + 1;
            return vec![mulm(p - c[0] % p, invm(c[1], p), p)];
        }
        *ops += 3;
        let disc = subm(mulm(c[1], c[1], p), mulm(4, mulm(c[2], c[0], p), p), p);
        *ops += 4 * logp; // Legendre + square root, as exponentiations
        let Some(sd) = sqrtm(disc, p) else {
            return vec![];
        };
        *ops += 2 * logp + 2;
        let i = invm(mulm(2, c[2], p), p);
        vec![
            mulm(subm(sd, c[1], p), i, p),
            mulm(subm(p - sd, c[1], p), i, p),
        ]
    };
    // S_3(a, b, X) as a quadratic in X (12 multiplications).
    let s3_in_last = |a: u64, b: u64, ops: &mut u64| -> [u64; 3] {
        *ops += 12;
        let d = subm(a, b, p);
        let s = addm(a, b, p);
        let pr = mulm(a, b, p);
        let c2 = mulm(d, d, p);
        let c1 = subm(
            0,
            mulm(2, addm(mulm(s, addm(pr, e.a, p), p), mulm(2, e.b, p), p), p),
            p,
        );
        let pa = subm(pr, e.a, p);
        let c0 = subm(mulm(pa, pa, p), mulm(4, mulm(e.b, s, p), p), p);
        [c0, c1, c2]
    };
    let base: Vec<(u64, u64)> = v
        .iter()
        .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
        .collect();
    ops += base.len() as u64 * 6 * logp; // lifting V once
    let vset: HashSet<u64> = v.iter().copied().collect();
    let mut rels: Vec<Vec<(u64, u64)>> = vec![];
    let push = |pts: Vec<(u64, u64)>, rels: &mut Vec<Vec<(u64, u64)>>| {
        for rel in signed_relations(e, &pts, r) {
            if !rels.contains(&rel) {
                rels.push(rel);
            }
        }
    };
    let lift = |x: u64| sqrtm(e.rhs(x), p).map(|y| (x, y));
    if m == 2 {
        for &p1 in &base {
            let q = s3_in_last(p1.0, r.0, &mut ops); // symmetric: S_3(x1, X, x2)
            for x2 in quad_roots(q, &mut ops) {
                if vset.contains(&x2) {
                    if let Some(p2) = lift(x2) {
                        push(vec![p1, p2], &mut rels);
                    }
                }
            }
        }
    } else {
        for (i, &p1) in base.iter().enumerate() {
            for &p2 in &base[i..] {
                let q = s3_in_last(p1.0, p2.0, &mut ops);
                for uu in quad_roots(q, &mut ops) {
                    let q2 = s3_in_last(uu, r.0, &mut ops);
                    for x3 in quad_roots(q2, &mut ops) {
                        if vset.contains(&x3) {
                            if let Some(p3) = lift(x3) {
                                push(vec![p1, p2, p3], &mut rels);
                            }
                        }
                    }
                }
            }
        }
    }
    rels.sort();
    (rels, ops)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn velu_odd_matches_point_map_and_codomain() {
        let p = 10007;
        let mut rng = Rng(1);
        for _ in 0..40 {
            let e = Curve {
                p,
                a: rng.below(p),
                b: rng.below(p),
            };
            if e.is_singular() {
                continue;
            }
            let n = e.order(&mut rng).unwrap();
            for l in [2u64, 3, 5, 7] {
                if n % l != 0 {
                    continue;
                }
                let gen = (0..20)
                    .map(|_| e.mul(n / l, e.random_point(&mut rng)))
                    .find(|g| g.is_some());
                let Some(gen) = gen else { continue };
                let mut ker = vec![];
                let mut k = gen;
                for _ in 1..l {
                    ker.push(k.unwrap());
                    k = e.add(k, gen);
                }
                let iso = Velu::new(&e, &ker);
                // isogenous curves have the same order
                assert_eq!(iso.codomain.order(&mut rng).unwrap(), n);
                for _ in 0..10 {
                    let pt = e.random_point(&mut rng);
                    let im = iso.map(pt);
                    assert!(iso.codomain.on_curve(im));
                    if let (Some((x, _)), Some((ix, _))) = (pt, im) {
                        assert_eq!(iso.step.eval(x, p), Some(ix));
                    }
                }
            }
        }
    }

    #[test]
    fn kummer_mixed_tower_verifies() {
        // p − 1 = 2·3·5·7·... pick p with 2·3·3·5 | p − 1
        let p = (1..)
            .map(|k| 90 * k + 1)
            .find(|&q| q > 20000 && is_prime(q))
            .unwrap();
        let t = kummer_tower(p, &[3, 2, 5, 3], None, 7).unwrap();
        t.verify().unwrap();
        assert_eq!(t.v.len(), 90);
    }

    #[test]
    fn isogeny_mixed_tower_verifies() {
        let p = 100_003;
        let found = search_aux(p, 13, 60, 3, 5000).unwrap();
        let s: u64 = found.degrees.iter().product();
        assert!(s >= 60);
        let mut degs = found.degrees.clone();
        degs.reverse(); // any order of the chain works
        let t = isogeny_tower_on(
            found.aux,
            found.order,
            found.q,
            &degs,
            9,
            found.curves_tried,
        )
        .unwrap();
        t.verify().unwrap();
    }

    #[test]
    fn m2_relations_are_real_decompositions() {
        let p = 100_003;
        let found = search_aux(p, 13, 300, 11, 5000).unwrap();
        let t = isogeny_tower_on(found.aux, found.order, found.q, &found.degrees, 1, 0).unwrap();
        t.verify().unwrap();
        let e = Curve { p, a: 5, b: 7 };
        let mut rng = Rng(4);
        let base: Vec<(u64, u64)> =
            t.v.iter()
                .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
                .collect();
        assert!(base.len() > 4);
        // a target that is a known sum
        let r = e.add(Some(base[0]), Some(base[1])).unwrap();
        let rels = decompose_m2(&e, &t.v, r);
        assert!(!rels.is_empty());
        for rel in &rels {
            assert_eq!(e.add(Some(rel.p1), Some(rel.p2)), Some(r));
        }
        let _ = rng.next();
        let sys = relation_system_msolve(&e, &t, 3, r.0);
        assert!(sys.lines().next().unwrap().contains("u1"));
    }

    #[test]
    fn cm_curves_have_the_predicted_orders() {
        for p in [10007u64, 100_003, 1_000_003, 998_244_353] {
            let cs = cm_curves(p, 50, 3).unwrap();
            assert!(!cs.is_empty());
            let mut rng = Rng(8);
            for c in &cs {
                assert!(c.trace.unsigned_abs() as f64 <= 2.0 * (p as f64).sqrt() + 1.0);
                // order agrees with the generic count, and twists per d
                assert_eq!(c.curve.order(&mut rng), Some(c.order), "p={p} d={}", c.d);
            }
            for &(d, _) in CLASS_NUMBER_ONE.iter() {
                let k = cs.iter().filter(|c| c.d == d).count();
                if let Some((t, v)) = cornacchia_4p(p, d) {
                    assert_eq!(
                        4 * p as u128,
                        (t as u128).pow(2) + d.unsigned_abs() as u128 * (v as u128).pow(2)
                    );
                    assert_eq!(k, cm_traces(d, t, v).len(), "p={p} d={d}");
                } else {
                    assert_eq!(k, 0);
                }
            }
        }
    }

    #[test]
    fn cm_aux_feeds_the_isogeny_tower() {
        // a fixed prime; scan a few until one CM order is smooth enough
        let mut done = false;
        for p in (100_000u64..101_000).filter(|&q| is_prime(q)) {
            if let Ok((f, _d)) = cm_aux(p, 13, 40, 5) {
                let t = isogeny_tower_on(f.aux, f.order, f.q, &f.degrees, 2, 0).unwrap();
                t.verify().unwrap();
                done = true;
                break;
            }
        }
        assert!(done);
    }

    #[test]
    fn poly_roots_finds_planted_roots() {
        let p = 1_000_003;
        let mut rng = Rng(5);
        let planted = [3u64, 17, 999_999, 123_456, 77];
        let mut f: Poly = vec![1];
        for &r in &planted {
            f = poly_mul(&f, &vec![p - r, 1], p);
        }
        // times an irreducible quadratic x^2 − n (n a non-residue)
        let n = (2..p).find(|&n| !is_square(n, p)).unwrap();
        f = poly_mul(&f, &vec![p - n, 0, 1], p);
        let mut want = planted.to_vec();
        want.sort_unstable();
        assert_eq!(poly_roots(&f, p, &mut rng), want);
        let (q, r) = poly_divrem(&f, &vec![p - 3, 1], p);
        assert!(poly_is_zero(&r));
        assert_eq!(poly_deg(&q), 6);
    }

    #[test]
    fn hilbert_cm_curves_are_certified_and_extend_class_number_one() {
        for p in [1_000_003u64, 998_244_353] {
            let (cs, st) = cm_curves_hilbert(p, 50, 200, 8, 3).unwrap();
            assert_eq!(st.poly_failed, 0);
            assert_eq!(st.incomplete_splits, 0, "{st:?}");
            assert_eq!(st.roots_rejected, 0, "{st:?}");
            assert!(
                cs.iter().any(|c| c.h > 1),
                "no class number > 1 curve at p = {p}"
            );
            let mut rng = Rng(8);
            for c in cs.iter().filter(|c| c.h > 1).take(12) {
                assert_eq!(c.curve.order(&mut rng), Some(c.order), "p={p} d={}", c.d);
            }
            // every class-number-1 order from the dedicated path reappears
            let one = cm_curves(p, 50, 3).unwrap();
            for c in &one {
                assert!(
                    cs.iter().any(|x| x.d == c.d && x.order == c.order),
                    "d={} missing",
                    c.d
                );
            }
        }
    }

    #[test]
    fn search_aux_exact_gives_a_tower_in_any_order() {
        let p = 100_003;
        let f = search_aux_exact(p, 36, 4, 20_000).unwrap();
        assert_eq!(f.order % 36, 0);
        for degs in [[2u64, 3, 2, 3], [3, 2, 3, 2], [3, 3, 2, 2]] {
            let t = isogeny_tower_on(f.aux, f.order, f.q, &degs, 1, f.curves_tried).unwrap();
            t.verify().unwrap();
        }
    }

    #[test]
    fn algebraic_oracle_finds_exactly_the_enumerated_relations() {
        let p = 10_009; // p − 1 = 2^3 · 3^2 · 139
        let e = Curve { p, a: 3, b: 7 };
        let mut rng = Rng(12);
        for degs in [vec![2u64, 2, 2], vec![3, 3], vec![2, 3]] {
            let t = kummer_tower(p, &degs, None, 5).unwrap();
            let base: Vec<(u64, u64)> =
                t.v.iter()
                    .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
                    .collect();
            assert!(base.len() >= 2);
            for m in [2usize, 3] {
                for trial in 0..3 {
                    // planted target (trial 0, 1) and a random one (trial 2)
                    let r = if trial < 2 {
                        let pts: Vec<_> = (0..m)
                            .map(|_| base[rng.below(base.len() as u64) as usize])
                            .collect();
                        match pts.iter().fold(None, |a, &q| e.add(a, Some(q))) {
                            Some(r) => r,
                            None => continue,
                        }
                    } else {
                        e.random_point(&mut rng).unwrap()
                    };
                    let alg =
                        algebraic_decompose(&e, &t, m, r, 40, std::time::Duration::from_secs(60));
                    assert!(!alg.undetermined, "degs {degs:?} m {m}");
                    let (en, ops) = enumerate_decompositions(&e, &t.v, m, r);
                    assert!(ops > 0);
                    assert_eq!(alg.relations, en, "degs {degs:?} m {m} trial {trial}");
                    if trial < 2 {
                        assert!(!en.is_empty(), "planted relation missing");
                    }
                }
            }
        }
        // isogeny form too
        let f = search_aux_exact(p, 6, 3, 50_000).unwrap();
        let t = isogeny_tower_on(f.aux, f.order, f.q, &[2, 3], 2, 0).unwrap();
        let base: Vec<(u64, u64)> =
            t.v.iter()
                .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
                .collect();
        if base.len() >= 2 {
            let r = e.add(Some(base[0]), Some(base[1])).unwrap();
            let alg = algebraic_decompose(&e, &t, 2, r, 40, std::time::Duration::from_secs(60));
            let (en, _) = enumerate_decompositions(&e, &t.v, 2, r);
            assert_eq!(alg.relations, en);
            assert!(!en.is_empty());
        }
    }
}
