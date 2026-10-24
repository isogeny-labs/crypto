//! `compare` and `null`: paired, interleaved wall-time runs of two perfbench
//! binaries, recorded row by row so that a crash keeps what was measured and
//! anyone can recompute the result.

use crate::host;
use crate::summarize::{self, group_raw, markdown, Arm, KernelInfo, Params, Summary};
use crate::util::{
    append_jsonl, read_json, read_jsonl, sha256_file, unix_time, write_json, SplitMix64,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub const TOOL_SCHEMA: u32 = 2;

/// Arguments of `compare`, and (flattened) of `null`.
#[derive(clap::Args, Clone, Debug)]
pub struct CompareArgs {
    /// Restrict to kernels whose id contains one of these (comma-separated, repeatable).
    #[arg(long)]
    pub filter: Vec<String>,
    /// Include the slow tier.
    #[arg(long)]
    pub full: bool,
    /// Area weights (default docs/perf/weights.json).
    #[arg(long)]
    pub weights: Option<PathBuf>,
    /// Run the kernels both binaries have and record the rest (the index is then not qualified).
    #[arg(long)]
    pub allow_missing: bool,
    /// Run despite differing harness sources, a dirty tree or identical binaries.
    #[arg(long)]
    pub allow_provenance_mismatch: bool,
    #[arg(long, default_value_t = 5)]
    pub rounds: usize,
    #[arg(long, default_value_t = 1)]
    pub threads: usize,
    #[arg(long, default_value_t = 5)]
    pub samples: usize,
    #[arg(long, default_value_t = 2.0)]
    pub max_seconds: f64,
    /// Per-invocation timeout, seconds.
    #[arg(long, default_value_t = 900.0)]
    pub timeout: f64,
    /// CPU list for taskset, e.g. 2.  Also makes steal accounting exact.
    #[arg(long)]
    pub taskset: Option<String>,
    /// Do not run the baseline against itself (verdicts are then unqualified).
    #[arg(long = "no-aa")]
    pub no_aa: bool,
    #[arg(long, default_value_t = 20260926)]
    pub seed: u64,
    /// Continue an interrupted run in --out.
    #[arg(long)]
    pub resume: bool,
    /// Start no new round that would end past this many seconds (round 1 always runs).
    #[arg(long)]
    pub budget_seconds: Option<f64>,
    /// Print the size of the run and stop.
    #[arg(long)]
    pub dry_run: bool,
    /// Pool contended rounds with quiet ones (reported either way).
    #[arg(long)]
    pub keep_contended: bool,
    #[arg(long, default_value_t = summarize::DEFAULT_MIN_CPU_RATIO)]
    pub min_cpu_ratio: f64,
    /// Flag a round above this many involuntary context switches per second (off by default).
    #[arg(long)]
    pub max_invol_per_s: Option<f64>,
    /// Hypervisor steal, as a fraction of an invocation, above which it is contended.
    #[arg(long, default_value_t = summarize::DEFAULT_MAX_STEAL)]
    pub max_steal: f64,
}

impl CompareArgs {
    pub fn aa(&self) -> bool {
        !self.no_aa
    }
}

// ----------------------------------------------------------------------------
// processes

/// Runs a command to completion or kills it after `timeout`.  Output is read
/// on threads so a chatty child cannot fill a pipe and stall.
pub fn run_capture(
    mut cmd: Command,
    timeout: Option<Duration>,
) -> Result<(String, String, bool), String> {
    cmd.stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null());
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("cannot run {:?}: {e}", cmd.get_program()))?;
    let mut out = child.stdout.take().unwrap();
    let mut err = child.stderr.take().unwrap();
    let t_out = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = out.read_to_string(&mut s);
        s
    });
    let t_err = std::thread::spawn(move || {
        let mut s = String::new();
        let _ = err.read_to_string(&mut s);
        s
    });
    let start = Instant::now();
    let ok = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) => {
                if timeout.is_some_and(|t| start.elapsed() > t) {
                    let _ = child.kill();
                    let _ = child.wait();
                    let stderr = t_err.join().unwrap_or_default();
                    return Err(format!(
                        "{:?} timed out after {:?}\n{}",
                        cmd.get_program(),
                        timeout.unwrap(),
                        tail(&stderr, 2000)
                    ));
                }
                std::thread::sleep(Duration::from_millis(2));
            }
            Err(e) => return Err(format!("waiting for {:?}: {e}", cmd.get_program())),
        }
    };
    Ok((
        t_out.join().unwrap_or_default(),
        t_err.join().unwrap_or_default(),
        ok,
    ))
}

