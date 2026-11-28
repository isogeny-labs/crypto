//! Retained cold batches of distinct S4 residuals. No pipeline extrapolation.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

type Key = (u64, u64, u64, String, u64, String);
type Case = (u64, u64, u64);
const ARMS: [&str; 4] = ["fresh_a", "trace_a", "fresh_b", "trace_b"];

fn integer(v: &Value, field: &str) -> Result<u64, String> {
    v[field]
        .as_u64()
        .ok_or_else(|| format!("missing integer {field}"))
}
fn key(v: &Value) -> Result<Key, String> {
    Ok((
        integer(v, "p")?,
        integer(v, "seed")?,
        integer(v, "ordinal")?,
        v["phase"].as_str().ok_or("missing phase")?.into(),
        integer(v, "round")?,
        v["arm"].as_str().ok_or("missing arm")?.into(),
    ))
}
fn context(v: &Value) -> Value {
    json!([
        v["family"],
        v["input_digest"],
        v["scalars"],
        v["residual_xs"],
        v["batch_iterations"],
        certifier(v)
    ])
}
fn certifier(v: &Value) -> Option<&str> {
    match v.get("trace_certifier") {
        None => Some("exhaustive"),
        Some(v) => v
            .as_str()
            .filter(|s| matches!(*s, "exhaustive" | "pair-graph")),
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
fn estimate(values: &[f64], draws: usize, mut state: u64) -> Value {
    if values.is_empty() {
        return Value::Null;
    }
    let mut boot = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut sum = 0.0;
        for _ in values {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            sum += values[state as usize % values.len()];
        }
        boot.push((sum / values.len() as f64).exp());
    }
    boot.sort_by(f64::total_cmp);
    json!({"geometric_mean":(values.iter().sum::<f64>()/values.len() as f64).exp(),
        "bootstrap_95":[boot[draws*25/1000],boot[(draws*975/1000).min(draws-1)]],"rounds":values.len()})
}
fn complete_outputs(v: &Value, p: u64, batch: usize) -> bool {
    v.as_array().is_some_and(|outputs| {
        outputs.len() == batch
            && outputs.iter().all(|roots| {
                roots.as_array().is_some_and(|roots| {
                    let mut previous = None;
                    roots.iter().all(|triple| {
                        let Some(a) = triple.as_array().filter(|a| a.len() == 3) else {
                            return false;
                        };
                        let Some(a) = a.iter().map(Value::as_u64).collect::<Option<Vec<_>>>()
                        else {
                            return false;
                        };
                        if a.iter().any(|&c| c >= p)
                            || a[0] > a[1]
                            || a[1] > a[2]
                            || previous.as_ref().is_some_and(|old| old >= &a)
                        {
                            return false;
                        }
                        previous = Some(a);
                        true
                    })
                })
            })
    })
}
fn valid_trace(v: &Value, batch: u64, pair_graph: bool) -> bool {
    let s = &v["pool"];
    let graph = &s["pair_graph"];
    if pair_graph {
        let counters = [
            "product_edges",
            "chain_edges",
            "direct_edges",
            "witness_tests",
            "resource_refusals",
        ];
        if counters.iter().any(|k| integer(graph, k).is_err())
            || graph["product_edges"]
                .as_u64()
                .and_then(|a| a.checked_add(graph["chain_edges"].as_u64()?))
                .and_then(|a| a.checked_add(graph["direct_edges"].as_u64()?))
                != s["certification_pairs"].as_u64()
            || graph["direct_edges"].as_u64() > s["certification_reductions"].as_u64()
        {
            return false;
        }
    } else if !graph.is_null() {
        return false;
    }
    let vals = [
        "learned",
        "accepted",
        "replay_attempts",
        "rejected",
        "fallbacks",
        "certification_reductions",
        "certification_pairs",
        "certification_multiplications",
        "certification_term_steps",
        "retained_words",
        "storage_refusals",
    ];
    if integer(v, "family_rejections").ok() != Some(0)
        || vals.iter().any(|k| integer(s, k).is_err())
    {
        return false;
    }
    let learned = s["learned"].as_u64().unwrap();
    let accepted = s["accepted"].as_u64().unwrap();
    let attempted = s["replay_attempts"].as_u64().unwrap();
    let rejected = s["rejected"].as_u64().unwrap();
    learned >= 1
        && learned <= batch
        && accepted <= batch
        && learned + accepted == batch
        && attempted < batch
        && rejected <= attempted
        && accepted == attempted - rejected
        && s["fallbacks"].as_u64().unwrap() <= rejected
        && s["retained_words"].as_u64().unwrap() <= 1_000_000
}

fn analyze(plan: &Value, rows: Vec<Value>, parse_failures: usize) -> Result<Value, String> {
    let expected_certifier = certifier(plan).ok_or("invalid trace certifier")?;
    let pair_graph = expected_certifier == "pair-graph";
    let source = plan["source_revision"]
        .as_str()
        .filter(|s| s.len() == 40 && s.bytes().all(|c| c.is_ascii_hexdigit()))
        .ok_or("source SHA required")?;
    let primes = plan["primes"]
        .as_array()
        .filter(|v| !v.is_empty())
        .ok_or("primes required")?;
    let seeds = plan["seeds"]
        .as_array()
        .filter(|v| !v.is_empty())
        .ok_or("seeds required")?;
    let batch = integer(plan, "batch_iterations")?;
    let residuals = integer(plan, "residuals_per_instance")?;
    let rounds = integer(plan, "rounds")?;
    let warm = integer(plan, "warmup_rounds")?;
    let draws = integer(plan, "bootstrap_draws")? as usize;
    let bootstrap_seed = integer(plan, "bootstrap_seed")?;
    let tolerance = plan["aa_tolerance"]
        .as_f64()
        .filter(|x| x.is_finite() && *x >= 1.0)
        .ok_or("AA tolerance required")?;
    if !(1..=64).contains(&batch)
        || residuals == 0
        || residuals > 512
        || !residuals.is_multiple_of(batch)
        || rounds < 5
        || warm == 0
        || draws < 100
        || bootstrap_seed == 0
        || plan["f4_max_degree"] != 24
        || plan["f4_deadline_ms"] != 0
        || plan["trace_capacity"] != 4
        || plan["trace_word_cap"] != 1_000_000
    {
        return Err("invalid frozen trace-batch plan".into());
    }
    let mut schedule = Vec::new();
    let mut cases = Vec::new();
    for p in primes {
        let p = p
            .as_u64()
            .filter(|p| *p > 3 && *p < 1 << 20 && p % 3 == 1)
            .ok_or("invalid prime fixture")?;
        for seed in seeds {
            let seed = seed.as_u64().ok_or("invalid seed")?;
            for ordinal in 0..residuals / batch {
                cases.push((p, seed, ordinal));
                for (phase, n) in [("warmup", warm), ("measurement", rounds)] {
                    for round in 0..n {
                        let mut arms = ARMS;
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            schedule.push((p, seed, ordinal, phase.into(), round, arm.into()));
                        }
                    }
                }
            }
        }
    }
    let mut problems = Vec::new();
    if parse_failures > 0 {
        problems.push(format!("{parse_failures} malformed raw lines"));
    }
    let mut fixtures = BTreeMap::<Case, Value>::new();
    let mut executions = BTreeMap::<Key, Value>::new();
    let mut starts = Vec::new();
    let mut order = Vec::new();
    let mut pending = None;
    let mut phases = BTreeMap::<String, usize>::new();
    let mut raw_executions = 0;
    for row in rows {
        if certifier(&row) != Some(expected_certifier) {
            problems.push("trace certifier identity mismatch".into());
        }
        if row["schema"] != "gaudry.cubic.trace-batch/v1" || row["source_revision"] != source {
            problems.push("raw schema/source mismatch".into());
        }
        match row["record_type"].as_str() {
            Some("fixture") => {
                let c = (
                    integer(&row, "p")?,
                    integer(&row, "seed")?,
                    integer(&row, "ordinal")?,
                );
                if !cases.contains(&c) || fixtures.insert(c, row).is_some() {
                    problems.push("unexpected/duplicate fixture".into());
                }
            }
            Some("execution_start") => {
                let k = key(&row)?;
                if pending.is_some() {
                    problems.push("overlapping starts".into());
                }
                let c = (k.0, k.1, k.2);
                if fixtures.get(&c).is_none_or(|f| context(f) != context(&row)) {
                    problems.push("start input identity mismatch".into());
                }
                pending = Some(k.clone());
                starts.push(k);
            }
            Some("execution") => {
                let k = key(&row)?;
                raw_executions += 1;
                if pending.as_ref() != Some(&k) {
                    problems.push("completion without matching ordered start".into());
                }
                pending = None;
                let c = (k.0, k.1, k.2);
                if fixtures.get(&c).is_none_or(|f| context(f) != context(&row)) {
                    problems.push("completion input identity mismatch".into());
                }
                *phases.entry(k.3.clone()).or_default() += 1;
                order.push(k.clone());
                if executions.insert(k, row).is_some() {
                    problems.push("duplicate completion".into());
                }
            }
            _ => problems.push("unexpected record type".into()),
        }
    }
    if order.len() > schedule.len()
        || order != schedule[..order.len().min(schedule.len())]
        || starts.len() > schedule.len()
        || starts != schedule[..starts.len().min(schedule.len())]
        || starts.len() < order.len()
        || starts.len() > order.len() + 1
    {
        problems.push("frozen schedule/start order mismatch".into());
    }
    let mut families = BTreeMap::<(u64, u64), Value>::new();
    let mut seen_xs = BTreeMap::<(u64, u64), BTreeSet<String>>::new();
    let mut shared_prepare = BTreeMap::<(u64, u64), u64>::new();
    let mut qualifier_ns = 0u64;
    let mut fixture_ns = 0u64;
    for (&(p, seed, ordinal), f) in &fixtures {
        let wanted: Vec<_> = (ordinal * batch + 1..=(ordinal + 1) * batch).collect();
        let xs = f["residual_xs"].as_array();
        if f["scalars"] != json!(wanted)
            || f["batch_iterations"] != batch
            || f["family"][0]["p"] != p
            || f["family"][5].as_u64().is_none_or(|n| n <= 2 * residuals)
            || f["input_digest"]
                .as_str()
                .is_none_or(|s| s.len() != 64 || !s.bytes().all(|c| c.is_ascii_hexdigit()))
            || xs.is_none_or(|v| v.len() != batch as usize)
        {
            problems.push("invalid frozen fixture metadata".into());
        }
        if let Some(xs) = xs {
            for x in xs {
                if x.as_array().is_none_or(|v| {
                    v.len() != 3 || v.iter().any(|v| v.as_u64().is_none_or(|v| v >= p))
                }) || !seen_xs.entry((p, seed)).or_default().insert(x.to_string())
                {
                    problems.push("noncanonical/repeated residual abscissa".into());
                }
            }
        }
        if families
            .get(&(p, seed))
            .is_some_and(|old| *old != f["family"])
        {
            problems.push("original curve/target family changed".into());
        }
        families.insert((p, seed), f["family"].clone());
        let prepare = integer(f, "shared_prepare_ns")?;
        if shared_prepare
            .insert((p, seed), prepare)
            .is_some_and(|old| old != prepare)
        {
            problems.push("shared preparation identity mismatch".into());
        }
        qualifier_ns = qualifier_ns
            .checked_add(integer(f, "qualifier_ns")?)
            .ok_or("qualification time overflow")?;
        fixture_ns = fixture_ns
            .checked_add(integer(f, "fixture_ns")?)
            .ok_or("fixture time overflow")?;
    }
    let mut strata = Vec::new();
    for (p, seed, ordinal) in cases {
        let mut reasons = Vec::new();
        let f = fixtures.get(&(p, seed, ordinal));
        if f.is_none_or(|f| {
            f["qualifier_complete"] != true
                || !complete_outputs(&f["qualifier_outputs"], p, batch as usize)
        }) {
            reasons.push("missing/unsupported dense qualifier".to_string());
        }
        let mut signatures = [None, None];
        let mut logs = Vec::new();
        let mut fresh_aa = Vec::new();
        let mut trace_aa = Vec::new();
        let mut complete = 0;
        let mut correct = 0;
        let mut accepted_all = true;
        let mut first_trace = Value::Null;
        for (phase, n) in [("warmup", warm), ("measurement", rounds)] {
            for round in 0..n {
                let mut times = [0.0; 4];
                let mut all = true;
                let mut valid = true;
                for (i, arm) in ARMS.iter().enumerate() {
                    let k = (p, seed, ordinal, phase.into(), round, (*arm).into());
                    let Some(row) = executions.get(&k) else {
                        all = false;
                        valid = false;
                        continue;
                    };
                    let signature = &row["signature"];
                    let trace = arm.starts_with("trace_");
                    valid &= row["valid"] == true
                        && complete_outputs(&signature["outputs"], p, batch as usize)
                        && f.is_some_and(|f| signature["outputs"] == f["qualifier_outputs"])
                        && signature["fp_muls"].as_u64().is_some_and(|v| v > 0)
                        && signature["quotient_dimension"].as_u64().is_some()
                        && if trace {
                            valid_trace(&signature["trace"], batch, pair_graph)
                        } else {
                            signature["trace"].is_null()
                        };
                    let slot = usize::from(trace);
                    if signatures[slot]
                        .as_ref()
                        .is_some_and(|old| old != signature)
                    {
                        reasons.push("same-method semantic/answer A/A mismatch".into());
                    }
                    signatures[slot].get_or_insert_with(|| signature.clone());
                    times[i] = row["phase_wall_ns"]
                        .as_f64()
                        .filter(|x| x.is_finite() && *x > 0.0)
                        .unwrap_or(0.0);
                    if times[i] == 0.0 {
                        valid = false;
                    }
                    if phase == "measurement" && trace {
                        accepted_all &= signature["trace"]["pool"]["accepted"]
                            .as_u64()
                            .is_some_and(|n| n > 0);
                        if pair_graph {
                            let graph = &signature["trace"]["pool"]["pair_graph"];
                            accepted_all &= graph["product_edges"]
                                .as_u64()
                                .unwrap_or(0)
                                .saturating_add(graph["chain_edges"].as_u64().unwrap_or(0))
                                > 0;
                        }
                        if first_trace.is_null() {
                            first_trace = signature["trace"].clone();
                        }
                    }
                }
                if !all {
                    reasons.push(format!("incomplete {phase} round {round}"));
                }
                if !valid {
                    reasons.push(format!("invalid {phase} round {round}"));
                }
                if phase == "measurement" && all {
                    complete += 1;
                    correct += usize::from(valid);
                    if times.iter().all(|x| *x > 0.0) {
                        logs.push(
                            0.5 * (times[1].ln() + times[3].ln() - times[0].ln() - times[2].ln()),
                        );
                        fresh_aa.push(times[0] / times[2]);
                        trace_aa.push(times[1] / times[3]);
                    }
                }
            }
        }
        let fa = (!fresh_aa.is_empty()).then(|| median(fresh_aa));
        let ta = (!trace_aa.is_empty()).then(|| median(trace_aa));
        if [fa, ta]
            .iter()
            .any(|x| x.is_none_or(|v| v < 1.0 / tolerance || v > tolerance))
        {
            reasons.push("timing A/A rejected".into());
        }
        if !problems.is_empty() {
            reasons.push("global protocol integrity failure".into());
        }
        reasons.sort();
        reasons.dedup();
        let eligible = reasons.is_empty() && complete == rounds as usize && correct == complete;
        let mut cost = estimate(&logs, draws, bootstrap_seed);
        if !cost.is_null() && logs.len() != rounds as usize {
            cost["bootstrap_95"] = Value::Null;
            cost["interval_status"] = json!("incomplete_no_usable_interval");
        }
        strata.push(
            json!({"p":p,"seed":seed,"ordinal":ordinal,"planned_rounds":rounds,
            "complete_rounds":complete,"correct_rounds":correct,"invalid_reasons":reasons,
            "eligible_local_controlled_comparison":eligible,"candidate_over_reference":cost,
            "median_fresh_aa_ratio":fa,"median_trace_aa_ratio":ta,
            "accepted_replay_in_all_measured_batches":accepted_all && complete==rounds as usize,
            "first_measured_trace_stats":first_trace}),
        );
    }
    let largest = primes.iter().filter_map(Value::as_u64).max().unwrap();
    let largest_gains = strata
        .iter()
        .filter(|s| {
            s["p"] == largest
                && s["eligible_local_controlled_comparison"] == true
                && s["accepted_replay_in_all_measured_batches"] == true
                && s["candidate_over_reference"]["bootstrap_95"][1]
                    .as_f64()
                    .is_some_and(|v| v < 1.0)
        })
        .count();
    let all_outputs_correct = strata.iter().all(|s| {
        s["complete_rounds"] == rounds
            && s["correct_rounds"] == rounds
            && !s["invalid_reasons"].as_array().unwrap().iter().any(|r| {
                r.as_str().unwrap().starts_with("invalid ")
                    || r.as_str().unwrap().contains("qualifier")
            })
    });
    let mut summary = json!({"schema":"gaudry.cubic.trace-batch-summary/v1","source_revision":source,"backend":plan["backend"],
        "observed_process_exit":plan["observed_process_exit"],
        "planned_batch_executions":schedule.len(),"attempted_batch_executions":starts.len(),
        "completed_batch_executions":executions.len(),"raw_batch_executions":raw_executions,
        "in_flight_without_completion":pending,"unattempted_batch_executions":schedule.len().saturating_sub(starts.len()),
        "phase_counts":phases,"fixture_count":fixtures.len(),"planned_coordinate_calls":schedule.len() as u64*batch,
        "global_problems":problems,"shared_cost_ns":{"curve_and_symbolic_once_per_instance":shared_prepare.values().map(|&x| x as u128).sum::<u128>(),
            "residual_fixture_construction":fixture_ns,"dense_qualification":qualifier_ns},
        "largest_field_controlled_gains_with_replay":largest_gains,
        "allows_later_frozen_full_ic":plan["observed_process_exit"]==0 && all_outputs_correct && starts.len()==schedule.len() && executions.len()==schedule.len() && problems.is_empty() && largest_gains>=6,
        "scope":"complete cold eight-distinct-residual coordinate batches; shared costs outside compared phase; no IC/rho extrapolation",
        "interval_scope":"paired timing rounds on fixed curves/batches, not independent curve inference",
        "standard_lab_admission":false,"strata":strata});
    if pair_graph {
        summary["trace_certifier"] = json!(expected_certifier);
    }
    Ok(summary)
}

