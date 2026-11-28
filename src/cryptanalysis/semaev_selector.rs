//! Division-free elimination for a quadratic Semaev equation and a split selector.
//!
//! For `q(Y) = a(X) Y^2 + b(X) Y + c(X)`, put `s = -b`, `z = ac`, and
//! `T_0 = 2`, `T_1 = s`, `T_(n+2) = s T_(n+1) - z T_n`. The monic-selector
//! resultant is `Res_Y(Y^d - 1, q) = a^d + c^d - T_d`. Binary doubling
//! computes it in `F_p[X]/(X^d - 1)` without dividing by `a`. In particular,
//! `a = (X - target_x)^2` need not be a unit in that quotient ring.
//!
//! The coordinate extractor solves both selector constraints, then solves a
//! quadratic only at roots of the support gcd. It does not enumerate pair
//! sums. Actual rational points, signs and a declared order annihilator are
//! checked separately by [`lift_selector_pairs`].
//!
//! The current reused polynomial backend is schoolbook. The norm construction
//! therefore has an `O(d^2 log d)` field-operation upper bound and `O(d)`
//! live polynomial storage, rather than a fast-multiplication performance
//! claim. Gcd, root extraction, candidate output and signed lifting add work.
//! The reused root finder uses Las Vegas splitting with no deterministic
//! retry bound. No benchmark or complete solving-cost claim is made here.

use std::collections::BTreeSet;

use super::pkm_smooth_tower::{is_prime, poly_roots, sqrtm, Curve, Pt, Rng};
use super::semaev_prime_poly::{poly_gcd, poly_mulmod, poly_powmod, poly_rem, trim, Fp};
use super::semaev_projection::{
    quadratic_root_keys, s3_quadratic, QuadraticRootKey, QuadraticRoots,
};

/// Explicit allocation/work-domain limit for this development component.
pub const MAX_SELECTOR_DEGREE: usize = 4096;

/// Refusal or failed consistency check, distinct from an empty solution set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorError {
    /// Only odd prime fields below `2^32` are supported.
    UnsupportedField,
    /// The selector degree must be between one and the explicit limit.
    UnsupportedDegree,
    /// A split selector requires `d | p - 1`.
    SelectorDoesNotSplit,
    /// A coefficient vector is empty, too long, or contains a residue >= p.
    InvalidPolynomial,
    /// Curve coefficients, target coordinates, or supplied pair coordinates
    /// must use their canonical representatives in `[0,p)`.
    NoncanonicalInput,
    SingularCurve,
    /// Point lifting requires a finite, rational target on the given curve.
    InvalidTarget,
    /// A requested order annihilator must be at least two.
    InvalidOrder,
    /// The supplied target is not annihilated by the requested order.
    TargetOutsideOrderKernel,
    /// An internal algebraic extraction check failed. This is not UNSAT.
    InconsistentExtraction,
}

impl std::fmt::Display for SelectorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for SelectorError {}

/// Algebraic output before rational-point and subgroup filtering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorExtraction {
    pub selector_degree: usize,
    /// The resultant reduced modulo `X^d - 1`, coefficients low to high.
    pub resultant: Vec<u64>,
    /// Monic gcd of that resultant with `X^d - 1`.
    pub support_gcd: Vec<u64>,
    /// All ordered coordinate pairs satisfying `S3(x1,x2,target_x)=0` and
    /// `x1^d=x2^d=1`, sorted and deduplicated. These are not yet relations.
    pub coordinate_pairs: Vec<(u64, u64)>,
}

/// Actual finite points whose sum has been replayed against the public target.
/// Both orientations are retained when their ordered coordinate pairs occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SignedPair {
    pub left: (u64, u64),
    pub right: (u64, u64),
}

fn validate_field(f: Fp) -> Result<(), SelectorError> {
    if f.p < 3 || f.p >= (1u64 << 32) || !is_prime(f.p) {
        return Err(SelectorError::UnsupportedField);
    }
    Ok(())
}

fn validate_degree(d: usize) -> Result<(), SelectorError> {
    if d == 0 || d > MAX_SELECTOR_DEGREE {
        return Err(SelectorError::UnsupportedDegree);
    }
    Ok(())
}

fn validate_curve(curve: &Curve) -> Result<Fp, SelectorError> {
    let f = Fp { p: curve.p };
    validate_field(f)?;
    if curve.a >= curve.p || curve.b >= curve.p {
        return Err(SelectorError::NoncanonicalInput);
    }
    if curve.is_singular() {
        return Err(SelectorError::SingularCurve);
    }
    Ok(f)
}

fn selector(f: Fp, d: usize) -> Vec<u64> {
    let mut l = vec![0; d + 1];
    l[0] = f.p - 1;
    l[d] = 1;
    l
}

fn add(f: Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (i, c) in out.iter_mut().enumerate() {
        *c = f.add(
            a.get(i).copied().unwrap_or(0),
            b.get(i).copied().unwrap_or(0),
        );
    }
    trim(out)
}

fn sub(f: Fp, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (i, c) in out.iter_mut().enumerate() {
        *c = f.sub(
            a.get(i).copied().unwrap_or(0),
            b.get(i).copied().unwrap_or(0),
        );
    }
    trim(out)
}

