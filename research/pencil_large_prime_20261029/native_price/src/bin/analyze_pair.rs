fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: analyze_pair INPUT.json (or --mapping-sha256)")?;
    if path == "--mapping" {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &pencil_accounting_price_draft::analyzer::mapping_contract()
            )
            .map_err(|e| e.to_string())?
        );
        return Ok(());
    }
    if path == "--mapping-sha256" {
        println!(
            "{}",
            pencil_accounting_price_draft::analyzer::mapping_sha256()
        );
        return Ok(());
    }
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let input = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let output = pencil_accounting_price_draft::analyzer::analyze(&input)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&output).map_err(|e| e.to_string())?
    );
    Ok(())
}
