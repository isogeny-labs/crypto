use hec_native_ic::{fp::Fp, jac, poly::Poly};
use std::fs;
const P: u64 = 127;
pub fn load(path: &str) -> (jac::Curve, jac::Div, jac::Div, u64) {
    let raw = fs::read_to_string(path).unwrap();
    let mut it = raw.split_whitespace();
    assert_eq!(it.next(), Some("COLD_INPUT"));
    let ns: Vec<u64> = it.map(|s| s.parse().unwrap()).collect();
    assert_eq!(&ns[..2], &[3, P]);
    let ell = ns[2];
    assert!(ell > 2 && (2u64..).take_while(|d| d * d <= ell).all(|d| ell % d != 0));
    let mut at = 3;
    let mut ps = Vec::new();
    for _ in 0..5 {
        let n = ns[at] as usize;
        at += 1;
        ps.push(Poly::from_coeffs(&ns[at..at + n]));
        at += n
    }
    assert_eq!(at, ns.len());
    let refc = jac::Curve::new(Fp::new(P), ps[0]);
    let d = jac::Div { u: ps[1], v: ps[2] };
    let b = jac::Div { u: ps[3], v: ps[4] };
    for x in [&d, &b] {
        assert!(refc.is_valid(x) && !refc.is_zero(x));
        assert!(refc.is_zero(&refc.mul(x, ell)))
    }
    (refc, d, b, ell)
}
