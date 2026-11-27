//! Standalone verification of finite recursive-factorization certificates.
//!
//! This module consumes plain serialized data and the crate's exact ring
//! arithmetic. It deliberately does not import recursive contraction,
//! deformation, packing, child construction, or packet tensors.

use crate::ring::{Element, Ring};
use crate::{integrity, overflow, BigInt, BigUint, Meter, Result};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const SCHEMA: u32 = 2;
const MAX_MODE_DEPTH: usize = 1_024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CertifiedElement(pub(crate) Vec<BigUint>);

impl CertifiedElement {
    #[cfg(test)]
    pub(crate) fn encode(value: Element) -> Self {
        Self(value.coordinates().to_vec())
    }

    fn validate(&self, ring: &Ring) -> std::result::Result<(), String> {
        if self.0.len() != ring.degree() || self.0.iter().any(|value| value >= ring.modulus()) {
            return Err("noncanonical certificate element".into());
        }
        Ok(())
    }

    pub(crate) fn decode(&self, ring: &Ring) -> std::result::Result<Element, String> {
        self.validate(ring)?;
        Ok(Element(self.0.clone()))
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(BigUint::is_zero)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CertifiedMode {
    pub(crate) packet: usize,
    pub(crate) source: usize,
    pub(crate) target: usize,
    pub(crate) weight: usize,
    pub(crate) rows: Vec<(usize, CertifiedElement)>,
    pub(crate) columns: Vec<(usize, BigUint)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct EntryTable {
    pub(crate) beta: Vec<usize>,
    pub(crate) group: usize,
    pub(crate) packet: usize,
    pub(crate) values: Vec<CertifiedElement>,
}

/// Schema-versioned finite data checked by [`verify`].
#[derive(Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub(crate) schema: u32,
    pub(crate) degree: usize,
    pub(crate) radix: usize,
    pub(crate) precision: usize,
    pub(crate) tiles: usize,
    pub(crate) edge_count: usize,
    pub(crate) packets: Vec<Vec<Option<usize>>>,
    pub(crate) left_groups: Vec<usize>,
    pub(crate) right_groups: Vec<usize>,
    pub(crate) weights: Vec<Vec<Vec<CertifiedElement>>>,
    pub(crate) modes: Vec<CertifiedMode>,
    pub(crate) tuples: Vec<Vec<usize>>,
    pub(crate) entries: Vec<EntryTable>,
}

/// Explicit structural description bounds derived from certificate dimensions.
#[derive(Clone, Debug, Serialize)]
pub struct DescriptionEnvelope {
    /// Maximum packet length present in the certificate.
    pub packet_length: usize,
    /// Validated tile period `2^(i+1)` used to construct modes.
    pub tiles: usize,
    /// Number of generated modes.
    pub modes: usize,
    /// Number of retained exponent tuples.
    pub tuples: usize,
    /// Source bound `K * tiles` on modes of each positive weight.
    pub modes_per_weight_bound: usize,
    /// Source bound `K * tiles * (p - 1)` on all retained modes.
    pub attempted_modes_bound: usize,
    /// Base-two exponent satisfying `T <= 2^tuple_log2_bound`.
    pub tuple_log2_bound: usize,
    /// Bound `T * R * K` on packet entry tables.
    pub entry_tables_bound: usize,
    /// Bound `T * K * (2^(2L) + R - 1)` on packet entry values.
    pub entry_values_bound: usize,
    /// Bound on weight, mode-row, and packet-entry ring elements.
    pub ring_elements_bound: usize,
    /// Actual ring-element count in the supplied certificate.
    pub actual_ring_elements: usize,
    /// Coarse byte ceiling for the schema-versioned JSON representation.
    pub json_bytes_bound: usize,
    /// Coarse bit ceiling for a canonical length-prefixed binary representation.
    pub canonical_binary_bits_bound: usize,
}

/// Scalar source coordinates used to refine an accepted factorization
/// certificate to one active recursion node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceDimensionWitness {
    /// Packet-length exponent, so every packet has `L=2^a` slots.
    pub a: u32,
    /// Number of retained valuation levels.
    pub u: u32,
    /// Active valuation coordinate `i<u`.
    pub coordinate: u32,
    /// Whether failure of `p u<=34 M D` rejects this source shape.
    pub require_normalized_precision: bool,
}

/// Independently reconstructed source dimensions for one accepted certificate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct SourceDimensionSummary {
    /// Serialized graph edge count `M`.
    pub edges: BigUint,
    /// Serialized packet count `K`.
    pub packets: BigUint,
    /// Reconstructed packet length `L=2^a`.
    pub packet_length: BigUint,
    /// Reconstructed tile period `tau=2^(i+1)`.
    pub tile_period: BigUint,
    /// Reconstructed interaction distance `D=L/2^i`.
    pub interaction_distance: BigUint,
    /// Serialized coefficient precision `p`.
    pub precision: BigUint,
    /// Nonnegative slack in `K L<=3M`.
    pub packing_slack: BigUint,
    /// Nonnegative slack in `p u<=34 M D`, when that inequality holds.
    pub precision_slack: Option<BigUint>,
    /// Whether the normalized precision inequality holds.
    pub normalized_precision_admitted: bool,
    /// Recomputed `K tau` mode bound for each weight.
    pub modes_per_weight_bound: usize,
    /// Recomputed `K tau (p-1)` total mode bound.
    pub attempted_modes_bound: usize,
    /// Recomputed weighted-tuple base-two exponent ceiling.
    pub tuple_log2_bound: usize,
}

/// One stored unit inverse in the production Cauchy kernel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CauchyFactorWitness {
    /// Grid difference `q=y-x` indexing the factor map.
    pub difference: i64,
    /// Exact power of two removed from `t-q`.
    pub valuation: usize,
    /// Canonical power-basis coordinates of the stored unit inverse.
    pub inverse: Vec<BigUint>,
}

