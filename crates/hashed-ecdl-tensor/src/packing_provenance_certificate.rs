//! Independent replay of finite packet-packing provenance.
//!
//! This module consumes a labeled bipartite edge skeleton and an already
//! accepted factorization certificate. It does not import the source packing,
//! deformation, contraction, or graph implementations.

use crate::factorization_certificate::Certificate as FactorizationCertificate;
use crate::{BigUint, Meter};
use serde::{Deserialize, Serialize};

/// Current packing-provenance wire schema.
pub const SCHEMA: u32 = 1;

/// Endpoint vertex indices for one edge in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeEndpoints {
    /// Index of the left endpoint vertex.
    pub left: usize,
    /// Index of the right endpoint vertex.
    pub right: usize,
}

/// Finite data needed to replay packet construction independently.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Packet label radix.
    pub radix: usize,
    /// Packet-length exponent `a`, so `L=2^a`.
    pub packet_exponent: u32,
    /// Active valuation coordinate used for factorization groups.
    pub coordinate: u32,
    /// Edge endpoints in the factorization certificate's edge order.
    pub edges: Vec<EdgeEndpoints>,
    /// Source labels for every left vertex, including isolates.
    pub row_labels: Vec<usize>,
    /// Source labels for every right vertex, including isolates.
    pub column_labels: Vec<usize>,
    /// Unordered radix-digit pair pattern attached to every emitted packet.
    pub packet_pairs: Vec<Vec<(usize, usize)>>,
}

/// Checked dimensions of one accepted packing-provenance replay.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Number of replayed edges.
    pub edges: usize,
    /// Number of left vertices, including isolates.
    pub left_vertices: usize,
    /// Number of right vertices, including isolates.
    pub right_vertices: usize,
    /// Number of reconstructed nonempty packets.
    pub packets: usize,
    /// Slots in every packet.
    pub packet_length: usize,
    /// Number of ordered row/column label classes.
    pub label_classes: usize,
    /// Number of unordered digit patterns replayed before empty suppression.
    pub orientation_patterns: usize,
    /// Nonnegative slack in `K L<=3M`, when that inequality holds.
    pub packing_slack: Option<BigUint>,
    /// Whether the replayed packet array satisfies source capacity.
    pub source_capacity_admitted: bool,
}

fn digit(label: usize, coordinate: u32, radix: usize) -> Option<usize> {
    radix
        .checked_pow(coordinate)
        .map(|place| (label / place) % radix)
}

