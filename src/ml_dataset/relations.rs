//! Replayable native one/two-residual relation collection and elimination.
//!
//! Residuals are discovered in an explicit finite point domain. No factor-base
//! log or planted scalar is supplied. This adapter uses the native signed
//! Frobenius eliminator; it exports source combinations rather than claiming to
//! expose the eliminator's private pivot-by-pivot implementation states.
use super::curves;
use super::types::{ArtifactPayload, CaseConfig, CaseData, SolverData};
use crate::binary_ecc::curve::{point_add, point_neg, scalar_mul};
use crate::binary_ecc::{BinaryPoint, F2mElement};
use crate::cryptanalysis::ec_index_calculus::gaussian_eliminate_mod_n_particular;
use crate::cryptanalysis::koblitz_index_calculus::{
    build_explicit_frobenius_point_factor_base, frobenius_eigenvalue, koblitz_point_count,
    point_key, points_with_x, FrobeniusFactorBase, KoblitzCurve,
};
use crate::cryptanalysis::koblitz_large_prime::{
    FeedOutcome, LargePrimeLimits, PartialCandidate, WideFullRelation, WideLargePrimeEliminator,
};
use crate::cryptanalysis::residual_walk::is_prime_u64;
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, HashSet};

#[path = "relations_generic.rs"]
mod generic;

fn point(p: &BinaryPoint) -> Value {
    match p {
        BinaryPoint::Infinity => Value::Null,
        BinaryPoint::Affine { x, y } => {
            json!({"x":x.to_biguint().to_string(),"y":y.to_biguint().to_string()})
        }
    }
}
fn big(v: &Value) -> Result<BigUint, String> {
    let s = v.as_str().ok_or("expected decimal integer string")?;
    let n = BigUint::parse_bytes(s.as_bytes(), 10).ok_or("invalid decimal integer")?;
    if n.to_string() != s {
        return Err("noncanonical integer".into());
    }
    Ok(n)
}
fn base_outcome(status: &str, reason: &str) -> Value {
    json!({"schema":"ml-dataset.solver-outcome/v1","status":status,"reason":reason,
        "solver_completed":false,"pdp_witness_verified":false,"scalar_recovered_without_witness":false,
        "scalar_replay_verified":false,"ecdlp_pipeline_completed":false,"trace_complete":false,
        "first_fall_degree":null,"last_fall_degree":null,"retired_hardware_instructions":null,
        "instruction_count_reason":"native primitive counters are not machine instructions","counters":{}})
}
fn unsupported(reason: &str) -> SolverData {
    SolverData {
        outcome: base_outcome("unsupported", reason),
        artifacts: Vec::new(),
    }
}
fn model(public: &Value) -> Result<KoblitzCurve, String> {
    if public["family"] != "koblitz" {
        return Err("native double-large-prime adapter currently supports Koblitz models; general binary/prime remain planned".into());
    }
    let curve = curves::binary_curve(public)?;
    let a = curve.a.to_biguint().to_u8().ok_or(
        "this twist has a coefficient outside F2; Koblitz Frobenius adapter does not support it",
    )?;
    if a > 1 || curve.b.to_biguint() != BigUint::one() {
        return Err(
            "this twist has coefficients outside F2; Koblitz Frobenius adapter does not support it"
                .into(),
        );
    }
    let r = curve
        .order
        .to_u64()
        .ok_or("subgroup order exceeds native range")?;
    if r <= 2
        || !is_prime_u64(r)
        || curve.generator == BinaryPoint::Infinity
        || !curve.is_on_curve(&curve.generator)
        || scalar_mul(&curve, &curve.generator, &curve.order) != BinaryPoint::Infinity
    {
        return Err("invalid prime-subgroup generator".into());
    }
    let group_order = big(&public["curve"]["group_order"])?;
    if &curve.order * &curve.cofactor != group_order {
        return Err("group order/cofactor mismatch".into());
    }
    if koblitz_point_count(a, curve.m) != group_order {
        return Err("Koblitz cardinality certificate mismatch".into());
    }
    let trace = if a == 0 { -1 } else { 1 };
    let lambda = frobenius_eigenvalue(&curve, trace, &curve.order)
        .ok_or("native Frobenius eigenvalue does not validate against generator")?;
    if lambda.modpow(&BigUint::from(curve.m), &curve.order) != BigUint::one() {
        return Err("Frobenius eigenvalue does not close".into());
    }
    Ok(KoblitzCurve {
        a,
        n: curve.m,
        trace,
        group_order,
        subgroup_order: curve.order.clone(),
        cofactor: curve.cofactor.clone(),
        lambda,
        frobenius_is_endomorphism: true,
        k: 1,
        q: 2,
        a_index: a as u64,
        b_index: 1,
        subfield_basis: vec![F2mElement::one(curve.m)],
        curve,
    })
}

