//! Reusable CPU decomposition for the independently verified paper reduction.
//! Generic meet-in-the-middle enumeration, not Boolean tensor contraction.
#[path = "../../../src/cryptanalysis/hashed_ecdl.rs"]
pub mod paper;
use paper::{Curve, DecompositionOracle, Error, Point};
mod certificate;
mod checked_sum;
pub use certificate::Archive;

/// Cumulative structural counters, including discarded trials and preparation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    pub preparations: u64,
    pub queries: u64,
    pub built_rows: u64,
    pub probed_rows: u64,
    pub query_inversions: u64,
    /// Retained sum-table capacities only; excludes bases, curve, scratch and
    /// transient allocations during preparation. This is not peak RSS.
    pub peak_table_bytes: usize,
}

struct Prepared {
    curve: [u64; 4],
    bases: Vec<Vec<Point>>,
    split: usize,
    prefix: Vec<Point>,
    suffix: Suffix,
}

// Point compression is injective on the validated odd-prime curve. Keep
// identity separate from every compressed finite point (p < 2^62).
fn point_key(point: Point) -> u64 {
    point.map_or(u64::MAX - 1, |(x, y)| 2 * x + (y & 1))
}
fn point_bucket(mut key: u64, mask: usize) -> usize {
    key ^= key >> 30;
    key = key.wrapping_mul(0xbf58476d1ce4e5b9);
    key ^= key >> 27;
    key = key.wrapping_mul(0x94d049bb133111eb);
    ((key ^ (key >> 31)) as usize) & mask
}
struct Suffix {
    slots: Vec<(u64, usize)>,
    rows: usize,
    unique: usize,
}
impl Suffix {
    fn new(points: Vec<Point>) -> Result<Self, Error> {
        if points.is_empty() {
            return Ok(Self {
                slots: vec![],
                rows: 0,
                unique: 0,
            });
        }
        let size = points
            .len()
            .checked_mul(2)
            .and_then(usize::checked_next_power_of_two)
            .ok_or(Error::Limit("CPU suffix index capacity"))?;
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(size)
            .map_err(|_| Error::Limit("CPU suffix index allocation"))?;
        slots.resize(size, (u64::MAX, 0));
        let mut out = Self {
            slots,
            rows: points.len(),
            unique: 0,
        };
        // Tuple order is lexicographic: retain the first ID on duplicate sums.
        for (id, point) in points.into_iter().enumerate() {
            let key = point_key(point);
            let mut i = point_bucket(key, size - 1);
            loop {
                let entry = &mut out.slots[i];
                if entry.0 == key {
                    break;
                }
                if entry.0 == u64::MAX {
                    *entry = (key, id);
                    out.unique += 1;
                    break;
                }
                i = (i + 1) & (size - 1);
            }
        }
        Ok(out)
    }
    fn find(&self, point: Point) -> Option<usize> {
        if self.slots.is_empty() {
            return None;
        }
        let key = point_key(point);
        let mask = self.slots.len() - 1;
        let mut i = point_bucket(key, mask);
        loop {
            let (stored, id) = self.slots[i];
            if stored == key {
                return Some(id);
            }
            if stored == u64::MAX {
                return None;
            }
            i = (i + 1) & mask;
        }
    }
}

/// An exact CPU oracle retaining both tuple-sum tables across targets.
///
/// Prefix tuples are visited in lexicographic order. Each equal suffix sum
/// retains its smallest tuple ID in a compressed-point index. Therefore the first returned full tuple is
/// identical to the original `paper::Enumerate`, including duplicate sums.
pub struct CachedMitm {
    max_rows: usize,
    batch: usize,
    prepared: Option<Prepared>,
    stats: Stats,
    scratch: Scratch,
}

