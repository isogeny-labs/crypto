fn main() {
    if let Err(e) = run() {
        eprintln!("complexity-catalog: {e}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let c = complexity_catalog::catalog()?;
    let query = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if query == "--check" {
        println!("{} version-pinned papers; complete frozen inventory; formal proofs not checked locally",c.papers.len());
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "snapshot": c.retrieved_at, "query": query, "papers": c.search(&query)
            }))?
        );
    }
    Ok(())
}
