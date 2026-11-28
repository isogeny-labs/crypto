//! Analysis of the explicitly incomplete S4 F4 basis phase; never an IC win.
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Fixture = (u64, u64, u64);
type Key = (u64, u64, u64, String, u64, String);
const ARMS: [&str; 4] = ["macaulay_a", "f4_a", "macaulay_b", "f4_b"];

fn median(v: &[f64]) -> f64 {
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) / 2.
    }
}
fn estimate(v: &[f64], mut state: u64) -> Value {
    if v.is_empty() {
        return Value::Null;
    }
    let mean = (v.iter().sum::<f64>() / v.len() as f64).exp();
    let mut draws = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let mut sum = 0.;
        for _ in v {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            sum += v[state as usize % v.len()];
        }
        draws.push((sum / v.len() as f64).exp());
    }
    draws.sort_by(f64::total_cmp);
    json!({"geometric_mean":mean,"bootstrap_95":[draws[250],draws[9749]],"rounds":v.len()})
}
fn positive(row: &Value, name: &str) -> Result<f64, Box<dyn Error>> {
    row[name]
        .as_f64()
        .filter(|v| v.is_finite() && *v > 0.)
        .ok_or_else(|| format!("invalid {name}").into())
}
fn analyze(plan: &Value, rows: Vec<Value>, parse_failures: usize) -> Result<Value, Box<dyn Error>> {
    let delayed = plan["driver_kind"] == "s4-delayed-component";
    let scalar_reference = delayed && plan["reference_kind"] == "scalar";
    if delayed && !matches!(plan["reference_kind"].as_str(), Some("scalar" | "dense")) {
        return Err("delayed comparison requires explicit scalar/dense reference".into());
    }
    let extraction = delayed || plan["driver_kind"] == "s4-krylov-component";
    let metric = if scalar_reference {
        "complete_delayed_over_scalar"
    } else if delayed {
        "complete_delayed_over_dense"
    } else {
        "complete_krylov_over_dense"
    };
    let arms = if scalar_reference {
        ["scalar_a", "delayed_a", "scalar_b", "delayed_b"]
    } else if delayed {
        ["dense_a", "delayed_a", "dense_b", "delayed_b"]
    } else if extraction {
        ["dense_a", "krylov_a", "dense_b", "krylov_b"]
    } else {
        ARMS
    };
    let schema = if delayed {
        "gaudry.cubic.s4-delayed/v1"
    } else if extraction {
        "gaudry.cubic.s4-extraction/v1"
    } else {
        "gaudry.cubic.s4-component/v1"
    };
    let source = plan["source_revision"]
        .as_str()
        .filter(|s| s.len() == 40)
        .ok_or("missing source SHA")?;
    let primes = plan["primes"].as_array().ok_or("missing primes")?;
    let seeds = plan["seeds"].as_array().ok_or("missing seeds")?;
    let residuals = plan["residuals_per_instance"]
        .as_u64()
        .ok_or("missing residual count")?;
    let rounds = plan["rounds"]
        .as_u64()
        .filter(|n| *n >= 5)
        .ok_or("five rounds required")?;
    let warmups = plan["warmup_rounds"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("warmup required")?;
    let tolerance = plan["aa_tolerance"]
        .as_f64()
        .filter(|n| *n >= 1.)
        .ok_or("AA tolerance required")?;
    let mut problems = Vec::new();
    if parse_failures > 0 {
        problems.push(format!("{parse_failures} malformed raw lines"));
    }
    let mut fixtures = BTreeMap::<Fixture, Value>::new();
    let mut executions = BTreeMap::<Key, Value>::new();
    let mut actual = Vec::new();
    let mut starts = Vec::new();
    let mut in_flight = None;
    for row in rows {
        if row["source_revision"] != source || row["schema"] != schema {
            problems.push("source/schema mismatch".to_owned());
        }
        let base = (
            row["p"].as_u64().ok_or("missing row p")?,
            row["seed"].as_u64().ok_or("missing seed")?,
            row["ordinal"].as_u64().ok_or("missing ordinal")?,
        );
        if !primes.contains(&json!(base.0))
            || !seeds.contains(&json!(base.1))
            || base.2 >= residuals
        {
            problems.push(format!("unexpected fixture {base:?}"));
        }
        match row["record_type"].as_str() {
            Some("fixture") => {
                if fixtures.insert(base, row).is_some() {
                    problems.push(format!("duplicate fixture {base:?}"));
                }
            }
            Some("execution") => {
                let key = (
                    base.0,
                    base.1,
                    base.2,
                    row["phase"].as_str().ok_or("phase missing")?.to_owned(),
                    row["round"].as_u64().ok_or("round missing")?,
                    row["arm"].as_str().ok_or("arm missing")?.to_owned(),
                );
                if extraction && in_flight.take().as_ref() != Some(&key) {
                    problems
                        .push("completion lacks matching immediately preceding start".to_owned());
                }
                actual.push(key.clone());
                if row["batch_iterations"] != plan["batch_iterations"]
                    || row["f4_max_degree"] != plan["f4_max_degree"]
                    || row["f4_deadline_ms"] != plan["f4_deadline_ms"]
                {
                    problems.push("frozen options mismatch".to_owned());
                }
                if executions.insert(key.clone(), row).is_some() {
                    problems.push(format!("duplicate execution {key:?}"));
                }
            }
            Some("execution_start") if extraction => {
                let key = (
                    base.0,
                    base.1,
                    base.2,
                    row["phase"]
                        .as_str()
                        .ok_or("start phase missing")?
                        .to_owned(),
                    row["round"].as_u64().ok_or("start round missing")?,
                    row["arm"].as_str().ok_or("start arm missing")?.to_owned(),
                );
                if in_flight.replace(key.clone()).is_some() {
                    problems.push("new start precedes completion of previous batch".to_owned());
                }
                starts.push(key);
            }
            _ => problems.push("unexpected raw record type".to_owned()),
        }
    }
    let mut expected = Vec::new();
    for p in primes {
        for seed in seeds {
            for ordinal in 0..residuals {
                for (phase, count) in [("warmup", warmups), ("measurement", rounds)] {
                    for round in 0..count {
                        let mut arms = arms;
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            expected.push((
                                p.as_u64().unwrap(),
                                seed.as_u64().unwrap(),
                                ordinal,
                                phase.to_owned(),
                                round,
                                arm.to_owned(),
                            ));
                        }
                    }
                }
            }
        }
    }
    if actual.len() > expected.len() || actual != expected[..actual.len().min(expected.len())] {
        problems.push("frozen execution order differs".to_owned());
    }
    if extraction
        && (starts.len() > expected.len()
            || starts != expected[..starts.len().min(expected.len())]
            || starts.len() < actual.len()
            || starts.len() > actual.len() + 1)
    {
        problems.push("execution start/completion order differs".to_owned());
    }
    let mut cases = Vec::new();
    for p in primes {
        for seed in seeds {
            for ordinal in 0..residuals {
                let base = (p.as_u64().unwrap(), seed.as_u64().unwrap(), ordinal);
                let fixture = fixtures.get(&base);
                let mut reasons = problems.clone();
                if fixture.is_none() {
                    reasons.push("fixture missing".to_owned());
                }
                let mut ratios = Vec::new();
                let mut mc_aa = Vec::new();
                let mut f4_aa = Vec::new();
                let mut completed = 0;
                for (phase, count) in [("warmup", warmups), ("measurement", rounds)] {
                    for round in 0..count {
                        let block: Vec<_> = arms
                            .iter()
                            .filter_map(|arm| {
                                executions.get(&(
                                    base.0,
                                    base.1,
                                    base.2,
                                    phase.to_owned(),
                                    round,
                                    arm.to_string(),
                                ))
                            })
                            .collect();
                        if block.len() != 4 {
                            reasons.push(format!("{phase} round {round} incomplete"));
                            continue;
                        }
                        let valid = block.iter().all(|r| {
                            r["valid"] == true
                                && r["intra_batch_consistent"] == true
                                && fixture.is_some_and(|f| {
                                    !f["input_digest"].is_null()
                                        && f["input_digest"] == r["input_digest"]
                                        && (!extraction
                                            || (!f["qualified_triples"].is_null()
                                                && r["signature"][0] == f["qualified_triples"]
                                                && r["output_kind"]
                                                    == "complete_S4_coordinate_solve"))
                                })
                        }) && block[0]["signature"] == block[2]["signature"]
                            && block[1]["signature"] == block[3]["signature"]
                            && (!scalar_reference
                                || (block.iter().all(|r| {
                                    r["signature"][1].as_u64().is_some()
                                        && r["signature"][2].as_u64().is_some()
                                }) && block[0]["signature"][1] == block[1]["signature"][1]
                                    && block[0]["signature"][2] == block[1]["signature"][2]));
                        if !valid {
                            reasons.push(format!(
                                "{phase} round {round} native/output/control failure"
                            ));
                        }
                        if phase == "warmup" {
                            continue;
                        }
                        completed += 1;
                        let t: Vec<_> = block
                            .iter()
                            .map(|r| positive(r, "phase_wall_ns"))
                            .collect::<Result<_, _>>()?;
                        ratios.push((t[1].ln() + t[3].ln() - t[0].ln() - t[2].ln()) / 2.);
                        mc_aa.push((t[0] / t[2]).ln());
                        f4_aa.push((t[1] / t[3]).ln());
                    }
                }
                let aa = |v: &[f64]| {
                    if v.len() == rounds as usize {
                        Some(median(v).exp())
                    } else {
                        None
                    }
                };
                let mc_control = aa(&mc_aa);
                let f4_control = aa(&f4_aa);
                if ![mc_control, f4_control]
                    .iter()
                    .all(|r| r.is_some_and(|r| (1. / tolerance..=tolerance).contains(&r)))
                {
                    reasons.push("median local AA tolerance failed or unavailable".to_owned());
                }
                let estimate = estimate(&ratios, 20261115);
                let eligible = reasons.is_empty() && completed == rounds;
                let mut case = json!({"p":p,"seed":seed,"ordinal":ordinal,"complete_rounds":completed,
            "basis_phase_over_complete_reference":estimate,"eligible_local_controlled_comparison":eligible,
            "invalid_reasons":reasons,"median_macaulay_aa":mc_control,"median_f4_aa":f4_control,
            "basis_phase_upper_below_half":eligible && estimate["bootstrap_95"][1].as_f64().is_some_and(|v|v<0.5),
            "pipeline_or_rho_improvement_claim":false});
                if extraction {
                    let o = case.as_object_mut().unwrap();
                    o.remove("basis_phase_over_complete_reference");
                    o.remove("basis_phase_upper_below_half");
                    o.remove("median_macaulay_aa");
                    o.remove("median_f4_aa");
                    o.insert(metric.into(), estimate.clone());
                    o.insert(
                        if delayed {
                            "median_reference_aa"
                        } else {
                            "median_dense_aa"
                        }
                        .into(),
                        json!(mc_control),
                    );
                    o.insert(
                        if delayed {
                            "median_delayed_aa"
                        } else {
                            "median_krylov_aa"
                        }
                        .into(),
                        json!(f4_control),
                    );
                    o.insert(
                        "complete_phase_upper_below_one".into(),
                        json!(
                            eligible
                                && estimate["bootstrap_95"][1].as_f64().is_some_and(|v| v < 1.)
                        ),
                    );
                    for (name, arm) in if delayed {
                        [
                            ("first_measured_reference", arms[0]),
                            ("first_measured_delayed", arms[1]),
                        ]
                    } else {
                        [
                            ("first_measured_dense", "dense_a"),
                            ("first_measured_krylov", "krylov_a"),
                        ]
                    } {
                        let first = executions.get(&(
                            base.0,
                            base.1,
                            base.2,
                            "measurement".to_owned(),
                            0,
                            arm.to_owned(),
                        ));
                        o.insert(
                            name.into(),
                            first.map(|r| r["signature"].clone()).unwrap_or(Value::Null),
                        );
                    }
                }
                cases.push(case);
            }
        }
    }
    let invalid_batches = executions.values().filter(|r| r["valid"] != true).count();
    let mut result = json!({"schema":"gaudry.cubic.s4-component-analysis/v1","source_revision":source,"global_problems":problems,
        "planned_execution_batches":expected.len(),"retained_execution_batches":actual.len(),"invalid_or_capped_batches":invalid_batches,
        "cases":cases,"boundary":"eight-call engine batches; F4 basis only versus full reference coordinate solve, extraction missing",
        "no_pipeline_or_rho_claim":true,"interval_scope":"deterministic paired-round bootstrap; fixed inputs, no independent-curve inference"});
    if extraction {
        result["schema"] = json!("gaudry.cubic.s4-extraction-analysis/v1");
        result["boundary"] = json!("complete native F4 basis/border/extraction engine batches; shared symbolic preparation/input generation/qualification excluded and reported");
        result["attempted_execution_batches"] = json!(starts.len());
        result["in_flight_batches_without_completion"] =
            json!(starts.len().saturating_sub(actual.len()));
    }
    if delayed {
        result["schema"] = json!("gaudry.cubic.s4-delayed-analysis/v1");
        result["reference_kind"] = json!(if scalar_reference { "scalar" } else { "dense" });
        result["comparison_metric"] = json!(metric);
    }
    Ok(result)
}

