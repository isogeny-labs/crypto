//! Conservative two-large-prime checkpoint gate; never a readiness or solve test.
//! A spanning tree supplies v-1 independent large-column rows. One nonzero
//! single-vertex row anchors that component and supplies its remaining pivot.
//! For N inputs, at most N - (V-C+A) independent small-only rows can result.
//! Weighted cycles may raise the actual large-column rank; ignoring that only
//! makes this bound less selective. The caller must still compute exact rank.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundSnapshot {
    pub sources: usize,
    pub vertices: usize,
    pub components: usize,
    pub anchored_components: usize,
    pub large_rank_lower_bound: usize,
    pub small_rank_upper_bound: usize,
    pub find_calls: u64,
    pub parent_probes: u64,
    pub successful_unions: usize,
}

#[derive(Clone, Debug)]
pub struct StructuralRankBound {
    parent: Vec<usize>,
    size: Vec<usize>,
    anchored: Vec<bool>,
    modulus: u64,
    sources: usize,
    vertices: usize,
    components: usize,
    anchored_components: usize,
    find_calls: u64,
    parent_probes: u64,
    successful_unions: usize,
}

impl StructuralRankBound {
    /// The caller supplies a prime modulus, as required by the relation solver.
    pub fn new(identity_limit: usize, prime_modulus: u64) -> Result<Self, &'static str> {
        if !(1..=65536).contains(&identity_limit) || prime_modulus < 3 {
            return Err("bounded identities and prime modulus required");
        }
        Ok(Self {
            parent: vec![usize::MAX; identity_limit],
            size: vec![0; identity_limit],
            anchored: vec![false; identity_limit],
            modulus: prime_modulus,
            sources: 0,
            vertices: 0,
            components: 0,
            anchored_components: 0,
            find_calls: 0,
            parent_probes: 0,
            successful_unions: 0,
        })
    }
    fn activate(&mut self, v: usize) {
        if self.parent[v] == usize::MAX {
            self.parent[v] = v;
            self.size[v] = 1;
            self.vertices += 1;
            self.components += 1;
        }
    }
    fn root(&mut self, mut v: usize) -> usize {
        self.find_calls += 1;
        loop {
            self.parent_probes += 1;
            let next = self.parent[v];
            if next == v {
                return v;
            }
            v = next;
        }
    }
    /// Validate before changing state. Only canonical homogeneous source rows
    /// with zero, one or two distinct nonzero large coefficients are accepted.
    pub fn add_row(&mut self, rhs: u64, large: &[(u64, u64)]) -> Result<(), &'static str> {
        if rhs != 0
            || large.len() > 2
            || self.sources >= 4096
            || large
                .iter()
                .any(|&(v, c)| v >= self.parent.len() as u64 || c == 0 || c >= self.modulus)
            || large.windows(2).any(|w| w[0].0 >= w[1].0)
        {
            return Err("noncanonical or nonhomogeneous bounded large support");
        }
        self.sources += 1;
        for &(v, _) in large {
            self.activate(v as usize);
        }
        if large.len() == 1 {
            let r = self.root(large[0].0 as usize);
            if !self.anchored[r] {
                self.anchored[r] = true;
                self.anchored_components += 1;
            }
        } else if large.len() == 2 {
            let (mut a, mut b) = (
                self.root(large[0].0 as usize),
                self.root(large[1].0 as usize),
            );
            if a != b {
                if self.size[a] < self.size[b] {
                    std::mem::swap(&mut a, &mut b);
                }
                if self.anchored[a] && self.anchored[b] {
                    self.anchored_components -= 1;
                }
                self.anchored[a] |= self.anchored[b];
                self.parent[b] = a;
                self.size[a] += self.size[b];
                self.components -= 1;
                self.successful_unions += 1;
            }
        }
        Ok(())
    }
    pub fn snapshot(&self) -> BoundSnapshot {
        let lower = self.vertices - self.components + self.anchored_components;
        BoundSnapshot {
            sources: self.sources,
            vertices: self.vertices,
            components: self.components,
            anchored_components: self.anchored_components,
            large_rank_lower_bound: lower,
            small_rank_upper_bound: self
                .sources
                .checked_sub(lower)
                .expect("forest rank bounded by source count"),
            find_calls: self.find_calls,
            parent_probes: self.parent_probes,
            successful_unions: self.successful_unions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn rank(rows: &[Vec<(u64, u64)>], columns: usize, p: u64) -> usize {
        let mut matrix = vec![vec![0; columns]; rows.len()];
        for (row, sparse) in matrix.iter_mut().zip(rows) {
            for &(i, v) in sparse {
                row[i as usize] = v;
            }
        }
        let mut rank = 0;
        for col in 0..columns {
            let Some(pivot) = (rank..matrix.len()).find(|&i| matrix[i][col] != 0) else {
                continue;
            };
            matrix.swap(rank, pivot);
            let (mut base, mut exponent, mut inverse) = (matrix[rank][col], p - 2, 1);
            while exponent != 0 {
                if exponent & 1 != 0 {
                    inverse = inverse * base % p;
                }
                base = base * base % p;
                exponent >>= 1;
            }
            for x in &mut matrix[rank] {
                *x = *x * inverse % p;
            }
            let pivot_row = matrix[rank].clone();
            for row in matrix.iter_mut().skip(rank + 1) {
                let factor = row[col];
                for (a, &b) in row.iter_mut().zip(&pivot_row) {
                    *a = (*a + p - factor * b % p) % p;
                }
            }
            rank += 1;
            if rank == matrix.len() {
                break;
            }
        }
        rank
    }
    #[test]
    fn anchored_components_merge_without_overstating_rank() {
        let mut gate = StructuralRankBound::new(4, 103).unwrap();
        for row in [
            vec![(0, 7)],
            vec![(1, 13)],
            vec![(0, 3), (1, 9)],
            vec![(2, 1), (3, 8)],
            vec![(1, 5), (2, 7)],
            vec![],
        ] {
            gate.add_row(0, &row).unwrap();
        }
        let s = gate.snapshot();
        assert_eq!(
            (
                s.vertices,
                s.components,
                s.anchored_components,
                s.large_rank_lower_bound,
                s.small_rank_upper_bound
            ),
            (4, 1, 1, 4, 2)
        );
    }
    #[test]
    fn weighted_cycles_can_raise_actual_rank_beyond_the_lower_bound() {
        let rows = vec![vec![(0, 1), (1, 1)], vec![(0, 1), (1, 2)]];
        let mut gate = StructuralRankBound::new(2, 103).unwrap();
        for row in &rows {
            gate.add_row(0, row).unwrap();
        }
        assert_eq!(rank(&rows, 2, 103), 2);
        assert_eq!(gate.snapshot().large_rank_lower_bound, 1);
        assert_eq!(gate.snapshot().small_rank_upper_bound, 1);
    }
    #[test]
    fn malformed_inputs_leave_state_unchanged() {
        let mut gate = StructuralRankBound::new(4, 103).unwrap();
        gate.add_row(0, &[(0, 1), (1, 102)]).unwrap();
        let before = gate.snapshot();
        for (rhs, row) in [
            (1, vec![]),
            (0, vec![(4, 1)]),
            (0, vec![(1, 0)]),
            (0, vec![(1, 103)]),
            (0, vec![(1, 1), (1, 2)]),
            (0, vec![(2, 1), (1, 1)]),
            (0, vec![(0, 1), (1, 1), (2, 1)]),
        ] {
            assert!(gate.add_row(rhs, &row).is_err());
            assert_eq!(gate.snapshot(), before);
        }
    }
    #[test]
    fn every_stream_prefix_bounds_independently_computed_weighted_matrix_rank() {
        let mut state = 0x91c035ad991617efu64;
        for p in [3, 103] {
            for _ in 0..4096 {
                let mut gate = StructuralRankBound::new(6, p).unwrap();
                let mut rows = Vec::new();
                for _ in 0..12 {
                    let mut draw = || {
                        state ^= state << 13;
                        state ^= state >> 7;
                        state ^= state << 17;
                        state
                    };
                    let len = draw() % 3;
                    let mut row = Vec::new();
                    for _ in 0..len {
                        let v = draw() % 6;
                        let c = 1 + draw() % (p - 1);
                        if !row.iter().any(|&(i, _)| i == v) {
                            row.push((v, c));
                        }
                    }
                    row.sort_unstable();
                    gate.add_row(0, &row).unwrap();
                    rows.push(row);
                    let exact = rank(&rows, 6, p);
                    let bound = gate.snapshot();
                    assert!(bound.large_rank_lower_bound <= exact);
                    assert!(bound.small_rank_upper_bound >= rows.len() - exact);
                }
            }
        }
    }
}
