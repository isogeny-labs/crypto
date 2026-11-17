//! Source-linked TR-04 reporting. Reads retained records; never runs a census.
//! Comparisons use exact rational arithmetic and keep models and B values separate.
use crate::tr04::{Combined, Manifest, Summary, BASE_SIZES, METHODS, PRIMES};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

const ARMS: [&str; 9] = [
    "random",
    "progression",
    "collision_avoid",
    "twist_l1",
    "twist_l2",
    "twist_l3",
    "twist_l4",
    "planted_rank",
    "b2_negative",
];
type MetricMaps = (Map<String, Value>, Map<String, Value>);
const LIMITS: &str = "Exact finite operation-counter audit only. Generic decomposition (bounded by sqrt(r); cannot beat rho). Primitive work counters are not independently recomputed; allocator/cache/branch/RNG-internal and byte-peak costs are unpriced. No physical runtime, complete GAE, ECDLP speedup, statistical interval or general-curve claim. Failed selector/rank mass is retained. Audit DP/trie/cache acceleration is not free algorithm work.";

fn hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
fn load(path: &Path) -> Result<Value, String> {
    serde_json::from_slice(&fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?)
        .map_err(|e| format!("{}: {e}", path.display()))
}
fn write(path: &Path, value: &Value) -> Result<(), String> {
    crate::evidence::write_json_atomic(path, value)
}
fn text(path: &Path, value: &str) -> Result<(), String> {
    fs::write(path, value).map_err(|e| format!("{}: {e}", path.display()))
}
fn file(path: &Path, kind: &str) -> Result<Value, String> {
    let p = fs::canonicalize(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let bytes = fs::read(&p).map_err(|e| e.to_string())?;
    Ok(json!({"path":p,"sha256":hash(&bytes),"bytes":bytes.len(),"kind":kind}))
}
fn safe(path: &str) -> Result<(), String> {
    if path.is_empty()
        || Path::new(path)
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err("unsafe evidence path".into());
    }
    Ok(())
}
fn optional(entry: &Value, key: &str) -> Option<PathBuf> {
    entry[key]
        .as_str()
        .map(PathBuf::from)
        .filter(|p| p.is_file())
}
fn metadata(entry: &Value, key: &str) -> Value {
    optional(entry, key)
        .and_then(|p| load(&p).ok())
        .unwrap_or(Value::Null)
}
fn utc() -> Result<String, String> {
    let result = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map_err(|e| e.to_string())?;
    if !result.status.success() {
        return Err("UTC date command failed".into());
    }
    Ok(String::from_utf8_lossy(&result.stdout).trim().into())
}

/// Overflow-free comparison of unsigned rational numbers by continued fractions.
fn rational_cmp(mut a: u128, mut b: u128, mut c: u128, mut d: u128) -> Result<Ordering, String> {
    if b == 0 || d == 0 {
        return Err("zero probability/work denominator".into());
    }
    let mut reverse = false;
    loop {
        let order = (a / b).cmp(&(c / d));
        if order != Ordering::Equal {
            return Ok(if reverse { order.reverse() } else { order });
        }
        let (r, s) = (a % b, c % d);
        if r == 0 || s == 0 {
            let order = r.cmp(&s);
            return Ok(if reverse { order.reverse() } else { order });
        }
        (a, b, c, d) = (b, r, d, s);
        reverse = !reverse;
    }
}
fn decimal(value: &Value) -> Result<u128, String> {
    value
        .as_str()
        .ok_or("expected decimal counter")?
        .parse()
        .map_err(|_| "invalid decimal counter".into())
}
fn checked_sum(mut values: impl Iterator<Item = u128>) -> Result<u128, String> {
    values.try_fold(0u128, |a, b| {
        a.checked_add(b)
            .ok_or_else(|| "report count overflow".into())
    })
}
fn components(value: &Combined) -> Result<BTreeMap<String, u128>, String> {
    if value.denominator == 0 {
        return Err("empty exact distribution".into());
    }
    let mut result = BTreeMap::new();
    for (prefix, hist) in value.rank_hist_by_prefix.iter().enumerate() {
        if checked_sum(hist.iter().copied())? != value.denominator {
            return Err("rank histogram denominator mismatch".into());
        }
        for rank in 1..hist.len() {
            result.insert(
                format!("rank.prefix{prefix}.rank{rank}"),
                checked_sum(hist[rank..].iter().copied())?,
            );
        }
    }
    Ok(result)
}
fn work_components(value: &Combined) -> Result<BTreeMap<String, u128>, String> {
    let mut result = BTreeMap::new();
    let total = serde_json::to_value(&value.total_work_sum).map_err(|e| e.to_string())?;
    for (key, count) in total.as_object().ok_or("work must be an object")? {
        result.insert(format!("total.{key}"), decimal(count)?);
    }
    for milestone in &value.milestones {
        if milestone.attained.checked_add(milestone.missing) != Some(value.denominator) {
            return Err("milestone denominator mismatch".into());
        }
        let work = serde_json::to_value(&milestone.work_sum).map_err(|e| e.to_string())?;
        for (key, count) in work.as_object().ok_or("work must be an object")? {
            result.insert(
                format!("milestone{}.{}", milestone.rank, key),
                decimal(count)?,
            );
        }
    }
    Ok(result)
}
fn contrast(candidate: &Combined, reference: &Combined) -> Result<Value, String> {
    let (utility, reference_utility) = (components(candidate)?, components(reference)?);
    let (work, reference_work) = (work_components(candidate)?, work_components(reference)?);
    if utility.keys().ne(reference_utility.keys()) || work.keys().ne(reference_work.keys()) {
        return Err("incompatible comparison components".into());
    }
    let (mut better, mut worse) = (0u64, 0u64);
    let mut details = Map::new();
    for (label, left, right, invert) in [
        ("rank_survival", &utility, &reference_utility, false),
        ("paid_work", &work, &reference_work, true),
    ] {
        let mut values = Map::new();
        for (name, value) in left {
            let baseline = right[name];
            let raw = rational_cmp(
                *value,
                candidate.denominator,
                baseline,
                reference.denominator,
            )?;
            let order = if invert { raw.reverse() } else { raw };
            match order {
                Ordering::Greater => better += 1,
                Ordering::Less => worse += 1,
                Ordering::Equal => {}
            }
            values.insert(name.clone(),json!({"candidate_numerator":value.to_string(),"candidate_denominator":candidate.denominator.to_string(),"reference_numerator":baseline.to_string(),"reference_denominator":reference.denominator.to_string(),"direction":match order{Ordering::Greater=>"better",Ordering::Less=>"worse",Ordering::Equal=>"equal"}}));
        }
        details.insert(label.into(), Value::Object(values));
    }
    let verdict = match (better, worse) {
        (0, 0) => "tie_within_model",
        (_, 0) => "within_model_pareto",
        (0, _) => "worse_within_model",
        _ => "mixed_tradeoff",
    };
    Ok(
        json!({"verdict":verdict,"better_components":better,"worse_components":worse,"components":details}),
    )
}

