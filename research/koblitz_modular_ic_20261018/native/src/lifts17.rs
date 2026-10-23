//! Factored theta translation lifts on the diagonal bundle; canonical character matching remains open.
use super::*;
fn decode(f: &F, v: &Value) -> E {
    E::from_biguint(
        &BigUint::parse_bytes(v.as_str().unwrap().as_bytes(), 16).unwrap(),
        f.m(),
    )
}
fn point(f: &F, v: &Value) -> P {
    if v["infinity"] == true {
        P::Infinity
    } else {
        P::Affine {
            x: decode(f, &v[0]),
            y: decode(f, &v[1]),
        }
    }
}
fn order(c: &BinaryCurve, p: &P) -> u64 {
    if *p == P::Infinity {
        return 1;
    }
    let mut n = 105;
    for l in [3, 5, 7] {
        if n % l == 0 && lf::times(c, p, n / l) == P::Infinity {
            n /= l
        }
    }
    assert_eq!(lf::times(c, p, n), P::Infinity);
    n
}
fn correction(f: &F, c: &BinaryCurve, a: &P, r: &P) -> Option<E> {
    if *a == P::Infinity {
        return Some(f.o());
    }
    let na = lf::neg(a);
    let t = P::Affine { x: f.z(), y: f.o() };
    let ta = lf::plus(c, &t, &na);
    let (P::Affine { x: ax, y: ay }, P::Affine { x: tx, .. }, P::Affine { x: rx, y: ry }) =
        (&na, &ta, r)
    else {
        return None;
    };
    let slope = f.mul(&ay.add(&f.o()), &f.inv(ax));
    let denominator = ry.add(&f.mul(&slope, rx)).add(&f.o());
    let numerator = rx.add(tx);
    if denominator.is_zero() || numerator.is_zero() {
        return None;
    }
    Some(f.mul(&numerator, &f.inv(&denominator)))
}
fn theta(f: &F, c: &BinaryCurve, a: &[P], r: &[P]) -> Option<E> {
    let mut v = f.o();
    for i in 0..4 {
        if a[i] == P::Infinity {
            continue;
        }
        let d = order(c, &a[i]);
        let n = [9, 63, 315, 855][i];
        assert_eq!(n % d, 0);
        let m = miller(f, c, &lf::neg(&a[i]), &r[i], d)?;
        if m.is_zero() {
            return None;
        }
        let g = correction(f, c, &a[i], &r[i])?;
        v = f.mul(&v, &f.mul(&f.powu(&m, n / d), &g));
    }
    Some(v)
}
fn shifted(c: &BinaryCurve, p: &[P], a: &[P]) -> Vec<P> {
    p.iter().zip(a).map(|(p, a)| lf::plus(c, p, a)).collect()
}
fn period(f: &F, c: &BinaryCurve, a: &[P], p: &[P], d: u64) -> Option<E> {
    let mut v = f.o();
    let mut q = p.to_vec();
    for _ in 0..d {
        v = f.mul(&v, &theta(f, c, a, &q)?);
        q = shifted(c, &q, a);
    }
    assert_eq!(q, p);
    Some(v)
}
fn chain(f: &F, c: &BinaryCurve, p: &P, n: u64) -> Vec<Value> {
    let mut v = p.clone();
    let mut out = vec![];
    for i in (0..63 - n.leading_zeros()).rev() {
        let q = lf::plus(c, &v, &v);
        out.push(json!({"square_previous":true,"P":lf::pj(&v),"Q":lf::pj(&v),"sum":lf::pj(&q)}));
        v = q;
        if n >> i & 1 == 1 {
            let q = lf::plus(c, &v, p);
            out.push(
                json!({"square_previous":false,"P":lf::pj(&v),"Q":lf::pj(p),"sum":lf::pj(&q)}),
            );
            v = q
        }
    }
    assert_eq!(v, P::Infinity);
    let _ = f;
    out
}
pub fn run(dir: &Path) {
    let data = read(&dir.join("g17-odd-cover-kernel-and-commutators.json"));
    let f = F::new(168, 95);
    assert!(f.irreducible());
    assert_eq!(data["field_modulus"], f.modulus().to_str_radix(16));
    let mut records = vec![];
    for row in data["records"].as_array().unwrap() {
        let graph = row["graph"].as_u64().unwrap();
        let trace = row["trace"].as_i64().unwrap();
        let c = f.curve(f.e(u64::from(trace == 1)), f.o());
        let gens = row["kernel_generators"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|v| point(&f, v))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let orders = [3u64, 3, 105, 105];
        let mut probes = vec![];
        let mut rejected = vec![];
        for attempt in 0..64 {
            let mut p = vec![];
            for i in 0..4 {
                let mut k = 1001 + attempt * 101 + i * 17;
                loop {
                    if let Some(q) = f.point(&c, &f.e(k)) {
                        p.push(q);
                        break;
                    }
                    k += 1;
                    assert!(k < 20000);
                }
            }
            let periods = gens
                .iter()
                .zip(orders)
                .map(|(a, d)| period(&f, &c, a, &p, d))
                .collect::<Option<Vec<_>>>();
            let Some(periods) = periods else {
                rejected
                    .push(json!({"attempt":attempt,"reason":"period evaluation meets zero/pole"}));
                continue;
            };
            let mut correction_checks = 0;
            for a in &gens {
                for i in 0..4 {
                    if a[i] == P::Infinity {
                        continue;
                    }
                    let P::Affine { x: s, .. } = &a[i] else {
                        unreachable!()
                    };
                    let P::Affine { x: px, .. } = &p[i] else {
                        unreachable!()
                    };
                    let P::Affine { x: qx, .. } = lf::plus(&c, &p[i], &a[i]) else {
                        panic!("accepted period includes pole")
                    };
                    let gg = correction(&f, &c, &a[i], &p[i]).unwrap();
                    assert_eq!(qx, f.mul(s, &f.mul(px, &f.sq(&gg))));
                    correction_checks += 1;
                }
            }
            let mut commutators = vec![];
            let mut good = true;
            for i in 0..4 {
                for j in i + 1..4 {
                    let values = (
                        theta(&f, &c, &gens[i], &p),
                        theta(&f, &c, &gens[j], &shifted(&c, &p, &gens[i])),
                        theta(&f, &c, &gens[j], &p),
                        theta(&f, &c, &gens[i], &shifted(&c, &p, &gens[j])),
                    );
                    if let (Some(a), Some(b), Some(c), Some(d)) = values {
                        let lhs = f.mul(&a, &b);
                        let rhs = f.mul(&c, &d);
                        assert_eq!(lhs, rhs, "theta lifts commute");
                        commutators
                            .push(json!({"pair":[i,j],"lhs":lf::ej(&lhs),"rhs":lf::ej(&rhs)}));
                    } else {
                        good = false;
                    }
                }
            }
            if !good {
                rejected.push(
                    json!({"attempt":attempt,"reason":"commutator evaluation meets zero/pole"}),
                );
                continue;
            }
            probes.push(json!({"attempt":attempt,"tuple":p.iter().map(lf::pj).collect::<Vec<_>>(),"Pic2_square_identity_checks":correction_checks,"periods":periods.iter().map(lf::ej).collect::<Vec<_>>(),"commutators":commutators}));
            if probes.len() == 3 {
                break;
            }
        }
        assert_eq!(probes.len(), 3, "valid theta probe cap");
        let mut operators = vec![];
        for i in 0..4 {
            let period = decode(&f, &probes[0]["periods"][i]);
            for r in &probes {
                assert_eq!(decode(&f, &r["periods"][i]), period)
            }
            let target = f.inv(&period);
            let mut scale = target.clone();
            let mut roots = vec![];
            for l in [3u64, 5, 7] {
                if orders[i] % l == 0 {
                    let mut p = vec![f.z(); l as usize + 1];
                    p[0] = scale.clone();
                    p[l as usize] = f.o();
                    let rs = f.roots(&p);
                    let root = rs[0].clone();
                    assert_eq!(f.powu(&root, l), scale);
                    roots.push(json!({"prime":l,"input":lf::ej(&scale),"root":lf::ej(&root),"all_roots":rs.iter().map(lf::ej).collect::<Vec<_>>()}));
                    scale = root;
                }
            }
            assert_eq!(f.powu(&scale, orders[i]), target);
            assert_eq!(f.mul(&f.powu(&scale, orders[i]), &period), f.o());
            let factors=gens[i].iter().enumerate().map(|(j,a)|{let d=order(&c,a);if *a==P::Infinity{return json!({"factor":j,"function":"1"})}let na=lf::neg(a);json!({"factor":j,"a":lf::pj(a),"exact_order":d,"line_bundle_degree":([9u64,63,315,855][j]),"Miller_point":lf::pj(&na),"Miller_exponent":([9u64,63,315,855][j]/d),"Miller_chain":chain(&f,&c,&na,d),"Pic2_correction":"(x(P)+x(T-a))/line(T,-a)(P)","T":["0","1"],"T_minus_a":lf::pj(&lf::plus(&c,&P::Affine{x:f.z(),y:f.o()},&na))})}).collect::<Vec<_>>();
            operators.push(json!({"generator":i,"order":orders[i],"period_scalar":lf::ej(&period),"normalizing_scalar":lf::ej(&scale),"root_extractions":roots,"normalized_order_checked":true,"factors":factors,"action":"s(P) -> scalar*product(factor_i(P_i))*s(P+a)","global_rational_function_argument":"Miller divisor plus explicit T-correction;period and commutator ratios have trivial divisor"}));
        }
        println!("G18:graph{graph},four factored theta lifts,orders3/3/105/105,three period probes and18 commutator controls;canonical character open");
        records.push(json!({"graph":graph,"operators":operators,"accepted_probes":probes,"rejected_probes":rejected,"geometric_kernel_linearization_constructed_under_written_divisor_bridge":true,"canonical_mixed_divisor_character_matched":false,"linearization_matrices_materialized":false,"invariant_sections_extracted":false,"target243_sections_constructed":false}));
    }
    crate::round02::save(
        dir,
        "g18-factored-odd-kernel-theta-lifts.json",
        &json!({"schema":"koblitz.round17/g18-v1","records":records,"field_degree":168,"modulus":f.modulus().to_str_radix(16),"cover_kernel_order":99225,"two_stage_section_dimensions":[8704314675u64,87723,243],"scope":"factored rational-function theta lifts with normalized generator orders and sampled native evaluation controls;global divisor bridge written;canonical character and invariant-section extraction open"}),
    );
}
