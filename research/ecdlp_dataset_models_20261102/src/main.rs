//! Development pipeline for the ECDLP/PDP dataset resource/success models (docs/ml-dataset/training-protocol-20261101.md).
//!
//! Three steps keep the protocol's custody boundary explicit:
//!
//! * `audit <export> <out.json>`: scans every public file of a native `crypto dataset export` for restricted material
//!   (forbidden keys inside `inputs`, and every known-answer / recovered scalar as a literal anywhere in the public
//!   bytes), and writes the allowlist of input field paths actually present. Exit 1 on any finding.
//! * `join <export> <table.jsonl> [--canary]`: the *evaluator* side. Reads restricted labels, featurises **public
//!   inputs only** into engineered descriptors, adds the attempted arm as a one-hot *query* input, and labels each row
//!   verified = 1 / censored-at-cap = 0 (budget_exhausted, timeout, partial). Unsupported arms are excluded and
//!   counted. `--canary` appends a column equal to the label: the positive control for the leakage tooling only.
//! * `train <table.jsonl> <out dir>`: leave-one-component-out evaluation over the exporter's `split_component`, with
//!   a stratum-majority baseline (arm x family x m), logistic regression and a small MLP; a within-stratum label
//!   permutation negative control; Brier score, log loss, accuracy and calibration bins. Development-only: nothing
//!   here selects a winner or estimates generalisation.
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde_json::{json, Value};

const SEED: u64 = 0x6563_646c_7020_1102; // fixed before data (protocol)
const ARMS: [&str; 4] = ["enumerate", "bsgs", "root", "double-large-prime"];
const FAMILIES: [&str; 3] = ["koblitz", "binary", "prime"];
/// Keys that must never appear inside a public `inputs` object (provenance, outcomes, secrets, solver identity).
const FORBIDDEN_KEYS: [&str; 14] = ["known_answer", "known_answer_scalar", "scalar", "recovered", "solver", "status", "labels",
    "certificate", "receipt", "elapsed", "elapsed_ms", "witness", "outcome", "private"];

fn read_jsonl(path: &Path) -> Vec<Value> {
    fs::read_to_string(path).unwrap_or_default().lines().filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("jsonl row")).collect()
}

// ------------------------------------------------------------------ audit

fn key_paths(v: &Value, prefix: &str, out: &mut BTreeSet<String>, forbidden: &mut Vec<String>) {
    match v {
        Value::Object(m) => for (k, x) in m {
            let p = format!("{prefix}.{k}");
            let lk = k.to_ascii_lowercase();
            if FORBIDDEN_KEYS.iter().any(|f| lk == *f || lk.contains("known_answer") || lk.contains("witness")) {
                forbidden.push(p.clone());
            }
            key_paths(x, &p, out, forbidden);
        },
        Value::Array(a) => { if let Some(x) = a.first() { key_paths(x, &format!("{prefix}[]"), out, forbidden); } else { out.insert(format!("{prefix}[]")); } }
        _ => { out.insert(prefix.to_string()); }
    }
}

fn audit(export: &Path, out: &Path) -> bool {
    let mut allow = BTreeSet::new();
    let mut forbidden = Vec::new();
    let public = ["features.jsonl", "prefix-features.jsonl", "algebraic-features.jsonl"];
    let mut public_bytes = String::new();
    let mut rows = 0;
    for f in public {
        let text = fs::read_to_string(export.join(f)).unwrap_or_default();
        for row in text.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(row).expect("public row");
            key_paths(&v["inputs"], &format!("{f}:inputs"), &mut allow, &mut forbidden);
            rows += 1;
        }
        public_bytes.push_str(&text);
    }
    // every restricted scalar must be absent from the public bytes (as a quoted decimal literal)
    let targets = read_jsonl(&export.join("restricted/targets.jsonl"));
    let mut scalars = BTreeSet::new();
    for t in &targets {
        for k in ["known_answer_scalar", "independently_recovered_scalar"] {
            if let Some(s) = t[k].as_str() { if s.len() >= 4 { scalars.insert(s.to_string()); } }
        }
    }
    let leaked: Vec<&String> = scalars.iter().filter(|s| public_bytes.contains(&format!("\"{s}\""))).collect();
    let ok = forbidden.is_empty() && leaked.is_empty();
    let report = json!({"schema": "ecdlp-dataset-models.audit/v1", "public_rows": rows, "restricted_rows": targets.len(),
        "allowlist": allow, "forbidden_key_paths": forbidden, "restricted_scalars_checked": scalars.len(),
        "restricted_scalars_found_in_public_bytes": leaked, "pass": ok});
    fs::write(out, serde_json::to_string_pretty(&report).unwrap() + "\n").unwrap();
    println!("audit: {} public rows, {} restricted scalars checked, {} forbidden key paths, {} leaked scalars -> {}",
        rows, scalars.len(), report["forbidden_key_paths"].as_array().unwrap().len(), leaked.len(), if ok { "PASS" } else { "FAIL" });
    ok
}

