//! Exact finite-domain compatibility checks for the existing Prime31 lane.
//!
//! Integration seam: pass the existing selector's complete root list
//! (`pkm_smooth_tower::SmoothTower::v` or Dickson `Base::values`) and the
//! DISTINCT coordinates actually retained after the existing curve/subgroup
//! filters (`FactorBase::abscissa_list` or distinct coordinates of
//! `Base::points`, not a list containing both point signs). Both
//! lists and the map must use the SAME coordinate chart. No selector, curve,
//! elliptic arithmetic, relation extractor, or logarithm solver is built here.
//!
//! A closed finite coordinate set does NOT establish a point action on the
//! target curve, an automorphism of a retained S3 chain, an action on signed
//! relations, or an action on a nonradical selector ideal. Callers must keep
//! those proof obligations separate. In particular, x -> -x is coordinate
//! negation, not the elliptic-curve negation (x,y) -> (x,-y).
//!
//! The only field arithmetic below evaluates degree-one rational maps with
//! u128 products. This makes the checker std-only and independently reusable;
//! it does not duplicate the production curve or selector implementation.
//! This module is deliberately restricted to the fixed public Prime31 field.

use std::collections::{BTreeMap, BTreeSet};

/// Existing ordinary Prime31 fixture; no new curve or target is selected.
pub const PRIME31_P: u64 = 2_147_483_647;
pub const PRIME31_A: u64 = 847_514_846;
pub const PRIME31_B: u64 = 2_087_444_724;
pub const PRIME31_ORDER: u64 = 2_147_446_831;
pub const PRIME31_GENERATOR: (u64, u64) = (625_037_715, 2_100_627_251);

/// This checker never upgrades a finite-set observation to a geometric claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceScope {
    FiniteCoordinateSetsOnly,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditError {
    NonCanonicalCoefficient { name: &'static str, value: u64 },
    SingularMobius,
    NonCanonicalCoordinate { domain: String, value: u64 },
    DuplicateCoordinate { domain: String, value: u64 },
    RetainedOutsideSelector { value: u64 },
    PermutationLength { leaves: usize, entries: usize },
    PermutationIndex { destination: usize, source: usize, leaves: usize },
    RepeatedPermutationSource { source: usize },
    NoLeaves,
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for AuditError {}

#[inline]
fn mul(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % PRIME31_P as u128) as u64
}

#[inline]
fn add(a: u64, b: u64) -> u64 {
    ((a as u128 + b as u128) % PRIME31_P as u128) as u64
}

#[inline]
fn sub(a: u64, b: u64) -> u64 {
    if a >= b { a - b } else { PRIME31_P - (b - a) }
}

fn inverse_nonzero(mut x: u64) -> u64 {
    // The fixed modulus 2^31-1 is prime. The caller excludes zero.
    let mut exponent = PRIME31_P - 2;
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mul(result, x);
        }
        x = mul(x, x);
        exponent >>= 1;
    }
    result
}

