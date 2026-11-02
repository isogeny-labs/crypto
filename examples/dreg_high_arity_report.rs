//! Deterministic report/export builder for the high-arity closure study.
//!
//! The machine export follows the canonical contract in
//! `crypto-autoresearcher/schemas/experiment-export.v1.json`; the scientific
//! classifications come only from `dreg_high_arity_analyze`.  This program
//! does not reinterpret closure evidence or clocks.
//!
//! Usage:
//! `dreg_high_arity_report --summary SUMMARY --protocol PROTOCOL --out-dir DIR [--check]`

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};

const MAX_JSON_BYTES: u64 = 32 * 1024 * 1024;
const OUTPUT_NAMES: [&str; 5] = [
    "REPORT.md",
    "closure-outcomes.svg",
    "report.typ",
    "report.pdf",
    "experiment.export.json",
];

struct Args {
    summary: PathBuf,
    protocol: PathBuf,
    out_dir: PathBuf,
    check: bool,
}

#[derive(Clone)]
struct Arm {
    id: String,
    role: String,
    binary_sha256: String,
    commit: String,
    image_digest: Option<String>,
}

struct Protocol {
    id: String,
    version: u64,
    created: String,
    repository: String,
    question: String,
    sha256: String,
    source_commit: String,
    backend: String,
    arms: Vec<Arm>,
    ladder_sha256: String,
    analyzer_sha256: String,
    report_sha256: String,
    protocol_path: PathBuf,
    binary_manifest_path: PathBuf,
    binary_manifest_sha256: String,
    analysis_path: PathBuf,
    analysis_sha256: String,
}

struct Bundle {
    files: BTreeMap<&'static str, Vec<u8>>,
}

fn usage(program: &str) -> String {
    format!("usage: {program} --summary SUMMARY --protocol PROTOCOL --out-dir DIR [--check]")
}

fn parse_args() -> Result<Args, String> {
    let raw: Vec<String> = env::args().collect();
    let program = raw
        .first()
        .map(String::as_str)
        .unwrap_or("dreg_high_arity_report");
    let mut summary = None;
    let mut protocol = None;
    let mut out_dir = None;
    let mut check = false;
    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--summary" | "--protocol" | "--out-dir" => {
                let flag = raw[i].clone();
                i += 1;
                let value = raw
                    .get(i)
                    .ok_or_else(|| format!("{flag} requires a value\n{}", usage(program)))?;
                match flag.as_str() {
                    "--summary" if summary.replace(PathBuf::from(value)).is_some() => {
                        return Err("--summary was supplied more than once".into());
                    }
                    "--protocol" if protocol.replace(PathBuf::from(value)).is_some() => {
                        return Err("--protocol was supplied more than once".into());
                    }
                    "--out-dir" if out_dir.replace(PathBuf::from(value)).is_some() => {
                        return Err("--out-dir was supplied more than once".into());
                    }
                    _ => {}
                }
            }
            "--check" if !check => check = true,
            "--check" => return Err("--check was supplied more than once".into()),
            other => return Err(format!("unknown argument {other:?}\n{}", usage(program))),
        }
        i += 1;
    }
    Ok(Args {
        summary: summary.ok_or_else(|| usage(program))?,
        protocol: protocol.ok_or_else(|| usage(program))?,
        out_dir: out_dir.ok_or_else(|| usage(program))?,
        check,
    })
}

fn read_json(path: &Path) -> Result<(Vec<u8>, Value), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| format!("cannot inspect {}: {e}", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!("input is not an ordinary file: {}", path.display()));
    }
    if metadata.len() > MAX_JSON_BYTES {
        return Err(format!(
            "input exceeds the {MAX_JSON_BYTES}-byte limit: {}",
            path.display()
        ));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    File::open(path)
        .and_then(|file| file.take(MAX_JSON_BYTES + 1).read_to_end(&mut bytes))
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let value = serde_json::from_slice(&bytes)
        .map_err(|e| format!("{} is not one JSON document: {e}", path.display()))?;
    Ok((bytes, value))
}

fn object<'a>(value: &'a Value, what: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{what} is not a JSON object"))
}

fn string<'a>(value: &'a Value, key: &str, what: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| format!("{what}.{key} is not a nonempty string"))
}

fn array<'a>(value: &'a Value, key: &str, what: &str) -> Result<&'a [Value], String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("{what}.{key} is not an array"))
}

fn is_hex(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn digest_file(path: &Path) -> Result<(String, u64), String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| format!("cannot inspect {}: {e}", path.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!("not an ordinary file: {}", path.display()));
    }
    let mut hasher = Sha256::new();
    let mut file = File::open(path).map_err(|e| format!("cannot open {}: {e}", path.display()))?;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|e| format!("cannot hash {}: {e}", path.display()))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok((format!("{:x}", hasher.finalize()), metadata.len()))
}

fn parse_protocol(value: &Value, protocol_path: &Path) -> Result<Protocol, String> {
    object(value, "protocol")?;
    if string(value, "schema", "protocol")? != "experiment.protocol/v1" {
        return Err("protocol has the wrong schema".into());
    }
    let freeze = value
        .get("freeze")
        .ok_or_else(|| "protocol.freeze is missing".to_string())?;
    if string(freeze, "status", "protocol.freeze")? != "frozen" {
        return Err("an experiment export may bind only a frozen protocol".into());
    }
    let protocol_sha256 = string(freeze, "protocol_sha256", "protocol.freeze")?;
    if !is_hex(protocol_sha256, 64) {
        return Err("protocol.freeze.protocol_sha256 is not lowercase SHA-256".into());
    }
    let freeze_commit = string(freeze, "commit", "protocol.freeze")?;
    if !is_hex(freeze_commit, 40) {
        return Err("protocol.freeze.commit is not a full lowercase commit".into());
    }
    let mut canonical_body = value.clone();
    canonical_body
        .as_object_mut()
        .ok_or_else(|| "protocol is not an object".to_string())?
        .remove("freeze");
    let canonical_body = serde_json::to_vec(&canonical_body).map_err(|e| e.to_string())?;
    if digest(&canonical_body) != protocol_sha256 {
        return Err("protocol body differs from its frozen SHA-256".into());
    }
    let canonical_protocol_path = absolute_existing(protocol_path)?;
    let binary_manifest_path = canonical_protocol_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("binary.sha256");
    let (binary_manifest_sha256, _) = digest_file(&binary_manifest_path)?;
    let raw_arms = array(value, "arms", "protocol")?;
    if raw_arms.len() != 5 {
        return Err(format!(
            "protocol has {} arms, expected five",
            raw_arms.len()
        ));
    }
    let mut arms = Vec::new();
    let mut ids = BTreeSet::new();
    let mut commits = BTreeSet::new();
    let mut ladder_hashes = BTreeSet::new();
    let mut analyzer_hashes = BTreeSet::new();
    let mut report_hashes = BTreeSet::new();
    for (index, raw) in raw_arms.iter().enumerate() {
        let label = format!("protocol.arms[{index}]");
        let id = string(raw, "id", &label)?.to_string();
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate protocol arm {id}"));
        }
        let role = string(raw, "role", &label)?.to_string();
        if !matches!(
            role.as_str(),
            "reference"
                | "baseline"
                | "candidate"
                | "control_aa"
                | "control_positive"
                | "control_negative"
        ) {
            return Err(format!("{label}.role is not an export role"));
        }
        let instrument = raw
            .get("instrument")
            .ok_or_else(|| format!("{label}.instrument is missing"))?;
        let commit = string(instrument, "commit", &format!("{label}.instrument"))?.to_string();
        let binary =
            string(instrument, "binary_sha256", &format!("{label}.instrument"))?.to_string();
        if !is_hex(&commit, 40) || !is_hex(&binary, 64) {
            return Err(format!(
                "{label} has an invalid frozen commit or binary hash"
            ));
        }
        commits.insert(commit.clone());
        let image_digest = instrument
            .get("image_digest")
            .and_then(Value::as_str)
            .map(str::to_string);
        let flags = instrument
            .get("flags")
            .ok_or_else(|| format!("{label}.instrument.flags is missing"))?;
        for (key, set) in [
            ("ladder_binary_sha256", &mut ladder_hashes),
            ("analyzer_binary_sha256", &mut analyzer_hashes),
            ("report_binary_sha256", &mut report_hashes),
        ] {
            let hash = string(flags, key, &format!("{label}.instrument.flags"))?;
            if !is_hex(hash, 64) {
                return Err(format!("{label}.instrument.flags.{key} is not frozen"));
            }
            set.insert(hash.to_string());
        }
        arms.push(Arm {
            id,
            role,
            binary_sha256: binary,
            commit,
            image_digest,
        });
    }
    if commits.len() != 1 || commits.first().map(String::as_str) != Some(freeze_commit) {
        return Err("protocol arms do not share the frozen source commit".into());
    }
    if ladder_hashes.len() != 1 || analyzer_hashes.len() != 1 || report_hashes.len() != 1 {
        return Err("protocol arms disagree on auxiliary binary identities".into());
    }
    let analysis = value
        .get("analysis_plan")
        .and_then(|v| v.get("script"))
        .ok_or_else(|| "protocol.analysis_plan.script is missing".to_string())?;
    let analysis_rel = string(analysis, "path", "protocol.analysis_plan.script")?;
    let analysis_sha256 = string(analysis, "sha256", "protocol.analysis_plan.script")?;
    if !is_hex(analysis_sha256, 64) {
        return Err("protocol analysis SHA-256 is not frozen".into());
    }
    let analysis_path = canonical_protocol_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(analysis_rel);
    let (actual_analysis, _) = digest_file(&analysis_path)?;
    if actual_analysis != analysis_sha256 {
        return Err(format!(
            "analysis source hash differs from the frozen protocol: {}",
            analysis_path.display()
        ));
    }
    let execution = value
        .get("execution")
        .ok_or_else(|| "protocol.execution is missing".to_string())?;
    let backend = string(execution, "backend", "protocol.execution")?.to_string();
    if !matches!(
        backend.as_str(),
        "local"
            | "taskq"
            | "isolab"
            | "isolab-k8s"
            | "spotlab"
            | "spot-system"
            | "ephemeral"
            | "modal"
            | "runpod"
            | "cairn"
            | "slurm"
            | "manual"
            | "ecbench"
            | "runbook"
            | "csd-lab"
            | "mixed"
    ) {
        return Err(format!("unsupported export backend {backend:?}"));
    }
    let version = value
        .get("version")
        .and_then(Value::as_u64)
        .filter(|v| *v >= 1)
        .ok_or_else(|| "protocol.version is not a positive integer".to_string())?;
    Ok(Protocol {
        id: string(value, "protocol_id", "protocol")?.to_string(),
        version,
        created: string(value, "created", "protocol")?.to_string(),
        repository: string(value, "repository", "protocol")?.to_string(),
        question: string(value, "question", "protocol")?.to_string(),
        sha256: protocol_sha256.to_string(),
        source_commit: freeze_commit.to_string(),
        backend,
        arms,
        ladder_sha256: ladder_hashes.into_iter().next().expect("five arms"),
        analyzer_sha256: analyzer_hashes.into_iter().next().expect("five arms"),
        report_sha256: report_hashes.into_iter().next().expect("five arms"),
        protocol_path: canonical_protocol_path,
        binary_manifest_path,
        binary_manifest_sha256,
        analysis_path,
        analysis_sha256: analysis_sha256.to_string(),
    })
}

const REGISTERED_CELLS: [&str; 3] = ["m5-n11-l2", "m5-n13-l3", "m6-n11-l2"];

fn required_u64(value: &Value, key: &str, what: &str) -> Result<u64, String> {
    value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("{what}.{key} is not a nonnegative integer"))
}

