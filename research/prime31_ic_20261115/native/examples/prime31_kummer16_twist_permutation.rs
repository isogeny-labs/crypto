//! Certified rational-fibre behaviour of the x-only [16] chain on Prime31.
//! Reuses the source-pinned original Curve, Arithmetic, Point, PRIME31.
//! This is a rational-root correctness gate, NOT an ECDLP extractor or speedup.
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use serde_json::{json, Value};
use std::time::Instant;

const P: u64 = 2_147_483_647;
const N: u64 = 2_147_446_831;
const TWIST_N: u64 = 2_147_520_465;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const NONSQUARE: u64 = 3;
const INV16_E: u64 = 134_215_427;
const INV16_T: u64 = 2_013_300_436;
const SEED: u64 = 0x4b55_4d4d_4552_3136;
const SAMPLES: usize = 512;

fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn add(a: u64, b: u64) -> u64 {
    let c = a + b;
    if c >= P { c - P } else { c }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { a + P - b }
}
fn pow(mut b: u64, mut e: u64) -> u64 {
    let mut z = 1;
    while e > 0 {
        if (e & 1) != 0 { z = mul(z, b); }
        b = mul(b, b);
        e >>= 1;
    }
    z
}
fn rhs(x: u64) -> u64 {
    add(add(mul(mul(x,x), x), mul(A,x)), B)
}
fn quadratic_character(x: u64) -> i8 {
    let r = rhs(x);
    assert_ne!(r,0,"no rational two-torsion on the original odd-order E");
    match pow(r, (P-1)/2) {
        1 => 1,
        v if v == P-1 => -1,
        _ => panic!("quadratic character outside +/-1"),
    }
}
fn x_double(x: u64) -> (u64, bool) {
    let ff = rhs(x);
    assert_ne!(ff,0,"no x-only doubling pole at rational x");
    let x2 = mul(x,x);
    let x4 = mul(x2,x2);
    let a2 = mul(A,A);
    let numerator = add(sub(sub(x4, mul(2, mul(A,x2))), mul(8,mul(B,x))),a2);
    let denom = mul(4, ff);
    let xx = mul(numerator,pow(denom,P-2));
    // Exact identity 64*f(x)^3*f(D(x))=M(x)^2, so chi is preserved.
    let left = sub(mul(4,mul(x,ff)),numerator);
    let m = sub(mul(add(mul(3,x2),A),left),mul(8,mul(ff,ff)));
    let ff_cubed=mul(ff,mul(ff,ff));
    let identity=mul(mul(64,ff_cubed),rhs(xx)) == mul(m,m);
    (xx,identity)
}
fn splitmix64(seed: &mut u64) -> u64 {
    *seed=seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z=*seed;
    z=(z^(z>>30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z=(z^(z>>27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z^(z>>31)
}
fn twist_curve() -> Curve {
    Curve {
        p:P,
        a:mul(A, mul(NONSQUARE,NONSQUARE)),
        b:mul(B, mul(NONSQUARE,mul(NONSQUARE,NONSQUARE))),
        n:TWIST_N,
        g:(0,0), // never used: we validate arbitrary public points, not a generator
    }
}
fn check_sample(x: u64, expected_class: i8, ar: &mut Arithmetic) -> (u64,u64) {
    assert_eq!(quadratic_character(x), expected_class);
    let curve=if expected_class==1 {PRIME31} else {twist_curve()};
    // x' = d*x on E^d converts an original twist abscissa to a rational point.
    let transformed_x=if expected_class==1 {x} else {mul(NONSQUARE,x)};
    let yy = ar.sqrt(ar.rhs(curve, transformed_x),P)
        .expect("public x is not on its predicted curve/twist");
    let original_point: Point=Some((transformed_x,yy));
    assert!(ar.on_curve(curve, original_point));
    let mut at=x;
    for _ in 0..4 {
        let (next,identity)=x_double(at);
        assert!(identity, "failed exact doubling square polynomial identity");
        assert_eq!(quadratic_character(next),expected_class, "twist jump");
        at=next;
    }
    let x16=at;
    let q16=ar.scalar(curve, 16, original_point)
        .expect("odd group order excludes order dividing 16");
    let canonical_x16=if expected_class==1 {q16.0} else {
        mul(q16.0,pow(NONSQUARE,P-2))
    };
    assert_eq!(x16,canonical_x16,"x-only chain versus original Rust group law");
    let inverse=if expected_class==1 {INV16_E} else {INV16_T};
    let recovered=ar.scalar(curve,inverse,Some(q16))
        .expect("inverse scalar must produce a point");
    assert_eq!(Some(recovered),original_point,"inverse [16] point failed");
    (x16,transformed_x)
}
fn run(samples: usize) -> Value {
    assert!((1..=512).contains(&samples));
    assert_eq!(PRIME31.p,P);
    assert_eq!((PRIME31.a,PRIME31.b,PRIME31.n),(A,B,N));
    assert_eq!(PRIME31.g,(625_037_715,2_100_627_251));
    assert_eq!(2*P+2-N,TWIST_N);
    assert_eq!(N%2,1);
    assert_eq!(TWIST_N%2,1);
    assert_eq!(pow(NONSQUARE,(P-1)/2),P-1);
    assert_eq!((INV16_E*16)%N,1);
    assert_eq!((INV16_T*16)%TWIST_N,1);
    let start=Instant::now();
    let mut ar=Arithmetic::default();
    check_curve(PRIME31,&mut ar).expect("original curve full-order certificate");
    let mut seed=SEED;
    let mut counts=[0usize;2];
    let mut certificates:Vec<Value>=Vec::new();
    let mut x_seed_draws=0u64;
    while counts[0]<samples || counts[1]<samples {
        let x=splitmix64(&mut seed)%P;
        x_seed_draws+=1;
        let class=quadratic_character(x);
        let i=if class==1 {0} else {1};
        if counts[i]==samples {continue;}
        let (x16,_)=check_sample(x,class,&mut ar);
        if counts[i]<32 {
            certificates.push(json!({
                "kind":if i==0 {"curve"} else {"twist"},
                "ordinal":counts[i], "x":x, "x16":x16
            }));
        }
        counts[i]+=1;
    }
    let mut basis=Vec::new();
    for x in 0..10_000u64 {
        if quadratic_character(x)==1 {
            check_sample(x,1,&mut ar);
            basis.push(x);
            if basis.len()==31 {break;}
        }
    }
    assert_eq!(basis.len(),31);
    assert_eq!(&basis[..8],&[0,1,2,3,5,6,8,9]);
    assert_eq!(counts,[samples,samples]);
    assert_eq!(certificates.len(),64);
    json!({
        "schema":"prime31.kummer16.rational_fibre.v1",
        "status":"source_specific_producer_unindependently_signed",
        "curve":{"p":P,"a":A,"b":B,"order":N,"cofactor":1},
        "twist":{"order":TWIST_N,"nonsquare":NONSQUARE,"a":twist_curve().a,"b":twist_curve().b},
        "inverse16":{"curve":INV16_E,"twist":INV16_T},
        "sample_seed":SEED.to_string(),
        "random_x_draws":x_seed_draws,
        "sample_counts":{"curve":counts[0],"twist":counts[1],"factor_base":basis.len()},
        "factor_base_x":basis,
        "certificates":certificates,
        "all_chain_and_inverse_assertions_passed":true,
        "original_rust_arithmetic_counts":ar.counts,
        "producer_wall_ns":start.elapsed().as_nanos().to_string(),
        "generic_sum_oracle_used":false,
        "public_only_ecdlp_extractor":false,
        "complete_ecdlp_solved":false,
        "pcc_verified":false
    })
}
fn main() {
    println!("{}",run(SAMPLES));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_curve_and_twist_orders_are_odd() {
        assert_eq!(2*P+2-N,TWIST_N);
        assert_eq!(pow(3,(P-1)/2),P-1);
        assert_eq!((16*INV16_E)%N,1);
        assert_eq!((16*INV16_T)%TWIST_N,1);
    }
    #[test]
    fn finite_field_doubling_identity_and_character() {
        for x in 0..300 {
            let (y,good)=x_double(x);
            assert!(good);
            assert_eq!(quadratic_character(x),quadratic_character(y));
        }
    }
    #[test]
    fn source_rust_inverse_replay_both_classes() {
        let mut ar=Arithmetic::default();
        for x in 0..15 {
            let class=quadratic_character(x);
            let (x16,_)=check_sample(x,class,&mut ar);
            let mut via=x;
            for _ in 0..4 {via=x_double(via).0;}
            assert_eq!(via,x16);
        }
    }
}