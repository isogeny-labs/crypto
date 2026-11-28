use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

pub type GateResult<T> = Result<T, String>;

pub const GRID: [(u64, u64, u64, &str, u64); 11] = [
    (12, 4_051, 5, "low", 3_600),
    (13, 8_191, 5, "low", 3_600),
    (14, 16_363, 5, "low", 3_600),
    (15, 32_719, 5, "low", 3_600),
    (16, 65_479, 5, "low", 3_600),
    (17, 131_071, 5, "low", 3_600),
    (18, 262_111, 5, "low", 3_600),
    (19, 524_287, 3, "mid", 3_600),
    (20, 1_048_507, 3, "mid", 3_600),
    (21, 2_097_091, 3, "high", 7_200),
    (22, 4_194_271, 3, "top", 10_800),
];
pub const SEEDS: [u64; 3] = [1, 2, 3];
pub const DIAGNOSTIC_Q: [u64; 3] = [4_051, 65_479, 262_111];
pub const IC_ARMS: [&str; 8] = ["B0", "S", "L0", "L1", "L", "D", "DL1", "DL"];
pub const PRIMARY: &str = "strong-b8-mul5-walker-table";
pub const DIAGNOSTICS: [&str; 3] = [
    "strong-b4-mul5-walker-table",
    "strong-b16-mul5-walker-table",
    "old-neg",
];
pub const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
pub const HISTORICAL_SHA256: &str =
    "aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42";
pub const HISTORICAL_BYTES: u64 = 382_105;

const RHO_PHASES: [&str; 9] = [
    "cold_setup",
    "cold_walk",
    "cold_collision",
    "cold_recovery",
    "cold_verify",
    "online_walk",
    "online_collision",
    "online_recovery",
    "online_verify",
];
const IC_PHASES: [&str; 8] = [
    "cold_setup",
    "cold_relations",
    "cold_filtering",
    "cold_solver",
    "cold_recovery",
    "cold_verify",
    "online_target_dependent",
    "online_verify",
];

fn object<'a>(value: &'a Value, label: &str) -> GateResult<&'a Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| format!("{label} is not an object"))
}

fn array<'a>(value: &'a Value, label: &str) -> GateResult<&'a Vec<Value>> {
    value
        .as_array()
        .ok_or_else(|| format!("{label} is not an array"))
}

fn field<'a>(object: &'a Map<String, Value>, key: &str, label: &str) -> GateResult<&'a Value> {
    object
        .get(key)
        .ok_or_else(|| format!("{label} lacks {key}"))
}

fn string<'a>(object: &'a Map<String, Value>, key: &str, label: &str) -> GateResult<&'a str> {
    field(object, key, label)?
        .as_str()
        .ok_or_else(|| format!("{label}.{key} is not a string"))
}

fn unsigned(object: &Map<String, Value>, key: &str, label: &str) -> GateResult<u64> {
    field(object, key, label)?
        .as_u64()
        .ok_or_else(|| format!("{label}.{key} is not an unsigned integer"))
}

fn finite_nonnegative(object: &Map<String, Value>, key: &str, label: &str) -> GateResult<f64> {
    let value = field(object, key, label)?
        .as_f64()
        .ok_or_else(|| format!("{label}.{key} is not numeric"))?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{label}.{key} is negative or non-finite"));
    }
    Ok(value)
}

fn boolean(object: &Map<String, Value>, key: &str, label: &str) -> GateResult<bool> {
    field(object, key, label)?
        .as_bool()
        .ok_or_else(|| format!("{label}.{key} is not Boolean"))
}

fn exact_strings(value: &Value, expected: &[&str], label: &str) -> GateResult<()> {
    let observed = array(value, label)?
        .iter()
        .map(|item| {
            item.as_str()
                .ok_or_else(|| format!("{label} contains a non-string"))
        })
        .collect::<GateResult<Vec<_>>>()?;
    if observed != expected {
        return Err(format!("{label} changed"));
    }
    Ok(())
}