fn u64_array(value: &Value, key: &str, what: &str) -> Result<Vec<u64>, String> {
    array(value, key, what)?
        .iter()
        .enumerate()
        .map(|(index, item)| {
            item.as_u64()
                .ok_or_else(|| format!("{what}.{key}[{index}] is not an integer"))
        })
        .collect()
}

fn validate_execution_pair(value: &Value, what: &str, required: bool) -> Result<bool, String> {
    match (value.get("execution_ref"), value.get("execution_commit")) {
        (Some(Value::String(reference)), Some(Value::String(commit)))
            if reference.starts_with("refs/heads/") && is_hex(commit, 40) =>
        {
            Ok(true)
        }
        (Some(Value::Null), Some(Value::Null)) if !required => Ok(false),
        _ if required => Err(format!(
            "{what} lacks the pushed execution ref/commit required by its retained claim"
        )),
        _ => Err(format!(
            "{what} has a partial or malformed pushed execution identity"
        )),
    }
}

fn validate_frozen_provenance(value: &Value, protocol: &Protocol) -> Result<(), String> {
    let provenance = value
        .get("frozen_provenance")
        .ok_or_else(|| "summary.frozen_provenance is missing".to_string())?;
    let object = object(provenance, "summary.frozen_provenance")?;
    let expected_keys: BTreeSet<_> = [
        "state",
        "protocol",
        "binary_manifest",
        "protocol_sha256",
        "source_commit",
        "binary_manifest_sha256",
        "reason",
    ]
    .into_iter()
    .collect();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected_keys {
        return Err("summary.frozen_provenance has missing or unexpected fields".into());
    }
    if string(provenance, "state", "summary.frozen_provenance")? != "ready" {
        return Err("summary frozen provenance is not ready".into());
    }
    let reported_protocol = absolute_existing(Path::new(string(
        provenance,
        "protocol",
        "summary.frozen_provenance",
    )?))?;
    let reported_manifest = absolute_existing(Path::new(string(
        provenance,
        "binary_manifest",
        "summary.frozen_provenance",
    )?))?;
    if reported_protocol != protocol.protocol_path
        || reported_manifest != protocol.binary_manifest_path
        || string(provenance, "protocol_sha256", "summary.frozen_provenance")? != protocol.sha256
        || string(provenance, "source_commit", "summary.frozen_provenance")?
            != protocol.source_commit
        || string(
            provenance,
            "binary_manifest_sha256",
            "summary.frozen_provenance",
        )? != protocol.binary_manifest_sha256
    {
        return Err("summary frozen provenance differs from the supplied frozen protocol".into());
    }
    string(provenance, "reason", "summary.frozen_provenance")?;
    Ok(())
}

fn validate_summary(value: &Value, protocol: &Protocol) -> Result<(), String> {
    object(value, "summary")?;
    if string(value, "schema", "summary")? != "dreg_high_arity.summary/v1" {
        return Err("summary has the wrong schema".into());
    }
    if value.get("timing_interpreted").and_then(Value::as_bool) != Some(false) {
        return Err("summary must explicitly state timing_interpreted=false".into());
    }
    validate_frozen_provenance(value, protocol)?;
    let global = string(value, "global_classification", "summary")?;
    if !matches!(
        global,
        "invalid_instrument"
            | "unmeasured"
            | "not_universally_four_in_registered_panel"
            | "bounded_four_across_registered_cells"
    ) {
        return Err(format!("unknown global classification {global:?}"));
    }
    let rows = array(value, "rows", "summary")?;
    if rows.len() != 19 {
        return Err(format!(
            "summary retains {} subprocess rows, expected 19",
            rows.len()
        ));
    }
    let panels = array(value, "draw_panels", "summary")?;
    if panels.len() != 3 {
        return Err("summary must retain exactly three draw panels".into());
    }
    let mut expected_rows: Vec<(String, &'static str, String, Option<u64>)> = vec![(
        "identity-m4-n12-l5-d24".into(),
        "identity",
        "m4-n12-l5".into(),
        None,
    )];
    let mut panel_states = BTreeMap::new();
    let mut cell_controls: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut cell_unsat: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (index, (panel, expected_cell)) in panels.iter().zip(REGISTERED_CELLS).enumerate() {
        let label = format!("summary.draw_panels[{index}]");
        if string(panel, "cell", &label)? != expected_cell {
            return Err(format!(
                "{label} is not the registered cell {expected_cell}"
            ));
        }
        let state = string(panel, "state", &label)?;
        if !matches!(state, "ready" | "invalid" | "unmeasured") {
            return Err(format!("{label}.state has unknown value {state:?}"));
        }
        let sat = u64_array(panel, "satisfiable_draws", &label)?;
        let unsat = u64_array(panel, "unsat_draws", &label)?;
        if sat.len() > 2 || unsat.len() > 4 {
            return Err(format!(
                "{label} exceeds the registered two-SAT/four-UNSAT panel"
            ));
        }
        let mut draws = BTreeSet::new();
        if sat.iter().chain(&unsat).any(|draw| !draws.insert(*draw)) {
            return Err(format!("{label} repeats a draw index"));
        }
        if state == "ready" && (sat.len() != 2 || unsat.len() != 4) {
            return Err(format!(
                "{label} is ready without two SAT and four UNSAT draws"
            ));
        }
        validate_execution_pair(panel, &label, state == "ready")?;
        required_u64(panel, "records", &label)?;
        string(panel, "path", &label)?;
        array(panel, "errors", &label)?;
        panel_states.insert(expected_cell.to_string(), state.to_string());

        let controls: Vec<_> = (0..2)
            .map(|slot| {
                sat.get(slot).map_or_else(
                    || format!("control-{expected_cell}-slot{slot}"),
                    |draw| format!("control-{expected_cell}-d{draw}"),
                )
            })
            .collect();
        let unsat_rows: Vec<_> = (0..4)
            .map(|slot| {
                unsat.get(slot).map_or_else(
                    || format!("unsat-{expected_cell}-slot{slot}"),
                    |draw| format!("unsat-{expected_cell}-d{draw}"),
                )
            })
            .collect();
        for (slot, run_id) in controls.iter().enumerate() {
            expected_rows.push((
                run_id.clone(),
                "satisfiable_control",
                expected_cell.to_string(),
                sat.get(slot).copied(),
            ));
        }
        for (slot, run_id) in unsat_rows.iter().enumerate() {
            expected_rows.push((
                run_id.clone(),
                "unsat",
                expected_cell.to_string(),
                unsat.get(slot).copied(),
            ));
        }
        cell_controls.insert(expected_cell.to_string(), controls);
        cell_unsat.insert(expected_cell.to_string(), unsat_rows);
    }

    let mut row_map = BTreeMap::new();
    for (index, (row, (run_id, expected_kind, expected_cell, expected_draw))) in
        rows.iter().zip(&expected_rows).enumerate()
    {
        let label = format!("summary.rows[{index}]");
        if string(row, "run_id", &label)? != run_id
            || string(row, "kind", &label)? != *expected_kind
            || string(row, "cell", &label)? != expected_cell
            || row.get("draw").and_then(Value::as_u64) != *expected_draw
            || (expected_draw.is_none() && row.get("draw") != Some(&Value::Null))
        {
            return Err(format!(
                "{label} differs from registered subprocess {run_id}"
            ));
        }
        let class = string(row, "classification", &label)?;
        let class_valid = match *expected_kind {
            "identity" => matches!(
                class,
                "passed" | "unmeasured" | "invalid_record" | "invalid_instrument"
            ),
            "satisfiable_control" => {
                matches!(class, "passed" | "unmeasured" | "invalid_record")
            }
            "unsat" => matches!(
                class,
                "closure_by_four" | "above_four" | "unmeasured" | "invalid_record"
            ),
            _ => false,
        };
        if !class_valid {
            return Err(format!(
                "{label} has impossible {expected_kind} class {class:?}"
            ));
        }
        if *expected_kind != "identity" {
            match panel_states
                .get(expected_cell)
                .map(String::as_str)
                .expect("registered panel")
            {
                "invalid" if class != "invalid_record" => {
                    return Err(format!(
                        "{label} was not invalidated with its invalid draw panel"
                    ));
                }
                "unmeasured" if !matches!(class, "unmeasured" | "invalid_record") => {
                    return Err(format!(
                        "{label} claims a result from an unmeasured draw panel"
                    ));
                }
                _ => {}
            }
        }
        string(row, "reason", &label)?;
        let attempts = array(row, "attempts", &label)?;
        let selected_attempt = match row.get("selected_attempt") {
            Some(Value::String(attempt)) if !attempt.is_empty() => Some(attempt.as_str()),
            Some(Value::Null) => None,
            _ => return Err(format!("{label}.selected_attempt is malformed")),
        };
        let mut selected_execution_bound = false;
        for attempt in attempts {
            let attempt_name = string(attempt, "attempt", "attempt")?;
            let attempt_state = string(attempt, "state", "attempt")?;
            let pair_present =
                validate_execution_pair(attempt, "attempt", attempt_state != "invalid")?;
            if selected_attempt == Some(attempt_name) {
                if !pair_present {
                    return Err(format!(
                        "{label} selected attempt {attempt_name} lacks execution provenance"
                    ));
                }
                selected_execution_bound = true;
            }
            if let Some(termination) = attempt.get("termination").and_then(Value::as_str) {
                if !matches!(
                    termination,
                    "success"
                        | "timeout"
                        | "signal_kill"
                        | "signal_term"
                        | "address_space"
                        | "checkpoint_invalid"
                        | "nonzero_exit"
                ) {
                    return Err(format!("{label} has unknown termination {termination:?}"));
                }
            }
        }
        if selected_attempt.is_some() && !selected_execution_bound {
            return Err(format!(
                "{label}.selected_attempt does not identify a provenance-bound retained attempt"
            ));
        }
        if matches!(class, "passed" | "closure_by_four" | "above_four")
            && selected_attempt.is_none()
        {
            return Err(format!(
                "{label} carries a scientific claim without a selected attempt"
            ));
        }
        if row_map.insert(run_id.clone(), row).is_some() {
            return Err(format!("duplicate summary subprocess {run_id}"));
        }
    }
    let packages = array(value, "packages", "summary")?;
    if packages.len() != 5 {
        return Err(format!(
            "summary has {} packages, expected five",
            packages.len()
        ));
    }
    let protocol_ids: BTreeSet<_> = protocol.arms.iter().map(|arm| arm.id.as_str()).collect();
    let protocol_roles: BTreeMap<_, _> = protocol
        .arms
        .iter()
        .map(|arm| (arm.id.as_str(), arm.role.as_str()))
        .collect();
    let expected_protocol_roles: BTreeMap<_, _> = [
        ("identity-replay", "reference"),
        ("satisfiable-controls", "control_positive"),
        ("m5-n11-l2", "candidate"),
        ("m5-n13-l3", "candidate"),
        ("m6-n11-l2", "candidate"),
    ]
    .into_iter()
    .collect();
    if protocol_ids != expected_protocol_roles.keys().copied().collect()
        || protocol_roles != expected_protocol_roles
    {
        return Err("protocol arms differ from the registered package identities or roles".into());
    }

    let identity = row_map
        .get("identity-m4-n12-l5-d24")
        .ok_or_else(|| "identity row is missing".to_string())?;
    let identity_class = string(identity, "classification", "identity row")?;
    let identity_reason = string(identity, "reason", "identity row")?;
    let control_ids: Vec<String> = REGISTERED_CELLS
        .iter()
        .flat_map(|cell| cell_controls.get(*cell).expect("registered cell").clone())
        .collect();
    let controls: Vec<&Value> = control_ids
        .iter()
        .map(|run_id| {
            row_map
                .get(run_id)
                .copied()
                .ok_or_else(|| format!("control row {run_id} is missing"))
        })
        .collect::<Result<_, _>>()?;
    let controls_invalid = controls.iter().any(|row| {
        matches!(
            row.get("classification").and_then(Value::as_str),
            Some("invalid_record" | "invalid_instrument")
        )
    });
    let controls_passed = controls
        .iter()
        .filter(|row| row.get("classification").and_then(Value::as_str) == Some("passed"))
        .count();
    let sat_package_class = if controls_invalid {
        "invalid"
    } else if controls_passed == 6 {
        "passed"
    } else {
        "unmeasured"
    };
    let sat_package_reason = match sat_package_class {
        "passed" => "all six complete saturated controls remained unrefuted",
        "invalid" => "at least one satisfiable control is invalid or refuted",
        _ => "at least one satisfiable control is missing or incomplete",
    };

    let cells = array(value, "cells", "summary")?;
    if cells.len() != 3 {
        return Err("summary must retain exactly three cell classifications".into());
    }
    let mut derived_cells = BTreeMap::new();
    for (index, (cell, cell_id)) in cells.iter().zip(REGISTERED_CELLS).enumerate() {
        let label = format!("summary.cells[{index}]");
        if string(cell, "cell", &label)? != cell_id {
            return Err(format!("{label} is not registered cell {cell_id}"));
        }
        let unsat_ids = cell_unsat.get(cell_id).expect("registered cell");
        let unsat_rows: Vec<_> = unsat_ids
            .iter()
            .map(|run_id| row_map.get(run_id).copied().expect("registered row"))
            .collect();
        let count = |name: &str| {
            unsat_rows
                .iter()
                .filter(|row| row.get("classification").and_then(Value::as_str) == Some(name))
                .count()
        };
        let closure = count("closure_by_four");
        let above = count("above_four");
        let unmeasured = count("unmeasured");
        let invalid = count("invalid_record");
        let panel_state = panel_states.get(cell_id).expect("registered panel");
        let controls_unmeasured = controls
            .iter()
            .any(|row| row.get("classification").and_then(Value::as_str) == Some("unmeasured"));
        let classification = if panel_state == "invalid"
            || matches!(identity_class, "invalid_record" | "invalid_instrument")
            || controls_invalid
            || invalid > 0
        {
            "invalid_instrument"
        } else if panel_state != "ready"
            || identity_class != "passed"
            || controls_unmeasured
            || controls_passed != 6
        {
            "unmeasured"
        } else if closure + above == 4 && closure > 0 && above > 0 {
            "draw_dependent"
        } else if above > 0 {
            "not_universally_four_at_cell"
        } else if unmeasured > 0 {
            "unmeasured"
        } else if closure == 4 {
            "bounded_four_at_cell"
        } else {
            "invalid_instrument"
        };
        for (key, expected) in [
            ("closure_by_four", closure),
            ("above_four", above),
            ("unmeasured", unmeasured),
            ("invalid", invalid),
            ("controls_passed", controls_passed),
        ] {
            if required_u64(cell, key, &label)? != expected as u64 {
                return Err(format!(
                    "{label}.{key} disagrees with retained subprocess rows"
                ));
            }
        }
        if string(cell, "classification", &label)? != classification {
            return Err(format!(
                "{label}.classification disagrees with retained evidence"
            ));
        }
        derived_cells.insert(cell_id.to_string(), classification.to_string());
    }

    let expected_packages: Vec<(&str, Vec<String>, String, String)> = {
        let mut values = vec![
            (
                "identity-replay",
                vec!["identity-m4-n12-l5-d24".to_string()],
                identity_class.to_string(),
                identity_reason.to_string(),
            ),
            (
                "satisfiable-controls",
                control_ids.clone(),
                sat_package_class.to_string(),
                sat_package_reason.to_string(),
            ),
        ];
        for cell_id in REGISTERED_CELLS {
            let ids = cell_unsat.get(cell_id).expect("registered cell").clone();
            let class = derived_cells.get(cell_id).expect("registered cell").clone();
            let cell = cells
                .iter()
                .find(|cell| cell.get("cell").and_then(Value::as_str) == Some(cell_id))
                .expect("registered cell");
            let reason = format!(
                "{} closure-by-four, {} above-four, {} unmeasured, {} invalid; {}/6 shared controls passed",
                required_u64(cell, "closure_by_four", "cell").expect("validated"),
                required_u64(cell, "above_four", "cell").expect("validated"),
                required_u64(cell, "unmeasured", "cell").expect("validated"),
                required_u64(cell, "invalid", "cell").expect("validated"),
                controls_passed
            );
            values.push((cell_id, ids, class, reason));
        }
        values
    };
    for (index, (package, (id, subprocesses, class, reason))) in
        packages.iter().zip(&expected_packages).enumerate()
    {
        let label = format!("summary.packages[{index}]");
        let role = protocol_roles
            .get(id)
            .copied()
            .ok_or_else(|| format!("protocol lacks package {id}"))?;
        let actual_subprocesses: Vec<_> = array(package, "subprocesses", &label)?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("{label} has a non-string subprocess"))
            })
            .collect::<Result<_, _>>()?;
        if string(package, "package_id", &label)? != *id
            || string(package, "role", &label)? != role
            || string(package, "classification", &label)? != class
            || string(package, "reason", &label)? != reason
            || actual_subprocesses != *subprocesses
        {
            return Err(format!(
                "{label} disagrees with its retained subprocess evidence"
            ));
        }
    }

    let derived_global = if derived_cells
        .values()
        .any(|class| class == "invalid_instrument")
        || matches!(identity_class, "invalid_record" | "invalid_instrument")
        || sat_package_class == "invalid"
    {
        "invalid_instrument"
    } else if identity_class != "passed" || sat_package_class != "passed" {
        "unmeasured"
    } else if derived_cells.values().any(|class| {
        matches!(
            class.as_str(),
            "not_universally_four_at_cell" | "draw_dependent"
        )
    }) {
        "not_universally_four_in_registered_panel"
    } else if derived_cells.values().any(|class| class == "unmeasured") {
        "unmeasured"
    } else if derived_cells
        .values()
        .all(|class| class == "bounded_four_at_cell")
    {
        "bounded_four_across_registered_cells"
    } else {
        "invalid_instrument"
    };
    if global != derived_global {
        return Err("summary global classification disagrees with retained packages".into());
    }

    let resume = value
        .get("resume_provenance")
        .ok_or_else(|| "summary.resume_provenance is missing".to_string())?;
    let resume_object = object(resume, "summary.resume_provenance")?;
    let resume_keys: BTreeSet<_> = ["state", "records", "reason"].into_iter().collect();
    if resume_object
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != resume_keys
    {
        return Err("summary.resume_provenance has missing or unexpected fields".into());
    }
    let resume_state = string(resume, "state", "summary.resume_provenance")?;
    required_u64(resume, "records", "summary.resume_provenance")?;
    string(resume, "reason", "summary.resume_provenance")?;
    if !matches!(resume_state, "ready" | "invalid")
        || (resume_state == "invalid" && derived_global != "invalid_instrument")
    {
        return Err("resume provenance state disagrees with the no-conclusion result".into());
    }
    if resume_state == "invalid"
        && rows
            .iter()
            .any(|row| row.get("classification").and_then(Value::as_str) != Some("invalid_record"))
    {
        return Err("invalid resume custody did not invalidate every subprocess row".into());
    }
    Ok(())
}

