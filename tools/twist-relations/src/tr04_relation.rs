//! Exhaustive, bounded public relation-query tries for TR-04.
//!
//! Each conceptual workload is one public target and one binary G/Q word.
//! Full coefficient rank is absorbing: unread word suffixes retain multiplicity
//! without paid work. Trie sharing is an audit acceleration, not an algorithm
//! runtime. Primitive counters model an ordinary cold execution of every word.
use crate::core::{prime, Curve, Element, Parameters, Point};
use crate::tr04_select::{increment, pair_table, Meter, PairTable, Work};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankMilestone {
    pub rank: usize,
    #[serde(with = "decimal")]
    pub attained: u128,
    #[serde(with = "decimal")]
    pub missing: u128,
    #[serde(with = "decimal_hist")]
    pub first_prefix_hist: BTreeMap<usize, u128>,
    /// Every observation contributes: first-attainment work if attained,
    /// otherwise its entire bounded execution. Never a survivor-only sum.
    pub work_sum: Work,
    /// First-attainment work restricted to attained observations; descriptive.
    pub attained_work_sum: Work,
    /// Full-budget work restricted to observations missing this rank.
    pub missing_work_sum: Work,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationAudit {
    pub schema: String,
    pub base_size: usize,
    pub walk_length: usize,
    pub target_count: u64,
    #[serde(with = "decimal")]
    pub words_per_target: u128,
    #[serde(with = "decimal")]
    pub observations: u128,
    pub setup_prepaid: bool,
    pub pair_table_entries: usize,
    pub pair_setup_work_per_case: Work,
    #[serde(with = "decimal_matrix")]
    pub rank_hist_by_prefix: Vec<Vec<u128>>,
    #[serde(with = "decimal_vec")]
    pub hits_sum_by_prefix: Vec<u128>,
    #[serde(with = "decimal_vec")]
    pub dependent_hits_sum_by_prefix: Vec<u128>,
    pub pair_setup_work_sum: Work,
    pub search_work_sum: Work,
    pub linear_work_sum: Work,
    pub verification_work_sum: Work,
    pub total_work_sum: Work,
    pub milestones: Vec<RankMilestone>,
    #[serde(with = "decimal")]
    pub full_rank_words: u128,
    #[serde(with = "decimal")]
    pub budget_exhausted_words: u128,
    /// SHA-256 of the documented public trie transcript; excludes costs.
    pub relation_digest: String,
}

#[derive(Clone)]
struct State {
    current: Element,
    a: u64,
    b: u64,
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    hits: u128,
    dependent_hits: u128,
    work: Work,
    search_work: Work,
    linear_work: Work,
    verification_work: Work,
    first_prefix: Vec<Option<usize>>,
    first_work: Vec<Option<Work>>,
}

struct Context<'a> {
    parameters: Parameters,
    n: u64,
    g: Point,
    q: Point,
    base: &'a [Point],
    table: &'a PairTable,
    walk_length: usize,
}

