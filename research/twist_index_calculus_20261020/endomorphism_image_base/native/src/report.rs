//! Native recomputation of study summaries, keeping target-level rows.
use super::*;
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&std::fs::read(p).unwrap()).unwrap()
}
/// Run-report metadata and recorded observations only; no timing selection.
pub(crate) fn job_reports(registry: &Path, out: &Path) {
    use std::path::PathBuf;
    fn files(root: &Path, found: &mut Vec<PathBuf>) {
        if !root.exists() {
            return;
        }
        for e in std::fs::read_dir(root).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            if e.file_type().unwrap().is_dir() {
                if !["source", "input", "previous-records"]
                    .contains(&e.file_name().to_str().unwrap())
                {
                    files(&p, found);
                }
            } else {
                found.push(p);
            }
        }
    }
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut index = Vec::new();
    for j in read(registry)["jobs"].as_array().unwrap() {
        let id = j["id"].as_str().unwrap();
        let cache = Path::new(j["cache"].as_str().unwrap());
        let status = read(cache.join("status.json"));
        assert!(["done", "interrupted", "timeout", "error", "failed"]
            .iter()
            .any(|s| status["phase"] == *s));
        let mut paths = Vec::new();
        files(&cache.join("out"), &mut paths);
        paths.sort();
        let mut exits = BTreeMap::<String, usize>::new();
        let mut summaries = BTreeMap::<String, usize>::new();
        let mut audits = BTreeMap::<String, usize>::new();
        let mut cold = Vec::new();
        let mut diagnostics = Vec::new();
        let mut incomplete = Vec::new();
        for p in paths {
            let name = p.file_name().unwrap().to_str().unwrap();
            if name.ends_with(".exit") {
                let code = std::fs::read_to_string(&p).unwrap().trim().to_owned();
                *exits.entry(code).or_default() += 1;
            } else if name == "audit.json" {
                let v = read(&p);
                *audits
                    .entry(v["status"].as_str().unwrap().into())
                    .or_default() += 1;
                diagnostics.push(json!({"path":p,"p":v["p"],"seed":v["seed"],"arm":v["arm"],"rank":v["rank"],"columns":v["columns"],"signed_relations":v["signed_relations"],"population":"independently audited processed prefix"}));
            } else if name == "summary.json" {
                let v = read(&p);
                if let Some(s) = v["status"].as_str() {
                    *summaries.entry(s.into()).or_default() += 1;
                }
                if v["arm"].is_string() && v["signed_relations"].is_u64() {
                    diagnostics.push(json!({"path":p,"p":v["p"],"seed":v["seed"],"arm":v["arm"],"rank":v["rank"],"columns":v["columns"],"signed_relations":v["signed_relations"],"population":"candidate processed prefix"}));
                } else if let Some(arms) = v["arms"].as_array() {
                    for a in arms {
                        if a["arm"].is_string() && a["signed_relations"].is_u64() {
                            diagnostics.push(json!({"path":p,"p":v["p"],"seed":v["seed"],"arm":a["arm"],"rank":a["rank"],"columns":a["columns"],"signed_relations":a["signed_relations"],"population":"reference full frozen stream"}));
                        }
                    }
                }
            } else if name == "cold.json" {
                let v: Value = match serde_json::from_slice(&std::fs::read(&p).unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        incomplete.push(json!({"path":p,"reason":e.to_string()}));
                        continue;
                    }
                };
                let d = &v["deterministic"];
                let arm = d["arm"].as_str().unwrap();
                let parent = p.parent().unwrap().parent().unwrap();
                let relative = p.strip_prefix(cache.join("out")).unwrap();
                let pass = relative
                    .components()
                    .next()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap();
                let measured = pass.starts_with("round");
                let origin = [
                    parent.join(format!("{arm}.origin-job.txt")),
                    parent.join("job-id.txt"),
                ]
                .into_iter()
                .find_map(|p| std::fs::read_to_string(p).ok())
                .unwrap_or_else(|| id.to_owned())
                .trim()
                .to_owned();
                let iso = std::fs::read(parent.join(format!("{arm}.isolation.json")))
                    .ok()
                    .and_then(|b| serde_json::from_slice::<Value>(&b).ok());
                let admitted = measured
                    && iso
                        .as_ref()
                        .is_some_and(super::cold_catalog::resource_valid);
                cold.push(json!({"p":d["p"],"seed":d["seed"],"arm":arm,"pass":pass,"origin_job":origin,"copied_from_other_job":origin!=id,"status":d["status"],"rank":d["rank"],"columns":d["columns"],"signed_relations":d["signed_relations"],"systems":d["systems"],"undetermined":d["undetermined"],"cold_ns":v["timing"]["cold_ns"],"resource_admitted":admitted,"timing":v["timing"],"memory":v["memory"]}));
            }
        }
        let command_sha = std::fs::read(cache.join("cmd.sh"))
            .ok()
            .map(|b| hex::encode(crate::hash::sha256::sha256(&b)));
        let measured = cold
            .iter()
            .filter(|r| r["resource_admitted"] == true && r["copied_from_other_job"] == false)
            .count();
        let report = json!({"schema":"twist-image-run-observations/v1","job":id,"kind":j["kind"],"notes":j["notes"],"status":status,"command_sha256":command_sha,"exit_counts":exits,"summary_status_counts":summaries,"audit_status_counts":audits,"diagnostic_records":diagnostics,"cold_records":cold,"incomplete_records":incomplete,"admitted_timings_originating_here":measured,"isolation_rule":"only resource-admitted timings from this origin may be plotted as wall measurements; censored intervals are consumed cost only"});
        write(&out.join(format!("{id}.json")), &report);
        index.push(json!({"job":id,"kind":j["kind"],"phase":status["phase"],"exit_code":status["exit_code"],"exit_counts":exits,"summary_status_counts":summaries,"audit_status_counts":audits,"admitted_timings_originating_here":measured}));
    }
    write(
        &out.join("index.json"),
        &json!({"schema":"twist-image-run-report-index/v1","jobs":index}),
    );
}
fn number(v: &Value, k: &str) -> u64 {
    v[k].as_u64().unwrap()
}
fn median(mut v: Vec<u64>) -> u64 {
    assert!(!v.is_empty());
    v.sort_unstable();
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        v[v.len() / 2 - 1] + (v[v.len() / 2] - v[v.len() / 2 - 1]) / 2
    }
}
fn float_summary(mut v: Vec<f64>) -> Value {
    assert!(!v.is_empty() && v.iter().all(|x| x.is_finite() && *x > 0.0));
    v.sort_by(f64::total_cmp);
    json!({"min":v[0],"median":v[v.len()/2],"max":v[v.len()-1]})
}
fn paired_costs(rows: &[Value]) -> Vec<Value> {
    let mut pairs: Vec<_> = ARMS
        .iter()
        .filter(|a| **a != "original")
        .map(|a| ("original", *a))
        .collect();
    pairs.extend([
        ("double-l0-active", "phi-l0-active"),
        ("l0-peel", "phi-l0-active"),
    ]);
    let mut result = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let get = |arm: &str| {
                rows.iter()
                    .find(|r| r["p"] == p && r["seed"] == seed && r["arm"] == arm)
                    .unwrap()
            };
            let original = get("original");
            for &(reference, candidate) in &pairs {
                let a = get(reference);
                let b = get(candidate);
                let mut admitted = a["isolation_admitted"] == true
                    && b["isolation_admitted"] == true
                    && original["isolation_admitted"] == true
                    && a["successful_solution"] == true
                    && b["successful_solution"] == true;
                let mut rounds = Vec::new();
                let mut ratios = Vec::new();
                for round in 0..5 {
                    let find = |r: &Value| {
                        r["rounds"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|v| v["round"] == round)
                            .unwrap()
                            .clone()
                    };
                    let ar = find(a);
                    let br = find(b);
                    if ar["status"] == "missing" || br["status"] == "missing" {
                        admitted = false;
                        continue;
                    }
                    let same = ar["origin_job"].as_str().is_some_and(|x| !x.is_empty())
                        && ar["origin_job"] == br["origin_job"];
                    admitted &= same
                        && ar["isolation_admitted"] == true
                        && br["isolation_admitted"] == true;
                    let ratio = number(&ar["timing"], "cold_ns") as f64
                        / number(&br["timing"], "cold_ns") as f64;
                    ratios.push(ratio);
                    rounds.push(json!({"round":round,"origin_job":ar["origin_job"],"same_origin":same,"reference_cold_ns":ar["timing"]["cold_ns"],"candidate_cold_ns":br["timing"]["cold_ns"],"reference_over_candidate":ratio}));
                }
                admitted &= rounds.len() == 5;
                result.push(json!({"p":p,"seed":seed,"reference_arm":reference,"candidate_arm":candidate,"admitted":admitted,"reference_over_candidate_cold":if admitted {float_summary(ratios.clone())}else{Value::Null},"all_five_reference_over_candidate_above_one":if admitted{json!(ratios.iter().all(|x|*x>1.0))}else{Value::Null},"original_aa_pairs":original["aa_pairs"],"rounds":rounds,"scope":"complete instrumented cold elapsed intervals on one public target; no GAE, external-fastest, rho or scaling admission"}));
            }
        }
    }
    result
}
fn isolation(path: &Path) -> (bool, Value) {
    if !path.exists() {
        return (false, json!({"reason":"missing isolation receipt"}));
    }
    let text = std::fs::read_to_string(path).unwrap();
    let lines: Vec<_> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(lines.len(), 1, "each measured execution has one receipt");
    let v: Value = serde_json::from_str(lines[0]).unwrap();
    let pass = super::cold_catalog::resource_valid(&v);
    (pass, v)
}
pub(crate) fn summarize(args: &[String]) {
    assert_eq!(args.len(), 9);
    let tape = Path::new(&args[0]);
    let refs = Path::new(&args[1]);
    let candidates = Path::new(&args[2]);
    let audits = Path::new(&args[3]);
    let double_refs = Path::new(&args[4]);
    let double_candidates = Path::new(&args[5]);
    let double_audits = Path::new(&args[6]);
    let cold = Path::new(&args[7]);
    let out = Path::new(&args[8]);
    assert!(!out.exists());
    std::fs::create_dir_all(out).unwrap();
    let mut all = Vec::new();
    let mut full_rows = Vec::new();
    let mut cold_rows = Vec::new();
    let mut rank_csv = String::from("p,seed,arm,queries,rank,columns\n");
    for (pi, p) in PRIMES.iter().enumerate() {
        for seed in 0..3 {
            let stem = format!("p{p}-seed{seed}");
            let case = Case::new(load(tape, INPUT_SHA, *p, seed), out);
            let mut original_sets = None;
            for (ai, arm) in ARMS.iter().enumerate() {
                let reference_path = if ai == 9 {
                    double_refs.join(&stem).join(format!("{arm}.json"))
                } else {
                    refs.join(&stem).join(format!("{arm}.json"))
                };
                let r = read(&reference_path);
                assert_eq!(r["p"], *p);
                assert_eq!(r["seed"], seed);
                assert_eq!(r["arm"], *arm);
                let idx = (pi * 3 + seed as usize) * 9 + ai;
                let candidate = if ai == 9 {
                    double_candidates.join(&stem)
                } else {
                    candidates.join(format!("shard{idx}"))
                };
                let audit = if ai == 9 {
                    double_audits.join(format!("double-{stem}/audit.json"))
                } else {
                    audits.join(format!("shard{idx}/audit.json"))
                };
                let a = read(&audit);
                let s = read(candidate.join("summary.json"));
                assert_eq!(a["status"], "pass", "audit has not passed");
                let mut tuples = BTreeSet::new();
                let mut rows = BTreeSet::new();
                let mut matrix = Matrix::new(case.n, case.columns(ai));
                let mut total = 0usize;
                let mut first = None;
                let mut all_sets = Vec::new();
                for (qi, rec) in r["trace"].as_array().unwrap().iter().enumerate() {
                    let got: BTreeSet<Terms> =
                        serde_json::from_value(rec["signed_relations"].clone()).unwrap();
                    let residual = case.residual(&case.raw["query_trace"][qi]);
                    for terms in &got {
                        case.verify_terms(terms, residual);
                        tuples.insert(terms.clone());
                        let row = case.row(terms, &case.raw["query_trace"][qi], ai);
                        if let Some(canonical) = canonical_row(row.clone(), case.n) {
                            rows.insert(canonical);
                        }
                        if matrix.rank() < matrix.cols {
                            matrix.insert(row);
                        }
                    }
                    total += got.len();
                    if first.is_none() && matrix.rank() == matrix.cols {
                        first = Some(qi);
                    }
                    assert_eq!(rec["rank_after"], matrix.rank());
                    if [0, 7, 8, 9].contains(&ai) {
                        rank_csv.push_str(&format!(
                            "{p},{seed},{arm},{},{},{}\n",
                            qi + 1,
                            matrix.rank(),
                            matrix.cols
                        ));
                    }
                    all_sets.push(got);
                }
                assert_eq!(r["signed_relations"], total);
                assert_eq!(r["unique_signed_tuples_across_stream"], tuples.len());
                assert_eq!(r["rank"], matrix.rank());
                assert_eq!(r["first_full_rank_query"], json!(first));
                if ai == 0 {
                    original_sets = Some(all_sets.clone());
                }
                if ai == 1 {
                    assert_eq!(
                        original_sets.as_ref().unwrap(),
                        &all_sets,
                        "psi control changed unique relations"
                    );
                }
                let full = json!({"p":p,"seed":seed,"arm":arm,"queries":r["queries"],"physical_peels":r["physical_peels"],"usable_physical_factors":r["usable_physical_factors"],"signed_relations":total,"unique_signed_tuples":tuples.len(),"unique_projective_augmented_rows":rows.len(),"rank":matrix.rank(),"columns":matrix.cols,"first_full_rank_query":first,"finite_peels_through_rank":r["finite_peels_through_rank"],"finite_signed_peels":r["finite_signed_peels"],"censored":first.is_none()});
                full_rows.push(full.clone());
                let row =
                    json!({"p":p,"seed":seed,"arm":arm,"full_stream":full,"candidate":s,"audit":a});
                all.push(row);
                // Cold artifacts may still be in flight; never convert missing rows to zeros.
                let mut timings = Vec::new();
                let mut setup = Vec::new();
                let mut online = Vec::new();
                let mut peak_rss = Vec::new();
                let mut paired_aa = Vec::new();
                let mut admitted = true;
                let mut first_deterministic = None;
                let mut run_details = Vec::new();
                for round in 0..5 {
                    let parent = cold.join(format!("round{round}")).join(&stem);
                    let path = parent.join(arm).join("cold.json");
                    if !path.exists() {
                        admitted = false;
                        run_details.push(json!({"round":round,"status":"missing"}));
                        continue;
                    }
                    let v = read(&path);
                    let d = &v["deterministic"];
                    for field in [
                        "p",
                        "seed",
                        "arm",
                        "queries",
                        "rank",
                        "columns",
                        "signed_relations",
                        "systems",
                        "solved_logs",
                        "undetermined",
                    ] {
                        assert_eq!(d[field], s[field], "cold/collector mismatch at {field}");
                    }
                    if let Some(prior) = &first_deterministic {
                        assert_eq!(prior, d);
                    } else {
                        first_deterministic = Some(d.clone());
                    }
                    for (qi, rec) in d["trace"].as_array().unwrap().iter().enumerate() {
                        let actual: BTreeSet<Terms> =
                            serde_json::from_value(rec["signed_relations"].clone()).unwrap();
                        assert_eq!(actual, all_sets[qi]);
                        assert_eq!(rec["rank_after"], r["trace"][qi]["rank_after"]);
                    }
                    let (ok, iso) = isolation(&parent.join(format!("{arm}.isolation.json")));
                    admitted &= ok;
                    let origin = std::fs::read_to_string(parent.join("job-id.txt")).ok();
                    admitted &= origin.as_ref().is_some_and(|x| !x.trim().is_empty());
                    let exit = std::fs::read_to_string(parent.join(format!("{arm}.exit"))).ok();
                    admitted &= exit.as_ref().is_some_and(|x| x.trim() == "0");
                    let t = &v["timing"];
                    assert_eq!(
                        number(t, "cold_ns"),
                        number(t, "setup_ns") + number(t, "online_ns")
                    );
                    let sum: [&str; 5] = [
                        "point_queries_ns",
                        "decomposition_ns",
                        "matrix_ns",
                        "recovery_ns",
                        "verification_ns",
                    ];
                    assert_eq!(
                        sum.iter().map(|k| number(t, k)).sum::<u64>(),
                        number(t, "online_ns")
                    );
                    timings.push(number(t, "cold_ns"));
                    setup.push(number(t, "setup_ns"));
                    online.push(number(t, "online_ns"));
                    if let Some(v) = iso["run"]["max_rss_kib"].as_u64() {
                        peak_rss.push(v);
                    }
                    if ai == 0 {
                        let aa_parent = cold.join(format!("round{round}-aa")).join(&stem);
                        if aa_parent.join("original/cold.json").exists() {
                            let aa = read(aa_parent.join("original/cold.json"));
                            assert_eq!(aa["deterministic"], *d);
                            let (aa_ok, _) = isolation(&aa_parent.join("original.isolation.json"));
                            let aa_origin =
                                std::fs::read_to_string(aa_parent.join("job-id.txt")).ok();
                            let aa_exit =
                                std::fs::read_to_string(aa_parent.join("original.exit")).ok();
                            admitted &= aa_ok
                                && aa_origin == origin
                                && aa_exit.as_ref().is_some_and(|x| x.trim() == "0");
                            paired_aa.push(json!({"round":round,"origin_job":origin.as_ref().map(|s|s.trim()),"first_ns":t["cold_ns"],"repeat_ns":aa["timing"]["cold_ns"],"repeat_over_first":number(&aa["timing"],"cold_ns") as f64/number(t,"cold_ns") as f64}));
                        } else {
                            admitted = false;
                        }
                    }
                    run_details.push(json!({"round":round,"origin_job":origin.as_ref().map(|s|s.trim()),"timing":t,"memory":v["memory"],"isolation_admitted":ok}));
                }
                let mut deterministic_replays = 0;
                if let Some(expected) = &first_deterministic {
                    for pass in ["warmup", "first", "aa"] {
                        let path = cold.join(pass).join(&stem).join(arm).join("cold.json");
                        if path.exists() {
                            assert_eq!(read(path)["deterministic"], *expected);
                            let exit = cold.join(pass).join(&stem).join(format!("{arm}.exit"));
                            if std::fs::read_to_string(exit).is_ok_and(|x| x.trim() == "0") {
                                deterministic_replays += 1;
                            }
                        }
                    }
                }
                let complete = timings.len() == 5 && deterministic_replays == 3;
                let aa_summary = if paired_aa.len() == 5 {
                    float_summary(
                        paired_aa
                            .iter()
                            .map(|v| v["repeat_over_first"].as_f64().unwrap())
                            .collect(),
                    )
                } else {
                    Value::Null
                };
                cold_rows.push(json!({"p":p,"seed":seed,"arm":arm,"completed_rounds":timings.len(),"isolation_admitted":admitted&&complete,"successful_solution":s["status"]=="pass","censored":s["censored"],"median_cold_ns":if complete {json!(median(timings.clone()))} else {Value::Null},"median_setup_ns":if complete {json!(median(setup))} else {Value::Null},"median_online_ns":if complete {json!(median(online))} else {Value::Null},"min_cold_ns":timings.iter().min(),"max_cold_ns":timings.iter().max(),"max_process_rss_kb":peak_rss.iter().max(),"aa_pairs":paired_aa,"aa_repeat_over_first":aa_summary,"rounds":run_details}));
            }
        }
    }
    let mut arm_totals = Vec::new();
    let mut candidate_totals = Vec::new();
    for arm in ARMS {
        let rows: Vec<_> = full_rows.iter().filter(|v| v["arm"] == arm).collect();
        let sum = |field: &str| rows.iter().map(|v| number(v, field)).sum::<u64>();
        let cases = rows.len();
        arm_totals.push(json!({"arm":arm,"cases":cases,"full_rank_cases":rows.iter().filter(|v|v["censored"]==false).count(),"full_stream_queries":sum("queries"),"signed_relations":sum("signed_relations"),"unique_signed_tuples":sum("unique_signed_tuples"),"unique_projective_augmented_rows":sum("unique_projective_augmented_rows"),"finite_signed_peels":sum("finite_signed_peels"),"relations_per_finite_peel":sum("signed_relations") as f64/sum("finite_signed_peels") as f64}));
        let prefix: Vec<_> = all.iter().filter(|v| v["arm"] == arm).collect();
        let sum = |field: &str| {
            prefix
                .iter()
                .map(|v| number(&v["candidate"], field))
                .sum::<u64>()
        };
        let projective: u64 = prefix
            .iter()
            .map(|v| number(&v["audit"], "unique_projective_augmented_rows"))
            .sum();
        candidate_totals.push(json!({"arm":arm,"cases":cases,"full_rank_cases":prefix.iter().filter(|v|v["candidate"]["status"]=="pass").count(),"queries_to_rank_or_cap":sum("queries"),"systems_to_rank_or_cap":sum("systems"),"signed_relations_to_rank_or_cap":sum("signed_relations"),"unique_projective_rows_to_rank_or_cap":projective,"f4_row_reduction_fp_muls":sum("f4_row_reduction_fp_muls"),"accounting":"row reduction is a partial counter; complete cold timing is separate"}));
    }
    let mut comparisons = Vec::new();
    for p in PRIMES {
        for seed in 0..3 {
            let row = |arm: &str| {
                all.iter()
                    .find(|v| v["p"] == p && v["seed"] == seed && v["arm"] == arm)
                    .unwrap()
            };
            let phi = row("phi-l0-active");
            let double = row("double-l0-active");
            let pure = row("l0-peel");
            let f = |r: &Value, field: &str| number(&r["full_stream"], field) as f64;
            let c = |r: &Value, field: &str| number(&r["candidate"], field) as f64;
            comparisons.push(json!({"p":p,"seed":seed,"phi_over_double_unique_relations":f(phi,"unique_signed_tuples")/f(double,"unique_signed_tuples"),"phi_over_double_unique_projective_rows":f(phi,"unique_projective_augmented_rows")/f(double,"unique_projective_augmented_rows"),"phi_over_pure_unique_relations":f(phi,"unique_signed_tuples")/f(pure,"unique_signed_tuples"),"double_over_pure_unique_relations":f(double,"unique_signed_tuples")/f(pure,"unique_signed_tuples"),"double_over_phi_systems_to_rank":c(double,"systems")/c(phi,"systems"),"pure_over_phi_systems_to_rank":c(pure,"systems")/c(phi,"systems"),"phi_queries":phi["candidate"]["queries"],"double_queries":double["candidate"]["queries"],"pure_queries":pure["candidate"]["queries"],"columns":phi["candidate"]["columns"]}));
        }
    }
    let costs = paired_costs(&cold_rows);
    let total = |arm: &str| arm_totals.iter().find(|r| r["arm"] == arm).unwrap();
    let prefix = |arm: &str| candidate_totals.iter().find(|r| r["arm"] == arm).unwrap();
    let phi = total("phi-l0-active");
    let double = total("double-l0-active");
    let physical_ratio =
        number(phi, "unique_signed_tuples") as f64 / number(double, "unique_signed_tuples") as f64;
    let row_ratio = number(phi, "unique_projective_augmented_rows") as f64
        / number(double, "unique_projective_augmented_rows") as f64;
    let systems_ratio = number(prefix("phi-l0-active"), "systems_to_rank_or_cap") as f64
        / number(prefix("double-l0-active"), "systems_to_rank_or_cap") as f64;
    let panel_diagnostics = json!({"phi_over_double_unique_signed_tuples":physical_ratio,"phi_minus_double_unique_signed_tuples":number(phi,"unique_signed_tuples") as i64-number(double,"unique_signed_tuples") as i64,"phi_signed_tuple_change_percent":100.0*(physical_ratio-1.0),"phi_over_double_unique_projective_rows":row_ratio,"phi_minus_double_unique_projective_rows":number(phi,"unique_projective_augmented_rows") as i64-number(double,"unique_projective_augmented_rows") as i64,"phi_projective_row_change_percent":100.0*(row_ratio-1.0),"phi_over_double_polynomial_systems_to_rank":systems_ratio,"phi_system_count_change_percent":100.0*(systems_ratio-1.0),"scope":"aggregate diagnostics on separate known-answer targets; no amortized DLP cost or inference of a population mean"});
    write(
        &out.join("summary.json"),
        &json!({"schema":"quartic-twist-image-summary/v2","status":"reference and candidate audits checked; cold admission is per row and pair","input_sha256":INPUT_SHA,"arm_totals":arm_totals,"candidate_totals":candidate_totals,"panel_diagnostics":panel_diagnostics,"phi_vs_controls":comparisons,"full_stream_rows":full_rows,"cold_rows":cold_rows,"paired_costs":costs}),
    );
    write(&out.join("joined.json"), &json!({"rows":all}));
    let mut csv=String::from("p,seed,arm,queries,peels,relations,unique_tuples,unique_projective_rows,rank,columns,first_full_rank_query,finite_peels_through_rank\n");
    for r in &full_rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}\n",
            r["p"],
            r["seed"],
            r["arm"].as_str().unwrap(),
            r["queries"],
            r["physical_peels"],
            r["signed_relations"],
            r["unique_signed_tuples"],
            r["unique_projective_augmented_rows"],
            r["rank"],
            r["columns"],
            r["first_full_rank_query"],
            r["finite_peels_through_rank"]
        ));
    }
    std::fs::write(out.join("full-stream.csv"), csv).unwrap();
    std::fs::write(out.join("rank-traces.csv"), rank_csv).unwrap();
    let mut csv=String::from("p,seed,arm,admitted,successful_solution,censored,median_cold_seconds,median_setup_seconds,median_online_seconds,max_process_rss_kib\n");
    for r in &cold_rows {
        let seconds = |key: &str| {
            r[key]
                .as_u64()
                .map(|v| format!("{:.9}", v as f64 / 1e9))
                .unwrap_or_default()
        };
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            r["p"],
            r["seed"],
            r["arm"].as_str().unwrap(),
            r["isolation_admitted"],
            r["successful_solution"],
            r["censored"],
            seconds("median_cold_ns"),
            seconds("median_setup_ns"),
            seconds("median_online_ns"),
            r["max_process_rss_kb"]
        ));
    }
    std::fs::write(out.join("cold-costs.csv"), csv).unwrap();
    let mut csv=String::from("p,seed,reference_arm,candidate_arm,admitted,median_reference_over_candidate,min_reference_over_candidate,max_reference_over_candidate\n");
    for r in &costs {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            r["p"],
            r["seed"],
            r["reference_arm"].as_str().unwrap(),
            r["candidate_arm"].as_str().unwrap(),
            r["admitted"],
            r["reference_over_candidate_cold"]["median"],
            r["reference_over_candidate_cold"]["min"],
            r["reference_over_candidate_cold"]["max"]
        ));
    }
    std::fs::write(out.join("paired-cold-costs.csv"), csv).unwrap();
    for v in &arm_totals {
        println!("{}", serde_json::to_string(v).unwrap());
    }
}
