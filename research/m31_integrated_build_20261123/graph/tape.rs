//! Exact matrix-free left-kernel projection, with forward and transpose products.
//! No cancellation witness or projected small matrix is expanded in this type.
use crate::{add, admission::validate_rows, inv, large_prime_filter::PartialRelation, mul, sub, ELL};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TapeLimits {
    pub max_tape_terms: usize,
    pub max_pivot_nonzeros: usize,
    pub max_build_multiply_add_terms: u64,
}
impl Default for TapeLimits {
    fn default() -> Self { Self { max_tape_terms: 1_000_000, max_pivot_nonzeros: 1_000_000, max_build_multiply_add_terms: 20_000_000 } }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TapeStats {
    pub input_rows: usize,
    pub lp_rank: usize,
    pub output_rows: usize,
    pub small_columns: usize,
    pub small_nonzeros: usize,
    pub tape_terms: usize,
    pub peak_pivot_and_working_nonzeros: usize,
    pub build_multiply_add_terms: u64,
    pub build_inverse_calls: u64,
}
#[derive(Clone, Debug)]
struct Node { dependencies: Vec<(usize, u64)>, scale: u64 }
#[derive(Clone, Debug)]
pub struct ProjectionTape {
    small: Vec<Vec<(usize, u64)>>,
    rhs: Vec<u64>,
    nodes: Vec<Node>,
    dependent: Vec<usize>,
    /// Sorted original small-column IDs; includes sparse high target ID safely.
    columns: Vec<u32>,
    stats: TapeStats,
}

impl ProjectionTape {
    pub fn columns(&self) -> &[u32] { &self.columns }
    pub fn stats(&self) -> &TapeStats { &self.stats }
    /// Build in original row order. Refusal returns no partial operator.
    pub fn build(rows: &[PartialRelation], limits: &TapeLimits) -> Result<Self, String> {
        validate_rows(rows)?;
        let columns: Vec<u32> = rows.iter().flat_map(|r| r.small.iter().map(|t| t.0)).collect::<BTreeSet<_>>().into_iter().collect();
        let col_of: BTreeMap<u32, usize> = columns.iter().enumerate().map(|(i, c)| (*c, i)).collect();
        let small: Vec<Vec<(usize, u64)>> = rows.iter().map(|r| r.small.iter().map(|&(c,a)| (col_of[&c],a)).collect()).collect();
        let mut stats = TapeStats { input_rows: rows.len(), small_columns: columns.len(), small_nonzeros: small.iter().map(Vec::len).sum(), ..Default::default() };
        let mut pivots: BTreeMap<u64, (usize, BTreeMap<u64, u64>)> = BTreeMap::new();
        let mut stored = 0usize;
        let mut nodes = Vec::new();
        let mut dependent = Vec::new();
        for (i, row) in rows.iter().enumerate() {
            let mut work: BTreeMap<u64, u64> = row.large.iter().copied().collect();
            let mut dependencies = Vec::new();
            let mut scale = 1;
            let mut is_pivot = false;
            loop {
                stats.peak_pivot_and_working_nonzeros = stats.peak_pivot_and_working_nonzeros.max(stored + work.len());
                if stored + work.len() > limits.max_pivot_nonzeros { return Err("LP pivot/working fill cap".into()); }
                let Some((&column, &coefficient)) = work.iter().next() else { break; };
                if let Some((source, pivot)) = pivots.get(&column) {
                    stats.tape_terms += 1;
                    if stats.tape_terms > limits.max_tape_terms { return Err("projection tape term cap".into()); }
                    let operations = pivot.len() as u64;
                    stats.build_multiply_add_terms += operations;
                    if stats.build_multiply_add_terms > limits.max_build_multiply_add_terms { return Err("LP preparation work cap".into()); }
                    dependencies.push((*source, coefficient));
                    for (&c, &a) in pivot {
                        let value = sub(work.get(&c).copied().unwrap_or(0), mul(coefficient, a));
                        if value == 0 { work.remove(&c); } else { work.insert(c, value); }
                        stats.peak_pivot_and_working_nonzeros = stats.peak_pivot_and_working_nonzeros.max(stored + work.len());
                        if stored + work.len() > limits.max_pivot_nonzeros { return Err("LP pivot/working fill cap".into()); }
                    }
                } else {
                    scale = inv(coefficient);
                    stats.build_inverse_calls += 1;
                    for a in work.values_mut() { *a = mul(*a, scale); }
                    stored += work.len();
                    pivots.insert(column, (i, work));
                    is_pivot = true;
                    break;
                }
            }
            nodes.push(Node { dependencies, scale });
            if !is_pivot { dependent.push(i); }
        }
        stats.lp_rank = pivots.len();
        stats.output_rows = dependent.len();
        Ok(Self { small, rhs: rows.iter().map(|r| r.rhs).collect(), nodes, dependent, columns, stats })
    }

    fn project_original_rows(&self, values: &[u64]) -> Result<Vec<u64>, String> {
        if values.len() != self.nodes.len() || values.iter().any(|v| *v >= ELL) { return Err("original-row vector shape/canonicality".into()); }
        let mut evaluated = vec![0; self.nodes.len()];
        for (i, node) in self.nodes.iter().enumerate() {
            let mut value = values[i];
            for &(source, coefficient) in &node.dependencies { value = sub(value, mul(coefficient, evaluated[source])); }
            evaluated[i] = mul(value, node.scale);
        }
        Ok(self.dependent.iter().map(|i| evaluated[*i]).collect())
    }
    /// Hx in the dependent-row order. Products never materialize H or witnesses.
    pub fn apply(&self, x: &[u64]) -> Result<Vec<u64>, String> {
        if x.len() != self.columns.len() || x.iter().any(|v| *v >= ELL) { return Err("column vector shape/canonicality".into()); }
        let values: Vec<u64> = self.small.iter().map(|r| r.iter().fold(0, |s, &(j,a)| add(s, mul(a,x[j])))).collect();
        self.project_original_rows(&values)
    }
    pub fn projected_rhs(&self) -> Vec<u64> { self.project_original_rows(&self.rhs).expect("validated rhs") }
    /// H^T y, by reverse accumulation through the exact same linear tape.
    pub fn apply_transpose(&self, y: &[u64]) -> Result<Vec<u64>, String> {
        if y.len() != self.dependent.len() || y.iter().any(|v| *v >= ELL) { return Err("output vector shape/canonicality".into()); }
        let mut adjoints = vec![0; self.nodes.len()];
        for (&row, &v) in self.dependent.iter().zip(y) { adjoints[row] = v; }
        let mut original = vec![0; self.nodes.len()];
        for i in (0..self.nodes.len()).rev() {
            let value = mul(adjoints[i], self.nodes[i].scale);
            original[i] = value;
            for &(source, coefficient) in &self.nodes[i].dependencies { adjoints[source] = sub(adjoints[source], mul(coefficient, value)); }
        }
        let mut out = vec![0; self.columns.len()];
        for (row, value) in self.small.iter().zip(original) {
            for &(column, coefficient) in row { out[column] = add(out[column], mul(coefficient, value)); }
        }
        Ok(out)
    }
}
