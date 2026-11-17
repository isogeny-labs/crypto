//! Independent norm-criterion and signed-point audit of the 2D census.
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs;

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::pow_mod;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

fn norm_square(curve: &Curve4, rhs: &E4) -> bool {
    let f = &curve.f;
    if rhs.is_zero() {
        return true;
    }
    let q = (f.p as u128).pow(2);
    let norm = f.pow(rhs, q + 1);
    assert_eq!(norm.0[1], 0);
    assert_eq!(norm.0[3], 0);
    f.pow(&norm, (q - 1) / 2) == E4::ONE
}

fn frob(curve: &Curve4, pt: Pt4, z: u64, h2: &E4, h3: &E4) -> Pt4 {
    if pt.inf {
        return pt;
    }
    Pt4 {
        x: curve.f.mul(h2, &curve.f.frob_p(&pt.x, z)),
        y: curve.f.mul(h3, &curve.f.frob_p(&pt.y, z)),
        inf: false,
    }
}

fn cubic(curve: &Curve4, pt: Pt4, omega: u64) -> Pt4 {
    Pt4 {
        x: curve.f.scale(&pt.x, omega),
        ..pt
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2, "usage: quartic_twist_2d_audit CENSUS");
    for line in fs::read_to_string(&args[1]).expect("census raw").lines() {
        let rec: Value = serde_json::from_str(line).unwrap();
        let (p, b0) = (rec["p"].as_u64().unwrap(), rec["b0"].as_u64().unwrap());
        let f0 = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let t = E4([0, 1, 0, 0]);
        let t3 = f0.f.mul(&f0.f.sq(&t), &t);
        let curve = Curve4::with_params(p, E4::ZERO, f0.f.scale(&t3, b0));
        let f = &curve.f;
        let z = f.pow(&t, (p - 1) as u128).0[0];
        let h = f.pow(&t, (p as u128).pow(4) - 1 - (p - 1) as u128 / 2);
        let h2 = f.sq(&h);
        let h3 = f.mul(&h2, &h);
        let omega = (2..p)
            .map(|a| pow_mod(a, (p - 1) / 3, p))
            .find(|&a| a != 1)
            .unwrap();
        let mut rng = StdRng::seed_from_u64(0x3244_4155_4449_54 ^ p ^ b0);
        let mut xs = HashSet::new();
        let mut l0 = 0u64;
        let mut l2 = 0u64;
        let mut inside = 0u64;
        let mut agreement = 0u64;
        for a in 0..p {
            for b in 0..p {
                if a == 0 && b == 0 {
                    continue;
                }
                let x = E4([a, 0, b, 0]);
                let rhs = curve.rhs(&x);
                let by_norm = norm_square(&curve, &rhs);
                assert_eq!(by_norm, f.is_square(&rhs));
                agreement += 1;
                if by_norm {
                    xs.insert(x);
                    if b == 0 {
                        l0 += 1;
                    } else if a == 0 {
                        l2 += 1;
                    } else {
                        inside += 1;
                    }
                }
            }
        }
        assert_eq!(l0, rec["line0"].as_u64().unwrap());
        assert_eq!(l2, rec["line2"].as_u64().unwrap());
        assert_eq!(inside, rec["interior"].as_u64().unwrap());
        assert_eq!(xs.len() as u64, rec["physical_base"].as_u64().unwrap());
        let mut seen_x = HashSet::new();
        let mut point_hist = BTreeMap::new();
        let mut verified_orbits = 0u64;
        for &x in &xs {
            if !seen_x.insert(x) {
                continue;
            }
            let pt = curve
                .lift_x(&x, &mut rng)
                .expect("norm-square abscissa lifts");
            assert!(curve.on_curve(&pt));
            let mut iterate = pt;
            for _ in 0..4 {
                iterate = frob(&curve, iterate, z, &h2, &h3);
            }
            assert_eq!(iterate, curve.neg(&pt));
            assert_eq!(
                frob(&curve, cubic(&curve, pt, omega), z, &h2, &h3),
                cubic(&curve, frob(&curve, pt, z, &h2, &h3), omega)
            );
            let mut queue = vec![pt];
            let mut seen_pt = HashSet::from([pt]);
            let mut next = 0;
            while next < queue.len() {
                let u = queue[next];
                next += 1;
                for v in [frob(&curve, u, z, &h2, &h3), cubic(&curve, u, omega)] {
                    assert!(curve.on_curve(&v));
                    assert!(xs.contains(&v.x));
                    if seen_pt.insert(v) {
                        queue.push(v);
                    }
                }
            }
            assert_eq!(seen_pt.len(), 24);
            let orbit_x: HashSet<_> = seen_pt.iter().map(|u| u.x).collect();
            assert_eq!(orbit_x.len(), 12);
            seen_x.extend(orbit_x);
            *point_hist.entry(seen_pt.len()).or_insert(0u64) += 1;
            verified_orbits += 1;
        }
        assert_eq!(seen_x.len(), xs.len());
        assert_eq!(verified_orbits, rec["folded_columns"].as_u64().unwrap());
        assert_eq!(point_hist.get(&24).copied(), Some(verified_orbits));
        println!(
            "{}",
            json!({
                "p":p,"b0":b0,"norm_character_agreements":agreement,
                "line0":l0,"line2":l2,"interior":inside,
                "lifted_abscissae":xs.len(),"point_orbits_checked":verified_orbits,
                "signed_point_orbit_hist":point_hist,"status":"pass"
            })
        );
    }
}
