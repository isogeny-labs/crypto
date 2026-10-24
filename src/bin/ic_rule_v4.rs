//! `ic_rule_v4`: the harness of rule v4, the paired comparison of two
//! recipes of `ic price --single-target` on the same targets
//! (`research/ic_tool_program/rule/v4/PROTOCOL.md`).
//!
//! Native, as AGENTS.md asks of research harnesses.  It changes no code it
//! measures: it launches the programme's `ic` and `isolated_bench`
//! binaries, and reads what they wrote.
//!
//! - `run`: each size's targets, two (or three) arms a target, the arms'
//!   order alternating from target to target, every process through
//!   `isolated_bench run --wait --cpus 2` after PSI has fallen.  A
//!   contended or failed process is kept and run again, at most twice.  An
//!   existing output is never overwritten, so a stopped run resumes.
//! - `train`: the recipe's tuning grid on training seeds, no isolation
//!   tool (the caller holds the heavy-compute lock and pins the CPU).
//! - `analyse`: the paired read-out, from the run tree only.
//! - `replay`: every recorded certificate replayed in `icprog`'s oracle
//!   (its own field arithmetic, not `ic`'s), and a sample re-run untimed.
//! - `reference`: the strong-walk fixture against the comparison's rho.

#[path = "icprog/json.rs"]
#[allow(dead_code)]
mod json;
#[path = "icprog/oracle.rs"]
#[allow(dead_code)]
mod oracle;

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use clap::{Parser, Subcommand};
use serde_json::{json, Value};

/// The six sizes in the order of `r` (ledger §23's).
const SIZES: [(u8, u32); 6] = [(1, 47), (0, 57), (0, 41), (0, 53), (1, 59), (0, 61)];
const TARGET_SEED_BASE: i64 = 23000;
const RHO_SEED_BASE: i64 = 2_293_760;
const CPUS: &str = "2";
const PSI_READY: f64 = 4.0;
const PSI_WAIT_MAX: Duration = Duration::from_secs(120);
const REFUSAL_WAIT: Duration = Duration::from_secs(15);
const REFUSAL_LIMIT: usize = 20;
const RETRIES: usize = 4;
/// Evaluation recipe seeds: pair `i` uses seed `201 + (i - 1) mod EVAL_SEEDS`,
/// so 201 (rule v3's) is one of 16 held-out seeds and each seed has two
/// targets in a panel of 32.
const EVAL_SEEDS: u32 = 16;
const BOOT: usize = 10_000;
const BOOT_SEED: u64 = 0x5eed_0004_0000_0001;

#[derive(Parser)]
#[command(
    name = "ic_rule_v4",
    about = "Rule v4: v3 recipe against v4 recipe, paired"
)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// The timed panel.
    Run {
        #[arg(long)]
        ic: PathBuf,
        #[arg(long)]
        isolate: PathBuf,
        /// The rule/v4 directory.
        #[arg(long)]
        root: PathBuf,
        /// The rule/v3 directory (parameter templates, earlier logarithms).
        #[arg(long)]
        v3: PathBuf,
        #[arg(long)]
        recipe: PathBuf,
        /// Targets a size.
        #[arg(long)]
        targets: u32,
        /// Targets (the first ones) that also run the v3 recipe a second time.
        #[arg(long, default_value_t = 0)]
        aa: u32,
        /// Targets (the first ones) that also run the old columns control (the
        /// v3 recipe with 0.6 times the columns; amendment A6 retired it).
        #[arg(long, default_value_t = 0)]
        pos: u32,
        /// Targets (the first ones) that also run the lever control: the v3
        /// recipe with `KIC_FREE_RELATIONS=1`, whose cost the training
        /// pairs put at 1.05 to 1.09 times the v3 recipe's.
        #[arg(long, default_value_t = 0)]
        lev: u32,
        /// Stop cleanly after the process that ends past this many minutes.
        #[arg(long, default_value_t = 25.0)]
        max_minutes: f64,
        /// Sizes by archive key (the `curve` field of analysis.json),
        /// comma separated; default all six.
        #[arg(long)]
        sizes: Option<String>,
        /// `isolated_bench --max-other-cpu`: the CPUs other processes may use
        /// on average (the tool's default, 0.1, refuses and flags anything more;
        /// amendment A8 raised it while neighbours ran unlocked jobs).
        #[arg(long, default_value_t = 0.1)]
        max_other_cpu: f64,
        /// Run directory name under root (default `runs`).
        #[arg(long, default_value = "runs")]
        runs: String,
    },
    /// The tuning grid on training seeds.
    Train {
        #[arg(long)]
        ic: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        v3: PathBuf,
        /// The frozen grid (`training/grid.json`).
        #[arg(long)]
        grid: PathBuf,
        #[arg(long, default_value_t = 25.0)]
        max_minutes: f64,
        #[arg(long)]
        sizes: Option<String>,
        /// Run the confirmation stage: each size's stage-1 candidate and the
        /// v3 configuration on the grid's `confirm_seeds`.
        #[arg(long)]
        confirm: bool,
    },
    /// The frozen selection rule, applied to the training runs.
    Select {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        v3: PathBuf,
        #[arg(long)]
        grid: PathBuf,
        /// The final stage: confirm the candidates on fresh training seeds
        /// and write `recipe.json` (without it, stage 1 writes the candidates).
        #[arg(long = "final")]
        final_stage: bool,
        /// Amendment A2: the model-based retune (lever off) from the grid's
        /// training runs; writes `recipe.json`.
        #[arg(long)]
        model: bool,
    },
    /// The lever's paired effect on the training runs (same configuration
    /// and seed, lever off and on): `training/lever.json`.
    Lever {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        v3: PathBuf,
        #[arg(long)]
        grid: PathBuf,
    },
    /// The README's tables, as markdown, from `analysis.json` only.
    Table {
        #[arg(long)]
        root: PathBuf,
    },
    /// The paired read-out.
    Analyse {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        v3: PathBuf,
        #[arg(long, default_value = "runs")]
        runs: String,
    },
    /// Independent replay of every recorded certificate.
    Replay {
        #[arg(long)]
        root: PathBuf,
        #[arg(long)]
        v3: PathBuf,
        #[arg(long, default_value = "runs")]
        runs: String,
        /// Also re-run (untimed) this many targets a size with both arms.
        #[arg(long, default_value_t = 0)]
        rerun: u32,
        #[arg(long)]
        ic: Option<PathBuf>,
        #[arg(long)]
        recipe: Option<PathBuf>,
    },
    /// The strong-walk fixture on T01..T08 of each size.
    Reference {
        #[arg(long)]
        fixture: PathBuf,
        #[arg(long)]
        isolate: PathBuf,
        #[arg(long)]
        root: PathBuf,
        #[arg(long, default_value_t = 8)]
        targets: u32,
        #[arg(long, default_value_t = 25.0)]
        max_minutes: f64,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Cmd::Run {
            ic,
            isolate,
            root,
            v3,
            recipe,
            targets,
            aa,
            pos,
            lev,
            max_minutes,
            sizes,
            max_other_cpu,
            runs,
        } => run(
            &ic,
            &isolate,
            &root,
            &v3,
            &recipe,
            targets,
            aa,
            pos,
            lev,
            max_minutes,
            sizes,
            max_other_cpu,
            &runs,
        ),
        Cmd::Train {
            ic,
            root,
            v3,
            grid,
            max_minutes,
            sizes,
            confirm,
        } => train(&ic, &root, &v3, &grid, max_minutes, sizes, confirm),
        Cmd::Select {
            root,
            v3,
            grid,
            final_stage,
            model,
        } => {
            if model {
                select_model(&root, &v3, &grid)
            } else {
                select(&root, &v3, &grid, final_stage)
            }
        }
        Cmd::Lever { root, v3, grid } => lever(&root, &v3, &grid),
        Cmd::Table { root } => table(&root),
        Cmd::Analyse { root, v3, runs } => analyse(&root, &v3, &runs),
        Cmd::Replay {
            root,
            v3,
            runs,
            rerun,
            ic,
            recipe,
        } => replay(&root, &v3, &runs, rerun, ic.as_deref(), recipe.as_deref()),
        Cmd::Reference {
            fixture,
            isolate,
            root,
            targets,
            max_minutes,
        } => reference(&fixture, &isolate, &root, targets, max_minutes),
    };
    match result {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("ic_rule_v4: {e}");
            std::process::exit(1);
        }
    }
}

// ── helpers ─────────────────────────────────────────────────────────

fn eval_seed(i: u32) -> i64 {
    201 + i64::from((i - 1) % EVAL_SEEDS)
}

fn slug(a: u8, n: u32) -> String {
    format!("k{a}n{n}")
}

fn read_json(path: &Path) -> Result<Value, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))
}

fn write_json(path: &Path, v: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    let text = serde_json::to_string_pretty(v).map_err(|e| e.to_string())?;
    fs::write(path, text + "\n").map_err(|e| format!("{}: {e}", path.display()))
}

fn append(path: &Path, line: &str) {
    if let Ok(mut f) = fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(line.as_bytes());
    }
}

fn utc_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    format!("unix:{secs}")
}

fn psi_some_avg10() -> f64 {
    let mut worst = 0.0f64;
    for kind in ["cpu", "memory"] {
        let Ok(text) = fs::read_to_string(format!("/proc/pressure/{kind}")) else {
            continue;
        };
        for line in text.lines().filter(|l| l.starts_with("some")) {
            if let Some(v) = line
                .split_once("avg10=")
                .and_then(|(_, rest)| rest.split_whitespace().next())
                .and_then(|v| v.parse::<f64>().ok())
            {
                worst = worst.max(v);
            }
        }
    }
    worst
}

fn wait_for_quiet() -> f64 {
    let start = Instant::now();
    while psi_some_avg10() >= PSI_READY && start.elapsed() < PSI_WAIT_MAX {
        std::thread::sleep(Duration::from_secs(1));
    }
    start.elapsed().as_secs_f64()
}

fn f(v: &Value, path: &[&str]) -> Option<f64> {
    let mut cur = v;
    for k in path {
        cur = cur.get(*k)?;
    }
    cur.as_f64()
}

fn size_filter(sizes: &Option<String>) -> Vec<(u8, u32)> {
    match sizes {
        None => SIZES.to_vec(),
        Some(list) => SIZES
            .iter()
            .copied()
            .filter(|&(a, n)| list.split(',').any(|s| s == slug(a, n)))
            .collect(),
    }
}

/// splitmix64: a small deterministic generator for the bootstrap.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// The ratio of sums `Σx / Σy` over a resample of the pairs, 95% percentile
/// interval.  Fixed seed, so the read-out is reproducible.
#[allow(dead_code)]
fn boot_ratio(x: &[f64], y: &[f64], salt: u64) -> (f64, f64, f64) {
    let n = x.len();
    let point = x.iter().sum::<f64>() / y.iter().sum::<f64>();
    if n < 2 {
        return (point, f64::NAN, f64::NAN);
    }
    let mut rng = Rng(BOOT_SEED ^ salt.wrapping_mul(0x2545_F491_4F6C_DD1D));
    let mut draws = Vec::with_capacity(BOOT);
    for _ in 0..BOOT {
        let (mut sx, mut sy) = (0.0, 0.0);
        for _ in 0..n {
            let i = rng.below(n);
            sx += x[i];
            sy += y[i];
        }
        draws.push(sx / sy);
    }
    draws.sort_by(f64::total_cmp);
    let q = |p: f64| draws[((p * (BOOT as f64 - 1.0)).round() as usize).min(BOOT - 1)];
    (point, q(0.025), q(0.975))
}

