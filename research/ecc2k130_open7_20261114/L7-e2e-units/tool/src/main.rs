//! L7: consolidated end-to-end table of complete binary / Koblitz index-calculus
//! rows against matched rho, scaling fits, crossing estimates, phase shares.
//! Reads only committed JSON/JSONL (and `ecbench band` outputs written by
//! `run.sh` under `raw/`). Usage: l7tab <repo-root> <lane-dir>
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const W: [f64; 3] = [0.0, 0.1, 1.0];

#[derive(Clone, Debug, Default)]
struct Row {
    source: String,
    regime: String,
    n: Option<f64>,
    slug: String,
    log2_r: f64,
    arm: String,
    series: String,
    method: String,
    columns: Option<f64>,
    m: Option<f64>,
    relations: Option<f64>,
    trials_per_rel: Option<f64>,
    cost_per_rel: Option<f64>,
    la_cost: Option<f64>,
    s: [Option<f64>; 3],
    s_frozen: [Option<f64>; 3],
    unit: String,
    rho_kind: String,
    rho_s: [Option<f64>; 3],
    band_reading: String,
    validation: String,
    source_path: String,
    large_prime: String,
    runs: Option<f64>,
    verified: Option<f64>,
    phases: Vec<(String, f64)>,
    note: String,
}

impl Row {
    fn ratio(&self, i: usize) -> Option<f64> {
        match (self.s[i], self.rho_s[i]) {
            (Some(a), Some(b)) if b > 0.0 => Some(a / b),
            _ => None,
        }
    }
    fn log2_total(&self, i: usize) -> Option<f64> {
        self.s[i].map(|s| s.log2() + self.log2_r / 2.0)
    }
    fn rho_log2_total(&self, i: usize) -> Option<f64> {
        self.rho_s[i].map(|s| s.log2() + self.log2_r / 2.0)
    }
}

fn rd(p: &Path) -> Value {
    let t = fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    serde_json::from_str(&t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}
fn f(v: &Value) -> Option<f64> {
    v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))
}
fn st(v: &Value) -> String {
    v.as_str().map(str::to_string).unwrap_or_default()
}

/// Band file rows: (curve, arm) -> (mean_s[3], runs, verified, log2_r)
type Band = BTreeMap<(String, String), ([Option<f64>; 3], f64, f64, f64)>;
fn read_band(p: &Path) -> Band {
    let mut out = Band::new();
    if !p.exists() {
        return out;
    }
    let v = rd(p);
    let ws: Vec<f64> = v["weights"].as_array().map(|a| a.iter().filter_map(f).collect()).unwrap_or(W.to_vec());
    for r in v["rows"].as_array().unwrap() {
        let ms = r["mean_s"].as_array().unwrap();
        let mut s = [None; 3];
        for (k, w) in W.iter().enumerate() {
            if let Some(i) = ws.iter().position(|x| (x - w).abs() < 1e-12) {
                s[k] = f(&ms[i]);
            }
        }
        out.insert(
            (st(&r["curve"]), st(&r["arm"])),
            (s, f(&r["runs"]).unwrap_or(0.0), f(&r["verified"]).unwrap_or(0.0), f(&r["log2_r"]).unwrap_or(0.0)),
        );
    }
    out
}

#[derive(Default, Debug, Clone)]
struct Agg {
    runs: f64,
    verified: f64,
    s: f64,
    gae: f64,
    phases: BTreeMap<String, f64>,
    relations: f64,
    trials: f64,
    columns: f64,
    r: f64,
    n: f64,
    slug: String,
    method: String,
    oracle: String,
}

/// Non-warmup records of one session grouped by arm.
fn read_records(p: &Path) -> BTreeMap<String, Agg> {
    let mut out: BTreeMap<String, Agg> = BTreeMap::new();
    let t = fs::read_to_string(p).unwrap_or_default();
    for line in t.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let r: Value = serde_json::from_str(line).unwrap();
        if r["warmup"].as_bool() == Some(true) {
            continue;
        }
        let a = out.entry(st(&r["arm"])).or_default();
        a.runs += 1.0;
        if st(&r["outcome"]["status"]) == "verified" {
            a.verified += 1.0;
        }
        a.s += f(&r["cost"]["s"]).unwrap_or(f64::NAN);
        a.gae += f(&r["cost"]["total_gae"]).unwrap_or(f64::NAN);
        for ph in r["phases"].as_array().into_iter().flatten() {
            *a.phases.entry(st(&ph["name"])).or_default() += f(&ph["gae"]).unwrap_or(0.0);
        }
        a.relations += f(&r["counters"]["relations_found"]).unwrap_or(f64::NAN);
        a.trials += f(&r["counters"]["targets_tried"]).unwrap_or(f64::NAN);
        a.columns = f(&r["factor_base"]["columns"]).unwrap_or(f64::NAN);
        a.r = f(&r["workload"]["curve"]["r"]).unwrap_or(f64::NAN);
        a.n = f(&r["workload"]["curve"]["field_degree"]).unwrap_or(f64::NAN);
        a.slug = st(&r["workload"]["curve"]["slug"]);
        a.method = st(&r["method"]["id"]);
        a.oracle = st(&r["method"]["params"]["oracle"]);
    }
    for a in out.values_mut() {
        let k = a.runs;
        a.s /= k;
        a.gae /= k;
        a.relations /= k;
        a.trials /= k;
        for v in a.phases.values_mut() {
            *v /= k;
        }
    }
    out
}

fn series_of(arm: &str) -> String {
    // strip per-size parameters: -c<digits>, -d<digits>
    arm.split('-')
        .filter(|p| !(p.len() > 1 && (p.starts_with('c') || p.starts_with('d')) && p[1..].chars().all(|c| c.is_ascii_digit())))
        .collect::<Vec<_>>()
        .join("-")
}

