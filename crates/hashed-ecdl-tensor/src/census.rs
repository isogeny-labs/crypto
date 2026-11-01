//! Exact boundary tensors and fraction-free flattening ranks.
//!
//! This module is the bounded instrument for the E1 boundary-rank census. It
//! enumerates only an explicitly capped side of a canonical Boolean tensor
//! network. Exhaustion is an error and is never reported as a zero tensor.

use crate::canonical::{CanonicalNetwork, PredicateProgram};
use crate::network::{Bit, Network, PredicateTag, Step};
use crate::{integrity, invalid, overflow, BigInt, BigUint, Meter, Result};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

/// Exact contraction of one side of a vertex cut.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryTensor {
    /// Canonically ordered edges crossing from the selected side to its complement.
    pub cut_edges: Vec<usize>,
    /// Canonically ordered edges internal to the selected side.
    pub internal_edges: Vec<usize>,
    /// Counts indexed by the little-endian assignment to `cut_edges`.
    pub values: Vec<BigUint>,
    /// Number of nonzero boundary assignments.
    pub support: usize,
    /// Complete assignments to free cut and internal edges that were examined.
    pub assignments_examined: u64,
}

/// Exact graph shape of one prospective boundary before assignment enumeration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundaryShape {
    /// Number of vertices on the selected side.
    pub selected_vertices: usize,
    /// Canonically ordered crossing edges.
    pub cut_edges: Vec<usize>,
    /// Canonically ordered edges internal to the selected side.
    pub internal_edges: Vec<usize>,
    /// Internal edges not fixed by a pin.
    pub free_internal_edges: Vec<usize>,
    /// Internal edges fixed by a pin.
    pub pinned_internal_edges: Vec<usize>,
    /// Number of Boolean edges enumerated by the current exact contractor.
    pub enumerated_free_width: usize,
}

/// Exact boundary tensor after quotienting cut-edge copies of encoder signals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionalBoundaryTensor {
    /// Every raw crossing edge in canonical construction order.
    pub raw_cut_edges: Vec<usize>,
    /// Raw crossing edges grouped by their exact encoder signal.
    pub cut_edge_classes: Vec<Vec<usize>>,
    /// Positions in the full free-input word that were enumerated.
    pub selected_free_input_positions: Vec<usize>,
    /// Exact tensor over one Boolean index per signal class.
    pub quotient: BoundaryTensor,
}

/// Exact raw-cut ranks computed from the signal-class quotient.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpandedRankSummary {
    /// Number of raw crossing edges before quotienting.
    pub raw_width: usize,
    /// Number of exact signal classes after quotienting.
    pub quotient_width: usize,
    /// Exact rank of the alternating raw-edge split.
    pub canonical_rank: usize,
    /// Minimum exact rank over all balanced raw-edge splits.
    pub best_balanced_rank: usize,
    /// Feasible rank-sufficient class patterns with at most one split class.
    pub balanced_class_patterns_examined: usize,
    /// Per-class state of the best split: 0 column-only, 1 split, 2 row-only.
    pub best_class_states: Vec<u8>,
    /// Fraction-free operations over the represented rank matrices.
    pub operations: RankOperations,
}

/// Exact receipt for a typed functional interior-rewiring draw.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FunctionalRewireReceipt {
    /// Checked functional definitions eligible for compatible pairing.
    pub candidate_definitions: usize,
    /// Exact-predicate groups containing at least one pair.
    pub groups_with_pairs: usize,
    /// Definition pairs whose complete active input bundles were exchanged.
    pub definition_pairs: usize,
    /// Pairs of graph edges whose consumer endpoints were exchanged.
    pub edge_endpoint_swaps: usize,
}

impl BoundaryTensor {
    /// Number of open Boolean indices in this boundary tensor.
    pub fn width(&self) -> usize {
        self.cut_edges.len()
    }

    /// Stable SHA-256 of the cut identifiers and exact count vector.
    pub fn sha256_hex(&self) -> Result<String> {
        fn put_len(hasher: &mut Sha256, value: usize) -> Result<()> {
            let value = u64::try_from(value).map_err(|_| overflow("boundary digest length"))?;
            hasher.update(value.to_le_bytes());
            Ok(())
        }
        let mut hasher = Sha256::new();
        hasher.update(b"hashed-ecdl-tensor boundary tensor v1\0");
        put_len(&mut hasher, self.cut_edges.len())?;
        for edge in &self.cut_edges {
            put_len(&mut hasher, *edge)?;
        }
        put_len(&mut hasher, self.values.len())?;
        for value in &self.values {
            let bytes = value.to_bytes_le();
            put_len(&mut hasher, bytes.len())?;
            hasher.update(bytes);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }
}

/// Counted integer operations used by exact rank elimination.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RankOperations {
    /// Big-integer additions or subtractions.
    pub additions: u64,
    /// Big-integer multiplications.
    pub multiplications: u64,
    /// Zero tests and pivot comparisons.
    pub comparisons: u64,
    /// Whole-row primitive normalizations.
    pub row_normalizations: u64,
}

/// Exact ranks of the canonical and best balanced flattenings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankSummary {
    /// Boundary width in Boolean indices.
    pub width: usize,
    /// Rank of the alternating canonical balanced split.
    pub canonical_rank: usize,
    /// Minimum exact rank over the admitted balanced splits.
    pub best_balanced_rank: usize,
    /// Number of distinct balanced splits examined.
    pub balanced_splits_examined: usize,
    /// Positions placed on the row side of the best split.
    pub best_row_positions: Vec<usize>,
    /// Fraction-free integer-operation counters over every examined split.
    pub operations: RankOperations,
}

fn checked_pow2(exponent: usize, phase: &str) -> Result<usize> {
    1usize
        .checked_shl(u32::try_from(exponent).map_err(|_| overflow(phase))?)
        .ok_or_else(|| overflow(phase))
}

