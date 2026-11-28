use std::fmt::Write as _;

#[derive(Clone)]
struct Curve {
    sector: &'static str,
    f: Vec<u8>,
    infinities: u8,
    horizon: usize,
}

fn add9(a: u8, b: u8) -> u8 {
    (a % 3 + b % 3) % 3 + 3 * ((a / 3 + b / 3) % 3)
}

fn mul9(a: u8, b: u8) -> u8 {
    (a % 3 * (b % 3) + 2 * (a / 3) * (b / 3)) % 3 + 3 * ((a % 3 * (b / 3) + a / 3 * (b % 3)) % 3)
}

fn pow9(mut a: u8, mut n: usize) -> u8 {
    let mut r = 1;
    while n != 0 {
        if n & 1 == 1 {
            r = mul9(r, a);
        }
        a = mul9(a, a);
        n /= 2;
    }
    r
}

fn field_controls() {
    assert_eq!(mul9(3, 3), 2);
    for a in 0..9 {
        assert_eq!(add9(a, 0), a);
        assert_eq!(mul9(a, 1), a);
        assert_eq!(add9(add9(a, a), a), 0);
        assert_eq!(pow9(a, 9), a);
        if a != 0 {
            assert!((1..9).any(|b| mul9(a, b) == 1));
        }
        for b in 0..9 {
            assert_eq!(add9(a, b), add9(b, a));
            assert_eq!(mul9(a, b), mul9(b, a));
            for c in 0..9 {
                assert_eq!(add9(add9(a, b), c), add9(a, add9(b, c)));
                assert_eq!(mul9(mul9(a, b), c), mul9(a, mul9(b, c)));
                assert_eq!(mul9(a, add9(b, c)), add9(mul9(a, b), mul9(a, c)));
            }
        }
    }
    for a in 0..3 {
        for b in 0..3 {
            assert_eq!(add9(a, b), (a + b) % 3);
            assert_eq!(mul9(a, b), a * b % 3);
        }
    }
}

fn monics(degree: usize) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for mut code in 0..3_usize.pow(degree as u32) {
        let mut f = Vec::new();
        for _ in 0..degree {
            f.push((code % 3) as u8);
            code /= 3;
        }
        f.push(1);
        result.push(f);
    }
    result.sort();
    result
}

fn divides(f: &[u8], g: &[u8]) -> bool {
    assert_eq!(g.last(), Some(&1));
    let d = g.len() - 1;
    let mut r = f.to_vec();
    for k in (d..f.len()).rev() {
        let c = r[k];
        for j in 0..=d {
            r[k - d + j] = (r[k - d + j] + 6 - c * g[j]) % 3;
        }
    }
    r.iter().all(|&x| x == 0)
}

fn irreducibles(degree: usize) -> Vec<Vec<u8>> {
    let small: Vec<_> = (1..=degree / 2).flat_map(monics).collect();
    monics(degree)
        .into_iter()
        .filter(|f| small.iter().all(|g| !divides(f, g)))
        .collect()
}

fn multiply3(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            result[i + j] = (result[i + j] + x * y) % 3;
        }
    }
    result
}

fn point_counts(f: &[u8], extension: bool, infinities: u8) -> (i128, Vec<u8>) {
    let q = if extension { 9 } else { 3 };
    let mut fibres = Vec::new();
    for x in 0..q {
        let mut fx = 0;
        for &coefficient in f.iter().rev() {
            fx = if extension {
                add9(mul9(fx, x), coefficient)
            } else {
                (fx * x + coefficient) % 3
            };
        }
        let count = (0..q)
            .filter(|&y| {
                let square = if extension { mul9(y, y) } else { y * y % 3 };
                square == fx
            })
            .count() as u8;
        fibres.push(count);
    }
    let total = i128::from(infinities) + fibres.iter().map(|&x| i128::from(x)).sum::<i128>();
    (total, fibres)
}

fn weil(n1: i128, n2: i128) -> (i128, i128) {
    let t = 4 - n1;
    let numerator = t * t - 10 + n2;
    assert_eq!(numerator % 2, 0);
    (t, numerator / 2)
}

fn sector_matches(sector: &str, t: i128, b: i128) -> bool {
    let p = [9, -3 * t, b, -t, 1].map(|x| x.rem_euclid(2));
    p == if sector == "33" {
        [1, 0, 1, 0, 1]
    } else {
        [1, 1, 1, 1, 1]
    }
}

