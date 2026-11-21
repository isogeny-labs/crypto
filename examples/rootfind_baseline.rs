//! Kernel-level baseline of the root-finding decomposition oracles
//! (`research/rootfind_baseline_100x_20261117/PROTOCOL.md`).
//!
//! For each (group, factor-base size `A`) cell and each oracle arm it times,
//! on one thread:
//!
//! * `prepare` (the pair-table build), once per round;
//! * `decompose` over a fixed list of targets, which for random targets are
//!   refutations (all `2A` signed points computed and probed);
//! * and reports the counted work the oracle charged (additions, field
//!   products and inversions where the group counts them, table lookups).
//!
//! Arms run interleaved, in an order rotated each round, with an A/A copy of
//! the first arm, so the spread between identical arms is on the page next
//! to every difference.  Output is JSON lines on stdout; nothing here is an
//! isolated measurement (`isolation: "none"`).
//!
//!     rootfind_baseline [--rounds 5] [--targets 256] [--prime-bits 24,32,40]
//!         [--prime-sizes 64,256,1024] [--koblitz 17:6,23:8] [--pairs-max 256]

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_instance, prime_instance_for, BinaryGroup, CountedGroup, FactorBase, GroupOps,
    OracleCounters, PrimeCurve, PrimeInstance,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{
    BinarySubspaceBase, PrimeAbscissaBase, RootfindSubtractOracle,
};
use crypto_lib::cryptanalysis::ic_framework::rootfind::{
    PairedAdd, RootfindMitmOracle, RootfindPairsOracle,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    DecompositionOracle, FactorBaseBuilder, InstanceCtx, Params,
};
use serde_json::json;

struct Round {
    build_s: f64,
    solve_s: f64,
    adds: u64,
    muls: u64,
    sqrs: u64,
    invs: u64,
    lookups: u64,
    found: u64,
    build_adds: u64,
}

fn one_round<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    fb: &FactorBase<G::Elt>,
    oracle: &mut dyn DecompositionOracle<G>,
    params: &Params,
    targets: &[G::Elt],
) -> Round {
    let mut build_ops = GroupOps::default();
    let t0 = Instant::now();
    oracle
        .prepare(ctx, fb, params, &mut build_ops)
        .expect("the oracle prepares");
    let build_s = t0.elapsed().as_secs_f64();
    let mut ops = GroupOps::default();
    let mut counters = OracleCounters::default();
    let mut found = 0u64;
    let t1 = Instant::now();
    for &t in targets {
        if black_box(oracle.decompose(ctx, fb, &mut ops, &mut counters, t)).is_some() {
            found += 1;
        }
    }
    let solve_s = t1.elapsed().as_secs_f64();
    Round {
        build_s,
        solve_s,
        adds: ops.adds,
        muls: ops.field_ops.muls,
        sqrs: ops.field_ops.sqrs,
        invs: ops.field_ops.invs,
        lookups: counters.lookups,
        found,
        build_adds: build_ops.adds,
    }
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

type Arm<'a, G> = (
    &'static str,
    Box<dyn FnMut() -> Box<dyn DecompositionOracle<G> + 'a> + 'a>,
    Params,
);

#[allow(clippy::too_many_arguments)]
fn run_cell<'a, G: CountedGroup>(
    family: &str,
    size_label: serde_json::Value,
    ctx: &InstanceCtx<G>,
    fb: &FactorBase<G::Elt>,
    targets: &[G::Elt],
    rounds: usize,
    mut arms: Vec<Arm<'a, G>>,
) {
    let a = fb.abscissa_list.len();
    let mut results: Vec<Vec<Round>> = (0..arms.len()).map(|_| Vec::new()).collect();
    for round in 0..=rounds {
        for step in 0..arms.len() {
            let i = (step + round) % arms.len();
            let (_, make, params) = &mut arms[i];
            let mut oracle = make();
            let r = one_round(ctx, fb, oracle.as_mut(), params, targets);
            // Round 0 is the warm-up: page faults, frequency, allocator.
            if round > 0 {
                results[i].push(r);
            }
        }
    }
    for (i, (name, _, _)) in arms.iter().enumerate() {
        let rs = &results[i];
        let mut build: Vec<f64> = rs.iter().map(|r| r.build_s).collect();
        let mut solve: Vec<f64> = rs.iter().map(|r| r.solve_s).collect();
        let last = rs.last().expect("at least one measured round");
        let (b_med, s_med) = (median(&mut build), median(&mut solve));
        let (s_min, s_max) = (solve[0], solve[solve.len() - 1]);
        println!(
            "{}",
            json!({
                "schema": "rootfind.baseline/v1",
                "isolation": "none",
                "family": family,
                "size": size_label,
                "abscissae": a,
                "base_points": fb.points.len(),
                "arm": name,
                "rounds": rs.len(),
                "targets": targets.len(),
                "found": last.found,
                "build_s_median": b_med,
                "build_adds": last.build_adds,
                "solve_s_median": s_med,
                "solve_s_min": s_min,
                "solve_s_max": s_max,
                "ns_per_target_median": s_med * 1e9 / targets.len() as f64,
                "adds": last.adds,
                "ns_per_add_median": if last.adds > 0 { s_med * 1e9 / last.adds as f64 } else { 0.0 },
                "ns_per_build_add_median": if last.build_adds > 0 { b_med * 1e9 / last.build_adds as f64 } else { 0.0 },
                "field_muls": last.muls,
                "field_sqrs": last.sqrs,
                "field_invs": last.invs,
                "lookups": last.lookups,
            })
        );
    }
}

