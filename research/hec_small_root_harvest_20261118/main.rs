mod harvest;
#[path = "../hec_centered_residual_20261118/centered.rs"] mod centered;
#[path = "../hec_compact_hits_20261118/compact.rs"] mod compact;
use hec_native_ic::{
    counted,
    instance::pick_instance,
    jac::{Curve, Div},
    rng::Rng,
};
use std::{hint::black_box, time::Instant};
const L: u64 = 239753;
#[path = "../hec_carried_pencil_20261118/buckets.rs"]
mod buckets;
#[path = "../hec_carried_pencil_20261118/carried.rs"]
mod carried;

#[path = "../hec_delayed_reductions_20261118/dag.rs"]
mod dag;
#[path = "solver.rs"]
mod delayed;

#[path = "../hec_signed_certificate_20261118/powers.rs"]
mod powers;
#[path = "../hec_singleton_recipes_20261118/recipe.rs"]
mod recipe;
mod phases { #[inline(always)] pub fn mark(_: &str) {} }
#[path = "../hec_bounded_buckets_20261118/bounded.rs"]
mod bounded;
#[path = "../hec_leading_quotient_20261118/quotient.rs"]
mod quotient;
#[path = "../hec_leading_quotient_20261118/lift.rs"]
mod lift;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/finish.rs"]
mod finish;
#[path = "../hec_residual_span_20261116/native-pencil-images/images.rs"]
mod images;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/linear.rs"]
mod linear;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/pencil.rs"]
mod pencil;
#[path = "../hec_place_pencil_20261118/place.rs"]
mod place;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/prepared.rs"]
mod prepared;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/public.rs"]
mod public;
#[path = "../hec_slope_recurrence_20261118/slopes.rs"]
mod slopes;
#[path = "../hec_residual_span_20261116/native-lazy-finisher/small.rs"]
mod small;
#[path = "../hec_carried_pencil_20261118/solver.rs"]
mod solver;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/table_oracle.rs"]
mod table_oracle;
#[path = "../hec_residual_span_20261116/native-prepared-solvers/tail.rs"]
mod tail;
#[path = "../hec_residual_span_20261116/native-deferred-membership/validate.rs"]
mod validate;
#[path = "../hec_wide_sieve_20261118/wide.rs"]
mod wide;
fn main() {
    for (g,p) in [(2,127),(2,257),(2,1021),(2,4093),(3,73),(3,127),(3,257),(3,383),(4,37),(4,43),(4,59),(4,67)] {
        counted::disable();
        let Some(inst)=pick_instance(p,g) else {println!("PANEL g={g} p={p} status=unselected");continue;};
        let ell=inst.ell;
        // Independently enumerate every quadratic on each genus-three field.
        if g == 3 {
            let fp = &inst.curve.fp;
            let roots = prepared::RootTable::build(fp);
            for b in 0..p {
                for c in 0..p {
                    let u = hec_native_ic::poly::Poly::from_coeffs(&[c,b,1]);
                    let stats = &mut table_oracle::OracleStats::default();
                    assert_eq!(centered::recover(&u,fp,stats,&roots),table_oracle::split(&u,fp,stats,&roots));
                }
            }
        }

        if !validate::eligible(&inst.curve,&inst.d1,&inst.d1,ell) {
            println!("PANEL g={g} p={p} c={} ell={ell} status=ineligible",inst.c);
            continue;
        }
        println!("PANEL g={g} p={p} c={} ell={ell} status=selected",inst.c);
        let c = &inst.curve;
        let mut rng = Rng::new(2026121307 ^ p ^ ((g as u64) << 32));
        for case in 0..4 {
            let d = c.mul(&inst.d1, 1 + rng.below(ell - 1));
            let truth = 1 + rng.below(ell - 1);
            let target = c.mul(&d, truth);
            let seed = 2026121311 ^ p ^ ((g as u64) << 32) ^ case as u64;
            let run = |arm: usize, audit: bool| {
                let powers = if arm >= 1 {
                    let table = powers::Powers::checked(c,&d,&target,ell,if g==2 {3} else {2},audit).unwrap();
                    if audit {
                        assert_eq!(lift::add(c,&d,&c.neg(&d),2,true),c.zero());
                        assert_eq!(lift::double(c,&c.zero(),2,true),c.zero());
                        assert_eq!(lift::add(c,&d,&d,2,true),c.add(&d,&d));
                        assert!(validate::eligible(c,&d,&target,ell));
                        assert!(powers::Powers::checked(c,&c.zero(),&target,ell,3,audit).is_none());
                        assert!(powers::Powers::checked(c,&d,&target,ell*2,3,audit).is_none());
                        assert!(powers::Powers::checked(c,&d,&target,3,3,audit).is_none());
                        for n in [0,1,ell/2,ell-1,truth,(truth+1)%ell] {
                            assert_eq!(table.product(c,n,true),c.mul(&d,n));
                        }
                        assert_ne!(table.product(c,(truth+1)%ell,true),target);
                    }
                    Some(table)
                } else {
                    assert!(validate::eligible(c,&d,&target,ell));
                    None
                };

                phases::mark("catalogue");
                let prep = prepared::prepare(c, audit);
                if arm == 0 {
                    let r = pencil::solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        pencil::Options {
                            normalize: true,
                            finish: false,
                            seed,
                        },
                    );
                    (r.k, r.anchors, 0, r.accepted, 0, 0, 0,0,0)
                } else {
                    phases::mark("initial_rows");
                    let solve = if arm == 9 { delayed::solve::<true> } else { delayed::solve::<false> }; let r = solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        powers.as_ref(), true,
                        delayed::Options {
                            layout: if g==2 {2} else {0},
                            integrated: true,
                            delay: true, defer: 2,
                            normalize: true,
                            finish: false,
                            seed,
                            certificate: false,
                            rolling: true,
                            carry: true,
                            indexed: true,
                            mode: if g == 3 { 1 } else { 3 },
                            eager: false,
                            cutoff: if g != 3 { 2 } else { 0 },
                            substitute: true,
                            fused: true,
                        },
                    );
                    (r.k, r.anchors, r.small, r.accepted,r.unready,r.pivots,r.direct,r.saved,r.replayed)
                }
            };
            let reference = run(8, false); let candidate=run(9,false); println!("STOP {g} {p} {case} {} {}",reference.1,candidate.1);
            assert_eq!(candidate.0,reference.0); assert_eq!(candidate.0,run(9,true).0);
            for rep in 0..24 {
                for pass in 0..3 {
                    let arm = [0, 8, 9][(rep + case + pass) % 3];
                    let start = Instant::now();
                    let result = black_box(run(black_box(arm), false));
                    let ns = start.elapsed().as_nanos();
                    assert_eq!(result.0, Some(truth));
                    if arm == 8 { assert_eq!(result, reference); } else if arm==9 { assert_eq!(result,candidate); }
                    assert_eq!(c.mul(&d, result.0.unwrap()), target);
                    println!("WALL {g} {p} {case} {rep} {arm} {ns}");
                }
            }
        }
    }
}
