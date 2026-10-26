//! Round 6, lead R6.11: is index-calculus relation yield the same on both
//! sides of a large-prime vertical isogeny gap?  (The direct toy-size test
//! of Conjecture 10′, round 5 R5.4: a counterexample to exponent invariance
//! over `F_p` needs an algorithm whose cost depends on `End(E)` beyond `O^×`
//! and `E(F_p)[h]`, across a prime conductor gap.)
//!
//! Instances.  For a prime `ℓ ∈ {5, 7, 11, 13}` and a class-number-one
//! discriminant `D ∈ {−11, −19, −43, −67, −163}` with `ℓ ∤ D`, a prime
//! `p = (t² + |D|v²)/4 ∈ [2^20, 2^22)` with `ℓ ∥ v`, `t ≢ 2 (mod ℓ)` and
//! `N = p + 1 − t` prime.  Then the CM curve `E₀` with `j = j(D)` has
//! `End(E₀) = O_K` (the crater of the `ℓ`-volcano, which has a single vertex
//! because `h(D) = 1`), the Frobenius conductor is `f_π = v` with
//! `v_ℓ(f_π) = 1`, and every root `j₁ ≠ j₀` of `Φ_ℓ(j₀, Y)` over `F_p` is a
//! floor curve `E₁` with `End(E₁) = Z + ℓO_K`: the two endomorphism rings
//! differ exactly at `ℓ`.  Because `t ≢ 2 (mod ℓ)`, `ℓ ∤ N`, so neither level
//! has rational `ℓ`-torsion and `E(F_p) ≅ Z/N` is cyclic of prime order at
//! both levels (`h = 1`, `G = E(F_p)`): `O^×` (`|Aut| = 2` at both levels,
//! `j ∉ {0, 1728}`) and `E(F_p)[h]` (trivial) agree, so any difference in
//! yield would be a dependence on the conductor itself.
//!
//! Each level's curve is the twist of `y² = x³ + 3j(1728−j)x + 2j(1728−j)²`
//! with `#E = N`, certified by `[N]P = O` on random points `P ≠ O` (`N`
//! prime and `N > p + 1 + 2√p − N`, so the order is exact).  The vertical
//! edge is certified with the library: `kernel::elkies_isogeny` builds the
//! Elkies kernel polynomial from `Φ_ℓ`, re-certifies it with the division
//! polynomial, and its Vélu codomain must be `F_p`-isomorphic to the floor
//! model.  The `ℓ`-adic level is read off the number of `F_p`-roots of
//! `Φ_ℓ(j, Y)`: `ℓ − (D/ℓ) + [(D/ℓ) = 1]` on the crater (Frobenius is the
//! scalar `t/2 mod ℓ` on `E₀[ℓ]`), exactly 1 on the floor.
//!
//! Census (RESEARCH_ISOGENY_FIBERS_ECDLP.md §15.2, elliptic version of
//! `r4_16_6_genus2_census.rs`).  For a signed base `F` of `|F|` points
//! (rule `smallx`: `x = 0, 1, 2, …` taken while `x³ + ax + b` is a nonzero
//! square, both signs; rule `randx`: the same with a seeded random order of
//! `x`), every `m`-multiset of base points is added exactly, `m = 2, 3`:
//! `D_m` is the number of distinct nonzero targets of `G` hit, `R_m` the
//! number of representations, `c = D_m · m! / |F|^m` the class factor,
//! `U = |F|/2` the columns (negation fold), `D_m / U` the decomposable targets
//! per unknown, and `U · (r / D_m) · L_m` (`L_2 = 1`, `L_3 = |F|`) the §15.2
//! cost model.  The birthday reference `c* = (m! r / |F|^m)(1 − e^{−|F|^m/(m! r)})`
//! is derived, not measured.
//!
//! ```text
//! r6_11_conductor_yield run --ell 5 --instances 6 --seed 1 --out <dir>
//! r6_11_conductor_yield summarise --dir <dir> --out summary.json
//! ```

use std::collections::HashSet;
use std::env;
use std::fs;

