use super::{
    model::{emit, write_new, Request, Result},
    polynomials, shapes,
};
use crypto_lib::cryptanalysis::{
    ecbench::{
        canonical::{derive_u64, sha256_hex},
        fbm::FactorBaseMeasurementConfig,
        methods::{dump_factor_base, FactorBaseDump},
        workload::Instance,
    },
    f4_fp::{self, F4Options, Ordering, Verdict},
    ic_boundary::{BinaryGroup, CountedGroup, GroupOps, IncrementalGauss, PrimePoint},
    ic_framework::stages::Params,
    koblitz_fast::FastPoint,
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::mpsc,
    time::{Duration, Instant},
};

fn integer(s: &str) -> Result<u64> {
    u64::from_str_radix(s.strip_prefix("0x").unwrap_or(s), 16).map_err(|e| e.to_string())
}
pub(super) fn root_order(dump: &FactorBaseDump) -> Result<String> {
    let mut leaves: Vec<String> = dump
        .points
        .iter()
        .enumerate()
        .map(|(j, p)| {
            Ok(sha256_hex(
                &serde_json::to_vec(&json!(["factor_base_sweep/order/leaf/v1", j, p]))
                    .map_err(|e| e.to_string())?,
            ))
        })
        .collect::<Result<_>>()?;
    if leaves.is_empty() {
        return Err("empty factor base".into());
    }
    while leaves.len() > 1 {
        leaves = leaves
            .chunks(2)
            .map(|p| {
                sha256_hex(
                    format!(
                        "factor_base_sweep/order/node/v1:{}:{}",
                        p[0],
                        p.get(1).unwrap_or(&p[0])
                    )
                    .as_bytes(),
                )
            })
            .collect();
    }
    Ok(sha256_hex(
        format!(
            "factor_base_sweep/order/root/v1:{}:{}",
            dump.points.len(),
            leaves[0]
        )
        .as_bytes(),
    ))
}
fn enforce_inventory_budget(spec: &str, max: usize) -> Result<()> {
    let (name, p) = Params::parse_spec(spec)?;
    let count = if let Some(s) = p.get("dimension") {
        1usize
            .checked_shl(s.parse().map_err(|_| "bad dimension")?)
            .unwrap_or(usize::MAX)
    } else if let Some(s) = p.get("t") {
        1usize
            .checked_shl(s.parse().map_err(|_| "bad tower depth")?)
            .unwrap_or(usize::MAX)
    } else if let Some(s) = p.get("size") {
        s.parse().map_err(|_| "bad size")?
    } else {
        return Err(format!(
            "constructor {name} needs an explicit bounded size, dimension or t"
        ));
    };
    if count > max {
        return Err(format!(
            "requested inventory {count} exceeds max_abscissae={max}"
        ));
    }
    Ok(())
}

pub(super) fn measurement_identity(
    dump: &FactorBaseDump,
    spec: &str,
    max_degree: u32,
) -> Result<crypto_lib::cryptanalysis::ecbench::fbm::FactorBaseMeasurementId> {
    let (_, params) = Params::parse_spec(spec)?;
    let formulation = if dump.curve.family == "prime" {
        "semaev-s3-direct-membership"
    } else {
        "semaev-s3-weil-subspace"
    };
    FactorBaseMeasurementConfig {
        schema: crypto_lib::cryptanalysis::ecbench::fbm::FBM_SCHEMA.into(),
        curve: dump.curve.slug.clone(),
        fb_id: dump.factor_base.fb_id.clone(),
        fb_sha256: dump.factor_base.fb_sha256.clone(),
        family: dump.factor_base.family.clone(),
        fb_params: params.0,
        oracle: formulation.into(),
        solver: format!("f4-fp:degree={max_degree}"),
        linalg: "incremental-gauss".into(),
        instrument: "factor-base-sweep/v1".into(),
    }
    .seal()
}

