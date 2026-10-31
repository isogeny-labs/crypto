use anyhow::{anyhow, bail, ensure, Context, Result};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::io::{Cursor, Read};
use std::path::Path;
use std::process::Command;

pub mod disjoint_enrollment;

pub const ARCHIVE_SHA256: &str = "550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339";
pub const BITS: [u64; 7] = [16, 20, 24, 28, 32, 36, 40];
const EXPECTED_ARCHIVE_ROWS: usize = 1260;

fn sha256(data: &[u8]) -> String {
    let digest = Sha256::digest(data);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn ordered(pairs: impl IntoIterator<Item = (&'static str, Value)>) -> Value {
    let mut map = Map::new();
    for (key, value) in pairs {
        map.insert(key.to_owned(), value);
    }
    Value::Object(map)
}

fn write_pretty(path: &Path, value: &Value) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

fn object_mut<'a>(value: &'a mut Value, name: &str) -> Result<&'a mut Map<String, Value>> {
    value
        .as_object_mut()
        .ok_or_else(|| anyhow!("{name} must be a JSON object"))
}

fn object<'a>(value: &'a Value, name: &str) -> Result<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow!("{name} must be a JSON object"))
}

fn array<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    value
        .as_array()
        .ok_or_else(|| anyhow!("{name} must be a JSON array"))
}

fn u64_field(value: &Value, name: &str) -> Result<u64> {
    value
        .get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| anyhow!("missing integer {name}"))
}

fn string_field<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing string {name}"))
}

fn normalized_seed(value: &Value) -> Result<String> {
    match value {
        Value::String(seed) => Ok(seed.clone()),
        Value::Number(seed) => Ok(seed.to_string()),
        _ => bail!("algorithm_seed must be an integer or decimal string"),
    }
}

fn verify_manifest(directory: &Path) -> Result<Value> {
    let manifest_path = directory.join("manifest.json");
    let manifest: Value = serde_json::from_slice(
        &fs::read(&manifest_path).with_context(|| format!("read {}", manifest_path.display()))?,
    )?;
    for (name, expected) in object(
        manifest
            .get("files")
            .ok_or_else(|| anyhow!("manifest has no files"))?,
        "manifest.files",
    )? {
        let expected = expected
            .as_str()
            .ok_or_else(|| anyhow!("manifest digest for {name} is not a string"))?;
        let path = directory.join(name);
        let actual = sha256(&fs::read(&path).with_context(|| format!("read {}", path.display()))?);
        ensure!(actual == expected, "input hash mismatch: {name}");
    }
    Ok(manifest)
}

fn create_output(path: &Path) -> Result<()> {
    fs::create_dir(path).with_context(|| format!("create new output directory {}", path.display()))
}

pub fn prepare_archive(archive: &Path, out: &Path) -> Result<()> {
    prepare_archive_with_contract(archive, out, ARCHIVE_SHA256, EXPECTED_ARCHIVE_ROWS)
}

