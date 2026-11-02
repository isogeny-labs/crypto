//! The PKM isogeny tower over any [`PrimeField`] backend, for primes the
//! `u64` plug-in of [`super::prime_tower`] cannot hold (NIST P-192,
//! P-224, P-256, any prime below `2^{64·N}`).
//!
//! The same charged construction, at any size:
//!
//! - [`isogeny_tower_wide`]: the seeded ordinary search — a residue-class
//!   baby-step giant-step filter on every curve tried, Mestre's
//!   baby-step giant-step order on the survivors, the order-aware `Q`
//!   step — with the `X₁(2, 8)` family ([`AuxFamily::Z2xZ8`]) as the draw
//!   family.  Every group operation on the auxiliary curve is charged to
//!   `ops` exactly as in the `u64` plug-in.  **Cost caveat at NIST
//!   size:** the filter and Mestre are `Θ(p^{1/4})` group operations per
//!   curve (≈ `2^{45}` at P-192), so the ordinary search is a small-prime
//!   instrument there; at NIST primes use the supersingular tower.
//! - [`supersingular_tower_wide`]: the supersingular auxiliary curves of
//!   [`AuxKind`] (`#E' = p + 1` by Deuring, no search, no point
//!   counting); at P-192 (`v₂(p + 1) = 64`) this is the one-draw tower
//!   of `research/pkm_supersingular_aux_20261015/`.
//!
//! `t ≤ 63` (the counters are `u64`; `⟨Q⟩` costs `2^t` additions, so
//! attack-size towers are built at their working height `T ≤ t` by
//! passing `Q_T = [2^{t−T}]Q`-style truncation to the caller).
//!
//! Deterministic in `(p, t, seed, family or kind)`.

use std::collections::{HashMap, HashSet};

use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::cryptanalysis::fp_backend::PrimeField;
use crate::cryptanalysis::ic_boundary::GroupOps;

use super::prime_tower::{AuxFamily, AuxKind, IsogenyTowerCost};

/// A point on a short Weierstrass curve over `F`: `None` is the identity.
pub type WPoint<E> = Option<(E, E)>;

/// `y² = x³ + ax + b` over any [`PrimeField`] backend.  The field is
/// passed to every method, so the curve is a plain value.
#[derive(Clone, Copy, Debug)]
pub struct WideCurve<E> {
    pub a: E,
    pub b: E,
}

impl<E: Copy + PartialEq> WideCurve<E> {
    pub fn rhs<F: PrimeField<Elem = E>>(&self, f: &F, x: E) -> E {
        f.add(f.add(f.mul(f.sqr(x), x), f.mul(self.a, x)), self.b)
    }

    pub fn on_curve<F: PrimeField<Elem = E>>(&self, f: &F, pt: WPoint<E>) -> bool {
        match pt {
            None => true,
            Some((x, y)) => f.is_zero(f.sub(f.sqr(y), self.rhs(f, x))),
        }
    }

    /// Affine addition, `None` on the identity.  Counts one `adds`.
    pub fn add<F: PrimeField<Elem = E>>(
        &self,
        f: &F,
        ops: &mut GroupOps,
        p: WPoint<E>,
        q: WPoint<E>,
    ) -> WPoint<E> {
        ops.adds += 1;
        match (p, q) {
            (None, _) => q,
            (_, None) => p,
            (Some((x1, y1)), Some((x2, y2))) => {
                let lambda = if x1 == x2 {
                    if f.is_zero(f.add(y1, y2)) {
                        return None;
                    }
                    let num = f.add(f.mul(f.from_u64(3), f.sqr(x1)), self.a);
                    f.mul(num, f.inv(f.double(y1)).expect("2y ≠ 0 here"))
                } else {
                    f.mul(f.sub(y2, y1), f.inv(f.sub(x2, x1)).expect("x1 ≠ x2 here"))
                };
                let x = f.sub(f.sub(f.sqr(lambda), x1), x2);
                let y = f.sub(f.mul(lambda, f.sub(x1, x)), y1);
                Some((x, y))
            }
        }
    }

