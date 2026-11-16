//! Frozen analysis of the wide-modulus sparse linear algebra benchmark.
//!
//! Implements, exactly and only, PROTOCOL.md ("Counted units", "Controls",
//! "Acceptance and stop rules", "Analysis (frozen)", "How the extrapolation
//! model is falsified") as modified by PROTOCOL_AMENDMENT_1.md (one RunPod pod
//! per seed; A/A per pod).  Native port of the earlier analysis.py: same
//! results.json schema, same `--md` summary.
//!
//!     wide-sla-analysis --raw RAW_DIR --out results.json [--md]
//!
//! RAW_DIR holds seed1/ seed2/ seed3/, each with grid.jsonl aa.jsonl
//! wide_coeffs.jsonl runs.jsonl host.json done.json (any may be missing).
//! The pod (directory) is the seed.  Bootstrap generator: xoshiro256**
//! seeded by SplitMix64 from 20261112 (see lib.rs).
//!
//! Test-only knobs (defaults are the frozen values): --resamples (10000),
//! --width-min-log2 (16).

use serde::Serialize;
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use wide_sla::notes::STATIC_NOTES;
use wide_sla::*;

const BOOT_SEED: u64 = 20261112;
const N_BOOT: usize = 10000;
const SEEDS: [i64; 3] = [1, 2, 3];
const SOLVERS: [&str; 2] = ["full-bw", "full-bl"];
const HOLDOUT_LOG2: i64 = 20;
const CURV_MIN_LOG2: i64 = 14;
const COUNT_TOL: f64 = 1.10;
const BETA_EXPECT: (f64, f64) = (1.9, 2.1);
const D_TARGET: f64 = 1e9;
const TARGET_ARM: &str = "U192@129";
const CONTROL_ARM: &str = "u64@61";
const ARMS: [&str; 7] = [
    "u64@61", "U128@61", "U128@89", "U128@127", "U192@129", "U192@190", "U256@255",
];
const COUNTED: [&str; 8] = [
    "row_ops",
    "krylov_products",
    "krylov_lanes",
    "core_dimension",
    "core_nonzeros",
    "filter_row_ops",
    "rows",
    "nonzeros",
];
const STATUS_NAMES: [&str; 7] = [
    "valid",
    "censored",
    "skipped",
    "timeout",
    "oom",
    "invalid_unrepeated",
    "missing",
];

type Key3 = (String, String, i64);
type Key4 = (String, String, i64, i64);

// ------------------------------------------------------------------ helpers
fn limbs_of(label: &str) -> i64 {
    match label.split('@').next().unwrap().to_lowercase().as_str() {
        "u64" => 1,
        "u128" => 2,
        "u192" => 3,
        "u256" => 4,
        _ => panic!("unknown arm label {label}"),
    }
}

fn natural_word(bits: i64) -> &'static str {
    match bits {
        61 => "u64",
        89 | 127 => "u128",
        129 | 190 => "u192",
        255 => "u256",
        _ => "?",
    }
}

fn arm_label(bits: i64, word: &str) -> String {
    if word == "u64" {
        format!("u64@{}", bits)
    } else {
        format!("{}@{}", word.to_uppercase(), bits)
    }
}

fn label_from_runs(bits: i64, word: Option<&Value>) -> Option<String> {
    match word {
        None | Some(Value::Null) => Some(arm_label(bits, natural_word(bits))),
        Some(Value::String(s)) if s.is_empty() || s == "-" => {
            Some(arm_label(bits, natural_word(bits)))
        }
        Some(Value::String(s)) => Some(arm_label(bits, s)),
        _ => None,
    }
}

fn to_int(v: &Value) -> Option<i64> {
    match v {
        Value::Number(n) => n.as_i64().or_else(|| n.as_f64().map(|f| f.trunc() as i64)),
        Value::String(s) => s.trim().parse::<i64>().ok(),
        Value::Bool(b) => Some(*b as i64),
        _ => None,
    }
}

fn log2d(d: i64) -> i64 {
    (d as f64).log2().round() as i64
}

fn num(r: &Value, k: &str) -> Option<f64> {
    r.get(k).and_then(|v| v.as_f64())
}

fn arm_str(r: &Value) -> String {
    match r.get("arm") {
        Some(Value::String(s)) => s.clone(),
        _ => "None".to_string(),
    }
}

fn jf(x: f64) -> Value {
    json!(x)
}

fn jvec(v: &[f64]) -> Value {
    Value::Array(v.iter().map(|&x| jf(x)).collect())
}

fn jints(v: &[i64]) -> Value {
    Value::Array(v.iter().map(|&x| json!(x)).collect())
}

fn jopt(x: Option<f64>) -> Value {
    x.map(jf).unwrap_or(Value::Null)
}

fn pystr(s: &str) -> String {
    if s.contains('\'') && !s.contains('"') {
        format!("\"{}\"", s)
    } else {
        format!("'{}'", s.replace('\\', "\\\\").replace('\'', "\\'"))
    }
}

fn pylist_str(v: &[String]) -> String {
    format!(
        "[{}]",
        v.iter().map(|s| pystr(s)).collect::<Vec<_>>().join(", ")
    )
}

fn key4_repr(k: &Key4) -> String {
    format!("('{}', '{}', {}, {})", k.0, k.1, k.2, k.3)
}

struct OMap<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
    idx: HashMap<K, usize>,
}

impl<K: Clone + Eq + std::hash::Hash, V> OMap<K, V> {
    fn new() -> Self {
        OMap {
            keys: vec![],
            vals: vec![],
            idx: HashMap::new(),
        }
    }
    fn entry(&mut self, k: K, default: impl FnOnce() -> V) -> &mut V {
        if let Some(&i) = self.idx.get(&k) {
            return &mut self.vals[i];
        }
        self.idx.insert(k.clone(), self.vals.len());
        self.keys.push(k);
        self.vals.push(default());
        self.vals.last_mut().unwrap()
    }
    fn get(&self, k: &K) -> Option<&V> {
        self.idx.get(k).map(|&i| &self.vals[i])
    }
    fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.vals.iter())
    }
}

// ------------------------------------------------------------------ loading
struct Pod {
    seed: i64,
    exists: bool,
    missing: Vec<String>,
    bad: Vec<(String, usize)>,
    records: HashMap<&'static str, Vec<Value>>,
    host: Option<Value>,
    done: Option<Value>,
}

fn read_jsonl(path: &Path) -> Option<(Vec<Value>, usize)> {
    let text = fs::read_to_string(path).ok()?;
    let (mut out, mut bad) = (vec![], 0usize);
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(line) {
            Ok(v) if v.is_object() => out.push(v),
            _ => bad += 1,
        }
    }
    Some((out, bad))
}

fn read_json(path: &Path) -> Option<Value> {
    let v: Value = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;
    if v.is_null() {
        None
    } else {
        Some(v)
    }
}

fn truthy(v: &Option<Value>) -> bool {
    match v {
        None => false,
        Some(Value::Object(m)) => !m.is_empty(),
        Some(Value::Null) => false,
        Some(_) => true,
    }
}

fn load_pod(raw: &Path, seed: i64) -> Pod {
    let d = raw.join(format!("seed{}", seed));
    let mut pod = Pod {
        seed,
        exists: d.is_dir(),
        missing: vec![],
        bad: vec![],
        records: HashMap::new(),
        host: None,
        done: None,
    };
    for name in ["grid", "aa", "wide_coeffs", "runs"] {
        let (recs, bad) = match read_jsonl(&d.join(format!("{}.jsonl", name))) {
            Some(x) => x,
            None => {
                pod.missing.push(format!("{}.jsonl", name));
                (vec![], 0)
            }
        };
        pod.records.insert(name, recs);
        pod.bad.push((name.to_string(), bad));
    }
    pod.host = read_json(&d.join("host.json"));
    pod.done = read_json(&d.join("done.json"));
    if pod.host.is_none() {
        pod.missing.push("host.json".into());
    }
    if pod.done.is_none() {
        pod.missing.push("done.json".into());
    }
    pod
}

