//! Instrument an exact algebraic boundary, not Shor's full postprocessor.
use num_bigint::BigInt;
use num_traits::{One, Zero};
use serde_json::json;
use std::io::Write;

fn modulo(x: BigInt, r: &BigInt) -> BigInt {
    ((x % r) + r) % r
}

fn recover(a: &BigInt, b: &BigInt, r: &BigInt) -> (Option<BigInt>, usize) {
    assert!(r > &BigInt::one());
    let (mut old_r, mut rem) = (r.clone(), modulo(b.clone(), r));
    let (mut old_s, mut s) = (BigInt::zero(), BigInt::one());
    let mut divisions = 0;
    while !rem.is_zero() {
        let q = &old_r / &rem;
        (old_r, rem) = (rem.clone(), old_r - &q * &rem);
        (old_s, s) = (s.clone(), old_s - &q * &s);
        divisions += 1;
    }
    if old_r != BigInt::one() {
        return (None, divisions);
    }
    (Some(modulo(-a * old_s, r)), divisions)
}

fn rnsl(n: u32) -> (u32, f64) {
    let log = (n as f64).log2();
    (9 * n + 2 * log.ceil() as u32 + 10,
     448.0 * (n as f64).powi(3) * log + 4090.0 * (n as f64).powi(3))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::env::args().nth(1).ok_or("usage: quantum-boundary OUTPUT.json")?;
    let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(output)?;
    let mut resources = Vec::new();
    for n in [192u32, 224, 256, 384, 521] {
        let (qubits, toffoli) = rnsl(n);
        for shots in [1u32, 2, 4, 8, 16] {
            for success in [0.25f64, 0.5, 0.75] {
                resources.push(json!({
                    "field_bits":n, "shots":shots,
                    "assumed_independent_per_shot_success":success,
                    "derived_at_least_one_success":1.0-(1.0-success).powi(shots as i32),
                    "classical_setup_point_doublings_once":2*n,
                    "modeled_compressed_table_bytes":2*(n+1)*(n.div_ceil(8)+1),
                    "candidate_verification_scalar_multiplications_per_success":1,
                    "candidate_verification_executed":false,
                    "rnsl2017_logical_qubit_bound":qubits,
                    "rnsl2017_toffoli_bound_per_shot":toffoli,
                    "rnsl2017_toffoli_bound_all_shots":toffoli*shots as f64,
                    "classification":"derived operation/resource counts; not measured runtime"
                }));
            }
        }
    }
    let r = BigInt::from(2305843009213693951u64);
    let mut diagnostics = Vec::new();
    for secret in [1u64, 17, 20261018] {
        for denominator in [0u64, 1, 2, 17, 65537, 104729] {
            let b = BigInt::from(denominator);
            let a = modulo(-&b * BigInt::from(secret), &r);
            let (answer, divisions) = recover(&a, &b, &r);
            let valid = match &answer {
                Some(d) => d == &BigInt::from(secret) && modulo(&a + &b*d, &r).is_zero(),
                None => denominator == 0,
            };
            if !valid { return Err("synthetic congruence diagnostic failed".into()); }
            diagnostics.push(json!({"r":r.to_string(),"synthetic_secret":secret,
                "a":a.to_string(),"b":denominator,
                "recovered":answer.map(|v|v.to_string()), "extended_euclid_divisions":divisions,
                "valid":valid, "status":if denominator==0 {"rejected_noninvertible"}else{"exact_congruence_recovered"},
                "elliptic_curve_verification":false}));
        }
    }
    serde_json::to_writer_pretty(&mut file,&json!({
        "schema":"quantum-classical-boundary/v1","item":20,
        "protocol":"research/ideas_execution_20261018/quantum_protocol.json",
        "scope":"Exact ideal congruences only; excludes noisy power-of-two Fourier recovery and actual point verification",
        "resources":resources,"diagnostics":diagnostics,
        "classification":"partial resource arithmetic and native classical diagnostic; no quantum run"
    }))?;
    file.write_all(b"\n")?;
    println!("wrote {} resource rows and {} checked exact-congruence diagnostics",resources.len(),diagnostics.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn congruences_cover_every_unit_and_secret_on_small_prime() {
        let r=BigInt::from(101);
        for d in 0..101 {
            for b in 1..101 {
                let a=modulo(BigInt::from(-b*d),&r);
                assert_eq!(recover(&a,&BigInt::from(b),&r).0,Some(BigInt::from(d)));
            }
        }
    }
    #[test]
    fn nonunits_are_rejected_instead_of_dividing() {
        assert_eq!(recover(&1.into(),&0.into(),&101.into()).0,None);
        assert_eq!(recover(&5.into(),&5.into(),&15.into()).0,None);
    }
    #[test]
    fn published_256_bit_qubit_bound() {
        let(q,t)=rnsl(256);
        assert_eq!(q,2330);
        assert!((t-128748355584.0).abs()<1.0);
    }
}
