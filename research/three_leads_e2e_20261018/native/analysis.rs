use super::*;
use std::collections::BTreeMap;
fn mean(xs: &[f64]) -> f64 {
    xs.iter().sum::<f64>() / xs.len() as f64
}
fn interval(xs: &[f64]) -> (f64, f64) {
    let mut rng = Rng(2026101823);
    let mut samples = Vec::new();
    for _ in 0..10000 {
        let values: Vec<_> = (0..xs.len())
            .map(|_| xs[(rng.next() % xs.len() as u64) as usize])
            .collect();
        samples.push(mean(&values));
    }
    samples.sort_by(f64::total_cmp);
    (samples[250], samples[9749])
}
pub fn coverage(input: &str, out: &str) {
    let rows: Vec<Value> = fs::read_to_string(input)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for row in &rows {
        let m = &row["meta"];
        let family = m["family"].as_str().unwrap();
        let key = match family {
            family if family.starts_with("binary") => format!("{family}-q{}", m["q"]),
            family if family.starts_with("odd") => format!("{family}-p{}-n{}", m["p"], m["n"]),
            _ => format!("{family}-d{}-lo{}", m["depth"], m["lo"]),
        };
        groups.entry(key).or_default().push(row);
    }
    let mut summaries = vec![];
    let mut csv = String::from(
        "cell,curves,structured,random,delta,standard_error,z,ci_lower,ci_upper,equivalence_005\n",
    );
    for (key, rows) in groups {
        let structured: Vec<_> = rows
            .iter()
            .map(|v| v["reachable"].as_f64().unwrap() / v["order"].as_f64().unwrap())
            .collect();
        let random: Vec<_> = rows
            .iter()
            .map(|v| {
                let controls = v["random_reachable"].as_array().unwrap();
                controls.iter().map(|x| x.as_u64().unwrap()).sum::<u64>() as f64
                    / controls.len() as f64
                    / v["order"].as_f64().unwrap()
            })
            .collect();
        let delta: Vec<_> = structured.iter().zip(&random).map(|(a, b)| a - b).collect();
        let m = mean(&delta);
        let se = if delta.len() > 1 {
            (delta.iter().map(|x| (x - m).powi(2)).sum::<f64>()
                / (delta.len() * (delta.len() - 1)) as f64)
                .sqrt()
        } else {
            0.0
        };
        let z = if se > 0.0 { Some(m / se) } else { None };
        let (lo, hi) = interval(&delta);
        let equivalent = lo >= -0.05 && hi <= 0.05;
        let v = json!({"cell":key,"curves":rows.len(),"structured":mean(&structured),"random":mean(&random),"delta":m,"standard_error":se,"z":z,"bootstrap_95":[lo,hi],"inside_absolute_005":equivalent});
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            key,
            rows.len(),
            mean(&structured),
            mean(&random),
            m,
            se,
            z.map(|x| x.to_string()).unwrap_or_default(),
            lo,
            hi,
            equivalent
        ));
        summaries.push(v);
    }
    fs::create_dir_all(out).unwrap();
    fs::write(Path::new(out).join("coverage-cells.csv"), csv).unwrap();
    write(
        Path::new(out).join("coverage-summary.json"),
        &json!(summaries),
    );
}
fn files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            files(&p, out)
        } else if p.file_name().is_some_and(|s| s == "records.jsonl") {
            out.push(p);
        }
    }
}
fn group(v: &Value, path: &Path) -> String {
    let c = &v["workload"]["curve_spec"];
    let family = c["family"].as_str().unwrap();
    let name = path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let suffix = if name.ends_with("-v2") || name.ends_with("-filtered") {
        "-v2"
    } else {
        "-v1"
    };
    if family == "scholten" {
        let dir = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        return dir.to_owned();
    }
    if family == "subfield" {
        return if c["p"] == 2 {
            format!(
                "binary-q{}{suffix}",
                1u64 << c["parameter"].as_u64().unwrap()
            )
        } else {
            format!("odd-p{}{suffix}", c["p"])
        };
    }
    let p = c["p"].as_u64().unwrap();
    let lo = if p < 8192 {
        4096
    } else if p < 16384 {
        8192
    } else {
        16384
    };
    format!("{family}-d{}-lo{lo}{suffix}", c["parameter"])
}
fn price(v: &Value, s: f64, i: f64, n: f64) -> f64 {
    let c = &v["counters"];
    let get = |name: &str| c[name].as_f64().unwrap();
    let arithmetic =
        get("field_multiplications") + s * get("field_squarings") + i * get("field_inversions");
    let unit = get("source_add_multiplications")
        + s * get("source_add_squarings")
        + i * get("source_add_inversions");
    let native = c
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("_uncharged"))
        .map(|(_, v)| v.as_f64().unwrap())
        .sum::<f64>();
    arithmetic / unit + n * native
}
#[allow(clippy::type_complexity)]
pub fn sessions(input: &str, out: &str) {
    let out = Path::new(out);
    fs::create_dir_all(out).unwrap();
    let mut paths = vec![];
    files(Path::new(input), &mut paths);
    paths.sort();
    let mut records = vec![];
    for path in paths {
        for line in fs::read_to_string(&path).unwrap().lines() {
            let v: Value = serde_json::from_str(line).unwrap();
            if v["warmup"] == true {
                continue;
            }
            records.push((path.clone(), v));
        }
    }
    let mut statuses: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    let mut pairs: BTreeMap<(String, String), BTreeMap<String, &Value>> = BTreeMap::new();
    let mut csv=String::from("group,workload,curve,p,r,h,arm,status,M,S,I,ec_adds,jac_adds,trials,first_cycle,duplicates,root_calls,oracle_M,oracle_S,oracle_I\n");
    for (path, v) in &records {
        let g = group(v, path);
        let status = v["outcome"]["status"].as_str().unwrap();
        *statuses
            .entry(g.clone())
            .or_default()
            .entry(status.into())
            .or_default() += 1;
        let w = v["workload"]["workload_id"].as_str().unwrap();
        let a = v["arm"].as_str().unwrap();
        pairs
            .entry((g.clone(), w.into()))
            .or_default()
            .insert(a.into(), v);
        let c = &v["counters"];
        let get = |name: &str| c[name].as_u64().unwrap_or(0);
        let spec = &v["workload"]["curve_spec"];
        csv.push_str(&format!(
            "{g},{w},{},{},{},{},{a},{status},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            v["workload"]["curve"]["slug"].as_str().unwrap(),
            spec["p"],
            spec["r"],
            spec["group_order"].as_u64().unwrap() / spec["r"].as_u64().unwrap(),
            get("field_multiplications"),
            get("field_squarings"),
            get("field_inversions"),
            get("source_ec_additions"),
            get("jacobian_additions"),
            get("processed_divisors")
                .max(get("walk_steps"))
                .max(get("decomposition_calls")),
            get("first_graph_cycle_trial"),
            get("duplicate_relation_divisors"),
            get("root_calls"),
            get("oracle_muls"),
            get("oracle_squares"),
            get("oracle_inversions")
        ));
    }
    fs::write(out.join("counts.csv"), csv).unwrap();
    write(out.join("statuses.json"), &json!(statuses));
    let mut aggregates: BTreeMap<(String, String, String), Vec<(&Value, &Value)>> = BTreeMap::new();
    for ((g, _), arms) in &pairs {
        for candidate in [
            "genus_ic",
            "jac_rho",
            "structured_ic",
            "random_ic",
            "filtered_ic",
        ] {
            if let Some(&c) = arms.get(candidate) {
                for reference in ["rho_neg", "rho_plain", "random_ic", "structured_ic"] {
                    if reference == candidate {
                        continue;
                    }
                    if let Some(&r) = arms.get(reference) {
                        if c["outcome"]["status"] == "verified"
                            && r["outcome"]["status"] == "verified"
                        {
                            aggregates
                                .entry((g.clone(), candidate.into(), reference.into()))
                                .or_default()
                                .push((c, r));
                            let curve = c["workload"]["curve"]["slug"].as_str().unwrap();
                            aggregates
                                .entry((
                                    format!("{g}::{curve}"),
                                    candidate.into(),
                                    reference.into(),
                                ))
                                .or_default()
                                .push((c, r));
                        }
                    }
                }
            }
        }
    }
    let mut prices=String::from("group,candidate,reference,matched_targets,S_per_M,I_per_M,native_GAE_per_operation,candidate_GAE,reference_GAE,ratio\n");
    let mut summary = vec![];
    for ((g, c, r), pairs) in aggregates {
        let mut low = f64::INFINITY;
        let mut high = 0f64;
        let mut arithlow = f64::INFINITY;
        let mut arithhigh = 0f64;
        let mut wins = 0;
        let mut cells = vec![];
        for i in [1., 10., 100., 1000.] {
            for s in [0., 0.5, 1.] {
                for n in [0., 0.1, 1.] {
                    let costc = pairs.iter().map(|(c, _)| price(c, s, i, n)).sum::<f64>();
                    let costr = pairs.iter().map(|(_, r)| price(r, s, i, n)).sum::<f64>();
                    let ratio = costc / costr;
                    low = low.min(ratio);
                    high = high.max(ratio);
                    if n == 0. {
                        arithlow = arithlow.min(ratio);
                        arithhigh = arithhigh.max(ratio);
                        if ratio > 1. {
                            wins += 1;
                        }
                    }
                    prices.push_str(&format!(
                        "{g},{c},{r},{},{s},{i},{n},{costc},{costr},{ratio}\n",
                        pairs.len()
                    ));
                    cells.push(json!({"S":s,"I":i,"native":n,"ratio":ratio}));
                }
            }
        }
        let default_ratio = pairs
            .iter()
            .map(|(c, _)| price(c, 0.75, 10., 0.))
            .sum::<f64>()
            / pairs
                .iter()
                .map(|(_, r)| price(r, 0.75, 10., 0.))
                .sum::<f64>();
        let operation = |name: &str, candidate: bool| {
            pairs
                .iter()
                .map(|(c, r)| {
                    if candidate {
                        c["counters"][name].as_f64().unwrap_or(0.)
                    } else {
                        r["counters"][name].as_f64().unwrap_or(0.)
                    }
                })
                .sum::<f64>()
                / pairs.len() as f64
        };
        let trials_per_p = mean(
            &pairs
                .iter()
                .map(|(c, _)| {
                    c["counters"]["processed_divisors"].as_f64().unwrap_or(0.)
                        / c["workload"]["curve_spec"]["p"].as_f64().unwrap()
                })
                .collect::<Vec<_>>(),
        );
        let rho_steps_per_p = mean(
            &pairs
                .iter()
                .map(|(_, r)| {
                    r["counters"]["walk_steps"].as_f64().unwrap_or(0.)
                        / r["workload"]["curve_spec"]["p"].as_f64().unwrap()
                })
                .collect::<Vec<_>>(),
        );
        let norm = |key: &str, candidate: bool, sqrt_r: bool| {
            mean(
                &pairs
                    .iter()
                    .map(|(c, r)| {
                        let v = if candidate { *c } else { *r };
                        let scale = if sqrt_r {
                            v["workload"]["curve_spec"]["r"].as_f64().unwrap().sqrt()
                        } else {
                            v["workload"]["curve_spec"]["p"].as_f64().unwrap()
                        };
                        v["counters"][key].as_f64().unwrap_or(0.) / scale
                    })
                    .collect::<Vec<_>>(),
            )
        };
        summary.push(json!({"group":g,"candidate":c,"reference":r,"matched_targets":pairs.len(),"arithmetic_ratio_range":[arithlow,arithhigh],"native_band_ratio_range":[low,high],"rho_wins_of_12":wins,"declared_default_ratio":default_ratio,"candidate_trials_per_p":trials_per_p,"rho_steps_per_p":rho_steps_per_p,"rho_steps_per_sqrt_r":norm("walk_steps",false,true),"candidate_jacobian_operations_per_p":norm("jacobian_additions",true,false),"rho_cold_ec_operations_per_p":norm("source_ec_additions",false,false),"mean_jacobian_operations":operation("jacobian_additions",true),"mean_source_operations":operation("source_ec_additions",false),"mean_first_cycle_trial":operation("first_graph_cycle_trial",true),"mean_processed_divisors":operation("processed_divisors",true),"mean_duplicate_relations":operation("duplicate_relation_divisors",true),"mean_root_calls":operation("root_calls",true),"mean_oracle_M":operation("oracle_muls",true),"mean_decomposition_calls":operation("decomposition_calls",true)}));
    }
    fs::write(out.join("cost-cells.csv"), prices).unwrap();
    report_observations(&records, out);
    mechanism_observations(&records, out);
    trace_filter_observations(&records, out);
    hardware_observations(&records, out);
    write(out.join("cost-summary.json"), &json!(summary));
    println!("{} records; {} target groups", records.len(), pairs.len());
}

