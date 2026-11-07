//! Native, solver-free extractor and renderer for the p739 timing-boundary audit.
//! It accepts only the exact reviewed source/protocol/record identities below.

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

type Result<T> = std::result::Result<T, String>;

const INSTRUMENT_COMMIT: &str = "543a54b4b1c3be7052872af3ffe393a44c69a3c3";
const EXPORTER_COMMIT: &str = "77765ba8b51d782aa0f32767b3e217e245a81cbd";
const RECORDS_SHA256: &str = "9ccc07c9b55b9180af36c3b910891803dde7f4249742881a7322f748bfdb73f7";
const PROTOCOL_SHA256: &str = "d2ca570ce39f2effa9564576bdb6b6e16148a0a6d511eeb05c3a01565e48d948";
const FLOOR_NS: u64 = 1_000_000;

#[derive(Clone, Copy)]
struct SourceSpec {
    label: &'static str,
    commit: &'static str,
    path: &'static str,
    oid: &'static str,
    sha256: &'static str,
    bytes: u64,
    output: &'static str,
}

const SOURCES: &[SourceSpec] = &[
    SourceSpec {
        label: "instrument_record",
        commit: INSTRUMENT_COMMIT,
        path: "src/cryptanalysis/ecbench/record.rs",
        oid: "ef172f810b4fcbe407a78b402868750f3ecbbd99",
        sha256: "5ebb2ecd907d4e388004e5b3db05614bbdd6c0c0e080bba9ccc7d302e621f407",
        bytes: 93_743,
        output: "instrument-record.rs",
    },
    SourceSpec {
        label: "instrument_three_leads",
        commit: INSTRUMENT_COMMIT,
        path: "src/cryptanalysis/ecbench/three_leads.rs",
        oid: "355b8134f3cb64d0c735240d9b59d88771979769",
        sha256: "02dd6a5e6f665e302c1ccb208f5d9810955dab195aa993714a478df6cd567716",
        bytes: 15_458,
        output: "instrument-three-leads.rs",
    },
    SourceSpec {
        label: "instrument_methods",
        commit: INSTRUMENT_COMMIT,
        path: "src/cryptanalysis/ecbench/methods.rs",
        oid: "87aff2084db53655614ac592a5b84036d16575e7",
        sha256: "7fbffc7d81be8af034c80794c8625351dd1e6deff261533c38ea567612c161a6",
        bytes: 345_878,
        output: "instrument-methods.rs",
    },
    SourceSpec {
        label: "canonical_exporter",
        commit: EXPORTER_COMMIT,
        path: "research/pencil_large_prime_20261029/native_canonical_export/src/lib.rs",
        oid: "2aba0eca0df1a16b4fa679688d8878f1332311f4",
        sha256: "3ba6866c6394c9c7b87cf96f4e2c201434156fee4b15642a1aa21543e99fb5ea",
        bytes: 36_254,
        output: "canonical-exporter-lib.rs",
    },
];

#[derive(Default)]
struct Arm {
    solve: Vec<u64>,
    method: Vec<u64>,
    online: Vec<u64>,
    outer_minus_inner: Vec<u64>,
    warmup: u64,
    measured: u64,
    verified: u64,
    matches_target: u64,
    levels: BTreeMap<String, u64>,
    too_short: u64,
    phase_sets: BTreeMap<String, u64>,
    methods: BTreeSet<String>,
}

struct PlotSeries {
    arm: String,
    label: String,
    minimum: u64,
    median: f64,
    maximum: u64,
}

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn read(path: &Path) -> Result<Vec<u8>> {
    fs::read(path).map_err(|e| format!("read {}: {e}", path.display()))
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("create {}: {e}", path.display()))?;
    f.write_all(bytes)
        .map_err(|e| format!("write {}: {e}", path.display()))
}

