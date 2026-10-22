//! Validation-gate tooling for the IC tool programme's rows (docs/VALIDATION_GATE.md).
//!
//! Independent of the code it checks: this binary uses none of the crate's
//! library modules. It carries its own GF(2^m) arithmetic and group law on the
//! Koblitz curve `y^2 + xy = x^3 + a x^2 + 1`, its own statistics, and reads
//! only the raw `*.price.json` / `*.isolation.jsonl` records.
//!
//! ```text
//! ic_validate certverify PATH... --out FILE      every certificate's scalar, with a negative control
//! ic_validate recompute MODE DIR --out FILE      MODE: round | rule | exploration | stages
//! ic_validate replay --bin DIR --scratch DIR --out FILE DIR...
//! ic_validate build [--check]                    the evidence directories' gate artifacts
//! ```
//!
//! Definitions (IC_TOOL_PROGRAM.md section 5; R01's PROTOCOL, "Accounting"):
//! a process's cold time is the median over its in-process repetitions of
//! `setup_ns + ic_online.wall_ns`; its figure is its first attempt (`rK`,
//! `rK-retry1`, ...) that the isolation record does not mark contended; a
//! paired comparison is the geometric mean of base/candidate ratios with a
//! 95% Student t interval on the log ratios.

use clap::{Parser, Subcommand};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

// ------------------------------------------------------------------ field

#[derive(Clone, Copy)]
struct Field {
    m: u32,
    modulus: u128,
}

impl Field {
    fn new(m: u32, low_terms: &[u32]) -> Self {
        let mut modulus = 1u128 << m;
        for &t in low_terms {
            modulus |= 1u128 << t;
        }
        Field { m, modulus }
    }

    fn reduce(&self, mut x: u128) -> u64 {
        while x >> self.m != 0 {
            let top = 127 - x.leading_zeros();
            x ^= self.modulus << (top - self.m);
        }
        x as u64
    }

    fn mul(&self, a: u64, b: u64) -> u64 {
        let (mut r, mut a, mut b) = (0u128, a as u128, b);
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            a <<= 1;
            b >>= 1;
        }
        self.reduce(r)
    }

    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0, "inverse of zero");
        // a^(2^m - 2)
        let (mut r, mut base, mut e) = (1u64, a, (1u128 << self.m) - 2);
        while e != 0 {
            if e & 1 == 1 {
                r = self.mul(r, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        r
    }
}

type Point = Option<(u64, u64)>;

struct Curve {
    f: Field,
    a: u64,
}

impl Curve {
    fn on_curve(&self, p: Point) -> bool {
        let Some((x, y)) = p else { return true };
        let f = &self.f;
        let lhs = f.mul(y, y) ^ f.mul(x, y);
        let x2 = f.mul(x, x);
        lhs == f.mul(x2, x) ^ (if self.a == 1 { x2 } else { 0 }) ^ 1
    }

    fn dbl(&self, p: Point) -> Point {
        let (x1, y1) = p?;
        if x1 == 0 {
            return None;
        }
        let f = &self.f;
        let lam = x1 ^ f.mul(y1, f.inv(x1));
        let x3 = f.mul(lam, lam) ^ lam ^ self.a;
        let y3 = f.mul(x1, x1) ^ f.mul(lam ^ 1, x3);
        Some((x3, y3))
    }

    fn add(&self, p: Point, q: Point) -> Point {
        let Some((x1, y1)) = p else { return q };
        let Some((x2, y2)) = q else { return p };
        if x1 == x2 {
            // q is p or -p
            return if y1 == y2 { self.dbl(p) } else { None };
        }
        let f = &self.f;
        let lam = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.mul(lam, lam) ^ lam ^ x1 ^ x2 ^ self.a;
        let y3 = f.mul(lam, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }

    fn mul(&self, k: u128, p: Point) -> Point {
        let mut r = None;
        for i in (0..128 - k.leading_zeros()).rev() {
            r = self.dbl(r);
            if (k >> i) & 1 == 1 {
                r = self.add(r, p);
            }
        }
        r
    }
}

fn big(v: &Value) -> u128 {
    match v {
        Value::String(s) => s.parse().unwrap_or_else(|_| panic!("not an integer: {s}")),
        Value::Number(n) => n
            .to_string()
            .parse()
            .unwrap_or_else(|_| panic!("not an integer: {n}")),
        _ => panic!("not an integer: {v}"),
    }
}

fn point(v: &Value) -> Point {
    Some((big(&v[0]) as u64, big(&v[1]) as u64))
}

/// `[s]G == T`, `[n]G == O`, both points on the curve, and `[s+1]G != T`.
fn check_certificate(
    cert: &Value,
    cache: &mut HashMap<String, (bool, bool)>,
) -> BTreeMap<&'static str, bool> {
    let c = &cert["curve"];
    let low: Vec<u32> = c["irreducible_low_terms"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| big(t) as u32)
        .collect();
    let curve = Curve {
        f: Field::new(big(&c["degree"]) as u32, &low),
        a: big(&c["curve_a"]) as u64,
    };
    let g = point(&c["generator"]);
    let key = c.to_string();
    let (g_on, order_ok) = *cache.entry(key).or_insert_with(|| {
        (
            curve.on_curve(g),
            curve.mul(big(&c["subgroup_order"]), g).is_none(),
        )
    });
    let t = point(&cert["target"]);
    let s = big(&cert["scalar"]);
    BTreeMap::from([
        ("generator_on_curve", g_on),
        ("order_kills_generator", order_ok),
        ("target_on_curve", curve.on_curve(t)),
        ("scalar_ok", curve.mul(s, g) == t),
        ("negative_control_rejected", curve.mul(s + 1, g) != t),
    ])
}

// ------------------------------------------------------------- statistics

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    let tiny = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c: f64 = 1.0;
    let mut d = 1.0 - qab * x / qap;
    d = 1.0 / if d.abs() > tiny { d } else { tiny };
    let mut h = d;
    for m in 1..400 {
        let m = m as f64;
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        d = 1.0 / if d.abs() > tiny { d } else { tiny };
        c = if c.abs() > tiny { 1.0 + aa / c } else { tiny };
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        d = 1.0 / if d.abs() > tiny { d } else { tiny };
        c = if c.abs() > tiny { 1.0 + aa / c } else { tiny };
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 1e-15 {
            break;
        }
    }
    h
}

fn ln_gamma(x: f64) -> f64 {
    // Lanczos, g = 7, n = 9
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = C[0];
    let t = x + 7.5;
    for (i, c) in C.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

fn ibeta(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let lbt = ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (-x).ln_1p();
    if x < (a + 1.0) / (a + b + 2.0) {
        lbt.exp() * betacf(a, b, x) / a
    } else {
        1.0 - lbt.exp() * betacf(b, a, 1.0 - x) / b
    }
}

fn t_cdf(t: f64, df: f64) -> f64 {
    let tail = 0.5 * ibeta(df / 2.0, 0.5, df / (df + t * t));
    if t > 0.0 {
        1.0 - tail
    } else {
        tail
    }
}

fn t_quantile(p: f64, df: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 1000.0);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if t_cdf(mid, df) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = s.len();
    if n % 2 == 1 {
        s[n / 2]
    } else {
        (s[n / 2 - 1] + s[n / 2]) / 2.0
    }
}

struct Ci {
    n: usize,
    geomean: Option<f64>,
    lo: Option<f64>,
    hi: Option<f64>,
}

