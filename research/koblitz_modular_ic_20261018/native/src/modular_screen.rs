//! Determinant-compatible necessary condition for an Eisenstein character pair.
use crate::{modular, round02};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::One;
use serde_json::json;
use std::path::Path;
pub fn run(dir: &Path) {
    let mut rows = Vec::new();
    for (n, ell, r) in [
        (3, 7, "7"),
        (5, 11, "11"),
        (131, 787, "680564733841876926932320129493409985129"),
    ] {
        let level = 49u64 * ell * ell;
        let d = modular::factorize(level)
            .into_iter()
            .map(|(p, k)| p.pow(k - 1) * (p - 1))
            .fold(1u64, |a, b| a.lcm(&b));
        assert_eq!(d % 2, 0);
        let prime = r.parse::<BigInt>().unwrap();
        for p in [3u64, 5, 13, 17, 19] {
            let mut points = 1;
            for x in 0..p {
                for y in 0..p {
                    if (y * y + x * y) % p
                        == ((x * x % p * x) as i64 - (x * x) as i64 - 2 * x as i64 - 1)
                            .rem_euclid(p as i64) as u64
                    {
                        points += 1;
                    }
                }
            }
            assert_eq!(p as i64 + 1 - points, 0);
            let value = (BigInt::from(-(p as i64)).modpow(&BigInt::from(d / 2), &prime)
                - BigInt::one())
            .mod_floor(&prime);
            let bezout = value.extended_gcd(&prime);
            assert_eq!(&bezout.x * &value + &bezout.y * &prime, bezout.gcd);
            rows.push(json!({"n":n,"level":level,"r":r,"p":p,"unit_exponent":d,"determinant_exponent":d/2,"power_minus_one_mod_r":value.to_string(),"gcd":bezout.gcd.to_string(),"bezout_value":bezout.x.to_string(),"bezout_r":bezout.y.to_string(),"necessary_condition_passes":!bezout.gcd.is_one()}));
        }
    }
    let n5fails = rows
        .iter()
        .filter(|r| r["n"] == 5 && r["necessary_condition_passes"] == false)
        .count();
    assert!(n5fails > 0);
    assert!(rows
        .iter()
        .filter(|r| r["n"] == 3)
        .all(|r| r["necessary_condition_passes"] == true));
    round02::save(
        dir,
        "mr3-determinant-screen.json",
        &json!({"schema":"koblitz.round03/mr3-v1","rows":rows,"n5_failed_prime_tests":n5fails,"scope":"uses epsilon*eta=chi^2 as well as the zero-Hecke condition; finite arithmetic necessary condition, no blanket modular-unit theorem"}),
    );
    println!("MR3: {n5fails} n5 prime tests fail the determinant-compatible necessary condition");
}
