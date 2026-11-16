mod fp61;
mod fp61_fast;

use hashed_ecdl_tensor::arithmetic::Seeded;
use hashed_ecdl_tensor::contraction::Counter;
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

fn is_prime_u128(n: u128) -> bool {
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

fn next_prime_u128(mut n: u128) -> u128 {
    if n <= 2 {
        return 2;
    }
    if n % 2 == 0 {
        n += 1;
    }
    while !is_prime_u128(n) {
        n += 2;
    }
    n
}

fn sqrt_mod(p: u128, n: u128) -> Option<u128> {
    if n == 0 {
        return Some(0);
    }
    let r = pow_mod(p, n, (p + 1) / 4);
    (mul_mod(p, r, r) == n % p).then_some(r)
}

pub(crate) fn find_curve(bits: u32) -> Option<(u128, u128, u128, u128, (u128, u128))> {
    let mut p = next_prime_u128(1u128 << (bits - 1));
    for _ in 0..80 {
        if p % 4 == 3 {
            for a in 1..6u128 {
                for b in 1..6u128 {
                    if (4 * a * a * a + 27 * b * b) % p == 0 {
                        continue;
                    }
                    let mut count = 1u128;
                    let mut gen = None;
                    for x in 0..p {
                        let rhs = (mul_mod(p, mul_mod(p, x, x), x) + mul_mod(p, a, x) + b) % p;
                        if rhs == 0 {
                            count += 1;
                        } else if pow_mod(p, rhs, (p - 1) / 2) == 1 {
                            count += 2;
                            if gen.is_none() {
                                gen = Some((x, sqrt_mod(p, rhs).unwrap()));
                            }
                        }
                    }
                    if is_prime_u128(count) && count > 5 && gen.is_some() {
                        return Some((p, a, b, count, gen.unwrap()));
                    }
                }
            }
        }
        p = next_prime_u128(p + 2);
    }
    None
}

pub(crate) fn big(x: u128) -> BigUint {
    BigUint::from(x)
}

pub(crate) fn point(xy: Option<(u128, u128)>) -> Point {
    match xy {
        None => Point::Infinity,
        Some((x, y)) => Point::Affine { x: big(x), y: big(y) },
    }
}

pub(crate) fn limits() -> Limits {
    Limits {
        steps: 5_000_000_000,
        vector: 8_000_000,
        bits: 4_000_000,
    }
}

fn bases(curve: &Curve, p: u128, meter: &mut Meter) -> (FactorBase, FactorBase) {
    let s = next_prime_u128(2 * p);
    let tau = big(8);
    let b1 = FactorBase::new(
        curve,
        big(s),
        tau.clone(),
        HashSeed { alpha: big(3), beta: big(5) },
        meter,
    )
    .expect("base 1");
    let b2 = FactorBase::new(
        curve,
        big(s),
        tau,
        HashSeed { alpha: big(7), beta: big(11) },
        meter,
    )
    .expect("base 2");
    (b1, b2)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--p61") {
        fp61::run((1u128 << 61) - 1);
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "--gadget-cost") {
        let p: u128 = args[pos + 1].parse().expect("prime");
        fp61_fast::gadget_cost(p);
        return;
    }
    if args.iter().any(|a| a == "--check-fast") {
        for simple in [false, true] {
            fp61_fast::check_small(simple, false, false, true);
        }
        for simple in [false, true] {
            for derived in [false, true] {
                for program in [false, true] {
                    fp61_fast::check_small(simple, derived, program, false);
                }
            }
        }
        return;
    }
    if args.iter().any(|a| a == "--p61-fast") {
        fp61_fast::run((1u128 << 61) - 1, false, false, false, false);
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "--fp-fast") {
        let p: u128 = args[pos + 1].parse().expect("prime in decimal");
        let simple = args.iter().any(|a| a == "--simple");
        let program = args.iter().any(|a| a == "--program");
        let derived = args.iter().any(|a| a == "--derived");
        let scalar = args.iter().any(|a| a == "--scalar");
        fp61_fast::run(p, simple, program, derived, scalar);
        return;
    }
    if let Some(pos) = args.iter().position(|a| a == "--fp") {
        let p: u128 = args[pos + 1].parse().expect("prime in decimal");
        fp61::run(p);
        return;
    }
    let eccp131 = args.iter().any(|a| a == "--eccp131");

    println!("=== decomposition network size (Lemma 11, slots=2)");
    println!("bits  edges      vertices   max_degree  interface");
    let mut built = Vec::new();
    for bits in [3u32, 5, 7, 9, 11, 13, 16, 17] {
        let Some((p, a, b, r, g)) = find_curve(bits) else {
            println!("{bits:<5} no prime-order curve found");
            continue;
        };
        let mut meter = Meter::new(limits());
        let curve = match Curve::new(big(p), big(a), big(b), big(r), point(Some(g)), &mut meter) {
            Ok(c) => c,
            Err(e) => {
                println!("{bits:<5} curve rejected: {e:?}");
                continue;
            }
        };
        let (b1, b2) = bases(&curve, p, &mut meter);
        let target = curve.mul(&point(Some(g)), &big(2), &mut meter).expect("target");
        let net = match decomposition_network(&curve, &[b1, b2], &target, &mut meter) {
            Ok(n) => n,
            Err(e) => {
                println!("{bits:<5} network rejected: {e:?}");
                continue;
            }
        };
        println!(
            "{bits:<5} {:<10} {:<10} {:<11} {}",
            net.edge_count(),
            net.vertex_count(),
            net.maximum_degree(),
            net.interface().len()
        );
        built.push((bits, curve, target, net));
    }

    println!();
    println!("=== exact count vs brute force (crate's own contains/add semantics)");
    println!("bits  |F1|  |F2|  brute  network  agree  backend  steps");
    for (bits, curve, target, net) in &built {
        if *bits > 5 {
            continue;
        }
        let mut meter = Meter::new(limits());
        let (p, _, _, _, _) = find_curve(*bits).unwrap();
        let (b1, b2) = bases(curve, p, &mut meter);
        let mut rng = Seeded(1234);
        let pts1 = b1.enumerate(curve, &mut rng, &mut meter).unwrap();
        let pts2 = b2.enumerate(curve, &mut rng, &mut meter).unwrap();

        let mut brute = BigUint::from(0u8);
        for x in &pts1 {
            for y in &pts2 {
                if &curve.add(x, y, &mut meter).unwrap() == target {
                    brute += 1u8;
                }
            }
        }

        let mut counter = Counter::default();
        let count = counter
            .count(net, &vec![None; net.edge_count()], &mut meter)
            .expect("count");
        println!(
            "{bits:<5} {:<5} {:<5} {:<6} {:<8}{}  {:?}  {}",
            pts1.len(),
            pts2.len(),
            brute,
            count,
            if count == brute { "yes" } else { "NO" },
            counter.statistics.last_backend,
            meter.used
        );
    }

    if eccp131 {
        println!();
        println!("=== ECCp-131 decomposition network");
        let raw = std::fs::read_to_string(
            "../../../crates/hashed-ecdl-tensor/examples/eccp131.json",
        )
        .expect("eccp131.json");
        let v: serde_json::Value = serde_json::from_str(&raw).unwrap();
        let get = |k: &str| -> BigUint { v[k].as_str().unwrap().parse().unwrap() };
        let parse = |x: &serde_json::Value| -> BigUint { x.as_str().unwrap().parse().unwrap() };
        let p = get("p");
        let a = get("a");
        let b = get("b");
        let r = get("r");
        let gx = parse(&v["generator"][0]);
        let gy = parse(&v["generator"][1]);
        let tx = parse(&v["target"][0]);
        let ty = parse(&v["target"][1]);
        let mut meter = Meter::new(Limits {
            steps: 2_000_000_000_000,
            vector: 64_000_000,
            bits: 1_000_000,
        });
        let curve = Curve::new(
            p.clone(),
            a,
            b,
            r,
            Point::Affine { x: gx, y: gy },
            &mut meter,
        )
        .expect("curve");
        let target = Point::Affine { x: tx, y: ty };
        let s = {
            let mut cand = (&p << 1u8) + 1u8;
            while !big_is_probable_prime(&cand) {
                cand += 1u8;
            }
            cand
        };
        let mut bs = Vec::new();
        for k in 0..2u8 {
            bs.push(
                FactorBase::new(
                    &curve,
                    s.clone(),
                    BigUint::from(8u8),
                    HashSeed {
                        alpha: BigUint::from(3u8 + k),
                        beta: BigUint::from(5u8),
                    },
                    &mut meter,
                )
                .expect("base"),
            );
        }
        let t0 = std::time::Instant::now();
        let net = decomposition_network(&curve, &bs, &target, &mut meter).expect("network");
        println!(
            "   edges={} vertices={} max_degree={} interface={} build={:?}",
            net.edge_count(),
            net.vertex_count(),
            net.maximum_degree(),
            net.interface().len(),
            t0.elapsed()
        );
    }
}

fn big_is_probable_prime(n: &BigUint) -> bool {
    use num_bigint::BigUint as B;
    if n < &B::from(2u8) {
        return false;
    }
    for p in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == B::from(0u8) {
            return *n == B::from(p);
        }
    }
    let one = B::from(1u8);
    let n_minus_one = n - &one;
    let mut d = n_minus_one.clone();
    let mut s = 0u32;
    while !d.bit(0) {
        d >>= 1;
        s += 1;
    }
    for a in [2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = B::from(a).modpow(&d, n);
        if x == one || x == n_minus_one {
            continue;
        }
        let mut ok = false;
        for _ in 1..s {
            x = (&x * &x) % n;
            if x == n_minus_one {
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
