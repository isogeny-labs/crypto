//! Native report of retained Run51 observations; no scientific worker.
use serde_json::{json, Value};
use std::{fmt::Write as _, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn label(s: &str) -> Result<&'static str> {
    Ok(match s {
        "original-ic" => "Original IC",
        "quadratic-table" => "Quadratic table",
        "pair-discriminant" => "Screened grounded graph",
        "pair-deferred" => "Deferred graph",
        "pair-adapted-six" => "Six-variable adapter",
        "pair-unit-five" => "Five-variable unit",
        "rho-plain-b1" => "Rho plain / batch 1",
        "rho-neg-b1" => "Rho negation / batch 1",
        "rho-plain-b16" => "Rho plain / batch 16",
        "rho-neg-b16" => "Rho negation / batch 16",
        _ => return Err("unknown original arm".into()),
    })
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 2 {
        return Err("usage: REPORT OBSERVATION_JSON NEW_REPORT_DIR".into());
    }
    let v: Value = serde_json::from_slice(&fs::read(&a[0])?)?;
    if v["original_qualification_exit"] != 137
        || v["original_run_qualified"] != false
        || v["post_execution_replay_exit"] != "0"
        || v["rho_parity"] != false
    {
        return Err("preserved failure/replay scope".into());
    }
    let rows = v["outcomes"].as_array().ok_or("outcomes")?;
    if rows.len() != 10 {
        return Err("ten retained arms".into());
    }
    let source = rows[0]["source_revision"].as_str().ok_or("source")?;
    if rows.iter().any(|r| r["source_revision"] != source) {
        return Err("source identity".into());
    }
    let out = Path::new(&a[1]);
    fs::create_dir(out)?;
    let mut md=String::from("# Run51: deferred graph IC with exact Nagao elimination\n\nAll ten workers completed. The frozen Runpod qualifier was killed with137; a separate unchanged-binary replay on the local16GiB machine exits0. Original137is retained. The replay checks3415unit inputs and3419lifted roots, plus all source/graph/rank/affine/elliptic equations. Producer evidence remains UNVERIFIED.\n\n| Arm | Observed incomplete bitops | Process seconds |\n|---|---:|---:|\n");
    let mut table = String::new();
    let mut plot = vec![];
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="415" viewBox="0 0 1000 415"><rect width="1000" height="415" fill="white"/><g font-family="DejaVu Sans" fill="#17334a"><text x="20" y="23" font-size="17">Run51 complete worker-process wall observations · N=1 per arm</text>"##,
    );
    for tick in [0, 30, 60, 90, 120, 150] {
        let x = 225.0 + tick as f64 / 150.0 * 650.0;
        write!(
            svg,
            r##"<line x1="{x:.1}" x2="{x:.1}" y1="38" y2="348" stroke="#e3e8ed"/><text x="{x:.1}" y="372" text-anchor="middle" font-size="13">{tick}</text>"##
        )?;
    }
    for (i, r) in rows.iter().enumerate() {
        let role = r["mode"].as_str().ok_or("role")?;
        let name = label(role)?;
        let count = r["observed_incomplete_bitops"]
            .as_str()
            .ok_or("diagnostic subtotal")?;
        let _: u128 = count.parse()?;
        let wall = r["process_wall_seconds_practicality_only"]
            .as_f64()
            .ok_or("process wall")?;
        if !wall.is_finite() || !(0.0..150.0).contains(&wall) {
            return Err("plot range".into());
        }
        writeln!(md, "| {name} | {count} | {wall:.3} |")?;
        writeln!(table, "[{name}],[{count}],[{wall:.3}],")?;
        let y = 48 + i * 30;
        let width = wall / 150.0 * 650.0;
        let color = if role.starts_with("rho-") {
            "#cc7633"
        } else {
            "#367da5"
        };
        write!(
            svg,
            r##"<text x="214" y="{}" text-anchor="end" font-size="14">{name}</text><rect x="225" y="{y}" width="{width:.2}" height="17" fill="{color}"/><text x="{:.2}" y="{}" font-size="13">{wall:.3}</text>"##,
            y + 14,
            233.0 + width,
            y + 14
        )?;
        plot.push(json!({"arm":role,"process_seconds":wall,"n":1,"interval":null,"scope":"nonadmitted fixed-order elapsed observation"}));
    }
    svg.push_str(r##"<text x="550" y="394" text-anchor="middle" font-size="14">seconds · no interval estimated · exposed fixed target · timing not admitted</text></g></svg>"##);
    md.push_str("\nThe unit candidate has a lower recorded diagnostic subtotal and a higher observed process wall than its matched six-variable adapter. One fixed-order exposed target does not establish a runtime improvement or parity. The plot has no uncertainty interval because none was recorded. [pcc:unverified]\n\nFlow: planned10, attempted10, completed workers10, excluded0, automatic retries0. Original qualifier137; separate mathematical replay0. No paid scientific worker was repeated.\n\nSource f5fdb2d04c2efdc89381d74ecae9da9bcea2d511 was published before the sole Runpod dispatch. CPU5c,4vCPU/8GiB,0.14USD/hour; first allowed CPU; cold processes; four rho configurations. Runner deleted204/independent404after the106077823Boriginal archive passed hash/privacy checks.\n\nComplete-cost ratios remain null. Counters omit native containers/control, some RNG/fixed-factor/hash/serialization work. No repeated/isolation/fresh-target/scaling or globally optimal rho admission. Current-root full build is not asserted: the scoped binary explicitly links the pinned Root41library. Primary evidence: ../README.md, ../run-observation.json, ../analysis-original.exit-code.txt, ../post-execution-replay-original-binary.json, ../original-raw.sha256. Large originals remain explicitly LOCAL.\n\nThe original unsigned branch entry4b5c69is archived; canonical PCC-20261123-9551e9 remains UNVERIFIED. Joux–Vitse §§3.1–3.3 and the existing exact unit reduction supply the method. Next investigate setup and F4 learning/replay with exact captured-input controls before a new Runpod schedule. Canonical scoreboards/frontier graphs remain unchanged because no new admitted primary value exists.\n");
    let typ = format!(
        r###"#set page(paper:"a4",margin:(x:14mm,y:12mm))
#set text(font:"DejaVu Sans",size:8.3pt)
#set par(leading:0.4em)
= Run51: graph IC with exact algebraic descent
*Run 17 November2026 · report 23 November2026 · native Rust source/data*

Ten workers completed. Original frozen Runpod qualifier137is preserved; separate unchanged-binary mathematical replay0checks all ten outputs,3415unit inputs and3419lifted roots. Original admission remains failed. Canonical PCC-20261123-9551e9is UNVERIFIED. [pcc:unverified]

#image("wall.svg",width:100%)

#table(columns:(1.8fr,1.45fr,0.9fr),inset:3pt,stroke:0.35pt,
[*Arm*],[*Incomplete bitop subtotal*],[*Process seconds*],
{table})

The unit candidate has fewer recorded operations and a higher observed process wall than its matched six-variable adapter. These are N=1fixed-order observations on an exposed target; no uncertainty interval was recorded. Neither timing improvement nor primary parity is admitted. [pcc:unverified]

*Flow and execution.* Planned10/attempted10/completed workers10/excluded0/retries0. Same p1009, subgroup263807419614139553, seed389, public41fixture, binary/host/first allowed CPU; cold processes. Runpod CPU5c4vCPU/8GiB,0.14USD/hour;7200second guarded lifetime. Source #raw("{source}") published before sole dispatch. Original archive106077823Bcollected/hash/privacy-checked; DELETE204and independent GET404at12:44:39Z. [pcc:unverified]

*Scope.* Complete native totals/ratios remain null; containers/control, some sampling/fixed-factor/hash/serialization work remain unpriced. No repeated/isolation/fresh-target/growing-size or global rho-optimum admission. The original qualifier's kill cause is not proved from137alone. The scoped binary uses its declared Root41library; a current-main full build is not asserted. [pcc:unverified]

*Evidence and next step.* The run-observation.json, original failed status/task stderr, separate full replay, raw hashes and deletion receipt sit beside this report. Large originals stay hash-bound LOCAL at /workspace/ic-runpod-runs/runpod-unit-deferred-20261117-51. Native source: unit_deferred_run_study.rs and unit_deferred_run_report.rs; Joux–Vitse §§3.1–3.3; existing unit component. Test reduced aggregation and existing F4trace reuse on captured inputs before a new Runpod comparison. Primary scoreboards remain unchanged. [pcc:unverified]
"###
    );
    fs::write(out.join("REPORT.md"), md)?;
    fs::write(out.join("wall.svg"), svg)?;
    fs::write(
        out.join("plot-data.json"),
        serde_json::to_vec_pretty(&plot)?,
    )?;
    fs::write(out.join("report.typ"), typ)?;
    Ok(())
}
