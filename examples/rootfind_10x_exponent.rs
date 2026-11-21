//! Aim: ~10× further e2e wall vs `rootfind-subtract:m=3`, plus an honest
//! growth-slope measurement (G3-style: slope of log₂(work) vs log₂(|Fₓ|)
//! against the pair-table floor of 1). Classification follows the numbers.
//! `[pcc:unverified]`. No security / sub-rho claim.
//!
//!     cargo run --release --example rootfind_10x_exponent -- \
//!         --bits 32 --sizes 32,64 --multi 8 --repeats 5
//!
//! Pinned 32-bit recipes (from the best-|F| grid): `kmitm:m=5` @ |F|≈32 and
//! `kmitm:m=4` @ |F|≈64 vs `subtract:m=3`.

use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, Calibration, CountedGroup, GroupOps, IncrementalGauss,
    OracleCounters, PrimePoint, RowStatus,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{PrimeAbscissaBase, RootfindSubtractOracle};
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_kmitm::RootfindKMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, InstanceCtx, Params, Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};
use serde_json::json;

fn list_u32(s: &str) -> Vec<u32> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}
fn list_usize(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}
fn median(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = xs.len();
    if n == 0 {
        0.0
    } else if n % 2 == 1 {
        xs[n / 2]
    } else {
        0.5 * (xs[n / 2 - 1] + xs[n / 2])
    }
}
fn linreg_log2(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sxx = 0.0;
    let mut sxy = 0.0;
    let mut k = 0.0;
    for (&x, &y) in xs.iter().zip(ys) {
        if x <= 0.0 || y <= 0.0 {
            continue;
        }
        let lx = x.log2();
        let ly = y.log2();
        sx += lx;
        sy += ly;
        sxx += lx * lx;
        sxy += lx * ly;
        k += 1.0;
    }
    if k < 3.0 {
        return None;
    }
    let den = k * sxx - sx * sx;
    if den.abs() < 1e-12 {
        return None;
    }
    let slope = (k * sxy - sx * sy) / den;
    let intercept = (sy - slope * sx) / k;
    Some((slope, intercept))
}