use crypto_lib::cryptanalysis::isogeny_walk::curve::{self, Model};
use crypto_lib::cryptanalysis::isogeny_walk::field::Field as LField;
use crypto_lib::cryptanalysis::isogeny_walk::{kernel, modpoly, poly as lpoly};
use crypto_lib::cryptanalysis::residual_walk::is_prime_u64;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

// ---------------------------------------------------------------------------
// F_p, p < 2^32.

#[derive(Clone, Copy, Debug)]
struct Fp {
    p: u64,
}

impl Fp {
    fn red_i(&self, v: i128) -> u64 {
        v.rem_euclid(self.p as i128) as u64
    }
    fn add(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    fn neg(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        a * b % self.p
    }
    fn pow(&self, mut a: u64, mut e: u64) -> u64 {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        self.pow(a, self.p - 2)
    }
    /// Legendre symbol as 0, 1, −1.
    fn legendre(&self, a: u64) -> i32 {
        if a == 0 {
            return 0;
        }
        if self.pow(a, (self.p - 1) / 2) == 1 {
            1
        } else {
            -1
        }
    }
    /// Tonelli–Shanks.
    fn sqrt(&self, a: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        let p = self.p;
        let mut q = p - 1;
        let mut s = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        if s == 1 {
            return Some(self.pow(a, (p + 1) / 4));
        }
        let z = (2..p).find(|&z| self.legendre(z) == -1).unwrap();
        let mut m = s;
        let mut c = self.pow(z, q);
        let mut t = self.pow(a, q);
        let mut r = self.pow(a, q.div_ceil(2));
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = self.mul(tt, tt);
                i += 1;
            }
            let mut b = c;
            for _ in 0..(m - i - 1) {
                b = self.mul(b, b);
            }
            m = i;
            c = self.mul(b, b);
            t = self.mul(t, c);
            r = self.mul(r, b);
        }
        Some(r)
    }
    fn least_nonresidue(&self) -> u64 {
        (2..self.p).find(|&d| self.legendre(d) == -1).unwrap()
    }
}

// ---------------------------------------------------------------------------
// Short Weierstrass y² = x³ + ax + b, affine points, `None` = ∞.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pt {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug)]
struct Curve {
    f: Fp,
    a: u64,
    b: u64,
}

impl Curve {
    fn rhs(&self, x: u64) -> u64 {
        let f = &self.f;
        f.add(f.mul(f.add(f.mul(x, x), self.a), x), self.b)
    }
    fn on(&self, q: &Pt) -> bool {
        self.f.mul(q.y, q.y) == self.rhs(q.x)
    }
    fn neg(&self, q: &Pt) -> Pt {
        Pt {
            x: q.x,
            y: self.f.neg(q.y),
        }
    }
    fn add(&self, p: Option<Pt>, q: Option<Pt>) -> Option<Pt> {
        let f = &self.f;
        let (p, q) = match (p, q) {
            (None, q) => return q,
            (p, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        let lam = if p.x == q.x {
            if f.add(p.y, q.y) == 0 {
                return None;
            }
            let num = f.add(f.mul(3, f.mul(p.x, p.x)), self.a);
            f.mul(num, f.inv(f.mul(2, p.y)))
        } else {
            f.mul(f.sub(q.y, p.y), f.inv(f.sub(q.x, p.x)))
        };
        let x3 = f.sub(f.sub(f.mul(lam, lam), p.x), q.x);
        let y3 = f.sub(f.mul(lam, f.sub(p.x, x3)), p.y);
        Some(Pt { x: x3, y: y3 })
    }
    fn mul(&self, p: &Pt, mut k: u64) -> Option<Pt> {
        let mut acc: Option<Pt> = None;
        let mut base = Some(*p);
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(acc, base);
            }
            base = self.add(base, base);
            k >>= 1;
        }
        acc
    }
    fn discriminant(&self) -> u64 {
        let f = &self.f;
        let a3 = f.mul(f.mul(self.a, self.a), self.a);
        f.add(f.mul(4, a3), f.mul(27, f.mul(self.b, self.b)))
    }
    fn j(&self) -> u64 {
        let f = &self.f;
        let a3 = f.mul(f.mul(self.a, self.a), self.a);
        f.mul(f.mul(1728 * 4 % f.p, a3), f.inv(self.discriminant()))
    }
    fn key(q: &Pt) -> u64 {
        (q.x << 32) | q.y
    }
    fn random_point(&self, rng: &mut StdRng) -> Pt {
        loop {
            let x = rng.gen_range(0..self.f.p);
            let r = self.rhs(x);
            if r == 0 {
                continue;
            }
            if let Some(y) = self.f.sqrt(r) {
                return Pt { x, y };
            }
        }
    }
    fn to_model(self, lf: &LField) -> Model {
        Model {
            a: lf.from_u64(self.a),
            b: lf.from_u64(self.b),
        }
    }
}