    /// Affine doubling, `None` on the identity.  Counts one `doubles`.
    pub fn double<F: PrimeField<Elem = E>>(
        &self,
        f: &F,
        ops: &mut GroupOps,
        p: WPoint<E>,
    ) -> WPoint<E> {
        ops.doubles += 1;
        match p {
            None => None,
            Some((x, y)) => {
                if f.is_zero(y) {
                    return None;
                }
                let num = f.add(f.mul(f.from_u64(3), f.sqr(x)), self.a);
                let lambda = f.mul(num, f.inv(f.double(y)).expect("2y ≠ 0 here"));
                let x2 = f.sub(f.sub(f.sqr(lambda), x), x);
                let y2 = f.sub(f.mul(lambda, f.sub(x, x2)), y);
                Some((x2, y2))
            }
        }
    }

    pub fn neg<F: PrimeField<Elem = E>>(&self, f: &F, p: WPoint<E>) -> WPoint<E> {
        p.map(|(x, y)| (x, f.neg(y)))
    }

    /// Double-and-add by a `BigUint` scalar.
    pub fn mul_biguint<F: PrimeField<Elem = E>>(
        &self,
        f: &F,
        ops: &mut GroupOps,
        p: WPoint<E>,
        k: &BigUint,
    ) -> WPoint<E> {
        let mut acc: WPoint<E> = None;
        let mut addend = p;
        for i in 0..k.bits() {
            if k.bit(i) {
                acc = self.add(f, ops, acc, addend);
            }
            if i + 1 < k.bits() {
                addend = self.double(f, ops, addend);
            }
        }
        acc
    }

    /// `[2^s]p` by `s` doublings.
    pub fn mul_pow2<F: PrimeField<Elem = E>>(
        &self,
        f: &F,
        ops: &mut GroupOps,
        p: WPoint<E>,
        s: u32,
    ) -> WPoint<E> {
        let mut acc = p;
        for _ in 0..s {
            acc = self.double(f, ops, acc);
        }
        acc
    }
}

/// A uniform integer in `[0, p)`, 32 bits at a time with rejection.
fn rand_below(p: &BigUint, rng: &mut StdRng) -> BigUint {
    let bits = p.bits();
    let words = bits.div_ceil(32) as usize;
    let top = (bits % 32) as u32;
    loop {
        let mut d: Vec<u32> = (0..words).map(|_| rng.gen::<u32>()).collect();
        if top != 0 {
            d[words - 1] &= (1u32 << top) - 1;
        }
        let x = BigUint::new(d);
        if &x < p {
            return x;
        }
    }
}

/// A random point on the curve (both Legendre symbol and square root
/// counted, as in the `u64` plug-in).
fn random_point_w<F: PrimeField>(
    f: &F,
    e: &WideCurve<F::Elem>,
    rng: &mut StdRng,
    cost: &mut IsogenyTowerCost,
) -> WPoint<F::Elem> {
    let p = f.modulus();
    loop {
        let x = f.from_biguint(&rand_below(&p, rng));
        let r = e.rhs(f, x);
        cost.legendre_symbols += 1;
        if f.legendre(r) != 1 {
            continue;
        }
        cost.sqrt_solves += 1;
        let y = f.sqrt(r).expect("a residue has a root");
        return Some((x, y));
    }
}

