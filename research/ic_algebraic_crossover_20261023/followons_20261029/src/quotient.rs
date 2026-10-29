//! Independent finite-kernel Vélu evaluation and a Koblitz endomorphism candidate.
use crate::field::{Curve, Field, Point};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn kernel_from_points(c: Curve, witnesses: &[Point], r: u64, h: usize) -> Vec<Point> {
    let mut k = vec![Point::Infinity];
    let mut set = BTreeSet::from([Point::Infinity]);
    for &p in witnesses {
        assert!(c.on(p));
        let g = c.mul(p, r);
        assert_eq!(c.mul(g, h as u64), Point::Infinity);
        if set.contains(&g) {
            continue;
        }
        let old = k.clone();
        let old_set = set.clone();
        let mut t = g;
        while !old_set.contains(&t) {
            for &q in &old {
                let z = c.add(q, t);
                if set.insert(z) {
                    k.push(z);
                    assert!(k.len() <= h, "supplied points exceed declared kernel");
                }
            }
            t = c.add(t, g);
        }
        if k.len() == h {
            break;
        }
    }
    k.sort_unstable();
    k
}
pub fn velu(c: Curve, k: &[Point], p: Point) -> Point {
    if k.binary_search(&p).is_ok() {
        return Point::Infinity;
    }
    let Point::Affine(mut x, mut y) = p else {
        return Point::Infinity;
    };
    for &q in k {
        if let Point::Affine(u, v) = q {
            let Point::Affine(a, b) = c.add(p, q) else {
                panic!("kernel membership guard failed")
            };
            x ^= a ^ u;
            y ^= b ^ v;
        }
    }
    Point::Affine(x, y)
}
pub fn normalize_point(p: Point, y_shift: u32) -> Point {
    match p {
        Point::Infinity => p,
        Point::Affine(x, y) => Point::Affine(x, y ^ y_shift),
    }
}
/// Solve all observed target equations, rather than choosing an unchecked target.
pub fn fit_target(f: Field, images: &[Point]) -> (Curve, u32) {
    let mut rows: Vec<[u32; 4]> = images
        .iter()
        .filter_map(|&p| match p {
            Point::Infinity => None,
            Point::Affine(x, y) => Some([f.sq(x), x, 1, f.sq(y) ^ f.mul(x, y) ^ f.mul(f.sq(x), x)]),
        })
        .collect();
    for col in 0..3 {
        let r = col;
        let i = (r..rows.len())
            .find(|&i| rows[i][col] != 0)
            .expect("insufficient independent image coordinates");
        rows.swap(i, r);
        let inv = f.inv(rows[r][col]);
        for value in &mut rows[r][col..] {
            *value = f.mul(*value, inv);
        }
        let pivot = rows[r];
        for (i, row) in rows.iter_mut().enumerate() {
            if i != r {
                let scale = row[col];
                for (value, &base) in row[col..].iter_mut().zip(&pivot[col..]) {
                    *value ^= f.mul(scale, base);
                }
            }
        }
    }
    assert!(
        rows[3..]
            .iter()
            .all(|r| r[..3].iter().any(|&x| x != 0) || r[3] == 0),
        "image fails target equations"
    );
    let raw = Curve {
        f,
        a2: rows[0][3],
        a4: rows[1][3],
        a6: rows[2][3],
    };
    assert!(images.iter().all(|&p| raw.on(p)));
    let target = Curve {
        f,
        a2: raw.a2,
        a4: 0,
        a6: raw.a6 ^ f.sq(raw.a4),
    };
    assert!(images
        .iter()
        .all(|&p| target.on(normalize_point(p, raw.a4))));
    (target, raw.a4)
}
pub fn fast_map(c: Curve, p: Point, s: u64, v_power: u32) -> Point {
    let mut p = p;
    for _ in 0..v_power {
        p = c.verschiebung(p);
    }
    c.mul(p, s)
}
fn orientation(c: Curve, reference: Point, candidate: Point) -> i32 {
    if reference == candidate {
        1
    } else if reference == c.neg(candidate) {
        -1
    } else {
        panic!("endomorphism and Vélu disagree beyond sign")
    }
}
pub fn small_control(n: u32, modulus: u32, r: u64, h: usize, v_power: u32) -> Value {
    let c = Curve {
        f: Field { n, modulus },
        a2: 0,
        a4: 0,
        a6: 1,
    };
    let points = c.points();
    assert_eq!(points.len(), r as usize * h);
    // Independent finite-field lift oracle: inspect every (x,y), rather than
    // trusting the Artin--Schreier solver used by points().
    for x in 0..1u32 << n {
        let lifts = c.lift(x);
        for y in 0..1u32 << n {
            let p = Point::Affine(x, y);
            assert_eq!(lifts.contains(&p), c.on(p));
        }
    }
    assert!(points
        .iter()
        .all(|&p| c.on(p) && c.mul(p, r * h as u64) == Point::Infinity));
    let k = kernel_from_points(c, &points, r, h);
    assert_eq!(k.len(), h);
    assert!(k.iter().all(|&p| c.mul(p, h as u64) == Point::Infinity));
    let raw: Vec<_> = points.iter().map(|&p| velu(c, &k, p)).collect();
    let (target, shift) = fit_target(c.f, &raw);
    assert_eq!((target.a2, target.a4, target.a6), (0, 0, 1));
    let maps: BTreeMap<_, _> = points
        .iter()
        .copied()
        .zip(raw.into_iter().map(|p| normalize_point(p, shift)))
        .collect();
    let distinct: BTreeSet<_> = maps.values().copied().collect();
    assert_eq!(distinct.len(), r as usize);
    let p = *points.iter().find(|p| maps[p] != Point::Infinity).unwrap();
    let sign = orientation(target, maps[&p], fast_map(c, p, 1, v_power));
    for &p in &points {
        let z = fast_map(c, p, 1, v_power);
        assert_eq!(maps[&p], if sign == 1 { z } else { target.neg(z) });
        assert_eq!(target.mul(maps[&p], r), Point::Infinity);
        let count = maps.values().filter(|&&z| z == maps[&p]).count();
        assert_eq!(count, h);
        for &q in &points {
            assert_eq!(maps[&c.add(p, q)], target.add(maps[&p], maps[&q]));
        }
    }
    json!({"field_degree":n,"modulus":modulus,"source":{"a2":0,"a4":0,"a6":1},"target":target,
        "registry_status":"fixed software construction control; no new ICV1 slug emitted","ambient_points_enumerated":points.len(),"r":r,"cofactor":h,
        "kernel_distinct":k.len(),"projected_distinct":distinct.len(),"homomorphism_pairs_checked":points.len()*points.len(),
        "all_fibers_size":h,"velu_y_normalization":shift,"endomorphism_orientation":sign,"vershiebung_power":v_power,
        "status":"complete finite software conformance control","complete_query_cost":null,"speedup":null})
}
pub fn original_e0_control() -> Value {
    let c = Curve {
        f: Field {
            n: 22,
            modulus: 4_194_307,
        },
        a2: 0,
        a4: 0,
        a6: 1,
    };
    let (r, h) = (991u64, 4232usize);
    let g = Point::Affine(97_814, 2_630_359);
    assert!(c.on(g) && c.mul(g, r) == Point::Infinity);
    let trace = |x: u32| {
        let (mut y, mut t) = (x, 0);
        for _ in 0..22 {
            t ^= y;
            y = c.f.sq(y);
        }
        t
    };
    let t = (0..22).map(|i| 1 << i).find(|&x| trace(x) == 1).unwrap();
    let low: Vec<_> = (0..64).flat_map(|x| c.lift(x)).collect();
    let incomplete = kernel_from_points(c, &low, r, h);
    assert_eq!(incomplete.len(), 2116);
    let mut witnesses = low;
    witnesses.extend((0..64).flat_map(|x| c.lift(t ^ x)));
    let k = kernel_from_points(c, &witnesses, r, h);
    assert_eq!(k.len(), h);
    for &p in &k {
        assert!(c.on(p));
        assert_eq!(c.mul(p, h as u64), Point::Infinity);
        assert_eq!(fast_map(c, p, 23, 3), Point::Infinity);
    }
    let seeds = [1, 2, 3, 5, 11, 17, 31, 73];
    let mut sampled: Vec<_> = seeds.iter().map(|&a| c.mul(g, a)).collect();
    for &a in &[1usize, 17, 2115] {
        sampled.push(c.add(g, k[a]));
    }
    for &(a, b) in &[(1u64, 2u64), (3, 17), (31, 73)] {
        sampled.push(c.mul(g, a + b));
    }
    let raw: Vec<_> = sampled.iter().map(|&p| velu(c, &k, p)).collect();
    let (target, shift) = fit_target(c.f, &raw);
    assert_eq!((target.a2, target.a4, target.a6), (0, 0, 1));
    let sign = orientation(
        target,
        normalize_point(raw[0], shift),
        fast_map(c, sampled[0], 23, 3),
    );
    for (&p, &v) in sampled.iter().zip(&raw) {
        let q = fast_map(c, p, 23, 3);
        let q = if sign == 1 { q } else { target.neg(q) };
        assert_eq!(normalize_point(v, shift), q);
        assert_eq!(target.mul(q, r), Point::Infinity);
    }
    assert_ne!(fast_map(c, g, 23, 3), Point::Infinity);
    // pi V=[2] on this E0; psi=[23]pi^3 complements phi=[23]V^3.
    for &p in &sampled {
        assert_eq!(c.frob(c.verschiebung(p)), c.mul(p, 2));
        let mut q = fast_map(c, p, 23, 3);
        for _ in 0..3 {
            q = c.frob(q);
        }
        q = c.mul(q, 23);
        assert_eq!(q, c.mul(p, h as u64));
    }
    json!({"field_degree":22,"modulus":4_194_307,"source":{"a2":0,"a4":0,"a6":1},"target":target,
        "ambient_order_input":4_193_912,"ambient_order_independently_counted":false,"r":r,"cofactor":h,
        "known_generator":[97814,2630359],"incomplete_kernel_negative_control":incomplete.len(),"kernel_distinct":k.len(),
        "kernel_points_checked_for_h_and_phi":k.len(),"phi":"[23](-1-Frob)^3","psi":"[23]Frob^3","psi_phi_scalar":4232,
        "velu_sample_points_checked":sampled.len(),"velu_y_normalization":shift,"endomorphism_orientation":sign,
        "registry_status":"existing fixed public E0 fixture; target parameters verified on recorded component controls",
        "scope":"conditional on recorded ambient order; full algebraic relation oracle and complete price remain unimplemented",
        "complete_query_cost":null,"speedup":null})
}