// ---------------------------------------------------------------------------
// Instances.

/// `j`-invariants of the class-number-one discriminants used (`|D|`, `j`).
const CM_J: [(u64, i128); 5] = [
    (11, -32768),
    (19, -884736),
    (43, -884736000),
    (67, -147197952000),
    (163, -262537412640768000),
];

fn kronecker_odd(d: i128, ell: u64) -> i32 {
    // (D/ℓ) for odd prime ℓ, via Euler's criterion.
    let f = Fp { p: ell };
    f.legendre(f.red_i(d))
}

#[derive(Clone, Debug)]
struct Inst {
    ell: u64,
    /// `|D|`.
    d: u64,
    j0: u64,
    p: u64,
    t: i64,
    v: u64,
    w: u64,
    n: u64,
    /// Frobenius eigenvalue `t/2 mod ℓ` on the crater's `E[ℓ]`.
    lambda: u64,
    /// `(D/ℓ)`.
    kron: i32,
}

fn find_instance(ell: u64, d: u64, lo: u64, hi: u64, rng: &mut StdRng) -> (Inst, u64) {
    let (_, j_int) = CM_J.iter().find(|(dd, _)| *dd == d).expect("known D");
    let mut tries = 0u64;
    loop {
        tries += 1;
        let four_hi = 4 * hi as u128;
        let wmax = (((four_hi / d as u128) as f64).sqrt() as u64) / ell;
        let w = rng.gen_range(1..=wmax.max(1));
        if w % 2 == 0 || w % ell == 0 {
            continue;
        }
        let v = ell * w;
        let sv = d as u128 * (v as u128) * (v as u128);
        if sv >= four_hi {
            continue;
        }
        let tmax = ((four_hi - 1 - sv) as f64).sqrt() as u64;
        let four_lo = 4 * lo as u128;
        let tmin = if sv >= four_lo {
            1
        } else {
            ((four_lo - sv) as f64).sqrt().ceil() as u64
        };
        if tmin > tmax {
            continue;
        }
        let t = rng.gen_range(tmin..=tmax);
        if t % 2 == 0 || t % ell == 2 % ell {
            continue;
        }
        let four_p = (t as u128) * (t as u128) + sv;
        if !four_p.is_multiple_of(4) {
            continue;
        }
        let p = (four_p / 4) as u64;
        if p < lo || p >= hi || !is_prime_u64(p) {
            continue;
        }
        let n = p + 1 - t;
        if !is_prime_u64(n) {
            continue;
        }
        let f = Fp { p };
        let j0 = f.red_i(*j_int);
        if j0 == 0 || j0 == 1728 % p {
            continue;
        }
        let lambda = (t % ell) * ell.div_ceil(2) % ell; // t/2 mod ℓ
        assert!(lambda != 1);
        assert!(!n.is_multiple_of(ell), "ℓ ∤ N by construction");
        return (
            Inst {
                ell,
                d,
                j0,
                p,
                t: t as i64,
                v,
                w,
                n,
                lambda,
                kron: kronecker_odd(-(d as i128), ell),
            },
            tries,
        );
    }
}

