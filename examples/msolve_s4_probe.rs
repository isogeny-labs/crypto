//! Probe of the published-method decomposer (msolve on the e_i-symmetrised Semaev S4):
//! for a Koblitz curve (a, n), subspace dimension l and random target abscissae, solve the
//! descended system with msolve, extract the roots from the reduced basis, and compare the
//! root set with brute-force enumeration of V^3 against the S4 polynomial.  Reports msolve's
//! instructions retired per call (`/usr/bin/time -l`, so run unsandboxed on macOS).
//! usage: msolve_s4_probe <n> <a> <l> <targets> <seed> [form e|x] [spec k] [skip_truth 0|1] [b]
//! `b` (decimal bit pattern) replaces the curve coefficient b (general-b check: points and truth on y^2+xy=x^3+ax^2+b).
#[path = "../src/bin/ecbench/msolve_s4.rs"]
mod msolve_s4;

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev_s4::symmetrised_s4_eval_b;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{points_with_x, KoblitzCurve};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::time::Instant;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a[1].parse().unwrap();
    let ca: u8 = a[2].parse().unwrap();
    let l: u32 = a[3].parse().unwrap();
    let targets: usize = a[4].parse().unwrap();
    let seed: u64 = a[5].parse().unwrap();
    let tune = msolve_s4::Tune {
        x_form: a.get(6).map(|f| f == "x").unwrap_or(false),
        spec: a.get(7).map(|k| k.parse().unwrap()).unwrap_or(0),
    };
    let skip_truth = a.get(8).map(|f| f == "1").unwrap_or(false);
    let msolve = std::env::var("MSOLVE").unwrap_or_else(|_| "/opt/homebrew/bin/msolve".into());
    let mut kc = KoblitzCurve::new(ca, n).expect("koblitz curve");
    if let Some(bv) = a.get(9) {
        kc.curve.b = F2mElement::from_biguint(&BigUint::from(bv.parse::<u64>().unwrap()), n);
    }
    let irr = kc.curve.irreducible.clone();
    let b = kc.curve.b.clone();
    let mut rng = StdRng::seed_from_u64(seed);
    let el = |x: u64| F2mElement::from_biguint(&BigUint::from(x), n);
    let mut done = 0;
    while done < targets {
        let xr_bits: u64 = rng.gen_range(1..(1u64 << n));
        let xr = el(xr_bits);
        if points_with_x(&kc.curve, &xr).is_empty() {
            continue;
        }
        done += 1;
        // Ground truth: ordered triples in V^3 with S4 = 0.
        let mut truth = 0u64;
        let sz = if skip_truth { 0 } else { 1u64 << l };
        for x1 in 0..sz {
            for x2 in 0..sz {
                for x3 in 0..sz {
                    let (p, q, r) = (el(x1), el(x2), el(x3));
                    let e1 = p.add(&q).add(&r);
                    let e2 = p.mul(&q, &irr).add(&p.mul(&r, &irr)).add(&q.mul(&r, &irr));
                    let e3 = p.mul(&q, &irr).mul(&r, &irr);
                    if symmetrised_s4_eval_b(&e1, &e2, &e3, &xr, &b, &irr).is_zero() {
                        truth += 1;
                    }
                }
            }
        }
        let t0 = Instant::now();
        let res = msolve_s4::solve_s4(&msolve, n, l, &irr, &b, &xr, &format!("p{done}"), tune);
        let wall = t0.elapsed().as_secs_f64();
        match res {
            Ok(o) => {
                let mut got: Vec<[u64; 3]> = o.roots.clone();
                got.sort();
                let agree = (skip_truth || got.len() as u64 == truth) && got.iter().all(|r| {
                    let (p, q, s) = (el(r[0]), el(r[1]), el(r[2]));
                    let e1 = p.add(&q).add(&s);
                    let e2 = p.mul(&q, &irr).add(&p.mul(&s, &irr)).add(&q.mul(&s, &irr));
                    let e3 = p.mul(&q, &irr).mul(&s, &irr);
                    symmetrised_s4_eval_b(&e1, &e2, &e3, &xr, &b, &irr).is_zero()
                });
                println!(
                    "{{\"n\":{n},\"a\":{ca},\"l\":{l},\"target\":{done},\"x_r\":{xr_bits},\"vars\":{},\"eqs\":{},\"gb_len\":{},\"quotient_dim\":{},\"roots\":{},\"truth_ordered_triples\":{truth},\"agree\":{agree},\"msolve_ir\":{},\"msolve_calls\":{},\"refuted\":{},\"form_x\":{},\"spec\":{},\"extraction_ops\":{},\"wall_s\":{wall:.2}}}",
                    o.n_vars, o.n_eqs, o.gb_len, o.quotient_dim, o.roots.len(), o.ir, o.calls, o.refuted_by_substitution, tune.x_form, tune.spec, o.ext_ops
                );
            }
            Err(e) => println!("{{\"n\":{n},\"l\":{l},\"target\":{done},\"error\":{:?},\"truth_ordered_triples\":{truth},\"wall_s\":{wall:.2}}}", e),
        }
    }
}