pub fn tail(s: &str, n: usize) -> &str {
    let start = s.len().saturating_sub(n);
    let mut i = start;
    while !s.is_char_boundary(i) {
        i += 1;
    }
    &s[i..]
}

/// Lines of stdout that are JSON objects.
pub fn json_lines(stdout: &str) -> Vec<Value> {
    stdout
        .lines()
        .filter(|l| l.starts_with('{'))
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect()
}

pub fn run_json(cmd: Command, timeout: Option<Duration>) -> Result<Vec<Value>, String> {
    let desc = format!("{:?}", cmd);
    let (stdout, stderr, ok) = run_capture(cmd, timeout)?;
    if !ok {
        return Err(format!("{desc} failed:\n{}", tail(&stderr, 4000)));
    }
    Ok(json_lines(&stdout))
}

pub fn list_kernels(
    binary: &Path,
    full: bool,
    filters: &[String],
) -> Result<Vec<KernelInfo>, String> {
    let mut cmd = Command::new(binary);
    cmd.arg("list");
    if full {
        cmd.arg("--full");
    }
    let rows = run_json(cmd, None)?;
    let filters: Vec<&str> = filters
        .iter()
        .flat_map(|s| s.split(','))
        .filter(|s| !s.is_empty())
        .collect();
    Ok(rows
        .iter()
        .filter_map(|k| {
            Some(KernelInfo {
                id: k.get("id")?.as_str()?.to_string(),
                area: k.get("area")?.as_str()?.to_string(),
            })
        })
        .filter(|k| filters.is_empty() || filters.iter().any(|f| k.id.contains(f)))
        .collect())
}

pub fn repo_root() -> Option<PathBuf> {
    let o = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    o.status
        .success()
        .then(|| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim()))
}

pub fn load_weights(path: Option<&Path>) -> Result<BTreeMap<String, f64>, String> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => match repo_root().map(|r| r.join("docs/perf/weights.json")) {
            Some(p) if p.exists() => p,
            _ => return Ok(BTreeMap::new()),
        },
    };
    let v = read_json(&path)?;
    Ok(v.get("areas")
        .and_then(Value::as_object)
        .map(|m| {
            m.iter()
                .filter_map(|(k, x)| Some((k.clone(), x.as_f64()?)))
                .collect()
        })
        .unwrap_or_default())
}

// ----------------------------------------------------------------------------
// provenance

/// What is known about a binary: its hash always, and the build record
/// `perfindex build` wrote beside it when there is one.
pub fn build_provenance(binary: &Path) -> Value {
    let path = std::fs::canonicalize(binary).unwrap_or_else(|_| binary.to_path_buf());
    let mut prov = serde_json::Map::new();
    prov.insert("path".into(), json!(path.display().to_string()));
    prov.insert("binary_sha256".into(), json!(sha256_file(&path).ok()));
    if let Some(meta) = path
        .parent()
        .map(|p| p.join("build.json"))
        .filter(|p| p.exists())
    {
        if let Ok(b) = read_json(&meta) {
            for key in [
                "rev",
                "dirty",
                "harness_sha256",
                "cargo_lock_sha256",
                "cargo_args",
                "build_seconds",
            ] {
                if let Some(v) = b.get(key) {
                    prov.insert(key.into(), v.clone());
                }
            }
            if let Some(h) = b.get("host") {
                prov.insert(
                    "build_host_cpu".into(),
                    h.get("cpu").cloned().unwrap_or(Value::Null),
                );
                prov.insert(
                    "build_rustc".into(),
                    h.get("rustc").cloned().unwrap_or(Value::Null),
                );
            }
        }
    }
    Value::Object(prov)
}

