//! Derived files of `research/bitop_ladder_20261019`, computed in Rust
//! (AGENTS.md, "Implementation language: no Python").
//!
//! ```text
//! cargo run --release --example bitop_ladder_analyse -- select   # Phase A
//! cargo run --release --example bitop_ladder_analyse -- gap      # gap to rho
//! cargo run --release --example bitop_ladder_analyse -- summary  # summary.json
//! cargo run --release --example bitop_ladder_analyse -- specs train|holdout
//! cargo run --release --example bitop_ladder_analyse -- specs ext <b> <arm> <sizes> <n>
//! ```
//!
//! Run from the repository root, after `cargo build --release --bin ecbench`.
//! The study directory can be overridden with `--study <dir>`.
//!
//! * `select` applies PROTOCOL.md's Phase A rule to the train sessions. It
//!   prices every train record with `ecbench work` (bitop/v1). Per curve size
//!   and arm family (`pair`, `rf3`), the winner is the candidate with the
//!   smallest mean bitops among candidates whose runs all verified and are all
//!   completely priced. A winner at either end of the grid tried so far needs
//!   an extension (one step of 2^(1/2) beyond it, not below 8; at most three a
//!   curve and arm). It writes `select/SELECTION.md` and
//!   `select/selection.json`, and `params.json` when no extension is needed;
//!   otherwise it prints one `specs ext …` command per extension and exits 3.
//! * `specs` writes PROTOCOL.md's frozen ecbench specs from `curves.jsonl`
//!   (and `params.json` for the hold-out), write-once; `--specs-out <dir>`
//!   writes them elsewhere, for comparison with the committed ones.
//! * `gap` writes `analysis/gap_to_rho.json`: each arm's mean bitop/v1 over
//!   rho-neg's, per curve, from `analysis/work.jsonl`
//!   (`ecbench work --counted`).
//! * `summary` writes `summary.json`, copying the bound fits, `params.json`
//!   and the gap table, with the SHA-256 of every source.
//!
//! These replace `select.py`, `make_specs.py` and two inline scripts that
//! produced the committed files; the Python files stay in the study as
//! provenance. The JSON is written in the layout of Python's
//! `json.dumps(indent=1)` (shortest round-trip floats, `1.0` for integral
//! floats, ASCII escapes), and means of integer counts are correctly
//! rounded, so the outputs are byte-identical to the committed ones.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

const MAX_EXT: u32 = 3;
const ARMS: [&str; 6] = [
    "rho-neg",
    "bsgs-neg",
    "kangaroo",
    "ic-pair-rule",
    "ic-pair-b32",
    "ic-rootfind-m3-rule",
];

/// A JSON value with key order kept as written.
enum J {
    Null,
    Int(i128),
    Float(f64),
    Str(String),
    Arr(Vec<J>),
    Obj(Vec<(String, J)>),
}

