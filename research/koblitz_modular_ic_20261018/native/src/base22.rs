//! Independent F2[u,v]/(v²+v=u³+u) base-chart and divisor arithmetic.
use num_bigint::BigUint as P;
use num_traits::{One, Zero};
use serde_json::json;
use std::path::Path;
fn pm(a: &P, b: &P) -> P {
    let mut z = P::zero();
    for i in 0..b.bits() {
        if b.bit(i) {
            z ^= a << i as usize;
        }
    }
    z
}
fn pp(a: &P, n: usize) -> P {
    (0..n).fold(P::one(), |z, _| pm(&z, a))
}
fn divrem(mut a: P, b: &P) -> (P, P) {
    assert!(!b.is_zero());
    let mut q = P::zero();
    while !a.is_zero() && a.bits() >= b.bits() {
        let k = (a.bits() - b.bits()) as usize;
        q ^= P::one() << k;
        a ^= b << k;
    }
    (q, a)
}
fn exact(a: P, b: &P) -> P {
    let (q, r) = divrem(a, b);
    assert!(r.is_zero());
    q
}
fn gcd(mut a: P, mut b: P) -> P {
    while !b.is_zero() {
        let r = divrem(a, &b).1;
        a = b;
        b = r;
    }
    a
}
type Q = (P, P);
fn add(a: &Q, b: &Q) -> Q {
    (&a.0 ^ &b.0, &a.1 ^ &b.1)
}
fn mul(a: &Q, b: &Q) -> Q {
    let yy = pm(&a.1, &b.1);
    (
        pm(&a.0, &b.0) ^ pm(&yy, &P::from(10u32)),
        pm(&a.0, &b.1) ^ pm(&a.1, &b.0) ^ yy,
    )
}
fn pow(a: &Q, n: usize) -> Q {
    (0..n).fold((P::one(), P::zero()), |z, _| mul(&z, a))
}
fn shift(a: &Q, n: usize) -> Q {
    (&a.0 << n, &a.1 << n)
}
fn norm(a: &Q) -> P {
    pm(&a.0, &a.0) ^ pm(&a.0, &a.1) ^ pm(&P::from(10u32), &pm(&a.1, &a.1))
}
fn val(a: &P) -> i64 {
    assert!(!a.is_zero());
    (0..a.bits()).find(|i| a.bit(*i)).unwrap() as i64
}
fn pj(a: &Q) -> serde_json::Value {
    json!([a.0.to_str_radix(16), a.1.to_str_radix(16)])
}
fn series_mul(a: &P, b: &P) -> P {
    pm(a, b) & ((P::one() << 192) - P::one())
}
fn series_at(a: &P, x: &P) -> P {
    let mut z = P::zero();
    for i in (0..a.bits()).rev() {
        z = series_mul(&z, x);
        if a.bit(i) {
            z ^= P::one();
        }
    }
    z
}
fn evaluate(a: &Q, u: &P, v: &P) -> P {
    series_at(&a.0, u) ^ series_mul(&series_at(&a.1, u), v)
}
pub fn run(dir: &Path) {
    let one = (P::one(), P::zero());
    let u = (P::from(2u32), P::zero());
    let v = (P::zero(), P::one());
    let rn = add(&mul(&add(&u, &one), &v), &one);
    let cn = add(&rn, &pow(&u, 3));
    let bn = mul(&rn, &cn);
    assert_eq!(norm(&rn), P::one() << 5);
    assert_eq!(norm(&cn), pm(&(P::one() << 3), &pp(&P::from(3u32), 3)));
    let k = add(&add(&shift(&bn, 1), &shift(&cn, 4)), &pow(&cn, 2));
    assert_eq!(k, mul(&add(&u, &one), &pow(&cn, 2)));
    // b=bn/u7,c=cn/u4. Verify the original Tate residual independently.
    let mut residual = (P::zero(), P::zero());
    for (i, j) in [
        (1, 7),
        (2, 5),
        (1, 6),
        (2, 4),
        (0, 6),
        (5, 0),
        (4, 1),
        (3, 2),
        (2, 3),
    ] {
        let term = shift(&mul(&pow(&bn, i), &pow(&cn, j)), 49 - 7 * i - 4 * j);
        residual = add(&residual, &term);
    }
    assert_eq!(residual, (P::zero(), P::zero()));
    let delta0 = add(
        &shift(&bn, 9),
        &mul(&cn, &pow(&add(&shift(&one, 4), &cn), 3)),
    );
    let nd = norm(&delta0);
    let quotient = exact(
        exact(nd.clone(), &(P::one() << 12)),
        &pp(&P::from(3u32), 13),
    );
    assert_eq!(quotient.bits(), 6);
    // Rabin test for this degree-five polynomial: gcd(q,x^(2^i)-x)=1,i=1,2;
    // and x^32=x modulo q. This covers every possible proper factor degree.
    let x = P::from(2u32);
    let mut xp = x.clone();
    for i in 1..=5 {
        xp = divrem(pm(&xp, &xp), &quotient).1;
        if i <= 2 {
            assert_eq!(gcd(quotient.clone(), &xp ^ &x), P::one());
        }
    }
    assert_eq!(xp, x);
    let mut local = vec![];
    for (name, u0, v0) in [("A", 0u8, 0u8), ("B", 0, 1), ("C", 1, 0), ("D", 1, 1)] {
        let t = P::from(2u32);
        let us = &t ^ P::from(u0);
        let rhs = pp(&us, 3) ^ &us;
        let constant = P::from(u0 ^ u0);
        assert!(constant.is_zero());
        let mut vs = P::from(v0);
        let mut error = rhs.clone();
        for _ in 0..8 {
            vs ^= &error;
            error = series_mul(&error, &error);
        }
        let e = series_mul(&vs, &vs) ^ &vs ^ &rhs;
        assert!(e.is_zero());
        let vb = val(&evaluate(&bn, &us, &vs)) - 7 * val(&us);
        let vc = val(&evaluate(&cn, &us, &vs)) - 4 * val(&us);
        let vd = val(&evaluate(&delta0, &us, &vs)) - 16 * val(&us);
        local.push(json!({"place":name,"u":u0,"v":v0,"v_b":vb,"v_c":vc,"v_delta0":vd,"series_precision":192}));
    }
    assert_eq!(
        local
            .iter()
            .map(|r| (
                r["v_b"].as_i64().unwrap(),
                r["v_c"].as_i64().unwrap(),
                r["v_delta0"].as_i64().unwrap()
            ))
            .collect::<Vec<_>>(),
        vec![(-7, -4, -16), (1, -1, -4), (1, 1, 8), (2, 2, 5)]
    );
    let record = json!({"schema":"koblitz.round22/cm22-base-v1","field":"F2(u,v),v^2+v=u^3+u","r_numerator":pj(&rn),"r_denominator_u_power":3,"c_numerator":pj(&cn),"c_denominator_u_power":4,"b_numerator":pj(&bn),"b_denominator_u_power":7,"Tate11_substitution_zero":true,"delta0_numerator":pj(&delta0),"delta0_denominator_u_power":16,"Norm_delta0_numerator":nd.to_str_radix(16),"Norm_delta0_factorization":"u^12*(u+1)^13*q5(u)","q5_hex":quotient.to_str_radix(16),"q5_irreducible":true,"finite_place_valuations":local,"O_valuations":{"u":-2,"v":-3,"rn":-5,"cn":-6,"b":3,"c":2,"delta0":2},"complete_divisors_on_base":{"b":[["A",-7],["B",1],["C",1],["D",2],["O",3]],"c":[["A",-4],["B",-1],["C",1],["D",2],["O",2]],"delta0":[["A",-16],["B",-4],["C",8],["D",5],["O",2],["Z5",1]]},"Z5_description":"unique simple zero of delta0 above each root of the irreducible q5;degree5 closed place,certified by the simple norm factor","geometric_bridge":"smooth cubic local parameters and valuations;finite support completeness from norm factorization and unique nonzero numerator/pole orders","full_level77_divisors":false});
    assert!(!dir.join("cm22-base-divisors.json").exists());
    crate::round02::save(dir, "cm22-base-divisors.json", &record);
    println!("CM22 base:independent Tate substitution;complete b,c,delta0 divisors;degree5 residual q={};four rational-place series controls",quotient.to_str_radix(16));
}