fn geo_ci(ratios: &[f64]) -> Ci {
    let n = ratios.len();
    if n == 0 {
        return Ci {
            n,
            geomean: None,
            lo: None,
            hi: None,
        };
    }
    let logs: Vec<f64> = ratios.iter().map(|r| r.ln()).collect();
    let m = mean(&logs);
    let (mut lo, mut hi) = (None, None);
    if n > 1 {
        let var = logs.iter().map(|l| (l - m) * (l - m)).sum::<f64>() / (n - 1) as f64;
        let h = t_quantile(0.975, (n - 1) as f64) * var.sqrt() / (n as f64).sqrt();
        lo = Some((m - h).exp());
        hi = Some((m + h).exp());
    }
    Ci {
        n,
        geomean: Some(m.exp()),
        lo,
        hi,
    }
}

fn round_to(x: f64, places: usize) -> f64 {
    format!("{x:.places$}").parse().unwrap()
}

// ---------------------------------------------------------------- records

fn repo() -> PathBuf {
    let mut p = std::env::current_dir().unwrap();
    while !p.join("Cargo.toml").exists() || !p.join("research/ic_tool_program").exists() {
        assert!(p.pop(), "run inside the repository");
    }
    p
}

fn sha256_bytes(b: &[u8]) -> String {
    hex::encode(Sha256::digest(b))
}

fn sha256_file(p: &Path) -> String {
    sha256_bytes(&fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display())))
}

fn rel_to(p: &Path, base: &Path) -> String {
    let p: Vec<_> = p.components().collect();
    let b: Vec<_> = base.components().collect();
    let common = p.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let mut out = PathBuf::new();
    for _ in common..b.len() {
        out.push("..");
    }
    for c in &p[common..] {
        out.push(c);
    }
    out.to_string_lossy().into_owned()
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, out);
        } else {
            out.push(p);
        }
    }
}

/// Every file of a `.tar.xz` by member name (extracted with the system `tar`).
fn untar(tar: &Path) -> Vec<(String, Vec<u8>)> {
    let tmp = std::env::temp_dir().join(format!(
        "ic_validate-{}-{}",
        std::process::id(),
        sha256_file(tar)
    ));
    if !tmp.exists() {
        fs::create_dir_all(&tmp).unwrap();
        let ok = Command::new("tar")
            .arg("-xJf")
            .arg(tar)
            .arg("-C")
            .arg(&tmp)
            .status()
            .unwrap()
            .success();
        assert!(ok, "tar -xJf {}", tar.display());
    }
    let mut files = Vec::new();
    walk(&tmp, &mut files);
    files
        .into_iter()
        .map(|p| (rel_to(&p, &tmp), fs::read(&p).unwrap()))
        .collect()
}

/// Every record file of an evidence directory: its tarballs, or else `runs/`.
fn record_files(d: &Path) -> (Vec<(String, Vec<u8>)>, Vec<Value>) {
    let repo = repo();
    let mut tars: Vec<PathBuf> = fs::read_dir(d)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.to_string_lossy().ends_with(".tar.xz"))
        .collect();
    tars.sort();
    let mut files = Vec::new();
    let mut inputs = Vec::new();
    if !tars.is_empty() {
        for t in &tars {
            inputs.push(json!({"path": rel_to(t, &repo), "sha256": sha256_file(t)}));
            files.extend(untar(t));
        }
    } else {
        let runs = d.join("runs");
        inputs.push(json!({"path": format!("{}/", rel_to(&runs, &repo)), "sha256": null}));
        let mut ps = Vec::new();
        walk(&runs, &mut ps);
        for p in ps {
            files.push((format!("runs/{}", rel_to(&p, &runs)), fs::read(&p).unwrap()));
        }
    }
    files.retain(|(n, _)| n.ends_with(".price.json") || n.ends_with(".isolation.jsonl"));
    files.sort_by(|a, b| a.0.cmp(&b.0));
    (files, inputs)
}

fn last_isolation(raw: &[u8]) -> Option<Value> {
    String::from_utf8_lossy(raw)
        .lines()
        .rfind(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
}

fn is_contended(iso: &Value) -> bool {
    iso.get("run")
        .and_then(|r| r.get("contended"))
        .or_else(|| iso.get("contended"))
        .and_then(Value::as_bool)
        == Some(true)
}

struct Records {
    price: BTreeMap<String, Value>,
    contended: HashMap<String, bool>,
    isolation: HashMap<String, Value>,
    inputs: Vec<Value>,
}

impl Records {
    fn load(d: &Path) -> Self {
        let (files, inputs) = record_files(d);
        let mut r = Records {
            price: BTreeMap::new(),
            contended: HashMap::new(),
            isolation: HashMap::new(),
            inputs,
        };
        for (name, raw) in files {
            if let Some(stem) = name.strip_suffix(".price.json") {
                r.price
                    .insert(stem.to_string(), serde_json::from_slice(&raw).unwrap());
            } else if let Some(stem) = name.strip_suffix(".isolation.jsonl") {
                if let Some(iso) = last_isolation(&raw) {
                    r.contended.insert(stem.to_string(), is_contended(&iso));
                    r.isolation.insert(stem.to_string(), iso);
                }
            }
        }
        r
    }

    /// The first attempt of process `stem` the isolation record does not mark contended.
    fn first_clean(&self, stem: &str) -> Option<&Value> {
        let attempts =
            std::iter::once(stem.to_string()).chain((1..10).map(|i| format!("{stem}-retry{i}")));
        for a in attempts {
            match self.price.get(&a) {
                None if a == stem => continue,
                None => break,
                Some(rec) => {
                    if !self.contended.get(&a).copied().unwrap_or(false) {
                        return Some(rec);
                    }
                }
            }
        }
        None
    }

    fn stems(&self, prefix: &str) -> Vec<String> {
        let mut out = BTreeSet::new();
        for k in self.price.keys().filter(|k| k.starts_with(prefix)) {
            out.insert(strip_retry(k).to_string());
        }
        out.into_iter().collect()
    }
}

fn strip_retry(k: &str) -> &str {
    match k.rfind("-retry") {
        Some(i) if k[i + 6..].chars().all(|c| c.is_ascii_digit()) && i + 6 < k.len() => &k[..i],
        _ => k,
    }
}

fn f(v: &Value) -> f64 {
    v.as_f64().unwrap_or_else(|| panic!("not a number: {v}"))
}

fn cold_ns(rec: &Value) -> f64 {
    let reps: Vec<f64> = rec["repetitions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| f(&r["setup_ns"]) + f(&r["ic_online"]["wall_ns"]))
        .collect();
    median(&reps)
}

// ----------------------------------------------------------------- report

struct Report {
    what: String,
    headline: Vec<Value>,
    raw: Vec<Value>,
    notes: Vec<String>,
    extra: Map<String, Value>,
}

impl Report {
    fn new(what: String) -> Self {
        Report {
            what,
            headline: Vec::new(),
            raw: Vec::new(),
            notes: Vec::new(),
            extra: Map::new(),
        }
    }

    fn check(&mut self, name: String, claimed: Option<f64>, recomputed: Option<f64>, rel_tol: f64) {
        match (claimed, recomputed) {
            (Some(c), Some(r)) => self
                .headline
                .push(json!({"name": name, "claimed": c, "recomputed": r, "rel_tol": rel_tol})),
            _ => self.notes.push(format!(
                "{name}: claimed {claimed:?}, recomputed {recomputed:?} (not compared)"
            )),
        }
    }

    fn disagreements(&self) -> Vec<Value> {
        self.headline
            .iter()
            .filter(|h| {
                let (c, r, tol) = (f(&h["claimed"]), f(&h["recomputed"]), f(&h["rel_tol"]));
                (c - r).abs() > tol * c.abs().max(r.abs()).max(1e-300)
            })
            .cloned()
            .collect()
    }

    fn doc(&self) -> Value {
        let bad = self.disagreements();
        let mut d = json!({
            "schema": "ic-tool-programme-validator-report/v2",
            "what": self.what,
            "tool": "src/bin/ic_validate.rs (ic_validate recompute)",
            "independent": true,
            "independence": "written for the validation gate; uses none of the crate's library code and none of icprog's; reads only the raw records",
            "status": if !self.headline.is_empty() && bad.is_empty() { "pass" } else { "fail" },
            "raw_data": self.raw,
            "headline": self.headline,
            "disagreements": bad,
            "notes": self.notes,
        });
        for (k, v) in &self.extra {
            d[k] = v.clone();
        }
        d
    }
}

fn opt(v: &Value) -> Option<f64> {
    v.as_f64()
}

fn compare_interval(rep: &mut Report, label: &str, published: &Value, ratios: &[f64]) {
    let ci = geo_ci(ratios);
    rep.check(
        format!("{label} n"),
        opt(&published["n"]),
        Some(ci.n as f64),
        0.0,
    );
    rep.check(
        format!("{label} geomean"),
        opt(&published["geomean"]),
        ci.geomean,
        1e-9,
    );
    rep.check(format!("{label} lo"), opt(&published["lo"]), ci.lo, 1e-9);
    rep.check(format!("{label} hi"), opt(&published["hi"]), ci.hi, 1e-9);
}

fn paired_ratios(recs: &Records, base: &str, cand: &str) -> (Vec<f64>, usize) {
    let (mut ratios, mut missing) = (Vec::new(), 0);
    for stem in recs.stems(base) {
        let other = format!("{cand}{}", &stem[base.len()..]);
        match (recs.first_clean(&stem), recs.first_clean(&other)) {
            (Some(b), Some(c)) => ratios.push(cold_ns(b) / cold_ns(c)),
            _ => missing += 1,
        }
    }
    (ratios, missing)
}

fn read_json(p: &Path) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()))).unwrap()
}

