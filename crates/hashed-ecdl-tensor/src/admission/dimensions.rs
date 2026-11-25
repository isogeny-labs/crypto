//! Exact dimension envelopes for the Appendix D implementation audit.
//!
//! These calculations do not admit a contraction, allocate a graph, or certify
//! the external theorem. They expose dimensional inputs to its resource ledger.
//! In particular, record counts are not bit-operation or allocator-byte counts.

use crate::{invalid, BigUint, Result};
use num_traits::{One, Zero};

/// Structural dimensions of one active interpolation evaluation.
#[derive(Clone, Debug)]
pub struct NodeDimensions {
    /// Current edge count M.
    pub edges: BigUint,
    /// Number K of packets, subject to K*2^a <= 3M.
    pub packets: BigUint,
    /// Packet-length exponent a, so L=2^a.
    pub a: u32,
    /// Positive number u of retained valuation levels; u <= a and u <= M.
    pub u: u32,
    /// Active valuation i, with i < u.
    pub coordinate: u32,
    /// Positive working precision p_t used to construct modes.
    pub precision: BigUint,
}

/// Logical counts, conditional on the checked structural inputs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeDimensionAudit {
    /// Packet capacity L.
    pub packet_length: BigUint,
    /// Tile period 2^(i+1).
    pub tiles: BigUint,
    /// Interaction distance D=L/2^i in the source mode and precision bounds.
    pub interaction_distance: BigUint,
    /// K*2^(i+1), bounding modes of each weight before zero filtering.
    pub modes_per_weight_bound: BigUint,
    /// K*2^(i+1)*(p_t-1), bounding all weight-loop iterations, including discards.
    pub attempted_modes_bound: BigUint,
    /// Upper bound on column modular-power calls across all weight candidates.
    pub column_modpow_calls_bound: BigUint,
    /// Upper bound on row coefficient calls across all weight candidates.
    pub row_coefficient_calls_bound: BigUint,
    /// A rigorous base-two exponent E such that the number of admissible mode
    /// tuples in one interpolation evaluation is at most 2^E.
    ///
    /// This uses the weighted generating function with at most
    /// `modes_per_weight_bound` variables of every positive weight below the
    /// precision.  The explicit elementary constant is intentionally loose.
    pub mode_tuple_log2_bound: BigUint,
    /// Base-two exponent bounding all radix-64 group-child invocations across
    /// every active Hermite node and interpolation sample of this call.
    pub active_call_child_invocations_log2_bound: BigUint,
    /// Scalar/term slots sufficient for all new packet descriptions in one
    /// group child, before allocator and integer-width pricing.
    pub packet_description_slots_bound: BigUint,
    /// Base-two exponent bounding the coefficient-choice leaves visited by one
    /// packet-entry evaluation.  Local column sums and outgoing powers are
    /// polynomial work outside this choice traversal.
    pub packet_entry_choice_log2_bound: BigUint,
    /// Smallest integer c with `precision*u <= c*edges*D`.
    ///
    /// A uniform constant bound on this value is the bridge from the finite
    /// tuple exponent to the source's `O(M/sqrt(u))` exponent.
    pub precision_md_over_u_coefficient: BigUint,
    /// Number I=M*(2L-1)+1 of interpolation samples at each active node.
    pub samples_per_node: BigUint,
    /// Exact number of nonzero grid nodes with valuation below u.
    pub active_nodes: BigUint,
    /// Exact sum of all qt on the complete grid; excludes sample/scratch storage.
    pub retained_jet_coefficients: BigUint,
    /// Lower bound N*I on retained outer Vec slots after interpolation/truncation.
    /// These slots hold Element metadata, not live coefficient payloads. Extra
    /// allocator capacity and all inner allocations require separate accounting.
    pub retained_jet_vector_slots_lower_bound: BigUint,
}

fn ceil_sqrt(value: &BigUint) -> BigUint {
    if value <= &BigUint::one() {
        return value.clone();
    }
    let mut estimate = BigUint::one() << value.bits().div_ceil(2) as usize;
    loop {
        let next = (&estimate + value / &estimate) >> 1usize;
        if next >= estimate {
            break;
        }
        estimate = next;
    }
    if &estimate * &estimate < *value {
        estimate + BigUint::one()
    } else {
        estimate
    }
}

fn ceil_log2(value: &BigUint) -> u64 {
    assert!(!value.is_zero());
    let bits = value.bits();
    if value == &(BigUint::one() << (bits - 1) as usize) {
        bits - 1
    } else {
        bits
    }
}

