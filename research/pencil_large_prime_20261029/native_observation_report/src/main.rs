use pencil_native_observation_report::{build, render};
use std::{fs, path::Path};
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    }
}
fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: observation-report REQUEST NEW_OUTPUT_DIRECTORY".into());
    }
    let r = serde_json::from_slice(&fs::read(&a[1]).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    render(&build(&r)?, Path::new(&a[2]))
}
