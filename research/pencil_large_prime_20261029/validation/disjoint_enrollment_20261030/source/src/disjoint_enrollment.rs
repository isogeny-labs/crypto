use super::{
    array, create_output, object, run_plan, sha256, string_field, u64_field, write_pretty,
};
use anyhow::{anyhow, ensure, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const REQUEST_SCHEMA: &str = "pencil-disjoint-enrollment-request/v1";
const PLAN_SCHEMA: &str = "ecbench.plan/v1";
const INSTRUMENT_SHA256: &str = "a1c8ca89748a22b8a07ea00fd3983061be0d8197d5cae49fca883dcf5dd50eab";
const INSTRUMENT_SOURCE_REVISION: &str = "543a54b4b1c3be7052872af3ffe393a44c69a3c3";
const BUILD_INFO_SHA256: &str = "17b5602dd978e0484677b4b9e27b8323ceb60b01ae5bbdcf9728e578d0d352c5";
const GATE_SPEC_SHA256: &str = "a2c25a203ad4c39aaea57287e1705dd4b3db394cd10507fff009980ec8634c18";
const GATE_PLAN_SHA256: &str = "bc57819e43862f5d9c9aee0f13b983fec186b2dd14dbc8b921d8999acc4b6776";
const REMAINDER_SPEC_NAME: &str = "cohort-16-remainder.json";
const REMAINDER_PLAN_NAME: &str = "cohort-16-remainder-plan.json";
const REMAINDER_LABEL: &str = "pencil-symbolic-paired-16-remainder-p739-excluded";

const ORIGINAL_SPEC_HASHES: [(&str, &str); 7] = [
    (
        "cohort-16.json",
        "1c5a421d5ad629be5ed66c360e7eafa49a35d3eebaec4bf7651848587828ee79",
    ),
    (
        "cohort-20.json",
        "33104eb19ee7a8995bccb43a3768824643b6ec6dae45bca4da809f88503fb639",
    ),
    (
        "cohort-24.json",
        "cbc91d501e44fb101a96ff0ce9a7d96cbd37d2761e910fa62cbd47dfd56ac315",
    ),
    (
        "cohort-28.json",
        "04c87c844c9216993cfa90aecbb1606080338232ebfa6ab91f12b6881d4b477c",
    ),
    (
        "cohort-32.json",
        "0bcb2f5f8811a370997724bf0186330fba14a896346d9206d3f67d8368cb22a3",
    ),
    (
        "cohort-36.json",
        "39cac1b85cb9672e5e5f3418957c994a005b59432dd2b6ca74226bed60d832fe",
    ),
    (
        "cohort-40.json",
        "fee38c5c381107272d65a89daa8151f75aa249082d0e9c82f1014211fdee2587",
    ),
];

const ORIGINAL_PLAN_HASHES: [(&str, &str); 7] = [
    (
        "cohort-16.json",
        "35e6311a7105a29c2459543ad0b281ed91e43aa63a99f66226f95a1f3c9cc2ba",
    ),
    (
        "cohort-20.json",
        "51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f",
    ),
    (
        "cohort-24.json",
        "7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4",
    ),
    (
        "cohort-28.json",
        "87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001",
    ),
    (
        "cohort-32.json",
        "a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96",
    ),
    (
        "cohort-36.json",
        "6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2",
    ),
    (
        "cohort-40.json",
        "8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5",
    ),
];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileBinding {
    name: String,
    path: PathBuf,
    sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct InstrumentBinding {
    binary: FileBinding,
    source_revision: String,
    build_info: FileBinding,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RemainderDeclaration {
    source_spec: String,
    output_spec: String,
    output_plan: String,
    label: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: String,
    instrument: InstrumentBinding,
    original_specs: Vec<FileBinding>,
    original_plans: Vec<FileBinding>,
    gate_spec: FileBinding,
    gate_plan: FileBinding,
    remainder: RemainderDeclaration,
    protocol_lineage: Vec<FileBinding>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SlotKey {
    workload_id: String,
    warmup: bool,
    round: u64,
    arm: String,
}

#[derive(Clone, Debug)]
struct Slot {
    plan: String,
    seq: u64,
    workload_index: u64,
    algorithm_seed: u64,
    arm_role: String,
    order_position: u64,
    workload: Value,
    arm: Value,
}

#[derive(Clone, Debug)]
struct WorkloadLocation {
    plan: String,
    index: u64,
    value: Value,
}

#[derive(Default)]
struct Inventory {
    slots: BTreeMap<SlotKey, Slot>,
    workloads: BTreeMap<String, WorkloadLocation>,
    orders: BTreeMap<(String, bool, u64), Vec<String>>,
    measured: usize,
    warmups: usize,
}

#[derive(Clone, Copy)]
struct ExpectedTotals {
    workloads: usize,
    slots: usize,
    measured: usize,
    warmups: usize,
}

const ACTUAL_TOTALS: ExpectedTotals = ExpectedTotals {
    workloads: 1_260,
    slots: 22_680,
    measured: 18_900,
    warmups: 3_780,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum IdentityCurve {
    ThreeLeads {
        p: u64,
        n: u32,
        modulus: u64,
        a: u64,
        b: u64,
        group_order: u64,
        r: u64,
        gx: u64,
        gy: u64,
        family: String,
        parameter: u64,
    },
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum IdentityTargetKind {
    #[default]
    Planted,
    PlantedShared,
    Public,
}

fn plain_planted(kind: &IdentityTargetKind) -> bool {
    *kind == IdentityTargetKind::Planted
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IdentityWorkloads {
    curves: Vec<IdentityCurve>,
    targets_per_curve: u64,
    target_seed: u64,
    #[serde(default, skip_serializing_if = "plain_planted")]
    target_kind: IdentityTargetKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum IdentityRole {
    Reference,
    Baseline,
    Candidate,
    Control,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IdentityMethod {
    id: String,
    #[serde(default)]
    params: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IdentityArm {
    name: String,
    role: IdentityRole,
    method: IdentityMethod,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum IdentityOrder {
    Alternate,
    Shuffle,
    AlternateMeasured,
}

fn default_identity_warmup() -> u32 {
    1
}

fn default_identity_order() -> IdentityOrder {
    IdentityOrder::Alternate
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
enum IdentityLevel {
    L0,
    L1,
    L2,
    L3,
}

fn default_identity_level() -> IdentityLevel {
    IdentityLevel::L2
}

fn default_identity_timeout() -> u64 {
    600
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum IdentitySeedRound {
    #[default]
    Absolute,
    MeasuredIndex,
}

fn absolute_seed_round(round: &IdentitySeedRound) -> bool {
    *round == IdentitySeedRound::Absolute
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IdentityMeasurement {
    rounds: u32,
    #[serde(default = "default_identity_warmup")]
    warmup: u32,
    #[serde(default = "default_identity_order")]
    order: IdentityOrder,
    seed: u64,
    #[serde(default, skip_serializing_if = "absolute_seed_round")]
    algorithm_seed_round: IdentitySeedRound,
    #[serde(default = "default_identity_level")]
    isolation_required: IdentityLevel,
    #[serde(default = "default_identity_timeout")]
    timeout_seconds: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IdentitySpec {
    schema: String,
    #[serde(rename = "label")]
    _label: String,
    #[serde(default, rename = "description")]
    _description: String,
    workloads: IdentityWorkloads,
    arms: Vec<IdentityArm>,
    measurement: IdentityMeasurement,
}

fn write_canonical(value: &Value, out: &mut String) -> Result<()> {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(flag) => out.push_str(if *flag { "true" } else { "false" }),
        Value::Number(number) => {
            let text = number.to_string();
            ensure!(
                number.is_i64() || number.is_u64(),
                "spec identity contains a non-integer number"
            );
            out.push_str(&text);
        }
        Value::String(text) => {
            out.push('"');
            for character in text.chars() {
                match character {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    '\u{08}' => out.push_str("\\b"),
                    '\u{0c}' => out.push_str("\\f"),
                    c if (' '..='~').contains(&c) => out.push(c),
                    c if (c as u32) < 0x80 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => {
                        let mut buffer = [0_u16; 2];
                        for unit in c.encode_utf16(&mut buffer) {
                            out.push_str(&format!("\\u{unit:04x}"));
                        }
                    }
                }
            }
            out.push('"');
        }
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_canonical(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            out.push('{');
            let mut keys = map.keys().collect::<Vec<_>>();
            keys.sort();
            for (index, key) in keys.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_canonical(&Value::String((*key).clone()), out)?;
                out.push(':');
                write_canonical(&map[*key], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn spec_identity(spec: &Value) -> Result<(String, String)> {
    let typed: IdentitySpec = serde_json::from_value(spec.clone())?;
    ensure!(
        typed.schema == "ecbench.spec/v1",
        "spec identity schema mismatch"
    );
    let view = json!({
        "schema": typed.schema,
        "workloads": typed.workloads,
        "arms": typed.arms,
        "measurement": typed.measurement,
    });
    let mut canonical = String::new();
    write_canonical(&view, &mut canonical)?;
    let digest = sha256(canonical.as_bytes());
    Ok((format!("ECS1h{}", &digest[..12]), digest))
}

fn validate_plan_spec_identity(spec: &Value, plan: &Value, name: &str) -> Result<String> {
    let (expected_id, digest) = spec_identity(spec)?;
    ensure!(
        string_field(plan, "spec_id")? == expected_id,
        "{name} plan spec_id does not match its typed specification identity"
    );
    Ok(digest)
}

fn expected_map(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(name, digest)| ((*name).to_owned(), (*digest).to_owned()))
        .collect()
}

fn resolve(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        base.join(path)
    }
}

fn valid_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn read_bound(base: &Path, binding: &FileBinding) -> Result<(PathBuf, Vec<u8>)> {
    ensure!(
        valid_digest(&binding.sha256),
        "{} has a malformed SHA-256",
        binding.name
    );
    let path = resolve(base, &binding.path);
    let bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;
    ensure!(
        sha256(&bytes) == binding.sha256,
        "bound file hash mismatch for {}",
        binding.name
    );
    Ok((path, bytes))
}

fn read_bound_json(base: &Path, binding: &FileBinding) -> Result<Value> {
    let (_, bytes) = read_bound(base, binding)?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {}", binding.name))
}

fn binding_value(binding: &FileBinding) -> Value {
    json!({
        "name": binding.name,
        "path": binding.path,
        "sha256": binding.sha256,
    })
}

fn read_named_json_set(
    base: &Path,
    bindings: &[FileBinding],
    expected: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, Value>> {
    ensure!(
        bindings.len() == expected.len(),
        "bound file inventory length mismatch"
    );
    let mut values = BTreeMap::new();
    for binding in bindings {
        let digest = expected
            .get(&binding.name)
            .ok_or_else(|| anyhow!("unexpected bound file {}", binding.name))?;
        ensure!(
            &binding.sha256 == digest,
            "approved hash mismatch for {}",
            binding.name
        );
        ensure!(
            values
                .insert(binding.name.clone(), read_bound_json(base, binding)?)
                .is_none(),
            "duplicate bound file {}",
            binding.name
        );
    }
    ensure!(
        values.keys().eq(expected.keys()),
        "bound file inventory mismatch"
    );
    Ok(values)
}

fn curves_mut(spec: &mut Value) -> Result<&mut Vec<Value>> {
    spec.pointer_mut("/workloads/curves")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| anyhow!("spec workloads.curves must be an array"))
}

fn label_mut(spec: &mut Value) -> Result<&mut Value> {
    spec.get_mut("label")
        .ok_or_else(|| anyhow!("spec has no label"))
}

fn validate_partition_specs(
    source: &Value,
    gate: &Value,
    remainder: &Value,
    remainder_label: &str,
) -> Result<Value> {
    ensure!(
        source.get("schema").and_then(Value::as_str) == Some("ecbench.spec/v1"),
        "source specification schema mismatch"
    );
    let gate_curves = array(
        gate.pointer("/workloads/curves")
            .ok_or_else(|| anyhow!("gate has no curves"))?,
        "gate curves",
    )?;
    ensure!(
        gate_curves.len() == 1,
        "gate must contain exactly one curve"
    );
    let gate_curve = &gate_curves[0];
    ensure!(
        gate_curve.get("p").and_then(Value::as_u64) == Some(739),
        "gate curve must be p739"
    );
    let source_curves = array(
        source
            .pointer("/workloads/curves")
            .ok_or_else(|| anyhow!("source has no curves"))?,
        "source curves",
    )?;
    ensure!(
        source_curves
            .iter()
            .filter(|curve| *curve == gate_curve)
            .count()
            == 1,
        "source must contain the exact gate curve once"
    );

    let mut expected_gate = source.clone();
    curves_mut(&mut expected_gate)?.retain(|curve| curve == gate_curve);
    *label_mut(&mut expected_gate)? = gate
        .get("label")
        .cloned()
        .ok_or_else(|| anyhow!("gate has no label"))?;
    ensure!(
        &expected_gate == gate,
        "gate differs from cohort16 beyond exact curve retention and label"
    );

    let mut expected_remainder = source.clone();
    curves_mut(&mut expected_remainder)?.retain(|curve| curve != gate_curve);
    ensure!(
        curves_mut(&mut expected_remainder)?.len() + 1 == source_curves.len(),
        "remainder did not remove exactly one curve"
    );
    *label_mut(&mut expected_remainder)? = json!(remainder_label);
    ensure!(
        &expected_remainder == remainder,
        "remainder has an unauthorized specification change"
    );
    Ok(expected_remainder)
}

fn derive_remainder(source: &Value, gate: &Value, remainder_label: &str) -> Result<Value> {
    let gate_curve = gate
        .pointer("/workloads/curves/0")
        .ok_or_else(|| anyhow!("gate has no sole curve"))?;
    let mut remainder = source.clone();
    curves_mut(&mut remainder)?.retain(|curve| curve != gate_curve);
    *label_mut(&mut remainder)? = json!(remainder_label);
    validate_partition_specs(source, gate, &remainder, remainder_label)?;
    Ok(remainder)
}

fn exact_keys(value: &Value, expected: &[&str], name: &str) -> Result<()> {
    let actual = object(value, name)?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    ensure!(actual == expected, "{name} field inventory mismatch");
    Ok(())
}

fn parse_plan(name: &str, plan: &Value) -> Result<Inventory> {
    exact_keys(
        plan,
        &["arms", "executions", "schema", "spec_id", "workloads"],
        name,
    )?;
    ensure!(
        plan.get("schema").and_then(Value::as_str) == Some(PLAN_SCHEMA),
        "{name} plan schema mismatch"
    );
    string_field(plan, "spec_id")?;
    let arms = array(
        plan.get("arms")
            .ok_or_else(|| anyhow!("{name} has no arms"))?,
        "plan arms",
    )?;
    ensure!(arms.len() == 3, "{name} must have three arms");
    let expected_arms = [
        ("pencil", "candidate"),
        ("rho", "reference"),
        ("rho_AA", "control"),
    ];
    let mut arm_names = BTreeSet::new();
    for (arm, (expected_name, expected_role)) in arms.iter().zip(expected_arms) {
        ensure!(
            string_field(arm, "name")? == expected_name
                && string_field(arm, "role")? == expected_role,
            "{name} arm name or role mismatch"
        );
        ensure!(arm.get("method").is_some(), "{name} arm has no method");
        ensure!(arm_names.insert(expected_name), "{name} duplicates an arm");
    }

    let workloads = array(
        plan.get("workloads")
            .ok_or_else(|| anyhow!("{name} has no workloads"))?,
        "plan workloads",
    )?;
    let executions = array(
        plan.get("executions")
            .ok_or_else(|| anyhow!("{name} has no executions"))?,
        "plan executions",
    )?;
    let mut inventory = Inventory::default();
    for (index, workload) in workloads.iter().enumerate() {
        let workload_id = string_field(workload, "workload_id")?.to_owned();
        ensure!(
            inventory
                .workloads
                .insert(
                    workload_id.clone(),
                    WorkloadLocation {
                        plan: name.to_owned(),
                        index: index as u64,
                        value: workload.clone(),
                    },
                )
                .is_none(),
            "{name} duplicates workload {workload_id}"
        );
    }

    let mut sequences = BTreeSet::new();
    let mut ordered_executions = Vec::with_capacity(executions.len());
    for execution in executions {
        exact_keys(
            execution,
            &[
                "algorithm_seed",
                "arm",
                "round",
                "seq",
                "warmup",
                "workload",
            ],
            "plan execution",
        )?;
        let seq = u64_field(execution, "seq")?;
        ensure!(sequences.insert(seq), "{name} duplicates sequence {seq}");
        ordered_executions.push((seq, execution));
    }
    ensure!(
        sequences.iter().copied().eq(0..executions.len() as u64),
        "{name} sequences must be contiguous from zero"
    );
    ordered_executions.sort_by_key(|(seq, _)| *seq);

    for (seq, execution) in ordered_executions {
        let workload_index = u64_field(execution, "workload")?;
        let workload = workloads
            .get(workload_index as usize)
            .ok_or_else(|| anyhow!("{name} workload index out of range"))?;
        let workload_id = string_field(workload, "workload_id")?.to_owned();
        let arm_index = u64_field(execution, "arm")?;
        let arm = arms
            .get(arm_index as usize)
            .ok_or_else(|| anyhow!("{name} arm index out of range"))?;
        let arm_name = string_field(arm, "name")?.to_owned();
        let arm_role = string_field(arm, "role")?.to_owned();
        let round = u64_field(execution, "round")?;
        let warmup = execution
            .get("warmup")
            .and_then(Value::as_bool)
            .ok_or_else(|| anyhow!("{name} warmup must be boolean"))?;
        let algorithm_seed = u64_field(execution, "algorithm_seed")?;
        if warmup {
            ensure!(round == 0, "{name} warmup round must be zero");
            inventory.warmups += 1;
        } else {
            ensure!((1..=5).contains(&round), "{name} measured round mismatch");
            inventory.measured += 1;
        }
        let order_group = (workload_id.clone(), warmup, round);
        let order_position = inventory
            .orders
            .get(&order_group)
            .map_or(0, |order| order.len() as u64);
        let key = SlotKey {
            workload_id: workload_id.clone(),
            warmup,
            round,
            arm: arm_name.clone(),
        };
        ensure!(
            inventory
                .slots
                .insert(
                    key,
                    Slot {
                        plan: name.to_owned(),
                        seq,
                        workload_index,
                        algorithm_seed,
                        arm_role,
                        order_position,
                        workload: workload.clone(),
                        arm: arm.clone(),
                    },
                )
                .is_none(),
            "{name} duplicates a scientific slot"
        );
        inventory
            .orders
            .entry(order_group)
            .or_default()
            .push(arm_name);
    }
    for ((workload, _, _), order) in &inventory.orders {
        ensure!(
            order.len() == 3 && order.iter().collect::<BTreeSet<_>>().len() == 3,
            "{name} has an incomplete arm order for {workload}"
        );
    }
    ensure!(
        inventory.slots.len() == inventory.workloads.len() * 18,
        "{name} execution count mismatch"
    );
    Ok(inventory)
}

fn merge_inventory(target: &mut Inventory, source: Inventory) -> Result<()> {
    for (key, value) in source.slots {
        ensure!(
            target.slots.insert(key, value).is_none(),
            "duplicate slot across plans"
        );
    }
    for (key, value) in source.workloads {
        ensure!(
            target.workloads.insert(key.clone(), value).is_none(),
            "duplicate workload across plans: {key}"
        );
    }
    for (key, value) in source.orders {
        ensure!(
            target.orders.insert(key, value).is_none(),
            "duplicate arm-order group"
        );
    }
    target.measured += source.measured;
    target.warmups += source.warmups;
    Ok(())
}

struct Comparison {
    workload_rows: Vec<Value>,
    slot_rows: Vec<Value>,
    enrolled_files: Vec<Value>,
}

fn compare_plans(
    originals: &BTreeMap<String, Value>,
    gate: &Value,
    remainder: &Value,
    plan_hashes: &BTreeMap<String, String>,
    totals: ExpectedTotals,
) -> Result<Comparison> {
    let mut source = Inventory::default();
    for (name, plan) in originals {
        merge_inventory(&mut source, parse_plan(name, plan)?)?;
    }
    let mut enrolled = Inventory::default();
    merge_inventory(&mut enrolled, parse_plan("gate-p739.json", gate)?)?;
    merge_inventory(&mut enrolled, parse_plan(REMAINDER_PLAN_NAME, remainder)?)?;
    for (name, plan) in originals {
        if name != "cohort-16.json" {
            merge_inventory(&mut enrolled, parse_plan(name, plan)?)?;
        }
    }

    ensure!(
        source.workloads.len() == totals.workloads,
        "original workload total mismatch"
    );
    ensure!(
        source.slots.len() == totals.slots,
        "original slot total mismatch"
    );
    ensure!(
        source.measured == totals.measured,
        "original measured total mismatch"
    );
    ensure!(
        source.warmups == totals.warmups,
        "original warmup total mismatch"
    );
    ensure!(
        enrolled.workloads.len() == totals.workloads,
        "enrolled workload total mismatch"
    );
    ensure!(
        enrolled.slots.len() == totals.slots,
        "enrolled slot total mismatch"
    );
    ensure!(
        enrolled.measured == totals.measured,
        "enrolled measured total mismatch"
    );
    ensure!(
        enrolled.warmups == totals.warmups,
        "enrolled warmup total mismatch"
    );
    ensure!(
        source.orders == enrolled.orders,
        "within-round arm order mismatch"
    );
    ensure!(
        source.workloads.keys().eq(enrolled.workloads.keys()),
        "workload membership mismatch"
    );
    ensure!(
        source.slots.keys().eq(enrolled.slots.keys()),
        "slot membership mismatch"
    );
    let expected_hash_names = originals
        .keys()
        .cloned()
        .chain(["gate-p739.json".to_owned(), REMAINDER_PLAN_NAME.to_owned()])
        .collect::<BTreeSet<_>>();
    ensure!(
        plan_hashes.keys().cloned().collect::<BTreeSet<_>>() == expected_hash_names,
        "plan hash inventory mismatch"
    );

    let mut workload_rows = Vec::with_capacity(totals.workloads);
    for (workload_id, original) in &source.workloads {
        let current = &enrolled.workloads[workload_id];
        ensure!(
            original.value == current.value,
            "complete workload mismatch for {workload_id}"
        );
        workload_rows.push(json!({
            "workload_id": workload_id,
            "original_plan": original.plan,
            "enrolled_plan": current.plan,
            "original_plan_sha256": plan_hashes[&original.plan],
            "enrolled_plan_sha256": plan_hashes[&current.plan],
            "original_workload_index": original.index,
            "enrolled_workload_index": current.index,
        }));
    }

    let mut slot_rows = Vec::with_capacity(totals.slots);
    for (key, original) in &source.slots {
        let current = &enrolled.slots[key];
        ensure!(
            original.workload == current.workload,
            "slot workload mismatch"
        );
        ensure!(
            original.arm == current.arm,
            "resolved arm or method mismatch"
        );
        ensure!(
            original.algorithm_seed == current.algorithm_seed,
            "algorithm seed mismatch for {}",
            key.workload_id
        );
        ensure!(
            original.arm_role == current.arm_role,
            "arm role mismatch for {}",
            key.workload_id
        );
        ensure!(
            original.order_position == current.order_position,
            "arm order position mismatch for {}",
            key.workload_id
        );
        slot_rows.push(json!({
            "workload_id": key.workload_id,
            "warmup": key.warmup,
            "round": key.round,
            "arm": key.arm,
            "arm_role": original.arm_role,
            "order_position": original.order_position,
            "algorithm_seed": original.algorithm_seed,
            "original_plan": original.plan,
            "enrolled_plan": current.plan,
            "original_plan_sha256": plan_hashes[&original.plan],
            "enrolled_plan_sha256": plan_hashes[&current.plan],
            "original_seq": original.seq,
            "enrolled_seq": current.seq,
            "original_workload_index": original.workload_index,
            "enrolled_workload_index": current.workload_index,
        }));
    }

    let mut enrolled_files = Vec::new();
    enrolled_files.push(json!({"name":"gate-p739.json","targets":parse_plan("gate-p739.json", gate)?.workloads.len(),"planned_executions":parse_plan("gate-p739.json", gate)?.slots.len()}));
    enrolled_files.push(json!({"name":REMAINDER_PLAN_NAME,"targets":parse_plan(REMAINDER_PLAN_NAME, remainder)?.workloads.len(),"planned_executions":parse_plan(REMAINDER_PLAN_NAME, remainder)?.slots.len()}));
    for (name, plan) in originals {
        if name != "cohort-16.json" {
            let parsed = parse_plan(name, plan)?;
            enrolled_files.push(json!({"name":name,"targets":parsed.workloads.len(),"planned_executions":parsed.slots.len()}));
        }
    }
    Ok(Comparison {
        workload_rows,
        slot_rows,
        enrolled_files,
    })
}

fn jsonl(rows: &[Value]) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for row in rows {
        serde_json::to_writer(&mut bytes, row)?;
        bytes.push(b'\n');
    }
    Ok(bytes)
}

fn enrolled_plan_bindings(
    request: &Request,
    original_plans: &BTreeMap<String, Value>,
    gate_plan: &Value,
    remainder_plan: &Value,
    remainder_plan_sha256: &str,
) -> Result<Vec<Value>> {
    let mut enrolled = Vec::new();
    enrolled.push(json!({
        "name": "gate-p739.json",
        "path": request.gate_plan.path,
        "sha256": request.gate_plan.sha256,
        "spec_id": string_field(gate_plan, "spec_id")?,
        "classification": "prior_gate",
    }));
    enrolled.push(json!({
        "name": REMAINDER_PLAN_NAME,
        "path": REMAINDER_PLAN_NAME,
        "sha256": remainder_plan_sha256,
        "spec_id": string_field(remainder_plan, "spec_id")?,
        "classification": "prospective_remainder",
    }));
    for binding in &request.original_plans {
        if binding.name == "cohort-16.json" {
            continue;
        }
        enrolled.push(json!({
            "name": binding.name,
            "path": binding.path,
            "sha256": binding.sha256,
            "spec_id": string_field(&original_plans[&binding.name], "spec_id")?,
            "classification": "prospective_unchanged_cohort",
        }));
    }
    ensure!(
        enrolled.len() == 8,
        "enrolled plan inventory must have eight members"
    );
    Ok(enrolled)
}

fn original_plan_bindings(
    request: &Request,
    original_plans: &BTreeMap<String, Value>,
) -> Result<Vec<Value>> {
    request
        .original_plans
        .iter()
        .map(|binding| {
            Ok(json!({
                "name": binding.name,
                "path": binding.path,
                "sha256": binding.sha256,
                "spec_id": string_field(&original_plans[&binding.name], "spec_id")?,
            }))
        })
        .collect()
}

fn prepared_receipt_document(
    request: &Request,
    request_sha256: &str,
    comparison: &Comparison,
    enrolled_plans: &[Value],
) -> Value {
    json!({
        "schema": "pencil-native-preparation-validation/v1",
        "passed": true,
        "solver_executions_performed": 0,
        "plan_only_invocations": 1,
        "binary_sha256": request.instrument.binary.sha256,
        "instrument_source_revision": request.instrument.source_revision,
        "request_sha256": request_sha256,
        "cohort_targets": ACTUAL_TOTALS.workloads,
        "cohort_planned_executions": ACTUAL_TOTALS.slots,
        "cohort_measured_executions": ACTUAL_TOTALS.measured,
        "cohort_warmup_executions": ACTUAL_TOTALS.warmups,
        "files": comparison.enrolled_files,
        "enrolled_plans": enrolled_plans,
        "scope": "Native specification derivation, plan-only generation, and structural bijection validation; no solver, cohort, timing, memory, admission, or cloud execution."
    })
}

struct CertificateInputs<'a> {
    request: &'a Request,
    request_sha256: &'a str,
    original_plans: &'a [Value],
    enrolled_plans: &'a [Value],
    remainder_spec_sha256: &'a str,
    remainder_plan_sha256: &'a str,
    workload_mapping: &'a [u8],
    slot_mapping: &'a [u8],
    workloads: usize,
    slots: usize,
}

fn certificate_document(inputs: CertificateInputs<'_>) -> Value {
    let request = inputs.request;
    json!({
        "schema": "pencil-disjoint-enrollment/v1",
        "passed": true,
        "request_sha256": inputs.request_sha256,
        "instrument": {
            "binary": binding_value(&request.instrument.binary),
            "source_revision": request.instrument.source_revision,
            "build_info": binding_value(&request.instrument.build_info),
        },
        "original_specs": request.original_specs.iter().map(binding_value).collect::<Vec<_>>(),
        "original_plans": inputs.original_plans,
        "gate_spec": binding_value(&request.gate_spec),
        "gate_plan": binding_value(&request.gate_plan),
        "enrolled_plans": inputs.enrolled_plans,
        "derived_remainder": {
            "spec_name": REMAINDER_SPEC_NAME,
            "spec_sha256": inputs.remainder_spec_sha256,
            "plan_name": REMAINDER_PLAN_NAME,
            "plan_sha256": inputs.remainder_plan_sha256,
            "label": REMAINDER_LABEL,
            "permitted_spec_changes": ["remove the exact sole p739 gate curve", "replace label with the declared remainder label"],
        },
        "protocol_lineage": request.protocol_lineage.iter().map(binding_value).collect::<Vec<_>>(),
        "mapping": {
            "workloads": inputs.workloads,
            "workload_mapping_file": "workload-mapping.jsonl",
            "workload_mapping_sha256": sha256(inputs.workload_mapping),
            "slots": inputs.slots,
            "slot_mapping_file": "slot-mapping.jsonl",
            "slot_mapping_sha256": sha256(inputs.slot_mapping),
            "original_and_enrolled_sequence_numbers_retained": true,
            "original_and_enrolled_workload_indices_retained": true,
        },
        "checks": {
            "exact_workload_objects": true,
            "exact_resolved_arms_and_methods": true,
            "exact_u64_algorithm_seeds": true,
            "within_round_arm_order": true,
            "duplicate_or_missing_slots": false,
            "gate_and_remainder_overlap": false,
        },
        "upper_gate": "p186379 target0 remains separately charged and is not enrolled by this partition",
        "claims": {
            "scientific_admission": false,
            "timing_qualification": false,
            "launch_authorization": false,
            "empirical_solver_execution": false,
        }
    })
}

fn validate_emitted_documents(
    certificate: &Value,
    receipt: &Value,
    expected_certificate: &Value,
    expected_receipt: &Value,
) -> Result<()> {
    ensure!(
        certificate == expected_certificate,
        "disjoint enrollment certificate differs from native recomputation"
    );
    ensure!(
        receipt == expected_receipt,
        "prepared receipt differs from native recomputation"
    );
    Ok(())
}

pub fn prepare_disjoint_enrollment(request_path: &Path, out: &Path) -> Result<()> {
    let request_bytes =
        fs::read(request_path).with_context(|| format!("read {}", request_path.display()))?;
    let request: Request = serde_json::from_slice(&request_bytes)?;
    ensure!(request.schema == REQUEST_SCHEMA, "request schema mismatch");
    let base = request_path.parent().unwrap_or_else(|| Path::new("."));

    ensure!(
        request.instrument.binary.sha256 == INSTRUMENT_SHA256,
        "instrument hash mismatch"
    );
    ensure!(
        request.instrument.source_revision == INSTRUMENT_SOURCE_REVISION,
        "instrument source revision mismatch"
    );
    ensure!(
        request.instrument.build_info.sha256 == BUILD_INFO_SHA256,
        "build-info hash mismatch"
    );
    let (binary_path, _) = read_bound(base, &request.instrument.binary)?;
    read_bound(base, &request.instrument.build_info)?;

    let original_specs = read_named_json_set(
        base,
        &request.original_specs,
        &expected_map(&ORIGINAL_SPEC_HASHES),
    )?;
    let original_plans = read_named_json_set(
        base,
        &request.original_plans,
        &expected_map(&ORIGINAL_PLAN_HASHES),
    )?;
    ensure!(
        request.gate_spec.name == "gate-p739.json",
        "gate spec name mismatch"
    );
    ensure!(
        request.gate_spec.sha256 == GATE_SPEC_SHA256,
        "gate spec approved hash mismatch"
    );
    ensure!(
        request.gate_plan.name == "gate-p739.json",
        "gate plan name mismatch"
    );
    ensure!(
        request.gate_plan.sha256 == GATE_PLAN_SHA256,
        "gate plan approved hash mismatch"
    );
    let gate_spec = read_bound_json(base, &request.gate_spec)?;
    let gate_plan = read_bound_json(base, &request.gate_plan)?;

    ensure!(
        request.remainder.source_spec == "cohort-16.json",
        "remainder source mismatch"
    );
    ensure!(
        request.remainder.output_spec == REMAINDER_SPEC_NAME,
        "remainder spec name mismatch"
    );
    ensure!(
        request.remainder.output_plan == REMAINDER_PLAN_NAME,
        "remainder plan name mismatch"
    );
    ensure!(
        request.remainder.label == REMAINDER_LABEL,
        "remainder label mismatch"
    );
    ensure!(
        !request.protocol_lineage.is_empty(),
        "protocol lineage must be supplied"
    );
    let mut lineage_names = BTreeSet::new();
    for binding in &request.protocol_lineage {
        ensure!(
            lineage_names.insert(&binding.name),
            "duplicate protocol-lineage name"
        );
        read_bound(base, binding)?;
    }

    for (name, spec) in &original_specs {
        super::validate_prepared_spec(spec, name)?;
        validate_plan_spec_identity(spec, &original_plans[name], name)?;
    }
    super::validate_prepared_spec(&gate_spec, "gate-p739.json")?;
    validate_plan_spec_identity(&gate_spec, &gate_plan, "gate-p739.json")?;
    let remainder_spec = derive_remainder(
        &original_specs["cohort-16.json"],
        &gate_spec,
        REMAINDER_LABEL,
    )?;
    super::validate_prepared_spec(&remainder_spec, REMAINDER_SPEC_NAME)?;

    create_output(out)?;
    write_pretty(&out.join(REMAINDER_SPEC_NAME), &remainder_spec)?;
    let remainder_plan_bytes = run_plan(&binary_path, &out.join(REMAINDER_SPEC_NAME))?;
    let remainder_plan: Value = serde_json::from_slice(&remainder_plan_bytes)?;
    validate_plan_spec_identity(&remainder_spec, &remainder_plan, REMAINDER_PLAN_NAME)?;
    fs::write(out.join(REMAINDER_PLAN_NAME), &remainder_plan_bytes)?;
    let mut plan_hashes = request
        .original_plans
        .iter()
        .map(|binding| (binding.name.clone(), binding.sha256.clone()))
        .collect::<BTreeMap<_, _>>();
    plan_hashes.insert(
        "gate-p739.json".to_owned(),
        request.gate_plan.sha256.clone(),
    );
    plan_hashes.insert(
        REMAINDER_PLAN_NAME.to_owned(),
        sha256(&remainder_plan_bytes),
    );
    let comparison = compare_plans(
        &original_plans,
        &gate_plan,
        &remainder_plan,
        &plan_hashes,
        ACTUAL_TOTALS,
    )?;
    let enrolled_plans = enrolled_plan_bindings(
        &request,
        &original_plans,
        &gate_plan,
        &remainder_plan,
        &plan_hashes[REMAINDER_PLAN_NAME],
    )?;
    let original_plan_bindings = original_plan_bindings(&request, &original_plans)?;

    let workload_bytes = jsonl(&comparison.workload_rows)?;
    let slot_bytes = jsonl(&comparison.slot_rows)?;
    fs::write(out.join("workload-mapping.jsonl"), &workload_bytes)?;
    fs::write(out.join("slot-mapping.jsonl"), &slot_bytes)?;

    let remainder_spec_bytes = fs::read(out.join(REMAINDER_SPEC_NAME))?;
    let request_sha = sha256(&request_bytes);
    let receipt = prepared_receipt_document(&request, &request_sha, &comparison, &enrolled_plans);
    write_pretty(&out.join("prepared-plan-validation.json"), &receipt)?;

    let certificate = certificate_document(CertificateInputs {
        request: &request,
        request_sha256: &request_sha,
        original_plans: &original_plan_bindings,
        enrolled_plans: &enrolled_plans,
        remainder_spec_sha256: &sha256(&remainder_spec_bytes),
        remainder_plan_sha256: &sha256(&remainder_plan_bytes),
        workload_mapping: &workload_bytes,
        slot_mapping: &slot_bytes,
        workloads: comparison.workload_rows.len(),
        slots: comparison.slot_rows.len(),
    });
    write_pretty(&out.join("disjoint-enrollment.json"), &certificate)
}

pub fn verify_disjoint_enrollment(request_path: &Path, out: &Path) -> Result<()> {
    let request_bytes =
        fs::read(request_path).with_context(|| format!("read {}", request_path.display()))?;
    let request: Request = serde_json::from_slice(&request_bytes)?;
    ensure!(request.schema == REQUEST_SCHEMA, "request schema mismatch");
    let base = request_path.parent().unwrap_or_else(|| Path::new("."));
    ensure!(
        request.instrument.binary.sha256 == INSTRUMENT_SHA256,
        "instrument hash mismatch"
    );
    ensure!(
        request.instrument.source_revision == INSTRUMENT_SOURCE_REVISION,
        "instrument revision mismatch"
    );
    ensure!(
        request.instrument.build_info.sha256 == BUILD_INFO_SHA256,
        "build-info hash mismatch"
    );
    read_bound(base, &request.instrument.binary)?;
    read_bound(base, &request.instrument.build_info)?;
    let original_specs = read_named_json_set(
        base,
        &request.original_specs,
        &expected_map(&ORIGINAL_SPEC_HASHES),
    )?;
    let original_plans = read_named_json_set(
        base,
        &request.original_plans,
        &expected_map(&ORIGINAL_PLAN_HASHES),
    )?;
    ensure!(
        request.gate_spec.name == "gate-p739.json",
        "gate spec name mismatch"
    );
    ensure!(
        request.gate_plan.name == "gate-p739.json",
        "gate plan name mismatch"
    );
    ensure!(
        request.gate_spec.sha256 == GATE_SPEC_SHA256,
        "gate spec hash mismatch"
    );
    ensure!(
        request.gate_plan.sha256 == GATE_PLAN_SHA256,
        "gate plan hash mismatch"
    );
    ensure!(
        request.remainder.source_spec == "cohort-16.json",
        "remainder source mismatch"
    );
    ensure!(
        request.remainder.output_spec == REMAINDER_SPEC_NAME,
        "remainder spec name mismatch"
    );
    ensure!(
        request.remainder.output_plan == REMAINDER_PLAN_NAME,
        "remainder plan name mismatch"
    );
    ensure!(
        request.remainder.label == REMAINDER_LABEL,
        "remainder label mismatch"
    );
    let gate_spec = read_bound_json(base, &request.gate_spec)?;
    let gate_plan = read_bound_json(base, &request.gate_plan)?;
    ensure!(
        !request.protocol_lineage.is_empty(),
        "protocol lineage must be supplied"
    );
    let mut lineage_names = BTreeSet::new();
    for binding in &request.protocol_lineage {
        ensure!(
            lineage_names.insert(&binding.name),
            "duplicate protocol-lineage name"
        );
        read_bound(base, binding)?;
    }

    let remainder_spec_bytes = fs::read(out.join(REMAINDER_SPEC_NAME))?;
    let remainder_spec: Value = serde_json::from_slice(&remainder_spec_bytes)?;
    validate_partition_specs(
        &original_specs["cohort-16.json"],
        &gate_spec,
        &remainder_spec,
        REMAINDER_LABEL,
    )?;
    let remainder_plan_bytes = fs::read(out.join(REMAINDER_PLAN_NAME))?;
    let remainder_plan: Value = serde_json::from_slice(&remainder_plan_bytes)?;
    for (name, spec) in &original_specs {
        validate_plan_spec_identity(spec, &original_plans[name], name)?;
    }
    validate_plan_spec_identity(&gate_spec, &gate_plan, "gate-p739.json")?;
    validate_plan_spec_identity(&remainder_spec, &remainder_plan, REMAINDER_PLAN_NAME)?;
    let mut plan_hashes = request
        .original_plans
        .iter()
        .map(|binding| (binding.name.clone(), binding.sha256.clone()))
        .collect::<BTreeMap<_, _>>();
    plan_hashes.insert(
        "gate-p739.json".to_owned(),
        request.gate_plan.sha256.clone(),
    );
    plan_hashes.insert(
        REMAINDER_PLAN_NAME.to_owned(),
        sha256(&remainder_plan_bytes),
    );
    let comparison = compare_plans(
        &original_plans,
        &gate_plan,
        &remainder_plan,
        &plan_hashes,
        ACTUAL_TOTALS,
    )?;
    let expected_workloads = jsonl(&comparison.workload_rows)?;
    let expected_slots = jsonl(&comparison.slot_rows)?;
    ensure!(
        fs::read(out.join("workload-mapping.jsonl"))? == expected_workloads,
        "workload mapping does not match native recomputation"
    );
    ensure!(
        fs::read(out.join("slot-mapping.jsonl"))? == expected_slots,
        "slot mapping does not match native recomputation"
    );

    let certificate: Value =
        serde_json::from_slice(&fs::read(out.join("disjoint-enrollment.json"))?)?;
    exact_keys(
        &certificate,
        &[
            "checks",
            "claims",
            "derived_remainder",
            "enrolled_plans",
            "gate_plan",
            "gate_spec",
            "instrument",
            "mapping",
            "original_plans",
            "original_specs",
            "passed",
            "protocol_lineage",
            "request_sha256",
            "schema",
            "upper_gate",
        ],
        "disjoint enrollment certificate",
    )?;
    ensure!(
        certificate.get("schema").and_then(Value::as_str) == Some("pencil-disjoint-enrollment/v1")
            && certificate.get("passed").and_then(Value::as_bool) == Some(true),
        "certificate status mismatch"
    );
    ensure!(
        certificate.get("request_sha256").and_then(Value::as_str) == Some(&sha256(&request_bytes)),
        "certificate request binding mismatch"
    );
    ensure!(
        certificate
            .pointer("/mapping/workloads")
            .and_then(Value::as_u64)
            == Some(ACTUAL_TOTALS.workloads as u64)
            && certificate
                .pointer("/mapping/slots")
                .and_then(Value::as_u64)
                == Some(ACTUAL_TOTALS.slots as u64)
            && certificate
                .pointer("/mapping/workload_mapping_sha256")
                .and_then(Value::as_str)
                == Some(&sha256(&expected_workloads))
            && certificate
                .pointer("/mapping/slot_mapping_sha256")
                .and_then(Value::as_str)
                == Some(&sha256(&expected_slots)),
        "certificate mapping binding mismatch"
    );
    ensure!(
        certificate
            .pointer("/derived_remainder/spec_sha256")
            .and_then(Value::as_str)
            == Some(&sha256(&remainder_spec_bytes))
            && certificate
                .pointer("/derived_remainder/plan_sha256")
                .and_then(Value::as_str)
                == Some(&sha256(&remainder_plan_bytes)),
        "certificate remainder binding mismatch"
    );
    ensure!(
        certificate.get("derived_remainder")
            == Some(&json!({
                "spec_name": REMAINDER_SPEC_NAME,
                "spec_sha256": sha256(&remainder_spec_bytes),
                "plan_name": REMAINDER_PLAN_NAME,
                "plan_sha256": sha256(&remainder_plan_bytes),
                "label": REMAINDER_LABEL,
                "permitted_spec_changes": [
                    "remove the exact sole p739 gate curve",
                    "replace label with the declared remainder label"
                ],
            })),
        "certificate remainder declaration mismatch"
    );
    ensure!(
        certificate.get("mapping")
            == Some(&json!({
                "workloads": ACTUAL_TOTALS.workloads,
                "workload_mapping_file": "workload-mapping.jsonl",
                "workload_mapping_sha256": sha256(&expected_workloads),
                "slots": ACTUAL_TOTALS.slots,
                "slot_mapping_file": "slot-mapping.jsonl",
                "slot_mapping_sha256": sha256(&expected_slots),
                "original_and_enrolled_sequence_numbers_retained": true,
                "original_and_enrolled_workload_indices_retained": true,
            })),
        "certificate mapping declaration mismatch"
    );
    let expected_enrolled = enrolled_plan_bindings(
        &request,
        &original_plans,
        &gate_plan,
        &remainder_plan,
        &plan_hashes[REMAINDER_PLAN_NAME],
    )?;
    ensure!(
        certificate.get("enrolled_plans") == Some(&Value::Array(expected_enrolled.clone())),
        "certificate enrolled plan inventory mismatch"
    );
    let expected_original = original_plan_bindings(&request, &original_plans)?;
    ensure!(
        certificate.get("original_plans") == Some(&Value::Array(expected_original.clone())),
        "certificate original plan inventory mismatch"
    );
    ensure!(
        certificate.get("original_specs")
            == Some(&Value::Array(
                request.original_specs.iter().map(binding_value).collect()
            ))
            && certificate.get("gate_spec") == Some(&binding_value(&request.gate_spec))
            && certificate.get("gate_plan") == Some(&binding_value(&request.gate_plan))
            && certificate.get("protocol_lineage")
                == Some(&Value::Array(
                    request.protocol_lineage.iter().map(binding_value).collect()
                )),
        "certificate input binding mismatch"
    );
    ensure!(
        certificate.get("instrument")
            == Some(&json!({
                "binary": binding_value(&request.instrument.binary),
                "source_revision": request.instrument.source_revision,
                "build_info": binding_value(&request.instrument.build_info),
            })),
        "certificate instrument binding mismatch"
    );
    ensure!(
        certificate.get("checks")
            == Some(&json!({
                "exact_workload_objects": true,
                "exact_resolved_arms_and_methods": true,
                "exact_u64_algorithm_seeds": true,
                "within_round_arm_order": true,
                "duplicate_or_missing_slots": false,
                "gate_and_remainder_overlap": false,
            }))
            && certificate.get("claims")
                == Some(&json!({
                    "scientific_admission": false,
                    "timing_qualification": false,
                    "launch_authorization": false,
                    "empirical_solver_execution": false,
                }))
            && certificate.get("upper_gate")
                == Some(&json!("p186379 target0 remains separately charged and is not enrolled by this partition")),
        "certificate claim boundary mismatch"
    );

    let receipt: Value =
        serde_json::from_slice(&fs::read(out.join("prepared-plan-validation.json"))?)?;
    exact_keys(
        &receipt,
        &[
            "binary_sha256",
            "cohort_measured_executions",
            "cohort_planned_executions",
            "cohort_targets",
            "cohort_warmup_executions",
            "enrolled_plans",
            "files",
            "instrument_source_revision",
            "passed",
            "plan_only_invocations",
            "request_sha256",
            "schema",
            "scope",
            "solver_executions_performed",
        ],
        "prepared plan receipt",
    )?;
    let request_sha = sha256(&request_bytes);
    let expected_receipt =
        prepared_receipt_document(&request, &request_sha, &comparison, &expected_enrolled);
    let expected_certificate = certificate_document(CertificateInputs {
        request: &request,
        request_sha256: &request_sha,
        original_plans: &expected_original,
        enrolled_plans: &expected_enrolled,
        remainder_spec_sha256: &sha256(&remainder_spec_bytes),
        remainder_plan_sha256: &sha256(&remainder_plan_bytes),
        workload_mapping: &expected_workloads,
        slot_mapping: &expected_slots,
        workloads: comparison.workload_rows.len(),
        slots: comparison.slot_rows.len(),
    });
    validate_emitted_documents(
        &certificate,
        &receipt,
        &expected_certificate,
        &expected_receipt,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn arm(name: &str, role: &str, method: &str) -> Value {
        json!({"name":name,"role":role,"method":{"id":method,"params":{"x":"1"},"method_sha256":method}})
    }

    fn workload(id: &str, p: u64) -> Value {
        json!({"workload_id":id,"curve_spec":{"p":p},"target_seed":9,"target_index":0,"target":["0x1","0x2"]})
    }

    fn plan(name: &str, workloads: Vec<Value>, seed_base: u64) -> Value {
        let arms = vec![
            arm("pencil", "candidate", "candidate"),
            arm("rho", "reference", "rho"),
            arm("rho_AA", "control", "rho"),
        ];
        let mut executions = Vec::new();
        let mut seq = 0_u64;
        for (workload_index, _) in workloads.iter().enumerate() {
            for (warmup, round) in
                std::iter::once((true, 0)).chain((1..=5).map(|round| (false, round)))
            {
                for arm_index in [2_u64, 0, 1] {
                    executions.push(json!({
                        "seq":seq,"round":round,"warmup":warmup,"arm":arm_index,
                        "workload":workload_index as u64,
                        "algorithm_seed":seed_base.wrapping_add((workload_index as u64)*10).wrapping_add(round)
                    }));
                    seq += 1;
                }
            }
        }
        json!({"schema":PLAN_SCHEMA,"spec_id":name,"arms":arms,"workloads":workloads,"executions":executions})
    }

    fn fixture() -> (BTreeMap<String, Value>, Value, Value) {
        let mut originals = BTreeMap::new();
        let p739 = workload("W739", 739);
        let p1151 = workload("W1151", 1151);
        originals.insert(
            "cohort-16.json".into(),
            plan("c16", vec![p739.clone(), p1151.clone()], u64::MAX - 100),
        );
        for (offset, bits) in [20, 24, 28, 32, 36, 40].into_iter().enumerate() {
            originals.insert(
                format!("cohort-{bits}.json"),
                plan(
                    &format!("c{bits}"),
                    vec![workload(&format!("W{bits}"), bits)],
                    1000 + offset as u64,
                ),
            );
        }
        let gate = plan("gate", vec![p739], u64::MAX - 100);
        let remainder = plan("remainder", vec![p1151], u64::MAX - 90);
        (originals, gate, remainder)
    }

    fn totals() -> ExpectedTotals {
        ExpectedTotals {
            workloads: 8,
            slots: 144,
            measured: 120,
            warmups: 24,
        }
    }

    fn hashes(
        originals: &BTreeMap<String, Value>,
        gate: &Value,
        remainder: &Value,
    ) -> BTreeMap<String, String> {
        let mut hashes = originals
            .iter()
            .map(|(name, plan)| (name.clone(), sha256(&serde_json::to_vec(plan).unwrap())))
            .collect::<BTreeMap<_, _>>();
        hashes.insert(
            "gate-p739.json".into(),
            sha256(&serde_json::to_vec(gate).unwrap()),
        );
        hashes.insert(
            REMAINDER_PLAN_NAME.into(),
            sha256(&serde_json::to_vec(remainder).unwrap()),
        );
        hashes
    }

    fn specs() -> (Value, Value, Value) {
        let source = json!({
            "schema":"ecbench.spec/v1","label":"cohort16","description":"x",
            "arms":[],"measurement":{},
            "workloads":{"target_seed":9,"targets_per_curve":60,"curves":[{"p":739,"a":1},{"p":1151,"a":2}]}
        });
        let mut gate = source.clone();
        *gate.get_mut("label").unwrap() = json!("gate");
        gate.pointer_mut("/workloads/curves")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .retain(|curve| curve["p"] == 739);
        let remainder = derive_remainder(&source, &gate, REMAINDER_LABEL).unwrap();
        (source, gate, remainder)
    }

    #[test]
    fn exact_partition_preserves_high_bit_u64_and_all_slots() {
        let (originals, gate, remainder) = fixture();
        let compared = compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals(),
        )
        .unwrap();
        assert_eq!(compared.workload_rows.len(), 8);
        assert_eq!(compared.slot_rows.len(), 144);
        assert!(compared
            .slot_rows
            .iter()
            .any(|row| row["algorithm_seed"].as_u64().unwrap() > i64::MAX as u64));
    }

    #[test]
    fn rejects_missing_and_overlapping_slots() {
        let (originals, gate, mut remainder) = fixture();
        remainder["executions"].as_array_mut().unwrap().pop();
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());

        let (originals, gate, mut remainder) = fixture();
        remainder["workloads"].as_array_mut().unwrap()[0] = workload("W739", 739);
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());
    }

    #[test]
    fn rejects_changed_method_seed_order_and_unknown_execution_field() {
        let (originals, mut gate, remainder) = fixture();
        gate["arms"][0]["method"]["id"] = json!("changed");
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());

        let (originals, mut gate, remainder) = fixture();
        gate["executions"][0]["algorithm_seed"] = json!(7);
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());

        let (originals, mut gate, remainder) = fixture();
        gate["executions"].as_array_mut().unwrap().swap(0, 1);
        gate["executions"][0]["seq"] = json!(0);
        gate["executions"][1]["seq"] = json!(1);
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());

        let (originals, mut gate, remainder) = fixture();
        gate["executions"][0]["future"] = json!(true);
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());
    }

    #[test]
    fn rejects_duplicate_and_wrong_schema() {
        let (originals, mut gate, remainder) = fixture();
        gate["executions"][1] = gate["executions"][0].clone();
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());

        let (originals, mut gate, remainder) = fixture();
        gate["schema"] = json!("ecbench.plan/v2");
        assert!(compare_plans(
            &originals,
            &gate,
            &remainder,
            &hashes(&originals, &gate, &remainder),
            totals()
        )
        .is_err());
    }

    #[test]
    fn remainder_allows_only_exact_curve_removal_and_label() {
        let (source, gate, remainder) = specs();
        validate_partition_specs(&source, &gate, &remainder, REMAINDER_LABEL).unwrap();
        let mut changed = remainder.clone();
        changed["description"] = json!("unauthorized");
        assert!(validate_partition_specs(&source, &gate, &changed, REMAINDER_LABEL).is_err());
        let mut bad_gate = gate.clone();
        bad_gate["measurement"]["rounds"] = json!(6);
        assert!(validate_partition_specs(&source, &bad_gate, &remainder, REMAINDER_LABEL).is_err());
    }

    #[test]
    fn bound_file_rejects_changed_hash_and_malformed_digest() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("bound.json"), b"{}\n").unwrap();
        let mut binding = FileBinding {
            name: "bound".into(),
            path: "bound.json".into(),
            sha256: "0".repeat(64),
        };
        assert!(read_bound(temp.path(), &binding).is_err());
        binding.sha256 = "ABC".into();
        assert!(read_bound(temp.path(), &binding).is_err());
    }

    #[test]
    fn typed_spec_identity_matches_all_pinned_vectors_and_rejects_arbitrary_plan_id() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let vectors = [
            ("cohort-16.json", "ECS1habc603bf5511"),
            ("cohort-20.json", "ECS1h2bd191253bbd"),
            ("cohort-24.json", "ECS1heaff76e306f7"),
            ("cohort-28.json", "ECS1he4c8727d2fc2"),
            ("cohort-32.json", "ECS1hdfe262979cdb"),
            ("cohort-36.json", "ECS1hcd32ffa92b64"),
            ("cohort-40.json", "ECS1hf76a63f60392"),
            ("gate-p739.json", "ECS1h1bab8ffa33b6"),
        ];
        for (name, expected) in vectors {
            let spec: Value = serde_json::from_slice(
                &fs::read(root.join("../prepared_specs").join(name)).unwrap(),
            )
            .unwrap();
            let (observed, digest) = spec_identity(&spec).unwrap();
            assert_eq!(observed, expected, "identity mismatch for {name}");
            assert_eq!(digest.len(), 64);
            validate_plan_spec_identity(&spec, &json!({"spec_id":expected}), name).unwrap();
            assert!(validate_plan_spec_identity(
                &spec,
                &json!({"spec_id":"ECS1h000000000000"}),
                name
            )
            .is_err());

            let mut relabelled = spec.clone();
            relabelled["label"] = json!("different-label");
            assert_eq!(spec_identity(&relabelled).unwrap(), (observed, digest));
        }
    }

    #[test]
    fn public_document_validator_rejects_certificate_and_receipt_tampering() {
        let certificate = json!({
            "instrument":{"binary":{"path":"/approved/ecbench","sha256":"abc"}},
            "mapping":{"slot_mapping_sha256":"slots"},
            "claims":{"scientific_admission":false}
        });
        let receipt = json!({
            "scope":"plan-only; no solver or admission",
            "plan_only_invocations":1,
            "enrolled_plans":["gate","remainder"]
        });
        validate_emitted_documents(&certificate, &receipt, &certificate, &receipt).unwrap();

        let mut changed = certificate.clone();
        changed["instrument"]["binary"]["path"] = json!("/substituted/ecbench");
        assert!(validate_emitted_documents(&changed, &receipt, &certificate, &receipt).is_err());

        let mut extra = certificate.clone();
        extra["instrument"]["unexpected"] = json!(true);
        assert!(validate_emitted_documents(&extra, &receipt, &certificate, &receipt).is_err());

        let mut changed = certificate.clone();
        changed["mapping"]["slot_mapping_sha256"] = json!("changed");
        assert!(validate_emitted_documents(&changed, &receipt, &certificate, &receipt).is_err());

        let mut changed = certificate.clone();
        changed["claims"]["scientific_admission"] = json!(true);
        assert!(validate_emitted_documents(&changed, &receipt, &certificate, &receipt).is_err());

        let mut changed_receipt = receipt.clone();
        changed_receipt["scope"] = json!("admitted");
        assert!(
            validate_emitted_documents(&certificate, &changed_receipt, &certificate, &receipt)
                .is_err()
        );
    }
}