impl CachedMitm {
    /// The combined prefix/suffix row cap is explicit; no input changes it.
    /// `batch=1` is an independent per-residual inversion control.
    pub fn new(max_rows: usize, batch: usize) -> Result<Self, Error> {
        if max_rows == 0 || batch == 0 || batch > max_rows {
            return Err(Error::Invalid("CPU table/batch caps"));
        }
        Ok(Self {
            max_rows,
            batch,
            prepared: None,
            stats: Stats::default(),
            scratch: Scratch::default(),
        })
    }
    pub fn stats(&self) -> &Stats {
        &self.stats
    }
    /// Release retained tables, preserving cumulative counters.
    pub fn clear(&mut self) {
        self.prepared = None;
    }
    fn prepare(&mut self, curve: &Curve, bases: &[Vec<Point>], max_work: u64) -> Result<(), Error> {
        if !(2..=8).contains(&bases.len())
            || bases.iter().any(|base| {
                base.iter().any(|&p| p.is_none() || !curve.contains(p))
                    || base.windows(2).any(|w| w[0] >= w[1])
            })
        {
            return Err(Error::Invalid("CPU oracle bases"));
        }
        let mut best = None;
        if bases.iter().any(Vec::is_empty) {
            best = Some((0, 1));
        } else {
            for split in 1..bases.len() {
                let count = |slots: &[Vec<Point>]| {
                    slots.iter().try_fold(1usize, |n, b| n.checked_mul(b.len()))
                };
                if let Some(total) = count(&bases[..split])
                    .and_then(|a| count(&bases[split..]).and_then(|b| a.checked_add(b)))
                {
                    if total <= self.max_rows && best.is_none_or(|(old, _)| total < old) {
                        best = Some((total, split));
                    }
                }
            }
        }
        let (total, split) = best.ok_or(Error::Limit("CPU tuple tables exceed row cap"))?;
        if total as u64 > max_work {
            return Err(Error::Limit("CPU decomposition work; not nonexistence"));
        }
        // Drop the previous trial before allocating its admitted replacement.
        // Rejected inputs cannot reuse it: find checks exact curve/base identity.
        self.prepared = None;
        let prefix = if total == 0 {
            vec![]
        } else {
            sums(curve, &bases[..split])?
        };
        let suffix = Suffix::new(if total == 0 {
            vec![]
        } else {
            sums(curve, &bases[split..])?
        })?;
        self.stats.preparations += 1;
        self.stats.built_rows += total as u64;
        self.stats.peak_table_bytes = self.stats.peak_table_bytes.max(
            prefix.capacity() * std::mem::size_of::<Point>()
                + suffix.slots.capacity() * std::mem::size_of::<(u64, usize)>(),
        );
        self.prepared = Some(Prepared {
            curve: curve_key(curve),
            bases: bases.to_vec(),
            split,
            prefix,
            suffix,
        });
        Ok(())
    }
}

// Validated curve parameters determine the group law. Auxiliary square-root
// tables are derived data: comparing/cloning them on every query is O(p).
fn curve_key(curve: &Curve) -> [u64; 4] {
    let (a, b) = curve.coefficients();
    [curve.modulus(), a, b, curve.order()]
}

fn sums(curve: &Curve, bases: &[Vec<Point>]) -> Result<Vec<Point>, Error> {
    let mut sums = vec![None];
    let mut scratch = Scratch::default();
    for base in bases {
        // Bases have already been validated in prepare. Batch each row of
        // sum+base using the same exceptional-case-aware subtraction kernel.
        // Lexicographic tuple order is unchanged. This setup is fully charged.
        let negatives: Vec<_> = base
            .iter()
            .map(|&point| point.map(|(x, y)| (x, sub(0, y, curve.modulus()))))
            .collect();
        let mut next = Vec::with_capacity(sums.len() * base.len());
        for &sum in &sums {
            scratch.subtract(curve, sum, &negatives);
            next.extend_from_slice(&scratch.output);
        }
        sums = next;
    }
    Ok(sums)
}
fn decode(bases: &[Vec<Point>], mut id: usize, out: &mut [Point]) {
    for (base, point) in bases.iter().zip(out.iter_mut()).rev() {
        *point = base[id % base.len()];
        id /= base.len();
    }
}
impl DecompositionOracle for CachedMitm {
    fn find(
        &mut self,
        curve: &Curve,
        bases: &[Vec<Point>],
        target: Point,
        max_work: u64,
    ) -> Result<Option<Vec<Point>>, Error> {
        if !curve.contains(target) || curve.modulus() >= (1u64 << 62) {
            return Err(Error::Invalid("CPU oracle curve or target"));
        }
        self.stats.queries += 1;
        if self
            .prepared
            .as_ref()
            .is_none_or(|p| p.curve != curve_key(curve) || p.bases != bases)
        {
            self.prepare(curve, bases, max_work)?;
        }
        let p = self.prepared.as_ref().unwrap();
        if (p.prefix.len() + p.suffix.rows) as u64 > max_work {
            return Err(Error::Limit("CPU decomposition work; not nonexistence"));
        }
        // A density heuristic changes scheduling only, never search order.
        // Sparse suffix coverage retains full batches; dense suffix
        // coverage starts small to avoid computing unused residuals.
        let dense = (p.suffix.unique as u128) * 8 >= u128::from(curve.order());
        let mut start = 0;
        let mut batch = if dense { 1 } else { self.batch };
        while start < p.prefix.len() {
            let end = start + batch.min(p.prefix.len() - start);
            let points = &p.prefix[start..end];
            self.stats.query_inversions += self.scratch.subtract(curve, target, points);
            for (offset, residual) in self.scratch.output.iter().enumerate() {
                self.stats.probed_rows += 1;
                if let Some(id) = p.suffix.find(*residual) {
                    let mut tuple = vec![None; bases.len()];
                    decode(&bases[..p.split], start + offset, &mut tuple[..p.split]);
                    decode(&bases[p.split..], id, &mut tuple[p.split..]);
                    // Validate all points and independently sum at the oracle boundary.
                    // Full certificate replay still uses the shared verifier.
                    if !checked_sum::matches(curve, &tuple, target)? {
                        return Err(Error::Verification);
                    }
                    return Ok(Some(tuple));
                }
            }
            start = end;
            batch = batch.saturating_mul(2).min(self.batch);
        }
        Ok(None)
    }
}

