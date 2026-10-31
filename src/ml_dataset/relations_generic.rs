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
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

const SCHEMA: &str = "ml-ecdlp-generic-large-prime-transcript/v1";
const PRIVATE_SCHEMA: &str = "ml-ecdlp-generic-large-prime-membership/v1";

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
pub(super) fn solve(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
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

pub(super) fn verify(public: &Value, base: &Value, saved: &Value) -> Result<(), String> {
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
    use super::*;
    fn config(family: &str, n: u32, twist: bool, m: usize) -> CaseConfig {
        CaseConfig {
            factor_base_policy: None,
            family: family.into(),
            n,
            m,
            twist,
            curve_seed: 7,
            generator_index: 1,
            key_index: 2,
            seed: 42,
            factor_base_size: 2,
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
