//! The two 131-bit Certicom curves (and same-family calibration curves)
//! behind one trait, so every oracle, census and collector in this module
//! runs unchanged on ECC2-131 and ECCp-131.
//!
//! The hot operation is [`SymCurve::plus_minus_keys`]: for a point `T` and
//! a slice of points `P_i`, the abscissae of both `T + P_i` and `T − P_i`.
//! The two sums share the denominator `x_T − x_{P_i}` (`x_T + x_{P_i}` in
//! characteristic 2), so one batched inversion serves both — the
//! negation-map symmetry of the summation polynomial `S_3(x_1, x_2, x_T)`,
//! which cannot tell `T + P` from `T − P`.  [`SymCurve::plus_minus_keys_unshared`]
//! evaluates the same keys with one inversion per sum and no batching: the
//! baseline the shared form is measured against.

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::Rng;

use super::gf2w::{self, Gf2W, W3};
use crate::binary_ecc::IrreduciblePoly;
use crate::cryptanalysis::prime_small_x::{Curve, MElem, MontField, PrimeField};
use crate::cryptanalysis::secg_standards::{SecgCurve, SecgField};

/// A canonical field-element key (little-endian words of the integer or
/// polynomial representative).
pub type Key = [u64; 3];

/// An affine point; the point at infinity is `None` wherever it can occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pt<E> {
    pub x: E,
    pub y: E,
}

pub trait SymCurve: Sync + Send {
    type E: Copy + Eq + Send + Sync + std::fmt::Debug;

    fn name(&self) -> &str;
    /// Bits of the field (`n` for `GF(2^n)`, `⌈log₂ p⌉` for `F_p`).
    fn field_bits(&self) -> u32;
    /// Order `r` of the prime-order subgroup the base and targets live in,
    /// when known (calibration curves are not point-counted).
    fn order(&self) -> Option<&BigUint>;
    fn key(&self, x: &Self::E) -> Key;
    fn add(&self, p: &Pt<Self::E>, q: &Pt<Self::E>) -> Option<Pt<Self::E>>;
    fn neg(&self, p: &Pt<Self::E>) -> Pt<Self::E>;
    fn on_curve(&self, p: &Pt<Self::E>) -> bool;
    /// Whether an affine point lies in the prime-order subgroup.
    fn in_subgroup(&self, p: &Pt<Self::E>) -> bool;
    /// A point with abscissa `x` in the subgroup, if one exists.
    fn lift_x(&self, x: &Self::E) -> Option<Pt<Self::E>>;
    /// The element with key `k` (used to enumerate abscissa ranges).
    fn element_of_key(&self, k: &Key) -> Option<Self::E>;
    fn random_point<R: Rng>(&self, rng: &mut R) -> Pt<Self::E>;

    /// `out[i] = [key x(T + P_i), key x(T − P_i)]`; a sum that is the point
    /// at infinity, or a `P_i` with `x(P_i) = x(T)`, gets [`NO_KEY`].
    fn plus_minus_keys(&self, t: &Pt<Self::E>, ps: &[Pt<Self::E>], out: &mut Vec<[Key; 2]>);

    /// The same keys, one inversion per sum, no shared denominator.
    fn plus_minus_keys_unshared(
        &self,
        t: &Pt<Self::E>,
        ps: &[Pt<Self::E>],
        out: &mut Vec<[Key; 2]>,
    ) {
        out.clear();
        for p in ps {
            let plus = self.add(t, p).map(|s| self.key(&s.x)).unwrap_or(NO_KEY);
            let minus = self
                .add(t, &self.neg(p))
                .map(|s| self.key(&s.x))
                .unwrap_or(NO_KEY);
            out.push([plus, minus]);
        }
    }

    /// `acc[i] ← acc[i] + jumps[i]` for every lane, one batched inversion
    /// for the whole slice: the step of an adding walk (Pollard rho's
    /// reference cost on the same field code).  Lanes whose sum is
    /// undefined here (equal abscissae) are left unchanged and counted.
    fn add_batch(&self, acc: &mut [Pt<Self::E>], jumps: &[Pt<Self::E>]) -> usize {
        let mut skipped = 0;
        for (a, j) in acc.iter_mut().zip(jumps) {
            match self.add(a, j) {
                Some(s) if a.x != j.x => *a = s,
                _ => skipped += 1,
            }
        }
        skipped
    }

