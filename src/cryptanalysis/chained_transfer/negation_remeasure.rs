//! # Negation-map re-measurement of the chained-transfer targets (2026-11-13).
//!
//! Protocol: `research/negation_remeasure_20261113/PROTOCOL.md`.  The
//! independent validation (`research/validation_20261112/REPORT.md`) refused
//! three targets because their matched rho was plain (`A = 1`) although the
//! negation map is available, and because rho's native work was not emitted.
//! This module provides the pieces of the re-measurement that are pure logic
//! and so can be tested:
//!
//! - [`rho_full`]: the tuned rho walk (negation or plain) on any counted group,
//!   with **every native counter emitted**: canonicalisations, partition hashes,
//!   distinguished-point table lookups and inserts, word operations.  The walk
//!   itself is not modified; a wrapper ([`Counted`]) counts the `key` and `neg`
//!   calls the walk makes, which is the exact total from which the partition
//!   hashes are derived.
//! - [`Class`], [`Arm`], [`classify`]: the pricing of every arm's native
//!   counters at the 5a band (0, 0.1, 1) and at the calibrated weight (0, `w_cal`,
//!   10 `w_cal`), applied to every arm in the same way, and the order class of
//!   an arm against the matched rho (a lower-bound arm is never classed `below`).
//! - [`best_of`], [`measure_class_times`]: the best-of-N timing that gives
//!   `w_cal = time(native operation) / time(group addition)` (AGENTS.md 5a,
//!   2026-11-12), with the host and method labelled by the caller.
//! - [`aut_kind`]: whether a curve has an automorphism fold (`j = 0`, `1728`).
//! - the v3 and cover instance builders, copies of the structural prefix of
//!   `chained_transfer_bench` and `cover_pushforward` so the re-measurement
//!   runs the very instances that were recorded.
//!
//! Every figure produced with it is `[pcc:unverified]` until a validator
//! replays it.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::hash::BuildHasherDefault;
use std::time::Instant;

use serde::Serialize;

use super::bielliptic::{Bielliptic, Quotient};
use super::certify;
use super::chain::{Chain, Elt, Hop};
use super::ec::{Curve, Pt, VeluIsogeny};
use crate::cryptanalysis::genus2_fp::Divisor;
use crate::cryptanalysis::ic_boundary::{
    rho_reference_walk, CountedGroup, GroupOps, MixHasher, RhoWalk,
};

/// The 5a band.
pub const BAND: [f64; 3] = [0.0, 0.1, 1.0];
/// Multiples of the calibrated weight at which the calibrated order is read.
pub const CAL_FACTORS: [f64; 3] = [0.0, 1.0, 10.0];
/// Batches and operations per batch of every best-of-N timing.
pub const TIMING_BATCHES: usize = 25;
pub const TIMING_BATCH_LEN: usize = 20_000;

// ── a counting wrapper ──────────────────────────────────────────────────

/// A [`CountedGroup`] that counts the `key` and `neg` calls made through it.
/// Group operations and results are those of the wrapped group.
pub struct Counted<'a, G: CountedGroup> {
    g: &'a G,
    keys: Cell<u64>,
    negs: Cell<u64>,
}

impl<'a, G: CountedGroup> Counted<'a, G> {
    pub fn new(g: &'a G) -> Self {
        Self {
            g,
            keys: Cell::new(0),
            negs: Cell::new(0),
        }
    }
    pub fn key_calls(&self) -> u64 {
        self.keys.get()
    }
    pub fn neg_calls(&self) -> u64 {
        self.negs.get()
    }
}

impl<G: CountedGroup> CountedGroup for Counted<'_, G> {
    type Elt = G::Elt;
    fn identity(&self) -> G::Elt {
        self.g.identity()
    }
    fn is_identity(&self, p: &G::Elt) -> bool {
        self.g.is_identity(p)
    }
    fn add(&self, ops: &mut GroupOps, p: G::Elt, q: G::Elt) -> G::Elt {
        self.g.add(ops, p, q)
    }
    fn double(&self, ops: &mut GroupOps, p: G::Elt) -> G::Elt {
        self.g.double(ops, p)
    }
    fn neg(&self, p: G::Elt) -> G::Elt {
        self.negs.set(self.negs.get() + 1);
        self.g.neg(p)
    }
    fn key(&self, p: &G::Elt) -> u64 {
        self.keys.set(self.keys.get() + 1);
        self.g.key(p)
    }
}

// ── rho with every native counter ───────────────────────────────────────

/// One rho run, split, with the native counters the group-operation unit does
/// not charge.
#[derive(Clone, Debug, Default, Serialize)]
pub struct RhoFull {
    pub kind: &'static str,
    pub recovered: Option<u64>,
    pub certified: bool,
    /// Every group operation: set-up, walk starts, walk, verification.
    pub total_ops: u64,
    /// Walk operations (steps, look-ahead additions, escape doublings).
    pub walk_ops: u64,
    pub other_ops: u64,
    pub steps: u64,
    pub walks: u64,
    /// Canonicalisations (negation: `neg` and two key evaluations each).
    pub canonicalisations: u64,
    /// Key-and-mix evaluations that pick the jump and test the
    /// distinguished-point bit: `key_evaluations` minus the keys inside
    /// canonicalisations and minus the keys of the table lookups.
    pub partition_hashes: u64,
    pub dp_lookups: u64,
    pub dp_inserts: u64,
    /// Word operations of the short-cycle window and Brent's detector
    /// (`work_native` of the walk).
    pub word_ops: u64,
    /// Every `CountedGroup::key` call (exact, from the wrapper).
    pub key_evaluations: u64,
    pub neg_calls: u64,
    pub useless_collisions: u64,
    pub look_ahead_additions: u64,
    pub cycle_escapes: u64,
}

