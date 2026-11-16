//! Independent recomputation of analysis/summary.json from the raw R2 and R3
//! runs (PROTOCOL.md and amendments 1-3; the producer's summary code is not
//! read), receipt and coverage checks, the R3 gates, and the rho-reference
//! band (AGENTS.md section 5a).

use crate::util::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;

const TOL: f64 = 1e-9;
const R2_BENCH: [&str; 6] = ["R2-L1", "R2-L2", "R2-L3", "R2-L4", "R2-L5", "R2-L6"];
const R2_ABL: [&str; 2] = ["R2-A1", "R2-A2"];
const R2_DLP: [&str; 2] = ["R2-D1", "R2-D2"];
const R3: [&str; 3] = ["R3-C", "R3-D1", "R3-D2"];
const HOST_A: [&str; 4] = ["L1", "L2", "L3", "L4"];
const FAMS: [&str; 5] = ["jv-nagao", "gaudry-s4", "gaudry-d4", "gaudry-s5", "dense"];
const SHAPES: [(u32, u32); 17] = [(2, 8), (2, 16), (2, 32), (3, 4), (3, 6), (3, 8), (3, 10), (4, 3), (4, 4), (4, 5),
                                   (5, 2), (5, 3), (5, 4), (6, 2), (6, 3), (7, 2), (8, 2)];
const DENSE_P: [u64; 3] = [65521, 2147483647, 4294967291];
const PIN_R2: (&str, &str) = ("c5612e2eb33393ca4d446b482685179a8418ff9d52682dd70c53d62f0e5ba1d3",
                              "afa830fc99087f12978f1e072c16b3f6e164c99b2b15e17fd6a3a7cc92bb050f");
const PIN_R3: (&str, &str) = ("06351d89429a1d50d5026e9bfe99c7299e25e3d15b0024b1c0b7cda4d686444b",
                              "a49184b0c25ae27538f083999459ca9c4cc88abf4f9c99cc06b007253b5feb85");
const BENCH_BIN: &str = "./target/release/examples/postgb_la_bench";
const JV_BIN: &str = "./target/release/examples/jv_cover";
const TIMING: [&str; 3] = ["f4_ms", "lin_ms", "wall_ms"];

// ---------------------------------------------------------------- cards ---

#[derive(Default, Clone)]
struct Card {
    binds: BTreeMap<String, String>,
    argv: Vec<String>,
    env: BTreeMap<String, String>,
    timeout: u64,
    receipt_commit: String,
    receipt_added_in: String,
}

fn unq(s: &str) -> String {
    let s = s.trim();
    s.strip_prefix('\'').and_then(|x| x.strip_suffix('\'')).unwrap_or(s).to_string()
}

