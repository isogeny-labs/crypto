//! **E40 controls** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.50,
//! G2.205): E25's ten curves, `--calls` calls each on stream
//! `seed ^ 0xE40C_0001` (per-target seeds as E37's Stage A).
//!
//! - **P1** (broken storage): V with every row stored from one column right
//!   of its lead.  Detected when V-P1 differs from A counted in solutions,
//!   degree, outcome, decomposition set, fallback flag, any phase count or
//!   the pivot digest (§9.50's Stage A identity).
//! - **P2** (broken count): V's stored-cell counter short by one per row.
//!   Detected when the counter differs from the measured row lengths.
//! - **Perturbed coefficient** (E31's): one coefficient of one equation moved
//!   by one; V's decomposition set must change on every call whose clean set
//!   is non-empty.
//! - Clean V against A counted (the identity itself) on the same calls.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e40_checks --calls 32 --json experiments/47_macaulay_entries/checks.json
//! ```

use std::collections::HashMap;

use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_profile::{profile_measure, ProfileCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const SALT: u64 = 0xE40C_0001;

type Out = (
    Option<Vec<[u64; 4]>>,
    u16,
    String,
    Vec<[u64; 4]>,
    bool,
    [u64; 9],
    u64,
);

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let per_curve: usize = arg(&args, "--calls", 32);
    let json_out: String = arg(&args, "--json", String::new());
    let mut rows = Vec::new();
    let (mut total, mut ident, mut p1, mut p2, mut nonempty, mut pert) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let mut rng = StdRng::seed_from_u64(seed ^ SALT);
            let sseed = seed ^ SALT;
            let n = inst.n;
            let (mut calls, mut residuals) = (0usize, 0u64);
            let (mut c_id, mut c_p1, mut c_p2, mut c_ne, mut c_pert) = (0, 0, 0, 0, 0);
            'outer: loop {
                let batch: Vec<(u64, u64, u64)> = (0..8u64)
                    .map(|k| {
                        (
                            rng.gen_range(0..n),
                            rng.gen_range(1..n),
                            mix64(sseed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                        )
                    })
                    .collect();
                for (a, b, tseed) in batch {
                    if calls >= per_curve {
                        break 'outer;
                    }
                    residuals += 1;
                    let c = inst.curve();
                    let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                    let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                        continue;
                    };
                    let mut fp = 0u64;
                    let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                    calls += 1;
                    let run = |cs: &[HashMap<[u8; 4], u64>], pv: Pivot| -> Out {
                        let mut trng = StdRng::seed_from_u64(tseed);
                        let mut st = QuarticSolve::default();
                        let (s, d, info) = solve_weighted_chain_with(
                            cs,
                            MACAULAY_BOUND,
                            p,
                            &mut trng,
                            &mut st,
                            pv,
                        );
                        let lb = lift_back_v4(&ki, &c, s.as_deref().unwrap_or(&[]), &r, &mut trng);
                        let ph = [
                            st.weil_muls,
                            st.triangularise_muls,
                            st.echelon_muls,
                            st.nf_muls,
                            st.charpoly_muls,
                            st.roots_muls,
                            st.eigenvector_muls,
                            st.verify_muls,
                            st.total_muls,
                        ];
                        (
                            s,
                            d,
                            st.outcome,
                            lb.decompositions,
                            info.fell_back,
                            ph,
                            st.pivot_digest,
                        )
                    };
                    let prof = |chk| Pivot::Profile(chk);
                    let a = run(
                        &comps,
                        Pivot::Counted(PivotRule::Lowest, PivotCheck::default()),
                    );
                    let v = run(&comps, prof(ProfileCheck::default()));
                    let v1 = run(
                        &comps,
                        prof(ProfileCheck {
                            drop_lead: true,
                            undercount: false,
                        }),
                    );
                    let m2 = profile_measure(
                        &comps,
                        MACAULAY_BOUND,
                        p,
                        ProfileCheck {
                            drop_lead: false,
                            undercount: true,
                        },
                    );
                    // E31's perturbed coefficient: the smallest key of the first
                    // non-empty equation, moved by one.
                    let mut bad = comps.clone();
                    if let Some(eq) = bad.iter_mut().find(|e| !e.is_empty()) {
                        let k = *eq.keys().min().unwrap();
                        let v0 = eq[&k];
                        eq.insert(k, (v0 + 1) % p);
                    }
                    let vp = run(&bad, prof(ProfileCheck::default()));
                    let id = v == a;
                    let d1 = v1 != a;
                    let d2 = m2.is_some_and(|m| m.counter != m.measured);
                    let ne = !v.3.is_empty();
                    let dp = ne && vp.3 != v.3;
                    c_id += usize::from(id);
                    c_p1 += usize::from(d1);
                    c_p2 += usize::from(d2);
                    c_ne += usize::from(ne);
                    c_pert += usize::from(dp);
                }
            }
            eprintln!(
                "p={p} seed={seed}: {calls} calls; V = A counted {c_id}; P1 detected {c_p1}; P2 detected {c_p2}; perturbed changed {c_pert}/{c_ne} non-empty"
            );
            rows.push(json!({"p": p, "seed": seed, "calls": calls, "v_equals_a_counted": c_id,
                "p1_detected": c_p1, "p2_detected": c_p2, "nonempty": c_ne, "perturbed_changed": c_pert}));
            total += calls;
            ident += c_id;
            p1 += c_p1;
            p2 += c_p2;
            nonempty += c_ne;
            pert += c_pert;
        }
    }
    let v = json!({"stream_salt": SALT, "calls": total, "v_equals_a_counted": ident,
        "p1_detected": p1, "p2_detected": p2, "nonempty": nonempty, "perturbed_changed": pert,
        "g2_205_holds": p1 == total && p2 == total && pert == nonempty && ident == total,
        "curves": rows});
    eprintln!("total: {v}");
    if !json_out.is_empty() {
        std::fs::write(&json_out, serde_json::to_string_pretty(&v).unwrap()).unwrap();
    }
}
