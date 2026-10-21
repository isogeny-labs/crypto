use crate::{
    extension_order, identity,
    jacobian::{Poly, Ring},
    next,
    toy_field::*,
    write_json, Fixture, Result,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, path::Path};
#[derive(Clone)]
struct Velu {
    h: Vec<u64>,
    n: Vec<u64>,
    m: Vec<u64>,
    a: u64,
    b: u64,
    ell: u64,
}
fn derivative(k: Sext, p: &[S]) -> Poly {
    Ring(k).trim(
        p.iter()
            .enumerate()
            .skip(1)
            .map(|(i, x)| k.mul(*x, k.c(i as u64)))
            .collect(),
    )
}
fn coefficients(k: Sext, p: &[S]) -> Vec<u64> {
    p.iter()
        .map(|x| k.fp(*x).expect("kernel map not defined over Fp"))
        .collect()
}
fn from_kernel(k: Sext, a: u64, b: u64, g: SP, ell: u64) -> Velu {
    let r = Ring(k);
    assert!(g.is_some());
    assert_eq!(k.pm(g, ell as u128, k.c(a)), None);
    let mut half = vec![];
    let mut h = vec![ONE];
    let mut tsum = Z;
    let mut wsum = Z;
    for i in 1..ell {
        let q = k.pm(g, i as u128, k.c(a)).unwrap();
        let xx = k.mul(q.0, q.0);
        tsum = k.add(tsum, k.add(k.mul(k.c(3), xx), k.c(a)));
        wsum = k.add(
            wsum,
            k.add(
                k.add(k.mul(k.c(5), k.mul(xx, q.0)), k.mul(k.c(3 * a % k.p), q.0)),
                k.c(2 * b % k.p),
            ),
        );
        if i <= (ell - 1) / 2 {
            half.push(q);
            h = r.mul(&h, &[k.neg(q.0), ONE]);
        }
    }
    let aa = k.sub(k.c(a), k.mul(k.c(5), tsum));
    let bb = k.sub(k.c(b), k.mul(k.c(7), wsum));
    let mut n = r.mul(&[Z, ONE], &r.mul(&h, &h));
    for (x, y) in half {
        let q = r.exact(&h, &[k.neg(x), ONE]);
        let t = k.mul(k.c(2), k.add(k.mul(k.c(3), k.mul(x, x)), k.c(a)));
        let u = k.mul(k.c(4), k.mul(y, y));
        n = r.add(
            &n,
            &r.add(&r.scale(&r.mul(&q, &h), t), &r.scale(&r.mul(&q, &q), u)),
        );
    }
    let m = r.sub(
        &r.mul(&derivative(k, &n), &h),
        &r.scale(&r.mul(&n, &derivative(k, &h)), k.c(2)),
    );
    Velu {
        h: coefficients(k, &h),
        n: coefficients(k, &n),
        m: coefficients(k, &m),
        a: k.fp(aa).unwrap(),
        b: k.fp(bb).unwrap(),
        ell,
    }
}
fn eval(poly: &[u64], x: u64, p: u64) -> u64 {
    poly.iter().rev().fold(0, |v, c| (v * x + c) % p)
}
impl Velu {
    fn fp(&self, e: Quad, p: P) -> P {
        let Some((x, y)) = p else { return None };
        assert_eq!((x.1, y.1), (0, 0));
        let h = eval(&self.h, x.0, e.p);
        if h == 0 {
            return None;
        }
        let hi = inv(h, e.p);
        Some((
            Q(eval(&self.n, x.0, e.p) * hi % e.p * hi % e.p, 0),
            Q(
                y.0 * eval(&self.m, x.0, e.p) % e.p * hi % e.p * hi % e.p * hi % e.p,
                0,
            ),
        ))
    }
    fn ext(&self, k: Sext, p: SP) -> SP {
        let Some((x, y)) = p else { return None };
        let r = Ring(k);
        let h = r.eval(&self.h.iter().map(|c| k.c(*c)).collect::<Vec<_>>(), x);
        if h == Z {
            return None;
        }
        let n = r.eval(&self.n.iter().map(|c| k.c(*c)).collect::<Vec<_>>(), x);
        let m = r.eval(&self.m.iter().map(|c| k.c(*c)).collect::<Vec<_>>(), x);
        Some((k.div(n, k.mul(h, h)), k.div(k.mul(y, m), k.pow(h, 3))))
    }
}
fn point(
    k: Sext,
    e: Quad,
    order: u64,
    ell: u64,
    lambda: Option<u64>,
    seed: &mut u64,
) -> Option<(SP, u64)> {
    let mut cofactor = extension_order(e.p, order, 6);
    while cofactor % ell as u128 == 0 {
        cofactor /= ell as u128;
    }
    for trial in 0..128 {
        let x = S(std::array::from_fn(|_| next(seed) % e.p));
        let rhs = k.add(k.add(k.mul(k.mul(x, x), x), k.mul(k.c(e.a), x)), k.c(e.b));
        let Some(y) = k.sqrt(rhs) else { continue };
        let mut q = k.pm(Some((x, y)), cofactor, k.c(e.a));
        if q.is_none() {
            continue;
        }
        while k.pm(q, ell as u128, k.c(e.a)).is_some() {
            q = k.pm(q, ell as u128, k.c(e.a));
        }
        if let Some(l) = lambda {
            let t = (e.p as i128 + 1 - order as i128).rem_euclid(ell as i128) as u64;
            let other = (t + ell - l) % ell;
            if l != other {
                q = k.pa(k.frob(q), k.pn(k.pm(q, other as u128, k.c(e.a))), k.c(e.a));
            }
            if q.is_none() || k.frob(q) != k.pm(q, l as u128, k.c(e.a)) {
                continue;
            }
        }
        if q.is_some() {
            return Some((q, trial + 1));
        }
    }
    None
}
pub fn run(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut registrations = BTreeMap::new();
    for f in fixtures
        .iter()
        .filter(|f| prime(f.order) && f.order % 2 == 1)
    {
        let e = f.curve();
        let k = Sext::new(e);
        let t = e.p as i64 + 1 - f.order as i64;
        let mut seed = 20261021 + e.p * 7919;
        rows.push(json!({"fixture":f.id,"ell":2,"status":"inapplicable: irreducible cubic gives no rational two-kernel","rational_cubic_roots":(0..e.p).filter(|x|e.rhs(Q(*x,0)).0==0).count()}));
        for ell in [3u64, 7] {
            let roots: Vec<_> = (1..ell)
                .filter(|l| {
                    ((l * l + ell - (t.rem_euclid(ell as i64) as u64) * l % ell + e.p % ell) % ell)
                        == 0
                })
                .collect();
            if roots.is_empty() {
                rows.push(json!({"fixture":f.id,"ell":ell,"status":"inapplicable: no Frobenius eigenline over F_ell"}));
                continue;
            }
            let Some((g, trials)) = point(k, e, f.order, ell, Some(roots[0]), &mut seed) else {
                rows.push(json!({"fixture":f.id,"ell":ell,"status":"bounded kernel search exhausted","trial_cap":128}));
                continue;
            };
            let map = from_kernel(k, e.a, e.b, g, ell);
            let dest = Quad::new(e.p, map.a, map.b);
            assert_eq!(dest.rational().len() as u64, f.order);
            let reg = identity(e.p, map.a, map.b, f.order as u128, None);
            let id = reg["slug"].as_str().unwrap().to_string();
            registrations.insert(id.clone(), reg);
            let mut inverse = BTreeMap::new();
            for p in e.rational() {
                let q = map.fp(e, p);
                assert!(dest.on(q));
                assert!(inverse.insert(q, p).is_none());
            }
            assert_eq!(inverse.len(), f.order as usize);
            let image_g = map.fp(e, f.group());
            assert!(image_g.is_some());
            assert_eq!(dest.pm(image_g, f.order as u128), None);
            for p in e.rational() {
                assert_eq!(
                    map.fp(e, e.pa(p, f.group())),
                    dest.pa(map.fp(e, p), image_g)
                );
                assert_eq!(inverse[&map.fp(e, p)], p);
            }
            let mut dual = None;
            let mut dual_scale = None;
            if roots.len() == 2 {
                if let Some((other, _)) = point(k, e, f.order, ell, Some(roots[1]), &mut seed) {
                    let dg = map.ext(k, other);
                    assert!(dg.is_some());
                    let dm = from_kernel(k, map.a, map.b, dg, ell);
                    let composite = dm.fp(dest, image_g);
                    let ell_g = e.pm(f.group(), ell as u128);
                    let u = (1..e.p)
                        .find(|u| {
                            e.a * pow(*u, 4, e.p) % e.p == dm.a
                                && e.b * pow(*u, 6, e.p) % e.p == dm.b
                                && ell_g.map(|(x, y)| {
                                    (
                                        Q(x.0 * pow(*u, 2, e.p) % e.p, 0),
                                        Q(y.0 * pow(*u, 3, e.p) % e.p, 0),
                                    )
                                }) == composite
                        })
                        .expect("dual model scale");
                    for p in e.rational() {
                        let lhs = dm.fp(dest, map.fp(e, p));
                        let rhs = e.pm(p, ell as u128).map(|(x, y)| {
                            (
                                Q(x.0 * pow(u, 2, e.p) % e.p, 0),
                                Q(y.0 * pow(u, 3, e.p) % e.p, 0),
                            )
                        });
                        assert_eq!(lhs, rhs);
                    }
                    for s in 0..8 {
                        let q = e.pm(
                            f.group(),
                            (1 + (20261021 + s * 7919) % (f.order - 1)) as u128,
                        );
                        let mapped = dm.fp(dest, map.fp(e, q));
                        let unscaled = mapped.map(|(x, y)| {
                            (
                                Q(x.0 * inv(pow(u, 2, e.p), e.p) % e.p, 0),
                                Q(y.0 * inv(pow(u, 3, e.p), e.p) % e.p, 0),
                            )
                        });
                        assert_eq!(e.pm(unscaled, inv(ell, f.order) as u128), q);
                    }
                    dual_scale = Some(u);
                    dual = Some(
                        json!({"kernel_polynomial":dm.h,"x_numerator":dm.n,"y_multiplier_numerator":dm.m,"normalized_target_coefficients":[dm.a,dm.b]}),
                    );
                }
            }
            let mut representations = vec![];
            for u in [1, 2, 3] {
                if u % e.p == 0 {
                    continue;
                }
                let rep = Quad::new(e.p, e.a * pow(u, 4, e.p) % e.p, e.b * pow(u, 6, e.p) % e.p);
                let reg = identity(e.p, rep.a, rep.b, f.order as u128, None);
                let name = reg["slug"].as_str().unwrap().to_string();
                registrations.insert(name.clone(), reg);
                for p in e.rational() {
                    let q = p.map(|(x, y)| {
                        (
                            Q(x.0 * pow(u, 2, e.p) % e.p, 0),
                            Q(y.0 * pow(u, 3, e.p) % e.p, 0),
                        )
                    });
                    assert!(rep.on(q));
                    assert_eq!(
                        q.map(|(x, y)| (
                            Q(x.0 * inv(pow(u, 2, e.p), e.p) % e.p, 0),
                            Q(y.0 * inv(pow(u, 3, e.p), e.p) % e.p, 0)
                        )),
                        p
                    );
                }
                representations.push(name);
            }
            rows.push(json!({"fixture":f.id,"destination":id,"ell":ell,"status":"verified-explicit-Velu-and-rational-subgroup","kernel_generator_tower_coordinates":g,"kernel_trials":trials,"frobenius_eigenvalues":roots,"kernel_polynomial":map.h,"x_numerator":map.n,"y_multiplier_numerator":map.m,"source_and_destination_points_checked":f.order,"generator_addition_homomorphism_checks":f.order,"finite_lookup_return_map_entries":inverse.len(),"algebraic_dual_return_map":dual,"dual_model_scale":dual_scale,"scalar_recovery_controls":if dual_scale.is_some(){8}else{0},"representation_isomorphism_controls":representations,"field_and_model_conversion_charged_in_pipeline":false,"IC_or_DLP_speedup":null}));
        }
        println!("{}", json!({"stage":"isogeny","fixture":f.id}));
    }
    write_json(out.join("E14-isogenies.json"), &rows)?;
    write_json(
        out.join("E14-registry-additions.json"),
        &registrations.into_values().collect::<Vec<_>>(),
    )?;
    Ok(rows)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn order_three_velu_standard_control() {
        let e = Quad::new(59, 0, 1);
        let k = Sext {
            p: 59,
            a: 1,
            b: 13,
            d: 2,
        };
        let g = Some((k.c(0), k.c(1)));
        let v = from_kernel(k, 0, 1, g, 3);
        assert_eq!(v.a, 0);
        assert_eq!(v.b, (59 - 27) % 59);
        let d = Quad::new(59, v.a, v.b);
        for p in e.rational() {
            assert!(d.on(v.fp(e, p)));
        }
    }
}
