//! Port of `vcommon.py` (shared loaders) and `vstats.py` (statistics).
//!
//! Counted sessions are loaded through `load_counted`, which removes every
//! wall or timing field before any other code sees the record (PROTOCOL
//! section 10); the only isolation data kept are `level` and `blockers`, and a
//! blocker that begins with the too-short prefix is cut back to the prefix.

use crate::py::{self, fsum_py, idiv, int, int_to_f64, lgamma, Num, PyRandom};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::hash::Hash;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const TOO_SHORT: &str = "L2: solve too short to admit wall time";
pub const IC_ARMS_WALL: [&str; 4] = ["dgb", "dgb-aa", "dgbf", "dgbf-edge"];

static ROOT: OnceLock<PathBuf> = OnceLock::new();

/// The study directory (research/ic_packed_filter_20261029).
pub fn root() -> &'static Path {
    ROOT.get_or_init(|| {
        if let Ok(r) = std::env::var("PFV_ROOT") {
            return PathBuf::from(r);
        }
        Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").canonicalize().expect("study root")
    })
}

/// The repository root.
pub fn repo() -> PathBuf {
    root().join("..").join("..").canonicalize().expect("repo root")
}

pub fn backlog() -> PathBuf {
    repo().join("backlog").join("XP-20261029-packed-filter")
}

pub fn stop(msg: impl AsRef<str>) -> ! {
    eprintln!("STOP: {}", msg.as_ref());
    std::process::exit(2)
}

pub fn sha256_file(p: &Path) -> String {
    let mut f = std::fs::File::open(p).unwrap_or_else(|e| stop(format!("open {}: {e}", p.display())));
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    loop {
        let n = f.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    hex(&h.finalize())
}

pub fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

pub fn jload(p: &Path) -> Value {
    let s = std::fs::read_to_string(p).unwrap_or_else(|e| stop(format!("read {}: {e}", p.display())));
    serde_json::from_str(&s).unwrap_or_else(|e| stop(format!("parse {}: {e}", p.display())))
}

pub fn read_text(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| stop(format!("read {}: {e}", p.display())))
}

pub fn session_dir(b: u32, kind: &str) -> PathBuf {
    root().join(format!("P{b}")).join(kind).join("eks-results")
}

/// The (want, name) lines of a SHA256SUMS file.
pub fn sums_lines(eks: &Path) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for line in read_text(&eks.join("SHA256SUMS")).lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let f = py::split_ws(line, 1);
        out.push((f[0].clone(), f[1].clone()));
    }
    out
}

/// Check every line of SHA256SUMS; return (ok, n, bad list).
pub fn check_sums(eks: &Path) -> (bool, usize, Vec<String>) {
    let mut bad = Vec::new();
    let lines = sums_lines(eks);
    for (want, name) in &lines {
        let p = eks.join(name);
        if !p.exists() || &sha256_file(&p) != want {
            bad.push(name.clone());
        }
    }
    (bad.is_empty(), lines.len(), bad)
}

pub fn is_timing_key(k: &str) -> bool {
    k.contains("wall")
        || k == "time"
        || k == "online"
        || k.ends_with("_ns")
        || k.ends_with("_us")
        || k.ends_with("_ms")
        || k.ends_with("_seconds")
        || k == "unix_ms"
}

fn strip(o: &Value) -> Value {
    match o {
        Value::Object(m) => {
            Value::Object(m.iter().filter(|(k, _)| !is_timing_key(k)).map(|(k, v)| (k.clone(), strip(v))).collect())
        }
        Value::Array(a) => Value::Array(a.iter().map(strip).collect()),
        x => x.clone(),
    }
}

pub fn mask_blockers(bl: &[Value]) -> Vec<Value> {
    bl.iter()
        .map(|s| {
            let s = s.as_str().unwrap();
            Value::from(if s.starts_with(TOO_SHORT) { TOO_SHORT } else { s })
        })
        .collect()
}

fn or_empty_list(v: Option<&Value>) -> Vec<Value> {
    match v {
        Some(x) if py::truthy(x) => x.as_array().unwrap().clone(),
        _ => Vec::new(),
    }
}