#[derive(Clone)]
struct Attempt {
    valid: bool,
    ver_false: bool,
    exit: Option<f64>,
    rec: Option<Value>,
}

struct Cell {
    attempts: Vec<Attempt>,
    skipped: bool,
}

fn build_cells(pod: &Pod, fname: &str) -> OMap<Key3, Cell> {
    let mut cells: OMap<Key3, Cell> = OMap::new();
    for r in &pod.records[fname] {
        let (key, att);
        if r.get("failed")
            .map(|v| match v {
                Value::Bool(b) => *b,
                Value::Null => false,
                Value::Number(n) => n.as_f64() != Some(0.0),
                Value::String(s) => !s.is_empty(),
                _ => true,
            })
            .unwrap_or(false)
        {
            let bits = match r.get("modulus_bits").and_then(to_int) {
                Some(b) => b,
                None => continue,
            };
            let lab = match label_from_runs(bits, r.get("word")) {
                Some(l) => l,
                None => continue,
            };
            let cols = match r.get("cols").and_then(to_int) {
                Some(c) => c,
                None => continue,
            };
            key = (lab, arm_str(r), cols);
            att = Attempt {
                valid: false,
                ver_false: false,
                exit: r.get("exit").and_then(|v| v.as_f64()),
                rec: None,
            };
        } else {
            let bits = match r.get("modulus_bits").and_then(to_int) {
                Some(b) => b,
                None => continue,
            };
            let word = match r.get("word_width") {
                Some(Value::String(s)) => s.clone(),
                _ => continue,
            };
            if r.get("arm").is_none() {
                continue;
            }
            let cols = match r.get("cols").and_then(to_int) {
                Some(c) => c,
                None => continue,
            };
            key = (arm_label(bits, &word), arm_str(r), cols);
            let ver = r.get("verified");
            att = Attempt {
                valid: ver == Some(&Value::Bool(true)),
                ver_false: ver == Some(&Value::Bool(false)),
                exit: Some(0.0),
                rec: Some(r.clone()),
            };
        }
        cells
            .entry(key, || Cell {
                attempts: vec![],
                skipped: false,
            })
            .attempts
            .push(att);
    }
    if fname == "grid" {
        for r in &pod.records["runs"] {
            if r.get("skipped").is_some() {
                let bits = match r.get("modulus_bits").and_then(to_int) {
                    Some(b) => b,
                    None => continue,
                };
                let lab = match label_from_runs(bits, r.get("word")) {
                    Some(l) => l,
                    None => continue,
                };
                let cols = match r.get("cols").and_then(to_int) {
                    Some(c) => c,
                    None => continue,
                };
                cells
                    .entry((lab, arm_str(r), cols), || Cell {
                        attempts: vec![],
                        skipped: false,
                    })
                    .skipped = true;
            }
        }
    }
    cells
}

fn cell_status(c: &Cell) -> &'static str {
    if c.attempts.iter().any(|a| a.valid) {
        return "valid";
    }
    if c.attempts.iter().any(|a| a.exit == Some(124.0)) {
        return "timeout";
    }
    if c.attempts.iter().any(|a| a.exit == Some(137.0)) {
        return "oom";
    }
    if c.skipped && c.attempts.is_empty() {
        return "skipped";
    }
    match c.attempts.len() {
        0 => "missing",
        1 => "invalid_unrepeated",
        _ => "censored",
    }
}

struct Cell4 {
    status: String,
    attempts: Vec<Attempt>,
    rec: Option<Value>,
    n_valid: usize,
}

fn build_all(pods: &[Pod]) -> OMap<Key4, Cell4> {
    let mut out: OMap<Key4, Cell4> = OMap::new();
    for pod in pods {
        let s = pod.seed;
        let cells = build_cells(pod, "grid");
        for ((lab, sol, d), c) in cells.iter() {
            let status = cell_status(c).to_string();
            let rec = c
                .attempts
                .iter()
                .find(|a| a.valid)
                .and_then(|a| a.rec.clone());
            let n_valid = c.attempts.iter().filter(|a| a.valid).count();
            let k = (lab.clone(), sol.clone(), *d, s);
            *out.entry(k, || Cell4 {
                status: String::new(),
                attempts: vec![],
                rec: None,
                n_valid: 0,
            }) = Cell4 {
                status,
                attempts: c.attempts.clone(),
                rec,
                n_valid,
            };
        }
        // stop rule: ignore larger D of an arm after its first timeout / OOM
        let mut stop: HashMap<(String, String), i64> = HashMap::new();
        for ((lab, sol, d, ss), c) in out.iter() {
            if *ss == s && (c.status == "timeout" || c.status == "oom") {
                let e = stop.entry((lab.clone(), sol.clone())).or_insert(*d);
                *e = (*e).min(*d);
            }
        }
        for i in 0..out.keys.len() {
            let (lab, sol, d, ss) = out.keys[i].clone();
            if ss == s {
                if let Some(&sd) = stop.get(&(lab, sol)) {
                    if d > sd && out.vals[i].status != "skipped" {
                        out.vals[i].status = "skipped".into();
                        out.vals[i].rec = None;
                    }
                }
            }
        }
    }
    out
}

// ------------------------------------------------------------------ size law
fn groups_by_size(points: &[(i64, f64)]) -> (Vec<i64>, Vec<Vec<usize>>) {
    let mut xs: Vec<i64> = points.iter().map(|p| p.0).collect();
    xs.sort();
    xs.dedup();
    let groups = xs
        .iter()
        .map(|&x| (0..points.len()).filter(|&i| points[i].0 == x).collect())
        .collect();
    (xs, groups)
}

fn line_design(points: &[(i64, f64)]) -> (Vec<Vec<f64>>, Vec<f64>) {
    (
        points.iter().map(|p| vec![1.0, p.0 as f64]).collect(),
        points.iter().map(|p| p.1).collect(),
    )
}

fn ols_pts(points: &[(i64, f64)]) -> (f64, f64) {
    let x: Vec<f64> = points.iter().map(|p| p.0 as f64).collect();
    let y: Vec<f64> = points.iter().map(|p| p.1).collect();
    ols_line(&x, &y)
}

fn size_law(points: &[(i64, f64)], b: usize) -> Value {
    let (xs, groups) = groups_by_size(points);
    let mut m = Map::new();
    if xs.len() < 3 {
        m.insert("status".into(), json!("insufficient sizes"));
        m.insert("sizes_log2".into(), jints(&xs));
        m.insert("n_points".into(), json!(points.len()));
        return Value::Object(m);
    }
    let (a, sl) = ols_pts(points);
    let mut rng = Rng::new(BOOT_SEED);
    let (design, y) = line_design(points);
    let coefs = bootstrap_coefs(&design, &y, &groups, b, &mut rng);
    let ba: Vec<f64> = coefs.iter().map(|c| c[0]).collect();
    let bs: Vec<f64> = coefs.iter().map(|c| c[1]).collect();
    m.insert("status".into(), json!("ok"));
    m.insert("sizes_log2".into(), jints(&xs));
    m.insert("n_points".into(), json!(points.len()));
    m.insert("a".into(), jf(a));
    m.insert(
        "a_ci95".into(),
        jvec(&[percentile(&ba, 2.5), percentile(&ba, 97.5)]),
    );
    m.insert("slope".into(), jf(sl));
    m.insert("beta".into(), jf(sl + 1.0));
    m.insert(
        "beta_ci95".into(),
        jvec(&[percentile(&bs, 2.5) + 1.0, percentile(&bs, 97.5) + 1.0]),
    );
    m.insert(
        "bootstrap".into(),
        json!({"resamples": b, "generator_seed": BOOT_SEED, "scheme": "percentile, seeds resampled within size"}),
    );
    let res: Vec<f64> = points.iter().map(|p| p.1 - (a + sl * p.0 as f64)).collect();
    let n = res.len() as f64;
    let mean = res.iter().sum::<f64>() / n;
    let sd = (res.iter().map(|r| (r - mean) * (r - mean)).sum::<f64>() / (n - 2.0)).sqrt();
    m.insert("resid_sd_log2".into(), jf(sd));
    Value::Object(m)
}