impl RhoFull {
    /// Native operations in all classes.
    pub fn native(&self) -> u64 {
        self.canonicalisations
            + self.partition_hashes
            + self.dp_lookups
            + self.dp_inserts
            + self.word_ops
    }
    /// As an [`Arm`] (counts by class; `walk` selects the walk-only reading).
    pub fn arm(&self) -> Arm {
        Arm {
            group_ops: self.total_ops as f64,
            native: vec![
                (Class::Canon, self.canonicalisations as f64),
                (Class::Hash, self.partition_hashes as f64),
                (Class::Lookup, self.dp_lookups as f64),
                (Class::Insert, self.dp_inserts as f64),
                (Class::Word, self.word_ops as f64),
            ],
            lower_bound: false,
        }
    }
}

/// The tuned walk (negation or plain), counted, certified by the caller's
/// independent `[k]G == T`.  The walk and its random stream are those of
/// [`rho_reference_walk`] exactly: the totals equal `instrument::rho_ref`.
pub fn rho_full<G: CountedGroup>(
    g: &G,
    generator: G::Elt,
    target: G::Elt,
    r: u64,
    seed: u64,
    negation: bool,
    certify: &dyn Fn(u64) -> bool,
) -> RhoFull {
    let cg = Counted::new(g);
    let max_steps = (60.0 * (r as f64).sqrt()) as u64 + 10_000;
    let walk = if negation {
        RhoWalk::negation()
    } else {
        RhoWalk::plain()
    };
    let rr = rho_reference_walk(&cg, generator, target, r, seed, max_steps, walk);
    let total = rr.group_ops.adds + rr.group_ops.doubles;
    let c = |k: &str| rr.counters.get(k).copied().unwrap_or(0);
    let walk_ops = rr
        .counters
        .get("walk_operations")
        .copied()
        .unwrap_or(rr.steps);
    let canon = c("canonicalisations_uncharged");
    let useless = c("useless_collisions");
    let dp_lookups = rr.distinguished_points;
    // A lookup that finds the stored point is a hit (useless or the solution);
    // every other lookup inserts.
    let hits = useless + u64::from(rr.recovered.is_some());
    let dp_inserts = dp_lookups.saturating_sub(hits);
    let keys = cg.key_calls();
    // Each canonicalisation of the negation classes evaluates the key twice;
    // the plain classes canonicalise nothing.
    let canon_keys = if negation { 2 * canon } else { 0 };
    let partition_hashes = keys.saturating_sub(canon_keys + dp_lookups);
    RhoFull {
        kind: if negation { "negation" } else { "plain" },
        recovered: rr.recovered,
        certified: rr.recovered.is_some_and(certify),
        total_ops: total,
        walk_ops,
        other_ops: total.saturating_sub(walk_ops),
        steps: rr.steps,
        walks: rr.walks,
        canonicalisations: canon,
        partition_hashes,
        dp_lookups,
        dp_inserts,
        word_ops: rr.work_native.values().sum(),
        key_evaluations: keys,
        neg_calls: cg.neg_calls(),
        useless_collisions: useless,
        look_ahead_additions: c("look_ahead_additions"),
        cycle_escapes: c("cycle_escape_doublings"),
    }
}

/// [`rho_full`] on an elliptic curve, certified by `Curve::mul`.
pub fn rho_full_ec(e: &Curve, gen: Pt, target: Pt, r: u64, seed: u64, negation: bool) -> RhoFull {
    let eg = super::measure::EcGroup(*e);
    rho_full(&eg, gen, target, r, seed, negation, &|k| {
        e.mul(&gen, k as u128) == target
    })
}

/// Means over runs (every field a mean, `runs` and `certified` counts).
#[derive(Clone, Debug, Default, Serialize)]
pub struct RhoMean {
    pub kind: String,
    pub runs: usize,
    pub certified: usize,
    pub total_ops: f64,
    pub walk_ops: f64,
    pub other_ops: f64,
    pub steps: f64,
    pub walks: f64,
    pub canonicalisations: f64,
    pub partition_hashes: f64,
    pub dp_lookups: f64,
    pub dp_inserts: f64,
    pub word_ops: f64,
    pub key_evaluations: f64,
    pub native: f64,
    /// `native / steps`, the quantity of prediction N2.
    pub native_per_step: f64,
    pub walk_share: f64,
}