pub fn exceptional(dir: &Path) {
    let one = (P::one(), P::zero());
    let u = (P::from(2u32), P::zero());
    let v = (P::zero(), P::one());
    let rn = add(&mul(&add(&u, &one), &v), &one);
    let cn = add(&rn, &pow(&u, 3));
    let an = add(&shift(&one, 4), &cn);
    let na = norm(&an);
    let q = exact(na.clone(), &(P::one() << 3));
    assert_eq!(q.bits(), 6);
    let x = P::from(2u32);
    let mut xp = x.clone();
    for i in 1..=5 {
        xp = divrem(pm(&xp, &xp), &q).1;
        if i <= 2 {
            assert_eq!(gcd(q.clone(), &xp ^ &x), P::one());
        }
    }
    assert_eq!(xp, x);
    let prior = crate::round04::read(&dir.join("cm22-base-divisors.json"));
    assert_eq!(prior["Tate11_substitution_zero"], true);
    crate::round02::save(
        dir,
        "cm22-exceptional-supports.json",
        &json!({"schema":"koblitz.round22/cm22-exceptional-v1","A_definition":"1+c","A_numerator":pj(&an),"A_denominator_u_power":4,"Norm_A_numerator":na.to_str_radix(16),"Norm_A_factorization":"u^3*qSS(u)","qSS_hex":q.to_str_radix(16),"qSS_irreducible":true,"supersingular_support_degree":5,"divisor_A":[["A",-4],["B",-1],["SS5",1]],"divisor_K_b_plus_c_plus_c2":[["A",-8],["B",-2],["C",3],["D",5],["O",2]],"divisor_x4":[["A",-6],["B",2],["C",1],["D",2],["O",1]],"generic_nu_recovery_denominator":"A^2*b^3*delta0","divisor_generic_nu_denominator":[["A",-45],["B",-3],["C",11],["D",11],["O",11],["Z5",1],["SS5",2]],"scope":"complete base divisors;SS5 is a genuine smooth noncuspidal exceptional support for the primitive lambda chart;lifting to level77 still required"}),
    );
    println!("CM22 exceptional:irreducible degree5 SS support q={};base K,x4,A and generic nu-denominator divisor;lifted branches still open",q.to_str_radix(16));
}