fn report_observations(records: &[(std::path::PathBuf, Value)], out: &Path) {
    let mut sessions: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    let mut primary = BTreeMap::new();
    let mut events: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for (path, v) in records {
        let name = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        sessions.entry(name.clone()).or_default().push(v);
        if v["method"]["id"] == "ic.three_leads_genus2_v2"
            && !name.contains("independent")
            && !name.contains("unfiltered")
            && v["arm"] == "genus_ic"
        {
            primary.insert(v["workload"]["workload_id"].as_str().unwrap().to_owned(), v);
            events.entry(name).or_default().push(v);
        }
    }
    let mut per_session = vec![];
    for (name, rows) in sessions {
        let mut arms: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
        for v in &rows {
            arms.entry(v["arm"].as_str().unwrap().to_owned())
                .or_default()
                .push(v);
        }
        let arms:Vec<_>=arms.into_iter().map(|(arm,rows)|{let values:Vec<_>=rows.iter().map(|v|price(v,0.75,10.,0.)).collect();let verified=rows.iter().filter(|v|v["outcome"]["status"]=="verified").count();json!({"arm":arm,"attempts":rows.len(),"verified":verified,"other":rows.len()-verified,"mean_GAE":mean(&values),"minimum_GAE":values.iter().copied().min_by(f64::total_cmp).unwrap(),"maximum_GAE":values.iter().copied().max_by(f64::total_cmp).unwrap()})}).collect();
        let v = rows[0];
        per_session.push(json!({"name":name,"session_id":v["session_id"],"records":rows.len(),"arms":arms,"field_model":"S/M=0.75; I/M=10; native work unpriced; no wall-time claim","isolation":"L0","source":format!("sessions/{name}/records.jsonl"),"target_seed":v["workload"]["target_seed"],"algorithm_seed_example":v["algorithm_seed"]}));
    }
    write(out.join("session-summaries.json"), &json!(per_session));
    let events:Vec<_>=events.into_iter().map(|(g,rows)|{let get=|v:&Value,k:&str|v["counters"][k].as_u64().unwrap_or(0);let at_first=rows.iter().filter(|v|get(v,"first_graph_cycle_trial")>0&&get(v,"first_graph_cycle_trial")==get(v,"processed_divisors")).count();let later=rows.iter().filter(|v|get(v,"first_graph_cycle_trial")>0&&get(v,"first_graph_cycle_trial")<get(v,"processed_divisors")).count();json!({"group":g,"targets":rows.len(),"solves_at_first_graph_cycle":at_first,"solves_after_first_graph_cycle":later,"without_graph_cycle":rows.len()-at_first-later,"negative_cycles":rows.iter().map(|v|get(v,"negative_cycles")).sum::<u64>(),"duplicate_relation_discards":rows.iter().map(|v|get(v,"duplicate_relation_divisors")).sum::<u64>(),"walk_cycle_restarts":rows.iter().map(|v|get(v,"walk_cycle_restarts")).sum::<u64>()})}).collect();
    write(out.join("genus-cycle-events.json"), &json!(events));
    let mut controls: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for (path, v) in records {
        let name = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        if v["method"]["id"] != "ic.three_leads_genus2_v2"
            || !(name.contains("independent") || name.contains("unfiltered"))
        {
            continue;
        }
        let mode = if name.contains("independent") {
            "independent"
        } else {
            "unfiltered"
        };
        let mode = format!(
            "{}_{}",
            if name.starts_with("order-") {
                "order"
            } else {
                "field"
            },
            mode
        );
        let id = v["workload"]["workload_id"].as_str().unwrap();
        let Some(&p) = primary.get(id) else {
            continue;
        };
        let field = v["workload"]["curve_spec"]["p"].as_f64().unwrap();
        let count = |x: &Value| x["counters"]["processed_divisors"].as_f64().unwrap();
        controls.entry(mode).or_default().push(json!({"p":field,"r":v["workload"]["curve_spec"]["r"],"workload":id,"primary_trials":count(p),"control_trials":count(v),"primary_trials_per_p":count(p)/field,"control_trials_per_p":count(v)/field,"primary_GAE":price(p,0.75,10.,0.),"control_GAE":price(v,0.75,10.,0.)}));
    }
    let mut control_summary = vec![];
    for (mode, rows) in controls {
        let ratio = |indices: &[usize]| {
            indices
                .iter()
                .map(|&i| rows[i]["control_trials_per_p"].as_f64().unwrap())
                .sum::<f64>()
                / indices
                    .iter()
                    .map(|&i| rows[i]["primary_trials_per_p"].as_f64().unwrap())
                    .sum::<f64>()
        };
        let mut rng = Rng(202610181260);
        let mut boot = vec![];
        for _ in 0..10000 {
            let ids: Vec<_> = (0..rows.len())
                .map(|_| rng.next() as usize % rows.len())
                .collect();
            boot.push(ratio(&ids));
        }
        boot.sort_by(f64::total_cmp);
        let cost_ratio = rows
            .iter()
            .map(|v| v["control_GAE"].as_f64().unwrap())
            .sum::<f64>()
            / rows
                .iter()
                .map(|v| v["primary_GAE"].as_f64().unwrap())
                .sum::<f64>();
        control_summary.push(json!({"mode":mode,"curves":rows.len(),"paired_normalized_trial_ratio":ratio(&(0..rows.len()).collect::<Vec<_>>()),"paired_curve_bootstrap_95":[boot[250],boot[9749]],"declared_cost_ratio":cost_ratio,"pairs":rows}));
    }
    write(out.join("genus-controls.json"), &json!(control_summary));
}

