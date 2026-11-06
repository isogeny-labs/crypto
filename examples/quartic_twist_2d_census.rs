//! Exact census of the two-dimensional invariant abscissa space Fp + t²Fp.
//! Frozen protocol: research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use std::collections::{BTreeMap, HashSet};
use std::time::Instant;

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, E4};
use crypto_lib::cryptanalysis::residual_walk::pow_mod;
use serde_json::json;

fn main() {
    const PANEL: [(u64, u64, usize, usize); 8] = [
        (13, 4, 0, 0),
        (37, 2, 36, 12),
        (73, 5, 48, 48),
        (97, 5, 36, 36),
        (157, 25, 108, 84),
        (229, 6, 96, 96),
        (241, 49, 96, 96),
        (337, 100, 180, 180),
    ];
    for (p, b0, expected_l0, expected_l2) in PANEL {
        let started = Instant::now();
        let f0 = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &f0.f;
        let t = E4([0, 1, 0, 0]);
        let t3 = f.mul(&f.sq(&t), &t);
        let curve = Curve4::with_params(p, E4::ZERO, f.scale(&t3, b0));
        let f = &curve.f;
        let z4 = f.pow(&t, (p - 1) as u128);
        assert!(z4.in_fp());
        let h = f.pow(&t, (p as u128).pow(4) - 1 - (p - 1) as u128 / 2);
        let h2 = f.sq(&h);
        assert!(h2.in_fp());
        let omega = (2..p)
            .map(|v| pow_mod(v, (p - 1) / 3, p))
            .find(|&v| v != 1)
            .unwrap();
        // On this twist x=0 has no lift. Excluding it is therefore exact.
        assert!(!f.is_square(&curve.b));
        f.reset_muls();
        let mut xs = HashSet::new();
        let mut axis0 = 0usize;
        let mut axis2 = 0usize;
        let mut interior = 0usize;
        for a in 0..p {
            for b in 0..p {
                if a == 0 && b == 0 {
                    continue;
                }
                let x = E4([a, 0, b, 0]);
                let rhs = f.add(&f.mul(&f.sq(&x), &x), &curve.b);
                if f.is_square(&rhs) {
                    assert!(xs.insert(x));
                    if b == 0 {
                        axis0 += 1;
                    } else if a == 0 {
                        axis2 += 1;
                    } else {
                        interior += 1;
                    }
                }
            }
        }
        assert_eq!(axis0, expected_l0);
        assert_eq!(axis2, expected_l2);
        let mut seen = HashSet::new();
        let mut orbit_hist = BTreeMap::new();
        for &x in &xs {
            if !seen.insert(x) {
                continue;
            }
            let mut queue = vec![x];
            let mut next = 0;
            while next < queue.len() {
                let v = queue[next];
                next += 1;
                let frob = f.mul(&h2, &f.frob_p(&v, z4.0[0]));
                let cubic = f.scale(&v, omega);
                for image in [frob, cubic] {
                    assert!(image.0[1] == 0 && image.0[3] == 0);
                    assert!(xs.contains(&image), "invariant space left the lifted set");
                    if seen.insert(image) {
                        queue.push(image);
                    }
                }
            }
            *orbit_hist.entry(queue.len()).or_insert(0usize) += 1;
        }
        assert_eq!(seen.len(), xs.len());
        assert_eq!(
            orbit_hist
                .iter()
                .map(|(size, count)| size * count)
                .sum::<usize>(),
            xs.len()
        );
        assert_eq!(
            orbit_hist.keys().copied().collect::<Vec<_>>(),
            if xs.is_empty() { vec![] } else { vec![12] }
        );
        let base = xs.len() as u64;
        println!(
            "{}",
            json!({
                "p": p, "b0": b0, "abscissa_candidates": p*p-1,
                "line0": axis0, "line2": axis2, "interior": interior,
                "physical_base": base, "folded_columns": base/12,
                "orbit_hist": orbit_hist,
                "signed_pair_entries": base*(base-1),
                "census_fp_muls": f.muls(),
                "wall_ms_l0": started.elapsed().as_secs_f64()*1e3,
                "status": "pass"
            })
        );
    }
}
