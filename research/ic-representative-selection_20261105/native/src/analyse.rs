//! The frozen analysis (PROTOCOL.md §7). Every number in the study report is
//! computed here from sealed ecbench `records.jsonl` files and the workload
//! labels `repsel specs` wrote; nothing is computed elsewhere.
//!
//! - **Price.** A run's priced cost at band weight `w` is
//!   `S(w) = (total_gae + w·(U + extra_ops)) / √r`, with `U` the record's
//!   unpriced native operations exactly as `ecbench band` counts them
//!   (`src/cryptanalysis/ecbench/band.rs`, memory-size counters excluded)
//!   and `extra_ops` the arm's charged transport or change of model. A record
//!   with a pinned calibration, or an unpriced counter without a count, is
//!   refused rather than mispriced.
//! - **Statistic.** For a comparison X vs Y on paired keys (unit, target,
//!   round): per unit the mean over complete pairs, per class the ratio of
//!   unit means, pooled `θ = exp(mean_c ln R_c)` over classes.
//! - **Interval.** Hierarchical bootstrap: classes, then units within class,
//!   then pairs within unit, all with replacement.
//! - **Test.** Unit-level sign flip (swap X and Y for all pairs of a unit),
//!   one-sided add-one p. Across the band the p of a hypothesis is the
//!   largest over the three weights (intersection–union), then Holm.

use std::collections::{BTreeMap, HashMap};

use serde_json::{json, Value};

pub const WEIGHTS: [f64; 3] = [0.0, 0.1, 1.0];
const NON_OPERATION_COUNTERS: [&str; 3] = [
    "residual_table_bytes_uncharged",
    "factor_base_vec_bytes_floor_uncharged",
    "native_table_bytes_uncharged",
];

/// The fields of one ecbench record the analysis reads.
#[derive(Clone, Debug)]
pub struct Run {
    pub session: String,
    pub key: String,
    pub arm: String,
    pub target: u64,
    pub round: u64,
    pub warmup: bool,
    pub ok: bool,
    pub status: String,
    pub total_gae: f64,
    pub unpriced: Option<u64>,
    pub sqrt_r: f64,
    pub relations: f64,
    pub trials: f64,
    pub planted: Option<u64>,
    pub recovered: Option<String>,
}

pub fn workload_key(p: &Value, a: &Value, b: &Value, gx: &Value, gy: &Value) -> String {
    format!("{p}:{a}:{b}:{gx}:{gy}")
}

