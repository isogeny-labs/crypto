//! Factorial ablation of the PR #5229 five-term MITM on the production Prime31
//! norm-one-torus/Dickson factor base, plus an affine x-only query arm.
//!
//! The precommitted sources of PR #5229 (`examples/norm_one_torus_mitm.rs` and
//! its support module) are left byte-identical so their manifest hashes stay
//! valid. This example copies the solver and parameterises it with four factors:
//!
//! * normalization: individual affine conversion | Montgomery batch
//! * three-sum table: full | canonical negation quotient keyed by x
//! * inversion: i128 extended Euclid (as in #5229) | i64 extended Euclid | Fermat
//! * hasher: std SipHash | Fx-style multiplicative hash
//!
//! The extra arm `x-only` keeps batch + quotient + i64 + Fx and replaces the
//! Jacobian query with affine `T - L` whose `x_L - x_T` are batch-inverted.
//! Only `x(T - L)` is formed before the lookup; `y` is formed on a hit only.
//!
//! Modes:
//!   fidelity  reproduce #5229's op counters and result digest
//!   ablate    time all 24 factorial arms
//!   xonly     time the five headline arms, including x-only
//!   recall    fresh 4000 random + 64 planted targets: equal hit masks across
//!             all arms plus an independent affine 3+2 brute force
//!   invbench  ns per inversion for the three inversion routines
//!
//! Usage:
//!   norm-one-torus-ablation --manifest PATH MODE [--rounds N] [--svg PATH]
//!       [--results-csv PATH --certificates PATH]   (fidelity mode)
//!
//! This is a standalone crate: the root library cannot be type-checked on a
//! 7 GB host. `fixture.rs` extracts the fixture contract; `fidelity` proves the
//! extraction against the library-produced public-precommit records.

mod fixture;
use fixture as common;

use std::collections::HashMap;
use std::hash::{BuildHasher, BuildHasherDefault, Hasher};
use std::time::{Duration, Instant};

use fixture::{result_digest, verify_results, Fixture, PrimePoint};

// ---------------------------------------------------------------- field
#[inline]
fn mul_mod(left: u64, right: u64, modulus: u64) -> u64 {
    if modulus == 2_147_483_647 {
        let product = left * right;
        let folded = (product & modulus) + (product >> 31);
        let folded = (folded & modulus) + (folded >> 31);
        if folded >= modulus {
            folded - modulus
        } else {
            folded
        }
    } else {
        ((u128::from(left) * u128::from(right)) % u128::from(modulus)) as u64
    }
}

#[inline]
fn add_mod(left: u64, right: u64, modulus: u64) -> u64 {
    let sum = left + right;
    if sum >= modulus {
        sum - modulus
    } else {
        sum
    }
}

