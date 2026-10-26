//! Measurement driver for the symmetry-broken `m = 2` oracle and the
//! double-large-prime graph on the two 131-bit Certicom curves and their
//! 61–127-bit same-family calibration curves.
//!
//! Everything is measured with one field implementation on one host, in
//! one process, so the index-calculus step cost and the Pollard-rho step
//! cost compare directly (AGENTS.md §5a: the walk step is rho's reference
//! unit).  Records are JSONL; `certicom131_symlp_report` lays them out.
//!
//! ```sh
//! cargo run --release --example certicom131_symlp_measure -- \
//!     research/certicom131_symlp_20261025/runs
//! ```
//!
//! Three record kinds, all tagged `kind`:
//! * `rate` — fixed step budget of each oracle variant on a curve: steps,
//!   in-domain pairs, confirmed decompositions, seconds.  The symmetric
//!   and baseline rows on the same curve give the symmetry-breaking cost
//!   per distinct pair.
//! * `yield` — a full sweep of the subspace `V_{l'}` on a calibration
//!   curve small enough to finish, over random targets: the mean
//!   decompositions per sweep and the 0/1/2-large-prime split against a
//!   factor base `V_l`, the yield the graph consumes.
//! * `rho` — the batched adding-walk step rate on the same field at full
//!   size: the reference step cost.

use std::fs;
use std::io::Write as _;
use std::path::PathBuf;
use std::time::Instant;

use num_bigint::BigUint;
use rand::{rngs::StdRng, SeedableRng};

use crypto_lib::cryptanalysis::certicom131_symlp::curves::{BinaryW, Pt, PrimeW, SymCurve};
use crypto_lib::cryptanalysis::certicom131_symlp::sym2::{BaselineOracle, Decomp, SymmetricOracle};
use crypto_lib::cryptanalysis::certicom131_symlp::sym2p::{PDecomp, PrimeBaseline, PrimeSymmetric};
use crypto_lib::cryptanalysis::certicom131_symlp::gf2w;

fn rec(out: &mut fs::File, v: serde_json::Value) {
    writeln!(out, "{v}").unwrap();
}

/// Rate of one binary oracle variant: sweep `2^k` values, time it.
fn binary_rate(
    out: &mut fs::File,
    c: &BinaryW,
    label: &str,
    lp_dim: u32,
    k: u32,
    symmetric: bool,
    rng: &mut StdRng,
) {
    let r = c.random_point(rng);
    let base = gf2w::ZERO;
    let t0 = Instant::now();
    let counts = if symmetric {
        let mut n = 0u64;
        let o = SymmetricOracle::new(c, lp_dim);
        let cs = o.sweep(&r, base, k, |_: Decomp| n += 1);
        let _ = n;
        cs
    } else {
        let o = BaselineOracle::new(c, lp_dim);
        o.sweep(&r, base, k, |_: Decomp| {})
    };
    let secs = t0.elapsed().as_secs_f64();
    rec(
        out,
        serde_json::json!({
            "kind": "rate", "family": "binary", "curve": c.name(),
            "n": c.field_bits(), "variant": label, "symmetric": symmetric,
            "lp_dim": lp_dim, "k": k, "steps": counts.steps,
            "trace_pass": counts.trace_pass, "in_domain": counts.in_domain,
            "decomps": counts.decomps, "seconds": secs,
            "steps_per_sec": counts.steps as f64 / secs,
        }),
    );
}

fn prime_rate(
    out: &mut fs::File,
    c: &PrimeW,
    label: &str,
    bound_bits: u32,
    count: u64,
    symmetric: bool,
    rng: &mut StdRng,
) {
    let r = c.random_point(rng);
    let bound = BigUint::from(1u8) << bound_bits;
    let start = BigUint::from(0u8);
    let t0 = Instant::now();
    let counts = if symmetric {
        PrimeSymmetric::new(c, &bound).sweep(&r, &start, count, |_: PDecomp| {})
    } else {
        PrimeBaseline::new(c, &bound).sweep(&r, &start, count, |_: PDecomp| {})
    };
    let secs = t0.elapsed().as_secs_f64();
    rec(
        out,
        serde_json::json!({
            "kind": "rate", "family": "prime", "curve": c.name(),
            "n": c.field_bits(), "variant": label, "symmetric": symmetric,
            "bound_bits": bound_bits, "count": count, "steps": counts.steps,
            "trace_pass": counts.trace_pass, "in_domain": counts.in_domain,
            "decomps": counts.decomps, "seconds": secs,
            "steps_per_sec": counts.steps as f64 / secs,
        }),
    );
}