    /// `[k]P` (`None` for the point at infinity).
    fn mul(&self, p: &Pt<Self::E>, k: &BigUint) -> Option<Pt<Self::E>> {
        let mut acc: Option<Pt<Self::E>> = None;
        for i in (0..k.bits()).rev() {
            acc = acc.and_then(|a| self.add(&a, &a));
            if k.bit(i) {
                acc = match acc {
                    Some(a) => self.add(&a, p),
                    None => Some(*p),
                };
            }
        }
        acc
    }
}

/// A key no canonical element has (all words saturated).
pub const NO_KEY: Key = [u64::MAX; 3];

// ── Binary curves y² + xy = x³ + a·x² + b ─────────────────────────────

#[derive(Clone, Debug)]
pub struct BinaryW {
    pub name: String,
    pub f: Gf2W,
    pub a: W3,
    pub b: W3,
    /// Prime subgroup order and cofactor, when known.
    pub order: Option<BigUint>,
    pub cofactor: Option<u64>,
    trace_a: u32,
}

impl BinaryW {
    pub fn new(
        name: &str,
        irr: &IrreduciblePoly,
        a: W3,
        b: W3,
        order: Option<BigUint>,
        cofactor: Option<u64>,
    ) -> Self {
        let f = Gf2W::new(irr);
        let trace_a = f.trace(&a);
        assert!(!gf2w::is_zero(&b), "b = 0 is singular");
        Self {
            name: name.into(),
            f,
            a,
            b,
            order,
            cofactor,
            trace_a,
        }
    }

    /// Certicom ECC2-131 with its published generator and target.
    pub fn ecc2_131() -> (Self, Pt<W3>, Pt<W3>) {
        let hex = |s: &str| gf2w::from_biguint(&BigUint::parse_bytes(s.as_bytes(), 16).unwrap());
        let irr = gf2w::low_weight_irreducible(131);
        let c = Self::new(
            "ECC2-131",
            &irr,
            hex("07EBCB7EECC296A1C4A1A14F2C9E44352E"),
            hex("00610B0A57C73649AD0093BDD622A61D81"),
            Some(BigUint::parse_bytes(b"0400000000000000026ABB991FE311FE83", 16).unwrap()),
            Some(2),
        );
        let g = Pt {
            x: hex("00439CBC8DC73AA981030D5BC57B331663"),
            y: hex("014904C07D4F25A16C2DE036D60B762BD4"),
        };
        let q = Pt {
            x: hex("0602339C5DB0E9C694AC8908528C51C440"),
            y: hex("04F7B99169FA1A0F2737813742B1588CB8"),
        };
        (c, g, q)
    }

    /// A random curve over `GF(2^n)` (calibration; not point-counted).
    pub fn random<R: Rng>(n: u32, rng: &mut R) -> Self {
        let irr = gf2w::low_weight_irreducible(n);
        let f = Gf2W::new(&irr);
        let a = f.random(rng);
        let mut b = f.random(rng);
        while gf2w::is_zero(&b) {
            b = f.random(rng);
        }
        Self::new(&format!("random-GF(2^{n})"), &irr, a, b, None, None)
    }
}

impl SymCurve for BinaryW {
    type E = W3;