/// `U`: the record's unpriced operation count (band.rs `unpriced_counts`).
pub fn unpriced_ops(r: &Value) -> Result<Option<u64>, String> {
    let names = r["cost"]["unpriced"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut total = 0u64;
    for n in names {
        let name = n.as_str().ok_or("unpriced name")?;
        if NON_OPERATION_COUNTERS.contains(&name) {
            continue;
        }
        if let Some(v) = r["counters"].get(name).and_then(Value::as_u64) {
            total += v;
            continue;
        }
        let base = name.strip_suffix("_uncharged").unwrap_or(name);
        let mut found = false;
        for ph in r["phases"].as_array().cloned().unwrap_or_default() {
            if let Some(v) = ph["native"]
                .get(name)
                .or_else(|| ph["native"].get(base))
                .and_then(Value::as_u64)
            {
                total += v;
                found = true;
            }
        }
        if !found && base.starts_with("solver_") {
            if let Some(v) = r["solver"]["ops"].as_u64() {
                total += v;
                continue;
            }
        }
        if !found {
            return Ok(None);
        }
    }
    Ok(Some(total))
}

pub fn parse_run(line: &str) -> Result<Run, String> {
    let r: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
    let cal = r["detail"]["calibration_pinned_units"]
        .as_array()
        .map_or(0, Vec::len);
    if cal > 0 {
        return Err(format!(
            "record {} carries a pinned calibration; the band formula here assumes none",
            r["record_id"]
        ));
    }
    let cs = &r["workload"]["curve_spec"];
    let rr = cs["r"].as_f64().ok_or("workload r")?;
    let status = r["outcome"]["status"].as_str().unwrap_or("").to_string();
    let ok = status == "verified"
        && r["outcome"]["matches_target"] == json!(true)
        && r["outcome"]["matches_planted"] == json!(true);
    Ok(Run {
        session: r["session_id"].as_str().unwrap_or("").to_string(),
        key: workload_key(&cs["p"], &cs["a"], &cs["b"], &cs["gx"], &cs["gy"]),
        arm: r["arm"].as_str().ok_or("arm")?.to_string(),
        target: r["workload"]["target_index"]
            .as_u64()
            .ok_or("target_index")?,
        round: r["round"].as_u64().ok_or("round")?,
        warmup: r["warmup"].as_bool().unwrap_or(true),
        ok,
        status,
        total_gae: r["cost"]["total_gae"].as_f64().unwrap_or(f64::NAN),
        unpriced: unpriced_ops(&r)?,
        sqrt_r: rr.sqrt(),
        relations: r["counters"]["relations_found"]
            .as_f64()
            .unwrap_or(f64::NAN),
        trials: r["counters"]["targets_tried"].as_f64().unwrap_or(f64::NAN),
        planted: r["workload"]["planted"].as_u64(),
        recovered: r["outcome"]["recovered"].as_str().map(String::from),
    })
}

/// One labelled observation: a run seen through one of its workload's labels.
#[derive(Clone, Debug)]
pub struct Obs {
    pub run: Run,
    pub label: String,
    pub unit: String,
    pub class: String,
    pub tier: String,
    pub extra_ops: u64,
}

impl Obs {
    pub fn priced_s(&self, w: f64) -> Option<f64> {
        let u = if w == 0.0 { 0 } else { self.run.unpriced? };
        Some((self.run.total_gae + w * (u + self.extra_ops) as f64) / self.run.sqrt_r)
    }
}

/// `(tier, class, label, unit, extra_ops)` of one workload label.
pub type LabelRow = (String, String, String, String, u64);
/// Labels by workload key.
pub type Labels = HashMap<String, Vec<LabelRow>>;

pub fn read_labels(texts: &[String]) -> Result<Labels, String> {
    let mut out: HashMap<String, Vec<_>> = HashMap::new();
    for t in texts {
        for line in t.lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
            let w = &v["workload"];
            let key = workload_key(&w["p"], &w["a"], &w["b"], &w["gx"], &w["gy"]);
            for l in v["labels"].as_array().ok_or("labels")? {
                out.entry(key.clone()).or_default().push((
                    v["tier"].as_str().unwrap_or("").into(),
                    v["class"].as_str().unwrap_or("").into(),
                    l["arm"].as_str().ok_or("label arm")?.into(),
                    l["unit"].as_str().ok_or("label unit")?.into(),
                    l["extra_ops"].as_u64().unwrap_or(0),
                ));
            }
        }
    }
    Ok(out)
}

pub fn observations(records: &[String], labels: &Labels) -> Result<(Vec<Obs>, Vec<Run>), String> {
    let mut obs = Vec::new();
    let mut unlabelled = Vec::new();
    for text in records {
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let run = parse_run(line)?;
            match labels.get(&run.key) {
                None => unlabelled.push(run),
                Some(ls) => {
                    for (tier, class, label, unit, extra) in ls {
                        obs.push(Obs {
                            run: run.clone(),
                            label: label.clone(),
                            unit: unit.clone(),
                            class: class.clone(),
                            tier: tier.clone(),
                            extra_ops: *extra,
                        });
                    }
                }
            }
        }
    }
    Ok((obs, unlabelled))
}

/// A tiny deterministic generator (SplitMix64) so the analysis replays bit
/// for bit from its seed.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }
    pub fn draw(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.draw() % n as u64) as usize
    }
}