// ------------------------------------------------------------------ join (evaluator side)

fn num(v: &Value) -> f64 {
    match v { Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN), Value::Number(n) => n.as_f64().unwrap_or(f64::NAN), Value::Bool(b) => *b as u8 as f64, _ => f64::NAN }
}

/// Engineered descriptors from public inputs only. Names are recorded in the table so the allowlist is explicit.
fn featurize(inp: &Value) -> Vec<(String, f64)> {
    let fam = inp["family"].as_str().unwrap_or("");
    let fb = &inp["factor_base"];
    let orbits = fb["orbits"].as_array().map(|a| a.len()).unwrap_or(0) as f64;
    let points = fb["points"].as_array().map(|a| a.len()).unwrap_or(0) as f64;
    let orbit_pts: f64 = fb["orbits"].as_array().map(|a| a.iter().map(|o| o["points"].as_array().map(|p| p.len()).unwrap_or(0) as f64).sum()).unwrap_or(0.0);
    let mut f = vec![
        ("n".into(), num(&inp["n"])),
        ("m".into(), num(&inp["m"])),
        ("twist".into(), num(&inp["twist"])),
        ("log2_subgroup_order".into(), num(&inp["curve"]["subgroup_order"]).log2()),
        ("log2_group_order".into(), num(&inp["curve"]["group_order"]).log2()),
        ("log2_cofactor".into(), num(&inp["curve"]["cofactor"]).log2()),
        ("factor_base_points".into(), points),
        ("factor_base_orbits".into(), orbits),
        ("mean_orbit_size".into(), if orbits > 0.0 { orbit_pts / orbits } else { 0.0 }),
        ("frobenius_power".into(), num(&inp["symmetry"]["frobenius_power"])),
        ("frobenius_nontrivial".into(), num(&inp["symmetry"]["frobenius_nontrivial_on_model"])),
        ("negation".into(), num(&inp["symmetry"]["negation"])),
        ("coefficient_field_degree".into(), num(&inp["symmetry"]["coefficient_field_degree"])),
        ("characteristic_two".into(), (inp["field"]["characteristic"].as_str() == Some("2") || inp["field"]["characteristic"] == json!(2)) as u8 as f64),
    ];
    for fam_name in FAMILIES { f.push((format!("family_{fam_name}"), (fam == fam_name) as u8 as f64)); }
    for (_, v) in f.iter_mut() { if !v.is_finite() { *v = 0.0; } }
    f
}

fn join(export: &Path, table: &Path, canary: bool) {
    let feats: HashMap<String, Value> = read_jsonl(&export.join("features.jsonl")).into_iter()
        .map(|r| (r["feature_id"].as_str().unwrap().to_string(), r)).collect();
    let mut out = String::new();
    let mut excluded: BTreeMap<String, usize> = BTreeMap::new();
    let mut kept = 0;
    let mut names: Vec<String> = Vec::new();
    for t in read_jsonl(&export.join("restricted/targets.jsonl")) {
        let status = t["labels"]["status"].as_str().unwrap_or("missing").to_string();
        let arm = t["solver"].as_str().unwrap_or("").to_string();
        let label = match status.as_str() {
            "verified" => 1.0,
            "budget_exhausted" | "timeout" | "partial" => 0.0,
            other => { *excluded.entry(format!("status:{other}")).or_default() += 1; continue; }
        };
        let Some(fid) = t["feature_id"].as_str() else { *excluded.entry("no_feature_identity".into()).or_default() += 1; continue };
        let Some(row) = feats.get(fid) else { *excluded.entry("feature_row_missing".into()).or_default() += 1; continue };
        if !ARMS.contains(&arm.as_str()) { *excluded.entry(format!("arm:{arm}")).or_default() += 1; continue; }
        let mut f = featurize(&row["inputs"]);
        for a in ARMS { f.push((format!("arm_{a}"), (arm == a) as u8 as f64)); }
        if canary { f.push(("LEAK_CANARY_label".into(), label)); }
        if names.is_empty() { names = f.iter().map(|x| x.0.clone()).collect(); }
        let stratum = format!("{arm}|{}|m{}", row["inputs"]["family"].as_str().unwrap_or("?"), row["inputs"]["m"]);
        out.push_str(&json!({"group": row["split_component"], "stratum": stratum, "arm": arm, "label": label, "status": status,
            "x": f.iter().map(|x| x.1).collect::<Vec<_>>()}).to_string());
        out.push('\n');
        kept += 1;
    }
    fs::write(table, &out).unwrap();
    fs::write(table.with_extension("meta.json"), serde_json::to_string_pretty(&json!({
        "schema": "ecdlp-dataset-models.table/v1", "feature_names": names, "rows": kept, "excluded": excluded, "canary": canary,
        "label": "1 = verified within the fixed work cap; 0 = right-censored at the cap (budget_exhausted, timeout, partial)",
        "query_input": "arm_* one-hot: the method being attempted, chosen before any outcome exists"})).unwrap() + "\n").unwrap();
    println!("join: {kept} rows, excluded {excluded:?}, {} features{}", names.len(), if canary { " (+ LEAK CANARY)" } else { "" });
}

