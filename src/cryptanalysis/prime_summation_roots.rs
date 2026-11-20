//! Structure-aware roots of specialized prime-field Semaev summation polynomials.
//!
//! For lifted points `P_1, ..., P_k` on a short-Weierstrass curve, the finite
//! roots of the point-origin specialization are the x-coordinates of
//! `P_1 + e_2 P_2 + ... + e_k P_k`, where each `e_i` is `-1` or `+1`.
//!
//! This is not a generic factorizer for an arbitrary coefficient vector.  It
//! also excludes point lifting, relation collection, and end-to-end ECDLP cost.

use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use super::prime_fast::{BatchAddScratch, FastCurve, Fp64, Pt};

/// Imported conservative crossover for coefficient construction.  Callers can
/// select either method explicitly; this is not a per-host calibration.
pub const PRODUCT_TREE_AUTO_THRESHOLD: usize = 256;

/// A public affine point in canonical (non-Montgomery) coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalPoint {
    pub x: u64,
    pub y: u64,
}

/// Finite roots retain multiplicity. Infinite signed sums are counted
/// separately because they lower the degree of the affine specialization.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SummationRootSet {
    pub finite_x: Vec<u64>,
    pub infinity_multiplicity: usize,
}

impl SummationRootSet {
    pub fn sort_finite(&mut self) {
        self.finite_x.sort_unstable();
    }

    pub fn distinct_finite_count(&self) -> usize {
        if self.finite_x.is_empty() {
            return 0;
        }
        let mut sorted = self.finite_x.clone();
        sorted.sort_unstable();
        1 + sorted.windows(2).filter(|w| w[0] != w[1]).count()
    }
}

/// Reusable storage for repeated all-root and target queries.
#[derive(Default)]
pub struct SummationRootWorkspace {
    current: Vec<Option<Pt>>,
    left: Vec<Option<Pt>>,
    right: Vec<Option<Pt>>,
    gray_left: Vec<Option<Pt>>,
    gray_right: Vec<Option<Pt>>,
    batch: BatchAddScratch,
    right_index: HashMap<Option<Pt>, RightWitness>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RightWitness {
    sign_mask: u64,
    canonical_negated: bool,
}

/// Coefficient construction for `prod (X - r_i)`, low degree first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoefficientMethod {
    Auto,
    Quadratic,
    ProductTree,
}

