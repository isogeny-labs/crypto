//! Factor bases of the index-calculus work (migration 0011). Sources:
//!
//! - every ecbench run whose record carries `factor_base` (FB1 facts) and
//!   `fbm_id`: the factor base, its FBM1 configuration (rebuilt the way
//!   `ecbench::fbm::config_from_ic_method` builds it) and the run's link;
//! - the factor-base catalog and dumps (`research/ecbench_factor_bases_20261013/`):
//!   plug-in strings, store paths and the points;
//! - the orbit-aware large-prime receipts and the SECG small-x yield files,
//!   whose bases have no FB1 id (ecdb mints an FBS1 spec id for them);
//! - the cryptanalysis factor-base archive index (`experiments/fb-archive/`);
//! - the n83 design grid (`research/koblitz_n83_factor_base_sweep_20261008/design.json`):
//!   its 1,050 base specs as FBS1 specs, grouped under `factor_base_design`;
//! - the IC tool program's candidate manifests and sealed claims
//!   (`research/ic_tool_program/rule/*/`): one `pointset` base per usable
//!   point set and one `ic_claim` per curve x target x round x rule version.
//!
//! FB1 and FBM1 ids are recomputed from their preimages with the ecbench
//! canonical JSON codec; the result is stored in `digest_verified`, and a
//! mismatch is reported but still loaded under the producer's id.

use super::{bulk, bulk1, pack, read_bytes, Ctx, Report};
use anyhow::{bail, Result};
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::Path;

pub const PACK: &str = "factor-bases";
const CATALOG_DIR: &str = "research/ecbench_factor_bases_20261013";
const LARGE_PRIME_DIR: &str = "research/orbit_aware_large_prime_20261011";
const SECG_DIR: &str = "research/secg_small_x_20261014/data";
const N83_DESIGN: &str = "research/koblitz_n83_factor_base_sweep_20261008/design.json";
const IC_TOOL_RULES: &str = "research/ic_tool_program/rule";
const ARCHIVE_DIR: &str = "experiments/fb-archive";
const FB_SCHEMA: &str = "ecbench.factor_base/v1";
const FBM_SCHEMA: &str = "ecbench.factor_base_measurement/v1";
const SPEC_SCHEMA: &str = "ecdb.factor_base_spec/v1";
/// `ecbench::fbm::INSTRUMENT_IC_PIPELINE`, the instrument every ecbench IC run is sealed under.
const INSTRUMENT_IC_PIPELINE: &str = "ecbench.ic_pipeline/v1";

// ------------------------------------------------- canonical JSON (ecbench/canonical.rs)

/// The ecbench identity codec: keys sorted, no whitespace, ASCII only with
/// Python's `ensure_ascii` escapes, integers only.
pub fn canonical(v: &Value) -> Result<String> {
    let mut out = String::new();
    write_canonical(v, &mut out)?;
    Ok(out)
}

fn write_canonical(v: &Value, out: &mut String) -> Result<()> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => {
            let lexeme = n.to_string();
            let digits = lexeme.strip_prefix('-').unwrap_or(&lexeme);
            if digits.is_empty()
                || !digits.bytes().all(|b| b.is_ascii_digit())
                || (digits != "0" && digits.starts_with('0'))
            {
                bail!("number {lexeme} in an identity is not a plain integer");
            }
            out.push_str(if lexeme == "-0" { "0" } else { &lexeme });
        }
        Value::String(s) => write_str(s, out),
        Value::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_canonical(item, out)?;
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (i, k) in keys.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_str(k, out);
                out.push(':');
                write_canonical(&map[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn write_str(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
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
                let mut buf = [0u16; 2];
                for unit in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{:04x}", unit));
                }
            }
        }
    }
    out.push('"');
}

/// (`<prefix>h` + 12 hex, full SHA-256) of the canonical form, as `ecbench::canonical::short_id`.
pub fn short_id(prefix: &str, v: &Value) -> Result<(String, String)> {
    let full = crate::math::sha256_hex(&canonical(v)?);
    Ok((format!("{prefix}h{}", &full[..12]), full))
}

/// Recompute an FB1 id from its facts: Some(true/false), or None when a preimage field is missing.
pub fn verify_fb1(curve_slug: &str, facts: &Value) -> Option<bool> {
    let pre = json!({
        "schema": FB_SCHEMA, "curve": curve_slug, "family": facts.get("family")?,
        "params": facts.get("params")?, "columns": facts.get("columns")?,
        "signed_points": facts.get("signed_points")?, "points_sha256": facts.get("points_sha256")?,
    });
    let (id, full) = short_id("FB1", &pre).ok()?;
    Some(Some(id.as_str()) == facts["fb_id"].as_str() && facts["fb_sha256"].as_str().is_none_or(|s| s == full))
}

/// The FBM1 configuration of an ecbench IC run (`ecbench::fbm::config_from_ic_method`).
pub fn fbm_config(curve_slug: &str, facts: &Value, method_params: &Value) -> Value {
    let p = |k: &str, default: &str| method_params[k].as_str().unwrap_or(default).to_string();
    json!({
        "schema": FBM_SCHEMA, "curve": curve_slug, "fb_id": facts["fb_id"], "fb_sha256": facts["fb_sha256"],
        "family": facts["family"], "fb_params": facts["params"], "oracle": p("oracle", ""), "solver": p("solver", ""),
        "linalg": p("linalg", "incremental-gauss"), "instrument": INSTRUMENT_IC_PIPELINE,
    })
}

/// FBS1: ecdb's id for a factor base known only by its construction.
pub fn spec_id(curve: &str, family: &str, params: &Value) -> Result<(String, String)> {
    short_id("FBS1", &json!({ "schema": SPEC_SCHEMA, "curve": curve, "family": family, "params": params }))
}

