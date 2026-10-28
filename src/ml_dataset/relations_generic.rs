//! Native large-prime graph filtering with identity automorphism and signed
//! negation columns. The public curve is preserved in every family and twist.
use super::{base_outcome, curves};
use crate::cryptanalysis::ec_index_calculus::gaussian_eliminate_mod_n_particular;
use crate::cryptanalysis::large_prime_filter::{
    self as native, CompleteRelation, LargePrimeFilterOptions, LargePrimeFilterReport,
    PartialRelation,
};
use crate::ml_dataset::types::{ArtifactPayload, CaseConfig, CaseData, SolverData};
use curves::{number, point_json, point_words, Arithmetic, SubgroupProjector, WPoint};
use native::trace::{self, FilterRun, FilterStop, FilterTraceLimits};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

const SCHEMA: &str = "ml-ecdlp-generic-large-prime-transcript/v1";
const PRIVATE_SCHEMA: &str = "ml-ecdlp-generic-large-prime-membership/v1";
pub(super) const V2_SCHEMA: &str = "ml-ecdlp-generic-large-prime-transcript/v2";
pub(super) const FILTER_ROLE: &str = "large-prime-filter-certificates/v1";
const FILTER_SCHEMA: &str = "ml-dataset.generic-large-prime-filter-batch/v1";
const POLICY: &str = "aggregate-native-filter-admission/v1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Limits {
    queries: usize,
    pairs: usize,
    residual_points: usize,
    residual_columns: usize,
    x_candidates: u64,
    filter_every: usize,
    max_small_weight: usize,
    max_sources: usize,
}
impl Limits {
    fn new(work: u64) -> Self {
        let work = work.min(100_000);
        let pairs = (work / 4).min(8192) as usize;
        let mut points = 0;
        while (points + 1) * (points + 2) / 2 <= pairs {
            points += 1;
        }
        Self {
            queries: work.min(4096) as usize,
            pairs,
            residual_points: points,
            residual_columns: 32,
            x_candidates: 2048,
            filter_every: 32,
            max_small_weight: 4096,
            max_sources: 4096,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SignedPoint {
    column: usize,
    coefficient: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Domain {
    points: Vec<Value>,
    columns: Vec<Value>,
    signed: Vec<SignedPoint>,
}
fn domain(c: &Arithmetic, points: &[WPoint], r: u64) -> Result<Domain, String> {
    let mut columns = Vec::new();
    let mut lookup = HashMap::new();
    let mut signed = Vec::new();
    let mut seen = HashSet::new();
    for &p in points {
        if p.is_none() || !c.on_curve(p) || c.reference_mul(p, r).is_some() || !seen.insert(p) {
            return Err("invalid or duplicated signed-domain point".into());
        }
        let representative = p.min(c.neg(p));
        let column = *lookup.entry(representative).or_insert_with(|| {
            let i = columns.len();
            columns.push(point_json(representative));
            i
        });
        signed.push(SignedPoint {
            column,
            coefficient: if p == representative { 1 } else { r - 1 },
        });
    }
    Ok(Domain {
        points: points.iter().copied().map(point_json).collect(),
        columns,
        signed,
    })
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pool {
    domain: Domain,
    construction: Vec<Value>,
    projection_metrics: Value,
}
fn pool(
    public: &Value,
    c: &Arithmetic,
    small: &Domain,
    r: u64,
    limits: &Limits,
    emit: bool,
) -> Result<(Pool, Vec<Value>), String> {
    let small_keys: HashSet<_> = small
        .points
        .iter()
        .map(point_words)
        .collect::<Result<_, _>>()?;
    let mut points = Vec::new();
    let mut seen = HashSet::new();
    let mut construction = Vec::new();
    let mut private = Vec::new();
    let mut projection = SubgroupProjector::new(public)?;
    if limits.residual_points >= 2 {
        for x in 1..=limits.x_candidates.min(c.q() - 1) {
            let mut candidates = Vec::new();
            for lifted in c.lift(x) {
                let (projected, multiplier, member, log) =
                    projection.project(&point_json(lifted))?;
                let p = point_words(&projected)?;
                let mut row = json!({"lifted":point_json(lifted),"projected":projected,"projection_multiplier":multiplier.to_string(),"cyclic_subgroup_member":member,"accepted":false});
                if let Some(log) = log {
                    private.push(json!({"x_candidate":x.to_string(),"lift_index":candidates.len(),"projected":projected,"log":log.to_string()}));
                }
                if !member {
                    row["reason"] = json!("outside_selected_nonzero_cyclic_subgroup");
                } else if small_keys.contains(&p) || small_keys.contains(&c.neg(p)) {
                    row["reason"] = json!("signed_orbit_intersects_small_base");
                } else if seen.contains(&p) {
                    row["reason"] = json!("duplicate_signed_orbit");
                } else if points.len() + 2 > limits.residual_points
                    || points.len() / 2 >= limits.residual_columns
                {
                    row["reason"] = json!("residual_domain_cap");
                } else {
                    row["accepted"] = json!(true);
                    row["signed_orbit"] = json!([point_json(p), point_json(c.neg(p))]);
                    for p in [p, c.neg(p)] {
                        if seen.insert(p) {
                            points.push(p);
                        }
                    }
                }
                candidates.push(row);
            }
            construction.push(json!({"x":x.to_string(),"candidates":candidates}));
            if emit {
                super::super::solvers::live_event(
                    &json!({"phase":"generic_large_prime_residual_construction","record":construction.last()}),
                )?;
            }
            if points.len() + 2 > limits.residual_points
                || points.len() / 2 >= limits.residual_columns
            {
                break;
            }
        }
    }
    Ok((
        Pool {
            domain: domain(c, &points, r)?,
            construction,
            projection_metrics: projection.counters(),
        },
        private,
    ))
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Pair {
    left: usize,
    right: usize,
    sum: Value,
}
fn pairs(
    c: &Arithmetic,
    pool: &Domain,
    limits: &Limits,
) -> Result<(Vec<Pair>, HashMap<WPoint, (usize, usize)>), String> {
    let points = pool
        .points
        .iter()
        .map(point_words)
        .collect::<Result<Vec<_>, _>>()?;
    let mut rows = Vec::new();
    let mut lookup = HashMap::new();
    for i in 0..points.len() {
        for j in i..points.len() {
            if rows.len() >= limits.pairs {
                return Err("pair-table cap exceeded".into());
            }
            let p = c.reference_add(points[i], points[j]);
            rows.push(Pair {
                left: i,
                right: j,
                sum: point_json(p),
            });
            // Identity pairs contain no residual information and must not turn
            // accidental R=0 into a purported large-prime discovery.
            if p.is_some() {
                lookup.entry(p).or_insert((i, j));
            }
        }
    }
    Ok((rows, lookup))
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Attempt {
    sequence: u64,
    coef_a: u64,
    coef_b: u64,
    target: Value,
    small_indices: Vec<usize>,
    remainder: Value,
    residual_pair: Option<(usize, usize)>,
    row: Option<PartialRelation>,
}
fn add_term<K: Ord>(terms: &mut BTreeMap<K, u64>, key: K, value: u64, r: u64) {
    let slot = terms.entry(key).or_default();
    *slot = ((*slot as u128 + value as u128) % r as u128) as u64;
}
fn make_row(
    a: &Attempt,
    small: &Domain,
    residual: &Domain,
    r: u64,
) -> Result<Option<PartialRelation>, String> {
    let Some((i, j)) = a.residual_pair else {
        return Ok(None);
    };
    let mut s = BTreeMap::new();
    for &index in &a.small_indices {
        let label = small.signed.get(index).ok_or("invalid small index")?;
        add_term(&mut s, label.column as u32, label.coefficient, r);
    }
    add_term(&mut s, small.columns.len() as u32, r - a.coef_b, r);
    let mut l = BTreeMap::new();
    for index in [i, j] {
        let label = residual.signed.get(index).ok_or("invalid residual index")?;
        add_term(&mut l, label.column as u64, label.coefficient, r);
    }
    Ok(Some(PartialRelation {
        id: a.sequence,
        small: s.into_iter().filter(|(_, v)| *v != 0).collect(),
        large: l.into_iter().filter(|(_, v)| *v != 0).collect(),
        rhs: a.coef_a,
    }))
}
fn equation(
    c: &Arithmetic,
    g: WPoint,
    q: WPoint,
    small: &Domain,
    residual: &Domain,
    row: &PartialRelation,
    r: u64,
) -> Result<(), String> {
    if row.rhs >= r {
        return Err("noncanonical relation rhs".into());
    }
    let mut sum = None;
    for &(column, k) in &row.small {
        if k >= r {
            return Err("noncanonical small coefficient".into());
        }
        let p = if column as usize == small.columns.len() {
            q
        } else {
            point_words(
                small
                    .columns
                    .get(column as usize)
                    .ok_or("small column outside domain")?,
            )?
        };
        sum = c.reference_add(sum, c.reference_mul(p, k));
    }
    for &(column, k) in &row.large {
        if k >= r {
            return Err("noncanonical residual coefficient".into());
        }
        let p = point_words(
            residual
                .columns
                .get(column as usize)
                .ok_or("residual column outside domain")?,
        )?;
        sum = c.reference_add(sum, c.reference_mul(p, k));
    }
    if sum != c.reference_mul(g, row.rhs) {
        return Err("independent generic relation equation failed".into());
    }
    Ok(())
}
fn check_attempt(
    c: &Arithmetic,
    g: WPoint,
    q: WPoint,
    small: &Domain,
    residual: &Domain,
    lookup: &HashMap<WPoint, (usize, usize)>,
    a: &Attempt,
    m: usize,
    r: u64,
) -> Result<(), String> {
    if a.coef_a >= r || a.coef_b == 0 || a.coef_b >= r || a.small_indices.len() != m - 2 {
        return Err("invalid generic query arity/coefficients".into());
    }
    let target = c.reference_add(c.reference_mul(g, a.coef_a), c.reference_mul(q, a.coef_b));
    let mut remainder = target;
    for &i in &a.small_indices {
        remainder = c.reference_add(
            remainder,
            c.neg(point_words(
                small
                    .points
                    .get(i)
                    .ok_or("query small index outside domain")?,
            )?),
        );
    }
    if a.target != point_json(target)
        || a.remainder != point_json(remainder)
        || a.residual_pair != lookup.get(&remainder).copied()
        || a.row != make_row(a, small, residual, r)?
    {
        return Err("query point, exact residual lookup, or encoded partial differs".into());
    }
    if let Some(row) = &a.row {
        equation(c, g, q, small, residual, row, r)?;
    }
    Ok(())
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Rank {
    columns: usize,
    rank: usize,
    determined: Vec<bool>,
    values: Vec<u64>,
    rref: Vec<Vec<String>>,
    rhs: Vec<String>,
}
fn reduce(rows: &[CompleteRelation], columns: usize, r: u64) -> Result<Rank, String> {
    if rows.is_empty() {
        return Ok(Rank {
            columns,
            rank: 0,
            determined: vec![false; columns],
            values: vec![0; columns],
            rref: Vec::new(),
            rhs: Vec::new(),
        });
    }
    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    for row in rows {
        let mut dense = vec![BigUint::from(0u64); columns];
        for &(i, k) in &row.small {
            if i as usize >= columns || k >= r {
                return Err("invalid completed matrix column/coefficient".into());
            }
            dense[i as usize] = k.into();
        }
        matrix.push(dense);
        rhs.push(BigUint::from(row.rhs));
    }
    let solution = gaussian_eliminate_mod_n_particular(&mut matrix, &mut rhs, &r.into())
        .ok_or("native modular reduction rejected verified equations")?;
    Ok(Rank {
        columns,
        rank: solution.rank,
        determined: solution.determined,
        values: solution
            .values
            .iter()
            .map(|v| v.to_u64().ok_or("matrix scalar exceeds u64"))
            .collect::<Result<_, _>>()?,
        rref: matrix
            .iter()
            .map(|row| row.iter().map(ToString::to_string).collect())
            .collect(),
        rhs: rhs.iter().map(ToString::to_string).collect(),
    })
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Batch {
    through_attempts: usize,
    source_count: usize,
    report: LargePrimeFilterReport,
    completed: Vec<CompleteRelation>,
    matrix: Rank,
}
fn batch(
    c: &Arithmetic,
    g: WPoint,
    q: WPoint,
    small: &Domain,
    residual: &Domain,
    inputs: &[PartialRelation],
    through: usize,
    r: u64,
    limits: &Limits,
) -> Result<Batch, String> {
    let options = LargePrimeFilterOptions {
        modulus: r,
        max_small_weight: limits.max_small_weight,
        max_sources: limits.max_sources,
    };
    let (completed, report) = native::filter_large_primes(inputs.to_vec(), &options)
        .map_err(|e| format!("native large-prime filter: {e:?}"))?;
    for row in &completed {
        if native::replay_complete_relation(row, inputs, r).as_ref() != Some(row) {
            return Err("native large-prime source combination failed replay".into());
        }
        equation(
            c,
            g,
            q,
            small,
            residual,
            &PartialRelation {
                id: 0,
                small: row.small.clone(),
                large: Vec::new(),
                rhs: row.rhs,
            },
            r,
        )?;
    }
    let matrix = reduce(&completed, small.columns.len() + 1, r)?;
    Ok(Batch {
        through_attempts: through,
        source_count: inputs.len(),
        report,
        completed,
        matrix,
    })
}
fn recovered(
    c: &Arithmetic,
    g: WPoint,
    q: WPoint,
    small: &Domain,
    b: &Batch,
) -> Result<(Option<u64>, Option<Vec<u64>>), String> {
    let target = small.columns.len();
    if !b.matrix.determined[target] {
        return Ok((None, None));
    }
    let scalar = b.matrix.values[target];
    if c.reference_mul(g, scalar) != q {
        return Err("generic large-prime target scalar failed independent replay".into());
    }
    let logs = if b.matrix.rank == target + 1 {
        for (p, &k) in small.columns.iter().zip(&b.matrix.values) {
            if c.reference_mul(g, k) != point_words(p)? {
                return Err("generic factor-base log failed independent replay".into());
            }
        }
        Some(b.matrix.values[..target].to_vec())
    } else {
        None
    };
    Ok((Some(scalar), logs))
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Transcript {
    schema: String,
    public_instance: Value,
    orbit_action: String,
    m: usize,
    requested_work: u64,
    limits: Limits,
    small: Domain,
    residual: Pool,
    pairs: Vec<Pair>,
    attempts: Vec<Attempt>,
    batches: Vec<Batch>,
    terminal: String,
    recovered_scalar: Option<u64>,
    recovered_column_logs: Option<Vec<u64>>,
}
fn inputs(
    public: &Value,
    base: &Value,
) -> Result<(Arithmetic, WPoint, WPoint, u64, usize, Domain), String> {
    curves::verify_public(public)?;
    curves::verify_factor_base(public, base)?;
    let c = curves::arithmetic(public)?;
    let r = number(&public["curve"]["subgroup_order"], "r")?;
    if r >= 1u64 << 63 {
        return Err("native generic large-prime modulus must be below 2^63".into());
    }
    let points = base["points"]
        .as_array()
        .ok_or("missing factor-base points")?
        .iter()
        .map(point_words)
        .collect::<Result<Vec<_>, _>>()?;
    let small = domain(&c, &points, r)?;
    Ok((
        c,
        point_words(&public["generator"])?,
        point_words(&public["target"])?,
        r,
        public["m"].as_u64().ok_or("missing arity")? as usize,
        small,
    ))
}
fn membership_artifact(public: &Value, limits: &Limits, metrics: &Value, logs: &[Value]) -> Value {
    json!({"schema":PRIVATE_SCHEMA,"public_instance":public,"limits":limits,"projection_metrics":metrics,"logs":logs,
        "feature_access":"forbidden; diagnostic subgroup setup provenance only","used_in_relation_elimination":false})
}
#[cfg(test)]
fn solve_v1(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
    let (c, g, q, r, m, small) = inputs(&case.public_instance, &case.factor_base)?;
    if config.m != m {
        return Err("configuration arity differs from public instance".into());
    }
    let limits = Limits::new(config.work_limit);
    let (residual, private) = pool(&case.public_instance, &c, &small, r, &limits, true)?;
    let (pair_table, lookup) = pairs(&c, &residual.domain, &limits)?;
    let domain_artifact = ArtifactPayload::json(
        "generic-large-prime-domains.json",
        "exact-large-prime-discovery-domains",
        &json!({"orbit_action":"identity-with-signed-negation","small":small,"residual":residual}),
    )?;
    let pair_artifact = ArtifactPayload::json(
        "generic-large-prime-pairs.json",
        "complete-large-prime-pair-table",
        &json!({"pairs":pair_table,"identity_sums_excluded":true,"duplicate_sums_keep_first":true}),
    )?;
    let private_artifact = ArtifactPayload::json(
        "generic-large-prime-membership-private.json",
        "restricted-large-prime-membership-provenance",
        &membership_artifact(
            &case.public_instance,
            &limits,
            &residual.projection_metrics,
            &private,
        ),
    )?;
    for a in [&domain_artifact, &pair_artifact, &private_artifact] {
        super::super::solvers::live_payload(a)?;
    }
    let mut attempts = Vec::new();
    let mut sources = Vec::new();
    let mut batches = Vec::new();
    let mut scalar = None;
    let mut logs = None;
    let mut rng = StdRng::seed_from_u64(
        config.seed
            ^ config.generator_index.rotate_left(17)
            ^ config.key_index.rotate_left(31)
            ^ 0x8a01_731f_94b0_d6c3,
    );
    let can_query = !lookup.is_empty() && (m == 2 || !small.points.is_empty());
    if can_query {
        for sequence in 0..limits.queries {
            let a = rng.gen_range(0..r);
            let b = rng.gen_range(1..r);
            let target = c.add(c.mul(g, a), c.mul(q, b));
            let mut remainder = target;
            let mut indices = Vec::new();
            for _ in 0..m - 2 {
                let i = rng.gen_range(0..small.points.len());
                indices.push(i);
                remainder = c.add(remainder, c.neg(point_words(&small.points[i])?));
            }
            let mut attempt = Attempt {
                sequence: sequence as u64,
                coef_a: a,
                coef_b: b,
                target: point_json(target),
                small_indices: indices,
                remainder: point_json(remainder),
                residual_pair: lookup.get(&remainder).copied(),
                row: None,
            };
            attempt.row = make_row(&attempt, &small, &residual.domain, r)?;
            check_attempt(&c, g, q, &small, &residual.domain, &lookup, &attempt, m, r)?;
            let accepted = attempt.row.is_some();
            if let Some(row) = &attempt.row {
                sources.push(row.clone());
            }
            super::super::solvers::live_event(
                &json!({"phase":"generic_large_prime_query","record":attempt}),
            )?;
            attempts.push(attempt);
            if (accepted && sources.len().is_multiple_of(limits.filter_every))
                || attempts.len() == limits.queries
            {
                super::super::solvers::live_event(
                    &json!({"phase":"generic_large_prime_filter_start","sources":sources.len(),"through_attempts":attempts.len()}),
                )?;
                let b = batch(
                    &c,
                    g,
                    q,
                    &small,
                    &residual.domain,
                    &sources,
                    attempts.len(),
                    r,
                    &limits,
                )?;
                (scalar, logs) = recovered(&c, g, q, &small, &b)?;
                super::super::solvers::live_event(
                    &json!({"phase":"generic_large_prime_filter_complete","record":b}),
                )?;
                batches.push(b);
                if scalar.is_some() {
                    break;
                }
            }
        }
    }
    let terminal = if scalar.is_some() {
        "verified"
    } else if !can_query {
        "empty_declared_domain"
    } else {
        "budget_exhausted"
    };
    let transcript = Transcript {
        schema: SCHEMA.into(),
        public_instance: case.public_instance.clone(),
        orbit_action: "identity-with-signed-negation".into(),
        m,
        requested_work: config.work_limit,
        limits,
        small,
        residual,
        pairs: pair_table,
        attempts,
        batches,
        terminal: terminal.into(),
        recovered_scalar: scalar,
        recovered_column_logs: logs,
    };
    let value = serde_json::to_value(&transcript).map_err(|e| e.to_string())?;
    let artifact = ArtifactPayload::json(
        "generic-large-prime-transcript.json",
        "large-prime-elimination-transcript",
        &value,
    )?;
    super::super::solvers::live_payload(&artifact)?;
    verify(&case.public_instance, &case.factor_base, &value)?;
    let mut outcome = base_outcome(
        if scalar.is_some() {
            "verified"
        } else {
            "budget_exhausted"
        },
        if scalar.is_some() {
            "Native identity-orbit signed large-prime filtering recovered the public target scalar with independent source and group replay"
        } else {
            "Bounded native identity-orbit large-prime collection did not determine the target scalar"
        },
    );
    for key in [
        "solver_completed",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
    ] {
        outcome[key] = json!(scalar.is_some());
    }
    outcome["read_private_witness"] = json!(false);
    outcome["factor_base_logs_complete"] = json!(transcript.recovered_column_logs.is_some());
    outcome["orbit_action"] = json!("identity-with-signed-negation");
    outcome["native_backend"] = json!("large_prime_filter::filter_large_primes");
    outcome["requested_m"] = json!(m);
    outcome["terminal_detail"] = json!(terminal);
    outcome["residual_projection_setup"] = transcript.residual.projection_metrics.clone();
    outcome["counters"] = json!({"query_attempts":transcript.attempts.len(),"accepted_partials":sources.len(),"pair_add_calls":transcript.pairs.len(),"query_scalar_mul_calls":2*transcript.attempts.len(),"query_group_add_calls":(m-1)*transcript.attempts.len(),"native_filter_calls":transcript.batches.len(),"native_filter_reports":transcript.batches.iter().map(|b|&b.report).collect::<Vec<_>>()});
    outcome["trace_gaps"] = json!([
        "native internal peel queue, pivot selection and arithmetic snapshots",
        "hardware retired instruction counters"
    ]);
    let mut artifacts = vec![artifact, domain_artifact, pair_artifact, private_artifact];
    if let Some(k) = scalar {
        artifacts.push(ArtifactPayload::json("scalar-recovery.json","restricted-independent-scalar-label",&json!({"scalar":k.to_string(),"generator":point_json(g),"target":point_json(q),"method":"native-generic-double-large-prime","read_private_witness":false}))?);
    }
    Ok(SolverData { outcome, artifacts })
}

fn verify_v1(public: &Value, base: &Value, saved: &Value) -> Result<(), String> {
    let t: Transcript = serde_json::from_value(saved.clone())
        .map_err(|e| format!("generic large-prime transcript: {e}"))?;
    let (c, g, q, r, m, small) = inputs(public, base)?;
    let limits = Limits::new(t.requested_work);
    if t.schema != SCHEMA
        || t.public_instance != *public
        || t.orbit_action != "identity-with-signed-negation"
        || t.m != m
        || t.limits != limits
        || t.small != small
    {
        return Err("generic relation identity/domain/budget mismatch".into());
    }
    let (residual, _) = pool(public, &c, &small, r, &limits, false)?;
    if t.residual != residual {
        return Err("generic residual domain differs from exact reconstruction".into());
    }
    let (pair_table, lookup) = pairs(&c, &residual.domain, &limits)?;
    if t.pairs != pair_table || t.attempts.len() > limits.queries {
        return Err("generic pair table/query cap mismatch".into());
    }
    let can_query = !lookup.is_empty() && (m == 2 || !small.points.is_empty());
    let mut sources = Vec::new();
    let mut batches = Vec::new();
    let mut scalar = None;
    let mut logs = None;
    for (index, a) in t.attempts.iter().enumerate() {
        if !can_query || scalar.is_some() || a.sequence != index as u64 {
            return Err("query outside declared lifecycle".into());
        }
        check_attempt(&c, g, q, &small, &residual.domain, &lookup, a, m, r)?;
        let accepted = a.row.is_some();
        if let Some(row) = &a.row {
            sources.push(row.clone());
        }
        if (accepted && sources.len().is_multiple_of(limits.filter_every))
            || index + 1 == limits.queries
        {
            let b = batch(
                &c,
                g,
                q,
                &small,
                &residual.domain,
                &sources,
                index + 1,
                r,
                &limits,
            )?;
            (scalar, logs) = recovered(&c, g, q, &small, &b)?;
            batches.push(b);
        }
    }
    let terminal = if scalar.is_some() {
        "verified"
    } else if !can_query {
        "empty_declared_domain"
    } else {
        "budget_exhausted"
    };
    if t.batches != batches
        || t.recovered_scalar != scalar
        || t.recovered_column_logs != logs
        || t.terminal != terminal
        || (can_query && scalar.is_none() && t.attempts.len() != limits.queries)
    {
        return Err(
            "generic filter/source/matrix/recovery or termination certificate differs".into(),
        );
    }
    Ok(())
}
pub(super) fn verify_membership(public: &Value, base: &Value, saved: &Value) -> Result<(), String> {
    let (c, _, _, r, _, small) = inputs(public, base)?;
    let limits: Limits =
        serde_json::from_value(saved["limits"].clone()).map_err(|e| e.to_string())?;
    if limits.residual_points > 127
        || limits.residual_columns != 32
        || limits.x_candidates != 2048
        || limits.filter_every != 32
        || limits.max_small_weight != 4096
        || limits.max_sources != 4096
        || limits.pairs > 8192
        || limits.queries > 4096
    {
        return Err("invalid restricted generic membership bounds".into());
    }
    let (pool, logs) = pool(public, &c, &small, r, &limits, false)?;
    if *saved != membership_artifact(public, &limits, &pool.projection_metrics, &logs) {
        return Err("generic membership provenance replay mismatch".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::solve_v1 as solve;
    use super::*;
    fn config(family: &str, n: u32, twist: bool, m: usize) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n,
            m,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 2,
            seed: 42,
            factor_base_size: 2,
            factor_base_policy: None,
            degree_bound: 3,
            work_limit: 20_000,
            solver: "double-large-prime".into(),
        }
    }
    #[test]
    fn identity_orbit_native_filter_recovers_without_private_witness_on_every_family_and_twist() {
        for family in ["koblitz", "binary", "prime"] {
            for twist in [false, true] {
                let cfg = config(family, 13, twist, 3);
                let mut case = curves::generate(&cfg).unwrap();
                case.private_witness = Value::Null;
                let solved = solve(&cfg, &case).unwrap();
                assert_eq!(
                    solved.outcome["scalar_recovered_without_witness"], true,
                    "{family} twist={twist}: {}",
                    solved.outcome
                );
                let t: Value = serde_json::from_slice(&solved.artifacts[0].bytes).unwrap();
                verify(&case.public_instance, &case.factor_base, &t).unwrap();
                let typed: Transcript = serde_json::from_value(t).unwrap();
                assert!(typed.batches.iter().any(|b| b.report.cycles_closed > 0));
                assert!(typed
                    .batches
                    .iter()
                    .flat_map(|b| &b.completed)
                    .any(|row| row.sources.len() > 1));
                let private: Value = serde_json::from_slice(&solved.artifacts[3].bytes).unwrap();
                verify_membership(&case.public_instance, &case.factor_base, &private).unwrap();
            }
        }
    }
    #[test]
    fn arbitrary_arity_and_requested_field_boundaries_keep_exact_instances() {
        for m in [2, 15] {
            let cfg = config("prime", 13, false, m);
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let solved = solve(&cfg, &case).unwrap();
            assert_eq!(solved.outcome["scalar_recovered_without_witness"], true);
        }
        for (family, n, twist) in [
            ("binary", 24, false),
            ("binary", 24, true),
            ("koblitz", 24, true),
            ("prime", 61, false),
            ("prime", 61, true),
        ] {
            let mut cfg = config(family, n, twist, 15);
            cfg.work_limit = 64;
            let mut case = curves::generate(&cfg).unwrap();
            case.private_witness = Value::Null;
            let solved = super::super::solve(&cfg, &case).unwrap();
            assert_ne!(
                solved.outcome["status"], "unsupported",
                "{family} n={n} twist={twist}"
            );
            let t: Value = serde_json::from_slice(&solved.artifacts[0].bytes).unwrap();
            assert_eq!(t["public_instance"], case.public_instance);
            assert_eq!(t["orbit_action"], "identity-with-signed-negation");
            verify(&case.public_instance, &case.factor_base, &t).unwrap();
        }
    }
    #[test]
    fn generic_transcripts_reject_modified_domains_sources_rank_and_caps() {
        let cfg = config("prime", 13, true, 3);
        let mut case = curves::generate(&cfg).unwrap();
        case.private_witness = Value::Null;
        let solved = solve(&cfg, &case).unwrap();
        let t: Value = serde_json::from_slice(&solved.artifacts[0].bytes).unwrap();
        for path in ["recovered_scalar", "m"] {
            let mut bad = t.clone();
            bad[path] = json!(0);
            assert!(verify(&case.public_instance, &case.factor_base, &bad).is_err());
        }
        let mut bad = t.clone();
        bad["limits"]["queries"] = json!(1);
        assert!(verify(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = t.clone();
        bad["pairs"][0]["sum"] = json!({"x":"0","y":"0"});
        assert!(verify(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = t.clone();
        bad["batches"][0]["matrix"]["rank"] = json!(0);
        assert!(verify(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = t.clone();
        let rows = bad["batches"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|b| !b["completed"].as_array().unwrap().is_empty())
            .unwrap();
        rows["completed"][0]["sources"][0][1] = json!(0);
        assert!(verify(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut base = case.factor_base.clone();
        base["points"] = json!([]);
        assert!(verify(&case.public_instance, &base, &t).is_err());
        let mut private: Value = serde_json::from_slice(&solved.artifacts[3].bytes).unwrap();
        private["logs"] = json!([{"log":"0"}]);
        assert!(verify_membership(&case.public_instance, &case.factor_base, &private).is_err());
    }
}

// V1 above remains an exact replay contract. V2 adds independently checked
// bounded native prefixes and never interprets a partial batch as a rank result.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Bindings {
    public_sha256: String,
    factor_base_sha256: String,
    config_sha256: String,
    domains_and_pairs_sha256: String,
}
fn canonical_hash<T: Serialize + ?Sized>(input: &T) -> Result<String, String> {
    fn sorted(value: Value) -> Value {
        match value {
            Value::Object(map) => {
                let ordered: BTreeMap<_, _> =
                    map.into_iter().map(|(k, v)| (k, sorted(v))).collect();
                Value::Object(ordered.into_iter().collect())
            }
            Value::Array(items) => Value::Array(items.into_iter().map(sorted).collect()),
            scalar => scalar,
        }
    }
    let value = sorted(serde_json::to_value(input).map_err(|e| e.to_string())?);
    Ok(super::super::digest(
        &serde_json::to_vec(&value).map_err(|e| e.to_string())?,
    ))
}
impl Bindings {
    fn new(
        public: &Value,
        base: &Value,
        config: &CaseConfig,
        small: &Domain,
        residual: &Pool,
        pairs: &[Pair],
    ) -> Result<Self, String> {
        Ok(Self {
            public_sha256: canonical_hash(public)?,
            factor_base_sha256: canonical_hash(base)?,
            config_sha256: canonical_hash(config)?,
            domains_and_pairs_sha256: canonical_hash(
                &json!({"small":small,"residual":residual,"pairs":pairs}),
            )?,
        })
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TraceTotals {
    acknowledged_events: u64,
    attempted_events: u64,
    acknowledged_bytes: u64,
    refused_event_bytes: u64,
    acknowledged_work: u64,
    computed_work: u64,
    inverse_calls: u64,
    acknowledged_inverse_calls: u64,
}
impl TraceTotals {
    fn remaining(&self, policy: FilterTraceLimits) -> Result<FilterTraceLimits, String> {
        Ok(FilterTraceLimits {
            events: policy
                .events
                .checked_sub(self.attempted_events)
                .ok_or("aggregate native events exceed policy")?,
            bytes: policy
                .bytes
                .checked_sub(self.acknowledged_bytes)
                .ok_or("aggregate native bytes exceed policy")?,
            work: policy
                .work
                .checked_sub(self.computed_work)
                .ok_or("aggregate native work exceeds policy")?,
            ..policy
        })
    }
    fn charge(&mut self, run: &FilterRun, policy: FilterTraceLimits) -> Result<(), String> {
        fn add(to: &mut u64, amount: u64) -> Result<(), String> {
            *to = to
                .checked_add(amount)
                .ok_or("native trace total overflow")?;
            Ok(())
        }
        let acknowledged =
            u64::try_from(run.events.len()).map_err(|_| "native event count overflow")?;
        add(&mut self.acknowledged_events, acknowledged)?;
        add(
            &mut self.attempted_events,
            acknowledged + u64::from(run.unacknowledged.is_some()),
        )?;
        add(&mut self.acknowledged_bytes, run.acknowledged_bytes)?;
        add(&mut self.refused_event_bytes, run.unacknowledged_bytes)?;
        add(&mut self.acknowledged_work, run.acknowledged_work)?;
        add(&mut self.computed_work, run.computed_work)?;
        add(&mut self.inverse_calls, run.inverse_calls)?;
        add(
            &mut self.acknowledged_inverse_calls,
            run.acknowledged_inverse_calls,
        )?;
        self.remaining(policy)?;
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BatchBinding {
    case: Bindings,
    batch_index: usize,
    through_attempts: usize,
    source_count: usize,
    ordered_sources_sha256: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
enum BatchTerminal {
    Completed { legacy_batch: Batch },
    Censored { stop: FilterStop },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TracedBatch {
    schema: String,
    binding: BatchBinding,
    native: FilterRun,
    terminal: BatchTerminal,
}
impl TracedBatch {
    fn complete(&self) -> Option<&Batch> {
        match &self.terminal {
            BatchTerminal::Completed { legacy_batch } => Some(legacy_batch),
            BatchTerminal::Censored { .. } => None,
        }
    }
    fn payload(&self) -> Result<ArtifactPayload, String> {
        ArtifactPayload::json(&filter_path(self.binding.batch_index), FILTER_ROLE, self)
    }
}
fn filter_path(index: usize) -> String {
    format!("generic-large-prime-filter-batch-{index:04}.json")
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TranscriptV2 {
    schema: String,
    public_instance: Value,
    config: CaseConfig,
    binding: Bindings,
    orbit_action: String,
    m: usize,
    requested_work: u64,
    limits: Limits,
    small: Domain,
    residual: Pool,
    pairs: Vec<Pair>,
    attempts: Vec<Attempt>,
    batches: Vec<TracedBatch>,
    trace_policy: String,
    trace_limits: FilterTraceLimits,
    trace_totals: TraceTotals,
    trace_work_scope: String,
    trace_byte_scope: String,
    terminal: String,
    recovered_scalar: Option<u64>,
    recovered_column_logs: Option<Vec<u64>>,
    recorded_row_transitions_verified: bool,
    execution_trace_complete_verified: bool,
    total_operation_count_verified: bool,
    dependency_basis_complete_verified: bool,
}
fn query_rng(config: &CaseConfig) -> StdRng {
    StdRng::seed_from_u64(
        config.seed
            ^ config.generator_index.rotate_left(17)
            ^ config.key_index.rotate_left(31)
            ^ 0x8a01_731f_94b0_d6c3,
    )
}
fn query_parameters(rng: &mut StdRng, r: u64, m: usize, small: usize) -> (u64, u64, Vec<usize>) {
    (
        rng.gen_range(0..r),
        rng.gen_range(1..r),
        (0..m - 2).map(|_| rng.gen_range(0..small)).collect(),
    )
}
fn filter_options(r: u64, limits: &Limits) -> LargePrimeFilterOptions {
    LargePrimeFilterOptions {
        modulus: r,
        max_small_weight: limits.max_small_weight,
        max_sources: limits.max_sources,
    }
}
fn native_failure(run: &FilterRun) -> Result<(), String> {
    if run.stop == FilterStop::SinkFailure {
        return Err(format!(
            "native generic filter live sink failed: {}",
            run.sink_error.as_deref().unwrap_or("missing sink error")
        ));
    }
    Ok(())
}
fn batch_binding(
    binding: &Bindings,
    index: usize,
    through: usize,
    sources: &[PartialRelation],
) -> Result<BatchBinding, String> {
    Ok(BatchBinding {
        case: binding.clone(),
        batch_index: index,
        through_attempts: through,
        source_count: sources.len(),
        ordered_sources_sha256: canonical_hash(sources)?,
    })
}
fn traced_batch(
    c: &Arithmetic,
    g: WPoint,
    q: WPoint,
    small: &Domain,
    residual: &Domain,
    sources: &[PartialRelation],
    binding: BatchBinding,
    r: u64,
    limits: &Limits,
    allowance: FilterTraceLimits,
) -> Result<TracedBatch, String> {
    super::super::solvers::live_event(
        &json!({"phase":"generic_large_prime_native_filter_start","binding":binding,"remaining_native_limits":allowance}),
    )?;
    let run = trace::filter_large_primes_with_trace(sources.to_vec(), &filter_options(r, limits), allowance, &mut |event| {
        super::super::solvers::live_event(&json!({"phase":"generic_large_prime_native_filter_event","binding":binding,"native":event}))
    }).map_err(|e| format!("bounded native large-prime filter: {e:?}"))?;
    native_failure(&run)?;
    trace::verify_filter_trace(sources, &filter_options(r, limits), &run)?;
    let terminal = if run.stop == FilterStop::Completed {
        // Replay/group/rank work is deliberately outside the native trace counter.
        let legacy_batch = batch(
            c,
            g,
            q,
            small,
            residual,
            sources,
            binding.through_attempts,
            r,
            limits,
        )?;
        if legacy_batch.completed != run.rows || legacy_batch.report != run.report {
            return Err("complete traced filter differs from legacy result/report".into());
        }
        BatchTerminal::Completed { legacy_batch }
    } else {
        BatchTerminal::Censored { stop: run.stop }
    };
    Ok(TracedBatch {
        schema: FILTER_SCHEMA.into(),
        binding,
        native: run,
        terminal,
    })
}
fn trace_censored(t: &TranscriptV2) -> bool {
    t.batches.last().is_some_and(|b| b.complete().is_none())
}
fn counters_v2(t: &TranscriptV2) -> Value {
    json!({"query_attempts":t.attempts.len(),"accepted_partials":t.attempts.iter().filter(|a|a.row.is_some()).count(),
        "pair_add_calls":t.pairs.len(),"query_scalar_mul_calls":2*t.attempts.len(),"query_group_add_calls":(t.m-1)*t.attempts.len(),
        "native_filter_calls":t.batches.len(),"native_filter_reports":t.batches.iter().filter_map(|b|b.complete().map(|v|&v.report)).collect::<Vec<_>>(),
        "native_filter_trace":t.trace_totals})
}
fn outcome_v2(t: &TranscriptV2) -> Value {
    let recovered = t.recovered_scalar.is_some();
    let mut outcome = base_outcome(
        if recovered {
            "verified"
        } else {
            "budget_exhausted"
        },
        if recovered {
            "Native signed generic filtering recovered the public scalar with independent trace/source/group replay"
        } else if trace_censored(t) {
            "Native filter trace admission stopped collection; the final batch is censored and supplies no rank or recovery"
        } else {
            "Bounded native identity-orbit large-prime collection did not determine the target scalar"
        },
    );
    for key in [
        "solver_completed",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
    ] {
        outcome[key] = json!(recovered);
    }
    outcome["read_private_witness"] = json!(false);
    outcome["factor_base_logs_complete"] = json!(t.recovered_column_logs.is_some());
    outcome["orbit_action"] = json!("identity-with-signed-negation");
    outcome["native_backend"] = json!("large_prime_filter::trace::filter_large_primes_with_trace");
    outcome["requested_m"] = json!(t.m);
    outcome["terminal_detail"] = json!(t.terminal);
    outcome["filter_trace_policy"] = json!(POLICY);
    outcome["filter_trace_censored"] = json!(trace_censored(t));
    outcome["filter_trace_stop"] = json!(t.batches.last().map(|b| b.native.stop));
    outcome["recorded_row_transitions_verified"] = json!(true);
    for key in [
        "execution_trace_complete_verified",
        "total_operation_count_verified",
        "dependency_basis_complete_verified",
    ] {
        outcome[key] = json!(false);
    }
    outcome["residual_projection_setup"] = t.residual.projection_metrics.clone();
    outcome["counters"] = counters_v2(t);
    outcome["trace_gaps"] = json!([
        "native modular rank reducer internal operations",
        "unexposed allocations, control, arithmetic internals and hardware retired instructions",
        "admitted Koblitz signed-Frobenius filter uses its separate v1 path"
    ]);
    outcome
}

pub(super) fn solve(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
    solve_v2(config, case, FilterTraceLimits::default())
}
fn solve_v2(
    config: &CaseConfig,
    case: &CaseData,
    policy: FilterTraceLimits,
) -> Result<SolverData, String> {
    let (c, g, q, r, m, small) = inputs(&case.public_instance, &case.factor_base)?;
    if config.m != m
        || case.public_instance["family"].as_str() != Some(config.family.as_str())
        || case.public_instance["n"].as_u64() != Some(config.n as u64)
        || case.public_instance["twist"].as_bool() != Some(config.twist)
        || config.solver != "double-large-prime"
    {
        return Err("configuration differs from generic relation public domain".into());
    }
    curves::verify_factor_base_config(config, &case.public_instance, &case.factor_base)?;
    let limits = Limits::new(config.work_limit);
    let (residual, private) = pool(&case.public_instance, &c, &small, r, &limits, true)?;
    let (pair_table, lookup) = pairs(&c, &residual.domain, &limits)?;
    let binding = Bindings::new(
        &case.public_instance,
        &case.factor_base,
        config,
        &small,
        &residual,
        &pair_table,
    )?;
    let domain_artifact = ArtifactPayload::json(
        "generic-large-prime-domains.json",
        "exact-large-prime-discovery-domains",
        &json!({"orbit_action":"identity-with-signed-negation","small":small,"residual":residual}),
    )?;
    let pair_artifact = ArtifactPayload::json(
        "generic-large-prime-pairs.json",
        "complete-large-prime-pair-table",
        &json!({"pairs":pair_table,"identity_sums_excluded":true,"duplicate_sums_keep_first":true}),
    )?;
    let private_artifact = ArtifactPayload::json(
        "generic-large-prime-membership-private.json",
        "restricted-large-prime-membership-provenance",
        &membership_artifact(
            &case.public_instance,
            &limits,
            &residual.projection_metrics,
            &private,
        ),
    )?;
    for payload in [&domain_artifact, &pair_artifact, &private_artifact] {
        super::super::solvers::live_payload(payload)?;
    }
    let mut attempts = Vec::new();
    let mut sources = Vec::new();
    let mut batches = Vec::new();
    let mut batch_artifacts = Vec::new();
    let mut scalar = None;
    let mut logs = None;
    let mut totals = TraceTotals::default();
    let mut rng = query_rng(config);
    let can_query = !lookup.is_empty() && (m == 2 || !small.points.is_empty());
    if can_query {
        for sequence in 0..limits.queries {
            let (a, b, indices) = query_parameters(&mut rng, r, m, small.points.len());
            let target = c.add(c.mul(g, a), c.mul(q, b));
            let mut remainder = target;
            for &i in &indices {
                remainder = c.add(remainder, c.neg(point_words(&small.points[i])?));
            }
            let mut attempt = Attempt {
                sequence: sequence as u64,
                coef_a: a,
                coef_b: b,
                target: point_json(target),
                small_indices: indices,
                remainder: point_json(remainder),
                residual_pair: lookup.get(&remainder).copied(),
                row: None,
            };
            attempt.row = make_row(&attempt, &small, &residual.domain, r)?;
            check_attempt(&c, g, q, &small, &residual.domain, &lookup, &attempt, m, r)?;
            let accepted = attempt.row.is_some();
            if let Some(row) = &attempt.row {
                sources.push(row.clone());
            }
            super::super::solvers::live_event(
                &json!({"phase":"generic_large_prime_query","binding":binding,"record":attempt}),
            )?;
            attempts.push(attempt);
            if (accepted && sources.len().is_multiple_of(limits.filter_every))
                || attempts.len() == limits.queries
            {
                let record = traced_batch(
                    &c,
                    g,
                    q,
                    &small,
                    &residual.domain,
                    &sources,
                    batch_binding(&binding, batches.len(), attempts.len(), &sources)?,
                    r,
                    &limits,
                    totals.remaining(policy)?,
                )?;
                totals.charge(&record.native, policy)?;
                let payload = record.payload()?;
                // Preserve the exact native prefix before subsequent rank/recovery labels
                // or queries can be published. A payload error stops the worker.
                super::super::solvers::live_payload(&payload)?;
                batch_artifacts.push(payload);
                let censored = record.complete().is_none();
                if let Some(complete) = record.complete() {
                    (scalar, logs) = recovered(&c, g, q, &small, complete)?;
                }
                batches.push(record);
                if censored || scalar.is_some() {
                    break;
                }
            }
        }
    }
    let censored = batches.last().is_some_and(|b| b.complete().is_none());
    let terminal = if censored {
        "filter_trace_censored"
    } else if scalar.is_some() {
        "verified"
    } else if !can_query {
        "empty_declared_domain"
    } else {
        "budget_exhausted"
    };
    let transcript = TranscriptV2 {
        schema: V2_SCHEMA.into(),
        public_instance: case.public_instance.clone(),
        config: config.clone(),
        binding,
        orbit_action: "identity-with-signed-negation".into(),
        m,
        requested_work: config.work_limit,
        limits,
        small,
        residual,
        pairs: pair_table,
        attempts,
        batches,
        trace_policy: POLICY.into(),
        trace_limits: policy,
        trace_totals: totals,
        trace_work_scope: trace::WORK_SCOPE.into(),
        trace_byte_scope: trace::BYTE_SCOPE.into(),
        terminal: terminal.into(),
        recovered_scalar: scalar,
        recovered_column_logs: logs,
        recorded_row_transitions_verified: true,
        execution_trace_complete_verified: false,
        total_operation_count_verified: false,
        dependency_basis_complete_verified: false,
    };
    verify_v2(
        &case.public_instance,
        &case.factor_base,
        &transcript,
        policy,
    )?;
    let artifact = ArtifactPayload::json(
        "generic-large-prime-transcript.json",
        "large-prime-elimination-transcript",
        &transcript,
    )?;
    super::super::solvers::live_payload(&artifact)?;
    let outcome = outcome_v2(&transcript);
    let mut artifacts = vec![artifact, domain_artifact, pair_artifact, private_artifact];
    artifacts.extend(batch_artifacts);
    if let Some(k) = scalar {
        artifacts.push(ArtifactPayload::json("scalar-recovery.json","restricted-independent-scalar-label",&json!({"scalar":k.to_string(),"generator":point_json(g),"target":point_json(q),"method":"native-generic-double-large-prime","read_private_witness":false}))?);
    }
    Ok(SolverData { outcome, artifacts })
}
fn shape_v2(
    public: &Value,
    base: &Value,
    t: &TranscriptV2,
    config: &CaseConfig,
    policy: FilterTraceLimits,
) -> Result<(), String> {
    if t.schema != V2_SCHEMA
        || t.config != *config
        || t.config.solver != "double-large-prime"
        || !(2..=15).contains(&t.m)
        || t.m != config.m
        || public["m"].as_u64() != Some(t.m as u64)
        || public["n"].as_u64() != Some(config.n as u64)
        || public["family"] != config.family
        || public["twist"].as_bool() != Some(config.twist)
        || t.public_instance != *public
        || t.orbit_action != "identity-with-signed-negation"
        || t.requested_work != config.work_limit
        || t.limits != Limits::new(config.work_limit)
        || t.trace_policy != POLICY
        || t.trace_limits != policy
        || t.trace_work_scope != trace::WORK_SCOPE
        || t.trace_byte_scope != trace::BYTE_SCOPE
        || !t.recorded_row_transitions_verified
        || t.execution_trace_complete_verified
        || t.total_operation_count_verified
        || t.dependency_basis_complete_verified
        || t.attempts.len() > t.limits.queries
        || t.batches.len() > t.limits.queries / t.limits.filter_every + 1
        || t.binding != Bindings::new(public, base, config, &t.small, &t.residual, &t.pairs)?
    {
        return Err("generic v2 public/config/domain/trace-policy binding mismatch".into());
    }
    Ok(())
}
fn verify_v2(
    public: &Value,
    base: &Value,
    t: &TranscriptV2,
    policy: FilterTraceLimits,
) -> Result<(), String> {
    shape_v2(public, base, t, &t.config, policy)?;
    let (c, g, q, r, m, small) = inputs(public, base)?;
    curves::verify_factor_base_config(&t.config, public, base)?;
    if t.small != small {
        return Err("generic v2 signed small domain differs".into());
    }
    let (residual, _) = pool(public, &c, &small, r, &t.limits, false)?;
    if t.residual != residual {
        return Err("generic v2 residual construction differs".into());
    }
    let (pairs, lookup) = pairs(&c, &residual.domain, &t.limits)?;
    if t.pairs != pairs {
        return Err("generic v2 pair table differs".into());
    }
    let can_query = !lookup.is_empty() && (m == 2 || !small.points.is_empty());
    let mut sources = Vec::new();
    let mut batch_index = 0;
    let mut scalar = None;
    let mut logs = None;
    let mut censored = false;
    let mut totals = TraceTotals::default();
    let mut rng = query_rng(&t.config);
    for (index, a) in t.attempts.iter().enumerate() {
        if !can_query || scalar.is_some() || censored || a.sequence != index as u64 {
            return Err("generic v2 query outside declared lifecycle".into());
        }
        let (expected_a, expected_b, indices) =
            query_parameters(&mut rng, r, m, small.points.len());
        if (a.coef_a, a.coef_b) != (expected_a, expected_b) || a.small_indices != indices {
            return Err("generic v2 query differs from frozen random stream".into());
        }
        check_attempt(&c, g, q, &small, &residual.domain, &lookup, a, m, r)?;
        let accepted = a.row.is_some();
        if let Some(row) = &a.row {
            sources.push(row.clone());
        }
        if (accepted && sources.len().is_multiple_of(t.limits.filter_every))
            || index + 1 == t.limits.queries
        {
            let b = t
                .batches
                .get(batch_index)
                .ok_or("missing scheduled generic v2 batch")?;
            if b.schema != FILTER_SCHEMA
                || b.binding != batch_binding(&t.binding, batch_index, index + 1, &sources)?
                || b.native.limits != totals.remaining(policy)?
            {
                return Err(
                    "generic v2 batch source/order/aggregate allowance binding differs".into(),
                );
            }
            native_failure(&b.native)?;
            trace::verify_filter_trace(&sources, &filter_options(r, &t.limits), &b.native)?;
            totals.charge(&b.native, policy)?;
            match &b.terminal {
                BatchTerminal::Completed { legacy_batch } => {
                    if b.native.stop != FilterStop::Completed {
                        return Err("unfinished native batch supplies rank".into());
                    }
                    let expected = batch(
                        &c,
                        g,
                        q,
                        &small,
                        &residual.domain,
                        &sources,
                        index + 1,
                        r,
                        &t.limits,
                    )?;
                    if *legacy_batch != expected
                        || b.native.rows != expected.completed
                        || b.native.report != expected.report
                    {
                        return Err("generic v2 completed rows/report/rank differ".into());
                    }
                    (scalar, logs) = recovered(&c, g, q, &small, &expected)?;
                }
                BatchTerminal::Censored { stop } => {
                    if *stop == FilterStop::Completed
                        || *stop != b.native.stop
                        || b.native.report_final
                    {
                        return Err(
                            "generic v2 censored terminal differs from native prefix".into()
                        );
                    }
                    censored = true;
                }
            }
            batch_index += 1;
        }
    }
    let terminal = if censored {
        "filter_trace_censored"
    } else if scalar.is_some() {
        "verified"
    } else if !can_query {
        "empty_declared_domain"
    } else {
        "budget_exhausted"
    };
    if batch_index != t.batches.len()
        || totals != t.trace_totals
        || scalar != t.recovered_scalar
        || logs != t.recovered_column_logs
        || t.terminal != terminal
        || (censored && (scalar.is_some() || logs.is_some()))
        || (can_query && !censored && scalar.is_none() && t.attempts.len() != t.limits.queries)
    {
        return Err("generic v2 aggregate/recovery/termination certificate differs".into());
    }
    Ok(())
}
pub(super) fn verify(public: &Value, base: &Value, saved: &Value) -> Result<(), String> {
    if saved["schema"] == V2_SCHEMA {
        let t: TranscriptV2 = serde_json::from_value(saved.clone())
            .map_err(|e| format!("generic v2 transcript: {e}"))?;
        verify_v2(public, base, &t, FilterTraceLimits::default())
    } else {
        verify_v1(public, base, saved)
    }
}

/// Structural consistency only; mathematical replay remains verify(). Used in
/// hash-only cell verification as well as before the full replay gate.
pub(super) fn verify_v2_labels(
    public: &Value,
    base: &Value,
    config: &CaseConfig,
    saved: &Value,
    outcome: &Value,
) -> Result<(), String> {
    let t: TranscriptV2 =
        serde_json::from_value(saved.clone()).map_err(|e| format!("generic v2 labels: {e}"))?;
    shape_v2(public, base, &t, config, FilterTraceLimits::default())?;
    let mut totals = TraceTotals::default();
    let mut last_through = 0;
    for (i, b) in t.batches.iter().enumerate() {
        if b.schema != FILTER_SCHEMA
            || b.binding.case != t.binding
            || b.binding.batch_index != i
            || b.binding.through_attempts <= last_through
            || b.binding.through_attempts > t.attempts.len()
            || b.native.limits != totals.remaining(t.trace_limits)?
        {
            return Err("generic v2 batch labels/budgets differ".into());
        }
        native_failure(&b.native)?;
        totals.charge(&b.native, t.trace_limits)?;
        last_through = b.binding.through_attempts;
        match &b.terminal {
            BatchTerminal::Completed { legacy_batch } => {
                if b.native.stop != FilterStop::Completed
                    || !b.native.report_final
                    || legacy_batch.through_attempts != b.binding.through_attempts
                    || legacy_batch.source_count != b.binding.source_count
                    || legacy_batch.completed != b.native.rows
                    || legacy_batch.report != b.native.report
                {
                    return Err("generic v2 completed batch labels differ".into());
                }
            }
            BatchTerminal::Censored { stop } => {
                if i + 1 != t.batches.len()
                    || *stop == FilterStop::Completed
                    || *stop != b.native.stop
                    || b.native.report_final
                    || last_through != t.attempts.len()
                    || t.recovered_scalar.is_some()
                    || t.recovered_column_logs.is_some()
                {
                    return Err(
                        "generic v2 censored batch cannot supply recovery or later work".into(),
                    );
                }
            }
        }
    }
    if totals != t.trace_totals || (trace_censored(&t) && t.terminal != "filter_trace_censored") {
        return Err("generic v2 aggregate or censor labels differ".into());
    }
    let expected = outcome_v2(&t);
    for key in [
        "status",
        "solver_completed",
        "pdp_witness_verified",
        "scalar_recovered_without_witness",
        "scalar_replay_verified",
        "ecdlp_pipeline_completed",
        "trace_complete",
        "read_private_witness",
        "factor_base_logs_complete",
        "orbit_action",
        "native_backend",
        "requested_m",
        "terminal_detail",
        "filter_trace_policy",
        "filter_trace_censored",
        "filter_trace_stop",
        "recorded_row_transitions_verified",
        "execution_trace_complete_verified",
        "total_operation_count_verified",
        "dependency_basis_complete_verified",
        "residual_projection_setup",
        "counters",
    ] {
        if outcome[key] != expected[key] {
            return Err(format!("generic v2 outcome label differs: {key}"));
        }
    }
    Ok(())
}
pub(super) fn expected_filter_payloads(saved: &Value) -> Result<Vec<ArtifactPayload>, String> {
    let t: TranscriptV2 =
        serde_json::from_value(saved.clone()).map_err(|e| format!("generic v2 payloads: {e}"))?;
    t.batches.iter().map(TracedBatch::payload).collect()
}

#[cfg(test)]
mod trace_integration_tests {
    use super::*;
    fn cfg(family: &str, n: u32, twist: bool, m: usize) -> CaseConfig {
        CaseConfig {
            family: family.into(),
            n,
            m,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 2,
            seed: 42,
            factor_base_size: 2,
            factor_base_policy: None,
            degree_bound: 3,
            work_limit: 20_000,
            solver: "double-large-prime".into(),
        }
    }
    fn case(cfg: &CaseConfig) -> CaseData {
        let mut case = curves::generate(cfg).unwrap();
        case.private_witness = Value::Null;
        case
    }
    fn transcript(solved: &SolverData) -> TranscriptV2 {
        serde_json::from_slice(&solved.artifacts[0].bytes).unwrap()
    }
    fn value(t: &TranscriptV2) -> Value {
        serde_json::to_value(t).unwrap()
    }
    fn exact_prefix_byte_policy(run: &FilterRun, acknowledged_events: usize) -> FilterTraceLimits {
        // Begin contains the limit itself. Solve the small decimal-width fixed
        // point using independently encoded events, not the old run's counter.
        let mut policy = run.limits;
        for _ in 0..8 {
            let mut events = run.events.clone();
            let trace::Transition::Begin { limits, .. } = &mut events[0].transition else {
                panic!("fixture lacks Begin")
            };
            *limits = policy;
            let bytes: u64 = events
                .iter()
                .take(acknowledged_events)
                .map(|e| serde_json::to_vec(e).unwrap().len() as u64)
                .sum();
            if policy.bytes == bytes {
                return policy;
            }
            policy.bytes = bytes;
        }
        panic!("byte fixture decimal-width fixed point did not converge")
    }
    #[test]
    fn generic_v2_native_certificates_preserve_legacy_recovery_and_cover_domains() {
        for (family, n, m) in [
            ("prime", 13, 2),
            ("binary", 13, 3),
            ("binary", 12, 2),
            ("koblitz", 13, 3),
        ] {
            for twist in [false, true] {
                let cfg = cfg(family, n, twist, m);
                let case = case(&cfg);
                let solved = solve(&cfg, &case).unwrap();
                let t = transcript(&solved);
                verify(&case.public_instance, &case.factor_base, &value(&t)).unwrap();
                verify_v2_labels(
                    &case.public_instance,
                    &case.factor_base,
                    &cfg,
                    &value(&t),
                    &solved.outcome,
                )
                .unwrap();
                let old = solve_v1(&cfg, &case).unwrap();
                let old: Transcript = serde_json::from_slice(&old.artifacts[0].bytes).unwrap();
                assert!(
                    !trace_censored(&t),
                    "unexpected small-fixture admission: {family}/{n}/{twist}"
                );
                assert_eq!(t.attempts, old.attempts);
                assert_eq!(t.recovered_scalar, old.recovered_scalar);
                assert_eq!(t.recovered_column_logs, old.recovered_column_logs);
                assert_eq!(
                    t.batches
                        .iter()
                        .map(|b| b.complete().unwrap().clone())
                        .collect::<Vec<_>>(),
                    old.batches
                );
                let payloads = expected_filter_payloads(&value(&t)).unwrap();
                for expected in payloads {
                    assert!(solved.artifacts.iter().any(|a| a.path == expected.path
                        && a.role == expected.role
                        && a.bytes == expected.bytes));
                }
                assert_eq!(solved.outcome["execution_trace_complete_verified"], false);
                assert_eq!(solved.outcome["total_operation_count_verified"], false);
            }
        }
    }
    #[test]
    fn generic_v2_finish_refusal_and_all_native_budget_kinds_are_censored() {
        let cfg = cfg("prime", 13, true, 3);
        let case = case(&cfg);
        let full = transcript(&solve(&cfg, &case).unwrap());
        let first = &full.batches[0].native;
        assert_eq!(first.stop, FilterStop::Completed);
        let defaults = FilterTraceLimits::default();
        for (policy, stop) in [
            (
                FilterTraceLimits {
                    events: first.events.len() as u64 - 1,
                    ..defaults
                },
                FilterStop::EventLimit,
            ),
            (
                exact_prefix_byte_policy(first, first.events.len() - 1),
                FilterStop::ByteLimit,
            ),
            (
                FilterTraceLimits {
                    work: 0,
                    ..defaults
                },
                FilterStop::WorkLimit,
            ),
            (
                FilterTraceLimits {
                    input_rows: 0,
                    ..defaults
                },
                FilterStop::InputLimit,
            ),
            (
                FilterTraceLimits {
                    row_terms: 0,
                    ..defaults
                },
                FilterStop::StateLimit,
            ),
        ] {
            let solved = solve_v2(&cfg, &case, policy).unwrap();
            let t = transcript(&solved);
            verify_v2(&case.public_instance, &case.factor_base, &t, policy).unwrap();
            assert_eq!(t.batches.len(), 1);
            let final_batch = &t.batches[0];
            assert_eq!(final_batch.native.stop, stop);
            assert!(final_batch.complete().is_none());
            assert!(t.recovered_scalar.is_none() && t.recovered_column_logs.is_none());
            assert_eq!(t.attempts.len(), final_batch.binding.through_attempts);
            for key in [
                "solver_completed",
                "scalar_recovered_without_witness",
                "scalar_replay_verified",
                "ecdlp_pipeline_completed",
            ] {
                assert_eq!(solved.outcome[key], false, "{key}");
            }
            if stop == FilterStop::EventLimit {
                assert_eq!(
                    final_batch.native.next_reservation.unwrap().phase,
                    trace::Phase::Finish
                );
                assert!(!final_batch.native.rows.is_empty());
            }
            let mut bad = t.clone();
            bad.batches[0].terminal = full.batches[0].terminal.clone();
            assert!(verify_v2(&case.public_instance, &case.factor_base, &bad, policy).is_err());
            let mut bad = t.clone();
            bad.attempts.push(full.attempts[0].clone());
            assert!(verify_v2(&case.public_instance, &case.factor_base, &bad, policy).is_err());
        }
    }
    #[test]
    fn native_budget_is_shared_across_cumulative_batches_including_zero_remainder() {
        // Independent of query success: the same public accepted prefix is
        // cumulatively refiltered, exactly as successive scheduled batches do.
        let cfg = cfg("prime", 13, false, 3);
        let case = case(&cfg);
        let full = transcript(&solve(&cfg, &case).unwrap());
        let first = &full.batches[0];
        let sources: Vec<_> = full.attempts[..first.binding.through_attempts]
            .iter()
            .filter_map(|a| a.row.clone())
            .collect();
        let r = number(&case.public_instance["curve"]["subgroup_order"], "r").unwrap();
        let options = filter_options(r, &full.limits);
        for policy in [
            FilterTraceLimits {
                events: first.native.events.len() as u64,
                ..FilterTraceLimits::default()
            },
            exact_prefix_byte_policy(&first.native, first.native.events.len()),
            FilterTraceLimits {
                work: first.native.computed_work,
                ..FilterTraceLimits::default()
            },
        ] {
            let one = trace::filter_large_primes_with_trace(
                sources.clone(),
                &options,
                policy,
                &mut |_| Ok(()),
            )
            .unwrap();
            assert_eq!(one.stop, FilterStop::Completed);
            let mut totals = TraceTotals::default();
            totals.charge(&one, policy).unwrap();
            let remaining = totals.remaining(policy).unwrap();
            assert!(remaining.events == 0 || remaining.bytes == 0 || remaining.work == 0);
            let mut cumulative = sources.clone();
            let mut next = sources[0].clone();
            next.id = full.attempts.len() as u64 + 1;
            cumulative.push(next);
            let two = trace::filter_large_primes_with_trace(
                cumulative.clone(),
                &options,
                remaining,
                &mut |_| Ok(()),
            )
            .unwrap();
            trace::verify_filter_trace(&cumulative, &options, &two).unwrap();
            assert_ne!(two.stop, FilterStop::Completed);
            totals.charge(&two, policy).unwrap();
            assert!(
                totals.computed_work <= policy.work
                    && totals.attempted_events <= policy.events
                    && totals.acknowledged_bytes <= policy.bytes
            );
            let mut forged = two.clone();
            forged.limits = policy;
            assert_ne!(forged.limits, remaining);
        }
    }
    #[test]
    fn generic_v2_rejects_source_trace_lifecycle_config_and_outcome_tampering() {
        let cfg = cfg("prime", 13, true, 3);
        let case = case(&cfg);
        let solved = solve(&cfg, &case).unwrap();
        let t = transcript(&solved);
        for mutate in [
            (|t: &mut TranscriptV2| t.binding.factor_base_sha256 = "0".repeat(64))
                as fn(&mut TranscriptV2),
            |t| t.batches[0].binding.ordered_sources_sha256 = "0".repeat(64),
            |t| t.batches[0].binding.source_count += 1,
            |t| t.batches[0].binding.through_attempts -= 1,
            |t| t.batches[0].native.computed_work += 1,
            |t| t.batches[0].native.events[0].sequence += 1,
            |t| {
                t.batches[0].native.events.remove(1);
            },
            |t| {
                t.batches[0].native.events.swap(0, 1);
            },
            |t| {
                t.batches.remove(0);
            },
            |t| {
                t.batches.push(t.batches[0].clone());
            },
            |t| t.trace_totals.acknowledged_bytes += 1,
            |t| t.attempts[0].coef_b = 0,
            |t| {
                t.attempts.pop();
            },
            |t| t.execution_trace_complete_verified = true,
            |t| t.config.seed ^= 1,
        ] {
            let mut bad = t.clone();
            mutate(&mut bad);
            assert!(verify(&case.public_instance, &case.factor_base, &value(&bad)).is_err());
        }
        let mut wrong = cfg.clone();
        wrong.key_index += 1;
        assert!(verify_v2_labels(
            &case.public_instance,
            &case.factor_base,
            &wrong,
            &value(&t),
            &solved.outcome
        )
        .is_err());
        for key in [
            "status",
            "trace_complete",
            "total_operation_count_verified",
            "dependency_basis_complete_verified",
            "recorded_row_transitions_verified",
            "filter_trace_censored",
            "counters",
        ] {
            let mut bad = solved.outcome.clone();
            bad[key] = Value::Null;
            assert!(
                verify_v2_labels(
                    &case.public_instance,
                    &case.factor_base,
                    &cfg,
                    &value(&t),
                    &bad
                )
                .is_err(),
                "{key}"
            );
        }
        let mut sink = t.batches[0].native.clone();
        sink.stop = FilterStop::SinkFailure;
        sink.sink_error = Some("injected disk error".into());
        assert!(native_failure(&sink)
            .unwrap_err()
            .contains("injected disk error"));
    }

    #[test]
    fn live_native_prefix_survives_callback_and_batch_payload_failure_without_sealing() {
        use crate::ml_dataset::capture::{AttemptBinding, CaptureLimits};
        use crate::ml_dataset::solvers::{solve_with_bound_event_sink, solve_with_event_sink};
        use std::fs;
        let cfg = cfg("prime", 13, true, 3);
        let case = case(&cfg);
        let root = std::env::temp_dir().join(format!(
            "generic-filter-live-{}-{}",
            std::process::id(),
            crate::ml_dataset::unix_nanos().unwrap()
        ));
        fs::create_dir(&root).unwrap();
        let binding = AttemptBinding {
            schema: "crypto.ml-dataset.attempt-binding/v1".into(),
            run_sha256: "1".repeat(64),
            plan_sha256: "2".repeat(64),
            protocol_sha256: "3".repeat(64),
            config_sha256: canonical_hash(&cfg).unwrap(),
            source_commit: "a".repeat(40),
            binary_sha256: "5".repeat(64),
            cell_index: 7,
            attempt_sha256: "6".repeat(64),
        };
        let limits = CaptureLimits {
            max_event_bytes: 1024 * 1024,
            max_stream_bytes: 64 * 1024 * 1024,
            max_payload_bytes: 64 * 1024 * 1024,
            max_payload_count: 1000,
            max_capture_bytes: 512 * 1024 * 1024,
            chunk_bytes: 1024 * 1024,
            max_chunks: 1000,
            metadata_reserve_bytes: 4096,
        };
        let full = root.join("full");
        let result =
            solve_with_bound_event_sink(&cfg, &case, &full, binding.clone(), limits.clone())
                .unwrap();
        assert!(result.artifacts.iter().any(|a| a.role == FILTER_ROLE));
        let bytes = fs::read(full.join("events.jsonl")).unwrap();
        let mut end = 0usize;
        let mut cutoff = None;
        let mut native_events = 0;
        let mut max_line = 512;
        for line in bytes.split_inclusive(|&b| b == b'\n') {
            let row: Value = serde_json::from_slice(line).unwrap();
            max_line = max_line.max(line.len());
            if row["event"]["phase"] == "generic_large_prime_native_filter_event" {
                native_events += 1;
                if native_events == 5 {
                    cutoff = Some(end);
                    break;
                }
            }
            end += line.len();
        }
        let cutoff = cutoff.expect("native events were durably emitted");
        assert!(max_line <= cutoff);
        let failed = root.join("callback-failed");
        let mut cap = limits;
        cap.max_stream_bytes = cutoff as u64;
        cap.max_event_bytes = max_line as u64;
        assert!(solve_with_bound_event_sink(&cfg, &case, &failed, binding, cap).is_err());
        let retained = fs::read(failed.join("events.jsonl")).unwrap();
        assert_eq!(retained, &bytes[..cutoff]);
        assert!(!failed
            .join("payloads/generic-large-prime-transcript.json")
            .exists());
        let payload_failed = root.join("payload-failed");
        fs::create_dir_all(payload_failed.join("payloads")).unwrap();
        fs::write(
            payload_failed.join("payloads").join(filter_path(0)),
            b"injected conflicting payload",
        )
        .unwrap();
        let error = solve_with_event_sink(&cfg, &case, &payload_failed).unwrap_err();
        assert!(error.contains("collision"));
        let rows = fs::read_to_string(payload_failed.join("events.jsonl")).unwrap();
        assert!(rows.contains("generic_large_prime_native_filter_event"));
        assert!(rows.contains("\"kind\":\"finish\""));
        assert!(!payload_failed
            .join("payloads/generic-large-prime-transcript.json")
            .exists());
        fs::remove_dir_all(root).unwrap();
    }
}