/// A nonsingular projective-linear map x -> (a*x+b)/(c*x+d).
/// Coefficients are private so every instance passes the constructor checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Mobius {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl Mobius {
    pub fn new(a: u64, b: u64, c: u64, d: u64) -> Result<Self, AuditError> {
        for (name, value) in [("a", a), ("b", b), ("c", c), ("d", d)] {
            if value >= PRIME31_P {
                return Err(AuditError::NonCanonicalCoefficient { name, value });
            }
        }
        if sub(mul(a, d), mul(b, c)) == 0 {
            return Err(AuditError::SingularMobius);
        }
        Ok(Self { a, b, c, d })
    }

    pub fn coefficients(&self) -> [u64; 4] {
        [self.a, self.b, self.c, self.d]
    }

    pub fn identity() -> Self {
        Self { a: 1, b: 0, c: 0, d: 1 }
    }

    pub fn coordinate_negation() -> Self {
        Self { a: PRIME31_P - 1, b: 0, c: 0, d: 1 }
    }

    pub fn reciprocal() -> Self {
        Self { a: 0, b: 1, c: 1, d: 0 }
    }

    pub fn affine(a: u64, b: u64) -> Result<Self, AuditError> {
        Self::new(a, b, 0, 1)
    }

    /// x -> center + k/(x-center). Nonzero k is required for invertibility.
    /// The name asserts a coordinate formula only, never target-curve torsion.
    pub fn centered_reciprocal(center: u64, k: u64) -> Result<Self, AuditError> {
        if center >= PRIME31_P {
            return Err(AuditError::NonCanonicalCoefficient { name: "center", value: center });
        }
        if k >= PRIME31_P {
            return Err(AuditError::NonCanonicalCoefficient { name: "k", value: k });
        }
        Self::new(center, sub(k, mul(center, center)), 1, sub(0, center))
    }

    /// None denotes a genuine pole/infinity and is never silently discarded.
    pub fn apply(&self, x: u64) -> Result<Option<u64>, AuditError> {
        if x >= PRIME31_P {
            return Err(AuditError::NonCanonicalCoordinate {
                domain: "map input".into(), value: x,
            });
        }
        let numerator = add(mul(self.a, x), self.b);
        let denominator = add(mul(self.c, x), self.d);
        if denominator == 0 {
            return Ok(None);
        }
        Ok(Some(mul(numerator, inverse_nonzero(denominator))))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DomainAudit {
    pub cardinality: usize,
    /// The full, sorted action table, with None for a pole.
    pub images: BTreeMap<u64, Option<u64>>,
    pub poles: Vec<u64>,
    /// Exact counterexamples (input, image) with finite image outside the set.
    pub escaped: Vec<(u64, u64)>,
    /// Mathematical set closure, including the vacuous empty-set case.
    pub set_closed: bool,
    /// True only for a nonempty set carried bijectively onto itself.
    pub nonvacuous_bijection: bool,
    pub scope: EvidenceScope,
}

fn checked_set(name: &str, values: &[u64]) -> Result<BTreeSet<u64>, AuditError> {
    let mut result = BTreeSet::new();
    for &value in values {
        if value >= PRIME31_P {
            return Err(AuditError::NonCanonicalCoordinate { domain: name.into(), value });
        }
        if !result.insert(value) {
            return Err(AuditError::DuplicateCoordinate { domain: name.into(), value });
        }
    }
    Ok(result)
}

fn audit_set(values: &BTreeSet<u64>, map: Mobius) -> Result<DomainAudit, AuditError> {
    let mut images = BTreeMap::new();
    let mut poles = Vec::new();
    let mut escaped = Vec::new();
    for &x in values {
        let image = map.apply(x)?;
        match image {
            None => poles.push(x),
            Some(y) if !values.contains(&y) => escaped.push((x, y)),
            Some(_) => (),
        }
        images.insert(x, image);
    }
    let set_closed = poles.is_empty() && escaped.is_empty();
    // A nonsingular Mobius map is injective on P^1. On a finite set with
    // no pole, containment therefore implies equality and bijectivity.
    Ok(DomainAudit {
        cardinality: values.len(), images, poles, escaped, set_closed,
        nonvacuous_bijection: !values.is_empty() && set_closed,
        scope: EvidenceScope::FiniteCoordinateSetsOnly,
    })
}

pub fn audit_domain(values: &[u64], map: Mobius) -> Result<DomainAudit, AuditError> {
    audit_set(&checked_set("domain", values)?, map)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectorSupportAudit {
    /// All supplied selector coordinates, BEFORE the existing point filters.
    pub selector: DomainAudit,
    /// Supplied coordinates retained AFTER the existing point filters.
    pub retained: DomainAudit,
    pub removed_by_filters: Vec<u64>,
    /// Domain-level eligibility only. Geometry and relations are separate.
    pub both_nonvacuously_closed: bool,
    pub scope: EvidenceScope,
}

/// Check both supplied layers rather than inferring retained support from the selector.
/// This verifies exactly the supplied lists; it does not certify completeness
/// of a selector root list or the correctness of an upstream point filter.
pub fn audit_selector_support(
    selector: &[u64], retained: &[u64], map: Mobius,
) -> Result<SelectorSupportAudit, AuditError> {
    let selector_set = checked_set("selector", selector)?;
    let retained_set = checked_set("retained support", retained)?;
    if let Some(&value) = retained_set.difference(&selector_set).next() {
        return Err(AuditError::RetainedOutsideSelector { value });
    }
    let removed_by_filters = selector_set.difference(&retained_set).copied().collect();
    let selector = audit_set(&selector_set, map)?;
    let retained = audit_set(&retained_set, map)?;
    let both_nonvacuously_closed = selector.nonvacuous_bijection && retained.nonvacuous_bijection;
    Ok(SelectorSupportAudit {
        selector, retained, removed_by_filters, both_nonvacuously_closed,
        scope: EvidenceScope::FiniteCoordinateSetsOnly,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafMismatch {
    pub destination: usize,
    pub source: usize,
    pub source_only: Vec<u64>,
    pub destination_only: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafPermutationAudit {
    /// Output leaf j receives the old value of source_for_destination[j].
    pub source_for_destination: Vec<usize>,
    pub mismatches: Vec<LeafMismatch>,
    pub empty_leaves: Vec<usize>,
    /// Equality of each source set and its destination set, before products.
    pub leafwise_equal: bool,
    /// Exact equality of Cartesian domains (true for the empty product set).
    pub cartesian_domain_equal: bool,
    pub nonvacuous_domain_compatible: bool,
    pub scope: EvidenceScope,
}

/// Audit domain exchangeability only. An arbitrary leaf permutation can
/// require rebuilding chain intermediates; this function proves nothing
/// about the syntactic S3 chain, target, signs, or relation multiplicities.
pub fn audit_leaf_permutation(
    domains: &[&[u64]], source_for_destination: &[usize],
) -> Result<LeafPermutationAudit, AuditError> {
    let leaves = domains.len();
    if leaves == 0 {
        return Err(AuditError::NoLeaves);
    }
    if source_for_destination.len() != leaves {
        return Err(AuditError::PermutationLength { leaves, entries: source_for_destination.len() });
    }
    let mut used = BTreeSet::new();
    for (destination, &source) in source_for_destination.iter().enumerate() {
        if source >= leaves {
            return Err(AuditError::PermutationIndex { destination, source, leaves });
        }
        if !used.insert(source) {
            return Err(AuditError::RepeatedPermutationSource { source });
        }
    }
    let sets: Vec<BTreeSet<u64>> = domains.iter().enumerate()
        .map(|(i, d)| checked_set(&format!("leaf {i}"), d))
        .collect::<Result<_, _>>()?;
    let empty_leaves: Vec<usize> = sets.iter().enumerate()
        .filter_map(|(i, s)| if s.is_empty() { Some(i) } else { None }).collect();
    let mut mismatches = Vec::new();
    for (destination, &source) in source_for_destination.iter().enumerate() {
        if sets[destination] != sets[source] {
            mismatches.push(LeafMismatch {
                destination, source,
                source_only: sets[source].difference(&sets[destination]).copied().collect(),
                destination_only: sets[destination].difference(&sets[source]).copied().collect(),
            });
        }
    }
    let leafwise_equal = mismatches.is_empty();
    let cartesian_domain_equal = !empty_leaves.is_empty() || leafwise_equal;
    let nonvacuous_domain_compatible = empty_leaves.is_empty() && leafwise_equal;
    Ok(LeafPermutationAudit {
        source_for_destination: source_for_destination.to_vec(), mismatches, empty_leaves,
        leafwise_equal, cartesian_domain_equal, nonvacuous_domain_compatible,
        scope: EvidenceScope::FiniteCoordinateSetsOnly,
    })
}
