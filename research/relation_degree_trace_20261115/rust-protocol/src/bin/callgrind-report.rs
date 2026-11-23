//! Report raw guest instructions without converting them to wall speedups.
use plotters::prelude::*;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn label(s: &str) -> String {
    let short = if s.chars().count() > 110 {
        format!("{}...", s.chars().take(107).collect::<String>())
    } else {
        s.to_string()
    };
    short
        .replace('|', "\\|")
        .replace('\n', " ")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 {
        return Err("usage: callgrind-report RAW NEW_REPORT RESULT_PCC".into());
    }
    let raw = Path::new(&a[1]);
    let out = Path::new(&a[2]);
    let s: Value = serde_json::from_slice(&fs::read(raw.join("summary.json"))?)?;
    if s["schema"] != "native-callgrind-profile/v1"
        || !s["failure"].is_null()
        || s["identities_unchanged"] != true
        || s["result"]["complete_basis_equals_frozen"] != true
    {
        return Err("incomplete profile".into());
    }
    let profile =
        compact_speed_analysis::callgrind::parse(&fs::read_to_string(raw.join("callgrind.out"))?)?;
    if serde_json::to_value(&profile)? != s["result"]["profile"] {
        return Err("profile recomputation differs".into());
    }
    let log = fs::read_to_string(raw.join("valgrind.log"))?;
    let collected = log
        .lines()
        .find(|l| l.contains("Collected :"))
        .and_then(|l| l.split_whitespace().last())
        .ok_or("Callgrind collected count")?
        .parse::<u64>()?;
    if collected != profile.total_ir {
        return Err("Callgrind log/reader total mismatch".into());
    }
    let qualification = s["qualification"] == true;
    let mut md=format!("# Native candidate instruction diagnosis\n\nQualification only: **{qualification}**. {}. One whole-process guest-instruction profile; generation/preparation and parent capture excluded. No native time/cycle estimate, speed ratio, A/A admission, confidence interval, new target or degree-of-regularity conclusion. [pcc:unverified]\n\nThe complete checked canonical answer equals the frozen uninstrumented candidate. Executable/runtime hashes are unchanged. Total guest Ir: **{}**, across{} attributed functions. Rust recomputation matches both raw summary/totals, every function row and the separately formatted Collected line from the same profiler. This is producer recomputation, not signed independent execution verification. [pcc:unverified]\n\nSelf costs are exclusive and add to total. Inclusive costs include callees, overlap and must not be added across functions. Inlined instructions belong to their containing compiled functions. Startup, libraries, allocation and exact checking remain included; instrumented dispatch can differ from native hardware dispatch. [pcc:unverified]\n\nFunction names longer than110 characters are abbreviated in tables; full names and object paths remain in functions.json.\n\n## Largest self costs\n\n| Rank | Function | Self Ir | Fraction of all Ir | Inclusive Ir (overlaps) |\n|---:|---|---:|---:|---:|\n",a[3],profile.total_ir,profile.functions.len());
    for (i, f) in profile.functions.iter().take(25).enumerate() {
        md += &format!(
            "| {} | {} | {} | {:.3}% | {} |\n",
            i + 1,
            label(&f.name),
            f.self_ir,
            100. * f.self_ir as f64 / profile.total_ir as f64,
            f.inclusive_ir
        );
    }
    let mut native: Vec<_> = profile
        .functions
        .iter()
        .filter(|f| {
            f.name.contains("certified_relations") || f.name.contains("relation_replay_checker")
        })
        .collect();
    native.sort_by_key(|f| std::cmp::Reverse(f.inclusive_ir));
    md+="\n## Native constructor and separate checker: largest inclusive counts\n\n| Function | Self Ir | Inclusive Ir (overlaps) |\n|---|---:|---:|\n";
    for f in native.iter().take(25) {
        md += &format!(
            "| {} | {} | {} |\n",
            label(&f.name),
            f.self_ir,
            f.inclusive_ir
        );
    }
    md+="\nAll functions, object paths, counters, raw call graph and exact answer remain in the raw archive and functions.json. Counts describe this compiled binary/input under Callgrind. They can guide a subsequent explicitly qualified optimization; they do not predict its native gain. [pcc:unverified]\n\n![Largest exclusive guest instruction counts](self.svg){width=95%}\n\nNumbers on the vertical axis map to the self-cost table.\n";
    fs::create_dir(out)?;
    fs::write(out.join("report.md"), md)?;
    fs::write(
        out.join("functions.json"),
        serde_json::to_vec_pretty(&profile)?,
    )?;
    fs::write(
        out.join("audit.json"),
        serde_json::to_vec_pretty(
            &json!({"qualification":qualification,"total_ir":profile.total_ir,"functions":profile.functions.len(),"all_rows_recomputed":true,"raw_totals_and_log_match":true,"scope":"descriptive guest Ir; no speed ratio or independent verdict"}),
        )?,
    )?;
    let path = out.join("self.svg");
    let root = SVGBackend::new(&path, (1200, 850)).into_drawing_area();
    root.fill(&WHITE)?;
    let rows: Vec<_> = profile.functions.iter().take(25).collect();
    let max = rows
        .iter()
        .map(|f| 100. * f.self_ir as f64 / profile.total_ir as f64)
        .fold(1., f64::max)
        * 1.1;
    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Exclusive guest instructions by attributed function",
            ("sans-serif", 23),
        )
        .margin(20)
        .x_label_area_size(55)
        .y_label_area_size(65)
        .build_cartesian_2d(0f64..max, 0f64..rows.len() as f64 + 1.)?;
    chart
        .configure_mesh()
        .disable_y_mesh()
        .x_desc("Percent of total guest Ir; not native wall time")
        .y_desc("Table rank")
        .y_labels(rows.len() + 1)
        .y_label_formatter(&|y| format!("{y:.0}"))
        .draw()?;
    chart.draw_series(rows.iter().enumerate().map(|(i, f)| {
        Rectangle::new(
            [
                (0., i as f64 + 0.65),
                (
                    100. * f.self_ir as f64 / profile.total_ir as f64,
                    i as f64 + 1.35,
                ),
            ],
            BLUE.mix(0.65).filled(),
        )
    }))?;
    root.present()?;
    Ok(())
}
