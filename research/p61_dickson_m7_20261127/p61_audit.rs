//! Native exact replay for the public P-61 (M61-1) test curve.
//!
//! Build: rustc -O -o p61_audit p61_audit.rs
//! Run:   ./p61_audit
//! Hash:  ./p61_audit --pair-x-u64be | sha256sum
//!
//! This enumerates the D_1024(x)=0 factor base and every unoriented
//! two-leaf x-chart. It does not solve a target discrete logarithm or
//! implement a seven-sum decomposition oracle.

use std::collections::HashSet;
use std::io::{self, Write};

const P: u64 = (1u64 << 61) - 1;
const A: u64 = P - 3;
const B: u64 = 0x54aa66b458b77ec;
const R: u64 = 2_305_843_010_027_268_529;
const G: Point = Point {
    x: 0x112c917afe3d4fd3,
    y: 0x0ef721929b08bfaa,
};
const Q: usize = 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Fp2(u64, u64); // i^2=-1, valid because P is 3 mod 4

#[inline]
fn add(a: u64, b: u64) -> u64 {
    let sum = a + b; // both < 2^61, so no u64 overflow
    if sum >= P {
        sum - P
    } else {
        sum
    }
}

#[inline]
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + P - b
    }
}

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn pow(mut x: u64, mut n: u64) -> u64 {
    let mut acc = 1;
    while n != 0 {
        if n & 1 != 0 {
            acc = mul(acc, x);
        }
        x = mul(x, x);
        n >>= 1;
    }
    acc
}

fn inv(x: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, P - 2)
}

fn rhs(x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(A, x)), B)
}

fn on_curve(p: Point) -> bool {
    mul(p.y, p.y) == rhs(p.x)
}

fn ec_add(u: Option<Point>, v: Option<Point>) -> Option<Point> {
    let (Some(u), Some(v)) = (u, v) else {
        return u.or(v);
    };
    if u.x == v.x && add(u.y, v.y) == 0 {
        return None;
    }
    let slope = if u.x == v.x {
        mul(add(mul(3, mul(u.x, u.x)), A), inv(add(u.y, u.y)))
    } else {
        mul(sub(v.y, u.y), inv(sub(v.x, u.x)))
    };
    let x = sub(sub(mul(slope, slope), u.x), v.x);
    let y = sub(mul(slope, sub(u.x, x)), u.y);
    Some(Point { x, y })
}

fn ec_mul(mut n: u64, mut p: Point) -> Option<Point> {
    let mut acc = None;
    while n != 0 {
        if n & 1 != 0 {
            acc = ec_add(acc, Some(p));
        }
        p = ec_add(Some(p), Some(p)).expect("generator multiple has nonzero double");
        n >>= 1;
    }
    acc
}

fn pow_mod(mut x: u64, mut n: u64, modulus: u64) -> u64 {
    let mut acc = 1;
    while n != 0 {
        if n & 1 != 0 {
            acc = ((acc as u128 * x as u128) % modulus as u128) as u64;
        }
        x = ((x as u128 * x as u128) % modulus as u128) as u64;
        n >>= 1;
    }
    acc
}