fn close_orbit(kc: &KoblitzCurve, p: &BinaryPoint) -> Result<Vec<BinaryPoint>, String> {
    if scalar_mul(&kc.curve, p, &kc.lambda) != kc.frobenius(p) {
        return Err("point does not share generator Frobenius eigenvalue".into());
    }
    let mut seen = HashSet::new();
    let mut points = Vec::new();
    let mut current = p.clone();
    for _ in 0..kc.n {
        for p in [current.clone(), point_neg(&current)] {
            if p == BinaryPoint::Infinity
                || !kc.curve.is_on_curve(&p)
                || scalar_mul(&kc.curve, &p, &kc.subgroup_order) != BinaryPoint::Infinity
            {
                return Err("orbit point outside subgroup".into());
            }
            if seen.insert(point_key(&p)) {
                points.push(p);
            }
        }
        current = kc.frobenius(&current);
    }
    if current != *p {
        return Err("Frobenius orbit failed to close".into());
    }
    Ok(points)
}
fn explicit_base(kc: &KoblitzCurve, case: &CaseData) -> Result<FrobeniusFactorBase, String> {
    let mut seen = HashSet::new();
    let mut points = Vec::new();
    for p in case.factor_base["points"]
        .as_array()
        .ok_or("missing base points")?
    {
        let p = curves::binary_point(p, kc.n)?;
        for p in close_orbit(kc, &p)? {
            if seen.insert(point_key(&p)) {
                points.push(p);
            }
        }
        if points.len() > 4096 {
            return Err("signed Frobenius base exceeds 4096-point trace cap".into());
        }
    }
    build_explicit_frobenius_point_factor_base(kc, points)
        .ok_or_else(|| "native explicit orbit-base validation failed".into())
}

