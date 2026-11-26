//! Separately authored symmetric-Semaev reference for public M31 fixtures.
//!
//! This module intentionally does not import the chord extractor or the
//! repository arithmetic adapter. It recovers symmetric coordinates first,
//! lifts each ordinate independently, and filters with its own affine group
//! law. The caller supplies an affine target with nonzero ordinate.

const P: u64 = 2_147_483_647;
type Affine = (u64, u64);
type Pair = (Affine, Affine);

#[inline]
fn add(x: u64, y: u64) -> u64 {
    (x + y) % P
}

#[inline]
fn sub(x: u64, y: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        P - (y - x)
    }
}

#[inline]
fn mul(x: u64, y: u64) -> u64 {
    ((u128::from(x) * u128::from(y)) % u128::from(P)) as u64
}

#[inline]
fn neg(x: u64) -> u64 {
    if x == 0 {
        0
    } else {
        P - x
    }
}

fn pow(mut x: u64, mut exponent: u64) -> u64 {
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            out = mul(out, x);
        }
        exponent >>= 1;
        if exponent != 0 {
            x = mul(x, x);
        }
    }
    out
}

fn inverse(x: u64) -> u64 {
    assert_ne!(x, 0, "reference inversion of zero");
    pow(x, P - 2)
}

/// All square roots, in increasing order and without multiplicity.
fn square_roots(x: u64) -> Vec<u64> {
    let root = pow(x, (P + 1) / 4);
    if mul(root, root) != x {
        return Vec::new();
    }
    if root == 0 {
        return vec![0];
    }
    let other = P - root;
    if root < other {
        vec![root, other]
    } else {
        vec![other, root]
    }
}

/// Roots of aa*X^2 + bb*X + cc, including its linear specialization.
fn polynomial_roots(aa: u64, bb: u64, cc: u64) -> Vec<u64> {
    if aa == 0 {
        if bb != 0 {
            return vec![mul(neg(cc), inverse(bb))];
        }
        assert_ne!(
            cc, 0,
            "reference encountered an identically zero polynomial"
        );
        return Vec::new();
    }
    let discriminant = sub(mul(bb, bb), mul(4, mul(aa, cc)));
    let inv_two_a = inverse(mul(2, aa));
    let mut roots: Vec<_> = square_roots(discriminant)
        .into_iter()
        .map(|root| mul(add(neg(bb), root), inv_two_a))
        .collect();
    roots.sort_unstable();
    roots.dedup();
    roots
}

fn rhs(a: u64, b: u64, x: u64) -> u64 {
    add(add(mul(mul(x, x), x), mul(a, x)), b)
}

/// Independent affine addition. None denotes the identity.
fn point_sum(a: u64, left: Affine, right: Affine) -> Option<Affine> {
    let (x1, y1) = left;
    let (x2, y2) = right;
    let slope = if x1 == x2 {
        if add(y1, y2) == 0 {
            return None;
        }
        // For two valid curve points with equal x and nonopposite y,
        // the ordinates are equal and the operation is doubling.
        assert_eq!(y1, y2);
        mul(add(mul(3, mul(x1, x1)), a), inverse(mul(2, y1)))
    } else {
        mul(sub(y2, y1), inverse(sub(x2, x1)))
    };
    let x3 = sub(sub(mul(slope, slope), x1), x2);
    let y3 = sub(mul(slope, sub(x1, x3)), y1);
    Some((x3, y3))
}

/// Recover every affine ordered pair whose sum is the supplied target and
/// whose symmetric coordinates satisfy the supplied Cayley constraint.
///
/// Preconditions: canonical a,b,w,y in F_P; y != 0; (w,y) is on the curve;
/// the curve is nonsingular (the harness certifies this). Some(lambda)
/// imposes q=1-lambda*s. None imposes s=0 and denotes Cayley product one.
///
/// Identities are deliberately outside this affine interface. Repeated
/// summands are retained. Returned pairs are sorted and deduplicated.
pub fn extract(a: u64, b: u64, w: u64, y: u64, lambda: Option<u64>) -> Vec<Pair> {
    assert!(a < P && b < P && w < P && y < P && y != 0);
    assert_eq!(
        mul(y, y),
        rhs(a, b, w),
        "reference target is not on the curve"
    );
    assert!(lambda.is_none_or(|value| value < P));

    let w_squared = mul(w, w);
    let symmetric_pairs: Vec<(u64, u64)> = match lambda {
        Some(lam) => {
            let one_minus_a = sub(1, a);
            let leading = mul(add(w, lam), add(w, lam));
            let linear = sub(
                sub(
                    sub(mul(4, mul(lam, w_squared)), mul(2, mul(w, add(1, a)))),
                    mul(2, mul(lam, one_minus_a)),
                ),
                mul(4, b),
            );
            let constant = sub(
                sub(mul(one_minus_a, one_minus_a), mul(4, w_squared)),
                mul(4, mul(b, w)),
            );
            polynomial_roots(leading, linear, constant)
                .into_iter()
                .map(|sum| (sum, sub(1, mul(lam, sum))))
                .collect()
        }
        None => {
            let linear = neg(add(mul(4, w_squared), mul(2, a)));
            let constant = sub(mul(a, a), mul(4, mul(b, w)));
            polynomial_roots(1, linear, constant)
                .into_iter()
                .map(|product| (0, product))
                .collect()
        }
    };

    let mut out = Vec::new();
    for (sum, product) in symmetric_pairs {
        for x1 in polynomial_roots(1, neg(sum), product) {
            let x2 = sub(sum, x1);
            assert_eq!(mul(x1, x2), product);
            let roots1 = square_roots(rhs(a, b, x1));
            let roots2 = square_roots(rhs(a, b, x2));
            for &y1 in &roots1 {
                for &y2 in &roots2 {
                    let left = (x1, y1);
                    let right = (x2, y2);
                    if point_sum(a, left, right) == Some((w, y)) {
                        out.push((left, right));
                        out.push((right, left));
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}
