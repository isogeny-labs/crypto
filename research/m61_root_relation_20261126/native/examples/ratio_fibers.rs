//! Cyclic ratio-fiber counts for the selected root x-base.
use ecdlp_pieces::{selected_root_factor_base, Point, P, ROOT_GENERATOR, ROOT_ORDER, SELECTED};
use std::collections::HashSet;

const SHUFFLES: u64 = 128;

fn mul_mod(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

fn add_mod(a: u64, b: u64) -> u64 {
    let sum = a + b;
    if sum >= P {
        sum - P
    } else {
        sum
    }
}

fn sub_mod(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        P - (b - a)
    }
}

fn pair_sum_quartic(t: u64, u: u64, x: u64) -> u64 {
    // F3(x, tx, u)/(t-1)^2, specialized to a=-3, b=40.
    let t2 = mul_mod(t, t);
    let tp1 = add_mod(t, 1);
    let tm1 = sub_mod(t, 1);
    let u2 = mul_mod(u, u);
    let c4 = t2;
    let c3 = sub_mod(0, mul_mod(2, mul_mod(mul_mod(t, tp1), u)));
    let c2 = add_mod(mul_mod(mul_mod(tm1, tm1), u2), mul_mod(6, t));
    let c1 = mul_mod(tp1, sub_mod(mul_mod(6, u), 160));
    let c0 = sub_mod(9, mul_mod(160, u));
    add_mod(
        mul_mod(
            add_mod(
                mul_mod(add_mod(mul_mod(add_mod(mul_mod(c4, x), c3), x), c2), x),
                c1,
            ),
            x,
        ),
        c0,
    )
}

fn pow_mod(mut base: u64, mut exponent: u64) -> u64 {
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul_mod(result, base);
        }
        base = mul_mod(base, base);
        exponent >>= 1;
    }
    result
}

