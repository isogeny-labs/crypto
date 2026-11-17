//! The **four-point signed-Frobenius pair claw** of
//! `aburan28/cryptanalysis#175` (`experiments/koblitz-pair-claw-20260929`,
//! the `PDP4qtable … LAnone` candidate), ported onto the ecbench ledger.
//!
//! The method, as that experiment runs it on `EC1N83Ckb1h876c2921cb64`:
//!
//! 1. **Known-log base.**  `S` seed scalars `a_i`, seed points `[a_i]G`,
//!    and every signed Frobenius conjugate `σφ^t[a_i]G`, whose logarithm
//!    is `σλ^t a_i` because `φ = [λ]` on the subgroup.  `B = 2nS` points,
//!    `S` folded columns.
//! 2. **Table** (target-independent).  Zero-pair quotient descriptors
//!    `P_i + σφ^t P_j` (`i ≤ j`, first point the unrotated seed), visited
//!    in a fixed permutation without repeats, keyed by the least
//!    normal-basis rotation of the sum's abscissa — the `{±φ^u}` class.
//! 3. **Queries** (online).  Unordered pairs `{F_k, F_l}` of base points,
//!    visited in a fixed permutation without repeats; each costs
//!    `F_k + F_l` and `Q − (F_k + F_l)`, then one canonicalisation and one
//!    probe.  A hit is a natural four-point relation
//!    `Q = F_k + F_l ± φ^u(P_i + σφ^t P_j)`, and every term's logarithm is
//!    known, so the scalar follows with no linear algebra.
//!
//! Because every base logarithm is known, this is a **generic** algorithm:
//! a randomised baby-step giant-step on signed Frobenius classes.  With a
//! table of `M` classes each query lands in it with probability
//! `2nM / r`, so with `M = c·√(r/n)` the expected cost is
//! `M + 2·r/(2nM) = (c + 1/c)·√(r/n)` additions, `S ≈ (c + 1/c)/√n`,
//! minimised at `c = 1` (`2/√n`), against the signed-Frobenius rho
//! constant `√(π/4n)`: about `2.26×` rho at best.  `expected_s` stays
//! `None` because the constant depends on `c`; the protocol states it.
//!
//! Charged: the seeds' scalar multiplications, every table and query
//! addition, and the addition that rebuilds the hit's table sum.
//! Counted, not charged: Frobenius maps, canonicalisations, table inserts
//! and probes (the memory the table holds is `M` entries, as BSGS's is).
//! The planted scalar is never seen; the candidate is returned unverified.

use std::collections::{BTreeMap, HashMap};

use num_traits::ToPrimitive;

use crate::cryptanalysis::ecbench::canonical::{derive_biguint, derive_u128, derive_u64};
use crate::cryptanalysis::ecbench::exact_scalar::ExactScalar;
use crate::cryptanalysis::ecbench::workload::{BigInstance, WideInstance};
use crate::cryptanalysis::ic_boundary::{BinaryGroup, BinaryInstance, CountedGroup, GroupOps};
use crate::cryptanalysis::ic_measurement::{self as measurement, Phase};
use crate::cryptanalysis::koblitz_big::{BigStrongRho, W192};
use crate::cryptanalysis::koblitz_fast::{FastPoint, FrobeniusCanon, FrobeniusPowers};
use crate::cryptanalysis::koblitz_strong_rho::{RawPointG, RhoScalar, WideStrongRho};

/// The two shape parameters, as multiples of their natural scales.
#[derive(Clone, Copy, Debug)]
pub struct ClawShape {
    /// Seed orbits `S = ⌈base_scale · r^{1/4} n^{−3/4}⌉`.  The PR's n=83
    /// base (48,194 seeds) is `base_scale ≈ 1.06`.
    pub base_scale: f64,
    /// Table classes `M = ⌈table_scale · √(r/n)⌉`.  `1` minimises the
    /// expected additions; the PR's memory-bound `2^31`–`2^33` tables at
    /// n=83 are `table_scale ≈ 1/32`.
    pub table_scale: f64,
    /// Total charged additions allowed before the run is exhausted.
    pub max_adds: u64,
}

impl ClawShape {
    /// `r` is the subgroup order as the nearest `f64` (for a `u128` order,
    /// `r as f64`, what these rules always read).
    pub fn seeds(&self, r: f64, n: u32) -> u64 {
        let s = self.base_scale * r.powf(0.25) * (n as f64).powf(-0.75);
        (s.ceil() as u64).max(2)
    }
    pub fn table(&self, r: f64, n: u32) -> u64 {
        let m = self.table_scale * (r / n as f64).sqrt();
        (m.ceil() as u64).max(1)
    }
}

