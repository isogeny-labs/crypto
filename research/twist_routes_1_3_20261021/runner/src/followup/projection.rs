//! Finite replay of the eigenspace and subgroup projections, with even controls.
use crate::{digest, toy_field::*, write_json, Fixture, Result};
use serde_json::{json, Value};
use std::{fs, path::Path};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn qp(e: Quad, mut x: Q, mut n: u64) -> Q {
    let mut y = Q(1, 0);
    while n != 0 {
        if n & 1 != 0 {
            y = e.mul(y, x);
        }
        x = e.mul(x, x);
        n >>= 1;
    }
    y
}
fn embed(e: Quad, point: P) -> P {
    point.map(|(x, y)| {
        assert_eq!((x.1, y.1), (0, 0));
        (
            Q(x.0 * inv(e.d, e.p) % e.p, 0),
            Q(0, y.0 * inv(e.d * e.d % e.p, e.p) % e.p),
        )
    })
}
pub fn replay(fixtures: &[Fixture], out: &Path) -> Result<Vec<Value>> {
    let mut rows = vec![];
    let mut gp = String::from("default(parisizemax,4000000000);\n");
    gp.push_str("twist_proj_embed(P,d,z)={if(P==[0],P,[P[1]/d,P[2]*z/d^2])};\n");
    gp.push_str("twist_proj_frob(P,p)={if(P==[0],P,[P[1]^p,P[2]^p])};\n");
    gp.push_str("twist_proj_check(p,a,b,d,n,nt)={my(z=ffgen(Mod(1,p)*(x^2-d),'z),E=ellinit([a,b],z),D=ellinit([Mod(d^2*a,p),Mod(d^3*b,p)]),bs=List());listput(bs,[0]);for(x=0,p-1,my(ys=ellordinate(D,Mod(x,p)));for(j=1,#ys,listput(bs,[Mod(x,p),ys[j]])));if(#bs!=nt,error(\"twist order mismatch\"));for(i=1,#bs,my(B=twist_proj_embed(bs[i],d,z));if(!ellisoncurve(E,B)||twist_proj_frob(B,p)!=ellneg(E,B),error(\"twist embedding or Frobenius\"));for(j=1,#bs,if(twist_proj_embed(elladd(D,bs[i],bs[j]),d,z)!=elladd(E,B,twist_proj_embed(bs[j],d,z)),error(\"embedding group law\"))));};\n");
    for f in fixtures.iter().filter(|f| f.p == 59 || !prime(f.order)) {
        let e = f.curve();
        let nt = 2 * (f.p + 1) - f.order;
        let et = Quad::new(
            f.p,
            f.a * f.d % f.p * f.d % f.p,
            f.b * f.d % f.p * f.d % f.p * f.d % f.p,
        );
        let rational = e.rational();
        let twist = et.rational();
        let all = e.points();
        assert_eq!(rational.len() as u64, f.order);
        assert_eq!(twist.len() as u64, nt);
        assert_eq!(all.len() as u64, f.order * nt);
        let mut frobenius_checks = 0u64;
        for &p in &all {
            let actual = p.map(|(x, y)| (qp(e, x, e.p), qp(e, y, e.p)));
            assert_eq!(e.frob(p), actual);
            assert_eq!(e.frob(e.frob(p)), p);
            frobenius_checks += 1;
        }
        for &a in &rational {
            assert_eq!(e.frob(a), a);
            assert_eq!(e.trace(a), e.pa(a, a));
        }
        let mut hom = 0u64;
        let mut mixed = 0u64;
        let mut admissible = 0u64;
        for &b in &twist {
            let eb = embed(e, b);
            assert!(e.on(eb));
            assert_eq!(e.frob(eb), e.pn(eb));
            assert_eq!(e.trace(eb), None);
            for &c in &twist {
                assert_eq!(embed(e, et.pa(b, c)), e.pa(eb, embed(e, c)));
                hom += 1;
            }
            for &a in &rational {
                let r = e.pa(a, eb);
                if e.frob(r) == r {
                    assert_eq!(e.pa(eb, eb), None);
                    admissible += 1;
                }
                mixed += 1;
            }
        }
        let coprime = gcd(f.order, nt) == 1;
        let mut projector_checks = 0u64;
        if f.order % 2 == 1 && coprime {
            // Euler inversion is not valid for the composite-order control.
            let nt_inv = (1..f.order).find(|x| nt * x % f.order == 1).unwrap();
            let half = (f.order + 1) / 2;
            for &p in &all {
                let pi = e.pm(p, (nt * nt_inv) as u128);
                assert_eq!(pi, e.pm(e.trace(p), half as u128));
                assert_eq!(e.pm(pi, f.order as u128), None);
                assert_eq!(e.frob(pi), pi);
                projector_checks += 1;
            }
        }
        let even_witness = rational
            .iter()
            .copied()
            .find(|p| p.is_some() && e.pa(*p, *p).is_none());
        if let Some(b) = even_witness {
            assert_eq!(e.frob(b), b);
            assert_eq!(e.frob(b), e.pn(b));
            // R=B, A=O is a nonzero common two-torsion mixed relation.
            assert_eq!(e.pa(None, b), b);
        }
        let nonzero_twist = twist
            .iter()
            .copied()
            .map(|p| embed(e, p))
            .find(|p| p.is_some() && e.pa(*p, *p).is_some())
            .unwrap();
        assert_ne!(
            e.frob(nonzero_twist),
            nonzero_twist,
            "wrong sign control did not fail"
        );
        let nonzero_rational = rational
            .iter()
            .copied()
            .find(|p| p.is_some() && e.pa(*p, *p).is_some())
            .unwrap();
        assert_ne!(
            e.trace(nonzero_rational),
            nonzero_rational,
            "missing factor two control did not fail"
        );
        let row = json!({"experiment":"E29","fixture":f.id,"p":f.p,"order":f.order,"twist_order":nt,
          "extension_points":all.len(),"actual_power_Frobenius_checks":frobenius_checks,"twist_embedding_addition_pairs":hom,
          "mixed_pairs_checked":mixed,"rational_mixed_pairs":admissible,"coprime_curve_twist_orders":coprime,
          "odd_component_projector_checks":projector_checks,"common_nonzero_two_torsion_witness":even_witness,
          "wrong_Frobenius_sign_rejected":true,"missing_trace_factor_two_rejected":true,
          "noninvertible_CRT_denominator_typed":!coprime,"coordinate_digest":digest(&serde_json::to_vec(&all)?),
          "status":"finite-coordinate-group-and-projection-replay","formal_concrete_field_bridge_complete":false,
          "full_E29_complete":false,"performance_claim":false});
        write_json(out.join(format!("{}.json", f.id)), &row)?;
        rows.push(row);
        gp.push_str(&format!(
            "twist_proj_check({},{},{},{},{},{});\n",
            f.p, f.a, f.b, f.d, f.order, nt
        ));
    }
    gp.push_str("print(\"ALL_PROJECTION_REPLAY_OK\");quit;\n");
    fs::write(out.join("verify.gp"), gp)?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn twist_embedding_uses_the_standard_model_and_actual_power_frobenius() {
        let e = Quad::new(7, 1, 1);
        let et = Quad::new(7, 2, 6);
        for p in et.rational() {
            let q = embed(e, p);
            assert!(e.on(q));
            assert_eq!(e.frob(q), e.pn(q));
            if let Some((x, y)) = q {
                assert_eq!(qp(e, x, 7), x);
                assert_eq!(qp(e, y, 7), e.neg(y));
            }
        }
    }
}