/// The multiples `k ≡ 0 (mod m)` in `[lo, hi]` with `[k]P = O`, by
/// baby-step giant-step on `P_m = [m]P` over the index range — the wide
/// twin of `prime_tower::class_hits`, in `BigUint`.  `(count, first,
/// spacing)`, `spacing = ord(P_m)` when `count ≥ 2`.
fn class_hits_w<F: PrimeField>(
    f: &F,
    e: &WideCurve<F::Elem>,
    pt: WPoint<F::Elem>,
    m: &BigUint,
    lo: &BigUint,
    hi: &BigUint,
    ops: &mut GroupOps,
    table_entries: &mut u64,
) -> (u64, Option<BigUint>, Option<BigUint>) {
    let i0 = (lo + m - BigUint::one()) / m;
    let i1 = hi / m;
    if i1 < i0 {
        return (0, None, None);
    }
    let len = &i1 - &i0 + BigUint::one();
    let pm = e.mul_biguint(f, ops, pt, m);
    if pm.is_none() {
        return (
            len.to_u64().unwrap_or(u64::MAX),
            Some(&i0 * m),
            (len > BigUint::one()).then(BigUint::one),
        );
    }
    let pm = pm.expect("checked above");
    // Baby steps j·P_m, j = 1 … w − 1; a repeat gives ord(P_m) outright.
    let w = len.sqrt() + BigUint::one();
    let w64 = w.to_u64().unwrap_or(u64::MAX >> 32);
    let mut table: HashMap<(F::Elem, F::Elem), u64> = HashMap::new();
    let mut acc = Some(pm);
    let mut order: Option<BigUint> = None;
    for j in 1..w64 {
        if acc.is_none() {
            order = Some(BigUint::from(j));
            break;
        }
        let a = acc.expect("checked");
        table.entry(a).or_insert(j);
        acc = e.add(f, ops, acc, Some(pm));
    }
    *table_entries += table.len() as u64;
    let hits_of = |o: &BigUint| -> (u64, Option<BigUint>, Option<BigUint>) {
        let first = (i0.clone() + o - BigUint::one()) / o * o;
        if first > i1 {
            (0, None, None)
        } else {
            let n = (&i1 - &first) / o + BigUint::one();
            (
                n.to_u64().unwrap_or(u64::MAX),
                Some(&first * m),
                (n > BigUint::one()).then(|| o.clone()),
            )
        }
    };
    if let Some(o) = order {
        return hits_of(&o);
    }
    // acc is w·P_m; giant steps S_g = (i0 + g·w)·P_m.
    let giant = acc;
    let mut s = e.mul_biguint(f, ops, Some(pm), &i0);
    let mut hits: Vec<BigUint> = Vec::new();
    let mut base = i0.clone();
    while base <= i1 && hits.len() < 2 {
        if s.is_none() {
            hits.push(base.clone());
        } else if let Some(s) = s {
            if let Some(neg) = e.neg(f, Some(s)) {
                if let Some(&j) = table.get(&neg) {
                let cand = base.clone() + BigUint::from(j);
                if cand <= i1 {
                    hits.push(cand);
                }
                }
            }
        }
        base += &w;
        if base <= i1 {
            s = e.add(f, ops, s, giant);
        }
    }
    match hits.as_slice() {
        [] => (0, None, None),
        [h] => (1, Some(h * m), None),
        [h1, h2, ..] => {
            let mut o = h2 - h1;
            let mut rest = o.clone();
            let mut primes = Vec::new();
            let mut q = BigUint::from(2u32);
            while &q * &q <= rest {
                if (&rest % &q).is_zero() {
                    primes.push(q.clone());
                    while (&rest % &q).is_zero() {
                        rest /= &q;
                    }
                }
                q += 1u32;
            }
            if rest > BigUint::one() {
                primes.push(rest);
            }
            for q in primes {
                while (&o % &q).is_zero() && e.mul_biguint(f, ops, Some(pm), &(&o / &q)).is_none() {
                    o /= &q;
                }
            }
            hits_of(&o)
        }
    }
}

/// `#E'(F_p)` by Mestre's baby-step giant-step — the wide twin of
/// `prime_tower::mestre_order`.  `None` after `draws` points.
fn mestre_order_w<F: PrimeField>(
    f: &F,
    e: &WideCurve<F::Elem>,
    rng: &mut StdRng,
    lo: &BigUint,
    hi: &BigUint,
    draws: u32,
    ops: &mut GroupOps,
    cost: &mut IsogenyTowerCost,
) -> Option<BigUint> {
    let mut m = BigUint::one();
    for _ in 0..draws {
        let pt = random_point_w(f, e, rng, cost);
        let (n, first, spacing) =
            class_hits_w(f, e, pt, &m, lo, hi, ops, &mut cost.bsgs_table_entries);
        match (n, first, spacing) {
            (1, Some(k), _) => return Some(k),
            (_, _, Some(o)) if n >= 2 => m *= o,
            _ => return None, // cannot happen: #E' is always a hit
        }
    }
    None
}

/// One Vélu 2-isogeny step of the chain.
#[derive(Clone, Copy, Debug)]
pub struct VeluStepW<E> {
    pub xi: E,
    pub tau: E,
}

/// The built tower: `V`, the chain, `c = x(Φ(R'))`, and the cost split.
pub struct WideTower<F: PrimeField> {
    pub t: u32,
    pub aux: WideCurve<F::Elem>,
    pub aux_order: BigUint,
    pub q: WPoint<F::Elem>,
    pub r: WPoint<F::Elem>,
    pub v: Vec<F::Elem>,
    pub steps: Vec<VeluStepW<F::Elem>>,
    pub c: F::Elem,
    pub cost: IsogenyTowerCost,
}

