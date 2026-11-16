//! CM construction of a prime-order curve over a 61-bit prime, plus u128 EC
//! arithmetic used only to find the twist and generator.

use hashed_ecdl_tensor::curve::Curve;
use hashed_ecdl_tensor::reduction::{decomposition_network, FactorBase, HashSeed};
use hashed_ecdl_tensor::curve::Point;
use hashed_ecdl_tensor::{BigUint, Limits, Meter};

fn mul_mod(p: u128, a: u128, b: u128) -> u128 {
    (a % p) * (b % p) % p
}

fn pow_mod(p: u128, mut base: u128, mut exp: u128) -> u128 {
    base %= p;
    let mut acc = 1u128;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = mul_mod(p, acc, base);
        }
        base = mul_mod(p, base, base);
        exp >>= 1;
    }
    acc
}

fn isqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }
    let mut x = (n as f64).sqrt() as u128 + 2;
    loop {
        let y = (x + n / x) / 2;
        if y >= x {
            break;
        }
        x = y;
    }
    while x * x > n {
        x -= 1;
    }
    x
}

pub fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    for a in [2u128, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = pow_mod(n, a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut ok = false;
        for _ in 1..s {
            x = mul_mod(n, x, x);
            if x == n - 1 {
                ok = true;
                break;
            }
        }
        if !ok {
            return false;
        }
    }
    true
}

/// p = a^2 + 3 b^2 for p == 1 mod 3 (Cornacchia, D = 3).
pub fn cornacchia_d3(p: u128) -> Option<(u128, u128)> {
    let n = (p + p - 3) % p; // -3 mod p
    let r = pow_mod(p, n, (p + 1) / 4);
    if mul_mod(p, r, r) != n {
        return None;
    }
    let (mut a, mut b) = (p, r.min(p - r));
    let lim = isqrt(p);
    while b > lim {
        let t = a % b;
        a = b;
        b = t;
    }
    let x = b;
    let rem = p - x * x;
    if rem % 3 != 0 {
        return None;
    }
    let y2 = rem / 3;
    let y = isqrt(y2);
    (y * y == y2).then_some((x, y))
}

type Pt = Option<(u128, u128)>;

fn ec_add(p: u128, a: u128, l: Pt, r: Pt) -> Pt {
    let m = |x: i128| -> u128 {
        let pp = p as i128;
        let mut v = x % pp;
        if v < 0 {
            v += pp;
        }
        v as u128
    };
    match (l, r) {
        (None, q) => q,
        (q, None) => q,
        (Some((x1, y1)), Some((x2, y2))) => {
            if x1 == x2 && (y1 + y2) % p == 0 {
                return None;
            }
            let lambda = if x1 == x2 && y1 == y2 {
                if y1 == 0 {
                    return None;
                }
                let num = m(3 * (x1 as i128) * (x1 as i128) + a as i128);
                mul_mod(p, num, pow_mod(p, 2 * y1 % p, p - 2))
            } else {
                let num = m(y2 as i128 - y1 as i128);
                let den = m(x2 as i128 - x1 as i128);
                mul_mod(p, num, pow_mod(p, den, p - 2))
            };
            let x3 = m((lambda as i128) * (lambda as i128) - x1 as i128 - x2 as i128);
            let y3 = m((lambda as i128) * (x1 as i128 - x3 as i128) - y1 as i128);
            Some((x3, y3))
        }
    }
}

fn ec_mul(p: u128, a: u128, mut k: u128, pt: Pt) -> Pt {
    let mut acc: Pt = None;
    let mut base = pt;
    while k > 0 {
        if k & 1 == 1 {
            acc = ec_add(p, a, acc, base);
        }
        base = ec_add(p, a, base, base);
        k >>= 1;
    }
    acc
}

fn sqrt_mod(p: u128, n: u128) -> Option<u128> {
    if n == 0 {
        return Some(0);
    }
    let r = pow_mod(p, n, (p + 1) / 4);
    (mul_mod(p, r, r) == n % p).then_some(r)
}

