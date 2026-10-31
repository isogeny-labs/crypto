// Non-Solinas primes with a real SNFS-type route to an elliptic curve: the
// pairing-friendly polynomial families. For each family, find concrete
// parameters (p = P(u), prime subgroup order r = R(u)), and VERIFY them with
// exact arithmetic: p and r prime, r | p + 1 - t, and the embedding degree
// k = min{k : r | p^k - 1} equals the family's k. A family whose published
// polynomials fail the check is reported as failed, not assumed.
//
//   cargo run --release --example snfs_pairing_prime_families -- --per-family 3
use crypto_lib::cryptanalysis::mov_attack::embedding_degree;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, Zero};

fn mr(n: &BigUint) -> bool {
    let two = BigUint::from(2u32);
    if n < &two {
        return false;
    }
    for p in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53] {
        let bp = BigUint::from(p);
        if n == &bp {
            return true;
        }
        if (n % &bp).is_zero() {
            return false;
        }
    }
    let nm1 = n - 1u32;
    let mut d = nm1.clone();
    let mut s = 0;
    while (&d & BigUint::one()).is_zero() {
        d >>= 1;
        s += 1;
    }
    'w: for a in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53] {
        let mut x = BigUint::from(a).modpow(&d, n);
        if x == BigUint::one() || x == nm1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = (&x * &x) % n;
            if x == nm1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

fn bi(x: i64) -> BigInt {
    BigInt::from(x)
}

fn poly(coeffs: &[i64], u: &BigInt) -> BigInt {
    // coeffs from highest degree to constant
    let mut acc = BigInt::zero();
    for &c in coeffs {
        acc = acc * u + bi(c);
    }
    acc
}

