//! Recover signed root-base pairs from a target point via ratio-fiber quartics.
//! This is a pair locator, not a six-term ECDLP relation generator.
use ecdlp_pieces::{selected_root_factor_base, Point, P, ROOT_GENERATOR, ROOT_ORDER, SELECTED};
use std::collections::{BTreeSet, HashMap};
use std::time::Instant;

fn add(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
fn power(mut a: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 != 0 {
            result = mul(result, a);
        }
        a = mul(a, a);
        exponent >>= 1;
    }
    result
}
fn inverse(a: u64) -> u64 {
    assert_ne!(a, 0);
    power(a, P - 2)
}

type Poly = Vec<u64>; // ascending coefficients
fn trim(a: &mut Poly) {
    while a.last() == Some(&0) {
        a.pop();
    }
}
fn monic(mut a: Poly) -> Poly {
    trim(&mut a);
    if let Some(&leading) = a.last() {
        let inv = inverse(leading);
        for coefficient in &mut a {
            *coefficient = mul(*coefficient, inv);
        }
    }
    a
}
fn div_rem(mut a: Poly, b: &[u64]) -> (Poly, Poly) {
    assert_eq!(b.last(), Some(&1));
    trim(&mut a);
    let mut quotient = vec![0; a.len().saturating_sub(b.len()) + 1];
    while !a.is_empty() && a.len() >= b.len() {
        let shift = a.len() - b.len();
        let factor = *a.last().unwrap();
        quotient[shift] = factor;
        for (j, &coefficient) in b.iter().enumerate() {
            a[shift + j] = sub(a[shift + j], mul(factor, coefficient));
        }
        trim(&mut a);
    }
    trim(&mut quotient);
    (quotient, a)
}
fn gcd(mut a: Poly, mut b: Poly) -> Poly {
    while !b.is_empty() {
        b = monic(b);
        let (_, remainder) = div_rem(a, &b);
        a = b;
        b = remainder;
    }
    monic(a)
}
fn multiply_mod(a: &[u64], b: &[u64], modulus: &[u64]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut product = vec![0; a.len() + b.len() - 1];
    for (i, &left) in a.iter().enumerate() {
        for (j, &right) in b.iter().enumerate() {
            product[i + j] = add(product[i + j], mul(left, right));
        }
    }
    div_rem(product, modulus).1
}
fn power_mod(mut base: Poly, mut exponent: u64, modulus: &[u64]) -> Poly {
    let mut result = vec![1];
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = multiply_mod(&result, &base, modulus);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = multiply_mod(&base, &base, modulus);
        }
    }
    result
}
fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}
fn split_roots(q: Poly, roots: &mut Vec<u64>, state: &mut u64) {
    if q.len() <= 1 {
        return;
    }
    if q.len() == 2 {
        roots.push(sub(0, q[0])); // monic linear factor
        return;
    }
    for _ in 0..128 {
        let c = splitmix64(state) % P;
        let mut test = power_mod(vec![c, 1], (P - 1) / 2, &q);
        if test.is_empty() {
            test.push(0);
        }
        test[0] = sub(test[0], 1);
        trim(&mut test);
        let divisor = gcd(q.clone(), test);
        if divisor.len() > 1 && divisor.len() < q.len() {
            let (quotient, remainder) = div_rem(q, &divisor);
            assert!(remainder.is_empty());
            split_roots(divisor, roots, state);
            split_roots(quotient, roots, state);
            return;
        }
    }
    panic!("linear-factor split exhausted deterministic samples");
}
fn field_roots(q: Poly, state: &mut u64) -> Vec<u64> {
    let q = monic(q);
    let mut xp_minus_x = power_mod(vec![0, 1], P, &q);
    if xp_minus_x.len() < 2 {
        xp_minus_x.resize(2, 0);
    }
    xp_minus_x[1] = sub(xp_minus_x[1], 1);
    trim(&mut xp_minus_x);
    let linear_part = gcd(q, xp_minus_x);
    let mut roots = Vec::new();
    split_roots(linear_part, &mut roots, state);
    roots.sort_unstable();
    roots.dedup();
    roots
}

