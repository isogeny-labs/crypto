//! Public-fixture adapter for the existing native r-adding rho reference.
//!
//! This is a preparation tool, not an admitted benchmark protocol. It keeps
//! source/input identity, censored outcomes and public scalar verification.
//! The engine currently exposes aggregate timing rather than separate walk,
//! collision and recovery intervals. No historical Python timing is mixed
//! with this adapter's future native runs.
use std::{env, fs, process, time::Instant};

use crypto_lib::cryptanalysis::{
    prime_fast::{rho_parallel, FastCurve, Pt, RhoConfig},
    residual_walk::is_prime_u64,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Fixture {
    schema: String,
    id: String,
    curve: PublicCurve,
}

#[derive(Deserialize)]
struct PublicCurve {
    p: u64,
    a: u64,
    b: u64,
    r: u64,
    #[serde(rename = "P")]
    generator: [u64; 2],
    #[serde(rename = "Q")]
    target: [u64; 2],
}

fn public_context(bytes: &[u8]) -> Result<(Fixture, FastCurve, Pt, Pt), String> {
    let fixture: Fixture = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let q = &fixture.curve;
    if fixture.schema != "prime-pools/v1"
        || q.p <= 3
        || q.r <= 3
        || q.p >= 1 << 62
        || q.r >= 1 << 62
        || !is_prime_u64(q.p)
        || !is_prime_u64(q.r)
        || q.a >= q.p
        || q.b >= q.p
        || q.generator.iter().chain(q.target.iter()).any(|&x| x >= q.p)
    {
        return Err("invalid public field, order, schema or coordinate".into());
    }
    let curve = FastCurve::new(&fixture.id, q.p, q.a, q.b, q.r);
    let f = curve.f;
    let discriminant = f.add(
        f.mul(f.to_mont(4 % q.p), f.mul(curve.a, f.mul(curve.a, curve.a))),
        f.mul(f.to_mont(27 % q.p), f.mul(curve.b, curve.b)),
    );
    let generator = curve.point(q.generator[0], q.generator[1]);
    let target = curve.point(q.target[0], q.target[1]);
    if discriminant == 0
        || !curve.is_on_curve(generator)
        || !curve.is_on_curve(target)
        || curve.mul(Some(generator), q.r).is_some()
        || curve.mul(Some(target), q.r).is_some()
    {
        return Err("singular curve, off-curve point or wrong subgroup".into());
    }
    Ok((fixture, curve, generator, target))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 6 {
        return Err(
            "usage: trace_incidence_rho FIXTURE SEED TABLE_BITS WALKERS DP_BITS MAX_STEPS".into(),
        );
    }
    if args[0].ends_with(".validation.json") {
        return Err("validation secrets are not solver inputs".into());
    }
    let value = |i: usize| args[i].parse::<u64>().map_err(|e| e.to_string());
    let (seed, table_bits, walkers, dp_bits, max_steps) =
        (value(1)?, value(2)?, value(3)?, value(4)?, value(5)?);
    if table_bits == 0
        || table_bits > 16
        || walkers == 0
        || walkers > 4096
        || dp_bits > 24
        || max_steps == 0
    {
        return Err("parameters exceed this adapter's finite bounds".into());
    }
    let revision = env::var("TRACE_INCIDENCE_SOURCE_REVISION")
        .map_err(|_| "TRACE_INCIDENCE_SOURCE_REVISION must name the built source commit")?;
    if revision.len() != 40 || !revision.bytes().all(|x| x.is_ascii_hexdigit()) {
        return Err("source revision must be a 40-character Git SHA".into());
    }
    let started = Instant::now();
    let bytes = fs::read(&args[0]).map_err(|e| e.to_string())?;
    let (fixture, curve, generator, target) = public_context(&bytes)?;
    let cfg = RhoConfig {
        threads: 1,
        walkers: walkers as usize,
        table_bits: table_bits as u32,
        dp_bits: dp_bits as u32,
        negation: true,
        seed,
        max_steps,
        // Legacy walk, as when this runner was written (fields added later).
        look_ahead: false,
        cycle_window: 0,
        coeff_bits: 0,
        stride_starts: false,
    };
    let answer = rho_parallel(&curve, generator, target, &cfg);
    let verified = answer
        .as_ref()
        .is_some_and(|r| curve.mul(Some(generator), r.log) == Some(target));
    if answer.is_some() && !verified {
        return Err("rho returned a scalar that fails the public group check".into());
    }
    let total = started.elapsed().as_secs_f64();
    println!(
        "{}",
        json!({
            "schema":"trace-incidence-rho-adapter/v1", "fixture_id":fixture.id,
            "source_revision_attestation":revision,
            "source_identity_scope":"caller attestation; external build and binary hashes still required",
            "fixture_path":args[0], "input_bytes":bytes.len(),
            "reference":"crypto_lib::cryptanalysis::prime_fast::rho_parallel",
            "status":if answer.is_some() {"solved"} else {"censored"},
            "recovered_scalar":answer.as_ref().map(|r|r.log),
            "public_scalar_check":verified,
            "total_steps":answer.as_ref().map(|r|r.total_steps),
            "engine_wall_seconds":answer.as_ref().map(|r|r.wall_secs),
            "complete_adapter_wall_seconds":total,
            "threads":1, "negation":true, "seed":seed, "table_bits":table_bits,
            "walkers":walkers, "dp_bits":dp_bits, "max_steps":max_steps,
            "admission":"development adapter; pending L2, A/A, phase instrumentation and independent replay"
        })
    );
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PUBLIC: &[u8] = include_bytes!(
        "../research/trace_incidence_20261116/historical/prime_fixtures/b008_c0_natural_s0.json"
    );

    #[test]
    fn public_fixture_and_known_answer_match() {
        let (_, c, p, q) = public_context(PUBLIC).unwrap();
        // The test-only scalar is not an argument to the adapter or rho.
        assert_eq!(c.mul(Some(p), 3356), Some(q));
    }

    #[test]
    fn off_curve_and_wrong_order_are_rejected() {
        let mut v: serde_json::Value = serde_json::from_slice(PUBLIC).unwrap();
        v["curve"]["Q"][1] = json!(1608);
        assert!(public_context(&serde_json::to_vec(&v).unwrap()).is_err());
        let mut v: serde_json::Value = serde_json::from_slice(PUBLIC).unwrap();
        v["curve"]["r"] = json!(4093);
        assert!(public_context(&serde_json::to_vec(&v).unwrap()).is_err());
    }

    #[test]
    fn rho_reference_recovers_public_toy_target() {
        let (_, c, p, q) = public_context(PUBLIC).unwrap();
        let cfg = RhoConfig {
            threads: 1,
            walkers: 8,
            table_bits: 4,
            dp_bits: 0,
            negation: true,
            seed: 20261116,
            max_steps: 100_000,
        };
        let answer = rho_parallel(&c, p, q, &cfg).expect("bounded toy correctness control");
        assert_eq!(c.mul(Some(p), answer.log), Some(q));
    }
}