// ------------------------------------------------- collection (into the pack)

fn read_doc(path: &Path) -> Option<Value> {
    read_bytes(path).ok().and_then(|b| serde_json::from_slice(&b).ok())
}

fn files_under(root: &Path, keep: &dyn Fn(&str) -> bool) -> Vec<std::path::PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if keep(&e.file_name().to_string_lossy()) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn rel(repo_root: &Path, p: &Path) -> String {
    p.strip_prefix(repo_root).unwrap_or(p).to_string_lossy().to_string()
}

/// ecbench runs: distinct FB1 facts and FBM1 configurations, and one link per run.
fn collect_ecbench(repo_root: &Path, rows: &mut Vec<Value>) {
    let mut fbs: BTreeMap<String, Value> = BTreeMap::new();
    let mut fbms: BTreeMap<String, Value> = BTreeMap::new();
    for dir in super::ecbench::session_dirs(repo_root) {
        let Ok(bytes) = read_bytes(&repo_root.join(&dir).join("records.jsonl")) else { continue };
        for line in bytes.split(|&b| b == b'\n') {
            // Most runs are rho / BSGS: skip them before parsing.
            if !line.windows(7).any(|w| w == b"\"fb_id\"") {
                continue;
            }
            let Ok(r) = serde_json::from_slice::<Value>(line) else { continue };
            let facts = &r["factor_base"];
            let (Some(fb_id), Some(record_id)) = (facts["fb_id"].as_str(), r["record_id"].as_str()) else {
                continue;
            };
            let slug = r["workload"]["curve"]["slug"].as_str().unwrap_or_default().to_string();
            fbs.entry(fb_id.to_string()).or_insert_with(|| {
                json!({ "kind": "fb1", "curve_slug": slug, "ec1": r["workload"]["curve"]["ec1"],
                        "facts": facts, "plug_in": r["method"]["params"]["factor_base"], "first_seen": dir })
            });
            let fbm_id = r["fbm_id"].as_str();
            if let Some(fbm_id) = fbm_id {
                fbms.entry(fbm_id.to_string()).or_insert_with(|| {
                    json!({ "kind": "fbm", "fbm_id": fbm_id, "config": fbm_config(&slug, facts, &r["method"]["params"]) })
                });
            }
            rows.push(json!({ "kind": "run", "record_id": record_id, "fb_id": fb_id, "fbm_id": fbm_id }));
        }
    }
    rows.extend(fbs.into_values());
    rows.extend(fbms.into_values());
}

/// The factor-base catalog and its dumps (points).
fn collect_catalog(repo_root: &Path, rows: &mut Vec<Value>) {
    let dir = repo_root.join(CATALOG_DIR);
    if let Some(cat) = read_doc(&dir.join("catalog.json")) {
        for e in cat["entries"].as_array().into_iter().flatten() {
            rows.push(json!({ "kind": "catalog", "entry": e, "commit": cat["commit"], "path": format!("{CATALOG_DIR}/catalog.json") }));
        }
    }
    for p in files_under(&dir.join("dumps"), &|n| n.ends_with(".json")) {
        if let Some(d) = read_doc(&p) {
            rows.push(json!({ "kind": "dump", "path": rel(repo_root, &p), "curve": d["curve"],
                              "facts": d["factor_base"], "points": d["points"] }));
        }
    }
}

/// Orbit-aware large-prime receipts: one result per cell.
fn collect_large_prime(repo_root: &Path, rows: &mut Vec<Value>) {
    for p in files_under(&repo_root.join(LARGE_PRIME_DIR), &|n| n.starts_with("cell_") && n.ends_with(".json")) {
        let Some(d) = read_doc(&p) else { continue };
        let rep = &d["report"];
        let cfg = &rep["config"];
        let fb = &rep["factor_base"];
        let params = json!({
            "small_dimension": super::s(&cfg["small_dimension"]), "envelope_dimension": super::s(&cfg["envelope_dimension"]),
            "large_quotient": if fb.get("orbit").is_some_and(|o| !o.is_null()) { "frobenius" } else { "none" },
        });
        let mut extra = json!({
            "n": d["n"], "rep": d["rep"], "planted_log": d["planted_log"], "config": cfg,
            "filter_rank": rep["filter_pipeline"]["rank"], "complete_relations": rep["filter_pipeline"]["complete_relations"],
            "exhausted": rep["exhausted"], "peak_rss_bytes": d["peak_rss_bytes"],
        });
        if let Some(o) = fb.get("orbit") {
            extra["orbit"] = json!({ "lambda": o["lambda"], "large_orbits": o["large_orbits"], "large_points": o["large_points"],
                                     "merged_points": o["merged_points"] });
        }
        rows.push(json!({
            "kind": "result", "source": "large_prime_cell", "source_key": rel(repo_root, &p),
            "curve": d["curve"]["name"], "family": "large-prime-envelope", "params": params,
            "fb": { "columns": fb["columns"], "signed_points": fb["raw_points"],
                    "description": format!("small subspace dim {} + large envelope dim {} ({} small / {} large columns)",
                        cfg["small_dimension"], cfg["envelope_dimension"], fb["small_columns"], fb["large_columns"]) },
            "arm": d["arm"], "seed": cfg["seed"], "status": if rep["exhausted"] == json!(true) { json!("exhausted") } else { json!("complete") },
            "solved": d["recovered_matches_planted"], "trials": rep["counters"]["trials"],
            "relations": rep["counters"]["accepted_relations"], "matrix_rank": rep["counters"]["matrix_rank"],
            "independent_rows": rep["counters"]["independent_rows"], "cpu_ns": d["cpu_process_ns"],
            "counters": rep["counters"], "extra": extra,
        }));
    }
}