/// Inspect a vertex cut exactly without enumerating any edge assignments.
///
/// The same validation and open-cut rules are used by [`contract_boundary`].
pub fn boundary_shape(
    network: &CanonicalNetwork,
    left_vertices: &[usize],
    meter: &mut Meter,
) -> Result<BoundaryShape> {
    network.validate(meter)?;
    if left_vertices.is_empty() || left_vertices.len() == network.vertices.len() {
        return Err(invalid("boundary side must be nonempty and proper"));
    }
    if left_vertices.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid("boundary vertex set must be sorted and unique"));
    }
    let mut selected = vec![false; network.vertices.len()];
    for &vertex in left_vertices {
        if vertex >= selected.len() || selected[vertex] {
            return Err(invalid("boundary vertex set"));
        }
        selected[vertex] = true;
    }

    let mut cut_edges = Vec::new();
    let mut internal_edges = Vec::new();
    for (edge, endpoints) in network.edges.iter().enumerate() {
        match (selected[endpoints.left], selected[endpoints.right]) {
            (true, true) => internal_edges.push(edge),
            (true, false) | (false, true) => cut_edges.push(edge),
            (false, false) => {}
        }
    }
    if cut_edges.is_empty() {
        return Err(invalid("boundary cut has no edges"));
    }
    if cut_edges.iter().any(|&edge| network.pins[edge].is_some()) {
        return Err(invalid("boundary cut edge is pinned"));
    }
    let (free_internal_edges, pinned_internal_edges): (Vec<_>, Vec<_>) = internal_edges
        .iter()
        .copied()
        .partition(|&edge| network.pins[edge].is_none());
    let enumerated_free_width = cut_edges
        .len()
        .checked_add(free_internal_edges.len())
        .ok_or_else(|| overflow("boundary free width"))?;
    Ok(BoundaryShape {
        selected_vertices: left_vertices.len(),
        cut_edges,
        internal_edges,
        free_internal_edges,
        pinned_internal_edges,
        enumerated_free_width,
    })
}

/// Contract a semantic encoder prefix by enumerating only its free source bits.
///
/// Every raw cut edge is mapped back to the exact signal recorded by the live
/// parsimonious encoder. Equality-tree copies therefore become one quotient
/// index. This path is available only on the original typed encoder network;
/// arbitrary or rewired predicate graphs have no trusted functional metadata.
pub fn contract_functional_boundary(
    network: &Network,
    left_vertices: &[usize],
    max_free_inputs: usize,
    max_quotient_width: usize,
    meter: &mut Meter,
) -> Result<FunctionalBoundaryTensor> {
    if left_vertices.is_empty() || left_vertices.len() == network.vertices.len() {
        return Err(invalid(
            "functional boundary side must be nonempty and proper",
        ));
    }
    if left_vertices.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid(
            "functional boundary vertex set must be sorted and unique",
        ));
    }
    let mut selected_vertices = vec![false; network.vertices.len()];
    for &vertex in left_vertices {
        if vertex >= selected_vertices.len() || selected_vertices[vertex] {
            return Err(invalid("functional boundary vertex set"));
        }
        selected_vertices[vertex] = true;
    }
    let mut raw_cut_edges = Vec::new();
    let mut internal_edges = Vec::new();
    for (edge, &(left, right)) in network.edges.iter().enumerate() {
        match (selected_vertices[left], selected_vertices[right]) {
            (true, true) => internal_edges.push(edge),
            (true, false) | (false, true) => raw_cut_edges.push(edge),
            (false, false) => {}
        }
    }
    if raw_cut_edges.is_empty() {
        return Err(invalid("functional boundary cut has no edges"));
    }
    let program = network
        .model
        .as_ref()
        .ok_or_else(|| invalid("functional boundary requires encoder metadata"))?;
    if program.signal_regions.len() != program.signals
        || program.edge_signals.len() != network.edges.len()
    {
        return Err(integrity("functional boundary metadata shape"));
    }
    let mut left_regions = BTreeSet::new();
    for &vertex in left_vertices {
        let tag = network.vertices[vertex]
            .predicate
            .canonical_tag()
            .ok_or_else(|| integrity("functional boundary vertex lacks tag"))?;
        let region = tag
            .parameters
            .iter()
            .find_map(|(key, value)| (key == "semantic_region").then_some(value))
            .ok_or_else(|| integrity("functional boundary vertex lacks region"))?;
        left_regions.insert(region.clone());
    }
    if program.signal_regions.iter().any(Option::is_none) {
        return Err(integrity("functional boundary signal lacks region"));
    }
    let selected_signals = program
        .signal_regions
        .iter()
        .map(|region| {
            region
                .as_ref()
                .is_some_and(|value| left_regions.contains(value))
        })
        .collect::<Vec<_>>();
    let selected_free_input_positions = program
        .free
        .iter()
        .enumerate()
        .filter_map(|(position, &signal)| selected_signals[signal].then_some(position))
        .collect::<Vec<_>>();
    if selected_free_input_positions.len() > max_free_inputs {
        return Err(crate::Error::Limit(
            "functional boundary free-input cap".into(),
        ));
    }

    let mut class_by_signal = BTreeMap::new();
    let mut class_signals = Vec::new();
    let mut cut_edge_classes: Vec<Vec<usize>> = Vec::new();
    for &edge in &raw_cut_edges {
        let signal = program.edge_signals[edge];
        let class = if let Some(&class) = class_by_signal.get(&signal) {
            class
        } else {
            let class = cut_edge_classes.len();
            class_by_signal.insert(signal, class);
            class_signals.push(signal);
            cut_edge_classes.push(Vec::new());
            class
        };
        cut_edge_classes[class].push(edge);
    }
    if cut_edge_classes.len() > max_quotient_width {
        return Err(crate::Error::Limit(
            "functional boundary quotient-width cap".into(),
        ));
    }
    if class_signals
        .iter()
        .any(|&signal| !selected_signals[signal])
    {
        return Err(integrity("functional cut signal is not on producer side"));
    }
    let assignment_count = checked_pow2(
        selected_free_input_positions.len(),
        "functional boundary assignments",
    )?;
    let value_count = checked_pow2(
        cut_edge_classes.len(),
        "functional boundary quotient entries",
    )?;
    meter.vector(assignment_count, "functional boundary assignments")?;
    meter.vector(value_count, "functional boundary quotient entries")?;
    let mut values = vec![BigUint::zero(); value_count];
    let mut free_values = vec![false; program.free.len()];
    for mask in 0..assignment_count {
        meter.charge(1, "functional boundary assignments")?;
        for (digit, &position) in selected_free_input_positions.iter().enumerate() {
            free_values[position] = mask & (1usize << digit) != 0;
        }
        let Some(signal_values) = program.evaluate_selected(
            &free_values,
            &selected_signals,
            &selected_vertices,
            network,
            meter,
        )?
        else {
            continue;
        };
        let mut quotient_assignment = 0usize;
        for (class, &signal) in class_signals.iter().enumerate() {
            if signal_values[signal] {
                quotient_assignment |= 1usize << class;
            }
        }
        values[quotient_assignment] += BigUint::one();
    }
    let support = values.iter().filter(|value| !value.is_zero()).count();
    Ok(FunctionalBoundaryTensor {
        raw_cut_edges,
        cut_edge_classes: cut_edge_classes.clone(),
        selected_free_input_positions,
        quotient: BoundaryTensor {
            cut_edges: cut_edge_classes.iter().map(|class| class[0]).collect(),
            internal_edges,
            values,
            support,
            assignments_examined: u64::try_from(assignment_count)
                .map_err(|_| overflow("functional boundary assignment counter"))?,
        },
    })
}