/// Enumerate all nonzero Q by public additions and all length-W binary words.
/// `setup_prepaid` is valid only when selection already paid for and retained
/// the identical canonical pair table (the collision-avoid arm).
pub fn audit(
    parameters: Parameters,
    n: u64,
    g: Point,
    base: &[Point],
    walk_length: usize,
    setup_prepaid: bool,
) -> Result<RelationAudit, String> {
    validate_inputs(parameters, n, g, base, walk_length)?;
    let words_per_target = 1u128
        .checked_shl(walk_length as u32)
        .ok_or("walk word census overflow")?;
    let observations = (n as u128 - 1)
        .checked_mul(words_per_target)
        .ok_or("relation census overflow")?;

    // This one construction is memoized audit work. The cold algorithm's
    // construction is charged separately to every conceptual observation.
    let mut table_meter = Meter::new(parameters);
    let table = pair_table(&mut table_meter, base)?;
    let setup = if setup_prepaid {
        Work::default()
    } else {
        table_meter.work
    };
    let dimension = base.len() + 1;
    let mut result = RelationAudit {
        schema: "twist-tr04/relation-audit/v1".into(),
        base_size: base.len(),
        walk_length,
        target_count: n - 1,
        words_per_target,
        observations,
        setup_prepaid,
        pair_table_entries: table.len(),
        pair_setup_work_per_case: setup.clone(),
        rank_hist_by_prefix: vec![vec![0; dimension + 1]; walk_length + 1],
        hits_sum_by_prefix: vec![0; walk_length + 1],
        dependent_hits_sum_by_prefix: vec![0; walk_length + 1],
        pair_setup_work_sum: Work::default(),
        search_work_sum: Work::default(),
        linear_work_sum: Work::default(),
        verification_work_sum: Work::default(),
        total_work_sum: Work::default(),
        milestones: (1..=dimension)
            .map(|rank| RankMilestone {
                rank,
                attained: 0,
                missing: 0,
                first_prefix_hist: BTreeMap::new(),
                work_sum: Work::default(),
                attained_work_sum: Work::default(),
                missing_work_sum: Work::default(),
            })
            .collect(),
        full_rank_words: 0,
        budget_exhausted_words: 0,
        relation_digest: String::new(),
    };
    let mut digest = Sha256::new();
    digest.update(b"twist-tr04-relation-trie/v1\0");
    for value in [
        parameters.p,
        parameters.a,
        parameters.b,
        n,
        g.x,
        g.y,
        base.len() as u64,
        walk_length as u64,
        u64::from(setup_prepaid),
        table.len() as u64,
    ] {
        put_u64(&mut digest, value);
    }
    for point in base {
        put_u64(&mut digest, point.x);
        put_u64(&mut digest, point.y);
    }

    // Public instance preparation is excluded uniformly from algorithm work.
    // The search never receives these target ordinals as scalar information.
    let mut fixture = Meter::new(parameters);
    let mut target = Some(g);
    for ordinal in 1..n {
        let q = target.ok_or("generator chain ended before the public target census")?;
        digest.update(b"T");
        put_u64(&mut digest, ordinal);
        put_u64(&mut digest, q.x);
        put_u64(&mut digest, q.y);
        let context = Context {
            parameters,
            n,
            g,
            q,
            base,
            table: &table,
            walk_length,
        };
        let initial = State {
            current: None,
            a: 0,
            b: 0,
            pivots: vec![None; dimension],
            rank: 0,
            hits: 0,
            dependent_hits: 0,
            work: setup.clone(),
            search_work: Work::default(),
            linear_work: Work::default(),
            verification_work: Work::default(),
            first_prefix: vec![None; dimension],
            first_work: vec![None; dimension],
        };
        visit(&context, initial, 0, 255, None, &mut result, &mut digest)?;
        target = fixture.add(target, Some(g))?;
    }
    if target.is_some() {
        return Err("generator chain did not close at declared group order".into());
    }
    result.relation_digest = digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    reconcile(&result)?;
    Ok(result)
}

fn validate_inputs(
    parameters: Parameters,
    n: u64,
    g: Point,
    base: &[Point],
    walk_length: usize,
) -> Result<(), String> {
    if parameters.p < 7
        || parameters.p > 127
        || parameters.p % 4 != 3
        || !prime(parameters.p)
        || parameters.a >= parameters.p
        || parameters.b >= parameters.p
        || n < 7
        || !prime(n)
        || !(2..=3).contains(&base.len())
        || walk_length > 16
    {
        return Err("invalid finite relation-audit scope".into());
    }
    let curve = Curve::new(parameters.p, parameters.a, parameters.b);
    if !curve.nonsingular() || !curve.on_curve(Some(g)) {
        return Err("invalid public curve or generator".into());
    }
    let mut last_x = None;
    for &point in base {
        if !curve.on_curve(Some(point))
            || point.y > parameters.p - point.y
            || last_x.is_some_and(|x| x >= point.x)
        {
            return Err("base must have distinct canonical points sorted by x".into());
        }
        last_x = Some(point.x);
    }
    // Validate the declared prime-order group without giving labels to search.
    let mut roots = HashSet::new();
    for y in 1..parameters.p {
        roots.insert(y * y % parameters.p);
    }
    let point_count = 1
        + (0..parameters.p)
            .map(|x| {
                let value = curve.f(x);
                if value == 0 {
                    1
                } else if roots.contains(&value) {
                    2
                } else {
                    0
                }
            })
            .sum::<u64>();
    if point_count != n {
        return Err("declared order does not match the public curve".into());
    }
    Ok(())
}

