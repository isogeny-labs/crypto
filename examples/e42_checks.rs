//! **E42 controls** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.54,
//! G2.231): E25's ten curves, `--calls` calls each on stream
//! `seed ^ 0xE42C_0001` (per-target seeds as E37's Stage A).
//!
//! - **P1** (broken W): rows capped at their build end and the fill check
//!   off (fill beyond a row's end dropped).  Detected when W-P1 differs from
//!   A counted in solutions, degree, outcome, decomposition set, fallback
//!   flag, any phase count or the pivot digest (§9.54's Stage A identity).
//! - **P2** (broken count): W's stored-cell counter short by one per row.
//!   Detected when the counter differs from the final row lengths plus the
//!   cells of the buffers re-allocations discarded.
//! - **G0** (growth path, must pass): rows capped at their build end, fill
//!   check on (re-allocation on most rows).  Must equal A counted and pass
//!   the counter check on every call.
//! - **Perturbed coefficient** (E31's, on the solution set): one coefficient
//!   of one equation moved by one; W's solution set must change on every
//!   call whose clean solution set is non-empty.
//! - Clean W against A counted (the identity itself) on the same calls.
//!
//! E42b (§9.56): `--salt` sets the stream (hex; default E42's `E42C0001`,
//! E42b's `E42C0002`); `g2_245_holds` is the kill (every per-call check),
//! `g2_246_holds` the descriptive G0 count `[200, 300]` per call; E42's
//! `g2_231_holds` (with §9.54's `[900, 960]`) is kept for the record.
//!
//! ```text
//! RAYON_NUM_THREADS=1 ./e42_checks --calls 32 --json experiments/49_macaulay_tails/checks.json
//! RAYON_NUM_THREADS=1 ./e42_checks --calls 32 --salt E42C0002 --json experiments/49_macaulay_tails/checks_b.json
//! ```

use std::collections::HashMap;

use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::macaulay_tails::{tails_measure, TailsCheck};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::json;

const SALT: u64 = 0xE42C_0001;