fn scale(f: Fp, a: &[u64], c: u64) -> Vec<u64> {
    trim(a.iter().map(|&x| f.mul(x, c)).collect())
}

fn eval(f: Fp, a: &[u64], x: u64) -> u64 {
    a.iter().rev().fold(0, |v, &c| f.add(f.mul(v, x), c))
}

/// `T_n` for `T_0=2, T_1=s, T_(n+2)=s*T_(n+1)-z*T_n`, modulo `l`.
/// All three loop values use the same processed exponent.
fn lucas(f: Fp, s: &[u64], z: &[u64], n: u64, l: &[u64]) -> Vec<u64> {
    let mut t = vec![2 % f.p];
    let mut next = s.to_vec();
    let mut z_power = vec![1];
    let bits = u64::BITS - n.leading_zeros();
    for bit in (0..bits).rev() {
        let twice_z = scale(f, &z_power, 2);
        let even = sub(f, &poly_mulmod(f, &t, &t, l), &twice_z);
        let odd = sub(
            f,
            &poly_mulmod(f, &t, &next, l),
            &poly_mulmod(f, s, &z_power, l),
        );
        let z_even = poly_mulmod(f, &z_power, &z_power, l);
        if (n >> bit) & 1 == 0 {
            t = even;
            next = odd;
            z_power = z_even;
        } else {
            let z_next = poly_mulmod(f, &z_power, z, l);
            let next_even = sub(f, &poly_mulmod(f, &next, &next, l), &scale(f, &z_next, 2));
            t = odd;
            next = next_even;
            z_power = poly_mulmod(f, &z_even, z, l);
        }
    }
    t
}

/// Compute `Res_Y(Y^d - 1, a(X)Y^2+b(X)Y+c(X)) mod (X^d - 1)`.
///
/// Coefficients are low to high and must be canonical. Each input vector is
/// bounded by `MAX_SELECTOR_DEGREE+1`; leading zeros are allowed. Unlike the
/// coordinate extractor, this algebraic identity does not require `d | p-1`.
/// It handles zero leading coefficients and the identically zero quadratic.
/// No inverse of `a(X)` is computed.
pub fn quadratic_selector_resultant(
    f: Fp,
    a: &[u64],
    b: &[u64],
    c: &[u64],
    d: usize,
) -> Result<Vec<u64>, SelectorError> {
    validate_field(f)?;
    validate_degree(d)?;
    for polynomial in [a, b, c] {
        if polynomial.is_empty()
            || polynomial.len() > MAX_SELECTOR_DEGREE + 1
            || polynomial.iter().any(|&coefficient| coefficient >= f.p)
        {
            return Err(SelectorError::InvalidPolynomial);
        }
    }
    let l = selector(f, d);
    let a = poly_rem(f, a, &l);
    let b = poly_rem(f, b, &l);
    let c = poly_rem(f, c, &l);
    let s: Vec<_> = b.iter().map(|&v| f.neg(v)).collect();
    let z = poly_mulmod(f, &a, &c, &l);
    let t = lucas(f, &s, &z, d as u64, &l);
    Ok(sub(
        f,
        &add(
            f,
            &poly_powmod(f, &a, d as u64, &l),
            &poly_powmod(f, &c, d as u64, &l),
        ),
        &t,
    ))
}