/// SECG small-x yield files.
fn collect_secg(repo_root: &Path, rows: &mut Vec<Value>) {
    for p in files_under(&repo_root.join(SECG_DIR), &|n| n.ends_with(".json")) {
        let Some(d) = read_doc(&p) else { continue };
        let a = &d["arguments"];
        if a["factor_base"].as_str() != Some("small_x") {
            continue;
        }
        let r = &d["report"];
        let trials = r["mc_trials"].as_i64();
        let hits = r["mc_hits_verified"].as_i64();
        rows.push(json!({
            "kind": "result", "source": "secg_small_x", "source_key": rel(repo_root, &p),
            "curve": a["curve"], "family": "small-x", "params": { "size": super::s(&a["size"]) },
            "fb": { "columns": r["base_abscissae"], "signed_points": r["base_points"], "abscissae": r["base_abscissae"],
                    "description": format!("the {} smallest abscissae (largest x = {})", a["size"], r["largest_small_x"]) },
            "arm": a["factor_base"], "seed": a["seed"], "status": d["status"], "solved": Value::Null,
            "trials": trials, "relations": hits,
            "yield_per_trial": match (trials, hits) { (Some(t), Some(h)) if t > 0 => json!(h as f64 / t as f64), _ => Value::Null },
            "yield_log2": r["exact_yield_log2"], "counters": Value::Null,
            "extra": { "exact_yield": r["exact_yield"], "model_yield_log2": r["model_yield_log2"], "mc_ci95": r["mc_ci95"],
                       "expected_trials_log2": r["expected_trials_log2"], "trials_for_full_system_log2": r["trials_for_full_system_log2"],
                       "rho_log2": r["rho_log2"], "controls_recovered": r["controls_recovered"], "timing_ms": r["timing_ms"],
                       "evidence_scope": d["evidence_scope"], "software": d["software"] },
        }));
    }
}

fn csv_rows(bytes: &[u8]) -> Vec<BTreeMap<String, String>> {
    let text = String::from_utf8_lossy(bytes);
    let mut lines = text.lines();
    let Some(header) = lines.next() else { return Vec::new() };
    let cols: Vec<&str> = header.split(',').collect();
    lines
        .filter(|l| !l.is_empty())
        .map(|l| cols.iter().zip(l.split(',')).map(|(k, v)| (k.to_string(), v.to_string())).collect())
        .collect()
}

/// The cryptanalysis factor-base archive index and its digest aliases.
fn collect_archive(repo_root: &Path, rows: &mut Vec<Value>) {
    let Some(root) = super::siblings::sibling_root(repo_root, "cryptanalysis") else { return };
    let dir = root.join(ARCHIVE_DIR);
    if let Ok(b) = read_bytes(&dir.join("index.csv")) {
        for r in csv_rows(&b) {
            rows.push(json!({ "kind": "archive", "row": r }));
        }
    }
    if let Ok(b) = read_bytes(&dir.join("aliases.csv")) {
        for r in csv_rows(&b) {
            rows.push(json!({ "kind": "archive_alias", "row": r }));
        }
    }
}

/// The n83 factor-base design grid: one row for the design (base specs inline).
fn collect_designs(repo_root: &Path, rows: &mut Vec<Value>) {
    if let Some(d) = read_doc(&repo_root.join(N83_DESIGN)) {
        rows.push(json!({ "kind": "design", "path": N83_DESIGN, "doc": d }));
    }
}

/// The IC tool program: every rule version's candidate manifests and sealed claims.
fn collect_ic_claims(repo_root: &Path, rows: &mut Vec<Value>) {
    let root = repo_root.join(IC_TOOL_RULES);
    let Ok(rd) = std::fs::read_dir(&root) else { return };
    let mut versions: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
    versions.sort();
    for v in versions {
        let base = root.join(&v);
        for p in files_under(&base.join("manifests").join("candidates"), &|n| n.ends_with(".json")) {
            if let Some(d) = read_doc(&p) {
                rows.push(json!({ "kind": "ic_candidate", "rule_version": v, "path": rel(repo_root, &p),
                                  "candidate_id": d["candidate_id"], "record_sha256": d["record_sha256"],
                                  "curve": d["record"]["curve"], "factor_base": d["record"]["factor_base"] }));
            }
        }
        for p in files_under(&base.join("claims"), &|n| n.ends_with(".claim.json")) {
            let Some(d) = read_doc(&p) else { continue };
            let file = p.file_name().unwrap_or_default().to_string_lossy().trim_end_matches(".claim.json").to_string();
            let dir = p.parent().and_then(|q| q.file_name()).unwrap_or_default().to_string_lossy().to_string();
            rows.push(json!({ "kind": "ic_claim", "rule_version": v, "row_key": format!("{dir}/{file}"),
                              "path": rel(repo_root, &p), "claim": d["claim"], "validation": d["validation"] }));
        }
    }
}

pub fn collect(repo_root: &Path) -> Result<Vec<Value>> {
    let mut rows = Vec::new();
    collect_ecbench(repo_root, &mut rows);
    collect_catalog(repo_root, &mut rows);
    collect_large_prime(repo_root, &mut rows);
    collect_secg(repo_root, &mut rows);
    collect_archive(repo_root, &mut rows);
    collect_designs(repo_root, &mut rows);
    collect_ic_claims(repo_root, &mut rows);
    Ok(rows)
}

// ------------------------------------------------- import

const FB_SQL: &str = "
INSERT INTO ec.factor_base (scheme, key, digest, digest_verified, curve_id, curve_slug, ec1, curve_name, family, params,
       description, signed_points, abscissae, columns, dimension, effective_columns, strict_points, quotient_rule,
       points_sha256, enumerated_set_sha256, store_path, record, provenance_id)