/// The curve with `j`-invariant `j` and `#E = N` (the unique such twist for
/// `j ∉ {0, 1728}`), certified by `[N]P = O` on `checks` random points.
fn curve_from_j(f: Fp, j: u64, n: u64, checks: usize, rng: &mut StdRng) -> Option<(Curve, bool)> {
    let k = f.sub(1728 % f.p, j);
    let a0 = f.mul(3, f.mul(j, k));
    let b0 = f.mul(2, f.mul(j, f.mul(k, k)));
    let d = f.least_nonresidue();
    for twisted in [false, true] {
        let (a, b) = if twisted {
            (f.mul(a0, f.mul(d, d)), f.mul(b0, f.mul(d, f.mul(d, d))))
        } else {
            (a0, b0)
        };
        let cv = Curve { f, a, b };
        assert!(cv.discriminant() != 0);
        assert_eq!(cv.j(), j);
        let ok = (0..checks).all(|_| {
            let q = cv.random_point(rng);
            cv.mul(&q, n).is_none()
        });
        if ok {
            return Some((cv, twisted));
        }
    }
    None
}

fn fe_to_u64(lf: &LField, x: &crypto_lib::cryptanalysis::isogeny_walk::field::Fe) -> u64 {
    lf.to_big(x).to_u64().unwrap()
}

/// Distinct `F_p`-roots of `Φ_ℓ(j, Y)`.
fn phi_roots(lf: &LField, phi: &modpoly::ModPoly, j: u64) -> Vec<u64> {
    let py = phi.at_x(lf, &lf.from_u64(j));
    let mut r: Vec<u64> = lpoly::roots(lf, &py, 0x6011)
        .iter()
        .map(|x| fe_to_u64(lf, x))
        .collect();
    r.sort_unstable();
    r
}

// ---------------------------------------------------------------------------
// Bases and census.

/// Signed base of `size` points by rule: `xs` is the order in which
/// x-coordinates are tried.  Returns the base and the number of x tried.
fn base_from_xs(cv: &Curve, size: usize, xs: impl Iterator<Item = u64>) -> (Vec<Pt>, u64) {
    let mut out = Vec::with_capacity(size);
    let mut tried = 0;
    for x in xs {
        if out.len() >= size {
            break;
        }
        tried += 1;
        let r = cv.rhs(x);
        if r == 0 {
            continue;
        }
        if let Some(y) = cv.f.sqrt(r) {
            let q = Pt { x, y };
            out.push(q);
            out.push(cv.neg(&q));
        }
    }
    assert_eq!(out.len(), size, "base rule ran out of x");
    (out, tried)
}

#[derive(Clone, Debug)]
struct MStat {
    distinct: u64,
    distinct_strict: u64,
    representations: u64,
    zero_sums: u64,
    c: f64,
    c_birthday: f64,
    per_unknown: f64,
    cost_model: f64,
}

impl MStat {
    fn js(&self) -> Value {
        json!({
            "distinct": self.distinct,
            "distinct_strict": self.distinct_strict,
            "representations": self.representations,
            "zero_sums": self.zero_sums,
            "c": self.c,
            "c_birthday_derived": self.c_birthday,
            "decomposable_per_unknown": self.per_unknown,
            "cost_model": self.cost_model,
        })
    }
}

fn mstat(d: usize, ds: usize, reps: u64, zeros: u64, n: usize, m: u32, r: u64, l: f64) -> MStat {
    let fact = if m == 2 { 2.0 } else { 6.0 };
    let nm = (n as f64).powi(m as i32);
    let u = n as f64 / 2.0;
    let lam = nm / (fact * r as f64);
    MStat {
        distinct: d as u64,
        distinct_strict: ds as u64,
        representations: reps,
        zero_sums: zeros,
        c: d as f64 * fact / nm,
        c_birthday: (1.0 - (-lam).exp()) / lam,
        per_unknown: d as f64 / u,
        cost_model: if d > 0 {
            u * (r as f64 / d as f64) * l
        } else {
            f64::NAN
        },
    }
}