pub fn counted_view(rec: &Value) -> Value {
    let empty = Value::Object(Map::new());
    let iso = match rec.get("isolation") {
        Some(x) if py::truthy(x) => x,
        _ => &empty,
    };
    let mut keep = Map::new();
    keep.insert("level".into(), iso.get("level").cloned().unwrap_or(Value::Null));
    keep.insert("blockers".into(), Value::Array(mask_blockers(&or_empty_list(iso.get("blockers")))));
    let mut r = Map::new();
    for (k, v) in rec.as_object().unwrap() {
        if k != "isolation" {
            r.insert(k.clone(), v.clone());
        }
    }
    let mut r = match strip(&Value::Object(r)) {
        Value::Object(m) => m,
        _ => unreachable!(),
    };
    r.insert("isolation".into(), Value::Object(keep));
    Value::Object(r)
}

fn read_jsonl(p: &Path) -> Vec<Value> {
    read_text(p)
        .lines()
        .map(|l| serde_json::from_str(l).unwrap_or_else(|e| stop(format!("parse {}: {e}", p.display()))))
        .collect()
}

pub fn load_counted(b: u32, kind: &str) -> Vec<Value> {
    let p = session_dir(b, kind).join("session").join("records.jsonl");
    read_jsonl(&p).iter().map(counted_view).collect()
}

/// Wall session records in full. The wall session runs only IC arms.
pub fn load_wall(b: u32) -> Vec<Value> {
    let p = session_dir(b, "wall").join("session").join("records.jsonl");
    let recs = read_jsonl(&p);
    for r in &recs {
        let arm = r["arm"].as_str().unwrap();
        if !IC_ARMS_WALL.contains(&arm) {
            stop(format!("wall session arm {arm} is not an IC arm"));
        }
    }
    recs
}

/// Wall session records with every wall field removed.
pub fn load_wall_counts_only(b: u32) -> Vec<Value> {
    load_counted(b, "wall")
}

// ---------------------------------------------------------------- ordered maps

/// A dict with Python insertion order: re-inserting a key replaces the value
/// and keeps the key's first position.
#[derive(Clone)]
pub struct OMap<K, V> {
    keys: Vec<K>,
    vals: Vec<V>,
    idx: HashMap<K, usize>,
}

impl<K: Clone + Eq + Hash, V> OMap<K, V> {
    pub fn new() -> Self {
        OMap { keys: Vec::new(), vals: Vec::new(), idx: HashMap::new() }
    }
    pub fn insert(&mut self, k: K, v: V) {
        if let Some(&i) = self.idx.get(&k) {
            self.vals[i] = v;
        } else {
            self.idx.insert(k.clone(), self.keys.len());
            self.keys.push(k);
            self.vals.push(v);
        }
    }
    pub fn contains(&self, k: &K) -> bool {
        self.idx.contains_key(k)
    }
    pub fn get(&self, k: &K) -> Option<&V> {
        self.idx.get(k).map(|&i| &self.vals[i])
    }
    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        match self.idx.get(k) {
            Some(&i) => Some(&mut self.vals[i]),
            None => None,
        }
    }
    pub fn entry_or(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
        if !self.idx.contains_key(&k) {
            self.insert(k.clone(), f());
        }
        let i = self.idx[&k];
        &mut self.vals[i]
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.keys.iter()
    }
    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.vals.iter()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.vals.iter())
    }
    pub fn key_set(&self) -> std::collections::BTreeSet<K>
    where
        K: Ord,
    {
        self.keys.iter().cloned().collect()
    }
}

impl<K: Clone + Eq + Hash, V> std::ops::Index<&K> for OMap<K, V> {
    type Output = V;
    fn index(&self, k: &K) -> &V {
        self.get(k).expect("KeyError")
    }
}

impl<K: Clone + Eq + Hash, V> Default for OMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub type Key = (String, i64, i64);
pub type Arms<'a> = OMap<String, OMap<Key, &'a Value>>;

pub fn key(r: &Value) -> Key {
    let w = &r["workload"];
    (
        w["curve"]["slug"].as_str().unwrap().to_string(),
        int(&w["target_index"]) as i64,
        int(&r["round"]) as i64,
    )
}

pub fn arm(r: &Value) -> String {
    r["arm"].as_str().unwrap().to_string()
}