SELECT r.scheme, r.key, r.digest, r.digest_verified,
       (SELECT CASE WHEN count(*) = 1 THEN min(x.id) END FROM ec.resolve_curve(r.curve_ref) x(id)),
       r.curve_slug, r.ec1, r.curve_name, r.family, COALESCE(r.params, '{}'), r.description, r.signed_points, r.abscissae,
       r.columns, r.dimension, r.effective_columns, r.strict_points, r.quotient_rule, r.points_sha256,
       r.enumerated_set_sha256, r.store_path, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(scheme text, key text, digest text, digest_verified boolean, curve_ref text,
     curve_slug text, ec1 text, curve_name text, family text, params jsonb, description text, signed_points bigint,
     abscissae bigint, columns bigint, dimension int, effective_columns bigint, strict_points bigint, quotient_rule text,
     points_sha256 text, enumerated_set_sha256 text, store_path text, record jsonb)
ON CONFLICT (scheme, key) DO UPDATE SET
  store_path = COALESCE(EXCLUDED.store_path, ec.factor_base.store_path),
  curve_id = COALESCE(ec.factor_base.curve_id, EXCLUDED.curve_id)";

const ALIAS_SQL: &str = "
INSERT INTO ec.factor_base_alias (factor_base_id, kind, value)
SELECT fb.id, r.kind, r.value
FROM jsonb_to_recordset($1::jsonb) AS r(scheme text, key text, kind text, value text)
JOIN ec.factor_base fb ON fb.scheme = r.scheme AND fb.key = r.key
WHERE r.value IS NOT NULL AND r.value <> ''
ON CONFLICT DO NOTHING";

const POINT_SQL: &str = "
INSERT INTO ec.factor_base_point (factor_base_id, idx, x, y, col, coef)
SELECT fb.id, r.idx, r.x, r.y, r.col, r.coef
FROM jsonb_to_recordset($1::jsonb) AS r(key text, idx int, x text, y text, col int, coef text)
JOIN ec.factor_base fb ON fb.scheme = 'fb1' AND fb.key = r.key
ON CONFLICT DO NOTHING";

const FBM_SQL: &str = "
INSERT INTO ec.factor_base_measurement (fbm_id, fbm_sha256, digest_verified, factor_base_id, curve_slug, oracle, solver,
       linalg, instrument, config, provenance_id)
SELECT r.fbm_id, r.fbm_sha256, r.digest_verified, fb.id, r.config->>'curve', r.config->>'oracle', r.config->>'solver',
       r.config->>'linalg', r.config->>'instrument', r.config, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(fbm_id text, fbm_sha256 text, digest_verified boolean, config jsonb)
JOIN ec.factor_base fb ON fb.scheme = 'fb1' AND fb.key = r.config->>'fb_id'
ON CONFLICT (fbm_id) DO NOTHING";

const RUN_SQL: &str = "
UPDATE ec.ecbench_run er SET factor_base_id = fb.id, fb_id = r.fb_id, fbm_id = r.fbm_id
FROM jsonb_to_recordset($1::jsonb) AS r(record_id text, fb_id text, fbm_id text)
JOIN ec.factor_base fb ON fb.scheme = 'fb1' AND fb.key = r.fb_id
WHERE er.record_id = r.record_id AND er.factor_base_id IS DISTINCT FROM fb.id";

const RESULT_SQL: &str = "
INSERT INTO ec.factor_base_result (factor_base_id, source, source_key, arm, seed, status, solved, trials, relations,
       yield_per_trial, yield_log2, matrix_rank, independent_rows, cpu_ns, counters, extra, record, provenance_id)
SELECT fb.id, r.source, r.source_key, r.arm, r.seed, r.status, r.solved, r.trials, r.relations, r.yield_per_trial,
       r.yield_log2, r.matrix_rank, r.independent_rows, r.cpu_ns, r.counters, COALESCE(r.extra, '{}'), r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(key text, source text, source_key text, arm text, seed bigint, status text,
     solved boolean, trials bigint, relations bigint, yield_per_trial double precision, yield_log2 double precision,
     matrix_rank bigint, independent_rows bigint, cpu_ns bigint, counters jsonb, extra jsonb, record jsonb)
JOIN ec.factor_base fb ON fb.scheme = 'spec' AND fb.key = r.key
ON CONFLICT (source, source_key) DO NOTHING";

/// Links that need rows other importers wrote: leaderboard rows by FB1 id,
/// and the PS1 labels of the sibling factor-base records by archive digest.
const DESIGN_SQL: &str = "
INSERT INTO ec.factor_base_design (study, schema, path, curve_models, base_spec_count, solver_axes, solver_radices,
       solver_tuple_count, total_tuple_count, disposition_counts, coverage, outside_grid, objective, runtime_claim_gate,
       record, provenance_id)
SELECT r.study, r.schema, r.path, r.curve_models, r.base_spec_count, r.solver_axes, r.solver_radices, r.solver_tuple_count,
       r.total_tuple_count, r.disposition_counts, r.coverage, r.outside_grid, r.objective, r.runtime_claim_gate, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(study text, schema text, path text, curve_models text[], base_spec_count int,
     solver_axes jsonb, solver_radices int[], solver_tuple_count bigint, total_tuple_count bigint, disposition_counts jsonb,
     coverage text, outside_grid text, objective jsonb, runtime_claim_gate text, record jsonb)
ON CONFLICT (study) DO NOTHING";

const MEMBER_SQL: &str = "
INSERT INTO ec.factor_base_design_member (design_id, ordinal, factor_base_id, spec)
SELECT d.id, r.ordinal, fb.id, r.spec
FROM jsonb_to_recordset($1::jsonb) AS r(study text, ordinal int, key text, spec jsonb)
JOIN ec.factor_base_design d ON d.study = r.study
JOIN ec.factor_base fb ON fb.scheme = 'spec' AND fb.key = r.key
ON CONFLICT DO NOTHING";