/// Exact census of `m = 2, 3` multiset sums of the signed base `base`
/// over the prime-order group `G = E(F_p)` (`h = 1`).
fn census(cv: &Curve, base: &[Pt], r: u64) -> (MStat, MStat) {
    let n = base.len();
    let keys: HashSet<u64> = base.iter().map(Curve::key).collect();
    assert_eq!(keys.len(), n, "duplicate base elements");
    for q in base {
        assert!(cv.on(q), "base point off the curve");
        assert!(keys.contains(&Curve::key(&cv.neg(q))), "base not signed");
    }
    let mut pairs: Vec<(usize, usize, Option<Pt>)> = Vec::with_capacity(n * (n + 1) / 2);
    let mut hits2: HashSet<u64> = HashSet::new();
    let mut hits2s: HashSet<u64> = HashSet::new();
    let (mut reps2, mut zero2) = (0u64, 0u64);
    for i in 0..n {
        for j in i..n {
            let s = cv.add(Some(base[i]), Some(base[j]));
            match s {
                None => zero2 += 1,
                Some(q) => {
                    hits2.insert(Curve::key(&q));
                    if i < j {
                        hits2s.insert(Curve::key(&q));
                    }
                    reps2 += 1;
                }
            }
            pairs.push((i, j, s));
        }
    }
    let mut hits3: HashSet<u64> = HashSet::new();
    let mut hits3s: HashSet<u64> = HashSet::new();
    let (mut reps3, mut zero3) = (0u64, 0u64);
    for (i, j, s) in &pairs {
        for (k, bk) in base.iter().enumerate().skip(*j) {
            match cv.add(*s, Some(*bk)) {
                None => zero3 += 1,
                Some(q) => {
                    hits3.insert(Curve::key(&q));
                    if i < j && *j < k {
                        hits3s.insert(Curve::key(&q));
                    }
                    reps3 += 1;
                }
            }
        }
    }
    (
        mstat(hits2.len(), hits2s.len(), reps2, zero2, n, 2, r, 1.0),
        mstat(hits3.len(), hits3s.len(), reps3, zero3, n, 3, r, n as f64),
    )
}

// ---------------------------------------------------------------------------
// One level (one curve) of one instance.

/// Everything a level record needs from its instance.
struct Ctx<'a> {
    inst: &'a Inst,
    lf: &'a LField,
    phi: &'a modpoly::ModPoly,
    sizes: &'a [usize],
    /// Seeded random order of x for the `randx` rule, shared by all curves
    /// of the instance.
    xorder: &'a [u64],
}

fn level_record(
    cx: &Ctx,
    label: &str,
    cv: &Curve,
    twisted: bool,
    rng: &mut StdRng,
    velu: Value,
) -> Value {
    let (inst, lf, phi, sizes, xorder) = (cx.inst, cx.lf, cx.phi, cx.sizes, cx.xorder);
    let roots = phi_roots(lf, phi, cv.j());
    let mut rows = Vec::new();
    for &s in sizes {
        for rule in ["smallx", "randx"] {
            let (base, tried) = if rule == "smallx" {
                base_from_xs(cv, s, 0..cv.f.p)
            } else {
                base_from_xs(cv, s, xorder.iter().copied())
            };
            let (m2, m3) = census(cv, &base, inst.n);
            rows.push(json!({
                "rule": rule, "size": s, "x_tried": tried, "columns": s / 2,
                "m2": m2.js(), "m3": m3.js(),
            }));
        }
    }
    // |Aut| = 2 for j ∉ {0, 1728}.
    let j = cv.j();
    let aut = if j == 0 {
        6
    } else if j == 1728 % cv.f.p {
        4
    } else {
        2
    };
    let order_checks = (0..4).all(|_| cv.mul(&cv.random_point(rng), inst.n).is_none());
    json!({
        "level": label,
        "j": j,
        "a": cv.a, "b": cv.b, "twisted": twisted,
        "aut_order": aut,
        "order_certified": order_checks,
        "n_mod_ell": inst.n % inst.ell,
        "abs_d_inst": inst.d,
        "phi_roots": roots,
        "n_phi_roots": roots.len(),
        "velu_certificate": velu,
        "census": rows,
    })
}