fn prepare_archive_with_contract(
    archive: &Path,
    out: &Path,
    expected_archive_sha256: &str,
    expected_rows: usize,
) -> Result<()> {
    let compressed = fs::read(archive).with_context(|| format!("read {}", archive.display()))?;
    let digest = sha256(&compressed);
    ensure!(
        digest == expected_archive_sha256,
        "unexpected frozen archive hash: {digest}"
    );

    let decoder = zstd::stream::read::Decoder::new(Cursor::new(&compressed))?;
    let mut tar = tar::Archive::new(decoder);
    let mut entries = BTreeMap::<String, Vec<u8>>::new();
    for entry in tar.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_string_lossy().into_owned();
        let wanted = BITS.iter().any(|bits| {
            let prefix = format!("sessions_projective_chart_v2/order-genus-{bits}/");
            path == format!("{prefix}spec.json") || path == format!("{prefix}records.jsonl")
        });
        if wanted {
            let mut data = Vec::new();
            entry.read_to_end(&mut data)?;
            ensure!(
                entries.insert(path.clone(), data).is_none(),
                "duplicate archive member {path}"
            );
        }
    }

    let mut payloads = BTreeMap::<String, Vec<u8>>::new();
    let mut rows = Vec::<Value>::new();
    let mut workload_ids = BTreeSet::<String>::new();
    for bits in BITS {
        let prefix = format!("sessions_projective_chart_v2/order-genus-{bits}");
        let spec = entries
            .remove(&format!("{prefix}/spec.json"))
            .ok_or_else(|| anyhow!("archive is missing {prefix}/spec.json"))?;
        payloads.insert(format!("archive-spec-{bits}.json"), spec);
        let records = entries
            .remove(&format!("{prefix}/records.jsonl"))
            .ok_or_else(|| anyhow!("archive is missing {prefix}/records.jsonl"))?;
        for (line_number, line) in records.split(|byte| *byte == b'\n').enumerate() {
            if line.is_empty() {
                continue;
            }
            let record: Value = serde_json::from_slice(line).with_context(|| {
                format!("parse records line {} for {bits} bits", line_number + 1)
            })?;
            if record.get("arm").and_then(Value::as_str) != Some("genus_projective") {
                continue;
            }
            let workload = record
                .get("workload")
                .ok_or_else(|| anyhow!("archive record has no workload"))?;
            let workload_id = string_field(workload, "workload_id")?.to_owned();
            ensure!(
                workload_ids.insert(workload_id.clone()),
                "duplicate workload_id {workload_id}"
            );
            rows.push(ordered([
                ("bits", json!(bits)),
                ("workload_id", json!(workload_id)),
                (
                    "target",
                    workload
                        .get("target")
                        .cloned()
                        .ok_or_else(|| anyhow!("workload has no target"))?,
                ),
                ("target_seed", json!(u64_field(workload, "target_seed")?)),
                ("target_index", json!(u64_field(workload, "target_index")?)),
                (
                    "algorithm_seed",
                    record
                        .get("algorithm_seed")
                        .cloned()
                        .ok_or_else(|| anyhow!("record has no algorithm_seed"))?,
                ),
            ]));
        }
    }
    ensure!(
        rows.len() == expected_rows && workload_ids.len() == expected_rows,
        "archive cohort inventory mismatch: rows={}, unique_workloads={}",
        rows.len(),
        workload_ids.len()
    );

    let mut jsonl = Vec::new();
    for row in &rows {
        serde_json::to_writer(&mut jsonl, row)?;
        jsonl.push(b'\n');
    }
    payloads.insert("expected-workloads.jsonl".to_owned(), jsonl);
    create_output(out)?;
    for (name, data) in &payloads {
        fs::write(out.join(name), data)?;
    }
    let files = payloads
        .iter()
        .map(|(name, data)| (name.clone(), Value::String(sha256(data))))
        .collect::<Map<_, _>>();
    let manifest = ordered([
        ("schema", json!("pencil-archive-input-validation/v1")),
        (
            "status",
            json!("expected_inputs_extracted_native_parity_pending"),
        ),
        ("source_archive_sha256", json!(digest)),
        ("rows", json!(rows.len())),
        (
            "purpose",
            json!("Run-free native plan comparison only. Archived specs are not the new execution protocol."),
        ),
        ("files", Value::Object(files)),
        (
            "projection",
            json!("Archived workload_id, target, target_seed, target_index, algorithm_seed and size only; no planted answer."),
        ),
    ]);
    write_pretty(&out.join("manifest.json"), &manifest)
}

fn candidate_arm() -> Value {
    ordered([
        ("name", json!("pencil")),
        ("role", json!("candidate")),
        (
            "method",
            ordered([
                ("id", json!("ic.three_leads_genus2_pencil_symbolic_v2")),
                (
                    "params",
                    ordered([
                        ("small_denominator", json!("8")),
                        ("matrix_coefficient_cap", json!("25000000")),
                    ]),
                ),
            ]),
        ),
    ])
}

fn reference_arm(name: &str, role: &str, method: Value) -> Value {
    ordered([
        ("name", json!(name)),
        ("role", json!(role)),
        ("method", method),
    ])
}