pub fn run(request: &Request, dir: &Path) -> Result<Value> {
    request.plan.validate()?;
    let started = Instant::now();
    let c = &request.candidate;
    emit(
        "stage_begin",
        json!({"stage":"construction","label":c.label}),
    )?;
    let inst = c.curve.build()?;
    let mut basis = None;
    let (r, h) = match &inst {
        Instance::Prime(i) => (i.r, i.cofactor),
        Instance::Binary(i) => (i.r, i.cofactor),
        _ => {
            return Err(
                "inapplicable: this backend requires a native small prime/binary instance".into(),
            )
        }
    };
    if let Instance::Prime(i) = &inst {
        if !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(i.curve.p) {
            return Err("coefficient modulus is not prime".into());
        }
    }
    if !crypto_lib::cryptanalysis::residual_walk::is_prime_u64(r) || h.is_multiple_of(r) {
        return Err(
            "invalid prime-subgroup/cofactor assumptions for projected relation rank".into(),
        );
    }
    let spec = if let Some(shape) = &c.shape {
        let Instance::Binary(i) = &inst else {
            return Err("binary shape on a non-binary curve".into());
        };
        let b = shapes::basis(i, shape, c.dimension, c.seed)?;
        let text = format!(
            "binary-subspace:dimension={},basis={}",
            b.len(),
            b.iter()
                .map(|v| format!("0x{v:x}"))
                .collect::<Vec<_>>()
                .join(";")
        );
        basis = Some(b);
        text
    } else {
        c.factor_base.clone().ok_or("missing constructor")?
    };
    write_new(
        &dir.join("construction.json"),
        &json!({"candidate":c,"constructor":spec,"basis_words":basis,"inventory_materialized":false}),
    )?;
    enforce_inventory_budget(&spec, request.plan.max_abscissae)?;
    let dump = dump_factor_base(&c.curve, &spec)?;
    write_new(&dir.join("base.json"), &dump)?;
    if basis.is_none() && dump.factor_base.family == "binary-subspace" {
        basis = Some(shapes::recipe_basis(
            &dump.factor_base.params,
            dump.factor_base
                .dimension
                .ok_or("missing subspace dimension")?,
        )?);
    }
    if dump.factor_base.abscissae as usize > request.plan.max_abscissae || dump.points.is_empty() {
        return Err("empty or over-budget materialized inventory".into());
    }
    let formulation = if matches!(inst, Instance::Prime(_)) {
        "semaev-s3-direct-membership"
    } else {
        "semaev-s3-weil-subspace"
    };
    let fbm = measurement_identity(&dump, &spec, request.plan.max_degree)?;
    let mut lookup = BTreeMap::new();
    for p in &dump.points {
        if lookup
            .insert(
                format!("{}:{}", p.x, p.y),
                json!({"column":p.col,"coefficient":p.coef}),
            )
            .is_some()
        {
            return Err("duplicate inventory point".into());
        }
    }
    write_new(&dir.join("lookup.json"), &lookup)?;
    write_new(
        &dir.join("derivation.json"),
        &json!({"candidate":c,"constructor":spec,"basis_words":basis,"order_merkle_root":root_order(&dump)?,"measurement_identity":fbm,"formulation":formulation,"certified_degree_of_regularity":null,"full_pipeline_speedup":null}),
    )?;
    emit(
        "inventory",
        json!({"fb_id":dump.factor_base.fb_id,"columns":dump.factor_base.columns,"signed_points":dump.points.len(),"order_root":root_order(&dump)?}),
    )?;
    match &inst {
        Instance::Prime(i)=>{
            if i.curve.p>1_000_000{return Err("direct prime root extraction requires p <=1000000; no implicit scaled substitute".into());}
            let points=dump.points.iter().map(|p|Ok(PrimePoint::affine(integer(&p.x)?,integer(&p.y)?))).collect::<Result<Vec<_>>>()?;
            if points.iter().any(|p|!i.curve.is_on_curve(*p)){return Err("off-curve inventory".into());}
            let xs:Vec<u64>=points.iter().map(|p|p.x).collect::<BTreeSet<_>>().into_iter().collect();
            execute(request,dir,started,&dump,&i.curve,i.generator_point(),i.r,i.cofactor,&points,|p|[p.x,p.y],|x|polynomials::prime(&i.curve,x,&xs),2,i.curve.p,|a|[a[0],a[1]])
        }
        Instance::Binary(i)=>{
            let basis=if let Some(b)=basis {b}else if dump.factor_base.family=="binary-subspace" {let dim=dump.factor_base.dimension.ok_or("missing subspace dimension")?; (0..dim).map(|j|1<<j).collect()}else{return Err("algebraic backend requires an explicit linear subspace; nonlinear inventory retained but not priced as its span".into());};
            let v=shapes::subspace(i,&basis);let points=dump.points.iter().map(|p|Ok(FastPoint::affine(integer(&p.x)?,integer(&p.y)?))).collect::<Result<Vec<_>>>()?;
            if points.iter().any(|p|!i.fast.is_on_curve(*p)){return Err("off-curve inventory".into());}
            let group=BinaryGroup(&i.fast);let vars=2*basis.len();
            execute(request,dir,started,&dump,&group,i.generator,i.r,i.cofactor,&points,|p|[p.x,p.y],|x|polynomials::binary(i,&v,x),vars,2,|a|{let mut x=[0,0];for k in 0..2{for(j,b)in basis.iter().enumerate(){if a[k*basis.len()+j]==1{x[k]^=b;}}}x})
        }
        _=>Err("this algebraic sweep backend supports native small prime/binary instances only; no implicit substitute".into()),
    }
}