fn command_output(mut command: Command, description: &str) -> Result<Vec<u8>> {
    let output = command
        .output()
        .map_err(|e| format!("run {description}: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "{description} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    Ok(output.stdout)
}

fn git_text(repo: &Path, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command.current_dir(repo).args(args);
    let bytes = command_output(command, "git")?;
    String::from_utf8(bytes)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("git output is not UTF-8: {e}"))
}

fn extract_sources(repo: &Path, out: &Path) -> Result<Vec<Value>> {
    let dir = out.join("source-blobs");
    fs::create_dir_all(&dir).map_err(|e| format!("create {}: {e}", dir.display()))?;
    let mut manifest = Vec::new();
    for spec in SOURCES {
        let object = git_text(
            repo,
            &["rev-parse", &format!("{}:{}", spec.commit, spec.path)],
        )?;
        if object != spec.oid {
            return Err(format!("{} git object mismatch", spec.label));
        }
        let mut command = Command::new("git");
        command
            .current_dir(repo)
            .args(["cat-file", "blob", spec.oid]);
        let bytes = command_output(command, "git cat-file blob")?;
        if bytes.len() as u64 != spec.bytes || hash(&bytes) != spec.sha256 {
            return Err(format!("{} source bytes do not match pin", spec.label));
        }
        write_new(&dir.join(spec.output), &bytes)?;
        manifest.push(json!({
            "label": spec.label,
            "repository_path": spec.path,
            "commit": spec.commit,
            "git_blob_oid_sha1": spec.oid,
            "sha256": spec.sha256,
            "bytes": spec.bytes,
            "packet_path": format!("source-blobs/{}", spec.output),
        }));
    }
    Ok(manifest)
}

fn line_of(text: &str, needle: &str) -> Result<u64> {
    text.lines()
        .position(|line| line.contains(needle))
        .map(|n| n as u64 + 1)
        .ok_or_else(|| format!("source marker absent: {needle}"))
}

fn median(values: &[u64]) -> Result<Value> {
    if values.is_empty() {
        return Err("empty timing series".into());
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;
    let (numerator, denominator) = if sorted.len() % 2 == 1 {
        (sorted[mid], 1)
    } else {
        (
            sorted[mid - 1]
                .checked_add(sorted[mid])
                .ok_or("median overflow")?,
            2,
        )
    };
    Ok(json!({"numerator":numerator,"denominator":denominator}))
}

fn series(values: &[u64]) -> Result<Value> {
    let min = *values.iter().min().ok_or("empty timing series")?;
    let max = *values.iter().max().ok_or("empty timing series")?;
    Ok(json!({
        "count": values.len(),
        "unit": "ns",
        "minimum": min,
        "median_exact": median(values)?,
        "maximum": max,
        "below_1ms_count": values.iter().filter(|&&v| v < FLOOR_NS).count(),
    }))
}

fn required_u64(value: &Value, pointer: &str) -> Result<u64> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("record lacks integer {pointer}"))
}

fn required_str<'a>(value: &'a Value, pointer: &str) -> Result<&'a str> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("record lacks string {pointer}"))
}

fn extract_records(path: &Path) -> Result<(BTreeMap<String, Arm>, u64)> {
    let bytes = read(path)?;
    if hash(&bytes) != RECORDS_SHA256 {
        return Err("records hash mismatch".into());
    }
    let file = File::open(path).map_err(|e| format!("open records: {e}"))?;
    let mut arms: BTreeMap<String, Arm> = BTreeMap::new();
    let mut rows = 0u64;
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("read record line: {e}"))?;
        let r: Value = serde_json::from_str(&line).map_err(|e| format!("parse record: {e}"))?;
        rows += 1;
        let arm_name = required_str(&r, "/arm")?.to_string();
        let arm = arms.entry(arm_name).or_default();
        let solve = required_u64(&r, "/time/solve_wall_ns")?;
        let method = required_u64(&r, "/time/method_call_wall_ns")?;
        let online = required_u64(&r, "/online/wall_ns")?;
        if method < solve {
            return Err("method-call interval is below solve interval".into());
        }
        arm.solve.push(solve);
        arm.method.push(method);
        arm.online.push(online);
        arm.outer_minus_inner.push(method - solve);
        if r["warmup"].as_bool().ok_or("warmup is not boolean")? {
            arm.warmup += 1;
        } else {
            arm.measured += 1;
        }
        if required_str(&r, "/outcome/status")? == "verified" {
            arm.verified += 1;
        }
        if r.pointer("/outcome/matches_target")
            .and_then(Value::as_bool)
            == Some(true)
        {
            arm.matches_target += 1;
        }
        *arm.levels
            .entry(required_str(&r, "/isolation/level")?.to_string())
            .or_default() += 1;
        let blockers = r
            .pointer("/isolation/blockers")
            .and_then(Value::as_array)
            .ok_or("isolation blockers are absent")?;
        if blockers.iter().any(|b| {
            b.as_str()
                .is_some_and(|s| s.contains("solve too short to admit wall time"))
        }) {
            arm.too_short += 1;
        }
        let phases = r
            .pointer("/online/phases_ns")
            .and_then(Value::as_object)
            .ok_or("online phases are absent")?;
        let phase_key = phases.keys().cloned().collect::<Vec<_>>().join(",");
        *arm.phase_sets.entry(phase_key).or_default() += 1;
        arm.methods
            .insert(required_str(&r, "/method/id")?.to_string());
    }
    if rows != 1_080 {
        return Err(format!("expected 1080 records, found {rows}"));
    }
    for name in ["pencil", "rho", "rho_AA"] {
        let arm = arms
            .get(name)
            .ok_or_else(|| format!("missing arm {name}"))?;
        if arm.solve.len() != 360 || arm.measured != 300 || arm.warmup != 60 {
            return Err(format!("unexpected {name} execution counts"));
        }
        if arm.verified != 360 || arm.matches_target != 360 {
            return Err(format!("unverified or mismatched outcome in {name}"));
        }
    }
    Ok((arms, rows))
}

