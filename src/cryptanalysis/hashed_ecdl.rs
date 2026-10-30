//! Executable reference for §3 of Bochum's *Subexponential ECDL*, v0.2.
//!
//! This implements affine-hashed factor bases and the index-calculus reduction,
//! with a separate complete tensor implementation exported as [`tensor`].
//! This bounded reference's default oracle enumerates group decompositions (generic
//! decomposition, bounded by sqrt(r); cannot beat rho). No subexponential or
//! performance claim is made. Cofactor-one prime-field curves below 2^63 are
//! supported using an exact prime-order witness; the original constructor
//! retains its small-field exhaustive check. Answers carry replayable certificates.
//!
//! Source: <https://github.com/bochum-crypto/subexponential-ecdl>, commit
//! `1ee44e562b1b9a50ea339f3b8f81112b5a24b96f`, §3.2–3.5, Figure 2.
//! The default CPU reduction matches cryptanalysis's Rust binding. Crypto also
//! exposes an optional base-construction hook for its standalone GPU adapter.

/// Arbitrary-precision compact encodings, recursive contraction and reduction.
pub use hashed_ecdl_tensor as tensor;

use std::collections::BTreeMap;

/// Affine coordinates; `None` denotes the identity.
pub type Point = Option<(u64, u64)>;

/// A rejected input, exhausted bound, or invalid oracle/certificate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid parameters or noncanonical coordinates.
    Invalid(&'static str),
    /// Work stopped; this is not a proof of nonexistence.
    Limit(&'static str),
    /// A proposed decomposition or certificate failed independent replay.
    Verification,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[clippy::msrv = "1.78"]
fn prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    // Deterministic Miller-Rabin witness set for the entire u64 domain.
    for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 {
            continue;
        }
        let mut x = pow(a % n, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut passed = false;
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                passed = true;
                break;
            }
        }
        if !passed {
            return false;
        }
    }
    true
}
fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    if p <= u64::from(u32::MAX) {
        a * b % p
    } else {
        (u128::from(a) * u128::from(b) % u128::from(p)) as u64
    }
}
// Reciprocal reduction amortizes division across products (HAC 14.3.3).
// This ceiling-reciprocal variant is restricted to canonical 32-bit residues.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Modulus {
    p: u64,
    reciprocal: u64,
}
impl Modulus {
    fn new(p: u64) -> Self {
        assert!(p > 1);
        Self {
            p,
            reciprocal: if p <= u64::from(u32::MAX) {
                u64::MAX / p + 1
            } else {
                0
            },
        }
    }
    fn mul(self, a: u64, b: u64) -> u64 {
        debug_assert!(a < self.p && b < self.p);
        if self.reciprocal == 0 {
            return mulmod(a, b, self.p);
        }
        let x = a * b;
        // With R=ceil(2^64/p), floor(xR/2^64) is floor(x/p) or one
        // larger, since x<2^64. The wrapped residual is either in [0,p)
        // or [2^64-p,2^64). These intervals are disjoint for p<2^32.
        let q = ((u128::from(x) * u128::from(self.reciprocal)) >> 64) as u64;
        let residual = x.wrapping_sub(q.wrapping_mul(self.p));
        if residual >= self.p {
            residual.wrapping_add(self.p)
        } else {
            residual
        }
    }
    fn pow(self, mut a: u64, mut n: u64) -> u64 {
        let mut r = 1;
        while n != 0 {
            if n & 1 != 0 {
                r = self.mul(r, a);
            }
            a = self.mul(a, a);
            n >>= 1;
        }
        r
    }
}
fn pow(a: u64, n: u64, p: u64) -> u64 {
    Modulus::new(p).pow(a, n)
}

fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

// Extended Euclid (HAC 2.107/2.142), for nonzero residues modulo a
// validated prime below 2^63 (including hash moduli). Public research
// inputs: variable time. Signed intermediates fit within i128.
pub(crate) fn inverse(a: u64, p: u64) -> u64 {
    debug_assert!(a > 0 && a < p && p < (1 << 63));
    let (mut r, mut next) = (p, a);
    let (mut t, mut next_t) = (0i128, 1i128);
    while next != 0 {
        let q = r / next;
        (r, next) = (next, r - q * next);
        (t, next_t) = (next_t, t - i128::from(q) * next_t);
    }
    debug_assert_eq!(r, 1);
    if t < 0 {
        (t + i128::from(p)) as u64
    } else {
        t as u64
    }
}

// For a validated prime p <= 4096, p = q*a + r gives
// a^-1 = -q*r^-1 mod p, where 0 < r < a. Charge this bounded
// (at most 32 KiB) field-arithmetic table to each Curve construction.
fn small_inverses(p: u64) -> Vec<u64> {
    if p > 4096 {
        return Vec::new();
    }
    let mut table = vec![0; p as usize];
    table[1] = 1;
    for a in 2..p {
        table[a as usize] = p - (p / a * table[(p % a) as usize]) % p;
    }
    table
}

fn inverse_with_table(a: u64, p: u64, table: &[u64]) -> u64 {
    debug_assert!(a > 0 && a < p);
    if table.is_empty() {
        inverse(a, p)
    } else {
        table[a as usize]
    }
}

fn addmod(a: u64, b: u64, p: u64) -> u64 {
    if p >= (1u64 << 63) {
        return if a >= p - b { a - (p - b) } else { a + b };
    }
    let sum = a + b;
    if sum >= p {
        sum - p
    } else {
        sum
    }
}

fn floor_sqrt(n: u128) -> u64 {
    // Only Hasse radicands <=4*(2^63-1), plus u64 test inputs.
    assert!(n < (1u128 << 65));
    let (mut lo, mut hi) = (0u64, 1u64 << 33);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if u128::from(mid) * u128::from(mid) <= n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

#[cfg(test)]
fn sqrtmod(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if pow(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow(a, (p + 1) / 4, p));
    }
    let mut m = (p - 1).trailing_zeros();
    let q = (p - 1) >> m;
    let z = (2..p).find(|&z| pow(z, (p - 1) / 2, p) == p - 1)?;
    let mut c = pow(z, q, p);
    let mut x = pow(a, q.div_ceil(2), p);
    let mut t = pow(a, q, p);
    while t != 1 {
        let (mut power, mut i) = (t, 0);
        while power != 1 && i < m {
            power = mulmod(power, power, p);
            i += 1;
        }
        if i == m {
            return None;
        }
        let b = pow(c, 1u64 << (m - i - 1), p);
        x = mulmod(x, b, p);
        c = mulmod(b, b, p);
        t = mulmod(t, c, p);
        m = i;
    }
    (mulmod(x, x, p) == a).then_some(x)
}

// Tonelli-Shanks with the field-dependent work retained once per curve.
// See HAC 3.34/3.36. The input-dependent exponent is shared between the
// candidate root, residual and Legendre test; the final square is checked.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SqrtContext {
    modulus: Modulus,
    m: u32,
    q: u64,
    c: u64,
}
impl SqrtContext {
    fn new(p: u64) -> Self {
        let m = (p - 1).trailing_zeros();
        let q = (p - 1) >> m;
        let c = if m == 1 {
            0
        } else {
            let z = (2..p)
                .find(|&z| pow(z, (p - 1) / 2, p) == p - 1)
                .expect("validated odd prime has a quadratic nonresidue");
            pow(z, q, p)
        };
        Self {
            modulus: Modulus::new(p),
            m,
            q,
            c,
        }
    }
    fn root(&self, a: u64, p: u64) -> Option<u64> {
        if a == 0 {
            return Some(0);
        }
        if self.m == 1 {
            let x = self.modulus.pow(a, (p + 1) / 4);
            return (self.modulus.mul(x, x) == a).then_some(x);
        }
        let w = self.modulus.pow(a, self.q / 2);
        let mut x = self.modulus.mul(w, a);
        let mut t = self.modulus.mul(w, x);
        let mut legendre = t;
        for _ in 1..self.m {
            legendre = self.modulus.mul(legendre, legendre);
        }
        if legendre != 1 {
            return None;
        }
        let (mut m, mut c) = (self.m, self.c);
        while t != 1 {
            let (mut power, mut i) = (t, 0);
            while power != 1 && i < m {
                power = self.modulus.mul(power, power);
                i += 1;
            }
            if i == m {
                return None;
            }
            let mut b = c;
            for _ in 0..m - i - 1 {
                b = self.modulus.mul(b, b);
            }
            x = self.modulus.mul(x, b);
            c = self.modulus.mul(b, b);
            t = self.modulus.mul(t, c);
            m = i;
        }
        (self.modulus.mul(x, x) == a).then_some(x)
    }
}

/// Explicit CPU reduction memory ceiling; callers must opt in above 512.
/// Two dense triangular workspaces can require several GiB at this ceiling.
pub const MAX_COLUMNS: usize = 16384;
/// Bounded ordered factor-base slots shared with the standalone GPU service.
pub const MAX_SLOTS: usize = 12;
/// Bound inverse-hash enumeration, including independent certificate replay.
pub const MAX_HASH_CANDIDATES: u64 = 1 << 24;

/// Validated short-Weierstrass curve of odd prime cardinality, cofactor one.
/// The tiny constructor counts points or finds an order witness; the scalable
/// constructor proves order from a supplied witness and the Hasse interval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    // Tiny constructor's lookup; empty for the order-witness constructor.
    roots: Vec<Option<u64>>,
    sqrt: Option<SqrtContext>,
    inverses: Vec<u64>,
    field: Modulus,
}

impl Curve {
    pub(crate) fn field_inverse(&self, a: u64) -> u64 {
        inverse_with_table(a, self.p, &self.inverses)
    }