/// Python's `repr(float)`: shortest round-trip digits, fixed notation for
/// decimal exponents in [-4, 16), otherwise `d.ddde+XX`.
fn py_float(x: f64) -> String {
    assert!(x.is_finite(), "non-finite float {x}");
    if x == 0.0 {
        return if x.is_sign_negative() { "-0.0" } else { "0.0" }.into();
    }
    let e = format!("{x:e}");
    let (mant, exp) = e.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let (neg, mant) = match mant.strip_prefix('-') {
        Some(m) => (true, m),
        None => (false, mant),
    };
    let digits: String = mant.chars().filter(|c| *c != '.').collect();
    let mut s = String::new();
    if neg {
        s.push('-');
    }
    if (-4..16).contains(&exp) {
        if exp < 0 {
            s.push_str("0.");
            s.push_str(&"0".repeat((-exp - 1) as usize));
            s.push_str(&digits);
        } else {
            let point = exp as usize + 1;
            if digits.len() <= point {
                s.push_str(&digits);
                s.push_str(&"0".repeat(point - digits.len()));
                s.push_str(".0");
            } else {
                s.push_str(&digits[..point]);
                s.push('.');
                s.push_str(&digits[point..]);
            }
        }
    } else {
        s.push_str(&digits[..1]);
        if digits.len() > 1 {
            s.push('.');
            s.push_str(&digits[1..]);
        }
        let _ = write!(s, "e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs());
    }
    s
}

/// Python's `f"{x:.4g}"`.
fn py_g4(x: f64) -> String {
    let e = format!("{x:.3e}");
    let (mant, exp) = e.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    if (-4..4).contains(&exp) {
        let s = format!("{:.*}", (3 - exp) as usize, x);
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    } else {
        let m = if mant.contains('.') {
            mant.trim_end_matches('0').trim_end_matches('.')
        } else {
            mant
        };
        format!("{m}e{}{:02}", if exp < 0 { '-' } else { '+' }, exp.abs())
    }
}

fn py_str(s: &str, out: &mut String) {
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
            c if (c as u32) < 0x20 || (c as u32) > 0x7e => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    let _ = write!(out, "\\u{u:04x}");
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// `json.dumps(value, indent=1)`.
fn dumps(v: &J) -> String {
    fn go(v: &J, depth: usize, out: &mut String) {
        let pad = |out: &mut String, d: usize| {
            out.push('\n');
            out.push_str(&" ".repeat(d));
        };
        match v {
            J::Null => out.push_str("null"),
            J::Int(i) => {
                let _ = write!(out, "{i}");
            }
            J::Float(x) => out.push_str(&py_float(*x)),
            J::Str(s) => py_str(s, out),
            J::Arr(a) if a.is_empty() => out.push_str("[]"),
            J::Arr(a) => {
                out.push('[');
                for (i, x) in a.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    pad(out, depth + 1);
                    go(x, depth + 1, out);
                }
                pad(out, depth);
                out.push(']');
            }
            J::Obj(o) if o.is_empty() => out.push_str("{}"),
            J::Obj(o) => {
                out.push('{');
                for (i, (k, x)) in o.iter().enumerate() {
                    if i > 0 {
                        out.push(',');
                    }
                    pad(out, depth + 1);
                    py_str(k, out);
                    out.push_str(": ");
                    go(x, depth + 1, out);
                }
                pad(out, depth);
                out.push('}');
            }
        }
    }
    let mut s = String::new();
    go(v, 0, &mut s);
    s
}

/// A parsed JSON value as Python's `json.load` would hold it, with every
/// object's keys sorted (`sort_keys=True` on output).
fn from_value_sorted(v: &Value) -> J {
    match v {
        Value::Null => J::Null,
        Value::Bool(_) => panic!("booleans are not expected in these files"),
        Value::Number(n) => {
            let t = n.to_string();
            if t.contains(['.', 'e', 'E']) {
                J::Float(t.parse().unwrap())
            } else {
                J::Int(t.parse().unwrap())
            }
        }
        Value::String(s) => J::Str(s.clone()),
        Value::Array(a) => J::Arr(a.iter().map(from_value_sorted).collect()),
        Value::Object(o) => {
            let mut kv: Vec<(String, J)> = o
                .iter()
                .map(|(k, x)| (k.clone(), from_value_sorted(x)))
                .collect();
            kv.sort_by(|a, b| a.0.cmp(&b.0));
            J::Obj(kv)
        }
    }
}

fn sort_obj(mut kv: Vec<(String, J)>) -> J {
    kv.sort_by(|a, b| a.0.cmp(&b.0));
    J::Obj(kv)
}

/// `sum / n` correctly rounded, as Python's `int / int` gives it. Both are
/// exact in f64 below 2^53, so one IEEE division rounds correctly.
fn int_mean(sum: u128, n: usize) -> f64 {
    assert!(
        sum < 1 << 53 && n > 0,
        "sum {sum} of {n} values exceeds exact range"
    );
    sum as f64 / n as f64
}

fn sha256_file(p: &Path) -> String {
    let bytes = std::fs::read(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    hex::encode(Sha256::digest(&bytes))
}

fn read_json(p: &Path) -> Value {
    let t = std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    serde_json::from_str(&t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn write(p: &Path, s: &str) {
    std::fs::write(p, s).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
}

/// The record's bitops when `work` priced it, as an integer.
fn bitops_of(work: &Value) -> Option<u128> {
    match work.get("bitops")? {
        Value::String(s) if !s.is_empty() => Some(s.parse().unwrap()),
        Value::Number(n) => Some(n.to_string().parse().unwrap()),
        _ => None,
    }
}

fn select(study: &Path, ecbench: &Path) -> i32 {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(study.join("sessions"))
        .expect("sessions/")
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with("train-P")
        })
        .collect();
    dirs.sort();
    if dirs.is_empty() {
        eprintln!("no train sessions");
        return 1;
    }
    let out = Command::new(ecbench)
        .arg("work")
        .args(&dirs)
        .output()
        .expect("run ecbench work");
    assert!(
        out.status.success(),
        "ecbench work failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let out = String::from_utf8(out.stdout).unwrap();

    // (bits, family) -> size -> runs of (verified, complete, bitops)
    type Runs = Vec<(bool, bool, Option<u128>)>;
    let mut runs: BTreeMap<(i64, String), BTreeMap<u64, Runs>> = BTreeMap::new();
    let mut exts: BTreeMap<(i64, String), u32> = BTreeMap::new();
    for d in &dirs {
        let name = d.file_name().unwrap().to_str().unwrap();
        if name.contains("-ext") {
            let rest = name.split_once("-P").unwrap().1;
            let mut it = rest.split('-');
            let b: i64 = it.next().unwrap().parse().unwrap();
            let fam = it.next().unwrap().to_string();
            *exts.entry((b, fam)).or_default() += 1;
        }
    }
    for line in out.lines() {
        let r: Value = serde_json::from_str(line).unwrap();
        let arm = r["arm"].as_str().unwrap();
        if r["warmup"].as_bool().unwrap() || arm == "rho-neg" {
            continue;
        }
        let (fam, size) = arm.split_once("-N").unwrap();
        let b = r["log2_r"].as_f64().unwrap().round_ties_even() as i64;
        let w = r["work"].get("Ok").filter(|w| !w.is_null());
        let complete = w.and_then(|w| w.get("complete")).and_then(Value::as_bool) == Some(true);
        let x = w.and_then(bitops_of);
        runs.entry((b, fam.to_string()))
            .or_default()
            .entry(size.parse().unwrap())
            .or_default()
            .push((r["status"] == "verified", complete, x));
    }

    struct Row {
        b: i64,
        fam: String,
        size: u64,
        n: usize,
        verified: usize,
        mean: Option<f64>,
    }
    let mut rows = Vec::new();
    let mut params: BTreeMap<String, Vec<(String, Option<u64>)>> =
        [("pair".into(), vec![]), ("rf3".into(), vec![])].into();
    let mut needed: Vec<(i64, String, u64, u32)> = Vec::new();
    for ((b, fam), cands) in &runs {
        let mut ok: BTreeMap<u64, Option<f64>> = BTreeMap::new();
        for (&s, rs) in cands {
            let good = rs.iter().all(|(v, c, x)| *v && *c && x.is_some());
            let mean = good.then(|| int_mean(rs.iter().map(|r| r.2.unwrap()).sum(), rs.len()));
            ok.insert(s, mean);
            rows.push(Row {
                b: *b,
                fam: fam.clone(),
                size: s,
                n: rs.len(),
                verified: rs.iter().filter(|r| r.0).count(),
                mean,
            });
        }
        let fam_params = params.entry(fam.clone()).or_default();
        // min over eligible sizes; the first of equal means wins, as min() does
        let mut win: Option<(u64, f64)> = None;
        for (&s, m) in &ok {
            if let Some(m) = *m {
                if win.map_or(true, |(_, w)| m < w) {
                    win = Some((s, m));
                }
            }
        }
        let Some((win, _)) = win else {
            fam_params.push((b.to_string(), None));
            continue;
        };
        fam_params.push((b.to_string(), Some(win)));
        let first = *cands.keys().next().unwrap();
        let last = *cands.keys().next_back().unwrap();
        let edge = if win == first && win > 8 {
            Some(8.max((win as f64 / 2f64.sqrt()).round_ties_even() as u64))
        } else if win == last {
            Some((win as f64 * 2f64.sqrt()).round_ties_even() as u64)
        } else {
            None
        };
        let done = exts.get(&(*b, fam.clone())).copied().unwrap_or(0);
        if let Some(edge) = edge {
            if !cands.contains_key(&edge) && done < MAX_EXT {
                needed.push((*b, fam.clone(), edge, done + 1));
            }
        }
    }
    let selected = |b: i64, fam: &str| -> Option<u64> {
        params
            .get(fam)
            .and_then(|v| v.iter().find(|(k, _)| *k == b.to_string()))
            .and_then(|(_, s)| *s)
    };

    let sel = study.join("select");
    std::fs::create_dir_all(&sel).unwrap();
    let mut md = String::from(
        "# Phase A selection (bitop/v1, train panel)\n\n\
         Generated by `select.py` from `sessions/train-P*` (PROTOCOL.md, Phase A).\n\n\
         | bits | arm | base | runs | verified | mean bitops | log2 | selected |\n\
         |--:|:--|--:|--:|--:|--:|--:|:--|\n",
    );
    for r in &rows {
        let mark = if selected(r.b, &r.fam) == Some(r.size) {
            "**yes**"
        } else {
            ""
        };
        let (mean, log2) = match r.mean {
            None => ("—".to_string(), "—".to_string()),
            Some(m) => (py_g4(m), format!("{:.3}", m.log2())),
        };
        let _ = writeln!(
            md,
            "| {} | {} | {} | {} | {} | {mean} | {log2} | {mark} |",
            r.b, r.fam, r.size, r.n, r.verified
        );
    }
    write(&sel.join("SELECTION.md"), &md);

    let opt = |s: Option<u64>| s.map_or(J::Null, |s| J::Int(s as i128));
    let params_j = |params: &BTreeMap<String, Vec<(String, Option<u64>)>>| -> Vec<(String, J)> {
        // insertion order: "pair" then "rf3", then any other family
        let mut fams: Vec<&String> = params.keys().collect();
        fams.sort_by_key(|f| match f.as_str() {
            "pair" => 0,
            "rf3" => 1,
            _ => 2,
        });
        fams.into_iter()
            .map(|f| {
                (
                    f.clone(),
                    J::Obj(
                        params[f]
                            .iter()
                            .map(|(k, s)| (k.clone(), opt(*s)))
                            .collect(),
                    ),
                )
            })
            .collect()
    };
    let selection = J::Obj(vec![
        (
            "rows".into(),
            J::Arr(
                rows.iter()
                    .map(|r| {
                        J::Arr(vec![
                            J::Int(r.b as i128),
                            J::Str(r.fam.clone()),
                            J::Int(r.size as i128),
                            J::Int(r.n as i128),
                            J::Int(r.verified as i128),
                            r.mean.map_or(J::Null, J::Float),
                        ])
                    })
                    .collect(),
            ),
        ),
        ("selected".into(), J::Obj(params_j(&params))),
        (
            "extensions_needed".into(),
            J::Arr(
                needed
                    .iter()
                    .map(|(b, f, e, n)| {
                        J::Arr(vec![
                            J::Int(*b as i128),
                            J::Str(f.clone()),
                            J::Int(*e as i128),
                            J::Int(*n as i128),
                        ])
                    })
                    .collect(),
            ),
        ),
    ]);
    write(&sel.join("selection.json"), &(dumps(&selection) + "\n"));
    if !needed.is_empty() {
        for (b, fam, edge, n) in &needed {
            println!("cargo run --release --example bitop_ladder_analyse -- specs ext {b} {fam} {edge} {n}");
        }
        return 3;
    }
    let mut p = params_j(&params);
    for (_, fam) in p.iter_mut() {
        if let J::Obj(kv) = fam {
            kv.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
    p.push((
        "_comment".into(),
        J::Str(
            "Frozen by select.py from the train panel (PROTOCOL.md Phase A): per size, the base \
             with the smallest mean bitop/v1 cost among candidates verified and completely priced \
             on every train target."
                .into(),
        ),
    ));
    write(&study.join("params.json"), &(dumps(&sort_obj(p)) + "\n"));
    println!("params.json written");
    0
}

fn gap(study: &Path) -> i32 {
    let text = std::fs::read_to_string(study.join("analysis/work.jsonl")).expect("work.jsonl");
    let mut m: BTreeMap<(i64, String), Vec<u128>> = BTreeMap::new();
    let mut inc: BTreeMap<String, u32> = BTreeMap::new();
    for line in text.lines() {
        let r: Value = serde_json::from_str(line).unwrap();
        let arm = r["arm"].as_str().unwrap().to_string();
        let Some(w) = r["work"].get("Ok").filter(|w| !w.is_null()) else {
            *inc.entry(arm).or_default() += 1;
            continue;
        };
        // an incomplete record is counted here and still averaged, as the
        // committed gap table was built
        if w["complete"].as_bool() != Some(true) {
            *inc.entry(arm.clone()).or_default() += 1;
        }
        let b = r["log2_r"].as_f64().unwrap().round_ties_even() as i64;
        m.entry((b, arm))
            .or_default()
            .push(bitops_of(w).expect("bitops"));
    }
    println!("incomplete/err: {inc:?}");
    let mean = |v: &Vec<u128>| int_mean(v.iter().sum(), v.len());
    let mut rows = Vec::new();
    for b in 16..=40i64 {
        let rho = mean(&m[&(b, "rho-neg".to_string())]);
        let mut row = vec![J::Int(b as i128)];
        for a in ARMS {
            row.push(match m.get(&(b, a.to_string())) {
                Some(v) if !v.is_empty() => J::Float(mean(v) / rho),
                _ => J::Null,
            });
        }
        rows.push(J::Arr(row));
    }
    let out = J::Obj(vec![
        (
            "note".into(),
            J::Str(
                "mean bitop/v1 per arm over mean bitop/v1 of rho-neg, per curve, from \
                 analysis/work.jsonl (ecbench work --counted)"
                    .into(),
            ),
        ),
        (
            "arms".into(),
            J::Arr(ARMS.iter().map(|a| J::Str(a.to_string())).collect()),
        ),
        ("rows".into(), J::Arr(rows)),
    ]);
    // json.dump writes no trailing newline
    write(&study.join("analysis/gap_to_rho.json"), &dumps(&out));
    println!("analysis/gap_to_rho.json written");
    0
}

fn summary(study: &Path) -> i32 {
    let mut bound_files: Vec<PathBuf> = std::fs::read_dir(study.join("analysis/bounds"))
        .expect("analysis/bounds/")
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    bound_files.sort();
    let mut fits = Vec::new();
    for f in &bound_files {
        let d = read_json(f);
        let (fi, c) = (&d["fit"], &d["constant"]["ratio_to_floor"]);
        let rel = format!(
            "analysis/bounds/{}",
            f.file_name().unwrap().to_str().unwrap()
        );
        let key = f.file_stem().unwrap().to_str().unwrap().to_string();
        let v = |x: &Value| from_value_sorted(x);
        fits.push((
            key,
            sort_obj(vec![
                ("bound_id".into(), v(&d["bound_id"])),
                ("tier".into(), v(&d["domain"]["tier"])),
                ("sizes".into(), v(&fi["sizes"])),
                ("runs".into(), v(&fi["points"])),
                ("alpha_bitops".into(), v(&fi["alpha"])),
                ("alpha_bitops_ci95".into(), v(&fi["alpha_ci95"])),
                ("alpha_additions".into(), v(&fi["alpha_in_additions"])),
                (
                    "alpha_additions_ci95".into(),
                    v(&fi["alpha_in_additions_ci95"]),
                ),
                ("ratio_to_floor".into(), v(&c["value"])),
                ("ratio_to_floor_ci95".into(), v(&c["ci95"])),
                ("admissibility".into(), v(&d["admissibility"]["status"])),
                ("file".into(), J::Str(rel)),
                ("sha256".into(), J::Str(sha256_file(f))),
            ]),
        ));
    }
    let n = fits.len();
    let sources: BTreeSet<&str> = [
        "analysis/gap_to_rho.json",
        "analysis/work.jsonl",
        "params.json",
        "PROTOCOL.md",
    ]
    .into();
    let out =
        sort_obj(vec![
        ("schema".into(), J::Str("bitop-ladder-summary/v1".into())),
        ("study".into(), J::Str("research/bitop_ladder_20261019".into())),
        ("unit".into(), J::Str("bitop/v1".into())),
        (
            "note".into(),
            J::Str(
                "Every value is copied from ecbench outputs named here: bound fits (ecbench bound \
                 fit --unit bitop/v1) and per-curve mean bitop/v1 of each arm over rho-neg \
                 (analysis/gap_to_rho.json, from ecbench work --counted). Nothing is measured or \
                 computed by the builder."
                    .into(),
            ),
        ),
        ("params".into(), from_value_sorted(&read_json(&study.join("params.json")))),
        ("fits".into(), sort_obj(fits)),
        (
            "gap_to_rho".into(),
            from_value_sorted(&read_json(&study.join("analysis/gap_to_rho.json"))),
        ),
        (
            "sources".into(),
            sort_obj(
                sources
                    .into_iter()
                    .map(|p| (p.to_string(), J::Str(sha256_file(&study.join(p)))))
                    .collect(),
            ),
        ),
    ]);
    // json.dump writes no trailing newline
    write(&study.join("summary.json"), &dumps(&out));
    println!("summary.json written ({n} fits)");
    0
}

const MAX_TRIALS: &str = "1000000000000";
const PAIR: &str = "mitm:negation_folded=1";
const ROOTFIND: &str = "rootfind-subtract:m=3";
const RF_GRID: [u64; 6] = [8, 16, 32, 64, 128, 256];
const RULE_MAX: i64 = 40;
const SMALL_MAX: i64 = 28;

/// `curves.jsonl` by field size, each curve's keys in file order. Every line
/// is one flat object of strings and integers.
fn curves(study: &Path) -> BTreeMap<i64, J> {
    let text = std::fs::read_to_string(study.join("curves.jsonl")).expect("curves.jsonl");
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let body = line.trim().trim_start_matches('{').trim_end_matches('}');
        let mut kv = Vec::new();
        let mut p = None;
        for field in body.split(", ") {
            let (k, v) = field.split_once(": ").expect("flat curve object");
            let k = k.trim_matches('"').to_string();
            let v = match v.strip_prefix('"') {
                Some(s) => J::Str(s.trim_end_matches('"').to_string()),
                None => J::Int(v.parse().expect("integer")),
            };
            if let (true, J::Int(x)) = (k == "p", &v) {
                p = Some(*x);
            }
            kv.push((k, v));
        }
        let p = p.expect("curve without p");
        out.insert(128 - p.leading_zeros() as i64, J::Obj(kv));
    }
    out
}

fn pair_grid(b: i64) -> Vec<u64> {
    let g = 32.0 * 2f64.powf(0.33 * (b - 20) as f64);
    let sizes: BTreeSet<u64> = (-4..3)
        .map(|k| (g * 2f64.powf(k as f64 / 2.0)).round_ties_even() as u64)
        .collect();
    sizes.into_iter().filter(|s| *s >= 8).collect()
}

fn obj(kv: Vec<(&str, J)>) -> J {
    J::Obj(kv.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

fn s(x: &str) -> J {
    J::Str(x.to_string())
}

fn ic_arm(name: &str, role: &str, oracle: &str, size: u64) -> J {
    obj(vec![
        ("name", s(name)),
        ("role", s(role)),
        (
            "method",
            obj(vec![
                ("id", s("ic.pipeline")),
                (
                    "params",
                    obj(vec![
                        ("factor_base", s(&format!("prime-abscissa:size={size}"))),
                        ("oracle", s(oracle)),
                        ("max_trials", s(MAX_TRIALS)),
                    ]),
                ),
            ]),
        ),
    ])
}

fn generic_arm(name: &str, role: &str, method: &str) -> J {
    obj(vec![
        ("name", s(name)),
        ("role", s(role)),
        ("method", obj(vec![("id", s(method))])),
    ])
}

fn spec(
    label: &str,
    curve: J,
    arms: Vec<J>,
    targets: i128,
    tseed: i128,
    seed: i128,
    timeout: i128,
) -> J {
    obj(vec![
        ("schema", s("ecbench.spec/v1")),
        ("label", s(label)),
        (
            "description",
            s(
                "research/bitop_ladder_20261019/PROTOCOL.md, frozen 2026-10-19. \
               Counts only (bitop/v1); public synthetic curve; no security claim.",
            ),
        ),
        (
            "workloads",
            obj(vec![
                ("curves", J::Arr(vec![curve])),
                ("targets_per_curve", J::Int(targets)),
                ("target_seed", J::Int(tseed)),
                ("target_kind", s("public")),
            ]),
        ),
        ("arms", J::Arr(arms)),
        (
            "measurement",
            obj(vec![
                ("rounds", J::Int(1)),
                ("warmup", J::Int(0)),
                ("order", s("alternate")),
                ("seed", J::Int(seed)),
                ("isolation_required", s("L0")),
                ("timeout_seconds", J::Int(timeout)),
            ]),
        ),
    ])
}

fn train_arms(pair: &[u64], rf: &[u64]) -> Vec<J> {
    let mut arms = vec![generic_arm("rho-neg", "reference", "rho.negation")];
    arms.extend(
        pair.iter()
            .map(|&n| ic_arm(&format!("pair-N{n}"), "candidate", PAIR, n)),
    );
    arms.extend(
        rf.iter()
            .map(|&n| ic_arm(&format!("rf3-N{n}"), "candidate", ROOTFIND, n)),
    );
    arms
}

fn train_targets(b: i64) -> i128 {
    if b <= 34 {
        8
    } else {
        4
    }
}

/// Write a spec once; specs are frozen inputs.
fn write_spec(dir: &Path, name: &str, spec: &J) -> Result<(), String> {
    std::fs::create_dir_all(dir).unwrap();
    let path = dir.join(name);
    if path.exists() {
        return Err(format!("{} exists; specs are write-once", path.display()));
    }
    write(&path, &(dumps(spec) + "\n"));
    println!("specs/{name}");
    Ok(())
}

/// PROTOCOL.md's specs: `train` (Phase A), `ext <b> <arm> <sizes> <n>` (one
/// selection extension) and `holdout` (Phase B, from `params.json`).
fn specs(study: &Path, out: &Path, args: &[String]) -> Result<(), String> {
    let cs = curves(study);
    let curve = |b: i64| cs.get(&b).map(clone_j).ok_or(format!("no {b}-bit curve"));
    match args.first().map(String::as_str) {
        Some("train") => {
            for b in 16..=RULE_MAX {
                let rf: &[u64] = if b <= SMALL_MAX { &RF_GRID } else { &[] };
                let label = format!("bitop ladder train panel: {b}-bit a=-3 prime-order curve");
                let arms = train_arms(&pair_grid(b), rf);
                let sp = spec(
                    &label,
                    curve(b)?,
                    arms,
                    train_targets(b),
                    20261019100,
                    20261019101,
                    1800,
                );
                write_spec(out, &format!("train-P{b}.json"), &sp)?;
            }
        }
        Some("ext") if args.len() == 5 => {
            let b: i64 = args[1].parse().map_err(|e| format!("bits: {e}"))?;
            let arm = args[2].as_str();
            let sizes: Vec<u64> = args[3]
                .split(',')
                .map(|x| x.parse().map_err(|e| format!("size {x}: {e}")))
                .collect::<Result<_, _>>()?;
            let n: u32 = args[4].parse().map_err(|e| format!("n: {e}"))?;
            let pair = if arm == "pair" { sizes.clone() } else { vec![] };
            let rf = if arm == "rf3" { sizes } else { vec![] };
            let label =
                format!("bitop ladder train panel extension {n}: {b}-bit a=-3 prime-order curve");
            let sp = spec(
                &label,
                curve(b)?,
                train_arms(&pair, &rf),
                train_targets(b),
                20261019100,
                20261019101,
                1800,
            );
            write_spec(out, &format!("train-P{b}-{arm}-ext{n}.json"), &sp)?;
        }
        Some("holdout") => {
            let params = read_json(&study.join("params.json"));
            let base = |fam: &str, b: i64| -> Option<u64> {
                params[fam]
                    .get(b.to_string())
                    .and_then(Value::as_u64)
                    .filter(|v| *v != 0)
            };
            for b in 16..=RULE_MAX {
                let mut arms = vec![
                    generic_arm("rho-neg", "reference", "rho.negation"),
                    generic_arm("rho-neg-aa", "control", "rho.negation"),
                    generic_arm("bsgs-neg", "baseline", "bsgs.negation"),
                    generic_arm("kangaroo", "baseline", "kangaroo.vow"),
                ];
                if let Some(n) = base("pair", b) {
                    arms.push(ic_arm("ic-pair-rule", "candidate", PAIR, n));
                }
                if b <= SMALL_MAX {
                    arms.push(ic_arm("ic-pair-b32", "candidate", PAIR, 32));
                    if let Some(n) = base("rf3", b) {
                        arms.push(ic_arm("ic-rootfind-m3-rule", "candidate", ROOTFIND, n));
                    }
                }
                let label = format!("bitop ladder hold-out: {b}-bit a=-3 prime-order curve");
                let sp = spec(&label, curve(b)?, arms, 80, 20261019200, 20261019201, 3600);
                write_spec(out, &format!("holdout-P{b}.json"), &sp)?;
            }
        }
        _ => {
            return Err(
                "specs train | specs ext <b> <pair|rf3> <size>[,<size>...] <n> | specs holdout"
                    .into(),
            )
        }
    }
    Ok(())
}

fn clone_j(v: &J) -> J {
    match v {
        J::Null => J::Null,
        J::Int(i) => J::Int(*i),
        J::Float(x) => J::Float(*x),
        J::Str(s) => J::Str(s.clone()),
        J::Arr(a) => J::Arr(a.iter().map(clone_j).collect()),
        J::Obj(o) => J::Obj(o.iter().map(|(k, x)| (k.clone(), clone_j(x))).collect()),
    }
}

const USAGE: &str = "usage: bitop_ladder_analyse [--study DIR] [--ecbench BIN] [--specs-out DIR] \
                     select | gap | summary | specs (train | ext <b> <arm> <sizes> <n> | holdout)";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut study = PathBuf::from("research/bitop_ladder_20261019");
    let mut ecbench = PathBuf::from("target/release/ecbench");
    let mut specs_out = None;
    let mut rest = Vec::new();
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--study" => study = it.next().expect("--study <dir>").into(),
            "--ecbench" => ecbench = it.next().expect("--ecbench <path>").into(),
            "--specs-out" => specs_out = Some(PathBuf::from(it.next().expect("--specs-out <dir>"))),
            _ => rest.push(a),
        }
    }
    let code = match rest.first().map(String::as_str) {
        Some("select") if rest.len() == 1 => select(&study, &ecbench),
        Some("gap") if rest.len() == 1 => gap(&study),
        Some("summary") if rest.len() == 1 => summary(&study),
        Some("specs") => {
            let out = specs_out.unwrap_or_else(|| study.join("specs"));
            match specs(&study, &out, &rest[1..]) {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            }
        }
        _ => {
            eprintln!("{USAGE}");
            2
        }
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floats_print_as_python_repr() {
        assert_eq!(py_float(1.0), "1.0");
        assert_eq!(py_float(96205318.0), "96205318.0");
        assert_eq!(py_float(0.5578589678023855), "0.5578589678023855");
        assert_eq!(py_float(1e16), "1e+16");
        assert_eq!(py_float(1.5e-5), "1.5e-05");
        assert_eq!(py_float(0.0001), "0.0001");
        assert_eq!(py_float(-2.5), "-2.5");
        assert_eq!(py_g4(59101714.0), "5.91e+07");
        assert_eq!(py_g4(96205318.0), "9.621e+07");
        assert_eq!(py_g4(12.5), "12.5");
    }
}
