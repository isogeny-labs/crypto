//! Producer-side arithmetic falsifiers for GHS strong-rho leads R17--R33.
//!
//! This standalone program deliberately shares no implementation with
//! `strong_rho.rs`.  It checks algebra and representation contracts only; it
//! neither measures performance nor supplies leaderboard evidence.

type F3 = [u64; 3];

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct CoreCounts {
    fq_mul: u64,
    fq_small_div: u64,
}

#[inline]
fn add(a: u64, b: u64, q: u64) -> u64 {
    let sum = a + b;
    if sum >= q {
        sum - q
    } else {
        sum
    }
}

#[inline]
fn sub(a: u64, b: u64, q: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + q - b
    }
}

#[inline]
fn mul(a: u64, b: u64, q: u64) -> u64 {
    ((a as u128 * b as u128) % q as u128) as u64
}

fn div2(x: u64, q: u64) -> u64 {
    assert!(q & 1 == 1 && x < q);
    let lifted = if x & 1 == 0 { x } else { x + q };
    let out = lifted / 2;
    assert_eq!(add(out, out, q), x);
    out
}

fn div3(x: u64, q: u64) -> u64 {
    assert!(q % 3 != 0 && x < q);
    for k in 0..3 {
        let lifted = x + k * q;
        if lifted % 3 == 0 {
            let out = lifted / 3;
            assert_eq!(mul(out, 3, q), x);
            return out;
        }
    }
    unreachable!("one of x, x+q, x+2q is divisible by three")
}

fn schoolbook(a: F3, b: F3, q: u64, c: u64) -> F3 {
    let mut d = [0_u64; 5];
    for i in 0..3 {
        for j in 0..3 {
            d[i + j] = add(d[i + j], mul(a[i], b[j], q), q);
        }
    }
    [
        add(d[0], mul(c, d[3], q), q),
        add(d[1], mul(c, d[4], q), q),
        d[2],
    ]
}

fn mul5(a: F3, b: F3, q: u64, c: u64) -> Option<(F3, CoreCounts)> {
    if q < 5 || q % 2 == 0 || q % 3 == 0 {
        return None;
    }
    let mut counts = CoreCounts::default();
    let mut fm = |x: u64, y: u64| {
        counts.fq_mul += 1;
        mul(x, y, q)
    };

    let v0 = fm(a[0], b[0]);
    let v1 = fm(
        add(add(a[0], a[1], q), a[2], q),
        add(add(b[0], b[1], q), b[2], q),
    );
    let vm1 = fm(
        add(sub(a[0], a[1], q), a[2], q),
        add(sub(b[0], b[1], q), b[2], q),
    );
    let a_at_2 = add(add(a[0], add(a[1], a[1], q), q), mul(4 % q, a[2], q), q);
    let b_at_2 = add(add(b[0], add(b[1], b[1], q), q), mul(4 % q, b[2], q), q);
    let v2 = fm(a_at_2, b_at_2);
    let vinf = fm(a[2], b[2]);
    drop(fm);

    counts.fq_small_div += 1;
    let even = div2(add(v1, vm1, q), q);
    let d2 = sub(sub(even, v0, q), vinf, q);
    counts.fq_small_div += 1;
    let odd = div2(sub(v1, vm1, q), q);
    let four_d2 = mul(4 % q, d2, q);
    let sixteen_d4 = mul(16 % q, vinf, q);
    counts.fq_small_div += 1;
    let at2 = div2(sub(sub(sub(v2, v0, q), four_d2, q), sixteen_d4, q), q);
    counts.fq_small_div += 1;
    let d3 = div3(sub(at2, odd, q), q);
    let d1 = sub(odd, d3, q);
    Some((
        [add(v0, mul(c, d3, q), q), add(d1, mul(c, vinf, q), q), d2],
        counts,
    ))
}

fn square5(a: F3, q: u64, c: u64) -> (F3, u64) {
    let m0 = mul(a[0], a[0], q);
    let m2 = mul(a[2], a[2], q);
    let m01 = mul(a[0], a[1], q);
    let m12 = mul(a[1], a[2], q);
    let sum = add(add(a[0], a[1], q), a[2], q);
    let ms = mul(sum, sum, q);
    let d1 = add(m01, m01, q);
    let d3 = add(m12, m12, q);
    let d2 = sub(sub(sub(sub(ms, m0, q), m2, q), d1, q), d3, q);
    (
        [add(m0, mul(c, d3, q), q), add(d1, mul(c, m2, q), q), d2],
        5,
    )
}

fn egcd_inverse(a: u64, q: u64) -> u64 {
    let (mut old_r, mut r) = (q as i128, a as i128);
    let (mut old_t, mut t) = (0_i128, 1_i128);
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_t, t) = (t, old_t - quotient * t);
    }
    assert_eq!(old_r, 1);
    old_t.rem_euclid(q as i128) as u64
}

