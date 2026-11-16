//! Exact binary-curve import and a bounded `m`-summand index-calculus
//! collector with the single- and double-large-prime variations.
//!
//! This is deliberately a correctness engine, not a scaling claim.  It
//! enumerates the meet-in-the-middle state space exactly and refuses before
//! allocation when `max_states` is too small.  In particular, `m = n - 1`
//! means exactly that; it is never replaced by a cheaper decomposition.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::binary_ecc::curve::scalar_mul;
use crate::binary_ecc::{BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use crate::cryptanalysis::gaudry_cubic::{LargePrimeEliminator, SparseRel};
use crate::cryptanalysis::ic_boundary::{
    ArtinSchreier, BinaryGroup, BinaryInstance, CountedGroup, GroupOps, IncrementalGauss, RowStatus,
};
use crate::cryptanalysis::ic_measurement as measurement;
use crate::cryptanalysis::koblitz_fast::{FastCurve, FastPoint};
use crate::cryptanalysis::koblitz_index_calculus::{
    frobenius_eigenvalue, is_irreducible_f2, koblitz_point_count, KoblitzCurve,
};
use crate::cryptanalysis::large_prime_filter as lpf;
use crate::cryptanalysis::residual_walk::is_prime_u64;
use crate::cryptanalysis::semaev_decomp::Gf2;

pub mod orbit;
pub mod trace_line;
use orbit::{LambdaPowers, RelationFrame};

pub const MANIFEST_SCHEMA: &str = "ecbench.curve-corpus/v1";
pub const MAX_MANIFEST_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusManifest {
    #[serde(default)]
    pub schema: Option<String>,
    pub families: Vec<CorpusFamily>,
}

/// The parsed corpus and the identity of the exact bytes that supplied it.
pub struct LoadedManifest {
    pub manifest: CorpusManifest,
    pub sha256: String,
    pub bytes: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusFamily {
    pub id: String,
    pub field: CorpusField,
    pub curve: CorpusCurve,
    pub factor_base: CorpusFactorBase,
    pub dlog_challenge: CorpusChallenge,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusField {
    pub characteristic: u32,
    pub degree: u32,
    pub modulus: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusCurve {
    pub equation: String,
    pub order: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusFactorBase {
    pub dimension: u32,
    pub small_dimension: u32,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CorpusPoint {
    pub x: u64,
    pub y: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CorpusChallenge {
    pub base_point: CorpusPoint,
    pub target_point: CorpusPoint,
    pub group_order: u64,
    pub subgroup_order: u64,
    pub cofactor: u64,
    pub planted_log: u64,
}

pub struct ImportedFamily {
    pub id: String,
    pub instance: BinaryInstance,
    pub target: FastPoint,
    pub small_dimension: u32,
    pub envelope_dimension: u32,
    /// Used by the CLI only after the solver returns; never passed to
    /// [`solve`].
    pub expected: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ValidationFacts {
    pub id: String,
    pub n: u32,
    pub modulus: u64,
    pub group_order: u64,
    pub subgroup_order: u64,
    pub cofactor: u64,
    pub small_dimension: u32,
    pub envelope_dimension: u32,
    pub base_point: CorpusPoint,
    pub target_point: CorpusPoint,
    pub checks: Vec<String>,
}

fn parse_manifest(bytes: &[u8]) -> Result<CorpusManifest, String> {
    let manifest: CorpusManifest =
        serde_json::from_slice(bytes).map_err(|e| format!("parse corpus manifest: {e}"))?;
    if manifest.families.is_empty() {
        return Err("corpus manifest contains no families".into());
    }
    if let Some(schema) = &manifest.schema {
        if schema != MANIFEST_SCHEMA {
            return Err(format!("unsupported corpus schema `{schema}`"));
        }
    }
    let mut ids = BTreeSet::new();
    for family in &manifest.families {
        if family.id.trim().is_empty() {
            return Err("corpus manifest contains an empty family id".into());
        }
        if !ids.insert(family.id.as_str()) {
            return Err(format!(
                "corpus manifest contains duplicate family id `{}`",
                family.id
            ));
        }
    }
    Ok(manifest)
}

/// Parse the external corpus and retain the identity of the bytes read.
pub fn load_manifest_with_source(path: &Path) -> Result<LoadedManifest, String> {
    let file =
        File::open(path).map_err(|e| format!("read corpus manifest {}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(MAX_MANIFEST_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|e| format!("read corpus manifest {}: {e}", path.display()))?;
    if bytes.len() as u64 > MAX_MANIFEST_BYTES {
        return Err(format!(
            "corpus manifest {} exceeds the {} byte limit",
            path.display(),
            MAX_MANIFEST_BYTES
        ));
    }
    let sha256 = hex::encode(crate::hash::sha256::sha256(&bytes));
    let manifest = parse_manifest(&bytes)?;
    Ok(LoadedManifest {
        manifest,
        sha256,
        bytes: bytes.len(),
    })
}

/// Parse an external corpus. Call [`load_manifest_with_source`] when a report
/// must retain the input's content identity.
pub fn load_manifest(path: &Path) -> Result<CorpusManifest, String> {
    Ok(load_manifest_with_source(path)?.manifest)
}

/// Construct an exact explicit binary instance and check every fact that can
/// be checked without re-counting all `2^n` abscissae.
pub fn binary_explicit_instance(
    n: u32,
    modulus: u64,
    a: u64,
    b: u64,
    group_order: u64,
    r: u64,
    gx: u64,
    gy: u64,
) -> Result<BinaryInstance, String> {
    if !(3..=FastCurve::MAX_DEGREE).contains(&n) {
        return Err(format!("binary_explicit takes 3 <= n <= 62, not {n}"));
    }
    let top = 1u64 << n;
    if modulus & top == 0 || modulus >> (n + 1) != 0 {
        return Err(format!("modulus 0x{modulus:x} does not have degree {n}"));
    }
    if !is_irreducible_f2(modulus) {
        return Err(format!("modulus 0x{modulus:x} is reducible over F_2"));
    }
    if a >= top || b == 0 || b >= top {
        return Err("binary_explicit requires a,b in the field and b != 0".into());
    }
    if r < 3 || !is_prime_u64(r) || !group_order.is_multiple_of(r) {
        return Err(format!(
            "subgroup order {r} must be prime and divide group order {group_order}"
        ));
    }
    let low_terms = (0..n).filter(|i| (modulus >> i) & 1 == 1).collect();
    let irreducible = IrreduciblePoly {
        degree: n,
        low_terms,
    };
    let gf = Gf2::new(&irreducible);
    let artin_schreier = ArtinSchreier::new(&gf);
    let shell = BinaryCurve {
        m: n,
        irreducible: irreducible.clone(),
        a: gf.to_element(a),
        b: gf.to_element(b),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(r),
        cofactor: BigUint::from(group_order / r),
    };
    let fast = FastCurve::new(&shell).ok_or("explicit binary curve is too wide")?;
    let generator = FastPoint::affine(gx, gy);
    if !fast.is_on_curve(generator) {
        return Err("explicit binary generator is not on the curve".into());
    }
    if !fast.mul_u64(generator, r).infinity {
        return Err("[r]G is not the identity".into());
    }
    let general = BinaryCurve {
        generator: fast.lower(generator),
        ..shell
    };
    // A curve with a,b in F_2 and b=1 is genuinely Koblitz regardless of
    // which irreducible polynomial represents F_{2^n}. Certify its point
    // count and Frobenius eigenvalue so ecbench can expose the matched 2n
    // automorphisms while this module still uses its polynomial-basis base.
    let koblitz = if a <= 1 && b == 1 {
        let expected = koblitz_point_count(a as u8, n);
        if expected != BigUint::from(group_order) {
            return Err(format!(
                "Koblitz recurrence gives #E={expected}, not the supplied {group_order}"
            ));
        }
        let trace = if a == 0 { -1 } else { 1 };
        let order = BigUint::from(r);
        let lambda = frobenius_eigenvalue(&general, trace, &order)
            .ok_or("could not certify the Frobenius eigenvalue")?;
        Some(KoblitzCurve {
            a: a as u8,
            n,
            curve: general,
            trace,
            group_order: BigUint::from(group_order),
            subgroup_order: order,
            cofactor: BigUint::from(group_order / r),
            lambda,
            frobenius_is_endomorphism: true,
            k: 1,
            q: 2,
            a_index: a,
            b_index: 1,
            subfield_basis: vec![F2mElement::one(n)],
        })
    } else {
        None
    };
    let mut instance = BinaryInstance {
        name: String::new(),
        n,
        irreducible,
        fast,
        gf,
        artin_schreier,
        a,
        b,
        group_order,
        r,
        cofactor: group_order / r,
        generator,
        koblitz,
    };
    instance.name = instance.curve_id().slug;
    Ok(instance)
}

pub fn import_family(family: &CorpusFamily) -> Result<(ImportedFamily, ValidationFacts), String> {
    if family.field.characteristic != 2 {
        return Err(format!("{} is not a binary-field family", family.id));
    }
    if family.curve.equation.replace(' ', "") != "y^2+x*y=x^3+x^2+1" {
        return Err(format!(
            "{} has unsupported equation `{}`",
            family.id, family.curve.equation
        ));
    }
    let c = &family.dlog_challenge;
    if family.curve.order != c.group_order {
        return Err(format!("{} disagrees on the curve group order", family.id));
    }
    if c.cofactor.checked_mul(c.subgroup_order) != Some(c.group_order) {
        return Err(format!("{} does not satisfy #E = h*r", family.id));
    }
    if family.factor_base.small_dimension > family.factor_base.dimension
        || family.factor_base.dimension > family.field.degree
    {
        return Err(format!(
            "{} has invalid nested factor-base dimensions",
            family.id
        ));
    }
    let instance = binary_explicit_instance(
        family.field.degree,
        family.field.modulus,
        1,
        1,
        c.group_order,
        c.subgroup_order,
        c.base_point.x,
        c.base_point.y,
    )?;
    let target = FastPoint::affine(c.target_point.x, c.target_point.y);
    if !instance.fast.is_on_curve(target) {
        return Err(format!("{} target is not on the curve", family.id));
    }
    if !instance.fast.mul_u64(target, instance.r).infinity {
        return Err(format!(
            "{} target is not in the order-r subgroup",
            family.id
        ));
    }
    if instance
        .fast
        .mul_u64(instance.generator, c.planted_log % instance.r)
        != target
    {
        return Err(format!(
            "{} planted public equality does not hold",
            family.id
        ));
    }
    independent_verify(&instance, target, c.planted_log % instance.r)?;
    let facts = ValidationFacts {
        id: family.id.clone(),
        n: family.field.degree,
        modulus: family.field.modulus,
        group_order: c.group_order,
        subgroup_order: c.subgroup_order,
        cofactor: c.cofactor,
        small_dimension: family.factor_base.small_dimension,
        envelope_dimension: family.factor_base.dimension,
        base_point: c.base_point,
        target_point: c.target_point,
        checks: vec![
            "irreducible_modulus".into(),
            "supported_curve_equation".into(),
            "group_order_equals_cofactor_times_prime_subgroup_order".into(),
            "base_and_target_on_curve".into(),
            "base_and_target_in_subgroup".into(),
            "planted_public_equality_fast_and_independent".into(),
        ],
    };
    Ok((
        ImportedFamily {
            id: family.id.clone(),
            instance,
            target,
            small_dimension: family.factor_base.small_dimension,
            envelope_dimension: family.factor_base.dimension,
            expected: c.planted_log % c.subgroup_order,
        },
        facts,
    ))
}

fn general_curve(instance: &BinaryInstance) -> BinaryCurve {
    BinaryCurve {
        m: instance.n,
        irreducible: instance.irreducible.clone(),
        a: instance.gf.to_element(instance.a),
        b: instance.gf.to_element(instance.b),
        generator: instance.fast.lower(instance.generator),
        order: BigUint::from(instance.r),
        cofactor: BigUint::from(instance.cofactor),
    }
}

pub fn independent_verify(
    instance: &BinaryInstance,
    target: FastPoint,
    scalar: u64,
) -> Result<(), String> {
    let curve = general_curve(instance);
    let got = scalar_mul(&curve, &curve.generator, &BigUint::from(scalar));
    if got != instance.fast.lower(target) {
        return Err("independent general-arithmetic verification failed".into());
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationClass {
    Full,
    SingleLargePrime,
    DoubleLargePrime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialRelation {
    pub trial: u64,
    pub a: u64,
    pub b: u64,
    pub class: RelationClass,
    pub raw_terms: Vec<usize>,
    pub columns: Vec<(usize, u64)>,
}

/// How partial relations with large primes are combined into complete ones.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LargePrimeGraph {
    /// The original streaming pivot elimination ([`LargePrimeEliminator`]).
    /// This is the default, and every report produced before the orbit
    /// quotient existed came from it.
    #[default]
    Online,
    /// Collect every partial relation, then run the batch
    /// [`lpf::filter_large_primes`] over *plain* large columns: one vertex
    /// per canonical projected point, coefficients `+-1`.  Every emitted
    /// complete relation is replayed against the curve.
    Filter,
    /// As [`Self::Filter`], but a large vertex is a signed Frobenius orbit
    /// and the column coefficient carries the phase, `+-lambda^k`.
    FilterOrbit,
    /// As [`Self::FilterOrbit`], and a large prime whose orbit contains a
    /// small (factor-base) column is rewritten into that small column.
    FilterOrbitAbsorb,
    /// The controlled version of [`Self::FilterOrbit`]: decompositions are
    /// accepted with the **plain** column count (so the accepted relation
    /// stream is exactly that of [`Self::Filter`]), and only the stored
    /// relations are re-expressed in orbit columns before the batch filter.
    ///
    /// [`Self::FilterOrbit`] counts large *orbit* columns when it decides
    /// whether a decomposition is acceptable, and the coarser column map
    /// accepts a superset of what [`Self::Filter`] accepts (and can pick a
    /// different first-hit decomposition of a trial).  The two therefore do
    /// not see an identical relation stream; this arm does.
    FilterOrbitPostCollect,
}

impl LargePrimeGraph {
    /// `serde` skip predicate: the default arm adds no field to a report.
    pub fn is_online(&self) -> bool {
        matches!(self, Self::Online)
    }

    pub fn uses_filter(self) -> bool {
        !self.is_online()
    }

    /// Whether large columns are quotiented by Frobenius orbits at all.
    pub fn orbit(self) -> bool {
        matches!(
            self,
            Self::FilterOrbit | Self::FilterOrbitAbsorb | Self::FilterOrbitPostCollect
        )
    }

    pub fn absorb(self) -> bool {
        matches!(self, Self::FilterOrbitAbsorb)
    }

    /// Whether the orbit map is applied only after collection, leaving the
    /// acceptance of decompositions on the plain column count.
    pub fn post_collect(self) -> bool {
        matches!(self, Self::FilterOrbitPostCollect)
    }
}

/// Optional instrumentation of a [`solve_with_options`] call.  The default
/// adds nothing to a report, so `solve` output is unchanged (`charge_checks`
/// is an accounting rule, not instrumentation, and is off by default).
#[derive(Clone, Debug, Default)]
pub struct SolveOptions {
    /// Explicit nested binary basis for abscissae. `None` preserves the
    /// historical polynomial-prefix base and its sealed replay records.
    pub abscissa_basis: Option<Vec<u64>>,
    /// The `_v2` accounting (`ic.large_prime_v2`): the target's `[r]Q`
    /// subgroup-membership check and the final `[d]G = Q` check are counted
    /// scalar multiplications in the verification phase.  Off, they are not
    /// charged (the membership check runs on a ledger nobody reads, the final
    /// check is the runner's), which is the first rule.
    pub charge_checks: bool,
    /// Hash every accepted decomposition `(trial, a, b, raw terms)` in order
    /// into [`FilterPipelineReport::stream`], so two arms can be shown to have
    /// seen the same relation stream.
    pub stream_digest: bool,
    /// Re-run the batch pipeline on prefixes of the stored relation stream to
    /// find, without a trial cap, how many trials a given verified rank takes.
    /// Filter arms only.
    pub trajectory: Option<TrajectoryRequest>,
    /// Test hook: damage one entry of the finished orbit table *after* its
    /// self-checks, to show that the replays (not the table build) are what
    /// catch a wrong coefficient.  Never set outside tests.
    #[doc(hidden)]
    pub table_fault: Option<TableFault>,
}

/// Which prefixes of the stored relation stream [`SolveOptions::trajectory`]
/// reports in full (in addition to the first-hit search).
#[derive(Clone, Debug, Default)]
pub struct TrajectoryRequest {
    /// Trial cutoffs: the prefix of stored relations with `trial <= cutoff`.
    pub cutoffs: Vec<u64>,
}

/// A deliberate defect in the orbit table, for negative tests.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableFault {
    /// Multiply the factor of absorbed large point number `index` (in table
    /// order; `None` = every one) by 2, which breaks `[factor]S = Q`.
    AbsorbFactor { index: Option<usize> },
    /// Multiply the factor of the *second* member of multi-member orbit number
    /// `index` (`None` = of every multi-member orbit) by lambda: the
    /// Frobenius phase of that member is off by one.  Only one member is
    /// damaged on purpose: a damage shared by all members of an orbit is
    /// the rescaling of one unknown, which the elimination absorbs, so only
    /// a wrong *relative* factor makes a relation false.
    OrbitFactor { index: Option<usize> },
}

#[derive(Clone, Debug, Serialize)]
pub struct SolveConfig {
    pub small_dimension: u32,
    pub envelope_dimension: u32,
    pub summands: u32,
    pub max_large_primes: u8,
    pub max_trials: u64,
    pub max_states: u64,
    pub seed: u64,
    /// Off by default: [`LargePrimeGraph::Online`] is the original code path
    /// and serialises without this field, so existing reports are unchanged.
    #[serde(skip_serializing_if = "LargePrimeGraph::is_online")]
    pub large_prime_graph: LargePrimeGraph,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SolveCounters {
    pub trials: u64,
    pub decompositions: u64,
    pub rejected_large_prime_count: u64,
    pub relation_histogram: [u64; 4],
    pub accepted_relations: u64,
    pub eliminated_full_relations: u64,
    pub independent_rows: u64,
    pub matrix_rank: u64,
    pub mitm_lookups_uncharged: u64,
    pub combination_states_uncharged: u64,
    pub lp_merge_ops_uncharged: u64,
    pub row_ops_uncharged: u64,
    pub lp_pivots: u64,
    /// Scalar multiplications the `_v2` accounting charges to the
    /// verification phase beyond the relation witness checks: the target's
    /// `[r]Q` membership check and the final `[d]G = Q`.  Zero (and absent
    /// from the report) for the first rule.
    #[serde(skip_serializing_if = "is_zero")]
    pub membership_checks: u64,
    #[serde(skip_serializing_if = "is_zero")]
    pub final_checks: u64,
    #[serde(skip_serializing_if = "is_zero")]
    pub final_checks_failed: u64,
}

fn is_zero(v: &u64) -> bool {
    *v == 0
}

#[derive(Clone, Debug, Serialize)]
pub struct FactorBaseReport {
    pub raw_points: usize,
    pub small_raw_points: usize,
    pub columns: usize,
    pub small_columns: usize,
    pub large_columns: usize,
    /// Canonical projected point keys in column order. The native ecbench
    /// adapter hashes these into its factor-base identity; reports omit them.
    #[serde(skip)]
    pub column_keys: Vec<u64>,
    /// Present only when the large columns were quotiented by Frobenius.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orbit: Option<OrbitFactorBaseReport>,
}

/// One large point as seen from its orbit representative.
#[derive(Clone, Debug, Serialize)]
pub struct OrbitMember {
    pub x: u64,
    pub y: u64,
    /// `k` with `pi^k(point) = +-rep`.
    pub phase: u32,
    pub negated: bool,
    /// `log point = factor * log rep (mod r)`.
    pub factor: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrbitExample {
    pub rep_x: u64,
    pub rep_y: u64,
    pub members: Vec<OrbitMember>,
}

/// What the Frobenius quotient did to the large columns.
#[derive(Clone, Debug, Serialize)]
pub struct OrbitFactorBaseReport {
    pub lambda: u64,
    pub absorb_into_small: bool,
    /// Distinct large projected points before the quotient.
    pub large_points: usize,
    /// Large columns after the quotient.
    pub large_orbits: usize,
    /// Large points that share an orbit vertex with another large point.
    pub merged_points: usize,
    /// Large points rewritten into a small column (absorb arm only).
    pub absorbed_into_small: usize,
    /// Large points whose orbit representative is the negative of the
    /// Frobenius image, so the factor carries a sign.
    pub negated_points: usize,
    /// How many large points needed Frobenius power `k = index`.
    pub phase_histogram: Vec<u64>,
    /// Up to eight orbits with more than one member.
    pub examples: Vec<OrbitExample>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MitmReport {
    pub left_terms: u32,
    pub right_terms: u32,
    pub left_states: u64,
    pub right_states: u64,
}

/// Process CPU time by phase, nanoseconds.  Setup is the factor base and the
/// exact combination table, which every arm builds identically.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct PhaseCpuNs {
    pub setup: u64,
    /// The trial loop: target residuals and the meet-in-the-middle probe.
    pub collect: u64,
    pub filter: u64,
    /// Linear replay and curve replay of every emitted complete relation,
    /// plus the incremental rank update.
    pub replay: u64,
    /// Curve-level audit of every input partial row.  Reported separately
    /// and excluded from the online cost.
    pub audit: u64,
}

/// The batch pipeline's accounting; present only for the filter arms.
#[derive(Clone, Debug, Serialize)]
pub struct FilterPipelineReport {
    pub graph: LargePrimeGraph,
    /// Partial relations by class: full, single large prime, double.
    pub partials_by_class: [u64; 3],
    pub filter: lpf::LargePrimeFilterReport,
    pub complete_relations: u64,
    pub replays_verified: u64,
    pub replays_failed: u64,
    /// Failures of the linear replay against the certificate.
    pub replay_failed_linear: u64,
    /// Failures of the group identity checked with scalar multiplications.
    pub replay_failed_curve: u64,
    pub independent_verified: u64,
    pub dependent_verified: u64,
    /// Input partial rows whose own group identity does not hold.
    pub partial_audit_failed: u64,
    pub rank: u64,
    pub cpu_ns: PhaseCpuNs,
    /// Only with [`SolveOptions::stream_digest`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<StreamReport>,
    /// Only for [`LargePrimeGraph::FilterOrbitPostCollect`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_collect: Option<PostCollectReport>,
    /// Only with [`SolveOptions::trajectory`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trajectory: Option<RankTrajectory>,
}

/// The accepted-decomposition stream of one run.
#[derive(Clone, Debug, Serialize)]
pub struct StreamReport {
    /// BLAKE3 over `(trial, a, b, number of terms, term atom indices)` of
    /// every accepted decomposition, in trial order.
    pub digest: String,
    pub accepted: u64,
}

/// What re-expressing the plain stream in orbit columns did.
#[derive(Clone, Debug, Serialize)]
pub struct PostCollectReport {
    /// Stored partial relations by number of large *orbit* columns (0, 1, 2).
    /// `FilterPipelineReport::partials_by_class` is the plain class and equals
    /// the plain arm's.
    pub partials_by_orbit_class: [u64; 3],
    /// Relations whose number of large columns fell under the orbit map.
    pub class_changed: u64,
}

/// First prefix of the stream that reaches a target.
#[derive(Clone, Debug, Serialize)]
pub struct TrajectoryHit {
    /// `rank>=q` or `recovery` (the target column is pinned).
    pub target: String,
    pub rank: u64,
    /// Stored partial relations in the first prefix that reaches the target;
    /// `None` when no prefix of the run does (censored).
    pub partials: Option<u64>,
    /// The trial index of the last relation of that prefix.
    pub trial: Option<u64>,
}

/// The state of one prefix (`trial <= cutoff_trial`) of the stream.
#[derive(Clone, Debug, Serialize)]
pub struct TrajectoryPoint {
    pub cutoff_trial: u64,
    pub partials: u64,
    pub complete: u64,
    pub independent: u64,
    pub rank: u64,
    pub pinned: bool,
}

/// Cap-free yield: how long the stream takes to reach a given verified rank.
#[derive(Clone, Debug, Serialize)]
pub struct RankTrajectory {
    /// Columns of the rank (small columns plus the target column).
    pub columns: u64,
    pub stored_partials: u64,
    /// Prefix evaluations performed (each one is a filter run, both replays
    /// and a rank update).
    pub evaluations: u64,
    /// The rank never decreased along the scanned prefixes.
    pub monotone: bool,
    /// The full prefix reproduces the run's own complete/independent/rank.
    pub consistent_with_pipeline: bool,
    pub first_hits: Vec<TrajectoryHit>,
    pub at_cutoffs: Vec<TrajectoryPoint>,
}

#[derive(Clone, Debug, Serialize)]
pub struct LargePrimeReport {
    /// Present only for explicit-base runs; relation_samples then retains
    /// every accepted relation, rather than just the first twelve.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_evidence: Option<trace_line::Evidence>,
    pub recovered: Option<u64>,
    pub exhausted: bool,
    pub verified_fast: bool,
    pub verified_independent: bool,
    pub verification_performed: bool,
    pub config: SolveConfig,
    pub factor_base: FactorBaseReport,
    pub mitm: MitmReport,
    pub counters: SolveCounters,
    pub phase_group_ops: BTreeMap<String, GroupOps>,
    pub solve_wall_ns: u64,
    pub relation_samples: Vec<PartialRelation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_pipeline: Option<FilterPipelineReport>,
}

#[derive(Clone)]
struct Atom {
    raw: FastPoint,
    column: Option<usize>,
    coefficient: u64,
    small: bool,
}

struct FactorBase {
    atoms: Vec<Atom>,
    small_columns: usize,
    /// Group element of each column (index = column): the canonical
    /// projected point of a plain column, the representative of an orbit.
    column_points: Vec<FastPoint>,
    report: FactorBaseReport,
}

fn canonical(curve: &FastCurve, p: FastPoint) -> (FastPoint, bool) {
    let neg = curve.neg(p);
    if (p.x, p.y) <= (neg.x, neg.y) {
        (p, false)
    } else {
        (neg, true)
    }
}

/// The Frobenius quotient requested for the large columns.
struct OrbitPlan {
    powers: LambdaPowers,
    absorb: bool,
}

fn build_factor_base(
    instance: &BinaryInstance,
    small_dimension: u32,
    envelope_dimension: u32,
    orbit: Option<&OrbitPlan>,
    fault: Option<TableFault>,
    basis: Option<&[u64]>,
    ops: &mut GroupOps,
) -> Result<FactorBase, String> {
    if small_dimension > envelope_dimension || envelope_dimension > instance.n {
        return Err("factor-base dimensions must satisfy small <= envelope <= n".into());
    }
    if envelope_dimension >= 63 {
        return Err("factor-base envelope does not fit a word".into());
    }
    if let Some(basis) = basis {
        trace_line::validate_basis(instance, basis, envelope_dimension)?;
    }
    let small_x = 1u64 << small_dimension;
    let envelope_x = 1u64 << envelope_dimension;
    let group = BinaryGroup(&instance.fast);
    let mut raw = Vec::new();
    let mut keys: BTreeMap<u64, bool> = BTreeMap::new();
    let mut points_by_key: BTreeMap<u64, FastPoint> = BTreeMap::new();
    for index in 0..envelope_x {
        let x = basis.map_or(index, |basis| trace_line::expand(basis, index));
        for p in instance.points_with_x(x) {
            let projected = group.mul(ops, p, instance.cofactor);
            if !projected.infinity {
                let (c, _) = canonical(&instance.fast, projected);
                keys.entry(c.pack())
                    .and_modify(|s| *s |= index < small_x)
                    .or_insert(index < small_x);
                points_by_key.entry(c.pack()).or_insert(c);
            }
            raw.push((p, projected, index < small_x));
        }
    }
    let r = instance.r;
    let mut col_of = HashMap::new();
    let mut factor_of: HashMap<u64, u64> = HashMap::new();
    let mut column_points: Vec<FastPoint> = Vec::new();
    let mut column_keys: Vec<u64> = Vec::new();
    let mut next = 0usize;
    for (&key, &small) in &keys {
        if small {
            col_of.insert(key, next);
            column_points.push(points_by_key[&key]);
            column_keys.push(key);
            next += 1;
        }
    }
    let small_columns = next;
    let mut orbit_report = None;
    if let Some(plan) = orbit {
        let fast = &instance.fast;
        let mut phase_histogram = vec![0u64; instance.n as usize];
        let mut negated_points = 0usize;
        let mut large_points = 0usize;
        // Orbit representative key -> (representative, members).
        let mut groups: BTreeMap<u64, (FastPoint, Vec<(u64, orbit::OrbitCanon)>)> = BTreeMap::new();
        for (&key, &small) in &keys {
            if small {
                continue;
            }
            large_points += 1;
            let point = points_by_key[&key];
            let canon = orbit::canonicalise(instance, &plan.powers, point);
            // The convention (direction of the phase, sign) is checked
            // against the real curve arithmetic for every large point.
            if fast.mul_u64(canon.rep, canon.factor) != point {
                return Err(format!(
                    "orbit table self-check failed for projected point {key}"
                ));
            }
            phase_histogram[canon.phase as usize] += 1;
            negated_points += usize::from(canon.negated);
            groups
                .entry(canon.rep.pack())
                .or_insert_with(|| (canon.rep, Vec::new()))
                .1
                .push((key, canon));
        }
        let mut small_by_rep: BTreeMap<u64, (usize, u64)> = BTreeMap::new();
        if plan.absorb {
            for (&key, &small) in &keys {
                if small {
                    let point = points_by_key[&key];
                    let canon = orbit::canonicalise(instance, &plan.powers, point);
                    // The factors of the small columns are used by the
                    // absorb arm, so they get the same curve check as the
                    // large points' factors: `[factor]rep == S`.
                    if fast.mul_u64(canon.rep, canon.factor) != point {
                        return Err(format!(
                            "orbit table self-check failed for small projected point {key}"
                        ));
                    }
                    small_by_rep
                        .entry(canon.rep.pack())
                        .or_insert((col_of[&key], canon.factor));
                }
            }
        }
        let mut absorbed = 0usize;
        let mut merged_orbits = 0usize;
        let mut examples = Vec::new();
        for (rep_key, (rep, members)) in &groups {
            if let Some(&(column, small_factor)) = small_by_rep.get(rep_key) {
                // log Q = f_Q log rep and log S = f_S log rep, so
                // log Q = f_Q f_S^{-1} log S.
                let inverse = orbit::inv_mod(small_factor, r).ok_or("zero orbit factor")?;
                for (key, canon) in members {
                    let factor = orbit::mul_mod(canon.factor, inverse, r);
                    // The inversion is checked on the curve as well:
                    // `[f_Q f_S^{-1}] S == Q`, S being the small column's point.
                    if fast.mul_u64(column_points[column], factor) != points_by_key[key] {
                        return Err(format!(
                            "absorb table self-check failed for projected point {key}"
                        ));
                    }
                    let factor = match fault {
                        Some(TableFault::AbsorbFactor { index })
                            if index.is_none_or(|i| i == absorbed) =>
                        {
                            orbit::mul_mod(factor, 2, r)
                        }
                        _ => factor,
                    };
                    col_of.insert(*key, column);
                    factor_of.insert(*key, factor);
                    absorbed += 1;
                }
                continue;
            }
            let column = next;
            next += 1;
            column_points.push(*rep);
            column_keys.push(*rep_key);
            for (position, (key, canon)) in members.iter().enumerate() {
                let factor = match fault {
                    Some(TableFault::OrbitFactor { index })
                        if position == 1 && index.is_none_or(|i| i == merged_orbits) =>
                    {
                        orbit::mul_mod(canon.factor, plan.powers.lambda, r)
                    }
                    _ => canon.factor,
                };
                col_of.insert(*key, column);
                factor_of.insert(*key, factor);
            }
            merged_orbits += usize::from(members.len() > 1);
            if members.len() > 1 && examples.len() < 8 {
                examples.push(OrbitExample {
                    rep_x: rep.x,
                    rep_y: rep.y,
                    members: members
                        .iter()
                        .map(|(key, canon)| {
                            let point = points_by_key[key];
                            OrbitMember {
                                x: point.x,
                                y: point.y,
                                phase: canon.phase,
                                negated: canon.negated,
                                factor: canon.factor,
                            }
                        })
                        .collect(),
                });
            }
        }
        let large_orbits = next - small_columns;
        orbit_report = Some(OrbitFactorBaseReport {
            lambda: plan.powers.lambda,
            absorb_into_small: plan.absorb,
            large_points,
            large_orbits,
            merged_points: large_points - absorbed - large_orbits,
            absorbed_into_small: absorbed,
            negated_points,
            phase_histogram,
            examples,
        });
    } else {
        for (&key, &small) in &keys {
            if !small {
                col_of.insert(key, next);
                column_points.push(points_by_key[&key]);
                column_keys.push(key);
                next += 1;
            }
        }
    }
    let atoms = raw
        .into_iter()
        .map(|(raw, projected, small)| {
            if projected.infinity {
                Atom {
                    raw,
                    column: None,
                    coefficient: 0,
                    small,
                }
            } else {
                let (c, negated) = canonical(&instance.fast, projected);
                let key = c.pack();
                let sign = if negated { r - 1 } else { 1 };
                let coefficient = match factor_of.get(&key) {
                    Some(&factor) => orbit::mul_mod(sign, factor, r),
                    None => sign,
                };
                Atom {
                    raw,
                    column: Some(col_of[&key]),
                    coefficient,
                    small,
                }
            }
        })
        .collect::<Vec<_>>();
    let small_raw_points = atoms.iter().filter(|a| a.small).count();
    Ok(FactorBase {
        report: FactorBaseReport {
            raw_points: atoms.len(),
            small_raw_points,
            columns: next,
            small_columns,
            large_columns: next - small_columns,
            column_keys,
            orbit: orbit_report,
        },
        atoms,
        small_columns,
        column_points,
    })
}

#[derive(Clone)]
struct State {
    sum: FastPoint,
    terms: Vec<usize>,
}

struct Decomposer {
    left: HashMap<FastPoint, Vec<Vec<usize>>>,
    right: Vec<State>,
    report: MitmReport,
}

fn combinations(n: usize, k: usize) -> u128 {
    if k == 0 {
        return 1;
    }
    let mut v = 1u128;
    for i in 1..=k {
        v = v.saturating_mul((n + i - 1) as u128) / i as u128;
    }
    v
}

fn enumerate_states(
    group: &BinaryGroup<'_>,
    atoms: &[Atom],
    terms: usize,
    ops: &mut GroupOps,
) -> Vec<State> {
    fn visit(
        group: &BinaryGroup<'_>,
        atoms: &[Atom],
        left: usize,
        start: usize,
        acc: FastPoint,
        current: &mut Vec<usize>,
        out: &mut Vec<State>,
        ops: &mut GroupOps,
    ) {
        if left == 0 {
            out.push(State {
                sum: acc,
                terms: current.clone(),
            });
            return;
        }
        for i in start..atoms.len() {
            let sum = group.add(ops, acc, atoms[i].raw);
            current.push(i);
            visit(group, atoms, left - 1, i, sum, current, out, ops);
            current.pop();
        }
    }
    let cap = combinations(atoms.len(), terms).min(usize::MAX as u128) as usize;
    let mut out = Vec::with_capacity(cap);
    visit(
        group,
        atoms,
        terms,
        0,
        FastPoint::INFINITY,
        &mut Vec::with_capacity(terms),
        &mut out,
        ops,
    );
    out
}

/// The columns and coefficients (mod `r`) of the decomposition `terms` under
/// the column map of `base`; zero coefficients are dropped.
fn project_terms(base: &FactorBase, terms: &[usize], r: u64) -> Vec<(usize, u64)> {
    let mut coeffs: BTreeMap<usize, u64> = BTreeMap::new();
    for &i in terms {
        let atom = &base.atoms[i];
        if let Some(c) = atom.column {
            let v = coeffs.entry(c).or_insert(0);
            *v = ((*v as u128 + atom.coefficient as u128) % r as u128) as u64;
        }
    }
    coeffs.into_iter().filter(|(_, v)| *v != 0).collect()
}

impl Decomposer {
    fn new(
        group: &BinaryGroup<'_>,
        base: &FactorBase,
        summands: u32,
        max_states: u64,
        ops: &mut GroupOps,
    ) -> Result<Self, String> {
        if summands < 2 {
            return Err("summands must be at least 2".into());
        }
        let left_terms = summands.div_ceil(2);
        let right_terms = summands - left_terms;
        let lc = combinations(base.atoms.len(), left_terms as usize);
        let rc = combinations(base.atoms.len(), right_terms as usize);
        let total = lc.saturating_add(rc);
        if total > max_states as u128 {
            return Err(format!(
                "exact {summands}-summand MITM needs {lc} + {rc} = {total} states, exceeding max_states={max_states}"
            ));
        }
        let left_states = enumerate_states(group, &base.atoms, left_terms as usize, ops);
        let right = enumerate_states(group, &base.atoms, right_terms as usize, ops);
        let mut left: HashMap<FastPoint, Vec<Vec<usize>>> = HashMap::new();
        for state in left_states {
            left.entry(state.sum).or_default().push(state.terms);
        }
        Ok(Self {
            left,
            report: MitmReport {
                left_terms,
                right_terms,
                left_states: lc as u64,
                right_states: rc as u64,
            },
            right,
        })
    }

    fn find(
        &self,
        group: &BinaryGroup<'_>,
        base: &FactorBase,
        target: FastPoint,
        max_large_primes: u8,
        r: u64,
        ops: &mut GroupOps,
        lookups: &mut u64,
    ) -> Option<(Vec<usize>, Vec<(usize, u64)>, usize)> {
        for right in &self.right {
            let need = group.add(ops, target, group.neg(right.sum));
            *lookups += 1;
            let Some(lefts) = self.left.get(&need) else {
                continue;
            };
            for left in lefts {
                let mut terms = left.clone();
                terms.extend_from_slice(&right.terms);
                let projected = project_terms(base, &terms, r);
                let lp = projected
                    .iter()
                    .filter(|(c, _)| *c >= base.small_columns)
                    .count();
                if lp <= max_large_primes as usize {
                    return Some((terms, projected, lp));
                }
            }
        }
        None
    }
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }
}

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Solve one target.  The planted logarithm is intentionally absent from the
/// signature.
pub fn solve(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
) -> Result<LargePrimeReport, String> {
    solve_with_verification(instance, target, config, true, SolveOptions::default())
}

/// [`solve`] with instrumentation ([`SolveOptions`]).  The default options
/// give exactly [`solve`]'s report.
pub fn solve_with_options(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
    options: SolveOptions,
) -> Result<LargePrimeReport, String> {
    solve_with_verification(instance, target, config, true, options)
}

/// Return the algebraic candidate without checking it against the target.
/// Native ecbench uses this entry point and verifies in its runner process.
pub fn solve_candidate(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
) -> Result<LargePrimeReport, String> {
    solve_with_verification(instance, target, config, false, SolveOptions::default())
}

/// [`solve_candidate`] under the `_v2` accounting (`ic.large_prime_v2`):
/// the target's `[r]Q` subgroup-membership check, which the first rule runs
/// on a ledger nobody reads, and the final `[d]G = Q` check, which it
/// leaves to the runner, are counted scalar multiplications in the
/// verification phase.  The search is the same.
pub fn solve_candidate_v2(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
) -> Result<LargePrimeReport, String> {
    solve_with_verification(
        instance,
        target,
        config,
        false,
        SolveOptions {
            charge_checks: true,
            ..Default::default()
        },
    )
}

/// An explicit-base candidate; the ecbench runner performs scalar verification.
pub fn solve_candidate_with_basis(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
    basis: Vec<u64>,
) -> Result<LargePrimeReport, String> {
    solve_with_verification(
        instance,
        target,
        config,
        false,
        SolveOptions {
            abscissa_basis: Some(basis),
            ..Default::default()
        },
    )
}

/// CPU time consumed by this process so far, in nanoseconds; 0 where the
/// clock is unavailable.  The collector is single-threaded, so this is the
/// collector's own CPU time.
pub fn process_cpu_ns() -> u64 {
    #[cfg(unix)]
    {
        let mut ts = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: clock_gettime only writes through the pointer to `ts`.
        let rc = unsafe { libc::clock_gettime(libc::CLOCK_PROCESS_CPUTIME_ID, &mut ts) };
        if rc == 0 {
            return ts.tv_sec as u64 * 1_000_000_000 + ts.tv_nsec as u64;
        }
    }
    0
}

/// An accepted partial relation held back for the batch filter.
struct StoredPartial {
    trial: u64,
    class: RelationClass,
    /// Columns in the collector's shifted indexing: small columns, then the
    /// target column `d`, then large columns.
    cols: Vec<(usize, u64)>,
    rhs: u64,
}

/// Combine every stored partial relation with the batch hypergraph filter,
/// replay each emitted complete relation, and feed only verified rows to the
/// rank tracker.
///
/// A complete relation is *verified* when both replays pass: the linear
/// replay of its source certificate must reproduce it exactly, and the group
/// identity it asserts must hold on the curve
/// ([`RelationFrame::row_holds`]).  The second replay is the one that
/// notices a wrong Frobenius phase coefficient.
fn run_filter_pipeline(
    instance: &BinaryInstance,
    target: FastPoint,
    base: &FactorBase,
    graph: LargePrimeGraph,
    stored: Vec<StoredPartial>,
    matrix: &mut IncrementalGauss,
    mut cpu: PhaseCpuNs,
    extras: PipelineExtras<'_>,
) -> Result<FilterPipelineReport, String> {
    let r = instance.r;
    let d_col = base.small_columns;
    let mut partials_by_class = [0u64; 3];
    let mut inputs = Vec::with_capacity(stored.len());
    for s in &stored {
        partials_by_class[match s.class {
            RelationClass::Full => 0,
            RelationClass::SingleLargePrime => 1,
            RelationClass::DoubleLargePrime => 2,
        }] += 1;
        let mut small = Vec::new();
        let mut large = Vec::new();
        for &(c, v) in &s.cols {
            if c <= d_col {
                small.push((c as u32, v));
            } else {
                large.push((c as u64, v));
            }
        }
        inputs.push(lpf::PartialRelation {
            id: s.trial,
            small,
            large,
            rhs: s.rhs,
        });
    }
    drop(stored);

    // Group element of every column id in the shifted indexing.
    let mut points = Vec::with_capacity(base.column_points.len() + 1);
    points.extend_from_slice(&base.column_points[..base.small_columns]);
    points.push(FastPoint::INFINITY);
    points.extend_from_slice(&base.column_points[base.small_columns..]);
    let frame = RelationFrame {
        instance,
        target,
        points: &points,
        d_col,
    };

    let t = process_cpu_ns();
    let options = lpf::LargePrimeFilterOptions {
        modulus: r,
        ..Default::default()
    };
    let (complete, filter_report) = lpf::filter_large_primes(inputs.clone(), &options)
        .map_err(|e| format!("large-prime filter refused its input: {e:?}"))?;
    cpu.filter = process_cpu_ns().saturating_sub(t);

    let t = process_cpu_ns();
    let index: HashMap<u64, usize> = inputs
        .iter()
        .enumerate()
        .map(|(i, row)| (row.id, i))
        .collect();
    let Verified {
        verified,
        failed_linear,
        failed_curve,
        failed,
        independent,
        dependent,
    } = verify_complete(&complete, &inputs, &index, &frame, r, d_col, matrix)?;
    cpu.replay = process_cpu_ns().saturating_sub(t);

    let t = process_cpu_ns();
    let mut audit_failed = 0u64;
    for row in &inputs {
        let terms = row
            .small
            .iter()
            .map(|&(c, v)| (c as u64, v))
            .chain(row.large.iter().copied());
        audit_failed += u64::from(!frame.row_holds(terms, row.rhs));
    }
    cpu.audit = process_cpu_ns().saturating_sub(t);

    // Instrumentation, outside every timed phase above.
    let trajectory = match extras.trajectory {
        Some(request) => Some(rank_trajectory(
            &inputs,
            &frame,
            r,
            d_col,
            request,
            (complete.len() as u64, independent, matrix.rank() as u64),
        )?),
        None => None,
    };

    Ok(FilterPipelineReport {
        graph,
        partials_by_class,
        filter: filter_report,
        complete_relations: complete.len() as u64,
        replays_verified: verified,
        replays_failed: failed,
        replay_failed_linear: failed_linear,
        replay_failed_curve: failed_curve,
        independent_verified: independent,
        dependent_verified: dependent,
        partial_audit_failed: audit_failed,
        rank: matrix.rank() as u64,
        cpu_ns: cpu,
        stream: extras.stream,
        post_collect: extras.post_collect,
        trajectory,
    })
}

/// Instrumentation handed to [`run_filter_pipeline`].
struct PipelineExtras<'a> {
    stream: Option<StreamReport>,
    post_collect: Option<PostCollectReport>,
    trajectory: Option<&'a TrajectoryRequest>,
}

#[derive(Default)]
struct Verified {
    verified: u64,
    failed_linear: u64,
    failed_curve: u64,
    failed: u64,
    independent: u64,
    dependent: u64,
}

/// Replay every complete relation twice and feed the verified ones to the
/// rank tracker.  `index` maps a partial relation's id to its position in
/// `inputs`.
fn verify_complete(
    complete: &[lpf::CompleteRelation],
    inputs: &[lpf::PartialRelation],
    index: &HashMap<u64, usize>,
    frame: &RelationFrame<'_>,
    r: u64,
    d_col: usize,
    matrix: &mut IncrementalGauss,
) -> Result<Verified, String> {
    let mut out = Verified::default();
    for c in complete {
        // Replay against only the rows the certificate names.
        let sources: Option<Vec<lpf::PartialRelation>> = c
            .sources
            .iter()
            .map(|(id, _)| index.get(id).map(|&i| inputs[i].clone()))
            .collect();
        let linear_ok = sources
            .and_then(|rows| lpf::replay_complete_relation(c, &rows, r))
            .is_some_and(|replayed| replayed.small == c.small && replayed.rhs == c.rhs);
        let curve_ok = frame.row_holds(c.small.iter().map(|&(col, v)| (col as u64, v)), c.rhs);
        out.failed_linear += u64::from(!linear_ok);
        out.failed_curve += u64::from(!curve_ok);
        if !(linear_ok && curve_ok) {
            out.failed += 1;
            continue;
        }
        out.verified += 1;
        let mut row = vec![0u64; d_col + 1];
        for &(col, v) in &c.small {
            row[col as usize] = v;
        }
        match matrix.add_row(row, c.rhs) {
            RowStatus::Independent => out.independent += 1,
            RowStatus::Dependent => out.dependent += 1,
            RowStatus::Inconsistent => {
                return Err("a verified relation made the system inconsistent".into());
            }
        }
    }
    Ok(out)
}

/// The state of the first `k` stored partial relations.
#[derive(Clone, Copy, PartialEq, Eq)]
struct PrefixState {
    complete: u64,
    independent: u64,
    rank: u64,
    pinned: bool,
}

/// Run the whole pipeline (filter, both replays, rank update) on a prefix of
/// the stored stream.
fn prefix_state(
    inputs: &[lpf::PartialRelation],
    k: usize,
    frame: &RelationFrame<'_>,
    r: u64,
    d_col: usize,
) -> Result<PrefixState, String> {
    let prefix = &inputs[..k];
    let options = lpf::LargePrimeFilterOptions {
        modulus: r,
        ..Default::default()
    };
    let (complete, _) = lpf::filter_large_primes(prefix.to_vec(), &options)
        .map_err(|e| format!("large-prime filter refused a prefix: {e:?}"))?;
    let index: HashMap<u64, usize> = prefix
        .iter()
        .enumerate()
        .map(|(i, row)| (row.id, i))
        .collect();
    let mut matrix = IncrementalGauss::new(d_col + 1, r);
    let v = verify_complete(&complete, prefix, &index, frame, r, d_col, &mut matrix)?;
    Ok(PrefixState {
        complete: complete.len() as u64,
        independent: v.independent,
        rank: matrix.rank() as u64,
        pinned: matrix.pinned(d_col).is_some(),
    })
}

/// How many trials each verified rank takes, found by re-running the pipeline
/// on prefixes of the stored relation stream (`inputs` is in trial order).
///
/// A coarse scan (about 64 prefixes) locates the first interval that reaches
/// each target, then every prefix inside that interval is evaluated, so the
/// reported prefix is the first one that reaches the target *given that the
/// rank does not decrease between scanned prefixes*; `monotone` reports
/// whether it did not along the scan.
fn rank_trajectory(
    inputs: &[lpf::PartialRelation],
    frame: &RelationFrame<'_>,
    r: u64,
    d_col: usize,
    request: &TrajectoryRequest,
    full: (u64, u64, u64),
) -> Result<RankTrajectory, String> {
    let columns = d_col as u64 + 1;
    let n = inputs.len();
    let mut cache: BTreeMap<usize, PrefixState> = BTreeMap::new();
    let mut evaluations = 0u64;
    let mut eval = |k: usize| -> Result<PrefixState, String> {
        if let Some(state) = cache.get(&k) {
            return Ok(*state);
        }
        evaluations += 1;
        let state = prefix_state(inputs, k, frame, r, d_col)?;
        cache.insert(k, state);
        Ok(state)
    };
    let step = (n / 64).max(1);
    let mut scan: Vec<usize> = (1..=n / step).map(|i| i * step).collect();
    if scan.last() != Some(&n) && n > 0 {
        scan.push(n);
    }
    let mut monotone = true;
    let mut previous = 0u64;
    let mut coarse = Vec::with_capacity(scan.len());
    for &k in &scan {
        let state = eval(k)?;
        monotone &= state.rank >= previous;
        previous = state.rank;
        coarse.push((k, state));
    }
    let consistent_with_pipeline = if n == 0 {
        full == (0, 0, 0)
    } else {
        let last = eval(n)?;
        (last.complete, last.independent, last.rank) == full
    };

    enum Target {
        Rank(u64),
        Recovery,
    }
    let mut targets: Vec<(String, u64, Target)> = Vec::new();
    for fraction in [0.5f64, 0.75, 0.9, 1.0] {
        let q = ((fraction * columns as f64).ceil() as u64).clamp(1, columns);
        if !targets.iter().any(|(_, existing, _)| *existing == q) {
            targets.push((format!("rank>={q}"), q, Target::Rank(q)));
        }
    }
    targets.push(("recovery".into(), columns, Target::Recovery));
    let mut first_hits = Vec::with_capacity(targets.len());
    for (label, rank, target) in &targets {
        let reached = |s: &PrefixState| match target {
            Target::Rank(q) => s.rank >= *q,
            Target::Recovery => s.pinned,
        };
        let mut hit = None;
        let mut lower = 0usize;
        for &(k, state) in &coarse {
            if reached(&state) {
                // Every prefix strictly between the previous scan point and
                // this one, in order.
                hit = Some(k);
                for j in lower + 1..k {
                    if reached(&eval(j)?) {
                        hit = Some(j);
                        break;
                    }
                }
                break;
            }
            lower = k;
        }
        first_hits.push(TrajectoryHit {
            target: label.clone(),
            rank: *rank,
            partials: hit.map(|k| k as u64),
            trial: hit.map(|k| inputs[k - 1].id),
        });
    }

    let mut at_cutoffs = Vec::with_capacity(request.cutoffs.len());
    for &cutoff in &request.cutoffs {
        let k = inputs.iter().take_while(|row| row.id <= cutoff).count();
        let state = if k == 0 {
            PrefixState {
                complete: 0,
                independent: 0,
                rank: 0,
                pinned: false,
            }
        } else {
            eval(k)?
        };
        at_cutoffs.push(TrajectoryPoint {
            cutoff_trial: cutoff,
            partials: k as u64,
            complete: state.complete,
            independent: state.independent,
            rank: state.rank,
            pinned: state.pinned,
        });
    }
    Ok(RankTrajectory {
        columns,
        stored_partials: n as u64,
        evaluations,
        monotone,
        consistent_with_pipeline,
        first_hits,
        at_cutoffs,
    })
}

fn solve_with_verification(
    instance: &BinaryInstance,
    target: FastPoint,
    config: SolveConfig,
    verify_candidate: bool,
    options: SolveOptions,
) -> Result<LargePrimeReport, String> {
    if config.max_large_primes > 2 {
        return Err("max_large_primes must be 0, 1, or 2".into());
    }
    let charge_checks = options.charge_checks;
    let group = BinaryGroup(&instance.fast);
    let mut verify_ops = GroupOps::default();
    let mut membership_checks = 0u64;
    let in_subgroup = if charge_checks {
        membership_checks = 1;
        instance.fast.is_on_curve(target) && group.mul(&mut verify_ops, target, instance.r).infinity
    } else {
        instance.fast.is_on_curve(target) && instance.fast.mul_u64(target, instance.r).infinity
    };
    if !in_subgroup {
        return Err("target is not a point in the configured prime-order subgroup".into());
    }
    let start = Instant::now();
    let cpu_start = process_cpu_ns();
    let graph = config.large_prime_graph;
    if options.trajectory.is_some() && !graph.uses_filter() {
        return Err(
            "a rank trajectory needs a filter arm (the online path stops at recovery)".into(),
        );
    }
    if options.table_fault.is_some() && !graph.orbit() {
        return Err("a table fault needs an orbit graph".into());
    }
    let orbit_plan = if graph.orbit() {
        Some(OrbitPlan {
            powers: LambdaPowers::new(instance)?,
            absorb: graph.absorb(),
        })
    } else {
        None
    };
    let mut phase_group_ops = BTreeMap::new();

    let mut fb_ops = GroupOps::default();
    let base = build_factor_base(
        instance,
        config.small_dimension,
        config.envelope_dimension,
        orbit_plan.as_ref(),
        options.table_fault,
        options.abscissa_basis.as_deref(),
        &mut fb_ops,
    )?;
    if base.small_columns == 0 {
        return Err("projected small factor base has no columns".into());
    }
    phase_group_ops.insert("factor_base".into(), fb_ops);

    // The post-collection arm decides acceptance on the plain column map and
    // only stores the relations in orbit columns.  Both bases come from the
    // same enumeration of raw points, so atom `i` is the same point in both
    // (checked), and a decomposition (a list of atom indices) means the same
    // thing in either.
    let plain_base = if graph.post_collect() {
        let mut plain_ops = GroupOps::default();
        let plain = build_factor_base(
            instance,
            config.small_dimension,
            config.envelope_dimension,
            None,
            None,
            options.abscissa_basis.as_deref(),
            &mut plain_ops,
        )?;
        let aligned = plain.atoms.len() == base.atoms.len()
            && plain.small_columns == base.small_columns
            && plain
                .atoms
                .iter()
                .zip(&base.atoms)
                .all(|(p, o)| p.raw == o.raw && p.small == o.small);
        if !aligned {
            return Err("plain and orbit atom tables disagree".into());
        }
        phase_group_ops.insert("factor_base_plain".into(), plain_ops);
        Some(plain)
    } else {
        None
    };
    let accept_base = plain_base.as_ref().unwrap_or(&base);

    let mut setup_ops = GroupOps::default();
    let decomposer = Decomposer::new(
        &group,
        accept_base,
        config.summands,
        config.max_states,
        &mut setup_ops,
    )?;
    phase_group_ops.insert("oracle_setup".into(), setup_ops);
    let cpu_after_setup = process_cpu_ns();

    let d_col = base.small_columns;
    let mut matrix = IncrementalGauss::new(base.small_columns + 1, instance.r);
    let mut eliminator = LargePrimeEliminator::new(base.small_columns + 1, instance.r);
    let mut rng = SplitMix64(config.seed ^ 0x4a6f_7578_4c50_7631);
    let mut rel_ops = GroupOps::default();
    let mut counters = SolveCounters {
        combination_states_uncharged: decomposer.report.left_states
            + decomposer.report.right_states,
        membership_checks,
        ..Default::default()
    };
    let mut samples = Vec::new();
    let mut stored: Vec<StoredPartial> = Vec::new();
    let mut recovered = None;
    let mut stream_hasher = options.stream_digest.then(blake3::Hasher::new);
    let mut post_collect_report = graph.post_collect().then_some(PostCollectReport {
        partials_by_orbit_class: [0; 3],
        class_changed: 0,
    });
    measurement::begin_online(measurement::Phase::TargetQuery);
    for trial in 1..=config.max_trials {
        counters.trials = trial;
        let a = rng.next() % instance.r;
        let b = 1 + rng.next() % (instance.r - 1);
        let ap = group.mul(&mut rel_ops, instance.generator, a);
        let bq = group.mul(&mut rel_ops, target, b);
        let residual = group.add(&mut rel_ops, ap, bq);
        let decomposition = {
            let _pdp = measurement::scope(measurement::Phase::TargetPdp);
            decomposer.find(
                &group,
                accept_base,
                residual,
                config.max_large_primes,
                instance.r,
                &mut rel_ops,
                &mut counters.mitm_lookups_uncharged,
            )
        };
        let Some((terms, projected, lp_count)) = decomposition else {
            continue;
        };
        counters.decompositions += 1;
        {
            let _check = measurement::scope(measurement::Phase::TargetRelationCheck);
            let mut check = FastPoint::INFINITY;
            for &i in &terms {
                check = group.add(&mut verify_ops, check, base.atoms[i].raw);
            }
            if check != residual {
                return Err("MITM returned a non-relation".into());
            }
        }
        counters.relation_histogram[lp_count.min(3)] += 1;
        if lp_count > config.max_large_primes as usize {
            counters.rejected_large_prime_count += 1;
            continue;
        }
        counters.accepted_relations += 1;
        let class = match lp_count {
            0 => RelationClass::Full,
            1 => RelationClass::SingleLargePrime,
            _ => RelationClass::DoubleLargePrime,
        };
        if let Some(hasher) = stream_hasher.as_mut() {
            hasher.update(&trial.to_le_bytes());
            hasher.update(&a.to_le_bytes());
            hasher.update(&b.to_le_bytes());
            hasher.update(&(terms.len() as u64).to_le_bytes());
            for &term in &terms {
                hasher.update(&(term as u64).to_le_bytes());
            }
        }
        // Acceptance used the plain columns above; the post-collection arm
        // now re-expresses the same decomposition in orbit columns.
        let projected = match post_collect_report.as_mut() {
            Some(report) => {
                let orbit_columns = project_terms(&base, &terms, instance.r);
                let orbit_lp = orbit_columns
                    .iter()
                    .filter(|(c, _)| *c >= base.small_columns)
                    .count();
                report.partials_by_orbit_class[orbit_lp.min(2)] += 1;
                report.class_changed += u64::from(orbit_lp != lp_count);
                orbit_columns
            }
            None => projected,
        };
        let mut cols = Vec::with_capacity(projected.len() + 1);
        for &(c, v) in &projected {
            cols.push((if c < base.small_columns { c } else { c + 1 }, v));
        }
        let hb = mod_mul(instance.cofactor % instance.r, b, instance.r);
        if hb != 0 {
            cols.push((d_col, instance.r - hb));
        }
        cols.sort_unstable();
        let rhs = mod_mul(instance.cofactor % instance.r, a, instance.r);
        if samples.len() < 12 || options.abscissa_basis.is_some() {
            samples.push(PartialRelation {
                trial,
                a,
                b,
                class,
                raw_terms: terms,
                columns: cols.clone(),
            });
        }
        if graph.uses_filter() {
            stored.push(StoredPartial {
                trial,
                class,
                cols,
                rhs,
            });
            continue;
        }
        let _descent = measurement::scope(measurement::Phase::TargetDescent);
        let sparse = SparseRel { cols, rhs };
        let full = if lp_count == 0 {
            Some(sparse)
        } else {
            let before = counters.lp_merge_ops_uncharged;
            let out = eliminator.feed(sparse, &mut counters.lp_merge_ops_uncharged);
            if out.is_some() {
                counters.eliminated_full_relations += 1;
            }
            debug_assert!(counters.lp_merge_ops_uncharged >= before);
            out
        };
        if let Some(full) = full {
            let mut row = vec![0u64; base.small_columns + 1];
            for (c, v) in full.cols {
                if c >= row.len() {
                    return Err("large-prime elimination left a large column".into());
                }
                row[c] = v;
            }
            match matrix.add_row(row, full.rhs) {
                RowStatus::Independent => counters.independent_rows += 1,
                RowStatus::Dependent => {}
                RowStatus::Inconsistent => return Err("relation system became inconsistent".into()),
            }
            if let Some(d) = matrix.pinned(d_col) {
                recovered = Some(d);
                break;
            }
        }
    }
    measurement::end_online();
    let filter_pipeline = if graph.uses_filter() {
        let cpu = PhaseCpuNs {
            setup: cpu_after_setup.saturating_sub(cpu_start),
            collect: process_cpu_ns().saturating_sub(cpu_after_setup),
            ..PhaseCpuNs::default()
        };
        let report = run_filter_pipeline(
            instance,
            target,
            &base,
            graph,
            std::mem::take(&mut stored),
            &mut matrix,
            cpu,
            PipelineExtras {
                stream: stream_hasher.take().map(|hasher| StreamReport {
                    digest: hex::encode(hasher.finalize().as_bytes()),
                    accepted: counters.accepted_relations,
                }),
                post_collect: post_collect_report.take(),
                trajectory: options.trajectory.as_ref(),
            },
        )?;
        recovered = matrix.pinned(d_col);
        counters.eliminated_full_relations = report.filter.cycles_closed as u64;
        counters.independent_rows = report.independent_verified;
        Some(report)
    } else {
        None
    };
    counters.matrix_rank = matrix.rank() as u64;
    counters.row_ops_uncharged = matrix.row_ops;
    counters.lp_pivots = match &filter_pipeline {
        Some(report) => report.filter.pivots as u64,
        None => eliminator.pivot_count() as u64,
    };
    if let Some(report) = &filter_pipeline {
        counters.lp_merge_ops_uncharged = report.filter.merges as u64;
    }
    if charge_checks {
        if let Some(d) = recovered {
            counters.final_checks = 1;
            if group.mul(&mut verify_ops, instance.generator, d) != target {
                counters.final_checks_failed = 1;
            }
        }
    }
    phase_group_ops.insert("relations".into(), rel_ops);
    phase_group_ops.insert("verification".into(), verify_ops);
    let verified_fast = verify_candidate
        && recovered.is_some_and(|d| instance.fast.mul_u64(instance.generator, d) == target);
    let verified_independent = verify_candidate
        && recovered.is_some_and(|d| independent_verify(instance, target, d).is_ok());
    Ok(LargePrimeReport {
        trace_evidence: options.abscissa_basis.map(|basis| trace_line::Evidence {
            basis,
            atoms: base.atoms.iter().map(|a| (a.raw.x, a.raw.y)).collect(),
            columns: base.column_points.iter().map(|p| (p.x, p.y)).collect(),
            small_columns: base.small_columns,
        }),
        recovered,
        exhausted: recovered.is_none(),
        verified_fast,
        verified_independent,
        verification_performed: verify_candidate,
        config,
        factor_base: base.report,
        mitm: decomposer.report,
        counters,
        phase_group_ops,
        solve_wall_ns: start.elapsed().as_nanos() as u64,
        relation_samples: samples,
        filter_pipeline,
    })
}

#[cfg(test)]
mod tests {
    use super::parse_manifest;

    const SMALL_MANIFEST: &[u8] =
        include_bytes!("../../tests/fixtures/ecbench_manifest_small.json");

    #[test]
    fn manifest_rejects_duplicate_family_ids() {
        let mut value: serde_json::Value = serde_json::from_slice(SMALL_MANIFEST).unwrap();
        let families = value["families"].as_array_mut().unwrap();
        let duplicate = families[0].clone();
        families.push(duplicate);
        let encoded = serde_json::to_vec(&value).unwrap();
        let error = parse_manifest(&encoded).unwrap_err();
        assert!(error.contains("duplicate family id `koblitz-n11-l4`"));
    }
}
