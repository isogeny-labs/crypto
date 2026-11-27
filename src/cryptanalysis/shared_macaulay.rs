//! Bounded shared row-space preparation for a parameterized polynomial family.
//!
//! A family consists of common equations and generators
//! `sum_k parameter^k * pencil[generator][k]`. The monomial universe and each
//! generator's permitted monomial multipliers are frozen from the UNION of
//! its coefficient supports. Specialization never silently enlarges that
//! envelope when a leading coefficient vanishes.
//!
//! The common Macaulay rows are reduced once. In pivot/free coordinates their
//! nonzero rows are `[I | B]`. A dynamic row block `[A | C]` is projected to
//! `C - A*B`. Coefficient blocks of the polynomial pencil may be projected
//! separately because this projection is linear. They MUST be recombined at
//! the requested parameter BEFORE any further row reduction: reducing each
//! coefficient ideal separately would, for example, turn `x - t` into `1`.
//!
//! All returned polynomials are consequences of the original specialized
//! equations. They are a preconditioner, not a complete Groebner basis or an
//! unsatisfiability decision. Callers retain the original equations and their
//! variable domains, and check every final witness. Bounds return errors,
//! never a claim that the specialized system has no solution.
//!
//! Reports count named field operations and matrix payloads. They do not count
//! primality testing, inversion internals, sorting, allocation, or all CPU
//! instructions. Payload bounds are not process RSS measurements. No timing or
//! performance assertion is made by this module.
//!
//! Returned equation terms use descending total-degree/lexicographic order.
//! Normalize them to a consumer's requested monomial order before calling a
//! helper that assumes that order; the existing F4 visitor normalizes its input.

use std::collections::BTreeMap;
use std::fmt;

