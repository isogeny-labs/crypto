//! R5 probe of the WDSat arm: for a Koblitz curve (a, n), subspace dimension l and random target
//! abscissae, emit the descended S4 system in WDSat's ANF format, run WDSat, and compare its
//! root set with brute-force enumeration of V^3 against the symmetrised S4 polynomial
//! (compared as sorted-unique triples, because `-b` returns one representative per permutation
//! class).  Reports WDSat's instructions retired per call (`/usr/bin/time -l`: run unsandboxed on macOS).
//! usage: wdsat_probe <n> <a> <l> <targets> <seed> <sym 0|1> [dump_dir]
//! With `dump_dir` the ANF files are also written there (sizing pass for WDSat's static config).
//! Env: WDSAT_BIN = the WDSat binary built for this cell.
#[path = "../src/bin/ecbench/wdsat_s4.rs"]
mod wdsat_s4;

use crypto_lib::binary_ecc::F2mElement;
use crypto_lib::cryptanalysis::binary_semaev_s4::symmetrised_s4_eval_b;
use crypto_lib::cryptanalysis::koblitz_index_calculus::{points_with_x, KoblitzCurve};
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::BTreeSet;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let n: u32 = a[1].parse().unwrap();
    let ca: u8 = a[2].parse().unwrap();
    let l: u32 = a[3].parse().unwrap();
    let targets: usize = a[4].parse().unwrap();
    let seed: u64 = a[5].parse().unwrap();
    let sym = a[6] == "1";
    let dump = a.get(7).cloned();
    let bin = std::env::var("WDSAT_BIN").unwrap_or_else(|_| "wdsat_solver".into());
    let kc = KoblitzCurve::new(ca, n).expect("koblitz curve");
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
        if let Some(d) = &dump {
            let em = wdsat_s4::emit(n, l, &irr, &b, &xr);
            std::fs::write(format!("{d}/n{n}a{ca}l{l}_{done}.anf"), em.text).unwrap();
        }
        let mut truth: BTreeSet<[u64; 3]> = BTreeSet::new();
        let mut truth_ordered = 0u64;
        let nobf = std::env::var("WDSAT_NOBF").is_ok(); // R2W: sizing/dump pass, skip brute force
        let sz = if nobf { 0 } else { 1u64 << l };
        for x1 in 0..sz {
            for x2 in 0..sz {
                for x3 in 0..sz {
                    let (p, q, r) = (el(x1), el(x2), el(x3));
                    let e1 = p.add(&q).add(&r);
                    let e2 = p.mul(&q, &irr).add(&p.mul(&r, &irr)).add(&q.mul(&r, &irr));
                    let e3 = p.mul(&q, &irr).mul(&r, &irr);
                    if symmetrised_s4_eval_b(&e1, &e2, &e3, &xr, &b, &irr).is_zero() {
                        truth_ordered += 1;
                        let mut t = [x1, x2, x3];
                        t.sort_unstable();
                        truth.insert(t);
                    }
                }
            }
        }
        if nobf { continue; }
        let res = wdsat_s4::solve_s4(&bin, n, l, &irr, &b, &xr, &format!("p{done}"), wdsat_s4::WdTune { sym, xg: std::env::var("WDSAT_NOX").is_err(), all: std::env::var("WDSAT_FIRST").is_err() });
        match res {
            Ok(o) => {
                let mut got: BTreeSet<[u64; 3]> = BTreeSet::new();
                let mut bad = 0;
                for r in &o.roots {
                    let (p, q, s) = (el(r[0]), el(r[1]), el(r[2]));
                    let e1 = p.add(&q).add(&s);
                    let e2 = p.mul(&q, &irr).add(&p.mul(&s, &irr)).add(&q.mul(&s, &irr));
                    let e3 = p.mul(&q, &irr).mul(&s, &irr);
                    if !symmetrised_s4_eval_b(&e1, &e2, &e3, &xr, &b, &irr).is_zero() {
                        bad += 1;
                    }
                    let mut t = *r;
                    t.sort_unstable();
                    got.insert(t);
                }
                let first = std::env::var("WDSAT_FIRST").is_ok();
                let agree = bad == 0 && if first { got.is_subset(&truth) && (got.is_empty() == truth.is_empty()) } else { got == truth };
                println!(
                    "{{\"n\":{n},\"a\":{ca},\"l\":{l},\"sym\":{sym},\"target\":{done},\"x_r\":{xr_bits},\"vars\":{},\"eqs\":{},\"inexact\":{},\"models\":{},\"unique_sorted\":{},\"truth_sorted\":{},\"truth_ordered\":{truth_ordered},\"invalid_models\":{bad},\"agree\":{agree},\"wdsat_ir\":{},\"calls\":{},\"refuted_by_emitter\":{},\"conf\":{}}}",
                    o.n_vars, o.n_eqs, o.inexact, o.roots.len(), got.len(), truth.len(), o.ir, o.calls, o.refuted_by_emitter, o.conflicts
                );
            }
            Err(e) => println!("{{\"n\":{n},\"l\":{l},\"target\":{done},\"error\":{:?},\"truth_sorted\":{}}}", e, truth.len()),
        }
    }
}