fn mode_round(d: &Path) -> Report {
    let repo = repo();
    let ana = read_json(&d.join("analysis.json"));
    let mut rep = Report::new(format!(
        "{}: every cold-time interval of analysis.json (A/A, suite rows, holdouts)",
        rel_to(d, &repo)
    ));
    let recs = Records::load(d);
    rep.raw = recs.inputs.clone();
    rep.raw.push(json!({"path": rel_to(&d.join("analysis.json"), &repo), "sha256": sha256_file(&d.join("analysis.json"))}));
    for row in ana["aa"].as_array().unwrap() {
        let slug = row["slug"].as_str().unwrap();
        let (r, _) = paired_ratios(
            &recs,
            &format!("runs/aa/A/{slug}/"),
            &format!("runs/aa/A2/{slug}/"),
        );
        compare_interval(&mut rep, &format!("A/A {slug} cold"), &row["cold"], &r);
    }
    for (key, sub) in [("suite", "compare"), ("holdouts", "holdout")] {
        for row in ana[key].as_array().unwrap() {
            let slug = row["slug"].as_str().unwrap();
            let (r, miss) = paired_ratios(
                &recs,
                &format!("runs/{sub}/base/{slug}/"),
                &format!("runs/{sub}/cand/{slug}/"),
            );
            compare_interval(&mut rep, &format!("{key} {slug} cold"), &row["cold"], &r);
            rep.check(
                format!("{key} {slug} missing pairs"),
                opt(&row["cold"]["missing_pairs"]),
                Some(miss as f64),
                0.0,
            );
        }
    }
    rep.extra.insert("records".into(), json!(recs.price.len()));
    rep
}

fn size_of(stem: &str) -> Option<String> {
    // `runs/k<a>n<n>/T<t>-R1`
    let parts: Vec<&str> = stem.split('/').collect();
    if parts.len() == 3
        && parts[0] == "runs"
        && parts[1].starts_with('k')
        && parts[2].starts_with('T')
        && parts[2].ends_with("-R1")
    {
        Some(parts[1].to_string())
    } else {
        None
    }
}

fn mode_rule(d: &Path) -> Report {
    let repo = repo();
    let ana = read_json(&d.join("analysis.json"));
    let mut rep = Report::new(format!(
        "{}: the S figures and cold ratios by size (analysis.json `sizes`)",
        rel_to(d, &repo)
    ));
    let recs = Records::load(d);
    rep.raw = recs.inputs.clone();
    rep.raw.push(json!({"path": rel_to(&d.join("analysis.json"), &repo), "sha256": sha256_file(&d.join("analysis.json"))}));
    let mut by_size: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for stem in recs.stems("runs/") {
        if let (Some(size), Some(rec)) = (size_of(&stem), recs.first_clean(&stem)) {
            by_size.entry(size).or_default().push(rec);
        }
    }
    let mut sizes = Vec::new();
    for s in ana["sizes"].as_array().unwrap() {
        let key = format!("k{}n{}", s["a"], s["n"]);
        let rows = by_size.get(&key).cloned().unwrap_or_default();
        let col = |k: &str| {
            rows.iter()
                .map(|r| f(&r["median"][k]))
                .collect::<Vec<f64>>()
        };
        let setup = median(&col("s_setup"));
        let ic_on = mean(&col("s_ic_online"));
        let rho_on = mean(&col("s_rho_online"));
        let rho_setup = median(&col("s_rho_setup"));
        let rho_model = mean(&col("s_rho_model"));
        let c = s["curve"].as_str().unwrap();
        rep.check(
            format!("{c} R1 rows"),
            opt(&s["rows"]["R1"]),
            Some(rows.len() as f64),
            0.0,
        );
        rep.check(
            format!("{c} s_setup"),
            opt(&s["s_setup"]),
            Some(setup),
            1e-9,
        );
        rep.check(
            format!("{c} s_ic_online_mean"),
            opt(&s["s_ic_online_mean"]),
            Some(ic_on),
            1e-9,
        );
        rep.check(
            format!("{c} s_rho_online_mean"),
            opt(&s["s_rho_online_mean"]),
            Some(rho_on),
            1e-9,
        );
        rep.check(
            format!("{c} s_rho_setup"),
            opt(&s["s_rho_setup"]),
            Some(rho_setup),
            1e-9,
        );
        rep.check(
            format!("{c} s_rho_model_mean"),
            opt(&s["s_rho_model_mean"]),
            Some(rho_model),
            1e-9,
        );
        rep.check(
            format!("{c} cold ratio IC/rho"),
            opt(&s["cold_ratio_ic_over_rho"]["value"]),
            Some((setup + ic_on) / (rho_setup + rho_on)),
            1e-9,
        );
        rep.check(
            format!("{c} online speedup"),
            opt(&s["online_speedup"]["mean_ratio"]),
            Some(rho_on / ic_on),
            1e-9,
        );
        let canon = median(
            &rows
                .iter()
                .map(|r| {
                    r["rho_policy"]["counters"]["canonicalisations_uncharged"]
                        .as_f64()
                        .unwrap_or(0.0)
                })
                .collect::<Vec<_>>(),
        );
        sizes.push(
            json!({"curve": c, "log2_r": s["log2_r"], "r": s["r"], "s_ic_cold": setup + ic_on,
            "s_rho_as_built": rho_setup + rho_on, "s_rho_canonical": rho_setup + rho_model,
            "rho_canonicalisations_uncharged_median": canon}),
        );
    }
    // The leaderboard's rows from this analysis show the reference at the canonical step
    // (scripts/build_ic_leaderboard.py): recompute what they display.
    let board_path = repo.join("docs/ic/leaderboard.json");
    let source = rel_to(&d.join("analysis.json"), &repo);
    if let Ok(raw) = fs::read(&board_path) {
        let board: Value = serde_json::from_slice(&raw).unwrap();
        let mut shown = 0;
        for row in board["board"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|r| r["source"] == source.as_str())
        {
            let Some(sz) = sizes.iter().find(|x| x["curve"] == row["slug"]) else {
                continue;
            };
            let slug = row["slug"].as_str().unwrap();
            let (ic, canon, built) = (
                f(&sz["s_ic_cold"]),
                f(&sz["s_rho_canonical"]),
                f(&sz["s_rho_as_built"]),
            );
            rep.check(
                format!("leaderboard {slug} S, IC"),
                opt(&row["best"]["s"]),
                Some(ic),
                1e-9,
            );
            rep.check(
                format!("leaderboard {slug} S, reference (canonical step)"),
                opt(&row["reference_s"]),
                Some(canon),
                1e-9,
            );
            rep.check(
                format!("leaderboard {slug} x reference (canonical step)"),
                opt(&row["best"]["ratio_rho"]),
                Some(ic / canon),
                1e-9,
            );
            rep.check(
                format!("leaderboard {slug} S, reference as built"),
                opt(&row["reference_s_as_built"]),
                Some(built),
                1e-9,
            );
            rep.check(
                format!("leaderboard {slug} x reference as built"),
                opt(&row["best"]["ratio_rho_as_built"]),
                Some(ic / built),
                1e-9,
            );
            shown += 1;
        }
        if shown > 0 {
            // cited, not pinned: the file changes whenever any other row does
            rep.raw
                .push(json!({"path": rel_to(&board_path, &repo), "sha256": null}));
        }
    }
    rep.extra.insert("sizes".into(), Value::Array(sizes));
    rep.extra.insert("records".into(), json!(recs.price.len()));
    rep
}