/// Compute exact dimension envelopes without building modes or Taylor data.
///
/// A successful result checks shape/packing premises only. It neither checks
/// the graph's packing nor asserts that numeric recursion guards are satisfied.
/// The caller must bind these inputs to the actual objects in a refinement.
pub fn audit_node_dimensions(input: &NodeDimensions) -> Result<NodeDimensionAudit> {
    if input.edges.is_zero()
        || input.packets.is_zero()
        || input.precision.is_zero()
        || input.u == 0
        || input.u > input.a
        || input.coordinate >= input.u
        || BigUint::from(input.u) > input.edges
    {
        return Err(invalid("active dimension shape"));
    }
    let length = BigUint::one() << input.a as usize;
    if &input.packets * &length > BigUint::from(3u8) * &input.edges {
        return Err(invalid("dimension packet capacity bound"));
    }
    let tiles = BigUint::one() << (input.coordinate as usize + 1);
    let distance = &length >> input.coordinate as usize;
    let per_weight = &input.packets * &tiles;
    debug_assert!(&per_weight * &distance <= BigUint::from(6u8) * &input.edges);
    let candidates = &per_weight * (&input.precision - BigUint::one());
    // For fixed b, each real column appears in exactly one tile. The coarse
    // L factor below also covers row calls, repeated independently per tile.
    let coefficient_calls = &candidates * &length;
    let samples = &input.edges * (BigUint::from(2u8) * &length - BigUint::one()) + BigUint::one();
    let tail = &length >> input.u as usize;
    let active_nodes = BigUint::from(2u8) * (&length - &tail);
    // If at most n variables have each weight j, the weighted generating
    // bound at x=exp(-theta), together with sum 1/h^2 <= 2, gives
    // log_2(tuple_count) <= 5*sqrt(n*p).  The constant 5 also absorbs ceilings.
    let tuple_log2 = BigUint::from(5u8) * ceil_sqrt(&(&per_weight * &input.precision));
    // The source fixes radix r=64.
    let active_child_log2 = &tuple_log2
        + BigUint::from(ceil_log2(&active_nodes))
        + BigUint::from(ceil_log2(&samples))
        + BigUint::from(6u8);
    // Across one group child, local matrices use at most K*L^2 entries.  Each
    // retained mode contributes at most L incoming coefficients or L outgoing
    // terms, plus metadata.  Counting every attempted mode in both directions
    // is a safe representation-level envelope.
    let description_slots = &input.packets
        + &input.packets * &length * &length
        + BigUint::from(2u8) * &candidates * (&length + BigUint::one());
    // PacketTensor::entry has at most L rows and at most `candidates` incoming
    // choices.  The constant/local choice adds one.
    let entry_choice_log2 = &length * BigUint::from(ceil_log2(&(&candidates + BigUint::one())));
    let md = &input.edges * &distance;
    let precision_coefficient =
        (&input.precision * BigUint::from(input.u) + &md - BigUint::one()) / md;
    let vector_slots = &active_nodes * &samples;
    let h =
        (BigUint::from(3u8) * &input.edges + BigUint::from(input.u - 1)) / BigUint::from(input.u);
    // Sum_i (L/2^i)*h*(u-i) = h*((2u-2)*L + 2L/2^u).
    let jets =
        h * (BigUint::from(2u64 * u64::from(input.u) - 2) * &length + BigUint::from(2u8) * tail);
    Ok(NodeDimensionAudit {
        packet_length: length,
        tiles,
        interaction_distance: distance,
        modes_per_weight_bound: per_weight,
        attempted_modes_bound: candidates,
        column_modpow_calls_bound: coefficient_calls.clone(),
        row_coefficient_calls_bound: coefficient_calls,
        mode_tuple_log2_bound: tuple_log2,
        active_call_child_invocations_log2_bound: active_child_log2,
        packet_description_slots_bound: description_slots,
        packet_entry_choice_log2_bound: entry_choice_log2,
        precision_md_over_u_coefficient: precision_coefficient,
        samples_per_node: samples,
        active_nodes,
        retained_jet_coefficients: jets,
        retained_jet_vector_slots_lower_bound: vector_slots,
    })
}

/// Coarse root-only precision bound, independently of original description size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathPrecisionEnvelope {
    /// A=4*floor(ceil(log2(s))/131072), bounding each source packet exponent.
    pub packet_exponent_bound: u64,
    /// L*=2^A, bounding packet capacities along a source-parameter path.
    pub packet_length_bound: BigUint,
    /// P=p0+16*s*(3*L*+A+1), bounding precision passed down such a path.
    pub precision_bound: BigUint,
}