pub fn prepare_specs(inputs: &Path, out: &Path) -> Result<()> {
    let input_manifest = verify_manifest(inputs)?;
    let archive_sha = string_field(&input_manifest, "source_archive_sha256")?;
    ensure!(
        archive_sha == ARCHIVE_SHA256,
        "unexpected source archive hash in input manifest"
    );
    let mut specs = BTreeMap::<String, Value>::new();
    for bits in BITS {
        let path = inputs.join(format!("archive-spec-{bits}.json"));
        let mut spec: Value = serde_json::from_slice(&fs::read(&path)?)?;
        let rho_method = array(
            spec.get("arms")
                .ok_or_else(|| anyhow!("spec has no arms"))?,
            "spec.arms",
        )?
        .iter()
        .find(|arm| arm.get("name").and_then(Value::as_str) == Some("rho_neg"))
        .and_then(|arm| arm.get("method"))
        .cloned()
        .ok_or_else(|| anyhow!("archive spec {bits} has no rho_neg method"))?;
        let spec_obj = object_mut(&mut spec, "spec")?;
        spec_obj.insert(
            "arms".into(),
            Value::Array(vec![
                candidate_arm(),
                reference_arm("rho", "reference", rho_method.clone()),
                reference_arm("rho_AA", "control", rho_method),
            ]),
        );
        spec_obj.insert(
            "label".into(),
            json!(format!("pencil-symbolic-paired-{bits}")),
        );
        spec_obj.insert(
            "description".into(),
            json!("Prepared specification; execution requires a frozen experiment protocol and admission. Public synthetic archived targets; fixed denominator8 selected before symbolic cohort observations. Cold/online clocks, failures and memory retained; no full-price GAE claim."),
        );
        let measurement = object_mut(
            spec_obj
                .get_mut("measurement")
                .ok_or_else(|| anyhow!("spec has no measurement"))?,
            "spec.measurement",
        )?;
        measurement.insert("warmup".into(), json!(1));
        measurement.insert("rounds".into(), json!(5));
        measurement.insert("algorithm_seed_round".into(), json!("measured_index"));
        measurement.insert("isolation_required".into(), json!("L2"));
        measurement.insert("timeout_seconds".into(), json!(3600));
        measurement.insert("order".into(), json!("shuffle"));
        specs.insert(format!("cohort-{bits}.json"), spec);
    }

    let mut small = specs
        .get("cohort-16.json")
        .cloned()
        .ok_or_else(|| anyhow!("missing cohort-16"))?;
    retain_curve(&mut small, 739, false)?;
    object_mut(&mut small, "gate p739")?.insert("label".into(), json!("pencil-symbolic-gate-p739"));
    specs.insert("gate-p739.json".into(), small);

    let mut upper = specs
        .get("cohort-32.json")
        .cloned()
        .ok_or_else(|| anyhow!("missing cohort-32"))?;
    retain_curve(&mut upper, 186_379, true)?;
    object_mut(&mut upper, "gate p186379")?.insert(
        "label".into(),
        json!("pencil-symbolic-gate-p186379-target0"),
    );
    specs.insert("gate-p186379-target0.json".into(), upper);

    create_output(out)?;
    let mut files = Map::new();
    for (name, spec) in &specs {
        let path = out.join(name);
        write_pretty(&path, spec)?;
        files.insert(name.clone(), json!(sha256(&fs::read(&path)?)));
    }
    let manifest = ordered([
        ("schema", json!("pencil-prepared-specs/v1")),
        ("status", json!("not_frozen_not_launch_authorization")),
        ("source_archive_sha256", json!(archive_sha)),
        ("small_denominator", json!(8)),
        ("matrix_coefficient_cap", json!(25_000_000)),
        ("memory_cap_gib", json!(8)),
        ("per_execution_timeout_seconds", json!(3600)),
        (
            "gate_order",
            json!(["gate-p739.json", "gate-p186379-target0.json"]),
        ),
        (
            "extension_condition",
            json!("upper target verifies under the original resource gate and independent replay; retain failure and do not automatically extend after censoring"),
        ),
        (
            "measured_seed_semantics",
            json!("index0 preserves archived seed; indices1..4 are additional predetermined paired algorithm streams, not repetitions of index0"),
        ),
        ("qualified_ratios_available", json!(false)),
        ("files", Value::Object(files)),
    ]);
    write_pretty(&out.join("manifest.json"), &manifest)
}

fn retain_curve(spec: &mut Value, p: u64, one_target: bool) -> Result<()> {
    let workloads = object_mut(
        object_mut(spec, "spec")?
            .get_mut("workloads")
            .ok_or_else(|| anyhow!("spec has no workloads"))?,
        "spec.workloads",
    )?;
    let curves = workloads
        .get_mut("curves")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("spec.workloads.curves must be an array"))?;
    curves.retain(|curve| curve.get("p").and_then(Value::as_u64) == Some(p));
    ensure!(
        curves.len() == 1,
        "gate p={p} did not select exactly one curve"
    );
    if one_target {
        workloads.insert("targets_per_curve".into(), json!(1));
    }
    Ok(())
}

fn run_plan(binary: &Path, spec: &Path) -> Result<Vec<u8>> {
    let binary =
        fs::canonicalize(binary).with_context(|| format!("resolve {}", binary.display()))?;
    let output = Command::new(&binary)
        .arg("plan")
        .arg("--spec")
        .arg(spec)
        .arg("--doc")
        .output()
        .with_context(|| format!("run {} plan for {}", binary.display(), spec.display()))?;
    ensure!(
        output.status.success(),
        "{} plan failed for {}: {}",
        binary.display(),
        spec.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(output.stdout)
}

fn expected_rows(inputs: &Path) -> Result<Vec<Value>> {
    let bytes = fs::read(inputs.join("expected-workloads.jsonl"))?;
    bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).map_err(Into::into))
        .collect()
}

fn plan_projection(plan: &Value, bits: u64) -> Result<Vec<Value>> {
    let arms = array(
        plan.get("arms")
            .ok_or_else(|| anyhow!("plan has no arms"))?,
        "plan.arms",
    )?;
    let workloads = array(
        plan.get("workloads")
            .ok_or_else(|| anyhow!("plan has no workloads"))?,
        "plan.workloads",
    )?;
    let mut observed = Vec::new();
    for execution in array(
        plan.get("executions")
            .ok_or_else(|| anyhow!("plan has no executions"))?,
        "plan.executions",
    )? {
        if execution.get("warmup").and_then(Value::as_bool) != Some(false)
            || u64_field(execution, "round")? != 0
        {
            continue;
        }
        let arm = &arms[u64_field(execution, "arm")? as usize];
        if arm.get("name").and_then(Value::as_str) != Some("genus_projective") {
            continue;
        }
        let workload = &workloads[u64_field(execution, "workload")? as usize];
        observed.push(ordered([
            ("bits", json!(bits)),
            ("workload_id", json!(string_field(workload, "workload_id")?)),
            (
                "target",
                workload
                    .get("target")
                    .cloned()
                    .ok_or_else(|| anyhow!("plan workload has no target"))?,
            ),
            ("target_seed", json!(u64_field(workload, "target_seed")?)),
            ("target_index", json!(u64_field(workload, "target_index")?)),
            (
                "algorithm_seed",
                json!(normalized_seed(
                    execution
                        .get("algorithm_seed")
                        .ok_or_else(|| anyhow!("execution has no algorithm_seed"))?
                )?),
            ),
        ]));
    }
    Ok(observed)
}

