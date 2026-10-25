// Explicit curves over the "other special shapes" of prime: Proth/FFT primes
// c*2^e + 1, cyclotomic-value primes Phi_d(m), primes with smooth p - 1 or
// p + 1, and a generic random-prime control. For each prime we use the CM
// method with class-number-one discriminants D to find a curve y^2 = x^3 +
// ax + b over F_p whose group order is r*h with r prime and h in {1,2,3,4,6,8},
// and CERTIFY it by the group law: Q = [h]P != O, [r]Q = O, r prime and
// r > 4 sqrt(p). (Q has order exactly r, so r divides #E; Hasse's interval
// has width 4 sqrt(p) < r, so #E is the unique multiple, namely r*h.) Then we
// compute the embedding degree k = ord_r(p) up to a cap.
//
//   cargo run --release --example special_prime_curve_constructions -- \
//       --bits 192 --per-family 2 --seed 20261023
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};

fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

fn rand_bits(s: &mut u64, bits: u32) -> BigUint {
    let mut v = BigUint::zero();
    let mut left = bits;
    while left > 0 {
        let take = left.min(64);
        let w = splitmix(s) & if take == 64 { u64::MAX } else { (1u64 << take) - 1 };
        v = (v << take) | BigUint::from(w);
        left -= take;
    }
    v
}

const SMALL: [u32; 16] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];