fn plot(summary: &Value, p: u64, path: &Path) -> Result<(), Box<dyn Error>> {
    let delayed = summary["schema"] == "gaudry.cubic.s4-delayed-analysis/v1";
    let extraction = delayed || summary["schema"] == "gaudry.cubic.s4-extraction-analysis/v1";
    let metric = if delayed {
        summary["comparison_metric"]
            .as_str()
            .ok_or("missing comparison metric")?
    } else if extraction {
        "complete_krylov_over_dense"
    } else {
        "basis_phase_over_complete_reference"
    };
    let cases: Vec<_> = summary["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["p"] == p)
        .collect();
    let ratio = |r: &&Value, i: usize| r[metric]["bootstrap_95"][i].as_f64();
    let lo = cases
        .iter()
        .filter_map(|r| ratio(r, 0))
        .fold(if extraction { 1.0f64 } else { 0.1f64 }, f64::min)
        * if extraction { 0.85 } else { 0.7 };
    let hi = cases
        .iter()
        .filter_map(|r| ratio(r, 1))
        .fold(if extraction { 1.0f64 } else { 2f64 }, f64::max)
        * if extraction { 1.1 } else { 1.4 };
    let root = SVGBackend::new(path, (1200, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            if delayed {
                format!(
                    "p{p}: complete delayed graph / {} coordinate solve",
                    summary["reference_kind"].as_str().unwrap_or("?")
                )
            } else if extraction {
                format!("p{p}: complete native F4 Krylov / dense coordinate solve")
            } else {
                format!("p{p}: F4 basis phase / complete S4 coordinate solve")
            },
            ("sans-serif", 24),
        )
        .margin(20)
        .x_label_area_size(65)
        .y_label_area_size(65)
        .build_cartesian_2d(0f64..cases.len().max(1) as f64, (lo..hi).log_scale())?;
    chart
        .configure_mesh()
        .y_desc(if extraction {
            "complete coordinate engine batch wall ratio"
        } else {
            "batch wall ratio; extraction still missing"
        })
        .y_labels(6)
        .max_light_lines(2)
        .y_label_formatter(&|v| format!("{v:.2}"))
        .x_labels(0)
        .draw()?;
    chart.draw_series(LineSeries::new(
        [(0., 1.), (cases.len() as f64, 1.)],
        &BLACK,
    ))?;
    for (i, r) in cases.iter().enumerate() {
        let (x, y) = chart.plotting_area().map_coordinate(&(i as f64 + 0.5, lo));
        root.draw(&Text::new(
            format!("s{}/r{}", r["seed"], r["ordinal"]),
            (x, y + 12),
            ("sans-serif", 16)
                .into_text_style(&root)
                .pos(Pos::new(HPos::Center, VPos::Top)),
        ))?;
        let v = &r[metric];
        if let (Some(mean), Some(a), Some(b)) = (
            v["geometric_mean"].as_f64(),
            v["bootstrap_95"][0].as_f64(),
            v["bootstrap_95"][1].as_f64(),
        ) {
            let color = if r["eligible_local_controlled_comparison"] == true {
                GREEN
            } else {
                RGBColor(140, 140, 140)
            };
            let x = i as f64 + 0.5;
            chart.draw_series([Rectangle::new(
                [(x - 0.28, lo), (x + 0.28, mean)],
                color.filled(),
            )])?;
            chart.draw_series([PathElement::new([(x, a), (x, b)], BLACK)])?;
        }
    }
    root.draw(&Text::new(if extraction { "Green: locally controlled; gray: retained invalid/incomplete. Complete coordinate phase; shared setup excluded." } else { "Green: locally controlled; gray: retained invalid/incomplete. Basis time is a lower bound, never a pipeline win." },(80,580),("sans-serif",15)))?;
    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        return Err("usage: s4-component-analysis RAW PLAN NEW-OUTPUT-DIR".into());
    }
    let plan: Value = serde_json::from_slice(&fs::read(&args[2])?)?;
    let mut rows = Vec::new();
    let mut failures = 0;
    for line in BufReader::new(fs::File::open(&args[1])?).lines() {
        match serde_json::from_str(&line?) {
            Ok(row) => rows.push(row),
            Err(_) => failures += 1,
        }
    }
    let summary = analyze(&plan, rows, failures)?;
    let dir = Path::new(&args[3]);
    fs::create_dir(dir)?;
    fs::write(
        dir.join("summary.json"),
        serde_json::to_vec_pretty(&summary)?,
    )?;
    for p in plan["primes"].as_array().unwrap() {
        let p = p.as_u64().unwrap();
        plot(&summary, p, &dir.join(format!("ratios-p{p}.svg")))?;
    }
    println!(
        "retained {} / {} batches; {} invalid/capped",
        summary["retained_execution_batches"],
        summary["planned_execution_batches"],
        summary["invalid_or_capped_batches"]
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extraction_requires_start_records_and_equal_complete_outputs() {
        let source = "1234567890123456789012345678901234567890";
        let plan = json!({"driver_kind":"s4-krylov-component","source_revision":source,
            "primes":[127],"seeds":[83],"residuals_per_instance":1,"rounds":5,"warmup_rounds":1,
            "aa_tolerance":1.1,"batch_iterations":2,"f4_max_degree":24,"f4_deadline_ms":0});
        let fixture = json!({"schema":"gaudry.cubic.s4-extraction/v1","record_type":"fixture",
            "source_revision":source,"p":127,"seed":83,"ordinal":0,"input_digest":"same","qualified_triples":[[1,2,3]]});
        let mut rows = vec![fixture];
        for (phase, count) in [("warmup", 1), ("measurement", 5)] {
            for round in 0..count {
                let mut arms = ["dense_a", "krylov_a", "dense_b", "krylov_b"];
                if round % 2 == 1 {
                    arms.reverse();
                }
                for arm in arms {
                    let base = json!({"schema":"gaudry.cubic.s4-extraction/v1","record_type":"execution_start",
                        "source_revision":source,"p":127,"seed":83,"ordinal":0,"phase":phase,"round":round,"arm":arm});
                    rows.push(base.clone());
                    let mut completed = base;
                    completed["record_type"] = json!("execution");
                    completed["batch_iterations"] = json!(2);
                    completed["f4_max_degree"] = json!(24);
                    completed["f4_deadline_ms"] = json!(0);
                    completed["valid"] = json!(true);
                    completed["intra_batch_consistent"] = json!(true);
                    completed["input_digest"] = json!("same");
                    completed["output_kind"] = json!("complete_S4_coordinate_solve");
                    completed["signature"] = json!([
                        [[1, 2, 3]],
                        if arm.starts_with("krylov") { 20 } else { 100 }
                    ]);
                    completed["phase_wall_ns"] =
                        json!(if arm.starts_with("krylov") { 20 } else { 100 });
                    rows.push(completed);
                }
            }
        }
        let good = analyze(&plan, rows.clone(), 0).unwrap();
        assert_eq!(good["cases"][0]["complete_phase_upper_below_one"], true);
        assert_eq!(good["attempted_execution_batches"], 24);
        assert!(good["cases"][0]["basis_phase_over_complete_reference"].is_null());
        let mut delayed_plan = plan.clone();
        delayed_plan["driver_kind"] = json!("s4-delayed-component");
        delayed_plan["reference_kind"] = json!("scalar");
        let mut delayed_rows = rows.clone();
        for r in &mut delayed_rows {
            r["schema"] = json!("gaudry.cubic.s4-delayed/v1");
            if let Some(arm) = r["arm"].as_str() {
                r["arm"] = json!(arm.replace("dense", "scalar").replace("krylov", "delayed"));
            }
            if r["record_type"] == "execution" {
                r["signature"][1] = json!(100);
                r["signature"].as_array_mut().unwrap().push(json!(64));
            }
        }
        let accepted = analyze(&delayed_plan, delayed_rows.clone(), 0).unwrap();
        assert_eq!(accepted["cases"][0]["complete_phase_upper_below_one"], true);
        assert!(accepted["cases"][0]["complete_krylov_over_dense"].is_null());
        delayed_rows[4]["signature"][1] = json!(99);
        assert_eq!(
            analyze(&delayed_plan, delayed_rows, 0).unwrap()["cases"][0]
                ["eligible_local_controlled_comparison"],
            false
        );
        rows[4]["signature"][0] = json!([[1, 2, 4]]);
        assert_eq!(
            analyze(&plan, rows.clone(), 0).unwrap()["cases"][0]
                ["eligible_local_controlled_comparison"],
            false
        );
        rows.remove(1);
        assert!(!analyze(&plan, rows, 0).unwrap()["global_problems"]
            .as_array()
            .unwrap()
            .is_empty());
    }
    #[test]
    fn bootstrap_and_incomplete_cohorts_cannot_claim_a_pipeline_win() {
        let e = estimate(&[2f64.ln(); 5], 7);
        assert!((e["geometric_mean"].as_f64().unwrap() - 2.).abs() < 1e-12);
        let plan = json!({"source_revision":"1234567890123456789012345678901234567890","primes":[67],"seeds":[59],
            "residuals_per_instance":1,"rounds":5,"warmup_rounds":1,"aa_tolerance":1.1});
        let s = analyze(&plan, vec![], 1).unwrap();
        assert_eq!(s["cases"][0]["eligible_local_controlled_comparison"], false);
        assert_eq!(s["cases"][0]["pipeline_or_rho_improvement_claim"], false);
        assert_eq!(s["planned_execution_batches"], 24);
        assert_eq!(s["retained_execution_batches"], 0);
    }

    #[test]
    fn complete_basis_phase_can_pass_without_claiming_pipeline_recovery() {
        let source = "1234567890123456789012345678901234567890";
        let plan = json!({"source_revision":source,"primes":[67],"seeds":[59],"residuals_per_instance":1,
            "rounds":5,"warmup_rounds":1,"aa_tolerance":1.1,"batch_iterations":8,"f4_max_degree":24,"f4_deadline_ms":50});
        let mut rows = vec![
            json!({"schema":"gaudry.cubic.s4-component/v1","record_type":"fixture",
            "source_revision":source,"p":67,"seed":59,"ordinal":0,"input_digest":"fixed"}),
        ];
        for (phase, count) in [("warmup", 1), ("measurement", 5)] {
            for round in 0..count {
                let mut arms = ARMS;
                if round % 2 == 1 {
                    arms.reverse();
                }
                for arm in arms {
                    let candidate = arm.starts_with("f4");
                    rows.push(json!({"schema":"gaudry.cubic.s4-component/v1","record_type":"execution",
                    "source_revision":source,"p":67,"seed":59,"ordinal":0,"phase":phase,"round":round,"arm":arm,
                    "input_digest":"fixed","valid":true,"intra_batch_consistent":true,"signature":if candidate {"basis"} else {"triples"},
                    "phase_wall_ns":if candidate {25} else {100},"batch_iterations":8,"f4_max_degree":24,"f4_deadline_ms":50}));
                }
            }
        }
        let good = analyze(&plan, rows.clone(), 0).unwrap();
        assert_eq!(
            good["cases"][0]["eligible_local_controlled_comparison"],
            true
        );
        assert_eq!(good["cases"][0]["basis_phase_upper_below_half"], true);
        assert_eq!(good["cases"][0]["pipeline_or_rho_improvement_claim"], false);
        rows[6]["signature"] = json!("changed");
        let bad = analyze(&plan, rows, 0).unwrap();
        assert_eq!(
            bad["cases"][0]["eligible_local_controlled_comparison"],
            false
        );
        assert_eq!(bad["cases"][0]["basis_phase_upper_below_half"], false);
    }
}
