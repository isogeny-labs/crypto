//! Independent generic-v2 supplied-domain, source, filter-prefix and rank checks.
//! Construction completeness, dependency-basis completeness and total cost are
//! explicitly excluded. Legacy v1 remains integrity-only in this module.
mod budget;
mod filter;
mod json;
mod rank;
pub mod schema;
#[cfg(test)]
mod tests;
use crate::{
    arithmetic::{self, Curve, Point},
    common::*,
};
use anyhow::{ensure, Context, Result};
use budget::Budget;
use rand::{rngs::StdRng, Rng, SeedableRng};
use schema::*;
use serde::Serialize;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
pub const ROLE: &str = "large-prime-filter-certificates/v1";
const PATH: &str = "generic-large-prime-transcript.json";
const TRANSCRIPT_ROLE: &str = "large-prime-elimination-transcript";
const SCHEMA: &str = "ml-ecdlp-generic-large-prime-transcript/v2";
const POLICY: &str = "aggregate-native-filter-admission/v1";
use json::FILE_MAX;
fn sorted(value: Value) -> Value {
    match value {
        Value::Object(m) => Value::Object(
            m.into_iter()
                .map(|(k, v)| (k, sorted(v)))
                .collect::<BTreeMap<_, _>>()
                .into_iter()
                .collect(),
        ),
        Value::Array(a) => Value::Array(a.into_iter().map(sorted).collect()),
        v => v,
    }
}
fn hash<T: Serialize + ?Sized>(v: &T) -> Result<String> {
    Ok(digest(&serde_json::to_vec(&sorted(serde_json::to_value(
        v,
    )?))?))
}
fn number(v: &Value, name: &str) -> Result<u64> {
    v.as_u64()
        .with_context(|| format!("invalid unsigned {name}"))
}
/// Returns no field for legacy evidence, preserving its historical export shape.
pub fn audit(
    root: &Path,
    members: &BTreeMap<String, Value>,
    public: &Value,
    base: &Value,
    config: &Value,
    outcome: &Value,
    arithmetic: bool,
) -> Result<Option<Value>> {
    let batch_members: Vec<_> = members
        .iter()
        .filter(|(p, a)| a["role"] == ROLE || p.starts_with("generic-large-prime-filter-batch-"))
        .collect();
    let has_claim = [
        "filter_trace_policy",
        "filter_trace_censored",
        "recorded_row_transitions_verified",
    ]
    .iter()
    .any(|k| outcome.get(*k).is_some());
    for (path, candidate) in members
        .iter()
        .filter(|(p, a)| a["role"] == TRANSCRIPT_ROLE && p.as_str() != PATH)
    {
        let other = json::classify(root, candidate)?;
        ensure!(
            other.as_deref() != Some(SCHEMA),
            "generic v2 transcript under wrong path: {path}"
        );
    }
    let Some(artifact) = members.get(PATH) else {
        ensure!(
            batch_members.is_empty() && !has_claim,
            "orphan generic filter evidence"
        );
        return Ok(None);
    };
    let classification = json::classify(root, artifact)?;
    if classification.as_deref() != Some(SCHEMA) {
        ensure!(
            batch_members.is_empty() && !has_claim,
            "new filter claims without v2"
        );
        ensure!(
            classification.as_deref() == Some("ml-ecdlp-generic-large-prime-transcript/v1"),
            "unknown generic transcript schema"
        );
        load_artifact_json(root, artifact)?; // Legacy duplicate-key and Value behavior is unchanged.
        return Ok(None);
    }
    ensure!(
        artifact["role"] == TRANSCRIPT_ROLE
            && artifact["encoding"] == "json"
            && number(&artifact["bytes"], "transcript bytes")? <= FILE_MAX,
        "generic transcript role/size"
    );
    ensure!(
        members
            .values()
            .filter(|a| a["role"] == TRANSCRIPT_ROLE)
            .count()
            == 1,
        "duplicate generic transcript"
    );
    let (saved, retained) = json::load(root, artifact, budget::MEMORY)?;
    let mut budget = Budget {
        retained,
        ..Budget::default()
    };
    shape_admission(&saved)?;
    let t: Transcript = serde_json::from_value(saved).context("strict generic v2 schema")?;
    let auxiliary = |name: &str, role: &str| -> Result<Value> {
        let member = members
            .get(name)
            .context("missing generic domain/pair payload")?;
        ensure!(member["role"] == role, "generic auxiliary role");
        let (value, _) = json::load(root, member, budget::MEMORY - budget.retained)?;
        Ok(value)
    };
    {
        let d: DomainsPayload = serde_json::from_value(auxiliary(
            "generic-large-prime-domains.json",
            "exact-large-prime-discovery-domains",
        )?)?;
        ensure!(
            d.orbit_action == t.orbit_action && d.small == t.small && d.residual == t.residual,
            "generic domain payload differs"
        );
    }
    {
        let p: PairsPayload = serde_json::from_value(auxiliary(
            "generic-large-prime-pairs.json",
            "complete-large-prime-pair-table",
        )?)?;
        ensure!(
            p.pairs == t.pairs && p.identity_sums_excluded && p.duplicate_sums_keep_first,
            "generic pair payload differs"
        );
    }
    if let Some(scalar) = t.recovered_scalar {
        let member = members
            .get("scalar-recovery.json")
            .context("missing scalar recovery payload")?;
        let saved = load_artifact_json(root, member)?;
        ensure!(
            member["role"] == "restricted-independent-scalar-label"
                && arithmetic::decimal(&saved["scalar"], "recovered scalar")? == scalar
                && saved["generator"] == public["generator"]
                && saved["target"] == public["target"]
                && saved["read_private_witness"] == false,
            "generic scalar artifact differs"
        );
    }
    ensure!(
        batch_members.len() == t.batches.len(),
        "orphan/missing generic batch artifacts"
    );
    let mut hashes = Vec::new();
    for (i, b) in t.batches.iter().enumerate() {
        let name = format!("generic-large-prime-filter-batch-{i:04}.json");
        let a = members.get(&name).context("missing indexed native batch")?;
        ensure!(a["role"] == ROLE, "wrong native batch role");
        let (payload, _) = json::load(root, a, budget::MEMORY - budget.retained)?;
        let decoded: TracedBatch = serde_json::from_value(payload)?;
        ensure!(decoded == *b, "batch payload/transcript differs");
        hashes.push(a["sha256"].clone());
    }
    let mut checked = verify(public, base, config, outcome, &t, arithmetic, &mut budget)?;
    checked["transcript_sha256"] = artifact["sha256"].clone();
    checked["batch_sha256"] = json!(hashes);
    Ok(Some(checked))
}
fn array_cap(v: &Value, max: usize, name: &str) -> Result<()> {
    ensure!(
        v.as_array().is_some_and(|a| a.len() <= max),
        "consumer_resource_limit: {name}"
    );
    Ok(())
}
fn shape_admission(t: &Value) -> Result<()> {
    for (v, max, name) in [
        (&t["small"]["points"], 4096, "small points"),
        (&t["small"]["columns"], 4096, "small columns"),
        (&t["small"]["signed"], 4096, "signed small"),
        (&t["residual"]["domain"]["points"], 127, "residual points"),
        (&t["residual"]["domain"]["columns"], 32, "residual columns"),
        (&t["pairs"], 8192, "pairs"),
        (&t["attempts"], 4096, "attempts"),
        (&t["batches"], 129, "batches"),
    ] {
        array_cap(v, max, name)?;
    }
    for batch in t["batches"].as_array().unwrap() {
        array_cap(&batch["native"]["events"], 100_000, "events")?;
        array_cap(&batch["native"]["rows"], 4096, "complete rows")?;
        if let Some(matrix) = batch["terminal"]["legacy_batch"].get("matrix") {
            array_cap(&matrix["rref"], 4096, "matrix rows")?;
            for row in matrix["rref"].as_array().unwrap() {
                array_cap(row, 4097, "matrix columns")?;
            }
        }
    }
    Ok(())
}
impl Totals {
    fn remaining(&self) -> Result<Limits> {
        let mut p = Limits::policy();
        p.events = p
            .events
            .checked_sub(self.attempted_events)
            .context("aggregate events")?;
        p.bytes = p
            .bytes
            .checked_sub(self.acknowledged_bytes)
            .context("aggregate bytes")?;
        p.work = p
            .work
            .checked_sub(self.computed_work)
            .context("aggregate work")?;
        Ok(p)
    }
    fn charge(&mut self, r: &Run) -> Result<()> {
        fn plus(a: &mut u64, b: u64) -> Result<()> {
            *a = a.checked_add(b).context("aggregate overflow")?;
            Ok(())
        }
        plus(&mut self.acknowledged_events, r.events.len() as u64)?;
        plus(
            &mut self.attempted_events,
            r.events.len() as u64 + u64::from(r.unacknowledged.is_some()),
        )?;
        plus(&mut self.acknowledged_bytes, r.acknowledged_bytes)?;
        plus(&mut self.refused_event_bytes, r.unacknowledged_bytes)?;
        plus(&mut self.acknowledged_work, r.acknowledged_work)?;
        plus(&mut self.computed_work, r.computed_work)?;
        plus(&mut self.inverse_calls, r.inverse_calls)?;
        plus(
            &mut self.acknowledged_inverse_calls,
            r.acknowledged_inverse_calls,
        )?;
        self.remaining()?;
        Ok(())
    }
}
fn structural(
    public: &Value,
    base: &Value,
    config: &Value,
    outcome: &Value,
    t: &Transcript,
) -> Result<()> {
    ensure!(
        t.schema == SCHEMA
            && t.public_instance == *public
            && t.config == *config
            && config["solver"] == "double-large-prime"
            && base["construction_status"] != "unsupported",
        "generic context"
    );
    ensure!(
        (2..=15).contains(&t.m)
            && number(&config["m"], "m")? == t.m as u64
            && number(&config["work_limit"], "work")? == t.requested_work
            && t.limits == CollectionLimits::new(t.requested_work),
        "generic parameters"
    );
    for key in ["family", "n", "m", "twist"] {
        ensure!(public[key] == config[key], "generic public/config {key}");
    }
    ensure!(
        t.orbit_action == "identity-with-signed-negation"
            && t.trace_policy == POLICY
            && t.trace_limits == Limits::policy()
            && t.trace_work_scope == WORK_SCOPE
            && t.trace_byte_scope == BYTE_SCOPE,
        "generic policy/scope"
    );
    ensure!(
        t.recorded_row_transitions_verified
            && !t.execution_trace_complete_verified
            && !t.total_operation_count_verified
            && !t.dependency_basis_complete_verified,
        "generic scope claim"
    );
    ensure!(
        t.attempts.len() <= t.limits.queries && t.batches.len() <= t.limits.queries / 32 + 1,
        "generic collection shape"
    );
    let expected = Bindings {
        public_sha256: hash(public)?,
        factor_base_sha256: hash(base)?,
        config_sha256: hash(config)?,
        domains_and_pairs_sha256: hash(
            &json!({"small":t.small,"residual":t.residual,"pairs":t.pairs}),
        )?,
    };
    ensure!(expected == t.binding, "generic canonical binding");
    let mut totals = Totals::default();
    let mut through = 0;
    for (i, b) in t.batches.iter().enumerate() {
        ensure!(
            b.schema == "ml-dataset.generic-large-prime-filter-batch/v1"
                && b.binding.case == t.binding
                && b.binding.batch_index == i
                && b.binding.through_attempts > through
                && b.binding.through_attempts <= t.attempts.len()
                && b.native.limits == totals.remaining()?,
            "generic batch binding/remaining caps"
        );
        let supplied: Vec<_> = t.attempts[..b.binding.through_attempts]
            .iter()
            .filter_map(|a| a.row.as_ref())
            .collect();
        ensure!(
            b.binding.source_count == supplied.len()
                && b.binding.ordered_sources_sha256 == hash(&supplied)?,
            "ordered supplied source binding"
        );
        ensure!(
            b.native.stop != Stop::SinkFailure && b.native.sink_error.is_none(),
            "sealed worker sink failure"
        );
        totals.charge(&b.native)?;
        through = b.binding.through_attempts;
        match &b.terminal {
            Terminal::Completed { legacy_batch: x } => ensure!(
                b.native.stop == Stop::Completed
                    && b.native.report_final
                    && x.through_attempts == through
                    && x.source_count == b.binding.source_count
                    && x.completed == b.native.rows
                    && x.report == b.native.report,
                "completed batch labels"
            ),
            Terminal::Censored { stop } => ensure!(
                i + 1 == t.batches.len()
                    && *stop != Stop::Completed
                    && *stop == b.native.stop
                    && !b.native.report_final
                    && through == t.attempts.len()
                    && t.recovered_scalar.is_none()
                    && t.recovered_column_logs.is_none(),
                "censored batch supplies later work/rank/recovery"
            ),
        }
    }
    ensure!(totals == t.trace_totals, "aggregate native totals");
    outcome_binding(t, outcome)
}
fn censored(t: &Transcript) -> bool {
    t.batches
        .last()
        .is_some_and(|b| matches!(b.terminal, Terminal::Censored { .. }))
}
fn outcome_binding(t: &Transcript, o: &Value) -> Result<()> {
    let recovered = t.recovered_scalar.is_some();
    let censor = censored(t);
    let reports: Vec<_> = t
        .batches
        .iter()
        .filter_map(|b| match &b.terminal {
            Terminal::Completed { legacy_batch } => Some(&legacy_batch.report),
            _ => None,
        })
        .collect();
    let expected = json!({"status":if recovered {"verified"} else {"budget_exhausted"},"solver_completed":recovered,"pdp_witness_verified":false,"scalar_recovered_without_witness":recovered,"scalar_replay_verified":recovered,"ecdlp_pipeline_completed":recovered,"trace_complete":false,"read_private_witness":false,"factor_base_logs_complete":t.recovered_column_logs.is_some(),"orbit_action":"identity-with-signed-negation","native_backend":"large_prime_filter::trace::filter_large_primes_with_trace","requested_m":t.m,"terminal_detail":t.terminal,"filter_trace_policy":POLICY,"filter_trace_censored":censor,"filter_trace_stop":t.batches.last().map(|b|b.native.stop),"recorded_row_transitions_verified":true,"execution_trace_complete_verified":false,"total_operation_count_verified":false,"dependency_basis_complete_verified":false,"residual_projection_setup":t.residual.projection_metrics,"counters":{"query_attempts":t.attempts.len(),"accepted_partials":t.attempts.iter().filter(|a|a.row.is_some()).count(),"pair_add_calls":t.pairs.len(),"query_scalar_mul_calls":2*t.attempts.len(),"query_group_add_calls":(t.m-1)*t.attempts.len(),"native_filter_calls":t.batches.len(),"native_filter_reports":reports,"native_filter_trace":t.trace_totals}});
    for (key, value) in expected.as_object().unwrap() {
        ensure!(o[key] == *value, "generic outcome {key}");
    }
    ensure!(
        !censor || t.terminal == "filter_trace_censored",
        "censor terminal"
    );
    Ok(())
}
fn scalar(c: &Curve, p: Point, k: u64, b: &mut Budget) -> Result<Point> {
    b.curves(2 * u64::from(64 - k.leading_zeros()))?;
    c.mul(p, k)
}
fn add(c: &Curve, a: Point, d: Point, b: &mut Budget) -> Result<Point> {
    b.curves(1)?;
    c.add(a, d)
}
fn point_json(p: Point) -> Value {
    p.map_or(
        Value::Null,
        |(x, y)| json!({"x":x.to_string(),"y":y.to_string()}),
    )
}
fn domain(c: &Curve, d: &Domain, r: u64, b: &mut Budget) -> Result<Vec<Point>> {
    let mut pts = Vec::new();
    let mut seen = BTreeSet::new();
    let mut columns = Vec::new();
    let mut labels = Vec::new();
    let mut lookup = BTreeMap::new();
    for v in &d.points {
        let p = c.point(v, "domain point", true)?;
        ensure!(
            seen.insert(p) && scalar(c, p, r, b)?.is_none(),
            "domain duplicates/torsion"
        );
        let rep = p.min(c.neg(p)?);
        let column = if let Some(i) = lookup.get(&rep) {
            *i
        } else {
            let i = columns.len();
            columns.push(point_json(rep));
            lookup.insert(rep, i);
            i
        };
        labels.push(Signed {
            column,
            coefficient: if p == rep { 1 } else { r - 1 },
        });
        pts.push(p);
    }
    ensure!(
        d.columns == columns && d.signed == labels,
        "signed domain representatives/order"
    );
    Ok(pts)
}
fn source(a: &Attempt, t: &Transcript, r: u64, b: &mut Budget) -> Result<Option<Partial>> {
    let Some((i, j)) = a.residual_pair else {
        return Ok(None);
    };
    let mut small = Vec::new();
    let mut large = Vec::new();
    for &k in &a.small_indices {
        let s = t.small.signed.get(k).context("small query index")?;
        small.push((s.column as u32, s.coefficient));
    }
    small.push((t.small.columns.len() as u32, r - a.coef_b));
    for k in [i, j] {
        let s = t
            .residual
            .domain
            .signed
            .get(k)
            .context("residual query index")?;
        large.push((s.column as u64, s.coefficient));
    }
    Ok(Some(Partial {
        id: a.sequence,
        small: filter::normalize(&small, r, b)?,
        large: filter::normalize(&large, r, b)?,
        rhs: a.coef_a,
    }))
}
#[allow(clippy::too_many_arguments)] // Explicit public group/domain binding; no hidden private inputs.
fn equation(
    c: &Curve,
    g: Point,
    q: Point,
    small: &[Point],
    residual: &[Point],
    row: &Partial,
    r: u64,
    b: &mut Budget,
) -> Result<()> {
    ensure!(row.rhs < r, "source RHS");
    let mut lhs = None;
    for &(j, k) in &row.small {
        ensure!(k > 0 && k < r, "source small coefficient");
        let p = if j as usize == small.len() {
            q
        } else {
            *small.get(j as usize).context("source small column")?
        };
        let term = scalar(c, p, k, b)?;
        lhs = add(c, lhs, term, b)?;
    }
    for &(j, k) in &row.large {
        ensure!(k > 0 && k < r, "source residual coefficient");
        let p = *residual.get(j as usize).context("source residual column")?;
        let term = scalar(c, p, k, b)?;
        lhs = add(c, lhs, term, b)?;
    }
    ensure!(
        lhs == scalar(c, g, row.rhs, b)?,
        "independent source group equation"
    );
    Ok(())
}
fn verify(
    public: &Value,
    base: &Value,
    config: &Value,
    outcome: &Value,
    t: &Transcript,
    arithmetic: bool,
    b: &mut Budget,
) -> Result<Value> {
    structural(public, base, config, outcome, t)?;
    let mut result = json!({"status":if arithmetic{"checked_supplied_finite_certificates"}else{"arithmetic_not_requested"},"supplied_batch_count":t.batches.len(),"completed_batches_checked":0,"censored_prefixes_checked":0,"source_equations_checked":0,"query_stream_verified":false,"recorded_native_transitions_verified":false,"complete_matrix_ranks_checked":0,"positive_recovery_checked":false,"execution_trace_complete_verified":false,"total_operation_count_verified":false,"dependency_basis_complete_verified":false,"residual_pool_construction_verified":false,"cyclic_membership_construction_verified":false,"unsat_verified":false,"work_scope":WORK_SCOPE,"byte_scope":BYTE_SCOPE});
    if !arithmetic {
        return Ok(result);
    }
    let context = arithmetic::public_curve(public, config)?;
    let c = &context.curve;
    let g = c.point(&public["generator"], "generator", true)?;
    let q = c.point(&public["target"], "target", true)?;
    let r = arithmetic::decimal(&public["curve"]["subgroup_order"], "subgroup order")?;
    ensure!(
        (3..1u64 << 63).contains(&r)
            && arithmetic::is_prime(r)
            && scalar(c, g, r, b)?.is_none()
            && scalar(c, q, r, b)?.is_none(),
        "prime-order public subgroup equations"
    );
    ensure!(
        base["points"] == json!(t.small.points),
        "small domain differs from factor base"
    );
    let small = domain(c, &t.small, r, b)?;
    let residual = domain(c, &t.residual.domain, r, b)?;
    ensure!(
        residual.len() <= t.limits.residual_points && t.residual.domain.columns.len() <= 32,
        "residual caps"
    );
    let sm: BTreeSet<_> = small
        .iter()
        .copied()
        .chain(
            small
                .iter()
                .map(|p| c.neg(*p))
                .collect::<Result<Vec<_>>>()?,
        )
        .collect();
    let rp: BTreeSet<_> = residual.iter().copied().collect();
    for p in &residual {
        ensure!(
            !sm.contains(p) && rp.contains(&c.neg(*p)?),
            "residual signed disjointness/closure"
        );
    }
    let small_cols = t
        .small
        .columns
        .iter()
        .map(|p| c.point(p, "small column", true))
        .collect::<Result<Vec<_>>>()?;
    let residual_cols = t
        .residual
        .domain
        .columns
        .iter()
        .map(|p| c.point(p, "residual column", true))
        .collect::<Result<Vec<_>>>()?;
    let mut lookup = BTreeMap::new();
    let mut pair_index = 0;
    for i in 0..residual.len() {
        for j in i..residual.len() {
            let sum = add(c, residual[i], residual[j], b)?;
            let saved = t.pairs.get(pair_index).context("missing pair")?;
            ensure!(
                saved.left == i
                    && saved.right == j
                    && c.point(&saved.sum, "pair sum", false)? == sum,
                "pair table"
            );
            if sum.is_some() {
                lookup.entry(sum).or_insert((i, j));
            }
            pair_index += 1;
        }
    }
    ensure!(
        pair_index == t.pairs.len() && pair_index <= t.limits.pairs,
        "pair table cardinality"
    );
    let can_query = !lookup.is_empty() && (t.m == 2 || !small.is_empty());
    let seed = number(&config["seed"], "seed")?
        ^ number(&config["generator_index"], "generator index")?.rotate_left(17)
        ^ number(&config["key_index"], "key index")?.rotate_left(31)
        ^ 0x8a01_731f_94b0_d6c3;
    ensure!(
        usize::BITS == 64,
        "query RNG requires frozen 64-bit sampling"
    );
    let mut rng = StdRng::seed_from_u64(seed);
    let mut sources = Vec::new();
    let mut batch_index = 0;
    let mut recovered = None;
    let mut logs = None;
    let mut censor = false;
    let mut totals = Totals::default();
    let mut completed = 0;
    for (index, a) in t.attempts.iter().enumerate() {
        ensure!(
            can_query && !censor && recovered.is_none() && a.sequence == index as u64,
            "query lifecycle"
        );
        let aa = rng.gen_range(0..r);
        let bb = rng.gen_range(1..r);
        let indices: Vec<usize> = (0..t.m - 2)
            .map(|_| rng.gen_range(0..small.len()))
            .collect();
        ensure!(
            a.coef_a == aa && a.coef_b == bb && a.small_indices == indices,
            "frozen query stream"
        );
        let ag = scalar(c, g, aa, b)?;
        let bq = scalar(c, q, bb, b)?;
        let target = add(c, ag, bq, b)?;
        ensure!(
            c.point(&a.target, "query target", false)? == target,
            "query target equation"
        );
        let mut rem = target;
        for k in indices {
            rem = add(c, rem, c.neg(small[k])?, b)?;
        }
        ensure!(
            c.point(&a.remainder, "remainder", false)? == rem
                && lookup.get(&rem).copied() == a.residual_pair,
            "query remainder/first pair"
        );
        let row = source(a, t, r, b)?;
        ensure!(row == a.row, "query source coefficients");
        let accepted = row.is_some();
        if let Some(row) = row {
            equation(c, g, q, &small_cols, &residual_cols, &row, r, b)?;
            sources.push(row);
        }
        if (accepted && sources.len() % 32 == 0) || index + 1 == t.limits.queries {
            let batch = t
                .batches
                .get(batch_index)
                .context("missing scheduled batch")?;
            ensure!(
                batch.binding
                    == BatchBinding {
                        case: t.binding.clone(),
                        batch_index,
                        through_attempts: index + 1,
                        source_count: sources.len(),
                        ordered_sources_sha256: hash(&sources)?
                    }
                    && batch.native.limits == totals.remaining()?,
                "cumulative source/batch binding"
            );
            filter::verify(
                &sources,
                &Options {
                    modulus: r,
                    max_small_weight: 4096,
                    max_sources: 4096,
                },
                &batch.native,
                b,
            )?;
            totals.charge(&batch.native)?;
            match &batch.terminal {
                Terminal::Censored { .. } => censor = true,
                Terminal::Completed { legacy_batch: x } => {
                    let matrix = rank::reduce(&batch.native.rows, small_cols.len() + 1, r, b)?;
                    ensure!(matrix == x.matrix, "completed matrix certificate");
                    completed += 1;
                    let target = small_cols.len();
                    if matrix.determined[target] {
                        let k = matrix.values[target];
                        ensure!(scalar(c, g, k, b)? == q, "recovered target equation");
                        recovered = Some(k);
                        if matrix.rank == target + 1 {
                            for (p, k) in small_cols.iter().zip(&matrix.values) {
                                ensure!(scalar(c, g, *k, b)? == *p, "recovered base log equation");
                            }
                            logs = Some(matrix.values[..target].to_vec());
                        }
                    }
                }
            }
            batch_index += 1;
        }
    }
    let terminal = if censor {
        "filter_trace_censored"
    } else if recovered.is_some() {
        "verified"
    } else if !can_query {
        "empty_declared_domain"
    } else {
        "budget_exhausted"
    };
    ensure!(
        batch_index == t.batches.len()
            && totals == t.trace_totals
            && recovered == t.recovered_scalar
            && logs == t.recovered_column_logs
            && t.terminal == terminal
            && (!can_query
                || censor
                || recovered.is_some()
                || t.attempts.len() == t.limits.queries),
        "final generic lifecycle/recovery"
    );
    result["completed_batches_checked"] = json!(completed);
    result["censored_prefixes_checked"] = json!(usize::from(censor));
    result["source_equations_checked"] = json!(sources.len());
    result["query_stream_verified"] = json!(!t.attempts.is_empty());
    result["recorded_native_transitions_verified"] = json!(!t.batches.is_empty());
    result["complete_matrix_ranks_checked"] = json!(completed);
    result["positive_recovery_checked"] = json!(recovered.is_some());
    result["consumer_admission"] = json!({"logical_units":b.logical,"reserved_curve_operations":b.curve,"dense_coefficient_cells":b.dense,"decoded_memory_model_bytes":b.retained,"total_cost_measured":false});
    Ok(result)
}
