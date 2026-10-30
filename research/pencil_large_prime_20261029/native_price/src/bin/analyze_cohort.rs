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
    if args
        .iter()
        .skip(1)
        .any(|a| a != "--summary" && a != "--p739-gate60")
    {
        return Err("unsupported analyze_cohort option".into());
    }
    let mut output = if args.iter().any(|a| a == "--p739-gate60") {
        pencil_accounting_price_draft::aggregate::aggregate_p739_gate60(&input)?
    } else {
        pencil_accounting_price_draft::aggregate::aggregate(&input)?
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