fn binary_inverse(a: u64, q: u64) -> u64 {
    assert!(a > 0 && a < q && q & 1 == 1);
    let (mut u, mut v) = (a, q);
    let (mut x1, mut x2) = (1_i128, 0_i128);
    while u != 1 && v != 1 {
        while u & 1 == 0 {
            u >>= 1;
            x1 = if x1 & 1 == 0 {
                x1 / 2
            } else {
                (x1 + q as i128) / 2
            };
        }
        while v & 1 == 0 {
            v >>= 1;
            x2 = if x2 & 1 == 0 {
                x2 / 2
            } else {
                (x2 + q as i128) / 2
            };
        }
        if u >= v {
            u -= v;
            x1 -= x2;
        } else {
            v -= u;
            x2 -= x1;
        }
    }
    let x = if u == 1 { x1 } else { x2 };
    x.rem_euclid(q as i128) as u64
}

fn f3_pow(mut base: F3, mut exponent: u64, q: u64, c: u64) -> F3 {
    let mut out = [1, 0, 0];
    while exponent != 0 {
        if exponent & 1 == 1 {
            out = schoolbook(out, base, q, c);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = schoolbook(base, base, q, c);
        }
    }
    out
}

fn adjugate_inverse(a: F3, q: u64, c: u64) -> F3 {
    assert_ne!(a, [0, 0, 0]);
    let adj0 = sub(mul(a[0], a[0], q), mul(c, mul(a[1], a[2], q), q), q);
    let adj1 = sub(mul(c, mul(a[2], a[2], q), q), mul(a[0], a[1], q), q);
    let adj2 = sub(mul(a[1], a[1], q), mul(a[0], a[2], q), q);
    let norm = add(
        mul(a[0], adj0, q),
        mul(c, add(mul(a[1], adj2, q), mul(a[2], adj1, q), q), q),
        q,
    );
    let ni = egcd_inverse(norm, q);
    [mul(adj0, ni, q), mul(adj1, ni, q), mul(adj2, ni, q)]
}

fn barrett_reciprocal(q: u64) -> u64 {
    ((u64::MAX as u128 + 1) / q as u128) as u64
}

fn barrett_reduce(q: u64, reciprocal: u64, x: u64) -> u64 {
    let quotient = ((x as u128 * reciprocal as u128) >> 64) as u64;
    let residual = x - quotient * q;
    if residual >= q {
        residual - q
    } else {
        residual
    }
}

fn branchless_add(a: u64, b: u64, q: u64) -> u64 {
    let sum = a + b;
    sum - q * u64::from(sum >= q)
}

fn branchless_sub(a: u64, b: u64, q: u64) -> u64 {
    a.wrapping_sub(b)
        .wrapping_add(q & 0_u64.wrapping_sub(u64::from(a < b)))
}

fn inverse_mod_2_32(q: u32) -> u32 {
    assert!(q & 1 == 1);
    let mut x = 1_u32;
    for _ in 0..5 {
        x = x.wrapping_mul(2_u32.wrapping_sub(q.wrapping_mul(x)));
    }
    assert_eq!(q.wrapping_mul(x), 1);
    x
}

fn montgomery_reduce(t: u64, q: u64, n_prime: u32) -> u64 {
    let m = (t as u32).wrapping_mul(n_prime) as u64;
    let u = ((t as u128 + m as u128 * q as u128) >> 32) as u64;
    if u >= q {
        u - q
    } else {
        u
    }
}

fn montgomery_encode(a: u64, q: u64) -> u64 {
    ((a as u128) << 32).rem_euclid(q as u128) as u64
}