fn subgroup_positive_control<G: CountedGroup>(
    group: &G,
    points: &[G::Elt],
    r: u64,
    xy: &impl Fn(G::Elt) -> [u64; 2],
    setup: &mut GroupOps,
) -> Result<G::Elt> {
    let mut checked = BTreeSet::new();
    for &a in points {
        for &b in points {
            let q = group.add(setup, a, b);
            if !group.is_identity(&q)
                && checked.insert(xy(q))
                && group.is_identity(&group.mul(setup, q, r))
            {
                return Ok(q);
            }
        }
    }
    Err("no finite two-point positive control in target prime subgroup".into())
}

#[allow(clippy::too_many_arguments)]
fn execute<G: CountedGroup>(
    req: &Request,
    dir: &Path,
    started: Instant,
    dump: &FactorBaseDump,
    group: &G,
    generator: G::Elt,
    r: u64,
    h: u64,
    points: &[G::Elt],
    xy: impl Fn(G::Elt) -> [u64; 2],
    encode: impl Fn(u64) -> Vec<f4_fp::Poly>,
    nvars: usize,
    p: u64,
    decode: impl Fn(&[u64]) -> [u64; 2],
) -> Result<Value> {
    let mut setup = GroupOps::default();
    let positive = subgroup_positive_control(group, points, r, &xy, &mut setup)?;
    let mut rows = IncrementalGauss::new(dump.factor_base.columns as usize, r);
    let mut records = Vec::new();
    let mut total_ops = 0;
    let mut stats = BTreeMap::new();
    let mut best_degree = None;
    for phase in ["positive_control", "train", "holdout"] {
        emit("stage_begin", json!({"stage":phase}))?;
        let count = match phase {
            "train" => req.plan.train,
            "holdout" => req.plan.holdout,
            _ => 1,
        };
        let phase_start = Instant::now();
        let mut hits = 0u32;
        let mut exhausted = 0u32;
        let mut refutations = 0u32;
        let mut rolling = std::collections::VecDeque::new();
        let mut rolling_start = phase_start;
        for j in 0..count {
            let scalar = 1 + derive_u64(
                &format!("factor-base-sweep/{phase}/v1"),
                &[req.plan.seed, j.into()],
            ) % (r - 1);
            let mut query_ops = GroupOps::default();
            let target = if phase == "positive_control" {
                positive
            } else {
                group.mul(&mut query_ops, generator, scalar)
            };
            let target_xy = xy(target);
            let mut verification_ops = GroupOps::default();
            let truth = points.iter().enumerate().find_map(|(a, &pa)| {
                points.iter().enumerate().find_map(|(b, &pb)| {
                    (group.add(&mut verification_ops, pa, pb) == target).then_some([a, b])
                })
            });
            let equations = encode(target_xy[0]);
            let system = json!({"schema":"factor_base_sweep.system/v1","phase":phase,"trial":j,"target":target_xy,"coefficient_modulus":p,"variables":nvars,"ordering":"grevlex","equations":equations,"constructor":req.candidate,"max_degree":req.plan.max_degree});
            let stem = format!("{phase}-{j:04}");
            write_new(&dir.join(format!("{stem}.system.json")), &system)?;
            let system_hash = sha256_hex(&serde_json::to_vec(&system).map_err(|e| e.to_string())?);
            emit(
                "solve_begin",
                json!({"phase":phase,"trial":j,"system_sha256":system_hash,"variables":nvars,"target":target_xy}),
            )?;
            let left =
                Duration::from_secs(req.plan.candidate_seconds).saturating_sub(started.elapsed());
            if left.is_zero() {
                return Err("candidate deadline before next solve".into());
            }
            let opts = F4Options::new(Ordering::Grevlex, req.plan.max_degree).with_budget(left);
            let (tx, rx) = mpsc::channel();
            let relay = std::thread::spawn(move || {
                let mut productive_degree = 0;
                for observation in rx {
                    let observation: f4_fp::F4Progress = observation;
                    if observation.event == "complete" {
                        productive_degree = productive_degree.max(observation.productive_degree);
                    }
                    let _ = emit("solver", json!(observation));
                }
                productive_degree
            });
            let solved = f4_fp::observe_progress(tx, || f4_fp::solve(&equations, nvars, p, &opts));
            let productive_degree = relay.join().map_err(|_| "solver event relay panicked")?;
            let input_degree = equations
                .iter()
                .flat_map(|f| f.iter().map(|(m, _)| m.iter().sum::<u32>()))
                .max()
                .unwrap_or(0);
            let root_degree = (!matches!(solved.verdict, Verdict::Undetermined))
                .then_some(productive_degree.max(input_degree));
            total_ops += solved.field_ops;
            let mut witness = None;
            let mut algebraic_roots = 0;
            let status = match &solved.verdict {
                Verdict::Undetermined => {
                    exhausted += 1;
                    "exhausted"
                }
                Verdict::Inconsistent => {
                    refutations += 1;
                    if truth.is_some() {
                        return Err(
                            "solver refutation contradicts independent point witness".into()
                        );
                    }
                    "refuted"
                }
                Verdict::Solutions(roots) => {
                    algebraic_roots = roots.len();
                    for root in roots {
                        if equations
                            .iter()
                            .any(|f| polynomials::evaluate(f, root, p) != 0)
                        {
                            return Err("solver root violates original system".into());
                        }
                        let xs = decode(root);
                        for (a, &pa) in points
                            .iter()
                            .enumerate()
                            .filter(|(_, pa)| xy(**pa)[0] == xs[0])
                        {
                            for (b, &pb) in points
                                .iter()
                                .enumerate()
                                .filter(|(_, pb)| xy(**pb)[0] == xs[1])
                            {
                                if group.add(&mut verification_ops, pa, pb) == target {
                                    witness = Some([a, b]);
                                    break;
                                }
                            }
                            if witness.is_some() {
                                break;
                            }
                        }
                        if witness.is_some() {
                            break;
                        }
                    }
                    if witness.is_some() {
                        "verified_decomposition"
                    } else {
                        if truth.is_some() {
                            return Err("completed solver omitted an independently known point decomposition".into());
                        }
                        "no_point_decomposition"
                    }
                }
            };
            let mut relation = None;
            if let Some(indices) = witness {
                hits += 1;
                if phase == "train" {
                    if let Some(d) = root_degree {
                        best_degree = Some(best_degree.map_or(d, |old: u32| old.min(d)));
                    }
                }
                if phase == "train" {
                    let mut row = vec![0u64; dump.factor_base.columns as usize];
                    for k in indices {
                        let point = &dump.points[k];
                        row[point.col as usize] = (row[point.col as usize] + point.coef) % r;
                    }
                    let rhs = (scalar as u128 * h as u128 % r as u128) as u64;
                    let rank_before = rows.rank();
                    rows.add_row(row.clone(), rhs);
                    if rows.inconsistent != 0 {
                        return Err(
                            "verified point relations produced an inconsistent projected matrix"
                                .into(),
                        );
                    }
                    relation = Some(
                        json!({"row":row,"rhs":rhs,"cofactor_projection":h,"rank_gain":rows.rank()-rank_before}),
                    );
                }
            }
            let phase_elapsed = phase_start.elapsed().as_secs_f64();
            let rate = hits as f64 / phase_elapsed.max(1e-9);
            rolling.push_back((Instant::now(), witness.is_some()));
            if rolling.len() > 8 {
                rolling_start = rolling.pop_front().expect("window has nine elements").0;
            }
            let rolling_hits = rolling.iter().filter(|(_, hit)| *hit).count();
            let rolling_rate =
                rolling_hits as f64 / rolling_start.elapsed().as_secs_f64().max(1e-9);
            let mut record = json!({"phase":phase,"trial":j,"status":status,"target":target_xy,"public_control_scalar":(phase!="positive_control").then_some(scalar),"ground_truth_decomposable":truth.is_some(),"witness_inventory_indices":witness,"witness_verified":witness.is_some(),"relation":relation,"algebraic_roots":algebraic_roots,"observed_solving_degree":root_degree,"solving_degree_definition":"maximum input degree and productive F4 step degree over the substitution tree; this solver procedure only","degree_processed":solved.degree_reached,"certified_degree_of_regularity":null,"field_reduction_multiplications":solved.field_ops,"max_rows":solved.max_rows,"max_columns":solved.max_cols,"timed_out":solved.timed_out,"system_sha256":system_hash,"query_ops":query_ops,"verification_ops":verification_ops});
            record["phase_elapsed_seconds_operational"] = json!(phase_elapsed);
            record["cumulative_verified"] = json!(hits);
            record["observed_verified_per_second"] = json!(rate);
            record["observed_negative_solving_degree"] =
                json!((status == "refuted").then_some(root_degree).flatten());
            record["certified_refutation_degree"] = Value::Null;
            record["negative_verdict_method"]=json!("degree-bounded F4 plus finite-field substitution/enumeration; the observed procedure degree is not an algebraic refutation certificate");
            record["summands"] = json!(2);
            write_new(&dir.join(format!("{stem}.result.json")), &record)?;
            records.push(record);
            emit(
                "trial",
                json!({"phase":phase,"trial":j+1,"total":count,"status":status,"verified":hits,"verified_yield":hits as f64/(j+1)as f64,"refuted":refutations,"exhausted":exhausted,"observed_solving_degree":root_degree,"verified_decompositions_per_second":rate,"rolling_verified_per_second":rolling_rate,"rolling_trials":rolling.len(),"rate_status":"provisional operational observation","rate_scope":"whole probe phase including independent finite-inventory census, system construction, solver, witness checks and incremental logging","relation_rows":if phase=="train"{hits}else{0},"independent_rank":rows.rank(),"independent_relations_per_second":if phase=="train"{Some(rows.rank()as f64/phase_start.elapsed().as_secs_f64().max(1e-9))}else{None},"columns":rows.columns(),"eta_seconds_provisional":((j+1)<count).then(||phase_start.elapsed().as_secs_f64()*(count-j-1)as f64/(j+1)as f64),"remaining_independent_relations":rows.columns().saturating_sub(rows.rank()),"rank_completion_eta_seconds_provisional":if phase=="train"&&rows.rank()>0{Some(phase_start.elapsed().as_secs_f64()*rows.columns().saturating_sub(rows.rank())as f64/rows.rank()as f64)}else{None},"eta_model":"constant observed throughput/independence yield; may stall; not end-to-end ETA","wall_time_status":"operational telemetry; not isolated speed evidence"}),
            )?;
            if phase == "positive_control" && witness.is_none() {
                return Err("positive control did not finish with a verified point decomposition; candidate remains unqualified".into());
            }
        }
        stats.insert(phase,json!({"trials":count,"verified":hits,"exhausted":exhausted,"refuted":refutations,"observed_verified_per_second":hits as f64/phase_start.elapsed().as_secs_f64().max(1e-9)}));
    }
    let result = json!({"schema":"factor_base_sweep.candidate_result/v1","candidate":req.candidate,"fb_id":dump.factor_base.fb_id,"phase_summary":stats,"positive_control_scope":"target-prime-subgroup","positive_control_subgroup_order":r,"best_observed_successful_solving_degree":best_degree,"certified_degree_of_regularity":null,"field_reduction_multiplications":total_ops,"training_independent_rank":rows.rank(),"relation_row_operations":rows.row_ops,"columns":rows.columns(),"trials":records,"setup_point_ops":setup,"unpriced_native_operations_complete":false,"missing_native_pricing":["system construction","root extraction","inventory construction and lookups","verification overhead"],"total_cost_winner":null,"full_pipeline_speedup":null,"elapsed_seconds_operational":started.elapsed().as_secs_f64()});
    write_new(&dir.join("result.json"), &result)?;
    emit(
        "candidate_complete",
        json!({"best_observed_successful_solving_degree":best_degree,"training_rank":rows.rank()}),
    )?;
    Ok(result)
}