// Same bounded ceiling-reciprocal reduction as the checked paper module.
// Kept private here because the shared module's API is owned by another lane.
// R=ceil(2^64/p); for x<2^64, floor(xR/2^64) overestimates floor(x/p)
// by at most one. For p<2^32, a wrapped negative residual is >=p.
fn reciprocal(p: u64) -> u64 {
    if p <= u32::MAX as u64 {
        u64::MAX / p + 1
    } else {
        0
    }
}
fn mul(a: u64, b: u64, p: u64, reciprocal: u64) -> u64 {
    debug_assert!(a < p && b < p && p > 1);
    if reciprocal == 0 {
        return ((a as u128 * b as u128) % p as u128) as u64;
    }
    let x = a * b;
    let q = ((x as u128 * reciprocal as u128) >> 64) as u64;
    let r = x.wrapping_sub(q.wrapping_mul(p));
    if r >= p {
        r.wrapping_add(p)
    } else {
        r
    }
}

fn sub(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}
#[derive(Default)]
struct Scratch {
    prefix: Vec<u64>,
    denominators: Vec<u64>,
    numerators: Vec<u64>,
    output: Vec<Point>,
}
impl Scratch {
    // Montgomery batch inversion, with identities/inverses handled explicitly.
    // The same affine equations as the checked CPU reference, one inversion
    // for every batch containing at least one ordinary addition or doubling.
    fn subtract(&mut self, curve: &Curve, target: Point, points: &[Point]) -> u64 {
        let p = curve.modulus();
        self.output.clear();
        self.output.extend_from_slice(points);
        let Some((x, y)) = target else {
            for point in &mut self.output {
                *point = point.map(|(x, y)| (x, sub(0, y, p)));
            }
            return 0;
        };
        let reciprocal = reciprocal(p);
        let mul = |a, b| mul(a, b, p, reciprocal);
        self.prefix.clear();
        self.denominators.clear();
        self.numerators.clear();
        let mut product = 1;
        let mut active = false;
        for (i, point) in points.iter().enumerate() {
            let (num, den) = match *point {
                None => {
                    self.output[i] = target;
                    (0, 0)
                }
                Some((xx, yy)) => {
                    let yy = sub(0, yy, p);
                    if x == xx && (y + yy == 0 || y + yy == p) {
                        self.output[i] = None;
                        (0, 0)
                    } else if x == xx {
                        ((3 * mul(x, x) + curve.coefficients().0) % p, 2 * y % p)
                    } else {
                        (sub(yy, y, p), sub(xx, x, p))
                    }
                }
            };
            self.prefix.push(product);
            self.denominators.push(den);
            self.numerators.push(num);
            if den != 0 {
                product = mul(product, den);
                active = true;
            }
        }
        if !active {
            return 0;
        }
        let mut inverse = curve.field_inverse(product);
        for i in (0..points.len()).rev() {
            let den = self.denominators[i];
            if den == 0 {
                continue;
            }
            let iz = mul(inverse, self.prefix[i]);
            inverse = mul(inverse, den);
            let slope = mul(self.numerators[i], iz);
            let xx = points[i].unwrap().0;
            let nx = sub(sub(mul(slope, slope), x, p), xx, p);
            self.output[i] = Some((nx, sub(mul(slope, sub(x, nx, p)), y, p)));
        }
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Curve, Point) {
        (Curve::new(211, 0, 7, 199).unwrap(), Some((3, 33)))
    }
    #[test]
    fn residual_multiplication_matches_independent_remainder() {
        for p in (2..128).chain([
            65521,
            65563,
            (1 << 32) - 5,
            (1 << 32) - 1,
            1 << 32,
            (1 << 40) + 87,
        ]) {
            let reciprocal = reciprocal(p);
            let residues: Vec<_> = if p < 128 {
                (0..p).collect()
            } else {
                (0..128)
                    .map(|x| (x * 7919) % p)
                    .chain([p - 1, p - 2, p / 2])
                    .collect()
            };
            for &a in &residues {
                for &b in &residues {
                    assert_eq!(
                        mul(a, b, p, reciprocal),
                        ((a as u128 * b as u128) % p as u128) as u64
                    );
                }
            }
        }
    }

