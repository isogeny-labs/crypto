//! Independent binding audit and conservative public routing projection.
//!
//! Positive certificate arithmetic is optional and reported separately. Neither
//! artifact hashes nor a successful positive replay certify UNSAT, costs, orders,
//! construction provenance, or fall-degree claims.

use anyhow::{anyhow, ensure, Context, Result};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::common::{
    bounded_bytes, check_artifact, contained, digest, load_artifact_json, read_json_bytes,
};

const PLAN_FIELDS: [&str; 15] = [
    "schema",
    "field_sizes",
    "arities",
    "families",
    "solvers",
    "curve_seeds",
    "generators",
    "keys",
    "seed",
    "factor_base_size",
    "degree_bound",
    "work_limit",
    "twists",
    "protocol_sha256",
    "split_rule",
];
const REQUIRED: [&str; 8] = [
    "config.json",
    "run.json",
    "plan.json",
    "protocol.json",
    "attempt.json",
    "worker-process.json",
    "stdout.log",
    "stderr.log",
];
const SUCCESS_ROLES: [&str; 2] = [
    "restricted-independent-scalar-label",
    "independently-verified-pdp",
];

fn uint(value: &Value, name: &str) -> Result<u64> {
    value.as_u64().with_context(|| format!("invalid {name}"))
}

fn text<'a>(value: &'a Value, name: &str) -> Result<&'a str> {
    value.as_str().with_context(|| format!("invalid {name}"))
}

fn array<'a>(value: &'a Value, name: &str) -> Result<&'a Vec<Value>> {
    value.as_array().with_context(|| format!("invalid {name}"))
}