fn markdown_escape(value: &str) -> String {
    value
        .replace('|', "\\|")
        .replace(['\r', '\n'], " ")
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn typst_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\r', '\n'], " ")
}

fn status(global: &str) -> (&'static str, &'static str) {
    match global {
        "invalid_instrument" => (
            "INVALID — NO SCIENTIFIC CONCLUSION",
            "Contradictory or malformed evidence invalidates interpretation. Every mathematical lead remains open.",
        ),
        "unmeasured" => (
            "INCOMPLETE / UNMEASURED — LIVE LEADS REMAIN OPEN",
            "At least one required package is absent or resource-censored. This is not a mathematical disproof.",
        ),
        "not_universally_four_in_registered_panel" => (
            "COMPLETE REGISTERED-PANEL COUNTEREXAMPLE",
            "At least one exact registered cell contains a complete draw above degree four; only the exact bounded-four statement for that cell is refuted.",
        ),
        "bounded_four_across_registered_cells" => (
            "COMPLETE BOUNDED REGISTERED-PANEL RESULT",
            "All registered exact draws closed by degree four, conditional on the registered identity and satisfiable controls.",
        ),
        _ => unreachable!("validated global classification"),
    }
}

fn attempt_count(row: &Value) -> Result<usize, String> {
    Ok(array(row, "attempts", "subprocess")?.len())
}

fn display_classification(classification: &str) -> &'static str {
    match classification {
        "closure_by_four" => "closure_le_4",
        "above_four" => "above_4",
        "passed" => "passed",
        "unmeasured" => "unmeasured",
        _ => "invalid",
    }
}

fn next_action(classification: &str) -> &'static str {
    match classification {
        "bounded_four_at_cell" => {
            "Keep the lead open beyond this finite panel; pursue other presentations, degree five, and larger parameters."
        }
        "not_universally_four_at_cell" | "draw_dependent" => {
            "The exact cell-level universal-degree-four statement is refuted; pursue the above-four mechanism and all other live variants."
        }
        "unmeasured" => {
            "Retry only the resource-censored or incomplete subprocesses; no lead is closed."
        }
        _ => "Repair or independently reproduce invalid evidence; no lead is closed.",
    }
}