fn source_findings(out: &Path) -> Result<Value> {
    let dir = out.join("source-blobs");
    let record = String::from_utf8(read(&dir.join("instrument-record.rs"))?)
        .map_err(|e| format!("record source UTF-8: {e}"))?;
    let three = String::from_utf8(read(&dir.join("instrument-three-leads.rs"))?)
        .map_err(|e| format!("three-leads source UTF-8: {e}"))?;
    let methods = String::from_utf8(read(&dir.join("instrument-methods.rs"))?)
        .map_err(|e| format!("methods source UTF-8: {e}"))?;
    let exporter = String::from_utf8(read(&dir.join("canonical-exporter-lib.rs"))?)
        .map_err(|e| format!("exporter source UTF-8: {e}"))?;

    let floor_line = line_of(&record, "pub const MIN_SOLVE_WALL_NS_V3")?;
    let classifier_line = line_of(&record, "let solve_ns = g.timing.solve_wall_ns;")?;
    let short_line = line_of(&record, "Some(n) if n < MIN_SOLVE_WALL_NS_V3")?;
    let outer_start = line_of(
        &record,
        "let method_call_start = std::time::Instant::now();",
    )?;
    let complete_line = line_of(&record, "crate::cryptanalysis::ecbench::methods::complete(")?;
    let outer_stop = line_of(
        &record,
        "u64::try_from(method_call_start.elapsed().as_nanos())",
    )?;
    let inner_start = line_of(&three, "let start = Instant::now();")?;
    let inner_stop = line_of(&three, "solve_wall_ns: start.elapsed().as_nanos() as u64")?;
    let rho_schema = line_of(&methods, "`rho_solve` (and `recovery_check`")?;
    let exporter_schema = line_of(&exporter, "let wanted: BTreeSet<_> = [")?;
    let exporter_reject = line_of(&exporter, "phase_names_not_exact_five")?;
    Ok(json!({
        "grading_wrong_field_for_protocol_primary": {
            "observed": true,
            "classifier_field": "time.solve_wall_ns",
            "protocol_primary_field": "time.method_call_wall_ns",
            "threshold_ns": FLOOR_NS,
            "source": {"blob":"source-blobs/instrument-record.rs","floor_line":floor_line,"classifier_line":classifier_line,"comparison_line":short_line}
        },
        "common_cold_boundary": {
            "start": "immediately before methods::solve_measured",
            "stop": "after nested instrumentation closes and methods::complete returns",
            "source": {"blob":"source-blobs/instrument-record.rs","start_line":outer_start,"complete_line":complete_line,"stop_line":outer_stop}
        },
        "inner_solve_boundary": {
            "field": "time.solve_wall_ns",
            "source": {"blob":"source-blobs/instrument-three-leads.rs","start_line":inner_start,"stop_line":inner_stop}
        },
        "rho_phase_schema_mismatch": {
            "observed": true,
            "instrument_contract": "rho_solve and recovery_check where marked",
            "exporter_requirement": "exactly the five index-calculus target phases for every arm",
            "instrument_source": {"blob":"source-blobs/instrument-methods.rs","line":rho_schema},
            "exporter_source": {"blob":"source-blobs/canonical-exporter-lib.rs","wanted_line":exporter_schema,"rejection_line":exporter_reject}
        }
    }))
}

fn rational_as_f64(v: &Value) -> Result<f64> {
    let n = v["numerator"].as_u64().ok_or("median numerator absent")?;
    let d = v["denominator"]
        .as_u64()
        .ok_or("median denominator absent")?;
    Ok(n as f64 / d as f64)
}