fn tsv(p: &Path) -> Vec<BTreeMap<String, String>> {
    let text = fs::read_to_string(p).unwrap();
    let mut lines = text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty());
    let header: Vec<String> = lines
        .next()
        .unwrap()
        .split('\t')
        .map(String::from)
        .collect();
    lines
        .map(|l| {
            header
                .iter()
                .cloned()
                .zip(l.split('\t').map(String::from))
                .collect()
        })
        .collect()
}

fn files_like(d: &Path, prefix: &str) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = fs::read_dir(d)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy();
            n.starts_with(prefix) && n.ends_with(".tsv")
        })
        .collect();
    v.sort();
    v
}

fn mode_exploration(d: &Path) -> Report {
    let repo = repo();
    let mut rep = Report::new(format!(
        "{}: every paired cold time and summary row",
        rel_to(d, &repo)
    ));
    let recs = Records::load(d);
    rep.raw = recs.inputs.clone();
    let mut by_tail: HashMap<String, String> = HashMap::new();
    for stem in recs.price.keys() {
        let tail = stem
            .split_once("runs/")
            .map(|(_, t)| t)
            .unwrap_or(stem)
            .to_string();
        // the timed pairs, not a stage-probe tree that reuses the names
        let replace = by_tail
            .get(&tail)
            .map(|s| s.contains("probe"))
            .unwrap_or(true);
        if replace {
            by_tail.insert(tail, stem.clone());
        }
    }
    for pf in files_like(d, "pairs") {
        rep.raw
            .push(json!({"path": rel_to(&pf, &repo), "sha256": sha256_file(&pf)}));
        let pname = pf.file_name().unwrap().to_string_lossy().into_owned();
        let rows = tsv(&pf);
        let arms: Vec<String> = rows[0]
            .keys()
            .filter_map(|c| c.strip_suffix("_cold_ns").map(String::from))
            .collect();
        // the column order is the file's: a over b
        let header: Vec<String> = fs::read_to_string(&pf)
            .unwrap()
            .lines()
            .next()
            .unwrap()
            .split('\t')
            .map(String::from)
            .collect();
        let mut ordered: Vec<String> = header
            .iter()
            .filter_map(|c| c.strip_suffix("_cold_ns").map(String::from))
            .collect();
        ordered.retain(|a| arms.contains(a));
        let (a, b) = (ordered[0].clone(), ordered[1].clone());
        let mut groups: BTreeMap<String, Vec<(f64, bool)>> = BTreeMap::new();
        for row in &rows {
            let curve = &row["curve"];
            let mut vals = HashMap::new();
            for arm in [&a, &b] {
                let Some(stem) = locate(&recs, &by_tail, arm, curve, &row["row"], &row["round"])
                else {
                    rep.notes.push(format!(
                        "{pname}: no raw record for {arm} {curve} {} {}",
                        row["row"], row["round"]
                    ));
                    continue;
                };
                let v = cold_ns(&recs.price[&stem]);
                vals.insert(arm.clone(), v);
                let label = format!("{pname} {arm} {curve} {} {}", row["row"], row["round"]);
                rep.check(
                    format!("{label} cold_ns"),
                    row[&format!("{arm}_cold_ns")].parse().ok(),
                    Some(v),
                    1e-9,
                );
                if let (Some(flag), Some(cont)) = (
                    row.get(&format!("{arm}_contended")),
                    recs.contended.get(&stem),
                ) {
                    rep.check(
                        format!("{label} contended"),
                        Some((flag == "true") as u8 as f64),
                        Some(*cont as u8 as f64),
                        0.0,
                    );
                }
            }
            if vals.len() == 2 {
                let clean = row.get(&format!("{a}_contended")).map(String::as_str) == Some("false")
                    && row.get(&format!("{b}_contended")).map(String::as_str) == Some("false");
                groups
                    .entry(curve.clone())
                    .or_default()
                    .push((vals[&a] / vals[&b], clean));
            }
        }
        let sf = pf.with_file_name(pname.replacen("pairs", "summary", 1));
        if sf.exists() {
            rep.raw
                .push(json!({"path": rel_to(&sf, &repo), "sha256": sha256_file(&sf)}));
            let sname = sf.file_name().unwrap().to_string_lossy().into_owned();
            for srow in tsv(&sf) {
                let g = groups.get(&srow["curve"]).cloned().unwrap_or_default();
                let clean: Vec<f64> = g.iter().filter(|(_, c)| *c).map(|(r, _)| *r).collect();
                let ci = geo_ci(&clean);
                let ratio_col = srow.keys().find(|c| c.contains("_over_")).unwrap().clone();
                let c = &srow["curve"];
                // the summaries print three decimals
                rep.check(
                    format!("{sname} {c} clean pairs"),
                    srow["clean_pairs"].parse().ok(),
                    Some(ci.n as f64),
                    0.0,
                );
                rep.check(
                    format!("{sname} {c} {ratio_col}"),
                    srow[&ratio_col].parse().ok(),
                    ci.geomean.map(|x| round_to(x, 3)),
                    0.0,
                );
                if ci.lo.is_some() {
                    rep.check(
                        format!("{sname} {c} lo95"),
                        srow["lo95"].parse().ok(),
                        ci.lo.map(|x| round_to(x, 3)),
                        0.0,
                    );
                    rep.check(
                        format!("{sname} {c} hi95"),
                        srow["hi95"].parse().ok(),
                        ci.hi.map(|x| round_to(x, 3)),
                        0.0,
                    );
                }
                if let Some(all) = srow.get("all_geomean") {
                    let every: Vec<f64> = g.iter().map(|(r, _)| *r).collect();
                    rep.check(
                        format!("{sname} {c} all_geomean"),
                        all.parse().ok(),
                        geo_ci(&every).geomean.map(|x| round_to(x, 3)),
                        0.0,
                    );
                }
            }
        }
    }
    rep.extra.insert("records".into(), json!(recs.price.len()));
    rep
}

