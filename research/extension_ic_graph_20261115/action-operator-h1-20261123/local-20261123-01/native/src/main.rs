//! Read-only arithmetic validation and rendering of the frozen local records.
//! No solver, randomness, timing, or new target is used here.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn require(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn number(v: &Value) -> Result<u128> {
    match v.as_u64() {
        Some(n) => Ok(u128::from(n)),
        None => Ok(v.as_str().ok_or("unsigned number")?.parse()?),
    }
}
fn add(a: &mut u128, b: u128) -> Result<()> {
    *a = a.checked_add(b).ok_or("counter sum overflow")?;
    Ok(())
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn run(root: &Path, out: &Path) -> Result<()> {
    let cases_bytes = fs::read(root.join("output/cases.jsonl"))?;
    let summary_bytes = fs::read(root.join("output/summary.json"))?;
    let summary: Value = serde_json::from_slice(&summary_bytes)?;
    let selection: Value = serde_json::from_slice(&fs::read(root.join("output/selection.json"))?)?;
    require(
        fs::read_to_string(root.join("exit.status"))?.trim() == "0",
        "native status",
    )?;
    require(
        fs::metadata(root.join("stderr.log"))?.len() == 0,
        "empty native stderr",
    )?;
    require(
        summary["passed"] == true
            && summary["f4_calls"] == 32
            && summary["candidate_linear_calls"] == 192,
        "summary status/call counts",
    )?;
    let mut sums = [[0u128; 6]; 2];
    let mut fallback = [0u128; 6];
    let mut compact = String::new();
    let mut count = 0usize;
    let mut roots = 0u128;
    for (i, line) in std::str::from_utf8(&cases_bytes)?.lines().enumerate() {
        let row: Value = serde_json::from_str(line)?;
        require(
            row["index"] == i && row["passed"] == true && row["f4_calls"] == 1,
            "ordered case status",
        )?;
        let actions = row["actions"].as_array().ok_or("actions")?;
        require(actions.len() == 6 && i < 32, "six actions and case range")?;
        let mut costs = Vec::new();
        for (a, v) in actions.iter().enumerate() {
            require(
                v["action"] == a && v["passed"] == true,
                "action identity/status",
            )?;
            require(
                v["roots_sha256"] == row["expected_roots_sha256"],
                "exact roots binding",
            )?;
            let value = number(&v["lin_muls_partial"])?;
            add(&mut sums[usize::from(i >= 16)][a], value)?;
            if v["dense_fallback"] == true {
                add(&mut fallback[a], 1)?;
            }
            if a == 5 {
                require(
                    v["lin_muls_partial"] == row["incumbent"]["lin_muls"],
                    "default case counter",
                )?;
            }
            costs.push(value.to_string());
        }
        add(&mut roots, number(&row["fp_roots"])?)?;
        compact.push_str(&json!({"index":i,"input_sha256":row["input_sha256"],
            "basis_sha256":row["basis_sha256"],"expected_roots_sha256":row["expected_roots_sha256"],
            "root_count":row["fp_roots"],"all_six_root_digests_match":true,
            "partial_muls":costs,"fallbacks":actions.iter().map(|v|v["dense_fallback"].clone()).collect::<Vec<_>>()}).to_string());
        compact.push('\n');
        count += 1;
    }
    require(count == 32, "all 32 cases")?;
    for a in 0..6 {
        require(
            number(&summary["development_partial_muls"][a])? == sums[0][a],
            "development total",
        )?;
        require(
            number(&summary["held_out_partial_muls"][a])? == sums[1][a],
            "held-out total",
        )?;
        require(
            number(&summary["dense_fallback_counts"][a])? == fallback[a],
            "fallback total",
        )?;
        require(
            number(&selection["partial_multiplication_sums"][a])? == sums[0][a],
            "selection totals",
        )?;
    }
    let selected = (0..6)
        .min_by_key(|&a| (sums[0][a], a != 5, a))
        .ok_or("selection")?;
    require(
        summary["selected"] == selected && selection["selected"] == selected,
        "frozen selection rule",
    )?;
    require(
        selection["rule_frozen_before_held_out"] == true,
        "selection record",
    )?;
    require(
        number(&summary["held_out_incumbent_partial_muls"])? == sums[1][5],
        "incumbent total",
    )?;
    require(
        number(&summary["held_out_selected_partial_muls"])? == sums[1][selected],
        "selected total",
    )?;
    let mut search = 0;
    for x in sums[0] {
        add(&mut search, x)?;
    }
    require(
        number(&summary["development_all_candidate_partial_muls"])? == search,
        "search charge",
    )?;
    let mut with_search = search;
    add(&mut with_search, sums[1][selected])?;
    require(
        number(&summary["selected_with_development_search_partial_muls"])? == with_search,
        "search plus held-out",
    )?;
    let mut incumbent_all = sums[0][5];
    add(&mut incumbent_all, sums[1][5])?;
    let rows:Vec<_>=(0..6).map(|a|json!({"action":a,"development":sums[0][a].to_string(),
        "held_out":sums[1][a].to_string(),"held_out_ratio_to_incumbent":sums[1][a] as f64/sums[1][5] as f64})).collect();
    let result = json!({"schema":"h1.local.record-analysis/v1","passed":true,"cases":count,
        "candidate_root_checks":count*6,"captured_roots_total":roots.to_string(),"rows":rows,
        "selected":selected,"development_search_partial_muls":search.to_string(),
        "selected_with_search_partial_muls":with_search.to_string(),"incumbent_both_halves_partial_muls":incumbent_all.to_string(),
        "raw_cases_sha256":hash(&cases_bytes),"original_summary_sha256":hash(&summary_bytes),
        "native_exit":0,"launcher_tool_reported_exit":1,"launcher_exit_discrepancy_unresolved":true,
        "counter_scope":"partial lin_muls; no elapsed or complete solver cost","signed_verdict":false});
    write_new(&out.join("compact-cases.jsonl"), compact.as_bytes())?;
    write_new(
        &out.join("derived.json"),
        serde_json::to_string_pretty(&result)?.as_bytes(),
    )?;
    let max = sums[1].iter().copied().max().ok_or("maximum")? as f64;
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1100\" height=\"510\" viewBox=\"0 0 1100 510\"><rect width=\"1100\" height=\"510\" fill=\"white\"/><g font-family=\"DejaVu Sans\" fill=\"#183449\"><text x=\"30\" y=\"35\" font-size=\"23\">H1 local run 01: held-out multiplication counters</text><text x=\"30\" y=\"65\" font-size=\"15\">16 saved systems; original coordinate 5 retained; incomplete arithmetic metric</text>");
    for (a, &value) in sums[1].iter().enumerate() {
        let y = 105 + a * 51;
        let width = value as f64 / max * 620.0;
        let color = if a == 5 { "#19866d" } else { "#4e7997" };
        svg.push_str(&format!("<text x=\"30\" y=\"{}\" font-size=\"17\">Coordinate {a}</text><rect x=\"160\" y=\"{y}\" width=\"{width:.2}\" height=\"30\" fill=\"{color}\"/><text x=\"{}\" y=\"{}\" font-size=\"16\">{value} ({:.3}x)</text>",y+21,170.0+width,y+21,value as f64/sums[1][5] as f64));
    }
    svg.push_str("<text x=\"30\" y=\"447\" font-size=\"16\">All 192 candidate root checks matched; no dense fallbacks.</text><text x=\"30\" y=\"477\" font-size=\"14\">Local, unisolated; not runtime, fresh-instance or rho-parity evidence.</text></g></svg>");
    write_new(&out.join("held-out.svg"), svg.as_bytes())?;
    println!("{}", result);
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    require(
        args.len() == 2,
        "usage: ANALYZE ORIGINAL_RUN_ROOT NEW_REPORT_DIRECTORY",
    )?;
    run(Path::new(&args[0]), Path::new(&args[1]))
}