fn render_markdown(
    summary: &Value,
    protocol: &Protocol,
    protocol_rel: &str,
    summary_rel: &str,
) -> Result<String, String> {
    let global = string(summary, "global_classification", "summary")?;
    let (headline, scope) = status(global);
    let mut out = String::new();
    out.push_str("# High-arity Boolean closure-degree result\n\n");
    out.push_str(&format!("Status: **{}**.\n\n", markdown_escape(headline)));
    out.push_str(scope);
    out.push_str(" No elapsed-time, asymptotic, cryptographic-scale, or end-to-end ECDLP conclusion is made.\n\n");
    out.push_str(&format!(
        "Machine sources: [frozen protocol]({}), [native analyzer summary]({}), and [unified export](experiment.export.json). The diagram is [closure-outcomes.svg](closure-outcomes.svg).\n\n",
        markdown_escape(protocol_rel),
        markdown_escape(summary_rel)
    ));
    out.push_str("## Registered question\n\n");
    out.push_str(&markdown_escape(&protocol.question));
    out.push_str("\n\n## Frozen instrument identities\n\n");
    out.push_str(&format!(
        "Source commit: `{}`. Ladder: `{}`. Analyzer: `{}`. Publisher: `{}`.\n\n",
        protocol.source_commit,
        protocol.ladder_sha256,
        protocol.analyzer_sha256,
        protocol.report_sha256,
    ));
    out.push_str("| arm | role | closure binary | image |\n|---|---|---|---|\n");
    for arm in &protocol.arms {
        out.push_str(&format!(
            "| {} | {} | `{}` | `{}` |\n",
            markdown_escape(&arm.id),
            markdown_escape(&arm.role),
            arm.binary_sha256,
            markdown_escape(arm.image_digest.as_deref().unwrap_or("native-host")),
        ));
    }
    out.push_str("\n## Deterministic draw-custody replay\n\n| cell | audit state | records | generator commit |\n|---|---|---:|---|\n");
    for panel in array(summary, "draw_panels", "summary")? {
        out.push_str(&format!(
            "| {} | `{}` | {} | `{}` |\n",
            markdown_escape(string(panel, "cell", "draw panel")?),
            markdown_escape(string(panel, "state", "draw panel")?),
            required_u64(panel, "records", "draw panel")?,
            markdown_escape(
                panel
                    .get("execution_commit")
                    .and_then(Value::as_str)
                    .unwrap_or("unavailable")
            ),
        ));
    }
    out.push('\n');
    out.push_str("## Package classifications\n\n| package | role | classification | reason |\n|---|---|---|---|\n");
    for package in array(summary, "packages", "summary")? {
        out.push_str(&format!(
            "| {} | {} | `{}` | {} |\n",
            markdown_escape(string(package, "package_id", "package")?),
            markdown_escape(string(package, "role", "package")?),
            markdown_escape(string(package, "classification", "package")?),
            markdown_escape(string(package, "reason", "package")?),
        ));
    }
    out.push_str("\n## All 19 subprocess classifications\n\n| subprocess | kind | cell | draw | classification | attempts | selected | reason |\n|---|---|---|---:|---|---:|---|---|\n");
    for row in array(summary, "rows", "summary")? {
        let draw = row
            .get("draw")
            .and_then(Value::as_u64)
            .map(|value| value.to_string())
            .unwrap_or_else(|| "—".into());
        let selected = row
            .get("selected_attempt")
            .and_then(Value::as_str)
            .unwrap_or("—");
        out.push_str(&format!(
            "| `{}` | {} | {} | {} | `{}` | {} | {} | {} |\n",
            markdown_escape(string(row, "run_id", "subprocess")?),
            markdown_escape(string(row, "kind", "subprocess")?),
            markdown_escape(string(row, "cell", "subprocess")?),
            draw,
            markdown_escape(string(row, "classification", "subprocess")?),
            attempt_count(row)?,
            markdown_escape(selected),
            markdown_escape(string(row, "reason", "subprocess")?),
        ));
    }
    out.push_str("\n## Retry, resume, and resource history\n\n| subprocess | attempt | exit | termination | state | reason | metadata |\n|---|---:|---:|---|---|---|---|\n");
    for row in array(summary, "rows", "summary")? {
        let run_id = string(row, "run_id", "subprocess")?;
        let attempts = array(row, "attempts", "subprocess")?;
        if attempts.is_empty() {
            out.push_str(&format!(
                "| `{}` | — | — | — | not_started | no closure attempt retained | — |\n",
                markdown_escape(run_id)
            ));
        }
        for attempt in attempts {
            let exit = attempt
                .get("exit_status")
                .and_then(Value::as_i64)
                .map(|value| value.to_string())
                .unwrap_or_else(|| "—".into());
            let termination = attempt
                .get("termination")
                .and_then(Value::as_str)
                .unwrap_or("—");
            let metadata = attempt
                .get("metadata")
                .and_then(Value::as_str)
                .unwrap_or("—");
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} | `{}` |\n",
                markdown_escape(run_id),
                markdown_escape(string(attempt, "attempt", "attempt")?),
                exit,
                markdown_escape(termination),
                markdown_escape(string(attempt, "state", "attempt")?),
                markdown_escape(string(attempt, "reason", "attempt")?),
                markdown_escape(metadata),
            ));
        }
    }
    out.push_str("\n## Per-cell retained counts\n\n| cell | classification | closure ≤ 4 | above 4 | unmeasured | invalid | controls passed |\n|---|---|---:|---:|---:|---:|---:|\n");
    let mut actions = Vec::new();
    for cell in array(summary, "cells", "summary")? {
        let number = |key: &str| cell.get(key).and_then(Value::as_u64).unwrap_or(0);
        out.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} / 6 |\n",
            markdown_escape(string(cell, "cell", "cell")?),
            markdown_escape(string(cell, "classification", "cell")?),
            number("closure_by_four"),
            number("above_four"),
            number("unmeasured"),
            number("invalid"),
            number("controls_passed"),
        ));
        actions.push(format!(
            "- Next action for `{}`: {}",
            markdown_escape(string(cell, "cell", "cell")?),
            next_action(string(cell, "classification", "cell")?)
        ));
    }
    out.push_str("\n### Scoped next actions\n\n");
    out.push_str(&actions.join("\n"));
    out.push('\n');
    out.push_str("\n## Interpretation boundary\n\n");
    out.push_str("The native classifier retains all 19 subprocess histories and all retry attempts. Resource limits, timeouts, missing rows, and incomplete saturation remain `unmeasured`; they never close a lead. A counterexample closes only the universal degree-four statement at its exact registered cell and draw panel. Other presentations, degree five, larger parameters, and end-to-end index calculus remain outside this result.\n");
    Ok(out)
}

fn render_svg(summary: &Value) -> Result<String, String> {
    let rows = array(summary, "rows", "summary")?;
    let global = string(summary, "global_classification", "summary")?;
    let (headline, _) = status(global);
    let height = 190 + rows.len() * 39;
    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="{height}" viewBox="0 0 1200 {height}" role="img" aria-labelledby="title desc">
<title id="title">High-arity closure outcomes</title>
<desc id="desc">One labeled, shape-encoded mark for each of nineteen registered subprocesses. No wall-time axis is shown.</desc>
<rect width="1200" height="{height}" fill="#f8fafc"/>
<style>text{{font-family:Inter,system-ui,sans-serif;fill:#172033}}.h{{font-size:27px;font-weight:700}}.s{{font-size:15px}}.l{{font-size:14px;font-weight:600}}.k{{font-size:13px}}.grid{{stroke:#cbd5e1;stroke-width:1}}</style>
"##,
    );
    svg.push_str(&format!(
        "<text class=\"h\" x=\"50\" y=\"55\">{}</text>\n",
        xml_escape(headline)
    ));
    svg.push_str("<text class=\"s\" x=\"50\" y=\"84\">All twelve UNSAT draws, six SAT controls, and the identity replay remain individually visible.</text>\n");
    svg.push_str("<text class=\"l\" x=\"50\" y=\"125\">subprocess</text><text class=\"l\" x=\"530\" y=\"125\">kind / cell</text><text class=\"l\" x=\"790\" y=\"125\">shape + exact classification</text><line class=\"grid\" x1=\"45\" y1=\"136\" x2=\"1150\" y2=\"136\"/>\n");
    for (index, row) in rows.iter().enumerate() {
        let y = 165 + index * 39;
        let run_id = string(row, "run_id", "subprocess")?;
        let kind = string(row, "kind", "subprocess")?;
        let cell = string(row, "cell", "subprocess")?;
        let class = display_classification(string(row, "classification", "subprocess")?);
        let (color, shape) = match class {
            "closure_le_4" => ("#2563eb", "square"),
            "above_4" => ("#7c3aed", "diamond"),
            "passed" => ("#15803d", "circle"),
            "unmeasured" => ("#d97706", "triangle"),
            _ => ("#b91c1c", "cross"),
        };
        svg.push_str(&format!(
            "<g id=\"row-{}\" data-class=\"{}\"><title>{}: {}</title><text class=\"k\" x=\"50\" y=\"{}\">{}</text><text class=\"k\" x=\"530\" y=\"{}\">{} / {}</text>",
            index + 1,
            xml_escape(class),
            xml_escape(run_id),
            xml_escape(class),
            y + 5,
            xml_escape(run_id),
            y + 5,
            xml_escape(kind),
            xml_escape(cell),
        ));
        match shape {
            "square" => svg.push_str(&format!("<rect x=\"790\" y=\"{}\" width=\"18\" height=\"18\" fill=\"{color}\"/>", y - 10)),
            "diamond" => svg.push_str(&format!("<path d=\"M799 {} L810 {y} L799 {} L788 {y} Z\" fill=\"{color}\"/>", y - 11, y + 11)),
            "circle" => svg.push_str(&format!("<circle cx=\"799\" cy=\"{y}\" r=\"10\" fill=\"{color}\"/>")),
            "triangle" => svg.push_str(&format!("<path d=\"M799 {} L811 {} L787 {} Z\" fill=\"{color}\"/>", y - 11, y + 10, y + 10)),
            _ => svg.push_str(&format!("<path d=\"M790 {} L808 {} M808 {} L790 {}\" stroke=\"{color}\" stroke-width=\"4\"/>", y - 9, y + 9, y - 9, y + 9)),
        }
        svg.push_str(&format!(
            "<text class=\"l\" x=\"825\" y=\"{}\">{} ({shape})</text></g><line class=\"grid\" x1=\"45\" y1=\"{}\" x2=\"1150\" y2=\"{}\"/>\n",
            y + 5,
            xml_escape(class),
            y + 18,
            y + 18,
        ));
    }
    svg.push_str(&format!("<text class=\"s\" x=\"50\" y=\"{}\">Scope: registered toy cells only. No timing is interpreted. Shape and text duplicate color.</text>\n</svg>\n", height - 24));
    Ok(svg)
}

#[allow(dead_code)]
fn render_typst(summary: &Value) -> Result<String, String> {
    let global = string(summary, "global_classification", "summary")?;
    let (headline, scope) = status(global);
    let mut out = String::new();
    out.push_str("#set page(paper: \"a4\", margin: 22mm)\n#set text(font: \"Libertinus Serif\", size: 10pt)\n#set heading(numbering: \"1.\")\n");
    out.push_str("= High-arity Boolean closure-degree result\n\n");
    out.push_str(&format!(
        "#box(fill: rgb(\"f1f5f9\"), inset: 10pt, radius: 4pt)[#strong[#raw(\"{}\")] \\\n#raw(\"{}\")]\n\n",
        typst_string(headline),
        typst_string(scope)
    ));
    out.push_str("#image(\"closure-outcomes.svg\", width: 100%)\n\n");
    out.push_str("== Package classifications\n\n#table(columns: (1.4fr, 1fr, 1.6fr), inset: 5pt, [*Package*], [*Role*], [*Classification*],\n");
    for package in array(summary, "packages", "summary")? {
        out.push_str(&format!(
            "  [#raw(\"{}\")], [#raw(\"{}\")], [#raw(\"{}\")],\n",
            typst_string(string(package, "package_id", "package")?),
            typst_string(string(package, "role", "package")?),
            typst_string(string(package, "classification", "package")?),
        ));
    }
    out.push_str(")\n\n== Scope\n\n");
    out.push_str("This report is derived from the frozen native analyzer summary. It interprets no elapsed time. Resource censoring is unmeasured, not a disproof. Any counterexample applies only to its exact registered cell and finite draw panel.\n");
    Ok(out)
}

fn render_typst_detailed(
    summary: &Value,
    protocol: &Protocol,
    protocol_rel: &str,
    summary_rel: &str,
) -> Result<String, String> {
    let global = string(summary, "global_classification", "summary")?;
    let (headline, scope) = status(global);
    let mut out = String::new();
    out.push_str("#set page(paper: \"a4\", margin: 22mm)\n#set text(font: \"Libertinus Serif\", size: 10pt)\n#set heading(numbering: \"1.\")\n");
    out.push_str("= High-arity Boolean closure-degree result\n\n");
    out.push_str(&format!(
        "#box(fill: rgb(\"f1f5f9\"), inset: 10pt, radius: 4pt)[#strong[#raw(\"{}\")] #linebreak() #raw(\"{}\")]\n\n",
        typst_string(headline),
        typst_string(scope)
    ));
    out.push_str("#image(\"closure-outcomes.svg\", width: 100%)\n\n");
    out.push_str("== Registered question\n\n");
    out.push_str(&format!(
        "#raw(\"{}\")\n\n",
        typst_string(&protocol.question)
    ));
    out.push_str("== Frozen provenance and instruments\n\n");
    out.push_str(&format!(
        "Frozen protocol: #raw(\"{}\"). Analyzer summary: #raw(\"{}\"). Source commit: #raw(\"{}\"). #linebreak()\nLadder: #raw(\"{}\"). Analyzer: #raw(\"{}\"). Publisher: #raw(\"{}\").\n\n",
        typst_string(protocol_rel),
        typst_string(summary_rel),
        protocol.source_commit,
        protocol.ladder_sha256,
        protocol.analyzer_sha256,
        protocol.report_sha256,
    ));
    out.push_str("== Deterministic draw-custody replay\n\n#table(columns: (1.4fr, 1fr, 0.8fr, 2fr), inset: 4pt, [*Cell*], [*State*], [*Records*], [*Generator commit*],\n");
    for panel in array(summary, "draw_panels", "summary")? {
        out.push_str(&format!(
            "  [#raw(\"{}\")], [#raw(\"{}\")], [{}], [#raw(\"{}\")],\n",
            typst_string(string(panel, "cell", "draw panel")?),
            typst_string(string(panel, "state", "draw panel")?),
            required_u64(panel, "records", "draw panel")?,
            typst_string(
                panel
                    .get("execution_commit")
                    .and_then(Value::as_str)
                    .unwrap_or("unavailable")
            ),
        ));
    }
    out.push_str(")\n\n");
    out.push_str("== Package classifications\n\n#table(columns: (1.4fr, 1fr, 1.6fr), inset: 5pt, [*Package*], [*Role*], [*Classification*],\n");
    for package in array(summary, "packages", "summary")? {
        out.push_str(&format!(
            "  [#raw(\"{}\")], [#raw(\"{}\")], [#raw(\"{}\")],\n",
            typst_string(string(package, "package_id", "package")?),
            typst_string(string(package, "role", "package")?),
            typst_string(string(package, "classification", "package")?),
        ));
    }
    out.push_str(")\n\n== All 19 subprocess classifications\n\n#table(columns: (2.2fr, 0.8fr, 1fr, 1.3fr, 0.6fr), inset: 4pt, [*Subprocess*], [*Kind*], [*Cell*], [*Classification*], [*Attempts*],\n");
    for row in array(summary, "rows", "summary")? {
        out.push_str(&format!(
            "  [#raw(\"{}\")], [#raw(\"{}\")], [#raw(\"{}\")], [#raw(\"{}\")], [{}],\n",
            typst_string(string(row, "run_id", "subprocess")?),
            typst_string(string(row, "kind", "subprocess")?),
            typst_string(string(row, "cell", "subprocess")?),
            typst_string(string(row, "classification", "subprocess")?),
            attempt_count(row)?,
        ));
    }
    out.push_str(")\n\n== Retry and resource history\n\n");
    for row in array(summary, "rows", "summary")? {
        let run_id = typst_string(string(row, "run_id", "subprocess")?);
        let attempts = array(row, "attempts", "subprocess")?;
        if attempts.is_empty() {
            out.push_str(&format!(
                "- #raw(\"{run_id}\"): no closure attempt retained.\n"
            ));
        }
        for attempt in attempts {
            let exit = attempt
                .get("exit_status")
                .and_then(Value::as_i64)
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".into());
            let termination = attempt
                .get("termination")
                .and_then(Value::as_str)
                .unwrap_or("none");
            out.push_str(&format!(
                "- #raw(\"{run_id}\") attempt #raw(\"{}\"): exit #raw(\"{}\"), #raw(\"{}\"), #raw(\"{}\").\n",
                typst_string(string(attempt, "attempt", "attempt")?),
                typst_string(&exit),
                typst_string(termination),
                typst_string(string(attempt, "state", "attempt")?),
            ));
        }
    }
    out.push_str("\n== Scope\n\nThis report is derived from the frozen native analyzer summary. It interprets no elapsed time. Resource censoring is unmeasured, not a disproof. Any counterexample applies only to its exact registered cell and finite draw panel.\n");
    Ok(out)
}

fn pdf_ascii(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            '\u{2013}' | '\u{2014}' => '-',
            '\u{2264}' => '<',
            c if c.is_ascii() => c,
            _ => '?',
        })
        .collect()
}

