//! Exact triple multiplicity and realized-support checks for prime-field bases.
//!
//! This is the native correction of the counting and comparison conventions
//! audited in crypto PR #5153. A tuple is an index triple `i <= j <= k`:
//! repeated indices, identity summands/targets and cancelling pairs all count.
//! Raw multiplicity is distinct from target coverage and from useful signed
//! relation rank. This module does not infer either rank or solving cost.
//!
//! Arithmetic reuses [`super::prime_fast`]. Points are Montgomery encoded;
//! `None` is the identity. The caller establishes the field/curve parameters
//! and any required subgroup certificate. Input point encodings are checked
//! here, but a support label is not proof of its builder or seed provenance.

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::Serialize;

use super::prime_fast::{FastCurve, Pt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidPoint {
        input: &'static str,
        index: usize,
    },
    DuplicatePoint {
        index: usize,
    },
    DuplicateOrbit {
        index: usize,
    },
    IdentityExcluded {
        index: usize,
    },
    InvalidClosedCardinality {
        points: usize,
        fixed_points: usize,
    },
    NotNegationClosed {
        singleton_orbits: usize,
    },
    DeclaredSizeMismatch {
        arm: &'static str,
        declared: usize,
        actual: usize,
    },
    EmptyComparisonSupport,
    EmptyComparisonTargets,
    RepresentationMismatch,
    IdentityPolicyMismatch,
    SupportProfileMismatch,
    TargetSequenceMismatch,
    CountOverflow,
    AllocationFailed,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPoint { input, index } => {
                write!(f, "{input}[{index}] is noncanonical or off the curve")
            }
            Self::DuplicatePoint { index } => write!(f, "duplicate support point at index {index}"),
            Self::DuplicateOrbit { index } => {
                write!(f, "duplicate negation orbit among representatives at index {index}")
            }
            Self::IdentityExcluded { index } => {
                write!(f, "identity at support index {index} is excluded by the declared policy")
            }
            Self::InvalidClosedCardinality { points, fixed_points } => write!(
                f,
                "a closed support of {points} points with {fixed_points} fixed points needs an even nonfixed remainder"
            ),
            Self::NotNegationClosed { singleton_orbits } => {
                write!(f, "claimed negation-closed support has {singleton_orbits} singleton orbits")
            }
            Self::DeclaredSizeMismatch { arm, declared, actual } => {
                write!(f, "{arm} declares {declared} points but supplies {actual}")
            }
            Self::EmptyComparisonSupport => write!(f, "comparison requires nonempty actual supports"),
            Self::EmptyComparisonTargets => write!(f, "comparison requires nonempty actual targets"),
            Self::RepresentationMismatch => write!(f, "comparison representation conventions differ"),
            Self::IdentityPolicyMismatch => write!(f, "comparison identity policies differ"),
            Self::SupportProfileMismatch => {
                write!(f, "realized orbit profiles or public anchor signs differ")
            }
            Self::TargetSequenceMismatch => write!(f, "ordered comparison target sequences differ"),
            Self::CountOverflow => write!(f, "exact count exceeds its integer representation"),
            Self::AllocationFailed => write!(f, "support expansion allocation failed"),
        }
    }
}

impl std::error::Error for Error {}

/// Add an exact count, leaving `total` unchanged on overflow.
pub fn checked_add_count(total: &mut u64, amount: u64) -> Result<(), Error> {
    *total = total.checked_add(amount).ok_or(Error::CountOverflow)?;
    Ok(())
}

fn validate_point(
    curve: &FastCurve,
    point: Option<Pt>,
    input: &'static str,
    index: usize,
) -> Result<(), Error> {
    if let Some(point) = point {
        if point.x >= curve.f.p || point.y >= curve.f.p || !curve.is_on_curve(point) {
            return Err(Error::InvalidPoint { input, index });
        }
    }
    Ok(())
}