    fn name(&self) -> &str {
        &self.name
    }
    fn field_bits(&self) -> u32 {
        self.f.n
    }
    fn order(&self) -> Option<&BigUint> {
        self.order.as_ref()
    }
    fn key(&self, x: &W3) -> Key {
        *x
    }
    fn add(&self, p: &Pt<W3>, q: &Pt<W3>) -> Option<Pt<W3>> {
        let f = &self.f;
        if p.x == q.x {
            // Q = P (doubling) or Q = −P (infinity); −P = (x, x + y).
            if p.y != q.y || gf2w::is_zero(&p.x) {
                return None;
            }
            let lam = gf2w::xor(p.x, f.mul(&p.y, &f.inv(&p.x)));
            let x3 = gf2w::xor(gf2w::xor(f.sqr(&lam), lam), self.a);
            let y3 = gf2w::xor(f.sqr(&p.x), f.mul(&gf2w::xor(lam, gf2w::ONE), &x3));
            return Some(Pt { x: x3, y: y3 });
        }
        let dx = gf2w::xor(p.x, q.x);
        let lam = f.mul(&gf2w::xor(p.y, q.y), &f.inv(&dx));
        let x3 = gf2w::xor(gf2w::xor(gf2w::xor(f.sqr(&lam), lam), dx), self.a);
        let y3 = gf2w::xor(gf2w::xor(f.mul(&lam, &gf2w::xor(p.x, x3)), x3), p.y);
        Some(Pt { x: x3, y: y3 })
    }
    fn neg(&self, p: &Pt<W3>) -> Pt<W3> {
        Pt {
            x: p.x,
            y: gf2w::xor(p.x, p.y),
        }
    }
    fn on_curve(&self, p: &Pt<W3>) -> bool {
        let f = &self.f;
        if !f.is_canonical(&p.x) || !f.is_canonical(&p.y) {
            return false;
        }
        let lhs = gf2w::xor(f.sqr(&p.y), f.mul(&p.x, &p.y));
        let x2 = f.sqr(&p.x);
        let rhs = gf2w::xor(gf2w::xor(f.mul(&x2, &p.x), f.mul(&self.a, &x2)), self.b);
        lhs == rhs
    }
    /// With cofactor 2 the prime-order subgroup is `2E`, and a point
    /// `P ≠ O` is in `2E` exactly when `Tr(x_P) = Tr(a)` (the halving
    /// criterion).  With another cofactor the check multiplies by `r`.
    fn in_subgroup(&self, p: &Pt<W3>) -> bool {
        match (self.cofactor, &self.order) {
            (Some(2), _) | (None, None) => self.f.trace(&p.x) == self.trace_a,
            (_, Some(r)) => self.mul(p, r).is_none(),
            _ => true,
        }
    }
    /// `y = x·z` with `z² + z = x + a + b/x²`.
    fn lift_x(&self, x: &W3) -> Option<Pt<W3>> {
        let f = &self.f;
        if gf2w::is_zero(x) || !f.is_canonical(x) {
            return None;
        }
        let x2 = f.sqr(x);
        let c = gf2w::xor(gf2w::xor(*x, self.a), f.mul(&self.b, &f.inv(&x2)));
        let z = f.solve_quadratic(&c)?;
        let p = Pt {
            x: *x,
            y: f.mul(x, &z),
        };
        debug_assert!(self.on_curve(&p));
        self.in_subgroup(&p).then_some(p)
    }
    fn element_of_key(&self, k: &Key) -> Option<W3> {
        self.f.is_canonical(k).then_some(*k)
    }
    fn random_point<R: Rng>(&self, rng: &mut R) -> Pt<W3> {
        loop {
            let x = self.f.random(rng);
            if let Some(p) = self.lift_x(&x) {
                return if rng.gen() { self.neg(&p) } else { p };
            }
        }
    }

    fn add_batch(&self, acc: &mut [Pt<W3>], jumps: &[Pt<W3>]) -> usize {
        let f = &self.f;
        let mut den: Vec<W3> = acc
            .iter()
            .zip(jumps)
            .map(|(a, j)| gf2w::xor(a.x, j.x))
            .collect();
        let mut scratch = Vec::with_capacity(den.len());
        f.batch_inv(&mut den, &mut scratch);
        let mut skipped = 0;
        for ((a, j), inv) in acc.iter_mut().zip(jumps).zip(den.iter()) {
            if gf2w::is_zero(inv) {
                skipped += 1;
                continue;
            }
            let dx = gf2w::xor(a.x, j.x);
            let lam = f.mul(&gf2w::xor(a.y, j.y), inv);
            let x3 = gf2w::xor(gf2w::xor(gf2w::xor(f.sqr(&lam), lam), dx), self.a);
            let y3 = gf2w::xor(gf2w::xor(f.mul(&lam, &gf2w::xor(a.x, x3)), x3), a.y);
            *a = Pt { x: x3, y: y3 };
        }
        skipped
    }

    fn plus_minus_keys(&self, t: &Pt<W3>, ps: &[Pt<W3>], out: &mut Vec<[Key; 2]>) {
        let f = &self.f;
        out.clear();
        let mut den: Vec<W3> = ps.iter().map(|p| gf2w::xor(t.x, p.x)).collect();
        let mut scratch = Vec::with_capacity(den.len());
        f.batch_inv(&mut den, &mut scratch);
        for (p, inv) in ps.iter().zip(den.iter()) {
            if gf2w::is_zero(inv) {
                out.push([NO_KEY, NO_KEY]);
                continue;
            }
            // T + P: λ = (y_T + y_P)/(x_T + x_P).  T − P uses −P = (x_P,
            // x_P + y_P): λ' = λ + x_P/(x_T + x_P).
            let s = gf2w::xor(t.x, gf2w::xor(p.x, self.a));
            let lam = f.mul(&gf2w::xor(t.y, p.y), inv);
            let lam2 = gf2w::xor(lam, f.mul(&p.x, inv));
            let xp = gf2w::xor(gf2w::xor(f.sqr(&lam), lam), s);
            let xm = gf2w::xor(gf2w::xor(f.sqr(&lam2), lam2), s);
            out.push([xp, xm]);
        }
    }
}

