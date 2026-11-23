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

mod core;
mod dag;
#[path = "../hec_cross_curve_holdout_20261118/solver.rs"]
mod delayed;

#[path = "../hec_cross_curve_holdout_20261118/powers.rs"]
mod powers;
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
                    (r.k, r.anchors, 0, r.accepted, 0, 0, 0)
                } else {
                    let r = delayed::solve(
                        c,
                        &d,
                        &target,
                        ell,
                        512,
                        audit,
                        &prep,
                        powers.as_ref(), true,
                        delayed::Options {
                            layout: if g==2 && arm>=2 {2} else {0},
                            integrated: arm==3,
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
                    (r.k, r.anchors, r.small, r.accepted,r.unready,r.pivots,r.direct)
                }
            };
            println!("BEGIN g={g} p={p} case={case} ell={ell}");
            let result=run(3,true);
            if let Some(k)=result.0 {
                assert_eq!(c.mul(&d,k),target);
                assert_eq!(k,truth);
            }
            println!("END g={g} p={p} case={case} k={:?} anchors={}",result.0,result.1);
        }
    }
}