/// One unit's runs: X's and Y's `(num, den)` values (S: `(S, 1)`; yield:
/// `(relations, trials)`). X and Y need not have equal counts: a unit is a
/// group of floor curves or a crater curve, and its arms are resampled
/// independently inside it.
#[derive(Clone, Debug, Default)]
pub struct Unit {
    pub x: Vec<(f64, f64)>,
    pub y: Vec<(f64, f64)>,
}

fn ratio(v: &[(f64, f64)]) -> f64 {
    let (n, d) = v.iter().fold((0.0, 0.0), |a, q| (a.0 + q.0, a.1 + q.1));
    n / d
}

fn theta(classes: &[Vec<Unit>]) -> f64 {
    let mut acc = 0.0;
    let mut n = 0.0;
    for units in classes {
        let (mut sx, mut sy, mut nu) = (0.0, 0.0, 0.0);
        for u in units {
            sx += ratio(&u.x);
            sy += ratio(&u.y);
            nu += 1.0;
        }
        if nu > 0.0 {
            acc += (sx / sy).ln();
            n += 1.0;
        }
    }
    (acc / n).exp()
}

fn quantile(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let i = ((v.len() - 1) as f64 * q).round() as usize;
    v[i]
}

pub struct Stat {
    pub theta: f64,
    pub ci95: (f64, f64),
    pub ci90: (f64, f64),
    pub p_less: f64,
    pub p_greater: f64,
    pub units: usize,
    pub runs: usize,
    pub classes: usize,
}

fn resample(v: &[(f64, f64)], rng: &mut Rng) -> Vec<(f64, f64)> {
    (0..v.len()).map(|_| v[rng.below(v.len())]).collect()
}

pub fn statistic(classes: &[Vec<Unit>], boot: usize, perm: usize, seed: u64) -> Stat {
    let th = theta(classes);
    let mut rng = Rng::new(seed);
    let mut bs = Vec::with_capacity(boot);
    for _ in 0..boot {
        let mut cs = Vec::with_capacity(classes.len());
        for _ in 0..classes.len() {
            let units = &classes[rng.below(classes.len())];
            let us: Vec<Unit> = (0..units.len())
                .map(|_| {
                    let u = &units[rng.below(units.len())];
                    Unit {
                        x: resample(&u.x, &mut rng),
                        y: resample(&u.y, &mut rng),
                    }
                })
                .collect();
            cs.push(us);
        }
        bs.push(theta(&cs));
    }
    // Sign flip at the unit level, on the unit means (the statistic only reads them).
    let means: Vec<Vec<(f64, f64)>> = classes
        .iter()
        .map(|us| us.iter().map(|u| (ratio(&u.x), ratio(&u.y))).collect())
        .collect();
    let (mut le, mut ge) = (0usize, 0usize);
    for _ in 0..perm {
        let mut acc = 0.0;
        for us in &means {
            let (mut sx, mut sy) = (0.0, 0.0);
            for &(a, b) in us {
                if rng.draw() & 1 == 1 {
                    sx += b;
                    sy += a;
                } else {
                    sx += a;
                    sy += b;
                }
            }
            acc += (sx / sy).ln();
        }
        let t = (acc / means.len() as f64).exp();
        if t <= th {
            le += 1;
        }
        if t >= th {
            ge += 1;
        }
    }
    Stat {
        theta: th,
        ci95: (quantile(&mut bs, 0.025), quantile(&mut bs, 0.975)),
        ci90: (quantile(&mut bs, 0.05), quantile(&mut bs, 0.95)),
        p_less: (1 + le) as f64 / (perm + 1) as f64,
        p_greater: (1 + ge) as f64 / (perm + 1) as f64,
        units: classes.iter().map(Vec::len).sum(),
        runs: classes
            .iter()
            .flatten()
            .map(|u| u.x.len() + u.y.len())
            .sum(),
        classes: classes.len(),
    }
}

type Index<'a> = BTreeMap<String, BTreeMap<String, Vec<&'a Obs>>>;

