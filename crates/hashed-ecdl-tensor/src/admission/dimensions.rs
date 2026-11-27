//! Exact dimension envelopes for the Appendix D implementation audit.
//!
//! These calculations do not admit a contraction, allocate a graph, or certify
//! the external theorem. They expose dimensional inputs to its resource ledger.
//! In particular, record counts are not bit-operation or allocator-byte counts.

use crate::{integrity, invalid, BigUint, Result};
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

/// Coarse bit-width and arithmetic-operation envelopes for one active call.
///
/// Counts are representation-level upper bounds for the current direct
/// algorithms. They are not calibrated machine-operation counts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArithmeticWidthAudit {
    /// Fixed number of coordinates in every extension-ring element.
    pub extension_degree: BigUint,
    /// Canonical bits in every stored extension coefficient.
    pub coefficient_bits_bound: BigUint,
    /// Bits sufficient for an unreduced coefficient product plus accumulation.
    pub ring_multiplication_temporary_bits_bound: BigUint,
    /// Bits sufficient for every integer binomial coefficient used in recovery.
    pub binomial_integer_bits_bound: BigUint,
    /// Uniform upper bound `4M` on one tapered multiplicity.
    pub maximum_multiplicity_bound: BigUint,
    /// Bits sufficient for one `e_zero*(-t/alpha)^k` rational multiplier.
    pub rational_multiplier_bits_bound: BigUint,
    /// Bits sufficient for numerator and denominator of a naively accumulated
    /// Hermite output coordinate.
    pub rational_accumulator_bits_bound: BigUint,
    /// Ring operations in all direct Lagrange interpolations of this call.
    pub interpolation_ring_operations_bound: BigUint,
    /// Ring operations in tapered series construction and recovery.
    pub recovery_ring_operations_bound: BigUint,
    /// Big-rational multiply/add operations in tapered recovery.
    pub recovery_rational_operations_bound: BigUint,
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

