//! Independent finite verification for TR-04.
//!
//! Data types are shared with the producer; arithmetic, selector transitions,
//! probability counting and matrix elimination are not.  These checks do not
//! independently establish implementation-level work counters or a timing claim.

use crate::core::{Parameters, Point};
use crate::tr04_relation::RelationAudit;
use crate::tr04_select::{SelectionAudit, Work};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize)]
pub struct VerificationScope {
    pub group_arithmetic_independent: bool,
    pub selector_multiplicities_independent: bool,
    pub relation_equalities_independent: bool,
    pub rank_histograms_independent: bool,
    pub digest_independent: bool,
    pub work_counters_independent: bool,
    pub scope_note: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelInfo {
    pub nonsingular: bool,
    pub group_order: Option<u64>,
    /// The fixed study generator is defined only for eligible prime-order
    /// groups; no generator claim is made for other rational-point groups.
    pub canonical_generator: Option<Point>,
    pub eligible: bool,
}

/// Recompute a model's disposition from every affine square, without using
/// production character, point-count, primality or generator helpers.
pub fn model_info(params: Parameters) -> Result<ModelInfo, String> {
    let Parameters { p, a, b } = params;
    if !(7..=127).contains(&p) || p % 4 != 3 || !independent_prime(p) || a >= p || b >= p {
        return Err("Independent model census received invalid bounded parameters".into());
    }
    if (4 * a as u128 * a as u128 * a as u128 + 27 * b as u128 * b as u128)
        .is_multiple_of(p as u128)
    {
        return Ok(ModelInfo {
            nonsingular: false,
            group_order: None,
            canonical_generator: None,
            eligible: false,
        });
    }
    let mut roots = vec![Vec::new(); p as usize];
    for y in 0..p {
        roots[(y * y % p) as usize].push(y);
    }
    let mut order = 1;
    let mut xs = Vec::new();
    let mut first = None;
    for x in 0..p {
        let value = ((x as u128 * x as u128 * x as u128 + a as u128 * x as u128 + b as u128)
            % p as u128) as usize;
        order += roots[value].len() as u64;
        if value != 0 && !roots[value].is_empty() {
            xs.push(x);
            if first.is_none() {
                first = Some(Point {
                    x,
                    y: roots[value][0],
                });
            }
        }
    }
    let eligible = order >= 7 && independent_prime(order);
    let canonical_generator = if eligible {
        let generator = first.ok_or("Eligible model has no nonzero canonical point")?;
        let curve = SquareCurve {
            params,
            roots,
            xs,
            order,
        };
        // The complete distinct orbit verifies the generator claim, not only
        // a divisibility test at a single point.
        generator_chain(&curve, generator)?;
        Some(generator)
    } else {
        None
    };
    Ok(ModelInfo {
        nonsingular: true,
        group_order: Some(order),
        canonical_generator,
        eligible,
    })
}

/// Check a serialized/model-record disposition against the independent census.
pub fn verify_model_info(
    params: Parameters,
    reported: &ModelInfo,
) -> Result<VerificationScope, String> {
    if &model_info(params)? != reported {
        return Err("Independent model disposition disagrees".into());
    }
    Ok(VerificationScope {
        group_arithmetic_independent: true,
        selector_multiplicities_independent: false,
        relation_equalities_independent: false,
        rank_histograms_independent: false,
        digest_independent: false,
        work_counters_independent: false,
        scope_note: "Singularity, exact affine point count, prime-order eligibility and eligible canonical generator orbit; no operation-cost claim.".into(),
    })
}

type Element = Option<Point>;
type PointKey = (bool, u64, u64);

fn key(point: Element) -> PointKey {
    point.map_or((false, 0, 0), |p| (true, p.x, p.y))
}

fn residue(value: i128, modulus: u64) -> u64 {
    let m = modulus as i128;
    ((value % m + m) % m) as u64
}

fn independent_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            return false;
        }
        divisor += 1;
    }
    true
}

fn inverse(value: u64, modulus: u64) -> Result<u64, String> {
    let (mut old_r, mut r) = (modulus as i128, value as i128);
    let (mut old_t, mut t) = (0i128, 1i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_t, t) = (t, old_t - quotient * t);
    }
    if old_r != 1 {
        return Err("Independent inversion received a nonunit".into());
    }
    Ok(residue(old_t, modulus))
}

struct SquareCurve {
    params: Parameters,
    roots: Vec<Vec<u64>>,
    xs: Vec<u64>,
    order: u64,
}