/// What one claw solve did, phase by phase.
#[derive(Clone, Debug, Default)]
pub struct ClawOutcome {
    pub recovered: Option<ExactScalar>,
    pub base: GroupOps,
    pub table: GroupOps,
    pub search: GroupOps,
    pub recover: GroupOps,
    /// The final `[d]G = Q` check, in the `_v2` accounting only.
    pub verification: GroupOps,
    pub counters: BTreeMap<String, u64>,
    pub exhausted: bool,
}

impl ClawOutcome {
    fn count(&mut self, name: &str, by: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += by;
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// A fixed permutation of `[0, d)`: `i ↦ (a·i + b) mod d` with
/// `gcd(a, d) = 1`, so a prefix visits distinct indices — the PR's
/// "unique schedule" without storing it.
struct Schedule {
    d: u64,
    a: u64,
    b: u64,
}

impl Schedule {
    fn new(d: u64, seed: u64, domain: &str) -> Self {
        let mut a = derive_u64(domain, &[seed, 0]) % d.max(1);
        while d > 1 && (a == 0 || gcd(a, d) != 1) {
            a = (a + 1) % d;
        }
        let b = derive_u64(domain, &[seed, 1]) % d.max(1);
        Self { d, a: a.max(1), b }
    }
    fn at(&self, i: u64) -> u64 {
        ((self.a as u128 * i as u128 + self.b as u128) % self.d as u128) as u64
    }
}

/// What the claw needs of a Koblitz curve: counted group operations, the
/// signed-Frobenius class of a point, and the Frobenius map.  The one-word
/// curve ([`NarrowClaw`]), the wide one ([`WideClaw`]) and the three-word
/// one ([`BigClaw`]) implement it; the claw is written once, so the wide and
/// big claws are the one-word claw.
pub trait ClawGroup {
    type P: Copy + PartialEq;
    type S: RhoScalar;
    /// The table key: a class name.  `u128` for the one-word and wide
    /// curves (as the committed claw keyed its table), a three-word word
    /// for the big curves, whose class names do not fit a `u128`.
    type K: Copy + Eq + std::hash::Hash;
    fn n(&self) -> u32;
    fn r(&self) -> Self::S;
    fn lambda(&self) -> Self::S;
    fn generator(&self) -> Self::P;
    /// One counted addition.
    fn add(&self, ops: &mut GroupOps, a: Self::P, b: Self::P) -> Self::P;
    /// One counted doubling.
    fn double(&self, ops: &mut GroupOps, a: Self::P) -> Self::P;
    fn neg(&self, a: Self::P) -> Self::P;
    fn is_identity(&self, a: &Self::P) -> bool;
    fn identity(&self) -> Self::P;
    /// The `{±φ^u}` class name of a finite point (never 0) and the shift
    /// `u` that carries its abscissa to the class's least rotation.
    fn class(&self, p: &Self::P) -> (Self::K, u32);
    fn frob(&self, t: u32, p: Self::P) -> Self::P;
    /// Seed scalar `i` in `[1, r − 1]` under `seed`.
    fn seed_scalar(&self, seed: u64, i: u64) -> Self::S;
    /// `[k]P` by double-and-add, counted exactly as
    /// [`CountedGroup::mul`] counts it.
    fn counted_mul(&self, ops: &mut GroupOps, p: Self::P, k: Self::S) -> Self::P {
        ops.scalar_mults += 1;
        if k == Self::S::zero() || self.is_identity(&p) {
            return self.identity();
        }
        let bits = k.bit_len();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.double(ops, acc);
            if k.bit(i) {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }
}

/// The one-word Koblitz curve, as the committed pair-claw session ran it.
pub struct NarrowClaw<'a> {
    inst: &'a BinaryInstance,
    g: BinaryGroup<'a>,
    canon: FrobeniusCanon,
    powers: FrobeniusPowers,
    lambda: u64,
}

impl<'a> NarrowClaw<'a> {
    pub fn new(inst: &'a BinaryInstance) -> Option<Self> {
        let kc = inst.koblitz.as_ref()?;
        if kc.k != 1 {
            return None;
        }
        Some(Self {
            inst,
            g: BinaryGroup(&inst.fast),
            canon: FrobeniusCanon::new(&inst.fast.field, inst.n)?,
            powers: FrobeniusPowers::new(&inst.fast.field, inst.n),
            lambda: (&kc.lambda % num_bigint::BigUint::from(inst.r)).to_u64()?,
        })
    }
}

impl ClawGroup for NarrowClaw<'_> {
    type P = FastPoint;
    type S = u64;
    type K = u128;
    fn n(&self) -> u32 {
        self.inst.n
    }
    fn r(&self) -> u64 {
        self.inst.r
    }
    fn lambda(&self) -> u64 {
        self.lambda
    }
    fn generator(&self) -> FastPoint {
        self.inst.generator
    }
    fn add(&self, ops: &mut GroupOps, a: FastPoint, b: FastPoint) -> FastPoint {
        self.g.add(ops, a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: FastPoint) -> FastPoint {
        self.g.double(ops, a)
    }
    fn neg(&self, a: FastPoint) -> FastPoint {
        self.g.neg(a)
    }
    fn is_identity(&self, a: &FastPoint) -> bool {
        a.infinity
    }
    fn identity(&self) -> FastPoint {
        FastPoint::INFINITY
    }
    fn class(&self, p: &FastPoint) -> (u128, u32) {
        let (key, t) = self.canon.canon_with_shift(p.x);
        // Offset by one so that no finite point shares a key with O.
        (u128::from(key) + 1, t)
    }
    fn frob(&self, t: u32, p: FastPoint) -> FastPoint {
        if t == 0 || p.infinity {
            p
        } else {
            FastPoint::affine(self.powers.apply(t, p.x), self.powers.apply(t, p.y))
        }
    }
    fn seed_scalar(&self, seed: u64, i: u64) -> u64 {
        1 + derive_u64("ecbench.claw.seed", &[seed, i]) % (self.inst.r - 1)
    }
}

/// A Koblitz curve on the strong reference's `u128` arithmetic and normal
/// basis: a wide curve (`64 ≤ n ≤ 127`) in ecbench, or any curve in a test.
/// Seed scalars come from a 128-bit draw over decimal-string parts
/// (`ecbench.claw.seed.wide`); the one-word rule is kept for one-word curves.
pub struct WideClaw<'a> {
    rho: &'a WideStrongRho,
    r: u128,
    lambda: u128,
}

impl<'a> WideClaw<'a> {
    pub fn new(wi: &'a WideInstance) -> Self {
        Self::from_parts(&wi.rho, wi.kc.r, wi.kc.lambda)
    }