fn bind_input_receipts(index: &Value, shards: &[Value]) -> Result<(), String> {
    let inputs = index["inputs"]
        .as_array()
        .ok_or("shard input index missing inputs")?;
    if inputs.len() != PRIMES.len() || shards.len() != PRIMES.len() {
        return Err("source receipt binding requires exactly three shards".into());
    }
    let mut seen = BTreeSet::new();
    for entry in inputs {
        let p = field_prime(entry).ok_or("source receipt binding lacks a frozen field identity")?;
        if !seen.insert(p) {
            return Err("duplicate field in source receipt binding".into());
        }
        let shard = shards
            .iter()
            .find(|s| s["prime"].as_u64() == Some(p))
            .ok_or("indexed field absent from summary shards")?;
        let manifest = optional(entry, "manifest_path").ok_or("indexed manifest is missing")?;
        let verification = optional(entry, "verification_path")
            .ok_or("indexed verification receipt is missing")?;
        let input = fs::canonicalize(Path::new(
            entry["input_path"]
                .as_str()
                .ok_or("indexed input path missing")?,
        ))
        .map_err(|e| e.to_string())?;
        let actual_manifest = fs::canonicalize(&manifest).map_err(|e| e.to_string())?;
        let actual_verification = fs::canonicalize(&verification).map_err(|e| e.to_string())?;
        if actual_manifest != input.join("manifest.json")
            || actual_verification != input.join("verification.json")
            || file(&manifest, "receipt")?["sha256"] != shard["manifest_sha256"]
            || file(&verification, "receipt")?["sha256"] != shard["verification_sha256"]
        {
            return Err(
                "indexed raw source receipts differ from the accepted summary shard".into(),
            );
        }
    }
    if seen != PRIMES.into_iter().collect() {
        return Err("missing indexed field in source receipt binding".into());
    }
    Ok(())
}
fn check_summary(input: &Path, protocol: &Value, index: &Value) -> Result<Summary, String> {
    let manifest: Manifest =
        serde_json::from_value(load(&input.join("manifest.json"))?).map_err(|e| e.to_string())?;
    if manifest.status != "complete"
        || manifest.kind != "summary"
        || manifest.error.is_some()
        || manifest.source.protocol_canonical_sha256.as_deref()
            != protocol["freeze"]["protocol_sha256"].as_str()
    {
        return Err("summary manifest is incomplete or protocol differs".into());
    }
    let mut seen = BTreeSet::new();
    for artifact in &manifest.artifacts {
        safe(&artifact.path)?;
        if !seen.insert(&artifact.path)
            || hash(&fs::read(input.join(&artifact.path)).map_err(|e| e.to_string())?)
                != artifact.sha256
        {
            return Err("summary artifact digest/inventory mismatch".into());
        }
    }
    if !seen.iter().any(|path| path.as_str() == "summary.json") {
        return Err("summary absent from manifest inventory".into());
    }
    let summary: Summary =
        serde_json::from_value(load(&input.join("summary.json"))?).map_err(|e| e.to_string())?;
    if summary.schema != "twist-tr04/summary/v1"
        || summary.cells.len() != 42
        || summary.shards.len() != 3
    {
        return Err("incomplete summary census".into());
    }
    bind_input_receipts(index, &summary.shards)?;
    let mut shard_primes = BTreeSet::new();
    for shard in &summary.shards {
        let p = shard["prime"]
            .as_u64()
            .ok_or("summary shard prime missing")?;
        if !PRIMES.contains(&p) || !shard_primes.insert(p) {
            return Err("duplicate/unexpected summary shard".into());
        }
        let manifest_path = PathBuf::from(
            shard["manifest_path"]
                .as_str()
                .ok_or("raw manifest path missing")?,
        );
        let verification_path = PathBuf::from(
            shard["verification_path"]
                .as_str()
                .ok_or("raw verification path missing")?,
        );
        let raw_manifest = load(&manifest_path)?;
        let verification = load(&verification_path)?;
        if file(&manifest_path, "receipt")?["sha256"] != shard["manifest_sha256"]
            || file(&verification_path, "receipt")?["sha256"] != shard["verification_sha256"]
            || raw_manifest["status"] != "complete"
            || raw_manifest["kind"] != "shard"
            || raw_manifest["prime"].as_u64() != Some(p)
            || raw_manifest["source"] != shard["source"]
            || verification["status"] != "passed"
            || verification["manifest_sha256"] != shard["manifest_sha256"]
            || verification["checked_model_dispositions"].as_u64() != Some(p * p)
            || verification["work_counters_independent"] != false
        {
            return Err("source shard/verification receipt changed or is incomplete".into());
        }
    }
    let mut cells = BTreeSet::new();
    let mut model_sets = BTreeMap::<u64, BTreeSet<(u64, u64)>>::new();
    for cell in &summary.cells {
        if !PRIMES.contains(&cell.prime)
            || !BASE_SIZES.contains(&cell.base_size)
            || !METHODS.contains(&cell.method.as_str())
            || !cells.insert((cell.prime, cell.base_size, &cell.method))
            || cell.models.is_empty()
            || cell.model_count != cell.models.len()
        {
            return Err("invalid or duplicate summary cell".into());
        }
        let models = cell
            .models
            .iter()
            .map(|m| (m.parameters.a, m.parameters.b))
            .collect::<BTreeSet<_>>();
        if models.len() != cell.models.len()
            || cell.models.iter().any(|m| m.parameters.p != cell.prime)
        {
            return Err("duplicate or wrong-prime summary model".into());
        }
        if let Some(expected) = model_sets.get(&cell.prime) {
            if expected != &models {
                return Err("unmatched models across selector/base cells".into());
            }
        } else {
            model_sets.insert(cell.prime, models);
        }
        for model in &cell.models {
            components(&model.aggregate)?;
            work_components(&model.aggregate)?;
        }
    }
    Ok(summary)
}
fn comparisons(summary: &Summary) -> Result<Vec<Value>, String> {
    let mut result = Vec::new();
    for p in PRIMES {
        for b in BASE_SIZES {
            for method in ["twist_l1", "twist_l2", "twist_l3", "twist_l4"] {
                for reference in ["random", "collision_avoid"] {
                    let left = summary
                        .cells
                        .iter()
                        .find(|c| c.prime == p && c.base_size == b && c.method == method)
                        .ok_or("candidate cell missing")?;
                    let right = summary
                        .cells
                        .iter()
                        .find(|c| c.prime == p && c.base_size == b && c.method == reference)
                        .ok_or("reference cell missing")?;
                    let mut models = Vec::new();
                    let mut counts = BTreeMap::<String, u64>::new();
                    for model in &left.models {
                        let other = right
                            .models
                            .iter()
                            .find(|m| {
                                m.parameters == model.parameters
                                    && m.group_order == model.group_order
                                    && m.g == model.g
                            })
                            .ok_or("comparison has unmatched model/order/generator")?;
                        let comparison = contrast(&model.aggregate, &other.aggregate)?;
                        let verdict = comparison["verdict"].as_str().ok_or("missing verdict")?;
                        *counts.entry(verdict.into()).or_default() += 1;
                        models.push(json!({"parameters":model.parameters,"group_order":model.group_order,"g":model.g,"candidate_case":model.case_ref,"reference_case":other.case_ref,"comparison":comparison}));
                    }
                    result.push(json!({"prime":p,"base_size":b,"candidate":method,"reference":reference,"model_count":models.len(),"verdict_counts":counts,"models":models}));
                }
            }
        }
    }
    Ok(result)
}