fn mechanism_observations(records: &[(std::path::PathBuf, Value)], out: &Path) {
    let mut groups: BTreeMap<String, Vec<&Value>> = BTreeMap::new();
    for (_, v) in records {
        if v["method"]["id"] == "ic.three_leads_decomposition_v2" {
            let family = v["workload"]["curve_spec"]["family"].as_str().unwrap();
            let mode = if v["method"]["params"]["random_base"] == "1" {
                "random"
            } else {
                "structured"
            };
            groups
                .entry(format!("{family}-{mode}"))
                .or_default()
                .push(v);
        }
    }
    let rows:Vec<_>=groups.into_iter().map(|(family,rows)|json!({"family":family,"verified_returns":rows.iter().filter(|v|v["outcome"]["status"]=="verified").count(),"before_first_decomposition":rows.iter().filter(|v|v["outcome"]["status"]=="verified"&&v["counters"]["relations"]==0).count(),"empty_base_returns":rows.iter().filter(|v|v["outcome"]["status"]=="verified"&&v["counters"]["base_signed_points"]==0).count(),"full_elimination_rank_returns":rows.iter().filter(|v|v["outcome"]["status"]=="verified"&&v["counters"]["linear_rank"].as_u64().unwrap()==1+v["counters"]["base_columns"].as_u64().unwrap()).count(),"meaning":"Zero-decomposition returns are generic identity hits. Full rank certifies elimination; smaller rank alone does not identify the final branch."})).collect();
    write(out.join("decomposition-mechanisms.json"), &json!(rows));
}