struct Family {
    name: &'static str,
    k: u32,
    // p, r, t as (numerator coeffs high->low, denominator)
    p: (&'static [i64], i64),
    r: (&'static [i64], i64),
    t: (&'static [i64], i64),
    deg_p: u32,
    ubits: u32,
}

fn eval_exact(f: &(&[i64], i64), u: &BigInt) -> Option<BigInt> {
    let num = poly(f.0, u);
    let den = bi(f.1);
    if (&num % &den).is_zero() {
        Some(num / den)
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let per: usize = args
        .iter()
        .position(|a| a == "--per-family")
        .and_then(|i| args.get(i + 1)?.parse().ok())
        .unwrap_or(2);
    let max_tries: u64 = 4_000_000;

    let fams = [
        Family {
            name: "BN (k=12)",
            k: 12,
            p: (&[36, 36, 24, 6, 1], 1),
            r: (&[36, 36, 18, 6, 1], 1),
            t: (&[6, 0, 1], 1),
            deg_p: 4,
            ubits: 63,
        },
        // BLS: r = Phi_k(u), p = (u-1)^2 r / 3 + u, t = u + 1; p and t are
        // computed in the search loop, so the p field below is unused.
        Family {
            name: "BLS12 (k=12)",
            k: 12,
            p: (&[0], 1),
            r: (&[1, 0, -1, 0, 1], 1),
            t: (&[1, 1], 1),
            deg_p: 6,
            ubits: 64,
        },
        Family {
            name: "BLS24 (k=24)",
            k: 24,
            p: (&[0], 1),
            r: (&[1, 0, 0, 0, -1, 0, 0, 0, 1], 1),
            t: (&[1, 1], 1),
            deg_p: 10,
            ubits: 48,
        },
        Family {
            name: "KSS16 (k=16)",
            k: 16,
            p: (&[1, 2, 5, 0, 48, 152, 240, 0, 625, 2398, 3125], 980),
            r: (&[1, 0, 0, 0, 48, 0, 0, 0, 625], 61250),
            t: (&[0], 1), // (2u^5 + 41u + 35)/35, computed in the loop
            deg_p: 10,
            ubits: 34,
        },
        Family {
            name: "KSS18 (k=18)",
            k: 18,
            p: (&[1, 5, 7, 37, 188, 259, 343, 1763, 2401], 21),
            r: (&[1, 0, 0, 37, 0, 0, 343], 343),
            t: (&[0], 1), // (u^4 + 16u + 7)/7, computed in the loop
            deg_p: 8,
            ubits: 44,
        },
        Family {
            name: "Freeman (k=10)",
            k: 10,
            p: (&[25, 25, 25, 10, 3], 1),
            r: (&[25, 25, 15, 5, 1], 1),
            t: (&[10, 5, 3], 1),
            deg_p: 4,
            ubits: 63,
        },
    ];

    println!("family\tu_bits\tdeg P\tlog2 p\tlog2 r\trho\tk(verified)\tlog2 p^k\tu (hex)\tchecks");
    for f in fams.iter() {
        // Build the exact (numerator, denominator) closures per family.
        let mut found = 0usize;
        let mut tries = 0u64;
        let mut u = (BigInt::one() << f.ubits) + bi(1);
        // u increments; negative u not searched
        while found < per && tries < max_tries {
            tries += 1;
            u += bi(1);
            let (p, r, t) = match f.name {
                "BLS12 (k=12)" | "BLS24 (k=24)" => {
                    let rr = poly(f.r.0, &u);
                    let um1 = &u - bi(1);
                    let num = &um1 * &um1 * &rr;
                    if !(&num % bi(3)).is_zero() {
                        continue;
                    }
                    (num / bi(3) + &u, rr, &u + bi(1))
                }
                "KSS16 (k=16)" => {
                    let p = match eval_exact(&f.p, &u) {
                        Some(x) => x,
                        None => continue,
                    };
                    let r = match eval_exact(&f.r, &u) {
                        Some(x) => x,
                        None => continue,
                    };
                    let num = bi(2) * u.pow(5) + bi(41) * &u + bi(35);
                    if !(&num % bi(35)).is_zero() {
                        continue;
                    }
                    (p, r, num / bi(35))
                }
                "KSS18 (k=18)" => {
                    let p = match eval_exact(&f.p, &u) {
                        Some(x) => x,
                        None => continue,
                    };
                    let r = match eval_exact(&f.r, &u) {
                        Some(x) => x,
                        None => continue,
                    };
                    let num = u.pow(4) + bi(16) * &u + bi(7);
                    if !(&num % bi(7)).is_zero() {
                        continue;
                    }
                    (p, r, num / bi(7))
                }
                _ => {
                    let p = poly(f.p.0, &u);
                    let r = poly(f.r.0, &u);
                    let t = poly(f.t.0, &u);
                    (p, r, t)
                }
            };
            if p.sign() != Sign::Plus || r.sign() != Sign::Plus {
                continue;
            }
            let pu = p.to_biguint().unwrap();
            let ru = r.to_biguint().unwrap();
            if !mr(&ru) || !mr(&pu) {
                continue;
            }
            // exact checks
            let order = &p + bi(1) - &t;
            let r_divides = (&order % &r).is_zero();
            let kk = embedding_degree(&pu, &ru, 2 * f.k + 2);
            let ok = r_divides && kk == Some(f.k);
            println!(
                "{}\t{}\t{}\t{:.1}\t{:.1}\t{:.2}\t{}\t{:.0}\t0x{:x}\t{}",
                f.name,
                f.ubits,
                f.deg_p,
                pu.bits() as f64,
                ru.bits() as f64,
                pu.bits() as f64 / ru.bits() as f64,
                kk.map(|k| k.to_string()).unwrap_or("none".into()),
                pu.bits() as f64 * kk.unwrap_or(0) as f64,
                u.to_biguint().unwrap(),
                if ok {
                    "ok: p,r prime; r | p+1-t; k matches"
                } else if !r_divides {
                    "FAIL: r does not divide p+1-t"
                } else {
                    "FAIL: embedding degree differs"
                }
            );
            found += 1;
        }
        if found == 0 {
            println!("{}\t{}\t{}\t-\t-\t-\t-\t-\t-\tno parameters found in {} tries", f.name, f.ubits, f.deg_p, tries);
        }
    }
}
