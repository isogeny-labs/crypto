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
    /// K*2^(i+1), bounding modes of each weight before zero filtering.
    pub modes_per_weight_bound: BigUint,
    /// K*2^(i+1)*(p_t-1), bounding all weight-loop iterations, including discards.
    pub attempted_modes_bound: BigUint,
    /// Upper bound on column modular-power calls across all weight candidates.
    pub column_modpow_calls_bound: BigUint,
    /// Upper bound on row coefficient calls across all weight candidates.
    pub row_coefficient_calls_bound: BigUint,
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
    let per_weight = &input.packets * &tiles;
    let candidates = &per_weight * (&input.precision - BigUint::one());
    // For fixed b, each real column appears in exactly one tile. The coarse
    // L factor below also covers row calls, repeated independently per tile.
    let coefficient_calls = &candidates * &length;
    let samples = &input.edges * (BigUint::from(2u8) * &length - BigUint::one()) + BigUint::one();
    let tail = &length >> input.u as usize;
    let active_nodes = BigUint::from(2u8) * (&length - &tail);
    let vector_slots = &active_nodes * &samples;
    let h =
        (BigUint::from(3u8) * &input.edges + BigUint::from(input.u - 1)) / BigUint::from(input.u);
    // Sum_i (L/2^i)*h*(u-i) = h*((2u-2)*L + 2L/2^u).
    let jets =
        h * (BigUint::from(2u64 * u64::from(input.u) - 2) * &length + BigUint::from(2u8) * tail);
    Ok(NodeDimensionAudit {
        packet_length: length,
        tiles,
        modes_per_weight_bound: per_weight,
        attempted_modes_bound: candidates,
        column_modpow_calls_bound: coefficient_calls.clone(),
        row_coefficient_calls_bound: coefficient_calls,
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
