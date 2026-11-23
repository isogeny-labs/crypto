//! E33 independent validation (validator-owned code, not E33's): the
//! G2.124 negative controls re-run on calls drawn from the validator's nonce,
//! through the public API of the recorded library (c443138920).  Not
//! examples/e33_checks.rs: written from §9.35's registration and the library
//! signatures only.
//!
//! Per call (E25's instance, base and precompute exactly as Stage A sets them
//! up, targets from `StdRng::seed_from_u64(nonce)`), the solvers
//!   A (chain), B (chain-sparsest), B counted, B' (incremental, verify),
//!   B'' (estimate)
//! and three controls on every call where B counted made a comparison:
//!   NC1 second-best pivot in the first contested column (B', verify):
//!       must change the pivot digest and must not change the output;
//!       also reports whether B's phase counts change (power of the
//!       phase-count proxy the identity check relies on);
//!   NC2 one maintained count off by one (B', verify, perturb_count):
//!       must be flagged by the recount check;
//! and on every call whose B decomposition set is non-empty:
//!   NC3 one coefficient of one equation perturbed (E31's control):
//!       must change the set (B counted).
//!
//!   e33val_controls --p P --seed S --calls N --nonce X [--nc3 K]
//! (`--nc3 K`: NC3 on K further calls with a non-empty set, drawn from a
//! second nonce stream; on the first stream NC3 runs where the set is
//! non-empty, which at 32 calls may be never.)
//! prints one JSON object on stdout.
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::{generate_gls_klein_instance, klein_base_with};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

type Out = (Option<Vec<[u64; 4]>>, u16, String, Vec<[u64; 4]>);

fn arg(args: &[String], k: &str) -> u64 {
    let i = args.iter().position(|a| a == k).expect(k);
    args[i + 1].parse().expect(k)
}

