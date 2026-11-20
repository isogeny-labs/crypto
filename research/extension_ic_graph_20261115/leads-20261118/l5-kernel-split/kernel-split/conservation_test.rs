//! Exact conservation tests of the counted linear stage against the frozen
//! uncounted one (complete-accounting lane). [pcc:unverified]
//!
//! Random zero-dimensional systems (dense quadrics in k unknowns over small
//! primes, k = 2..4) are solved to a certified Gröbner basis by `f4_fp`. On
//! each basis the frozen `zero_dim_la_orig::fast_stage` and the counted
//! `zero_dim_la::fast_stage` run with cloned RNGs under `jv_bitops` counting.
//! Required, exactly, for every system:
//!   (d) identical outputs and legacy `FastStats` counters;
//!   (a) tally(counted) − tally(orig) = the charged `LinOps`, by primitive,
//!       zero in every other kind (inversions stay hooked once);
//!   (b) legacy muls() + mul_outside_legacy
//!       = mac + mul + legacy_overcount + ring_muls.
//! Planted-solution systems make the read-out run with roots.
use crypto_lib::cryptanalysis::{
    f4_fp::{self, Poly},
    jv_bitops::{self as work, Kind, OpTally, Phase},
    jv_cover::{self as cover, StopMode},
    zero_dim_la::{self as counted, LinOps},
    zero_dim_la_orig as orig,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn monomials_deg_le2(k: usize) -> Vec<Vec<u32>> {
    let mut out = Vec::new();
    for i in 0..k {
        for j in i..k {
            let mut e = vec![0u32; k];
            e[i] += 1;
            e[j] += 1;
            out.push(e);
        }
    }
    for i in 0..k {
        let mut e = vec![0u32; k];
        e[i] = 1;
        out.push(e);
    }
    out.push(vec![0u32; k]);
    out
}

/// k random quadrics vanishing at a planted point (so roots exist).
fn planted_system(k: usize, p: u64, rng: &mut StdRng) -> Vec<Poly> {
    let pt: Vec<u64> = (0..k).map(|_| rng.gen_range(0..p)).collect();
    let mons = monomials_deg_le2(k);
    let ev = |e: &Vec<u32>| -> u64 {
        let mut t = 1u128;
        for (i, &x) in e.iter().enumerate() {
            for _ in 0..x {
                t = t * pt[i] as u128 % p as u128;
            }
        }
        t as u64
    };
    (0..k)
        .map(|_| {
            let mut poly: Vec<(Vec<u32>, u64)> = Vec::new();
            let mut val = 0u128;
            for e in &mons[..mons.len() - 1] {
                let c = rng.gen_range(0..p);
                if c != 0 {
                    val = (val + c as u128 * ev(e) as u128) % p as u128;
                    poly.push((e.clone(), c));
                }
            }
            let c0 = (p as u128 - val) % p as u128;
            if c0 != 0 {
                poly.push((vec![0u32; k], c0 as u64));
            }
            poly
        })
        .collect()
}

fn harness(f: impl FnOnce()) -> OpTally {
    let before = work::snapshot();
    {
        let _g = work::enter(Phase::Harness);
        f();
    }
    *work::snapshot().since(&before).phase(Phase::Harness)
}

#[test]
fn counted_linear_stage_conserves_against_the_frozen_stage() {
    work::claim();
    let mut systems = 0;
    let mut with_roots = 0;
    let mut gave_up = 0;
    let mut total = LinOps::default();
    for &p in &[101u64, 1009, 65_521] {
        work::set_moduli(p, 1_000_003);
        work::enable(true);
        let mut rng = StdRng::seed_from_u64(p ^ 0xACC0);
        for k in 2..=4usize {
            for _ in 0..12 {
                let input = planted_system(k, p, &mut rng);
                let opts = cover::opts_for_mode(24, 600.0, StopMode::Full);
                let r = f4_fp::f4(&input, k, p, &opts);
                if r.timed_out || r.inconsistent || r.pairs_above_bound > 0 {
                    continue;
                }
                let seed: u64 = rng.gen();
                let (mut ra, mut rb) = (StdRng::seed_from_u64(seed), StdRng::seed_from_u64(seed));
                let (mut fa, mut fb) = (orig::FastStats::default(), counted::FastStats::default());
                let (mut oa, mut ob) = (None, None);
                let ho = harness(|| oa = Some(orig::fast_stage(&r.basis, &input, k, p, &mut ra, &mut fa)));
                let l0 = counted::lin_ops_snapshot();
                let hc = harness(|| ob = Some(counted::fast_stage(&r.basis, &input, k, p, &mut rb, &mut fb)));
                let d = counted::lin_ops_snapshot().since(&l0);
                // (d)
                assert_eq!(oa, ob, "outputs p={p} k={k}");
                assert_eq!(fa.muls(), fb.muls());
                assert_eq!((fa.delta, fa.roots, fa.gave_up), (fb.delta, fb.roots, fb.gave_up));
                // (a)
                for kd in Kind::ALL {
                    let want = match kd {
                        Kind::FieldMul => d.field_mul(),
                        Kind::FieldAdd => d.field_add(),
                        Kind::WordOp => d.word_op(),
                        Kind::HashProbe => d.probe,
                        _ => 0,
                    };
                    assert_eq!(hc.get(kd) - ho.get(kd), want, "{} p={p} k={k}", work::kind_name(kd));
                }
                assert_eq!(hc.rng_words - ho.rng_words, d.rng);
                // (b)
                assert_eq!(
                    fb.muls() + d.mul_outside_legacy,
                    d.mac + d.mul + d.legacy_overcount + d.ring_muls,
                    "legacy reconciliation p={p} k={k}"
                );
                // L5 (e): per-kernel tallies sum to the call's LinOps exactly
                let mut ksum = LinOps::default();
                for kk in 0..counted::NK {
                    ksum.add_assign(&fb.kernel_ops[kk]);
                }
                assert_eq!(ksum, d, "kernel sum p={p} k={k}");
                assert_eq!(fb.kernel_ops[0], LinOps::default(), "unscoped p={p} k={k}");
                // L5 (f): krylov kernel macs equal the legacy krylov counter
                assert_eq!(fb.kernel_ops[3].mac + fb.kernel_ops[3].mul, fb.krylov_muls, "krylov p={p} k={k}");
                // the uncounted frozen stage saw only hooked inversions/ring work
                assert!(hc.get(Kind::FieldMul) > ho.get(Kind::FieldMul) || fb.gave_up.is_some());
                systems += 1;
                with_roots += usize::from(fb.roots > 0);
                gave_up += usize::from(fb.gave_up.is_some());
                total.add_assign(&d);
            }
        }
    }
    eprintln!("systems {systems}, with roots {with_roots}, gave up {gave_up}, totals {total:?}");
    assert!(systems >= 90, "too few zero-dimensional systems: {systems}");
    assert!(with_roots >= systems / 2);
    assert_eq!(work::foreign(), 0);
}