/// Typed scalar witness for one production Cauchy kernel.
///
/// This is deliberately separate from schema 2: the finite factorization
/// certificate remains stable, while callers that possess the source kernel
/// can additionally bind its concrete stored values to the scalar formulas.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CauchySourceWitness {
    /// Packet-length exponent, so every packet has `L=2^a` slots.
    pub a: u32,
    /// Active valuation coordinate `i`, giving tile period `2^(i+1)`.
    pub coordinate: u32,
    /// Nonzero Cauchy center `t` in the signed grid `[-L,L)`.
    pub center: i64,
    /// Canonical coordinates of the extension-ring sample `z`.
    pub sample: Vec<BigUint>,
    /// Canonical coordinates of the stored diagonal value.
    pub base: Vec<BigUint>,
    /// Canonical coordinates of the stored common numerator unit.
    pub numerator_unit: Vec<BigUint>,
    /// Complete stored factor map, in increasing difference order.
    pub factors: Vec<CauchyFactorWitness>,
}

/// Counts from independent scalar Cauchy witness verification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CauchySourceSummary {
    /// Signed grid size `2L`.
    pub grid_points: usize,
    /// Number of independently reconstructed noncenter factors.
    pub factors: usize,
    /// Number of packet-matrix weights checked.
    pub weights: usize,
    /// Number of nonzero mode-row coefficients checked.
    pub mode_rows: usize,
    /// Number of nonzero mode-column coefficients checked.
    pub mode_columns: usize,
}

fn checked_product(values: &[usize], label: &str) -> std::result::Result<usize, String> {
    values.iter().try_fold(1usize, |product, &value| {
        product
            .checked_mul(value)
            .ok_or_else(|| format!("{label} overflow"))
    })
}

fn checked_sum(values: &[usize], label: &str) -> std::result::Result<usize, String> {
    values.iter().try_fold(0usize, |sum, &value| {
        sum.checked_add(value)
            .ok_or_else(|| format!("{label} overflow"))
    })
}

fn index_width(maximum: usize) -> usize {
    usize::BITS as usize - maximum.leading_zeros() as usize
}

fn ceil_sqrt(value: usize) -> usize {
    if value <= 1 {
        return value;
    }
    let mut low = 1usize;
    let mut high = value;
    while low < high {
        let middle = low + (high - low) / 2;
        let quotient = value / middle;
        let remainder = value % middle;
        if middle > quotient || (middle == quotient && remainder == 0) {
            high = middle;
        } else {
            low = middle + 1;
        }
    }
    low
}

fn ceil_log2_count(value: usize) -> usize {
    if value <= 1 {
        0
    } else {
        usize::BITS as usize - (value - 1).leading_zeros() as usize
    }
}

fn source_count_bounds(
    certificate: &Certificate,
    packet_length: usize,
) -> std::result::Result<(usize, usize, usize), String> {
    if certificate.tiles < 2
        || !certificate.tiles.is_power_of_two()
        || certificate.tiles > packet_length
    {
        return Err("certificate tile period".into());
    }
    let modes_per_weight_bound = checked_product(
        &[certificate.packets.len(), certificate.tiles],
        "modes per weight bound",
    )?;
    let attempted_modes_bound = checked_product(
        &[
            modes_per_weight_bound,
            certificate.precision.saturating_sub(1),
        ],
        "attempted modes bound",
    )?;
    let tuple_argument = checked_product(
        &[modes_per_weight_bound, certificate.precision],
        "tuple exponent argument",
    )?;
    let tuple_log2_bound = ceil_sqrt(tuple_argument)
        .checked_mul(5)
        .ok_or_else(|| "tuple exponent bound overflow".to_string())?;
    Ok((
        modes_per_weight_bound,
        attempted_modes_bound,
        tuple_log2_bound,
    ))
}