fn run_instance(ell: u64, d: u64, idx: u64, seed: u64, sizes: &[usize]) -> Value {
    let mut rng = StdRng::seed_from_u64(seed ^ (ell << 32) ^ (idx << 16) ^ d);
    let (inst, tries) = find_instance(ell, d, 1 << 20, 1 << 22, &mut rng);
    let f = Fp { p: inst.p };
    eprintln!(
        "ℓ={ell} D=−{d} #{idx}: p={} t={} v={} (w={}) N={} λ={} (D/ℓ)={} after {tries} draws",
        inst.p, inst.t, inst.v, inst.w, inst.n, inst.lambda, inst.kron
    );
    let (e0, tw0) = curve_from_j(f, inst.j0, inst.n, 4, &mut rng)
        .expect("CM j must give a twist with #E = N (checks the j constant)");
    let lf = LField::new(&BigUint::from(inst.p)).expect("prime field");
    let phi = modpoly::modular_polynomial(&lf, ell).expect("Φ_ℓ");
    let roots0 = phi_roots(&lf, &phi, inst.j0);
    let floors: Vec<u64> = roots0.iter().copied().filter(|&r| r != inst.j0).collect();
    let expected_crater = (ell as i64 - inst.kron as i64 + i64::from(inst.kron == 1)) as usize;
    assert_eq!(
        roots0.len(),
        expected_crater,
        "crater must have ℓ − (D/ℓ) + [(D/ℓ)=1] distinct Φ_ℓ roots"
    );
    let mut xorder: Vec<u64> = (0..inst.p).collect();
    xorder.shuffle(&mut rng);
    let m0 = e0.to_model(&lf);
    let cx = Ctx {
        inst: &inst,
        lf: &lf,
        phi: &phi,
        sizes,
        xorder: &xorder,
    };
    let crater = level_record(
        &cx,
        "crater",
        &e0,
        tw0,
        &mut rng,
        json!({"note": "domain of the Elkies isogenies below"}),
    );
    let mut floor_recs = Vec::new();
    for &j1 in &floors {
        let (e1, tw1) = curve_from_j(f, j1, inst.n, 4, &mut rng)
            .expect("floor j must give a twist with #E = N (isogenous curves have equal order)");
        let velu = match kernel::elkies_isogeny(&lf, &m0, &phi, &lf.from_u64(j1)) {
            Ok(iso) => {
                let cj = iso.codomain.j(&lf).map(|x| fe_to_u64(&lf, &x));
                let iso_u2 = curve::isomorphism(&lf, &iso.codomain, &e1.to_model(&lf))
                    .map(|s| fe_to_u64(&lf, &s));
                json!({
                    "elkies_kernel_degree": iso.kernel.len().saturating_sub(1),
                    "velu_codomain_j": cj,
                    "codomain_j_matches": cj == Some(j1),
                    "fp_isomorphic_to_floor_model": iso_u2.is_some(),
                    "isomorphism_u2": iso_u2,
                    "certified": cj == Some(j1) && iso_u2.is_some(),
                })
            }
            Err(e) => json!({"error": format!("{e:?}"), "certified": false}),
        };
        floor_recs.push(level_record(&cx, "floor", &e1, tw1, &mut rng, velu));
    }
    json!({
        "lead": "R6.11",
        "ell": ell,
        "D": -(d as i64),
        "instance": idx,
        "seed": seed,
        "p": inst.p,
        "log2_p": (inst.p as f64).log2(),
        "t": inst.t,
        "v": inst.v,
        "w": inst.w,
        "v_ell_conductor": 1,
        "N": inst.n,
        "h": 1,
        "r": inst.n,
        "lambda_frobenius_mod_ell": inst.lambda,
        "kronecker_D_ell": inst.kron,
        "j_crater": inst.j0,
        "phi_checks_passed": phi.checks_passed,
        "crater_phi_roots_expected": expected_crater,
        "n_floor_curves": floors.len(),
        "floor_curves_expected": ell as i64 - inst.kron as i64,
        "sizes": sizes,
        "search_draws": tries,
        "crater": crater,
        "floors": floor_recs,
    })
}

// ---------------------------------------------------------------------------
// Summary over instances.