fn trace_filter_observations(records: &[(std::path::PathBuf, Value)], out: &Path) {
    let mut baseline = BTreeMap::new();
    for (_, v) in records {
        if v["method"]["id"] == "ic.three_leads_decomposition_v2"
            && v["arm"] == "structured_ic"
            && v["method"]["params"]["oracle"] == "linearized"
        {
            baseline.insert(v["workload"]["workload_id"].as_str().unwrap(), v);
        }
    }
    let mut rows = vec![];
    for (_, v) in records {
        if v["arm"] != "filtered_ic" {
            continue;
        }
        let id = v["workload"]["workload_id"].as_str().unwrap();
        let b = baseline[id];
        assert_eq!(v["algorithm_seed"], b["algorithm_seed"]);
        assert_eq!(v["outcome"]["recovered"], b["outcome"]["recovered"]);
        for key in ["decomposition_calls", "relations", "linear_rank"] {
            assert_eq!(v["counters"][key], b["counters"][key]);
        }
        let n = v["workload"]["curve_spec"]["n"].as_u64().unwrap();
        let fsetup = v["phases"][0]["native"]["field_squarings"]
            .as_u64()
            .unwrap();
        let bsetup = b["phases"][0]["native"]["field_squarings"]
            .as_u64()
            .unwrap();
        assert_eq!(fsetup - bsetup, n * (n - 1));
        rows.push(json!({"workload":id,"curve":v["workload"]["curve"]["slug"],"q":1u64<<v["workload"]["curve_spec"]["parameter"].as_u64().unwrap(),"tested":v["counters"]["trace_filter_tests"],"skipped":v["counters"]["trace_filter_skips"],"same_algorithm_seed_and_stopping_counts":true,"mask_setup_S":fsetup-bsetup,"unfiltered_GAE":price(b,0.75,10.,0.),"filtered_GAE":price(v,0.75,10.,0.)}));
    }
    assert_eq!(rows.len(), 168);
    let tests = rows
        .iter()
        .map(|v| v["tested"].as_u64().unwrap())
        .sum::<u64>();
    let skips = rows
        .iter()
        .map(|v| v["skipped"].as_u64().unwrap())
        .sum::<u64>();
    write(
        out.join("trace-filter.json"),
        &json!({"targets":rows.len(),"tests":tests,"skips":skips,"skipped_fraction":skips as f64/tests as f64,"same_stopping_and_relation_counts":true,"per_finite_target_cost":"4 logical bit operations, zero field M/S/I after setup","mask_setup":"n*(n-1) field squares, plus explicit bit operations, cold on each solve","pairs":rows}),
    );
}

