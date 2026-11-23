//! Bounded correctness oracles. These are separate implementations used only
//! by this integration test; production bindings contain no substitutes.

use crypto_lib::cryptanalysis::{
    ic_boundary::{
        CountedGroup, FactorBase, GroupOps, IncrementalGauss, PrimeCurve, PrimePoint,
        RelationSolver, RowStatus,
    },
    ic_framework::{
        stages::InstanceCtx,
        useful_rank::{replay, AuditedMatrix},
    },
    semaev_prime_poly::{
        poly_support_holds, poly_support_jacobian_at_root, poly_support_residual, Fp,
    },
};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

fn add(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}
fn sub(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}
fn mul(p: u64, a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn inverse(p: u64, a: u64) -> u64 {
    assert!(a > 0 && a < p);
    let (mut r0, mut r1) = (p as i128, a as i128);
    let (mut s0, mut s1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (s0, s1) = (s1, s0 - q * s1);
    }
    assert_eq!(r0, 1);
    s0.rem_euclid(p as i128) as u64
}
fn trim(mut a: Vec<u64>) -> Vec<u64> {
    while a.len() > 1 && a.last() == Some(&0) {
        a.pop();
    }
    if a.is_empty() {
        a.push(0);
    }
    a
}
fn product(p: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = add(p, out[i + j], mul(p, x, y));
        }
    }
    trim(out)
}
fn remainder(p: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let b = trim(b.to_vec());
    assert_ne!(b, [0]);
    let lead_inv = inverse(p, *b.last().unwrap());
    let mut r = trim(a.to_vec());
    while r != [0] && r.len() >= b.len() {
        let offset = r.len() - b.len();
        let factor = mul(p, *r.last().unwrap(), lead_inv);
        for (j, &x) in b.iter().enumerate() {
            r[offset + j] = sub(p, r[offset + j], mul(p, factor, x));
        }
        r = trim(r);
    }
    r
}
fn from_roots(p: u64, roots: &[u64]) -> Vec<u64> {
    roots
        .iter()
        .fold(vec![1], |a, &root| product(p, &a, &[sub(p, 0, root), 1]))
}

