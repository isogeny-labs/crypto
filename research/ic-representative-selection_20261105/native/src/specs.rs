//! Frozen ecbench job specs and their workload labels.
//!
//! One class directory (written by `volcano-rho --ecbench`, plus the
//! `transport.jsonl` of `repsel ascend`) becomes the jobs of each stage:
//!
//! | stage | workloads | arms | measurement seed |
//! |:--|:--|:--|:--|
//! | `ab` | per floor curve of a group: A (itself) and B0 (its sibling) | rho-neg, rho-neg-aa, ic-mitm | main |
//! | `b` | per group: B (the crater instance every member ascends to), more targets | same | main |
//! | `aa` | A only (label `Aaa`) | same | aa |
//! | `ctl` | A, B, B0 (labels `ctlA`, `ctlB`, `ctlB0`), two targets | + bsgs-neg, ic-mitm-plus1 | main |
//! | `cal` | every crater curve in every model (label `cal`) | rho-neg, rho-neg-aa, ic-mitm | main |
//! | `hold` | per crater curve: model 0, the IC-selected and the rho-selected model | same | main |
//! | `holdaa` | model 0 only (label `Cdefaa`) | same | holdaa |
//!
//! Every spec asks for L2, five measured rounds, one warmup, alternate arm
//! order and a reference plus an A/A control, as the Iso Lab ecbench
//! contract requires; each is checked against the 64 KiB request bound and
//! against the job budget of the frozen cost model.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::{json, Value};

pub const MAX_SPEC_BYTES: usize = 60_000;

fn u(v: &Value, k: &str) -> Result<u64, String> {
    v.get(k)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing integer field {k}"))
}

fn tier_cfg<'a>(design: &'a Value, tier: &str) -> Result<&'a Value, String> {
    design["tiers"]
        .get(tier)
        .ok_or_else(|| format!("design.json has no tier {tier}"))
}

fn ic(size: u64) -> Value {
    json!({"id": "ic.pipeline", "params": {
        "factor_base": format!("prime-abscissa:size={size}"), "oracle": "mitm:negation_folded=1"}})
}

