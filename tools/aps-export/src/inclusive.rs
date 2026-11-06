//! Pinned inclusive instruction evidence. Method and independent scalar-check
//! frames are charged separately, then summed in one unit. This never turns
//! simulated instructions into wall time, GAE, or execution attestation.
use crate::{array, audit, canonical, hash, require, text, EvidenceRoot, Result};
use serde_json::{json, Value};
use std::path::Path;

#[path = "../../../src/cryptanalysis/ecbench/callgrind.rs"]
pub mod native;

/// Ignore clocks and placement, retaining the actual mathematical trajectory
/// and every emitted algorithm counter. A profiled run whose wall deadline
/// changes its search cannot supply an unprofiled run's cost.
pub fn trajectory(record: &Value) -> Result<String> {
    Ok(hash(&canonical(
        &json!({"method":record["method"],"workload":record["workload"],
        "seed":record["algorithm_seed"],"counters":record["counters"],"detail":record["detail"],
        "status":record["outcome"]["status"],"recovered":record["outcome"]["recovered"]}),
    )?))
}

fn exact(n: u64) -> Value {
    if n > 9_007_199_254_740_991 {
        json!(n.to_string())
    } else {
        json!(n)
    }
}

struct Frame {
    report: native::SolveIr,
    files: Vec<Value>,
    contents: Vec<(String, String)>,
}

fn input(root: &EvidenceRoot, path: &str, kind: &str) -> Result<(Vec<u8>, Value)> {
    let bytes = root.bytes(path)?;
    let pin = json!({"path":path,"sha256":hash(&bytes),"bytes":bytes.len(),"kind":kind});
    Ok((bytes, pin))
}

fn frame(root: &EvidenceRoot, entries: &Value) -> Result<Frame> {
    let entries = array(entries)?;
    require(
        (2..=4096).contains(&entries.len()),
        "profile needs bounded complete marker parts",
    )?;
    let mut contents = vec![];
    let mut pins = vec![];
    let mut prefix = None;
    let mut total_bytes = 0usize;
    for (i, e) in entries.iter().enumerate() {
        let path = text(&e["path"])?;
        let (bytes, pin) = input(root, path, "raw")?;
        require(pin["sha256"] == e["sha256"], "profile part digest mismatch")?;
        total_bytes = total_bytes
            .checked_add(bytes.len())
            .ok_or("profile byte count overflow")?;
        require(
            total_bytes as u64 <= crate::MAX_FILE_BYTES,
            "profile frame exceeds total byte bound",
        )?;
        let name = Path::new(path)
            .file_name()
            .and_then(|p| p.to_str())
            .ok_or("invalid profile filename")?;
        let (stem, part) = path
            .rsplit_once('.')
            .ok_or("profile part requires numeric suffix")?;
        require(
            part == (i + 1).to_string(),
            "profile part numbers are missing, duplicated or reordered",
        )?;
        if let Some(first) = &prefix {
            require(first == stem, "mixed profile prefixes")?;
        } else {
            prefix = Some(stem.to_string());
        }
        let body = std::str::from_utf8(&bytes).map_err(|e| e.to_string())?;
        // Keep the source parser's valid summary/totals support, but reject
        // contradictory repeated headers before invoking it.
        let mut events = None;
        let mut totals = None;
        for line in body.lines() {
            if let Some(s) = line.strip_prefix("events:") {
                let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
                if let Some(old) = &events {
                    require(old == &s, "contradictory profile events")?;
                }
                events = Some(s);
            }
            if let Some(s) = line
                .strip_prefix("summary:")
                .or_else(|| line.strip_prefix("totals:"))
            {
                let s = s.split_whitespace().collect::<Vec<_>>().join(" ");
                if let Some(old) = &totals {
                    require(old == &s, "contradictory profile totals")?;
                }
                totals = Some(s);
            }
        }
        let labels = events
            .as_ref()
            .ok_or("missing profile events")?
            .split_whitespace()
            .collect::<Vec<_>>();
        require(
            labels.contains(&"Ir")
                && labels
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
                    == labels.len(),
            "ambiguous profile event columns",
        )?;
        require(
            totals
                .as_ref()
                .ok_or("missing profile totals")?
                .split_whitespace()
                .count()
                == labels.len(),
            "profile event/total width mismatch",
        )?;
        contents.push((name.to_owned(), body.to_owned()));
        pins.push(pin);
    }
    let ir = native::solve_ir_from_parts(&contents)?;
    require(
        ir.solve_ir > 0
            && ir
                .parts
                .last()
                .is_some_and(|p| p.trigger == "ecbench_solve"),
        "profile lacks a terminal complete solve frame",
    )?;
    Ok(Frame {
        report: ir,
        files: pins,
        contents,
    })
}