    pub fn from_parts(rho: &'a WideStrongRho, r: u128, lambda: u128) -> Self {
        Self { rho, r, lambda }
    }
}

impl ClawGroup for WideClaw<'_> {
    type P = RawPointG<u128>;
    type S = u128;
    type K = u128;
    fn n(&self) -> u32 {
        self.rho.degree()
    }
    fn r(&self) -> u128 {
        self.r
    }
    fn lambda(&self) -> u128 {
        self.lambda
    }
    fn generator(&self) -> Self::P {
        self.rho.generator()
    }
    fn add(&self, ops: &mut GroupOps, a: Self::P, b: Self::P) -> Self::P {
        ops.adds += 1;
        self.rho.add(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: Self::P) -> Self::P {
        ops.doubles += 1;
        self.rho.add(a, a)
    }
    fn neg(&self, a: Self::P) -> Self::P {
        match a {
            RawPointG::Affine { x, y } => RawPointG::Affine { x, y: y ^ x },
            inf => inf,
        }
    }
    fn is_identity(&self, a: &Self::P) -> bool {
        *a == RawPointG::Infinity
    }
    fn identity(&self) -> Self::P {
        RawPointG::Infinity
    }
    fn class(&self, p: &Self::P) -> (u128, u32) {
        let RawPointG::Affine { x, .. } = *p else {
            return (0, 0);
        };
        let (key, t) = self.rho.x_orbit(x);
        // A least rotation is below 2^127, so the offset cannot wrap.
        (key + 1, t)
    }
    fn frob(&self, t: u32, p: Self::P) -> Self::P {
        self.rho.frobenius(t, p)
    }
    fn seed_scalar(&self, seed: u64, i: u64) -> u128 {
        1 + derive_u128("ecbench.claw.seed.wide", &[seed.to_string(), i.to_string()]) % (self.r - 1)
    }
}

/// A Koblitz curve on the three-word arithmetic and normal basis of
/// [`BigStrongRho`]: a big curve (`128 ≤ n ≤ 191`, subgroup order past
/// `2^128`) in ecbench.  Seed scalars come from the whole 256-bit digest of
/// decimal-string parts (`ecbench.claw.seed.big`), reduced mod `r − 1`; the
/// class name is the least normal-basis rotation plus one, as a three-word
/// word.
pub struct BigClaw<'a> {
    rho: &'a BigStrongRho,
    r: crate::cryptanalysis::koblitz_big::U192,
    lambda: crate::cryptanalysis::koblitz_big::U192,
}

