//! Native fixture/spec preparation and recorded-value analysis for the
//! arbitrary-target known-log study. The measured runner is ecbench itself.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crypto_lib::cryptanalysis::ecbench::{
    band,
    canonical::sha256_hex,
    record::Record,
    workload::{CurveSpec, Instance},
};
use crypto_lib::cryptanalysis::ic_boundary::{
    invmod, CountedGroup, GroupOps, PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::prime_small_x::{toy_curve_with_cofactor, PrimeField};
use num_traits::ToPrimitive;
use serde_json::{json, Value};

#[path = "known_log_targets/registry.rs"]
mod registry;

fn integer(v: &Value) -> Result<u64, String> {
    if let Some(n) = v.as_u64() {
        return Ok(n);
    }
    let text = v.as_str().ok_or("expected unsigned integer")?;
    match text.strip_prefix("0x") {
        Some(hex) => u64::from_str_radix(hex, 16),
        None => text.parse(),
    }
    .map_err(|e| e.to_string())
}

fn read(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

fn write(path: &Path, value: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, serde_json::to_string_pretty(value).unwrap() + "\n").map_err(|e| e.to_string())
}

fn explicit(i: &PrimeInstance) -> Value {
    json!({"kind":"prime_explicit", "p":i.curve.p, "a":i.curve.a,
        "b":i.curve.b, "group_order":i.group_order, "r":i.r,
        "gx":i.generator.0, "gy":i.generator.1})
}

fn prime(v: &Value) -> Result<PrimeInstance, String> {
    let spec: CurveSpec = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    match spec.build()? {
        Instance::Prime(i) => Ok(i),
        _ => Err("study requires prime-field instances".into()),
    }
}

fn registered_prime(registry: &Value, bits: u32) -> Result<PrimeInstance, String> {
    let call = format!("find_prime_order_curve({bits}, 59297)");
    let row = registry["curves"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["params"]["generator_call"] == call)
        .ok_or_else(|| format!("registered constructor missing: {call}"))?;
    let rep = row["representations"]
        .as_array()
        .and_then(|r| r.first())
        .ok_or("registered curve lacks a representation")?;
    prime(
        &json!({"kind":"prime_explicit", "p":integer(&row["params"]["p"])?,
        "a":integer(&row["params"]["a"])?, "b":integer(&row["params"]["b"])?,
        "group_order":integer(&row["order"])?, "r":integer(&rep["curve"]["subgroup_order"])?,
        "gx":integer(&rep["curve"]["generator"][0])?,
        "gy":integer(&rep["curve"]["generator"][1])?}),
    )
}

/// The native point counter constructs these bounded fixtures. Recount by
/// Legendre symbols independently before writing their source records.
fn fixtures(study: &Path, repo: &Path) -> Result<(), String> {
    let mut rows = Vec::new();
    for h in [2, 4, 8] {
        let c = toy_curve_with_cofactor(16, 20261021, h)?;
        let r = c.order.to_u64().ok_or("fixture order exceeds u64")?;
        let curve = PrimeCurve {
            p: c.field
                .modulus()
                .to_u64()
                .ok_or("fixture modulus exceeds u64")?,
            a: c.a,
            b: c.b,
        };
        let mut counted = 1;
        for x in 0..curve.p {
            counted += match curve.legendre(curve.rhs(x)) {
                0 => 1,
                1 => 2,
                _ => 0,
            };
        }
        if counted != h * r {
            return Err("independent fixture point count disagrees".into());
        }
        let mut generator = None;
        for x in 0..curve.p {
            if let Some(y) = curve.sqrt(curve.rhs(x)) {
                let g = curve.mul(&mut GroupOps::default(), PrimePoint::affine(x, y), h);
                if !g.infinity && curve.mul(&mut GroupOps::default(), g, r).infinity {
                    generator = Some((g.x, g.y));
                    break;
                }
            }
        }
        let i = PrimeInstance {
            name: String::new(),
            curve,
            group_order: h * r,
            r,
            cofactor: h,
            generator: generator.ok_or("no fixture subgroup generator")?,
        };
        let id = i.curve_id();
        rows.push(json!({"a":i.curve.a,"b":i.curve.b,"cofactor":h,
            "generator":{"x":i.generator.0,"y":i.generator.1},
            "generator_call":format!("toy_curve_with_cofactor(16, 20261021, {h}); first nonzero cofactor-projected x"),
            "group_order":i.group_order,"subgroup_order":r,"p":i.curve.p,
            "name":id.slug,"slug":id.slug,"icv1":id.icv1,"explicit":explicit(&i),
            "point_count_validation":"square-bitset construction and independent Legendre-symbol recount agree"}));
    }
    // Also record the changed generator representations. Registry ingestion
    // merges these by model and retains each exact generator's EC1 identity.
    let registry = read(&repo.join("docs/curves/registry.json"))?;
    let original = registered_prime(&registry, 16)?;
    for c in [2, 7] {
        let g = original
            .curve
            .mul(&mut GroupOps::default(), original.generator_point(), c);
        let id = original.curve_id();
        rows.push(
            json!({"a":original.curve.a,"b":original.curve.b,"cofactor":1,
            "generator":{"x":g.x,"y":g.y},"group_order":original.group_order,
            "subgroup_order":original.r,"p":original.curve.p,
            "generator_call":format!("[{}]G of find_prime_order_curve(16, 59297)",c),
            "name":id.slug,"slug":id.slug,"icv1":id.icv1}),
        );
    }
    write(
        &study.join("fixtures.json"),
        &json!({"schema":"known-log-study.fixtures/v1", "curves":rows}),
    )?;
    let path = repo.join("docs/curves/sources/generated.json");
    let mut source = read(&path)?;
    let existing = source["curves"]
        .as_array_mut()
        .ok_or("generated source has no curves")?;
    for mut row in rows {
        row.as_object_mut().unwrap().remove("explicit");
        row.as_object_mut()
            .unwrap()
            .remove("point_count_validation");
        if !existing
            .iter()
            .any(|v| v["slug"] == row["slug"] && v["generator"] == row["generator"])
        {
            existing.push(row);
        }
    }
    write(&path, &source)
}

fn known(multiplier: u64) -> Value {
    json!({"id":"ic.known_log_mitm2_v1", "params":{"size":"32", "base_seed":"20261021",
        "base_multiplier":multiplier.to_string(), "max_pair_work":"1100", "max_trials":"2000000"}})
}

fn arms(multiplier: u64, projected: bool) -> Vec<Value> {
    let baseline = json!({"id":"ic.pipeline", "params":{
        "factor_base":"prime-abscissa:size=32", "oracle":"mitm:m=2,negation_folded=1",
        "targets":"walk", "linalg":"incremental-gauss", "max_trials":"2000000"}});
    let mut out = vec![
        json!({"name":"rho-neg","role":"reference","method":{"id":"rho.negation"}}),
        json!({"name":"abscissa","role":"baseline","method":baseline}),
        json!({"name":"known-log","role":"candidate","method":known(1)}),
        json!({"name":"known-log-aa","role":"control","method":known(1)}),
        json!({"name":"abscissa-aa","role":"control","method":baseline}),
    ];
    if multiplier != 1 {
        out.push(json!({"name":"transported","role":"candidate","method":known(multiplier)}));
    }
    if projected {
        let mut matched = baseline;
        matched["params"]["factor_base"] = json!("prime-projected-abscissa:size=32,scan_cap=65518");
        out.push(json!({"name":"projected","role":"baseline","method":matched}));
    }
    out
}

fn spec(curves: Vec<Value>, target_seed: u64, arms: Vec<Value>, smoke: bool) -> Value {
    json!({"schema":"ecbench.spec/v1", "label":"arbitrary-target known-log two-sum study",
        "description":"Frozen one-target cold-cost experiment; see ../PROTOCOL.md. Public target logs are absent.",
        "workloads":{"curves":curves,"targets_per_curve":if smoke {2} else {8},
            "target_seed":target_seed,"target_kind":"public"},
        "arms":arms,"measurement":{"rounds":if smoke {1} else {5},"warmup":1,
            "order":"alternate","seed":20261021,"isolation_required":"L2","timeout_seconds":120}})
}

fn specs(study: &Path, repo: &Path) -> Result<(), String> {
    let registry = read(&repo.join("docs/curves/registry.json"))?;
    let mut curves = Vec::new();
    for bits in [12, 16, 20, 24] {
        curves.push(explicit(&registered_prime(&registry, bits)?));
    }
    let dir = study.join("specs");
    write(
        &dir.join("primary.json"),
        &spec(curves.clone(), 2026102101, arms(1, false), false),
    )?;
    write(
        &dir.join("holdout.json"),
        &spec(curves, 2026102102, arms(1, false), false),
    )?;
    write(
        &dir.join("smoke.json"),
        &spec(
            vec![explicit(&registered_prime(&registry, 12)?)],
            2026102100,
            arms(1, false),
            true,
        ),
    )?;
    let original = registered_prime(&registry, 16)?;
    for c in [1, 2, 7] {
        let mut changed = explicit(&original);
        let g = original
            .curve
            .mul(&mut GroupOps::default(), original.generator_point(), c);
        changed["gx"] = json!(g.x);
        changed["gy"] = json!(g.y);
        let inverse = invmod(c, original.r).ok_or("generator multiplier not invertible")?;
        write(
            &dir.join(format!("generator-{c}.json")),
            &spec(vec![changed], 2026102103, arms(inverse, false), false),
        )?;
    }
    let fixtures = read(&study.join("fixtures.json"))?;
    let cofactor = fixtures["curves"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|r| r.get("explicit").cloned())
        .collect();
    write(
        &dir.join("cofactor-v2.json"),
        &spec(cofactor, 2026102104, arms(1, true), false),
    )?;
    let mut hashes = BTreeMap::new();
    for entry in fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        hashes.insert(
            path.file_name().unwrap().to_string_lossy().into_owned(),
            sha256_hex(&fs::read(&path).map_err(|e| e.to_string())?),
        );
    }
    write(&study.join("spec-hashes.json"), &json!(hashes))
}