// ------------------------------------------------------------------ models

struct Row { group: String, stratum: String, arm: String, y: f64, x: Vec<f64> }

fn load(table: &Path) -> Vec<Row> {
    read_jsonl(table).into_iter().map(|r| Row {
        group: r["group"].as_str().unwrap_or("?").to_string(), stratum: r["stratum"].as_str().unwrap().to_string(),
        arm: r["arm"].as_str().unwrap().to_string(), y: r["label"].as_f64().unwrap(),
        x: r["x"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect() }).collect()
}

trait Model { fn predict(&self, r: &Row) -> f64; }

/// Laplace-smoothed verified rate per stratum, falling back to the arm, then to the global rate.
struct Majority { strata: HashMap<String, f64>, arms: HashMap<String, f64>, global: f64 }
impl Majority {
    fn fit(rows: &[&Row]) -> Self {
        let mut s: HashMap<String, (f64, f64)> = HashMap::new();
        let mut a: HashMap<String, (f64, f64)> = HashMap::new();
        for r in rows { let e = s.entry(r.stratum.clone()).or_default(); e.0 += r.y; e.1 += 1.0; let e = a.entry(r.arm.clone()).or_default(); e.0 += r.y; e.1 += 1.0; }
        let g = (rows.iter().map(|r| r.y).sum::<f64>() + 1.0) / (rows.len() as f64 + 2.0);
        Majority { strata: s.into_iter().map(|(k, (p, n))| (k, (p + 1.0) / (n + 2.0))).collect(),
                   arms: a.into_iter().map(|(k, (p, n))| (k, (p + 1.0) / (n + 2.0))).collect(), global: g }
    }
}
impl Model for Majority {
    fn predict(&self, r: &Row) -> f64 { *self.strata.get(&r.stratum).or(self.arms.get(&r.arm)).unwrap_or(&self.global) }
}

/// Dense MLP (hidden = 0 gives logistic regression) on standardised inputs, Adam, BCE, L2.
struct Net { mean: Vec<f64>, sd: Vec<f64>, d: usize, h: usize, p: Vec<f64> }
impl Net {
    fn fit(rows: &[&Row], h: usize, epochs: usize, seed: u64) -> Self {
        let d = rows[0].x.len();
        let n = rows.len() as f64;
        let mean: Vec<f64> = (0..d).map(|j| rows.iter().map(|r| r.x[j]).sum::<f64>() / n).collect();
        // A feature constant in the training fold (sd = 0) carries no information the model can use, and standardising
        // a held-out value by a floored sd would explode (e.g. field size n under leave-one-component-out). Mark it with
        // sd = +inf so its standardised value is 0 for every row, train and test alike.
        let sd: Vec<f64> = (0..d).map(|j| {
            let v = (rows.iter().map(|r| (r.x[j] - mean[j]).powi(2)).sum::<f64>() / n).sqrt();
            if v < 1e-12 { f64::INFINITY } else { v }
        }).collect();
        let mut rng = StdRng::seed_from_u64(seed);
        let np = if h == 0 { d + 1 } else { d * h + h + h + 1 };
        let mut p: Vec<f64> = (0..np).map(|_| (rng.gen::<f64>() * 2.0 - 1.0) * (1.0 / (d as f64).sqrt())).collect();
        let (mut m1, mut m2) = (vec![0.0; np], vec![0.0; np]);
        let mut net = Net { mean, sd, d, h, p: vec![] };
        let mut idx: Vec<usize> = (0..rows.len()).collect();
        let (lr, b1, b2, l2) = (0.01, 0.9, 0.999, 1e-3);
        let mut t = 0;
        for _ in 0..epochs {
            idx.shuffle(&mut rng);
            for chunk in idx.chunks(32) {
                let mut g = vec![0.0; np];
                for &i in chunk { net.p = std::mem::take(&mut p); net.grad(&rows[i].x, rows[i].y, &mut g); p = std::mem::take(&mut net.p); }
                t += 1;
                for k in 0..np {
                    let gk = g[k] / chunk.len() as f64 + l2 * p[k];
                    m1[k] = b1 * m1[k] + (1.0 - b1) * gk;
                    m2[k] = b2 * m2[k] + (1.0 - b2) * gk * gk;
                    p[k] -= lr * (m1[k] / (1.0 - b1.powi(t))) / ((m2[k] / (1.0 - b2.powi(t))).sqrt() + 1e-8);
                }
            }
        }
        net.p = p;
        net
    }
    fn z(&self, x: &[f64]) -> Vec<f64> { (0..self.d).map(|j| (x[j] - self.mean[j]) / self.sd[j]).collect() }
    fn forward(&self, x: &[f64]) -> (f64, Vec<f64>, Vec<f64>) {
        let z = self.z(x);
        if self.h == 0 {
            let s = self.p[self.d] + (0..self.d).map(|j| self.p[j] * z[j]).sum::<f64>();
            return (sig(s), z, vec![]);
        }
        let (d, h) = (self.d, self.h);
        let hid: Vec<f64> = (0..h).map(|k| (self.p[d * h + k] + (0..d).map(|j| self.p[j * h + k] * z[j]).sum::<f64>()).max(0.0)).collect();
        let o = d * h + h;
        let s = self.p[o + h] + (0..h).map(|k| self.p[o + k] * hid[k]).sum::<f64>();
        (sig(s), z, hid)
    }
    fn grad(&self, x: &[f64], y: f64, g: &mut [f64]) {
        let (p, z, hid) = self.forward(x);
        let dz = p - y;
        if self.h == 0 { for j in 0..self.d { g[j] += dz * z[j]; } g[self.d] += dz; return; }
        let (d, h) = (self.d, self.h);
        let o = d * h + h;
        for k in 0..h {
            g[o + k] += dz * hid[k];
            if hid[k] > 0.0 {
                let dh = dz * self.p[o + k];
                g[d * h + k] += dh;
                for j in 0..d { g[j * h + k] += dh * z[j]; }
            }
        }
        g[o + h] += dz;
    }
}
impl Model for Net { fn predict(&self, r: &Row) -> f64 { self.forward(&r.x).0 } }

fn sig(s: f64) -> f64 { 1.0 / (1.0 + (-s).exp()) }

#[derive(Default, Clone)]
struct Score { n: usize, brier: f64, logloss: f64, correct: usize, bins: [(f64, f64, usize); 5] }
impl Score {
    fn add(&mut self, p: f64, y: f64) {
        let pc = p.clamp(1e-6, 1.0 - 1e-6);
        self.n += 1; self.brier += (p - y).powi(2); self.logloss += -(y * pc.ln() + (1.0 - y) * (1.0 - pc).ln());
        self.correct += ((p >= 0.5) as u8 as f64 == y) as usize;
        let b = ((p * 5.0) as usize).min(4);
        self.bins[b].0 += p; self.bins[b].1 += y; self.bins[b].2 += 1;
    }
    fn json(&self) -> Value {
        if self.n == 0 {
            return json!({"n": 0, "brier": null, "logloss": null, "accuracy": null, "calibration": []});
        }
        let n = self.n as f64;
        json!({"n": self.n, "brier": self.brier / n, "logloss": self.logloss / n, "accuracy": self.correct as f64 / n,
            "calibration": self.bins.iter().enumerate().filter(|(_, b)| b.2 > 0)
                .map(|(i, b)| json!({"bin": format!("{:.1}-{:.1}", i as f64 / 5.0, (i + 1) as f64 / 5.0), "mean_pred": b.0 / b.2 as f64, "observed": b.1 / b.2 as f64, "n": b.2})).collect::<Vec<_>>()})
    }
}

fn permute_within_stratum(rows: &mut [Row], seed: u64) {
    let mut by: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, r) in rows.iter().enumerate() { by.entry(r.stratum.clone()).or_default().push(i); }
    let mut rng = StdRng::seed_from_u64(seed);
    for idx in by.values() {
        let mut ys: Vec<f64> = idx.iter().map(|&i| rows[i].y).collect();
        ys.shuffle(&mut rng);
        for (k, &i) in idx.iter().enumerate() { rows[i].y = ys[k]; }
    }
}