impl<'a> BigClaw<'a> {
    pub fn new(bi: &'a BigInstance) -> Self {
        Self::from_parts(&bi.rho, bi.kc.r192(), bi.kc.lambda192())
    }

    pub fn from_parts(
        rho: &'a BigStrongRho,
        r: crate::cryptanalysis::koblitz_big::U192,
        lambda: crate::cryptanalysis::koblitz_big::U192,
    ) -> Self {
        Self { rho, r, lambda }
    }
}

impl ClawGroup for BigClaw<'_> {
    type P = RawPointG<W192>;
    type S = crate::cryptanalysis::koblitz_big::U192;
    type K = W192;
    fn n(&self) -> u32 {
        self.rho.degree()
    }
    fn r(&self) -> Self::S {
        self.r
    }
    fn lambda(&self) -> Self::S {
        self.lambda
    }
    fn generator(&self) -> Self::P {
        self.rho.generator()
    }
    fn add(&self, ops: &mut GroupOps, a: Self::P, b: Self::P) -> Self::P {
        ops.adds += 1;
        self.rho.add(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: Self::P) -> Self::P {
        ops.doubles += 1;
        self.rho.add(a, a)
    }
    fn neg(&self, a: Self::P) -> Self::P {
        match a {
            RawPointG::Affine { x, y } => RawPointG::Affine { x, y: y ^ x },
            inf => inf,
        }
    }
    fn is_identity(&self, a: &Self::P) -> bool {
        *a == RawPointG::Infinity
    }
    fn identity(&self) -> Self::P {
        RawPointG::Infinity
    }
    fn class(&self, p: &Self::P) -> (W192, u32) {
        let RawPointG::Affine { x, .. } = *p else {
            return (W192::ZERO, 0);
        };
        let (key, t) = self.rho.x_orbit(x);
        // A least rotation is below 2^191, so the offset cannot wrap.
        (key.plus_one(), t)
    }
    fn frob(&self, t: u32, p: Self::P) -> Self::P {
        self.rho.frobenius(t, p)
    }
    fn seed_scalar(&self, seed: u64, i: u64) -> Self::S {
        let r = self.r.to_big();
        let k = derive_biguint("ecbench.claw.seed.big", &[seed.to_string(), i.to_string()])
            % (&r - 1u8)
            + 1u8;
        <Self::S as RhoScalar>::from_biguint(&k).expect("k < r")
    }
}

/// Run the claw on a one-word Koblitz instance.  `None` for any other
/// curve.
pub fn pair_claw(
    inst: &BinaryInstance,
    target: FastPoint,
    seed: u64,
    shape: ClawShape,
) -> Option<ClawOutcome> {
    let g = NarrowClaw::new(inst)?;
    Some(pair_claw_on(&g, target, seed, shape))
}

/// Run the claw on a wide Koblitz instance.
pub fn pair_claw_wide(
    wi: &WideInstance,
    target: RawPointG<u128>,
    seed: u64,
    shape: ClawShape,
) -> ClawOutcome {
    pair_claw_on(&WideClaw::new(wi), target, seed, shape)
}

/// Run the claw on a big (three-word) Koblitz instance.
pub fn pair_claw_big(
    bi: &BigInstance,
    target: RawPointG<W192>,
    seed: u64,
    shape: ClawShape,
    v2: bool,
) -> ClawOutcome {
    pair_claw_with(&BigClaw::new(bi), target, seed, shape, v2)
}

/// [`pair_claw`] under the `_v2` accounting ([`pair_claw_with`]).
pub fn pair_claw_v2(
    inst: &BinaryInstance,
    target: FastPoint,
    seed: u64,
    shape: ClawShape,
) -> Option<ClawOutcome> {
    let g = NarrowClaw::new(inst)?;
    Some(pair_claw_with(&g, target, seed, shape, true))
}