/// The few fields of a frozen-command card needed here, without a YAML dependency.
fn parse_card(text: &str) -> Card {
    let mut c = Card::default();
    let ls: Vec<&str> = text.lines().collect();
    let mut i = 0;
    while i < ls.len() {
        if let Some(p) = ls[i].strip_prefix("- path: ") {
            if let Some(h) = ls.get(i + 1).and_then(|l| l.trim().strip_prefix("sha256: ")) {
                c.binds.insert(p.trim().to_string(), h.trim().to_string());
            }
        }
        if ls[i] == "- id: run" {
            let mut j = i + 1;
            while j < ls.len() && !ls[j].starts_with("bus_address") && !ls[j].starts_with("- id:") {
                let l = ls[j];
                if l == "  argv:" {
                    let mut k = j + 1;
                    while k < ls.len() && ls[k].starts_with("  - ") {
                        c.argv.push(unq(&ls[k][4..]));
                        k += 1;
                    }
                } else if let Some(t) = l.strip_prefix("  timeout_seconds: ") {
                    c.timeout = t.trim().parse().unwrap();
                } else if l == "  env:" {
                    let mut k = j + 1;
                    while k < ls.len() && ls[k].starts_with("    ") {
                        let (a, b) = ls[k].trim().split_once(": ").unwrap();
                        c.env.insert(a.to_string(), unq(b));
                        k += 1;
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
    c
}

fn opts(argv: &[String]) -> BTreeMap<String, String> {
    let mut m = BTreeMap::new();
    let mut i = 1;
    while i + 1 < argv.len() {
        m.insert(argv[i].clone(), argv[i + 1].clone());
        i += 2;
    }
    m
}

struct Ctx {
    snap: String,
    led: Ledger,
    cards: BTreeMap<String, Card>,
    hosts: BTreeMap<String, String>,
}

impl Ctx {
    fn p(&self, rel: &str) -> String {
        format!("{STUDY}/{rel}")
    }
    fn json(&self, rel: &str) -> Value {
        git_json(&self.snap, &self.p(rel))
    }
    fn sha(&self, rel: &str) -> String {
        sha_hex(&git_show(&self.snap, &self.p(rel)))
    }
}

fn receipts(cx: &mut Ctx, cards: &[&str], pins: (&str, &str), strict: bool) {
    let mut seq: Vec<(String, Card)> = Vec::new();
    for &c in cards {
        let base = format!("cards/CARD-postgb-la-20261026-{c}");
        let text = String::from_utf8(git_show(&cx.snap, &cx.p(&format!("{base}.yaml")))).unwrap();
        let mut card = parse_card(&text);
        let r = cx.json(&format!("{base}.receipt.json"));
        let mut res: Vec<(String, bool, Value)> = Vec::new();
        let mut chk = |name: String, ok: bool, d: Value| res.push((name, ok, d));
        let card_sha = sha_hex(&git_show(&cx.snap, &format!("{STUDY}/{base}.yaml")));
        chk(format!("{c}: receipt card_sha256 = committed card"), r["card_sha256"] == json!(card_sha), Value::Null);
        chk(format!("{c}: receipt completed"), r["status"] == json!("completed"), r["status"].clone());
        let cmds = r["commands"].as_array().unwrap();
        let all_ok = cmds.iter().all(|x| x["exit_code"] == json!(0) && x["succeeded"] == json!(true)
            && x["timed_out"] == json!(false) && x["attempts"] == json!(1));
        chk(format!("{c}: every command exit 0, one attempt, no timeout"), all_ok && r["commands_total"] == json!(cmds.len()),
            json!(cmds.iter().map(|x| json!([x["id"], x["exit_code"]])).collect::<Vec<_>>()));
        let ids: Vec<String> = cmds.iter().map(|x| x["id"].as_str().unwrap().to_string()).collect();
        chk(format!("{c}: git-clean, disk and lscpu records, quiet-CPU wait, then run"),
            ids == ["git-clean", "output-absent", "mkdir-runs", "disk-record", "lscpu-record", "disk-before-run", "quiet-before-run", "run"],
            json!(ids));
        if strict {
            chk(format!("{c}: card binds the bench and jv_cover pins"),
                card.binds.get(BENCH_BIN).map(|s| s.as_str()) == Some(pins.0) && card.binds.get(JV_BIN).map(|s| s.as_str()) == Some(pins.1),
                json!([card.binds.get(BENCH_BIN), card.binds.get(JV_BIN)]));
            chk(format!("{c}: card binds PROTOCOL.md at its committed sha256"),
                card.binds.get(&cx.p("PROTOCOL.md")) == Some(&cx.sha("PROTOCOL.md")), Value::Null);
        }
        let run = cmds.iter().find(|x| x["id"] == json!("run")).unwrap();
        let argv: Vec<String> = run["argv"].as_array().unwrap().iter().map(|a| a.as_str().unwrap().to_string()).collect();
        chk(format!("{c}: receipt run argv = card run argv"), argv == card.argv, json!(argv));
        for (path, h) in run["outputs"].as_object().unwrap() {
            let committed = git_try(&cx.snap, path).map(|b| sha_hex(&b));
            chk(format!("{c}: output {} sha256 = committed file", path.rsplit('/').next().unwrap()),
                committed.as_deref() == h.as_str(), json!([h, committed]));
        }
        for stream in ["stdout", "stderr"] {
            chk(format!("{c}: run {stream} sha256 = committed file"),
                run[format!("{stream}_sha256")] == json!(cx.sha(&format!("{base}.run.{stream}"))), Value::Null);
        }
        let lscpu = cmds.iter().find(|x| x["id"] == json!("lscpu-record")).unwrap();
        chk(format!("{c}: lscpu sha256 = committed file"), lscpu["stdout_sha256"] == json!(cx.sha(&format!("{base}.lscpu.stdout"))), Value::Null);
        let lt = String::from_utf8(git_show(&cx.snap, &cx.p(&format!("{base}.lscpu.stdout")))).unwrap();
        let field = |k: &str| lt.lines().find(|l| l.starts_with(k)).map(|l| l.split_once(':').unwrap().1.trim().to_string()).unwrap_or_default();
        cx.hosts.insert(c.to_string(), format!("{} | L2 {}", field("Model name"), field("L2 cache")));
        let commit = r["commit"].as_str().unwrap().to_string();
        let (anc, _) = git(&["merge-base", "--is-ancestor", &commit, &cx.snap]);
        chk(format!("{c}: receipt commit is an ancestor of the snapshot"), anc, json!(commit));
        let cards_dir = format!("{STUDY}/cards/");
        let db: Vec<String> = r["dirty_before"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect();
        chk(format!("{c}: dirty_before only under cards/"), db.iter().all(|p| p.starts_with(&cards_dir)), json!(db));
        if let Some(log) = git_try(&cx.snap, &cx.p(&format!("{base}.launcher.log"))) {
            let log = String::from_utf8_lossy(&log).to_string();
            let named = log.lines().find_map(|l| l.strip_prefix("receipt: ")).and_then(|l| l.rsplit_once("sha256 ")).map(|x| x.1.trim().to_string());
            chk(format!("{c}: launcher log names the committed receipt's sha256"),
                named.as_deref() == Some(cx.sha(&format!("{base}.receipt.json")).as_str()), json!(named));
        }
        let (_, added) = git(&["log", "--diff-filter=A", "--format=%H", &cx.snap, "--", &format!("{STUDY}/{base}.receipt.json")]);
        for (name, ok, d) in res {
            if strict {
                cx.led.check(&name, ok, d);
            } else if !ok {
                cx.led.checks.push(json!({"check": name, "ok": false, "detail": d, "beside_only": true}));
            }
        }
        card.receipt_commit = commit;
        card.receipt_added_in = added.lines().last().unwrap_or("").to_string();
        cx.cards.insert(c.to_string(), card.clone());
        seq.push((c.to_string(), card));
    }
    if strict {
        for w in seq.windows(2) {
            cx.led.check(&format!("{} started after {}'s receipt was committed (one card at a time)", w[1].0, w[0].0),
                         w[1].1.receipt_commit == w[0].1.receipt_added_in,
                         json!([w[0].1.receipt_added_in, w[1].1.receipt_commit]));
        }
    }
}

// ------------------------------------------------------------------ rows ---

#[derive(Clone)]
struct Row {
    card: String,
    si: usize,
    v: Value,
}

type Key = (String, String);

fn key(v: &Value) -> Key {
    (v["family"].as_str().unwrap().to_string(), v["label"].as_str().unwrap().to_string())
}

fn arm<'a>(v: &'a Value, name: &str) -> Option<&'a Value> {
    v["arms"].as_array().unwrap().iter().find(|a| a["arm"] == json!(name))
}

fn med(v: &Value, name: &str) -> f64 {
    num(&arm(v, name).unwrap()["median_ms"])
}

fn muls(v: &Value, name: &str) -> f64 {
    num(&arm(v, name).unwrap()["muls"])
}

fn upper_median(xs: &[f64]) -> f64 {
    let mut v = xs.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn gmean(xs: &[f64]) -> f64 {
    (xs.iter().map(|x| x.ln()).sum::<f64>() / xs.len() as f64).exp()
}

fn avg_ranks(xs: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..xs.len()).collect();
    idx.sort_by(|&a, &b| xs[a].partial_cmp(&xs[b]).unwrap());
    let mut r = vec![0.0; xs.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && xs[idx[j + 1]] == xs[idx[i]] {
            j += 1;
        }
        for k in i..=j {
            r[idx[k]] = (i + j) as f64 / 2.0 + 1.0;
        }
        i = j + 1;
    }
    r
}

fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let (ma, mb) = (a.iter().sum::<f64>() / n, b.iter().sum::<f64>() / n);
    let num: f64 = a.iter().zip(b).map(|(x, y)| (x - ma) * (y - mb)).sum();
    let da: f64 = a.iter().map(|x| (x - ma).powi(2)).sum();
    let db: f64 = b.iter().map(|y| (y - mb).powi(2)).sum();
    num / (da * db).sqrt()
}

/// Own percentile bootstrap of the geometric mean over systems: 4000 resamples,
/// `StdRng::seed_from_u64(20261027)` per family (amendment 3 names the RNG and
/// seed, not the draw order), nearest-rank 2.5% / 97.5%.  A stochastic check.
fn bootstrap(ratios: &[f64]) -> [f64; 2] {
    let mut rng = StdRng::seed_from_u64(20261027);
    let logs: Vec<f64> = ratios.iter().map(|x| x.ln()).collect();
    let k = logs.len();
    let n = 4000;
    let mut means: Vec<f64> = (0..n)
        .map(|_| ((0..k).map(|_| logs[rng.gen_range(0..k)]).sum::<f64>() / k as f64).exp())
        .collect();
    means.sort_by(|a, b| a.partial_cmp(b).unwrap());
    [means[(0.025 * n as f64) as usize], means[(0.975 * n as f64) as usize - 1]]
}

fn expected_corpus() -> BTreeSet<Key> {
    let mut e = BTreeSet::new();
    let mut add = |f: &str, l: String| {
        e.insert((f.to_string(), l));
    };
    for p in [53u64, 101, 251, 1009] {
        for k in 0..12 {
            add("jv-nagao", format!("p{p}-{k}{}", if k >= 8 { "c" } else { "" }));
        }
    }
    for p in [271u64, 1039, 8191] {
        for k in 0..8 {
            add("gaudry-s4", format!("p{p}-{k}"));
        }
    }
    for p in [97u64, 1033, 4093] {
        for k in 0..8 {
            add("gaudry-d4", format!("p{p}-{k}"));
        }
    }
    for p in [13u64, 101] {
        for k in 0..2 {
            add("gaudry-s5", format!("p{p}-{k}"));
        }
    }
    for (n, d) in SHAPES {
        for p in DENSE_P {
            for k in 0..3 {
                add("dense", format!("n{n}-d{d}-p{}-{k}", 64 - p.leading_zeros()));
            }
        }
    }
    e
}

fn load_rows(cx: &mut Ctx, cards: &[&str], reps: usize, simd: &str, check_meta: bool) -> Vec<Row> {
    let mut rows = Vec::new();
    for &c in cards {
        let doc = cx.json(&format!("runs/{c}.json"));
        if check_meta {
            let card = cx.cards[c].clone();
            let o = opts(&card.argv);
            let primes = o.get("--p").map(|p| json!(p.split(',').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>())).unwrap_or(Value::Null);
            let want_env: BTreeMap<String, String> = if simd == "portable" {
                [("ZERO_DIM_LA_SIMD".to_string(), "portable".to_string())].into()
            } else {
                BTreeMap::new()
            };
            let ok = doc["families"] == json!(o["--families"]) && doc["per"] == json!(o["--per"].parse::<u64>().unwrap())
                && doc["reps"] == json!(reps) && o["--reps"] == reps.to_string()
                && doc["arms"] == json!(o.get("--arms").cloned().unwrap_or("dense,sparse,fast".into()))
                && doc["shapes"] == o.get("--shapes").map(|s| json!(s)).unwrap_or(Value::Null)
                && doc["primes"] == primes && doc["simd"] == json!(simd) && doc["fast_threads"] == json!(1)
                && card.env == want_env;
            cx.led.check(&format!("{c}: run meta matches card argv and env"), ok,
                         json!({"families": doc["families"], "reps": doc["reps"], "arms": doc["arms"], "simd": doc["simd"], "env": card.env}));
            cx.led.check(&format!("{c}: skipped list is empty"), doc["skipped"] == json!([]), doc["skipped"].clone());
        }
        let want_arms: Vec<String> = doc["arms"].as_str().unwrap().split(',').map(|s| s.to_string()).collect();
        let mut bad = Vec::new();
        for (si, v) in doc["rows"].as_array().unwrap().iter().enumerate() {
            let names: Vec<String> = v["arms"].as_array().unwrap().iter().map(|a| a["arm"].as_str().unwrap().to_string()).collect();
            let exp: Vec<String> = want_arms.iter().filter(|a| !(a.as_str() == "dense" && v["delta"].as_u64().unwrap() > 1024)).cloned().collect();
            if names != exp {
                bad.push(json!([v["label"], "arms", names]));
            }
            for a in v["arms"].as_array().unwrap() {
                let ms: Vec<f64> = a["ms"].as_array().unwrap().iter().map(num).collect();
                if ms.len() != reps || num(&a["median_ms"]) != upper_median(&ms) {
                    bad.push(json!([v["label"], a["arm"], "rounds/median"]));
                }
            }
            rows.push(Row { card: c.to_string(), si, v: v.clone() });
        }
        if check_meta {
            cx.led.check(&format!("{c}: arms present (dense iff delta <= 1024), {reps} rounds, median_ms = sorted(ms)[n/2]"), bad.is_empty(), json!(bad));
        }
    }
    rows
}

// --------------------------------------------------------------- blocks ---

type CountFn<'a> = Box<dyn Fn(&Row) -> Option<f64> + 'a>;

fn block(rows: &[Row], num_arm: &str, den_arm: &str, counted: &BTreeMap<String, CountFn>) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    for fam in FAMS {
        let rs: Vec<&Row> = rows.iter().filter(|r| r.v["family"] == json!(fam) && arm(&r.v, num_arm).is_some() && arm(&r.v, den_arm).is_some()).collect();
        if rs.is_empty() {
            continue;
        }
        let ratios: Vec<f64> = rs.iter().map(|r| med(&r.v, num_arm) / med(&r.v, den_arm)).collect();
        let mut cands = Map::new();
        for (name, f) in counted {
            let c: Option<Vec<f64>> = rs.iter().map(|r| f(r)).collect();
            if let Some(c) = c {
                cands.insert(name.clone(), json!({
                    "min": c.iter().cloned().fold(f64::INFINITY, f64::min),
                    "max": c.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    "geomean": gmean(&c),
                }));
            }
        }
        let deltas: Vec<u64> = rs.iter().map(|r| r.v["delta"].as_u64().unwrap()).collect();
        let ns: BTreeSet<u64> = rs.iter().map(|r| r.v["n"].as_u64().unwrap()).collect();
        let ps: BTreeSet<u64> = rs.iter().map(|r| r.v["p"].as_u64().unwrap()).collect();
        out.insert(fam.to_string(), json!({
            "systems": rs.len(),
            "delta": [deltas.iter().min(), deltas.iter().max()],
            "n": ns.into_iter().collect::<Vec<_>>(),
            "p": ps.into_iter().collect::<Vec<_>>(),
            "geomean": gmean(&ratios),
            "ci95": bootstrap(&ratios),
            "min": ratios.iter().cloned().fold(f64::INFINITY, f64::min),
            "max": ratios.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            "median_ms": {num_arm: upper_median(&rs.iter().map(|r| med(&r.v, num_arm)).collect::<Vec<_>>()),
                          den_arm: upper_median(&rs.iter().map(|r| med(&r.v, den_arm)).collect::<Vec<_>>())},
            "counted_candidates": Value::Object(cands),
            "_ratios": ratios,
        }));
    }
    out
}

struct Cmp {
    headline: Vec<Value>,
    mismatched: Vec<Value>,
    compared: usize,
    stochastic: Vec<Value>,
    notes: Vec<Value>,
}

impl Cmp {
    fn num(&mut self, name: &str, claimed: &Value, recomputed: f64, tol: f64, head: bool) -> bool {
        self.compared += 1;
        let ok = claimed.as_f64().map(|c| rel(c, recomputed) <= tol).unwrap_or(false);
        let item = json!({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": tol});
        if head {
            self.headline.push(item.clone());
        }
        if !ok {
            self.mismatched.push(item);
        }
        ok
    }
    fn eq(&mut self, name: &str, claimed: &Value, recomputed: &Value) -> bool {
        self.compared += 1;
        let ok = values_equal(claimed, recomputed);
        if !ok {
            self.mismatched.push(json!({"name": name, "claimed": claimed, "recomputed": recomputed}));
        }
        ok
    }
}

/// Equality that treats 177001 and 177001.0 alike (the summary writes some counts as floats).
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => {
            let (x, y) = (x.as_f64().unwrap(), y.as_f64().unwrap());
            x == y || rel(x, y) <= 1e-12
        }
        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| values_equal(p, q)),
        (Value::Object(x), Value::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| values_equal(v, w))),
        _ => a == b,
    }
}

