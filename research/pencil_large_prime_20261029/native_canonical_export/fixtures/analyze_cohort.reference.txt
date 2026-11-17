fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: analyze_cohort MANIFEST.json")?;
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let input = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let mut output = pencil_accounting_price_draft::aggregate::aggregate(&input)?;
    if std::env::args().nth(2).as_deref() == Some("--summary") {
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