fn fill_from_agg(row: &mut Row, a: &Agg) {
    row.n = Some(a.n);
    row.columns = Some(a.columns);
    row.method = format!("{} {}", a.method, a.oracle).trim().to_string();
    if a.relations.is_finite() && a.relations > 0.0 {
        row.relations = Some(a.relations);
        row.trials_per_rel = Some(a.trials / a.relations);
        if let Some(g) = a.phases.get("relations") {
            row.cost_per_rel = Some(g / a.relations);
        }
    }
    row.la_cost = a.phases.get("linear_algebra").copied();
    let tot: f64 = a.phases.values().sum();
    if tot > 0.0 {
        row.phases = a.phases.iter().map(|(k, v)| (k.clone(), v / tot)).collect();
    }
    row.runs = Some(a.runs);
    row.verified = Some(a.verified);
}

fn validation_of(repo: &Path, dir: &str, date: &str) -> String {
    if repo.join(dir).join("VALIDATION.json").exists() {
        "validated (VALIDATION.json valid per scripts/validation_gate.py)".into()
    } else if date < "20261019" {
        "unvalidated (pre-gate)".into()
    } else {
        "unvalidated (post-gate research record)".into()
    }
}

// ---------------------------------------------------------------- sources

/// Record-backed ecbench sources: FC, FO, LZ.
fn src_records(repo: &Path, lane: &Path, rows: &mut Vec<Row>, excl: &mut Vec<Value>, id: &str, dir: &str, sess_glob: &[&str], ic_arms: &dyn Fn(&str) -> bool, frozen_band: Option<&str>, unit: &str) {
    let date = dir.rsplit('_').next().unwrap_or("");
    let frozen = frozen_band.map(|p| read_band(&repo.join(dir).join(p))).unwrap_or_default();
    for s in sess_glob {
        let sdir = format!("{dir}/{s}");
        let aggs = read_records(&repo.join(&sdir).join("records.jsonl"));
        let cur = read_band(&lane.join(format!("raw/band-{id}-{}.json", s.rsplit('/').next().unwrap())));
        let rho = match aggs.get("rho-frob") {
            Some(r) => r.clone(),
            None => continue,
        };
        let rho_cur = cur.get(&(rho.slug.clone(), "rho-frob".into())).map(|x| x.0);
        let rho_fz = frozen.get(&(rho.slug.clone(), "rho-frob".into())).map(|x| x.0);
        for (arm, a) in &aggs {
            if !ic_arms(arm) {
                continue;
            }
            let mut row = Row {
                source: id.into(),
                regime: "koblitz".into(),
                slug: a.slug.clone(),
                log2_r: a.r.log2(),
                arm: arm.clone(),
                series: series_of(arm),
                unit: unit.into(),
                rho_kind: "rho-frob: signed-Frobenius + negation walk (A = 2n), same session".into(),
                validation: validation_of(repo, dir, date),
                source_path: format!("crypto:{sdir}/records.jsonl"),
                large_prime: "none".into(),
                ..Default::default()
            };
            fill_from_agg(&mut row, a);
            let c = cur.get(&(a.slug.clone(), arm.clone()));
            let fz = frozen.get(&(a.slug.clone(), arm.clone()));
            if let Some(fz) = fz {
                row.s_frozen = fz.0;
            }
            match (c, rho_cur) {
                (Some(c), Some(rc)) => {
                    row.s = c.0;
                    row.rho_s = rc;
                    row.band_reading = "ecbench band (this lane's build, current reading: bytes unpriced, table work priced)".into();
                }
                _ => {
                    if let (Some(fz), Some(rf)) = (fz, rho_fz) {
                        row.s = fz.0;
                        row.rho_s = rf;
                        row.band_reading = format!("frozen band file {}", frozen_band.unwrap());
                    } else {
                        row.s = [Some(a.s), None, None];
                        row.rho_s = [Some(rho.s), None, None];
                        row.band_reading = "record cost.s mean (w = 0 only)".into();
                    }
                }
            }
            if a.verified < a.runs {
                excl.push(json!({"source": id, "slug": a.slug, "arm": arm, "reason": format!("{}/{} verified or uncounted unpriced work", a.verified, a.runs)}));
                continue;
            }
            if arm.contains("shared") {
                excl.push(json!({"source": id, "slug": a.slug, "arm": arm, "reason": "uncounted unpriced work (field_arithmetic, hash_and_allocation, modular_combination): unrankable lower bound"}));
                continue;
            }
            rows.push(row);
        }
    }
}

fn src_bandfiles(repo: &Path, rows: &mut Vec<Row>, excl: &mut Vec<Value>, id: &str, dir: &str, files: &[&str], rho_arm: &str, rho_kind: &str, lp_arm: &str) {
    let date = dir.rsplit('_').next().unwrap_or("");
    for fl in files {
        let b = read_band(&repo.join(dir).join(fl));
        let mut curves: Vec<String> = b.keys().map(|k| k.0.clone()).collect();
        curves.dedup();
        for cv in curves {
            let rho = match b.get(&(cv.clone(), rho_arm.to_string())) {
                Some(r) => r.clone(),
                None => continue,
            };
            for ((c, arm), (s, runs, ver, l2r)) in &b {
                if c != &cv || arm.starts_with("rho") || arm.ends_with("-aa") {
                    continue;
                }
                if ver < runs {
                    excl.push(json!({"source": id, "slug": c, "arm": arm, "reason": format!("{ver}/{runs} verified")}));
                    continue;
                }
                let n = c.split('-').nth(1).and_then(|x| x.trim_start_matches("f2m").parse::<f64>().ok());
                rows.push(Row {
                    source: id.into(),
                    regime: "koblitz".into(),
                    n,
                    slug: c.clone(),
                    log2_r: *l2r,
                    arm: arm.clone(),
                    series: arm.clone(),
                    method: arm.clone(),
                    s: *s,
                    unit: "ecbench GAE / sqrt(r)".into(),
                    rho_kind: rho_kind.into(),
                    rho_s: rho.0,
                    band_reading: format!("frozen band file {fl} (reading of its date)"),
                    validation: validation_of(repo, dir, date),
                    source_path: format!("crypto:{dir}/{fl}"),
                    large_prime: if arm == lp_arm { "double large prime (2LP), tuned (K, c)".into() } else { "none".into() },
                    runs: Some(*runs),
                    verified: Some(*ver),
                    note: if s[1].is_none() { "band not computable above w = 0 (uncounted native categories)".into() } else { String::new() },
                    ..Default::default()
                });
            }
        }
    }
}

