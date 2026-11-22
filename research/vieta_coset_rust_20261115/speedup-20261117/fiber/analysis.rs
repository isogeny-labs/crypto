use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
fn stats(mut xs: Vec<f64>) -> Value {
    if xs.is_empty() {
        return json!({"n":0});
    }
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    let med = if n.is_multiple_of(2) {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    } else {
        xs[n / 2]
    };
    let mut state = 0x424f4f5453545241u64 ^ n as u64;
    let mut boot = Vec::with_capacity(4096);
    for _ in 0..4096 {
        let mut sample = Vec::with_capacity(n);
        for _ in 0..n {
            state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            sample.push(xs[((z ^ (z >> 31)) % n as u64) as usize]);
        }
        sample.sort_by(f64::total_cmp);
        boot.push(if n.is_multiple_of(2) {
            (sample[n / 2 - 1] + sample[n / 2]) / 2.0
        } else {
            sample[n / 2]
        });
    }
    boot.sort_by(f64::total_cmp);
    json!({"n":n,"min":xs[0],"median":med,"max":xs[n-1],"values":xs,
        "bootstrap_median95":[boot[102],boot[3993]],"bootstrap_replicates":4096,
        "uncertainty_scope":"descriptive percentile resampling conditional on these sampled targets; small-N, no p-value or population/exponent inference"})
}
pub fn summarize(paths: &[String]) {
    let mut groups: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    let (mut total, mut ok, mut warm) = (0, 0, 0);
    for path in paths {
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if !p
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("cell-")
                || p.extension().is_none_or(|e| e != "json")
            {
                continue;
            }
            let r: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
            total += 1;
            if r["status"] == "ok" {
                ok += 1;
            }
            if r["warmup"] == true {
                warm += 1;
                continue;
            }
            let c = &r["config"];
            let key = format!(
                "{}:{}:m{}:{}:{}",
                c["representation"].as_str().unwrap(),
                c["field"].as_str().unwrap(),
                c["m"],
                r["population"].as_str().unwrap(),
                if r["target"].is_null() {
                    "identity"
                } else {
                    "finite"
                }
            );
            groups.entry(key).or_default().push(r);
        }
    }
    let mut output = vec![];
    for (key, rows) in groups {
        let mut arms = BTreeMap::new();
        let names = [
            "fiber", "fiber-half", "fiber-half-aa", "fiber-half-double",
            "ntt",
            "ntt-half",
            "ntt-aa",
            "ntt-double",
            "ntt-half-aa",
            "ntt-half-double",
            "school",
            "newton",
            "half",
            "fast",
            "fast-half",
            "fast-aa",
            "fast-double",
            "lookup",
            "lookup-fast",
        ];
        for arm in names {
            let ar: Vec<_> = rows.iter().filter(|r| r["arm"] == arm).collect();
            let good: Vec<_> = ar.iter().filter(|r| r["status"] == "ok").collect();
            let mut v =
                json!({"attempted":ar.len(),"successful":good.len(),"failed":ar.len()-good.len()});
            for phase in ["field_setup_ns", "construction_ns", "query_ns", "cold_ns"] {
                v[phase] = stats(
                    good.iter()
                        .map(|r| r[phase].as_str().unwrap().parse::<f64>().unwrap())
                        .collect(),
                );
            }
            for phase in ["construction_work", "query_work", "total_work"] {
                v[format!("{phase}_muls")] = stats(
                    good.iter()
                        .map(|r| r[phase]["muls"].as_u64().unwrap() as f64)
                        .collect(),
                );
            }
            arms.insert(arm.to_string(), v);
        }
        let mut ratios = BTreeMap::new();
        for (num, den) in [
            ("newton", "fiber-half"), ("fast", "fiber-half"), ("ntt-half", "fiber-half"), ("fiber-half", "fiber-half-aa"), ("fiber-half-double", "fiber-half"), ("lookup-fast", "fiber-half"),
            ("newton", "ntt-half"),
            ("fast", "ntt-half"),
            ("fast-half", "ntt-half"),
            ("ntt-half", "ntt-half-aa"),
            ("ntt-half-double", "ntt-half"),
            ("lookup-fast", "ntt-half"),
            ("newton", "fast"),
            ("half", "fast-half"),
            ("school", "fast"),
            ("lookup-fast", "fast"),
            ("lookup", "fast"),
            ("fast", "fast-aa"),
            ("fast-double", "fast"),
        ] {
            for phase in ["query_ns", "cold_ns", "query_muls"] {
                let mut xs = vec![];
                for r in rows
                    .iter()
                    .filter(|r| r["arm"] == num && r["status"] == "ok")
                {
                    if let Some(s) = rows.iter().find(|s| {
                        s["arm"] == den
                            && s["input_index"] == r["input_index"]
                            && s["status"] == "ok"
                    }) {
                        let val = |x: &Value| {
                            if phase == "query_muls" {
                                x["query_work"]["muls"].as_u64().unwrap() as f64
                            } else {
                                x[phase].as_str().unwrap().parse::<f64>().unwrap()
                            }
                        };
                        let b = val(s);
                        if b > 0.0 {
                            xs.push(val(r) / b);
                        }
                    }
                }
                ratios.insert(format!("{num}/{den}:{phase}"), stats(xs));
            }
        }
        output.push(json!({"group":key,"arms":arms,"paired_ratios":ratios}));
    }
    println!(
        "{}",
        json!({"schema":"vieta-speedup-summary/v1","attempted":total,"successful":ok,"failed":total-ok,"warmup_calls":warm,"groups":output,"counts_scope":"Core primitive counters; fast field constructor/precomputation is timed inside cold but its internal primitive operations are unpriced. No fully priced fast cold-count claim.","timing_standing":"Unprofiled local observations subject to A/A and isolation audit; no L2 or independent signed standing inferred."})
    );
}
pub fn forecast(paths: &[String], rounds: usize, warmups: usize) {
    let mut classes = vec![];
    let mut total = 0.0;
    for path in paths {
        let fixture: Value =
            serde_json::from_slice(&fs::read(Path::new(path).join("fixture.json")).unwrap())
                .unwrap();
        let fixed = fixture["fixture_ns"]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap()
            / 1e9;
        let mut by: BTreeMap<u64, f64> = BTreeMap::new();
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().is_none_or(|x| x != "json") {
                continue;
            }
            let name = p.file_name().unwrap().to_string_lossy();
            if name.starts_with("cell-") {
                let r: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
                *by.entry(r["input_index"].as_u64().unwrap()).or_default() +=
                    r["process_ns"].as_str().unwrap().parse::<f64>().unwrap() / 1e9;
            } else if name.starts_with("input-") {
                let r: Value = serde_json::from_slice(&fs::read(p).unwrap()).unwrap();
                *by.entry(r["input_index"].as_u64().unwrap()).or_default() += r["preparation_ns"]
                    .as_str()
                    .unwrap()
                    .parse::<f64>()
                    .unwrap()
                    / 1e9;
            }
        }
        let maximum = by.values().copied().fold(0.0, f64::max);
        let estimate = fixed + (rounds + warmups) as f64 * maximum;
        total += estimate;
        classes.push(json!({"class":path,"fixed_seconds":fixed,"observed_complete_per_input_seconds":by.values().collect::<Vec<_>>(),"maximum_complete_per_input_seconds":maximum,"forecast_seconds":estimate}));
    }
    println!(
        "{}",
        json!({"schema":"vieta-speedup-forecast/v1","rounds":rounds,"warmups":warmups,"classes":classes,"total_seconds":total,"standing":"Planning bound from fixed fixture plus requested inputs times maximum observed calibration process/preparation cost; not a guarantee or admitted timing."})
    );
}