fn visit(
    context: &Context<'_>,
    state: State,
    depth: usize,
    direction: u8,
    incoming_pair: Option<(usize, usize)>,
    result: &mut RelationAudit,
    digest: &mut Sha256,
) -> Result<(), String> {
    let weight = 1u128 << (context.walk_length - depth);
    record_prefix(result, &state, depth, weight)?;
    record_node(digest, &state, depth, direction, incoming_pair, weight);
    if state.rank == context.base.len() + 1 {
        // All unused suffixes are uncharged; every later prefix retains the
        // same rank and hit count for every word represented by this node.
        for later in depth + 1..=context.walk_length {
            record_prefix(result, &state, later, weight)?;
        }
        finish(result, &state, weight)?;
        return Ok(());
    }
    if depth == context.walk_length {
        finish(result, &state, weight)?;
        return Ok(());
    }
    for bit in [0, 1] {
        let (next, pair) = advance(context, state.clone(), depth + 1, bit)?;
        visit(context, next, depth + 1, bit, pair, result, digest)?;
    }
    Ok(())
}

fn record_prefix(
    result: &mut RelationAudit,
    state: &State,
    depth: usize,
    weight: u128,
) -> Result<(), String> {
    increment(&mut result.rank_hist_by_prefix[depth][state.rank], weight)?;
    increment(
        &mut result.hits_sum_by_prefix[depth],
        state
            .hits
            .checked_mul(weight)
            .ok_or("weighted hit overflow")?,
    )?;
    increment(
        &mut result.dependent_hits_sum_by_prefix[depth],
        state
            .dependent_hits
            .checked_mul(weight)
            .ok_or("weighted dependent-hit overflow")?,
    )
}

fn advance(
    context: &Context<'_>,
    mut state: State,
    depth: usize,
    bit: u8,
) -> Result<(State, Option<(usize, usize)>), String> {
    let mut search = Meter::new(context.parameters);
    increment(&mut search.work.linear_adds, 1)?;
    increment(&mut search.work.linear_reductions, 1)?;
    let step = if bit == 0 {
        state.a = (state.a + 1) % context.n;
        context.g
    } else {
        state.b = (state.b + 1) % context.n;
        context.q
    };
    state.current = search.add(state.current, Some(step))?;
    let pair = context.table.lookup(&mut search, state.current)?;
    state.work.accumulate(&search.work)?;
    state.search_work.accumulate(&search.work)?;
    let Some((i, j)) = pair else {
        return Ok((state, None));
    };
    increment(&mut state.hits, 1)?;

    // This fresh public equality replay is paid verification work. It never
    // uses a target scalar or a base-log oracle.
    let mut check = Meter::new(context.parameters);
    let left = check.mul(state.a, Some(context.g))?;
    let right = check.mul(state.b, Some(context.q))?;
    let query = check.add(left, right)?;
    let sum = check.add(Some(context.base[i]), Some(context.base[j]))?;
    if !check.same_point(query, sum)? || !check.same_point(state.current, sum)? {
        return Err("emitted relation fails its public group equality".into());
    }
    state.work.accumulate(&check.work)?;
    state.verification_work.accumulate(&check.work)?;

    let mut linear = Work::default();
    let size = context.base.len();
    let mut row = vec![0; size + 2];
    increment(&mut linear.linear_cells, (size + 6) as u128)?;
    row[i] = linear_add(row[i], 1, context.n, &mut linear)?;
    row[j] = linear_add(row[j], 1, context.n, &mut linear)?;
    row[size] = linear_sub(0, state.b, context.n, &mut linear)?;
    row[size + 1] = state.a;
    let inserted = insert_row(&mut state.pivots, row, context.n, &mut linear)?;
    state.work.accumulate(&linear)?;
    state.linear_work.accumulate(&linear)?;
    if inserted {
        state.rank += 1;
        state.first_prefix[state.rank - 1] = Some(depth);
        state.first_work[state.rank - 1] = Some(state.work.clone());
    } else {
        increment(&mut state.dependent_hits, 1)?;
    }
    Ok((state, pair))
}