fn arm_json(arm: &Arm) -> Result<Value> {
    Ok(json!({
        "executions": arm.solve.len(),
        "measured": arm.measured,
        "warmup": arm.warmup,
        "verified": arm.verified,
        "matches_target": arm.matches_target,
        "methods": arm.methods,
        "isolation_levels": arm.levels,
        "too_short_blocker_count": arm.too_short,
        "online_phase_sets": arm.phase_sets,
        "solve_wall_ns": series(&arm.solve)?,
        "method_call_wall_ns": series(&arm.method)?,
        "online_wall_ns": series(&arm.online)?,
        "method_minus_solve_ns": series(&arm.outer_minus_inner)?,
    }))
}

fn protocol_facts(protocol: &Value) -> Result<Value> {
    let primary = protocol
        .pointer("/metrics/primary")
        .and_then(Value::as_array)
        .ok_or("protocol primary metrics absent")?
        .iter()
        .find(|m| m["id"] == "cold_call")
        .ok_or("cold_call metric absent")?;
    let secondary = protocol
        .pointer("/metrics/secondary")
        .and_then(Value::as_array)
        .ok_or("protocol secondary metrics absent")?
        .iter()
        .find(|m| m["id"] == "online")
        .ok_or("online metric absent")?;
    let definition = primary["definition"]
        .as_str()
        .ok_or("cold definition absent")?;
    if primary["source"] != "records.jsonl time.method_call_wall_ns" {
        return Err("cold_call does not select method_call_wall_ns".into());
    }
    if !definition.contains("immediate return before profiling dump") {
        return Err("expected stale cold endpoint prose was not found".into());
    }
    Ok(json!({
        "cold_call_source": primary["source"],
        "cold_call_definition": definition,
        "online_source": secondary["source"],
        "online_definition": secondary["definition"],
        "prose_endpoint_mismatch": {
            "observed": true,
            "protocol_says": "immediate return before profiling dump",
            "selected_field_source_stops": "after instrumentation closure and methods::complete"
        }
    }))
}

fn build_audit(
    records_path: &Path,
    protocol_path: &Path,
    source_manifest: Vec<Value>,
    findings: Value,
    arms: &BTreeMap<String, Arm>,
    rows: u64,
) -> Result<Value> {
    let protocol_bytes = read(protocol_path)?;
    if hash(&protocol_bytes) != PROTOCOL_SHA256 {
        return Err("protocol hash mismatch".into());
    }
    let protocol: Value =
        serde_json::from_slice(&protocol_bytes).map_err(|e| format!("parse protocol: {e}"))?;
    let mut arm_values = Map::new();
    for (name, arm) in arms {
        arm_values.insert(name.clone(), arm_json(arm)?);
    }
    let rho_min_online = arms["rho"]
        .online
        .iter()
        .chain(arms["rho_AA"].online.iter())
        .copied()
        .min()
        .ok_or("no rho online intervals")?;
    let minimum_integer_repetitions = FLOOR_NS.div_ceil(rho_min_online);
    Ok(json!({
        "schema": "pencil-timing-boundary-audit/v1",
        "audit_id": "timing_boundary_audit_20261030",
        "scope": "Neutral source-and-record audit of public synthetic known-answer p739 observations. No solver execution, timing qualification, cost judgment, protocol amendment or experiment approval.",
        "evidence_identity": {
            "instrument_commit": INSTRUMENT_COMMIT,
            "canonical_exporter_commit": EXPORTER_COMMIT,
            "sources": source_manifest,
            "records": {
                "sha256": RECORDS_SHA256,
                "bytes": fs::metadata(records_path).map_err(|e| e.to_string())?.len(),
                "rows": rows,
                "local_source_path": records_path,
                "immutable_object_key": "runs/pencil-p739-20261030-a1-epoch-1/1af573c8-a09c-4dcb-89a4-1af63e1d13e0/session/records.jsonl",
                "immutable_object_version": "j80kl6D8Jf9k6kO82KJRl1G.fWfPCc38",
                "ingestion_receipt": {
                    "path": "../actual_gate_results_20261030/ingestion-receipt.json",
                    "sha256": "e44bee3631fcfcd967b51bff2394dc1b2d3e5a9986f3f881e68da9c902f861a5"
                }
            },
            "protocol": {
                "packet_path": "source-blobs/diagnostic-protocol-v6.json",
                "sha256": PROTOCOL_SHA256,
                "original_path": protocol_path
            },
            "renderer_source_sha256": hash(include_bytes!("main.rs")),
        },
        "observed_facts": {
            "arm_execution_counts": {"pencil":360,"rho":360,"rho_AA":360,"measured_per_arm":300,"warmup_per_arm":60},
            "all_outcomes_verified_and_match_target": true,
            "timing_distributions": arm_values,
            "source_and_protocol": findings,
            "protocol": protocol_facts(&protocol)?,
            "classification": "The classifier/protocol field mismatch is observed. Separately, every rho and rho_AA method-call interval is also below the unchanged 1ms floor, so selecting that broader field alone would admit none of the 720 control rows.",
            "timing_qualification": null,
            "qualified_ratios": null
        },
        "prospective_option_not_approved": {
            "status": "unapproved_design_candidate_requiring_new_predata_protocol_budget_and_review",
            "fixed_repetitions_discussed": 8,
            "observed_minimum_rho_or_rho_AA_online_ns": rho_min_online,
            "minimum_integer_repetitions_to_cross_1ms_for_that_observed_minimum": minimum_integer_repetitions,
            "arithmetic_only": {"eight_times_observed_minimum_ns": rho_min_online.checked_mul(8).ok_or("multiplication overflow")?},
            "constraints_if_reviewed": [
                "unchanged 1ms admission floor",
                "same exact targets, arms, controls and stream identities",
                "fresh cold state or an independently justified reset boundary",
                "fixed repetition count frozen before new timing data",
                "retain every raw repetition and fail closed below threshold",
                "one-invocation operation counts kept separate from timing amplification",
                "method-family phase schemas with exact integer phase sums",
                "no reinterpretation or edit of the present records or frozen protocol"
            ],
            "non_claim": "The audit does not select, approve or authorize K=8, a new phase schema, a new grading rule, a budget or another experiment."
        },
        "limits": [
            "Minima, exact medians and maxima describe these 1080 retained rows only.",
            "Warmup and measured records are both included in the displayed ranges and are counted separately.",
            "No confidence interval, significance test, speed ratio or performance verdict is computed.",
            "The packet audits source semantics and retained observations; it is not an independent mathematical proof or timing qualification."
        ]
    }))
}

