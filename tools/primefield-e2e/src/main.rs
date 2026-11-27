//! Analyze existing public records; this executable does not run a DLP solver.
#[path = "../../../src/cryptanalysis/ecbench/complexity_accounting.rs"]
mod complexity_accounting;
#[path = "../../../src/cryptanalysis/ecbench/complexity_model.rs"]
mod complexity_model;

use complexity_accounting::{account_jsonl, descriptive_fit, GroupAccounting};
use complexity_model::{
    aggregate_phase_budgets, compare_slopes, finite_relation_budget, optimize_exponent, AffineCost,
    ExponentModel, FiniteRelationInput, MemoryConstraint, PhaseBudget, PhaseWork,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::Path,
    process::Command,
};

const SOURCE_COMMIT: &str = "aab505dd2d24cd5c98f9be1a2bd2243793a2001b";
const SOURCE_DIR: &str = "research/pkm_adaptive_sweep_20261123";
const FILES: &[&str] = &[
    "degree32.jsonl",
    "degree32_replication.jsonl",
    "degree32_replication_extend.jsonl",
    "e2e_full_rank_13bit_20261185.jsonl",
    "e2e_full_rank_13bit_20261221.jsonl",
    "e2e_full_rank_13bit_20261276.jsonl",
    "e2e_full_rank_15bit_20261300.jsonl",
    "e2e_full_rank_15bit_20261301.jsonl",
    "e2e_full_rank_15bit_20261302.jsonl",
    "e2e_full_rank_16bit_20261400_engines.jsonl",
    "e2e_full_rank_16bit_20261400_f4.jsonl",
    "e2e_full_rank_16bit_20261401_engines.jsonl",
    "e2e_full_rank_16bit_20261401_f4.jsonl",
    "e2e_full_rank_16bit_20261402_engines.jsonl",
    "e2e_full_rank_16bit_20261402_f4.jsonl",
];

