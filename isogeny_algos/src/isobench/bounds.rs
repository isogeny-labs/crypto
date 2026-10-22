//! Bounds, challenges, verdicts and the frontier.
//!
//! * A **bound** (`isobench.bound/v1`, sealed `ISBND1h…`) records what one method costs on one
//!   problem: the mean cost per size with a bootstrap interval, and the fit
//!   cost = C size^alpha by least squares in log-log space with bootstrap intervals for alpha and
//!   C (instances resampled within each size; 2000 resamples, fixed seed). The exponent is a
//!   scaling claim only with at least four sizes. The same fit is made for wall time.
//! * A **challenge** (`ISCH1h…`) is a frozen paired spec: incumbent, candidate and an A/A
//!   control (the incumbent run twice), with its acceptance rules.
//! * A **verdict** (`ISVD1h…`) pairs the candidate with the incumbent execution by execution:
//!   the ratio sum(candidate) / sum(incumbent) with a bootstrap interval decides `advances`
//!   (interval below 1), `regresses` (above 1) or `matches`; the counted unit and wall time
//!   pointing opposite ways is a `trade` (wall time counts only when its own A/A interval
//!   includes 1); an A/A cost interval excluding 1 makes it `inadmissible`.
//!   The level is `exponent` only when the two arms' alpha intervals are disjoint over at least
//!   four sizes, otherwise `constant`.
//! * The **frontier** lists, per problem and parameters, the bounds no other bound dominates
//!   (dominance is conservative: alpha intervals disjoint, or overlapping alphas and the mean
//!   cost at the largest common size with disjoint intervals; ties stand).
use super::harness::{read_records, seal, sealed, Spec};
use super::problems;
use crate::field::Rng;
use crate::json::Json;
use std::collections::BTreeMap;
use std::path::Path;

pub const RESAMPLES: usize = 2000;
pub const BOOT_SEED: u64 = 20261022;
pub const MIN_SIZES_FOR_SCALING: usize = 4;