fn evaluate(rows: &[Row]) -> Value {
    let groups: BTreeSet<String> = rows.iter().map(|r| r.group.clone()).collect();
    let names = ["stratum_majority", "logistic_regression", "mlp_32"];
    let mut pooled: Vec<Score> = vec![Score::default(); 3];
    let mut folds = Vec::new();
    for (gi, g) in groups.iter().enumerate() {
        let train: Vec<&Row> = rows.iter().filter(|r| &r.group != g).collect();
        let test: Vec<&Row> = rows.iter().filter(|r| &r.group == g).collect();
        if train.is_empty() || test.is_empty() { continue; }
        let models: Vec<Box<dyn Model>> = vec![Box::new(Majority::fit(&train)), Box::new(Net::fit(&train, 0, 60, SEED ^ gi as u64)),
                                               Box::new(Net::fit(&train, 32, 60, SEED ^ 0x100 ^ gi as u64))];
        let mut fold = serde_json::Map::new();
        for (mi, m) in models.iter().enumerate() {
            let mut s = Score::default();
            for r in &test { let p = m.predict(r); s.add(p, r.y); pooled[mi].add(p, r.y); }
            fold.insert(names[mi].into(), s.json());
        }
        folds.push(json!({"held_out_component": g, "train_rows": train.len(), "test_rows": test.len(),
            "test_verified_rate": test.iter().map(|r| r.y).sum::<f64>() / test.len() as f64, "models": fold}));
    }
    json!({"components": groups.len(), "folds": folds,
        "pooled": names.iter().zip(&pooled).map(|(n, s)| (n.to_string(), s.json())).collect::<serde_json::Map<_, _>>()})
}