/// Reasons the two builds cannot be compared at all.
pub fn provenance_problems(base: &Value, cand: &Value) -> Vec<String> {
    let mut bad = Vec::new();
    let s = |v: &Value, k: &str| v.get(k).and_then(Value::as_str).map(str::to_string);
    if let (Some(hb), Some(hc)) = (s(base, "harness_sha256"), s(cand, "harness_sha256")) {
        if hb != hc {
            bad.push(format!(
                "the two binaries were built from different harness sources ({} vs {}): they time different kernels",
                &hb[..12.min(hb.len())],
                &hc[..12.min(hc.len())]
            ));
        }
    }
    if let (Some(a), Some(b)) = (s(base, "binary_sha256"), s(cand, "binary_sha256")) {
        if a == b {
            bad.push(
                "base and candidate are the same binary (every speedup would be noise)".into(),
            );
        }
    }
    for (name, prov) in [("base", base), ("candidate", cand)] {
        if prov.get("dirty").and_then(Value::as_bool) == Some(true) {
            bad.push(format!(
                "the {name} build was made from a tree with uncommitted changes under src/"
            ));
        }
    }
    bad
}

/// (kernels both have, base only, candidate only)
pub fn kernel_set(
    base: &[KernelInfo],
    cand: &[KernelInfo],
) -> (Vec<KernelInfo>, Vec<String>, Vec<String>) {
    let kb: BTreeMap<&str, &KernelInfo> = base.iter().map(|k| (k.id.as_str(), k)).collect();
    let kc: BTreeMap<&str, &KernelInfo> = cand.iter().map(|k| (k.id.as_str(), k)).collect();
    let both = kc
        .iter()
        .filter(|(id, _)| kb.contains_key(*id))
        .map(|(_, k)| (*k).clone())
        .collect();
    let only_base = kb
        .keys()
        .filter(|id| !kc.contains_key(*id))
        .map(|s| s.to_string())
        .collect();
    let only_cand = kc
        .keys()
        .filter(|id| !kb.contains_key(*id))
        .map(|s| s.to_string())
        .collect();
    (both, only_base, only_cand)
}

// ----------------------------------------------------------------------------
// the schedule

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub seq: usize,
    pub round: usize,
    pub kernel: String,
    pub arm: Arm,
}

/// The exact order of every invocation, fixed by the seed: each round the
/// kernels are shuffled and, per kernel, the arms rotate one place further.
pub fn make_schedule(ids: &[String], rounds: usize, aa: bool, seed: u64) -> Vec<Item> {
    let mut rng = SplitMix64::new(seed);
    let mut items = Vec::new();
    for r in 0..rounds {
        let mut order = ids.to_vec();
        rng.shuffle(&mut order);
        for kid in order {
            let mut arms = if aa {
                vec![Arm::A, Arm::B, Arm::A2]
            } else {
                vec![Arm::A, Arm::B]
            };
            let rot = r % arms.len();
            arms.rotate_left(rot);
            for arm in arms {
                items.push(Item {
                    seq: items.len(),
                    round: r,
                    kernel: kid.clone(),
                    arm,
                });
            }
        }
    }
    items
}

pub fn schedule_hash(items: &[Item]) -> String {
    let mut text = String::new();
    for it in items {
        text.push_str(&format!("{}|{}|{}\n", it.round, it.kernel, it.arm.name()));
    }
    crate::util::sha256_hex(text.as_bytes())[..16].to_string()
}

// ----------------------------------------------------------------------------
// the plan

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Plan {
    pub tool: Value,
    pub created_unix: f64,
    pub kernels: Vec<KernelInfo>,
    pub only_base: Vec<String>,
    pub only_cand: Vec<String>,
    pub rounds: usize,
    pub aa: bool,
    pub seed: u64,
    pub threads: usize,
    pub samples: usize,
    pub max_seconds: f64,
    pub taskset: Option<String>,
    pub full: bool,
    pub filter: Vec<String>,
    pub exclude_contended: bool,
    pub min_cpu_ratio: f64,
    pub max_invol_per_s: Option<f64>,
    pub max_steal: f64,
    pub weights: BTreeMap<String, f64>,
    pub base: Value,
    pub cand: Value,
    pub schedule_hash: String,
    pub invocations: usize,
    #[serde(default)]
    pub provenance_problems: Vec<String>,
    /// Set by `reanalyze`: where this plan came from and which thresholds changed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reanalysis: Option<Value>,
}