#[inline]
fn sub_mod(left: u64, right: u64, modulus: u64) -> u64 {
    if left >= right {
        left - right
    } else {
        left + modulus - right
    }
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut accumulator = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            accumulator = mul_mod(accumulator, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    accumulator
}

/// The PR #5229 inversion, verbatim.
fn inverse_i128(value: u64, modulus: u64) -> u64 {
    let (mut old_r, mut r) = (i128::from(value), i128::from(modulus));
    let (mut old_s, mut s) = (1i128, 0i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    assert_eq!(old_r, 1);
    old_s.rem_euclid(i128::from(modulus)) as u64
}

/// The same algorithm in native 64-bit words. Valid for moduli below 2^62.
fn inverse_i64(value: u64, modulus: u64) -> u64 {
    debug_assert!(modulus < 1 << 62);
    let (mut old_r, mut r) = (value as i64, modulus as i64);
    let (mut old_s, mut s) = (1i64, 0i64);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    assert_eq!(old_r, 1);
    old_s.rem_euclid(modulus as i64) as u64
}

fn inverse_fermat(value: u64, modulus: u64) -> u64 {
    pow_mod(value, modulus - 2, modulus)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Inversion {
    I128,
    I64,
    Fermat,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct OpCounts {
    multiplications: u64,
    squarings: u64,
    inversions: u64,
    additions: u64,
    doublings: u64,
}

struct Ops {
    counts: OpCounts,
    inversion: Inversion,
}

impl Ops {
    fn new(inversion: Inversion) -> Self {
        Self {
            counts: OpCounts::default(),
            inversion,
        }
    }

    #[inline]
    fn mul(&mut self, left: u64, right: u64, modulus: u64) -> u64 {
        self.counts.multiplications += 1;
        mul_mod(left, right, modulus)
    }

    #[inline]
    fn square(&mut self, value: u64, modulus: u64) -> u64 {
        self.counts.squarings += 1;
        mul_mod(value, value, modulus)
    }

    #[inline]
    fn inverse(&mut self, value: u64, modulus: u64) -> u64 {
        assert_ne!(value, 0);
        self.counts.inversions += 1;
        match self.inversion {
            Inversion::I128 => inverse_i128(value, modulus),
            Inversion::I64 => inverse_i64(value, modulus),
            Inversion::Fermat => inverse_fermat(value, modulus),
        }
    }
}

// ---------------------------------------------------------------- Jacobian (copied from #5229)
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Jacobian {
    x: u64,
    y: u64,
    z: u64,
}

impl Jacobian {
    const INFINITY: Self = Self { x: 0, y: 1, z: 0 };

    fn from_affine(point: PrimePoint) -> Self {
        if point.infinity {
            Self::INFINITY
        } else {
            Self {
                x: point.x,
                y: point.y,
                z: 1,
            }
        }
    }

    fn double(self, curve_a: u64, modulus: u64, ops: &mut Ops) -> Self {
        ops.counts.doublings += 1;
        if self.z == 0 || self.y == 0 {
            return Self::INFINITY;
        }
        let xx = ops.square(self.x, modulus);
        let yy = ops.square(self.y, modulus);
        let yyyy = ops.square(yy, modulus);
        let x_plus_yy = add_mod(self.x, yy, modulus);
        let mut d = sub_mod(
            sub_mod(ops.square(x_plus_yy, modulus), xx, modulus),
            yyyy,
            modulus,
        );
        d = add_mod(d, d, modulus);
        let zz = ops.square(self.z, modulus);
        let zzzz = ops.square(zz, modulus);
        let three_xx = add_mod(add_mod(xx, xx, modulus), xx, modulus);
        let e = add_mod(three_xx, ops.mul(curve_a, zzzz, modulus), modulus);
        let f = ops.square(e, modulus);
        let x3 = sub_mod(f, add_mod(d, d, modulus), modulus);
        let eight_yyyy = (0..3).fold(yyyy, |value, _| add_mod(value, value, modulus));
        let y3 = sub_mod(
            ops.mul(e, sub_mod(d, x3, modulus), modulus),
            eight_yyyy,
            modulus,
        );
        let yz = ops.mul(self.y, self.z, modulus);
        Self {
            x: x3,
            y: y3,
            z: add_mod(yz, yz, modulus),
        }
    }

    fn add_mixed(self, other: PrimePoint, curve_a: u64, modulus: u64, ops: &mut Ops) -> Self {
        ops.counts.additions += 1;
        if other.infinity {
            return self;
        }
        if self.z == 0 {
            return Self::from_affine(other);
        }
        let z1z1 = ops.square(self.z, modulus);
        let u2 = ops.mul(other.x, z1z1, modulus);
        let z1_cubed = ops.mul(self.z, z1z1, modulus);
        let s2 = ops.mul(other.y, z1_cubed, modulus);
        let h = sub_mod(u2, self.x, modulus);
        let delta = sub_mod(s2, self.y, modulus);
        if h == 0 {
            return if delta == 0 {
                self.double(curve_a, modulus, ops)
            } else {
                Self::INFINITY
            };
        }
        let hh = ops.square(h, modulus);
        let i = (0..2).fold(hh, |value, _| add_mod(value, value, modulus));
        let j = ops.mul(h, i, modulus);
        let r = add_mod(delta, delta, modulus);
        let v = ops.mul(self.x, i, modulus);
        let x3 = sub_mod(
            sub_mod(ops.square(r, modulus), j, modulus),
            add_mod(v, v, modulus),
            modulus,
        );
        let y1j = ops.mul(self.y, j, modulus);
        let y3 = sub_mod(
            ops.mul(r, sub_mod(v, x3, modulus), modulus),
            add_mod(y1j, y1j, modulus),
            modulus,
        );
        let z_plus_h = add_mod(self.z, h, modulus);
        let z3 = sub_mod(
            sub_mod(ops.square(z_plus_h, modulus), z1z1, modulus),
            hh,
            modulus,
        );
        Self {
            x: x3,
            y: y3,
            z: z3,
        }
    }

    fn to_affine(self, modulus: u64, ops: &mut Ops) -> PrimePoint {
        if self.z == 0 {
            return PrimePoint::INFINITY;
        }
        let inverse_z = ops.inverse(self.z, modulus);
        from_inverse_z(self, inverse_z, modulus, ops)
    }
}

fn from_inverse_z(point: Jacobian, inverse_z: u64, modulus: u64, ops: &mut Ops) -> PrimePoint {
    let inverse_z2 = ops.square(inverse_z, modulus);
    let x = ops.mul(point.x, inverse_z2, modulus);
    let inverse_z3 = ops.mul(inverse_z2, inverse_z, modulus);
    let y = ops.mul(point.y, inverse_z3, modulus);
    PrimePoint::affine(x, y)
}

fn batch_invert(values: &mut [u64], modulus: u64, ops: &mut Ops) {
    match values.len() {
        0 => return,
        1 => {
            values[0] = ops.inverse(values[0], modulus);
            return;
        }
        _ => {}
    }
    let mut prefixes = Vec::with_capacity(values.len());
    prefixes.push(values[0]);
    for &value in &values[1..] {
        prefixes.push(ops.mul(*prefixes.last().unwrap(), value, modulus));
    }
    let mut inverse = ops.inverse(*prefixes.last().unwrap(), modulus);
    for index in (1..values.len()).rev() {
        let value = values[index];
        values[index] = ops.mul(inverse, prefixes[index - 1], modulus);
        inverse = ops.mul(inverse, value, modulus);
    }
    values[0] = inverse;
}

fn normalize_batch<W: Copy>(
    states: &[(Jacobian, W)],
    modulus: u64,
    ops: &mut Ops,
) -> Vec<(PrimePoint, W)> {
    let finite: Vec<usize> = states
        .iter()
        .enumerate()
        .filter_map(|(index, (point, _))| (point.z != 0).then_some(index))
        .collect();
    let mut inverses: Vec<u64> = finite.iter().map(|&index| states[index].0.z).collect();
    batch_invert(&mut inverses, modulus, ops);
    let mut inverse_at = vec![0u64; states.len()];
    for (&index, inverse) in finite.iter().zip(inverses) {
        inverse_at[index] = inverse;
    }
    states
        .iter()
        .enumerate()
        .map(|(index, &(point, witness))| {
            let affine = if point.z == 0 {
                PrimePoint::INFINITY
            } else {
                from_inverse_z(point, inverse_at[index], modulus, ops)
            };
            (affine, witness)
        })
        .collect()
}

fn normalize_individual<W: Copy>(
    states: &[(Jacobian, W)],
    modulus: u64,
    ops: &mut Ops,
) -> Vec<(PrimePoint, W)> {
    states
        .iter()
        .map(|&(point, witness)| (point.to_affine(modulus, ops), witness))
        .collect()
}

fn pair_states(fixture: &Fixture, ops: &mut Ops) -> Vec<(Jacobian, [u16; 2])> {
    let curve = &fixture.instance.curve;
    let mut states = Vec::new();
    for first in 0..fixture.base.len() {
        let left = Jacobian::from_affine(fixture.base[first]);
        for second in first..fixture.base.len() {
            states.push((
                left.add_mixed(fixture.base[second], curve.a, curve.p, ops),
                [first as u16, second as u16],
            ));
        }
    }
    states
}

fn negated_witness(mut witness: [u16; 3], neg_index: &[usize]) -> [u16; 3] {
    for index in &mut witness {
        *index = neg_index[usize::from(*index)] as u16;
    }
    witness.sort_unstable();
    witness
}

fn triple_states(fixture: &Fixture, quotient: bool, ops: &mut Ops) -> Vec<(Jacobian, [u16; 3])> {
    let curve = &fixture.instance.curve;
    let mut states = Vec::new();
    for first in 0..fixture.base.len() {
        let left = Jacobian::from_affine(fixture.base[first]);
        for second in first..fixture.base.len() {
            let pair = left.add_mixed(fixture.base[second], curve.a, curve.p, ops);
            for third in second..fixture.base.len() {
                let witness = [first as u16, second as u16, third as u16];
                if quotient && witness > negated_witness(witness, &fixture.neg_index) {
                    continue;
                }
                states.push((
                    pair.add_mixed(fixture.base[third], curve.a, curve.p, ops),
                    witness,
                ));
            }
        }
    }
    states
}

fn negate(point: PrimePoint, modulus: u64) -> PrimePoint {
    if point.infinity || point.y == 0 {
        point
    } else {
        PrimePoint::affine(point.x, modulus - point.y)
    }
}

// ---------------------------------------------------------------- hashing
/// Fx-style multiplicative hasher (the rustc-hash constant); no new dependency.
#[derive(Clone, Copy, Default)]
struct FxHasher(u64);

impl FxHasher {
    #[inline]
    fn add(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.add(u64::from(byte));
        }
    }
    #[inline]
    fn write_u8(&mut self, value: u8) {
        self.add(u64::from(value));
    }
    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.add(value);
    }
    #[inline]
    fn write_usize(&mut self, value: usize) {
        self.add(value as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

type FxBuild = BuildHasherDefault<FxHasher>;
type SipBuild = std::collections::hash_map::RandomState;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PointKey {
    x: u64,
    y: u64,
    infinity: bool,
}

impl From<PrimePoint> for PointKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            y: point.y,
            infinity: point.infinity,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct XKey {
    x: u64,
    infinity: bool,
}

impl From<PrimePoint> for XKey {
    fn from(point: PrimePoint) -> Self {
        Self {
            x: point.x,
            infinity: point.infinity,
        }
    }
}

// ---------------------------------------------------------------- solver
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Query {
    Jacobian,
    AffineXOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Config {
    batch: bool,
    quotient: bool,
    inversion: Inversion,
    fx_hash: bool,
    query: Query,
}

impl Config {
    const PR_CONTROL: Self = Self {
        batch: false,
        quotient: false,
        inversion: Inversion::I128,
        fx_hash: false,
        query: Query::Jacobian,
    };
    const PR_CANDIDATE: Self = Self {
        batch: true,
        quotient: true,
        inversion: Inversion::I128,
        fx_hash: false,
        query: Query::Jacobian,
    };
    const FAIR_CONTROL: Self = Self {
        batch: false,
        quotient: false,
        inversion: Inversion::I64,
        fx_hash: true,
        query: Query::Jacobian,
    };
    const BEST_PR_STYLE: Self = Self {
        batch: true,
        quotient: true,
        inversion: Inversion::I64,
        fx_hash: true,
        query: Query::Jacobian,
    };
    const X_ONLY: Self = Self {
        batch: true,
        quotient: true,
        inversion: Inversion::I64,
        fx_hash: true,
        query: Query::AffineXOnly,
    };

    fn label(&self) -> String {
        format!(
            "{}+{}+{}+{}{}",
            if self.batch { "batch" } else { "indiv" },
            if self.quotient { "quot" } else { "full" },
            match self.inversion {
                Inversion::I128 => "inv128",
                Inversion::I64 => "inv64",
                Inversion::Fermat => "fermat",
            },
            if self.fx_hash { "fx" } else { "sip" },
            match self.query {
                Query::Jacobian => "",
                Query::AffineXOnly => "+xonly",
            },
        )
    }

    fn factorial() -> Vec<Self> {
        let mut configs = Vec::new();
        for inversion in [Inversion::I128, Inversion::I64, Inversion::Fermat] {
            for fx_hash in [false, true] {
                for batch in [false, true] {
                    for quotient in [false, true] {
                        configs.push(Self {
                            batch,
                            quotient,
                            inversion,
                            fx_hash,
                            query: Query::Jacobian,
                        });
                    }
                }
            }
        }
        configs
    }
}

enum RightTable<S> {
    Full(HashMap<PointKey, [u16; 3], S>),
    SignedQuotient(HashMap<XKey, (PrimePoint, [u16; 3]), S>),
}

struct Solver<S> {
    left: Vec<(PrimePoint, [u16; 2])>,
    right: RightTable<S>,
    config: Config,
    neg_index: Vec<usize>,
    curve_a: u64,
    modulus: u64,
}

type Witness = Option<Vec<usize>>;

fn joined(left: [u16; 2], right: [u16; 3]) -> Witness {
    Some(left.into_iter().chain(right).map(usize::from).collect())
}

impl<S: BuildHasher + Default> Solver<S> {
    fn new(fixture: &Fixture, config: Config) -> (Self, OpCounts) {
        assert!(config.query == Query::Jacobian || (config.batch && config.quotient));
        let modulus = fixture.instance.curve.p;
        let mut ops = Ops::new(config.inversion);
        let left_states = pair_states(fixture, &mut ops);
        let right_states = triple_states(fixture, config.quotient, &mut ops);
        let (left, right_points) = if config.batch {
            (
                normalize_batch(&left_states, modulus, &mut ops),
                normalize_batch(&right_states, modulus, &mut ops),
            )
        } else {
            (
                normalize_individual(&left_states, modulus, &mut ops),
                normalize_individual(&right_states, modulus, &mut ops),
            )
        };
        let right = if config.quotient {
            let mut table = HashMap::with_capacity_and_hasher(right_points.len(), S::default());
            for (point, witness) in right_points {
                table.entry(point.into()).or_insert((point, witness));
            }
            RightTable::SignedQuotient(table)
        } else {
            let mut table = HashMap::with_capacity_and_hasher(right_points.len(), S::default());
            for (point, witness) in right_points {
                table.entry(point.into()).or_insert(witness);
            }
            RightTable::Full(table)
        };
        (
            Self {
                left,
                right,
                config,
                neg_index: fixture.neg_index.clone(),
                curve_a: fixture.instance.curve.a,
                modulus,
            },
            ops.counts,
        )
    }

    fn lookup(&self, point: PrimePoint) -> Option<[u16; 3]> {
        match &self.right {
            RightTable::Full(table) => table.get(&point.into()).copied(),
            RightTable::SignedQuotient(table) => {
                table.get(&point.into()).map(|&(stored, witness)| {
                    if point == stored {
                        witness
                    } else {
                        assert_eq!(point, negate(stored, self.modulus));
                        negated_witness(witness, &self.neg_index)
                    }
                })
            }
        }
    }

    fn complement(&self, target: PrimePoint, left: PrimePoint, ops: &mut Ops) -> Jacobian {
        Jacobian::from_affine(target).add_mixed(
            negate(left, self.modulus),
            self.curve_a,
            self.modulus,
            ops,
        )
    }

    fn decomposition(&self, target: PrimePoint, block_size: usize, ops: &mut Ops) -> Witness {
        match (self.config.query, self.config.batch) {
            (Query::AffineXOnly, _) => self.decomposition_x_only(target, block_size, ops),
            (Query::Jacobian, false) => {
                for &(left, left_witness) in &self.left {
                    let point = self
                        .complement(target, left, ops)
                        .to_affine(self.modulus, ops);
                    if let Some(right_witness) = self.lookup(point) {
                        return joined(left_witness, right_witness);
                    }
                }
                None
            }
            (Query::Jacobian, true) => {
                for block in self.left.chunks(block_size) {
                    let complements: Vec<_> = block
                        .iter()
                        .map(|&(left, witness)| (self.complement(target, left, ops), witness))
                        .collect();
                    for (point, left_witness) in normalize_batch(&complements, self.modulus, ops) {
                        if let Some(right_witness) = self.lookup(point) {
                            return joined(left_witness, right_witness);
                        }
                    }
                }
                None
            }
        }
    }

    /// Affine `T + (-L)` with one batched inversion of `x_L - x_T` per block.
    /// The quotient table is keyed by x, so `y` is formed only on a hit.
    fn decomposition_x_only(&self, target: PrimePoint, block_size: usize, ops: &mut Ops) -> Witness {
        let RightTable::SignedQuotient(table) = &self.right else {
            unreachable!("x-only query requires the signed quotient table")
        };
        let modulus = self.modulus;
        let mut denominators = Vec::with_capacity(block_size);
        for block in self.left.chunks(block_size) {
            denominators.clear();
            let mut exceptional = target.infinity;
            for &(left, _) in block {
                let dx = if left.infinity {
                    1
                } else {
                    sub_mod(left.x, target.x, modulus)
                };
                exceptional |= dx == 0;
                denominators.push(dx.max(1));
            }
            if exceptional {
                // x_L == x_T (T = ±L) or T = O: use the Jacobian path for this block.
                for &(left, left_witness) in block {
                    let point = self.complement(target, left, ops).to_affine(modulus, ops);
                    if let Some(right_witness) = self.lookup(point) {
                        return joined(left_witness, right_witness);
                    }
                }
                continue;
            }
            batch_invert(&mut denominators, modulus, ops);
            for (index, &(left, left_witness)) in block.iter().enumerate() {
                if left.infinity {
                    if let Some(right_witness) = self.lookup(target) {
                        return joined(left_witness, right_witness);
                    }
                    continue;
                }
                let numerator = sub_mod(0, add_mod(left.y, target.y, modulus), modulus);
                let lambda = ops.mul(numerator, denominators[index], modulus);
                let x3 = sub_mod(
                    sub_mod(ops.square(lambda, modulus), target.x, modulus),
                    left.x,
                    modulus,
                );
                let key = XKey {
                    x: x3,
                    infinity: false,
                };
                if let Some(&(stored, witness)) = table.get(&key) {
                    let y3 = sub_mod(
                        ops.mul(lambda, sub_mod(target.x, x3, modulus), modulus),
                        target.y,
                        modulus,
                    );
                    let point = PrimePoint::affine(x3, y3);
                    let right_witness = if point == stored {
                        witness
                    } else {
                        assert_eq!(point, negate(stored, modulus));
                        negated_witness(witness, &self.neg_index)
                    };
                    return joined(left_witness, right_witness);
                }
            }
        }
        None
    }
}

struct TimedRun {
    setup: Duration,
    query: Duration,
    setup_ops: OpCounts,
    query_ops: OpCounts,
    results: Vec<Witness>,
}

impl TimedRun {
    fn total_ms(&self) -> f64 {
        (self.setup + self.query).as_secs_f64() * 1e3
    }
    fn mask(&self) -> Vec<bool> {
        self.results.iter().map(Option::is_some).collect()
    }
}

fn run_with<S: BuildHasher + Default>(fixture: &Fixture, config: Config, block: usize) -> TimedRun {
    let started = Instant::now();
    let (solver, setup_ops) = Solver::<S>::new(fixture, config);
    let setup = started.elapsed();
    let started = Instant::now();
    let mut ops = Ops::new(config.inversion);
    let results: Vec<_> = fixture
        .targets
        .iter()
        .map(|&target| solver.decomposition(target, block, &mut ops))
        .collect();
    let query = started.elapsed();
    std::hint::black_box(&results);
    TimedRun {
        setup,
        query,
        setup_ops,
        query_ops: ops.counts,
        results,
    }
}

fn run(fixture: &Fixture, config: Config, block: usize) -> TimedRun {
    if config.fx_hash {
        run_with::<FxBuild>(fixture, config, block)
    } else {
        run_with::<SipBuild>(fixture, config, block)
    }
}

// ---------------------------------------------------------------- statistics
fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        0.5 * (sorted[n / 2 - 1] + sorted[n / 2])
    }
}

/// Deterministic LCG for shuffling and bootstrap; the stream is fixed so the
/// run order and intervals are reproducible from the code alone.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as usize
    }
    fn shuffle(&mut self, values: &mut [usize]) {
        for index in (1..values.len()).rev() {
            let other = self.next() % (index + 1);
            values.swap(index, other);
        }
    }
}

/// median(reference) / median(arm), with a round-paired bootstrap 95% interval.
fn ratio_ci(reference: &[f64], arm: &[f64], rng: &mut Lcg) -> (f64, f64, f64) {
    let point = median(reference) / median(arm);
    let n = reference.len();
    let mut ratios: Vec<f64> = (0..2000)
        .map(|_| {
            let picks: Vec<usize> = (0..n).map(|_| rng.next() % n).collect();
            let r: Vec<f64> = picks.iter().map(|&i| reference[i]).collect();
            let a: Vec<f64> = picks.iter().map(|&i| arm[i]).collect();
            median(&r) / median(&a)
        })
        .collect();
    ratios.sort_by(f64::total_cmp);
    (point, ratios[50], ratios[1949])
}

struct Samples {
    setup: Vec<f64>,
    query: Vec<f64>,
    total: Vec<f64>,
}

fn time_arms(fixture: &Fixture, configs: &[Config], rounds: usize, block: usize, seed: u64) -> Vec<Samples> {
    let mut samples: Vec<Samples> = configs
        .iter()
        .map(|_| Samples {
            setup: Vec::new(),
            query: Vec::new(),
            total: Vec::new(),
        })
        .collect();
    // one warm-up per arm
    for &config in configs {
        run(fixture, config, block);
    }
    let mut rng = Lcg(seed);
    for _ in 0..rounds {
        let mut order: Vec<usize> = (0..configs.len()).collect();
        rng.shuffle(&mut order);
        for index in order {
            let timed = run(fixture, configs[index], block);
            samples[index].setup.push(timed.setup.as_secs_f64() * 1e3);
            samples[index].query.push(timed.query.as_secs_f64() * 1e3);
            samples[index].total.push(timed.total_ms());
        }
    }
    samples
}

fn check_all(fixture: &Fixture, configs: &[Config], block: usize) -> Vec<bool> {
    let reference = run(fixture, Config::PR_CONTROL, block);
    assert!(verify_results(fixture, &reference.results));
    for &config in configs {
        let timed = run(fixture, config, block);
        assert!(verify_results(fixture, &timed.results), "{}", config.label());
        assert_eq!(timed.mask(), reference.mask(), "{}", config.label());
    }
    reference.mask()
}

// ---------------------------------------------------------------- modes
/// Reproduce the library-produced public-precommit records exactly.
fn fidelity(fixture: &Fixture, block: usize, manifest_bytes: &[u8], results_csv: Option<&str>, certificates: Option<&str>) {
    let control = run(fixture, Config::PR_CONTROL, block);
    let candidate = run(fixture, Config::PR_CANDIDATE, block);
    if let Some(path) = results_csv {
        let text = std::fs::read_to_string(path).expect("read results.csv");
        let mut lines = text.lines();
        let header: Vec<&str> = lines.next().unwrap().split(',').collect();
        let col = |name: &str| header.iter().position(|h| *h == name).unwrap();
        let mut checked = 0;
        for line in lines {
            let f: Vec<&str> = line.split(',').collect();
            let timed = if f[col("implementation")] == "individual" { &control } else { &candidate };
            let expect = [
                ("setup_inversions", timed.setup_ops.inversions),
                ("query_inversions", timed.query_ops.inversions),
                ("setup_multiplications", timed.setup_ops.multiplications),
                ("query_multiplications", timed.query_ops.multiplications),
                ("setup_squarings", timed.setup_ops.squarings),
                ("query_squarings", timed.query_ops.squarings),
                ("setup_additions", timed.setup_ops.additions),
                ("query_additions", timed.query_ops.additions),
            ];
            for (name, value) in expect {
                assert_eq!(f[col(name)].parse::<u64>().unwrap(), value, "{name} in {line}");
            }
            assert_eq!(f[col("result_digest")], result_digest(&timed.results));
            checked += 1;
        }
        println!("results_csv_rows_matched={checked} (all 8 op counters + result digest per row)");
    }
    if let Some(path) = certificates {
        let bundle = fixture::read_certificates(path);
        assert_eq!(bundle.manifest_sha256, fixture::sha256_hex(manifest_bytes));
        assert_eq!(bundle.factor_base_points, fixture.base.len());
        assert_eq!(bundle.factor_base_abscissae, fixture.abscissae);
        assert_eq!(bundle.result_digest, result_digest(&candidate.results));
        assert_eq!(bundle.targets.len(), fixture.targets.len());
        for ((cert, &target), witness) in bundle.targets.iter().zip(&fixture.targets).zip(&candidate.results) {
            assert_eq!((cert.target.x, cert.target.y, cert.target.infinity), (target.x, target.y, target.infinity));
            assert_eq!(&cert.witness, witness);
        }
        println!("certificates_matched={} targets (points, witnesses, digest, manifest hash)", bundle.targets.len());
    }

    println!("factor_base_points={} abscissae={} targets={}", fixture.base.len(), fixture.abscissae, fixture.targets.len());
    println!("arm,setup_inversions,query_inversions,setup_multiplications,query_multiplications,setup_squarings,query_squarings,setup_additions,query_additions,successes,result_digest");
    for config in [Config::PR_CONTROL, Config::PR_CANDIDATE, Config::X_ONLY] {
        let timed = run(fixture, config, block);
        assert!(verify_results(fixture, &timed.results));
        println!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            config.label(),
            timed.setup_ops.inversions,
            timed.query_ops.inversions,
            timed.setup_ops.multiplications,
            timed.query_ops.multiplications,
            timed.setup_ops.squarings,
            timed.query_ops.squarings,
            timed.setup_ops.additions,
            timed.query_ops.additions,
            timed.results.iter().filter(|r| r.is_some()).count(),
            result_digest(&timed.results),
        );
    }
}

fn ablate(fixture: &Fixture, rounds: usize, block: usize) {
    let configs = Config::factorial();
    assert_eq!(configs[0], Config::PR_CONTROL);
    check_all(fixture, &configs, block);
    let samples = time_arms(fixture, &configs, rounds, block, 0x5229);
    let mut rng = Lcg(0x5262);
    println!("arm,setup_median_ms,query_median_ms,total_median_ms,ratio_vs_pr_control,ci95_low,ci95_high,setup_inversions,query_inversions,query_multiplications");
    for (index, config) in configs.iter().enumerate() {
        let (ratio, low, high) = ratio_ci(&samples[0].total, &samples[index].total, &mut rng);
        let counts = run(fixture, *config, block);
        println!(
            "{},{:.3},{:.3},{:.3},{ratio:.3},{low:.3},{high:.3},{},{},{}",
            config.label(),
            median(&samples[index].setup),
            median(&samples[index].query),
            median(&samples[index].total),
            counts.setup_ops.inversions,
            counts.query_ops.inversions,
            counts.query_ops.multiplications,
        );
    }
    println!("# raw total_ms per round");
    for (index, config) in configs.iter().enumerate() {
        let raw: Vec<String> = samples[index].total.iter().map(|v| format!("{v:.3}")).collect();
        println!("# {},{}", config.label(), raw.join(","));
    }
}

const HEADLINE: [(&str, Config); 5] = [
    ("PR #5229 control", Config::PR_CONTROL),
    ("Fair control (i64 inv + Fx)", Config::FAIR_CONTROL),
    ("PR #5229 candidate", Config::PR_CANDIDATE),
    ("PR candidate + i64 inv + Fx", Config::BEST_PR_STYLE),
    ("Affine x-only query", Config::X_ONLY),
];

fn headline(fixture: &Fixture, rounds: usize, block: usize, svg: Option<&str>) {
    let configs: Vec<Config> = HEADLINE.iter().map(|(_, c)| *c).collect();
    check_all(fixture, &configs, block);
    let digest = result_digest(&run(fixture, Config::PR_CANDIDATE, block).results);
    assert_eq!(digest, result_digest(&run(fixture, Config::X_ONLY, block).results));
    let samples = time_arms(fixture, &configs, rounds, block, 0xabc);
    let mut rng = Lcg(0x5262);
    let mut rows = Vec::new();
    println!("arm,config,setup_median_ms,query_median_ms,total_median_ms,ratio_vs_pr_control,ci95_low,ci95_high,ratio_vs_fair_control,ci95_low,ci95_high,query_multiplications,query_squarings,query_inversions");
    for (index, (name, config)) in HEADLINE.iter().enumerate() {
        let pr = ratio_ci(&samples[0].total, &samples[index].total, &mut rng);
        let fair = ratio_ci(&samples[1].total, &samples[index].total, &mut rng);
        let counts = run(fixture, *config, block);
        println!(
            "{name},{},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{},{},{}",
            config.label(),
            median(&samples[index].setup),
            median(&samples[index].query),
            median(&samples[index].total),
            pr.0, pr.1, pr.2, fair.0, fair.1, fair.2,
            counts.query_ops.multiplications,
            counts.query_ops.squarings,
            counts.query_ops.inversions,
        );
        rows.push((name.to_string(), median(&samples[index].setup), median(&samples[index].query), fair));
    }
    println!("result_digest={digest}");
    if let Some(path) = svg {
        std::fs::write(path, render_svg(&rows)).expect("write svg");
    }
}

/// Stacked setup/query bars with the ratio against the fair control.
fn render_svg(rows: &[(String, f64, f64, (f64, f64, f64))]) -> String {
    let width = 760.0;
    let left = 230.0;
    let bar_h = 34.0;
    let gap = 18.0;
    let top = 64.0;
    let max_ms = rows.iter().map(|r| r.1 + r.2).fold(0.0, f64::max) * 1.08;
    let scale = (width - left - 150.0) / max_ms;
    let height = top + rows.len() as f64 * (bar_h + gap) + 70.0;
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\" font-family=\"Helvetica, Arial, sans-serif\">\n<rect width=\"100%\" height=\"100%\" fill=\"white\"/>\n<text x=\"20\" y=\"28\" font-size=\"17\" font-weight=\"bold\">Prime31 norm-one-torus m=5 MITM: median setup + query time (40 targets)</text>\n<text x=\"20\" y=\"48\" font-size=\"12\" fill=\"#555\">Local non-isolated diagnostic, 1 pinned core, randomized order. Ratios vs fair control, paired bootstrap 95% CI. [pcc:unverified]</text>\n"
    );
    for (index, (name, setup, query, (ratio, low, high))) in rows.iter().enumerate() {
        let y = top + index as f64 * (bar_h + gap);
        let w_setup = setup * scale;
        let w_query = query * scale;
        s += &format!(
            "<text x=\"{}\" y=\"{}\" font-size=\"13\" text-anchor=\"end\">{name}</text>\n<rect x=\"{left}\" y=\"{y}\" width=\"{w_setup:.1}\" height=\"{bar_h}\" fill=\"#4C78A8\"/>\n<rect x=\"{:.1}\" y=\"{y}\" width=\"{w_query:.1}\" height=\"{bar_h}\" fill=\"#F58518\"/>\n<text x=\"{:.1}\" y=\"{}\" font-size=\"12\">{:.1} ms · {ratio:.2}× [{low:.2}–{high:.2}]</text>\n",
            left - 10.0,
            y + bar_h * 0.65,
            left + w_setup,
            left + w_setup + w_query + 8.0,
            y + bar_h * 0.65,
            setup + query,
        );
    }
    let axis_y = top + rows.len() as f64 * (bar_h + gap);
    s += &format!(
        "<rect x=\"{left}\" y=\"{}\" width=\"14\" height=\"14\" fill=\"#4C78A8\"/><text x=\"{}\" y=\"{}\" font-size=\"12\">setup (tables)</text>\n<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"14\" fill=\"#F58518\"/><text x=\"{}\" y=\"{}\" font-size=\"12\">query (40 targets)</text>\n</svg>\n",
        axis_y + 6.0,
        left + 20.0,
        axis_y + 18.0,
        left + 140.0,
        axis_y + 6.0,
        left + 160.0,
        axis_y + 18.0,
    );
    s
}

