//! Impact analysis for `docs/ecbench/CLASS_SETUP_AUDIT.md`: rewrite a
//! **scratch copy** of a pre-2026-10-18 `rho.quartic` / `rho.cubic`
//! session so each record's `total_gae`, `S` and ratio to the floor carry
//! the class set-up its phases already hold (total = phase sum), then
//! reseal the records and rehash the file so `ecbench compare` reads it.
//!
//! The copy is analysis input, never evidence: it is not the session the
//! binary wrote, and committed sessions are never edited.  Usage:
//!
//! ```text
//! cp -r <session> /tmp/copy && cargo run --release --example \
//!     ecbench_class_setup_corrected_copy -- /tmp/copy
//! ecbench compare --dir /tmp/copy --a rho-neg --b rho-quartic --json
//! ```
use crypto_lib::cryptanalysis::ecbench::canonical::sha256_hex;
use crypto_lib::cryptanalysis::ecbench::record::Record;
use crypto_lib::cryptanalysis::ecbench::runner::Session;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = std::path::Path::new(a.get(1).expect("usage: <session copy>"));
    assert!(
        !out.join("session.json")
            .canonicalize()
            .unwrap()
            .starts_with(env!("CARGO_MANIFEST_DIR")),
        "refusing to rewrite a session inside the repository; copy it first"
    );
    let text = std::fs::read_to_string(out.join("records.jsonl")).unwrap();
    let mut lines = Vec::new();
    for l in text.lines() {
        let mut rec: Record = serde_json::from_str(l).unwrap();
        if let Some(t) = rec.cost.total_gae {
            let sum: f64 = rec.phases.iter().map(|p| p.gae).sum();
            if sum != t {
                let floor = rec.cost.s.unwrap() / rec.boundaries.ratio_to_floor.unwrap();
                let s = sum / (rec.workload.curve.r as f64).sqrt();
                rec.cost.total_gae = Some(sum);
                rec.cost.s = Some(s);
                rec.boundaries.ratio_to_floor = Some(s / floor);
            }
        }
        lines.push(rec.seal());
    }
    let body = lines.join("\n") + "\n";
    std::fs::write(out.join("records.jsonl"), &body).unwrap();
    let mut session: Session =
        serde_json::from_str(&std::fs::read_to_string(out.join("session.json")).unwrap()).unwrap();
    session.records_sha256 = Some(sha256_hex(body.as_bytes()));
    std::fs::write(
        out.join("session.json"),
        serde_json::to_string_pretty(&session).unwrap() + "\n",
    )
    .unwrap();
}