fn src_px(repo: &Path, rows: &mut Vec<Row>) {
    let dir = "research/ic_planner_exhaustive_20261026";
    let v = rd(&repo.join(dir).join("analysis/summary.json"));
    for (sid, s) in v["sessions"].as_object().unwrap() {
        let regime = if sid.starts_with("K-") { "koblitz" } else if sid.starts_with("B-") { "char2" } else { continue };
        let slug = st(&s["slug"]);
        let l2r = f(&s["log2_r"]).unwrap();
        let get = |arm: &str| -> [Option<f64>; 3] {
            let m = &s["mean_s"][arm];
            [f(&m["0.0"]), f(&m["0.1"]), f(&m["1.0"])]
        };
        let rho = get("rho");
        for (arm, ver) in s["verified"].as_object().unwrap() {
            if arm == "rho" {
                continue;
            }
            let vv = ver.as_array().unwrap();
            let n = slug.split('-').nth(1).and_then(|x| x.trim_start_matches("f2m").parse::<f64>().ok());
            rows.push(Row {
                source: "PX".into(),
                regime: regime.into(),
                n,
                slug: slug.clone(),
                log2_r: l2r,
                arm: arm.clone(),
                series: format!("{regime}:{arm}"),
                method: arm.clone(),
                s: get(arm),
                unit: "ecbench GAE / sqrt(r)".into(),
                rho_kind: if regime == "koblitz" { "rho.signed_frobenius_strong (A = 2n), same session".into() } else { "negation-map rho (A = 2), same session".into() },
                rho_s: rho,
                band_reading: "analysis/summary.json (2026-10-26 reading)".into(),
                validation: "unvalidated (post-gate research record)".into(),
                source_path: format!("crypto:{dir}/analysis/summary.json#sessions.{sid}"),
                large_prime: "none".into(),
                runs: f(&vv[1]),
                verified: f(&vv[0]),
                note: if get(arm)[1].is_none() { "band not computable above w = 0".into() } else { String::new() },
                ..Default::default()
            });
        }
    }
}

fn src_fb(repo: &Path, rows: &mut Vec<Row>) {
    let dir = "research/fes_then_batched_20261112";
    let v = rd(&repo.join(dir).join("results.json"));
    for (i, inst) in v.as_array().unwrap().iter().enumerate() {
        let slug = st(&inst["instance"]);
        let rho = f(&inst["rho_mean_s"]);
        for r in inst["rows"].as_array().unwrap() {
            if r["verified_all"].as_bool() != Some(true) {
                continue;
            }
            let s = f(&r["s_mean"]);
            let tg = f(&r["total_gae"]).unwrap();
            let l2r = 2.0 * (tg / s.unwrap()).log2();
            let n = slug.split('-').nth(1).and_then(|x| x.trim_start_matches("f2m").parse::<f64>().ok());
            let label = st(&r["label"]);
            let short = label.split(" + ").nth(1).unwrap_or("").to_string() + "+" + label.split(" + ").nth(2).unwrap_or("");
            rows.push(Row {
                source: "FB".into(),
                regime: st(&inst["regime"]),
                n,
                slug: slug.clone(),
                log2_r: l2r,
                arm: label.clone(),
                series: format!("FB:{short}"),
                method: label,
                relations: f(&r["matrix_rows"]),
                trials_per_rel: match (f(&r["targets_tried"]), f(&r["matrix_rows"])) { (Some(t), Some(m)) if m > 0.0 => Some(t / m), _ => None },
                s: [s, None, None],
                unit: "ecbench GAE / sqrt(r)".into(),
                rho_kind: st(&inst["rho_method"]),
                rho_s: [rho, None, None],
                band_reading: "results.json mean S (w = 0 only)".into(),
                validation: "validated (VALIDATION.json valid per scripts/validation_gate.py)".into(),
                source_path: format!("crypto:{dir}/results.json#[{i}]"),
                large_prime: "none".into(),
                runs: f(&r["repeats"]),
                verified: f(&r["repeats"]),
                ..Default::default()
            });
        }
    }
}

fn src_lb(repo: &Path, rows: &mut Vec<Row>) {
    let v = rd(&repo.join("docs/ic/leaderboard.json"));
    for (i, b) in v["board"].as_array().unwrap().iter().enumerate() {
        let regime = st(&b["regime"]);
        if regime == "prime" {
            continue;
        }
        let best = &b["best"];
        let s = f(&best["s"]);
        let rho = f(&b["reference_s"]);
        let slug = st(&b["slug"]);
        let n = slug.split('-').nth(1).and_then(|x| x.trim_start_matches("f2m").parse::<f64>().ok());
        let fam = st(&b["family"]);
        let (unit, rk) = match fam.as_str() {
            "A" => ("timed: batched-addition units / sqrt(r), one process (rule v9)", "one-target matched rho, same process, set-up + walk"),
            "B" => ("GAE / sqrt(r) at pinned per-instance calibration", "matched negation-map rho (A = 2), counted GAE"),
            _ => ("timed: batched-addition units / sqrt(r) per target, 32-target batch", "batch rho (k = 32), same unit"),
        };
        let mut phases = vec![];
        if let Some(p) = best["phases_s"].as_object() {
            let tot: f64 = p.values().filter_map(f).sum();
            phases = p.iter().map(|(k, v)| (k.clone(), f(v).unwrap_or(0.0) / tot)).collect();
        }
        let rels = f(&best["relations"]);
        rows.push(Row {
            source: format!("LB-{fam}"),
            regime: regime.clone(),
            n,
            slug,
            log2_r: f(&b["log2_r"]).unwrap(),
            arm: st(&best["variant"]),
            series: format!("LB-{fam}:{}", if fam == "B" { "best" } else { "best" }),
            method: st(&best["label"]),
            columns: f(&best["columns"]),
            m: f(&best["m"]),
            relations: rels,
            trials_per_rel: match (f(&best["trials"]), rels) { (Some(t), Some(r)) if r > 0.0 => Some(t / r), _ => None },
            cost_per_rel: match (best["phases_s"]["relations"].as_f64(), rels) { (Some(g), Some(r)) if r > 0.0 => Some(g * 2f64.powf(f(&b["log2_r"]).unwrap() / 2.0) / r), _ => None },
            la_cost: best["phases_s"]["linear_algebra"].as_f64().map(|x| x * 2f64.powf(f(&b["log2_r"]).unwrap() / 2.0)),
            s: [s, s, s],
            unit: unit.into(),
            rho_kind: rk.into(),
            rho_s: [rho, rho, rho],
            band_reading: if fam == "B" { "pinned calibration (board); no band in board JSON: one figure".into() } else { "timed in one process: nothing unpriced, the band gives one figure".into() },
            validation: st(&b["validation"]["status"]),
            source_path: format!("crypto:docs/ic/leaderboard.json#board[{i}] <- {}", st(&b["source"])),
            large_prime: "none".into(),
            phases,
            note: if fam == "C" { "32 targets solved together, per target; historical diagnostic under the one-target rule".into() } else { String::new() },
            ..Default::default()
        });
    }
}

