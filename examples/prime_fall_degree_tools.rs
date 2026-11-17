//! Tools of `research/prime_fall_degree_20261027/` (PREREGISTRATION.md):
//!
//! - `score DIR`: read the closure rows (`DIR/runs/[ABC]-*.jsonl`, engine
//!   `hky_fp_tower`) and the F4 rows on the same systems
//!   (`DIR/runs/D*.jsonl`), apply the §6 rules as registered, check gate G2
//!   (`d_F ≤ D` on every draw with both) and the repeat check of Amendment 2,
//!   and write `DIR/TABLE.md` and `DIR/score.json`.
//! - `gate3 DIR PILOT [--max-n N] [--jobs J] [--budget S]`: gate G3. For every
//!   closure row (random target, tower or null control) with `d_F = d`, run
//!   capped F4 (`PILOT --engine tower --cap c --targets k`) on the same
//!   system at `c = d − 1` (must not refute) and `c = d` (must refute). A row
//!   without `1` up to its last completed closure `c` is run at `c` (must not
//!   refute). The system's `x_r` is compared. Writes `DIR/gates/g3.jsonl`.
//! - `ns-degree DIR [--t-max T]`: for each Kummer `m = 2` closure row, the
//!   degree `2 + popdeg(f⁻¹)` of the only Nullstellensatz certificate, which
//!   needs no fall (see `derive_ns` below). Writes `DIR/derived/ns_degree.jsonl`.
//!
//! - `spec-chain DIR [--cmax C] [--points P] [--m-max M]`: the
//!   specialisation bound. Evaluating the tower variables at a point of the
//!   grid `V^m` is a ring map `φ: R → F_p[u]` that never raises degree, and
//!   `φ(V_{F,c}) ⊆ V_{φF,c}` (by induction on the closure: a tower variable
//!   maps to a constant, `u_k` to itself). So an inconsistent system has
//!   `d_F(F) ≥ d_F(φF)`, and `φF` is the `S₃` chain with constant summands in
//!   `m − 2` free unknowns. For every Kummer chain row (`m ≥ 3`) this
//!   computes `d_F(φF)` at `P` grid points and checks the row's `d_F`
//!   against their maximum (a violation is a bug). Rows past the closure's
//!   reach get a proved lower bound. Writes `DIR/derived/spec_chain.jsonl`.
//!
//! - `spec-1d DIR [--t-max T] [--draws K] [--cmax C]` (Amendment 4): the
//!   one-summand specialisation. Evaluating `z_2` at `ζ^j` is a ring map that
//!   never raises degree, so `d_F(f) ≥ d_F(q_j)`, `q_j(z) = S₃(g·z, g·ζ^j, x_R)`
//!   on one Kummer tower. (i) For each Kummer `m = 2` closure row, `d_F(q_j)`
//!   at 4 grid points, checked against the row. (ii) A synthetic ladder
//!   `t = 2 … T` at `P1`, `K` draws per `t`, with `q` checked to have no root on
//!   `μ_K`. Writes `DIR/derived/spec_1d_rows.jsonl` and
//!   `DIR/derived/spec_1d_ladder.jsonl` (+ `.md`).
//!
//! `cargo run --release --example prime_fall_degree_tools -- score research/prime_fall_degree_20261027`
use crypto_lib::cryptanalysis::hky_fp_tower::{self, HkyLimits, HkyRing};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

fn rows(dir: &Path, prefix: &[&str]) -> Vec<Value> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir.join("runs"))
        .expect("runs/")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy().to_string();
            n.ends_with(".jsonl") && prefix.iter().any(|x| n.starts_with(x))
        })
        .collect();
    files.sort();
    let mut out = Vec::new();
    for f in files {
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        for l in fs::read_to_string(&f).unwrap().lines() {
            if l.starts_with('{') {
                let mut v: Value = serde_json::from_str(l).expect("row");
                // Run summaries (no `x_r`) are not measured rows.
                if v.get("x_r").is_none() || v.get("kind").is_none() {
                    continue;
                }
                v["_file"] = json!(name);
                out.push(v);
            }
        }
    }
    out
}

fn u(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap_or_else(|| panic!("field {k}"))
}

type Key = (u64, String, u64, u64, String, u64, u64);

fn key(r: &Value) -> Key {
    (
        u(r, "p"),
        r["kind"].as_str().unwrap().to_string(),
        u(r, "m"),
        u(r, "t"),
        r["control"].as_str().unwrap().to_string(),
        u(r, "target_index"),
        u(r, "x_r"),
    )
}

/// `d_F` of a closure row: `(value, censoring)`, censoring `""` (exact),
/// `">"` (no 1 up to cmax: `d_F > cmax`, value cmax + 1) or `">="` (stopped
/// by a limit at c: `d_F ≥ c`).
fn d_f(r: &Value) -> (u64, &'static str) {
    if let Some(d) = r["last_fall_degree"].as_u64() {
        return (d, "");
    }
    let last = r["closures"].as_array().unwrap().last().expect("a closure");
    let c = u(last, "c");
    if last["censored"].is_null() {
        (c + 1, ">")
    } else {
        (c, ">=")
    }
}