fn summarize(session: &Path, output: &Path) -> Result<(), String> {
    let bytes = fs::read(session.join("records.jsonl")).map_err(|e| e.to_string())?;
    let records: Vec<Record> = std::str::from_utf8(&bytes)
        .map_err(|e| e.to_string())?
        .lines()
        .map(|line| serde_json::from_str(line).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let pricing = band::band(&records, &band::DEFAULT_WEIGHTS)?;
    let mut ratios = Vec::new();
    for candidate in pricing
        .rows
        .iter()
        .filter(|r| matches!(r.arm.as_str(), "known-log" | "transported"))
    {
        for reference in ["abscissa", "projected", "rho-neg"] {
            if let Some(base) = pricing
                .rows
                .iter()
                .find(|r| r.curve == candidate.curve && r.arm == reference)
            {
                let ratios_at_weights: Vec<_> = base
                    .mean_s
                    .iter()
                    .zip(&candidate.mean_s)
                    .map(|(a, b)| match (a, b) {
                        (Some(a), Some(b)) if *b > 0.0 => Some(a / b),
                        _ => None,
                    })
                    .collect();
                ratios.push(json!({"curve":candidate.curve,"candidate":candidate.arm,"reference":reference,
                    "ratio_reference_over_candidate":ratios_at_weights,
                    "all_runs_verified":candidate.runs == candidate.verified && base.runs == base.verified,
                    "native_counts_complete":candidate.uncounted.is_empty() && base.uncounted.is_empty()}));
            }
        }
    }
    let mut groups: BTreeMap<(String, String), Vec<&Record>> = BTreeMap::new();
    for r in records.iter().filter(|r| !r.warmup) {
        groups
            .entry((r.workload.workload_id.clone(), r.arm.clone()))
            .or_default()
            .push(r);
    }
    let mut workloads = Vec::new();
    for ((workload_id, arm), rows) in groups {
        let mut statuses = BTreeMap::new();
        let mut phase_gae: BTreeMap<String, f64> = BTreeMap::new();
        let mut counters: BTreeMap<String, u64> = BTreeMap::new();
        let mut native = BTreeMap::new();
        let mut priced_totals = [Some(0.0); 3];
        let mut online_complete = 0;
        let mut online_wall = 0u64;
        let mut fb_hashes = BTreeMap::new();
        for r in &rows {
            *statuses.entry(r.outcome.status.clone()).or_insert(0u64) += 1;
            for p in &r.phases {
                *phase_gae.entry(p.name.clone()).or_default() += p.gae;
            }
            for (k, v) in &r.counters {
                *counters.entry(k.clone()).or_default() += v;
            }
            for (k, v) in band::unpriced_counts(r) {
                match v {
                    Some(n) => *native.entry(k).or_insert(0u64) += n,
                    None => return Err(format!("uncounted native work in {}", r.record_id)),
                }
            }
            if let Some(fb) = &r.factor_base {
                fb_hashes.insert(fb.points_sha256.clone(), fb.signed_points);
            }
            if let Some(online) = &r.online {
                if online.phases_ns.values().sum::<u64>() != online.wall_ns {
                    return Err("online phases do not sum to the interval".into());
                }
                online_complete += 1;
                online_wall += online.wall_ns;
            }
            for (index, weight) in band::DEFAULT_WEIGHTS.iter().enumerate() {
                priced_totals[index] = match (priced_totals[index], band::priced_gae(r, *weight)?) {
                    (Some(a), Some(b)) => Some(a + b),
                    _ => None,
                };
            }
        }
        let r = rows[0];
        workloads.push(json!({"workload_id":workload_id,"arm":arm,
            "curve":r.workload.curve,"target":r.workload.target,"rounds":rows.len(),
            "statuses":statuses,"priced_gae_totals":priced_totals,"phase_gae_totals":phase_gae,
            "counters":counters,"native_unpriced_totals":native,"online_complete":online_complete,
            "first_run_counters":r.counters,
            "online_wall_ns_sum_descriptive":online_wall,"factor_base_point_hashes":fb_hashes}));
    }
    write(
        output,
        &json!({"schema":"known-log-study.summary/v1",
        "records_sha256":sha256_hex(&bytes),"session":session.to_string_lossy(),
        "weights":band::DEFAULT_WEIGHTS,"band":pricing,"ratios":ratios,"workloads":workloads}),
    )
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    match args.get(1).map(String::as_str) {
        Some("fixtures") if args.len() == 3 => fixtures(Path::new(&args[2]), repo),
        Some("register") if args.len() == 3 => registry::register(Path::new(&args[2]), repo),
        Some("specs") if args.len() == 3 => specs(Path::new(&args[2]), repo),
        Some("summarize") if args.len() == 4 => summarize(Path::new(&args[2]), Path::new(&args[3])),
        _ => Err(
            "usage: known_log_targets fixtures STUDY | register STUDY | specs STUDY | summarize SESSION OUTPUT"
                .into(),
        ),
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_exact_registry_integers_without_float_conversion() {
        for (input, expected) in [(json!(17), 17), (json!("17"), 17), (json!("0x11"), 17)] {
            assert_eq!(integer(&input).unwrap(), expected);
        }
        assert!(integer(&json!(1.5)).is_err());
    }
    #[test]
    fn specs_use_public_single_target_workloads_and_frozen_budgets() {
        let doc = spec(Vec::new(), 2026102101, arms(1, true), false);
        assert_eq!(doc["workloads"]["target_kind"], "public");
        assert_eq!(doc["measurement"]["timeout_seconds"], 120);
        assert_eq!(doc["arms"].as_array().unwrap().len(), 6);
        assert_eq!(doc["arms"][2]["method"], doc["arms"][3]["method"]);
        assert_eq!(doc["arms"][1]["method"], doc["arms"][4]["method"]);
    }
}