fn get_c(level: &Value, rule: &str, size: u64, m: &str) -> Option<f64> {
    level["census"]
        .as_array()?
        .iter()
        .find(|r| r["rule"] == rule && r["size"] == size)
        .and_then(|r| r[m]["c"].as_f64())
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn std_sample(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return f64::NAN;
    }
    let mu = mean(v);
    (v.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
}

fn summarise(dir: &str) -> Value {
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("dir")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "json")
                && p.file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with("r6_11_ell"))
        })
        .collect();
    files.sort();
    let recs: Vec<Value> = files
        .iter()
        .map(|p| serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap())
        .collect();
    let mut ells: Vec<u64> = recs.iter().filter_map(|r| r["ell"].as_u64()).collect();
    ells.sort_unstable();
    ells.dedup();
    let sizes: Vec<u64> = recs[0]["sizes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_u64().unwrap())
        .collect();
    let mut rows = Vec::new();
    let mut certified_all = true;
    let mut level_evidence = Vec::new();
    for r in &recs {
        let fl = r["floors"].as_array().unwrap();
        for f in fl {
            if f["velu_certificate"]["certified"] != true {
                certified_all = false;
            }
        }
        level_evidence.push(json!({
            "ell": r["ell"], "D": r["D"], "instance": r["instance"], "p": r["p"], "N": r["N"],
            "j_crater": r["j_crater"],
            "crater_phi_roots": r["crater"]["n_phi_roots"],
            "crater_phi_roots_expected": r["crater_phi_roots_expected"],
            "floor_phi_roots": fl.iter().map(|f| f["n_phi_roots"].clone()).collect::<Vec<_>>(),
            "floor_j": fl.iter().map(|f| f["j"].clone()).collect::<Vec<_>>(),
            "n_floor": fl.len(),
            "floor_curves_expected": r["floor_curves_expected"],
            "velu_certified": fl.iter().map(|f| f["velu_certificate"]["certified"].clone()).collect::<Vec<_>>(),
            "aut_crater": r["crater"]["aut_order"],
            "aut_floor": fl.iter().map(|f| f["aut_order"].clone()).collect::<Vec<_>>(),
            "order_certified": fl.iter().chain(std::iter::once(&r["crater"])).all(|f| f["order_certified"] == true),
        }));
    }
    let mut falsifier_fired = false;
    for &ell in &ells {
        for rule in ["smallx", "randx"] {
            for &size in &sizes {
                for m in ["m2", "m3"] {
                    let mut per = Vec::new();
                    let mut ratio0 = Vec::new();
                    for r in recs.iter().filter(|r| r["ell"] == ell) {
                        let cc = get_c(&r["crater"], rule, size, m).unwrap();
                        let cf: Vec<f64> = r["floors"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|f| get_c(f, rule, size, m).unwrap())
                            .collect();
                        let fmin = cf.iter().cloned().fold(f64::INFINITY, f64::min);
                        let fmax = cf.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                        let fmean = mean(&cf);
                        let fstd = std_sample(&cf);
                        let rho0 = cc / cf[0];
                        ratio0.push(rho0);
                        per.push(json!({
                            "instance": r["instance"], "D": r["D"], "p": r["p"],
                            "c_crater": cc, "c_floor0": cf[0], "c_floor_mean": fmean,
                            "c_floor_min": fmin, "c_floor_max": fmax, "c_floor_std": fstd,
                            "n_floor": cf.len(),
                            "ratio_crater_floor0": rho0,
                            "ratio_crater_floor_mean": cc / fmean,
                            "crater_inside_floor_range": cc >= fmin && cc <= fmax,
                            "z_within": (cc - fmean) / fstd,
                        }));
                    }
                    let mu = mean(&ratio0);
                    let sd = std_sample(&ratio0);
                    let n_out = ratio0
                        .iter()
                        .filter(|x| (*x - 1.0).abs() > 2.0 * sd)
                        .count();
                    let n_outside_range = per
                        .iter()
                        .filter(|q| q["crater_inside_floor_range"] == false)
                        .count();
                    let n_z3 = per
                        .iter()
                        .filter(|q| q["z_within"].as_f64().is_some_and(|z| z.abs() > 3.0))
                        .count();
                    let fired = n_out >= 2 && n_z3 >= 2;
                    if rule == "smallx" && size <= 100 {
                        falsifier_fired |= fired;
                    }
                    rows.push(json!({
                        "ell": ell, "rule": rule, "size": size, "m": m,
                        "n_instances": ratio0.len(),
                        "ratio0_mean": mu, "ratio0_std": sd,
                        "ratio0_min": ratio0.iter().cloned().fold(f64::INFINITY, f64::min),
                        "ratio0_max": ratio0.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                        "interval_2sd": [mu - 2.0 * sd, mu + 2.0 * sd],
                        "n_outside_2sd_of_1": n_out,
                        "n_crater_outside_floor_range": n_outside_range,
                        "n_abs_z_within_gt_3": n_z3,
                        "falsifier_fired_here": fired,
                        "instances": per,
                    }));
                }
            }
        }
    }
    json!({
        "lead": "R6.11",
        "files": files.iter().map(|p| p.file_name().unwrap().to_string_lossy().to_string()).collect::<Vec<_>>(),
        "n_instances": recs.len(),
        "ells": ells,
        "sizes": sizes,
        "all_vertical_edges_certified": certified_all,
        "level_evidence": level_evidence,
        "falsifier": "fires when, at some (ℓ, m, |F| ≤ 100, smallx), at least two instances have |c_crater/c_floor − 1| > 2·(across-instance sd of the ratio) and |z_within| > 3 (z = (c_crater − mean floor c)/sd over the same-class floor curves)",
        "falsifier_fired": falsifier_fired,
        "rows": rows,
    })
}