use super::exact_modular_matrix::{Backend, DenseShape, KernelCounts, ModMatrix32};
use super::f4_fp::Poly;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize)]
pub struct Limits {
    pub max_variables: usize,
    pub max_degree: u32,
    pub max_monomials: usize,
    /// Common plus dynamic rows, before any row reduction.
    pub max_rows: usize,
    /// Conservative peak simultaneous u32 matrix payload, in entries.
    pub max_cells: usize,
    pub max_input_terms: usize,
    pub max_parameter_powers: usize,
    /// One preparation or one target call; excludes inversion internals.
    pub max_scalar_products: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_variables: 8,
            max_degree: 12,
            max_monomials: 2048,
            max_rows: 2048,
            max_cells: 1_048_576,
            max_input_terms: 16_384,
            max_parameter_powers: 17,
            max_scalar_products: 64 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

/// An exact, collision-free cache identity. Construction is private and all
/// input terms and generator lists are normalized before this key is made.
/// The arithmetic backend is not part of the mathematical family identity.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub struct FamilyKey {
    modulus: u64,
    variables: usize,
    degree: u32,
    common: Vec<Poly>,
    pencil: Vec<Vec<Poly>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub struct ArithmeticCounts {
    /// Includes scalar helpers AND kernel products. Do not add the kernel's
    /// field product count again; `kernels` is the implementation-detail subset.
    pub logical_products: u64,
    pub coefficient_additions: u64,
    pub inversions: u64,
    pub row_updates: u64,
    pub kernels: KernelCounts,
}

impl ArithmeticCounts {
    pub fn merge(&mut self, other: Self) {
        self.logical_products = self
            .logical_products
            .checked_add(other.logical_products)
            .expect("arithmetic product counter overflow");
        self.coefficient_additions = self
            .coefficient_additions
            .checked_add(other.coefficient_additions)
            .expect("arithmetic addition counter overflow");
        self.inversions = self
            .inversions
            .checked_add(other.inversions)
            .expect("arithmetic inversion counter overflow");
        self.row_updates = self
            .row_updates
            .checked_add(other.row_updates)
            .expect("arithmetic row counter overflow");
        self.kernels += other.kernels;
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct FreshSetupReport {
    pub common_rows: usize,
    pub dynamic_rows: usize,
    pub parameter_powers: usize,
    pub monomials: usize,
    pub input_terms: usize,
    pub coefficient_blocks: usize,
    pub omitted_common_generators: usize,
    pub omitted_pencil_generators: usize,
    pub matrix_payload_bytes_upper_bound: usize,
    pub numerical_elimination_performed: bool,
    pub numerical_projection_performed: bool,
    pub cost_counter_coverage_complete: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct SetupReport {
    pub metadata: FreshSetupReport,
    pub common_rows: usize,
    pub dynamic_rows: usize,
    pub parameter_powers: usize,
    pub monomials: usize,
    pub static_rank: usize,
    pub free_columns: usize,
    pub omitted_common_generators: usize,
    pub omitted_pencil_generators: usize,
    pub matrix_assembly_terms: u64,
    pub common_elimination: ArithmeticCounts,
    pub coefficient_projection: ArithmeticCounts,
    pub projected_coefficient_cells: usize,
    pub matrix_payload_bytes_upper_bound: usize,
    pub cost_counter_coverage_complete: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct TargetReport {
    pub strategy: &'static str,
    pub parameter: u64,
    pub matrix_rows_before_reduction: usize,
    pub matrix_columns: usize,
    pub matrix_assembly_terms: u64,
    pub parameter_combination: ArithmeticCounts,
    pub elimination: ArithmeticCounts,
    pub cost_counter_coverage_complete: bool,
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PreparedTarget {
    /// Safe extra equations. Keep the caller's original equations as well.
    pub consequences: Vec<Poly>,
    pub static_rank: usize,
    pub residual_rank: usize,
    pub total_rank: usize,
    pub report: TargetReport,
}

#[derive(Clone, Debug)]
struct RowPlan {
    generator: usize,
    multiplier: Vec<u32>,
}

struct ProductBudget {
    used: u64,
    limit: u64,
}

impl ProductBudget {
    fn charge(&mut self, products: u64) -> Result<(), Error> {
        self.used = self
            .used
            .checked_add(products)
            .ok_or_else(|| Error("scalar-product count overflow".into()))?;
        if self.used > self.limit {
            return Err(Error(format!(
                "scalar-product budget exceeded: {} > {}",
                self.used, self.limit
            )));
        }
        Ok(())
    }
}

/// Immutable prepared family. Calls neither alter its cache nor share mutable
/// pivot state between targets.
pub struct SharedMacaulay {
    field: ModMatrix32,
    backend: Backend,
    limits: Limits,
    key: FamilyKey,
    monomials: Vec<Vec<u32>>,
    columns: BTreeMap<Vec<u32>, usize>,
    common_plan: Vec<RowPlan>,
    dynamic_plan: Vec<RowPlan>,
    common_basis: Vec<u32>,
    static_pivots: Vec<usize>,
    free: Vec<usize>,
    /// One row-major (dynamic_rows x free_columns) matrix per parameter power.
    projected: Vec<Vec<u32>>,
    report: SetupReport,
}

/// A genuinely fresh comparator: its constructor admits and freezes input
/// metadata, without forming or eliminating a numerical Macaulay matrix.
/// It can be constructed even when shared coefficient projection would exceed
/// its product or payload budget. Each target call starts from original rows.
pub struct FreshMacaulay {
    field: ModMatrix32,
    backend: Backend,
    limits: Limits,
    key: FamilyKey,
    monomials: Vec<Vec<u32>>,
    columns: BTreeMap<Vec<u32>, usize>,
    common_plan: Vec<RowPlan>,
    dynamic_plan: Vec<RowPlan>,
    report: FreshSetupReport,
}

fn checked_cells(parts: &[usize], limit: usize) -> Result<usize, Error> {
    let total = parts
        .iter()
        .try_fold(0usize, |sum, &part| sum.checked_add(part));
    match total {
        Some(total) if total <= limit => Ok(total),
        _ => Err(Error("matrix payload exceeds configured cell bound".into())),
    }
}

fn product_size(a: usize, b: usize) -> Result<usize, Error> {
    a.checked_mul(b)
        .ok_or_else(|| Error("matrix dimensions overflow".into()))
}

fn validate_limits(limits: Limits, variables: usize, degree: u32) -> Result<(), Error> {
    if variables == 0
        || variables > limits.max_variables
        || degree > limits.max_degree
        || limits.max_variables > 16
        || limits.max_degree > 32
        || limits.max_monomials == 0
        || limits.max_monomials > 8192
        || limits.max_rows == 0
        || limits.max_rows > 8192
        || limits.max_cells == 0
        || limits.max_cells > 16 * 1024 * 1024
        || limits.max_input_terms == 0
        || limits.max_input_terms > 1_048_576
        || limits.max_parameter_powers == 0
        || limits.max_parameter_powers > 65
        || limits.max_scalar_products == 0
        || limits.max_scalar_products > 64 * 1024 * 1024
    {
        return Err(Error("invalid or exceeded Macaulay resource limits".into()));
    }
    Ok(())
}

fn degree_of(poly: &Poly) -> u32 {
    poly.iter()
        .map(|(exponent, _)| exponent.iter().sum())
        .max()
        .unwrap_or(0)
}

fn canonical_poly(poly: &Poly, variables: usize, field: &ModMatrix32) -> Result<Poly, Error> {
    let mut coefficients = BTreeMap::<Vec<u32>, u32>::new();
    for (exponent, coefficient) in poly {
        if exponent.len() != variables
            || *coefficient >= u64::from(field.modulus())
            || exponent
                .iter()
                .try_fold(0_u32, |a, b| a.checked_add(*b))
                .is_none()
        {
            return Err(Error(
                "noncanonical coefficient or malformed exponent vector".into(),
            ));
        }
        if *coefficient != 0 {
            let value = coefficients.entry(exponent.clone()).or_default();
            *value = field.add(*value, *coefficient as u32);
        }
    }
    Ok(coefficients
        .into_iter()
        .filter(|(_, coefficient)| *coefficient != 0)
        .map(|(exponent, coefficient)| (exponent, u64::from(coefficient)))
        .collect())
}

fn monomial_list(variables: usize, degree: u32, limit: usize) -> Result<Vec<Vec<u32>>, Error> {
    // Count first, without allocating or iterating over an inadmissible space.
    let mut count = 1_u128;
    for i in 1..=variables {
        count = count * (u128::from(degree) + i as u128) / i as u128;
        if count > limit as u128 {
            return Err(Error("monomial universe exceeds configured bound".into()));
        }
    }
    fn visit(out: &mut Vec<Vec<u32>>, exponent: &mut [u32], position: usize, left: u32) {
        if position == exponent.len() {
            out.push(exponent.to_vec());
            return;
        }
        for power in 0..=left {
            exponent[position] = power;
            visit(out, exponent, position + 1, left - power);
        }
    }
    let mut result = Vec::with_capacity(count as usize);
    visit(&mut result, &mut vec![0; variables], 0, degree);
    // Deterministic graded lexicographic column order; constants come last.
    result.sort_by(|a, b| {
        let da: u32 = a.iter().sum();
        let db: u32 = b.iter().sum();
        db.cmp(&da).then_with(|| b.cmp(a))
    });
    Ok(result)
}

fn row_plan(
    degrees: impl Iterator<Item = (usize, Option<u32>)>,
    variables: usize,
    degree: u32,
    available_rows: usize,
) -> Result<(Vec<RowPlan>, usize), Error> {
    let mut plan = Vec::new();
    let mut omitted = 0;
    for (generator, generator_degree) in degrees {
        let Some(generator_degree) = generator_degree else {
            continue;
        };
        if generator_degree > degree {
            omitted += 1;
            continue;
        }
        let multipliers = monomial_list(variables, degree - generator_degree, available_rows)?;
        if plan.len().saturating_add(multipliers.len()) > available_rows {
            return Err(Error("Macaulay rows exceed configured bound".into()));
        }
        plan.extend(multipliers.into_iter().map(|multiplier| RowPlan {
            generator,
            multiplier,
        }));
    }
    Ok((plan, omitted))
}

fn shifted_column(
    exponent: &[u32],
    multiplier: &[u32],
    columns: &BTreeMap<Vec<u32>, usize>,
) -> usize {
    let shifted: Vec<u32> = exponent
        .iter()
        .zip(multiplier)
        .map(|(a, b)| a + b)
        .collect();
    // The multiplier plan was admitted against the union degree before use.
    columns[&shifted]
}

fn rref(
    matrix: &mut Vec<u32>,
    rows: usize,
    columns: usize,
    field: &ModMatrix32,
    backend: Backend,
    budget: &mut ProductBudget,
    counts: &mut ArithmeticCounts,
) -> Result<Vec<usize>, Error> {
    rref_with_prefix(matrix, rows, columns, 0, field, backend, budget, counts)
        .map(|(pivots, _)| pivots)
}

#[allow(clippy::too_many_arguments)]
fn subtract_row(
    matrix: &mut [u32],
    row: usize,
    pivot: usize,
    columns: usize,
    factor: u32,
    field: &ModMatrix32,
    backend: Backend,
    budget: &mut ProductBudget,
    counts: &mut ArithmeticCounts,
) -> Result<(), Error> {
    if factor == 0 {
        return Ok(());
    }
    budget.charge(columns as u64)?;
    let kernel = if row < pivot {
        let (before, pivot_and_after) = matrix.split_at_mut(pivot * columns);
        field.row_sub_assign(
            &mut before[row * columns..(row + 1) * columns],
            &pivot_and_after[..columns],
            factor,
            backend,
        )
    } else {
        let (before, row_and_after) = matrix.split_at_mut(row * columns);
        field.row_sub_assign(
            &mut row_and_after[..columns],
            &before[pivot * columns..(pivot + 1) * columns],
            factor,
            backend,
        )
    }
    .map_err(|error| Error(error.to_string()))?;
    counts.logical_products += columns as u64;
    counts.coefficient_additions += columns as u64;
    counts.row_updates += 1;
    counts.kernels += kernel;
    Ok(())
}

/// One incremental elimination pass. The rank after the common input prefix
/// is observed during that pass, without separately re-eliminating common rows.
#[allow(clippy::too_many_arguments)]
fn rref_with_prefix(
    matrix: &mut Vec<u32>,
    rows: usize,
    columns: usize,
    prefix_rows: usize,
    field: &ModMatrix32,
    backend: Backend,
    budget: &mut ProductBudget,
    counts: &mut ArithmeticCounts,
) -> Result<(Vec<usize>, usize), Error> {
    let mut pivots = Vec::new();
    let mut prefix_rank = 0;
    for row in 0..rows {
        let rank = pivots.len();
        for (pivot, &column) in pivots.iter().enumerate() {
            let factor = matrix[row * columns + column];
            subtract_row(
                matrix, row, pivot, columns, factor, field, backend, budget, counts,
            )?;
        }
        if let Some(column) = (0..columns).find(|&column| matrix[row * columns + column] != 0) {
            let inverse = field
                .inv(matrix[row * columns + column])
                .ok_or_else(|| Error("noninvertible pivot in admitted prime field".into()))?;
            counts.inversions += 1;
            budget.charge(columns as u64)?;
            counts.logical_products += columns as u64;
            for value in &mut matrix[row * columns..(row + 1) * columns] {
                *value = field.mul(*value, inverse, backend);
            }
            for previous in 0..rank {
                let factor = matrix[previous * columns + column];
                subtract_row(
                    matrix, previous, row, columns, factor, field, backend, budget, counts,
                )?;
            }
            for j in 0..columns {
                matrix.swap(rank * columns + j, row * columns + j);
            }
            pivots.push(column);
        }
        if row + 1 == prefix_rows {
            prefix_rank = pivots.len();
        }
    }
    matrix.truncate(pivots.len() * columns);
    // Put the reduced basis into canonical pivot order, using in-place cycles.
    let mut order: Vec<_> = (0..pivots.len()).collect();
    order.sort_by_key(|&i| pivots[i]);
    let mut destination = vec![0; order.len()];
    for (new, old) in order.into_iter().enumerate() {
        destination[old] = new;
    }
    for i in 0..pivots.len() {
        while destination[i] != i {
            let j = destination[i];
            for column in 0..columns {
                matrix.swap(i * columns + column, j * columns + column);
            }
            destination.swap(i, j);
            pivots.swap(i, j);
        }
    }
    Ok((pivots, prefix_rank))
}

fn rows_to_polys(matrix: &[u32], columns: &[usize], monomials: &[Vec<u32>]) -> Vec<Poly> {
    if columns.is_empty() {
        return Vec::new();
    }
    matrix
        .chunks_exact(columns.len())
        .map(|row| {
            row.iter()
                .zip(columns)
                .filter(|(coefficient, _)| **coefficient != 0)
                .map(|(&coefficient, &column)| (monomials[column].clone(), u64::from(coefficient)))
                .collect()
        })
        .collect()
}

impl FreshMacaulay {
    /// Freeze metadata only. `pencil[g][k]` is the coefficient polynomial of
    /// parameter^k in dynamic generator g. No numerical row space is prepared.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        p: u64,
        variables: usize,
        degree: u32,
        common: &[Poly],
        pencil: &[Vec<Poly>],
        limits: Limits,
        backend: Backend,
    ) -> Result<Self, Error> {
        validate_limits(limits, variables, degree)?;
        let field = ModMatrix32::new(p).map_err(|error| Error(error.to_string()))?;
        let mut input_terms = 0usize;
        // Bound even empty generator lists; otherwise metadata alone could be unbounded.
        if common.len().saturating_add(pencil.len()) > limits.max_input_terms {
            return Err(Error(
                "input generator count exceeds configured bound".into(),
            ));
        }
        if pencil
            .iter()
            .any(|generator| generator.len() > limits.max_parameter_powers)
        {
            return Err(Error("parameter degree exceeds configured bound".into()));
        }
        let coefficient_blocks = pencil.iter().try_fold(common.len(), |sum, generator| {
            sum.checked_add(generator.len())
        });
        if coefficient_blocks.is_none_or(|count| count > limits.max_input_terms) {
            return Err(Error(
                "input coefficient-block metadata exceeds configured bound".into(),
            ));
        }
        for polynomial in common.iter().chain(pencil.iter().flatten()) {
            input_terms = input_terms
                .checked_add(polynomial.len())
                .ok_or_else(|| Error("input term count overflow".into()))?;
            if input_terms > limits.max_input_terms {
                return Err(Error("input terms exceed configured bound".into()));
            }
        }
        let mut common: Vec<Poly> = common
            .iter()
            .map(|poly| canonical_poly(poly, variables, &field))
            .collect::<Result<_, _>>()?;
        common.retain(|poly| !poly.is_empty());
        common.sort();
        common.dedup();
        let mut pencil: Vec<Vec<Poly>> = pencil
            .iter()
            .map(|generator| {
                let mut generator = generator
                    .iter()
                    .map(|poly| canonical_poly(poly, variables, &field))
                    .collect::<Result<Vec<_>, _>>()?;
                while generator.last().is_some_and(Vec::is_empty) {
                    generator.pop();
                }
                Ok(generator)
            })
            .collect::<Result<_, Error>>()?;
        pencil.retain(|generator| !generator.is_empty());
        pencil.sort();
        pencil.dedup();
        let powers = pencil.iter().map(Vec::len).max().unwrap_or(0);
        let monomials = monomial_list(variables, degree, limits.max_monomials)?;
        let columns: BTreeMap<_, _> = monomials.iter().cloned().zip(0..).collect();
        let width = monomials.len();
        let (common_plan, omitted_common) = row_plan(
            common
                .iter()
                .enumerate()
                .map(|(i, poly)| (i, Some(degree_of(poly)))),
            variables,
            degree,
            limits.max_rows,
        )?;
        let (dynamic_plan, omitted_pencil) = row_plan(
            pencil.iter().enumerate().map(|(i, generator)| {
                (
                    i,
                    generator
                        .iter()
                        .filter(|poly| !poly.is_empty())
                        .map(degree_of)
                        .max(),
                )
            }),
            variables,
            degree,
            limits.max_rows - common_plan.len(),
        )?;
        let full_cells = product_size(common_plan.len() + dynamic_plan.len(), width)?;
        checked_cells(&[full_cells], limits.max_cells)?;
        let report = FreshSetupReport {
            common_rows: common_plan.len(),
            dynamic_rows: dynamic_plan.len(),
            parameter_powers: powers,
            monomials: width,
            input_terms,
            coefficient_blocks: coefficient_blocks.expect("checked coefficient blocks"),
            omitted_common_generators: omitted_common,
            omitted_pencil_generators: omitted_pencil,
            matrix_payload_bytes_upper_bound: full_cells * std::mem::size_of::<u32>(),
            numerical_elimination_performed: false,
            numerical_projection_performed: false,
            cost_counter_coverage_complete: false,
        };
        Ok(Self {
            field,
            backend,
            limits,
            key: FamilyKey {
                modulus: p,
                variables,
                degree,
                common,
                pencil,
            },
            monomials,
            columns,
            common_plan,
            dynamic_plan,
            report,
        })
    }

    pub fn key(&self) -> &FamilyKey {
        &self.key
    }
    pub fn setup_report(&self) -> &FreshSetupReport {
        &self.report
    }
    pub fn monomials(&self) -> &[Vec<u32>] {
        &self.monomials
    }

    pub fn fresh(&self, parameter: u64) -> Result<PreparedTarget, Error> {
        fresh_target(
            &self.field,
            self.backend,
            self.limits,
            &self.key,
            &self.monomials,
            &self.columns,
            &self.common_plan,
            &self.dynamic_plan,
            parameter,
        )
    }

    pub fn specialize(&self, parameter: u64) -> Result<PreparedTarget, Error> {
        self.fresh(parameter)
    }

    pub fn original_equations(&self, parameter: u64) -> Result<Vec<Poly>, Error> {
        original_equations(&self.field, self.backend, &self.key, parameter)
    }
}

impl SharedMacaulay {
    /// Prepare the common numerical row space and each projected coefficient
    /// matrix. Metadata preparation is identical to [`FreshMacaulay::prepare`].
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        p: u64,
        variables: usize,
        degree: u32,
        common: &[Poly],
        pencil: &[Vec<Poly>],
        limits: Limits,
        backend: Backend,
    ) -> Result<Self, Error> {
        let FreshMacaulay {
            field,
            backend,
            limits,
            key,
            monomials,
            columns,
            common_plan,
            dynamic_plan,
            report: metadata,
        } = FreshMacaulay::prepare(p, variables, degree, common, pencil, limits, backend)?;
        let common = &key.common;
        let pencil = &key.pencil;
        let width = monomials.len();
        let powers = metadata.parameter_powers;
        let common_cells = product_size(common_plan.len(), width)?;
        let dynamic_cells = product_size(dynamic_plan.len(), width)?;
        // Rank-independent conservative bound before the first matrix allocation:
        // common RREF + [I|B] block + all projected coefficients + a split online
        // block + a fresh combined matrix + a residual working copy.
        let coefficient_cells = product_size(powers, dynamic_cells)?;
        let full_cells = product_size(common_plan.len() + dynamic_plan.len(), width)?;
        let cells = checked_cells(
            &[
                common_cells,
                common_cells,
                coefficient_cells,
                dynamic_cells,
                full_cells,
                dynamic_cells,
            ],
            limits.max_cells,
        )?;
        let mut common_basis = vec![0; common_cells];
        let mut assembly_terms = 0;
        for (row, plan) in common_plan.iter().enumerate() {
            for (exponent, coefficient) in &common[plan.generator] {
                let column = shifted_column(exponent, &plan.multiplier, &columns);
                common_basis[row * width + column] = *coefficient as u32;
                assembly_terms += 1;
            }
        }
        let mut budget = ProductBudget {
            used: 0,
            limit: limits.max_scalar_products,
        };
        let mut common_elimination = ArithmeticCounts::default();
        let static_pivots = rref(
            &mut common_basis,
            common_plan.len(),
            width,
            &field,
            backend,
            &mut budget,
            &mut common_elimination,
        )?;
        let rank = static_pivots.len();
        let mut pivot_position = vec![None; width];
        for (position, &column) in static_pivots.iter().enumerate() {
            pivot_position[column] = Some(position);
        }
        let free: Vec<_> = (0..width)
            .filter(|&i| pivot_position[i].is_none())
            .collect();
        let mut free_position = vec![None; width];
        for (position, &column) in free.iter().enumerate() {
            free_position[column] = Some(position);
        }
        let mut b = Vec::with_capacity(rank * free.len());
        for row in 0..rank {
            b.extend(
                free.iter()
                    .map(|&column| common_basis[row * width + column]),
            );
        }
        let mut projected = Vec::with_capacity(powers);
        let mut coefficient_projection = ArithmeticCounts::default();
        for power in 0..powers {
            let mut a = vec![0; dynamic_plan.len() * rank];
            let mut c = vec![0; dynamic_plan.len() * free.len()];
            for (row, plan) in dynamic_plan.iter().enumerate() {
                if let Some(polynomial) = pencil[plan.generator].get(power) {
                    for (exponent, coefficient) in polynomial {
                        let column = shifted_column(exponent, &plan.multiplier, &columns);
                        if let Some(position) = pivot_position[column] {
                            a[row * rank + position] = *coefficient as u32;
                        } else {
                            c[row * free.len() + free_position[column].expect("free column")] =
                                *coefficient as u32;
                        }
                        assembly_terms += 1;
                    }
                }
            }
            if rank != 0 && !dynamic_plan.is_empty() && !free.is_empty() {
                let products = (dynamic_plan.len() as u64) * (rank as u64) * (free.len() as u64);
                budget.charge(products)?;
                let kernel = field
                    .sub_mul_dense(
                        &mut c,
                        &a,
                        &b,
                        DenseShape {
                            rows: dynamic_plan.len(),
                            inner: rank,
                            columns: free.len(),
                        },
                        backend,
                    )
                    .map_err(|error| Error(error.to_string()))?;
                coefficient_projection.logical_products += kernel.field_multiplications;
                coefficient_projection.coefficient_additions += kernel.field_additions;
                coefficient_projection.kernels += kernel;
            }
            projected.push(c);
        }
        let report = SetupReport {
            omitted_common_generators: metadata.omitted_common_generators,
            omitted_pencil_generators: metadata.omitted_pencil_generators,
            metadata,
            common_rows: common_plan.len(),
            dynamic_rows: dynamic_plan.len(),
            parameter_powers: powers,
            monomials: width,
            static_rank: rank,
            free_columns: free.len(),
            matrix_assembly_terms: assembly_terms,
            common_elimination,
            coefficient_projection,
            projected_coefficient_cells: powers * dynamic_plan.len() * free.len(),
            matrix_payload_bytes_upper_bound: cells * std::mem::size_of::<u32>(),
            cost_counter_coverage_complete: false,
        };
        Ok(Self {
            field,
            backend,
            limits,
            key,
            monomials,
            columns,
            common_plan,
            dynamic_plan,
            common_basis,
            static_pivots,
            free,
            projected,
            report,
        })
    }

    pub fn key(&self) -> &FamilyKey {
        &self.key
    }
    pub fn setup_report(&self) -> &SetupReport {
        &self.report
    }
    pub fn monomials(&self) -> &[Vec<u32>] {
        &self.monomials
    }

    fn parameter(&self, value: u64) -> Result<u32, Error> {
        if value >= self.key.modulus {
            Err(Error("parameter must be a canonical field element".into()))
        } else {
            Ok(value as u32)
        }
    }

    /// Recombine the projected pencil, then derive its residual row basis.
    pub fn specialize(&self, parameter: u64) -> Result<PreparedTarget, Error> {
        let z = self.parameter(parameter)?;
        let mut budget = ProductBudget {
            used: 0,
            limit: self.limits.max_scalar_products,
        };
        let mut combination = ArithmeticCounts::default();
        let cells = self.dynamic_plan.len() * self.free.len();
        let mut residual = self
            .projected
            .last()
            .cloned()
            .unwrap_or_else(|| vec![0; cells]);
        for coefficients in self.projected.iter().rev().skip(1) {
            budget.charge(cells as u64)?;
            for (value, coefficient) in residual.iter_mut().zip(coefficients) {
                *value = self
                    .field
                    .add(self.field.mul(*value, z, self.backend), *coefficient);
            }
            combination.logical_products += cells as u64;
            combination.coefficient_additions += cells as u64;
        }
        let mut elimination = ArithmeticCounts::default();
        let residual_pivots = rref(
            &mut residual,
            self.dynamic_plan.len(),
            self.free.len(),
            &self.field,
            self.backend,
            &mut budget,
            &mut elimination,
        )?;
        let all_columns: Vec<_> = (0..self.monomials.len()).collect();
        let mut consequences = rows_to_polys(&self.common_basis, &all_columns, &self.monomials);
        consequences.extend(rows_to_polys(&residual, &self.free, &self.monomials));
        let static_rank = self.static_pivots.len();
        let residual_rank = residual_pivots.len();
        Ok(PreparedTarget {
            consequences,
            static_rank,
            residual_rank,
            total_rank: static_rank + residual_rank,
            report: TargetReport {
                strategy: "preprojected",
                parameter,
                matrix_rows_before_reduction: self.dynamic_plan.len(),
                matrix_columns: self.free.len(),
                matrix_assembly_terms: 0,
                parameter_combination: combination,
                elimination,
                cost_counter_coverage_complete: false,
            },
        })
    }

    /// Fresh comparison at the IDENTICAL frozen multiplier envelope. This
    /// assembles common rows and the specialized original pencil directly;
    /// it neither uses the cached common basis nor projects coefficient blocks.
    pub fn fresh(&self, parameter: u64) -> Result<PreparedTarget, Error> {
        fresh_target(
            &self.field,
            self.backend,
            self.limits,
            &self.key,
            &self.monomials,
            &self.columns,
            &self.common_plan,
            &self.dynamic_plan,
            parameter,
        )
    }

    /// Original common and specialized dynamic equations, with zero rows
    /// removed. This convenience construction is outside target-report counts;
    /// a complete pipeline charges it in its enclosing solver interval.
    pub fn original_equations(&self, parameter: u64) -> Result<Vec<Poly>, Error> {
        original_equations(&self.field, self.backend, &self.key, parameter)
    }
}

/// Specialize each generator once, before it is copied into multiplier rows.
/// A constant coefficient requires no multiplication by one.
fn specialize_generators(
    field: &ModMatrix32,
    backend: Backend,
    key: &FamilyKey,
    parameter: u64,
    budget: &mut ProductBudget,
    counts: &mut ArithmeticCounts,
) -> Result<Vec<Poly>, Error> {
    if parameter >= key.modulus {
        return Err(Error("parameter must be a canonical field element".into()));
    }
    let length = key.pencil.iter().map(Vec::len).max().unwrap_or(0);
    let mut powers = vec![1_u32; length];
    for k in 1..length {
        budget.charge(1)?;
        powers[k] = field.mul(powers[k - 1], parameter as u32, backend);
        counts.logical_products += 1;
    }
    let mut specialized = Vec::with_capacity(key.pencil.len());
    for generator in &key.pencil {
        let mut terms = BTreeMap::<Vec<u32>, u32>::new();
        for (power, polynomial) in generator.iter().enumerate() {
            for (exponent, coefficient) in polynomial {
                let value = if power == 0 {
                    *coefficient as u32
                } else {
                    budget.charge(1)?;
                    counts.logical_products += 1;
                    field.mul(*coefficient as u32, powers[power], backend)
                };
                if value != 0 {
                    match terms.entry(exponent.clone()) {
                        std::collections::btree_map::Entry::Vacant(entry) => {
                            entry.insert(value);
                        }
                        std::collections::btree_map::Entry::Occupied(mut entry) => {
                            let old = *entry.get();
                            *entry.get_mut() = field.add(old, value);
                            counts.coefficient_additions += 1;
                        }
                    }
                }
            }
        }
        specialized.push(
            terms
                .into_iter()
                .filter(|(_, value)| *value != 0)
                .map(|(exponent, value)| (exponent, u64::from(value)))
                .collect(),
        );
    }
    Ok(specialized)
}

fn original_equations(
    field: &ModMatrix32,
    backend: Backend,
    key: &FamilyKey,
    parameter: u64,
) -> Result<Vec<Poly>, Error> {
    // At most the admitted input term count and power count are multiplied.
    let mut budget = ProductBudget {
        used: 0,
        limit: 64 * 1024 * 1024,
    };
    let mut counts = ArithmeticCounts::default();
    let mut equations = key.common.clone();
    equations.extend(
        specialize_generators(field, backend, key, parameter, &mut budget, &mut counts)?
            .into_iter()
            .filter(|poly| !poly.is_empty()),
    );
    for polynomial in &mut equations {
        polynomial.sort_by(|(a, _), (b, _)| {
            let da: u32 = a.iter().sum();
            let db: u32 = b.iter().sum();
            db.cmp(&da).then_with(|| b.cmp(a))
        });
    }
    Ok(equations)
}

#[allow(clippy::too_many_arguments)]
fn fresh_target(
    field: &ModMatrix32,
    backend: Backend,
    limits: Limits,
    key: &FamilyKey,
    monomials: &[Vec<u32>],
    columns: &BTreeMap<Vec<u32>, usize>,
    common_plan: &[RowPlan],
    dynamic_plan: &[RowPlan],
    parameter: u64,
) -> Result<PreparedTarget, Error> {
    let mut budget = ProductBudget {
        used: 0,
        limit: limits.max_scalar_products,
    };
    let mut combination = ArithmeticCounts::default();
    let generators = specialize_generators(
        field,
        backend,
        key,
        parameter,
        &mut budget,
        &mut combination,
    )?;
    let width = monomials.len();
    let rows = common_plan.len() + dynamic_plan.len();
    let mut matrix = vec![0; rows * width];
    let mut assembly_terms = 0;
    for (row, plan) in common_plan.iter().enumerate() {
        for (exponent, coefficient) in &key.common[plan.generator] {
            let column = shifted_column(exponent, &plan.multiplier, columns);
            matrix[row * width + column] = *coefficient as u32;
            assembly_terms += 1;
        }
    }
    for (dynamic_row, plan) in dynamic_plan.iter().enumerate() {
        let row = common_plan.len() + dynamic_row;
        for (exponent, coefficient) in &generators[plan.generator] {
            let column = shifted_column(exponent, &plan.multiplier, columns);
            matrix[row * width + column] = *coefficient as u32;
            assembly_terms += 1;
        }
    }
    let mut elimination = ArithmeticCounts::default();
    let (pivots, static_rank) = rref_with_prefix(
        &mut matrix,
        rows,
        width,
        common_plan.len(),
        field,
        backend,
        &mut budget,
        &mut elimination,
    )?;
    let total_rank = pivots.len();
    let all_columns: Vec<_> = (0..width).collect();
    Ok(PreparedTarget {
        consequences: rows_to_polys(&matrix, &all_columns, monomials),
        static_rank,
        residual_rank: total_rank - static_rank,
        total_rank,
        report: TargetReport {
            strategy: "fresh",
            parameter,
            matrix_rows_before_reduction: rows,
            matrix_columns: width,
            matrix_assembly_terms: assembly_terms,
            parameter_combination: combination,
            elimination,
            cost_counter_coverage_complete: false,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn poly(terms: &[(&[u32], u64)]) -> Poly {
        terms.iter().map(|(e, c)| (e.to_vec(), *c)).collect()
    }

    // Independent integer Gauss elimination, deliberately not using the
    // float kernel, the projection, or this module's RREF implementation.
    fn integer_rref(polynomials: &[Poly], monomials: &[Vec<u32>], p: u64) -> Vec<Vec<u64>> {
        fn pow(mut a: u64, mut n: u64, p: u64) -> u64 {
            let mut r = 1;
            while n != 0 {
                if n & 1 != 0 {
                    r = ((r as u128 * a as u128) % p as u128) as u64;
                }
                a = ((a as u128 * a as u128) % p as u128) as u64;
                n >>= 1;
            }
            r
        }
        let mut rows: Vec<Vec<u64>> = polynomials
            .iter()
            .map(|polynomial| {
                monomials
                    .iter()
                    .map(|m| {
                        polynomial
                            .iter()
                            .filter(|(e, _)| e == m)
                            .map(|(_, c)| *c)
                            .sum::<u64>()
                            % p
                    })
                    .collect()
            })
            .collect();
        let mut rank = 0;
        for column in 0..monomials.len() {
            let Some(k) = (rank..rows.len()).find(|&k| rows[k][column] != 0) else {
                continue;
            };
            rows.swap(rank, k);
            let inverse = pow(rows[rank][column], p - 2, p);
            for value in &mut rows[rank] {
                *value = ((*value as u128 * inverse as u128) % p as u128) as u64;
            }
            let pivot = rows[rank].clone();
            for (index, row) in rows.iter_mut().enumerate() {
                if index == rank {
                    continue;
                }
                let factor = row[column];
                for (value, &coefficient) in row.iter_mut().zip(&pivot) {
                    let product = ((factor as u128 * coefficient as u128) % p as u128) as u64;
                    *value = (*value + p - product) % p;
                }
            }
            rank += 1;
        }
        rows.truncate(rank);
        rows
    }

    fn eval(poly: &Poly, xs: &[u64], p: u64) -> u64 {
        poly.iter().fold(0, |sum, (exponent, coefficient)| {
            let mut value = *coefficient;
            for (&x, &degree) in xs.iter().zip(exponent) {
                for _ in 0..degree {
                    value = ((value as u128 * x as u128) % p as u128) as u64;
                }
            }
            (sum + value) % p
        })
    }

    /// Independent source-to-row expansion using the declared monomial
    /// envelope, without the production row plan or shifted-column lookup.
    fn reference_macaulay_rows(
        common: &[Poly],
        pencil: &[Vec<Poly>],
        monomials: &[Vec<u32>],
        degree: u32,
        parameter: u64,
        p: u64,
    ) -> Vec<Poly> {
        let mut generators: Vec<(Poly, u32)> = common
            .iter()
            .filter(|poly| !poly.is_empty())
            .map(|poly| {
                let maximum = poly.iter().map(|(e, _)| e.iter().sum()).max().unwrap();
                (poly.clone(), maximum)
            })
            .collect();
        for generator in pencil {
            let Some(maximum) = generator
                .iter()
                .flatten()
                .filter(|(_, coefficient)| *coefficient != 0)
                .map(|(e, _)| e.iter().sum::<u32>())
                .max()
            else {
                continue;
            };
            let mut power = 1_u64;
            let mut specialized = Vec::new();
            for coefficient in generator {
                for (exponent, value) in coefficient {
                    let value = ((*value as u128 * power as u128) % p as u128) as u64;
                    specialized.push((exponent.clone(), value));
                }
                power = ((power as u128 * parameter as u128) % p as u128) as u64;
            }
            generators.push((specialized, maximum));
        }
        let mut rows = Vec::new();
        for (generator, maximum) in generators {
            if maximum > degree {
                continue;
            }
            for multiplier in monomials {
                if multiplier.iter().sum::<u32>() > degree - maximum {
                    continue;
                }
                rows.push(
                    generator
                        .iter()
                        .map(|(exponent, coefficient)| {
                            (
                                exponent
                                    .iter()
                                    .zip(multiplier)
                                    .map(|(a, b)| a + b)
                                    .collect(),
                                *coefficient,
                            )
                        })
                        .collect(),
                );
            }
        }
        rows
    }

    #[test]
    fn parameter_coefficients_are_combined_before_elimination() {
        // x - t has a root for EVERY t. Separately reducing the coefficient
        // equations x and -1 would falsely produce the unit ideal.
        for backend in [Backend::Integer, Backend::Fma, Backend::Split16] {
            let pencil = vec![vec![poly(&[(&[1], 1)]), poly(&[(&[0], 16)])]];
            let family =
                SharedMacaulay::prepare(17, 1, 1, &[], &pencil, Limits::default(), backend)
                    .unwrap();
            for t in 0..17 {
                let cached = family.specialize(t).unwrap();
                let fresh = family.fresh(t).unwrap();
                assert_eq!(cached.total_rank, 1);
                assert_eq!(cached.total_rank, fresh.total_rank);
                assert!(cached.consequences.iter().all(|f| eval(f, &[t], 17) == 0));
                assert_eq!(
                    integer_rref(&cached.consequences, family.monomials(), 17),
                    integer_rref(&fresh.consequences, family.monomials(), 17)
                );
            }
        }
    }

    #[test]
    fn exact_projection_matches_independent_rowspaces_and_all_small_roots() {
        // x^2=1, y^2=1, x*y + t*x + t^2*y = 0 over F7.
        let common = vec![
            poly(&[(&[2, 0], 1), (&[0, 0], 6)]),
            poly(&[(&[0, 2], 1), (&[0, 0], 6)]),
        ];
        let pencil = vec![vec![
            poly(&[(&[1, 1], 1)]),
            poly(&[(&[1, 0], 1)]),
            poly(&[(&[0, 1], 1)]),
        ]];
        let mut reference_setup = None;
        for backend in [Backend::Integer, Backend::Fma, Backend::Split16] {
            let family =
                SharedMacaulay::prepare(7, 2, 3, &common, &pencil, Limits::default(), backend)
                    .unwrap();
            assert!(
                family
                    .setup_report()
                    .coefficient_projection
                    .kernels
                    .matrix_products
                    > 0
            );
            let rank = family.setup_report().static_rank;
            if let Some(previous) = reference_setup {
                assert_eq!(rank, previous);
            }
            reference_setup = Some(rank);
            for t in 0..7 {
                let cached = family.specialize(t).unwrap();
                let fresh = family.fresh(t).unwrap();
                let a = integer_rref(&cached.consequences, family.monomials(), 7);
                let b = integer_rref(&fresh.consequences, family.monomials(), 7);
                assert_eq!(a, b, "backend={backend:?}, t={t}");
                let independently_assembled =
                    reference_macaulay_rows(&common, &pencil, family.monomials(), 3, t, 7);
                assert_eq!(
                    a,
                    integer_rref(&independently_assembled, family.monomials(), 7),
                    "independent source-row assembly, backend={backend:?}, t={t}"
                );
                assert_eq!(cached.total_rank, a.len());
                assert_eq!(cached.total_rank, fresh.total_rank);
                let original = family.original_equations(t).unwrap();
                for x in 0..7 {
                    for y in 0..7 {
                        if original.iter().all(|f| eval(f, &[x, y], 7) == 0) {
                            assert!(cached.consequences.iter().all(|f| eval(f, &[x, y], 7) == 0));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn rank_drop_and_vanishing_leading_support_use_frozen_envelope() {
        let pencil = vec![vec![Vec::new(), poly(&[(&[1], 1)])]];
        let family =
            SharedMacaulay::prepare(17, 1, 2, &[], &pencil, Limits::default(), Backend::Fma)
                .unwrap();
        assert_eq!(family.specialize(0).unwrap().total_rank, 0);
        assert_eq!(family.fresh(0).unwrap().total_rank, 0);
        assert_eq!(family.specialize(1).unwrap().total_rank, 2);
        // x + t*x^2 has one permitted multiplier at degree 2, including t=0.
        let pencil = vec![vec![poly(&[(&[1], 1)]), poly(&[(&[2], 1)])]];
        let family =
            SharedMacaulay::prepare(17, 1, 2, &[], &pencil, Limits::default(), Backend::Split16)
                .unwrap();
        assert_eq!(family.specialize(0).unwrap().total_rank, 1);
        assert_eq!(family.fresh(0).unwrap().total_rank, 1);
        assert_eq!(family.setup_report().dynamic_rows, 1);
    }

    #[test]
    fn rank_drop_after_nontrivial_common_projection_matches_fresh() {
        // Modulo x-y, the dynamic equations y+t and y+1 coincide exactly at
        // t=1. At other targets their difference supplies a nonzero constant.
        let common = vec![poly(&[(&[1, 0], 1), (&[0, 1], 16)])];
        let pencil = vec![
            vec![poly(&[(&[0, 1], 1)]), poly(&[(&[0, 0], 1)])],
            vec![poly(&[(&[0, 1], 1), (&[0, 0], 1)])],
        ];
        for backend in [Backend::Integer, Backend::Fma, Backend::Split16] {
            let shared =
                SharedMacaulay::prepare(17, 2, 1, &common, &pencil, Limits::default(), backend)
                    .unwrap();
            let fresh =
                FreshMacaulay::prepare(17, 2, 1, &common, &pencil, Limits::default(), backend)
                    .unwrap();
            assert_eq!(shared.key(), fresh.key());
            for t in 0..17 {
                let a = shared.specialize(t).unwrap();
                let b = fresh.specialize(t).unwrap();
                assert_eq!(a.static_rank, 1);
                assert_eq!(a.static_rank, b.static_rank);
                assert_eq!(a.total_rank, if t == 1 { 2 } else { 3 });
                assert_eq!(a.total_rank, b.total_rank);
                let independently_assembled =
                    reference_macaulay_rows(&common, &pencil, shared.monomials(), 1, t, 17);
                let reference = integer_rref(&independently_assembled, shared.monomials(), 17);
                assert_eq!(
                    integer_rref(&a.consequences, shared.monomials(), 17),
                    reference
                );
                assert_eq!(
                    integer_rref(&b.consequences, shared.monomials(), 17),
                    reference
                );
            }
        }
    }

    #[test]
    fn standalone_fresh_survives_shared_specific_setup_refusals() {
        let common = vec![poly(&[(&[1, 0], 1), (&[0, 1], 16)])];
        let pencil = vec![vec![Vec::new(), poly(&[(&[1, 0], 1)])]];
        let limits = Limits {
            max_scalar_products: 6,
            ..Limits::default()
        };
        let fresh =
            FreshMacaulay::prepare(17, 2, 1, &common, &pencil, limits, Backend::Integer).unwrap();
        assert!(!fresh.setup_report().numerical_elimination_performed);
        assert!(!fresh.setup_report().numerical_projection_performed);
        // At t=0 the dynamic row vanishes. Fresh specialization and one common
        // pivot cost five logical products. Shared coefficient projections
        // exceed the six-product setup allowance before any target is known.
        let target = fresh.fresh(0).unwrap();
        assert_eq!(target.total_rank, 1);
        assert_eq!(
            target.report.parameter_combination.logical_products
                + target.report.elimination.logical_products,
            5
        );
        assert!(
            SharedMacaulay::prepare(17, 2, 1, &common, &pencil, limits, Backend::Integer)
                .err()
                .unwrap()
                .0
                .contains("product budget")
        );
        let limits = Limits {
            max_cells: 10,
            ..Limits::default()
        };
        let fresh =
            FreshMacaulay::prepare(17, 2, 1, &common, &pencil, limits, Backend::Integer).unwrap();
        assert_eq!(fresh.fresh(0).unwrap().total_rank, 1);
        assert!(
            SharedMacaulay::prepare(17, 2, 1, &common, &pencil, limits, Backend::Integer).is_err()
        );
    }

    #[test]
    fn canonical_keys_bind_every_family_input() {
        let a = poly(&[(&[1], 1), (&[0], 16)]);
        let b = poly(&[(&[0], 8), (&[1], 1), (&[0], 8), (&[0], 0)]);
        let first = SharedMacaulay::prepare(
            17,
            1,
            2,
            std::slice::from_ref(&a),
            &[],
            Limits::default(),
            Backend::Integer,
        )
        .unwrap();
        let second =
            SharedMacaulay::prepare(17, 1, 2, &[b, a], &[], Limits::default(), Backend::Fma)
                .unwrap();
        assert_eq!(first.key(), second.key());
        let different = SharedMacaulay::prepare(
            17,
            1,
            2,
            &[poly(&[(&[1], 1), (&[0], 15)])],
            &[],
            Limits::default(),
            Backend::Integer,
        )
        .unwrap();
        assert_ne!(first.key(), different.key());
        assert!(first.specialize(17).is_err());
        assert!(first.fresh(u64::MAX).is_err());
    }

    #[test]
    fn malformed_inputs_and_resource_caps_fail_before_solution_claims() {
        let empty: Vec<Poly> = Vec::new();
        for p in [0, 1, 4, 15, 1u64 << 32] {
            assert!(SharedMacaulay::prepare(
                p,
                1,
                1,
                &empty,
                &[],
                Limits::default(),
                Backend::Integer
            )
            .is_err());
        }
        for polynomial in [
            poly(&[(&[1], 17)]),
            poly(&[(&[1, 0], 1)]),
            poly(&[(&[u32::MAX, u32::MAX], 1)]),
        ] {
            assert!(SharedMacaulay::prepare(
                17,
                1,
                1,
                &[polynomial],
                &[],
                Limits::default(),
                Backend::Integer
            )
            .is_err());
        }
        let tiny = Limits {
            max_monomials: 3,
            ..Limits::default()
        };
        assert!(SharedMacaulay::prepare(17, 2, 3, &empty, &[], tiny, Backend::Integer).is_err());
        let tiny = Limits {
            max_cells: 1,
            ..Limits::default()
        };
        assert!(SharedMacaulay::prepare(
            17,
            1,
            2,
            &[poly(&[(&[1], 1)])],
            &[],
            tiny,
            Backend::Integer
        )
        .is_err());
        let tiny = Limits {
            max_rows: 1,
            ..Limits::default()
        };
        assert!(SharedMacaulay::prepare(
            17,
            1,
            2,
            &[poly(&[(&[1], 1)])],
            &[],
            tiny,
            Backend::Integer
        )
        .is_err());
        let tiny = Limits {
            max_scalar_products: 1,
            ..Limits::default()
        };
        assert!(SharedMacaulay::prepare(
            17,
            1,
            2,
            &[poly(&[(&[1], 1)])],
            &[],
            tiny,
            Backend::Integer
        )
        .is_err());
        let tiny = Limits {
            max_input_terms: 2,
            ..Limits::default()
        };
        assert!(SharedMacaulay::prepare(
            17,
            1,
            1,
            &[],
            &[vec![Vec::new(), Vec::new(), Vec::new()]],
            tiny,
            Backend::Integer,
        )
        .is_err());
    }

    #[test]
    fn omitted_high_degree_generators_remain_in_originals() {
        let original = poly(&[(&[3], 1), (&[0], 16)]);
        let family = SharedMacaulay::prepare(
            17,
            1,
            1,
            std::slice::from_ref(&original),
            &[],
            Limits::default(),
            Backend::Integer,
        )
        .unwrap();
        assert_eq!(family.setup_report().omitted_common_generators, 1);
        assert_eq!(family.specialize(2).unwrap().total_rank, 0);
        let returned = family.original_equations(2).unwrap();
        assert_eq!(returned.len(), 1);
        assert_eq!(
            canonical_poly(&returned[0], 1, &family.field).unwrap(),
            canonical_poly(&original, 1, &family.field).unwrap()
        );
    }

    #[test]
    fn full_static_rank_and_empty_family_are_well_defined() {
        let family = SharedMacaulay::prepare(
            7,
            1,
            1,
            &[poly(&[(&[0], 1)])],
            &[vec![poly(&[(&[1], 1)])]],
            Limits::default(),
            Backend::Fma,
        )
        .unwrap();
        assert_eq!(family.setup_report().free_columns, 0);
        assert_eq!(family.specialize(0).unwrap().total_rank, 2);
        assert_eq!(family.fresh(0).unwrap().total_rank, 2);
        let empty = SharedMacaulay::prepare(7, 1, 0, &[], &[], Limits::default(), Backend::Integer)
            .unwrap();
        assert_eq!(empty.specialize(0).unwrap().total_rank, 0);
        assert_eq!(empty.fresh(0).unwrap().total_rank, 0);
    }
}