impl SquareCurve {
    fn new(params: Parameters) -> Result<Self, String> {
        let Parameters { p, a, b } = params;
        if !(7..=127).contains(&p) || p % 4 != 3 || !independent_prime(p) || a >= p || b >= p {
            return Err("Independent verifier received invalid bounded parameters".into());
        }
        if (4 * a as u128 * a as u128 * a as u128 + 27 * b as u128 * b as u128)
            .is_multiple_of(p as u128)
        {
            return Err("Independent verifier received a singular curve".into());
        }
        let mut roots = vec![Vec::new(); p as usize];
        for y in 0..p {
            roots[(y * y % p) as usize].push(y);
        }
        let mut result = Self {
            params,
            roots,
            xs: Vec::new(),
            order: 1,
        };
        for x in 0..p {
            let values = &result.roots[result.rhs(x) as usize];
            result.order += values.len() as u64;
            if values.len() == 2 {
                result.xs.push(x);
            }
        }
        if result.order < 7 || !independent_prime(result.order) {
            return Err("Independent verifier requires prime curve order at least seven".into());
        }
        if result.xs.len() as u64 * 2 + 1 != result.order {
            return Err("Prime-order model unexpectedly has rational two-torsion".into());
        }
        Ok(result)
    }

    fn rhs(&self, x: u64) -> u64 {
        let Parameters { p, a, b } = self.params;
        ((x as u128 * x as u128 * x as u128 + a as u128 * x as u128 + b as u128) % p as u128) as u64
    }

    fn character(&self, x: u64) -> i8 {
        let value = self.rhs(x);
        if value == 0 {
            0
        } else if self.roots[value as usize].is_empty() {
            -1
        } else {
            1
        }
    }

    fn canonical(&self, x: u64) -> Result<Point, String> {
        if x >= self.params.p || self.character(x) != 1 {
            return Err("Independent verifier received a non-base coordinate".into());
        }
        Ok(Point {
            x,
            y: self.roots[self.rhs(x) as usize][0],
        })
    }

    fn on_curve(&self, point: Element) -> bool {
        point.is_none_or(|p| {
            p.x < self.params.p && p.y < self.params.p && p.y * p.y % self.params.p == self.rhs(p.x)
        })
    }

    fn add(&self, left: Element, right: Element) -> Result<Element, String> {
        if !self.on_curve(left) || !self.on_curve(right) {
            return Err("Independent addition received an off-curve point".into());
        }
        let (Some(l), Some(r)) = (left, right) else {
            return Ok(left.or(right));
        };
        let p = self.params.p;
        if l.x == r.x && (l.y + r.y) % p == 0 {
            return Ok(None);
        }
        let (numerator, denominator) = if l.x == r.x {
            if l != r {
                return Err("Invalid same-abscissa points".into());
            }
            (
                3 * l.x as i128 * l.x as i128 + self.params.a as i128,
                2 * l.y as i128,
            )
        } else {
            (r.y as i128 - l.y as i128, r.x as i128 - l.x as i128)
        };
        let slope = residue(numerator * inverse(residue(denominator, p), p)? as i128, p);
        let x = residue(slope as i128 * slope as i128 - l.x as i128 - r.x as i128, p);
        let y = residue(slope as i128 * (l.x as i128 - x as i128) - l.y as i128, p);
        let answer = Some(Point { x, y });
        if !self.on_curve(answer) {
            return Err("Independent group law produced an off-curve point".into());
        }
        Ok(answer)
    }

    fn base_mask(&self, base: &[Point], target_size: usize) -> Result<u128, String> {
        if base.len() > target_size {
            return Err("Base exceeds its declared target size".into());
        }
        let mut mask = 0;
        let mut previous = None;
        for &point in base {
            if previous.is_some_and(|x| x >= point.x) || self.canonical(point.x)? != point {
                return Err("Base ordering, sign, or coordinate is not canonical".into());
            }
            previous = Some(point.x);
            let index = self
                .xs
                .binary_search(&point.x)
                .map_err(|_| "Unknown base x")?;
            mask |= 1u128 << index;
        }
        Ok(mask)
    }

    fn points_for_mask(&self, mask: u128) -> Result<Vec<Point>, String> {
        self.xs
            .iter()
            .enumerate()
            .filter(|(i, _)| mask & (1u128 << i) != 0)
            .map(|(_, &x)| self.canonical(x))
            .collect()
    }