pub fn mean_rho(rs: &[RhoFull]) -> RhoMean {
    let n = rs.len().max(1) as f64;
    let m = |f: &dyn Fn(&RhoFull) -> f64| rs.iter().map(f).sum::<f64>() / n;
    let steps = m(&|r| r.steps as f64);
    let native = m(&|r| r.native() as f64);
    let total = m(&|r| r.total_ops as f64);
    let walk = m(&|r| r.walk_ops as f64);
    RhoMean {
        kind: rs.first().map_or("", |r| r.kind).to_string(),
        runs: rs.len(),
        certified: rs.iter().filter(|r| r.certified).count(),
        total_ops: total,
        walk_ops: walk,
        other_ops: m(&|r| r.other_ops as f64),
        steps,
        walks: m(&|r| r.walks as f64),
        canonicalisations: m(&|r| r.canonicalisations as f64),
        partition_hashes: m(&|r| r.partition_hashes as f64),
        dp_lookups: m(&|r| r.dp_lookups as f64),
        dp_inserts: m(&|r| r.dp_inserts as f64),
        word_ops: m(&|r| r.word_ops as f64),
        key_evaluations: m(&|r| r.key_evaluations as f64),
        native,
        native_per_step: native / steps.max(1.0),
        walk_share: walk / total.max(1.0),
    }
}

impl RhoMean {
    pub fn arm(&self) -> Arm {
        Arm {
            group_ops: self.total_ops,
            native: vec![
                (Class::Canon, self.canonicalisations),
                (Class::Hash, self.partition_hashes),
                (Class::Lookup, self.dp_lookups),
                (Class::Insert, self.dp_inserts),
                (Class::Word, self.word_ops),
            ],
            lower_bound: false,
        }
    }
}

// ── native classes, arms, pricing ───────────────────────────────────────

/// A class of native operation, priced by its own calibrated weight.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Class {
    /// Multiplication mod `p` (relation phase, oracle, factor base).
    MulP,
    /// Multiplication mod `r` (linear algebra).
    MulR,
    /// Inversion mod `r` (Fermat).
    InvR,
    /// Canonicalisation: `neg`, two key evaluations, a compare.
    Canon,
    /// Key plus mix: a partition (jump or distinguished-bit) hash.
    Hash,
    /// Hash-table lookup.
    Lookup,
    /// Hash-table insert.
    Insert,
    /// A random draw (xorshift).
    Draw,
    /// A 64-bit word operation (window or Brent compare).
    Word,
}

impl Class {
    pub const ALL: [Class; 9] = [
        Class::MulP,
        Class::MulR,
        Class::InvR,
        Class::Canon,
        Class::Hash,
        Class::Lookup,
        Class::Insert,
        Class::Draw,
        Class::Word,
    ];
    pub fn name(self) -> &'static str {
        match self {
            Class::MulP => "mul_p",
            Class::MulR => "mul_r",
            Class::InvR => "inv_r",
            Class::Canon => "canon",
            Class::Hash => "hash",
            Class::Lookup => "lookup",
            Class::Insert => "insert",
            Class::Draw => "draw",
            Class::Word => "word",
        }
    }
}

/// An arm: group operations of its own group (1 GAE each) and its native
/// counters by class.  `lower_bound`: some native work is not emitted.
#[derive(Clone, Debug, Default)]
pub struct Arm {
    pub group_ops: f64,
    pub native: Vec<(Class, f64)>,
    pub lower_bound: bool,
}

/// Nanoseconds per operation of every class, best of N.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct ClassTimes {
    pub mul_p: f64,
    pub mul_r: f64,
    pub inv_r: f64,
    pub canon: f64,
    pub hash: f64,
    pub lookup: f64,
    pub insert: f64,
    pub draw: f64,
    pub word: f64,
}

impl ClassTimes {
    pub fn get(&self, c: Class) -> f64 {
        match c {
            Class::MulP => self.mul_p,
            Class::MulR => self.mul_r,
            Class::InvR => self.inv_r,
            Class::Canon => self.canon,
            Class::Hash => self.hash,
            Class::Lookup => self.lookup,
            Class::Insert => self.insert,
            Class::Draw => self.draw,
            Class::Word => self.word,
        }
    }
    /// `w_cal` of a class: its time over the time of one group addition.
    pub fn w_cal(&self, c: Class, t_add_ns: f64) -> f64 {
        self.get(c) / t_add_ns
    }
}

impl Arm {
    /// Total at `w` GAE per native operation, the same `w` on every class.
    pub fn total(&self, w: f64) -> f64 {
        self.group_ops + w * self.native.iter().map(|(_, n)| n).sum::<f64>()
    }
    /// Total with every class at `factor` times its own `w_cal`.
    pub fn total_cal(&self, t: &ClassTimes, t_add_ns: f64, factor: f64) -> f64 {
        self.group_ops
            + self
                .native
                .iter()
                .map(|&(c, n)| factor * t.w_cal(c, t_add_ns) * n)
                .sum::<f64>()
    }
    pub fn band(&self) -> [f64; 3] {
        BAND.map(|w| self.total(w))
    }
    pub fn cal(&self, t: &ClassTimes, t_add_ns: f64) -> [f64; 3] {
        CAL_FACTORS.map(|f| self.total_cal(t, t_add_ns, f))
    }
    pub fn native_total(&self) -> f64 {
        self.native.iter().map(|(_, n)| n).sum()
    }
    /// Counts by class name (zero classes left out).
    pub fn native_map(&self) -> BTreeMap<String, f64> {
        let mut m = BTreeMap::new();
        for &(c, n) in &self.native {
            *m.entry(c.name().to_string()).or_insert(0.0) += n;
        }
        m.retain(|_, v| *v != 0.0);
        m
    }
}