fn receipt_files(entry: &Value) -> Result<Vec<Value>, String> {
    let mut result = Vec::new();
    for (key, kind) in [
        ("manifest_path", "receipt"),
        ("verification_path", "receipt"),
        ("execution_path", "host"),
        ("outcome_path", "receipt"),
        ("assigned_job_path", "host"),
    ] {
        if let Some(p) = optional(entry, key) {
            result.push(file(&p, kind)?);
        }
    }
    let path = PathBuf::from(entry["input_path"].as_str().ok_or("input path missing")?);
    if path.is_dir() {
        for name in ["events.jsonl", "controls.json", "active-case.json"] {
            let p = path.join(name);
            if p.is_file() {
                result.push(file(&p, "raw")?);
            }
        }
    }
    result.sort_by_key(|v| v["path"].as_str().unwrap_or("").to_string());
    result.dedup_by(|a, b| a["path"] == b["path"]);
    Ok(result)
}
fn attempted_arms(entry: &Value, complete: bool) -> Result<BTreeSet<String>, String> {
    if complete {
        return Ok(ARMS.into_iter().map(str::to_string).collect());
    }
    let path = PathBuf::from(
        entry["input_path"]
            .as_str()
            .ok_or("missing raw input path")?,
    );
    let mut arms = BTreeSet::new();
    if let Ok(events) = fs::read_to_string(path.join("events.jsonl")) {
        for line in events.lines() {
            if let Ok(event) = serde_json::from_str::<Value>(line) {
                if event["event"]["type"] == "case_started" {
                    if let Some(method) = event["event"]["method"].as_str() {
                        if METHODS.contains(&method) {
                            arms.insert(method.into());
                        }
                    }
                }
            }
        }
    }
    if path.join("controls.json").is_file() {
        arms.insert("planted_rank".into());
        arms.insert("b2_negative".into());
    }
    Ok(arms)
}
fn field_prime(entry: &Value) -> Option<u64> {
    metadata(entry, "manifest_path")["prime"]
        .as_u64()
        .or_else(|| metadata(entry, "execution_path")["prime"].as_u64())
        .filter(|p| PRIMES.contains(p))
}
fn row_metrics(summary: &Summary, p: u64, arm: &str) -> Result<MetricMaps, String> {
    let (mut counted, mut counters) = (Map::new(), Map::new());
    for cell in summary
        .cells
        .iter()
        .filter(|c| c.prime == p && c.method == arm)
    {
        for m in &cell.models {
            let prefix = format!(
                "B{}.a{}.b{}",
                cell.base_size, m.parameters.a, m.parameters.b
            );
            counted.insert(
                format!("{prefix}.denominator"),
                json!(m.aggregate.denominator.to_string()),
            );
            counted.insert(
                format!("{prefix}.selector_success"),
                json!(format!(
                    "{}/{}",
                    m.aggregate.selector_success_words,
                    m.aggregate
                        .selector_success_words
                        .checked_add(m.aggregate.selector_failure_words)
                        .ok_or("selector denominator overflow")?
                )),
            );
            for (key, value) in components(&m.aggregate)? {
                counted.insert(
                    format!("{prefix}.{key}"),
                    json!(format!("{value}/{}", m.aggregate.denominator)),
                );
            }
            for (key, value) in work_components(&m.aggregate)? {
                counted.insert(
                    format!("{prefix}.paid_work.{key}"),
                    json!(format!("{value}/{}", m.aggregate.denominator)),
                );
                counters.insert(
                    format!("{prefix}.{key}.numerator"),
                    json!(value.to_string()),
                );
            }
        }
    }
    Ok((counted, counters))
}

struct Records {
    runs: Vec<Value>,
    files: Vec<Value>,
    source: Value,
    backend: String,
    missing: Vec<String>,
    hosts: Vec<Value>,
}
fn records(index: &Value, summary: Option<&Summary>) -> Result<Records, String> {
    let entries = index["inputs"]
        .as_array()
        .ok_or("shard input index missing inputs")?;
    let (mut runs, mut files, mut commits, mut backends, mut hosts) = (
        Vec::new(),
        Vec::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        Vec::new(),
    );
    let mut covered = BTreeSet::new();
    for entry in entries {
        let Some(p) = field_prime(entry) else {
            continue;
        };
        if !covered.insert(p) {
            return Err("duplicate known field in shard metadata".into());
        }
        let manifest = metadata(entry, "manifest_path");
        let execution = metadata(entry, "execution_path");
        let outcome = metadata(entry, "outcome_path");
        let good = summary.is_some() && manifest["status"] == "complete";
        let actual = attempted_arms(entry, good)?;
        let artifacts = receipt_files(entry)?;
        files.extend(artifacts.clone());
        if let Some(commit) = manifest["source"]["working_tree_revision"].as_str() {
            commits.insert(commit.to_string());
        }
        if execution["backend"] == "isolab-k8s" && execution["runner_label"] == "crypto-eks-linux" {
            backends.insert("isolab-k8s");
        } else {
            backends.insert("manual");
        }
        if !execution.is_null() {
            hosts.push(execution.clone());
        }
        let path = PathBuf::from(entry["input_path"].as_str().ok_or("input path missing")?);
        let controls = load(&path.join("controls.json")).unwrap_or(Value::Null);
        let controls_pass = controls["schema"] == "twist-tr04/controls/v1"
            && controls["determinant_mod_order"] == 4
            && controls["audit"]["pair_table_entries"] == 3;
        for arm in actual {
            let is_control = arm == "planted_rank" || arm == "b2_negative";
            let succeeded = if is_control {
                controls_pass && good
            } else {
                good
            };
            let mut counted = Map::new();
            let mut counters = Map::new();
            if succeeded {
                if is_control {
                    counted.insert("controls_passed".into(), json!(1));
                } else if let Some(summary) = summary {
                    (counted, counters) = row_metrics(summary, p, &arm)?;
                }
            }
            let status = if succeeded {
                "succeeded"
            } else if manifest["status"] == "running" || manifest["finished_unix_utc"].is_null() {
                "interrupted"
            } else {
                "failed"
            };
            let excluded = !succeeded;
            runs.push(json!({"run_id":format!("p{p}/{arm}"),"arm":arm,"instance_id":format!("p{p}"),"seed":null,"replicate":0,"warmup":false,
                "status":status,"outcome_class":if succeeded{"completed"}else{"not_completed"},"attempts":1,"resumed":0,"checkpoint":{"policy":"none","generations_written":0,"restored_from_generation":null},
                "measurements":{"counted":counted,"wall_seconds":null,"wall_valid":false,"cpu_user_seconds":null,"cpu_sys_seconds":null,"max_rss_bytes":null,"counters":counters},
                "isolation":{"tier":if good{Some("L0")}else{None},"contended":null,"blockers":["No L2 admission or valid wall-clock measurement; exact counters only."]},
                "verification":{"status":if succeeded{"verified"}else{"not_run"},"verifier":if succeeded{Some("tr04_verify structural arithmetic/rank/census/digest; primitive work independently recomputed=false")}else{None}},
                "exclusion":{"excluded":excluded,"rule":if excluded{Some("incomplete, failed or independently unverified field shard")}else{None}},
                "artifacts":artifacts,"started_at":execution["started_at"],"finished_at":outcome["finished_at"],"worker":execution["runner_name"],"backend_ref":execution["job_url"]}));
        }
    }
    runs.sort_by_key(|v| v["run_id"].as_str().unwrap_or("").to_string());
    let actual = runs
        .iter()
        .filter_map(|r| r["run_id"].as_str())
        .collect::<BTreeSet<_>>();
    let missing = PRIMES
        .into_iter()
        .flat_map(|p| ARMS.into_iter().map(move |arm| format!("p{p}/{arm}")))
        .filter(|id| !actual.contains(id.as_str()))
        .collect();
    files.sort_by_key(|v| v["path"].as_str().unwrap_or("").to_string());
    files.dedup_by(|a, b| a["path"] == b["path"]);
    let commit = if commits.len() == 1 {
        commits.first().cloned()
    } else {
        None
    };
    let source = json!({"repo":"isogeny-labs/crypto","commit":commit,"dirty":null});
    let backend = if backends.len() == 1 {
        backends.first().unwrap_or(&"manual").to_string()
    } else if backends.is_empty() {
        "manual".into()
    } else {
        "mixed".into()
    };
    Ok(Records {
        runs,
        files,
        source,
        backend,
        missing,
        hosts,
    })
}
fn flow(runs: &[Value]) -> Value {
    let mut excluded = BTreeMap::<String, Vec<String>>::new();
    for row in runs {
        if row["exclusion"]["excluded"] == true {
            excluded
                .entry(
                    row["exclusion"]["rule"]
                        .as_str()
                        .unwrap_or("unspecified")
                        .into(),
                )
                .or_default()
                .push(row["run_id"].as_str().unwrap_or("").into());
        }
    }
    json!({"planned":runs.len(),"attempted":runs.len(),"completed":runs.iter().filter(|r|r["outcome_class"]=="completed").count(),"not_completed":runs.iter().filter(|r|r["outcome_class"]=="not_completed").count(),"interrupted":runs.iter().filter(|r|r["status"]=="interrupted").count(),"resumed":0,"excluded":excluded.into_iter().map(|(rule,run_ids)|json!({"rule":rule,"run_ids":run_ids})).collect::<Vec<_>>()})
}