fn quartic(t: u64, u: u64) -> Poly {
    let t2 = mul(t, t);
    let tp1 = add(t, 1);
    let tm1 = sub(t, 1);
    vec![
        sub(9, mul(160, u)),
        mul(tp1, sub(mul(6, u), 160)),
        add(mul(mul(tm1, tm1), mul(u, u)), mul(6, t)),
        sub(0, mul(2, mul(mul(t, tp1), u))),
        t2,
    ]
}

fn locate(target: Point, base: &[Point]) -> Vec<(usize, usize, i8, i8)> {
    let target_x = match target {
        Point::Affine { x, .. } => x,
        Point::Infinity => panic!("target at infinity"),
    };
    let by_x = base
        .iter()
        .enumerate()
        .map(|(i, point)| match point {
            Point::Affine { x, .. } => (*x, i),
            Point::Infinity => unreachable!(),
        })
        .collect::<HashMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut ratio = ROOT_GENERATOR;
    let mut state = target_x ^ 0x65d3_14a1_1ed5_2026;
    for _ in 1..ROOT_ORDER {
        for x in field_roots(quartic(ratio, target_x), &mut state) {
            let Some(&i) = by_x.get(&x) else { continue };
            let Some(&j) = by_x.get(&mul(ratio, x)) else {
                continue;
            };
            if i >= j {
                continue;
            }
            for (left_sign, left) in [(1, base[i]), (-1, SELECTED.neg(base[i]))] {
                for (right_sign, right) in [(1, base[j]), (-1, SELECTED.neg(base[j]))] {
                    if SELECTED.add(left, right).unwrap() == target {
                        seen.insert((i, j, left_sign, right_sign));
                    }
                }
            }
        }
        ratio = mul(ratio, ROOT_GENERATOR);
    }
    assert_eq!(ratio, 1);
    seen.into_iter().collect()
}

fn brute(target: Point, base: &[Point]) -> Vec<(usize, usize, i8, i8)> {
    let mut found = Vec::new();
    for (i, &p) in base.iter().enumerate() {
        for (j, &q) in base.iter().enumerate().skip(i + 1) {
            for (left_sign, left) in [(1, p), (-1, SELECTED.neg(p))] {
                for (right_sign, right) in [(1, q), (-1, SELECTED.neg(q))] {
                    if SELECTED.add(left, right).unwrap() == target {
                        found.push((i, j, left_sign, right_sign));
                    }
                }
            }
        }
    }
    found
}

fn main() {
    let base = selected_root_factor_base();
    let mut root_test_state = 1;
    assert_eq!(
        field_roots(vec![24, P - 50, 35, P - 10, 1], &mut root_test_state),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        field_roots(vec![4, P - 12, 13, P - 6, 1], &mut root_test_state),
        vec![1, 2]
    );
    assert!(field_roots(vec![1, 0, 1], &mut root_test_state).is_empty());
    let target = SELECTED.add(base[0], base[1]).unwrap();
    let start = Instant::now();
    let found = locate(target, &base);
    let elapsed = start.elapsed();
    assert!(found.contains(&(0, 1, 1, 1)));
    println!(
        "field={} base={} ratios={} target_pair=(0,1,+,+) locator_witnesses={} locator_ms={}",
        P,
        base.len(),
        ROOT_ORDER - 1,
        found.len(),
        elapsed.as_millis()
    );
    println!("first_witness={:?}", found[0]);
    // Compare against exhaustive group additions on a small prefix only.
    let prefix = &base[..16];
    let check_target = SELECTED.add(prefix[0], prefix[1]).unwrap();
    let expected = brute(check_target, prefix);
    let recovered = found
        .iter()
        .copied()
        .filter(|&(i, j, _, _)| i < prefix.len() && j < prefix.len())
        .collect::<Vec<_>>();
    assert_eq!(recovered, expected);
    println!(
        "prefix_brute_columns={} prefix_witnesses={}",
        prefix.len(),
        expected.len()
    );
    let second_target = SELECTED.add(base[17], SELECTED.neg(base[203])).unwrap();
    let second = locate(second_target, &base);
    assert!(second.contains(&(17, 203, 1, -1)));
    println!(
        "second_planted_witness=(17,203,+,-) locator_witnesses={}",
        second.len()
    );
}
