//! Experiment E3 helper (research/generator_effect_ic_20261019): exact
//! sizes and sum counts for the Kummer tower bases and the random-scalar
//! known-log bases E3 compares at equal B.
//!
//! `cargo run --release --example generator_effect_e3 -- P B_COEF R GX GY T1,T2,… [M]`
//! `cargo run --release --example generator_effect_e3 -- f4cost P B_COEF R GX GY T N_RANDOM N_PLANTED SEED [SPEC]`
//!
//! For each `t`: the columns `B` of `prime-kummer:t=T`, the exact number
//! `N_m` of points that are a sum of `m` base points (either sign,
//! repetition allowed), and the same count for `known-log:size=B,
//! scalars=random` at the same `B`.  `N_m / r` is the hit rate a uniform
//! target has, which E3's measured yields are checked against.  One JSON
//! line per `t`.  Group additions of the enumeration are not part of any
//! arm's cost.
//!
//! `f4cost` runs `pkm-semaev` (default `m=3`, chain) directly on
//! `N_RANDOM` uniform targets `[s]G` and `N_PLANTED` planted sums of three
//! base points, one JSON line per system: F4's multiply-adds, its solving
//! degree, its wall time, and the verdict against the grid's.  It separates
//! the refutation cost from the cost of a consistent system, which the
//! capped `ecbench` runs meet at rate `N_m / r` only.
use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, OracleCounters, PrimeCurve, PrimeInstance,
};
use crypto_lib::cryptanalysis::ic_framework::pkm_semaev::{decomposable_points, PkmSemaevOracle};
use crypto_lib::cryptanalysis::ic_framework::plugins::KnownLogBase;
use crypto_lib::cryptanalysis::ic_framework::prime_tower::PrimeKummerBase;
use crypto_lib::cryptanalysis::ic_framework::stages::DecompositionOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let mut a: Vec<String> = std::env::args().skip(1).collect();
    if a.first().map(String::as_str) == Some("f4cost") {
        a.remove(0);
        return f4cost(&a);
    }
    let n = |i: usize| -> u64 { a[i].parse().expect("integer") };
    let (p, b, r, gx, gy) = (n(0), n(1), n(2), n(3), n(4));
    let ts: Vec<u32> = a[5].split(',').map(|s| s.parse().unwrap()).collect();
    let m: u32 = a.get(6).map_or(3, |s| s.parse().unwrap());
    let inst = PrimeInstance {
        name: "e3".into(),
        curve: PrimeCurve { p, a: p - 3, b },
        group_order: r,
        r,
        cofactor: 1,
        generator: (gx, gy),
    };
    let ctx = InstanceCtx {
        group: &inst.curve,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r,
        cofactor: 1,
        group_order: r,
        name: "e3".into(),
        field_degree: None,
    };
    for t in ts {
        let mut kp = Params::default();
        kp.set("t", t.to_string());
        let fb = PrimeKummerBase { instance: &inst }
            .build(&ctx, &kp, &mut GroupOps::default())
            .expect("kummer base");
        let n_k = decomposable_points(&inst.curve, &fb, m, &mut GroupOps::default()).len();
        let mut lp = Params::default();
        lp.set("size", fb.columns.to_string());
        lp.set("scalars", "random");
        let kl = KnownLogBase
            .build(&ctx, &lp, &mut GroupOps::default())
            .expect("known-log base");
        let n_l = decomposable_points(&inst.curve, &kl, m, &mut GroupOps::default()).len();
        println!(
            "{{\"p\": {p}, \"r\": {r}, \"t\": {t}, \"m\": {m}, \"V\": {}, \"B\": {}, \"N_kummer\": {n_k}, \"hit_kummer\": {:.6e}, \"N_knownlog\": {n_l}, \"hit_knownlog\": {:.6e}}}",
            1u64 << t,
            fb.columns,
            n_k as f64 / r as f64,
            n_l as f64 / r as f64
        );
    }
}

fn f4cost(a: &[String]) {
    let n = |i: usize| -> u64 { a[i].parse().expect("integer") };
    let (p, b, r, gx, gy, t) = (n(0), n(1), n(2), n(3), n(4), n(5) as u32);
    let (n_random, n_planted, seed) = (n(6), n(7), n(8));
    let spec = a.get(9).cloned().unwrap_or_else(|| "m=3".into());
    let inst = PrimeInstance {
        name: "e3".into(),
        curve: PrimeCurve { p, a: p - 3, b },
        group_order: r,
        r,
        cofactor: 1,
        generator: (gx, gy),
    };
    let curve = &inst.curve;
    let ctx = InstanceCtx {
        group: curve,
        generator: inst.generator_point(),
        target: inst.generator_point(),
        r,
        cofactor: 1,
        group_order: r,
        name: "e3".into(),
        field_degree: None,
    };
    let mut kp = Params::default();
    kp.set("t", t.to_string());
    let fb = PrimeKummerBase { instance: &inst }
        .build(&ctx, &kp, &mut GroupOps::default())
        .expect("kummer base");
    let (_, params) = Params::parse_spec(&format!("pkm-semaev:{spec}")).unwrap();
    let m = params.u64_or("m", 3).unwrap() as u32;
    let mut gp = params.clone();
    gp.set("engine", "exhaustive");
    let mut grid = PkmSemaevOracle::new(m, t, 1);
    grid.prepare(&ctx, &fb, &gp, &mut GroupOps::default())
        .unwrap();
    let mut rng = StdRng::seed_from_u64(seed);
    for k in 0..n_random + n_planted {
        let planted = k >= n_random;
        let target = if planted {
            let mut ops = GroupOps::default();
            (0..m).fold(curve.identity(), |acc, _| {
                curve.add(&mut ops, acc, fb.points[rng.gen_range(0..fb.points.len())])
            })
        } else {
            curve.mul(
                &mut GroupOps::default(),
                inst.generator_point(),
                rng.gen_range(1..r),
            )
        };
        if target.infinity {
            continue;
        }
        // A fresh oracle per system, so its totals are this system's.
        let mut o = PkmSemaevOracle::new(m, t, 1);
        o.prepare(&ctx, &fb, &params, &mut GroupOps::default())
            .unwrap();
        let started = std::time::Instant::now();
        let got = o.decompose(
            &ctx,
            &fb,
            &mut GroupOps::default(),
            &mut OracleCounters::default(),
            target,
        );
        let wall = started.elapsed().as_secs_f64();
        let found = grid
            .decompose(
                &ctx,
                &fb,
                &mut GroupOps::default(),
                &mut OracleCounters::default(),
                target,
            )
            .is_some();
        let tot = o.solver_totals().unwrap();
        let native = o.relation_counters();
        println!(
            "{{\"p\": {p}, \"t\": {t}, \"spec\": \"{spec}\", \"B\": {}, \"k\": {k}, \"planted\": {planted}, \"decomposed\": {}, \"grid_decomposed\": {found}, \"muladds\": {}, \"solving_degree\": {}, \"budget_exceeded\": {}, \"system_mults\": {}, \"grid_mults\": {}, \"max_cols\": {}, \"wall_s\": {wall:.4}}}",
            fb.columns,
            got.is_some(),
            tot.ops,
            tot.solving_degree_max,
            tot.budget_exceeded,
            native.get("pkm_system_fp_mults_uncharged").copied().unwrap_or(0),
            native.get("pkm_grid_fp_mults_uncharged").copied().unwrap_or(0),
            tot.extra.get("max_cols_max").copied().unwrap_or(0),
        );
        assert_eq!(got.is_some(), found, "F4 and grid disagree on system {k}");
    }
}