fn svg_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn x_for(ns: f64, left: f64, width: f64) -> f64 {
    let ms = ns / 1_000_000.0;
    let low = 0.1f64.log10();
    let high = 20.0f64.log10();
    left + (ms.log10() - low) / (high - low) * width
}

fn series_points(audit: &Value) -> Result<Vec<PlotSeries>> {
    let mut out = Vec::new();
    for arm in ["pencil", "rho", "rho_AA"] {
        for (field, label) in [
            ("solve_wall_ns", "solve"),
            ("method_call_wall_ns", "method-call"),
            ("online_wall_ns", "online"),
        ] {
            let s = &audit["observed_facts"]["timing_distributions"][arm][field];
            out.push(PlotSeries {
                arm: arm.to_string(),
                label: label.to_string(),
                minimum: s["minimum"].as_u64().ok_or("minimum absent")?,
                median: rational_as_f64(&s["median_exact"])?,
                maximum: s["maximum"].as_u64().ok_or("maximum absent")?,
            });
        }
    }
    Ok(out)
}

fn make_svg(audit: &Value) -> Result<String> {
    let rows = series_points(audit)?;
    let left = 250.0;
    let width = 870.0;
    let mut out = String::from(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1200\" height=\"760\" viewBox=\"0 0 1200 760\"><rect width=\"1200\" height=\"760\" fill=\"white\"/><g font-family=\"DejaVu Sans,sans-serif\" fill=\"#172b3a\">",
    );
    out.push_str("<text x=\"40\" y=\"42\" font-size=\"25\" font-weight=\"bold\">Observed single-call timing intervals; no timing qualification</text><text x=\"40\" y=\"70\" font-size=\"16\">All 1,080 retained p739 rows; each arm n=360 (300 measured + 60 warmup). Whisker=min-max, dot=exact median.</text>");
    for (tick_ms, label) in [
        (0.1, "0.1"),
        (0.2, "0.2"),
        (0.5, "0.5"),
        (1.0, "1"),
        (2.0, "2"),
        (5.0, "5"),
        (10.0, "10"),
        (20.0, "20"),
    ] {
        let x = x_for(tick_ms * 1_000_000.0, left, width);
        let special = tick_ms == 1.0;
        out.push_str(&format!("<line x1=\"{x:.2}\" y1=\"100\" x2=\"{x:.2}\" y2=\"660\" stroke=\"{}\" stroke-width=\"{}\" stroke-dasharray=\"{}\"/><text x=\"{x:.2}\" y=\"690\" font-size=\"14\" text-anchor=\"middle\">{label}</text>", if special {"#b8322c"} else {"#d5dee5"}, if special {2} else {1}, if special {"6 4"} else {"2 5"}));
    }
    out.push_str(&format!("<text x=\"{:.2}\" y=\"94\" fill=\"#b8322c\" font-size=\"14\" text-anchor=\"middle\">unchanged 1 ms floor</text>", x_for(1_000_000.0,left,width)));
    for (i, row) in rows.iter().enumerate() {
        let y = 125.0 + i as f64 * 57.0;
        let color = match row.label.as_str() {
            "solve" => "#2b6cb0",
            "method-call" => "#c05621",
            _ => "#2f855a",
        };
        let x1 = x_for(row.minimum as f64, left, width);
        let xm = x_for(row.median, left, width);
        let x2 = x_for(row.maximum as f64, left, width);
        let row_label = svg_escape(&format!("{} / {}", row.arm, row.label));
        out.push_str(&format!("<text x=\"40\" y=\"{:.2}\" font-size=\"16\">{row_label}</text><line x1=\"{x1:.2}\" y1=\"{y:.2}\" x2=\"{x2:.2}\" y2=\"{y:.2}\" stroke=\"{color}\" stroke-width=\"5\"/><line x1=\"{x1:.2}\" y1=\"{:.2}\" x2=\"{x1:.2}\" y2=\"{:.2}\" stroke=\"{color}\" stroke-width=\"2\"/><line x1=\"{x2:.2}\" y1=\"{:.2}\" x2=\"{x2:.2}\" y2=\"{:.2}\" stroke=\"{color}\" stroke-width=\"2\"/><circle cx=\"{xm:.2}\" cy=\"{y:.2}\" r=\"6\" fill=\"white\" stroke=\"{color}\" stroke-width=\"3\"/>",y+5.0,y-9.0,y+9.0,y-9.0,y+9.0));
    }
    out.push_str("<text x=\"685\" y=\"730\" font-size=\"17\" text-anchor=\"middle\">wall interval (milliseconds, logarithmic axis)</text><text x=\"40\" y=\"752\" font-size=\"13\">Source: hash-bound records.jsonl. Display is descriptive; no ratio or admission is inferred.</text></g></svg>");
    Ok(out)
}