#[derive(Serialize, Deserialize)]
struct Source {
    path: String,
    sha256: String,
    bytes: usize,
}
#[derive(Serialize, Deserialize)]
struct Manifest {
    schema: String,
    repository: String,
    commit: String,
    sources: Vec<Source>,
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn source_bytes(repo: &str, commit: &str, path: &str) -> Result<Vec<u8>, String> {
    if commit.len() != 40
        || !commit.bytes().all(|c| c.is_ascii_hexdigit())
        || path.starts_with('/')
        || path.split('/').any(|s| s == "..")
        || path.contains(':')
    {
        return Err(
            "source must use a full hexadecimal commit and repository-relative path".into(),
        );
    }
    let output = Command::new("git")
        .args(["-C", repo, "show", &format!("{commit}:{path}")])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "git show failed for {path}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let mut bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))
}

fn pin(repo: &str, output: &str) -> Result<(), String> {
    let mut sources = Vec::new();
    for name in FILES {
        let path = format!("{SOURCE_DIR}/{name}");
        let bytes = source_bytes(repo, SOURCE_COMMIT, &path)?;
        sources.push(Source {
            path,
            sha256: digest(&bytes),
            bytes: bytes.len(),
        });
    }
    write_json(
        Path::new(output),
        &Manifest {
            schema: "primefield-e2e.sources/v1".into(),
            repository: "https://github.com/isogeny-labs/crypto".into(),
            commit: SOURCE_COMMIT.into(),
            sources,
        },
    )
}

fn metric_mean(m: &complexity_accounting::Metric) -> Option<f64> {
    m.complete_mean()
}

fn compact_group(g: &GroupAccounting) -> Value {
    let mut phases = BTreeMap::new();
    for (name, metric) in &g.phases_gae {
        phases.insert(name, metric.complete_mean());
    }
    json!({"source":g.source,"stage":g.stage,"arm":g.arm,"base":g.method,"engine":g.engine,
        "arity":g.arity,"base_parameters":g.factor_base_parameters,"linalg":g.linalg,"instance":g.instance,
        "n":g.subgroup_order,"rows":g.rows,"producer_verified":g.producer_verified,"recovered":g.recovered,
        "exhausted":g.exhausted,"full_rank_reported":g.full_rank_reported,"missing_source_commit":g.missing_source_commit,
        "complete_cold_gae":g.complete_cold_gae,"recorded_gae_mean":metric_mean(&g.recorded_gae),
        "row_wall_seconds_mean":metric_mean(&g.row_wall_seconds),"solver_ops_mean":metric_mean(&g.solver_ops),
        "solver_gae_mean":metric_mean(&g.solver_gae),"solver_calls_mean":metric_mean(&g.solver_calls),
        "solver_ops_sum":g.solver_ops.complete_mean().map(|_|g.solver_ops.sum),"recorded_gae_sum":g.recorded_gae.complete_mean().map(|_|g.recorded_gae.sum),
        "solver_budget_exceeded_sum":g.solver_budget_exceeded.complete_mean().map(|_|g.solver_budget_exceeded.sum),"attempts_mean":metric_mean(&g.attempts),
        "relations_mean":metric_mean(&g.relations),"rank_mean":metric_mean(&g.final_rank),
        "rank_goal_mean":metric_mean(&g.rank_goal),"signed_population_mean":metric_mean(&g.signed_population),
        "base_columns_mean":metric_mean(&g.base_columns),"phases_gae_mean":phases,
        "phase_sum_mismatch":g.phase_sum_mismatch,"gaps":g.gaps,"unpriced_native":g.unpriced_counters,
        "solver_units":g.solver_units})
}

fn analyze(repo: &str, manifest_path: &str, output: &str) -> Result<(), String> {
    let manifest_bytes = fs::read(manifest_path).map_err(|e| e.to_string())?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).map_err(|e| e.to_string())?;
    if manifest.schema != "primefield-e2e.sources/v1" || manifest.sources.is_empty() {
        return Err("invalid manifest".into());
    }
    let mut all = Vec::new();
    let mut source_summary = Vec::new();
    let mut incomplete_sources = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut basenames = BTreeSet::new();
    for source in &manifest.sources {
        if !seen.insert(&source.path) {
            return Err(format!("duplicate source: {}", source.path));
        }
        let bytes = source_bytes(repo, &manifest.commit, &source.path)?;
        if bytes.len() != source.bytes || digest(&bytes) != source.sha256 {
            return Err(format!("source hash/length mismatch: {}", source.path));
        }
        let name = source.path.rsplit('/').next().unwrap();
        if !basenames.insert(name.to_owned()) {
            return Err(format!("duplicate source basename: {name}"));
        }
        let parsed = account_jsonl(
            name,
            std::str::from_utf8(&bytes).map_err(|e| e.to_string())?,
        )?;
        let preflight_rows: usize = parsed
            .groups
            .iter()
            .filter(|g| g.stage == "preflight")
            .map(|g| g.rows)
            .sum();
        let preflight_exhausted: usize = parsed
            .groups
            .iter()
            .filter(|g| g.stage == "preflight")
            .map(|g| g.exhausted)
            .sum();
        let other_rows: usize = parsed
            .groups
            .iter()
            .filter(|g| g.stage == "other")
            .map(|g| g.rows)
            .sum();
        if parsed.failed_timed_envelopes > 0
            || other_rows > 0
            || parsed
                .missing_output_by_stage_arm
                .keys()
                .any(|k| !k.starts_with("preflight:"))
        {
            incomplete_sources.insert(name.to_owned());
        }
        source_summary.push(json!({"source":name,"json_lines":parsed.json_lines,"failed_envelopes":parsed.failed_envelopes,
            "preflight_rows":preflight_rows,"preflight_exhausted":preflight_exhausted,"other_rows":other_rows,
            "failed_timed_envelopes":parsed.failed_timed_envelopes,"missing_output_by_stage_arm":parsed.missing_output_by_stage_arm}));
        all.extend(parsed.groups);
    }
    let timed: Vec<_> = all.iter().filter(|g| g.stage == "timed").collect();
    let mut comparisons = Vec::new();
    for source in &manifest.sources {
        let name = source.path.rsplit('/').next().unwrap();
        let rows: Vec<_> = timed
            .iter()
            .copied()
            .filter(|g| g.source == name && g.engine == "f4")
            .collect();
        let baseline: Vec<_> = rows
            .iter()
            .copied()
            .filter(|g| g.method == "prime-abscissa")
            .collect();
        let candidate: Vec<_> = rows
            .iter()
            .copied()
            .filter(|g| g.method == "prime-isogeny")
            .collect();
        if baseline.is_empty() || candidate.is_empty() {
            continue;
        }
        let bs = baseline.iter().map(|g| g.solver_ops.sum).sum::<f64>();
        let cs = candidate.iter().map(|g| g.solver_ops.sum).sum::<f64>();
        let bg = baseline.iter().map(|g| g.recorded_gae.sum).sum::<f64>();
        let cg = candidate.iter().map(|g| g.recorded_gae.sum).sum::<f64>();
        let br: usize = baseline.iter().map(|g| g.rows).sum();
        let cr: usize = candidate.iter().map(|g| g.rows).sum();
        let identities = |groups: &[&GroupAccounting]| {
            let mut result = BTreeMap::<String, usize>::new();
            for group in groups {
                for (key, count) in &group.comparison_identities {
                    *result.entry(key.clone()).or_default() += count;
                }
            }
            result
        };
        let compatible = rows
            .iter()
            .all(|g| !g.gaps.contains_key("comparison_identity_incomplete"))
            && identities(&baseline) == identities(&candidate)
            && !incomplete_sources.contains(name)
            && rows
                .iter()
                .flat_map(|g| g.solver_units.iter())
                .collect::<BTreeSet<_>>()
                .len()
                == 1
            && rows
                .iter()
                .all(|g| g.solver_units.iter().all(|u| !u.trim().is_empty()));
        let valid = rows
            .iter()
            .all(|g| g.solver_ops.missing == 0 && g.recorded_gae.missing == 0)
            && br == cr
            && bs > 0.0
            && cs > 0.0
            && bg > 0.0
            && cg > 0.0
            && compatible;
        comparisons.push(json!({"source":name,"baseline_rows":br,"candidate_rows":cr,
            "comparable_recorded_context":compatible,
            "raw_ops_ratio":valid.then_some(cs/bs),"recorded_gae_ratio":valid.then_some(cg/bg),
            "raw_ops_speedup":valid.then_some(bs/cs),"recorded_gae_speedup":valid.then_some(bg/cg),
            "scope":"ratio of recorded arm sums; not a complete cold-cost comparison or independent paired inference"}));
    }
    let mut fit_groups = BTreeMap::<String, Vec<(f64, f64)>>::new();
    for g in &timed {
        if !g.source.starts_with("e2e_full_rank_") || g.solver_units.len() != 1 {
            continue;
        }
        if incomplete_sources.contains(&g.source) {
            continue;
        }
        let scale = g
            .factor_base_parameters
            .get("t")
            .and_then(Value::as_str)
            .map(|t| format!("t={t}"))
            .or_else(|| {
                g.factor_base_parameters
                    .get("below")
                    .and_then(Value::as_str)
                    .map(|b| format!("below={b}"))
            });
        let Some(scale) = scale else {
            continue;
        };
        if let (Some(n), Some(ops)) = (g.subgroup_order, g.solver_ops.complete_mean()) {
            if ops > 0.0 {
                let key = format!(
                    "{}|{}|m={:?}|{}|{}|{}",
                    g.method,
                    g.engine,
                    g.arity,
                    g.linalg,
                    g.solver_units.iter().next().unwrap(),
                    scale
                );
                fit_groups.entry(key).or_default().push((n, ops));
            }
        }
    }
    let fits: Vec<_> = fit_groups
        .iter()
        .filter_map(|(key, pts)| {
            descriptive_fit(pts)
                .ok()
                .map(|fit| json!({"method":key,"fit":fit}))
        })
        .collect();
    let timed_rows: usize = timed.iter().map(|g| g.rows).sum();
    let verified: usize = timed.iter().map(|g| g.producer_verified).sum();
    let full_rank: usize = timed.iter().map(|g| g.full_rank_reported).sum();
    let exhausted: usize = timed.iter().map(|g| g.exhausted).sum();
    let missing_source: usize = timed.iter().map(|g| g.missing_source_commit).sum();
    let result = json!({"schema":"primefield-e2e.accounting/v1","source_commit":manifest.commit,
        "manifest_sha256":digest(&manifest_bytes),"scope":"archived producer counters; no new solver run; no independent recovery replay",
        "totals":{"timed_rows":timed_rows,"producer_verified":verified,"full_rank_reported":full_rank,
            "exhausted":exhausted,"missing_source_commit":missing_source,"complete_cost_qualified":0},
        "sources":source_summary,"groups":timed.iter().map(|g|compact_group(g)).collect::<Vec<_>>(),
        "comparisons":comparisons,"descriptive_solver_fits":fits,
        "qualification":"All complete cold totals unknown. Tiny fixed-t heterogeneous panels do not identify an optimized asymptotic exponent. Historical failures and preflight counts retained."});
    fs::create_dir_all(output).map_err(|e| e.to_string())?;
    write_json(&Path::new(output).join("results.json"), &result)?;
    let mut csv=String::from("source,base,engine,n,rows,recorded_gae_mean,solver_ops_mean,base_columns_mean,signed_population_mean,attempts_mean,relations_mean,rank_mean,exhausted\n");
    for g in &timed {
        let f = |v: Option<f64>| v.map(|v| v.to_string()).unwrap_or_default();
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            g.source,
            g.method,
            g.engine,
            f(g.subgroup_order),
            g.rows,
            f(metric_mean(&g.recorded_gae)),
            f(metric_mean(&g.solver_ops)),
            f(metric_mean(&g.base_columns)),
            f(metric_mean(&g.signed_population)),
            f(metric_mean(&g.attempts)),
            f(metric_mean(&g.relations)),
            f(metric_mean(&g.final_rank)),
            g.exhausted
        ));
    }
    fs::write(Path::new(output).join("measurements.csv"), csv).map_err(|e| e.to_string())?;
    println!("accounted {timed_rows} timed rows from {} hash-verified files; complete cold costs remain unknown",manifest.sources.len());
    Ok(())
}