fn locate(
    recs: &Records,
    by_tail: &HashMap<String, String>,
    arm: &str,
    curve: &str,
    row: &str,
    rnd: &str,
) -> Option<String> {
    let n: String = curve
        .strip_prefix("icv1-f2m")?
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect();
    for a in ["0", "1"] {
        if let Some(stem) = by_tail.get(&format!("{arm}/k{a}n{n}/{row}/{rnd}")) {
            if recs.price[stem]["curve"] == curve {
                return Some(stem.clone());
            }
        }
    }
    None
}

fn mode_stages(d: &Path) -> Report {
    let repo = repo();
    let mut rep = Report::new(format!(
        "{}: the scan's stages and the phase shares (stages.tsv, shares.tsv)",
        rel_to(d, &repo)
    ));
    let recs = Records::load(d);
    rep.raw = recs.inputs.clone();
    let mut per: BTreeMap<String, Vec<BTreeMap<&'static str, f64>>> = BTreeMap::new();
    for (stem, rec) in &recs.price {
        if recs.contended.get(stem).copied().unwrap_or(true) || rec.get("scan_probes").is_none() {
            continue;
        }
        let p = &rec["scan_probes"];
        let (hz, n) = (f(&p["tsc_hz"]), f(&p["counts"]["summands"]));
        let mut x = BTreeMap::new();
        for k in ["subtract", "key", "filter", "admitted", "trial"] {
            x.insert(k, f(&p["stages"][k]) / hz * 1e9 / n);
        }
        let reps = rec["repetitions"].as_array().unwrap();
        let sum = |g: &dyn Fn(&Value) -> f64| reps.iter().map(g).sum::<f64>();
        let col = sum(&|r| f(&r["setup_phases_ns"]["collect"]));
        let cold = sum(&|r| f(&r["setup_ns"]) + f(&r["ic_online"]["wall_ns"]));
        let stages: f64 = p["stages"].as_object().unwrap().values().map(f).sum();
        x.insert("admitted_a_summand", f(&p["counts"]["admitted"]) / n);
        x.insert("scan_of_collection", stages / hz * 1e9 / col);
        x.insert("collection_of_cold", col / cold);
        x.insert(
            "build_of_cold",
            sum(&|r| f(&r["setup_phases_ns"]["build"])) / cold,
        );
        x.insert(
            "selection_of_cold",
            sum(&|r| f(&r["setup_phases_ns"]["select"])) / cold,
        );
        per.entry(rec["curve"].as_str().unwrap().to_string())
            .or_default()
            .push(x);
    }
    for name in ["stages.tsv", "shares.tsv"] {
        let p = d.join(name);
        rep.raw
            .push(json!({"path": rel_to(&p, &repo), "sha256": sha256_file(&p)}));
        for row in tsv(&p) {
            let c = &row["curve"];
            let g = per.get(c).cloned().unwrap_or_default();
            let m = |k: &str| mean(&g.iter().map(|x| x[k]).collect::<Vec<_>>());
            rep.check(
                format!("{name} {c} processes"),
                row["processes"].parse().ok(),
                Some(g.len() as f64),
                0.0,
            );
            if name == "stages.tsv" {
                let scan = m("subtract") + m("key") + m("filter") + m("admitted");
                for (col, v) in [
                    ("subtract_ns", m("subtract")),
                    ("key_ns", m("key")),
                    ("filter_ns", m("filter")),
                    ("admitted_ns", m("admitted")),
                    ("scan_ns", scan),
                    ("trial_ns", m("trial")),
                ] {
                    rep.check(
                        format!("{name} {c} {col}"),
                        row[col].parse().ok(),
                        Some(round_to(v, 2)),
                        0.0,
                    );
                }
                rep.check(
                    format!("{name} {c} admitted_a_summand"),
                    row["admitted_a_summand"].parse().ok(),
                    Some(round_to(m("admitted_a_summand"), 4)),
                    0.0,
                );
            } else {
                for k in [
                    "scan_of_collection",
                    "collection_of_cold",
                    "build_of_cold",
                    "selection_of_cold",
                ] {
                    rep.check(
                        format!("{name} {c} {k}"),
                        row[k].parse().ok(),
                        Some(round_to(m(k), 3)),
                        0.0,
                    );
                }
            }
        }
    }
    rep.extra.insert("records".into(), json!(recs.price.len()));
    rep
}

fn recompute(mode: &str, d: &Path) -> Report {
    match mode {
        "round" => mode_round(d),
        "rule" => mode_rule(d),
        "exploration" => mode_exploration(d),
        "stages" => mode_stages(d),
        _ => panic!("mode {mode}"),
    }
}

// ------------------------------------------------------------- certverify

fn certverify(paths: &[PathBuf]) -> Value {
    let repo = repo();
    let mut cache = HashMap::new();
    let (mut records, mut certificates) = (0usize, 0usize);
    let mut failures = Vec::new();
    let mut degrees = BTreeSet::new();
    let mut inputs = Vec::new();
    for root in paths {
        let root = fs::canonicalize(root).unwrap();
        let files: Vec<(String, Vec<u8>)> = if root.is_dir() {
            let mut ps = Vec::new();
            walk(&root, &mut ps);
            ps.into_iter()
                .map(|p| (rel_to(&p, &root), fs::read(&p).unwrap()))
                .collect()
        } else {
            untar(&root)
        };
        let mut n_in = 0;
        for (name, raw) in files.iter().filter(|(n, _)| n.ends_with(".price.json")) {
            n_in += 1;
            records += 1;
            let doc: Value = serde_json::from_slice(raw).unwrap();
            let certs = &doc["certificates"];
            let arms: Vec<&str> = ["ic", "rho"]
                .into_iter()
                .filter(|a| certs.get(*a).is_some_and(Value::is_object))
                .collect();
            if arms.is_empty() {
                failures.push(json!({"record": name, "reason": "no certificate"}));
                continue;
            }
            for arm in &arms {
                let r = check_certificate(&certs[*arm], &mut cache);
                certificates += 1;
                degrees.insert(big(&certs[*arm]["curve"]["degree"]) as u32);
                if r.values().any(|ok| !ok) {
                    failures.push(json!({"record": name, "arm": arm, "checks": r}));
                }
            }
            let recovered: Vec<String> = doc["counts"]["recovered"]
                .as_array()
                .map(|v| v.iter().map(|x| big(x).to_string()).collect())
                .unwrap_or_default();
            if !recovered.is_empty()
                && arms
                    .iter()
                    .any(|a| !recovered.contains(&big(&certs[*a]["scalar"]).to_string()))
            {
                failures.push(json!({"record": name, "reason": "recovered != certificate scalar"}));
            }
        }
        let digest = if root.is_file() {
            json!(sha256_file(&root))
        } else {
            Value::Null
        };
        inputs.push(json!({"path": rel_to(&root, &repo), "records": n_in, "sha256": digest}));
    }
    json!({
        "tool": "src/bin/ic_validate.rs (ic_validate certverify)",
        "independent": true,
        "checks": "[s]G = T, [n]G = O, G and T on the curve, and [s+1]G != T (a negative control), for every ic and rho certificate",
        "status": if records > 0 && failures.is_empty() { "pass" } else { "fail" },
        "records": records, "certificates": certificates, "field_degrees": degrees,
        "failures": failures, "inputs": inputs,
    })
}