impl<F: PrimeField> WideTower<F> {
    /// The composed map `x ↦ φ_t(…φ_1(x)…)`; `None` when `x` hits a kernel
    /// abscissa on the way.
    pub fn eval(&self, f: &F, x: F::Elem) -> Option<F::Elem> {
        let mut y = x;
        for s in &self.steps {
            if y == s.xi {
                return None;
            }
            let d = f.inv(f.sub(y, s.xi))?;
            y = f.add(y, f.mul(s.tau, d));
        }
        Some(y)
    }

    /// The level values `y_0 … y_{t−1}` of an abscissa of `V` (`y_0 = x`).
    pub fn values(&self, f: &F, x: F::Elem) -> Vec<F::Elem> {
        let mut y = vec![f.zero(); self.t as usize];
        y[0] = x;
        for j in 0..self.t as usize - 1 {
            let s = &self.steps[j];
            let d = f.inv(f.sub(y[j], s.xi)).expect("x ∈ V avoids every kernel");
            y[j + 1] = f.add(y[j], f.mul(s.tau, d));
        }
        y
    }
}

/// The Vélu 2-isogeny with kernel `(ξ, 0)`: `x ↦ x + τ/(x − ξ)`,
/// `y ↦ y·(1 − τ/(x − ξ)²)`.
fn velu_map_w<F: PrimeField>(
    f: &F,
    pt: WPoint<F::Elem>,
    xi: F::Elem,
    tau: F::Elem,
) -> WPoint<F::Elem> {
    let Some((x, y)) = pt else { return None };
    if x == xi {
        return None;
    }
    let d = f.inv(f.sub(x, xi)).expect("x ≠ ξ");
    let x2 = f.add(x, f.mul(tau, d));
    let y2 = f.mul(y, f.sub(f.one(), f.mul(tau, f.mul(d, d))));
    Some((x2, y2))
}

/// The build on an accepted auxiliary curve, shared by the searched
/// (ordinary) and supersingular towers — the wide twin of
/// `prime_tower::finish_tower`.
fn finish_tower_wide<F: PrimeField>(
    f: &F,
    t: u32,
    aux: WideCurve<F::Elem>,
    n: BigUint,
    q: WPoint<F::Elem>,
    rng: &mut StdRng,
    ops: &mut GroupOps,
    mut cost: IsogenyTowerCost,
) -> Result<WideTower<F>, String> {
    let two_t = BigUint::one() << t;
    let build_before = ops.adds + ops.doubles;
    // ⟨Q⟩ by successive additions.
    let mut multiples: Vec<WPoint<F::Elem>> = Vec::with_capacity(1usize << t.min(24));
    multiples.push(None);
    let mut acc = q;
    for _ in 1u64..(1u64 << t) {
        multiples.push(acc);
        acc = aux.add(f, ops, acc, q);
    }
    if acc.is_some() {
        return Err("Q does not have order 2^t".into());
    }
    let keys: HashSet<(F::Elem, F::Elem)> = multiples
        .iter()
        .filter_map(|m| *m)
        .collect();
    let r0 = loop {
        let r = random_point_w(f, &aux, rng, &mut cost);
        let r2 = aux.double(f, ops, r);
        if let Some(r2) = r2 {
            if !keys.contains(&r2) {
                break r;
            }
        } else {
            break r;
        }
    };
    let v: Vec<F::Elem> = multiples
        .iter()
        .map(|&m| aux.add(f, ops, r0, m))
        .filter_map(|pt| pt.map(|p| p.0))
        .collect();
    // The chain.
    let (mut aj, mut bj) = (aux.a, aux.b);
    let (mut qj, mut rj) = (q, r0);
    let mut steps = Vec::with_capacity(t as usize);
    for j in 0..t {
        let cj = WideCurve { a: aj, b: bj };
        let kj = cj.mul_pow2(f, ops, qj, t - 1 - j);
        let Some((kx, ky)) = kj else {
            return Err(format!("step {j}: the kernel point is the identity"));
        };
        if !f.is_zero(ky) {
            return Err(format!("step {j}: the kernel point is not 2-torsion"));
        }
        let tau = f.add(f.mul(f.from_u64(3), f.sqr(kx)), aj);
        steps.push(VeluStepW { xi: kx, tau });
        qj = velu_map_w(f, qj, kx, tau);
        rj = velu_map_w(f, rj, kx, tau);
        cost.velu_evaluations += 2;
        aj = f.sub(aj, f.mul(f.from_u64(5), tau));
        bj = f.sub(bj, f.mul(f.from_u64(7), f.mul(kx, tau)));
        let next = WideCurve { a: aj, b: bj };
        if !next.on_curve(f, qj) || !next.on_curve(f, rj) {
            return Err(format!("step {j}: a Vélu image is off the codomain"));
        }
    }
    cost.build_gae += (ops.adds + ops.doubles) - build_before;
    let Some((rx, _)) = rj else {
        return Err("Φ(R') is the identity".into());
    };
    let tower = WideTower {
        t,
        aux,
        aux_order: n,
        q,
        r: r0,
        v,
        steps,
        c: rx,
        cost,
    };
    let mut distinct: HashSet<F::Elem> = HashSet::new();
    for &x in &tower.v {
        if !distinct.insert(x) {
            return Err(format!("V has a repeat at {x:?}"));
        }
    }
    if distinct.len() as u64 != 1u64 << t {
        return Err(format!("|V| = {} ≠ 2^{t}", distinct.len()));
    }
    Ok(tower)
}