/// Derive explicit JSON and canonical-binary description ceilings for a
/// certificate already accepted by [`verify`].
///
/// For each packet, group arities sum to at most `2L`, so convexity gives
/// `sum_g 2^arity_g <= 2^(2L) + R - 1`. The remaining terms count packet
/// weights, mode rows/columns, tuples, repeated entry-table headers, JSON
/// containers, and fixed-width ring coordinates.
pub fn description_envelope(
    certificate: &Certificate,
) -> std::result::Result<DescriptionEnvelope, String> {
    if certificate.degree == 0 || certificate.precision == 0 || certificate.radix == 0 {
        return Err("certificate context".into());
    }
    let edges = certificate.edge_count;
    let packets = certificate.packets.len();
    let packet_length = certificate.packets.iter().map(Vec::len).max().unwrap_or(0);
    let modes = certificate.modes.len();
    let tuples = certificate.tuples.len();
    let radix = certificate.radix;
    let degree = certificate.degree;
    let precision = certificate.precision;
    let (modes_per_weight_bound, attempted_modes_bound, tuple_log2_bound) =
        source_count_bounds(certificate, packet_length)?;
    if modes > attempted_modes_bound || ceil_log2_count(tuples) > tuple_log2_bound {
        return Err("certificate exceeds source count envelope".into());
    }
    let mut modes_by_weight = BTreeMap::new();
    for mode in &certificate.modes {
        *modes_by_weight.entry(mode.weight).or_insert(0usize) += 1;
    }
    if modes_by_weight
        .values()
        .any(|&count| count > modes_per_weight_bound)
    {
        return Err("certificate exceeds per-weight mode envelope".into());
    }
    let twice_length = packet_length
        .checked_mul(2)
        .ok_or_else(|| "packet arity overflow".to_string())?;
    let assignment_bound = 1usize
        .checked_shl(u32::try_from(twice_length).map_err(|_| "packet arity overflow".to_string())?)
        .ok_or_else(|| "packet assignment bound overflow".to_string())?;
    let group_value_bound = assignment_bound
        .checked_add(radix - 1)
        .ok_or_else(|| "packet group-value bound overflow".to_string())?;
    let entry_tables_bound = checked_product(&[tuples, radix, packets], "entry table bound")?;
    let entry_values_bound =
        checked_product(&[tuples, packets, group_value_bound], "entry value bound")?;
    let weight_elements_bound = checked_product(
        &[packets, packet_length, packet_length],
        "weight element bound",
    )?;
    let mode_elements_bound = checked_product(&[modes, packet_length], "mode element bound")?;
    let ring_elements_bound = checked_sum(
        &[
            weight_elements_bound,
            mode_elements_bound,
            entry_values_bound,
        ],
        "ring element bound",
    )?;
    let actual_weight_elements = certificate.weights.iter().flatten().flatten().count();
    let actual_mode_elements = certificate.modes.iter().try_fold(0usize, |sum, mode| {
        sum.checked_add(mode.rows.len())
            .ok_or_else(|| "actual mode elements overflow".to_string())
    })?;
    let actual_entry_values = certificate.entries.iter().try_fold(0usize, |sum, table| {
        sum.checked_add(table.values.len())
            .ok_or_else(|| "actual entry values overflow".to_string())
    })?;
    let actual_ring_elements = checked_sum(
        &[
            actual_weight_elements,
            actual_mode_elements,
            actual_entry_values,
        ],
        "actual ring elements",
    )?;
    if certificate.entries.len() > entry_tables_bound
        || actual_weight_elements > weight_elements_bound
        || actual_mode_elements > mode_elements_bound
        || actual_entry_values > entry_values_bound
        || actual_ring_elements > ring_elements_bound
    {
        return Err("certificate exceeds description envelope".into());
    }
    let mode_columns_bound = checked_product(&[modes, packet_length], "mode column bound")?;
    let index_atoms = checked_sum(
        &[
            6,
            checked_product(&[packets, packet_length], "packet slot atoms")?,
            edges
                .checked_mul(2)
                .ok_or_else(|| "incidence atoms overflow".to_string())?,
            modes
                .checked_mul(4)
                .ok_or_else(|| "mode header atoms overflow".to_string())?,
            mode_columns_bound
                .checked_mul(2)
                .ok_or_else(|| "mode placement atoms overflow".to_string())?,
            checked_product(&[tuples, modes], "tuple atoms")?,
            checked_product(
                &[
                    entry_tables_bound,
                    modes
                        .checked_add(2)
                        .ok_or_else(|| "entry header width overflow".to_string())?,
                ],
                "entry header atoms",
            )?,
        ],
        "index atoms",
    )?;
    let limbs = precision
        .checked_add(31)
        .ok_or_else(|| "coefficient limb bound overflow".to_string())?
        / 32;
    let coefficient_arrays = checked_sum(
        &[
            checked_product(&[ring_elements_bound, degree], "ring coefficient arrays")?,
            mode_columns_bound,
        ],
        "coefficient arrays",
    )?;
    let limb_atoms = checked_product(&[coefficient_arrays, limbs], "coefficient limb atoms")?;
    let atoms = index_atoms
        .checked_add(limb_atoms)
        .ok_or_else(|| "JSON atom bound overflow".to_string())?;
    let containers = checked_sum(
        &[
            8,
            packets
                .checked_mul(2)
                .ok_or_else(|| "packet container bound overflow".to_string())?,
            checked_product(&[packets, packet_length], "weight row containers")?,
            checked_product(
                &[
                    ring_elements_bound,
                    degree
                        .checked_add(1)
                        .ok_or_else(|| "ring element container width overflow".to_string())?,
                ],
                "ring element containers",
            )?,
            modes
                .checked_mul(3)
                .ok_or_else(|| "mode containers overflow".to_string())?,
            mode_columns_bound
                .checked_mul(3)
                .ok_or_else(|| "mode pair containers overflow".to_string())?,
            tuples,
            entry_tables_bound
                .checked_mul(3)
                .ok_or_else(|| "entry containers overflow".to_string())?,
        ],
        "JSON containers",
    )?;
    let fields = checked_sum(
        &[
            13,
            modes
                .checked_mul(6)
                .ok_or_else(|| "mode field bound overflow".to_string())?,
            entry_tables_bound
                .checked_mul(4)
                .ok_or_else(|| "entry field bound overflow".to_string())?,
        ],
        "JSON fields",
    )?;
    let json_units = checked_sum(&[atoms, containers, fields, 1], "JSON units")?;
    let json_bytes_bound = checked_product(&[json_units, 64], "JSON byte description bound")?;
    let maximum_index = [
        edges,
        packets,
        packet_length,
        certificate.tiles,
        radix,
        modes,
        tuples,
        entry_tables_bound,
        assignment_bound,
    ]
    .into_iter()
    .max()
    .unwrap_or(1);
    let width = index_width(maximum_index.max(1));
    let value_bits = checked_product(
        &[coefficient_arrays, precision],
        "canonical coefficient bits",
    )?;
    let index_bits = checked_product(&[index_atoms, width], "canonical index bits")?;
    let framing_bits = checked_product(
        &[
            containers
                .checked_add(fields)
                .and_then(|value| value.checked_add(1))
                .ok_or_else(|| "canonical framing units overflow".to_string())?,
            64,
        ],
        "canonical framing bits",
    )?;
    let canonical_binary_bits_bound = checked_sum(
        &[value_bits, index_bits, framing_bits],
        "canonical binary description bound",
    )?;
    Ok(DescriptionEnvelope {
        packet_length,
        tiles: certificate.tiles,
        modes,
        tuples,
        modes_per_weight_bound,
        attempted_modes_bound,
        tuple_log2_bound,
        entry_tables_bound,
        entry_values_bound,
        ring_elements_bound,
        actual_ring_elements,
        json_bytes_bound,
        canonical_binary_bits_bound,
    })
}