fn recall(manifest: &common::Manifest, random: usize, planted: usize, block: usize) {
    let mut fresh = manifest.clone();
    fresh.stream_derivation.domain = "isogeny-labs/norm-one-torus-mitm/ablation-recall/2026-11-27/v1".into();
    fresh.configuration.random_targets = random;
    fresh.configuration.planted_targets = planted;
    let fixture = common::load_fixture(&fresh);
    let mut configs = Config::factorial();
    configs.push(Config::X_ONLY);
    let mask = check_all(&fixture, &configs, block);
    let random_hits = mask[..random].iter().filter(|&&b| b).count();
    let planted_hits = mask[random..].iter().filter(|&&b| b).count();
    println!("domain={}", fresh.stream_derivation.domain);
    println!("targets random={random} planted={planted}; hits random={random_hits} planted={planted_hits}");
    println!("arms_checked={} all_masks_equal=true all_witnesses_replayed=true", configs.len());

    // Independent brute force with the library's affine group law.
    let curve = &fixture.instance.curve;
    let add = |a: PrimePoint, b: PrimePoint| curve.add(a, b);
    let base = &fixture.base;
    let mut pairs: HashMap<PointKey, (), FxBuild> = HashMap::default();
    for i in 0..base.len() {
        for j in i..base.len() {
            pairs.insert(add(base[i], base[j]).into(), ());
        }
    }
    let mut triples = Vec::new();
    for i in 0..base.len() {
        for j in i..base.len() {
            let s = add(base[i], base[j]);
            for k in j..base.len() {
                triples.push(add(s, base[k]));
            }
        }
    }
    let mut decomposable = 0;
    let mut mismatches = 0;
    for (index, &target) in fixture.targets.iter().enumerate() {
        let found = triples
            .iter()
            .any(|&t| pairs.contains_key(&add(target, curve.neg(t)).into()));
        decomposable += usize::from(found);
        mismatches += usize::from(found != mask[index]);
    }
    println!("brute_force_affine_3plus2 decomposable={decomposable} mismatches={mismatches}");
    assert_eq!(mismatches, 0);
}