/// Points of one arm: size -> per-instance costs (mean over rounds), verified records only.
fn arm_points(records: &[Json], arm: &str, key: &str) -> (BTreeMap<u64, Vec<f64>>, usize) {
    let mut by: BTreeMap<(u64, u64), Vec<f64>> = BTreeMap::new();
    let mut unverified = 0;
    for r in records {
        if r.get("arm").and_then(|a| a.as_str()) != Some(arm) {
            continue;
        }
        if r.get("status").and_then(|s| s.as_str()) != Some("verified") {
            unverified += 1;
            continue;
        }
        let size = r.get("size").and_then(|v| v.as_f64()).unwrap() as u64;
        let index = r.get("index").and_then(|v| v.as_f64()).unwrap() as u64;
        if let Some(c) = r.get(key).and_then(|v| v.as_f64()) {
            by.entry((size, index)).or_default().push(c);
        }
    }
    let mut out: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for ((size, _), v) in by {
        out.entry(size)
            .or_default()
            .push(v.iter().sum::<f64>() / v.len() as f64);
    }
    (out, unverified)
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

/// OLS of log y on log x: (alpha, log C).
fn fit(pts: &[(f64, f64)]) -> (f64, f64) {
    let n = pts.len() as f64;
    let (lx, ly): (Vec<f64>, Vec<f64>) = pts
        .iter()
        .map(|&(x, y)| (x.ln(), y.max(1e-12).ln()))
        .unzip();
    let (mx, my) = (mean(&lx), mean(&ly));
    let sxx: f64 = lx.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = lx.iter().zip(&ly).map(|(x, y)| (x - mx) * (y - my)).sum();
    let a = if sxx > 0.0 { sxy / sxx } else { 0.0 };
    let _ = n;
    (a, my - a * mx)
}

fn quantiles(mut v: Vec<f64>) -> (f64, f64) {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| v[((p * (v.len() - 1) as f64).round() as usize).min(v.len() - 1)];
    (q(0.025), q(0.975))
}

fn resample(v: &[f64], rng: &mut Rng) -> Vec<f64> {
    (0..v.len())
        .map(|_| v[rng.below(v.len() as u64) as usize])
        .collect()
}

/// The fit of size -> instance costs, with bootstrap intervals.
pub struct Fit {
    pub alpha: f64,
    pub alpha_ci: (f64, f64),
    pub c: f64,
    pub c_ci: (f64, f64),
    pub per_size: Vec<(u64, usize, f64, (f64, f64))>,
}

pub fn fit_points(pts: &BTreeMap<u64, Vec<f64>>) -> Option<Fit> {
    if pts.is_empty() || pts.values().any(|v| v.is_empty()) {
        return None;
    }
    let mut rng = Rng::new(BOOT_SEED);
    let base: Vec<(f64, f64)> = pts.iter().map(|(&s, v)| (s as f64, mean(v))).collect();
    let (alpha, lc) = if base.len() >= 2 {
        fit(&base)
    } else {
        (0.0, base[0].1.ln())
    };
    let mut alphas = vec![];
    let mut cs = vec![];
    let mut means: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    for _ in 0..RESAMPLES {
        let b: Vec<(f64, f64)> = pts
            .iter()
            .map(|(&s, v)| {
                let m = mean(&resample(v, &mut rng));
                means.entry(s).or_default().push(m);
                (s as f64, m)
            })
            .collect();
        if b.len() >= 2 {
            let (a, l) = fit(&b);
            alphas.push(a);
            cs.push(l.exp());
        }
    }
    let per_size = pts
        .iter()
        .map(|(&s, v)| (s, v.len(), mean(v), quantiles(means[&s].clone())))
        .collect();
    let (alpha_ci, c_ci) = if alphas.is_empty() {
        ((alpha, alpha), (lc.exp(), lc.exp()))
    } else {
        (quantiles(alphas), quantiles(cs))
    };
    Some(Fit {
        alpha,
        alpha_ci,
        c: lc.exp(),
        c_ci,
        per_size,
    })
}

fn f(x: f64) -> Json {
    // six significant digits keep sealed documents stable and readable
    if x == 0.0 || !x.is_finite() {
        return Json::Float(x);
    }
    let d = 6 - x.abs().log10().floor() as i32 - 1;
    let m = 10f64.powi(d.max(-300));
    Json::Float((x * m).round() / m)
}

fn ci(c: (f64, f64)) -> Json {
    Json::Arr(vec![f(c.0), f(c.1)])
}

fn fit_json(fit: &Fit) -> Json {
    Json::obj(vec![
        ("alpha", f(fit.alpha)),
        ("alpha_ci95", ci(fit.alpha_ci)),
        ("c", f(fit.c)),
        ("c_ci95", ci(fit.c_ci)),
        (
            "per_size",
            Json::Arr(
                fit.per_size
                    .iter()
                    .map(|(s, n, m, c)| {
                        Json::obj(vec![
                            ("size", Json::Num(*s as i64)),
                            ("instances", Json::Num(*n as i64)),
                            ("mean", f(*m)),
                            ("mean_ci95", ci(*c)),
                        ])
                    })
                    .collect(),
            ),
        ),
    ])
}

/// Fit one arm of one or more sessions (same problem and parameters) into a sealed bound.
pub fn bound(sessions: &[&Path], arm: &str) -> Result<Json, String> {
    let mut records = vec![];
    let mut spec0: Option<Spec> = None;
    let mut ids = vec![];
    for d in sessions {
        let spec =
            Spec::parse(&std::fs::read_to_string(d.join("spec.json")).map_err(|e| e.to_string())?)?;
        if let Some(s0) = &spec0 {
            if s0.problem != spec.problem || s0.params != spec.params {
                return Err("sessions of one bound must share problem and parameters".into());
            }
        }
        let session = Json::parse(
            &std::fs::read_to_string(d.join("session.json")).map_err(|e| e.to_string())?,
        )?;
        ids.push(Json::str(
            session.get("id").and_then(|i| i.as_str()).unwrap_or("?"),
        ));
        records.extend(read_records(d)?);
        spec0.get_or_insert(spec);
    }
    let spec = spec0.ok_or("no session")?;
    let method = spec
        .arms
        .iter()
        .find(|a| a.name == arm)
        .ok_or_else(|| format!("no arm {arm}"))?
        .method
        .clone();
    let decl = problems::problem(&spec.problem)?;
    let (pts, unverified) = arm_points(&records, arm, "cost");
    let (wall, _) = arm_points(&records, arm, "wall_ms");
    let cost = fit_points(&pts).ok_or("no verified records for this arm")?;
    let wallfit = fit_points(&wall);
    let sizes = pts.len();
    let doc = Json::obj(vec![
        ("schema", Json::str("isobench.bound/v1")),
        (
            "domain",
            Json::obj(vec![
                ("problem", Json::str(&spec.problem)),
                (
                    "params",
                    Json::Obj(
                        spec.params
                            .iter()
                            .map(|(k, v)| (k.clone(), Json::str(v)))
                            .collect(),
                    ),
                ),
                ("size", Json::str(decl.size)),
                ("unit", Json::str(decl.unit)),
            ]),
        ),
        ("method", Json::str(&method)),
        ("sessions", Json::Arr(ids)),
        (
            "calibration",
            records
                .iter()
                .find_map(|r| {
                    r.get("calibration")
                        .filter(|c| !matches!(c, Json::Null))
                        .cloned()
                })
                .unwrap_or(Json::Null),
        ),
        ("declared_alpha", f(decl.declared_alpha)),
        ("sizes", Json::Num(sizes as i64)),
        ("scaling_claim", Json::Bool(sizes >= MIN_SIZES_FOR_SCALING)),
        ("unverified_records", Json::Num(unverified as i64)),
        ("cost", fit_json(&cost)),
        ("wall_ms", wallfit.as_ref().map_or(Json::Null, fit_json)),
        (
            "bootstrap",
            Json::obj(vec![
                ("resamples", Json::Num(RESAMPLES as i64)),
                ("seed", Json::Num(BOOT_SEED as i64)),
            ]),
        ),
    ]);
    Ok(sealed("ISBND1h", doc))
}

/// Re-derive a committed bound from its sessions: the document must be identical.
pub fn check_bound(doc_text: &str, session_root: &Path) -> Result<(), String> {
    let doc = Json::parse(doc_text)?;
    let (claimed, recomputed) = super::harness::reseal("ISBND1h", &doc).ok_or("bound: no id")?;
    if claimed != recomputed {
        return Err("bound: seal does not match".into());
    }
    let Some(Json::Arr(sids)) = doc.get("sessions") else {
        return Err("bound: no sessions".into());
    };
    let dirs: Vec<std::path::PathBuf> = sids
        .iter()
        .map(|s| find_session(session_root, s.as_str().unwrap_or("")))
        .collect::<Result<_, _>>()?;
    let method = doc
        .get("method")
        .and_then(|m| m.as_str())
        .ok_or("bound: method")?;
    let spec = Spec::parse(
        &std::fs::read_to_string(dirs[0].join("spec.json")).map_err(|e| e.to_string())?,
    )?;
    let arm = spec
        .arms
        .iter()
        .find(|a| a.method == method && a.role != "control")
        .ok_or("bound: no arm with that method")?
        .name
        .clone();
    let refs: Vec<&Path> = dirs.iter().map(|d| d.as_path()).collect();
    if bound(&refs, &arm)?.dump() != doc.dump() {
        return Err("bound differs from the one re-derived from its sessions".into());
    }
    Ok(())
}

/// The session directory under `root` whose session.json has this id.
pub fn find_session(root: &Path, id: &str) -> Result<std::path::PathBuf, String> {
    for e in walk(root) {
        if e.file_name().and_then(|n| n.to_str()) == Some("session.json") {
            if let Ok(t) = std::fs::read_to_string(&e) {
                if t.contains(id) {
                    return Ok(e.parent().unwrap().to_path_buf());
                }
            }
        }
    }
    Err(format!("session {id} not found under {}", root.display()))
}

fn walk(root: &Path) -> Vec<std::path::PathBuf> {
    let mut out = vec![];
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    stack.push(p);
                } else {
                    out.push(p);
                }
            }
        }
    }
    out.sort();
    out
}