/// [`pair_claw_wide`] under the `_v2` accounting ([`pair_claw_with`]).
pub fn pair_claw_wide_v2(
    wi: &WideInstance,
    target: RawPointG<u128>,
    seed: u64,
    shape: ClawShape,
) -> ClawOutcome {
    pair_claw_with(&WideClaw::new(wi), target, seed, shape, true)
}

/// The claw on any [`ClawGroup`].
pub fn pair_claw_on<G: ClawGroup>(g: &G, target: G::P, seed: u64, shape: ClawShape) -> ClawOutcome {
    pair_claw_with(g, target, seed, shape, false)
}

/// The claw on any [`ClawGroup`].  With `v2` (the `claw.pair_table_v2`
/// accounting) the charged-addition budget `shape.max_adds` counts the
/// recover phase beside the base, table and search (the first rule leaves it
/// out, so a run can spend past the cap by its recover additions), and the
/// candidate is checked `[d]G = Q` on `out.verification`, one counted scalar
/// multiplication.  The walk is the same either way.
pub fn pair_claw_with<G: ClawGroup>(
    g: &G,
    target: G::P,
    seed: u64,
    shape: ClawShape,
    v2: bool,
) -> ClawOutcome {
    type Sc<G> = <G as ClawGroup>::S;
    let (n, r) = (g.n(), g.r());
    let lambda = g.lambda();
    let mut lambda_pow = Vec::with_capacity(n as usize);
    let mut cur = Sc::<G>::one();
    for _ in 0..n {
        lambda_pow.push(cur);
        cur = Sc::<G>::mul_mod(cur, lambda, r);
    }
    let mulmod = |a: Sc<G>, b: Sc<G>| Sc::<G>::mul_mod(a, b, r);
    let addmod = |a: Sc<G>, b: Sc<G>| Sc::<G>::add_mod(a, b, r);
    let negmod = |a: Sc<G>| Sc::<G>::sub_mod(Sc::<G>::zero(), a, r);

    let mut out = ClawOutcome::default();
    let s = shape.seeds(r.to_f64(), n);
    let m = shape.table(r.to_f64(), n);
    out.count("seed_orbits", s);
    out.count("table_target", m);

    // 1. Known-log base: B = 2nS points, index b ↦ (i, t, σ).
    let two_n = 2 * n as u64;
    let b_count = two_n * s;
    out.count("base_points", b_count);
    let mut seeds: Vec<Sc<G>> = Vec::with_capacity(s as usize);
    let mut base: Vec<G::P> = Vec::with_capacity(b_count as usize);
    for i in 0..s {
        let a = g.seed_scalar(seed, i);
        seeds.push(a);
        let p = g.counted_mul(&mut out.base, g.generator(), a);
        for t in 0..n {
            let q = g.frob(t, p);
            base.push(q);
            base.push(g.neg(q));
        }
    }
    out.count("frobenius_maps_uncharged", s * (n as u64 - 1));
    let log_of = |b: u64| -> Sc<G> {
        let i = (b / two_n) as usize;
        let t = ((b % two_n) / 2) as usize;
        let l = mulmod(seeds[i], lambda_pow[t]);
        if b % 2 == 1 {
            negmod(l)
        } else {
            l
        }
    };
    let charged = |o: &ClawOutcome| {
        o.base.gae() + o.table.gae() + o.search.gae() + if v2 { o.recover.gae() } else { 0.0 }
    };
    // The `_v2` final check: one scalar multiplication of the candidate.
    let verify = |o: &mut ClawOutcome| {
        if let (true, Some(d)) = (v2, o.recovered) {
            let d = <G::S as RhoScalar>::from_biguint(&d.to_biguint())
                .expect("a candidate is below the subgroup order");
            o.count("verification_checks", 1);
            let mut ops = GroupOps::default();
            if g.counted_mul(&mut ops, g.generator(), d) != target {
                o.count("failed_candidates", 1);
            }
            o.verification.merge(ops);
        }
    };

    // 2. Table: descriptor d ∈ [0, S·B) ↦ (i = d / B, b = d mod B), the sum
    // P_i + F_b with i ≤ seed(b); keyed by class, first descriptor wins.
    let d_table = s * b_count;
    let sched = Schedule::new(d_table, seed, "ecbench.claw.table");
    // Reserve at most 2^24 slots up front: the table grows as it fills, and
    // a wide curve's target M (2^37 at n = 83) would otherwise be reserved
    // before the step budget can stop it.  Capacity changes no count.
    let mut table: HashMap<G::K, u64> = HashMap::with_capacity(m.min(1 << 24) as usize);
    let mut visited = 0u64;
    while (table.len() as u64) < m && visited < d_table {
        let d = sched.at(visited);
        visited += 1;
        let (i, b) = (d / b_count, d % b_count);
        if i > b / two_n {
            out.count("table_descriptors_skipped", 1);
            continue;
        }
        let sum = g.add(&mut out.table, base[(i * two_n) as usize], base[b as usize]);
        if g.is_identity(&sum) {
            out.count("table_identity_sums", 1);
            continue;
        }
        let (key, _) = g.class(&sum);
        out.count("canonicalisations_uncharged", 1);
        out.count("inserts_uncharged", 1);
        // The first descriptor of a class is kept, deterministically.
        match table.entry(key) {
            std::collections::hash_map::Entry::Occupied(_) => {
                out.count("table_duplicate_classes", 1);
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(d);
            }
        }
        if charged(&out) as u64 >= shape.max_adds {
            break;
        }
    }
    out.count("table_classes", table.len() as u64);
    out.count("table_descriptors_visited", visited);

    // 3. Queries: unordered pairs k < l from [0, B)², online.
    let d_query = b_count * b_count;
    let qsched = Schedule::new(d_query, seed, "ecbench.claw.query");
    let mut qi = 0u64;
    // The online window: from the first target-dependent addition to the
    // returned candidate.  The base and the table are reusable set-up.
    measurement::begin_online(Phase::RhoSolve);
    while qi < d_query {
        if charged(&out) as u64 >= shape.max_adds {
            out.exhausted = true;
            out.count("queries_visited", qi);
            measurement::end_online();
            return out;
        }
        let d = qsched.at(qi);
        qi += 1;
        let (k, l) = (d / b_count, d % b_count);
        if k >= l {
            out.count("query_pairs_skipped", 1);
            continue;
        }
        out.count("queries", 1);
        let pair = g.add(&mut out.search, base[k as usize], base[l as usize]);
        let y = g.add(&mut out.search, target, g.neg(pair));
        let pair_log = addmod(log_of(k), log_of(l));
        if g.is_identity(&y) {
            out.count("queries_visited", qi);
            out.count("direct_pair_hits", 1);
            out.recovered = ExactScalar::from_biguint(&pair_log.to_biguint());
            verify(&mut out);
            measurement::end_online();
            return out;
        }
        let (key, ty) = g.class(&y);
        out.count("canonicalisations_uncharged", 1);
        out.count("lookups_uncharged", 1);
        let Some(&d) = table.get(&key) else { continue };
        // 4. Recover: rebuild the table sum T, align the two conjugates and
        // read the sign off the ordinate:  φ^{tT}T = σ·φ^{ty}Y.
        let (i, b) = (d / b_count, d % b_count);
        let t_sum = g.add(
            &mut out.recover,
            base[(i * two_n) as usize],
            base[b as usize],
        );
        let (_, tt) = g.class(&t_sum);
        out.count("canonicalisations_uncharged", 1);
        let (ct, cy) = (g.frob(tt, t_sum), g.frob(ty, y));
        out.count("frobenius_maps_uncharged", 4);
        let sign_neg = if ct == cy {
            false
        } else if ct == g.neg(cy) {
            true
        } else {
            out.count("key_mismatch", 1);
            continue;
        };
        // Y = σ φ^{tT − tY} T, so log Y = σ λ^{(tT − tY) mod n} log T.
        let u = ((tt + n - ty) % n) as usize;
        let t_log = addmod(log_of(i * two_n), log_of(b));
        let mut y_log = mulmod(lambda_pow[u], t_log);
        if sign_neg {
            y_log = negmod(y_log);
        }
        out.count("queries_visited", qi);
        out.count("relations", 1);
        out.recovered = ExactScalar::from_biguint(&addmod(pair_log, y_log).to_biguint());
        verify(&mut out);
        measurement::end_online();
        return out;
    }
    measurement::end_online();
    out.count("queries_visited", qi);
    out.count("query_domain_exhausted", 1);
    out.exhausted = true;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::koblitz_instance;

    fn shape(base_scale: f64, table_scale: f64) -> ClawShape {
        ClawShape {
            base_scale,
            table_scale,
            max_adds: u64::MAX,
        }
    }

    #[test]
    fn claw_recovers_planted_logs_on_koblitz_curves() {
        for (a, n) in [(0u8, 13u32), (0, 41)] {
            let inst = koblitz_instance(a, n).expect("Koblitz instance");
            let g = BinaryGroup(&inst.fast);
            for (j, k) in [3u64, inst.r - 2, inst.r / 5 + 11].into_iter().enumerate() {
                let mut ops = GroupOps::default();
                let q = g.mul(&mut ops, inst.generator, k);
                for sh in [shape(4.0, 1.0), shape(1.06, 1.0 / 32.0)] {
                    let o = pair_claw(&inst, q, 7 + j as u64, sh).expect("koblitz");
                    if n == 13 && o.exhausted {
                        continue; // a 13-bit group can exhaust a tiny base
                    }
                    assert_eq!(
                        o.recovered,
                        Some(ExactScalar::from(k)),
                        "n={n} k={k} {sh:?} {:?}",
                        o.counters
                    );
                }
            }
        }
    }

    /// The claw on the u128 group (the wide path's arithmetic, class names
    /// and seed rule) recovers planted logs on a curve cheap enough to solve.
    #[test]
    fn wide_claw_recovers_planted_logs() {
        use crate::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
        use crate::cryptanalysis::koblitz_strong_rho::{RawPoint, StrongRho, StrongRhoG};
        use crate::cryptanalysis::wide_gf2m::WideGf2;
        for (a, n) in [(0u8, 41u32), (1, 47)] {
            let kc = KoblitzCurve::new(a, n).unwrap();
            let narrow = StrongRho::new(&kc);
            let RawPoint::Affine { x, y } = narrow.generator() else {
                unreachable!()
            };
            let rho: WideStrongRho = StrongRhoG::from_parts(
                WideGf2::new(&kc.curve.irreducible),
                u128::from(a),
                u128::from(narrow.modulus()),
                u128::from(narrow.lambda()),
                RawPointG::Affine {
                    x: x as u128,
                    y: y as u128,
                },
            );
            let g = WideClaw::from_parts(
                &rho,
                u128::from(narrow.modulus()),
                u128::from(narrow.lambda()),
            );
            for (j, k) in [5u128, g.r() - 3, g.r() / 7 + 2].into_iter().enumerate() {
                let q = rho.scalar_mul(rho.generator(), k);
                for sh in [shape(4.0, 1.0), shape(1.06, 1.0 / 32.0)] {
                    let o = pair_claw_on(&g, q, 3 + j as u64, sh);
                    assert!(!o.exhausted, "n={n} k={k} {sh:?} {:?}", o.counters);
                    assert_eq!(
                        o.recovered,
                        Some(ExactScalar::from(k)),
                        "n={n} k={k} {sh:?} {:?}",
                        o.counters
                    );
                }
            }
        }
    }

    /// The three-word claw ([`BigClaw`]: `Gf2W`, `U192`, three-word class
    /// keys) on curves cheap enough to solve.  With the wide seed rule it is
    /// the wide claw draw for draw: the same recovered logarithm, every
    /// counter and every phase's operations.  With its own seed rule
    /// (`ecbench.claw.seed.big`) it recovers the planted logarithms.
    #[test]
    fn big_claw_is_the_wide_claw_and_recovers_planted_logs() {
        use crate::cryptanalysis::certicom131_symlp::gf2w::Gf2W;
        use crate::cryptanalysis::koblitz_big::{BigStrongRho, U192};
        use crate::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
        use crate::cryptanalysis::koblitz_strong_rho::{RawPoint, StrongRho, StrongRhoG};
        use crate::cryptanalysis::wide_gf2m::WideGf2;

        /// [`BigClaw`] with [`WideClaw`]'s seed rule.
        struct WideSeeded<'a>(BigClaw<'a>, u128);
        impl ClawGroup for WideSeeded<'_> {
            type P = RawPointG<W192>;
            type S = U192;
            type K = W192;
            fn n(&self) -> u32 {
                self.0.n()
            }
            fn r(&self) -> U192 {
                self.0.r()
            }
            fn lambda(&self) -> U192 {
                self.0.lambda()
            }
            fn generator(&self) -> Self::P {
                self.0.generator()
            }
            fn add(&self, ops: &mut GroupOps, a: Self::P, b: Self::P) -> Self::P {
                self.0.add(ops, a, b)
            }
            fn double(&self, ops: &mut GroupOps, a: Self::P) -> Self::P {
                self.0.double(ops, a)
            }
            fn neg(&self, a: Self::P) -> Self::P {
                self.0.neg(a)
            }
            fn is_identity(&self, a: &Self::P) -> bool {
                self.0.is_identity(a)
            }
            fn identity(&self) -> Self::P {
                self.0.identity()
            }
            fn class(&self, p: &Self::P) -> (W192, u32) {
                self.0.class(p)
            }
            fn frob(&self, t: u32, p: Self::P) -> Self::P {
                self.0.frob(t, p)
            }
            fn seed_scalar(&self, seed: u64, i: u64) -> U192 {
                let k =
                    1 + derive_u128("ecbench.claw.seed.wide", &[seed.to_string(), i.to_string()])
                        % (self.1 - 1);
                U192::from_u128(k)
            }
        }

        for (a, n) in [(0u8, 41u32), (1, 47)] {
            let kc = KoblitzCurve::new(a, n).unwrap();
            let narrow = StrongRho::new(&kc);
            let RawPoint::Affine { x, y } = narrow.generator() else {
                unreachable!()
            };
            let (r, lambda) = (u128::from(narrow.modulus()), u128::from(narrow.lambda()));
            let wide: WideStrongRho = StrongRhoG::from_parts(
                WideGf2::new(&kc.curve.irreducible),
                u128::from(a),
                r,
                lambda,
                RawPointG::Affine {
                    x: x as u128,
                    y: y as u128,
                },
            );
            let big: BigStrongRho = StrongRhoG::from_parts(
                Gf2W::new(&kc.curve.irreducible),
                W192::from(u64::from(a)),
                U192::from_u128(r),
                U192::from_u128(lambda),
                RawPointG::Affine {
                    x: W192::from(x),
                    y: W192::from(y),
                },
            );
            let gw = WideClaw::from_parts(&wide, r, lambda);
            let gb = BigClaw::from_parts(&big, U192::from_u128(r), U192::from_u128(lambda));
            let seeded = WideSeeded(
                BigClaw::from_parts(&big, U192::from_u128(r), U192::from_u128(lambda)),
                r,
            );
            for (j, k) in [5u128, r - 3, r / 7 + 2].into_iter().enumerate() {
                let qw = wide.scalar_mul(wide.generator(), k);
                let qb = big.scalar_mul(big.generator(), U192::from_u128(k));
                for sh in [shape(4.0, 1.0), shape(1.06, 1.0 / 32.0)] {
                    for v2 in [false, true] {
                        let ow = pair_claw_with(&gw, qw, 3 + j as u64, sh, v2);
                        let os = pair_claw_with(&seeded, qb, 3 + j as u64, sh, v2);
                        assert_eq!(ow.recovered, Some(ExactScalar::from(k)));
                        assert_eq!(
                            (
                                &ow.recovered,
                                ow.exhausted,
                                &ow.counters,
                                ow.base,
                                ow.table,
                                ow.search,
                                ow.recover,
                                ow.verification
                            ),
                            (
                                &os.recovered,
                                os.exhausted,
                                &os.counters,
                                os.base,
                                os.table,
                                os.search,
                                os.recover,
                                os.verification
                            ),
                            "n={n} k={k} {sh:?} v2={v2}"
                        );
                        let ob = pair_claw_with(&gb, qb, 3 + j as u64, sh, v2);
                        assert!(!ob.exhausted, "n={n} k={k} {sh:?} {:?}", ob.counters);
                        assert_eq!(ob.recovered, Some(ExactScalar::from(k)), "n={n} k={k}");
                    }
                }
            }
        }
    }

    #[test]
    fn claw_is_deterministic_in_seed() {
        let inst = koblitz_instance(0, 41).unwrap();
        let g = BinaryGroup(&inst.fast);
        let mut ops = GroupOps::default();
        let q = g.mul(&mut ops, inst.generator, 123_456_789);
        let a = pair_claw(&inst, q, 99, shape(4.0, 1.0)).unwrap();
        let b = pair_claw(&inst, q, 99, shape(4.0, 1.0)).unwrap();
        assert_eq!(a.counters, b.counters);
        assert_eq!(a.search.adds, b.search.adds);
    }
}