pub fn by_arm(records: &[Value], measured_only: bool) -> Arms<'_> {
    let mut d: Arms = OMap::new();
    for r in records {
        if measured_only && py::truthy(&r["warmup"]) {
            continue;
        }
        let a = d.entry_or(arm(r), OMap::new);
        let k = key(r);
        if a.contains(&k) {
            stop(format!("duplicate key {k:?} on arm {}", arm(r)));
        }
        a.insert(k, r);
    }
    d
}

/// `collections.defaultdict(dict)` keyed by arm then key, warm-ups included
/// (j3_work.all_by_key).
pub fn all_by_key(records: &[Value]) -> Arms<'_> {
    let mut d: Arms = OMap::new();
    for r in records {
        d.entry_or(arm(r), OMap::new).insert(key(r), r);
    }
    d
}

fn empty_obj() -> &'static Value {
    static E: OnceLock<Value> = OnceLock::new();
    E.get_or_init(|| Value::Object(Map::new()))
}

/// `x.get(k) or {}` for a dict field.
pub fn get_or_empty<'a>(v: &'a Value, k: &str) -> &'a Value {
    match v.get(k) {
        Some(x) if py::truthy(x) => x,
        _ => empty_obj(),
    }
}

/// U of band.rs at 3271a25d02 (vcommon.unpriced_total).
pub fn unpriced_total(r: &Value) -> (Num, bool) {
    let mut total = Num::Int(0);
    let mut missing = false;
    let counters = get_or_empty(r, "counters");
    for name in r["cost"]["unpriced"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let base = name.strip_suffix("_uncharged").unwrap_or(name);
        if let Some(c) = counters.get(name) {
            total = total.add(Num::of(c));
            continue;
        }
        let (mut found, mut t) = (false, Num::Int(0));
        for p in r["phases"].as_array().unwrap() {
            let nat = get_or_empty(p, "native");
            if let Some(x) = nat.get(name) {
                t = t.add(Num::of(x));
                found = true;
            } else if let Some(x) = nat.get(base) {
                t = t.add(Num::of(x));
                found = true;
            }
        }
        if !found && base.starts_with("solver_") && r.get("solver").is_some_and(py::truthy) {
            total = total.add(Num::of(&r["solver"]["ops"]));
            continue;
        }
        if !found {
            missing = true;
        }
        total = total.add(t);
    }
    (total, missing)
}

pub fn calibrated(r: &Value) -> (f64, i128) {
    let det = get_or_empty(r, "detail");
    let pinned = or_empty_list(det.get("calibration_pinned_units"));
    if !pinned.is_empty() {
        stop(format!("record {} carries pinned calibration units {}", r["record_id"], py::repr(&Value::Array(pinned))));
    }
    (0.0, 0)
}

/// `G - C + w * (U + n_C)`.
pub fn priced_gae(r: &Value, w: f64) -> f64 {
    let g = Num::of(&r["cost"]["total_gae"]);
    let (u, missing) = unpriced_total(r);
    if missing && w > 0.0 {
        stop(format!("record {} has an uncounted unpriced counter", r["record_id"]));
    }
    let (c, nc) = calibrated(r);
    let gc = g.sub(Num::Float(c));
    gc.add(Num::Float(w).mul(u.add(Num::Int(nc)))).f()
}

/// table_filter_bit_sets summed over phases (0 if absent) and the phases carrying it.
pub fn tfbs(r: &Value) -> (i128, Vec<String>) {
    let (mut t, mut where_) = (0i128, Vec::new());
    for p in r["phases"].as_array().unwrap() {
        let wn = get_or_empty(p, "work_native");
        if let Some(x) = wn.get("table_filter_bit_sets") {
            t += int(x);
            where_.push(p["name"].as_str().unwrap().to_string());
        }
    }
    (t, where_)
}

pub fn curve_r_sqrt(r: &Value) -> f64 {
    int_to_f64(int(&r["workload"]["curve"]["r"])).sqrt()
}

pub fn sprime(r: &Value, w: f64) -> f64 {
    let (t, _) = tfbs(r);
    (priced_gae(r, w) + w * int_to_f64(t)) / curve_r_sqrt(r)
}

// ---------------------------------------------------------------- vstats