fn pdf_escape(value: &str) -> String {
    pdf_ascii(value)
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

fn wrap_ascii(value: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in pdf_ascii(value).split_whitespace() {
        if !line.is_empty() && line.len() + 1 + word.len() > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn render_pdf(
    summary: &Value,
    protocol: &Protocol,
    summary_sha256: &str,
) -> Result<Vec<u8>, String> {
    let global = string(summary, "global_classification", "summary")?;
    let (headline, scope) = status(global);
    let mut lines = vec![
        (18, "High-arity Boolean closure-degree result".to_string()),
        (12, pdf_ascii(headline)),
    ];
    for line in wrap_ascii(scope, 86) {
        lines.push((10, line));
    }
    lines.push((10, String::new()));
    lines.push((12, "Registered question".into()));
    for line in wrap_ascii(&protocol.question, 86) {
        lines.push((9, line));
    }
    lines.push((10, String::new()));
    lines.push((12, "Deterministic draw-custody replay".into()));
    for panel in array(summary, "draw_panels", "summary")? {
        lines.push((
            9,
            format!(
                "{}: state={}, records={}",
                string(panel, "cell", "draw panel")?,
                string(panel, "state", "draw panel")?,
                required_u64(panel, "records", "draw panel")?,
            ),
        ));
    }
    lines.push((10, String::new()));
    lines.push((12, "Registered package classifications".into()));
    for package in array(summary, "packages", "summary")? {
        let id = string(package, "package_id", "package")?;
        let classification = string(package, "classification", "package")?;
        lines.push((10, format!("{id}: {classification}")));
    }
    lines.push((10, String::new()));
    lines.push((12, "All 19 subprocess classifications".into()));
    for row in array(summary, "rows", "summary")? {
        lines.push((
            8,
            format!(
                "{}: {} (attempts={})",
                string(row, "run_id", "subprocess")?,
                string(row, "classification", "subprocess")?,
                attempt_count(row)?,
            ),
        ));
    }
    lines.push((10, String::new()));
    lines.push((12, "Evidence boundary".into()));
    for line in wrap_ascii(
        "All 19 subprocess histories and retry attempts remain in the native analyzer summary. Resource censoring is unmeasured, not a disproof. Any counterexample applies only to its exact registered cell and finite draw panel. No elapsed time is interpreted.",
        86,
    ) {
        lines.push((10, line));
    }
    lines.push((10, String::new()));
    lines.push((12, "Provenance".into()));
    lines.push((
        8,
        format!("Protocol: {} v{}", protocol.id, protocol.version),
    ));
    lines.push((8, format!("Protocol SHA-256: {}", protocol.sha256)));
    lines.push((8, format!("Analyzer summary SHA-256: {summary_sha256}")));
    lines.push((8, format!("Source commit: {}", protocol.source_commit)));
    lines.push((
        8,
        format!("Ladder binary SHA-256: {}", protocol.ladder_sha256),
    ));
    lines.push((
        8,
        format!("Analyzer binary SHA-256: {}", protocol.analyzer_sha256),
    ));
    lines.push((
        8,
        format!("Publisher binary SHA-256: {}", protocol.report_sha256),
    ));

    let mut stream = String::from("BT\n50 790 Td\n");
    let mut previous_size = 0;
    for (index, (size, line)) in lines.iter().enumerate() {
        if *size != previous_size {
            stream.push_str(&format!("/F1 {size} Tf\n"));
            previous_size = *size;
        }
        if index > 0 {
            let leading = if *size >= 16 {
                28
            } else if *size >= 12 {
                20
            } else {
                15
            };
            stream.push_str(&format!("0 -{leading} Td\n"));
        }
        stream.push_str(&format!("({}) Tj\n", pdf_escape(line)));
    }
    stream.push_str("ET\n");

    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".as_bytes().to_vec(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>"
            .as_bytes()
            .to_vec(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>"
            .as_bytes()
            .to_vec(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>"
            .as_bytes()
            .to_vec(),
        format!("<< /Length {} >>\nstream\n{}endstream", stream.len(), stream)
            .into_bytes(),
    ];
    let mut pdf = b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = vec![0_usize];
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        pdf.extend_from_slice(object);
        pdf.extend_from_slice(b"\nendobj\n");
    }
    let xref = pdf.len();
    pdf.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
    pdf.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    pdf.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    Ok(pdf)
}

fn absolute_existing(path: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(path).map_err(|e| format!("cannot resolve {}: {e}", path.display()))
}

fn absolute_future_dir(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return fs::canonicalize(path)
            .map_err(|e| format!("cannot resolve {}: {e}", path.display()));
    }
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()
            .map_err(|e| format!("cannot resolve current directory: {e}"))?
            .join(path)
    };
    let mut normalized = PathBuf::new();
    for component in joined.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() {
                    return Err(format!(
                        "output path escapes its filesystem root: {}",
                        path.display()
                    ));
                }
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    Ok(normalized)
}

fn relative_path(from_dir: &Path, target: &Path) -> Result<String, String> {
    let from: Vec<_> = from_dir.components().collect();
    let to: Vec<_> = target.components().collect();
    if matches!(from.first(), Some(Component::Prefix(_)))
        || matches!(to.first(), Some(Component::Prefix(_)))
        || from.first() != to.first()
    {
        return Err("cannot construct a relative path across filesystem roots".into());
    }
    let mut common = 0;
    while common < from.len() && common < to.len() && from[common] == to[common] {
        common += 1;
    }
    let mut result = PathBuf::new();
    for _ in common..from.len() {
        result.push("..");
    }
    for component in &to[common..] {
        result.push(component.as_os_str());
    }
    Ok(result.to_string_lossy().replace('\\', "/"))
}

fn file_entry(path: String, bytes: &[u8], kind: &str) -> Value {
    json!({"path": path, "sha256": digest(bytes), "bytes": bytes.len(), "kind": kind})
}

fn disk_file_entry(base: &Path, path: &Path, kind: &str) -> Result<Value, String> {
    let (sha256, bytes) = digest_file(path)?;
    Ok(json!({
        "path": relative_path(base, &absolute_existing(path)?)?,
        "sha256": sha256,
        "bytes": bytes,
        "kind": kind
    }))
}

fn evidence_kind(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("log") => "log",
        Some("jsonl") => "raw",
        Some("json") if path.file_name().and_then(|s| s.to_str()) == Some("protocol.json") => {
            "protocol"
        }
        Some("json") if path.file_name().and_then(|s| s.to_str()) == Some("summary.json") => {
            "summary"
        }
        Some("json") => "receipt",
        _ => "raw",
    }
}