/// [`boot_ratio`] resampling clusters (recipe seeds) rather than pairs: the
/// set-up is a function of the seed, so pairs of one seed are not independent.
fn boot_ratio_clustered(x: &[f64], y: &[f64], cluster: &[u32], salt: u64) -> (f64, f64, f64) {
    let point = x.iter().sum::<f64>() / y.iter().sum::<f64>();
    let mut ids: Vec<u32> = cluster.to_vec();
    ids.sort_unstable();
    ids.dedup();
    let m = ids.len();
    if m < 2 {
        return (point, f64::NAN, f64::NAN);
    }
    let sums: Vec<(f64, f64)> = ids
        .iter()
        .map(|c| {
            let mut sx = 0.0;
            let mut sy = 0.0;
            for k in 0..x.len() {
                if cluster[k] == *c {
                    sx += x[k];
                    sy += y[k];
                }
            }
            (sx, sy)
        })
        .collect();
    let mut rng = Rng(BOOT_SEED ^ salt.wrapping_mul(0x2545_F491_4F6C_DD1D));
    let mut draws = Vec::with_capacity(BOOT);
    for _ in 0..BOOT {
        let (mut sx, mut sy) = (0.0, 0.0);
        for _ in 0..m {
            let (a, b) = sums[rng.below(m)];
            sx += a;
            sy += b;
        }
        draws.push(sx / sy);
    }
    draws.sort_by(f64::total_cmp);
    let q = |p: f64| draws[((p * (BOOT as f64 - 1.0)).round() as usize).min(BOOT - 1)];
    (point, q(0.025), q(0.975))
}

fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    if s.is_empty() {
        return f64::NAN;
    }
    let m = s.len() / 2;
    if s.len() % 2 == 1 {
        s[m]
    } else {
        (s[m - 1] + s[m]) / 2.0
    }
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn salt_of(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3)
    })
}

// ── the recipes and their parameter files ──────────────────────────

/// The parameter file of `arm` at `(size, i)`: v3's `T01` file with the
/// target's seed set and, for the v4 arm, the recipe's overrides.
fn params_for(
    v3: &Path,
    recipe: &Value,
    arm: &str,
    a: u8,
    n: u32,
    i: u32,
) -> Result<Value, String> {
    let sl = slug(a, n);
    let mut p = read_json(&v3.join(format!("runs/{sl}/T01.params.json")))?;
    p["targets"][0]["public_hash_seed"] = json!(TARGET_SEED_BASE + i64::from(i));
    p["seed"] = json!(eval_seed(i));
    p["factor_base"]["spec"]["seed"] = json!(eval_seed(i));
    let base_name = p["name"].as_str().unwrap_or("").replace("-T01", "");
    if arm == "v4" {
        let over = recipe["sizes"]
            .get(&sl)
            .ok_or_else(|| format!("recipe has no size {sl}"))?;
        if let Some(points) = over.get("points") {
            p["factor_base"]["spec"]["points"] = points.clone();
        }
        if let Some(w) = over.get("collection_window") {
            p["collection_window"] = w.clone();
        }
        if let Some(u) = over.get("unit_trials") {
            p["collection"]["unit_trials"] = u.clone();
        }
        if let Some(m) = over.get("max_trials") {
            p["max_trials"] = m.clone();
        }
    }
    if arm == "pos" {
        let scale = recipe["positive_control"]["columns_scale"]
            .as_f64()
            .unwrap_or(0.6);
        let cols = p["factor_base"]["spec"]["points"].as_u64().unwrap() / (2 * u64::from(n));
        let cols = ((cols as f64 * scale).round() as u64).max(8);
        p["factor_base"]["spec"]["points"] = json!(cols * 2 * u64::from(n));
    }
    p["name"] = json!(format!("{base_name}-s{}-{arm}-T{i:02}", eval_seed(i)));
    Ok(p)
}

fn arm_env(recipe: &Value, arm: &str, sl: &str) -> Vec<(String, String)> {
    let mut env = vec![("RAYON_NUM_THREADS".to_string(), "1".to_string())];
    // The lever is on for the v4 arm where the recipe says so for the size.
    if (arm == "v4" && recipe["sizes"][sl]["free"].as_bool() == Some(true)) || arm == "lev" {
        env.push(("KIC_FREE_RELATIONS".to_string(), "1".to_string()));
    }
    env
}

// ── run ─────────────────────────────────────────────────────────────

fn record_clean(rec: &Path) -> bool {
    let Ok(text) = fs::read_to_string(rec) else {
        return false;
    };
    let Some(last) = text.lines().last() else {
        return false;
    };
    let Ok(v) = serde_json::from_str::<Value>(last) else {
        return false;
    };
    let run = &v["run"];
    run["exit_status"].as_i64() == Some(0) && run["contended"].as_bool() == Some(false)
}

fn report_complete(out: &Path) -> bool {
    read_json(out).is_ok_and(|v| {
        v["status"].as_str() == Some("complete") && v["all_verified"].as_bool() == Some(true)
    })
}

#[allow(clippy::too_many_arguments)]
fn launch_isolated(
    isolate: &Path,
    cmd: &[String],
    env: &[(String, String)],
    out: &Path,
    label: &str,
    log_dir: &Path,
    max_other_cpu: f64,
) -> Result<(), String> {
    let rec = out.with_extension("").with_extension("isolation.jsonl");
    let err = out.with_extension("").with_extension("stderr");
    let tmp = out.with_extension("").with_extension("stderr.tmp");
    for attempt in 0..REFUSAL_LIMIT {
        let waited = wait_for_quiet();
        let stderr = fs::File::create(&tmp).map_err(|e| format!("{}: {e}", tmp.display()))?;
        let mut c = Command::new(isolate);
        c.args(["run", "--wait", "--cpus", CPUS, "--max-other-cpu"])
            .arg(max_other_cpu.to_string())
            .arg("--out")
            .arg(&rec)
            .args(["--label", label, "--"])
            .args(cmd)
            .stdout(Stdio::null())
            .stderr(Stdio::from(stderr));
        for (k, v) in env {
            c.env(k, v);
        }
        c.status()
            .map_err(|e| format!("{}: {e}", isolate.display()))?;
        if rec.exists() {
            fs::rename(&tmp, &err).map_err(|e| e.to_string())?;
            append(
                &log_dir.join("psi-waits.log"),
                &format!("{label} {waited:.1}\n"),
            );
            return Ok(());
        }
        let why = fs::read_to_string(&tmp).unwrap_or_default();
        append(
            &log_dir.join("refusals.log"),
            &format!(
                "{} {label} attempt {}: {}\n",
                utc_now(),
                attempt + 1,
                why.trim()
            ),
        );
        let _ = fs::remove_file(&tmp);
        std::thread::sleep(REFUSAL_WAIT);
    }
    Err(format!(
        "{label}: the machine stayed busy for {REFUSAL_LIMIT} tries; stopped (resumable)"
    ))
}

/// One arm's row: the first clean, complete attempt of up to three.
#[allow(clippy::too_many_arguments)]
fn price_row(
    ic: &Path,
    isolate: &Path,
    params: &Path,
    out_base: &Path,
    rho_seed: i64,
    env: &[(String, String)],
    log_dir: &Path,
    label: &str,
    max_other_cpu: f64,
) -> Result<bool, String> {
    for k in 0..=RETRIES {
        let out = if k == 0 {
            out_base.to_path_buf()
        } else {
            let name = out_base
                .file_name()
                .unwrap()
                .to_string_lossy()
                .replace(".price.json", &format!("-retry{k}.price.json"));
            out_base.with_file_name(name)
        };
        let rec = out.with_extension("").with_extension("isolation.jsonl");
        if !out.exists() || !rec.exists() {
            let cmd = vec![
                ic.to_string_lossy().into_owned(),
                "price".into(),
                "--params".into(),
                params.to_string_lossy().into_owned(),
                "--json".into(),
                "--out".into(),
                out.to_string_lossy().into_owned(),
                "--single-target".into(),
                "--rho-seed".into(),
                rho_seed.to_string(),
            ];
            launch_isolated(
                isolate,
                &cmd,
                env,
                &out,
                &format!("{label}#{k}"),
                log_dir,
                max_other_cpu,
            )?;
        }
        if record_clean(&rec) && report_complete(&out) {
            return Ok(true);
        }
    }
    Ok(false)
}

#[allow(clippy::too_many_arguments)]
fn run(
    ic: &Path,
    isolate: &Path,
    root: &Path,
    v3: &Path,
    recipe_path: &Path,
    targets: u32,
    aa: u32,
    pos: u32,
    lev: u32,
    max_minutes: f64,
    sizes: Option<String>,
    max_other_cpu: f64,
    runs: &str,
) -> Result<i32, String> {
    let recipe = read_json(recipe_path)?;
    let started = Instant::now();
    let log_dir = root.join(runs);
    fs::create_dir_all(&log_dir).map_err(|e| e.to_string())?;
    let mut holes = 0usize;
    for (a, n) in size_filter(&sizes) {
        let sl = slug(a, n);
        let dir = log_dir.join(&sl);
        fs::create_dir_all(dir.join("params")).map_err(|e| e.to_string())?;
        for i in 1..=targets {
            // Arm order alternates with the target: v3 first on odd ones.
            let mut arms: Vec<&str> = if i % 2 == 1 {
                vec!["v3", "v4"]
            } else {
                vec!["v4", "v3"]
            };
            if i <= aa {
                // The A/A arm sits at the other end from the first v3.
                if i % 2 == 1 {
                    arms.push("v3b");
                } else {
                    arms.insert(0, "v3b");
                }
            }
            if i <= pos {
                arms.push("pos");
            }
            if i <= lev {
                arms.push("lev");
            }
            for arm in arms {
                let out = dir.join(format!("T{i:02}-{arm}.price.json"));
                let any = (0..=RETRIES).any(|k| {
                    let o = if k == 0 {
                        out.clone()
                    } else {
                        out.with_file_name(format!("T{i:02}-{arm}-retry{k}.price.json"))
                    };
                    let rec = o.with_extension("").with_extension("isolation.jsonl");
                    record_clean(&rec) && report_complete(&o)
                });
                if any {
                    continue;
                }
                if started.elapsed().as_secs_f64() > max_minutes * 60.0 {
                    println!("time limit reached before {sl}/T{i:02}-{arm}; resumable");
                    return Ok(3);
                }
                let pname = match arm {
                    "v4" => "v4",
                    "pos" => "pos",
                    "lev" => "lev",
                    _ => "v3",
                };
                let params = params_for(v3, &recipe, pname, a, n, i)?;
                let params_path = dir
                    .join("params")
                    .join(format!("T{i:02}-{arm}.params.json"));
                if !params_path.exists() {
                    write_json(&params_path, &params)?;
                }
                let env = arm_env(&recipe, pname, &sl);
                let ok = match price_row(
                    ic,
                    isolate,
                    &params_path,
                    &out,
                    RHO_SEED_BASE + i64::from(i),
                    &env,
                    &log_dir,
                    &format!("{sl}/T{i:02}-{arm}.price.json"),
                    max_other_cpu,
                ) {
                    Ok(ok) => ok,
                    Err(e) if e.contains("stayed busy") => {
                        // Neighbours are using the host: give the lock back and
                        // be re-queued rather than hold it idle.
                        println!("{e}");
                        return Ok(3);
                    }
                    Err(e) => return Err(e),
                };
                println!(
                    "{sl} T{i:02} {arm}: {}",
                    if ok {
                        "clean"
                    } else {
                        "NOT CLEAN after retries"
                    }
                );
                if !ok {
                    append(
                        &dir.join("HOLES"),
                        &format!("T{i:02}-{arm} not clean after {RETRIES} retries\n"),
                    );
                    holes += 1;
                }
            }
        }
    }
    println!("panel complete for the requested sizes, {holes} rows without a clean attempt");
    Ok(if holes > 0 { 4 } else { 0 })
}

// ── train ───────────────────────────────────────────────────────────