/// Rewire complete input bundles between identical typed definitions on one
/// semantic side while preserving the graph's local predicates and degrees.
///
/// Each exchange is a sequence of degree-preserving double-edge swaps between
/// consumer incidences. Output incidences, interface edges, cut edges and the
/// equality-tree producer endpoints remain fixed. The parsimonious program is
/// updated to the new input signals and every later contraction replays the
/// physically rewired graph, so stale functional metadata fails rather than
/// silently defining a different object.
pub fn rewire_functional_definitions(
    source: &Network,
    left_vertices: &[usize],
    seed: u64,
    meter: &mut Meter,
) -> Result<(Network, FunctionalRewireReceipt)> {
    rewire_functional_definitions_capped(source, left_vertices, seed, usize::MAX, meter)
}

/// Apply the same typed functional rewiring with an exact global pair cap.
/// The first admissible pairs in seeded order are retained; a zero cap is
/// invalid and no result is returned unless at least one pair changes.
pub fn rewire_functional_definitions_capped(
    source: &Network,
    left_vertices: &[usize],
    seed: u64,
    maximum_definition_pairs: usize,
    meter: &mut Meter,
) -> Result<(Network, FunctionalRewireReceipt)> {
    #[derive(Clone, Copy)]
    struct Candidate {
        define_step: usize,
        vertex: usize,
        input_ports: usize,
    }
    struct Group {
        tag: PredicateTag,
        program: PredicateProgram,
        input_ports: usize,
        candidates: Vec<Candidate>,
    }
    fn other_endpoint(edge: (usize, usize), vertex: usize) -> Result<usize> {
        match edge {
            (left, right) if left == vertex => Ok(right),
            (left, right) if right == vertex => Ok(left),
            _ => Err(integrity("rewiring edge absent from consumer")),
        }
    }
    fn replace_endpoint(edge: &mut (usize, usize), old: usize, new: usize) -> Result<()> {
        if edge.0 == old {
            edge.0 = new;
        } else if edge.1 == old {
            edge.1 = new;
        } else {
            return Err(integrity("rewiring endpoint replacement"));
        }
        if edge.0 == edge.1 {
            return Err(integrity("rewiring produced a self-loop"));
        }
        Ok(())
    }

    if maximum_definition_pairs == 0 {
        return Err(invalid("zero typed functional rewiring pair cap"));
    }
    if left_vertices.is_empty() || left_vertices.len() == source.vertices.len() {
        return Err(invalid(
            "functional rewiring side must be nonempty and proper",
        ));
    }
    if left_vertices.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(invalid(
            "functional rewiring vertex set must be sorted and unique",
        ));
    }
    let mut selected = vec![false; source.vertices.len()];
    for &vertex in left_vertices {
        if vertex >= selected.len() || selected[vertex] {
            return Err(invalid("functional rewiring vertex set"));
        }
        selected[vertex] = true;
    }
    let source_program = source
        .model
        .as_ref()
        .ok_or_else(|| invalid("functional rewiring requires encoder metadata"))?;
    let mut signal_ready_step = vec![usize::MAX; source_program.signals];
    for &signal in &source_program.free {
        signal_ready_step[signal] = 0;
    }
    for (step, value) in source_program.steps.iter().enumerate() {
        if let Step::Define { outputs, .. } = value {
            for bit in outputs.iter().flat_map(|word| &word.0) {
                let Bit::Signal(signal) = bit else {
                    return Err(integrity("functional definition output is constant"));
                };
                signal_ready_step[*signal] = step + 1;
            }
        }
    }
    let interface = source.interface.iter().copied().collect::<BTreeSet<_>>();
    let mut groups: Vec<Group> = Vec::new();
    for define_step in 0..source_program.steps.len().saturating_sub(1) {
        let Step::Define {
            inputs, outputs, ..
        } = &source_program.steps[define_step]
        else {
            continue;
        };
        let Step::Check { vertex, ports } = &source_program.steps[define_step + 1] else {
            continue;
        };
        if !selected[*vertex] {
            continue;
        }
        let input_ports = inputs
            .iter()
            .flat_map(|word| &word.0)
            .filter(|bit| matches!(bit, Bit::Signal(_)))
            .count();
        let output_ports = outputs
            .iter()
            .flat_map(|word| &word.0)
            .filter(|bit| matches!(bit, Bit::Signal(_)))
            .count();
        if input_ports == 0
            || input_ports + output_ports != ports.len()
            || ports.len() != source.vertices[*vertex].edges.len()
        {
            continue;
        }
        let predicate = &source.vertices[*vertex].predicate;
        let Some(tag) = predicate.canonical_tag().cloned() else {
            continue;
        };
        let Some(program) = predicate.canonical_program().cloned() else {
            continue;
        };
        if source.vertices[*vertex].edges[..input_ports]
            .iter()
            .any(|edge| {
                interface.contains(edge)
                    || source.edges[*edge].0 >= selected.len()
                    || source.edges[*edge].1 >= selected.len()
                    || !selected[source.edges[*edge].0]
                    || !selected[source.edges[*edge].1]
            })
        {
            continue;
        }
        let candidate = Candidate {
            define_step,
            vertex: *vertex,
            input_ports,
        };
        if let Some(group) = groups.iter_mut().find(|group| {
            group.input_ports == input_ports && group.tag == tag && group.program == program
        }) {
            group.candidates.push(candidate);
        } else {
            groups.push(Group {
                tag,
                program,
                input_ports,
                candidates: vec![candidate],
            });
        }
    }
    let candidate_definitions = groups.iter().map(|group| group.candidates.len()).sum();
    let groups_with_pairs = groups
        .iter()
        .filter(|group| group.candidates.len() >= 2)
        .count();
    let mut network = source.clone();
    let mut definition_pairs = 0usize;
    let mut edge_endpoint_swaps = 0usize;
    'groups: for group in &mut groups {
        group.candidates.sort_by_key(|candidate| {
            let mut hasher = Sha256::new();
            hasher.update(b"typed functional rewiring v1\0");
            hasher.update(seed.to_le_bytes());
            hasher.update((candidate.vertex as u64).to_le_bytes());
            <[u8; 32]>::from(hasher.finalize())
        });
        for pair in group.candidates.chunks_exact(2) {
            if definition_pairs == maximum_definition_pairs {
                break 'groups;
            }
            let left = pair[0];
            let right = pair[1];
            let mut swaps = Vec::with_capacity(left.input_ports);
            let mut admissible = true;
            let earliest_step = left.define_step.min(right.define_step);
            for port in 0..left.input_ports {
                let left_edge = network.vertices[left.vertex].edges[port];
                let right_edge = network.vertices[right.vertex].edges[port];
                if left_edge == right_edge
                    || interface.contains(&left_edge)
                    || interface.contains(&right_edge)
                {
                    admissible = false;
                    break;
                }
                let left_other = other_endpoint(network.edges[left_edge], left.vertex)?;
                let right_other = other_endpoint(network.edges[right_edge], right.vertex)?;
                let left_signal = source_program.edge_signals[left_edge];
                let right_signal = source_program.edge_signals[right_edge];
                if left_other == right.vertex
                    || right_other == left.vertex
                    || !selected[left_other]
                    || !selected[right_other]
                    || signal_ready_step[left_signal] > earliest_step
                    || signal_ready_step[right_signal] > earliest_step
                {
                    admissible = false;
                    break;
                }
                swaps.push((port, left_edge, right_edge));
            }
            if !admissible {
                continue;
            }
            for (port, left_edge, right_edge) in swaps {
                replace_endpoint(&mut network.edges[left_edge], left.vertex, right.vertex)?;
                replace_endpoint(&mut network.edges[right_edge], right.vertex, left.vertex)?;
                network.vertices[left.vertex].edges[port] = right_edge;
                network.vertices[right.vertex].edges[port] = left_edge;
                edge_endpoint_swaps += 1;
            }
            definition_pairs += 1;
        }
    }
    if definition_pairs == 0 || edge_endpoint_swaps == 0 {
        return Err(invalid("no admissible typed functional rewiring"));
    }
    meter.charge(
        u64::try_from(edge_endpoint_swaps).map_err(|_| overflow("rewiring swap count"))?,
        "typed functional edge rewiring",
    )?;

    let edge_signals = source_program.edge_signals.clone();
    let define_updates = groups
        .iter()
        .flat_map(|group| &group.candidates)
        .map(|candidate| {
            let signals = network.vertices[candidate.vertex].edges[..candidate.input_ports]
                .iter()
                .map(|&edge| edge_signals[edge])
                .collect::<Vec<_>>();
            (candidate.define_step, signals)
        })
        .collect::<Vec<_>>();
    let check_updates = source_program
        .steps
        .iter()
        .enumerate()
        .filter_map(|(step, value)| match value {
            Step::Check { vertex, .. } => Some((
                step,
                network.vertices[*vertex]
                    .edges
                    .iter()
                    .map(|&edge| Bit::Signal(edge_signals[edge]))
                    .collect::<Vec<_>>(),
            )),
            Step::Define { .. } => None,
        })
        .collect::<Vec<_>>();
    let program = Arc::make_mut(
        network
            .model
            .as_mut()
            .ok_or_else(|| integrity("rewired functional metadata disappeared"))?,
    );
    for (step, signals) in define_updates {
        let Step::Define { inputs, .. } = &mut program.steps[step] else {
            return Err(integrity("rewiring define-step identity"));
        };
        let mut signals = signals.into_iter();
        for bit in inputs.iter_mut().flat_map(|word| &mut word.0) {
            if matches!(bit, Bit::Signal(_)) {
                *bit = Bit::Signal(
                    signals
                        .next()
                        .ok_or_else(|| integrity("rewiring input signal underflow"))?,
                );
            }
        }
        if signals.next().is_some() {
            return Err(integrity("rewiring input signal overflow"));
        }
    }
    for (step, updated_ports) in check_updates {
        let Step::Check { ports, .. } = &mut program.steps[step] else {
            return Err(integrity("rewiring check-step identity"));
        };
        *ports = updated_ports;
    }
    Ok((
        network,
        FunctionalRewireReceipt {
            candidate_definitions,
            groups_with_pairs,
            definition_pairs,
            edge_endpoint_swaps,
        },
    ))
}