#[cfg(test)]
mod identity_tests {
    use super::*;
    #[test]
    fn a_proper_subfield_control_cannot_qualify_an_unreachable_prime_subgroup() {
        let i = crypto_lib::cryptanalysis::ic_boundary::koblitz_instance(1, 9).unwrap();
        let basis = shapes::basis(&i, "subfield", 3, 17).unwrap();
        let mut points = Vec::new();
        for mask in 1..8 {
            let x = basis
                .iter()
                .enumerate()
                .filter(|(j, _)| mask & (1 << j) != 0)
                .fold(0, |a, (_, b)| a ^ b);
            points.extend(i.points_with_x(x));
        }
        assert!(!points.is_empty());
        let err = subgroup_positive_control(
            &BinaryGroup(&i.fast),
            &points,
            i.r,
            &|p| [p.x, p.y],
            &mut GroupOps::default(),
        )
        .unwrap_err();
        assert!(err.contains("target prime subgroup"));
    }
    #[test]
    fn accepted_control_is_finite_and_annihilated_by_the_recorded_prime_order() {
        let i = crypto_lib::cryptanalysis::ic_boundary::koblitz_instance(1, 7).unwrap();
        let points: Vec<_> = (1..8).flat_map(|x| i.points_with_x(x)).collect();
        let group = BinaryGroup(&i.fast);
        let mut ops = GroupOps::default();
        let q = subgroup_positive_control(&group, &points, i.r, &|p| [p.x, p.y], &mut ops).unwrap();
        assert!(!group.is_identity(&q));
        assert!(group.is_identity(&group.mul(&mut ops, q, i.r)));
        assert!(ops.scalar_mults > 0);
    }
    #[test]
    fn a_prime_orbit_constructor_keeps_the_prime_formulation_identity() {
        let spec = "glv-orbit:size=4";
        let dump = dump_factor_base(
            &crypto_lib::cryptanalysis::ecbench::workload::CurveSpec::PrimeRoster { bits: 16 },
            spec,
        )
        .unwrap();
        assert_eq!(dump.curve.family, "prime");
        assert_eq!(
            measurement_identity(&dump, spec, 8).unwrap().config.oracle,
            "semaev-s3-direct-membership"
        );
    }
}