    /// Certify cofactor one without scanning the field. A nonzero point killed
    /// by prime r has exact order r. The unique multiple of r in the integer
    /// Hasse interval must be r itself. No unverified cardinality is accepted.
    pub fn with_order_witness(
        p: u64,
        a: u64,
        b: u64,
        r: u64,
        generator: Point,
    ) -> Result<Self, Error> {
        if !(5..(1u64 << 63)).contains(&p)
            || !(3..(1u64 << 63)).contains(&r)
            || !prime(p)
            || !prime(r)
            || a >= p
            || b >= p
        {
            return Err(Error::Invalid("prime field/order witness parameters"));
        }
        if (mulmod(4, mulmod(mulmod(a, a, p), a, p), p) + mulmod(27 % p, mulmod(b, b, p), p))
            .is_multiple_of(p)
        {
            return Err(Error::Invalid("singular curve"));
        }
        let curve = Self {
            p,
            a,
            b,
            r,
            roots: Vec::new(),
            sqrt: Some(SqrtContext::new(p)),
            inverses: small_inverses(p),
            field: Modulus::new(p),
        };
        if generator.is_none() || !curve.contains(generator) || curve.mul(generator, r)?.is_some() {
            return Err(Error::Invalid("invalid prime-order witness"));
        }
        let radius = floor_sqrt(4 * u128::from(p));
        let (lower, upper) = (p + 1 - radius, p + 1 + radius);
        if lower.div_ceil(r) != 1 || upper / r != 1 {
            // Preserve exact validation for ambiguous tiny Hasse intervals.
            return Self::new(p, a, b, r);
        }
        Ok(curve)
    }
    /// Check primality, nonsingularity, and the exact prime group order.
    #[clippy::msrv = "1.78"]
    pub fn new(p: u64, a: u64, b: u64, r: u64) -> Result<Self, Error> {
        if !(5..=65521).contains(&p) || !(3..=131044).contains(&r) || !prime(p) || !prime(r) {
            return Err(Error::Invalid(
                "small prime field and odd prime order required",
            ));
        }
        if a >= p || b >= p || (4 * a * a % p * a + 27 * b * b) % p == 0 {
            return Err(Error::Invalid("noncanonical or singular curve"));
        }
        let radius = floor_sqrt(4 * u128::from(p));
        if p >= 1024 && (p + 1 - radius).div_ceil(r) == 1 && (p + 1 + radius) / r == 1 {
            // The same exact certificate as with_order_witness, discovering
            // a public point rather than enumerating every x in the field.
            // Prime r and a nonidentity r-torsion point prove r divides #E;
            // the unique multiple in the Hasse interval proves #E = r.
            let curve = Self {
                p,
                a,
                b,
                r,
                roots: Vec::new(),
                sqrt: Some(SqrtContext::new(p)),
                inverses: small_inverses(p),
                field: Modulus::new(p),
            };
            let witness = (0..p).find_map(|x| curve.decode(2 * x));
            if witness.is_some() && curve.mul(witness, r)?.is_none() {
                return Ok(curve);
            }
            return Err(Error::Invalid(
                "declared order is not the full curve cardinality",
            ));
        }
        Self::counted(p, a, b, r)
    }

    // Parameters must already have passed primality and nonsingularity checks.
    fn counted(p: u64, a: u64, b: u64, r: u64) -> Result<Self, Error> {
        let mut roots = vec![None; p as usize];
        // y^2 advances by 2*y+1. The smaller root is in 0..=p/2,
        // so this fills exactly the same canonical table with no divisions.
        let (mut square, mut delta) = (0, 1);
        for y in 0..=p / 2 {
            roots[square as usize] = Some(y);
            square = addmod(square, delta, p);
            delta = addmod(delta, 2, p);
        }
        let curve = Self {
            p,
            a,
            b,
            r,
            roots,
            sqrt: None,
            inverses: small_inverses(p),
            field: Modulus::new(p),
        };
        // Finite differences of x^3+a*x+b: first difference at zero
        // is 1+a, second is 6, and the third difference is constant 6.
        let (mut rhs, mut first, mut second) = (b, (1 + a) % p, 6 % p);
        let mut count = 1;
        for _ in 0..p {
            count += match curve.roots[rhs as usize] {
                None => 0,
                Some(0) => 1,
                Some(_) => 2,
            };
            rhs = addmod(rhs, first, p);
            first = addmod(first, second, p);
            second = addmod(second, 6 % p, p);
        }
        if count != r {
            return Err(Error::Invalid(
                "declared order is not the full curve cardinality",
            ));
        }
        Ok(curve)
    }

    /// Prime field modulus.
    pub fn modulus(&self) -> u64 {
        self.p
    }
    /// Canonical short-Weierstrass coefficients `(a, b)` for external backends.
    pub fn coefficients(&self) -> (u64, u64) {
        (self.a, self.b)
    }
    /// Verified full group cardinality.
    pub fn order(&self) -> u64 {
        self.r
    }
    fn rhs(&self, x: u64) -> u64 {
        addmod(
            addmod(
                self.field.mul(self.field.mul(x, x), x),
                self.field.mul(self.a, x),
                self.p,
            ),
            self.b,
            self.p,
        )
    }

    /// Test canonical coordinates and the curve equation; accepts the identity.
    pub fn contains(&self, point: Point) -> bool {
        match point {
            None => true,
            Some((x, y)) => x < self.p && y < self.p && self.field.mul(y, y) == self.rhs(x),
        }
    }

    /// Injective nonidentity encoding `2*x + (y mod 2)` from §3.2.
    pub fn encode(&self, point: Point) -> Result<u64, Error> {
        if !self.contains(point) {
            return Err(Error::Invalid("point"));
        }
        point
            .map(|(x, y)| 2 * x + (y & 1))
            .ok_or(Error::Invalid("identity has no factor-base encoding"))
    }

    /// Invert the compressed encoding, rejecting absent roots and wrong parity.
    pub fn decode(&self, z: u64) -> Point {
        if z >= 2 * self.p {
            return None;
        }
        let x = z / 2;
        let mut y = if self.roots.is_empty() {
            self.sqrt.as_ref()?.root(self.rhs(x), self.p)?
        } else {
            self.roots[self.rhs(x) as usize]?
        };
        if y & 1 != z & 1 {
            y = (self.p - y) % self.p;
        }
        (y & 1 == z & 1).then_some((x, y))
    }

    fn neg(&self, a: Point) -> Point {
        a.map(|(x, y)| (x, (self.p - y) % self.p))
    }
    fn add_valid(&self, a: Point, b: Point) -> Point {
        let (x, y) = match a {
            None => return b,
            Some(p) => p,
        };
        let (xx, yy) = match b {
            None => return a,
            Some(p) => p,
        };
        let p = self.p;
        if x == xx && addmod(y, yy, p) == 0 {
            return None;
        }
        let (num, den) = if x == xx {
            let square = self.field.mul(x, x);
            (
                addmod(addmod(addmod(square, square, p), square, p), self.a, p),
                addmod(y, y, p),
            )
        } else {
            (sub(yy, y, p), sub(xx, x, p))
        };
        let slope = self.field.mul(num, self.field_inverse(den));
        let nx = sub(sub(self.field.mul(slope, slope), x, p), xx, p);
        Some((nx, sub(self.field.mul(slope, sub(x, nx, p)), y, p)))
    }

    /// Checked affine addition, including identity, inverses, and doubling.
    pub fn add(&self, a: Point, b: Point) -> Result<Point, Error> {
        if !self.contains(a) || !self.contains(b) {
            return Err(Error::Invalid("point"));
        }
        Ok(self.add_valid(a, b))
    }

    /// Checked scalar multiplication (variable time, public research inputs).
    pub fn mul(&self, mut a: Point, mut n: u64) -> Result<Point, Error> {
        if !self.contains(a) {
            return Err(Error::Invalid("point"));
        }
        let mut out = None;
        while n != 0 {
            if n & 1 != 0 {
                out = self.add_valid(out, a);
            }
            a = self.add_valid(a, a);
            n >>= 1;
        }
        Ok(out)
    }
}

/// One affine hash and its threshold, defining a slot-specific factor base.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashBase {
    /// Prime hash modulus, in `[2p,4p]` and representable as u64.
    pub s: u64,
    /// Nonzero multiplier modulo `s`.
    pub alpha: u64,
    /// Additive offset modulo `s`.
    pub beta: u64,
    /// Membership is `(alpha*enc(A)+beta) mod s < tau`.
    pub tau: u64,
}

impl HashBase {
    fn validate_parameters(&self, curve: &Curve) -> Result<(), Error> {
        if self.s < 2 * curve.p
            || u128::from(self.s) > 4 * u128::from(curve.p)
            || self.alpha == 0
            || self.alpha >= self.s
            || self.beta >= self.s
            || self.tau == 0
            || self.tau > self.s
        {
            return Err(Error::Invalid("hash parameters"));
        }
        Ok(())
    }

    fn validate(&self, curve: &Curve) -> Result<(), Error> {
        self.validate_parameters(curve)?;
        if !prime(self.s) {
            return Err(Error::Invalid("hash parameters"));
        }
        Ok(())
    }

    /// Enumerate by inverse hashing, not by scanning all curve points (§3.5).
    pub fn enumerate(&self, curve: &Curve) -> Result<Vec<Point>, Error> {
        self.validate(curve)?;
        self.enumerate_validated(curve)
    }

    // Callers must establish all parameters and the primality of s locally.
    fn enumerate_validated(&self, curve: &Curve) -> Result<Vec<Point>, Error> {
        if self.tau > MAX_HASH_CANDIDATES {
            return Err(Error::Limit("inverse-hash candidate bound"));
        }
        // Keep the established signed EEA bound; use Fermat for full-u64 hashes.
        let inv = if self.s < (1u64 << 63) {
            inverse(self.alpha, self.s)
        } else {
            pow(self.alpha, self.s - 2, self.s)
        };
        let mut points = Vec::new();
        // z(t) = alpha^-1 * (t-beta) mod s, so z(t+1)=z(t)+inv mod s.
        // Both operands are canonical; addmod also handles full-u64 hash moduli.
        let mut z = mulmod(inv, sub(0, self.beta, self.s), self.s);
        for _ in 0..self.tau {
            if let Some(point) = curve.decode(z) {
                points.push(Some(point));
            }
            z = addmod(z, inv, self.s);
        }
        points.sort_unstable();
        Ok(points)
    }
}