fn hardware_observations(records: &[(std::path::PathBuf, Value)], out: &Path) {
    let mut pairs: BTreeMap<(String, String), BTreeMap<String, &Value>> = BTreeMap::new();
    for (path, v) in records {
        let g = group(v, path);
        if !g.ends_with("-v2") && !g.starts_with("order-genus-") {
            continue;
        }
        pairs
            .entry((g, v["workload"]["workload_id"].as_str().unwrap().into()))
            .or_default()
            .insert(v["arm"].as_str().unwrap().into(), v);
    }
    let get = |v: &Value| -> Option<u64> {
        let h = &v["time"]["hw_solve"];
        let count = h["instructions"].as_u64()?;
        let enabled = h["instructions_time_enabled_ns"].as_u64()?;
        let running = h["instructions_time_running_ns"].as_u64()?;
        if h["error"].is_null() && count > 0 && enabled > 0 && enabled == running {
            Some(count)
        } else {
            None
        }
    };
    type InstructionPairs<'a> =
        BTreeMap<(String, String, String), Vec<(&'a Value, &'a Value, u64, u64)>>;
    let mut groups: InstructionPairs<'_> = BTreeMap::new();
    for ((g, _), arms) in pairs {
        for candidate in [
            "genus_ic",
            "jac_rho",
            "structured_ic",
            "random_ic",
            "filtered_ic",
        ] {
            if let Some(&c) = arms.get(candidate) {
                for reference in ["rho_neg", "rho_plain"] {
                    if let Some(&r) = arms.get(reference) {
                        if c["outcome"]["status"] != "verified"
                            || r["outcome"]["status"] != "verified"
                            || !c["binary_sha256"].is_string()
                            || c["binary_sha256"] != r["binary_sha256"]
                        {
                            continue;
                        }
                        if let (Some(ci), Some(ri)) = (get(c), get(r)) {
                            groups
                                .entry((g.clone(), candidate.into(), reference.into()))
                                .or_default()
                                .push((c, r, ci, ri));
                        }
                    }
                }
            }
        }
    }
    let mut output = vec![];
    for ((group, candidate, reference), pairs) in groups {
        let mut curves: BTreeMap<&str, Vec<(u64, u64)>> = BTreeMap::new();
        for (c, _, ci, ri) in &pairs {
            curves
                .entry(c["workload"]["curve"]["slug"].as_str().unwrap())
                .or_default()
                .push((*ci, *ri));
        }
        let curves: Vec<_> = curves.into_values().collect();
        let mut rng = Rng(20261019091);
        let mut boot = vec![];
        for _ in 0..4096 {
            let (mut a, mut b) = (0f64, 0f64);
            for _ in 0..curves.len() {
                let curve = &curves[rng.next() as usize % curves.len()];
                for _ in 0..curve.len() {
                    let (x, y) = curve[rng.next() as usize % curve.len()];
                    a += x as f64;
                    b += y as f64;
                }
            }
            boot.push(a / b);
        }
        boot.sort_by(f64::total_cmp);
        let numerator = pairs.iter().map(|x| x.2).sum::<u64>();
        let denominator = pairs.iter().map(|x| x.3).sum::<u64>();
        let wall = pairs
            .iter()
            .map(|x| x.0["time"]["solve_wall_ns"].as_f64().unwrap())
            .sum::<f64>()
            / pairs
                .iter()
                .map(|x| x.1["time"]["solve_wall_ns"].as_f64().unwrap())
                .sum::<f64>();
        output.push(json!({"group":group,"candidate":candidate,"reference":reference,"matched_targets":pairs.len(),"curves":curves.len(),"candidate_instructions":numerator,"reference_instructions":denominator,"instruction_ratio":numerator as f64/denominator as f64,"paired_curve_and_target_bootstrap_95":[boot[102],boot[3993]],"unqualified_wall_ratio":wall,"pmu_quality":"user-space calling-process counters; same frozen binary; positive counts; time_enabled=time_running for both arms","wall_qualified":false,"meaning":"Retired instructions of the complete instrumented solve, including native work and ledger/report overhead; wall ratios remain descriptive at L0"}));
    }
    write(out.join("retired-instruction-summary.json"), &json!(output));
}

