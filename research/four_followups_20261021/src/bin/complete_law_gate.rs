//! Independent implementation of selected Ozbay Gurler–Hisil projective
//! branches: 1+1 (equations 13/14), cancellation 1+2 (equation 15),
//! and generic disjoint 1+2 (equation 18).
//! https://doi.org/10.1007/s13389-024-00351-7 (CC BY 4.0 article).
//! This is an arithmetic gate, not the complete 1+2/2+2 law or a DLP run.
use five_leads_study::three_leads::{Divisor, Field, Jacobian, Scholten};
use serde_json::json;

const PRIMES: [u64; 21] = [
    739, 1151, 1187, 3371, 3527, 3931, 11731, 11887, 12143, 48383, 48823, 49451, 186379, 186451,
    186583, 741967, 748807, 749299, 2967343, 2968367, 2972219,
];

#[derive(Clone, Copy)]
struct D1 {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}
enum ResultPoint {
    Identity,
    DegreeOne(D1),
    DegreeTwo([u64; 6]),
}
fn add22_generic(a: Divisor, b: Divisor, f: &Field) -> Option<ResultPoint> {
    // Equations (10) and (12), with denominators cleared into output weights.
    assert_eq!(a.degree, 2);
    assert_eq!(b.degree, 2);
    let [r1, q1, _] = a.u;
    let [t1, s1] = a.v;
    let [r2, q2, _] = b.u;
    let [t2, s2] = b.v;
    let dq = f.sub(q1, q2);
    let dr = f.sub(r1, r2);
    let dt = f.sub(t1, t2);
    let ds = f.sub(s1, s2);
    let common = f.sub(f.mul(q2, dq), dr);
    let aa = f.sub(f.mul(dt, common), f.mul(r2, f.mul(dq, ds)));
    let bb = f.sub(f.mul(dr, common), f.mul(r2, f.sq(dq)));
    let cc = f.sub(f.mul(dq, dt), f.mul(dr, ds));
    if bb == 0 || cc == 0 {
        return None;
    }
    let c2 = f.sq(cc);
    let c3 = f.mul(c2, cc);
    let c5 = f.mul(c3, c2);
    let b2 = f.sq(bb);
    let qn = f.sub(f.add(f.mul(dq, c2), f.mul(2, f.mul(aa, cc))), b2);
    let rn = f.sub(
        f.add(
            f.add(f.mul(dq, f.mul(aa, cc)), f.sq(aa)),
            f.mul(f.add(q1, q2), b2),
        ),
        f.mul(f.add(s1, s2), f.mul(bb, cc)),
    );
    let delta_r = f.sub(f.mul(r1, c2), rn);
    let delta_q = f.sub(f.mul(q1, c2), qn);
    let sn = f.sub(
        f.add(
            f.sub(f.mul(delta_r, c2), f.mul(qn, delta_q)),
            f.mul(delta_q, f.mul(aa, cc)),
        ),
        f.mul(s1, f.mul(bb, c3)),
    );
    let tn = f.sub(
        f.sub(f.mul(delta_r, f.mul(aa, c3)), f.mul(rn, f.mul(delta_q, c2))),
        f.mul(t1, f.mul(bb, c5)),
    );
    Some(ResultPoint::DegreeTwo([qn, f.mul(rn, c2), sn, tn, cc, bb]))
}
fn double22_generic(a: Divisor, curve: &[u64], f: &Field) -> Option<ResultPoint> {
    // Equations (7) and (9), denominators cleared into output weights.
    assert_eq!(a.degree, 2);
    let [r, q, _] = a.u;
    let [t, s] = a.v;
    let q2 = f.sq(q);
    let common = f.add(
        f.sub(f.mul(f.sub(f.add(q2, curve[3]), f.mul(4, r)), q), curve[2]),
        f.sq(s),
    );
    let common2 = f.add(f.sub(f.mul(3, q2), f.mul(2, r)), curve[3]);
    let qs_t = f.sub(f.mul(q, s), t);
    let aa = f.add(f.mul(common, qs_t), f.mul(common2, f.mul(r, s)));
    let bb = f.sub(f.mul(2, f.mul(qs_t, t)), f.mul(2, f.mul(r, f.sq(s))));
    let cc = f.add(f.mul(common, s), f.mul(common2, t));
    if bb == 0 || cc == 0 {
        return None;
    }
    let c2 = f.sq(cc);
    let c3 = f.mul(c2, cc);
    let c5 = f.mul(c3, c2);
    let b2 = f.sq(bb);
    let qn = f.sub(f.mul(2, f.mul(aa, cc)), b2);
    let rn = f.sub(
        f.add(f.sq(aa), f.mul(2, f.mul(q, b2))),
        f.mul(2, f.mul(s, f.mul(bb, cc))),
    );
    let delta_r = f.sub(f.mul(r, c2), rn);
    let delta_q = f.sub(f.mul(q, c2), qn);
    let sn = f.sub(
        f.add(
            f.sub(f.mul(delta_r, c2), f.mul(qn, delta_q)),
            f.mul(delta_q, f.mul(aa, cc)),
        ),
        f.mul(s, f.mul(bb, c3)),
    );
    let tn = f.sub(
        f.sub(f.mul(delta_r, f.mul(aa, c3)), f.mul(rn, f.mul(delta_q, c2))),
        f.mul(t, f.mul(bb, c5)),
    );
    Some(ResultPoint::DegreeTwo([qn, f.mul(rn, c2), sn, tn, cc, bb]))
}
fn add12_weighted_generic(a: D1, d2: [u64; 6], curve: &[u64], f: &Field) -> Option<ResultPoint> {
    // Equation (17), with both operands kept in weighted Jacobian coordinates.
    let [q, r, s, t, z, w] = d2;
    let z2 = f.sq(z);
    let z4 = f.sq(z2);
    let z5 = f.mul(z4, z);
    let az2 = f.sq(a.z);
    let az4 = f.sq(az2);
    let az3 = f.mul(az2, a.z);
    let az5 = f.mul(az4, a.z);
    let k = f.mul(
        f.add(
            f.add(f.mul(q, f.mul(z2, f.mul(a.x, az2))), f.mul(f.sq(a.x), z4)),
            f.mul(r, az4),
        ),
        a.w,
    );
    if k == 0 {
        return None;
    }
    let aa = f.sub(
        f.mul(a.y, f.mul(z5, w)),
        f.mul(
            f.add(f.mul(s, f.mul(az3, f.mul(a.x, z2))), f.mul(t, az5)),
            a.w,
        ),
    );
    let bb = f.add(f.mul(s, f.mul(k, a.z)), f.mul(aa, q));
    let cc = f.add(f.mul(t, f.mul(k, a.z)), f.mul(aa, r));
    let ll = f.mul(k, f.mul(a.z, w));
    let mm = f.mul(k, f.mul(z, w));
    let ll2 = f.sq(ll);
    let ll4 = f.sq(ll2);
    let mm2 = f.sq(mm);
    let q3 = f.sub(f.sub(f.mul(a.x, mm2), f.mul(q, ll2)), f.sq(aa));
    let r3 = f.add(
        f.sub(
            f.mul(f.add(f.sub(f.mul(curve[3], z4), r), f.sq(q)), ll4),
            f.mul(aa, f.mul(f.add(bb, f.mul(s, f.mul(k, a.z))), ll2)),
        ),
        f.mul(q3, f.mul(a.x, mm2)),
    );
    let s3 = f.sub(f.mul(aa, q3), f.mul(bb, ll2));
    let t3 = f.sub(f.mul(aa, r3), f.mul(cc, ll4));
    Some(ResultPoint::DegreeTwo([
        q3,
        r3,
        s3,
        t3,
        f.mul(k, f.mul(z, f.mul(a.z, w))),
        1,
    ]))
}
fn triple(a: D1, curve: &[u64], f: &Field) -> ResultPoint {
    // Equation (16), the repeated-support 1+2 case.
    let x2 = f.sq(a.x);
    let x3 = f.mul(x2, a.x);
    let x4 = f.sq(x2);
    let z2 = f.sq(a.z);
    let z4 = f.sq(z2);
    let z6 = f.mul(z4, z2);
    let z8 = f.sq(z4);
    let y2 = f.sq(a.y);
    let fp = f.mul(
        f.add(
            f.add(
                f.add(
                    f.mul(curve[1], z8),
                    f.mul(2, f.mul(curve[2], f.mul(a.x, z6))),
                ),
                f.mul(3, f.mul(curve[3], f.mul(x2, z4))),
            ),
            f.mul(5, x4),
        ),
        a.w,
    );
    let fpp = f.mul(
        2,
        f.add(
            f.add(
                f.mul(curve[2], z6),
                f.mul(3, f.mul(curve[3], f.mul(a.x, z4))),
            ),
            f.mul(10, x3),
        ),
    );
    let k = f.mul(8, f.mul(y2, a.y));
    let aa = f.mul(f.sub(f.mul(2, f.mul(fpp, y2)), f.sq(fp)), a.w);
    let bb = f.sub(f.mul(2, f.mul(fp, y2)), f.mul(aa, a.x));
    let cc = f.sub(
        f.mul(a.y, k),
        f.mul(f.add(f.mul(aa, a.x), f.mul(2, bb)), f.mul(a.x, a.w)),
    );
    let k2 = f.sq(k);
    let k4 = f.sq(k2);
    let z3 = f.mul(k, a.z);
    let q3 = f.sub(f.mul(3, f.mul(a.x, k2)), f.sq(aa));
    let r3 = f.sub(
        f.mul(curve[3], f.sq(f.sq(z3))),
        f.mul(
            f.sub(
                f.mul(4, f.mul(aa, bb)),
                f.mul(3, f.mul(a.x, f.sub(q3, f.mul(a.x, k2)))),
            ),
            k2,
        ),
    );
    let s3 = f.sub(
        f.mul(aa, f.mul(q3, a.w)),
        f.mul(2, f.mul(bb, f.mul(k2, a.w))),
    );
    let t3 = f.sub(f.mul(aa, f.mul(r3, a.w)), f.mul(cc, k4));
    ResultPoint::DegreeTwo([q3, r3, s3, t3, z3, a.w])
}
fn add12(a: D1, b: &Divisor, curve: &[u64], f: &Field) -> (Option<ResultPoint>, &'static str) {
    assert_eq!(b.degree, 2);
    // Equation (18), with the degree-two input in affine Jacobian coordinates.
    let [r2, q2, _] = b.u;
    let [t2, s2] = b.v;
    let z1sq = f.sq(a.z);
    let z1four = f.sq(z1sq);
    let z1three = f.mul(z1sq, a.z);
    let z1five = f.mul(z1four, a.z);
    let k = f.mul(
        f.add(
            f.add(f.mul(q2, f.mul(a.x, z1sq)), f.sq(a.x)),
            f.mul(r2, z1four),
        ),
        a.w,
    );
    if k == 0 {
        let point_y = f.mul(
            f.add(f.mul(s2, f.mul(z1three, a.x)), f.mul(t2, z1five)),
            a.w,
        );
        if a.y == f.neg(point_y) {
            // Equation (15): cancel the opposite point in the degree-two support.
            let x4 = f.sub(f.neg(f.mul(q2, z1sq)), a.x);
            let y4 = f.add(f.mul(s2, f.mul(z1three, x4)), f.mul(t2, z1five));
            return (
                Some(ResultPoint::DegreeOne(D1 {
                    x: x4,
                    y: y4,
                    z: a.z,
                    w: 1,
                })),
                "cancellation",
            );
        }
        if a.y == point_y {
            // Equation (17) after equation (13): 2P plus the remaining P4.
            let x4 = f.sub(f.neg(f.mul(q2, z1sq)), a.x);
            let y4 = f.add(f.mul(s2, f.mul(z1three, x4)), f.mul(t2, z1five));
            let ResultPoint::DegreeTwo(twice) = dbl(a, curve, f) else {
                unreachable!()
            };
            if let Some(output) = add12_weighted_generic(
                D1 {
                    x: x4,
                    y: y4,
                    z: a.z,
                    w: 1,
                },
                twice,
                curve,
                f,
            ) {
                return (Some(output), "coincident_double_plus_other");
            }
            assert_eq!(x4, a.x, "remaining point must equal repeated point");
            assert_eq!(f.mul(y4, a.w), a.y);
            return (Some(triple(a, curve, f)), "tripling");
        }
        return (None, "coincident_noninverse");
    }
    let aa = f.sub(
        a.y,
        f.mul(
            f.add(f.mul(s2, f.mul(z1three, a.x)), f.mul(t2, z1five)),
            a.w,
        ),
    );
    let bb = f.add(f.mul(s2, f.mul(k, a.z)), f.mul(aa, q2));
    let cc = f.add(f.mul(t2, f.mul(k, a.z)), f.mul(aa, r2));
    let ll = f.mul(k, a.z);
    let mm = k;
    let ll2 = f.sq(ll);
    let ll4 = f.sq(ll2);
    let mm2 = f.sq(mm);
    let q3 = f.sub(f.sub(f.mul(a.x, mm2), f.mul(q2, ll2)), f.sq(aa));
    let r3 = f.add(
        f.sub(
            f.mul(f.add(f.sub(curve[3], r2), f.sq(q2)), ll4),
            f.mul(aa, f.mul(f.add(bb, f.mul(s2, f.mul(k, a.z))), ll2)),
        ),
        f.mul(q3, f.mul(a.x, mm2)),
    );
    let s3 = f.sub(f.mul(aa, q3), f.mul(bb, ll2));
    let t3 = f.sub(f.mul(aa, r3), f.mul(cc, ll4));
    (
        Some(ResultPoint::DegreeTwo([q3, r3, s3, t3, ll, 1])),
        "generic",
    )
}
fn choose(n: usize, k: usize) -> u64 {
    let mut out = 1;
    for i in 0..k {
        out = out * (n - i) as u64 / (i + 1) as u64;
    }
    out
}
fn transform(poly: &[u64], alpha: u64, beta: u64, scale: u64, f: &Field) -> Vec<u64> {
    let mut out = vec![0; poly.len()];
    let scale_inv = f.inv(scale);
    for (k, &coefficient) in poly.iter().enumerate() {
        for (j, slot) in out.iter_mut().enumerate().take(k + 1) {
            let term = f.mul(
                coefficient,
                f.mul(
                    choose(k, j) % f.p,
                    f.mul(f.pow(alpha, j as u64), f.pow(beta, (k - j) as u64)),
                ),
            );
            *slot = f.add(*slot, f.mul(term, scale_inv));
        }
    }
    while out.last() == Some(&0) {
        out.pop();
    }
    out
}
fn normal_curve(p: u64) -> Jacobian {
    let s = Scholten::new(p);
    let old = &s.j.polynomial;
    let lead = old[5];
    let shift = s.j.f.neg(s.j.f.div(old[4], s.j.f.mul(5, lead)));
    let normalized = transform(old, lead, shift, s.j.f.pow(lead, 6), &s.j.f);
    assert_eq!(normalized.len(), 6);
    assert_eq!(normalized[5], 1);
    assert_eq!(normalized[4], 0);
    Jacobian {
        f: Field::new(p, 1, 0),
        polynomial: normalized,
    }
}
fn dbl(d: D1, a: &[u64], f: &Field) -> ResultPoint {
    let z2 = f.sq(d.z);
    let z4 = f.sq(z2);
    let z6 = f.mul(z4, z2);
    let z8 = f.sq(z4);
    let z10 = f.mul(z8, z2);
    let x2 = f.sq(d.x);
    let x3 = f.mul(x2, d.x);
    let x4 = f.sq(x2);
    let x5 = f.mul(x4, d.x);
    let q = f.neg(f.mul(2, d.x));
    let r = x2;
    let s = f.mul(
        f.add(
            f.add(
                f.add(f.mul(a[1], z8), f.mul(2, f.mul(a[2], f.mul(d.x, z6)))),
                f.mul(3, f.mul(a[3], f.mul(x2, z4))),
            ),
            f.mul(5, x4),
        ),
        d.w,
    );
    let t = f.mul(
        f.sub(
            f.sub(
                f.add(f.mul(2, f.mul(a[0], z10)), f.mul(a[1], f.mul(d.x, z8))),
                f.mul(a[3], f.mul(x3, z4)),
            ),
            f.mul(3, x5),
        ),
        d.w,
    );
    ResultPoint::DegreeTwo([q, r, s, t, d.z, f.mul(2, d.y)])
}
fn add11(a: D1, b: D1, curve: &[u64], f: &Field) -> (ResultPoint, &'static str) {
    let az2 = f.sq(a.z);
    let bz2 = f.sq(b.z);
    let axbz = f.mul(a.x, bz2);
    let bxaz = f.mul(b.x, az2);
    let ay = f.mul(a.y, f.mul(f.pow(b.z, 5), b.w));
    let by = f.mul(b.y, f.mul(f.pow(a.z, 5), a.w));
    if axbz == bxaz && ay == f.neg(by) {
        return (ResultPoint::Identity, "inverse");
    }
    if axbz == bxaz && ay == by {
        return (dbl(a, curve, f), "doubling");
    }
    assert_ne!(axbz, bxaz);
    let q = f.neg(f.add(axbz, bxaz));
    let r = f.mul(axbz, bxaz);
    let s = f.sub(ay, by);
    let t = f.sub(f.mul(axbz, by), f.mul(bxaz, ay));
    let z = f.mul(a.z, b.z);
    let w = f.mul(f.sub(axbz, bxaz), f.mul(a.w, b.w));
    (ResultPoint::DegreeTwo([q, r, s, t, z, w]), "distinct")
}
fn normalize(result: ResultPoint, f: &Field) -> Divisor {
    let (q, r, s, t, z, w) = match result {
        ResultPoint::Identity => return Divisor::O,
        ResultPoint::DegreeOne(d) => {
            let iz = f.inv(d.z);
            let iw = f.inv(d.w);
            let x = f.mul(d.x, f.sq(iz));
            let y = f.mul(d.y, f.mul(f.pow(iz, 5), iw));
            return Divisor {
                degree: 1,
                u: [f.neg(x), 1, 0],
                v: [y, 0],
            };
        }
        ResultPoint::DegreeTwo(values) => {
            let [q, r, s, t, z, w] = values;
            (q, r, s, t, z, w)
        }
    };
    assert_ne!(z, 0);
    assert_ne!(w, 0);
    let iz = f.inv(z);
    let iw = f.inv(w);
    let iz2 = f.sq(iz);
    let iz3 = f.mul(iz2, iz);
    let iz4 = f.sq(iz2);
    let iz5 = f.mul(iz4, iz);
    Divisor {
        degree: 2,
        u: [f.mul(r, iz4), f.mul(q, iz2), 1],
        v: [f.mul(t, f.mul(iz5, iw)), f.mul(s, f.mul(iz3, iw))],
    }
}
fn evaluate(f: &Field, poly: &[u64], x: u64) -> u64 {
    poly.iter().rev().fold(0, |acc, &c| f.add(f.mul(acc, x), c))
}
fn main() {
    for p in PRIMES {
        let j = normal_curve(p);
        let mut points = Vec::new();
        for x in 0..p {
            if let Some(y) = j.f.sqrt(evaluate(&j.f, &j.polynomial, x)) {
                points.push((x, y));
                points.push((x, j.f.neg(y)));
                if points.len() >= 18 {
                    break;
                }
            }
        }
        assert_eq!(points.len(), 18);
        let mut inverse = 0;
        let mut doubling = 0;
        let mut distinct = 0;
        for &scale in &[1u64, 2] {
            for &(x1, y1) in &points {
                for &(x2, y2) in &points {
                    let mu = if scale == 1 { 1 } else { 3 };
                    let make = |x: u64, y: u64| D1 {
                        x: j.f.mul(j.f.sq(scale), x),
                        y: j.f.mul(j.f.mul(j.f.pow(scale, 5), mu), y),
                        z: scale,
                        w: mu,
                    };
                    let (sum, branch) = add11(make(x1, y1), make(x2, y2), &j.polynomial, &j.f);
                    let got = normalize(sum, &j.f);
                    let expected = j.add(j.point(x1, y1), j.point(x2, y2));
                    assert_eq!(got, expected, "p={p} x1={x1} x2={x2} {branch}");
                    match branch {
                        "inverse" => inverse += 1,
                        "doubling" => doubling += 1,
                        "distinct" => distinct += 1,
                        _ => unreachable!(),
                    }
                }
            }
        }
        let mut d2_inputs = Vec::new();
        for &(x1, y1) in &points {
            for &(x2, y2) in &points {
                let d = j.add(j.point(x1, y1), j.point(x2, y2));
                if d.degree == 2 && !d2_inputs.contains(&d) {
                    d2_inputs.push(d);
                }
                if d2_inputs.len() >= 16 {
                    break;
                }
            }
            if d2_inputs.len() >= 16 {
                break;
            }
        }
        assert_eq!(d2_inputs.len(), 16);
        let mut generic12 = 0;
        let mut cancellation12 = 0;
        let mut coincident12 = 0;
        let mut tripling12 = 0;
        let mut exceptional12 = 0;
        for &scale in &[1u64, 2] {
            for &(x, y) in &points {
                let mu = if scale == 1 { 1 } else { 3 };
                let d1 = D1 {
                    x: j.f.mul(j.f.sq(scale), x),
                    y: j.f.mul(j.f.mul(j.f.pow(scale, 5), mu), y),
                    z: scale,
                    w: mu,
                };
                for d2 in &d2_inputs {
                    let (result, branch) = add12(d1, d2, &j.polynomial, &j.f);
                    if let Some(output) = result {
                        let got = normalize(output, &j.f);
                        let expected = j.add(j.point(x, y), *d2);
                        assert_eq!(got, expected, "1+2 p={p} x={x} d2={d2:?}");
                        match branch {
                            "generic" => generic12 += 1,
                            "cancellation" => cancellation12 += 1,
                            "coincident_double_plus_other" => coincident12 += 1,
                            "tripling" => tripling12 += 1,
                            _ => unreachable!(),
                        }
                    } else {
                        exceptional12 += 1;
                    }
                }
            }
        }
        let mut d2_generic = Vec::new();
        let seed = d2_inputs[0];
        for k in 1..=32 {
            let d = j.mul(seed, k);
            if d.degree == 2 && !d2_generic.contains(&d) {
                d2_generic.push(d);
            }
            if d2_generic.len() == 16 {
                break;
            }
        }
        assert_eq!(d2_generic.len(), 16);
        let mut generic22 = 0;
        let mut doubling22 = 0;
        let mut exceptional22 = 0;
        for &a in &d2_generic {
            for &b in &d2_generic {
                let output = if a == b {
                    double22_generic(a, &j.polynomial, &j.f)
                } else {
                    add22_generic(a, b, &j.f)
                };
                if let Some(output) = output {
                    let got = normalize(output, &j.f);
                    assert_eq!(got, j.add(a, b), "2+2 p={p} a={a:?} b={b:?}");
                    if a == b {
                        doubling22 += 1;
                    } else {
                        generic22 += 1;
                    }
                } else {
                    exceptional22 += 1;
                }
            }
        }
        println!(
            "{}",
            json!({"schema":"complete-law-branches-gate/v2","p":p,
            "cases":inverse+doubling+distinct,"inverse":inverse,"doubling":doubling,
            "distinct":distinct,"projective_scales":[1,2],"checked_against_cantor":true,
            "generic_1plus2_checked":generic12,"cancellation_1plus2_checked":cancellation12,
            "coincident_1plus2_checked":coincident12,
            "tripling_1plus2_checked":tripling12,
            "exceptional_1plus2_unchecked":exceptional12,
            "generic_2plus2_checked":generic22,"doubling_2plus2_checked":doubling22,
            "exceptional_2plus2_unchecked":exceptional22})
        );
    }
}