/// First root log-size for the elementary uniform-history absorption proof.
///
/// This is a threshold in `lambda=ceil(log2(s))`, not in the edge count `s`.
pub const UNIFORM_HISTORY_LAMBDA_THRESHOLD: u64 = 1 << 26;

/// Scale floor obtained at every potentially active depth above the threshold.
pub const UNIFORM_HISTORY_SCALE_FLOOR: u64 = 24;

/// Exact absorption checks at one depth that is not excluded by shrinkage and
/// the fixed root leaf cutoff.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniformHistoryDepthAudit {
    /// Zero-based recursive depth.
    pub depth: u32,
    /// `8^depth`; an active history requires this to be below the leaf divisor.
    pub shrink_factor: BigUint,
    /// Source scale `floor(lambda/(2^17*2^depth))`.
    pub u: u64,
    /// Twice the sum of all earlier source scales.
    pub prior_a: u64,
    /// Current cumulative packet exponent `prior_a+2*u`.
    pub current_a: u64,
    /// Left side of `81*h*u <= 2^(u+1)`.
    pub inherited_absorption_lhs: BigUint,
    /// Exponent on the right side of the inherited-precision check.
    pub inherited_absorption_rhs_exponent: u64,
    /// Whether the inherited-precision check holds at this depth.
    pub inherited_absorption_holds: bool,
    /// Left side of `8*u*(current_a+1) <= 2^(prior_a+u+1)`.
    pub evaluation_absorption_lhs: BigUint,
    /// Exponent on the right side of the evaluation-loss check.
    pub evaluation_absorption_rhs_exponent: u64,
    /// Whether the evaluation-loss check holds at this depth.
    pub evaluation_absorption_holds: bool,
}

/// Finite source-schedule audit covering every depth that an active history
/// can reach under the checked eight-fold child shrinkage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UniformHistoryAudit {
    /// Symbolic root log-size `ceil(log2(s))`.
    pub lambda: u64,
    /// `floor(sqrt(lambda))`, used by the fixed root leaf cutoff `floor(s/h)`.
    pub leaf_divisor: u64,
    /// Root source scale `floor(lambda/2^17)`.
    pub root_u: u64,
    /// Whether the elementary all-lambda proof applies at this root size.
    pub threshold_proof_applies: bool,
    /// Uniform lower bound on `u` used by the threshold proof.
    pub scale_floor: u64,
    /// Coarse finite-history precision constant.
    pub inherited_precision_constant: u64,
    /// Resulting local normalized-precision constant.
    pub normalized_precision_constant: u64,
    /// Exact checks for every depth not excluded by the leaf/shrink argument.
    pub depths: Vec<UniformHistoryDepthAudit>,
    /// Whether both absorption inequalities hold at every listed depth.
    pub all_absorption_checks_hold: bool,
}