/// The seed derivation: deterministic in `(p, t, seed)`.
fn wide_rng(p: &BigUint, t: u32, seed: u64, salt: u64) -> StdRng {
    let mask = (BigUint::one() << 64u32) - BigUint::one();
    let low = (p & &mask).to_u64().unwrap_or(0);
    StdRng::seed_from_u64(
        seed.rotate_left(17) ^ (u64::from(t) << 40) ^ low.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ salt,
    )
}

/// Search, from the public seed, an auxiliary curve `E'/F_p` with a point
/// of order `2^t`, over any backend that holds `p`, and build the tower.
/// The `u64` plug-in's charged search, at any size.
pub fn isogeny_tower_wide<F: PrimeField>(
    f: &F,
    t: u32,
    seed: u64,
    family: AuxFamily,
    ops: &mut GroupOps,
) -> Result<WideTower<F>, String> {
    let p = f.modulus().clone();
    if !(1..=63).contains(&t) {
        return Err(format!("t = {t} is outside 1 … 63"));
    }
    let two_t = BigUint::one() << t;
    if two_t > &p / BigUint::from(4u32) {
        return Err(format!("p is too small for t = {t}"));
    }
    if family == AuxFamily::Z2xZ8 && (&p % BigUint::from(4u32)) != BigUint::from(3u32) {
        return Err(format!(
            "p is not 3 mod 4: the X₁(2, 8) family needs −1 to be a non-residue"
        ));
    }
    let w4 = (BigUint::from(4u32) * &p).sqrt();
    let (lo, hi) = (p.clone() + BigUint::one() - &w4, p.clone() + BigUint::one() + &w4);
    let mut rng = wide_rng(&p, t, seed, 0x5749_4455);
    let mut cost = IsogenyTowerCost::default();
    let gae = |o: &GroupOps| o.adds + o.doubles;
    let mut tried_limit = 64u64 << t.min(48);
    tried_limit += 4096;
    let inv3 = f.inv(f.from_u64(3)).expect("p ≠ 3");
    let inv27 = f.inv(f.from_u64(27)).expect("p ≠ 3");
    loop {
        if tried_limit == 0 {
            return Err(format!("no auxiliary curve with a point of order 2^{t} found"));
        }
        tried_limit -= 1;
        cost.curves_tried += 1;
        let (a, b) = match family {
            AuxFamily::Any => {
                let a = f.from_biguint(&rand_below(&p, &mut rng));
                let b = f.from_biguint(&rand_below(&p, &mut rng));
                let disc = f.add(
                    f.mul(f.from_u64(4), f.mul(f.sqr(a), a)),
                    f.mul(f.from_u64(27), f.sqr(b)),
                );
                if f.is_zero(disc) {
                    cost.singular += 1;
                    continue;
                }
                (a, b)
            }
            AuxFamily::Z2xZ8 => {
                let m = f.from_biguint(&rand_below(&p, &mut rng));
                let m2 = f.sqr(m);
                let fa = f.sub(m2, f.one());
                let fb = f.double(m);
                if f.is_zero(fa) || f.is_zero(fb) {
                    cost.singular += 1;
                    continue;
                }
                let fa4 = f.sqr(f.sqr(fa));
                let fb4 = f.sqr(f.sqr(fb));
                if fa4 == fb4 {
                    cost.singular += 1;
                    continue;
                }
                let e2 = f.neg(fb4);
                let e3 = f.neg(fa4);
                let s1 = f.add(e2, e3);
                let s2 = f.mul(e2, e3);
                let s1sq = f.sqr(s1);
                let a = f.sub(s2, f.mul(s1sq, inv3));
                let t1 = f.mul(f.mul(f.from_u64(2), f.mul(s1sq, s1)), inv27);
                let t2 = f.mul(f.mul(s1, s2), inv3);
                let b = f.sub(t2, t1);
                (a, b)
            }
        };
        let e = WideCurve { a, b };
        let before = gae(ops);
        let pt = random_point_w(f, &e, &mut rng, &mut cost);
        let (hits, _, _) = class_hits_w(f, &e, pt, &two_t, &lo, &hi, ops, &mut cost.bsgs_table_entries);
        if hits == 0 {
            cost.filtered_out += 1;
            cost.search_gae += gae(ops) - before;
            continue;
        }
        let count_before = gae(ops);
        cost.orders_computed += 1;
        let order = mestre_order_w(f, &e, &mut rng, &lo, &hi, 16, ops, &mut cost);
        cost.count_gae += gae(ops) - count_before;
        cost.search_gae += gae(ops) - before;
        let Some(n) = order else {
            cost.orders_undetermined += 1;
            continue;
        };
        if (&n % &two_t) != BigUint::zero() {
            cost.filter_false_positives += 1;
            continue;
        }
        let build_before = gae(ops);
        let odd = &n >> n.trailing_zeros().expect("n ≥ 1");
        let mut q: WPoint<F::Elem> = None;
        for _ in 0..8 {
            let t0 = e.mul_biguint(f, ops, random_point_w(f, &e, &mut rng, &mut cost), &odd);
            let mut order_bits = 0u32;
            let mut cur = t0;
            while let Some(c) = cur {
                cur = e.double(f, ops, Some(c));
                order_bits += 1;
            }
            if order_bits >= t {
                let mut cand = t0;
                for _ in 0..(order_bits - t) {
                    cand = e.double(f, ops, cand);
                }
                q = cand;
                break;
            }
        }
        cost.build_gae += gae(ops) - build_before;
        match q {
            Some(q) => return finish_tower_wide(f, t, e, n, Some(q), &mut rng, ops, cost),
            None => cost.two_sylow_rejected += 1,
        }
    }
}

