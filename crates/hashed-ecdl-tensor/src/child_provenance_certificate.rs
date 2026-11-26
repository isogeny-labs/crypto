//! Independent replay of finite group-child graph skeletons.
//!
//! The verifier consumes checked packing and factorization objects plus a
//! serialized child. It does not import graph, deformation, packet-tensor, or
//! contraction implementations.

use crate::factorization_certificate::{
    Certificate as FactorizationCertificate, SCHEMA as FACTORIZATION_SCHEMA,
};
use crate::packing_provenance_certificate::{
    Certificate as PackingCertificate, SCHEMA as PACKING_SCHEMA,
};
use crate::Meter;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Current child-provenance wire schema.
pub const SCHEMA: u32 = 2;

/// Side of the parent bipartite graph supplying one retained old tensor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentSide {
    /// Parent left side.
    Row,
    /// Parent right side.
    Column,
}

/// Stable source reference for one retained old tensor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OldTensorSource {
    /// Parent bipartite side.
    pub side: ParentSide,
    /// Vertex index on that side.
    pub vertex: usize,
}

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
    /// Mode-exponent tuple used to construct the child packet tensors.
    pub beta: Vec<usize>,
    /// Degrees of retained old vertices in child order.
    pub old_vertex_degrees: Vec<usize>,
    /// Parent source of every retained old tensor in child order.
    pub old_tensor_sources: Vec<OldTensorSource>,
    /// Actual creation precision of every retained old tensor.
    pub old_creation_precisions: Vec<usize>,
    /// Degrees of packet vertices in packet order.
    pub packet_vertex_degrees: Vec<usize>,
    /// Actual creation precision of every packet tensor.
    pub packet_creation_precisions: Vec<usize>,
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
    /// Number of packet coefficient tables bound to this child.
    pub packet_entry_tables: usize,
    /// Number of independently checked packet-entry values bound to this child.
    pub packet_entry_values: usize,
}

/// Aggregate dimensions of a complete tuple/group child family.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct FamilyVerificationSummary {
    /// Number of retained mode tuples.
    pub tuples: usize,
    /// Number of radix groups per tuple.
    pub groups: usize,
    /// Exact number of verified children.
    pub children: usize,
    /// Total child incidences across the family.
    pub child_edges: usize,
    /// Largest edge count of any one child.
    pub maximum_child_edges: usize,
    /// Largest vertex degree of any one child.
    pub maximum_degree: usize,
    /// Total packet coefficient tables bound across all children.
    pub packet_entry_tables: usize,
    /// Total packet-entry values bound across all children.
    pub packet_entry_values: usize,
    /// Whether every checked child has at most one eighth of the parent edges.
    pub eightfold_edge_shrink_admitted: bool,
    /// Whether every child degree is at most the claimed source cap and `2L`.
    pub packet_degree_guard_admitted: bool,
    /// Whether the fixed radix-64 source premises and both strict guards hold.
    pub strict_source_refinement_admitted: bool,
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
/// replication including isolates, old-tensor sources and creation precision,
/// packet ownership and creation precision, packet coefficient-table keys,
/// packet-port order, and child edge order. The factorization certificate must
/// be verified separately to establish the contents of those coefficient
/// tables.
pub fn verify(
    packing: &PackingCertificate,
    factorization: &FactorizationCertificate,
    child: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    if packing.schema != PACKING_SCHEMA
        || child.schema != SCHEMA
        || child.group >= packing.radix
        || factorization.schema != FACTORIZATION_SCHEMA
        || factorization.radix != packing.radix
        || factorization.edge_count != packing.edges.len()
        || packing.row_creation_precisions.len() != packing.row_labels.len()
        || packing.column_creation_precisions.len() != packing.column_labels.len()
        || factorization
            .packets
            .iter()
            .flatten()
            .flatten()
            .any(|&edge| edge >= packing.edges.len())
        || !factorization.tuples.iter().any(|beta| beta == &child.beta)
    {
        return Err("child provenance context".into());
    }
    let (left_ports, right_ports) = parent_ports(packing)?;
    let mut row_map = vec![None; packing.row_labels.len()];
    let mut column_map = vec![None; packing.column_labels.len()];
    let mut old_vertex_degrees = Vec::new();
    let mut old_tensor_sources = Vec::new();
    let mut old_creation_precisions = Vec::new();
    for (side, labels, precisions, ports, map) in [
        (
            ParentSide::Row,
            &packing.row_labels,
            &packing.row_creation_precisions,
            &left_ports,
            &mut row_map,
        ),
        (
            ParentSide::Column,
            &packing.column_labels,
            &packing.column_creation_precisions,
            &right_ports,
            &mut column_map,
        ),
    ] {
        for (vertex, ((&label, &precision), ports)) in
            labels.iter().zip(precisions).zip(ports).enumerate()
        {
            if digit(label, packing.coordinate, packing.radix)
                .ok_or_else(|| "child provenance group width".to_string())?
                == child.group
            {
                map[vertex] = Some(old_vertex_degrees.len());
                old_vertex_degrees.push(ports.len());
                old_tensor_sources.push(OldTensorSource { side, vertex });
                old_creation_precisions.push(precision);
            }
        }
    }

    let mut packet_vertex_degrees = vec![0usize; factorization.packets.len()];
    let mut expected_edges = Vec::new();
    for (packet_id, packet) in factorization.packets.iter().enumerate() {
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
    let mut packet_entry_values = 0usize;
    for (packet, &degree) in packet_vertex_degrees.iter().enumerate() {
        let mut tables = factorization.entries.iter().filter(|table| {
            table.beta == child.beta && table.group == child.group && table.packet == packet
        });
        let table = tables
            .next()
            .ok_or_else(|| "child provenance packet table".to_string())?;
        if tables.next().is_some()
            || table.values.len()
                != 1usize
                    .checked_shl(
                        u32::try_from(degree)
                            .map_err(|_| "child provenance packet arity".to_string())?,
                    )
                    .ok_or_else(|| "child provenance packet arity".to_string())?
        {
            return Err("child provenance packet table".into());
        }
        packet_entry_values = packet_entry_values
            .checked_add(table.values.len())
            .ok_or_else(|| "child provenance packet values overflow".to_string())?;
    }
    if child.old_vertex_degrees != old_vertex_degrees
        || child.old_tensor_sources != old_tensor_sources
        || child.old_creation_precisions != old_creation_precisions
        || child.packet_vertex_degrees != packet_vertex_degrees
        || child.packet_creation_precisions.len() != packet_vertex_degrees.len()
        || child
            .packet_creation_precisions
            .iter()
            .any(|&precision| precision != factorization.precision)
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
        packet_entry_tables: packet_vertex_degrees.len(),
        packet_entry_values,
    })
}