// ------------------------------------------------------------------ replay

const PER_DIR: usize = 4;
const COMPARED: [&str; 5] = [
    "counts",
    "rho_counts",
    "all_verified",
    "ic_and_rho_agree",
    "counts_identical_across_repetitions",
];

fn seed(repo: &Path) -> String {
    let text = fs::read_to_string(repo.join("research/ic_tool_program/validation/REPLAY_SEED.md"))
        .unwrap();
    let start = text.find("**Seed:** `").unwrap() + "**Seed:** `".len();
    text[start..start + text[start..].find('`').unwrap()].to_string()
}

fn certs_of(doc: &Value) -> Value {
    let mut m = Map::new();
    for arm in ["ic", "rho"] {
        if let Some(c) = doc["certificates"].get(arm) {
            m.insert(arm.into(), json!({"curve": c["curve"], "target": c["target"], "scalar": c["scalar"], "statement": c["statement"]}));
        }
    }
    Value::Object(m)
}

fn replay(bin: &Path, scratch: &Path, dirs: &[PathBuf]) -> Value {
    let repo = repo();
    let s = seed(&repo);
    let mut bin_sha = HashMap::new();
    for e in fs::read_dir(bin).unwrap().flatten() {
        if e.path().is_file() {
            bin_sha.insert(
                e.file_name().to_string_lossy().into_owned(),
                sha256_file(&e.path()),
            );
        }
    }
    let mut results = Vec::new();
    for d in dirs {
        let d = fs::canonicalize(d).unwrap();
        let ev = rel_to(&d, &repo);
        let (files, _) = record_files(&d);
        let iso: HashMap<&str, &Vec<u8>> = files
            .iter()
            .filter_map(|(n, b)| n.strip_suffix(".isolation.jsonl").map(|s| (s, b)))
            .collect();
        let mut clean: Vec<(String, Value, Value)> = Vec::new();
        for (name, raw) in &files {
            let Some(stem) = name.strip_suffix(".price.json") else {
                continue;
            };
            let Some(last) = iso.get(stem).and_then(|b| last_isolation(b)) else {
                continue;
            };
            if !is_contended(&last) {
                clean.push((name.clone(), serde_json::from_slice(raw).unwrap(), last));
            }
        }
        clean.sort_by_key(|(n, _, _)| sha256_bytes(format!("{s}|{ev}|{n}").as_bytes()));
        for (name, doc, last) in clean.into_iter().take(PER_DIR) {
            let mut cmd: Vec<String> = last
                .get("command")
                .or_else(|| last["run"].get("command"))
                .and_then(Value::as_array)
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect();
            let exe = Path::new(&cmd[0])
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let mut r = json!({"evidence": ev, "record": name, "binary": exe, "binary_sha256": bin_sha.get(&exe)});
            if !bin_sha.contains_key(&exe) {
                r["ok"] = json!(false);
                r["reason"] = json!("binary not found");
                results.push(r);
                continue;
            }
            let out = scratch
                .join(ev.replace('/', "_"))
                .join(name.replace('/', "_"));
            fs::create_dir_all(out.parent().unwrap()).unwrap();
            cmd[0] = bin.join(&exe).to_string_lossy().into_owned();
            let i = cmd.iter().position(|c| c == "--out").unwrap();
            cmd[i + 1] = out.to_string_lossy().into_owned();
            // parameter files named by absolute path in the record live at the same repository path
            let prog = repo.join("research/ic_tool_program");
            for c in cmd.iter_mut() {
                if let Some(k) = c.find("/research/ic_tool_program/") {
                    *c = format!(
                        "{}/{}",
                        prog.display(),
                        &c[k + "/research/ic_tool_program/".len()..]
                    );
                }
            }
            // a parameter file renamed after the run (A18: `k0n61-...` became the curve's slug):
            // use the renamed file and say so; the replay's identical outputs then check its content
            let i = cmd.iter().position(|c| c == "--params").unwrap();
            let pf = PathBuf::from(&cmd[i + 1]);
            if !pf.exists() {
                let fname = pf.file_name().unwrap().to_string_lossy().into_owned();
                if let Some(rest) = fname
                    .strip_prefix('k')
                    .and_then(|t| t.find('-').map(|j| &t[j..]))
                {
                    let alt =
                        pf.with_file_name(format!("{}{rest}", doc["curve"].as_str().unwrap()));
                    if alt.exists() {
                        r["params_renamed"] =
                            json!({"recorded": rel_to(&pf, &repo), "used": rel_to(&alt, &repo)});
                        cmd[i + 1] = alt.to_string_lossy().into_owned();
                    }
                }
            }
            let t0 = Instant::now();
            let p = Command::new(&cmd[0])
                .args(&cmd[1..])
                .env_clear()
                .env("RAYON_NUM_THREADS", "1")
                .env("PATH", "/usr/bin:/bin")
                .output()
                .unwrap();
            r["seconds"] = json!((t0.elapsed().as_secs_f64() * 100.0).round() / 100.0);
            if !p.status.success() || !out.exists() {
                r["ok"] = json!(false);
                r["reason"] = json!(format!(
                    "exit {:?}: {}",
                    p.status.code(),
                    String::from_utf8_lossy(&p.stderr)
                ));
                println!("FAIL {ev} {name}");
                results.push(r);
                continue;
            }
            let new = read_json(&out);
            let mut diffs: Vec<&str> = COMPARED
                .iter()
                .copied()
                .filter(|k| doc.get(*k) != new.get(*k))
                .collect();
            if certs_of(&doc) != certs_of(&new) {
                diffs.push("certificates");
            }
            let mut cache = HashMap::new();
            let verified = ["ic", "rho"]
                .iter()
                .filter(|a| new["certificates"].get(**a).is_some())
                .all(|a| {
                    check_certificate(&new["certificates"][*a], &mut cache)
                        .values()
                        .all(|ok| *ok)
                });
            let scalars: Map<String, Value> = certs_of(&new)
                .as_object()
                .unwrap()
                .iter()
                .map(|(k, v)| (k.clone(), v["scalar"].clone()))
                .collect();
            r["ok"] = json!(diffs.is_empty() && verified);
            r["differing"] = json!(diffs);
            r["replay_scalars_verified"] = json!(verified);
            r["scalars"] = Value::Object(scalars);
            println!(
                "{} {ev} {name} {}s {:?}",
                if diffs.is_empty() && verified {
                    "OK  "
                } else {
                    "FAIL"
                },
                r["seconds"],
                diffs
            );
            results.push(r);
        }
    }
    let identical = results.iter().filter(|r| r["ok"] == true).count();
    let host = Command::new("uname")
        .arg("-srm")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    json!({"schema": "ic-tool-programme-native-replay/v2", "tool": "src/bin/ic_validate.rs (ic_validate replay)",
           "seed": s, "per_directory": PER_DIR, "draw": "research/ic_tool_program/validation/REPLAY_SEED.md",
           "host": host, "replays": results, "replayed": results.len(), "identical": identical})
}

// ------------------------------------------------------------------- build

struct Row {
    key: &'static str,
    dir: &'static str,
    mode: &'static str,
    /// the arm the row reports, as a record-path prefix (rounds and explorations)
    arm: &'static str,
    evidence: &'static [&'static str],
}