impl CoefficientMethod {
    pub fn resolved(self, roots: usize) -> Self {
        match self {
            Self::Auto if roots < PRODUCT_TREE_AUTO_THRESHOLD => Self::Quadratic,
            Self::Auto => Self::ProductTree,
            explicit => explicit,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Quadratic => "quadratic",
            Self::ProductTree => "product-tree",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetSearchResult {
    pub target_count: usize,
    pub liftable_targets: usize,
    pub unliftable_targets: usize,
    pub left_count: usize,
    /// Nominal right-half signed sums before the global-negation quotient.
    pub right_signed_count: usize,
    /// Stored right-half representatives after the global-negation quotient.
    pub right_count: usize,
    pub matched_target_x: Option<u64>,
    pub sign_mask: Option<u64>,
    pub sum: Option<CanonicalPoint>,
}

impl TargetSearchResult {
    pub fn found(&self) -> bool {
        self.matched_target_x.is_some()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SummationRootError {
    NoInputPoints,
    TooManySignBits(usize),
    StateLimit { requested: usize, limit: usize },
    AllocationFailed(usize),
    InvalidPoint(usize),
    UnliftableInput(u64),
    SignMaskOutOfRange { mask: u64, sign_bits: usize },
}

impl Display for SummationRootError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoInputPoints => f.write_str("at least one lifted input point is required"),
            Self::TooManySignBits(bits) => {
                write!(f, "{bits} sign bits do not fit the u64 certificate format")
            }
            Self::StateLimit { requested, limit } => {
                write!(
                    f,
                    "query needs {requested} signed states, above limit {limit}"
                )
            }
            Self::AllocationFailed(states) => {
                write!(f, "could not allocate storage for {states} signed states")
            }
            Self::InvalidPoint(index) => write!(f, "input point {index} is not on the curve"),
            Self::UnliftableInput(x) => {
                write!(f, "x={x} is not a point x-coordinate on the curve")
            }
            Self::SignMaskOutOfRange { mask, sign_bits } => write!(
                f,
                "sign mask {mask} uses bits outside the {sign_bits}-bit input certificate"
            ),
        }
    }
}

impl Error for SummationRootError {}

fn checked_state_count(bits: usize, limit: usize) -> Result<usize, SummationRootError> {
    if bits >= u64::BITS as usize || bits >= usize::BITS as usize {
        return Err(SummationRootError::TooManySignBits(bits));
    }
    let requested = 1usize << bits;
    if requested > limit {
        return Err(SummationRootError::StateLimit { requested, limit });
    }
    Ok(requested)
}

fn validate_points(curve: &FastCurve, points: &[Pt]) -> Result<(), SummationRootError> {
    if points.is_empty() {
        return Err(SummationRootError::NoInputPoints);
    }
    for (index, &point) in points.iter().enumerate() {
        if !curve.is_on_curve(point) {
            return Err(SummationRootError::InvalidPoint(index));
        }
    }
    Ok(())
}

/// Lift canonical x-coordinates with [`FastCurve`]'s deterministic y-sign
/// convention, rejecting the first non-residue.
pub fn lift_input_xs(curve: &FastCurve, xs: &[u64]) -> Result<Vec<Pt>, SummationRootError> {
    if xs.is_empty() {
        return Err(SummationRootError::NoInputPoints);
    }
    xs.iter()
        .map(|&x| {
            curve
                .lift_x(x)
                .ok_or(SummationRootError::UnliftableInput(x))
        })
        .collect()
}

/// Expand every signed sum with one batched affine addition per level.
/// `max_states` is an explicit caller memory bound on the nominal root count.
pub fn all_roots_into(
    curve: &FastCurve,
    points: &[Pt],
    max_states: usize,
    workspace: &mut SummationRootWorkspace,
    out: &mut SummationRootSet,
) -> Result<(), SummationRootError> {
    validate_points(curve, points)?;
    checked_state_count(points.len() - 1, max_states)?;

    workspace.current.clear();
    workspace.current.push(Some(points[0]));
    for &point in &points[1..] {
        let next_len = workspace.current.len() * 2;
        workspace.left.clear();
        workspace.right.clear();
        workspace
            .left
            .try_reserve_exact(next_len)
            .map_err(|_| SummationRootError::AllocationFailed(next_len))?;
        workspace
            .right
            .try_reserve_exact(next_len)
            .map_err(|_| SummationRootError::AllocationFailed(next_len))?;
        let negated = curve.neg(point);
        for &sum in &workspace.current {
            workspace.left.push(sum);
            workspace.right.push(Some(point));
            workspace.left.push(sum);
            workspace.right.push(Some(negated));
        }
        curve.add_many_batched(&mut workspace.left, &workspace.right, &mut workspace.batch);
        std::mem::swap(&mut workspace.current, &mut workspace.left);
    }

    out.finite_x.clear();
    out.infinity_multiplicity = 0;
    out.finite_x
        .try_reserve_exact(workspace.current.len())
        .map_err(|_| SummationRootError::AllocationFailed(workspace.current.len()))?;
    for &sum in &workspace.current {
        match sum {
            Some(point) => out.finite_x.push(curve.canonical(point).0),
            None => out.infinity_multiplicity += 1,
        }
    }
    Ok(())
}

pub fn all_roots(
    curve: &FastCurve,
    points: &[Pt],
    max_states: usize,
) -> Result<SummationRootSet, SummationRootError> {
    let mut workspace = SummationRootWorkspace::default();
    let mut out = SummationRootSet::default();
    all_roots_into(curve, points, max_states, &mut workspace, &mut out)?;
    Ok(out)
}

/// One certified signed sum. Bit `j-1` selects `+P_j` when set and
/// `-P_j` when clear; `P_0` is fixed positive.
pub fn one_root(
    curve: &FastCurve,
    points: &[Pt],
    sign_mask: u64,
) -> Result<Option<CanonicalPoint>, SummationRootError> {
    validate_points(curve, points)?;
    let sign_bits = points.len() - 1;
    if sign_bits >= u64::BITS as usize {
        return Err(SummationRootError::TooManySignBits(sign_bits));
    }
    if (sign_mask >> sign_bits) != 0 {
        return Err(SummationRootError::SignMaskOutOfRange {
            mask: sign_mask,
            sign_bits,
        });
    }
    let mut sum = Some(points[0]);
    for (bit, &point) in points[1..].iter().enumerate() {
        let term = if (sign_mask >> bit) & 1 == 1 {
            point
        } else {
            curve.neg(point)
        };
        sum = curve.add(sum, Some(term));
    }
    Ok(sum.map(|point| {
        let (x, y) = curve.canonical(point);
        CanonicalPoint { x, y }
    }))
}

fn enumerate_gray_into(
    curve: &FastCurve,
    base: Option<Pt>,
    vars: &[Pt],
    limit: usize,
    out: &mut Vec<Option<Pt>>,
) -> Result<(), SummationRootError> {
    let count = checked_state_count(vars.len(), limit)?;
    out.clear();
    out.try_reserve_exact(count)
        .map_err(|_| SummationRootError::AllocationFailed(count))?;
    let mut sum = base;
    let mut twice = Vec::new();
    twice
        .try_reserve_exact(vars.len())
        .map_err(|_| SummationRootError::AllocationFailed(vars.len()))?;
    for &point in vars {
        sum = curve.add(sum, Some(curve.neg(point)));
        twice.push(curve.double(point));
    }
    out.push(sum);
    let mut old_gray = 0usize;
    for i in 1..count {
        let gray = i ^ (i >> 1);
        let changed = gray ^ old_gray;
        let bit = changed.trailing_zeros() as usize;
        let step = if (gray >> bit) & 1 == 1 {
            twice[bit]
        } else {
            twice[bit].map(|point| curve.neg(point))
        };
        sum = curve.add(sum, step);
        out.push(sum);
        old_gray = gray;
    }
    Ok(())
}

fn target_split_bits(sign_bits: usize, targets: usize) -> usize {
    let targets = targets.max(1) as u128;
    let mut best = 0;
    let mut best_cost = u128::MAX;
    for left_bits in 0..=sign_bits {
        let right_bits = sign_bits - left_bits;
        let left = 1u128 << left_bits;
        let right = 1u128 << right_bits.saturating_sub(1);
        let cost = right.saturating_add(2u128.saturating_mul(targets).saturating_mul(left));
        if cost < best_cost {
            best_cost = cost;
            best = left_bits;
        }
    }
    best
}

fn negation_key(curve: &FastCurve, point: Option<Pt>) -> (Option<Pt>, bool) {
    match point {
        Some(point) => {
            let (canonical, negated) = curve.canonical_sign(point);
            (Some(canonical), negated)
        }
        None => (None, false),
    }
}

/// Search liftable target x-coordinates without materializing every root.
/// A hit returns a replayable sign-mask certificate.
pub fn find_target_root(
    curve: &FastCurve,
    points: &[Pt],
    target_xs: &[u64],
    max_states: usize,
    workspace: &mut SummationRootWorkspace,
) -> Result<TargetSearchResult, SummationRootError> {
    validate_points(curve, points)?;
    let sign_bits = points.len() - 1;
    if sign_bits >= u64::BITS as usize || sign_bits >= usize::BITS as usize {
        return Err(SummationRootError::TooManySignBits(sign_bits));
    }

    let mut targets = Vec::new();
    for &x in target_xs {
        if let Some(point) = curve.lift_x(x) {
            targets.push((x, point));
        }
    }
    let unliftable_targets = target_xs.len() - targets.len();
    let left_bits = target_split_bits(sign_bits, targets.len());
    let right_bits = sign_bits - left_bits;
    let left_count = checked_state_count(left_bits, max_states)?;
    let right_signed_count = 1usize << right_bits;
    let right_count = checked_state_count(right_bits.saturating_sub(1), max_states)?;
    if left_count.saturating_add(right_count) > max_states {
        return Err(SummationRootError::StateLimit {
            requested: left_count.saturating_add(right_count),
            limit: max_states,
        });
    }

    enumerate_gray_into(
        curve,
        Some(points[0]),
        &points[1..1 + left_bits],
        max_states,
        &mut workspace.gray_left,
    )?;
    let right_begin = 1 + left_bits;
    if right_bits == 0 {
        enumerate_gray_into(curve, None, &[], max_states, &mut workspace.gray_right)?;
    } else {
        // Global sign complementation maps every right sum R to -R.  Fix the
        // first right sign positive and retain its orientation so a lookup can
        // recover either member of the pair with a mask complement.
        enumerate_gray_into(
            curve,
            Some(points[right_begin]),
            &points[right_begin + 1..],
            max_states,
            &mut workspace.gray_right,
        )?;
    }
    workspace.right_index.clear();
    workspace
        .right_index
        .try_reserve(workspace.gray_right.len())
        .map_err(|_| SummationRootError::AllocationFailed(workspace.gray_right.len()))?;
    for (index, &point) in workspace.gray_right.iter().enumerate() {
        let rest_gray = index ^ (index >> 1);
        let sign_mask = if right_bits == 0 {
            0
        } else {
            1 | ((rest_gray as u64) << 1)
        };
        let (key, canonical_negated) = negation_key(curve, point);
        workspace.right_index.entry(key).or_insert(RightWitness {
            sign_mask,
            canonical_negated,
        });
    }
    let right_complement = (1u64 << right_bits).wrapping_sub(1);

    for &(target_x, target) in &targets {
        for signed_target in [target, curve.neg(target)] {
            workspace.left.clear();
            workspace.right.clear();
            workspace
                .left
                .resize(workspace.gray_left.len(), Some(signed_target));
            workspace.right.extend(
                workspace
                    .gray_left
                    .iter()
                    .map(|point| point.map(|p| curve.neg(p))),
            );
            curve.add_many_batched(&mut workspace.left, &workspace.right, &mut workspace.batch);
            for (left_index, needed) in workspace.left.iter().enumerate() {
                let (key, needed_negated) = negation_key(curve, *needed);
                let Some(&witness) = workspace.right_index.get(&key) else {
                    continue;
                };
                let left_gray = left_index ^ (left_index >> 1);
                let right_mask = if needed_negated == witness.canonical_negated {
                    witness.sign_mask
                } else {
                    witness.sign_mask ^ right_complement
                };
                let sign_mask = (left_gray as u64) | (right_mask << left_bits);
                let sum = one_root(curve, points, sign_mask)?;
                debug_assert_eq!(sum.map(|p| p.x), Some(target_x));
                return Ok(TargetSearchResult {
                    target_count: target_xs.len(),
                    liftable_targets: targets.len(),
                    unliftable_targets,
                    left_count,
                    right_signed_count,
                    right_count,
                    matched_target_x: Some(target_x),
                    sign_mask: Some(sign_mask),
                    sum,
                });
            }
        }
    }

    Ok(TargetSearchResult {
        target_count: target_xs.len(),
        liftable_targets: targets.len(),
        unliftable_targets,
        left_count,
        right_signed_count,
        right_count,
        matched_target_x: None,
        sign_mask: None,
        sum: None,
    })
}

fn trim(poly: &mut Vec<u64>) {
    while poly.len() > 1 && poly.last() == Some(&0) {
        poly.pop();
    }
    if poly.is_empty() {
        poly.push(0);
    }
}

fn poly_is_zero(poly: &[u64]) -> bool {
    poly.len() == 1 && poly[0] == 0
}

fn poly_add(f: &Fp64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (index, value) in out.iter_mut().enumerate() {
        *value = f.add(
            a.get(index).copied().unwrap_or(0),
            b.get(index).copied().unwrap_or(0),
        );
    }
    trim(&mut out);
    out
}

fn poly_sub(f: &Fp64, a: &[u64], b: &[u64]) -> Vec<u64> {
    let mut out = vec![0; a.len().max(b.len())];
    for (index, value) in out.iter_mut().enumerate() {
        *value = f.sub(
            a.get(index).copied().unwrap_or(0),
            b.get(index).copied().unwrap_or(0),
        );
    }
    trim(&mut out);
    out
}

fn poly_mul_quadratic(f: &Fp64, a: &[u64], b: &[u64]) -> Vec<u64> {
    if poly_is_zero(a) || poly_is_zero(b) {
        return vec![0];
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &left) in a.iter().enumerate() {
        for (j, &right) in b.iter().enumerate() {
            out[i + j] = f.add(out[i + j], f.mul(left, right));
        }
    }
    trim(&mut out);
    out
}

const KARATSUBA_THRESHOLD: usize = 32;

fn poly_slice(poly: &[u64], begin: usize, end: usize) -> Vec<u64> {
    if begin >= poly.len() || begin >= end {
        return vec![0];
    }
    let mut out = poly[begin..end.min(poly.len())].to_vec();
    trim(&mut out);
    out
}

fn accumulate_shifted(f: &Fp64, out: &mut Vec<u64>, input: &[u64], shift: usize) {
    if poly_is_zero(input) {
        return;
    }
    out.resize(out.len().max(input.len() + shift), 0);
    for (index, &value) in input.iter().enumerate() {
        out[index + shift] = f.add(out[index + shift], value);
    }
}

fn poly_mul_fast(f: &Fp64, a: &[u64], b: &[u64]) -> Vec<u64> {
    if poly_is_zero(a) || poly_is_zero(b) {
        return vec![0];
    }
    let small = a.len().min(b.len());
    let large = a.len().max(b.len());
    if small <= KARATSUBA_THRESHOLD || large > 2 * small {
        return poly_mul_quadratic(f, a, b);
    }
    let split = large / 2;
    let a0 = poly_slice(a, 0, split);
    let a1 = poly_slice(a, split, a.len());
    let b0 = poly_slice(b, 0, split);
    let b1 = poly_slice(b, split, b.len());
    if poly_is_zero(&a1) || poly_is_zero(&b1) {
        return poly_mul_quadratic(f, a, b);
    }
    let z0 = poly_mul_fast(f, &a0, &b0);
    let z2 = poly_mul_fast(f, &a1, &b1);
    let z1 = poly_mul_fast(f, &poly_add(f, &a0, &a1), &poly_add(f, &b0, &b1));
    let z1 = poly_sub(f, &poly_sub(f, &z1, &z0), &z2);
    let mut out = vec![0; a.len() + b.len() - 1];
    accumulate_shifted(f, &mut out, &z0, 0);
    accumulate_shifted(f, &mut out, &z1, split);
    accumulate_shifted(f, &mut out, &z2, 2 * split);
    trim(&mut out);
    out
}

fn coefficients_quadratic(f: &Fp64, roots: &[u64]) -> Vec<u64> {
    let mut out = vec![f.one];
    for &root in roots {
        out.push(0);
        for index in (1..out.len()).rev() {
            out[index] = f.sub(out[index - 1], f.mul(root, out[index]));
        }
        out[0] = f.neg(f.mul(root, out[0]));
    }
    out
}

fn coefficients_product_tree(f: &Fp64, roots: &[u64]) -> Vec<u64> {
    if roots.is_empty() {
        return vec![f.one];
    }
    let mut level: Vec<Vec<u64>> = roots.iter().map(|&root| vec![f.neg(root), f.one]).collect();
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut pairs = level.chunks_exact(2);
        for pair in &mut pairs {
            next.push(poly_mul_fast(f, &pair[0], &pair[1]));
        }
        if let Some(last) = pairs.remainder().first() {
            next.push(last.clone());
        }
        level = next;
    }
    level.pop().expect("non-empty product tree")
}

/// Materialize the monic polynomial with the supplied canonical finite roots.
/// Coefficients are returned canonical and low degree first.
pub fn coefficients_from_roots(
    curve: &FastCurve,
    roots_canonical: &[u64],
    method: CoefficientMethod,
) -> Vec<u64> {
    let roots: Vec<u64> = roots_canonical
        .iter()
        .map(|&root| curve.f.to_mont(root))
        .collect();
    let selected = method.resolved(roots.len());
    let montgomery = match selected {
        CoefficientMethod::Auto => unreachable!("resolved above"),
        CoefficientMethod::Quadratic => coefficients_quadratic(&curve.f, &roots),
        CoefficientMethod::ProductTree => coefficients_product_tree(&curve.f, &roots),
    };
    montgomery
        .into_iter()
        .map(|coefficient| curve.f.from_mont(coefficient))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::semaev_prime_poly;

    fn curve() -> FastCurve {
        FastCurve::for_arithmetic("summation-roots-test", 1009, 2, 3)
    }

    fn points(curve: &FastCurve, count: usize) -> Vec<Pt> {
        (0..curve.f.p)
            .filter_map(|x| curve.lift_x(x))
            .take(count)
            .collect()
    }

    fn naive(curve: &FastCurve, points: &[Pt]) -> SummationRootSet {
        let mut out = SummationRootSet::default();
        for mask in 0..(1usize << (points.len() - 1)) {
            match one_root(curve, points, mask as u64).unwrap() {
                Some(point) => out.finite_x.push(point.x),
                None => out.infinity_multiplicity += 1,
            }
        }
        out.sort_finite();
        out
    }

    #[test]
    fn batched_tree_matches_naive_signed_sums_and_s3() {
        let curve = curve();
        let all = points(&curve, 7);
        for count in 1..=7 {
            let mut got = all_roots(&curve, &all[..count], 1 << 12).unwrap();
            got.sort_finite();
            assert_eq!(got, naive(&curve, &all[..count]), "point count {count}");
        }
        let mut roots = all_roots(&curve, &all[..2], 4).unwrap();
        roots.sort_finite();
        let reference = semaev_prime_poly::Curve::new(1009, 2, 3);
        let x0 = curve.canonical(all[0]).0;
        let x1 = curve.canonical(all[1]).0;
        for root in roots.finite_x {
            assert_eq!(reference.s3(x0, x1, root), 0);
        }
    }

    #[test]
    fn infinity_and_multiplicity_are_preserved() {
        let curve = curve();
        let point = points(&curve, 1)[0];
        let roots = all_roots(&curve, &[point, point], 2).unwrap();
        assert_eq!(roots.infinity_multiplicity, 1);
        assert_eq!(roots.finite_x.len(), 1);
        let repeated = all_roots(&curve, &[point, point, point], 4).unwrap();
        assert_eq!(repeated.finite_x.len() + repeated.infinity_multiplicity, 4);
    }

    #[test]
    fn one_root_masks_and_target_certificates_replay() {
        let curve = curve();
        let points = points(&curve, 6);
        let roots = all_roots(&curve, &points, 64).unwrap();
        for mask in 0..(1u64 << (points.len() - 1)) {
            let root = one_root(&curve, &points, mask).unwrap();
            match root {
                Some(point) => assert!(roots.finite_x.contains(&point.x)),
                None => assert!(roots.infinity_multiplicity > 0),
            }
        }
        let hit = one_root(&curve, &points, 9).unwrap().unwrap();
        let mut workspace = SummationRootWorkspace::default();
        let result = find_target_root(&curve, &points, &[hit.x], 64, &mut workspace).unwrap();
        assert!(result.found());
        let replay = one_root(&curve, &points, result.sign_mask.unwrap())
            .unwrap()
            .unwrap();
        assert_eq!(replay.x, hit.x);
        assert_eq!(result.sum.unwrap().x, hit.x);
    }

    #[test]
    fn target_miss_and_unliftable_target_are_exact() {
        let curve = curve();
        let points = points(&curve, 4);
        let mut roots = all_roots(&curve, &points, 16).unwrap();
        roots.sort_finite();
        let miss = (0..curve.f.p)
            .find(|&x| curve.lift_x(x).is_some() && roots.finite_x.binary_search(&x).is_err())
            .unwrap();
        let unliftable = (0..curve.f.p).find(|&x| curve.lift_x(x).is_none()).unwrap();
        let mut workspace = SummationRootWorkspace::default();
        let result =
            find_target_root(&curve, &points, &[miss, unliftable], 32, &mut workspace).unwrap();
        assert!(!result.found());
        assert_eq!(result.liftable_targets, 1);
        assert_eq!(result.unliftable_targets, 1);
        assert_eq!(result.right_signed_count, 2 * result.right_count);
    }

    #[test]
    fn target_negation_fold_replays_hits_and_infinity_orientation() {
        let curve = curve();
        let points = points(&curve, 7);
        let mut roots = all_roots(&curve, &points, 64).unwrap();
        roots.sort_finite();
        roots.finite_x.dedup();
        let mut workspace = SummationRootWorkspace::default();
        for target_x in roots.finite_x {
            let result =
                find_target_root(&curve, &points, &[target_x], 64, &mut workspace).unwrap();
            assert!(result.found(), "target x={target_x}");
            assert_eq!(result.right_signed_count, 2 * result.right_count);
            let replay = one_root(&curve, &points, result.sign_mask.unwrap())
                .unwrap()
                .unwrap();
            assert_eq!(replay.x, target_x);
        }

        let point = points[0];
        let target_x = curve.canonical(point).0;
        let degenerate = [point, points[1], points[1]];
        let result = find_target_root(&curve, &degenerate, &[target_x], 8, &mut workspace).unwrap();
        assert!(result.found());
        assert_eq!(result.right_signed_count, 2 * result.right_count);
        assert_eq!(
            one_root(&curve, &degenerate, result.sign_mask.unwrap())
                .unwrap()
                .unwrap()
                .x,
            target_x
        );
    }

    #[test]
    fn coefficient_builders_match_and_roots_evaluate_to_zero() {
        let curve = curve();
        for count in [0, 1, 2, 3, 7, 31, 32, 127, 256] {
            let roots: Vec<u64> = (0..count).map(|i| (i as u64 * 17) % 1009).collect();
            let quadratic = coefficients_from_roots(&curve, &roots, CoefficientMethod::Quadratic);
            let product = coefficients_from_roots(&curve, &roots, CoefficientMethod::ProductTree);
            let automatic = coefficients_from_roots(&curve, &roots, CoefficientMethod::Auto);
            assert_eq!(quadratic, product, "root count {count}");
            assert_eq!(quadratic, automatic, "auto root count {count}");
            for &root in &roots {
                let value = quadratic.iter().rev().fold(0u64, |acc, &coefficient| {
                    ((acc as u128 * root as u128 + coefficient as u128) % 1009) as u64
                });
                assert_eq!(value, 0, "root {root}, count {count}");
            }
        }
    }

    #[test]
    fn bounds_and_invalid_inputs_are_rejected() {
        let curve = curve();
        let points = points(&curve, 4);
        assert!(matches!(
            all_roots(&curve, &points, 2),
            Err(SummationRootError::StateLimit { .. })
        ));
        assert!(matches!(
            one_root(&curve, &points, 1 << 3),
            Err(SummationRootError::SignMaskOutOfRange { .. })
        ));
        assert!(matches!(
            lift_input_xs(&curve, &[]),
            Err(SummationRootError::NoInputPoints)
        ));
    }
}
