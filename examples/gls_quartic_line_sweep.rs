//! Exact sweep of all four Frobenius eigenlines on the frozen quartic twists.
use std::collections::{BTreeMap, HashSet};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::pow_mod;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

fn psi(curve: &Curve4, pt: Pt4, h2: &E4, h3: &E4) -> Pt4 {
    if pt.inf {
        return pt;
    }
    Pt4 {
        x: curve.f.mul(h2, &curve.f.pow(&pt.x, curve.f.p as u128)),
        y: curve.f.mul(h3, &curve.f.pow(&pt.y, curve.f.p as u128)),
        inf: false,
    }
}

fn cubic(curve: &Curve4, pt: Pt4, omega: u64) -> Pt4 {
    if pt.inf {
        return pt;
    }
    Pt4 {
        x: curve.f.scale(&pt.x, omega),
        ..pt
    }
}

fn orbit_hist(curve: &Curve4, pts: &[Pt4], omega: u64, h2: &E4, h3: &E4) -> BTreeMap<usize, usize> {
    let all: HashSet<_> = pts.iter().copied().collect();
    let mut seen = HashSet::new();
    let mut hist = BTreeMap::new();
    for &pt in pts {
        if !seen.insert(pt) {
            continue;
        }
        let mut queue = vec![pt];
        let mut i = 0;
        while i < queue.len() {
            let p = queue[i];
            i += 1;
            for q in [psi(curve, p, h2, h3), cubic(curve, p, omega)] {
                assert!(all.contains(&q));
                if seen.insert(q) {
                    queue.push(q);
                }
            }
        }
        *hist.entry(queue.len()).or_insert(0) += 1;
    }
    assert_eq!(seen.len(), all.len());
    hist
}

fn main() {
    for (p, b0) in [
        (13, 4),
        (37, 2),
        (73, 5),
        (97, 5),
        (157, 25),
        (229, 6),
        (241, 49),
        (337, 100),
    ] {
        let f0 = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &f0.f;
        let t = E4([0, 1, 0, 0]);
        let t2 = f.sq(&t);
        let t3 = f.mul(&t2, &t);
        let curve = Curve4::with_params(p, E4::ZERO, f.scale(&t3, b0));
        let z = f.pow(&t, (p - 1) as u128).0[0];
        let h = f.pow(&t, (p as u128).pow(4) - 1 - (p - 1) as u128 / 2);
        let h2 = f.sq(&h);
        let h3 = f.mul(&h2, &h);
        let omega = (2..p)
            .map(|x| pow_mod(x, (p - 1) / 3, p))
            .find(|&x| x != 1)
            .unwrap();
        let mut rng = StdRng::seed_from_u64(0x4c49_4e45 ^ p ^ b0);
        for (j, v) in [E4::ONE, t, t2, t3].iter().enumerate() {
            let mut xs = Vec::new();
            let mut pts = Vec::new();
            let scale = pow_mod(z, (j + 3) as u64 % 4, p); // z^(j-1)
            for s in 1..p {
                let x = f.scale(v, s);
                if let Some(pt) = curve.lift_x(&x, &mut rng) {
                    assert!(curve.on_curve(&pt));
                    let image = psi(&curve, pt, &h2, &h3);
                    assert!(curve.on_curve(&image));
                    assert_eq!(image.x, f.scale(&x, scale));
                    assert_eq!(cubic(&curve, pt, omega).x, f.scale(&x, omega));
                    xs.push(s);
                    pts.push(pt);
                    pts.push(curve.neg(&pt));
                }
            }
            let hist = orbit_hist(&curve, &pts, omega, &h2, &h3);
            println!(
                "{}",
                json!({
                    "p":p,"b0":b0,"line_power":j,"z":z,"omega":omega,
                    "lifted_abscissae":xs,"abscissa_count":xs.len(),
                    "physical_points":pts.len(),"orbit_hist":hist,
                    "status":"pass"
                })
            );
        }
    }
}