fn src_gk(repo: &Path, rows: &mut Vec<Row>) -> Value {
    let dir = "research/gaudry_k4_char2_crossover_20261015";
    let v = rd(&repo.join(dir).join("raw/summary.json"));
    let theory = f(&v["rho_theory_s"]).unwrap();
    for t in v["part_e"]["targets"].as_array().unwrap() {
        if st(&t["status"]) != "verified" {
            continue;
        }
        let m = f(&t["m"]).unwrap();
        let rho_meas = f(&t["rho_s"]);
        rows.push(Row {
            source: "GK".into(),
            regime: "char2 k=4 (composite degree 4m, subfield-dependent)".into(),
            n: Some(4.0 * m),
            slug: format!("k4-m{m}-{}", st(&t["workload"])),
            log2_r: f(&t["log2_r"]).unwrap(),
            arm: "ic k=4 (Gaudry-Diem, sparse Wiedemann)".into(),
            series: "GK:k4".into(),
            method: "ic_wide k=4 descent, wiedemann LA".into(),
            m: Some(m),
            relations: f(&t["relations"]),
            trials_per_rel: f(&t["trials_per_relation"]),
            s: [f(&t["ic_s"]), None, None],
            unit: "ecbench GAE / sqrt(r) (fixed field-work prices; unit price unresolved per README)".into(),
            rho_kind: format!("rho THEORY 0.886 sqrt(r) (A = 2); measured negation rho on the same target: {}", rho_meas.map(|x| format!("{x:.3}")).unwrap_or("not finished".into())),
            rho_s: [Some(theory), None, None],
            band_reading: "summary.json (w = 0 only); rho rows are lower bounds (canonicalisation uncharged)".into(),
            validation: "unvalidated (pre-gate); sessions not replayed".into(),
            source_path: format!("crypto:{dir}/raw/summary.json#part_e.targets"),
            large_prime: "none (Part F measures LP separately)".into(),
            runs: Some(1.0),
            verified: Some(1.0),
            phases: vec![("linear_algebra".into(), f(&t["la_share"]).unwrap_or(0.0))],
            note: "one target per row".into(),
            ..Default::default()
        });
    }
    v["part_f"].clone()
}

/// Rule v9 set-up/online timed phase shares per size (validated Table A).
fn v9_phases(repo: &Path) -> Value {
    let base = repo.join("research/ic_tool_program/rule/v9/runs");
    let mut out = Map::new();
    let mut ents: Vec<PathBuf> = fs::read_dir(&base).unwrap().filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.is_dir() && p.file_name().unwrap().to_str().unwrap().starts_with('k')).collect();
    ents.sort();
    for d in ents {
        let mut acc: BTreeMap<String, f64> = BTreeMap::new();
        let mut k = 0.0;
        let mut files: Vec<PathBuf> = fs::read_dir(&d).unwrap().filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.to_str().unwrap().ends_with("-R1.price.json")).collect();
        files.sort();
        let mut collect = (0.0, 0.0, 0.0, 0.0); // relations, trials, la row ops, columns
        for p in &files {
            let v = rd(p);
            if let Some(c) = v["counts"]["pass"].as_object() {
                collect.0 += f(&c["collect"]["relations_collected"]).unwrap_or(0.0);
                collect.1 += f(&c["collect"]["trials"]).unwrap_or(0.0);
                collect.2 += f(&c["logs"]["linear_algebra"]["row_ops"]).unwrap_or(0.0);
                collect.3 += f(&c["logs"]["columns"]).unwrap_or(0.0);
            }
            for rep in v["repetitions"].as_array().into_iter().flatten() {
                k += 1.0;
                for (ph, ns) in rep["setup_phases_ns"].as_object().unwrap() {
                    *acc.entry(format!("setup.{ph}")).or_default() += f(ns).unwrap();
                }
                *acc.entry("online.ic".into()).or_default() += f(&rep["ic_online"]["wall_ns"]).unwrap();
                *acc.entry("rho.setup".into()).or_default() += f(&rep["rho_setup_ns"]).unwrap();
                *acc.entry("rho.online".into()).or_default() += f(&rep["rho_online"]["wall_ns"]).unwrap();
            }
        }
        let ic_tot: f64 = acc.iter().filter(|(k, _)| !k.starts_with("rho.")).map(|(_, v)| v).sum();
        let nf = files.len() as f64;
        let shares: Map<String, Value> = acc.iter().filter(|(k, _)| !k.starts_with("rho.")).map(|(k, v)| (k.clone(), json!(v / ic_tot))).collect();
        out.insert(
            d.file_name().unwrap().to_str().unwrap().into(),
            json!({"files": files.len(), "repetitions": k, "ic_phase_share_of_cold_ns": shares,
                   "ic_cold_ns_mean": ic_tot / k, "rho_cold_ns_mean": (acc["rho.setup"] + acc["rho.online"]) / k,
                   "relations_collected_mean": collect.0 / nf, "collect_trials_mean": collect.1 / nf,
                   "trials_per_relation": collect.1 / collect.0, "la_row_ops_mean": collect.2 / nf, "columns_mean": collect.3 / nf}),
        );
    }
    Value::Object(out)
}

