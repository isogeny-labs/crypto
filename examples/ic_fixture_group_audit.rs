//! Finite curve-membership audit of the original Boolean-chain fixtures.
//! Curve lifts are not subgroup-membership or complete-decomposition claims.
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[allow(dead_code)]
#[path = "ic_algebraic_followup/exact.rs"]
mod exact;
fn span(bs: &[u64]) -> Vec<u64> {
    let mut xs = vec![0];
    for &b in bs {
        let len = xs.len();
        for i in 0..len {
            xs.push(xs[i] ^ b);
        }
    }
    xs.sort_unstable();
    assert_eq!(xs.len(), xs.iter().collect::<BTreeSet<_>>().len());
    xs
}
fn trace(gf: &Gf2, mut x: u64) -> u64 {
    let mut t = 0;
    for _ in 0..gf.n {
        t ^= x;
        x = gf.sqr(x);
    }
    assert!(t <= 1);
    t
}
fn lifts(gf: &Gf2, quad: &exact::Quadratic<'_>, x: u64) -> Vec<u64> {
    let rhs = gf.mul(gf.sqr(x), x) ^ 1;
    let roots = quad.roots(1, x, rhs).unwrap();
    assert!(roots.iter().all(|&y| gf.sqr(y) ^ gf.mul(x, y) == rhs));
    if x == 0 {
        assert_eq!(roots, vec![1]);
    } else {
        let t = trace(gf, x ^ gf.sqr(gf.inv(x)));
        assert_eq!(roots.len(), if t == 0 { 2 } else { 0 });
    }
    roots
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 1, "original input.jsonl");
    let source = std::fs::read(&args[0]).unwrap();
    let records: Vec<Value> = std::str::from_utf8(&source)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(records.len(), 7);
    let mut draws = vec![];
    let (mut unsat, mut unsat_liftable, mut sat, mut sat_liftable) = (0u64, 0u64, 0u64, 0u64);
    for r in records {
        let n = r["n"].as_u64().unwrap() as u32;
        assert_eq!(n, 22);
        assert_eq!(r["ell"], 8);
        let irr = find_irreducible_sparse(n).unwrap();
        let gf = Gf2::new(&irr);
        let quad = exact::Quadratic::new(&gf);
        let xr = r["x_r"].as_u64().unwrap();
        let ys = lifts(&gf, &quad, xr);
        let bs: Vec<u64> = serde_json::from_value(r["v_basis"].clone()).unwrap();
        let xs = span(&bs);
        let (mut curve_x, mut curve_points) = (0u64, 0u64);
        for &x in &xs {
            let roots = lifts(&gf, &quad, x);
            curve_x += u64::from(!roots.is_empty());
            curve_points += roots.len() as u64;
        }
        let is_sat = r["solutions"].as_u64().unwrap() > 0;
        if is_sat {
            sat += 1;
            sat_liftable += u64::from(!ys.is_empty());
        } else {
            unsat += 1;
            unsat_liftable += u64::from(!ys.is_empty());
        }
        draws.push(json!({"draw":r["draw"],"n":n,"ell":8,
            "field_modulus":{"degree":irr.degree,"low_terms":irr.low_terms},
            "v_basis":bs,"x_r":xr,"boolean_chain_solutions":r["solutions"],
            "target_curve_y_roots":ys,"target_curve_liftable":!ys.is_empty(),
            "candidate_abscissae":xs.len(),"curve_abscissae_including_zero":curve_x,
            "curve_points_including_two_torsion":curve_points,"subgroup_membership":null}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"schema":"ic-fixture-curve-membership/v1",
        "status":"enumerated_and_curve_equations_checked","curve_equation":"y^2+x*y=x^3+1",
        "input_sha256":format!("{:x}",Sha256::digest(&source)),"draws":draws,
        "unsat_draws":unsat,"unsat_curve_liftable":unsat_liftable,"sat_draws":sat,"sat_curve_liftable":sat_liftable,
        "scope":"Curve membership of frozen Boolean fixtures only; subgroup membership and complete point decompositions remain untested."})).unwrap());
}