impl Plan {
    pub fn params(&self) -> Params {
        Params {
            threads: self.threads,
            exclude_contended: self.exclude_contended,
            min_cpu_ratio: self.min_cpu_ratio,
            max_invol_per_s: self.max_invol_per_s,
            max_steal: self.max_steal,
            rounds_planned: Some(self.rounds),
            deterministic: false,
        }
    }
}

/// Recompute the summary from nothing but a plan and its raw rows.  The
/// bootstrap has its own generator, so a replay needs only the seed.
pub fn summarize_from_plan(plan: &Plan, raw: &[Value]) -> Summary {
    let per = group_raw(&plan.kernels, raw);
    let mut rng = SplitMix64::new(plan.seed ^ 0x5EED);
    summarize::summarize(&plan.kernels, &per, &plan.weights, &mut rng, &plan.params())
}

/// The hash of this program's own source, so a record says which version of
/// the tool wrote it.
pub fn tool_provenance() -> Value {
    let sources = [
        include_str!("main.rs"),
        include_str!("util.rs"),
        include_str!("stats.rs"),
        include_str!("host.rs"),
        include_str!("summarize.rs"),
        include_str!("compare.rs"),
        include_str!("instr.rs"),
        include_str!("evidence.rs"),
        include_str!("fingerprints.rs"),
        include_str!("buildcmd.rs"),
    ];
    json!({"schema": TOOL_SCHEMA, "source_sha256": crate::util::sha256_hex(sources.concat().as_bytes())})
}

// ----------------------------------------------------------------------------
// one invocation

pub fn invoke(binary: &Path, kid: &str, a: &CompareArgs) -> Result<Value, String> {
    let mut cmd;
    let cpus = a.taskset.as_deref().and_then(host::parse_cpu_list);
    if let Some(list) = &a.taskset {
        cmd = Command::new("taskset");
        cmd.args(["-c", list]).arg(binary);
    } else {
        cmd = Command::new(binary);
    }
    cmd.args([
        "run",
        "--filter",
        kid,
        "--exact",
        "--samples",
        &a.samples.to_string(),
        "--max-seconds",
        &a.max_seconds.to_string(),
    ]);
    cmd.env("RAYON_NUM_THREADS", a.threads.to_string());
    let before = host::cpu_ticks(cpus.as_ref());
    let t0 = Instant::now();
    let rows = run_json(cmd, Some(Duration::from_secs_f64(a.timeout)))?;
    let secs = t0.elapsed().as_secs_f64();
    let after = host::cpu_ticks(cpus.as_ref());
    let [mut row] = <[Value; 1]>::try_from(rows)
        .map_err(|r| format!("{kid}: expected one row, got {}", r.len()))?;
    let o = row.as_object_mut().ok_or("row is not an object")?;
    o.insert("invocation_s".into(), json!((secs * 1e4).round() / 1e4));
    if let (Some(b), Some(a)) = (before, after) {
        o.insert("steal_ticks".into(), json!(a.steal.saturating_sub(b.steal)));
        o.insert("host_ticks".into(), json!(a.total.saturating_sub(b.total)));
        o.insert("clk_tck".into(), json!(host::clk_tck()));
    }
    Ok(row)
}

// ----------------------------------------------------------------------------
// cost