fn train(table: &Path, out: &Path) {
    fs::create_dir_all(out).unwrap();
    let rows = load(table);
    let primary = evaluate(&rows);
    let mut permuted = load(table);
    permute_within_stratum(&mut permuted, SEED ^ 0xfeed);
    let control = evaluate(&permuted);
    let summary = json!({"schema": "ecdlp-dataset-models.summary/v1", "status": "development-only",
        "rows": rows.len(), "evaluation": "leave-one-component-out over the exporter's split_component",
        "primary": primary, "negative_control_within_stratum_permutation": control,
        "note": "Development data may debug the pipeline; it may not select a winner or estimate generalisation (training-protocol-20261101)."});
    fs::write(out.join("summary.json"), serde_json::to_string_pretty(&summary).unwrap() + "\n").unwrap();
    for (label, s) in [("primary", &summary["primary"]), ("permuted", &summary["negative_control_within_stratum_permutation"])] {
        let p = &s["pooled"];
        println!("{label:9} pooled Brier: majority {:.4} | logreg {:.4} | mlp {:.4}   (n={}, components={})",
            p["stratum_majority"]["brier"].as_f64().unwrap_or(f64::NAN), p["logistic_regression"]["brier"].as_f64().unwrap_or(f64::NAN),
            p["mlp_32"]["brier"].as_f64().unwrap_or(f64::NAN), p["stratum_majority"]["n"], s["components"]);
    }
    let sha = |p: &Path| { use sha2::Digest; format!("{:x}", sha2::Sha256::digest(fs::read(p).unwrap())) };
    println!("table sha256 {}  summary sha256 {}", sha(table), sha(&out.join("summary.json")));
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("audit") => { if !audit(Path::new(&a[2]), Path::new(&a[3])) { std::process::exit(1); } }
        Some("join") => join(Path::new(&a[2]), Path::new(&a[3]), a.get(4).map(String::as_str) == Some("--canary")),
        Some("train") => train(Path::new(&a[2]), Path::new(&a[3])),
        _ => eprintln!("usage: ecdlp-dataset-models audit <export> <out.json> | join <export> <table.jsonl> [--canary] | train <table.jsonl> <out dir>"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic two-component export: 2 field sizes x 2 families x 3 arms, labels a deterministic function of arm.
    fn synthetic_export(dir: &Path, leak_scalar_in_public: bool, forbidden_key: bool) {
        fs::create_dir_all(dir.join("restricted")).unwrap();
        let (mut feats, mut targets) = (String::new(), String::new());
        let mut i = 0;
        for n in [23, 25] {
            for fam in ["koblitz", "prime"] {
                for k in 0..6 {
                    let fid = format!("f{n}{fam}{k}");
                    let scalar = format!("{}", 1_000_003 + i);
                    let mut inputs = json!({"n": n, "m": 2, "family": fam, "twist": k % 2 == 0,
                        "curve": {"subgroup_order": "2095853", "group_order": "8383412", "cofactor": "4"},
                        "factor_base": {"points": [1, 2, 3], "orbits": [{"points": [1, 2]}]},
                        "symmetry": {"frobenius_power": 1, "negation": true}, "field": {"characteristic": 2}});
                    if leak_scalar_in_public && i == 3 { inputs["note"] = json!(scalar.clone()); }
                    if forbidden_key && i == 5 { inputs["solver"] = json!("bsgs"); }
                    feats.push_str(&json!({"feature_id": fid, "split_component": format!("field-size-n{n}"), "inputs": inputs}).to_string());
                    feats.push('\n');
                    for (a, arm) in ["enumerate", "bsgs", "root"].iter().enumerate() {
                        let status = if (a + k) % 3 == 0 { "budget_exhausted" } else { "verified" };
                        targets.push_str(&json!({"feature_id": fid, "solver": arm, "labels": {"status": status},
                            "known_answer_scalar": scalar}).to_string());
                        targets.push('\n');
                    }
                    i += 1;
                }
            }
        }
        fs::write(dir.join("features.jsonl"), feats).unwrap();
        fs::write(dir.join("restricted/targets.jsonl"), targets).unwrap();
    }

    fn tmp(name: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("edm-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn audit_passes_clean_export() {
        let d = tmp("clean"); synthetic_export(&d, false, false);
        assert!(audit(&d, &d.join("a.json")));
    }

    #[test]
    fn audit_detects_planted_scalar_positive_control() {
        let d = tmp("leak"); synthetic_export(&d, true, false);
        assert!(!audit(&d, &d.join("a.json")));
        let r: Value = serde_json::from_str(&fs::read_to_string(d.join("a.json")).unwrap()).unwrap();
        assert_eq!(r["restricted_scalars_found_in_public_bytes"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn audit_detects_forbidden_key_positive_control() {
        let d = tmp("key"); synthetic_export(&d, false, true);
        assert!(!audit(&d, &d.join("a.json")));
    }

    #[test]
    fn join_uses_public_inputs_and_censors_at_cap() {
        let d = tmp("join"); synthetic_export(&d, false, false);
        join(&d, &d.join("t.jsonl"), false);
        let rows = load(&d.join("t.jsonl"));
        assert_eq!(rows.len(), 72);
        assert!(rows.iter().any(|r| r.y == 0.0) && rows.iter().any(|r| r.y == 1.0));
        let meta: Value = serde_json::from_str(&fs::read_to_string(d.join("t.meta.json")).unwrap()).unwrap();
        assert!(!meta["feature_names"].as_array().unwrap().iter().any(|n| n.as_str().unwrap().contains("CANARY")));
    }

    #[test]
    fn canary_column_is_learned_positive_control() {
        let d = tmp("canary"); synthetic_export(&d, false, false);
        join(&d, &d.join("t.jsonl"), true);
        let rows = load(&d.join("t.jsonl"));
        let r = evaluate(&rows);
        assert_eq!(r["components"], 2);
        assert!(r["pooled"]["logistic_regression"]["brier"].as_f64().unwrap() < 0.05, "{r}");
    }

    #[test]
    fn permutation_keeps_stratum_label_counts() {
        let d = tmp("perm"); synthetic_export(&d, false, false);
        join(&d, &d.join("t.jsonl"), false);
        let mut rows = load(&d.join("t.jsonl"));
        let before: BTreeMap<String, f64> = rows.iter().fold(BTreeMap::new(), |mut m, r| { *m.entry(r.stratum.clone()).or_default() += r.y; m });
        permute_within_stratum(&mut rows, 7);
        let after: BTreeMap<String, f64> = rows.iter().fold(BTreeMap::new(), |mut m, r| { *m.entry(r.stratum.clone()).or_default() += r.y; m });
        assert_eq!(before, after);
    }
}