#[test]
fn support_agrees_with_independent_power_divisibility_for_2830_cases() {
    let mut checked = 0;
    for p in [3u64, 5, 7] {
        let f = Fp::new(p);
        let supports = [
            vec![p - 1, 0, 1],
            vec![0, 0, 1],
            vec![1, 0, 1],
            vec![2, 0, 2, 0],
            vec![1],
        ];
        for degree in 1..=3usize.min(p as usize - 1) {
            for code in 0..p.pow(degree as u32) {
                let mut digits = code;
                let mut candidate = vec![0; degree + 1];
                for coefficient in &mut candidate[..degree] {
                    *coefficient = digits % p;
                    digits /= p;
                }
                candidate[degree] = 1;
                for support in &supports {
                    // This oracle never differentiates g. It constructs F^q
                    // by repeated convolution and divides it by g directly.
                    let mut power = vec![1];
                    for _ in 0..degree {
                        power = product(p, &power, support);
                    }
                    let expected = remainder(p, &power, &candidate) == [0];
                    assert_eq!(
                        poly_support_holds(f, support, &candidate).unwrap(),
                        expected,
                        "p={p}, F={support:?}, g={candidate:?}"
                    );
                    let residual = poly_support_residual(f, support, &candidate).unwrap();
                    assert!(residual.len() <= degree);
                    assert!(residual.iter().all(|&c| c < p));
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 2830);
}

// Independent dual-number long division evaluates the original residual at
// g + epsilon*X^column, epsilon^2=0. It does not use the quotient/Jacobian
// formula implemented by the production API.
fn tangent_residual(p: u64, support: &[u64], g: &[u64], column: usize) -> Vec<u64> {
    let degree = g.len() - 1;
    let divisor: Vec<_> = g
        .iter()
        .enumerate()
        .map(|(i, &value)| (value, u64::from(i == column)))
        .collect();
    let mut numerator = vec![(0, 0); support.len() + degree - 1];
    for (i, &s) in support.iter().enumerate() {
        for j in 1..g.len() {
            numerator[i + j - 1].0 = add(
                p,
                numerator[i + j - 1].0,
                mul(p, s, mul(p, j as u64 % p, g[j])),
            );
            if j == column {
                numerator[i + j - 1].1 = add(p, numerator[i + j - 1].1, mul(p, s, j as u64 % p));
            }
        }
    }
    for high in (degree..numerator.len()).rev() {
        let factor = numerator[high];
        let offset = high - degree;
        for (j, &(a, b)) in divisor.iter().enumerate() {
            let value = mul(p, factor.0, a);
            let tangent = add(p, mul(p, factor.0, b), mul(p, factor.1, a));
            numerator[offset + j].0 = sub(p, numerator[offset + j].0, value);
            numerator[offset + j].1 = sub(p, numerator[offset + j].1, tangent);
        }
    }
    numerator.resize(degree, (0, 0));
    assert!(numerator.iter().all(|&(value, _)| value == 0));
    numerator.into_iter().map(|(_, tangent)| tangent).collect()
}

fn determinant(p: u64, a: &[Vec<u64>]) -> u64 {
    if a.is_empty() {
        return 1;
    }
    let mut value = 0;
    for j in 0..a.len() {
        let minor: Vec<_> = a[1..]
            .iter()
            .map(|row| {
                row.iter()
                    .enumerate()
                    .filter_map(|(i, &x)| (i != j).then_some(x))
                    .collect()
            })
            .collect();
        let term = mul(p, a[0][j], determinant(p, &minor));
        value = if j % 2 == 0 {
            add(p, value, term)
        } else {
            sub(p, value, term)
        };
    }
    value
}

#[test]
fn jacobian_matches_dual_number_residual_and_squarefree_controls() {
    let p = 7;
    let f = Fp::new(p);
    for (support_roots, candidate_roots, nonsingular) in [
        (vec![1, 2], vec![1, 1], true),
        (vec![1, 2], vec![1, 1, 2], true),
        (vec![1, 1], vec![1], false),
        (vec![1, 1], vec![1, 1], false),
        (vec![1, 2, 2], vec![1, 1, 1], true),
    ] {
        let support = from_roots(p, &support_roots);
        let g = from_roots(p, &candidate_roots);
        let jacobian = poly_support_jacobian_at_root(f, &support, &g).unwrap();
        for column in 0..g.len() - 1 {
            let expected = tangent_residual(p, &support, &g, column);
            for row in 0..g.len() - 1 {
                assert_eq!(jacobian[row][column], expected[row]);
            }
        }
        assert_eq!(determinant(p, &jacobian) != 0, nonsingular);
    }

    // Prime31 degree sixteen, with multiplicities four. Check all sixteen
    // Jacobian columns by dual numbers, without a 16! determinant expansion.
    let p = 2_147_483_647;
    let support = from_roots(p, &[0, 1, 2, p - 1]);
    let roots: Vec<_> = [0, 1, 2, p - 1]
        .into_iter()
        .flat_map(|root| [root; 4])
        .collect();
    let g = from_roots(p, &roots);
    let matrix = poly_support_jacobian_at_root(Fp::new(p), &support, &g).unwrap();
    for column in 0..16 {
        let expected = tangent_residual(p, &support, &g, column);
        for row in 0..16 {
            assert_eq!(matrix[row][column], expected[row]);
        }
    }
}

#[test]
fn characteristic_and_representation_guards_are_explicit() {
    let f = Fp::new(3);
    let support = [2, 1]; // X-1.
    let candidate = [0, 0, 0, 1]; // X^3, derivative zero.
    assert!(poly_support_residual(f, &support, &candidate).is_err());
    assert!(poly_support_holds(f, &support, &candidate).is_err());
    assert!(poly_support_jacobian_at_root(f, &support, &candidate).is_err());
    let f = Fp::new(7);
    for (support, candidate) in [
        (vec![], vec![0, 1]),
        (vec![1], vec![]),
        (vec![0, 0], vec![0, 1]),
        (vec![7, 1], vec![0, 1]),
        (vec![1], vec![7, 1]),
        (vec![1], vec![0]),
        (vec![1], vec![1]),
        (vec![1], vec![0, 2]),
        (vec![1], vec![0, 1, 0]),
    ] {
        assert!(poly_support_holds(f, &support, &candidate).is_err());
        assert!(poly_support_jacobian_at_root(f, &support, &candidate).is_err());
    }
    assert!(!poly_support_holds(f, &[1], &[0, 1]).unwrap());
    assert!(poly_support_jacobian_at_root(f, &[1], &[0, 1]).is_err());
    assert!(poly_support_holds(f, &[0, 0, 1], &[0, 0, 1]).unwrap());
    for modulus in [0, 1, 2, 8, 1 << 62] {
        assert!(poly_support_holds(Fp { p: modulus }, &[1], &[0, 1]).is_err());
    }
}

fn trial_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

// Independent affine arithmetic, without the production group type or hooks.
fn affine_add(
    p: u64,
    a: u64,
    left: Option<(u64, u64)>,
    right: Option<(u64, u64)>,
) -> Option<(u64, u64)> {
    let (x1, y1) = match left {
        None => return right,
        Some(point) => point,
    };
    let (x2, y2) = match right {
        None => return left,
        Some(point) => point,
    };
    if x1 == x2 && add(p, y1, y2) == 0 {
        return None;
    }
    let (numerator, denominator) = if left == right {
        (add(p, mul(p, 3, mul(p, x1, x1)), a), mul(p, 2, y1))
    } else {
        (sub(p, y2, y1), sub(p, x2, x1))
    };
    let slope = mul(p, numerator, inverse(p, denominator));
    let x = sub(p, sub(p, mul(p, slope, slope), x1), x2);
    Some((x, sub(p, mul(p, slope, sub(p, x1, x)), y1)))
}

fn affine_mul(p: u64, a: u64, point: (u64, u64), mut scalar: u64) -> Option<(u64, u64)> {
    let mut result = None;
    let mut addend = Some(point);
    while scalar != 0 {
        if scalar & 1 != 0 {
            result = affine_add(p, a, result, addend);
        }
        addend = affine_add(p, a, addend, addend);
        scalar >>= 1;
    }
    result
}

#[test]
fn prime31_arithmetic_and_declared_subgroup_have_independent_checks() {
    let p = 2_147_483_647;
    let r = 2_147_446_831;
    let a = 847_514_846;
    let b = 2_087_444_724;
    let generator = (625_037_715, 2_100_627_251);
    assert!(trial_prime(p) && trial_prime(r));
    assert_ne!(p, r);
    assert_eq!(
        mul(p, generator.1, generator.1),
        add(
            p,
            add(
                p,
                mul(p, mul(p, generator.0, generator.0), generator.0),
                mul(p, a, generator.0)
            ),
            b
        )
    );
    assert_ne!(
        add(
            p,
            mul(p, 4, mul(p, mul(p, a, a), a)),
            mul(p, 27, mul(p, b, b))
        ),
        0
    );
    assert_eq!(affine_mul(p, a, generator, r), None);
    let double = affine_mul(p, a, generator, 2).unwrap();
    assert_eq!(
        affine_mul(p, a, generator, r - 2),
        Some((double.0, p - double.1))
    );
    assert_ne!(
        affine_mul(p, a, generator, p - 2),
        Some((double.0, p - double.1))
    );
    // A conservative Hasse interval contains only the positive multiple r.
    // Together with prime r and a nonidentity r-torsion G, this establishes
    // the declared h=1 order, without exhaustive Prime31 point enumeration.
    let bound = 92_682;
    assert!(bound * bound >= 4 * p);
    assert!(r >= p + 1 - bound && r <= p + 1 + bound);
    assert!(2 * r > p + 1 + bound && p + 1 > bound);

    let f = Fp::new(p);
    let residues = [0, 1, 2, 3, p / 2, p - 3, p - 2, p - 1];
    for x in residues {
        for y in residues {
            let bx = BigUint::from(x);
            let by = BigUint::from(y);
            let bp = BigUint::from(p);
            assert_eq!(f.add(x, y), ((&bx + &by) % &bp).to_u64().unwrap());
            assert_eq!(f.sub(x, y), ((&bx + &bp - &by) % &bp).to_u64().unwrap());
            assert_eq!(f.mul(x, y), ((&bx * &by) % &bp).to_u64().unwrap());
        }
        if x != 0 {
            assert_eq!(f.inv(x), inverse(p, x));
        }
    }
}

fn rank_by_minors(p: u64, a: &[Vec<u64>], columns: usize) -> usize {
    assert!(a.len() < usize::BITS as usize && columns <= 3);
    for size in (1..=columns.min(a.len())).rev() {
        for row_mask in 0usize..1usize << a.len() {
            if row_mask.count_ones() as usize != size {
                continue;
            }
            for col_mask in 0usize..1usize << columns {
                if col_mask.count_ones() as usize != size {
                    continue;
                }
                let minor: Vec<_> = a
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| row_mask >> i & 1 != 0)
                    .map(|(_, row)| {
                        row[..columns]
                            .iter()
                            .enumerate()
                            .filter_map(|(j, &x)| (col_mask >> j & 1 != 0).then_some(x))
                            .collect()
                    })
                    .collect();
                if determinant(p, &minor) != 0 {
                    return size;
                }
            }
        }
    }
    0
}

#[test]
fn audited_rank_agrees_with_all_minors_and_keeps_target_information() {
    let c = PrimeCurve { p: 17, a: 2, b: 2 };
    let g = PrimePoint::affine(5, 1);
    let r = 19;
    let mut ops = GroupOps::default();
    let q = c.mul(&mut ops, g, 2);
    assert_eq!((q.x, q.y), affine_mul(17, 2, (5, 1), 2).unwrap());
    let count = 1
        + (0..17)
            .flat_map(|x| (0..17).map(move |y| (x, y)))
            .filter(|&(x, y)| {
                mul(17, y, y) == add(17, add(17, mul(17, mul(17, x, x), x), mul(17, 2, x)), 2)
            })
            .count();
    assert_eq!(count, r as usize);
    let fb = FactorBase::from_column_map(
        "F17 independent rank oracle".into(),
        vec![g, c.neg(g), q, c.neg(q)],
        vec![0, 0, 1, 1],
        vec![1, r - 1, 1, r - 1],
        2,
        |point| c.key(point),
        |point| c.key(&c.neg(*point)),
        |point| point.x,
    )
    .unwrap();
    let ctx = InstanceCtx {
        group: &c,
        generator: g,
        target: q,
        r,
        cofactor: 1,
        group_order: r,
        name: "independent determinant-minor check".into(),
        field_degree: None,
    };
    let mut matrix = AuditedMatrix::new(&ctx, &fb, Box::new(IncrementalGauss::new(3, r))).unwrap();
    let mut accepted = Vec::new();
    for (row, rhs, expected_status, target_pinned) in [
        (vec![2, 0, 18], 0, RowStatus::Independent, false),
        (vec![0, 1, 18], 0, RowStatus::Independent, false),
        (vec![2, 1, 17], 0, RowStatus::Dependent, false),
        (vec![4, 2, 15], 0, RowStatus::Dependent, false),
        (vec![1, 0, 0], 1, RowStatus::Independent, true),
        (vec![0, 0, 0], 0, RowStatus::Dependent, true),
    ] {
        assert_eq!(matrix.add_row(row.clone(), rhs), expected_status);
        accepted.push(row);
        assert_eq!(matrix.audit.rank, rank_by_minors(r, &accepted, 3));
        assert_eq!(matrix.audit.base_rank, rank_by_minors(r, &accepted, 2));
        assert_eq!(matrix.audit.target_is_determined(), target_pinned);
        assert_eq!(matrix.pinned(2), target_pinned.then_some(2));
        replay(&ctx, &fb, &matrix.audit).unwrap();
    }
    assert_eq!(matrix.audit.duplicate_rows, 1);
    assert_eq!(matrix.audit.independent_rows, 3);
    assert_eq!(matrix.audit.dependent_rows, 3);
    assert_eq!(matrix.add_row(vec![0, 0, 0], 1), RowStatus::Inconsistent);
    assert_eq!(matrix.audit.invalid_rows, 1);
    assert_eq!(matrix.audit.rank, 3);
    assert!(!matrix.audit.target_is_determined());
    assert!(replay(&ctx, &fb, &matrix.audit).is_err());
}