// ------------------------------------------------------------------ challenges and verdicts

/// Seal a challenge: a paired spec (incumbent, candidate, A/A control) frozen before running.
pub fn challenge(
    problem: &str,
    params: &BTreeMap<String, String>,
    incumbent: &str,
    candidate: &str,
    sizes: &[u64],
    instances: u64,
    rounds: u64,
    seed: u64,
) -> Result<Json, String> {
    let decl = problems::problem(problem)?;
    for m in [incumbent, candidate] {
        if !decl.methods.contains(&m) {
            return Err(format!("{problem} has no method {m}"));
        }
    }
    if sizes.len() < 2 {
        return Err("a challenge needs at least two sizes".into());
    }
    let spec = Json::obj(vec![
        ("schema", Json::str(super::harness::SPEC_SCHEMA)),
        (
            "label",
            Json::str(format!("challenge {problem}: {candidate} vs {incumbent}")),
        ),
        (
            "description",
            Json::str(
                "paired challenge: incumbent, candidate, and the incumbent again as an A/A control",
            ),
        ),
        ("problem", Json::str(problem)),
        (
            "params",
            Json::Obj(
                params
                    .iter()
                    .map(|(k, v)| (k.clone(), Json::str(v)))
                    .collect(),
            ),
        ),
        (
            "sizes",
            Json::Arr(sizes.iter().map(|&s| Json::Num(s as i64)).collect()),
        ),
        ("instances_per_size", Json::Num(instances as i64)),
        (
            "arms",
            Json::Arr(vec![
                Json::obj(vec![
                    ("name", Json::str("incumbent")),
                    ("role", Json::str("baseline")),
                    ("method", Json::str(incumbent)),
                ]),
                Json::obj(vec![
                    ("name", Json::str("candidate")),
                    ("role", Json::str("candidate")),
                    ("method", Json::str(candidate)),
                ]),
                Json::obj(vec![
                    ("name", Json::str("aa")),
                    ("role", Json::str("control")),
                    ("method", Json::str(incumbent)),
                ]),
            ]),
        ),
        (
            "measurement",
            Json::obj(vec![
                ("rounds", Json::Num(rounds as i64)),
                ("seed", Json::Num(seed as i64)),
            ]),
        ),
    ]);
    let doc = Json::obj(vec![
        ("schema", Json::str("isobench.challenge/v1")),
        ("spec", spec),
        (
            "acceptance",
            Json::obj(vec![
                ("min_sizes", Json::Num(2)),
                ("min_runs_per_size", Json::Num(4)),
                ("require_replay_all", Json::Bool(true)),
                ("aa_must_include_one", Json::Bool(true)),
            ]),
        ),
    ]);
    Ok(sealed("ISCH1h", doc))
}