fn model_outputs(output: &str) -> Result<(), String> {
    let mut scenarios = Vec::new();
    let mut plot_points = Vec::new();
    for (name, delta, la) in [
        ("unit trial, sparse LA", 0.0, 2.0),
        ("linear B trial, sparse LA", 1.0, 2.0),
        ("quadratic B trial, sparse LA", 2.0, 2.0),
        ("linear B trial, dense LA", 1.0, 3.0),
    ] {
        for arity in 2..=10 {
            let model = ExponentModel {
                arity,
                trial_b_exponent: delta,
                linear_algebra_b_exponent: la,
                ..Default::default()
            };
            let optimum = optimize_exponent(&model).map_err(|e| format!("{e:?}"))?;
            scenarios.push(json!({"name":name,"arity":arity,"beta_opt":optimum.beta,"exponent":optimum.work_exponent,
                "normalized_slope":optimum.normalized_slope,"dominant_phases":optimum.dominant_phases}));
            if arity <= 7 {
                for i in 0..=20 {
                    let beta = i as f64 / 40.0;
                    let rel = 1.0 + (delta + 1.0 - arity as f64) * beta;
                    let saturated = (delta + 1.0) * beta;
                    let la_phase = la * beta;
                    plot_points.push(json!([
                        name,
                        arity,
                        beta,
                        rel.max(saturated).max(la_phase).max(beta)
                    ]));
                }
            }
        }
    }
    let stress_models = [
        (
            "m5 linear trial; n^0.1 retention loss",
            ExponentModel {
                arity: 5,
                retention_n_penalty: 0.1,
                ..Default::default()
            },
        ),
        (
            "m3 linear trial; B^3 setup",
            ExponentModel {
                arity: 3,
                extra_costs: vec![AffineCost {
                    name: "setup".into(),
                    intercept: 0.0,
                    slope: 3.0,
                }],
                ..Default::default()
            },
        ),
        (
            "m5 linear trial; memory beta<=0.15",
            ExponentModel {
                arity: 5,
                memory_constraints: vec![MemoryConstraint {
                    name: "base_memory".into(),
                    intercept: 0.0,
                    slope: 1.0,
                    max_exponent: 0.15,
                }],
                ..Default::default()
            },
        ),
    ];
    let mut stress = Vec::new();
    for (name, model) in stress_models {
        stress.push(json!({"name":name,"model":model,"optimum":optimize_exponent(&model).map_err(|e|format!("{e:?}"))?}));
    }
    fs::create_dir_all(output).map_err(|e| e.to_string())?;
    // The plot file is a small numeric CSV; all exponent arithmetic is native.
    let mut csv = String::from("scenario,arity,beta,exponent\n");
    for point in &plot_points {
        csv.push_str(&format!(
            "\"{}\",{},{},{}\n",
            point[0].as_str().unwrap(),
            point[1],
            point[2],
            point[3]
        ));
    }
    fs::write(Path::new(output).join("model-curves.csv"), csv).map_err(|e| e.to_string())?;
    write_json(
        &Path::new(output).join("model-results.json"),
        &json!({"schema":"primefield-e2e.model/v1",
        "assumptions":["fixed arity; base population n^beta; Theta(B) independent rank increments; at most one per attempt",
            "random-sum coverage with constant useful-retention unless specified; polynomial powers are supplied assumptions",
            "sparse LA assumes O(mB) nonzeros and O(B) iterations; dense auditing must be separately charged",
            "cold single target; exponents omit fixed-arity constants and polylogarithmic arithmetic"],
        "scenarios":scenarios,"stress_cases":stress}),
    )
}