// Each invocation establishes its own prime-modulus facts; no fact is shared
// between collection and independent certificate replays. Only consecutive
// identical moduli reuse primality, while every hash's bounds are checked.
fn enumerate_hash_bases(curve: &Curve, hashes: &[HashBase]) -> Result<Vec<Vec<Point>>, Error> {
    let mut checked_prime = None;
    hashes
        .iter()
        .map(|h| {
            h.validate_parameters(curve)?;
            if checked_prime != Some(h.s) {
                if !prime(h.s) {
                    return Err(Error::Invalid("hash parameters"));
                }
                checked_prime = Some(h.s);
            }
            h.enumerate_validated(curve)
        })
        .collect()
}

/// Deterministic decomposition interface. The oracle receives no `u`, `v`,
/// generator, or target logarithm. `None` must certify exhaustive nonexistence;
/// a resource limit must return `Error::Limit` instead.
pub trait DecompositionOracle {
    /// Construct exactly the sorted signed bases defined by these hashes.
    /// The default retains CPU enumeration; accelerators may override it.
    /// Returned points are checked before use, and certificate replay always
    /// reconstructs the complete bases independently on the CPU. Completeness
    /// is part of the backend contract; a truncated search must return a limit.
    fn enumerate_bases(
        &mut self,
        curve: &Curve,
        hashes: &[HashBase],
    ) -> Result<Vec<Vec<Point>>, Error> {
        enumerate_hash_bases(curve, hashes)
    }

    /// Find an ordered slot-specific decomposition of `target`.
    fn find(
        &mut self,
        curve: &Curve,
        bases: &[Vec<Point>],
        target: Point,
        max_work: u64,
    ) -> Result<Option<Vec<Point>>, Error>;
}

/// Generic reference oracle: enumerate the first `m-1` slots and look up the
/// final residual. This is not Boolean tensor contraction.
#[derive(Default)]
pub struct Enumerate;

impl DecompositionOracle for Enumerate {
    fn find(
        &mut self,
        curve: &Curve,
        bases: &[Vec<Point>],
        target: Point,
        max_work: u64,
    ) -> Result<Option<Vec<Point>>, Error> {
        if bases.len() < 2
            || bases.len() > MAX_SLOTS
            || !curve.contains(target)
            || bases.iter().any(|b| {
                b.iter().any(|&a| a.is_none() || !curve.contains(a))
                    || b.windows(2).any(|w| w[0] >= w[1])
            })
        {
            return Err(Error::Invalid("oracle input"));
        }
        if bases.iter().any(Vec::is_empty) {
            return Ok(None);
        }
        let mut indices = vec![0; bases.len() - 1];
        for _ in 0..max_work {
            let mut tuple: Vec<_> = indices
                .iter()
                .enumerate()
                .map(|(i, &j)| bases[i][j])
                .collect();
            let sum = tuple.iter().fold(None, |acc, &a| curve.add_valid(acc, a));
            let last = curve.add_valid(target, curve.neg(sum));
            if bases.last().unwrap().binary_search(&last).is_ok() {
                tuple.push(last);
                return Ok(Some(tuple));
            }
            let mut slot = indices.len();
            loop {
                if slot == 0 {
                    return Ok(None);
                }
                slot -= 1;
                indices[slot] += 1;
                if indices[slot] < bases[slot].len() {
                    break;
                }
                indices[slot] = 0;
            }
        }
        Err(Error::Limit(
            "decomposition work; nonexistence not established",
        ))
    }
}

/// Bounded reduction parameters. Bounds make exhaustion explicit.
#[derive(Clone, Debug)]
pub struct Config {
    /// Number of ordered factor-base slots, 2 through MAX_SLOTS.
    pub m: usize,
    /// Override the Lemma 13 threshold; `None` uses its integer ceiling.
    pub tau: Option<u64>,
    /// Public reproducibility seed for SplitMix64; not a cryptographic RNG.
    pub seed: u64,
    /// Maximum number of fresh factor-base trials.
    pub max_trials: usize,
    /// Per-query oracle work bound (first `m-1` tuples for `Enumerate`).
    pub max_work: u64,
    /// Maximum total slot-specific columns, at most MAX_COLUMNS; default 512.
    pub max_columns: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            m: 2,
            tau: None,
            seed: 1,
            max_trials: 16,
            max_work: 1_000_000,
            max_columns: 512,
        }
    }
}

/// A checked relation `[u]P + [v]Q = sum(points)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relation {
    /// Coefficient of the generator, reduced modulo `r`.
    pub u: u64,
    /// Coefficient of the target, reduced modulo `r`.
    pub v: u64,
    /// One point per ordered factor-base slot.
    pub points: Vec<Point>,
}

/// Independently replayable certificate for the successful trial.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Certificate {
    /// Recovered logarithm, checked against the target before returning.
    pub scalar: u64,
    /// Hash seeds, kept fixed throughout the successful trial.
    pub hashes: Vec<HashBase>,
    /// Exactly `N+1` verified successful relations (except identity target).
    pub relations: Vec<Relation>,
    /// Deterministically chosen nonzero row dependency over `F_r`.
    pub dependency: Vec<u64>,
    /// Number of trials attempted, including the successful one.
    pub trials: usize,
    /// Total target queries, including unsuccessful and discarded trials.
    pub attempts: usize,
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        // Rejection removes modulo bias; SplitMix is still only a fixture RNG.
        let threshold = n.wrapping_neg() % n;
        loop {
            let x = self.next();
            if x >= threshold {
                return x % n;
            }
        }
    }
}

fn matrix(bases: &[Vec<Point>], relations: &[Relation]) -> Result<Vec<Vec<u64>>, Error> {
    let n: usize = bases.iter().map(Vec::len).sum();
    relations
        .iter()
        .map(|rel| {
            if rel.points.len() != bases.len() {
                return Err(Error::Verification);
            }
            let mut row = vec![0; n];
            let mut offset = 0;
            for (base, point) in bases.iter().zip(&rel.points) {
                let index = base.binary_search(point).map_err(|_| Error::Verification)?;
                row[offset + index] = 1;
                offset += base.len();
            }
            Ok(row)
        })
        .collect()
}

// Store sparse pivot tails only when their payload is no larger than dense.
// Pivot traversal and normalization stay unchanged, preserving the first
// dependent row and its exact recovered coefficients.
enum PivotTail {
    Sparse(Vec<(usize, u64)>),
    Dense(Vec<u64>),
}
impl PivotTail {
    fn new(row: &[u64]) -> Self {
        let nonzero = row.iter().filter(|&&x| x != 0).count();
        if nonzero * std::mem::size_of::<(usize, u64)>() <= std::mem::size_of_val(row) {
            let mut entries = Vec::with_capacity(nonzero);
            entries.extend(row.iter().copied().enumerate().filter(|&(_, x)| x != 0));
            Self::Sparse(entries)
        } else {
            Self::Dense(row.to_vec())
        }
    }
    fn subtract_from(&self, row: &mut [u64], k: u64, modulus: Modulus) {
        let r = modulus.p;
        let update = |a: &mut u64, b| {
            if b != 0 {
                let product = if k == 1 {
                    b
                } else if b == 1 {
                    k
                } else {
                    modulus.mul(k, b)
                };
                *a = sub(*a, product, r);
            }
        };
        match self {
            Self::Sparse(entries) => {
                for &(i, b) in entries {
                    update(&mut row[i], b);
                }
            }
            Self::Dense(entries) => {
                for (a, &b) in row.iter_mut().zip(entries) {
                    update(a, b);
                }
            }
        }
    }
}

// First row dependency, independent of u/v. Keep elimination factors instead
// of updating a full provenance vector at every pivot; reconstruct once by
// triangular back-substitution when the first dependent row is reached.
fn dependency(rows: &[Vec<u64>], r: u64) -> Option<Vec<u64>> {
    let modulus = Modulus::new(r);
    struct Pivot {
        tail: PivotTail,
        source: usize,
        inverse: u64,
        factors: Vec<(usize, u64)>,
    }
    let mut columns = BTreeMap::<usize, usize>::new();
    let mut pivots: Vec<Pivot> = Vec::new();
    for (i, source) in rows.iter().enumerate() {
        let mut row = source.clone();
        let mut factors = Vec::new();
        for (&col, &index) in &columns {
            let k = row[col];
            if k == 0 {
                continue;
            }
            factors.push((index, k));
            row[col] = 0; // the normalized pivot entry is exactly one
            pivots[index]
                .tail
                .subtract_from(&mut row[col + 1..], k, modulus);
        }
        match row.iter().position(|&a| a != 0) {
            None => {
                let mut combination = vec![0; rows.len()];
                combination[i] = 1;
                let mut weights = vec![0; pivots.len()];
                for (j, k) in factors {
                    weights[j] = sub(0, k, r);
                }
                for j in (0..pivots.len()).rev() {
                    let pivot = &pivots[j];
                    let k = modulus.mul(weights[j], pivot.inverse);
                    combination[pivot.source] = k;
                    if k != 0 {
                        for &(ancestor, factor) in &pivot.factors {
                            weights[ancestor] = sub(weights[ancestor], modulus.mul(k, factor), r);
                        }
                    }
                }
                return Some(combination);
            }
            Some(col) => {
                let inv = inverse(row[col], r);
                for a in &mut row[col + 1..] {
                    if *a != 0 && inv != 1 {
                        *a = modulus.mul(*a, inv);
                    }
                }
                columns.insert(col, pivots.len());
                pivots.push(Pivot {
                    tail: PivotTail::new(&row[col + 1..]),
                    source: i,
                    inverse: inv,
                    factors,
                });
            }
        }
    }
    None
}

fn extract(relations: &[Relation], coeff: &[u64], r: u64) -> Option<u64> {
    let (mut u, mut v) = (0, 0);
    for (rel, &c) in relations.iter().zip(coeff) {
        u = (u + mulmod(c, rel.u, r)) % r;
        v = (v + mulmod(c, rel.v, r)) % r;
    }
    (v != 0).then(|| mulmod(sub(0, u, r), inverse(v, r), r))
}

/// Run Figure 2 with the bounded generic enumeration backend.
pub fn solve(curve: &Curve, p: Point, q: Point, config: &Config) -> Result<Certificate, Error> {
    solve_with_oracle(curve, p, q, config, &mut Enumerate)
}