/// Measured observations of `(label, method)` in one tier: class → unit → runs.
fn index<'a>(obs: &'a [Obs], tier: &str, label: &str, method: &str) -> Index<'a> {
    let mut m: Index = BTreeMap::new();
    for o in obs
        .iter()
        .filter(|o| !o.run.warmup && o.tier == tier && o.label == label && o.run.arm == method)
    {
        m.entry(o.class.clone())
            .or_default()
            .entry(o.unit.clone())
            .or_default()
            .push(o);
    }
    m
}

fn value(o: &Obs, metric: &str, w: f64, plant: f64) -> Option<(f64, f64)> {
    match metric {
        "yield" => Some((o.run.relations * plant, o.run.trials)),
        _ => o.priced_s(w).map(|s| (s * plant, 1.0)),
    }
}

/// One comparison of `comparisons.json` at every weight it applies to.
pub fn compare(
    obs: &[Obs],
    c: &Value,
    boot: usize,
    perm: usize,
    seed: u64,
) -> Result<Value, String> {
    let tier = c["tier"].as_str().ok_or("comparison tier")?;
    let (xl, xm) = (
        c["x"]["arm"].as_str().ok_or("x.arm")?,
        c["x"]["method"].as_str().ok_or("x.method")?,
    );
    let (yl, ym) = (
        c["y"]["arm"].as_str().ok_or("y.arm")?,
        c["y"]["method"].as_str().ok_or("y.method")?,
    );
    let metric = c["metric"].as_str().unwrap_or("S");
    let plant = c["plant_x"].as_f64().unwrap_or(1.0);
    let only: Option<Vec<String>> = c["classes"].as_array().map(|a| {
        a.iter()
            .filter_map(|x| x.as_str().map(String::from))
            .collect()
    });
    let ix = index(obs, tier, xl, xm);
    let iy = index(obs, tier, yl, ym);
    let weights: Vec<f64> = if metric == "yield" {
        vec![0.0]
    } else {
        WEIGHTS.to_vec()
    };
    let mut per_w = Vec::new();
    let mut censored = 0usize;
    for (wi, &w) in weights.iter().enumerate() {
        let mut classes = Vec::new();
        let mut class_names = Vec::new();
        censored = 0;
        for (class, xu) in &ix {
            if only.as_ref().is_some_and(|o| !o.contains(class)) {
                continue;
            }
            let Some(yu) = iy.get(class) else { continue };
            let mut units = Vec::new();
            for (unit, xs) in xu {
                let Some(ys) = yu.get(unit) else { continue };
                let mut u = Unit::default();
                for (dst, src, pl) in [(&mut u.x, xs, plant), (&mut u.y, ys, 1.0)] {
                    for o in src {
                        if !o.run.ok {
                            censored += 1;
                            continue;
                        }
                        dst.push(value(o, metric, w, pl).ok_or_else(|| {
                            format!("{}: an unpriced counter has no count at w = {w}", c["id"])
                        })?);
                    }
                }
                if !u.x.is_empty() && !u.y.is_empty() {
                    units.push(u);
                }
            }
            if !units.is_empty() {
                classes.push(units);
                class_names.push(class.clone());
            }
        }
        if classes.is_empty() {
            return Err(format!("{}: no unit has runs of both arms", c["id"]));
        }
        let s = statistic(&classes, boot, perm, seed.wrapping_add(wi as u64));
        per_w.push(json!({
            "w": w, "theta": s.theta, "ci95": [s.ci95.0, s.ci95.1], "ci90": [s.ci90.0, s.ci90.1],
            "p_less": s.p_less, "p_greater": s.p_greater,
            "classes": s.classes, "units": s.units, "runs": s.runs, "class_ids": class_names,
        }));
    }
    let alt = c["alternative"].as_str().unwrap_or("less");
    let key = if alt == "greater" {
        "p_greater"
    } else {
        "p_less"
    };
    let p_iut = per_w
        .iter()
        .map(|r| r[key].as_f64().unwrap())
        .fold(0.0, f64::max);
    Ok(json!({
        "id": c["id"], "role": c["role"], "tier": tier, "metric": metric, "alternative": alt,
        "x": {"arm": xl, "method": xm}, "y": {"arm": yl, "method": ym},
        "plant_x": plant, "excluded_unverified_runs": censored,
        "by_weight": per_w, "p_iut": p_iut,
    }))
}