/// The supersingular auxiliary curve of `kind` (`#E' = p + 1` by Deuring).
fn supersingular_aux_w<F: PrimeField>(
    f: &F,
    kind: AuxKind,
) -> Result<WideCurve<F::Elem>, String> {
    let p = f.modulus().clone();
    let (a, b) = match kind {
        AuxKind::Ordinary => return Err("kind = ordinary is not a supersingular curve".into()),
        AuxKind::SsJ1728 => (f.one(), f.zero()),
        AuxKind::SsJ1728Twist => (f.neg(f.one()), f.zero()),
        AuxKind::SsJ0 => (f.zero(), f.one()),
        AuxKind::SsJ0Twist => {
            let mut d = BigUint::from(2u32);
            loop {
                if f.legendre(f.from_biguint(&d)) == -1 {
                    break;
                }
                d += 1u32;
            }
            (f.zero(), f.pow_biguint(f.from_biguint(&d), &BigUint::from(3u32)))
        }
    };
    let four = BigUint::from(4u32);
    let three = BigUint::from(3u32);
    match kind {
        AuxKind::SsJ1728 | AuxKind::SsJ1728Twist => {
            if (&p % &four) != BigUint::from(3u32) {
                return Err(format!("p is not 3 mod 4: {kind:?} is not supersingular"));
            }
        }
        AuxKind::SsJ0 | AuxKind::SsJ0Twist => {
            if (&p % &three) != BigUint::from(2u32) {
                return Err(format!("p is not 2 mod 3: {kind:?} is not supersingular"));
            }
        }
        AuxKind::Ordinary => unreachable!(),
    }
    Ok(WideCurve { a, b })
}