/// Reconstruct packet ordering, slots, orientations, and active groups.
///
/// The factorization certificate must be verified separately. Acceptance here
/// proves that its packet and group arrays are exactly reproduced by the
/// serialized labeled edge skeleton under the source packet-construction
/// schedule. It does not prove how the vertex labels were selected or that the
/// external contraction theorem applies.
pub fn verify(
    factorization: &FactorizationCertificate,
    provenance: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    if provenance.schema != SCHEMA
        || provenance.radix == 0
        || provenance.coordinate >= provenance.packet_exponent
        || factorization.radix != provenance.radix
        || provenance.edges.len() != factorization.edge_count
    {
        return Err("packing provenance context".into());
    }
    let packet_length = 1usize
        .checked_shl(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance packet width".to_string())?;
    let labels = provenance
        .radix
        .checked_pow(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance label width".to_string())?;
    let label_classes = labels
        .checked_mul(labels)
        .ok_or_else(|| "packing provenance class width".to_string())?;
    if factorization
        .packets
        .iter()
        .any(|packet| packet.len() != packet_length)
        || provenance
            .row_labels
            .iter()
            .chain(&provenance.column_labels)
            .any(|&label| label >= labels)
        || provenance.edges.iter().any(|edge| {
            edge.left >= provenance.row_labels.len() || edge.right >= provenance.column_labels.len()
        })
    {
        return Err("packing provenance shape".into());
    }
    meter
        .vector(label_classes, "packing provenance classes")
        .map_err(|error| error.to_string())?;
    let mut classes = vec![Vec::new(); label_classes];
    for (edge_index, edge) in provenance.edges.iter().enumerate() {
        let class = provenance.row_labels[edge.left]
            .checked_mul(labels)
            .and_then(|value| value.checked_add(provenance.column_labels[edge.right]))
            .ok_or_else(|| "packing provenance class index".to_string())?;
        classes[class].push(edge_index);
        let left_group = digit(
            provenance.row_labels[edge.left],
            provenance.coordinate,
            provenance.radix,
        )
        .ok_or_else(|| "packing provenance group width".to_string())?;
        let right_group = digit(
            provenance.column_labels[edge.right],
            provenance.coordinate,
            provenance.radix,
        )
        .ok_or_else(|| "packing provenance group width".to_string())?;
        if factorization.left_groups.get(edge_index) != Some(&left_group)
            || factorization.right_groups.get(edge_index) != Some(&right_group)
        {
            return Err("packing provenance groups".into());
        }
    }

    let maximum = classes.iter().map(Vec::len).max().unwrap_or(0);
    let height = maximum
        .div_ceil(packet_length)
        .checked_mul(packet_length)
        .ok_or_else(|| "packing provenance height".to_string())?;
    let pairs: Vec<_> = (0..provenance.radix)
        .flat_map(|low| (low..provenance.radix).map(move |high| (low, high)))
        .collect();
    let orientation_patterns = pairs
        .len()
        .checked_pow(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance pattern width".to_string())?;
    meter
        .vector(orientation_patterns, "packing provenance patterns")
        .map_err(|error| error.to_string())?;

    let mut expected_packets = Vec::new();
    let mut expected_pairs = Vec::new();
    for mut pattern in 0..orientation_patterns {
        meter
            .charge(1, "packing provenance pattern")
            .map_err(|error| error.to_string())?;
        let mut chosen = Vec::with_capacity(provenance.packet_exponent as usize);
        for _ in 0..provenance.packet_exponent {
            chosen.push(pairs[pattern % pairs.len()]);
            pattern /= pairs.len();
        }
        let unequal = chosen.iter().filter(|&&(low, high)| low != high).count();
        let copies = 1usize
            .checked_shl(provenance.packet_exponent - unequal as u32)
            .ok_or_else(|| "packing provenance copy width".to_string())?;
        for start in (0..height).step_by(copies) {
            meter
                .vector(packet_length, "packing provenance packet slots")
                .map_err(|error| error.to_string())?;
            let mut slots = Vec::with_capacity(packet_length);
            for slot in 0..packet_length {
                meter
                    .charge(1, "packing provenance slot")
                    .map_err(|error| error.to_string())?;
                let (mut row, mut column, mut place, mut copy, mut copy_bit) =
                    (0usize, 0usize, 1usize, 0usize, 0usize);
                for (coordinate, &(low, high)) in chosen.iter().enumerate() {
                    let bit = (slot >> coordinate) & 1;
                    let (row_digit, column_digit) = if low == high {
                        copy |= bit << copy_bit;
                        copy_bit += 1;
                        (low, high)
                    } else if bit == 0 {
                        (low, high)
                    } else {
                        (high, low)
                    };
                    row = row
                        .checked_add(
                            row_digit
                                .checked_mul(place)
                                .ok_or_else(|| "packing provenance row label".to_string())?,
                        )
                        .ok_or_else(|| "packing provenance row label".to_string())?;
                    column = column
                        .checked_add(
                            column_digit
                                .checked_mul(place)
                                .ok_or_else(|| "packing provenance column label".to_string())?,
                        )
                        .ok_or_else(|| "packing provenance column label".to_string())?;
                    place = place
                        .checked_mul(provenance.radix)
                        .ok_or_else(|| "packing provenance label place".to_string())?;
                }
                let class = row
                    .checked_mul(labels)
                    .and_then(|value| value.checked_add(column))
                    .ok_or_else(|| "packing provenance slot class".to_string())?;
                slots.push(classes[class].get(start + copy).copied());
            }
            if slots.iter().any(Option::is_some) {
                meter
                    .vector(expected_packets.len() + 1, "packing provenance packets")
                    .map_err(|error| error.to_string())?;
                expected_packets.push(slots);
                expected_pairs.push(chosen.clone());
            }
        }
    }
    if expected_packets != factorization.packets || expected_pairs != provenance.packet_pairs {
        return Err("packing provenance replay".into());
    }
    let packing_lhs = BigUint::from(expected_packets.len()) * BigUint::from(packet_length);
    let packing_rhs = BigUint::from(3u8) * BigUint::from(provenance.edges.len());
    let source_capacity_admitted = packing_lhs <= packing_rhs;
    Ok(VerificationSummary {
        edges: provenance.edges.len(),
        left_vertices: provenance.row_labels.len(),
        right_vertices: provenance.column_labels.len(),
        packets: expected_packets.len(),
        packet_length,
        label_classes,
        orientation_patterns,
        packing_slack: source_capacity_admitted.then(|| packing_rhs - packing_lhs),
        source_capacity_admitted,
    })
}