pub fn instruction_aa(study: &str) {
    let study = Path::new(study);
    let mut rounds: BTreeMap<u64, BTreeMap<String, Value>> = BTreeMap::new();
    for line in fs::read_to_string(study.join("instruction-AA-session/records.jsonl"))
        .unwrap()
        .lines()
    {
        let v: Value = serde_json::from_str(line).unwrap();
        if v["warmup"] == true {
            continue;
        }
        assert_eq!(v["outcome"]["status"], "verified");
        rounds
            .entry(v["round"].as_u64().unwrap())
            .or_default()
            .insert(v["arm"].as_str().unwrap().into(), v);
    }
    assert_eq!(rounds.len(), 5);
    let mut rows = vec![];
    let mut ratios = vec![];
    for (round, arms) in rounds {
        let a = &arms["rho_neg"];
        let b = &arms["rho_AA"];
        assert_eq!(a["counters"], b["counters"]);
        assert_eq!(a["field_ops"], b["field_ops"]);
        assert_eq!(a["binary_sha256"], b["binary_sha256"]);
        for v in [a, b] {
            let h = &v["time"]["hw_solve"];
            assert!(h["error"].is_null());
            assert_eq!(
                h["instructions_time_enabled_ns"],
                h["instructions_time_running_ns"]
            );
        }
        let x = a["time"]["hw_solve"]["instructions"].as_u64().unwrap();
        let y = b["time"]["hw_solve"]["instructions"].as_u64().unwrap();
        let ratio = y as f64 / x as f64;
        ratios.push(ratio);
        rows.push(json!({"round":round,"reference_instructions":x,"repeat_instructions":y,"ratio":ratio,"counts_identical":true}));
    }
    ratios.sort_by(f64::total_cmp);
    write(
        study.join("results/instruction-AA.json"),
        &json!({"measured_pairs":5,"warmup_pairs":1,"all_primitive_and_native_counts_identical":true,"ratio_range":[ratios[0],ratios[4]],"median_ratio":ratios[2],"max_absolute_relative_difference":ratios.iter().map(|x|(x-1.).abs()).fold(0.,f64::max),"wall_qualified":false,"pairs":rows}),
    );
}