fn hex(value: &Value, length: usize) -> bool {
    value.as_str().is_some_and(|s| {
        s.len() == length
            && s.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

/// Parsing and hashing use one bounded read, including the manifest identity.
fn document(path: &Path) -> Result<(Value, Vec<u8>)> {
    let bytes = bounded_bytes(path)?;
    let value = read_json_bytes(&bytes)?;
    ensure!(
        value.is_object(),
        "expected JSON object: {}",
        path.display()
    );
    Ok((value, bytes))
}

fn validate_plan(plan: &Value) -> Result<u64> {
    let object = plan.as_object().context("plan must be an object")?;
    let versioned = plan["schema"] == "crypto.ml-dataset.plan/v2";
    ensure!(
        object.len() == PLAN_FIELDS.len() + usize::from(versioned)
            && PLAN_FIELDS.iter().all(|key| object.contains_key(*key))
            && (!versioned || object.contains_key("factor_base_policies")),
        "unknown/missing plan fields"
    );
    ensure!(
        (plan["schema"] == "crypto.ml-dataset.plan/v1" || versioned) && plan["twists"] == true,
        "unsupported plan schema/twist policy"
    );
    ensure!(
        plan["split_rule"] == "curve-twist-component-sha256-v1",
        "unknown split rule"
    );
    let mut total = 2u64;
    for key in ["families", "field_sizes", "arities", "solvers"] {
        let values = array(&plan[key], key)?;
        ensure!(!values.is_empty(), "empty plan axis: {key}");
        let mut unique = BTreeSet::new();
        for value in values {
            ensure!(
                unique.insert(serde_json::to_string(value)?),
                "duplicate plan axis: {key}"
            );
        }
        total = total
            .checked_mul(u64::try_from(values.len())?)
            .context("plan count overflow")?;
    }
    for n in array(&plan["field_sizes"], "field sizes")? {
        ensure!(
            (3..=61).contains(&uint(n, "field size")?),
            "invalid field sizes"
        );
    }
    if versioned {
        let policies = array(&plan["factor_base_policies"], "policy axis")?;
        ensure!(
            (1..=32).contains(&policies.len()),
            "invalid policy axis length"
        );
        let mut unique = BTreeSet::new();
        for policy in policies {
            let mut canonical = Value::Null;
            for n in array(&plan["field_sizes"], "field sizes")? {
                canonical = crate::policy::recipe(policy, uint(n, "field size")?)?;
            }
            ensure!(
                unique.insert(serde_json::to_vec(&canonical)?),
                "duplicate policy recipe"
            );
        }
        total = total
            .checked_mul(policies.len() as u64)
            .context("plan count overflow")?;
    }
    for m in array(&plan["arities"], "arities")? {
        ensure!((2..=15).contains(&uint(m, "arity")?), "invalid arities");
    }
    for family in array(&plan["families"], "families")? {
        ensure!(
            matches!(family.as_str(), Some("koblitz" | "binary" | "prime")),
            "unknown family"
        );
    }
    for solver in array(&plan["solvers"], "solvers")? {
        ensure!(!text(solver, "solver")?.is_empty(), "empty solver");
        if versioned {
            ensure!(
                matches!(
                    solver.as_str(),
                    Some(
                        "enumerate"
                            | "pdp"
                            | "bsgs"
                            | "sat"
                            | "weil"
                            | "weil-descent"
                            | "f4"
                            | "f5"
                            | "f6"
                            | "root"
                            | "pkm"
                            | "double-large-prime"
                    )
                ),
                "unknown versioned-plan solver"
            );
        }
    }
    for key in [
        "curve_seeds",
        "generators",
        "keys",
        "factor_base_size",
        "degree_bound",
        "work_limit",
    ] {
        ensure!(uint(&plan[key], key)? > 0, "invalid {key}");
    }
    for key in ["curve_seeds", "generators", "keys"] {
        total = total
            .checked_mul(uint(&plan[key], key)?)
            .context("plan count overflow")?;
    }
    if versioned {
        ensure!(
            (2..=4096).contains(&uint(&plan["factor_base_size"], "factor-base size")?)
                && (1..=32).contains(&uint(&plan["degree_bound"], "degree bound")?),
            "versioned plan resource bounds"
        );
    }
    uint(&plan["seed"], "seed")?;
    ensure!(
        hex(&plan["protocol_sha256"], 64),
        "invalid protocol identity"
    );
    Ok(total)
}

pub(crate) fn canonical_plan(plan: &Value) -> Result<Value> {
    validate_plan(plan)?;
    let mut ordered = Map::new();
    for key in PLAN_FIELDS {
        ordered.insert(key.into(), plan[key].clone());
    }
    if plan["schema"] == "crypto.ml-dataset.plan/v2" {
        let n = uint(&plan["field_sizes"][0], "field size")?;
        let policies = array(&plan["factor_base_policies"], "policy axis")?
            .iter()
            .map(|p| crate::policy::recipe(p, n))
            .collect::<Result<Vec<_>>>()?;
        ordered.insert("factor_base_policies".into(), Value::Array(policies));
    }
    Ok(Value::Object(ordered))
}

struct PlanInfo {
    plan: Value,
    plan_sha: String,
    binding: Value,
    run_sha: String,
    total: u64,
    protocol_preflight_reserved: usize,
}

fn plan_info(root: &Path) -> Result<PlanInfo> {
    let (plan, _) = document(&contained(root, "plan.json")?)?;
    let total = validate_plan(&plan)?;
    let bytes = bounded_bytes(&contained(root, "protocol.json")?)?;
    ensure!(
        digest(&bytes) == text(&plan["protocol_sha256"], "protocol hash")?,
        "protocol hash mismatch"
    );
    let protocol_preflight_reserved = match read_json_bytes(&bytes) {
        Ok(protocol) => {
            ensure!(
                protocol["schema"] == "experiment.protocol/v1",
                "protocol schema"
            );
            0
        }
        Err(error)
            if error
                .downcast_ref::<serde_json::Error>()
                .is_some_and(|e| e.to_string().starts_with("number out of range")) =>
        {
            crate::phase::verify_large_protocol(&bytes)?
        }
        Err(error) => return Err(error),
    };
    // Producer DatasetPlan is a struct: JSON field order is part of its digest.
    // serde_json's preserve_order feature is required by the parent crate.
    let plan_sha = digest(&serde_json::to_vec(&canonical_plan(&plan)?)?);
    let (binding, bytes) = document(&contained(root, "run.json")?)?;
    ensure!(
        binding["schema"] == "crypto.ml-dataset.run/v1" && binding["plan_sha256"] == plan_sha,
        "run/plan binding mismatch"
    );
    ensure!(
        hex(&binding["binary_sha256"], 64),
        "invalid binary identity"
    );
    ensure!(
        hex(&binding["source_commit"], 40),
        "invalid source identity"
    );
    let shards = uint(&binding["shard_count"], "shard count")?;
    ensure!(
        shards > 0 && uint(&binding["shard_index"], "shard index")? < shards,
        "invalid shard"
    );
    Ok(PlanInfo {
        plan,
        plan_sha,
        binding,
        run_sha: digest(&bytes),
        total,
        protocol_preflight_reserved,
    })
}

/// Exact producer mixed-radix index without allocating ranges for large axes.
pub fn config_at(plan: &Value, mut index: u64) -> Result<Value> {
    let total = validate_plan(plan)?;
    ensure!(index < total, "cell index outside plan");
    let mut config = Map::new();
    for key in ["seed", "factor_base_size", "degree_bound", "work_limit"] {
        config.insert(key.into(), plan[key].clone());
    }
    for (key, axis) in [
        ("solver", "solvers"),
        ("factor_base_policy", "factor_base_policies"),
        ("twist", ""),
        ("m", "arities"),
        ("key_index", "keys"),
        ("generator_index", "generators"),
        ("curve_seed", "curve_seeds"),
        ("n", "field_sizes"),
        ("family", "families"),
    ] {
        if key == "factor_base_policy" && plan["schema"] == "crypto.ml-dataset.plan/v1" {
            continue;
        }
        let value = if key == "twist" {
            let value = Value::Bool(index % 2 == 1);
            index /= 2;
            value
        } else if matches!(key, "key_index" | "generator_index" | "curve_seed") {
            let size = uint(&plan[axis], axis)?;
            let value = json!(index % size);
            index /= size;
            value
        } else {
            let values = array(&plan[axis], axis)?;
            let size = u64::try_from(values.len())?;
            let value = values[usize::try_from(index % size)?].clone();
            index /= size;
            value
        };
        config.insert(key.into(), value);
    }
    ensure!(index == 0, "cell index outside plan");
    Ok(Value::Object(config))
}

fn optional_counter(value: &Value) -> Result<Option<u64>> {
    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(uint(value, "hardware counter")?))
    }
}

fn validate_metrics(metrics: &Value) -> Result<()> {
    let hardware = metrics["hardware"]
        .as_object()
        .context("missing hardware record")?;
    let get = |key: &str| hardware.get(key).unwrap_or(&Value::Null);
    let mut any_counter = false;
    let mut expected_mux = false;
    for event in ["instructions", "cycles"] {
        let raw = optional_counter(get(event))?;
        let enabled = optional_counter(get(&format!("{event}_time_enabled_ns")))?;
        let running = optional_counter(get(&format!("{event}_time_running_ns")))?;
        let actual_scaled = optional_counter(get(&format!("{event}_scaled")))?;
        let scaled = match (raw, enabled, running) {
            (Some(raw), Some(enabled), Some(running)) => {
                any_counter = true;
                ensure!(running <= enabled, "invalid PMU running/enabled time");
                expected_mux |= running < enabled;
                if running == enabled {
                    Some(raw)
                } else if running > 0 {
                    Some(
                        (u128::from(raw) * u128::from(enabled) / u128::from(running))
                            .min(u128::from(u64::MAX)) as u64,
                    )
                } else {
                    None
                }
            }
            (None, None, None) => None,
            _ => return Err(anyhow!("incomplete {event} counter tuple")),
        };
        ensure!(actual_scaled == scaled, "inconsistent {event} scaling");
    }
    let multiplexed = get("multiplexed");
    ensure!(
        multiplexed.is_null() || multiplexed.is_boolean(),
        "invalid multiplexing flag"
    );
    ensure!(
        multiplexed.as_bool() == any_counter.then_some(expected_mux),
        "inconsistent aggregate multiplexing flag"
    );
    for key in ["inherit", "grouped"] {
        ensure!(
            get(key).is_null() || get(key).is_boolean(),
            "invalid counter {key} flag"
        );
    }
    let raw = optional_counter(get("instructions"))?;
    let running = optional_counter(get("instructions_time_running_ns"))?;
    let usable = raw.is_some() && running.is_some_and(|n| n > 0);
    let status = if !usable {
        "unavailable"
    } else if multiplexed == false {
        "measured_unmultiplexed"
    } else {
        "measured_multiplexed_estimate"
    };
    ensure!(
        metrics["instructions_status"] == status,
        "instruction status contradicts hardware"
    );
    let raw_label = optional_counter(&metrics["retired_instructions_raw"])?;
    let exact_label = optional_counter(&metrics["retired_instructions_exact_in_scope"])?;
    ensure!(
        raw_label == if usable { raw } else { None },
        "raw instruction label mismatch"
    );
    ensure!(
        exact_label
            == if usable && multiplexed == false {
                raw
            } else {
                None
            },
        "exact instruction label mismatch"
    );
    ensure!(
        metrics["clock_valid_for_comparison"] == false,
        "unsupported comparative clock claim"
    );
    ensure!(
        !text(&metrics["counter_scope"], "counter scope")?.is_empty(),
        "missing counter scope/exclusions"
    );
    let excludes = array(&metrics["excludes"], "counter exclusions")?;
    ensure!(
        !excludes.is_empty()
            && excludes
                .iter()
                .all(|x| x.as_str().is_some_and(|s| !s.is_empty())),
        "missing counter scope/exclusions"
    );
    let returned = metrics["returned_ok"]
        .as_bool()
        .context("missing phase completion status")?;
    ensure!(
        metrics["returned_error"].is_null() || metrics["returned_error"].is_string(),
        "invalid phase error"
    );
    ensure!(
        metrics["returned_error"].is_null() == returned,
        "phase error/status mismatch"
    );
    ensure!(
        metrics["interrupted_phase_readback"] == false,
        "unsupported interrupted phase readback"
    );
    Ok(())
}

fn phase_measurements(root: &Path, members: &BTreeMap<String, Value>) -> Result<Value> {
    let mut starts = BTreeMap::new();
    let mut ends = BTreeMap::new();
    for artifact in members.values() {
        let role = text(&artifact["role"], "artifact role")?;
        if !matches!(role, "phase-start" | "hardware-and-resource-telemetry") {
            continue;
        }
        let record = load_artifact_json(root, artifact)?;
        let phase = text(&record["phase"], "telemetry phase")?.to_owned();
        ensure!(
            !phase.is_empty()
                && phase.len() <= 64
                && phase.as_bytes()[0].is_ascii_lowercase()
                && phase
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
            "invalid telemetry phase"
        );
        let start = role == "phase-start";
        let expected = if start {
            "crypto.ml-dataset.phase-start/v1"
        } else {
            "crypto.ml-dataset.phase-metrics/v1"
        };
        let suffix = if start { "-start" } else { "" };
        ensure!(
            record["schema"] == expected
                && artifact["path"] == format!("telemetry/{phase}{suffix}.json"),
            "telemetry schema/path mismatch"
        );
        let target = if start { &mut starts } else { &mut ends };
        ensure!(
            target
                .insert(phase, (record, artifact["sha256"].clone()))
                .is_none(),
            "duplicate telemetry phase"
        );
    }
    ensure!(
        ends.keys().all(|key| starts.contains_key(key)),
        "phase metrics lack durable start boundary"
    );
    let mut result = Vec::new();
    for (phase, (start, start_sha)) in starts {
        let mut entry = json!({"phase":phase,"start_sha256":start_sha,"start":start,
            "metrics_sha256":null,"producer_reported":null,"measurement_state":"interrupted_or_unfinished",
            "independently_verified":false});
        if let Some((metrics, sha)) = ends.get(&phase) {
            validate_metrics(metrics)?;
            entry["metrics_sha256"] = sha.clone();
            entry["producer_reported"] = metrics.clone();
            entry["measurement_state"] = json!("reported_phase_readback");
        }
        result.push(entry);
    }
    Ok(Value::Array(result))
}

fn audit_cell(root: &Path, info: &PlanInfo, arithmetic: bool) -> Result<Value> {
    let (manifest, manifest_bytes) = document(&contained(root, "manifest.json")?)?;
    ensure!(
        manifest["schema"] == "crypto.ml-dataset.cell/v1",
        "cell schema"
    );
    let index = uint(&manifest["cell_index"], "cell index")?;
    ensure!(
        root.file_name().and_then(|s| s.to_str()) == Some(format!("{index:012}").as_str()),
        "cell directory/index"
    );
    ensure!(
        index % uint(&info.binding["shard_count"], "shard count")?
            == uint(&info.binding["shard_index"], "shard index")?,
        "cell outside shard"
    );
    let config = config_at(&info.plan, index)?;
    ensure!(
        manifest["plan_sha256"] == info.plan_sha && manifest["config"] == config,
        "cell/plan config"
    );
    let mut members = BTreeMap::new();
    for artifact in array(&manifest["artifacts"], "manifest artifacts")? {
        let name = text(&artifact["path"], "artifact path")?.to_owned();
        ensure!(
            members.insert(name, artifact.clone()).is_none(),
            "duplicate artifact path"
        );
        check_artifact(root, artifact)?;
        ensure!(
            !text(&artifact["role"], "artifact role")?.is_empty(),
            "empty artifact role"
        );
    }
    ensure!(
        REQUIRED.iter().all(|key| members.contains_key(*key)),
        "missing mandatory provenance artifact"
    );
    let read = |name: &str| -> Result<Value> {
        match members.get(name) {
            Some(artifact) => load_artifact_json(root, artifact),
            None => Ok(Value::Null),
        }
    };
    ensure!(read("config.json")? == config, "config artifact mismatch");
    ensure!(
        read("run.json")? == info.binding && read("plan.json")? == info.plan,
        "transplanted provenance"
    );
    ensure!(
        members["protocol.json"]["sha256"] == info.plan["protocol_sha256"],
        "cell protocol mismatch"
    );
    if info.protocol_preflight_reserved == 0 {
        ensure!(
            read("protocol.json")?["schema"] == "experiment.protocol/v1",
            "cell protocol schema"
        );
    } else {
        // Root preflight verified the strict frozen protocol. Its exact digest
        // equals this descriptor, and a fresh bounded hash binds cell bytes.
        check_artifact(root, &members["protocol.json"])?;
    }
    let outcome = &manifest["outcome"];
    text(&outcome["status"], "outcome status")?;
    let roles: BTreeSet<String> = members
        .values()
        .map(|a| text(&a["role"], "artifact role").map(str::to_owned))
        .collect::<Result<_>>()?;
    let public = read("public.json")?;
    let private = read("witness.json")?;
    let base = read("factor-base.json")?;
    let scalar = outcome["scalar_recovered_without_witness"] == true;
    let pdp = outcome["pdp_witness_verified"] == true;
    let success = scalar || pdp || SUCCESS_ROLES.iter().any(|role| roles.contains(*role));
    if success {
        ensure!(
            !public.is_null() && !private.is_null() && !base.is_null(),
            "success certificate lacks public/witness/factor-base inputs"
        );
        ensure!(
            outcome["solver_completed"] == true,
            "success with incomplete solver"
        );
        ensure!(
            outcome["exhaustive_no_decomposition"] != true,
            "contradictory SAT/UNSAT labels"
        );
        ensure!(
            matches!(outcome["status"].as_str(), Some("verified" | "solved")),
            "success with failure status"
        );
    }
    if scalar {
        ensure!(
            roles.contains(SUCCESS_ROLES[0]) && outcome["scalar_replay_verified"] == true,
            "scalar label lacks certificate/replay"
        );
    }
    if pdp {
        ensure!(
            roles.contains(SUCCESS_ROLES[1]),
            "PDP label lacks certificate"
        );
    }
    if outcome["known_answer_verified"] == true {
        ensure!(
            !public.is_null() && !private.is_null(),
            "known answer lacks inputs"
        );
    }
    if outcome["factor_base_verified"] == true {
        ensure!(
            !public.is_null() && !base.is_null(),
            "factor base lacks inputs"
        );
    }
    if !public.is_null() {
        ensure!(
            public["schema"] == "ml-ecdlp-instance/v1",
            "public instance schema"
        );
        for key in ["family", "n", "m", "twist"] {
            ensure!(public[key] == config[key], "public/config {key} mismatch");
        }
        ensure!(
            public["split_component"] == format!("field-size-n{}", uint(&config["n"], "n")?),
            "unbound split component"
        );
        if !private.is_null() {
            ensure!(
                private["schema"] == "ml-ecdlp-private-witness/v1",
                "private witness schema"
            );
            let mut expected = Map::new();
            for key in ["seed", "curve_seed", "generator_index", "key_index"] {
                expected.insert(key.into(), config[key].clone());
            }
            ensure!(
                private["sampling_provenance"] == Value::Object(expected),
                "transplanted sampling provenance"
            );
        }
        if !base.is_null() {
            ensure!(
                base["requested_size"] == config["factor_base_size"],
                "factor-base size provenance"
            );
            crate::policy::bind_base(&public, &base, &config)?;
            let points = array(&base["points"], "factor-base points")?;
            if base["schema"] == "ml-ecdlp-factor-base/v1"
                && base["construction_status"] == "verified"
            {
                let r = decimal_u64(&public["curve"]["subgroup_order"])?;
                let count = u64::try_from(points.len())?;
                ensure!(
                    count >= uint(&config["factor_base_size"], "factor-base size")?
                        || r.checked_sub(1) == Some(count),
                    "factor-base completion falsely asserted"
                );
            }
        }
    }
    if base["schema"] == "ml-ecdlp-factor-base/v2" && base["construction_status"] == "unsupported" {
        ensure!(
            !success && outcome["status"] == "unsupported",
            "unsupported policy has solver success or non-unsupported outcome"
        );
    }
    let mut checks = Vec::new();
    for artifact in members.values() {
        let role = text(&artifact["role"], "artifact role")?;
        if !SUCCESS_ROLES.contains(&role) {
            continue;
        }
        let certificate = load_artifact_json(root, artifact)?;
        ensure!(
            !public["target"].is_null() && certificate["target"] == public["target"],
            "certificate target mismatch"
        );
        if role == SUCCESS_ROLES[1] {
            ensure!(
                u64::try_from(array(&certificate["summands"], "PDP summands")?.len())?
                    == uint(&config["m"], "arity")?,
                "PDP arity mismatch"
            );
            if certificate.get("factor_base_definition").is_some() {
                ensure!(
                    config["solver"] == "pkm",
                    "alternate factor base outside PKM"
                );
            }
        }
        if arithmetic {
            let result =
                crate::arithmetic::verify_certificate(&public, &base, &config, &certificate, role)?;
            checks.push(json!({"role":role,"artifact_sha256":artifact["sha256"],"checks":result}));
        }
    }
    let root_checks =
        root_certificates(root, &members, &public, &base, &config, outcome, arithmetic)?;
    let filter_checks =
        crate::large_prime::audit(root, &members, &public, &base, &config, outcome, arithmetic)?;
    let manifest_sha = digest(&manifest_bytes);
    let checked_phase = crate::phase::audit_sealed(
        root,
        &members,
        index,
        &info.plan_sha,
        info.protocol_preflight_reserved,
    )?;
    let (measurements, phase_summary) = match checked_phase {
        Some(checked) => (checked.measurements, Some(checked.summary)),
        None => (phase_measurements(root, &members)?, None),
    };
    let mut row = json!({"id":manifest_sha,"index":index,"config":config,"public":public,"factor_base":base,
        "outcome":outcome,"phase_measurements":measurements,
        "positive_certificate_arithmetic":checks,"root_certificate_arithmetic":root_checks,
        "artifact_roles":roles,"manifest_sha256":manifest_sha});
    if let Some(summary) = phase_summary {
        row["checked_phase_contract"] = summary;
    }
    if let Some(checks) = filter_checks {
        row["large_prime_filter_arithmetic"] = checks;
    }
    Ok(row)
}

fn root_certificates(
    root: &Path,
    members: &BTreeMap<String, Value>,
    public: &Value,
    base: &Value,
    config: &Value,
    outcome: &Value,
    arithmetic: bool,
) -> Result<Value> {
    use crate::roots::{BINARY_ROLE, PRIME_ROLE};
    let certificates: Vec<_> = members
        .values()
        .filter(|a| matches!(a["role"].as_str(), Some(PRIME_ROLE | BINARY_ROLE)))
        .collect();
    let call_artifacts: Vec<_> = members
        .values()
        .filter(|a| {
            matches!(
                a["role"].as_str(),
                Some("native-prime-root-call" | "native-binary-root-call")
            )
        })
        .collect();
    if certificates.is_empty() {
        ensure!(
            [
                "root_certificate_scope",
                "root_execution_trace_complete_verified",
                "root_total_operation_count_verified"
            ]
            .iter()
            .all(|k| outcome.get(*k).is_none())
                && call_artifacts.is_empty(),
            "root outcome/call evidence lacks a recognized certificate"
        );
        return Ok(
            json!({"status":if outcome["status"]=="unsupported" {"unsupported_cell"}
            else if config["solver"]=="root" {"legacy_root_evidence_unverified"} else {"not_applicable"},
            "completed_root_sets_checked":0,"all_supplied_root_sets_verified":false,
            "execution_trace_complete_verified":false,"total_operation_count_verified":false,"unsat_verified":false}),
        );
    }
    ensure!(
        certificates.len() == 1 && !public.is_null() && !base.is_null(),
        "root certificate lacks unique public/domain context"
    );
    let artifact = certificates[0];
    let role = text(&artifact["role"], "root certificate role")?;
    ensure!(
        artifact["encoding"] == "json",
        "root certificate must be JSON"
    );
    let certificate = load_artifact_json(root, artifact)?;
    let queries = array(&certificate["queries"], "root certificate queries")?;
    ensure!(
        call_artifacts.len() == queries.len(),
        "root call artifacts differ from certificate queries"
    );
    for (index, query) in queries.iter().enumerate() {
        let (path, call_role) = if role == BINARY_ROLE {
            (
                format!("binary-root-call-{index:04}.json"),
                "native-binary-root-call",
            )
        } else {
            (
                format!("root-call-{index:04}.json"),
                "native-prime-root-call",
            )
        };
        let member = members.get(&path).context("missing root call artifact")?;
        ensure!(
            member["role"] == call_role
                && member["encoding"] == "json"
                && load_artifact_json(root, member)? == *query,
            "root call artifact/certificate mismatch"
        );
    }
    let mut checked = crate::roots::verify_dataset(
        public,
        base,
        config,
        outcome,
        &certificate,
        role,
        arithmetic,
    )?;
    checked["artifact_sha256"] = artifact["sha256"].clone();
    checked["artifact_role"] = json!(role);
    Ok(checked)
}

pub fn audit(root: &Path, expected: Option<&str>, arithmetic: bool) -> Result<(Value, Vec<Value>)> {
    let info = plan_info(root)?;
    if let Some(expected) = expected {
        ensure!(info.run_sha == expected, "trusted run digest mismatch");
    }
    let mut rows = Vec::new();
    let mut incomplete = Vec::new();
    let cells = contained(root, "cells")?;
    if cells.exists() {
        let mut paths = fs::read_dir(&cells)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<std::io::Result<Vec<_>>>()?;
        paths.sort();
        for cell in paths {
            let metadata = fs::symlink_metadata(&cell)?;
            let name = cell
                .file_name()
                .and_then(|s| s.to_str())
                .context("non-UTF8 cell directory")?;
            ensure!(
                metadata.is_dir()
                    && !metadata.file_type().is_symlink()
                    && name.len() == 12
                    && name.bytes().all(|b| b.is_ascii_digit()),
                "invalid cell directory"
            );
            if cell.join("manifest.json").try_exists()? {
                rows.push(audit_cell(&cell, &info, arithmetic)?);
            } else {
                incomplete.push(name.to_owned());
            }
        }
    }
    let mut counts = BTreeMap::<String, u64>::new();
    let mut checked_certificates = 0u64;
    let mut checked_root_sets = 0u64;
    for row in &rows {
        let status = text(&row["outcome"]["status"], "outcome status")?.to_owned();
        let count = counts.entry(status).or_default();
        *count = count.checked_add(1).context("outcome count overflow")?;
        checked_certificates = checked_certificates
            .checked_add(u64::try_from(
                array(&row["positive_certificate_arithmetic"], "arithmetic checks")?.len(),
            )?)
            .context("certificate count overflow")?;
        checked_root_sets = checked_root_sets
            .checked_add(uint(
                &row["root_certificate_arithmetic"]["completed_root_sets_checked"],
                "checked root set count",
            )?)
            .context("root certificate count overflow")?;
    }
    let mut summary = json!({"schema":"ml-dataset.consumer-audit/v1","run_sha256":info.run_sha,
        "trusted_run_anchor_checked":expected.is_some(),"plan_sha256":info.plan_sha,"planned_cells":info.total,
        "checked_cells":rows.len(),"incomplete_attempts":incomplete,"outcomes":counts,
        "hashes_and_cross_artifact_bindings_verified":true,"mathematics_independently_verified":false,
        "producer_native_replay_passed":false,"independent_arithmetic_requested":arithmetic,
        "positive_certificates_arithmetic_checked":checked_certificates,
        "completed_distinct_root_sets_checked":checked_root_sets,
        "remote_rechecked":false,"full_scope_complete":false});
    if rows
        .iter()
        .any(|r| r.get("large_prime_filter_arithmetic").is_some())
    {
        let mut counts = BTreeMap::new();
        for field in [
            "completed_batches_checked",
            "censored_prefixes_checked",
            "source_equations_checked",
            "complete_matrix_ranks_checked",
        ] {
            let mut total = 0u64;
            for row in &rows {
                if let Some(check) = row.get("large_prime_filter_arithmetic") {
                    total = total
                        .checked_add(uint(&check[field], field)?)
                        .context("filter check count overflow")?;
                }
            }
            counts.insert(field, total);
        }
        summary["generic_filter_finite_checks"] = json!(counts);
    }
    Ok((summary, rows))
}

fn decimal_u64(value: &Value) -> Result<u64> {
    let value = text(value, "public decimal")?;
    ensure!(
        !value.is_empty()
            && value.len() <= 20
            && value.bytes().all(|b| b.is_ascii_digit())
            && (value == "0" || !value.starts_with('0')),
        "invalid public decimal"
    );
    value.parse().context("public decimal exceeds u64")
}

fn public_point(point: &Value) -> Result<Value> {
    if point.is_null() {
        return Ok(Value::Null);
    }
    ensure!(point.is_object(), "invalid public point");
    decimal_u64(&point["x"])?;
    decimal_u64(&point["y"])?;
    Ok(json!({"x":point["x"],"y":point["y"]}))
}

fn public_points(points: &Value) -> Result<Value> {
    Ok(Value::Array(
        array(points, "public points")?
            .iter()
            .map(public_point)
            .collect::<Result<_>>()?,
    ))
}

fn select(value: &Value, keys: &[&str]) -> Result<Map<String, Value>> {
    let object = value
        .as_object()
        .context("public field must be an object")?;
    Ok(keys
        .iter()
        .filter_map(|key| object.get(*key).map(|v| ((*key).to_owned(), v.clone())))
        .collect())
}

fn static_features(row: &Value) -> Result<Value> {
    let public = &row["public"];
    let base = &row["factor_base"];
    let config = &row["config"];
    if public.is_null() || base.is_null() {
        return Ok(Value::Null);
    }
    let versioned = base["schema"] == "ml-ecdlp-factor-base/v2";
    let policy = if versioned {
        crate::policy::bind_base(public, base, config)?;
        if base["construction_status"] == "unsupported" {
            return Ok(Value::Null);
        }
        Some(crate::policy::recipe(
            &base["policy"],
            uint(&config["n"], "field size")?,
        )?)
    } else {
        None
    };
    ensure!(
        versioned
            || matches!(
                base["rule"].as_str(),
                Some(
                    "small-x-cofactor-projection"
                        | "small-x-prime-primary-projection-with-cyclic-membership"
                )
            ),
        "unsupported factor-base rule"
    );
    let base_points = array(&base["points"], "factor-base points")?;
    let mut base_view = Map::new();
    base_view.insert("rule".into(), base["rule"].clone());
    if let Some(policy) = policy {
        base_view.insert("policy".into(), policy);
    }
    for key in ["negation_action", "frobenius_action"] {
        if let Some(action) = base.get(key) {
            ensure!(
                array(action, "factor-base action")?.iter().all(
                    |x| x.is_null() || x.as_u64().is_some_and(|n| n < base_points.len() as u64)
                ),
                "invalid factor-base action"
            );
            base_view.insert(key.into(), action.clone());
        }
    }
    if let Some(power) = base.get("frobenius_power") {
        ensure!(
            uint(power, "Frobenius power")? > 0,
            "invalid Frobenius power"
        );
        base_view.insert("frobenius_power".into(), power.clone());
    }
    base_view.insert("points".into(), public_points(&base["points"])?);
    if let Some(orbits) = base.get("orbits") {
        let orbits = array(orbits, "factor-base orbits")?.iter().map(|orbit| -> Result<Value> {
            ensure!(uint(&orbit["stabilizer_size"], "orbit stabilizer")? > 0, "invalid orbit stabilizer");
            Ok(json!({"points":public_points(&orbit["points"])?,"stabilizer_size":orbit["stabilizer_size"]}))
        }).collect::<Result<Vec<_>>>()?;
        base_view.insert("orbits".into(), json!(orbits));
    }
    let field = select(
        &public["field"],
        &[
            "characteristic",
            "degree",
            "modulus",
            "irreducible_low_terms",
            "basis",
        ],
    )?;
    for key in ["characteristic", "modulus"] {
        if let Some(value) = field.get(key) {
            decimal_u64(value)?;
        }
    }
    let degree = uint(field.get("degree").unwrap_or(&Value::Null), "field degree")?;
    ensure!((1..=61).contains(&degree), "invalid field degree");
    ensure!(
        matches!(
            field.get("basis").and_then(Value::as_str),
            Some("canonical-residue" | "polynomial-lsb-first")
        ),
        "unknown field basis"
    );
    if let Some(terms) = field.get("irreducible_low_terms") {
        ensure!(
            array(terms, "polynomial terms")?
                .iter()
                .all(|x| x.as_u64().is_some_and(|n| n < degree)),
            "invalid polynomial terms"
        );
    }
    let curve = select(
        &public["curve"],
        &[
            "model",
            "a",
            "b",
            "base_a",
            "base_b",
            "twist_parameter",
            "group_order",
            "subgroup_order",
            "cofactor",
        ],
    )?;
    ensure!(
        matches!(
            curve.get("model").and_then(Value::as_str),
            Some("short-weierstrass" | "binary-ordinary")
        ),
        "unknown curve model"
    );
    for (key, value) in &curve {
        if key != "model" {
            decimal_u64(value)?;
        }
    }
    Ok(
        json!({"family":public["family"],"n":public["n"],"m":public["m"],"twist":public["twist"],
        "field":field,"curve":curve,"generator":public_point(&public["generator"])?,"target":public_point(&public["target"])?,
        "factor_base":base_view,"solver":config["solver"],"degree_bound":config["degree_bound"],"work_limit":config["work_limit"]}),
    )
}

fn split_for(component: &str) -> Result<&'static str> {
    let hash = digest(format!("ml-ecdlp-static-split/v1:{component}").as_bytes());
    let mut remainder = 0u16;
    for digit in hash.chars() {
        remainder = (remainder * 16
            + u16::try_from(digit.to_digit(16).context("invalid SHA-256 digit")?)?)
            % 100;
    }
    Ok(if remainder < 80 {
        "train"
    } else if remainder < 90 {
        "validation"
    } else {
        "test"
    })
}

