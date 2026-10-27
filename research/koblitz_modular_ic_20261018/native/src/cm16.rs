//! Integral, not merely rational, Frobenius sign separation on the level77 factor.
use crate::{cmray15::an, exact16 as e, round04::read};
use e::{P, Q};
use num_bigint::BigInt as Z;
use num_integer::Integer;
use num_traits::{One, Signed, ToPrimitive, Zero};
use serde_json::json;
use std::path::Path;
fn q(p: &[Z]) -> P {
    p.iter().map(|n| Q::new(n.clone(), Z::one())).collect()
}
fn modulus(a: &[Z], p: i64) -> Vec<i64> {
    a.iter()
        .map(|c| c.mod_floor(&Z::from(p)).to_i64().unwrap())
        .collect()
}
fn trim(p: &mut Vec<i64>) {
    while p.last() == Some(&0) {
        p.pop();
    }
}
fn gcd(mut a: Vec<i64>, mut b: Vec<i64>, p: i64) -> Vec<i64> {
    trim(&mut a);
    trim(&mut b);
    while !b.is_empty() {
        let inv = (1..p).find(|k| k * b.last().unwrap() % p == 1).unwrap();
        while a.len() >= b.len() {
            let k = a.len() - b.len();
            let c = a.last().unwrap() * inv % p;
            for j in 0..b.len() {
                a[k + j] = (a[k + j] - c * b[j]).rem_euclid(p);
            }
            trim(&mut a);
        }
        let r = a;
        a = b;
        b = r;
    }
    if !a.is_empty() {
        let inv = (1..p).find(|k| k * a.last().unwrap() % p == 1).unwrap();
        for x in &mut a {
            *x = *x * inv % p;
        }
    }
    a
}
fn sylvester(a: &[Z], b: &[Z]) -> Vec<Vec<Z>> {
    let (m, n) = (a.len() - 1, b.len() - 1);
    let mut s = vec![vec![Z::zero(); m + n]; m + n];
    for i in 0..n {
        for j in 0..=m {
            s[i][i + j] = a[m - j].clone();
        }
    }
    for i in 0..m {
        for j in 0..=n {
            s[n + i][i + j] = b[n - j].clone();
        }
    }
    s
}
fn expression(p: &[Z], ty: &str) -> String {
    p.iter()
        .enumerate()
        .filter(|(_, c)| !c.is_zero())
        .map(|(i, c)| format!("({c}:{ty})*x^{i}"))
        .collect::<Vec<_>>()
        .join(" + ")
}
pub fn run(dir: &Path) {
    let (a, b) = (an(5, -1), an(5, 1));
    let s = sylvester(&a, &b);
    let resultant = e::det(s.clone());
    assert!(!resultant.is_zero());
    assert!(e::det(sylvester(&a, &a)).is_zero());
    let (mut r0, mut r1) = (q(&a), q(&b));
    let (mut s0, mut s1) = (vec![Q::z(1)], vec![]);
    let (mut t0, mut t1) = (vec![], vec![Q::z(1)]);
    let mut steps = 0;
    while !r1.is_empty() {
        let (d, r) = e::div(&r0, &r1);
        let ss = e::add(&s0, &e::neg(&e::mul(&d, &s1)));
        let tt = e::add(&t0, &e::neg(&e::mul(&d, &t1)));
        r0 = r1;
        r1 = r;
        s0 = s1;
        s1 = ss;
        t0 = t1;
        t1 = tt;
        steps += 1;
        assert!(steps <= 20);
    }
    assert_eq!(r0.len(), 1);
    for x in s0.iter_mut().chain(t0.iter_mut()) {
        *x = x.div(&r0[0]);
    }
    let den = s0
        .iter()
        .chain(t0.iter())
        .fold(Z::one(), |d, c| d.lcm(&c.d));
    let aa = s0.iter().map(|c| &c.n * (&den / &c.d)).collect::<Vec<_>>();
    let bb = t0.iter().map(|c| &c.n * (&den / &c.d)).collect::<Vec<_>>();
    assert_eq!(
        e::add(&e::mul(&q(&aa), &q(&a)), &e::mul(&q(&bb), &q(&b))),
        vec![Q::new(den.clone(), Z::one())]
    );
    assert!((&resultant % &den).is_zero());
    let mut v = resultant.abs();
    let mut v11 = 0;
    while (&v % 11u32).is_zero() {
        v /= 11u32;
        v11 += 1;
    }
    let controls = [3, 5, 7, 11, 13, 19, 23, 29, 31]
        .into_iter()
        .map(|p| json!({"prime":p,"monic_gcd":gcd(modulus(&a,p),modulus(&b,p),p)}))
        .collect::<Vec<_>>();
    let ev_a = a.iter().sum::<Z>();
    let ev_b = b.iter().sum::<Z>();
    assert_eq!(ev_a, Z::from(11));
    assert_eq!(ev_b, Z::from(11));
    let prod = e::mul(&q(&a), &q(&b))
        .iter()
        .map(|c| c.integer().to_string())
        .collect::<Vec<_>>();
    let prior = read(&dir.join("cm15-one-prime-carriers.json"));
    assert_eq!(
        json!(prod),
        prior["winners"][0]["exact_Frobenius_polynomial_X_ascending"]
    );
    crate::round02::save(
        dir,
        "cm16-integral-sign-separation.json",
        &json!({"schema":"koblitz.round16/cm16-v1","n":5,"level":77,"subgroup_prime":11,"minus":a.iter().map(ToString::to_string).collect::<Vec<_>>(),"plus":b.iter().map(ToString::to_string).collect::<Vec<_>>(),"Sylvester":s.iter().map(|r|r.iter().map(ToString::to_string).collect::<Vec<_>>()).collect::<Vec<_>>(),"resultant":resultant.to_string(),"resultant_v11":v11,"primitive_Bezout_denominator":den.to_string(),"Bezout_minus":aa.iter().map(ToString::to_string).collect::<Vec<_>>(),"Bezout_plus":bb.iter().map(ToString::to_string).collect::<Vec<_>>(),"modular_gcd_controls":controls,"both_at_one":[ev_a.to_string(),ev_b.to_string()],"Frobenius_projector_integral_at11":false,"arbitrary_subgroup_map_excluded":false,"scope":"obstruction only to integral Frobenius-polynomial sign projectors;explicit maps and alternative correspondences remain open"}),
    );
    let mut lean=String::from("import KoblitzRound15CM\nimport Mathlib.Algebra.Polynomial.Eval.Defs\nnamespace KoblitzRound16CM\ninstance : Fact (Nat.Prime 11) := ⟨by decide⟩\n");
    lean.push_str(&format!("theorem actual_integer_Bezout {{R:Type*}} [CommRing R] (x:R) : ({})*({})+({})*({})=({}:R) := by ring\n",expression(&aa,"R"),expression(&a,"R"),expression(&bb,"R"),expression(&b,"R"),den));
    lean.push_str(&format!("noncomputable def pminus : Polynomial (ZMod 11) := let x := Polynomial.X; {}\nnoncomputable def pplus : Polynomial (ZMod 11) := let x := Polynomial.X; {}\ntheorem actual_common_root : Polynomial.eval 1 pminus=0 ∧ Polynomial.eval 1 pplus=0  := by\n constructor <;> norm_num [pminus,pplus] <;> decide +kernel\ntheorem no_integral_projector (A B:Polynomial (ZMod 11)) : A*pminus+B*pplus ≠ 1 := by\n intro h\n have hh := congrArg (Polynomial.eval 1) h\n simp only [Polynomial.eval_add,Polynomial.eval_mul,actual_common_root.1,actual_common_root.2,mul_zero,add_zero,Polynomial.eval_one] at hh\n have hn : (0:ZMod 11) ≠ 1 := by decide +kernel\n exact hn hh\n",expression(&a,"Polynomial (ZMod 11)"),expression(&b,"Polynomial (ZMod 11)")));
    lean.push_str("end KoblitzRound16CM\n#print axioms KoblitzRound16CM.actual_integer_Bezout\n#print axioms KoblitzRound16CM.actual_common_root\n#print axioms KoblitzRound16CM.no_integral_projector\n");
    assert!(!dir.join("KoblitzRound16CM.lean").exists());
    std::fs::write(dir.join("KoblitzRound16CM.lean"), lean).unwrap();
    println!("CM16:resultant{resultant},v11={v11},primitive Bezout denominator{den};both sign polynomials at1=11;no integral Frobenius projector at11");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn modular_gcd_has_real_euclidean_remainder_controls() {
        assert_eq!(gcd(vec![10, 0, 1], vec![10, 1], 11), vec![10, 1]);
        assert_eq!(
            gcd(modulus(&an(5, -1), 3), modulus(&an(5, 1), 3), 3),
            vec![1]
        );
        let g = gcd(modulus(&an(5, -1), 11), modulus(&an(5, 1), 11), 11);
        assert_eq!(g.len(), 7);
        assert_eq!(g.iter().sum::<i64>() % 11, 0);
    }
}