/// Paired ratio sum(a) / sum(b) over executions, with a bootstrap interval (executions
/// resampled).
fn paired_ratio(pairs: &[(f64, f64)], rng: &mut Rng) -> (f64, (f64, f64)) {
    let r =
        |p: &[(f64, f64)]| p.iter().map(|x| x.0).sum::<f64>() / p.iter().map(|x| x.1).sum::<f64>();
    let point = r(pairs);
    let boots: Vec<f64> = (0..RESAMPLES)
        .map(|_| {
            let b: Vec<(f64, f64)> = (0..pairs.len())
                .map(|_| pairs[rng.below(pairs.len() as u64) as usize])
                .collect();
            r(&b)
        })
        .collect();
    (point, quantiles(boots))
}

/// Judge a challenge from its session (run from the challenge's spec, audited separately).
pub fn verdict(challenge_text: &str, session: &Path) -> Result<Json, String> {
    let ch = Json::parse(challenge_text)?;
    let (claimed, again) = super::harness::reseal("ISCH1h", &ch).ok_or("challenge: no id")?;
    if claimed != again {
        return Err("challenge: seal does not match".into());
    }
    let spec = Spec::parse(
        &std::fs::read_to_string(session.join("spec.json")).map_err(|e| e.to_string())?,
    )?;
    if spec.id != seal("ISS1h", ch.get("spec").ok_or("challenge: spec")?) {
        return Err("the session did not run this challenge's spec".into());
    }
    let records = read_records(session)?;
    let key = |r: &Json| {
        (
            r.get("size").and_then(|v| v.as_f64()).unwrap() as u64,
            r.get("index").and_then(|v| v.as_f64()).unwrap() as u64,
            r.get("round").and_then(|v| v.as_f64()).unwrap() as u64,
        )
    };
    let mut by: BTreeMap<(u64, u64, u64), BTreeMap<String, (f64, f64, bool)>> = BTreeMap::new();
    for r in &records {
        let ok = r.get("status").and_then(|s| s.as_str()) == Some("verified");
        let c = r.get("cost").and_then(|v| v.as_f64()).unwrap_or(f64::NAN);
        let w = r
            .get("wall_ms")
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::NAN);
        by.entry(key(r)).or_default().insert(
            r.get("arm").and_then(|a| a.as_str()).unwrap_or("").into(),
            (c, w, ok),
        );
    }
    let mut cost_pairs = vec![];
    let mut wall_pairs = vec![];
    let mut aa_pairs = vec![];
    let mut aa_wall = vec![];
    let mut failures = 0;
    for v in by.values() {
        let (Some(i), Some(c), Some(a)) = (v.get("incumbent"), v.get("candidate"), v.get("aa"))
        else {
            continue;
        };
        if !(i.2 && c.2 && a.2) {
            failures += 1;
            continue;
        }
        cost_pairs.push((c.0, i.0));
        wall_pairs.push((c.1, i.1));
        aa_pairs.push((a.0, i.0));
        aa_wall.push((a.1, i.1));
    }
    let mut rng = Rng::new(BOOT_SEED);
    let (cr, cci) = paired_ratio(&cost_pairs, &mut rng);
    let (wr, wci) = paired_ratio(&wall_pairs, &mut rng);
    let (ar, aci) = paired_ratio(&aa_pairs, &mut rng);
    let (awr, awci) = paired_ratio(&aa_wall, &mut rng);
    let dir = |c: (f64, f64)| {
        if c.1 < 1.0 {
            1
        } else if c.0 > 1.0 {
            -1
        } else {
            0
        }
    };
    let (dc, dw) = (dir(cci), dir(wci));
    let sizes = spec.sizes.len();
    let per_size_runs = cost_pairs.len() / sizes.max(1);
    let mut reasons = vec![];
    if aci.0 > 1.0 || aci.1 < 1.0 {
        reasons.push("the A/A control's cost interval excludes 1".to_string());
    }
    // wall time decides a trade only when its own A/A control is clean
    let dw = if awci.0 > 1.0 || awci.1 < 1.0 { 0 } else { dw };
    if failures > 0 {
        reasons.push(format!("{failures} executions not verified"));
    }
    if per_size_runs < 4 {
        reasons.push("fewer than 4 paired runs per size".into());
    }
    let outcome = if !reasons.is_empty() {
        "inadmissible"
    } else if dc * dw < 0 {
        "trade"
    } else if dc > 0 {
        "advances"
    } else if dc < 0 {
        "regresses"
    } else {
        "matches"
    };
    // the level: per-arm alpha intervals within this session
    let (pi, _) = arm_points(&records, "incumbent", "cost");
    let (pc, _) = arm_points(&records, "candidate", "cost");
    let (fi, fc) = (fit_points(&pi), fit_points(&pc));
    let level = match (&fi, &fc) {
        (Some(a), Some(b))
            if sizes >= MIN_SIZES_FOR_SCALING
                && (a.alpha_ci.1 < b.alpha_ci.0 || b.alpha_ci.1 < a.alpha_ci.0) =>
        {
            "exponent"
        }
        _ => "constant",
    };
    let doc = Json::obj(vec![
        ("schema", Json::str("isobench.verdict/v1")),
        ("challenge", Json::str(claimed)),
        ("spec", Json::str(&spec.id)),
        (
            "session",
            Json::str(
                Json::parse(
                    &std::fs::read_to_string(session.join("session.json"))
                        .map_err(|e| e.to_string())?,
                )?
                .get("id")
                .and_then(|i| i.as_str())
                .unwrap_or("?"),
            ),
        ),
        ("pairs", Json::Num(cost_pairs.len() as i64)),
        ("cost_ratio", f(cr)),
        ("cost_ratio_ci95", ci(cci)),
        ("wall_ratio", f(wr)),
        ("wall_ratio_ci95", ci(wci)),
        ("aa_ratio", f(ar)),
        ("aa_ratio_ci95", ci(aci)),
        ("aa_wall_ratio", f(awr)),
        ("aa_wall_ratio_ci95", ci(awci)),
        (
            "incumbent_alpha_ci95",
            fi.as_ref().map_or(Json::Null, |x| ci(x.alpha_ci)),
        ),
        (
            "candidate_alpha_ci95",
            fc.as_ref().map_or(Json::Null, |x| ci(x.alpha_ci)),
        ),
        ("outcome", Json::str(outcome)),
        ("level", Json::str(level)),
        (
            "reasons",
            Json::Arr(reasons.into_iter().map(Json::str).collect()),
        ),
    ]);
    Ok(sealed("ISVD1h", doc))
}

