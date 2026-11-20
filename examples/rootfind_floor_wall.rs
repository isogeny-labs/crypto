//! Wall-time comparison of `rootfind-floor` against `rootfind-subtract`
//! on a synthetic 32-bit prime curve. Classification: constant-factor
//! engineering. `[pcc:unverified]` on a shared host. Fail-closed: every
//! hit of either arm re-adds in the group (oracle-internal).
//!
//!     cargo run --release --example rootfind_floor_wall -- \
//!         --bits 32 --sizes 64,256,1024 --targets 2048

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, CountedGroup, GroupOps, OracleCounters, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::RootfindSubtractOracle;
use crypto_lib::cryptanalysis::ic_framework::rootfind_floor::RootfindFloorOracle;
use crypto_lib::cryptanalysis::ic_framework::stages::{DecompositionOracle, InstanceCtx, Params};
use serde_json::json;

fn list(s: &str) -> Vec<usize> {
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
    let bits: u32 = get("--bits", "32").parse().unwrap();
    let sizes = list(&get("--sizes", "64,256,1024"));
    let n_targets: usize = get("--targets", "2048").parse().unwrap();

    let inst = prime_instance_for(bits, 59297);
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
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
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

    let (_, params) = Params::parse_spec("rootfind-floor:m=2").unwrap();
    let mut rows = Vec::new();
    println!(
        "rootfind-floor vs rootfind-subtract  bits={bits} targets={n_targets}  [pcc:unverified]"
    );
    for &size in &sizes {
        let fb = prime_factor_base(&inst, size);
        let mut floor = RootfindFloorOracle::new(2, &inst);
        let mut sub = RootfindSubtractOracle::new(2, &inst);
        let mut bf = GroupOps::default();
        let mut bs = GroupOps::default();
        floor.prepare(&ctx, &fb, &params, &mut bf).unwrap();
        sub.prepare(&ctx, &fb, &params, &mut bs).unwrap();

        // Warm-up.
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

        let a = fb.abscissa_list.len();
        let per_pt_f = ns_f / (2.0 * a as f64);
        let per_pt_s = ns_s / (2.0 * a as f64);
        let ratio = ns_s / ns_f;
        println!(
            "A={size:>4}  |Fx|={a:>4}  subtract {ns_s:8.1} ns/tgt ({per_pt_s:6.1} ns/pt)  \
             floor {ns_f:8.1} ns/tgt ({per_pt_f:6.1} ns/pt)  ratio {ratio:5.2}x  \
             hits {hits_s}/{hits_f}  adds {adds_s}/{adds_f}"
        );
        assert_eq!(adds_f, adds_s, "charged additions must match");
        assert_eq!(hits_f, hits_s, "hit counts must match");
        assert_eq!(floor.mismatches, 0);
        assert_eq!(sub.mismatches, 0);
        rows.push(json!({
            "A": size,
            "abscissae": a,
            "targets": n_targets,
            "subtract_ns_per_target": ns_s,
            "floor_ns_per_target": ns_f,
            "subtract_ns_per_point": per_pt_s,
            "floor_ns_per_point": per_pt_f,
            "speedup_subtract_over_floor": ratio,
            "hits": hits_f,
            "charged_adds": adds_f,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "bits": bits,
            "pcc": "unverified",
            "classification": "engineering",
            "host_note": "shared VM; wall times are diagnostic",
            "rows": rows,
        }))
        .unwrap()
    );
}