/// Order of an arm against a reference at three prices.  A `lower_bound` arm
/// is never `below`: a lower bound under the reference proves nothing.
pub fn classify(arm: &[f64; 3], reference: &[f64; 3], lower_bound: bool) -> &'static str {
    let below = arm.iter().zip(reference).all(|(a, r)| a < r);
    let above = arm.iter().zip(reference).all(|(a, r)| a > r);
    match (above, below, lower_bound) {
        (true, _, _) => "above",
        (_, true, false) => "below",
        (_, true, true) => "unresolved (lower bound)",
        (_, _, true) => "unresolved (lower bound)",
        _ => "unresolved",
    }
}

/// Elementwise ratio.
pub fn ratio3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0] / b[0], a[1] / b[1], a[2] / b[2]]
}

/// Elementwise minimum (the strongest of two references at each price).
pub fn min3(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[2].min(b[2])]
}

/// The matched rho references of a row.
pub struct RefSet {
    pub neg: Arm,
    pub plain: Arm,
    /// Time of one addition on the rho curve (ns), for `w_cal`.
    pub t_add_ns: f64,
    pub times: ClassTimes,
}

/// What an arm's report holds against the references.
#[derive(Clone, Debug, Serialize)]
pub struct ArmReport {
    pub name: String,
    pub group_ops: f64,
    pub native: BTreeMap<String, f64>,
    pub lower_bound: bool,
    pub t_add_ns: f64,
    pub total_band: [f64; 3],
    pub total_cal: [f64; 3],
    pub ratio_neg_band: [f64; 3],
    pub ratio_neg_cal: [f64; 3],
    pub ratio_plain_band: [f64; 3],
    pub ratio_best_band: [f64; 3],
    pub class_neg_band: String,
    pub class_neg_cal: String,
    pub class_plain_band: String,
    pub class_best_band: String,
    pub class_best_cal: String,
    /// The calibrated order (0, `w_cal`, 10 `w_cal`) is one order, `above` or
    /// `below`, for the negation reference.
    pub ranked_at_calibrated_price: bool,
}

/// Compare `arm` (whose group addition takes `t_add_ns`, native classes timed
/// by `arm_times`) with the references of the row.
pub fn report_arm(
    name: &str,
    arm: &Arm,
    t_add_ns: f64,
    arm_times: &ClassTimes,
    refs: &RefSet,
) -> ArmReport {
    let tb = arm.band();
    let tc = arm.cal(arm_times, t_add_ns);
    let (nb, pb) = (refs.neg.band(), refs.plain.band());
    let (nc, pc) = (
        refs.neg.cal(&refs.times, refs.t_add_ns),
        refs.plain.cal(&refs.times, refs.t_add_ns),
    );
    let (bb, bc) = (min3(&nb, &pb), min3(&nc, &pc));
    let class_neg_cal = classify(&tc, &nc, arm.lower_bound);
    ArmReport {
        name: name.to_string(),
        group_ops: arm.group_ops,
        native: arm.native_map(),
        lower_bound: arm.lower_bound,
        t_add_ns,
        total_band: tb,
        total_cal: tc,
        ratio_neg_band: ratio3(&tb, &nb),
        ratio_neg_cal: ratio3(&tc, &nc),
        ratio_plain_band: ratio3(&tb, &pb),
        ratio_best_band: ratio3(&tb, &bb),
        class_neg_band: classify(&tb, &nb, arm.lower_bound).to_string(),
        class_neg_cal: class_neg_cal.to_string(),
        class_plain_band: classify(&tb, &pb, arm.lower_bound).to_string(),
        class_best_band: classify(&tb, &bb, arm.lower_bound).to_string(),
        class_best_cal: classify(&tc, &bc, arm.lower_bound).to_string(),
        ranked_at_calibrated_price: class_neg_cal == "above" || class_neg_cal == "below",
    }
}

// ── best-of-N timing ────────────────────────────────────────────────────

/// Nanoseconds per call of `f`: the minimum over `batches` batches of
/// `batch_len` calls.  A best-of-N figure, not an isolated-host measurement.
pub fn best_of<F: FnMut()>(batches: usize, batch_len: usize, mut f: F) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..batches {
        let t = Instant::now();
        for _ in 0..batch_len {
            f();
        }
        let ns = t.elapsed().as_nanos() as f64 / batch_len as f64;
        best = best.min(ns);
    }
    best
}

fn mix_like(v: u64) -> u64 {
    // The arithmetic of the walk's partition mix (xor, two multiplies, two
    // xor-shifts); the walk's own `mix` is private.
    let mut h = v ^ 0x2545_F491_4F6C_DD1D;
    h = h.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    h ^= h >> 31;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 29;
    h
}

