use prime31_rr9_and_closures::{
    f4_fp::{solve_visit, F4Options, Ordering},
    prime31_field::{build_factor_base, CurveArith, PRIME31},
    rr9_remainder_tower::{
        reset_and_assert_zero_forbidden, universal_degree8_multiplicity_strata,
    },
};
use std::time::{Duration, Instant};

fn main() {
    reset_and_assert_zero_forbidden();
    let mut ar = CurveArith::default();
    let (_base4, h4_vec) = build_factor_base(PRIME31, 4, &mut ar);
    let strata4 = universal_degree8_multiplicity_strata(4);
    println!("B=4 strata count = {}", strata4.len());
    for s in &strata4 {
        let t0 = Instant::now();
        let (nv, ne, in_deg, polys) =
            prime31_rr9_and_closures::rr9_remainder_tower::debug_yun_stratum(
                PRIME31.p,
                PRIME31.a,
                PRIME31.g,
                &h4_vec,
                s,
            );
        let opts = F4Options::new(Ordering::Grevlex, 8).with_budget(Duration::from_secs(5));
        let domains = vec![None; nv];
        let rep = solve_visit(&polys, nv, PRIME31.p, &opts, &domains, |_| false).unwrap();
        println!(
            "  stratum={s:?} nv={nv} ne={ne} in_deg={in_deg} complete={} sdeg={} cols={} f4_runs={} uni={} ff={} ops={} ms={:.1}",
            rep.complete,
            rep.solving_degree,
            rep.max_cols,
            rep.f4_runs,
            rep.factored_univariates,
            rep.full_field_branches,
            rep.field_ops,
            t0.elapsed().as_secs_f64() * 1e3
        );
    }
}