fn plot(summary: &Value, path: &Path) -> Result<(), Box<dyn Error>> {
    let strata = summary["strata"].as_array().ok_or("missing strata")?;
    let width = (strata.len() as u32 * 105 + 120).max(1100);
    let root = SVGBackend::new(path, (width, 620)).into_drawing_area();
    root.fill(&WHITE)?;
    let bounds: Vec<_> = strata
        .iter()
        .flat_map(|s| {
            [
                s["candidate_over_reference"]["bootstrap_95"][0].as_f64(),
                s["candidate_over_reference"]["bootstrap_95"][1].as_f64(),
                s["candidate_over_reference"]["geometric_mean"].as_f64(),
            ]
        })
        .flatten()
        .collect();
    let lo = bounds.iter().copied().fold(1.0, f64::min).max(0.001) * 0.8;
    let hi = bounds.iter().copied().fold(1.0, f64::max) * 1.2;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Cold certified trace / fresh F4 coordinate batches",
            ("sans-serif", 24),
        )
        .margin(24)
        .x_label_area_size(65)
        .y_label_area_size(65)
        .build_cartesian_2d(0f64..strata.len().max(1) as f64, (lo..hi).log_scale())?;
    chart
        .configure_mesh()
        .y_desc("wall ratio; lower costs less")
        .x_labels(strata.len())
        .x_label_formatter(&|x| {
            strata
                .get(*x as usize)
                .map(|s| format!("p{} s{} b{}", s["p"], s["seed"], s["ordinal"]))
                .unwrap_or_default()
        })
        .disable_x_mesh()
        .draw()?;
    chart.draw_series(LineSeries::new(
        vec![(0.0, 1.0), (strata.len() as f64, 1.0)],
        BLACK,
    ))?;
    for (i, s) in strata.iter().enumerate() {
        if let Some(mean) = s["candidate_over_reference"]["geometric_mean"].as_f64() {
            let color = if s["eligible_local_controlled_comparison"] == true {
                BLUE
            } else {
                RGBColor(160, 160, 160)
            };
            let x = i as f64 + 0.5;
            chart.draw_series(std::iter::once(Rectangle::new(
                [(x - 0.28, lo), (x + 0.28, mean)],
                color.filled(),
            )))?;
            if let (Some(a), Some(b)) = (
                s["candidate_over_reference"]["bootstrap_95"][0].as_f64(),
                s["candidate_over_reference"]["bootstrap_95"][1].as_f64(),
            ) {
                chart.draw_series([
                    PathElement::new(vec![(x, a), (x, b)], BLACK),
                    PathElement::new(vec![(x - 0.06, a), (x + 0.06, a)], BLACK),
                    PathElement::new(vec![(x - 0.06, b), (x + 0.06, b)], BLACK),
                ])?;
            }
        }
    }
    root.draw(&Text::new("Blue: controlled; gray: rejected/incomplete. Intervals cover fixed-case repeated timing only.",(25,605),("sans-serif",14)))?;
    root.present()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 4 {
        return Err("usage: s4-trace-analysis RAW PLAN NEW-OUTPUT-DIR".into());
    }
    let mut plan: Value = serde_json::from_slice(&fs::read(&args[2])?)?;
    let status_path = Path::new(&args[1])
        .parent()
        .ok_or("raw parent required")?
        .join("exit-code.txt");
    if status_path.exists() {
        plan["observed_process_exit"] =
            json!(fs::read_to_string(status_path)?.trim().parse::<u64>()?);
    }
    let mut rows = Vec::new();
    let mut malformed = 0;
    for line in BufReader::new(fs::File::open(&args[1])?).lines() {
        match serde_json::from_str(&line?) {
            Ok(v) => rows.push(v),
            Err(_) => malformed += 1,
        }
    }
    let summary = analyze(&plan, rows, malformed)?;
    let dir = Path::new(&args[3]);
    fs::create_dir(dir)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join("summary.json"))?;
    serde_json::to_writer_pretty(&mut out, &summary)?;
    writeln!(out)?;
    plot(&summary, &dir.join("ratios.svg"))?;
    let primes: BTreeSet<_> = summary["strata"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["p"].as_u64())
        .collect();
    for p in primes {
        let mut subset = summary.clone();
        subset["strata"] = json!(summary["strata"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["p"] == p)
            .collect::<Vec<_>>());
        plot(&subset, &dir.join(format!("ratios-p{p}.svg")))?;
    }
    println!(
        "batches={}/{}, largest controlled gains with replay={}, full-IC followup={}",
        summary["completed_batch_executions"],
        summary["planned_batch_executions"],
        summary["largest_field_controlled_gains_with_replay"],
        summary["allows_later_frozen_full_ic"]
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> (Value, Vec<Value>) {
        let source = "0".repeat(40);
        let plan = json!({"source_revision":source,"backend":"local_user_authorized",
            "primes":[67],"seeds":[173],"batch_iterations":8,"residuals_per_instance":8,
            "rounds":5,"warmup_rounds":1,"bootstrap_draws":1000,"bootstrap_seed":20261115,
            "aa_tolerance":1.1,"f4_max_degree":24,"f4_deadline_ms":0,
            "trace_capacity":4,"trace_word_cap":1000000});
        let outputs = json!(vec![Vec::<[u64; 3]>::new(); 8]);
        let base = json!({"schema":"gaudry.cubic.trace-batch/v1","record_type":"fixture",
            "source_revision":source,"p":67,"seed":173,"ordinal":0,
            "family":[{"p":67,"c":2},[1,2,3],[3,2,1],{}, {},100003],
            "scalars":(1..=8).collect::<Vec<_>>(),"residual_xs":(1..=8).map(|d| [d,0,0]).collect::<Vec<_>>(),
            "batch_iterations":8,"input_digest":"f".repeat(64),
            "shared_prepare_ns":1000,"fixture_ns":200,"qualifier_ns":3000,
            "qualifier_outputs":outputs,"qualifier_complete":true});
        let trace = json!({"family_rejections":0,"pool":{"learned":1,"accepted":7,
            "replay_attempts":7,"rejected":0,"fallbacks":0,"certification_reductions":28,
            "certification_pairs":7,"certification_multiplications":100,"certification_term_steps":42,
            "retained_words":1000,"storage_refusals":0}});
        let mut rows = vec![base.clone()];
        for (phase, n) in [("warmup", 1), ("measurement", 5)] {
            for round in 0..n {
                let mut arms = ARMS;
                if round % 2 == 1 {
                    arms.reverse();
                }
                for arm in arms {
                    let mut row = base.clone();
                    row["record_type"] = json!("execution_start");
                    row["phase"] = json!(phase);
                    row["round"] = json!(round);
                    row["arm"] = json!(arm);
                    rows.push(row.clone());
                    row["record_type"] = json!("execution");
                    row["valid"] = json!(true);
                    let candidate = arm.starts_with("trace_");
                    row["phase_wall_ns"] = json!(if candidate { 50 } else { 100 });
                    row["signature"] = json!({"outputs":outputs,"fp_muls":if candidate {600}else{1000},
                        "quotient_dimension":480,"trace":if candidate {trace.clone()}else{Value::Null}});
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
        assert_eq!(s["allows_later_frozen_full_ic"], false);
    }
    #[test]
    fn complete_cold_batch_retains_flow_without_pipeline_extrapolation() {
        let (p, r) = fixture();
        let s = analyze(&p, r, 0).unwrap();
        assert!(s["global_problems"].as_array().unwrap().is_empty());
        assert_eq!(s["completed_batch_executions"], 24);
        assert_eq!(s["phase_counts"]["warmup"], 4);
        assert_eq!(s["phase_counts"]["measurement"], 20);
        assert_eq!(s["planned_coordinate_calls"], 192);
        assert_eq!(s["strata"][0]["eligible_local_controlled_comparison"], true);
        assert!(
            (s["strata"][0]["candidate_over_reference"]["geometric_mean"]
                .as_f64()
                .unwrap()
                - 0.5)
                .abs()
                < 1e-12
        );
        assert_eq!(s["allows_later_frozen_full_ic"], false); // fewer than six largest cases
    }

    #[test]
    fn pair_graph_requires_declared_identity_and_all_edge_counters() {
        let (mut p, mut rows) = fixture();
        p["trace_certifier"] = json!("pair-graph");
        for row in &mut rows {
            row["trace_certifier"] = json!("pair-graph");
            if row["record_type"] == "execution"
                && row["arm"].as_str().unwrap().starts_with("trace_")
            {
                row["signature"]["trace"]["pool"]["pair_graph"] = json!({"product_edges":2,"chain_edges":3,
                    "direct_edges":2,"witness_tests":20,"resource_refusals":0});
            }
        }
        let s = analyze(&p, rows.clone(), 0).unwrap();
        assert_eq!(s["strata"][0]["eligible_local_controlled_comparison"], true);
        assert_eq!(s["trace_certifier"], "pair-graph");
        for kind in 0..4 {
            let mut changed = rows.clone();
            for row in &mut changed {
                if kind == 0 {
                    row["trace_certifier"] = json!("exhaustive");
                }
                if row["record_type"] == "execution"
                    && row["arm"].as_str().unwrap().starts_with("trace_")
                {
                    let graph = &mut row["signature"]["trace"]["pool"]["pair_graph"];
                    match kind {
                        1 => *graph = Value::Null,
                        2 => graph["direct_edges"] = json!(1),
                        3 => graph["witness_tests"] = json!(-1),
                        _ => {}
                    }
                }
            }
            rejected(&analyze(&p, changed, 0).unwrap());
        }
        let (old, _) = fixture();
        rejected(&analyze(&old, rows, 0).unwrap());
    }

    #[test]
    fn six_largest_gains_require_an_observed_zero_process_exit() {
        let (mut p, base) = fixture();
        let seeds = [173, 179, 181, 191, 193, 197];
        p["seeds"] = json!(seeds);
        let mut rows = Vec::new();
        for seed in seeds {
            let mut part = base.clone();
            for row in &mut part {
                row["seed"] = json!(seed);
            }
            rows.extend(part);
        }
        p["observed_process_exit"] = json!(0);
        assert_eq!(
            analyze(&p, rows.clone(), 0).unwrap()["allows_later_frozen_full_ic"],
            true
        );
        for status in [json!(124), Value::Null] {
            p["observed_process_exit"] = status;
            assert_eq!(
                analyze(&p, rows.clone(), 0).unwrap()["allows_later_frozen_full_ic"],
                false
            );
        }
    }
    #[test]
    fn wrong_answer_cannot_hide_behind_a_valid_flag() {
        let (p, mut r) = fixture();
        r[2]["signature"]["outputs"][0] = json!([[1, 2, 3]]);
        rejected(&analyze(&p, r, 0).unwrap());
    }
    #[test]
    fn unsupported_coordinates_are_never_empty_roots() {
        let (p, mut r) = fixture();
        r[0]["qualifier_complete"] = json!(false);
        r[0]["qualifier_outputs"][0] = Value::Null;
        for row in r.iter_mut().filter(|r| r["record_type"] == "execution") {
            row["signature"]["outputs"][0] = Value::Null;
        }
        rejected(&analyze(&p, r, 0).unwrap());
    }
    #[test]
    fn warm_pool_cannot_omit_the_first_learning_cost() {
        let (p, mut r) = fixture();
        for row in r.iter_mut().filter(|r| {
            r["record_type"] == "execution" && r["arm"].as_str().unwrap().starts_with("trace_")
        }) {
            row["signature"]["trace"]["pool"]["learned"] = json!(0);
            row["signature"]["trace"]["pool"]["accepted"] = json!(8);
            row["signature"]["trace"]["pool"]["replay_attempts"] = json!(8);
        }
        rejected(&analyze(&p, r, 0).unwrap());
    }
    #[test]
    fn semantic_and_timing_aa_failures_stay_rejected() {
        for timing in [false, true] {
            let (p, mut r) = fixture();
            for row in r
                .iter_mut()
                .filter(|r| r["record_type"] == "execution" && r["arm"] == "trace_b")
            {
                if timing {
                    row["phase_wall_ns"] = json!(100);
                } else {
                    row["signature"]["fp_muls"] = json!(601);
                }
            }
            rejected(&analyze(&p, r, 0).unwrap());
        }
    }
    #[test]
    fn censor_keeps_unmatched_start_and_suppresses_incomplete_interval() {
        let (p, mut r) = fixture();
        r.truncate(18);
        let s = analyze(&p, r, 0).unwrap();
        assert!(s["global_problems"].as_array().unwrap().is_empty());
        assert_eq!(s["attempted_batch_executions"], 9);
        assert_eq!(s["completed_batch_executions"], 8);
        assert_eq!(s["unattempted_batch_executions"], 15);
        assert!(!s["in_flight_without_completion"].is_null());
        assert_eq!(s["strata"][0]["complete_rounds"], 1);
        assert!(s["strata"][0]["candidate_over_reference"]["bootstrap_95"].is_null());
        rejected(&s);
    }
    #[test]
    fn source_duplicate_and_order_tampering_fail_global_integrity() {
        for kind in 0..4 {
            let (p, mut r) = fixture();
            match kind {
                0 => r[2]["source_revision"] = json!("1".repeat(40)),
                1 => r.push(r[2].clone()),
                2 => r.swap(1, 3),
                _ => {}
            }
            let s = analyze(&p, r, usize::from(kind == 3)).unwrap();
            assert!(!s["global_problems"].as_array().unwrap().is_empty());
            rejected(&s);
        }
    }
    #[test]
    fn ordered_inputs_and_distinctness_are_checked_independently() {
        for repeated in [false, true] {
            let (p, mut r) = fixture();
            if repeated {
                for row in &mut r {
                    row["residual_xs"][1] = row["residual_xs"][0].clone();
                }
            } else {
                r[1]["family"][4] = json!({"changed_target":true});
            }
            let s = analyze(&p, r, 0).unwrap();
            assert!(!s["global_problems"].as_array().unwrap().is_empty());
            rejected(&s);
        }
    }
}
