//! First point-table study: one explicitly supplied native two-slot batch.
//! `run INPUT OUTPUT` executes a batch; `check INPUT OUTPUT` replays answers
//! with the existing word-field arithmetic. Output clocks carry no L2 admission.
//! This instrument submits no cloud jobs, establishes no complete pricing or frozen
//! protocol. Development tests use only the fixed public curve over F_17.

use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use crypto_lib::cryptanalysis::isogeny_eval::counted_fp::is_prime_u64;
use hashed_ecdl_tensor::{
    arithmetic::Seeded,
    curve::{Curve, Point},
    finite::{self, Domain, Options, PrimeDomain, Statistics},
    reduction::{FactorBase, HashSeed},
    BigUint, Error, Limits, Meter,
};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{hash_map::DefaultHasher, BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::hash::{BuildHasher, Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;
use std::time::Instant;

#[path = "accounting.rs"]
pub(crate) mod accounting;
#[path = "pilot.rs"]
mod pilot;

type Result<T> = std::result::Result<T, String>;
const MAX_INPUT_BYTES: u64 = 1_048_576;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Arm {
    Current,
    CurrentAa,
    Prepared,
    Ordered,
    Hashed,
    Fingerprint,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    Counts,
    Clocks,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct WirePoint {
    x: u64,
    y: u64,
    infinity: bool,
}
impl WirePoint {
    fn native(self) -> Point {
        if self.infinity {
            Point::Infinity
        } else {
            Point::Affine {
                x: self.x.into(),
                y: self.y.into(),
            }
        }
    }
    fn word(self) -> PrimePoint {
        PrimePoint {
            x: self.x,
            y: self.y,
            infinity: self.infinity,
        }
    }
    fn from_native(p: &Point) -> Result<Self> {
        Ok(match p {
            Point::Infinity => Self {
                x: 0,
                y: 0,
                infinity: true,
            },
            Point::Affine { x, y } => Self {
                x: x.to_u64().ok_or("point x exceeds word checker")?,
                y: y.to_u64().ok_or("point y exceeds word checker")?,
                infinity: false,
            },
        })
    }
}
impl From<PrimePoint> for WirePoint {
    fn from(p: PrimePoint) -> Self {
        Self {
            x: p.x,
            y: p.y,
            infinity: p.infinity,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurveInput {
    p: u64,
    a: u64,
    b: u64,
    order: u64,
    generator: WirePoint,
}
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BaseInput {
    prime: u64,
    threshold: u64,
    alpha: u64,
    beta: u64,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WorkLimits {
    steps: u64,
    vector: usize,
    bits: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Input {
    schema: String,
    id: String,
    arm: Arm,
    mode: Mode,
    force_hash_collisions: bool,
    curve: CurveInput,
    bases: [BaseInput; 2],
    queries: Vec<WirePoint>,
    limits: WorkLimits,
}
impl Input {
    fn validate(&self) -> Result<()> {
        if self.schema != "native-point-hash.batch-input/v1"
            || self.id.is_empty()
            || self.id.len() > 128
            || !(5..=65_535).contains(&self.curve.p)
            || self.curve.a >= self.curve.p
            || self.curve.b >= self.curve.p
            || !(3..=131_071).contains(&self.curve.order)
            || self.queries.is_empty()
            || self.queries.len() > 256
            || self.limits.steps == 0
            || self.limits.steps > 1_000_000_000
            || self.limits.vector == 0
            || self.limits.vector > 1_000_000
            || !(32..=256).contains(&self.limits.bits)
        {
            return Err("unsupported batch schema, small-prime domain or limits".into());
        }
        for b in self.bases {
            if !(self.curve.p * 2..=self.curve.p * 4).contains(&b.prime)
                || !(1..=128).contains(&b.threshold)
                || b.threshold > b.prime
                || b.alpha == 0
                || b.alpha >= b.prime
                || b.beta >= b.prime
            {
                return Err("invalid two-slot factor-base parameters".into());
            }
        }
        for p in self.queries.iter().chain([&self.curve.generator]) {
            if (p.infinity && (p.x != 0 || p.y != 0))
                || (!p.infinity && (p.x >= self.curve.p || p.y >= self.curve.p))
            {
                return Err("noncanonical input point".into());
            }
        }
        if self.curve.generator.infinity {
            return Err("identity generator".into());
        }
        Ok(())
    }
    fn hash(&self) -> Result<String> {
        Ok(format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(self).map_err(|e| e.to_string())?)
        ))
    }
}

// The ordered and hash arms use exactly the same full key and construct a key
// on every contains call. Thus their contrast does not remove serialization
// or coordinate-copy work from only one side.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum RawKey {
    Infinity,
    Affine(BigUint, BigUint),
}
impl From<&Point> for RawKey {
    fn from(p: &Point) -> Self {
        match p {
            Point::Infinity => Self::Infinity,
            Point::Affine { x, y } => Self::Affine(x.clone(), y.clone()),
        }
    }
}
#[derive(Clone, Debug)]
struct CountedKey(RawKey);
impl CountedKey {
    fn new(key: RawKey) -> Self {
        accounting::event(accounting::Event::KeyConstruction);
        Self(key)
    }
}
impl PartialEq for CountedKey {
    fn eq(&self, other: &Self) -> bool {
        accounting::event(accounting::Event::Equality);
        self.0 == other.0
    }
}
impl Eq for CountedKey {}
impl Ord for CountedKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        accounting::event(accounting::Event::Ordering);
        self.0.cmp(&other.0)
    }
}
impl PartialOrd for CountedKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Hash for CountedKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        accounting::event(accounting::Event::KeyHash);
        self.0.hash(state);
    }
}
#[derive(Clone, Copy)]
struct TableHasher<const TRACK: bool> {
    constant: bool,
}
struct TableHash<const TRACK: bool> {
    inner: DefaultHasher,
    constant: bool,
}
impl<const TRACK: bool> BuildHasher for TableHasher<TRACK> {
    type Hasher = TableHash<TRACK>;
    fn build_hasher(&self) -> TableHash<TRACK> {
        if TRACK {
            accounting::event(accounting::Event::HashBuild);
        }
        TableHash {
            inner: DefaultHasher::new(),
            constant: self.constant,
        }
    }
}
impl<const TRACK: bool> Hasher for TableHash<TRACK> {
    fn finish(&self) -> u64 {
        if TRACK {
            accounting::event(accounting::Event::HashFinish);
        }
        if self.constant {
            0
        } else {
            self.inner.finish()
        }
    }
    fn write(&mut self, bytes: &[u8]) {
        if TRACK {
            accounting::event(accounting::Event::HashBytes(bytes.len()));
        }
        self.inner.write(bytes);
    }
}
enum Lookup {
    Arithmetic,
    Ordered(BTreeSet<RawKey>),
    Hashed(HashSet<RawKey, TableHasher<false>>),
    Fingerprint(HashMap<u64, Vec<RawKey>, TableHasher<false>>),
    CountedOrdered(BTreeSet<CountedKey>),
    CountedHashed(HashSet<CountedKey, TableHasher<true>>),
    CountedFingerprint(HashMap<u64, Vec<CountedKey>, TableHasher<true>>),
}
fn hash_table<K: Eq + Hash, const TRACK: bool>(
    points: impl Iterator<Item = K>,
    constant: bool,
) -> HashSet<K, TableHasher<TRACK>> {
    let mut table = HashSet::with_hasher(TableHasher { constant });
    table.extend(points);
    table
}
fn fingerprint_table<K: Eq + Hash, const TRACK: bool>(
    points: impl Iterator<Item = K>,
    constant: bool,
) -> HashMap<u64, Vec<K>, TableHasher<TRACK>> {
    let hasher = TableHasher { constant };
    let mut table: HashMap<u64, Vec<K>, _> = HashMap::with_hasher(hasher);
    for key in points {
        table.entry(hasher.hash_one(&key)).or_default().push(key);
    }
    table
}
impl Lookup {
    fn build(arm: Arm, points: impl Iterator<Item = RawKey>, constant: bool, track: bool) -> Self {
        match (arm, track) {
            (Arm::Ordered, false) => Self::Ordered(points.collect()),
            (Arm::Hashed, false) => Self::Hashed(hash_table(points, constant)),
            (Arm::Fingerprint, false) => Self::Fingerprint(fingerprint_table(points, constant)),
            (Arm::Ordered, true) => Self::CountedOrdered(points.map(CountedKey::new).collect()),
            (Arm::Hashed, true) => {
                Self::CountedHashed(hash_table(points.map(CountedKey::new), constant))
            }
            (Arm::Fingerprint, true) => {
                Self::CountedFingerprint(fingerprint_table(points.map(CountedKey::new), constant))
            }
            _ => Self::Arithmetic,
        }
    }
    fn contains(&self, point: &Point) -> Option<bool> {
        Some(match self {
            Self::Arithmetic => return None,
            Self::Ordered(table) => table.contains(&RawKey::from(point)),
            Self::Hashed(table) => table.contains(&RawKey::from(point)),
            Self::Fingerprint(table) => {
                let key = RawKey::from(point);
                table
                    .get(&table.hasher().hash_one(&key))
                    .is_some_and(|bucket| bucket.iter().any(|p| p == &key))
            }
            Self::CountedOrdered(table) => {
                accounting::event(accounting::Event::Lookup);
                table.contains(&CountedKey::new(RawKey::from(point)))
            }
            Self::CountedHashed(table) => {
                accounting::event(accounting::Event::Lookup);
                table.contains(&CountedKey::new(RawKey::from(point)))
            }
            Self::CountedFingerprint(table) => {
                accounting::event(accounting::Event::Lookup);
                let key = CountedKey::new(RawKey::from(point));
                table
                    .get(&table.hasher().hash_one(&key))
                    .is_some_and(|bucket| bucket.iter().any(|p| p == &key))
            }
        })
    }
}

struct PreparedDomain {
    source: PrimeDomain,
    points: Vec<Option<Point>>,
    lookup: Lookup,
}
impl Domain<Curve> for PreparedDomain {
    fn validate(&self, curve: &Curve, meter: &mut Meter) -> hashed_ecdl_tensor::Result<()> {
        self.source.validate(curve, meter)
    }
    fn limit(&self) -> &BigUint {
        self.source.limit()
    }
    fn decode(
        &self,
        _: &Curve,
        selector: &BigUint,
        _: &mut Meter,
    ) -> hashed_ecdl_tensor::Result<Option<Point>> {
        let i = selector
            .to_usize()
            .filter(|i| *i < self.points.len())
            .ok_or_else(|| Error::Invalid("prepared selector outside domain".into()))?;
        Ok(self.points[i].clone())
    }
    fn contains(
        &self,
        curve: &Curve,
        point: &Point,
        meter: &mut Meter,
    ) -> hashed_ecdl_tensor::Result<bool> {
        if matches!(self.lookup, Lookup::Arithmetic) {
            self.source.contains(curve, point, meter)
        } else {
            Ok(self.lookup.contains(point).unwrap_or(false))
        }
    }
}

struct Prepared {
    curve: Curve,
    bases: Vec<FactorBase>,
    // Retain the original ordered matrix indexes in every arm, even though the
    // component driver does not perform elimination. Their cost is common.
    indexes: Vec<BTreeMap<Point, usize>>,
    domains: Vec<PreparedDomain>,
}
fn prepare(input: &Input, meter: &mut Meter) -> Result<Prepared> {
    let c = &input.curve;
    let curve = Curve::new(
        c.p.into(),
        c.a.into(),
        c.b.into(),
        c.order.into(),
        c.generator.native(),
        meter,
    )
    .map_err(|e| e.to_string())?;
    let mut bases = Vec::new();
    let mut indexes = Vec::new();
    let mut domains = Vec::new();
    let mut next = 0;
    for spec in input.bases {
        let base = FactorBase::new(
            &curve,
            spec.prime.into(),
            spec.threshold.into(),
            HashSeed {
                alpha: spec.alpha.into(),
                beta: spec.beta.into(),
            },
            meter,
        )
        .map_err(|e| e.to_string())?;
        let points = base
            .enumerate(&curve, &mut Seeded(0x706f696e74686173), meter)
            .map_err(|e| e.to_string())?;
        meter
            .vector(points.len(), "study column index")
            .map_err(|e| e.to_string())?;
        let mut index = BTreeMap::new();
        for point in points {
            index.insert(point, next);
            next += 1;
        }
        if !matches!(input.arm, Arm::Current | Arm::CurrentAa) {
            let n = usize::try_from(spec.threshold).map_err(|e| e.to_string())?;
            meter
                .vector(n, "study prepared selectors")
                .map_err(|e| e.to_string())?;
            let mut selectors = vec![None; n];
            for point in index.keys() {
                let selector = ((&base.seed().alpha
                    * curve.encode(point).map_err(|e| e.to_string())?
                    + &base.seed().beta)
                    % base.prime())
                .to_usize()
                .ok_or("prepared selector exceeds usize")?;
                if selector >= n || selectors[selector].replace(point.clone()).is_some() {
                    return Err("invalid or duplicate prepared selector".into());
                }
            }
            let source = PrimeDomain::new(base.clone(), meter).map_err(|e| e.to_string())?;
            let lookup = Lookup::build(
                input.arm,
                index.keys().map(RawKey::from),
                input.force_hash_collisions,
                input.mode == Mode::Counts,
            );
            domains.push(PreparedDomain {
                source,
                points: selectors,
                lookup,
            });
        }
        bases.push(base);
        indexes.push(index);
    }
    Ok(Prepared {
        curve,
        bases,
        indexes,
        domains,
    })
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum Outcome {
    Witness { points: [WirePoint; 2] },
    Absent,
    Unknown { reason: String },
    NotAttempted,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Prefix {
    queries: usize,
    cold_ns: Option<u64>,
    meter_steps: u64,
    curve_additions: u64,
    root_requests: u64,
    work: Option<accounting::Snapshot>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Record {
    schema: String,
    input_sha256: String,
    arm: Arm,
    mode: Mode,
    base_sizes: [usize; 2],
    preparation: Prefix,
    prefixes: Vec<Prefix>,
    final_cost: Prefix,
    outcomes: Vec<Outcome>,
    pricing: String,
    clock_admission: String,
}
fn prefix(queries: usize, start: Option<Instant>, meter: &Meter, track: bool) -> Result<Prefix> {
    Ok(Prefix {
        queries,
        cold_ns: start
            .map(|t| u64::try_from(t.elapsed().as_nanos()).map_err(|e| e.to_string()))
            .transpose()?,
        meter_steps: meter.used,
        curve_additions: meter.curve_additions,
        root_requests: meter.square_roots,
        work: if track {
            Some(accounting::snapshot().ok_or("missing active accounting interval")?)
        } else {
            None
        },
    })
}
fn native_replay(
    prepared: &Prepared,
    target: &Point,
    points: &[Point],
    meter: &mut Meter,
) -> hashed_ecdl_tensor::Result<()> {
    if points.len() != 2 {
        return Err(Error::Integrity("two-slot witness arity".into()));
    }
    for (point, base) in points.iter().zip(&prepared.bases) {
        // Always use the original arithmetic membership, including in indexed
        // arms. Replaying through the lookup itself would not be independent.
        if !base.contains(&prepared.curve, point, meter)? {
            return Err(Error::Integrity(
                "study arithmetic membership replay".into(),
            ));
        }
    }
    if prepared.curve.add(&points[0], &points[1], meter)? != *target {
        return Err(Error::Integrity("study sum replay".into()));
    }
    Ok(())
}
fn run_batch(input: &Input) -> Result<Record> {
    input.validate()?;
    if input.mode == Mode::Clocks && accounting::has_allocator() {
        return Err("accounting binary refuses clocks; use the ordinary study binary".into());
    }
    let input_sha256 = input.hash()?;
    let mut meter = Meter::new(Limits {
        steps: input.limits.steps,
        vector: input.limits.vector,
        bits: input.limits.bits,
    });
    // Reporter storage is prepared outside the native accounting/clock interval.
    let mut outcomes = Vec::with_capacity(input.queries.len());
    let mut prefixes = Vec::with_capacity(5);
    let track = input.mode == Mode::Counts;
    let scope = if track {
        Some(accounting::Scope::start()?)
    } else {
        None
    };
    let start = (input.mode == Mode::Clocks).then(Instant::now);
    let prepared = prepare(input, &mut meter)?;
    let preparation = prefix(0, start, &meter, track)?;
    let mut statistics = Statistics::default();
    let mut stopped = false;
    let mut attempted = 0;
    for target in &input.queries {
        if stopped {
            outcomes.push(Outcome::NotAttempted);
            continue;
        }
        attempted += 1;
        let target = target.native();
        let answer = if matches!(input.arm, Arm::Current | Arm::CurrentAa) {
            finite::prime_decompose(
                &prepared.curve,
                &prepared.bases,
                &target,
                &Options::default(),
                &mut statistics,
                &mut meter,
            )
        } else {
            finite::decompose(
                &prepared.curve,
                &prepared.domains,
                &target,
                &Options::default(),
                &mut statistics,
                &mut meter,
            )
        }
        .and_then(|answer| {
            if let Some(points) = &answer {
                native_replay(&prepared, &target, points, &mut meter)?;
            }
            Ok(answer)
        });
        outcomes.push(match answer {
            Ok(Some(points)) => Outcome::Witness {
                points: [
                    WirePoint::from_native(&points[0])?,
                    WirePoint::from_native(&points[1])?,
                ],
            },
            Ok(None) => Outcome::Absent,
            Err(Error::Limit(reason)) => {
                stopped = true;
                Outcome::Unknown { reason }
            }
            Err(e) => return Err(e.to_string()),
        });
        if [1, 4, 16, 64, 256].contains(&attempted) {
            prefixes.push(prefix(attempted, start, &meter, track)?);
        }
    }
    let final_cost = prefix(attempted, start, &meter, track)?;
    drop(scope);
    Ok(Record {
        schema: "native-point-hash.batch-record/v2".into(),
        input_sha256,
        arm: input.arm,
        mode: input.mode,
        base_sizes: [prepared.indexes[0].len(), prepared.indexes[1].len()],
        preparation,
        prefixes,
        final_cost,
        outcomes,
        pricing: "incomplete: native/key/hash diagnostics and optional observed allocation requests; field pricing, internal probe groups and RSS unknown; no calibrated S".into(),
        clock_admission: "unadmitted: require external native L2, A/A and protocol admission; checker cost excluded".into(),
    })
}

fn word_member(curve: &PrimeCurve, base: BaseInput, p: PrimePoint) -> bool {
    !p.infinity
        && curve.is_on_curve(p)
        && (base.alpha * (2 * p.x + (p.y & 1)) + base.beta) % base.prime < base.threshold
}
fn word_bases(input: &Input) -> Result<(PrimeCurve, [Vec<PrimePoint>; 2])> {
    input.validate()?;
    let c = &input.curve;
    let curve = PrimeCurve {
        p: c.p,
        a: c.a,
        b: c.b,
    };
    if !is_prime_u64(c.p)
        || !is_prime_u64(c.order)
        || !input.bases.iter().all(|b| is_prime_u64(b.prime))
        || num_integer::Integer::is_multiple_of(&(4 * c.a * c.a * c.a + 27 * c.b * c.b), &c.p)
        || !curve.is_on_curve(c.generator.word())
        || curve.point_count() != c.order
        || curve.mul(&mut GroupOps::default(), c.generator.word(), c.order) != PrimePoint::INFINITY
    {
        return Err("word checker rejected domain/order/generator".into());
    }
    let mut bases: [Vec<PrimePoint>; 2] = [Vec::new(), Vec::new()];
    let mut p = PrimePoint::INFINITY;
    for _ in 0..c.order {
        for (base, spec) in bases.iter_mut().zip(input.bases) {
            if word_member(&curve, spec, p) {
                base.push(p);
            }
        }
        p = curve.add(&mut GroupOps::default(), p, c.generator.word());
    }
    for (base, spec) in bases.iter_mut().zip(input.bases) {
        // Recover native selector order independently by scanning the whole
        // prime-order group. Do not consult prepared native point lists.
        base.sort_by_key(|p| (spec.alpha * (2 * p.x + (p.y & 1)) + spec.beta) % spec.prime);
    }
    Ok((curve, bases))
}
fn check_record(input: &Input, record: &Record) -> Result<()> {
    if record.schema != "native-point-hash.batch-record/v2"
        || record.input_sha256 != input.hash()?
        || record.arm != input.arm
        || record.mode != input.mode
        || record.outcomes.len() != input.queries.len()
    {
        return Err("record/input binding mismatch".into());
    }
    let (curve, bases) = word_bases(input)?;
    if record.base_sizes != [bases[0].len(), bases[1].len()] {
        return Err("word checker rejected native base cardinality".into());
    }
    let first = usize::from(input.bases[1].threshold < input.bases[0].threshold);
    let last = 1 - first;
    let mut stopped = false;
    for (query, outcome) in input.queries.iter().zip(&record.outcomes) {
        if !curve.is_on_curve(query.word()) {
            return Err("word checker rejected query".into());
        }
        if stopped {
            if *outcome != Outcome::NotAttempted {
                return Err("attempted outcome after resource stop".into());
            }
            continue;
        }
        match outcome {
            Outcome::Unknown { .. } => {
                stopped = true;
                continue;
            }
            Outcome::NotAttempted => return Err("unexplained skipped query".into()),
            _ => {}
        }
        let expected = bases[first].iter().find_map(|p| {
            let q = curve.add(&mut GroupOps::default(), query.word(), curve.neg(*p));
            if word_member(&curve, input.bases[last], q) {
                let mut points = [WirePoint::from(*p), WirePoint::from(q)];
                if first == 1 {
                    points.swap(0, 1);
                }
                Some(points)
            } else {
                None
            }
        });
        match (outcome, expected) {
            (Outcome::Absent, None) => {}
            (Outcome::Witness { points }, Some(expected)) if *points == expected => {}
            _ => {
                return Err(
                    "word checker rejected witness, first-witness order or complete absence".into(),
                )
            }
        }
    }
    for p in std::iter::once(&record.preparation)
        .chain(&record.prefixes)
        .chain([&record.final_cost])
    {
        if (input.mode == Mode::Counts && (p.cold_ns.is_some() || p.work.is_none()))
            || (input.mode == Mode::Clocks && (p.cold_ns.is_none() || p.work.is_some()))
        {
            return Err("record mixes instrumented counts and clocks".into());
        }
        if let Some(work) = p.work {
            if work.keys.overflow || work.allocations.is_some_and(|a| a.overflow) {
                return Err("overflowed diagnostic counters are not valid measurements".into());
            }
            if work.internal_probe_groups.is_some() || work.peak_live_bytes.is_some() {
                return Err("unsupported probe/live-byte metrics were supplied".into());
            }
        }
    }
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    let file = fs::File::open(path).map_err(|e| e.to_string())?;
    let mut bytes = Vec::new();
    file.take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err("input exceeds bounded read".into());
    }
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}
fn write_new(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    serde_json::to_writer(&mut file, value).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())
}
pub(crate) fn entry(
    args: Vec<std::ffi::OsString>,
    hooks: Option<accounting::AllocationHooks>,
) -> Result<()> {
    accounting::configure(hooks)?;
    (|| -> Result<()> {
        if args.get(1).and_then(|s| s.to_str()).is_some_and(|s| {
            [
                "make-plan",
                "make-development-plan",
                "check-plan",
                "execute-plan",
                "analyze-plan",
                "qualify-development-accounting",
            ]
            .contains(&s)
        }) {
            return pilot::cli(&args);
        }
        if args.len() != 4 {
            return Err("usage: native_point_hash_study run|check INPUT OUTPUT".into());
        }
        let input: Input = read_json(Path::new(&args[2]))?;
        match args[1].to_str() {
            Some("run") => {
                // Refuse overwrite before doing work. A failed run retains a
                // structured error artifact; no failure becomes an absent row.
                if Path::new(&args[3]).exists() {
                    return Err("output already exists".into());
                }
                match run_batch(&input) {
                    Ok(record) => {
                        write_new(Path::new(&args[3]), &record)?;
                        check_record(&input, &record)
                    }
                    Err(error) => {
                        write_new(
                            Path::new(&args[3]),
                            &serde_json::json!({
                                "schema":"native-point-hash.batch-failure/v1",
                                "input_sha256":input.hash()?, "error":error
                            }),
                        )?;
                        Err(error)
                    }
                }
            }
            Some("check") => {
                check_record(&input, &read_json(Path::new(&args[3]))?)?;
                println!(
                    "{}",
                    serde_json::json!({
                        "schema":"native-point-hash.answer-replay/v1",
                        "input_sha256":input.hash()?,
                        "scope":"input binding, complete base cardinalities and attempted witness/absence answers only",
                        "not_checked":["work counters", "clocks", "L2 admission", "PCC standing"],
                        "standing":"producer replay adapter using existing word arithmetic; no independent audit verdict"
                    })
                );
                Ok(())
            }
            _ => Err("expected run or check".into()),
        }
    })()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(arm: Arm) -> Input {
        let curve = PrimeCurve { p: 17, a: 2, b: 2 };
        let generator = PrimePoint::affine(5, 1);
        Input {
            schema: "native-point-hash.batch-input/v1".into(),
            id: "public-f17-development-control".into(),
            arm,
            mode: Mode::Counts,
            force_hash_collisions: false,
            curve: CurveInput {
                p: 17,
                a: 2,
                b: 2,
                order: 19,
                generator: generator.into(),
            },
            bases: [
                BaseInput {
                    prime: 37,
                    threshold: 24,
                    alpha: 1,
                    beta: 2,
                },
                BaseInput {
                    prime: 37,
                    threshold: 12,
                    alpha: 3,
                    beta: 4,
                },
            ],
            queries: (0..19)
                .map(|k| curve.mul(&mut GroupOps::default(), generator, k).into())
                .collect(),
            limits: WorkLimits {
                steps: 10_000_000,
                vector: 4096,
                bits: 64,
            },
        }
    }
    #[test]
    fn counted_keys_delegate_identity_and_plain_tables_emit_no_events() {
        let input = fixture(Arm::Hashed);
        let scope = accounting::Scope::start().unwrap();
        let raw: Vec<_> = input
            .queries
            .iter()
            .map(|p| RawKey::from(&p.native()))
            .collect();
        for a in &raw {
            for b in &raw {
                let ca = CountedKey::new(a.clone());
                let cb = CountedKey::new(b.clone());
                assert_eq!(a == b, ca == cb);
                assert_eq!(a.cmp(b), ca.cmp(&cb));
            }
            for constant in [false, true] {
                let hash = TableHasher::<false> { constant };
                assert_eq!(hash.hash_one(a), hash.hash_one(CountedKey::new(a.clone())));
            }
        }
        let before = accounting::snapshot().unwrap();
        for arm in [Arm::Ordered, Arm::Hashed, Arm::Fingerprint] {
            let table = Lookup::build(arm, raw.clone().into_iter(), false, false);
            for point in &input.queries {
                assert_eq!(table.contains(&point.native()), Some(true));
            }
        }
        assert_eq!(accounting::snapshot().unwrap(), before);
        drop(scope);
    }
    #[test]
    fn indexed_arms_report_key_work_and_reject_unsupported_diagnostics() {
        for arm in [Arm::Ordered, Arm::Hashed, Arm::Fingerprint] {
            let input = fixture(arm);
            let mut record = run_batch(&input).unwrap();
            let keys = record.final_cost.work.unwrap().keys;
            assert!(keys.key_constructions > 0 && keys.table_lookups > 0);
            if arm == Arm::Ordered {
                assert!(keys.full_key_orderings > 0);
                assert_eq!(keys.key_hash_calls, 0);
            } else {
                assert!(keys.key_hash_calls > 0 && keys.hash_input_bytes > 0);
                assert!(keys.hash_builds > 0 && keys.hash_finishes > 0);
            }
            record.final_cost.work.as_mut().unwrap().keys.overflow = true;
            assert!(check_record(&input, &record).is_err());
            record.final_cost.work.as_mut().unwrap().keys.overflow = false;
            record
                .final_cost
                .work
                .as_mut()
                .unwrap()
                .internal_probe_groups = Some(0);
            assert!(check_record(&input, &record).is_err());
            record
                .final_cost
                .work
                .as_mut()
                .unwrap()
                .internal_probe_groups = None;
            record.final_cost.work.as_mut().unwrap().peak_live_bytes = Some(0);
            assert!(check_record(&input, &record).is_err());
        }
    }
    #[test]
    fn six_arms_preserve_complete_first_witness_stream() {
        let baseline = run_batch(&fixture(Arm::Current)).unwrap();
        assert!(baseline
            .outcomes
            .iter()
            .any(|o| matches!(o, Outcome::Witness { .. })));
        for arm in [
            Arm::Current,
            Arm::CurrentAa,
            Arm::Prepared,
            Arm::Ordered,
            Arm::Hashed,
            Arm::Fingerprint,
        ] {
            let input = fixture(arm);
            let record = run_batch(&input).unwrap();
            check_record(&input, &record).unwrap();
            assert_eq!(record.outcomes, baseline.outcomes);
            assert!(record.final_cost.cold_ns.is_none());
        }
    }
    #[test]
    fn forced_collisions_do_not_change_answers_or_signs() {
        for arm in [Arm::Hashed, Arm::Fingerprint] {
            let mut input = fixture(arm);
            let original = run_batch(&input).unwrap();
            input.force_hash_collisions = true;
            let collided = run_batch(&input).unwrap();
            check_record(&input, &collided).unwrap();
            assert_eq!(collided.outcomes, original.outcomes);
        }
        assert_ne!(
            RawKey::from(&Point::Affine {
                x: 5u8.into(),
                y: 1u8.into()
            }),
            RawKey::from(&Point::Affine {
                x: 5u8.into(),
                y: 16u8.into()
            })
        );
        assert_ne!(
            RawKey::Infinity,
            RawKey::from(&Point::Affine {
                x: 0u8.into(),
                y: 0u8.into()
            })
        );
    }
    #[test]
    fn empty_base_and_complete_negatives_are_replayed() {
        let mut input = fixture(Arm::Fingerprint);
        input.bases[0] = BaseInput {
            prime: 37,
            threshold: 1,
            alpha: 1,
            beta: 3,
        };
        let record = run_batch(&input).unwrap();
        assert_eq!(record.base_sizes[0], 0);
        assert!(record.outcomes.iter().all(|o| *o == Outcome::Absent));
        check_record(&input, &record).unwrap();
    }
    #[test]
    fn limits_remain_unknown_and_stop_later_queries() {
        let mut input = fixture(Arm::Fingerprint);
        let full = run_batch(&input).unwrap();
        input.limits.steps = full.preparation.meter_steps + 1;
        let record = run_batch(&input).unwrap();
        assert!(matches!(record.outcomes[0], Outcome::Unknown { .. }));
        assert!(record.outcomes[1..]
            .iter()
            .all(|o| *o == Outcome::NotAttempted));
        check_record(&input, &record).unwrap();
    }
    #[test]
    fn replay_rejects_corruption_and_input_swaps() {
        let input = fixture(Arm::Ordered);
        let mut record = run_batch(&input).unwrap();
        let i = record
            .outcomes
            .iter()
            .position(|o| matches!(o, Outcome::Witness { .. }))
            .unwrap();
        record.outcomes[i] = Outcome::Absent;
        assert!(check_record(&input, &record).is_err());
        let mut changed = input.clone();
        changed.bases[0].beta += 1;
        assert!(check_record(&changed, &run_batch(&input).unwrap()).is_err());
    }
    #[test]
    fn noncanonical_and_unknown_input_fields_are_rejected() {
        let mut input = fixture(Arm::Current);
        input.queries[0].x = 17;
        assert!(run_batch(&input).is_err());
        let mut json = serde_json::to_value(fixture(Arm::Current)).unwrap();
        json["hidden_scalar"] = 7.into();
        assert!(serde_json::from_value::<Input>(json).is_err());
    }
    #[test]
    fn slot_order_ties_and_new_base_configuration_match_current() {
        for thresholds in [[12, 24], [12, 12]] {
            let mut input = fixture(Arm::Current);
            input.bases[0].threshold = thresholds[0];
            input.bases[1].threshold = thresholds[1];
            input.bases[0].beta = 7;
            let original = run_batch(&input).unwrap();
            for arm in [Arm::Prepared, Arm::Ordered, Arm::Hashed, Arm::Fingerprint] {
                input.arm = arm;
                let record = run_batch(&input).unwrap();
                check_record(&input, &record).unwrap();
                assert_eq!(record.outcomes, original.outcomes);
            }
        }
    }
    #[test]
    fn bounded_reads_and_existing_outputs_are_preserved() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "native-point-hash-control-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&dir).unwrap();
        let path = dir.join("record.json");
        write_new(&path, &serde_json::json!({"original":true})).unwrap();
        assert!(write_new(&path, &serde_json::json!({"replacement":true})).is_err());
        assert_eq!(
            read_json::<serde_json::Value>(&path).unwrap(),
            serde_json::json!({"original":true})
        );
        fs::remove_file(&path).unwrap();
        fs::write(&path, vec![b' '; MAX_INPUT_BYTES as usize + 1]).unwrap();
        assert!(read_json::<Input>(&path)
            .unwrap_err()
            .contains("bounded read"));
        fs::remove_file(&path).unwrap();
        fs::remove_dir(&dir).unwrap();
    }
}