fn train(
    ic: &Path,
    root: &Path,
    v3: &Path,
    grid_path: &Path,
    max_minutes: f64,
    sizes: Option<String>,
    confirm: bool,
) -> Result<i32, String> {
    let grid = read_json(grid_path)?;
    let started = Instant::now();
    let out_dir = root.join("training/raw");
    fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
    let ints = |key: &str| -> Result<Vec<i64>, String> {
        Ok(grid[key]
            .as_array()
            .ok_or(format!("grid has no {key}"))?
            .iter()
            .filter_map(Value::as_i64)
            .collect())
    };
    let tseeds = ints("target_seeds")?;
    // (free, columns, window, unit) configurations a size runs, and the seeds.
    let stage1 = if confirm {
        Some(read_json(&root.join("training/stage1.json"))?)
    } else {
        None
    };
    let seeds = if confirm {
        ints("confirm_seeds")?
    } else {
        ints("seeds")?
    };
    for (a, n) in size_filter(&sizes) {
        let sl = slug(a, n);
        let base = read_json(&v3.join(format!("runs/{sl}/T01.params.json")))?;
        let cols0 = grid["sizes"][&sl]["columns"]
            .as_u64()
            .ok_or("grid: columns")?;
        let win0 = base["collection_window"].as_u64().unwrap();
        let unit0 = base["collection"]["unit_trials"].as_u64().unwrap();
        let ppc = 2 * u64::from(n);
        let mut configs: Vec<(bool, u64, u64, u64)> = Vec::new();
        if let Some(st) = &stage1 {
            let cand = &st[&sl]["candidate"];
            if cand.is_null() {
                continue;
            }
            configs.push((false, cols0, win0, unit0));
            configs.push((
                true,
                cand["columns"].as_u64().ok_or("candidate columns")?,
                cand["window"].as_u64().ok_or("candidate window")?,
                cand["unit_trials"].as_u64().ok_or("candidate unit")?,
            ));
        } else {
            let free_modes: Vec<bool> = grid["free"]
                .as_array()
                .map(|a| a.iter().filter_map(Value::as_bool).collect())
                .unwrap_or_else(|| vec![false]);
            for &free in &free_modes {
                for cs in grid["columns_scale"].as_array().unwrap() {
                    for ws in grid["window_scale"].as_array().unwrap() {
                        for us in grid["unit_scale"].as_array().unwrap() {
                            configs.push((
                                free,
                                (cols0 as f64 * cs.as_f64().unwrap()).round() as u64,
                                ((win0 as f64 * ws.as_f64().unwrap()).round() as u64).max(8),
                                ((unit0 as f64 * us.as_f64().unwrap()).round() as u64).max(1),
                            ));
                        }
                    }
                }
            }
        }
        for (free, cols, win, unit) in configs {
            for &seed in &seeds {
                let tag = format!("{sl}-f{}-c{cols}-w{win}-u{unit}-s{seed}", u8::from(free));
                let out = out_dir.join(format!("{tag}.price.json"));
                if out.exists() && read_json(&out).is_ok() {
                    continue;
                }
                if started.elapsed().as_secs_f64() > max_minutes * 60.0 {
                    println!("time limit reached before {tag}; resumable");
                    return Ok(3);
                }
                let mut p = base.clone();
                p["seed"] = json!(seed);
                p["factor_base"]["spec"]["seed"] = json!(seed);
                p["factor_base"]["spec"]["points"] = json!(cols * ppc);
                p["collection_window"] = json!(win);
                p["collection"]["unit_trials"] = json!(unit);
                p["targets"] = json!(tseeds
                    .iter()
                    .map(|s| json!({"public_hash_seed": s}))
                    .collect::<Vec<_>>());
                p["name"] = json!(tag);
                let pp = out_dir.join(format!("{tag}.params.json"));
                write_json(&pp, &p)?;
                let mut c = Command::new("taskset");
                c.args(["-c", CPUS])
                    .arg(ic)
                    .args(["price", "--params"])
                    .arg(&pp)
                    .args(["--json", "--repeats", "3", "--repeats-fast", "3", "--out"])
                    .arg(&out)
                    .env("RAYON_NUM_THREADS", "1")
                    .stdout(Stdio::null());
                if free {
                    c.env("KIC_FREE_RELATIONS", "1");
                }
                let stderr = fs::File::create(out_dir.join(format!("{tag}.stderr")))
                    .map_err(|e| e.to_string())?;
                let status = c
                    .stderr(Stdio::from(stderr))
                    .status()
                    .map_err(|e| e.to_string())?;
                println!("{tag}: exit {:?}", status.code());
            }
        }
    }
    Ok(0)
}

// ── select ──────────────────────────────────────────────────────────

/// A batch-mode training run's cold cost, `S` units: the least over its
/// repetitions of (every phase but the descent's) plus one mean descent.
/// The work is deterministic, so interference only adds; the least
/// repetition is the least biased estimate of it.
fn cold_min(v: &Value) -> Option<f64> {
    let k = v["targets"].as_f64()?;
    let sqrt_r = v["r"].as_f64()?.sqrt();
    v["repetitions"]
        .as_array()?
        .iter()
        .filter_map(|r| {
            let ph = &r["phases_units"];
            let per = ph["descent"].as_f64()? + ph["verify_final"].as_f64()?;
            Some((r["total_units"].as_f64()? - per + per / k) / sqrt_r)
        })
        .fold(None, |m: Option<f64>, x| Some(m.map_or(x, |m| m.min(x))))
}

/// The frozen rule (PROTOCOL.md, "Selection").  Stage 1: per size, among the
/// grid's lever-on configurations the one with the least mean cold cost
/// over the training seeds is a candidate if it is at least 2% cheaper than
/// the v3 configuration and cheaper on at least two thirds of the seeds.
/// Final stage: a candidate becomes the v4 recipe only if it repeats that on
/// fresh training seeds (`confirm_seeds`); otherwise the v3 recipe stands,
/// lever off.
fn select(root: &Path, v3: &Path, grid_path: &Path, final_stage: bool) -> Result<i32, String> {
    type Key = (String, u8, u64, u64, u64);
    let grid = read_json(grid_path)?;
    let ints = |key: &str| -> Vec<i64> {
        grid[key]
            .as_array()
            .map(|a| a.iter().filter_map(Value::as_i64).collect())
            .unwrap_or_default()
    };
    let (seeds, confirm_seeds) = (ints("seeds"), ints("confirm_seeds"));
    let raw = root.join("training/raw");
    // (size, free, cols, window, unit) -> seed -> cold cost
    let mut table: BTreeMap<Key, BTreeMap<i64, f64>> = BTreeMap::new();
    for e in fs::read_dir(&raw)
        .map_err(|e| format!("{}: {e}", raw.display()))?
        .filter_map(Result::ok)
    {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(tag) = name.strip_suffix(".price.json") else {
            continue;
        };
        let parts: Vec<&str> = tag.split('-').collect();
        if parts.len() != 6 {
            continue;
        }
        let num = |s: &str, p: char| s.strip_prefix(p).and_then(|x| x.parse::<i64>().ok());
        let (Some(free), Some(cols), Some(win), Some(unit), Some(seed)) = (
            num(parts[1], 'f'),
            num(parts[2], 'c'),
            num(parts[3], 'w'),
            num(parts[4], 'u'),
            num(parts[5], 's'),
        ) else {
            continue;
        };
        let Ok(v) = read_json(&e.path()) else {
            continue;
        };
        if v["status"].as_str() != Some("complete") || v["all_verified"].as_bool() != Some(true) {
            continue;
        }
        let Some(j) = cold_min(&v) else { continue };
        table
            .entry((
                parts[0].to_string(),
                free as u8,
                cols as u64,
                win as u64,
                unit as u64,
            ))
            .or_default()
            .insert(seed, j);
    }
    let mean_over = |m: &BTreeMap<i64, f64>, which: &[i64]| -> Option<f64> {
        which
            .iter()
            .all(|s| m.contains_key(s))
            .then(|| which.iter().map(|s| m[s]).sum::<f64>() / which.len() as f64)
    };
    let wins = |a: &BTreeMap<i64, f64>, b: &BTreeMap<i64, f64>, which: &[i64]| -> usize {
        which.iter().filter(|s| a[s] < b[s]).count()
    };
    let stage1_path = root.join("training/stage1.json");
    let stage1 = if final_stage {
        Some(read_json(&stage1_path)?)
    } else {
        None
    };
    let mut recipe_sizes = serde_json::Map::new();
    let mut summary = serde_json::Map::new();
    let mut cands = serde_json::Map::new();
    for (a, n) in SIZES {
        let sl = slug(a, n);
        let base_params = read_json(&v3.join(format!("runs/{sl}/T01.params.json")))?;
        let cols0 = grid["sizes"][&sl]["columns"].as_u64().ok_or("columns")?;
        let win0 = base_params["collection_window"].as_u64().unwrap();
        let unit0 = base_params["collection"]["unit_trials"].as_u64().unwrap();
        let max_trials = base_params["max_trials"].clone();
        let base_key: Key = (sl.clone(), 0u8, cols0, win0, unit0);
        let base = table
            .get(&base_key)
            .ok_or(format!("{sl}: no baseline runs"))?;
        let jb =
            mean_over(base, &seeds).ok_or(format!("{sl}: the baseline lacks training seeds"))?;
        // Stage 1.
        let mut rows = Vec::new();
        let mut best: Option<(Key, f64)> = None;
        let mut best_off: Option<(Key, f64)> = None;
        for (k, m) in table.iter().filter(|(k, _)| k.0 == sl) {
            let Some(j) = mean_over(m, &seeds) else {
                continue;
            };
            rows.push(
                json!({"free": k.1, "columns": k.2, "window": k.3, "unit_trials": k.4, "J": j,
                             "per_seed": seeds.iter().map(|s| m[s]).collect::<Vec<_>>()}),
            );
            let slot = if k.1 == 1 { &mut best } else { &mut best_off };
            if slot.as_ref().is_none_or(|(_, bj)| j < *bj) {
                *slot = Some((k.clone(), j));
            }
        }
        let mut cand = Value::Null;
        let mut verdict = "no lever-on configuration has complete training runs".to_string();
        if let Some((k, jc)) = &best {
            let w = wins(&table[k], base, &seeds);
            let eligible = *jc <= 0.98 * jb && w * 3 >= seeds.len() * 2;
            verdict = format!("stage 1: best lever-on J {jc:.4} against baseline J {jb:.4} (ratio {:.4}), cheaper on {w} of {} seeds: {}",
                              jc / jb, seeds.len(), if eligible { "a candidate" } else { "not a candidate" });
            if eligible {
                cand = json!({"columns": k.2, "window": k.3, "unit_trials": k.4, "J": jc});
            }
        }
        cands.insert(
            sl.clone(),
            json!({"candidate": cand, "J_baseline": jb, "verdict": verdict}),
        );
        if let Some(st) = &stage1 {
            // Final stage.
            let c = &st[&sl]["candidate"];
            let mut chosen = json!({"points": cols0 * 2 * u64::from(n), "collection_window": win0, "unit_trials": unit0,
                                    "max_trials": max_trials, "free": false, "accepted": false});
            let mut final_verdict = format!("{verdict}; no candidate, v3 recipe stands");
            let mut confirm = Value::Null;
            if !c.is_null() {
                let ck: Key = (
                    sl.clone(),
                    1,
                    c["columns"].as_u64().unwrap(),
                    c["window"].as_u64().unwrap(),
                    c["unit_trials"].as_u64().unwrap(),
                );
                if let (Some(cm), Some(jcc), Some(jbc)) = (
                    table.get(&ck),
                    table.get(&ck).and_then(|m| mean_over(m, &confirm_seeds)),
                    mean_over(base, &confirm_seeds),
                ) {
                    let w = wins(cm, base, &confirm_seeds);
                    let ok = jcc <= 0.98 * jbc && w * 3 >= confirm_seeds.len() * 2;
                    confirm = json!({"seeds": confirm_seeds, "J_candidate": jcc, "J_baseline": jbc, "ratio": jcc / jbc,
                                     "cheaper_on": w, "of": confirm_seeds.len(), "confirmed": ok});
                    final_verdict = format!("{verdict}; confirmation on {} fresh seeds: ratio {:.4}, cheaper on {w}: {}",
                                            confirm_seeds.len(), jcc / jbc, if ok { "taken" } else { "not confirmed, v3 recipe stands" });
                    if ok {
                        chosen = json!({"points": ck.2 * 2 * u64::from(n), "collection_window": ck.3, "unit_trials": ck.4,
                                        "max_trials": max_trials, "free": true, "accepted": true});
                    }
                } else {
                    final_verdict = format!(
                        "{verdict}; the confirmation runs are incomplete, v3 recipe stands"
                    );
                }
            }
            chosen["training_J_baseline"] = json!(jb);
            chosen["training_J_best_lever_on"] = json!(best.as_ref().map(|b| b.1));
            chosen["training_J_best_lever_off"] = json!(best_off.as_ref().map(|b| b.1));
            chosen["best_lever_off_config"] = json!(best_off
                .as_ref()
                .map(|b| json!({"columns": b.0.2, "window": b.0.3, "unit_trials": b.0.4})));
            chosen["confirmation"] = confirm;
            recipe_sizes.insert(sl.clone(), chosen.clone());
            summary.insert(sl.clone(), json!({"baseline_J": jb, "verdict": final_verdict, "configs": rows, "chosen": chosen}));
            println!("{sl}: {final_verdict}");
        } else {
            summary.insert(
                sl.clone(),
                json!({"baseline_J": jb, "verdict": verdict, "configs": rows}),
            );
            println!("{sl}: {verdict}");
        }
    }
    write_json(&stage1_path, &Value::Object(cands))?;
    write_json(&root.join("training/summary.json"), &Value::Object(summary))?;
    if final_stage {
        let recipe = json!({"schema": "ic-rule-v4-recipe/1", "sizes": recipe_sizes,
                            "positive_control": {"columns_scale": grid["positive_control_columns_scale"].clone()},
                            "selected_by": "ic_rule_v4 select --final, the rule frozen in PROTOCOL.md (with amendment A1)"});
        write_json(&root.join("recipe.json"), &recipe)?;
    }
    Ok(0)
}

