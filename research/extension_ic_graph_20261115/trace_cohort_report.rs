//! Native report for exact saved-record counters; no timing or solver calls.
use serde_json::{json, Value};
use std::{fmt::Write as _, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: COHORT_REPORT SUMMARY NEW_REPORT_DIR".into());
    }
    let input: Value = serde_json::from_slice(&fs::read(&args[0])?)?;
    if input["schema"] != "cover.unit-trace.saved-cohort-counters/v1"
        || input["systems"] != 3415
        || input["roots"] != 3419
        || input["every_coordinate_multiset_equal"] != true
        || input["new_solver_calls"] != 0
        || input["new_timing_calls"] != 0
        || input["complete_cost"] != false
    {
        return Err("saved cohort contract".into());
    }
    let arms = input["arms"].as_array().ok_or("arms")?;
    if arms.len() != 3 {
        return Err("all three arms".into());
    }
    let labels = [
        ("unit-untraced", "Unit without trace"),
        ("unit-traced", "Unit with trace"),
        ("six-traced", "Six with trace"),
    ];
    let mut table = String::new();
    let mut md_table = String::new();
    let mut data = Vec::new();
    let mut svg = String::from(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="940" height="250" viewBox="0 0 940 250"><rect width="940" height="250" fill="white"/><g font-family="DejaVu Sans" fill="#17334a"><text x="15" y="24" font-size="18">All 3415 saved systems: reported solver multiplication subtotals</text>"##,
    );
    for (i, ((id, label), row)) in labels.iter().zip(arms).enumerate() {
        if row["arm"] != *id || row["total"]["systems"] != 3415 || row["total"]["roots"] != 3419 {
            return Err("arm cohort identity".into());
        }
        let f: u128 = row["total"]["f4_muls"]
            .as_str()
            .ok_or("F4 count")?
            .parse()?;
        let l: u128 = row["total"]["lin_muls"]
            .as_str()
            .ok_or("linear count")?
            .parse()?;
        let sum = f.checked_add(l).ok_or("sum overflow")?;
        if row["total"]["reported_sum"] != sum.to_string() {
            return Err("subtotal identity".into());
        }
        let y = 55 + i * 44;
        let fw = f as f64 / 3_000_000_000.0 * 570.0;
        let lw = l as f64 / 3_000_000_000.0 * 570.0;
        write!(
            svg,
            r##"<text x="180" y="{}" text-anchor="end" font-size="15">{label}</text><rect x="195" y="{y}" width="{fw:.2}" height="24" fill="#367da5"/><rect x="{:.2}" y="{y}" width="{lw:.2}" height="24" fill="#d98e39"/><text x="{:.2}" y="{}" font-size="14">{sum}</text>"##,
            y + 18,
            195.0 + fw,
            203.0 + fw + lw,
            y + 18
        )?;
        writeln!(table, "[{label}],[{f}],[{l}],[{sum}],")?;
        writeln!(md_table, "| {label} | {f} | {l} | {sum} |")?;
        data.push(json!({"arm":id,"systems":3415,"roots":3419,"f4_muls":f.to_string(),"lin_muls":l.to_string(),"reported_sum":sum.to_string(),"scope":"saved solver counters; learning tracked-RREF and other pipeline phases absent"}));
    }
    svg.push_str(r##"<rect x="195" y="200" width="17" height="13" fill="#367da5"/><text x="220" y="212" font-size="14">reported F4</text><rect x="365" y="200" width="17" height="13" fill="#d98e39"/><text x="390" y="212" font-size="14">reported linear stage</text><text x="15" y="242" font-size="13">Fp multiplication subtotals only · learning tracked-RREF omitted · no whole-cost or timing inference</text></g></svg>"##);
    let md=format!("# Complete saved-cohort counter comparison\n\nAll3415 captured systems and3419 root multisets agree across the original unit and both traced paths. The analysis reads saved outputs only: zero new solver, relation-yield or timing calls. SHA256pins bind the three raw inputs and both full replay summaries. Unsigned PCC-20261123-400f7d. [pcc:unverified]\n\n![Saved solver subtotals](solver-counts.svg)\n\n| Backend | Reported F4 Fp muls | Reported linear Fp muls | Sum of those counters |\n|---|---:|---:|---:|\n{md_table}\nThe full-cohort reported subtotal is lower for traced six than traced five. Reducing the coordinate count alone does not make this retained solver workload cheaper. This compares only the two emitted counters, not complete F4, descent or pipeline cost. The old and traced five linear-stage totals are identical; the six-variable basis has a different linear-stage count. [pcc:unverified]\n\nThe five path has3348replays,66mismatch flags and1recording; six3381/33/1. Every captured root set matches. These fixed controls do not prove a generic completeness guarantee for every future exceptional shape; the source acceptance/fallback audit remains open. [pcc:unverified]\n\nAccounting finding: tracked RREF used to learn a trace lies outside the old field_ops counter. The retained rows lack that event trace, so summing them cannot reconstruct its cost. The separate Nov18descent-finder study gives its own bound; this analysis does not transfer it or reprice historical results. It agrees numerically with that study's six-trace F4/linear subtotals but is not its signed validation. [pcc:unverified]\n\nNext comparison keeps both traced arities and charges trace learning, reduction/lifting, cold collection/graph/kernel work, random generation and verification. Runpod remains unfunded and checkpoint upload unavailable; no paid call started and no parity conclusion follows.\n\nSources: [proof-bound counters](../proof-bound-counter-summary.json), [analysis plan](../PLAN.md), [source review](../SOURCES.md), [native source](../../trace_cohort_summary.rs), [existing descent-finder study](../../leads-20261118/descent-finder/README.md), [prior full root controls](../../trace-integration-20261123/README.md).\n");
    let typ = format!(
        r###"#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: 0.6em)
= Complete saved-cohort solver counters
Rust graph IC continuation · 23 November 2026

All 3415 saved systems and 3419 root multisets agree across the three recorded paths. This analysis performs no new solver, relation search or timing call. All raw inputs and both replay summaries are hash-bound. Producer observation: PCC-20261123-400f7d, *UNVERIFIED*. [pcc:unverified]

#image("solver-counts.svg", width: 100%)
#table(columns:(1.7fr,1fr,1fr,1.1fr),align:(left,right,right,right),[Backend],[Reported F4],[Reported linear],[Reported sum],{table})

== What the comparison changes
The retained six-coordinate trace has the lower *reported subtotal* over the entire cohort. The five-coordinate trace has 3348 replay flags, 66 mismatch flags and one recording; six has 3381, 33 and one. Every captured root set matches. Coordinate elimination alone does not establish a cheaper solver workload, and these counters do not decide the complete-cost winner. [pcc:unverified]

== Missing learning work
The pinned engine's tracked RREF used to learn a trace is outside its field_ops counter. The saved rows do not contain the event trace needed to recover that cost. A separate Nov18 descent-finder study reports its own bound; it is not transferred to these binaries or the five-variable arm. Other missing work includes reduction/lifting, cold collection and graph/kernel phases, native control, RNG, hashing and serialization.

== Correctness and execution limits
The analysis checks the exact same coordinate multisets and explicit status flags on every record; checked u128 sums retain large integer counts. Existing full-equation replay is bound through its saved proofs. Fixed-cohort agreement is not a generic completeness proof for all future exceptional shapes; a focused acceptance/fallback audit remains open.

No complete-cost ratio, wall-time gain or parity is inferred. A funded Runpod comparison must retain both traced arities and account for all learning and cold setup. Provider balance and checkpoint upload remain external blockers; no new paid call occurred.

Sources: proof-bound-counter-summary.json, PLAN.md and SOURCES.md beside this report; native trace_cohort_summary.rs and trace_cohort_report.rs; existing leads-20261118/descent-finder study. #link("https://github.com/isogeny-labs/crypto/pull/4986")[PR 4986] retains the implementation and earlier controls.
"###
    );
    let out = Path::new(&args[1]);
    fs::create_dir(out)?;
    for (file, text) in [
        ("REPORT.md", md),
        ("report.typ", typ),
        ("solver-counts.svg", svg),
    ] {
        fs::write(out.join(file), text)?;
    }
    fs::write(
        out.join("plot-data.json"),
        serde_json::to_vec_pretty(&data)?,
    )?;
    Ok(())
}
