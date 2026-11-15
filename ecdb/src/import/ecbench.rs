//! ecbench sessions: every directory holding an `ecbench.session/v1`
//! `session.json`, with its runs (`records.jsonl` or `records.jsonl.gz`,
//! `ecbench.record/v1`), its paired comparisons (`comparisons/*.json`) and a
//! summary of its replay audit (`audit-replay-all.json`). Runs are kept as
//! measured: cost in GAE, S = cost / sqrt(r), the unpriced work listed, and
//! the outcome; a run's curve is linked only through its exact ICV1 slug.
//! Other session schemas (isobench, icms, ...) are counted and skipped.

use super::{bulk, bulk1, pack, read_bytes, s, Ctx, Report};
use anyhow::Result;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub const PACK: &str = "ecbench";
const SESSION_SCHEMA: &str = "ecbench.session/v1";
const RECORD_SCHEMA: &str = "ecbench.record/v1";

const SESSION_SQL: &str = "
INSERT INTO ec.ecbench_session (session_id, path, repository, schema, label, spec_id, spec_sha256, git_commit, git_dirty,
       binary_sha256, host_sha256, env_class_id, status, started_at, finished_at, executions_planned, records_written,
       records_sha256, status_counts, audit, record, provenance_id)
SELECT r.session_id, r.path, r.repository, r.schema, r.label, r.spec_id, r.spec_sha256, r.git_commit, r.git_dirty,
       r.binary_sha256, r.host_sha256, r.env_class_id, r.status,
       to_timestamp(r.started_unix_ms / 1000.0), to_timestamp(r.finished_unix_ms / 1000.0),
       r.executions_planned, r.records_written, r.records_sha256, r.status_counts, r.audit, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(session_id text, path text, repository text, schema text, label text,
     spec_id text, spec_sha256 text, git_commit text, git_dirty boolean, binary_sha256 text, host_sha256 text,
     env_class_id text, status text, started_unix_ms double precision, finished_unix_ms double precision,
     executions_planned int, records_written int, records_sha256 text, status_counts jsonb, audit jsonb, record jsonb)
ON CONFLICT (session_id) DO UPDATE SET audit = EXCLUDED.audit, status = EXCLUDED.status,
  records_written = EXCLUDED.records_written, records_sha256 = EXCLUDED.records_sha256,
  status_counts = EXCLUDED.status_counts, record = EXCLUDED.record";

const RUN_SQL: &str = "
INSERT INTO ec.ecbench_run (record_id, session_id, run_id, seq, round, warmup, arm, role, method_id, method,
       method_family, method_params, workload_id, curve_id, curve_slug, ec1, curve_uid, r, target_index, outcome,
       matches_target, cost_unit, total_gae, s, lower_bound, unpriced, floor_s, ratio_to_floor, phases, counters)
SELECT r.record_id, r.session_id, r.run_id, r.seq, r.round, r.warmup, r.arm, r.role, r.method_id, r.method,
       r.method_family, r.method_params, r.workload_id, c.id, r.slug, r.ec1, r.curve_uid, r.r::numeric, r.target_index,
       r.outcome, r.matches_target, r.cost_unit, r.total_gae, r.s, r.lower_bound, r.unpriced, r.floor_s,
       r.ratio_to_floor, r.phases, r.counters
FROM jsonb_to_recordset($1::jsonb) AS r(record_id text, session_id text, run_id text, seq int, round int,
     warmup boolean, arm text, role text, method_id text, method text, method_family text, method_params jsonb,
     workload_id text, slug text, ec1 text, curve_uid text, r text, target_index int, outcome text,
     matches_target boolean, cost_unit text, total_gae double precision, s double precision, lower_bound boolean,
     unpriced text[], floor_s double precision, ratio_to_floor double precision, phases jsonb, counters jsonb)
JOIN ec.ecbench_session es ON es.session_id = r.session_id
LEFT JOIN ec.curve c ON c.icv1_slug = r.slug
ON CONFLICT (record_id) DO NOTHING";

const COMPARISON_SQL: &str = "
INSERT INTO ec.ecbench_comparison (comparison_id, session_id, path, a, b, record)
SELECT r.comparison_id, r.session_id, r.path, r.a, r.b, r.record
FROM jsonb_to_recordset($1::jsonb) AS r(comparison_id text, session_id text, path text, a jsonb, b jsonb, record jsonb)
JOIN ec.ecbench_session es ON es.session_id = r.session_id
ON CONFLICT (comparison_id, session_id) DO NOTHING";