/// Gauss-Jordan inverse of a small symmetric matrix.
fn invert(mut a: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut inv: Vec<Vec<f64>> = (0..n)
        .map(|i| (0..n).map(|j| f64::from(u8::from(i == j))).collect())
        .collect();
    for c in 0..n {
        let piv = (c..n).max_by(|&x, &y| a[x][c].abs().total_cmp(&a[y][c].abs()))?;
        if a[piv][c].abs() < 1e-12 {
            return None;
        }
        a.swap(c, piv);
        inv.swap(c, piv);
        let d = a[c][c];
        for j in 0..n {
            a[c][j] /= d;
            inv[c][j] /= d;
        }
        for r in 0..n {
            if r != c {
                let f = a[r][c];
                for j in 0..n {
                    a[r][j] -= f * a[c][j];
                    inv[r][j] -= f * inv[c][j];
                }
            }
        }
    }
    Some(inv)
}

/// Amendment A2.  Per size, over the lever-off grid runs on the training
/// seeds, fit `ln J = seed + b1 lc + b2 lc^2 + b3 lw + b4 lw^2 + b5 lu` by
/// ordinary least squares (`lc`, `lw`, `lu`: the logs of columns, window and
/// unit size over v3's), predict every grid configuration's contrast with
/// the v3 configuration, and take the least.  It becomes the v4 recipe (lever
/// off) if the predicted ratio is at most 0.97 and its one-sided two-sigma
/// bound is below 1; otherwise the v3 recipe stands at that size.
fn select_model(root: &Path, v3: &Path, grid_path: &Path) -> Result<i32, String> {
    let grid = read_json(grid_path)?;
    let seeds: Vec<i64> = grid["seeds"]
        .as_array()
        .ok_or("seeds")?
        .iter()
        .filter_map(Value::as_i64)
        .collect();
    let raw = root.join("training/raw");
    let mut recipe_sizes = serde_json::Map::new();
    let mut model_out = serde_json::Map::new();
    for (a, n) in SIZES {
        let sl = slug(a, n);
        let base_params = read_json(&v3.join(format!("runs/{sl}/T01.params.json")))?;
        let cols0 = grid["sizes"][&sl]["columns"].as_u64().ok_or("columns")?;
        let win0 = base_params["collection_window"].as_u64().unwrap();
        let unit0 = base_params["collection"]["unit_trials"].as_u64().unwrap();
        // Lever-off runs on the training seeds: (cols, window, unit) -> seed -> J.
        let mut by_cfg: BTreeMap<(u64, u64, u64), BTreeMap<i64, f64>> = BTreeMap::new();
        for e in fs::read_dir(&raw)
            .map_err(|e| e.to_string())?
            .filter_map(Result::ok)
        {
            let name = e.file_name().to_string_lossy().into_owned();
            let Some(tag) = name.strip_suffix(".price.json") else {
                continue;
            };
            let parts: Vec<&str> = tag.split('-').collect();
            if parts.len() != 6 || parts[0] != sl || parts[1] != "f0" {
                continue;
            }
            let num = |s: &str, p: char| s.strip_prefix(p).and_then(|x| x.parse::<i64>().ok());
            let (Some(c), Some(w), Some(u), Some(seed)) = (
                num(parts[2], 'c'),
                num(parts[3], 'w'),
                num(parts[4], 'u'),
                num(parts[5], 's'),
            ) else {
                continue;
            };
            if !seeds.contains(&seed) {
                continue;
            }
            let Ok(v) = read_json(&e.path()) else {
                continue;
            };
            if v["status"].as_str() != Some("complete") || v["all_verified"].as_bool() != Some(true)
            {
                continue;
            }
            if let Some(j) = cold_min(&v) {
                by_cfg
                    .entry((c as u64, w as u64, u as u64))
                    .or_default()
                    .insert(seed, j);
            }
        }
        let feat = |k: &(u64, u64, u64)| -> [f64; 5] {
            let lc = (k.0 as f64 / cols0 as f64).ln();
            let lw = (k.1 as f64 / win0 as f64).ln();
            let lu = (k.2 as f64 / unit0 as f64).ln();
            [lc, lc * lc, lw, lw * lw, lu]
        };
        let ns = seeds.len();
        let p = ns + 5;
        let mut xtx = vec![vec![0.0; p]; p];
        let mut xty = vec![0.0; p];
        let mut rows: Vec<(Vec<f64>, f64)> = Vec::new();
        for (k, m) in &by_cfg {
            for (si, sd) in seeds.iter().enumerate() {
                let Some(j) = m.get(sd) else { continue };
                let mut x = vec![0.0; p];
                x[si] = 1.0;
                x[ns..].copy_from_slice(&feat(k));
                rows.push((x, j.ln()));
            }
        }
        if rows.len() < p + 8 {
            return Err(format!("{sl}: only {} lever-off training runs", rows.len()));
        }
        for (x, y) in &rows {
            for i in 0..p {
                xty[i] += x[i] * y;
                for j in 0..p {
                    xtx[i][j] += x[i] * x[j];
                }
            }
        }
        let inv = invert(xtx).ok_or(format!("{sl}: singular design"))?;
        let beta: Vec<f64> = (0..p)
            .map(|i| (0..p).map(|j| inv[i][j] * xty[j]).sum())
            .collect();
        let rss: f64 = rows
            .iter()
            .map(|(x, y)| (y - x.iter().zip(&beta).map(|(a, b)| a * b).sum::<f64>()).powi(2))
            .sum();
        let sigma2 = rss / (rows.len() - p) as f64;
        // Amendment A3: knob by knob.  For each knob, the grid value of least
        // predicted cost with the other knobs at v3's values; the move is made
        // if its predicted ratio is at most 0.99 and its one-sided two-sigma
        // bound is below 1.  The accepted moves are applied together.
        let values = |sel: usize| -> Vec<u64> {
            let mut v: Vec<u64> = by_cfg.keys().map(|k| [k.0, k.1, k.2][sel]).collect();
            v.sort_unstable();
            v.dedup();
            v
        };
        let base_cfg = (cols0, win0, unit0);
        let mut moves = Vec::new();
        let mut cfg = base_cfg;
        let mut joint = 0.0f64;
        let mut preds = Vec::new();
        for (sel, name) in [(0usize, "columns"), (1, "window"), (2, "unit_trials")] {
            let mut best: Option<(u64, f64, f64)> = None;
            for v in values(sel) {
                let mut k = base_cfg;
                match sel {
                    0 => k.0 = v,
                    1 => k.1 = v,
                    _ => k.2 = v,
                }
                let f = feat(&k);
                let d: f64 = (0..5).map(|i| f[i] * beta[ns + i]).sum();
                let var: f64 = (0..5)
                    .map(|i| {
                        (0..5)
                            .map(|j| f[i] * f[j] * sigma2 * inv[ns + i][ns + j])
                            .sum::<f64>()
                    })
                    .sum();
                let se = var.max(0.0).sqrt();
                preds.push(json!({"knob": name, "value": v, "ratio_to_v3": d.exp(), "se_log": se}));
                if best.as_ref().is_none_or(|b| d < b.1) {
                    best = Some((v, d, se));
                }
            }
            if let Some((v, d, se)) = best {
                let take = d <= 0.99f64.ln() && d + 2.0 * se <= 0.0;
                moves.push(json!({"knob": name, "best_value": v, "predicted_ratio": d.exp(), "se_log": se, "taken": take}));
                if take {
                    joint += d;
                    match sel {
                        0 => cfg.0 = v,
                        1 => cfg.1 = v,
                        _ => cfg.2 = v,
                    }
                }
            }
        }
        // Amendment A4: the recipe is the grid configuration of least predicted
        // cost (A2's argmin), taken whenever its prediction is below the v3
        // configuration's; A3's knob-wise gate is kept above as a report.
        let mut gbest: Option<((u64, u64, u64), f64, f64)> = None;
        for k in by_cfg.keys() {
            let f = feat(k);
            let d: f64 = (0..5).map(|i| f[i] * beta[ns + i]).sum();
            let var: f64 = (0..5)
                .map(|i| {
                    (0..5)
                        .map(|j| f[i] * f[j] * sigma2 * inv[ns + i][ns + j])
                        .sum::<f64>()
                })
                .sum();
            if gbest.as_ref().is_none_or(|b| d < b.1) {
                gbest = Some((*k, d, var.max(0.0).sqrt()));
            }
        }
        let (gk, gd, gse) = gbest.ok_or("no grid points")?;
        let accept = gd < 0.0 && gk != base_cfg;
        let (k, d, se) = if accept {
            (gk, gd, gse)
        } else {
            (base_cfg, 0.0, 0.0)
        };
        let _ = (cfg, joint);
        let chosen = json!({"points": k.0 * 2 * u64::from(n), "collection_window": k.1, "unit_trials": k.2,
                            "max_trials": base_params["max_trials"].clone(), "free": false, "accepted": accept});
        let mut chosen = chosen;
        chosen["model"] = json!({"coefficients": {"lc": beta[ns], "lc2": beta[ns+1], "lw": beta[ns+2], "lw2": beta[ns+3], "lu": beta[ns+4]},
            "se": {"lc": (sigma2 * inv[ns][ns]).sqrt(), "lc2": (sigma2 * inv[ns+1][ns+1]).sqrt(), "lw": (sigma2 * inv[ns+2][ns+2]).sqrt(),
                   "lw2": (sigma2 * inv[ns+3][ns+3]).sqrt(), "lu": (sigma2 * inv[ns+4][ns+4]).sqrt()},
            "sigma_log": sigma2.sqrt(), "runs": rows.len(), "best_grid_config": {"columns": k.0, "window": k.1, "unit_trials": k.2},
            "predicted_ratio": d.exp(), "se_log": se, "a3_knobwise_report": moves, "rule": "amendment A4: the grid configuration of least predicted cost, taken whenever it is predicted below the v3 configuration; A3's knob-wise gate is only reported"});
        println!("{sl}: columns {} window {} unit {} (v3: {} {} {}), predicted ratio {:.4} (se {:.4} in log): {}",
                 k.0, k.1, k.2, cols0, win0, unit0, d.exp(), se, if accept { "taken" } else { "v3 recipe stands" });
        recipe_sizes.insert(sl.clone(), chosen.clone());
        model_out.insert(
            sl.clone(),
            json!({"chosen": chosen, "grid_predictions": preds}),
        );
    }
    write_json(&root.join("training/model.json"), &Value::Object(model_out))?;
    let recipe = json!({"schema": "ic-rule-v4-recipe/1", "sizes": recipe_sizes,
                        "positive_control": {"columns_scale": grid["positive_control_columns_scale"].clone()},
                        "selected_by": "ic_rule_v4 select --model, amendments A2 to A4 of PROTOCOL.md"});
    write_json(&root.join("recipe.json"), &recipe)?;
    Ok(0)
}