#[derive(Deserialize)]
struct PipelineInput {
    work_unit: String,
    exponent_model: ExponentModel,
    relation_input: FiniteRelationInput,
    additional_phases: Vec<PhaseBudget>,
    required_phases: Vec<String>,
    target_count_log2: f64,
}

fn evaluate(input: &str, output: &str) -> Result<(), String> {
    let bytes = fs::read(input).map_err(|e| e.to_string())?;
    let request: PipelineInput = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if request.work_unit.trim().is_empty() {
        return Err("work_unit is required".into());
    }
    let exponent = optimize_exponent(&request.exponent_model).map_err(|e| format!("{e:?}"))?;
    if request.exponent_model.arity != request.relation_input.arity {
        return Err("finite and asymptotic relation arities must agree".into());
    }
    let finite_beta = request.relation_input.factor_base_population_log2
        / request.relation_input.group_order_log2;
    if !finite_beta.is_finite()
        || finite_beta < exponent.feasible_beta_min
        || finite_beta > exponent.feasible_beta_max
    {
        return Err("finite factor-base beta violates the model's feasible interval".into());
    }
    let relation = finite_relation_budget(&request.relation_input).map_err(|e| format!("{e:?}"))?;
    let slope_comparison =
        compare_slopes(exponent.work_exponent, 0.5).map_err(|e| format!("{e:?}"))?;
    let mut phases = request.additional_phases;
    phases.push(PhaseBudget {
        name: "relation_collection".into(),
        work: PhaseWork::Known(relation.relation_work_log2),
        reusable: false,
    });
    let aggregate =
        aggregate_phase_budgets(&phases, &request.required_phases, request.target_count_log2);
    let (total, error) = match aggregate {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error)),
    };
    write_json(
        Path::new(output),
        &json!({"schema":"primefield-e2e.pipeline/v1","input_sha256":digest(&bytes),
        "work_unit":request.work_unit,"conditional_exponent":exponent,"slope_comparison":slope_comparison,"conditional_finite_relations":relation,
        "phase_ledger":phases,"complete_budget":total,"complete_budget_error":error,
        "scope":"user-supplied conditional assumptions; arithmetic accounting, not measured or certified performance"}),
    )
}