/// Verify exact Cartesian coverage of every retained tuple and radix group.
///
/// Besides invoking [`verify`] on each child, this rejects duplicate or missing
/// `(beta, group)` keys and checks that each tuple has exactly two child
/// incidences per parent edge, one for each endpoint.
pub fn verify_family(
    packing: &PackingCertificate,
    factorization: &FactorizationCertificate,
    children: &[Certificate],
    meter: &mut Meter,
) -> Result<FamilyVerificationSummary, String> {
    let expected_children = factorization
        .tuples
        .len()
        .checked_mul(factorization.radix)
        .ok_or_else(|| "child provenance family coverage overflow".to_string())?;
    if children.len() != expected_children {
        return Err("child provenance family coverage".into());
    }
    meter
        .vector(children.len(), "child provenance family")
        .map_err(|error| error.to_string())?;
    let expected_keys: BTreeSet<_> = factorization
        .tuples
        .iter()
        .flat_map(|beta| (0..factorization.radix).map(move |group| (beta.clone(), group)))
        .collect();
    let mut actual_keys = BTreeSet::new();
    let mut edges_by_tuple = BTreeMap::new();
    let mut child_edges = 0usize;
    let mut maximum_child_edges = 0usize;
    let mut maximum_degree = 0usize;
    let mut packet_entry_tables = 0usize;
    let mut packet_entry_values = 0usize;
    for child in children {
        if !actual_keys.insert((child.beta.clone(), child.group)) {
            return Err("child provenance family duplicate".into());
        }
        let summary = verify(packing, factorization, child, meter)?;
        let tuple_edges = edges_by_tuple.entry(child.beta.clone()).or_insert(0usize);
        *tuple_edges = tuple_edges
            .checked_add(summary.edges)
            .ok_or_else(|| "child provenance family tuple edges overflow".to_string())?;
        child_edges = child_edges
            .checked_add(summary.edges)
            .ok_or_else(|| "child provenance family edges overflow".to_string())?;
        maximum_child_edges = maximum_child_edges.max(summary.edges);
        maximum_degree = maximum_degree.max(summary.maximum_degree);
        packet_entry_tables = packet_entry_tables
            .checked_add(summary.packet_entry_tables)
            .ok_or_else(|| "child provenance family tables overflow".to_string())?;
        packet_entry_values = packet_entry_values
            .checked_add(summary.packet_entry_values)
            .ok_or_else(|| "child provenance family values overflow".to_string())?;
    }
    let twice_edges = factorization
        .edge_count
        .checked_mul(2)
        .ok_or_else(|| "child provenance family edge target overflow".to_string())?;
    let expected_tables = expected_children
        .checked_mul(factorization.packets.len())
        .ok_or_else(|| "child provenance family table target overflow".to_string())?;
    if actual_keys != expected_keys
        || edges_by_tuple.len() != factorization.tuples.len()
        || edges_by_tuple.values().any(|&edges| edges != twice_edges)
        || packet_entry_tables != expected_tables
    {
        return Err("child provenance family coverage".into());
    }
    let eightfold_edge_shrink_admitted = maximum_child_edges
        .checked_mul(8)
        .is_some_and(|edges| edges <= factorization.edge_count);
    let packet_length = 1usize
        .checked_shl(packing.packet_exponent)
        .ok_or_else(|| "child provenance family packet width".to_string())?;
    let packet_degree_guard_admitted = packing.source_degree_cap.is_some_and(|delta| {
        packet_length
            .checked_mul(2)
            .is_some_and(|packet_degree| packet_degree <= delta && maximum_degree <= delta)
    });
    let strict_source_refinement_admitted = packing.source_degree_cap.is_some()
        && packing.radix == 64
        && eightfold_edge_shrink_admitted
        && packet_degree_guard_admitted;
    Ok(FamilyVerificationSummary {
        tuples: factorization.tuples.len(),
        groups: factorization.radix,
        children: children.len(),
        child_edges,
        maximum_child_edges,
        maximum_degree,
        packet_entry_tables,
        packet_entry_values,
        eightfold_edge_shrink_admitted,
        packet_degree_guard_admitted,
        strict_source_refinement_admitted,
    })
}
