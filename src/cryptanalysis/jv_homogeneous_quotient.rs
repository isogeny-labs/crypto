//! A checked codimension-one homogeneous RREF reduces affine rows to two columns.
//! Known algebra/state events are counted; these are not complete native costs.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtraCounts {
    pub products: u64,
    pub additions: u64,
    pub word_events: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Quotient {
    pub free_column: usize,
    pub kernel: Vec<u64>,
    pub counts: ExtraCounts,
    modulus: u64,
}
impl Quotient {
    /// Rank must already have been certified against the source matrix.
    pub(crate) fn from_rank_certified_kernel(
        modulus: u64,
        kernel: Vec<u64>,
        rank: usize,
    ) -> Result<Self, &'static str> {
        if modulus < 3
            || kernel.len() < 2
            || kernel.len() > 4096
            || rank.checked_add(1) != Some(kernel.len())
            || kernel.last() != Some(&1)
            || kernel.iter().any(|&x| x >= modulus)
        {
            return Err("canonical certified kernel required");
        }
        let free_column = kernel.len() - 1;
        let counts = ExtraCounts {
            word_events: (kernel.len() + 4) as u64,
            ..Default::default()
        };
        Ok(Self {
            free_column,
            kernel,
            counts,
            modulus,
        })
    }
    pub fn from_rref(
        modulus: u64,
        columns: usize,
        pivots: &[(usize, Vec<u64>, u64)],
    ) -> Result<Self, &'static str> {
        if modulus < 3 || !(1..=4095).contains(&columns) || pivots.len() + 1 != columns {
            return Err("codimension one required");
        }
        let mut counts = ExtraCounts::default();
        let mut marked = vec![false; columns];
        counts.word_events += columns as u64;
        for (c, row, rhs) in pivots {
            counts.word_events += 4;
            if *c >= columns || marked[*c] || row.len() != columns || *rhs != 0 {
                return Err("homogeneous pivot shape");
            }
            marked[*c] = true;
            counts.word_events += 1;
        }
        let mut free = None;
        for (c, &used) in marked.iter().enumerate() {
            counts.word_events += 2;
            if !used && free.replace(c).is_some() {
                return Err("multiple free columns");
            }
        }
        let free = free.ok_or("no free column")?;
        let mut kernel = vec![0; columns];
        counts.word_events += columns as u64;
        kernel[free] = 1;
        counts.word_events += 1;
        for (c, row, _) in pivots {
            for (j, &value) in row.iter().enumerate() {
                counts.word_events += 4;
                if value >= modulus || (marked[j] && value != u64::from(j == *c)) {
                    return Err("canonical RREF required");
                }
            }
            let value = row[free];
            counts.word_events += 3;
            if value != 0 {
                kernel[*c] = modulus - value;
                counts.additions += 1;
            }
        }
        Ok(Self {
            free_column: free,
            kernel,
            counts,
            modulus,
        })
    }
    pub fn project(&mut self, row: &[u64], rhs: u64) -> Result<([u64; 2], u64), &'static str> {
        self.counts.word_events += 2;
        if row.len() != self.kernel.len() + 1 || rhs >= self.modulus {
            return Err("affine shape/RHS");
        }
        let mut dot = 0;
        for (&a, &h) in row[..self.kernel.len()].iter().zip(&self.kernel) {
            self.counts.word_events += 5;
            if a >= self.modulus {
                return Err("noncanonical affine coefficient");
            }
            if (a != 0) & (h != 0) {
                let product = ((a as u128 * h as u128) % self.modulus as u128) as u64;
                dot = ((dot as u128 + product as u128) % self.modulus as u128) as u64;
                self.counts.products += 1;
                self.counts.additions += 1;
            }
        }
        let target = *row.last().ok_or("target column")?;
        self.counts.word_events += 2;
        if target >= self.modulus {
            return Err("noncanonical target coefficient");
        }
        Ok(([dot, target], rhs))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn certified_kernel_constructor_shape_and_projection() {
        let mut q = Quotient::from_rank_certified_kernel(101, vec![99, 1], 1).unwrap();
        assert_eq!(q.project(&[3, 4, 1], 17).unwrap(), ([99, 1], 17));
        for (h, r) in [
            (vec![99, 1], 0),
            (vec![101, 1], 1),
            (vec![99, 0], 1),
            (vec![], 0),
        ] {
            assert!(Quotient::from_rank_certified_kernel(101, h, r).is_err());
        }
    }
    #[test]
    fn hand_kernel_and_projection() {
        let mut q = Quotient::from_rref(101, 2, &[(0, vec![1, 2], 0)]).unwrap();
        assert_eq!(q.kernel, vec![99, 1]);
        assert_eq!(q.free_column, 1);
        assert_eq!(q.counts.additions, 1);
        assert_eq!(q.project(&[3, 4, 1], 17).unwrap(), ([99, 1], 17));
        assert_eq!(q.project(&[1, 0, 2], 9).unwrap(), ([99, 2], 9));
        assert_eq!(q.counts.products, 3);
        assert_eq!(q.counts.additions, 4);
    }
    #[test]
    fn malformed_rref_and_affine_rows_rejected() {
        for pivots in [
            vec![],
            vec![(0, vec![1, 2], 1)],
            vec![(0, vec![2, 1], 0)],
            vec![(0, vec![1, 101], 0)],
        ] {
            assert!(Quotient::from_rref(101, 2, &pivots).is_err());
        }
        assert!(
            Quotient::from_rref(101, 3, &[(0, vec![1, 0, 1], 0), (0, vec![1, 0, 2], 0)]).is_err()
        );
        assert!(
            Quotient::from_rref(101, 3, &[(0, vec![1, 1, 1], 0), (1, vec![0, 1, 2], 0)]).is_err()
        );
        let mut q = Quotient::from_rref(101, 2, &[(0, vec![1, 2], 0)]).unwrap();
        assert!(q.project(&[0, 1], 0).is_err());
        assert!(q.project(&[101, 0, 1], 0).is_err());
        assert!(q.project(&[0, 0, 101], 0).is_err());
        assert!(q.project(&[0, 0, 1], 101).is_err());
    }
    #[test]
    fn every_free_column_and_out_of_order_pivots() {
        for p in [3, 101, (1u64 << 61) - 1, (1u64 << 63) + 29] {
            for columns in 1..12 {
                for free in 0..columns {
                    let kernel: Vec<_> = (0..columns)
                        .map(|c| {
                            if c == free {
                                1
                            } else {
                                ((c * 7 + 3) as u64) % p
                            }
                        })
                        .collect();
                    let mut pivots = Vec::new();
                    for c in 0..columns {
                        if c == free {
                            continue;
                        }
                        let mut row = vec![0; columns];
                        row[c] = 1;
                        row[free] = if kernel[c] == 0 { 0 } else { p - kernel[c] };
                        pivots.push((c, row, 0));
                    }
                    pivots.reverse();
                    let q = Quotient::from_rref(p, columns, &pivots).unwrap();
                    assert_eq!(q.free_column, free);
                    assert_eq!(q.kernel, kernel);
                }
            }
        }
    }
}