struct ResidualPool {
    points: Vec<BinaryPoint>,
    construction: Vec<Value>,
    orbits: usize,
    projection_metrics: Value,
    restricted_membership_logs: Vec<Value>,
}
fn residual_pool(
    public: &Value,
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    point_cap: usize,
    emit: bool,
) -> Result<ResidualPool, String> {
    let mut residual_points = Vec::new();
    let small_keys: HashSet<_> = fb.points.iter().map(point_key).collect();
    let mut residual_keys = HashSet::new();
    let mut construction = Vec::new();
    let mut residual_orbits = 0;
    let mut projector = curves::SubgroupProjector::new(public)?;
    let mut restricted_membership_logs = Vec::new();
    for x in 1..=2048u64.min((1u64 << kc.n) - 1) {
        let lifts = points_with_x(&kc.curve, &F2mElement::from_biguint(&x.into(), kc.n));
        let mut candidates = Vec::new();
        for p in lifts {
            let (projected, multiplier, member, diagnostic_log) = projector.project(&point(&p))?;
            let projected = curves::binary_point(&projected, kc.n)?;
            let mut row = json!({"lifted":point(&p),"projected":point(&projected),
                "projection_multiplier":multiplier.to_string(),"cyclic_subgroup_member":member,"accepted":false});
            if let Some(log) = diagnostic_log {
                restricted_membership_logs.push(json!({"x_candidate":x.to_string(),
                    "lift_index":candidates.len(),"projected":point(&projected),"log":log.to_string()}));
            }
            if !member {
                row["reason"] = json!("outside_selected_nonzero_cyclic_subgroup");
            } else if projected != BinaryPoint::Infinity
                && !small_keys.contains(&point_key(&projected))
                && !residual_keys.contains(&point_key(&projected))
            {
                let orbit = close_orbit(kc, &projected)?;
                if orbit.iter().any(|p| small_keys.contains(&point_key(p))) {
                    row["reason"] = json!("orbit_intersects_small_base");
                } else if residual_points.len() + orbit.len() > point_cap {
                    row["reason"] = json!("residual_point_cap");
                } else {
                    row["accepted"] = json!(true);
                    row["orbit"] = json!(orbit.iter().map(point).collect::<Vec<_>>());
                    for p in orbit {
                        if residual_keys.insert(point_key(&p)) {
                            residual_points.push(p);
                        }
                    }
                    residual_orbits += 1;
                }
            }
            candidates.push(row);
        }
        construction.push(json!({"x":x.to_string(),"candidates":candidates}));
        if emit {
            super::solvers::live_event(
                &json!({"phase":"large_prime_residual_construction","record":construction.last()}),
            )?;
        }
        if residual_orbits >= 4 || residual_points.len() + 2 * kc.n as usize > point_cap {
            break;
        }
    }
    Ok(ResidualPool {
        points: residual_points,
        construction,
        orbits: residual_orbits,
        projection_metrics: projector.counters(),
        restricted_membership_logs,
    })
}
fn partial_json(p: &PartialCandidate) -> Value {
    json!({"source_id":p.source_id,"coef_a":p.coef_a.to_string(),"coef_b":p.coef_b.to_string(),"small_point_indices":p.small_point_indices,"residual_points":p.residual_points.iter().map(point).collect::<Vec<_>>()})
}
fn read_partial(v: &Value, n: u32) -> Result<PartialCandidate, String> {
    Ok(PartialCandidate {
        source_id: v["source_id"].as_u64().ok_or("invalid source id")?,
        coef_a: big(&v["coef_a"])?,
        coef_b: big(&v["coef_b"])?,
        small_point_indices: v["small_point_indices"]
            .as_array()
            .ok_or("missing small indices")?
            .iter()
            .map(|x| {
                x.as_u64()
                    .and_then(|x| usize::try_from(x).ok())
                    .ok_or_else(|| "invalid small index".to_string())
            })
            .collect::<Result<_, _>>()?,
        residual_points: v["residual_points"]
            .as_array()
            .ok_or("missing residuals")?
            .iter()
            .map(|x| curves::binary_point(x, n))
            .collect::<Result<_, _>>()?,
    })
}
fn full_json(r: &WideFullRelation) -> Value {
    json!({"coef_a":r.coef_a.to_string(),"coef_b":r.coef_b.to_string(),"small":r.small.iter().map(|(i,c)|json!({"column":i,"coefficient":c.to_string()})).collect::<Vec<_>>(),"source_coeffs":r.source_coeffs.iter().map(|(i,c)|json!({"source_id":i,"coefficient":c.to_string()})).collect::<Vec<_>>(),"merge_depth":r.merge_depth})
}
fn feed_json(outcome: &FeedOutcome) -> Value {
    match outcome {
        FeedOutcome::PivotStored { merge_depth } => {
            json!({"status":"pivot_stored","merge_depth":merge_depth})
        }
        FeedOutcome::Full(row) => json!({"status":"completed_row","row":full_json(row)}),
        FeedOutcome::UnknownCap {
            reason,
            merge_depth,
        } => {
            json!({"status":"unknown_cap","reason":format!("{reason:?}"),"merge_depth":merge_depth})
        }
    }
}
fn verify_partial(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    q: &BinaryPoint,
    p: &PartialCandidate,
) -> Result<(), String> {
    if p.coef_a >= kc.subgroup_order || p.coef_b >= kc.subgroup_order || p.residual_points.len() > 2
    {
        return Err("noncanonical partial coefficients/residual count".into());
    }
    let lhs = point_add(
        &kc.curve,
        &scalar_mul(&kc.curve, kc.generator(), &p.coef_a),
        &scalar_mul(&kc.curve, q, &p.coef_b),
    );
    let mut rhs = BinaryPoint::Infinity;
    for &i in &p.small_point_indices {
        rhs = point_add(
            &kc.curve,
            &rhs,
            fb.points.get(i).ok_or("small index out of range")?,
        );
    }
    for point in &p.residual_points {
        if *point == BinaryPoint::Infinity
            || !kc.curve.is_on_curve(point)
            || scalar_mul(&kc.curve, point, &kc.subgroup_order) != BinaryPoint::Infinity
        {
            return Err("invalid residual subgroup point".into());
        }
        rhs = point_add(&kc.curve, &rhs, point);
    }
    if lhs != rhs {
        return Err("independent partial equation replay failed".into());
    }
    Ok(())
}
fn verify_sources(
    kc: &KoblitzCurve,
    fb: &FrobeniusFactorBase,
    row: &WideFullRelation,
    sources: &BTreeMap<u64, PartialCandidate>,
) -> Result<(), String> {
    let mut a = BigUint::zero();
    let mut b = BigUint::zero();
    let mut residuals = BinaryPoint::Infinity;
    let mut small = BinaryPoint::Infinity;
    for (id, coefficient) in &row.source_coeffs {
        let source = sources
            .get(id)
            .ok_or("completed row references absent source")?;
        a = (a + coefficient * &source.coef_a) % &kc.subgroup_order;
        b = (b + coefficient * &source.coef_b) % &kc.subgroup_order;
        for p in &source.residual_points {
            residuals = point_add(
                &kc.curve,
                &residuals,
                &scalar_mul(&kc.curve, p, coefficient),
            );
        }
        for &index in &source.small_point_indices {
            small = point_add(
                &kc.curve,
                &small,
                &scalar_mul(&kc.curve, &fb.points[index], coefficient),
            );
        }
    }
    let mut reduced_small = BinaryPoint::Infinity;
    for (column, coefficient) in &row.small {
        let index = *fb
            .signed_orbits
            .get(*column)
            .and_then(|x| x.first())
            .ok_or("invalid orbit column")?;
        reduced_small = point_add(
            &kc.curve,
            &reduced_small,
            &scalar_mul(&kc.curve, &fb.points[index], coefficient),
        );
    }
    if a != row.coef_a
        || b != row.coef_b
        || residuals != BinaryPoint::Infinity
        || small != reduced_small
    {
        return Err("completed-row source combination does not eliminate residuals".into());
    }
    Ok(())
}
fn matrix(
    rows: &[WideFullRelation],
    columns: usize,
    r: &BigUint,
) -> Result<(Vec<Vec<BigUint>>, Vec<BigUint>), String> {
    let mut matrix = Vec::new();
    let mut rhs = Vec::new();
    for row in rows {
        let mut values = row.dense_row(columns).map_err(str::to_string)?;
        values.push((r - &row.coef_b) % r);
        matrix.push(values);
        rhs.push(row.coef_a.clone());
    }
    Ok((matrix, rhs))
}