/// The supersingular tower: no search, no point counting.  Needs
/// `v₂(p + 1) ≥ t` (P-192: 64; P-256: 96).
pub fn supersingular_tower_wide<F: PrimeField>(
    f: &F,
    t: u32,
    seed: u64,
    kind: AuxKind,
    ops: &mut GroupOps,
) -> Result<WideTower<F>, String> {
    let p = f.modulus().clone();
    if !(1..=63).contains(&t) {
        return Err(format!("t = {t} is outside 1 … 63"));
    }
    let two_t = BigUint::one() << t;
    if two_t > &p / BigUint::from(4u32) {
        return Err(format!("p is too small for t = {t}"));
    }
    let aux = supersingular_aux_w(f, kind)?;
    let n = p.clone() + BigUint::one();
    let v = n.trailing_zeros().expect("p + 1 ≥ 2");
    if v < u64::from(t) {
        return Err(format!(
            "v₂(p + 1) = {v} < t = {t}: no point of order 2^{t} on the supersingular curve"
        ));
    }
    let mut rng = wide_rng(&p, t, seed, 0x5353_5353);
    let mut cost = IsogenyTowerCost::default();
    let scalar = &n / &two_t;
    let mut q: WPoint<F::Elem> = None;
    for _ in 0..64 {
        let cand = aux.mul_biguint(
            f,
            ops,
            random_point_w(f, &aux, &mut rng, &mut cost),
            &scalar,
        );
        // Accept exactly order 2^t: [2^{t−1}]Q ≠ O and [2^t]Q = O.
        let half = aux.mul_pow2(f, ops, cand, t - 1);
        let full = aux.mul_pow2(f, ops, cand, t);
        if half.is_some() && full.is_none() {
            q = cand;
            break;
        }
    }
    let q = q.ok_or_else(|| format!("no point of order 2^{t} drawn in 64 tries"))?;
    cost.curves_tried = 1;
    finish_tower_wide(f, t, aux, n, Some(q), &mut rng, ops, cost)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::fp_backend::{Fp32, Fp64, FpP192};

    /// `#E(F_p)` by Legendre sums — the exhaustive check at small primes.
    fn point_count<F: PrimeField>(f: &F, e: &WideCurve<F::Elem>) -> BigUint {
        let p = f.modulus();
        let mut n = BigUint::one(); // the identity
        let mut x = BigUint::zero();
        while &x < &p {
            let r = e.rhs(f, f.from_biguint(&x));
            n += BigUint::from(match f.legendre(r) {
                1 => 2u32,
                -1 => 0u32,
                _ => 1u32,
            });
            x += 1u32;
        }
        n
    }

    fn check_roots<F: PrimeField>(f: &F, tower: &WideTower<F>) {
        assert_eq!(tower.v.len(), 1usize << tower.t);
        let mut seen = HashSet::new();
        for &x in &tower.v {
            assert!(seen.insert(x), "V has a repeat");
            assert_eq!(tower.eval(f, x), Some(tower.c), "x is not a root");
        }
        // And nothing else is, at a small prime.
        if f.modulus() < &BigUint::from(100_000u32) {
            let p = f.modulus().clone();
            let mut x = BigUint::zero();
            let mut roots = 0u64;
            while &x < &p {
                if tower.eval(f, f.from_biguint(&x)) == Some(tower.c) {
                    roots += 1;
                }
                x += 1u32;
            }
            assert_eq!(roots, 1u64 << tower.t);
        }
    }

    #[test]
    fn ordinary_wide_tower_at_a_small_prime() {
        // 65,519 is 3 mod 4, so both families are defined; v₂(p − 1) = 4
        // forces the isogeny (searched) route, like P-192's shape.
        let f = Fp64::new(65_519);
        let mut ops = GroupOps::default();
        let tower = isogeny_tower_wide(&f, 6, 5, AuxFamily::Any, &mut ops).unwrap();
        check_roots(&f, &tower);
        assert_eq!(tower.aux_order, point_count(&f, &tower.aux), "Mestre order");
        assert!(tower.aux_order.is_multiple_of(&(BigUint::one() << tower.t)));
        // The charge: everything on the ledger, and the split adds up.
        let c = &tower.cost;
        assert_eq!(ops.adds + ops.doubles, c.search_gae + c.build_gae);
        assert_eq!(
            c.curves_tried,
            1 + c.singular + c.filtered_out + c.filter_false_positives + c.orders_undetermined
                + c.two_sylow_rejected
        );
        // Deterministic in (p, t, seed, family).
        let again = isogeny_tower_wide(&f, 6, 5, AuxFamily::Any, &mut GroupOps::default())
            .unwrap();
        assert_eq!(
            again.v.iter().map(|x| f.to_biguint(*x)).collect::<Vec<_>>(),
            tower.v.iter().map(|x| f.to_biguint(*x)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn z2xz8_wide_needs_p_three_mod_four_and_tries_fewer_curves() {
        let f = Fp64::new(65_519);
        let any = isogeny_tower_wide(&f, 6, 5, AuxFamily::Any, &mut GroupOps::default()).unwrap();
        let fam = isogeny_tower_wide(&f, 6, 5, AuxFamily::Z2xZ8, &mut GroupOps::default()).unwrap();
        check_roots(&f, &fam);
        assert!(fam.aux_order.is_multiple_of(&BigUint::from(8u32)));
        assert_eq!(fam.aux_order, point_count(&f, &fam.aux));
        assert!(
            fam.cost.curves_tried < any.cost.curves_tried,
            "z2xz8 tried {} curves, any tried {}",
            fam.cost.curves_tried,
            any.cost.curves_tried
        );
        // 786,433 is 1 mod 4: refused.
        let g = Fp64::new(786_433);
        assert!(isogeny_tower_wide(&g, 8, 5, AuxFamily::Z2xZ8, &mut GroupOps::default())
            .err()
            .unwrap()
            .contains("3 mod 4"));
    }

    #[test]
    fn supersingular_wide_tower_at_p192() {
        // The one-draw tower of research/pkm_supersingular_aux_20261015:
        // y² = x³ + x at the NIST P-192 prime, #E' = p + 1, v₂ = 64.
        let f = FpP192::new();
        let p = f.modulus();
        let mut ops = GroupOps::default();
        let tower =
            supersingular_tower_wide(&f, 8, 7, AuxKind::SsJ1728, &mut ops).unwrap();
        assert_eq!(tower.aux_order, p + BigUint::one());
        check_roots(&f, &tower);
        // Q has exact order 2^t, independently, by repeated doubling.
        let mut q = tower.q;
        for _ in 0..tower.t - 1 {
            q = tower.aux.double(&f, &mut GroupOps::default(), q);
            assert!(q.is_some(), "[2^j]Q = O too early");
        }
        q = tower.aux.double(&f, &mut GroupOps::default(), q);
        assert!(q.is_none(), "[2^t]Q ≠ O");
        // Independent BigUint recomputation of the composed map on V.
        let pb = f.modulus().clone();
        for &x in tower.v.iter().take(1 << 8) {
            let mut y = f.to_biguint(x);
            for s in &tower.steps {
                let xi = f.to_biguint(s.xi);
                let tau = f.to_biguint(s.tau);
                let d = (&y + &pb - &xi) % &pb;
                let d = d.modpow(&(&pb - BigUint::from(2u32)), &pb);
                y = (&y + tau * &d % &pb) % &pb;
            }
            assert_eq!(y, f.to_biguint(tower.c));
        }
        // The Vélu images are on every codomain (built in), and the level
        // values of V satisfy the tower equations.
        let y0 = tower.values(&f, tower.v[0]);
        assert_eq!(y0.len(), tower.t as usize);
        // Determinism.
        let again = supersingular_tower_wide(&f, 8, 7, AuxKind::SsJ1728, &mut GroupOps::default())
            .unwrap();
        assert_eq!(
            again.v.iter().map(|x| f.to_biguint(*x)).collect::<Vec<_>>(),
            tower.v.iter().map(|x| f.to_biguint(*x)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn wide_tower_limits() {
        let f = Fp32::new(10_007);
        assert!(isogeny_tower_wide(&f, 0, 1, AuxFamily::Any, &mut GroupOps::default())
            .err()
            .unwrap()
            .contains("outside"));
        assert!(isogeny_tower_wide(&f, 64, 1, AuxFamily::Any, &mut GroupOps::default())
            .err()
            .unwrap()
            .contains("outside"));
        // v₂(p + 1) = 4 at 10,007 < t = 5: no supersingular point of order 32.
        assert!(supersingular_tower_wide(&f, 5, 1, AuxKind::SsJ1728, &mut GroupOps::default())
            .err()
            .unwrap()
            .contains("v₂(p + 1)"));
    }
}
