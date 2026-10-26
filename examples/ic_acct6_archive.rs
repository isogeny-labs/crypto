//! Derived files of `research/ic_acct6_archive_20261021`, computed in Rust
//! (AGENTS.md, "Implementation language: no Python").
//!
//! ```text
//! cargo run --release --example ic_acct6_archive -- breakdown --root <research dir> \
//!     --label <text> --out <breakdown.json>
//! cargo run --release --example ic_acct6_archive -- render --before <a.json> \
//!     --after <b.json> --out BEFORE_AFTER.md
//! cargo run --release --example ic_acct6_archive -- plan --breakdown <b.json> \
//!     --repo <repo root> --out <plan.json>
//! ```
//!
//! * `breakdown` finds every `records.jsonl` below each `--root`, prices each
//!   verified non-warm-up record with `ecbench::work::price_record` (the
//!   `bitop/v1` rules compiled into this build), and writes the counts of
//!   complete, incomplete and unpriceable records by method and curve family,
//!   by reason, and by session (with the sessions' summed process wall time,
//!   warm-ups and failures included, which is what a re-run repeats).  A
//!   directory whose records do not parse is listed, not fatal.
//! * `render` writes the before/after tables of two breakdowns.
//! * `plan` checks, for every session with an incomplete record, whether the
//!   current build re-plans its frozen `spec.json` to the same spec id and the
//!   same resolved method ids, whether a complete record of each incomplete
//!   configuration exists anywhere in the archive, and whether the session's
//!   study feeds the index-calculus leaderboard (a `research/<study>` path
//!   named in `scripts/build_ic_leaderboard.py` or `docs/ic/LEADERBOARD.md`),
//!   and orders the sessions: leaderboard first, then most incomplete records.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crypto_lib::cryptanalysis::ecbench::{runner, spec, work};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct Counts {
    records: u64,
    complete: u64,
    incomplete: u64,
    errors: u64,
}

impl Counts {
    fn add(&mut self, state: State) {
        self.records += 1;
        match state {
            State::Complete => self.complete += 1,
            State::Incomplete => self.incomplete += 1,
            State::Error => self.errors += 1,
        }
    }
}