    #[test]
    fn suffix_index_preserves_duplicates_identity_and_collision_chains() {
        let (c, p) = fixture();
        let points: Vec<_> = (0..199).map(|k| c.mul(p, k).unwrap()).collect();
        let mut input = points[..150].to_vec();
        input.extend_from_slice(&points[..50]);
        let index = Suffix::new(input.clone()).unwrap();
        assert_eq!(index.rows, input.len());
        assert_eq!(
            index.unique,
            input
                .iter()
                .collect::<std::collections::BTreeSet<_>>()
                .len()
        );
        for &point in &points {
            assert_eq!(index.find(point), input.iter().position(|&x| x == point));
        }
        assert!(Suffix::new(vec![]).unwrap().find(None).is_none());
        let mut buckets = std::collections::BTreeSet::new();
        assert!(points[..150]
            .iter()
            .any(|&point| !buckets.insert(point_bucket(point_key(point), index.slots.len() - 1))));
        assert_eq!(index.find(None), Some(0));
    }

    #[test]
    fn batched_subtraction_matches_every_group_pair() {
        let (c, p) = fixture();
        let points: Vec<_> = (0..199).map(|k| c.mul(p, k).unwrap()).collect();
        let mut scratch = Scratch::default();
        for &target in &points {
            scratch.subtract(&c, target, &points);
            for (&point, &got) in points.iter().zip(&scratch.output) {
                let neg = point.map(|(x, y)| (x, (211 - y) % 211));
                assert_eq!(got, c.add(target, neg).unwrap());
            }
        }
    }
    #[test]
    fn wide_field_batch_and_tuple_witnesses_match_checked_reference() {
        let c = Curve::with_order_witness(
            68719476731,
            68719476728,
            33,
            68719022539,
            Some((2, 51741851039)),
        )
        .unwrap();
        let p = Some((2, 51741851039));
        let points: Vec<_> = (0..80).map(|k| c.mul(p, k).unwrap()).collect();
        let mut scratch = Scratch::default();
        for &target in &points {
            scratch.subtract(&c, target, &points);
            for (&point, &actual) in points.iter().zip(&scratch.output) {
                let neg = point.map(|(x, y)| (x, (c.modulus() - y) % c.modulus()));
                assert_eq!(actual, c.add(target, neg).unwrap());
            }
        }
        let mut base = points[1..8].to_vec();
        base.sort_unstable();
        let bases = vec![base; 4];
        let mut oracle = CachedMitm::new(10000, 64).unwrap();
        for target in points {
            assert_eq!(
                oracle.find(&c, &bases, target, 10000).unwrap(),
                paper::Enumerate.find(&c, &bases, target, 10000).unwrap()
            );
        }
        assert_eq!(oracle.stats.preparations, 1);
    }
    #[test]
    fn exact_lexicographic_witnesses_all_targets_and_resets() {
        let (c, p) = fixture();
        for slots in 2..=6 {
            let mut bases: Vec<_> = (0..slots)
                .map(|_| (1..=3).map(|k| c.mul(p, k).unwrap()).collect::<Vec<_>>())
                .collect();
            for b in &mut bases {
                b.sort_unstable();
            }
            let mut reference = paper::Enumerate;
            for batch in [1, 2, 7, 256] {
                let mut oracle = CachedMitm::new(10000, batch).unwrap();
                for k in 0..199 {
                    let target = c.mul(p, k).unwrap();
                    assert_eq!(
                        oracle.find(&c, &bases, target, 10000).unwrap(),
                        reference.find(&c, &bases, target, 10000).unwrap()
                    );
                }
                assert_eq!(oracle.stats.preparations, 1);
                assert!(oracle.find(&c, &bases, p, 0).is_err());
                bases[0].clear();
                assert_eq!(oracle.find(&c, &bases, p, 0).unwrap(), None);
                bases[0] = bases[1].clone();
            }
        }
    }
    #[test]
    fn full_recovery_certificates_equal_original() {
        let (c, p) = fixture();
        for (m, seed) in [(2, 1), (3, 7), (4, 19)] {
            let config = paper::Config {
                m,
                seed,
                tau: Some(60),
                max_trials: 64,
                ..Default::default()
            };
            let q = c.mul(p, 42).unwrap();
            let expected = paper::solve(&c, p, q, &config).unwrap();
            let mut oracle = CachedMitm::new(100000, 64).unwrap();
            let actual = paper::solve_with_oracle(&c, p, q, &config, &mut oracle).unwrap();
            assert_eq!(actual, expected);
            paper::verify(&c, p, q, &actual).unwrap();
        }
    }
    #[test]
    fn complete_certificates_match_frozen_preoptimization_records() {
        let records = [
            include_str!(
                "../../../research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-0-1.txt"
            ),
            include_str!(
                "../../../research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-1-1.txt"
            ),
            include_str!(
                "../../../research/hashed_ecdl_cpu_20261030/panel2/results/cert-v-2-1.txt"
            ),
        ];
        for (case, &(p, a, b, r, generator, m, tau, scalar)) in [
            (211, 0, 7, 199, Some((3, 33)), 3, 40, 42),
            (65521, 65518, 3, 65563, Some((0, 64894)), 3, 80, 45678),
            (65521, 65518, 3, 65563, Some((0, 64894)), 4, 64, 54321),
        ]
        .iter()
        .enumerate()
        {
            let c = Curve::new(p, a, b, r).unwrap();
            let q = c.mul(generator, scalar).unwrap();
            let config = paper::Config {
                m,
                tau: Some(tau),
                seed: 111 * (case as u64 + 1),
                max_trials: 16,
                max_work: 1_000_000,
                ..Default::default()
            };
            let mut oracle = CachedMitm::new(1_000_000, 64).unwrap();
            let cert = paper::solve_with_oracle(&c, generator, q, &config, &mut oracle).unwrap();
            assert_eq!(format!("{cert:?}\n"), records[case]);
            paper::verify(&c, generator, q, &cert).unwrap();
        }
    }
    #[test]
    fn caps_and_bad_bases_fail_closed() {
        let (c, p) = fixture();
        let mut oracle = CachedMitm::new(2, 1).unwrap();
        assert!(oracle.find(&c, &[vec![p, p], vec![p]], p, 100).is_err());
        assert!(oracle.find(&c, &[vec![None], vec![p]], p, 100).is_err());
        assert!(oracle
            .find(&c, &[vec![p], vec![p]], Some((999, 0)), 100)
            .is_err());
        assert!(oracle.find(&c, &vec![vec![p]; 8], p, 1).is_err());
    }
    #[test]
    fn equivalent_validated_curve_representations_reuse_tables() {
        let (curve, generator) = fixture();
        let witness = Curve::with_order_witness(211, 0, 7, 199, generator).unwrap();
        let bases = vec![vec![generator]; 2];
        let mut oracle = CachedMitm::new(10, 2).unwrap();
        let target = curve.mul(generator, 2).unwrap();
        let expected = oracle.find(&curve, &bases, target, 10).unwrap();
        assert_eq!(oracle.find(&witness, &bases, target, 10).unwrap(), expected);
        assert_eq!(oracle.stats.preparations, 1);
    }
}
