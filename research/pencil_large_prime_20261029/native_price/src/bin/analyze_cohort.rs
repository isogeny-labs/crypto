fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let path = args.first().ok_or("usage: analyze_cohort MANIFEST.json")?;
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let input = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    if args.iter().skip(1).any(|a| {
        a != "--summary"
            && a != "--p739-gate60"
            && a != "--p186379-target0"
            && a != "--full1260-disjoint-p739"
    }) {
        return Err("unsupported analyze_cohort option".into());
    }
    let selected = [
        "--p739-gate60",
        "--p186379-target0",
        "--full1260-disjoint-p739",
    ]
    .into_iter()
    .filter(|flag| args.iter().any(|a| a == flag))
    .collect::<Vec<_>>();
    if selected.len() > 1 {
        return Err("analysis scope options are mutually exclusive".into());
    }
    let mut output = match selected.first().copied() {
        Some("--p739-gate60") => {
            pencil_accounting_price_draft::aggregate::aggregate_p739_gate60(&input)?
        }
        Some("--p186379-target0") => {
            pencil_accounting_price_draft::aggregate::aggregate_p186379_target0(&input)?
        }
        Some("--full1260-disjoint-p739") => {
            pencil_accounting_price_draft::aggregate::aggregate_disjoint_p739(&input)?
        }
        Some(_) => unreachable!(),
        None => pencil_accounting_price_draft::aggregate::aggregate(&input)?,
    };
    if args.iter().any(|a| a == "--summary") {
        for key in ["pair_rows", "missing_executions", "target_controls"] {
            let count = output[key]
                .as_array()
                .ok_or("invalid aggregate output")?
                .len();
            output[format!("{key}_count")] = serde_json::json!(count);
            output.as_object_mut().unwrap().remove(key);
        }
        output["projection"]=serde_json::json!("summary omits per-slot rows; rerun without --summary for complete rows using same bound inputs");
    }
    println!(
        "{}",
        serde_json::to_string(&output).map_err(|e| e.to_string())?
    );
    Ok(())
}