fn salt_arg(args: &[String]) -> u64 {
    args.iter()
        .position(|a| a == "--salt")
        .and_then(|i| args.get(i + 1))
        .map(|v| u64::from_str_radix(v.trim_start_matches("0x"), 16).expect("hex salt"))
        .unwrap_or(SALT)
}

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
    let salt = salt_arg(&args);
    let mut rows = Vec::new();
    let (mut total, mut ident, mut p1, mut p2, mut nonempty, mut pert) =
        (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    let (mut g0_id, mut g0_ctr, mut g0_reallocs, mut w_ctr, mut w_reallocs) =
        (0usize, 0usize, 0u64, 0usize, 0u64);
    for p in [101u64, 149, 197, 269, 397] {
        for seed in [1u64, 2] {
            let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
            let inst = ki.inst;
            let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
            let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
            let pcurve = inst.curve();
            let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
            let mut rng = StdRng::seed_from_u64(seed ^ salt);
            let sseed = seed ^ salt;
            let n = inst.n;
            let (mut calls, mut residuals) = (0usize, 0u64);
            let (mut c_id, mut c_p1, mut c_p2, mut c_ne, mut c_pert) = (0, 0, 0, 0, 0);
            let (mut c_g0, mut c_g0c, mut c_g0r, mut c_wc, mut c_wr) = (0, 0, 0u64, 0, 0u64);
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
                    let tails = |chk| Pivot::Tails(chk);
                    let a = run(
                        &comps,
                        Pivot::Counted(PivotRule::Lowest, PivotCheck::default()),
                    );
                    let w = run(&comps, tails(TailsCheck::default()));
                    let p1c = TailsCheck {
                        exact_caps: true,
                        no_fill: true,
                        undercount: false,
                    };
                    let g0c = TailsCheck {
                        exact_caps: true,
                        ..Default::default()
                    };
                    let p2c = TailsCheck {
                        undercount: true,
                        ..Default::default()
                    };
                    let v1 = run(&comps, tails(p1c));
                    let g0 = run(&comps, tails(g0c));
                    let ok = |m: Option<_>| -> (bool, u64) {
                        match m {
                            Some(m) => {
                                let m: crypto_lib::cryptanalysis::macaulay_tails::TailsMeasure = m;
                                (m.counter == m.final_lengths + m.discarded, m.reallocs)
                            }
                            None => (false, 0),
                        }
                    };
                    let (m2_ok, _) = ok(tails_measure(&comps, MACAULAY_BOUND, p, p2c));
                    let (g0_ok, g0_r) = ok(tails_measure(&comps, MACAULAY_BOUND, p, g0c));
                    let (w_ok, w_r) = ok(tails_measure(
                        &comps,
                        MACAULAY_BOUND,
                        p,
                        TailsCheck::default(),
                    ));
                    // E31's perturbed coefficient: the smallest key of the first
                    // non-empty equation, moved by one.
                    let mut bad = comps.clone();
                    if let Some(eq) = bad.iter_mut().find(|e| !e.is_empty()) {
                        let k = *eq.keys().min().unwrap();
                        let v0 = eq[&k];
                        eq.insert(k, (v0 + 1) % p);
                    }
                    let vp = run(&bad, tails(TailsCheck::default()));
                    let v = w;
                    c_g0 += usize::from(g0 == a);
                    c_g0c += usize::from(g0_ok);
                    c_g0r += g0_r;
                    c_wc += usize::from(w_ok);
                    c_wr += w_r;
                    let id = v == a;
                    let d1 = v1 != a;
                    let d2 = !m2_ok;
                    let ne = v.0.as_ref().is_some_and(|s| !s.is_empty());
                    let dp = ne && vp.0 != v.0;
                    c_id += usize::from(id);
                    c_p1 += usize::from(d1);
                    c_p2 += usize::from(d2);
                    c_ne += usize::from(ne);
                    c_pert += usize::from(dp);
                }
            }
            eprintln!(
                "p={p} seed={seed}: {calls} calls; W = A counted {c_id}; W counter ok {c_wc} (reallocs {c_wr}); P1 detected {c_p1}; P2 detected {c_p2}; G0 = A counted {c_g0}, counter ok {c_g0c}, reallocs {c_g0r}; perturbed changed {c_pert}/{c_ne} non-empty"
            );
            rows.push(
                json!({"p": p, "seed": seed, "calls": calls, "w_equals_a_counted": c_id,
                "w_counter_ok": c_wc, "w_reallocs": c_wr,
                "p1_detected": c_p1, "p2_detected": c_p2,
                "g0_equals_a_counted": c_g0, "g0_counter_ok": c_g0c, "g0_reallocs": c_g0r,
                "nonempty": c_ne, "perturbed_changed": c_pert}),
            );
            g0_id += c_g0;
            g0_ctr += c_g0c;
            g0_reallocs += c_g0r;
            w_ctr += c_wc;
            w_reallocs += c_wr;
            total += calls;
            ident += c_id;
            p1 += c_p1;
            p2 += c_p2;
            nonempty += c_ne;
            pert += c_pert;
        }
    }
    let g0_mean = g0_reallocs as f64 / total.max(1) as f64;
    let kill_ok = p1 == total
        && p2 == total
        && pert == nonempty
        && ident == total
        && w_ctr == total
        && g0_id == total
        && g0_ctr == total;
    let v = json!({"stream_salt": salt, "calls": total, "w_equals_a_counted": ident,
        "w_counter_ok": w_ctr, "w_reallocs": w_reallocs,
        "p1_detected": p1, "p2_detected": p2,
        "g0_equals_a_counted": g0_id, "g0_counter_ok": g0_ctr, "g0_reallocs": g0_reallocs,
        "g0_reallocs_per_call": g0_mean,
        "nonempty": nonempty, "perturbed_changed": pert,
        "g2_231_holds": kill_ok && (900.0..=960.0).contains(&g0_mean),
        "g2_245_holds": kill_ok,
        "g2_246_holds": (200.0..=300.0).contains(&g0_mean),
        "curves": rows});
    eprintln!("total: {v}");
    if !json_out.is_empty() {
        std::fs::write(&json_out, serde_json::to_string_pretty(&v).unwrap()).unwrap();
    }
}
