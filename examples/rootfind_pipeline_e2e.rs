//! End-to-end planted-log IC: factor base → prepare → relation collection →
//! Gauss → verify `[d]G = Q`, with the decomposition oracle swapped.
//!
//! Unlike `rootfind_kmitm_wall` (oracle microbench), this times the **whole
//! pipeline** wall. Classification: engineering. `[pcc:unverified]`.
//! Public synthetic primes only; fail-closed verify; no security claim.
//!
//! Arms:
//! - `rootfind-subtract:m=2|3` — best existing CPU rootfind
//! - `rootfind-floor:m=2|3` — PLAN lever 1
//! - `rootfind-kmitm:m=3|4|6|10|12` — MITM + hot-path (pipeline uses single
//!   `decompose`; charged `S` unchanged vs its own search)
//! - `kmitm-thr:m=12,threads=1|2|4` — same search, but relation collection
//!   probes independent walk targets via `decompose_threaded` (PLAN lever 4)
//!
//! ```bash
//! cargo run --release --example rootfind_pipeline_e2e -- \
//!   --bits 20 --size 24 --repeats 3
//! ```

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, Calibration, CountedGroup, GroupOps, IncrementalGauss,
    RowStatus,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{PrimeAbscissaBase, RootfindSubtractOracle};
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_kmitm::RootfindKMitmOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, InstanceCtx, Params, Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec, RunReport};
use serde_json::json;

#[derive(Clone)]
struct ArmResult {
    arm: String,
    verified: bool,
    exhausted: bool,
    wall_s: f64,
    s: f64,
    total_gae: f64,
    setup_wall_s: f64,
    decomp_wall_s: f64,
    targets: u64,
    relations: u64,
    recovered: Option<u64>,
}

fn report_to_arm(arm: &str, r: &RunReport) -> ArmResult {
    ArmResult {
        arm: arm.to_string(),
        verified: r.verified,
        exhausted: r.exhausted,
        wall_s: r.wall_seconds,
        s: r.s,
        total_gae: r.total_gae,
        setup_wall_s: r.decomposition.setup.wall_ns as f64 / 1e9,
        decomp_wall_s: r.decomposition.cost.wall_ns as f64 / 1e9,
        targets: r.decomposition.targets_tried,
        relations: r.decomposition.relations_found,
        recovered: r.recovered,
    }
}

fn run_standard(
    arm: &str,
    oracle_spec: &str,
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    size: usize,
    seed: u64,
    planted: u64,
) -> Result<ArmResult, String> {
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
    let (name, params) = Params::parse_spec(oracle_spec)?;
    let mut spec = PipelineSpec {
        factor_base: "prime-abscissa".into(),
        oracle: name.clone(),
        oracle_params: params,
        targets: Targets::Walk,
        max_trials: 2_000_000,
        seed,
        ..Default::default()
    };
    spec.factor_base_params.set("size", size.to_string());
    let m = spec.oracle_params.u64_or("m", 2)? as u32;
    let base = PrimeAbscissaBase { instance: inst };
    let report = match name.as_str() {
        "rootfind-subtract" => {
            let mut o = RootfindSubtractOracle::new(m, inst);
            run_pipeline(
                &ctx,
                &spec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        "rootfind-floor" => {
            let mut o = RootfindFloorOracle::new(m, inst);
            run_pipeline(
                &ctx,
                &spec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        "rootfind-kmitm" => {
            let mut o = RootfindKMitmOracle::new_prime(m, inst);
            run_pipeline(
                &ctx,
                &spec,
                &base,
                &mut o,
                planted,
                &Calibration::default(),
                None,
            )?
        }
        other => return Err(format!("unsupported oracle `{other}`")),
    };
    Ok(report_to_arm(arm, &report))
}

/// Relation collection + Gauss + verify using an already-prepared kmitm
/// oracle (PLAN lever 4 via `decompose_threaded`). `setup_wall_s` is 0 —
/// prepare is outside this call.
fn collect_kmitm_threaded(
    arm: &str,
    oracle: &RootfindKMitmOracle<'_>,
    fb: &crypto_lib::cryptanalysis::ic_boundary::FactorBase<
        crypto_lib::cryptanalysis::ic_boundary::PrimePoint,
    >,
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    threads: usize,
    seed: u64,
    planted: u64,
) -> Result<ArmResult, String> {
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
    let mut relations = 0u64;
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
            let point = ctx.group.add(&mut ops, ag, bq);
            pts.push(point);
            metas.push((a, b));
        }
        let t0 = Instant::now();
        let hits = oracle.decompose_threaded(&ctx, fb, &pts, threads);
        decomp_wall_s += t0.elapsed().as_secs_f64();
        targets_tried += n as u64;
        black_box(&hits);

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
                    relations += 1;
                    if matrix.pinned(fb.columns).is_some() {
                        break 'outer;
                    }
                }
                RowStatus::Dependent => {}
                RowStatus::Inconsistent => {
                    return Err("inconsistent relation in threaded collect".into());
                }
            }
        }
    }

    let d = matrix
        .pinned(fb.columns)
        .ok_or_else(|| format!("{arm}: exhausted without pinning target column"))?;
    let check = ctx.group.mul(&mut ops, ctx.generator, d);
    let verified = check == ctx.target;
    let wall_s = wall0.elapsed().as_secs_f64();
    Ok(ArmResult {
        arm: arm.to_string(),
        verified,
        exhausted: !verified,
        wall_s,
        s: 0.0,
        total_gae: 0.0,
        setup_wall_s: 0.0,
        decomp_wall_s,
        targets: targets_tried,
        relations,
        recovered: verified.then_some(d),
    })
}

