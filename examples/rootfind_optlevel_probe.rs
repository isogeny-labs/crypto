//! Optlevel probe for prime rootfind kernels (engineering, not a speed claim).
//!
//! Timed region is the **probe** only (prepare excluded). Emits one
//! `optlevel.ops/v1` line whose `ops` are Montgomery field multiplications
//! charged on the probe path (`GroupOps.field_ops.muls + sqrs`), class
//! `modmuladd_p62`. `[pcc:unverified]`.
//!
//! ```text
//! cargo run --release --example rootfind_optlevel_probe -- \
//!   --arm kmitm:m=4 --mode ifma --bits 32 --size 32 --targets 4096 --seconds 1.5
//! ```

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, CountedGroup, GroupOps, OracleCounters, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::RootfindSubtractOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_kmitm::RootfindKMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let arm = get("--arm", "kmitm:m=4");
    let mode = get("--mode", "single"); // single | lanes | ifma
    let bits: u32 = get("--bits", "32").parse().unwrap();
    let size: usize = get("--size", "32").parse().unwrap();
    let n_targets: usize = get("--targets", "4096").parse().unwrap();
    let min_secs: f64 = get("--seconds", "1.5").parse().unwrap();

    let inst = prime_instance_for(bits, 59297);
    let fb = prime_factor_base(&inst, size);
    let g = &inst.curve;
    let ctx = InstanceCtx {
        group: g,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let a = fb.abscissa_list.len();

    let mut state = 0xC0FF_EE42_u64;
    let mut scratch = GroupOps::default();
    let targets: Vec<PrimePoint> = (0..n_targets)
        .map(|_| {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            g.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            )
        })
        .collect();

    let (_, params) = Params::parse_spec(&arm).unwrap_or_else(|e| panic!("bad --arm {arm}: {e}"));
    eprintln!(
        "rootfind_optlevel_probe arm={arm} mode={mode} bits={bits} |Fx|={a} targets={n_targets} min_secs={min_secs} [pcc:unverified]"
    );

    let (ops, kernel_seconds, probes, ifma) = if arm.starts_with("rootfind-kmitm")
        || arm.starts_with("kmitm:")
    {
        let spec = if arm.starts_with("kmitm:") {
            format!("rootfind-{arm}")
        } else {
            arm.clone()
        };
        let (_, params) = Params::parse_spec(&spec).unwrap();
        let m: u32 = params
            .0
            .get("m")
            .and_then(|s| s.parse().ok())
            .unwrap_or(4);
        let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
        oracle
            .prepare(&ctx, &fb, &params, &mut GroupOps::default())
            .unwrap_or_else(|e| panic!("prepare: {e}"));
        let ifma = oracle.ifma_prepared();
        match mode.as_str() {
            "ifma" => {
                if !ifma {
                    eprintln!("IFMA not prepared on this host/p; falling back to scalar lanes");
                }
                oracle.set_ifma_enabled(true);
            }
            "lanes" => oracle.set_ifma_enabled(false),
            _ => {}
        }
        // Warm.
        warm_kmitm(&mut oracle, &ctx, &fb, &targets, &mode);
        run_kmitm(&mut oracle, &ctx, &fb, &targets, &mode, min_secs, ifma)
    } else if arm.starts_with("rootfind-floor") || arm.starts_with("floor:") {
        let spec = if arm.starts_with("floor:") {
            format!("rootfind-{arm}")
        } else {
            arm.clone()
        };
        let (_, params) = Params::parse_spec(&spec).unwrap();
        let m: u32 = params
            .0
            .get("m")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);
        let mut oracle = RootfindFloorOracle::new(m, &inst);
        oracle
            .prepare(&ctx, &fb, &params, &mut GroupOps::default())
            .unwrap_or_else(|e| panic!("prepare: {e}"));
        warm_single(&mut oracle, &ctx, &fb, &targets);
        run_single(&mut oracle, &ctx, &fb, &targets, min_secs)
    } else if arm.starts_with("rootfind-subtract") || arm.starts_with("subtract:") {
        let spec = if arm.starts_with("subtract:") {
            format!("rootfind-{arm}")
        } else {
            arm.clone()
        };
        let (_, params) = Params::parse_spec(&spec).unwrap();
        let m: u32 = params
            .0
            .get("m")
            .and_then(|s| s.parse().ok())
            .unwrap_or(3);
        let mut oracle = RootfindSubtractOracle::new(m, &inst);
        oracle
            .prepare(&ctx, &fb, &params, &mut GroupOps::default())
            .unwrap_or_else(|e| panic!("prepare: {e}"));
        warm_single(&mut oracle, &ctx, &fb, &targets);
        run_single(&mut oracle, &ctx, &fb, &targets, min_secs)
    } else {
        panic!("unsupported --arm {arm} (want kmitm:/floor:/subtract: or full ids)");
    };

    let _ = params;
    let bytes_declared = 0.0; // INTENSITY_UNKNOWN unless we declare traffic
    let line = json!({
        "schema": "optlevel.ops/v1",
        "ops": ops,
        "op_class": "modmuladd_p62",
        "kernel_seconds": kernel_seconds,
        "bytes_declared": bytes_declared,
        "working_set_bytes": 0,
        "arm": arm,
        "mode": mode,
        "bits": bits,
        "abscissae": a,
        "targets": n_targets,
        "probes": probes,
        "ifma_prepared": ifma,
        "note": "ops = charged Montgomery muls+sqrs on probe path; prepare excluded",
        "pcc": "unverified",
    });
    println!("{line}");
}

