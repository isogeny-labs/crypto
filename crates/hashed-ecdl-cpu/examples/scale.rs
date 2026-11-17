//! Bounded CPU feasibility ladder. Targets are public points from x search;
//! no scalar is constructed or provided to the solver.
use hashed_ecdl_cpu::{paper, Archive, CachedMitm};
use std::{env, fs, time::Instant};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        return Err("scale ARM[u|x] CASE[0..5] SEED[101|202] OUT".into());
    }
    let arm = &args[1];
    let case: usize = args[2].parse()?;
    let seed: u64 = args[3].parse()?;
    if !["u", "x"].contains(&arm.as_str()) || case > 5 || ![101, 202].contains(&seed) {
        return Err("invalid fixture".into());
    }
    // Exact fixtures from the repository's independently checked prime frontier.
    let fixtures = [
        (1048571, 1048568, 44, 1048189, 2, 317355),
        (16777199, 16777196, 31, 16782299, 2, 5412647),
        (268435399, 268435396, 3, 268407199, 1, 1),
        (4294967291, 4294967288, 31, 4294959973, 2, 2120051858),
        (68719476731, 68719476728, 33, 68719022539, 2, 51741851039),
        (
            1099511627563,
            1099511627560,
            56,
            1099512551701,
            1,
            15267301451,
        ),
    ];
    let (p, a, b, r, px, py) = fixtures[case];
    let start = Instant::now();
    let generator = Some((px, py));
    let curve = paper::Curve::with_order_witness(p, a, b, r, generator)?;
    let mut x = seed * 1009 % p;
    let target = loop {
        if let Some(point) = curve.decode(2 * x) {
            break Some(point);
        }
        x = (x + 1) % p;
    };
    let (mut lo, mut hi) = (0u64, r);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if (0..8).fold(1u128, |n, _| n.saturating_mul(mid as u128)) < r as u128 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let tau = 2 * hi + 2;
    let config = paper::Config {
        m: 8,
        tau: Some(tau),
        seed,
        max_trials: 16,
        max_work: 20_000_000,
        max_columns: 16384,
    };
    let mut oracle = CachedMitm::new(20_000_000, 64)?;
    let mut profile = paper::ReductionStats::default();
    let cert = paper::solve_with_oracle_profile(
        &curve,
        generator,
        target,
        &config,
        &mut oracle,
        &mut profile,
    )?;
    let (relations, attempts, scalar) = (cert.relations.len(), cert.attempts, cert.scalar);
    let archive = Archive::new([p, a, b, r], generator, target, cert);
    let check_start = Instant::now();
    archive.verify()?;
    let second_replay = check_start.elapsed().as_secs_f64();
    let serialized = serde_json::to_vec_pretty(&archive)?;
    let complete = start.elapsed().as_secs_f64();
    // Corruption control outside the complete interval.
    let mut bad = serde_json::to_value(&archive)?;
    bad["scalar"] = ((scalar + 1) % r).into();
    assert!(serde_json::from_value::<Archive>(bad)?.verify().is_err());
    let s = oracle.stats();
    fs::write(
        format!("{}/scale-{arm}-{case}-{seed}.json", args[4]),
        serialized,
    )?;
    println!("{arm},{case},{seed},0,false,verified,{},{tau},{scalar},{relations},{attempts},{complete},{},{},{},{},{},{second_replay},{},{},{},{},{},L0",
        20+4*case,profile.bases.as_secs_f64(),profile.targets.as_secs_f64(),profile.oracle.as_secs_f64(),profile.linear.as_secs_f64(),profile.replay.as_secs_f64(),s.preparations,s.built_rows,s.probed_rows,s.query_inversions,s.peak_table_bytes);
    Ok(())
}