fn is_prime(n: &BigUint) -> bool {
    if n < &BigUint::from(2u32) {
        return false;
    }
    for p in SMALL {
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
    'w: for a in SMALL {
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

fn is_square_mod(a: &BigUint, p: &BigUint) -> bool {
    if a.is_zero() {
        return true;
    }
    a.modpow(&((p - 1u32) >> 1), p) == BigUint::one()
}

fn sqrt_mod(a: &BigUint, p: &BigUint) -> Option<BigUint> {
    let a = a % p;
    if a.is_zero() {
        return Some(a);
    }
    if !is_square_mod(&a, p) {
        return None;
    }
    let pm1 = p - 1u32;
    let mut q = pm1.clone();
    let mut s = 0u32;
    while (&q & BigUint::one()).is_zero() {
        q >>= 1;
        s += 1;
    }
    let mut z = BigUint::from(2u32);
    while is_square_mod(&z, p) {
        z += 1u32;
    }
    let mut m = s;
    let mut c = z.modpow(&q, p);
    let mut t = a.modpow(&q, p);
    let mut r = a.modpow(&((&q + 1u32) >> 1), p);
    while t != BigUint::one() {
        let mut i = 0;
        let mut tt = t.clone();
        while tt != BigUint::one() {
            tt = (&tt * &tt) % p;
            i += 1;
            if i == m {
                return None;
            }
        }
        let b = c.modpow(&(BigUint::one() << (m - i - 1)), p);
        m = i;
        c = (&b * &b) % p;
        t = (&t * &c) % p;
        r = (&r * &b) % p;
    }
    Some(r)
}

fn isqrt(n: &BigUint) -> BigUint {
    n.sqrt()
}

// 4p = x^2 + D y^2 by Cornacchia, if solvable.
fn cornacchia_4p(d: u64, p: &BigUint) -> Option<(BigUint, BigUint)> {
    let m: BigUint = p << 2usize;
    let minus_d = (p - (BigUint::from(d) % p)) % p;
    let s = sqrt_mod(&minus_d, p)?;
    let mut r0 = None;
    for root in [s.clone(), p - &s] {
        for k in 0u32..4 {
            let cand = &root + p * k;
            if ((&cand * &cand + BigUint::from(d)) % &m).is_zero() {
                r0 = Some(cand);
                break;
            }
        }
        if r0.is_some() {
            break;
        }
    }
    let mut r0 = r0?;
    if r0 > (&m >> 1) {
        r0 = &m - &r0;
    }
    let lim = isqrt(&m);
    let (mut a, mut b) = (m.clone(), r0);
    while b > lim {
        let t = &a % &b;
        a = b;
        b = t;
    }
    let x = b;
    let rest = &m - &x * &x;
    let dd = BigUint::from(d);
    if !(&rest % &dd).is_zero() {
        return None;
    }
    let y2 = rest / dd;
    let y = isqrt(&y2);
    if &y * &y == y2 {
        Some((x, y))
    } else {
        None
    }
}

// ---- affine curve arithmetic over F_p ----
type Pt = Option<(BigUint, BigUint)>;

fn inv(a: &BigUint, p: &BigUint) -> BigUint {
    a.modpow(&(p - 2u32), p)
}

fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    ((a + p) - (b % p)) % p
}

fn add(p1: &Pt, p2: &Pt, a: &BigUint, p: &BigUint) -> Pt {
    let (x1, y1) = match p1 {
        None => return p2.clone(),
        Some(v) => v,
    };
    let (x2, y2) = match p2 {
        None => return p1.clone(),
        Some(v) => v,
    };
    let lam;
    if x1 == x2 {
        if ((y1 + y2) % p).is_zero() {
            return None;
        }
        lam = ((BigUint::from(3u32) * x1 * x1 + a) % p) * inv(&((y1 << 1) % p), p) % p;
    } else {
        lam = sub(y2, y1, p) * inv(&sub(x2, x1, p), p) % p;
    }
    let x3 = sub(&sub(&(&lam * &lam % p), x1, p), x2, p);
    let y3 = sub(&(&lam * sub(x1, &x3, p) % p), y1, p);
    Some((x3, y3))
}

fn mul(k: &BigUint, pt: &Pt, a: &BigUint, p: &BigUint) -> Pt {
    let mut acc: Pt = None;
    let mut base = pt.clone();
    let mut k = k.clone();
    while !k.is_zero() {
        if (&k & BigUint::one()) == BigUint::one() {
            acc = add(&acc, &base, a, p);
        }
        base = add(&base, &base, a, p);
        k >>= 1;
    }
    acc
}

fn rand_point(a: &BigUint, b: &BigUint, p: &BigUint, s: &mut u64) -> (BigUint, BigUint) {
    loop {
        let x = rand_bits(s, p.bits() as u32 + 8) % p;
        let rhs = (&x * &x % p * &x + a * &x + b) % p;
        if let Some(y) = sqrt_mod(&rhs, p) {
            if !y.is_zero() {
                return (x, y);
            }
        }
    }
}

fn mod_signed(j: i128, p: &BigUint) -> BigUint {
    let jj = BigUint::from(j.unsigned_abs()) % p;
    if j < 0 {
        (p - jj) % p
    } else {
        jj
    }
}

// (D, j-invariant) for the class-number-one discriminants.
const CM: [(u64, i128); 9] = [
    (3, 0),
    (4, 1728),
    (7, -3375),
    (8, 8000),
    (11, -32768),
    (19, -884736),
    (43, -884736000),
    (67, -147197952000),
    (163, -262537412640768000),
];

struct Cert {
    d: u64,
    h: u32,
    r: BigUint,
    a: BigUint,
    b: BigUint,
    px: BigUint,
    py: BigUint,
    ok: bool,
    k: Option<u32>,
}

fn embedding_degree_capped(p: &BigUint, r: &BigUint, cap: u32) -> Option<u32> {
    let mut x = BigUint::one();
    for k in 1..=cap {
        x = (&x * p) % r;
        if x == BigUint::one() {
            return Some(k);
        }
    }
    None
}

fn candidate_orders(d: u64, p: &BigUint, x: &BigUint, y: &BigUint) -> Vec<BigUint> {
    let bx = BigInt::from(x.clone());
    let by = BigInt::from(y.clone());
    let mut traces: Vec<BigInt> = vec![bx.clone(), -bx.clone()];
    if d == 3 {
        for sgn in [1i32, -1] {
            let t = (&bx + BigInt::from(3 * sgn) * &by) / BigInt::from(2);
            traces.push(t.clone());
            traces.push(-t);
        }
    }
    if d == 4 {
        let t = BigInt::from(2) * &by;
        traces.push(t.clone());
        traces.push(-t);
    }
    let bp = BigInt::from(p.clone());
    traces
        .into_iter()
        .filter_map(|t| (&bp + BigInt::one() - t).to_biguint())
        .collect()
}

fn construct(p: &BigUint, s: &mut u64, cap: u32) -> Option<Cert> {
    let four_sqrt = (isqrt(p) + 1u32) << 2;
    for &(d, j) in CM.iter() {
        let Some((x, y)) = cornacchia_4p(d, p) else { continue };
        for n in candidate_orders(d, p, &x, &y) {
            // N = r * h with r prime
            let mut found = None;
            for h in [1u32, 2, 3, 4, 6, 8] {
                let (q, rem) = n.div_rem(&BigUint::from(h));
                if rem.is_zero() && q > four_sqrt && is_prime(&q) {
                    found = Some((h, q));
                    break;
                }
            }
            let Some((h, r)) = found else { continue };
            // find the twist whose group has exactly this order
            for _attempt in 0..120 {
                let (a, b) = match d {
                    3 => (BigUint::zero(), rand_bits(s, p.bits() as u32 + 8) % p),
                    4 => (rand_bits(s, p.bits() as u32 + 8) % p, BigUint::zero()),
                    _ => {
                        let jm = mod_signed(j, p);
                        let c = sub(&BigUint::from(1728u32), &jm, p);
                        let a0 = BigUint::from(3u32) * &jm % p * &c % p;
                        let b0 = BigUint::from(2u32) * &jm % p * &c % p * &c % p;
                        let u = (rand_bits(s, p.bits() as u32 + 8) % (p - 1u32)) + 1u32;
                        (a0 * &u % p * &u % p, b0 * &u % p * &u % p * &u % p)
                    }
                };
                if (BigUint::from(4u32) * &a % p * &a % p * &a % p
                    + BigUint::from(27u32) * &b % p * &b % p)
                    .is_multiple_of(p)
                {
                    continue;
                }
                let (px, py) = rand_point(&a, &b, p, s);
                let pt = Some((px.clone(), py.clone()));
                let q = mul(&BigUint::from(h), &pt, &a, p);
                if q.is_none() {
                    continue;
                }
                if mul(&r, &q, &a, p).is_none() {
                    // certify with independent checks
                    let (qx, qy) = q.clone().unwrap();
                    let on_curve = (&qy * &qy) % p == (&qx * &qx % p * &qx + &a * &qx + &b) % p;
                    let ok = on_curve && is_prime(&r) && r > four_sqrt;
                    return Some(Cert {
                        d,
                        h,
                        r: r.clone(),
                        a,
                        b,
                        px: qx,
                        py: qy,
                        ok,
                        k: embedding_degree_capped(p, &r, cap),
                    });
                }
            }
        }
    }
    None
}

fn phi(d: u32, m: &BigUint) -> BigUint {
    let m2 = m * m;
    match d {
        3 => &m2 + m + 1u32,
        4 => &m2 + 1u32,
        5 => &m2 * &m2 + &m2 * m + &m2 + m + 1u32,
        6 => &m2 - m + 1u32,
        8 => &m2 * &m2 + 1u32,
        10 => &m2 * &m2 + &m2 - &m2 * m - m + 1u32,
        12 => &m2 * &m2 - &m2 + 1u32,
        _ => unreachable!(),
    }
}

fn first_primes(limit: u32) -> Vec<u32> {
    let mut sieve = vec![true; limit as usize + 1];
    let mut v = Vec::new();
    for i in 2..=limit as usize {
        if sieve[i] {
            v.push(i as u32);
            let mut j = i * i;
            while j <= limit as usize {
                sieve[j] = false;
                j += i;
            }
        }
    }
    v
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let get = |n: &str, d: u64| -> u64 {
        args.iter()
            .position(|a| a == n)
            .and_then(|i| args.get(i + 1)?.parse().ok())
            .unwrap_or(d)
    };
    let bits = get("--bits", 192) as u32;
    let per = get("--per-family", 2) as usize;
    let max_primes = get("--max-primes", 4000) as usize;
    let cap = get("--k-cap", 4000) as u32;
    let mut s = get("--seed", 20261023);
    let small_primes = first_primes(1 << 20);

    println!("family\tshape\tbits_p\tp_hex\tD\th\tbits_r\tk_up_to_cap\tcertified\ta_hex\tb_hex\tQ_x_hex\tQ_y_hex");
    let families: Vec<(&str, Option<u32>)> = vec![
        ("proth", None),
        ("phi_d", Some(3)),
        ("phi_d", Some(4)),
        ("phi_d", Some(5)),
        ("phi_d", Some(6)),
        ("phi_d", Some(8)),
        ("phi_d", Some(10)),
        ("phi_d", Some(12)),
        ("smooth_p_minus_1", None),
        ("smooth_p_plus_1", None),
        ("generic_control", None),
    ];
    for (fam, dd) in families {
        let (mut hits, mut tried, mut cm_none) = (0usize, 0usize, 0usize);
        while hits < per && tried < max_primes {
            // draw a prime of this shape
            let (p, shape) = match (fam, dd) {
                ("proth", _) => {
                    let e = bits - 14;
                    let c = (rand_bits(&mut s, 13) << 1usize) + BigUint::one() + (BigUint::one() << 13usize);
                    ((c.clone() << e) + 1u32, format!("{}*2^{}+1", c, e))
                }
                ("phi_d", Some(d)) => {
                    let deg = match d {
                        3 | 4 | 6 => 2,
                        _ => 4,
                    };
                    let m = rand_bits(&mut s, bits / deg) | (BigUint::one() << (bits / deg - 1));
                    (phi(d, &m), format!("Phi_{}({})", d, m))
                }
                ("smooth_p_minus_1", _) | ("smooth_p_plus_1", _) => {
                    let mut prod = BigUint::from(2u32);
                    let mut maxq = 0u32;
                    while prod.bits() < (bits - 1) as u64 {
                        let q = small_primes[(splitmix(&mut s) as usize) % small_primes.len()];
                        prod *= q;
                        maxq = maxq.max(q);
                    }
                    let p = if fam == "smooth_p_minus_1" { &prod + 1u32 } else { &prod - 1u32 };
                    (p, format!("{} smooth to {}", if fam == "smooth_p_minus_1" { "p-1" } else { "p+1" }, maxq))
                }
                _ => {
                    let c = rand_bits(&mut s, bits) | (BigUint::one() << (bits - 1)) | BigUint::one();
                    (c, "random".to_string())
                }
            };
            if !is_prime(&p) {
                continue;
            }
            tried += 1;
            match construct(&p, &mut s, cap) {
                Some(c) => {
                    let fam_name = if let Some(d) = dd { format!("{}_{}", fam, d) } else { fam.to_string() };
                    println!(
                        "{}\t{}\t{}\t0x{:x}\t{}\t{}\t{}\t{}\t{}\t0x{:x}\t0x{:x}\t0x{:x}\t0x{:x}",
                        fam_name,
                        shape,
                        p.bits(),
                        p,
                        c.d,
                        c.h,
                        c.r.bits(),
                        c.k.map(|k| k.to_string()).unwrap_or(format!(">{}", cap)),
                        if c.ok { "yes" } else { "NO" },
                        c.a,
                        c.b,
                        c.px,
                        c.py
                    );
                    hits += 1;
                }
                None => cm_none += 1,
            }
        }
        eprintln!(
            "{} {:?}: {} hits, {} primes of this shape tried, {} without a prime-order CM curve",
            fam, dd, hits, tried, cm_none
        );
    }
    let _ = (Sign::Plus, 0u32.to_f64());
}