/// Full IC solve whose relation phase probes walk targets in parallel via
/// [`RootfindKMitmOracle::decompose_threaded`] (PLAN lever 4). Same search /
/// charged adds as serial kmitm; wall only.
fn run_kmitm_threaded_collect(
    arm: &str,
    m: u32,
    threads: usize,
    inst: &crypto_lib::cryptanalysis::ic_boundary::PrimeInstance,
    size: usize,
    seed: u64,
    planted: u64,
) -> Result<ArmResult, String> {
    let wall0 = Instant::now();
    let fb = prime_factor_base(inst, size);
    let (_, params) = Params::parse_spec(&format!("rootfind-kmitm:m={m}"))?;
    let mut oracle = RootfindKMitmOracle::new_prime(m, inst);
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
    let prep0 = Instant::now();
    oracle.prepare(&prep_ctx, &fb, &params, &mut GroupOps::default())?;
    let setup_wall_s = prep0.elapsed().as_secs_f64();
    let mut row = collect_kmitm_threaded(arm, &oracle, &fb, inst, threads, seed, planted)?;
    row.setup_wall_s = setup_wall_s;
    row.wall_s = wall0.elapsed().as_secs_f64();
    Ok(row)
}

fn median_f64(xs: &mut [f64]) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = xs.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        0.5 * (xs[n / 2 - 1] + xs[n / 2])
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let bits: u32 = get("--bits", "20").parse().unwrap();
    let size: usize = get("--size", "24").parse().unwrap();
    let repeats: usize = get("--repeats", "3").parse().unwrap();
    let seed0: u64 = get("--seed", "59297").parse().unwrap();

    let inst = prime_instance_for(bits, seed0);
    println!(
        "rootfind pipeline e2e  bits={bits} r={} |Fx|≈{size} repeats={repeats}  [pcc:unverified]",
        inst.r
    );
    println!("classification: engineering; fail-closed verify; no security claim");

    let standard_arms: &[(&str, &str)] = &[
        ("subtract:m=2", "rootfind-subtract:m=2"),
        ("subtract:m=3", "rootfind-subtract:m=3"),
        ("floor:m=2", "rootfind-floor:m=2"),
        ("floor:m=3", "rootfind-floor:m=3"),
        ("kmitm:m=3", "rootfind-kmitm:m=3"),
        ("kmitm:m=4", "rootfind-kmitm:m=4"),
        ("kmitm:m=6", "rootfind-kmitm:m=6"),
        ("kmitm:m=10", "rootfind-kmitm:m=10"),
        ("kmitm:m=12", "rootfind-kmitm:m=12"),
    ];

    let mut all_rows = Vec::new();
    let mut summary = Vec::new();

    for &(arm, spec) in standard_arms {
        let mut walls = Vec::new();
        let mut decomp_walls = Vec::new();
        let mut ok = 0usize;
        let mut last: Option<ArmResult> = None;
        for rep in 0..repeats {
            let seed = seed0.wrapping_add(rep as u64 * 0x9E37);
            let planted = 1 + (seed % (inst.r - 1));
            let row = run_standard(arm, spec, &inst, size, seed, planted)?;
            if row.verified {
                ok += 1;
            }
            walls.push(row.wall_s);
            decomp_walls.push(row.decomp_wall_s);
            println!(
                "  {arm:<14} rep={rep} wall={:.4}s decomp={:.4}s setup={:.4}s S={:.3} targets={} rels={} verified={}",
                row.wall_s,
                row.decomp_wall_s,
                row.setup_wall_s,
                row.s,
                row.targets,
                row.relations,
                row.verified
            );
            all_rows.push(json!({
                "arm": arm,
                "mode": "run_pipeline",
                "rep": rep,
                "wall_s": row.wall_s,
                "decomp_wall_s": row.decomp_wall_s,
                "setup_wall_s": row.setup_wall_s,
                "s": row.s,
                "total_gae": row.total_gae,
                "targets": row.targets,
                "relations": row.relations,
                "verified": row.verified,
                "exhausted": row.exhausted,
                "recovered": row.recovered,
            }));
            last = Some(row);
        }
        let med_wall = median_f64(&mut walls);
        let med_decomp = median_f64(&mut decomp_walls);
        let last = last.unwrap();
        summary.push(json!({
            "arm": arm,
            "mode": "run_pipeline",
            "median_wall_s": med_wall,
            "median_decomp_wall_s": med_decomp,
            "verified_reps": ok,
            "repeats": repeats,
            "s": last.s,
        }));
        println!(
            "  {arm:<14} MEDIAN wall={med_wall:.4}s decomp={med_decomp:.4}s verified={ok}/{repeats}"
        );
    }

    // PLAN lever 4 inside a full solve: threaded relation collection for m=12.
    for threads in [1usize, 2, 4] {
        let arm = format!("kmitm:m=12,thr={threads}");
        let mut walls = Vec::new();
        let mut decomp_walls = Vec::new();
        let mut ok = 0usize;
        for rep in 0..repeats {
            let seed = seed0.wrapping_add(rep as u64 * 0x9E37);
            let planted = 1 + (seed % (inst.r - 1));
            let row = run_kmitm_threaded_collect(
                &arm, 12, threads, &inst, size, seed, planted,
            )?;
            if row.verified {
                ok += 1;
            }
            walls.push(row.wall_s);
            decomp_walls.push(row.decomp_wall_s);
            println!(
                "  {arm:<18} rep={rep} wall={:.4}s decomp={:.4}s setup={:.4}s targets={} rels={} verified={}",
                row.wall_s,
                row.decomp_wall_s,
                row.setup_wall_s,
                row.targets,
                row.relations,
                row.verified
            );
            all_rows.push(json!({
                "arm": arm,
                "mode": "threaded_collect",
                "threads": threads,
                "rep": rep,
                "wall_s": row.wall_s,
                "decomp_wall_s": row.decomp_wall_s,
                "setup_wall_s": row.setup_wall_s,
                "targets": row.targets,
                "relations": row.relations,
                "verified": row.verified,
                "exhausted": row.exhausted,
                "recovered": row.recovered,
            }));
        }
        let med_wall = median_f64(&mut walls);
        let med_decomp = median_f64(&mut decomp_walls);
        summary.push(json!({
            "arm": arm,
            "mode": "threaded_collect",
            "threads": threads,
            "median_wall_s": med_wall,
            "median_decomp_wall_s": med_decomp,
            "verified_reps": ok,
            "repeats": repeats,
        }));
        println!(
            "  {arm:<18} MEDIAN wall={med_wall:.4}s decomp={med_decomp:.4}s verified={ok}/{repeats}"
        );
    }

    // Amortized multi-DLP: prepare once, solve many planted logs (relation
    // phase dominates). Shows floor / thread gains when setup is shared.
    let multi_n: usize = get("--multi", "16").parse().unwrap();
    println!("\n## Amortized multi-DLP (prepare once, {multi_n} planted logs)");
    let mut amort = Vec::new();
    for &(arm_name, oracle_spec) in &[
        ("subtract:m=3", "rootfind-subtract:m=3"),
        ("floor:m=3", "rootfind-floor:m=3"),
        ("kmitm:m=4", "rootfind-kmitm:m=4"),
        ("kmitm:m=12", "rootfind-kmitm:m=12"),
    ] {
        let (name, params) = Params::parse_spec(oracle_spec)?;
        let m = params.u64_or("m", 2)? as u32;
        let fb = prime_factor_base(&inst, size);
        let mut prep_ops = GroupOps::default();
        let dummy_ctx = {
            let g = inst.generator_point();
            InstanceCtx {
                group: &inst.curve,
                generator: g,
                target: g,
                r: inst.r,
                cofactor: inst.cofactor,
                group_order: inst.group_order,
                name: inst.name.clone(),
                field_degree: None,
            }
        };
        let t_prep = Instant::now();
        match name.as_str() {
            "rootfind-subtract" => {
                let mut o = RootfindSubtractOracle::new(m, &inst);
                o.prepare(&dummy_ctx, &fb, &params, &mut prep_ops)?;
            }
            "rootfind-floor" => {
                let mut o = RootfindFloorOracle::new(m, &inst);
                o.prepare(&dummy_ctx, &fb, &params, &mut prep_ops)?;
            }
            "rootfind-kmitm" => {
                let mut o = RootfindKMitmOracle::new_prime(m, &inst);
                o.prepare(&dummy_ctx, &fb, &params, &mut prep_ops)?;
            }
            _ => {}
        }
        let prep_s = t_prep.elapsed().as_secs_f64();
        // Re-prepare inside each solve via run_pipeline (honest cold amort
        // would share tables; here we time N full pipelines and report
        // total/N plus a separate shared-prepare threaded collect).
        let mut walls = Vec::new();
        let mut ok = 0usize;
        let t_all = Instant::now();
        for i in 0..multi_n {
            let seed = seed0.wrapping_add(10_000 + i as u64 * 0x9E37);
            let planted = 1 + (seed % (inst.r - 1));
            let row = run_standard(arm_name, oracle_spec, &inst, size, seed, planted)?;
            if row.verified {
                ok += 1;
            }
            walls.push(row.wall_s);
        }
        let total = t_all.elapsed().as_secs_f64();
        let per = total / multi_n as f64;
        println!(
            "  {arm_name:<14} {multi_n} DLPs total={total:.4}s per={per:.4}s prep_once≈{prep_s:.4}s verified={ok}/{multi_n}"
        );
        amort.push(json!({
            "arm": arm_name,
            "mode": "multi_cold_pipeline",
            "n": multi_n,
            "total_wall_s": total,
            "per_dlp_wall_s": per,
            "prep_once_s": prep_s,
            "verified": ok,
        }));
    }

    // Shared-prepare threaded collect: one m=12 prepare, then multi_n solves
    // with thr=1/2/4 (relation phase only amortized).
    println!("\n## Shared-prepare m=12 threaded collect ({multi_n} DLPs)");
    let fb = prime_factor_base(&inst, size);
    let (_, params) = Params::parse_spec("rootfind-kmitm:m=12")?;
    let mut shared = RootfindKMitmOracle::new_prime(12, &inst);
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
    shared.prepare(&prep_ctx, &fb, &params, &mut GroupOps::default())?;
    let shared_prep_s = t_prep.elapsed().as_secs_f64();
    let mut thr_amort = Vec::new();
    for threads in [1usize, 2, 4] {
        let t_all = Instant::now();
        let mut ok = 0usize;
        let mut decomp_s = 0.0f64;
        let mut wall_s = 0.0f64;
        for i in 0..multi_n {
            let seed = seed0.wrapping_add(20_000 + i as u64 * 0x9E37);
            let planted = 1 + (seed % (inst.r - 1));
            let row = collect_kmitm_threaded(
                &format!("shared-thr={threads}"),
                &shared,
                &fb,
                &inst,
                threads,
                seed,
                planted,
            )?;
            if row.verified {
                ok += 1;
            }
            decomp_s += row.decomp_wall_s;
            wall_s += row.wall_s;
        }
        let total = t_all.elapsed().as_secs_f64();
        let per_decomp = decomp_s / multi_n as f64;
        let thr1_ref = thr_amort.first().and_then(|r: &serde_json::Value| {
            r.get("per_decomp_s").and_then(|v| v.as_f64())
        });
        let vs1 = thr1_ref.map(|t1| if per_decomp > 0.0 { t1 / per_decomp } else { 0.0 });
        println!(
            "  thr={threads}  {multi_n} DLPs total={total:.4}s collect_sum={wall_s:.4}s per_decomp={per_decomp:.4}s verified={ok}/{multi_n}{}",
            vs1.map(|x| format!("  vs_thr1={x:.2}x")).unwrap_or_default()
        );
        thr_amort.push(json!({
            "threads": threads,
            "n": multi_n,
            "total_wall_s": total,
            "collect_sum_s": wall_s,
            "decomp_sum_s": decomp_s,
            "per_decomp_s": per_decomp,
            "shared_prep_s": shared_prep_s,
            "decomp_speedup_vs_thr1": vs1,
            "verified": ok,
        }));
    }

    // Gains vs subtract:m=3 (best existing same-arity-ish baseline).
    let base_wall = summary
        .iter()
        .find(|r| r["arm"] == "subtract:m=3")
        .and_then(|r| r["median_wall_s"].as_f64());
    let base_decomp = summary
        .iter()
        .find(|r| r["arm"] == "subtract:m=3")
        .and_then(|r| r["median_decomp_wall_s"].as_f64());
    let thr1 = summary
        .iter()
        .find(|r| r["arm"] == "kmitm:m=12,thr=1")
        .and_then(|r| r["median_decomp_wall_s"].as_f64());

    println!("\n## Gains (median wall / decomp vs subtract:m=3; thr vs thr=1 decomp)");
    let mut gains = Vec::new();
    if let (Some(bw), Some(bd)) = (base_wall, base_decomp) {
        for row in &summary {
            let arm = row["arm"].as_str().unwrap_or("?");
            let mw = row["median_wall_s"].as_f64().unwrap_or(0.0);
            let md = row["median_decomp_wall_s"].as_f64().unwrap_or(0.0);
            let wall_x = if mw > 0.0 { bw / mw } else { 0.0 };
            let decomp_x = if md > 0.0 { bd / md } else { 0.0 };
            let thr_x = if arm.starts_with("kmitm:m=12,thr=") {
                thr1.map(|t1| if md > 0.0 { t1 / md } else { 0.0 })
            } else {
                None
            };
            println!(
                "  {arm:<18} wall_speedup={wall_x:6.2}x  decomp_speedup={decomp_x:6.2}x{}",
                thr_x
                    .map(|x| format!("  thr_vs_1={x:5.2}x"))
                    .unwrap_or_default()
            );
            gains.push(json!({
                "arm": arm,
                "wall_speedup_vs_subtract_m3": wall_x,
                "decomp_speedup_vs_subtract_m3": decomp_x,
                "decomp_speedup_vs_thr1": thr_x,
            }));
        }
    }

    let out = json!({
        "bits": bits,
        "factor_base_size": size,
        "r": inst.r,
        "repeats": repeats,
        "seed": seed0,
        "pcc": "unverified",
        "classification": "engineering",
        "what_this_is": "Complete planted-log IC pipeline wall (factor base + prepare + relation collection + Gauss + verify). Not an oracle-only microbench.",
        "summary": summary,
        "amortized_multi_dlp": amort,
        "threaded_amortized": thr_amort,
        "gains": gains,
        "rows": all_rows,
    });
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
    Ok(())
}