fn visit_evidence(
    path: &Path,
    excluded: &[PathBuf],
    seen: &mut BTreeSet<PathBuf>,
) -> Result<(), String> {
    if excluded
        .iter()
        .any(|root| path == root || path.starts_with(root))
    {
        return Ok(());
    }
    let metadata = fs::symlink_metadata(path)
        .map_err(|e| format!("cannot inspect evidence path {}: {e}", path.display()))?;
    if metadata.file_type().is_symlink() {
        return Err(format!(
            "evidence trees may not contain symlinks: {}",
            path.display()
        ));
    }
    if metadata.is_file() {
        seen.insert(absolute_existing(path)?);
        return Ok(());
    }
    if !metadata.is_dir() {
        return Err(format!(
            "evidence tree contains a special file: {}",
            path.display()
        ));
    }
    let mut children = fs::read_dir(path)
        .map_err(|e| format!("cannot enumerate {}: {e}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("cannot enumerate {}: {e}", path.display()))?;
    children.sort_by_key(|entry| entry.file_name());
    for child in children {
        visit_evidence(&child.path(), excluded, seen)?;
    }
    Ok(())
}

fn collect_evidence_entries(
    summary: &Value,
    summary_path: &Path,
    protocol_path: &Path,
    analysis_path: &Path,
    out_abs: &Path,
) -> Result<Vec<Value>, String> {
    let mut seen = BTreeSet::new();
    let reports_root = out_abs
        .parent()
        .ok_or_else(|| "publication directory has no reports parent".to_string())?;
    if reports_root.file_name().and_then(|name| name.to_str()) != Some("reports")
        || out_abs
            .file_name()
            .and_then(|name| name.to_str())
            .is_none_or(|name| !name.starts_with("run-"))
    {
        return Err("publication directory must be reports/run-*".into());
    }
    // Reports and later review records are derived judgments, not raw
    // scientific evidence.  Excluding the whole registered reports namespace
    // keeps an export byte-stable when a sibling ../reviews record is filed;
    // this bundle's five generated files are added explicitly below.
    let excluded = [reports_root.to_path_buf()];
    for root_key in ["draw_dir", "output_root"] {
        let root = PathBuf::from(string(summary, root_key, "summary")?);
        let root = absolute_existing(&root)?;
        visit_evidence(&root, &excluded, &mut seen)?;
    }
    for required in [summary_path, protocol_path, analysis_path] {
        seen.insert(absolute_existing(required)?);
    }
    if let Some(path) = summary.get("historical_identity").and_then(Value::as_str) {
        let path = Path::new(path);
        if path.exists() {
            seen.insert(absolute_existing(path)?);
        }
    }
    for row in array(summary, "rows", "summary")? {
        for attempt in array(row, "attempts", "subprocess")? {
            let Some(metadata) = attempt.get("metadata").and_then(Value::as_str) else {
                continue;
            };
            let metadata = Path::new(metadata);
            if !metadata.exists() {
                continue;
            }
            let (_, value) = read_json(metadata)?;
            if let Some(input) = value
                .get("input")
                .and_then(|value| value.get("path"))
                .and_then(Value::as_str)
            {
                let input = Path::new(input);
                if input.exists() {
                    seen.insert(absolute_existing(input)?);
                }
            }
        }
    }
    for name in [
        "sources.sha256",
        "binary.sha256",
        "REPORT_EXPORT_CONTRACT.md",
    ] {
        let path = protocol_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(name);
        seen.insert(absolute_existing(&path).map_err(|error| {
            format!("required frozen publication input {name} is unavailable: {error}")
        })?);
    }
    if let Some(path) = summary
        .get("frozen_provenance")
        .and_then(|v| v.get("binary_manifest"))
        .and_then(Value::as_str)
    {
        seen.insert(absolute_existing(Path::new(path))?);
    }
    let mut entries = Vec::with_capacity(seen.len());
    for path in seen {
        entries.push(disk_file_entry(out_abs, &path, evidence_kind(&path))?);
    }
    entries.sort_by(|left, right| {
        left.get("path")
            .and_then(Value::as_str)
            .cmp(&right.get("path").and_then(Value::as_str))
    });
    Ok(entries)
}

fn package_artifacts(
    package_id: &str,
    subprocesses: &[String],
    evidence_entries: &[Value],
) -> Result<Vec<Value>, String> {
    let cells: &[&str] = if package_id == "satisfiable-controls" {
        &["m5-n11-l2", "m5-n13-l3", "m6-n11-l2"]
    } else {
        &[]
    };
    let mut artifacts = Vec::new();
    for entry in evidence_entries {
        let path = string(entry, "path", "evidence file")?;
        let provenance = path.ends_with("protocol.json")
            || path.ends_with("sources.sha256")
            || path.ends_with("binary.sha256")
            || path.ends_with("REPORT_EXPORT_CONTRACT.md")
            || path.ends_with("summary.json")
            || path.contains("resume-import-");
        let subprocess = subprocesses.iter().any(|run_id| path.contains(run_id));
        let cell_input = if package_id == "identity-replay" {
            path.contains("m4-12-5") || path.contains("fresh-m4-12-5")
        } else if package_id == "satisfiable-controls" {
            cells.iter().any(|cell| path.contains(cell))
        } else {
            path.contains(package_id)
        };
        if provenance || subprocess || cell_input {
            artifacts.push(entry.clone());
        }
    }
    if artifacts.is_empty() {
        return Err(format!(
            "package {package_id} has no linked evidence artifacts"
        ));
    }
    Ok(artifacts)
}

fn package_run(
    package: &Value,
    arm: &Arm,
    summary: &Value,
    evidence_entries: &[Value],
) -> Result<Value, String> {
    let classification = string(package, "classification", "package")?;
    let valid = matches!(
        classification,
        "passed" | "bounded_four_at_cell" | "not_universally_four_at_cell" | "draw_dependent"
    );
    let invalid = matches!(
        classification,
        "invalid" | "invalid_record" | "invalid_instrument"
    );
    if !valid && !invalid && classification != "unmeasured" {
        return Err(format!(
            "package {} has unknown classification {classification:?}",
            arm.id
        ));
    }
    let status = if valid {
        "succeeded"
    } else if invalid {
        "failed"
    } else {
        "interrupted"
    };
    let verification = if valid {
        "verified"
    } else if invalid {
        "error"
    } else {
        "not_run"
    };
    let row_map: BTreeMap<_, _> = array(summary, "rows", "summary")?
        .iter()
        .map(|row| Ok((string(row, "run_id", "subprocess")?, row)))
        .collect::<Result<_, String>>()?;
    let subprocesses: Vec<_> = array(package, "subprocesses", "package")?
        .iter()
        .map(|run_id| {
            run_id
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| "package subprocess is not a string".to_string())
        })
        .collect::<Result<_, _>>()?;
    let mut closure_attempts = 0_usize;
    let mut resumed = 0_usize;
    let mut resource_attempts = 0_usize;
    let mut invalid_subprocesses = 0_usize;
    for run_id in &subprocesses {
        let row = row_map
            .get(run_id.as_str())
            .ok_or_else(|| format!("package {} references absent row {run_id}", arm.id))?;
        let attempts = array(row, "attempts", "subprocess")?;
        closure_attempts += attempts.len();
        resumed += attempts
            .iter()
            .filter(|attempt| {
                attempt.get("checkpoint_restored").and_then(Value::as_bool) == Some(true)
            })
            .count();
        resource_attempts += attempts
            .iter()
            .filter(|attempt| {
                matches!(
                    attempt.get("termination").and_then(Value::as_str),
                    Some(
                        "timeout"
                            | "signal_kill"
                            | "signal_term"
                            | "address_space"
                            | "checkpoint_invalid"
                    )
                )
            })
            .count();
        if display_classification(string(row, "classification", "subprocess")?) == "invalid" {
            invalid_subprocesses += 1;
        }
    }
    let artifacts = package_artifacts(&arm.id, &subprocesses, evidence_entries)?;
    // The export schema requires at least one attempt per composite row.  The
    // native package classification itself is that one composite attempt when
    // an invalid prerequisite prevented every constituent closure launch;
    // closure_attempts retains the exact, possibly-zero subprocess count.
    let composite_attempts = closure_attempts.max(1);
    Ok(json!({
        "run_id": format!("package-{}", arm.id),
        "arm": arm.id,
        "instance_id": "0",
        "seed": 20261101,
        "replicate": 0,
        "warmup": false,
        "status": status,
        "outcome_class": if valid { "completed" } else { "not_completed" },
        "attempts": composite_attempts,
        "resumed": resumed,
        "checkpoint": null,
        "measurements": {
            "counted": {"closure_class": classification},
            "wall_seconds": null,
            "wall_valid": false,
            "cpu_user_seconds": null,
            "cpu_sys_seconds": null,
            "max_rss_bytes": null,
            "counters": {
                "closure_attempts": closure_attempts,
                "subprocesses": subprocesses.len(),
                "resource_interrupted_attempts": resource_attempts,
                "invalid_subprocesses": invalid_subprocesses
            }
        },
        "isolation": {
            "tier": null,
            "contended": null,
            "blockers": ["No elapsed time is interpreted by this exact-computation study"]
        },
        "verification": {
            "status": verification,
            "verifier": "dreg_high_arity_analyze: exact registered-package classifier"
        },
        "exclusion": {"excluded": false, "rule": null},
        "artifacts": artifacts,
        "started_at": null,
        "finished_at": null,
        "worker": null,
        "backend_ref": null
    }))
}

fn execution_history(summary: &Value) -> Result<Vec<Value>, String> {
    let mut history = BTreeSet::new();
    let mut add = |execution_ref: Option<&str>, execution_commit: Option<&str>, what: &str| match (
        execution_ref,
        execution_commit,
    ) {
        (None, None) => Ok(()),
        (Some(reference), Some(commit))
            if reference.starts_with("refs/heads/") && is_hex(commit, 40) =>
        {
            history.insert((reference.to_string(), commit.to_string()));
            Ok(())
        }
        _ => Err(format!(
            "{what} has incomplete or invalid execution provenance"
        )),
    };
    for panel in array(summary, "draw_panels", "summary")? {
        add(
            panel.get("execution_ref").and_then(Value::as_str),
            panel.get("execution_commit").and_then(Value::as_str),
            "draw panel",
        )?;
    }
    for row in array(summary, "rows", "summary")? {
        for attempt in array(row, "attempts", "subprocess")? {
            add(
                attempt.get("execution_ref").and_then(Value::as_str),
                attempt.get("execution_commit").and_then(Value::as_str),
                "closure attempt",
            )?;
        }
    }
    Ok(history
        .into_iter()
        .map(|(reference, commit)| json!({"ref": reference, "commit": commit}))
        .collect())
}

fn compute_flow(runs: &[Value]) -> Value {
    let completed = runs
        .iter()
        .filter(|r| r.get("outcome_class").and_then(Value::as_str) == Some("completed"))
        .count();
    let interrupted = runs
        .iter()
        .filter(|r| r.get("status").and_then(Value::as_str) == Some("interrupted"))
        .count();
    let resumed = runs
        .iter()
        .filter_map(|run| run.get("resumed").and_then(Value::as_u64))
        .sum::<u64>();
    json!({
        "planned": runs.len(),
        "attempted": runs.len(),
        "completed": completed,
        "not_completed": runs.len() - completed,
        "interrupted": interrupted,
        "resumed": resumed,
        "excluded": []
    })
}

#[allow(clippy::too_many_arguments)]
fn render_export(
    summary: &Value,
    summary_bytes: &[u8],
    summary_path: &Path,
    protocol_path: &Path,
    protocol: &Protocol,
    out_abs: &Path,
    markdown: &[u8],
    svg: &[u8],
    typst: &[u8],
    pdf: &[u8],
    mut evidence_entries: Vec<Value>,
) -> Result<Vec<u8>, String> {
    let summary_rel = relative_path(out_abs, &absolute_existing(summary_path)?)?;
    let protocol_rel = relative_path(out_abs, &absolute_existing(protocol_path)?)?;
    let analysis_abs = absolute_existing(&protocol.analysis_path)?;
    let analysis_rel = relative_path(out_abs, &analysis_abs)?;
    let (_, analysis_bytes) = digest_file(&analysis_abs)?;

    let packages = array(summary, "packages", "summary")?;
    let package_map: BTreeMap<_, _> = packages
        .iter()
        .map(|package| {
            Ok((
                string(package, "package_id", "package")?.to_string(),
                package,
            ))
        })
        .collect::<Result<_, String>>()?;
    let runs: Vec<_> = protocol
        .arms
        .iter()
        .map(|arm| {
            package_run(
                package_map
                    .get(&arm.id)
                    .ok_or_else(|| format!("missing package {}", arm.id))?,
                arm,
                summary,
                &evidence_entries,
            )
        })
        .collect::<Result<_, _>>()?;
    let arms: Vec<_> = protocol
        .arms
        .iter()
        .map(|arm| {
            let mut value = json!({
                "id": arm.id,
                "role": arm.role,
                "binary_sha256": arm.binary_sha256,
                "commit": arm.commit
            });
            if let Some(image) = &arm.image_digest {
                value["image_digest"] = Value::String(image.clone());
            }
            value
        })
        .collect();
    let summary_entry = file_entry(summary_rel.clone(), summary_bytes, "analysis");
    let report_entry = file_entry("REPORT.md".into(), markdown, "report");
    let svg_entry = file_entry("closure-outcomes.svg".into(), svg, "report");
    let typst_entry = file_entry("report.typ".into(), typst, "report");
    let pdf_entry = file_entry("report.pdf".into(), pdf, "report");
    let analysis_entry = json!({
        "path": analysis_rel,
        "sha256": protocol.analysis_sha256,
        "bytes": analysis_bytes,
        "kind": "analysis"
    });
    let summary_sha = digest(summary_bytes);
    evidence_entries.extend([
        report_entry.clone(),
        svg_entry.clone(),
        typst_entry.clone(),
        pdf_entry.clone(),
    ]);
    let export = json!({
        "schema": "experiment.export/v1",
        "export_id": format!("DREG-HIGH-ARITY-{}", &summary_sha[..16]),
        "created": protocol.created,
        "protocol": {
            "path": protocol_rel,
            "sha256": protocol.sha256,
            "protocol_id": protocol.id,
            "version": protocol.version
        },
        "producer": {
            "runtime": "native-rust",
            "model": null,
            "session": null,
            "role": "executor"
        },
        "source": {
            "repo": protocol.repository,
            "commit": protocol.source_commit,
            "dirty": false
        },
        "backend": protocol.backend,
        "host": {
            "path": null,
            "sha256": null,
            "class_id": null,
            "summary": {
                "cpu_model": null,
                "logical_cpus": null,
                "memory_bytes": null,
                "kernel": null,
                "hypervisor": null,
                "isolation_tier_max": null
            }
        },
        "arms": arms,
        "runs": runs,
        "flow": compute_flow(&runs),
        "files": evidence_entries,
        "analysis": {
            "script": analysis_entry,
            "blinded": false,
            "blinding_key_sha256": null,
            "outputs": [summary_entry, report_entry, svg_entry, typst_entry, pdf_entry],
            "unmasked_at": null
        },
        "notes": [{
            "kind": "observation",
            "text": format!("Native analyzer global_classification={}", string(summary, "global_classification", "summary")?),
            "by": "dreg_high_arity_analyze"
        }],
        "reviews_dir": "../reviews",
        "backend_record": {
            "summary_schema": "dreg_high_arity.summary/v1",
            "summary_sha256": summary_sha,
            "global_classification": summary.get("global_classification"),
            "frozen_provenance": summary.get("frozen_provenance"),
            "resume_provenance": summary.get("resume_provenance"),
            "execution_history": execution_history(summary)?,
            "draw_panels": summary.get("draw_panels"),
            "rows": summary.get("rows"),
            "cells": summary.get("cells"),
            "packages": summary.get("packages"),
            "timing_interpreted": false
        }
    });
    let mut bytes = serde_json::to_vec_pretty(&export).map_err(|e| e.to_string())?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn build(args: &Args) -> Result<Bundle, String> {
    let (summary_bytes, summary) = read_json(&args.summary)?;
    let (_, protocol_value) = read_json(&args.protocol)?;
    let protocol = parse_protocol(&protocol_value, &args.protocol)?;
    validate_summary(&summary, &protocol)?;
    let out_abs = absolute_future_dir(&args.out_dir)?;
    let summary_rel = relative_path(&out_abs, &absolute_existing(&args.summary)?)?;
    let protocol_rel = relative_path(&out_abs, &absolute_existing(&args.protocol)?)?;
    let markdown = render_markdown(&summary, &protocol, &protocol_rel, &summary_rel)?.into_bytes();
    let svg = render_svg(&summary)?.into_bytes();
    let typst =
        render_typst_detailed(&summary, &protocol, &protocol_rel, &summary_rel)?.into_bytes();
    let summary_sha256 = digest(&summary_bytes);
    let pdf = render_pdf(&summary, &protocol, &summary_sha256)?;
    let evidence_entries = collect_evidence_entries(
        &summary,
        &args.summary,
        &args.protocol,
        &protocol.analysis_path,
        &out_abs,
    )?;
    let export = render_export(
        &summary,
        &summary_bytes,
        &args.summary,
        &args.protocol,
        &protocol,
        &out_abs,
        &markdown,
        &svg,
        &typst,
        &pdf,
        evidence_entries,
    )?;
    Ok(Bundle {
        files: BTreeMap::from([
            ("REPORT.md", markdown),
            ("closure-outcomes.svg", svg),
            ("report.typ", typst),
            ("report.pdf", pdf),
            ("experiment.export.json", export),
        ]),
    })
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o644);
    }
    let mut file = options
        .open(path)
        .map_err(|e| format!("cannot create {}: {e}", path.display()))?;
    file.write_all(bytes)
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("cannot install {}: {e}", path.display()))
}

