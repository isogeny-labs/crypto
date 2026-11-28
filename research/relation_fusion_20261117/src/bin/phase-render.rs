//! Read-only presentation of recorded integers. No timing ratios or selection.
use serde_json::Value;
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn read(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage:phase-render STUDY NEW_OUT".into());
    }
    let study = Path::new(&args[1]);
    let out = Path::new(&args[2]);
    fs::create_dir(out)?;
    let phase = read(&study.join("PHASE_SUMMARY_71.json"))?;
    if phase["all_correct"] != true
        || phase["planned"] != 18
        || phase["ratios_or_admission_computed"] != false
    {
        return Err("complete descriptive phase roster required".into());
    }
    let rows = phase["outcomes"]
        .as_array()
        .filter(|v| v.len() == 18)
        .ok_or("18outcomes")?;
    let max = rows
        .iter()
        .map(|r| r["parent_ns"].as_u64().ok_or("parent"))
        .collect::<std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .ok_or("maximum")?;
    let mut svg=String::from("<svg xmlns='http://www.w3.org/2000/svg' width='1350' height='1060' viewBox='0 0 1350 1060'><rect width='1350' height='1060' fill='white'/><g font-family='sans-serif'><text x='24' y='32' font-size='23'>Fusion phase71: all18 one-call observations (n=1 per cell)</text><text x='24' y='60' font-size='16'>Descriptive milliseconds; no ratios, confidence intervals or performance admission. Outliers retained.</text><text x='24' y='84' font-size='15'>Blue parent; orange construction+wire; green independent replay. L=legacy kernel, F=frontier103; V=vector, P=packed.</text>");
    let x0 = 300.;
    let width = 990.;
    for tick in 0..=6 {
        let v = max as f64 * tick as f64 / 6.;
        let x = x0 + width * tick as f64 / 6.;
        svg+=&format!("<line x1='{x}' x2='{x}' y1='112' y2='1002' stroke='#ddd'/><text x='{x}' y='1025' text-anchor='middle' font-size='14'>{:.2}</text>",v/1e6);
    }
    for (i, row) in rows.iter().enumerate() {
        let y = 120. + i as f64 * 48.;
        let p = row["prime"].as_u64().ok_or("prime")?;
        let n = row["n"].as_u64().ok_or("n")?;
        let label = format!(
            "{} n{} {}/{}",
            if p == 2147483647 { "p31" } else { "p53" },
            n,
            if row["kernel"] == "legacy" { "L" } else { "F" },
            if row["checker"] == "legacy" { "V" } else { "P" }
        );
        svg += &format!("<text x='20' y='{}' font-size='16'>{label}</text>", y + 15.);
        for (j, key, color) in [
            (0, "parent_ns", "#2365a7"),
            (1, "construct_ns", "#e28a2d"),
            (2, "replay_ns", "#298b61"),
        ] {
            let value = row[key].as_u64().ok_or("clock")?;
            let length = width * value as f64 / max as f64;
            svg += &format!(
                "<rect x='{x0}' y='{}' width='{length}' height='9' fill='{color}'/>",
                y + j as f64 * 11.
            );
        }
    }
    svg+="<text x='760' y='1050' text-anchor='middle' font-size='16'>Milliseconds; raw integers:PHASE_SUMMARY_71.json; unsigned producer evidence</text></g></svg>";
    fs::write(out.join("phases.svg"), svg)?;
    let mut md=String::from("# Complete-certificate work counts\n\nEach entry is a correctness-case receipt, not a timing sample or speedup. All whole-proof obligations remain. Versions1/2 represent the same complete reduced grevlex basis; larger DAG node count is prospectively bounded. [pcc:unverified]\n\n|Prime|n|Constructor|Definitions|Cofactors|Referenced term products|\n|---|---|---|---:|---:|---:|\n");
    let frontier = read(&study.join("QUALIFICATION.json"))?;
    let flat = read(&study.join("CUBIC_QUALIFICATION_72.json"))?;
    let dag = read(&study.join("DAG_QUALIFICATION_74.json"))?;
    let pruned = read(&study.join("PRUNED_QUALIFICATION_75.json"))?;
    for p in [2147483647u64, 9007199254740881] {
        for n in [3, 4, 6] {
            for (label, cases) in [
                ("frontier103", &frontier["correctness_corpus"]["cases"]),
                ("flat cubics", &flat["cases"]),
                ("shared DAG", &dag["cases"]),
                ("pruned DAG", &pruned["cases"]),
            ] {
                let r = cases
                    .as_array()
                    .ok_or("cases")?
                    .iter()
                    .find(|v| {
                        v["prime"] == p
                            && v["n"] == n
                            && (label != "frontier103" || v["kernel"] == "frontier")
                    })
                    .ok_or("fixed cell")?;
                let s = &r["stats"];
                md += &format!(
                    "|{p}|{n}|{label}|{}|{}|{}|\n",
                    s["definitions"], s["cofactor_terms"], s["referenced_term_products"]
                );
            }
        }
    }
    fs::write(out.join("certificate-work.md"), md)?;
    Ok(())
}
