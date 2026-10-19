//! Supersingular equal-map component: actual automorphisms and Hurwitz torsion modules.
use crate::{round02, round04::Field};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
type M = [i64; 4];
type V = (i64, i64);
type Q = [i32; 4];
type Pt = Option<(u32, u32)>;
fn mm(a: M, b: M, n: i64) -> M {
    [
        (a[0] * b[0] + a[1] * b[2]).rem_euclid(n),
        (a[0] * b[1] + a[1] * b[3]).rem_euclid(n),
        (a[2] * b[0] + a[3] * b[2]).rem_euclid(n),
        (a[2] * b[1] + a[3] * b[3]).rem_euclid(n),
    ]
}
fn det(a: M, n: i64) -> i64 {
    (a[0] * a[3] - a[1] * a[2]).rem_euclid(n)
}
fn inv(x: i64, n: i64) -> i64 {
    (1..n).find(|i| (x * i).rem_euclid(n) == 1).unwrap()
}
fn normqs(n: i32) -> Vec<Q> {
    let mut r = vec![];
    for a in -7i32..=7 {
        for b in -7i32..=7 {
            for c in -7i32..=7 {
                for d in -7i32..=7 {
                    if a * a + b * b + c * c + d * d == 4 * n
                        && [b, c, d].iter().all(|v| (*v - a).rem_euclid(2) == 0)
                    {
                        r.push([a, b, c, d]);
                    }
                }
            }
        }
    }
    r
}
fn mat(q: Q, n: i64) -> M {
    let i = [0, n - 1, 1, 0];
    let j = if n == 49 {
        [2, 17, 17, n - 2]
    } else {
        [1, 3, 3, n - 1]
    };
    let k = mm(i, j, n);
    let id = [1, 0, 0, 1];
    std::array::from_fn(|r| {
        ((q[0] as i64 * id[r] + q[1] as i64 * i[r] + q[2] as i64 * j[r] + q[3] as i64 * k[r])
            * inv(2, n))
        .rem_euclid(n)
    })
}
fn mv(a: M, p: V, n: i64) -> V {
    (
        (a[0] * p.0 + a[1] * p.1).rem_euclid(n),
        (a[2] * p.0 + a[3] * p.1).rem_euclid(n),
    )
}
fn proj(p: V) -> V {
    if p.0 % 7 != 0 {
        (1, (p.1 * inv(p.0, 49)).rem_euclid(49))
    } else {
        ((p.0 * inv(p.1, 49)).rem_euclid(49), 1)
    }
}
fn act(a: M, p: V) -> V {
    proj(mv(a, p, 49))
}
fn bm(a: M) -> M {
    a.min(a.map(|x| (-x).rem_euclid(11)))
}
fn plus(f: Field, p: Pt, q: Pt) -> Pt {
    let (Some((x, y)), Some((u, v))) = (p, q) else {
        return p.or(q);
    };
    if x == u && y != v {
        return None;
    }
    let l = if x == u {
        f.sq(x)
    } else {
        f.mul(y ^ v, f.inv(x ^ u))
    };
    let xx = f.sq(l) ^ x ^ u;
    Some((xx, f.mul(l, x ^ xx) ^ y ^ 1))
}
fn neg(p: Pt) -> Pt {
    p.map(|(x, y)| (x, y ^ 1))
}
fn auto(f: Field, u: u32, r: u32, t: u32, p: Pt) -> Pt {
    p.map(|(x, y)| {
        (
            f.mul(f.sq(u), x) ^ r,
            y ^ f.mul(f.mul(f.sq(u), f.sq(r)), x) ^ t,
        )
    })
}
fn bind() -> (Q, serde_json::Value) {
    let f = Field::find(8);
    let omega = (2..256).find(|x| f.sq(*x) ^ x ^ 1 == 0).unwrap();
    let mut points = vec![None];
    for x in 0..256 {
        for y in 0..256 {
            if f.sq(y) ^ y == f.pow(x, 3) {
                points.push(Some((x, y)))
            }
        }
    }
    assert_eq!(points.len(), 225);
    let ip = |p| auto(f, 1, 1, omega, p);
    let jp = |p| auto(f, 1, omega, omega, p);
    let kp = |p| ip(jp(p));
    for p in &points {
        assert_eq!(ip(ip(*p)), neg(*p));
        assert_eq!(jp(jp(*p)), neg(*p));
        assert_eq!(ip(jp(*p)), neg(jp(ip(*p))));
    }
    let components = |p| [p, ip(p), jp(p), kp(p)];
    let eval = |q: Q, p: Pt| {
        let c = components(p);
        let mut z = None;
        for i in 0..4 {
            for _ in 0..q[i].unsigned_abs() {
                z = plus(f, z, if q[i] < 0 { neg(c[i]) } else { c[i] });
            }
        }
        z
    };
    let mut autos = vec![];
    for u in [1, omega, f.sq(omega)] {
        for r in [0, 1, omega, f.sq(omega)] {
            for t in [0, 1, omega, f.sq(omega)] {
                if f.sq(t) ^ t == f.pow(r, 3) {
                    autos.push((u, r, t));
                }
            }
        }
    }
    assert_eq!(autos.len(), 24);
    let mut bindings = vec![];
    for q in normqs(1) {
        let found: Vec<_> = autos
            .iter()
            .filter(|(u, r, t)| {
                points.iter().all(|p| {
                    let ap = auto(f, *u, *r, *t, *p);
                    plus(f, ap, ap) == eval(q, *p)
                })
            })
            .collect();
        assert_eq!(found.len(), 1);
        bindings.push(json!({"twice_quaternion_coordinates":q,"automorphism_u_r_t":found[0]}));
    }
    let pi: Vec<_> = normqs(2)
        .into_iter()
        .filter(|q| q[0] == 0 && q.iter().all(|v| v % 2 == 0))
        .filter(|q| {
            points.iter().all(|p| {
                let half = q.map(|v| v / 2);
                eval(half, *p) == p.map(|(x, y)| (f.sq(x), f.sq(y)))
            })
        })
        .collect();
    assert_eq!(pi.len(), 1);
    (
        pi[0],
        json!({"model":"y²+y=x³","field_modulus":f.modulus,"F4_root":omega,"points_checked":225,"I":"(x+1,y+x+omega)","J":"(x+omega,y+omega²*x+omega)","unit_bindings":bindings,"Frobenius_twice_quaternion":pi[0],"global_identity_bridge":"the checked differences are sums of at most six degree1 automorphisms,hence degree<=36;225 distinct zeros force identity;[2] cancellation is in the endomorphism domain"}),
    )
}
pub fn run(dir: &Path) {
    let (pi, binding) = bind();
    let units = normqs(1);
    assert_eq!(units.len(), 24);
    let mut us = BTreeMap::new();
    for q in &units {
        us.entry(bm(mat(*q, 11))).or_insert(mat(*q, 49));
    }
    let us: Vec<_> = us.into_iter().collect();
    assert_eq!(us.len(), 12);
    let lines: Vec<V> = (0..49)
        .map(|a| (1, a))
        .chain((0..7).map(|a| (7 * a, 1)))
        .collect();
    let class = |s: V| us.iter().map(|(_, u)| act(*u, s)).min().unwrap();
    let mut class_sizes = BTreeMap::new();
    for s in &lines {
        *class_sizes.entry(class(*s)).or_insert(0usize) += 1;
    }
    assert_eq!(class_sizes.len(), 6);
    let norm11 = normqs(11);
    assert_eq!(norm11.len(), 288);
    let mut kernels = BTreeMap::new();
    for q in &norm11 {
        let a = mat(*q, 11);
        assert_eq!(det(a, 11), 0);
        let line = (0..12)
            .find(|i| {
                let v = if *i == 11 { (0, 1) } else { (1, *i) };
                mv(a, v, 11) == (0, 0)
            })
            .unwrap();
        assert_eq!(det(mat(*q, 49), 49), 11);
        kernels.entry(line).or_insert(*q);
    }
    assert_eq!(kernels.len(), 12);
    let mut bases = BTreeSet::new();
    for a in 0..11 {
        for b in 0..11 {
            for c in 0..11 {
                for d in 0..11 {
                    let m = [a, b, c, d];
                    if det(m, 11) == 1 {
                        bases.insert(bm(m));
                    }
                }
            }
        }
    }
    assert_eq!(bases.len(), 660);
    let mut canonical = BTreeMap::new();
    let mut representatives = BTreeSet::new();
    for b in &bases {
        let dest = us
            .iter()
            .map(|(u, v)| (bm(mm(*u, *b, 11)), *v))
            .min()
            .unwrap();
        canonical.insert(*b, dest);
        representatives.insert(dest.0);
    }
    let representatives: Vec<_> = representatives.into_iter().collect();
    assert_eq!(representatives.len(), 55);
    let basis_index: BTreeMap<_, _> = representatives
        .iter()
        .enumerate()
        .map(|(i, b)| (*b, i))
        .collect();
    let line11 = |v: V| {
        if v.0 == 0 {
            11
        } else {
            (v.1 * inv(v.0, 11)).rem_euclid(11)
        }
    };
    let mut matches = vec![];
    let mut lower = 0usize;
    let mut match_keys = BTreeMap::new();
    for (bi, b) in representatives.iter().enumerate() {
        let a0 = mat(kernels[&line11((b[0], b[2]))], 49);
        let ai = mat(kernels[&line11((b[1], b[3]))], 49);
        for s in &lines {
            let t0 = class(act(a0, *s));
            let ti = class(act(ai, *s));
            if t0 == ti {
                let e = 12 / class_sizes[&t0];
                lower += e;
                match_keys.insert((bi, *s), matches.len());
                matches.push(json!({"basis_orbit":bi,"cyclic49_line":s,"common_target_orbit":t0,"target_stabilizer":e,"multiplicity_lower_bound":e}));
            }
        }
    }
    let pi49 = mat(pi, 49);
    let pi11 = mat(pi, 11);
    let mut successor = vec![];
    for point in &matches {
        let bi = point["basis_orbit"].as_u64().unwrap() as usize;
        let s = (
            point["cyclic49_line"][0].as_i64().unwrap(),
            point["cyclic49_line"][1].as_i64().unwrap(),
        );
        let b = bm(mm(mm(pi11, representatives[bi], 11), [1, 0, 0, 6], 11));
        let (dest, u) = canonical[&b];
        let s = act(u, act(pi49, s));
        let ix = match_keys[&(basis_index[&dest], s)];
        successor.push(ix);
    }
    let mut seen = BTreeSet::new();
    let mut closed = vec![];
    for i in 0..matches.len() {
        if seen.contains(&i) {
            continue;
        }
        let mut orbit = vec![];
        let mut j = i;
        while seen.insert(j) {
            orbit.push(j);
            j = successor[j];
        }
        assert_eq!(i, j);
        let m = matches[i]["multiplicity_lower_bound"].as_u64().unwrap();
        assert!(orbit
            .iter()
            .all(|j| matches[*j]["multiplicity_lower_bound"] == m));
        closed.push(
            json!({"geometric_labels":orbit,"degree":orbit.len(),"multiplicity_lower_bound":m}),
        );
    }
    // Ordinary j=1 can only use the two horizontal kernels alpha and conjugate.
    let w = [0, -2, 1, 1];
    let alpha = [1, -4, 2, 3];
    let conjugate = [3, 4, -2, 1];
    let ordinary_matches = lines
        .iter()
        .filter(|s| act(alpha, **s) == act(conjugate, **s))
        .count();
    assert_eq!(ordinary_matches, 0);
    assert_eq!(mm(w, w, 49), [47, 47, 1, 48]);
    let remaining = 1280usize - 420;
    assert!(
        lower <= remaining,
        "supersingular lower bound contradicts retained diagonal degree"
    );
    let complete = lower == remaining;
    round02::save(
        dir,
        "m15-supersingular-diagonal-component.json",
        &json!({"schema":"koblitz.round10/m15-v1","actual_automorphisms":binding,"torsion_modules":{"I":[0,-1,1,0],"J_mod49":[2,17,17,-2],"J_mod11":[1,3,3,-1],"interpretation":"chosen split representations of the actual maximal Hurwitz order;torsion-basis existence is written,not explicit torsion point coordinates"},"quaternion_units":24,"unit_classes_mod_sign":12,"norm11_elements":288,"kernel_representatives":kernels,"cyclic49_lines":56,"target49_orbit_sizes":class_sizes.into_iter().map(|(v,n)|json!([v,n])).collect::<Vec<_>>(),"SL2_bases_mod_sign":660,"basis_orbit_representatives":representatives,"modular_point_classes":3080,"ordinary_j1_matching_cyclic49_lines":ordinary_matches,"geometric_equal_map_labels":matches,"Frobenius_successor":successor,"closed_labels":closed,"diagonal_map_degree":1280,"diagonal_cusp_degree":420,"residual_diagonal_degree":remaining,"supersingular_multiplicity_lower_sum":lower,"degree_gap":remaining-lower,"diagonal_component_complete_by_degree_exhaustion":complete,"antidiagonal_complete":false,"scope":"exact quaternion moduli labels and actual automorphism binding;target-stabilizer multiplicities are lower bounds until degree exhaustion;no explicit49/11 torsion coordinates or full coupled relation row"}),
    );
    println!("M15:{} matching SS labels,lower multiplicity sum{lower},remaining diagonal degree{remaining},complete={complete}",match_keys.len());
}
