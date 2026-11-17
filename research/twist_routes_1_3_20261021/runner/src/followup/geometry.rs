//! Explicit BHLS genus-two family and exact cleared-denominator map replay.
use super::{
    bigfield::{BPoly, Field},
    Source,
};
use crate::{write_json, Result};
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::{fs, path::Path};

fn strings(f: &[BigUint]) -> Vec<String> {
    f.iter().map(ToString::to_string).collect()
}
fn scale(k: &Field, f: &[BigUint], s: &BigUint) -> BPoly {
    k.trim(f.iter().map(|v| k.mulp(v, s)).collect())
}
fn add(k: &Field, f: &[BigUint], g: &[BigUint]) -> BPoly {
    let ng = scale(k, g, &(&k.p - 1u32));
    k.psub(f, &ng)
}
fn power(k: &Field, f: &[BigUint], n: u32) -> BPoly {
    (0..n).fold(vec![BigUint::one()], |r, _| k.pmul(&r, f))
}
fn derivative(k: &Field, f: &[BigUint]) -> BPoly {
    k.trim(
        f.iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| k.mulp(c, &BigUint::from(i)))
            .collect(),
    )
}
fn cube(k: &Field, x: &BigUint) -> BigUint {
    k.mulp(&k.mulp(x, x), x)
}
fn val(k: &Field, n: u64) -> BigUint {
    BigUint::from(n) % &k.p
}
pub fn family(
    k: &Field,
    a: &BigUint,
    b: &BigUint,
    c: &BigUint,
    d: &BigUint,
) -> (BPoly, [BPoly; 2], [BPoly; 2], [BPoly; 2], [[BigUint; 3]; 2]) {
    let s = |n, x: &BigUint| k.mulp(&val(k, n), x);
    let da = k.addp(&cube(k, a), &k.mulp(b, b));
    let dc = k.addp(&cube(k, c), &k.mulp(d, d));
    assert!(!da.is_zero() && !dc.is_zero());
    assert_eq!(
        k.addp(&s(12, &k.mulp(a, c)), &s(16, &k.mulp(b, d))),
        BigUint::one()
    );
    let h1 = vec![s(2, b), s(3, a), BigUint::zero(), BigUint::one()];
    let h2 = k.trim(vec![BigUint::one(), BigUint::zero(), s(3, c), s(2, d)]);
    let n1 = scale(
        k,
        &[c.clone(), k.subp(&BigUint::zero(), &s(2, d))],
        &s(12, &da),
    );
    let n2 = scale(
        k,
        &[
            BigUint::zero(),
            BigUint::zero(),
            k.subp(&BigUint::zero(), &s(2, b)),
            a.clone(),
        ],
        &s(12, &dc),
    );
    let v1 = scale(
        k,
        &[
            &k.p - 1u32,
            BigUint::zero(),
            k.subp(&BigUint::zero(), &s(12, c)),
            s(16, d),
        ],
        &da,
    );
    let v2 = scale(
        k,
        &[
            k.subp(&BigUint::zero(), &s(16, b)),
            s(12, a),
            BigUint::zero(),
            BigUint::one(),
        ],
        &dc,
    );
    let e12 = s(12, &k.subp(&s(2, &k.mulp(&k.mulp(a, a), d)), &k.mulp(b, c)));
    let e14 = s(
        12,
        &k.mulp(
            &k.addp(&s(16, &k.mulp(a, &k.mulp(d, d))), &s(3, &k.mulp(c, c))),
            &da,
        ),
    );
    let e16 = s(512, &k.mulp(&k.mulp(&da, &da), &cube(k, d)));
    let e22 = s(12, &k.subp(&s(2, &k.mulp(b, &k.mulp(c, c))), &k.mulp(a, d)));
    let e24 = s(
        12,
        &k.mulp(
            &k.addp(&s(16, &k.mulp(&k.mulp(b, b), c)), &s(3, &k.mulp(a, a))),
            &dc,
        ),
    );
    let e26 = s(512, &k.mulp(&k.mulp(&dc, &dc), &cube(k, b)));
    (
        k.pmul(&h1, &h2),
        [h1, h2],
        [n1, n2],
        [v1, v2],
        [[e12, e14, e16], [e22, e24, e26]],
    )
}
pub fn short(k: &Field, e: &[BigUint; 3], t: &BigUint) -> (BigUint, BigUint) {
    let a = k.subp(&e[1], &k.divp(&k.mulp(&e[0], &e[0]), &val(k, 3)));
    let b = k.addp(
        &k.subp(&e[2], &k.divp(&k.mulp(&e[0], &e[1]), &val(k, 3))),
        &k.divp(&k.mulp(&val(k, 2), &cube(k, &e[0])), &val(k, 27)),
    );
    (k.mulp(&k.mulp(t, t), &a), k.mulp(&cube(k, t), &b))
}
pub fn replay(
    k: &Field,
    f: &[BigUint],
    h: &[BigUint],
    n: &[BigUint],
    v: &[BigUint],
    e: &[BigUint; 3],
    t: &BigUint,
    u: &BigUint,
    a: &BigUint,
    b: &BigUint,
) -> Value {
    let u2 = k.mulp(u, u);
    let u3 = k.mulp(&u2, u);
    let xs = k.mulp(&u2, t);
    let ys = k.mulp(&u3, &k.mulp(t, t));
    let xn = scale(k, &add(k, n, &scale(k, h, &k.divp(&e[0], &val(k, 3)))), &xs);
    let yn = scale(k, v, &ys);
    let lhs = scale(k, &k.pmul(f, &power(k, &yn, 2)), &k.invp(t));
    let rhs = add(
        k,
        &add(
            k,
            &k.pmul(&power(k, &xn, 3), h),
            &scale(k, &k.pmul(&xn, &power(k, h, 3)), a),
        ),
        &scale(k, &power(k, h, 4), b),
    );
    assert_eq!(lhs, rhs, "cleared map identity failed");
    assert_eq!(
        k.pgcd(&xn, h).len(),
        1,
        "x-map numerator and denominator share a factor"
    );
    assert_eq!(xn.len().max(h.len()) - 1, 3, "x-map has wrong degree");
    let dx = k.psub(
        &k.pmul(&derivative(k, &xn), h),
        &k.pmul(&xn, &derivative(k, h)),
    );
    assert!(!dx.is_empty(), "inseparable map");
    let false_x = add(k, &xn, h);
    let false_rhs = add(
        k,
        &add(
            k,
            &k.pmul(&power(k, &false_x, 3), h),
            &scale(k, &k.pmul(&false_x, &power(k, h, 3)), a),
        ),
        &scale(k, &power(k, h, 4), b),
    );
    assert_ne!(lhs, false_rhs, "false shifted-x map was not rejected");
    json!({"degree":3,"separable":true,"target_a":a.to_string(),"target_b":b.to_string(),
      "x_numerator":strings(&xn),"x_denominator":strings(h),"y_factor_numerator":strings(&yn),"y_factor_denominator":strings(&power(k,h,2)),
      "cleared_identity_lhs":strings(&lhs),"cleared_identity_rhs":strings(&rhs),"cleared_identity_verified":true,"false_shifted_x_rejected":true})
}
pub fn input(src: &[Source], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    for s in src
        .iter()
        .filter(|s| s.residue_trace(3) == 0 && (&s.p % 3u32).is_one())
    {
        let mut gp = include_str!("geometry.gp").to_string();
        gp.push_str(&format!(
            "\ntwist_construct({},{},{},{},\"{}\");\nprint(\"ALL_GEOMETRY_SEARCH_DONE\");quit;\n",
            s.p, s.a, s.b, s.d, s.name
        ));
        let file = format!("{}.gp", s.name);
        fs::write(out.join(&file), gp)?;
        rows.push(json!({"source":s.model(),"script":file,"status":"constructor-input-prepared-not-executed"}));
    }
    Ok(rows)
}
pub fn verify(src: &[Source], stdout: &Path, out: &Path) -> Result<Vec<Value>> {
    let data = fs::read_to_string(stdout)?;
    let mut rows = vec![];
    for (line_no, line) in data.lines().enumerate() {
        let Some(line) = line.strip_prefix("CANDIDATE ") else {
            continue;
        };
        let v: Value = serde_json::from_str(line)?;
        let s = src
            .iter()
            .find(|s| v["source"] == s.name)
            .ok_or("unknown source in geometry candidate")?;
        let number = |key: &str| -> Result<BigUint> {
            Ok(v[key].as_str().ok_or("missing parameter")?.parse()?)
        };
        let (a, b, c, d, t) = (
            number("a")?,
            number("b")?,
            number("c")?,
            number("d")?,
            number("t")?,
        );
        let us = [number("u1")?, number("u2")?];
        let k = Field::new(s.p.clone(), s.d.clone());
        let (f, hs, ns, vs, es) = family(&k, &a, &b, &c, &d);
        assert!((5..=6).contains(&(f.len() - 1)));
        assert_eq!(k.pgcd(&f, &derivative(&k, &f)).len(), 1);
        let mut maps = vec![];
        for i in 0..2 {
            let (aa, bb) = short(&k, &es[i], &t);
            let u2 = k.mulp(&us[i], &us[i]);
            let ta = if i == 0 {
                s.a.clone()
            } else {
                k.mulp(&k.mulp(&s.d, &s.d), &s.a)
            };
            let tb = if i == 0 {
                s.b.clone()
            } else {
                k.mulp(&cube(&k, &s.d), &s.b)
            };
            assert_eq!(k.mulp(&k.mulp(&u2, &u2), &aa), ta);
            assert_eq!(k.mulp(&cube(&k, &u2), &bb), tb);
            maps.push(replay(
                &k, &f, &hs[i], &ns[i], &vs[i], &es[i], &t, &us[i], &ta, &tb,
            ));
        }
        let mut point_count = None;
        if let Some(p) = s.p.to_u64() {
            if p <= 16384 {
                let f = scale(&k, &f, &k.invp(&t));
                let mut n = 0u64;
                for x in 0..p {
                    let x = BigUint::from(x);
                    let y2 = f
                        .iter()
                        .rev()
                        .fold(BigUint::zero(), |r, c| k.addp(&k.mulp(&r, &x), c));
                    n += if y2.is_zero() {
                        1
                    } else if k.is_square(&y2) {
                        2
                    } else {
                        0
                    };
                }
                n += if f.len() == 6 {
                    1
                } else if k.is_square(f.last().unwrap()) {
                    2
                } else {
                    0
                };
                assert_eq!(n, p + 1, "curve count does not match E x twist prediction");
                point_count = Some(n);
            }
        }
        let row = json!({"experiment":"E19","source":s.model(),"constructor_parameters":v,"input_line":line_no+1,
          "curve":{"form":"t*y^2=f(x)","f":strings(&f),"t":t.to_string(),"genus":2,"squarefree":true,"infinity":if f.len()==6{"one_rational"}else{"two_over_algebraic_closure"}},
          "maps":maps,"exact_toy_Fp_point_count":point_count,"point_count_expected_from_factor_traces":"p+1",
          "degree_coprime_to_source_order":(&s.n%3u32)!=BigUint::zero(),
          "status":"explicit-curve-and-two-degree-three-maps-verified-native",
          "divisor_pullback_executed":false,"arbitrary_class_norm_executed":false,"polarized_graph_kernel_independently_identified":false,
          "formal_source_map_bridge_complete":false,"full_E19_complete":false,"performance_claim":false});
        write_json(out.join(format!("{}-{:03}.json", s.name, rows.len())), &row)?;
        rows.push(row);
    }
    if rows.is_empty() {
        return Err("no verified geometry candidates in supplied stdout".into());
    }
    Ok(rows)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_published_maps_replay_and_shifted_maps_fail() {
        let k = Field::new(7u32.into(), 3u32.into());
        let (a, b, c, d) = (1u32.into(), 1u32.into(), 1u32.into(), 5u32.into());
        let (f, hs, ns, vs, es) = family(&k, &a, &b, &c, &d);
        for i in 0..2 {
            let (aa, bb) = short(&k, &es[i], &1u32.into());
            let r = replay(
                &k,
                &f,
                &hs[i],
                &ns[i],
                &vs[i],
                &es[i],
                &1u32.into(),
                &1u32.into(),
                &aa,
                &bb,
            );
            assert_eq!(r["degree"], 3);
        }
    }
}