// ---------------------------------------------------------------------------

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("help");
    match mode {
        "run" => {
            let ell: u64 = arg(&args, "--ell", "5").parse().unwrap();
            let n_inst: u64 = arg(&args, "--instances", "6").parse().unwrap();
            let seed: u64 = arg(&args, "--seed", "1").parse().unwrap();
            let out = arg(&args, "--out", ".");
            let sizes: Vec<usize> = arg(&args, "--sizes", "60,80,100,200")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            fs::create_dir_all(&out).unwrap();
            let ds: Vec<u64> = CM_J
                .iter()
                .map(|(d, _)| *d)
                .filter(|d| d % ell != 0)
                .collect();
            for idx in 0..n_inst {
                let d = ds[idx as usize % ds.len()];
                let rec = run_instance(ell, d, idx, seed, &sizes);
                let path = format!("{out}/r6_11_ell{ell}_i{idx}.json");
                fs::write(&path, serde_json::to_string_pretty(&rec).unwrap()).unwrap();
                let c = &rec["crater"]["census"][2];
                let f0 = &rec["floors"][0]["census"][2];
                eprintln!(
                    "  wrote {path}: floors={} |F|=100 smallx c2 crater {:.5} floor0 {:.5}; c3 {:.5} / {:.5}",
                    rec["n_floor_curves"],
                    c["m2"]["c"].as_f64().unwrap(),
                    f0["m2"]["c"].as_f64().unwrap(),
                    c["m3"]["c"].as_f64().unwrap(),
                    f0["m3"]["c"].as_f64().unwrap(),
                );
            }
        }
        "summarise" => {
            let dir = arg(&args, "--dir", ".");
            let out = arg(&args, "--out", "summary.json");
            let s = summarise(&dir);
            fs::write(&out, serde_json::to_string_pretty(&s).unwrap()).unwrap();
            eprintln!(
                "summary: {} instances, vertical edges certified: {}, falsifier fired: {}",
                s["n_instances"], s["all_vertical_edges_certified"], s["falsifier_fired"]
            );
        }
        _ => {
            eprintln!(
                "usage: r6_11_conductor_yield run --ell L --instances N --seed S --out DIR [--sizes 60,80,100,200]\n       r6_11_conductor_yield summarise --dir DIR --out summary.json"
            );
        }
    }
}
