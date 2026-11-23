//! Complete-process paired wall analysis; counters never replace missing wall.
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};
fn median(v: &[f64]) -> f64 {
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn geometric(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn band(v: &[f64]) -> [f64; 2] {
    let mut rng = StdRng::seed_from_u64(0xE2E_1009);
    let mut b = Vec::with_capacity(10000);
    for _ in 0..10000 {
        let sample: Vec<_> = (0..v.len()).map(|_| v[rng.gen_range(0..v.len())]).collect();
        b.push(geometric(&sample));
    }
    b.sort_by(f64::total_cmp);
    [b[249], b[9749]]
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: ANALYZE RUN_DIR NEW_SUMMARY_JSON".into());
    }
    let rows: Vec<Value> = fs::read_to_string(Path::new(&a[0]).join("runs.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?;
    let final_row = rows.last().ok_or("missing records")?;
    let mut reasons = Vec::new();
    if final_row["type"] != "complete"
        || final_row["planned"] != 24
        || final_row["valid"] != 24
        || final_row["invalid"] != 0
    {
        reasons.push("planned calls incomplete/invalid".to_string());
    }
    let mut groups: BTreeMap<(String, String), BTreeMap<u64, f64>> = BTreeMap::new();
    let mut expected = BTreeSet::new();
    for arm in ["baseline", "unit", "rho"] {
        expected.insert(("warmup".to_string(), 0, arm.to_string()));
        for i in 0..2 {
            expected.insert(("control".to_string(), i, arm.to_string()));
        }
        for i in 0..5 {
            expected.insert(("measurement".to_string(), i, arm.to_string()));
        }
    }
    let mut starts = BTreeSet::new();
    let mut finishes = BTreeSet::new();
    for r in rows
        .iter()
        .filter(|r| r["type"] == "start" || r["type"] == "result")
    {
        let key = (
            r["phase"].as_str().ok_or("phase")?.to_string(),
            r["round"].as_u64().ok_or("round")?,
            r["arm"].as_str().ok_or("arm")?.to_string(),
        );
        let set = if r["type"] == "start" {
            &mut starts
        } else {
            &mut finishes
        };
        if !set.insert(key) {
            return Err("duplicate execution key".into());
        }
    }
    if starts != expected || finishes != expected {
        reasons.push("registered start/result key set is incomplete or changed".into());
    }
    let mut all = Vec::new();
    for r in rows.iter().filter(|r| r["type"] == "result") {
        let phase = r["phase"].as_str().ok_or("phase")?.to_owned();
        let arm = r["arm"].as_str().ok_or("arm")?.to_owned();
        let round = r["round"].as_u64().ok_or("round")?;
        if r["valid"] != true || r["isolation_admitted"] != true {
            reasons.push(format!("invalid {phase}/{round}/{arm}: {}", r["errors"]));
        }
        if let Some(w) = r["wall_seconds"].as_f64() {
            if w <= 0.0 || !w.is_finite() {
                return Err("invalid wall".into());
            }
            if groups
                .entry((phase.clone(), arm.clone()))
                .or_default()
                .insert(round, w)
                .is_some()
            {
                return Err("duplicate run key".into());
            }
            all.push(json!({"phase":phase,"round":round,"arm":arm,"wall_seconds":w,"valid":r["valid"],"isolation_admitted":r["isolation_admitted"],"scalar":r["result"]["scalar"],"residuals":r["result"]["residuals"]}));
        }
    }
    let mut controls = Vec::new();
    let mut summaries = Vec::new();
    for arm in ["baseline", "unit", "rho"] {
        let c = groups.get(&("control".into(), arm.into()));
        let m = groups.get(&("measurement".into(), arm.into()));
        if c.map_or(0, BTreeMap::len) != 2 || m.map_or(0, BTreeMap::len) != 5 {
            reasons.push(format!("missing registered calls for {arm}"));
            continue;
        }
        let c: Vec<f64> = c.unwrap().values().copied().collect();
        let ratio = c[0].max(c[1]) / c[0].min(c[1]);
        if ratio > 1.10 {
            reasons.push(format!("A/A exceeds1.10 for {arm}: {ratio}"));
        }
        controls.push(json!({"arm":arm,"walls":c,"max_over_min":ratio,"admitted":ratio<=1.10}));
        let m: Vec<f64> = m.unwrap().values().copied().collect();
        summaries.push(json!({"arm":arm,"walls":m,"median_seconds":median(&m),"minimum_seconds":m.iter().copied().fold(f64::INFINITY,f64::min)}));
    }
    let mut comparisons = Vec::new();
    if reasons.is_empty() {
        for (n, d) in [("unit", "baseline"), ("unit", "rho"), ("baseline", "rho")] {
            let x = &groups[&("measurement".into(), n.into())];
            let y = &groups[&("measurement".into(), d.into())];
            let ratios: Vec<_> = (0..5).map(|i| x[&i] / y[&i]).collect();
            let b = band(&ratios);
            comparisons.push(json!({"numerator":n,"denominator":d,"paired_ratios":ratios,"geometric_mean":geometric(&ratios),"bootstrap95":b,"bootstrap_draws":10000,"median_ratio":median(&x.values().copied().collect::<Vec<_>>())/median(&y.values().copied().collect::<Vec<_>>())}));
        }
    }
    let summary = json!({"schema":"nagao.e2e.summary/v1","source_revision":rows[0]["source_revision"],"input_sha256":rows[0]["input_sha256"],"unit":"complete worker process wall seconds","admitted":reasons.is_empty(),"rejection_reasons":reasons,"runs":all,"controls":controls,"arms":summaries,"comparisons":if reasons.is_empty(){json!(comparisons)}else{Value::Null},"complete_operation_count":null,"general_rho_parity":null,"scope":"fixed known-answer weak Fp6 instance; cold per-target state; VM host isolation limits retained"});
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[1])?;
    serde_json::to_writer_pretty(&mut f, &summary)?;
    f.write_all(b"\n")?;
    println!(
        "{}",
        json!({"admitted":summary["admitted"],"rejection_reasons":summary["rejection_reasons"],"arms":summary["arms"],"comparisons":summary["comparisons"]})
    );
    Ok(())
}
