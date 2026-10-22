//! Canonical quartic twist S5 pilot on frozen L0 query traces.
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{
    base_quadruples, mitm_decompositions, solve_s5_subspace, Curve4, Pt4, QuarticSolve,
    SymmetrisedS5, E4,
};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

fn u(v: &Value) -> u64 {
    v.as_u64().expect("u64")
}
fn a4(v: &Value) -> E4 {
    let a = v.as_array().expect("F4 element");
    E4([u(&a[0]), u(&a[1]), u(&a[2]), u(&a[3])])
}
fn point(x: &Value, y: &Value) -> Pt4 {
    Pt4 {
        x: a4(x),
        y: a4(y),
        inf: false,
    }
}

fn direct_sign_check(c: &Curve4, base: &[Pt4], r: &Pt4, xs: &[u64; 4]) -> bool {
    let mut ids = Vec::new();
    for &x in xs {
        let Some(i) = base.iter().position(|p| p.x == E4([x, 0, 0, 0])) else {
            return false;
        };
        ids.push(i);
    }
    if ids
        .iter()
        .copied()
        .collect::<std::collections::HashSet<_>>()
        .len()
        != 4
    {
        return false;
    }
    for mask in 0..16 {
        let mut sum = Pt4::INF;
        for (j, &i) in ids.iter().enumerate() {
            let q = if mask & (1 << j) == 0 {
                base[i]
            } else {
                c.neg(&base[i])
            };
            sum = c.add(&sum, &q);
        }
        if sum == *r {
            return true;
        }
    }
    false
}

fn run(p: u64, raw: &str) -> Value {
    let records: Vec<Value> = fs::read_to_string(raw)
        .expect("frozen raw trace")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .collect();
    let rec = records
        .iter()
        .find(|r| u(&r["p"]) == p && u(&r["seed"]) == 0 && r["line_powers"] == json!([0]))
        .expect("registered L0 record");
    let b0 = u(&rec["b0"]);
    let c = Curve4::with_params(p, E4::ZERO, E4([0, 0, 0, b0]));
    let g = point(&rec["generator_x"], &rec["generator_y"]);
    let q = point(&rec["target_x"], &rec["target_y"]);
    let base: Vec<Pt4> = rec["base_coordinates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|z| point(&z[0], &z[1]))
        .collect();
    assert!(c.on_curve(&g) && c.on_curve(&q));
    assert_eq!(c.mul(&g, 271), q);
    assert!(base.iter().all(|z| c.on_curve(z) && z.x.in_fp()));
    let trace = rec["query_trace"].as_array().unwrap();
    let first_success = trace
        .iter()
        .position(|z| u(&z[2]) > 0)
        .expect("success target");
    let first_failure = trace
        .iter()
        .position(|z| u(&z[2]) == 0)
        .expect("failure target");
    let mut rng = StdRng::seed_from_u64(0x5355_4253_5041_4345 ^ p);
    c.f.reset_muls();
    let started = Instant::now();
    eprintln!("p={p} S5 symmetrisation starting");
    let pre = SymmetrisedS5::precompute(&c, &mut rng);
    let pre_ms = started.elapsed().as_secs_f64() * 1e3;
    eprintln!(
        "p={p} S5 symmetrisation complete: {:.1} ms, {} Fp muls",
        pre_ms, pre.precompute_muls
    );
    let _ = io::stderr().flush();
    let mut rows = Vec::new();
    for (kind, index) in [
        ("first_success", first_success),
        ("first_failure", first_failure),
    ] {
        let z = &trace[index];
        let (a, b) = (u(&z[0]), u(&z[1]));
        let residual = c.add(&c.mul(&g, a), &c.mul(&q, b));
        let oracle = mitm_decompositions(&c, &base, &residual);
        let mut st = QuarticSolve::default();
        c.f.reset_muls();
        eprintln!("p={p} {kind} query {index} S5 solve starting");
        let started = Instant::now();
        let solutions = solve_s5_subspace(&c, &pre, &residual.x, &mut rng, &mut st, false);
        let elapsed_ms = started.elapsed().as_secs_f64() * 1e3;
        let solved = solutions.is_some();
        let candidates: BTreeSet<[u64; 4]> = solutions
            .as_ref()
            .map(|s| base_quadruples(s, &base))
            .unwrap_or_default();
        let verified = candidates
            .iter()
            .all(|xs| direct_sign_check(&c, &base, &residual, xs));
        let agree = solved && candidates == oracle && verified;
        eprintln!(
            "p={p} {kind} query {index}: outcome={} agree={agree} {:.1} ms",
            st.outcome, elapsed_ms
        );
        rows.push(json!({"kind":kind,"query_index":index,"a":a,"b":b,
            "recorded_decompositions":u(&z[2]),"oracle_quadruples":oracle,
            "solver_quadruples":candidates,"solver_returned":solved,
            "all_solver_candidates_group_verified":verified,"agree":agree,
            "solve":st,"wall_ms_l0":elapsed_ms}));
    }
    json!({"p":p,"b0":b0,"seed":0,"r":u(&rec["n"]),
        "base_abscissae":base.len(),"precompute_muls":pre.precompute_muls,
        "precompute_ms_l0":pre_ms,"monomials":pre.monos.len(),"rows":rows})
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 4, "usage: quartic_twist_s5_pilot --p P RAW");
    assert_eq!(args[1], "--p");
    let p: u64 = args[2].parse().expect("p");
    assert!([37, 73, 97].contains(&p));
    println!("{}", run(p, &args[3]));
}