/// The free-relation lever's paired effect, from the grid: every configuration
/// and training seed run with the lever off and on.  Geometric means of the
/// ratios (on over off) of the cold cost, the build phase and the collection
/// trials, by size and by columns scale, with the relations the lever made.
fn lever(root: &Path, v3: &Path, grid_path: &Path) -> Result<i32, String> {
    let grid = read_json(grid_path)?;
    let seeds: Vec<i64> = grid["seeds"]
        .as_array()
        .ok_or("seeds")?
        .iter()
        .filter_map(Value::as_i64)
        .collect();
    let raw = root.join("training/raw");
    let mut runs: BTreeMap<(String, u8, u64, u64, u64, i64), Value> = BTreeMap::new();
    for e in fs::read_dir(&raw)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
    {
        let name = e.file_name().to_string_lossy().into_owned();
        let Some(tag) = name.strip_suffix(".price.json") else {
            continue;
        };
        let parts: Vec<&str> = tag.split('-').collect();
        if parts.len() != 6 {
            continue;
        }
        let num = |s: &str, p: char| s.strip_prefix(p).and_then(|x| x.parse::<i64>().ok());
        let (Some(fr), Some(c), Some(w), Some(u), Some(sd)) = (
            num(parts[1], 'f'),
            num(parts[2], 'c'),
            num(parts[3], 'w'),
            num(parts[4], 'u'),
            num(parts[5], 's'),
        ) else {
            continue;
        };
        if !seeds.contains(&sd) {
            continue;
        }
        let Ok(v) = read_json(&e.path()) else {
            continue;
        };
        if v["status"].as_str() == Some("complete") {
            runs.insert(
                (
                    parts[0].to_string(),
                    fr as u8,
                    c as u64,
                    w as u64,
                    u as u64,
                    sd,
                ),
                v,
            );
        }
    }
    let least = |v: &Value, ph: &str| -> Option<f64> {
        v["repetitions"]
            .as_array()?
            .iter()
            .filter_map(|r| r["phases_units"][ph].as_f64())
            .fold(None, |m: Option<f64>, x| Some(m.map_or(x, |m| m.min(x))))
    };
    let mut out = serde_json::Map::new();
    for (a, n) in SIZES {
        let sl = slug(a, n);
        let cols0 = grid["sizes"][&sl]["columns"].as_u64().ok_or("columns")?;
        let mut groups: BTreeMap<u64, Vec<[f64; 5]>> = BTreeMap::new();
        let mut log_cost = Vec::new();
        for (k, off) in runs.iter().filter(|(k, _)| k.0 == sl && k.1 == 0) {
            let Some(on) = runs.get(&(k.0.clone(), 1, k.2, k.3, k.4, k.5)) else {
                continue;
            };
            let (Some(jo), Some(jn)) = (cold_min(off), cold_min(on)) else {
                continue;
            };
            let (Some(bo), Some(bn)) = (least(off, "build"), least(on, "build")) else {
                continue;
            };
            let t_off = off["counts"]["collect"]["trials"]
                .as_f64()
                .unwrap_or(f64::NAN);
            let t_on = on["counts"]["collect"]["trials"]
                .as_f64()
                .unwrap_or(f64::NAN);
            let made = on["counts"]["free_relations"]["relations_made"]
                .as_f64()
                .unwrap_or(0.0);
            groups.entry(k.2).or_default().push([
                (jn / jo).ln(),
                (bn / bo).ln(),
                (t_on / t_off).ln(),
                made,
                on["counts"]["logs"]["relations_accepted"]
                    .as_f64()
                    .unwrap_or(f64::NAN),
            ]);
            log_cost.push((jn / jo).ln());
        }
        let m = |v: &[f64]| v.iter().sum::<f64>() / v.len() as f64;
        let sd = |v: &[f64]| {
            (v.iter().map(|x| (x - m(v)).powi(2)).sum::<f64>() / (v.len() as f64 - 1.0)).sqrt()
        };
        let by_cols: Vec<Value> = groups
            .iter()
            .map(|(c, g)| {
                let col = |i: usize| g.iter().map(|r| r[i]).collect::<Vec<_>>();
                json!({"columns": c, "columns_over_v3": *c as f64 / cols0 as f64, "pairs": g.len(),
                       "cost_on_over_off": m(&col(0)).exp(), "build_on_over_off": m(&col(1)).exp(),
                       "trials_on_over_off": m(&col(2)).exp(), "relations_made_mean": m(&col(3)), "relations_accepted_mean": m(&col(4))})
            })
            .collect();
        out.insert(sl, json!({"pairs": log_cost.len(), "cost_on_over_off": m(&log_cost).exp(),
                               "log_cost_se": sd(&log_cost) / (log_cost.len() as f64).sqrt(), "by_columns": by_cols}));
    }
    let _ = v3;
    write_json(
        &root.join("training/lever.json"),
        &Value::Object(out.clone()),
    )?;
    for (k, v) in &out {
        println!(
            "{k}: lever on over off, cost {:.3} (se log {:.3}, {} pairs)",
            v["cost_on_over_off"].as_f64().unwrap(),
            v["log_cost_se"].as_f64().unwrap(),
            v["pairs"]
        );
    }
    Ok(0)
}

// ── per-process records ─────────────────────────────────────────────

struct Proc {
    i: u32,
    report: Value,
}

/// The first clean, complete attempt of `T<i>-<arm>`.
fn first_clean(dir: &Path, i: u32, arm: &str) -> Option<Proc> {
    for k in 0..=RETRIES {
        let name = if k == 0 {
            format!("T{i:02}-{arm}.price.json")
        } else {
            format!("T{i:02}-{arm}-retry{k}.price.json")
        };
        let out = dir.join(name);
        let rec = out.with_extension("").with_extension("isolation.jsonl");
        if out.exists() && record_clean(&rec) && report_complete(&out) {
            return Some(Proc {
                i,
                report: read_json(&out).ok()?,
            });
        }
    }
    None
}

fn arm_procs(dir: &Path, arm: &str) -> BTreeMap<u32, Proc> {
    let mut m = BTreeMap::new();
    for i in 1..=200u32 {
        if let Some(p) = first_clean(dir, i, arm) {
            m.insert(p.i, p);
        }
    }
    m
}

/// Cold units of a process: set-up plus the index calculus's online.
fn cold_units(r: &Value) -> Option<f64> {
    Some(f(r, &["median", "setup_units"])? + f(r, &["median", "ic_online_units"])?)
}

fn setup_units(r: &Value) -> Option<f64> {
    f(r, &["median", "setup_units"])
}

fn online_units(r: &Value) -> Option<f64> {
    f(r, &["median", "ic_online_units"])
}

/// Wall nanoseconds of the median repetition's set-up and online.
fn wall_cold_ns(r: &Value) -> Option<f64> {
    let reps = r["repetitions"].as_array()?;
    let setups: Vec<f64> = reps.iter().filter_map(|x| x["setup_ns"].as_f64()).collect();
    let onl: Vec<f64> = reps
        .iter()
        .filter_map(|x| x["ic_online"]["wall_ns"].as_f64())
        .collect();
    Some(median(&setups) + median(&onl))
}

/// The least over the repetitions of set-up plus online, in units: the
/// deterministic work's least interfered measurement (secondary).
fn cold_min_units(r: &Value) -> Option<f64> {
    r["repetitions"]
        .as_array()?
        .iter()
        .filter_map(|x| Some(x["setup_units"].as_f64()? + x["ic_online"]["units"].as_f64()?))
        .fold(None, |m: Option<f64>, x| Some(m.map_or(x, |m| m.min(x))))
}

fn rho_cold_canonical_units(r: &Value) -> Option<f64> {
    Some(f(r, &["median", "rho_setup_units"])? + f(r, &["median", "rho_model_units"])?)
}

fn rho_cold_asbuilt_units(r: &Value) -> Option<f64> {
    Some(f(r, &["median", "rho_setup_units"])? + f(r, &["median", "rho_online_units"])?)
}

// ── the §5a band (counted GAE) ─────────────────────────────────────

/// Counted group-addition equivalents of the index calculus's complete
/// solve for one process, with the unpriced native work counted beside.
/// A scalar multiplication is priced at 1.5 n additions (double and add);
/// the field work inside an addition is not split out.  Lower bounds where
/// a counter is not emitted (the linear algebra, the descent's trial
/// bookkeeping), as AGENTS.md section 5a asks to be said.
fn ic_counts(r: &Value, n: u32) -> (f64, f64) {
    let pass = &r["counts"]["pass"];
    let stored = pass["build"]["stored_pairs"].as_f64().unwrap_or(0.0);
    let scanned = pass["collect"]["summands_scanned"].as_f64().unwrap_or(0.0);
    let trials = pass["collect"]["trials"].as_f64().unwrap_or(0.0);
    let cof = pass["select"]["selection_cost"]["cofactor_multiplications"]
        .as_f64()
        .unwrap_or(0.0);
    let dtrials = pass["descent"]["trials_total"].as_f64().unwrap_or(0.0);
    let sm = 1.5 * f64::from(n);
    let charged = stored + scanned + sm * (trials + cof + 2.0) + dtrials;
    // Canonical keys and presence-filter probes: one of each per stored
    // pair, per scanned summand and per descent trial; table inserts.
    let canon = stored + scanned + dtrials;
    let probes = scanned + dtrials;
    let inserts = stored;
    (charged, canon + probes + inserts)
}

fn rho_counts(r: &Value, n: u32) -> (f64, f64) {
    let c = &r["rho_policy"]["counters"];
    let walk = c["walk_operations"].as_f64().unwrap_or(0.0);
    let adds = walk
        + c["setup_additions"].as_f64().unwrap_or(0.0)
        + c["setup_doublings"].as_f64().unwrap_or(0.0)
        + c["target_setup_additions"].as_f64().unwrap_or(0.0)
        + c["target_setup_doublings"].as_f64().unwrap_or(0.0)
        + c["start_additions"].as_f64().unwrap_or(0.0)
        + c["look_ahead_additions"].as_f64().unwrap_or(0.0)
        + c["verification_operations"].as_f64().unwrap_or(0.0);
    let sm = 1.5 * f64::from(n);
    let scalar = sm
        * (r["rho_setup"]["setup_ops"]["scalar_mults"]
            .as_f64()
            .unwrap_or(0.0)
            + r["rho_counts"]["target_ops"]["scalar_mults"]
                .as_f64()
                .unwrap_or(0.0));
    let canon = c["canonicalisations_uncharged"].as_f64().unwrap_or(0.0);
    let dp = c["distinguished_points_stored"].as_f64().unwrap_or(0.0)
        + c["distinguished_points_stored"].as_f64().unwrap_or(0.0);
    (adds + scalar, canon + dp)
}

// ── analyse ─────────────────────────────────────────────────────────

/// The strong walk's ns a step on T01..T08 of a size, from the reference
/// stage (`ic_rule_v4 reference`): its verified rows only.
fn strong_ns_per_step(root: &Path, a: u8, n: u32) -> Vec<f64> {
    let dir = root.join("runs/reference");
    (1..=8u32)
        .filter_map(|i| {
            let out = dir.join(format!("{}-T{i:02}.strong.json", slug(a, n)));
            let rec = out.with_extension("").with_extension("isolation.jsonl");
            if !record_clean(&rec) {
                return None;
            }
            let line = fs::read_to_string(&out).ok()?;
            let v: Value = serde_json::from_str(line.lines().next()?).ok()?;
            if v["verified"] != true {
                return None;
            }
            Some(v["walk_ms"].as_f64()? * 1e6 / v["walk_steps"].as_f64()?)
        })
        .collect()
}

fn v3_scalar(v3: &Path, a: u8, n: u32, i: u32) -> Option<String> {
    let p = v3.join(format!("runs/{}/T{i:02}-R1.price.json", slug(a, n)));
    let v = read_json(&p).ok()?;
    v["certificates"]["ic"]["scalar"]
        .as_str()
        .map(str::to_string)
}