fn xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
fn chart_label(row: &Value) -> String {
    format!(
        "p{} B{} {} / {}",
        row["prime"],
        row["base_size"],
        row["candidate"].as_str().unwrap_or("?"),
        row["reference"].as_str().unwrap_or("?")
    )
}
const CATEGORIES: [(&str, &str); 4] = [
    ("within_model_pareto", "#2166ac"),
    ("tie_within_model", "#bdbdbd"),
    ("worse_within_model", "#b2182b"),
    ("mixed_tradeoff", "#d8a21b"),
];
fn svg(rows: &[Value], id: &str, attempts: usize) -> String {
    if rows.is_empty() {
        return format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1040\" height=\"280\" viewBox=\"0 0 1040 280\"><rect width=\"1040\" height=\"280\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"25\" y=\"35\" font-size=\"20\">TR-04 incomplete execution: no comparable complete census</text><text x=\"25\" y=\"65\" font-size=\"14\">Export {} · {attempts} evidenced logical attempts; expected 27.</text><rect x=\"35\" y=\"110\" width=\"260\" height=\"65\" fill=\"#edf2f7\" stroke=\"#61738b\"/><text x=\"50\" y=\"148\">Retained actual input receipts</text><path d=\"M300 142 H375\" stroke=\"#61738b\"/><rect x=\"380\" y=\"110\" width=\"260\" height=\"65\" fill=\"#edf2f7\" stroke=\"#61738b\"/><text x=\"395\" y=\"148\">Missing/failed fields explicit</text><path d=\"M645 142 H715\" stroke=\"#61738b\"/><rect x=\"720\" y=\"110\" width=\"275\" height=\"65\" fill=\"#fff4e5\" stroke=\"#9c6b13\"/><text x=\"735\" y=\"148\">No rank/work comparison admitted</text><text x=\"25\" y=\"225\" font-size=\"13\">Diagram replaces quantitative comparisons: fewer than two comparable complete values.</text></g></svg>\n",xml(id));
    }
    let height = 140 + rows.len() * 30;
    let mut out=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1100\" height=\"{height}\" viewBox=\"0 0 1100 {height}\"><rect width=\"1100\" height=\"{height}\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"22\" y=\"30\" font-size=\"20\">TR-04 exact within-model rank/work classifications</text><text x=\"22\" y=\"53\" font-size=\"12\">{} · Unit: coefficient models; fields, B and L remain separate. No sampling interval.</text>",xml(id));
    for (i, (name, color)) in CATEGORIES.iter().enumerate() {
        let x = 22 + i * 265;
        out.push_str(&format!("<rect x=\"{x}\" y=\"70\" width=\"12\" height=\"12\" fill=\"{color}\"/><text x=\"{}\" y=\"81\" font-size=\"11\">{name}</text>",x+18));
    }
    for (i, row) in rows.iter().enumerate() {
        let y = 110 + i * 30;
        out.push_str(&format!(
            "<text x=\"22\" y=\"{}\" font-size=\"12\">{}</text>",
            y + 13,
            xml(&chart_label(row))
        ));
        let total = row["model_count"].as_u64().unwrap_or(0);
        let mut x = 380f64;
        for (name, color) in CATEGORIES {
            let n = row["verdict_counts"][name].as_u64().unwrap_or(0);
            let w = if total == 0 {
                0.0
            } else {
                550.0 * n as f64 / total as f64
            };
            out.push_str(&format!(
                "<rect x=\"{x:.3}\" y=\"{y}\" width=\"{w:.3}\" height=\"18\" fill=\"{color}\"/>",
            ));
            x += w;
        }
        out.push_str(&format!(
            "<text x=\"945\" y=\"{}\" font-size=\"11\">n={total}; {}/{}/{}/{}</text>",
            y + 13,
            row["verdict_counts"][CATEGORIES[0].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[1].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[2].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[3].0].as_u64().unwrap_or(0)
        ));
    }
    out.push_str(&format!("<text x=\"22\" y=\"{}\" font-size=\"11\">Source: comparisons.json. Every rank/prefix, total Work and every rank-milestone Work are compared exactly; physical runtime is unmeasured.</text></g></svg>\n",height-15));
    out
}
fn pdf_escape(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_ascii() { c } else { '?' })
        .collect::<String>()
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}
fn pdf_line(page: &mut String, x: u32, y: u32, size: u32, value: &str) {
    page.push_str(&format!(
        "BT /F1 {size} Tf {x} {y} Td ({}) Tj ET\n",
        pdf_escape(value)
    ));
}
fn wrap(value: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut line = String::new();
    for word in value.split_whitespace() {
        if line.len() + word.len() + 1 > width && !line.is_empty() {
            result.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ')
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        result.push(line);
    }
    result
}
fn pdf(pages: &[String]) -> Vec<u8> {
    let mut objects = Vec::new();
    objects.push("<< /Type /Catalog /Pages 2 0 R >>".to_string());
    objects.push(format!(
        "<< /Type /Pages /Kids [{}] /Count {} >>",
        (0..pages.len())
            .map(|i| format!("{} 0 R", 4 + i * 2))
            .collect::<Vec<_>>()
            .join(" "),
        pages.len()
    ));
    objects.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    for (i, page) in pages.iter().enumerate() {
        objects.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 1100 700] /Resources << /Font << /F1 3 0 R >> >> /Contents {} 0 R >>",5+i*2));
        objects.push(format!(
            "<< /Length {} >>\nstream\n{}endstream",
            page.len(),
            page
        ));
    }
    let mut out = b"%PDF-1.4\n".to_vec();
    let mut offsets = vec![0usize];
    for (i, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{}\nendobj\n", i + 1, object).as_bytes());
    }
    let start = out.len();
    out.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", objects.len() + 1).as_bytes(),
    );
    for offset in offsets.iter().skip(1) {
        out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{start}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    out
}
fn report_pdf(lines: &[String], rows: &[Value]) -> (Vec<u8>, String) {
    let mut pages = Vec::new();
    for chunk in lines.chunks(31) {
        let mut page = String::from("0.09 0.17 0.30 rg\n");
        for (i, line) in chunk.iter().enumerate() {
            pdf_line(
                &mut page,
                30,
                665 - i as u32 * 20,
                if i == 0 { 16 } else { 11 },
                line,
            );
        }
        pages.push(page);
    }
    for (index, chunk) in rows.chunks(15).enumerate() {
        let mut page = String::new();
        pdf_line(
            &mut page,
            25,
            670,
            19,
            "TR-04: exact classification counts by matched field/B/threshold/reference",
        );
        pdf_line(&mut page,25,645,11,&format!("Recorded/derived finite coefficient-model counts. Panel {}. Full exact components: comparisons.json",index+1));
        for (i, (name, color)) in CATEGORIES.iter().enumerate() {
            let hex = |start| {
                u8::from_str_radix(&color[start..start + 2], 16).unwrap_or(0) as f64 / 255.0
            };
            page.push_str(&format!(
                "{:.4} {:.4} {:.4} rg {} 615 12 12 re f\n",
                hex(1),
                hex(3),
                hex(5),
                25 + i * 265
            ));
            page.push_str("0 0 0 rg\n");
            pdf_line(&mut page, 45 + i as u32 * 265, 616, 10, name);
        }
        for (i, row) in chunk.iter().enumerate() {
            let y = 570 - i as u32 * 30;
            pdf_line(&mut page, 25, y + 5, 11, &chart_label(row));
            let total = row["model_count"].as_u64().unwrap_or(0);
            let mut x = 380.0;
            for (name, color) in CATEGORIES {
                let n = row["verdict_counts"][name].as_u64().unwrap_or(0);
                let width = if total == 0 {
                    0.0
                } else {
                    520.0 * n as f64 / total as f64
                };
                let hex = |start| {
                    u8::from_str_radix(&color[start..start + 2], 16).unwrap_or(0) as f64 / 255.0
                };
                page.push_str(&format!(
                    "{:.4} {:.4} {:.4} rg {x:.3} {y} {width:.3} 18 re f\n",
                    hex(1),
                    hex(3),
                    hex(5)
                ));
                x += width;
            }
            page.push_str("0 0 0 rg\n");
            pdf_line(
                &mut page,
                920,
                y + 5,
                10,
                &format!(
                    "n={total}: {}/{}/{}/{}",
                    row["verdict_counts"][CATEGORIES[0].0].as_u64().unwrap_or(0),
                    row["verdict_counts"][CATEGORIES[1].0].as_u64().unwrap_or(0),
                    row["verdict_counts"][CATEGORIES[2].0].as_u64().unwrap_or(0),
                    row["verdict_counts"][CATEGORIES[3].0].as_u64().unwrap_or(0)
                ),
            );
        }
        pdf_line(&mut page,25,60,11,"No probability/work pooling, statistical uncertainty or physical-runtime/speedup inference.");
        pages.push(page);
    }
    if rows.is_empty() {
        let mut page = String::new();
        pdf_line(&mut page, 30, 650, 20, "Incomplete execution diagram");
        for (x, label) in [
            (40, "Retained actual input receipts"),
            (400, "Missing/failed fields explicit"),
            (760, "No comparison admitted"),
        ] {
            page.push_str(&format!(
                "0.94 0.96 0.98 rg {x} 430 300 90 re f\n0 0 0 rg\n"
            ));
            pdf_line(&mut page, x + 15, 470, 13, label);
        }
        pdf_line(
            &mut page,
            30,
            370,
            13,
            "Fewer than two comparable complete values: quantitative comparison is unavailable.",
        );
        pages.push(page);
    }
    for page in &mut pages {
        page.push_str("0.2 0.2 0.2 rg\n");
        if let Some(identity) = lines.iter().find(|line| line.starts_with("Export ")) {
            pdf_line(page, 25, 18, 9, identity);
        }
        if let Some(fixture) = lines
            .iter()
            .find(|line| line.starts_with("SYNTHETIC DEVELOPMENT FIXTURE"))
        {
            pdf_line(page, 25, 32, 9, fixture);
        }
    }
    let source = pages
        .iter()
        .enumerate()
        .map(|(i, p)| format!("% Page {}\n{p}", i + 1))
        .collect::<Vec<_>>()
        .join("\n");
    (pdf(&pages), source)
}

