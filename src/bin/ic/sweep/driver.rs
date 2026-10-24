//! The study loop: ask the sampler, evaluate on the rung ladder, prune, tell.
//!
//! ```text
//!  baselines ─► for each trial:  ask(TPE) ─► rung 1 ─► rung 2 ─► … ─► project to target
//!                                              │           │
//!                                        median prune  median prune           ─► store
//! ```
//!
//! A rung is a size.  Each is evaluated by a separate `ic sweep-trial`
//! process (counts are single-threaded on purpose; parallelism is across
//! trials), so a crash, a hang or an out-of-memory kill costs one trial and
//! is recorded as one.  The frozen baseline is scored on every rung before
//! the first trial, and a rung it cannot be scored on (it exhausted, was
//! refused) is dropped from the study: there is nothing to divide by.
//!
//! Pruning is Optuna's median rule: a trial is stopped after a rung if its
//! `log2 ratio` there is worse than the median of the other trials that have
//! reported that rung, once enough have.  Pruned and failed trials stay in
//! the sampler's history, ranked behind the complete ones.

use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde_json::Value;

use super::objective::{project, worst_ratio};
use super::space::{Assignment, Space};
use super::store::{BaselineLine, RungLine, Store, TrialLine, TrialState};
use super::tpe::{suggest, Obs, Rng, State, TpeOptions};
use super::trial::{solver_unit, Class, TrialConfig, TrialRecord, SCHEMA};

/// Runs one configuration and says what happened.
pub trait Evaluator: Sync {
    fn evaluate(&self, config: &TrialConfig) -> TrialRecord;
}

/// Runs each trial in a child `ic sweep-trial` process.
pub struct ProcessEvaluator {
    pub exe: PathBuf,
    /// Per-trial wall-clock safety net, seconds; 0 for none.
    pub timeout_seconds: u64,
    /// Per-trial peak-memory safety net, MiB; 0 for none.
    pub max_rss_mb: u64,
}

fn crashed(config: &TrialConfig, detail: String) -> TrialRecord {
    TrialRecord {
        schema: SCHEMA.into(),
        class: Class::Crashed,
        detail,
        config: config.clone(),
        instance: None,
        log2_r: None,
        repeats_run: 0,
        repeats_verified: 0,
        band: None,
        context: Value::Null,
    }
}

fn tail(text: &str, n: usize) -> String {
    let t = text.trim();
    t.chars()
        .rev()
        .take(n)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

impl Evaluator for ProcessEvaluator {
    fn evaluate(&self, config: &TrialConfig) -> TrialRecord {
        let json = match serde_json::to_string(config) {
            Ok(j) => j,
            Err(e) => return crashed(config, format!("cannot encode the configuration: {e}")),
        };
        let mut child = match Command::new(&self.exe)
            .args(["--json", "sweep-trial", "--config-json", &json])
            .args(["--timeout-seconds", &self.timeout_seconds.to_string()])
            .args(["--max-rss-mb", &self.max_rss_mb.to_string()])
            .env("RAYON_NUM_THREADS", "1")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => return crashed(config, format!("cannot start {}: {e}", self.exe.display())),
        };
        // Drain both pipes on their own threads, or a chatty child blocks.
        let drain = |mut r: Box<dyn Read + Send>| {
            std::thread::spawn(move || {
                let mut s = String::new();
                let _ = r.read_to_string(&mut s);
                s
            })
        };
        let out = drain(Box::new(child.stdout.take().expect("piped")));
        let err = drain(Box::new(child.stderr.take().expect("piped")));
        // The child enforces its own limits; this is the backstop for a child
        // that cannot (it hung before arming them, or was stopped).
        let hard =
            (self.timeout_seconds > 0).then(|| Duration::from_secs(self.timeout_seconds + 60));
        let started = Instant::now();
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) => {}
                Err(e) => return crashed(config, format!("cannot wait for the trial: {e}")),
            }
            if hard.is_some_and(|h| started.elapsed() > h) {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        let stdout = out.join().unwrap_or_default();
        let stderr = err.join().unwrap_or_default();
        let Some(status) = status else {
            let mut r = crashed(config, "the trial did not stop and was killed".into());
            r.class = Class::Exhausted;
            return r;
        };
        match serde_json::from_str::<Value>(stdout.trim())
            .ok()
            .and_then(|v| v.get("trial").cloned())
            .and_then(|t| serde_json::from_value::<TrialRecord>(t).ok())
        {
            Some(record) => record,
            None => crashed(
                config,
                format!(
                    "the trial process ended ({status}) without a record; stderr ends: {}",
                    tail(&stderr, 400)
                ),
            ),
        }
    }
}