/// Every pair of rho-neg and rho-neg-aa runs on one workload, target and round of a session must
/// cost the same and recover the same scalar (same seeds: a determinism A/A).
pub fn check_determinism(runs: &[Run]) -> Value {
    let mut by: HashMap<(String, String, u64, u64), Vec<&Run>> = HashMap::new();
    for r in runs
        .iter()
        .filter(|r| r.arm == "rho-neg" || r.arm == "rho-neg-aa")
    {
        by.entry((r.session.clone(), r.key.clone(), r.target, r.round))
            .or_default()
            .push(r);
    }
    let mut pairs = 0;
    let mut bad = Vec::new();
    for (k, v) in &by {
        if v.len() == 2 {
            pairs += 1;
            if v[0].total_gae != v[1].total_gae || v[0].recovered != v[1].recovered {
                bad.push(format!("{k:?}"));
            }
        } else {
            bad.push(format!("{k:?}: {} runs", v.len()));
        }
    }
    json!({"id": "check:determinism", "pairs": pairs, "mismatches": bad.len(), "examples": bad.into_iter().take(5).collect::<Vec<_>>(),
           "pass": pairs > 0 && by.values().all(|v| v.len() == 2 && v[0].total_gae == v[1].total_gae && v[0].recovered == v[1].recovered)})
}

/// BSGS exactness and instance pairing: within a tier, class and unit, the
/// ctl-stage A, B and B0 workloads of one target carry the same planted
/// scalar and BSGS spends exactly the same GAE on them in every round.
pub fn check_transport(obs: &[Obs], tier: &str) -> Value {
    let mut cost: HashMap<(String, String, u64), Vec<f64>> = HashMap::new();
    let mut planted: HashMap<(String, String, u64), Vec<Option<u64>>> = HashMap::new();
    for o in obs.iter().filter(|o| {
        o.tier == tier && !o.run.warmup && ["ctlA", "ctlB", "ctlB0"].contains(&o.label.as_str())
    }) {
        let k = (o.class.clone(), o.unit.clone(), o.run.target);
        planted.entry(k.clone()).or_default().push(o.run.planted);
        if o.run.arm == "bsgs-neg" {
            cost.entry(k).or_default().push(o.run.total_gae);
        }
    }
    let distinct = |v: &Vec<f64>| {
        let mut w = v.clone();
        w.sort_by(|a, b| a.partial_cmp(b).unwrap());
        w.dedup();
        w.len()
    };
    let bsgs_bad = cost.values().filter(|v| distinct(v) != 1).count();
    let planted_bad = planted
        .values()
        .filter(|v| v.windows(2).any(|w| w[0] != w[1]) || v.iter().any(Option::is_none))
        .count();
    json!({"id": format!("check:transport:{tier}"), "keys": cost.len(),
           "bsgs_keys_with_more_than_one_cost": bsgs_bad, "keys_with_unequal_planted": planted_bad,
           "pass": !cost.is_empty() && bsgs_bad == 0 && planted_bad == 0})
}

