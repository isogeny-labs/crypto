//! CM construction of a prime-order curve over a 61-bit prime, plus u128 EC
//! arithmetic used only to find the twist and generator.

use hashed_ecdl_tensor_fast::curve::Curve;
use hashed_ecdl_tensor_fast::reduction::{decomposition_network, FactorBase, HashSeed};
use hashed_ecdl_tensor_fast::curve::Point;
use hashed_ecdl_tensor_fast::{BigUint, Limits, Meter};

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

pub fn run(p: u128, simple: bool, program: bool, derived: bool, scalar: bool) {
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
            HashSeed { alpha: if simple { big(1) } else { big(3 + k as u128) }, beta: if simple { big(0) } else { big(5) } },
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
    let net_result = if scalar {
        hashed_ecdl_tensor_fast::reduction::decomposition_network_scalar(&curve, &bases, &target, &mut meter)
    } else if derived {
        hashed_ecdl_tensor_fast::reduction::decomposition_network_derived(&curve, &bases, &target, program, &mut meter)
    } else if program {
        hashed_ecdl_tensor_fast::reduction::decomposition_network_program(&curve, &bases, &target, &mut meter)
    } else {
        decomposition_network(&curve, &bases, &target, &mut meter)
    };
    match net_result {
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

pub fn check_small(simple: bool, derived: bool, program: bool, scalar: bool) {
    use hashed_ecdl_tensor_fast::arithmetic::Seeded as FSeeded;
    use hashed_ecdl_tensor_fast::contraction::Counter as FCounter;
    use hashed_ecdl_tensor_fast::curve::{Curve as FCurve, Point as FPoint};
    use hashed_ecdl_tensor_fast::reduction::{
        decomposition_network as fnet, decomposition_network_program as fnet_program,
        FactorBase as FBase, HashSeed as FSeed,
    };
    use hashed_ecdl_tensor_fast::{BigUint as FBig, Limits as FLimits, Meter as FMeter};

    println!("=== fast-copy count vs brute force (patched encoder)");
    for bits in [3u32, 5] {
        let (p, a, b, r, g) = crate::find_curve(bits).unwrap();
        let mut meter = FMeter::new(FLimits {
            steps: 5_000_000_000,
            vector: 8_000_000,
            bits: 4_000_000,
        });
        let fbig = |x: u128| FBig::from(x);
        let fpt = |xy: Option<(u128, u128)>| match xy {
            None => FPoint::Infinity,
            Some((x, y)) => FPoint::Affine { x: fbig(x), y: fbig(y) },
        };
        let curve = FCurve::new(fbig(p), fbig(a), fbig(b), fbig(r), fpt(Some(g)), &mut meter)
            .expect("fast curve");
        let s = {
            let mut cand = 2 * p + 1;
            while !crate::is_prime_u128(cand) { cand += 2; }
            cand
        };
        let mk = |k: u128, meter: &mut FMeter| {
            FBase::new(
                &curve,
                fbig(s),
                fbig(8),
                FSeed {
                    alpha: if simple { fbig(1) } else { fbig(3 + k) },
                    beta: if simple { fbig(0) } else { fbig(5) },
                },
                meter,
            )
            .expect("base")
        };
        let b1 = mk(0, &mut meter);
        let b2 = mk(1, &mut meter);
        let target = curve.mul(&fpt(Some(g)), &fbig(2), &mut meter).expect("target");
        let net = if scalar {
            hashed_ecdl_tensor_fast::reduction::decomposition_network_scalar(
                &curve, &[b1.clone(), b2.clone()], &target, &mut meter,
            ).expect("scalar net")
        } else if derived {
            hashed_ecdl_tensor_fast::reduction::decomposition_network_derived(
                &curve,
                &[b1.clone(), b2.clone()],
                &target,
                program,
                &mut meter,
            )
            .expect("derived net")
        } else if program {
            fnet_program(&curve, &[b1.clone(), b2.clone()], &target, &mut meter)
                .expect("program net")
        } else {
            fnet(&curve, &[b1.clone(), b2.clone()], &target, &mut meter).expect("net")
        };
        let mut rng = FSeeded(1234);
        let pts1 = b1.enumerate(&curve, &mut rng, &mut meter).unwrap();
        let pts2 = b2.enumerate(&curve, &mut rng, &mut meter).unwrap();
        let mut brute = FBig::from(0u8);
        for x in &pts1 {
            for y in &pts2 {
                if &curve.add(x, y, &mut meter).unwrap() == &target {
                    brute += 1u8;
                }
            }
        }
        let mut counter = FCounter::default();
        let count = counter.count(&net, &vec![None; net.edge_count()], &mut meter).unwrap();
        println!(
            "   bits={bits} simple={simple} derived={derived} program={program} scalar={scalar} |F1|={} |F2|={} brute={} network={} agree={} backend={:?}",
            pts1.len(), pts2.len(), brute, count, count == brute,
            counter.statistics.last_backend
        );
    }
}

pub fn gadget_cost(p: u128) {
    use hashed_ecdl_tensor_fast::encoding::Encoder;
    let (coeff, r, (gx, gy)) = prime_order_curve(p).unwrap();
    let mut meter = Meter::new(Limits { steps: 2_000_000_000_000, vector: 64_000_000, bits: 1_000_000 });
    let curve = Curve::new(big(p), big(0), big(coeff), big(r), Point::Affine { x: big(gx), y: big(gy) }, &mut meter).unwrap();
    let width = curve.modulus().bits() as usize;
    let mk_meter = || Meter::new(Limits { steps: 2_000_000_000_000, vector: 64_000_000, bits: 1_000_000 });

    let mut e = Encoder::new();
    let x = e.input_word(width, &meter).unwrap();
    let y = e.input_word(width, &meter).unwrap();
    e.modular_multiply(&x, &y, curve.modulus(), &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   modular_multiply edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let x = e.input_word(width, &meter).unwrap();
    let y = e.input_word(width, &meter).unwrap();
    e.modular_add(&x, &y, curve.modulus(), &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   modular_add      edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let a = e.input_point(&curve, false, &meter).unwrap();
    let b = e.input_point(&curve, false, &meter).unwrap();
    e.point_add(&curve, &a, &b, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   point_add        edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let _pt = e.input_point(&curve, false, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   input_point only edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let pt = e.input_point(&curve, false, &meter).unwrap();
    e.valid_point_affine(&curve, &pt, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   input+valid      edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let pt = e.input_point(&curve, false, &meter).unwrap();
    let s2 = { let mut c = 2*p+1; while !is_prime(c) { c += 2; } c };
    e.hash_member(&curve, &pt, &big(s2), &big(1), &big(0), &big(8), &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   input+hash(simple) edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let pt = e.input_point(&curve, false, &meter).unwrap();
    e.hash_member(&curve, &pt, &big(s2), &big(1), &big(0), &big(8), &meter).unwrap();
    let _ = e.valid_point_affine(&curve, &pt, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   input+hash+valid edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let x = e.input_word(width, &meter).unwrap();
    let qw = Encoder::constant(width, curve.modulus()).unwrap();
    e.less_than(&x, &qw, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   less_than        edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let x = e.input_word(width, &meter).unwrap();
    let y = e.input_word(width, &meter).unwrap();
    let g = e.input_word(1, &meter).unwrap();
    e.gated_equal(&g, &x, &y, &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   gated_equal      edges={} degree={}", net.edge_count(), net.maximum_degree());

    let mut e = Encoder::new();
    let pt = e.input_point(&curve, false, &meter).unwrap();
    let s = {
        let mut cand = 2 * p + 1;
        while !is_prime(cand) { cand += 2; }
        cand
    };
    e.hash_member(&curve, &pt, &big(s), &big(3), &big(5), &big(8), &meter).unwrap();
    let net = e.finish(&meter).unwrap();
    println!("   hash_member      edges={} degree={}", net.edge_count(), net.maximum_degree());
    let _ = mk_meter;
}