fn string_set(value: &Value, expected: &[&str], label: &str) -> GateResult<()> {
    let observed = array(value, label)?
        .iter()
        .map(|item| {
            item.as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| format!("{label} contains a non-string"))
        })
        .collect::<GateResult<BTreeSet<_>>>()?;
    let expected = expected
        .iter()
        .map(|item| (*item).to_owned())
        .collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(format!("{label} changed"));
    }
    Ok(())
}

fn is_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn image_uri(value: &str) -> bool {
    let prefix = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:";
    value
        .strip_prefix(prefix)
        .is_some_and(|digest| is_hex(digest, 64))
}

pub fn read_json(path: &Path) -> GateResult<Value> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    serde_json::from_slice(&bytes).map_err(|error| format!("{}: {error}", path.display()))
}

pub fn sha256_file(path: &Path) -> GateResult<String> {
    let bytes = fs::read(path).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

pub fn validate_protocol(value: &Value) -> GateResult<()> {
    let protocol = object(value, "protocol")?;
    let identity = object(field(protocol, "identity", "protocol")?, "identity")?;
    if string(identity, "schema", "identity")? != "ghs.matched-frontier.protocol/v1" {
        return Err("wrong protocol schema".into());
    }
    if string(identity, "protocol_id", "identity")? != "ghs-matched-frontier-m3-composed-v1" {
        return Err("wrong protocol id".into());
    }
    let evidence = object(
        field(protocol, "evidence_status", "protocol")?,
        "evidence_status",
    )?;
    if boolean(evidence, "measurements_present", "evidence_status")?
        || boolean(evidence, "promotion_allowed", "evidence_status")?
        || string(evidence, "status", "evidence_status")? != "implemented_unverified"
    {
        return Err("protocol improperly claims measurement or promotion".into());
    }
    let family = object(field(protocol, "family", "protocol")?, "family")?;
    if string(family, "scope", "family")? != "restricted-ghs-weak-plane-quartic"
        || string(family, "curve", "family")? != "E_0"
        || string(family, "field", "family")? != "F_(q^3)"
        || string(family, "order", "family")? != "4*ell"
    {
        return Err("curve-family contract changed".into());
    }

    let grid = array(field(protocol, "grid", "protocol")?, "grid")?;
    if grid.len() != GRID.len() {
        return Err("grid length changed".into());
    }
    for (row, expected) in grid.iter().zip(GRID) {
        let row = object(row, "grid row")?;
        let observed = (
            unsigned(row, "k", "grid row")?,
            unsigned(row, "q", "grid row")?,
            unsigned(row, "repetitions_per_seed", "grid row")?,
            string(row, "split", "grid row")?,
            unsigned(row, "deadline_seconds", "grid row")?,
        );
        if observed != expected {
            return Err("grid, repetitions, split, or deadline changed".into());
        }
    }
    let sampling = object(field(protocol, "sampling", "protocol")?, "sampling")?;
    let seeds = array(field(sampling, "seeds", "sampling")?, "seeds")?
        .iter()
        .map(|item| {
            item.as_u64()
                .ok_or_else(|| "seed is not unsigned".to_owned())
        })
        .collect::<GateResult<Vec<_>>>()?;
    if seeds != SEEDS || string(sampling, "target_policy", "sampling")? != "ic-repeat" {
        return Err("seed or target policy changed".into());
    }
    let diagnostic_q = array(field(sampling, "diagnostic_q", "sampling")?, "diagnostic_q")?
        .iter()
        .map(|item| {
            item.as_u64()
                .ok_or_else(|| "diagnostic q is not unsigned".to_owned())
        })
        .collect::<GateResult<Vec<_>>>()?;
    if diagnostic_q != DIAGNOSTIC_Q
        || unsigned(sampling, "diagnostic_repetitions_per_seed", "sampling")? != 1
    {
        return Err("diagnostic sampling changed".into());
    }

    let arms = object(field(protocol, "arms", "protocol")?, "arms")?;
    if string(arms, "rho_primary", "arms")? != PRIMARY {
        return Err("composed primary changed".into());
    }
    exact_strings(
        field(arms, "rho_diagnostics", "arms")?,
        &DIAGNOSTICS,
        "rho diagnostics",
    )?;
    exact_strings(
        field(arms, "historical_ic", "arms")?,
        &IC_ARMS,
        "historical IC arms",
    )?;
    let model = object(field(protocol, "cost_model", "protocol")?, "cost_model")?;
    let weights = array(field(model, "weights", "cost_model")?, "weights")?
        .iter()
        .map(|item| {
            item.as_f64()
                .ok_or_else(|| "weight is not numeric".to_owned())
        })
        .collect::<GateResult<Vec<_>>>()?;
    if weights != WEIGHTS {
        return Err("pricing band changed".into());
    }
    let calibration = object(field(model, "calibration", "cost_model")?, "calibration")?;
    if unsigned(calibration, "interleaved_triples_per_q_seed", "calibration")? != 600
        || !boolean(calibration, "same_weights_for_ic_and_rho", "calibration")?
    {
        return Err("calibration contract changed".into());
    }
    let analysis = object(field(protocol, "analysis", "protocol")?, "analysis")?;
    if string(analysis, "bootstrap_clusters", "analysis")? != "instance_seed"
        || unsigned(analysis, "bootstrap_resamples", "analysis")? != 10_000
        || !boolean(
            analysis,
            "repetitions_averaged_within_seed_first",
            "analysis",
        )?
    {
        return Err("analysis contract changed".into());
    }
    let execution = object(field(protocol, "execution", "protocol")?, "execution")?;
    if finite_nonnegative(execution, "usd_all_attempt_ceiling", "execution")? != 5.0
        || unsigned(execution, "automatic_retries", "execution")? != 0
        || unsigned(execution, "manual_infrastructure_retries_max", "execution")? != 1
        || unsigned(execution, "job_cpu", "execution")? != 4
        || string(execution, "job_memory", "execution")? != "4Gi"
    {
        return Err("execution envelope changed".into());
    }
    let wall = object(
        field(execution, "wall_admission", "execution")?,
        "wall admission",
    )?;
    if string(wall, "required_level", "wall admission")? != "L2"
        || boolean(wall, "current_bridge_can_earn_L2", "wall admission")?
        || string(wall, "disposition", "wall admission")? != "blocked_until_custom_binary_L2_bridge"
    {
        return Err("wall admission blocker changed or was bypassed".into());
    }
    let archive = object(
        field(protocol, "historical_archive", "protocol")?,
        "archive",
    )?;
    if string(archive, "sha256", "archive")? != HISTORICAL_SHA256
        || unsigned(archive, "bytes", "archive")? != HISTORICAL_BYTES
    {
        return Err("historical archive identity changed".into());
    }
    let coverage = object(
        field(protocol, "required_result_phase_coverage", "protocol")?,
        "phase coverage",
    )?;
    string_set(
        field(coverage, "rho", "phase coverage")?,
        &RHO_PHASES,
        "rho phases",
    )?;
    string_set(
        field(coverage, "ic", "phase coverage")?,
        &IC_PHASES,
        "IC phases",
    )?;
    let acceptance = object(field(protocol, "acceptance", "protocol")?, "acceptance")?;
    let interval = array(
        field(acceptance, "rho_steps_over_sqrt_ell", "acceptance")?,
        "rho admission interval",
    )?;
    if interval != &[Value::from(0.75), Value::from(1.15)]
        || !boolean(
            acceptance,
            "all_sizes_required_for_full_verdict",
            "acceptance",
        )?
        || string(acceptance, "missing_size_verdict", "acceptance")? != "limited"
    {
        return Err("acceptance rule changed".into());
    }
    if !string(protocol, "scope_warning", "protocol")?.contains("only") {
        return Err("restricted-family warning missing".into());
    }
    Ok(())
}

pub fn validate_templates(value: &Value) -> GateResult<()> {
    let root = object(value, "templates")?;
    if string(root, "schema", "templates")? != "ghs.matched-frontier.request-templates/v1" {
        return Err("wrong template schema".into());
    }
    let materialization = object(
        field(root, "materialization", "templates")?,
        "materialization",
    )?;
    if boolean(materialization, "launch_ready", "materialization")? {
        return Err("unbound templates must not be launch-ready".into());
    }
    if array(
        field(materialization, "blocked_until", "materialization")?,
        "blocked_until",
    )?
    .len()
        < 4
    {
        return Err("template blockers incomplete".into());
    }
    let runner = object(
        field(materialization, "runner_contract", "materialization")?,
        "runner contract",
    )?;
    if boolean(runner, "present", "runner contract")?
        || boolean(runner, "l2_admission_capable", "runner contract")?
        || string(runner, "path", "runner contract")?
            != "research/ghs_frontier_3_7_20261119/ws5_matched_ghs/run_split.sh"
    {
        return Err("absent runner/L2 bridge blocker changed or was bypassed".into());
    }
    let requests = object(field(root, "requests", "templates")?, "requests")?;
    let deadlines = BTreeMap::from([
        ("low", 3_600_u64),
        ("mid", 3_600),
        ("high", 7_200),
        ("top", 10_800),
    ]);
    if requests.keys().map(String::as_str).collect::<BTreeSet<_>>()
        != deadlines.keys().copied().collect::<BTreeSet<_>>()
    {
        return Err("request split census changed".into());
    }
    for (split, seconds) in deadlines {
        let request = object(field(requests, split, "requests")?, "request")?;
        if string(request, "schema", "request")? != "isolab.eks.command-request/v1" {
            return Err(format!("wrong request schema: {split}"));
        }
        let job = object(field(request, "job", "request")?, "job")?;
        if unsigned(job, "cpu", "job")? != 4
            || string(job, "memory", "job")? != "4Gi"
            || unsigned(job, "seconds", "job")? != seconds
            || unsigned(job, "retries", "job")? != 0
            || unsigned(job, "wait_seconds", "job")? != 0
            || string(job, "isolation_profile", "job")? != "standard"
            || string(job, "image", "job")? != "${IMAGE_DIGEST_URI}"
        {
            return Err(format!("request envelope changed: {split}"));
        }
        let command = array(field(job, "command", "job")?, "command")?;
        if command.last().and_then(Value::as_str) != Some(split) {
            return Err(format!("command split mismatch: {split}"));
        }
        let provenance = object(field(request, "provenance", "request")?, "provenance")?;
        if string(provenance, "source_revision", "provenance")? != "${SOURCE_REVISION}"
            || string(provenance, "protocol_id", "provenance")?
                != "ghs-matched-frontier-m3-composed-v1"
        {
            return Err(format!("request provenance changed: {split}"));
        }
    }
    Ok(())
}

type CellKey = (String, u64, u64, String);

fn expected_cells() -> BTreeMap<CellKey, u64> {
    let mut cells = BTreeMap::new();
    for (_, q, repetitions, _, _) in GRID {
        for seed in SEEDS {
            cells.insert(("rho_primary".into(), q, seed, PRIMARY.into()), repetitions);
        }
    }
    for q in DIAGNOSTIC_Q {
        for seed in SEEDS {
            for arm in DIAGNOSTICS {
                cells.insert(("rho_diagnostic".into(), q, seed, arm.into()), 1);
            }
        }
    }
    cells
}

fn phase_set(value: &Value, expected: &[&str], label: &str) -> GateResult<()> {
    string_set(value, expected, label)
}

pub fn validate_results(
    protocol_path: &Path,
    manifest_path: &Path,
) -> GateResult<BTreeMap<&'static str, String>> {
    let protocol = read_json(protocol_path)?;
    validate_protocol(&protocol)?;
    let value = read_json(manifest_path)?;
    let manifest = object(&value, "result manifest")?;
    if string(manifest, "schema", "result manifest")? != "ghs.matched-frontier.result-manifest/v1" {
        return Err("wrong result-manifest schema".into());
    }
    if string(manifest, "protocol_sha256", "result manifest")? != sha256_file(protocol_path)? {
        return Err("result is bound to different protocol bytes".into());
    }
    let revision = string(manifest, "source_revision", "result manifest")?;
    if !is_hex(revision, 40)
        || !is_hex(string(manifest, "binary_sha256", "result manifest")?, 64)
        || !is_hex(string(manifest, "runner_sha256", "result manifest")?, 64)
        || !image_uri(string(manifest, "image_uri", "result manifest")?)
    {
        return Err("invalid source, binary, runner, or image identity".into());
    }
    let archive = object(
        field(manifest, "historical_archive", "result manifest")?,
        "manifest archive",
    )?;
    if string(archive, "sha256", "manifest archive")? != HISTORICAL_SHA256
        || unsigned(archive, "bytes", "manifest archive")? != HISTORICAL_BYTES
    {
        return Err("manifest historical archive mismatch".into());
    }
    let spending = object(field(manifest, "spending", "result manifest")?, "spending")?;
    if finite_nonnegative(spending, "all_attempts_usd", "spending")? > 5.0
        || unsigned(spending, "manual_retries", "spending")? > 1
    {
        return Err("budget or retry ceiling exceeded".into());
    }
    let retry_reasons = array(
        field(spending, "retry_reasons", "spending")?,
        "retry reasons",
    )?;
    if retry_reasons.len() as u64 != unsigned(spending, "manual_retries", "spending")?
        || !retry_reasons
            .iter()
            .all(|reason| reason.as_str() == Some("infrastructure"))
    {
        return Err("retry reasons are not exactly infrastructure retries".into());
    }
    let splits = object(field(manifest, "splits", "result manifest")?, "splits")?;
    for split in ["low", "mid", "high", "top"] {
        let row = object(field(splits, split, "splits")?, "split")?;
        if string(row, "state", "split")? != "succeeded"
            || string(row, "job_uid", "split")?.is_empty()
            || string(row, "pod_uid", "split")?.is_empty()
            || !is_hex(string(row, "receipt_sha256", "split")?, 64)
        {
            return Err(format!("split is incomplete: {split}"));
        }
    }
    if !boolean(manifest, "wave1_terminal_before_wave2", "result manifest")? {
        return Err("wave ordering not established".into());
    }

    let expected = expected_cells();
    let mut observed = BTreeSet::new();
    for cell in array(field(manifest, "cells", "result manifest")?, "cells")? {
        let cell = object(cell, "cell")?;
        let key = (
            string(cell, "role", "cell")?.to_owned(),
            unsigned(cell, "q", "cell")?,
            unsigned(cell, "seed", "cell")?,
            string(cell, "arm", "cell")?.to_owned(),
        );
        let repetitions = *expected
            .get(&key)
            .ok_or_else(|| format!("unexpected cell: {key:?}"))?;
        if !observed.insert(key.clone()) {
            return Err(format!("duplicate cell: {key:?}"));
        }
        if string(cell, "target_policy", "cell")? != "ic-repeat"
            || string(cell, "status", "cell")? != "complete"
            || unsigned(cell, "expected_repetitions", "cell")? != repetitions
            || unsigned(cell, "observed_repetitions", "cell")? != repetitions
            || unsigned(cell, "scalar_verifications", "cell")? != repetitions
            || !boolean(cell, "count_authoritative", "cell")?
            || !is_hex(string(cell, "target_digest", "cell")?, 64)
        {
            return Err(format!("incomplete cell: {key:?}"));
        }
        phase_set(
            field(cell, "phase_coverage", "cell")?,
            &RHO_PHASES,
            "rho phases",
        )?;
    }
    if observed != expected.keys().cloned().collect::<BTreeSet<_>>() {
        return Err("primary/diagnostic cell census incomplete".into());
    }

    let expected_calibrations = GRID.len() * SEEDS.len();
    let mut calibrations = BTreeSet::new();
    for row in array(
        field(manifest, "calibrations", "result manifest")?,
        "calibrations",
    )? {
        let row = object(row, "calibration")?;
        let key = (
            unsigned(row, "q", "calibration")?,
            unsigned(row, "seed", "calibration")?,
        );
        if !GRID.iter().any(|grid| grid.1 == key.0)
            || !SEEDS.contains(&key.1)
            || !calibrations.insert(key)
            || unsigned(row, "triples", "calibration")? != 600
            || finite_nonnegative(row, "r_inv", "calibration")? <= 0.0
            || finite_nonnegative(row, "r_lmul", "calibration")? <= 0.0
        {
            return Err(format!("invalid calibration: {key:?}"));
        }
    }
    if calibrations.len() != expected_calibrations {
        return Err("calibration census incomplete".into());
    }
    let ic = object(
        field(manifest, "historical_ic_coverage", "result manifest")?,
        "historical IC coverage",
    )?;
    string_set(
        field(ic, "declared_arms", "historical IC coverage")?,
        &IC_ARMS,
        "IC arms",
    )?;
    phase_set(
        field(ic, "phase_coverage", "historical IC coverage")?,
        &IC_PHASES,
        "IC phases",
    )?;
    if !boolean(ic, "scalar_verification_complete", "historical IC coverage")? {
        return Err("historical IC scalar verification incomplete".into());
    }
    let replay = object(
        field(manifest, "duplicate_replay", "result manifest")?,
        "replay",
    )?;
    if unsigned(replay, "q", "replay")? != 4_051
        || unsigned(replay, "seed", "replay")? != 1
        || !boolean(replay, "non_timing_identical", "replay")?
    {
        return Err("duplicate replay failed".into());
    }
    let independent = object(
        field(manifest, "independent_validation", "result manifest")?,
        "independent validation",
    )?;
    if !boolean(independent, "replay_all", "independent validation")?
        || !boolean(independent, "headline_recomputed", "independent validation")?
        || !is_hex(
            string(independent, "receipt_sha256", "independent validation")?,
            64,
        )
    {
        return Err("independent validation incomplete".into());
    }

    let root = manifest_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .canonicalize()
        .map_err(|error| format!("manifest root: {error}"))?;
    let mut artifact_paths = BTreeSet::new();
    let artifacts = array(
        field(manifest, "artifacts", "result manifest")?,
        "artifacts",
    )?;
    if artifacts.is_empty() {
        return Err("artifact manifest empty".into());
    }
    for artifact in artifacts {
        let artifact = object(artifact, "artifact")?;
        let relative = Path::new(string(artifact, "path", "artifact")?);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| component == Component::ParentDir)
        {
            return Err(format!("unsafe artifact path: {}", relative.display()));
        }
        let path = root.join(relative);
        let canonical = path
            .canonicalize()
            .map_err(|error| format!("artifact {}: {error}", relative.display()))?;
        if !canonical.starts_with(&root) || !artifact_paths.insert(canonical.clone()) {
            return Err(format!(
                "escaping or duplicate artifact: {}",
                relative.display()
            ));
        }
        let metadata = fs::metadata(&canonical)
            .map_err(|error| format!("artifact {}: {error}", relative.display()))?;
        if metadata.len() != unsigned(artifact, "bytes", "artifact")?
            || sha256_file(&canonical)? != string(artifact, "sha256", "artifact")?
        {
            return Err(format!(
                "artifact identity mismatch: {}",
                relative.display()
            ));
        }
    }
    let primary_runs = expected
        .iter()
        .filter(|(key, _)| key.0 == "rho_primary")
        .map(|(_, repetitions)| *repetitions)
        .sum::<u64>();
    let diagnostic_runs = expected
        .iter()
        .filter(|(key, _)| key.0 == "rho_diagnostic")
        .map(|(_, repetitions)| *repetitions)
        .sum::<u64>();
    Ok(BTreeMap::from([
        ("source_revision", revision.to_owned()),
        ("cells", observed.len().to_string()),
        ("primary_runs", primary_runs.to_string()),
        ("diagnostic_runs", diagnostic_runs.to_string()),
        ("calibrations", calibrations.len().to_string()),
        ("artifacts", artifact_paths.len().to_string()),
    ]))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Components {
    pub fq_mul: u64,
    pub fq_inv: u64,
    pub mod_ell_mul: u64,
    pub band_ops: u64,
}