/// Lower median; censored if a censored value is at or below it.
fn lower_median(v: &[(u64, &'static str)]) -> Option<(u64, &'static str)> {
    if v.is_empty() {
        return None;
    }
    let mut s = v.to_vec();
    s.sort_by_key(|x| x.0);
    let med = s[(s.len() - 1) / 2].0;
    let cens = s.iter().any(|&(x, c)| !c.is_empty() && x <= med);
    Some((med, if cens { ">=" } else { "" }))
}

fn fmt(x: Option<(u64, &str)>) -> String {
    match x {
        None => "—".into(),
        Some((v, c)) => format!("{c}{v}"),
    }
}

/// The §6 level rule on completed (uncensored) medians in ladder order.
fn reading(meds: &[Option<(u64, &str)>]) -> &'static str {
    let vs: Vec<u64> = meds
        .iter()
        .filter_map(|m| m.and_then(|(v, c)| c.is_empty().then_some(v)))
        .collect();
    if vs.len() < 2 {
        return "insufficient";
    }
    if vs.iter().all(|&v| v == vs[0]) {
        return "flat";
    }
    if *vs.last().unwrap() >= vs[0] + 2 {
        return "grows";
    }
    "mixed"
}

fn lambda(p: u64, t: u64, m: u64) -> f64 {
    let fact: f64 = (1..=m).map(|i| i as f64).product();
    (2f64).powi((t * m) as i32) * (2f64).powi(m as i32 - 1) / (fact * p as f64)
}

struct Cell {
    p: u64,
    kind: String,
    m: u64,
    control: String,
    t: u64,
    rows: Vec<Value>,
    d_f: Vec<(u64, &'static str)>,
    ffd: Vec<(u64, &'static str)>,
    d: Vec<(u64, &'static str)>,
    /// `F_p` multiply-adds per draw: the closure (all closures computed) and
    /// F4 (`muladds`), where recorded.
    hky_ops: Vec<u64>,
    f4_ops: Vec<u64>,
    files: Vec<String>,
}

fn score(dir: &Path) {
    let hky: Vec<Value> = rows(dir, &["A-", "B-", "C-"])
        .into_iter()
        .filter(|r| r["engine"] == "hky_fp_tower")
        .collect();
    let f4: HashMap<Key, Value> = rows(dir, &["D"])
        .into_iter()
        .map(|r| (key(&r), r))
        .collect();
    // One row per system; repeats (C repeating A/B) must agree (Amendment 2).
    let mut seen: BTreeMap<Key, Value> = BTreeMap::new();
    let (mut repeats, mut mismatches) = (0, 0);
    for r in hky {
        let k = key(&r);
        if let Some(a) = seen.get(&k) {
            repeats += 1;
            let same = ["last_fall_degree", "first_fall_degree", "falls"]
                .iter()
                .all(|f| a[*f] == r[*f]);
            if !same {
                mismatches += 1;
                eprintln!("REPEAT MISMATCH {:?}", k);
            }
            continue;
        }
        seen.insert(k, r);
    }
    let mut cells: BTreeMap<(u64, String, u64, String, u64), Cell> = BTreeMap::new();
    let (mut g2_checked, mut g2_bad) = (0, 0);
    let mut planted_bad = 0;
    for (k, r) in &seen {
        if r["target"] != "random" {
            continue;
        }
        if r["planted_ok"] == json!(false) {
            planted_bad += 1;
        }
        let ck = (k.0, k.1.clone(), k.2, k.4.clone(), k.3);
        let c = cells.entry(ck).or_insert_with(|| Cell {
            p: k.0,
            kind: k.1.clone(),
            m: k.2,
            control: k.4.clone(),
            t: k.3,
            rows: Vec::new(),
            d_f: Vec::new(),
            ffd: Vec::new(),
            d: Vec::new(),
            hky_ops: Vec::new(),
            f4_ops: Vec::new(),
            files: Vec::new(),
        });
        let df = d_f(r);
        c.d_f.push(df);
        if let Some(f) = r["first_fall_degree"].as_u64() {
            c.ffd.push((f, ""));
        }
        if let Some(x) = r["muladds"].as_u64() {
            c.hky_ops.push(x);
        }
        if let Some(g) = f4.get(k) {
            if let Some(x) = g["muladds"].as_u64() {
                c.f4_ops.push(x);
            }
            let dv = u(g, "solving_degree_max");
            if g["timed_out"] == json!(true) {
                c.d.push((dv, ">="));
            } else {
                c.d.push((dv, ""));
                if df.1.is_empty() {
                    g2_checked += 1;
                    if df.0 > dv {
                        g2_bad += 1;
                        eprintln!("G2 VIOLATION d_F {} > D {} at {:?}", df.0, dv, k);
                    }
                }
            }
        }
        let f = r["_file"].as_str().unwrap().to_string();
        if !c.files.contains(&f) {
            c.files.push(f);
        }
        c.rows.push(r.clone());
    }
    let mut md = vec![
        "# Scored table (generated by `prime_fall_degree_tools score`; do not edit)".to_string(),
        String::new(),
        format!("Gate G2 (`d_F ≤ D`): {g2_checked} draws checked, {g2_bad} violations. Repeated systems: {repeats}, mismatches: {mismatches}. Planted-solution violations: {planted_bad}."),
        String::new(),
        "| p | kind | m | control | t | N | λ | d_F | ffd | D | falls | max cols | closure F_p muladds | F4 F_p muladds |".into(),
        "|--:|:--|--:|:--|--:|--:|--:|:--|:--|:--|:--|--:|:--|:--|".into(),
    ];
    let mut cells_json = Vec::new();
    for c in cells.values() {
        let small = (1u64 << c.t) < c.m;
        let falls: Vec<Value> = c.rows.iter().map(|r| r["falls"].clone()).collect();
        let max_cols = c
            .rows
            .iter()
            .flat_map(|r| {
                r["closures"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|x| u(x, "cols"))
            })
            .max()
            .unwrap_or(0);
        let ms: Vec<u64> = c
            .rows
            .iter()
            .map(|r| r["ms"].as_f64().unwrap_or(0.0) as u64)
            .collect();
        let ops = |v: &[u64]| {
            if v.is_empty() {
                "—".to_string()
            } else {
                v.iter()
                    .map(|x| format!("{:.2e}", *x as f64))
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        };
        let fl = |v: &[(u64, &str)]| {
            v.iter()
                .map(|&(x, s)| format!("{s}{x}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        md.push(format!(
            "| {} | {} | {} | {}{} | {} | {} | {:.2e} | {} | {} | {} | {} | {} | {} | {} |",
            c.p,
            c.kind,
            c.m,
            c.control,
            if small { " (small base)" } else { "" },
            c.t,
            c.m * c.t,
            lambda(c.p, c.t, c.m),
            fl(&c.d_f),
            fl(&c.ffd),
            if c.d.is_empty() {
                "—".into()
            } else {
                fl(&c.d)
            },
            serde_json::to_string(&falls).unwrap(),
            max_cols,
            ops(&c.hky_ops),
            ops(&c.f4_ops)
        ));
        cells_json.push(json!({"p": c.p, "kind": c.kind, "m": c.m, "control": c.control, "t": c.t,
            "N": c.m * c.t, "lambda": lambda(c.p, c.t, c.m), "small_base": small,
            "d_F": fl(&c.d_f), "d_F_median": fmt(lower_median(&c.d_f)), "ffd": fl(&c.ffd),
            "ffd_median": fmt(lower_median(&c.ffd)), "D": fl(&c.d),
            "D_median": if c.d.len() == c.d_f.len() { fmt(lower_median(&c.d)) } else { "—".into() },
            "max_cols": max_cols, "ms": ms, "hky_muladds": c.hky_ops, "f4_muladds": c.f4_ops, "files": c.files}));
    }
    // Ladders in t at fixed (p, kind, m, control).
    md.extend([
        String::new(),
        "## Ladder readings (§6 rules as registered)".into(),
        String::new(),
        "| p | kind | m | control | t | median d_F | median ffd | median D | H-LFD | H-FFD | d_F<D / = / > |".into(),
        "|--:|:--|--:|:--|:--|:--|:--|:--|:--|:--|:--|".into(),
    ]);
    let mut lad: BTreeMap<(u64, String, u64, String), Vec<&Cell>> = BTreeMap::new();
    for c in cells.values() {
        lad.entry((c.p, c.kind.clone(), c.m, c.control.clone()))
            .or_default()
            .push(c);
    }
    let mut ladders_json = Vec::new();
    for ((p, kind, m, control), cs) in &lad {
        if cs.len() < 2 {
            continue;
        }
        let dmed: Vec<_> = cs.iter().map(|c| lower_median(&c.d_f)).collect();
        let fmed: Vec<_> = cs.iter().map(|c| lower_median(&c.ffd)).collect();
        let d_med: Vec<_> = cs
            .iter()
            .map(|c| {
                if c.d.len() == c.d_f.len() {
                    lower_median(&c.d)
                } else {
                    None
                }
            })
            .collect();
        let gaps: Vec<i64> = d_med
            .iter()
            .zip(&fmed)
            .filter_map(|(d, f)| match (d, f) {
                (Some((dv, "")), Some((fv, _))) => Some(*dv as i64 - *fv as i64),
                _ => None,
            })
            .collect();
        let hffd = if gaps.len() < 2 {
            "no D"
        } else if gaps.last().unwrap() - gaps[0] >= 2 {
            "FFD assumption fails"
        } else if gaps.iter().all(|g| (g - gaps[0]).abs() <= 1) {
            "consistent"
        } else {
            "mixed"
        };
        let (mut lt, mut eq, mut gt) = (0, 0, 0);
        for c in cs {
            for (a, b) in c.d_f.iter().zip(&c.d) {
                if a.1.is_empty() && b.1.is_empty() {
                    match a.0.cmp(&b.0) {
                        std::cmp::Ordering::Less => lt += 1,
                        std::cmp::Ordering::Equal => eq += 1,
                        std::cmp::Ordering::Greater => gt += 1,
                    }
                }
            }
        }
        let j = |v: &[Option<(u64, &str)>]| v.iter().map(|x| fmt(*x)).collect::<Vec<_>>().join(" ");
        let hl = reading(&dmed);
        md.push(format!(
            "| {p} | {kind} | {m} | {control} | {}–{} | {} | {} | {} | {hl} | {hffd} | {lt} / {eq} / {gt} |",
            cs[0].t,
            cs.last().unwrap().t,
            j(&dmed),
            j(&fmed),
            j(&d_med)
        ));
        ladders_json.push(json!({"p": p, "kind": kind, "m": m, "control": control,
            "t": cs.iter().map(|c| c.t).collect::<Vec<_>>(), "d_F_median": j(&dmed),
            "ffd_median": j(&fmed), "D_median": j(&d_med), "H-LFD": hl, "H-FFD": hffd,
            "dF<D": lt, "dF=D": eq, "dF>D": gt}));
    }
    // H-m: ladders in m at fixed (p, kind, t) over the C cells.
    md.extend([
        String::new(),
        "## H-m: d_F against m at fixed t (cells C)".into(),
        String::new(),
        "| p | kind | t | m | median d_F | median ffd | H-m |".into(),
        "|--:|:--|--:|:--|:--|:--|:--|".into(),
    ]);
    let mut ml: BTreeMap<(u64, String, u64), Vec<&Cell>> = BTreeMap::new();
    for c in cells.values() {
        // Every tower cell at (p, kind, t): the same seed gives the same
        // system in A, B and C, and repeats are merged above.
        if c.control == "tower" && (c.t == 2 || c.t == 3) {
            ml.entry((c.p, c.kind.clone(), c.t)).or_default().push(c);
        }
    }
    let mut hm_json = Vec::new();
    for ((p, kind, t), cs) in &ml {
        let mut cs = cs.clone();
        cs.sort_by_key(|c| c.m);
        let dmed: Vec<_> = cs.iter().map(|c| lower_median(&c.d_f)).collect();
        let fmed: Vec<_> = cs.iter().map(|c| lower_median(&c.ffd)).collect();
        let rd = reading(&dmed);
        let ms: Vec<String> = cs.iter().map(|c| c.m.to_string()).collect();
        let j = |v: &[Option<(u64, &str)>]| v.iter().map(|x| fmt(*x)).collect::<Vec<_>>().join(" ");
        md.push(format!(
            "| {p} | {kind} | {t} | {} | {} | {} | {rd} |",
            ms.join(" "),
            j(&dmed),
            j(&fmed)
        ));
        hm_json.push(json!({"p": p, "kind": kind, "t": t, "m": ms, "d_F_median": j(&dmed), "ffd_median": j(&fmed), "H-m": rd}));
    }
    fs::write(dir.join("TABLE.md"), md.join("\n") + "\n").unwrap();
    fs::write(
        dir.join("score.json"),
        serde_json::to_string_pretty(
            &json!({"gate_G2": {"checked": g2_checked, "violations": g2_bad},
            "repeats": repeats, "repeat_mismatches": mismatches, "planted_violations": planted_bad,
            "cells": cells_json, "ladders": ladders_json, "H-m": hm_json}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("{}", md.join("\n"));
    if g2_bad > 0 || mismatches > 0 || planted_bad > 0 {
        std::process::exit(1);
    }
}

fn gate3(dir: &Path, pilot: &str, max_n: u64, jobs: usize, budget: u64) {
    let mut work: Vec<(Value, u64, bool)> = Vec::new();
    for r in rows(dir, &["A-", "B-", "C-"]) {
        if r["engine"] != "hky_fp_tower" || r["target"] != "random" || r["control"] == "naive" {
            continue;
        }
        if u(&r, "N") > max_n {
            continue;
        }
        let (d, cens) = d_f(&r);
        match cens {
            "" => {
                if d >= 2 {
                    work.push((r.clone(), d - 1, false));
                }
                work.push((r, d, true));
            }
            ">" => work.push((r, d - 1, false)),
            _ => {
                if d >= 2 {
                    work.push((r, d - 1, false));
                }
            }
        }
    }
    // One system per (key, cap): C cells repeat A/B systems.
    let mut uniq: BTreeMap<(Key, u64), (Value, u64, bool)> = BTreeMap::new();
    for w in work {
        uniq.entry((key(&w.0), w.1)).or_insert(w);
    }
    let work: Vec<(Value, u64, bool)> = uniq.into_values().collect();
    fs::create_dir_all(dir.join("gates")).unwrap();
    let out = Arc::new(Mutex::new(
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(dir.join("gates/g3.jsonl"))
            .unwrap(),
    ));
    let queue = Arc::new(Mutex::new(work));
    let bad = Arc::new(Mutex::new(0usize));
    let total = queue.lock().unwrap().len();
    let handles: Vec<_> = (0..jobs)
        .map(|_| {
            let (queue, out, bad) = (queue.clone(), out.clone(), bad.clone());
            let pilot = pilot.to_string();
            std::thread::spawn(move || loop {
                let Some((r, cap, want)) = queue.lock().unwrap().pop() else { break };
                let t = u(&r, "t").to_string();
                let mut cmd = Command::new(&pilot);
                cmd.args(["--engine", "tower", "--p", &u(&r, "p").to_string(), "--kinds"])
                    .arg(r["kind"].as_str().unwrap())
                    .args(["--m", &u(&r, "m").to_string(), "--controls"])
                    .arg(r["control"].as_str().unwrap())
                    .args(["--t-min", &t, "--max-t", &t, "--max-t-m3", &t, "--max-t-m4", &t])
                    .args(["--ladder-t", "none", "--planted", "0", "--random", "2"])
                    .args(["--cap", &cap.to_string(), "--targets", &u(&r, "target_index").to_string()])
                    .args(["--budget", &budget.to_string(), "--out"]);
                let tmp = std::env::temp_dir().join(format!(
                    "g3-{}-{}-{}-{}-{}.jsonl",
                    u(&r, "p"),
                    r["kind"].as_str().unwrap(),
                    u(&r, "m"),
                    u(&r, "x_r"),
                    cap
                ));
                cmd.arg(&tmp);
                if r["control"] == "null" && u(&r, "m") >= 3 {
                    cmd.arg("--null-chain");
                }
                let o = cmd.output().expect("pilot");
                assert!(o.status.success(), "pilot failed: {}", String::from_utf8_lossy(&o.stderr));
                let text = fs::read_to_string(&tmp).expect("capped rows");
                let _ = fs::remove_file(&tmp);
                let rs: Vec<Value> = text
                    .lines()
                    .filter(|l| l.starts_with('{'))
                    .map(|l| serde_json::from_str(l).unwrap())
                    .filter(|v: &Value| v.get("x_r").is_some())
                    .collect();
                assert_eq!(rs.len(), 1, "one capped row expected: {text}");
                let f = &rs[0];
                let timed_out = f["timed_out"] == json!(true);
                let refuted = f["inconsistent"] == json!(true) && !timed_out;
                let seed_match = f["x_r"] == r["x_r"];
                let agree = seed_match && !timed_out && refuted == want;
                if !agree {
                    *bad.lock().unwrap() += 1;
                    eprintln!("G3 DISAGREE {:?} cap {cap} want {want} refuted {refuted} timed_out {timed_out}", key(&r));
                }
                let line = json!({"p": r["p"], "kind": r["kind"], "m": r["m"], "t": r["t"],
                    "control": r["control"], "target_index": r["target_index"], "x_r": r["x_r"],
                    "cap": cap, "closure_contains_one": want, "f4_refuted": refuted,
                    "f4_timed_out": timed_out, "f4_solving_degree_max": f["solving_degree_max"],
                    "seed_match": seed_match, "agree": agree});
                writeln!(out.lock().unwrap(), "{line}").unwrap();
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let b = *bad.lock().unwrap();
    println!("G3: {total} capped runs, {b} disagreements");
    if b > 0 {
        std::process::exit(1);
    }
}

// ── ns-degree ────────────────────────────────────────────────────────

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

/// In-place length-K DFT `a[k] = Σ_j a[j]·w^{jk}` (radix 2).
fn ntt(a: &mut [u64], w: u64, p: u64) {
    let k = a.len();
    let bits = k.trailing_zeros();
    for i in 0..k {
        let j = if bits == 0 {
            0
        } else {
            i.reverse_bits() >> (usize::BITS - bits)
        };
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= k {
        let wl = powm(w, (k / len) as u64, p);
        for s in (0..k).step_by(len) {
            let mut x = 1u64;
            for j in 0..len / 2 {
                let (lo, hi) = (a[s + j], mulm(a[s + j + len / 2], x, p));
                a[s + j] = (lo + hi) % p;
                a[s + j + len / 2] = (lo + p - hi) % p;
                x = mulm(x, wl, p);
            }
        }
        len *= 2;
    }
}

/// 2-D inverse transform of a K×K grid function (row-major, index
/// `j1·K + j2` ↔ `(ζ^{j1}, ζ^{j2})`): the coefficients of `z1^{k1} z2^{k2}`.
fn coefficients(grid: &[u64], kk: usize, zeta: u64, p: u64) -> Vec<u64> {
    let inv_z = powm(zeta, p - 2, p);
    let mut a = grid.to_vec();
    for r in 0..kk {
        ntt(&mut a[r * kk..(r + 1) * kk], inv_z, p);
    }
    let mut col = vec![0u64; kk];
    for c in 0..kk {
        for r in 0..kk {
            col[r] = a[r * kk + c];
        }
        ntt(&mut col, inv_z, p);
        for r in 0..kk {
            a[r * kk + c] = col[r];
        }
    }
    let inv = powm((kk * kk) as u64 % p, p - 2, p);
    a.iter().map(|&x| mulm(x, inv, p)).collect()
}

fn s3(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> u64 {
    let d = (x1 + p - x2) % p;
    let t1 = mulm(mulm(mulm(d, d, p), x3, p), x3, p);
    let s = (x1 + x2) % p;
    let pr = mulm(x1, x2, p);
    let inner = (mulm(s, (pr + a) % p, p) + mulm(2, b, p)) % p;
    let t2 = mulm(p - 2, mulm(inner, x3, p), p);
    let q = (pr + p - a) % p;
    let t3 = (mulm(q, q, p) + p - mulm(mulm(4, b, p), s, p)) % p;
    (t1 + t2 + t3) % p
}

/// On a Kummer tower `R = F_p[z1, z2]/(z_i^K − 1)`, `y_{i,j} = z_i^{2^j}`,
/// and `z1^{k1} z2^{k2}` has degree `popcount(k1) + popcount(k2)`.
/// Evaluation on `μ_K²` is a ring isomorphism, so an inconsistent system's
/// reduced `S₃`, `f`, is a unit, `1 = h·f` has the one solution `h = f⁻¹`,
/// and `V_{F,c}` contains `u·f` whenever `popcount(u) ≤ c − 2`. Hence
/// `d_F ≤ NS(f) = 2 + popdeg(f⁻¹)`, the degree of the only certificate
/// without a fall. Also checks the isomorphism: `f`'s coefficients must be
/// supported in `{0, 1, 2}²`.
fn ns_degree(dir: &Path, t_max: u64) {
    let mut out = Vec::new();
    let mut md = vec![
        "| t | target | grid zeros | supp f ⊆ {0,1,2}² | deg f | popdeg f⁻¹ | NS = deg f + popdeg | 2t+2 | d_F | ffd |".to_string(),
        "|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|".into(),
    ];
    let mut seen = std::collections::BTreeSet::new();
    for r in rows(dir, &["A-", "C-"]) {
        if r["engine"] != "hky_fp_tower"
            || r["kind"] != "kummer"
            || u(&r, "m") != 2
            || r["control"] != "tower"
        {
            continue;
        }
        let t = u(&r, "t");
        if !(2..=t_max).contains(&t) || !seen.insert(key(&r)) {
            continue;
        }
        let p = u(&r, "p");
        let kk = 1usize << t;
        let (a, b) = (u(&r["curve"], "a"), u(&r["curve"], "b"));
        let (g, zeta) = (u(&r["tower"], "g"), u(&r["tower"], "zeta"));
        let xr = u(&r, "x_r");
        let xs: Vec<u64> = (0..kk)
            .map(|j| mulm(g, powm(zeta, j as u64, p), p))
            .collect();
        let mut grid = vec![0u64; kk * kk];
        for j1 in 0..kk {
            for j2 in 0..kk {
                grid[j1 * kk + j2] = s3(xs[j1], xs[j2], xr, a, b, p);
            }
        }
        let zeros = grid.iter().filter(|&&v| v == 0).count();
        let pc = |k: usize| k.count_ones() as u64;
        let cf = coefficients(&grid, kk, zeta, p);
        let mut shape_ok = true;
        let mut deg_f = 0;
        for k1 in 0..kk {
            for k2 in 0..kk {
                if cf[k1 * kk + k2] != 0 {
                    shape_ok &= k1 <= 2 && k2 <= 2;
                    deg_f = deg_f.max(pc(k1) + pc(k2));
                }
            }
        }
        let (popdeg, frac) = if zeros == 0 {
            let inv: Vec<u64> = grid.iter().map(|&v| powm(v, p - 2, p)).collect();
            let ch = coefficients(&inv, kk, zeta, p);
            let mut pd = 0;
            let mut nz = 0usize;
            for k1 in 0..kk {
                for k2 in 0..kk {
                    if ch[k1 * kk + k2] != 0 {
                        nz += 1;
                        pd = pd.max(pc(k1) + pc(k2));
                    }
                }
            }
            (Some(pd), nz as f64 / (kk * kk) as f64)
        } else {
            (None, 0.0)
        };
        let ns = popdeg.map(|x| x + deg_f);
        md.push(format!(
            "| {t} | {} | {zeros} | {shape_ok} | {deg_f} | {} | {} | {} | {} | {} |",
            r["target_index"],
            popdeg.map_or("—".into(), |x| x.to_string()),
            ns.map_or("—".into(), |x| x.to_string()),
            2 * t + 2,
            r["last_fall_degree"],
            r["first_fall_degree"]
        ));
        out.push(json!({"p": p, "t": t, "target_index": r["target_index"], "x_r": xr, "grid_zeros": zeros,
            "f_support_in_012": shape_ok, "deg_f": deg_f, "popdeg_inv": popdeg, "ns_degree": ns,
            "inv_support_fraction": frac, "d_F": r["last_fall_degree"], "ffd": r["first_fall_degree"]}));
    }
    fs::create_dir_all(dir.join("derived")).unwrap();
    let mut f = fs::File::create(dir.join("derived/ns_degree.jsonl")).unwrap();
    for v in &out {
        writeln!(f, "{v}").unwrap();
    }
    fs::write(dir.join("derived/ns_degree.md"), md.join("\n") + "\n").unwrap();
    println!("{}", md.join("\n"));
}

// ── spec-chain ───────────────────────────────────────────────────────

/// Polynomials in `n` variables as exponent vectors (raw, before the ring).
type Raw = BTreeMap<Vec<u32>, u64>;

fn raw_add(a: &Raw, b: &Raw, p: u64) -> Raw {
    let mut o = a.clone();
    for (e, c) in b {
        let v = o.entry(e.clone()).or_insert(0);
        *v = (*v + c) % p;
    }
    o.retain(|_, c| *c != 0);
    o
}
fn raw_scale(a: &Raw, k: u64, p: u64) -> Raw {
    let mut o = Raw::new();
    for (e, c) in a {
        let v = mulm(*c, k % p, p);
        if v != 0 {
            o.insert(e.clone(), v);
        }
    }
    o
}
fn raw_mul(a: &Raw, b: &Raw, p: u64) -> Raw {
    let mut o = Raw::new();
    for (e1, c1) in a {
        for (e2, c2) in b {
            let e: Vec<u32> = e1.iter().zip(e2).map(|(x, y)| x + y).collect();
            let v = o.entry(e).or_insert(0);
            *v = (*v + mulm(*c1, *c2, p)) % p;
        }
    }
    o.retain(|_, c| *c != 0);
    o
}
fn raw_const(n: usize, c: u64, p: u64) -> Raw {
    let mut o = Raw::new();
    if !c.is_multiple_of(p) {
        o.insert(vec![0; n], c % p);
    }
    o
}
fn raw_var(n: usize, i: usize) -> Raw {
    let mut e = vec![0; n];
    e[i] = 1;
    let mut o = Raw::new();
    o.insert(e, 1);
    o
}
/// `S₃(X1, X2, X3)` exactly as `pkm_tower_pilot::s3`.
fn raw_s3(x1: &Raw, x2: &Raw, x3: &Raw, a: u64, b: u64, p: u64, n: usize) -> Raw {
    let d = raw_add(x1, &raw_scale(x2, p - 1, p), p);
    let t1 = raw_mul(&raw_mul(&raw_mul(&d, &d, p), x3, p), x3, p);
    let s = raw_add(x1, x2, p);
    let pr = raw_mul(x1, x2, p);
    let inner = raw_add(
        &raw_mul(&s, &raw_add(&pr, &raw_const(n, a, p), p), p),
        &raw_const(n, mulm(2, b, p), p),
        p,
    );
    let t2 = raw_scale(&raw_mul(&inner, x3, p), p - 2, p);
    let q = raw_add(&pr, &raw_const(n, p - a % p, p), p);
    let t3 = raw_add(&raw_mul(&q, &q, p), &raw_scale(&s, p - mulm(4, b, p), p), p);
    raw_add(&raw_add(&t1, &t2, p), &t3, p)
}

/// `d_F(φF)` of the `S₃` chain with constant summands `xs` and target
/// `xr`: `(value, censoring)` as [`d_f`].
#[allow(clippy::too_many_arguments)]
fn spec_value(
    xs: &[u64],
    xr: u64,
    a: u64,
    b: u64,
    p: u64,
    cmax: u64,
    budget: u64,
) -> ((u64, &'static str), f64, u128) {
    let m = xs.len();
    let n = m - 2;
    let ring = HkyRing::new(p, Vec::new(), n);
    let c = |v: u64| raw_const(n, v, p);
    let uu = |k: usize| raw_var(n, k);
    let mut eqs = vec![raw_s3(&c(xs[0]), &c(xs[1]), &uu(0), a, b, p, n)];
    for k in 1..m - 2 {
        eqs.push(raw_s3(&uu(k - 1), &c(xs[k + 1]), &uu(k), a, b, p, n));
    }
    eqs.push(raw_s3(&uu(m - 3), &c(xs[m - 1]), &c(xr), a, b, p, n));
    let gens: Vec<_> = eqs
        .iter()
        .map(|e| ring.from_raw(&e.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>()))
        .filter(|g| !g.is_empty())
        .collect();
    let lim = HkyLimits {
        budget: std::time::Duration::from_secs(budget),
        ..HkyLimits::default()
    };
    let fd = hky_fp_tower::fall_degrees(&ring, &gens, 1, cmax as u32, &lim);
    let last = fd.closures.last().unwrap();
    let ms = fd.closures.iter().map(|c| c.ms).sum();
    let v = match fd.last_fall {
        Some(d) => (d as u64, ""),
        None if last.censored.is_some() => (last.c as u64, ">="),
        None => (last.c as u64 + 1, ">"),
    };
    (v, ms, last.cols)
}

/// `m` grid points of a Kummer base `{g·ζ^k}`, `k < 2^t`, from a seed.
fn grid_point(m: usize, t: u64, g: u64, zeta: u64, p: u64, seed: u64) -> Vec<u64> {
    let mut h = 0x9e37_79b9_7f4a_7c15u64 ^ seed;
    (0..m)
        .map(|_| {
            h ^= h >> 31;
            h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
            h ^= h >> 29;
            mulm(g, powm(zeta, h % (1u64 << t), p), p)
        })
        .collect()
}

/// The specialised chain for `m = 3 … m_max` on the curve, base and target
/// of the first Kummer `m = 2` closure row at prime `p_want`: `points` grid
/// points per `m`. A bound for the `m`-summand system on that base and
/// target, whether or not it was measured.
fn spec_synthetic(dir: &Path, cmax: u64, points: u64, m_max: u64, budget: u64) {
    let r = rows(dir, &["A-"])
        .into_iter()
        .find(|r| {
            r["engine"] == "hky_fp_tower"
                && r["kind"] == "kummer"
                && r["control"] == "tower"
                && u(r, "m") == 2
                && u(r, "t") >= 2
                && u(r, "p") == 2013265921
        })
        .expect("an A row");
    let p = u(&r, "p");
    let (a, b) = (u(&r["curve"], "a"), u(&r["curve"], "b"));
    let (g, zeta) = (u(&r["tower"], "g"), u(&r["tower"], "zeta"));
    let (t, xr) = (u(&r, "t"), u(&r, "x_r"));
    let mut md = vec![
        format!(
            "Specialised chains on the base of `{}` (p = {p}, t = {t}, x_R = {xr}).",
            r["_file"].as_str().unwrap()
        ),
        String::new(),
        "| m | free unknowns | d_F(φF) per grid point | max | cols at last c | ms |".to_string(),
        "|--:|--:|:--|:--|--:|:--|".into(),
    ];
    let mut out = Vec::new();
    for m in 3..=m_max as usize {
        let mut vals = Vec::new();
        let mut cols = 0;
        let mut mss = Vec::new();
        for pt in 0..points {
            let xs = grid_point(m, t, g, zeta, p, (pt << 32) ^ (m as u64) << 48 ^ xr);
            let (v, ms, c) = spec_value(&xs, xr, a, b, p, cmax, budget);
            vals.push(v);
            cols = cols.max(c);
            mss.push(ms as u64);
        }
        let mx = vals.iter().map(|v| v.0).max().unwrap();
        let vs: Vec<String> = vals.iter().map(|(v, c)| format!("{c}{v}")).collect();
        md.push(format!(
            "| {m} | {} | {} | {mx} | {cols} | {mss:?} |",
            m - 2,
            vs.join(" ")
        ));
        out.push(json!({"p": p, "t": t, "x_r": xr, "m": m, "spec": vs, "max": mx, "cols": cols as u64, "ms": mss}));
        eprintln!("m={m} {:?}", vs);
    }
    fs::create_dir_all(dir.join("derived")).unwrap();
    let mut f = fs::File::create(dir.join("derived/spec_chain_synthetic.jsonl")).unwrap();
    for v in &out {
        writeln!(f, "{v}").unwrap();
    }
    fs::write(
        dir.join("derived/spec_chain_synthetic.md"),
        md.join("\n") + "\n",
    )
    .unwrap();
    println!("{}", md.join("\n"));
}

fn spec_chain(dir: &Path, cmax: u64, points: u64, m_max: u64) {
    let mut out = Vec::new();
    let mut md = vec![
        "| p | kind | m | t | target | d_F (closure) | d_F(φF) at the grid points | bound = max | check |".to_string(),
        "|--:|:--|--:|--:|--:|:--|:--|:--|:--|".into(),
    ];
    let mut seen = std::collections::BTreeSet::new();
    let mut bad = 0;
    for r in rows(dir, &["A-", "B-", "C-"]) {
        if r["engine"] != "hky_fp_tower"
            || r["kind"] != "kummer"
            || r["control"] != "tower"
            || r["target"] != "random"
        {
            continue;
        }
        let m = u(&r, "m") as usize;
        if m < 3 || m as u64 > m_max || !seen.insert(key(&r)) {
            continue;
        }
        let p = u(&r, "p");
        let t = u(&r, "t");
        let (a, b) = (u(&r["curve"], "a"), u(&r["curve"], "b"));
        let (g, zeta) = (u(&r["tower"], "g"), u(&r["tower"], "zeta"));
        let xr = u(&r, "x_r");
        let mut vals = Vec::new();
        for pt in 0..points {
            let xs = grid_point(m, t, g, zeta, p, (pt << 32) ^ xr);
            vals.push(spec_value(&xs, xr, a, b, p, cmax, 600).0);
        }
        // The bound: the largest exact value; a censored value is itself a
        // lower bound (d_F(φF) ≥ c or > cmax), so it bounds too.
        let bound = vals.iter().map(|v| v.0).max().unwrap();
        let bound_cens = vals.iter().any(|v| v.0 == bound && !v.1.is_empty());
        let (dfv, dfc) = d_f(&r);
        // check: an exact closure d_F must be ≥ every exact specialised value.
        let exact_max = vals
            .iter()
            .filter(|v| v.1.is_empty())
            .map(|v| v.0)
            .max()
            .unwrap_or(0);
        let ok = !(dfc.is_empty() && dfv < exact_max);
        if !ok {
            bad += 1;
            eprintln!("SPEC VIOLATION {:?}: d_F {dfv} < {exact_max}", key(&r));
        }
        let vs: Vec<String> = vals.iter().map(|(v, c)| format!("{c}{v}")).collect();
        md.push(format!(
            "| {p} | kummer | {m} | {t} | {} | {dfc}{dfv} | {} | {}{bound} | {} |",
            r["target_index"],
            vs.join(" "),
            if bound_cens { ">=" } else { "" },
            if ok { "ok" } else { "VIOLATION" }
        ));
        out.push(json!({"p": p, "m": m, "t": t, "target_index": r["target_index"], "x_r": xr,
            "d_F": format!("{dfc}{dfv}"), "spec": vs, "bound": format!("{}{bound}", if bound_cens { ">=" } else { "" }), "ok": ok}));
    }
    fs::create_dir_all(dir.join("derived")).unwrap();
    let mut f = fs::File::create(dir.join("derived/spec_chain.jsonl")).unwrap();
    for v in &out {
        writeln!(f, "{v}").unwrap();
    }
    fs::write(dir.join("derived/spec_chain.md"), md.join("\n") + "\n").unwrap();
    println!("{}", md.join("\n"));
    if bad > 0 {
        std::process::exit(1);
    }
}

// ── spec-1d ──────────────────────────────────────────────────────────

fn kummer_rules(t: usize) -> Vec<crypto_lib::cryptanalysis::f4_fp_tower::SquareRule> {
    use crypto_lib::cryptanalysis::f4_fp_tower::SquareRule;
    (0..t)
        .map(|j| {
            let top = j + 1 == t;
            SquareRule {
                next: (!top).then_some((j + 1) as u8),
                a: 0,
                b: if top { 0 } else { 1 },
                c: 0,
                d: if top { 1 } else { 0 },
            }
        })
        .collect()
}

/// `d_F` of `q(z) = S₃(g·z, x2, x_R)` on one Kummer tower of height `t`
/// (`y_0 = z`, `y_{j+1} = y_j²`, `y_{t−1}² = 1`), with the closure dims.
#[allow(clippy::too_many_arguments)]
fn one_d(
    t: usize,
    g: u64,
    x2: u64,
    xr: u64,
    a: u64,
    b: u64,
    p: u64,
    cmax: u64,
    budget: u64,
) -> ((u64, &'static str), Option<u32>, Vec<u32>, f64, u128) {
    let ring = HkyRing::new(p, kummer_rules(t), 0);
    let n = t;
    let x1 = raw_scale(&raw_var(n, 0), g, p);
    let q = raw_s3(&x1, &raw_const(n, x2, p), &raw_const(n, xr, p), a, b, p, n);
    let gen = ring.from_raw(&q.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>());
    let lim = HkyLimits {
        budget: std::time::Duration::from_secs(budget),
        ..HkyLimits::default()
    };
    let fd = hky_fp_tower::fall_degrees(&ring, &[gen], 1, cmax as u32, &lim);
    let last = fd.closures.last().unwrap();
    let ms = fd.closures.iter().map(|c| c.ms).sum();
    let v = match fd.last_fall {
        Some(d) => (d as u64, ""),
        None if last.censored.is_some() => (last.c as u64, ">="),
        None => (last.c as u64 + 1, ">"),
    };
    (v, fd.first_fall, fd.falls.clone(), ms, last.cols)
}

fn splitmix(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn spec_1d(dir: &Path, t_max: u64, draws: u64, cmax: u64) {
    fs::create_dir_all(dir.join("derived")).unwrap();
    // (i) rows
    let mut md = vec![
        "| p | t | target | d_F (m = 2 closure) | d_F(q_j) at 4 grid points | check |".to_string(),
        "|--:|--:|--:|:--|:--|:--|".into(),
    ];
    let mut out = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut bad = 0;
    for r in rows(dir, &["A-", "C-"]) {
        if r["engine"] != "hky_fp_tower"
            || r["kind"] != "kummer"
            || u(&r, "m") != 2
            || r["control"] != "tower"
            || r["target"] != "random"
        {
            continue;
        }
        let t = u(&r, "t") as usize;
        if t < 2 || !seen.insert(key(&r)) {
            continue;
        }
        let p = u(&r, "p");
        let (a, b) = (u(&r["curve"], "a"), u(&r["curve"], "b"));
        let (g, zeta) = (u(&r["tower"], "g"), u(&r["tower"], "zeta"));
        let xr = u(&r, "x_r");
        let mut vals = Vec::new();
        for j in 0..4u64 {
            let k = (j * 0x2545_f491) % (1u64 << t);
            let x2 = mulm(g, powm(zeta, k, p), p);
            vals.push(one_d(t, g, x2, xr, a, b, p, cmax, 900).0);
        }
        let (dfv, dfc) = d_f(&r);
        let exact_max = vals
            .iter()
            .filter(|v| v.1.is_empty())
            .map(|v| v.0)
            .max()
            .unwrap_or(0);
        let ok = !(dfc.is_empty() && dfv < exact_max);
        if !ok {
            bad += 1;
        }
        let vs: Vec<String> = vals.iter().map(|(v, c)| format!("{c}{v}")).collect();
        md.push(format!(
            "| {p} | {t} | {} | {dfc}{dfv} | {} | {} |",
            r["target_index"],
            vs.join(" "),
            if ok { "ok" } else { "VIOLATION" }
        ));
        out.push(json!({"p": p, "t": t, "target_index": r["target_index"], "x_r": xr, "d_F": format!("{dfc}{dfv}"), "spec_1d": vs, "ok": ok}));
    }
    let mut f = fs::File::create(dir.join("derived/spec_1d_rows.jsonl")).unwrap();
    for v in &out {
        writeln!(f, "{v}").unwrap();
    }
    fs::write(dir.join("derived/spec_1d_rows.md"), md.join("\n") + "\n").unwrap();
    println!("{}", md.join("\n"));
    // (ii) synthetic ladder at P1
    let p: u64 = 2_013_265_921;
    let nonres = (2..p).find(|&h| powm(h, (p - 1) / 2, p) == p - 1).unwrap();
    let mut md2 = vec![
        format!("Synthetic 1-D ladder at p = {p}: q(z) = S₃(g·z, x₂, x_R) on one Kummer tower; {draws} draws per t; cmax {cmax}."),
        String::new(),
        "| t | d_F per draw | median | ffd per draw | falls (draw 0) | cols at last c | ms |".to_string(),
        "|--:|:--|:--|:--|:--|--:|:--|".into(),
    ];
    let mut f2 = fs::File::create(dir.join("derived/spec_1d_ladder.jsonl")).unwrap();
    for t in 2..=t_max as usize {
        let zeta = powm(nonres, (p - 1) >> t, p);
        let kk = 1u64 << t;
        let mut st = 0x5eed_0000u64 ^ (t as u64) << 40;
        let mut vals = Vec::new();
        let mut ffds = Vec::new();
        let mut falls0 = Vec::new();
        let mut cols = 0;
        let mut mss = Vec::new();
        let mut d = 0;
        while d < draws {
            let a = splitmix(&mut st) % p;
            let b = 1 + splitmix(&mut st) % (p - 1);
            let g = 2 + splitmix(&mut st) % (p - 2);
            let x2 = splitmix(&mut st) % p;
            let xr = splitmix(&mut st) % p;
            if (mulm(4, mulm(mulm(a, a, p), a, p), p) + mulm(27, mulm(b, b, p), p))
                .is_multiple_of(p)
            {
                continue;
            }
            // no root of q on V = g·μ_K (else the 1-D system is consistent)
            let mut w = g;
            let mut root = false;
            for _ in 0..kk {
                if s3(w, x2, xr, a, b, p) == 0 {
                    root = true;
                    break;
                }
                w = mulm(w, zeta, p);
            }
            if root {
                continue;
            }
            let (v, ffd, falls, ms, c) = one_d(t, g, x2, xr, a, b, p, cmax, 1800);
            if d == 0 {
                falls0 = falls;
            }
            vals.push(v);
            ffds.push(ffd);
            cols = cols.max(c);
            mss.push(ms as u64);
            writeln!(f2, "{}", json!({"p": p, "t": t, "draw": d, "a": a, "b": b, "g": g, "x2": x2, "x_r": xr, "zeta": zeta,
                "d_F": format!("{}{}", v.1, v.0), "ffd": ffd, "ms": ms, "cols": c as u64})).unwrap();
            d += 1;
        }
        let med = lower_median(&vals);
        let vs: Vec<String> = vals.iter().map(|(v, c)| format!("{c}{v}")).collect();
        let fs_: Vec<String> = ffds
            .iter()
            .map(|x| x.map_or("—".into(), |y| y.to_string()))
            .collect();
        md2.push(format!(
            "| {t} | {} | {} | {} | {falls0:?} | {cols} | {mss:?} |",
            vs.join(" "),
            fmt(med),
            fs_.join(" ")
        ));
        eprintln!("t={t} {:?}", vs);
        if vals.iter().all(|v| v.1 == ">=") {
            md2.push(format!(
                "| (stopped: every draw at t = {t} hit a closure limit) | | | | | | |"
            ));
            break;
        }
    }
    fs::write(dir.join("derived/spec_1d_ladder.md"), md2.join("\n") + "\n").unwrap();
    println!("{}", md2.join("\n"));
    if bad > 0 {
        std::process::exit(1);
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let opt = |name: &str, d: u64| -> u64 {
        a.iter()
            .position(|x| x == name)
            .map(|i| a[i + 1].parse().expect(name))
            .unwrap_or(d)
    };
    match a.first().map(String::as_str) {
        Some("score") => score(Path::new(&a[1])),
        Some("gate3") => gate3(
            Path::new(&a[1]),
            &a[2],
            opt("--max-n", 24),
            opt("--jobs", 2) as usize,
            opt("--budget", 900),
        ),
        Some("ns-degree") => ns_degree(Path::new(&a[1]), opt("--t-max", 12)),
        Some("spec-synthetic") => spec_synthetic(
            Path::new(&a[1]),
            opt("--cmax", 16),
            opt("--points", 3),
            opt("--m-max", 10),
            opt("--budget", 900),
        ),
        Some("spec-1d") => spec_1d(
            Path::new(&a[1]),
            opt("--t-max", 24),
            opt("--draws", 4),
            opt("--cmax", 12),
        ),
        Some("spec-chain") => spec_chain(
            Path::new(&a[1]),
            opt("--cmax", 12),
            opt("--points", 4),
            opt("--m-max", 10),
        ),
        _ => {
            eprintln!("usage: prime_fall_degree_tools score DIR | gate3 DIR PILOT [--max-n N] [--jobs J] [--budget S] | ns-degree DIR [--t-max T]");
            std::process::exit(2);
        }
    }
}
