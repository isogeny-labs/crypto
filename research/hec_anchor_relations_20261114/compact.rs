//! Exact bounded kernel; assumptions and recurrence in COMPACT-PROTOCOL.md.
use hec_native_ic::{fp::Fp, jac::Curve, poly::Poly};

pub struct Orbit {
    pairs: [(Poly, Poly); 16],
    len: usize,
}
impl Orbit {
    pub fn as_slice(&self) -> &[(Poly, Poly)] {
        &self.pairs[..self.len]
    }
}

fn product(points: &[(u64, u64)], omitted: usize, f: &Fp) -> [u64; 6] {
    let mut a = [0; 6];
    a[0] = 1;
    let mut degree = 0;
    for (i, &(x, _)) in points.iter().enumerate() {
        if omitted & (1 << i) != 0 {
            continue;
        }
        for k in (1..=degree + 1).rev() {
            a[k] = f.sub(a[k - 1], f.mul(x, a[k]));
        }
        a[0] = f.neg(f.mul(x, a[0]));
        degree += 1;
    }
    a
}

pub fn signed_orbit(c: &Curve, points: &[(u64, u64)], weights: &[u64]) -> Orbit {
    let g = c.g;
    let n = g + 1;
    let f = &c.fp;
    assert!((2..=4).contains(&g));
    assert_eq!(points.len(), n);
    assert_eq!(weights.len(), n);
    let a = product(points, 0, f);
    let mut basis = [[0; 5]; 5];
    let mut h = [0; 5];
    for i in 0..n {
        let ai = product(points, 1 << i, f);
        for k in 0..=g {
            basis[i][k] = f.mul(weights[i], ai[k]);
            h[k] = f.add(h[k], basis[i][k]);
        }
    }
    let mut cross = [[[0; 5]; 5]; 5];
    for i in 0..n {
        for j in i + 1..n {
            let b = product(points, (1 << i) | (1 << j), f);
            let scale = f.mul(4, f.mul(weights[i], weights[j]));
            for k in 0..g {
                let v = f.mul(scale, b[k]);
                cross[i][j][k] = v;
                cross[j][i][k] = v;
            }
        }
    }
    let mut q = [0; 5];
    q[g] = 1;
    for k in (0..g).rev() {
        let degree = g + 1 + k;
        let mut value = c.f.c[degree];
        // Only the upper half of h^2 contributes to the quotient.
        for i in degree - g..=g {
            value = f.sub(value, f.mul(h[i], h[degree - i]));
        }
        for j in k + 1..=g {
            value = f.sub(value, f.mul(a[degree - j], q[j]));
        }
        q[k] = value;
    }
    let mut out = Orbit {
        pairs: [(Poly::zero(), Poly::zero()); 16],
        len: 1 << g,
    };
    let mut negative = [false; 5];
    for step in 0..1usize << g {
        if step != 0 {
            let j = step.trailing_zeros() as usize + 1;
            for i in 0..n {
                if i != j {
                    for k in 0..g {
                        q[k] = if negative[i] == negative[j] {
                            f.add(q[k], cross[i][j][k])
                        } else {
                            f.sub(q[k], cross[i][j][k])
                        };
                    }
                }
            }
            for k in 0..=g {
                let twice = f.add(basis[j][k], basis[j][k]);
                h[k] = if negative[j] {
                    f.add(h[k], twice)
                } else {
                    f.sub(h[k], twice)
                };
            }
            negative[j] = !negative[j];
        }
        out.pairs[step] = (Poly::from_coeffs(&h[..=g]), Poly::from_coeffs(&q[..=g]));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anchor::orbit::{inverse_differences, signed_orbit_with_weights};
    use hec_native_ic::{
        instance::{factor_base, pick_instance, thread_curve},
        rng::Rng,
    };
    fn check(c: &Curve, points: &[(u64, u64)]) -> usize {
        let f = &c.fp;
        let n = points.len();
        let inverses = inverse_differences(&points.iter().map(|p| p.0).collect::<Vec<_>>(), f);
        let weights: Vec<_> = points
            .iter()
            .enumerate()
            .map(|(i, &(_, y))| {
                (0..n)
                    .filter(|&j| i != j)
                    .fold(y, |v, j| f.mul(v, inverses[i * n + j]))
            })
            .collect();
        let compact = signed_orbit(c, points, &weights);
        let reference = signed_orbit_with_weights(c, points, &weights);
        assert_eq!(compact.as_slice(), reference.as_slice());
        for (step, &(h, q)) in compact.as_slice().iter().enumerate() {
            let gray = step ^ (step >> 1);
            let (mut fresh, mut a) = (Poly::zero(), Poly::one());
            for (i, &(x, y)) in points.iter().enumerate() {
                let y = if i > 0 && gray & (1 << (i - 1)) != 0 {
                    f.neg(y)
                } else {
                    y
                };
                let value = f.mul(f.sub(y, fresh.eval(x, f)), f.inv(a.eval(x, f)));
                fresh = fresh.add(&a.scale(value, f), f);
                a = a.mul(&Poly::x_minus(x, f), f);
            }
            assert_eq!(h, fresh);
            let (expected, remainder) = c.f.sub(&fresh.mul(&fresh, f), f).divrem(&a, f);
            assert!(remainder.is_zero());
            assert_eq!(q, expected);
        }
        compact.len
    }
    #[test]
    fn random_orbits_equal_reference_and_full_newton_norm() {
        let mut rng = Rng::new(20261116);
        let mut count = 0;
        for (g, p) in [
            (2, 101),
            (2, 251),
            (2, 503),
            (2, 1009),
            (2, 2003),
            (2, 4001),
            (3, 61),
            (3, 211),
            (3, 401),
            (4, 31),
            (4, 61),
        ] {
            let i = pick_instance(p, g).unwrap();
            let fb = factor_base(&i.curve);
            for _ in 0..100 {
                let mut points = Vec::new();
                while points.len() < g + 1 {
                    let pt = fb[rng.below(fb.len() as u64) as usize];
                    if !points.iter().any(|&(x, _)| x == pt.0) {
                        points.push(pt);
                    }
                }
                count += check(&i.curve, &points);
            }
        }
        println!("COMPACT_RANDOM_PAIRS={count}");
    }
    fn tuples(c: &Curve, fb: &[(u64, u64)], start: usize, points: &mut Vec<(u64, u64)>) -> usize {
        if points.len() == c.g + 1 {
            return check(c, points);
        }
        let mut count = 0;
        for j in start..fb.len() {
            points.push(fb[j]);
            count += tuples(c, fb, j + 1, points);
            points.pop();
        }
        count
    }
    #[test]
    fn exhaustive_small_field_tuples_include_branch_places() {
        let mut count = 0;
        let mut branches = 0;
        for g in 2..=4 {
            for p in [11, 13] {
                // Enumerate every nonsingular member, not a timing-selected family.
                for coefficient in 1..p {
                    let Some(c) = thread_curve(p, coefficient, g) else {
                        continue;
                    };
                    let fb = factor_base(&c);
                    branches += fb.iter().filter(|p| p.1 == 0).count();
                    count += tuples(&c, &fb, 0, &mut Vec::new());
                }
            }
        }
        assert!(branches > 0);
        assert!(count > 0);
        println!("COMPACT_EXHAUSTIVE_PAIRS={count} BRANCH_PLACES={branches}");
    }
}