fn sort_rows(rows: &mut [Value]) -> Result<()> {
    for row in rows.iter() {
        string_field(row, "workload_id")?;
    }
    rows.sort_by(|a, b| {
        a.get("workload_id")
            .and_then(Value::as_str)
            .cmp(&b.get("workload_id").and_then(Value::as_str))
    });
    Ok(())
}

fn compare_expected_plan(
    expected_dir: Option<&Path>,
    name: &str,
    bytes: &[u8],
) -> Result<Option<bool>> {
    expected_dir
        .map(|directory| {
            let expected_path = directory.join(name);
            let expected = fs::read(&expected_path)
                .with_context(|| format!("read expected plan {}", expected_path.display()))?;
            Ok(expected == bytes)
        })
        .transpose()
}

pub fn check_archive(
    binary: &Path,
    inputs: &Path,
    out: &Path,
    expected_plans: Option<&Path>,
) -> Result<()> {
    let manifest = verify_manifest(inputs)?;
    ensure!(
        string_field(&manifest, "source_archive_sha256")? == ARCHIVE_SHA256,
        "unexpected archive hash"
    );
    let mut expected = expected_rows(inputs)?;
    let mut observed = Vec::new();
    let mut exact_plan_bytes = true;
    create_output(out)?;
    for bits in BITS {
        let name = format!("plan-{bits}.json");
        let bytes = run_plan(binary, &inputs.join(format!("archive-spec-{bits}.json")))?;
        fs::write(out.join(&name), &bytes)?;
        if let Some(equal) = compare_expected_plan(expected_plans, &name, &bytes)? {
            exact_plan_bytes &= equal;
        }
        observed.extend(plan_projection(&serde_json::from_slice(&bytes)?, bits)?);
    }
    sort_rows(&mut expected)?;
    sort_rows(&mut observed)?;
    let passed = expected.len() == EXPECTED_ARCHIVE_ROWS
        && observed.len() == EXPECTED_ARCHIVE_ROWS
        && expected == observed
        && exact_plan_bytes;
    let result = ordered([
        ("schema", json!("pencil-native-input-parity/v2")),
        ("passed", json!(passed)),
        ("expected", json!(expected.len())),
        ("observed", json!(observed.len())),
        (
            "binary_sha256",
            json!(sha256(&fs::read(binary).with_context(|| format!("read {}", binary.display()))?)),
        ),
        ("source_archive_sha256", json!(ARCHIVE_SHA256)),
        ("exact_retained_plan_bytes", json!(expected_plans.map(|_| exact_plan_bytes))),
        ("solver_executions_performed", json!(0)),
        (
            "scope",
            json!("Workload identities, target coordinates and archived first algorithm seeds only; plan generation only, with no solves, cost measurements or source-build attestation."),
        ),
    ]);
    write_pretty(&out.join("result.json"), &result)?;
    ensure!(
        passed,
        "native workload parity mismatch; retained plans and result"
    );
    Ok(())
}

#[derive(Default)]
struct PreparedCounts {
    targets: usize,
    planned: usize,
    measured: usize,
    warmups: usize,
}

