//! `transfer_atlas`: census of the transfer graph reachable from a prime-field
//! elliptic curve, with positive controls for the "down" edges.
//!
//! ```text
//! transfer_atlas run [--json out.json]
//! ```
//!
//! For many bielliptic curves `C/F_p` it tabulates, per node (`E1`, `E2`,
//! `J(C)`) and per edge, whether a *second* transfer or a *descent* to an
//! easier representation exists. The headline questions and their measured
//! answers (`[pcc:unverified]`):
//!
//! - **Transfer again to the other factor `E2`?** Only through `φ2_* φ1^*`,
//!   which is zero — verified on every sample. `E2` carries the `E1`-log only
//!   when `E1 ~ E2` (same order), and same order forces the same trace, so
//!   `E2` is then exactly as hard as `E1`.
//! - **Descend to an anomalous curve (easy DLP)?** An isogeny preserves the
//!   trace, so no edge reaches an anomalous node unless the start is anomalous.
//!   The census counts how often any reachable node is anomalous while `E1` is
//!   not: it does not happen. A positive control runs Smart's additive
//!   transfer on a genuinely anomalous curve to show the easy node is real —
//!   just unreachable by transfer.
//! - **Descend to a binary curve?** No edge here changes the characteristic;
//!   there is no binary node in the reachable set.

use std::process::ExitCode;

use num_bigint::BigInt;

use crypto_lib::cryptanalysis::canonical_lift::{find_anomalous_curve, smart_attack_anomalous};
use crypto_lib::cryptanalysis::chained_transfer::atlas::{
    atlas_report, node_facts, sweep_target, AtlasReport,
};
use crypto_lib::cryptanalysis::chained_transfer::bielliptic::{Bielliptic, Quotient};
use crypto_lib::cryptanalysis::chained_transfer::ec::Curve;
use crypto_lib::cryptanalysis::chained_transfer::field::Fp;

#[derive(Default, serde::Serialize)]
struct Census {
    p: u64,
    curves: u64,
    factors_same_order: u64,
    cross_zero_all: u64,
    e1_anomalous: u64,
    e1_supersingular: u64,
    e2_anomalous: u64,
    e2_supersingular: u64,
    /// Same order but E1/E2 facts differ (would break the "same order ⇒ same
    /// hardness" claim). Expected 0.
    same_order_facts_differ: u64,
    /// E1 not special, yet a reachable node is easier. Expected 0.
    free_easier_node: u64,
    /// Min embedding degree of the E1 largest-prime subgroup seen (pairing
    /// weakness would show as a small number).
    min_embedding_degree: u64,
}

fn survey_p(p: u64, target: u64) -> Census {
    let mut c = Census {
        p,
        min_embedding_degree: u64::MAX,
        ..Default::default()
    };
    for seed in 0..200_000u64 {
        if c.curves >= target {
            break;
        }
        let coeffs = [
            1 + (seed.wrapping_mul(2_654_435_761) >> 3) % (p - 1),
            (seed.wrapping_mul(40_503) >> 5) % p,
            (seed.wrapping_mul(2_246_822_519) >> 7) % p,
            1 + (seed.wrapping_mul(3_266_489_917) >> 9) % (p - 1),
        ];
        let Ok(b) = Bielliptic::new(p, coeffs) else {
            continue;
        };
        let r: AtlasReport = atlas_report(&b, 24);
        c.curves += 1;
        c.factors_same_order += r.factors_same_order as u64;
        c.cross_zero_all += (r.cross_e1_to_e2_zero && r.cross_e2_to_e1_zero) as u64;
        c.e1_anomalous += r.e1.anomalous as u64;
        c.e1_supersingular += r.e1.supersingular as u64;
        c.e2_anomalous += r.e2.anomalous as u64;
        c.e2_supersingular += r.e2.supersingular as u64;
        if r.factors_same_order {
            let same = r.e1.anomalous == r.e2.anomalous
                && r.e1.supersingular == r.e2.supersingular
                && r.e1.largest_prime == r.e2.largest_prime;
            if !same {
                c.same_order_facts_differ += 1;
            }
        }
        let e1_special = r.e1.anomalous || r.e1.supersingular;
        if !e1_special && r.easier_node_reachable {
            c.free_easier_node += 1;
        }
        if let Some(k) = r.e1.embedding_degree {
            c.min_embedding_degree = c.min_embedding_degree.min(k);
        }
    }
    if c.min_embedding_degree == u64::MAX {
        c.min_embedding_degree = 0;
    }
    c
}