/// Contract exactly the predicates on `left_vertices`, leaving cut edges open.
///
/// `max_free_edges` caps the combined number of free cut and left-internal
/// edges. Cut pins are rejected because an E1 boundary must remain open.
pub fn contract_boundary(
    network: &CanonicalNetwork,
    left_vertices: &[usize],
    max_free_edges: usize,
    meter: &mut Meter,
) -> Result<BoundaryTensor> {
    let shape = boundary_shape(network, left_vertices, meter)?;
    let cut_edges = shape.cut_edges;
    let internal_edges = shape.internal_edges;
    let free_internal = shape.free_internal_edges;
    let free_width = shape.enumerated_free_width;
    let mut selected = vec![false; network.vertices.len()];
    for &vertex in left_vertices {
        selected[vertex] = true;
    }
    if free_width > max_free_edges {
        return Err(crate::Error::Limit("boundary free-edge cap".into()));
    }
    let assignment_count = checked_pow2(free_width, "boundary assignments")?;
    meter.vector(assignment_count, "boundary assignment cap")?;
    let value_count = checked_pow2(cut_edges.len(), "boundary tensor entries")?;
    meter.vector(value_count, "boundary tensor entries")?;
    let mut values = vec![BigUint::zero(); value_count];
    let mut assignment = network.pins.clone();
    let free_edges: Vec<_> = cut_edges.iter().chain(&free_internal).copied().collect();

    for mask in 0..assignment_count {
        meter.charge(1, "boundary assignments")?;
        for (position, &edge) in free_edges.iter().enumerate() {
            assignment[edge] = Some(mask & (1usize << position) != 0);
        }
        let mut accepted = true;
        for (vertex_id, vertex) in network.vertices.iter().enumerate() {
            if !selected[vertex_id] {
                continue;
            }
            let ports: Vec<_> = vertex
                .edges
                .iter()
                .map(|&edge| {
                    assignment[edge].ok_or_else(|| integrity("unassigned left predicate edge"))
                })
                .collect::<Result<_>>()?;
            if !vertex.predicate.evaluate(&ports, meter)? {
                accepted = false;
                break;
            }
        }
        if accepted {
            let boundary_mask = mask & (value_count - 1);
            values[boundary_mask] += BigUint::one();
        }
    }
    let support = values.iter().filter(|value| !value.is_zero()).count();
    Ok(BoundaryTensor {
        cut_edges,
        internal_edges,
        values,
        support,
        assignments_examined: u64::try_from(assignment_count)
            .map_err(|_| overflow("boundary assignment counter"))?,
    })
}