fn compare_block(cmp: &mut Cmp, name: &str, claimed: &Value, rec: &BTreeMap<String, Value>, intended: &str, head: bool) {
    for (fam, cv) in claimed.as_object().unwrap() {
        if cv["geomean"].is_null() {
            cmp.eq(&format!("{name}.{fam} (absent)"), &json!(rec.contains_key(fam)), &json!(false));
            continue;
        }
        let Some(rv) = rec.get(fam) else {
            cmp.mismatched.push(json!({"name": format!("{name}.{fam}"), "claimed": "present", "recomputed": "no rows"}));
            continue;
        };
        for k in ["systems", "delta", "n", "p"] {
            cmp.eq(&format!("{name}.{fam}.{k}"), &cv[k], &rv[k]);
        }
        for k in ["geomean", "min", "max"] {
            cmp.num(&format!("{name}.{fam}.{k}"), &cv[k], num(&rv[k]), TOL, head);
        }
        for (a, v) in cv["median_ms"].as_object().unwrap() {
            cmp.num(&format!("{name}.{fam}.median_ms.{a}"), v, num(&rv["median_ms"][a]), TOL, false);
        }
        for i in 0..2 {
            cmp.stochastic.push(json!({"name": format!("{name}.{fam}.ci95[{i}]"), "claimed": cv["ci95"][i],
                                        "recomputed_own_bootstrap": rv["ci95"][i], "rel_diff": rel(num(&cv["ci95"][i]), num(&rv["ci95"][i]))}));
            // reproduced by the validator's own StdRng(20261027) bootstrap, so also compared at 1e-9
            cmp.num(&format!("{name}.{fam}.ci95[{i}]"), &cv["ci95"][i], num(&rv["ci95"][i]), TOL, head);
        }
        // counted ratio: which definition does the claimed figure follow?
        // A block may carry none (amendment 3: no fixed-counter counts exist for the
        // dense arm, the ablation or host A); `sparse_over_fast` must carry one.
        if cv.get("counted_ratio").map_or(true, |v| v.is_null()) {
            cmp.compared += 1;
            if name == "sparse_over_fast" {
                cmp.mismatched.push(json!({"name": format!("{name}.{fam}.counted_ratio"), "claimed": "absent", "recomputed": "required (R3 sparse/fast)"}));
            } else {
                cmp.notes.push(json!({"absent_counted_ratio": format!("{name}.{fam}")}));
            }
            continue;
        }
        let cands = rv["counted_candidates"].as_object().unwrap();
        let matches: Vec<String> = cands.iter()
            .filter(|(_, c)| ["min", "max", "geomean"].iter().all(|k| rel(num(&cv["counted_ratio"][*k]), num(&c[*k])) <= TOL))
            .map(|(n, _)| n.clone()).collect();
        let ok = matches.iter().any(|m| m == intended);
        cmp.compared += 1;
        let item = json!({"name": format!("{name}.{fam}.counted_ratio"), "intended_definition": intended,
                          "claimed": cv["counted_ratio"], "matches_definitions": matches,
                          "intended_value": cands.get(intended)});
        if ok {
            if head {
                for k in ["min", "max", "geomean"] {
                    cmp.headline.push(json!({"name": format!("{name}.{fam}.counted_ratio.{k}"), "claimed": cv["counted_ratio"][k],
                                             "recomputed": cands[intended][k], "rel_tol": TOL}));
                }
            }
        } else {
            cmp.mismatched.push(item);
        }
    }
}

// ------------------------------------------------------------------ main ---

