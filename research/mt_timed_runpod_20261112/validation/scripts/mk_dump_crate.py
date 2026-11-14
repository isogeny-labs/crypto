#!/usr/bin/env python3
"""VALIDATOR dump build of examples/mt_fieldops.rs (frozen source), used only for the counter replay cross-check and the
independent EC check of recovered logs, never for timing.

Writes a standalone crate <dir> with src/main.rs = the frozen example plus one block after the shared verification
pass: when MTFO_DUMP=<path> is set it writes {"xs": planted logs, "qx","qy": target points, "found": recovered logs}
as JSON to <path>.  No counted operation is added and nothing before the output line changes.  Dependencies pinned to
the repository's Cargo.lock versions (rand 0.8.8 with small_rng, serde_json 1.0.151 with arbitrary_precision and
raw_value); the cargo feature mt-fasthash is declared so both builds can be made.
usage: mk_dump_crate.py <frozen example .rs> <repository Cargo.lock> <dir>"""
import os
import sys

SRC, LOCK, DIR = sys.argv[1:4]
src = open(SRC).read()
anchor = "    let phases: Vec<String> = snap"
assert src.count(anchor) == 1, "anchor not unique"
dump = r'''    if let Ok(path) = std::env::var("MTFO_DUMP") {
        let j = |v: &Vec<String>| format!("[{}]", v.join(","));
        let s = format!(
            "{{\"xs\":{},\"qx\":{},\"qy\":{},\"found\":{}}}",
            j(&xs.iter().map(|v| v.to_string()).collect()),
            j(&qs.iter().map(|q| q.map(|t| t.0.to_string()).unwrap_or("null".into())).collect()),
            j(&qs.iter().map(|q| q.map(|t| t.1.to_string()).unwrap_or("null".into())).collect()),
            j(&found.iter().map(|v| v.to_string()).collect())
        );
        std::fs::write(path, s).unwrap();
    }
'''
os.makedirs(os.path.join(DIR, "src"), exist_ok=True)
open(os.path.join(DIR, "src/main.rs"), "w").write(src.replace(anchor, dump + anchor))
open(os.path.join(DIR, "Cargo.toml"), "w").write('''[package]
name = "mtfo_dump"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = { version = "=0.8.8", features = ["small_rng"] }
serde_json = { version = "=1.0.151", features = ["arbitrary_precision", "raw_value"] }

[features]
mt-fasthash = []

[profile.release]
opt-level = 3
''')
open(os.path.join(DIR, "Cargo.lock"), "w").write(open(LOCK).read())
print("wrote", DIR)