fn validate_prepared_spec(spec: &Value, name: &str) -> Result<()> {
    let arms = array(
        spec.get("arms")
            .ok_or_else(|| anyhow!("{name} has no arms"))?,
        "spec.arms",
    )?;
    ensure!(arms.len() == 3, "{name} must have three arms");
    let expected = [
        ("pencil", "candidate"),
        ("rho", "reference"),
        ("rho_AA", "control"),
    ];
    for (arm, (arm_name, role)) in arms.iter().zip(expected) {
        ensure!(
            arm.get("name").and_then(Value::as_str) == Some(arm_name),
            "{name} arm name mismatch"
        );
        ensure!(
            arm.get("role").and_then(Value::as_str) == Some(role),
            "{name} arm role mismatch"
        );
    }
    let params = arms[0]
        .pointer("/method/params")
        .ok_or_else(|| anyhow!("{name} candidate has no params"))?;
    ensure!(
        arms[0].pointer("/method/id").and_then(Value::as_str)
            == Some("ic.three_leads_genus2_pencil_symbolic_v2"),
        "{name} candidate method mismatch"
    );
    ensure!(
        params.get("small_denominator").and_then(Value::as_str) == Some("8"),
        "{name} denominator mismatch"
    );
    ensure!(
        params.get("matrix_coefficient_cap").and_then(Value::as_str) == Some("25000000"),
        "{name} matrix cap mismatch"
    );
    ensure!(
        arms[1].get("method") == arms[2].get("method"),
        "{name} reference and A/A methods differ"
    );
    let measurement = spec
        .get("measurement")
        .ok_or_else(|| anyhow!("{name} has no measurement"))?;
    ensure!(
        measurement.get("warmup").and_then(Value::as_u64) == Some(1),
        "{name} warmup mismatch"
    );
    ensure!(
        measurement.get("rounds").and_then(Value::as_u64) == Some(5),
        "{name} rounds mismatch"
    );
    ensure!(
        measurement
            .get("algorithm_seed_round")
            .and_then(Value::as_str)
            == Some("measured_index"),
        "{name} seed semantics mismatch"
    );
    ensure!(
        measurement
            .get("isolation_required")
            .and_then(Value::as_str)
            == Some("L2"),
        "{name} isolation mismatch"
    );
    ensure!(
        measurement.get("timeout_seconds").and_then(Value::as_u64) == Some(3600),
        "{name} timeout mismatch"
    );
    ensure!(
        measurement.get("order").and_then(Value::as_str) == Some("shuffle"),
        "{name} order mismatch"
    );
    Ok(())
}

fn validate_prepared_plan(
    plan: &Value,
    expected_by_id: &HashMap<String, Value>,
    source_name: &str,
    expected_bits: u64,
    cohort: bool,
) -> Result<PreparedCounts> {
    let arms = array(
        plan.get("arms")
            .ok_or_else(|| anyhow!("plan has no arms"))?,
        "plan.arms",
    )?;
    ensure!(arms.len() == 3, "{source_name} resolved arm count mismatch");
    ensure!(
        arms[0].get("name").and_then(Value::as_str) == Some("pencil"),
        "{source_name} candidate resolution mismatch"
    );
    ensure!(
        arms[1].get("name").and_then(Value::as_str) == Some("rho"),
        "{source_name} reference resolution mismatch"
    );
    ensure!(
        arms[2].get("name").and_then(Value::as_str) == Some("rho_AA"),
        "{source_name} A/A resolution mismatch"
    );
    ensure!(
        arms[1].get("method") == arms[2].get("method"),
        "{source_name} resolved reference/A/A methods differ"
    );
    let workloads = array(
        plan.get("workloads")
            .ok_or_else(|| anyhow!("plan has no workloads"))?,
        "plan.workloads",
    )?;
    let executions = array(
        plan.get("executions")
            .ok_or_else(|| anyhow!("plan has no executions"))?,
        "plan.executions",
    )?;
    let mut paired = HashMap::<(usize, u64), (String, u8)>::new();
    let mut first_measured = HashMap::<usize, String>::new();
    let mut counts = PreparedCounts {
        targets: workloads.len(),
        planned: executions.len(),
        measured: 0,
        warmups: 0,
    };
    for (index, workload) in workloads.iter().enumerate() {
        let workload_id = string_field(workload, "workload_id")?;
        let expected = expected_by_id
            .get(workload_id)
            .ok_or_else(|| anyhow!("{source_name} has unregistered workload {workload_id}"))?;
        ensure!(
            workload.get("target") == expected.get("target"),
            "{source_name} target mismatch for {workload_id}"
        );
        ensure!(
            workload.get("target_seed") == expected.get("target_seed"),
            "{source_name} target seed mismatch for {workload_id}"
        );
        ensure!(
            workload.get("target_index") == expected.get("target_index"),
            "{source_name} target index mismatch for {workload_id}"
        );
        ensure!(
            expected.get("bits").and_then(Value::as_u64) == Some(expected_bits),
            "{source_name} size mismatch for {workload_id}"
        );
        ensure!(index < workloads.len(), "workload index overflow");
    }
    for execution in executions {
        let workload = u64_field(execution, "workload")? as usize;
        ensure!(
            workload < workloads.len(),
            "{source_name} execution workload index out of range"
        );
        let round = u64_field(execution, "round")?;
        let arm = u64_field(execution, "arm")? as usize;
        ensure!(arm < 3, "{source_name} execution arm index out of range");
        let seed = normalized_seed(
            execution
                .get("algorithm_seed")
                .ok_or_else(|| anyhow!("execution has no algorithm seed"))?,
        )?;
        if execution.get("warmup").and_then(Value::as_bool) == Some(true) {
            ensure!(round == 0, "{source_name} warmup round must be zero");
            counts.warmups += 1;
            continue;
        }
        ensure!(
            (1..=5).contains(&round),
            "{source_name} measured round is out of range"
        );
        counts.measured += 1;
        let (paired_seed, arm_mask) = paired
            .entry((workload, round))
            .or_insert_with(|| (seed.clone(), 0));
        ensure!(
            paired_seed == &seed,
            "{source_name} paired arms use different seeds"
        );
        let arm_bit = 1_u8 << arm;
        ensure!(
            *arm_mask & arm_bit == 0,
            "{source_name} duplicates an arm within a workload round"
        );
        *arm_mask |= arm_bit;
        if round == 1 {
            match first_measured.entry(workload) {
                std::collections::hash_map::Entry::Vacant(entry) => {
                    entry.insert(seed);
                }
                std::collections::hash_map::Entry::Occupied(entry) => {
                    ensure!(
                        entry.get() == &seed,
                        "{source_name} first measured arms use different seeds"
                    );
                }
            }
        }
    }
    ensure!(
        counts.planned == counts.measured + counts.warmups,
        "{source_name} execution counts do not close"
    );
    ensure!(
        counts.measured == workloads.len() * 5 * 3,
        "{source_name} measured execution count mismatch"
    );
    ensure!(
        counts.warmups == workloads.len() * 3,
        "{source_name} warmup execution count mismatch"
    );
    ensure!(
        paired.len() == workloads.len() * 5
            && paired.values().all(|(_, arm_mask)| *arm_mask == 0b111),
        "{source_name} does not contain each arm exactly once per workload round"
    );
    ensure!(
        first_measured.len() == workloads.len(),
        "{source_name} is missing a first measured seed"
    );
    for (index, seed) in first_measured {
        let workload_id = string_field(&workloads[index], "workload_id")?;
        let expected = expected_by_id.get(workload_id).unwrap();
        ensure!(
            normalized_seed(
                expected
                    .get("algorithm_seed")
                    .ok_or_else(|| anyhow!("expected row has no algorithm seed"))?
            )? == seed,
            "{source_name} first measured seed mismatch for {workload_id}"
        );
    }
    if cohort {
        ensure!(
            workloads.len() == 180,
            "{source_name} cohort must contain 180 targets"
        );
    }
    Ok(counts)
}