pub fn cli(args: Vec<String>) -> Result<(), String> {
    if args.iter().any(|s| s == "--help" || s == "-h") {
        println!("tr04-report --input SUMMARY_DIR --protocol PROTOCOL.json --output-dir FRESH_DIR\nReads retained records only; partial reports/export return nonzero. PDF inspection remains an external publication check.");
        return Ok(());
    }
    let (mut input, mut protocol, mut output) = (None, None, None);
    let mut at = 0;
    while at < args.len() {
        let flag = &args[at];
        let value = args.get(at + 1).ok_or("missing report argument value")?;
        match flag.as_str() {
            "--input" if input.is_none() => input = Some(PathBuf::from(value)),
            "--protocol" if protocol.is_none() => protocol = Some(PathBuf::from(value)),
            "--output-dir" if output.is_none() => output = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown/duplicate report argument {flag}")),
        }
        at += 2;
    }
    run(
        &input.ok_or("--input required")?,
        &protocol.ok_or("--protocol required")?,
        &output.ok_or("--output-dir required")?,
    )
}
fn run(input: &Path, protocol_path: &Path, out: &Path) -> Result<(), String> {
    fs::create_dir(out)
        .map_err(|e| format!("fresh report output {} required: {e}", out.display()))?;
    let result = generate(input, protocol_path, out);
    write(
        &out.join("report-status.json"),
        &json!({"schema":"twist-tr04/report-status/v1","status":if result.is_ok(){"complete"}else{"incomplete_or_error"},"error":result.as_ref().err(),"pdf_inspection":"External inspection required before publication; generation is not inspection.","work_counters_independent":false,"wall_valid":false}),
    )?;
    result
}
fn portable_files(export: &mut Value, out: &Path) -> Result<(), String> {
    fn visit(
        value: &mut Value,
        root: &Path,
        provenance: &mut BTreeMap<String, Value>,
    ) -> Result<(), String> {
        if value.is_object()
            && value.get("sha256").is_some()
            && value.get("bytes").is_some()
            && value.get("path").is_some()
        {
            let original =
                PathBuf::from(value["path"].as_str().ok_or("file path is not a string")?);
            let original = fs::canonicalize(&original).map_err(|e| e.to_string())?;
            let bytes = fs::read(&original).map_err(|e| e.to_string())?;
            if value["sha256"] != hash(&bytes)
                || value["bytes"].as_u64() != Some(bytes.len() as u64)
            {
                return Err("source file changed during report packaging".into());
            }
            let relative = if let Ok(local) = original.strip_prefix(root) {
                local.to_path_buf()
            } else {
                let name = original
                    .file_name()
                    .and_then(|v| v.to_str())
                    .ok_or("source file has no UTF-8 name")?;
                let relative =
                    PathBuf::from("source-records").join(format!("{}-{name}", hash(&bytes)));
                let target = root.join(&relative);
                if target.exists() {
                    if fs::read(&target).map_err(|e| e.to_string())? != bytes {
                        return Err("portable source filename collision".into());
                    }
                } else {
                    fs::write(&target, &bytes).map_err(|e| e.to_string())?;
                }
                provenance.insert(original.display().to_string(),json!({"original_path":original,"copied_path":relative,"sha256":hash(&bytes),"bytes":bytes.len()}));
                relative
            };
            value["path"] = json!(relative);
            return Ok(());
        }
        if let Some(values) = value.as_array_mut() {
            for child in values {
                visit(child, root, provenance)?;
            }
        }
        if let Some(values) = value.as_object_mut() {
            for child in values.values_mut() {
                visit(child, root, provenance)?;
            }
        }
        Ok(())
    }
    let root = fs::canonicalize(out).map_err(|e| e.to_string())?;
    fs::create_dir(root.join("source-records")).map_err(|e| e.to_string())?;
    let original_protocol = export["protocol"]["path"]
        .as_str()
        .ok_or("export protocol path missing")?
        .to_string();
    let mut provenance = BTreeMap::new();
    visit(export, &root, &mut provenance)?;
    let copied = provenance
        .get(&original_protocol)
        .ok_or("protocol not retained in portable files")?;
    export["protocol"]["path"] = copied["copied_path"].clone();
    let path = root.join("source-records/provenance.json");
    write(
        &path,
        &json!({"schema":"twist-tr04/portable-source-provenance/v1","copies":provenance.values().collect::<Vec<_>>(),"raw_scope":"Compact source receipts, controls and summary are copied byte-for-byte. Original manifests retain raw case/cache references and hashes; full raw artifacts remain separate and require an explicit relocation map when downloaded."}),
    )?;
    let mut receipt = file(&path, "receipt")?;
    receipt["path"] = json!("source-records/provenance.json");
    export["files"]
        .as_array_mut()
        .ok_or("export files missing")?
        .push(receipt);
    Ok(())
}
fn generate(input: &Path, protocol_path: &Path, out: &Path) -> Result<(), String> {
    let protocol = load(protocol_path)?;
    crate::tr04_protocol::validate(protocol_path, true)?;
    let index = load(&input.join("shard-inputs.json"))?;
    if index["schema"] != "twist-tr04/shard-input-index/v1" {
        return Err("incompatible shard input index".into());
    }
    let summary_result = check_summary(input, &protocol, &index);
    let issue = summary_result.as_ref().err().cloned();
    let summary = summary_result.ok();
    let comparisons = summary
        .as_ref()
        .map(comparisons)
        .transpose()?
        .unwrap_or_default();
    let mut records = records(&index, summary.as_ref())?;
    if summary.is_some() {
        let summary_file = file(&input.join("summary.json"), "summary")?;
        for row in &mut records.runs {
            if METHODS.contains(&row["arm"].as_str().unwrap_or("")) {
                row["artifacts"]
                    .as_array_mut()
                    .ok_or("row artifact list missing")?
                    .push(summary_file.clone());
            }
        }
    }
    let complete = summary.is_some()
        && records.runs.len() == 27
        && records.runs.iter().all(|r| r["status"] == "succeeded")
        && records.missing.is_empty();
    let now = utc()?;
    let export_id = format!(
        "TR04-{}",
        now.chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .collect::<String>()
    );
    let fl = flow(&records.runs);
    let analysis = json!({"schema":"twist-tr04/analysis/v1","status":if complete{"complete"}else{"incomplete"},"comparison_scope":"Every rank at every prefix; total unconditional Work and every rank-milestone unconditional Work, independently by coefficient model/B/L/reference. Incomplete selectors retain rank0 and their paid work.","work_counters_independent":false,"wall_valid":false,"comparisons":comparisons,"source_summary":if summary.is_some(){Some(file(&input.join("summary.json"),"summary")?)}else{None},"summary_error":issue});
    write(&out.join("comparisons.json"), &analysis)?;
    text(
        &out.join("recorded-values.svg"),
        &svg(&comparisons, &export_id, records.runs.len()),
    )?;
    write(
        &out.join("graph-data.json"),
        &json!({"export_id":export_id,"unit":"coefficient models","uncertainty":"none_exact","data":comparisons.iter().map(|v|json!({"prime":v["prime"],"base_size":v["base_size"],"candidate":v["candidate"],"reference":v["reference"],"model_count":v["model_count"],"verdict_counts":v["verdict_counts"]})).collect::<Vec<_>>()}),
    )?;
    let mut markdown=format!("# TR-04 retained-run evidence\n\nExport `{export_id}`; generated {now}. Status: **{}**.\n\n{}\n\nRequested scope is all531 coefficient-model dispositions across p7,p11,p19, B2/B3, seven selectors, every declared finite candidate/query word and six actual auxiliary control receipts. A complete table has27 logical rows and42 selector/B cells.\n\nCanonical flow (actual evidenced rows): planned {}, attempted {}, completed {}, not completed {}, interrupted {}, resumed {}. Protocol-expected rows:27. Missing or unattempted IDs: `{}`. Canonical flow planned means actual exported rows; absent attempted evidence is never invented.\n\n[Recorded quantitative graph/partial diagram](recorded-values.svg), [graph source data](graph-data.json), [full exact comparisons](comparisons.json), [PDF](report.pdf), [native PDF drawing source](report.pdf-commands.txt), [canonical export](export.json). PDF publication requires external visual inspection.\n\nComparisons use every rank at every prefix and every component of total and rank-milestone unconditional Work, keeping coefficient models, B values and thresholds separate. Pareto/tie/worse/mixed labels describe these declared components only. The progression arm changes the candidate-word family and is contextual.\n\nThe exact model values, phase counters, attained/missing work split and denominators remain in the hashed raw case records and summary, not a pooled scalar average. No sample confidence interval applies to an exhaustive finite population.\n",if complete{"complete finite census, structural verification passed"}else{"incomplete; no finite-census comparison accepted"},LIMITS,fl["planned"],fl["attempted"],fl["completed"],fl["not_completed"],fl["interrupted"],fl["resumed"],records.missing.join(", "));
    if protocol["extensions"]["development_fixture"] == true {
        markdown.insert_str(0,"**SYNTHETIC DEVELOPMENT FIXTURE. These records are manufactured software-test inputs, not a research run.**\n\n");
    }
    if let Some(reason) = &issue {
        markdown.push_str(&format!("\nSummary failure: `{}`. Partial attempts and raw artifacts remain listed in export.json.\n",reason.replace('`',"'")));
    }
    markdown.push_str("\n| Field/B/candidate/reference | Models | Within-model Pareto | Tie | Worse | Mixed |\n|---|---:|---:|---:|---:|---:|\n");
    for row in &comparisons {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            chart_label(row),
            row["model_count"],
            row["verdict_counts"][CATEGORIES[0].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[1].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[2].0].as_u64().unwrap_or(0),
            row["verdict_counts"][CATEGORIES[3].0].as_u64().unwrap_or(0)
        ));
    }
    markdown.push_str("\nEvidence/source identities and every raw file SHA-256/byte count appear in export.json. SEC1 v2 section2.2.1 defines the underlying short-Weierstrass arithmetic; prior TR01-03 evidence is separate. No leaderboard promotion or physical speed figure is admitted.\n");
    text(&out.join("REPORT.md"), &markdown)?;
    let mut pdf_lines=vec!["TR-04 exhaustive bounded relation finding: retained evidence".into(),format!("Export {export_id}; generated {now}"),format!("Status: {}; actual rows {} of27",if complete{"complete finite census"}else{"incomplete"},records.runs.len()),format!("Flow: planned {}; attempted {}; completed {}; not_completed {}; interrupted {}; resumed {}",fl["planned"],fl["attempted"],fl["completed"],fl["not_completed"],fl["interrupted"],fl["resumed"]),format!("Protocol {} v{}; frozen SHA256 {}",protocol["protocol_id"].as_str().unwrap_or("?"),protocol["version"],protocol["freeze"]["protocol_sha256"].as_str().unwrap_or("?"))];
    if protocol["extensions"]["development_fixture"] == true {
        pdf_lines.insert(1,"SYNTHETIC DEVELOPMENT FIXTURE: manufactured test inputs; no scientific execution or result.".into());
    }
    pdf_lines.extend(wrap(LIMITS, 145));
    pdf_lines.extend(wrap("Domain: every coefficient model over p7,p11,p19 (531 dispositions); eligible nonsingular prime order >=7; canonical G; B2/B3; length8 candidate words and length8 binary G/Q query words; all nonzero targets. Six actual field control receipts required.",145));
    pdf_lines.extend(wrap("Comparison scope: every rank at every prefix, total unconditional Work and every milestone unconditional Work. Separate exact coefficient-model decisions; counts below aggregate classification labels only, never probability or cost values.",145));
    pdf_lines.extend(wrap("Sources: summary.json, shard-inputs.json, raw model/case/relation records and independent verification receipts; paths, identities, byte sizes and SHA256 digests in export.json. Full exact contrast numerators/denominators: comparisons.json. Editable graph source: graph-data.json. SEC1 v2 section2.2.1. Prior TR01-03 evidence remains separate.",145));
    if !records.missing.is_empty() {
        pdf_lines.extend(wrap(
            &format!("Missing/unattempted IDs: {}", records.missing.join(", ")),
            145,
        ));
    }
    if let Some(error) = &issue {
        pdf_lines.extend(wrap(&format!("Summary failure: {error}"), 145));
    }
    let (pdf_bytes, pdf_source) = report_pdf(&pdf_lines, &comparisons);
    fs::write(out.join("report.pdf"), pdf_bytes).map_err(|e| e.to_string())?;
    text(&out.join("report.pdf-commands.txt"), &pdf_source)?;
    let outputs = [
        ("comparisons.json", "analysis"),
        ("graph-data.json", "analysis"),
        ("recorded-values.svg", "report"),
        ("REPORT.md", "report"),
        ("report.pdf", "report"),
        ("report.pdf-commands.txt", "analysis"),
    ]
    .into_iter()
    .map(|(p, k)| file(&out.join(p), k))
    .collect::<Result<Vec<_>, _>>()?;
    let analysis_path = protocol_path
        .parent()
        .ok_or("protocol parent missing")?
        .join(
            protocol["analysis_plan"]["script"]["path"]
                .as_str()
                .ok_or("analysis script missing")?,
        );
    let script = file(&analysis_path, "analysis")?;
    if script["sha256"] != protocol["analysis_plan"]["script"]["sha256"] {
        return Err("analysis script differs from frozen protocol".into());
    }
    let mut all_files = records.files.clone();
    all_files.push(file(protocol_path, "protocol")?);
    for name in ["manifest.json", "shard-inputs.json", "summary.json"] {
        let p = input.join(name);
        if p.is_file() {
            all_files.push(file(&p, "summary")?);
        }
    }
    let arms=protocol["arms"].as_array().ok_or("protocol arms missing")?.iter().map(|arm|{
        let pins=index["inputs"].as_array().into_iter().flatten().filter_map(|e|metadata(e,"manifest_path")["source"]["binary_sha256"].as_str().map(str::to_string)).collect::<BTreeSet<_>>();
        json!({"id":arm["id"],"role":arm["role"],"binary_sha256":if pins.len()==1{pins.first().cloned()}else{None},"commit":records.source["commit"]})
    }).collect::<Vec<_>>();
    let mut export = json!({"schema":"experiment.export/v1","export_id":export_id,"created":now,"protocol":{"path":fs::canonicalize(protocol_path).map_err(|e|e.to_string())?,"sha256":protocol["freeze"]["protocol_sha256"],"protocol_id":protocol["protocol_id"],"version":protocol["version"]},"producer":{"runtime":"native-rust","model":null,"session":null,"role":"reporter"},"source":records.source,"backend":records.backend,"host":{"path":null,"sha256":null,"class_id":null,"summary":{"cpu_model":null,"logical_cpus":null,"memory_bytes":null,"kernel":null,"hypervisor":null,"isolation_tier_max":null}},"arms":arms,"runs":records.runs,"flow":fl,"files":all_files,"analysis":{"script":script,"blinded":false,"blinding_key_sha256":null,"outputs":outputs,"unmasked_at":null},"notes":[{"kind":"observation","text":LIMITS,"by":"tr04-report"},{"kind":"observation","text":format!("Protocol expects27 rows; missing/unattempted IDs: {}. Counter independence=false; wall_valid=false. PDF generated; external visual inspection required.",records.missing.join(", ")),"by":"tr04-report"}],"reviews_dir":"reviews","backend_record":{"protocol_expected_rows":27,"actual_rows":fl["planned"],"missing_run_ids":records.missing,"execution_metadata":records.hosts,"summary_error":issue,"finite_census_complete":complete,"raw_paths":"Portable listed files are retained under source-records. Full raw case/cache artifacts remain separate; original manifest paths/hashes and relocation provenance are retained."}});
    portable_files(&mut export, out)?;
    if protocol["extensions"]["development_fixture"] == true {
        export["notes"].as_array_mut().ok_or("notes array missing")?.insert(0,json!({"kind":"observation","text":"SYNTHETIC DEVELOPMENT FIXTURE: every receipt and attempted-row marker is manufactured to test software behavior; no research run or result.","by":"tr04-report"}));
    }
    write(&out.join("export.json"), &export)?;
    if complete {
        Ok(())
    } else {
        Err("TR-04 census incomplete: report/export retain only evidenced attempts and no complete-census verdict".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn synthetic_combined(denominator: u128, success: u128, cost_sum: u128) -> Combined {
        let work = crate::tr04_select::Work {
            candidate_reads: cost_sum,
            ..Default::default()
        };
        let attained = crate::tr04_select::Work {
            candidate_reads: cost_sum * success / denominator,
            ..Default::default()
        };
        let missing = crate::tr04_select::Work {
            candidate_reads: cost_sum - attained.candidate_reads,
            ..Default::default()
        };
        Combined {
            denominator,
            selector_success_words: denominator,
            selector_failure_words: 0,
            rank_hist_by_prefix: vec![vec![denominator, 0], vec![denominator - success, success]],
            hits_sum_by_prefix: vec![0, success],
            dependent_hits_sum_by_prefix: vec![0, 0],
            full_rank_observations: success,
            budget_exhausted_observations: denominator - success,
            selection_work_sum: work.clone(),
            pair_setup_work_sum: Default::default(),
            search_work_sum: Default::default(),
            linear_work_sum: Default::default(),
            verification_work_sum: Default::default(),
            total_work_sum: work.clone(),
            milestones: vec![crate::tr04_relation::RankMilestone {
                rank: 1,
                attained: success,
                missing: denominator - success,
                first_prefix_hist: BTreeMap::from([(1, success)]),
                work_sum: work,
                attained_work_sum: attained,
                missing_work_sum: missing,
            }],
        }
    }
    #[test]
    fn exact_unconditional_means_and_rank_components_preserve_tradeoffs() {
        let reference = synthetic_combined(10, 5, 100);
        assert_eq!(
            contrast(&synthetic_combined(20, 10, 200), &reference).unwrap()["verdict"],
            "tie_within_model"
        );
        assert_eq!(
            contrast(&synthetic_combined(10, 8, 110), &reference).unwrap()["verdict"],
            "mixed_tradeoff"
        );
        assert_eq!(
            contrast(&synthetic_combined(10, 8, 90), &reference).unwrap()["verdict"],
            "within_model_pareto"
        );
        assert_eq!(
            contrast(&synthetic_combined(10, 4, 110), &reference).unwrap()["verdict"],
            "worse_within_model"
        );
        let result = contrast(&synthetic_combined(10, 0, 90), &reference).unwrap();
        assert_eq!(
            result["components"]["rank_survival"]["rank.prefix1.rank1"]["candidate_denominator"],
            "10"
        );
        assert_eq!(result["verdict"], "mixed_tradeoff");
    }
    fn scratch(name: &str) -> PathBuf {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("tr04-report-{name}-{}-{id}", std::process::id()));
        fs::create_dir(&path).unwrap();
        path
    }
    #[test]
    fn indexed_same_prime_receipt_substitution_cannot_rebind_summary_sources() {
        let root = scratch("crossbinding");
        let mut entries = Vec::new();
        let mut shards = Vec::new();
        for p in PRIMES {
            let path = root.join(format!("p{p}"));
            fs::create_dir(&path).unwrap();
            write(
                &path.join("manifest.json"),
                &json!({"prime":p,"status":"complete","source":"accepted synthetic receipt"}),
            )
            .unwrap();
            write(
                &path.join("verification.json"),
                &json!({"prime":p,"status":"passed","fixture":true}),
            )
            .unwrap();
            entries.push(json!({"input_path":path,"manifest_path":path.join("manifest.json"),"verification_path":path.join("verification.json")}));
            shards.push(json!({"prime":p,"manifest_sha256":file(&path.join("manifest.json"),"receipt").unwrap()["sha256"],"verification_sha256":file(&path.join("verification.json"),"receipt").unwrap()["sha256"]}));
        }
        let index = json!({"inputs":entries});
        assert!(bind_input_receipts(&index, &shards).is_ok());
        let other = root.join("other-p7");
        fs::create_dir(&other).unwrap();
        write(&other.join("manifest.json"),&json!({"prime":7,"status":"complete","source":"substituted same-prime synthetic receipt"})).unwrap();
        fs::copy(
            root.join("p7/verification.json"),
            other.join("verification.json"),
        )
        .unwrap();
        let mut substituted = index.clone();
        substituted["inputs"][0] = json!({"input_path":other,"manifest_path":other.join("manifest.json"),"verification_path":other.join("verification.json")});
        assert!(bind_input_receipts(&substituted, &shards)
            .unwrap_err()
            .contains("differ from the accepted summary"));
        fs::copy(root.join("p7/manifest.json"), other.join("manifest.json")).unwrap();
        write(
            &other.join("verification.json"),
            &json!({"prime":7,"status":"passed","fixture":"substituted verification"}),
        )
        .unwrap();
        assert!(bind_input_receipts(&substituted, &shards).is_err());
        let mut disconnected = index.clone();
        disconnected["inputs"][0]["input_path"] = json!(other);
        assert!(bind_input_receipts(&disconnected, &shards).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn partial_receipts_do_not_invent_attempts_and_portable_files_survive_relocation() {
        let root = scratch("partial");
        let raw = root.join("raw");
        let out = root.join("report");
        fs::create_dir(&raw).unwrap();
        fs::create_dir(&out).unwrap();
        write(&raw.join("manifest.json"),&json!({"prime":7,"status":"failed","finished_unix_utc":1,"source":{"working_tree_revision":"1111111111111111111111111111111111111111"}})).unwrap();
        text(
            &raw.join("events.jsonl"),
            "{\"event\":{\"type\":\"case_started\",\"method\":\"random\"}}\n",
        )
        .unwrap();
        let index = json!({"inputs":[{"input_path":raw,"manifest_path":raw.join("manifest.json")},{"input_path":root.join("missing-p11")},{"input_path":root.join("missing-p19")}]});
        let records = records(&index, None).unwrap();
        assert_eq!(records.runs.len(), 1);
        assert_eq!(records.runs[0]["run_id"], "p7/random");
        assert_eq!(records.missing.len(), 26);
        assert_eq!(flow(&records.runs)["not_completed"], 1);
        let protocol = raw.join("protocol.json");
        text(&protocol, "{\"fixture\":true}\n").unwrap();
        let mut export = json!({"protocol":{"path":protocol},"files":[file(&protocol,"protocol").unwrap()],"runs":records.runs,"analysis":{"outputs":[]}});
        portable_files(&mut export, &out).unwrap();
        fs::remove_dir_all(&raw).unwrap();
        for item in export["files"]
            .as_array()
            .unwrap()
            .iter()
            .chain(export["runs"][0]["artifacts"].as_array().unwrap())
        {
            let path = Path::new(item["path"].as_str().unwrap());
            assert!(!path.is_absolute());
            let bytes = fs::read(out.join(path)).unwrap();
            assert_eq!(hash(&bytes), item["sha256"]);
            assert_eq!(bytes.len() as u64, item["bytes"].as_u64().unwrap());
        }
        assert!(out
            .join(export["protocol"]["path"].as_str().unwrap())
            .is_file());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn rational_comparison_avoids_cross_product_overflow() {
        assert_eq!(
            rational_cmp(u128::MAX, u128::MAX - 1, u128::MAX - 1, u128::MAX - 2).unwrap(),
            Ordering::Less
        );
        assert_eq!(rational_cmp(2, 3, 4, 6).unwrap(), Ordering::Equal);
        assert_eq!(rational_cmp(0, 1, 1, u128::MAX).unwrap(), Ordering::Less);
        assert!(rational_cmp(1, 0, 1, 2).is_err());
    }
    #[test]
    fn rational_comparison_matches_safe_cross_products() {
        for a in 0..25 {
            for b in 1..25 {
                for c in 0..25 {
                    for d in 1..25 {
                        assert_eq!(rational_cmp(a, b, c, d).unwrap(), (a * d).cmp(&(b * c)));
                    }
                }
            }
        }
    }
    #[test]
    fn flow_counts_only_actual_rows_and_retains_failure() {
        let rows = vec![
            json!({"run_id":"p7/random","status":"failed","outcome_class":"not_completed","exclusion":{"excluded":true,"rule":"missing shard"}}),
        ];
        let value = flow(&rows);
        assert_eq!(value["planned"], 1);
        assert_eq!(value["attempted"], 1);
        assert_eq!(value["not_completed"], 1);
        assert_eq!(value["completed"], 0);
        assert_eq!(value["excluded"][0]["run_ids"][0], "p7/random");
    }
    #[test]
    fn native_pdf_has_valid_offsets_and_zero_data_diagram() {
        let (pdf, source) = report_pdf(&["Fixture, no scientific data".into()], &[]);
        let body = String::from_utf8(pdf).unwrap();
        assert!(body.starts_with("%PDF-1.4"));
        assert!(body.ends_with("%%EOF\n"));
        assert!(body.contains("/Count 2"));
        assert!(source.contains("Incomplete execution diagram"));
        assert!(svg(&[], "fixture", 0).contains("expected 27"));
    }
}
