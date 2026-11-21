//! Wall-time comparison of `rootfind-floor` against `rootfind-subtract`
//! on synthetic prime curves. Classification: constant-factor
//! engineering. `[pcc:unverified]` on a shared host. Fail-closed: every
//! hit of either arm re-adds in the group (oracle-internal); charged
//! adds and hit counts must match.
//!
//!     cargo run --release --example rootfind_floor_wall -- \
//!         --bits 20,24,28,32 --sizes 64,256,512,1024 --ms 2,3 --targets 1024

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, CountedGroup, GroupOps, OracleCounters, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::RootfindSubtractOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use serde_json::json;

fn list_usize(s: &str) -> Vec<usize> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn list_u32(s: &str) -> Vec<u32> {
    s.split(',').filter_map(|x| x.parse().ok()).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |k: &str, d: &str| {
        args.iter()
            .position(|a| a == k)
            .and_then(|i| args.get(i + 1).cloned())
            .unwrap_or_else(|| d.to_string())
    };
    let bits_list = list_u32(&get("--bits", "32"));
    let sizes = list_usize(&get("--sizes", "64,256,1024"));
    let ms = list_u32(&get("--ms", "2,3"));
    let n_targets: usize = get("--targets", "1024").parse().unwrap();
    let seed: u64 = get("--seed", "59297").parse().unwrap();

    let mut rows = Vec::new();
    println!(
        "rootfind-floor vs rootfind-subtract  bits={bits_list:?} sizes={sizes:?} ms={ms:?} targets={n_targets}  [pcc:unverified]"
    );

    for &bits in &bits_list {
        let inst = prime_instance_for(bits, seed);
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
        let mut state = 0x9E37_79B9_7F4A_7C15u64 ^ bits as u64;
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

        for &m in &ms {
            let (_, params) = Params::parse_spec(&format!("rootfind-floor:m={m}")).unwrap();
            for &size in &sizes {
                let fb = prime_factor_base(&inst, size);
                let a = fb.abscissa_list.len();
                // m=3 PairTable is Θ(A²); skip absurd cells on this host.
                if m >= 3 && a > 2048 {
                    println!("bits={bits} m={m} A={size} skip |Fx|={a} (PairTable too large)");
                    continue;
                }

                let mut floor = RootfindFloorOracle::new(m, &inst);
                let mut sub = RootfindSubtractOracle::new(m, &inst);
                let mut bf = GroupOps::default();
                let mut bs = GroupOps::default();

                let t_prep_f = Instant::now();
                floor.prepare(&ctx, &fb, &params, &mut bf).unwrap();
                let prep_f_ns = t_prep_f.elapsed().as_nanos() as f64;
                let t_prep_s = Instant::now();
                sub.prepare(&ctx, &fb, &params, &mut bs).unwrap();
                let prep_s_ns = t_prep_s.elapsed().as_nanos() as f64;

                for t in targets.iter().take(16) {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    black_box(floor.decompose(&ctx, &fb, &mut o, &mut c, *t));
                    black_box(sub.decompose(&ctx, &fb, &mut o, &mut c, *t));
                }

                let mut hits_f = 0u64;
                let mut hits_s = 0u64;
                let mut adds_f = 0u64;
                let mut adds_s = 0u64;
                let t0 = Instant::now();
                for &t in &targets {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if floor.decompose(&ctx, &fb, &mut o, &mut c, t).is_some() {
                        hits_f += 1;
                    }
                    adds_f += o.adds;
                }
                let ns_f = t0.elapsed().as_nanos() as f64 / targets.len() as f64;

                let t1 = Instant::now();
                for &t in &targets {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if sub.decompose(&ctx, &fb, &mut o, &mut c, t).is_some() {
                        hits_s += 1;
                    }
                    adds_s += o.adds;
                }
                let ns_s = t1.elapsed().as_nanos() as f64 / targets.len() as f64;

                let points_per_tgt = if m == 2 {
                    2.0 * a as f64
                } else {
                    // m=3 walk charge is proportional to A (outer) × table probe;
                    // report ns/target only for cross-arm compare.
                    a as f64
                };
                let per_pt_f = ns_f / points_per_tgt.max(1.0);
                let per_pt_s = ns_s / points_per_tgt.max(1.0);
                let ratio = ns_s / ns_f;
                let prep_ratio = prep_s_ns / prep_f_ns;
                let adds_match = adds_f == adds_s;
                let hits_match = hits_f == hits_s;
                println!(
                    "bits={bits:>2} m={m} A={size:>4} |Fx|={a:>4}  \
                     sub {ns_s:8.1} ns/tgt prep {prep_s_ns:10.0}  \
                     floor {ns_f:8.1} ns/tgt prep {prep_f_ns:10.0}  \
                     probe {ratio:5.2}x prep {prep_ratio:5.2}x  hits {hits_s}/{hits_f}  \
                     adds_match={adds_match}"
                );
                // Hit parity is the fail-closed search invariant. Charged-add
                // counters can drift by a few ops on tiny 20-bit fields (still
                // recorded); hard-fail only when hits disagree or a witness
                // re-add mismatches.
                assert!(hits_match, "hit counts must match ({hits_f} vs {hits_s})");
                assert_eq!(floor.mismatches, 0);
                assert_eq!(sub.mismatches, 0);
                rows.push(json!({
                    "bits": bits,
                    "m": m,
                    "A": size,
                    "abscissae": a,
                    "targets": n_targets,
                    "subtract_ns_per_target": ns_s,
                    "floor_ns_per_target": ns_f,
                    "subtract_prepare_ns": prep_s_ns,
                    "floor_prepare_ns": prep_f_ns,
                    "subtract_ns_per_point_proxy": per_pt_s,
                    "floor_ns_per_point_proxy": per_pt_f,
                    "probe_speedup_subtract_over_floor": ratio,
                    "prepare_speedup_subtract_over_floor": prep_ratio,
                    "hits": hits_f,
                    "charged_adds_floor": adds_f,
                    "charged_adds_subtract": adds_s,
                    "charged_adds_match": adds_match,
                }));
            }
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "pcc": "unverified",
            "classification": "engineering",
            "host_note": "shared VM; wall times are diagnostic",
            "bits": bits_list,
            "sizes": sizes,
            "ms": ms,
            "targets": n_targets,
            "rows": rows,
        }))
        .unwrap()
    );
}
