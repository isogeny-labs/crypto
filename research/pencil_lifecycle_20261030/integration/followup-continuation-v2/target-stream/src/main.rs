use pencil_symbolic_graph::relation_stream::Header;
use pencil_target_stream_offline::{decode, replay, ReplayLimits};
use std::{fs::File, io::Read};
fn read_bounded(path: &str) -> Result<String, String> {
    const MAX: u64 = 16 * 1024 * 1024;
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(MAX + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() as u64 > MAX {
        return Err("input exceeds 16MiB diagnostic limit".into());
    }
    String::from_utf8(bytes).map_err(|e| e.to_string())
}
fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 4 || a[1] != "--expected-header" {
        return Err(
            "usage: pencil-target-stream-offline --expected-header EXPECTED.json STREAM.json[l]"
                .into(),
        );
    }
    let expected: Header =
        serde_json::from_str(&read_bounded(&a[2])?).map_err(|e| format!("expected header: {e}"))?;
    let records = decode(&read_bounded(&a[3])?)?;
    let report = replay(&records, &expected, ReplayLimits::default())?;
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("target-stream rejected: {e}");
        std::process::exit(2);
    }
}
