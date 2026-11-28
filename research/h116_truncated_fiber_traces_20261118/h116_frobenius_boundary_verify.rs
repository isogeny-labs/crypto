use std::{env, fs};

const P: u64 = 31;
const A: u64 = 1;
const B: u64 = 19;
const SCALE: u64 = 25;
const PRECISION: usize = 18;

const PHI_N_REV: [u64; 38] = [
    1, 7, 11, 7, 15, 30, 20, 14, 25, 11, 2, 16, 10, 4, 19, 14, 0, 26, 17,
    6, 0, 12, 12, 28, 12, 5, 8, 14, 26, 17, 15, 7, 28, 1, 11, 20, 21, 3,
];
const PHI_D_REV: [u64; 37] = [
    1, 7, 22, 22, 27, 19, 11, 13, 26, 12, 25, 18, 22, 6, 6, 7, 18, 18, 6,
    29, 29, 9, 3, 17, 28, 17, 6, 15, 24, 29, 27, 4, 9, 18, 28, 11, 19,
];

type Poly = Vec<u64>;

fn trim(mut x: Poly) -> Poly {
    while x.len() > 1 && x.last() == Some(&0) { x.pop(); }
    x
}
fn add(x: u64, y: u64) -> u64 { (x + y) % P }
fn sub(x: u64, y: u64) -> u64 { (x + P - y) % P }
fn mul(x: u64, y: u64) -> u64 { x * y % P }
fn pow_mod(mut x: u64, mut n: u64) -> u64 {
    let mut out = 1;
    while n > 0 { if n & 1 == 1 { out = mul(out, x); } x = mul(x, x); n >>= 1; }
    out
}
fn padd(x: &[u64], y: &[u64]) -> Poly {
    trim((0..x.len().max(y.len())).map(|i| add(x.get(i).copied().unwrap_or(0), y.get(i).copied().unwrap_or(0))).collect())
}
fn psub(x: &[u64], y: &[u64]) -> Poly {
    trim((0..x.len().max(y.len())).map(|i| sub(x.get(i).copied().unwrap_or(0), y.get(i).copied().unwrap_or(0))).collect())
}
fn pmul(x: &[u64], y: &[u64]) -> Poly {
    let mut out = vec![0; x.len() + y.len() - 1];
    for (i, &a) in x.iter().enumerate() { for (j, &b) in y.iter().enumerate() { out[i + j] = add(out[i + j], mul(a, b)); } }
    trim(out)
}
fn pscale(x: &[u64], a: u64) -> Poly { trim(x.iter().map(|&v| mul(v, a)).collect()) }
fn ppow(x: &[u64], n: usize) -> Poly { let mut out = vec![1]; for _ in 0..n { out = pmul(&out, x); } out }
fn pderivative(x: &[u64]) -> Poly { if x.len() < 2 { vec![0] } else { trim(x.iter().enumerate().skip(1).map(|(i, &v)| mul(i as u64 % P, v)).collect()) } }
fn shift_x(x: &[u64]) -> Poly { let mut out = vec![0]; out.extend_from_slice(x); trim(out) }

fn curve() -> Poly { vec![B, A, 0, 1] }

fn scalar_six_division_polynomials() -> (Poly, Poly) {
    let f = curve();
    let a2 = mul(A, A); let b2 = mul(B, B);
    let psi3 = vec![sub(0, a2), mul(12, B), mul(6, A), 0, 3];
    let g = vec![sub(sub(0, mul(8, b2)), mul(a2, A)), sub(0, mul(4, mul(A, B))), sub(0, mul(5, a2)), mul(20, B), mul(5, A), 0, 1];
    let psi5 = psub(&pscale(&pmul(&ppow(&f, 2), &g), 32 % P), &ppow(&psi3, 3));
    let h = psub(&psi5, &pscale(&ppow(&g, 2), 4));
    let psi6_squared = pscale(&pmul(&pmul(&f, &ppow(&psi3, 2)), &ppow(&h, 2)), 4);
    let psi7 = psub(
        &pmul(&psi5, &ppow(&psi3, 3)),
        &pscale(&pmul(&ppow(&f, 2), &ppow(&g, 3)), 128 % P),
    );
    let numerator = psub(&shift_x(&psi6_squared), &pmul(&psi5, &psi7));
    (numerator, psi6_squared)
}

fn identity(n: &[u64], d: &[u64], at: u64, bt: u64) -> bool {
    let h = psub(&pmul(&pderivative(n), d), &pmul(n, &pderivative(d)));
    let lhs = pmul(&curve(), &ppow(&h, 2));
    let d2 = ppow(d, 2); let d3 = pmul(&d2, d);
    let rhs = pmul(d, &padd(&padd(&ppow(n, 3), &pscale(&pmul(n, &d2), at)), &pscale(&d3, bt)));
    lhs == rhs
}

