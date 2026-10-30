//! Executable reference for §3 of Bochum's *Subexponential ECDL*, v0.2.
//!
//! This implements affine-hashed factor bases and the index-calculus reduction,
//! with a separate complete tensor implementation exported as [`tensor`].
//! This bounded reference's default oracle enumerates group decompositions (generic
//! decomposition, bounded by sqrt(r); cannot beat rho). No subexponential or
//! performance claim is made. Cofactor-one prime-field curves below 2^60 are
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
fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    while n != 0 {
        if n & 1 != 0 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        n >>= 1;
    }
    r
}
fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

fn floor_sqrt(n: u64) -> u64 {
    let (mut lo, mut hi) = (0u64, 1u64 << 32);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if u128::from(mid) * u128::from(mid) <= u128::from(n) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

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

/// Explicit CPU reduction memory ceiling; callers must opt in above 512.
/// Two dense triangular workspaces can require several GiB at this ceiling.
pub const MAX_COLUMNS: usize = 16384;
/// Bound inverse-hash enumeration, including independent certificate replay.
pub const MAX_HASH_CANDIDATES: u64 = 1 << 24;

/// Validated short-Weierstrass curve of odd prime cardinality, cofactor one.
/// The tiny constructor counts points; the scalable constructor proves order
/// from a nonzero torsion witness and the Hasse interval.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Curve {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    // Tiny constructor's lookup; empty for the order-witness constructor.
    roots: Vec<Option<u64>>,
}