fn analyse(root: &Path, v3: &Path, runs: &str) -> Result<i32, String> {
    let mut sizes_out = Vec::new();
    let mut lever_pairs: Vec<f64> = Vec::new();
    let mut improved = 0;
    let mut regressed = 0;
    let mut incomplete = Vec::new();
    for (a, n) in SIZES {
        let sl = slug(a, n);
        let dir = root.join(runs).join(&sl);
        let p3 = arm_procs(&dir, "v3");
        let p4 = arm_procs(&dir, "v4");
        let pb = arm_procs(&dir, "v3b");
        let pb_pos = arm_procs(&dir, "pos");
        let pb_lev = arm_procs(&dir, "lev");
        let ids: Vec<u32> = p3.keys().copied().filter(|i| p4.contains_key(i)).collect();
        if ids.is_empty() {
            incomplete.push(sl.clone());
            continue;
        }
        // Correctness: every process verified (report_complete) and every
        // logarithm is the one rule v3 recovered.
        let mut log_ok = 0;
        let mut log_bad = Vec::new();
        for &i in &ids {
            let want = v3_scalar(v3, a, n, i);
            for (name, procs) in [("v3", &p3), ("v4", &p4)] {
                let r = &procs[&i].report;
                let ic = r["certificates"]["ic"]["scalar"]
                    .as_str()
                    .map(str::to_string);
                let rho = r["certificates"]["rho"]["scalar"]
                    .as_str()
                    .map(str::to_string);
                if ic.is_some() && ic == rho && want.clone().is_none_or(|w| Some(w) == ic) {
                    log_ok += 1;
                } else {
                    log_bad.push(format!("T{i:02}-{name}"));
                }
            }
        }
        let get = |procs: &BTreeMap<u32, Proc>, g: &dyn Fn(&Value) -> Option<f64>| -> Vec<f64> {
            ids.iter()
                .map(|i| g(&procs[i].report).unwrap_or(f64::NAN))
                .collect()
        };
        let c3 = get(&p3, &cold_units);
        let c4 = get(&p4, &cold_units);
        let s3 = get(&p3, &setup_units);
        let s4 = get(&p4, &setup_units);
        let o3 = get(&p3, &online_units);
        let o4 = get(&p4, &online_units);
        let w3 = get(&p3, &wall_cold_ns);
        let w4 = get(&p4, &wall_cold_ns);
        // How busy the neighbours were during this size's processes.
        let neigh: Vec<f64> = ["v3", "v4"]
            .iter()
            .flat_map(|arm| {
                let (dir, arm) = (&dir, *arm);
                ids.iter().filter_map(move |i| {
                    let rec = dir.join(format!("T{i:02}-{arm}.isolation.jsonl"));
                    let last = fs::read_to_string(&rec).ok()?.lines().last()?.to_string();
                    let v: Value = serde_json::from_str(&last).ok()?;
                    Some(
                        v["run"]["other_cpu_seconds"].as_f64()?
                            / v["run"]["wall_seconds"].as_f64()?,
                    )
                })
            })
            .collect();
        let salt = salt_of(&sl);
        let clusters: Vec<u32> = ids.iter().map(|i| (i - 1) % EVAL_SEEDS).collect();
        let br = |x: &[f64], y: &[f64], k: u64| boot_ratio_clustered(x, y, &clusters, salt ^ k);
        let cold = br(&c4, &c3, 0);
        let setup = br(&s4, &s3, 1);
        let online = br(&o4, &o3, 2);
        let wall = br(&w4, &w3, 3);
        let m3 = get(&p3, &cold_min_units);
        let m4 = get(&p4, &cold_min_units);
        let cold_min = br(&m4, &m3, 6);
        let per_target: Vec<f64> = c4.iter().zip(&c3).map(|(x, y)| x / y).collect();
        let per_seed: Vec<Value> = (0..EVAL_SEEDS)
            .filter_map(|c| {
                let idx: Vec<usize> = (0..ids.len()).filter(|&k| clusters[k] == c).collect();
                if idx.is_empty() {
                    return None;
                }
                let (x, y) = (idx.iter().map(|&k| c4[k]).sum::<f64>(), idx.iter().map(|&k| c3[k]).sum::<f64>());
                Some(json!({"seed": 201 + i64::from(c), "pairs": idx.len(), "cold_v4_over_v3": x / y}))
            })
            .collect();
        // A/A: the second v3 process over the first, on the targets that have both.
        let aa_ids: Vec<u32> = pb.keys().copied().filter(|i| p3.contains_key(i)).collect();
        let aa = if aa_ids.len() >= 2 {
            let x: Vec<f64> = aa_ids
                .iter()
                .map(|i| cold_units(&pb[i].report).unwrap_or(f64::NAN))
                .collect();
            let y: Vec<f64> = aa_ids
                .iter()
                .map(|i| cold_units(&p3[i].report).unwrap_or(f64::NAN))
                .collect();
            let aa_clusters: Vec<u32> = aa_ids.iter().map(|i| (i - 1) % EVAL_SEEDS).collect();
            let r = boot_ratio_clustered(&x, &y, &aa_clusters, salt ^ 4);
            // Amendment A7: the interval must contain 1 (the first text also
            // asked for a point estimate within 3%, which eight pairs cannot
            // promise: their noise is 3%).
            let pass = r.1 <= 1.0 && 1.0 <= r.2;
            json!({"targets": aa_ids.len(), "cold_ratio": r.0, "ci95": [r.1, r.2], "passes": pass})
        } else {
            Value::Null
        };
        let raw_verdict = if cold.2 < 1.0 {
            "IMPROVEMENT"
        } else if cold.1 > 1.0 {
            "REGRESSION"
        } else {
            "NOT_SEPARATED"
        };
        let resolved = aa.is_null() || aa["passes"] == true;
        let verdict = if !resolved {
            "UNRESOLVED"
        } else {
            match raw_verdict {
                "IMPROVEMENT" => improved += 1,
                "REGRESSION" => regressed += 1,
                _ => {}
            }
            raw_verdict
        };
        let pos_ids: Vec<u32> = pb_pos
            .keys()
            .copied()
            .filter(|i| p3.contains_key(i))
            .collect();
        let lev_ids: Vec<u32> = pb_lev
            .keys()
            .copied()
            .filter(|i| p3.contains_key(i))
            .collect();
        let lever_control = if lev_ids.len() >= 2 {
            let x: Vec<f64> = lev_ids
                .iter()
                .map(|i| cold_units(&pb_lev[i].report).unwrap_or(f64::NAN))
                .collect();
            let y: Vec<f64> = lev_ids
                .iter()
                .map(|i| cold_units(&p3[i].report).unwrap_or(f64::NAN))
                .collect();
            let lev_clusters: Vec<u32> = lev_ids.iter().map(|i| (i - 1) % EVAL_SEEDS).collect();
            let r = boot_ratio_clustered(&x, &y, &lev_clusters, salt ^ 7);
            for (a, b) in x.iter().zip(&y) {
                lever_pairs.push((a / b).ln());
            }
            json!({"targets": lev_ids.len(), "cold_ratio": r.0, "ci95": [r.1, r.2]})
        } else {
            Value::Null
        };
        let positive = if pos_ids.len() >= 2 {
            let x: Vec<f64> = pos_ids
                .iter()
                .map(|i| cold_units(&pb_pos[i].report).unwrap_or(f64::NAN))
                .collect();
            let y: Vec<f64> = pos_ids
                .iter()
                .map(|i| cold_units(&p3[i].report).unwrap_or(f64::NAN))
                .collect();
            let pos_clusters: Vec<u32> = pos_ids.iter().map(|i| (i - 1) % EVAL_SEEDS).collect();
            let r = boot_ratio_clustered(&x, &y, &pos_clusters, salt ^ 5);
            json!({"targets": pos_ids.len(), "cold_ratio": r.0, "ci95": [r.1, r.2], "detected": r.1 > 1.05})
        } else {
            Value::Null
        };
        // The reference, priced in each arm's own process.
        let r0 = &p3[&ids[0]].report;
        let r_big = f(r0, &["r"]).unwrap_or(f64::NAN);
        let sqrt_r = r_big.sqrt();
        let step = |procs: &BTreeMap<u32, Proc>| -> Vec<f64> {
            ids.iter()
                .map(|i| {
                    f(&procs[i].report, &["step_prices", "canonical_step_units"])
                        .unwrap_or(f64::NAN)
                })
                .collect()
        };
        let rc3 = get(&p3, &rho_cold_canonical_units);
        let rc4 = get(&p4, &rho_cold_canonical_units);
        let ra3 = get(&p3, &rho_cold_asbuilt_units);
        let ra4 = get(&p4, &rho_cold_asbuilt_units);
        let gap = |x: &[f64], y: &[f64], k: u64| {
            let r = br(x, y, k);
            json!({"value": r.0, "ci95": [r.1, r.2]})
        };
        let upstep = |procs: &BTreeMap<u32, Proc>| -> Vec<f64> {
            ids.iter()
                .map(|i| f(&procs[i].report, &["median", "rho_units_per_step"]).unwrap_or(f64::NAN))
                .collect()
        };
        // The band.
        let band = |w: f64| -> Value {
            let ic3: Vec<f64> = ids
                .iter()
                .map(|i| {
                    let (c, u) = ic_counts(&p3[i].report, n);
                    c + w * u
                })
                .collect();
            let ic4: Vec<f64> = ids
                .iter()
                .map(|i| {
                    let (c, u) = ic_counts(&p4[i].report, n);
                    c + w * u
                })
                .collect();
            let rh3: Vec<f64> = ids
                .iter()
                .map(|i| {
                    let (c, u) = rho_counts(&p3[i].report, n);
                    c + w * u
                })
                .collect();
            let g3 = br(&ic3, &rh3, 10);
            let g4 = br(&ic4, &rh3, 11);
            let rr = br(&ic4, &ic3, 12);
            json!({"ic_v3_gae": mean(&ic3), "ic_v4_gae": mean(&ic4), "rho_gae": mean(&rh3),
                   "gap_v3": g3.0, "gap_v4": g4.0, "gap_v4_ci95": [g4.1, g4.2],
                   "v4_over_v3": rr.0, "v4_over_v3_ci95": [rr.1, rr.2]})
        };
        // The strongest available rho: the walk of `ic price` or the strong
        // fixture (the optimised strong rho), whichever takes fewer units a
        // step; the fixture's ns a step is converted at this arm's unit.
        let strong_ns = strong_ns_per_step(root, a, n);
        let parallel_ns: Vec<f64> = ids
            .iter()
            .filter(|&&i| i <= 8)
            .filter_map(|i| {
                let r = &p3[i].report;
                Some(
                    r["median"]["rho_online_wall_ns"].as_f64()?
                        / r["repetitions"][0]["rho_online"]["steps"].as_f64()?,
                )
            })
            .collect();
        let unit_ns = median(
            &ids.iter()
                .flat_map(|i| {
                    p3[i].report["step_prices"]["unit_ns"]
                        .as_array()
                        .cloned()
                        .unwrap_or_default()
                })
                .filter_map(|x| x.as_f64())
                .collect::<Vec<_>>(),
        );
        let strong_ups = if strong_ns.is_empty() {
            f64::NAN
        } else {
            median(&strong_ns) / unit_ns
        };
        let best = |procs: &BTreeMap<u32, Proc>| -> Vec<f64> {
            ids.iter()
                .map(|i| {
                    let r = &procs[i].report;
                    let steps = r["rho_counts"]["steps"].as_f64().unwrap_or(f64::NAN);
                    let own = f(r, &["median", "rho_units_per_step"]).unwrap_or(f64::NAN);
                    let ups = if strong_ups.is_finite() {
                        own.min(strong_ups)
                    } else {
                        own
                    };
                    f(r, &["median", "rho_setup_units"]).unwrap_or(f64::NAN) + steps * ups
                })
                .collect()
        };
        let rb3 = best(&p3);
        let rb4 = best(&p4);
        let (b0, b1, b2) = (band(0.0), band(0.1), band(1.0));
        let order_holds = [&b0, &b1, &b2].iter().all(|b| {
            b["gap_v4"].as_f64().unwrap_or(0.0) > 1.0
                && b["gap_v3"].as_f64().unwrap_or(0.0) > 1.0
                && b["v4_over_v3_ci95"][1].as_f64().unwrap_or(2.0) < 1.0
        });
        let counts_of = |procs: &BTreeMap<u32, Proc>| -> Value {
            let r = &procs[&ids[0]].report["counts"]["pass"];
            json!({
                "columns": r["select"]["columns"], "points": r["select"]["points"],
                "stored_pairs": r["build"]["stored_pairs"], "tier": r["build"]["tier"],
                "trials": r["collect"]["trials"], "units": r["collect"]["units"],
                "summands_scanned": r["collect"]["summands_scanned"],
                "relations_collected": r["collect"]["relations_collected"],
                "relations_accepted": r["logs"]["relations_accepted"],
                "duplicates": r["logs"]["duplicates"], "rejected": r["logs"]["rejected"],
                "descent_trials_mean": mean(&ids.iter().map(|i| procs[i].report["counts"]["pass"]["descent"]["trials_total"].as_f64().unwrap_or(f64::NAN)).collect::<Vec<_>>()),
                "free_relations": procs[&ids[0]].report["counts"]["pass"]["free_relations"].clone(),
            })
        };
        let s_of = |x: &[f64]| mean(x) / sqrt_r;
        sizes_out.push(json!({
            "curve": slug(a, n), "a": a, "n": n, "r": r_big, "log2_r": f(r0, &["log2_r"]),
            "targets_paired": ids.len(),
            "neighbour_cpu_per_wall": {"median": median(&neigh), "share_over_0_2": neigh.iter().filter(|&&x| x > 0.2).count() as f64 / neigh.len().max(1) as f64},
            "logarithms": {"checked": log_ok, "mismatched": log_bad},
            "paired_v4_over_v3": {
                "cold": {"value": cold.0, "ci95": [cold.1, cold.2]},
                "setup": {"value": setup.0, "ci95": [setup.1, setup.2]},
                "online": {"value": online.0, "ci95": [online.1, online.2]},
                "wall_cold": {"value": wall.0, "ci95": [wall.1, wall.2]},
                "cold_least_repetition": {"value": cold_min.0, "ci95": [cold_min.1, cold_min.2]},
                "per_target_median": median(&per_target),
                "per_seed": per_seed,
                "targets_below_one": per_target.iter().filter(|&&x| x < 1.0).count(),
            },
            "verdict": verdict,
            "verdict_if_resolved": raw_verdict,
            "aa_v3b_over_v3": aa,
            "positive_control_over_v3": positive,
            "lever_control_over_v3": lever_control,
            "s": {
                "ic_cold_v3": s_of(&c3), "ic_cold_v4": s_of(&c4),
                "ic_setup_v3": s_of(&s3), "ic_setup_v4": s_of(&s4),
                "ic_online_v3": s_of(&o3), "ic_online_v4": s_of(&o4),
                "rho_cold_canonical_v3proc": s_of(&rc3), "rho_cold_canonical_v4proc": s_of(&rc4),
                "rho_cold_asbuilt_v3proc": s_of(&ra3), "rho_cold_asbuilt_v4proc": s_of(&ra4),
            },
            "rho_reference_check": {
                "strong_fixture_ns_per_step_median": if strong_ns.is_empty() { Value::Null } else { json!(median(&strong_ns)) },
                "parallel_rho_ns_per_step_median_T01_T08": if parallel_ns.is_empty() { Value::Null } else { json!(median(&parallel_ns)) },
                "parallel_over_strong": if strong_ns.is_empty() || parallel_ns.is_empty() { Value::Null } else { json!(median(&parallel_ns) / median(&strong_ns)) },
                "parallel_rho_admissible": if strong_ns.is_empty() || parallel_ns.is_empty() { Value::Null } else { json!(median(&parallel_ns) <= median(&strong_ns)) },
                "strong_units_per_step": if strong_ups.is_finite() { json!(strong_ups) } else { Value::Null },
                "unit_ns_median": unit_ns,
            },
            "rho": {
                "canonical_step_units_v3proc": mean(&step(&p3)), "canonical_step_units_v4proc": mean(&step(&p4)),
                "units_per_step_as_built_v3proc": mean(&upstep(&p3)), "units_per_step_as_built_v4proc": mean(&upstep(&p4)),
                "steps_mean": mean(&ids.iter().map(|i| p3[i].report["rho_counts"]["steps"].as_f64().unwrap_or(f64::NAN)).collect::<Vec<_>>()),
            },
            "gap_ic_cold_over_rho_cold": {
                "v3_canonical": gap(&c3, &rc3, 20), "v4_canonical": gap(&c4, &rc4, 21),
                "v3_as_built": gap(&c3, &ra3, 22), "v4_as_built": gap(&c4, &ra4, 23),
                "v3_strongest_as_built": gap(&c3, &rb3, 24), "v4_strongest_as_built": gap(&c4, &rb4, 25),
            },
            "band_5a": {"w0": b0, "w0_1": b1, "w1": b2, "order_holds_across_band": order_holds},
            "counts_v3": counts_of(&p3), "counts_v4": counts_of(&p4),
        }));
    }
    let n_sizes = sizes_out.len();
    // The lever control, pooled: the geometric mean of the per-pair ratios
    // over all sizes, a bootstrap over pairs.
    let lever_pooled = if lever_pairs.len() >= 8 {
        let n = lever_pairs.len();
        let gm = (lever_pairs.iter().sum::<f64>() / n as f64).exp();
        let mut rng = Rng(BOOT_SEED ^ 0xA6);
        let mut draws: Vec<f64> = (0..BOOT)
            .map(|_| ((0..n).map(|_| lever_pairs[rng.below(n)]).sum::<f64>() / n as f64).exp())
            .collect();
        draws.sort_by(f64::total_cmp);
        let q = |p: f64| draws[((p * (BOOT as f64 - 1.0)).round() as usize).min(BOOT - 1)];
        let above = sizes_out
            .iter()
            .filter(|s| {
                s["lever_control_over_v3"]["cold_ratio"]
                    .as_f64()
                    .is_some_and(|r| r > 1.0)
            })
            .count();
        json!({"pairs": n, "geometric_mean_ratio": gm, "ci95": [q(0.025), q(0.975)], "sizes_above_one": above,
               "detector_demonstrated": q(0.025) > 1.0 && above >= 5})
    } else {
        Value::Null
    };
    let outcome = if !incomplete.is_empty() {
        "INCOMPLETE"
    } else if improved >= 4 {
        "BASELINE_IMPROVED"
    } else if improved >= 1 {
        "PARTIAL"
    } else if regressed >= 1 {
        "NEGATIVE"
    } else {
        "NULL"
    };
    let doc = json!({
        "what_this_is": "Rule v4: v4 recipe over v3 recipe, paired on the same targets, interleaved; cold units = set-up + the index calculus's online interval, each in its own process's batched-addition unit.",
        "decision_rule": "improvement at a size when the 95% bootstrap interval of the paired cold ratio lies below 1; baseline improvement when at least 4 of 6 sizes improve",
        "sizes": sizes_out,
        "sizes_improved": improved, "sizes_regressed": regressed, "sizes_read": n_sizes,
        "incomplete_sizes": incomplete, "outcome": outcome,
        "lever_control_pooled": lever_pooled,
    });
    write_json(&root.join("analysis.json"), &doc)?;
    for s in doc["sizes"].as_array().unwrap() {
        println!(
            "{} n={} v4/v3 cold {:.4} [{:.4}, {:.4}] {} | gap v4 canonical {:.2} | aa {}",
            s["curve"].as_str().unwrap(),
            s["n"],
            s["paired_v4_over_v3"]["cold"]["value"].as_f64().unwrap(),
            s["paired_v4_over_v3"]["cold"]["ci95"][0].as_f64().unwrap(),
            s["paired_v4_over_v3"]["cold"]["ci95"][1].as_f64().unwrap(),
            s["verdict"].as_str().unwrap(),
            s["gap_ic_cold_over_rho_cold"]["v4_canonical"]["value"]
                .as_f64()
                .unwrap(),
            s["aa_v3b_over_v3"]["cold_ratio"]
        );
    }
    println!("outcome: {outcome} ({improved} of {n_sizes} sizes improved)");
    Ok(0)
}

