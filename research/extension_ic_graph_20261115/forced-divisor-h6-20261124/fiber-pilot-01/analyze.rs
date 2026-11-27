//! Read-only aggregation of the complete frozen count pilot; no solve.
use serde_json::{json, Value};
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
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(a.len() == 3, "RAW_ROOT NEW_ANALYSIS_DIR")?;
    let root = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    ensure(!out.exists(), "exclusive output")?;
    let raw: Value = serde_json::from_slice(&fs::read(root.join("output/summary.json"))?)?;
    let conditions: Value = serde_json::from_slice(&fs::read(root.join("conditions.jsonl"))?)?;
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
    ensure(
        raw["planned_calls"] == 27
            && raw["attempted_calls"] == 27
            && raw["not_run_calls"] == 0
            && raw["stopped"].is_null()
            && raw["complete_reference_comparison"] == true,
        "complete comparison",
    )?;
    let samples = raw["samples"].as_array().ok_or("samples")?;
    ensure(samples.len() == 27, "denominator")?;
    let parameters = [369u64, 0, 1, 2, 3, 4, 5, 6, 7];
    let modes = ["full-reference", "plain", "traced"];
    let mut rows = Vec::new();
    let mut details = Vec::new();
    for (arm, mode) in modes.iter().enumerate() {
        let (
            mut all,
            mut dev,
            mut f4,
            mut lin,
            mut roots,
            mut dev_roots,
            mut accepted,
            mut dev_accepted,
            mut post,
        ) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
        let (mut dmin, mut dmax) = (u64::MAX, 0u64);
        let mut replayed = 0u64;
        for (index, &parameter) in parameters.iter().enumerate() {
            let s = &samples[arm * 9 + index];
            let reference = &samples[index];
            ensure(
                s["mode"] == *mode && s["case"] == index && s["parameter"] == parameter,
                "fixed order/values",
            )?;
            ensure(
                s["resolved"] == true && s["invalid_original_replay"] == false,
                "resolved original replay",
            )?;
            ensure(
                s["input_sha256"] == reference["input_sha256"]
                    && s["roots"] == reference["roots"]
                    && s["functions"] == reference["functions"],
                "same input/roots/functions",
            )?;
            if arm != 0 {
                ensure(
                    s["matches_full_reference"] == true
                        && s["resolution_flags_match_full_reference"] == true,
                    "candidate/reference flags",
                )?;
            }
            let st = &s["root_stats"];
            ensure(
                st["timed_out"] == false && st["incomplete"] == false && st["nonseparating"] == 0,
                "root flags",
            )?;
            let products = n(&st["f4_muls"])? + n(&st["lin_muls"])?;
            all = all.checked_add(products).ok_or("sum overflow")?;
            f4 += n(&st["f4_muls"])?;
            lin += n(&st["lin_muls"])?;
            let functions = s["functions"].as_array().ok_or("functions")?;
            ensure(
                functions
                    .iter()
                    .all(|f| f.get("function_refusal").is_none()),
                "observed norm functions have no saturation refusal",
            )?;
            let count = s["roots"].as_array().ok_or("roots")?.len() as u64;
            ensure(functions.len() as u64 == count, "root/function denominator")?;
            let valid = functions
                .iter()
                .filter(|r| r.get("support").is_some() && r["original_group_replay"] == true)
                .count() as u64;
            roots += count;
            accepted += valid;
            if index != 0 {
                dev += products;
                dev_roots += count;
                dev_accepted += valid;
            }
            post += n(&s["post_legacy_fq_products"])?;
            let delta = n(&st["delta"])?;
            dmin = dmin.min(delta);
            dmax = dmax.max(delta);
            replayed += u64::from(st["replayed"] == true);
            details.push(json!({"mode":mode,"case":index,"parameter":parameter,"solver_products":products,"delta":delta,"roots":count,"admissible_principal_rows":valid,"control":index==0}));
        }
        ensure(
            roots == 10 && dev_roots == 8 && accepted == 1 && dev_accepted == 0,
            "observed geometry/yield scope",
        )?;
        rows.push(json!({"mode":mode,"calls":9,"solver_f4_products":f4,"solver_linear_products":lin,"solver_products":all,"positive_control_solver_products":all-dev,"development_solver_products":dev,"root_occurrences":roots,"development_norm_functions":dev_roots,"admissible_control_rows":accepted-dev_accepted,"admissible_development_rows":dev_accepted,"delta_min":dmin,"delta_max":dmax,"trace_replays":replayed,"post_legacy_fq_products":post}));
    }
    let baseline = n(&rows[1]["solver_products"])? as f64;
    ensure(baseline > 0.0, "positive comparison denominator")?;
    for row in &mut rows {
        row["solver_ratio_to_plain"] = json!(n(&row["solver_products"])? as f64 / baseline);
    }
    let result = json!({"schema":"graph-ic.h6-fiber-analysis/v1","source_revision":"a2c085beed182c1ed9fb6a8d6db1ab9855434e18","parameters":parameters,"rows":rows,"cases":details,"setups":raw["setups"],"admission_legacy_fq_products":raw["admission_and_positive_binding_legacy_fq_products"],"native_conditions":conditions,"unit":"existing F4 plus root-stage product counters; setup/post legacy counters separate","full_native_cost_available":false,"wall_comparison":false,"cost_per_new_admissible_row":null,"rho_comparison":null,"signed_verification":false});
    fs::create_dir(out)?;
    fs::write(
        out.join("derived.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"950\" height=\"285\"><rect width=\"950\" height=\"285\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"24\" y=\"30\" font-size=\"20\">H6 fiber pilot: matching answers, no new admissible principal row</text><text x=\"24\" y=\"54\" font-size=\"14\">Nine fibers per arm (one positive control + eight development values); solver products only</text>");
    let maximum = n(&result["rows"][0]["solver_products"])? as f64;
    for (i, row) in result["rows"].as_array().unwrap().iter().enumerate() {
        let value = n(&row["solver_products"])?;
        let width = value as f64 / maximum * 520.0;
        let y = 80 + i * 52;
        let color = ["#8799aa", "#52738c", "#007f73"][i];
        svg.push_str(&format!("<text x=\"24\" y=\"{}\" font-size=\"15\">{}</text><rect x=\"180\" y=\"{y}\" width=\"{width}\" height=\"28\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"14\">{value}</text>",y+20,row["mode"].as_str().unwrap(),width+192.0,y+20));
    }
    svg.push_str("<text x=\"24\" y=\"260\" font-size=\"13\">Field coverage incomplete. Retained relation excluded from new yield. No wall ranking or rho claim. PCC unsigned.</text></g></svg>");
    fs::write(out.join("solver-products.svg"), svg)?;
    println!("{}", serde_json::to_string_pretty(&result["rows"])?);
    Ok(())
}