/// Bind an unprofiled record, a separately sealed profile replay, its raw Ir
/// parts and the scalar-check frame. Caller also binds this to its frozen
/// protocol. File content is re-read only through the selected EvidenceRoot.
pub fn read(root: &EvidenceRoot, path: &str, record: &Value) -> Result<(Value, Vec<Value>)> {
    let (packet_bytes, packet_pin) = input(root, path, "receipt")?;
    let packet: Value = serde_json::from_slice(&packet_bytes).map_err(|e| e.to_string())?;
    require(
        packet["schema"] == "aps.inclusive-ir/v1" && packet["record_id"] == record["record_id"],
        "inclusive packet/record identity mismatch",
    )?;
    require(
        packet["scope"] == "method_plus_independent_scalar_check"
            && packet["thread_mode"] == "combined",
        "unsupported instruction scope/thread mode",
    )?;
    require(
        packet["source_commit"]
            .as_str()
            .is_some_and(|s| s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit())),
        "source commit is not pinned",
    )?;
    require(
        packet["protocol_sha256"]
            .as_str()
            .is_some_and(|s| s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())),
        "profile protocol is not pinned",
    )?;
    let profiled_path = text(&packet["profiled_record"])?;
    let (bytes, profiled_pin) = input(root, profiled_path, "raw")?;
    let profiled = audit::record_line(
        std::str::from_utf8(&bytes)
            .map_err(|e| e.to_string())?
            .trim_end_matches('\n'),
    )?;
    let binary = text(&record["binary_sha256"])?;
    require(
        binary.len() == 64
            && binary.bytes().all(|b| b.is_ascii_hexdigit())
            && profiled["binary_sha256"] == record["binary_sha256"]
            && packet["binary_sha256"] == record["binary_sha256"],
        "instruction source binary differs from the record",
    )?;
    let fingerprint = trajectory(record)?;
    require(
        profiled["record_id"] != record["record_id"],
        "profile replay must be separately sealed",
    )?;
    require(
        fingerprint == trajectory(&profiled)? && packet["trajectory_sha256"] == fingerprint,
        "profiling changed the algorithm trajectory",
    )?;
    let Frame {
        report: method,
        mut files,
        contents,
    } = frame(root, &packet["method_parts"])?;
    let mut scalar_ir = 0;
    let mut verification = Value::Null;
    if record["outcome"]["recovered"].is_null() {
        require(
            packet["scalar_parts"].is_null() && record["outcome"]["status"] != "verified",
            "missing scalar frame for a successful recovery",
        )?;
    } else {
        let scalar = audit::scalar_record(record)?;
        if record["outcome"]["status"] == "verified" {
            require(
                scalar["matches_target"] == true,
                "wrong scalar cannot carry successful instruction cost",
            )?;
        }
        let scalar_frame = frame(root, &packet["scalar_parts"])?;
        scalar_ir = scalar_frame.report.solve_ir;
        files.extend(scalar_frame.files);
        verification = scalar;
        let verifier = text(&packet["verifier_binary_sha256"])?;
        require(
            verifier.len() == 64 && verifier.bytes().all(|b| b.is_ascii_hexdigit()),
            "verifier binary is not pinned",
        )?;
        let receipt_path = text(&packet["verifier_receipt"])?;
        let (receipt_bytes, receipt_pin) = input(root, receipt_path, "receipt")?;
        let receipt: Value = serde_json::from_slice(&receipt_bytes).map_err(|e| e.to_string())?;
        require(
            receipt["schema"] == "aps.scalar-check/v1"
                && receipt["record_id"] == record["record_id"]
                && receipt["binary_sha256"] == packet["verifier_binary_sha256"]
                && receipt["matches_target"] == verification["matches_target"],
            "scalar-check receipt identity/result mismatch",
        )?;
        files.push(receipt_pin);
    }
    files.push(packet_pin);
    files.push(profiled_pin);
    let mut paths = std::collections::BTreeSet::new();
    for pin in &files {
        require(
            paths.insert(root.resolve(text(&pin["path"])?)?),
            "profile evidence reuses or aliases another input",
        )?;
    }
    let total = method
        .solve_ir
        .checked_add(scalar_ir)
        .ok_or("inclusive instruction sum overflow")?;
    let online = native::online_ir_from_parts(&contents).ok();
    Ok((
        json!({"schema":"aps.inclusive-ir-audit/v1","record_id":record["record_id"],
        "source_commit":packet["source_commit"],"protocol_sha256":packet["protocol_sha256"],
        "unit":"ir","method_ir":exact(method.solve_ir),"independent_scalar_check_ir":exact(scalar_ir),"total_ir":exact(total),
        "method_online_ir":online.as_ref().map(|r|exact(r.online_ir)),
        "trajectory_sha256":fingerprint,"verification":verification,"counts_reconstructed":true,
        "scope":"complete user-space simulated instructions in the method call and separate scalar-check body; input loading and record sealing/serialization are outside these frames",
        "execution_attested":false,"wall_qualified":false,"gae_calibrated":false}),
        files,
    ))
}