fn validate_points(
    curve: &FastCurve,
    points: &[Option<Pt>],
    input: &'static str,
) -> Result<(), Error> {
    for (index, &point) in points.iter().enumerate() {
        validate_point(curve, point, input, index)?;
    }
    Ok(())
}

/// Algorithmic counters only: validation, hashing, allocation and all other
/// solver phases are outside the two group-addition counts.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct TripleStats {
    pub queries: usize,
    pub covered_queries: usize,
    pub total_multiplicity: u64,
    pub pair_additions: u64,
    pub complement_additions: u64,
}

impl TripleStats {
    pub fn mean_multiplicity(&self) -> Option<f64> {
        (self.queries != 0).then(|| self.total_multiplicity as f64 / self.queries as f64)
    }

    pub fn coverage(&self) -> Option<f64> {
        (self.queries != 0).then(|| self.covered_queries as f64 / self.queries as f64)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TripleCounts {
    /// One entry per supplied query, preserving order and repetitions.
    pub per_target: Vec<u64>,
    pub stats: TripleStats,
}

/// Count all indexed triples `i <= j <= k` on the supplied point lists.
///
/// Before querying third index `k`, the sparse pair table contains every
/// `i <= j <= k`, including pair sums equal to the identity. Thus each tuple
/// is counted once. Repeated points in `base` remain distinct indexed entries;
/// [`support_profile`] separately enforces distinct-point support conventions.
/// Expected hash-table work is `O(B^2 + B*T)` and storage is `O(B^2 + T)`.
pub fn triple_counts(
    curve: &FastCurve,
    base: &[Option<Pt>],
    targets: &[Option<Pt>],
) -> Result<TripleCounts, Error> {
    validate_points(curve, base, "base")?;
    validate_points(curve, targets, "targets")?;
    let mut out = TripleCounts {
        per_target: vec![0; targets.len()],
        stats: TripleStats {
            queries: targets.len(),
            ..TripleStats::default()
        },
    };
    if targets.is_empty() {
        return Ok(out);
    }
    let mut pairs = HashMap::<Option<Pt>, u64>::new();
    for (k, &last) in base.iter().enumerate() {
        for &first in &base[..=k] {
            let sum = curve.add(first, last);
            checked_add_count(pairs.entry(sum).or_default(), 1)?;
            checked_add_count(&mut out.stats.pair_additions, 1)?;
        }
        let negative_last = last.map(|point| curve.neg(point));
        for (&target, count) in targets.iter().zip(&mut out.per_target) {
            let complement = curve.add(target, negative_last);
            checked_add_count(&mut out.stats.complement_additions, 1)?;
            if let Some(&multiplicity) = pairs.get(&complement) {
                checked_add_count(count, multiplicity)?;
            }
        }
    }
    for &count in &out.per_target {
        checked_add_count(&mut out.stats.total_multiplicity, count)?;
        out.stats.covered_queries += usize::from(count != 0);
    }
    Ok(out)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Representation {
    /// The list is exactly the supplied support; no implicit signs are added.
    ExplicitPoints,
    /// The supplied list already contains every opposite point.
    NegationClosedPoints,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityPolicy {
    Allow,
    Exclude,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AnchorSigns {
    pub positive: bool,
    pub negative: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SupportProfile {
    /// Actual unique points, including any identity/fixed points.
    pub points: usize,
    pub inverse_pairs: usize,
    /// Nonfixed negation orbits with exactly one supplied sign.
    pub singleton_orbits: usize,
    pub identity_points: usize,
    /// Nonidentity points fixed by negation, each represented once.
    pub two_torsion_points: usize,
    /// Nonidentity negation orbits, including two-torsion points.
    pub orbit_columns: usize,
    /// Nonidentity orbits not coinciding with any supplied public anchor.
    /// This is a representation count, not a measured independent rank.
    pub unanchored_columns: usize,
    /// In anchor-list order. For fixed anchors the two flags coincide.
    pub anchor_signs: Vec<AnchorSigns>,
}

/// Necessary cardinality condition for a closed support with this many fixed
/// points. With no identity or two-torsion point, odd sizes fail explicitly.
pub fn require_closed_cardinality(points: usize, fixed_points: usize) -> Result<(), Error> {
    if fixed_points > points || !(points - fixed_points).is_multiple_of(2) {
        return Err(Error::InvalidClosedCardinality {
            points,
            fixed_points,
        });
    }
    Ok(())
}

fn orbit_key(curve: &FastCurve, point: Pt) -> Pt {
    let opposite = curve.neg(point);
    Pt {
        x: point.x,
        y: point.y.min(opposite.y),
    }
}

/// Expand one representative per nonidentity negation orbit into its actual
/// signed support. Reject duplicate orbits instead of silently deduplicating;
/// a two-torsion representative appears once, not twice.
pub fn expand_signed_support(
    curve: &FastCurve,
    representatives: &[Pt],
) -> Result<Vec<Option<Pt>>, Error> {
    let capacity = representatives
        .len()
        .checked_mul(2)
        .ok_or(Error::CountOverflow)?;
    let mut out = Vec::new();
    out.try_reserve_exact(capacity)
        .map_err(|_| Error::AllocationFailed)?;
    let mut seen = HashSet::new();
    for (index, &point) in representatives.iter().enumerate() {
        validate_point(curve, Some(point), "representatives", index)?;
        if !seen.insert(orbit_key(curve, point)) {
            return Err(Error::DuplicateOrbit { index });
        }
        out.push(Some(point));
        let opposite = curve.neg(point);
        if opposite != point {
            out.push(Some(opposite));
        }
    }
    Ok(out)
}

/// Profile the actual support without assuming that a family name or an
/// invariant eligibility predicate implies closure of a truncated sample.
pub fn support_profile(
    curve: &FastCurve,
    points: &[Option<Pt>],
    anchors: &[Option<Pt>],
    representation: Representation,
    identity_policy: IdentityPolicy,
) -> Result<SupportProfile, Error> {
    validate_points(curve, points, "support")?;
    validate_points(curve, anchors, "anchors")?;
    let mut present = HashSet::new();
    let mut orbits = HashMap::<Pt, usize>::new();
    for (index, &point) in points.iter().enumerate() {
        if !present.insert(point) {
            return Err(Error::DuplicatePoint { index });
        }
        if let Some(point) = point {
            *orbits.entry(orbit_key(curve, point)).or_default() += 1;
        } else if identity_policy == IdentityPolicy::Exclude {
            return Err(Error::IdentityExcluded { index });
        }
    }
    let mut out = SupportProfile {
        points: points.len(),
        identity_points: usize::from(present.contains(&None)),
        orbit_columns: orbits.len(),
        ..SupportProfile::default()
    };
    for (&point, &occupancy) in &orbits {
        if curve.neg(point) == point {
            out.two_torsion_points += 1;
        } else if occupancy == 2 {
            out.inverse_pairs += 1;
        } else {
            out.singleton_orbits += 1;
        }
    }
    if representation == Representation::NegationClosedPoints {
        require_closed_cardinality(out.points, out.identity_points + out.two_torsion_points)?;
        if out.singleton_orbits != 0 {
            return Err(Error::NotNegationClosed {
                singleton_orbits: out.singleton_orbits,
            });
        }
    }
    let anchor_orbits: HashSet<Pt> = anchors
        .iter()
        .flatten()
        .map(|&point| orbit_key(curve, point))
        .collect();
    out.unanchored_columns = orbits
        .keys()
        .filter(|point| !anchor_orbits.contains(*point))
        .count();
    out.anchor_signs = anchors
        .iter()
        .map(|&anchor| AnchorSigns {
            positive: present.contains(&anchor),
            negative: present.contains(&anchor.map(|point| curve.neg(point))),
        })
        .collect();
    Ok(out)
}

#[derive(Clone, Copy, Debug)]
pub struct ComparisonArm<'a> {
    pub points: &'a [Option<Pt>],
    pub targets: &'a [Option<Pt>],
    pub declared_points: usize,
    pub representation: Representation,
    pub identity_policy: IdentityPolicy,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct PairedAdmission {
    pub profile: SupportProfile,
    pub query_count: usize,
    pub representation: Representation,
    pub identity_policy: IdentityPolicy,
}

/// Admit actual paired inputs with identical realized support conventions,
/// orbit profiles, anchor signs and ordered target multisets.
///
/// This does not establish builder provenance, preregistration, random-control
/// distribution, useful relation rank, or a historical replay. Those require
/// their own input records and checks; matching a profile alone proves none.
pub fn admit_pair(
    curve: &FastCurve,
    anchors: &[Option<Pt>],
    reference: ComparisonArm<'_>,
    control: ComparisonArm<'_>,
) -> Result<PairedAdmission, Error> {
    for (name, arm) in [("reference", reference), ("control", control)] {
        if arm.declared_points != arm.points.len() {
            return Err(Error::DeclaredSizeMismatch {
                arm: name,
                declared: arm.declared_points,
                actual: arm.points.len(),
            });
        }
    }
    if reference.points.is_empty() || control.points.is_empty() {
        return Err(Error::EmptyComparisonSupport);
    }
    if reference.targets.is_empty() || control.targets.is_empty() {
        return Err(Error::EmptyComparisonTargets);
    }
    if reference.representation != control.representation {
        return Err(Error::RepresentationMismatch);
    }
    if reference.identity_policy != control.identity_policy {
        return Err(Error::IdentityPolicyMismatch);
    }
    let profile = support_profile(
        curve,
        reference.points,
        anchors,
        reference.representation,
        reference.identity_policy,
    )?;
    let other = support_profile(
        curve,
        control.points,
        anchors,
        control.representation,
        control.identity_policy,
    )?;
    if profile != other {
        return Err(Error::SupportProfileMismatch);
    }
    validate_points(curve, reference.targets, "reference targets")?;
    validate_points(curve, control.targets, "control targets")?;
    if reference.targets != control.targets {
        return Err(Error::TargetSequenceMismatch);
    }
    Ok(PairedAdmission {
        profile,
        query_count: reference.targets.len(),
        representation: reference.representation,
        identity_policy: reference.identity_policy,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::prime_fast::FastFactorBase;

    const P: u64 = 2_147_483_647;
    const N: u64 = 2_147_446_831;

    #[test]
    fn fast_factor_base_exports_actual_signed_support() {
        let (curve, g) = fixture();
        let base = FastFactorBase::build(&curve, 8);
        let original_representatives = base.pts.clone();
        let original_xs = base.xs.clone();
        let support = base.signed_support(&curve).unwrap();
        let profile = base.support_profile(&curve, &[Some(g)]).unwrap();
        assert_eq!(base.len(), 8);
        assert_eq!(support.len(), 16);
        assert_eq!(profile.points, 16);
        assert_eq!(profile.inverse_pairs, 8);
        assert_eq!(profile.singleton_orbits, 0);
        assert_eq!(profile.identity_points, 0);
        assert_eq!(profile.orbit_columns, 8);
        for (index, &point) in base.pts.iter().enumerate() {
            assert!(support.contains(&Some(point)));
            assert!(support.contains(&Some(curve.neg(point))));
            assert_eq!(base.lookup(curve.canonical(point).0), Some(index));
        }
        let targets = [Some(base.pts[0]), None, Some(base.pts[0])];
        let counts = base.triple_counts(&curve, &targets).unwrap();
        let mut expected = vec![0; targets.len()];
        for i in 0..support.len() {
            for j in i..support.len() {
                for k in j..support.len() {
                    let sum = curve.add(curve.add(support[i], support[j]), support[k]);
                    for (query, &target) in targets.iter().enumerate() {
                        if sum == target {
                            expected[query] += 1;
                        }
                    }
                }
            }
        }
        assert_eq!(counts.per_target, expected);
        assert_eq!(counts.per_target[0], counts.per_target[2]);
        assert_eq!(base.pts, original_representatives);
        assert_eq!(base.xs, original_xs);
    }

    fn fixture() -> (FastCurve, Pt) {
        // The existing research/prime31_ic_20261115/native::PRIME31 fixture.
        let curve = FastCurve::new(
            "prime31-factor-base-controls",
            P,
            847_514_846,
            2_087_444_724,
            N,
        );
        let generator = curve.point(625_037_715, 2_100_627_251);
        (curve, generator)
    }

    fn multiples(curve: &FastCurve, generator: Pt, scalars: &[i64]) -> Vec<Option<Pt>> {
        scalars
            .iter()
            .map(|&scalar| curve.mul(Some(generator), scalar.rem_euclid(N as i64) as u64))
            .collect()
    }

    // Independent three-loop enumeration, without the rolling pair table.
    fn exhaustive(curve: &FastCurve, base: &[Option<Pt>], targets: &[Option<Pt>]) -> Vec<u64> {
        let mut counts = vec![0; targets.len()];
        for i in 0..base.len() {
            for j in i..base.len() {
                for k in j..base.len() {
                    let sum = curve.add(curve.add(base[i], base[j]), base[k]);
                    for (count, &target) in counts.iter_mut().zip(targets) {
                        *count += u64::from(sum == target);
                    }
                }
            }
        }
        counts
    }

    fn explicit<'a>(points: &'a [Option<Pt>], targets: &'a [Option<Pt>]) -> ComparisonArm<'a> {
        ComparisonArm {
            points,
            targets,
            declared_points: points.len(),
            representation: Representation::ExplicitPoints,
            identity_policy: IdentityPolicy::Allow,
        }
    }

    #[test]
    fn principal_m31_fixture_and_cancellation_counterexample() {
        let (curve, g) = fixture();
        assert_eq!(curve.f.p, (1 << 31) - 1);
        assert!(curve.is_on_curve(g));
        assert_eq!(curve.mul(Some(g), N), None);
        let first = multiples(&curve, g, &[1, -1, 2]);
        let reordered = multiples(&curve, g, &[1, 2, -1]);
        let targets = multiples(&curve, g, &[2]);
        for base in [&first, &reordered] {
            assert_eq!(
                triple_counts(&curve, base, &targets).unwrap().per_target,
                vec![1]
            );
        }
    }

    #[test]
    fn all_permutations_match_exhaustive_with_identity_and_repeated_targets() {
        let (curve, g) = fixture();
        let base = multiples(&curve, g, &[0, 1, -1, 2]);
        let targets = multiples(&curve, g, &[0, 1, -1, 2, -2, 3, -3, 4, 5, 6, 0, 2, 2]);
        let expected = exhaustive(&curve, &base, &targets);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        let indices = [a, b, c, d];
                        if indices.iter().copied().collect::<HashSet<_>>().len() == 4 {
                            let reordered: Vec<_> = indices.map(|i| base[i]).into();
                            assert_eq!(
                                triple_counts(&curve, &reordered, &targets)
                                    .unwrap()
                                    .per_target,
                                expected
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn identity_target_is_counted_without_an_inverse_pair_in_support() {
        let (curve, g) = fixture();
        let base = multiples(&curve, g, &[1, 2, -3]);
        assert_eq!(
            triple_counts(&curve, &base, &[None]).unwrap().per_target,
            vec![1]
        );
    }

    #[test]
    fn repeated_queries_preserve_weights_and_separate_coverage() {
        let (curve, g) = fixture();
        let base = multiples(&curve, g, &[1, 2, 3]);
        let targets = multiples(&curve, g, &[6, 6, 20]);
        let result = triple_counts(&curve, &base, &targets).unwrap();
        assert_eq!(result.per_target, vec![2, 2, 0]);
        assert_eq!(result.stats.total_multiplicity, 4);
        assert_eq!(result.stats.covered_queries, 2);
        assert_eq!(result.stats.mean_multiplicity(), Some(4.0 / 3.0));
        assert_eq!(result.stats.coverage(), Some(2.0 / 3.0));
        assert_eq!(result.stats.pair_additions, 6);
        assert_eq!(result.stats.complement_additions, 9);
    }

    #[test]
    fn empty_inputs_and_indexed_repeated_base_entries_are_explicit() {
        let (curve, g) = fixture();
        assert_eq!(
            triple_counts(&curve, &[], &[None, Some(g)])
                .unwrap()
                .per_target,
            vec![0, 0]
        );
        let empty = triple_counts(&curve, &[Some(g)], &[]).unwrap();
        assert_eq!(empty.stats.mean_multiplicity(), None);
        assert_eq!(empty.stats.coverage(), None);
        assert_eq!(empty.stats.pair_additions, 0);
        assert_eq!(
            triple_counts(&curve, &[None], &[None, None])
                .unwrap()
                .per_target,
            vec![1, 1]
        );
        let target = curve.mul(Some(g), 3);
        assert_eq!(
            triple_counts(&curve, &[Some(g), Some(g)], &[target])
                .unwrap()
                .per_target,
            vec![4]
        );
        assert_eq!(
            support_profile(
                &curve,
                &[Some(g), Some(g)],
                &[],
                Representation::ExplicitPoints,
                IdentityPolicy::Allow
            ),
            Err(Error::DuplicatePoint { index: 1 })
        );
    }

    #[test]
    fn total_tuple_mass_and_closed_support_negation_symmetry() {
        let (curve, g) = fixture();
        for scalars in [&[1, -1, 2, -2][..], &[1, 2, 3, 4], &[0, 1, -1, 2]] {
            let base = multiples(&curve, g, scalars);
            // Every possible scalar sum for these deterministic fixtures is in
            // [-12,12], much smaller than N. This is an independent mass check.
            let target_scalars: Vec<i64> = (-12..=12).collect();
            let targets = multiples(&curve, g, &target_scalars);
            let result = triple_counts(&curve, &base, &targets).unwrap();
            assert_eq!(result.per_target, exhaustive(&curve, &base, &targets));
            assert_eq!(result.stats.total_multiplicity, 20); // C(4+2,3).
        }
        let closed = multiples(&curve, g, &[1, -1, 2, -2]);
        let targets = multiples(&curve, g, &[1, -1, 2, -2, 3, -3]);
        let counts = triple_counts(&curve, &closed, &targets).unwrap().per_target;
        for pair in counts.chunks_exact(2) {
            assert_eq!(pair[0], pair[1]);
        }
    }

    #[test]
    fn mixed_profile_and_public_anchor_signs_are_realized() {
        let (curve, g) = fixture();
        let points = multiples(&curve, g, &[1, -1, 2, 3]);
        let anchors = multiples(&curve, g, &[1, 2, -1]);
        let profile = support_profile(
            &curve,
            &points,
            &anchors,
            Representation::ExplicitPoints,
            IdentityPolicy::Exclude,
        )
        .unwrap();
        assert_eq!(
            (
                profile.points,
                profile.inverse_pairs,
                profile.singleton_orbits
            ),
            (4, 1, 2)
        );
        assert_eq!((profile.orbit_columns, profile.unanchored_columns), (3, 1));
        assert_eq!(
            profile.anchor_signs,
            vec![
                AnchorSigns {
                    positive: true,
                    negative: true
                },
                AnchorSigns {
                    positive: true,
                    negative: false
                },
                AnchorSigns {
                    positive: true,
                    negative: true
                }
            ]
        );
    }

    #[test]
    fn fifty_one_points_are_not_silently_rounded_or_declared_closed() {
        let (curve, g) = fixture();
        let points = multiples(&curve, g, &(1..=51).collect::<Vec<i64>>());
        // Even size alone does not establish closure of a truncated support.
        assert_eq!(
            support_profile(
                &curve,
                &points[..2],
                &[],
                Representation::NegationClosedPoints,
                IdentityPolicy::Exclude,
            ),
            Err(Error::NotNegationClosed {
                singleton_orbits: 2,
            })
        );
        let profile = support_profile(
            &curve,
            &points,
            &[],
            Representation::ExplicitPoints,
            IdentityPolicy::Exclude,
        )
        .unwrap();
        assert_eq!((profile.points, profile.singleton_orbits), (51, 51));
        assert_eq!(
            support_profile(
                &curve,
                &points,
                &[],
                Representation::NegationClosedPoints,
                IdentityPolicy::Exclude
            ),
            Err(Error::InvalidClosedCardinality {
                points: 51,
                fixed_points: 0
            })
        );
        let queries = [Some(g)];
        let mut underfilled = explicit(&points[..50], &queries);
        underfilled.declared_points = 51;
        assert_eq!(
            admit_pair(&curve, &[], explicit(&points, &queries), underfilled),
            Err(Error::DeclaredSizeMismatch {
                arm: "control",
                declared: 51,
                actual: 50
            })
        );
        assert!(require_closed_cardinality(51, 1).is_ok());
        assert!(require_closed_cardinality(0, 1).is_err());
    }

    #[test]
    fn identity_and_two_torsion_are_explicit_fixed_points() {
        let (curve, g) = fixture();
        let points = [None, Some(g), Some(curve.neg(g))];
        let profile = support_profile(
            &curve,
            &points,
            &[None],
            Representation::NegationClosedPoints,
            IdentityPolicy::Allow,
        )
        .unwrap();
        assert_eq!(
            (
                profile.points,
                profile.inverse_pairs,
                profile.identity_points
            ),
            (3, 1, 1)
        );
        assert_eq!(
            profile.anchor_signs,
            vec![AnchorSigns {
                positive: true,
                negative: true
            }]
        );
        assert_eq!(
            support_profile(
                &curve,
                &points,
                &[],
                Representation::NegationClosedPoints,
                IdentityPolicy::Exclude
            ),
            Err(Error::IdentityExcluded { index: 0 })
        );
        // Separate public arithmetic fixture over the same M31 field; no
        // odd-prime subgroup is claimed for its nonzero two-torsion point.
        let torsion_curve = FastCurve::for_arithmetic("m31-two-torsion", P, 1, 0);
        let torsion = torsion_curve.point(0, 0);
        let expanded = expand_signed_support(&torsion_curve, &[torsion]).unwrap();
        assert_eq!(expanded, vec![Some(torsion)]);
        let profile = support_profile(
            &torsion_curve,
            &expanded,
            &[Some(torsion)],
            Representation::NegationClosedPoints,
            IdentityPolicy::Exclude,
        )
        .unwrap();
        assert_eq!(
            (
                profile.points,
                profile.two_torsion_points,
                profile.orbit_columns,
                profile.unanchored_columns
            ),
            (1, 1, 1, 0)
        );
    }

    #[test]
    fn signed_expansion_rejects_duplicate_orbits_and_bad_encodings() {
        let (curve, g) = fixture();
        let reps = [g, curve.mul(Some(g), 2).unwrap()];
        let expanded = expand_signed_support(&curve, &reps).unwrap();
        assert_eq!(expanded.len(), 4);
        let profile = support_profile(
            &curve,
            &expanded,
            &[],
            Representation::NegationClosedPoints,
            IdentityPolicy::Exclude,
        )
        .unwrap();
        assert_eq!(profile.inverse_pairs, 2);
        assert_eq!(
            expand_signed_support(&curve, &[g, curve.neg(g)]),
            Err(Error::DuplicateOrbit { index: 1 })
        );
        let invalid = Pt { x: P, y: g.y };
        assert!(matches!(
            expand_signed_support(&curve, &[invalid]),
            Err(Error::InvalidPoint { .. })
        ));
        assert!(matches!(
            triple_counts(&curve, &[Some(invalid)], &[Some(g)]),
            Err(Error::InvalidPoint { .. })
        ));
        assert!(matches!(
            triple_counts(&curve, &[Some(g)], &[Some(invalid)]),
            Err(Error::InvalidPoint { .. })
        ));
        let off_curve = curve.point(0, 0);
        assert!(matches!(
            support_profile(
                &curve,
                &[Some(off_curve)],
                &[],
                Representation::ExplicitPoints,
                IdentityPolicy::Allow
            ),
            Err(Error::InvalidPoint { .. })
        ));
    }

    #[test]
    fn admission_matches_profiles_anchor_signs_and_ordered_target_multisets() {
        let (curve, g) = fixture();
        let reference = multiples(&curve, g, &[1, -1, 2]);
        let control = multiples(&curve, g, &[1, -1, 3]);
        let targets = multiples(&curve, g, &[4, 4, 5]);
        let anchors = [Some(g)];
        let admission = admit_pair(
            &curve,
            &anchors,
            explicit(&reference, &targets),
            explicit(&control, &targets),
        )
        .unwrap();
        assert_eq!(admission.query_count, 3);
        let wrong_profile = multiples(&curve, g, &[1, 2, 3]);
        assert_eq!(
            admit_pair(
                &curve,
                &anchors,
                explicit(&reference, &targets),
                explicit(&wrong_profile, &targets)
            ),
            Err(Error::SupportProfileMismatch)
        );
        let flipped = multiples(&curve, g, &[-1, 2, 3]);
        assert_eq!(
            admit_pair(
                &curve,
                &anchors,
                explicit(&wrong_profile, &targets),
                explicit(&flipped, &targets)
            ),
            Err(Error::SupportProfileMismatch)
        );
        let reordered_targets = multiples(&curve, g, &[4, 5, 4]);
        assert_eq!(
            admit_pair(
                &curve,
                &anchors,
                explicit(&reference, &targets),
                explicit(&control, &reordered_targets)
            ),
            Err(Error::TargetSequenceMismatch)
        );
        assert_eq!(
            admit_pair(
                &curve,
                &anchors,
                explicit(&reference, &targets),
                explicit(&control, &targets[..2])
            ),
            Err(Error::TargetSequenceMismatch)
        );
    }

    #[test]
    fn admission_rejects_policy_mismatch_empty_inputs_and_invalid_targets() {
        let (curve, g) = fixture();
        let points = [Some(g), Some(curve.neg(g))];
        let targets = [Some(g)];
        let reference = explicit(&points, &targets);
        let mut changed = reference;
        changed.representation = Representation::NegationClosedPoints;
        assert_eq!(
            admit_pair(&curve, &[], reference, changed),
            Err(Error::RepresentationMismatch)
        );
        changed = reference;
        changed.identity_policy = IdentityPolicy::Exclude;
        assert_eq!(
            admit_pair(&curve, &[], reference, changed),
            Err(Error::IdentityPolicyMismatch)
        );
        assert_eq!(
            admit_pair(
                &curve,
                &[],
                explicit(&[], &targets),
                explicit(&[], &targets)
            ),
            Err(Error::EmptyComparisonSupport)
        );
        assert_eq!(
            admit_pair(&curve, &[], explicit(&points, &[]), explicit(&points, &[])),
            Err(Error::EmptyComparisonTargets)
        );
        let bad_targets = [Some(Pt { x: g.x, y: P })];
        assert!(matches!(
            admit_pair(
                &curve,
                &[],
                explicit(&points, &bad_targets),
                explicit(&points, &bad_targets)
            ),
            Err(Error::InvalidPoint { .. })
        ));
    }

    #[test]
    fn overflow_returns_error_without_mutating_the_count() {
        let mut count = u64::MAX - 1;
        assert_eq!(checked_add_count(&mut count, 1), Ok(()));
        assert_eq!(count, u64::MAX);
        assert_eq!(checked_add_count(&mut count, 1), Err(Error::CountOverflow));
        assert_eq!(count, u64::MAX);
        assert_eq!(checked_add_count(&mut count, 0), Ok(()));
    }
}