const CLAIM_SQL: &str = "
INSERT INTO ec.ic_claim (program, rule_version, row_key, run_id, candidate_id, candidate_manifest_sha256, factor_base_id,
       curve_id, curve_slug, ec1, n, a, log2_r, workload_id, target_index, round, timing_class, record_class,
       ic_online_wall_ms, rho_online_wall_ms, online_speedup, s_ic_online, s_rho_online, s_setup, s_ic_cold, s_rho_cold,
       cold_ratio_ic_over_rho, ic_online_operations,
       rho_online_operations, ic_scalar_verified, rho_scalar_verified, verdict, validation_status, independent_validation,
       ic_online_phase_ms, rho_online_phase_ms, units, claim_boundary, report_path, source_path, record, provenance_id)
SELECT 'ic_tool_program', r.rule_version, r.row_key, r.run_id, r.candidate_id, r.candidate_manifest_sha256,
       (SELECT fb.id FROM ec.factor_base fb WHERE fb.scheme = 'pointset' AND fb.key = r.fb_key),
       (SELECT c.id FROM ec.curve c WHERE c.icv1_slug = r.curve_slug),
       r.curve_slug, r.ec1, r.n, r.a, r.log2_r, r.workload_id, r.target_index, r.round, r.timing_class, r.record_class,
       r.ic_online_wall_ms, r.rho_online_wall_ms, r.online_speedup, r.s_ic_online, r.s_rho_online, r.s_setup,
       r.s_ic_cold, r.s_rho_cold, r.cold_ratio_ic_over_rho,
       r.ic_online_operations, r.rho_online_operations, r.ic_scalar_verified, r.rho_scalar_verified, r.verdict,
       r.validation_status, r.independent_validation, r.ic_online_phase_ms, r.rho_online_phase_ms, r.units,
       r.claim_boundary, r.report_path, r.source_path, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(rule_version text, row_key text, run_id text, candidate_id text,
     candidate_manifest_sha256 text, fb_key text, curve_slug text, ec1 text, n int, a int, log2_r double precision,
     workload_id text, target_index int, round int, timing_class text, record_class text,
     ic_online_wall_ms double precision, rho_online_wall_ms double precision, online_speedup double precision,
     s_ic_online double precision, s_rho_online double precision, s_setup double precision, s_ic_cold double precision,
     s_rho_cold double precision, cold_ratio_ic_over_rho double precision, ic_online_operations bigint,
     rho_online_operations bigint, ic_scalar_verified boolean, rho_scalar_verified boolean, verdict text,
     validation_status text, independent_validation boolean, ic_online_phase_ms jsonb, rho_online_phase_ms jsonb,
     units jsonb, claim_boundary text, report_path text, source_path text, record jsonb)
ON CONFLICT (program, rule_version, row_key) DO NOTHING";

const LINK_SQL: &str = "
UPDATE ec.ic_board_row b SET factor_base_id = fb.id
  FROM ec.factor_base fb
 WHERE fb.scheme = 'fb1' AND fb.key = b.factor_base->>'fb_id' AND b.factor_base_id IS DISTINCT FROM fb.id;
INSERT INTO ec.factor_base_alias (factor_base_id, kind, value)
SELECT DISTINCT fb.id, 'ps1_label', e.record->>'ps1_label'
  FROM ec.external_curve_record e
  JOIN ec.factor_base fb ON fb.scheme = 'fbarchive' AND fb.key = e.record->>'factor_base_sha256'
 WHERE e.kind = 'factor_base' AND e.record->>'ps1_label' IS NOT NULL
ON CONFLICT DO NOTHING";