fn quotient(n: &[u64], d: &[u64]) -> Vec<u64> {
    let inv_d0 = pow_mod(d[0], P - 2);
    let mut out = vec![0; PRECISION];
    for k in 0..PRECISION {
        let mut value = n.get(k).copied().unwrap_or(0);
        for i in 1..=k.min(d.len() - 1) { value = sub(value, mul(d[i], out[k - i])); }
        out[k] = mul(value, inv_d0);
    }
    out
}

fn parse_csv(x: &str) -> Vec<u64> { let out: Vec<u64> = x.split(',').map(|v| v.parse().unwrap()).collect(); assert_eq!(out.len(), PRECISION); out }
fn first_difference(x: &[u64], y: &[u64]) -> i64 { x.iter().zip(y).position(|(a,b)| a != b).map_or(-1, |i| i as i64) }

fn main() {
    let path = env::args().nth(1).expect("usage: verifier RESULT.tsv");
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    assert_eq!(lines.next(), Some("schema\th116.frobenius-boundary-result/v1"));
    assert_eq!(lines.next().unwrap().split('\t').count(), 18);
    let rows: Vec<&str> = lines.collect(); assert_eq!(rows.len(), 1);
    let f: Vec<&str> = rows[0].split('\t').collect(); assert_eq!(f.len(), 18);
    assert_eq!((f[0], f[1], f[2]), ("31", "25", "18"));

    let recorded_phi = parse_csv(f[3]); let recorded_scalar = parse_csv(f[4]);
    let expected_phi = quotient(&PHI_N_REV, &PHI_D_REV); assert_eq!(recorded_phi, expected_phi);
    let (scalar_n, scalar_d) = scalar_six_division_polynomials();
    assert_eq!((scalar_n.len() - 1, scalar_d.len() - 1), (36, 35));
    let scale2 = mul(SCALE, SCALE); let scale4 = mul(scale2, scale2); let scale6 = mul(scale4, scale2);
    let scalar_normalized_n = pscale(&scalar_n, scale2);
    let mut scalar_n_rev = scalar_normalized_n.clone(); scalar_n_rev.reverse();
    let mut scalar_d_rev = scalar_d.clone(); scalar_d_rev.reverse();
    let expected_scalar = quotient(&scalar_n_rev, &scalar_d_rev); assert_eq!(recorded_scalar, expected_scalar);
    let first = first_difference(&expected_phi, &expected_scalar); assert_eq!(first, 15); assert_eq!(f[5].parse::<i64>().unwrap(), first);
    let delta = sub(expected_phi[first as usize], expected_scalar[first as usize]); assert_eq!(delta, 10); assert_eq!(f[6].parse::<u64>().unwrap(), delta);
    assert_eq!((f[7], f[8]), ("15", "10"));

    let at = mul(scale4, A); let bt = mul(scale6, B);
    let mut phi_n = PHI_N_REV.to_vec(); phi_n.reverse(); let mut phi_d = PHI_D_REV.to_vec(); phi_d.reverse();
    assert!(identity(&phi_n, &phi_d, at, bt)); assert_eq!(f[9], "true");
    assert!(identity(&scalar_normalized_n, &scalar_d, at, bt)); assert_eq!(f[10], "true");
    let mut scalar_unscaled_rev = scalar_n.clone(); scalar_unscaled_rev.reverse();
    let omitted_first = first_difference(&expected_phi, &quotient(&scalar_unscaled_rev, &scalar_d_rev)); assert_eq!(omitted_first, 0); assert_eq!(f[11], "0");
    assert!(!identity(&scalar_normalized_n, &scalar_d, mul(scale2, A), mul(scale4, B))); assert_eq!(f[12], "true");
    let mut mutated_rev = scalar_n_rev.clone(); mutated_rev[15] = add(mutated_rev[15], 1);
    assert_eq!(first_difference(&quotient(&mutated_rev, &scalar_d_rev), &expected_scalar), 15); assert_eq!(f[13], "15");
    mutated_rev.reverse(); assert!(!identity(&mutated_rev, &scalar_d, at, bt)); assert_eq!(f[14], "true");
    assert!(f[15].parse::<u64>().unwrap() > 0 && f[16].parse::<u64>().unwrap() > 0 && f[17].parse::<u64>().unwrap() > 0);
    println!("{{\"schema\":\"h116.frobenius-boundary-independent-verification/v1\",\"verified_cells\":1,\"first_difference\":15,\"delta\":10,\"equal_coefficients\":15,\"map_identities\":2,\"negative_controls_rejected\":3}}");
}
