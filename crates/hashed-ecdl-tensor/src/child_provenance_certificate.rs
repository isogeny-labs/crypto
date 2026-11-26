//! Independent replay of finite group-child graph skeletons.
//!
//! The verifier consumes a checked packing-provenance object, its packet slots,
//! and a serialized child skeleton. It does not import graph, deformation,
//! packet-tensor, or contraction implementations.

use crate::packing_provenance_certificate::{
    Certificate as PackingCertificate, SCHEMA as PACKING_SCHEMA,
};
use crate::Meter;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Current child-provenance wire schema.
pub const SCHEMA: u32 = 1;

/// One edge of the serialized child graph skeleton.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChildEdge {
    /// Child old-vertex index.
    pub left_vertex: usize,
    /// Preserved parent port index.
    pub left_port: usize,
    /// Child packet-vertex index.
    pub right_vertex: usize,
    /// Port index within the child packet vertex.
    pub right_port: usize,
}

/// Finite group-child topology emitted by the producer-side control.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Selected radix group.
    pub group: usize,
    /// Degrees of retained old vertices in child order.
    pub old_vertex_degrees: Vec<usize>,
    /// Degrees of packet vertices in packet order.
    pub packet_vertex_degrees: Vec<usize>,
    /// Child edges in producer order.
    pub edges: Vec<ChildEdge>,
}

/// Checked dimensions of one accepted child-skeleton replay.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Selected radix group.
    pub group: usize,
    /// Number of retained old vertices, including selected isolates.
    pub old_vertices: usize,
    /// Number of packet vertices, including isolates.
    pub packet_vertices: usize,
    /// Number of child edges.
    pub edges: usize,
    /// Maximum child degree on either side.
    pub maximum_degree: usize,
}

fn digit(label: usize, coordinate: u32, radix: usize) -> Option<usize> {
    radix
        .checked_pow(coordinate)
        .map(|place| (label / place) % radix)
}

fn parent_ports(
    packing: &PackingCertificate,
) -> Result<(Vec<BTreeSet<usize>>, Vec<BTreeSet<usize>>), String> {
    let mut left = vec![BTreeSet::new(); packing.row_labels.len()];
    let mut right = vec![BTreeSet::new(); packing.column_labels.len()];
    for edge in &packing.edges {
        if edge.left >= left.len()
            || edge.right >= right.len()
            || !left[edge.left].insert(edge.left_port)
            || !right[edge.right].insert(edge.right_port)
        {
            return Err("child provenance parent ports".into());
        }
    }
    if left
        .iter()
        .chain(&right)
        .any(|ports| !ports.iter().copied().eq(0..ports.len()))
    {
        return Err("child provenance parent port order".into());
    }
    Ok((left, right))
}

/// Reconstruct one selected group child exactly from parent provenance.
///
/// Acceptance proves retained old-vertex order and degree, packet-vertex
/// replication including isolates, old-port preservation, packet ownership,
/// packet-port order, and child edge order. Tensor identity, coefficients, and
/// creation precision remain separate obligations.
pub fn verify(
    packing: &PackingCertificate,
    packets: &[Vec<Option<usize>>],
    child: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    if packing.schema != PACKING_SCHEMA
        || child.schema != SCHEMA
        || child.group >= packing.radix
        || packets
            .iter()
            .flatten()
            .flatten()
            .any(|&edge| edge >= packing.edges.len())
    {
        return Err("child provenance context".into());
    }
    let (left_ports, right_ports) = parent_ports(packing)?;
    let mut row_map = vec![None; packing.row_labels.len()];
    let mut column_map = vec![None; packing.column_labels.len()];
    let mut old_vertex_degrees = Vec::new();
    for (labels, ports, map) in [
        (&packing.row_labels, &left_ports, &mut row_map),
        (&packing.column_labels, &right_ports, &mut column_map),
    ] {
        for (vertex, (&label, ports)) in labels.iter().zip(ports).enumerate() {
            if digit(label, packing.coordinate, packing.radix)
                .ok_or_else(|| "child provenance group width".to_string())?
                == child.group
            {
                map[vertex] = Some(old_vertex_degrees.len());
                old_vertex_degrees.push(ports.len());
            }
        }
    }

    let mut packet_vertex_degrees = vec![0usize; packets.len()];
    let mut expected_edges = Vec::new();
    for (packet_id, packet) in packets.iter().enumerate() {
        for &edge_index in packet.iter().flatten() {
            let edge = &packing.edges[edge_index];
            if let Some(left_vertex) = row_map[edge.left] {
                expected_edges.push(ChildEdge {
                    left_vertex,
                    left_port: edge.left_port,
                    right_vertex: packet_id,
                    right_port: packet_vertex_degrees[packet_id],
                });
                packet_vertex_degrees[packet_id] += 1;
            }
        }
        for &edge_index in packet.iter().flatten() {
            let edge = &packing.edges[edge_index];
            if let Some(left_vertex) = column_map[edge.right] {
                expected_edges.push(ChildEdge {
                    left_vertex,
                    left_port: edge.right_port,
                    right_vertex: packet_id,
                    right_port: packet_vertex_degrees[packet_id],
                });
                packet_vertex_degrees[packet_id] += 1;
            }
        }
    }
    meter
        .vector(expected_edges.len(), "child provenance edges")
        .map_err(|error| error.to_string())?;
    if child.old_vertex_degrees != old_vertex_degrees
        || child.packet_vertex_degrees != packet_vertex_degrees
        || child.edges != expected_edges
    {
        return Err("child provenance replay".into());
    }
    let maximum_degree = old_vertex_degrees
        .iter()
        .chain(&packet_vertex_degrees)
        .copied()
        .max()
        .unwrap_or(0);
    Ok(VerificationSummary {
        group: child.group,
        old_vertices: old_vertex_degrees.len(),
        packet_vertices: packet_vertex_degrees.len(),
        edges: expected_edges.len(),
        maximum_degree,
    })
}
