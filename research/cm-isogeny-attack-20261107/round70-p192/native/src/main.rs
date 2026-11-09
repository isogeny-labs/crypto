#![allow(dead_code, unused_imports)]
include!("helpers.rs");
fn fid<const P: i64, const D: i64>(x: F<P, D>) -> usize {
    (x.0 + P * x.1) as usize
}
struct Tables<const P: i64, const D: i64> {
    q: usize,
    elems: Vec<F<P, D>>,
    roots: Vec<Vec<usize>>,
    fx: Vec<usize>,
    chi: Vec<i64>,
}
fn tables<const P: i64, const D: i64>(a: i64, b: i64, degree: u32) -> Tables<P, D> {
    let q = P.pow(degree) as usize;
    let elems: Vec<_> = (0..q).map(|i| F::new(i as i64 % P, i as i64 / P)).collect();
    let mut roots = vec![vec![]; q];
    for (i, &y) in elems.iter().enumerate() {
        roots[fid(y * y)].push(i)
    }
    let chi: Vec<_> = roots.iter().map(|v| v.len() as i64 - 1).collect();
    let fx = elems
        .iter()
        .map(|&x| fid(x * x * x + F::c(a) * x + F::c(b)))
        .collect();
    Tables {
        q,
        elems,
        roots,
        fx,
        chi,
    }
}
fn shiftid<const P: i64, const D: i64>(t: &Tables<P, D>, id: usize, s: i64) -> usize {
    fid(t.elems[id] - F::c(s))
}
fn field_fibers<const P: i64, const D: i64>(t: &Tables<P, D>, a: i64, b: i64) -> Vec<Value> {
    let ne = 1 + t.fx.iter().map(|&id| t.roots[id].len() as i64).sum::<i64>();
    let mut out = vec![];
    for s in 1..P {
        let mut nh = 2i64;
        let mut nc = 2i64;
        let mut maps = 0usize;
        let mut wrong = 0usize;
        let mut witnesses = vec![];
        let mut principal = 0usize;
        let mut positive = 0usize;
        for x in 0..t.q {
            let xs = shiftid(t, x, s);
            let a_id = t.fx[x];
            let b_id = t.fx[xs];
            nh += 1 + t.chi[a_id] * t.chi[b_id];
            nc += (t.roots[a_id].len() * t.roots[b_id].len()) as i64;
            let rhs = t.elems[a_id] * t.elems[b_id];
            for &yi in &t.roots[a_id] {
                for &wi in &t.roots[b_id] {
                    let y = t.elems[yi];
                    let w = t.elems[wi];
                    let z = y * w;
                    assert_eq!(z * z, rhs);
                    assert_eq!(t.elems[x] - t.elems[xs], F::c(s));
                    maps += 1;
                    let bad = y + w;
                    if bad * bad != rhs {
                        wrong += 1
                    }
                    if witnesses.len() < 4 {
                        witnesses.push(json!({"C":[t.elems[x].code(),y.code(),w.code()],"H":[t.elems[x].code(),z.code()]}))
                    }
                }
            }
            if t.q == (P * P) as usize && t.elems[x].1 == 0 {
                for &yi in &t.roots[a_id] {
                    let y = t.elems[yi];
                    if y.1 != 0 {
                        continue;
                    }
                    let ws = &t.roots[b_id];
                    assert_eq!(ws.len(), 2);
                    let z0 = y * t.elems[ws[0]];
                    let z1 = y * t.elems[ws[1]];
                    assert_eq!(z0 + z1, F::c(0));
                    principal += 1;
                    let d = add(Some((t.elems[x], y)), Some((t.elems[x], y)), a);
                    assert!(d.is_some());
                    positive += 1;
                }
            }
        }
        assert_eq!(nc, 2 * ne + nh - 2 * (t.q as i64 + 1));
        assert_eq!(maps as i64, nc - 2);
        assert!(wrong > 0);
        out.push(json!({"s":s,"q":t.q,"E_points":ne,"H_points":nh,"C_points":nc,"affine_C_to_H_maps_checked":maps,"wrong_Z_addition_rejections":wrong,"wrong_infinity_correction_rejected":nc-1!=2*ne+nh-2*(t.q as i64+1),"principal_conjugate_pullback_pairs":principal,"positive_elliptic_double_nonzero":positive,"point_map_witnesses":witnesses}));
    }
    out
}
fn poly_transform<const P: i64, const D: i64>(
    f: &[F<P, D>],
    a: &[F<P, D>],
    b: &[F<P, D>],
) -> Vec<F<P, D>> {
    let n = f.len() - 1;
    let mut ap = vec![vec![F::c(1)]];
    let mut bp = ap.clone();
    for i in 1..=n {
        ap.push(pm(&ap[i - 1], a));
        bp.push(pm(&bp[i - 1], b))
    }
    let mut out = vec![F::c(0)];
    for i in 0..=n {
        pa(&mut out, &pm(&ap[i], &bp[n - i]), f[i])
    }
    trim(out)
}
fn cubic_j<const P: i64, const D: i64>(q: &[F<P, D>]) -> F<P, D> {
    let b = q[2];
    let c = q[1] * q[3];
    let d = q[0] * q[3] * q[3];
    let b2 = F::c(4) * b;
    let b4 = F::c(2) * c;
    let b6 = F::c(4) * d;
    let b8 = F::c(4) * b * d - c * c;
    let disc = -b2 * b2 * b8 - F::c(8) * b4 * b4 * b4 - F::c(27) * b6 * b6 + F::c(9) * b2 * b4 * b6;
    assert_ne!(disc, F::c(0));
    let c4 = b2 * b2 - F::c(24) * b4;
    c4 * c4 * c4 / disc
}
fn quotient_map<const P: i64, const D: i64>(
    x: F<P, D>,
    z: F<P, D>,
    l: F<P, D>,
    m: F<P, D>,
) -> [Pt<P, D>; 2] {
    if x == m {
        return [None, Some((F::c(0), z))];
    }
    if x == l {
        return [Some((F::c(0), -z)), None];
    }
    let u = (x - l) / (x - m);
    let v = z * (u - F::c(1)).pow(3);
    [Some((u * u, v)), Some((F::c(1) / (u * u), v / u.pow(3)))]
}
fn cubic_on<const P: i64, const D: i64>(p: Pt<P, D>, q: &[F<P, D>]) -> bool {
    p.map(|(x, y)| y * y == ev(q, x)).unwrap_or(true)
}
fn elliptic_card<const P: i64, const D: i64>(q: &[F<P, D>], tab: &Tables<P, D>) -> i64 {
    1 + tab
        .elems
        .iter()
        .map(|&x| tab.roots[fid(ev(q, x))].len() as i64)
        .sum::<i64>()
}
fn special<const P: i64, const D: i64>(
    a: i64,
    b: i64,
    t1: &Tables<P, D>,
    t2: &Tables<P, D>,
    fibers: &[Value],
) -> Value {
    if a == 0 {
        return json!({"status":"not_applicable_a_zero","fibers_still_enumerated":P-1});
    }
    let disc = (-4 * a * a * a - 27 * b * b).rem_euclid(P);
    let root = (1..P).find(|&v| v * v % P == disc).unwrap();
    let i = t2.elems[t2.roots[fid(F::c(-1))][0]];
    assert_eq!(i * i, F::c(-1));
    let mut rows = vec![];
    for d in [F::c(root), F::c(-root)] {
        let s = d / F::c(3 * a);
        assert_eq!(s.1, 0);
        let t = (s - F::c(3 * b) / F::c(a)) / F::c(2);
        let bb = F::c(a) / F::c(3) - t * s;
        assert_eq!(t * t + bb, -s * s);
        let f = vec![F::c(b), F::c(a), F::c(0), F::c(1)];
        let fs = vec![
            F::c(b) - F::c(a) * s - s.pow(3),
            F::c(a) + F::c(3) * s * s,
            F::c(-3) * s,
            F::c(1),
        ];
        let h = pm(&f, &fs);
        let tr = poly_transform(&h, &[bb, t], &[-t, F::c(1)]);
        assert_eq!(tr, trim(scaled(&h, -s.pow(6))));
        let l = t + i * s;
        let m = t - i * s;
        let g = poly_transform(&h, &[-l, m], &[F::c(-1), F::c(1)]);
        assert_eq!(g.len(), 7);
        for k in [1, 3, 5] {
            assert_eq!(g[k], F::c(0))
        }
        let qs = [vec![g[0], g[2], g[4], g[6]], vec![g[6], g[4], g[2], g[0]]];
        let js = qs.iter().map(|q| cubic_j(q).code()).collect::<Vec<_>>();
        let mut checks = vec![];
        for tab in [t1, t2] {
            let mut checked = 0;
            let mut exceptional = 0;
            for &x in &tab.elems {
                let rhs = ev(&h, x);
                if fid(rhs) >= tab.q {
                    panic!("base fiber not defined over tested field")
                }
                for &zi in &tab.roots[fid(rhs)] {
                    let z = tab.elems[zi];
                    let images = quotient_map(x, z, l, m);
                    if x == l || x == m {
                        exceptional += 1
                    }
                    for j in 0..2 {
                        assert!(cubic_on(images[j], &qs[j]))
                    }
                    checked += 1;
                }
            }
            for sign in [F::c(1), F::c(-1)] {
                let v = sign * (m - l).pow(3);
                assert!(cubic_on(Some((F::c(1), v)), &qs[0]));
                assert!(cubic_on(Some((F::c(1), v)), &qs[1]));
            }
            checks.push(json!({"source_q":tab.q,"affine_H_points_checked":checked,"finite_exception_chart_points":exceptional,"infinity_points_checked":2}));
        }
        let fp_defined = qs.iter().flatten().all(|c| c.1 == 0);
        let n2: Vec<_> = qs.iter().map(|q| elliptic_card(q, t2)).collect();
        let n1 = if fp_defined {
            Some(qs.iter().map(|q| elliptic_card(q, t1)).collect::<Vec<_>>())
        } else {
            None
        };
        let jac = fibers[(s.0 - 1) as usize]["JacH_order"].as_i64().unwrap();
        let order_match = n1
            .as_ref()
            .map(|n| n[0] * n[1] == jac)
            .unwrap_or(n2[0] == jac);
        assert!(order_match);
        rows.push(json!({"s":s.code(),"delta_f":d.code(),"t":t.code(),"B":bb.code(),"l":l.code(),"m":m.code(),"Hs_coefficients":h.iter().map(|x|x.code()).collect::<Vec<_>>(),"even_sextic":g.iter().map(|x|x.code()).collect::<Vec<_>>(),"quotient_cubics":qs.iter().map(|q|q.iter().map(|x|x.code()).collect::<Vec<_>>()).collect::<Vec<_>>(),"quotient_j":js,"quotients_defined_over_Fp":fp_defined,"quotient_orders_Fp":n1,"quotient_orders_Fp2":n2,"JacH_order_matches_descent":order_match,"point_map_checks":checks}));
    }
    json!({"status":"constructed","rows":rows})
}
fn panel<const P: i64, const D: i64>(a: i64, b: i64, r: i64) -> Value {
    let t1 = tables::<P, D>(a, b, 1);
    let t2 = tables::<P, D>(a, b, 2);
    assert!(t1.fx.iter().all(|&id| id != 0));
    let q1 = field_fibers(&t1, a, b);
    let q2 = field_fibers(&t2, a, b);
    let ne = q1[0]["E_points"].as_i64().unwrap();
    assert_eq!(ne, r);
    let trace = P + 1 - r;
    assert_eq!(
        q2[0]["E_points"].as_i64().unwrap(),
        (P + 1) * (P + 1) - trace * trace
    );
    let mut rows = vec![];
    for i in 0..(P - 1) as usize {
        let n1 = q1[i]["H_points"].as_i64().unwrap();
        let n2 = q2[i]["H_points"].as_i64().unwrap();
        let a1 = P + 1 - n1;
        let num = a1 * a1 - (P * P + 1 - n2);
        assert_eq!(num % 2, 0);
        let a2 = num / 2;
        let jac = 1 + P * P + a2 - (P + 1) * a1;
        assert_eq!(jac, (n1 * n1 + n2) / 2 - P);
        let u = a1 - trace;
        let has_e = a2 == 2 * P + trace * u && u * u <= 4 * P;
        rows.push(json!({"s":i+1,"Fp":q1[i],"Fp2":q2[i],"L_H":[1,-a1,a2,-P*a1,P*P],"JacH_order":jac,"r_divides_JacH":jac%r==0,"E_polynomial_divides_LH":has_e,"other_trace_if_divides":if has_e{Some(u)}else{None}}));
    }
    for i in 0..(P - 1) as usize {
        let j = (P - 2) - i as i64;
        assert_eq!(rows[i]["L_H"], rows[j as usize]["L_H"]);
    }
    let specials = special(a, b, &t1, &t2, &rows);
    json!({"p":P,"quadratic_D":D,"a":a,"b":b,"r":r,"E_trace":trace,"fibers":rows,"special_fibers":specials,"boundary_torus_order":P*P+P+1,"boundary_r_gcd":gcd_i(P*P+P+1,r)})
}
fn gcd_i(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r
    }
    a
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let out = json!({"all_checks_pass":true,"panels":[panel::<11,2>(2,7,7),panel::<17,3>(2,2,19),panel::<211,210>(0,7,199)],"whole_qualifier_counts":counts(),"scope":"236 complete nonzero Fp fibers; point counts over Fp/Fp2 and explicit maps; no automatic subgroup transfer"});
    if let Some(i) = args.iter().position(|s| s == "--check") {
        let old: Value = serde_json::from_slice(&fs::read(&args[i + 1]).unwrap()).unwrap();
        assert_eq!(out, old, "changed fiber certificate")
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap())
}