fn insert_row(
    pivots: &mut [Option<Vec<u64>>],
    mut row: Vec<u64>,
    n: u64,
    work: &mut Work,
) -> Result<bool, String> {
    let width = row.len();
    for col in 0..pivots.len() {
        if row[col] == 0 {
            continue;
        }
        if let Some(old) = &pivots[col] {
            let scale = row[col];
            for at in col..width {
                let product = linear_mul(scale, old[at], n, work)?;
                row[at] = linear_sub(row[at], product, n, work)?;
                increment(&mut work.linear_cells, 1)?;
            }
        } else {
            increment(&mut work.linear_inversions, 1)?;
            let inverse = linear_pow(row[col], n - 2, n, work)?;
            for value in &mut row[col..] {
                *value = linear_mul(*value, inverse, n, work)?;
                increment(&mut work.linear_cells, 1)?;
            }
            increment(&mut work.linear_cells, width as u128)?;
            pivots[col] = Some(row);
            return Ok(true);
        }
    }
    if row[width - 1] != 0 {
        return Err("valid public relations produced an inconsistent row".into());
    }
    Ok(false)
}

fn linear_add(a: u64, b: u64, n: u64, work: &mut Work) -> Result<u64, String> {
    increment(&mut work.linear_adds, 1)?;
    increment(&mut work.linear_reductions, 1)?;
    Ok(((a as u128 + b as u128) % n as u128) as u64)
}
fn linear_sub(a: u64, b: u64, n: u64, work: &mut Work) -> Result<u64, String> {
    increment(&mut work.linear_adds, 1)?;
    increment(&mut work.linear_reductions, 1)?;
    Ok((a as i128 - b as i128).rem_euclid(n as i128) as u64)
}
fn linear_mul(a: u64, b: u64, n: u64, work: &mut Work) -> Result<u64, String> {
    increment(&mut work.linear_muls, 1)?;
    increment(&mut work.linear_reductions, 1)?;
    Ok(((a as u128 * b as u128) % n as u128) as u64)
}
fn linear_pow(mut a: u64, mut exponent: u64, n: u64, work: &mut Work) -> Result<u64, String> {
    let mut value = 1;
    while exponent != 0 {
        increment(&mut work.linear_exponent_bits, 1)?;
        if exponent & 1 != 0 {
            value = linear_mul(value, a, n, work)?;
        }
        a = linear_mul(a, a, n, work)?;
        exponent >>= 1;
    }
    Ok(value)
}

fn finish(result: &mut RelationAudit, state: &State, weight: u128) -> Result<(), String> {
    result.total_work_sum.add_scaled(&state.work, weight)?;
    result
        .pair_setup_work_sum
        .add_scaled(&result.pair_setup_work_per_case, weight)?;
    result
        .search_work_sum
        .add_scaled(&state.search_work, weight)?;
    result
        .linear_work_sum
        .add_scaled(&state.linear_work, weight)?;
    result
        .verification_work_sum
        .add_scaled(&state.verification_work, weight)?;
    if state.rank == result.base_size + 1 {
        increment(&mut result.full_rank_words, weight)?;
    } else {
        increment(&mut result.budget_exhausted_words, weight)?;
    }
    for (at, milestone) in result.milestones.iter_mut().enumerate() {
        if let Some(prefix) = state.first_prefix[at] {
            increment(&mut milestone.attained, weight)?;
            increment(
                milestone.first_prefix_hist.entry(prefix).or_default(),
                weight,
            )?;
            milestone.attained_work_sum.add_scaled(
                state.first_work[at]
                    .as_ref()
                    .ok_or("missing first-attainment work")?,
                weight,
            )?;
        } else {
            increment(&mut milestone.missing, weight)?;
            milestone.missing_work_sum.add_scaled(&state.work, weight)?;
        }
        milestone
            .work_sum
            .add_scaled(state.first_work[at].as_ref().unwrap_or(&state.work), weight)?;
    }
    Ok(())
}