/// Holm over the primary hypotheses, then the verdict vocabulary of PROTOCOL.md §8.
pub fn decide(
    spec: &Value,
    results: &BTreeMap<String, Value>,
    checks: &BTreeMap<String, Value>,
) -> Result<Value, String> {
    let alpha = spec["family_alpha"].as_f64().unwrap_or(0.05);
    let hyps = spec["hypotheses"].as_array().ok_or("hypotheses")?;
    let m = hyps.len();
    let p_of = |id: &str| {
        results
            .get(id)
            .and_then(|r| r["p_iut"].as_f64())
            .ok_or(format!("no result for {id}"))
    };
    let mut order: Vec<(f64, usize)> = hyps
        .iter()
        .enumerate()
        .map(|(i, h)| Ok((p_of(h["comparison"].as_str().unwrap_or(""))?, i)))
        .collect::<Result<_, String>>()?;
    order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut rejected = vec![false; m];
    let mut thresholds = vec![0.0; m];
    let mut stop = false;
    for (rank, &(p, i)) in order.iter().enumerate() {
        thresholds[i] = alpha / (m - rank) as f64;
        if !stop && p < thresholds[i] {
            rejected[i] = true;
        } else {
            stop = true;
        }
    }
    let gate_ok = |g: &Value| -> Result<bool, String> {
        let id = g["id"].as_str().ok_or("gate id")?;
        if id.starts_with("check:") {
            return Ok(checks
                .get(id)
                .map(|c| c["pass"] == json!(true))
                .unwrap_or(false));
        }
        let r = results.get(id).ok_or(format!("gate {id} has no result"))?;
        let w0 = &r["by_weight"][0];
        let (lo, hi) = (
            w0["ci95"][0].as_f64().unwrap(),
            w0["ci95"][1].as_f64().unwrap(),
        );
        Ok(match g["rule"].as_str().unwrap_or("") {
            "ci95_includes_1" => lo <= 1.0 && hi >= 1.0,
            "ci95_excludes_1" => lo > 1.0 || hi < 1.0,
            "p_iut_below" => r["p_iut"].as_f64().unwrap() < g["threshold"].as_f64().unwrap_or(0.0),
            other => return Err(format!("unknown gate rule {other}")),
        })
    };
    let smallest = alpha / m as f64;
    let mut out = Vec::new();
    for (i, h) in hyps.iter().enumerate() {
        let cid = h["comparison"].as_str().unwrap_or("");
        let r = &results[cid];
        let mut failed = Vec::new();
        for g in h["gates"].as_array().cloned().unwrap_or_default() {
            if !gate_ok(&g)? {
                failed.push(g["id"].clone());
            }
        }
        let powered = match h["power_control"].as_str() {
            Some(pc) => results
                .get(pc)
                .and_then(|x| x["p_iut"].as_f64())
                .map(|p| p < smallest)
                .unwrap_or(false),
            None => false,
        };
        let w0 = &r["by_weight"][0];
        let key = if r["alternative"] == "greater" {
            "p_greater"
        } else {
            "p_less"
        };
        let p0 = w0[key].as_f64().unwrap();
        let margin = h["vanish_margin"].as_f64().unwrap_or(0.03);
        let lo90 = w0["ci90"][0].as_f64().unwrap();
        let verdict = if !failed.is_empty() {
            "invalid_measurement"
        } else if rejected[i] {
            "supported_within_scope"
        } else if p0 < thresholds[i] {
            "unresolved_across_band"
        } else if powered && lo90 > 1.0 - margin {
            "vanished_within_margin"
        } else {
            "inconclusive"
        };
        out.push(json!({"id": h["id"], "comparison": cid, "p_iut": r["p_iut"], "holm_threshold": thresholds[i],
                        "holm_rejected": rejected[i], "failed_gates": failed, "power_control_passed": powered,
                        "verdict": verdict}));
    }
    Ok(Value::Array(out))
}