/// Bind an accepted finite certificate to the scalar source premises.
///
/// The caller must first run [`verify`]. This independent refinement checks
/// uniform packet length, the tile/coordinate identity, packet capacity, and
/// the normalized precision inequality directly from serialized dimensions.
/// The witness chooses whether a failed normalized-precision inequality is a
/// rejection or a reported diagnostic boundary. This function does not prove
/// that a graph or the source packing routine produced the certificate.
pub fn verify_source_dimensions(
    certificate: &Certificate,
    witness: &SourceDimensionWitness,
) -> std::result::Result<SourceDimensionSummary, String> {
    if witness.u == 0
        || witness.u > witness.a
        || witness.coordinate >= witness.u
        || certificate.edge_count == 0
        || certificate.packets.is_empty()
        || certificate.precision == 0
    {
        return Err("source dimension context".into());
    }
    let packet_length = 1usize
        .checked_shl(witness.a)
        .ok_or_else(|| "source packet length width".to_string())?;
    if certificate
        .packets
        .iter()
        .any(|packet| packet.len() != packet_length)
    {
        return Err("source packet length".into());
    }
    let tile_period = 1usize
        .checked_shl(
            witness
                .coordinate
                .checked_add(1)
                .ok_or_else(|| "source tile width".to_string())?,
        )
        .ok_or_else(|| "source tile width".to_string())?;
    if certificate.tiles != tile_period {
        return Err("source tile period".into());
    }
    let interaction_distance = packet_length >> witness.coordinate;
    let edges = BigUint::from(certificate.edge_count);
    let packets = BigUint::from(certificate.packets.len());
    let length = BigUint::from(packet_length);
    let tiles = BigUint::from(tile_period);
    let distance = BigUint::from(interaction_distance);
    let precision = BigUint::from(certificate.precision);
    if BigUint::from(witness.u) > edges {
        return Err("source valuation levels".into());
    }
    let packing_lhs = &packets * &length;
    let packing_rhs = BigUint::from(3u8) * &edges;
    if packing_lhs > packing_rhs {
        return Err("source packet capacity".into());
    }
    let modes_per_weight = &packets * &tiles;
    if &modes_per_weight * &distance > BigUint::from(6u8) * &edges {
        return Err("source mode distance".into());
    }
    let precision_lhs = &precision * BigUint::from(witness.u);
    let precision_rhs = BigUint::from(34u8) * &edges * &distance;
    let normalized_precision_admitted = precision_lhs <= precision_rhs;
    if witness.require_normalized_precision && !normalized_precision_admitted {
        return Err("source precision".into());
    }
    let envelope = description_envelope(certificate)?;
    if envelope.packet_length != packet_length
        || envelope.tiles != tile_period
        || BigUint::from(envelope.modes_per_weight_bound) != modes_per_weight
    {
        return Err("source description refinement".into());
    }
    Ok(SourceDimensionSummary {
        edges,
        packets,
        packet_length: length,
        tile_period: tiles,
        interaction_distance: distance,
        precision,
        packing_slack: packing_rhs - packing_lhs,
        precision_slack: normalized_precision_admitted
            .then(|| precision_rhs.clone() - precision_lhs),
        normalized_precision_admitted,
        modes_per_weight_bound: envelope.modes_per_weight_bound,
        attempted_modes_bound: envelope.attempted_modes_bound,
        tuple_log2_bound: envelope.tuple_log2_bound,
    })
}

fn cauchy_valuation(value: i64) -> usize {
    debug_assert_ne!(value, 0);
    value.unsigned_abs().trailing_zeros() as usize
}

struct IndependentCauchy {
    base: Element,
    numerator_unit: Element,
    factors: BTreeMap<i64, (usize, Element)>,
}