pub fn arms(stage: &str, ic_size: u64) -> Value {
    let mut a = vec![
        json!({"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}),
        json!({"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}}),
    ];
    if stage == "ctl" {
        a.push(json!({"name": "bsgs-neg", "role": "baseline", "method": {"id": "bsgs.negation"}}));
    }
    a.push(json!({"name": "ic-mitm", "role": "candidate", "method": ic(ic_size)}));
    if stage == "ctl" {
        a.push(json!({"name": "ic-mitm-plus1", "role": "candidate", "method": ic(ic_size + 1)}));
    }
    Value::Array(a)
}

/// Estimated measurement seconds of one job under the frozen cost model.
pub fn estimate(cfg: &Value, stage: &str, workloads: u64) -> f64 {
    let c = &cfg["est_seconds_per_run"];
    let f = |k: &str| c[k].as_f64().unwrap_or(f64::NAN);
    let per = 2.0 * f("rho")
        + f("ic")
        + if stage == "ctl" {
            f("bsgs") + f("ic")
        } else {
            0.0
        };
    let runs_per_arm =
        (cfg["rounds"].as_u64().unwrap_or(5) + cfg["warmup"].as_u64().unwrap_or(1)) as f64;
    workloads as f64 * runs_per_arm * per
}

struct Job {
    name: String,
    stage: String,
    curves: Vec<Value>,
    labels: Vec<Value>,
    targets: u64,
    target_seed: u64,
    measurement_seed: u64,
}

fn workload(p: u64, order: u64, r: u64, a: u64, b: u64, gx: u64, gy: u64) -> Value {
    json!({"kind": "prime_explicit", "p": p, "a": a, "b": b,
           "group_order": order, "r": r, "gx": gx, "gy": gy})
}

#[allow(clippy::too_many_arguments)]
fn label(tier: &str, class: &str, stage: &str, job: &str, w: &Value, arms: Vec<Value>) -> Value {
    json!({"schema": "repsel.label/v1", "tier": tier, "class": class, "stage": stage, "job": job,
           "workload": {"p": w["p"], "a": w["a"], "b": w["b"], "gx": w["gx"], "gy": w["gy"], "r": w["r"]},
           "labels": arms})
}

fn arm_label(arm: &str, unit: &str, model: u64, extra_ops: u64) -> Value {
    json!({"arm": arm, "unit": unit, "model": model, "extra_ops": extra_ops})
}

pub struct ClassInput {
    pub rows: Vec<Value>,
    pub transport: Vec<Value>,
    pub p: u64,
    pub order: u64,
    pub r: u64,
    pub depth: u64,
}

pub fn load_class(dir: &Path) -> Result<ClassInput, String> {
    let read = |n: &str| {
        fs::read_to_string(dir.join(n)).map_err(|e| format!("{}/{n}: {e}", dir.display()))
    };
    let spec: Value = serde_json::from_str(&read("spec.json")?).map_err(|e| e.to_string())?;
    let c0 = &spec["workloads"]["curves"][0];
    let rows = read("curves.jsonl")?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Value>, _>>()?;
    let transport = match fs::read_to_string(dir.join("transport.jsonl")) {
        Ok(t) => t
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).map_err(|e| e.to_string()))
            .collect::<Result<Vec<Value>, _>>()?,
        Err(_) => Vec::new(),
    };
    let depth = rows
        .iter()
        .filter_map(|x| x["level"].as_u64())
        .max()
        .ok_or("empty curves.jsonl")?;
    Ok(ClassInput {
        rows,
        transport,
        p: u(c0, "p")?,
        order: u(c0, "group_order")?,
        r: u(c0, "r")?,
        depth,
    })
}

/// Floor vertices (model 0, BFS order, capped) and crater vertices (capped).
pub fn units(ci: &ClassInput, floor_cap: usize, crater_cap: usize) -> (Vec<u64>, Vec<u64>) {
    let mut floor = Vec::new();
    let mut crater = Vec::new();
    for x in &ci.rows {
        let (lv, m, v) = (
            x["level"].as_u64(),
            x["model"].as_u64(),
            x["vertex"].as_u64(),
        );
        if m != Some(0) {
            continue;
        }
        if lv == Some(ci.depth) && floor.len() < floor_cap {
            floor.push(v.unwrap());
        }
        if lv == Some(0) && crater.len() < crater_cap {
            crater.push(v.unwrap());
        }
    }
    (floor, crater)
}

fn row(ci: &ClassInput, vertex: u64, model: u64) -> Result<&Value, String> {
    ci.rows
        .iter()
        .find(|x| x["vertex"].as_u64() == Some(vertex) && x["model"].as_u64() == Some(model))
        .ok_or_else(|| format!("vertex {vertex} model {model} is not in curves.jsonl"))
}

/// Floor curves grouped by the crater instance arm B lands on. Every floor
/// descendant of one crater vertex ascends to the same crater model and the
/// same generator image, so B has one workload per group and the group (not
/// the floor curve) is the analysis unit of arms A, B and B0. Groups are
/// ordered by their first floor vertex in BFS order.
pub struct Group {
    pub unit: String,
    pub members: Vec<u64>,
}

pub fn groups(ci: &ClassInput, floor: &[u64]) -> Result<Vec<Group>, String> {
    let mut out: Vec<(String, Group)> = Vec::new();
    for &v in floor {
        let t = transport(ci, v)?;
        let key = format!(
            "{}:{}:{}:{}",
            t["B"]["a"], t["B"]["b"], t["B"]["gx"], t["B"]["gy"]
        );
        match out.iter_mut().find(|(k, _)| *k == key) {
            Some((_, g)) => g.members.push(v),
            None => out.push((
                key,
                Group {
                    unit: format!("g{v}"),
                    members: vec![v],
                },
            )),
        }
    }
    Ok(out.into_iter().map(|(_, g)| g).collect())
}

fn transport(ci: &ClassInput, v: u64) -> Result<&Value, String> {
    ci.transport
        .iter()
        .find(|t| t["floor_vertex"].as_u64() == Some(v))
        .ok_or_else(|| format!("no transport row for floor vertex {v}: run repsel ascend"))
}

fn floor_jobs(
    ci: &ClassInput,
    cfg: &Value,
    tier: &str,
    class: &str,
    stage: &str,
    gs: &[Group],
) -> Result<Vec<Job>, String> {
    let shard = cfg["shard"][stage].as_u64().ok_or("missing shard size")? as usize;
    let (tk, mk) = match stage {
        "ab" => ("ab", "main"),
        "b" => ("b", "main"),
        "aa" => ("ab", "aa"),
        "ctl" => ("ctl", "main"),
        _ => unreachable!(),
    };
    let targets = cfg["targets"][tk].as_u64().ok_or("targets")?;
    let tseed = cfg["target_seed"]["ab"].as_u64().ok_or("target_seed")?;
    let tseed = if stage == "ctl" {
        cfg["target_seed"]["ctl"].as_u64().ok_or("target_seed")?
    } else {
        tseed
    };
    let mseed = cfg["measurement_seed"][mk]
        .as_u64()
        .ok_or("measurement_seed")?;
    let pre = if stage == "ctl" { "ctl" } else { "" };
    let mut jobs = Vec::new();
    for (k, chunk) in gs.chunks(shard).enumerate() {
        let name = format!("repsel-{class}-{stage}-{k}");
        let mut curves = Vec::new();
        let mut labels = Vec::new();
        let mut push = |w: Value, arm: String, unit: &str, ops: u64| {
            labels.push(label(
                tier,
                class,
                stage,
                &name,
                &w,
                vec![arm_label(&arm, unit, 0, ops)],
            ));
            curves.push(w);
        };
        for g in chunk {
            if stage == "b" || stage == "ctl" {
                // One B workload per group, charged the largest ascent among its members.
                let t = transport(ci, g.members[0])?;
                let e = &t["B"];
                let mut ops = 0;
                for &v in &g.members {
                    ops = ops.max(u(&transport(ci, v)?["B"], "ops_total")?);
                }
                push(
                    workload(
                        ci.p,
                        ci.order,
                        ci.r,
                        u(e, "a")?,
                        u(e, "b")?,
                        u(e, "gx")?,
                        u(e, "gy")?,
                    ),
                    format!("{pre}B"),
                    &g.unit,
                    ops,
                );
            }
            if stage == "b" {
                continue;
            }
            for &v in &g.members {
                let src = row(ci, v, 0)?;
                let a = workload(
                    ci.p,
                    ci.order,
                    ci.r,
                    u(src, "a")?,
                    u(src, "b")?,
                    u(src, "gx")?,
                    u(src, "gy")?,
                );
                push(
                    a,
                    if stage == "aa" {
                        "Aaa".into()
                    } else {
                        format!("{pre}A")
                    },
                    &g.unit,
                    0,
                );
                if stage == "aa" {
                    continue;
                }
                let e = &transport(ci, v)?["B0"];
                push(
                    workload(
                        ci.p,
                        ci.order,
                        ci.r,
                        u(e, "a")?,
                        u(e, "b")?,
                        u(e, "gx")?,
                        u(e, "gy")?,
                    ),
                    format!("{pre}B0"),
                    &g.unit,
                    u(e, "ops_total")?,
                );
            }
        }
        jobs.push(Job {
            name,
            stage: stage.into(),
            curves,
            labels,
            targets,
            target_seed: tseed,
            measurement_seed: mseed,
        });
    }
    Ok(jobs)
}

fn cal_jobs(
    ci: &ClassInput,
    cfg: &Value,
    tier: &str,
    class: &str,
    crater: &[u64],
) -> Result<Vec<Job>, String> {
    let shard = cfg["shard"]["cal"].as_u64().ok_or("missing shard size")? as usize;
    let models = cfg["models"].as_u64().ok_or("models")?;
    let mut jobs = Vec::new();
    for (k, chunk) in crater.chunks(shard).enumerate() {
        let name = format!("repsel-{class}-cal-{k}");
        let mut curves = Vec::new();
        let mut labels = Vec::new();
        for &v in chunk {
            for m in 0..models {
                let x = row(ci, v, m)?;
                let w = workload(
                    ci.p,
                    ci.order,
                    ci.r,
                    u(x, "a")?,
                    u(x, "b")?,
                    u(x, "gx")?,
                    u(x, "gy")?,
                );
                curves.push(w.clone());
                labels.push(label(
                    tier,
                    class,
                    "cal",
                    &name,
                    &w,
                    vec![arm_label("cal", &format!("c{v}"), m, 0)],
                ));
            }
        }
        jobs.push(Job {
            name,
            stage: "cal".into(),
            curves,
            labels,
            targets: cfg["targets"]["cal"].as_u64().ok_or("targets")?,
            target_seed: cfg["target_seed"]["cal"].as_u64().ok_or("target_seed")?,
            measurement_seed: cfg["measurement_seed"]["main"].as_u64().ok_or("seed")?,
        });
    }
    Ok(jobs)
}

/// Charged native operations of the change of model `(x, y) ↦ (u²x, u³y)`
/// for `G` and `Q`: `u²`, `u³` and two products per point.
pub const ISO_OPS: u64 = 6;

fn hold_jobs(
    ci: &ClassInput,
    cfg: &Value,
    tier: &str,
    class: &str,
    sel: &Value,
    aa: bool,
) -> Result<Vec<Job>, String> {
    let stage = if aa { "holdaa" } else { "hold" };
    let shard = cfg["shard"][stage].as_u64().ok_or("missing shard size")? as usize;
    let picks = sel["selection"]
        .as_array()
        .ok_or("selection.json has no selection")?;
    let mut jobs = Vec::new();
    for (k, chunk) in picks.chunks(shard).enumerate() {
        let name = format!("repsel-{class}-{stage}-{k}");
        let mut curves = Vec::new();
        let mut labels = Vec::new();
        for pick in chunk {
            let v = u(pick, "vertex")?;
            let unit = format!("c{v}");
            let (mi, mr) = (u(pick, "ic_model")?, u(pick, "rho_model")?);
            let models: BTreeSet<u64> = if aa { [0].into() } else { [0, mi, mr].into() };
            for m in models {
                let x = row(ci, v, m)?;
                let w = workload(
                    ci.p,
                    ci.order,
                    ci.r,
                    u(x, "a")?,
                    u(x, "b")?,
                    u(x, "gx")?,
                    u(x, "gy")?,
                );
                let ops = if m == 0 { 0 } else { ISO_OPS };
                let mut ls = Vec::new();
                if aa {
                    ls.push(arm_label("Cdefaa", &unit, 0, 0));
                } else {
                    if m == 0 {
                        ls.push(arm_label("Cdef", &unit, 0, 0));
                    }
                    if m == mi {
                        ls.push(arm_label("Csel", &unit, m, ops));
                    }
                    if m == mr {
                        ls.push(arm_label("Crho", &unit, m, ops));
                    }
                }
                curves.push(w.clone());
                labels.push(label(tier, class, stage, &name, &w, ls));
            }
        }
        jobs.push(Job {
            name,
            stage: stage.into(),
            curves,
            labels,
            targets: cfg["targets"]["hold"].as_u64().ok_or("targets")?,
            target_seed: cfg["target_seed"]["hold"].as_u64().ok_or("target_seed")?,
            measurement_seed: cfg["measurement_seed"][if aa { "holdaa" } else { "main" }]
                .as_u64()
                .ok_or("seed")?,
        });
    }
    Ok(jobs)
}

fn spec_of(job: &Job, cfg: &Value, tier: &str) -> Value {
    json!({
        "schema": "ecbench.spec/v1",
        "label": format!("ic representative selection {tier}-bit: {}", job.name),
        "description": "research/ic-representative-selection_20261105 (PROTOCOL.md); written by repsel specs",
        "workloads": {"curves": job.curves, "targets_per_curve": job.targets,
                      "target_seed": job.target_seed, "target_kind": "planted_shared"},
        "arms": arms(&job.stage, cfg["ic_size"].as_u64().unwrap_or(32)),
        "measurement": {"rounds": cfg["rounds"], "warmup": cfg["warmup"], "order": "alternate",
                        "seed": job.measurement_seed, "isolation_required": "L2",
                        "timeout_seconds": cfg["timeout_seconds"]},
    })
}

/// Write each job's spec, its labels and a `jobs.tsv` line; refuse a job
/// over the byte bound or the frozen time budget, and any duplicate workload.
fn write_jobs(jobs: Vec<Job>, cfg: &Value, tier: &str, out: &Path) -> Result<Vec<String>, String> {
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let budget = cfg["job_budget_seconds"]
        .as_f64()
        .ok_or("job_budget_seconds")?;
    let mut lines = Vec::new();
    for job in jobs {
        let mut seen = BTreeSet::new();
        for c in &job.curves {
            if !seen.insert(c.to_string()) {
                return Err(format!("{}: duplicate workload {c}", job.name));
            }
        }
        let spec = spec_of(&job, cfg, tier);
        let text = serde_json::to_string(&spec).unwrap();
        if text.len() > MAX_SPEC_BYTES {
            return Err(format!(
                "{}: spec is {} bytes (> {MAX_SPEC_BYTES})",
                job.name,
                text.len()
            ));
        }
        let est = estimate(cfg, &job.stage, job.curves.len() as u64 * job.targets);
        if est > budget {
            return Err(format!(
                "{}: estimated {est:.0} s exceeds the {budget} s job budget",
                job.name
            ));
        }
        fs::write(out.join(format!("{}.json", job.name)), text + "\n")
            .map_err(|e| e.to_string())?;
        let mut lab = String::new();
        for l in &job.labels {
            lab += &l.to_string();
            lab.push('\n');
        }
        fs::write(out.join(format!("{}.labels.jsonl", job.name)), lab)
            .map_err(|e| e.to_string())?;
        lines.push(format!(
            "{}\t{}\t{}\t{}\t{:.0}",
            job.name,
            tier,
            job.stage,
            job.curves.len() as u64 * job.targets,
            est
        ));
    }
    Ok(lines)
}

/// Stages `ab`, `aa`, `ctl` and (for a class in the tier's (c) subset) `cal`.
pub fn pre_selection(
    class_dir: &Path,
    design: &Value,
    tier: &str,
    class: &str,
    with_c: bool,
    out: &Path,
) -> Result<Vec<String>, String> {
    let cfg = tier_cfg(design, tier)?;
    let ci = load_class(class_dir)?;
    let (floor, crater) = units(
        &ci,
        usize::MAX,
        cfg["crater_cap"].as_u64().unwrap_or(16) as usize,
    );
    let mut gs = groups(&ci, &floor)?;
    gs.truncate(cfg["group_cap"].as_u64().unwrap_or(8) as usize);
    let (gmin, cmin) = (
        cfg["group_min"].as_u64().unwrap_or(6) as usize,
        cfg["crater_min"].as_u64().unwrap_or(8) as usize,
    );
    if gs.len() < gmin || crater.len() < cmin {
        return Err(format!(
            "{class}: {} floor groups and {} crater curves, below the acceptance rule",
            gs.len(),
            crater.len()
        ));
    }
    let mut jobs = Vec::new();
    for stage in ["ab", "b", "aa", "ctl"] {
        jobs.extend(floor_jobs(&ci, cfg, tier, class, stage, &gs)?);
    }
    if with_c {
        jobs.extend(cal_jobs(&ci, cfg, tier, class, &crater)?);
    }
    write_jobs(jobs, cfg, tier, out)
}

/// Stages `hold` and `holdaa` from a frozen `selection.json`.
pub fn post_selection(
    class_dir: &Path,
    design: &Value,
    tier: &str,
    class: &str,
    selection: &Value,
    out: &Path,
) -> Result<Vec<String>, String> {
    let cfg = tier_cfg(design, tier)?;
    let ci = load_class(class_dir)?;
    let mut jobs = hold_jobs(&ci, cfg, tier, class, selection, false)?;
    jobs.extend(hold_jobs(&ci, cfg, tier, class, selection, true)?);
    write_jobs(jobs, cfg, tier, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arms_meet_the_isolab_contract() {
        for stage in ["ab", "ctl", "cal"] {
            let a = arms(stage, 32);
            let roles: Vec<&str> = a
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x["role"].as_str().unwrap())
                .collect();
            assert_eq!(roles[0], "reference");
            // run-ecbench pairs the first reference with the first control: rho-neg-aa.
            let first_control = a
                .as_array()
                .unwrap()
                .iter()
                .find(|x| x["role"] == "control")
                .unwrap();
            assert_eq!(first_control["name"], "rho-neg-aa");
            assert_eq!(roles.iter().filter(|r| **r == "control").count(), 1);
        }
    }

    #[test]
    fn cost_model_counts_every_arm_round_and_warmup() {
        let cfg = json!({"rounds": 5, "warmup": 1, "est_seconds_per_run": {"ic": 0.15, "rho": 0.1, "bsgs": 0.1}});
        assert!((estimate(&cfg, "ab", 100) - 100.0 * 6.0 * 0.35).abs() < 1e-9);
        assert!((estimate(&cfg, "ctl", 100) - 100.0 * 6.0 * 0.6).abs() < 1e-9);
    }
}