fn run() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("pin") if args.len()==4=>pin(&args[2],&args[3]),
        Some("analyze") if args.len()==5=>analyze(&args[2],&args[3],&args[4]),
        Some("models") if args.len()==3=>model_outputs(&args[2]),
        Some("evaluate") if args.len()==4=>evaluate(&args[2],&args[3]),
        _=>Err("usage: primefield-e2e pin REPO MANIFEST | analyze REPO MANIFEST OUTPUT_DIR | models OUTPUT_DIR | evaluate INPUT_JSON OUTPUT_JSON".into()),
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("primefield-e2e: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_pipeline_preserves_unknowns_and_rejects_mixed_models() {
        let dir = env::temp_dir().join(format!("primefield-e2e-input-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let input = dir.join("input.json");
        let output = dir.join("output.json");
        let mut data: Value = serde_json::from_str(include_str!(
            "../../../research/primefield_e2e_20261125/pipeline-input.json"
        ))
        .unwrap();
        write_json(&input, &data).unwrap();
        evaluate(input.to_str().unwrap(), output.to_str().unwrap()).unwrap();
        let result: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        assert!(result["complete_budget"].is_null());
        assert_eq!(
            result["complete_budget_error"]["code"],
            "unknown_phase_cost"
        );
        data["relation_input"]["arity"] = json!(4);
        write_json(&input, &data).unwrap();
        assert!(evaluate(input.to_str().unwrap(), output.to_str().unwrap())
            .unwrap_err()
            .contains("arities"));
        data["relation_input"]["arity"] = json!(5);
        data["exponent_model"]["beta_max"] = json!(0.1);
        write_json(&input, &data).unwrap();
        assert!(evaluate(input.to_str().unwrap(), output.to_str().unwrap())
            .unwrap_err()
            .contains("feasible interval"));
        data["exponent_model"]["beta_max"] = json!(1.0);
        data["additional_phases"][2]["work"] = json!({"status":"known","log2_work":10.0});
        data["target_count_log2"] = json!(4.0);
        write_json(&input, &data).unwrap();
        evaluate(input.to_str().unwrap(), output.to_str().unwrap()).unwrap();
        let result: Value = serde_json::from_slice(&fs::read(&output).unwrap()).unwrap();
        let cold = result["complete_budget"]["cold_work_log2"]
            .as_f64()
            .unwrap();
        let total = result["complete_budget"]["total_work_log2"]
            .as_f64()
            .unwrap();
        let average = result["complete_budget"]["amortized_work_log2"]
            .as_f64()
            .unwrap();
        assert!(cold > 46.0 && cold < 48.0);
        assert!(total > cold && average < cold);
        assert!((total - average - 4.0).abs() < 1e-12);
        fs::remove_dir_all(dir).unwrap();
    }
}