fn tex_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\textbackslash{}"),
            '_' => out.push_str("\\_"),
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '#' => out.push_str("\\#"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '$' => out.push_str("\\$"),
            '^' => out.push_str("\\textasciicircum{}"),
            '~' => out.push_str("\\textasciitilde{}"),
            _ => out.push(c),
        }
    }
    out
}

fn ns_range(audit: &Value, arm: &str, field: &str) -> Result<String> {
    let s = &audit["observed_facts"]["timing_distributions"][arm][field];
    let min = s["minimum"].as_u64().ok_or("minimum absent")?;
    let max = s["maximum"].as_u64().ok_or("maximum absent")?;
    let med = rational_as_f64(&s["median_exact"])?;
    Ok(format!(
        "{:.3}--{:.3} ms; median {:.3} ms",
        min as f64 / 1e6,
        max as f64 / 1e6,
        med / 1e6
    ))
}

fn make_tex(audit: &Value) -> Result<String> {
    let rows = series_points(audit)?;
    let mut out = String::from("\\documentclass[10pt]{article}\n\\usepackage[margin=0.65in]{geometry}\n\\usepackage{booktabs,longtable,hyperref,tikz,xcolor}\n\\usetikzlibrary{positioning}\n\\setlength{\\parindent}{0pt}\\setlength{\\parskip}{5pt}\n\\definecolor{solve}{HTML}{2B6CB0}\\definecolor{method}{HTML}{C05621}\\definecolor{online}{HTML}{2F855A}\\definecolor{floor}{HTML}{B8322C}\n\\begin{document}\n\\raggedright\\sloppy\n{\\LARGE p739 timing-boundary audit}\\\\[4pt]\n{\\small Neutral source-and-record evidence packet, 2026-10-30}\\par\n\\textbf{Scope.} Public synthetic known-answer observations only. This report records source semantics and retained ranges. It makes no performance judgment, protocol amendment, experiment approval, qualified ratio, or new solver execution.\n\\section*{Bound evidence}\nInstrument commit: \\texttt{543a54b4b1c3be7052872af3ffe393a44c69a3c3}.\\\\\nRecord SHA-256: \\texttt{9ccc07c9b55b9180af36c3b910891803dde7f4249742881a7322f748bfdb73f7}.\\\\\nProtocol SHA-256: \\texttt{d2ca570ce39f2effa9564576bdb6b6e16148a0a6d511eeb05c3a01565e48d948}.\\\\\nThe packet retains complete source blobs and their Git object IDs in \\texttt{audit.json} and \\texttt{source-blobs/}.\n\\section*{Observed interval ranges}\n\\begin{longtable}{llll}\n\\toprule Arm & solve wall & common method-call wall & online wall \\\\ \\midrule\n");
    for arm in ["pencil", "rho", "rho_AA"] {
        out.push_str(&format!(
            "{} & {} & {} & {} \\\\",
            tex_escape(arm),
            tex_escape(&ns_range(audit, arm, "solve_wall_ns")?),
            tex_escape(&ns_range(audit, arm, "method_call_wall_ns")?),
            tex_escape(&ns_range(audit, arm, "online_wall_ns")?)
        ));
        out.push('\n');
    }
    out.push_str("\\bottomrule\\end{longtable}\nEach arm contains 360 rows: 300 measured and 60 warmup. All 1,080 outcomes are verified and match the public target. Pencil rows are L2. All rho and rho\\_AA rows are L1 and carry the solve-too-short blocker.\n\\begin{center}\\begin{tikzpicture}[x=1cm,y=1cm]\n");
    let left = 2.3;
    let width = 13.2;
    for (tick_ms, label) in [
        (0.1, "0.1"),
        (0.2, "0.2"),
        (0.5, "0.5"),
        (1.0, "1"),
        (2.0, "2"),
        (5.0, "5"),
        (10.0, "10"),
        (20.0, "20"),
    ] {
        let x = x_for(tick_ms * 1e6, left, width);
        let special = tick_ms == 1.0;
        out.push_str(&format!("\\draw[{},{}] ({x:.3},0.2) -- ({x:.3},9.7); \\node[below] at ({x:.3},0.2) {{{label}}};\n", if special{"floor"}else{"gray!35"},if special{"thick,dashed"}else{"thin,dotted"}));
    }
    out.push_str(&format!(
        "\\node[floor,above] at ({:.3},9.7) {{unchanged 1 ms floor}};\n",
        x_for(1e6, left, width)
    ));
    for (i, row) in rows.iter().enumerate() {
        let y = 9.2 - i as f64;
        let color = match row.label.as_str() {
            "solve" => "solve",
            "method-call" => "method",
            _ => "online",
        };
        let x1 = x_for(row.minimum as f64, left, width);
        let xm = x_for(row.median, left, width);
        let x2 = x_for(row.maximum as f64, left, width);
        out.push_str(&format!("\\node[anchor=east] at (2.15,{y:.3}) {{{}}}; \\draw[{color},line width=1.2pt] ({x1:.3},{y:.3}) -- ({x2:.3},{y:.3}); \\draw[{color}] ({x1:.3},{:.3})--({x1:.3},{:.3}) ({x2:.3},{:.3})--({x2:.3},{:.3}); \\draw[{color},fill=white,line width=1pt] ({xm:.3},{y:.3}) circle (2.2pt);\n",tex_escape(&format!("{} / {}", row.arm, row.label)),y-0.12,y+0.12,y-0.12,y+0.12));
    }
    out.push_str("\\node at (8.9,-0.55) {wall interval (milliseconds, logarithmic axis)};\\end{tikzpicture}\\end{center}\n\\textit{Figure.} Min--max whiskers and exact median dot from all retained rows. This is descriptive only.\n\\section*{Observed source and protocol facts}\n\\begin{enumerate}\n\\item Grading version 3 compares \\texttt{time.solve\\_wall\\_ns} with the unchanged 1,000,000 ns floor. The protocol primary cold field is \\texttt{time.method\\_call\\_wall\\_ns}.\n\\item The protocol prose says the cold endpoint is the immediate method return before the profiling dump. The selected method-call field actually stops after nested instrumentation closes and \\texttt{methods::complete} returns.\n\\item Every one of the 720 rho/rho\\_AA method-call intervals is also below 1 ms. Changing only the classifier field would therefore admit none of those rows.\n\\item The instrument contract permits \\texttt{rho\\_solve} and \\texttt{recovery\\_check} for a generic method. The canonical exporter checks every arm for exactly the five index-calculus target phase names.\n\\end{enumerate}\n\\textbf{Qualification status.} Raw intervals and operation/count evidence remain observations. Timing qualification and qualified ratios are null in this audit. Process wall is not substituted because it includes work outside the declared common method boundary.\n\\section*{Prospective design question; not approved}\nA fixed repetition count of eight was discussed because the smallest observed rho/rho\\_AA online interval is 160,313 ns and eight times that observation is 1,282,504 ns. This arithmetic does not approve K=8. Any repetition factor, boundary, phase schema, budget, or new run requires a new pre-data protocol and review. A follow-up would have to preserve the exact targets, arms, controls, streams and unchanged 1 ms floor; retain each repetition; establish a cold-state/reset boundary; separate one-invocation operation counts from timing amplification; and fail closed.\n\\section*{Limits}\nRanges include warmup and measured rows, counted separately. No uncertainty interval, significance test, speed ratio, cost ratio, or ranking is computed. This is a source-and-record audit, not an independent mathematical proof or timing qualification.\n\\section*{Reproduction}\nThe native Rust extractor in \\texttt{native\\_extract/} accepts only the pinned source objects, record hash, and protocol hash. It invokes no solver. Exact commands and the packet manifest are in \\texttt{README.md} and \\texttt{SHA256SUMS}.\n\\end{document}\n");
    Ok(out)
}