/// Full-subspace yield on a binary calibration curve: sweep every `e_1 ∈
/// V_{l'}` for many targets, count decompositions and their large-prime
/// split against `V_l`.
fn binary_yield(out: &mut fs::File, n: u32, l: u32, lp_dim: u32, targets: u32, rng: &mut StdRng) {
    let c = BinaryW::random(n, rng);
    let o = SymmetricOracle::new(&c, lp_dim);
    let (mut d0, mut d1, mut d2, mut total) = (0u64, 0u64, 0u64, 0u64);
    for _ in 0..targets {
        let r = c.random_point(rng);
        o.sweep(&r, gf2w::ZERO, lp_dim, |d: Decomp| {
            match d.large_primes(l) {
                0 => d0 += 1,
                1 => d1 += 1,
                _ => d2 += 1,
            }
            total += 1;
        });
    }
    let sweep = 1u64 << lp_dim;
    rec(
        out,
        serde_json::json!({
            "kind": "yield", "family": "binary", "n": n, "l": l, "lp_dim": lp_dim,
            "targets": targets, "sweep_steps": sweep,
            "decomps": total, "lp0": d0, "lp1": d1, "lp2": d2,
            "decomps_per_sweep": total as f64 / targets as f64,
            "decomp_prob_per_step": total as f64 / (targets as u64 * sweep) as f64,
        }),
    );
}

/// Batched adding-walk step rate at full size: the rho reference cost.
fn rho_rate<C: SymCurve>(out: &mut fs::File, c: &C, family: &str, lanes: usize, rounds: u64, rng: &mut StdRng) {
    let mut acc: Vec<Pt<C::E>> = (0..lanes).map(|_| c.random_point(rng)).collect();
    let jumps: Vec<Pt<C::E>> = (0..lanes).map(|_| c.random_point(rng)).collect();
    // Warm up, then time.
    c.add_batch(&mut acc, &jumps);
    let t0 = Instant::now();
    let mut skipped = 0u64;
    for _ in 0..rounds {
        skipped += c.add_batch(&mut acc, &jumps) as u64;
    }
    let secs = t0.elapsed().as_secs_f64();
    let steps = rounds * lanes as u64;
    rec(
        out,
        serde_json::json!({
            "kind": "rho", "family": family, "curve": c.name(),
            "n": c.field_bits(), "lanes": lanes, "rounds": rounds,
            "steps": steps, "skipped": skipped, "seconds": secs,
            "steps_per_sec": steps as f64 / secs,
        }),
    );
}

fn main() {
    let dir = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "research/certicom131_symlp_20261025/runs".into()),
    );
    fs::create_dir_all(&dir).unwrap();
    let mut rng = StdRng::seed_from_u64(20261025);

    // ── Rho reference, full size ──────────────────────────────────────
    let mut rho = fs::File::create(dir.join("rho.jsonl")).unwrap();
    let (b131, _, _) = BinaryW::ecc2_131();
    let (p131, _, _) = PrimeW::eccp_131();
    rho_rate(&mut rho, &b131, "binary", 1024, 20_000, &mut rng);
    rho_rate(&mut rho, &p131, "prime", 1024, 20_000, &mut rng);

    // ── Oracle step rate, full size (both variants) ───────────────────
    let mut rate = fs::File::create(dir.join("rate.jsonl")).unwrap();
    // 2^22 steps is a few seconds per variant and enough for a stable rate.
    binary_rate(&mut rate, &b131, "ecc2-131", 40, 22, false, &mut rng);
    binary_rate(&mut rate, &b131, "ecc2-131", 40, 22, true, &mut rng);
    prime_rate(&mut rate, &p131, "eccp-131", 40, 1 << 21, false, &mut rng);
    prime_rate(&mut rate, &p131, "eccp-131", 40, 1 << 21, true, &mut rng);
    // Calibration-curve rates (same instrument, smaller fields).
    for n in [61u32, 71, 83, 97, 113, 127] {
        let bc = BinaryW::random(n, &mut rng);
        binary_rate(&mut rate, &bc, "cal", 40, 22, false, &mut rng);
        binary_rate(&mut rate, &bc, "cal", 40, 22, true, &mut rng);
        let pc = PrimeW::random(n, &mut rng);
        prime_rate(&mut rate, &pc, "cal", 40, 1 << 21, false, &mut rng);
        prime_rate(&mut rate, &pc, "cal", 40, 1 << 21, true, &mut rng);
    }

    // ── Yield, calibration curves with a full subspace sweep ──────────
    let mut yld = fs::File::create(dir.join("yield.jsonl")).unwrap();
    // l ≈ n/3 is the balancing factor-base dimension; l' a few bits above
    // it opens the large-prime domain.  Keep lp_dim ≤ 24 so 2^{lp_dim}
    // targets finish in seconds.
    for &(n, l, lp) in &[
        (19u32, 6u32, 18u32),
        (23, 8, 20),
        (29, 10, 22),
        (31, 10, 24),
    ] {
        let targets = if lp >= 23 { 16 } else { 64 };
        binary_yield(&mut yld, n, l, lp, targets, &mut rng);
    }

    eprintln!("wrote records to {}", dir.display());
}
