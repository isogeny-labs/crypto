//! Independent check of the Oakley Pratt certificates (no code shared with
//! `lean/OakleyPrimality.lean`). Reads `validation/oakley_pratt_certificates.json`
//! and re-verifies every certificate: the factors multiply to `p - 1`,
//! `a^(p-1) = 1 (mod p)`, and `a^((p-1)/q) != 1 (mod p)` for every listed `q`.
//! Each listed `q` is below the small-prime bound (prime by trial division) or
//! has its own certificate. It also checks the two roots against the integers
//! in `lean/OakleyOrders.lean`. Exit status 0 means every check passed.

use num_bigint::BigUint;
use num_traits::One;
use std::collections::{HashMap, HashSet};
use std::{fs, process::exit};

fn trial_prime(n: u64) -> bool {
    n >= 2 && (2u64..).take_while(|d| d * d <= n).all(|d| n % d != 0)
}

fn lean_def(src: &str, name: &str) -> BigUint {
    let key = format!("def {name} : ℕ := ");
    let rest = &src[src.find(&key).unwrap_or_else(|| panic!("{name} not found")) + key.len()..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse().expect("number")
}

struct Cert {
    witness: BigUint,
    factors: Vec<BigUint>,
}

fn check(
    p: &BigUint,
    bound: u64,
    certs: &HashMap<BigUint, Cert>,
    proved: &mut HashSet<BigUint>,
) -> Result<(), String> {
    if proved.contains(p) {
        return Ok(());
    }
    if *p < BigUint::from(bound) {
        let small: u64 = p.to_string().parse().unwrap();
        if !trial_prime(small) {
            return Err(format!("{p} is not prime"));
        }
    } else {
        let c = certs.get(p).ok_or(format!("no certificate for {p}"))?;
        let one = BigUint::one();
        let pm1 = p - &one;
        let prod = c.factors.iter().fold(BigUint::one(), |a, q| a * q);
        if prod != pm1 {
            return Err(format!("factors of {p}-1 do not multiply out"));
        }
        if c.witness.modpow(&pm1, p) != one {
            return Err(format!("Fermat condition fails for {p}"));
        }
        let distinct: HashSet<&BigUint> = c.factors.iter().collect();
        for q in distinct {
            if c.witness.modpow(&(&pm1 / q), p) == one {
                return Err(format!("witness {} fails at q={q} for {p}", c.witness));
            }
            check(q, bound, certs, proved)?;
        }
    }
    proved.insert(p.clone());
    Ok(())
}

fn run() -> Result<(), String> {
    let dir = std::env::args().nth(1).unwrap_or_else(|| ".".into());
    let data: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(format!("{dir}/validation/oakley_pratt_certificates.json"))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let bound = data["small_prime_bound"].as_u64().ok_or("bound")?;
    let parse = |v: &serde_json::Value| -> BigUint { v.to_string().trim_matches('"').parse().unwrap() };
    let mut certs = HashMap::new();
    for (k, v) in data["certificates"].as_object().ok_or("certificates")? {
        certs.insert(
            k.parse::<BigUint>().unwrap(),
            Cert {
                witness: parse(&v["witness"]),
                factors: v["p_minus_1_factors"].as_array().ok_or("factors")?.iter().map(parse).collect(),
            },
        );
    }
    let lean = fs::read_to_string(format!("{dir}/lean/OakleyOrders.lean")).map_err(|e| e.to_string())?;
    let mut proved = HashSet::new();
    for name in ["oakley3Prime", "oakley4Prime"] {
        let root = lean_def(&lean, name);
        check(&root, bound, &certs, &mut proved)?;
        println!("PASS {name} = {root} ({} bits) prime by Pratt certificate", root.bits());
    }
    let unused = certs.keys().filter(|p| !proved.contains(*p)).count();
    if unused != 0 {
        return Err(format!("{unused} unused certificates"));
    }
    println!("PASS {} certificates, {} primes checked", certs.len(), proved.len());
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("FAIL {e}");
        exit(1);
    }
}