fn montgomery_product(a: u64, b: u64, q: u64) -> u64 {
    let n_prime = inverse_mod_2_32(q as u32).wrapping_neg();
    let am = montgomery_encode(a, q);
    let bm = montgomery_encode(b, q);
    let product_m = montgomery_reduce(am * bm, q, n_prime);
    montgomery_reduce(product_m, q, n_prime)
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn element(index: u64, q: u64) -> F3 {
    [index % q, (index / q) % q, (index / q / q) % q]
}

fn main() {
    let mut exhaustive_pairs = 0_u64;
    let mut exhaustive_squares = 0_u64;
    for q in [5_u64, 7] {
        let element_count = q * q * q;
        for c in 1..q {
            for ai in 0..element_count {
                let a = element(ai, q);
                let (candidate_square, products) = square5(a, q, c);
                assert_eq!(candidate_square, schoolbook(a, a, q, c));
                assert_eq!(products, 5);
                exhaustive_squares += 1;
                for bi in 0..element_count {
                    let b = element(bi, q);
                    let (candidate, counts) = mul5(a, b, q, c).unwrap();
                    assert_eq!(candidate, schoolbook(a, b, q, c));
                    assert_eq!(
                        counts,
                        CoreCounts {
                            fq_mul: 5,
                            fq_small_div: 4
                        }
                    );
                    exhaustive_pairs += 1;
                }
            }
        }
    }
    println!(
        "mul5 PASS: {exhaustive_pairs} exhaustive pairs over q=5,7/all nonzero c; exact core count 5 fq_mul + 4 fq_small_div"
    );
    println!("square5 PASS: {exhaustive_squares} exhaustive elements over q=5,7/all nonzero c");

    let mut rng = 0x0ecb_e223_u64;
    let mut random_pairs = 0_u64;
    let mut random_cells = 0_u64;
    for q in [11_u64, 13, 223, 4_051, 65_537, (1_u64 << 31) - 1] {
        for c in [1_u64, 2, 16_u64.min(q - 1)] {
            random_cells += 1;
            for _ in 0..10_000 {
                let a = [
                    splitmix64(&mut rng) % q,
                    splitmix64(&mut rng) % q,
                    splitmix64(&mut rng) % q,
                ];
                let b = [
                    splitmix64(&mut rng) % q,
                    splitmix64(&mut rng) % q,
                    splitmix64(&mut rng) % q,
                ];
                assert_eq!(mul5(a, b, q, c).unwrap().0, schoolbook(a, b, q, c));
                random_pairs += 1;
            }
        }
    }
    println!("mul5 random PASS: {random_pairs} deterministic pairs over {random_cells} cells");

    for q in [5_u64, 7, 11, 13, 223, 4_051, 65_537, (1_u64 << 31) - 1] {
        for x in 0..q.min(100_000) {
            assert_eq!(mul(div2(x, q), 2, q), x);
            if q % 3 != 0 {
                assert_eq!(mul(div3(x, q), 3, q), x);
            }
        }
    }
    assert!(mul5([1, 0, 0], [1, 0, 0], 2, 1).is_none());
    assert!(mul5([1, 0, 0], [1, 0, 0], 3, 1).is_none());
    assert!(mul5([1, 0, 0], [1, 0, 0], 9, 1).is_none());
    println!("small division/domain PASS: canonical div2/div3 and gcd(q,6) fallback");

    let q = 7_u64;
    let c = 2_u64;
    for index in 1..q * q * q {
        let a = element(index, q);
        let candidate = adjugate_inverse(a, q, c);
        let reference = f3_pow(a, q * q * q - 2, q, c);
        assert_eq!(candidate, reference);
        assert_eq!(schoolbook(a, candidate, q, c), [1, 0, 0]);
    }
    println!("adjugate inversion PASS: 342 exhaustive nonzero F_(7^3) elements for z^3=2");

    let mut base_inverse_cases = 0_u64;
    for q in [5_u64, 7, 11, 13, 223, 4_051, 65_537, (1_u64 << 31) - 1] {
        for a in 1..q.min(100_000) {
            let binary = binary_inverse(a, q);
            let reference = egcd_inverse(a, q);
            assert_eq!(binary, reference);
            assert_eq!(mul(a, binary, q), 1);
            base_inverse_cases += 1;
        }
    }
    println!("binary inversion PASS: {base_inverse_cases} base-field cases versus Euclid");

    let mut reducer_cases = 0_u64;
    let mut representation_cases = 0_u64;
    for q in [5_u64, 7, 11, 13, 223, 4_051, 65_537, (1_u64 << 31) - 1] {
        let reciprocal = barrett_reciprocal(q);
        let boundaries = [0, 1, q - 1, q, q + 1, q * q - q, q * q - 1];
        for x in boundaries {
            assert_eq!(barrett_reduce(q, reciprocal, x), x % q);
            reducer_cases += 1;
        }
        for _ in 0..10_000 {
            let a = splitmix64(&mut rng) % q;
            let b = splitmix64(&mut rng) % q;
            assert_eq!(barrett_reduce(q, reciprocal, a * b), mul(a, b, q));
            assert_eq!(branchless_add(a, b, q), add(a, b, q));
            assert_eq!(branchless_sub(a, b, q), sub(a, b, q));
            assert_eq!(montgomery_product(a, b, q), mul(a, b, q));
            assert_eq!(a as u32 as u64, a);
            reducer_cases += 1;
            representation_cases += 1;
        }
    }
    println!("reduction PASS: {reducer_cases} Barrett boundary/random cases");
    println!("representation PASS: {representation_cases} branchless/u32/Montgomery cases");

    let max_q = (1_u64 << 31) - 1;
    let max_product = (max_q as u128 - 1) * (max_q as u128 - 1);
    let safe_u64_terms = u64::MAX as u128 / max_product;
    assert_eq!(safe_u64_terms, 4);
    println!(
        "lazy accumulation SCREEN: only {safe_u64_terms} maximal products fit in u64 at q=2^31-1; five/nine-product accumulation requires u128 or staged reduction"
    );
}