fn exact_rank(mut matrix: Vec<Vec<BigInt>>, operations: &mut RankOperations) -> Result<usize> {
    if matrix.is_empty() {
        return Ok(0);
    }
    let columns = matrix[0].len();
    if matrix.iter().any(|row| row.len() != columns) {
        return Err(invalid("ragged rank matrix"));
    }
    let mut pivot_row = 0usize;
    for column in 0..columns {
        if pivot_row == matrix.len() {
            break;
        }
        let mut pivot = None;
        for row in pivot_row..matrix.len() {
            operations.comparisons += 1;
            if !matrix[row][column].is_zero() {
                pivot = Some(row);
                break;
            }
        }
        let Some(pivot) = pivot else { continue };
        matrix.swap(pivot_row, pivot);
        let pivot_value = matrix[pivot_row][column].clone();
        for row in pivot_row + 1..matrix.len() {
            operations.comparisons += 1;
            if matrix[row][column].is_zero() {
                continue;
            }
            let factor = matrix[row][column].clone();
            for current in column..columns {
                let left = &pivot_value * &matrix[row][current];
                let right = &factor * &matrix[pivot_row][current];
                operations.multiplications += 2;
                operations.additions += 1;
                matrix[row][current] = left - right;
            }
            let divisor = matrix[row][column + 1..]
                .iter()
                .fold(BigInt::zero(), |gcd, value| gcd.gcd(&value.abs()));
            if divisor > BigInt::one() {
                for value in &mut matrix[row][column + 1..] {
                    *value /= &divisor;
                }
                operations.row_normalizations += 1;
            }
            matrix[row][column] = BigInt::zero();
        }
        pivot_row += 1;
    }
    Ok(pivot_row)
}

fn split_rank(
    tensor: &BoundaryTensor,
    row_positions: &[usize],
    operations: &mut RankOperations,
    meter: &Meter,
) -> Result<usize> {
    let row_set: BTreeSet<_> = row_positions.iter().copied().collect();
    if row_set.len() != row_positions.len()
        || row_positions
            .iter()
            .any(|&position| position >= tensor.width())
    {
        return Err(invalid("flattening row positions"));
    }
    let column_positions: Vec<_> = (0..tensor.width())
        .filter(|position| !row_set.contains(position))
        .collect();
    let rows = checked_pow2(row_positions.len(), "flattening rows")?;
    let columns = checked_pow2(column_positions.len(), "flattening columns")?;
    meter.vector(
        rows.checked_mul(columns)
            .ok_or_else(|| overflow("flattening entries"))?,
        "flattening entries",
    )?;
    let mut matrix = vec![vec![BigInt::zero(); columns]; rows];
    for (assignment, value) in tensor.values.iter().enumerate() {
        let mut row = 0usize;
        let mut column = 0usize;
        for (output, &input) in row_positions.iter().enumerate() {
            if assignment & (1usize << input) != 0 {
                row |= 1usize << output;
            }
        }
        for (output, &input) in column_positions.iter().enumerate() {
            if assignment & (1usize << input) != 0 {
                column |= 1usize << output;
            }
        }
        matrix[row][column] = BigInt::from(value.clone());
    }
    exact_rank(matrix, operations)
}

fn quotient_pattern_rank(
    tensor: &FunctionalBoundaryTensor,
    states: &[u8],
    operations: &mut RankOperations,
    meter: &Meter,
) -> Result<usize> {
    if states.len() != tensor.cut_edge_classes.len() || states.iter().any(|&state| state > 2) {
        return Err(invalid("quotient rank class states"));
    }
    let coordinates = tensor
        .quotient
        .values
        .iter()
        .enumerate()
        .filter(|(_, value)| !value.is_zero())
        .map(|(assignment, value)| {
            let mut row = 0usize;
            let mut column = 0usize;
            let mut row_position = 0usize;
            let mut column_position = 0usize;
            for (class, &state) in states.iter().enumerate() {
                let bit = assignment & (1usize << class) != 0;
                if state != 0 {
                    if bit {
                        row |= 1usize << row_position;
                    }
                    row_position += 1;
                }
                if state != 2 {
                    if bit {
                        column |= 1usize << column_position;
                    }
                    column_position += 1;
                }
            }
            (row, column, value)
        })
        .collect::<Vec<_>>();
    if coordinates.is_empty() {
        return Ok(0);
    }
    let row_keys = coordinates
        .iter()
        .map(|entry| entry.0)
        .collect::<BTreeSet<_>>();
    let column_keys = coordinates
        .iter()
        .map(|entry| entry.1)
        .collect::<BTreeSet<_>>();
    let row_index = row_keys
        .into_iter()
        .enumerate()
        .map(|(index, key)| (key, index))
        .collect::<BTreeMap<_, _>>();
    let column_index = column_keys
        .into_iter()
        .enumerate()
        .map(|(index, key)| (key, index))
        .collect::<BTreeMap<_, _>>();
    let rows = row_index.len();
    let columns = column_index.len();
    meter.vector(
        rows.checked_mul(columns)
            .ok_or_else(|| overflow("quotient flattening entries"))?,
        "quotient flattening entries",
    )?;
    let mut matrix = vec![vec![BigInt::zero(); columns]; rows];
    for (row_key, column_key, value) in coordinates {
        let row = row_index[&row_key];
        let column = column_index[&column_key];
        if !matrix[row][column].is_zero() {
            return Err(integrity("quotient assignments collide in flattening"));
        }
        matrix[row][column] = BigInt::from(value.clone());
    }
    exact_rank(matrix, operations)
}

fn balanced_class_pattern(class_sizes: &[usize], states: &[u8], row_width: usize) -> bool {
    let mut minimum = 0usize;
    let mut maximum = 0usize;
    for (&size, &state) in class_sizes.iter().zip(states) {
        match state {
            0 => {}
            1 if size >= 2 => {
                minimum += 1;
                maximum += size - 1;
            }
            1 => return false,
            2 => {
                minimum += size;
                maximum += size;
            }
            _ => return false,
        }
    }
    minimum <= row_width && row_width <= maximum
}

