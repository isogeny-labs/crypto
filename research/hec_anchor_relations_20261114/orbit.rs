//! Signed interpolation orbit; one norm division per tuple.
use hec_native_ic::{fp::Fp, jac::Curve, poly::Poly};

/// Inverse differences in row-major order, diagonal zero, one field inversion.
pub fn inverse_differences(xs: &[u64], f: &Fp) -> Vec<u64> {
    let n = xs.len();
    let mut out = vec![0; n * n];
    let mut prefixes = vec![1; n * n];
    let mut product = 1;
    for i in 0..n {
        for j in 0..n {
            if i != j {
                let at = i * n + j;
                prefixes[at] = product;
                let difference = f.sub(xs[i], xs[j]);
                assert_ne!(difference, 0, "distinct anchors");
                product = f.mul(product, difference);
            }
        }
    }
    let mut inverse = f.inv(product);
    for i in (0..n).rev() {
        for j in (0..n).rev() {
            if i != j {
                let at = i * n + j;
                out[at] = f.mul(inverse, prefixes[at]);
                inverse = f.mul(inverse, f.sub(xs[i], xs[j]));
            }
        }
    }
    out
}

#[cfg(test)]
pub fn signed_orbit(c: &Curve, points: &[(u64, u64)]) -> Vec<(Poly, Poly)> {
    let xs: Vec<_> = points.iter().map(|p| p.0).collect();
    let inv = inverse_differences(&xs, &c.fp);
    let n = points.len();
    let weights: Vec<_> = points
        .iter()
        .enumerate()
        .map(|(i, &(_, y))| {
            (0..n)
                .filter(|&j| i != j)
                .fold(y, |w, j| c.fp.mul(w, inv[i * n + j]))
        })
        .collect();
    signed_orbit_with_weights(c, points, &weights)
}

pub fn signed_orbit_with_weights(
    c: &Curve,
    points: &[(u64, u64)],
    weights: &[u64],
) -> Vec<(Poly, Poly)> {
    let f = &c.fp;
    assert_eq!(points.len(), c.g + 1);
    let n = points.len();
    let mut a = Poly::one();
    for &(x, _) in points {
        a = a.mul(&Poly::x_minus(x, f), f);
    }
    assert_eq!(weights.len(), n);
    let mut basis = Vec::with_capacity(n);
    let mut h = Poly::zero();
    for (i, &(x, _)) in points.iter().enumerate() {
        let ai = a.div_exact(&Poly::x_minus(x, f), f);
        let w = weights[i];
        let hi = ai.scale(w, f);
        basis.push(hi);
        h = h.add(&hi, f);
    }
    let mut pair = vec![vec![Poly::zero(); n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let mut b = Poly::one();
            for (k, &(x, _)) in points.iter().enumerate() {
                if k != i && k != j {
                    b = b.mul(&Poly::x_minus(x, f), f);
                }
            }
            let b = b.scale(f.mul(4, f.mul(weights[i], weights[j])), f);
            pair[i][j] = b;
            pair[j][i] = b;
        }
    }
    let mut q = c.f.sub(&h.mul(&h, f), f).div_exact(&a, f);
    let mut out = vec![(h, q)];
    let mut negative = vec![false; n];
    for step in 1usize..1usize << c.g {
        // Reflected Gray order: coordinate zero stays positive.
        let j = step.trailing_zeros() as usize + 1;
        for i in 0..n {
            if i != j {
                q = if negative[i] == negative[j] {
                    q.add(&pair[i][j], f)
                } else {
                    q.sub(&pair[i][j], f)
                };
            }
        }
        let twice = basis[j].scale(2, f);
        h = if negative[j] {
            h.add(&twice, f)
        } else {
            h.sub(&twice, f)
        };
        negative[j] = !negative[j];
        out.push((h, q));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use hec_native_ic::{
        instance::{factor_base, pick_instance},
        rng::Rng,
    };

    #[test]
    fn every_gray_residual_matches_fresh_newton_interpolation() {
        let mut rng = Rng::new(20261115);
        for (g, p) in [(2, 101), (2, 4001), (3, 61), (3, 401), (4, 31), (4, 61)] {
            let instance = pick_instance(p, g).unwrap();
            let c = &instance.curve;
            let f = &c.fp;
            let fb = factor_base(c);
            for _ in 0..100 {
                let mut points = Vec::new();
                while points.len() < g + 1 {
                    let pt = fb[rng.below(fb.len() as u64) as usize];
                    if !points.iter().any(|&(x, _)| x == pt.0) {
                        points.push(pt);
                    }
                }
                for (step, (h, q)) in signed_orbit(c, &points).into_iter().enumerate() {
                    let gray = step ^ (step >> 1);
                    let (mut fresh, mut a) = (Poly::zero(), Poly::one());
                    for (j, &(x, y)) in points.iter().enumerate() {
                        let neg = j > 0 && gray & (1 << (j - 1)) != 0;
                        let y = if neg { f.neg(y) } else { y };
                        let k = f.mul(f.sub(y, fresh.eval(x, f)), f.inv(a.eval(x, f)));
                        fresh = fresh.add(&a.scale(k, f), f);
                        a = a.mul(&Poly::x_minus(x, f), f);
                    }
                    assert_eq!(h, fresh, "g={g} p={p} step={step}");
                    let (expected, remainder) = c.f.sub(&fresh.mul(&fresh, f), f).divrem(&a, f);
                    assert!(remainder.is_zero());
                    assert_eq!(q, expected, "g={g} p={p} step={step}");
                }
            }
        }
    }
}
