use relation_degree_trace::{analyze, System};
use serde_json::{json, Value};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn defaults() -> Result<Vec<System>, serde_json::Error> {
    [
        include_str!("../fixtures/system-2147483647.json"),
        include_str!("../fixtures/system-9007199254740881.json"),
    ]
    .into_iter()
    .map(serde_json::from_str)
    .collect()
}
fn chart(reports: &[Value]) -> String {
    let height = 320 * reports.len() + 45;
    let mut svg = format!("<svg xmlns='http://www.w3.org/2000/svg' width='1120' height='{height}' viewBox='0 0 1120 {height}'><rect width='100%' height='100%' fill='white'/><g font-family='sans-serif' font-size='13' fill='#182433'><text x='20' y='25'>Native relation effects — observed closure threshold; top-form Hilbert function</text>");
    for (ri, r) in reports.iter().enumerate() {
        let y = 55 + 320 * ri;
        svg += &format!("<text x='25' y='{y}'>p = {}</text>", r["prime"]);
        let mut runs: Vec<(String, &Value)> = vec![("base".into(), &r["baseline"])];
        runs.extend(
            r["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| (c["id"].as_str().unwrap().into(), &c["single_addition"])),
        );
        runs.push(("selected".into(), &r["greedy_final"]));
        for (i, (label, run)) in runs.iter().enumerate() {
            let bx = 25 + i * 43;
            let base = y + 195;
            let degree = run["solving_degree"].as_u64();
            let h = degree.unwrap_or(0) * 25;
            svg += &format!("<rect x='{bx}' y='{}' width='30' height='{h}' fill='#567db1'/><text x='{bx}' y='{}'>{label}</text><text x='{bx}' y='{}'>{}</text>",base-h as usize,base+20,base-h as usize-6,degree.map(|d| d.to_string()).unwrap_or_else(|| "cap".into()));
            let hy = y + 20 + i * 22;
            svg += &format!("<text x='540' y='{}'>{label}</text>", hy + 15);
            for entry in run["top_forms"]["trace"].as_array().unwrap() {
                let d = entry["degree"].as_u64().unwrap();
                let h = entry["hilbert_function"].as_u64().unwrap();
                let hx = 610 + d * 48;
                svg += &format!("<rect x='{hx}' y='{hy}' width='46' height='21' fill='{}'/><text x='{}' y='{}'>{h}</text>",if h == 0 {"#e4f2ed"} else {"#d3e1f3"},hx+17,hy+15);
            }
        }
        svg += &format!("<text x='25' y='{}'>Matrix degree (one candidate added per bar)</text><text x='610' y='{}'>Homogeneous degree increases left to right; blank = not traced</text>",y+250,y+285);
    }
    svg + "</g></svg>"
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args().skip(1);
    let mut input = None;
    let mut out = PathBuf::from("out");
    let mut cap = 8;
    let mut limit = 8;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = Some(args.next().ok_or("--input requires a path")?),
            "--out" => out = PathBuf::from(args.next().ok_or("--out requires a path")?),
            "--max-degree" => cap = args.next().ok_or("missing degree")?.parse()?,
            "--candidate-limit" => limit = args.next().ok_or("missing candidate limit")?.parse()?,
            "--help" => {
                println!("relation-degree-trace [--input SYSTEM.json] [--out DIR] [--max-degree 8] [--candidate-limit 8]");
                return Ok(());
            }
            _ => return Err(format!("unknown option {arg}").into()),
        }
    }
    let systems = if let Some(path) = input {
        vec![serde_json::from_str::<System>(&fs::read_to_string(path)?)?]
    } else {
        defaults()?
    };
    if out.exists() {
        return Err(format!(
            "output {} already exists; use a fresh directory to preserve prior runs",
            out.display()
        )
        .into());
    }
    let mut reports = Vec::new();
    for system in systems {
        let report = analyze(&system, cap, limit)?;
        println!(
            "p={}: baseline={}, selected={}, downstream={}, discovery-inclusive peak={}",
            system.prime,
            report["baseline"]["solving_degree"],
            report["greedy_selected"],
            report["greedy_final"]["solving_degree"],
            report["end_to_end_peak_matrix_degree"]
        );
        reports.push(report);
    }
    fs::create_dir_all(&out)?;
    fs::write(
        out.join("trace.json"),
        serde_json::to_string_pretty(&json!({"reports":reports}))? + "\n",
    )?;
    let mut csv =
        String::from("prime,scenario,observed_closure_degree,top_form_d_reg,rows_processed\n");
    let mut degree_csv = String::from("prime,scenario,degree,closure_completed,columns,rows,nonzeros,rank,new_pivots,degree_falls,certificate_product_degree\n");
    for report in &reports {
        let mut runs = vec![
            ("baseline".to_string(), &report["baseline"]),
            ("selected".to_string(), &report["greedy_final"]),
            ("all".to_string(), &report["all_candidates"]),
        ];
        runs.extend(
            report["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .map(|c| (c["id"].as_str().unwrap().to_string(), &c["single_addition"])),
        );
        runs.extend(report["leave_one_out"].as_array().unwrap().iter().map(|c| {
            (
                format!("remove_{}", c["removed"].as_str().unwrap()),
                &c["solve"],
            )
        }));
        for (label, run) in runs {
            let number = |v: &Value| v.as_u64().map(|x| x.to_string()).unwrap_or_default();
            let rows: u64 = run["trace"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["new_rows_processed"].as_u64().unwrap())
                .sum();
            csv += &format!(
                "{},{label},{},{},{rows}\n",
                report["prime"],
                number(&run["solving_degree"]),
                number(&run["top_forms"]["top_form_d_reg"])
            );
            for t in run["trace"].as_array().unwrap() {
                degree_csv += &format!(
                    "{},{label},{},{},{},{},{},{},{},{},{}\n",
                    report["prime"],
                    t["degree"],
                    t["closure_completed"],
                    t["columns"],
                    t["new_rows_processed"],
                    t["new_row_nonzeros"],
                    t["rank"],
                    t["new_pivots"],
                    t["degree_falls"],
                    t["max_new_certificate_product_degree"]
                );
            }
        }
    }
    fs::write(out.join("relation-effects.csv"), csv)?;
    fs::write(out.join("degrees.csv"), degree_csv)?;
    fs::write(out.join("relation-effects.svg"), chart(&reports))?;
    Ok(())
}
