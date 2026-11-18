//! Exact n=9 gate for the norm-fiber Riemann--Roch router.
//!
//! The binary exhausts the frozen one-step degree-four representative family
//! on the transported elliptic subgroup and one certified nonzero kernel
//! coset.  It emits raw sparse relations so a separate validator can replay
//! the rank and cost accounting.

use clap::Parser;
use crypto_lib::binary_ecc::curve::{point_add, scalar_mul, BinaryCurve};
use crypto_lib::binary_ecc::{
    BinaryCoverPoint, BinaryPoint, F2mElement, F2mPoly, IrreduciblePoly, MumfordDivisor,
    OrdinaryBinaryCover,
};
use crypto_lib::cryptanalysis::binary_hyperelliptic_index_calculus::split_into_linear_factors;
use crypto_lib::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

const M: u32 = 9;
const Q: u16 = 512;
const R: u16 = 127;
const COFACTOR: u16 = 4;
const BOUNDS: [usize; 6] = [8, 16, 32, 64, 96, 126];
const AMBIENT_SEED: u64 = 0x8309_2026_1118_0001;
const BLIND_SEED: u64 = 0x8309_2026_1118_10a1;

#[derive(Parser)]
#[command(
    name = "n83-norm-fiber-rr-router-audit",
    about = "Run the frozen exact n=9 norm-fiber Riemann--Roch gate"
)]
struct Args {
    /// Write the deterministic certificate to this path; omit for stdout.
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct PointKey {
    infinity: bool,
    x: u16,
    y: u16,
}

#[derive(Clone, Debug, Serialize)]
struct FactorBaseEntry {
    index: usize,
    point: PointKey,
    discrete_log: u16,
}

#[derive(Clone, Debug, Serialize)]
struct RelationRecord {
    class_index: usize,
    constant: u16,
    rhs_log: u16,
    /// Four cofactor-projected component labels, encoded by their n=9 log.
    /// Zero is the identity and creates no matrix column.
    projected_label_logs: Vec<u16>,
}

#[derive(Clone, Debug, Default, Serialize)]
struct CandidateCounts {
    candidates: u64,
    exact_degree_four: u64,
    nonsplit: u64,
    split: u64,
    rejections: BTreeMap<String, u64>,
    consistency_failures: u64,
}

#[derive(Clone, Debug, Serialize)]
struct GridResult {
    factor_base_bound: usize,
    rows: usize,
    columns: usize,
    nonzeros: usize,
    rank: usize,
    rank_per_row: f64,
    rank_per_candidate: f64,
    relation_probability_per_candidate: f64,
    relation_probability_per_exact_representation: f64,
    raw_projected_components: usize,
    raw_nonidentity_components: usize,
    sum_distinct_labels_including_identity: usize,
    sum_distinct_matrix_labels: usize,
    global_distinct_matrix_labels: usize,
    projected_collision_count_including_identity: usize,
    projected_collision_rate_including_identity: f64,
    csr_bytes_u32_columns_u16_values: usize,
    consistent: bool,
    full_rank: bool,
    solution_matches_enumerated_logs: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
struct ArmResult {
    name: String,
    class_count: usize,
    counts: CandidateCounts,
    grid: Vec<GridResult>,
    relations: Vec<RelationRecord>,
    elapsed_milliseconds: u128,
}

#[derive(Clone, Debug, Serialize)]
struct BlindTrial {
    target_index: usize,
    planted_log_for_verification: u16,
    recovered_log: Option<u16>,
    success: bool,
    masks_tried: usize,
    candidates_tried: usize,
}

#[derive(Clone, Debug, Serialize)]
struct BlindResult {
    arm: String,
    factor_base_bound: Option<usize>,
    trials: Vec<BlindTrial>,
    successes: usize,
    total_candidates: usize,
}

#[derive(Clone, Debug, Serialize)]
struct DivisorRecord {
    u: Vec<String>,
    v: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
struct Certificate {
    schema: &'static str,
    status: &'static str,
    claim_scope: &'static str,
    protocol_registration: BTreeMap<&'static str, &'static str>,
    field: FieldRecord,
    subgroup: SubgroupRecord,
    kernel: KernelRecord,
    factor_base: Vec<FactorBaseEntry>,
    arms: Vec<ArmResult>,
    blind_individual_logs: Vec<BlindResult>,
    advancement_gate: AdvancementGate,
}

#[derive(Clone, Debug, Serialize)]
struct FieldRecord {
    degree: u32,
    cardinality: u16,
    modulus_nonleading_terms: Vec<u32>,
    enumerated_target_group_order: usize,
    predicted_target_group_order: u16,
    curve: &'static str,
    cover: &'static str,
}

#[derive(Clone, Debug, Serialize)]
struct SubgroupRecord {
    order: u16,
    cofactor: u16,
    generator: PointKey,
    unique_multiples: usize,
    order_check: bool,
}

#[derive(Clone, Debug, Serialize)]
struct KernelRecord {
    seed_source_point: PointKey,
    seed_divisor: DivisorRecord,
    kernel_divisor: DivisorRecord,
    kernel_nonidentity: bool,
    kernel_pushforward_is_identity: bool,
    seed_pushforward_is_identity: bool,
}

#[derive(Clone, Debug, Serialize)]
struct AdvancementGate {
    nonzero_image_compatible_rank: bool,
    full_rank_system: bool,
    blind_success_at_least_15_of_16_each_eligible_arm: bool,
    consistency_checks_pass: bool,
    finite_crossover_against_source_rho_predicted: bool,
    advance_to_n13: bool,
    reason: &'static str,
    registered_generic_collection_exponent_in_q: f64,
    registered_generic_matrix_exponent_in_q: f64,
    registered_balanced_total_exponent_in_q: f64,
    source_rho_exponent_in_q: f64,
}

#[derive(Clone)]
struct Subgroup {
    curve: BinaryCurve,
    generator: BinaryPoint,
    multiples: Vec<BinaryPoint>,
    log_by_key: HashMap<PointKey, u16>,
    factor_base: Vec<FactorBaseEntry>,
    fb_index_by_log: Vec<Option<usize>>,
}

#[derive(Clone)]
struct ClassInput {
    divisor: MumfordDivisor,
    rhs_log: u16,
}

enum CandidateOutcome {
    Rejected(&'static str),
    Nonsplit,
    Split(RelationRecord),
}

#[derive(Clone)]
struct SolvedSystem {
    bound: usize,
    solution: Vec<u16>,
}

#[derive(Clone, Copy)]
struct XorShift64(u64);

impl XorShift64 {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
}

fn point_key(point: &BinaryPoint) -> PointKey {
    match point {
        BinaryPoint::Infinity => PointKey {
            infinity: true,
            x: 0,
            y: 0,
        },
        BinaryPoint::Affine { x, y } => PointKey {
            infinity: false,
            x: x.to_biguint().to_u16().expect("n=9 x coordinate"),
            y: y.to_biguint().to_u16().expect("n=9 y coordinate"),
        },
    }
}

fn source_point_key(point: &BinaryCoverPoint) -> PointKey {
    match point {
        BinaryCoverPoint::Infinity => point_key(&BinaryPoint::Infinity),
        BinaryCoverPoint::Affine { u, v } => PointKey {
            infinity: false,
            x: u.to_biguint().to_u16().expect("n=9 u coordinate"),
            y: v.to_biguint().to_u16().expect("n=9 v coordinate"),
        },
    }
}

fn divisor_record(divisor: &MumfordDivisor) -> DivisorRecord {
    let coefficients = |poly: &F2mPoly| {
        poly.coeffs
            .iter()
            .map(|coefficient| format!("0x{:x}", coefficient.to_biguint()))
            .collect()
    };
    DivisorRecord {
        u: coefficients(&divisor.u),
        v: coefficients(&divisor.v),
    }
}

fn add_many(curve: &BinaryCurve, points: impl IntoIterator<Item = BinaryPoint>) -> BinaryPoint {
    points
        .into_iter()
        .fold(BinaryPoint::Infinity, |sum, point| {
            point_add(curve, &sum, &point)
        })
}

fn mod_add(a: u16, b: u16) -> u16 {
    (a + b) % R
}

fn mod_sub(a: u16, b: u16) -> u16 {
    (a + R - b) % R
}

fn mod_mul(a: u16, b: u16) -> u16 {
    ((a as u32 * b as u32) % R as u32) as u16
}

fn mod_inverse(value: u16) -> Option<u16> {
    if value == 0 {
        return None;
    }
    (1..R).find(|candidate| mod_mul(value, *candidate) == 1)
}

fn build_subgroup(
    cover: &OrdinaryBinaryCover,
    irreducible: &IrreduciblePoly,
) -> Result<(Subgroup, usize), String> {
    let placeholder = BinaryCurve {
        m: M,
        irreducible: irreducible.clone(),
        a: F2mElement::zero(M),
        b: F2mElement::one(M),
        generator: BinaryPoint::Infinity,
        order: BigUint::from(R),
        cofactor: BigUint::from(COFACTOR),
    };

    let mut target_points = 1usize; // infinity
    let mut generator = None;
    for x_value in 0..Q {
        let x = F2mElement::from_biguint(&BigUint::from(x_value), M);
        for y_value in 0..Q {
            let y = F2mElement::from_biguint(&BigUint::from(y_value), M);
            let point = BinaryPoint::Affine { x: x.clone(), y };
            if !cover.target_is_on_curve(&point) {
                continue;
            }
            target_points += 1;
            if generator.is_none()
                && scalar_mul(&placeholder, &point, &BigUint::from(R)) == BinaryPoint::Infinity
            {
                generator = Some(point);
            }
        }
    }
    if target_points != 508 {
        return Err(format!(
            "target point enumeration found {target_points}, expected 508"
        ));
    }
    let generator = generator.ok_or_else(|| "no order-127 point found".to_string())?;
    let curve = BinaryCurve {
        generator: generator.clone(),
        ..placeholder
    };
    if generator == BinaryPoint::Infinity
        || scalar_mul(&curve, &generator, &BigUint::from(R)) != BinaryPoint::Infinity
    {
        return Err("selected generator failed exact order check".into());
    }

    let mut multiples = Vec::with_capacity(R as usize);
    let mut log_by_key = HashMap::new();
    for scalar in 0..R {
        let point = scalar_mul(&curve, &generator, &BigUint::from(scalar));
        let key = point_key(&point);
        if log_by_key.insert(key, scalar).is_some() {
            return Err("subgroup multiples are not unique".into());
        }
        multiples.push(point);
    }

    let mut sorted: Vec<(PointKey, u16)> = (1..R)
        .map(|log| (point_key(&multiples[log as usize]), log))
        .collect();
    sorted.sort_unstable_by_key(|(key, _)| *key);
    let factor_base: Vec<_> = sorted
        .into_iter()
        .enumerate()
        .map(|(index, (point, discrete_log))| FactorBaseEntry {
            index,
            point,
            discrete_log,
        })
        .collect();
    let mut fb_index_by_log = vec![None; R as usize];
    for entry in &factor_base {
        fb_index_by_log[entry.discrete_log as usize] = Some(entry.index);
    }

    Ok((
        Subgroup {
            curve,
            generator,
            multiples,
            log_by_key,
            factor_base,
            fb_index_by_log,
        },
        target_points,
    ))
}

fn source_rational_points(cover: &OrdinaryBinaryCover) -> Vec<BinaryCoverPoint> {
    let mut points = Vec::new();
    for u_value in 0..Q {
        let u = F2mElement::from_biguint(&BigUint::from(u_value), M);
        for v_value in 0..Q {
            let v = F2mElement::from_biguint(&BigUint::from(v_value), M);
            let point = BinaryCoverPoint::Affine { u: u.clone(), v };
            if cover.source_is_on_curve(&point) {
                points.push(point);
            }
        }
    }
    points
}

fn projected_log(subgroup: &Subgroup, point: &BinaryPoint) -> Result<u16, String> {
    let projected = scalar_mul(&subgroup.curve, point, &BigUint::from(COFACTOR));
    subgroup
        .log_by_key
        .get(&point_key(&projected))
        .copied()
        .ok_or_else(|| "cofactor-projected point is outside the order-127 subgroup".into())
}

fn find_kernel_class(
    cover: &OrdinaryBinaryCover,
    source_points: &[BinaryCoverPoint],
) -> Result<(BinaryCoverPoint, MumfordDivisor, MumfordDivisor), String> {
    let curve = cover.source_curve();
    for point in source_points {
        let BinaryCoverPoint::Affine { u, v } = point else {
            continue;
        };
        let seed = MumfordDivisor::try_from_point(curve, u, v).map_err(|e| e.to_string())?;
        let three_seed = seed
            .try_scalar_mul(&BigUint::from(3u8), curve)
            .map_err(|e| e.to_string())?;
        let image = cover
            .elliptic_image_endomorphism(&seed)
            .map_err(|e| e.to_string())?;
        let kernel = three_seed
            .try_add(&image.try_neg(curve).map_err(|e| e.to_string())?, curve)
            .map_err(|e| e.to_string())?;
        if kernel.is_identity() {
            continue;
        }
        let pushforward = cover
            .pushforward_arbitrary_jacobian_class(&kernel)
            .map_err(|e| e.to_string())?;
        if pushforward == BinaryPoint::Infinity {
            return Ok((point.clone(), seed, kernel));
        }
    }
    Err("failed to find a nonzero rationally generated kernel class".into())
}

fn build_class_inputs(
    cover: &OrdinaryBinaryCover,
    subgroup: &Subgroup,
    kernel: &MumfordDivisor,
    source_points: &[BinaryCoverPoint],
) -> Result<(Vec<ClassInput>, Vec<ClassInput>, Vec<ClassInput>), String> {
    let curve = cover.source_curve();
    let mut transported = Vec::with_capacity(R as usize);
    let mut kernel_coset = Vec::with_capacity(R as usize);
    for scalar in 0..R {
        let pullback = cover
            .pullback_point_class(&subgroup.multiples[scalar as usize])
            .map_err(|e| e.to_string())?;
        let direct = cover
            .pushforward_arbitrary_jacobian_class(&pullback)
            .map_err(|e| e.to_string())?;
        let rhs = projected_log(subgroup, &direct)?;
        let expected = mod_mul(12, scalar);
        if rhs != expected {
            return Err(format!(
                "transported class {scalar} has RHS {rhs}, expected {expected}"
            ));
        }
        transported.push(ClassInput {
            divisor: pullback.clone(),
            rhs_log: rhs,
        });

        let coset = pullback.try_add(kernel, curve).map_err(|e| e.to_string())?;
        let coset_direct = cover
            .pushforward_arbitrary_jacobian_class(&coset)
            .map_err(|e| e.to_string())?;
        let coset_rhs = projected_log(subgroup, &coset_direct)?;
        if coset_rhs != expected {
            return Err(format!(
                "kernel-coset class {scalar} has RHS {coset_rhs}, expected {expected}"
            ));
        }
        kernel_coset.push(ClassInput {
            divisor: coset,
            rhs_log: coset_rhs,
        });
    }

    let point_divisors: Result<Vec<_>, String> = source_points
        .iter()
        .map(|point| match point {
            BinaryCoverPoint::Infinity => Ok(MumfordDivisor::identity(M)),
            BinaryCoverPoint::Affine { u, v } => {
                MumfordDivisor::try_from_point(curve, u, v).map_err(|e| e.to_string())
            }
        })
        .collect();
    let point_divisors = point_divisors?;
    let mut rng = XorShift64(AMBIENT_SEED);
    let mut ambient = Vec::with_capacity(R as usize);
    for _ in 0..R {
        let mut divisor = MumfordDivisor::identity(M);
        for _ in 0..3 {
            let index = rng.next() as usize % point_divisors.len();
            divisor = divisor
                .try_add(&point_divisors[index], curve)
                .map_err(|e| e.to_string())?;
        }
        let direct = cover
            .pushforward_arbitrary_jacobian_class(&divisor)
            .map_err(|e| e.to_string())?;
        ambient.push(ClassInput {
            divisor,
            rhs_log: projected_log(subgroup, &direct)?,
        });
    }
    Ok((transported, kernel_coset, ambient))
}

fn evaluate_candidate(
    cover: &OrdinaryBinaryCover,
    subgroup: &Subgroup,
    class_index: usize,
    input: &ClassInput,
    constant: u16,
) -> Result<CandidateOutcome, String> {
    let curve = cover.source_curve();
    let c = F2mElement::from_biguint(&BigUint::from(constant), M);
    let v_lift = curve
        .h
        .add(&input.divisor.v)
        .add(&input.divisor.u.scalar_mul(&c, &curve.irr));
    let numerator = curve
        .f
        .add(&curve.h.mul(&v_lift, &curve.irr))
        .add(&v_lift.mul(&v_lift, &curve.irr));
    let (u_effective, remainder) = numerator.divrem(&input.divisor.u, &curve.irr);
    if !remainder.is_zero() {
        return Ok(CandidateOutcome::Rejected("nonexact_quotient"));
    }
    if u_effective.degree() != Some(4) {
        return Ok(CandidateOutcome::Rejected("degree_not_four"));
    }
    if u_effective.lead() != F2mElement::one(M) {
        return Ok(CandidateOutcome::Rejected("nonmonic_effective_support"));
    }
    let reconstructed =
        match MumfordDivisor::try_from_unreduced(curve, u_effective.clone(), v_lift.clone()) {
            Ok(divisor) => divisor,
            Err(_) => return Ok(CandidateOutcome::Rejected("unreduced_pair_rejected")),
        };
    if reconstructed != input.divisor {
        return Ok(CandidateOutcome::Rejected("reconstruction_mismatch"));
    }

    let Some(roots) = split_into_linear_factors(&u_effective, &curve.irr) else {
        return Ok(CandidateOutcome::Nonsplit);
    };
    let multiplicity_sum: usize = roots.iter().map(|(_, multiplicity)| *multiplicity).sum();
    if multiplicity_sum != 4 {
        return Err(format!(
            "split support multiplicities sum to {multiplicity_sum}, expected 4"
        ));
    }

    let mut unprojected = Vec::with_capacity(4);
    let mut projected_label_logs = Vec::with_capacity(4);
    for (root, multiplicity) in roots {
        let value = v_lift.eval(&root, &curve.irr);
        let source_point = BinaryCoverPoint::Affine { u: root, v: value };
        if !cover.source_is_on_curve(&source_point) {
            return Err("split support produced an off-curve source point".into());
        }
        let image = cover.map_point(&source_point).map_err(|e| e.to_string())?;
        let label_log = projected_log(subgroup, &image)?;
        for _ in 0..multiplicity {
            unprojected.push(image.clone());
            projected_label_logs.push(label_log);
        }
    }
    let component_sum = add_many(&subgroup.curve, unprojected);
    let direct = cover
        .pushforward_arbitrary_jacobian_class(&input.divisor)
        .map_err(|e| e.to_string())?;
    if component_sum != direct {
        return Err("split component sum differs from direct pushforward".into());
    }
    let projected_sum_log = projected_label_logs.iter().copied().fold(0u16, mod_add);
    if projected_sum_log != input.rhs_log {
        return Err(format!(
            "projected label sum {projected_sum_log} differs from RHS {}",
            input.rhs_log
        ));
    }
    Ok(CandidateOutcome::Split(RelationRecord {
        class_index,
        constant,
        rhs_log: input.rhs_log,
        projected_label_logs,
    }))
}

fn row_for_bound(relation: &RelationRecord, subgroup: &Subgroup, bound: usize) -> Option<Vec<u16>> {
    let mut row = vec![0u16; bound];
    for log in &relation.projected_label_logs {
        if *log == 0 {
            continue;
        }
        let index = subgroup.fb_index_by_log[*log as usize]?;
        if index >= bound {
            return None;
        }
        row[index] = mod_add(row[index], 1);
    }
    Some(row)
}

fn eliminate(mut rows: Vec<Vec<u16>>, mut rhs: Vec<u16>) -> (usize, bool, Option<Vec<u16>>) {
    if rows.is_empty() {
        return (0, rhs.iter().all(|value| *value == 0), None);
    }
    let columns = rows[0].len();
    let mut pivot_row = 0usize;
    let mut pivot_columns = Vec::new();
    for column in 0..columns {
        let Some(found) = (pivot_row..rows.len()).find(|row| rows[*row][column] != 0) else {
            continue;
        };
        rows.swap(pivot_row, found);
        rhs.swap(pivot_row, found);
        let inverse = mod_inverse(rows[pivot_row][column]).expect("nonzero pivot");
        for entry in &mut rows[pivot_row][column..] {
            *entry = mod_mul(*entry, inverse);
        }
        rhs[pivot_row] = mod_mul(rhs[pivot_row], inverse);

        let pivot = rows[pivot_row].clone();
        let pivot_rhs = rhs[pivot_row];
        for row in 0..rows.len() {
            if row == pivot_row || rows[row][column] == 0 {
                continue;
            }
            let factor = rows[row][column];
            for index in column..columns {
                rows[row][index] = mod_sub(rows[row][index], mod_mul(factor, pivot[index]));
            }
            rhs[row] = mod_sub(rhs[row], mod_mul(factor, pivot_rhs));
        }
        pivot_columns.push(column);
        pivot_row += 1;
        if pivot_row == rows.len() {
            break;
        }
    }
    let consistent = rows
        .iter()
        .zip(&rhs)
        .all(|(row, value)| row.iter().any(|coefficient| *coefficient != 0) || *value == 0);
    let solution = if consistent && pivot_columns.len() == columns {
        let mut values = vec![0u16; columns];
        for (row, column) in pivot_columns.iter().copied().enumerate() {
            values[column] = rhs[row];
        }
        Some(values)
    } else {
        None
    };
    (pivot_columns.len(), consistent, solution)
}

fn summarize_grid(
    relations: &[RelationRecord],
    counts: &CandidateCounts,
    subgroup: &Subgroup,
) -> (Vec<GridResult>, Vec<SolvedSystem>) {
    let mut grid = Vec::new();
    let mut solved = Vec::new();
    for bound in BOUNDS {
        let selected: Vec<_> = relations
            .iter()
            .filter_map(|relation| {
                row_for_bound(relation, subgroup, bound).map(|row| (relation, row))
            })
            .collect();
        let rows: Vec<Vec<u16>> = selected.iter().map(|(_, row)| row.clone()).collect();
        let rhs: Vec<u16> = selected
            .iter()
            .map(|(relation, _)| relation.rhs_log)
            .collect();
        let nonzeros = rows
            .iter()
            .map(|row| row.iter().filter(|value| **value != 0).count())
            .sum();
        let (rank, consistent, solution) = eliminate(rows, rhs);
        let solution_matches = solution.as_ref().map(|values| {
            values
                .iter()
                .enumerate()
                .all(|(index, value)| *value == subgroup.factor_base[index].discrete_log)
        });
        if let Some(values) = solution.clone() {
            solved.push(SolvedSystem {
                bound,
                solution: values,
            });
        }

        let mut raw_nonidentity = 0usize;
        let mut distinct_including_identity_sum = 0usize;
        let mut distinct_matrix_sum = 0usize;
        let mut global = BTreeSet::new();
        for (relation, _) in &selected {
            let all: BTreeSet<_> = relation.projected_label_logs.iter().copied().collect();
            let matrix: BTreeSet<_> = relation
                .projected_label_logs
                .iter()
                .copied()
                .filter(|log| *log != 0)
                .collect();
            raw_nonidentity += relation
                .projected_label_logs
                .iter()
                .filter(|log| **log != 0)
                .count();
            distinct_including_identity_sum += all.len();
            distinct_matrix_sum += matrix.len();
            global.extend(matrix);
        }
        let raw_components = selected.len() * 4;
        let collisions = raw_components.saturating_sub(distinct_including_identity_sum);
        let rows_len = selected.len();
        grid.push(GridResult {
            factor_base_bound: bound,
            rows: rows_len,
            columns: bound,
            nonzeros,
            rank,
            rank_per_row: ratio(rank, rows_len),
            rank_per_candidate: ratio(rank, counts.candidates as usize),
            relation_probability_per_candidate: ratio(rows_len, counts.candidates as usize),
            relation_probability_per_exact_representation: ratio(
                rows_len,
                counts.exact_degree_four as usize,
            ),
            raw_projected_components: raw_components,
            raw_nonidentity_components: raw_nonidentity,
            sum_distinct_labels_including_identity: distinct_including_identity_sum,
            sum_distinct_matrix_labels: distinct_matrix_sum,
            global_distinct_matrix_labels: global.len(),
            projected_collision_count_including_identity: collisions,
            projected_collision_rate_including_identity: ratio(collisions, raw_components),
            csr_bytes_u32_columns_u16_values: nonzeros * 6 + (rows_len + 1) * 4,
            consistent,
            full_rank: rank == bound,
            solution_matches_enumerated_logs: solution_matches,
        });
    }
    (grid, solved)
}

fn ratio(numerator: usize, denominator: usize) -> f64 {
    if denominator == 0 {
        0.0
    } else {
        numerator as f64 / denominator as f64
    }
}

fn run_arm(
    name: &str,
    cover: &OrdinaryBinaryCover,
    subgroup: &Subgroup,
    classes: &[ClassInput],
) -> Result<(ArmResult, Vec<SolvedSystem>), String> {
    let started = Instant::now();
    let mut counts = CandidateCounts::default();
    let mut relations = Vec::new();
    for (class_index, input) in classes.iter().enumerate() {
        for constant in 0..Q {
            counts.candidates += 1;
            match evaluate_candidate(cover, subgroup, class_index, input, constant)? {
                CandidateOutcome::Rejected(reason) => {
                    *counts.rejections.entry(reason.into()).or_default() += 1;
                }
                CandidateOutcome::Nonsplit => {
                    counts.exact_degree_four += 1;
                    counts.nonsplit += 1;
                }
                CandidateOutcome::Split(relation) => {
                    counts.exact_degree_four += 1;
                    counts.split += 1;
                    relations.push(relation);
                }
            }
        }
    }
    let (grid, solved) = summarize_grid(&relations, &counts, subgroup);
    Ok((
        ArmResult {
            name: name.into(),
            class_count: classes.len(),
            counts,
            grid,
            relations,
            elapsed_milliseconds: started.elapsed().as_millis(),
        },
        solved,
    ))
}

fn relation_value(relation: &RelationRecord, subgroup: &Subgroup, system: &SolvedSystem) -> u16 {
    relation
        .projected_label_logs
        .iter()
        .copied()
        .filter(|log| *log != 0)
        .fold(0u16, |sum, log| {
            let index = subgroup.fb_index_by_log[log as usize].expect("nonidentity label");
            debug_assert!(index < system.bound);
            mod_add(sum, system.solution[index])
        })
}

fn blind_solve(
    cover: &OrdinaryBinaryCover,
    subgroup: &Subgroup,
    kernel: &MumfordDivisor,
    system: Option<&SolvedSystem>,
    arm: &str,
) -> Result<BlindResult, String> {
    let Some(system) = system else {
        return Ok(BlindResult {
            arm: arm.into(),
            factor_base_bound: None,
            trials: Vec::new(),
            successes: 0,
            total_candidates: 0,
        });
    };
    let inverse_twelve = mod_inverse(12).expect("12 invertible mod 127");
    let mut rng = XorShift64(BLIND_SEED);
    let mut trials = Vec::new();
    for target_index in 0..16 {
        let planted = (rng.next() % (R as u64 - 1) + 1) as u16;
        let target = subgroup.multiples[planted as usize].clone();
        let mut recovered = None;
        let mut masks_tried = 0usize;
        let mut candidates_tried = 0usize;
        'masks: for mask in 0..R {
            masks_tried += 1;
            let masked = point_add(&subgroup.curve, &target, &subgroup.multiples[mask as usize]);
            let mut divisor = cover
                .pullback_point_class(&masked)
                .map_err(|e| e.to_string())?;
            if !kernel.is_identity() {
                divisor = divisor
                    .try_add(kernel, cover.source_curve())
                    .map_err(|e| e.to_string())?;
            }
            let input = ClassInput {
                divisor,
                rhs_log: 0, // unknown to the relation routine
            };
            for constant in 0..Q {
                candidates_tried += 1;
                let outcome =
                    evaluate_blind_candidate(cover, subgroup, target_index, &input, constant)?;
                let Some(relation) = outcome else {
                    continue;
                };
                if row_for_bound(&relation, subgroup, system.bound).is_none() {
                    continue;
                }
                let masked_log =
                    mod_mul(inverse_twelve, relation_value(&relation, subgroup, system));
                recovered = Some(mod_sub(masked_log, mask));
                break 'masks;
            }
        }
        let success = recovered.is_some_and(|value| {
            scalar_mul(&subgroup.curve, &subgroup.generator, &BigUint::from(value)) == target
        });
        trials.push(BlindTrial {
            target_index,
            planted_log_for_verification: planted,
            recovered_log: recovered,
            success,
            masks_tried,
            candidates_tried,
        });
    }
    let successes = trials.iter().filter(|trial| trial.success).count();
    let total_candidates = trials.iter().map(|trial| trial.candidates_tried).sum();
    Ok(BlindResult {
        arm: arm.into(),
        factor_base_bound: Some(system.bound),
        trials,
        successes,
        total_candidates,
    })
}

fn evaluate_blind_candidate(
    cover: &OrdinaryBinaryCover,
    subgroup: &Subgroup,
    class_index: usize,
    input: &ClassInput,
    constant: u16,
) -> Result<Option<RelationRecord>, String> {
    // The ordinary candidate checker compares label sums with a known RHS.
    // For an individual logarithm that RHS is deliberately unavailable, so
    // reconstruct it from the support labels only after every geometric check.
    let curve = cover.source_curve();
    let c = F2mElement::from_biguint(&BigUint::from(constant), M);
    let v_lift = curve
        .h
        .add(&input.divisor.v)
        .add(&input.divisor.u.scalar_mul(&c, &curve.irr));
    let numerator = curve
        .f
        .add(&curve.h.mul(&v_lift, &curve.irr))
        .add(&v_lift.mul(&v_lift, &curve.irr));
    let (u_effective, remainder) = numerator.divrem(&input.divisor.u, &curve.irr);
    if !remainder.is_zero()
        || u_effective.degree() != Some(4)
        || u_effective.lead() != F2mElement::one(M)
    {
        return Ok(None);
    }
    let reconstructed =
        MumfordDivisor::try_from_unreduced(curve, u_effective.clone(), v_lift.clone())
            .map_err(|e| e.to_string())?;
    if reconstructed != input.divisor {
        return Err("blind candidate failed reconstruction".into());
    }
    let Some(roots) = split_into_linear_factors(&u_effective, &curve.irr) else {
        return Ok(None);
    };
    let mut labels = Vec::with_capacity(4);
    let mut unprojected = Vec::with_capacity(4);
    for (root, multiplicity) in roots {
        let source = BinaryCoverPoint::Affine {
            v: v_lift.eval(&root, &curve.irr),
            u: root,
        };
        if !cover.source_is_on_curve(&source) {
            return Err("blind split support is off the source curve".into());
        }
        let image = cover.map_point(&source).map_err(|e| e.to_string())?;
        let log = projected_log(subgroup, &image)?;
        labels.extend(std::iter::repeat_n(log, multiplicity));
        unprojected.extend(std::iter::repeat_n(image, multiplicity));
    }
    if labels.len() != 4 {
        return Err("blind split support does not have degree four".into());
    }
    let component_sum = add_many(&subgroup.curve, unprojected);
    let direct = cover
        .pushforward_arbitrary_jacobian_class(&input.divisor)
        .map_err(|e| e.to_string())?;
    if component_sum != direct {
        return Err("blind component sum differs from direct pushforward".into());
    }
    let rhs_log = labels.iter().copied().fold(0u16, mod_add);
    Ok(Some(RelationRecord {
        class_index,
        constant,
        rhs_log,
        projected_label_logs: labels,
    }))
}

fn execute() -> Result<Certificate, String> {
    let irreducible = find_irreducible_sparse(M)
        .ok_or_else(|| "no deterministic sparse modulus for n=9".to_string())?;
    let cover = OrdinaryBinaryCover::new(
        M,
        irreducible.clone(),
        F2mElement::zero(M),
        F2mElement::one(M),
    )
    .map_err(|e| e.to_string())?;
    cover.verify_certificate().map_err(|e| e.to_string())?;
    let (subgroup, target_group_order) = build_subgroup(&cover, &irreducible)?;
    let source_points = source_rational_points(&cover);
    let (seed_point, seed_divisor, kernel) = find_kernel_class(&cover, &source_points)?;
    let kernel_pushforward = cover
        .pushforward_arbitrary_jacobian_class(&kernel)
        .map_err(|e| e.to_string())?;
    let seed_pushforward = cover
        .pushforward_arbitrary_jacobian_class(&seed_divisor)
        .map_err(|e| e.to_string())?;
    if kernel.is_identity() || kernel_pushforward != BinaryPoint::Infinity {
        return Err("selected kernel fixture failed its defining checks".into());
    }

    let (transported, kernel_coset, ambient) =
        build_class_inputs(&cover, &subgroup, &kernel, &source_points)?;
    let two_torsion = BinaryPoint::Affine {
        x: F2mElement::zero(M),
        y: F2mElement::one(M),
    };
    let two_torsion_pullback = cover
        .pullback_point_class(&two_torsion)
        .map_err(|e| e.to_string())?;
    let exception_divisors = vec![
        MumfordDivisor::identity(M),
        two_torsion_pullback,
        kernel.clone(),
        seed_divisor.clone(),
    ];
    let mut exceptions = Vec::new();
    for divisor in exception_divisors {
        let direct = cover
            .pushforward_arbitrary_jacobian_class(&divisor)
            .map_err(|e| e.to_string())?;
        exceptions.push(ClassInput {
            divisor,
            rhs_log: projected_log(&subgroup, &direct)?,
        });
    }

    let (transported_result, transported_solved) =
        run_arm("transported_k0", &cover, &subgroup, &transported)?;
    let (kernel_result, kernel_solved) = run_arm("kernel_coset", &cover, &subgroup, &kernel_coset)?;
    let (ambient_result, _ambient_solved) =
        run_arm("ordinary_ambient", &cover, &subgroup, &ambient)?;
    let (exception_result, _exception_solved) =
        run_arm("exceptional", &cover, &subgroup, &exceptions)?;

    let transported_system = transported_solved.first();
    let kernel_system = kernel_solved.first();
    let blind_k0 = blind_solve(
        &cover,
        &subgroup,
        &MumfordDivisor::identity(M),
        transported_system,
        "transported_k0",
    )?;
    let blind_kernel = blind_solve(&cover, &subgroup, &kernel, kernel_system, "kernel_coset")?;

    let arms = vec![
        transported_result,
        kernel_result,
        ambient_result,
        exception_result,
    ];
    let blind_individual_logs = vec![blind_k0, blind_kernel];
    let image_arms = &arms[..2];
    let nonzero_rank = image_arms
        .iter()
        .flat_map(|arm| &arm.grid)
        .any(|cell| cell.rank > 0);
    let full_rank = image_arms
        .iter()
        .flat_map(|arm| &arm.grid)
        .any(|cell| cell.full_rank);
    let consistency_checks_pass = arms.iter().all(|arm| {
        arm.counts.consistency_failures == 0
            && arm
                .grid
                .iter()
                .all(|cell| cell.consistent && cell.solution_matches_enumerated_logs != Some(false))
    });
    let blind_pass = blind_individual_logs.iter().all(|result| {
        result.factor_base_bound.is_some() && result.trials.len() == 16 && result.successes >= 15
    });
    // The frozen generic model is q^(8/5), while source rho is q^(1/2).
    // A single q=512 exact gate cannot overturn that exponent with a
    // conservative fitted finite-crossover model, so the registered gate
    // stops here even if the structural checks pass.
    let finite_crossover = false;

    let mut protocol_registration = BTreeMap::new();
    protocol_registration.insert(
        "PROTOCOL.md",
        "399bfdec32f08fd6a2ae71a45eb8cda5fd155881de7742a4b545729e632d1c3a",
    );
    protocol_registration.insert(
        "REQUIREMENTS.md",
        "9e5871e8be09ab089c3c8062ce59296035039a41e905bafbdd81d03d648a4a5d",
    );

    Ok(Certificate {
        schema: "crypto.n83-norm-fiber-rr-router-audit/v1",
        status: "pass",
        claim_scope: "exact_n9_registered_one_step_degree4_gate; not_full_kernel_exhaustion; not_n83_timing; not_speedup",
        protocol_registration,
        field: FieldRecord {
            degree: M,
            cardinality: Q,
            modulus_nonleading_terms: irreducible.low_terms.clone(),
            enumerated_target_group_order: target_group_order,
            predicted_target_group_order: 508,
            curve: "y^2 + x*y = x^3 + 1",
            cover: "(u,v) -> (u^3,u*v+1) from v^2+u^2*v=u^7+u",
        },
        subgroup: SubgroupRecord {
            order: R,
            cofactor: COFACTOR,
            generator: point_key(&subgroup.generator),
            unique_multiples: subgroup.log_by_key.len(),
            order_check: scalar_mul(
                &subgroup.curve,
                &subgroup.generator,
                &BigUint::from(R),
            ) == BinaryPoint::Infinity,
        },
        kernel: KernelRecord {
            seed_source_point: source_point_key(&seed_point),
            seed_divisor: divisor_record(&seed_divisor),
            kernel_divisor: divisor_record(&kernel),
            kernel_nonidentity: !kernel.is_identity(),
            kernel_pushforward_is_identity: kernel_pushforward == BinaryPoint::Infinity,
            seed_pushforward_is_identity: seed_pushforward == BinaryPoint::Infinity,
        },
        factor_base: subgroup.factor_base.clone(),
        arms,
        blind_individual_logs,
        advancement_gate: AdvancementGate {
            nonzero_image_compatible_rank: nonzero_rank,
            full_rank_system: full_rank,
            blind_success_at_least_15_of_16_each_eligible_arm: blind_pass,
            consistency_checks_pass,
            finite_crossover_against_source_rho_predicted: finite_crossover,
            advance_to_n13: nonzero_rank
                && full_rank
                && blind_pass
                && consistency_checks_pass
                && finite_crossover,
            reason: "registered q^(8/5) model has no finite crossover against q^(1/2) source rho; stop after the exact n=9 diagnostic",
            registered_generic_collection_exponent_in_q: 1.6,
            registered_generic_matrix_exponent_in_q: 1.6,
            registered_balanced_total_exponent_in_q: 1.6,
            source_rho_exponent_in_q: 0.5,
        },
    })
}

fn main() {
    let args = Args::parse();
    match execute() {
        Ok(certificate) => {
            let encoded = serde_json::to_string_pretty(&certificate)
                .expect("certificate serialization must succeed");
            if let Some(path) = args.output {
                if let Err(error) = fs::write(&path, format!("{encoded}\n")) {
                    eprintln!("failed to write {}: {error}", path.display());
                    std::process::exit(1);
                }
            } else {
                println!("{encoded}");
            }
        }
        Err(error) => {
            eprintln!("n=9 norm-fiber RR audit failed: {error}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elimination_recovers_unique_solution_mod_127() {
        let rows = vec![vec![1, 2, 0], vec![0, 1, 3], vec![4, 0, 1]];
        let expected = [11, 29, 83];
        let rhs = rows
            .iter()
            .map(|row| {
                row.iter()
                    .zip(expected)
                    .fold(0u16, |sum, (a, b)| mod_add(sum, mod_mul(*a, b)))
            })
            .collect();
        let (rank, consistent, solution) = eliminate(rows, rhs);
        assert_eq!(rank, 3);
        assert!(consistent);
        assert_eq!(solution, Some(expected.to_vec()));
    }

    #[test]
    fn inverse_reduction_reconstructs_generic_pullback() {
        let irreducible = find_irreducible_sparse(M).unwrap();
        let cover =
            OrdinaryBinaryCover::new(M, irreducible, F2mElement::zero(M), F2mElement::one(M))
                .unwrap();
        let point = BinaryPoint::Affine {
            x: F2mElement::one(M),
            y: F2mElement::zero(M),
        };
        let reduced = cover.pullback_point_class(&point).unwrap();
        assert_eq!(reduced.u.degree(), Some(3));
        let curve = cover.source_curve();
        for constant in [0u16, 1, 17, 511] {
            let c = F2mElement::from_biguint(&BigUint::from(constant), M);
            let v_lift = curve
                .h
                .add(&reduced.v)
                .add(&reduced.u.scalar_mul(&c, &curve.irr));
            let numerator = curve
                .f
                .add(&curve.h.mul(&v_lift, &curve.irr))
                .add(&v_lift.mul(&v_lift, &curve.irr));
            let (u_effective, remainder) = numerator.divrem(&reduced.u, &curve.irr);
            assert!(remainder.is_zero());
            assert_eq!(u_effective.degree(), Some(4));
            assert_eq!(
                MumfordDivisor::try_from_unreduced(curve, u_effective, v_lift).unwrap(),
                reduced
            );
        }
    }
}