/// Run the reduction using a deterministic oracle. No runtime property of
/// an injected oracle is assumed or claimed. Every witness is checked.
pub fn solve_with_oracle(
    curve: &Curve,
    p: Point,
    q: Point,
    config: &Config,
    oracle: &mut impl DecompositionOracle,
) -> Result<Certificate, Error> {
    solve_reduction::<false>(curve, p, q, config, oracle, &mut ReductionStats::default())
}

/// Descriptive stage clocks, accumulated across all attempted trials.
/// These are not isolated benchmark measurements or an operation ledger.
#[derive(Default, Debug)]
pub struct ReductionStats {
    /// Hash-base construction and validation.
    pub bases: std::time::Duration,
    /// Target scalar multiplication and accepted relation checking.
    pub targets: std::time::Duration,
    /// Oracle calls, including preparation on the first query of each trial.
    pub oracle: std::time::Duration,
    /// Matrix construction, dependency and extraction.
    pub linear: std::time::Duration,
    /// Independent certificate replay before returning.
    pub replay: std::time::Duration,
}

/// The same reduction with accumulated diagnostic stage clocks.
pub fn solve_with_oracle_profile(
    curve: &Curve,
    p: Point,
    q: Point,
    config: &Config,
    oracle: &mut impl DecompositionOracle,
    stats: &mut ReductionStats,
) -> Result<Certificate, Error> {
    solve_reduction::<true>(curve, p, q, config, oracle, stats)
}

// One reduction body preserves identical random draws, bounds and verification.
// The ordinary API does not read clocks for statistics the caller discarded.
fn solve_reduction<const PROFILE: bool>(
    curve: &Curve,
    p: Point,
    q: Point,
    config: &Config,
    oracle: &mut impl DecompositionOracle,
    stats: &mut ReductionStats,
) -> Result<Certificate, Error> {
    use std::time::Instant;
    if p.is_none() || !curve.contains(p) || !curve.contains(q) {
        return Err(Error::Invalid("generator or target"));
    }
    if !(2..=MAX_SLOTS).contains(&config.m)
        || config.max_trials == 0
        || config.max_trials > 1000
        || config.max_columns == 0
        || config.max_columns > MAX_COLUMNS
        || config.max_work == 0
    {
        return Err(Error::Invalid("reduction bounds"));
    }
    if q.is_none() {
        return Ok(Certificate {
            scalar: 0,
            hashes: vec![],
            relations: vec![],
            dependency: vec![],
            trials: 0,
            attempts: 0,
        });
    }
    let upper = (4 * u128::from(curve.p)).min(u128::from(u64::MAX)) as u64;
    let s = (2 * curve.p..=upper)
        .find(|&s| prime(s))
        .ok_or(Error::Limit("no supported u64 hash prime in [2p,4p]"))?;
    // Binary search and saturating products avoid both a sqrt(r) setup loop
    // and overflow for large exponents; only comparison against r matters.
    let (mut lower, mut root) = (0, curve.r);
    while lower + 1 < root {
        let mid = (lower + root) / 2;
        let value = (0..config.m).fold(1u128, |v, _| v.saturating_mul(u128::from(mid)));
        if value < u128::from(curve.r) {
            lower = mid;
        } else {
            root = mid;
        }
    }
    let tau = config.tau.unwrap_or_else(|| {
        (u128::from(s) * u128::from(root.max(10 * config.m as u64)))
            .div_ceil(u128::from(curve.r - 1)) as u64
    });
    if tau == 0 || tau > s || tau > MAX_HASH_CANDIDATES {
        return Err(Error::Invalid(
            "threshold outside 1..=s; use a larger toy group or explicit tau",
        ));
    }
    let mut rng = Rng(config.seed);
    let mut attempts = 0;
    let stage = PROFILE.then(Instant::now);
    let fixed = FixedPair::new(curve, p, q);
    if let Some(stage) = stage {
        stats.targets += stage.elapsed();
    }
    for trial in 1..=config.max_trials {
        let stage = PROFILE.then(Instant::now);
        let hashes: Vec<_> = (0..config.m)
            .map(|_| HashBase {
                s,
                alpha: 1 + rng.below(s - 1),
                beta: rng.below(s),
                tau,
            })
            .collect();
        let bases = oracle.enumerate_bases(curve, &hashes)?;
        if bases.len() != hashes.len()
            || bases.iter().zip(&hashes).any(|(base, h)| {
                base.windows(2).any(|w| w[0] >= w[1])
                    || base.iter().any(|&point| match curve.encode(point) {
                        Ok(z) => addmod(mulmod(h.alpha, z, h.s), h.beta, h.s) >= h.tau,
                        Err(_) => true,
                    })
            })
        {
            return Err(Error::Verification);
        }
        let n: usize = bases.iter().map(Vec::len).sum();
        if let Some(stage) = stage {
            stats.bases += stage.elapsed();
        }
        if n > config.max_columns {
            return Err(Error::Limit("dense relation matrix columns"));
        }
        if bases.iter().any(Vec::is_empty) {
            continue;
        }
        let mut relations = Vec::new();
        for _ in 0..20 * (n + 1) {
            attempts += 1;
            let stage = PROFILE.then(Instant::now);
            let (u, v) = (rng.below(curve.r), rng.below(curve.r));
            let target = fixed.evaluate(curve, u, v);
            if let Some(stage) = stage {
                stats.targets += stage.elapsed();
            }
            let stage = PROFILE.then(Instant::now);
            let found = oracle.find(curve, &bases, target, config.max_work);
            if let Some(stage) = stage {
                stats.oracle += stage.elapsed();
            }
            if let Some(points) = found? {
                let stage = PROFILE.then(Instant::now);
                let rel = Relation { u, v, points };
                check_collected_relation(curve, &bases, &rel, target)?;
                relations.push(rel);
                if let Some(stage) = stage {
                    stats.targets += stage.elapsed();
                }
                if relations.len() == n + 1 {
                    break;
                }
            }
        }
        if relations.len() != n + 1 {
            continue;
        }
        let stage = PROFILE.then(Instant::now);
        let coeff = dependency(&matrix(&bases, &relations)?, curve.r).ok_or(Error::Verification)?;
        let scalar = extract(&relations, &coeff, curve.r);
        if let Some(stage) = stage {
            stats.linear += stage.elapsed();
        }
        if let Some(scalar) = scalar {
            let cert = Certificate {
                scalar,
                hashes,
                relations,
                dependency: coeff,
                trials: trial,
                attempts,
            };
            let stage = PROFILE.then(Instant::now);
            verify(curve, p, q, &cert)?;
            if let Some(stage) = stage {
                stats.replay += stage.elapsed();
            }
            return Ok(cert);
        }
    }
    Err(Error::Limit(
        "factor-base trials exhausted; logarithm unresolved",
    ))
}

// Jacobian coordinates x=X/Z^2, y=Y/Z^3; Z=0 is the identity.
// EFD madd-2004-hmv and standard Jacobian doubling, with explicit equal,
// inverse and identity cases. Variable time on public research inputs.
#[derive(Clone, Copy)]
struct Jacobian {
    x: u64,
    y: u64,
    z: u64,
}
impl Jacobian {
    fn from_affine(point: Point) -> Self {
        point.map_or(Self { x: 0, y: 1, z: 0 }, |(x, y)| Self { x, y, z: 1 })
    }
    fn affine(self, modulus: Modulus, inverses: &[u64]) -> Point {
        if self.z == 0 {
            return None;
        }
        let iz = inverse_with_table(self.z, modulus.p, inverses);
        let iz2 = modulus.mul(iz, iz);
        Some((
            modulus.mul(self.x, iz2),
            modulus.mul(self.y, modulus.mul(iz, iz2)),
        ))
    }
    fn batch_affine(points: &[Self], modulus: Modulus, inverses: &[u64]) -> Vec<Point> {
        let mut prefix = Vec::with_capacity(points.len());
        let mut product = 1;
        for point in points {
            prefix.push(product);
            if point.z != 0 {
                product = modulus.mul(product, point.z);
            }
        }
        let mut inv = inverse_with_table(product, modulus.p, inverses);
        let mut out = vec![None; points.len()];
        for (i, point) in points.iter().enumerate().rev() {
            if point.z == 0 {
                continue;
            }
            let iz = modulus.mul(inv, prefix[i]);
            inv = modulus.mul(inv, point.z);
            let iz2 = modulus.mul(iz, iz);
            out[i] = Some((
                modulus.mul(point.x, iz2),
                modulus.mul(point.y, modulus.mul(iz, iz2)),
            ));
        }
        out
    }
    fn double(self, a: u64, modulus: Modulus) -> Self {
        if self.z == 0 || self.y == 0 {
            return Self::from_affine(None);
        }
        let p = modulus.p;
        let mul = |x, y| modulus.mul(x, y);
        let twice = |x| addmod(x, x, p);
        let xx = mul(self.x, self.x);
        let yy = mul(self.y, self.y);
        let yyyy = mul(yy, yy);
        let zz = mul(self.z, self.z);
        let s = twice(twice(mul(self.x, yy)));
        let m = addmod(addmod(twice(xx), xx, p), mul(a, mul(zz, zz)), p);
        let x = sub(mul(m, m), twice(s), p);
        let y = sub(mul(m, sub(s, x, p)), twice(twice(twice(yyyy))), p);
        Self {
            x,
            y,
            z: twice(mul(self.y, self.z)),
        }
    }
    fn add_affine(self, point: Point, a: u64, modulus: Modulus) -> Self {
        let Some((x, y)) = point else {
            return self;
        };
        if self.z == 0 {
            return Self::from_affine(point);
        }
        let p = modulus.p;
        let mul = |x, y| modulus.mul(x, y);
        let z2 = mul(self.z, self.z);
        let h = sub(mul(x, z2), self.x, p);
        let r = sub(mul(y, mul(z2, self.z)), self.y, p);
        if h == 0 {
            return if r == 0 {
                self.double(a, modulus)
            } else {
                Self::from_affine(None)
            };
        }
        let hh = mul(h, h);
        let hhh = mul(hh, h);
        let v = mul(self.x, hh);
        let nx = sub(sub(mul(r, r), addmod(v, v, p), p), hhh, p);
        let ny = sub(mul(r, sub(v, nx, p)), mul(self.y, hhh), p);
        Self {
            x: nx,
            y: ny,
            z: mul(self.z, h),
        }
    }
}

