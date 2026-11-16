use signed_collector_replay::{parse_expected_header, replay_jsonl, Limits};
use std::{fs::File, io::Read};
fn bounded(path: &str, limit: usize) -> Result<Vec<u8>, String> {
    let mut b = vec![];
    File::open(path)
        .map_err(|e| e.to_string())?
        .take(limit as u64 + 1)
        .read_to_end(&mut b)
        .map_err(|e| e.to_string())?;
    if b.len() > limit {
        return Err("file size limit".into());
    }
    Ok(b)
}
fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: signed-collector-replay EXPECTED_HEADER.json STREAM.jsonl".into());
    }
    let h = parse_expected_header(&bounded(&a[1], 65536)?)?;
    let limits = Limits::default();
    let r = replay_jsonl(&bounded(&a[2], limits.bytes)?, &h, limits)?;
    println!("{}", serde_json::to_string(&r).map_err(|e| e.to_string())?);
    Ok(())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