impl Curve {
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
        if !(5..(1u64 << 60)).contains(&p)
            || !(3..(1u64 << 61)).contains(&r)
            || !prime(p)
            || !prime(r)
            || a >= p
            || b >= p
        {
            return Err(Error::Invalid("prime field/order witness parameters"));
        }
        if (mulmod(4, mulmod(mulmod(a, a, p), a, p), p) + mulmod(27 % p, mulmod(b, b, p), p)) % p
            == 0
        {
            return Err(Error::Invalid("singular curve"));
        }
        let curve = Self {
            p,
            a,
            b,
            r,
            roots: Vec::new(),
        };
        if generator.is_none() || !curve.contains(generator) || curve.mul(generator, r)?.is_some() {
            return Err(Error::Invalid("invalid prime-order witness"));
        }
        let radius = floor_sqrt(4 * p);
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
        let mut roots = vec![None; p as usize];
        for y in 0..p {
            roots[(y * y % p) as usize].get_or_insert(y);
        }
        let curve = Self { p, a, b, r, roots };
        let count = 1
            + (0..p)
                .map(|x| match curve.roots[curve.rhs(x) as usize] {
                    None => 0,
                    Some(0) => 1,
                    Some(_) => 2,
                })
                .sum::<u64>();
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
        (mulmod(mulmod(x, x, self.p), x, self.p) + mulmod(self.a, x, self.p) + self.b) % self.p
    }

    /// Test canonical coordinates and the curve equation; accepts the identity.
    pub fn contains(&self, point: Point) -> bool {
        match point {
            None => true,
            Some((x, y)) => x < self.p && y < self.p && mulmod(y, y, self.p) == self.rhs(x),
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
            sqrtmod(self.rhs(x), self.p)?
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
        if x == xx && (y + yy) % p == 0 {
            return None;
        }
        let (num, den) = if x == xx {
            ((3 * mulmod(x, x, p) + self.a) % p, 2 * y % p)
        } else {
            (sub(yy, y, p), sub(xx, x, p))
        };
        let slope = mulmod(num, pow(den, p - 2, p), p);
        let nx = sub(sub(mulmod(slope, slope, p), x, p), xx, p);
        Some((nx, sub(mulmod(slope, sub(x, nx, p), p), y, p)))
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
    /// Prime hash modulus, in `[2p,4p]`.
    pub s: u64,
    /// Nonzero multiplier modulo `s`.
    pub alpha: u64,
    /// Additive offset modulo `s`.
    pub beta: u64,
    /// Membership is `(alpha*enc(A)+beta) mod s < tau`.
    pub tau: u64,
}

impl HashBase {
    fn validate(&self, curve: &Curve) -> Result<(), Error> {
        if self.s < 2 * curve.p
            || self.s > 4 * curve.p
            || !prime(self.s)
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

    /// Enumerate by inverse hashing, not by scanning all curve points (§3.5).
    pub fn enumerate(&self, curve: &Curve) -> Result<Vec<Point>, Error> {
        self.validate(curve)?;
        if self.tau > MAX_HASH_CANDIDATES {
            return Err(Error::Limit("inverse-hash candidate bound"));
        }
        let inv = pow(self.alpha, self.s - 2, self.s);
        let mut points = Vec::new();
        for t in 0..self.tau {
            let z = mulmod(inv, sub(t, self.beta, self.s), self.s);
            if let Some(point) = curve.decode(z) {
                points.push(Some(point));
            }
        }
        points.sort_unstable();
        Ok(points)
    }
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
        hashes.iter().map(|h| h.enumerate(curve)).collect()
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
            || bases.len() > 8
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
    /// Number of ordered factor-base slots, 2 through 8.
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

// First row dependency, independent of u/v. Keep elimination factors instead
// of updating a full provenance vector at every pivot; reconstruct once by
// triangular back-substitution when the first dependent row is reached.
fn dependency(rows: &[Vec<u64>], r: u64) -> Option<Vec<u64>> {
    struct Pivot {
        row: Vec<u64>,
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
            for (a, &b) in row[col..].iter_mut().zip(&pivots[index].row[col..]) {
                *a = sub(*a, mulmod(k, b, r), r);
            }
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
                    let k = mulmod(weights[j], pivot.inverse, r);
                    combination[pivot.source] = k;
                    if k != 0 {
                        for &(ancestor, factor) in &pivot.factors {
                            weights[ancestor] = sub(weights[ancestor], mulmod(k, factor, r), r);
                        }
                    }
                }
                return Some(combination);
            }
            Some(col) => {
                let inv = pow(row[col], r - 2, r);
                for a in &mut row[col..] {
                    *a = mulmod(*a, inv, r);
                }
                columns.insert(col, pivots.len());
                pivots.push(Pivot {
                    row,
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
    (v != 0).then(|| mulmod(sub(0, u, r), pow(v, r - 2, r), r))
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
    solve_with_oracle_profile(curve, p, q, config, oracle, &mut ReductionStats::default())
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
    use std::time::Instant;
    if p.is_none() || !curve.contains(p) || !curve.contains(q) {
        return Err(Error::Invalid("generator or target"));
    }
    if !(2..=8).contains(&config.m)
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
    let s = (2 * curve.p..=4 * curve.p).find(|&s| prime(s)).unwrap();
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
    for trial in 1..=config.max_trials {
        let stage = Instant::now();
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
                        Ok(z) => (mulmod(h.alpha, z, h.s) + h.beta) % h.s >= h.tau,
                        Err(_) => true,
                    })
            })
        {
            return Err(Error::Verification);
        }
        let n: usize = bases.iter().map(Vec::len).sum();
        stats.bases += stage.elapsed();
        if n > config.max_columns {
            return Err(Error::Limit("dense relation matrix columns"));
        }
        if bases.iter().any(Vec::is_empty) {
            continue;
        }
        let mut relations = Vec::new();
        for _ in 0..20 * (n + 1) {
            attempts += 1;
            let stage = Instant::now();
            let (u, v) = (rng.below(curve.r), rng.below(curve.r));
            let target = curve.add_valid(curve.mul(p, u)?, curve.mul(q, v)?);
            stats.targets += stage.elapsed();
            let stage = Instant::now();
            let found = oracle.find(curve, &bases, target, config.max_work);
            stats.oracle += stage.elapsed();
            if let Some(points) = found? {
                let stage = Instant::now();
                let rel = Relation { u, v, points };
                check_relation(curve, p, q, &bases, &rel)?;
                relations.push(rel);
                stats.targets += stage.elapsed();
                if relations.len() == n + 1 {
                    break;
                }
            }
        }
        if relations.len() != n + 1 {
            continue;
        }
        let stage = Instant::now();
        let coeff = dependency(&matrix(&bases, &relations)?, curve.r).ok_or(Error::Verification)?;
        let scalar = extract(&relations, &coeff, curve.r);
        stats.linear += stage.elapsed();
        if let Some(scalar) = scalar {
            let cert = Certificate {
                scalar,
                hashes,
                relations,
                dependency: coeff,
                trials: trial,
                attempts,
            };
            let stage = Instant::now();
            verify(curve, p, q, &cert)?;
            stats.replay += stage.elapsed();
            return Ok(cert);
        }
    }
    Err(Error::Limit(
        "factor-base trials exhausted; logarithm unresolved",
    ))
}

fn check_relation(
    curve: &Curve,
    p: Point,
    q: Point,
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
    if sum != curve.add_valid(curve.mul(p, rel.u)?, curve.mul(q, rel.v)?) {
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
    if !(2..=8).contains(&cert.hashes.len()) {
        return Err(Error::Verification);
    }
    let bases: Vec<_> = cert
        .hashes
        .iter()
        .map(|h| h.enumerate(curve))
        .collect::<Result<_, _>>()?;
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
    for rel in &cert.relations {
        check_relation(curve, p, q, &bases, rel)?;
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
            let x = u128::from(floor_sqrt(n));
            assert!(x * x <= u128::from(n) && (x + 1) * (x + 1) > u128::from(n));
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
                let rows: Vec<Vec<u64>> = (0..n + 1)
                    .map(|_| (0..n).map(|_| rng.below(r)).collect())
                    .collect();
                assert_eq!(dependency(&rows, r), eager(&rows, r));
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
