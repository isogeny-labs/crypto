//! Frozen descriptive native analysis. No fitted exponent or significance test.
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
type Key = (String, String, u64, String, String);
fn q(v: &mut [u128], n: usize, d: usize) -> Option<String> {
    v.sort_unstable();
    if v.is_empty() {
        None
    } else {
        Some(v[(v.len() - 1) * n / d].to_string())
    }
}
fn fq(v: &mut [f64]) -> Option<f64> {
    v.sort_by(f64::total_cmp);
    if v.is_empty() {
        None
    } else {
        Some(v[(v.len() - 1) / 2])
    }
}
fn ns(v: &Value) -> Option<u128> {
    v["elapsed_ns"].as_str().map(|s| s.parse().unwrap())
}
fn muls(v: &Value) -> u128 {
    v["work"]["muls"].as_u64().unwrap() as u128
}
fn main() {
    let files: Vec<_> = std::env::args().skip(1).collect();
    assert!(
        !files.is_empty(),
        "usage: vieta-coset-analyze records.jsonl [...]"
    );
    let mut groups: BTreeMap<(Key, String), Vec<Value>> = BTreeMap::new();
    let mut queries: BTreeMap<(Key, String, u64), BTreeMap<String, Value>> = BTreeMap::new();
    let mut setup: BTreeMap<(String, String, u64, String), Value> = BTreeMap::new();
    let mut flow: BTreeMap<String, (u64, u64, u64, u64)> = BTreeMap::new();
    for path in files {
        for line in BufReader::new(File::open(path).unwrap()).lines() {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            let s = &row["scope"];
            if !s.is_object() {
                continue;
            }
            let panel = s["panel"].as_str().unwrap_or("legacy").to_string();
            let curve = s["curve"].as_str().unwrap().to_string();
            let m = s["m"].as_u64().unwrap();
            let kind = row["kind"].as_str().unwrap().to_string();
            if !["query", "three-query"].contains(&kind.as_str()) {
                setup.insert((panel.clone(), curve.clone(), m, kind.clone()), row.clone());
            }
            if ["query", "three-query"].contains(&kind.as_str()) {
                let f = flow.entry(panel.clone()).or_default();
                f.0 += 1;
                if row["warmup"] == true {
                    f.1 += 1;
                }
                if row["status"] == "ok" {
                    f.2 += 1;
                } else {
                    f.3 += 1;
                }
            }
            if row["warmup"] == true {
                continue;
            }
            let mut population = row["population"].as_str().unwrap_or("setup").to_string();
            if ["query", "three-query"].contains(&kind.as_str()) && row["target"].is_null() {
                population.push_str("/identity");
            }
            let arm = row["arm"].as_str().unwrap_or("setup").to_string();
            let key = (panel, curve, m, kind, population);
            if ["query", "three-query"].contains(&key.3.as_str()) {
                let k = (
                    key.clone(),
                    row["target"].to_string(),
                    row["round"].as_u64().unwrap(),
                );
                assert!(
                    queries
                        .entry(k)
                        .or_default()
                        .insert(arm.clone(), row.clone())
                        .is_none(),
                    "duplicate query identity"
                );
            }
            groups.entry((key, arm)).or_default().push(row);
        }
    }
    let mut summaries = vec![];
    for ((key, arm), group) in &groups {
        let success: Vec<_> = group
            .iter()
            .filter(|r| r["status"].as_str().is_none_or(|s| s == "ok"))
            .collect();
        let mut times: Vec<_> = success.iter().filter_map(|r| ns(r)).collect();
        let timed = times.len();
        let low = q(&mut times, 1, 4);
        let med = q(&mut times, 1, 2);
        let high = q(&mut times, 3, 4);
        let mut mm: Vec<_> = success.iter().map(|r| muls(r)).collect();
        summaries.push(json!({"panel":key.0,"curve":key.1,"m":key.2,"kind":key.3,"population":key.4,"arm":arm,"attempted":group.len(),"succeeded":success.len(),"failed":group.len()-success.len(),"timed_rows":timed,"elapsed_ns_lower_quartile":low,"elapsed_ns_median":med,"elapsed_ns_upper_quartile":high,"muls_median":q(&mut mm,1,2),"relation_counts":success.iter().filter_map(|r|r["relations"].as_u64()).collect::<Vec<_>>()}));
    }
    let mut matched: BTreeMap<(Key, String), Vec<Value>> = BTreeMap::new();
    for ((key, target, round), arms) in queries {
        let three = key.3 == "three-query";
        let reference = if three { "mitm" } else { "batched" };
        let aa = if three { "mitm-aa" } else { "batched-aa" };
        let Some(r) = arms.get(reference).filter(|r| r["status"] == "ok") else {
            continue;
        };
        let sk = |kind: &str| setup.get(&(key.0.clone(), key.1.clone(), key.2, kind.into()));
        let d = sk("descriptor").expect("descriptor");
        let b = sk("setup").expect("base setup");
        let pair = sk("pair-setup");
        for (arm, c) in &arms {
            if arm == reference || c["status"] != "ok" {
                continue;
            }
            assert_eq!(
                c["result_sha256"], r["result_sha256"],
                "mismatched complete output"
            );
            let setup_c = if three || arm != "vieta" {
                muls(d) + muls(b)
            } else {
                muls(d)
            };
            let setup_r = muls(d)
                + muls(b)
                + if three {
                    muls(pair.expect("pair setup"))
                } else {
                    0
                };
            let cold_c = setup_c + muls(c);
            let cold_r = setup_r + muls(r);
            let is_aa = arm == aa;
            let (cold_c, cold_r) = if is_aa {
                (setup_r + muls(c), cold_r)
            } else {
                (cold_c, cold_r)
            };
            let time_ratio = ns(c)
                .zip(ns(r))
                .filter(|(_, r)| *r > 0)
                .map(|(c, r)| c as f64 / r as f64);
            let cst = ns(d)
                .zip(ns(b))
                .map(|(d, b)| if three || arm != "vieta" { d + b } else { d });
            let rst = ns(d)
                .zip(ns(b))
                .map(|(d, b)| d + b + if three { ns(pair.unwrap()).unwrap() } else { 0 });
            let cold_time = cst
                .zip(rst)
                .zip(ns(c).zip(ns(r)))
                .map(|((cs, rs), (c, r))| {
                    ((if is_aa { rs } else { cs }) + c) as f64 / (rs + r) as f64
                });
            let row = json!({"target":target,"round":round,"warm_muls_candidate":muls(c).to_string(),"warm_muls_reference":muls(r).to_string(),"cold_muls_candidate":cold_c.to_string(),"cold_muls_reference":cold_r.to_string(),"warm_muls_ratio":if muls(r)>0{Some(muls(c) as f64/muls(r) as f64)}else{None},"cold_muls_ratio":if cold_r>0{Some(cold_c as f64/cold_r as f64)}else{None},"warm_time_ratio":time_ratio,"cold_time_ratio":cold_time});
            matched
                .entry((key.clone(), arm.clone()))
                .or_default()
                .push(row);
        }
    }
    let mut comparisons = vec![];
    for ((key, arm), rows) in matched {
        let med = |field: &str| {
            let mut v: Vec<_> = rows.iter().filter_map(|r| r[field].as_f64()).collect();
            fq(&mut v)
        };
        let vals: Vec<_> = rows
            .iter()
            .filter_map(|r| r["warm_time_ratio"].as_f64())
            .collect();
        let min = vals.iter().copied().reduce(f64::min);
        let max = vals.iter().copied().reduce(f64::max);
        comparisons.push(json!({"panel":key.0,"curve":key.1,"m":key.2,"kind":key.3,"population":key.4,"arm":arm,"reference":if key.3=="three-query"{"mitm"}else{"batched"},"matched":rows.len(),"warm_muls_ratio_median":med("warm_muls_ratio"),"cold_muls_ratio_median":med("cold_muls_ratio"),"warm_time_ratio_median":med("warm_time_ratio"),"cold_time_ratio_median":med("cold_time_ratio"),"warm_time_ratio_min":min,"warm_time_ratio_max":max,"pairs":rows}));
    }
    let flows:Vec<_>=flow.into_iter().map(|(panel,(attempted,warmup,ok,failed))|json!({"panel":panel,"attempted":attempted,"warmup":warmup,"successful":ok,"failed":failed})).collect();
    println!(
        "{}",
        json!({"schema":"vieta-coset-analysis/v2","quantile":"sorted floor((n-1)*q), descriptive only","groups":summaries,"comparisons":comparisons,"flow":flows,"standing":"unverified; counts primary; local clocks diagnostic with missing PSI, no L2 certification"})
    );
}