/// Positive control: Smart's additive transfer solves the DLP on a genuinely
/// anomalous curve. Returns `(p, recovered == x)`.
fn smart_control() -> Option<(u64, bool)> {
    for p in [43u64, 53, 59, 83, 101, 149, 179, 197] {
        let Some((a, bb)) = find_anomalous_curve(p) else {
            continue;
        };
        let k = Fp::new(p)?;
        let e = Curve::new(k, 0, a, bb)?; // y² = x³ + a x + b
        if e.order() != p {
            continue;
        }
        let gen = e.point_from_seed(7);
        let (gx, gy) = gen?;
        let x = (p / 3).max(2) % p;
        let q = e.mul(&gen, x as u128);
        let (qx, qy) = q?;
        let bi = |v: u64| BigInt::from(v);
        let got = smart_attack_anomalous(
            &bi(p),
            &bi(a),
            &bi(bb),
            &bi(gx.a),
            &bi(gy.a),
            &bi(qx.a),
            &bi(qy.a),
        )
        .ok()?;
        let ok = ((&got % bi(p)) + bi(p)) % bi(p) == bi(x);
        return Some((p, ok));
    }
    None
}

/// Parse `--flag value` as u64.
fn flag_u64(args: &[String], name: &str) -> Option<u64> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
}

/// `sweep` mode: fix one target curve over `F_p` and sweep its cover, reporting
/// the complement-order spectrum. Used by the parallel census workflow.
fn run_sweep(args: &[String]) -> ExitCode {
    let p = flag_u64(args, "--p").unwrap_or(1009);
    let maxc = flag_u64(args, "--maxc").unwrap_or(p);
    // Target cubic coefficients c0,c1,c2,c3 from --target "c0,c1,c2,c3".
    let f: [u64; 4] = args
        .iter()
        .position(|a| a == "--target")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| {
            let v: Vec<u64> = s.split(',').filter_map(|x| x.parse().ok()).collect();
            <[u64; 4]>::try_from(v).ok()
        })
        .unwrap_or([7, 3, 2, 1]);
    match sweep_target(p, f, maxc) {
        Some(rep) => {
            let out = serde_json::to_string(&rep).unwrap();
            if let Some(path) = args
                .iter()
                .position(|a| a == "--json")
                .and_then(|i| args.get(i + 1))
            {
                std::fs::write(path, &out).expect("write json");
            }
            println!("{out}");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("sweep: no valid target cover for p={p}, target={f:?}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "sweep") {
        return run_sweep(&args);
    }
    let json = args
        .iter()
        .position(|a| a == "--json")
        .and_then(|i| args.get(i + 1));

    let censuses: Vec<Census> = [1009u64, 2003, 4001, 8009]
        .iter()
        .map(|&p| survey_p(p, 300))
        .collect();

    println!(
        "{:>6} {:>7} {:>9} {:>10} {:>6} {:>6} {:>6} {:>6} {:>9} {:>9} {:>8}",
        "p",
        "curves",
        "same_ord",
        "cross0_all",
        "E1anom",
        "E1ss",
        "E2anom",
        "E2ss",
        "factsDiff",
        "freeEasy",
        "minEmbK"
    );
    for c in &censuses {
        println!(
            "{:>6} {:>7} {:>9} {:>10} {:>6} {:>6} {:>6} {:>6} {:>9} {:>9} {:>8}",
            c.p,
            c.curves,
            c.factors_same_order,
            c.cross_zero_all,
            c.e1_anomalous,
            c.e1_supersingular,
            c.e2_anomalous,
            c.e2_supersingular,
            c.same_order_facts_differ,
            c.free_easier_node,
            c.min_embedding_degree,
        );
    }

    let ctrl = smart_control();
    match ctrl {
        Some((p, ok)) => println!(
            "\nSmart additive-transfer positive control: anomalous curve over F_{p}, recovered log = {} [pcc:unverified]",
            if ok { "correct" } else { "WRONG" }
        ),
        None => println!("\nSmart control: no small anomalous curve found in the list"),
    }

    println!(
        "\nsame_ord   = #E1 = #E2 (⇒ same trace ⇒ E2 as hard as E1)\n\
         cross0_all = φ2_*φ1^* = 0 AND φ1_*φ2^* = 0 on all samples (second transfer to the other factor is zero)\n\
         factsDiff  = same order but E1/E2 facts differ (expected 0)\n\
         freeEasy   = E1 not special yet a reachable node is easier (expected 0)\n\
         minEmbK    = smallest embedding degree of an E1 subgroup seen (pairing weakness would be a small number)\n\
         All counts [pcc:unverified]. No node changes characteristic: no binary node is reachable."
    );

    if let Some(path) = json {
        #[derive(serde::Serialize)]
        struct Out<'a> {
            censuses: &'a [Census],
            smart_control: Option<(u64, bool)>,
        }
        let out = Out {
            censuses: &censuses,
            smart_control: ctrl,
        };
        std::fs::write(path, serde_json::to_string_pretty(&out).unwrap()).expect("write json");
        eprintln!("wrote {path}");
    }
    // A node-facts sanity line for E1 of the first curve, so the schema is seen.
    let _ = node_facts;
    let _ = Quotient::E1;
    ExitCode::SUCCESS
}