// ------------------------------------------------------------------ frontier

struct B {
    file: String,
    id: String,
    problem: String,
    params: String,
    method: String,
    unit: String,
    alpha: (f64, f64, f64),
    sizes: usize,
    scaling: bool,
    largest: (u64, f64, (f64, f64)),
    per: BTreeMap<u64, (f64, (f64, f64))>,
    declared: f64,
}

fn tri(j: &Json, k: &str, kci: &str) -> (f64, f64, f64) {
    let v = j.get(k).and_then(|x| x.as_f64()).unwrap_or(f64::NAN);
    let c = match j.get(kci) {
        Some(Json::Arr(a)) if a.len() == 2 => {
            (a[0].as_f64().unwrap_or(v), a[1].as_f64().unwrap_or(v))
        }
        _ => (v, v),
    };
    (v, c.0, c.1)
}

fn load_bound(path: &Path) -> Result<B, String> {
    let t = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let j = Json::parse(&t)?;
    let d = j.get("domain").ok_or("bound: domain")?;
    let params = match d.get("params") {
        Some(Json::Obj(kv)) => kv
            .iter()
            .map(|(k, v)| format!("{k}={}", v.as_str().unwrap_or("")))
            .collect::<Vec<_>>()
            .join(" "),
        _ => String::new(),
    };
    let cost = j.get("cost").ok_or("bound: cost")?;
    let mut per = BTreeMap::new();
    if let Some(Json::Arr(ps)) = cost.get("per_size") {
        for p in ps {
            let (m, lo, hi) = tri(p, "mean", "mean_ci95");
            per.insert(
                p.get("size").and_then(|s| s.as_f64()).unwrap() as u64,
                (m, (lo, hi)),
            );
        }
    }
    let (&ls, &(lm, lci)) = per.iter().next_back().ok_or("bound: no sizes")?;
    Ok(B {
        file: path.file_name().unwrap().to_string_lossy().into(),
        id: j.get("id").and_then(|i| i.as_str()).unwrap_or("?").into(),
        problem: d
            .get("problem")
            .and_then(|s| s.as_str())
            .unwrap_or("?")
            .into(),
        params,
        method: j
            .get("method")
            .and_then(|s| s.as_str())
            .unwrap_or("?")
            .into(),
        unit: d.get("unit").and_then(|s| s.as_str()).unwrap_or("?").into(),
        alpha: tri(cost, "alpha", "alpha_ci95"),
        sizes: j.get("sizes").and_then(|s| s.as_f64()).unwrap_or(0.0) as usize,
        scaling: matches!(j.get("scaling_claim"), Some(Json::Bool(true))),
        largest: (ls, lm, lci),
        per,
        declared: j
            .get("declared_alpha")
            .and_then(|s| s.as_f64())
            .unwrap_or(f64::NAN),
    })
}