/// Replace every string value under an `arm` key using `map` (masking and unmasking).
pub fn relabel(v: &mut Value, map: &HashMap<String, String>) {
    match v {
        Value::Object(o) => {
            for (k, x) in o.iter_mut() {
                if k == "arm" {
                    if let Value::String(s) = x {
                        if let Some(n) = map.get(s) {
                            *s = n.clone();
                        }
                    }
                } else {
                    relabel(x, map);
                }
            }
        }
        Value::Array(a) => a.iter_mut().for_each(|x| relabel(x, map)),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn synth(classes: usize, units: usize, runs: usize, ratio: f64, seed: u64) -> Vec<Vec<Unit>> {
        let mut rng = Rng::new(seed);
        // Run costs spread like the v2 IC runs (CV near 0.3-0.5).
        let mut draw = |m: f64| (m * (0.5 + (rng.draw() % 1000) as f64 / 1000.0), 1.0);
        (0..classes)
            .map(|_| {
                (0..units)
                    .map(|_| Unit {
                        x: (0..runs).map(|_| draw(77.0 * ratio)).collect(),
                        y: (0..runs).map(|_| draw(77.0)).collect(),
                    })
                    .collect()
            })
            .collect()
    }

    #[test]
    fn a_planted_ten_percent_effect_is_detected_and_a_null_is_not() {
        let s = statistic(&synth(6, 12, 20, 0.90, 1), 300, 999, 7);
        assert!(s.theta < 0.95 && s.ci95.1 < 1.0, "theta {}", s.theta);
        assert!(s.p_less < 0.01);
        let n = statistic(&synth(6, 12, 20, 1.0, 2), 300, 999, 7);
        assert!(n.ci95.0 < 1.0 && n.ci95.1 > 1.0);
        assert!(n.p_less > 0.01);
    }

    #[test]
    fn the_statistic_replays_from_its_seed() {
        let d = synth(3, 5, 8, 1.0, 3);
        let (a, b) = (statistic(&d, 50, 50, 11), statistic(&d, 50, 50, 11));
        assert_eq!((a.ci95, a.p_less), (b.ci95, b.p_less));
    }

    #[test]
    fn unpriced_operations_follow_the_band_rules() {
        let r = json!({"cost": {"unpriced": ["lookups_uncharged", "sqrt_solves_uncharged", "residual_table_bytes_uncharged"]},
                       "counters": {"lookups_uncharged": 5},
                       "phases": [{"native": {"sqrt_solves": 3}}, {"native": {"sqrt_solves_uncharged": 4}}]});
        assert_eq!(unpriced_ops(&r).unwrap(), Some(12));
        let missing =
            json!({"cost": {"unpriced": ["row_ops_uncharged"]}, "counters": {}, "phases": []});
        assert_eq!(unpriced_ops(&missing).unwrap(), None);
    }

    #[test]
    fn holm_and_verdicts() {
        let res = |p: f64, p0: f64, lo90: f64| json!({"p_iut": p, "alternative": "less", "by_weight": [{"p_less": p0, "ci90": [lo90, 1.1], "ci95": [lo90 - 0.01, 1.1]}]});
        let mut results = BTreeMap::new();
        results.insert("c1".to_string(), res(0.001, 0.001, 0.9));
        results.insert("c2".to_string(), res(0.2, 0.001, 0.9));
        results.insert("c3".to_string(), res(0.5, 0.5, 0.985));
        results.insert("pc".to_string(), res(0.0001, 0.0001, 0.9));
        let spec = json!({"family_alpha": 0.05, "hypotheses": [
            {"id": "H1", "comparison": "c1"}, {"id": "H2", "comparison": "c2"},
            {"id": "H3", "comparison": "c3", "power_control": "pc", "vanish_margin": 0.03}]});
        let d = decide(&spec, &results, &BTreeMap::new()).unwrap();
        let v: Vec<&str> = d
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x["verdict"].as_str().unwrap())
            .collect();
        assert_eq!(
            v,
            [
                "supported_within_scope",
                "unresolved_across_band",
                "vanished_within_margin"
            ]
        );
    }

    #[test]
    fn relabel_touches_only_arm_fields() {
        let mut v = json!({"x": {"arm": "B", "method": "B"}, "id": "B"});
        relabel(
            &mut v,
            &HashMap::from([("B".to_string(), "K07".to_string())]),
        );
        assert_eq!(v, json!({"x": {"arm": "K07", "method": "B"}, "id": "B"}));
    }
}
