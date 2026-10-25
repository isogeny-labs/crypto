//! Exact finite genus-character diagnostic; no class-number upper certificate.
use crypto_lib::cryptanalysis::isogeny_degree_search::class_number::is_prime_u64;
use serde_json::json;
use std::collections::BTreeMap;
const D: i128 = -6649310858224389603;
const FACTORS: [u64; 5] = [3, 97, 113, 35831, 5643470711];
const DEGREES: [u64; 8] = [3, 7, 11, 13, 19, 23, 53, 61];
fn powmod(mut a: u64, mut n: u64, p: u64) -> u64 {
    let mut r = 1;
    a %= p;
    while n > 0 {
        if n & 1 == 1 {
            r = (r as u128 * a as u128 % p as u128) as u64;
        }
        a = (a as u128 * a as u128 % p as u128) as u64;
        n >>= 1;
    }
    r
}
fn legendre(a: u64, p: u64) -> i8 {
    match powmod(a, (p - 1) / 2, p) {
        0 => 0,
        1 => 1,
        x if x == p - 1 => -1,
        _ => panic!("Euler criterion failed"),
    }
}
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
fn mask(n: u64) -> u8 {
    FACTORS.iter().enumerate().fold(0, |bits, (i, &p)| {
        let c = legendre(n, p);
        assert_ne!(c, 0, "represented norm must be coprime to D");
        bits | if c == -1 { 1 << i } else { 0 }
    })
}
fn rank(rows: &[u8]) -> usize {
    let mut basis = [0u8; 5];
    let mut r = 0;
    for &row in rows {
        let mut v = row;
        for bit in (0..5).rev() {
            if v & (1 << bit) == 0 {
                continue;
            }
            if basis[bit] == 0 {
                basis[bit] = v;
                r += 1;
                break;
            }
            v ^= basis[bit];
        }
    }
    r
}
fn span(rows: &[u8]) -> BTreeMap<u8, u16> {
    assert!(rows.len() <= 8);
    let mut out = BTreeMap::new();
    for subset in 0..(1u16 << rows.len()) {
        let value =
            rows.iter().enumerate().fold(
                0,
                |v, (i, row)| {
                    if subset & (1 << i) != 0 {
                        v ^ row
                    } else {
                        v
                    }
                },
            );
        out.entry(value).or_insert(subset);
    }
    out
}
fn form(ell: u64) -> (i128, i128, i128) {
    let a = ell as i128;
    let b = (1..=2 * a).find(|b| (b * b - D) % (4 * a) == 0).unwrap();
    let c = (b * b - D) / (4 * a);
    assert_eq!(b * b - 4 * a * c, D);
    assert_eq!(gcd(gcd(a as u128, b as u128), c as u128), 1);
    (a, b, c)
}
fn main() {
    assert!(FACTORS.iter().all(|&p| is_prime_u64(p)));
    assert_eq!(-3i128 * 97 * 113 * -35831 * -5643470711, D);
    let mut rows = Vec::new();
    let mut records = Vec::new();
    let mut checked_norms = 0;
    for ell in DEGREES {
        let (a, b, c) = form(ell);
        let (x, y) = if gcd(ell as u128, D.unsigned_abs()) == 1 {
            (1, 0)
        } else {
            (0, 1)
        };
        let n = (a * x * x + b * x * y + c * y * y) as u64;
        assert_eq!(gcd(n as u128, D.unsigned_abs()), 1);
        let m = mask(n);
        assert_eq!(m.count_ones() % 2, 0);
        for u in -3i128..=3 {
            for v in -3i128..=3 {
                if gcd(u.unsigned_abs(), v.unsigned_abs()) != 1 {
                    continue;
                }
                let represented = (a * u * u + b * u * v + c * v * v) as u64;
                if gcd(represented as u128, D.unsigned_abs()) == 1 {
                    assert_eq!(mask(represented), m);
                    checked_norms += 1;
                }
            }
        }
        rows.push(m);
        records.push(
            json!({"prime":ell,"form":[a,b,c],"represented_by":[x,y],"represented_norm":n,"mask":m,
            "characters":FACTORS.map(|p|legendre(n,p))}),
        );
    }
    let all = span(&rows);
    let r = rank(&rows);
    assert_eq!(all.len(), 1usize << r);
    let complete = (0u8..32).all(|m| all.contains_key(&m) == (m.count_ones() % 2 == 0));
    let witnesses: Vec<_> = all.iter().map(|(&m,&s)|json!({"mask":m,"subset_bits":s,
        "degrees":DEGREES.iter().enumerate().filter_map(|(i,&p)| if s&(1<<i)!=0 {Some(p)} else {None}).collect::<Vec<_>>() })).collect();
    println!(
        "{}",
        json!({"schema":"prime61.genus-span/v1","discriminant":D,"prime_discriminants":[-3i64,97,113,-35831,-5643470711i64],
        "rows":records,"rank":r,"exhaustive_span_size":all.len(),"all_even_parity_masks_spanned":complete,"witnesses":witnesses,
        "primitive_representations_checked":checked_norms,"full_class_number_certified":false,"full_class_generation_certified":false,
        "scope":"Finite character computations and span only. Genus/class-group and prior exact-order bridges must be stated separately; a conditional full class number remains conditional."})
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn euler_matches_exhaustive_square_sets() {
        for p in [3u64, 97, 113, 35831] {
            let mut squares = vec![false; p as usize];
            for a in 1..p {
                squares[(a * a % p) as usize] = true;
            }
            for a in 0..p {
                assert_eq!(
                    legendre(a, p),
                    if a == 0 {
                        0
                    } else if squares[a as usize] {
                        1
                    } else {
                        -1
                    }
                );
            }
        }
    }
    #[test]
    fn elimination_matches_exhaustive_subsets_including_deficient_cases() {
        assert_eq!(rank(&[3, 3, 3]), 1);
        assert_eq!(span(&[3, 3, 3]).len(), 2);
        assert_eq!(rank(&[3, 5, 9, 17]), 4);
        for seed in 0u64..512 {
            let rows: Vec<_> = (0..8)
                .map(|i| {
                    ((seed.wrapping_mul(6364136223846793005).rotate_left(i * 3) >> 11) & 31) as u8
                })
                .collect();
            assert_eq!(span(&rows).len(), 1usize << rank(&rows));
        }
    }
}
