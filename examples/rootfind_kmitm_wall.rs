//! Wall/build comparison: full `ksum` vs `rootfind-kmitm` for m=3..6,
//! and kmitm-only for m=8,10,12 (full ksum cannot). Synthetic prime
//! curve. Classification: engineering. `[pcc:unverified]`.
//!
//!     cargo run --release --example rootfind_kmitm_wall -- \
//!         --bits 24 --size 14 --targets 64

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    prime_factor_base, prime_instance_for, CountedGroup, GroupOps, OracleCounters, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::KSumOracle;
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
    let bits: u32 = get("--bits", "24").parse().unwrap();
    let size: usize = get("--size", "14").parse().unwrap();
    let n_targets: usize = get("--targets", "64").parse().unwrap();

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
    println!("rootfind-kmitm vs ksum  bits={bits} |Fx|={a} targets={n_targets}  [pcc:unverified]");

    let mut state = 0xDEAD_BEEF_u64;
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

    let mut rows = Vec::new();
    for m in [3u32, 4, 5, 6, 8, 10, 12] {
        let (_, pm) = Params::parse_spec(&format!("rootfind-kmitm:m={m}")).unwrap();
        let mut mitm = RootfindKMitmOracle::new_prime(m, &inst);
        let mut bm = GroupOps::default();
        let t_build_m = Instant::now();
        mitm.prepare(&ctx, &fb, &pm, &mut bm).unwrap();
        let build_m_ns = t_build_m.elapsed().as_nanos() as f64;
        let left_entries = mitm.left_entries();
        let left_k = mitm.left_k();
        let right_k = mitm.right_k();
        let right_pre = mitm.right_precomputed();

        // Warm-up.
        for t in targets.iter().take(4) {
            let mut o = GroupOps::default();
            let mut c = OracleCounters::default();
            black_box(mitm.decompose(&ctx, &fb, &mut o, &mut c, *t));
        }
        let mut hits_m = 0u64;
        let t0 = Instant::now();
        for &t in &targets {
            let mut o = GroupOps::default();
            let mut c = OracleCounters::default();
            if mitm.decompose(&ctx, &fb, &mut o, &mut c, t).is_some() {
                hits_m += 1;
            }
        }
        let probe_m_ns = t0.elapsed().as_nanos() as f64 / targets.len() as f64;

        // PLAN lever 2 (scalar): 4-target shared batch-inv throughput.
        let lane = 4usize;
        let mut lane_out = vec![None; lane];
        for chunk in targets.chunks(lane).take(1) {
            let mut o = GroupOps::default();
            let mut c = OracleCounters::default();
            mitm.decompose_lanes(
                &ctx,
                &fb,
                &mut o,
                &mut c,
                chunk,
                &mut lane_out[..chunk.len()],
            );
            black_box(&lane_out);
        }
        let mut hits_lanes = 0u64;
        let t_lanes = Instant::now();
        for chunk in targets.chunks(lane) {
            let mut o = GroupOps::default();
            let mut c = OracleCounters::default();
            mitm.decompose_lanes(
                &ctx,
                &fb,
                &mut o,
                &mut c,
                chunk,
                &mut lane_out[..chunk.len()],
            );
            for slot in lane_out.iter().take(chunk.len()) {
                if slot.is_some() {
                    hits_lanes += 1;
                }
            }
        }
        let probe_lanes_ns = t_lanes.elapsed().as_nanos() as f64 / targets.len() as f64;

        // PLAN lever 4: threads over independent targets (shared Arc tables).
        let mut thr_ns = [0.0f64; 3];
        let mut thr_hits = [0u64; 3];
        for (ti, threads) in [1usize, 2, 4].into_iter().enumerate() {
            black_box(mitm.decompose_threaded(&ctx, &fb, &targets[..targets.len().min(4)], threads));
            let t_thr = Instant::now();
            let out = mitm.decompose_threaded(&ctx, &fb, &targets, threads);
            thr_ns[ti] = t_thr.elapsed().as_nanos() as f64 / targets.len() as f64;
            thr_hits[ti] = out.iter().filter(|h| h.is_some()).count() as u64;
            black_box(&out);
        }

        let (ksum_build_ns, ksum_probe_ns, ksum_entries, ksum_hits, ksum_ok) = if m <= 9 {
            let (_, pk) = Params::parse_spec(&format!("ksum:m={m},prune=1")).unwrap();
            let mut ksum = KSumOracle::new(m);
            let mut bk = GroupOps::default();
            let tb = Instant::now();
            let prepared = ksum.prepare(&ctx, &fb, &pk, &mut bk);
            if let Err(e) = prepared {
                println!("m={m:>2}  ksum refused: {e}");
                (None, None, None, None, false)
            } else {
                let build_ns = tb.elapsed().as_nanos() as f64;
                let entries = <KSumOracle as DecompositionOracle<
                    crypto_lib::cryptanalysis::ic_boundary::PrimeCurve,
                >>::setup_work_native(&ksum)
                .get("table_insert_attempts")
                .copied();
                for t in targets.iter().take(4) {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    black_box(ksum.decompose(&ctx, &fb, &mut o, &mut c, *t));
                }
                let mut hits = 0u64;
                let tp = Instant::now();
                for &t in &targets {
                    let mut o = GroupOps::default();
                    let mut c = OracleCounters::default();
                    if ksum.decompose(&ctx, &fb, &mut o, &mut c, t).is_some() {
                        hits += 1;
                    }
                }
                let probe_ns = tp.elapsed().as_nanos() as f64 / targets.len() as f64;
                (Some(build_ns), Some(probe_ns), entries, Some(hits), true)
            }
        } else {
            (None, None, None, None, false)
        };

        let build_ratio = ksum_build_ns.map(|k| k / build_m_ns);
        let probe_ratio = ksum_probe_ns.map(|k| k / probe_m_ns);
        let lane_speedup = probe_m_ns / probe_lanes_ns;
        let thr2_speedup = thr_ns[0] / thr_ns[1];
        let thr4_speedup = thr_ns[0] / thr_ns[2];
        println!(
            "m={m:>2}  split {left_k}/{right_k}  kmitm build {build_m_ns:10.0} ns  \
             left {left_entries:<8} right_pre {right_pre:<6}  \
             probe {probe_m_ns:10.1} ns/tgt  lanes4 {probe_lanes_ns:10.1} ns/tgt \
             ({lane_speedup:4.2}x)  thr1/2/4 {thr1:10.1}/{thr2:10.1}/{thr4:10.1} \
             ({thr2_speedup:4.2}x/{thr4_speedup:4.2}x) hits {hits_m}/{hits_lanes}/{h1}/{h2}/{h4}   \
             ksum build {}  probe {}  ratio build/probe {} / {}",
            ksum_build_ns
                .map(|v| format!("{v:10.0} ns"))
                .unwrap_or_else(|| "     n/a   ".into()),
            ksum_probe_ns
                .map(|v| format!("{v:10.1} ns/tgt"))
                .unwrap_or_else(|| "     n/a      ".into()),
            build_ratio
                .map(|v| format!("{v:6.1}x"))
                .unwrap_or_else(|| "   n/a".into()),
            probe_ratio
                .map(|v| format!("{v:6.1}x"))
                .unwrap_or_else(|| "   n/a".into()),
            thr1 = thr_ns[0],
            thr2 = thr_ns[1],
            thr4 = thr_ns[2],
            h1 = thr_hits[0],
            h2 = thr_hits[1],
            h4 = thr_hits[2],
        );
        rows.push(json!({
            "m": m,
            "left_k": left_k,
            "right_k": right_k,
            "split": "probe",
            "abscissae": a,
            "kmitm_build_ns": build_m_ns,
            "kmitm_left_entries": left_entries,
            "kmitm_right_precomputed": right_pre,
            "kmitm_probe_ns_per_target": probe_m_ns,
            "kmitm_probe_ns_per_target_lanes4": probe_lanes_ns,
            "kmitm_lanes4_speedup": lane_speedup,
            "kmitm_probe_ns_per_target_thr1": thr_ns[0],
            "kmitm_probe_ns_per_target_thr2": thr_ns[1],
            "kmitm_probe_ns_per_target_thr4": thr_ns[2],
            "kmitm_thr2_speedup": thr2_speedup,
            "kmitm_thr4_speedup": thr4_speedup,
            "kmitm_hits": hits_m,
            "kmitm_hits_lanes4": hits_lanes,
            "kmitm_hits_thr1": thr_hits[0],
            "kmitm_hits_thr2": thr_hits[1],
            "kmitm_hits_thr4": thr_hits[2],
            "ksum_ok": ksum_ok,
            "ksum_build_ns": ksum_build_ns,
            "ksum_probe_ns_per_target": ksum_probe_ns,
            "ksum_insert_attempts": ksum_entries,
            "ksum_hits": ksum_hits,
            "build_speedup_ksum_over_kmitm": build_ratio,
            "probe_speedup_ksum_over_kmitm": probe_ratio,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "bits": bits,
            "factor_base_size": size,
            "abscissae": a,
            "targets": n_targets,
            "pcc": "unverified",
            "classification": "engineering",
            "rows": rows,
        }))
        .unwrap()
    );
}
