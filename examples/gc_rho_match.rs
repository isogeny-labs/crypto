//! `gc_rho_match`: matched-rho reference and t_GA for the Gröbner solver
//! shootout, amendment-1 prerequisite PA-2
//! (`research/groebner_solver_shootout_20261116`, driver `run_local.py`).
//!
//! One JSON request on stdin:
//! `{"a": 1, "n": 17, "dlps": [[d, walk_seed], ...], "tga_batches": 7,
//!   "tga_adds": 1000000}`.
//! For every `(d, walk_seed)` it builds `Q = [d]G` on `KoblitzCurve::new(a, n)`
//! and runs the repository's signed-Frobenius rho
//! (`koblitz_signed_frobenius_rho_with_progress`, default options except the
//! seed), recording the exact operation ledger (walk group additions, the
//! unpriced partition hashes, canonicalisations, Frobenius maps, negations
//! examined), the setup additions, whether `[d']G == Q` verified with
//! `d' == d`, and the process CPU spent on that DLP (CLOCK_PROCESS_CPUTIME_ID
//! around the call). t_GA is the best of `tga_batches` batches of `tga_adds`
//! sequential single `FastCurve::add` calls, CPU seconds per addition.
//! The defining polynomial of the curve's field is reported (low terms and
//! the integer encoding) so the driver can check it against z^17 + z^3 + 1.
//!
//! One JSON answer on stdout. Nothing here is performance evidence by itself.

use std::io::Read;

use crypto_lib::cryptanalysis::koblitz_fast::FastCurve;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    koblitz_signed_frobenius_rho_with_progress, rho_expected_steps, KoblitzCurve,
    KoblitzSignedRhoOptions,
};
use num_bigint::BigUint;
use serde_json::{json, Value};

fn cpu_seconds() -> f64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: valid pointer to a timespec on the stack.
    unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts) };
    ts.tv_sec as f64 + ts.tv_nsec as f64 * 1e-9
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("stdin");
    let req: Value = serde_json::from_str(&input).expect("json request");
    let a = req["a"].as_u64().unwrap_or(1) as u8;
    let n = req["n"].as_u64().unwrap_or(17) as u32;
    let kc = match KoblitzCurve::new(a, n) {
        Some(kc) => kc,
        None => {
            println!(
                "{}",
                json!({"status": "error", "reason": "KoblitzCurve::new refused"})
            );
            std::process::exit(2);
        }
    };
    let fc = FastCurve::new(&kc.curve).expect("fast curve");
    let r_u64 = kc.subgroup_order.to_u64_digits()[0];
    let low: Vec<u32> = kc.curve.irreducible.low_terms.clone();
    let modulus_int: u64 =
        (1u64 << kc.curve.irreducible.degree) | low.iter().fold(0u64, |acc, &t| acc | (1u64 << t));

    // t_GA: sequential single additions, best of the batches.
    let batches = req["tga_batches"].as_u64().unwrap_or(7);
    let adds = req["tga_adds"].as_u64().unwrap_or(1_000_000);
    let g = fc.lift(kc.generator());
    let mut tga = Vec::new();
    for _ in 0..batches {
        let mut acc = g;
        let t0 = cpu_seconds();
        for _ in 0..adds {
            acc = fc.add(acc, g);
        }
        let dt = cpu_seconds() - t0;
        std::hint::black_box(acc);
        tga.push(dt / adds as f64);
    }
    let tga_best = tga.iter().cloned().fold(f64::INFINITY, f64::min);

    let mut rows = Vec::new();
    for item in req["dlps"].as_array().map(|v| v.as_slice()).unwrap_or(&[]) {
        let d = item[0].as_u64().expect("d");
        let seed = item[1].as_u64().expect("seed");
        let q = kc.mul(kc.generator(), &BigUint::from(d));
        let opts = KoblitzSignedRhoOptions {
            seed,
            ..Default::default()
        };
        let c0 = cpu_seconds();
        let rep = koblitz_signed_frobenius_rho_with_progress(&kc, &q, &opts, &mut |_| {});
        let cpu = cpu_seconds() - c0;
        let ch = &rep.charges;
        rows.push(json!({
            "d": d,
            "walk_seed": seed,
            "verified": rep.verified,
            "log_matches": rep.recovered_log == Some(BigUint::from(d)),
            "exhausted": rep.exhausted,
            "iterations": rep.iterations,
            "restarts_attempted": rep.restarts_attempted,
            "parallel_walks": rep.parallel_walks,
            "walk_group_additions": ch.walk_group_additions,
            "setup_group_additions": ch.setup_group_additions,
            "setup_scalar_multiplications": ch.setup_scalar_multiplications,
            "verification_scalar_multiplications": ch.candidate_verification_scalar_multiplications,
            "partition_hashes": ch.partition_hashes,
            "canonicalizations": ch.canonicalizations,
            "frobenius_maps": ch.frobenius_maps,
            "negations_examined": ch.negations_examined,
            "collisions": ch.collisions,
            "failed_collisions": ch.failed_collisions,
            "fruitless_cycles": ch.fruitless_cycles,
            "cpu_s": cpu,
            "walk_ns": rep.walk_ns as u64,
            "setup_ns": rep.setup_ns as u64,
            "verification_ns": rep.verification_ns as u64,
        }));
    }
    println!(
        "{}",
        json!({
            "status": "ok",
            "a": a,
            "n": n,
            "group_order": kc.group_order.to_string(),
            "subgroup_order": r_u64,
            "cofactor": kc.cofactor.to_string(),
            "modulus_low_terms": low,
            "modulus_int": modulus_int,
            "expected_steps": rho_expected_steps(r_u64, n),
            "tga_seconds_best": tga_best,
            "tga_seconds_batches": tga,
            "tga_adds_per_batch": adds,
            "tga_method": "best of batches of sequential single FastCurve::add, CLOCK_PROCESS_CPUTIME_ID",
            "dlps": rows,
        })
    );
}
