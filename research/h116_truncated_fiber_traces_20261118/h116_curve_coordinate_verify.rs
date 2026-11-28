use std::{env, fs};

const PRECISION: usize = 24;
const EXPECTED: [(&str, u64, u64, u64, i64); 5] = [
    ("fp31-r37", 31, 1, 19, -5),
    ("fp211-r197", 211, 2, 24, 15),
    ("fp241-r257", 241, 2, 4, -15),
    ("fp431-r397", 431, 1, 225, 35),
    ("fp587-r547", 587, 1, 342, 41),
];

fn pow_mod(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut out = 1;
    while n > 0 { if n & 1 == 1 { out = out * x % p; } x = x * x % p; n >>= 1; }
    out
}

fn mul(x: &[u64], y: &[u64], p: u64) -> Vec<u64> {
    let mut out = vec![0; PRECISION];
    for i in 0..PRECISION { for j in 0..(PRECISION - i) { out[i + j] = (out[i + j] + x[i] * y[j]) % p; } }
    out
}

fn inverse(x: &[u64], p: u64) -> Vec<u64> {
    assert_ne!(x[0], 0);
    let mut out = vec![0; PRECISION];
    out[0] = pow_mod(x[0], p - 2, p);
    for n in 1..PRECISION {
        let mut total = 0;
        for i in 1..=n { total = (total + x[i] * out[n - i]) % p; }
        out[n] = (p - out[0] * total % p) % p;
    }
    out
}

fn q_recurrence(a: u64, b: u64, p: u64) -> Vec<u64> {
    let mut q = vec![0; PRECISION];
    q[0] = 1;
    for n in 1..PRECISION {
        let q2 = mul(&q, &q, p);
        let q3 = mul(&q2, &q, p);
        let term_a = if n >= 4 { a * q2[n - 4] % p } else { 0 };
        let term_b = if n >= 6 { b * q3[n - 6] % p } else { 0 };
        q[n] = (term_a + term_b) % p;
    }
    q
}

fn compose(outer: &[u64], inner: &[u64], p: u64) -> Vec<u64> {
    assert_eq!(inner[0], 0);
    let mut out = vec![0; PRECISION];
    let mut power = vec![0; PRECISION]; power[0] = 1;
    for &coefficient in outer {
        for i in 0..PRECISION { out[i] = (out[i] + coefficient * power[i]) % p; }
        power = mul(&power, inner, p);
    }
    out
}

fn scale(x: &[u64], scalar: u64, p: u64) -> Vec<u64> { x.iter().map(|v| v * scalar % p).collect() }

fn rescale(q: &[u64], scalar: u64, p: u64) -> Vec<u64> {
    (0..PRECISION).map(|i| q[i] * pow_mod(scalar, i as u64, p) % p).collect()
}

fn parse_csv(value: &str) -> Vec<u64> {
    let parsed: Vec<u64> = value.split(',').map(|x| x.parse().unwrap()).collect();
    assert_eq!(parsed.len(), PRECISION);
    parsed
}

fn first_difference(x: &[u64], y: &[u64]) -> i64 {
    x.iter().zip(y).position(|(a, b)| a != b).map_or(-1, |i| i as i64)
}

fn main() {
    let path = env::args().nth(1).expect("usage: verifier RESULT.tsv");
    let input = fs::read_to_string(path).unwrap();
    let mut lines = input.lines();
    assert_eq!(lines.next(), Some("schema\th116.curve-coordinate-control/v1"));
    let header = lines.next().unwrap();
    assert_eq!(header.split('\t').count(), 21);
    let rows: Vec<&str> = lines.collect();
    assert_eq!(rows.len(), EXPECTED.len());
    let mut curve = 0; let mut rescale_ok = 0; let mut double = 0; let mut ratio = 0;
    let mut wrong_scale = 0; let mut wrong_parameter = 0;
    for (row, expected) in rows.iter().zip(EXPECTED) {
        let f: Vec<&str> = row.split('\t').collect(); assert_eq!(f.len(), 21);
        let p: u64 = f[1].parse().unwrap(); let a: u64 = f[2].parse().unwrap();
        let b: u64 = f[3].parse().unwrap(); let trace: i64 = f[4].parse().unwrap();
        assert_eq!((f[0], p, a, b, trace), expected); assert_eq!(f[6], "24");
        let scalar = ((trace.rem_euclid(p as i64) as u64) + p - 1) % p; assert_eq!(f[5].parse::<u64>().unwrap(), scalar);
        let source_q = parse_csv(f[7]); let target_q = parse_csv(f[8]); let z = parse_csv(f[9]);
        let target_at = parse_csv(f[10]); let source_at = parse_csv(f[11]);
        let expected_source = q_recurrence(a, b, p);
        let ta = a * pow_mod(scalar, 4, p) % p; let tb = b * pow_mod(scalar, 6, p) % p;
        let expected_target = q_recurrence(ta, tb, p);
        assert_eq!(source_q, expected_source); assert_eq!(target_q, expected_target);
        assert_eq!(target_q, rescale(&source_q, scalar, p)); rescale_ok += 1;
        assert_eq!(target_at, compose(&target_q, &z, p));
        let z_scalar = scale(&z, scalar, p); assert_eq!(source_at, compose(&source_q, &z_scalar, p));
        assert_eq!(target_at, source_at); double += 1;
        let unit: Vec<u64> = z[1..].iter().copied().chain(std::iter::once(0)).collect();
        let pole = mul(&inverse(&unit, p), &inverse(&unit, p), p);
        let common = mul(&source_q, &pole, p);
        assert_eq!(mul(&common, &inverse(&target_at, p), p), mul(&common, &inverse(&source_at, p), p)); ratio += 1;
        assert_eq!(f[12], "true"); curve += 1;
        assert_eq!(f[13], "true"); assert_eq!(f[14], "true"); assert_eq!(f[15], "true");
        let wa = a * pow_mod(scalar, 2, p) % p; let wb = b * pow_mod(scalar, 4, p) % p;
        let wrong_at = compose(&q_recurrence(wa, wb, p), &z, p);
        let ws = first_difference(&wrong_at, &source_at); assert!(ws >= 0); assert_eq!(f[16].parse::<i64>().unwrap(), ws); wrong_scale += 1;
        let wrong_z = scale(&z, (scalar + 1) % p, p);
        let wp = first_difference(&target_at, &compose(&source_q, &wrong_z, p)); assert!(wp >= 0); assert_eq!(f[17].parse::<i64>().unwrap(), wp); wrong_parameter += 1;
        assert!(f[18].parse::<u64>().unwrap() > 0 && f[19].parse::<u64>().unwrap() > 0 && f[20].parse::<u64>().unwrap() > 0);
    }
    println!("{{\"schema\":\"h116.curve-coordinate-independent-verification/v1\",\"verified_cells\":{},\"curve_identity_cells\":{},\"rescale_matches\":{},\"double_substitution_matches\":{},\"normalized_ratio_matches\":{},\"wrong_scaling_rejected\":{},\"wrong_parameter_rejected\":{}}}", rows.len(), curve, rescale_ok, double, ratio, wrong_scale, wrong_parameter);
}
