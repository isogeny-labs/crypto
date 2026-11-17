//! E33 (§9.35, G2.124) checks through the public API: the same calls as the
//! unit test `klein_v4::tests::e33_counted_pivot_selection_is_bit_identical`
//! (E25 `p = 101` seed 2 and `p = 149` seed 1, 16 targets each), runnable
//! from the release library without a test build.
//!
//! On every call: A counted, B counted and B′ return the uncounted solver's
//! solutions, degree and decomposition set with the same phase counts; B″
//! returns the same output; B counted and B′ choose the same pivots
//! (digests); B′'s maintained counts equal a recount; B's initial pass is
//! `rows · cols`; the uncounted solvers serialise no E33 field.  Negative
//! controls on every call with a contested column: the second-best pivot
//! once (digest must change, output must not) and a count off by one (the
//! recount check must flag it).  Exits non-zero on any failure.
//!
//! ```text
//! ./target/release/examples/e33_checks
//! ```

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

fn key(s: &QuarticSolve) -> ([u64; 9], (usize, usize, usize), String) {
    (
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
        ],
        (s.rows, s.cols, s.pivots),
        s.outcome.clone(),
    )
}

fn main() {
    let mut fails: Vec<String> = Vec::new();
    let mut check = |ok: bool, what: String| {
        if !ok {
            eprintln!("FAIL: {what}");
            fails.push(what);
        }
    };
    let (mut calls, mut contested, mut digest_flagged, mut count_flagged) = (0, 0, 0, 0);
    for (p, seed) in [(101u64, 2u64), (149, 1)] {
        let ki = generate_gls_klein_instance(p, seed, 16).unwrap();
        let curve = ki.inst.curve();
        let f = &curve.f;
        let mut rng = StdRng::seed_from_u64(p ^ 0xE33);
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
            let tseed: u64 = rng.gen();
            let run = |pivot: Pivot| -> Run {
                let mut tr = StdRng::seed_from_u64(tseed);
                let mut st = QuarticSolve::default();
                let (s, d, info) =
                    solve_weighted_chain_with(&comps, MACAULAY_BOUND, p, &mut tr, &mut st, pivot);
                let lb = lift_back_v4(&ki, &curve, s.as_deref().unwrap_or(&[]), &r, &mut tr);
                (s, d, lb.decompositions, st, info.fell_back)
            };
            let co = |rule: PivotRule| {
                Pivot::Counted(
                    rule,
                    PivotCheck {
                        verify: true,
                        ..Default::default()
                    },
                )
            };
            let ua = run(Pivot::Uncounted { sparsest: false });
            let ub = run(Pivot::Uncounted { sparsest: true });
            let ac = run(co(PivotRule::Lowest));
            let bc = run(co(PivotRule::SparsestRecount));
            let bi = run(co(PivotRule::SparsestIncremental));
            let be = run(co(PivotRule::SparsestEstimate));
            let at = format!("p = {p} call {calls}");
            for (x, base, what) in [
                (&ac, &ua, "A counted"),
                (&bc, &ub, "B counted"),
                (&bi, &ub, "B′"),
            ] {
                check(
                    (&x.0, x.1, &x.2) == (&base.0, base.1, &base.2),
                    format!("{at}: {what} output"),
                );
                check(
                    key(&x.3) == key(&base.3),
                    format!("{at}: {what} phase counts"),
                );
            }
            check(
                (&be.0, be.1, &be.2) == (&ua.0, ua.1, &ua.2),
                format!("{at}: B″ output"),
            );
            for old in [&ua.3, &ub.3] {
                let js = serde_json::to_string(old).unwrap();
                check(
                    !js.contains("pivot_") && !js.contains("lead_scan"),
                    format!("{at}: uncounted solver emits an E33 field"),
                );
            }
            check(
                ac.3.pivot_init_ops + ac.3.pivot_compare_ops + ac.3.pivot_update_ops == 0,
                format!("{at}: A counted charged selection work"),
            );
            check(
                bc.3.pivot_digest == bi.3.pivot_digest && bc.3.pivot_digest != 0,
                format!("{at}: B′ digest differs from B's"),
            );
            check(
                bi.3.pivot_count_mismatches == 0 && bc.3.pivot_count_mismatches == 0,
                format!("{at}: maintained counts differ from a recount"),
            );
            if !bc.4 {
                check(
                    bc.3.pivot_init_ops == (bc.3.rows * bc.3.cols) as u64,
                    format!("{at}: B initial pass is not rows·cols"),
                );
            }
            check(
                bi.3.pivot_init_ops < bc.3.pivot_init_ops,
                format!("{at}: B′ initial not below B's"),
            );
            check(
                bc.3.pivot_compare_ops == bi.3.pivot_compare_ops
                    && bc.3.lead_scan_reads == bi.3.lead_scan_reads,
                format!("{at}: comparisons or lead scans differ between B and B′"),
            );
            if bc.3.pivot_compare_ops > 0 {
                contested += 1;
                let bad = run(Pivot::Counted(
                    PivotRule::SparsestIncremental,
                    PivotCheck {
                        verify: true,
                        second_best_at: Some(0),
                        ..Default::default()
                    },
                ));
                if bad.3.pivot_digest != bc.3.pivot_digest {
                    digest_flagged += 1;
                }
                check(
                    (&bad.0, bad.1, &bad.2) == (&ua.0, ua.1, &ua.2),
                    format!("{at}: a second-best pivot changed the output"),
                );
                let bad = run(Pivot::Counted(
                    PivotRule::SparsestIncremental,
                    PivotCheck {
                        verify: true,
                        perturb_count: true,
                        ..Default::default()
                    },
                ));
                if bad.3.pivot_count_mismatches > 0 {
                    count_flagged += 1;
                }
            }
            calls += 1;
        }
    }
    check(calls == 32, format!("calls {calls}"));
    check(contested > 0, "no contested column".into());
    check(
        digest_flagged == contested,
        format!("second-best control detected {digest_flagged}/{contested}"),
    );
    check(
        count_flagged == contested,
        format!("off-by-one control detected {count_flagged}/{contested}"),
    );
    println!(
        "{{\"calls\": {calls}, \"contested\": {contested}, \"second_best_detected\": {digest_flagged}, \"off_by_one_detected\": {count_flagged}, \"failures\": {}}}",
        fails.len()
    );
    if !fails.is_empty() {
        std::process::exit(1);
    }
}
