use super::{
    bigfield::{Field, Point},
    Source,
};
use crate::{write_json, Result};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Value};
use std::{collections::BTreeSet, fs, path::Path};

type Mat = [u64; 4];
fn mm(a: Mat, b: Mat, l: u64) -> Mat {
    [
        (a[0] * b[0] + a[1] * b[2]) % l,
        (a[0] * b[1] + a[1] * b[3]) % l,
        (a[2] * b[0] + a[3] * b[2]) % l,
        (a[2] * b[1] + a[3] * b[3]) % l,
    ]
}
pub fn anti(a: Mat, b: Mat, l: u64) -> Vec<Mat> {
    let mut out = vec![];
    for m0 in 0..l {
        for m1 in 0..l {
            for m2 in 0..l {
                for m3 in 0..l {
                    let m = [m0, m1, m2, m3];
                    if (m0 * m3 + l * l - m1 * m2) % l == l - 1 && mm(m, a, l) == mm(b, m, l) {
                        out.push(m);
                    }
                }
            }
        }
    }
    out
}
pub fn filters(src: &[Source], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    gp.push_str("check(p,t,l,n)={my(A=Mod([0,-p;1,t],l),m=0);for(a=0,l-1,for(b=0,l-1,for(c=0,l-1,for(d=0,l-1,my(M=Mod([a,b;c,d],l));if(lift(matdet(M))==l-1&&M*A==-A*M,m++)))));if(m!=n,error(\"module count mismatch\"));};\n");
    for s in src {
        for l in [3u64, 5, 7, 11, 13, 17] {
            let p = (&s.p % l).to_u64().unwrap();
            let t = s.residue_trace(l);
            let a = [0, (l - p) % l, 1, t];
            let b = a.map(|x| (l - x) % l);
            let excluded = p == 0 || (&s.n % l).is_zero();
            let matrices = if excluded { vec![] } else { anti(a, b, l) };
            let row = json!({"experiment":if l==3{"E20-regression-E18"}else{"E20"},"source":s.model(),"ell":l,
          "p_mod_ell":p,"trace_mod_ell":t,"excluded":excluded,"source_subgroup_coprime_to_ell":s.n.gcd(&BigUint::from(l)).is_one(),
          "companion_Frobenius":a,"twist_companion_Frobenius":b,"anti_isometry_matrices":matrices,"matrix_count":matrices.len(),
          "actual_torsion_bridge_verified":false,"quotient_or_maps_constructed":false,"status":"necessary-companion-module-filter-only"});
            if !excluded {
                gp.push_str(&format!("check({p},{t},{l},{});\n", matrices.len()));
            }
            write_json(out.join(format!("{}-l{l}.json", s.name)), &row)?;
            rows.push(row);
        }
    }
    gp.push_str("print(\"INDEPENDENT_MODULE_REPLAY_OK\");quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    Ok(rows)
}
fn coords(_k: &Field, basis: &[Point], p: &Point) -> [u64; 2] {
    let i = basis
        .iter()
        .position(|q| q == p)
        .expect("point outside constructed torsion basis");
    [i as u64 / 3, i as u64 % 3]
}
fn table(k: &Field, p: &Point, q: &Point, a: &BigUint) -> Vec<Point> {
    (0..3)
        .flat_map(|i| (0..3).map(move |j| (i, j)))
        .map(|(i, j)| k.pa(&k.pm(p, i, a), &k.pm(q, j, a), a))
        .collect()
}
fn gp_f(_k: &Field, x: &super::bigfield::F) -> String {
    x.0.iter()
        .enumerate()
        .map(|(i, c)| format!("({c})*z^{i}"))
        .collect::<Vec<_>>()
        .join("+")
}
fn gp_p(k: &Field, p: &Point) -> String {
    match p {
        None => "[0]".into(),
        Some((x, y)) => format!("[{},{}]", gp_f(k, x), gp_f(k, y)),
    }
}
fn certificate(s: &Source) -> Value {
    let k = Field::new(s.p.clone(), s.d.clone());
    let (f, g, trials) = k.division_quadratics(&s.a, &s.b);
    let mut points = vec![];
    for x in [k.quadratic_roots(&f), k.quadratic_roots(&g)]
        .into_iter()
        .flatten()
    {
        let y2 = k.rhs(&x, &s.a, &s.b);
        assert!(
            k.sqrtq(&y2).is_none(),
            "3-torsion y unexpectedly in quadratic subfield"
        );
        let beta = k.add(&k.fp(&k.eta), &k.w());
        let y = k.mul(
            &k.v(),
            &k.sqrtq(&k.div(&y2, &beta))
                .expect("nonsquare ratio must be square in Fp2"),
        );
        assert_eq!(k.mul(&y, &y), y2);
        for y in [y.clone(), k.neg(&y)] {
            let p = Some((x.clone(), y));
            assert!(k.on(&p, &s.a, &s.b));
            assert_eq!(k.pm(&p, 3, &s.a), None);
            points.push(p);
        }
    }
    points.sort();
    points.dedup();
    assert_eq!(points.len(), 8);
    let p = points[0].clone();
    let q = points
        .iter()
        .find(|q| **q != p && **q != k.pn(&p))
        .unwrap()
        .clone();
    let basis = table(&k, &p, &q, &s.a);
    assert_eq!(basis.iter().cloned().collect::<BTreeSet<_>>().len(), 9);
    let mut extension_checks = vec![];
    for degree in 1..=4 {
        let fixed = basis
            .iter()
            .filter(|p| {
                let mut q = (*p).clone();
                for _ in 0..degree {
                    q = k.frob(&q);
                }
                q == **p
            })
            .count();
        assert_eq!(fixed, if degree == 4 { 9 } else { 1 });
        extension_checks.push(json!({"degree":degree,"fixed_torsion_points_including_origin":fixed,"full_3_torsion_rational":fixed==9}));
    }
    let zeta = k.weil3(&p, &q, &s.a);
    assert_ne!(zeta, k.c(1));
    assert_eq!(k.pow(&zeta, &BigUint::from(3u32)), k.c(1));
    let ap = coords(&k, &basis, &k.frob(&p));
    let aq = coords(&k, &basis, &k.frob(&q));
    let a = [ap[0], aq[0], ap[1], aq[1]];
    let d2 = k.mulp(&s.d, &s.d);
    let ta = k.mulp(&d2, &s.a);
    let tb = k.mulp(&k.mulp(&d2, &s.d), &s.b);
    let tw = |r: &Point| {
        r.as_ref()
            .map(|(x, y)| (k.mul(&k.fp(&s.d), x), k.mul(&k.mul(&k.fp(&s.d), &k.w()), y)))
    };
    let tp = tw(&p);
    let tq = tw(&q);
    let tt = table(&k, &tp, &tq, &ta);
    assert!(tt.iter().all(|p| k.on(p, &ta, &tb)));
    let bp = coords(&k, &tt, &k.frob(&tp));
    let bq = coords(&k, &tt, &k.frob(&tq));
    let b = [bp[0], bq[0], bp[1], bq[1]];
    assert_eq!(b, a.map(|v| (3 - v) % 3));
    assert_eq!(k.weil3(&tp, &tq, &ta), zeta);
    let mats = anti(a, b, 3);
    assert_eq!(mats.len(), 4);
    let mut maps = vec![];
    for m in mats {
        let mp = k.pa(&k.pm(&tp, m[0], &ta), &k.pm(&tq, m[2], &ta), &ta);
        let mq = k.pa(&k.pm(&tp, m[1], &ta), &k.pm(&tq, m[3], &ta), &ta);
        assert_eq!(k.weil3(&mp, &mq, &ta), k.pow(&zeta, &BigUint::from(2u32)));
        for (i, r) in basis.iter().enumerate() {
            let image = k.pa(
                &k.pm(&mp, i as u64 / 3, &ta),
                &k.pm(&mq, i as u64 % 3, &ta),
                &ta,
            );
            let ij = coords(&k, &basis, &k.frob(r));
            let image_of_frob = k.pa(&k.pm(&mp, ij[0], &ta), &k.pm(&mq, ij[1], &ta), &ta);
            assert_eq!(image_of_frob, k.frob(&image));
        }
        maps.push(json!({"matrix":m,"P_image":k.point_json(&mp),"Q_image":k.point_json(&mq),"graph_points_checked":9,"Galois_equivariant":true,"reverses_Weil_pairing":true}));
    }
    let mut bilinear = 0;
    for (i, r) in basis.iter().enumerate() {
        for (j, t) in basis.iter().enumerate() {
            let exponent = ((i / 3) * (j % 3) + 9 - (i % 3) * (j / 3)) % 3;
            assert_eq!(k.weil3(r, t, &s.a), k.pow(&zeta, &BigUint::from(exponent)));
            bilinear += 1;
        }
    }
    let modulus = vec![
        k.subp(&k.mulp(&k.eta, &k.eta), &s.d),
        BigUint::zero(),
        k.subp(&BigUint::zero(), &(&k.eta * 2u32 % &k.p)),
        BigUint::zero(),
        BigUint::one(),
    ];
    let mut gp=format!("p={};d={};a={};b={};M=Mod(1,p)*({}-{}*x^2+x^4);if(!polisirreducible(M),error(\"reducible field\"));z=ffgen(M,'z);E=ellinit([a,b],z);P={};Q={};Z={};\n",s.p,s.d,s.a,s.b,modulus[0],k.eta.clone()*2u32,gp_p(&k,&p),gp_p(&k,&q),gp_f(&k,&zeta));
    gp.push_str("if(!ellisoncurve(E,P)||!ellisoncurve(E,Q)||ellmul(E,P,3)!=[0]||ellmul(E,Q,3)!=[0]||P==Q||P==ellneg(E,Q),error(\"bad basis\"));if(ellweilpairing(E,P,Q,3)!=Z||Z^3!=1||Z==1,error(\"pairing mismatch\"));\n");
    gp.push_str(&format!("A=Mod([{},{};{},{}],3);ET=ellinit([d^2*a,d^3*b],z);TP={};TQ={};if(ellweilpairing(ET,TP,TQ,3)!=Z,error(\"twist pairing mismatch\"));\n",a[0],a[1],a[2],a[3],gp_p(&k,&tp),gp_p(&k,&tq)));
    gp.push_str("twist_frob(P)={if(P==[0],P,[P[1]^p,P[2]^p])};\n");
    gp.push_str("twist_linear(E,P,Q,u,v)={elladd(E,ellmul(E,P,u),ellmul(E,Q,v))};\n");
    gp.push_str("if(twist_frob(P)!=twist_linear(E,P,Q,lift(A[1,1]),lift(A[2,1]))||twist_frob(Q)!=twist_linear(E,P,Q,lift(A[1,2]),lift(A[2,2])),error(\"Frobenius mismatch\"));if(twist_frob(TP)!=twist_linear(ET,TP,TQ,lift(-A[1,1]),lift(-A[2,1]))||twist_frob(TQ)!=twist_linear(ET,TP,TQ,lift(-A[1,2]),lift(-A[2,2])),error(\"twist Frobenius mismatch\"));\n");
    for map in &maps {
        let m: Vec<u64> = serde_json::from_value(map["matrix"].clone()).unwrap();
        gp.push_str(&format!("MP=twist_linear(ET,TP,TQ,{},{});MQ=twist_linear(ET,TP,TQ,{},{});if(ellweilpairing(ET,MP,MQ,3)!=Z^2,error(\"anti pairing\"));for(i=0,2,for(j=0,2,my(C=lift(A*Mod([i,j]~,3)));if(twist_frob(twist_linear(ET,MP,MQ,i,j))!=twist_linear(ET,MP,MQ,C[1],C[2]),error(\"graph Frobenius\"))));\n",m[0],m[2],m[1],m[3]));
    }
    gp.push_str(&format!("print(\"TORSION_REPLAY_OK {}\");\n", s.name));
    json!({"experiment":"E18","status":"actual-3-torsion-and-anti-isometries-constructed-native-verifier-pending-independent-GP",
      "source":s.model(),"field":{"characteristic":s.p.to_string(),"degree":4,"modulus":modulus.iter().map(ToString::to_string).collect::<Vec<_>>(),"flat_basis":"1,v,v^2,v^3","w":"v^2-eta","eta":k.eta.to_string(),"tower_nonsquare_conditions_verified":true,"canonical_extension_identity_pending":true},
      "division_polynomial_factors":[f.iter().map(ToString::to_string).collect::<Vec<_>>(),g.iter().map(ToString::to_string).collect::<Vec<_>>()],"factor_split_trials":trials,
      "extension_degree_checks_in_increasing_order":extension_checks,"minimum_full_torsion_degree":4,
      "P":k.point_json(&p),"Q":k.point_json(&q),"Weil_pairing":k.strings(&zeta),"nonzero_torsion_points":points.iter().map(|p|k.point_json(p)).collect::<Vec<_>>(),
      "Frobenius_matrix":a,"twist_Frobenius_matrix":b,"twist_basis":[k.point_json(&tp),k.point_json(&tq)],
      "anti_isometries":maps,"bilinearity_checks":bilinear,
      "negative_controls":{"dependent_basis_rejected":k.weil3(&p,&p,&s.a)==k.c(1),"identity_matrix_pairing_not_reversing":k.weil3(&tp,&tq,&ta)!=k.pow(&zeta,&BigUint::from(2u32)),"wrong_twist_Frobenius_sign_rejected":b!=a},
      "gluing_curve_constructed":false,"subgroup_DLP_executed":false,"independent_gp_script":gp})
}
pub fn torsion(src: &[Source], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    for s in src {
        if s.residue_trace(3) != 0 || (&s.p % 3u32) != BigUint::one() {
            continue;
        }
        let mut row = certificate(s);
        gp.push_str(row["independent_gp_script"].as_str().unwrap());
        row.as_object_mut().unwrap().remove("independent_gp_script");
        write_json(out.join(format!("{}.json", s.name)), &row)?;
        println!(
            "{}",
            json!({"experiment":"E18","source":s.name,"status":"native-certificate-constructed"})
        );
        rows.push(row);
    }
    gp.push_str("print(\"ALL_TORSION_REPLAY_OK\");quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_rejects_wrong_trace_and_checks_nonsplit_counts() {
        assert!(anti([0, 1, 1, 1], [0, 2, 2, 2], 3).is_empty());
        let a = [0, 2, 1, 0];
        let b = [0, 1, 2, 0];
        let ms = anti(a, b, 3);
        assert_eq!(ms.len(), 4);
        assert!(ms.iter().all(|m| mm(*m, a, 3) == mm(b, *m, 3)));
    }
}