/// Extract all ordered coordinate solutions for a split multiplicative selector.
///
/// The target coordinate is public and canonical. It need not itself lift to
/// a rational curve point at this algebraic stage; [`lift_selector_pairs`]
/// imposes that requirement. Refusals return an error, never an empty set.
/// Root extraction reuses the repository's Las Vegas polynomial splitter.
pub fn selector_coordinate_pairs(
    curve: &Curve,
    target_x: u64,
    d: usize,
    seed: u64,
) -> Result<SelectorExtraction, SelectorError> {
    let f = validate_curve(curve)?;
    validate_degree(d)?;
    if target_x >= f.p {
        return Err(SelectorError::NoncanonicalInput);
    }
    if !(f.p - 1).is_multiple_of(d as u64) {
        return Err(SelectorError::SelectorDoesNotSplit);
    }

    // S3(X,Y,t) as a quadratic in Y, without normalizing its leading term.
    let t = target_x;
    let a = vec![f.mul(t, t), f.neg(f.mul(2, t)), 1];
    let b = vec![
        f.neg(f.mul(2, f.add(f.mul(curve.a, t), f.mul(2, curve.b)))),
        f.neg(f.mul(2, f.add(f.mul(t, t), curve.a))),
        f.neg(f.mul(2, t)),
    ];
    let c = vec![
        f.sub(f.mul(curve.a, curve.a), f.mul(f.mul(4 % f.p, curve.b), t)),
        f.sub(f.neg(f.mul(2, f.mul(curve.a, t))), f.mul(4 % f.p, curve.b)),
        f.mul(t, t),
    ];
    let l = selector(f, d);
    let resultant = quadratic_selector_resultant(f, &a, &b, &c, d)?;
    let mut support_gcd = poly_gcd(f, &l, &resultant);
    let inverse_lead = f.inv(
        *support_gcd
            .last()
            .ok_or(SelectorError::InconsistentExtraction)?,
    );
    support_gcd = scale(f, &support_gcd, inverse_lead);
    let mut rng = Rng(seed);
    let x_roots = poly_roots(&support_gcd, f.p, &mut rng);
    // L is squarefree and split. Therefore every gcd factor must be linear;
    // a degree/root-count mismatch is an extraction failure, not UNSAT.
    if x_roots.len() + 1 != support_gcd.len() {
        return Err(SelectorError::InconsistentExtraction);
    }
    let mut all_selector_roots: Option<Vec<u64>> = None;
    let mut pairs = BTreeSet::new();
    for x in x_roots {
        if f.pow(x, d as u64) != 1 || eval(f, &support_gcd, x) != 0 {
            return Err(SelectorError::InconsistentExtraction);
        }
        let quadratic = s3_quadratic(curve, x, t);
        let roots = match quadratic_root_keys(quadratic, f.p) {
            QuadraticRoots::None => Vec::new(),
            QuadraticRoots::Finite(keys) => keys
                .into_iter()
                .filter_map(|key| match key {
                    QuadraticRootKey::Base(y) => Some(y),
                    QuadraticRootKey::Irreducible { .. } => None,
                })
                .collect(),
            QuadraticRoots::All => {
                if all_selector_roots.is_none() {
                    let roots = poly_roots(&l, f.p, &mut rng);
                    if roots.len() != d || roots.iter().any(|&y| f.pow(y, d as u64) != 1) {
                        return Err(SelectorError::InconsistentExtraction);
                    }
                    all_selector_roots = Some(roots);
                }
                all_selector_roots.as_ref().unwrap().clone()
            }
        };
        let mut found = false;
        for y in roots {
            if f.pow(y, d as u64) == 1 {
                if eval(f, &quadratic, y) != 0 {
                    return Err(SelectorError::InconsistentExtraction);
                }
                pairs.insert((x, y));
                found = true;
            }
        }
        if !found {
            return Err(SelectorError::InconsistentExtraction);
        }
    }
    Ok(SelectorExtraction {
        selector_degree: d,
        resultant,
        support_gcd,
        coordinate_pairs: pairs.into_iter().collect(),
    })
}

fn ordinate_signs(y: u64, p: u64) -> Vec<u64> {
    if y == 0 {
        vec![0]
    } else {
        vec![y, p - y]
    }
}

/// Lift supplied ordered coordinate pairs and retain each rational signed pair
/// whose sum equals the supplied public target.
///
/// Selector membership is established by [`selector_coordinate_pairs`], not
/// by this standalone lifting function. Non-lifting coordinates are rejected
/// as candidates. Noncanonical coordinates refuse the whole call.
///
/// If `required_order=Some(r)`, the target and both summands must satisfy
/// `[r]P=O`. This is an **annihilator check**, not a proof of exact order or
/// membership in a particular cyclic subgroup: the caller must establish the
/// uniqueness of that subgroup (as on the registered prime-order Prime31
/// instance) or provide a separate membership certificate. Target infinity
/// is explicitly outside this two-finite-point target interface.
pub fn lift_selector_pairs(
    curve: &Curve,
    target: Pt,
    pairs: &[(u64, u64)],
    required_order: Option<u64>,
) -> Result<Vec<SignedPair>, SelectorError> {
    validate_curve(curve)?;
    let (tx, ty) = target.ok_or(SelectorError::InvalidTarget)?;
    if tx >= curve.p || ty >= curve.p {
        return Err(SelectorError::NoncanonicalInput);
    }
    if !curve.on_curve(target) {
        return Err(SelectorError::InvalidTarget);
    }
    if let Some(r) = required_order {
        if r < 2 {
            return Err(SelectorError::InvalidOrder);
        }
        if curve.mul(r, target).is_some() {
            return Err(SelectorError::TargetOutsideOrderKernel);
        }
    }
    if pairs.iter().any(|&(x1, x2)| x1 >= curve.p || x2 >= curve.p) {
        return Err(SelectorError::NoncanonicalInput);
    }
    let mut accepted = BTreeSet::new();
    for &(x1, x2) in pairs {
        let (Some(y1), Some(y2)) = (sqrtm(curve.rhs(x1), curve.p), sqrtm(curve.rhs(x2), curve.p))
        else {
            continue;
        };
        if let Some(r) = required_order {
            if curve.mul(r, Some((x1, y1))).is_some() || curve.mul(r, Some((x2, y2))).is_some() {
                continue;
            }
        }
        for signed_y1 in ordinate_signs(y1, curve.p) {
            for signed_y2 in ordinate_signs(y2, curve.p) {
                let left = (x1, signed_y1);
                let right = (x2, signed_y2);
                if curve.on_curve(Some(left))
                    && curve.on_curve(Some(right))
                    && curve.add(Some(left), Some(right)) == target
                {
                    accepted.insert(SignedPair { left, right });
                }
            }
        }
    }
    Ok(accepted.into_iter().collect())
}