fn run_pdflatex(out: &Path) -> Result<()> {
    let output = Command::new("pdflatex")
        .current_dir(out)
        .env("SOURCE_DATE_EPOCH", "1793318400")
        .env("FORCE_SOURCE_DATE", "1")
        .args([
            "-no-shell-escape",
            "-interaction=nonstopmode",
            "-halt-on-error",
            "report.tex",
        ])
        .output()
        .map_err(|e| format!("run pdflatex: {e}"))?;
    write_new(&out.join("pdflatex.stdout.log"), &output.stdout)?;
    write_new(&out.join("pdflatex.stderr.log"), &output.stderr)?;
    if !output.status.success() {
        return Err("pdflatex failed; logs retained".into());
    }
    for name in ["report.aux", "report.out", "report.log"] {
        let path = out.join(name);
        if path.exists() {
            fs::remove_file(&path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        }
    }
    Ok(())
}

fn usage() -> String {
    "usage: pencil-timing-boundary-audit build REPO RECORDS.jsonl PROTOCOL.json OUTPUT_DIRECTORY"
        .into()
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 || args[1] != "build" {
        return Err(usage());
    }
    let repo = PathBuf::from(&args[2]);
    let records = PathBuf::from(&args[3]);
    let protocol = PathBuf::from(&args[4]);
    let out = PathBuf::from(&args[5]);
    fs::create_dir_all(&out).map_err(|e| format!("create output: {e}"))?;
    for name in [
        "audit.json",
        "timing-ranges.svg",
        "report.tex",
        "report.pdf",
    ] {
        if out.join(name).exists() {
            return Err(format!(
                "refusing to overwrite {}",
                out.join(name).display()
            ));
        }
    }
    let source_manifest = extract_sources(&repo, &out)?;
    let protocol_bytes = read(&protocol)?;
    if hash(&protocol_bytes) != PROTOCOL_SHA256 {
        return Err("protocol hash mismatch".into());
    }
    write_new(
        &out.join("source-blobs/diagnostic-protocol-v6.json"),
        &protocol_bytes,
    )?;
    let (arms, rows) = extract_records(&records)?;
    let findings = source_findings(&out)?;
    let audit = build_audit(&records, &protocol, source_manifest, findings, &arms, rows)?;
    write_new(
        &out.join("audit.json"),
        &serde_json::to_vec_pretty(&audit).map_err(|e| format!("serialize audit: {e}"))?,
    )?;
    write_new(&out.join("timing-ranges.svg"), make_svg(&audit)?.as_bytes())?;
    write_new(&out.join("report.tex"), make_tex(&audit)?.as_bytes())?;
    run_pdflatex(&out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_median_is_not_rounded() {
        assert_eq!(
            median(&[1, 2]).unwrap(),
            json!({"numerator":3,"denominator":2})
        );
        assert_eq!(
            median(&[3, 1, 2]).unwrap(),
            json!({"numerator":2,"denominator":1})
        );
    }

    #[test]
    fn logarithmic_axis_pins_floor_between_endpoints() {
        let left = 250.0;
        let width = 870.0;
        let low = x_for(100_000.0, left, width);
        let floor = x_for(1_000_000.0, left, width);
        let high = x_for(20_000_000.0, left, width);
        assert!((low - left).abs() < 1e-9);
        assert!(floor > low && floor < high);
        assert!((high - left - width).abs() < 1e-9);
    }

    #[test]
    fn series_retains_exact_floor_count() {
        let s = series(&[999_999, 1_000_000, 1_000_001]).unwrap();
        assert_eq!(s["below_1ms_count"], 1);
        assert_eq!(
            s["median_exact"],
            json!({"numerator":1_000_000,"denominator":1})
        );
    }
}