impl Components {
    pub fn from_value(value: &Value, label: &str) -> GateResult<Self> {
        let object = object(value, label)?;
        let keys = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
        if keys
            != ["band_ops", "fq_inv", "fq_mul", "mod_ell_mul"]
                .into_iter()
                .collect()
        {
            return Err(format!("{label} has wrong component keys"));
        }
        Ok(Self {
            fq_mul: unsigned(object, "fq_mul", label)?,
            fq_inv: unsigned(object, "fq_inv", label)?,
            mod_ell_mul: unsigned(object, "mod_ell_mul", label)?,
            band_ops: unsigned(object, "band_ops", label)?,
        })
    }

    pub fn price(self, r_inv: f64, r_lmul: f64, weight: f64) -> f64 {
        self.fq_mul as f64
            + r_inv * self.fq_inv as f64
            + r_lmul * self.mod_ell_mul as f64
            + weight * (56.0 + r_inv) * self.band_ops as f64
    }
}

pub fn classify(size_rows: &[(bool, Vec<(f64, f64)>)]) -> &'static str {
    let mut unadmitted = false;
    let mut unresolved = false;
    let mut rejected = false;
    for (admitted, intervals) in size_rows {
        if !admitted {
            unadmitted = true;
            continue;
        }
        for (lower, upper) in intervals {
            rejected |= *lower >= 1.0;
            unresolved |= *upper >= 1.0;
        }
    }
    if rejected {
        "rejected"
    } else if unresolved {
        "unresolved_interval_crosses_one"
    } else if unadmitted {
        "limited_unadmitted_sizes"
    } else {
        "supported_on_measured_grid"
    }
}