/// Price the current direct interpolation and Hermite-recovery representation.
///
/// The rational-width bound deliberately permits a naive product of all term
/// denominators. Reduction by `num_rational` can only decrease the represented
/// numerator and denominator. Ring coefficients remain canonical modulo
/// `2^precision`; only a multiply/add temporary reaches `2*precision+1` bits.
pub fn audit_arithmetic_widths(
    input: &NodeDimensions,
    extension_degree: &BigUint,
) -> Result<ArithmeticWidthAudit> {
    if extension_degree.is_zero() {
        return Err(invalid("zero extension degree"));
    }
    let dimensions = audit_node_dimensions(input)?;
    let one = BigUint::one();
    let two = BigUint::from(2u8);
    let four = BigUint::from(4u8);
    let grid_points = &two * &dimensions.packet_length;
    let maximum_multiplicity = &four * &input.edges;
    let precision_log = BigUint::from(ceil_log2(&(&input.precision + &one)));

    // Every base numerator and denominator in e_zero is smaller than 2L, so
    // its logarithmic size is at most a+1.  The total exponent over the grid
    // is bounded by the retained jet mass J.  The subsequent ratio power has
    // exponent below one multiplicity.
    let multiplier_bits = (&dimensions.retained_jet_coefficients + &maximum_multiplicity)
        * BigUint::from(u64::from(input.a) + 1)
        + &one;
    let rational_accumulator_bits = &dimensions.retained_jet_coefficients * &multiplier_bits
        + &input.precision
        + BigUint::from(ceil_log2(&(&dimensions.retained_jet_coefficients + &one)))
        + &one;

    let samples = &dimensions.samples_per_node;
    // The direct Lagrange routine builds each basis polynomial by multiplying
    // by n-1 linear factors.  Four n^3 plus the Newton-inverse allowance is a
    // simple envelope for its ring calls.
    let interpolation_per_node = &four * samples.pow(3)
        + samples * (BigUint::from(3u8) * &precision_log + BigUint::from(8u8));
    let interpolation_ring_operations = &dimensions.active_nodes * interpolation_per_node;

    // For each jet and grid point, cover the binomial series, one dense
    // truncated product, and a Newton inverse.  The remaining reciprocal and
    // quotient products contribute four further q_max^2 terms.
    let q_squared = maximum_multiplicity.pow(2);
    let recovery_per_jet = &four * &grid_points * &q_squared
        + &four * &q_squared
        + &four * &grid_points * &maximum_multiplicity
        + &grid_points * (BigUint::from(3u8) * &precision_log + &one);
    let recovery_ring_operations = &dimensions.active_nodes * recovery_per_jet;

    let rational_power_operations = &dimensions.active_nodes
        * &grid_points
        * (BigUint::from(2u8) * BigUint::from(ceil_log2(&(&maximum_multiplicity + &one))) + &one);
    let rational_reconstruction_operations =
        &dimensions.retained_jet_coefficients * (&two * extension_degree + &one);

    Ok(ArithmeticWidthAudit {
        extension_degree: extension_degree.clone(),
        coefficient_bits_bound: input.precision.clone(),
        ring_multiplication_temporary_bits_bound: &two * &input.precision + &one,
        binomial_integer_bits_bound: &maximum_multiplicity + &one,
        maximum_multiplicity_bound: maximum_multiplicity,
        rational_multiplier_bits_bound: multiplier_bits,
        rational_accumulator_bits_bound: rational_accumulator_bits,
        interpolation_ring_operations_bound: interpolation_ring_operations,
        recovery_ring_operations_bound: recovery_ring_operations,
        recovery_rational_operations_bound: rational_power_operations
            + rational_reconstruction_operations,
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
pub const UNIFORM_HISTORY_SCALE_FLOOR: u64 = 32;

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

/// One exact scale comparison used to compose local branching exponents.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecursionCompositionDepthAudit {
    /// Zero-based recursive depth.
    pub depth: u32,
    /// Source scale at this depth.
    pub u: u64,
    /// Whether `u_0 <= 4^(d+1)*u_d`.
    pub geometric_scale_comparison_holds: bool,
}

/// Depth-independent exponent constants for the complete recursion tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecursionCompositionAudit {
    /// Symbolic root log-size.
    pub lambda: u64,
    /// Root source scale.
    pub root_u: u64,
    /// Per-active-call child-invocation exponent constant.
    pub local_child_exponent_constant: u64,
    /// Constant in `sum_d M_d/sqrt(u_d) <= C*s/sqrt(u_0)`.
    pub path_sum_constant: u64,
    /// Constant for the product of active-call branching factors.
    pub active_tree_exponent_constant: u64,
    /// Constant for exhaustive assignments at each leaf.
    pub leaf_exponent_constant: u64,
    /// Whether the root leaf divisor is at least `sqrt(u_0)`.
    pub leaf_scale_absorbed: bool,
    /// Constant absorbing packet-entry coefficient traversal.
    pub packet_entry_exponent_constant: u64,
    /// Combined constant in the exponent relative to `s/sqrt(u_0)`.
    pub total_root_u_exponent_constant: u64,
    /// Combined constant relative to `s/sqrt(lambda)`.
    pub total_root_log_exponent_constant: u64,
    /// Polynomial factor in the root packet-entry exponent bound.
    pub packet_entry_polynomial_factor: BigUint,
    /// Whether the packet-entry exponent is at most `s/sqrt(u_0)`.
    pub packet_entry_absorbed: bool,
    /// Exact scale checks for every potentially active depth.
    pub depths: Vec<RecursionCompositionDepthAudit>,
    /// Whether all finite composition checks hold.
    pub all_composition_checks_hold: bool,
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
/// For every `lambda>=2^26`, elementary integer inequalities give `u_d>=32`.
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

/// Compose the active-call exponent over the entire strict-shrink recursion.
///
/// The local constant 96 covers the weighted-tuple exponent after the
/// normalized-precision bound, factorization-certificate description
/// overhead, active nodes, interpolation samples, and the fixed radix. Since
/// `M_d<=s/8^d` and
/// `u_0<=4^(d+1)*u_d`, the geometric path sum is below
/// `3*s/sqrt(u_0)`.  Direct leaves and packet-entry choice traversal each
/// add at most one further unit.  All remaining arithmetic widths and direct
/// operation counts from [`audit_arithmetic_widths`] are polynomial factors.
pub fn audit_recursion_composition(lambda: u64) -> Result<RecursionCompositionAudit> {
    let history = audit_uniform_history(lambda)?;
    if !history.threshold_proof_applies || !history.all_absorption_checks_hold {
        return Err(invalid("uniform history absorption is not established"));
    }
    let depths: Vec<_> = history
        .depths
        .iter()
        .map(|case| {
            let scale = BigUint::from(4u8).pow(case.depth + 1) * BigUint::from(case.u);
            RecursionCompositionDepthAudit {
                depth: case.depth,
                u: case.u,
                geometric_scale_comparison_holds: BigUint::from(history.root_u) <= scale,
            }
        })
        .collect();

    // With K*L<=3M, p*u<=34*M*D, u<=u_0 and a<=4u_0:
    // candidates < 102*s^2*2^(5u_0), so one packet entry has exponent at most
    // 2^(4u_0)*(8+2lambda+5u_0).  Multiplication by u_0 below is stronger than
    // the sqrt(u_0) needed to absorb this into s/sqrt(u_0), using
    // s>2^(lambda-1).
    let packet_entry_factor = BigUint::from(8u8)
        + BigUint::from(2u8) * BigUint::from(lambda)
        + BigUint::from(5u8) * BigUint::from(history.root_u);
    let entry_power = history
        .root_u
        .checked_mul(4)
        .ok_or_else(|| invalid("entry exponent width"))?;
    let entry_rhs_exponent = lambda
        .checked_sub(1)
        .and_then(|value| value.checked_sub(entry_power))
        .ok_or_else(|| invalid("entry absorption width"))?;
    let packet_entry_absorbed = at_most_power_of_two(
        &(&packet_entry_factor * BigUint::from(history.root_u)),
        entry_rhs_exponent,
    );

    let local_child_exponent_constant = 96u64;
    let path_sum_constant = 3u64;
    let active_tree_exponent_constant = local_child_exponent_constant * path_sum_constant;
    let leaf_exponent_constant = 1u64;
    let leaf_scale_absorbed =
        BigUint::from(history.leaf_divisor).pow(2) >= BigUint::from(history.root_u);
    let packet_entry_exponent_constant = 1u64;
    let total_root_u_exponent_constant =
        active_tree_exponent_constant + leaf_exponent_constant + packet_entry_exponent_constant;
    // For lambda>=2^26, floor(lambda/2^17)>=lambda/2^18, whose square-root
    // conversion factor is exactly 2^9=512.
    let total_root_log_exponent_constant = total_root_u_exponent_constant * 512;
    let all_composition_checks_hold = packet_entry_absorbed
        && leaf_scale_absorbed
        && depths
            .iter()
            .all(|case| case.geometric_scale_comparison_holds);
    Ok(RecursionCompositionAudit {
        lambda,
        root_u: history.root_u,
        local_child_exponent_constant,
        path_sum_constant,
        active_tree_exponent_constant,
        leaf_exponent_constant,
        leaf_scale_absorbed,
        packet_entry_exponent_constant,
        total_root_u_exponent_constant,
        total_root_log_exponent_constant,
        packet_entry_polynomial_factor: packet_entry_factor,
        packet_entry_absorbed,
        depths,
        all_composition_checks_hold,
    })
}

/// Serialize the producer-side strict-history and composition audits into the
/// plain scalar format consumed by [`crate::symbolic_history_certificate`].
///
/// The resulting certificate contains no graph or callback. Its standalone
/// verifier independently reconstructs the fixed source schedule and exact
/// inequalities; graph realization and the external contraction theorem
/// remain separate premises.
pub fn symbolic_history_certificate(
    lambda: u64,
) -> Result<crate::symbolic_history_certificate::Certificate> {
    use crate::symbolic_history_certificate::{Certificate, DepthCertificate, SCHEMA};

    let history = audit_uniform_history(lambda)?;
    let composition = audit_recursion_composition(lambda)?;
    if history.depths.len() != composition.depths.len() {
        return Err(integrity("symbolic history depth alignment"));
    }
    let depths = history
        .depths
        .iter()
        .zip(&composition.depths)
        .map(|(history_case, composition_case)| {
            if history_case.depth != composition_case.depth {
                return Err(integrity("symbolic history depth alignment"));
            }
            let packet_shift = usize::try_from(history_case.current_a)
                .map_err(|_| invalid("symbolic packet length width"))?;
            let packet_length = BigUint::one() << packet_shift;
            let normalized_edges = BigUint::from(history_case.u) * &packet_length;
            let normalized_packets = BigUint::from(3u8) * BigUint::from(history_case.u);
            let first_distance_exponent = history_case.current_a;
            let last_distance_exponent = history_case
                .current_a
                .checked_sub(history_case.u)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| invalid("symbolic tile-distance range"))?;
            let first_distance = BigUint::one()
                << usize::try_from(first_distance_exponent)
                    .map_err(|_| invalid("symbolic distance width"))?;
            let last_distance = BigUint::one()
                << usize::try_from(last_distance_exponent)
                    .map_err(|_| invalid("symbolic distance width"))?;
            let description_polynomial = history
                .lambda
                .checked_add(history_case.current_a)
                .and_then(|value| value.checked_add(history_case.u))
                .and_then(|value| value.checked_add(1))
                .and_then(|value| value.checked_mul(16))
                .ok_or_else(|| invalid("symbolic description exponent width"))?;
            let description_overhead_exponent =
                BigUint::from(2u8) * &packet_length + BigUint::from(description_polynomial);
            let floor_root = floor_sqrt_u64(history_case.u);
            let ceiling_root = floor_root + u64::from(floor_root * floor_root != history_case.u);
            let description_absorption_lhs = &description_overhead_exponent
                * BigUint::from(history.leaf_divisor)
                * BigUint::from(ceiling_root);
            let description_absorption_rhs_exponent = history
                .lambda
                .checked_sub(1)
                .ok_or_else(|| invalid("symbolic description absorption width"))?;
            let description_absorbed = at_most_power_of_two(
                &description_absorption_lhs,
                description_absorption_rhs_exponent,
            );
            Ok(DepthCertificate {
                depth: history_case.depth,
                shrink_factor: history_case.shrink_factor.clone(),
                u: history_case.u,
                prior_a: history_case.prior_a,
                current_a: history_case.current_a,
                first_tile_exponent: 1,
                last_tile_exponent: history_case.u,
                first_distance_exponent,
                last_distance_exponent,
                normalized_edges,
                normalized_packets,
                normalized_first_precision: BigUint::from(34u8) * &packet_length * first_distance,
                normalized_last_precision: BigUint::from(34u8) * &packet_length * last_distance,
                description_overhead_exponent,
                description_absorption_lhs,
                description_absorption_rhs_exponent,
                description_absorbed,
                inherited_lhs: history_case.inherited_absorption_lhs.clone(),
                inherited_rhs_exponent: history_case.inherited_absorption_rhs_exponent,
                evaluation_lhs: history_case.evaluation_absorption_lhs.clone(),
                evaluation_rhs_exponent: history_case.evaluation_absorption_rhs_exponent,
                geometric_scale_comparison_holds: composition_case.geometric_scale_comparison_holds,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Certificate {
        schema: SCHEMA,
        lambda: history.lambda,
        leaf_divisor: history.leaf_divisor,
        root_u: history.root_u,
        scale_floor: history.scale_floor,
        inherited_precision_constant: history.inherited_precision_constant,
        normalized_precision_constant: history.normalized_precision_constant,
        packet_capacity_constant: 3,
        mode_distance_constant: 6,
        mode_precision_product_constant: 204,
        tuple_square_root_ceiling: 15,
        tuple_exponent_constant: 80,
        description_overhead_constant: 1,
        packing_radix: 64,
        group_port_balance_constant: 3,
        endpoint_sides: 2,
        strict_edge_shrink_factor: 8,
        child_edge_bound_numerator: 6,
        strict_edge_bound_numerator: 48,
        packet_degree_factor: 2,
        provenance_header_multiplier: 11,
        provenance_entry_multiplier: 3,
        provenance_vertex_multiplier: 7,
        provenance_combined_multiplier: 14,
        provenance_description_composed: true,
        depths,
        local_child_exponent_constant: composition.local_child_exponent_constant,
        path_sum_constant: composition.path_sum_constant,
        active_tree_exponent_constant: composition.active_tree_exponent_constant,
        leaf_exponent_constant: composition.leaf_exponent_constant,
        leaf_scale_absorbed: composition.leaf_scale_absorbed,
        packet_entry_exponent_constant: composition.packet_entry_exponent_constant,
        packet_entry_polynomial_factor: composition.packet_entry_polynomial_factor,
        packet_entry_absorbed: composition.packet_entry_absorbed,
        total_root_u_exponent_constant: composition.total_root_u_exponent_constant,
        total_root_log_exponent_constant: composition.total_root_log_exponent_constant,
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