// ---------------------------------------------------------------- statistics

fn tq(df: f64) -> f64 {
    let t = [12.706, 4.303, 3.182, 2.776, 2.571, 2.447, 2.365, 2.306, 2.262, 2.228, 2.201, 2.179, 2.160, 2.145, 2.131];
    if df < 1.0 { f64::NAN } else if (df as usize) <= t.len() { t[df as usize - 1] } else if df <= 20.0 { 2.086 } else if df <= 30.0 { 2.042 } else { 1.96 }
}

#[derive(Clone, Debug)]
struct Fit { a: f64, b: f64, n: usize, s2: f64, sxx: f64, xbar: f64, r2: f64 }
fn ols(x: &[f64], y: &[f64]) -> Option<Fit> {
    let n = x.len();
    if n < 3 { return None; }
    let xbar = x.iter().sum::<f64>() / n as f64;
    let ybar = y.iter().sum::<f64>() / n as f64;
    let sxx: f64 = x.iter().map(|v| (v - xbar).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - xbar) * (b - ybar)).sum();
    let b = sxy / sxx;
    let a = ybar - b * xbar;
    let sse: f64 = x.iter().zip(y).map(|(u, v)| (v - a - b * u).powi(2)).sum();
    let sst: f64 = y.iter().map(|v| (v - ybar).powi(2)).sum();
    Some(Fit { a, b, n, s2: sse / (n as f64 - 2.0), sxx, xbar, r2: 1.0 - sse / sst })
}
impl Fit {
    fn se_b(&self) -> f64 { (self.s2 / self.sxx).sqrt() }
    fn df(&self) -> f64 { self.n as f64 - 2.0 }
    fn pred(&self, x: f64) -> f64 { self.a + self.b * x }
    fn se_mean(&self, x: f64) -> f64 { (self.s2 * (1.0 / self.n as f64 + (x - self.xbar).powi(2) / self.sxx)).sqrt() }
    fn js(&self) -> Value {
        let t = tq(self.df());
        json!({"slope": self.b, "slope_ci95": [self.b - t * self.se_b(), self.b + t * self.se_b()], "intercept": self.a, "points": self.n, "r2": self.r2})
    }
}

struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 { self.0 ^= self.0 << 13; self.0 ^= self.0 >> 7; self.0 ^= self.0 << 17; ((self.0 >> 11) as f64 + 0.5) / (1u64 << 53) as f64 }
    fn z(&mut self) -> f64 { (-2.0 * self.u().ln()).sqrt() * (2.0 * std::f64::consts::PI * self.u()).cos() }
    fn tscale(&mut self, df: f64) -> f64 { let k = df as usize; let c: f64 = (0..k).map(|_| self.z().powi(2)).sum(); (df / c).sqrt() }
    /// (a, b) draw: centred parametrisation is independent: mean at xbar and slope.
    fn draw(&mut self, ft: &Fit) -> (f64, f64) {
        let sc = self.tscale(ft.df());
        let m = ft.pred(ft.xbar) + sc * (ft.s2 / ft.n as f64).sqrt() * self.z();
        let b = ft.b + sc * ft.se_b() * self.z();
        (m - b * ft.xbar, b)
    }
}

fn pct(v: &mut Vec<f64>, p: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if v.is_empty() { return f64::NAN; }
    v[((v.len() - 1) as f64 * p).round() as usize]
}

fn crossing(ic: &Fit, rho: &Fit, xmax: f64, seed: u64) -> Value {
    let pt = if (ic.b - rho.b).abs() > 1e-12 { (rho.a - ic.a) / (ic.b - rho.b) } else { f64::NAN };
    let cur = ic.pred(ic.xbar) > rho.pred(ic.xbar);
    let mut rng = Rng(seed | 1);
    let mut xs = vec![];
    let (mut ahead, mut before) = (0usize, 0usize);
    let nd = 10000;
    for _ in 0..nd {
        let (a1, b1) = rng.draw(ic);
        let (a2, b2) = rng.draw(rho);
        if b1 < b2 {
            let x = (a2 - a1) / (b1 - b2);
            if x > ic.xbar { ahead += 1; xs.push(x); if x <= xmax { before += 1; } }
        }
    }
    let (lo, md, hi) = if xs.len() >= 100 { (pct(&mut xs.clone(), 0.025), pct(&mut xs.clone(), 0.5), pct(&mut xs, 0.975)) } else { (f64::NAN, f64::NAN, f64::NAN) };
    json!({"point_estimate_x": if pt.is_finite() { json!(pt) } else { Value::Null },
           "ic_above_rho_at_data_centre": cur,
           "draws": nd, "share_ic_slope_below_rho_and_crossing_ahead": ahead as f64 / nd as f64,
           "share_crossing_at_or_before_xmax": before as f64 / nd as f64, "xmax": xmax,
           "crossing_given_crossing_median_2.5_97.5": if lo.is_finite() { json!([md, lo, hi]) } else { Value::Null}})
}