fn powmod_u64(a: u64, mut e: u64, m: u64) -> u64 {
    let (mut b, mut res) = (a % m, 1u64);
    while e > 0 {
        if e & 1 == 1 {
            res = res * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    res
}

/// Time of every class of native operation, `batches` x `batch_len` best-of-N.
/// `e` and `pt` give the canonicalisation and hash their real element type;
/// `p < 2^31` and `r < 2^31` keep the products in a `u64`.
pub fn measure_class_times(
    p: u64,
    r: u64,
    e: &Curve,
    pt: Pt,
    batches: usize,
    batch_len: usize,
) -> ClassTimes {
    use std::hint::black_box;
    let eg = super::measure::EcGroup(*e);
    let mut x = black_box(0x1234_5678u64 % p) | 1;
    let y = black_box(0x2BCD_EF01u64 % p) | 1;
    let mul_p = best_of(batches, batch_len, || {
        x = x * y % p;
        black_box(&x);
    });
    let mut z = black_box(0x1234_5678u64 % r) | 1;
    let w = black_box(0x2BCD_EF01u64 % r) | 1;
    let mul_r = best_of(batches, batch_len, || {
        z = z * w % r;
        black_box(&z);
    });
    let mut inv = black_box(3u64);
    let inv_r = best_of(batches, batch_len / 20, || {
        inv = powmod_u64(black_box(inv % r) | 1, r - 2, r);
        black_box(&inv);
    });
    let canon = best_of(batches, batch_len, || {
        let n = eg.neg(black_box(pt));
        let swap = eg.key(&n) < eg.key(&pt);
        black_box(swap);
    });
    let hash = best_of(batches, batch_len, || {
        black_box(mix_like(eg.key(black_box(&pt))));
    });
    // The walk's own table type (MixHasher), 512 entries, alternating hit/miss.
    let mut table: HashMapFast = HashMapFast::with_capacity_and_hasher(1024, Default::default());
    for i in 0..512u64 {
        table.insert(mix_like(i), (i, i));
    }
    let mut i = 0u64;
    let lookup = best_of(batches, batch_len, || {
        i = i.wrapping_add(1);
        black_box(table.get(&mix_like(black_box(i & 1023))));
    });
    let mut t2: HashMapFast = HashMapFast::with_capacity_and_hasher(1024, Default::default());
    let mut j = 0u64;
    let insert = best_of(batches, batch_len, || {
        j = j.wrapping_add(1);
        if t2.len() >= 1024 {
            t2.clear();
        }
        t2.insert(mix_like(j), (j, j));
    });
    let mut s = black_box(0x9E37_79B9_7F4A_7C15u64);
    let draw = best_of(batches, batch_len, || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        black_box(&s);
    });
    let (mut a, b) = (black_box(5u64), black_box(7u64));
    let word = best_of(batches, batch_len, || {
        a = a.wrapping_add(u64::from(a == b));
        black_box(&a);
    });
    ClassTimes {
        mul_p,
        mul_r,
        inv_r,
        canon,
        hash,
        lookup,
        insert,
        draw,
        word,
    }
}

type HashMapFast = std::collections::HashMap<u64, (u64, u64), BuildHasherDefault<MixHasher>>;

/// Time of one addition on an elliptic curve (ns), over a ring of 64 multiples
/// of `gen`.
pub fn time_add_ec(e: &Curve, gen: Pt, batches: usize, batch_len: usize) -> f64 {
    use std::hint::black_box;
    let pts: Vec<Pt> = (1..=64u128).map(|m| e.mul(&gen, m * 7)).collect();
    let mut acc: Pt = None;
    let mut i = 0usize;
    let t = best_of(batches, batch_len, || {
        acc = e.add(&acc, &pts[i % 64]);
        i += 1;
        black_box(&acc);
    });
    black_box(&acc);
    t
}

// ── automorphism folds ──────────────────────────────────────────────────

/// `Some("cubic (j = 0)")` or `Some("quartic (j = 1728)")` where the curve has
/// an automorphism beyond negation; `None` for a generic curve.
pub fn aut_kind(e: &Curve) -> Option<&'static str> {
    let j = e.j_invariant();
    if j == 0 {
        Some("cubic (j = 0)")
    } else if j == 1728 % e.k.p {
        Some("quartic (j = 1728)")
    } else {
        None
    }
}

// ── the recorded instances ──────────────────────────────────────────────

pub fn largest_cyclic_prime(mut n: u64) -> u64 {
    let mut best = 1u64;
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            if e == 1 {
                best = best.max(d);
            }
        }
        d += 1;
    }
    if n > 1 {
        best = best.max(n);
    }
    best
}

pub fn ec_instance(e: &Curve, r: u64, x: u64) -> Option<(Pt, Pt)> {
    let g = ec_gen(e, r)?;
    Some((g, e.mul(&g, x as u128)))
}

pub fn ec_gen(e: &Curve, r: u64) -> Option<Pt> {
    let order = e.order();
    if !order.is_multiple_of(r) {
        return None;
    }
    let cof = order / r;
    for s in 1..5000u64 {
        let g = e.mul(&e.point_from_seed(s.wrapping_mul(0x9E37_79B9)), cof as u128);
        if g.is_some() && e.mul(&g, r as u128).is_none() {
            return Some(g);
        }
    }
    None
}