    fn collision_free(&self, mask: u128) -> Result<bool, String> {
        let points = self.points_for_mask(mask)?;
        let mut seen = BTreeMap::new();
        for (i, &left) in points.iter().enumerate() {
            for &right in &points[i..] {
                if seen
                    .insert(key(self.add(Some(left), Some(right))?), ())
                    .is_some()
                {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn transition(&self, mask: u128, x: u64, method: &str) -> Result<u128, String> {
        if self.character(x) != 1 {
            return Ok(mask);
        }
        let threshold = match method {
            "twist_l1" => 1,
            "twist_l2" => 2,
            "twist_l3" => 3,
            "twist_l4" => 4,
            "random" | "progression" | "collision_avoid" => 0,
            _ => return Err("Independent verifier received an unknown selector".into()),
        };
        for offset in 1..=threshold {
            if self.character((x + offset) % self.params.p) != -1 {
                return Ok(mask);
            }
        }
        let index = self
            .xs
            .binary_search(&x)
            .map_err(|_| "Unknown candidate coordinate")?;
        let candidate = mask | (1u128 << index);
        if method == "collision_avoid" && !self.collision_free(candidate)? {
            return Ok(mask);
        }
        Ok(candidate)
    }
}

/// Independently count every finite selector word.  Implementation work fields
/// are deliberately outside this verifier's asserted coverage.
pub fn verify_selection(
    params: Parameters,
    base_size: usize,
    method: &str,
    budget: usize,
    audit: &SelectionAudit,
) -> Result<VerificationScope, String> {
    let curve = SquareCurve::new(params)?;
    if !(2..=3).contains(&base_size) || budget > 16 || audit.method != method {
        return Err("Invalid selector verification contract".into());
    }
    if ![
        "random",
        "progression",
        "collision_avoid",
        "twist_l1",
        "twist_l2",
        "twist_l3",
        "twist_l4",
    ]
    .contains(&method)
    {
        return Err("Unknown selector verification contract".into());
    }
    let mut expected = BTreeMap::<u128, u128>::new();
    let total_words;
    if method == "progression" {
        total_words = params.p as u128 * (params.p - 1) as u128;
        for start in 0..params.p {
            for step in 1..params.p {
                let mut mask = 0u128;
                let mut x = start;
                for _ in 0..budget {
                    if mask.count_ones() as usize == base_size {
                        break;
                    }
                    mask = curve.transition(mask, x, method)?;
                    x = (x + step) % params.p;
                }
                *expected.entry(mask).or_default() += 1;
            }
        }
    } else {
        total_words = (params.p as u128)
            .checked_pow(budget as u32)
            .ok_or("Selector word count overflows independent verifier")?;
        expected.insert(0, 1);
        for _ in 0..budget {
            let mut next = BTreeMap::new();
            for (&mask, &ways) in &expected {
                if mask.count_ones() as usize == base_size {
                    *next.entry(mask).or_default() += ways * params.p as u128;
                } else {
                    for x in 0..params.p {
                        *next.entry(curve.transition(mask, x, method)?).or_default() += ways;
                    }
                }
            }
            expected = next;
        }
    }
    let mut actual = BTreeMap::new();
    for outcome in &audit.outcomes {
        let mask = curve.base_mask(&outcome.base, base_size)?;
        if outcome.ways == 0 || actual.insert(mask, outcome.ways).is_some() {
            return Err("Selector has zero-weight or duplicate outcomes".into());
        }
    }
    if audit.total_words != total_words
        || expected.values().sum::<u128>() != total_words
        || actual != expected
    {
        return Err("Independent exhaustive selector distribution disagrees".into());
    }
    Ok(VerificationScope {
        group_arithmetic_independent: true,
        selector_multiplicities_independent: true,
        relation_equalities_independent: false,
        rank_histograms_independent: false,
        digest_independent: false,
        work_counters_independent: false,
        scope_note: "Exact canonical-base probabilities and every success/failure outcome; implementation-level work counters are not independently verified.".into(),
    })
}

fn batch_rank(rows: &[Vec<u64>], modulus: u64, columns: usize) -> Result<usize, String> {
    let mut matrix = Vec::with_capacity(rows.len());
    for row in rows {
        if row.len() < columns || row.iter().any(|&value| value >= modulus) {
            return Err("Invalid independent matrix row".into());
        }
        matrix.push(row[..columns].to_vec());
    }
    let mut rank = 0;
    for column in 0..columns {
        let Some(pivot) = (rank..matrix.len()).find(|&row| matrix[row][column] != 0) else {
            continue;
        };
        matrix.swap(rank, pivot);
        let multiplier = inverse(matrix[rank][column], modulus)?;
        for entry in &mut matrix[rank] {
            *entry = ((*entry as u128 * multiplier as u128) % modulus as u128) as u64;
        }
        let pivot_row = matrix[rank].clone();
        for (row_index, row) in matrix.iter_mut().enumerate() {
            if row_index == rank {
                continue;
            }
            let factor = row[column];
            for (entry, &basis) in row.iter_mut().zip(&pivot_row) {
                *entry = residue(*entry as i128 - factor as i128 * basis as i128, modulus);
            }
        }
        rank += 1;
    }
    Ok(rank)
}

#[derive(Clone)]
struct IndependentState {
    a: u64,
    b: u64,
    current: Element,
    rows: Vec<Vec<u64>>,
    rank: usize,
    hits: u128,
    dependent: u128,
    first_prefix: Vec<Option<usize>>,
}

struct IndependentMilestone {
    attained: u128,
    missing: u128,
    first_prefix_hist: BTreeMap<usize, u128>,
}

struct IndependentAudit {
    rank_hist: Vec<Vec<u128>>,
    hits: Vec<u128>,
    dependent: Vec<u128>,
    milestones: Vec<IndependentMilestone>,
    full_rank: u128,
    exhausted: u128,
}

struct PublicContext<'a> {
    curve: &'a SquareCurve,
    order: u64,
    g: Point,
    q: Point,
    g_chain: &'a [Element],
    q_chain: &'a [Element],
    base: &'a [Point],
    pairs: &'a BTreeMap<PointKey, (usize, usize)>,
    walk_length: usize,
}

fn put_u64(digest: &mut Sha256, value: u64) {
    digest.update(value.to_le_bytes());
}

fn digest_node(
    digest: &mut Sha256,
    state: &IndependentState,
    depth: usize,
    direction: u8,
    pair: Option<(usize, usize)>,
    weight: u128,
) {
    digest.update(b"N");
    put_u64(digest, depth as u64);
    digest.update([direction]);
    put_u64(digest, state.a);
    put_u64(digest, state.b);
    match state.current {
        None => digest.update([0]),
        Some(point) => {
            digest.update([1]);
            put_u64(digest, point.x);
            put_u64(digest, point.y);
        }
    }
    match pair {
        None => digest.update([0]),
        Some((i, j)) => {
            digest.update([1]);
            put_u64(digest, i as u64);
            put_u64(digest, j as u64);
        }
    }
    put_u64(digest, state.rank as u64);
    digest.update(state.hits.to_le_bytes());
    digest.update(state.dependent.to_le_bytes());
    digest.update(weight.to_le_bytes());
}

fn record_independent_prefix(
    expected: &mut IndependentAudit,
    state: &IndependentState,
    depth: usize,
    weight: u128,
) {
    expected.rank_hist[depth][state.rank] += weight;
    expected.hits[depth] += state.hits * weight;
    expected.dependent[depth] += state.dependent * weight;
}

fn inspect_trie(
    context: &PublicContext<'_>,
    state: IndependentState,
    depth: usize,
    direction: u8,
    pair: Option<(usize, usize)>,
    expected: &mut IndependentAudit,
    digest: &mut Sha256,
) -> Result<(), String> {
    let weight = 1u128 << (context.walk_length - depth);
    record_independent_prefix(expected, &state, depth, weight);
    digest_node(digest, &state, depth, direction, pair, weight);
    let dimension = context.base.len() + 1;
    if state.rank == dimension || depth == context.walk_length {
        if state.rank == dimension {
            expected.full_rank += weight;
            for later in depth + 1..=context.walk_length {
                record_independent_prefix(expected, &state, later, weight);
            }
        } else {
            expected.exhausted += weight;
        }
        for (index, milestone) in expected.milestones.iter_mut().enumerate() {
            match state.first_prefix[index] {
                Some(first) => {
                    milestone.attained += weight;
                    *milestone.first_prefix_hist.entry(first).or_default() += weight;
                }
                None => milestone.missing += weight,
            }
        }
        return Ok(());
    }
    for bit in [0, 1] {
        let mut next = state.clone();
        let step = if bit == 0 {
            next.a = (next.a + 1) % context.order;
            context.g
        } else {
            next.b = (next.b + 1) % context.order;
            context.q
        };
        next.current = context.curve.add(next.current, Some(step))?;
        let public_sum = context.curve.add(
            context.g_chain[next.a as usize],
            context.q_chain[next.b as usize],
        )?;
        if next.current != public_sum {
            return Err("Independent tracked public query disagrees with its coefficients".into());
        }
        let incoming = context.pairs.get(&key(next.current)).copied();
        if let Some((i, j)) = incoming {
            if context
                .curve
                .add(Some(context.base[i]), Some(context.base[j]))?
                != public_sum
            {
                return Err("Independent relation fails its public group equality".into());
            }
            next.hits += 1;
            let mut row = vec![0; dimension + 1];
            row[i] += 1;
            row[j] += 1;
            row[context.base.len()] = (context.order - next.b) % context.order;
            row[dimension] = next.a;
            next.rows.push(row);
            let new_rank = batch_rank(&next.rows, context.order, dimension)?;
            if batch_rank(&next.rows, context.order, dimension + 1)? != new_rank {
                return Err("Independent augmented relation matrix is inconsistent".into());
            }
            if new_rank == next.rank {
                next.dependent += 1;
            } else if new_rank == next.rank + 1 {
                next.first_prefix[new_rank - 1] = Some(depth + 1);
                next.rank = new_rank;
            } else {
                return Err("A single valid row produced an impossible rank jump".into());
            }
        }
        inspect_trie(context, next, depth + 1, bit, incoming, expected, digest)?;
    }
    Ok(())
}

fn generator_chain(curve: &SquareCurve, generator: Point) -> Result<Vec<Element>, String> {
    let mut chain = Vec::with_capacity(curve.order as usize);
    let mut point = None;
    let mut seen = BTreeMap::new();
    for _ in 0..curve.order {
        if seen.insert(key(point), ()).is_some() {
            return Err("Independent generator chain closed early".into());
        }
        chain.push(point);
        point = curve.add(point, Some(generator))?;
    }
    if point.is_some() {
        return Err("Independent generator chain does not close at the curve order".into());
    }
    Ok(chain)
}

fn work_fields(work: &Work) -> Result<BTreeMap<String, u128>, String> {
    let value = serde_json::to_value(work).map_err(|e| e.to_string())?;
    let object = value.as_object().ok_or("Work is not an object")?;
    object
        .iter()
        .map(|(name, value)| {
            let decimal = value
                .as_str()
                .ok_or("Work counter is not an exact decimal string")?;
            decimal
                .parse()
                .map(|number| (name.clone(), number))
                .map_err(|_| "Invalid decimal work counter".into())
        })
        .collect()
}

/// This is storage/aggregation reconciliation, not independent verification
/// that the producer emitted every operation required by its implementation.
fn reconcile_work_storage(audit: &RelationAudit) -> Result<(), String> {
    let total = work_fields(&audit.total_work_sum)?;
    let phases = [
        work_fields(&audit.pair_setup_work_sum)?,
        work_fields(&audit.search_work_sum)?,
        work_fields(&audit.linear_work_sum)?,
        work_fields(&audit.verification_work_sum)?,
    ];
    let setup_per_case = work_fields(&audit.pair_setup_work_per_case)?;
    for (name, &value) in &total {
        let sum = phases.iter().try_fold(0u128, |sum, fields| {
            sum.checked_add(fields[name])
                .ok_or("Independent phase sum overflow")
        })?;
        if sum != value
            || setup_per_case[name].checked_mul(audit.observations) != Some(phases[0][name])
            || (audit.setup_prepaid && setup_per_case[name] != 0)
        {
            return Err("Independent work aggregation reconciliation failed".into());
        }
    }
    for milestone in &audit.milestones {
        let work = work_fields(&milestone.work_sum)?;
        let attained_work = work_fields(&milestone.attained_work_sum)?;
        let missing_work = work_fields(&milestone.missing_work_sum)?;
        for (name, &value) in &work {
            if attained_work[name].checked_add(missing_work[name]) != Some(value)
                || (milestone.attained == 0 && attained_work[name] != 0)
                || (milestone.missing == 0 && missing_work[name] != 0)
                || setup_per_case[name]
                    .checked_mul(milestone.attained)
                    .is_none_or(|minimum| attained_work[name] < minimum)
                || setup_per_case[name]
                    .checked_mul(milestone.missing)
                    .is_none_or(|minimum| missing_work[name] < minimum)
            {
                return Err(
                    "Rank milestone attained/missing work partition does not reconcile".into(),
                );
            }
        }
        if work.iter().any(|(name, &value)| value > total[name])
            || (milestone.attained == 0 && work != total)
        {
            return Err("Rank milestone work violates bounded first-passage aggregation".into());
        }
    }
    Ok(())
}

/// Independently replay all nonzero public targets and every bounded binary
/// query word.  A prefix trie preserves exact unused-tail multiplicity while
/// matrix ranks are recomputed from scratch with batch Gauss-Jordan elimination.
pub fn verify_relation(
    params: Parameters,
    order: u64,
    generator: Point,
    base: &[Point],
    walk_length: usize,
    audit: &RelationAudit,
) -> Result<VerificationScope, String> {
    let curve = SquareCurve::new(params)?;
    if curve.order != order
        || generator != curve.canonical(*curve.xs.first().ok_or("Empty original curve")?)?
        || !(2..=3).contains(&base.len())
        || walk_length > 16
    {
        return Err("Invalid independent relation verification contract".into());
    }
    curve.base_mask(base, base.len())?;
    let dimension = base.len() + 1;
    let words_per_target = 1u128 << walk_length;
    let observations = (order as u128 - 1) * words_per_target;
    if audit.schema != "twist-tr04/relation-audit/v1"
        || audit.base_size != base.len()
        || audit.walk_length != walk_length
        || audit.target_count != order - 1
        || audit.words_per_target != words_per_target
        || audit.observations != observations
        || audit.milestones.len() != dimension
    {
        return Err("Independent relation population or schema disagrees".into());
    }
    let mut pairs = BTreeMap::new();
    for (i, &left) in base.iter().enumerate() {
        for (j, &right) in base.iter().enumerate().skip(i) {
            pairs
                .entry(key(curve.add(Some(left), Some(right))?))
                .or_insert((i, j));
        }
    }
    if pairs.len() != audit.pair_table_entries {
        return Err("Independent pair-table cardinality disagrees".into());
    }
    let mut expected = IndependentAudit {
        rank_hist: vec![vec![0; dimension + 1]; walk_length + 1],
        hits: vec![0; walk_length + 1],
        dependent: vec![0; walk_length + 1],
        milestones: (0..dimension)
            .map(|_| IndependentMilestone {
                attained: 0,
                missing: 0,
                first_prefix_hist: BTreeMap::new(),
            })
            .collect(),
        full_rank: 0,
        exhausted: 0,
    };
    let mut digest = Sha256::new();
    digest.update(b"twist-tr04-relation-trie/v1\0");
    for value in [
        params.p,
        params.a,
        params.b,
        order,
        generator.x,
        generator.y,
        base.len() as u64,
        walk_length as u64,
        u64::from(audit.setup_prepaid),
        pairs.len() as u64,
    ] {
        put_u64(&mut digest, value);
    }
    for point in base {
        put_u64(&mut digest, point.x);
        put_u64(&mut digest, point.y);
    }
    let g_chain = generator_chain(&curve, generator)?;
    for (ordinal, &target) in g_chain.iter().enumerate().skip(1) {
        let q = target.ok_or("Independent target census contains the identity")?;
        digest.update(b"T");
        put_u64(&mut digest, ordinal as u64);
        put_u64(&mut digest, q.x);
        put_u64(&mut digest, q.y);
        let q_chain = generator_chain(&curve, q)?;
        let context = PublicContext {
            curve: &curve,
            order,
            g: generator,
            q,
            g_chain: &g_chain,
            q_chain: &q_chain,
            base,
            pairs: &pairs,
            walk_length,
        };
        let state = IndependentState {
            a: 0,
            b: 0,
            current: None,
            rows: Vec::new(),
            rank: 0,
            hits: 0,
            dependent: 0,
            first_prefix: vec![None; dimension],
        };
        inspect_trie(&context, state, 0, 255, None, &mut expected, &mut digest)?;
    }
    let computed_digest = format!("{:x}", digest.finalize());
    if audit.rank_hist_by_prefix != expected.rank_hist
        || audit.hits_sum_by_prefix != expected.hits
        || audit.dependent_hits_sum_by_prefix != expected.dependent
        || audit.full_rank_words != expected.full_rank
        || audit.budget_exhausted_words != expected.exhausted
        || audit.relation_digest != computed_digest
    {
        return Err("Independent exhaustive relation transcript or rank profile disagrees".into());
    }
    for (index, (actual, expected)) in audit
        .milestones
        .iter()
        .zip(&expected.milestones)
        .enumerate()
    {
        if actual.rank != index + 1
            || actual.attained != expected.attained
            || actual.missing != expected.missing
            || actual.first_prefix_hist != expected.first_prefix_hist
        {
            return Err("Independent first-passage rank census disagrees".into());
        }
    }
    reconcile_work_storage(audit)?;
    Ok(VerificationScope {
        group_arithmetic_independent: true,
        selector_multiplicities_independent: false,
        relation_equalities_independent: true,
        rank_histograms_independent: true,
        digest_independent: true,
        work_counters_independent: false,
        scope_note: "Exact public equalities, every target/word, absorbing-stop rank and first-passage histograms, and structural digest. Exclusive phases and attained/missing first-passage work partitions are reconciled, but primitive counter completeness and prepaid-setup provenance are not independently verified.".into(),
    })
}

/// A literal control independent of the production selector and relation
/// engine.  Its three rows have determinant four modulo thirteen and hence
/// full coefficient rank; all three are also checked as public point sums.
pub fn verify_fixed_controls() -> Result<VerificationScope, String> {
    let params = Parameters { p: 7, a: 0, b: 3 };
    let curve = SquareCurve::new(params)?;
    let generator = Point { x: 1, y: 2 };
    let base = [generator, Point { x: 2, y: 2 }];
    if curve.order != 13 || curve.canonical(curve.xs[0])? != generator {
        return Err("Fixed independent control has wrong order or generator".into());
    }
    let chain = generator_chain(&curve, generator)?;
    let mut pairs = BTreeMap::new();
    for (i, &left) in base.iter().enumerate() {
        for (j, &right) in base.iter().enumerate().skip(i) {
            pairs
                .entry(key(curve.add(Some(left), Some(right))?))
                .or_insert((i, j));
        }
    }
    let (mut a, mut b, mut current) = (0, 0, None);
    let mut prefix_rows = Vec::new();
    let mut rows = Vec::new();
    for (position, bit) in [0, 0, 1, 1, 0, 0, 0, 0].into_iter().enumerate() {
        if bit == 0 {
            a = (a + 1) % 13;
        } else {
            b = (b + 1) % 13;
        }
        current = curve.add(current, Some(generator))?;
        if current != chain[((a + b) % 13) as usize] {
            return Err("Fixed control coefficients disagree with its public point".into());
        }
        if let Some(&(i, j)) = pairs.get(&key(current)) {
            if curve.add(Some(base[i]), Some(base[j]))? != current {
                return Err("Fixed control relation equality failed".into());
            }
            let mut row = vec![0; 4];
            row[i] += 1;
            row[j] += 1;
            row[2] = (13 - b) % 13;
            row[3] = a;
            prefix_rows.push((position + 1, row.clone()));
            rows.push(row);
            if batch_rank(&rows, 13, 3)? == 3 {
                break;
            }
        }
    }
    let required = vec![
        (2, vec![2, 0, 0, 2]),
        (4, vec![1, 1, 11, 2]),
        (6, vec![0, 2, 11, 4]),
    ];
    if prefix_rows != required || batch_rank(&rows, 13, 3)? != 3 || batch_rank(&rows, 13, 4)? != 3 {
        return Err("Fixed independent control rows or ranks disagree".into());
    }
    let r = &rows;
    let determinant = residue(
        r[0][0] as i128 * (r[1][1] as i128 * r[2][2] as i128 - r[1][2] as i128 * r[2][1] as i128)
            - r[0][1] as i128
                * (r[1][0] as i128 * r[2][2] as i128 - r[1][2] as i128 * r[2][0] as i128)
            + r[0][2] as i128
                * (r[1][0] as i128 * r[2][1] as i128 - r[1][1] as i128 * r[2][0] as i128),
        13,
    );
    if determinant != 4 {
        return Err("Fixed independent control determinant is not four".into());
    }
    Ok(VerificationScope {
        group_arithmetic_independent: true,
        selector_multiplicities_independent: false,
        relation_equalities_independent: true,
        rank_histograms_independent: false,
        digest_independent: false,
        work_counters_independent: false,
        scope_note: "Literal p=7,a=0,b=3,n=13,G=Q=(1,2), base[(1,2),(2,2)], GGQQGGGG control: public identities at prefixes2/4/6, coefficient rows [2,0,0],[1,1,11],[0,2,11], determinant4 and rank3 modulo13; no operation-cost claim.".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const PARAMS: Parameters = Parameters { p: 7, a: 0, b: 3 };

    #[test]
    fn independent_model_census_rejects_singularity_order_and_generator_tampering() {
        let eligible = model_info(PARAMS).unwrap();
        assert!(eligible.nonsingular && eligible.eligible);
        assert_eq!(eligible.group_order, Some(13));
        assert_eq!(eligible.canonical_generator, Some(Point { x: 1, y: 2 }));
        verify_model_info(PARAMS, &eligible).unwrap();
        let mut changed = eligible.clone();
        changed.group_order = Some(11);
        assert!(verify_model_info(PARAMS, &changed).is_err());
        changed = eligible;
        changed.canonical_generator = Some(Point { x: 1, y: 5 });
        assert!(verify_model_info(PARAMS, &changed).is_err());
        let singular_params = Parameters { p: 7, a: 0, b: 0 };
        let mut singular = model_info(singular_params).unwrap();
        assert!(!singular.nonsingular && !singular.eligible);
        assert_eq!(singular.group_order, None);
        singular.group_order = Some(7);
        assert!(verify_model_info(singular_params, &singular).is_err());
        let composite = model_info(Parameters { p: 7, a: 0, b: 1 }).unwrap();
        assert_eq!(composite.group_order, Some(12));
        assert!(!composite.eligible);
        assert_eq!(composite.canonical_generator, None);
    }

    #[test]
    fn fixed_public_relation_control_has_three_independent_rows() {
        let scope = verify_fixed_controls().unwrap();
        assert!(scope.relation_equalities_independent);
        assert!(!scope.work_counters_independent);
    }

    #[test]
    fn independent_square_table_and_group_chain_cover_the_tiny_curve() {
        let curve = SquareCurve::new(PARAMS).unwrap();
        assert_eq!(curve.order, 13);
        let generator = curve.canonical(curve.xs[0]).unwrap();
        let chain = generator_chain(&curve, generator).unwrap();
        for (i, &left) in chain.iter().enumerate() {
            for (j, &right) in chain.iter().enumerate() {
                assert_eq!(curve.add(left, right).unwrap(), chain[(i + j) % 13]);
            }
        }
    }

    #[test]
    fn independent_selector_rejects_multiplicity_sign_and_missing_outcome_tampering() {
        for method in [
            "random",
            "progression",
            "twist_l1",
            "twist_l2",
            "twist_l3",
            "twist_l4",
            "collision_avoid",
        ] {
            let original = crate::tr04_select::selection(PARAMS, 2, method, 3).unwrap();
            let scope = verify_selection(PARAMS, 2, method, 3, &original).unwrap();
            assert!(scope.selector_multiplicities_independent);
            assert!(!scope.work_counters_independent);
            let mut changed = original.clone();
            changed.outcomes[0].ways += 1;
            assert!(verify_selection(PARAMS, 2, method, 3, &changed).is_err());
            changed = original.clone();
            changed.outcomes.pop();
            assert!(verify_selection(PARAMS, 2, method, 3, &changed).is_err());
        }
        let mut changed = crate::tr04_select::selection(PARAMS, 2, "random", 3).unwrap();
        let point = changed
            .outcomes
            .iter_mut()
            .find_map(|row| row.base.first_mut())
            .unwrap();
        point.y = PARAMS.p - point.y;
        assert!(verify_selection(PARAMS, 2, "random", 3, &changed).is_err());
    }

    #[test]
    fn independent_batch_ranks_and_digest_reject_relation_tampering() {
        let curve = SquareCurve::new(PARAMS).unwrap();
        let generator = curve.canonical(curve.xs[0]).unwrap();
        let base: Vec<_> = curve
            .xs
            .iter()
            .take(2)
            .map(|&x| curve.canonical(x).unwrap())
            .collect();
        let original = crate::tr04_relation::audit(PARAMS, 13, generator, &base, 4, false).unwrap();
        let scope = verify_relation(PARAMS, 13, generator, &base, 4, &original).unwrap();
        assert!(scope.rank_histograms_independent && scope.digest_independent);
        assert!(!scope.work_counters_independent);
        let mut changed = original.clone();
        changed.rank_hist_by_prefix[1][0] += 1;
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());
        changed = original.clone();
        changed.milestones[0].attained += 1;
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());
        changed = original.clone();
        changed.relation_digest = "00".repeat(32);
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());
        changed = original;
        changed.total_work_sum.group_additions += 1;
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());
    }

    #[test]
    fn three_point_collision_control_and_first_pair_policy_are_replayed() {
        for method in [
            "random",
            "progression",
            "twist_l1",
            "twist_l2",
            "twist_l3",
            "twist_l4",
            "collision_avoid",
        ] {
            let audit = crate::tr04_select::selection(PARAMS, 3, method, 4).unwrap();
            verify_selection(PARAMS, 3, method, 4, &audit).unwrap();
        }
        let curve = SquareCurve::new(PARAMS).unwrap();
        let generator = curve.canonical(curve.xs[0]).unwrap();
        let base: Vec<_> = curve
            .xs
            .iter()
            .take(3)
            .map(|&x| curve.canonical(x).unwrap())
            .collect();
        let mask = curve.base_mask(&base, 3).unwrap();
        assert!(!curve.collision_free(mask).unwrap());
        let audit = crate::tr04_relation::audit(PARAMS, 13, generator, &base, 5, false).unwrap();
        assert!(audit.pair_table_entries < 6);
        verify_relation(PARAMS, 13, generator, &base, 5, &audit).unwrap();
    }

    #[test]
    fn zero_budget_and_prepaid_flags_are_explicitly_covered() {
        let curve = SquareCurve::new(PARAMS).unwrap();
        let generator = curve.canonical(curve.xs[0]).unwrap();
        let base: Vec<_> = curve
            .xs
            .iter()
            .take(3)
            .map(|&x| curve.canonical(x).unwrap())
            .collect();
        for prepaid in [false, true] {
            let audit =
                crate::tr04_relation::audit(PARAMS, 13, generator, &base, 0, prepaid).unwrap();
            verify_relation(PARAMS, 13, generator, &base, 0, &audit).unwrap();
        }
        for method in ["random", "progression"] {
            let audit = crate::tr04_select::selection(PARAMS, 2, method, 0).unwrap();
            verify_selection(PARAMS, 2, method, 0, &audit).unwrap();
        }
    }

    #[test]
    fn attained_and_missing_work_partitions_cannot_hide_costs() {
        let curve = SquareCurve::new(PARAMS).unwrap();
        let generator = curve.canonical(curve.xs[0]).unwrap();
        let base: Vec<_> = curve
            .xs
            .iter()
            .take(2)
            .map(|&x| curve.canonical(x).unwrap())
            .collect();
        let original = crate::tr04_relation::audit(PARAMS, 13, generator, &base, 4, false).unwrap();
        verify_relation(PARAMS, 13, generator, &base, 4, &original).unwrap();
        let mut changed = original.clone();
        changed.milestones[0].attained_work_sum.group_additions += 1;
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());
        changed = original;
        assert!(changed.milestones[0].missing > 0 && changed.milestones[0].attained > 0);
        changed.milestones[0].missing_work_sum = Work::default();
        changed.milestones[0].attained_work_sum = changed.milestones[0].work_sum.clone();
        assert!(verify_relation(PARAMS, 13, generator, &base, 4, &changed).is_err());

        let mut zero_budget =
            crate::tr04_relation::audit(PARAMS, 13, generator, &base, 0, false).unwrap();
        assert_eq!(zero_budget.milestones[0].attained, 0);
        zero_budget.milestones[0].attained_work_sum.group_additions = 1;
        zero_budget.milestones[0].missing_work_sum.group_additions -= 1;
        assert!(verify_relation(PARAMS, 13, generator, &base, 0, &zero_budget).is_err());
    }
}