fn floor_sqrt_u64(value: u64) -> u64 {
    if value < 2 {
        return value;
    }
    let mut lo = 1u64;
    let mut hi = 1u64 << value.ilog2().div_ceil(2);
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if mid <= value / mid {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    if hi <= value / hi {
        hi
    } else {
        lo
    }
}

fn at_most_power_of_two(value: &BigUint, exponent: u64) -> bool {
    if value.is_zero() {
        return true;
    }
    let bits = value.bits();
    if bits <= exponent {
        return true;
    }
    if bits != exponent.saturating_add(1) {
        return false;
    }
    let Ok(shift) = usize::try_from(exponent) else {
        return false;
    };
    value == &(BigUint::one() << shift)
}

/// Audit the precision-absorption conditions uniformly over possible active
/// depths for one symbolic root log-size.
///
/// Let the root have `s` edges and `lambda=ceil(log2(s))`.  An active node at
/// depth `d` has more than `floor(s/h)` edges for
/// `h=floor(sqrt(lambda))`, while strict child checking gives
/// `8^d*M_d<=s`.  Hence it can exist only when `8^d<h`; this routine audits
/// exactly those depths.
///
/// For every `lambda>=2^26`, elementary integer inequalities give `u_d>=24`.
/// They then imply both conditions checked below for all possible depths:
/// the finite-history constant 81 is absorbed into one local unit, and the
/// evaluation loss is absorbed into another.  Combined with the native
/// taper bound this yields `p_next*u <= 34*M*2^(a-i)`.  This function checks
/// the finite schedule; it does not extract a graph history or price integer
/// and rational arithmetic.
pub fn audit_uniform_history(lambda: u64) -> Result<UniformHistoryAudit> {
    let leaf_divisor = floor_sqrt_u64(lambda);
    let root_u = lambda / super::EPSILON_DENOMINATOR;
    if leaf_divisor <= 1 || root_u == 0 {
        return Err(invalid("uniform history root scale"));
    }

    let mut depths = Vec::new();
    let mut depth = 0u32;
    let mut shrink_factor = BigUint::one();
    let mut prior_a = 0u64;
    while shrink_factor < BigUint::from(leaf_divisor) {
        let u = root_u.checked_shr(depth).unwrap_or(0);
        if u == 0 {
            break;
        }
        let current_a = prior_a
            .checked_add(
                2u64.checked_mul(u)
                    .ok_or_else(|| invalid("history scale width"))?,
            )
            .ok_or_else(|| invalid("history scale width"))?;
        let inherited_lhs = BigUint::from(81u8) * BigUint::from(leaf_divisor) * BigUint::from(u);
        let inherited_rhs_exponent = u
            .checked_add(1)
            .ok_or_else(|| invalid("history exponent width"))?;
        let evaluation_lhs = BigUint::from(8u8)
            * BigUint::from(u)
            * BigUint::from(
                current_a
                    .checked_add(1)
                    .ok_or_else(|| invalid("history exponent width"))?,
            );
        let evaluation_rhs_exponent = prior_a
            .checked_add(u)
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| invalid("history exponent width"))?;
        let inherited_holds = at_most_power_of_two(&inherited_lhs, inherited_rhs_exponent);
        let evaluation_holds = at_most_power_of_two(&evaluation_lhs, evaluation_rhs_exponent);
        depths.push(UniformHistoryDepthAudit {
            depth,
            shrink_factor: shrink_factor.clone(),
            u,
            prior_a,
            current_a,
            inherited_absorption_lhs: inherited_lhs,
            inherited_absorption_rhs_exponent: inherited_rhs_exponent,
            inherited_absorption_holds: inherited_holds,
            evaluation_absorption_lhs: evaluation_lhs,
            evaluation_absorption_rhs_exponent: evaluation_rhs_exponent,
            evaluation_absorption_holds: evaluation_holds,
        });
        prior_a = current_a;
        depth = depth
            .checked_add(1)
            .ok_or_else(|| invalid("history depth width"))?;
        shrink_factor *= BigUint::from(8u8);
    }
    let all_absorption_checks_hold = depths
        .iter()
        .all(|case| case.inherited_absorption_holds && case.evaluation_absorption_holds);
    Ok(UniformHistoryAudit {
        lambda,
        leaf_divisor,
        root_u,
        threshold_proof_applies: lambda >= UNIFORM_HISTORY_LAMBDA_THRESHOLD,
        scale_floor: UNIFORM_HISTORY_SCALE_FLOOR,
        inherited_precision_constant: 81,
        normalized_precision_constant: 34,
        depths,
        all_absorption_checks_hold,
    })
}

/// Price the precision increments on a strictly shrinking source-parameter path.
///
/// Premises for use: root M <= s, each next M <= M/8, source (D.4)
/// parameters, and active 1 <= u <= M. Then sum M <= 2s and each increment
/// is at most 8M*(3L*+A+1). No asymptotic admission or implementation
/// refinement follows from evaluating this expression. To obtain a polynomial
/// in s alone, initial precision must itself have a fixed polynomial bound.
pub fn path_precision_envelope(
    root_edges: &BigUint,
    initial_precision: &BigUint,
) -> Result<PathPrecisionEnvelope> {
    if root_edges < &BigUint::from(2u8) || initial_precision.is_zero() {
        return Err(invalid("root dimension shape"));
    }
    let lambda = (root_edges - BigUint::one()).bits();
    let a = 4 * (lambda / super::EPSILON_DENOMINATOR);
    let shift = usize::try_from(a).map_err(|_| invalid("dimension exponent width"))?;
    let length = BigUint::one() << shift;
    let precision = initial_precision
        + BigUint::from(16u8)
            * root_edges
            * (BigUint::from(3u8) * &length + BigUint::from(a) + BigUint::one());
    Ok(PathPrecisionEnvelope {
        packet_exponent_bound: a,
        packet_length_bound: length,
        precision_bound: precision,
    })
}