/// Does a dominate b (conservatively)?
fn dominates(a: &B, b: &B) -> bool {
    if a.scaling && b.scaling && a.alpha.2 < b.alpha.1 {
        return true;
    }
    let overlap = a.alpha.1 <= b.alpha.2 && b.alpha.1 <= a.alpha.2;
    if !(overlap || !(a.scaling && b.scaling)) {
        return false;
    }
    // the largest size both measured
    let common = a.per.keys().filter(|s| b.per.contains_key(s)).max();
    match common {
        Some(s) => a.per[s].1 .1 < b.per[s].1 .0,
        None => false,
    }
}

/// Build the frontier page (markdown) and its JSON from every bound record in `dir`.
pub fn frontier(dir: &Path) -> Result<(String, Json), String> {
    let mut bounds = vec![];
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();
    files.sort();
    for p in &files {
        bounds.push(load_bound(p)?);
    }
    let mut groups: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (i, b) in bounds.iter().enumerate() {
        groups
            .entry((b.problem.clone(), b.params.clone()))
            .or_default()
            .push(i);
    }
    let mut md = String::from(
        "# Isogeny frontier\n\nGenerated by `isobench frontier build` from `docs/isobench/bounds/records/`; \
         do not edit. A method is on the frontier when no other bound for the same problem and parameters \
         dominates it (alpha intervals disjoint, or overlapping alphas and disjoint mean-cost intervals at \
         the largest common size). Exponents are scaling claims only with at least four sizes. Units: \
         `isobench.fme` field-multiplication equivalents (pinned calibration), `isobench.oracle_calls` \
         neighbour-oracle root findings, `isobench.norm_bits` log2 of the output norm.\n",
    );
    let mut jgroups = vec![];
    for ((problem, params), idx) in &groups {
        md.push_str(&format!("\n## {problem} ({params})\n\n"));
        md.push_str("| method | status | unit | alpha (95% CI) | declared | sizes | mean cost at largest size (95% CI) | bound |\n");
        md.push_str("|:--|:--|:--|:--|--:|--:|:--|:--|\n");
        let mut rows = vec![];
        let mut order = idx.clone();
        order.sort_by(|&a, &b| {
            bounds[a]
                .largest
                .1
                .partial_cmp(&bounds[b].largest.1)
                .unwrap()
        });
        for &i in &order {
            let b = &bounds[i];
            let by: Vec<&str> = idx
                .iter()
                .filter(|&&k| k != i && dominates(&bounds[k], b))
                .map(|&k| bounds[k].method.as_str())
                .collect();
            let status = if by.is_empty() {
                "frontier".to_string()
            } else {
                format!("dominated by {}", by.join(", "))
            };
            let (l, lm, lci) = b.largest;
            md.push_str(&format!(
                "| {} | {} | {} | {:.3} [{:.3}, {:.3}]{} | {} | {} | {} @ {} [{}, {}] | `{}` |\n",
                b.method,
                status,
                b.unit.trim_start_matches("isobench."),
                b.alpha.0,
                b.alpha.1,
                b.alpha.2,
                if b.scaling { "" } else { " (no scaling claim)" },
                b.declared,
                b.sizes,
                sig(lm),
                l,
                sig(lci.0),
                sig(lci.1),
                b.file
            ));
            rows.push(Json::obj(vec![
                ("method", Json::str(&b.method)),
                ("bound", Json::str(&b.id)),
                ("status", Json::str(&status)),
            ]));
        }
        jgroups.push(Json::obj(vec![
            ("problem", Json::str(problem)),
            ("params", Json::str(params)),
            ("methods", Json::Arr(rows)),
        ]));
    }
    let j = Json::obj(vec![
        ("schema", Json::str("isobench.frontier/v1")),
        ("groups", Json::Arr(jgroups)),
    ]);
    Ok((md, j))
}

fn sig(x: f64) -> String {
    if x >= 1e6 || (x != 0.0 && x < 1e-2) {
        format!("{x:.3e}")
    } else if x >= 100.0 {
        format!("{x:.0}")
    } else {
        format!("{x:.2}")
    }
}