fn install_atomic(out_dir: &Path, bundle: &Bundle) -> Result<(), String> {
    if out_dir.exists() {
        return Err(format!(
            "refusing to overwrite existing output directory {}",
            out_dir.display()
        ));
    }
    let parent = out_dir.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|e| format!("cannot create output parent {}: {e}", parent.display()))?;
    let leaf = out_dir
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "output directory has a non-UTF-8 or absent final name".to_string())?;
    let suffix = bundle
        .files
        .get("experiment.export.json")
        .map(|bytes| digest(bytes)[..12].to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let temp = parent.join(format!(".{leaf}.tmp-{}-{suffix}", std::process::id()));
    fs::create_dir(&temp)
        .map_err(|e| format!("cannot create staging directory {}: {e}", temp.display()))?;
    let result = (|| {
        for name in OUTPUT_NAMES {
            let bytes = bundle
                .files
                .get(name)
                .ok_or_else(|| format!("internal error: missing generated {name}"))?;
            write_new(&temp.join(name), bytes)?;
        }
        File::open(&temp)
            .and_then(|file| file.sync_all())
            .map_err(|e| format!("cannot sync staging directory: {e}"))?;
        fs::rename(&temp, out_dir).map_err(|e| {
            format!(
                "cannot atomically rename {} to {}: {e}",
                temp.display(),
                out_dir.display()
            )
        })?;
        File::open(parent)
            .and_then(|file| file.sync_all())
            .map_err(|e| format!("cannot sync output parent: {e}"))?;
        Ok(())
    })();
    if result.is_err() && temp.exists() {
        let _ = fs::remove_dir_all(&temp);
    }
    result
}

fn check_exact(out_dir: &Path, bundle: &Bundle) -> Result<(), String> {
    let metadata = fs::symlink_metadata(out_dir)
        .map_err(|e| format!("cannot inspect {}: {e}", out_dir.display()))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(format!(
            "output is not an ordinary directory: {}",
            out_dir.display()
        ));
    }
    let mut actual_names = BTreeSet::new();
    for entry in
        fs::read_dir(out_dir).map_err(|e| format!("cannot enumerate {}: {e}", out_dir.display()))?
    {
        let entry = entry.map_err(|e| format!("cannot enumerate {}: {e}", out_dir.display()))?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|e| format!("cannot inspect {}: {e}", entry.path().display()))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "publication contains a non-ordinary entry: {}",
                entry.path().display()
            ));
        }
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "publication contains a non-UTF-8 filename".to_string())?;
        actual_names.insert(name);
    }
    let expected_names: BTreeSet<_> = OUTPUT_NAMES
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    if actual_names != expected_names {
        return Err(format!(
            "publication file set differs: expected {expected_names:?}, got {actual_names:?}"
        ));
    }
    let mut mismatches = Vec::new();
    for name in OUTPUT_NAMES {
        let path = out_dir.join(name);
        let actual = fs::read(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        if bundle.files.get(name) != Some(&actual) {
            mismatches.push(name);
        }
    }
    if mismatches.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "generated outputs are stale or modified: {}",
            mismatches.join(", ")
        ))
    }
}

