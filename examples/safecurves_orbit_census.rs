//! SafeCurves/SECG trapdoor-slot round C: native CM-derived twist-orbit
//! census and grind (research/safecurves_trapdoor_slots_20261017).
//!
//! Replaces the round-A/B SageMath execution paths per the repository's
//! no-Python rule.  The six sextic-twist orders of a j = 0 curve over
//! p = 1 mod 6 are derived from the CM representation 4p = L^2 + 27 M^2
//! (L = 1 mod 3) as p + 1 -/+ t for t in {L, (L + 9M)/2, (9M - L)/2}, with
//! no SEA point counting; the j = 1728 quartic orbit over p = 1 mod 4 is
//! p + 1 -/+ {2a, 2b} for p = a^2 + b^2.  Round A's SEA census
//! (census.json, SageMath) is the cross-check fixture.
//!
//! ```text
//! cargo run --release --example safecurves_orbit_census -- --census --json
//! cargo run --release --example safecurves_orbit_census -- --grind 96
//! cargo run --release --example safecurves_orbit_census -- --grind 128 --max-primes 5000
//! cargo run --release --example safecurves_orbit_census -- --probe256 --max-primes 64
//! ```

use crypto_lib::cryptanalysis::p256_structural::{pollard_rho_factor, trial_factor};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, Zero};

// ── field helpers ──────────────────────────────────────────────────

fn legendre(x: &BigUint, p: &BigUint) -> i8 {
    if x.is_zero() {
        return 0;
    }
    let e = (p - BigUint::one()) >> 1u32;
    if x.modpow(&e, p).is_one() {
        1
    } else {
        -1
    }
}

