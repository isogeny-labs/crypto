//! Structural probe for the quartic twist line, frozen in
//! research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use std::collections::{BTreeMap, HashSet};

use crypto_lib::cryptanalysis::gaudry_quartic::{Curve4, Pt4, E4};
use crypto_lib::cryptanalysis::residual_walk::{is_prime_u64, pow_mod};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

fn trace_fp(p: u64, b: u64) -> i128 {
    let mut count = 1u64;
    for x in 0..p {
        let f = (pow_mod(x, 3, p) + b) % p;
        count += if f == 0 {
            1
        } else if pow_mod(f, (p - 1) / 2, p) == 1 {
            2
        } else {
            0
        };
    }
    (p + 1) as i128 - count as i128
}

fn frob_point(curve: &Curve4, pt: &Pt4, h2: &E4, h3: &E4) -> Pt4 {
    if pt.inf {
        return *pt;
    }
    let f = &curve.f;
    Pt4 {
        x: f.mul(h2, &f.pow(&pt.x, f.p as u128)),
        y: f.mul(h3, &f.pow(&pt.y, f.p as u128)),
        inf: false,
    }
}

fn cubic_point(curve: &Curve4, pt: &Pt4, omega: u64) -> Pt4 {
    if pt.inf {
        return *pt;
    }
    Pt4 {
        x: curve.f.scale(&pt.x, omega),
        y: pt.y,
        inf: false,
    }
}

fn exact_root(curve: &Curve4, g: &Pt4, image: &Pt4, r: u64, order: u64) -> u64 {
    assert_eq!((r - 1) % order, 0);
    let root = (2..r)
        .map(|x| pow_mod(x, (r - 1) / order, r))
        .find(|&x| {
            if order == 3 {
                x != 1
            } else {
                pow_mod(x, order / 2, r) == r - 1
            }
        })
        .expect("no primitive root of requested order");
    (1..order)
        .filter(|&j| order == 3 || j % 2 == 1)
        .map(|j| pow_mod(root, j, r))
        .find(|&x| curve.mul(g, x) == *image)
        .expect("map is not a scalar root on generator")
}

fn orbit_hist(
    curve: &Curve4,
    points: &[Pt4],
    omega: u64,
    h2: &E4,
    h3: &E4,
    combined: bool,
) -> BTreeMap<usize, usize> {
    let all: HashSet<_> = points.iter().copied().collect();
    let mut seen = HashSet::new();
    let mut hist = BTreeMap::new();
    for &pt in points {
        if !seen.insert(pt) {
            continue;
        }
        let mut queue = vec![pt];
        let mut j = 0;
        while j < queue.len() {
            let x = queue[j];
            j += 1;
            let a = cubic_point(curve, &x, omega);
            let b = if combined {
                frob_point(curve, &x, h2, h3)
            } else {
                curve.neg(&x)
            };
            for y in [a, b] {
                assert!(all.contains(&y), "factor base not closed under map");
                if seen.insert(y) {
                    queue.push(y);
                }
            }
        }
        *hist.entry(queue.len()).or_insert(0) += 1;
    }
    assert_eq!(seen.len(), all.len());
    hist
}

fn direct_order_p13(curve: &Curve4) -> u64 {
    let p = curve.f.p;
    assert_eq!(p, 13);
    let mut squares = std::collections::HashMap::<E4, u64>::new();
    for a in 0..p {
        for b in 0..p {
            for c in 0..p {
                for d in 0..p {
                    let y = E4([a, b, c, d]);
                    *squares.entry(curve.f.sq(&y)).or_insert(0) += 1;
                }
            }
        }
    }
    let mut order = 1;
    for a in 0..p {
        for b in 0..p {
            for c in 0..p {
                for d in 0..p {
                    order += squares
                        .get(&curve.rhs(&E4([a, b, c, d])))
                        .copied()
                        .unwrap_or(0);
                }
            }
        }
    }
    order
}