fn nz(x: Option<f64>) -> Value { x.filter(|v| v.is_finite()).map(|v| json!(v)).unwrap_or(Value::Null) }
fn c(x: Option<f64>) -> String { x.filter(|v| v.is_finite()).map(|v| format!("{v:.6}")).unwrap_or_default() }

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let repo = PathBuf::from(&a[1]);
    let lane = PathBuf::from(&a[2]);
    let mut rows = vec![];
    let mut excl = vec![];
    let ksess = ["K13", "K17", "K19", "K23", "K37", "K41", "K43", "K47"].map(|s| format!("phase1/sessions/{s}"));
    let ks: Vec<&str> = ksess.iter().map(|s| s.as_str()).collect();
    src_records(&repo, &lane, &mut rows, &mut excl, "FC", "research/ic_frontier_campaign_20261015", &ks, &|a: &str| a.starts_with("ic-"), Some("analysis/band-koblitz.json"), "ecbench GAE / sqrt(r)");
    let osess = ["a0-n13", "a1-n17", "a0-n19", "a0-n23", "a1-n29", "a0-n31", "a0-n37", "a0-n41"].map(|s| format!("sessions/{s}"));
    let os: Vec<&str> = osess.iter().map(|s| s.as_str()).collect();
    src_records(&repo, &lane, &mut rows, &mut excl, "FO", "research/ecbench_frobenius_orbit_vs_rho_20261014", &os, &|a: &str| a.starts_with("ic-"), None, "ecbench GAE / sqrt(r), fully priced at host-measured unit prices (ic.pipeline_full)");
    src_records(&repo, &lane, &mut rows, &mut excl, "LZ", "research/ecbench_lazy_orbit_members_20261018", &os, &|a: &str| a.starts_with("ic-"), None, "ecbench GAE / sqrt(r), fully priced at host-measured unit prices");
    src_bandfiles(&repo, &mut rows, &mut excl, "DT", "research/ic_koblitz_dlp_tuning_20261023", &["bands/holdout-n37.band.json", "bands/holdout-n41.band.json", "bands/holdout-n43.band.json", "bands/holdout-n47.band.json"], "rho-frob", "rho-frob: rho.signed_frobenius (A = 2n), same session", "dlp-selected");
    src_bandfiles(&repo, &mut rows, &mut excl, "DR", "research/ic_koblitz_descent_routing_20261026", &["analysis/band-k47.json", "analysis/band-k41.json", "analysis/band-k53.json"], "rho", "rho.signed_frobenius_strong (A = 2n), same session", "");
    src_px(&repo, &mut rows);
    src_fb(&repo, &mut rows);
    src_lb(&repo, &mut rows);
    let gk_lp = src_gk(&repo, &mut rows);
    let v9 = v9_phases(&repo);
    // DT/PX/DR rows carry the source's own series label
    for r in rows.iter_mut() {
        if r.series.is_empty() { r.series = r.arm.clone(); }
        if !r.series.contains(':') { r.series = format!("{}:{}", r.source, r.series); }
    }
    rows.sort_by(|a, b| (a.source.clone(), a.series.clone(), a.log2_r.to_bits()).cmp(&(b.source.clone(), b.series.clone(), b.log2_r.to_bits())));

    // ---- positive control: current band vs frozen band at w = 0 (FC)
    let mut pc = vec![];
    for r in rows.iter().filter(|r| r.source == "FC") {
        if let (Some(a), Some(b)) = (r.s[0], r.s_frozen[0]) {
            pc.push(json!({"slug": r.slug, "arm": r.arm, "w0_current": a, "w0_frozen": b, "rel_diff": (a - b).abs() / b,
                           "w1_current": nz(r.s[2]), "w1_frozen": nz(r.s_frozen[2])}));
        }
    }

    // ---- fits
    let mut series: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for r in &rows { series.entry(r.series.clone()).or_default().push(r); }
    let mut fits = vec![];
    for (name, rs) in &series {
        let mut rs: Vec<&Row> = rs.clone();
        rs.sort_by(|a, b| a.log2_r.partial_cmp(&b.log2_r).unwrap());
        // one point per curve: GK has two targets per size -> mean of log values per slug prefix (m)
        let mut pts: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
        for r in &rs { pts.entry(if r.source == "GK" { format!("m{}", r.m.unwrap()) } else { r.slug.clone() }).or_default().push(r); }
        if pts.len() < 4 { continue; }
        for wi in 0..3 {
            let mut x = vec![]; let mut xn = vec![]; let mut yi = vec![]; let mut yr = vec![]; let mut yq = vec![];
            for g in pts.values() {
                let ys: Vec<f64> = g.iter().filter_map(|r| r.log2_total(wi)).collect();
                let rr: Vec<f64> = g.iter().filter_map(|r| r.rho_log2_total(wi)).collect();
                if ys.len() != g.len() || rr.len() != g.len() { continue; }
                let m = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
                x.push(g[0].log2_r); xn.push(g[0].n.unwrap_or(f64::NAN)); yi.push(m(&ys)); yr.push(m(&rr)); yq.push(m(&ys) - m(&rr));
            }
            if x.len() < 4 { continue; }
            let fi = ols(&x, &yi).unwrap(); let fr = ols(&x, &yr).unwrap(); let fq = ols(&x, &yq).unwrap();
            let t = tq(fq.df());
            let at = |ft: &Fit, x0: f64| { let p = ft.pred(x0); let se = (ft.s2 + ft.se_mean(x0).powi(2)).sqrt(); json!({"x": x0, "log2_ratio": p, "ratio": 2f64.powf(p), "pred_interval95_ratio": [2f64.powf(p - t * se), 2f64.powf(p + t * se)]}) };
            let mut o = json!({"series": name, "weight": W[wi], "sizes": x.len(), "log2_r_range": [x[0], x[x.len()-1]],
                "x_axis_log2_r": {"ic_log2_cost": fi.js(), "rho_log2_cost": fr.js(), "log2_ratio": fq.js(),
                    "crossing": crossing(&fi, &fr, 129.0, 0x9e3779b97f4a7c15 ^ (wi as u64 + 1) ^ (name.len() as u64) << 20),
                    "ratio_at_log2_r_129": at(&fq, 129.0), "ratio_at_last_size": at(&fq, x[x.len()-1])}});
            if xn.iter().all(|v| v.is_finite()) {
                let fin = ols(&xn, &yi).unwrap(); let frn = ols(&xn, &yr).unwrap(); let fqn = ols(&xn, &yq).unwrap();
                o["x_axis_n"] = json!({"ic_log2_cost": fin.js(), "rho_log2_cost": frn.js(), "log2_ratio": fqn.js(),
                    "crossing": crossing(&fin, &frn, 131.0, 0x51ed27f1 ^ (wi as u64 + 7) ^ (name.len() as u64) << 24)});
                o["x_axis_n"]["n_values"] = json!(xn);
            }
            fits.push(o);
        }
    }

    // ---- outputs
    let hdr = "source,regime,n,slug,log2_r,arm,series,method,columns,m,relations,trials_per_relation,cost_per_relation,la_cost,S_w0,S_w0.1,S_w1,unit,log2_total_cost_w0,rho_kind,rho_S_w0,rho_S_w0.1,rho_S_w1,ratio_w0,ratio_w0.1,ratio_w1,band_reading,validation,large_prime,runs,verified,source_path";
    let mut csv = String::from(hdr); csv.push('\n');
    let q = |s: &str| format!("\"{}\"", s.replace('"', "'"));
    let mut jrows = vec![];
    for r in &rows {
        csv += &[q(&r.source), q(&r.regime), c(r.n), q(&r.slug), c(Some(r.log2_r)), q(&r.arm), q(&r.series), q(&r.method), c(r.columns), c(r.m), c(r.relations), c(r.trials_per_rel), c(r.cost_per_rel), c(r.la_cost), c(r.s[0]), c(r.s[1]), c(r.s[2]), q(&r.unit), c(r.log2_total(0)), q(&r.rho_kind), c(r.rho_s[0]), c(r.rho_s[1]), c(r.rho_s[2]), c(r.ratio(0)), c(r.ratio(1)), c(r.ratio(2)), q(&r.band_reading), q(&r.validation), q(&r.large_prime), c(r.runs), c(r.verified), q(&r.source_path)].join(",");
        csv.push('\n');
        jrows.push(json!({"source": r.source, "regime": r.regime, "n": nz(r.n), "slug": r.slug, "log2_r": r.log2_r, "arm": r.arm, "series": r.series, "method": r.method,
            "columns": nz(r.columns), "m": nz(r.m), "relations": nz(r.relations), "trials_per_relation": nz(r.trials_per_rel), "cost_per_relation": nz(r.cost_per_rel), "la_cost": nz(r.la_cost),
            "S": [nz(r.s[0]), nz(r.s[1]), nz(r.s[2])], "S_frozen_band": [nz(r.s_frozen[0]), nz(r.s_frozen[1]), nz(r.s_frozen[2])], "unit": r.unit,
            "log2_total_cost_w0": nz(r.log2_total(0)), "rho_kind": r.rho_kind, "rho_S": [nz(r.rho_s[0]), nz(r.rho_s[1]), nz(r.rho_s[2])],
            "ratio_to_rho": [nz(r.ratio(0)), nz(r.ratio(1)), nz(r.ratio(2))], "band_reading": r.band_reading, "validation": r.validation, "large_prime": r.large_prime,
            "runs": nz(r.runs), "verified": nz(r.verified), "phase_shares_w0": r.phases.iter().map(|(k, v)| (k.clone(), json!(v))).collect::<Map<_, _>>(), "note": r.note, "source_path": r.source_path}));
    }
    fs::write(lane.join("table.csv"), csv).unwrap();
    let out = json!({"schema": "l7.e2e_units/v1", "pcc": "[pcc:unverified]", "weights": W, "rows": jrows, "excluded": excl, "fits": fits,
        "positive_control_fc_band_w0": pc, "rule_v9_timed_phase_shares": v9, "gk_part_f_large_primes": gk_lp});
    fs::write(lane.join("results.json"), serde_json::to_string_pretty(&out).unwrap() + "\n").unwrap();
    svg(&lane, &rows, &fits);
    eprintln!("rows {} excluded {} fits {}", rows.len(), excl.len(), fits.len());
}

