//! Frozen retained-chain probes and complete public toy relation pipelines.
//! This driver reports raw counters, not an admitted performance comparison.
use crypto_lib::cryptanalysis::{
    challenge_ladder::is_prime_u64,
    ic_boundary::{
        Calibration, CountedGroup, GroupOps, OracleCounters, PrimeCurve, PrimeInstance, PrimePoint,
    },
    ic_framework::{
        self,
        pkm_semaev::{decomposable_points, PkmSemaevOracle},
        prime_tower::PrimeKummerBase,
        stages::{DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params, Targets},
        useful_rank, PipelineSpec,
    },
};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Write,
    path::Path,
    time::{Duration, Instant},
};

fn fixture(p: u64, a: u64, b: u64, r: u64) -> Result<PrimeInstance, String> {
    let curve = PrimeCurve { p, a, b };
    if !is_prime_u64(p) || !is_prime_u64(r) {
        return Err("fixture primality".into());
    }
    let mut order = 1;
    for x in 0..p {
        let rhs = curve.rhs(x);
        order += if rhs == 0 {
            1
        } else if curve.legendre(rhs) == 1 {
            2
        } else {
            0
        };
    }
    if order != r {
        return Err(format!("fixture order {order} != {r}"));
    }
    let generator = (0..p)
        .find_map(|x| curve.sqrt(curve.rhs(x)).filter(|&y| y != 0).map(|y| (x, y)))
        .ok_or("fixture has no point")?;
    let instance = PrimeInstance {
        name: format!("retained-fp{p}-a{a}-b{b}"),
        curve,
        group_order: r,
        r,
        cofactor: 1,
        generator,
    };
    if !instance
        .curve
        .mul(&mut GroupOps::default(), instance.generator_point(), r)
        .infinity
    {
        return Err("fixture point order".into());
    }
    Ok(instance)
}

fn context(inst: &PrimeInstance, target: PrimePoint) -> InstanceCtx<'_, PrimeCurve> {
    InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target,
        r: inst.r,
        cofactor: 1,
        group_order: inst.r,
        name: inst.name.clone(),
        field_degree: None,
    }
}

fn variants(m: u32) -> Vec<(&'static str, &'static str)> {
    match m {
        2 => vec![
            ("full-blocks", "mode=full,layout=blocks"),
            ("full-levels", "mode=full,layout=levels"),
            ("full-blocks-aa", "mode=full,layout=blocks"),
        ],
        3 => vec![
            ("full-blocks", "mode=full,layout=blocks"),
            ("chain-blocks", "charts=all,layout=blocks"),
            ("chain-levels", "charts=all,layout=levels"),
            ("full-blocks-aa", "mode=full,layout=blocks"),
        ],
        _ => vec![
            ("left-blocks", "charts=all,layout=blocks,tree=left"),
            ("left-levels", "charts=all,layout=levels,tree=left"),
            ("balanced-blocks", "charts=all,layout=blocks,tree=balanced"),
            ("balanced-levels", "charts=all,layout=levels,tree=balanced"),
            ("left-blocks-aa", "charts=all,layout=blocks,tree=left"),
        ],
    }
}

fn emit(file: &mut fs::File, rows: &mut Vec<Value>, row: Value) -> Result<(), String> {
    serde_json::to_writer(&mut *file, &row).map_err(|e| e.to_string())?;
    writeln!(file).map_err(|e| e.to_string())?;
    file.flush().map_err(|e| e.to_string())?;
    rows.push(row);
    Ok(())
}