fn betacf(a: f64, b: f64, x: f64) -> f64 {
    const MAXIT: i32 = 400;
    const EPS: f64 = 3e-16;
    const FPMIN: f64 = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0f64;
    let mut d = 1.0 - qab * x / qap;
    d = 1.0 / (if d.abs() > FPMIN { d } else { FPMIN });
    let mut h = d;
    for m in 1..=MAXIT {
        let mf = m as f64;
        let m2 = (2 * m) as f64;
        let aa = mf * (b - mf) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + mf) * (qab + mf) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < FPMIN {
            d = FPMIN;
        }
        c = 1.0 + aa / c;
        if c.abs() < FPMIN {
            c = FPMIN;
        }
        d = 1.0 / d;
        let de = d * c;
        h *= de;
        if (de - 1.0).abs() < EPS {
            break;
        }
    }
    h
}

fn betai(a: f64, b: f64, x: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let lbt = lgamma(a + b) - lgamma(a) - lgamma(b) + a * x.ln() + b * (-x).ln_1p();
    let bt = lbt.exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        return bt * betacf(a, b, x) / a;
    }
    1.0 - bt * betacf(b, a, 1.0 - x) / b
}

fn t_cdf(t: f64, nu: f64) -> f64 {
    let x = nu / (nu + t * t);
    let p = 0.5 * betai(nu / 2.0, 0.5, x);
    if t > 0.0 {
        1.0 - p
    } else {
        p
    }
}