/// Compute exact ranks of the expanded raw cut without materializing its
/// exponentially repeated equality-copy dimensions.
///
/// A raw split is rank-equivalent to one of three states per signal class:
/// column-only, split across both sides, or row-only. Splitting a class makes
/// its matrix block diagonal; moving it wholly to either side cannot increase
/// rank. Row edges can be transferred between two split classes until one is
/// whole while preserving balance. Repeating proves that an exact minimum is
/// attained with at most one split class, which is the finite set enumerated.
pub fn expanded_rank_summary(
    tensor: &FunctionalBoundaryTensor,
    meter: &mut Meter,
) -> Result<ExpandedRankSummary> {
    let quotient_width = tensor.cut_edge_classes.len();
    if tensor.quotient.width() != quotient_width
        || tensor.quotient.values.len() != checked_pow2(quotient_width, "quotient tensor shape")?
        || tensor.cut_edge_classes.iter().any(Vec::is_empty)
    {
        return Err(invalid("functional quotient tensor shape"));
    }
    let raw_width = tensor.raw_cut_edges.len();
    let class_sizes = tensor
        .cut_edge_classes
        .iter()
        .map(Vec::len)
        .collect::<Vec<_>>();
    if class_sizes.iter().sum::<usize>() != raw_width {
        return Err(invalid("functional quotient class coverage"));
    }
    let row_width = raw_width / 2;
    let raw_positions = tensor
        .raw_cut_edges
        .iter()
        .enumerate()
        .map(|(position, &edge)| (edge, position))
        .collect::<BTreeMap<_, _>>();
    let mut canonical_states = Vec::with_capacity(quotient_width);
    for class in &tensor.cut_edge_classes {
        let row_count = class
            .iter()
            .filter(|edge| {
                raw_positions
                    .get(edge)
                    .is_some_and(|&position| position % 2 == 0 && position / 2 < row_width)
            })
            .count();
        canonical_states.push(if row_count == 0 {
            0
        } else if row_count == class.len() {
            2
        } else {
            1
        });
    }
    if !balanced_class_pattern(&class_sizes, &canonical_states, row_width) {
        return Err(integrity("canonical quotient split is not balanced"));
    }
    let mut operations = RankOperations::default();
    let canonical_rank = quotient_pattern_rank(tensor, &canonical_states, &mut operations, meter)?;

    let no_mixed_patterns = checked_pow2(quotient_width, "quotient class patterns")?;
    let one_mixed_patterns = if quotient_width == 0 {
        0
    } else {
        quotient_width
            .checked_mul(checked_pow2(
                quotient_width - 1,
                "mixed quotient class patterns",
            )?)
            .ok_or_else(|| overflow("mixed quotient class patterns"))?
    };
    meter.vector(
        no_mixed_patterns
            .checked_add(one_mixed_patterns)
            .ok_or_else(|| overflow("rank-sufficient class patterns"))?,
        "rank-sufficient quotient class patterns",
    )?;
    let mut feasible = 0usize;
    let mut best: Option<(usize, Vec<u8>)> = None;
    for mixed_class in 0..=quotient_width {
        let binary_width = quotient_width - usize::from(mixed_class < quotient_width);
        let pattern_count = checked_pow2(binary_width, "binary quotient class patterns")?;
        for encoded in 0..pattern_count {
            let mut binary_position = 0usize;
            let states = (0..quotient_width)
                .map(|class| {
                    if class == mixed_class {
                        1
                    } else {
                        let state = if encoded & (1usize << binary_position) == 0 {
                            0
                        } else {
                            2
                        };
                        binary_position += 1;
                        state
                    }
                })
                .collect::<Vec<_>>();
            if !balanced_class_pattern(&class_sizes, &states, row_width) {
                continue;
            }
            feasible += 1;
            meter.charge(1, "balanced rank-sufficient class pattern")?;
            let rank = quotient_pattern_rank(tensor, &states, &mut operations, meter)?;
            if best.as_ref().map_or(true, |(old, _)| rank < *old) {
                best = Some((rank, states));
            }
        }
    }
    let (best_balanced_rank, best_class_states) =
        best.ok_or_else(|| integrity("no balanced quotient class pattern"))?;
    Ok(ExpandedRankSummary {
        raw_width,
        quotient_width,
        canonical_rank,
        best_balanced_rank,
        balanced_class_patterns_examined: feasible,
        best_class_states,
        operations,
    })
}

fn balanced_splits(width: usize, exhaustive_width: usize) -> Result<Vec<Vec<usize>>> {
    let row_width = width / 2;
    let canonical: Vec<_> = (0..width).step_by(2).take(row_width).collect();
    if width > exhaustive_width || width <= 1 {
        return Ok(vec![canonical]);
    }
    let masks = checked_pow2(width, "balanced split masks")?;
    let mut splits = Vec::new();
    for mask in 0..masks {
        if mask.count_ones() as usize != row_width {
            continue;
        }
        if width % 2 == 0 && row_width > 0 && mask & 1 == 0 {
            continue;
        }
        splits.push(
            (0..width)
                .filter(|position| mask & (1usize << position) != 0)
                .collect(),
        );
    }
    if !splits.contains(&canonical) {
        splits.push(canonical);
    }
    splits.sort();
    splits.dedup();
    Ok(splits)
}