pub fn find_isogeny(e: &Curve, r: u64) -> Option<(VeluIsogeny, Pt)> {
    let order = e.order();
    for ell in [2u64, 3, 5, 7] {
        if !order.is_multiple_of(ell) || r == ell {
            continue;
        }
        let cof = order / ell;
        for s in 1..2000u64 {
            let g = e.mul(
                &e.point_from_seed(s.wrapping_mul(2_654_435_761)),
                cof as u128,
            );
            if g.is_none() {
                continue;
            }
            if let Some(iso) = VeluIsogeny::from_generator(e, &g) {
                return Some((iso, g));
            }
        }
    }
    None
}

/// The v3 instance of one candidate (the structural prefix of
/// `chained_transfer_bench::measure_curve`).
pub struct V3Inst {
    pub e1: Curve,
    pub jac: crate::cryptanalysis::genus2_fp::Genus2Curve,
    pub r: u64,
    pub x: u64,
    pub gen: Pt,
    pub target: Pt,
    pub dg: Divisor,
    pub dt: Divisor,
    pub e1_order: u64,
    pub e2_order: u64,
    pub jac_order: u64,
    pub seed: u64,
}

/// `Err` exactly where the bench skipped the candidate.
pub fn v3_candidate(p: u64, c: [u64; 4], seed: u64) -> Result<V3Inst, &'static str> {
    let b = Bielliptic::new(p, c).map_err(|_| "bielliptic_rejected")?;
    let e1 = *b.curve(Quotient::E1);
    let e2 = *b.curve(Quotient::E2);
    let jac = b.jac.clone();
    let (e1_order, e2_order) = (e1.order(), e2.order());
    let jac_order = e1_order * e2_order;
    let r = largest_cyclic_prime(e1_order);
    if r < 50 {
        return Err("r_below_50");
    }
    if e2_order.is_multiple_of(r) || jac_order.is_multiple_of(r * r) {
        return Err("r_not_cyclic");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let (gen, target) = ec_instance(&e1, r, x).ok_or("no_order_r_point")?;
    certify::pt_elem(&gen).map_err(|_| "non_rational_point")?;
    certify::pt_elem(&target).map_err(|_| "non_rational_point")?;
    let (iso, _kernel) = find_isogeny(&e1, r).ok_or("no_small_isogeny")?;
    let hop = Hop::isogeny(iso, &gen, &target, r).map_err(|_| "isogeny_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("isogeny_chain_unverified");
    }
    let (gp, tp) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::E(a), Elt::E(b)) => (a, b),
        _ => (None, None),
    };
    certify::pt_elem(&gp).map_err(|_| "non_rational_point")?;
    certify::pt_elem(&tp).map_err(|_| "non_rational_point")?;
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).map_err(|_| "pullback_hop_failed")?;
    let ch = Chain::new(vec![hop]);
    if !ch.verify(&Elt::E(gen), r, x as u128) {
        return Err("pullback_chain_unverified");
    }
    let (dg, dt) = match (ch.transport(&Elt::E(gen)), ch.transport(&Elt::E(target))) {
        (Elt::J(a), Elt::J(b)) => (a, b),
        _ => return Err("pullback_not_a_divisor"),
    };
    Ok(V3Inst {
        e1,
        jac,
        r,
        x,
        gen,
        target,
        dg,
        dt,
        e1_order,
        e2_order,
        jac_order,
        seed,
    })
}