// ── table ───────────────────────────────────────────────────────────

fn table(root: &Path) -> Result<i32, String> {
    let a = read_json(&root.join("analysis.json"))?;
    let mut sizes: Vec<&Value> = a["sizes"].as_array().ok_or("no sizes")?.iter().collect();
    sizes.sort_by(|x, y| {
        x["log2_r"]
            .as_f64()
            .unwrap()
            .total_cmp(&y["log2_r"].as_f64().unwrap())
    });
    let ci = |v: &Value, k: usize| {
        format!(
            "{:.k$} [{:.k$}, {:.k$}]",
            v["value"].as_f64().unwrap_or(f64::NAN),
            v["ci95"][0].as_f64().unwrap_or(f64::NAN),
            v["ci95"][1].as_f64().unwrap_or(f64::NAN)
        )
    };
    println!("| n | log2 r | pairs | **cold, v4 / v3 [95%]** | verdict | set-up | online | cold, least repetition | wall | A/A (v3 again / v3) | lever control / v3 | Table A instance (seed 201) |");
    println!("|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|");
    for s in &sizes {
        let p = &s["paired_v4_over_v3"];
        let aa = &s["aa_v3b_over_v3"];
        let lc = &s["lever_control_over_v3"];
        let seed201 = p["per_seed"]
            .as_array()
            .and_then(|v| v.iter().find(|x| x["seed"] == 201))
            .map(|x| {
                format!(
                    "{:.3} ({} pairs)",
                    x["cold_v4_over_v3"].as_f64().unwrap(),
                    x["pairs"]
                )
            })
            .unwrap_or_default();
        println!(
            "| {} | {:.1} | {} | **{}** | {} | {} | {} | {} | {} | {} | {} | {} |",
            s["n"],
            s["log2_r"].as_f64().unwrap(),
            s["targets_paired"],
            ci(&p["cold"], 3),
            s["verdict"].as_str().unwrap(),
            ci(&p["setup"], 3),
            ci(&p["online"], 2),
            ci(&p["cold_least_repetition"], 3),
            ci(&p["wall_cold"], 3),
            if aa.is_null() {
                "-".to_string()
            } else {
                format!(
                    "{} ({} pairs)",
                    ci(&json!({"value": aa["cold_ratio"], "ci95": aa["ci95"]}), 3),
                    aa["targets"]
                )
            },
            if lc.is_null() {
                "-".to_string()
            } else {
                ci(&json!({"value": lc["cold_ratio"], "ci95": lc["ci95"]}), 3)
            },
            seed201
        );
    }
    println!();
    println!("| n | S_IC cold v3 | S_IC cold v4 | S_IC set-up v3 / v4 | S_IC online v3 / v4 | S_rho cold, canonical step | S_rho cold, as built | canonical step (units) | rho units / step as built | **gap v3, canonical** | **gap v4, canonical [95%]** | gap v3, as built | gap v4, as built | gap v4, strongest rho |");
    println!("|--:|--:|--:|:--|:--|--:|--:|--:|--:|--:|:--|--:|--:|--:|");
    for s in &sizes {
        let (sv, g, r) = (&s["s"], &s["gap_ic_cold_over_rho_cold"], &s["rho"]);
        let f = |v: &Value| v.as_f64().unwrap_or(f64::NAN);
        println!("| {} | {:.3} | {:.3} | {:.3} / {:.3} | {:.4} / {:.4} | {:.4} | {:.4} | {:.2} | {:.1} | **{:.2}** | **{}** | {:.2} | {:.2} | {:.2} |",
            s["n"], f(&sv["ic_cold_v3"]), f(&sv["ic_cold_v4"]), f(&sv["ic_setup_v3"]), f(&sv["ic_setup_v4"]), f(&sv["ic_online_v3"]), f(&sv["ic_online_v4"]),
            f(&sv["rho_cold_canonical_v4proc"]), f(&sv["rho_cold_asbuilt_v4proc"]), f(&r["canonical_step_units_v4proc"]), f(&r["units_per_step_as_built_v4proc"]),
            f(&g["v3_canonical"]["value"]), ci(&g["v4_canonical"], 2), f(&g["v3_as_built"]["value"]), f(&g["v4_as_built"]["value"]), f(&g["v4_strongest_as_built"]["value"]));
    }
    println!();
    println!("| n | ParallelRho ns/step (T01-T08) | strong fixture ns/step | parallel / strong | admissible |");
    println!("|--:|--:|--:|--:|:--|");
    for s in &sizes {
        let c = &s["rho_reference_check"];
        println!(
            "| {} | {} | {} | {} | {} |",
            s["n"],
            c["parallel_rho_ns_per_step_median_T01_T08"]
                .as_f64()
                .map_or("-".into(), |x| format!("{x:.1}")),
            c["strong_fixture_ns_per_step_median"]
                .as_f64()
                .map_or("-".into(), |x| format!("{x:.1}")),
            c["parallel_over_strong"]
                .as_f64()
                .map_or("-".into(), |x| format!("{x:.2}")),
            c["parallel_rho_admissible"]
        );
    }
    println!();
    println!("| n | gap v4 (w = 0 / 0.1 / 1) | v4 / v3 counted (w = 0 / 0.1 / 1) | order holds across the band |");
    println!("|--:|:--|:--|:--|");
    for s in &sizes {
        let b = &s["band_5a"];
        let g = |w: &str, k: &str| b[w][k].as_f64().unwrap_or(f64::NAN);
        let rr = |w: &str| {
            format!(
                "{:.3} [{:.3}, {:.3}]",
                g(w, "v4_over_v3"),
                b[w]["v4_over_v3_ci95"][0].as_f64().unwrap_or(f64::NAN),
                b[w]["v4_over_v3_ci95"][1].as_f64().unwrap_or(f64::NAN)
            )
        };
        println!(
            "| {} | {:.2} / {:.2} / {:.2} | {} / {} / {} | {} |",
            s["n"],
            g("w0", "gap_v4"),
            g("w0_1", "gap_v4"),
            g("w1", "gap_v4"),
            rr("w0"),
            rr("w0_1"),
            rr("w1"),
            b["order_holds_across_band"]
        );
    }
    println!();
    println!("| n | columns v3 / v4 | window v3 / v4 | unit trials v3 / v4 | collection trials v3 / v4 | set-up units (M) v3 / v4 |");
    println!("|--:|:--|:--|:--|:--|:--|");
    for s in &sizes {
        let (c3, c4) = (&s["counts_v3"], &s["counts_v4"]);
        println!(
            "| {} | {} / {} | - | - | {} / {} | {:.2} / {:.2} |",
            s["n"],
            c3["columns"],
            c4["columns"],
            c3["trials"],
            c4["trials"],
            s["s"]["ic_setup_v3"].as_f64().unwrap() * s["r"].as_f64().unwrap().sqrt() / 1e6,
            s["s"]["ic_setup_v4"].as_f64().unwrap() * s["r"].as_f64().unwrap().sqrt() / 1e6
        );
    }
    println!();
    println!(
        "outcome: {} ({} improved, {} regressed of {})",
        a["outcome"].as_str().unwrap(),
        a["sizes_improved"],
        a["sizes_regressed"],
        a["sizes_read"]
    );
    if !a["lever_control_pooled"].is_null() {
        println!("lever control pooled: {}", a["lever_control_pooled"]);
    }
    Ok(0)
}