/// Compute the canonical and minimum admitted exact balanced flattening ranks.
pub fn rank_summary(
    tensor: &BoundaryTensor,
    exhaustive_width: usize,
    meter: &mut Meter,
) -> Result<RankSummary> {
    if tensor.values.len() != checked_pow2(tensor.width(), "boundary tensor shape")? {
        return Err(invalid("boundary tensor shape"));
    }
    let canonical: Vec<_> = (0..tensor.width())
        .step_by(2)
        .take(tensor.width() / 2)
        .collect();
    let splits = balanced_splits(tensor.width(), exhaustive_width)?;
    meter.vector(splits.len(), "balanced split list")?;
    let mut operations = RankOperations::default();
    let mut canonical_rank = None;
    let mut best: Option<(usize, Vec<usize>)> = None;
    for split in splits {
        meter.charge(1, "balanced flattening")?;
        let rank = split_rank(tensor, &split, &mut operations, meter)?;
        if split == canonical {
            canonical_rank = Some(rank);
        }
        if best.as_ref().map_or(true, |(old, _)| rank < *old) {
            best = Some((rank, split));
        }
    }
    let (best_balanced_rank, best_row_positions) =
        best.ok_or_else(|| integrity("no balanced flattening"))?;
    Ok(RankSummary {
        width: tensor.width(),
        canonical_rank: canonical_rank.ok_or_else(|| integrity("canonical split absent"))?,
        best_balanced_rank,
        balanced_splits_examined: balanced_splits(tensor.width(), exhaustive_width)?.len(),
        best_row_positions,
        operations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical::{CanonicalNetwork, ExportPolicy, IrProvenance};
    use crate::encoding::Encoder;
    use crate::network::{FnPredicate, GraphBuilder, PredicateTag};
    use crate::Limits;
    use num_rational::Ratio;

    fn work() -> Meter {
        Meter::new(Limits {
            steps: 10_000_000,
            vector: 1_000_000,
            bits: 1_000_000,
        })
    }

    fn tensor_from_matrix(matrix: &[&[u32]]) -> BoundaryTensor {
        let rows = matrix.len();
        let columns = matrix[0].len();
        assert!(rows.is_power_of_two() && columns.is_power_of_two());
        let row_bits = rows.ilog2() as usize;
        let column_bits = columns.ilog2() as usize;
        let width = row_bits + column_bits;
        let mut values = vec![BigUint::zero(); 1usize << width];
        for (row, entries) in matrix.iter().enumerate() {
            for (column, value) in entries.iter().enumerate() {
                values[row | (column << row_bits)] = BigUint::from(*value);
            }
        }
        BoundaryTensor {
            cut_edges: (0..width).collect(),
            internal_edges: vec![],
            support: values.iter().filter(|value| !value.is_zero()).count(),
            values,
            assignments_examined: 0,
        }
    }

    fn rational_rank(matrix: &[Vec<BigInt>]) -> usize {
        let mut matrix: Vec<Vec<Ratio<BigInt>>> = matrix
            .iter()
            .map(|row| row.iter().cloned().map(Ratio::from_integer).collect())
            .collect();
        let mut pivot_row = 0usize;
        for column in 0..matrix[0].len() {
            let Some(pivot) = (pivot_row..matrix.len()).find(|&row| !matrix[row][column].is_zero())
            else {
                continue;
            };
            matrix.swap(pivot_row, pivot);
            let pivot_value = matrix[pivot_row][column].clone();
            for current in column..matrix[0].len() {
                matrix[pivot_row][current] /= &pivot_value;
            }
            for row in 0..matrix.len() {
                if row == pivot_row {
                    continue;
                }
                let factor = matrix[row][column].clone();
                for current in column..matrix[0].len() {
                    let subtract = &factor * &matrix[pivot_row][current];
                    matrix[row][current] -= subtract;
                }
            }
            pivot_row += 1;
            if pivot_row == matrix.len() {
                break;
            }
        }
        pivot_row
    }

    #[test]
    fn fraction_free_rank_controls_are_exact() {
        let mut operations = RankOperations::default();
        assert_eq!(
            exact_rank(
                vec![vec![1.into(), 2.into()], vec![2.into(), 4.into()]],
                &mut operations
            )
            .unwrap(),
            1
        );
        assert_eq!(
            exact_rank(
                vec![vec![1.into(), 0.into()], vec![0.into(), 1.into()]],
                &mut operations
            )
            .unwrap(),
            2
        );
        let rank_four =
            tensor_from_matrix(&[&[1, 0, 0, 0], &[0, 1, 0, 0], &[0, 0, 1, 0], &[0, 0, 0, 1]]);
        assert_eq!(
            split_rank(&rank_four, &[0, 1], &mut operations, &work()).unwrap(),
            4
        );
    }

    #[test]
    fn fraction_free_rank_matches_rational_elimination() {
        let mut state = 0x243f_6a88_85a3_08d3u64;
        for rows in 1..=6 {
            for columns in 1..=6 {
                for _case in 0..32 {
                    let matrix: Vec<Vec<BigInt>> = (0..rows)
                        .map(|_| {
                            (0..columns)
                                .map(|_| {
                                    state ^= state << 13;
                                    state ^= state >> 7;
                                    state ^= state << 17;
                                    BigInt::from((state % 11) as i64 - 5)
                                })
                                .collect()
                        })
                        .collect();
                    let expected = rational_rank(&matrix);
                    let actual = exact_rank(matrix, &mut RankOperations::default()).unwrap();
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn canonical_network_boundary_is_identity_rank_two() {
        let mut meter = work();
        let mut builder = GraphBuilder::new();
        let equality = builder.vertex(FnPredicate::tagged(
            2,
            PredicateTag::new("control.equality", vec![]),
            |ports, _| Ok(ports[0] == ports[1]),
        ));
        let left_endpoint = builder.vertex(FnPredicate::tagged(
            1,
            PredicateTag::new("control.endpoint", vec![]),
            |_, _| Ok(true),
        ));
        let right_endpoint = builder.vertex(FnPredicate::tagged(
            1,
            PredicateTag::new("control.endpoint", vec![]),
            |_, _| Ok(true),
        ));
        builder.edge(equality, left_endpoint, &mut meter).unwrap();
        builder.edge(equality, right_endpoint, &mut meter).unwrap();
        let graph = builder.finish(vec![]).unwrap();
        let snapshot = CanonicalNetwork::capture(
            &graph,
            &vec![None; graph.edge_count()],
            IrProvenance {
                source: "rank-control".into(),
                generator: "unit-test".into(),
                revision: "0000000000000000000000000000000000000000".into(),
            },
            ExportPolicy::RequireTagged,
            &mut meter,
        )
        .unwrap();
        let tensor = contract_boundary(&snapshot, &[equality], 2, &mut meter).unwrap();
        assert_eq!(
            tensor.values,
            vec![1u8.into(), 0u8.into(), 0u8.into(), 1u8.into()]
        );
        let summary = rank_summary(&tensor, 16, &mut meter).unwrap();
        assert_eq!(summary.canonical_rank, 2);
        assert_eq!(summary.best_balanced_rank, 2);
    }

    #[test]
    fn functional_signal_quotient_matches_raw_boundary_and_rank() {
        let mut meter = work();
        let mut encoder = Encoder::new();
        encoder.set_semantic_region("left").unwrap();
        let input = encoder.input_word(1, &meter).unwrap();
        encoder.mark_interface(&input, &meter).unwrap();
        encoder.set_semantic_region("right").unwrap();
        let zero = Encoder::constant(1, &BigUint::zero()).unwrap();
        encoder.assert_equal(&input, &zero, &meter).unwrap();
        encoder.assert_equal(&input, &zero, &meter).unwrap();
        let network = encoder.finish(&meter).unwrap();
        let left_vertices = network
            .vertices
            .iter()
            .enumerate()
            .filter_map(|(vertex_id, vertex)| {
                vertex
                    .predicate
                    .canonical_tag()
                    .and_then(|tag| {
                        tag.parameters
                            .iter()
                            .find_map(|(key, value)| (key == "semantic_region").then_some(value))
                    })
                    .is_some_and(|region| region == "left")
                    .then_some(vertex_id)
            })
            .collect::<Vec<_>>();
        let snapshot = CanonicalNetwork::capture(
            &network,
            &vec![None; network.edge_count()],
            IrProvenance {
                source: "functional-boundary-control".into(),
                generator: "unit-test".into(),
                revision: "test-v1".into(),
            },
            ExportPolicy::RequireTagged,
            &mut meter,
        )
        .unwrap();
        let direct = contract_boundary(&snapshot, &left_vertices, 16, &mut meter).unwrap();
        let quotient =
            contract_functional_boundary(&network, &left_vertices, 8, 8, &mut meter).unwrap();
        assert_eq!(quotient.quotient.width(), 1);
        assert_eq!(quotient.cut_edge_classes.len(), 1);
        assert_eq!(quotient.cut_edge_classes[0].len(), direct.width());
        for raw_assignment in 0..direct.values.len() {
            let first = raw_assignment & 1;
            let consistent =
                (0..direct.width()).all(|position| (raw_assignment >> position) & 1 == first);
            let expected = if consistent {
                quotient.quotient.values[first].clone()
            } else {
                BigUint::zero()
            };
            assert_eq!(direct.values[raw_assignment], expected);
        }
        let direct_rank = rank_summary(&direct, 16, &mut meter).unwrap();
        let quotient_rank = expanded_rank_summary(&quotient, &mut meter).unwrap();
        assert_eq!(quotient_rank.canonical_rank, direct_rank.canonical_rank);
        assert_eq!(
            quotient_rank.best_balanced_rank,
            direct_rank.best_balanced_rank
        );
        let mut unsupported = network.clone();
        unsupported.model = None;
        assert!(
            contract_functional_boundary(&unsupported, &left_vertices, 8, 8, &mut meter).is_err()
        );
    }

    #[test]
    fn quotient_symmetry_rank_matches_exhaustive_raw_splits() {
        let mut meter = work();
        let class_size_sets = [
            vec![1],
            vec![2],
            vec![1, 2],
            vec![2, 2],
            vec![1, 2, 3],
            vec![2, 3, 3],
        ];
        for (case, class_sizes) in class_size_sets.iter().enumerate() {
            let raw_width = class_sizes.iter().sum::<usize>();
            let quotient_width = class_sizes.len();
            let quotient_entries = 1usize << quotient_width;
            for salt in 0..8usize {
                let quotient_values = (0..quotient_entries)
                    .map(|assignment| {
                        BigUint::from(((assignment * 7 + case * 5 + salt * 3) % 5) as u8)
                    })
                    .collect::<Vec<_>>();
                let mut next_edge = 0usize;
                let classes = class_sizes
                    .iter()
                    .map(|&size| {
                        let class = (next_edge..next_edge + size).collect::<Vec<_>>();
                        next_edge += size;
                        class
                    })
                    .collect::<Vec<_>>();
                let functional = FunctionalBoundaryTensor {
                    raw_cut_edges: (0..raw_width).collect(),
                    cut_edge_classes: classes.clone(),
                    selected_free_input_positions: Vec::new(),
                    quotient: BoundaryTensor {
                        cut_edges: classes.iter().map(|class| class[0]).collect(),
                        internal_edges: Vec::new(),
                        support: quotient_values
                            .iter()
                            .filter(|value| !value.is_zero())
                            .count(),
                        values: quotient_values.clone(),
                        assignments_examined: 0,
                    },
                };
                let mut raw_values = vec![BigUint::zero(); 1usize << raw_width];
                for (raw_assignment, value) in raw_values.iter_mut().enumerate() {
                    let mut quotient_assignment = 0usize;
                    let mut consistent = true;
                    for (class_id, class) in classes.iter().enumerate() {
                        let bit = raw_assignment >> class[0] & 1;
                        if class
                            .iter()
                            .any(|&position| raw_assignment >> position & 1 != bit)
                        {
                            consistent = false;
                            break;
                        }
                        quotient_assignment |= bit << class_id;
                    }
                    if consistent {
                        *value = quotient_values[quotient_assignment].clone();
                    }
                }
                let raw = BoundaryTensor {
                    cut_edges: (0..raw_width).collect(),
                    internal_edges: Vec::new(),
                    support: raw_values.iter().filter(|value| !value.is_zero()).count(),
                    values: raw_values,
                    assignments_examined: 0,
                };
                let expected = rank_summary(&raw, 16, &mut meter).unwrap();
                let actual = expanded_rank_summary(&functional, &mut meter).unwrap();
                assert_eq!(
                    (actual.canonical_rank, actual.best_balanced_rank),
                    (expected.canonical_rank, expected.best_balanced_rank),
                    "case {case}, salt {salt}"
                );
            }
        }
    }

    #[test]
    fn free_edge_cap_and_cut_pins_fail_closed() {
        let mut meter = work();
        let mut builder = GraphBuilder::new();
        let left = builder.vertex(FnPredicate::tagged(
            1,
            PredicateTag::new("control.left", vec![]),
            |_, _| Ok(true),
        ));
        let right = builder.vertex(FnPredicate::tagged(
            1,
            PredicateTag::new("control.right", vec![]),
            |_, _| Ok(true),
        ));
        builder.edge(left, right, &mut meter).unwrap();
        let graph = builder.finish(vec![]).unwrap();
        let provenance = IrProvenance {
            source: "rank-control".into(),
            generator: "unit-test".into(),
            revision: "0000000000000000000000000000000000000000".into(),
        };
        let unpinned = CanonicalNetwork::capture(
            &graph,
            &[None],
            provenance.clone(),
            ExportPolicy::RequireTagged,
            &mut meter,
        )
        .unwrap();
        assert!(contract_boundary(&unpinned, &[left], 0, &mut meter).is_err());
        assert!(contract_boundary(&unpinned, &[right, left], 1, &mut meter).is_err());
        let pinned = CanonicalNetwork::capture(
            &graph,
            &[Some(false)],
            provenance,
            ExportPolicy::RequireTagged,
            &mut meter,
        )
        .unwrap();
        assert!(contract_boundary(&pinned, &[left], 1, &mut meter).is_err());
    }
}