/// Directories holding a session.json, repository-relative, sorted.
pub fn session_dirs(repo_root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![PathBuf::new()];
    while let Some(rel) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(repo_root.join(&rel)) else { continue };
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let Ok(ft) = e.file_type() else { continue };
            if ft.is_dir() {
                if !matches!(name.as_str(), ".git" | "target" | "node_modules" | super::pack::DIR | "ecdb") {
                    stack.push(rel.join(&name));
                }
            } else if name == "session.json" {
                out.push(rel.to_string_lossy().to_string());
            }
        }
    }
    out.sort();
    out
}

fn f(v: &Value) -> Value {
    if v.is_number() {
        v.clone()
    } else {
        Value::Null
    }
}

fn int(v: &Value) -> Value {
    if v.as_i64().is_some() {
        v.clone()
    } else {
        Value::Null
    }
}

fn run_row(session_id: &str, r: &Value) -> Value {
    let curve = &r["workload"]["curve"];
    let unpriced: Vec<Value> = r["cost"]["unpriced"].as_array().cloned().unwrap_or_default();
    json!({
        "record_id": r["record_id"], "session_id": session_id, "run_id": r["run_id"], "seq": int(&r["seq"]),
        "round": int(&r["round"]), "warmup": r["warmup"], "arm": r["arm"], "role": r["role"],
        "method_id": r["method"]["method_id"], "method": r["method"]["id"], "method_family": r["method"]["family"],
        "method_params": r["method"]["params"], "workload_id": r["workload"]["workload_id"],
        "slug": curve["slug"], "ec1": curve["ec1"], "curve_uid": curve["curve_uid"], "r": s(&curve["r"]),
        "target_index": int(&r["workload"]["target_index"]), "outcome": r["outcome"]["status"],
        "matches_target": r["outcome"]["matches_target"], "cost_unit": r["cost"]["unit"],
        "total_gae": f(&r["cost"]["total_gae"]), "s": f(&r["cost"]["s"]), "lower_bound": r["cost"]["lower_bound"],
        "unpriced": unpriced, "floor_s": f(&r["boundaries"]["floor_s"]),
        "ratio_to_floor": f(&r["boundaries"]["ratio_to_floor"]), "phases": r["phases"], "counters": r["counters"],
    })
}

/// One session directory as a pack row, or None when it is not an ecbench session.
pub fn collect_session(repo_root: &Path, dir: &str) -> Option<Value> {
    let base = repo_root.join(dir);
    let session_bytes = std::fs::read(base.join("session.json")).ok()?;
    let session_sha = hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&session_bytes));
    let session: Value = serde_json::from_slice(&session_bytes).ok()?;
    if session["schema"].as_str() != Some(SESSION_SCHEMA) {
        return Some(json!({ "dir": dir, "skipped": session["schema"] }));
    }
    let sid = session["session_id"].as_str()?.to_string();
    let (mut runs, mut bad, mut records_sha) = (Vec::new(), 0u64, Value::Null);
    let mut records_file = Value::Null;
    if let Ok(bytes) = read_bytes(&base.join("records.jsonl")) {
        records_sha = json!(hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&bytes)));
        records_file = json!(format!("{dir}/records.jsonl"));
        for line in bytes.split(|&b| b == b'\n').filter(|l| !l.is_empty()) {
            match serde_json::from_slice::<Value>(line) {
                Ok(r) if r["schema"].as_str() == Some(RECORD_SCHEMA) && r["record_id"].is_string() => {
                    runs.push(run_row(&sid, &r))
                }
                _ => bad += 1,
            }
        }
    }
    let audit = std::fs::read(base.join("audit-replay-all.json"))
        .ok()
        .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
        .map(|a| {
            json!({
                "schema": a["schema"], "ok": a["ok"], "session_status": a["session_status"],
                "records": a["records"], "verified_records": a["verified_records"], "regraded": a["regraded"],
                "replays": a["replays"].as_array().map(|v| v.len()),
                "problems": a["problems"].as_array().map(|v| v.len()),
                "checks": a["checks"], "auditor_binary_sha256": a["auditor_binary_sha256"],
            })
        });
    let mut comparisons = Vec::new();
    if let Ok(rd) = std::fs::read_dir(base.join("comparisons")) {
        let mut names: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        names.sort();
        for n in names.into_iter().filter(|n| n.ends_with(".json")) {
            if let Some(doc) = std::fs::read(base.join("comparisons").join(&n))
                .ok()
                .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            {
                if let Some(cid) = doc["comparison_id"].as_str() {
                    comparisons.push(json!({
                        "comparison_id": cid, "session_id": sid, "path": format!("{dir}/comparisons/{n}"),
                        "a": doc["a"], "b": doc["b"], "record": doc,
                    }));
                }
            }
        }
    }
    Some(json!({
        "dir": dir, "session": session, "session_file_sha256": session_sha, "records_file": records_file, "records_file_sha256": records_sha,
        "bad_lines": bad, "audit": audit, "comparisons": comparisons, "runs": runs,
    }))
}