/// What the measurement itself cost.
pub fn cost_ledger(raw: &[Value], kernels: &[KernelInfo], wall_total: Option<f64>) -> Value {
    let area_of: BTreeMap<&str, &str> = kernels
        .iter()
        .map(|k| (k.id.as_str(), k.area.as_str()))
        .collect();
    let (mut by_kernel, mut by_area, mut by_round): (
        BTreeMap<String, f64>,
        BTreeMap<String, f64>,
        BTreeMap<usize, f64>,
    ) = Default::default();
    let (mut sample_cpu, mut sample_wall, mut peak, mut n) = (0.0f64, 0.0f64, 0u64, 0usize);
    for r in raw
        .iter()
        .filter(|r| r.get("type").and_then(Value::as_str) == Some("invocation"))
    {
        n += 1;
        let row = &r["row"];
        let kid = r["kernel"].as_str().unwrap_or("?");
        let s = row
            .get("invocation_s")
            .and_then(Value::as_f64)
            .unwrap_or(0.0);
        *by_kernel.entry(kid.to_string()).or_default() += s;
        *by_area
            .entry(area_of.get(kid).copied().unwrap_or("?").to_string())
            .or_default() += s;
        *by_round
            .entry(r["round"].as_u64().unwrap_or(0) as usize)
            .or_default() += s;
        let sum = |key: &str| {
            row.get(key)
                .and_then(Value::as_array)
                .map_or(0.0, |a| a.iter().filter_map(Value::as_f64).sum::<f64>())
        };
        sample_wall += sum("samples_ns") / 1e9;
        sample_cpu += sum("cpu_ns") / 1e9;
        peak = peak.max(row.get("maxrss_kb").and_then(Value::as_u64).unwrap_or(0));
    }
    let total: f64 = by_kernel.values().sum();
    let mut top: Vec<(&String, &f64)> = by_kernel.iter().collect();
    top.sort_by(|a, b| b.1.total_cmp(a.1).then(a.0.cmp(b.0)));
    let r2 = |x: f64| (x * 100.0).round() / 100.0;
    json!({
        "invocations": n,
        "invocation_seconds": r2(total),
        "wall_seconds": wall_total.map(r2),
        "timed_sample_wall_seconds": (sample_wall * 1000.0).round() / 1000.0,
        "timed_sample_cpu_seconds": (sample_cpu * 1000.0).round() / 1000.0,
        "overhead_fraction": (total > 0.0).then(|| ((1.0 - sample_wall / total) * 1e4).round() / 1e4),
        "peak_rss_kb": (peak > 0).then_some(peak),
        "by_area_seconds": by_area.iter().map(|(k, v)| (k.clone(), json!(r2(*v)))).collect::<serde_json::Map<_, _>>(),
        "by_round_seconds": by_round.iter().map(|(k, v)| (k.to_string(), json!(r2(*v)))).collect::<serde_json::Map<_, _>>(),
        "costliest_kernels": top.iter().take(10).map(|(k, v)| json!({"id": k, "seconds": r2(**v)})).collect::<Vec<_>>(),
    })
}

// ----------------------------------------------------------------------------
// compare

pub fn make_plan(
    a: &CompareArgs,
    kernels: &[KernelInfo],
    only_base: Vec<String>,
    only_cand: Vec<String>,
    base: Value,
    cand: Value,
    weights: BTreeMap<String, f64>,
    problems: Vec<String>,
) -> Plan {
    let sched = make_schedule(
        &kernels.iter().map(|k| k.id.clone()).collect::<Vec<_>>(),
        a.rounds,
        a.aa(),
        a.seed,
    );
    Plan {
        tool: tool_provenance(),
        created_unix: unix_time(),
        kernels: kernels.to_vec(),
        only_base,
        only_cand,
        rounds: a.rounds,
        aa: a.aa(),
        seed: a.seed,
        threads: a.threads,
        samples: a.samples,
        max_seconds: a.max_seconds,
        taskset: a.taskset.clone(),
        full: a.full,
        filter: a.filter.clone(),
        exclude_contended: !a.keep_contended,
        min_cpu_ratio: a.min_cpu_ratio,
        max_invol_per_s: a.max_invol_per_s,
        max_steal: a.max_steal,
        weights,
        base,
        cand,
        schedule_hash: schedule_hash(&sched),
        invocations: sched.len(),
        provenance_problems: problems,
        reanalysis: None,
    }
}