fn traces(t: i128, b: i128, maximum: usize) -> Vec<i128> {
    let mut s = vec![0; maximum + 1];
    s[0] = 4;
    s[1] = t;
    s[2] = t * t - 2 * b;
    s[3] = t * s[2] - b * s[1] + 9 * t;
    s[4] = t * s[3] - b * s[2] + 3 * t * s[1] - 36;
    for n in 5..=maximum {
        s[n] = t * s[n - 1] - b * s[n - 2] + 3 * t * s[n - 3] - 9 * s[n - 4];
    }
    s
}

type Matrix = [[i128; 4]; 4];

fn matrix_product(a: &Matrix, b: &Matrix) -> Matrix {
    let mut c = [[0_i128; 4]; 4];
    for (i, row) in c.iter_mut().enumerate() {
        for (j, entry) in row.iter_mut().enumerate() {
            for (k, b_row) in b.iter().enumerate() {
                *entry = entry
                    .checked_add(a[i][k].checked_mul(b_row[j]).unwrap())
                    .unwrap();
            }
        }
    }
    c
}

fn determinant(mut a: Matrix) -> i128 {
    let mut sign = 1;
    let mut previous = 1;
    for k in 0..3 {
        if a[k][k] == 0 {
            let Some(pivot) = (k + 1..4).find(|&i| a[i][k] != 0) else {
                return 0;
            };
            a.swap(k, pivot);
            sign = -sign;
        }
        let pivot = a[k][k];
        for i in k + 1..4 {
            for j in k + 1..4 {
                let numerator = a[i][j]
                    .checked_mul(pivot)
                    .unwrap()
                    .checked_sub(a[i][k].checked_mul(a[k][j]).unwrap())
                    .unwrap();
                assert_eq!(numerator % previous, 0);
                a[i][j] = numerator / previous;
            }
            a[i][k] = 0;
        }
        previous = pivot;
    }
    sign * a[3][3]
}

fn extension_orders(t: i128, b: i128, horizon: usize) -> Vec<u64> {
    let s = traces(t, b, 2 * horizon);
    let companion = [[0, 0, 0, -9], [1, 0, 0, 3 * t], [0, 1, 0, -b], [0, 0, 1, t]];
    let mut power = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
    let mut orders = Vec::new();
    for n in 1..=horizon {
        let qn = 3_i128.pow(n as u32);
        let exterior_numerator = s[n] * s[n] - s[2 * n];
        assert_eq!(exterior_numerator % 2, 0);
        let order = qn * qn + 1 - (qn + 1) * s[n] + exterior_numerator / 2;
        assert!(order > 0);
        power = matrix_product(&power, &companion);
        let mut difference = power;
        for (i, row) in difference.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                *entry = i128::from(i == j) - *entry;
            }
        }
        assert_eq!(
            order,
            determinant(difference),
            "Newton/resultant mismatch at n={n}"
        );
        orders.push(u64::try_from(order).unwrap());
    }
    assert_eq!(i128::from(orders[0]), 10 - 4 * t + b);
    orders
}

fn elliptic_order(t: i128, n: usize) -> i128 {
    let mut previous = 2;
    let mut current = t;
    for _ in 2..=n {
        (previous, current) = (current, t * current - 3 * previous);
    }
    3_i128.pow(n as u32) + 1 - current
}

fn elliptic_factors(t: i128, b: i128) -> Option<[i128; 2]> {
    let discriminant = t * t - 4 * (b - 6);
    let square_root = (0..=24).find(|&x| x * x == discriminant)?;
    if (t + square_root) & 1 != 0 {
        return None;
    }
    let factors = [(t - square_root) / 2, (t + square_root) / 2];
    assert!(factors.iter().all(|x| x * x <= 12));
    assert_eq!(factors[0] + factors[1], t);
    assert_eq!(factors[0] * factors[1] + 6, b);
    Some(factors)
}

fn integers<T: std::fmt::Display>(values: &[T]) -> String {
    let mut result = String::new();
    for (i, value) in values.iter().enumerate() {
        if i != 0 {
            result.push(',');
        }
        write!(&mut result, "{value}").unwrap();
    }
    result
}

