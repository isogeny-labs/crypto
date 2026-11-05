//! Measured results of this repository, as their producers froze them:
//! the IC leaderboard (`docs/ic/leaderboard.json`), the progress timeline
//! (`docs/ic/progress-timeline.json`), the boundary ledger
//! (`docs/ic/boundary_targets.json`) and the `benchmarks/` index. Rows are
//! upserted: a regenerated leaderboard replaces the previous snapshot of the
//! same (family, regime, curve). Nothing is re-ranked or recomputed here.

use super::{bulk, pack, Ctx, Report};
use anyhow::{Context, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

pub const LEADERBOARD: &str = "docs/ic/leaderboard.json";
pub const PROGRESS: &str = "docs/ic/progress-timeline.json";
pub const BOUNDARY: &str = "docs/ic/boundary_targets.json";
pub const BENCHMARKS: &str = "benchmarks";

const FAMILY_SQL: &str = "
INSERT INTO ec.ic_family (code, title, section, status, targets, unit, unit_how, reference, floor, unit_short, ref_short,
                          regimes, record, provenance_id)
SELECT r.code, r.title, r.section, r.status, r.targets, r.unit, r.unit_how, r.reference, r.floor, r.unit_short,
       r.ref_short, r.regimes, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(code text, title text, section text, status text, targets text, unit text,
     unit_how text, reference text, floor text, unit_short text, ref_short text, regimes text[], record jsonb)
ON CONFLICT (code) DO UPDATE SET title = EXCLUDED.title, section = EXCLUDED.section, status = EXCLUDED.status,
  targets = EXCLUDED.targets, unit = EXCLUDED.unit, unit_how = EXCLUDED.unit_how, reference = EXCLUDED.reference,
  floor = EXCLUDED.floor, unit_short = EXCLUDED.unit_short, ref_short = EXCLUDED.ref_short,
  regimes = EXCLUDED.regimes, record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

const BOARD_SQL: &str = "
INSERT INTO ec.ic_board_row (family, regime, curve_slug, curve_id, ec1, legacy_name, log2_r, cofactor, reference,
       reference_s, reference_mean_total_gae, floor_s, targets, operation_unit, round, class, best_variant, best_label,
       best_m, best_base_points, best_s, best_ratio_rho, best_ratio_rho_ci, best_ratio_floor, best_mean_total_gae,
       validation_status, validation, factor_base, curve_record, source_path, record, provenance_id)
SELECT r.family, r.regime, r.slug, (SELECT c.id FROM ec.curve c WHERE c.icv1_slug = r.slug), r.ec1, r.legacy,
       r.log2_r, r.cofactor, r.reference, r.reference_s, r.reference_mean_total_gae, r.floor_s, r.targets,
       r.operation_unit, r.round, r.class, r.best_variant, r.best_label, r.best_m, r.best_base_points, r.best_s,
       r.best_ratio_rho, r.best_ratio_rho_ci, r.best_ratio_floor, r.best_mean_total_gae, r.validation_status,
       r.validation, r.factor_base, r.curve_record, r.source, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(family text, regime text, slug text, ec1 text, legacy text,
     log2_r double precision, cofactor numeric, reference text, reference_s double precision,
     reference_mean_total_gae double precision, floor_s double precision, targets text, operation_unit text,
     round text, class text, best_variant text, best_label text, best_m int, best_base_points bigint,
     best_s double precision, best_ratio_rho double precision, best_ratio_rho_ci double precision[],
     best_ratio_floor double precision, best_mean_total_gae double precision, validation_status text,
     validation jsonb, factor_base jsonb, curve_record jsonb, source text, record jsonb)
ON CONFLICT (family, regime, curve_slug) DO UPDATE SET curve_id = EXCLUDED.curve_id, ec1 = EXCLUDED.ec1,
  legacy_name = EXCLUDED.legacy_name, log2_r = EXCLUDED.log2_r, cofactor = EXCLUDED.cofactor,
  reference = EXCLUDED.reference, reference_s = EXCLUDED.reference_s,
  reference_mean_total_gae = EXCLUDED.reference_mean_total_gae, floor_s = EXCLUDED.floor_s,
  targets = EXCLUDED.targets, operation_unit = EXCLUDED.operation_unit, round = EXCLUDED.round,
  class = EXCLUDED.class, best_variant = EXCLUDED.best_variant, best_label = EXCLUDED.best_label,
  best_m = EXCLUDED.best_m, best_base_points = EXCLUDED.best_base_points, best_s = EXCLUDED.best_s,
  best_ratio_rho = EXCLUDED.best_ratio_rho, best_ratio_rho_ci = EXCLUDED.best_ratio_rho_ci,
  best_ratio_floor = EXCLUDED.best_ratio_floor, best_mean_total_gae = EXCLUDED.best_mean_total_gae,
  validation_status = EXCLUDED.validation_status, validation = EXCLUDED.validation,
  factor_base = EXCLUDED.factor_base, curve_record = EXCLUDED.curve_record, source_path = EXCLUDED.source_path,
  record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

const RECIPE_SQL: &str = "
INSERT INTO ec.ic_recipe (board_row_id, variant, label, m, base_points, s, s_online, s_setup, ratio_rho, ratio_rho_ci,
       ratio_floor, online_speedup, verified, repetitions, mean_total_gae, phases_s, is_best, record)
SELECT b.id, r.variant, r.label, r.m, r.base_points, r.s, r.s_online, r.s_setup, r.ratio_rho, r.ratio_rho_ci,
       r.ratio_floor, r.online_speedup, r.verified, r.repetitions, r.mean_total_gae, r.phases_s, r.is_best, r.record
FROM jsonb_to_recordset($1::jsonb) AS r(family text, regime text, slug text, variant text, label text, m int,
     base_points bigint, s double precision, s_online double precision, s_setup double precision,
     ratio_rho double precision, ratio_rho_ci double precision[], ratio_floor double precision,
     online_speedup double precision, verified boolean, repetitions int, mean_total_gae double precision,
     phases_s jsonb, is_best boolean, record jsonb)
JOIN ec.ic_board_row b ON b.family = r.family AND b.regime = r.regime AND b.curve_slug = r.slug
WHERE $2::bigint IS NOT NULL
ON CONFLICT (board_row_id, variant) DO UPDATE SET label = EXCLUDED.label, m = EXCLUDED.m,
  base_points = EXCLUDED.base_points, s = EXCLUDED.s, s_online = EXCLUDED.s_online, s_setup = EXCLUDED.s_setup,
  ratio_rho = EXCLUDED.ratio_rho, ratio_rho_ci = EXCLUDED.ratio_rho_ci, ratio_floor = EXCLUDED.ratio_floor,
  online_speedup = EXCLUDED.online_speedup, verified = EXCLUDED.verified, repetitions = EXCLUDED.repetitions,
  mean_total_gae = EXCLUDED.mean_total_gae, phases_s = EXCLUDED.phases_s, is_best = EXCLUDED.is_best,
  record = EXCLUDED.record";

const SECTION_SQL: &str = "
INSERT INTO ec.ic_leaderboard_section (key, record, provenance_id)
SELECT r.key, r.record, $2::bigint FROM jsonb_to_recordset($1::jsonb) AS r(key text, record jsonb)
ON CONFLICT (key) DO UPDATE SET record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

const SERIES_SQL: &str = "
INSERT INTO ec.progress_series (id, label, note, barrier_test, record, provenance_id)
SELECT r.id, r.label, r.note, r.barrier_test, r.record, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(id text, label text, note text, barrier_test boolean, record jsonb)
ON CONFLICT (id) DO UPDATE SET label = EXCLUDED.label, note = EXCLUDED.note, barrier_test = EXCLUDED.barrier_test,
  record = EXCLUDED.record, provenance_id = EXCLUDED.provenance_id";

const POINT_SQL: &str = "
INSERT INTO ec.progress_point (series_id, date, label, value, reference, class, detail, source, anchor, record)
SELECT r.series_id, r.date, r.label, r.value, r.reference, r.class, r.detail, r.source, r.anchor, r.record
FROM jsonb_to_recordset($1::jsonb) AS r(series_id text, date date, label text, value double precision,
     reference text, class text, detail text, source text, anchor text, record jsonb)
WHERE $2::bigint IS NOT NULL
ON CONFLICT (series_id, date, label) DO UPDATE SET value = EXCLUDED.value, reference = EXCLUDED.reference,
  class = EXCLUDED.class, detail = EXCLUDED.detail, source = EXCLUDED.source, anchor = EXCLUDED.anchor,
  record = EXCLUDED.record";

const BOUNDARY_SQL: &str = "
INSERT INTO ec.boundary_record (regime, stage, status, summary, metrics, next_target, evidence, history, regime_record,
       provenance_id)
SELECT r.regime, r.stage, r.status, r.summary, r.metrics, r.next_target, r.evidence, r.history, r.regime_record,
       $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(regime text, stage text, status text, summary text, metrics jsonb,
     next_target jsonb, evidence text[], history jsonb, regime_record jsonb)
ON CONFLICT (regime, stage) DO UPDATE SET status = EXCLUDED.status, summary = EXCLUDED.summary,
  metrics = EXCLUDED.metrics, next_target = EXCLUDED.next_target, evidence = EXCLUDED.evidence,
  history = EXCLUDED.history, regime_record = EXCLUDED.regime_record, provenance_id = EXCLUDED.provenance_id";

const BENCH_SQL: &str = "
INSERT INTO ec.benchmark_entry (dir, date, slug, listed, readme_row, validation, copies, provenance_id)
SELECT r.dir, r.date, r.slug, r.listed, r.readme_row, r.validation, r.copies, $2::bigint
FROM jsonb_to_recordset($1::jsonb) AS r(dir text, date date, slug text, listed text, readme_row text,
     validation jsonb, copies jsonb)
ON CONFLICT (dir) DO UPDATE SET date = EXCLUDED.date, slug = EXCLUDED.slug, listed = EXCLUDED.listed,
  readme_row = EXCLUDED.readme_row, validation = EXCLUDED.validation, copies = EXCLUDED.copies,
  provenance_id = EXCLUDED.provenance_id";

fn f(v: &Value) -> Value {
    match v {
        Value::Number(_) => v.clone(),
        Value::String(s) => s.parse::<f64>().map(|x| json!(x)).unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn i(v: &Value) -> Value {
    match v {
        Value::Number(n) if n.as_i64().is_some() => v.clone(),
        Value::Number(n) => n.as_f64().filter(|x| x.fract() == 0.0).map(|x| json!(x as i64)).unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

fn text(v: &Value) -> Value {
    match v {
        Value::Null => Value::Null,
        Value::String(_) => v.clone(),
        other => Value::String(other.to_string()),
    }
}

fn ci(v: &Value) -> Value {
    match v.as_array() {
        Some(a) if a.len() == 2 && a.iter().all(|x| x.is_number()) => v.clone(),
        _ => Value::Null,
    }
}

fn recipe_row(family: &str, regime: &str, slug: &str, r: &Value, is_best: bool) -> Value {
    json!({
        "family": family, "regime": regime, "slug": slug, "variant": r["variant"], "label": r["label"],
        "m": i(&r["m"]), "base_points": i(&r["base_points"]), "s": f(&r["s"]), "s_online": f(&r["s_online"]),
        "s_setup": f(&r["s_setup"]), "ratio_rho": f(&r["ratio_rho"]), "ratio_rho_ci": ci(&r["ratio_rho_ci"]),
        "ratio_floor": f(&r["ratio_floor"]), "online_speedup": f(&r["online_speedup"]), "verified": r["verified"],
        "repetitions": i(&r["repetitions"]), "mean_total_gae": f(&r["mean_total_gae"]), "phases_s": r["phases_s"],
        "is_best": is_best, "record": r,
    })
}

pub async fn leaderboard(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read(LEADERBOARD)?;
    let mut report = Report::new(&input);
    let src = ctx
        .source(&input.rel_path, &input.sha256, "Index-calculus leaderboard (scripts/build_ic_leaderboard.py)")
        .await?;
    let prov = ctx.provenance(src, "ic leaderboard snapshot", None).await?;
    let doc = &input.doc;

    let mut families = Vec::new();
    for (code, fam) in doc["families"].as_object().into_iter().flatten() {
        families.push(json!({
            "code": code, "title": fam["title"], "section": text(&fam["section"]), "status": text(&fam["status"]),
            "targets": text(&fam["targets"]), "unit": text(&fam["unit"]), "unit_how": text(&fam["unit_how"]),
            "reference": text(&fam["reference"]), "floor": text(&fam["floor"]), "unit_short": text(&fam["unit_short"]),
            "ref_short": text(&fam["ref_short"]), "regimes": fam["regimes"], "record": fam,
        }));
    }
    report.count("families", bulk(ctx.client, FAMILY_SQL, &families, prov).await?);

    let (mut rows, mut recipes) = (Vec::new(), Vec::new());
    for b in doc["board"].as_array().into_iter().flatten() {
        let (Some(family), Some(regime), Some(slug)) = (b["family"].as_str(), b["regime"].as_str(), b["slug"].as_str())
        else {
            report.problem(format!(
                "board row without family/regime/slug: {}",
                b.to_string().chars().take(120).collect::<String>()
            ));
            continue;
        };
        let best = &b["best"];
        rows.push(json!({
            "family": family, "regime": regime, "slug": slug, "ec1": b["ec1"], "legacy": text(&b["legacy"]),
            "log2_r": f(&b["log2_r"]), "cofactor": i(&b["cofactor"]), "reference": text(&b["reference"]),
            "reference_s": f(&b["reference_s"]), "reference_mean_total_gae": f(&b["reference_mean_total_gae"]),
            "floor_s": f(&b["floor_s"]), "targets": text(&b["targets"]), "operation_unit": text(&b["operation_unit"]),
            "round": text(&b["round"]), "class": text(&b["class"]), "best_variant": best["variant"],
            "best_label": best["label"], "best_m": i(&best["m"]), "best_base_points": i(&best["base_points"]),
            "best_s": f(&best["s"]), "best_ratio_rho": f(&best["ratio_rho"]), "best_ratio_rho_ci": ci(&best["ratio_rho_ci"]),
            "best_ratio_floor": f(&best["ratio_floor"]), "best_mean_total_gae": f(&best["mean_total_gae"]),
            "validation_status": b["validation"]["status"], "validation": b["validation"],
            "factor_base": b["factor_base"], "curve_record": b["curve_record"], "source": text(&b["source"]), "record": b,
        }));
        let best_variant = best["variant"].as_str();
        for r in b["recipes"].as_array().into_iter().flatten() {
            if r["variant"].is_string() {
                recipes.push(recipe_row(family, regime, slug, r, r["variant"].as_str() == best_variant));
            }
        }
    }
    report.count("board_rows", bulk(ctx.client, BOARD_SQL, &rows, prov).await?);
    report.count("recipes", bulk(ctx.client, RECIPE_SQL, &recipes, prov).await?);
    let unlinked: i64 =
        ctx.client.query_one("SELECT count(*) FROM ec.ic_board_row WHERE curve_id IS NULL", &[]).await?.get(0);
    if unlinked > 0 {
        report.problem(format!("{unlinked} board rows name a slug with no curve row"));
    }

    let sections: Vec<Value> = doc
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(k, _)| !matches!(k.as_str(), "board" | "families"))
        .map(|(k, v)| json!({ "key": k, "record": v }))
        .collect();
    report.count("sections", bulk(ctx.client, SECTION_SQL, &sections, prov).await?);
    Ok(report)
}

pub async fn progress(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read(PROGRESS)?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "IC progress timeline").await?;
    let prov = ctx.provenance(src, "ic progress timeline snapshot", None).await?;
    let (mut series, mut points) = (Vec::new(), Vec::new());
    for s in input.doc["series"].as_array().into_iter().flatten() {
        let Some(id) = s["id"].as_str() else { continue };
        let mut rec = s.clone();
        if let Some(o) = rec.as_object_mut() {
            o.remove("points");
        }
        series.push(json!({ "id": id, "label": s["label"], "note": text(&s["note"]), "barrier_test": s["barrier_test"], "record": rec }));
        for p in s["points"].as_array().into_iter().flatten() {
            points.push(json!({
                "series_id": id, "date": p["date"], "label": p["label"], "value": f(&p["value"]),
                "reference": text(&p["reference"]), "class": text(&p["class"]), "detail": text(&p["detail"]),
                "source": text(&p["source"]), "anchor": text(&p["anchor"]), "record": p,
            }));
        }
    }
    report.count("series", bulk(ctx.client, SERIES_SQL, &series, prov).await?);
    report.count("points", bulk(ctx.client, POINT_SQL, &points, prov).await?);
    Ok(report)
}

pub async fn boundary(ctx: &Ctx<'_>) -> Result<Report> {
    let input = ctx.read(BOUNDARY)?;
    let mut report = Report::new(&input);
    let src = ctx.source(&input.rel_path, &input.sha256, "IC boundary ledger").await?;
    let prov = ctx.provenance(src, "ic boundary ledger snapshot", None).await?;
    let mut rows = Vec::new();
    for (regime, reg) in input.doc["regimes"].as_object().into_iter().flatten() {
        let mut regime_record = reg.clone();
        if let Some(o) = regime_record.as_object_mut() {
            o.remove("records");
        }
        for (stage, r) in reg["records"].as_object().into_iter().flatten() {
            let evidence: Vec<String> = r["evidence"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|e| e.as_str().map(str::to_string).unwrap_or_else(|| e.to_string()))
                .collect();
            rows.push(json!({
                "regime": regime, "stage": stage, "status": text(&r["status"]),
                "summary": text(&r["current"]["summary"]), "metrics": r["current"]["metrics"],
                "next_target": r["next_target"], "evidence": evidence, "history": r["history"],
                "regime_record": regime_record,
            }));
        }
    }
    report.count("records", bulk(ctx.client, BOUNDARY_SQL, &rows, prov).await?);
    Ok(report)
}

/// The benchmarks/ entries, as pack rows: one per directory.
pub fn collect_benchmarks(repo_root: &Path) -> Result<Vec<Value>> {
    let root = repo_root.join(BENCHMARKS);
    let readme = std::fs::read_to_string(root.join("README.md")).context("benchmarks/README.md")?;
    let archive = std::fs::read_to_string(root.join("ARCHIVE.md")).unwrap_or_default();
    let mut rows = Vec::new();
    let mut dirs: Vec<String> = std::fs::read_dir(&root)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    dirs.sort();
    for d in dirs {
        let link = format!("({d}/)");
        let row_in = |text: &str| text.lines().find(|l| l.starts_with('|') && l.contains(&link)).map(str::to_string);
        let (listed, readme_row) = match (row_in(&readme), row_in(&archive)) {
            (Some(r), _) => ("admitted", Some(r)),
            (None, Some(r)) => ("archive", Some(r)),
            (None, None) => ("unlisted", None),
        };
        let load = |name: &str| -> Value {
            std::fs::read(root.join(&d).join(name))
                .ok()
                .and_then(|b| serde_json::from_slice(&b).ok())
                .unwrap_or(Value::Null)
        };
        let date = d
            .get(..8)
            .filter(|s| s.bytes().all(|b| b.is_ascii_digit()))
            .map(|s| format!("{}-{}-{}", &s[..4], &s[4..6], &s[6..8]));
        rows.push(json!({
            "dir": format!("{BENCHMARKS}/{d}"), "date": date, "slug": d.get(9..).unwrap_or(&d), "listed": listed,
            "readme_row": readme_row, "validation": load("VALIDATION.json"), "copies": load("copies.json"),
        }));
    }
    Ok(rows)
}

pub async fn benchmarks(ctx: &Ctx<'_>) -> Result<Report> {
    let (rows, sha) = pack::load_or(ctx, "benchmarks", || collect_benchmarks(&ctx.repo_root))?;
    let mut report = Report {
        input: format!("{BENCHMARKS}/ (README.md, ARCHIVE.md, */VALIDATION.json)"),
        sha256: sha.clone(),
        ..Default::default()
    };
    let src = ctx.source(&format!("{BENCHMARKS}/"), &sha, "benchmarks/ index").await?;
    let prov = ctx.provenance(src, "benchmarks directory index", None).await?;
    let mut by_listed: BTreeMap<String, u64> = BTreeMap::new();
    for r in &rows {
        *by_listed.entry(format!("benchmarks_{}", r["listed"].as_str().unwrap_or("?"))).or_default() += 1;
    }
    report.count("benchmark_entries", bulk(ctx.client, BENCH_SQL, &rows, prov).await?);
    for (k, v) in by_listed {
        report.count(&k, v);
    }
    Ok(report)
}