const ROWS: &[Row] = &[
    Row {
        key: "R06",
        dir: "rounds/R06-scan-key",
        mode: "round",
        arm: "runs/compare/cand/",
        evidence: &["benchmarks/20261016-ic-r06-scan-key"],
    },
    Row {
        key: "R08",
        dir: "rounds/R08-scan-fold",
        mode: "round",
        arm: "runs/compare/cand/",
        evidence: &["benchmarks/20261016-ic-r08-scan-fold"],
    },
    Row {
        key: "R10",
        dir: "rounds/R10-main-head",
        mode: "round",
        arm: "runs/compare/cand/",
        evidence: &["benchmarks/20261016-ic-r10-main-head"],
    },
    Row {
        key: "R11",
        dir: "rounds/R11-build-streams",
        mode: "round",
        arm: "runs/compare/cand/",
        evidence: &["benchmarks/20261016-ic-r11-build-streams"],
    },
    Row {
        key: "R12",
        dir: "rounds/R12-admitted-prefetch",
        mode: "round",
        arm: "runs/compare/cand/",
        evidence: &["benchmarks/20261017-ic-r12-admitted-prefetch"],
    },
    Row {
        key: "v6",
        dir: "rule/v6",
        mode: "rule",
        arm: "",
        evidence: &["benchmarks/20261016-ic-rule-v6"],
    },
    Row {
        key: "v7",
        dir: "rule/v7",
        mode: "rule",
        arm: "",
        evidence: &[
            "benchmarks/20261016-ic-rule-v7",
            "research/ic_tool_program/rule/v7",
        ],
    },
    Row {
        key: "A14",
        dir: "explorations/A14-prefetch-size-gate-20261017",
        mode: "exploration",
        arm: "runs/gate/",
        evidence: &["benchmarks/20261017-ic-a14-prefetch-size-gate"],
    },
    Row {
        key: "A15",
        dir: "explorations/A15-filter-slots-20261017",
        mode: "exploration",
        arm: "runs/a15/",
        evidence: &["benchmarks/20261017-ic-a15-filter-slots"],
    },
    Row {
        key: "A16",
        dir: "explorations/A16-huge-pages-20261017",
        mode: "exploration",
        arm: "runs/filter/",
        evidence: &["benchmarks/20261017-ic-a16-huge-pages"],
    },
    Row {
        key: "A17",
        dir: "explorations/A17-filter-huge-pages-20261017",
        mode: "exploration",
        arm: "a17/runs/a17/",
        evidence: &["benchmarks/20261017-ic-a17-filter-huge-pages"],
    },
    Row {
        key: "A18",
        dir: "explorations/A18-wider-base-2p47-20261017",
        mode: "exploration",
        arm: "a18/runs/v8-c400/",
        evidence: &["benchmarks/20261017-ic-a18-wider-base"],
    },
    Row {
        key: "A19",
        dir: "explorations/A19-filter-pages-build-parts-20261017",
        mode: "exploration",
        arm: "a19/runs/a19/",
        evidence: &["benchmarks/20261017-ic-a19-filter-pages-build-parts"],
    },
    Row {
        key: "S8",
        dir: "explorations/scan-stages-v8-20261017",
        mode: "stages",
        arm: "runs/",
        evidence: &["benchmarks/20261017-ic-scan-stages-v8"],
    },
];

struct Band {
    curve: String,
    log2_r: f64,
    s_ic: f64,
    s_rho_canonical: f64,
    s_rho_as_built: f64,
    canon: f64,
    sqrt_r: f64,
}

fn band_records(d: &Path, arm: &str) -> Band {
    let recs = Records::load(d);
    let mut by_size: BTreeMap<i64, Vec<&Value>> = BTreeMap::new();
    for stem in recs.stems(arm) {
        if let Some(rec) = recs.first_clean(&stem) {
            by_size
                .entry((f(&rec["log2_r"]) * 10.0).round() as i64)
                .or_default()
                .push(rec);
        }
    }
    let (size, rows) = by_size.into_iter().next_back().unwrap();
    let med = |g: &dyn Fn(&Value) -> f64| {
        median(&rows.iter().map(|r| g(&r["median"])).collect::<Vec<_>>())
    };
    Band {
        curve: rows[0]["curve"].as_str().unwrap().into(),
        log2_r: size as f64 / 10.0,
        s_ic: med(&|m| f(&m["s_ic_cold"])),
        s_rho_canonical: med(&|m| f(&m["s_rho_setup"]) + f(&m["s_rho_model"])),
        s_rho_as_built: med(&|m| f(&m["s_rho_cold"])),
        canon: median(
            &rows
                .iter()
                .map(|r| {
                    r["rho_policy"]["counters"]["canonicalisations_uncharged"]
                        .as_f64()
                        .unwrap_or(0.0)
                })
                .collect::<Vec<_>>(),
        ),
        sqrt_r: f(&rows[0]["r"]).sqrt(),
    }
}

fn band_rule(rep: &Report) -> Band {
    let s = rep.extra["sizes"]
        .as_array()
        .unwrap()
        .iter()
        .max_by(|a, b| f(&a["log2_r"]).partial_cmp(&f(&b["log2_r"])).unwrap())
        .unwrap();
    Band {
        curve: s["curve"].as_str().unwrap().into(),
        log2_r: f(&s["log2_r"]),
        s_ic: f(&s["s_ic_cold"]),
        s_rho_canonical: f(&s["s_rho_canonical"]),
        s_rho_as_built: f(&s["s_rho_as_built"]),
        canon: f(&s["rho_canonicalisations_uncharged_median"]),
        sqrt_r: f(&s["r"]).sqrt(),
    }
}

fn rho_reference(b: &Band) -> Value {
    let mut band = Map::new();
    let mut sides = BTreeSet::new();
    for w in ["0", "0.1", "1"] {
        let rho = b.s_rho_canonical + w.parse::<f64>().unwrap() * b.canon / b.sqrt_r;
        sides.insert(b.s_ic > rho);
        band.insert(w.into(), json!({"ic": b.s_ic, "rho": rho}));
    }
    json!({
        "kind": "negation_frobenius",
        "protocol": "ledger section 23's one-target rho on the same public point, in the same `ic price --single-target` process: the signed-Frobenius r-adding walk (negation and Frobenius classes), its set-up plus its walk; the walk priced at the canonical batched step this process measures (`s_rho_model`), the stronger of the two references the process records",
        "unit": "S: cold time in v0's unit (one batched addition) over sqrt(r)",
        "binary_or_process": "the row's own `ic` binary and process: both arms rebuilt from nothing each repetition, the index calculus first",
        "matches_protocol": true,
        "strongest_available": true,
        "same_unit": true,
        "same_binary_or_process": true,
        "gae_band": band,
        "order_holds_across_band": sides.len() == 1,
        "band_basis": format!("the row's reported arm at its largest size ({}, 2^{:.1}); the index calculus is timed by the clock, so it carries no unpriced native work; the canonicalisations rho's walk leaves uncharged (median {} a process) are priced at 0, 0.1 and 1 GAE each", b.curve, b.log2_r, b.canon),
        "ic_over_rho_canonical": b.s_ic / b.s_rho_canonical,
        "rho_as_built": {"s": b.s_rho_as_built, "ic_over_rho": b.s_ic / b.s_rho_as_built,
            "note": "the reference the published row quotes; it pays more than the canonical step for a walk step, so the published distance to rho is the smaller of the two"},
    })
}