pub fn export_rows(rows: &[Value], path: &Path) -> Result<()> {
    static SERIAL: AtomicU64 = AtomicU64::new(0);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();
    let temporary: PathBuf = parent.join(format!(
        ".consumer-export-{}-{stamp}-{}",
        std::process::id(),
        SERIAL.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    let result = (|| -> Result<()> {
        for row in rows {
            let component = format!("field-size-n{}", uint(&row["config"]["n"], "field size")?);
            let checks = row
                .get("positive_certificate_arithmetic")
                .cloned()
                .unwrap_or_else(|| json!([]));
            let versioned = row["config"].get("factor_base_policy").is_some();
            let root_checks = row.get("root_certificate_arithmetic").cloned().unwrap_or_else(||
                json!({"status":"not_checked","completed_root_sets_checked":0,"all_supplied_root_sets_verified":false}));
            let features = static_features(row)?;
            let mut record = json!({"schema":if versioned {"ml-dataset.static-routing-view/v2"} else {"ml-dataset.static-routing-view/v1"},"record_id":row["id"],
                "split":split_for(&component)?,"independent_unit":component,"features":features,
                "labels":{"producer_reported":row["outcome"],"independently_verified":false,
                    "phase_measurements":row["phase_measurements"],"positive_certificate_arithmetic":checks,
                    "root_certificate_arithmetic":root_checks,
                    "root_sets_independently_verified":root_checks["all_supplied_root_sets_verified"]==true,
                    "whole_worker_instruction_count":null},
                "feature_policy":"allowlisted public static inputs only; no trace/manifest/witness features"});
            if let Some(checks) = row.get("large_prime_filter_arithmetic") {
                record["labels"]["large_prime_filter_arithmetic"] = checks.clone();
            }
            if let Some(summary) = row.get("checked_phase_contract") {
                record["labels"]["checked_phase_contract"] = summary.clone();
            }
            if versioned {
                record["feature_policy"] = json!("consumer-static-policy-recipes-v2; distinct from producer strict-structural-allowlist-v4-policy-recipes");
                record["feature_status"] = json!(if row["factor_base"]["construction_status"]
                    == "unsupported"
                {
                    "unsupported_factor_base_policy"
                } else if features.is_null() {
                    "missing_public_inputs"
                } else {
                    "available"
                });
                record["labels"]["factor_base_construction"] = json!({
                    "producer_reported_status":row["factor_base"]["construction_status"],
                    "producer_reported_counters":row["factor_base"]["construction_counters"],
                    "independently_verified":false});
                record["producer_training_export_equivalence_verified"] = json!(false);
            }
            serde_json::to_writer(&mut file, &record)?;
            file.write_all(b"\n")?;
        }
        file.flush()?;
        file.sync_all()?;
        // Link is an atomic create-if-absent installation; never replace an export.
        fs::hard_link(&temporary, path)?;
        File::open(parent)?.sync_all()?;
        Ok(())
    })();
    drop(file);
    let cleanup = fs::remove_file(&temporary);
    result?;
    cleanup?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan() -> Value {
        json!({"schema":"crypto.ml-dataset.plan/v1","field_sizes":[23],"arities":[2,4],
            "families":["koblitz","prime"],"solvers":["bsgs","sat"],"curve_seeds":2,
            "generators":2,"keys":2,"seed":20261027,"factor_base_size":4,"degree_bound":3,
            "work_limit":20000,"twists":true,"protocol_sha256":"a".repeat(64),
            "split_rule":"curve-twist-component-sha256-v1"})
    }

    fn metrics() -> Value {
        json!({"hardware":{"instructions":100,"instructions_scaled":100,"instructions_time_enabled_ns":10,
            "instructions_time_running_ns":10,"cycles":200,"cycles_scaled":200,"cycles_time_enabled_ns":10,
            "cycles_time_running_ns":10,"multiplexed":false,"inherit":true,"grouped":true},
            "instructions_status":"measured_unmultiplexed","retired_instructions_raw":100,
            "retired_instructions_exact_in_scope":100,"clock_valid_for_comparison":false,
            "counter_scope":"calling thread user space","excludes":["startup","upload"],
            "returned_ok":true,"returned_error":null,"interrupted_phase_readback":false})
    }

    #[test]
    fn mixed_radix_and_checked_plan_count() {
        let plan = plan();
        assert_eq!(validate_plan(&plan).unwrap(), 128);
        assert_eq!(config_at(&plan, 0).unwrap()["solver"], "bsgs");
        assert_eq!(config_at(&plan, 1).unwrap()["solver"], "sat");
        assert_eq!(config_at(&plan, 2).unwrap()["twist"], true);
        assert_eq!(config_at(&plan, 4).unwrap()["m"], 4);
        assert_eq!(config_at(&plan, 127).unwrap()["family"], "prime");
        assert!(config_at(&plan, 128).is_err());
        let mut bad = plan.clone();
        bad["keys"] = json!(u64::MAX);
        assert!(validate_plan(&bad).is_err());
        let mut bad = plan.clone();
        bad["arities"] = json!([2, 2]);
        assert!(validate_plan(&bad).is_err());
        let mut bad = plan;
        bad["generators"] = json!(true);
        assert!(validate_plan(&bad).is_err());
    }

    #[test]
    fn versioned_recipe_axis_preserves_matched_legacy_cells_and_hash_order() {
        let legacy = plan();
        let mut versioned = legacy.clone();
        versioned["schema"] = json!("crypto.ml-dataset.plan/v2");
        versioned["factor_base_policies"] = json!([
            {"kind":"legacy"},
            {"kind":"sampled_x_projected_control","candidate_limit":32},
            {"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":4},
            {"kind":"binary_frobenius_union_intersection","basis":[1,2],"candidate_limit":64}
        ]);
        assert_eq!(validate_plan(&versioned).unwrap(), 512);
        for i in 0..128 {
            for p in 0..4 {
                let index = (i / 2) * 8 + p * 2 + i % 2;
                let mut config = config_at(&versioned, index).unwrap();
                assert_eq!(
                    config["factor_base_policy"],
                    versioned["factor_base_policies"][p as usize]
                );
                config.as_object_mut().unwrap().remove("factor_base_policy");
                assert_eq!(config, config_at(&legacy, i).unwrap());
            }
        }
        let original = canonical_plan(&versioned).unwrap();
        versioned["factor_base_policies"][2] = serde_json::from_str(
            r#"{"candidate_limit":4,"basis":[1,2],"kind":"binary_subspace_intersection"}"#,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_vec(&original).unwrap(),
            serde_json::to_vec(&canonical_plan(&versioned).unwrap()).unwrap()
        );
        for axis in [
            json!([]),
            json!([{"kind":"legacy"},{"kind":"legacy"}]),
            json!([{"kind":"legacy","extra":1}]),
            json!([{"kind":"binary_subspace_intersection","basis":[1,2,3],"candidate_limit":8}]),
        ] {
            let mut bad = versioned.clone();
            bad["factor_base_policies"] = axis;
            assert!(validate_plan(&bad).is_err());
        }
        versioned["schema"] = json!("crypto.ml-dataset.plan/v1");
        assert!(validate_plan(&versioned).is_err());
        let mut missing = legacy;
        missing["schema"] = json!("crypto.ml-dataset.plan/v2");
        assert!(validate_plan(&missing).is_err());
    }

    #[test]
    fn frozen_legacy_pilot_hash_and_all_720_cells_are_unchanged() {
        let pilot: Value = serde_json::from_str(r#"{"schema":"crypto.ml-dataset.plan/v1","field_sizes":[23],"arities":[2,4,15],"families":["koblitz","binary","prime"],"solvers":["bsgs","enumerate","root","sat","f4","f5","f6","weil","pkm","double-large-prime"],"curve_seeds":1,"generators":2,"keys":2,"seed":20261027,"factor_base_size":4,"degree_bound":3,"work_limit":20000,"twists":true,"protocol_sha256":"05a54dacc052b49bf7b9d6ffd776c461f49757e01488088c9a79bbf136f11604","split_rule":"curve-twist-component-sha256-v1"}"#).unwrap();
        assert_eq!(
            digest(&serde_json::to_vec(&canonical_plan(&pilot).unwrap()).unwrap()),
            "c905e878e3dcb29e407e743874d18b6b7091c097984f829820f26673ccd47f21"
        );
        assert_eq!(validate_plan(&pilot).unwrap(), 720);
        for i in 0..720 {
            let config = config_at(&pilot, i).unwrap();
            assert_eq!(config["solver"], pilot["solvers"][(i % 10) as usize]);
            assert_eq!(config["twist"], (i / 10) % 2 == 1);
            assert_eq!(config["m"], [2, 4, 15][((i / 20) % 3) as usize]);
            assert_eq!(config["key_index"], (i / 60) % 2);
            assert_eq!(config["generator_index"], (i / 120) % 2);
            assert_eq!(config["family"], pilot["families"][(i / 240) as usize]);
            assert!(config.get("factor_base_policy").is_none());
        }
    }

    #[test]
    fn both_hardware_counters_and_global_mux_are_checked() {
        validate_metrics(&metrics()).unwrap();
        for (key, value) in [
            ("cycles_scaled", json!(999)),
            ("cycles_time_running_ns", json!(11)),
            ("instructions", json!(true)),
            ("multiplexed", json!(true)),
        ] {
            let mut bad = metrics();
            bad["hardware"][key] = value;
            assert!(validate_metrics(&bad).is_err(), "{key}");
        }
        let mut mux = metrics();
        mux["hardware"]["cycles_time_enabled_ns"] = json!(20);
        mux["hardware"]["cycles_scaled"] = json!(400);
        mux["hardware"]["multiplexed"] = json!(true);
        mux["instructions_status"] = json!("measured_multiplexed_estimate");
        mux["retired_instructions_exact_in_scope"] = Value::Null;
        validate_metrics(&mux).unwrap();
    }

    #[test]
    fn complete_cycles_do_not_make_incomplete_instruction_timing_exact() {
        let mut incomplete = metrics();
        validate_metrics(&incomplete).unwrap();

        // The cycle tuple remains complete and unmultiplexed. It cannot
        // establish instruction exactness when that event's enabled time is
        // missing, even if the raw instruction label and running time exist.
        incomplete["hardware"]["instructions_time_enabled_ns"] = Value::Null;
        incomplete["hardware"]["instructions_scaled"] = Value::Null;
        let error = validate_metrics(&incomplete).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("incomplete instructions counter tuple"),
            "{error}"
        );
    }

    #[test]
    fn zero_running_missing_counters_and_saturating_scaling() {
        let mut m = metrics();
        m["hardware"] = json!({"instructions":null,"error":"denied"});
        m["instructions_status"] = json!("unavailable");
        m["retired_instructions_raw"] = Value::Null;
        m["retired_instructions_exact_in_scope"] = Value::Null;
        validate_metrics(&m).unwrap();
        m["hardware"]["instructions_scaled"] = json!(123);
        assert!(validate_metrics(&m).is_err());
        m["hardware"] = json!({"instructions":100,"instructions_scaled":100,
            "instructions_time_enabled_ns":0,"instructions_time_running_ns":0,"multiplexed":false});
        validate_metrics(&m).unwrap();
        m["hardware"] = json!({"instructions":u64::MAX,"instructions_scaled":u64::MAX,
            "instructions_time_enabled_ns":u64::MAX,"instructions_time_running_ns":1,"multiplexed":true});
        m["instructions_status"] = json!("measured_multiplexed_estimate");
        m["retired_instructions_raw"] = json!(u64::MAX);
        validate_metrics(&m).unwrap();
    }

    #[test]
    fn static_features_drop_nested_private_fields_and_all_telemetry() {
        let row = json!({"public":{"family":"prime","n":23,"m":2,"twist":false,
            "field":{"characteristic":"17","degree":1,"modulus":"17","basis":"canonical-residue","seed":"PRIVATE-FIELD"},
            "curve":{"model":"short-weierstrass","a":"2","b":"2","subgroup_order":"19","secret_seed":"PRIVATE-CURVE"},
            "generator":{"x":"5","y":"1","scalar":"PRIVATE-SCALAR"},"target":{"x":"6","y":"3"}},
            "factor_base":{"rule":"small-x-cofactor-projection","points":[{"x":"5","y":"1","log":"PRIVATE-LOG"}],
                "orbits":[{"points":[{"x":"5","y":"1","trace":"PRIVATE-TRACE"}],"stabilizer_size":1,"secret":"PRIVATE-ORBIT"}]},
            "config":{"solver":"bsgs","degree_bound":3,"work_limit":20000,"seed":17},
            "phase_measurements":[{"instructions":999}],"outcome":{"scalar":"PRIVATE-OUTCOME"}});
        let features = static_features(&row).unwrap();
        let serialized = serde_json::to_string(&features).unwrap();
        assert!(!serialized.contains("PRIVATE"));
        assert!(!serialized.contains("instructions"));
        assert!(features["generator"].get("scalar").is_none());
        let mut bad = row;
        bad["public"]["curve"]["a"] = json!({"seed":42});
        assert!(static_features(&bad).is_err());
    }

    #[test]
    fn policy_export_is_separate_and_excludes_unsupported_features() {
        let (public, mut base, config) = crate::policy::tests::fixture();
        base["construction_candidates"] = json!({"private_log":"NEVER-A-FEATURE"});
        let row = json!({"id":"a".repeat(64),"public":public,"factor_base":base,"config":config,
            "outcome":{"status":"solved","scalar":"NEVER-AN-INPUT"},
            "phase_measurements":[],"positive_certificate_arithmetic":[]});
        let mut unsupported = row.clone();
        let recipe =
            json!({"kind":"binary_subspace_intersection","basis":[1,2],"candidate_limit":4});
        unsupported["config"]["factor_base_policy"] = recipe.clone();
        let base = &mut unsupported["factor_base"];
        base["policy"] = recipe;
        base["rule"] = json!("binary_subspace_intersection");
        base["subgroup_treatment"] = json!("literal-coordinate-domain-intersection");
        base["construction_status"] = json!("unsupported");
        base["unsupported_reason"] = json!("binary policy on prime field");
        base["points"] = json!([]);
        base["actual_size"] = json!(0);
        base["negation_action"] = json!([]);
        base["frobenius_action"] = json!([]);
        base["construction_counters"]["candidate_coordinates"] = json!(0);
        unsupported["outcome"] = json!({"status":"unsupported","solver_completed":false});
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("view.jsonl");
        export_rows(&[row, unsupported], &path).unwrap();
        let rows: Vec<Value> = fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        for record in &rows {
            assert_eq!(record["schema"], "ml-dataset.static-routing-view/v2");
            assert_eq!(
                record["producer_training_export_equivalence_verified"],
                false
            );
            assert_eq!(record["labels"]["root_sets_independently_verified"], false);
            assert_eq!(
                record["labels"]["factor_base_construction"]["independently_verified"],
                false
            );
            let features = serde_json::to_string(&record["features"]).unwrap();
            for secret in ["NEVER", "construction", "sampling_seed", "scalar"] {
                assert!(!features.contains(secret));
            }
        }
        assert_eq!(
            rows[0]["features"]["factor_base"]["policy"],
            json!({"kind":"legacy"})
        );
        assert_eq!(rows[0]["feature_status"], "available");
        assert_eq!(rows[1]["feature_status"], "unsupported_factor_base_policy");
        assert!(rows[1]["features"].is_null());
        assert_eq!(
            rows[1]["labels"]["producer_reported"]["status"],
            "unsupported"
        );
    }
}