fn probes(out: &Path, file: &mut fs::File, rows: &mut Vec<Value>) -> Result<(), String> {
    for (p, b, r) in [(769, 5, 751), (12289, 28, 12373)] {
        let inst = fixture(p, 2, b, r)?;
        for t in 1..=3 {
            let (_, params) = Params::parse_spec(&format!("prime-kummer:t={t}"))?;
            let builder = PrimeKummerBase { instance: &inst };
            let ctx = context(&inst, inst.generator_point());
            let setup = Instant::now();
            let base_result = builder.build(&ctx, &params, &mut GroupOps::default());
            let base_setup_ns = setup.elapsed().as_nanos();
            for m in 2..=4 {
                let mut reference_ops = GroupOps::default();
                let expected = base_result
                    .as_ref()
                    .ok()
                    .map(|base| decomposable_points(&inst.curve, base, m, &mut reference_ops));
                for seed in [20261109, 20261110] {
                    let scalar = StdRng::seed_from_u64(seed).gen_range(1..r);
                    let target =
                        inst.curve
                            .mul(&mut GroupOps::default(), inst.generator_point(), scalar);
                    for (name, options) in variants(m) {
                        let id = format!("p{p}-t{t}-m{m}-s{seed}-{name}");
                        let base = match &base_result {
                            Ok(base) if base.columns > 0 => base,
                            other => {
                                emit(
                                    file,
                                    rows,
                                    json!({"case":id,"p":p,"t":t,"m":m,
                                    "variant":name,"status":"unavailable_base",
                                    "error":other.as_ref().err(),"base_setup_ns":base_setup_ns}),
                                )?;
                                continue;
                            }
                        };
                        let (_, op) = Params::parse_spec(&format!(
                            "pkm-semaev:m={m},degree=5,max_nnz=1000000,{options}"
                        ))?;
                        let mut oracle = PkmSemaevOracle::new(m, t, 1)
                            .with_budget(Some(Duration::from_millis(250)));
                        let setup = Instant::now();
                        oracle.prepare(&ctx, base, &op, &mut GroupOps::default())?;
                        let oracle_setup_ns = setup.elapsed().as_nanos();
                        let system = oracle
                            .prepared_system_artifact()
                            .ok_or("missing prepared system")?;
                        let system_bytes =
                            serde_json::to_vec_pretty(&system).map_err(|e| e.to_string())?;
                        let system_name = format!("{id}.system.json");
                        fs::write(out.join(&system_name), &system_bytes)
                            .map_err(|e| e.to_string())?;
                        let (mut ops, mut counters) =
                            (GroupOps::default(), OracleCounters::default());
                        let started = Instant::now();
                        let witness = oracle.decompose(&ctx, base, &mut ops, &mut counters, target);
                        let diagnostic_ns = started.elapsed().as_nanos();
                        let mut check_ops = GroupOps::default();
                        let witness_ok = witness.as_ref().is_none_or(|indices| {
                            let sum = indices.iter().fold(inst.curve.identity(), |acc, &i| {
                                inst.curve.add(&mut check_ops, acc, base.points[i])
                            });
                            sum == target
                        });
                        let truth = expected
                            .as_ref()
                            .expect("base reference")
                            .contains(&inst.curve.key(&target));
                        let correct = witness_ok && witness.is_some() == truth;
                        emit(
                            file,
                            rows,
                            json!({"schema":"retained-chain-probe/v1","case":id,
                            "p":p,"a":2,"b":b,"r":r,"t":t,"m":m,"seed":seed,
                            "variant":name,"target":[target.x,target.y],"base_columns":base.columns,
                            "base_setup_ns":base_setup_ns,"oracle_setup_ns":oracle_setup_ns,
                            "system_path":system_name,"system_sha256":hex::encode(Sha256::digest(&system_bytes)),
                            "solver":oracle.solver_totals(),"native":oracle.relation_counters(),
                            "shape":oracle.last_system(),"adds":ops.adds,"doubles":ops.doubles,
                            "diagnostic_ns":diagnostic_ns,"timing_admitted":false,
                            "reference_setup_adds":reference_ops.adds,"reference_setup_doubles":reference_ops.doubles,
                            "witness_check_adds":check_ops.adds,"witness":witness,
                            "expected_decomposable":truth,"producer_self_check":correct,
                            "status":if correct{"checked_by_producer"}else{"mismatch"}}),
                        )?;
                        if !correct {
                            return Err(format!("probe mismatch at {id}"));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn pipelines(file: &mut fs::File, rows: &mut Vec<Value>) -> Result<(), String> {
    let inst = PrimeInstance {
        name: "retained-fp2393".into(),
        curve: PrimeCurve {
            p: 2393,
            a: 2390,
            b: 1614,
        },
        group_order: 2377,
        r: 2377,
        cofactor: 1,
        generator: (0, 80),
    };
    let planted = StdRng::seed_from_u64(20261109).gen_range(1..inst.r);
    let target = inst
        .curve
        .mul(&mut GroupOps::default(), inst.generator_point(), planted);
    let ctx = context(&inst, target);
    let builder = PrimeKummerBase { instance: &inst };
    for m in [3, 4] {
        let mut arms = variants(m);
        let grid = if m == 3 {
            "mode=full,engine=exhaustive"
        } else {
            "charts=all,engine=exhaustive"
        };
        arms.push(("grid", grid));
        for (name, options) in arms {
            let (_, factor_base_params) = Params::parse_spec("prime-kummer:t=2")?;
            let (_, oracle_params) = Params::parse_spec(&format!(
                "pkm-semaev:m={m},degree=5,max_nnz=1000000,audit_rank=1,{options}"
            ))?;
            let spec = PipelineSpec {
                factor_base: "prime-kummer".into(),
                factor_base_params,
                oracle: "pkm-semaev".into(),
                oracle_params,
                solver: None,
                solver_params: Params::default(),
                targets: Targets::Walk,
                linalg: "incremental-gauss-full-rank-checked".into(),
                max_trials: 1000,
                seed: 5,
                solver_budget_seconds: 1,
            };
            let mut oracle = PkmSemaevOracle::from_spec(m, &spec, &inst)?;
            let (report, base) = ic_framework::run_pipeline_with_base(
                &ctx,
                &spec,
                &builder,
                &mut oracle,
                planted,
                &Calibration::default(),
                None,
            )?;
            let audit = report.useful_rank.as_ref().ok_or("rank audit missing")?;
            let rank_replay = useful_rank::replay(&ctx, &base, audit);
            let scalar_ok = report.recovered.is_some_and(|k| {
                k == planted
                    && inst
                        .curve
                        .mul(&mut GroupOps::default(), inst.generator_point(), k)
                        == target
            });
            let ok = rank_replay.is_ok() && scalar_ok && report.verified;
            let invalid = rank_replay.is_err()
                || (report.recovered.is_some() && !scalar_ok)
                || (scalar_ok && !report.verified);
            emit(
                file,
                rows,
                json!({"schema":"retained-chain-pipeline/v1","case":format!("m{m}-{name}"),
                "p":2393,"m":m,"variant":name,"target":[target.x,target.y],
                "producer_rank_replay":rank_replay.is_ok(),"producer_scalar_check":scalar_ok,
                "status":if ok{"checked_by_producer"}else if report.exhausted{"exhausted"}else{"mismatch"},
                "cost_qualification":"raw units; no common field/solver price; native framework GAE/S fields are not admitted totals",
                "timing_admitted":false,"report":report}),
            )?;
            if invalid {
                return Err("pipeline validation mismatch".into());
            }
        }
    }
    Ok(())
}

fn html(rows: &[Value]) -> String {
    let mut text=String::from("<!doctype html><meta charset=utf-8><title>Retained-chain study</title><style>body{font:15px system-ui;margin:3em}table{border-collapse:collapse}td,th{border:1px solid #ddd;padding:.5em}</style><h1>Retained-chain study</h1><p>Producer observations. Independent PCC standing and timing admission are separate. Missing cloud measurements remain pending.</p><table><tr><th>Case</th><th>Status</th><th>F4 multiply-adds</th><th>Rank</th><th>Rows</th><th>LA work</th></tr>");
    for row in rows {
        text.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
            row["case"].as_str().unwrap_or(""),
            row["status"].as_str().unwrap_or(""),
            row.pointer("/solver/ops")
                .or_else(|| row.pointer("/report/decomposition/solver/ops"))
                .unwrap_or(&Value::Null),
            row.pointer("/report/linear_algebra/rank")
                .unwrap_or(&Value::Null),
            row.pointer("/report/linear_algebra/rows")
                .unwrap_or(&Value::Null),
            row.pointer("/report/linear_algebra/work")
                .unwrap_or(&Value::Null)
        ));
    }
    text.push_str("</table>");
    text
}

pub fn run(phase: &str, out: &Path) -> Result<(), String> {
    if !matches!(phase, "probe" | "pipeline") {
        return Err("phase must be probe or pipeline".into());
    }
    fs::create_dir(out).map_err(|e| format!("new output directory {}: {e}", out.display()))?;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .map_err(|e| e.to_string())?;
    pool.install(|| {
        let mut file=fs::File::create(out.join("records.jsonl")).map_err(|e|e.to_string())?;
        let mut rows=Vec::new();
        let result=if phase=="probe" {probes(out,&mut file,&mut rows)}else{pipelines(&mut file,&mut rows)};
        let planned=if phase=="probe" {144}else{11};
        let flow=json!({"schema":"retained-chain-flow/v1","phase":phase,"planned":planned,
            "attempted":rows.len(),"completed":rows.iter().filter(|r|r["status"]=="checked_by_producer").count(),
            "unavailable":rows.iter().filter(|r|r["status"]=="unavailable_base").count(),
            "error":result.as_ref().err(),"timing_admitted":false,
            "binary_sha256":std::env::current_exe().ok().and_then(|p|fs::read(p).ok()).map(|b|hex::encode(Sha256::digest(b)))});
        fs::write(out.join("flow.json"),serde_json::to_vec_pretty(&flow).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        fs::write(out.join("report.html"),html(&rows)).map_err(|e|e.to_string())?;
        let mut paths=fs::read_dir(out).map_err(|e|e.to_string())?.map(|e|e.map(|e|e.path())).collect::<Result<Vec<_>,_>>().map_err(|e|e.to_string())?;
        paths.sort();
        let mut manifest=String::new();
        for path in paths {if path.is_file() {
            let bytes=fs::read(&path).map_err(|e|e.to_string())?;
            manifest.push_str(&format!("{}  {}\n",hex::encode(Sha256::digest(bytes)),path.file_name().unwrap().to_string_lossy()));
        }}
        fs::write(out.join("SHA256SUMS"),manifest).map_err(|e|e.to_string())?;
        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn retained_study_plan_has_exact_registered_counts() {
        assert_eq!(
            (2..=4).map(|m| variants(m).len()).sum::<usize>() * 2 * 3 * 2,
            144
        );
        assert_eq!(variants(3).len() + variants(4).len() + 2, 11);
        assert!(run("unknown", Path::new("unused-retained-study")).is_err());
        for source in [
            include_str!("../../../research/retained_chain_ic_20261109/pipeline-m3.json"),
            include_str!("../../../research/retained_chain_ic_20261109/pipeline-m4.json"),
        ] {
            let spec = crypto_lib::cryptanalysis::ecbench::spec::Spec::from_json(source).unwrap();
            crypto_lib::cryptanalysis::ecbench::spec::plan(spec).unwrap();
        }
    }
}