// ── Prime curves y² = x³ + a·x + b ────────────────────────────────────

pub type Fp3 = MontField<3>;
pub type E3 = MElem<3>;

#[derive(Clone)]
pub struct PrimeW {
    pub c: Curve<Fp3>,
    bits: u32,
    /// Whether `c.order` is the real subgroup order (false for
    /// calibration curves, which are not point-counted).
    known_order: bool,
}

impl PrimeW {
    fn wrap(c: Curve<Fp3>, known_order: bool) -> Self {
        let bits = c.field.modulus().bits() as u32;
        Self {
            c,
            bits,
            known_order,
        }
    }

    /// Certicom ECCp-131 with its published generator and target.
    pub fn eccp_131() -> (Self, Pt<E3>, Pt<E3>) {
        let p = crate::ecc::curve::CurveParams::eccp131();
        let record = SecgCurve {
            name: p.name.into(),
            oid: String::new(),
            field: SecgField::Prime { p: p.p.clone() },
            a: p.a.clone(),
            b: p.b.clone(),
            gx: p.gx.clone(),
            gy: p.gy.clone(),
            order: p.n.clone(),
            cofactor: p.h as u64,
        };
        let c = Curve::<Fp3>::from_secg(&record).expect("ECCp-131 parameters");
        let f = c.field.clone();
        let g = Pt {
            x: f.embed_big(&p.gx).unwrap(),
            y: f.embed_big(&p.gy).unwrap(),
        };
        // Published target Q (Certicom curves list; same values as
        // ecc-challenges multicurve/instances.json).
        let qx = BigUint::parse_bytes(b"03AA6F004FC62E2DA1ED0BFB62C3FFB568", 16).unwrap();
        let qy = BigUint::parse_bytes(b"009C21C284BA8A445BB2701BF55E3A67ED", 16).unwrap();
        let q = Pt {
            x: f.embed_big(&qx).unwrap(),
            y: f.embed_big(&qy).unwrap(),
        };
        (Self::wrap(c, true), g, q)
    }

    /// A random curve over a random `bits`-bit prime `p ≡ 3 (mod 4)`
    /// (calibration; not point-counted).
    pub fn random<R: Rng>(bits: u32, rng: &mut R) -> Self {
        use num_bigint::RandBigInt;
        let p = loop {
            let mut cand = rng.gen_biguint(bits as u64);
            cand.set_bit(bits as u64 - 1, true);
            cand |= BigUint::from(3u8);
            if is_probable_prime(&cand, rng) {
                break cand;
            }
        };
        let field = Fp3::new(&p).expect("fits three limbs");
        let a = rng.gen_biguint_below(&p);
        let b = rng.gen_biguint_below(&p);
        let c = Curve {
            name: format!("random-F_p ({bits}-bit)"),
            a: field.embed_big(&a).unwrap(),
            b: field.embed_big(&b).unwrap(),
            order: BigUint::zero(),
            order_is_prime: false,
            cofactor: 1,
            field,
        };
        Self::wrap(c, false)
    }

    pub fn embed(&self, v: &BigUint) -> E3 {
        self.c.field.embed_big(v).expect("canonical")
    }
}