fn main() {
    const PANEL: [(u64, u64); 8] = [
        (13, 4),
        (37, 2),
        (73, 5),
        (97, 5),
        (157, 25),
        (229, 6),
        (241, 49),
        (337, 100),
    ];
    for (p, b0) in PANEL {
        assert!(is_prime_u64(p) && p % 12 == 1);
        let f0 = Curve4::with_params(p, E4::ZERO, E4::ZERO);
        let f = &f0.f;
        let t = E4([0, 1, 0, 0]);
        assert!(!f.is_square(&t));
        let z = f.pow(&t, (p - 1) as u128);
        assert!(z.in_fp() && pow_mod(z.0[0], 4, p) == 1 && pow_mod(z.0[0], 2, p) == p - 1);
        let h = f.pow(
            &t,
            p as u128 * p as u128 * p as u128 * p as u128 - 1 - (p - 1) as u128 / 2,
        );
        let h2 = f.sq(&h);
        let h3 = f.mul(&h2, &h);
        let b = f.scale(&f.mul(&f.sq(&t), &t), b0);
        let curve = Curve4::with_params(p, E4::ZERO, b);
        let t1 = trace_fp(p, b0);
        let t2 = t1 * t1 - 2 * p as i128;
        let t4 = t2 * t2 - 2 * (p as i128) * (p as i128);
        let n = (p as i128).pow(4) + 1 + t4;
        assert!(n > 0 && n <= u64::MAX as i128);
        let r = n as u64;
        assert!(is_prime_u64(r), "selected full order is composite");
        assert!(
            r > 4 * p * p,
            "Hasse interval could contain two multiples of r"
        );
        let mut rng = StdRng::seed_from_u64(0x474c_5334 ^ p ^ b0);
        let cert_point = loop {
            let x = f.random(&mut rng);
            if let Some(pt) = curve.lift_x(&x, &mut rng) {
                if !pt.inf {
                    break pt;
                }
            }
        };
        assert!(curve.on_curve(&cert_point) && curve.mul(&cert_point, r).inf);
        let direct = if p == 13 {
            Some(direct_order_p13(&curve))
        } else {
            None
        };
        if let Some(direct) = direct {
            assert_eq!(direct, r);
        }
        let mut base = Vec::new();
        let mut line_x = Vec::new();
        for x in 0..p {
            if let Some(pt) = curve.lift_x(&E4([x, 0, 0, 0]), &mut rng) {
                assert!(curve.on_curve(&pt) && !pt.y.is_zero());
                line_x.push(x);
                base.push(pt);
                base.push(curve.neg(&pt));
            }
        }
        if base.is_empty() {
            println!(
                "{}",
                json!({
                    "p":p,"b0":b0,"d":curve.f.c,"trace_fp":t1,"r":r,
                    "direct_order":direct,"order_certificate":{"x":cert_point.x.0,"y":cert_point.y.0},
                    "b_curve":b.0,"line_x":line_x,"base_size":0,"status":"empty_base"
                })
            );
            continue;
        }
        let g = base[0];
        assert!(curve.mul(&g, r).inf);
        let omega = (2..p)
            .map(|x| pow_mod(x, (p - 1) / 3, p))
            .find(|&x| x != 1)
            .unwrap();
        let psi_g = frob_point(&curve, &g, &h2, &h3);
        let cubic_g = cubic_point(&curve, &g, omega);
        let lambda_psi = exact_root(&curve, &g, &psi_g, r, 8);
        let lambda_cubic = exact_root(&curve, &g, &cubic_g, r, 3);
        let mut evals = 0;
        for pt in &base {
            assert!(curve.on_curve(pt) && curve.mul(pt, r).inf);
            let mut y = *pt;
            for _ in 0..4 {
                y = frob_point(&curve, &y, &h2, &h3);
                evals += 1;
            }
            assert_eq!(y, curve.neg(pt));
            let mut zpt = *pt;
            for _ in 0..3 {
                zpt = cubic_point(&curve, &zpt, omega);
            }
            assert_eq!(zpt, *pt);
            assert_eq!(
                frob_point(&curve, &cubic_point(&curve, pt, omega), &h2, &h3),
                cubic_point(&curve, &frob_point(&curve, pt, &h2, &h3), omega)
            );
            assert_eq!(frob_point(&curve, pt, &h2, &h3), curve.mul(pt, lambda_psi));
            assert_eq!(cubic_point(&curve, pt, omega), curve.mul(pt, lambda_cubic));
            assert!(curve.on_curve(&frob_point(&curve, pt, &h2, &h3)));
        }
        let hist_control = orbit_hist(&curve, &base, omega, &h2, &h3, false);
        let hist_combined = orbit_hist(&curve, &base, omega, &h2, &h3, true);
        println!(
            "{}",
            json!({
                "p":p,"b0":b0,"d":curve.f.c,"z":z.0[0],"omega":omega,
                "trace_fp":t1,"r":r,"direct_order":direct,
                "b_curve":b.0,"h":h.0,"lambda_psi":lambda_psi,
                "lambda_cubic":lambda_cubic,"lambda_product_order":24,
                "generator":{"x":g.x.0,"y":g.y.0},
                "order_certificate":{"x":cert_point.x.0,"y":cert_point.y.0},
                "line_x":line_x,"base_points":base.iter().map(|q|json!([q.x.0,q.y.0])).collect::<Vec<_>>(),
                "base_size":base.len(),"control_orbits":hist_control,"combined_orbits":hist_combined,
                "control_columns":hist_control.values().sum::<usize>(),
                "combined_columns":hist_combined.values().sum::<usize>(),
                "psi_evals_checked":evals,"status":"pass"
            })
        );
    }
}
