//! `verify_certs`: re-check every `chained.solve.certificate/v1` record in a
//! JSON-lines file from the curve data alone, using only
//! `chained_transfer::certify` (fresh double-and-add; no solver code).
//!
//! ```text
//! verify_certs FILE.jsonl
//! ```
//!
//! Prints one line per certificate and a summary. Exit status is 0 only when
//! every certificate is present, parses and verifies; a certificate whose
//! solver returned no log counts as not verified. `[pcc:unverified]`

use std::process::ExitCode;

use crypto_lib::cryptanalysis::chained_transfer::certify::{verify, Cert};

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: verify_certs FILE.jsonl");
        return ExitCode::from(2);
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("cannot read {path}: {e}");
            return ExitCode::from(2);
        }
    };
    let (mut issued, mut verified, mut failed, mut no_claim, mut unparsed) = (0u64, 0, 0, 0, 0);
    let mut by_arm: std::collections::BTreeMap<String, (u64, u64)> = Default::default();
    for (i, line) in text
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
    {
        issued += 1;
        let cert: Cert = match serde_json::from_str(line) {
            Ok(c) => c,
            Err(e) => {
                unparsed += 1;
                println!("line {:>3}: UNPARSEABLE: {e}", i + 1);
                continue;
            }
        };
        let v = verify(&cert);
        let e = by_arm.entry(cert.arm.clone()).or_default();
        e.0 += 1;
        if v.ok {
            verified += 1;
            e.1 += 1;
            println!(
                "line {:>3}: ok     arm={:<12} instance={:<8} p={:<5} r={:<5} k={:?}",
                i + 1,
                cert.arm,
                cert.instance_type,
                cert.p,
                cert.r,
                cert.claimed_k
            );
        } else {
            if v.no_claim {
                no_claim += 1;
            } else {
                failed += 1;
            }
            println!(
                "line {:>3}: FAILED arm={:<12} p={:<5} r={:<5}: {}",
                i + 1,
                cert.arm,
                cert.p,
                cert.r,
                v.failures.join("; ")
            );
        }
    }
    println!("\nfile: {path}");
    for (arm, (n, ok)) in &by_arm {
        println!("arm {arm:<12} issued {n:>3} verified {ok:>3}");
    }
    println!(
        "issued {issued}, verified {verified}, failed {failed}, no_claim {no_claim}, unparseable {unparsed}"
    );
    if issued > 0 && verified == issued {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