fn stat3(v: &[f64]) -> Value {
    json!({"median": median(v), "min": min_f(v), "max": max_f(v)})
}

// ------------------------------------------------------------------ analysis
struct Analysis {
    res: Value,
}

fn analyse(raw: &Path, b: usize, wmin: i64) -> Analysis {
    let mut notes: Vec<String> = vec![];
    let pods: Vec<Pod> = SEEDS.iter().map(|&s| load_pod(raw, s)).collect();
    let cells = build_all(&pods);

    let mut pods_info = Map::new();
    for p in &pods {
        let h = p.host.clone().unwrap_or(Value::Object(Map::new()));
        let mut host = Map::new();
        for k in [
            "nproc",
            "cpu_model",
            "mem_total_kb",
            "rustc",
            "kernel",
            "cgroup_cpu_max",
            "cgroup_mem_max",
            "binary_sha256",
            "src_sha256",
            "pod_id",
        ] {
            host.insert(k.into(), h.get(k).cloned().unwrap_or(Value::Null));
        }
        let mut bad = Map::new();
        for (n, c) in &p.bad {
            bad.insert(n.clone(), json!(c));
        }
        let status = p
            .done
            .as_ref()
            .and_then(|d| d.get("status"))
            .cloned()
            .unwrap_or(Value::Null);
        pods_info.insert(
            format!("seed{}", p.seed),
            json!({
                "dir_exists": p.exists, "missing_files": p.missing, "bad_json_lines": bad,
                "status": status, "host": host,
                "n_grid_lines": p.records["grid"].len(), "n_runs_lines": p.records["runs"].len(),
            }),
        );
    }
    let pyrepr = |v: Option<&Value>| -> String {
        match v {
            None | Some(Value::Null) => "None".to_string(),
            Some(Value::String(s)) => s.clone(),
            Some(o) => o.to_string(),
        }
    };
    let host_set = |key: &str| -> Vec<String> {
        let mut v: Vec<String> = pods
            .iter()
            .filter(|p| truthy(&p.host))
            .map(|p| pyrepr(p.host.as_ref().unwrap().get(key)))
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let cpus = host_set("cpu_model");
    let bins = host_set("binary_sha256");
    if cpus.len() > 1 {
        notes.push(format!(
            "pods report different cpu_model values: {}; wall-time comparisons across seeds cross hosts",
            pylist_str(&cpus)
        ));
    }
    if bins.len() > 1 {
        notes.push(format!(
            "pods report different binary_sha256 values: {}",
            pylist_str(&bins)
        ));
    }
    let mut integrity = Map::new();
    for p in &pods {
        let runs: Vec<&Value> = p.records["runs"]
            .iter()
            .filter(|r| r.get("skipped").is_none())
            .collect();
        let recfiles = ["grid", "aa", "wide_coeffs"];
        integrity.insert(
            format!("seed{}", p.seed),
            json!({
                "runs_attempt_lines": runs.len(),
                "runs_valid": runs.iter().filter(|r| r.get("valid") == Some(&Value::Bool(true))).count(),
                "record_lines_all_files": recfiles.iter().map(|f| p.records[f].len()).sum::<usize>(),
                "records_verified_true_all_files": recfiles.iter().map(|f| p.records[f].iter().filter(|r| r.get("verified") == Some(&Value::Bool(true))).count()).sum::<usize>(),
            }),
        );
    }

    // valid cells in order
    let mut valid: Vec<(Key4, Value)> = vec![];
    for (k, c) in cells.iter() {
        if c.status == "valid" {
            valid.push((k.clone(), c.rec.clone().unwrap_or(Value::Null)));
        }
    }
    let valid_map: HashMap<Key4, usize> = valid
        .iter()
        .enumerate()
        .map(|(i, (k, _))| (k.clone(), i))
        .collect();
    let vget = |k: &Key4| -> Option<&Value> { valid_map.get(k).map(|&i| &valid[i].1) };

    // grid sanity: seed recorded vs pod
    for ((lab, sol, d, s), c) in cells.iter() {
        if let Some(rec) = &c.rec {
            let rs = rec.get("seed");
            if rs.and_then(|v| v.as_f64()) != Some(*s as f64) {
                notes.push(format!(
                    "record seed {} in seed{}/grid.jsonl for {} {} D={}",
                    match rs {
                        None | Some(Value::Null) => "None".to_string(),
                        Some(v) => v.to_string(),
                    },
                    s,
                    lab,
                    sol,
                    d
                ));
                break;
            }
        }
    }

    // ---- coverage
    let mut sizes: Vec<i64> = cells.keys.iter().map(|k| k.2).collect();
    sizes.extend((12..=20).map(|k| 1i64 << k));
    sizes.sort();
    sizes.dedup();
    let cover_d = sizes;
    let mut coverage = Map::new();
    let mut totals: Vec<(String, i64)> = STATUS_NAMES.iter().map(|s| (s.to_string(), 0)).collect();
    for lab in ARMS {
        for sol in SOLVERS {
            for &d in &cover_d {
                let mut cnt: Vec<(String, i64)> =
                    STATUS_NAMES.iter().map(|s| (s.to_string(), 0)).collect();
                for s in SEEDS {
                    let st = cells
                        .get(&(lab.to_string(), sol.to_string(), d, s))
                        .map(|c| c.status.clone())
                        .unwrap_or_else(|| "missing".into());
                    cnt.iter_mut().find(|(n, _)| *n == st).unwrap().1 += 1;
                }
                for (n, c) in &cnt {
                    totals.iter_mut().find(|(t, _)| t == n).unwrap().1 += c;
                }
                let mut m = Map::new();
                for (n, c) in cnt {
                    m.insert(n, json!(c));
                }
                coverage.insert(format!("{}|{}|{}", lab, sol, d), Value::Object(m));
            }
        }
    }
    let mut totals_m = Map::new();
    for (n, c) in &totals {
        totals_m.insert(n.clone(), json!(c));
    }
    let repeated = cells
        .iter()
        .filter(|(_, c)| c.status == "valid" && c.attempts.len() > 1)
        .count();
    let multi_valid = cells.iter().filter(|(_, c)| c.n_valid > 1).count();
    if multi_valid > 0 {
        notes.push(format!(
            "{} cells had two valid attempts; the first valid one is used",
            multi_valid
        ));
    }

    // ---- size laws
    let mut size = Map::new();
    for lab in ARMS {
        for sol in SOLVERS {
            let mut ent = Map::new();
            for q in ["row_ops", "core_dimension", "core_nonzeros"] {
                let mut pts: Vec<(i64, f64)> = vec![];
                for ((l, so, d, _), r) in &valid {
                    if l == lab && so == sol {
                        if let Some(v) = num(r, q) {
                            let y = v / *d as f64;
                            if y > 0.0 {
                                pts.push((log2d(*d), y.log2()));
                            }
                        }
                    }
                }
                ent.insert(q.into(), size_law(&pts, b));
            }
            let r0 = ent.get_mut("row_ops").unwrap().as_object_mut().unwrap();
            if r0["status"] == json!("ok") {
                let beta = r0["beta"].as_f64().unwrap();
                r0.insert(
                    "beta_in_expected_range_1.9_2.1".into(),
                    json!(BETA_EXPECT.0 <= beta && beta <= BETA_EXPECT.1),
                );
            }
            size.insert(format!("{}|{}", lab, sol), Value::Object(ent));
        }
    }

    // ---- ns per row op
    let mut ns: HashMap<Key4, f64> = HashMap::new();
    for (k, r) in &valid {
        if let (Some(ro), Some(sv)) = (num(r, "row_ops"), num(r, "solve_s")) {
            if ro > 0.0 {
                ns.insert(k.clone(), 1e9 * sv / ro);
            }
        }
    }

    // ---- A/A first (the bands are needed by the width law)
    let mut aa_recs: OMap<(String, String), Vec<(i64, Value)>> = OMap::new();
    let mut aa_per_pod = Map::new();
    let mut pod_band: HashMap<i64, Option<f64>> = HashMap::new();
    for p in &pods {
        for r in &p.records["aa"] {
            if r.get("failed")
                .map(|v| v == &Value::Bool(true))
                .unwrap_or(false)
                || r.get("word_width").is_none()
            {
                continue;
            }
            let (bits, word) = match (
                r.get("modulus_bits").and_then(to_int),
                r["word_width"].as_str(),
            ) {
                (Some(b), Some(w)) => (b, w.to_string()),
                _ => continue,
            };
            let lab = arm_label(bits, &word);
            aa_recs
                .entry((lab, arm_str(r)), Vec::new)
                .push((p.seed, r.clone()));
        }
        let mut bands = Map::new();
        let mut bv: Vec<f64> = vec![];
        for lab in [TARGET_ARM, CONTROL_ARM] {
            let ts: Vec<&Value> = aa_recs
                .get(&(lab.to_string(), "full-bw".to_string()))
                .map(|l| {
                    l.iter()
                        .filter(|(sd, r)| {
                            *sd == p.seed
                                && r.get("solve_s")
                                    .map(|v| match v {
                                        Value::Null => false,
                                        Value::Number(n) => n.as_f64() != Some(0.0),
                                        _ => true,
                                    })
                                    .unwrap_or(false)
                        })
                        .map(|(_, r)| &r["solve_s"])
                        .collect()
                })
                .unwrap_or_default();
            let tsf: Vec<f64> = ts.iter().filter_map(|v| v.as_f64()).collect();
            let band = if tsf.len() >= 2 && min_f(&tsf) > 0.0 {
                Some(max_f(&tsf) / min_f(&tsf))
            } else {
                None
            };
            if let Some(x) = band {
                bv.push(x);
            }
            bands.insert(
                lab.into(),
                json!({"n_runs": ts.len(), "solve_s": ts, "band_max_over_min": jopt(band)}),
            );
        }
        let pb = if bv.is_empty() {
            None
        } else {
            Some(max_f(&bv))
        };
        pod_band.insert(p.seed, pb);
        aa_per_pod.insert(
            format!("seed{}", p.seed),
            json!({"arms": bands, "pod_band": jopt(pb)}),
        );
    }
    let mut aa = Map::new();
    aa.insert("per_pod".into(), Value::Object(aa_per_pod));
    let mut det = Map::new();
    let mut all_det = true;
    let mut aa_sorted: Vec<(&(String, String), &Vec<(i64, Value)>)> = aa_recs.iter().collect();
    aa_sorted.sort_by(|a, b| a.0.cmp(b.0));
    for ((lab, sol), lst) in aa_sorted {
        let mut tuples: Vec<(Vec<Value>, Vec<i64>)> = vec![];
        for (sd, r) in lst {
            let t: Vec<Value> = COUNTED
                .iter()
                .map(|k| r.get(*k).cloned().unwrap_or(Value::Null))
                .collect();
            let ts: Vec<String> = t.iter().map(|v| v.to_string()).collect();
            if let Some(e) = tuples
                .iter_mut()
                .find(|(x, _)| x.iter().map(|v| v.to_string()).collect::<Vec<_>>() == ts)
            {
                e.1.push(*sd);
            } else {
                tuples.push((t, vec![*sd]));
            }
        }
        let ident = tuples.len() == 1;
        all_det &= ident;
        let mut diff = vec![];
        for (i, k) in COUNTED.iter().enumerate() {
            let set: HashSet<String> = tuples.iter().map(|(t, _)| t[i].to_string()).collect();
            if set.len() > 1 {
                diff.push(*k);
            }
        }
        let mut pods_v: Vec<i64> = lst.iter().map(|(s, _)| *s).collect();
        pods_v.sort();
        pods_v.dedup();
        let values: Vec<Value> = tuples
            .iter()
            .map(|(t, _)| {
                let mut m = Map::new();
                for (k, v) in COUNTED.iter().zip(t) {
                    m.insert((*k).into(), v.clone());
                }
                Value::Object(m)
            })
            .collect();
        det.insert(
            format!("{}|{}", lab, sol),
            json!({"n_runs": lst.len(), "pods": pods_v, "identical": ident, "fields_that_differ": diff, "values": values}),
        );
    }
    let have_aa = !aa_recs.keys.is_empty();
    aa.insert("determinism".into(), Value::Object(det));
    aa.insert(
        "all_identical".into(),
        if have_aa { json!(all_det) } else { Value::Null },
    );
    aa.insert(
        "expected_runs_per_arm".into(),
        json!("3 per pod (before, mid, after), 9 over three pods"),
    );
    if !have_aa {
        notes.push("no A/A records found".into());
    }
    if have_aa && !all_det {
        notes.push("A/A DETERMINISM FAILURE: per protocol the session is void".into());
    }

    // ---- width law (timed)
    let mut ds_w: Vec<i64> = ns
        .keys()
        .filter(|k| log2d(k.2) >= wmin)
        .map(|k| k.2)
        .collect();
    ds_w.sort();
    ds_w.dedup();
    let mut width = Map::new();
    width.insert("min_log2_D".into(), json!(wmin));
    let mut bands_used = Map::new();
    for s in SEEDS {
        bands_used.insert(s.to_string(), jopt(pod_band[&s]));
    }
    width.insert("bands_used_per_pod".into(), Value::Object(bands_used));
    let mut table = Map::new();
    let mut width_law_viol: Vec<Value> = vec![];
    // limb-fit per_seed values kept for the extrapolation: (sol, D) -> Vec<fitted t(3)>
    let mut t3_by: HashMap<(String, i64), Vec<f64>> = HashMap::new();
    for sol in SOLVERS {
        for &d in &ds_w {
            let mut arms_m = Map::new();
            for lab in ARMS {
                let vals: Vec<(i64, f64)> = SEEDS
                    .iter()
                    .filter_map(|&s| {
                        ns.get(&(lab.to_string(), sol.to_string(), d, s))
                            .map(|&x| (s, x))
                    })
                    .collect();
                if vals.is_empty() {
                    continue;
                }
                let v: Vec<f64> = vals.iter().map(|x| x.1).collect();
                let mut per_seed = Map::new();
                for (s, x) in &vals {
                    per_seed.insert(s.to_string(), jf(*x));
                }
                let mut e = Map::new();
                e.insert("limbs".into(), json!(limbs_of(lab)));
                e.insert("n_seeds".into(), json!(v.len()));
                e.insert("ns_per_row_op_median".into(), jf(median(&v)));
                e.insert("min".into(), jf(min_f(&v)));
                e.insert("max".into(), jf(max_f(&v)));
                e.insert("per_seed".into(), Value::Object(per_seed));
                let mut ratios = Map::new();
                let mut rv = vec![];
                for (s, x) in &vals {
                    if let Some(&base) = ns.get(&(CONTROL_ARM.to_string(), sol.to_string(), d, *s))
                    {
                        if base != 0.0 {
                            let rr = x / base;
                            let bnd = pod_band[s];
                            rv.push(rr);
                            ratios.insert(
                                s.to_string(),
                                json!({"ratio_to_u64_61": rr, "pod_band": jopt(bnd),
                                       "resolved": bnd.map(|bb| rr > bb || rr < 1.0 / bb)}),
                            );
                        }
                    }
                }
                if !ratios.is_empty() {
                    e.insert(
                        "ratio_to_u64_61".into(),
                        json!({"median": median(&rv), "min": min_f(&rv), "max": max_f(&rv), "per_seed_within_pod": ratios}),
                    );
                }
                arms_m.insert(lab.into(), Value::Object(e));
            }
            // limb fit per pod
            let mut fits: Vec<(i64, Value, f64, f64, [f64; 4], [f64; 4], [f64; 4])> = vec![];
            for s in SEEDS {
                let mut tl = [f64::NAN; 4];
                let mut have = 0;
                for l in 1..=4i64 {
                    let xs: Vec<f64> = ARMS
                        .iter()
                        .filter(|lab| limbs_of(lab) == l)
                        .filter_map(|lab| {
                            ns.get(&(lab.to_string(), sol.to_string(), d, s)).copied()
                        })
                        .collect();
                    if !xs.is_empty() {
                        tl[(l - 1) as usize] = median(&xs);
                        have += 1;
                    }
                }
                if have == 4 {
                    let design: Vec<Vec<f64>> =
                        (1..=4).map(|l| vec![1.0, (l * l) as f64]).collect();
                    let c = lstsq(&design, &tl);
                    let (c0, c2) = (c[0], c[1]);
                    let mut fitted = [0.0; 4];
                    let mut fr = [0.0; 4];
                    let mut or = [0.0; 4];
                    for l in 1..=4usize {
                        fitted[l - 1] = c0 + c2 * (l * l) as f64;
                        fr[l - 1] = (c0 + c2 * (l * l) as f64) / (c0 + c2);
                        or[l - 1] = tl[l - 1] / tl[0];
                    }
                    let l4 = |a: &[f64; 4]| {
                        let mut m = Map::new();
                        for l in 1..=4 {
                            m.insert(l.to_string(), jf(a[l - 1]));
                        }
                        Value::Object(m)
                    };
                    let v = json!({"t_L_ns": l4(&tl), "c0": c0, "c2": c2, "fitted_t_ns": l4(&fitted),
                        "fitted_ratio_t_L_over_t_1": l4(&fr), "observed_ratio_t_L_over_t_1": l4(&or)});
                    fits.push((s, v, c0, c2, fitted, fr, or));
                }
            }
            let limb_fit = if !fits.is_empty() {
                let mut summ = Map::new();
                for (key, sel) in [("c0", 0usize), ("c2", 1usize)] {
                    let v: Vec<f64> = fits
                        .iter()
                        .map(|f| if sel == 0 { f.2 } else { f.3 })
                        .collect();
                    summ.insert(key.into(), stat3(&v));
                }
                for (key, sel) in [
                    ("fitted_ratio_t_L_over_t_1", 5usize),
                    ("observed_ratio_t_L_over_t_1", 6usize),
                ] {
                    let mut m = Map::new();
                    for l in 0..4usize {
                        let v: Vec<f64> = fits
                            .iter()
                            .map(|f| if sel == 5 { f.5[l] } else { f.6[l] })
                            .collect();
                        m.insert((l + 1).to_string(), stat3(&v));
                    }
                    summ.insert(key.into(), Value::Object(m));
                }
                let mut per_seed = Map::new();
                for f in &fits {
                    per_seed.insert(f.0.to_string(), f.1.clone());
                }
                t3_by.insert((sol.to_string(), d), fits.iter().map(|f| f.4[2]).collect());
                json!({"per_seed": per_seed, "summary_over_seeds": summ, "n_seeds": fits.len()})
            } else {
                json!({"status": "needs all four limb counts in one pod"})
            };
            table.insert(
                format!("{}|{}", sol, d),
                json!({"arms": arms_m, "limb_fit": limb_fit}),
            );
            // same-word comparisons within a pod against the pod's A/A band
            for (gname, labs) in [
                ("U128", vec!["U128@61", "U128@89", "U128@127"]),
                ("U192", vec!["U192@129", "U192@190"]),
            ] {
                for s in SEEDS {
                    let present: Vec<&str> = labs
                        .iter()
                        .copied()
                        .filter(|l| ns.contains_key(&(l.to_string(), sol.to_string(), d, s)))
                        .collect();
                    let v: Vec<f64> = present
                        .iter()
                        .map(|l| ns[&(l.to_string(), sol.to_string(), d, s)])
                        .collect();
                    if v.len() >= 2 {
                        let rr = max_f(&v) / min_f(&v);
                        let bnd = pod_band[&s];
                        width_law_viol.push(
                            json!({"solver": sol, "D": d, "seed": s, "group": gname,
                            "arms_present": present, "max_over_min_ns": rr, "pod_band": jopt(bnd),
                            "beyond_band": bnd.map(|bb| rr > bb)}),
                        );
                    }
                }
            }
        }
    }
    width.insert("table".into(), Value::Object(table));

    // ---- secondary: wide coeffs vs unit
    let mut wide_cells: OMap<Key4, Option<Value>> = OMap::new();
    for p in &pods {
        for ((lab, sol, d), c) in build_cells(p, "wide_coeffs").iter() {
            let rec = c
                .attempts
                .iter()
                .find(|a| a.valid)
                .and_then(|a| a.rec.clone());
            *wide_cells.entry((lab.clone(), sol.clone(), *d, p.seed), || None) = rec;
        }
    }
    let mut wl: Vec<(&Key4, &Option<Value>)> = wide_cells.iter().collect();
    wl.sort_by_key(|(k, _)| key4_repr(k));
    let mut wide = Map::new();
    for ((lab, sol, d, s), r) in wl {
        let u = vget(&(lab.clone(), sol.clone(), *d, *s));
        let wr = r.as_ref().and_then(|r| r.get("row_ops")).cloned();
        let ur = u.and_then(|u| u.get("row_ops")).cloned();
        let ratio = match (r, u) {
            (Some(r), Some(u)) => match (num(r, "row_ops"), num(u, "row_ops")) {
                (Some(a), Some(bb)) if bb != 0.0 => Some(a / bb),
                _ => None,
            },
            _ => None,
        };
        wide.insert(
            format!("{}|{}|D={}|seed{}", lab, sol, d, s),
            json!({"wide_row_ops": wr, "unit_row_ops": ur, "ratio_wide_over_unit": jopt(ratio)}),
        );
    }

    // ---- falsification
    let mut hold = Map::new();
    let mut curv = Map::new();
    for lab in ARMS {
        for sol in SOLVERS {
            let pts: Vec<(i64, f64, i64)> = valid
                .iter()
                .filter(|((l, so, _, _), _)| l == lab && so == sol)
                .filter_map(|((_, _, d, s), r)| {
                    num(r, "row_ops").map(|v| (log2d(*d), (v / *d as f64).log2(), *s))
                })
                .collect();
            let key = format!("{}|{}", lab, sol);
            let tr: Vec<(i64, f64)> = pts
                .iter()
                .filter(|p| p.0 < HOLDOUT_LOG2)
                .map(|p| (p.0, p.1))
                .collect();
            let te: Vec<&(i64, f64, i64)> = pts.iter().filter(|p| p.0 == HOLDOUT_LOG2).collect();
            let mut trx: Vec<i64> = tr.iter().map(|p| p.0).collect();
            trx.sort();
            trx.dedup();
            if trx.len() < 3 || te.is_empty() {
                hold.insert(
                    key.clone(),
                    json!({"status": "not evaluable",
                           "reason": format!("needs >=3 sizes below 2^{} and observed D=2^{}", HOLDOUT_LOG2, HOLDOUT_LOG2),
                           "train_sizes_log2": trx, "n_test": te.len()}),
                );
            } else {
                let x: Vec<f64> = tr.iter().map(|p| p.0 as f64).collect();
                let y: Vec<f64> = tr.iter().map(|p| p.1).collect();
                let n = x.len();
                let (a, sl) = ols_pts(&tr);
                let s2 = x
                    .iter()
                    .zip(&y)
                    .map(|(xx, yy)| (yy - a - sl * xx).powi(2))
                    .sum::<f64>()
                    / (n as f64 - 2.0);
                let xbar = x.iter().sum::<f64>() / n as f64;
                let sxx = x.iter().map(|v| (v - xbar).powi(2)).sum::<f64>();
                let tq = t_ppf(0.975, (n - 2) as f64);
                let mut per = vec![];
                for &&(xt, yt, s) in &te {
                    let xt = xt as f64;
                    let pred = a + sl * xt;
                    let half =
                        tq * (s2 * (1.0 + 1.0 / n as f64 + (xt - xbar).powi(2) / sxx)).sqrt();
                    per.push(json!({"seed": s, "observed_log2_y": yt, "predicted_log2_y": pred,
                                    "pi95": [pred - half, pred + half], "outside": (yt - pred).abs() > half}));
                }
                let bad = per.iter().any(|q| q["outside"] == json!(true));
                hold.insert(
                    key.clone(),
                    json!({"status": if bad {"FAIL"} else {"pass"}, "train_sizes_log2": trx, "df": n - 2,
                           "t_975": tq, "per_seed": per}),
                );
            }
            let cs: Vec<(i64, f64)> = pts
                .iter()
                .filter(|p| p.0 >= CURV_MIN_LOG2)
                .map(|p| (p.0, p.1))
                .collect();
            let mut csx: Vec<i64> = cs.iter().map(|p| p.0).collect();
            csx.sort();
            csx.dedup();
            if csx.len() < 4 {
                curv.insert(
                    key,
                    json!({"status": "not evaluable", "reason": format!("needs >=4 sizes at D>=2^{}", CURV_MIN_LOG2),
                           "sizes_log2": csx}),
                );
            } else {
                let mut rng = Rng::new(BOOT_SEED);
                let design: Vec<Vec<f64>> = cs
                    .iter()
                    .map(|p| {
                        let x = p.0 as f64 - 16.0;
                        vec![1.0, x, x * x]
                    })
                    .collect();
                let y: Vec<f64> = cs.iter().map(|p| p.1).collect();
                let (_, groups) = groups_by_size(&cs);
                let coefs = bootstrap_coefs(&design, &y, &groups, b, &mut rng);
                let b2: Vec<f64> = coefs.iter().map(|c| c[2]).collect();
                let coef = lstsq(&design, &y);
                let (lo, hi) = (percentile(&b2, 2.5), percentile(&b2, 97.5));
                curv.insert(
                    key,
                    json!({"status": if lo > 0.0 || hi < 0.0 {"FAIL"} else {"pass"}, "sizes_log2": csx,
                           "coef_log2D_sq": coef[2], "ci95": [lo, hi]}),
                );
            }
        }
    }
    // width independence of the count
    let mut worst: Option<Value> = None;
    let mut worst_rr = f64::NEG_INFINITY;
    let mut ncells = 0usize;
    let mut dvals: Vec<i64> = valid.iter().map(|(k, _)| k.2).collect();
    dvals.sort();
    dvals.dedup();
    for sol in SOLVERS {
        for &d in &dvals {
            for s in SEEDS {
                let v: Vec<(&str, f64)> = ARMS
                    .iter()
                    .filter_map(|l| {
                        vget(&(l.to_string(), sol.to_string(), d, s))
                            .and_then(|r| num(r, "row_ops"))
                            .map(|x| (*l, x))
                    })
                    .collect();
                if v.len() >= 2 {
                    ncells += 1;
                    let mx = v.iter().fold(f64::NEG_INFINITY, |a, x| a.max(x.1));
                    let mn = v.iter().fold(f64::INFINITY, |a, x| a.min(x.1));
                    let rr = mx / mn;
                    if worst.is_none() || rr > worst_rr {
                        worst_rr = rr;
                        worst = Some(json!({"solver": sol, "D": d, "seed": s, "max_over_min": rr,
                            "max_arm": v.iter().find(|x| x.1 == mx).unwrap().0,
                            "min_arm": v.iter().find(|x| x.1 == mn).unwrap().0}));
                    }
                }
            }
        }
    }
    let wi_status = match &worst {
        None => "not evaluable",
        Some(_) => {
            if worst_rr > COUNT_TOL {
                "FAIL"
            } else {
                "pass"
            }
        }
    };
    let wi = json!({"threshold": COUNT_TOL, "n_cells_compared": ncells, "worst": worst, "status": wi_status});
    // width law
    let evald: Vec<&Value> = width_law_viol
        .iter()
        .filter(|v| !v["beyond_band"].is_null())
        .collect();
    let viol: Vec<&Value> = evald
        .iter()
        .copied()
        .filter(|v| v["beyond_band"] == json!(true))
        .collect();
    let wl_status = if evald.is_empty() {
        "not evaluable"
    } else if !viol.is_empty() {
        "FAIL"
    } else {
        "pass"
    };
    let wlaw = json!({"status": wl_status, "n_comparisons": width_law_viol.len(),
        "n_without_band": width_law_viol.len() - evald.len(), "violations": viol, "comparisons": width_law_viol});
    // planted
    let mut plant: Vec<(Value, bool)> = vec![];
    let mut cl: Vec<(&Key4, &Cell4)> = cells.iter().collect();
    cl.sort_by_key(|(k, _)| key4_repr(k));
    for ((lab, sol, d, s), c) in cl.iter().map(|(k, c)| (*k, *c)) {
        if (c.status == "censored" || c.status == "invalid_unrepeated")
            && c.attempts.iter().any(|a| a.ver_false)
        {
            let surv = c.attempts.len() >= 2;
            plant.push((
                json!({"arm": lab, "solver": sol, "D": d, "seed": s, "attempts": c.attempts.len(), "survived_repeat": surv}),
                surv,
            ));
        }
    }
    let n_surv = plant.iter().filter(|p| p.1).count();
    let planted = json!({"status": if n_surv > 0 {"FAIL"} else {"pass"},
        "unverified_cells": plant.iter().map(|p| p.0.clone()).collect::<Vec<_>>(), "n_surviving_repeat": n_surv});
    // verdict
    let mut verdict = Map::new();
    let mut verdict_str: HashMap<&str, String> = HashMap::new();
    for sol in SOLVERS {
        let key = format!("{}|{}", TARGET_ARM, sol);
        let planted_target = if plant
            .iter()
            .any(|p| p.0["arm"] == json!(TARGET_ARM) && p.0["solver"] == json!(sol) && p.1)
        {
            "FAIL"
        } else {
            "pass"
        };
        let parts = json!({"holdout": hold[&key]["status"], "curvature": curv[&key]["status"],
            "width_independence_of_count": wi["status"], "width_law": wlaw["status"],
            "planted_check_target_arm": planted_target});
        let vals: Vec<&str> = parts
            .as_object()
            .unwrap()
            .values()
            .map(|v| v.as_str().unwrap())
            .collect();
        let v = if vals.contains(&"FAIL") {
            "rejected: extrapolation unsupported"
        } else if vals.contains(&"not evaluable") {
            "undetermined: not all tests evaluable"
        } else {
            "survives all five tests"
        };
        verdict_str.insert(sol, v.to_string());
        verdict.insert(
            sol.into(),
            json!({"tests": parts, "verdict": v,
                   "session_void_aa_determinism_failure": if have_aa { json!(!all_det) } else { Value::Null }}),
        );
    }
    let mut fals = Map::new();
    fals.insert("holdout".into(), Value::Object(hold.clone()));
    fals.insert("curvature".into(), Value::Object(curv));
    fals.insert("width_independence_of_count".into(), wi);
    fals.insert("width_law".into(), wlaw);
    fals.insert("planted_check".into(), planted);
    fals.insert(
        "verdict_for_extrapolation_target".into(),
        Value::Object(verdict),
    );

    // ---- extrapolation (joint bootstrap)
    let mut extrap = Map::new();
    for sol in SOLVERS {
        let key = format!("{}|{}", TARGET_ARM, sol);
        let slv = &size[&key]["row_ops"];
        let mut ent = Map::new();
        ent.insert("label".into(), json!("extrapolated, about 3 decades beyond the largest measured size (2^20 ~ 1.05e6 -> 1e9); single-host CPU seconds; no communication, distribution or memory beyond one host"));
        ent.insert("arm".into(), json!(TARGET_ARM));
        ent.insert("solver".into(), json!(sol));
        ent.insert("D_target".into(), json!(1_000_000_000i64));
        if slv["status"] != json!("ok") {
            ent.insert("status".into(), json!("insufficient sizes"));
            extrap.insert(sol.into(), Value::Object(ent));
            continue;
        }
        let mut refd: Option<i64> = None;
        for &d in ds_w.iter().rev() {
            if t3_by.contains_key(&(sol.to_string(), d)) {
                refd = Some(d);
                break;
            }
        }
        let refd = match refd {
            Some(d) => d,
            None => {
                ent.insert(
                    "status".into(),
                    json!(format!(
                        "no t(3): width law needs D>=2^{} with all four limb counts in a pod",
                        wmin
                    )),
                );
                extrap.insert(sol.into(), Value::Object(ent));
                continue;
            }
        };
        let t3 = &t3_by[&(sol.to_string(), refd)];
        let t3_point = median(t3);
        let pts: Vec<(i64, f64)> = valid
            .iter()
            .filter(|((l, so, _, _), _)| l == TARGET_ARM && so == sol)
            .filter_map(|((_, _, d, _), r)| {
                num(r, "row_ops").map(|v| (log2d(*d), (v / *d as f64).log2()))
            })
            .collect();
        let (a, slope) = ols_pts(&pts);
        let lg = D_TARGET.log2();
        let row_ops_pt = 2f64.powf(lg + a + slope * lg);
        let mut rng = Rng::new(BOOT_SEED);
        let (design, y) = line_design(&pts);
        let (_, groups) = groups_by_size(&pts);
        let coefs = bootstrap_coefs(&design, &y, &groups, b, &mut rng);
        let mut bt3 = Vec::with_capacity(b);
        for _ in 0..b {
            let draw: Vec<f64> = (0..t3.len())
                .map(|_| t3[rng.below(t3.len() as u64) as usize])
                .collect();
            bt3.push(median(&draw));
        }
        let row_ops_b: Vec<f64> = coefs
            .iter()
            .map(|c| 2f64.powf(lg + c[0] + c[1] * lg))
            .collect();
        let time_b: Vec<f64> = row_ops_b
            .iter()
            .zip(&bt3)
            .map(|(r, t)| r * t * 1e-9)
            .collect();
        let time_pt = row_ops_pt * t3_point * 1e-9;
        let pc = |v: &[f64]| [percentile(v, 2.5), percentile(v, 97.5)];
        let tci = pc(&time_b);
        ent.insert("status".into(), json!("ok"));
        ent.insert("size_law_sizes_log2".into(), slv["sizes_log2"].clone());
        ent.insert("a".into(), jf(a));
        ent.insert("beta".into(), jf(slope + 1.0));
        ent.insert("D_ref_for_t3_log2".into(), json!(log2d(refd)));
        ent.insert(
            "t3_ns_per_row_op_fitted_median_over_pods".into(),
            jf(t3_point),
        );
        ent.insert("t3_n_pods".into(), json!(t3.len()));
        ent.insert("t3_ci95_ns".into(), jvec(&pc(&bt3)));
        ent.insert("row_ops_at_target".into(), jf(row_ops_pt));
        ent.insert("row_ops_ci95".into(), jvec(&pc(&row_ops_b)));
        ent.insert("time_s".into(), jf(time_pt));
        ent.insert("time_s_ci95".into(), jvec(&tci));
        ent.insert("time_h".into(), jf(time_pt / 3600.0));
        ent.insert(
            "time_h_ci95".into(),
            jvec(&[tci[0] / 3600.0, tci[1] / 3600.0]),
        );
        ent.insert("time_days".into(), jf(time_pt / 86400.0));
        ent.insert(
            "time_days_ci95".into(),
            jvec(&[tci[0] / 86400.0, tci[1] / 86400.0]),
        );
        ent.insert("interval".into(), json!(format!(
            "95% percentile of the joint bootstrap (size law a,beta resampled over seeds within size; t(3) = median over resampled pods of the per-pod fitted c0+9*c2); {} resamples, generator seed {}",
            b, BOOT_SEED)));
        ent.insert("falsification_verdict".into(), json!(verdict_str[sol]));
        extrap.insert(sol.into(), Value::Object(ent));
    }

    notes.extend(STATIC_NOTES.iter().map(|s| s.to_string()));
    let mut ok_sizes: Vec<i64> = valid.iter().map(|(k, _)| log2d(k.2)).collect();
    ok_sizes.sort();
    ok_sizes.dedup();
    let abs = std::path::absolute(raw).unwrap_or_else(|_| raw.to_path_buf());
    let mut res = Map::new();
    res.insert(
        "protocol".into(),
        json!(
            "research/ic_techniques_20261012/wide_sparse_la/PROTOCOL.md + PROTOCOL_AMENDMENT_1.md"
        ),
    );
    res.insert("raw_dir".into(), json!(abs.to_string_lossy()));
    res.insert("pods".into(), Value::Object(pods_info));
    res.insert("integrity".into(), Value::Object(integrity));
    res.insert("sizes_log2_with_valid_data".into(), jints(&ok_sizes));
    res.insert("coverage_totals_over_cells".into(), Value::Object(totals_m));
    res.insert("cells_valid_after_repeat".into(), json!(repeated));
    res.insert("coverage".into(), Value::Object(coverage));
    res.insert("size_law".into(), Value::Object(size));
    res.insert("width_law".into(), Value::Object(width));
    res.insert("extrapolation".into(), Value::Object(extrap));
    res.insert("falsification".into(), Value::Object(fals));
    res.insert("aa".into(), Value::Object(aa));
    res.insert("wide_coeffs".into(), Value::Object(wide));
    res.insert("notes".into(), json!(notes));
    Analysis {
        res: Value::Object(res),
    }
}

// ------------------------------------------------------------------ markdown
fn f(v: &Value) -> String {
    match v.as_f64() {
        Some(x) => fmt_g(x, 3),
        None => "-".into(),
    }
}

fn pyval(v: &Value) -> String {
    match v {
        Value::Null => "None".into(),
        Value::Bool(true) => "True".into(),
        Value::Bool(false) => "False".into(),
        Value::String(s) => s.clone(),
        o => o.to_string(),
    }
}

fn pyints(v: &Value) -> String {
    format!(
        "[{}]",
        v.as_array()
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn make_md(res: &Value) -> String {
    let mut l: Vec<String> = vec![];
    l.push("# Wide-modulus sparse LA: analysis summary".into());
    let t = res["coverage_totals_over_cells"].as_object().unwrap();
    l.push(format!(
        "\nCoverage over (arm, solver, D, seed) cells: {}; sizes with valid data (log2): {}",
        t.iter()
            .filter(|(_, v)| v.as_i64() != Some(0))
            .map(|(k, v)| format!("{} {}", k, v))
            .collect::<Vec<_>>()
            .join(", "),
        pyints(&res["sizes_log2_with_valid_data"])
    ));
    for (k, p) in res["pods"].as_object().unwrap() {
        let mf: Vec<String> = p["missing_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s.as_str().unwrap().to_string())
            .collect();
        l.push(format!(
            "- {}: status {}, missing files {}",
            k,
            pyval(&p["status"]),
            if mf.is_empty() {
                "none".to_string()
            } else {
                pylist_str(&mf)
            }
        ));
    }
    l.push("\n## Size law, row_ops/D (beta = slope + 1, 95% bootstrap)\n".into());
    l.push("| arm | solver | beta | CI | core_dim beta | core_nnz beta | note |\n|:--|:--|--:|:--|--:|--:|:--|".into());
    for (k, e) in res["size_law"].as_object().unwrap() {
        let (arm, sol) = k.split_once('|').unwrap();
        let r = &e["row_ops"];
        if r["status"] != json!("ok") {
            if r["n_points"].as_i64().unwrap_or(0) != 0 {
                l.push(format!(
                    "| {} | {} | - | - | - | - | {} |",
                    arm,
                    sol,
                    r["status"].as_str().unwrap()
                ));
            }
            continue;
        }
        let (cd, cn) = (&e["core_dimension"], &e["core_nonzeros"]);
        l.push(format!(
            "| {} | {} | {} | [{}, {}] | {} | {} | {} |",
            arm,
            sol,
            f(&r["beta"]),
            f(&r["beta_ci95"][0]),
            f(&r["beta_ci95"][1]),
            f(&cd["beta"]),
            f(&cn["beta"]),
            if r["beta_in_expected_range_1.9_2.1"] == json!(true) {
                "in 1.9-2.1"
            } else {
                "outside 1.9-2.1"
            }
        ));
    }
    l.push("\n## Width law (t(L)/t(1), median [min, max] over pods)\n".into());
    let wt = res["width_law"]["table"].as_object().unwrap();
    if wt.is_empty() {
        l.push(format!(
            "No D >= 2^{} data.",
            res["width_law"]["min_log2_D"]
        ));
    } else {
        l.push("| solver | D | t(1) ns | fit c0, c2 | t(2)/t(1) | t(3)/t(1) | t(4)/t(1) |\n|:--|--:|--:|:--|:--|:--|:--|".into());
        for (k, e) in wt {
            let (sol, d) = k.split_once('|').unwrap();
            let lg = log2d(d.parse::<i64>().unwrap());
            let lf = &e["limb_fit"];
            let t1 = f(&e["arms"][CONTROL_ARM]["ns_per_row_op_median"]);
            if lf.get("summary_over_seeds").is_some() {
                let s = &lf["summary_over_seeds"];
                let rr: Vec<String> = ["2", "3", "4"]
                    .iter()
                    .map(|i| {
                        let r = &s["observed_ratio_t_L_over_t_1"][*i];
                        format!("{} [{}, {}]", f(&r["median"]), f(&r["min"]), f(&r["max"]))
                    })
                    .collect();
                l.push(format!(
                    "| {} | 2^{} | {} | {}, {} | {} | {} | {} |",
                    sol,
                    lg,
                    t1,
                    f(&s["c0"]["median"]),
                    f(&s["c2"]["median"]),
                    rr[0],
                    rr[1],
                    rr[2]
                ));
            } else {
                l.push(format!(
                    "| {} | 2^{} | {} | no 4-limb fit | | | |",
                    sol, lg, t1
                ));
            }
        }
    }
    l.push(format!(
        "\n## Extrapolation to D = 1e9, {} (EXTRAPOLATED)\n",
        TARGET_ARM
    ));
    for (sol, e) in res["extrapolation"].as_object().unwrap() {
        if e["status"] != json!("ok") {
            l.push(format!("- {}: {}", sol, pyval(&e["status"])));
        } else {
            l.push(format!(
                "- {}: row_ops {} [{}, {}], t(3) {} ns, time {} h [{}, {}] ({} days), law: {}",
                sol,
                f(&e["row_ops_at_target"]),
                f(&e["row_ops_ci95"][0]),
                f(&e["row_ops_ci95"][1]),
                f(&e["t3_ns_per_row_op_fitted_median_over_pods"]),
                f(&e["time_h"]),
                f(&e["time_h_ci95"][0]),
                f(&e["time_h_ci95"][1]),
                f(&e["time_days"]),
                e["falsification_verdict"].as_str().unwrap()
            ));
        }
    }
    l.push("\n## Falsification tests\n".into());
    let fa = &res["falsification"];
    l.push("| test | solver | status | detail |\n|:--|:--|:--|:--|".into());
    for sol in SOLVERS {
        let k = format!("{}|{}", TARGET_ARM, sol);
        let (h, c) = (&fa["holdout"][&k], &fa["curvature"][&k]);
        l.push(format!(
            "| hold-out ({}) | {} | {} | {} |",
            TARGET_ARM,
            sol,
            h["status"].as_str().unwrap(),
            h.get("reason").and_then(|r| r.as_str()).unwrap_or("")
        ));
        let detail = match c.get("reason").and_then(|r| r.as_str()) {
            Some(r) => r.to_string(),
            None => format!(
                "coef {} CI [{}, {}]",
                f(&c["coef_log2D_sq"]),
                f(&c["ci95"][0]),
                f(&c["ci95"][1])
            ),
        };
        l.push(format!(
            "| curvature ({}) | {} | {} | {} |",
            TARGET_ARM,
            sol,
            c["status"].as_str().unwrap(),
            detail
        ));
    }
    let w = &fa["width_independence_of_count"];
    l.push(format!(
        "| width independence of count | all | {} | worst max/min {} |",
        w["status"].as_str().unwrap(),
        f(&w["worst"]["max_over_min"])
    ));
    let wl = &fa["width_law"];
    l.push(format!(
        "| width law (same word) | all | {} | {} comparisons, {} beyond band |",
        wl["status"].as_str().unwrap(),
        wl["n_comparisons"],
        wl["violations"].as_array().unwrap().len()
    ));
    let pc = &fa["planted_check"];
    l.push(format!(
        "| planted check | all | {} | {} surviving unverified cells |",
        pc["status"].as_str().unwrap(),
        pc["n_surviving_repeat"]
    ));
    for (sol, v) in fa["verdict_for_extrapolation_target"].as_object().unwrap() {
        l.push(format!(
            "\nVerdict {}: {}",
            sol,
            v["verdict"].as_str().unwrap()
        ));
    }
    l.push("\n## A/A\n".into());
    l.push(format!(
        "Determinism (counted units identical across all A/A runs per arm): {}",
        pyval(&res["aa"]["all_identical"])
    ));
    for (k, p) in res["aa"]["per_pod"].as_object().unwrap() {
        l.push(format!(
            "- {} pod band {} ({})",
            k,
            f(&p["pod_band"]),
            p["arms"]
                .as_object()
                .unwrap()
                .iter()
                .map(|(a, bb)| format!(
                    "{} n={} band {}",
                    a,
                    bb["n_runs"],
                    f(&bb["band_max_over_min"])
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    l.push(format!(
        "\nNotes: {} ambiguity/handling notes in results.json `notes`.",
        res["notes"].as_array().unwrap().len()
    ));
    l.join("\n")
}

fn ascii_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii() {
            o.push(c);
        } else {
            let mut buf = [0u16; 2];
            for u in c.encode_utf16(&mut buf) {
                o.push_str(&format!("\\u{:04x}", u));
            }
        }
    }
    o
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (mut raw, mut out, mut md) = (None, None, false);
    let (mut resamples, mut wmin) = (N_BOOT, 16i64);
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--raw" => {
                i += 1;
                raw = Some(args[i].clone());
            }
            "--out" => {
                i += 1;
                out = Some(args[i].clone());
            }
            "--md" => md = true,
            "--resamples" => {
                i += 1;
                resamples = args[i].parse().expect("--resamples N");
            }
            "--width-min-log2" => {
                i += 1;
                wmin = args[i].parse().expect("--width-min-log2 N");
            }
            "-h" | "--help" => {
                println!("wide-sla-analysis --raw RAW_DIR --out results.json [--md] [--resamples N] [--width-min-log2 N]");
                return;
            }
            o => {
                eprintln!("unknown argument {o}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let (raw, out) = match (raw, out) {
        (Some(r), Some(o)) => (r, o),
        _ => {
            eprintln!("--raw and --out are required");
            std::process::exit(2);
        }
    };
    let mut an = analyse(Path::new(&raw), resamples, wmin);
    if resamples != N_BOOT || wmin != 16 {
        an.res["notes"].as_array_mut().unwrap().push(json!(format!(
            "NON-FROZEN test knobs used: resamples={} width_min_log2={}",
            resamples, wmin
        )));
    }
    let mut buf = Vec::new();
    let fmtr = serde_json::ser::PrettyFormatter::with_indent(b" ");
    let mut ser = serde_json::Serializer::with_formatter(&mut buf, fmtr);
    an.res.serialize(&mut ser).unwrap();
    fs::write(&out, ascii_escape(&String::from_utf8(buf).unwrap())).expect("write results");
    if md {
        println!("{}", make_md(&an.res));
    }
}
