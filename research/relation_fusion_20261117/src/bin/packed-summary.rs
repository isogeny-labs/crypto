//! Descriptive phase accounting from frozen analysis211/raw200; does not alter admission.
use serde_json::{json, Value};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn med(v: &[u64]) -> u64 {
    let mut v = v.to_vec();
    v.sort_unstable();
    v[v.len() / 2]
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage:packed-summary ANALYSIS RAW_SUMMARY NEW_REPORT".into());
    }
    let analysis: Value = serde_json::from_slice(&fs::read(&a[1])?)?;
    let raw: Value = serde_json::from_slice(&fs::read(&a[2])?)?;
    let mut rows = Vec::new();
    let mut md=String::from("# Complete packed comparison200: all six cases\n\nNo15x admission. All1260calls complete;1260full saved-proof replays. Zero monitor-flagged contention. All observations/outliers retained. [pcc:unverified]\n\n|Prime|n|Primary median ratio|Six-case lower|Max symmetric A/A|Primary parent ms|Construct+wire ms|Worker replay ms|Strongest control median ms|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|\n");
    for case in analysis["cases"].as_array().ok_or("cases")? {
        let arms = case["arms"].as_array().ok_or("arms")?;
        let primary = arms
            .iter()
            .find(|r| r["arm"] == "packed-dag-487")
            .ok_or("primary")?;
        let p = case["prime"].as_u64().ok_or("prime")?;
        let n = case["n"].as_u64().ok_or("n")?;
        let parent = primary["median_parent_ns"]
            .as_f64()
            .ok_or("parent median")?;
        let parse = |key: &str| -> Result<Vec<u64>> {
            primary[key]
                .as_array()
                .ok_or("phase array")?
                .iter()
                .map(|v| v.as_u64().ok_or_else(|| "phase integer".into()))
                .collect()
        };
        let construct = med(&parse("construct_ns")?);
        let replay = med(&parse("replay_ns")?);
        let strongest = arms[..12]
            .iter()
            .min_by(|a, b| {
                a["median_parent_ns"]
                    .as_f64()
                    .unwrap()
                    .total_cmp(&b["median_parent_ns"].as_f64().unwrap())
            })
            .ok_or("control")?;
        let control = strongest["median_parent_ns"]
            .as_f64()
            .ok_or("control median")?;
        let mut phase_rows = Vec::new();
        for r in raw["outcomes"].as_array().ok_or("raw")?.iter().filter(|r| {
            r["stage"] == "AB" && r["prime"] == p && r["n"] == n && r["arm"] == "packed-dag-487"
        }) {
            let total = r["parent_ns"].as_u64().ok_or("parent")?;
            let construction = r["construct_ns"].as_u64().ok_or("construction")?;
            let whole = r["worker_ns"].as_u64().ok_or("worker")?;
            phase_rows.push(json!({"round":r["round"],"parent_ns":total,"constructor_and_wire_ns":construction,"whole_worker_through_proof_write_ns":whole,"outside_worker_ns":total.checked_sub(whole).ok_or("phase containment")?,"hypothetical_speedup_if_only_constructor_becomes_free_and_rest_fixed":total as f64/(total-construction) as f64}));
        }
        let bounds = &case["primary_packed_dag_487_vs_strongest"];
        let ratio = bounds["median"].as_f64().ok_or("ratio")?;
        let lower = bounds["six_case_simultaneous_lower"]
            .as_f64()
            .ok_or("lower")?;
        let aa = case["aa_symmetric_max_ratio"].as_f64().ok_or("AA")?;
        md += &format!(
            "|{p}|{n}|{ratio:.6}|{lower:.6}|{aa:.6}|{:.6}|{:.6}|{:.6}|{:.6}|\n",
            parent / 1e6,
            construct as f64 / 1e6,
            replay as f64 / 1e6,
            control / 1e6
        );
        rows.push(json!({"prime":p,"n":n,"primary_ratio":ratio,"six_case_lower":lower,"pointwise95_high":bounds["pointwise95_high"],"aa_symmetric_max":aa,"primary_median_parent_ns":parent,"primary_median_construct_and_wire_ns":construct,"primary_median_replay_ns":replay,"strongest_control_by_median":strongest["arm"],"strongest_control_median_parent_ns":control,"15x_target_ns_against_that_control_median":control/15.0,"raw_primary_phase_rows":phase_rows}));
    }
    md+="\nConstruction and replay phase clocks are descriptive. Ratios use the fastest of12controls in each paired round, which can differ from the arm with the lowest overall median. Hypothetical free-constructor calculations assume every other measured cost stays fixed; they are not runtime results or universal lower bounds. No F4 solve runs; observed solving degree remains null. Full ordinary root unit qualification remains incomplete.\n";
    let dir = Path::new(&a[3]);
    fs::create_dir(dir)?;
    for (name, b) in [
        (
            "summary.json",
            serde_json::to_vec_pretty(
                &json!({"schema":"packed-descriptive-summary/v1","cases":rows,"primary_15x_admitted":analysis["primary_15x_admitted"],"admission_source":"frozen analyzer211","conditional_phase_model_is_not_a_runtime_result":true}),
            )?,
        ),
        ("report.md", md.into_bytes()),
    ] {
        fs::File::options()
            .write(true)
            .create_new(true)
            .open(dir.join(name))?
            .write_all(&b)?;
    }
    Ok(())
}