// Fixed-base radix-16 multiplication. Constructed inside each solve/replay,
// never shared between targets or excluded from the complete timing interval.
// A replay constructs its own tables and still checks every relation.
struct FixedPair {
    modulus: Modulus,
    p: Vec<[Point; 16]>,
    q: Vec<[Point; 16]>,
}
impl FixedPair {
    fn new(curve: &Curve, p: Point, q: Point) -> Self {
        let windows = (64 - (curve.r - 1).leading_zeros()).div_ceil(4) as usize;
        let modulus = curve.field;
        let table = |mut base| {
            let mut out = Vec::with_capacity(windows);
            for window in 0..windows {
                let mut row = [Jacobian::from_affine(None); 17];
                row[1] = Jacobian::from_affine(base);
                for digit in 2..17 {
                    row[digit] = row[digit - 1].add_affine(base, curve.a, modulus);
                }
                // Include 16*base in the same inversion for the next window.
                let affine = Jacobian::batch_affine(&row, modulus, &curve.inverses);
                let mut table_row = [None; 16];
                table_row.copy_from_slice(&affine[..16]);
                if window + 1 < windows {
                    base = affine[16];
                }
                out.push(table_row);
            }
            out
        };
        Self {
            modulus: curve.field,
            p: table(p),
            q: table(q),
        }
    }
    fn evaluate(&self, curve: &Curve, mut u: u64, mut v: u64) -> Point {
        debug_assert!(u < curve.r && v < curve.r);
        let mut out = Jacobian::from_affine(None);
        for (p, q) in self.p.iter().zip(&self.q) {
            out = out.add_affine(p[(u & 15) as usize], curve.a, self.modulus);
            out = out.add_affine(q[(v & 15) as usize], curve.a, self.modulus);
            u >>= 4;
            v >>= 4;
        }
        out.affine(self.modulus, &curve.inverses)
    }
}

// Collection already computed target=[u]P+[v]Q immediately before the query.
// Check every returned point and the affine tuple sum against that exact target.
// Only the collector uses this helper; independent certificate replay below
// always recomputes the target from the recorded u/v with check_relation.
fn check_collected_relation(
    curve: &Curve,
    bases: &[Vec<Point>],
    rel: &Relation,
    target: Point,
) -> Result<(), Error> {
    if rel.u >= curve.r || rel.v >= curve.r || rel.points.len() != bases.len() {
        return Err(Error::Verification);
    }
    let mut sum = None;
    for (base, &point) in bases.iter().zip(&rel.points) {
        if point.is_none() || !curve.contains(point) || base.binary_search(&point).is_err() {
            return Err(Error::Verification);
        }
        sum = curve.add_valid(sum, point);
    }
    if sum != target {
        return Err(Error::Verification);
    }
    Ok(())
}

fn check_relation(
    curve: &Curve,
    fixed: &FixedPair,
    bases: &[Vec<Point>],
    rel: &Relation,
) -> Result<(), Error> {
    if rel.u >= curve.r || rel.v >= curve.r || rel.points.len() != bases.len() {
        return Err(Error::Verification);
    }
    let mut sum = None;
    for (base, &point) in bases.iter().zip(&rel.points) {
        if point.is_none() || !curve.contains(point) || base.binary_search(&point).is_err() {
            return Err(Error::Verification);
        }
        sum = curve.add_valid(sum, point);
    }
    if sum != fixed.evaluate(curve, rel.u, rel.v) {
        return Err(Error::Verification);
    }
    Ok(())
}