fn pretty(v: &Value) -> String {
    serde_json::to_string_pretty(v).unwrap() + "\n"
}

fn build(check: bool) -> i32 {
    let repo = repo();
    let prog = repo.join("research/ic_tool_program");
    let raw = prog.join("validation/raw");
    let replays = read_json(&raw.join("native-replay.json"));
    let mut changed = Vec::new();
    let mut put = |p: PathBuf, text: String| {
        if fs::read_to_string(&p).ok().as_deref() != Some(text.as_str()) {
            changed.push(rel_to(&p, &repo));
            if !check {
                fs::write(&p, text).unwrap();
            }
        }
    };
    for row in ROWS {
        let d = prog.join(row.dir);
        let rep = recompute(row.mode, &d);
        let band = if row.mode == "rule" {
            band_rule(&rep)
        } else {
            band_records(&d, row.arm)
        };
        let cv = read_json(&raw.join(format!("certverify-{}.json", row.key)));
        let ev_name = rel_to(&d, &repo);
        let mine: Vec<&Value> = replays["replays"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["evidence"] == ev_name.as_str())
            .collect();
        for e in row.evidence {
            let ev = repo.join(e);
            let mut vdoc = rep.doc();
            let mut rawlist = Vec::new();
            for item in vdoc["raw_data"].as_array().unwrap() {
                let p = repo.join(item["path"].as_str().unwrap().trim_end_matches('/'));
                if p.is_file() {
                    rawlist.push(json!({"path": rel_to(&p, &ev), "sha256": item["sha256"]}));
                } else if p.is_dir() {
                    // a directory of records: cite each record
                    let mut ps = Vec::new();
                    walk(&p, &mut ps);
                    for q in ps
                        .into_iter()
                        .filter(|q| q.to_string_lossy().ends_with(".price.json"))
                    {
                        rawlist.push(json!({"path": rel_to(&q, &ev), "sha256": sha256_file(&q)}));
                    }
                }
            }
            vdoc["raw_data"] = Value::Array(rawlist);
            let ok =
                cv["status"] == "pass" && !mine.is_empty() && mine.iter().all(|r| r["ok"] == true);
            let cert = json!({
                "schema": "ic-tool-programme-replay-certificate/v2",
                "status": if ok { "verified" } else { "failed" },
                "independent": true,
                "scalars_verified": cv["status"] == "pass" && mine.iter().all(|r| r["replay_scalars_verified"] == true),
                "what": format!("{ev_name}: every recorded scalar checked by an independent GF(2^m) verifier ([s]G = T, [n]G = O, [s+1]G != T), and {} records drawn by REPLAY_SEED.md rerun natively with their own binaries, certificates and counts compared", mine.len()),
                "scalar_verification": {"tool": "src/bin/ic_validate.rs (ic_validate certverify)", "records": cv["records"],
                    "certificates": cv["certificates"], "failures": cv["failures"].as_array().map(Vec::len),
                    "field_degrees": cv["field_degrees"], "output": rel_to(&raw.join(format!("certverify-{}.json", row.key)), &ev)},
                "native_replay": {"tool": "src/bin/ic_validate.rs (ic_validate replay)", "seed": replays["seed"],
                    "seed_file": rel_to(&prog.join("validation/REPLAY_SEED.md"), &ev), "host": replays["host"],
                    "output": rel_to(&raw.join("native-replay.json"), &ev)},
                "runs": mine.iter().map(|r| {
                    let mut x = json!({"record": r["record"], "binary": r["binary"], "binary_sha256": r["binary_sha256"],
                        "identical": r["ok"], "scalars": r["scalars"]});
                    if let Some(p) = r.get("params_renamed") { x["params_renamed"] = p.clone(); }
                    x
                }).collect::<Vec<_>>(),
            });
            let (vtext, ctext) = (pretty(&vdoc), pretty(&cert));
            let man = json!({
                "schema": "validation-gate/v1",
                "replay_certificate": {"path": "replay-certificate.json", "sha256": sha256_bytes(ctext.as_bytes())},
                "validator_report": {"path": "validator-report.json", "sha256": sha256_bytes(vtext.as_bytes())},
                "rho_reference": rho_reference(&band),
                "built_by": "src/bin/ic_validate.rs (ic_validate build)",
            });
            println!(
                "{:4} {e}: report {} ({} figures), certificate {}, IC/rho {:.2} (as built {:.2})",
                row.key,
                vdoc["status"],
                vdoc["headline"].as_array().unwrap().len(),
                cert["status"],
                man["rho_reference"]["ic_over_rho_canonical"]
                    .as_f64()
                    .unwrap(),
                man["rho_reference"]["rho_as_built"]["ic_over_rho"]
                    .as_f64()
                    .unwrap()
            );
            put(ev.join("validator-report.json"), vtext);
            put(ev.join("replay-certificate.json"), ctext);
            put(ev.join("VALIDATION.json"), pretty(&man));
        }
    }
    if check && !changed.is_empty() {
        println!("would change:");
        for c in &changed {
            println!("  {c}");
        }
        return 1;
    }
    0
}

// --------------------------------------------------------------------- cli

#[derive(Parser)]
#[command(about = "Validation-gate tooling for the IC tool programme (docs/VALIDATION_GATE.md)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check every certificate's scalar in record trees or tarballs.
    Certverify {
        paths: Vec<PathBuf>,
        #[arg(long)]
        out: PathBuf,
    },
    /// Recompute an evidence directory's published figures from its raw records.
    Recompute {
        mode: String,
        dir: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Rerun REPLAY_SEED.md's draw with the recorded binaries.
    Replay {
        #[arg(long)]
        bin: PathBuf,
        #[arg(long)]
        scratch: PathBuf,
        #[arg(long)]
        out: PathBuf,
        dirs: Vec<PathBuf>,
    },
    /// Write every evidence directory's VALIDATION.json and its two artifacts.
    Build {
        #[arg(long)]
        check: bool,
    },
}

fn main() {
    let code = match Cli::parse().cmd {
        Cmd::Certverify { paths, out } => {
            let doc = certverify(&paths);
            fs::write(&out, pretty(&doc)).unwrap();
            println!(
                "{}",
                json!({"status": doc["status"], "records": doc["records"], "certificates": doc["certificates"], "field_degrees": doc["field_degrees"]})
            );
            (doc["status"] != "pass") as i32
        }
        Cmd::Recompute { mode, dir, out } => {
            let rep = recompute(&mode, &fs::canonicalize(&dir).unwrap());
            let doc = rep.doc();
            if let Some(out) = out {
                fs::write(&out, pretty(&doc)).unwrap();
            }
            println!(
                "{}",
                json!({"status": doc["status"], "headline": doc["headline"].as_array().unwrap().len(),
                "disagreements": doc["disagreements"].as_array().unwrap().len(), "notes": doc["notes"].as_array().unwrap().len()})
            );
            for d in doc["disagreements"].as_array().unwrap().iter().take(12) {
                eprintln!("DISAGREE {d}");
            }
            (doc["status"] != "pass") as i32
        }
        Cmd::Replay {
            bin,
            scratch,
            out,
            dirs,
        } => {
            let doc = replay(&bin, &scratch, &dirs);
            fs::write(&out, pretty(&doc)).unwrap();
            println!(
                "{}",
                json!({"replayed": doc["replayed"], "identical": doc["identical"]})
            );
            (doc["replayed"] != doc["identical"] || doc["replayed"] == 0) as i32
        }
        Cmd::Build { check } => build(check),
    };
    std::process::exit(code);
}