fn cold_pipeline(
    spec: &str,
    bits: u32,
    size: usize,
    seed: u64,
    planted: u64,
) -> Result<(f64, f64, f64, f64, bool), String> {
    let inst = prime_instance_for(bits, seed);
    let mut ops = GroupOps::default();
    let g = inst.generator_point();
    let q = inst.curve.mul(&mut ops, g, planted);
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: g,
        target: q,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let (name, params) = Params::parse_spec(spec)?;
    let mut pspec = PipelineSpec {
        factor_base: "prime-abscissa".into(),
        oracle: name.clone(),
        oracle_params: params,
        targets: Targets::Walk,
        max_trials: 2_000_000,
        seed,
        ..Default::default()
    };
    pspec.factor_base_params.set("size", size.to_string());
    let m = pspec.oracle_params.u64_or("m", 2)? as u32;
    let base = PrimeAbscissaBase { instance: &inst };
    let t0 = Instant::now();
    let report = match name.as_str() {
        "rootfind-subtract" => {
            let mut o = RootfindSubtractOracle::new(m, &inst);
            run_pipeline(
                &ctx,
                &pspec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        "rootfind-floor" => {
            let mut o = RootfindFloorOracle::new(m, &inst);
            run_pipeline(
                &ctx,
                &pspec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        "rootfind-kmitm" => {
            let mut o = RootfindKMitmOracle::new_prime(m, &inst);
            run_pipeline(
                &ctx,
                &pspec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        other => return Err(format!("unsupported {other}")),
    };
    Ok((
        t0.elapsed().as_secs_f64(),
        report.decomposition.setup.wall_ns as f64 / 1e9,
        report.decomposition.cost.wall_ns as f64 / 1e9,
        report.s,
        report.verified && report.recovered == Some(planted),
    ))
}

/// Threaded walk-collect for a prepared kmitm oracle (PLAN lever 4).
fn collect_kmitm_threaded(
    oracle: &RootfindKMitmOracle<'_>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    threads: usize,
    seed: u64,
    planted: u64,
) -> Result<(f64, f64, bool), String> {
    let wall0 = Instant::now();
    let mut ops = GroupOps::default();
    let g = inst.generator_point();
    let q = inst.curve.mul(&mut ops, g, planted);
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: g,
        target: q,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let mut matrix = IncrementalGauss::new(fb.columns + 1, ctx.r);
    let mut state = seed ^ 0xC0FF_EE00_u64;
    let mut targets_tried = 0u64;
    let mut decomp_wall_s = 0.0f64;
    let batch = threads.max(1) * 8;
    let max_trials = 2_000_000u64;
    let h = ctx.cofactor % ctx.r;
    'outer: while targets_tried < max_trials {
        let n = batch.min((max_trials - targets_tried) as usize);
        let mut pts = Vec::with_capacity(n);
        let mut metas = Vec::with_capacity(n);
        for _ in 0..n {
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let a = 1 + state % (ctx.r - 1);
            state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
            let b = 1 + state % (ctx.r - 1);
            let ag = ctx.group.mul(&mut ops, ctx.generator, a);
            let bq = ctx.group.mul(&mut ops, ctx.target, b);
            pts.push(ctx.group.add(&mut ops, ag, bq));
            metas.push((a, b));
        }
        let t0 = Instant::now();
        let hits = oracle.decompose_threaded(&ctx, fb, &pts, threads);
        decomp_wall_s += t0.elapsed().as_secs_f64();
        targets_tried += n as u64;
        for (i, hit) in hits.into_iter().enumerate() {
            let Some(idxs) = hit else { continue };
            let mut row = vec![0u64; fb.columns + 1];
            for &pi in &idxs {
                let col = fb.col_of[pi];
                let coef = fb.coef_of[pi];
                row[col] = (row[col] + coef) % ctx.r;
            }
            let (a, b) = metas[i];
            row[fb.columns] = (ctx.r - (h * b % ctx.r)) % ctx.r;
            let rhs = (h * a) % ctx.r;
            match matrix.add_row(row, rhs) {
                RowStatus::Independent => {
                    if matrix.pinned(fb.columns).is_some() {
                        break 'outer;
                    }
                }
                RowStatus::Dependent => {}
                RowStatus::Inconsistent => return Err("inconsistent relation".into()),
            }
        }
    }
    let d = matrix
        .pinned(fb.columns)
        .ok_or_else(|| "exhausted without pinning".to_string())?;
    let verified = ctx.group.mul(&mut ops, ctx.generator, d) == ctx.target;
    Ok((wall0.elapsed().as_secs_f64(), decomp_wall_s, verified))
}

/// Walk-collect + Gauss + verify using a caller-prepared oracle.
fn collect_with_oracle<O: DecompositionOracle<crypto_lib::cryptanalysis::ic_boundary::PrimeCurve>>(
    oracle: &mut O,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<PrimePoint>,
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    seed: u64,
    planted: u64,
) -> Result<(f64, f64, bool), String> {
    let wall0 = Instant::now();
    let mut ops = GroupOps::default();
    let g = inst.generator_point();
    let q = inst.curve.mul(&mut ops, g, planted);
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: g,
        target: q,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let mut matrix = IncrementalGauss::new(fb.columns + 1, ctx.r);
    let mut state = seed ^ 0xC0FF_EE00_u64;
    let mut targets_tried = 0u64;
    let mut decomp_wall_s = 0.0f64;
    let h = ctx.cofactor % ctx.r;
    let max_trials = 2_000_000u64;
    while targets_tried < max_trials {
        state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
        let a = 1 + state % (ctx.r - 1);
        state = state.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(1);
        let b = 1 + state % (ctx.r - 1);
        let ag = ctx.group.mul(&mut ops, ctx.generator, a);
        let bq = ctx.group.mul(&mut ops, ctx.target, b);
        let point = ctx.group.add(&mut ops, ag, bq);
        let mut o = GroupOps::default();
        let mut c = OracleCounters::default();
        let t0 = Instant::now();
        let hit = oracle.decompose(&ctx, fb, &mut o, &mut c, point);
        decomp_wall_s += t0.elapsed().as_secs_f64();
        targets_tried += 1;
        let Some(idxs) = hit else { continue };
        let mut row = vec![0u64; fb.columns + 1];
        for &pi in &idxs {
            let col = fb.col_of[pi];
            let coef = fb.coef_of[pi];
            row[col] = (row[col] + coef) % ctx.r;
        }
        row[fb.columns] = (ctx.r - (h * b % ctx.r)) % ctx.r;
        let rhs = (h * a) % ctx.r;
        match matrix.add_row(row, rhs) {
            RowStatus::Independent => {
                if matrix.pinned(fb.columns).is_some() {
                    break;
                }
            }
            RowStatus::Dependent => {}
            RowStatus::Inconsistent => return Err("inconsistent relation".into()),
        }
    }
    let d = matrix
        .pinned(fb.columns)
        .ok_or_else(|| "exhausted without pinning target column".to_string())?;
    let check = ctx.group.mul(&mut ops, ctx.generator, d);
    let verified = check == ctx.target;
    Ok((wall0.elapsed().as_secs_f64(), decomp_wall_s, verified))
}

fn shared_multi(
    bits: u32,
    size: usize,
    spec: &str,
    multi: usize,
    seed0: u64,
) -> Result<serde_json::Value, String> {
    let inst = prime_instance_for(bits, seed0);
    let fb = prime_factor_base(&inst, size);
    let (name, params) = Params::parse_spec(spec)?;
    let m = params.u64_or("m", 3)? as u32;
    let g = inst.generator_point();
    let prep_ctx = InstanceCtx {
        group: &inst.curve,
        generator: g,
        target: g,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let t_prep = Instant::now();
    let mut ok = 0usize;
    let mut collect_sum = 0.0;
    let mut decomp_sum = 0.0;
    match name.as_str() {
        "rootfind-floor" => {
            let mut oracle = RootfindFloorOracle::new(m, &inst);
            oracle.prepare(&prep_ctx, &fb, &params, &mut GroupOps::default())?;
            let prep_s = t_prep.elapsed().as_secs_f64();
            let t_all = Instant::now();
            for i in 0..multi {
                let seed = seed0.wrapping_add(20_000 + i as u64 * 0x9E37);
                let planted = 1 + (seed % (inst.r - 1));
                let (wall, decomp, verified) =
                    collect_with_oracle(&mut oracle, &fb, &inst, seed, planted)?;
                collect_sum += wall;
                decomp_sum += decomp;
                if verified {
                    ok += 1;
                }
            }
            let total = t_all.elapsed().as_secs_f64();
            Ok(json!({
                "arm": spec, "mode": "shared_prepare_multi", "bits": bits, "size": size,
                "multi": multi, "prep_s": prep_s, "collect_sum_s": collect_sum,
                "decomp_sum_s": decomp_sum,
                "per_dlp_collect_s": collect_sum / multi as f64,
                "per_dlp_with_amort_prep_s": collect_sum / multi as f64 + prep_s / multi as f64,
                "wall_clock_s": total, "verified": ok,
            }))
        }
        "rootfind-kmitm" => {
            let mut oracle = RootfindKMitmOracle::new_prime(m, &inst);
            oracle.prepare(&prep_ctx, &fb, &params, &mut GroupOps::default())?;
            let prep_s = t_prep.elapsed().as_secs_f64();
            let threads = std::thread::available_parallelism()
                .map(|n| n.get().min(4))
                .unwrap_or(1);
            let t_all = Instant::now();
            for i in 0..multi {
                let seed = seed0.wrapping_add(30_000 + i as u64 * 0x9E37);
                let planted = 1 + (seed % (inst.r - 1));
                let (wall, decomp, verified) =
                    collect_kmitm_threaded(&oracle, &fb, &inst, threads, seed, planted)?;
                collect_sum += wall;
                decomp_sum += decomp;
                if verified {
                    ok += 1;
                }
            }
            let total = t_all.elapsed().as_secs_f64();
            Ok(json!({
                "arm": spec, "mode": "shared_prepare_multi_threaded",
                "bits": bits, "size": size, "threads": threads,
                "multi": multi, "prep_s": prep_s, "collect_sum_s": collect_sum,
                "decomp_sum_s": decomp_sum,
                "per_dlp_collect_s": collect_sum / multi as f64,
                "per_dlp_with_amort_prep_s": collect_sum / multi as f64 + prep_s / multi as f64,
                "wall_clock_s": total, "verified": ok,
            }))
        }
        other => Err(format!("shared_multi unsupported {other}")),
    }
}

fn oracle_growth_cell(bits: u32, size: usize, n_targets: usize) -> Result<serde_json::Value, String> {
    let inst = prime_instance_for(bits, 59297);
    let fb = prime_factor_base(&inst, size);
    let a = fb.abscissa_list.len();
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
    let mut scratch = GroupOps::default();
    let mut state = 0x9E37_79B9_7F4A_7C15u64 ^ (bits as u64) << 17 ^ size as u64;
    let targets: Vec<PrimePoint> = (0..n_targets)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            g.mul(
                &mut scratch,
                inst.generator_point(),
                1 + state % (inst.r - 1),
            )
        })
        .collect();

    let mut arms = Vec::new();
    for (label, spec) in [
        ("subtract:m=3", "rootfind-subtract:m=3"),
        ("floor:m=3", "rootfind-floor:m=3"),
        ("kmitm:m=4", "rootfind-kmitm:m=4"),
        ("kmitm:m=5", "rootfind-kmitm:m=5"),
        ("kmitm:m=4,balanced", "rootfind-kmitm:m=4,split=balanced"),
    ] {
        let (name, params) = Params::parse_spec(spec)?;
        let m = params.u64_or("m", 3)? as u32;
        let t_prep = Instant::now();
        let prep_err = match name.as_str() {
            "rootfind-subtract" => {
                let mut o = RootfindSubtractOracle::new(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())
                    .err()
            }
            "rootfind-floor" => {
                let mut o = RootfindFloorOracle::new(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())
                    .err()
            }
            "rootfind-kmitm" => {
                let mut o = RootfindKMitmOracle::new_prime(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())
                    .err()
            }
            _ => Some("bad".into()),
        };
        let prep_ns = t_prep.elapsed().as_nanos() as f64;
        if let Some(e) = prep_err {
            arms.push(json!({"arm": label, "ok": false, "error": e, "prep_ns": prep_ns}));
            continue;
        }
        let mut adds = 0u64;
        let mut hits = 0u64;
        let probe_ns = match name.as_str() {
            "rootfind-subtract" => {
                let mut o = RootfindSubtractOracle::new(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())?;
                for t in targets.iter().take(8) {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    let _ = o.decompose(&ctx, &fb, &mut op, &mut c, *t);
                }
                let t0 = Instant::now();
                for &t in &targets {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if o.decompose(&ctx, &fb, &mut op, &mut c, t).is_some() {
                        hits += 1;
                    }
                    adds += op.adds;
                }
                t0.elapsed().as_nanos() as f64 / targets.len() as f64
            }
            "rootfind-floor" => {
                let mut o = RootfindFloorOracle::new(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())?;
                for t in targets.iter().take(8) {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    let _ = o.decompose(&ctx, &fb, &mut op, &mut c, *t);
                }
                let t0 = Instant::now();
                for &t in &targets {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if o.decompose(&ctx, &fb, &mut op, &mut c, t).is_some() {
                        hits += 1;
                    }
                    adds += op.adds;
                }
                t0.elapsed().as_nanos() as f64 / targets.len() as f64
            }
            "rootfind-kmitm" => {
                let mut o = RootfindKMitmOracle::new_prime(m, &inst);
                o.prepare(&ctx, &fb, &params, &mut GroupOps::default())?;
                for t in targets.iter().take(8) {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    let _ = o.decompose(&ctx, &fb, &mut op, &mut c, *t);
                }
                let t0 = Instant::now();
                for &t in &targets {
                    let mut op = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if o.decompose(&ctx, &fb, &mut op, &mut c, t).is_some() {
                        hits += 1;
                    }
                    adds += op.adds;
                }
                t0.elapsed().as_nanos() as f64 / targets.len() as f64
            }
            _ => 0.0,
        };
        arms.push(json!({
            "arm": label, "ok": true, "abscissae": a, "prep_ns": prep_ns,
            "probe_ns_per_target": probe_ns,
            "charged_adds_per_target": adds as f64 / targets.len() as f64,
            "hits": hits,
        }));
    }
    Ok(json!({
        "bits": bits, "size": size, "abscissae": a, "targets": n_targets,
        "r": inst.r, "arms": arms,
    }))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let bits_list = list_u32(&get("--bits", "32"));
    let sizes = list_usize(&get("--sizes", "32,64"));
    let multi: usize = get("--multi", "8").parse().unwrap();
    let repeats: usize = get("--repeats", "5").parse().unwrap();
    let seed0: u64 = get("--seed", "59297").parse().unwrap();
    let focus_bits = *bits_list.last().unwrap_or(&28);
    let focus_size = *sizes.last().unwrap_or(&64);

    println!(
        "rootfind 10x+exponent probe  bits={bits_list:?} sizes={sizes:?} focus={focus_bits}/{focus_size} multi={multi}  [pcc:unverified]"
    );

    let cold_arms: &[(&str, &str)] = &[
        ("subtract:m=3", "rootfind-subtract:m=3"),
        ("floor:m=3", "rootfind-floor:m=3"),
        ("kmitm:m=4", "rootfind-kmitm:m=4"),
        ("kmitm:m=5", "rootfind-kmitm:m=5"),
        ("kmitm:m=4,balanced", "rootfind-kmitm:m=4,split=balanced"),
    ];

    println!("\n## Cold planted-log e2e @ bits={focus_bits} |Fx|≈{focus_size}");
    let mut cold_summary = Vec::new();
    let mut cold_rows = Vec::new();
    for &(arm, spec) in cold_arms {
        let mut walls = Vec::new();
        let mut setups = Vec::new();
        let mut decomps = Vec::new();
        let mut ss = Vec::new();
        let mut ok = 0usize;
        let mut err = None;
        for rep in 0..repeats {
            let seed = seed0.wrapping_add(rep as u64 * 0x9E37);
            let planted = {
                let inst = prime_instance_for(focus_bits, seed);
                1 + (seed % (inst.r - 1))
            };
            match cold_pipeline(spec, focus_bits, focus_size, seed, planted) {
                Ok((wall, setup, decomp, s, verified)) => {
                    walls.push(wall);
                    setups.push(setup);
                    decomps.push(decomp);
                    ss.push(s);
                    if verified {
                        ok += 1;
                    }
                    println!(
                        "  {arm:<22} rep={rep} wall={wall:.4}s setup={setup:.4}s decomp={decomp:.4}s S={s:.3} verified={verified}"
                    );
                    cold_rows.push(json!({
                        "arm": arm, "bits": focus_bits, "size": focus_size, "rep": rep,
                        "wall_s": wall, "setup_s": setup, "decomp_s": decomp, "S": s,
                        "verified": verified,
                    }));
                }
                Err(e) => {
                    println!("  {arm:<22} SKIP: {e}");
                    err = Some(e);
                    break;
                }
            }
        }
        if let Some(e) = err {
            cold_summary.push(json!({"arm": arm, "skipped": true, "skip_reason": e}));
            continue;
        }
        let med_wall = median(&mut walls);
        let row = json!({
            "arm": arm,
            "median_wall_s": med_wall,
            "median_setup_s": median(&mut setups),
            "median_decomp_s": median(&mut decomps),
            "median_S": median(&mut ss),
            "verified_reps": ok,
            "repeats": repeats,
        });
        println!(
            "  {arm:<22} MEDIAN wall={med_wall:.4}s verified={ok}/{repeats}"
        );
        cold_summary.push(row);
    }
    let sub_wall = cold_summary
        .iter()
        .find(|r| r.get("arm").and_then(|v| v.as_str()) == Some("subtract:m=3"))
        .and_then(|r| r.get("median_wall_s").and_then(|v| v.as_f64()));

    println!("\n## Shared-prepare multi-DLP (N={multi})");
    let mut shared_rows = Vec::new();
    for spec in [
        "rootfind-floor:m=3",
        "rootfind-kmitm:m=4",
        "rootfind-kmitm:m=5",
        "rootfind-kmitm:m=4,split=balanced",
    ] {
        match shared_multi(focus_bits, focus_size, spec, multi, seed0) {
            Ok(mut v) => {
                let per = v
                    .get("per_dlp_with_amort_prep_s")
                    .and_then(|x| x.as_f64())
                    .unwrap_or(0.0);
                let vs = sub_wall.map(|s| if per > 0.0 { s / per } else { 0.0 });
                println!(
                    "  {spec:<40} per_amort={per:.4}s vs_sub_cold={} verified={}/{}",
                    vs.map(|x| format!("{x:.2}x")).unwrap_or_else(|| "n/a".into()),
                    v.get("verified").and_then(|x| x.as_u64()).unwrap_or(0),
                    multi
                );
                if let Some(x) = vs {
                    v.as_object_mut()
                        .unwrap()
                        .insert("wall_speedup_vs_subtract_cold".into(), json!(x));
                }
                shared_rows.push(v);
            }
            Err(e) => {
                println!("  {spec:<40} SKIP: {e}");
                shared_rows.push(json!({"arm": spec, "skipped": true, "skip_reason": e}));
            }
        }
    }

    // Honest G3-style ladder: fix bits, vary |Fx| only (mixed-bit fits are
    // confounded by hit-rate). Prefer high-bit cells so most probes miss.
    let growth_bits = focus_bits;
    let growth_sizes = [8usize, 12, 16, 24, 32, 48, 64];
    println!("\n## Oracle growth ladder (fixed bits={growth_bits}, vary |Fx|)");
    let mut growth_rows = Vec::new();
    for &size in &growth_sizes {
        match oracle_growth_cell(growth_bits, size, 512) {
            Ok(cell) => {
                let a = cell.get("abscissae").and_then(|v| v.as_u64()).unwrap_or(0);
                print!("  A={size} |Fx|={a}");
                if let Some(arms) = cell.get("arms").and_then(|v| v.as_array()) {
                    for arm in arms {
                        let name = arm.get("arm").and_then(|v| v.as_str()).unwrap_or("?");
                        if arm.get("ok") == Some(&json!(false)) {
                            print!(" | {name}=SKIP");
                        } else {
                            print!(
                                " | {name} {:.0}ns/{:.1}adds hits={}",
                                arm.get("probe_ns_per_target")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.0),
                                arm.get("charged_adds_per_target")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.0),
                                arm.get("hits").and_then(|v| v.as_u64()).unwrap_or(0)
                            );
                        }
                    }
                }
                println!();
                growth_rows.push(cell);
            }
            Err(e) => println!("  A={size} SKIP: {e}"),
        }
    }

    let mut slopes = Vec::new();
    println!("\n## Growth slopes (log2 adds/target vs log2 |Fx|; floor=1; bits={growth_bits})");
    for arm_name in [
        "subtract:m=3",
        "floor:m=3",
        "kmitm:m=4",
        "kmitm:m=5",
        "kmitm:m=4,balanced",
    ] {
        let mut xs = Vec::new();
        let mut ys_adds = Vec::new();
        let mut ys_probe = Vec::new();
        for cell in &growth_rows {
            let a = cell.get("abscissae").and_then(|v| v.as_f64()).unwrap_or(0.0);
            if a <= 0.0 {
                continue;
            }
            if let Some(arms) = cell.get("arms").and_then(|v| v.as_array()) {
                for arm in arms {
                    if arm.get("arm").and_then(|v| v.as_str()) != Some(arm_name) {
                        continue;
                    }
                    if arm.get("ok") != Some(&json!(true)) {
                        continue;
                    }
                    // Drop cells where hit-rate > 25% — early exits flatten adds.
                    let hits = arm.get("hits").and_then(|v| v.as_f64()).unwrap_or(0.0);
                    let tgts = cell.get("targets").and_then(|v| v.as_f64()).unwrap_or(1.0);
                    if hits / tgts > 0.25 {
                        continue;
                    }
                    xs.push(a);
                    ys_adds.push(
                        arm.get("charged_adds_per_target")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                    );
                    ys_probe.push(
                        arm.get("probe_ns_per_target")
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0),
                    );
                }
            }
        }
        let slope_adds = linreg_log2(&xs, &ys_adds);
        let slope_probe = linreg_log2(&xs, &ys_probe);
        let floor = 1.0;
        let below = slope_adds.map(|(s, _)| s < floor - 0.15).unwrap_or(false);
        println!(
            "  {arm_name:<22} adds_slope={} probe_slope={} below_floor={below} n={}",
            slope_adds
                .map(|(s, _)| format!("{s:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            slope_probe
                .map(|(s, _)| format!("{s:.3}"))
                .unwrap_or_else(|| "n/a".into()),
            xs.len()
        );
        slopes.push(json!({
            "arm": arm_name,
            "bits": growth_bits,
            "n": xs.len(),
            "xs_Fx": xs,
            "ys_adds_per_target": ys_adds,
            "slope_log2_adds_vs_log2_Fx": slope_adds.map(|t| t.0),
            "slope_log2_probe_ns_vs_log2_Fx": slope_probe.map(|t| t.0),
            "pair_table_floor": floor,
            "below_floor": below,
            "note": "hit-rate>25% cells dropped",
        }));
    }

    let any_below = slopes
        .iter()
        .any(|s| s.get("below_floor") == Some(&json!(true)));
    let best_shared = shared_rows
        .iter()
        .filter_map(|r| {
            r.get("wall_speedup_vs_subtract_cold")
                .and_then(|v| v.as_f64())
                .map(|x| (r.get("arm").and_then(|a| a.as_str()).unwrap_or("?"), x))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let best_cold = cold_summary
        .iter()
        .filter_map(|r| {
            let arm = r.get("arm").and_then(|a| a.as_str())?;
            if arm == "subtract:m=3" {
                return None;
            }
            let w = r.get("median_wall_s").and_then(|v| v.as_f64())?;
            let sw = sub_wall?;
            Some((arm, sw / w))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Same-seed paired wall ratios (subtract / challenger) — more honest than
    // median-of-medians when subtract variance is high.
    let mut paired = Vec::new();
    for challenger in ["floor:m=3", "kmitm:m=4", "kmitm:m=5", "kmitm:m=4,balanced"] {
        let mut ratios = Vec::new();
        for rep in 0..repeats {
            let sub = cold_rows.iter().find(|r| {
                r.get("arm").and_then(|a| a.as_str()) == Some("subtract:m=3")
                    && r.get("rep").and_then(|v| v.as_u64()) == Some(rep as u64)
            });
            let ch = cold_rows.iter().find(|r| {
                r.get("arm").and_then(|a| a.as_str()) == Some(challenger)
                    && r.get("rep").and_then(|v| v.as_u64()) == Some(rep as u64)
            });
            if let (Some(s), Some(c)) = (sub, ch) {
                let sw = s.get("wall_s").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let cw = c.get("wall_s").and_then(|v| v.as_f64()).unwrap_or(0.0);
                if cw > 0.0 {
                    ratios.push(sw / cw);
                }
            }
        }
        if ratios.is_empty() {
            continue;
        }
        let med = median(&mut ratios.clone());
        println!(
            "  paired {challenger:<22} median_ratio={med:.3}x n={}",
            ratios.len()
        );
        paired.push(json!({
            "challenger": challenger,
            "median_paired_wall_ratio_vs_subtract": med,
            "min": ratios.iter().cloned().fold(f64::INFINITY, f64::min),
            "max": ratios.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            "n": ratios.len(),
            "ratios": ratios,
        }));
    }
    let best_paired = paired
        .iter()
        .filter_map(|r| {
            Some((
                r.get("challenger").and_then(|a| a.as_str())?,
                r.get("median_paired_wall_ratio_vs_subtract")
                    .and_then(|v| v.as_f64())?,
            ))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let out = json!({
        "pcc": "unverified",
        "classification": if any_below { "exponent_candidate" } else { "engineering" },
        "g3_any_below_pair_table_floor": any_below,
        "focus": {"bits": focus_bits, "size": focus_size, "multi": multi},
        "subtract_cold_median_wall_s": sub_wall,
        "best_cold_speedup": best_cold.map(|(a,x)| json!({"arm": a, "x": x})),
        "best_paired_wall_speedup": best_paired.map(|(a,x)| json!({"arm": a, "x": x})),
        "best_shared_speedup": best_shared.map(|(a,x)| json!({"arm": a, "x": x})),
        "paired_wall_ratios": paired,
        "cold_summary": cold_summary,
        "cold_rows": cold_rows,
        "shared_prepare_multi": shared_rows,
        "oracle_growth": growth_rows,
        "growth_slopes": slopes,
        "claim_hygiene": "no security / sub-rho / key-recovery claim; multi-target amort is secondary; G3 = oracle adds/target vs |Fx| only",
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    Ok(())
}
