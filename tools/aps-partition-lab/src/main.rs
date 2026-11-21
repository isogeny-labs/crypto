use aps_partition_lab::{coloring, curve::Curve, labels, signature};
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize)]
struct Report {
    schema: &'static str,
    source_revision: String,
    source_manifest_sha256: String,
    experiment: &'static str,
    family_law: &'static str,
    performance_trial: bool,
    execution_attested: bool,
    signed_standing: &'static str,
    production_aps_sampler_tested: bool,
    known_label_group_control_tested: bool,
    labeled_pool_scope: &'static str,
    f4_trace_reuse_tested: bool,
    maximum_points: usize,
    maximum_colorings: usize,
    coloring: Vec<coloring::Coverage>,
    signatures: Vec<signature::SignatureReport>,
    labeled_control: Vec<labels::LabelReport>,
}

fn hex(s: &str, n: usize) -> bool {
    s.len() == n
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 8
        || args[1] != "structural"
        || args[2] != "--source-revision"
        || args[4] != "--source-manifest-sha256"
        || args[6] != "--out"
        || !hex(&args[3], 40)
        || !hex(&args[5], 64)
    {
        return Err("usage: aps-partition-lab structural --source-revision HEX40 --source-manifest-sha256 HEX64 --out NEW_JSON".into());
    }
    let mut cases = Vec::new();
    // Freeze the entire panel here: no family extension in response to coverage.
    for n in [8, 12] {
        for m in [3, 4] {
            for size in [1, 8, 32] {
                let family = coloring::candidate_family(n, m, size)?;
                cases.push(coloring::audit(n, m, &family)?);
            }
        }
    }
    let mut signatures = Vec::new();
    for (p, a, b) in [(17, 2, 2), (19, 2, 3), (31, 2, 3)] {
        signatures.push(signature::experiment(Curve::new(p, a, b)?)?);
    }
    let report = Report {
        schema: "aps.partition-structural/v1",
        source_revision: args[3].clone(),
        source_manifest_sha256: args[5].clone(),
        experiment: "bounded-structural-correctness-only",
        family_law: "id-mod-arity then splitmix64(id + k*golden-ratio-constant) mod arity; k=1..31",
        performance_trial: false,
        execution_attested: false,
        signed_standing: "unverified",
        production_aps_sampler_tested: false,
        known_label_group_control_tested: true,
        labeled_pool_scope:
            "public-known-answer-order-19-fixture-only; production APS sampler not tested",
        f4_trace_reuse_tested: false,
        maximum_points: coloring::MAX_POINTS,
        maximum_colorings: coloring::MAX_FAMILY,
        coloring: cases,
        signatures,
        labeled_control: vec![labels::experiment(3)?, labels::experiment(4)?],
    };
    let bytes = serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?;
    // Exclusive creation: a later invocation cannot replace an earlier receipt.
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&args[7])
        .map_err(|e| e.to_string())?;
    file.write_all(&bytes)
        .and_then(|()| file.write_all(b"\n"))
        .map_err(|e| e.to_string())?;
    println!("structural report written; no speed or execution qualification");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("aps-partition-lab: {error}");
        std::process::exit(1);
    }
}
