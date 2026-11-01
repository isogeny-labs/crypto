//! Index of the curve registry for `ecbench` children.
//!
//! `docs/curves/registry.json` is 20 MB and every measured `ecbench`
//! execution is a fresh process that needs one entry of it, so
//! `cryptanalysis::ecbench::workload::registry_entry` cuts the entry out
//! of the text instead of parsing the document.  Finding where to cut by
//! scanning the text for the slug cost 37 million instructions for an
//! entry deep in the document (2026-11-01, Callgrind, a 32-bit child).
//! This script writes `registry_index.rs` into `OUT_DIR`: every
//! `"slug": "<slug>"` pair of the document, its first occurrence in
//! document order, sorted by slug, with the byte offset of the value's
//! opening quote, as `static REGISTRY_INDEX: &[(&str, usize)]`.  The
//! runtime still checks the entry it cuts out carries the slug and falls
//! back to the scan and the full parse, so the index changes no answer.
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let registry = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/curves/registry.json");
    println!("cargo:rerun-if-changed={}", registry.display());
    let text = fs::read_to_string(&registry).expect("docs/curves/registry.json is readable");
    let key = "\"slug\"";
    let mut pairs: Vec<(&str, usize)> = Vec::new();
    let mut from = 0;
    while let Some(pos) = text[from..].find(key) {
        let after = from + pos + key.len();
        from = after;
        let Some(rest) = text[after..].trim_start().strip_prefix(':') else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(value) = rest.strip_prefix('"') else {
            continue;
        };
        let Some(end) = value.find('"') else {
            continue;
        };
        let slug = &value[..end];
        if slug.contains('\\') {
            continue;
        }
        pairs.push((slug, text.len() - rest.len()));
    }
    pairs.sort_by(|a, b| a.0.cmp(b.0).then(a.1.cmp(&b.1)));
    pairs.dedup_by(|later, first| later.0 == first.0);
    let mut src = String::from("static REGISTRY_INDEX: &[(&str, usize)] = &[\n");
    for (slug, at) in &pairs {
        src.push_str(&format!("    ({slug:?}, {at}),\n"));
    }
    src.push_str("];\n");
    let out = Path::new(&env::var("OUT_DIR").expect("OUT_DIR")).join("registry_index.rs");
    fs::write(&out, src).expect("registry_index.rs is written");
}