const RUNS_PER_LINE: usize = 5000;
const PART_BYTES: usize = 24 << 20;

/// Stream every session into `<out>/ecbench-NNNN.jsonl.gz` parts of about
/// 24 MB uncompressed (3 MB gzipped), so each part fits one Lambda
/// invocation payload. A session with many runs spans several lines
/// (`chunk` 0, 1, ...), each carrying the session and up to 5,000 runs.
/// Returns (sessions, runs, part paths).
pub fn write_pack(repo_root: &Path, out: &Path) -> Result<(usize, usize, Vec<PathBuf>)> {
    use std::io::Write;
    std::fs::create_dir_all(out)?;
    for e in std::fs::read_dir(out)?.flatten() {
        let n = e.file_name().to_string_lossy().to_string();
        if n.starts_with(&format!("{PACK}-")) && n.ends_with(".jsonl.gz") {
            std::fs::remove_file(e.path())?;
        }
    }
    let (mut sessions, mut runs, mut parts) = (0, 0, Vec::new());
    let mut cur: Option<(flate2::write::GzEncoder<std::fs::File>, usize)> = None;
    for dir in session_dirs(repo_root) {
        let Some(mut row) = collect_session(repo_root, &dir) else { continue };
        sessions += 1;
        let all = row.get_mut("runs").map(Value::take).and_then(|v| v.as_array().cloned()).unwrap_or_default();
        runs += all.len();
        let chunks: Vec<&[Value]> = if all.is_empty() { vec![&[][..]] } else { all.chunks(RUNS_PER_LINE).collect() };
        for (k, chunk) in chunks.into_iter().enumerate() {
            row["chunk"] = json!(k);
            row["runs"] = Value::Array(chunk.to_vec());
            if k > 0 {
                row["comparisons"] = json!([]);
            }
            let mut line = serde_json::to_vec(&row)?;
            line.push(b'\n');
            if cur.as_ref().is_none_or(|(_, n)| *n > 0 && n + line.len() > PART_BYTES) {
                if let Some((enc, _)) = cur.take() {
                    enc.finish()?.flush()?;
                }
                let path = out.join(format!("{PACK}-{:04}.jsonl.gz", parts.len()));
                parts.push(path.clone());
                cur = Some((
                    flate2::write::GzEncoder::new(std::fs::File::create(&path)?, flate2::Compression::new(9)),
                    0,
                ));
            }
            let (enc, n) = cur.as_mut().unwrap();
            enc.write_all(&line)?;
            *n += line.len();
        }
    }
    if let Some((enc, _)) = cur.take() {
        enc.finish()?.flush()?;
    }
    Ok((sessions, runs, parts))
}

/// Import pack lines from `reader` (one part, gzip already undone).
pub async fn import_reader(ctx: &Ctx<'_>, reader: impl std::io::Read, report: &mut Report) -> Result<()> {
    use std::io::BufRead;
    for line in std::io::BufReader::with_capacity(1 << 20, reader).split(b'\n') {
        let line = line?;
        if !line.is_empty() {
            load_session(ctx, &serde_json::from_slice(&line)?, report).await?;
        }
    }
    Ok(())
}

/// Import one gzipped pack part (the admin Lambda's `import_pack` action).
pub async fn import_part(ctx: &Ctx<'_>, gz: &[u8]) -> Result<Report> {
    let mut report = Report {
        input: format!("{}/{PACK}-*.jsonl.gz (one part)", pack::DIR),
        sha256: hex::encode(<sha2::Sha256 as sha2::Digest>::digest(gz)),
        ..Default::default()
    };
    import_reader(ctx, flate2::read::GzDecoder::new(gz), &mut report).await?;
    Ok(report)
}