fn main() {
    let result = parse_args().and_then(|args| {
        let bundle = build(&args)?;
        if args.check {
            check_exact(&args.out_dir, &bundle)?;
            println!("report bundle is byte-exact: {}", args.out_dir.display());
        } else {
            install_atomic(&args.out_dir, &bundle)?;
            println!("report bundle: {}", args.out_dir.display());
        }
        Ok(())
    });
    if let Err(error) = result {
        eprintln!("dreg_high_arity_report: {error}");
        std::process::exit(65);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn validation_protocol() -> (Protocol, PathBuf) {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "dreg_high_arity_report_validation_{}_{}",
            std::process::id(),
            nonce
        ));
        fs::create_dir(&dir).expect("create validation directory");
        let protocol_path = dir.join("protocol.json");
        let binary_manifest_path = dir.join("binary.sha256");
        fs::write(&protocol_path, b"{}\n").expect("write protocol marker");
        fs::write(&binary_manifest_path, b"manifest\n").expect("write binary manifest");
        let manifest_sha = digest_file(&binary_manifest_path).unwrap().0;
        let hash = "1".repeat(64);
        let commit = "2".repeat(40);
        let arms = [
            ("m5-n11-l2", "candidate"),
            ("m5-n13-l3", "candidate"),
            ("m6-n11-l2", "candidate"),
            ("satisfiable-controls", "control_positive"),
            ("identity-replay", "reference"),
        ]
        .into_iter()
        .map(|(id, role)| Arm {
            id: id.into(),
            role: role.into(),
            binary_sha256: hash.clone(),
            commit: commit.clone(),
            image_digest: Some(format!("sha256:{hash}")),
        })
        .collect();
        (
            Protocol {
                id: "DREG-HIGH-ARITY-TEST".into(),
                version: 1,
                created: "2026-11-01".into(),
                repository: "crypto".into(),
                question: "Does the registered panel close by degree four?".into(),
                sha256: hash.clone(),
                source_commit: commit,
                backend: "ephemeral".into(),
                arms,
                ladder_sha256: hash.clone(),
                analyzer_sha256: hash.clone(),
                report_sha256: hash.clone(),
                protocol_path: fs::canonicalize(&protocol_path).unwrap(),
                binary_manifest_path: fs::canonicalize(&binary_manifest_path).unwrap(),
                binary_manifest_sha256: manifest_sha,
                analysis_path: protocol_path,
                analysis_sha256: hash,
            },
            dir,
        )
    }

    fn consistent_summary(protocol: &Protocol) -> Value {
        let completed_attempt = || {
            json!({
                "attempt":"0001","exit_status":0,"termination":"success",
                "execution_ref":"refs/heads/test","execution_commit":"4".repeat(40),
                "checkpoint_restored":false,"checkpoint_generation":null,
                "state":"candidate_complete","reason":"fixture completion",
                "output":"/runs/out.jsonl","status":"/runs/status",
                "finished":"/runs/finished.json","metadata":"/runs/meta.json"
            })
        };
        let mut rows = vec![json!({
            "run_id":"identity-m4-n12-l5-d24","kind":"identity","cell":"m4-n12-l5",
            "draw":null,"classification":"passed","reason":"identity ok",
            "selected_attempt":"0001","attempts":[completed_attempt()]
        })];
        let mut control_ids = Vec::new();
        let mut packages = Vec::new();
        let mut panels = Vec::new();
        let mut cells = Vec::new();
        let mut candidate_ids = BTreeMap::new();
        for cell in REGISTERED_CELLS {
            panels.push(json!({
                "cell":cell,"state":"ready","path":format!("/draws/{cell}.jsonl"),
                "records":6,"unsat_draws":[0,1,2,3],"satisfiable_draws":[4,5],
                "execution_ref":"refs/heads/test","execution_commit":"3".repeat(40),
                "errors":[]
            }));
            for draw in 4..=5 {
                let run_id = format!("control-{cell}-d{draw}");
                control_ids.push(run_id.clone());
                rows.push(json!({
                    "run_id":run_id,"kind":"satisfiable_control","cell":cell,"draw":draw,
                    "classification":"passed","reason":"control ok",
                    "selected_attempt":"0001","attempts":[completed_attempt()]
                }));
            }
            let mut ids = Vec::new();
            for draw in 0..4 {
                let run_id = format!("unsat-{cell}-d{draw}");
                ids.push(run_id.clone());
                rows.push(json!({
                    "run_id":run_id,"kind":"unsat","cell":cell,"draw":draw,
                    "classification":"closure_by_four","reason":"closed",
                    "selected_attempt":"0001","attempts":[completed_attempt()]
                }));
            }
            candidate_ids.insert(cell.to_string(), ids);
            cells.push(json!({
                "cell":cell,"classification":"bounded_four_at_cell",
                "closure_by_four":4,"above_four":0,"unmeasured":0,"invalid":0,
                "controls_passed":6
            }));
        }
        packages.push(json!({
            "package_id":"identity-replay","role":"reference","classification":"passed",
            "reason":"identity ok","subprocesses":["identity-m4-n12-l5-d24"]
        }));
        packages.push(json!({
            "package_id":"satisfiable-controls","role":"control_positive",
            "classification":"passed",
            "reason":"all six complete saturated controls remained unrefuted",
            "subprocesses":control_ids
        }));
        for cell in REGISTERED_CELLS {
            packages.push(json!({
                "package_id":cell,"role":"candidate","classification":"bounded_four_at_cell",
                "reason":"4 closure-by-four, 0 above-four, 0 unmeasured, 0 invalid; 6/6 shared controls passed",
                "subprocesses":candidate_ids.remove(cell).unwrap()
            }));
        }
        json!({
            "schema":"dreg_high_arity.summary/v1",
            "timing_interpreted":false,
            "frozen_provenance":{
                "state":"ready",
                "protocol":protocol.protocol_path,
                "binary_manifest":protocol.binary_manifest_path,
                "protocol_sha256":protocol.sha256,
                "source_commit":protocol.source_commit,
                "binary_manifest_sha256":protocol.binary_manifest_sha256,
                "reason":"ready"
            },
            "resume_provenance":{"state":"ready","records":0,"reason":"none"},
            "global_classification":"bounded_four_across_registered_cells",
            "draw_panels":panels,"rows":rows,"cells":cells,"packages":packages
        })
    }

    fn invalidate_for_resume(summary: &mut Value) {
        for row in summary["rows"].as_array_mut().unwrap() {
            row["classification"] = json!("invalid_record");
            row["reason"] = json!("resume invalid");
            row["selected_attempt"] = Value::Null;
        }
        for cell in summary["cells"].as_array_mut().unwrap() {
            cell["classification"] = json!("invalid_instrument");
            cell["closure_by_four"] = json!(0);
            cell["above_four"] = json!(0);
            cell["unmeasured"] = json!(0);
            cell["invalid"] = json!(4);
            cell["controls_passed"] = json!(0);
        }
        let packages = summary["packages"].as_array_mut().unwrap();
        packages[0]["classification"] = json!("invalid_record");
        packages[0]["reason"] = json!("resume invalid");
        packages[1]["classification"] = json!("invalid");
        packages[1]["reason"] = json!("at least one satisfiable control is invalid or refuted");
        for package in &mut packages[2..] {
            package["classification"] = json!("invalid_instrument");
            package["reason"] = json!(
                "0 closure-by-four, 0 above-four, 0 unmeasured, 4 invalid; 0/6 shared controls passed"
            );
        }
        summary["global_classification"] = json!("invalid_instrument");
        summary["resume_provenance"] =
            json!({"state":"invalid","records":0,"reason":"tampered import"});
    }

    fn summary(global: &str, cell_class: &str) -> Value {
        let cells: Vec<_> = ["m5-n11-l2", "m5-n13-l3", "m6-n11-l2"]
            .into_iter()
            .map(|cell| {
                json!({
                    "cell": cell,
                    "classification": cell_class,
                    "closure_by_four": if cell_class == "bounded_four_at_cell" { 4 } else { 0 },
                    "above_four": if cell_class == "not_universally_four_at_cell" { 1 } else { 0 },
                    "unmeasured": if cell_class == "unmeasured" { 4 } else if cell_class == "not_universally_four_at_cell" { 3 } else { 0 },
                    "invalid": if cell_class == "invalid_instrument" { 4 } else { 0 },
                    "controls_passed": 6
                })
            })
            .collect();
        let mut rows = vec![json!({
            "run_id":"identity-m4-n12-l5-d24","kind":"identity","cell":"m4-n12-l5",
            "draw":null,"classification":"unmeasured","reason":"fixture","selected_attempt":null,
            "attempts":[]
        })];
        for cell in ["m5-n11-l2", "m5-n13-l3", "m6-n11-l2"] {
            for draw in 0..2 {
                rows.push(json!({
                    "run_id":format!("control-{cell}-d{draw}"),"kind":"control","cell":cell,
                    "draw":draw,"classification":"unmeasured","reason":"fixture",
                    "selected_attempt":null,"attempts":[]
                }));
            }
            for draw in 0..4 {
                rows.push(json!({
                    "run_id":format!("unsat-{cell}-d{draw}"),"kind":"unsat","cell":cell,
                    "draw":draw,"classification":"unmeasured","reason":"fixture",
                    "selected_attempt":null,"attempts":[]
                }));
            }
        }
        json!({"global_classification": global, "cells": cells, "rows": rows})
    }

    #[test]
    fn visual_keeps_unmeasured_rows_visible() {
        let svg = render_svg(&summary("unmeasured", "unmeasured")).unwrap();
        assert!(svg.contains("INCOMPLETE / UNMEASURED"));
        assert!(svg.contains("#d97706"));
        assert_eq!(svg.matches("<g id=\"row-").count(), 19);
        assert_eq!(svg.matches("(triangle)</text>").count(), 19);
    }

    #[test]
    fn invalid_banner_refuses_a_conclusion() {
        let (headline, scope) = status("invalid_instrument");
        assert!(headline.contains("NO SCIENTIFIC CONCLUSION"));
        assert!(scope.contains("Every mathematical lead remains open"));
    }

    #[test]
    fn package_mapping_never_upgrades_unmeasured() {
        let arm = Arm {
            id: "m5-n11-l2".into(),
            role: "candidate".into(),
            binary_sha256: "0".repeat(64),
            commit: "0".repeat(40),
            image_digest: None,
        };
        let run_id = "unsat-m5-n11-l2-d0";
        let summary = json!({"rows":[{
            "run_id":run_id,"classification":"unmeasured","attempts":[
                {"termination":"timeout","checkpoint_restored":false},
                {"termination":"signal_kill","checkpoint_restored":false},
                {"termination":"signal_term","checkpoint_restored":false},
                {"termination":"address_space","checkpoint_restored":false},
                {"termination":"checkpoint_invalid","checkpoint_restored":false}
            ]
        }]});
        let package = json!({
            "package_id":arm.id,"classification":"unmeasured","subprocesses":[run_id]
        });
        let evidence = vec![json!({
            "path":"../../summary.json","sha256":"0".repeat(64),"bytes":1,"kind":"summary"
        })];
        let row = package_run(&package, &arm, &summary, &evidence).unwrap();
        assert_eq!(row["status"], "interrupted");
        assert_eq!(row["outcome_class"], "not_completed");
        assert_eq!(row["verification"]["status"], "not_run");
        assert_eq!(
            row["measurements"]["counters"]["resource_interrupted_attempts"],
            5
        );
    }

    #[test]
    fn consistency_validator_rejects_tampered_scientific_labels() {
        let (protocol, directory) = validation_protocol();
        let valid = consistent_summary(&protocol);
        validate_summary(&valid, &protocol).expect("consistent summary");

        let mut row_tamper = valid.clone();
        row_tamper["rows"][3]["classification"] = json!("above_four");
        assert!(validate_summary(&row_tamper, &protocol).is_err());

        let mut package_tamper = valid.clone();
        package_tamper["packages"][2]["classification"] = json!("above_four");
        assert!(validate_summary(&package_tamper, &protocol).is_err());

        let mut global_tamper = valid;
        global_tamper["global_classification"] = json!("not_universally_four_in_registered_panel");
        assert!(validate_summary(&global_tamper, &protocol).is_err());
        fs::remove_dir_all(directory).expect("remove validation directory");
    }

    #[test]
    fn consistency_validator_binds_claims_to_execution_provenance() {
        let (protocol, directory) = validation_protocol();
        let valid = consistent_summary(&protocol);
        validate_summary(&valid, &protocol).expect("consistent provenance-bound summary");

        let mut panel_without_provenance = valid.clone();
        panel_without_provenance["draw_panels"][0]["execution_ref"] = Value::Null;
        panel_without_provenance["draw_panels"][0]["execution_commit"] = Value::Null;
        assert!(validate_summary(&panel_without_provenance, &protocol).is_err());

        let mut selected_without_provenance = valid.clone();
        selected_without_provenance["rows"][0]["attempts"][0]["execution_ref"] = Value::Null;
        selected_without_provenance["rows"][0]["attempts"][0]["execution_commit"] = Value::Null;
        assert!(validate_summary(&selected_without_provenance, &protocol).is_err());

        let mut selected_attempt_missing = valid.clone();
        selected_attempt_missing["rows"][0]["selected_attempt"] = json!("9999");
        assert!(validate_summary(&selected_attempt_missing, &protocol).is_err());

        let mut claim_without_selection = valid.clone();
        claim_without_selection["rows"][0]["selected_attempt"] = Value::Null;
        assert!(validate_summary(&claim_without_selection, &protocol).is_err());

        let mut with_malformed_attempt = valid.clone();
        with_malformed_attempt["rows"][0]["attempts"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "attempt":"0002","exit_status":1,"termination":"nonzero_exit",
                "execution_ref":null,"execution_commit":null,
                "checkpoint_restored":null,"checkpoint_generation":null,
                "state":"invalid","reason":"malformed retained attempt",
                "output":"/runs/out.jsonl","status":"/runs/status",
                "finished":"/runs/finished.json","metadata":"/runs/meta.json"
            }));
        validate_summary(&with_malformed_attempt, &protocol)
            .expect("an unselected invalid attempt may lack trusted execution provenance");

        fs::remove_dir_all(directory).expect("remove validation directory");
    }

    #[test]
    fn invalid_resume_is_publishable_only_as_no_conclusion() {
        let (protocol, directory) = validation_protocol();
        let mut invalid = consistent_summary(&protocol);
        invalidate_for_resume(&mut invalid);
        validate_summary(&invalid, &protocol).expect("invalid evidence is publishable");

        invalid["global_classification"] = json!("unmeasured");
        assert!(validate_summary(&invalid, &protocol).is_err());
        fs::remove_dir_all(directory).expect("remove validation directory");
    }
}