pub fn main(args: &[String]) {
    let snap = full_commit(&need(args, "--snapshot"));
    let round1 = full_commit(&arg(args, "--round1").unwrap_or("e9bf8737a38fa4446acec23812eaaf085d8426a6".into()));
    let out = PathBuf::from(need(args, "--out"));
    let mut cx = Ctx { snap: snap.clone(), led: Ledger::default(), cards: BTreeMap::new(), hosts: BTreeMap::new() };

    // R2 raw records unchanged since round 1
    let mut changed = Vec::new();
    for c in R2_BENCH.iter().chain(&R2_ABL).chain(&R2_DLP).chain(&HOST_A) {
        let p = format!("runs/{c}.json");
        if git_try(&round1, &cx.p(&p)).map(|b| sha_hex(&b)) != Some(cx.sha(&p)) {
            changed.push(p);
        }
    }
    let (_, diff) = git(&["diff", "--name-only", &round1, &snap, "--", &format!("{STUDY}/cards"), &format!("{STUDY}/runs")]);
    let r2_touched: Vec<&str> = diff.lines().filter(|l| !l.contains("R3-")).collect();
    cx.led.check("R2, R2-A, R2-D and L1-L4 runs and cards byte-identical to the round-1 snapshot", changed.is_empty() && r2_touched.is_empty(),
                 json!({"runs_changed": changed, "cards_or_runs_touched": r2_touched}));

    receipts(&mut cx, &R2_BENCH.iter().chain(&R2_ABL).chain(&R2_DLP).cloned().collect::<Vec<_>>(), PIN_R2, true);
    receipts(&mut cx, &R3, PIN_R3, true);
    receipts(&mut cx, &HOST_A, ("", ""), false);
    let host_b: BTreeSet<String> = R2_BENCH.iter().chain(&R2_ABL).chain(&R2_DLP).chain(&R3).map(|c| cx.hosts[*c].clone()).collect();
    let host_a: BTreeSet<String> = HOST_A.iter().map(|c| cx.hosts[*c].clone()).collect();
    cx.led.check("every R2 and R3 card ran on one host (host B)", host_b.len() == 1, json!(host_b));
    cx.led.check("L1-L4 (host A) on a different host, beside only", host_a.len() == 1 && host_a.is_disjoint(&host_b), json!(host_a));
    for (c, lin) in [("R2-D1", "sparse"), ("R2-D2", "fast"), ("R3-D1", "sparse"), ("R3-D2", "fast")] {
        let card = cx.cards[c].clone();
        let mut want: BTreeMap<String, String> = [("JV_LIN".to_string(), lin.to_string())].into();
        if c.starts_with("R3") {
            want.insert("JV_RHO_NEG".into(), "1".into());
            want.insert("JV_DUMP_INSTANCE".into(), format!("{STUDY}/runs/{c}.instances.jsonl"));
        }
        let argv_ok = card.argv[1..] == ["--exp", "dlp", "--sizes", "53,101", "--seeds", "2", "--rho-runs", "4", "--check-every", "64", "--json"]
            .iter().map(|s| s.to_string()).chain([format!("{STUDY}/runs/{c}.json")]).collect::<Vec<_>>()[..];
        cx.led.check(&format!("{c}: env and argv as the protocol and amendment 3"), card.env == want && argv_ok, json!({"env": card.env, "argv": card.argv}));
    }
    {
        let card = cx.cards["R3-C"].clone();
        cx.led.check("R3-C: argv is amendment 3's command", card.argv[1..] == ["--families", "jv-nagao,gaudry-s4,gaudry-d4,gaudry-s5,dense", "--per", "8", "--reps", "1", "--arms", "sparse,fast", "--json"]
            .iter().map(|s| s.to_string()).chain([format!("{STUDY}/runs/R3-C.json")]).collect::<Vec<_>>()[..] && card.timeout == 7000 && card.env.is_empty(), json!(card.argv));
    }
    for c in R3 {
        let t = cx.cards[c].timeout;
        cx.led.check(&format!("{c}: run timeout 7000 s"), t == 7000, json!(t));
    }

    let main_rows = load_rows(&mut cx, &R2_BENCH, 7, "avx512f", true);
    let abl_rows = load_rows(&mut cx, &R2_ABL, 3, "portable", true);
    let r3_rows = load_rows(&mut cx, &["R3-C"], 1, "avx512f", true);
    let ha_rows = load_rows(&mut cx, &HOST_A, 7, "avx512f", false);
    let exp = expected_corpus();
    for (name, rows) in [("R2 main", &main_rows), ("R2 ablation", &abl_rows), ("R3-C", &r3_rows)] {
        let ks: Vec<Key> = rows.iter().map(|r| key(&r.v)).collect();
        let set: BTreeSet<Key> = ks.iter().cloned().collect();
        cx.led.check(&format!("{name}: exactly the protocol corpus, each system once (253)"), ks.len() == set.len() && set == exp,
                     json!({"n": ks.len(), "missing": exp.difference(&set).collect::<Vec<_>>(), "extra": set.difference(&exp).collect::<Vec<_>>()}));
    }
    let by: HashMap<Key, Row> = main_rows.iter().map(|r| (key(&r.v), r.clone())).collect();
    let by3: HashMap<Key, Row> = r3_rows.iter().map(|r| (key(&r.v), r.clone())).collect();

    // gates (R2)
    let lab = |r: &Row| format!("{}:{}", r.card, r.v["label"].as_str().unwrap());
    let disagree: Vec<String> = main_rows.iter().filter(|r| r.v["agree"] != json!(true)).map(lab).collect();
    let planted_lost: Vec<String> = main_rows.iter().filter(|r| r.v["family"] == json!("dense") && r.v["planted_found"] != json!(true)).map(lab).collect();
    let gave_up: Vec<String> = main_rows.iter().filter(|r| !arm(&r.v, "fast").unwrap()["gave_up"].is_null())
        .map(|r| format!("{}:{}", lab(r), arm(&r.v, "fast").unwrap()["gave_up"].as_str().unwrap())).collect();
    let sparse_fb: Vec<String> = main_rows.iter().filter(|r| arm(&r.v, "sparse").unwrap()["fallback"] == json!(true)).map(lab).collect();
    let sols_differ: Vec<String> = main_rows.iter().filter(|r| r.v["arms"].as_array().unwrap().iter().map(|a| a["sols"].to_string()).collect::<BTreeSet<_>>().len() != 1).map(lab).collect();
    let c_empty: Vec<String> = main_rows.iter().filter(|r| r.v["family"] == json!("jv-nagao") && r.v["label"].as_str().unwrap().ends_with('c') && arm(&r.v, "fast").unwrap()["sols"].as_u64().unwrap() < 1).map(lab).collect();
    let abl_dis: Vec<String> = abl_rows.iter().filter(|r| r.v["agree"] != json!(true)
        || r.v["arms"].as_array().unwrap().iter().any(|a| a["sols"] != arm(&by[&key(&r.v)].v, "fast").unwrap()["sols"])).map(lab).collect();
    cx.led.check("R2 gate: every arm agrees on every system", disagree.is_empty(), json!(disagree));
    cx.led.check("R2 gate: every dense-family planted point found", planted_lost.is_empty(), json!(planted_lost));
    cx.led.check("R2: solution counts equal across arms", sols_differ.is_empty(), json!(sols_differ));
    cx.led.check("R2: every constructed jv-nagao system has >= 1 solution", c_empty.is_empty(), json!(c_empty));
    cx.led.check("R2 ablation agrees and matches the main runs' solution counts", abl_dis.is_empty(), json!(abl_dis));
    // gates (R3-C)
    let mut r3_mismatch = Vec::new();
    let (mut same_si, mut sparse_eq, mut fast_other_eq, mut fast_roots_diff) = (0, 0, 0, 0);
    let mut fast_other_bad = Vec::new();
    for r in &r3_rows {
        let m = &by[&key(&r.v)];
        let s2 = arm(&m.v, "sparse").unwrap()["sols"].clone();
        let ok = r.v["agree"] == json!(true) && ["sparse", "fast"].iter().all(|a| arm(&r.v, a).unwrap()["sols"] == s2)
            && r.v["delta"] == m.v["delta"] && r.v["planted_found"] == m.v["planted_found"]
            && arm(&r.v, "fast").unwrap()["gave_up"] == arm(&m.v, "fast").unwrap()["gave_up"]
            && arm(&r.v, "sparse").unwrap()["fallback"] == arm(&m.v, "sparse").unwrap()["fallback"];
        if !ok {
            r3_mismatch.push(lab(r));
        }
        if r.si == m.si {
            same_si += 1;
            if muls(&r.v, "sparse") == muls(&m.v, "sparse") {
                sparse_eq += 1;
            }
            let (f3, f2) = (&arm(&r.v, "fast").unwrap()["fast"], &arm(&m.v, "fast").unwrap()["fast"]);
            let others = ["krylov_muls", "bm_muls", "nf_muls", "readout_muls", "delta", "dense_cols", "functionals_tried", "nnz", "nf_rows", "roots", "gave_up"];
            if others.iter().all(|k| f3[*k] == f2[*k]) {
                fast_other_eq += 1;
            } else {
                fast_other_bad.push(lab(r));
            }
            if f3["roots_muls"] != f2["roots_muls"] {
                fast_roots_diff += 1;
            }
        }
    }
    cx.led.check("R3-C gate: solution sets equal R2's on every system (sols per arm, agree, delta, planted, give-up, fallback)", r3_mismatch.is_empty(), json!(r3_mismatch));
    cx.led.check("R3-C: the counter fix leaves the sparse arm's counts unchanged (equal to R2 at the same si)", sparse_eq == same_si && same_si > 0,
                 json!({"same_si": same_si, "sparse_muls_equal": sparse_eq}));
    cx.led.check("R3-C: at the same si the fast arm's other counters (Krylov, BM, NF, read-out, delta, functionals, nnz, roots) equal R2's; only roots_muls moves",
                 fast_other_eq == same_si, json!({"same_si": same_si, "equal": fast_other_eq, "roots_muls_changed": fast_roots_diff, "bad": fast_other_bad}));

    // blocks
    let ratio = |a: CountFn<'static>, b: CountFn<'static>| -> CountFn<'static> { Box::new(move |r: &Row| Some(a(r)? / b(r)?)) };
    let by3s: &'static HashMap<Key, Row> = Box::leak(Box::new(by3.clone()));
    let byms: &'static HashMap<Key, Row> = Box::leak(Box::new(by.clone()));
    let r3s = |n: &'static str| -> CountFn<'static> { Box::new(move |r: &Row| Some(muls(&by3s[&key(&r.v)].v, n))) };
    let r2s = |n: &'static str| -> CountFn<'static> { Box::new(move |r: &Row| arm(&byms[&key(&r.v)].v, n).map(|a| num(&a["muls"]))) };
    let ownc = |n: &'static str| -> CountFn<'static> { Box::new(move |r: &Row| arm(&r.v, n).map(|a| num(&a["muls"]))) };
    let mk = |v: Vec<(&str, CountFn<'static>)>| -> BTreeMap<String, CountFn<'static>> { v.into_iter().map(|(k, f)| (k.to_string(), f)).collect() };
    let sof = block(&main_rows, "sparse", "fast", &mk(vec![
        ("R3 sparse/R3 fast", ratio(r3s("sparse"), r3s("fast"))),
        ("R2 sparse/R2 fast", ratio(ownc("sparse"), ownc("fast"))),
    ]));
    let dof = block(&main_rows, "dense", "fast", &mk(vec![
        ("R2 dense/R3 fast", ratio(ownc("dense"), r3s("fast"))),
        ("R2 dense/R2 fast", ratio(ownc("dense"), ownc("fast"))),
        ("R2 sparse/R2 fast", ratio(ownc("sparse"), ownc("fast"))),
        ("R3 sparse/R3 fast", ratio(r3s("sparse"), r3s("fast"))),
    ]));
    let abl = block(&abl_rows, "sparse", "fast", &mk(vec![
        ("R3 sparse/R3 fast", ratio(r3s("sparse"), r3s("fast"))),
        ("A sparse/A fast (pre-fix counter)", ratio(ownc("sparse"), ownc("fast"))),
    ]));
    let has = block(&ha_rows, "sparse", "fast", &mk(vec![
        ("R3 sparse/R3 fast", ratio(r3s("sparse"), r3s("fast"))),
        ("L sparse/L fast (pre-fix counter)", ratio(ownc("sparse"), ownc("fast"))),
    ]));
    let had = block(&ha_rows, "dense", "fast", &mk(vec![
        ("L dense/R3 fast", ratio(ownc("dense"), r3s("fast"))),
        ("L dense/L fast (pre-fix counter)", ratio(ownc("dense"), ownc("fast"))),
        ("L sparse/L fast (pre-fix counter)", ratio(ownc("sparse"), ownc("fast"))),
        ("R2 sparse/R2 fast", ratio(r2s("sparse"), r2s("fast"))),
    ]));
    let mut port = BTreeMap::new();
    for fam in FAMS {
        let pairs: Vec<f64> = abl_rows.iter().filter(|r| r.v["family"] == json!(fam)).map(|r| med(&r.v, "fast") / med(&by[&key(&r.v)].v, "fast")).collect();
        port.insert(fam.to_string(), gmean(&pairs));
    }
    let dense_rows: Vec<&Row> = main_rows.iter().filter(|r| r.v["family"] == json!("dense")).collect();
    let spearman = pearson(&avg_ranks(&dense_rows.iter().map(|r| med(&r.v, "sparse") / med(&r.v, "fast")).collect::<Vec<_>>()),
                           &avg_ranks(&dense_rows.iter().map(|r| r.v["delta"].as_f64().unwrap()).collect::<Vec<_>>()));
    let gave_up_frac = gave_up.len() as f64 / main_rows.len() as f64;
    let r3_ratio: Vec<f64> = r3_rows.iter().map(|r| muls(&r.v, "sparse") / muls(&r.v, "fast")).collect();
    let outside_fs = r3_ratio.iter().filter(|x| { let f = 1.0 / **x; !(0.75..=1.25).contains(&f) }).count();
    let outside_sf = r3_ratio.iter().filter(|x| !(0.75..=1.25).contains(*x)).count();
    let range_sf = [r3_ratio.iter().cloned().fold(f64::INFINITY, f64::min), r3_ratio.iter().cloned().fold(f64::NEG_INFINITY, f64::max)];
    let mut outside_by_family = Map::new();
    for fam in FAMS {
        let n = r3_rows.iter().filter(|r| r.v["family"] == json!(fam)).filter(|r| { let f = muls(&r.v, "fast") / muls(&r.v, "sparse"); !(0.75..=1.25).contains(&f) }).count();
        outside_by_family.insert(fam.to_string(), json!(n));
    }

    // ---------------------------------------------------------- DLP ---
    let rows_of = |c: &str| -> BTreeMap<(u64, u64), Value> {
        git_json(&snap, &format!("{STUDY}/runs/{c}.json")).as_array().unwrap().iter()
            .map(|r| ((r["p"].as_u64().unwrap(), r["seed"].as_u64().unwrap()), r.clone())).collect()
    };
    let (d1, d2, e1, e2) = (rows_of("R2-D1"), rows_of("R2-D2"), rows_of("R3-D1"), rows_of("R3-D2"));
    let keys: Vec<(u64, u64)> = vec![(53, 1), (53, 2), (101, 1), (101, 2)];
    cx.led.check("DLP: R2-D1, R2-D2, R3-D1, R3-D2 each cover (p, seed) = {53,101} x {1,2}",
                 [&d1, &d2, &e1, &e2].iter().all(|m| m.keys().cloned().collect::<Vec<_>>() == keys), Value::Null);
    let counted_fields: BTreeSet<&str> = ["lin_muls", "oracle_muls", "total_muls", "s", "s_over_rho", "c_cov", "relation_over_rho", "predicted_s_over_rho", "r", "r_last"].into();
    let neg_fields: BTreeSet<&str> = ["rho_neg", "rho_neg_s_mean", "s_over_rho_neg"].into();
    let mut dlp_rec = Vec::new();
    let mut band_rows = Vec::new();
    for k in &keys {
        let mut rec = Map::new();
        rec.insert("p".into(), json!(k.0));
        rec.insert("seed".into(), json!(k.1));
        for (tag, r2, r3) in [("sparse", &d1[k], &e1[k]), ("fast", &d2[k], &e2[k])] {
            let l = num(&r3["l"]);
            let oracle = ["weil_muls", "f4_muls", "lin_muls", "post_muls"].iter().map(|f| r3[*f].as_u64().unwrap()).sum::<u64>();
            let la = r3["la_ops"].as_u64().unwrap() * 16;
            let total = ["setup_muls", "base_muls", "stream_muls", "verify_muls"].iter().map(|f| r3[*f].as_u64().unwrap()).sum::<u64>() + oracle + la;
            let s = total as f64 / (num(&r3["c_add_e"]) * l.sqrt());
            let rho_s: Vec<f64> = r3["rho"].as_array().unwrap().iter().map(|x| num(&x["group_ops"]) / l.sqrt()).collect();
            let neg_s: Vec<f64> = r3["rho_neg"].as_array().unwrap().iter().map(|x| num(&x["group_ops"]) / l.sqrt()).collect();
            let rho_m = rho_s.iter().sum::<f64>() / rho_s.len() as f64;
            let neg_m = neg_s.iter().sum::<f64>() / neg_s.len() as f64;
            cx.led.check(&format!("R3 DLP {tag} {k:?}: oracle, la and total muls rebuilt from parts; S = total/(c_add_E sqrt l); rho and rho_neg S = group_ops/sqrt l; S/rho, S/rho_neg"),
                oracle == r3["oracle_muls"].as_u64().unwrap() && la == r3["la_muls"].as_u64().unwrap() && total == r3["total_muls"].as_u64().unwrap()
                && rel(s, num(&r3["s"])) <= TOL && rel(rho_m, num(&r3["rho_s_mean"])) <= TOL && rel(neg_m, num(&r3["rho_neg_s_mean"])) <= TOL
                && rel(s / rho_m, num(&r3["s_over_rho"])) <= TOL && rel(s / neg_m, num(&r3["s_over_rho_neg"])) <= TOL
                && r3["rho"].as_array().unwrap().iter().zip(&rho_s).all(|(x, v)| rel(*v, num(&x["s"])) <= TOL)
                && r3["rho_neg"].as_array().unwrap().iter().zip(&neg_s).all(|(x, v)| rel(*v, num(&x["s"])) <= TOL), Value::Null);
            let all_correct = r3["solved"] == json!(true) && r3["correct"] == json!(true)
                && r3["rho"].as_array().unwrap().iter().chain(r3["rho_neg"].as_array().unwrap()).all(|x| x["correct"] == json!(true))
                && r3["rho_neg"].as_array().unwrap().len() == 4 && r3["timed_out"] == json!(0) && r3["mismatches"] == json!(0);
            cx.led.check(&format!("R3 DLP {tag} {k:?}: solved, correct, all 4 rho and 4 rho_neg runs correct, no timeouts or mismatches"), all_correct, Value::Null);
            // R3 vs R2: relations, plain rho runs, solved, and only counted fields move
            let fields: BTreeSet<String> = r2.as_object().unwrap().keys().chain(r3.as_object().unwrap().keys()).cloned().collect();
            let differing: Vec<String> = fields.iter().filter(|f| !TIMING.contains(&f.as_str()) && r2.get(f.as_str()) != r3.get(f.as_str())).cloned().collect();
            let allowed = |f: &str| neg_fields.contains(f) || (tag == "fast" && counted_fields.contains(f));
            cx.led.check(&format!("R3 DLP {tag} {k:?}: relations, solved log, instance and plain rho runs equal R2-D; differing fields only {}",
                                  if tag == "sparse" { "the new rho_neg ones" } else { "lin_muls-derived counts and rho_neg" }),
                r2["relations"] == r3["relations"] && r2["rho"] == r3["rho"] && r2["correct"] == r3["correct"] && r2["l"] == r3["l"]
                && differing.iter().all(|f| allowed(f)), json!(differing));
            rec.insert(format!("{tag}"), json!({
                "s": s, "s_over_rho": s / rho_m, "s_over_rho_neg": s / neg_m, "rho_s_mean": rho_m, "rho_neg_s_mean": neg_m,
                "lin_muls": r3["lin_muls"], "lin_muls_r2": r2["lin_muls"], "lin_ms_r2": r2["lin_ms"], "f4_ms_r2": r2["f4_ms"],
                "wall_ms_r2": r2["wall_ms"], "relations": r3["relations"], "correct": r3["correct"],
                "rho_all_correct": all_correct,
                "rho_neg_runs": r3["rho_neg"], "rho_runs": r3["rho"], "l": r3["l"],
            }));
        }
        let (sp, fa) = (rec["sparse"].clone(), rec["fast"].clone());
        rec.insert("s_ratio_sparse_over_fast".into(), json!(num(&sp["s"]) / num(&fa["s"])));
        rec.insert("lin_ms_ratio_sparse_over_fast".into(), json!(num(&sp["lin_ms_r2"]) / num(&fa["lin_ms_r2"])));
        band_rows.push(rec.clone());
        dlp_rec.push(Value::Object(rec));
    }

    // ------------------------------------------------ rho reference band ---
    // rho_e_neg's unpriced native work (section 5a: canonicalisations, partition
    // hashes, table inserts and lookups), bounded from its emitted counters:
    //   per step: 1 table lookup, 1 insert, 1 partition hash of the current point;
    //   per candidate addition: 1 canonicalisation, 1 partition hash, 2 coefficient updates.
    //   candidates >= steps (each step tries one) and <= group_ops (each is a group op).
    //   lower bound u_lo = 3 steps + 2 steps = 5 steps (listed section-5a ops only);
    //   upper bound u_hi = 3 steps + 4 group_ops (all native non-group work).
    // The index calculus emits no unpriced counter: its S at weight w > 0 is a lower bound.
    let weights = [0.0f64, 0.1, 1.0];
    let mut band = Vec::new();
    let mut pooled_neg: Vec<(f64, f64, f64)> = Vec::new(); // (group_ops, steps, sqrt l) per distinct rho_neg run
    let mut fast_s = Vec::new();
    let mut sparse_s = Vec::new();
    let mut order_holds = true;
    for rec in &band_rows {
        let fa = &rec["fast"];
        let l = num(&fa["l"]);
        let runs = fa["rho_neg_runs"].as_array().unwrap();
        let same_runs = rec["sparse"]["rho_neg_runs"] == fa["rho_neg_runs"];
        cx.led.check(&format!("rho_neg runs identical in both arms' processes at {:?}", (rec["p"].clone(), rec["seed"].clone())), same_runs, Value::Null);
        let mut per_w = Map::new();
        for &w in &weights {
            let lo: f64 = runs.iter().map(|x| (num(&x["group_ops"]) + w * 5.0 * num(&x["steps"])) / l.sqrt()).sum::<f64>() / runs.len() as f64;
            let hi: f64 = runs.iter().map(|x| (num(&x["group_ops"]) + w * (3.0 * num(&x["steps"]) + 4.0 * num(&x["group_ops"]))) / l.sqrt()).sum::<f64>() / runs.len() as f64;
            let ic = num(&fa["s"]);
            order_holds &= ic > hi && num(&rec["sparse"]["s"]) > hi;
            per_w.insert(format!("{w}"), json!({"ic_fast_lower_bound": ic, "ic_sparse_lower_bound": rec["sparse"]["s"],
                                                 "rho_neg_lo": lo, "rho_neg_hi": hi, "ic_fast_over_rho_neg_hi": ic / hi, "ic_fast_over_rho_neg_lo": ic / lo}));
        }
        for x in runs {
            pooled_neg.push((num(&x["group_ops"]), num(&x["steps"]), l.sqrt()));
        }
        fast_s.push(num(&fa["s"]));
        sparse_s.push(num(&rec["sparse"]["s"]));
        band.push(json!({"p": rec["p"], "seed": rec["seed"], "fast_S": fa["s"], "sparse_S": rec["sparse"]["s"],
                         "rho_neg_S_mean": fa["rho_neg_s_mean"], "rho_S_mean": fa["rho_s_mean"],
                         "fast_S_over_rho_neg": fa["s_over_rho_neg"], "rho_neg_steps_over_sqrt_l": runs.iter().map(|x| num(&x["steps"]) / l.sqrt()).sum::<f64>() / runs.len() as f64,
                         "by_weight": Value::Object(per_w)}));
    }
    let mut pooled = Map::new();
    let pool_mean = |w: f64, hi: bool| -> f64 {
        pooled_neg.iter().map(|(g, s, r)| (g + w * if hi { 3.0 * s + 4.0 * g } else { 5.0 * s }) / r).sum::<f64>() / pooled_neg.len() as f64
    };
    let fast_mean = fast_s.iter().sum::<f64>() / fast_s.len() as f64;
    let sparse_mean = sparse_s.iter().sum::<f64>() / sparse_s.len() as f64;
    for &w in &weights {
        pooled.insert(format!("{w}"), json!({"ic_fast_mean_S_lower_bound": fast_mean, "ic_sparse_mean_S_lower_bound": sparse_mean,
            "rho_neg_mean_S_lo": pool_mean(w, false), "rho_neg_mean_S_hi": pool_mean(w, true),
            "ic_fast_over_rho_neg_hi": fast_mean / pool_mean(w, true), "ic_fast_over_rho_neg_lo": fast_mean / pool_mean(w, false)}));
    }
    let mut per_p = Map::new();
    for pp in [53u64, 101] {
        let recs: Vec<&Map<String, Value>> = band_rows.iter().filter(|r| r["p"] == json!(pp)).collect();
        let fm = recs.iter().map(|r| num(&r["fast"]["s"])).sum::<f64>() / recs.len() as f64;
        let runs: Vec<(f64, f64, f64)> = recs.iter().flat_map(|r| { let l = num(&r["fast"]["l"]).sqrt(); r["fast"]["rho_neg_runs"].as_array().unwrap().iter().map(move |x| (num(&x["group_ops"]), num(&x["steps"]), l)).collect::<Vec<_>>() }).collect();
        let mut ws = Map::new();
        for &w in &weights {
            let lo = runs.iter().map(|(g, s, r)| (g + w * 5.0 * s) / r).sum::<f64>() / runs.len() as f64;
            let hi = runs.iter().map(|(g, s, r)| (g + w * (3.0 * s + 4.0 * g)) / r).sum::<f64>() / runs.len() as f64;
            ws.insert(format!("{w}"), json!({"ic_fast_mean_S_lower_bound": fm, "rho_neg_mean_S_lo": lo, "rho_neg_mean_S_hi": hi, "ic_fast_over_rho_neg_hi": fm / hi, "ic_fast_over_rho_neg_lo": fm / lo}));
        }
        per_p.insert(format!("p{pp}"), json!({"instances": recs.len(), "rho_neg_runs": runs.len(), "by_weight": Value::Object(ws)}));
    }
    let rho_reference = json!({
        "pooled_by_p": Value::Object(per_p),
        "reference": "rho_e_neg (JV_RHO_NEG=1), same jv_cover process and Spec as the index calculus, after its clock",
        "unit": "group additions on E(F_{p^6}) per sqrt(l): IC = counted F_p muls / (c_add_E sqrt l), c_add_E = measured F_p muls of one E addition; rho = counted group operations (additions and doublings, setup included) / sqrt(l)",
        "rho_neg_unpriced_bounds": "per step: 1 lookup + 1 insert + 1 partition hash; per candidate addition: 1 canonicalisation + 1 hash + 2 coefficient updates; u_lo = 5 steps, u_hi = 3 steps + 4 group_ops (derived from emitted steps and group_ops; the binary emits no unpriced counter)",
        "ic_unpriced": "not emitted (F_p additions, hash lookups, F4 symbolic work); IC S at w > 0 is a lower bound",
        "per_instance": band,
        "pooled": Value::Object(pooled),
        "pooled_rho_neg_runs": pooled_neg.len(),
        "pooled_rho_neg_steps_over_sqrt_l": pooled_neg.iter().map(|(_, s, r)| s / r).sum::<f64>() / pooled_neg.len() as f64,
        "theory_negation_rho_steps_over_sqrt_l": (std::f64::consts::PI / 4.0).sqrt(),
        "order_holds_across_band": order_holds,
        "order": "rho_neg below both IC arms at every weight (IC lower bound > rho_neg upper bound)",
    });
    cx.led.check("rho band: IC (lower bound) above rho_neg (upper bound) on every instance at 0, 0.1 and 1 GAE", order_holds, Value::Null);

    // ------------------------------------------------- compare summary ---
    let claimed = cx.json("analysis/summary.json");
    let mut cmp = Cmp { headline: vec![], mismatched: vec![], compared: 0, stochastic: vec![], notes: vec![] };
    compare_block(&mut cmp, "sparse_over_fast", &claimed["sparse_over_fast"], &sof, "R3 sparse/R3 fast", true);
    compare_block(&mut cmp, "dense_over_fast", &claimed["dense_over_fast"], &dof, "R2 dense/R3 fast", true);
    compare_block(&mut cmp, "ablation_sparse_over_portable_fast", &claimed["ablation_sparse_over_portable_fast"], &abl, "R3 sparse/R3 fast", true);
    compare_block(&mut cmp, "host_a_sparse_over_fast", &claimed["host_a_sparse_over_fast"], &has, "R3 sparse/R3 fast", false);
    compare_block(&mut cmp, "host_a_dense_over_fast", &claimed["host_a_dense_over_fast"], &had, "L dense/R3 fast", false);
    for (fam, v) in claimed["portable_over_simd_fast"].as_object().unwrap() {
        cmp.num(&format!("portable_over_simd_fast.{fam}"), v, port[fam], TOL, true);
    }
    let pr = &claimed["predictions"];
    cmp.num("P2.spearman_rho", &pr["P2"]["spearman_rho"], spearman, TOL, true);
    cmp.eq("P2.systems", &pr["P2"]["systems"], &json!(dense_rows.len()));
    cmp.num("P3.gave_up_fraction", &pr["P3"]["gave_up_fraction"], gave_up_frac, TOL, true);
    for (fam, v) in pr["P1"]["by_family"].as_object().unwrap() {
        cmp.num(&format!("P1.by_family.{fam}.geomean"), &v[0], num(&sof[fam]["geomean"]), TOL, false);
        cmp.eq(&format!("P1.by_family.{fam}.ci95 = sparse_over_fast.ci95"), &v[1], &claimed["sparse_over_fast"][fam]["ci95"]);
    }
    for (fam, v) in pr["P5"]["by_family"].as_object().unwrap() {
        cmp.num(&format!("P5.by_family.{fam}"), v, num(&abl[fam]["geomean"]), TOL, false);
    }
    let p1 = FAMS.iter().all(|f| num(&sof[*f]["geomean"]) >= 1.5 && num(&claimed["sparse_over_fast"][*f]["ci95"][0]) > 1.0 && num(&sof[*f]["ci95"][0]) > 1.0);
    let p5 = FAMS.iter().all(|f| num(&abl[*f]["geomean"]) >= 1.2);
    let logs_ok = band_rows.iter().all(|r| r["sparse"]["correct"] == json!(true) && r["fast"]["correct"] == json!(true));
    let s10 = band_rows.iter().all(|r| (num(&r["fast"]["s"]) / num(&r["sparse"]["s"]) - 1.0).abs() <= 0.10);
    let lin_lower = band_rows.iter().all(|r| num(&r["fast"]["lin_ms_r2"]) < num(&r["sparse"]["lin_ms_r2"]));
    let within = outside_fs == 0;
    let p4 = logs_ok && within && s10 && lin_lower;
    cmp.eq("P1.held", &pr["P1"]["held"], &json!(p1));
    cmp.eq("P2.held", &pr["P2"]["held"], &json!(spearman > 0.0));
    cmp.eq("P3.held", &pr["P3"]["held"], &json!(gave_up_frac <= 0.05));
    cmp.eq("P5.held", &pr["P5"]["held"], &json!(p5));
    cmp.eq("P4.logs_correct", &pr["P4"]["logs_correct"], &json!(logs_ok));
    cmp.eq("P4.s_within_10pct", &pr["P4"]["s_within_10pct"], &json!(s10));
    cmp.eq("P4.lin_ms_lower_every_run", &pr["P4"]["lin_ms_lower_every_run"], &json!(lin_lower));
    cmp.eq("P4.counted_within_25pct_every_system", &pr["P4"]["counted_within_25pct_every_system"], &json!(within));
    cmp.eq("P4.held", &pr["P4"]["held"], &json!(p4));
    let out25 = pr["P4"]["counted_outside_25pct"].as_u64().unwrap() as usize;
    // direction convention: the summary states counted ratios as sparse/fast; P4 words it as fast relative to sparse
    cmp.compared += 1;
    if out25 == outside_sf || out25 == outside_fs {
        cmp.headline.push(json!({"name": format!("P4.counted_outside_25pct ({})", if out25 == outside_sf { "R3 sparse/fast outside [0.75, 1.25]" } else { "R3 fast/sparse outside [0.75, 1.25]" }),
                                 "claimed": out25, "recomputed": out25, "rel_tol": TOL}));
    } else {
        cmp.mismatched.push(json!({"name": "P4.counted_outside_25pct", "claimed": out25, "recomputed": [outside_sf, outside_fs]}));
    }
    cmp.num("P4.counted_ratio_range[0] (R3 sparse/fast)", &pr["P4"]["counted_ratio_range"][0], range_sf[0], TOL, true);
    cmp.num("P4.counted_ratio_range[1] (R3 sparse/fast)", &pr["P4"]["counted_ratio_range"][1], range_sf[1], TOL, true);
    cmp.notes.push(json!({"P4_outside_25pct": {"fast_over_sparse_direction": outside_fs, "sparse_over_fast_direction": outside_sf}}));
    // gates block
    let g = &claimed["gates"];
    cmp.eq("gates.systems", &g["systems"], &json!(main_rows.len()));
    cmp.eq("gates.disagree", &g["disagree"], &json!(disagree));
    cmp.eq("gates.planted_lost", &g["planted_lost"], &json!(planted_lost));
    cmp.eq("gates.fast_gave_up", &g["fast_gave_up"], &json!(gave_up));
    cmp.eq("gates.sparse_fell_back", &g["sparse_fell_back"], &json!(sparse_fb));
    cmp.eq("gates.ablation_disagree", &g["ablation_disagree"], &json!(abl_dis));
    cmp.eq("gates.r3_counts_mismatch", &g["r3_counts_mismatch"], &json!(r3_mismatch));
    cmp.eq("gates.pass", &g["pass"], &json!(disagree.is_empty() && planted_lost.is_empty() && r3_mismatch.is_empty()));
    cmp.eq("dlp_mismatch", &claimed["dlp_mismatch"], &json!([]));
    // DLP block
    for cd in claimed["dlp"].as_array().unwrap() {
        let k = (cd["p"].as_u64().unwrap(), cd["seed"].as_u64().unwrap());
        let r = band_rows.iter().find(|r| r["p"] == json!(k.0) && r["seed"] == json!(k.1)).unwrap();
        let t = format!("dlp[p={},seed={}]", k.0, k.1);
        for (i, a) in ["sparse", "fast"].iter().enumerate() {
            for (f, src, head) in [("s", "s", true), ("s_over_rho", "s_over_rho", true), ("s_over_rho_neg", "s_over_rho_neg", true),
                                    ("rho_s_mean", "rho_s_mean", false), ("rho_neg_s_mean", "rho_neg_s_mean", true),
                                    ("lin_ms", "lin_ms_r2", true), ("f4_ms", "f4_ms_r2", false), ("wall_ms", "wall_ms_r2", false),
                                    ("lin_muls", "lin_muls", true), ("lin_muls_r2", "lin_muls_r2", false)] {
                cmp.num(&format!("{t}.{f}.{a}"), &cd[f][i], num(&r[*a][src]), TOL, head);
            }
            cmp.eq(&format!("{t}.relations.{a}"), &cd["relations"][i], &r[*a]["relations"]);
            cmp.eq(&format!("{t}.correct.{a}"), &cd["correct"][i], &r[*a]["correct"]);
        }
        cmp.num(&format!("{t}.s_ratio_sparse_over_fast"), &cd["s_ratio_sparse_over_fast"], num(&r["s_ratio_sparse_over_fast"]), TOL, true);
        cmp.num(&format!("{t}.lin_ms_ratio_sparse_over_fast"), &cd["lin_ms_ratio_sparse_over_fast"], num(&r["lin_ms_ratio_sparse_over_fast"]), TOL, true);
        cmp.eq(&format!("{t}.rho_all_correct"), &cd["rho_all_correct"], &json!(r["sparse"]["rho_all_correct"] == json!(true) && r["fast"]["rho_all_correct"] == json!(true)));
    }
    // systems list
    let mut bad_sys = Vec::new();
    let claimed_sys = claimed["systems"].as_array().unwrap();
    let ck: BTreeSet<Key> = claimed_sys.iter().map(key).collect();
    cmp.eq("systems: keys = the 253 corpus systems", &json!(ck.len() == 253 && ck == exp), &json!(true));
    for s in claimed_sys {
        let k = key(s);
        let (m, r3) = (&by[&k], &by3[&k]);
        let fa = &arm(&m.v, "fast").unwrap()["fast"];
        let want = json!({
            "card": m.card, "n": m.v["n"], "p": m.v["p"], "delta": m.v["delta"], "f4_ms": m.v["f4_ms"], "sols": m.v["arms"][0]["sols"],
            "agree": m.v["agree"], "planted_found": m.v["planted_found"],
            "ms": m.v["arms"].as_array().unwrap().iter().map(|a| (a["arm"].as_str().unwrap().to_string(), a["median_ms"].clone())).collect::<Map<_, _>>(),
            "muls_r2": m.v["arms"].as_array().unwrap().iter().map(|a| (a["arm"].as_str().unwrap().to_string(), a["muls"].clone())).collect::<Map<_, _>>(),
            "muls": {"sparse": muls(&r3.v, "sparse"), "fast": muls(&r3.v, "fast")},
            "fast_phases_ms": (["nf_ms", "krylov_ms", "bm_ms", "roots_ms", "readout_ms"].iter().map(|f| (f.to_string(), fa[*f].clone())).collect::<Map<_, _>>()),
        });
        let got: Map<String, Value> = want.as_object().unwrap().keys().map(|f| (f.clone(), s[f].clone())).collect();
        if !values_equal(&Value::Object(got.clone()), &want) {
            bad_sys.push(json!({"system": [k.0, k.1], "claimed": got, "raw": want}));
        }
        cmp.compared += 1;
    }
    if !bad_sys.is_empty() {
        cmp.mismatched.push(json!({"name": "systems entries", "bad": bad_sys.iter().take(5).collect::<Vec<_>>(), "count": bad_sys.len()}));
    }
    // meta, skipped, host
    for (blk, cards) in [("meta", R2_BENCH.to_vec()), ("meta_ablation", R2_ABL.to_vec()), ("meta_host_a", HOST_A.to_vec()), ("meta_counts", vec!["R3-C"])] {
        for c in cards {
            let doc = cx.json(&format!("runs/{c}.json"));
            let cv = &claimed[blk][c];
            let want: Map<String, Value> = cv.as_object().map(|o| o.keys().map(|k| (k.clone(), doc[k].clone())).collect()).unwrap_or_default();
            cmp.eq(&format!("{blk}.{c}"), cv, &Value::Object(want));
        }
    }
    for (c, v) in claimed["skipped"].as_object().unwrap() {
        cmp.eq(&format!("skipped.{c}"), v, &cx.json(&format!("runs/{c}.json"))["skipped"]);
    }
    cmp.eq("host", &claimed["host"], &json!("B (amendment 2); counts R3 (amendment 3)"));
    let known: BTreeSet<&str> = ["ablation_sparse_over_portable_fast", "dense_over_fast", "dlp", "dlp_mismatch", "gates", "host", "host_a_dense_over_fast",
        "host_a_sparse_over_fast", "meta", "meta_ablation", "meta_counts", "meta_host_a", "portable_over_simd_fast", "predictions", "schema",
        "skipped", "sparse_over_fast", "systems", "tool"].into();
    let unmatched: Vec<String> = claimed.as_object().unwrap().keys().filter(|k| !known.contains(k.as_str())).cloned().collect();
    let worst_ci = cmp.stochastic.iter().map(|x| num(&x["rel_diff"])).fold(0.0, f64::max);
    let ci_all_above_1 = FAMS.iter().all(|f| num(&sof[*f]["ci95"][0]) > 1.0);
    cx.led.check("own bootstrap CI (sparse/fast) above 1 in every family", ci_all_above_1, Value::Null);
    cx.led.check("claimed bootstrap CIs within 5% of an independent StdRng bootstrap", worst_ci <= 0.05, json!(worst_ci));
    cx.led.check("every compared summary.json number matches its recomputation under its stated definition", cmp.mismatched.is_empty(),
                 json!(cmp.mismatched.iter().map(|m| m["name"].clone()).collect::<Vec<_>>()));
    cx.led.check("every top-level summary.json block was recomputed", unmatched.is_empty(), json!(unmatched));

    // ------------------------------------------------------------ write ---
    let mut raw = Vec::new();
    for c in R2_BENCH.iter().chain(&R2_ABL).chain(&R2_DLP).chain(&HOST_A).chain(&R3) {
        raw.push(format!("runs/{c}.json"));
    }
    raw.push("runs/R3-D1.instances.jsonl".into());
    raw.push("runs/R3-D2.instances.jsonl".into());
    for c in R2_BENCH.iter().chain(&R2_ABL).chain(&R2_DLP).chain(&R3) {
        for s in ["yaml", "receipt.json", "lscpu.stdout", "run.stderr", "run.stdout"] {
            raw.push(format!("cards/CARD-postgb-la-20261026-{c}.{s}"));
        }
    }
    for p in ["PROTOCOL.md", "PROTOCOL_AMENDMENT_1.md", "PROTOCOL_AMENDMENT_2.md", "PROTOCOL_AMENDMENT_3.md", "analysis/summary.json"] {
        raw.push(p.into());
    }
    let raw_data: Vec<Value> = raw.iter().map(|p| json!({"path": format!("../{p}"), "repo_path": cx.p(p), "sha256": cx.sha(p)})).collect();
    let strip = |m: BTreeMap<String, Value>| -> Value {
        Value::Object(m.into_iter().map(|(k, mut v)| { v.as_object_mut().unwrap().remove("_ratios"); (k, v) }).collect())
    };
    let status = if cx.led.problems.is_empty() { "pass" } else { "fail" };
    let report = json!({
        "schema": "postgb-la-validator-report/v2",
        "status": status,
        "independent": true,
        "independence": "validation/vtool (Rust) written by the validator from PROTOCOL.md and amendments 1-3; examples/postgb_la_summary.rs was not read or run",
        "tool": "validation/vtool recompute",
        "snapshot_commit": snap,
        "inputs_read_via": "git show <snapshot_commit>:<path>",
        "raw_data_paths_relative_to": format!("{STUDY}/validation"),
        "raw_data": raw_data,
        "headline": cmp.headline,
        "hosts": cx.hosts,
        "checks": cx.led.checks,
        "problems": cx.led.problems,
        "compared": cmp.compared,
        "mismatched": cmp.mismatched,
        "unmatched": unmatched,
        "stochastic_ci_comparison": cmp.stochastic,
        "notes": cmp.notes,
        "rho_reference": rho_reference,
        "recomputed": {
            "sparse_over_fast": strip(sof), "dense_over_fast": strip(dof), "ablation_sparse_over_portable_fast": strip(abl),
            "host_a_sparse_over_fast": strip(has), "host_a_dense_over_fast": strip(had), "portable_over_simd_fast": port,
            "P2_spearman_rho": spearman, "P3_gave_up_fraction": gave_up_frac,
            "P4": {"counted_outside_25pct_fast_over_sparse_by_family": outside_by_family, "counted_outside_25pct_fast_over_sparse": outside_fs, "counted_outside_25pct_sparse_over_fast": outside_sf,
                   "counted_ratio_range_sparse_over_fast": range_sf, "logs_correct": logs_ok, "s_within_10pct": s10,
                   "lin_ms_lower_every_run": lin_lower, "held": p4},
            "P1_held": p1, "P5_held": p5,
            "dlp": dlp_rec,
        },
    });
    write_json(&out, &report);
    println!("recompute: status {status}; {} checks, {} problems; {} compared, {} headline, {} mismatched; worst CI rel diff {worst_ci:.4}",
             report["checks"].as_array().unwrap().len(), report["problems"].as_array().unwrap().len(), cmp.compared,
             report["headline"].as_array().unwrap().len(), report["mismatched"].as_array().unwrap().len());
    for p in report["problems"].as_array().unwrap() {
        let s = p.to_string();
        println!("PROBLEM {}", &s[..s.len().min(700)]);
    }
    for m in report["mismatched"].as_array().unwrap() {
        let s = m.to_string();
        println!("MISMATCH {}", &s[..s.len().min(500)]);
    }
}