#[derive(Clone, Copy)]
enum State {
    Complete,
    Incomplete,
    Error,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct SessionRow {
    dir: String,
    session_id: String,
    spec_id: String,
    /// Every record of the session, warm-ups and failures included.
    records_all: u64,
    /// Summed `time.process_wall_ns` of every record.
    process_wall_ns_all: u64,
    counted: Counts,
    /// method id → counts.
    methods: BTreeMap<String, Counts>,
    /// reason → incomplete or unpriceable records naming it.
    reasons: BTreeMap<String, u64>,
    /// `method params-json` of each incomplete configuration.
    incomplete_configs: BTreeSet<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Breakdown {
    schema: String,
    label: String,
    roots: Vec<String>,
    session_dirs: u64,
    unreadable: Vec<(String, String)>,
    totals: Counts,
    /// `method|family` → counts.
    by_method_family: BTreeMap<String, Counts>,
    /// `unpriced:<x>`, `uncounted:<x>` or `error:<x>` → records.
    by_reason: BTreeMap<String, u64>,
    /// `method params-json` → counts, over the whole archive.
    by_config: BTreeMap<String, Counts>,
    sessions: Vec<SessionRow>,
}

fn find_record_dirs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    let mut subdirs = Vec::new();
    for e in entries.flatten() {
        let p = e.path();
        let Ok(ft) = e.file_type() else { continue };
        if ft.is_dir() {
            subdirs.push(p);
        } else if p.file_name().is_some_and(|n| n == "records.jsonl") {
            out.push(dir.to_path_buf());
        }
    }
    subdirs.sort();
    for s in subdirs {
        find_record_dirs(&s, out);
    }
}

fn rel(p: &Path, base: &Path) -> String {
    p.strip_prefix(base)
        .unwrap_or(p)
        .to_string_lossy()
        .into_owned()
}

fn config_key(r: &crypto_lib::cryptanalysis::ecbench::record::Record) -> String {
    format!(
        "{} {}",
        r.method.id,
        serde_json::to_string(&r.method.params).unwrap_or_default()
    )
}

fn breakdown(roots: &[PathBuf], label: &str, base: &Path) -> Breakdown {
    let mut b = Breakdown {
        schema: "ic-acct6.breakdown/v1".into(),
        label: label.into(),
        roots: roots.iter().map(|r| rel(r, base)).collect(),
        ..Default::default()
    };
    let mut dirs = Vec::new();
    for r in roots {
        find_record_dirs(r, &mut dirs);
    }
    b.session_dirs = dirs.len() as u64;
    for d in dirs {
        let records = match runner::read_records(&d) {
            Ok(r) => r,
            Err(e) => {
                b.unreadable.push((rel(&d, base), e));
                continue;
            }
        };
        let session: Value = std::fs::read_to_string(d.join("session.json"))
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or(Value::Null);
        let mut row = SessionRow {
            dir: rel(&d, base),
            session_id: session["session_id"].as_str().unwrap_or("").into(),
            spec_id: session["spec_id"].as_str().unwrap_or("").into(),
            ..Default::default()
        };
        for r in &records {
            row.records_all += 1;
            row.process_wall_ns_all += r.time.process_wall_ns;
            if !r.counts() {
                continue;
            }
            let (state, reasons): (State, Vec<String>) = match work::price_record(r) {
                Ok(w) if w.complete => (State::Complete, vec![]),
                Ok(w) => (
                    State::Incomplete,
                    w.unpriced
                        .iter()
                        .map(|u| format!("unpriced:{u}"))
                        .chain(w.known_uncounted.iter().map(|k| format!("uncounted:{k}")))
                        .collect(),
                ),
                Err(e) => (State::Error, vec![format!("error:{e}")]),
            };
            let family = &r.workload.curve.family;
            b.totals.add(state);
            b.by_method_family
                .entry(format!("{}|{family}", r.method.id))
                .or_default()
                .add(state);
            b.by_config.entry(config_key(r)).or_default().add(state);
            row.counted.add(state);
            row.methods
                .entry(r.method.id.clone())
                .or_default()
                .add(state);
            if !matches!(state, State::Complete) {
                row.incomplete_configs.insert(config_key(r));
            }
            for reason in reasons {
                *b.by_reason.entry(reason.clone()).or_default() += 1;
                *row.reasons.entry(reason).or_default() += 1;
            }
        }
        b.sessions.push(row);
    }
    b
}

fn read_breakdown(p: &Path) -> Breakdown {
    let t = std::fs::read_to_string(p).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
    serde_json::from_str(&t).unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

fn delta(a: u64, b: u64) -> String {
    if a == b {
        "0".into()
    } else {
        format!("{:+}", b as i64 - a as i64)
    }
}

fn render(before: &Breakdown, after: &Breakdown) -> String {
    let mut s = String::new();
    let (tb, ta) = (&before.totals, &after.totals);
    s += "# Archive breakdown in `bitop/v1`: before and after phase A\n\n";
    s += "Generated by `cargo run --release --example ic_acct6_archive -- render` from\n";
    s += "`breakdown-before.json` and `breakdown-after.json` (each written by the\n";
    s += "example's `breakdown` subcommand). Do not edit by hand.\n\n";
    s += &format!("- before: {}\n- after: {}\n", before.label, after.label);
    s += &format!(
        "- archive: every `records.jsonl` below `{}` ({} session directories, {} unreadable before, {} after); only verified non-warm-up records are counted\n\n",
        after.roots.join("`, `"),
        after.session_dirs,
        before.unreadable.len(),
        after.unreadable.len()
    );
    s += "A record is **complete** when `work.unpriced` and `work.known_uncounted` are both empty, **incomplete** when either is not (its total is a lower bound), and an **error** when it cannot be priced at all.\n\n";
    s += "## Totals\n\n| | before | after | change |\n|:--|--:|--:|--:|\n";
    for (name, a, b) in [
        ("records", tb.records, ta.records),
        ("complete", tb.complete, ta.complete),
        ("incomplete", tb.incomplete, ta.incomplete),
        ("errors", tb.errors, ta.errors),
    ] {
        s += &format!("| {name} | {a} | {b} | {} |\n", delta(a, b));
    }
    s += "\n## By method and curve family\n\n";
    s += "Rows whose counts are not all complete, before or after. `c / i / e` is complete / incomplete / errors.\n\n";
    s += "| method | family | records | before c / i / e | after c / i / e |\n|:--|:--|--:|:--|:--|\n";
    let keys: BTreeSet<&String> = before
        .by_method_family
        .keys()
        .chain(after.by_method_family.keys())
        .collect();
    let zero = Counts::default();
    for k in keys {
        let a = before.by_method_family.get(k).unwrap_or(&zero);
        let b = after.by_method_family.get(k).unwrap_or(&zero);
        if a.incomplete + a.errors + b.incomplete + b.errors == 0 {
            continue;
        }
        let (m, f) = k.split_once('|').unwrap_or((k, ""));
        s += &format!(
            "| `{m}` | {f} | {} | {} / {} / {} | {} / {} / {} |\n",
            b.records.max(a.records),
            a.complete,
            a.incomplete,
            a.errors,
            b.complete,
            b.incomplete,
            b.errors
        );
    }
    s += "\n## By reason\n\n";
    s += "Records naming each reason (a record can name several). `unpriced:` is a counter with no `bitop/v1` rule; `uncounted:` is work the method's code does with no counter (`known_uncounted`); `error:` is a record that cannot be priced.\n\n";
    s += "| reason | before | after | change |\n|:--|--:|--:|--:|\n";
    let keys: BTreeSet<&String> = before
        .by_reason
        .keys()
        .chain(after.by_reason.keys())
        .collect();
    for k in keys {
        let a = before.by_reason.get(k).copied().unwrap_or(0);
        let b = after.by_reason.get(k).copied().unwrap_or(0);
        s += &format!("| `{k}` | {a} | {b} | {} |\n", delta(a, b));
    }
    if !after.unreadable.is_empty() {
        s += "\n## Unreadable session directories (after)\n\n";
        for (d, e) in &after.unreadable {
            let e: String = e.chars().take(160).collect();
            s += &format!("- `{d}`: {e}\n");
        }
    }
    s
}

#[derive(Debug, Serialize)]
struct PlanRow {
    priority: usize,
    dir: String,
    study: String,
    session_id: String,
    spec_id: String,
    leaderboard: bool,
    incomplete: u64,
    counted: u64,
    records_all: u64,
    est_hours: f64,
    /// `ok`, or why the current build cannot re-run the frozen spec unchanged.
    rerun: String,
    /// Incomplete configurations that are complete somewhere in the archive.
    configs_seen_complete: usize,
    configs_incomplete: usize,
    /// The session's reasons, most frequent first.
    reasons: Vec<(String, u64)>,
}

fn study_of(dir: &str) -> String {
    let mut parts = dir.split('/');
    match (parts.next(), parts.next()) {
        (Some("research"), Some(s)) => format!("research/{s}"),
        (Some(a), _) => a.to_string(),
        _ => dir.to_string(),
    }
}

fn leaderboard_studies(repo: &Path) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for f in ["scripts/build_ic_leaderboard.py", "docs/ic/LEADERBOARD.md"] {
        let Ok(t) = std::fs::read_to_string(repo.join(f)) else {
            eprintln!("warning: {f} not readable");
            continue;
        };
        let mut rest = t.as_str();
        while let Some(at) = rest.find("research/") {
            let tail = &rest[at + "research/".len()..];
            let name: String = tail
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
                .collect();
            if !name.is_empty() && !name.ends_with(".md") {
                out.insert(format!("research/{}", name.trim_end_matches('.')));
            }
            rest = tail;
        }
    }
    out
}

/// Whether the current build plans the frozen spec unchanged: same spec id
/// and, for every arm, the method id the committed records carry.
fn rerun_check(dir: &Path) -> String {
    let text = match std::fs::read_to_string(dir.join("spec.json")) {
        Ok(t) => t,
        Err(e) => return format!("no frozen spec.json: {e}"),
    };
    let session: Value = std::fs::read_to_string(dir.join("session.json"))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(Value::Null);
    let s = match spec::Spec::from_json(&text) {
        Ok(s) => s,
        Err(e) => return format!("spec does not parse: {e}"),
    };
    let plan = match spec::plan(s) {
        Ok(p) => p,
        Err(e) => return format!("current build refuses the spec: {e}"),
    };
    if let Some(id) = session["spec_id"].as_str() {
        if id != plan.spec_id {
            return format!("spec id changed: {id} → {}", plan.spec_id);
        }
    }
    let records = match runner::read_records(dir) {
        Ok(r) => r,
        Err(e) => return format!("records: {e}"),
    };
    let committed: BTreeMap<&str, &str> = records
        .iter()
        .map(|r| (r.arm.as_str(), r.method.method_id.as_str()))
        .collect();
    for a in &plan.arms {
        match committed.get(a.name.as_str()) {
            Some(&id) if id == a.method.method_id => {}
            Some(&id) => {
                return format!(
                    "arm {} resolves to {} (committed {id})",
                    a.name, a.method.method_id
                )
            }
            None => {}
        }
    }
    if plan.executions.len() as u64 != records.len() as u64 {
        return format!(
            "plan has {} executions, session {} records",
            plan.executions.len(),
            records.len()
        );
    }
    for m in plan
        .arms
        .iter()
        .filter(|a| a.method.params.contains_key("backend"))
    {
        let b = &m.method.params["backend"];
        if b.starts_with('/') && !Path::new(b).exists() {
            return format!("arm {} needs native backend {b}, absent here", m.name);
        }
    }
    "ok".into()
}

fn plan(b: &Breakdown, repo: &Path, archive: &Path) -> Vec<PlanRow> {
    let lb = leaderboard_studies(repo);
    let mut rows: Vec<PlanRow> = Vec::new();
    for s in &b.sessions {
        let incomplete = s.counted.incomplete + s.counted.errors;
        if incomplete == 0 {
            continue;
        }
        let study = study_of(&s.dir);
        let seen = s
            .incomplete_configs
            .iter()
            .filter(|c| b.by_config.get(*c).is_some_and(|x| x.complete > 0))
            .count();
        let mut reasons: Vec<(String, u64)> =
            s.reasons.iter().map(|(k, v)| (k.clone(), *v)).collect();
        reasons.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        eprintln!("checking {}", s.dir);
        rows.push(PlanRow {
            priority: 0,
            dir: s.dir.clone(),
            leaderboard: lb.contains(&study),
            study,
            session_id: s.session_id.clone(),
            spec_id: s.spec_id.clone(),
            incomplete,
            counted: s.counted.records,
            records_all: s.records_all,
            est_hours: s.process_wall_ns_all as f64 / 3.6e12,
            rerun: rerun_check(&archive.join(&s.dir)),
            configs_seen_complete: seen,
            configs_incomplete: s.incomplete_configs.len(),
            reasons,
        });
    }
    rows.sort_by(|a, b| {
        b.leaderboard
            .cmp(&a.leaderboard)
            .then((a.rerun != "ok").cmp(&(b.rerun != "ok")))
            .then(b.incomplete.cmp(&a.incomplete))
            .then(a.dir.cmp(&b.dir))
    });
    for (i, r) in rows.iter_mut().enumerate() {
        r.priority = i + 1;
    }
    rows
}

/// The plan as a table, with totals by re-run status.  `done` maps a
/// session directory to the name of the re-run that replaced it.
/// One row per re-run session: the committed session it repeats, the
/// audit's verdict, and how its counted records price.
fn tranche_md(
    b: &Breakdown,
    archive: &Breakdown,
    sessions_dir: &Path,
    tranche: &[Value],
) -> String {
    let mut s = String::new();
    s += "# First tranche: re-runs on the instrumented binary\n\n";
    s += "Generated by `cargo run --release --example ic_acct6_archive -- tranche-md` from\n";
    s += "`breakdown-tranche.json`, `tranche.json` and each session's `verify.json`. Do not\n";
    s += "edit by hand.\n\n";
    s += "`superseded` is the repeated session's incomplete (or unpriceable) counted records,\n";
    s += "counted only when the re-run's audit is ok and every one of its counted records is\n";
    s += "complete. The repeated session itself is unchanged.\n\n";
    s += "| re-run | repeats | verify | counted | complete | incomplete | errors | superseded | what incomplete records miss |\n";
    s += "|:--|:--|:--|--:|--:|--:|--:|--:|:--|\n";
    let (mut c, mut k, mut superseded) = (Counts::default(), 0usize, 0u64);
    for t in tranche {
        let name = t["name"].as_str().unwrap_or("");
        let from = t["from"].as_str().unwrap_or("");
        let row = b
            .sessions
            .iter()
            .find(|r| r.dir.ends_with(&format!("/{name}")));
        let verify: Value = std::fs::read_to_string(sessions_dir.join(name).join("verify.json"))
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or(Value::Null);
        let v = if verify.is_null() {
            "no verify.json".to_string()
        } else {
            let problems = verify["problems"].as_array().map(|a| a.len()).unwrap_or(0);
            let replays = verify["replays"].as_array().map(|a| a.len()).unwrap_or(0);
            format!(
                "ok={} ({problems} problems, {replays} replays)",
                verify["ok"]
            )
        };
        let original = archive
            .sessions
            .iter()
            .find(|r| r.dir == from)
            .map(|r| r.counted.incomplete + r.counted.errors)
            .unwrap_or(0);
        match row {
            Some(r) => {
                k += 1;
                let sup = if verify["ok"] == true
                    && r.counted.records > 0
                    && r.counted.complete == r.counted.records
                {
                    original
                } else {
                    0
                };
                superseded += sup;
                c.records += r.counted.records;
                c.complete += r.counted.complete;
                c.incomplete += r.counted.incomplete;
                c.errors += r.counted.errors;
                let miss: Vec<String> = r
                    .reasons
                    .iter()
                    .map(|(k, v)| format!("`{k}` ({v})"))
                    .collect();
                s += &format!(
                    "| `{name}` | `{from}` | {v} | {} | {} | {} | {} | {sup} | {} |\n",
                    r.counted.records,
                    r.counted.complete,
                    r.counted.incomplete,
                    r.counted.errors,
                    miss.join(", ")
                );
            }
            None => s += &format!("| `{name}` | `{from}` | {v} | no records | | | | | |\n"),
        }
    }
    s += &format!(
        "\n{k} sessions: {} counted records, {} complete, {} incomplete, {} errors.\n",
        c.records, c.complete, c.incomplete, c.errors
    );
    let t = &archive.totals;
    s += &format!(
        "\n## Updated archive breakdown\n\n| | records |\n|:--|--:|\n\
         | archive counted records (`{}`) | {} |\n\
         | complete | {} |\n\
         | incomplete or unpriceable | {} |\n\
         | of which superseded by a complete, audited re-run | {superseded} |\n\
         | of which still without a complete price | {} |\n\
         | complete records added by the re-runs | {} |\n",
        archive.label,
        t.records,
        t.complete,
        t.incomplete + t.errors,
        (t.incomplete + t.errors).saturating_sub(superseded),
        c.complete
    );
    s
}

fn plan_md(rows: &[Value], done: &BTreeMap<String, String>) -> String {
    let h = |r: &Value| r["est_hours"].as_f64().unwrap_or(0.0);
    let n = |r: &Value, k: &str| r[k].as_u64().unwrap_or(0);
    let ok = |r: &Value| r["rerun"] == "ok";
    let mut s = String::new();
    s += "# Re-run plan: every session with an incomplete record\n\n";
    s += "Generated by `cargo run --release --example ic_acct6_archive -- plan-md` from\n";
    s += "`plan.json` (the example's `plan` subcommand). Do not edit by hand.\n\n";
    s += "`hours` is the summed `time.process_wall_ns` of every record of the committed\n";
    s += "session (warm-ups and failures included), on the host that ran it. `rerun` is\n";
    s += "`ok` when this build plans the frozen `spec.json` to the same spec id, the same\n";
    s += "resolved method ids and the same number of executions. `seen` is how many of\n";
    s += "the session's incomplete configurations have a complete record anywhere in the\n";
    s += "archive (a proxy for whether a re-run prices complete).\n\n";
    let total: f64 = rows.iter().map(h).sum();
    let rr: Vec<&Value> = rows.iter().filter(|r| ok(r)).collect();
    let lb: Vec<&&Value> = rr.iter().filter(|r| r["leaderboard"] == true).collect();
    let ran: Vec<&Value> = rows
        .iter()
        .filter(|r| done.contains_key(r["dir"].as_str().unwrap_or("")))
        .collect();
    s += "## Totals\n\n| | sessions | incomplete records | hours |\n|:--|--:|--:|--:|\n";
    let line = |name: &str, v: &[&Value]| {
        format!(
            "| {name} | {} | {} | {:.2} |\n",
            v.len(),
            v.iter().map(|r| n(r, "incomplete")).sum::<u64>(),
            v.iter().map(|r| h(r)).sum::<f64>()
        )
    };
    let all: Vec<&Value> = rows.iter().collect();
    s += &line("all with an incomplete record", &all);
    s += &line("re-runnable unchanged", &rr);
    s += &line(
        "  of which feed the leaderboard",
        &lb.iter().map(|r| **r).collect::<Vec<_>>(),
    );
    let not: Vec<&Value> = rows.iter().filter(|r| !ok(r)).collect();
    s += &line("not re-runnable unchanged", &not);
    s += &line("re-run in this study's first tranche", &ran);
    let rest: Vec<&Value> = rr
        .iter()
        .filter(|r| !done.contains_key(r["dir"].as_str().unwrap_or("")))
        .copied()
        .collect();
    s += &line("re-runnable, still to run", &rest);
    s += &format!("\nAll hours: {total:.2}.\n\n## Sessions, in priority order\n\n");
    s +=
        "| # | session | leaderboard | incomplete / counted | hours | seen | rerun | re-run as |\n";
    s += "|--:|:--|:--|--:|--:|--:|:--|:--|\n";
    for r in rows {
        let dir = r["dir"].as_str().unwrap_or("");
        let why: String = r["rerun"].as_str().unwrap_or("").chars().take(90).collect();
        s += &format!(
            "| {} | `{dir}` | {} | {} / {} | {:.3} | {}/{} | {} | {} |\n",
            n(r, "priority"),
            if r["leaderboard"] == true { "yes" } else { "" },
            n(r, "incomplete"),
            n(r, "counted"),
            h(r),
            n(r, "configs_seen_complete"),
            n(r, "configs_incomplete"),
            why.replace('|', "/"),
            done.get(dir).map(|d| format!("`{d}`")).unwrap_or_default()
        );
    }
    s
}

fn arg(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
}

fn args_all(args: &[String], name: &str) -> Vec<String> {
    args.windows(2)
        .filter(|w| w[0] == name)
        .map(|w| w[1].clone())
        .collect()
}

fn write(out: &str, text: &str) {
    std::fs::write(out, text).unwrap_or_else(|e| panic!("{out}: {e}"));
    eprintln!("wrote {out}");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("");
    let need = |n: &str| arg(&args, n).unwrap_or_else(|| panic!("{cmd} needs {n}"));
    match cmd {
        "breakdown" => {
            let roots: Vec<PathBuf> = args_all(&args, "--root")
                .into_iter()
                .map(PathBuf::from)
                .collect();
            assert!(!roots.is_empty(), "breakdown needs --root");
            let base = PathBuf::from(arg(&args, "--base").unwrap_or_else(|| ".".into()));
            let b = breakdown(&roots, &need("--label"), &base);
            eprintln!(
                "{} dirs, {} records: {} complete, {} incomplete, {} errors; {} unreadable",
                b.session_dirs,
                b.totals.records,
                b.totals.complete,
                b.totals.incomplete,
                b.totals.errors,
                b.unreadable.len()
            );
            write(
                &need("--out"),
                &(serde_json::to_string_pretty(&b).unwrap() + "\n"),
            );
        }
        "render" => {
            let a = read_breakdown(Path::new(&need("--before")));
            let b = read_breakdown(Path::new(&need("--after")));
            write(&need("--out"), &render(&a, &b));
        }
        "plan" => {
            let b = read_breakdown(Path::new(&need("--breakdown")));
            let repo = PathBuf::from(need("--repo"));
            let archive = PathBuf::from(arg(&args, "--base").unwrap_or_else(|| need("--repo")));
            let rows = plan(&b, &repo, &archive);
            write(
                &need("--out"),
                &(serde_json::to_string_pretty(&rows).unwrap() + "\n"),
            );
        }
        "tranche-md" => {
            let b = read_breakdown(Path::new(&need("--breakdown")));
            let t = std::fs::read_to_string(need("--tranche")).expect("tranche");
            let tranche: Value = serde_json::from_str(&t).expect("tranche json");
            let rows = tranche["sessions"].as_array().cloned().unwrap_or_default();
            write(
                &need("--out"),
                &tranche_md(
                    &b,
                    &read_breakdown(Path::new(&need("--archive"))),
                    Path::new(&need("--sessions")),
                    &rows,
                ),
            );
        }
        "plan-md" => {
            let t = std::fs::read_to_string(need("--plan")).expect("plan");
            let rows: Vec<Value> = serde_json::from_str(&t).expect("plan json");
            let done: BTreeMap<String, String> = args_all(&args, "--done")
                .iter()
                .filter_map(|d| d.split_once('='))
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            write(&need("--out"), &plan_md(&rows, &done));
        }
        _ => {
            eprintln!("usage: ic_acct6_archive breakdown|render|plan …");
            std::process::exit(2);
        }
    }
}