fn find_point(p: u128, b: u128) -> Option<(u128, u128)> {
    for x in 1..2000u128 {
        let rhs = (mul_mod(p, mul_mod(p, x, x), x) + b) % p;
        if let Some(y) = sqrt_mod(p, rhs) {
            return Some((x, y));
        }
    }
    None
}

/// Returns (b, r, generator) with E: y^2 = x^3 + b having prime order r.
pub fn prime_order_curve(p: u128) -> Option<(u128, u128, (u128, u128))> {
    let (a, b) = cornacchia_d3(p)?;
    let (ai, bi, pi) = (a as i128, b as i128, p as i128);
    let mut traces = vec![
        2 * ai,
        -2 * ai,
        ai + 3 * bi,
        -(ai + 3 * bi),
        ai - 3 * bi,
        -(ai - 3 * bi),
    ];
    traces.sort_unstable();
    traces.dedup();
    for t in traces {
        let r = pi + 1 - t;
        if r <= 5 {
            continue;
        }
        let r = r as u128;
        if !is_prime(r) {
            continue;
        }
        for coeff in 1..64u128 {
            let Some(g) = find_point(p, coeff) else { continue };
            if ec_mul(p, 0, r, Some(g)) == None {
                return Some((coeff, r, g));
            }
        }
    }
    None
}

fn big(x: u128) -> BigUint {
    BigUint::from(x)
}

pub fn run(p: u128) {
    println!("=== Fp instance: p = {p} ({} bits)", 128 - p.leading_zeros());
    let t0 = std::time::Instant::now();
    let Some((coeff, r, (gx, gy))) = prime_order_curve(p) else {
        println!("   no prime-order j=0 curve found (need p = 1 mod 3, p = 3 mod 4)");
        return;
    };
    println!(
        "   CM: E: y^2 = x^3 + {coeff}, prime order r = {r} ({} bits, {:.2?})",
        128 - r.leading_zeros(),
        t0.elapsed()
    );

    let mut meter = Meter::new(Limits {
        steps: 2_000_000_000_000,
        vector: 64_000_000,
        bits: 1_000_000,
    });
    let t0 = std::time::Instant::now();
    let curve = match Curve::new(
        big(p),
        big(0),
        big(coeff),
        big(r),
        Point::Affine { x: big(gx), y: big(gy) },
        &mut meter,
    ) {
        Ok(c) => c,
        Err(e) => {
            println!("   crate rejected the curve: {e:?}");
            return;
        }
    };
    println!("   Curve::new (primality) {:.2?}, steps={}", t0.elapsed(), meter.used);

    let s = {
        let mut cand = 2 * p + 1;
        while !is_prime(cand) {
            cand += 2;
        }
        cand
    };
    let t0 = std::time::Instant::now();
    let mut bases = Vec::new();
    for k in 0..2u8 {
        match FactorBase::new(
            &curve,
            big(s),
            big(8),
            HashSeed { alpha: big(3 + k as u128), beta: big(5) },
            &mut meter,
        ) {
            Ok(b) => bases.push(b),
            Err(e) => {
                println!("   factor base rejected: {e:?}");
                return;
            }
        }
    }
    println!("   two factor bases {:.2?}, steps={}", t0.elapsed(), meter.used);

    let g = Point::Affine { x: big(gx), y: big(gy) };
    let target = curve.mul(&g, &big(2), &mut meter).expect("2G");
    let t0 = std::time::Instant::now();
    match decomposition_network(&curve, &bases, &target, &mut meter) {
        Ok(net) => println!(
            "   Lemma 11 network: edges={} vertices={} max_degree={} interface={} ({:.2?})",
            net.edge_count(),
            net.vertex_count(),
            net.maximum_degree(),
            net.interface().len(),
            t0.elapsed()
        ),
        Err(e) => println!("   network rejected: {e:?}"),
    }
}