/// Runs `compare`.  Returns the process exit code: 0 ok, 1 invalid result,
/// 2 refused.
pub fn run_compare(base: &Path, cand: &Path, out: &Path, a: &CompareArgs) -> Result<i32, String> {
    let (base_prov, cand_prov) = (build_provenance(base), build_provenance(cand));
    let bad = provenance_problems(&base_prov, &cand_prov);
    if !bad.is_empty() && !a.allow_provenance_mismatch {
        for b in &bad {
            eprintln!("refused: {b}");
        }
        eprintln!("(--allow-provenance-mismatch records the problem and runs anyway)");
        return Ok(2);
    }
    let (kernels, only_base, only_cand) = kernel_set(
        &list_kernels(base, a.full, &a.filter)?,
        &list_kernels(cand, a.full, &a.filter)?,
    );
    if (!only_base.is_empty() || !only_cand.is_empty()) && !a.allow_missing {
        // A candidate that drops its slowest kernel would raise the index.
        eprintln!(
            "refused: kernels on one side only. base only: {only_base:?}; candidate only: {only_cand:?}. \
             (--allow-missing runs the common set and records the rest)"
        );
        return Ok(2);
    }
    if kernels.is_empty() {
        eprintln!("refused: no kernel in common");
        return Ok(2);
    }
    let weights = load_weights(a.weights.as_deref())?;
    let mut plan = make_plan(
        a,
        &kernels,
        only_base,
        only_cand,
        base_prov.clone(),
        cand_prov.clone(),
        weights,
        bad,
    );
    let ids: Vec<String> = kernels.iter().map(|k| k.id.clone()).collect();
    let sched = make_schedule(&ids, a.rounds, a.aa(), a.seed);
    if a.dry_run {
        println!(
            "{} kernels x {} rounds x {} arms = {} invocations; each costs process start + setup + warm-up + up to {} samples \
             (<= {}s of sampling). Schedule {}.",
            kernels.len(),
            a.rounds,
            if a.aa() { 3 } else { 2 },
            sched.len(),
            a.samples,
            a.max_seconds,
            plan.schedule_hash
        );
        return Ok(0);
    }
    let mut raw: Vec<Value> = Vec::new();
    let mut done: BTreeSet<(usize, String, String)> = BTreeSet::new();
    if out.exists() {
        if !a.resume {
            eprintln!(
                "refused: {} exists (use --resume to continue it)",
                out.display()
            );
            return Ok(2);
        }
        let old: Plan = serde_json::from_value(read_json(&out.join("plan.json"))?)
            .map_err(|e| e.to_string())?;
        let same_bin = |o: &Value, n: &Value| o.get("binary_sha256") == n.get("binary_sha256");
        if old.schedule_hash != plan.schedule_hash
            || !same_bin(&old.base, &base_prov)
            || !same_bin(&old.cand, &cand_prov)
        {
            eprintln!("refused: --resume with a different schedule or different binaries");
            return Ok(2);
        }
        raw = read_jsonl(&out.join("raw.jsonl"))?;
        for r in &raw {
            if r.get("type").and_then(Value::as_str) == Some("invocation") {
                done.insert((
                    r["round"].as_u64().unwrap_or(0) as usize,
                    r["kernel"].as_str().unwrap_or("").to_string(),
                    r["arm"].as_str().unwrap_or("").to_string(),
                ));
            }
        }
        plan = old;
    } else {
        std::fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
        write_json(
            &out.join("plan.json"),
            &serde_json::to_value(&plan).map_err(|e| e.to_string())?,
        )?;
    }
    let t_start = Instant::now();
    let mut stopped_by_budget = false;
    let mut round_times: Vec<f64> = Vec::new();
    let mut fh = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out.join("raw.jsonl"))
        .map_err(|e| e.to_string())?;
    for r in 0..a.rounds {
        let todo: Vec<&Item> = sched
            .iter()
            .filter(|it| {
                it.round == r
                    && !done.contains(&(it.round, it.kernel.clone(), it.arm.name().to_string()))
            })
            .collect();
        if todo.is_empty() {
            continue;
        }
        if let (Some(budget), true) = (a.budget_seconds, r > 0) {
            let spent = t_start.elapsed().as_secs_f64();
            let est = if round_times.is_empty() {
                0.0
            } else {
                round_times.iter().sum::<f64>() / round_times.len() as f64
            };
            if spent + est > budget {
                stopped_by_budget = true;
                eprintln!(
                    "budget: {spent:.0}s spent, a round costs ~{est:.0}s: stopping before round {}",
                    r + 1
                );
                break;
            }
        }
        append_jsonl(
            &mut fh,
            &json!({"type": "round", "round": r, "conditions": host::conditions()}),
        )
        .map_err(|e| e.to_string())?;
        let t_round = Instant::now();
        for it in todo {
            let binary = if it.arm == Arm::B { cand } else { base };
            let row = invoke(binary, &it.kernel, a)?;
            let probs = summarize::row_problems(&row, &it.kernel, Some(a.threads));
            let mut rec = json!({"type": "invocation", "seq": it.seq, "round": it.round, "kernel": it.kernel, "arm": it.arm.name(), "row": row});
            if !probs.is_empty() {
                rec["row_problems"] = json!(probs);
            }
            append_jsonl(&mut fh, &rec).map_err(|e| e.to_string())?;
            eprintln!(
                "round {}/{} {} {}={:.3}ms{}",
                r + 1,
                a.rounds,
                it.kernel,
                it.arm.name(),
                rec["row"]["median_ns"].as_f64().unwrap_or(f64::NAN) / 1e6,
                if probs.is_empty() {
                    String::new()
                } else {
                    format!("  !{}", probs.join("; "))
                }
            );
            raw.push(rec);
        }
        let secs = t_round.elapsed().as_secs_f64();
        round_times.push(secs);
        append_jsonl(&mut fh, &json!({"type": "round_end", "round": r, "conditions": host::conditions(), "seconds": secs}))
            .map_err(|e| e.to_string())?;
    }
    let mut summary = summarize_from_plan(&plan, &raw);
    let bad_rows = raw
        .iter()
        .filter(|r| r.get("row_problems").is_some())
        .count();
    if bad_rows > 0 {
        summary.valid = false;
        summary.performance_index = None;
        summary.performance_index_ci95 = None;
        summary.qualified = false;
        summary.warnings.push(format!("{bad_rows} row(s) failed validation (see raw.jsonl row_problems): the comparison is refused"));
    }
    if stopped_by_budget {
        summary.warnings.push(format!(
            "stopped by --budget-seconds after {} of {} planned rounds",
            summary.rounds, a.rounds
        ));
        summary.qualified = false;
    }
    if !plan.provenance_problems.is_empty() {
        summary.warnings.push(format!(
            "provenance: {}",
            plan.provenance_problems.join("; ")
        ));
        summary.qualified = false;
    }
    if !plan.only_base.is_empty() || !plan.only_cand.is_empty() {
        summary.warnings.push(format!(
            "kernels left out of the index: base only {:?}, candidate only {:?}",
            plan.only_base, plan.only_cand
        ));
        summary.qualified = false;
    }
    let mut host = host::host_manifest(true);
    host["loadavg_end"] = host::conditions()["loadavg"].clone();
    let wall = t_start.elapsed().as_secs_f64();
    let cost = cost_ledger(&raw, &kernels, Some(wall));
    for (k, v) in [
        ("schema", json!(TOOL_SCHEMA)),
        ("mode", json!("wall")),
        ("unit", json!("ms")),
        ("base", json!(base.display().to_string())),
        ("cand", json!(cand.display().to_string())),
        ("threads", json!(plan.threads)),
        ("samples_per_invocation", json!(plan.samples)),
        ("schedule_hash", json!(plan.schedule_hash)),
        ("stopped_by_budget", json!(stopped_by_budget)),
        ("host", host),
        ("cost", cost.clone()),
    ] {
        summary.extra.insert(k.into(), v);
    }
    write_json(&out.join("cost.json"), &cost)?;
    write_json(
        &out.join("compare.json"),
        &serde_json::to_value(&summary).map_err(|e| e.to_string())?,
    )?;
    let md = markdown(&summary, "perfbench wall-time comparison");
    std::fs::write(out.join("compare.md"), &md).map_err(|e| e.to_string())?;
    print!("{md}");
    eprintln!(
        "cost: {} invocations, {}s in them ({}s of it timed samples), peak RSS {} kB",
        cost["invocations"],
        cost["invocation_seconds"],
        cost["timed_sample_wall_seconds"],
        cost["peak_rss_kb"]
    );
    Ok(if summary.valid { 0 } else { 1 })
}