/// Reconcile every exhaustive census and the exclusive algorithm phases.
pub fn reconcile(result: &RelationAudit) -> Result<(), String> {
    if !(2..=3).contains(&result.base_size) || result.walk_length > 16 {
        return Err("invalid relation audit dimension".into());
    }
    let dimension = result.base_size + 1;
    if result.schema != "twist-tr04/relation-audit/v1"
        || result.words_per_target != 1u128 << result.walk_length
        || (result.target_count as u128).checked_mul(result.words_per_target)
            != Some(result.observations)
        || result.pair_table_entries == 0
        || result.pair_table_entries > result.base_size * (result.base_size + 1) / 2
        || result.rank_hist_by_prefix.len() != result.walk_length + 1
        || result.hits_sum_by_prefix.len() != result.walk_length + 1
        || result.dependent_hits_sum_by_prefix.len() != result.walk_length + 1
        || result.milestones.len() != dimension
    {
        return Err("invalid relation audit shape".into());
    }
    for (depth, histogram) in result.rank_hist_by_prefix.iter().enumerate() {
        if histogram.len() != dimension + 1
            || checked_sum(histogram.iter().copied())? != result.observations
        {
            return Err("relation prefix census does not reconcile".into());
        }
        let mut rank_sum = 0;
        for (rank, &count) in histogram.iter().enumerate() {
            increment(
                &mut rank_sum,
                count
                    .checked_mul(rank as u128)
                    .ok_or("weighted rank overflow")?,
            )?;
        }
        if histogram
            .iter()
            .enumerate()
            .any(|(rank, &count)| rank > depth && count != 0)
            || result.hits_sum_by_prefix[depth]
                .checked_sub(result.dependent_hits_sum_by_prefix[depth])
                != Some(rank_sum)
            || result.hits_sum_by_prefix[depth]
                > result
                    .observations
                    .checked_mul(depth as u128)
                    .ok_or("hit bound overflow")?
        {
            return Err("rank, independent-hit and prefix-budget counts disagree".into());
        }
    }
    if result
        .full_rank_words
        .checked_add(result.budget_exhausted_words)
        != Some(result.observations)
        || result.rank_hist_by_prefix[result.walk_length][dimension] != result.full_rank_words
    {
        return Err("full-rank and budget-failure census does not reconcile".into());
    }
    let mut total = result.pair_setup_work_sum.clone();
    total.accumulate(&result.search_work_sum)?;
    total.accumulate(&result.linear_work_sum)?;
    total.accumulate(&result.verification_work_sum)?;
    if total != result.total_work_sum {
        return Err("relation phases do not sum to total paid work".into());
    }
    if result
        .pair_setup_work_per_case
        .scaled(result.observations)?
        != result.pair_setup_work_sum
    {
        return Err("cold pair setup was not paid once per conceptual workload".into());
    }
    for (at, milestone) in result.milestones.iter().enumerate() {
        if milestone.rank != at + 1
            || milestone.attained.checked_add(milestone.missing) != Some(result.observations)
            || checked_sum(milestone.first_prefix_hist.values().copied())? != milestone.attained
            || checked_sum(
                result.rank_hist_by_prefix[result.walk_length][at + 1..]
                    .iter()
                    .copied(),
            )? != milestone.attained
            || milestone
                .first_prefix_hist
                .keys()
                .any(|&depth| depth < at + 1 || depth > result.walk_length)
            || milestone
                .attained_work_sum
                .add(&milestone.missing_work_sum)?
                != milestone.work_sum
            || (milestone.attained == 0 && milestone.attained_work_sum != Work::default())
            || (milestone.missing == 0 && milestone.missing_work_sum != Work::default())
        {
            return Err("rank milestone census does not reconcile".into());
        }
    }
    Ok(())
}
fn checked_sum(mut values: impl Iterator<Item = u128>) -> Result<u128, String> {
    values.try_fold(0u128, |sum, value| {
        sum.checked_add(value).ok_or("census sum overflow".into())
    })
}
fn put_u64(digest: &mut Sha256, value: u64) {
    digest.update(value.to_le_bytes());
}
fn record_node(
    digest: &mut Sha256,
    state: &State,
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
    if let Some(point) = state.current {
        digest.update([1]);
        put_u64(digest, point.x);
        put_u64(digest, point.y);
    } else {
        digest.update([0]);
    }
    if let Some((i, j)) = pair {
        digest.update([1]);
        put_u64(digest, i as u64);
        put_u64(digest, j as u64);
    } else {
        digest.update([0]);
    }
    put_u64(digest, state.rank as u64);
    digest.update(state.hits.to_le_bytes());
    digest.update(state.dependent_hits.to_le_bytes());
    digest.update(weight.to_le_bytes());
}

