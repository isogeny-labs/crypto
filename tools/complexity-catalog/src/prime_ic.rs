//! Finite prime-field controls for proposed IC compression ideas.
//! These are exact toy arithmetic checks, not performance or theorem evidence.
fn power(mut x: u64, mut n: u64, p: u64) -> u64 {
    let mut y = 1;
    while n > 0 {
        if n & 1 == 1 {
            y = y * x % p;
        }
        x = x * x % p;
        n >>= 1;
    }
    y
}
fn rank(mut a: Vec<Vec<u64>>, p: u64) -> usize {
    let mut r = 0;
    for c in 0..a[0].len() {
        let Some(pivot) = (r..a.len()).find(|&i| a[i][c] != 0) else {
            continue;
        };
        a.swap(r, pivot);
        let inv = power(a[r][c], p - 2, p);
        for value in &mut a[r] {
            *value = *value * inv % p;
        }
        for i in 0..a.len() {
            if i == r {
                continue;
            }
            let factor = a[i][c];
            for j in c..a[0].len() {
                a[i][j] = (a[i][j] + p - factor * a[r][j] % p) % p;
            }
        }
        r += 1;
        if r == a.len() {
            break;
        }
    }
    r
}
fn s3(x: u64, y: u64, z: u64, a: u64, b: u64, p: u64) -> u64 {
    let d = (x + p - y) % p;
    let q = (x * y + p - a) % p;
    let middle = ((x + y) % p * ((x * y + a) % p) + 2 * b) % p;
    (d * d % p * z % p * z + p - 2 * middle % p * z % p + q * q % p + p
        - 4 * b % p * ((x + y) % p) % p)
        % p
}
#[test]
fn small_polynomial_value_rank_does_not_bound_exact_zero_filter_rank() {
    for (p, b) in [(5, 4), (7, 6), (17, 8), (101, 8)] {
        let values: Vec<Vec<_>> = (0..b)
            .map(|x| (0..b).map(|y| (x + p - y) % p).collect())
            .collect();
        let zeros: Vec<Vec<_>> = values
            .iter()
            .map(|row| row.iter().map(|&v| u64::from(v == 0)).collect())
            .collect();
        assert_eq!(rank(values, p), 2);
        assert_eq!(rank(zeros, p), b as usize);
    }
}
#[test]
fn actual_semaev_values_and_zero_filters_are_audited_separately() {
    // Nonsingular prime-field curve y^2=x^3+2x+3, not a j=0 curve.
    let p = 17;
    let xs: Vec<_> = (0..p)
        .filter(|&x| {
            let rhs = (x * x % p * x + 2 * x + 3) % p;
            rhs == 0 || power(rhs, (p - 1) / 2, p) == 1
        })
        .collect();
    assert!((4 * 8 + 27 * 9) % p != 0);
    let mut larger_filter_seen = false;
    for &z in &xs {
        let values: Vec<Vec<_>> = xs
            .iter()
            .map(|&x| xs.iter().map(|&y| s3(x, y, z, 2, 3, p)).collect())
            .collect();
        let zeros: Vec<Vec<_>> = values
            .iter()
            .map(|row| row.iter().map(|&v| u64::from(v == 0)).collect())
            .collect();
        let vr = rank(values, p);
        let zr = rank(zeros, p);
        assert!(vr <= 3);
        larger_filter_seen |= zr > 3;
        println!(
            "p={p} a=2 b=3 z={z} rational_abscissae={} value_rank={vr} zero_filter_rank={zr}",
            xs.len()
        );
    }
    assert!(larger_filter_seen);
}
#[test]
fn conditional_ic_cost_hurdle_keeps_oracle_and_linear_algebra_costs() {
    // Exponents in hundredths, under the README's explicitly heuristic model.
    let cost = |m: i64, beta: i64, alpha: i64, delta: i64| {
        [beta, 100 - (m - 1) * beta + alpha + delta, 2 * beta]
    };
    assert_eq!(cost(4, 20, 10, 0), [20, 50, 40]); // This oracle only reaches rho's exponent.
    assert!(cost(4, 20, 9, 0).into_iter().all(|x| x < 50)); // Strictly harder solver requirement.
    assert!(cost(3, 20, 0, 0)[1] > 50);
    assert!(cost(4, 20, 9, 2)[1] > 50); // A usefulness penalty can erase the margin.
    assert!(cost(4, 26, 0, 0)[2] > 50); // Linear algebra alone fails this hurdle.
}

#[test]
fn prime_order_subgroup_zero_filter_also_exceeds_value_rank() {
    type Pt = Option<(u64, u64)>;
    fn add(a: Pt, b: Pt) -> Pt {
        let (Some((x, y)), Some((u, v))) = (a, b) else {
            return a.or(b);
        };
        let p = 17;
        if x == u && (y + v) % p == 0 {
            return None;
        }
        let (num, den) = if x == u {
            ((3 * x * x + 2) % p, 2 * y % p)
        } else {
            ((v + p - y) % p, (u + p - x) % p)
        };
        let slope = num * power(den, p - 2, p) % p;
        let xx = (slope * slope + 2 * p - x - u) % p;
        Some((xx, (slope * ((x + p - xx) % p) + p - y) % p))
    }
    let mut subgroup = Vec::new();
    for x in 0..17 {
        for y in 0..17 {
            if y * y % 17 == (x * x * x + 2 * x + 3) % 17 {
                let point = Some((x, y));
                if (0..11).fold(None, |sum, _| add(sum, point)).is_none() {
                    subgroup.push((x, y));
                }
            }
        }
    }
    assert_eq!(subgroup.len(), 10);
    let xs: Vec<_> = subgroup
        .iter()
        .map(|&(x, _)| x)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(xs.len(), 5);
    let mut larger = false;
    for &z in &xs {
        let values: Vec<Vec<_>> = xs
            .iter()
            .map(|&x| xs.iter().map(|&y| s3(x, y, z, 2, 3, 17)).collect())
            .collect();
        let zeros: Vec<Vec<_>> = values
            .iter()
            .map(|row| row.iter().map(|&v| u64::from(v == 0)).collect())
            .collect();
        let vr = rank(values, 17);
        let zr = rank(zeros, 17);
        assert!(vr <= 3);
        larger |= zr > 3;
        println!("p=17 subgroup_order=11 signed_points=10 abscissae=5 z={z} value_rank={vr} zero_filter_rank={zr}");
    }
    assert!(larger);
}