fn warm_kmitm(
    oracle: &mut RootfindKMitmOracle<'_>,
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    targets: &[PrimePoint],
    mode: &str,
) {
    let n = targets.len().min(16);
    match mode {
        "lanes" | "ifma" => {
            let mut out = vec![None; n.min(8)];
            for chunk in targets[..n].chunks(8) {
                let mut o = GroupOps::default();
                let mut c = OracleCounters::default();
                oracle.decompose_lanes(ctx, fb, &mut o, &mut c, chunk, &mut out[..chunk.len()]);
                black_box(&out);
            }
        }
        _ => {
            for &t in &targets[..n] {
                let mut o = GroupOps::default();
                let mut c = OracleCounters::default();
                black_box(oracle.decompose(ctx, fb, &mut o, &mut c, t));
            }
        }
    }
}

fn run_kmitm(
    oracle: &mut RootfindKMitmOracle<'_>,
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    targets: &[PrimePoint],
    mode: &str,
    min_secs: f64,
    ifma: bool,
) -> (u64, f64, u64, bool) {
    let mut total_ops = 0u64;
    let mut probes = 0u64;
    let t0 = Instant::now();
    loop {
        match mode {
            "lanes" | "ifma" => {
                let mut out = vec![None; 8];
                for chunk in targets.chunks(8) {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    oracle.decompose_lanes(
                        ctx,
                        fb,
                        &mut o,
                        &mut c,
                        chunk,
                        &mut out[..chunk.len()],
                    );
                    total_ops += o.field_ops.muls + o.field_ops.sqrs;
                    probes += chunk.len() as u64;
                    black_box(&out);
                }
            }
            _ => {
                for &t in targets {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    black_box(oracle.decompose(ctx, fb, &mut o, &mut c, t));
                    total_ops += o.field_ops.muls + o.field_ops.sqrs;
                    probes += 1;
                }
            }
        }
        if t0.elapsed().as_secs_f64() >= min_secs {
            break;
        }
    }
    (total_ops, t0.elapsed().as_secs_f64(), probes, ifma)
}

fn warm_single<O: DecompositionOracle<crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>>(
    oracle: &mut O,
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    targets: &[PrimePoint],
) {
    for &t in targets.iter().take(8) {
        let mut o = GroupOps::default();
        let mut c = OracleCounters::default();
        black_box(oracle.decompose(ctx, fb, &mut o, &mut c, t));
    }
}

fn run_single<O: DecompositionOracle<crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>>(
    oracle: &mut O,
    ctx: &InstanceCtx<'_, crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    targets: &[PrimePoint],
    min_secs: f64,
) -> (u64, f64, u64, bool) {
    let mut total_ops = 0u64;
    let mut probes = 0u64;
    let t0 = Instant::now();
    loop {
        for &t in targets {
            let mut o = GroupOps::default();
            let mut c = OracleCounters::default();
            black_box(oracle.decompose(ctx, fb, &mut o, &mut c, t));
            total_ops += o.field_ops.muls + o.field_ops.sqrs;
            probes += 1;
        }
        if t0.elapsed().as_secs_f64() >= min_secs {
            break;
        }
    }
    (total_ops, t0.elapsed().as_secs_f64(), probes, false)
}