mod decimal {
    use serde::{de::Error, Deserialize, Deserializer, Serializer};
    pub fn serialize<S: Serializer>(value: &u128, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u128, D::Error> {
        String::deserialize(deserializer)?
            .parse()
            .map_err(D::Error::custom)
    }
}
mod decimal_vec {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(values: &[u128], serializer: S) -> Result<S::Ok, S::Error> {
        values
            .iter()
            .map(u128::to_string)
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u128>, D::Error> {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|v| v.parse().map_err(D::Error::custom))
            .collect()
    }
}
mod decimal_matrix {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    pub fn serialize<S: Serializer>(
        values: &[Vec<u128>],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        values
            .iter()
            .map(|row| row.iter().map(u128::to_string).collect::<Vec<_>>())
            .collect::<Vec<_>>()
            .serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Vec<u128>>, D::Error> {
        Vec::<Vec<String>>::deserialize(deserializer)?
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|v| v.parse().map_err(D::Error::custom))
                    .collect()
            })
            .collect()
    }
}
mod decimal_hist {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::BTreeMap;
    pub fn serialize<S: Serializer>(
        values: &BTreeMap<usize, u128>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        values
            .iter()
            .map(|(k, v)| (*k, v.to_string()))
            .collect::<BTreeMap<_, _>>()
            .serialize(serializer)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<BTreeMap<usize, u128>, D::Error> {
        BTreeMap::<usize, String>::deserialize(deserializer)?
            .into_iter()
            .map(|(k, v)| v.parse().map(|v| (k, v)).map_err(D::Error::custom))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Parameters, u64, Point, Vec<Point>) {
        // The a=-3 discovery helper can return order 3 or 5 at p=7;
        // the declared exhaustive panel intentionally requires n>=7.
        let parameters = Parameters { p: 7, a: 0, b: 3 };
        let (n, g) = (13, Point { x: 1, y: 2 });
        let mut e = Meter::new(parameters);
        let base = (0..parameters.p)
            .filter_map(|x| e.point(x).unwrap())
            .take(3)
            .collect();
        (parameters, n, g, base)
    }
    #[test]
    fn all_words_and_rank_milestones_reconcile() {
        let (parameters, n, g, base) = fixture();
        let result = audit(parameters, n, g, &base, 4, false).unwrap();
        assert_eq!(result.observations, (n as u128 - 1) * 16);
        assert_eq!(result.rank_hist_by_prefix[0][0], result.observations);
        for milestone in &result.milestones {
            assert_eq!(milestone.attained + milestone.missing, result.observations);
            if milestone.attained == 0 {
                assert_eq!(milestone.work_sum, result.total_work_sum);
            }
        }
        assert!(result.total_work_sum.group_additions > 0);
        let json = serde_json::to_string(&result).unwrap();
        assert_eq!(
            serde_json::from_str::<RelationAudit>(&json).unwrap(),
            result
        );
    }
    #[test]
    fn prepaid_setup_changes_costs_but_not_relation_counts() {
        let (parameters, n, g, base) = fixture();
        let paid = audit(parameters, n, g, &base[..2], 3, false).unwrap();
        let prepaid = audit(parameters, n, g, &base[..2], 3, true).unwrap();
        assert_eq!(paid.rank_hist_by_prefix, prepaid.rank_hist_by_prefix);
        assert_eq!(paid.hits_sum_by_prefix, prepaid.hits_sum_by_prefix);
        assert_eq!(paid.pair_table_entries, 3);
        assert_eq!(prepaid.pair_setup_work_sum, Work::default());
        assert_eq!(paid.search_work_sum, prepaid.search_work_sum);
        assert!(paid.total_work_sum.group_additions > prepaid.total_work_sum.group_additions);
    }
    #[test]
    fn missing_milestone_keeps_its_full_paid_budget() {
        let (parameters, n, g, base) = fixture();
        let result = audit(parameters, n, g, &base, 0, false).unwrap();
        assert_eq!(result.full_rank_words, 0);
        assert!(result.total_work_sum.group_additions > 0);
        for milestone in &result.milestones {
            assert_eq!(milestone.attained, 0);
            assert_eq!(milestone.missing, result.observations);
            assert_eq!(milestone.work_sum, result.total_work_sum);
        }
        let mut tampered = result;
        tampered.rank_hist_by_prefix[0][0] -= 1;
        assert!(reconcile(&tampered).is_err());
    }

    #[test]
    fn malformed_dimensions_and_inconsistent_hit_counts_fail_closed() {
        let (parameters, n, g, base) = fixture();
        let result = audit(parameters, n, g, &base, 3, false).unwrap();
        let mut malformed = result.clone();
        malformed.base_size = usize::MAX;
        assert!(reconcile(&malformed).is_err());
        let mut malformed = result.clone();
        malformed.walk_length = usize::MAX;
        assert!(reconcile(&malformed).is_err());
        let mut malformed = result;
        malformed.hits_sum_by_prefix[1] += 1;
        assert!(reconcile(&malformed).is_err());
    }
}
