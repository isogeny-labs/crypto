//! Prime-order twists of class-number-one CM curves, with their canonical lifts.
//!
//! For each discriminant `D` and prime `p`, scan `d = 1, 2, ...` and test the
//! twist `(a, b) = (a0 d^2, b0 d^3) mod p` for prime order.  The integers
//! `A = a0 d^2`, `B = b0 d^3` define a lift of that curve to `Z/p^k` whose
//! reduction is the twist and which is the CM lift up to `Z_p`-isomorphism.
//! Each hit is printed as `D p d a b lift_a lift_b r` (for `k = 4`) and written
//! to the TSV file given as the first argument (default `hits.tsv`), ready for
//! `jet_digit_degree --a --b --lift-a --lift-b`.
use crypto_lib::cryptanalysis::ic_boundary::PrimeCurve;
use num_bigint::BigInt;
use num_traits::{ToPrimitive, Zero};
use std::io::Write;

const K: u32 = 4;
const MAX_D: u64 = 2000;
const HITS_PER_PAIR: usize = 2;

/// `(D, j, a0, b0)`; `None` models are derived from `j`.
fn models() -> Vec<(i64, BigInt, BigInt, BigInt)> {
    let explicit: [(i64, i64, i64, i64); 3] = [
        (-7, -3375, -35, -98),
        (-8, 8000, -30, -56),
        (-11, -32768, -264, -1694),
    ];
    let mut v: Vec<_> = explicit
        .iter()
        .map(|&(d, j, a, b)| (d, BigInt::from(j), BigInt::from(a), BigInt::from(b)))
        .collect();
    let derived: [(i64, &str); 4] = [
        (-19, "-884736"),
        (-43, "-884736000"),
        (-67, "-147197952000"),
        (-163, "-262537412640768000"),
    ];
    for (d, js) in derived {
        let j: BigInt = js.parse().unwrap();
        let m = BigInt::from(1728) - &j;
        let a = BigInt::from(3) * &j * &m;
        let b = BigInt::from(2) * &j * &m * &m;
        v.push((d, j, a, b));
    }
    v
}

/// True iff `1728 * 4 a^3 / (4 a^3 + 27 b^2) == j` as an exact rational.
fn j_matches(a: &BigInt, b: &BigInt, j: &BigInt) -> bool {
    let four_a3 = BigInt::from(4) * a * a * a;
    let den = &four_a3 + BigInt::from(27) * b * b;
    !den.is_zero() && BigInt::from(1728) * four_a3 == j * den
}

fn rem(x: &BigInt, m: &BigInt) -> BigInt {
    ((x % m) + m) % m
}

/// `(a, b, lift_a, lift_b)` for the twist by `d` at prime `p` with `k = K`.
fn lift(a0: &BigInt, b0: &BigInt, d: u64, p: u64) -> (u64, u64, u64, u64) {
    let pb = BigInt::from(p);
    let dd = BigInt::from(d);
    let big_a = a0 * &dd * &dd;
    let big_b = b0 * &dd * &dd * &dd;
    let pk = pb.pow(K);
    let pk1 = pb.pow(K - 1);
    let one = |x: &BigInt| {
        let r = rem(x, &pb);
        let l = rem(&((x - &r) / &pb), &pk1);
        (r.to_u64().unwrap(), l.to_u64().unwrap())
    };
    let (a, la) = one(&big_a);
    let (b, lb) = one(&big_b);
    assert_eq!(
        rem(&(BigInt::from(a) + BigInt::from(la) * &pb), &pk),
        rem(&big_a, &pk)
    );
    assert_eq!(
        rem(&(BigInt::from(b) + BigInt::from(lb) * &pb), &pk),
        rem(&big_b, &pk)
    );
    (a, b, la, lb)
}

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| "hits.tsv".into());
    let mut out = std::fs::File::create(&path).unwrap();
    writeln!(out, "D\tp\td\ta\tb\tlift_a\tlift_b\tr").unwrap();
    for (dd, j, a0, b0) in models() {
        assert!(j_matches(&a0, &b0, &j), "j mismatch for D={dd}");
        for p in [40961u64, 12289] {
            let mut hits = 0;
            let mut skipped = 0;
            for d in 1..=MAX_D {
                if hits == HITS_PER_PAIR {
                    break;
                }
                let (a, b, la, lb) = lift(&a0, &b0, d, p);
                let disc = (4 * (a as u128).pow(3) + 27 * (b as u128).pow(2)) % p as u128;
                if disc == 0 {
                    skipped += 1;
                    continue;
                }
                if let Some(r) = (PrimeCurve { p, a, b }).prime_order() {
                    if r == p {
                        continue;
                    }
                    let line = format!("{dd}\t{p}\t{d}\t{a}\t{b}\t{la}\t{lb}\t{r}");
                    println!("{}", line.replace('\t', " "));
                    writeln!(out, "{line}").unwrap();
                    hits += 1;
                }
            }
            if hits == 0 {
                println!(
                    "# D={dd} p={p}: no prime-order twist for d<={MAX_D} (singular d: {skipped})"
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_models_have_expected_j() {
        for (d, j, a, b) in models().into_iter().take(3) {
            assert!(j_matches(&a, &b, &j), "D={d}");
        }
        assert!(!j_matches(
            &BigInt::from(-35),
            &BigInt::from(-98),
            &BigInt::from(1)
        ));
    }

    #[test]
    fn derived_models_have_expected_j() {
        for (d, j, a, b) in models() {
            assert!(j_matches(&a, &b, &j), "D={d}");
        }
    }

    #[test]
    fn lift_identity_on_a_hit() {
        let (_, _, a0, b0) = models().remove(4); // D = -43, which has hits at p = 40961
        let p = 40961u64;
        let pk = BigInt::from(p).pow(K);
        for d in 1..=MAX_D {
            let (a, b, la, lb) = lift(&a0, &b0, d, p);
            if (PrimeCurve { p, a, b }).prime_order().is_some() {
                let dd = BigInt::from(d);
                let aa = &a0 * &dd * &dd;
                let bb = &b0 * &dd * &dd * &dd;
                let ta = BigInt::from(a) + BigInt::from(la) * BigInt::from(p);
                let tb = BigInt::from(b) + BigInt::from(lb) * BigInt::from(p);
                assert_eq!(rem(&ta, &pk), rem(&aa, &pk));
                assert_eq!(rem(&tb, &pk), rem(&bb, &pk));
                return;
            }
        }
        panic!("no hit");
    }
}
