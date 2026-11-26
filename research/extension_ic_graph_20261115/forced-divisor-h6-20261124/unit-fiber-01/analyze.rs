//! Read-only Rust analysis of the sole unit pilot and its frozen references.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ensure(b: bool, s: &str) -> Result<()> {
    if b {
        Ok(())
    } else {
        Err(s.into())
    }
}
fn n(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "integer counter".into())
}
fn read(root: &Path, name: &str) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(root.join(name))?)?)
}
fn sha(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn products(s: &Value) -> Result<u64> {
    Ok(n(&s["root_stats"]["f4_muls"])? + n(&s["root_stats"]["lin_muls"])?)
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(a.len() == 3, "RAW_ROOT NEW_ANALYSIS_DIR")?;
    let root = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    ensure(!out.exists(), "exclusive output")?;
    for name in ["native.status", "posthash.status", "launcher.status"] {
        ensure(
            fs::read_to_string(root.join(name))?.trim() == "0",
            "native/hash/launcher status",
        )?;
    }
    ensure(
        fs::metadata(root.join("worker.stderr"))?.len() == 0,
        "empty stderr",
    )?;
    let reference_bytes = fs::read(root.join("reference.json"))?;
    ensure(
        sha(&reference_bytes) == "71d049019f747b85ee1f71086fadf878ec9b33e42cceb8d4d282b38686c7a3db",
        "frozen reference hash",
    )?;
    let reference: Value = serde_json::from_slice(&reference_bytes)?;
    let raw = read(root, "output/summary.json")?;
    let seed = read(root, "seed.json")?;
    let conditions = read(root, "conditions.jsonl")?;
    ensure(
        raw["planned_calls"] == 18
            && raw["attempted_calls"] == 18
            && raw["not_run_calls"] == 0
            && raw["stopped"].is_null()
            && raw["complete_reference_comparison"] == true,
        "complete panel",
    )?;
    ensure(
        reference["planned_calls"] == 27
            && reference["attempted_calls"] == 27
            && reference["complete_reference_comparison"] == true,
        "complete frozen panel",
    )?;
    let parameters = [369u64, 0, 1, 2, 3, 4, 5, 6, 7];
    ensure(
        raw["parameters"] == json!(parameters) && reference["parameters"] == json!(parameters),
        "parameters",
    )?;
    let samples = raw["samples"].as_array().ok_or("samples")?;
    let old = reference["samples"].as_array().ok_or("reference samples")?;
    ensure(
        samples.len() == 18 && old.len() == 27,
        "complete denominators",
    )?;
    let events: Vec<Value> = fs::read_to_string(root.join("output/events.jsonl"))?
        .lines()
        .map(serde_json::from_str)
        .collect::<std::result::Result<_, _>>()?;
    ensure(events.len() == 36, "all start/result events")?;
    let p = n(&seed["p"])?;
    let im = n(&seed["h"][6][1])?;
    let k = ((p - im) % p) * p.div_ceil(2) % p;
    let mut rows = Vec::new();
    let mut details = Vec::new();
    for (arm, mode) in ["plain", "traced"].iter().enumerate() {
        let setup = &raw["setups"][arm];
        ensure(setup["mode"] == *mode, "setup order")?;
        let (mut all, mut dev, mut old_all, mut old_dev, mut f4, mut lin, mut replays, mut post) =
            (0, 0, 0, 0, 0, 0, 0, 0);
        let (mut roots, mut accepted, mut dev_accepted) = (0, 0, 0);
        let mut coefficient_products = n(&setup["unit_coefficient_work"]["products"])?;
        let mut coefficient_additions = n(&setup["unit_coefficient_work"]["additions"])?;
        for (index, &parameter) in parameters.iter().enumerate() {
            let pos = arm * 9 + index;
            let s = &samples[pos];
            let r = &old[index];
            let baseline = &old[(arm + 1) * 9 + index];
            ensure(
                s["mode"] == *mode && s["case"] == index && s["parameter"] == parameter,
                "fixed call order",
            )?;
            ensure(
                events[2 * pos]["event"] == "start"
                    && events[2 * pos]["mode"] == *mode
                    && events[2 * pos]["case"] == index
                    && events[2 * pos + 1] == *s,
                "event/result equality",
            )?;
            ensure(
                s["resolved"] == true
                    && s["invalid_original_replay"] == false
                    && s["matches_full_reference"] == true
                    && s["resolution_flags_match_full_reference"] == true,
                "complete result flags",
            )?;
            ensure(
                s["roots"] == r["roots"]
                    && s["functions"] == r["functions"]
                    && baseline["roots"] == r["roots"]
                    && baseline["functions"] == r["functions"],
                "all reference outputs",
            )?;
            let st = &s["root_stats"];
            for name in ["timed_out", "incomplete", "inconsistent", "nonseparating"] {
                ensure(st[name] == r["root_stats"][name], "resolution flags")?;
            }
            ensure(st["delta"] == 70, "quotient dimension")?;
            let mut lifted = Vec::new();
            for z in s["reduced_roots"].as_array().ok_or("reduced roots")? {
                let z: Vec<u64> = serde_json::from_value(z.clone())?;
                ensure(
                    z.len() == 6 && z.iter().all(|&x| x < p),
                    "canonical reduced root",
                )?;
                let mut point = vec![(z[4] * z[5] + k) % p];
                point.extend(z);
                lifted.push(point);
            }
            lifted.sort();
            lifted.dedup();
            ensure(json!(lifted) == s["roots"], "independent coordinate lift")?;
            for name in ["specialization_work", "lifting_work"] {
                coefficient_products += n(&s[name]["products"])?;
                coefficient_additions += n(&s[name]["additions"])?;
            }
            let count = s["roots"].as_array().ok_or("roots")?.len() as u64;
            let functions = s["functions"].as_array().ok_or("functions")?;
            ensure(functions.len() as u64 == count, "root/function count")?;
            let valid = functions
                .iter()
                .filter(|r| r["original_group_replay"] == true)
                .count() as u64;
            roots += count;
            accepted += valid;
            let cost = products(s)?;
            let old_cost = products(baseline)?;
            all += cost;
            old_all += old_cost;
            f4 += n(&st["f4_muls"])?;
            lin += n(&st["lin_muls"])?;
            replays += u64::from(st["replayed"] == true);
            post += n(&s["post_legacy_fq_products"])?;
            if index > 0 {
                dev += cost;
                old_dev += old_cost;
                dev_accepted += valid;
            } else {
                ensure(
                    s["retained_positive_control_pass"] == true && valid == 1,
                    "positive control",
                )?;
            }
            details.push(json!({"mode":mode,"case":index,"parameter":parameter,"solver_products":cost,"prior_same_mode_solver_products":old_cost,"roots":count,"admissible_rows":valid,"delta":st["delta"],"solving_degree":st["solving_degree"]}));
        }
        ensure(
            roots == 10 && accepted == 1 && dev_accepted == 0,
            "yield scope",
        )?;
        rows.push(json!({"mode":mode,"calls":9,"solver_products":all,"f4_products":f4,"linear_products":lin,"development_products":dev,"prior_same_mode_solver_products":old_all,"prior_same_mode_development_products":old_dev,"ratio_to_prior_same_mode":all as f64/old_all as f64,"unit_coefficient_products":coefficient_products,"unit_coefficient_additions":coefficient_additions,"post_legacy_fq_products":post,"trace_replays":replays,"admissible_control_rows":1,"admissible_development_rows":0}));
    }
    let old_traced = n(&rows[1]["prior_same_mode_solver_products"])?;
    for row in &mut rows {
        row["ratio_to_prior_traced"] =
            json!(n(&row["solver_products"])? as f64 / old_traced as f64);
    }
    let result = json!({"schema":"graph-ic.h6-unit-analysis/v1","source_revision":"fe68282737851bb99b8342df7c32f5c7a6cdcada","rows":rows,"cases":details,"setups":raw["setups"],"parameters":parameters,"conditions":conditions,"source_reference_sha256":sha(&reference_bytes),"unit":"existing F4+linear solver products; new coefficient work and legacy Fq counters separate","full_native_cost_available":false,"wall_comparison":false,"rho_comparison":null,"signed_verification":false,"cost_per_new_admissible_row":null});
    fs::create_dir(out)?;
    fs::write(
        out.join("derived.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    let bars = [
        (
            "prior plain",
            n(&rows[0]["prior_same_mode_solver_products"])?,
        ),
        ("prior traced", old_traced),
        ("unit plain", n(&rows[0]["solver_products"])?),
        ("unit traced", n(&rows[1]["solver_products"])?),
    ];
    let max = bars.iter().map(|(_, v)| *v).max().unwrap() as f64;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"950\" height=\"325\"><rect width=\"950\" height=\"325\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"24\" y=\"30\" font-size=\"20\">H6 unit elimination: exact solutions, higher solver cost</text><text x=\"24\" y=\"54\" font-size=\"14\">Same nine fibers; prior baseline reused from frozen data; solver product counters only</text>");
    for (i, (label, value)) in bars.iter().enumerate() {
        let y = 80 + i * 49;
        let width = *value as f64 / max * 520.0;
        let color = ["#52738c", "#007f73", "#b07b57", "#a05d42"][i];
        svg.push_str(&format!("<text x=\"24\" y=\"{}\" font-size=\"15\">{label}</text><rect x=\"180\" y=\"{y}\" width=\"{width}\" height=\"28\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"14\">{value}</text>",y+20,width+192.0,y+20));
    }
    svg.push_str("<text x=\"24\" y=\"305\" font-size=\"13\">New coefficient work is separate. No new graph rows; no wall ranking. Producer evidence UNVERIFIED.</text></g></svg>");
    fs::write(out.join("solver-products.svg"), svg)?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
