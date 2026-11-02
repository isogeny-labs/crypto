//! E36 (§9.41, G2.154) checks and negative controls through the public
//! API, runnable from the release library without a test build (E33's
//! `e33_checks` pattern: E25 `p = 101` seed 2 and `p = 149` seed 1, 16
//! targets each).
//!
//! On every call: F, S, H, K, L and A's rule through E36's echelon return
//! the uncounted solver A's solutions, degree and decomposition set; the
//! A-rule control has A's phase counts; F and L charge no selection work.
//! Negative controls: (1) E31's perturbed coefficient (one coefficient of
//! the first equation moved by one) must change every non-empty solution
//! set under each new rule; (2) S's and H's inverse permutation corrupted by
//! one swap must change the output (or force the charged fallback) on
//! every call; (3) on calls with a contested column, F's and L's pivot
//! digests must differ from A's on at least one call each.  Exits non-zero
//! on any failure.
//!
//! ```text
//! ./e36_checks
//! ```

use std::collections::HashMap;

use crypto_lib::cryptanalysis::chain_pivot_rules::{take_diag, ArrivalRule, RuleCheck};
use crypto_lib::cryptanalysis::gaudry_quartic::{PivotCheck, PivotRule, QuarticSolve};
use crypto_lib::cryptanalysis::klein_v4::{
    lift_back_v4, solve_weighted_chain_with, v4_system, Pivot, SymmetrisedV4, MACAULAY_BOUND,
};
use crypto_lib::cryptanalysis::quartic_folds::generate_gls_klein_instance;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

type Run = (
    Option<Vec<[u64; 4]>>,
    u16,
    Vec<[u64; 4]>,
    QuarticSolve,
    bool,
);

const RULES: [(ArrivalRule, &str); 5] = [
    (ArrivalRule::Fifo, "F"),
    (ArrivalRule::Static, "S"),
    (ArrivalRule::FifoStatic, "H"),
    (ArrivalRule::EstimateK2, "K"),
    (ArrivalRule::Lifo, "L"),
];

fn phases(s: &QuarticSolve) -> [u64; 9] {
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
    ]
}