/// The bench's candidate coefficients for loop index `s` at prime `p`.
pub fn candidate_coeffs(p: u64, s: u64) -> [u64; 4] {
    [
        1 + (s.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
        (s.wrapping_mul(40_503) >> 5) % p,
        (s.wrapping_mul(2_246_822_519) >> 7) % p,
        1 + (s.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
    ]
}

/// First v3 candidate of prime number `i` (`0xC0FFEE + 101 i + s`) and its
/// loop index.
pub fn v3_first(p: u64, i: usize) -> Option<(u64, V3Inst)> {
    (0..4000u64).find_map(|s| {
        v3_candidate(p, candidate_coeffs(p, s), 0xC0FFEE + i as u64 * 101 + s)
            .ok()
            .map(|v| (s, v))
    })
}

/// The cover_pushforward curve of a prime: the first candidate that passes the
/// same admissibility checks (a copy of `cover_pushforward::admissible`).
pub struct CoverCurve {
    pub b: Bielliptic,
    pub candidate: u64,
    pub seed0: u64,
    pub r: u64,
    pub gen: Pt,
}

fn cover_admissible(p: u64, c: [u64; 4], seed: u64) -> Result<(Bielliptic, u64, Pt), &'static str> {
    let b = Bielliptic::new(p, c).map_err(|_| "bielliptic_rejected")?;
    let (n1, n2) = (b.e1.order(), b.e2.order());
    let r = largest_cyclic_prime(n1);
    if r < 50 {
        return Err("r_below_50");
    }
    if n2.is_multiple_of(r) {
        return Err("r_divides_e2_order");
    }
    if (n1 * n2).is_multiple_of(r * r) {
        return Err("r_squared_divides_jac_order");
    }
    let x = (seed.wrapping_mul(2_246_822_519).wrapping_add(7)) % r;
    let gen = ec_gen(&b.e1, r).ok_or("no_order_r_point_on_E1")?;
    let target = b.e1.mul(&gen, x as u128);
    let (iso, _k) = find_isogeny(&b.e1, r).ok_or("no_rational_small_isogeny")?;
    let hop = Hop::isogeny(iso, &gen, &target, r).map_err(|_| "isogeny_hop_failed")?;
    if !Chain::new(vec![hop]).verify(&Elt::E(gen), r, x as u128) {
        return Err("isogeny_chain_unverified");
    }
    let hop = Hop::pullback(&b, Quotient::E1, &gen, r).map_err(|_| "pullback_hop_failed")?;
    if !Chain::new(vec![hop]).verify(&Elt::E(gen), r, x as u128) {
        return Err("pullback_chain_unverified");
    }
    Ok((b, r, gen))
}

pub fn cover_select(p: u64, i: usize) -> Option<CoverCurve> {
    (0..4000u64).find_map(|s| {
        let seed0 = 0xC0FFEE + i as u64 * 101 + s;
        cover_admissible(p, candidate_coeffs(p, s), seed0)
            .ok()
            .map(|(b, r, gen)| CoverCurve {
                b,
                candidate: s,
                seed0,
                r,
                gen,
            })
    })
}

#[cfg(test)]
mod tests {
    use super::super::field::Fp;
    use super::super::instrument::rho_on_curve;
    use super::*;

    fn small_curve() -> (Curve, Pt, Pt, u64, u64) {
        let k = Fp::new(1009).unwrap();
        for a6 in 1..400u64 {
            let Some(e) = Curve::new(k, 0, 3, a6) else {
                continue;
            };
            let r = largest_cyclic_prime(e.order());
            if r < 150 {
                continue;
            }
            let Some(g) = ec_gen(&e, r) else { continue };
            let x = 37 % r;
            return (e, g, e.mul(&g, x as u128), r, x);
        }
        panic!("no curve");
    }

    #[test]
    fn rho_full_totals_equal_the_instrument_walk_and_counters_add_up() {
        let (e, g, t, r, x) = small_curve();
        for neg in [true, false] {
            for seed in [5u64, 6, 7] {
                let a = rho_full_ec(&e, g, t, r, seed, neg);
                let b = rho_on_curve(&e, g, t, r, seed, neg);
                assert!(a.certified && a.recovered.map(|k| k % r) == Some(x));
                // the wrapper does not change the walk
                assert_eq!(a.total_ops, b.total_ops);
                assert_eq!(a.walk_ops, b.walk_ops);
                assert_eq!(a.recovered, b.recovered);
                assert_eq!(a.total_ops, a.walk_ops + a.other_ops);
                // every key call is a canonicalisation key, a table key or a
                // partition hash
                let canon_keys = if neg { 2 * a.canonicalisations } else { 0 };
                assert_eq!(
                    a.key_evaluations,
                    canon_keys + a.dp_lookups + a.partition_hashes
                );
                // one partition hash per attempted step, per escape and per
                // walk start (which is at most one per walk)
                let attempts = a.steps + a.look_ahead_additions + a.cycle_escapes;
                assert!(a.partition_hashes >= attempts, "{a:?}");
                assert!(a.partition_hashes <= attempts + a.walks, "{a:?}");
                // every lookup is a hit or an insert
                assert!(a.dp_inserts + a.useless_collisions + 1 >= a.dp_lookups);
                assert!(a.dp_inserts <= a.dp_lookups);
                assert_eq!(a.neg_calls > 0, neg);
                assert_eq!(a.canonicalisations > 0, neg);
                assert!(a.native() > 0);
            }
        }
    }

    #[test]
    fn negation_walk_emits_more_native_work_per_step_than_plain() {
        let (e, g, t, r, _) = small_curve();
        let n: Vec<RhoFull> = (0..8)
            .map(|s| rho_full_ec(&e, g, t, r, 100 + s, true))
            .collect();
        let p: Vec<RhoFull> = (0..8)
            .map(|s| rho_full_ec(&e, g, t, r, 100 + s, false))
            .collect();
        let (mn, mp) = (mean_rho(&n), mean_rho(&p));
        assert_eq!((mn.runs, mn.certified), (8, 8));
        assert_eq!((mp.runs, mp.certified), (8, 8));
        assert!(mn.native_per_step > mp.native_per_step);
        assert!(mp.native_per_step >= 1.0, "a hash per step at least");
        assert_eq!(mp.canonicalisations, 0.0);
    }

    #[test]
    fn classify_never_calls_a_lower_bound_below() {
        let rho = [10.0, 20.0, 30.0];
        assert_eq!(classify(&[1.0, 2.0, 3.0], &rho, false), "below");
        assert_eq!(
            classify(&[1.0, 2.0, 3.0], &rho, true),
            "unresolved (lower bound)"
        );
        assert_eq!(classify(&[11.0, 21.0, 31.0], &rho, true), "above");
        assert_eq!(classify(&[11.0, 21.0, 31.0], &rho, false), "above");
        assert_eq!(classify(&[1.0, 25.0, 31.0], &rho, false), "unresolved");
        // equal at a price is not an order
        assert_eq!(classify(&[10.0, 25.0, 31.0], &rho, false), "unresolved");
    }

    #[test]
    fn pricing_applies_one_weight_to_every_class_and_the_calibrated_one_per_class() {
        let arm = Arm {
            group_ops: 100.0,
            native: vec![(Class::MulP, 1000.0), (Class::Hash, 10.0)],
            lower_bound: false,
        };
        assert_eq!(arm.band(), [100.0, 100.0 + 101.0, 100.0 + 1010.0]);
        let t = ClassTimes {
            mul_p: 2.0,
            hash: 10.0,
            ..Default::default()
        };
        // group add 1000 ns: w_cal(mul_p) = 0.002, w_cal(hash) = 0.01
        let c = arm.cal(&t, 1000.0);
        assert!((c[0] - 100.0).abs() < 1e-12);
        assert!((c[1] - (100.0 + 2.0 + 0.1)).abs() < 1e-12);
        assert!((c[2] - (100.0 + 20.0 + 1.0)).abs() < 1e-12);
        assert_eq!(arm.native_map().len(), 2);
    }

    #[test]
    fn report_arm_ranks_and_flags() {
        let (e, g, t, r, _) = small_curve();
        let neg = mean_rho(
            &(0..4)
                .map(|s| rho_full_ec(&e, g, t, r, s, true))
                .collect::<Vec<_>>(),
        );
        let plain = mean_rho(
            &(0..4)
                .map(|s| rho_full_ec(&e, g, t, r, s, false))
                .collect::<Vec<_>>(),
        );
        let times = ClassTimes {
            mul_p: 2.0,
            mul_r: 2.0,
            inv_r: 100.0,
            canon: 5.0,
            hash: 4.0,
            lookup: 10.0,
            insert: 20.0,
            draw: 1.0,
            word: 0.5,
        };
        let refs = RefSet {
            neg: neg.arm(),
            plain: plain.arm(),
            t_add_ns: 400.0,
            times,
        };
        let big = Arm {
            group_ops: 1e6,
            native: vec![(Class::MulR, 1e6)],
            lower_bound: true,
        };
        let rep = report_arm("big", &big, 4000.0, &times, &refs);
        assert_eq!(rep.class_neg_band, "above");
        assert_eq!(rep.class_neg_cal, "above");
        assert!(rep.ranked_at_calibrated_price);
        let small = Arm {
            group_ops: 1.0,
            native: vec![],
            lower_bound: true,
        };
        let rep = report_arm("small", &small, 4000.0, &times, &refs);
        assert_eq!(rep.class_neg_band, "unresolved (lower bound)");
        assert!(!rep.ranked_at_calibrated_price);
        let small = Arm {
            lower_bound: false,
            ..small
        };
        assert_eq!(
            report_arm("small", &small, 4000.0, &times, &refs).class_neg_band,
            "below"
        );
    }

    #[test]
    fn timing_gives_positive_finite_times() {
        let (e, g, _, r, _) = small_curve();
        let t = measure_class_times(1009, r, &e, g, 3, 400);
        for c in Class::ALL {
            let v = t.get(c);
            assert!(v.is_finite() && v > 0.0, "{c:?} {v}");
        }
        let a = time_add_ec(&e, g, 3, 400);
        assert!(a.is_finite() && a > 0.0);
        assert!(t.w_cal(Class::MulP, a) > 0.0);
        // a counter increments are what a best-of timing times
        let mut n = 0u32;
        let ns = best_of(2, 10, || n += 1);
        assert_eq!(n, 20);
        assert!(ns.is_finite());
    }

    #[test]
    fn automorphism_kinds_are_detected() {
        let k = Fp::new(1009).unwrap();
        // y^2 = x^3 + 3: j = 0 ; y^2 = x^3 + 3x: j = 1728
        assert_eq!(
            aut_kind(&Curve::new(k, 0, 0, 3).unwrap()),
            Some("cubic (j = 0)")
        );
        assert_eq!(
            aut_kind(&Curve::new(k, 0, 3, 0).unwrap()),
            Some("quartic (j = 1728)")
        );
        assert_eq!(aut_kind(&small_curve().0), None);
    }

    #[test]
    fn v3_p211_reproduces_the_recorded_candidate_and_certifies() {
        // results_v3.json: candidate 9 at p = 211, rho on E1 428 group ops
        let (cand, v) = v3_first(211, 0).expect("candidate");
        assert_eq!(cand, 9);
        assert_eq!(v.e1.mul(&v.gen, v.x as u128), v.target);
        assert!(v.jac.mul(&v.dg, v.r).is_identity());
        assert_eq!(v.jac.mul(&v.dg, v.x), v.dt);
        let (cc, cv) = {
            let c = cover_select(211, 0).expect("cover curve");
            (c.candidate, c)
        };
        assert_eq!(cc, 9, "cover_pushforward uses the same first candidate");
        assert_eq!(cv.r, v.r);
        // a rho on the very instance
        let rr = rho_full_ec(&v.e1, v.gen, v.target, v.r, 1000, true);
        assert!(rr.certified);
    }
}
