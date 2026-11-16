//! Native analysis of retained matched IC/rho executions. Failures, incomplete
//! plans and warmups remain visible; none can generate a parity acceptance.
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const ARMS: [&str; 5] = ["pivot_a", "graph", "pivot_b", "rho_a", "rho_b"];
type Key = (u64, u64, String, String, u64, String);

fn number(row: &Value, key: &str) -> Result<f64, String> {
    row[key]
        .as_f64()
        .filter(|n| n.is_finite() && *n > 0.0)
        .ok_or_else(|| format!("invalid positive {key}"))
}

fn identity(row: &Value) -> Value {
    json!([
        row["field"],
        row["a"],
        row["b"],
        row["generator"],
        row["target"],
        row["subgroup_order"],
        row["known_answer_scalar"]
    ])
}

fn signature(row: &Value, rho: bool) -> Value {
    let report = &row["report"];
    if rho {
        json!([
            report["steps"],
            report["group_ops"],
            report["recovered_scalar"]
        ])
    } else {
        json!([
            report["residuals"],
            report["relations_verified"],
            report["oracle_fp_muls"],
            report["group_ops"],
            report["lp_histogram"],
            report["recovered_scalar"],
            report["lp_input_digest"]
        ])
    }
}

fn median(values: &[f64]) -> f64 {
    let mut values = values.to_vec();
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

fn estimate(log_ratios: &[f64], draws: usize, mut state: u64) -> Value {
    if log_ratios.is_empty() {
        return Value::Null;
    }
    let n = log_ratios.len();
    let mean = (log_ratios.iter().sum::<f64>() / n as f64).exp();
    let mut bootstrap = Vec::with_capacity(draws);
    for _ in 0..draws {
        let mut sum = 0.0;
        for _ in 0..n {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            sum += log_ratios[state as usize % n];
        }
        bootstrap.push((sum / n as f64).exp());
    }
    bootstrap.sort_by(f64::total_cmp);
    json!({"geometric_mean":mean,"bootstrap_95":[bootstrap[draws * 25 / 1000],bootstrap[(draws * 975 / 1000).min(draws - 1)]],"rounds":n})
}

fn analyze(plan: &Value, rows: Vec<Value>, parse_failures: usize) -> Result<Value, String> {
    let source = plan["source_revision"]
        .as_str()
        .filter(|s| s.len() == 40)
        .ok_or("runtime plan must pin source SHA")?;
    let primes = plan["primes"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("missing primes")?;
    let seeds = plan["seeds"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("missing seeds")?;
    let oracles = plan["oracles"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("missing oracles")?;
    let rounds = plan["rounds"]
        .as_u64()
        .filter(|n| *n >= 5)
        .ok_or("need five rounds")?;
    let warmups = plan["warmup_rounds"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("need warmup")?;
    let draws = plan["bootstrap_draws"]
        .as_u64()
        .filter(|n| *n >= 1000)
        .ok_or("need bootstrap draws")? as usize;
    let seed = plan["bootstrap_seed"]
        .as_u64()
        .filter(|n| *n != 0)
        .ok_or("need bootstrap seed")?;
    let tolerance = plan["aa_tolerance"]
        .as_f64()
        .filter(|n| *n >= 1.0)
        .ok_or("invalid AA tolerance")?;
    let mut executions = BTreeMap::<Key, Value>::new();
    let mut duplicate = BTreeSet::new();
    let mut problems = Vec::new();
    let mut identities = BTreeMap::new();
    let mut phase_counts = BTreeMap::<String, usize>::new();
    let mut actual_order = Vec::new();
    let starts_expected = plan["record_start_events"] == true;
    let mut starts_order = Vec::new();
    let mut in_flight = None;
    for row in rows {
        let is_start = row["record_type"] == "execution_start";
        if row["record_type"] != "execution" && !is_start {
            continue;
        }
        let p = row["p"].as_u64().ok_or("row missing p")?;
        let fixture_seed = row["seed"].as_u64().ok_or("row missing seed")?;
        let oracle = row["oracle_block"]
            .as_str()
            .ok_or("row missing oracle")?
            .to_owned();
        let phase = row["phase"].as_str().ok_or("row missing phase")?.to_owned();
        let round = row["round"].as_u64().ok_or("row missing round")?;
        let arm = row["arm"].as_str().ok_or("row missing arm")?.to_owned();
        if row["schema"] != "gaudry.cubic.graph-comparison/v2" || row["source_revision"] != source {
            problems.push(format!(
                "source/schema mismatch p={p} seed={fixture_seed} {oracle} {arm}"
            ));
        }
        if plan.get("sampler").is_some() && row["sampler"] != plan["sampler"] {
            problems.push(format!(
                "sampler treatment mismatch p={p} seed={fixture_seed}"
            ));
        }
        if !primes.contains(&json!(p))
            || !seeds.contains(&json!(fixture_seed))
            || !oracles.contains(&json!(oracle))
            || !ARMS.contains(&arm.as_str())
            || !matches!(phase.as_str(), "warmup" | "measurement")
            || round >= if phase == "warmup" { warmups } else { rounds }
        {
            problems.push(format!(
                "unexpected execution p={p} seed={fixture_seed} {oracle} {phase} {round} {arm}"
            ));
        }
        let id = identity(&row);
        let old = identities
            .entry((p, fixture_seed))
            .or_insert_with(|| id.clone());
        if old != &id {
            problems.push(format!("instance mismatch p={p} seed={fixture_seed}"));
        }
        let key = (p, fixture_seed, oracle, phase, round, arm);
        if is_start {
            if !starts_expected {
                problems.push(
                    "unexpected start record without frozen start-event treatment".to_owned(),
                );
            }
            if in_flight.replace(key.clone()).is_some() {
                problems.push("start precedes completion of previous arm".to_owned());
            }
            starts_order.push(key);
            continue;
        }
        *phase_counts.entry(key.3.clone()).or_default() += 1;
        if starts_expected && in_flight.take().as_ref() != Some(&key) {
            problems.push("execution lacks matching preceding start".to_owned());
        }
        actual_order.push(key.clone());
        if executions.insert(key.clone(), row).is_some() {
            duplicate.insert(key);
        }
    }
    if !duplicate.is_empty() {
        problems.push(format!("{} duplicate execution keys", duplicate.len()));
    }
    if parse_failures != 0 {
        problems.push(format!("{parse_failures} malformed raw lines retained"));
    }
    let mut expected_order = Vec::new();
    for p in primes {
        for fixture_seed in seeds {
            for oracle in oracles {
                for (phase, count) in [("warmup", warmups), ("measurement", rounds)] {
                    for round in 0..count {
                        let mut arms = ARMS;
                        if round % 2 == 1 {
                            arms.reverse();
                        }
                        for arm in arms {
                            expected_order.push((
                                p.as_u64().ok_or("invalid prime")?,
                                fixture_seed.as_u64().ok_or("invalid seed")?,
                                oracle.as_str().ok_or("invalid oracle")?.to_owned(),
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
    if actual_order.len() > expected_order.len()
        || actual_order != expected_order[..actual_order.len().min(expected_order.len())]
    {
        problems.push("execution order differs from the frozen interleaved schedule".to_owned());
    }
    if starts_expected
        && (starts_order.len() > expected_order.len()
            || starts_order != expected_order[..starts_order.len().min(expected_order.len())]
            || starts_order.len() < actual_order.len()
            || starts_order.len() > actual_order.len() + 1)
    {
        problems.push("start-event count/order differs from frozen schedule".to_owned());
    }
    if !plan["paired_semantic_oracles"].is_null() {
        let semantic = plan["paired_semantic_oracles"]
            .as_array()
            .ok_or("invalid semantic oracle pairs")?;
        for pair in semantic {
            if !plan["paired_oracles"]
                .as_array()
                .is_some_and(|pairs| pairs.contains(pair))
            {
                return Err("semantic oracle pair must also declare stream equality".into());
            }
        }
    }
    if let Some(pairs) = plan["paired_oracles"].as_array() {
        for pair in pairs {
            let reference = pair[0].as_str().ok_or("invalid paired oracle")?;
            let candidate = pair[1].as_str().ok_or("invalid paired oracle")?;
            if !oracles.contains(&json!(reference)) || !oracles.contains(&json!(candidate)) {
                return Err("paired oracle is outside frozen oracle set".into());
            }
            for (key, row) in &executions {
                if key.2 != reference || key.5.starts_with("rho_") {
                    continue;
                }
                let mut candidate_key = key.clone();
                candidate_key.2 = candidate.to_owned();
                let stream = |r: &Value| {
                    json!([
                        identity(r),
                        r["report"]["residuals"],
                        r["report"]["relations_verified"],
                        r["report"]["group_ops"],
                        r["report"]["recovered_scalar"],
                        r["report"]["lp_input_digest"]
                    ])
                };
                if executions
                    .get(&candidate_key)
                    .is_none_or(|other| stream(row) != stream(other))
                {
                    problems.push(format!("paired oracle stream mismatch {key:?}"));
                }
                if plan["paired_semantic_oracles"]
                    .as_array()
                    .is_some_and(|pairs| pairs.contains(pair))
                {
                    let reference_work = row["report"]["oracle_fp_muls"].as_u64();
                    let candidate_work = executions
                        .get(&candidate_key)
                        .and_then(|other| other["report"]["oracle_fp_muls"].as_u64());
                    if reference_work.is_none() || reference_work != candidate_work {
                        problems.push(format!("paired oracle semantic product mismatch {key:?}"));
                    }
                }
            }
        }
    }
    let mut strata = Vec::new();
    for p in primes {
        let p = p.as_u64().ok_or("invalid prime")?;
        for fixture_seed in seeds {
            let fixture_seed = fixture_seed.as_u64().ok_or("invalid seed")?;
            for oracle in oracles {
                let oracle = oracle.as_str().ok_or("invalid oracle")?;
                let mut reasons = Vec::new();
                let mut graph = Vec::new();
                let mut pivot = Vec::new();
                let mut graph_cost = Vec::new();
                let mut pivot_aa = Vec::new();
                let mut rho_aa = Vec::new();
                let mut complete = 0;
                let mut correct = 0;
                let mut n = 0;
                let mut group_share = Vec::new();
                let mut oracle_share = Vec::new();
                for (phase, count) in [("warmup", warmups), ("measurement", rounds)] {
                    for round in 0..count {
                        let mut block = Vec::new();
                        for arm in ARMS {
                            let key = (
                                p,
                                fixture_seed,
                                oracle.to_owned(),
                                phase.to_owned(),
                                round,
                                arm.to_owned(),
                            );
                            if let Some(row) = executions.get(&key) {
                                block.push(row);
                            }
                        }
                        if block.len() != ARMS.len() {
                            reasons.push(format!(
                                "{phase} round {round}: incomplete block ({}/5 arms)",
                                block.len()
                            ));
                            continue;
                        }
                        if phase == "measurement" {
                            complete += 1;
                        }
                        let pa = block[0];
                        let g = block[1];
                        let pb = block[2];
                        let ra = block[3];
                        let rb = block[4];
                        n = g["subgroup_order"].as_u64().unwrap_or(0);
                        let solved = block.iter().all(|r| {
                            r["group_replay"] == true
                                && r["report"]["correct"] == true
                                && r["report"]["solved"] == true
                                && r["report"]["recovered_scalar"] == r["known_answer_scalar"]
                        });
                        let same_input = signature(pa, false) == signature(g, false)
                            && signature(pa, false) == signature(pb, false)
                            && !g["report"]["lp_input_digest"].is_null()
                            && signature(ra, true) == signature(rb, true);
                        if !solved || !same_input {
                            reasons.push(format!(
                                "{phase} round {round}: solve/input/control failure"
                            ));
                        } else if phase == "measurement" {
                            correct += 1;
                        }
                        if phase == "warmup" {
                            continue;
                        }
                        // Ineligible blocks still contribute spent-work diagnostics.
                        let times = block
                            .iter()
                            .map(|r| number(r, "unadmitted_wall_ns"))
                            .collect::<Result<Vec<_>, _>>()?;
                        let costs = block
                            .iter()
                            .map(|r| number(r, "legacy_model_cold_gae"))
                            .collect::<Result<Vec<_>, _>>()?;
                        let rho_time = (times[3].ln() + times[4].ln()) / 2.0;
                        graph.push(times[1].ln() - rho_time);
                        pivot.push((times[0].ln() + times[2].ln()) / 2.0 - rho_time);
                        graph_cost.push(costs[1].ln() - (costs[3].ln() + costs[4].ln()) / 2.0);
                        pivot_aa.push((times[0] / times[2]).ln());
                        rho_aa.push((times[3] / times[4]).ln());
                        let total = number(&g["report"], "total_ops")?;
                        let fp_price = number(&g["report"], "fp_muls_per_add")?;
                        group_share.push(g["report"]["group_ops"].as_f64().unwrap_or(0.0) / total);
                        oracle_share.push(
                            g["report"]["oracle_fp_muls"].as_f64().unwrap_or(0.0)
                                / fp_price
                                / total,
                        );
                    }
                }
                let aa_pass = !rho_aa.is_empty()
                    && median(&rho_aa).abs() <= tolerance.ln()
                    && median(&pivot_aa).abs() <= tolerance.ln();
                if !aa_pass {
                    reasons.push("local median AA tolerance failed or unavailable".to_owned());
                }
                let eligible = reasons.is_empty()
                    && problems.is_empty()
                    && complete == rounds
                    && correct == rounds;
                let graph_estimate = estimate(&graph, draws, seed);
                let min_n = plan["parity_min_subgroup_order"]
                    .as_u64()
                    .ok_or("missing minimum order")?;
                let local_parity = eligible
                    && n >= min_n
                    && graph_estimate["bootstrap_95"][1]
                        .as_f64()
                        .is_some_and(|x| x <= 1.0);
                strata.push(json!({"p":p,"seed":fixture_seed,"oracle":oracle,"subgroup_order":n,
                    "planned_rounds":rounds,"complete_rounds":complete,"correct_rounds":correct,
                    "eligible_local_controlled_comparison":eligible,"invalid_reasons":reasons,
                    "diagnostic_graph_wall_over_rho":graph_estimate,"diagnostic_pivot_wall_over_rho":estimate(&pivot,draws,seed),
                    "diagnostic_graph_legacy_cost_over_rho":estimate(&graph_cost,draws,seed),
                    "median_pivot_aa_ratio":if pivot_aa.is_empty(){Value::Null}else{json!(median(&pivot_aa).exp())},
                    "median_rho_aa_ratio":if rho_aa.is_empty(){Value::Null}else{json!(median(&rho_aa).exp())},
                    "diagnostic_legacy_group_share":if group_share.is_empty(){Value::Null}else{json!(median(&group_share))},
                    "diagnostic_legacy_oracle_share":if oracle_share.is_empty(){Value::Null}else{json!(median(&oracle_share))},
                    "large_order_local_reference_parity":local_parity}));
            }
        }
    }
    let mut oracle_comparisons = Vec::new();
    if let Some(pairs) = plan["paired_oracles"].as_array() {
        for pair in pairs {
            let reference = pair[0].as_str().ok_or("invalid paired oracle")?;
            let candidate = pair[1].as_str().ok_or("invalid paired oracle")?;
            for p in primes {
                for fixture_seed in seeds {
                    let mut log_ratios = Vec::new();
                    for round in 0..rounds {
                        let key = |oracle: &str| {
                            (
                                p.as_u64().unwrap(),
                                fixture_seed.as_u64().unwrap(),
                                oracle.to_owned(),
                                "measurement".to_owned(),
                                round,
                                "graph".to_owned(),
                            )
                        };
                        if let (Some(a), Some(b)) = (
                            executions.get(&key(reference)),
                            executions.get(&key(candidate)),
                        ) {
                            log_ratios.push(
                                number(b, "unadmitted_wall_ns")?.ln()
                                    - number(a, "unadmitted_wall_ns")?.ln(),
                            );
                        }
                    }
                    let eligible = log_ratios.len() == rounds as usize
                        && [reference, candidate].iter().all(|oracle| {
                            strata.iter().any(|s| {
                                s["p"] == *p
                                    && s["seed"] == *fixture_seed
                                    && s["oracle"] == *oracle
                                    && s["eligible_local_controlled_comparison"] == true
                            })
                        });
                    oracle_comparisons.push(json!({"p":p,"seed":fixture_seed,"reference":reference,"candidate":candidate,
                        "candidate_graph_wall_over_reference":estimate(&log_ratios,draws,seed),"eligible_local_controlled_comparison":eligible}));
                }
            }
        }
    }
    let mut mechanism_targets = Vec::new();
    let reference_oracle = oracles[0].as_str().ok_or("invalid reference oracle")?;
    for p in primes {
        for fixture_seed in seeds {
            let key = (
                p.as_u64().unwrap(),
                fixture_seed.as_u64().unwrap(),
                reference_oracle.to_owned(),
                "measurement".to_owned(),
                0,
                "graph".to_owned(),
            );
            if let Some(row) = executions.get(&key) {
                let report = &row["report"];
                let counters = &report["relation_point_stats"];
                if counters.is_object() {
                    mechanism_targets.push(json!({"p":p,"seed":fixture_seed,"oracle":reference_oracle,
                        "residuals":report["residuals"],"group_ops":report["group_ops"],"oracle_fp_muls":report["oracle_fp_muls"],
                        "sampler_setup_group_ops":report["sampler_setup_group_ops"],"sampling_group_ops":report["sampling_group_ops"],
                        "sampler_restarts":report["sampler_restarts"],"relation_point_stats":counters,
                        "first_repeat_at_recovery":!counters["first_repeat_residual"].is_null()
                            && counters["first_repeat_residual"] == report["residuals"] && report["correct"] == true}));
                }
            }
        }
    }
    let repeat_at_recovery = mechanism_targets
        .iter()
        .filter(|r| r["first_repeat_at_recovery"] == true)
        .count();
    let mut result = json!({"schema":"gaudry.cubic.local-analysis/v1","source_revision":source,"backend":"local_user_authorized",
        "standard_lab_admission":false,"reference_scope":plan["reference_scope"],"sampler":plan["sampler"],"raw_execution_count":actual_order.len(),
        "unique_execution_count":executions.len(),"planned_execution_count":expected_order.len(),
        "phase_counts":phase_counts,"global_problems":problems,"strata":strata,
        "oracle_comparisons":oracle_comparisons,
        "relation_mechanism":{"targets":mechanism_targets,"first_repeat_at_recovery_count":repeat_at_recovery,
            "scope":"first measured graph row of the first declared oracle per target; diagnostic counters, not a causal proof"},
        "extended_summary_status":"post-call summary of the predeclared paired-oracle and epoch controls; original acceptance fields unchanged",
        "interval_scope":"deterministic round bootstrap; timing repeat variation, not independent curve samples",
        "cost_scope":"legacy multiplication-equivalent diagnostics with unpriced work",
        "failed_blocks":"retained in spent-work diagnostics; invalidate their stratum"});
    if starts_expected {
        result["attempted_execution_count"] = json!(starts_order.len());
        result["in_flight_without_completion"] = json!(in_flight);
        result["in_flight_execution_count"] = json!(usize::from(in_flight.is_some()));
    }
    Ok(result)
}

fn plot(result: &Value, path: &Path) -> Result<(), Box<dyn Error>> {
    let strata = result["strata"].as_array().ok_or("missing strata")?;
    let max = strata
        .iter()
        .flat_map(|s| {
            [
                "diagnostic_graph_wall_over_rho",
                "diagnostic_pivot_wall_over_rho",
            ]
            .map(|k| s[k]["bootstrap_95"][1].as_f64().unwrap_or(1.0))
        })
        .fold(2.0_f64, f64::max)
        * 1.4;
    let min = strata
        .iter()
        .flat_map(|s| {
            [
                "diagnostic_graph_wall_over_rho",
                "diagnostic_pivot_wall_over_rho",
            ]
            .map(|k| s[k]["bootstrap_95"][0].as_f64().unwrap_or(1.0))
        })
        .fold(0.1_f64, f64::min)
        * 0.7;
    let width = 1200u32.max(strata.len() as u32 * 125);
    let root = SVGBackend::new(path, (width, 620)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Local cold wall time / same-instance rho",
            ("sans-serif", 24),
        )
        .margin(25)
        .x_label_area_size(90)
        .y_label_area_size(70)
        .build_cartesian_2d(0f64..strata.len().max(1) as f64, (min..max).log_scale())?;
    chart
        .configure_mesh()
        .y_labels(6)
        .max_light_lines(2)
        .y_label_formatter(&|v| format!("{v:.2}"))
        .x_labels(0)
        .x_label_style(("sans-serif", 11))
        .y_desc("ratio, log scale (lower costs less)")
        .draw()?;
    chart.draw_series(LineSeries::new(
        [(0.0, 1.0), (strata.len() as f64, 1.0)],
        &BLACK,
    ))?;
    for (i, s) in strata.iter().enumerate() {
        let (x, y) = chart.plotting_area().map_coordinate(&(i as f64 + 0.5, min));
        let name = match s["oracle"].as_str().unwrap_or("?") {
            "MeetInTheMiddle" => "Hybrid triple",
            "Groebner" => "S4",
            "PairOnly" => "Pair",
            "SymmetricPairOnly" => "Symmetric pair",
            "PreparedSymmetricPairOnly" => "Prepared pair",
            "PseudoRemainderPairOnly" => "Pseudo pair",
            "DelayedPseudoRemainderPairOnly" => "Delayed pair",
            "F4Krylov" => "F4 sparse FGLM",
            "F4KrylovDelayed" => "F4 delayed graph",
            "F4TraceDelayed" => "F4 certified trace",
            "F4TracePairGraph" => "F4 pair certificates",
            "F4Groebner" => "Native F4",
            "SymmetricMeetInTheMiddle" => "Symmetric triple",
            other => other,
        };
        let label_style = ("sans-serif", 16)
            .into_text_style(&root)
            .pos(Pos::new(HPos::Center, VPos::Top));
        root.draw(&Text::new(
            format!("p{} s{}", s["p"], s["seed"]),
            (x, y + 10),
            label_style.clone(),
        ))?;
        root.draw(&Text::new(name, (x, y + 31), label_style))?;
        for (j, key, color) in [
            (0, "diagnostic_pivot_wall_over_rho", BLUE),
            (1, "diagnostic_graph_wall_over_rho", RED),
        ] {
            if let Some(value) = s[key]["geometric_mean"].as_f64() {
                let color = if s["eligible_local_controlled_comparison"] == true {
                    color.mix(0.8)
                } else {
                    RGBColor(140, 140, 140).mix(0.8)
                };
                chart.draw_series([Rectangle::new(
                    [
                        (i as f64 + 0.12 + j as f64 * 0.38, min),
                        (i as f64 + 0.43 + j as f64 * 0.38, value.max(min)),
                    ],
                    color.filled(),
                )])?;
                if let (Some(lo), Some(hi)) = (
                    s[key]["bootstrap_95"][0].as_f64(),
                    s[key]["bootstrap_95"][1].as_f64(),
                ) {
                    let center = i as f64 + 0.275 + j as f64 * 0.38;
                    chart.draw_series([
                        PathElement::new(vec![(center, lo), (center, hi)], BLACK),
                        PathElement::new(vec![(center - 0.05, lo), (center + 0.05, lo)], BLACK),
                        PathElement::new(vec![(center - 0.05, hi), (center + 0.05, hi)], BLACK),
                    ])?;
                }
            }
        }
    }
    root.draw(&Text::new("Blue: pivot; red: graph; gray: failed controls/incomplete case. Ratios remain diagnostics for those cases.",(35,600),("sans-serif",14)))?;
    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 4 {
        return Err(
            "usage: cubic-graph-analysis RAW.jsonl RUNTIME-PLAN.json NEW-OUTPUT-DIR".into(),
        );
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
    let result = analyze(&plan, rows, failures)?;
    let dir = Path::new(&args[3]);
    fs::create_dir(dir)?;
    let mut out = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join("summary.json"))?;
    serde_json::to_writer_pretty(&mut out, &result)?;
    writeln!(out)?;
    plot(&result, &dir.join("ratios.svg"))?;
    let names: BTreeSet<_> = result["strata"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|s| s["oracle"].as_str())
        .collect();
    for name in names {
        let mut subset = result.clone();
        subset["strata"] = json!(result["strata"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["oracle"] == name)
            .collect::<Vec<_>>());
        plot(&subset, &dir.join(format!("ratios-{name}.svg")))?;
    }
    for s in result["strata"].as_array().unwrap() {
        println!(
            "p={} seed={} {} graph/rho={} pivot/rho={} controlled={} correct={}/{}",
            s["p"],
            s["seed"],
            s["oracle"],
            s["diagnostic_graph_wall_over_rho"]["geometric_mean"],
            s["diagnostic_pivot_wall_over_rho"]["geometric_mean"],
            s["eligible_local_controlled_comparison"],
            s["correct_rounds"],
            s["planned_rounds"]
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> (Value, Vec<Value>) {
        let source = "1234567890123456789012345678901234567890";
        let plan = json!({"source_revision":source,"primes":[67],"seeds":[3],"oracles":["PairOnly"],"rounds":5,"warmup_rounds":1,"bootstrap_draws":1000,"bootstrap_seed":7,"aa_tolerance":1.1,"parity_min_subgroup_order":1073741824});
        let mut rows = Vec::new();
        for (phase, count) in [("warmup", 1), ("measurement", 5)] {
            for round in 0..count {
                let mut arms = ARMS;
                if round % 2 == 1 {
                    arms.reverse();
                }
                for arm in arms {
                    rows.push(json!({"schema":"gaudry.cubic.graph-comparison/v2","record_type":"execution","source_revision":source,"p":67,"seed":3,"oracle_block":"PairOnly","phase":phase,"round":round,"arm":arm,"field":{"p":67},"a":[1],"b":[2],"generator":[3],"target":[4],"subgroup_order":300001,"known_answer_scalar":13,"group_replay":true,"unadmitted_wall_ns":if arm=="graph"{50}else{100},"legacy_model_cold_gae":100,"report":{"correct":true,"solved":true,"recovered_scalar":13,"steps":4,"group_ops":10,"oracle_fp_muls":100,"relations_verified":2,"residuals":3,"lp_histogram":[1],"lp_input_digest":"same","total_ops":100,"fp_muls_per_add":60}}));
                }
            }
        }
        (plan, rows)
    }
    #[test]
    fn complete_case_counts_warmups_and_rejects_tiny_parity() {
        let (p, r) = fixture();
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["phase_counts"]["warmup"], 5);
        assert_eq!(s["strata"][0]["eligible_local_controlled_comparison"], true);
        assert!(
            (s["strata"][0]["diagnostic_graph_wall_over_rho"]["geometric_mean"]
                .as_f64()
                .unwrap()
                - 0.5)
                .abs()
                < 1e-12
        );
        assert_eq!(s["strata"][0]["large_order_local_reference_parity"], false);
    }
    #[test]
    fn start_events_count_in_flight_work_and_reject_unmatched_completions() {
        let (mut plan, executions) = fixture();
        plan["record_start_events"] = json!(true);
        let mut rows = Vec::new();
        for r in executions {
            let mut start = r.clone();
            start["record_type"] = json!("execution_start");
            rows.push(start);
            rows.push(r);
        }
        let good = analyze(&plan, rows.clone(), 0).unwrap();
        assert_eq!(good["attempted_execution_count"], 30);
        assert_eq!(good["raw_execution_count"], 30);
        assert_eq!(good["phase_counts"]["warmup"], 5);
        assert_eq!(good["in_flight_execution_count"], 0);
        assert_eq!(
            good["strata"][0]["eligible_local_controlled_comparison"],
            true
        );
        let mut partial = rows.clone();
        partial.pop();
        let s = analyze(&plan, partial, 0).unwrap();
        assert_eq!(s["attempted_execution_count"], 30);
        assert_eq!(s["raw_execution_count"], 29);
        assert_eq!(s["in_flight_execution_count"], 1);
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
        rows.remove(0);
        let bad = analyze(&plan, rows, 0).unwrap();
        assert!(!bad["global_problems"].as_array().unwrap().is_empty());
        assert_eq!(
            bad["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
    }
    #[test]
    fn a_failed_execution_invalidates_case_without_dropping_spent_work() {
        let (p, mut r) = fixture();
        r[6]["report"]["correct"] = json!(false);
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
        assert_eq!(s["strata"][0]["complete_rounds"], 5);
        assert_eq!(
            s["strata"][0]["diagnostic_graph_wall_over_rho"]["rounds"],
            5
        );
    }
    #[test]
    fn incomplete_cohort_never_accepts() {
        let (p, mut r) = fixture();
        r.pop();
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(s["strata"][0]["complete_rounds"], 4);
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
    }
    #[test]
    fn source_identity_and_duplicate_failures_remain_global() {
        let (p, mut r) = fixture();
        let mut copy = r[0].clone();
        copy["target"] = json!([99]);
        r.push(copy);
        r[9]["source_revision"] = json!("other");
        let s = analyze(&p, r, 1).unwrap();
        assert_eq!(s["raw_execution_count"], 31);
        assert_eq!(s["unique_execution_count"], 30);
        assert!(s["global_problems"].as_array().unwrap().len() >= 4);
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
    }
    #[test]
    fn sampler_metadata_cannot_mix_treatments() {
        let (mut p, mut r) = fixture();
        p["sampler"] = json!("fresh");
        for row in &mut r {
            row["sampler"] = json!("fresh");
        }
        r[6]["sampler"] = json!("r-adding");
        let s = analyze(&p, r, 0).unwrap();
        assert!(!s["global_problems"].as_array().unwrap().is_empty());
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
    }

    #[test]
    fn paired_oracles_must_preserve_relation_streams() {
        let (mut p, mut rows) = fixture();
        p["oracles"] = json!(["PairOnly", "PreparedSymmetricPairOnly"]);
        p["paired_oracles"] = json!([["PairOnly", "PreparedSymmetricPairOnly"]]);
        rows[6]["report"]["relation_point_stats"] = json!({"decomposed_residuals":2,
            "distinct_abscissae":1,"repeated_abscissae":1,"first_repeat_residual":3});
        let mut candidate = rows.clone();
        for r in &mut candidate {
            r["oracle_block"] = json!("PreparedSymmetricPairOnly");
            // Genuine arithmetic savings are allowed; the admitted relation
            // stream, group work and recovered scalar must still agree.
            r["report"]["oracle_fp_muls"] = json!(75);
            if r["arm"] == "graph" {
                r["unadmitted_wall_ns"] = json!(75);
            }
        }
        rows.extend(candidate);
        let good = analyze(&p, rows.clone(), 0).unwrap();
        assert!(good["global_problems"].as_array().unwrap().is_empty());
        let ratio = good["oracle_comparisons"][0]["candidate_graph_wall_over_reference"]
            ["geometric_mean"]
            .as_f64()
            .unwrap();
        assert!((ratio - 1.5).abs() < 1e-12);
        assert_eq!(
            good["relation_mechanism"]["first_repeat_at_recovery_count"],
            1
        );
        rows[36]["report"]["lp_input_digest"] = json!("changed");
        let s = analyze(&p, rows, 0).unwrap();
        assert!(s["global_problems"].as_array().unwrap().iter().any(|p| p
            .as_str()
            .unwrap()
            .contains("paired oracle stream mismatch")));
        assert_eq!(
            s["strata"][1]["eligible_local_controlled_comparison"],
            false
        );
    }

    #[test]
    fn semantic_ablation_rejects_changed_or_missing_product_counts() {
        let (mut p, mut rows) = fixture();
        p["oracles"] = json!(["PairOnly", "DelayedPseudoRemainderPairOnly"]);
        p["paired_oracles"] = json!([["PairOnly", "DelayedPseudoRemainderPairOnly"]]);
        p["paired_semantic_oracles"] = p["paired_oracles"].clone();
        let mut candidate = rows.clone();
        for row in &mut candidate {
            row["oracle_block"] = json!("DelayedPseudoRemainderPairOnly");
        }
        rows.extend(candidate);
        assert!(analyze(&p, rows.clone(), 0).unwrap()["global_problems"]
            .as_array()
            .unwrap()
            .is_empty());
        for missing in [false, true] {
            let mut tampered = rows.clone();
            for row in &mut tampered {
                if row["oracle_block"] == "DelayedPseudoRemainderPairOnly"
                    && !row["arm"].as_str().unwrap().starts_with("rho_")
                {
                    if missing {
                        row["report"]
                            .as_object_mut()
                            .unwrap()
                            .remove("oracle_fp_muls");
                    } else {
                        row["report"]["oracle_fp_muls"] = json!(999999);
                    }
                }
            }
            let s = analyze(&p, tampered, 0).unwrap();
            assert!(s["global_problems"]
                .as_array()
                .unwrap()
                .iter()
                .any(|reason| reason
                    .as_str()
                    .unwrap()
                    .contains("semantic product mismatch")));
            assert!(s["strata"]
                .as_array()
                .unwrap()
                .iter()
                .all(|s| s["eligible_local_controlled_comparison"] == false));
        }
        p["paired_semantic_oracles"] = json!([["PairOnly", "missing"]]);
        assert!(analyze(&p, rows, 0).is_err());
    }

    #[test]
    fn input_mismatch_and_aa_timing_failure_are_rejected() {
        let (p, mut r) = fixture();
        for row in &mut r {
            if row["arm"] == "rho_b" {
                row["unadmitted_wall_ns"] = json!(200);
            }
        }
        r[6]["report"]["lp_input_digest"] = json!("different");
        let s = analyze(&p, r, 0).unwrap();
        assert_eq!(
            s["strata"][0]["eligible_local_controlled_comparison"],
            false
        );
        assert!(s["strata"][0]["invalid_reasons"].as_array().unwrap().len() >= 2);
    }
}