// Deterministic Miller-Rabin witnesses for all n < 2^64.
fn is_prime64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % q == 0 {
            return n == q;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'witness: for base in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        let a = base % n;
        if a == 0 {
            continue;
        }
        let mut x = pow_mod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let (mut lo, mut hi) = (0u64, 1u64 << 32);
    while lo + 1 < hi {
        let mid = (lo + hi) / 2;
        if (mid as u128) * (mid as u128) <= n as u128 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn fp2_mul(x: Fp2, y: Fp2) -> Fp2 {
    Fp2(
        sub(mul(x.0, y.0), mul(x.1, y.1)),
        add(mul(x.0, y.1), mul(x.1, y.0)),
    )
}

fn fp2_pow(mut x: Fp2, mut n: u64) -> Fp2 {
    let mut acc = Fp2(1, 0);
    while n != 0 {
        if n & 1 != 0 {
            acc = fp2_mul(acc, x);
        }
        x = fp2_mul(x, x);
        n >>= 1;
    }
    acc
}

fn norm_one_generator() -> Fp2 {
    assert_eq!(P % 4, 3);
    assert_eq!(P + 1, 1u64 << 61);
    for a in 1..10000u64 {
        let a2 = mul(a, a);
        let den = inv(add(a2, 1));
        let z = Fp2(mul(sub(a2, 1), den), mul(mul(2, a), den));
        assert_eq!(add(mul(z.0, z.0), mul(z.1, z.1)), 1);
        if fp2_pow(z, (P + 1) / 2) != Fp2(1, 0) {
            return z; // the norm-one group has order 2^61
        }
    }
    panic!("no full-order norm-one element found");
}

fn s3(u: u64, v: u64, w: u64) -> u64 {
    // S_3(u,v,w) for y^2=x^3+Ax+B, reduced modulo P throughout.
    let uv = mul(u, v);
    let u_plus_v = add(u, v);
    let t1 = mul(mul(sub(u, v), sub(u, v)), mul(w, w));
    let t2 = mul(mul(2, add(mul(u_plus_v, add(uv, A)), mul(2, B))), w);
    let t3 = mul(sub(uv, A), sub(uv, A));
    let t4 = mul(mul(4, B), u_plus_v);
    sub(add(sub(t1, t2), t3), t4)
}

fn choose_u128(n: u128, k: u128) -> u128 {
    let mut value = 1u128;
    for i in 1..=k {
        value = value.checked_mul(n + 1 - i).expect("binomial overflow") / i;
    }
    value
}

fn main() -> io::Result<()> {
    assert!(is_prime64(P) && is_prime64(R));
    assert!(on_curve(G));
    assert_eq!(ec_mul(R, G), None);
    let h = 2 * isqrt(P) + 2;
    assert!(P + 1 - h < R && R < P + 1 + h && P + 1 + h < 2 * R);

    let torus_generator = norm_one_generator();
    let z = fp2_pow(torus_generator, (P + 1) / (4 * Q as u64));
    assert_eq!(fp2_pow(z, 4 * Q as u64), Fp2(1, 0));
    assert_ne!(fp2_pow(z, 2 * Q as u64), Fp2(1, 0));
    let step = fp2_mul(z, z);
    let mut w = z;
    let mut root_x = HashSet::with_capacity(Q);
    let mut leaves = Vec::with_capacity(Q);
    for _ in 0..Q {
        let x = add(w.0, w.0); // w + w^-1 = 2 Re(w)
        assert_ne!(x, 0);
        assert!(root_x.insert(x));
        let mut d = x;
        for _ in 0..10 {
            d = sub(mul(d, d), 2);
        }
        assert_eq!(d, 0); // D_1024(x)=0
        let y2 = rhs(x);
        if y2 == 0 || pow(y2, (P - 1) / 2) == 1 {
            let y = pow(y2, (P + 1) / 4);
            let point = Point { x, y };
            assert!(on_curve(point));
            leaves.push(point);
        }
        w = fp2_mul(w, step);
    }
    assert_eq!(root_x.len(), Q);
    assert_eq!(leaves.len(), 486);

    let mut pair_x = HashSet::with_capacity(leaves.len() * leaves.len());
    let mut charts = 0usize;
    let mut zero_charts = 0usize;
    let mut duplicate_charts = 0usize;
    for (i, first) in leaves.iter().enumerate() {
        for (j, second) in leaves.iter().enumerate().skip(i) {
            for sign in 0..(if i == j { 1 } else { 2 }) {
                charts += 1;
                let term = Point {
                    x: second.x,
                    y: if sign == 0 {
                        second.y
                    } else {
                        sub(0, second.y)
                    },
                };
                let Some(sum) = ec_add(Some(*first), Some(term)) else {
                    zero_charts += 1;
                    continue;
                };
                assert!(on_curve(sum));
                assert_eq!(s3(first.x, second.x, sum.x), 0);
                if !pair_x.insert(sum.x) {
                    duplicate_charts += 1;
                }
            }
        }
    }
    assert_eq!(charts, leaves.len() * leaves.len());
    assert_eq!((zero_charts, duplicate_charts), (0, 0));
    assert_eq!(pair_x.len(), 236_196);

    let root_intersection = pair_x.intersection(&root_x).count();
    let negative_pairs = pair_x
        .iter()
        .filter(|&&x| pair_x.contains(&sub(0, x)))
        .count()
        / 2;
    let d2_image: HashSet<u64> = pair_x.iter().map(|&x| sub(mul(x, x), 2)).collect();
    assert_eq!(
        (root_intersection, negative_pairs, d2_image.len()),
        (0, 0, 236_196)
    );

    // Every signed seven-multiset maps to one group target. For independent
    // uniform nonidentity queries, this bounds the mean number of returned
    // seven-sum relations per query even if every witness is returned.
    let signed_base_size = 2 * leaves.len();
    let signed_seven_multisets = choose_u128((signed_base_size + 6) as u128, 7);
    let expected_query_lower_num = (leaves.len() as u128 + 1) * (R as u128 - 1);
    let expected_query_lower_den = signed_seven_multisets;
    let full_pair_scan_lower_num = expected_query_lower_num * pair_x.len() as u128;
    let unrestricted_seven_sum_support_lower = 3 * (2 * pair_x.len()) + signed_base_size;

    // Evaluate (but do not prove) the proposed random-model coverage bound.
    // q=2^10 and m=7 let us cancel 2^18 before multiplying, so u128 suffices.
    let q = Q as u128;
    let p_minus_one = (P - 1) as u128;
    let kappa = 4u128;
    let theta = 2_214_051_840u128; // 13!! * 4^7
    let d_prime = 53_248u128; // 13 * 2^12
    let q7 = q.pow(7);
    let c = 2 * 4u128.pow(7) * d_prime;
    let covered_num = (q7 / (1 << 18)) * p_minus_one;
    let covered_den = theta * p_minus_one + c * q7;
    let generic_covered_targets_at_least = covered_num.div_ceil(covered_den);
    let sqrt_upper = (isqrt(P) + 1) as u128;
    let variance_num = q * p_minus_one + q * (q - 1) * (11 * sqrt_upper + 5);
    let good_model_prob_den = kappa * q * q * p_minus_one;
    let good_model_prob_num = (kappa - 1) * q * q * p_minus_one - 4 * kappa * variance_num;
    assert_eq!(generic_covered_targets_at_least, 5029);
    assert!(good_model_prob_num > 0);

    if std::env::args().nth(1).as_deref() == Some("--pair-x-u64be") {
        let mut sorted: Vec<u64> = pair_x.into_iter().collect();
        sorted.sort_unstable();
        let mut out = io::stdout().lock();
        for x in sorted {
            out.write_all(&x.to_be_bytes())?;
        }
        return Ok(());
    }
    println!("{{\"curve\":\"P-61\",\"p\":{P},\"group_order\":{R},\"dickson_degree\":{Q},\"dickson_root_count\":{},\"lifted_leaf_abscissae\":{},\"unoriented_pair_charts\":{charts},\"zero_pair_charts\":{zero_charts},\"duplicate_pair_x_charts\":{duplicate_charts},\"distinct_pair_abscissae_D\":{},\"pair_x_intersect_dickson_roots\":{root_intersection},\"negative_pair_x_pairs\":{negative_pairs},\"D2_image_size\":{},\"signed_seven_multisets\":\"{signed_seven_multisets}\",\"uniform_query_lower_num\":\"{expected_query_lower_num}\",\"uniform_query_lower_den\":\"{expected_query_lower_den}\",\"full_pair_scan_lower_num\":\"{full_pair_scan_lower_num}\",\"unrestricted_seven_sum_support_lower\":{unrestricted_seven_sum_support_lower},\"proposed_generic_covered_targets_at_least\":{generic_covered_targets_at_least},\"proposed_good_model_prob_num\":\"{good_model_prob_num}\",\"proposed_good_model_prob_den\":\"{good_model_prob_den}\"}}", root_x.len(), leaves.len(), pair_x.len(), d2_image.len());
    Ok(())
}