fn invbench() {
    let modulus = 2_147_483_647u64;
    let n = 200_000u64;
    let values: Vec<u64> = (1..=n).map(|i| mul_mod(i, 1_234_567, modulus).max(1)).collect();
    for &v in values.iter().take(2000) {
        let reference = inverse_i128(v, modulus);
        assert_eq!(reference, inverse_i64(v, modulus));
        assert_eq!(reference, inverse_fermat(v, modulus));
    }
    let routines: [(&str, fn(u64, u64) -> u64); 3] = [
        ("i128_eea", inverse_i128),
        ("i64_eea", inverse_i64),
        ("fermat", inverse_fermat),
    ];
    println!("routine,best_ns_per_inversion");
    for (name, routine) in routines {
        let mut best = f64::MAX;
        for _ in 0..7 {
            let started = Instant::now();
            let mut acc = 0u64;
            for &v in &values {
                acc ^= routine(std::hint::black_box(v), modulus);
            }
            std::hint::black_box(acc);
            best = best.min(started.elapsed().as_secs_f64() * 1e9 / n as f64);
        }
        println!("{name},{best:.1}");
    }
    let mut best = f64::MAX;
    for _ in 0..7 {
        let started = Instant::now();
        let mut acc = 1u64;
        for &v in &values {
            acc = mul_mod(acc, std::hint::black_box(v), modulus);
        }
        std::hint::black_box(acc);
        best = best.min(started.elapsed().as_secs_f64() * 1e9 / n as f64);
    }
    println!("mersenne_mul_dependent_chain,{best:.2}");
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let usage = "usage: norm-one-torus-ablation --manifest PATH (fidelity|ablate|xonly|recall|invbench) [--rounds N] [--svg PATH] [--results-csv PATH] [--certificates PATH]";
    assert!(arguments.len() >= 4 && arguments[1] == "--manifest", "{usage}");
    let (manifest, manifest_bytes) = common::read_manifest(&arguments[2]);
    let mode = arguments[3].as_str();
    let mut rounds = 41;
    let mut svg = None;
    let mut results_csv = None;
    let mut certificates = None;
    let mut index = 4;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--rounds" => rounds = arguments[index + 1].parse().expect("rounds"),
            "--svg" => svg = Some(arguments[index + 1].clone()),
            "--results-csv" => results_csv = Some(arguments[index + 1].clone()),
            "--certificates" => certificates = Some(arguments[index + 1].clone()),
            other => panic!("unknown argument {other}; {usage}"),
        }
        index += 2;
    }
    let block = manifest.configuration.query_block_size;
    match mode {
        "fidelity" => fidelity(
            &common::load_fixture(&manifest),
            block,
            &manifest_bytes,
            results_csv.as_deref(),
            certificates.as_deref(),
        ),
        "ablate" => ablate(&common::load_fixture(&manifest), rounds, block),
        "xonly" => headline(&common::load_fixture(&manifest), rounds, block, svg.as_deref()),
        "recall" => recall(&manifest, 4000, 64, block),
        "invbench" => invbench(),
        _ => panic!("{usage}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::{dickson_factor_base, fp_p31_subexp_mersenne_instance};

    fn fixture_with_targets(count: usize) -> Fixture {
        let instance = fp_p31_subexp_mersenne_instance();
        let factor_base = dickson_factor_base(&instance, 6, 0, 0);
        let generator = instance.generator_point();
        let mut targets: Vec<_> = (1..=count as u64)
            .map(|k| instance.curve.mul(generator, k * 7_919 + 3))
            .collect();
        // planted: sums of five base points, including a repeated index
        for witness in [[0usize, 3, 9, 20, 41], [1, 1, 5, 30, 66]] {
            targets.push(witness.iter().fold(instance.curve.identity(), |sum, &i| {
                instance.curve.add(sum, factor_base.points[i])
            }));
        }
        Fixture {
            instance,
            base: factor_base.points,
            neg_index: factor_base.neg_index,
            abscissae: factor_base.abscissae,
            targets,
        }
    }

    #[test]
    fn inversions_agree() {
        let modulus = 2_147_483_647;
        for v in [1u64, 2, 3, 65_537, 1_234_567_890, modulus - 1] {
            let reference = inverse_i128(v, modulus);
            assert_eq!(mul_mod(reference, v, modulus), 1);
            assert_eq!(reference, inverse_i64(v, modulus));
            assert_eq!(reference, inverse_fermat(v, modulus));
        }
    }

    #[test]
    fn every_arm_matches_the_pr_control() {
        let fixture = fixture_with_targets(24);
        let mut configs = Config::factorial();
        configs.push(Config::X_ONLY);
        let mask = check_all(&fixture, &configs, 512);
        assert!(mask[24..].iter().all(|&hit| hit), "planted targets decompose");
        for block in [1, 7, 512, 4096] {
            let timed = run(&fixture, Config::X_ONLY, block);
            assert!(verify_results(&fixture, &timed.results));
            assert_eq!(timed.mask(), mask);
        }
    }
}