async fn load_session(ctx: &Ctx<'_>, row: &Value, report: &mut Report) -> Result<()> {
    if row.get("skipped").is_some() {
        report.count("sessions_other_schema", 1);
        return Ok(());
    }
    let dir = row["dir"].as_str().unwrap_or_default();
    let session = &row["session"];
    let (path, sha) = match (row["records_file"].as_str(), row["records_file_sha256"].as_str()) {
        (Some(p), Some(h)) => (p.to_string(), h.to_string()),
        _ => (format!("{dir}/session.json"), row["session_file_sha256"].as_str().unwrap_or_default().to_string()),
    };
    let src = ctx.source(&path, &sha, "ecbench session").await?;
    let prov = ctx.provenance(src, "ecbench session records", None).await?;
    let srow = json!({
        "session_id": session["session_id"], "path": dir, "repository": ctx.repository, "schema": session["schema"],
        "label": session["label"], "spec_id": session["spec_id"], "spec_sha256": session["spec_sha256"],
        "git_commit": session["git_commit"], "git_dirty": session["git_dirty"],
        "binary_sha256": session["binary_sha256"], "host_sha256": session["host_sha256"],
        "env_class_id": session["env_class_id"], "status": session["status"],
        "started_unix_ms": f(&session["started_unix_ms"]), "finished_unix_ms": f(&session["finished_unix_ms"]),
        "executions_planned": int(&session["executions_planned"]), "records_written": int(&session["records_written"]),
        "records_sha256": session["records_sha256"], "status_counts": session["status_counts"],
        "audit": row["audit"], "record": session,
    });
    let n = bulk(ctx.client, SESSION_SQL, &[srow], prov).await?;
    if row["chunk"].as_u64().unwrap_or(0) == 0 {
        report.count("sessions", n);
    }
    let runs = row["runs"].as_array().cloned().unwrap_or_default();
    if runs.is_empty() {
        report.count("sessions_without_records", 1);
    }
    report.count("runs", bulk1(ctx.client, RUN_SQL, &runs).await?);
    report.count("runs_seen", runs.len() as u64);
    report.count("record_lines_unparsed", row["bad_lines"].as_u64().unwrap_or(0));
    let comps = row["comparisons"].as_array().cloned().unwrap_or_default();
    report.count("comparisons", bulk1(ctx.client, COMPARISON_SQL, &comps).await?);
    Ok(())
}

/// Import the named session directories only (repository-relative).
pub async fn import_dirs(ctx: &Ctx<'_>, dirs: &[&str]) -> Result<Report> {
    let mut report = Report { input: format!("{} ecbench session directories", dirs.len()), ..Default::default() };
    for dir in dirs {
        if let Some(row) = collect_session(&ctx.repo_root, dir) {
            load_session(ctx, &row, &mut report).await?;
        }
    }
    Ok(report)
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let mut report = Report { input: "ecbench sessions (*/session.json)".into(), ..Default::default() };
    let pack_dir = ctx.repo_root.join(pack::DIR);
    let mut parts: Vec<PathBuf> = std::fs::read_dir(&pack_dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with(&format!("{PACK}-"))))
        .collect();
    parts.sort();
    if !parts.is_empty() {
        report.input = format!("{}/{PACK}-*.jsonl.gz ({} parts)", pack::DIR, parts.len());
        for p in parts {
            import_reader(ctx, flate2::read::GzDecoder::new(std::fs::File::open(&p)?), &mut report).await?;
        }
    } else {
        for dir in session_dirs(&ctx.repo_root) {
            if let Some(row) = collect_session(&ctx.repo_root, &dir) {
                load_session(ctx, &row, &mut report).await?;
            }
        }
    }
    let unlinked: i64 = ctx
        .client
        .query_one(
            "SELECT count(DISTINCT curve_slug) FROM ec.ecbench_run WHERE curve_id IS NULL AND curve_slug IS NOT NULL",
            &[],
        )
        .await?
        .get(0);
    if unlinked > 0 {
        report.problem(format!("{unlinked} run curve slugs have no curve row (kept as text)"));
    }
    Ok(report)
}