/// Native large-prime discovery, graph reduction, rank and verified recovery.
pub fn solve(config: &CaseConfig, case: &CaseData) -> Result<SolverData, String> {
    if !(2..=15).contains(&config.m) {
        return Err("large-prime arity must lie in 2..15".into());
    }
    let kc = match model(&case.public_instance) {
        Ok(kc) => kc,
        Err(_) => return generic::solve(config, case),
    };
    let q = curves::binary_point(&case.public_instance["target"], kc.n)?;
    if q == BinaryPoint::Infinity
        || !kc.curve.is_on_curve(&q)
        || scalar_mul(&kc.curve, &q, &kc.subgroup_order) != BinaryPoint::Infinity
    {
        return Err("invalid public target".into());
    }
    let fb = match explicit_base(&kc, case) {
        Ok(fb) => fb,
        Err(reason) => return Ok(unsupported(&reason)),
    };
    let r = kc.subgroup_order.to_u64().ok_or("subgroup exceeds word")?;
    let work = config.work_limit.min(100_000);
    let pair_cap = (work / 4).min(8192) as usize;
    let mut point_cap = ((2 * pair_cap) as f64).sqrt().floor() as usize;
    while point_cap * (point_cap + 1) / 2 > pair_cap {
        point_cap -= 1;
    }
    let query_cap = work.min(4096);
    if point_cap < 2 || query_cap == 0 {
        return Ok(SolverData {
            outcome: base_outcome(
                "budget_exhausted",
                "work budget cannot admit a residual pair table",
            ),
            artifacts: Vec::new(),
        });
    }
    let pool = residual_pool(&case.public_instance, &kc, &fb, point_cap, true)?;
    let residual_points = pool.points;
    let construction = pool.construction;
    let residual_orbits = pool.orbits;
    let membership_artifact = ArtifactPayload::json(
        "large-prime-membership-private.json",
        "restricted-large-prime-membership-provenance",
        &json!({"schema":"ml-ecdlp-large-prime-membership-provenance/v1", "generator":case.public_instance["generator"],
            "point_cap":point_cap,"logs":pool.restricted_membership_logs,"projection_metrics":pool.projection_metrics,
            "feature_access":"forbidden; diagnostic setup provenance only", "used_in_relation_elimination":false}),
    )?;
    super::solvers::live_payload(&membership_artifact)?;
    let domain_artifact = ArtifactPayload::json(
        "large-prime-domains.json",
        "exact-large-prime-discovery-domains",
        &json!({"small_points":fb.points.iter().map(point).collect::<Vec<_>>(),"residual_points":residual_points.iter().map(point).collect::<Vec<_>>(),"lambda":kc.lambda.to_string(),"frobenius_power":1}),
    )?;
    super::solvers::live_payload(&domain_artifact)?;
    let mut pairs = Vec::new();
    let mut pair_lookup = HashMap::new();
    for i in 0..residual_points.len() {
        for j in i..residual_points.len() {
            if pairs.len() >= pair_cap {
                return Err("internal residual pair budget mismatch".into());
            }
            let sum = point_add(&kc.curve, &residual_points[i], &residual_points[j]);
            pairs.push(json!({"left":i,"right":j,"sum":point(&sum)}));
            // Opposite residuals form an already smooth relation; exclude those
            // from this discovery lane and retain their table row explicitly.
            if sum != BinaryPoint::Infinity {
                pair_lookup.entry(point_key(&sum)).or_insert((i, j));
            }
        }
    }
    let limits = LargePrimeLimits {
        max_merge_depth: 64,
        max_row_terms: 4096,
        max_pivots: query_cap.max(1) as usize,
    };
    let pair_artifact = ArtifactPayload::json(
        "large-prime-pair-table.json",
        "complete-large-prime-pair-table",
        &json!({"pairs":pairs,"identity_sums_excluded_from_lookup":true,"duplicate_sums_keep_first_pair":true}),
    )?;
    super::solvers::live_payload(&pair_artifact)?;
    let mut eliminator =
        WideLargePrimeEliminator::new(&kc, &fb, q.clone(), limits).map_err(str::to_string)?;
    let mut rng = StdRng::seed_from_u64(config.seed ^ 0x44504c50524f4245);
    let mut attempts = Vec::new();
    let mut full_rows = Vec::new();
    let mut sources = BTreeMap::new();
    let mut rank_history = Vec::new();
    let mut recovered = None;
    let mut recovered_logs = None;
    let setup_group_adds = pairs.len() as u64;
    let mut query_group_adds = 0u64;
    for trial in 0..query_cap {
        if residual_points.is_empty() {
            break;
        }
        let a = rng.gen_range(0..r);
        let b = rng.gen_range(1..r);
        let target = point_add(
            &kc.curve,
            &scalar_mul(&kc.curve, kc.generator(), &a.into()),
            &scalar_mul(&kc.curve, &q, &b.into()),
        );
        query_group_adds += 1;
        let indices: Vec<_> = (0..config.m - 2)
            .map(|_| rng.gen_range(0..fb.points.len()))
            .collect();
        let mut remainder = target.clone();
        for &i in &indices {
            remainder = point_add(&kc.curve, &remainder, &point_neg(&fb.points[i]));
            query_group_adds += 1;
        }
        let mut event = json!({"trial":trial,"coef_a":a.to_string(),"coef_b":b.to_string(),"target":point(&target),"small_point_indices":indices,"remainder":point(&remainder),"status":"no_residual_pair"});
        if let Some(&(i, j)) = pair_lookup.get(&point_key(&remainder)) {
            let candidate = PartialCandidate {
                source_id: trial,
                coef_a: a.into(),
                coef_b: b.into(),
                small_point_indices: indices,
                residual_points: vec![residual_points[i].clone(), residual_points[j].clone()],
            };
            verify_partial(&kc, &fb, &q, &candidate)?;
            event["partial"] = partial_json(&candidate);
            event["residual_pair_indices"] = json!([i, j]);
            sources.insert(trial, candidate.clone());
            let feed = eliminator.feed(candidate).map_err(str::to_string)?;
            event["status"] = json!("partial_verified");
            event["feed"] = feed_json(&feed);
            if let FeedOutcome::Full(row) = feed {
                verify_sources(&kc, &fb, &row, &sources)?;
                full_rows.push(row);
                let (mut values, mut rhs) =
                    matrix(&full_rows, fb.signed_orbits.len(), &kc.subgroup_order)?;
                let reduction =
                    gaussian_eliminate_mod_n_particular(&mut values, &mut rhs, &kc.subgroup_order)
                        .ok_or("native row reduction failed")?;
                let target_column = fb.signed_orbits.len();
                rank_history.push(json!({"completed_rows":full_rows.len(),"columns":target_column+1,"rank":reduction.rank,"determined_columns":reduction.determined,"rref":values.iter().map(|row|row.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"reduced_rhs":rhs.iter().map(ToString::to_string).collect::<Vec<_>>()}));
                if let Some(logs) = eliminator
                    .solve_completed_full_rank(&full_rows)
                    .map_err(str::to_string)?
                {
                    if scalar_mul(&kc.curve, kc.generator(), &logs.target_log) != q {
                        return Err("independent recovered scalar replay failed".into());
                    }
                    recovered = Some(logs.target_log);
                    recovered_logs = Some(logs.column_logs);
                } else if reduction.determined.get(target_column) == Some(&true) {
                    let scalar = reduction.values[target_column].clone();
                    if scalar_mul(&kc.curve, kc.generator(), &scalar) != q {
                        return Err("determined target-column scalar replay failed".into());
                    }
                    recovered = Some(scalar);
                }
            }
        }
        super::solvers::live_event(&json!({"phase":"large_prime_query","record":event}))?;
        attempts.push(event);
        if recovered.is_some() {
            break;
        }
    }
    let stats = eliminator.stats();
    let transcript = json!({"schema":"ml-ecdlp-large-prime-transcript/v1","target":point(&q),"m":config.m,"requested_work_limit":config.work_limit,
        "model":{"lambda":kc.lambda.to_string(),"frobenius_power":1,"generator":point(kc.generator())},
        "small_base":{"points":fb.points.iter().map(point).collect::<Vec<_>>(),"signed_orbits":fb.signed_orbits,"signed_orbit_of":fb.signed_orbit_of},
        "residual_pool":{"points":residual_points.iter().map(point).collect::<Vec<_>>(),"construction":construction,"signed_orbit_count":residual_orbits,"pair_table":pairs,"projection_metrics":pool.projection_metrics},
        "limits":{"max_merge_depth":limits.max_merge_depth,"max_row_terms":limits.max_row_terms,"max_pivots":limits.max_pivots,"query_cap":query_cap,"pair_cap":pair_cap,"point_cap":point_cap,"x_candidate_cap":2048},
        "attempts":attempts,"completed_rows":full_rows.iter().map(full_json).collect::<Vec<_>>(),"rank_history":rank_history,
        "recovered_scalar":recovered.as_ref().map(ToString::to_string),"recovered_column_logs":recovered_logs.as_ref().map(|logs|logs.iter().map(ToString::to_string).collect::<Vec<_>>()),
        "trace_scope":"complete pool candidates, pair sums, sampled query attempts, accepted partials, feed outcomes, completed source combinations and rank reductions; native internal pivot snapshots are not exported"});
    let transcript_artifact = ArtifactPayload::json(
        "large-prime-transcript.json",
        "large-prime-elimination-transcript",
        &transcript,
    )?;
    super::solvers::live_payload(&transcript_artifact)?;
    verify_transcript(&case.public_instance, &case.factor_base, &transcript)?;
    let mut outcome = base_outcome(
        if recovered.is_some() {
            "verified"
        } else {
            "budget_exhausted"
        },
        if recovered.is_some() {
            "Public-input double-large-prime relation discovery and native graph elimination recovered and independently replayed the target scalar"
        } else {
            "Bounded public-input partial relation discovery finished without a determined target scalar"
        },
    );
    outcome["solver_completed"] = json!(recovered.is_some());
    outcome["scalar_recovered_without_witness"] = json!(recovered.is_some());
    outcome["scalar_replay_verified"] = json!(recovered.is_some());
    outcome["ecdlp_pipeline_completed"] = json!(recovered.is_some());
    outcome["factor_base_logs_complete"] = json!(recovered_logs.is_some());
    outcome["requested_m"] = json!(config.m);
    outcome["small_summands_per_query"] = json!(config.m - 2);
    outcome["residual_summands_per_query"] = json!(2);
    outcome["residual_signed_orbits"] = json!(residual_orbits);
    outcome["read_private_witness"] = json!(false);
    outcome["residual_projection_setup"] = pool.projection_metrics;
    outcome["trace_gaps"] = json!([
        "native internal pivot and row-reduction arithmetic snapshots",
        "hardware retired instruction counters"
    ]);
    outcome["counters"] = json!({"query_attempts":attempts.len(),"accepted_partials":stats.accepted_candidates,"pivot_rows":stats.pivot_rows,"pivot_reductions":stats.pivot_reductions,"completed_rows":stats.full_rows,"rows_completed_by_merging":stats.full_from_partials,"capped_rows":stats.capped_rows,"setup_pair_add_calls":setup_group_adds,"query_group_add_calls":query_group_adds,"query_scalar_mul_calls":2*attempts.len(),"independent_source_replay_calls":full_rows.len(),"rank_reductions":rank_history.len()});
    let mut artifacts = vec![
        transcript_artifact,
        domain_artifact,
        pair_artifact,
        membership_artifact,
    ];
    if let Some(scalar) = recovered {
        artifacts.push(ArtifactPayload::json("scalar-recovery.json","restricted-independent-scalar-label",&json!({"scalar":scalar.to_string(),"generator":point(kc.generator()),"target":point(&q),"method":"native-double-large-prime","read_private_witness":false}))?);
    }
    Ok(SolverData { outcome, artifacts })
}

/// Rebuild the native graph from retained public partials and independently
/// check every source combination, rank certificate and recovered scalar.
pub fn verify_transcript(public: &Value, factor_base: &Value, t: &Value) -> Result<(), String> {
    if t["schema"] == "ml-ecdlp-generic-large-prime-transcript/v1" {
        return generic::verify(public, factor_base, t);
    }
    if t["schema"] != "ml-ecdlp-large-prime-transcript/v1" || t["target"] != public["target"] {
        return Err("wrong large-prime transcript identity".into());
    }
    let kc = model(public)?;
    curves::verify_factor_base(public, factor_base)?;
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let expected_base = explicit_base(&kc, &case)?;
    if t["model"]["lambda"] != json!(kc.lambda.to_string())
        || t["model"]["generator"] != public["generator"]
    {
        return Err("wrong Frobenius/generator metadata".into());
    }
    let points = t["small_base"]["points"]
        .as_array()
        .ok_or("missing small base")?
        .iter()
        .map(|p| curves::binary_point(p, kc.n))
        .collect::<Result<_, _>>()?;
    let fb = build_explicit_frobenius_point_factor_base(&kc, points)
        .ok_or("invalid signed-orbit base")?;
    if fb.points != expected_base.points {
        return Err("signed Frobenius base differs from closure of input factor base".into());
    }
    if t["small_base"]["signed_orbits"] != json!(fb.signed_orbits)
        || t["small_base"]["signed_orbit_of"] != json!(fb.signed_orbit_of)
    {
        return Err("wrong signed-orbit maps".into());
    }
    let q = curves::binary_point(&t["target"], kc.n)?;
    let cap = |name: &str| -> Result<usize, String> {
        t["limits"][name]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v > 0 && v <= 100_000)
            .ok_or_else(|| format!("invalid graph limit {name}"))
    };
    let limits = LargePrimeLimits {
        max_merge_depth: cap("max_merge_depth")?,
        max_row_terms: cap("max_row_terms")?,
        max_pivots: cap("max_pivots")?,
    };
    let work = t["requested_work_limit"]
        .as_u64()
        .ok_or("missing work limit")?
        .min(100_000);
    let pair_cap = (work / 4).min(8192) as usize;
    let mut point_cap = ((2 * pair_cap) as f64).sqrt().floor() as usize;
    while point_cap * (point_cap + 1) / 2 > pair_cap {
        point_cap -= 1;
    }
    let query_cap = work.min(4096) as usize;
    if limits.max_merge_depth != 64
        || limits.max_row_terms != 4096
        || limits.max_pivots != query_cap.max(1)
        || cap("query_cap")? != query_cap
        || cap("pair_cap")? != pair_cap
        || cap("point_cap")? != point_cap
        || cap("x_candidate_cap")? != 2048
    {
        return Err("declared limits differ from adapter budget policy".into());
    }
    let mut eliminator =
        WideLargePrimeEliminator::new(&kc, &fb, q.clone(), limits).map_err(str::to_string)?;
    let residuals = t["residual_pool"]["points"]
        .as_array()
        .ok_or("missing residual points")?
        .iter()
        .map(|p| curves::binary_point(p, kc.n))
        .collect::<Result<Vec<_>, _>>()?;
    let expected_pool = residual_pool(public, &kc, &fb, point_cap, false)?;
    if residuals != expected_pool.points
        || t["residual_pool"]["construction"] != json!(expected_pool.construction)
        || t["residual_pool"]["signed_orbit_count"] != json!(expected_pool.orbits)
        || t["residual_pool"]["projection_metrics"] != expected_pool.projection_metrics
    {
        return Err(
            "residual pool or construction receipts differ from independent reconstruction".into(),
        );
    }
    let mut pair_keys = HashMap::new();
    let pairs = t["residual_pool"]["pair_table"]
        .as_array()
        .ok_or("missing pair table")?;
    if pairs.len() > pair_cap || pairs.len() != residuals.len() * (residuals.len() + 1) / 2 {
        return Err("incomplete residual pair table".into());
    }
    let mut pair_index = 0;
    for i in 0..residuals.len() {
        for j in i..residuals.len() {
            let row = &pairs[pair_index];
            pair_index += 1;
            let sum = point_add(&kc.curve, &residuals[i], &residuals[j]);
            if row["left"].as_u64() != Some(i as u64)
                || row["right"].as_u64() != Some(j as u64)
                || row["sum"] != point(&sum)
            {
                return Err("wrong residual pair sum".into());
            }
            if sum != BinaryPoint::Infinity {
                pair_keys.entry(point_key(&sum)).or_insert((i, j));
            }
        }
    }
    let m = t["m"]
        .as_u64()
        .filter(|m| (2..=15).contains(m))
        .ok_or("invalid transcript arity")? as usize;
    if public["m"].as_u64() != Some(m as u64) {
        return Err("transcript arity differs from public instance".into());
    }
    if t["attempts"].as_array().ok_or("missing attempts")?.len() > query_cap {
        return Err("query transcript exceeds declared cap".into());
    }
    let mut sources = BTreeMap::new();
    let mut completed = Vec::new();
    let mut ranks = Vec::new();
    let mut determined_scalar = None;
    for (trial, event) in t["attempts"]
        .as_array()
        .ok_or("missing query attempts")?
        .iter()
        .enumerate()
    {
        if residuals.is_empty() || determined_scalar.is_some() {
            return Err("query outside declared large-prime lifecycle".into());
        }
        if event["trial"].as_u64() != Some(trial as u64) {
            return Err("nonsequential attempt record".into());
        }
        let a = big(&event["coef_a"])?;
        let b = big(&event["coef_b"])?;
        if a >= kc.subgroup_order || b.is_zero() || b >= kc.subgroup_order {
            return Err("invalid query coefficients".into());
        }
        let query = point_add(
            &kc.curve,
            &scalar_mul(&kc.curve, kc.generator(), &a),
            &scalar_mul(&kc.curve, &q, &b),
        );
        if event["target"] != point(&query) {
            return Err("query target replay failed".into());
        }
        let indices = event["small_point_indices"]
            .as_array()
            .ok_or("missing query small indices")?;
        if indices.len() != m - 2 {
            return Err("incorrect small summand count".into());
        }
        let mut remainder = query;
        for i in indices {
            let i = i
                .as_u64()
                .and_then(|i| usize::try_from(i).ok())
                .ok_or("bad small index")?;
            remainder = point_add(
                &kc.curve,
                &remainder,
                &point_neg(fb.points.get(i).ok_or("small index out of range")?),
            );
        }
        if event["remainder"] != point(&remainder) {
            return Err("query remainder replay failed".into());
        }
        if event.get("partial").is_some() {
            if event["status"] != "partial_verified" {
                return Err("incorrect accepted-query label".into());
            }
            let p = read_partial(&event["partial"], kc.n)?;
            if p.source_id != trial as u64
                || p.coef_a != a
                || p.coef_b != b
                || json!(p.small_point_indices) != event["small_point_indices"]
            {
                return Err("partial differs from query".into());
            }
            let &(i, j) = pair_keys
                .get(&point_key(&remainder))
                .ok_or("partial has no pair-table hit")?;
            if p.residual_points != vec![residuals[i].clone(), residuals[j].clone()]
                || event["residual_pair_indices"] != json!([i, j])
            {
                return Err("partial differs from residual pair hit".into());
            }
            verify_partial(&kc, &fb, &q, &p)?;
            sources.insert(p.source_id, p.clone());
            let feed = eliminator.feed(p).map_err(str::to_string)?;
            if event["feed"] != feed_json(&feed) {
                return Err("graph feed outcome differs on replay".into());
            }
            if let FeedOutcome::Full(row) = feed {
                verify_sources(&kc, &fb, &row, &sources)?;
                completed.push(row);
                let (mut values, mut rhs) =
                    matrix(&completed, fb.signed_orbits.len(), &kc.subgroup_order)?;
                let reduced =
                    gaussian_eliminate_mod_n_particular(&mut values, &mut rhs, &kc.subgroup_order)
                        .ok_or("rank replay failed")?;
                let target_column = fb.signed_orbits.len();
                if reduced.determined.get(target_column) == Some(&true) {
                    determined_scalar = Some(reduced.values[target_column].clone());
                }
                ranks.push(json!({"completed_rows":completed.len(),"columns":fb.signed_orbits.len()+1,"rank":reduced.rank,"determined_columns":reduced.determined,"rref":values.iter().map(|row|row.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"reduced_rhs":rhs.iter().map(ToString::to_string).collect::<Vec<_>>()}));
            }
        } else {
            if event["status"] != "no_residual_pair"
                || event.get("feed").is_some()
                || event.get("residual_pair_indices").is_some()
            {
                return Err("incorrect rejected-query label".into());
            }
            if pair_keys.contains_key(&point_key(&remainder)) {
                return Err("rejected query actually has a residual pair".into());
            }
        }
    }
    if t["completed_rows"] != json!(completed.iter().map(full_json).collect::<Vec<_>>())
        || t["rank_history"] != json!(ranks)
    {
        return Err("completed rows or ranks differ on replay".into());
    }
    if t["recovered_scalar"] != json!(determined_scalar.as_ref().map(ToString::to_string))
        || (!residuals.is_empty()
            && determined_scalar.is_none()
            && t["attempts"].as_array().ok_or("missing attempts")?.len() != query_cap)
    {
        return Err("large-prime recovery or terminal completeness differs on replay".into());
    }
    if !t["recovered_scalar"].is_null() {
        let scalar = big(&t["recovered_scalar"])?;
        if scalar >= kc.subgroup_order || scalar_mul(&kc.curve, kc.generator(), &scalar) != q {
            return Err("recovered scalar replay failed".into());
        }
        let (mut values, mut rhs) = matrix(&completed, fb.signed_orbits.len(), &kc.subgroup_order)?;
        let reduction =
            gaussian_eliminate_mod_n_particular(&mut values, &mut rhs, &kc.subgroup_order)
                .ok_or("recovery rank missing")?;
        let target_column = fb.signed_orbits.len();
        if reduction.determined.get(target_column) != Some(&true)
            || reduction.values.get(target_column) != Some(&scalar)
        {
            return Err("scalar is not determined by retained rows".into());
        }
    }
    if !t["recovered_column_logs"].is_null() {
        let solution = eliminator
            .solve_completed_full_rank(&completed)
            .map_err(str::to_string)?
            .ok_or("column logs asserted without complete matrix rank")?;
        let expected = json!(solution
            .column_logs
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>());
        if t["recovered_column_logs"] != expected
            || t["recovered_scalar"] != json!(solution.target_log.to_string())
        {
            return Err("saved column logs differ from verified full-rank solution".into());
        }
        for (orbit, log) in solution.column_logs.iter().enumerate() {
            let representative = &fb.points[fb.signed_orbits[orbit][0]];
            if scalar_mul(&kc.curve, kc.generator(), log) != *representative {
                return Err("independent factor-base logarithm replay failed".into());
            }
        }
    }
    Ok(())
}

pub fn verify_membership_provenance(
    public: &Value,
    factor_base: &Value,
    saved: &Value,
) -> Result<(), String> {
    if saved["schema"] == "ml-ecdlp-generic-large-prime-membership/v1" {
        return generic::verify_membership(public, factor_base, saved);
    }
    if saved["schema"] != "ml-ecdlp-large-prime-membership-provenance/v1"
        || saved["generator"] != public["generator"]
        || saved["used_in_relation_elimination"] != false
    {
        return Err("invalid restricted membership provenance identity".into());
    }
    let point_cap = saved["point_cap"]
        .as_u64()
        .filter(|&n| (2..=127).contains(&n))
        .ok_or("invalid residual membership point cap")? as usize;
    let kc = model(public)?;
    curves::verify_factor_base(public, factor_base)?;
    let case = CaseData {
        public_instance: public.clone(),
        private_witness: Value::Null,
        factor_base: factor_base.clone(),
    };
    let fb = explicit_base(&kc, &case)?;
    let expected = residual_pool(public, &kc, &fb, point_cap, false)?;
    if saved["logs"] != json!(expected.restricted_membership_logs)
        || saved["projection_metrics"] != expected.projection_metrics
    {
        return Err(
            "restricted cyclic membership provenance differs from independent reconstruction"
                .into(),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn genuine_unknown_scalar_large_prime_cycle_replay() {
        let config = CaseConfig {
            factor_base_policy: None,
            family: "koblitz".into(),
            n: 13,
            m: 3,
            twist: false,
            curve_seed: 7,
            generator_index: 1,
            key_index: 2,
            seed: 42,
            factor_base_size: 2,
            degree_bound: 3,
            work_limit: 20_000,
            solver: "double-large-prime".into(),
        };
        let mut case = curves::generate(&config).unwrap();
        case.private_witness = Value::Null;
        let result = solve(&config, &case).unwrap();
        assert_eq!(
            result.outcome["scalar_recovered_without_witness"], true,
            "{}",
            result.outcome
        );
        assert!(
            result.outcome["counters"]["rows_completed_by_merging"]
                .as_u64()
                .unwrap()
                > 0
        );
        let transcript: Value = serde_json::from_slice(&result.artifacts[0].bytes).unwrap();
        verify_transcript(&case.public_instance, &case.factor_base, &transcript).unwrap();
        let mut truncated = transcript.clone();
        truncated["attempts"] = json!([]);
        truncated["completed_rows"] = json!([]);
        truncated["rank_history"] = json!([]);
        truncated["recovered_scalar"] = Value::Null;
        truncated["recovered_column_logs"] = Value::Null;
        assert!(
            verify_transcript(&case.public_instance, &case.factor_base, &truncated)
                .unwrap_err()
                .contains("terminal completeness")
        );
        let mut omitted = transcript.clone();
        omitted["recovered_scalar"] = Value::Null;
        omitted["recovered_column_logs"] = Value::Null;
        assert!(
            verify_transcript(&case.public_instance, &case.factor_base, &omitted)
                .unwrap_err()
                .contains("terminal completeness")
        );
        let membership: Value = serde_json::from_slice(&result.artifacts[3].bytes).unwrap();
        verify_membership_provenance(&case.public_instance, &case.factor_base, &membership)
            .unwrap();
        let mut bad_membership = membership.clone();
        bad_membership["logs"] = json!([{"log":"0"}]);
        assert!(verify_membership_provenance(
            &case.public_instance,
            &case.factor_base,
            &bad_membership
        )
        .is_err());
        let mut bad = transcript.clone();
        bad["recovered_scalar"] = json!("0");
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = transcript.clone();
        bad["completed_rows"][0]["coef_a"] = json!("0");
        if bad != transcript {
            assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        }
        let mut bad = transcript.clone();
        bad["residual_pool"]["construction"][0]["x"] = json!("0");
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = transcript.clone();
        let row = bad["residual_pool"]["construction"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|row| !row["candidates"].as_array().unwrap().is_empty())
            .unwrap();
        row["candidates"][0]["projection_multiplier"] = json!("0");
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = transcript.clone();
        bad["m"] = json!(15);
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = transcript.clone();
        bad["limits"]["query_cap"] = json!(1);
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad = transcript.clone();
        bad["recovered_column_logs"] = json!(["0"]);
        assert!(verify_transcript(&case.public_instance, &case.factor_base, &bad).is_err());
        let mut bad_base = case.factor_base.clone();
        bad_base["points"] = json!([]);
        assert!(verify_transcript(&case.public_instance, &bad_base, &transcript).is_err());
    }
}