fn targets_of<G: CountedGroup>(g: &G, generator: G::Elt, r: u64, count: usize) -> Vec<G::Elt> {
    let mut scratch = GroupOps::default();
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    (0..count)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            g.mul(&mut scratch, generator, 1 + state % (r - 1))
        })
        .collect()
}

fn prime_cell(
    inst: &PrimeInstance,
    bits: u32,
    size: usize,
    n_targets: usize,
    rounds: usize,
    pairs_max: usize,
) {
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: None,
    };
    let (_, bparams) = Params::parse_spec(&format!("prime-abscissa:size={size}")).unwrap();
    let mut ops = GroupOps::default();
    let fb = match (PrimeAbscissaBase { instance: inst }).build(&ctx, &bparams, &mut ops) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("prime {bits}-bit size {size}: {e}");
            return;
        }
    };
    let targets = targets_of(&inst.curve, inst.generator_point(), inst.r, n_targets);
    let none = Params::default();
    let (_, m3) = Params::parse_spec("rootfind-subtract:m=3").unwrap();
    let mut arms: Vec<Arm<PrimeCurve>> = vec![
        (
            "rootfind-mitm",
            Box::new(|| Box::new(RootfindMitmOracle::new(3))),
            none.clone(),
        ),
        (
            "rootfind-mitm-prime-batch",
            Box::new(|| Box::new(RootfindMitmOracle::new_prime_batch(3))),
            none.clone(),
        ),
        (
            "rootfind-subtract",
            Box::new(|| Box::new(RootfindSubtractOracle::new(3, inst))),
            m3,
        ),
        (
            "rootfind-mitm-aa",
            Box::new(|| Box::new(RootfindMitmOracle::new(3))),
            none.clone(),
        ),
    ];
    if size <= pairs_max {
        arms.push((
            "rootfind-pairs",
            Box::new(|| Box::new(RootfindPairsOracle::new(3))),
            none,
        ));
    }
    run_cell(
        "prime",
        json!({"bits": bits, "p": inst.curve.p, "r": inst.r}),
        &ctx,
        &fb,
        &targets,
        rounds,
        arms,
    );
}

fn koblitz_cell(n: u32, dim: u32, n_targets: usize, rounds: usize, pairs_max: usize) {
    let Some(inst) = [1u8, 0].into_iter().find_map(|a| koblitz_instance(a, n)) else {
        eprintln!("no registered Koblitz instance at n = {n}");
        return;
    };
    let g = BinaryGroup(&inst.fast);
    let ctx = InstanceCtx {
        group: &g,
        generator: inst.generator,
        target: inst.generator,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(inst.n),
    };
    let (_, bparams) = Params::parse_spec(&format!("binary-subspace:dimension={dim}")).unwrap();
    let mut ops = GroupOps::default();
    let fb = match (BinarySubspaceBase { instance: &inst }).build(&ctx, &bparams, &mut ops) {
        Ok(fb) => fb,
        Err(e) => {
            eprintln!("koblitz n={n} d={dim}: {e}");
            return;
        }
    };
    let targets = targets_of(&g, inst.generator, inst.r, n_targets);
    let none = Params::default();
    let mut arms: Vec<Arm<BinaryGroup>> = vec![
        (
            "rootfind-mitm",
            Box::new(|| Box::new(RootfindMitmOracle::new(3))),
            none.clone(),
        ),
        (
            "rootfind-mitm-aa",
            Box::new(|| Box::new(RootfindMitmOracle::new(3))),
            none.clone(),
        ),
    ];
    if fb.abscissa_list.len() <= pairs_max {
        arms.push((
            "rootfind-pairs",
            Box::new(|| Box::new(RootfindPairsOracle::new(3))),
            none,
        ));
    }
    run_cell(
        "koblitz",
        json!({"n": n, "dimension": dim, "r": inst.r, "name": inst.name}),
        &ctx,
        &fb,
        &targets,
        rounds,
        arms,
    );
}

