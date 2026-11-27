//! Read-only accounting of the complete registered raw panel. No algebraic
//! solve or new timing sample. All figures derive from preserved raw fields.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn number(v: &Value) -> Result<u64> {
    v.as_u64().ok_or_else(|| "integer field".into())
}
fn sum_cost(sample: &Value) -> Result<u64> {
    sample["cases"]
        .as_array()
        .ok_or("cases")?
        .iter()
        .try_fold(0u64, |total, c| {
            ["weil_muls", "f4_muls", "lin_muls", "post_muls"]
                .iter()
                .try_fold(total, |n, key| {
                    n.checked_add(number(&c["cost"][key])?)
                        .ok_or_else(|| "overflow".into())
                })
        })
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    ensure(a.len() == 3, "RAW_ROOT NEW_OUTPUT_DIRECTORY")?;
    let root = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    ensure(!out.exists(), "exclusive analysis output")?;
    let public = fs::read(root.join("public.json"))?;
    let expected: Value = serde_json::from_slice(&fs::read(root.join("expected.json"))?)?;
    let raw: Value = serde_json::from_slice(&fs::read(root.join("output/summary.json"))?)?;
    let native: Value = serde_json::from_slice(&fs::read(root.join("conditions.jsonl"))?)?;
    let hash = format!("{:x}", Sha256::digest(&public));
    ensure(
        expected["public_sha256"] == hash && raw["public_sha256"] == hash,
        "public binding",
    )?;
    for f in ["native.status", "posthash.status", "launcher.status"] {
        ensure(fs::read_to_string(root.join(f))?.trim() == "0", "status0")?;
    }
    ensure(
        fs::metadata(root.join("worker.stderr"))?.len() == 0,
        "empty stderr",
    )?;
    ensure(
        raw["attempted_intervals"] == 18
            && raw["not_run_intervals"] == 0
            && raw["stopped"].is_null(),
        "complete registered panel",
    )?;
    let samples = raw["samples"].as_array().ok_or("samples")?;
    ensure(samples.len() == 18, "sample denominator")?;
    let mut reference = None;
    let mut original_roots = 0u64;
    for (i, s) in samples.iter().enumerate() {
        ensure(
            s["ordinal"] == i && s["correct"] == true,
            "ordinal/correctness",
        )?;
        ensure(
            s["full_native_cost_available"] == false,
            "partial native coverage",
        )?;
        let cases = s["cases"].as_array().ok_or("two cases")?;
        ensure(cases.len() == 2, "case denominator")?;
        for (j, c) in cases.iter().enumerate() {
            ensure(c["roots"] == expected["roots"][j], "exact expected roots")?;
            original_roots += c["roots"].as_array().ok_or("roots array")?.len() as u64;
            let st = &c["root_stats"];
            ensure(
                st["incomplete"] == false
                    && st["timed_out"] == false
                    && st["inconsistent"] == false
                    && st["nonseparating"] == 0,
                "resolved root flags",
            )?;
            ensure(
                c["decompositions"]
                    .as_array()
                    .ok_or("decompositions")?
                    .is_empty(),
                "actual zero decomposition scope",
            )?;
            ensure(
                c["cost"]["split"] == 0 && c["cost"]["decompositions"] == 0,
                "materializer zeros",
            )?;
        }
        let fingerprint = json!({"base":s["factor_base_x_sha256"],"cases":cases.iter().map(|c|json!({"roots":c["roots"],"decompositions":c["decompositions"]})).collect::<Vec<_>>()});
        if let Some(r) = &reference {
            ensure(r == &fingerprint, "matched roots/base/materialization")?;
        } else {
            reference = Some(fingerprint);
        }
    }
    let panel: Vec<_> = samples.iter().filter(|s| s["phase"] == "panel").collect();
    let expected_order = [
        "plain", "traced", "template", "traced", "template", "plain", "template", "plain",
        "traced", "plain", "template", "traced", "template", "traced", "plain",
    ];
    ensure(
        panel.len() == 15
            && panel
                .iter()
                .zip(expected_order)
                .all(|(s, arm)| s["arm"] == arm),
        "frozen round order",
    )?;
    let mut rows = Vec::new();
    for arm in ["plain", "traced", "template"] {
        let these: Vec<_> = panel.iter().copied().filter(|s| s["arm"] == arm).collect();
        ensure(these.len() == 5, "five samples per arm")?;
        let mut walls: Vec<_> = these
            .iter()
            .map(|s| s["wall_s"].as_f64().ok_or("wall float"))
            .collect::<std::result::Result<_, _>>()?;
        ensure(
            walls.iter().all(|v| v.is_finite() && *v > 0.0),
            "finite positive wall",
        )?;
        walls.sort_by(f64::total_cmp);
        let fields: Vec<_> = these.iter().map(|s| sum_cost(s)).collect::<Result<_>>()?;
        let f4s: Vec<u64> = these
            .iter()
            .map(|s| {
                s["cases"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|c| number(&c["cost"]["f4_muls"]))
                    .sum()
            })
            .collect::<Result<_>>()?;
        for s in &these {
            if arm == "traced" {
                ensure(
                    s["cases"][0]["root_stats"]["replayed"] == false
                        && s["cases"][1]["root_stats"]["replayed"] == true
                        && s["cases"][1]["root_stats"]["trace_mismatch"] == false,
                    "traced learns then holds",
                )?;
            }
            if arm == "template" {
                let t = &s["cases"][1]["template_audit"];
                ensure(
                    t["basis_accepted"] == true
                        && t["fallback_used"] == false
                        && t["checker_counter_feature"] == true,
                    "exact candidate path",
                )?;
                ensure(
                    number(&t["attempted_elimination"]["modular_products"])?
                        + number(&t["checker"]["modular_products"])?
                        == number(&s["cases"][1]["cost"]["f4_muls"])?,
                    "no dropped template/checker products",
                )?;
                ensure(
                    s["cases"][0]["learning"]["compiled_rows"] == 2184
                        && s["cases"][0]["learning"]["compiled_columns"] == 1716,
                    "candidate geometry",
                )?;
                ensure(
                    number(&s["cases"][0]["learning"]["f4_muls"])?
                        + number(&s["cases"][0]["learning"]["compilation_checker_products"])?
                        == number(&s["cases"][0]["cost"]["f4_muls"])?,
                    "fresh development and compilation charged",
                )?;
            }
        }
        rows.push(json!({"arm":arm,"samples":5,"wall_median_s":walls[2],"wall_min_s":walls[0],"wall_max_s":walls[4],"partial_nagao_field_products_min":fields.iter().min(),"partial_nagao_field_products_max":fields.iter().max(),"f4_and_template_products_min":f4s.iter().min(),"f4_and_template_products_max":f4s.iter().max(),"useful_decompositions":0}));
    }
    let traced = rows[1]["partial_nagao_field_products_min"]
        .as_u64()
        .unwrap() as f64;
    let traced_wall = rows[1]["wall_median_s"].as_f64().unwrap();
    for row in &mut rows {
        row["partial_field_ratio_to_traced"] =
            json!(row["partial_nagao_field_products_min"].as_u64().unwrap() as f64 / traced);
        row["local_wall_median_ratio_to_traced"] =
            json!(row["wall_median_s"].as_f64().unwrap() / traced_wall);
    }
    let aa: Vec<_> = samples
        .iter()
        .filter(|s| s["phase"] == "aa")
        .map(|s| s["wall_s"].as_f64().unwrap())
        .collect();
    ensure(aa.len() == 2, "two A/A intervals")?;
    let aa_ratio = aa[0].max(aa[1]) / aa[0].min(aa[1]);
    ensure(
        aa_ratio <= 1.10 && raw["aa_max_over_min"].as_f64() == Some(aa_ratio),
        "frozen A/A acceptance",
    )?;
    ensure(
        native["run"]["exit_status"] == 0 && native["run"]["contended"] == false,
        "native process subgate",
    )?;
    let result = json!({"schema":"nagao.template-cost.analysis/v1","public_sha256":hash,"source_revision":"99e80565ce7c3e72637bba7f2b73c8b1fb35278a","all_intervals":18,"panel_intervals":15,"saved_root_occurrences_checked":original_roots,"nonempty_divisor_checks":0,"correctness":"saved root sets and matched zero materializations; planted positive control is separate","aa_max_over_min":aa_ratio,"warmup_wall_s":samples[0]["wall_s"],"experiment_wall_s":raw["experiment_wall_s_including_common_warmup"],"rows":rows,"native_conditions":native,"full_native_cost_available":false,"signed_admission":false,"rho_ratio":null,"external_solver_comparison":"not run"});
    fs::create_dir(out)?;
    fs::write(
        out.join("derived.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    // A standard static scientific figure: logarithmic field-count axis with
    // every arm measured in the same explicitly partial unit.
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"900\" height=\"310\" viewBox=\"0 0 900 310\"><rect width=\"900\" height=\"310\" fill=\"white\"/><g font-family=\"sans-serif\" fill=\"#172b4d\"><text x=\"24\" y=\"30\" font-size=\"21\">H2 matches roots, but dense elimination loses on this exposed pair</text><text x=\"24\" y=\"54\" font-size=\"14\">Partial Nagao field products; two rejection residuals; five repetitions per arm</text>");
    let left = 170.0;
    let scale = 125.0;
    for exponent in 5..=10 {
        let x = left + (exponent as f64 - 5.0) * scale;
        svg.push_str(&format!("<path d=\"M{x} 72V240\" stroke=\"#d8e0e8\"/><text x=\"{}\" y=\"260\" font-size=\"13\">10^{exponent}</text>",x-16.0));
    }
    for (i, r) in result["rows"].as_array().unwrap().iter().enumerate() {
        let value = r["partial_nagao_field_products_min"].as_u64().unwrap();
        let x = left + ((value as f64).log10() - 5.0) * scale;
        let y = 99 + i * 58;
        let color = ["#52738c", "#007f73", "#b54a3b"][i];
        svg.push_str(&format!("<text x=\"24\" y=\"{}\" font-size=\"16\">{}</text><circle cx=\"{x}\" cy=\"{y}\" r=\"7\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"14\">{value}</text>",y+5,r["arm"].as_str().unwrap(),x+12.0,y+5));
    }
    svg.push_str("<text x=\"24\" y=\"292\" font-size=\"13\">Zero useful decompositions. Setup bookkeeping is incomplete; no full native or rho claim. PCC unsigned.</text></g></svg>");
    fs::write(out.join("counts.svg"), svg)?;
    println!("{}", serde_json::to_string_pretty(&result["rows"])?);
    Ok(())
}