/// Tonelli–Shanks; returns the smaller of the two roots.
fn sqrt_mod(x: &BigUint, p: &BigUint) -> Option<BigUint> {
    if x.is_zero() {
        return Some(BigUint::zero());
    }
    if legendre(x, p) != 1 {
        return None;
    }
    let one = BigUint::one();
    if (p % 4u32) == BigUint::from(3u32) {
        return Some(x.modpow(&((p + &one) >> 2u32), p));
    }
    let mut q = p - &one;
    let mut s = 0u32;
    while q.is_even() {
        q >>= 1u32;
        s += 1;
    }
    let mut z = BigUint::from(2u32);
    while legendre(&z, p) != -1 {
        z += &one;
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = x.modpow(&q, p);
    let mut r = x.modpow(&((q + &one) >> 1u32), p);
    while !t.is_one() {
        let mut i = 0u32;
        let mut t2 = t.clone();
        while !t2.is_one() {
            t2 = &t2 * &t2 % p;
            i += 1;
            if i == m {
                return None;
            }
        }
        let mut b = c.clone();
        for _ in 0..(m - i - 1) {
            b = &b * &b % p;
        }
        m = i;
        c = &b * &b % p;
        t = &t * &c % p;
        r = &r * &b % p;
    }
    let other = p - &r;
    Some(std::cmp::min(r, other))
}

fn is_prime(n: &BigUint) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    }
    for small in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let s = BigUint::from(small);
        if n == &s {
            return true;
        }
        if (n % &s).is_zero() {
            return false;
        }
    }
    // Miller-Rabin, deterministic-style fixed bases plus random-ish extra.
    let mut d = n - BigUint::one();
    let mut r = 0u32;
    while d.is_even() {
        d >>= 1u32;
        r += 1;
    }
    let bases = [
        2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
    ];
    'outer: for a in bases {
        let a = BigUint::from(a);
        if a >= *n {
            continue;
        }
        let mut x = a.modpow(&d, n);
        if x.is_one() || x == n - BigUint::one() {
            continue;
        }
        for _ in 0..r.saturating_sub(1) {
            x = &x * &x % n;
            if x == n - BigUint::one() {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn next_prime(mut n: BigUint) -> BigUint {
    if n < BigUint::from(2u32) {
        return BigUint::from(2u32);
    }
    let two = BigUint::from(2u32);
    if n.is_even() {
        n += &one_const();
    }
    while !is_prime(&n) {
        n += &two;
    }
    n
}

fn one_const() -> BigUint {
    BigUint::one()
}

fn bits_of(n: &BigUint) -> u64 {
    n.bits()
}

fn isqrt(n: &BigUint) -> BigUint {
    let mut x = BigUint::one() << n.bits().div_ceil(2);
    loop {
        let y = (&x + n / &x) >> 1u32;
        if y >= x {
            return x;
        }
        x = y;
    }
}

fn is_square(n: &BigUint) -> Option<BigUint> {
    let r = isqrt(n);
    if &r * &r == *n {
        Some(r)
    } else {
        None
    }
}

// ── CM representations ─────────────────────────────────────────────

/// Cornacchia: solve x^2 + d y^2 = m for m = 4p (d odd) or m = p.
/// The chain seed is a square root of -d modulo m, lifted from the root
/// modulo p (for m = 4p the seed must also be odd, since -d is odd).
fn cornacchia(d: &BigUint, m: &BigUint, p: &BigUint) -> Vec<(BigUint, BigUint)> {
    let mut out = Vec::new();
    let negd = (m - (d % m)) % m;
    let r = match sqrt_mod(&negd, p) {
        Some(r) => r,
        None => return out,
    };
    let mut seeds = Vec::new();
    if m == p {
        seeds.push(std::cmp::min(r.clone(), p - &r));
    } else {
        // m = 4p: enumerate lifts x = (+/- r + k p) mod 4p, keep odd seeds.
        let rp = r.clone();
        for k in 0u32..4 {
            for base in [rp.clone(), p - &rp] {
                let mut x = (base + (p * k)) % m;
                if x.is_odd() && (&x * &x + d) % m == BigUint::zero() {
                    let other = m - &x;
                    if other < x {
                        x = other;
                    }
                    if !seeds.contains(&x) {
                        seeds.push(x);
                    }
                }
            }
        }
    }
    for x0 in seeds {
        if x0.is_zero() {
            continue;
        }
        let mut a = m.clone();
        let mut b = x0;
        while !b.is_zero() && &b * &b >= *m {
            let next = &a % &b;
            a = b.clone();
            b = next;
        }
        if b.is_zero() || &b * &b >= *m {
            continue;
        }
        let rem = m - &b * &b;
        if rem.is_zero() {
            continue;
        }
        if (&rem % d).is_zero() {
            if let Some(y) = is_square(&(&rem / d)) {
                if &b * &b + d * &y * &y == *m {
                    out.push((b, y));
                }
            }
        }
    }
    out
}

/// j = 0 orbit over p = 1 mod 6: returns (L, M) with 4p = L^2 + 27 M^2.
/// The six traces are +- of {|L|, |L + 9M|/2, |9M - L|/2}, so the sign of L
/// does not affect the order set; no 1-mod-3 sign fix is needed here.
fn cm_j0(p: &BigUint) -> Option<(BigUint, BigUint)> {
    let d27 = BigUint::from(27u32);
    let m4 = p << 2u32;
    cornacchia(&d27, &m4, p).into_iter().next()
}

/// The three positive traces of the j = 0 sextic orbit: sorted
/// {|L|, |L + 9M| / 2, |9M - L| / 2}.
fn j0_traces(p: &BigUint) -> Option<Vec<BigUint>> {
    let (l, m) = cm_j0(p)?;
    let t1 = &l + BigUint::from(9u32) * &m;
    if t1.is_odd() {
        return None;
    }
    let t1 = t1 >> 1u32;
    let t2 = if BigUint::from(9u32) * &m >= l {
        BigUint::from(9u32) * &m - l.clone()
    } else {
        l.clone() - BigUint::from(9u32) * &m
    };
    if t2.is_odd() {
        return None;
    }
    let t2 = t2 >> 1u32;
    let mut ts = vec![l, t1, t2];
    ts.sort();
    ts.dedup();
    Some(ts)
}

/// j = 1728 quartic orbit over p = 1 mod 4: traces {2a, 2b} for p = a^2 + b^2.
fn j1728_traces(p: &BigUint) -> Option<Vec<BigUint>> {
    let one = BigUint::one();
    let solutions = cornacchia(&one, p, p);
    let (a, b) = solutions.into_iter().next()?;
    Some(vec![&a << 1u32, &b << 1u32])
}

// ── factoring with certification ───────────────────────────────────

/// Result of a budgeted full factorisation attempt.
#[derive(Clone, Debug)]
pub struct FactorCert {
    /// All prime factors found (with multiplicity), sorted.
    pub primes: Vec<BigUint>,
    /// True when the factorisation is complete (every piece resolved).
    pub complete: bool,
}

fn factor_cert(n: &BigUint, trial_bound: u64, rho_iters: u64) -> FactorCert {
    let (small, residue) = trial_factor(n, trial_bound);
    let mut primes: Vec<BigUint> = Vec::new();
    for (q, e) in small {
        for _ in 0..e {
            primes.push(BigUint::from(q));
        }
    }
    let mut work = vec![residue];
    let mut complete = true;
    let mut seed = 1u64;
    while let Some(c) = work.pop() {
        if c <= BigUint::one() {
            continue;
        }
        if is_prime(&c) {
            primes.push(c);
            continue;
        }
        match pollard_rho_factor(&c, rho_iters, seed) {
            Some(d) => {
                let other = &c / &d;
                work.push(d);
                work.push(other);
                seed = seed.wrapping_add(1);
            }
            None => {
                // Unresolved composite: keep it visible, mark incomplete.
                primes.push(c);
                complete = false;
            }
        }
    }
    primes.sort();
    FactorCert { primes, complete }
}

// ── census ─────────────────────────────────────────────────────────

struct J0Curve {
    name: &'static str,
    p: BigUint,
    b: BigUint,
    n: BigUint,
}

fn secg_j0_curves() -> Vec<J0Curve> {
    let p192 = (BigUint::one() << 192u32)
        - (BigUint::one() << 32u32)
        - (BigUint::one() << 12u32)
        - (BigUint::one() << 8u32)
        - (BigUint::one() << 7u32)
        - (BigUint::one() << 6u32)
        - (BigUint::one() << 3u32)
        - BigUint::one();
    let p224 = (BigUint::one() << 224u32)
        - (BigUint::one() << 32u32)
        - (BigUint::one() << 12u32)
        - (BigUint::one() << 11u32)
        - (BigUint::one() << 9u32)
        - (BigUint::one() << 7u32)
        - (BigUint::one() << 4u32)
        - BigUint::from(2u32)
        - BigUint::one();
    let p256 = (BigUint::one() << 256u32) - (BigUint::one() << 32u32) - BigUint::from(977u32);
    vec![
        J0Curve {
            name: "secp192k1",
            p: p192,
            b: BigUint::from(3u32),
            n: BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFE26F2FC170F69466A74DEFD8D", 16)
                .unwrap(),
        },
        J0Curve {
            name: "secp224k1",
            p: p224,
            b: BigUint::from(5u32),
            n: BigUint::parse_bytes(
                b"10000000000000000000000000001DCE8D2EC6184CAF0A971769FB1F7",
                16,
            )
            .unwrap(),
        },
        J0Curve {
            name: "secp256k1",
            p: p256,
            b: BigUint::from(7u32),
            n: BigUint::parse_bytes(
                b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
                16,
            )
            .unwrap(),
        },
    ]
}

/// Six orders of the j = 0 orbit over p, from CM alone.
fn j0_orders(p: &BigUint) -> Option<Vec<BigUint>> {
    let ts = j0_traces(p)?;
    let mut orders = Vec::new();
    for t in ts {
        orders.push(p + BigUint::one() - &t);
        orders.push(p + BigUint::one() + &t);
    }
    orders.sort();
    Some(orders)
}

fn run_census(json: bool) -> i32 {
    // Fixture: round A SEA census (SageMath), committed as census.json.
    let fixture =
        std::fs::read_to_string("research/safecurves_trapdoor_slots_20261017/census.json")
            .expect("census.json fixture (run from repository root)");
    let rows: serde_json::Value = serde_json::from_str(&fixture).unwrap();
    let mut ok = true;
    let mut report = Vec::new();
    for curve in secg_j0_curves() {
        assert!(
            !curve.b.is_zero() && curve.b < curve.p,
            "invalid j=0 coefficient"
        );
        let computed = j0_orders(&curve.p).expect("CM representation");
        let mut sea_orders: Vec<BigUint> = Vec::new();
        let row = rows
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["curve"] == curve.name)
            .unwrap();
        for c in row["classes"].as_array().unwrap() {
            sea_orders.push(
                BigUint::parse_bytes(c["order"].to_string().trim_matches('"').as_bytes(), 10)
                    .unwrap(),
            );
        }
        sea_orders.sort();
        let mut sea_sorted = sea_orders.clone();
        sea_sorted.sort();
        sea_sorted.dedup();
        let match_all = computed.len() == sea_sorted.len()
            && computed.iter().zip(sea_sorted.iter()).all(|(a, b)| a == b);
        // published order must appear in the computed orbit
        let has_published = computed.contains(&curve.n);
        ok &= match_all && has_published;
        report.push(serde_json::json!({
            "curve": curve.name,
            "orders_match_sea": match_all,
            "published_order_in_orbit": has_published,
        }));
        if !json {
            println!(
                "## {}: CM orders match SEA census: {}, published order in orbit: {}",
                curve.name, match_all, has_published
            );
            println!("  CM orders: {:?}", computed);
        }
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    }
    // j = 1728 quartic fixture (SageMath-verified, /tmp/cm5.sage):
    // p = 281474976710677: traces +/-30797298, +/-13320148
    let q = BigUint::from(281474976710677u64);
    let qts = j1728_traces(&q).expect("j1728 CM");
    let mut qts_sorted = qts.clone();
    qts_sorted.sort();
    let expected = vec![BigUint::from(13320148u64), BigUint::from(30797298u64)];
    let q_ok = qts_sorted == expected;
    ok &= q_ok;
    if !json {
        println!(
            "## j=1728 quartic fixture p=281474976710677: traces {:?} (expected 13320148, 30797298): {}",
            qts_sorted, q_ok
        );
    }
    if ok {
        0
    } else {
        1
    }
}

// ── grind and bounded probe share one scan ─────────────────────────

struct GrindHit {
    p: BigUint,
    scanned: u64,
    published_order: BigUint,
    published_lf_bits: u64,
    twist_order: BigUint,
    twist_lf_bits: u64,
    weak_order: BigUint,
    weak_lf_bits: u64,
    classes: Vec<(BigUint, Option<u64>)>,
}

/// Scan primes p = 1 mod 6 above 2^bits for a certified (published,
/// quadratic twist, weak sibling) triple.  `probe` mode additionally
/// counts candidates whose six orders could not be fully factored within
/// budget (only meaningful at large sizes).
fn run_grind(bits: u64, max_primes: u64, rho_iters: u64, json: bool, probe: bool) -> i32 {
    let strong_bits = (0.79 * bits as f64).ceil() as u64;
    let weak_bits = (0.40 * bits as f64).floor() as u64;
    let strong_thr = BigUint::one() << strong_bits;
    let weak_thr = BigUint::one() << weak_bits;
    let mut p = next_prime(BigUint::one() << bits);
    let mut scanned = 0u64;
    let mut uncertified = 0u64;
    while scanned < max_primes {
        if (&p % BigUint::from(6u32)) == BigUint::one() {
            scanned += 1;
            if let Some(orders) = j0_orders(&p) {
                // orders sorted ascending: pair (i, 5-i) is (p+1+t, p+1-t).
                let mut lfs: Vec<Option<BigUint>> = Vec::with_capacity(6);
                let mut complete = true;
                for o in &orders {
                    let cert = factor_cert(o, 1_000_000, rho_iters);
                    if cert.complete {
                        lfs.push(Some(cert.primes.last().unwrap().clone()));
                    } else {
                        complete = false;
                        lfs.push(None);
                    }
                }
                if !complete {
                    uncertified += 1;
                    if probe {
                        eprintln!(
                            "prime #{}: uncertifiable orbit ({} so far)",
                            scanned, uncertified
                        );
                    }
                    p = next_prime(p + BigUint::one());
                    continue;
                }
                let lfs: Vec<BigUint> = lfs.into_iter().map(|x| x.unwrap()).collect();
                for i in 0..3usize {
                    let lf_pub = &lfs[5 - i];
                    let lf_tw = &lfs[i];
                    if lf_pub >= &strong_thr && lf_tw >= &strong_thr {
                        for j in 0..3usize {
                            if j == i {
                                continue;
                            }
                            let lf_weak = std::cmp::min(&lfs[5 - j], &lfs[j]);
                            if lf_weak <= &weak_thr {
                                let hit = GrindHit {
                                    p: p.clone(),
                                    scanned,
                                    published_order: orders[5 - i].clone(),
                                    published_lf_bits: bits_of(lf_pub),
                                    twist_order: orders[i].clone(),
                                    twist_lf_bits: bits_of(lf_tw),
                                    weak_order: if lfs[5 - j] <= lfs[j] {
                                        orders[5 - j].clone()
                                    } else {
                                        orders[j].clone()
                                    },
                                    weak_lf_bits: bits_of(lf_weak),
                                    classes: orders
                                        .iter()
                                        .zip(lfs.iter())
                                        .map(|(o, lf)| (o.clone(), Some(bits_of(lf))))
                                        .collect(),
                                };
                                emit_hit(&hit, bits, json);
                                if probe {
                                    eprintln!(
                                        "note: {} of {} scanned primes had uncertifiable orders",
                                        uncertified, scanned
                                    );
                                }
                                return 0;
                            }
                        }
                    }
                }
            }
        }
        p = next_prime(p + BigUint::one());
    }
    if json {
        println!(
            "{}",
            serde_json::json!({
                "bit_length": bits,
                "found": false,
                "scanned_primes": scanned,
                "uncertified": uncertified,
            })
        );
    } else {
        println!(
            "# {} bits: no certified pair in {} primes = 1 mod 6 ({} uncertifiable)",
            bits, scanned, uncertified
        );
    }
    1
}

fn emit_hit(hit: &GrindHit, bits: u64, json: bool) {
    if json {
        let classes: Vec<serde_json::Value> = hit
            .classes
            .iter()
            .map(|(o, lf)| {
                serde_json::json!({
                    "order": o.to_string(),
                    "largest_factor_bits": lf,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "bit_length": bits,
                "found": true,
                "scanned_primes": hit.scanned,
                "p": hit.p.to_string(),
                "published_order": hit.published_order.to_string(),
                "published_lf_bits": hit.published_lf_bits,
                "twist_order": hit.twist_order.to_string(),
                "twist_lf_bits": hit.twist_lf_bits,
                "weak_order": hit.weak_order.to_string(),
                "weak_lf_bits": hit.weak_lf_bits,
                "classes": classes,
            }))
            .unwrap()
        );
    } else {
        println!(
            "# {} bits (found after {} primes = 1 mod 6)",
            bits, hit.scanned
        );
        println!("- p = {}", hit.p);
        println!(
            "- published order {}, largest factor {} bits",
            hit.published_order, hit.published_lf_bits
        );
        println!(
            "- quadratic twist order {}, largest factor {} bits",
            hit.twist_order, hit.twist_lf_bits
        );
        println!(
            "- weak sibling order {}, largest factor {} bits (rho ~ 2^{})",
            hit.weak_order,
            hit.weak_lf_bits,
            hit.weak_lf_bits as f64 / 2.0
        );
        println!("| order | largest factor (bits) |");
        println!("|---|---:|");
        for (o, lf) in &hit.classes {
            println!("| {} | {:?} |", o, lf);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json = args.iter().any(|a| a == "--json");
    let max_primes: u64 = args
        .iter()
        .position(|a| a == "--max-primes")
        .and_then(|i| args[i + 1].parse().ok())
        .unwrap_or(4096);
    let code = if args.iter().any(|a| a == "--census") {
        run_census(json)
    } else if args.iter().any(|a| a == "--probe256") {
        run_grind(256, max_primes, 1 << 21, json, true)
    } else if args.iter().any(|a| a == "--grind") {
        let bits: u64 = args
            .iter()
            .position(|a| a == "--grind")
            .and_then(|i| args[i + 1].parse().ok())
            .expect("--grind BITS");
        run_grind(bits, max_primes, 1 << 24, json, false)
    } else {
        eprintln!("usage: --census | --grind BITS | --probe256 [--max-primes N] [--json]");
        2
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cornacchia_j0_secp256k1() {
        // Round-A fixture: the six SEA orders of secp256k1's sextic orbit.
        let p = (BigUint::one() << 256u32) - (BigUint::one() << 32u32) - BigUint::from(977u32);
        let orders = j0_orders(&p).expect("CM orbit");
        assert_eq!(orders.len(), 6);
        // published order (SEC 2) must be in the orbit
        let n = BigUint::parse_bytes(
            b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap();
        assert!(orders.contains(&n));
        // sum of all six = 6(p+1)
        let sum: BigUint = orders.iter().sum();
        assert_eq!(sum, (&p + BigUint::one()) * BigUint::from(6u32));
    }

    #[test]
    fn j1728_fixture() {
        let q = BigUint::from(281474976710677u64);
        let mut ts = j1728_traces(&q).expect("quartic CM");
        ts.sort();
        assert_eq!(
            ts,
            vec![BigUint::from(13320148u64), BigUint::from(30797298u64)]
        );
    }

    #[test]
    fn cornacchia_j0_secp192k1_and_224k1() {
        // pari qfbsolve fixtures (sage, /tmp/corn2.sage):
        // secp192k1: L = 125699780136902255086547808575 (sign-free),
        //            M = 18567167572617867047915583263
        let p192 = (BigUint::one() << 192u32)
            - (BigUint::one() << 32u32)
            - (BigUint::one() << 12u32)
            - (BigUint::one() << 8u32)
            - (BigUint::one() << 7u32)
            - (BigUint::one() << 6u32)
            - (BigUint::one() << 3u32)
            - BigUint::one();
        let (l, m) = cm_j0(&p192).expect("secp192k1 CM");
        assert_eq!(
            (l, m),
            (
                BigUint::parse_bytes(b"125699780136902255086547808575", 10).unwrap(),
                BigUint::parse_bytes(b"18567167572617867047915583263", 10).unwrap()
            )
        );
        let orders = j0_orders(&p192).expect("orbit");
        let n192 =
            BigUint::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFE26F2FC170F69466A74DEFD8D", 16).unwrap();
        assert!(orders.contains(&n192));
    }

    #[test]
    fn factor_cert_complete_on_small() {
        // 2^5 * 3 * 101 * 65537 * 2^31-1 composite
        let n = BigUint::from(2u32).pow(5)
            * BigUint::from(3u32)
            * BigUint::from(101u32)
            * BigUint::from(65537u32)
            * BigUint::from(2147483647u64);
        let cert = factor_cert(&n, 100, 1 << 16);
        assert!(cert.complete);
        assert_eq!(cert.primes.last().unwrap(), &BigUint::from(2147483647u64));
    }

    #[test]
    fn grind_hits_at_48_bits() {
        // Round B found a certified pair within 2 primes at 48 bits.
        let code = run_grind(48, 64, 1 << 20, true, false);
        assert_eq!(code, 0);
    }
}
