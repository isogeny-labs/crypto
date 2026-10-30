//! Shape of the low-degree gadget networks as the field size grows, shared by
//! the `ecdlscale` command and the root `ic tensor scale` subcommand.
//! Networks are built but never evaluated; the modulus is p = 2^(b-1)+1,
//! which need not be prime because only the structure is measured.

use num_bigint::BigUint;

use crate::{gadget_shapes, Arith, NetworkStats};

/// The Go command's default field sizes.
pub const DEFAULT_BITS: [usize; 5] = [64, 256, 1024, 4096, 16384];

/// One row of the shape table.
#[derive(Clone, Debug)]
pub struct ScaleRow {
    pub b: usize,
    /// Block width, block count and NTT length chosen by `Arith::new`.
    pub t: usize,
    pub n: usize,
    pub l: usize,
    pub mul: NetworkStats,
    pub add: NetworkStats,
    pub point: NetworkStats,
}

impl ScaleRow {
    /// Builds the mul, complete-add and point networks for a b-bit modulus.
    /// Returns None for b < 4, which the Go command rejects.
    pub fn new(b: usize) -> Option<ScaleRow> {
        if b < 4 {
            return None;
        }
        let p: BigUint = (BigUint::from(1u32) << (b - 1)) + 1u32;
        let ar = Arith::new(&p);
        let (mul, add, point) = gadget_shapes(&ar);
        Some(ScaleRow {
            b,
            t: ar.t,
            n: ar.n,
            l: ar.l,
            mul,
            add,
            point,
        })
    }

    pub fn max_degree(&self) -> usize {
        [&self.mul, &self.add, &self.point]
            .iter()
            .map(|st| st.max_degree)
            .max()
            .unwrap_or(0)
    }

    pub fn is_graph(&self) -> bool {
        self.mul.is_graph && self.add.is_graph && self.point.is_graph
    }

    /// Edges per bit of one network.
    pub fn per_bit(&self, st: &NetworkStats) -> f64 {
        st.edges as f64 / self.b as f64
    }

    /// Max degree / b^(3/4), which should stay bounded.
    pub fn degree_ratio(&self) -> f64 {
        self.max_degree() as f64 / (self.b as f64).powf(0.75)
    }

    /// The table header, as the Go command prints it.
    pub fn header() -> String {
        format!(
            "{:>7} {:>5} {:>4} {:>4} | {:>8} {:>8} {:>8} | {:>7} {:>9}",
            "b", "t", "n", "L", "mul e/b", "add e/b", "point e/b", "maxdeg", "deg/b^.75"
        )
    }

    /// The table row, as the Go command prints it.
    pub fn line(&self) -> String {
        format!(
            "{:>7} {:>5} {:>4} {:>4} | {:>8.1} {:>8.1} {:>8.1} | {:>7} {:>9.1}",
            self.b,
            self.t,
            self.n,
            self.l,
            self.per_bit(&self.mul),
            self.per_bit(&self.add),
            self.per_bit(&self.point),
            self.max_degree(),
            self.degree_ratio()
        )
    }
}