fn phases(s: &QuarticSolve) -> [u64; 12] {
    [
        s.weil_muls,
        s.triangularise_muls,
        s.echelon_muls,
        s.nf_muls,
        s.charpoly_muls,
        s.roots_muls,
        s.eigenvector_muls,
        s.verify_muls,
        s.total_muls,
        s.rows as u64,
        s.cols as u64,
        s.pivots as u64,
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let p = arg(&args, "--p");
    let seed = arg(&args, "--seed");
    let ncalls = arg(&args, "--calls") as usize;
    let nonce = arg(&args, "--nonce");
    let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
    let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
    let _fb = klein_base_with(&ki, false, &mut rng).expect("base");
    let pcurve = ki.inst.curve();
    let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
    let n = ki.inst.n;
    let mut vr = StdRng::seed_from_u64(nonce);
    let verify = PivotCheck {
        verify: true,
        ..Default::default()
    };
    let mut c = [0u64; 20];
    // 0 calls, 1 contested, 2 b_counted_out_eq_b, 3 b_counted_phases_eq_b,
    // 4 inc_out_eq_b, 5 inc_phases_eq_b, 6 inc_digest_eq_b_counted,
    // 7 est_out_eq_b, 8 count_mismatches(b counted + inc),
    // 9 nc1_digest_changed, 10 nc1_output_same, 11 nc1_phases_changed,
    // 12 nc2_flagged, 13 nc3_nonempty, 14 nc3_changed, 15 a_out_eq_b,
    // 16 est_digest_eq_b_counted, 17 nc1_echelon_muls_changed
    let mut rows = Vec::new();
    while (c[0] as usize) < ncalls {
        let (a, b) = (vr.gen_range(0..n), vr.gen_range(1..n));
        let cv = ki.inst.curve();
        let r = cv.add(&cv.mul(&ki.inst.g, a), &cv.mul(&ki.inst.q, b));
        let Some(z_r) = ki.z_of_x(&cv.f, &r.x) else {
            continue;
        };
        let mut fp = 0u64;
        let comps = v4_system(&pre, &cv.f, &z_r, &mut fp);
        let tseed: u64 = vr.gen();
        let run = |cs: &[HashMap<[u8; 4], u64>], pv: Pivot| -> (Out, QuarticSolve) {
            let mut tr = StdRng::seed_from_u64(tseed);
            let mut st = QuarticSolve::default();
            let (s, d, _info) = solve_weighted_chain_with(cs, MACAULAY_BOUND, p, &mut tr, &mut st, pv);
            let lb = lift_back_v4(&ki, &cv, s.as_deref().unwrap_or(&[]), &r, &mut tr);
            ((s, d, st.outcome.clone(), lb.decompositions), st)
        };
        c[0] += 1;
        let (oa, _sa) = run(&comps, Pivot::Uncounted { sparsest: false });
        let (ob, sb) = run(&comps, Pivot::Uncounted { sparsest: true });
        let (obc, sbc) = run(&comps, Pivot::Counted(PivotRule::SparsestRecount, verify));
        let (obi, sbi) = run(&comps, Pivot::Counted(PivotRule::SparsestIncremental, verify));
        let (obe, sbe) = run(
            &comps,
            Pivot::Counted(PivotRule::SparsestEstimate, PivotCheck::default()),
        );
        c[15] += u64::from(oa == ob);
        c[2] += u64::from(obc == ob);
        c[3] += u64::from(phases(&sbc) == phases(&sb));
        c[4] += u64::from(obi == ob);
        c[5] += u64::from(phases(&sbi) == phases(&sb));
        c[6] += u64::from(sbi.pivot_digest == sbc.pivot_digest && sbc.pivot_digest != 0);
        c[7] += u64::from(obe == ob);
        c[16] += u64::from(sbe.pivot_digest == sbc.pivot_digest);
        c[8] += sbc.pivot_count_mismatches + sbi.pivot_count_mismatches;
        let contested = sbc.pivot_compare_ops > 0;
        let mut row = format!(
            "{{\"call\":{},\"a\":{},\"b\":{},\"sel_b\":[{},{},{}],\"sel_inc\":[{},{},{}],\"sel_est\":[{},{},{}],\"digest_b_counted\":\"{:016x}\",\"digest_inc\":\"{:016x}\",\"digest_est\":\"{:016x}\",\"c_b\":{},\"c_est\":{},\"decompositions_b\":{}",
            c[0] - 1, a, b,
            sbc.pivot_init_ops, sbc.pivot_compare_ops, sbc.pivot_update_ops,
            sbi.pivot_init_ops, sbi.pivot_compare_ops, sbi.pivot_update_ops,
            sbe.pivot_init_ops, sbe.pivot_compare_ops, sbe.pivot_update_ops,
            sbc.pivot_digest, sbi.pivot_digest, sbe.pivot_digest,
            sb.total_muls, sbe.total_muls, ob.3.len()
        );
        if contested {
            c[1] += 1;
            let (o1, s1) = run(
                &comps,
                Pivot::Counted(
                    PivotRule::SparsestIncremental,
                    PivotCheck {
                        verify: true,
                        second_best_at: Some(0),
                        ..Default::default()
                    },
                ),
            );
            let d1 = s1.pivot_digest != sbc.pivot_digest;
            c[9] += u64::from(d1);
            c[10] += u64::from(o1 == ob);
            c[11] += u64::from(phases(&s1) != phases(&sb));
            c[17] += u64::from(s1.echelon_muls != sb.echelon_muls);
            let (_o2, s2) = run(
                &comps,
                Pivot::Counted(
                    PivotRule::SparsestIncremental,
                    PivotCheck {
                        verify: true,
                        perturb_count: true,
                        ..Default::default()
                    },
                ),
            );
            c[12] += u64::from(s2.pivot_count_mismatches > 0);
            row += &format!(
                ",\"nc1_digest\":\"{:016x}\",\"nc1_digest_changed\":{},\"nc1_output_same\":{},\"nc1_echelon_muls\":{},\"b_echelon_muls\":{},\"nc2_mismatches\":{}",
                s1.pivot_digest, d1, o1 == ob, s1.echelon_muls, sb.echelon_muls, s2.pivot_count_mismatches
            );
        }
        if !ob.3.is_empty() {
            c[13] += 1;
            let mut cs = comps.clone();
            let key = *cs[0].keys().min().expect("nonempty equation");
            let v = cs[0][&key] % p;
            let mut w = (v + 1) % p;
            if w == 0 {
                w = (v + 2) % p;
            }
            cs[0].insert(key, w);
            let (o3, _s3) = run(&cs, Pivot::Counted(PivotRule::SparsestRecount, verify));
            let changed = o3.3 != ob.3;
            c[14] += u64::from(changed);
            row += &format!(",\"nc3_changed\":{changed},\"nc3_decompositions\":{}", o3.3.len());
        }
        row.push('}');
        rows.push(row);
    }
    // NC3 needs calls with a non-empty decomposition set (about 1 % of
    // targets decompose): a second target stream from the nonce, drawn until
    // `--nc3` such calls are found (at most 200,000 targets).
    let want = args
        .iter()
        .position(|a| a == "--nc3")
        .map_or(0, |i| args[i + 1].parse::<u64>().expect("--nc3"));
    let mut vr3 = StdRng::seed_from_u64(nonce ^ 0x4E43_3300);
    let mut nc3_rows = Vec::new();
    let (mut drawn, mut found, mut changed_bc, mut changed_be) = (0u64, 0u64, 0u64, 0u64);
    while found < want && drawn < 200_000 {
        let (a, b) = (vr3.gen_range(0..n), vr3.gen_range(1..n));
        let cv = ki.inst.curve();
        let r = cv.add(&cv.mul(&ki.inst.g, a), &cv.mul(&ki.inst.q, b));
        let Some(z_r) = ki.z_of_x(&cv.f, &r.x) else {
            continue;
        };
        drawn += 1;
        let mut fp = 0u64;
        let comps = v4_system(&pre, &cv.f, &z_r, &mut fp);
        let tseed: u64 = vr3.gen();
        let run = |cs: &[HashMap<[u8; 4], u64>], pv: Pivot| -> Out {
            let mut tr = StdRng::seed_from_u64(tseed);
            let mut st = QuarticSolve::default();
            let (s, d, _info) = solve_weighted_chain_with(cs, MACAULAY_BOUND, p, &mut tr, &mut st, pv);
            let lb = lift_back_v4(&ki, &cv, s.as_deref().unwrap_or(&[]), &r, &mut tr);
            (s, d, st.outcome.clone(), lb.decompositions)
        };
        let ob = run(&comps, Pivot::Uncounted { sparsest: true });
        if ob.3.is_empty() {
            continue;
        }
        found += 1;
        let obc = run(&comps, Pivot::Counted(PivotRule::SparsestRecount, verify));
        let mut cs = comps.clone();
        let key = *cs[0].keys().min().expect("nonempty equation");
        let v = cs[0][&key] % p;
        let mut w = (v + 1) % p;
        if w == 0 {
            w = (v + 2) % p;
        }
        cs[0].insert(key, w);
        let o3 = run(&cs, Pivot::Counted(PivotRule::SparsestRecount, verify));
        let o4 = run(&cs, Pivot::Counted(PivotRule::SparsestEstimate, PivotCheck::default()));
        let (c1, c2) = (o3.3 != ob.3, o4.3 != ob.3);
        changed_bc += u64::from(c1);
        changed_be += u64::from(c2);
        nc3_rows.push(format!(
            "{{\"target\":{drawn},\"a\":{a},\"b\":{b},\"decompositions\":{},\"b_counted_equals_b\":{},\"perturbed_decompositions_b_counted\":{},\"changed_b_counted\":{c1},\"changed_b2\":{c2}}}",
            ob.3.len(),
            obc == ob,
            o3.3.len()
        ));
    }
    let nc3 = format!(
        "{{\"wanted\":{want},\"targets_drawn\":{drawn},\"nonempty\":{found},\"changed_b_counted\":{changed_bc},\"changed_b2\":{changed_be},\"rows\":[{}]}}",
        nc3_rows.join(",")
    );
    println!(
        "{{\"schema\":\"e33val-controls/v1\",\"nc3_stream\":{nc3},\"p\":{p},\"seed\":{seed},\"nonce\":{nonce},\"calls\":{},\"contested\":{},\"a_out_eq_b\":{},\"b_counted_out_eq_b\":{},\"b_counted_phases_eq_b\":{},\"inc_out_eq_b\":{},\"inc_phases_eq_b\":{},\"inc_digest_eq_b_counted\":{},\"est_out_eq_b\":{},\"est_digest_eq_b_counted\":{},\"count_mismatches_b_counted_and_inc\":{},\"nc1_second_best_digest_changed\":{},\"nc1_output_same\":{},\"nc1_phase_counts_changed\":{},\"nc1_echelon_muls_changed\":{},\"nc2_off_by_one_flagged\":{},\"nc3_nonempty\":{},\"nc3_changed\":{},\"per_call\":[{}]}}",
        c[0], c[1], c[15], c[2], c[3], c[4], c[5], c[6], c[7], c[16], c[8], c[9], c[10], c[11], c[17], c[12], c[13], c[14],
        rows.join(",")
    );
}