pub fn selftest() -> GateResult<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocol = read_json(&root.join("protocol.json"))?;
    let templates = read_json(&root.join("request_templates.json"))?;
    validate_protocol(&protocol)?;
    validate_templates(&templates)?;

    let mut mutation = protocol.clone();
    mutation["grid"][0]["q"] = Value::from(4_053_u64);
    if validate_protocol(&mutation).is_ok() {
        return Err("grid mutation was accepted".into());
    }
    let mut mutation = protocol.clone();
    mutation["arms"]["rho_primary"] = Value::from("strong-b8");
    if validate_protocol(&mutation).is_ok() {
        return Err("primary-arm mutation was accepted".into());
    }
    let mut mutation = protocol.clone();
    mutation["execution"]["usd_all_attempt_ceiling"] = Value::from(6.0);
    if validate_protocol(&mutation).is_ok() {
        return Err("budget mutation was accepted".into());
    }
    let mut mutation = templates.clone();
    mutation["materialization"]["launch_ready"] = Value::from(true);
    if validate_templates(&mutation).is_ok() {
        return Err("unbound launch-ready template was accepted".into());
    }
    let mut mutation = templates.clone();
    mutation["materialization"]["runner_contract"]["present"] = Value::from(true);
    if validate_templates(&mutation).is_ok() {
        return Err("unreviewed runner-presence mutation was accepted".into());
    }
    let unit = Components {
        fq_mul: 100,
        fq_inv: 2,
        mod_ell_mul: 3,
        band_ops: 4,
    };
    let expected = 100.0 + 10.0 * 2.0 + 0.5 * 3.0 + 0.1 * 66.0 * 4.0;
    if (unit.price(10.0, 0.5, 0.1) - expected).abs() > 1e-12 {
        return Err("cost formula self-test failed".into());
    }
    if classify(&[(true, vec![(0.6, 0.9)])]) != "supported_on_measured_grid"
        || classify(&[(false, vec![])]) != "limited_unadmitted_sizes"
        || classify(&[(true, vec![(0.8, 1.1)])]) != "unresolved_interval_crosses_one"
        || classify(&[(true, vec![(1.0, 1.2)])]) != "rejected"
    {
        return Err("verdict classifier self-test failed".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frozen_contract_and_tamper_matrix() {
        selftest().unwrap();
    }

    #[test]
    fn expected_run_census() {
        let expected = expected_cells();
        let primary = expected
            .iter()
            .filter(|(key, _)| key.0 == "rho_primary")
            .map(|(_, repetitions)| *repetitions)
            .sum::<u64>();
        let diagnostics = expected
            .iter()
            .filter(|(key, _)| key.0 == "rho_diagnostic")
            .map(|(_, repetitions)| *repetitions)
            .sum::<u64>();
        assert_eq!(primary, 141);
        assert_eq!(diagnostics, 27);
        assert_eq!(expected.len(), 60);
    }
}