// ---------------------------------------------------------------- figures

fn svg(lane: &Path, rows: &[Row], fits: &[Value]) {
    let pal = ["#2a6fdb", "#d1495b", "#2e933c", "#e08e0b", "#7b4fd6", "#0f9d9a", "#8c564b", "#c2185b", "#555555", "#9e9d24"];
    let keep: Vec<&str> = vec!["FC:ic-compact", "LZ:ic-orbit-renew-lazy", "FO:ic-orbit-renew", "PX:koblitz:pipeline", "DT:dlp-selected", "DT:base-frozen", "LB-A:best", "LB-B:best", "PX:char2:plan", "GK:k4"];
    // Fig 1: log2 total cost vs log2 r, IC (filled) and rho (open), w = 0
    let (w, h, l, r_, t, b) = (900.0, 600.0, 70.0, 220.0, 30.0, 50.0);
    let (x0, x1, y0, y1) = (8.0, 132.0, 0.0, 80.0);
    let sx = |x: f64| l + (x - x0) / (x1 - x0) * (w - l - r_);
    let sy = |y: f64| h - b - (y - y0) / (y1 - y0) * (h - t - b);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='11'><rect width='100%' height='100%' fill='white'/>");
    s += &format!("<text x='{}' y='18' font-size='13'>log2 total cost (S·√r, w = 0) vs log2 r: complete binary/Koblitz IC (filled) and matched rho (open); dashed: OLS extrapolation to log2 r = 129 [pcc:unverified]</text>", l);
    for gx in (10..=130).step_by(10) { let X = sx(gx as f64); s += &format!("<line x1='{X}' y1='{}' x2='{X}' y2='{}' stroke='#eee'/><text x='{X}' y='{}' text-anchor='middle'>{gx}</text>", sy(y0), sy(y1), h - b + 15.0); }
    for gy in (0..=80).step_by(10) { let Y = sy(gy as f64); s += &format!("<line x1='{}' y1='{Y}' x2='{}' y2='{Y}' stroke='#eee'/><text x='{}' y='{}' text-anchor='end'>{gy}</text>", sx(x0), sx(x1), l - 5.0, Y + 4.0); }
    s += &format!("<line x1='{0}' y1='{1}' x2='{0}' y2='{2}' stroke='#999' stroke-dasharray='2,3'/><text x='{0}' y='{3}' fill='#999'>ECC2K-130 (log2 r = 129)</text>", sx(129.0), sy(y0), sy(y1), sy(y1) + 12.0);
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>log2 r</text><text x='15' y='{}' transform='rotate(-90 15 {})' text-anchor='middle'>log2 total cost (source unit)</text>", (l + w - r_) / 2.0, h - 10.0, h / 2.0, h / 2.0);
    for (i, k) in keep.iter().enumerate() {
        let col = pal[i % pal.len()];
        let pts: Vec<&Row> = rows.iter().filter(|r| r.series == *k).collect();
        for p in &pts {
            if let Some(y) = p.log2_total(0) { s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='{col}'/>", sx(p.log2_r), sy(y)); }
            if let Some(y) = p.rho_log2_total(0) { s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='none' stroke='{col}'/>", sx(p.log2_r), sy(y)); }
        }
        if let Some(ft) = fits.iter().find(|f| f["series"] == *k && f["weight"] == 0.0) {
            let g = &ft["x_axis_log2_r"];
            for (key, dash) in [("ic_log2_cost", "6,3"), ("rho_log2_cost", "1,3")] {
                let (a, bb) = (g[key]["intercept"].as_f64().unwrap(), g[key]["slope"].as_f64().unwrap());
                let xa = ft["log2_r_range"][0].as_f64().unwrap();
                s += &format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{col}' stroke-dasharray='{dash}' stroke-width='1'/>", sx(xa), sy(a + bb * xa), sx(129.0), sy((a + bb * 129.0).min(y1)));
            }
        }
        s += &format!("<circle cx='{}' cy='{}' r='4' fill='{col}'/><text x='{}' y='{}'>{k}</text>", w - r_ + 15.0, t + 20.0 + 16.0 * i as f64, w - r_ + 25.0, t + 24.0 + 16.0 * i as f64);
    }
    s += &format!("<text x='{}' y='{}' fill='#555'>filled = IC, open = rho;</text><text x='{}' y='{}' fill='#555'>units differ by source:</text><text x='{}' y='{}' fill='#555'>compare within a colour only</text></svg>", w - r_ + 15.0, t + 200.0, w - r_ + 15.0, t + 214.0, w - r_ + 15.0, t + 228.0);
    fs::write(lane.join("fig1_log2cost_vs_log2r.svg"), s).unwrap();

    // Fig 2: log2(IC/rho) at w = 0 vs log2 r, with ratio fits extended to 129
    let (y0, y1) = (-3.0, 16.0);
    let sy = |y: f64| h - b - (y - y0) / (y1 - y0) * (h - t - b);
    let mut s = format!("<svg xmlns='http://www.w3.org/2000/svg' width='{w}' height='{h}' font-family='sans-serif' font-size='11'><rect width='100%' height='100%' fill='white'/>");
    s += &format!("<text x='{}' y='18' font-size='13'>Distance to matched rho, log2(S_IC / S_rho) at w = 0, vs log2 r; dashed: OLS on log2 r extended to 129 [pcc:unverified]</text>", l);
    for gx in (10..=130).step_by(10) { let X = sx(gx as f64); s += &format!("<line x1='{X}' y1='{}' x2='{X}' y2='{}' stroke='#eee'/><text x='{X}' y='{}' text-anchor='middle'>{gx}</text>", sy(y0), sy(y1), h - b + 15.0); }
    for gy in -3..=16 { let Y = sy(gy as f64); s += &format!("<line x1='{}' y1='{Y}' x2='{}' y2='{Y}' stroke='{}'/><text x='{}' y='{}' text-anchor='end'>{gy}</text>", sx(x0), sx(x1), if gy == 0 { "#000" } else { "#eee" }, l - 5.0, Y + 4.0); }
    s += &format!("<text x='{}' y='{}'>parity with rho</text>", sx(100.0), sy(0.0) - 4.0);
    s += &format!("<line x1='{0}' y1='{1}' x2='{0}' y2='{2}' stroke='#999' stroke-dasharray='2,3'/><text x='{3}' y='{4}' fill='#999' text-anchor='end'>log2 r = 129</text>", sx(129.0), sy(y0), sy(y1), sx(129.0) - 3.0, sy(y1) + 12.0);
    s += &format!("<text x='{}' y='{}' text-anchor='middle'>log2 r</text><text x='15' y='{}' transform='rotate(-90 15 {})' text-anchor='middle'>log2(IC / matched rho)</text>", (l + w - r_) / 2.0, h - 10.0, h / 2.0, h / 2.0);
    for (i, k) in keep.iter().enumerate() {
        let col = pal[i % pal.len()];
        for p in rows.iter().filter(|r| r.series == *k) {
            if let Some(q) = p.ratio(0) { s += &format!("<circle cx='{}' cy='{}' r='3.5' fill='{col}'/>", sx(p.log2_r), sy(q.log2().clamp(y0, y1))); }
        }
        if let Some(ft) = fits.iter().find(|f| f["series"] == *k && f["weight"] == 0.0) {
            let g = &ft["x_axis_log2_r"]["log2_ratio"];
            let (a, bb) = (g["intercept"].as_f64().unwrap(), g["slope"].as_f64().unwrap());
            let xa = ft["log2_r_range"][0].as_f64().unwrap();
            let xe = 129.0f64;
            s += &format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{col}' stroke-dasharray='6,3'/>", sx(xa), sy((a + bb * xa).clamp(y0, y1)), sx(xe), sy((a + bb * xe).clamp(y0, y1)));
        }
        s += &format!("<circle cx='{}' cy='{}' r='4' fill='{col}'/><text x='{}' y='{}'>{k}</text>", w - r_ + 15.0, t + 20.0 + 16.0 * i as f64, w - r_ + 25.0, t + 24.0 + 16.0 * i as f64);
    }
    s += &format!("<text x='{}' y='{}' fill='#555'>GK: vs rho THEORY;</text><text x='{}' y='{}' fill='#555'>composite degree 4m,</text><text x='{}' y='{}' fill='#555'>subfield-dependent (§8b)</text></svg>", w - r_ + 15.0, t + 200.0, w - r_ + 15.0, t + 214.0, w - r_ + 15.0, t + 228.0);
    fs::write(lane.join("fig2_ratio_vs_log2r.svg"), s).unwrap();
}