/// Everything a study is told, apart from the space and the store.
pub struct Study {
    /// Sizes, ascending: the field degree (binary) or subgroup bits (prime).
    pub rungs: Vec<u32>,
    pub target_log2_r: f64,
    pub min_fit: usize,
    /// Trials the finished study contains in total (a resumed study runs
    /// only the missing ones).
    pub trials: usize,
    pub workers: usize,
    pub seed: u64,
    pub repeats: usize,
    pub max_trials: u64,
    pub rho_runs: usize,
    /// Other trials that must have reported a rung before it prunes.
    pub prune_startup: usize,
    pub tpe: TpeOptions,
}

impl Study {
    fn stamp(&self, mut cfg: TrialConfig) -> TrialConfig {
        cfg.repeats = self.repeats;
        cfg.seed = self.seed;
        cfg.max_trials = self.max_trials;
        cfg.rho_runs = self.rho_runs;
        cfg
    }
}

#[derive(Debug, Default)]
pub struct Summary {
    pub complete: usize,
    pub pruned: usize,
    pub failed: usize,
    pub ran_now: usize,
    /// Rungs the baseline could be scored on.
    pub usable_rungs: Vec<u32>,
    pub best: Option<TrialLine>,
}

fn median(values: &[f64]) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

struct Shared<'s> {
    store: &'s mut Store,
    history: Vec<Obs>,
    pending: Vec<(usize, Assignment)>,
    next_id: usize,
    started: usize,
    rng: Rng,
    /// `log2 ratio` reported at each rung by every trial that got there.
    rung_values: Vec<Vec<(usize, f64)>>,
    error: Option<String>,
}

fn to_obs(t: &TrialLine) -> Obs {
    let last = t.rungs.iter().rev().find_map(|r| r.log2_ratio);
    match t.state {
        TrialState::Complete => Obs {
            assignment: t.assignment.clone(),
            state: State::Complete,
            value: t.objective.unwrap_or(f64::INFINITY),
            rungs_done: t.rungs.len(),
        },
        TrialState::Pruned => Obs {
            assignment: t.assignment.clone(),
            state: State::Pruned,
            value: last.unwrap_or(f64::INFINITY),
            rungs_done: t.rungs.len(),
        },
        TrialState::Failed => Obs {
            assignment: t.assignment.clone(),
            state: State::Failed,
            value: f64::INFINITY,
            rungs_done: t.rungs.len().saturating_sub(1),
        },
    }
}

/// Score the frozen baseline on every rung (or reuse what the store has).
fn score_baselines(
    space: &Space,
    study: &Study,
    store: &mut Store,
    eval: &dyn Evaluator,
    log: &dyn Fn(&str),
) -> Result<Vec<(u32, TrialRecord)>, String> {
    let mut out = Vec::new();
    for &scale in &study.rungs {
        let cached = store.baselines.iter().find(|b| b.scale == scale).cloned();
        let line = match cached {
            Some(b) => b,
            None => {
                let cfg = study.stamp(space.baseline(scale));
                log(&format!(
                    "baseline at {scale}: {} + {}",
                    cfg.factor_base, cfg.oracle
                ));
                let record = eval.evaluate(&cfg);
                let line = BaselineLine { scale, record };
                store.append_baseline(line.clone())?;
                line
            }
        };
        if line.record.class == Class::Ok && line.record.band.is_some() {
            out.push((scale, line.record));
        } else {
            log(&format!(
                "baseline at {scale} is {}: {} — the rung is dropped",
                line.record.class.as_str(),
                line.record.detail
            ));
        }
    }
    Ok(out)
}