/// The price of one group addition done the serial and the batched way,
/// away from any table: `R + P_k` over the base, repeated.
fn add_kernel<G: PairedAdd>(
    family: &str,
    label: serde_json::Value,
    g: &G,
    pts: &[G::Elt],
    r: G::Elt,
) {
    let reps = 20_000_000 / pts.len().max(1);
    let mut ops = GroupOps::default();
    let mut counters = OracleCounters::default();
    let mut out = Vec::with_capacity(pts.len());
    let mut best = [f64::INFINITY; 3];
    for _ in 0..3 {
        let t = Instant::now();
        for _ in 0..reps {
            for &q in pts {
                black_box(g.add(&mut ops, r, q));
            }
        }
        best[0] = best[0].min(t.elapsed().as_secs_f64());
        let t = Instant::now();
        for _ in 0..reps {
            for block in pts.chunks(32) {
                out.clear();
                g.add_many(&mut ops, &mut counters, r, block, &mut out);
                black_box(&out);
            }
        }
        best[1] = best[1].min(t.elapsed().as_secs_f64());
        let t = Instant::now();
        for _ in 0..reps {
            for block in pts.chunks(32) {
                out.clear();
                g.add_many_batched(&mut ops, &mut counters, r, block, &mut out);
                black_box(&out);
            }
        }
        best[2] = best[2].min(t.elapsed().as_secs_f64());
    }
    let n = (reps * pts.len()) as f64;
    println!(
        "{}",
        json!({
            "schema": "rootfind.baseline.kernel/v1",
            "isolation": "none",
            "family": family,
            "size": label,
            "points": pts.len(),
            "ns_per_add_serial": best[0] * 1e9 / n,
            "ns_per_add_many_block32": best[1] * 1e9 / n,
            "ns_per_add_many_batched_block32": best[2] * 1e9 / n,
        })
    );
}

fn list<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split(',')
        .filter(|x| !x.is_empty())
        .filter_map(|x| x.parse().ok())
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let rounds: usize = get("--rounds", "5").parse().unwrap();
    let n_targets: usize = get("--targets", "256").parse().unwrap();
    let prime_bits: Vec<u32> = list(&get("--prime-bits", "24,32,40"));
    let prime_sizes: Vec<usize> = list(&get("--prime-sizes", "64,256,1024"));
    let pairs_max: usize = get("--pairs-max", "256").parse().unwrap();
    let koblitz: Vec<(u32, u32)> = get("--koblitz", "17:6,23:8")
        .split(',')
        .filter_map(|s| {
            let (n, d) = s.split_once(':')?;
            Some((n.parse().ok()?, d.parse().ok()?))
        })
        .collect();

    for &bits in &prime_bits {
        let inst = prime_instance_for(bits, 59297);
        for &size in &prime_sizes {
            prime_cell(&inst, bits, size, n_targets, rounds, pairs_max);
        }
        let ctx_g = &inst.curve;
        let mut scratch = GroupOps::default();
        let pts: Vec<_> = (2..=257u64)
            .map(|k| ctx_g.mul(&mut scratch, inst.generator_point(), k))
            .collect();
        let r = ctx_g.mul(&mut scratch, inst.generator_point(), 0x1234_5677 % inst.r);
        add_kernel(
            "prime",
            json!({"bits": bits, "p": inst.curve.p}),
            ctx_g,
            &pts,
            r,
        );
    }
    for &(n, dim) in &koblitz {
        koblitz_cell(n, dim, n_targets, rounds, pairs_max);
        if let Some(inst) = [1u8, 0].into_iter().find_map(|a| koblitz_instance(a, n)) {
            let g = BinaryGroup(&inst.fast);
            let mut scratch = GroupOps::default();
            let pts: Vec<_> = (2..=257u64)
                .map(|k| g.mul(&mut scratch, inst.generator, k))
                .collect();
            let r = g.mul(&mut scratch, inst.generator, 0x1234_5677 % inst.r);
            add_kernel("koblitz", json!({"n": n}), &g, &pts, r);
        }
    }
}