/// Independently rebuild bases and columns, replay every relation, check a
/// nonzero modular dependency and `V != 0`, and verify `[scalar]P = Q`.
/// Trial/attempt counters are descriptive; replay does not attest their history.
pub fn verify(curve: &Curve, p: Point, q: Point, cert: &Certificate) -> Result<(), Error> {
    if p.is_none() || !curve.contains(p) || !curve.contains(q) || cert.scalar >= curve.r {
        return Err(Error::Verification);
    }
    if q.is_none() {
        return if cert.scalar == 0
            && cert.hashes.is_empty()
            && cert.relations.is_empty()
            && cert.dependency.is_empty()
            && cert.trials == 0
            && cert.attempts == 0
        {
            Ok(())
        } else {
            Err(Error::Verification)
        };
    }
    if !(2..=MAX_SLOTS).contains(&cert.hashes.len()) {
        return Err(Error::Verification);
    }
    let bases = enumerate_hash_bases(curve, &cert.hashes)?;
    let n: usize = bases.iter().map(Vec::len).sum();
    if n > MAX_COLUMNS
        || bases.iter().any(Vec::is_empty)
        || cert.relations.len() != n + 1
        || cert.dependency.len() != n + 1
        || cert.dependency.iter().all(|&c| c == 0)
        || cert.dependency.iter().any(|&c| c >= curve.r)
    {
        return Err(Error::Verification);
    }
    let fixed = FixedPair::new(curve, p, q);
    for rel in &cert.relations {
        check_relation(curve, &fixed, &bases, rel)?;
    }
    // A relation has exactly one entry per slot. Replay the dependency in
    // O(m * relations) space/time for entries, without a dense N-by-N matrix.
    let mut sums = vec![0u64; n];
    for (rel, &coefficient) in cert.relations.iter().zip(&cert.dependency) {
        let mut offset = 0;
        for (base, point) in bases.iter().zip(&rel.points) {
            let index = base.binary_search(point).map_err(|_| Error::Verification)?;
            sums[offset + index] = (sums[offset + index] + coefficient) % curve.r;
            offset += base.len();
        }
    }
    if sums.iter().any(|&x| x != 0) {
        return Err(Error::Verification);
    }
    if extract(&cert.relations, &cert.dependency, curve.r) != Some(cert.scalar)
        || curve.mul(p, cert.scalar)? != q
    {
        return Err(Error::Verification);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Curve, Point) {
        (Curve::new(211, 0, 7, 199).unwrap(), Some((3, 33)))
    }

    #[test]
    fn twelve_slot_replay_and_bounded_oracle() {
        let (curve, p) = fixture();
        let q = curve.mul(p, 42).unwrap();
        // Public known-answer certificate: each separate slot contains P only.
        // enc(P)=7, hence (enc(P)+424) mod431=0 is the sole accepted encoding.
        let hash = HashBase {
            s: 431,
            alpha: 1,
            beta: 424,
            tau: 1,
        };
        assert_eq!(hash.enumerate(&curve).unwrap(), vec![p]);
        for m in [9, 10, MAX_SLOTS] {
            let bases = vec![vec![p]; m];
            let target = curve.mul(p, m as u64).unwrap();
            assert_eq!(
                Enumerate.find(&curve, &bases, target, 1).unwrap(),
                Some(vec![p; m])
            );
            assert_eq!(Enumerate.find(&curve, &bases, None, 1).unwrap(), None);
            let mut relations = vec![
                Relation {
                    u: m as u64,
                    v: 0,
                    points: vec![p; m]
                };
                m + 1
            ];
            relations[m].u = (m as u64 + curve.r - 42) % curve.r;
            relations[m].v = 1;
            let mut dependency = vec![0; m + 1];
            dependency[0] = 1;
            dependency[m] = curve.r - 1;
            let cert = Certificate {
                scalar: 42,
                hashes: vec![hash; m],
                relations,
                dependency,
                trials: 1,
                attempts: m + 1,
            };
            verify(&curve, p, q, &cert).unwrap();
            let mut bad = cert.clone();
            bad.relations[0].points.pop();
            assert_eq!(verify(&curve, p, q, &bad), Err(Error::Verification));
            bad = cert;
            bad.hashes.resize(MAX_SLOTS + 1, hash);
            assert_eq!(verify(&curve, p, q, &bad), Err(Error::Verification));
        }
        let oversized = vec![vec![p]; MAX_SLOTS + 1];
        assert!(matches!(
            Enumerate.find(&curve, &oversized, p, 1),
            Err(Error::Invalid("oracle input"))
        ));
        let config = Config {
            m: MAX_SLOTS + 1,
            ..Config::default()
        };
        assert!(matches!(
            solve(&curve, p, q, &config),
            Err(Error::Invalid("reduction bounds"))
        ));
    }

    #[test]
    fn collected_relation_reuses_target_but_full_replay_recomputes_it() {
        let (curve, p) = fixture();
        let q = curve.mul(p, 42).unwrap();
        let config = Config {
            m: 3,
            tau: Some(40),
            seed: 111,
            max_trials: 16,
            ..Config::default()
        };
        let cert = solve(&curve, p, q, &config).unwrap();
        let bases: Vec<_> = cert
            .hashes
            .iter()
            .map(|h| h.enumerate(&curve).unwrap())
            .collect();
        let fixed = FixedPair::new(&curve, p, q);
        for rel in &cert.relations {
            let target = fixed.evaluate(&curve, rel.u, rel.v);
            assert_eq!(
                check_collected_relation(&curve, &bases, rel, target),
                check_relation(&curve, &fixed, &bases, rel)
            );
            let wrong = curve.add(target, p).unwrap();
            assert_eq!(
                check_collected_relation(&curve, &bases, rel, wrong),
                Err(Error::Verification)
            );
            for point in [None, Some((0, 0)), Some((211, 33))] {
                let mut bad = rel.clone();
                bad.points[0] = point;
                assert_eq!(
                    check_collected_relation(&curve, &bases, &bad, target),
                    Err(Error::Verification)
                );
            }
            let mut bad = rel.clone();
            bad.u = curve.r;
            assert_eq!(
                check_collected_relation(&curve, &bases, &bad, target),
                Err(Error::Verification)
            );
            bad = rel.clone();
            bad.points.pop();
            assert_eq!(
                check_collected_relation(&curve, &bases, &bad, target),
                Err(Error::Verification)
            );
        }
        let mut bad = cert.clone();
        bad.relations[0].u = (bad.relations[0].u + 1) % curve.r;
        assert_eq!(verify(&curve, p, q, &bad), Err(Error::Verification));
        verify(&curve, p, q, &cert).unwrap();
    }

    #[test]
    fn optional_profile_preserves_queries_certificates_and_stats() {
        struct Trace(Vec<Point>);
        impl DecompositionOracle for Trace {
            fn find(
                &mut self,
                c: &Curve,
                b: &[Vec<Point>],
                t: Point,
                w: u64,
            ) -> Result<Option<Vec<Point>>, Error> {
                self.0.push(t);
                Enumerate.find(c, b, t, w)
            }
        }
        let (curve, p) = fixture();
        for scalar in [0, 1, 42, 198] {
            for seed in [111, 222, 333] {
                let q = curve.mul(p, scalar).unwrap();
                let config = Config {
                    m: 3,
                    tau: Some(40),
                    seed,
                    max_trials: 16,
                    ..Config::default()
                };
                let mut plain = Trace(vec![]);
                let mut profiled = Trace(vec![]);
                let expected = solve_with_oracle(&curve, p, q, &config, &mut plain).unwrap();
                let marker = std::time::Duration::from_secs(1);
                let mut stats = ReductionStats {
                    bases: marker,
                    targets: marker,
                    oracle: marker,
                    linear: marker,
                    replay: marker,
                };
                let actual =
                    solve_with_oracle_profile(&curve, p, q, &config, &mut profiled, &mut stats)
                        .unwrap();
                assert_eq!(actual, expected);
                assert_eq!(plain.0, profiled.0);
                verify(&curve, p, q, &actual).unwrap();
                for clock in [
                    stats.bases,
                    stats.targets,
                    stats.oracle,
                    stats.linear,
                    stats.replay,
                ] {
                    assert!(clock >= marker);
                }
                let mut untouched = ReductionStats {
                    bases: marker,
                    targets: marker,
                    oracle: marker,
                    linear: marker,
                    replay: marker,
                };
                let no_clocks =
                    solve_reduction::<false>(&curve, p, q, &config, &mut Enumerate, &mut untouched)
                        .unwrap();
                assert_eq!(no_clocks, expected);
                for clock in [
                    untouched.bases,
                    untouched.targets,
                    untouched.oracle,
                    untouched.linear,
                    untouched.replay,
                ] {
                    assert_eq!(clock, marker);
                }
            }
        }
    }

    #[test]
    fn jacobian_mixed_add_matches_affine_with_nonunit_scales() {
        let (curve, generator) = fixture();
        let modulus = curve.field;
        let points: Vec<_> = (0..curve.r)
            .map(|k| curve.mul(generator, k).unwrap())
            .collect();
        for (i, &point) in points.iter().enumerate() {
            let z = (i as u64 % (curve.p - 1)) + 1;
            let projective = point.map_or(Jacobian::from_affine(None), |(x, y)| {
                let z2 = modulus.mul(z, z);
                Jacobian {
                    x: modulus.mul(x, z2),
                    y: modulus.mul(y, modulus.mul(z, z2)),
                    z,
                }
            });
            assert_eq!(projective.affine(modulus, &curve.inverses), point);
            assert_eq!(
                Jacobian::batch_affine(
                    &[
                        Jacobian::from_affine(None),
                        projective,
                        projective.double(curve.a, modulus)
                    ],
                    modulus,
                    &curve.inverses
                ),
                vec![None, point, curve.add(point, point).unwrap()]
            );
            for &other in &points {
                assert_eq!(
                    projective
                        .add_affine(other, curve.a, modulus)
                        .affine(modulus, &curve.inverses),
                    curve.add(point, other).unwrap()
                );
            }
        }
        // A point with y=0 doubles to identity, including a nonunit scale.
        let m = Modulus::new(7);
        assert_eq!(
            Jacobian { x: 0, y: 0, z: 3 }.double(1, m).affine(m, &[]),
            None
        );
    }

    #[test]
    fn reciprocal_products_match_wide_remainders() {
        let reference =
            |a: u64, b: u64, p: u64| (u128::from(a) * u128::from(b) % u128::from(p)) as u64;
        for p in 2..256 {
            let m = Modulus::new(p);
            for a in 0..p {
                for b in 0..p {
                    assert_eq!(m.mul(a, b), reference(a, b, p));
                }
            }
        }
        let mut rng = Rng(0x126799);
        for p in [
            65521,
            65536,
            65563,
            (1 << 31) - 1,
            1 << 31,
            (1 << 32) - 5,
            (1 << 32) - 1,
            1 << 32,
            (1 << 40) + 87,
            u64::MAX,
        ] {
            let m = Modulus::new(p);
            for a in [0, 1, p / 2, p - 2, p - 1] {
                for b in [0, 1, p / 2, p - 2, p - 1] {
                    assert_eq!(m.mul(a, b), reference(a, b, p));
                }
            }
            for _ in 0..10000 {
                let (a, b) = (rng.below(p), rng.below(p));
                assert_eq!(m.mul(a, b), reference(a, b, p));
            }
        }
    }

    #[test]
    fn cached_curve_arithmetic_matches_independent_remainders() {
        // Test oracle uses wide integer remainder and Fermat inversion, neither
        // the reciprocal reducer nor the inverse cache used by production.
        fn product(a: u64, b: u64, p: u64) -> u64 {
            (u128::from(a) * u128::from(b) % u128::from(p)) as u64
        }
        fn power(mut a: u64, mut n: u64, p: u64) -> u64 {
            let mut r = 1;
            while n != 0 {
                if n & 1 != 0 {
                    r = product(r, a, p);
                }
                a = product(a, a, p);
                n >>= 1;
            }
            r
        }
        fn add(a: Point, b: Point, c: &Curve) -> Point {
            let (Some((x, y)), Some((xx, yy))) = (a, b) else {
                return a.or(b);
            };
            let p = c.p;
            if x == xx && (y + yy) % p == 0 {
                return None;
            }
            let (n, d) = if x == xx {
                ((3 * product(x, x, p) + c.a) % p, 2 * y % p)
            } else {
                ((yy + p - y) % p, (xx + p - x) % p)
            };
            let slope = product(n, power(d, p - 2, p), p);
            let nx = (product(slope, slope, p) + 2 * p - x - xx) % p;
            Some((nx, (product(slope, (x + p - nx) % p, p) + p - y) % p))
        }
        for (p, a, b, r, g) in [
            (17, 2, 2, 19, Some((5, 1))),
            (211, 0, 7, 199, Some((3, 33))),
            (65521, 65518, 3, 65563, Some((0, 64894))),
            (
                1_099_511_627_563,
                1_099_511_627_560,
                56,
                1_099_512_551_701,
                Some((1, 15_267_301_451)),
            ),
        ] {
            let c = Curve::with_order_witness(p, a, b, r, g).unwrap();
            let mut points = vec![None];
            for _ in 1..r.min(200) {
                points.push(add(*points.last().unwrap(), g, &c));
            }
            for &x in &points {
                assert!(c.contains(x));
                for &y in &points {
                    assert_eq!(c.add(x, y).unwrap(), add(x, y, &c));
                }
            }
            let mut rng = Rng(0x9c43);
            for x in [0, 1, p / 2, p - 1]
                .into_iter()
                .chain((0..256).map(|_| rng.below(p)))
            {
                let rhs = (product(product(x, x, p), x, p) + product(a, x, p) + b) % p;
                assert_eq!(c.rhs(x), rhs);
                for y in [0, 1, p / 2, p - 1] {
                    assert_eq!(c.contains(Some((x, y))), product(y, y, p) == rhs);
                }
            }
            for invalid in [Some((p, 0)), Some((0, p)), Some((u64::MAX, u64::MAX))] {
                assert!(!c.contains(invalid));
                assert!(c.add(invalid, g).is_err());
            }
        }
    }

    #[test]
    fn bounded_inverse_tables_match_independent_references() {
        for p in [5, 7, 17, 211, 4093] {
            assert!(prime(p));
            let table = small_inverses(p);
            assert_eq!(table.len(), p as usize);
            for a in 1..p {
                let actual = inverse_with_table(a, p, &table);
                assert_eq!(actual, inverse(a, p));
                assert_eq!(actual, pow(a, p - 2, p));
                assert_eq!(a * actual % p, 1);
            }
        }
        for p in [4099, 65521, 2_305_843_009_213_693_951] {
            let table = small_inverses(p);
            assert!(table.is_empty());
            for a in [1, 2, p / 2, p - 1] {
                assert_eq!(inverse_with_table(a, p, &table), pow(a, p - 2, p));
            }
        }
        let (curve, generator) = fixture();
        let witnessed =
            Curve::with_order_witness(curve.p, curve.a, curve.b, curve.r, generator).unwrap();
        assert_eq!(curve.inverses, witnessed.inverses);
    }

    #[test]
    fn euclidean_inverses_match_fermat_reference() {
        for p in [5, 17, 211, 65521] {
            for a in 1..p {
                assert_eq!(inverse(a, p), pow(a, p - 2, p));
            }
        }
        for p in [
            1_099_511_627_563,
            2_305_843_009_213_693_951,
            9_223_372_036_854_775_783,
        ] {
            assert!(prime(p));
            let mut rng = Rng(0x12345678);
            for a in [1, 2, p / 2, p - 2, p - 1]
                .into_iter()
                .chain((0..4096).map(|_| 1 + rng.below(p - 1)))
            {
                assert_eq!(inverse(a, p), pow(a, p - 2, p));
            }
        }
    }

    #[test]
    fn cached_square_roots_match_uncached_reference() {
        for p in [5, 13, 17, 97, 211, 65521] {
            let context = SqrtContext::new(p);
            for a in 0..p {
                assert_eq!(context.root(a, p), sqrtmod(a, p));
            }
        }
        for p in [1_099_511_627_563, 2_305_843_009_213_693_951] {
            let context = SqrtContext::new(p);
            let mut rng = Rng(456);
            for _ in 0..4096 {
                let a = rng.below(p);
                assert_eq!(context.root(a, p), sqrtmod(a, p));
                let square = mulmod(a, a, p);
                assert_eq!(context.root(square, p), sqrtmod(square, p));
            }
        }
    }

    #[test]
    fn fixed_pair_matches_independent_binary_multiplication() {
        for (p, a, b, r, generator) in [
            (211, 0, 7, 199, Some((3, 33))),
            (
                1_099_511_627_563,
                1_099_511_627_560,
                56,
                1_099_512_551_701,
                Some((1, 15_267_301_451)),
            ),
        ] {
            let c = Curve::with_order_witness(p, a, b, r, generator).unwrap();
            let mut rng = Rng(123);
            for target in [None, generator, c.mul(generator, 42).unwrap()] {
                let fixed = FixedPair::new(&c, generator, target);
                for (u, v) in [(0, 0), (0, r - 1), (r - 1, 0), (r - 1, r - 1)]
                    .into_iter()
                    .chain((0..4096).map(|_| (rng.below(r), rng.below(r))))
                {
                    assert_eq!(
                        fixed.evaluate(&c, u, v),
                        c.add(c.mul(generator, u).unwrap(), c.mul(target, v).unwrap())
                            .unwrap()
                    );
                }
            }
        }
    }

    #[test]
    fn finite_difference_constructor_matches_exhaustive_reference() {
        for (p, a, b, r) in [(17, 2, 2, 19), (211, 0, 7, 199), (65521, 65518, 3, 65563)] {
            let curve = Curve::counted(p, a, b, r).unwrap();
            let automatic = Curve::new(p, a, b, r).unwrap();
            for z in 0..2 * p {
                assert_eq!(automatic.decode(z), curve.decode(z));
            }
            let mut roots = vec![None; p as usize];
            for y in 0..p {
                roots[(y * y % p) as usize].get_or_insert(y);
            }
            assert_eq!(curve.roots, roots);
            let count = 1
                + (0..p)
                    .map(|x| match roots[curve.rhs(x) as usize] {
                        None => 0,
                        Some(0) => 1,
                        Some(_) => 2,
                    })
                    .sum::<u64>();
            assert_eq!(count, r);
            assert!(Curve::new(p, a, b, r + 2).is_err());
        }
    }

    #[test]
    fn order_witness_and_square_roots_match_exhaustive_constructor() {
        for (p, a, b, r, generator) in [
            (17, 2, 2, 19, Some((5, 1))),
            (211, 0, 7, 199, Some((3, 33))),
        ] {
            let tiny = Curve::new(p, a, b, r).unwrap();
            let wide = Curve::with_order_witness(p, a, b, r, generator).unwrap();
            for z in 0..2 * p {
                assert_eq!(tiny.decode(z), wide.decode(z));
            }
            for scalar in 0..r {
                assert_eq!(tiny.mul(generator, scalar), wide.mul(generator, scalar));
            }
            assert!(Curve::with_order_witness(p, a, b, r, None).is_err());
            assert!(Curve::with_order_witness(p, a, b, r + 2, generator).is_err());
        }
        for p in [5, 13, 17, 97, 211] {
            for a in 0..p {
                let got = sqrtmod(a, p);
                assert_eq!(got.is_some(), (0..p).any(|x| x * x % p == a));
                if let Some(x) = got {
                    assert_eq!(x * x % p, a);
                }
            }
        }
    }

    #[test]
    fn wide_arithmetic_and_certified_40_bit_curve() {
        let p = 1_099_511_627_563;
        let r = 1_099_512_551_701;
        let generator = Some((1, 15_267_301_451));
        let c = Curve::with_order_witness(p, p - 3, 56, r, generator).unwrap();
        assert!(c.roots.is_empty());
        assert_eq!(c.mul(generator, r).unwrap(), None);
        assert_eq!(c.decode(c.encode(generator).unwrap()), generator);
        for a in [0, 1, p - 1, p / 2] {
            for b in [0, 1, p - 1, p / 2] {
                assert_eq!(
                    mulmod(a, b, p),
                    (u128::from(a) * u128::from(b) % u128::from(p)) as u64
                );
            }
        }
        for n in [0, 1, 4, 341, 561, 3215031751, 3825123056546413051] {
            assert!(!prime(n));
        }
        for n in [2, 17, 65537, p, r, 18446744073709551557] {
            assert!(prime(n));
        }
        for n in [0, 1, 2, 3, 4, 15, 16, 17, u64::MAX] {
            let x = u128::from(floor_sqrt(u128::from(n)));
            assert!(x * x <= u128::from(n) && (x + 1) * (x + 1) > u128::from(n));
        }
    }

    #[test]
    fn field63_full_hash_and_hasse_boundaries() {
        let p = 9_223_372_036_854_775_783u64;
        let s = 18_446_744_073_709_551_557u64;
        assert!(prime(p) && prime(s));
        for modulus in [p, s, u64::MAX] {
            for a in [0, 1, modulus / 2, modulus - 2, modulus - 1] {
                for b in [0, 1, modulus / 2, modulus - 2, modulus - 1] {
                    assert_eq!(
                        addmod(a, b, modulus),
                        ((u128::from(a) + u128::from(b)) % u128::from(modulus)) as u64
                    );
                }
            }
        }
        for n in [u128::from(u64::MAX), 4 * u128::from(p), (1u128 << 65) - 1] {
            let x = u128::from(floor_sqrt(n));
            assert!(x * x <= n && (x + 1) * (x + 1) > n);
        }
        // Arithmetic fixture only: no group order is asserted by this struct.
        let c = Curve {
            p,
            a: p - 3,
            b: 7,
            r: 3,
            roots: vec![],
            sqrt: Some(SqrtContext::new(p)),
            inverses: small_inverses(p),
            field: Modulus::new(p),
        };
        for x in [0, 1, p / 2, p - 2, p - 1] {
            let expected = ((u128::from(mulmod(mulmod(x, x, p), x, p))
                + u128::from(mulmod(c.a, x, p))
                + u128::from(c.b))
                % u128::from(p)) as u64;
            assert_eq!(c.rhs(x), expected);
            if let Some(q) = c.decode(2 * x) {
                assert_eq!(c.decode(c.encode(Some(q)).unwrap()), Some(q));
                assert!(c.contains(c.add(Some(q), Some(q)).unwrap()));
            }
        }
        // Use a smaller 63-bit field so [2p,4p] contains u64 primes.
        let hp = 6_917_529_027_641_081_737u64;
        assert!(prime(hp));
        let hc = Curve {
            p: hp,
            a: 0,
            b: 7,
            r: 3,
            roots: vec![],
            sqrt: Some(SqrtContext::new(hp)),
            inverses: small_inverses(hp),
            field: Modulus::new(hp),
        };
        for alpha in [1, 17, s - 1] {
            for beta in [0, s / 2, s - 1] {
                let h = HashBase {
                    s,
                    alpha,
                    beta,
                    tau: 257,
                };
                for point in h.enumerate(&hc).unwrap() {
                    let z = hc.encode(point).unwrap();
                    let expected = ((u128::from(mulmod(alpha, z, s)) + u128::from(beta))
                        % u128::from(s)) as u64;
                    assert!(expected < h.tau && hc.contains(point));
                }
            }
        }
    }

    #[test]
    fn deferred_dependency_matches_eager_reference() {
        // Independent eager provenance elimination retained only as a test
        // oracle. Compare exact coefficients, not merely a zero product.
        fn eager(rows: &[Vec<u64>], r: u64) -> Option<Vec<u64>> {
            let mut pivots = BTreeMap::<usize, (Vec<u64>, Vec<u64>)>::new();
            for (i, source) in rows.iter().enumerate() {
                let mut row = source.clone();
                let mut coeff = vec![0; rows.len()];
                coeff[i] = 1;
                for (&col, (pivot, pc)) in &pivots {
                    let k = row[col];
                    for (a, &b) in row.iter_mut().zip(pivot) {
                        *a = sub(*a, mulmod(k, b, r), r);
                    }
                    for (a, &b) in coeff.iter_mut().zip(pc) {
                        *a = sub(*a, mulmod(k, b, r), r);
                    }
                }
                if let Some(col) = row.iter().position(|&x| x != 0) {
                    let inv = pow(row[col], r - 2, r);
                    for x in &mut row {
                        *x = mulmod(*x, inv, r);
                    }
                    for x in &mut coeff {
                        *x = mulmod(*x, inv, r);
                    }
                    pivots.insert(col, (row, coeff));
                } else {
                    return Some(coeff);
                }
            }
            None
        }
        let mut rng = Rng(321);
        for r in [3, 199, 1_099_512_551_701] {
            for n in 1..25 {
                for density in [1, 2, 8, 32] {
                    let rows: Vec<Vec<u64>> = (0..n + 1)
                        .map(|_| {
                            (0..n)
                                .map(|_| {
                                    if rng.below(density) == 0 {
                                        rng.below(r)
                                    } else {
                                        0
                                    }
                                })
                                .collect()
                        })
                        .collect();
                    assert_eq!(dependency(&rows, r), eager(&rows, r));
                }
            }
        }
    }

    #[test]
    fn rejects_corrupt_accelerated_base_before_relation_queries() {
        struct BadBase;
        impl DecompositionOracle for BadBase {
            fn enumerate_bases(
                &mut self,
                _: &Curve,
                hashes: &[HashBase],
            ) -> Result<Vec<Vec<Point>>, Error> {
                Ok(vec![vec![None]; hashes.len()])
            }
            fn find(
                &mut self,
                _: &Curve,
                _: &[Vec<Point>],
                _: Point,
                _: u64,
            ) -> Result<Option<Vec<Point>>, Error> {
                panic!("invalid base reached relation oracle")
            }
        }
        let (c, p) = fixture();
        let q = c.mul(p, 42).unwrap();
        assert_eq!(
            solve_with_oracle(&c, p, q, &Config::default(), &mut BadBase),
            Err(Error::Verification)
        );
    }

    #[test]
    fn validates_curve_and_canonical_points() {
        let (c, p) = fixture();
        assert!(Curve::new(211, 0, 7, 197).is_err());
        assert!(Curve::new(15, 2, 2, 19).is_err());
        assert!(Curve::new(17, 0, 0, 19).is_err());
        assert!(Curve::new(u64::MAX, 0, 7, 199).is_err());
        assert!(c.encode(None).is_err());
        assert!(c.add(Some((214, 33)), p).is_err());
        assert_eq!(c.mul(p, 199).unwrap(), None);
        assert_eq!(c.add(p, c.neg(p)).unwrap(), None);
        assert_eq!(c.add(None, p).unwrap(), p);
        assert_eq!(c.add(p, p).unwrap(), c.mul(p, 2).unwrap());
    }

    #[test]
    fn inverse_hash_progression_matches_direct_products() {
        fn direct(h: &HashBase, c: &Curve) -> Vec<Point> {
            let inv = pow(h.alpha, h.s - 2, h.s);
            let mut out = Vec::new();
            for t in 0..h.tau {
                let z = (u128::from(inv) * u128::from((t + h.s - h.beta) % h.s) % u128::from(h.s))
                    as u64;
                if let Some(point) = c.decode(z) {
                    out.push(Some(point));
                }
            }
            out.sort_unstable();
            out
        }
        let c = Curve::new(17, 2, 2, 19).unwrap();
        for s in [37, 41, 43] {
            for alpha in 1..s {
                for beta in 0..s {
                    for tau in [1, s / 2, s] {
                        let h = HashBase {
                            s,
                            alpha,
                            beta,
                            tau,
                        };
                        assert_eq!(h.enumerate(&c).unwrap(), direct(&h, &c));
                    }
                }
            }
        }
        let p = 1_099_511_627_563;
        let c =
            Curve::with_order_witness(p, p - 3, 56, 1_099_512_551_701, Some((1, 15_267_301_451)))
                .unwrap();
        let s = (2 * p..4 * p).find(|&s| prime(s)).unwrap();
        for alpha in [1, 7, s / 2, s - 1] {
            for beta in [0, 1, s / 2, s - 1] {
                let h = HashBase {
                    s,
                    alpha,
                    beta,
                    tau: 64,
                };
                assert_eq!(h.enumerate(&c).unwrap(), direct(&h, &c));
            }
        }
    }

    #[test]
    fn grouped_hash_validation_matches_standalone_enumeration() {
        let (c, _) = fixture();
        let first = HashBase {
            s: 431,
            alpha: 7,
            beta: 19,
            tau: 44,
        };
        let second = HashBase {
            s: 433,
            alpha: 11,
            beta: 29,
            tau: 64,
        };
        for hashes in [
            vec![],
            vec![first],
            vec![first, first, second, second, first],
        ] {
            let expected: Result<Vec<_>, _> = hashes.iter().map(|h| h.enumerate(&c)).collect();
            assert_eq!(enumerate_hash_bases(&c, &hashes), expected);
        }
        for bad in [
            HashBase { s: 432, ..first },
            HashBase {
                s: 1u64 << 63,
                ..first
            },
            HashBase { s: 421, ..first },
            HashBase { s: 853, ..first },
            HashBase { alpha: 0, ..first },
            HashBase {
                alpha: 431,
                ..first
            },
            HashBase { beta: 431, ..first },
            HashBase { tau: 0, ..first },
            HashBase { tau: 432, ..first },
        ] {
            assert!(bad.enumerate(&c).is_err());
            for hashes in [vec![bad, first], vec![first, bad], vec![first, first, bad]] {
                let expected: Result<Vec<_>, _> = hashes.iter().map(|h| h.enumerate(&c)).collect();
                assert_eq!(enumerate_hash_bases(&c, &hashes), expected);
            }
        }
    }

    #[test]
    fn inverse_hash_enumeration_matches_complete_group_scan() {
        let (c, p) = fixture();
        for (alpha, beta, tau) in [(1, 0, 1), (7, 19, 44), (430, 430, 431)] {
            let h = HashBase {
                s: 431,
                alpha,
                beta,
                tau,
            };
            let mut expected = Vec::new();
            for k in 1..c.r {
                let point = c.mul(p, k).unwrap();
                let z = c.encode(point).unwrap();
                assert_eq!(c.decode(z), point);
                if (alpha * z + beta) % 431 < tau {
                    expected.push(point);
                }
            }
            expected.sort_unstable();
            assert_eq!(h.enumerate(&c).unwrap(), expected);
        }
        assert_eq!(c.decode(2 * c.p), None);
        assert!(HashBase {
            s: 431,
            alpha: 0,
            beta: 0,
            tau: 20
        }
        .enumerate(&c)
        .is_err());
    }

    #[test]
    fn oracle_matches_cartesian_sum_for_every_target() {
        let (c, p) = fixture();
        let bases = vec![
            vec![c.mul(p, 1).unwrap(), c.mul(p, 2).unwrap()],
            vec![c.mul(p, 3).unwrap(), c.mul(p, 4).unwrap()],
        ];
        let mut bases = bases;
        for b in &mut bases {
            b.sort_unstable();
        }
        for k in 0..c.r {
            let target = c.mul(p, k).unwrap();
            let expected = bases[0]
                .iter()
                .any(|&a| bases[1].iter().any(|&b| c.add(a, b).unwrap() == target));
            let found = Enumerate.find(&c, &bases, target, 2).unwrap();
            assert_eq!(found.is_some(), expected);
        }
        assert!(matches!(
            Enumerate.find(&c, &bases, None, 1),
            Err(Error::Limit(_))
        ));
    }

    #[test]
    fn overlapping_slots_keep_separate_columns() {
        let (c, p) = fixture();
        let rows = matrix(
            &[vec![p], vec![p]],
            &[Relation {
                u: 2,
                v: 0,
                points: vec![p, p],
            }],
        )
        .unwrap();
        assert_eq!(rows, vec![vec![1, 1]]);
        let rows = vec![vec![1, 0], vec![0, 1], vec![1, 1]];
        assert_eq!(dependency(&rows, c.r).unwrap(), vec![198, 198, 1]);
    }

    #[test]
    fn solves_and_replays_seeded_known_answers() {
        let (c, p) = fixture();
        for (seed, scalar, m) in [(1, 1, 2), (2, 42, 2), (3, 198, 2), (5, 87, 3)] {
            let q = c.mul(p, scalar).unwrap();
            let config = Config {
                seed,
                m,
                ..Config::default()
            };
            let cert = solve(&c, p, q, &config).unwrap();
            assert_eq!(cert.scalar, scalar);
            verify(&c, p, q, &cert).unwrap();
            assert_eq!(cert, solve(&c, p, q, &config).unwrap());
            let mut bad = cert.clone();
            bad.relations[0].u = (bad.relations[0].u + 1) % c.r;
            assert!(verify(&c, p, q, &bad).is_err());
            let mut bad = cert.clone();
            bad.dependency.fill(0);
            assert!(verify(&c, p, q, &bad).is_err());
            let mut bad = cert.clone();
            bad.scalar = (bad.scalar + 1) % c.r;
            assert!(verify(&c, p, q, &bad).is_err());
            let mut bad = cert;
            bad.hashes[0].alpha = 0;
            assert!(verify(&c, p, q, &bad).is_err());
        }
    }

    #[test]
    fn identity_and_bounds_are_explicit() {
        let (c, p) = fixture();
        let cert = solve(&c, p, None, &Config::default()).unwrap();
        verify(&c, p, None, &cert).unwrap();
        for config in [
            Config {
                m: 1,
                ..Config::default()
            },
            Config {
                tau: Some(0),
                ..Config::default()
            },
            Config {
                max_trials: 0,
                ..Config::default()
            },
        ] {
            assert!(solve(&c, p, p, &config).is_err());
        }
        assert!(matches!(
            solve(
                &c,
                p,
                p,
                &Config {
                    max_columns: 1,
                    ..Config::default()
                }
            ),
            Err(Error::Limit(_))
        ));
        assert_eq!(
            extract(
                &[Relation {
                    u: 0,
                    v: 0,
                    points: vec![]
                }],
                &[1],
                c.r
            ),
            None
        );
    }

    #[test]
    fn small_curve_all_logarithms_and_uninformative_dependency() {
        let curve = Curve::new(17, 2, 2, 19).unwrap();
        let p = Some((5, 1));
        let config = Config {
            tau: Some(20),
            ..Config::default()
        };
        for scalar in 0..19 {
            let q = curve.mul(p, scalar).unwrap();
            let cert = solve(&curve, p, q, &config).unwrap();
            assert_eq!(cert.scalar, scalar);
            verify(&curve, p, q, &cert).unwrap();
        }
        assert!(Curve::new(17, 2, 2, 18446744073709551557).is_err());
        // A genuine but uninformative dependency has V = 0 and must restart.
        assert_eq!(
            extract(
                &[
                    Relation {
                        u: 7,
                        v: 3,
                        points: vec![]
                    },
                    Relation {
                        u: 7,
                        v: 3,
                        points: vec![]
                    },
                ],
                &[1, 18],
                19
            ),
            None
        );
    }

    #[test]
    fn exhausted_trials_and_empty_bases_are_not_successes() {
        struct Never;
        impl DecompositionOracle for Never {
            fn find(
                &mut self,
                _: &Curve,
                _: &[Vec<Point>],
                _: Point,
                _: u64,
            ) -> Result<Option<Vec<Point>>, Error> {
                Ok(None)
            }
        }
        let (c, p) = fixture();
        let config = Config {
            max_trials: 1,
            ..Config::default()
        };
        assert!(matches!(
            solve_with_oracle(&c, p, p, &config, &mut Never),
            Err(Error::Limit(_))
        ));
        assert_eq!(
            Enumerate.find(&c, &[vec![], vec![p]], None, 0).unwrap(),
            None
        );
    }

    #[test]
    fn rejects_corrupt_oracle_instead_of_accepting_a_relation() {
        struct Bad;
        impl DecompositionOracle for Bad {
            fn find(
                &mut self,
                _: &Curve,
                _: &[Vec<Point>],
                _: Point,
                _: u64,
            ) -> Result<Option<Vec<Point>>, Error> {
                Ok(Some(vec![None, None]))
            }
        }
        let (c, p) = fixture();
        assert_eq!(
            solve_with_oracle(&c, p, p, &Config::default(), &mut Bad),
            Err(Error::Verification)
        );
    }
}