/// `null`: run `compare` with the candidate a copy of the baseline that
/// differs by one trailing byte (so the hashes differ and the provenance gate
/// lets it through).  Nothing differs between the arms, so every kernel the
/// verdict rule calls faster or slower is a false positive; the count
/// measures the rule on this host, at this round count, under this load.
pub fn run_null(bin: &Path, out: &Path, a: &CompareArgs) -> Result<i32, String> {
    if a.dry_run {
        let ks = list_kernels(bin, a.full, &a.filter)?;
        let arms = if a.aa() { 3 } else { 2 };
        println!(
            "{} kernels x {} rounds x {} arms = {} invocations",
            ks.len(),
            a.rounds,
            arms,
            ks.len() * a.rounds * arms
        );
        return Ok(0); // nothing is created, not even the output directory
    }
    std::fs::create_dir(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let copy_dir = out.join("copy");
    std::fs::create_dir(&copy_dir).map_err(|e| e.to_string())?;
    let twin = copy_dir.join(bin.file_name().ok_or("binary has no file name")?);
    std::fs::copy(bin, &twin).map_err(|e| e.to_string())?;
    {
        use std::io::Write;
        let mut f = OpenOptions::new()
            .append(true)
            .open(&twin)
            .map_err(|e| e.to_string())?;
        f.write_all(b"\n").map_err(|e| e.to_string())?; // bytes after the last ELF segment are never loaded
    }
    if let Some(meta) = bin
        .parent()
        .map(|p| p.join("build.json"))
        .filter(|p| p.exists())
    {
        std::fs::copy(meta, copy_dir.join("build.json")).map_err(|e| e.to_string())?;
    }
    let cmp_dir = out.join("compare");
    let rc = run_compare(bin, &twin, &cmp_dir, a)?;
    if (rc != 0 && rc != 1) || a.dry_run {
        return Ok(rc);
    }
    let s: Summary = serde_json::from_value(read_json(&cmp_dir.join("compare.json"))?)
        .map_err(|e| e.to_string())?;
    let n = s.kernels.len();
    let false_pos = s
        .kernels
        .iter()
        .filter(|k| k.verdict == "faster" || k.verdict == "slower")
        .count();
    let unqual = s
        .kernels
        .iter()
        .filter(|k| k.verdict.starts_with("unqualified-"))
        .count();
    let uncorr = s
        .kernels
        .iter()
        .filter(|k| (k.verdict == "faster" || k.verdict == "slower") && !k.corroborated)
        .count();
    let rate = |x: usize| (n > 0).then(|| x as f64 / n as f64);
    let report = json!({
        "schema": TOOL_SCHEMA,
        "kernels": n,
        "rounds": s.rounds,
        "false_verdicts": false_pos,
        "false_verdicts_uncorroborated": uncorr,
        "unqualified_verdicts": unqual,
        "false_rate": rate(false_pos),
        "false_rate_corroborated": rate(false_pos - uncorr),
        "performance_index": s.performance_index,
        "performance_index_ci95": s.performance_index_ci95,
        "aa_noise_median_pct": s.aa_noise_median_pct,
        "false_kernels": s.kernels.iter().filter(|k| k.verdict == "faster" || k.verdict == "slower").map(|k| json!({
            "id": k.id, "verdict": k.verdict, "speedup": k.speedup, "best_of": k.speedup_min,
            "sign_test_p": k.sign_test_p, "corroborated": k.corroborated})).collect::<Vec<_>>(),
        "contended_kernels": s.kernels.iter().filter(|k| !k.rounds_contended.is_empty()).count(),
        "host": s.extra.get("host"),
    });
    write_json(&out.join("null.json"), &report)?;
    println!(
        "null comparison: {false_pos} of {n} kernels got a faster/slower verdict between identical binaries ({:.1}%; {uncorr} of them \
         uncorroborated by best-of) after {} rounds; PI {} (a perfect run is 1.000x)",
        100.0 * rate(false_pos).unwrap_or(0.0),
        s.rounds,
        s.performance_index.map_or("n/a".to_string(), |p| format!("{p:.3}x"))
    );
    Ok(0)
}
