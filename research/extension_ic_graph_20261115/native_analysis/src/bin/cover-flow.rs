//! Presentation-only flow export; original raw and native summary stay intact.
use serde_json::{json, Value};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: cover-flow ORIGINAL-RAW.jsonl NEW-FLOW.jsonl".into());
    }
    let mut out = BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&a[2])?,
    );
    for (i, line) in BufReader::new(File::open(&a[1])?).lines().enumerate() {
        let line = line?;
        let v: Value = serde_json::from_str(&line)?;
        let mut row = json!({
            "schema":"cover.flow-excerpt/v1", "original_line":i+1,
            "original_line_blake3_excluding_newline":blake3::hash(line.as_bytes()).to_hex().to_string(),
            "complete_raw_record":false
        });
        for key in [
            "record_type",
            "p",
            "seed",
            "source_revision",
            "input_digest",
            "phase",
            "round",
            "arm",
            "valid",
            "child_exit",
            "progress_path",
            "cold_process_wall_ns",
        ] {
            if let Some(value) = v.get(key) {
                row[key] = value.clone();
            }
        }
        if let Some(w) = v.get("worker") {
            for key in [
                "recovered_scalar",
                "verified",
                "capped",
                "solver_answer_placeholder",
            ] {
                if let Some(value) = w.get(key) {
                    row[key] = value.clone();
                }
            }
        }
        serde_json::to_writer(&mut out, &row)?;
        writeln!(out)?;
    }
    out.flush()?;
    Ok(())
}
