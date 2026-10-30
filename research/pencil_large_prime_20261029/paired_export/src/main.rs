fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 8 && !(a.len() == 10 && a[8] == "--doc") {
        return Err("usage: pencil-paired-export RECORDS.jsonl CANDIDATE REFERENCE AA_FIRST AA_SECOND OUT.json OUT.csv [--doc PLAN.json]".into());
    }
    let input = std::fs::read_to_string(&a[1])?;
    let labels = [&a[2][..], &a[3][..], &a[4][..], &a[5][..]];
    let bundle = if a.len() == 10 {
        pencil_paired_export::export_with_plan(&input, labels, &std::fs::read_to_string(&a[9])?)?
    } else {
        pencil_paired_export::export(&input, labels)?
    };
    // Refuse overwriting the input or any preexisting output.
    use std::io::Write;
    let mut json = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[6])?;
    let mut csv = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[7])?;
    json.write_all(serde_json::to_string_pretty(&bundle)?.as_bytes())?;
    csv.write_all(pencil_paired_export::csv(&bundle).as_bytes())?;
    Ok(())
}