// ── replay ──────────────────────────────────────────────────────────

fn to_j(v: &Value) -> Result<json::J, String> {
    json::parse(&serde_json::to_string(v).map_err(|e| e.to_string())?)
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(crypto_lib::hash::sha256::sha256(bytes))
}

fn replay_one(path: &Path, v3: &Path, a: u8, n: u32, i: u32) -> Result<Value, String> {
    let bytes = fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let r: Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let fixture = to_j(&r["identity_inputs"]["fixture"])?;
    let curve = oracle::Curve::new(&fixture)?;
    let tj = to_j(&r["identity_inputs"]["fixture"]["targets"][0])?;
    let target = curve
        .decode(&tj)?
        .ok_or("target is the point at infinity")?;
    let mut arms = serde_json::Map::new();
    for arm in ["ic", "rho"] {
        let cert = &r["certificates"][arm];
        let scalar: u128 = cert["scalar"]
            .as_str()
            .ok_or("no scalar")?
            .parse()
            .map_err(|_| "bad scalar")?;
        let same_target = cert["target"] == r["identity_inputs"]["fixture"]["targets"][0];
        let in_range = scalar > 0 && scalar < u128::from(curve.r);
        let ok = in_range && same_target && curve.mul(Some(curve.g), scalar) == Some(target);
        let digest_ok = r["certificates"][format!("{arm}_sha256")].as_str()
            == Some(
                sha256_hex(
                    serde_json::to_string(cert)
                        .map_err(|e| e.to_string())?
                        .as_bytes(),
                )
                .as_str(),
            );
        arms.insert(
            arm.to_string(),
            json!({"scalar": scalar.to_string(), "verified": ok, "digest_matches": digest_ok}),
        );
    }
    let want = v3_scalar(v3, a, n, i);
    let same_as_v3 = want.is_none_or(|w| arms["ic"]["scalar"].as_str() == Some(w.as_str()));
    let v3_target = {
        let p = v3.join(format!("runs/{}/T{i:02}-R1.price.json", slug(a, n)));
        read_json(&p).ok().map(|v| v["target"]["point"].clone())
    };
    let same_target_as_v3 = v3_target.is_none_or(|t| t == r["target"]["point"]);
    let ok = arms
        .values()
        .all(|x| x["verified"] == true && x["digest_matches"] == true)
        && arms["ic"]["scalar"] == arms["rho"]["scalar"]
        && same_as_v3
        && same_target_as_v3;
    Ok(json!({
        "file_sha256": sha256_hex(&bytes), "arms": arms,
        "logarithm_is_rule_v3s": same_as_v3, "target_is_rule_v3s": same_target_as_v3, "ok": ok,
    }))
}

fn replay(
    root: &Path,
    v3: &Path,
    runs: &str,
    rerun: u32,
    ic: Option<&Path>,
    recipe: Option<&Path>,
) -> Result<i32, String> {
    let mut entries = Vec::new();
    let mut failures = 0usize;
    for (a, n) in SIZES {
        let dir = root.join(runs).join(slug(a, n));
        let Ok(rd) = fs::read_dir(&dir) else { continue };
        let mut names: Vec<String> = rd
            .filter_map(Result::ok)
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|s| s.ends_with(".price.json"))
            .collect();
        names.sort();
        for name in names {
            let Some(i) = name
                .strip_prefix('T')
                .and_then(|s| s.get(..2))
                .and_then(|s| s.parse::<u32>().ok())
            else {
                continue;
            };
            let rel = format!("{}/{}/{}", runs, slug(a, n), name);
            match replay_one(&dir.join(&name), v3, a, n, i) {
                Ok(mut e) => {
                    if e["ok"] != true {
                        failures += 1;
                    }
                    e["path"] = json!(rel);
                    entries.push(e);
                }
                Err(err) => {
                    failures += 1;
                    entries.push(json!({"path": rel, "ok": false, "error": err}));
                }
            }
        }
    }
    // A sample re-run, untimed: a fresh `ic price` of each arm on targets
    // drawn from the panel, whose recovered scalars must be the recorded ones.
    let mut reruns = Vec::new();
    if rerun > 0 {
        let ic = ic.ok_or("--rerun needs --ic")?;
        let recipe = read_json(recipe.ok_or("--rerun needs --recipe")?)?;
        let tmp = root.join("replay-rerun");
        fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
        for (a, n) in SIZES {
            let sl = slug(a, n);
            let dir = root.join(runs).join(&sl);
            for j in 0..rerun {
                // Targets 1, then the last one present, spread by j.
                let have: Vec<u32> = arm_procs(&dir, "v4").keys().copied().collect();
                if have.is_empty() {
                    continue;
                }
                let i = have[if rerun <= 1 {
                    0
                } else {
                    j as usize * (have.len() - 1) / (rerun as usize - 1)
                }];
                for arm in ["v3", "v4"] {
                    let params = params_for(v3, &recipe, arm, a, n, i)?;
                    let pp = tmp.join(format!("{sl}-T{i:02}-{arm}.params.json"));
                    write_json(&pp, &params)?;
                    let out = tmp.join(format!("{sl}-T{i:02}-{arm}.price.json"));
                    let mut c = Command::new(ic);
                    c.args(["price", "--params"])
                        .arg(&pp)
                        .args([
                            "--json",
                            "--repeats",
                            "1",
                            "--repeats-fast",
                            "1",
                            "--single-target",
                            "--rho-seed",
                        ])
                        .arg((RHO_SEED_BASE + i64::from(i)).to_string())
                        .arg("--out")
                        .arg(&out)
                        .env("RAYON_NUM_THREADS", "1")
                        .stdout(Stdio::null())
                        .stderr(Stdio::null());
                    for (k, v) in arm_env(&recipe, arm, &sl) {
                        c.env(k, v);
                    }
                    let st = c.status().map_err(|e| e.to_string())?;
                    let fresh = read_json(&out).ok();
                    let recorded = first_clean(&dir, i, arm).map(|p| p.report);
                    let scalar = |v: &Option<Value>| {
                        v.as_ref().and_then(|x| {
                            x["certificates"]["ic"]["scalar"]
                                .as_str()
                                .map(str::to_string)
                        })
                    };
                    let ok = st.success()
                        && scalar(&fresh).is_some()
                        && scalar(&fresh) == scalar(&recorded)
                        && fresh.as_ref().is_some_and(|x| x["all_verified"] == true);
                    if !ok {
                        failures += 1;
                    }
                    reruns.push(json!({"size": sl, "target": i, "arm": arm, "fresh_scalar": scalar(&fresh), "recorded_scalar": scalar(&recorded), "ok": ok}));
                }
            }
        }
    }
    let total = entries.len();
    let doc = json!({
        "schema": "replay-certificate/v1",
        "status": if failures == 0 && total > 0 { "verified" } else { "failed" },
        "independent": true,
        "scalars_verified": failures == 0 && total > 0,
        "checker": "ic_rule_v4 replay: icprog's oracle (its own GF(2^n) arithmetic, shift-and-add, curve and group order re-derived from the Frobenius trace), a separate process from ic; certificates re-hashed; scalars compared with rule v3's",
        "runs": entries,
        "process_count": total,
        "failures": failures,
        "untimed_reruns": reruns,
    });
    write_json(&root.join("replay-certificate.json"), &doc)?;
    println!(
        "replayed {total} processes, {failures} failures, {} re-runs",
        reruns.len()
    );
    Ok(i32::from(failures != 0))
}

// ── reference ───────────────────────────────────────────────────────

fn reference(
    fixture: &Path,
    isolate: &Path,
    root: &Path,
    targets: u32,
    max_minutes: f64,
) -> Result<i32, String> {
    let started = Instant::now();
    let dir = root.join("runs/reference");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    for (a, n) in SIZES {
        for i in 1..=targets {
            let out = dir.join(format!("{}-T{i:02}.strong.json", slug(a, n)));
            let rec = out.with_extension("").with_extension("isolation.jsonl");
            if out.exists() && record_clean(&rec) {
                continue;
            }
            if started.elapsed().as_secs_f64() > max_minutes * 60.0 {
                println!("time limit reached; resumable");
                return Ok(3);
            }
            let cmd = vec![
                fixture.to_string_lossy().into_owned(),
                n.to_string(),
                a.to_string(),
                "signed_frobenius".into(),
                "1".into(),
                "strong".into(),
                format!("{}", 0x230000 + i64::from(i)),
                format!("hash:{}", TARGET_SEED_BASE + i64::from(i)),
            ];
            // The fixture writes its row to standard output.
            let env = vec![("RAYON_NUM_THREADS".to_string(), "1".to_string())];
            let rec_path = rec.clone();
            let _ = rec_path;
            // Capture stdout through a shell-free redirect.
            let label = format!("reference/{}-T{i:02}", slug(a, n));
            let stdout = fs::File::create(&out).map_err(|e| e.to_string())?;
            let stderr = fs::File::create(out.with_extension("").with_extension("stderr"))
                .map_err(|e| e.to_string())?;
            wait_for_quiet();
            let mut c = Command::new(isolate);
            c.args(["run", "--wait", "--cpus", CPUS, "--out"])
                .arg(&rec)
                .args(["--label", &label, "--"])
                .args(&cmd)
                .stdout(Stdio::from(stdout))
                .stderr(Stdio::from(stderr));
            for (k, v) in &env {
                c.env(k, v);
            }
            c.status().map_err(|e| e.to_string())?;
            println!(
                "{label}: {}",
                if record_clean(&rec) {
                    "clean"
                } else {
                    "not clean"
                }
            );
        }
    }
    Ok(0)
}