/// Miller–Rabin with 32 random bases.
pub fn is_probable_prime<R: Rng>(n: &BigUint, rng: &mut R) -> bool {
    use num_bigint::RandBigInt;
    let two = BigUint::from(2u8);
    if n < &BigUint::from(4u8) {
        return n >= &two;
    }
    if !n.bit(0) {
        return false;
    }
    let n1 = n - BigUint::one();
    let s = n1.trailing_zeros().unwrap_or(0);
    let d = &n1 >> s;
    'outer: for _ in 0..32 {
        let a = rng.gen_biguint_range(&two, &n1);
        let mut x = a.modpow(&d, n);
        if x.is_one() || x == n1 {
            continue;
        }
        for _ in 1..s {
            x = x.modpow(&two, n);
            if x == n1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

impl SymCurve for PrimeW {
    type E = E3;

    fn name(&self) -> &str {
        &self.c.name
    }
    fn field_bits(&self) -> u32 {
        self.bits
    }
    fn order(&self) -> Option<&BigUint> {
        self.known_order.then_some(&self.c.order)
    }
    fn key(&self, x: &E3) -> Key {
        self.c.field.key(*x)
    }
    fn add(&self, p: &Pt<E3>, q: &Pt<E3>) -> Option<Pt<E3>> {
        let mut f = self.c.field.clone();
        let a = crate::cryptanalysis::prime_small_x::Pt { x: p.x, y: p.y };
        let b = crate::cryptanalysis::prime_small_x::Pt { x: q.x, y: q.y };
        self.c
            .add_points(&mut f, &a, &b)
            .map(|s| Pt { x: s.x, y: s.y })
    }
    fn neg(&self, p: &Pt<E3>) -> Pt<E3> {
        let mut f = self.c.field.clone();
        Pt {
            x: p.x,
            y: f.neg(p.y),
        }
    }
    fn on_curve(&self, p: &Pt<E3>) -> bool {
        let mut f = self.c.field.clone();
        self.c.on_curve(
            &mut f,
            &crate::cryptanalysis::prime_small_x::Pt { x: p.x, y: p.y },
        )
    }
    fn in_subgroup(&self, _p: &Pt<E3>) -> bool {
        // ECCp-131 has cofactor 1; calibration curves are not counted and
        // treat the whole group as the target group.
        true
    }
    fn lift_x(&self, x: &E3) -> Option<Pt<E3>> {
        let mut f = self.c.field.clone();
        let r = self.c.rhs(&mut f, *x);
        let y = self.c.sqrt(&mut f, r)?;
        if f.is_zero(y) {
            return None;
        }
        Some(Pt { x: *x, y })
    }
    fn element_of_key(&self, k: &Key) -> Option<E3> {
        let v = BigUint::from_slice(&[
            k[0] as u32,
            (k[0] >> 32) as u32,
            k[1] as u32,
            (k[1] >> 32) as u32,
            k[2] as u32,
            (k[2] >> 32) as u32,
        ]);
        self.c.field.embed_big(&v).ok()
    }
    fn random_point<R: Rng>(&self, rng: &mut R) -> Pt<E3> {
        use num_bigint::RandBigInt;
        let p = self.c.field.modulus();
        loop {
            let x = self.embed(&rng.gen_biguint_below(&p));
            if let Some(pt) = self.lift_x(&x) {
                return if rng.gen() { self.neg(&pt) } else { pt };
            }
        }
    }

    fn add_batch(&self, acc: &mut [Pt<E3>], jumps: &[Pt<E3>]) -> usize {
        let mut f = self.c.field.clone();
        let den: Vec<E3> = acc
            .iter()
            .zip(jumps)
            .map(|(a, j)| f.sub(j.x, a.x))
            .collect();
        let mut prefix = Vec::with_capacity(den.len());
        let mut prod = f.one();
        for d in &den {
            prefix.push(prod);
            if !f.is_zero(*d) {
                prod = f.mul(prod, *d);
            }
        }
        let mut inv = f.inv(prod).expect("nonzero product");
        let mut skipped = 0;
        for i in (0..den.len()).rev() {
            if f.is_zero(den[i]) {
                skipped += 1;
                continue;
            }
            let di = f.mul(inv, prefix[i]);
            inv = f.mul(inv, den[i]);
            let (a, j) = (acc[i], jumps[i]);
            let dy = f.sub(j.y, a.y);
            let lam = f.mul(dy, di);
            let l2 = f.sqr(lam);
            let sx = f.add(a.x, j.x);
            let x3 = f.sub(l2, sx);
            let dx = f.sub(a.x, x3);
            let t = f.mul(lam, dx);
            let y3 = f.sub(t, a.y);
            acc[i] = Pt { x: x3, y: y3 };
        }
        skipped
    }

    fn plus_minus_keys(&self, t: &Pt<E3>, ps: &[Pt<E3>], out: &mut Vec<[Key; 2]>) {
        let mut f = self.c.field.clone();
        out.clear();
        // Batched inversion of x_P − x_T.
        let den: Vec<E3> = ps.iter().map(|p| f.sub(p.x, t.x)).collect();
        let mut prefix = Vec::with_capacity(den.len());
        let mut acc = f.one();
        for d in &den {
            prefix.push(acc);
            if !f.is_zero(*d) {
                acc = f.mul(acc, *d);
            }
        }
        let mut inv = f.inv(acc).expect("nonzero product");
        let mut invs = vec![f.zero(); den.len()];
        for i in (0..den.len()).rev() {
            if f.is_zero(den[i]) {
                continue;
            }
            invs[i] = f.mul(inv, prefix[i]);
            inv = f.mul(inv, den[i]);
        }
        let sx = |f: &mut Fp3, p: &Pt<E3>| f.add(t.x, p.x);
        for (p, inv) in ps.iter().zip(invs.iter()) {
            if f.is_zero(*inv) {
                out.push([NO_KEY, NO_KEY]);
                continue;
            }
            // T + P: λ = (y_P − y_T)/(x_P − x_T); T − P: λ' = (−y_P − y_T)/(x_P − x_T).
            let s = sx(&mut f, p);
            let num_p = f.sub(p.y, t.y);
            let sum_y = f.add(p.y, t.y);
            let num_m = f.neg(sum_y);
            let lp = f.mul(num_p, *inv);
            let lm = f.mul(num_m, *inv);
            let lp2 = f.sqr(lp);
            let lm2 = f.sqr(lm);
            let xp = f.sub(lp2, s);
            let xm = f.sub(lm2, s);
            out.push([f.key(xp), f.key(xm)]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    fn check_curve<C: SymCurve>(c: &C, g: &Pt<C::E>, q: &Pt<C::E>) {
        assert!(c.on_curve(g) && c.on_curve(q));
        let r = c.order().expect("order");
        assert!(c.mul(g, r).is_none(), "[r]G = O");
        assert!(c.mul(q, r).is_none(), "[r]Q = O");
        assert!(c.in_subgroup(g) && c.in_subgroup(q));
    }

    fn check_plus_minus<C: SymCurve>(c: &C, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let t = c.random_point(&mut rng);
        let ps: Vec<Pt<C::E>> = (0..17).map(|_| c.random_point(&mut rng)).collect();
        let mut shared = Vec::new();
        let mut unshared = Vec::new();
        c.plus_minus_keys(&t, &ps, &mut shared);
        c.plus_minus_keys_unshared(&t, &ps, &mut unshared);
        assert_eq!(shared, unshared);
    }

    #[test]
    fn published_ecc2_131_points_are_in_the_subgroup() {
        let (c, g, q) = BinaryW::ecc2_131();
        check_curve(&c, &g, &q);
        // The trace criterion agrees with [r]P = O on random points.
        let mut rng = StdRng::seed_from_u64(1);
        let r = c.order.clone().unwrap();
        for _ in 0..8 {
            let x = c.f.random(&mut rng);
            let x2 = c.f.sqr(&x);
            let rhs = gf2w::xor(gf2w::xor(x, c.a), c.f.mul(&c.b, &c.f.inv(&x2)));
            let Some(z) = c.f.solve_quadratic(&rhs) else {
                continue;
            };
            let p = Pt {
                x,
                y: c.f.mul(&x, &z),
            };
            assert!(c.on_curve(&p));
            assert_eq!(c.in_subgroup(&p), c.mul(&p, &r).is_none());
        }
    }

    #[test]
    fn published_eccp_131_points_are_in_the_subgroup() {
        let (c, g, q) = PrimeW::eccp_131();
        check_curve(&c, &g, &q);
    }

    fn check_add_batch<C: SymCurve>(c: &C, seed: u64) {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut acc: Vec<Pt<C::E>> = (0..33).map(|_| c.random_point(&mut rng)).collect();
        let jumps: Vec<Pt<C::E>> = (0..33).map(|_| c.random_point(&mut rng)).collect();
        let want: Vec<Pt<C::E>> = acc
            .iter()
            .zip(&jumps)
            .map(|(a, j)| c.add(a, j).unwrap())
            .collect();
        assert_eq!(c.add_batch(&mut acc, &jumps), 0);
        assert_eq!(acc, want);
    }

    #[test]
    fn batched_walk_step_equals_single_additions() {
        check_add_batch(&BinaryW::ecc2_131().0, 7);
        check_add_batch(&PrimeW::eccp_131().0, 8);
    }

    #[test]
    fn shared_plus_minus_keys_equal_separate_additions() {
        check_plus_minus(&BinaryW::ecc2_131().0, 2);
        check_plus_minus(&PrimeW::eccp_131().0, 3);
        let mut rng = StdRng::seed_from_u64(4);
        check_plus_minus(&BinaryW::random(67, &mut rng), 5);
        check_plus_minus(&PrimeW::random(67, &mut rng), 6);
    }
}