fn independent_cauchy(
    packet_length: usize,
    a: usize,
    center: i64,
    sample: &Element,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<IndependentCauchy, String> {
    let length =
        i64::try_from(packet_length).map_err(|_| "Cauchy signed packet length".to_string())?;
    let grid_points = packet_length
        .checked_mul(2)
        .ok_or_else(|| "Cauchy grid width".to_string())?;
    meter
        .vector(grid_points, "Cauchy witness grid")
        .map_err(|error| error.to_string())?;
    if center == 0 || !(-length..length).contains(&center) {
        return Err("Cauchy witness center".into());
    }

    let mut denominator = ring.one();
    let mut denominator_valuation = 0usize;
    for q in -length..length {
        if q == 0 {
            continue;
        }
        let valuation = cauchy_valuation(q);
        denominator_valuation = denominator_valuation
            .checked_add(valuation)
            .ok_or_else(|| "Cauchy denominator valuation".to_string())?;
        let odd = BigInt::from(-q) >> valuation;
        denominator = ring
            .mul(&denominator, &ring.signed(&odd), meter)
            .map_err(|error| error.to_string())?;
    }
    let inverse_denominator = ring
        .inverse(&denominator, meter)
        .map_err(|error| error.to_string())?;

    let mut product = ring.one();
    let mut numerator_valuation = 0usize;
    let mut factors = BTreeMap::new();
    for q in -length..length {
        if q == center {
            continue;
        }
        let difference = center
            .checked_sub(q)
            .ok_or_else(|| "Cauchy grid difference".to_string())?;
        let valuation = cauchy_valuation(difference);
        if valuation > a {
            return Err("Cauchy grid difference valuation".into());
        }
        numerator_valuation = numerator_valuation
            .checked_add(valuation)
            .ok_or_else(|| "Cauchy numerator valuation".to_string())?;
        let odd = BigInt::from(difference) >> valuation;
        let shift = a
            .checked_add(1)
            .and_then(|value| value.checked_sub(valuation))
            .ok_or_else(|| "Cauchy factor shift".to_string())?;
        let scaled_sample = ring
            .shift(sample, shift)
            .map_err(|error| error.to_string())?;
        let factor = ring
            .add(&ring.signed(&odd), &scaled_sample, meter)
            .map_err(|error| error.to_string())?;
        product = ring
            .mul(&product, &factor, meter)
            .map_err(|error| error.to_string())?;
        let inverse = ring
            .inverse(&factor, meter)
            .map_err(|error| error.to_string())?;
        factors.insert(q, (valuation, inverse));
    }
    if numerator_valuation != denominator_valuation {
        return Err("Cauchy derivative content".into());
    }
    let base = ring
        .mul(&product, &inverse_denominator, meter)
        .map_err(|error| error.to_string())?;
    let numerator_unit = ring
        .mul(&base, sample, meter)
        .map_err(|error| error.to_string())?;
    Ok(IndependentCauchy {
        base,
        numerator_unit,
        factors,
    })
}

fn independent_cauchy_weight(
    cauchy: &IndependentCauchy,
    a: usize,
    center: i64,
    x: usize,
    y: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<Element, String> {
    let x = i64::try_from(x).map_err(|_| "Cauchy row position".to_string())?;
    let y = i64::try_from(y).map_err(|_| "Cauchy column position".to_string())?;
    let q = y
        .checked_sub(x)
        .ok_or_else(|| "Cauchy packet difference".to_string())?;
    if q == center {
        return Ok(cauchy.base.clone());
    }
    let (valuation, inverse) = cauchy
        .factors
        .get(&q)
        .ok_or_else(|| "Cauchy witness factor coverage".to_string())?;
    let product = ring
        .mul(&cauchy.numerator_unit, inverse, meter)
        .map_err(|error| error.to_string())?;
    let shift = a
        .checked_add(1)
        .and_then(|value| value.checked_sub(*valuation))
        .ok_or_else(|| "Cauchy weight shift".to_string())?;
    ring.shift(&product, shift)
        .map_err(|error| error.to_string())
}

fn independent_mode_coefficient(
    cauchy: &IndependentCauchy,
    a: usize,
    coordinate: usize,
    x: usize,
    y0: usize,
    exponent: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<Element, String> {
    let x = i64::try_from(x).map_err(|_| "Cauchy mode row".to_string())?;
    let y0 = i64::try_from(y0).map_err(|_| "Cauchy mode tile".to_string())?;
    let q = y0
        .checked_sub(x)
        .ok_or_else(|| "Cauchy mode difference".to_string())?;
    let (valuation, inverse) = cauchy
        .factors
        .get(&q)
        .ok_or_else(|| "Cauchy mode factor coverage".to_string())?;
    if *valuation > coordinate || coordinate > a {
        return Err("Cauchy mode valuation bound".into());
    }
    let mode_shift = coordinate
        .checked_add(1)
        .and_then(|value| value.checked_sub(*valuation))
        .ok_or_else(|| "Cauchy mode shift".to_string())?;
    let base_shift = a
        .checked_add(1)
        .and_then(|value| value.checked_sub(*valuation))
        .ok_or_else(|| "Cauchy mode base shift".to_string())?;
    let shift = mode_shift
        .checked_mul(exponent)
        .and_then(|value| value.checked_add(base_shift))
        .ok_or_else(|| "Cauchy mode coefficient shift".to_string())?;
    if shift >= ring.precision() {
        return Ok(ring.zero());
    }
    let power = ring
        .pow(inverse, exponent + 1, meter)
        .map_err(|error| error.to_string())?;
    let coefficient = ring
        .mul(&cauchy.numerator_unit, &power, meter)
        .map_err(|error| error.to_string())?;
    ring.shift(&coefficient, shift)
        .map_err(|error| error.to_string())
}

/// Independently bind an accepted schema-2 factorization certificate to the
/// concrete scalar values stored by one production Cauchy kernel.
///
/// The verifier imports only exact ring arithmetic and serialized data. It
/// reconstructs the signed grid, derivative content, every unit factor and
/// inverse, the base and numerator unit, every packet weight, and every
/// nonzero mode row/column. It does not import deformation, packing, child
/// construction, or recursive contraction code.
pub fn verify_cauchy_source(
    certificate: &Certificate,
    witness: &CauchySourceWitness,
    meter: &mut Meter,
) -> std::result::Result<CauchySourceSummary, String> {
    let ring = Ring::new(certificate.degree, certificate.precision, meter)
        .map_err(|error| error.to_string())?;
    verify_with_ring(certificate, &ring, meter)?;

    let a = usize::try_from(witness.a).map_err(|_| "Cauchy witness exponent".to_string())?;
    let coordinate =
        usize::try_from(witness.coordinate).map_err(|_| "Cauchy witness coordinate".to_string())?;
    if witness.center == 0 || coordinate > a || cauchy_valuation(witness.center) != coordinate {
        return Err("Cauchy witness active coordinate".into());
    }
    let packet_length = 1usize
        .checked_shl(witness.a)
        .ok_or_else(|| "Cauchy witness packet width".to_string())?;
    if certificate
        .packets
        .iter()
        .any(|packet| packet.len() != packet_length)
    {
        return Err("Cauchy witness packet length".into());
    }
    let tiles = 1usize
        .checked_shl(
            witness
                .coordinate
                .checked_add(1)
                .ok_or_else(|| "Cauchy witness tile width".to_string())?,
        )
        .ok_or_else(|| "Cauchy witness tile width".to_string())?;
    if certificate.tiles != tiles {
        return Err("Cauchy witness tile period".into());
    }

    let sample = CertifiedElement(witness.sample.clone()).decode(&ring)?;
    let expected = independent_cauchy(packet_length, a, witness.center, &sample, &ring, meter)?;
    if expected.base.coordinates() != witness.base.as_slice() {
        return Err("Cauchy witness base".into());
    }
    if expected.numerator_unit.coordinates() != witness.numerator_unit.as_slice() {
        return Err("Cauchy witness numerator unit".into());
    }

    let mut claimed_factors = BTreeMap::new();
    for factor in &witness.factors {
        let inverse = CertifiedElement(factor.inverse.clone()).decode(&ring)?;
        if claimed_factors
            .insert(factor.difference, (factor.valuation, inverse))
            .is_some()
        {
            return Err("Cauchy witness factor duplicate".into());
        }
    }
    if claimed_factors.len() != expected.factors.len()
        || claimed_factors.keys().ne(expected.factors.keys())
    {
        return Err("Cauchy witness factor coverage".into());
    }
    for (difference, (valuation, inverse)) in &expected.factors {
        let (claimed_valuation, claimed_inverse) = claimed_factors
            .get(difference)
            .ok_or_else(|| "Cauchy witness factor coverage".to_string())?;
        if claimed_valuation != valuation {
            return Err("Cauchy witness factor valuation".into());
        }
        if claimed_inverse != inverse {
            return Err("Cauchy witness factor inverse".into());
        }
    }

    let mut weights = 0usize;
    for (packet, rows) in certificate.weights.iter().enumerate() {
        for (row, columns) in rows.iter().enumerate() {
            for (column, actual) in columns.iter().enumerate() {
                let expected_weight = independent_cauchy_weight(
                    &expected,
                    a,
                    witness.center,
                    row,
                    column,
                    &ring,
                    meter,
                )?;
                if expected_weight.coordinates() != actual.0.as_slice() {
                    return Err(format!(
                        "Cauchy packet weight: packet={packet}, row={row}, column={column}"
                    ));
                }
                weights = weights
                    .checked_add(1)
                    .ok_or_else(|| "Cauchy weight count".to_string())?;
            }
        }
    }

    let mut mode_rows = 0usize;
    let mut mode_columns = 0usize;
    for (mode_index, mode) in certificate.modes.iter().enumerate() {
        let exponent = mode.weight - 1;
        let y0 = mode.columns[0].0 % tiles;
        let slots = &certificate.packets[mode.packet];
        let mut expected_rows = BTreeMap::new();
        for (row, edge) in slots.iter().enumerate() {
            let Some(edge) = edge else {
                continue;
            };
            if certificate.left_groups[*edge] != mode.target {
                continue;
            }
            let coefficient = independent_mode_coefficient(
                &expected, a, coordinate, row, y0, exponent, &ring, meter,
            )?;
            if !coefficient.is_zero() {
                expected_rows.insert(row, coefficient);
            }
        }
        let actual_rows: BTreeMap<_, _> = mode
            .rows
            .iter()
            .map(|(row, value)| Ok((*row, value.decode(&ring)?)))
            .collect::<std::result::Result<_, String>>()?;
        if actual_rows != expected_rows {
            return Err(format!("Cauchy mode rows: mode={mode_index}"));
        }

        let mut expected_columns = BTreeMap::new();
        for column in (y0..packet_length).step_by(tiles) {
            let Some(edge) = slots[column] else {
                continue;
            };
            if certificate.right_groups[edge] != mode.source {
                return Err(format!("Cauchy mode tile group: mode={mode_index}"));
            }
            let j = (column - y0) / tiles;
            let value = BigUint::from(j).modpow(&BigUint::from(exponent), ring.modulus());
            if !value.is_zero() {
                expected_columns.insert(column, value);
            }
        }
        let actual_columns: BTreeMap<_, _> = mode.columns.iter().cloned().collect();
        if actual_columns != expected_columns {
            return Err(format!("Cauchy mode columns: mode={mode_index}"));
        }
        mode_rows = mode_rows
            .checked_add(actual_rows.len())
            .ok_or_else(|| "Cauchy mode-row count".to_string())?;
        mode_columns = mode_columns
            .checked_add(actual_columns.len())
            .ok_or_else(|| "Cauchy mode-column count".to_string())?;
    }

    Ok(CauchySourceSummary {
        grid_points: packet_length
            .checked_mul(2)
            .ok_or_else(|| "Cauchy grid count".to_string())?,
        factors: expected.factors.len(),
        weights,
        mode_rows,
        mode_columns,
    })
}

fn preflight(certificate: &Certificate, meter: &mut Meter) -> std::result::Result<(), String> {
    if certificate.schema != SCHEMA
        || certificate.degree == 0
        || certificate.precision == 0
        || certificate.radix == 0
        || certificate.tiles == 0
    {
        return Err("certificate context".into());
    }
    if certificate.modes.len() > MAX_MODE_DEPTH {
        return Err("certificate mode-depth limit".into());
    }
    for (length, phase) in [
        (certificate.edge_count, "certificate edges"),
        (certificate.packets.len(), "certificate packets"),
        (certificate.modes.len(), "certificate modes"),
        (certificate.tuples.len(), "certificate tuples"),
        (certificate.entries.len(), "certificate entry tables"),
    ] {
        meter
            .vector(length, phase)
            .map_err(|error| error.to_string())?;
    }
    for slots in &certificate.packets {
        meter
            .vector(slots.len(), "certificate packet slots")
            .map_err(|error| error.to_string())?;
    }
    for tuple in &certificate.tuples {
        meter
            .vector(tuple.len(), "certificate tuple width")
            .map_err(|error| error.to_string())?;
    }
    for table in &certificate.entries {
        meter
            .vector(table.values.len(), "certificate entry values")
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn independent_tuples(
    certificate: &Certificate,
    meter: &mut Meter,
) -> std::result::Result<BTreeSet<Vec<usize>>, String> {
    let mut bounds = BTreeMap::new();
    for (packet, slots) in certificate.packets.iter().enumerate() {
        for &edge in slots.iter().flatten() {
            *bounds
                .entry((packet, certificate.left_groups[edge]))
                .or_insert(0usize) += 1;
        }
    }

    fn visit(
        index: usize,
        certificate: &Certificate,
        bounds: &BTreeMap<(usize, usize), usize>,
        beta: &mut [usize],
        usage: &mut BTreeMap<(usize, usize), usize>,
        weight: usize,
        out: &mut BTreeSet<Vec<usize>>,
        meter: &mut Meter,
    ) -> std::result::Result<(), String> {
        meter
            .charge(1, "certificate tuple enumeration")
            .map_err(|error| error.to_string())?;
        if index == certificate.modes.len() {
            if out.len() >= meter.limits.vector {
                return Err("resource limit: certificate tuple set".into());
            }
            out.insert(beta.to_vec());
            return Ok(());
        }
        let mode = &certificate.modes[index];
        let group = (mode.packet, mode.target);
        let group_capacity =
            bounds.get(&group).copied().unwrap_or(0) - usage.get(&group).copied().unwrap_or(0);
        let weight_capacity = (certificate.precision - 1 - weight) / mode.weight;
        for exponent in 0..=group_capacity.min(weight_capacity) {
            beta[index] = exponent;
            *usage.entry(group).or_insert(0) += exponent;
            let next_weight = exponent
                .checked_mul(mode.weight)
                .and_then(|extra| weight.checked_add(extra))
                .ok_or_else(|| "weighted tuple arithmetic overflow".to_string())?;
            visit(
                index + 1,
                certificate,
                bounds,
                beta,
                usage,
                next_weight,
                out,
                meter,
            )?;
            *usage.get_mut(&group).unwrap() -= exponent;
        }
        beta[index] = 0;
        Ok(())
    }

    let mut beta = vec![0usize; certificate.modes.len()];
    let mut usage = BTreeMap::new();
    let mut out = BTreeSet::new();
    visit(
        0,
        certificate,
        &bounds,
        &mut beta,
        &mut usage,
        0,
        &mut out,
        meter,
    )?;
    Ok(out)
}

fn expected_entry(
    certificate: &Certificate,
    beta: &[usize],
    group: usize,
    packet: usize,
    mask: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> Result<Element> {
    let slots = &certificate.packets[packet];
    let rows: Vec<_> = slots
        .iter()
        .enumerate()
        .filter_map(|(slot, edge)| {
            edge.filter(|&edge| certificate.left_groups[edge] == group)
                .map(|_| slot)
        })
        .collect();
    let columns: Vec<_> = slots
        .iter()
        .enumerate()
        .filter_map(|(slot, edge)| {
            edge.filter(|&edge| certificate.right_groups[edge] == group)
                .map(|_| slot)
        })
        .collect();
    let arity = rows
        .len()
        .checked_add(columns.len())
        .ok_or_else(|| overflow("certificate packet arity"))?;
    let bits: Vec<_> = (0..arity).map(|bit| mask & (1usize << bit) != 0).collect();
    let active_incoming: Vec<_> = certificate
        .modes
        .iter()
        .enumerate()
        .filter(|(mode, description)| {
            beta[*mode] != 0 && description.packet == packet && description.target == group
        })
        .collect();
    let mut constants = Vec::new();
    let mut incoming = vec![Vec::new(); active_incoming.len()];
    for (row_index, &row) in rows.iter().enumerate() {
        let mut local = ring.zero();
        for (column_index, &column) in columns.iter().enumerate() {
            if bits[rows.len() + column_index] {
                local = ring.add(
                    &local,
                    &certificate.weights[packet][row][column]
                        .decode(ring)
                        .map_err(|error| integrity(&error))?,
                    meter,
                )?;
            }
        }
        constants.push(if bits[row_index] {
            local
        } else {
            ring.sub(&ring.one(), &local, meter)?
        });
        for (active, (_, mode)) in active_incoming.iter().enumerate() {
            let coefficient = mode
                .rows
                .iter()
                .find(|&&(slot, _)| slot == row)
                .map(|(_, value)| value.decode(ring).map_err(|error| integrity(&error)))
                .transpose()?
                .unwrap_or_else(|| ring.zero());
            incoming[active].push(if bits[row_index] {
                coefficient
            } else {
                ring.sub(&ring.zero(), &coefficient, meter)?
            });
        }
    }
    let choice_radix = active_incoming
        .len()
        .checked_add(1)
        .ok_or_else(|| overflow("certificate row-choice radix"))?;
    let assignments = choice_radix
        .checked_pow(
            u32::try_from(rows.len()).map_err(|_| overflow("certificate row-choice arity"))?,
        )
        .ok_or_else(|| overflow("certificate row-choice assignments"))?;
    let mut coefficient = ring.zero();
    for mut choices in 0..assignments {
        let mut counts = vec![0usize; active_incoming.len()];
        let mut product = ring.one();
        for (row, constant) in constants.iter().enumerate() {
            let choice = choices % choice_radix;
            choices /= choice_radix;
            let factor = if choice == 0 {
                constant
            } else {
                counts[choice - 1] += 1;
                &incoming[choice - 1][row]
            };
            product = ring.mul(&product, factor, meter)?;
        }
        if active_incoming
            .iter()
            .zip(&counts)
            .all(|((mode, _), &count)| beta[*mode] == count)
        {
            coefficient = ring.add(&coefficient, &product, meter)?;
        }
    }
    for (mode, description) in certificate.modes.iter().enumerate() {
        if beta[mode] == 0 || description.packet != packet || description.source != group {
            continue;
        }
        let mut form = BigUint::zero();
        for (column_index, &column) in columns.iter().enumerate() {
            if !bits[rows.len() + column_index] {
                continue;
            }
            if let Some((_, value)) = description
                .columns
                .iter()
                .find(|&&(slot, _)| slot == column)
            {
                form = (form + value) % ring.modulus();
            }
        }
        let power = ring.pow(&ring.integer(&form), beta[mode], meter)?;
        coefficient = ring.mul(&coefficient, &power, meter)?;
    }
    Ok(coefficient)
}

pub(crate) fn verify_with_ring(
    certificate: &Certificate,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<(), String> {
    preflight(certificate, meter)?;
    if certificate.degree != ring.degree() || certificate.precision != ring.precision() {
        return Err("certificate context".into());
    }
    for value in certificate
        .weights
        .iter()
        .flatten()
        .flatten()
        .chain(
            certificate
                .modes
                .iter()
                .flat_map(|mode| mode.rows.iter().map(|(_, value)| value)),
        )
        .chain(certificate.entries.iter().flat_map(|table| &table.values))
    {
        value.validate(ring)?;
    }
    if certificate.left_groups.len() != certificate.edge_count
        || certificate.right_groups.len() != certificate.edge_count
        || certificate
            .left_groups
            .iter()
            .chain(&certificate.right_groups)
            .any(|&group| group >= certificate.radix)
    {
        return Err("incidence group partition".into());
    }
    if certificate.weights.len() != certificate.packets.len() {
        return Err("packet weight count".into());
    }
    let packet_length = certificate.packets.iter().map(Vec::len).max().unwrap_or(0);
    let (modes_per_weight_bound, attempted_modes_bound, tuple_log2_bound) =
        source_count_bounds(certificate, packet_length)?;
    if certificate.modes.len() > attempted_modes_bound {
        return Err("mode source count bound".into());
    }
    let mut occurrences = vec![0usize; certificate.edge_count];
    for slots in &certificate.packets {
        for &edge in slots.iter().flatten() {
            let Some(count) = occurrences.get_mut(edge) else {
                return Err("packet edge range".into());
            };
            *count += 1;
        }
    }
    if occurrences.iter().any(|&count| count != 1) {
        return Err("packet coverage".into());
    }
    let mut mode_keys = BTreeSet::new();
    let mut modes_by_weight = BTreeMap::new();
    for mode in &certificate.modes {
        if mode.packet >= certificate.packets.len()
            || mode.source >= certificate.radix
            || mode.target >= certificate.radix
            || mode.source == mode.target
            || !(1..certificate.precision).contains(&mode.weight)
            || mode.rows.is_empty()
            || mode.columns.is_empty()
        {
            return Err("mode header".into());
        }
        let slots = &certificate.packets[mode.packet];
        let mut row_slots = BTreeSet::new();
        for &(slot, ref value) in &mode.rows {
            let Some(Some(edge)) = slots.get(slot) else {
                return Err("mode row slot".into());
            };
            if !row_slots.insert(slot) {
                return Err("mode row duplicate".into());
            }
            if value.is_zero() || certificate.left_groups[*edge] != mode.target {
                return Err("mode row placement".into());
            }
        }
        let mut column_slots = BTreeSet::new();
        for &(slot, ref value) in &mode.columns {
            let Some(Some(edge)) = slots.get(slot) else {
                return Err("mode column slot".into());
            };
            if !column_slots.insert(slot) {
                return Err("mode column duplicate".into());
            }
            if value.is_zero()
                || value >= ring.modulus()
                || certificate.right_groups[*edge] != mode.source
            {
                return Err("mode column placement".into());
            }
        }
        let residue = mode.columns[0].0 % certificate.tiles;
        if mode
            .columns
            .iter()
            .any(|(slot, _)| slot % certificate.tiles != residue)
        {
            return Err("mode tile placement".into());
        }
        if !mode_keys.insert((mode.packet, mode.weight, residue)) {
            return Err("mode tile duplicate".into());
        }
        *modes_by_weight.entry(mode.weight).or_insert(0usize) += 1;
    }
    if modes_by_weight
        .values()
        .any(|&count| count > modes_per_weight_bound)
    {
        return Err("mode per-weight source bound".into());
    }
    for (packet, slots) in certificate.packets.iter().enumerate() {
        if certificate.weights[packet].len() != slots.len()
            || certificate.weights[packet]
                .iter()
                .any(|row| row.len() != slots.len())
        {
            return Err("packet weight shape".into());
        }
        for (row, &row_edge) in slots
            .iter()
            .enumerate()
            .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
        {
            for (column, &column_edge) in slots
                .iter()
                .enumerate()
                .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
            {
                let target = certificate.left_groups[row_edge];
                let source = certificate.right_groups[column_edge];
                if target == source {
                    continue;
                }
                let mut expanded = ring.zero();
                for mode in certificate.modes.iter().filter(|mode| {
                    mode.packet == packet && mode.target == target && mode.source == source
                }) {
                    let Some((_, row_value)) = mode.rows.iter().find(|&&(slot, _)| slot == row)
                    else {
                        continue;
                    };
                    let Some((_, column_value)) =
                        mode.columns.iter().find(|&&(slot, _)| slot == column)
                    else {
                        continue;
                    };
                    let term = ring
                        .mul(&row_value.decode(ring)?, &ring.integer(column_value), meter)
                        .map_err(|error| error.to_string())?;
                    expanded = ring
                        .add(&expanded, &term, meter)
                        .map_err(|error| error.to_string())?;
                }
                if expanded.coordinates() != certificate.weights[packet][row][column].0.as_slice() {
                    return Err("crossing mode series".into());
                }
            }
        }
    }
    if certificate
        .tuples
        .iter()
        .any(|tuple| tuple.len() != certificate.modes.len())
    {
        return Err("weighted tuple shape".into());
    }
    let actual_tuples: BTreeSet<_> = certificate.tuples.iter().cloned().collect();
    if actual_tuples.len() != certificate.tuples.len()
        || actual_tuples != independent_tuples(certificate, meter)?
    {
        return Err("weighted tuple coverage".into());
    }
    if ceil_log2_count(certificate.tuples.len()) > tuple_log2_bound {
        return Err("weighted tuple source bound".into());
    }
    let expected_tables = certificate
        .tuples
        .len()
        .checked_mul(certificate.radix)
        .and_then(|count| count.checked_mul(certificate.packets.len()))
        .ok_or_else(|| "entry table count overflow".to_string())?;
    if certificate.entries.len() != expected_tables {
        return Err("entry table coverage".into());
    }
    let tuple_set: BTreeSet<_> = certificate.tuples.iter().cloned().collect();
    let mut table_keys = BTreeSet::new();
    for table in &certificate.entries {
        if !tuple_set.contains(&table.beta)
            || table.group >= certificate.radix
            || table.packet >= certificate.packets.len()
            || !table_keys.insert((table.beta.clone(), table.group, table.packet))
        {
            return Err("entry table key".into());
        }
        let slots = &certificate.packets[table.packet];
        let arity = slots
            .iter()
            .flatten()
            .filter(|&&edge| certificate.left_groups[edge] == table.group)
            .count()
            .checked_add(
                slots
                    .iter()
                    .flatten()
                    .filter(|&&edge| certificate.right_groups[edge] == table.group)
                    .count(),
            )
            .ok_or_else(|| "entry table arity overflow".to_string())?;
        let assignments = 1usize
            .checked_shl(
                u32::try_from(arity).map_err(|_| "entry table arity overflow".to_string())?,
            )
            .ok_or_else(|| "entry table arity overflow".to_string())?;
        if table.values.len() != assignments {
            return Err("entry table shape".into());
        }
        for (mask, value) in table.values.iter().enumerate() {
            let expected = expected_entry(
                certificate,
                &table.beta,
                table.group,
                table.packet,
                mask,
                ring,
                meter,
            )
            .map_err(|error| error.to_string())?;
            if expected.coordinates() != value.0.as_slice() {
                return Err(format!(
                    "packet coefficient table: beta={:?}, group={}, packet={}, mask={}, slots={:?}, left={:?}, right={:?}, expected={expected:?}, actual={value:?}",
                    table.beta,
                    table.group,
                    table.packet,
                    mask,
                    certificate.packets[table.packet],
                    certificate.left_groups,
                    certificate.right_groups,
                ));
            }
        }
    }
    Ok(())
}

/// Verify a parsed certificate under explicit caller-supplied resource limits.
pub fn verify(certificate: &Certificate, meter: &mut Meter) -> std::result::Result<(), String> {
    preflight(certificate, meter)?;
    let ring = Ring::new(certificate.degree, certificate.precision, meter)
        .map_err(|error| error.to_string())?;
    verify_with_ring(certificate, &ring, meter)
}