/// Run (or resume) a study.  `log` receives one line per event.
pub fn run_study(
    space: &Space,
    study: &Study,
    store: &mut Store,
    eval: &dyn Evaluator,
    log: &(dyn Fn(&str) + Sync),
) -> Result<Summary, String> {
    if study.rungs.is_empty() || study.rungs.windows(2).any(|w| w[0] >= w[1]) {
        return Err("rungs must be a non-empty, strictly ascending list".into());
    }
    let baselines = score_baselines(space, study, store, eval, log)?;
    if baselines.len() < 2 {
        return Err(format!(
            "the baseline could be scored on {} rung(s); a study needs at least 2 \
             (see the baseline lines in {})",
            baselines.len(),
            store.path().display()
        ));
    }
    let usable: Vec<u32> = baselines.iter().map(|b| b.0).collect();
    let history: Vec<Obs> = store.trials.iter().map(to_obs).collect();
    let mut rung_values: Vec<Vec<(usize, f64)>> = vec![Vec::new(); usable.len()];
    for t in &store.trials {
        for r in &t.rungs {
            if let (Some(k), Some(v)) = (usable.iter().position(|&s| s == r.scale), r.log2_ratio) {
                rung_values[k].push((t.id, v));
            }
        }
    }
    let already = store.trials.len();
    let next_id = store.next_id();
    let shared = Mutex::new(Shared {
        store,
        history,
        pending: Vec::new(),
        next_id,
        started: already,
        rng: Rng::new(study.seed ^ 0x7A5B_1E55 ^ (already as u64).wrapping_mul(0x9E37_79B9)),
        rung_values,
        error: None,
    });
    let ran = std::sync::atomic::AtomicUsize::new(0);

    let worker = || loop {
        // Ask.
        let (id, assignment) = {
            let mut g = shared.lock().unwrap();
            if g.error.is_some() || g.started >= study.trials {
                return;
            }
            g.started += 1;
            let id = g.next_id;
            g.next_id += 1;
            // Pending trials are lied about as failed, so a parallel study
            // does not propose the same region twice.
            let mut hist = g.history.clone();
            hist.extend(g.pending.iter().map(|(_, a)| Obs {
                assignment: a.clone(),
                state: State::Failed,
                value: f64::INFINITY,
                rungs_done: 0,
            }));
            let g = &mut *g;
            let a = suggest(space, &hist, &mut g.rng, &study.tpe);
            g.pending.push((id, a.clone()));
            (id, a)
        };

        // Evaluate on the ladder.
        let mut rungs: Vec<RungLine> = Vec::new();
        let mut points: Vec<(f64, f64)> = Vec::new();
        let mut state = TrialState::Complete;
        let mut note = String::new();
        for (k, (scale, base)) in baselines.iter().enumerate() {
            let cfg = match space.build(&assignment, *scale) {
                Ok(c) => study.stamp(c),
                Err(e) => {
                    state = TrialState::Failed;
                    note = format!("rung {scale}: {e}");
                    rungs.push(RungLine {
                        scale: *scale,
                        class: Class::InvalidConfig,
                        detail: e,
                        log2_r: None,
                        band: None,
                        ratio: None,
                        log2_ratio: None,
                        op_unit: None,
                        rankable: true,
                        other_native: Value::Null,
                    });
                    break;
                }
            };
            let rec = eval.evaluate(&cfg);
            // A score is comparable with the baseline's only where the
            // solver counts in the same unit.  Otherwise the ratio measures
            // which engine counts coarsely (AGENTS.md §6: changing the unit),
            // and a search would learn to pick it.  Record the rung in full,
            // rank nothing from it.
            let (unit, base_unit) = (solver_unit(&rec), solver_unit(base));
            let rankable = unit == base_unit;
            let scored = match (&rec.class, &rec.band, &base.band) {
                (Class::Ok, Some(b), Some(bb)) if rankable => worst_ratio(b, bb).map(Some),
                _ => Ok(None),
            };
            let ratio = match scored {
                Ok(r) => r,
                Err(e) => {
                    state = TrialState::Failed;
                    note = format!("rung {scale}: {e}");
                    None
                }
            };
            let log2_ratio = ratio.map(f64::log2);
            let mut detail = rec.detail.clone();
            if rec.class == Class::Ok && !rankable {
                detail = format!(
                    "not rankable: solver counts in `{}`, the baseline in `{}`",
                    unit.as_deref().unwrap_or("none"),
                    base_unit.as_deref().unwrap_or("none")
                );
                state = TrialState::Failed;
                note = format!("rung {scale}: {detail}");
            }
            rungs.push(RungLine {
                scale: *scale,
                class: rec.class,
                detail,
                log2_r: rec.log2_r,
                band: rec.band.clone(),
                ratio,
                log2_ratio,
                op_unit: unit,
                rankable,
                other_native: rec
                    .context
                    .get("other_native_unscored")
                    .cloned()
                    .unwrap_or(Value::Null),
            });
            let (Some(log2_ratio), Some(log2_r)) = (log2_ratio, rec.log2_r) else {
                state = TrialState::Failed;
                if note.is_empty() {
                    note = format!("rung {scale}: {} — {}", rec.class.as_str(), rec.detail);
                }
                break;
            };
            points.push((log2_r, log2_ratio));
            // Report to the pruner, and ask it.
            let prune = {
                let mut g = shared.lock().unwrap();
                let others: Vec<f64> = g.rung_values[k]
                    .iter()
                    .filter(|(tid, _)| *tid != id)
                    .map(|(_, v)| *v)
                    .collect();
                g.rung_values[k].push((id, log2_ratio));
                k + 1 < baselines.len()
                    && others.len() >= study.prune_startup
                    && log2_ratio > median(&others)
            };
            if prune {
                state = TrialState::Pruned;
                note = format!("worse than the median at rung {scale}");
                break;
            }
        }

        // Tell.
        let (objective, projected, slope) = if state == TrialState::Complete {
            match project(&points, study.target_log2_r, study.min_fit) {
                Some(p) => (Some(p.value), p.projected, p.slope),
                None => (None, false, None),
            }
        } else {
            (None, false, None)
        };
        let line = TrialLine {
            id,
            assignment: assignment.clone(),
            state,
            rungs,
            objective,
            projected,
            slope,
            note,
        };
        let mut g = shared.lock().unwrap();
        g.pending.retain(|(pid, _)| *pid != id);
        g.history.push(to_obs(&line));
        if let Err(e) = g.store.append_trial(line.clone()) {
            g.error = Some(e);
            return;
        }
        ran.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        log(&format!(
            "trial {id}: {:?}{}{}",
            line.state,
            line.objective.map_or(String::new(), |o| format!(
                ", log2 ratio at target {o:+.3}{}",
                if line.projected {
                    " (projected)"
                } else {
                    " (last rung, not projected)"
                }
            )),
            if line.note.is_empty() {
                String::new()
            } else {
                format!(" — {}", line.note)
            }
        ));
    };

    std::thread::scope(|scope| {
        for _ in 0..study.workers.max(1) {
            scope.spawn(worker);
        }
    });

    let g = shared.into_inner().unwrap();
    if let Some(e) = g.error {
        return Err(e);
    }
    let trials = &g.store.trials;
    let best = trials
        .iter()
        .filter(|t| t.state == TrialState::Complete)
        .min_by(|a, b| {
            a.objective
                .unwrap_or(f64::INFINITY)
                .total_cmp(&b.objective.unwrap_or(f64::INFINITY))
        })
        .cloned();
    Ok(Summary {
        complete: trials
            .iter()
            .filter(|t| t.state == TrialState::Complete)
            .count(),
        pruned: trials
            .iter()
            .filter(|t| t.state == TrialState::Pruned)
            .count(),
        failed: trials
            .iter()
            .filter(|t| t.state == TrialState::Failed)
            .count(),
        ran_now: ran.load(std::sync::atomic::Ordering::SeqCst),
        usable_rungs: usable,
        best,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sweep::space::{space_for, InstanceKind};
    use crate::sweep::store::{Header, SCHEMA as STORE_SCHEMA};
    use crate::sweep::trial::Band;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn instance() -> InstanceKind {
        InstanceKind {
            regime: "koblitz".into(),
            koblitz_a: Some(0),
            family: None,
            max_cofactor: None,
        }
    }

    /// A stand-in pipeline whose cost is a known function of the
    /// configuration, so the study loop can be tested without running any
    /// cryptography.  Crossbred is genuinely cheaper; SAT cannot finish on
    /// the larger rungs; everything else is near the baseline.
    struct Mock {
        calls: AtomicUsize,
    }

    fn field<'a>(s: &'a str, key: &str) -> Option<&'a str> {
        s.split(|c| c == ':' || c == ',')
            .find_map(|kv| kv.strip_prefix(&format!("{key}=")))
    }

    impl Evaluator for Mock {
        fn evaluate(&self, cfg: &TrialConfig) -> TrialRecord {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let n = cfg.degree as f64;
            let solver = cfg.solver.clone().unwrap_or_default();
            let mut rec = TrialRecord {
                schema: SCHEMA.into(),
                class: Class::Ok,
                detail: String::new(),
                config: cfg.clone(),
                instance: Some("mock".into()),
                log2_r: Some(n - 2.0),
                repeats_run: cfg.repeats,
                repeats_verified: cfg.repeats,
                band: None,
                context: Value::Null,
            };
            if solver.starts_with("sat-cdcl") && cfg.degree >= 19 {
                rec.class = Class::Exhausted;
                rec.detail = "conflict budget".into();
                return rec;
            }
            let dim: f64 = field(&cfg.factor_base, "dimension")
                .unwrap()
                .parse()
                .unwrap();
            let m: f64 = field(&cfg.oracle, "m").unwrap().parse().unwrap();
            let ideal = n / 3.0;
            // `matrix-f5` is genuinely cheaper and in the baseline's unit.
            // `crossbred-f2` and `buchberger-f2` look far cheaper, but count
            // in other units: a ratio against the word-XOR baseline would
            // reward the unit, not the engine.
            let solver_factor = if solver.starts_with("matrix-f5") {
                0.5
            } else if solver.starts_with("crossbred-f2") {
                0.3
            } else if solver.starts_with("buchberger-f2") {
                0.01
            } else if solver.starts_with("fes-f2") {
                3.0
            } else {
                1.0
            };
            let unit = if solver.starts_with("buchberger-f2") {
                "monomial operations"
            } else if solver.starts_with("crossbred-f2") {
                "word operations (partial: extraction, transform and filter)"
            } else if solver.starts_with("sat-cdcl") {
                "conflicts"
            } else {
                "word XORs (elimination only)"
            };
            rec.context = serde_json::json!({"solver": {"op_unit": unit}});
            let growth = 2f64.powf(0.5 * n);
            let shape =
                1.0 + 4.0 * ((dim - ideal) / ideal).powi(2) + if m == 2.0 { 0.2 } else { 0.0 };
            let g = growth * shape * solver_factor;
            let s: Vec<f64> = [0.0, 0.1, 1.0]
                .iter()
                .map(|w| (g + w * 2.0 * g) / 10.0)
                .collect();
            rec.band = Some(Band {
                weights: vec![0.0, 0.1, 1.0],
                s,
                group_gae: g,
                native_ops: 2.0 * g,
                solver_ops: g,
            });
            rec
        }
    }

    fn header(study: &Study, space: &Space) -> Header {
        Header {
            schema: STORE_SCHEMA.into(),
            space: space.name.into(),
            track: space.track.as_str().into(),
            counted: space.track.counted(),
            instance: "koblitz a=0".into(),
            rungs: study.rungs.clone(),
            target_log2_r: study.target_log2_r,
            min_fit: study.min_fit,
            repeats: study.repeats,
            seed: study.seed,
            max_trials: study.max_trials,
            rho_runs: study.rho_runs,
            git_commit: None,
            binary_blake3: None,
        }
    }

    fn study(trials: usize) -> Study {
        Study {
            rungs: vec![13, 17, 19, 23],
            // Near the rungs: these tests are about the loop, and the
            // projection has its own tests.  A far target multiplies the
            // rung-to-rung rounding noise of the mock.
            target_log2_r: 30.0,
            min_fit: 4,
            trials,
            workers: 1,
            seed: 42,
            repeats: 1,
            max_trials: 1000,
            rho_runs: 0,
            prune_startup: 5,
            tpe: TpeOptions::default(),
        }
    }

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("ic-sweep-driver-{}-{name}", std::process::id()));
        let _ = std::fs::remove_file(&p);
        p
    }

    #[test]
    fn a_study_finds_the_cheaper_solver_prunes_and_records_everything() {
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let st = study(60);
        let path = tmp("find");
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        let mock = Mock {
            calls: AtomicUsize::new(0),
        };
        let summary = run_study(&space, &st, &mut store, &mock, &|_| {}).unwrap();
        assert_eq!(summary.complete + summary.pruned + summary.failed, 60);
        assert_eq!(summary.ran_now, 60);
        let best = summary.best.expect("some trial completes");
        assert!(
            best.objective.unwrap() < 0.0,
            "the best configuration must beat the baseline: {:?}",
            best.objective
        );
        assert!(
            matches!(
                best.assignment["solver"].as_str(),
                Some("inherited-f4" | "matrix-f4" | "matrix-f5" | "f4-f2" | "fes-f2")
            ),
            "only a solver in the baseline's unit can win: {:?}",
            best.assignment
        );
        assert!(best.projected, "four rungs are enough for a fit");
        assert!(
            summary.pruned > 0,
            "the median rule should stop some trials early"
        );
        assert!(
            summary.failed > 0,
            "SAT exhausts on the larger rungs and must be recorded as failed"
        );
        // A pruned trial never ran the later rungs.
        let pruned = store
            .trials
            .iter()
            .find(|t| t.state == TrialState::Pruned)
            .unwrap();
        assert!(pruned.rungs.len() < 4);
        // Four baselines were scored once each, then trials.
        assert_eq!(store.baselines.len(), 4);
        let _ = std::fs::remove_file(&path);
    }

    /// The unit guard.  The mock makes `buchberger-f2` (monomial operations)
    /// 100× and `crossbred-f2` (partial word operations) 3× cheaper than the
    /// word-XOR baseline by their units alone.  A search that minimised the
    /// ratio would learn to pick them; it must not be able to win with them.
    #[test]
    fn a_coarser_solver_unit_cannot_win_by_changing_the_unit() {
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let st = study(80);
        let path = tmp("units");
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        let mock = Mock {
            calls: AtomicUsize::new(0),
        };
        let summary = run_study(&space, &st, &mut store, &mock, &|_| {}).unwrap();
        let other_unit = |t: &TrialLine| {
            matches!(
                t.assignment["solver"].as_str(),
                Some("buchberger-f2" | "crossbred-f2" | "sat-cdcl")
            )
        };
        let tried: Vec<&TrialLine> = store.trials.iter().filter(|t| other_unit(t)).collect();
        assert!(
            !tried.is_empty(),
            "the search must have tried a coarse-unit solver for this test to mean anything"
        );
        for t in &tried {
            assert_eq!(
                t.state,
                TrialState::Failed,
                "trial {} used {:?}",
                t.id,
                t.assignment["solver"]
            );
            assert!(t.objective.is_none());
        }
        // The first rung of such a trial is recorded in full, marked not
        // rankable, with no ratio, so the store shows what was refused.
        let rec = tried
            .iter()
            .find(|t| t.assignment["solver"] != serde_json::json!("sat-cdcl"))
            .and_then(|t| t.rungs.last())
            .expect("a recorded rung");
        assert!(!rec.rankable && rec.ratio.is_none() && rec.band.is_some());
        assert!(rec.detail.contains("not rankable"), "{}", rec.detail);
        assert_eq!(rec.op_unit.as_deref().is_some(), true);
        let best = summary.best.expect("a rankable trial completes");
        assert!(!other_unit(&best), "{:?}", best.assignment);
        assert!(
            store
                .trials
                .iter()
                .filter(|t| t.state == TrialState::Complete)
                .all(|t| !other_unit(t)),
            "no coarse-unit trial may complete"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_resumed_study_runs_only_the_missing_trials() {
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let path = tmp("resume");
        let first = {
            let st = study(12);
            let mut store = Store::open(&path, &header(&st, &space)).unwrap();
            let mock = Mock {
                calls: AtomicUsize::new(0),
            };
            run_study(&space, &st, &mut store, &mock, &|_| {}).unwrap();
            mock.calls.load(Ordering::SeqCst)
        };
        assert!(first > 4, "baselines plus trials");
        let st = study(20);
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        assert_eq!(store.trials.len(), 12);
        let mock = Mock {
            calls: AtomicUsize::new(0),
        };
        let summary = run_study(&space, &st, &mut store, &mock, &|_| {}).unwrap();
        assert_eq!(summary.ran_now, 8, "only the 8 missing trials run");
        let ids: Vec<usize> = store.trials.iter().map(|t| t.id).collect();
        assert_eq!(
            ids,
            (0..20).collect::<Vec<_>>(),
            "ids continue with no gaps or repeats"
        );
        // The baselines came from the store, not from new runs: 8 trials of
        // at most 4 rungs each.
        assert!(mock.calls.load(Ordering::SeqCst) <= 8 * 4);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn rungs_the_baseline_cannot_be_scored_on_are_dropped() {
        struct Capped(Mock);
        impl Evaluator for Capped {
            fn evaluate(&self, cfg: &TrialConfig) -> TrialRecord {
                let mut r = self.0.evaluate(cfg);
                if cfg.degree >= 23 && cfg.oracle == "descent-algebraic:m=2" {
                    // The incumbent F4 baseline runs out of budget here.
                    r.class = Class::Exhausted;
                    r.band = None;
                }
                r
            }
        }
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let st = study(8);
        let path = tmp("drop");
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        let summary = run_study(
            &space,
            &st,
            &mut store,
            &Capped(Mock {
                calls: AtomicUsize::new(0),
            }),
            &|_| {},
        )
        .unwrap();
        assert_eq!(summary.usable_rungs, vec![13, 17, 19]);
        assert!(
            summary.best.map_or(true, |b| !b.projected),
            "three rungs are below min_fit, so nothing is projected"
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn a_study_needs_two_scorable_rungs() {
        struct Never;
        impl Evaluator for Never {
            fn evaluate(&self, cfg: &TrialConfig) -> TrialRecord {
                crashed(cfg, "no".into())
            }
        }
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let st = study(3);
        let path = tmp("never");
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        let err = run_study(&space, &st, &mut store, &Never, &|_| {}).unwrap_err();
        assert!(err.contains("at least 2"), "{err}");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn parallel_workers_complete_every_trial_exactly_once() {
        let space = space_for("koblitz-algebraic", &instance()).unwrap();
        let mut st = study(24);
        st.workers = 4;
        let path = tmp("parallel");
        let mut store = Store::open(&path, &header(&st, &space)).unwrap();
        let summary = run_study(
            &space,
            &st,
            &mut store,
            &Mock {
                calls: AtomicUsize::new(0),
            },
            &|_| {},
        )
        .unwrap();
        assert_eq!(summary.complete + summary.pruned + summary.failed, 24);
        let mut ids: Vec<usize> = store.trials.iter().map(|t| t.id).collect();
        ids.sort();
        assert_eq!(ids, (0..24).collect::<Vec<_>>());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn the_median_is_the_median() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&[4.0, 1.0, 2.0, 3.0]), 2.5);
    }
}