pub fn check_prepared(
    binary: &Path,
    inputs: &Path,
    specs: &Path,
    out: &Path,
    expected_plans: Option<&Path>,
) -> Result<()> {
    let input_manifest = verify_manifest(inputs)?;
    let spec_manifest = verify_manifest(specs)?;
    ensure!(
        string_field(&input_manifest, "source_archive_sha256")? == ARCHIVE_SHA256,
        "unexpected input archive hash"
    );
    ensure!(
        string_field(&spec_manifest, "source_archive_sha256")? == ARCHIVE_SHA256,
        "unexpected spec archive hash"
    );
    ensure!(
        spec_manifest.get("gate_order")
            == Some(&json!(["gate-p739.json", "gate-p186379-target0.json"])),
        "gate order mismatch"
    );
    ensure!(
        spec_manifest.get("status").and_then(Value::as_str)
            == Some("not_frozen_not_launch_authorization"),
        "prepared manifest status mismatch"
    );
    ensure!(
        spec_manifest
            .get("small_denominator")
            .and_then(Value::as_u64)
            == Some(8)
            && spec_manifest
                .get("matrix_coefficient_cap")
                .and_then(Value::as_u64)
                == Some(25_000_000)
            && spec_manifest.get("memory_cap_gib").and_then(Value::as_u64) == Some(8)
            && spec_manifest
                .get("per_execution_timeout_seconds")
                .and_then(Value::as_u64)
                == Some(3600),
        "prepared manifest resource contract mismatch"
    );

    let expected = expected_rows(inputs)?;
    ensure!(
        expected.len() == EXPECTED_ARCHIVE_ROWS,
        "expected workload inventory mismatch"
    );
    let expected_by_id = expected
        .into_iter()
        .map(|row| Ok((string_field(&row, "workload_id")?.to_owned(), row)))
        .collect::<Result<HashMap<_, _>>>()?;
    ensure!(
        expected_by_id.len() == EXPECTED_ARCHIVE_ROWS,
        "duplicate expected workloads"
    );

    create_output(out)?;
    let mut cohort_counts = PreparedCounts::default();
    let mut gate_counts = PreparedCounts::default();
    let mut cohort_ids = BTreeSet::<String>::new();
    let mut file_results = Vec::new();
    let mut all_exact = true;
    let spec_files = object(
        spec_manifest
            .get("files")
            .ok_or_else(|| anyhow!("spec manifest has no files"))?,
        "spec manifest files",
    )?;
    let expected_spec_files = BTreeSet::from([
        "cohort-16.json",
        "cohort-20.json",
        "cohort-24.json",
        "cohort-28.json",
        "cohort-32.json",
        "cohort-36.json",
        "cohort-40.json",
        "gate-p186379-target0.json",
        "gate-p739.json",
    ]);
    ensure!(
        spec_files
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>()
            == expected_spec_files,
        "prepared manifest file inventory mismatch"
    );
    for name in spec_files.keys() {
        let spec_path = specs.join(name);
        let spec: Value = serde_json::from_slice(&fs::read(&spec_path)?)?;
        validate_prepared_spec(&spec, name)?;
        let plan_bytes = run_plan(binary, &spec_path)?;
        fs::write(out.join(name), &plan_bytes)?;
        let exact = compare_expected_plan(expected_plans, name, &plan_bytes)?;
        if let Some(equal) = exact {
            all_exact &= equal;
        }
        let plan: Value = serde_json::from_slice(&plan_bytes)?;
        let cohort = name.starts_with("cohort-");
        let expected_bits = match name.as_str() {
            "cohort-16.json" | "gate-p739.json" => 16,
            "cohort-20.json" => 20,
            "cohort-24.json" => 24,
            "cohort-28.json" => 28,
            "cohort-32.json" | "gate-p186379-target0.json" => 32,
            "cohort-36.json" => 36,
            "cohort-40.json" => 40,
            _ => bail!("unexpected prepared spec {name}"),
        };
        let counts = validate_prepared_plan(&plan, &expected_by_id, name, expected_bits, cohort)?;
        if cohort {
            for workload in array(plan.get("workloads").unwrap(), "plan.workloads")? {
                ensure!(
                    cohort_ids.insert(string_field(workload, "workload_id")?.to_owned()),
                    "duplicate cohort workload across sizes"
                );
            }
            cohort_counts.targets += counts.targets;
            cohort_counts.planned += counts.planned;
            cohort_counts.measured += counts.measured;
            cohort_counts.warmups += counts.warmups;
        } else {
            gate_counts.targets += counts.targets;
            gate_counts.planned += counts.planned;
            gate_counts.measured += counts.measured;
            gate_counts.warmups += counts.warmups;
            let curves = array(
                spec.pointer("/workloads/curves")
                    .ok_or_else(|| anyhow!("gate has no curves"))?,
                "gate curves",
            )?;
            ensure!(curves.len() == 1, "{name} gate must have one curve");
            let p = u64_field(&curves[0], "p")?;
            if name == "gate-p739.json" {
                ensure!(
                    p == 739 && counts.targets == 60,
                    "small gate membership mismatch"
                );
            } else if name == "gate-p186379-target0.json" {
                ensure!(
                    p == 186_379 && counts.targets == 1,
                    "upper gate membership mismatch"
                );
                let target_index = plan
                    .pointer("/workloads/0/target_index")
                    .and_then(Value::as_u64);
                ensure!(
                    target_index == Some(0),
                    "upper gate must contain target index zero"
                );
            } else {
                bail!("unexpected prepared non-cohort spec {name}");
            }
        }
        file_results.push(ordered([
            ("spec", json!(name)),
            ("sha256", json!(sha256(&plan_bytes))),
            ("exact_retained_plan_bytes", json!(exact)),
            ("targets", json!(counts.targets)),
            ("planned_executions", json!(counts.planned)),
        ]));
    }
    ensure!(
        cohort_ids.len() == EXPECTED_ARCHIVE_ROWS,
        "cohort workload coverage mismatch"
    );
    ensure!(
        cohort_counts.targets == 1260,
        "cohort target count mismatch"
    );
    ensure!(
        cohort_counts.planned == 22_680,
        "cohort planned execution count mismatch"
    );
    ensure!(
        cohort_counts.measured == 18_900,
        "cohort measured execution count mismatch"
    );
    ensure!(
        cohort_counts.warmups == 3_780,
        "cohort warmup count mismatch"
    );
    let passed = all_exact;
    let result = ordered([
        ("schema", json!("pencil-native-preparation-validation/v1")),
        ("passed", json!(passed)),
        ("solver_executions_performed", json!(0)),
        (
            "binary_sha256",
            json!(sha256(&fs::read(binary).with_context(|| format!("read {}", binary.display()))?)),
        ),
        ("source_archive_sha256", json!(ARCHIVE_SHA256)),
        ("cohort_targets", json!(cohort_counts.targets)),
        ("cohort_planned_executions", json!(cohort_counts.planned)),
        ("cohort_measured_executions", json!(cohort_counts.measured)),
        ("cohort_warmup_executions", json!(cohort_counts.warmups)),
        ("gate_targets", json!(gate_counts.targets)),
        ("gate_planned_executions", json!(gate_counts.planned)),
        ("first_measured_seeds_match_archive", json!(true)),
        ("all_three_arms_share_each_workload_round_seed", json!(true)),
        ("reference_and_aa_resolved_methods_identical", json!(true)),
        ("candidate_resource_parameters_match_manifest", json!(true)),
        ("measurement_and_gate_contracts_match_manifest", json!(true)),
        ("exact_retained_plan_bytes", json!(expected_plans.map(|_| all_exact))),
        ("files", Value::Array(file_results)),
        (
            "scope",
            json!("Native plan generation and structural validation only; no solver, cohort, timing, memory or key-recovery execution."),
        ),
    ]);
    write_pretty(&out.join("result.json"), &result)?;
    ensure!(
        passed,
        "prepared native plan parity mismatch; retained plans and result"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepared_specs_match_the_retained_bytes() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let inputs = root.join("../validation_inputs");
        let retained = root.join("../prepared_specs");
        let temp = tempfile::tempdir().unwrap();
        let out = temp.path().join("prepared");
        prepare_specs(&inputs, &out).unwrap();
        for name in [
            "cohort-16.json",
            "cohort-20.json",
            "cohort-24.json",
            "cohort-28.json",
            "cohort-32.json",
            "cohort-36.json",
            "cohort-40.json",
            "gate-p739.json",
            "gate-p186379-target0.json",
            "manifest.json",
        ] {
            assert_eq!(
                fs::read(out.join(name)).unwrap(),
                fs::read(retained.join(name)).unwrap(),
                "byte mismatch for {name}"
            );
        }
    }

    #[test]
    fn archive_extractor_projects_only_the_declared_public_fields() {
        let mut tar_bytes = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut tar_bytes);
            for bits in BITS {
                let prefix = format!("sessions_projective_chart_v2/order-genus-{bits}");
                append(
                    &mut builder,
                    &format!("{prefix}/spec.json"),
                    b"{\"schema\":\"fixture\"}\n",
                );
                let record = format!(
                    "{{\"arm\":\"genus_projective\",\"algorithm_seed\":\"{}\",\"workload\":{{\"workload_id\":\"W{bits}\",\"target\":[\"0x1\",\"0x2\"],\"target_seed\":7,\"target_index\":0,\"planted\":99}}}}\n{{\"arm\":\"rho_neg\",\"algorithm_seed\":\"2\",\"workload\":{{}}}}\n",
                    bits + 100
                );
                append(
                    &mut builder,
                    &format!("{prefix}/records.jsonl"),
                    record.as_bytes(),
                );
            }
            builder.finish().unwrap();
        }
        let compressed = zstd::stream::encode_all(Cursor::new(tar_bytes), 1).unwrap();
        let temp = tempfile::tempdir().unwrap();
        let archive = temp.path().join("fixture.tar.zst");
        fs::write(&archive, &compressed).unwrap();
        let out = temp.path().join("out");
        prepare_archive_with_contract(&archive, &out, &sha256(&compressed), BITS.len()).unwrap();
        let projected = fs::read_to_string(out.join("expected-workloads.jsonl")).unwrap();
        assert_eq!(projected.lines().count(), BITS.len());
        assert!(!projected.contains("planted"));
        assert!(projected.contains("\"algorithm_seed\":\"116\""));
        let manifest = fs::read_to_string(out.join("manifest.json")).unwrap();
        assert!(
            manifest.find("\"files\"").unwrap() < manifest.find("\"projection\"").unwrap(),
            "manifest key order must remain byte-compatible with the retained projection"
        );
        verify_manifest(&out).unwrap();
    }

    #[test]
    fn prepared_plan_requires_complete_paired_measured_rounds() {
        let workload = json!({
            "workload_id": "W16",
            "target": ["0x1", "0x2"],
            "target_seed": 7,
            "target_index": 0
        });
        let mut expected = HashMap::new();
        expected.insert(
            "W16".to_owned(),
            json!({
                "bits": 16,
                "workload_id": "W16",
                "target": ["0x1", "0x2"],
                "target_seed": 7,
                "target_index": 0,
                "algorithm_seed": "42"
            }),
        );
        let arms = json!([
            {"name": "pencil", "method": {"id": "candidate"}},
            {"name": "rho", "method": {"id": "reference"}},
            {"name": "rho_AA", "method": {"id": "reference"}}
        ]);
        let mut executions = Vec::new();
        for arm in 0..3 {
            executions.push(json!({
                "round": 0,
                "warmup": true,
                "arm": arm,
                "workload": 0,
                "algorithm_seed": 9
            }));
        }
        for round in 1..=5 {
            let seed = if round == 1 { 42 } else { 100 + round };
            for arm in 0..3 {
                executions.push(json!({
                    "round": round,
                    "warmup": false,
                    "arm": arm,
                    "workload": 0,
                    "algorithm_seed": seed
                }));
            }
        }
        let plan = json!({
            "arms": arms,
            "workloads": [workload],
            "executions": executions
        });
        let counts = validate_prepared_plan(&plan, &expected, "fixture", 16, false).unwrap();
        assert_eq!(counts.planned, 18);
        assert_eq!(counts.measured, 15);
        assert_eq!(counts.warmups, 3);

        let mut incomplete = plan;
        incomplete
            .get_mut("executions")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .pop();
        assert!(validate_prepared_plan(&incomplete, &expected, "fixture", 16, false).is_err());
    }

    fn append(builder: &mut tar::Builder<&mut Vec<u8>>, path: &str, data: &[u8]) {
        let mut header = tar::Header::new_gnu();
        header.set_size(data.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, path, Cursor::new(data))
            .unwrap();
    }
}