pub fn t_ppf(q: f64, nu: f64) -> f64 {
    let (mut lo, mut hi) = (0.0f64, 50.0f64);
    for _ in 0..200 {
        let mid = (lo + hi) / 2.0;
        if t_cdf(mid, nu) < q {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// A pair: (wall_a, gae_a, wall_b, gae_b); walls are ints, GAE floats.
pub type Pair = (i128, f64, i128, f64);

#[derive(Clone)]
pub struct Ratio {
    pub n: usize,
    pub ns_per_gae_a: f64,
    pub ns_per_gae_b: f64,
    pub ratio: f64,
    pub se_log: f64,
    pub dof: f64, // an int (n - 1) in the ratio-of-sums case; see dof_int
    pub dof_int: Option<i128>,
    pub sum_wall_a: i128,
    pub sum_wall_b: i128,
    pub sum_gae_a: f64,
    pub sum_gae_b: f64,
    pub pairs_below_level: usize,
    pub ci95_log: [f64; 2],
    pub ci95_linear: [f64; 2],
}

/// R = (sum wall_b / sum gae_b) / (sum wall_a / sum gae_a) and the linearised
/// standard error of ln R (delta method on the four sums).
pub fn ratio_of_sums(pairs: &[Pair]) -> Ratio {
    let n = pairs.len();
    let sa: i128 = pairs.iter().map(|p| p.0).sum();
    let ga = fsum_py(pairs.iter().map(|p| p.1));
    let sb: i128 = pairs.iter().map(|p| p.2).sum();
    let gb = fsum_py(pairs.iter().map(|p| p.3));
    let r = (int_to_f64(sb) / gb) / (int_to_f64(sa) / ga);
    let (ma, mga, mb, mgb) = (idiv(sa, n as i128), ga / n as f64, idiv(sb, n as i128), gb / n as f64);
    let u: Vec<f64> =
        pairs.iter().map(|p| int_to_f64(p.2) / mb - p.3 / mgb - int_to_f64(p.0) / ma + p.1 / mga).collect();
    let mu = fsum_py(u.iter().copied()) / u.len() as f64;
    let s2 = fsum_py(u.iter().map(|x| (x - mu).powf(2.0))) / (n as f64 - 1.0);
    let se = (s2 / n as f64).sqrt();
    Ratio {
        n,
        ns_per_gae_a: int_to_f64(sa) / ga,
        ns_per_gae_b: int_to_f64(sb) / gb,
        ratio: r,
        se_log: se,
        dof: (n - 1) as f64,
        dof_int: Some((n - 1) as i128),
        sum_wall_a: sa,
        sum_wall_b: sb,
        sum_gae_a: ga,
        sum_gae_b: gb,
        pairs_below_level: 0,
        ci95_log: [0.0; 2],
        ci95_linear: [0.0; 2],
    }
}

pub fn ci_log(r: f64, se: f64, dof: f64) -> [f64; 2] {
    let t = t_ppf(0.5 + 0.95 / 2.0, dof);
    [r * (-t * se).exp(), r * (t * se).exp()]
}

pub fn ci_linear(r: f64, se: f64, dof: f64) -> [f64; 2] {
    let t = t_ppf(0.5 + 0.95 / 2.0, dof);
    [r * (1.0 - t * se), r * (1.0 + t * se)]
}

pub struct Corrected {
    pub ratio: f64,
    pub se_log: f64,
    pub dof: f64,
    pub ci95_log: [f64; 2],
    pub ci95_linear: [f64; 2],
}

/// corrected = ratio / aa_ratio, log SEs combined as independent,
/// Welch-Satterthwaite degrees of freedom.
pub fn corrected(r: &Ratio, aa: &Ratio) -> Corrected {
    let c = r.ratio / aa.ratio;
    let (v1, v2) = (r.se_log.powf(2.0), aa.se_log.powf(2.0));
    let se = (v1 + v2).sqrt();
    let dof = (v1 + v2).powf(2.0) / (v1 * v1 / r.dof + v2 * v2 / aa.dof);
    Corrected { ratio: c, se_log: se, dof, ci95_log: ci_log(c, se, dof), ci95_linear: ci_linear(c, se, dof) }
}

pub fn pair_val(x: [f64; 2]) -> Value {
    Value::Array(vec![py::fval(x[0]), py::fval(x[1])])
}

impl Ratio {
    /// The dict `ratio_of_sums` returns, plus summarize()'s keys when `summarized`.
    pub fn val(&self, summarized: bool) -> Value {
        let mut o = py::Obj::new();
        o.set("n", self.n as u64)
            .set("ns_per_gae_a", py::fval(self.ns_per_gae_a))
            .set("ns_per_gae_b", py::fval(self.ns_per_gae_b))
            .set("ratio", py::fval(self.ratio))
            .set("se_log", py::fval(self.se_log))
            .set("dof", py::int_val(self.dof_int.unwrap()))
            .set("sum_wall_a", py::int_val(self.sum_wall_a))
            .set("sum_wall_b", py::int_val(self.sum_wall_b))
            .set("sum_gae_a", py::fval(self.sum_gae_a))
            .set("sum_gae_b", py::fval(self.sum_gae_b));
        if summarized {
            o.set("pairs_below_level", self.pairs_below_level as u64)
                .set("ci95_log", pair_val(self.ci95_log))
                .set("ci95_linear", pair_val(self.ci95_linear));
        }
        o.val()
    }
}

impl Corrected {
    pub fn val(&self) -> Value {
        let mut o = py::Obj::new();
        o.set("ratio", py::fval(self.ratio))
            .set("se_log", py::fval(self.se_log))
            .set("dof", py::fval(self.dof))
            .set("ci95_log", pair_val(self.ci95_log))
            .set("ci95_linear", pair_val(self.ci95_linear));
        o.val()
    }
}

/// vstats.percentile over numbers (an element is returned as-is when k is integral).
pub fn percentile(xs: &[Num], q: f64) -> Num {
    let mut xs = xs.to_vec();
    xs.sort_by(|a, b| a.cmp(*b));
    let k = (xs.len() - 1) as f64 * q;
    let (f, c) = (k.floor(), k.ceil());
    if f == c {
        return xs[f as usize];
    }
    let (xf, xc) = (xs[f as usize], xs[c as usize]);
    xf.add(Num::Float(xc.sub(xf).mul(Num::Float(k - f)).f()))
}

pub fn percentile_f(xs: &[f64], q: f64) -> f64 {
    let v: Vec<Num> = xs.iter().map(|x| Num::Float(*x)).collect();
    percentile(&v, q).f()
}

/// Percentile interval from B resamples of whole clusters with replacement.
pub fn cluster_bootstrap<C>(clusters: &[C], stat: impl Fn(&[&C]) -> f64, b: usize, seed: u128) -> [f64; 2] {
    let mut rng = PyRandom::new(seed);
    let n = clusters.len();
    let mut vals = Vec::with_capacity(b);
    for _ in 0..b {
        let sample: Vec<&C> = (0..n).map(|_| &clusters[rng.randrange(n as u64) as usize]).collect();
        vals.push(stat(&sample));
    }
    [percentile_f(&vals, 0.025), percentile_f(&vals, 0.975)]
}