fn main() {
    field_controls();
    let cubics = irreducibles(3);
    let quintics = irreducibles(5);
    assert_eq!(cubics.len(), 8);
    assert_eq!(quintics.len(), 48);
    let mut curves = Vec::new();
    for (i, a) in cubics.iter().enumerate() {
        println!("CUBIC id={i} f={}", integers(a));
        for b in cubics.iter().skip(i + 1) {
            curves.push(Curve {
                sector: "33",
                f: multiply3(a, b),
                infinities: 2,
                horizon: 12,
            });
        }
    }
    for f in quintics {
        curves.push(Curve {
            sector: "51",
            f,
            infinities: 1,
            horizon: 20,
        });
    }
    curves.sort_by(|a, b| a.sector.cmp(b.sector).then(a.f.cmp(&b.f)));
    assert_eq!(curves.len(), 76);
    assert!(curves.windows(2).all(|w| w[0].f != w[1].f));
    let mut sectors = [0; 2];
    let mut split_counts = [0; 2];
    let mut minimal_counts = [0; 2];
    let mut ordinary_simple = [0; 2];
    let mut selected_split = None;
    let mut selected_nonsplit = None;
    let mut known_control = false;
    let mut mutants_rejected = 0;
    let mut comparisons = 0;
    let mut output_bytes = 0;
    println!("PROTOCOL realcurve-A q=3 F9=X^2+1 coefficient_order=ascending lex=vector");
    for (id, curve) in curves.iter().enumerate() {
        let sector_index = usize::from(curve.sector == "51");
        sectors[sector_index] += 1;
        let (n1, fibres1) = point_counts(&curve.f, false, curve.infinities);
        let (n2, fibres2) = point_counts(&curve.f, true, curve.infinities);
        let (t, b) = weil(n1, n2);
        assert!(sector_matches(curve.sector, t, b));
        let (mutant_t, mutant_b) = weil(n1, n2 + 2);
        assert!(!sector_matches(curve.sector, mutant_t, mutant_b));
        assert!(sector_matches(curve.sector, t, b));
        mutants_rejected += 1;
        let discriminant = t * t - 4 * b + 24;
        assert!(discriminant >= 0);
        assert!(t * t <= 48);
        let bound = 24 + 4 * b;
        assert!(bound >= 0 && bound * bound >= 192 * t * t);
        let orders = extension_orders(t, b, curve.horizon);
        comparisons += orders.len();
        let valuations: Vec<_> = orders.iter().map(|x| x.trailing_zeros()).collect();
        let expected_divisor = if sector_index == 0 { 2 } else { 4 };
        assert!(valuations
            .iter()
            .all(|x| x.is_multiple_of(expected_divisor)));
        assert_eq!(orders[curve.horizon - 1] % 4096, 0);
        let minimal = valuations[curve.horizon - 1] == 12;
        minimal_counts[sector_index] += usize::from(minimal);
        let ordinary = b % 3 != 0;
        let factors = elliptic_factors(t, b);
        let factor_text = if let Some(traces) = factors {
            split_counts[sector_index] += 1;
            for (i, &order) in orders.iter().enumerate() {
                assert_eq!(
                    i128::from(order),
                    elliptic_order(traces[0], i + 1) * elliptic_order(traces[1], i + 1)
                );
            }
            if sector_index == 0 && minimal && selected_split.is_none() {
                selected_split = Some(id);
            }
            integers(&traces)
        } else {
            ordinary_simple[sector_index] += usize::from(ordinary);
            if sector_index == 0 && minimal && ordinary && selected_nonsplit.is_none() {
                selected_nonsplit = Some(id);
            }
            "none".to_string()
        };
        if sector_index == 1 {
            assert!(factors.is_none());
        }
        if curve.f == [2, 0, 1, 0, 1, 0, 1] {
            assert_eq!((n1, n2, t, b), (2, 20, 2, 7));
            assert_eq!(factors, Some([1, 1]));
            assert_eq!(orders[11], 283_875_840_000);
            assert_eq!(orders[11] / 4096, 69_305_625);
            assert!(minimal);
            known_control = true;
        }
        let line = format!("CURVE id={id} sector={} f={} infinity={} N1={n1} N2={n2} fibres1={} fibres9={} t={t} b={b} D={discriminant} ordinary={} e_traces={factor_text} orders={} v2={} minimal8={}",
            curve.sector, integers(&curve.f), curve.infinities, integers(&fibres1),
            integers(&fibres2), u8::from(ordinary), integers(&orders),
            integers(&valuations), u8::from(minimal));
        output_bytes += line.len() + 1;
        println!("{line}");
    }
    assert_eq!(sectors, [28, 48]);
    assert_eq!(split_counts[1], 0);
    assert!(known_control);
    assert_eq!(mutants_rejected, 76);
    assert_eq!(comparisons, 1296);
    assert!(output_bytes < 60_000);
    println!("SUMMARY curves={sectors:?} split={split_counts:?} minimal8={minimal_counts:?} ordinary_nonsplit={ordinary_simple:?} selected_split={selected_split:?} selected_nonsplit={selected_nonsplit:?} rejected_mutants={mutants_rejected} determinant_comparisons={comparisons} known_control=pass field_controls=pass");
}
