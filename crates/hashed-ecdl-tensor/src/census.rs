//! Exact boundary tensors and fraction-free flattening ranks.
//!
//! This module is the bounded instrument for the E1 boundary-rank census. It
//! enumerates only an explicitly capped side of a canonical Boolean tensor
//! network. Exhaustion is an error and is never reported as a zero tensor.

use crate::canonical::CanonicalNetwork;
use crate::{integrity, invalid, overflow, BigInt, BigUint, Meter, Result};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

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

    let free_internal: Vec<_> = internal_edges
        .iter()
        .copied()
        .filter(|&edge| network.pins[edge].is_none())
        .collect();
    let free_width = cut_edges
        .len()
        .checked_add(free_internal.len())
        .ok_or_else(|| overflow("boundary free width"))?;
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
