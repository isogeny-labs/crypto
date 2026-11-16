//! Cold public-input processes on the restricted weak Fp6 cover family.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Key = (u64, u64, String, u64, String);
const ARMS: [&str; 6] = ["ic_a", "graph_a", "rho_a", "ic_b", "graph_b", "rho_b"];
const METHODS: [&str; 3] = ["ic", "graph", "rho"];
pub(crate) fn int(v: &Value, k: &str) -> Result<u64, String> {
    v[k].as_u64().ok_or_else(|| format!("missing integer {k}"))
}
fn key(v: &Value) -> Result<Key, String> {
    Ok((
        int(v, "p")?,
        int(v, "seed")?,
        v["phase"].as_str().ok_or("phase")?.into(),
        int(v, "round")?,
        v["arm"].as_str().ok_or("arm")?.into(),
    ))
}
pub(crate) fn context(v: &Value) -> Value {
    json!([
        v["family"],
        v["input_digest"],
        v["retained_percent"],
        v["rho_options"]
    ])
}
pub(crate) fn clean(v: &mut Value) {
    if let Some(o) = v.as_object_mut() {
        o.remove("wall_ms");
        o.remove("filter_call_wall_ns");
        for v in o.values_mut() {
            clean(v)
        }
    } else if let Some(a) = v.as_array_mut() {
        for v in a {
            clean(v)
        }
    }
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.0
    }
}
fn estimate(logs: &[f64], draws: usize, mut seed: u64, complete: bool) -> Value {
    if logs.is_empty() {
        return Value::Null;
    }
    let mean = (logs.iter().sum::<f64>() / logs.len() as f64).exp();
    if !complete {
        return json!({"geometric_mean":mean,"bootstrap_95":null,"rounds":logs.len(),"interval_status":"incomplete_no_usable_interval"});
    }
    let mut boot = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut sum = 0.0;
        for _ in logs {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            sum += logs[seed as usize % logs.len()]
        }
        boot.push((sum / logs.len() as f64).exp())
    }
    boot.sort_by(f64::total_cmp);
    json!({"geometric_mean":mean,"bootstrap_95":[boot[draws*25/1000],boot[(draws*975/1000).min(draws-1)]],"rounds":logs.len()})
}
pub(crate) fn public_family(f: &Value, p: u64) -> bool {
    let l = f["l"].as_u64();
    let coords = |v: &Value| {
        v.as_array().is_some_and(|v| {
            v.len() == 3
                && v.iter().all(|a| {
                    a.as_array().is_some_and(|a| {
                        a.len() == 2 && a.iter().all(|n| n.as_u64().is_some_and(|n| n < p))
                    })
                })
        })
    };
    f["p"] == p
        && l.is_some_and(|l| (3..1u64 << 63).contains(&l) && l % 2 == 1)
        && coords(&f["alpha"])
        && ["g", "q"]
            .iter()
            .all(|k| f[*k]["inf"] == false && coords(&f[*k]["x"]) && coords(&f[*k]["y"]))
        && f.get("d").is_none()
        && f.get("expected_scalar").is_none()
}
pub(crate) fn valid_graph(g: &Value) -> bool {
    let keys = [
        "calls",
        "projected",
        "refusals",
        "fallback_solves",
        "replayed_rows",
        "input_rows",
        "output_rows",
    ];
    if keys.iter().any(|k| int(g, k).is_err()) {
        return false;
    }
    let calls = int(g, "calls").unwrap();
    calls > 0
        && int(g, "projected")
            .unwrap()
            .checked_add(int(g, "refusals").unwrap())
            == Some(calls)
        && int(g, "fallback_solves").unwrap() <= calls
        && g["replayed_rows"] == g["output_rows"]
        && g["output_rows"].as_u64() <= g["input_rows"].as_u64()
        && g["input_digest"]
            .as_str()
            .is_some_and(|s| s.len() == 64 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        && g["counter_scope"]
            .as_str()
            .is_some_and(|s| s.contains("omit"))
}
fn analyze(plan: &Value, rows: Vec<Value>, parse_errors: usize) -> Result<Value, String> {
    let source = plan["source_revision"]
        .as_str()
        .filter(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        .ok_or("source SHA required")?;
    let primes = plan["primes"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("primes")?;
    let seeds = plan["seeds"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("seeds")?;
    let rounds = int(plan, "rounds")?;
    let warm = int(plan, "warmup_rounds")?;
    let draws = int(plan, "bootstrap_draws")? as usize;
    let seed = int(plan, "bootstrap_seed")?;
    let tolerance = plan["aa_tolerance"]
        .as_f64()
        .filter(|x| x.is_finite() && *x >= 1.0)
        .ok_or("AA tolerance")?;
    if rounds < 5
        || warm == 0
        || draws < 1000
        || seed == 0
        || plan["driver_kind"] != "cover-sieve-graph"
        || plan["retained_percent"] != 75
        || plan["max_small_weight"] != 4096
        || plan["max_sources"] != 4096
    {
        return Err("invalid frozen cover plan".into());
    }
    let mut cases = Vec::new();
    let mut schedule = Vec::new();
    for p in primes {
        let p = p
            .as_u64()
            .filter(|p| (5..=1823).contains(p))
            .ok_or("prime fixture")?;
        for seed in seeds {
            let seed = seed.as_u64().ok_or("seed")?;
            if cases.contains(&(p, seed)) {
                return Err("duplicate planned case".into());
            }
            cases.push((p, seed));
            for (phase, n) in [("warmup", warm), ("measurement", rounds)] {
                for round in 0..n {
                    let mut arms = ARMS;
                    if round % 2 == 1 {
                        arms.reverse()
                    }
                    for arm in arms {
                        schedule.push((p, seed, phase.into(), round, arm.into()))
                    }
                }
            }
        }
    }
    let mut global = BTreeSet::new();
    if parse_errors > 0 {
        global.insert(format!("{parse_errors} malformed lines"));
    }
    let mut fixtures = BTreeMap::new();
    let mut executions = BTreeMap::new();
    let mut starts = Vec::new();
    let mut completions = Vec::new();
    let mut pending = None;
    let mut raw_count = 0;
    for row in rows {
        if (plan.get("sieve_recipe").is_some() && row["sieve_recipe"] != plan["sieve_recipe"])
            || (plan.get("worker_timeout_seconds").is_some()
                && row["worker_timeout_seconds"] != plan["worker_timeout_seconds"])
            || (plan.get("sieve_recipe").is_some() && row["validation_only"] != false)
        {
            global.insert("recipe/timeout/comparison mode mismatch".into());
        }
        if row["schema"] != "cover.sieve.graph-process/v1" || row["source_revision"] != source {
            global.insert("schema/source mismatch".into());
        }
        match row["record_type"].as_str() {
            Some("fixture") => {
                let c = (int(&row, "p")?, int(&row, "seed")?);
                if !cases.contains(&c) || fixtures.insert(c, row).is_some() {
                    global.insert("unexpected/duplicate fixture".into());
                }
            }
            Some("execution_start") => {
                let k = key(&row)?;
                if pending.is_some() {
                    global.insert("overlapping starts".into());
                }
                if fixtures
                    .get(&(k.0, k.1))
                    .is_none_or(|f| context(f) != context(&row))
                {
                    global.insert("start family identity mismatch".into());
                }
                pending = Some(k.clone());
                starts.push(k);
            }
            Some("execution") => {
                let k = key(&row)?;
                raw_count += 1;
                if pending.as_ref() != Some(&k) {
                    global.insert("completion without matching ordered start".into());
                }
                pending = None;
                if fixtures
                    .get(&(k.0, k.1))
                    .is_none_or(|f| context(f) != context(&row))
                {
                    global.insert("completion family identity mismatch".into());
                }
                completions.push(k.clone());
                if executions.insert(k, row).is_some() {
                    global.insert("duplicate completion".into());
                }
            }
            _ => {
                global.insert("unknown record type".into());
            }
        }
    }
    if starts.len() > schedule.len()
        || completions.len() > schedule.len()
        || starts != schedule[..starts.len().min(schedule.len())]
        || completions != schedule[..completions.len().min(schedule.len())]
        || starts.len() < completions.len()
        || starts.len() > completions.len() + 1
    {
        global.insert("frozen ordered schedule mismatch".into());
    }
    let mut prep = 0u128;
    let mut seen_families = BTreeSet::new();
    for (&(p, _), f) in &fixtures {
        let family = &f["family"];
        let digest = blake3::hash(&serde_json::to_vec(family).map_err(|e| e.to_string())?)
            .to_hex()
            .to_string();
        if !seen_families.insert(digest.clone()) {
            global.insert("duplicate public challenge under different case labels".into());
        }
        let l = family["l"].as_u64().unwrap_or(0);
        let dp = if l > 0 {
            16.min((l.ilog2() / 2).saturating_sub(4))
        } else {
            0
        };
        if !public_family(family, p)
            || f["input_digest"] != digest
            || f["retained_percent"] != plan["retained_percent"]
            || f["expected_scalar"]
                .as_u64()
                .is_none_or(|d| d == 0 || d >= l)
            || f["rho_options"] != json!({"negation":true,"batch":16,"dp_bits":dp,"r":256})
        {
            global.insert("invalid public fixture/digest/options/held-out answer".into());
        }
        prep += int(f, "challenge_preparation_ns")? as u128;
    }
    let mut results = Vec::new();
    for &(p, case_seed) in &cases {
        let mut reasons = BTreeSet::new();
        let fixture = fixtures.get(&(p, case_seed));
        if fixture.is_none() {
            reasons.insert("missing public fixture".to_string());
        }
        let expected = fixture
            .map(|f| f["expected_scalar"].clone())
            .unwrap_or(Value::Null);
        let mut signatures: [Option<Value>; 3] = [None, None, None];
        let mut outer_aa: [Vec<f64>; 3] = Default::default();
        let mut inner_aa: [Vec<f64>; 3] = Default::default();
        let mut outer_logs: [Vec<f64>; 3] = Default::default();
        let mut inner_logs: [Vec<f64>; 3] = Default::default();
        let mut first_work = json!({});
        let mut complete = 0;
        let mut correct = 0;
        let mut graph_used = true;
        for (phase, n) in [("warmup", warm), ("measurement", rounds)] {
            for round in 0..n {
                let mut outer = [0.0; 6];
                let mut inner = [0.0; 6];
                let mut all = true;
                let mut valid = true;
                for (i, arm) in ARMS.iter().enumerate() {
                    let k = (p, case_seed, phase.into(), round, (*arm).into());
                    let Some(row) = executions.get(&k) else {
                        all = false;
                        valid = false;
                        continue;
                    };
                    let w = &row["worker"];
                    let method = i % 3;
                    let work = &w["work"];
                    let mut cleaned = work.clone();
                    clean(&mut cleaned);
                    let signature = json!({"scalar":w["recovered_scalar"],"verified":w["verified"],"capped":w["capped"],"work":cleaned});
                    outer[i] = row["cold_process_wall_ns"]
                        .as_f64()
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .unwrap_or(0.0);
                    inner[i] = w["inner_wall_ns"]
                        .as_f64()
                        .filter(|v| v.is_finite() && *v > 0.0)
                        .unwrap_or(0.0);
                    let native = match method {
                        0 => {
                            work["solved"] == true
                                && work["correct"] == false
                                && work["graph_filter"].is_null()
                        }
                        1 => {
                            work["solved"] == true
                                && work["correct"] == false
                                && valid_graph(&work["graph_filter"])
                        }
                        _ => {
                            work["run"]["correct"] == false
                                && work["verified"] == true
                                && work["run"]["capped"] == false
                                && fixture.is_some_and(|f| {
                                    let opts = &f["rho_options"];
                                    ["negation", "batch", "dp_bits", "r"]
                                        .iter()
                                        .all(|k| work["run"][*k] == opts[*k])
                                })
                        }
                    };
                    valid &= row["valid"] == true
                        && row["child_exit"] == 0
                        && w["method"] == METHODS[method]
                        && w["solver_answer_placeholder"] == 0
                        && w["recovered_scalar"] == expected
                        && w["verified"] == true
                        && w["capped"] == false
                        && w["signature"] == signature
                        && native
                        && outer[i] >= inner[i]
                        && inner[i] > 0.0;
                    if signatures[method]
                        .as_ref()
                        .is_some_and(|old| *old != signature)
                    {
                        reasons.insert("same-method answer/semantic A/A mismatch".into());
                    }
                    signatures[method].get_or_insert(signature);
                    if phase == "measurement" {
                        if first_work.get(METHODS[method]).is_none() {
                            first_work[METHODS[method]] = work.clone();
                        }
                        if method == 1 {
                            let g = &work["graph_filter"];
                            graph_used &= g["projected"].as_u64().is_some_and(|v| v > 0)
                                && g["output_rows"].as_u64() < g["input_rows"].as_u64();
                        }
                    }
                }
                if !all {
                    reasons.insert(format!("incomplete {phase} round {round}"));
                }
                if !valid {
                    reasons.insert(format!("invalid {phase} round {round}"));
                }
                if phase == "measurement" && all {
                    complete += 1;
                    correct += usize::from(valid);
                    if outer.iter().chain(inner.iter()).all(|v| *v > 0.0) {
                        for method in 0..3 {
                            outer_aa[method].push(outer[method] / outer[method + 3]);
                            inner_aa[method].push(inner[method] / inner[method + 3]);
                        }
                        let pairs = [(1, 0), (1, 2), (0, 2)];
                        for (pair, (a, b)) in pairs.iter().copied().enumerate() {
                            outer_logs[pair].push(
                                0.5 * (outer[a].ln() + outer[a + 3].ln()
                                    - outer[b].ln()
                                    - outer[b + 3].ln()),
                            );
                            inner_logs[pair].push(
                                0.5 * (inner[a].ln() + inner[a + 3].ln()
                                    - inner[b].ln()
                                    - inner[b + 3].ln()),
                            );
                        }
                    }
                }
            }
        }
        let mut aa = json!({});
        for i in 0..3 {
            let o = (!outer_aa[i].is_empty()).then(|| median(outer_aa[i].clone()));
            let n = (!inner_aa[i].is_empty()).then(|| median(inner_aa[i].clone()));
            if [o, n]
                .iter()
                .any(|v| v.is_none_or(|v| v < 1.0 / tolerance || v > tolerance))
            {
                reasons.insert(format!("{} inner/outer timing A/A rejected", METHODS[i]));
            }
            aa[METHODS[i]] = json!({"outer":o,"inner":n});
        }
        if !global.is_empty() {
            reasons.insert("global protocol integrity failure".into());
        }
        let eligible = reasons.is_empty() && complete == rounds && correct == rounds as usize;
        let mut ratios = json!({});
        for (i, name) in ["graph_over_ic", "graph_over_rho", "ic_over_rho"]
            .iter()
            .enumerate()
        {
            ratios[*name] = json!({"outer":estimate(&outer_logs[i],draws,seed,outer_logs[i].len()==rounds as usize),"inner":estimate(&inner_logs[i],draws,seed,inner_logs[i].len()==rounds as usize)});
        }
        results.push(json!({"p":p,"seed":case_seed,"l":fixture.map(|f|f["family"]["l"].clone()),"planned_rounds":rounds,"complete_rounds":complete,"correct_rounds":correct,"invalid_reasons":reasons,"eligible_local_controlled_comparison":eligible,"nontrivial_graph_projection_in_all_measured_arms":graph_used&&complete==rounds,"timing_aa":aa,"ratios":ratios,"first_measured_work":first_work}));
    }
    let all_valid = global.is_empty()
        && starts.len() == schedule.len()
        && executions.len() == schedule.len()
        && plan["observed_process_exit"] == 0
        && results.iter().all(|r| {
            r["eligible_local_controlled_comparison"] == true
                && r["nontrivial_graph_projection_in_all_measured_arms"] == true
        });
    let parity = plan["allow_local_reference_parity"] == true
        && all_valid
        && cases.len() >= 3
        && results.iter().all(|r| {
            r["l"].as_u64().is_some_and(|l| l >= 1 << 30)
                && ["outer", "inner"].iter().all(|k| {
                    r["ratios"]["graph_over_rho"][*k]["bootstrap_95"][1]
                        .as_f64()
                        .is_some_and(|u| u <= 1.0)
                })
        });
    Ok(
        json!({"schema":"cover.sieve.graph-summary/v1","source_revision":source,"backend":plan["backend"],"observed_process_exit":plan["observed_process_exit"],"planned_executions":schedule.len(),"attempted_executions":starts.len(),"completed_executions":executions.len(),"raw_executions":raw_count,"in_flight_without_completion":pending,"unattempted_executions":schedule.len().saturating_sub(starts.len()),"warmup_executions":completions.iter().filter(|k|k.2=="warmup").count(),"measured_executions":completions.iter().filter(|k|k.2=="measurement").count(),"global_problems":global,"shared_challenge_preparation_ns":prep,"allows_later_frozen_scale_trial":all_valid,"local_native_reference_parity":parity,"scope":"fixed public weak-Fp6 instances; complete cold child processes, planted answer withheld; legacy costs incomplete; no general-curve or admitted ranking inference","strata":results}),
    )
}
fn plot(s: &Value, path: &Path) -> Result<(), Box<dyn Error>> {
    let cases = s["strata"].as_array().ok_or("strata")?;
    let root = SVGBackend::new(path, (1200, 650)).into_drawing_area();
    root.fill(&WHITE)?;
    let names = ["graph_over_ic", "graph_over_rho", "ic_over_rho"];
    let labels: Vec<_> = cases
        .iter()
        .flat_map(|r| {
            names
                .iter()
                .map(move |n| format!("p{} s{} {}", r["p"], r["seed"], n.replace("_over_", "/")))
        })
        .collect();
    let values: Vec<_> = cases
        .iter()
        .flat_map(|r| {
            names.iter().flat_map(move |n| {
                [
                    r["ratios"][*n]["outer"]["geometric_mean"].as_f64(),
                    r["ratios"][*n]["outer"]["bootstrap_95"][0].as_f64(),
                    r["ratios"][*n]["outer"]["bootstrap_95"][1].as_f64(),
                ]
            })
        })
        .flatten()
        .collect();
    let lo = values.iter().copied().fold(1.0, f64::min).max(0.0001) * 0.8;
    let hi = values.iter().copied().fold(1.0, f64::max) * 1.2;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Weak-cover IC and checked graph filtering: cold-process wall",
            ("sans-serif", 22),
        )
        .margin(24)
        .x_label_area_size(80)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0..labels.len().max(1) as f64, (lo..hi).log_scale())?;
    chart
        .configure_mesh()
        .y_desc("wall ratio; lower costs less")
        .x_labels(labels.len())
        .x_label_formatter(&|x| labels.get(*x as usize).cloned().unwrap_or_default())
        .disable_x_mesh()
        .draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 1.0), (labels.len() as f64, 1.0)],
        BLACK,
    ))?;
    for (case, r) in cases.iter().enumerate() {
        for (pair, name) in names.iter().enumerate() {
            let x = (case * 3 + pair) as f64 + 0.5;
            let v = &r["ratios"][*name]["outer"];
            let color = if r["eligible_local_controlled_comparison"] == true {
                BLUE
            } else {
                RGBColor(150, 150, 150)
            };
            if let Some(mean) = v["geometric_mean"].as_f64() {
                chart.draw_series(std::iter::once(Rectangle::new(
                    [(x - 0.25, lo), (x + 0.25, mean)],
                    color.filled(),
                )))?;
                if let (Some(a), Some(b)) =
                    (v["bootstrap_95"][0].as_f64(), v["bootstrap_95"][1].as_f64())
                {
                    chart.draw_series(std::iter::once(PathElement::new(
                        vec![(x, a), (x, b)],
                        BLACK,
                    )))?;
                }
            }
        }
    }
    root.present()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: cover-graph-analysis RAW PLAN NEW-DIR".into());
    }
    let mut plan: Value = serde_json::from_reader(fs::File::open(&a[2])?)?;
    plan["observed_process_exit"] = fs::read_to_string(
        Path::new(&a[1])
            .parent()
            .ok_or("raw parent")?
            .join("exit-code.txt"),
    )
    .ok()
    .and_then(|s| s.trim().parse::<i32>().ok())
    .map_or(Value::Null, |x| json!(x));
    let mut rows = Vec::new();
    let mut failures = 0;
    for line in BufReader::new(fs::File::open(&a[1])?).lines() {
        match serde_json::from_str::<Value>(&line?) {
            Ok(v) => rows.push(v),
            Err(_) => failures += 1,
        }
    }
    let summary = analyze(&plan, rows, failures)?;
    let dir = Path::new(&a[3]);
    fs::create_dir(dir)?;
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(dir.join("summary.json"))?;
    serde_json::to_writer_pretty(&mut f, &summary)?;
    writeln!(f)?;
    plot(&summary, &dir.join("ratios.svg"))?;
    println!(
        "completed={}/{}, scale={}, local-native-reference-parity={}",
        summary["completed_executions"],
        summary["planned_executions"],
        summary["allows_later_frozen_scale_trial"],
        summary["local_native_reference_parity"]
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture(seed: u64, l: u64) -> (Value, Vec<Value>) {
        let source = "0".repeat(40);
        let dp = 16.min((l.ilog2() / 2).saturating_sub(4));
        let plan = json!({"source_revision":source,"primes":[127],"seeds":[seed],"rounds":5,"warmup_rounds":1,"bootstrap_draws":1000,"bootstrap_seed":20261115,"aa_tolerance":1.1,"driver_kind":"cover-sieve-graph","retained_percent":75,"max_small_weight":4096,"max_sources":4096,"observed_process_exit":0,"allow_local_reference_parity":false});
        let family = json!({"p":127,"l":l,"alpha":[[1,0],[2,0],[3,0]],"g":{"x":[[1,0],[0,0],[0,0]],"y":[[2,0],[0,0],[0,0]],"inf":false},"q":{"x":[[seed%127,0],[0,0],[0,0]],"y":[[3,0],[0,0],[0,0]],"inf":false}});
        let digest = blake3::hash(&serde_json::to_vec(&family).unwrap())
            .to_hex()
            .to_string();
        let base = json!({"schema":"cover.sieve.graph-process/v1","source_revision":source,"p":127,"seed":seed,"family":family,"input_digest":digest,"retained_percent":75,"rho_options":{"negation":true,"batch":16,"dp_bits":dp,"r":256}});
        let mut row = base.clone();
        row["record_type"] = json!("fixture");
        row["expected_scalar"] = json!(3);
        row["challenge_preparation_ns"] = json!(1000);
        let mut rows = vec![row];
        for (phase, n) in [("warmup", 1), ("measurement", 5)] {
            for round in 0..n {
                let mut arms = ARMS;
                if round % 2 == 1 {
                    arms.reverse()
                }
                for arm in arms {
                    let mut row = base.clone();
                    row["record_type"] = json!("execution_start");
                    row["phase"] = json!(phase);
                    row["round"] = json!(round);
                    row["arm"] = json!(arm);
                    rows.push(row.clone());
                    let method = arm.split('_').next().unwrap();
                    let graph = json!({"calls":1,"projected":1,"refusals":0,"fallback_solves":0,"replayed_rows":4,"input_rows":5,"output_rows":4,"filter_call_wall_ns":1,"input_digest":"f".repeat(64),"last_report":{},"counter_scope":"legacy counters omit graph work"});
                    let work = if method == "rho" {
                        json!({"run":{"correct":false,"capped":false,"negation":true,"batch":16,"dp_bits":dp,"r":256},"verified":true})
                    } else {
                        json!({"solved":true,"correct":false,"graph_filter":if method=="graph"{graph}else{Value::Null}})
                    };
                    let mut cleaned = work.clone();
                    clean(&mut cleaned);
                    let signature =
                        json!({"scalar":3,"verified":true,"capped":false,"work":cleaned});
                    row["record_type"] = json!("execution");
                    row["valid"] = json!(true);
                    row["child_exit"] = json!(0);
                    let t = match method {
                        "graph" => 50,
                        "ic" => 100,
                        _ => 200,
                    };
                    row["cold_process_wall_ns"] = json!(t + 10);
                    row["worker"] = json!({"method":method,"solver_answer_placeholder":0,"recovered_scalar":3,"verified":true,"capped":false,"inner_wall_ns":t,"work":work,"signature":signature});
                    rows.push(row);
                }
            }
        }
        (plan, rows)
    }
    fn rejected(s: &Value) {
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
        assert_eq!(s["allows_later_frozen_scale_trial"], false);
        assert_eq!(s["local_native_reference_parity"], false);
    }
    #[test]
    fn complete_public_fixture_keeps_flow_and_singleton_cannot_claim_parity() {
        let (p, r) = fixture(239, 1_000_003);
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["completed_executions"], 36);
        assert_eq!(s["warmup_executions"], 6);
        assert_eq!(s["measured_executions"], 30);
        assert_eq!(s["allows_later_frozen_scale_trial"], true);
        assert_eq!(s["local_native_reference_parity"], false);
    }
    #[test]
    fn explicit_sieve_recipe_and_process_limits_bind_every_record() {
        let (mut p, mut rows) = fixture(3, 1_000_003);
        p["sieve_recipe"] = json!("incidence_shared");
        p["worker_timeout_seconds"] = json!(900);
        for row in &mut rows {
            row["sieve_recipe"] = json!("incidence_shared");
            row["worker_timeout_seconds"] = json!(900);
            row["validation_only"] = json!(false);
        }
        assert_eq!(
            analyze(&p, rows.clone(), 0).unwrap()["allows_later_frozen_scale_trial"],
            true
        );
        for k in ["sieve_recipe", "worker_timeout_seconds", "validation_only"] {
            let mut damaged = rows.clone();
            damaged[1][k] = Value::Null;
            assert_eq!(
                analyze(&p, damaged, 0).unwrap()["allows_later_frozen_scale_trial"],
                false
            );
        }
    }
    #[test]
    fn false_valid_flag_or_placeholder_cannot_hide_wrong_scalar() {
        for kind in 0..4 {
            let (p, mut r) = fixture(239, 1_000_003);
            for row in r.iter_mut().filter(|r| r["record_type"] == "execution") {
                match kind {
                    0 => row["worker"]["recovered_scalar"] = json!(4),
                    1 => row["worker"]["solver_answer_placeholder"] = json!(3),
                    2 => row["worker"]["capped"] = json!(true),
                    _ => row["worker"]["verified"] = json!(false),
                }
            }
            rejected(&analyze(&p, r, 0).unwrap())
        }
    }
    #[test]
    fn public_digest_source_and_duplicate_case_are_enforced() {
        for kind in 0..4 {
            let (p, mut r) = fixture(239, 1_000_003);
            match kind {
                0 => r[0]["family"]["q"]["x"][0][0] = json!(0),
                1 => r[2]["source_revision"] = json!("1".repeat(40)),
                2 => r.push(r[2].clone()),
                _ => r.swap(1, 3),
            }
            rejected(&analyze(&p, r, 0).unwrap())
        }
    }
    #[test]
    fn native_answer_flags_graph_proofs_and_signatures_stay_checked() {
        for kind in 0..3 {
            let (p, mut r) = fixture(239, 1_000_003);
            for row in r
                .iter_mut()
                .filter(|r| r["record_type"] == "execution" && r["worker"]["method"] == "graph")
            {
                let w = &mut row["worker"];
                match kind {
                    0 => w["work"]["correct"] = json!(true),
                    1 => w["work"]["graph_filter"]["refusals"] = json!(1),
                    _ => w["signature"]["scalar"] = json!(4),
                }
            }
            rejected(&analyze(&p, r, 0).unwrap())
        }
    }
    #[test]
    fn inner_or_outer_aa_failure_is_retained() {
        for inner in [false, true] {
            let (p, mut r) = fixture(239, 1_000_003);
            for row in r
                .iter_mut()
                .filter(|r| r["record_type"] == "execution" && r["arm"] == "rho_b")
            {
                if inner {
                    row["worker"]["inner_wall_ns"] = json!(100)
                } else {
                    row["cold_process_wall_ns"] = json!(420)
                }
            }
            rejected(&analyze(&p, r, 0).unwrap())
        }
    }
    #[test]
    fn censor_keeps_inflight_and_never_makes_a_partial_interval() {
        let (p, mut r) = fixture(239, 1_000_003);
        r.truncate(26);
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["attempted_executions"], 13);
        assert_eq!(s["completed_executions"], 12);
        assert!(!s["in_flight_without_completion"].is_null());
        assert!(s["strata"][0]["ratios"]["graph_over_rho"]["outer"]["bootstrap_95"].is_null());
        rejected(&s)
    }
    #[test]
    fn parity_requires_three_distinct_public_challenges_and_zero_exit() {
        let (mut p, _) = fixture(239, (1u64 << 31) - 1);
        p["seeds"] = json!([239, 241, 251]);
        p["allow_local_reference_parity"] = json!(true);
        let mut rows = Vec::new();
        for seed in [239, 241, 251] {
            rows.extend(fixture(seed, (1u64 << 31) - 1).1)
        }
        assert_eq!(
            analyze(&p, rows.clone(), 0).unwrap()["local_native_reference_parity"],
            true
        );
        for status in [json!(124), Value::Null] {
            p["observed_process_exit"] = status;
            assert_eq!(
                analyze(&p, rows.clone(), 0).unwrap()["allows_later_frozen_scale_trial"],
                false
            )
        }
        p["observed_process_exit"] = json!(0);
        for row in rows.iter_mut().filter(|r| r["seed"] == 241) {
            row["family"] = fixture(239, (1u64 << 31) - 1).1[0]["family"].clone();
            row["input_digest"] = blake3::hash(&serde_json::to_vec(&row["family"]).unwrap())
                .to_hex()
                .to_string()
                .into()
        }
        assert!(!analyze(&p, rows, 0).unwrap()["global_problems"]
            .as_array()
            .unwrap()
            .is_empty());
    }
}