fn int(v: &Value) -> Value {
    match v {
        Value::Number(n) if n.as_i64().is_some() => v.clone(),
        Value::String(s) => s.parse::<i64>().map(|n| json!(n)).unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn fb1_row(curve_slug: &str, ec1: &Value, facts: &Value, store_path: Option<String>, report: &mut Report) -> Value {
    let verified = verify_fb1(curve_slug, facts);
    if verified == Some(false) {
        report
            .problem(format!("{}: FB1 id does not match its preimage (loaded under the recorded id)", facts["fb_id"]));
    }
    json!({
        "scheme": "fb1", "key": facts["fb_id"], "digest": facts["fb_sha256"], "digest_verified": verified,
        "curve_ref": curve_slug, "curve_slug": curve_slug, "ec1": ec1, "family": facts["family"],
        "params": facts["params"], "description": facts["description"], "signed_points": int(&facts["signed_points"]),
        "abscissae": int(&facts["abscissae"]), "columns": int(&facts["columns"]), "dimension": int(&facts["dimension"]),
        "points_sha256": facts["points_sha256"], "store_path": store_path, "record": facts,
    })
}

/// Target index and round of a tool-program claim row (`k0n41/T07-R2` -> (7, 2)).
fn target_round(row_key: &str) -> (Option<i64>, Option<i64>) {
    let file = row_key.rsplit('/').next().unwrap_or_default();
    let mut it = file.split('-');
    let num = |x: Option<&str>, p: char| x.and_then(|x| x.strip_prefix(p)).and_then(|x| x.parse::<i64>().ok());
    (num(it.next(), 'T'), num(it.next(), 'R'))
}

pub async fn import(ctx: &Ctx<'_>) -> Result<Report> {
    let (rows, sha) = pack::load_or(ctx, PACK, || collect(&ctx.repo_root))?;
    let mut report = Report {
        input: format!(
            "ecbench records (factor_base, fbm_id), {CATALOG_DIR}/, {LARGE_PRIME_DIR}/, {SECG_DIR}/, cryptanalysis:{ARCHIVE_DIR}/"
        ),
        sha256: sha.clone(),
        ..Default::default()
    };
    let src = ctx.source(&format!("{}/{PACK}.jsonl", pack::DIR), &sha, "factor bases of the IC work").await?;
    let prov = ctx.provenance(src, "factor-base import (FB1/FBM1 recomputed)", None).await?;

    let (mut fbs, mut aliases, mut points, mut fbms, mut runs, mut results) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut specs: BTreeMap<String, Value> = BTreeMap::new();
    let (mut designs, mut members, mut candidates, mut claim_rows) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    let mut fb_verified = [0u64; 3];
    for r in &rows {
        match r["kind"].as_str().unwrap_or_default() {
            "fb1" => {
                let slug = r["curve_slug"].as_str().unwrap_or_default();
                let row = fb1_row(slug, &r["ec1"], &r["facts"], None, &mut report);
                fb_verified[match row["digest_verified"].as_bool() {
                    Some(true) => 0,
                    Some(false) => 1,
                    None => 2,
                }] += 1;
                fbs.push(row);
                if let Some(p) = r["plug_in"].as_str() {
                    aliases.push(json!({ "scheme": "fb1", "key": r["facts"]["fb_id"], "kind": "plug_in", "value": p }));
                }
            }
            "fbm" => {
                let (id, full) = short_id("FBM1", &r["config"])?;
                let ok = Some(id.as_str()) == r["fbm_id"].as_str();
                if !ok {
                    report.problem(format!("{}: FBM1 id does not match the rebuilt configuration", r["fbm_id"]));
                }
                report.count(if ok { "fbm_verified" } else { "fbm_unverified" }, 1);
                fbms.push(json!({ "fbm_id": r["fbm_id"], "fbm_sha256": if ok { json!(full) } else { Value::Null },
                                  "digest_verified": ok, "config": r["config"] }));
            }
            "run" => runs.push(r.clone()),
            "catalog" => {
                let e = &r["entry"];
                for (kind, v) in [("plug_in", &e["plug_in"]), ("catalog_id", &e["id"]), ("store_path", &e["store"])] {
                    aliases.push(json!({ "scheme": "fb1", "key": e["fb_id"], "kind": kind, "value": v }));
                }
            }
            "dump" => {
                let slug = r["curve"]["slug"].as_str().unwrap_or_default();
                let fb_id = r["facts"]["fb_id"].clone();
                let store = format!("store:factor-bases/{slug}/{}.json", fb_id.as_str().unwrap_or_default());
                fbs.push(fb1_row(slug, &r["curve"]["ec1"], &r["facts"], Some(store), &mut report));
                for (i, p) in r["points"].as_array().into_iter().flatten().enumerate() {
                    points.push(json!({ "key": fb_id, "idx": i, "x": super::s(&p["x"]), "y": super::s(&p["y"]),
                                        "col": p["col"], "coef": super::s(&p["coef"]) }));
                }
            }
            "result" => {
                let curve = r["curve"].as_str().unwrap_or_default();
                let family = r["family"].as_str().unwrap_or_default();
                let (key, full) = spec_id(curve, family, &r["params"])?;
                let fb = &r["fb"];
                specs.entry(key.clone()).or_insert_with(|| {
                    json!({ "scheme": "spec", "key": key, "digest": full, "digest_verified": Value::Null,
                            "curve_ref": curve, "curve_slug": if curve.starts_with("icv1-") { json!(curve) } else { Value::Null },
                            "curve_name": curve, "family": family, "params": r["params"], "description": fb["description"],
                            "signed_points": int(&fb["signed_points"]), "abscissae": int(&fb["abscissae"]),
                            "columns": int(&fb["columns"]),
                            "record": { "schema": SPEC_SCHEMA, "curve": curve, "family": family, "params": r["params"] } })
                });
                let mut row = r.clone();
                row["key"] = json!(key);
                row["seed"] = int(&r["seed"]);
                row["record"] = json!({ "source_key": r["source_key"], "fb": r["fb"] });
                results.push(row);
            }
            "archive" => {
                let a = &r["row"];
                let g = |k: &str| a[k].as_str().filter(|s| !s.is_empty()).map(String::from);
                let key = g("factor_base_sha256");
                let mut params = Map::new();
                for k in ["n", "l", "seed"] {
                    if let Some(v) = g(k) {
                        params.insert(k.into(), json!(v));
                    }
                }
                fbs.push(json!({
                    "scheme": "fbarchive", "key": key, "digest": key, "digest_verified": Value::Null,
                    "curve_ref": g("curve_id"), "ec1": g("curve_id"), "family": g("family").unwrap_or_else(|| "unknown".into()),
                    "params": params, "signed_points": int(&json!(g("fb_points"))),
                    "effective_columns": int(&json!(g("effective_columns"))), "columns": int(&json!(g("effective_columns"))),
                    "strict_points": int(&json!(g("strict_points"))), "quotient_rule": g("quotient_rule"),
                    "enumerated_set_sha256": g("enumerated_set_sha256"),
                    "store_path": g("path").map(|p| format!("cryptanalysis:{ARCHIVE_DIR}/{p}")), "record": a,
                }));
                if let Some(f) = g("file_sha256") {
                    aliases.push(json!({ "scheme": "fbarchive", "key": key, "kind": "file_sha256", "value": f }));
                }
            }
            "archive_alias" => {
                let a = &r["row"];
                aliases.push(json!({ "scheme": "fbarchive", "key": a["factor_base_sha256"], "kind": "archive_alias",
                                     "value": a["cited_sha256"] }));
            }
            "design" => {
                let d = &r["doc"];
                let study = d["study"].as_str().unwrap_or_default().to_string();
                let models: Vec<String> = d["curve_models"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(|m| m.as_str().map(String::from))
                    .collect();
                for (ordinal, spec) in d["base_specs"].as_array().into_iter().flatten().enumerate() {
                    let Some(curve) = spec["a"].as_u64().and_then(|a| models.get(a as usize)) else {
                        report.problem(format!("{study} base spec {ordinal}: no curve model for a = {}", spec["a"]));
                        continue;
                    };
                    let family = spec["family"].as_str().unwrap_or_default();
                    let params = json!({ "closure": spec["closure"], "parameter": super::s(&spec["parameter"]),
                                         "parameter_unit": spec["parameter_unit"], "seed": super::s(&spec["seed"]) });
                    let (key, full) = spec_id(curve, family, &params)?;
                    let size = int(&spec["parameter"]);
                    let unit = spec["parameter_unit"].as_str().unwrap_or_default();
                    specs.entry(key.clone()).or_insert_with(|| {
                        json!({ "scheme": "spec", "key": key, "digest": full, "digest_verified": Value::Null,
                                "curve_ref": curve, "curve_slug": curve, "curve_name": curve, "family": family, "params": params,
                                "description": format!("{family}, {} {unit}, closure {} (design grid {study}, not run as such)",
                                                       spec["parameter"], spec["closure"].as_str().unwrap_or("?")),
                                "columns": if unit == "orbit_columns" { size.clone() } else { Value::Null },
                                "dimension": if unit == "binary_dimension" { size } else { Value::Null },
                                "record": { "schema": SPEC_SCHEMA, "curve": curve, "family": family, "params": params } })
                    });
                    members.push(json!({ "study": study, "ordinal": ordinal, "key": key, "spec": spec }));
                }
                let mut record = d.clone();
                if let Some(o) = record.as_object_mut() {
                    o.remove("base_specs");
                }
                designs.push(json!({
                    "study": study, "schema": d["schema"], "path": r["path"], "curve_models": models,
                    "base_spec_count": d["base_spec_count"], "solver_axes": d["solver_axes"],
                    "solver_radices": d["solver_radices"], "solver_tuple_count": d["solver_tuple_count"],
                    "total_tuple_count": d["total_tuple_count"], "disposition_counts": d["disposition_counts"],
                    "coverage": d["coverage"], "outside_grid": d["outside_grid"], "objective": d["objective"],
                    "runtime_claim_gate": d["runtime_claim_gate"], "record": record,
                }));
            }
            "ic_candidate" => candidates.push(r.clone()),
            "ic_claim" => claim_rows.push(r.clone()),
            other => report.problem(format!("unknown pack row kind {other:?}")),
        }
    }
    fbs.extend(specs.into_values());
    // IC tool-program candidates: a factor base per usable point set, named
    // on the curve slug its claims use (the manifest's EC1 is the kb form).
    let mut slug_of: BTreeMap<String, String> = BTreeMap::new();
    for c in &claim_rows {
        if let (Some(id), Some(slug)) = (c["claim"]["candidate_id"].as_str(), c["claim"]["curve"].as_str()) {
            slug_of.insert(id.to_string(), slug.to_string());
        }
    }
    let mut fb_of: BTreeMap<String, String> = BTreeMap::new();
    for c in &candidates {
        let fbr = &c["factor_base"];
        let inv = &fbr["inventory"];
        let (Some(id), Some(set)) = (c["candidate_id"].as_str(), inv["usable_set_sha256"].as_str()) else {
            report.problem(format!("{}: candidate without a usable_set_sha256", c["path"]));
            continue;
        };
        fb_of.insert(id.to_string(), set.to_string());
        let slug = slug_of.get(id).cloned();
        let construction = &fbr["construction"];
        let mut params = Map::new();
        for (k, v) in construction.as_object().into_iter().flatten().filter(|(k, _)| *k != "kind") {
            params.insert(k.clone(), json!(super::s(v).unwrap_or_else(|| v.to_string())));
        }
        fbs.push(json!({
            "scheme": "pointset", "key": set, "digest": set, "digest_verified": Value::Null,
            "curve_ref": slug, "curve_slug": slug, "ec1": c["curve"]["curve_id"],
            "family": construction["kind"].as_str().unwrap_or("unknown"), "params": params,
            "description": format!("{} usable points, {} columns, quotient {}", inv["usable_point_count"],
                                   inv["effective_columns"], inv["quotient"].as_str().unwrap_or("?")),
            "signed_points": int(&inv["usable_point_count"]), "columns": int(&inv["effective_columns"]),
            "quotient_rule": inv["quotient"], "record": fbr,
        }));
        aliases.push(json!({ "scheme": "pointset", "key": set, "kind": "ic1_candidate", "value": id }));
        aliases.push(json!({ "scheme": "pointset", "key": set, "kind": "geometric_set_sha256", "value": inv["geometric_set_sha256"] }));
    }
    let f = |v: &Value| if v.is_number() { v.clone() } else { Value::Null };
    let claims: Vec<Value> = claim_rows
        .iter()
        .map(|c| {
            let k = &c["claim"];
            let (dir_t, round) = target_round(c["row_key"].as_str().unwrap_or_default());
            let ops = &k["operation_accounting"];
            json!({
                "rule_version": c["rule_version"], "row_key": c["row_key"], "run_id": k["run_id"],
                "candidate_id": k["candidate_id"], "candidate_manifest_sha256": k["candidate_manifest_sha256"],
                "fb_key": k["candidate_id"].as_str().and_then(|id| fb_of.get(id)),
                "curve_slug": k["curve"], "ec1": k["curve_id"], "n": int(&k["n"]), "a": int(&k["a"]),
                "log2_r": f(&k["log2_r"]), "workload_id": k["workload_id"], "target_index": dir_t, "round": round,
                "timing_class": k["timing_class"], "record_class": k["record_class"],
                "ic_online_wall_ms": f(&k["ic_online_wall_ms"]), "rho_online_wall_ms": f(&k["rho_online_wall_ms"]),
                "online_speedup": f(&k["online_speedup"]), "s_ic_online": f(&k["units"]["s_ic_online"]),
                "s_rho_online": f(&k["units"]["s_rho_online"]), "s_setup": f(&k["units"]["s_setup"]),
                "s_ic_cold": f(&k["units"]["s_ic_cold"]), "s_rho_cold": f(&k["units"]["s_rho_cold"]),
                "cold_ratio_ic_over_rho": f(&k["units"]["cold_ratio_ic_over_rho"]),
                "ic_online_operations": int(&ops["ic_online_operations"]),
                "rho_online_operations": int(&ops["rho_online_operations"]),
                "ic_scalar_verified": k["ic_scalar_verified"].as_bool(), "rho_scalar_verified": k["rho_scalar_verified"].as_bool(),
                "verdict": k["verdict"], "validation_status": c["validation"]["status"],
                "independent_validation": k["independent_validation"].as_bool(),
                "ic_online_phase_ms": k["ic_online_phase_ms"], "rho_online_phase_ms": k["rho_online_phase_ms"],
                "units": k["units"], "claim_boundary": k["claim_boundary"], "report_path": k["report"],
                "source_path": c["path"], "record": { "claim": k, "validation": c["validation"] },
            })
        })
        .collect();
    // Duplicate keys in one statement would make ON CONFLICT DO UPDATE fail: keep the first of each.
    let mut seen = std::collections::BTreeSet::new();
    fbs.retain(|f| seen.insert((f["scheme"].to_string(), f["key"].to_string())));
    report.count("fb1_digest_verified", fb_verified[0]);
    report.count("fb1_digest_mismatch", fb_verified[1]);
    report.count("fb1_digest_unchecked", fb_verified[2]);
    report.count("factor_bases", bulk(ctx.client, FB_SQL, &fbs, prov).await?);
    report.count("aliases", bulk1(ctx.client, ALIAS_SQL, &aliases).await?);
    report.count("points", bulk1(ctx.client, POINT_SQL, &points).await?);
    report.count("measurements", bulk(ctx.client, FBM_SQL, &fbms, prov).await?);
    report.count("ecbench_runs_linked", bulk1(ctx.client, RUN_SQL, &runs).await?);
    report.count("ecbench_runs_with_fb1", runs.len() as u64);
    report.count("results", bulk(ctx.client, RESULT_SQL, &results, prov).await?);
    report.count("designs", bulk(ctx.client, DESIGN_SQL, &designs, prov).await?);
    report.count("design_members", bulk1(ctx.client, MEMBER_SQL, &members).await?);
    report.count("ic_claims", bulk(ctx.client, CLAIM_SQL, &claims, prov).await?);
    report.count("ic_claims_read", claims.len() as u64);
    report.count("ic_candidates_read", candidates.len() as u64);
    ctx.client.batch_execute(LINK_SQL).await?;
    let linked: i64 = ctx
        .client
        .query_one("SELECT count(*) FROM ec.ic_board_row WHERE factor_base_id IS NOT NULL", &[])
        .await?
        .get(0);
    report.count("leaderboard_rows_linked", linked as u64);
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record from research/bitop_ladder_20261019/sessions/holdout-P16 (FB1 and FBM1 as written by ecbench).
    #[test]
    fn fb1_and_fbm1_recompute() {
        let facts = json!({
            "fb_id": "FB1h8e805e4ad71c",
            "fb_sha256": "8e805e4ad71c14c36b5f41b478c9cf420496aa9767aadfd4e19e62060ddcf437",
            "family": "prime-abscissa", "params": {"size": "26"}, "description": "smallest 26 abscissae",
            "signed_points": 52, "abscissae": 26, "columns": 26, "dimension": null,
            "points_sha256": "8064712ddf5dd63020b0e486e25585b857890c5ddc100dd4a9747eaa0f74387d"
        });
        let slug = "icv1-fp16-t73-6dc97f36";
        assert_eq!(verify_fb1(slug, &facts), Some(true));
        let params = json!({"factor_base": "prime-abscissa:size=26", "linalg": "incremental-gauss",
                            "oracle": "mitm:negation_folded=1", "solver": ""});
        let (fbm, _) = short_id("FBM1", &fbm_config(slug, &facts, &params)).unwrap();
        assert_eq!(fbm, "FBM1hba26fedb694e");
        let mut wrong = facts.clone();
        wrong["columns"] = json!(27);
        assert_eq!(verify_fb1(slug, &wrong), Some(false));
        assert_eq!(verify_fb1(slug, &json!({"fb_id": "FB1hx"})), None);
    }

    #[test]
    fn canonical_matches_python_dumps() {
        let v = json!({"b": [1, "é\u{7f}"], "a": {"y": null, "x": true}});
        assert_eq!(canonical(&v).unwrap(), "{\"a\":{\"x\":true,\"y\":null},\"b\":[1,\"\\u00e9\\u007f\"]}");
        assert!(canonical(&json!({"f": 1.5})).is_err());
    }

    #[test]
    fn claim_rows_name_target_and_round() {
        assert_eq!(target_round("k0n41/T07-R2"), (Some(7), Some(2)));
        assert_eq!(target_round("summary"), (None, None));
    }

    #[test]
    fn spec_ids_depend_on_the_construction() {
        let (a, _) = spec_id("secp112r1", "small-x", &json!({"size": "64"})).unwrap();
        let (b, _) = spec_id("secp112r1", "small-x", &json!({"size": "128"})).unwrap();
        assert!(a.starts_with("FBS1h") && a.len() == 17 && a != b);
    }
}