// Monic Euclidean gcd; coefficients are in ascending order over F_p.
fn polynomial_gcd_degree(mut a: Vec<u64>, mut b: Vec<u64>) -> usize {
    while !b.is_empty() {
        let inverse = pow_mod(*b.last().unwrap(), P - 2);
        while a.len() >= b.len() && !a.is_empty() {
            let shift = a.len() - b.len();
            let factor = mul_mod(*a.last().unwrap(), inverse);
            for (j, &coefficient) in b.iter().enumerate() {
                let product = mul_mod(factor, coefficient);
                let index = shift + j;
                a[index] = if a[index] >= product {
                    a[index] - product
                } else {
                    P - (product - a[index])
                };
            }
            while a.last() == Some(&0) {
                a.pop();
            }
        }
        (a, b) = (b, a);
    }
    a.len().saturating_sub(1)
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn shuffle(mask: &mut [bool], seed: u64) {
    let mut state = seed;
    for i in (1..mask.len()).rev() {
        let j = (splitmix64(&mut state) as usize) % (i + 1);
        mask.swap(i, j);
    }
}

fn ratio_counts(mask: &[bool]) -> Vec<u32> {
    let doubled = mask.iter().chain(mask).copied().collect::<Vec<_>>();
    (0..mask.len())
        .map(|shift| {
            mask.iter()
                .enumerate()
                .filter(|&(i, valid)| *valid && doubled[i + shift])
                .count() as u32
        })
        .collect()
}

fn nonzero_extrema(counts: &[u32]) -> (u32, u32) {
    let tail = &counts[1..];
    (*tail.iter().min().unwrap(), *tail.iter().max().unwrap())
}

fn nonzero_rms(counts: &[u32], mean: f64) -> f64 {
    (counts[1..]
        .iter()
        .map(|&count| (count as f64 - mean).powi(2))
        .sum::<f64>()
        / (counts.len() - 1) as f64)
        .sqrt()
}

fn main() {
    let base = selected_root_factor_base();
    let x_set = base
        .iter()
        .map(|point| match point {
            Point::Affine { x, .. } => *x,
            Point::Infinity => panic!("root base contains infinity"),
        })
        .collect::<HashSet<_>>();
    assert_eq!(x_set.len(), base.len());

    let mut mask = Vec::with_capacity(ROOT_ORDER as usize);
    let mut seen = HashSet::new();
    let mut x = 1_u64;
    for _ in 0..ROOT_ORDER {
        assert!(seen.insert(x));
        mask.push(x_set.contains(&x));
        x = mul_mod(x, ROOT_GENERATOR);
    }
    assert_eq!(x, 1);
    assert_eq!(mask.iter().filter(|&&value| value).count(), base.len());

    let (x1, x2) = match (base[0], base[1]) {
        (Point::Affine { x: x1, .. }, Point::Affine { x: x2, .. }) => (x1, x2),
        _ => unreachable!(),
    };
    let pair_sum = SELECTED.add(base[0], base[1]).unwrap();
    let u = match pair_sum {
        Point::Affine { x, .. } => x,
        Point::Infinity => panic!("fixture pair has zero sum"),
    };
    let t = mul_mod(x2, pow_mod(x1, P - 2));
    assert_ne!(t, 1);
    assert_eq!(pair_sum_quartic(t, u, x1), 0);
    let mut checked_signed_pairs = 0;
    for i in 0..64 {
        let p = base[i];
        let q = base[i + 1];
        let first_x = match p {
            Point::Affine { x, .. } => x,
            Point::Infinity => unreachable!(),
        };
        let second_x = match q {
            Point::Affine { x, .. } => x,
            Point::Infinity => unreachable!(),
        };
        let ratio = mul_mod(second_x, pow_mod(first_x, P - 2));
        for right in [q, SELECTED.neg(q)] {
            let sum = SELECTED.add(p, right).unwrap();
            let sum_x = match sum {
                Point::Affine { x, .. } => x,
                Point::Infinity => panic!("distinct-x pair has zero sum"),
            };
            assert_eq!(pair_sum_quartic(ratio, sum_x, first_x), 0);
            checked_signed_pairs += 1;
        }
    }

    let f = vec![40, P - 3, 0, 1];
    let mut shared_branch_ratios = 0;
    let mut ratio = ROOT_GENERATOR;
    for _ in 1..ROOT_ORDER {
        let ratio_cubic = mul_mod(mul_mod(ratio, ratio), ratio);
        let scaled_f = vec![40, P - mul_mod(3, ratio), 0, ratio_cubic];
        if polynomial_gcd_degree(f.clone(), scaled_f) != 0 {
            shared_branch_ratios += 1;
        }
        ratio = mul_mod(ratio, ROOT_GENERATOR);
    }
    assert_eq!(ratio, 1);
    assert_eq!(shared_branch_ratios, 0);

    let actual = ratio_counts(&mask);
    assert_eq!(actual[0] as usize, base.len());
    assert_eq!(
        actual.iter().map(|&value| value as u64).sum::<u64>(),
        (base.len() * base.len()) as u64
    );
    for shift in 1..mask.len() {
        assert_eq!(actual[shift], actual[mask.len() - shift]);
    }
    let (minimum, maximum) = nonzero_extrema(&actual);
    let opposite_x_pairs = actual[mask.len() / 2];
    assert_eq!(opposite_x_pairs, 1232);
    let (left, right) = base
        .iter()
        .copied()
        .find_map(|left| match left {
            Point::Affine { x, .. } => base
                .iter()
                .copied()
                .find(|right| matches!(right, Point::Affine { x: other_x, .. } if *other_x == sub_mod(0, x)))
                .map(|right| (left, right)),
            Point::Infinity => None,
        })
        .unwrap();
    let (first_x, w) = match (left, right) {
        (Point::Affine { x, y }, Point::Affine { y: other_y, .. }) => (x, mul_mod(y, other_y)),
        _ => unreachable!(),
    };
    let x_squared = mul_mod(first_x, first_x);
    let quotient_u = sub_mod(2, x_squared);
    let quotient_rhs = add_mod(
        sub_mod(
            mul_mod(mul_mod(quotient_u, quotient_u), quotient_u),
            mul_mod(3, quotient_u),
        ),
        1598,
    );
    assert_eq!(mul_mod(w, w), quotient_rhs);
    let sum_x = match SELECTED.add(left, right).unwrap() {
        Point::Affine { x, .. } => x,
        Point::Infinity => panic!("opposite-x pair has zero sum"),
    };
    assert_eq!(
        sum_x,
        mul_mod(sub_mod(40, w), pow_mod(mul_mod(2, x_squared), P - 2))
    );
    let nonzero_mean = (base.len() * (base.len() - 1)) as f64 / (mask.len() - 1) as f64;
    let root_rms = nonzero_rms(&actual, nonzero_mean);

    let mut control_minima = Vec::new();
    let mut control_maxima = Vec::new();
    let mut control_rms = Vec::new();
    for control in 0..SHUFFLES {
        let mut shuffled = mask.clone();
        shuffle(&mut shuffled, 2026 + control);
        let counts = ratio_counts(&shuffled);
        let (low, high) = nonzero_extrema(&counts);
        control_minima.push(low);
        control_maxima.push(high);
        control_rms.push(nonzero_rms(&counts, nonzero_mean));
    }
    let min_control_mean = control_minima.iter().map(|&x| x as f64).sum::<f64>() / SHUFFLES as f64;
    let max_control_mean = control_maxima.iter().map(|&x| x as f64).sum::<f64>() / SHUFFLES as f64;
    let controls_at_least_maximum = control_maxima.iter().filter(|&&x| x >= maximum).count();
    let controls_at_most_minimum = control_minima.iter().filter(|&&x| x <= minimum).count();

    println!(
        "root_order={} lift_valid={} nonzero_ratio_mean={nonzero_mean:.6} root_min={minimum} root_max={maximum} root_rms={root_rms:.6}",
        mask.len(), base.len()
    );
    println!(
        "nonidentity_ratio_shared_cubic_roots={shared_branch_ratios} nonidentity_ratio_cover_genus=4 quotient_product_curve_genus=2"
    );
    println!(
        "opposite_x_ordered_pairs={opposite_x_pairs} opposite_x_unordered_pairs={}",
        opposite_x_pairs / 2
    );
    println!("opposite_x_quotient_witness_x={first_x} pair_sum_x={sum_x} quotient_checked=1");
    println!("quartic_witness_x={x1} ratio={t} pair_sum_x={u} quartic_value=0");
    println!("quartic_checked_signed_pairs={checked_signed_pairs}");
    println!(
        "shuffled_controls={SHUFFLES} minimum_mean={min_control_mean:.3} minimum_range={}..={} maximum_mean={max_control_mean:.3} maximum_range={}..={} controls_max_at_least_root={controls_at_least_maximum} controls_min_at_most_root={controls_at_most_minimum}",
        control_minima.iter().min().unwrap(),
        control_minima.iter().max().unwrap(),
        control_maxima.iter().min().unwrap(),
        control_maxima.iter().max().unwrap()
    );
    println!(
        "control_rms_mean={:.6} control_rms_range={:.6}..={:.6} controls_rms_at_most_root={}",
        control_rms.iter().sum::<f64>() / control_rms.len() as f64,
        control_rms.iter().copied().reduce(f64::min).unwrap(),
        control_rms.iter().copied().reduce(f64::max).unwrap(),
        control_rms
            .iter()
            .filter(|&&value| value <= root_rms)
            .count()
    );
}