fn main() {
    let mut fails: Vec<String> = Vec::new();
    let mut check = |ok: bool, what: String| {
        if !ok {
            eprintln!("FAIL: {what}");
            fails.push(what);
        }
    };
    let (mut calls, mut nonempty, mut contested) = (0usize, 0usize, 0usize);
    let mut perturb_detected = [0usize; 5];
    let mut inverse_detected = [0usize; 2];
    let mut digest_differs = [0usize; 5];
    for (p, seed) in [(101u64, 2u64), (149, 1)] {
        let ki = generate_gls_klein_instance(p, seed, 16).unwrap();
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(p ^ 0xE36);
        let pre = SymmetrisedV4::precompute(&ki, &curve, &mut rng, 4).unwrap();
        let n = ki.inst.n;
        let mut done = 0;
        while done < 16 {
            let (a, b) = (rng.gen_range(0..n), rng.gen_range(1..n));
            let r = curve.add(&curve.mul(&ki.inst.g, a), &curve.mul(&ki.inst.q, b));
            let Some(z_r) = ki.z_of_x(f, &r.x) else {
                continue;
            };
            done += 1;
            let mut fp = 0u64;
            let comps = v4_system(&pre, f, &z_r, &mut fp);
            // E31's control: one coefficient of the first equation moved by one.
            let mut bent: Vec<HashMap<[u8; 4], u64>> = comps.clone();
            if let Some(eq) = bent.first_mut() {
                let mut keys: Vec<[u8; 4]> = eq.keys().copied().collect();
                keys.sort_unstable();
                if let Some(k) = keys.first() {
                    let v = eq[k];
                    eq.insert(*k, (v + 1) % p);
                }
            }
            let tseed: u64 = rng.gen();
            let run = |cs: &[HashMap<[u8; 4], u64>], pivot: Pivot| -> Run {
                let mut tr = StdRng::seed_from_u64(tseed);
                let mut st = QuarticSolve::default();
                let (s, d, info) =
                    solve_weighted_chain_with(cs, MACAULAY_BOUND, p, &mut tr, &mut st, pivot);
                let lb = lift_back_v4(&ki, &curve, s.as_deref().unwrap_or(&[]), &r, &mut tr);
                (s, d, lb.decompositions, st, info.fell_back)
            };
            let at = format!("p = {p} call {calls}");
            let ua = run(&comps, Pivot::Uncounted { sparsest: false });
            let ac = run(&comps, Pivot::Counted(PivotRule::Lowest, PivotCheck::default()));
            let _ = take_diag();
            let lo = run(&comps, Pivot::Rule(ArrivalRule::Lowest, RuleCheck::default()));
            let dg = take_diag();
            check(
                (&lo.0, lo.1, &lo.2) == (&ua.0, ua.1, &ua.2),
                format!("{at}: A's rule via E36 output"),
            );
            check(
                phases(&lo.3) == phases(&ac.3)
                    && lo.3.pivot_digest == ac.3.pivot_digest
                    && lo.3.lead_scan_reads == ac.3.lead_scan_reads,
                format!("{at}: A's rule via E36 phase counts / digest / lead scans"),
            );
            let has_sols = ua.0.as_ref().is_some_and(|s| !s.is_empty());
            if has_sols {
                nonempty += 1;
            }
            if dg.contested > 0 {
                contested += 1;
            }
            for (i, (rule, name)) in RULES.iter().enumerate() {
                let x = run(&comps, Pivot::Rule(*rule, RuleCheck::default()));
                check(
                    (&x.0, x.1, &x.2) == (&ua.0, ua.1, &ua.2),
                    format!("{at}: {name} output"),
                );
                if matches!(rule, ArrivalRule::Fifo | ArrivalRule::Lifo) {
                    check(
                        x.3.pivot_init_ops + x.3.pivot_compare_ops + x.3.pivot_update_ops == 0,
                        format!("{at}: {name} charged selection work"),
                    );
                }
                if x.3.pivot_digest != ac.3.pivot_digest {
                    digest_differs[i] += 1;
                }
                if has_sols {
                    let y = run(&bent, Pivot::Rule(*rule, RuleCheck::default()));
                    if y.0 != x.0 {
                        perturb_detected[i] += 1;
                    }
                }
            }
            for (j, rule) in [ArrivalRule::Static, ArrivalRule::FifoStatic]
                .into_iter()
                .enumerate()
            {
                let bad = run(
                    &comps,
                    Pivot::Rule(
                        rule,
                        RuleCheck {
                            corrupt_inverse: true,
                        },
                    ),
                );
                if (&bad.0, bad.1, &bad.2) != (&ua.0, ua.1, &ua.2) || bad.4 != ua.4 {
                    inverse_detected[j] += 1;
                }
            }
            calls += 1;
        }
    }
    check(calls == 32, format!("calls {calls}"));
    check(nonempty > 0, "no call with a non-empty solution set".into());
    for (i, (_, name)) in RULES.iter().enumerate() {
        check(
            perturb_detected[i] == nonempty,
            format!("{name}: perturbed coefficient detected {}/{nonempty}", perturb_detected[i]),
        );
    }
    for (j, name) in ["S", "H"].iter().enumerate() {
        check(
            inverse_detected[j] == calls,
            format!("{name}: corrupt inverse detected {}/{calls}", inverse_detected[j]),
        );
    }
    check(
        contested == 0 || (digest_differs[0] > 0 && digest_differs[4] > 0),
        format!("F/L digests never differ from A's on {contested} contested calls"),
    );
    println!(
        "{{\"calls\": {calls}, \"nonempty\": {nonempty}, \"contested\": {contested}, \"perturbed_detected\": {perturb_detected:?}, \"corrupt_inverse_detected\": {inverse_detected:?}, \"digest_differs_from_a\": {digest_differs:?}, \"rules\": [\"F\", \"S\", \"H\", \"K\", \"L\"], \"failures\": {}}}",
        fails.len()
    );
    if !fails.is_empty() {
        std::process::exit(1);
    }
}
