//! Verifier-backed engineering challenges for ECDLP index calculus.
//!
//! The committed v1 fixture is deliberately tiny and public.  It proves that
//! contribution artifacts can be checked end to end; it is not a benchmark or
//! evidence of an attack on a standard curve.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::hash::sha256::sha256;

pub const FIXTURE_SCHEMA: &str = "crypto.ecdlp-challenge-fixture/v1";
pub const SUBMISSION_SCHEMA: &str = "crypto.ecdlp-challenge-submission/v1";
pub const RECEIPT_SCHEMA: &str = "crypto.ecdlp-challenge-receipt/v1";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AffinePoint {
    pub x: u64,
    pub y: u64,
}

pub type Point = Option<AffinePoint>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurveSpec {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub order: u64,
    pub generator: Point,
    pub target: Point,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationFixture {
    pub factor_base: Vec<Point>,
    pub max_rows: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactorBaseFixture {
    pub universe: Vec<Point>,
    pub targets: Vec<Point>,
    pub max_size: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixFixture {
    pub modulus: u64,
    pub matrix: Vec<Vec<i64>>,
    pub rhs: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescentFixture {
    pub root: Point,
    pub factor_base: Vec<Point>,
    pub max_nodes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferFixture {
    pub map: String,
    pub sources: Vec<Point>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrbitFixture {
    pub action: String,
    pub universe: Vec<Point>,
    pub relation_rows: Vec<Vec<i64>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeFixture {
    pub schema: String,
    pub suite_id: String,
    pub curve: CurveSpec,
    pub relation_forge: RelationFixture,
    pub factor_base_lab: FactorBaseFixture,
    pub matrix_smash: MatrixFixture,
    pub descent_tree: DescentFixture,
    pub transfer_audit: TransferFixture,
    pub orbit_relations: OrbitFixture,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationRow {
    pub factor_coefficients: Vec<i64>,
    pub a: i64,
    pub b: i64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationSubmission {
    pub rows: Vec<RelationRow>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Decomposition {
    pub target: usize,
    pub left: usize,
    pub right: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FactorBaseSubmission {
    pub selected: Vec<Point>,
    pub decompositions: Vec<Decomposition>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixSubmission {
    pub solution: Vec<i64>,
    pub rank: usize,
    pub left_kernel: Vec<Vec<i64>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescentNode {
    pub id: String,
    pub point: Point,
    #[serde(default)]
    pub children: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DescentSubmission {
    pub root: String,
    pub nodes: Vec<DescentNode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferImage {
    pub source: Point,
    pub image: Point,
    pub recovered: Point,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferObligations {
    pub homomorphism: String,
    pub subgroup_preservation: String,
    pub recovery: String,
    pub asymptotic_advantage: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSubmission {
    pub map: String,
    pub images: Vec<TransferImage>,
    pub obligations: TransferObligations,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrbitMember {
    pub member: Point,
    pub representative: Point,
    pub action: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrbitSubmission {
    pub representatives: Vec<Point>,
    pub reconstruction: Vec<OrbitMember>,
    pub folded_rows: Vec<Vec<i64>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeSubmission {
    pub schema: String,
    pub suite_id: String,
    pub implementation: String,
    pub relation_forge: RelationSubmission,
    pub factor_base_lab: FactorBaseSubmission,
    pub matrix_smash: MatrixSubmission,
    pub descent_tree: DescentSubmission,
    pub transfer_audit: TransferSubmission,
    pub orbit_relations: OrbitSubmission,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrackReceipt {
    pub status: String,
    pub metrics: BTreeMap<String, u64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub obligations: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrontierCoordinate {
    pub track: String,
    pub metric: String,
    pub direction: String,
    pub value: u64,
    pub unit: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimBoundary {
    pub performance: String,
    pub security: String,
    pub transfer_advantage: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeReceipt {
    pub schema: String,
    pub suite_id: String,
    pub fixture_sha256: String,
    pub submission_sha256: String,
    pub implementation: String,
    pub status: String,
    pub tracks: BTreeMap<String, TrackReceipt>,
    pub frontier: Vec<FrontierCoordinate>,
    pub claim_boundary: ClaimBoundary,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChallengeError(pub String);

impl fmt::Display for ChallengeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ChallengeError {}

fn err<T>(message: impl Into<String>) -> Result<T, ChallengeError> {
    Err(ChallengeError(message.into()))
}

fn mod_i128(value: i128, modulus: u64) -> u64 {
    let m = modulus as i128;
    let mut value = value % m;
    if value < 0 {
        value += m;
    }
    value as u64
}

fn add_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 + b as u128) % modulus as u128) as u64
}

fn sub_mod(a: u64, b: u64, modulus: u64) -> u64 {
    mod_i128(a as i128 - b as i128, modulus)
}

fn mul_mod(a: u64, b: u64, modulus: u64) -> u64 {
    ((a as u128 * b as u128) % modulus as u128) as u64
}

fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn inverse(value: u64, modulus: u64) -> Result<u64, ChallengeError> {
    if value == 0 {
        return err("attempted to invert zero");
    }
    Ok(pow_mod(value, modulus - 2, modulus))
}

fn is_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    }
    if value.is_multiple_of(2) {
        return value == 2;
    }
    let mut divisor = 3u64;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            return false;
        }
        divisor += 2;
    }
    true
}

fn on_curve(curve: &CurveSpec, point: &Point) -> bool {
    let Some(point) = point else {
        return true;
    };
    if point.x >= curve.p || point.y >= curve.p {
        return false;
    }
    let lhs = mul_mod(point.y, point.y, curve.p);
    let x2 = mul_mod(point.x, point.x, curve.p);
    let x3 = mul_mod(x2, point.x, curve.p);
    let rhs = add_mod(
        add_mod(x3, mul_mod(curve.a, point.x, curve.p), curve.p),
        curve.b,
        curve.p,
    );
    lhs == rhs
}

fn negate(curve: &CurveSpec, point: &Point) -> Point {
    point.as_ref().map(|point| AffinePoint {
        x: point.x,
        y: if point.y == 0 { 0 } else { curve.p - point.y },
    })
}

fn add(curve: &CurveSpec, left: &Point, right: &Point) -> Result<Point, ChallengeError> {
    match (left, right) {
        (None, _) => Ok(right.clone()),
        (_, None) => Ok(left.clone()),
        (Some(left), Some(right)) => {
            if left.x == right.x && add_mod(left.y, right.y, curve.p) == 0 {
                return Ok(None);
            }
            let slope = if left == right {
                if left.y == 0 {
                    return Ok(None);
                }
                let x2 = mul_mod(left.x, left.x, curve.p);
                let numerator = add_mod(mul_mod(3, x2, curve.p), curve.a, curve.p);
                let denominator = mul_mod(2, left.y, curve.p);
                mul_mod(numerator, inverse(denominator, curve.p)?, curve.p)
            } else {
                let numerator = sub_mod(right.y, left.y, curve.p);
                let denominator = sub_mod(right.x, left.x, curve.p);
                mul_mod(numerator, inverse(denominator, curve.p)?, curve.p)
            };
            let x = sub_mod(
                sub_mod(mul_mod(slope, slope, curve.p), left.x, curve.p),
                right.x,
                curve.p,
            );
            let y = sub_mod(
                mul_mod(slope, sub_mod(left.x, x, curve.p), curve.p),
                left.y,
                curve.p,
            );
            Ok(Some(AffinePoint { x, y }))
        }
    }
}

fn scalar_mul(curve: &CurveSpec, point: &Point, mut scalar: u64) -> Result<Point, ChallengeError> {
    let mut result = None;
    let mut addend = point.clone();
    while scalar != 0 {
        if scalar & 1 == 1 {
            result = add(curve, &result, &addend)?;
        }
        addend = add(curve, &addend, &addend)?;
        scalar >>= 1;
    }
    Ok(result)
}

fn signed_mul(curve: &CurveSpec, point: &Point, scalar: i64) -> Result<Point, ChallengeError> {
    scalar_mul(curve, point, mod_i128(scalar as i128, curve.order))
}

fn linear_combination(
    curve: &CurveSpec,
    points: &[Point],
    coefficients: &[i64],
) -> Result<Point, ChallengeError> {
    if points.len() != coefficients.len() {
        return err("point/coefficient length mismatch");
    }
    let mut result = None;
    for (point, coefficient) in points.iter().zip(coefficients) {
        result = add(curve, &result, &signed_mul(curve, point, *coefficient)?)?;
    }
    Ok(result)
}

fn validate_points(
    curve: &CurveSpec,
    label: &str,
    points: &[Point],
    unique: bool,
) -> Result<(), ChallengeError> {
    let mut seen = BTreeSet::new();
    for point in points {
        if point.is_none() || !on_curve(curve, point) {
            return err(format!("{label} contains infinity or an off-curve point"));
        }
        if scalar_mul(curve, point, curve.order)?.is_some() {
            return err(format!(
                "{label} contains a point outside the declared subgroup"
            ));
        }
        if unique && !seen.insert(point.clone()) {
            return err(format!("{label} contains a duplicate point"));
        }
    }
    Ok(())
}

fn validate_fixture(fixture: &ChallengeFixture) -> Result<(), ChallengeError> {
    if fixture.schema != FIXTURE_SCHEMA {
        return err(format!("unsupported fixture schema {}", fixture.schema));
    }
    if fixture.suite_id.is_empty() {
        return err("fixture suite_id is empty");
    }
    let curve = &fixture.curve;
    if !is_prime(curve.p) || !is_prime(curve.order) {
        return err("curve field and subgroup order must both be prime");
    }
    if curve.a >= curve.p || curve.b >= curve.p {
        return err("curve coefficients are not reduced");
    }
    let discriminant = add_mod(
        mul_mod(
            4,
            mul_mod(mul_mod(curve.a, curve.a, curve.p), curve.a, curve.p),
            curve.p,
        ),
        mul_mod(27, mul_mod(curve.b, curve.b, curve.p), curve.p),
        curve.p,
    );
    if discriminant == 0 {
        return err("curve is singular");
    }
    if curve.generator.is_none() || !on_curve(curve, &curve.generator) {
        return err("generator is infinity or off curve");
    }
    if scalar_mul(curve, &curve.generator, curve.order)?.is_some() {
        return err("generator does not have the declared prime order");
    }
    if curve.target.is_none()
        || !on_curve(curve, &curve.target)
        || scalar_mul(curve, &curve.target, curve.order)?.is_some()
    {
        return err("target is not a nonzero point in the declared subgroup");
    }
    validate_points(
        curve,
        "relation factor base",
        &fixture.relation_forge.factor_base,
        true,
    )?;
    if fixture.relation_forge.factor_base.is_empty() || fixture.relation_forge.max_rows == 0 {
        return err("relation fixture has no factor base or row budget");
    }
    validate_points(
        curve,
        "factor-base universe",
        &fixture.factor_base_lab.universe,
        true,
    )?;
    validate_points(
        curve,
        "factor-base targets",
        &fixture.factor_base_lab.targets,
        true,
    )?;
    if fixture.factor_base_lab.max_size == 0
        || fixture.factor_base_lab.max_size > fixture.factor_base_lab.universe.len()
    {
        return err("factor-base size budget is invalid");
    }
    if fixture.matrix_smash.modulus != curve.order || !is_prime(fixture.matrix_smash.modulus) {
        return err("matrix modulus must equal the prime subgroup order");
    }
    rectangular(&fixture.matrix_smash.matrix)?;
    if fixture.matrix_smash.matrix.len() != fixture.matrix_smash.rhs.len() {
        return err("matrix row and right-hand-side lengths differ");
    }
    if fixture.matrix_smash.matrix.is_empty() || fixture.matrix_smash.matrix[0].is_empty() {
        return err("matrix fixture is empty");
    }
    validate_points(
        curve,
        "descent factor base",
        &fixture.descent_tree.factor_base,
        true,
    )?;
    validate_points(
        curve,
        "descent root",
        std::slice::from_ref(&fixture.descent_tree.root),
        false,
    )?;
    if fixture.descent_tree.max_nodes == 0 {
        return err("descent node budget is zero");
    }
    if fixture.transfer_audit.map != "negation" {
        return err("v1 supports only the explicit negation transfer control");
    }
    validate_points(
        curve,
        "transfer sources",
        &fixture.transfer_audit.sources,
        true,
    )?;
    if fixture.orbit_relations.action != "negation" {
        return err("v1 supports only negation orbit folding");
    }
    validate_points(
        curve,
        "orbit universe",
        &fixture.orbit_relations.universe,
        true,
    )?;
    let universe: BTreeSet<_> = fixture.orbit_relations.universe.iter().cloned().collect();
    for point in &fixture.orbit_relations.universe {
        if !universe.contains(&negate(curve, point)) {
            return err("orbit universe is not closed under negation");
        }
    }
    for row in &fixture.orbit_relations.relation_rows {
        if row.len() != fixture.orbit_relations.universe.len() {
            return err("orbit relation row width differs from the universe");
        }
    }
    Ok(())
}

fn rectangular(matrix: &[Vec<i64>]) -> Result<usize, ChallengeError> {
    let columns = matrix.first().map_or(0, Vec::len);
    if matrix.iter().any(|row| row.len() != columns) {
        return err("matrix is not rectangular");
    }
    Ok(columns)
}

fn normalize_matrix(matrix: &[Vec<i64>], modulus: u64) -> Vec<Vec<u64>> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| mod_i128(*value as i128, modulus))
                .collect()
        })
        .collect()
}

fn rref(
    mut matrix: Vec<Vec<u64>>,
    modulus: u64,
) -> Result<(Vec<Vec<u64>>, Vec<usize>), ChallengeError> {
    if matrix.is_empty() {
        return Ok((matrix, Vec::new()));
    }
    let columns = matrix[0].len();
    if matrix.iter().any(|row| row.len() != columns) {
        return err("matrix is not rectangular");
    }
    let mut row = 0usize;
    let mut pivots = Vec::new();
    for column in 0..columns {
        let Some(pivot) = (row..matrix.len()).find(|index| matrix[*index][column] != 0) else {
            continue;
        };
        matrix.swap(row, pivot);
        let scale = inverse(matrix[row][column], modulus)?;
        for value in &mut matrix[row] {
            *value = mul_mod(*value, scale, modulus);
        }
        for other in 0..matrix.len() {
            if other == row || matrix[other][column] == 0 {
                continue;
            }
            let factor = matrix[other][column];
            for entry in column..columns {
                matrix[other][entry] = sub_mod(
                    matrix[other][entry],
                    mul_mod(factor, matrix[row][entry], modulus),
                    modulus,
                );
            }
        }
        pivots.push(column);
        row += 1;
        if row == matrix.len() {
            break;
        }
    }
    Ok((matrix, pivots))
}

fn rank_i64(matrix: &[Vec<i64>], modulus: u64) -> Result<usize, ChallengeError> {
    Ok(rref(normalize_matrix(matrix, modulus), modulus)?.1.len())
}

fn digest_json<T: Serialize>(value: &T) -> Result<String, ChallengeError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| ChallengeError(format!("cannot canonicalize JSON: {error}")))?;
    Ok(sha256(&bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn metrics(entries: &[(&str, u64)]) -> BTreeMap<String, u64> {
    entries
        .iter()
        .map(|(name, value)| ((*name).to_string(), *value))
        .collect()
}

fn verified(metrics: BTreeMap<String, u64>) -> TrackReceipt {
    TrackReceipt {
        status: "verified".to_string(),
        metrics,
        obligations: BTreeMap::new(),
    }
}

fn verify_relations(
    fixture: &ChallengeFixture,
    submission: &RelationSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    if submission.rows.is_empty() || submission.rows.len() > fixture.relation_forge.max_rows {
        return err("relation_forge row count is zero or exceeds its budget");
    }
    let mut rank_rows = Vec::with_capacity(submission.rows.len());
    for (index, row) in submission.rows.iter().enumerate() {
        if row.factor_coefficients.len() != fixture.relation_forge.factor_base.len() {
            return err(format!("relation_forge row {index} has the wrong width"));
        }
        let lhs = linear_combination(
            &fixture.curve,
            &fixture.relation_forge.factor_base,
            &row.factor_coefficients,
        )?;
        let a_term = signed_mul(&fixture.curve, &fixture.curve.generator, row.a)?;
        let b_term = signed_mul(&fixture.curve, &fixture.curve.target, row.b)?;
        let rhs = add(&fixture.curve, &a_term, &b_term)?;
        if lhs != rhs {
            return err(format!("relation_forge row {index} fails its EC equality"));
        }
        let mut rank_row = row.factor_coefficients.clone();
        rank_row.push(-row.b);
        rank_rows.push(rank_row);
    }
    let rank = rank_i64(&rank_rows, fixture.curve.order)?;
    Ok(verified(metrics(&[
        ("verified_rows", submission.rows.len() as u64),
        ("independent_rank", rank as u64),
        (
            "unknown_columns",
            (fixture.relation_forge.factor_base.len() + 1) as u64,
        ),
    ])))
}

fn verify_factor_base(
    fixture: &ChallengeFixture,
    submission: &FactorBaseSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    if submission.selected.is_empty()
        || submission.selected.len() > fixture.factor_base_lab.max_size
    {
        return err("factor_base_lab selection is empty or exceeds its budget");
    }
    let universe: BTreeSet<_> = fixture.factor_base_lab.universe.iter().cloned().collect();
    let mut selected = BTreeSet::new();
    for point in &submission.selected {
        if !universe.contains(point) {
            return err("factor_base_lab selected a point outside the allowed universe");
        }
        if !selected.insert(point.clone()) {
            return err("factor_base_lab selected a duplicate point");
        }
    }
    let mut covered = BTreeSet::new();
    for decomposition in &submission.decompositions {
        if decomposition.target >= fixture.factor_base_lab.targets.len()
            || decomposition.left >= submission.selected.len()
            || decomposition.right >= submission.selected.len()
        {
            return err("factor_base_lab decomposition index is out of range");
        }
        if !covered.insert(decomposition.target) {
            return err("factor_base_lab repeats a target decomposition");
        }
        let sum = add(
            &fixture.curve,
            &submission.selected[decomposition.left],
            &submission.selected[decomposition.right],
        )?;
        if sum != fixture.factor_base_lab.targets[decomposition.target] {
            return err(format!(
                "factor_base_lab target {} decomposition is false",
                decomposition.target
            ));
        }
    }
    Ok(verified(metrics(&[
        ("factor_base_size", submission.selected.len() as u64),
        ("decomposed_targets", covered.len() as u64),
        (
            "available_targets",
            fixture.factor_base_lab.targets.len() as u64,
        ),
    ])))
}

fn verify_matrix(
    fixture: &ChallengeFixture,
    submission: &MatrixSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    let matrix = &fixture.matrix_smash.matrix;
    let rows = matrix.len();
    let columns = rectangular(matrix)?;
    let modulus = fixture.matrix_smash.modulus;
    if submission.solution.len() != columns {
        return err("matrix_smash solution has the wrong width");
    }
    for (row_index, row) in matrix.iter().enumerate() {
        let mut value = 0u64;
        for (coefficient, solution) in row.iter().zip(&submission.solution) {
            value = add_mod(
                value,
                mul_mod(
                    mod_i128(*coefficient as i128, modulus),
                    mod_i128(*solution as i128, modulus),
                    modulus,
                ),
                modulus,
            );
        }
        if value != mod_i128(fixture.matrix_smash.rhs[row_index] as i128, modulus) {
            return err(format!(
                "matrix_smash residual is nonzero in row {row_index}"
            ));
        }
    }
    let rank = rank_i64(matrix, modulus)?;
    if submission.rank != rank {
        return err("matrix_smash claimed rank differs from recomputed rank");
    }
    let expected_nullity = rows - rank;
    if submission.left_kernel.len() != expected_nullity {
        return err("matrix_smash left-kernel dimension is incomplete");
    }
    let mut normalized_kernel = Vec::new();
    for (index, vector) in submission.left_kernel.iter().enumerate() {
        if vector.len() != rows {
            return err(format!(
                "matrix_smash left-kernel vector {index} has the wrong width"
            ));
        }
        if vector
            .iter()
            .all(|value| mod_i128(*value as i128, modulus) == 0)
        {
            return err("matrix_smash contains a zero left-kernel vector");
        }
        for column in 0..columns {
            let mut value = 0u64;
            for row in 0..rows {
                value = add_mod(
                    value,
                    mul_mod(
                        mod_i128(vector[row] as i128, modulus),
                        mod_i128(matrix[row][column] as i128, modulus),
                        modulus,
                    ),
                    modulus,
                );
            }
            if value != 0 {
                return err(format!("matrix_smash left-kernel vector {index} fails"));
            }
        }
        let rhs_dot = vector.iter().zip(&fixture.matrix_smash.rhs).fold(
            0u64,
            |accumulator, (left, right)| {
                add_mod(
                    accumulator,
                    mul_mod(
                        mod_i128(*left as i128, modulus),
                        mod_i128(*right as i128, modulus),
                        modulus,
                    ),
                    modulus,
                )
            },
        );
        if rhs_dot != 0 {
            return err("matrix_smash left-kernel vector is inconsistent with rhs");
        }
        normalized_kernel.push(vector.clone());
    }
    if rank_i64(&normalized_kernel, modulus)? != expected_nullity {
        return err("matrix_smash left-kernel vectors are not independent");
    }
    Ok(verified(metrics(&[
        ("rows", rows as u64),
        ("columns", columns as u64),
        ("rank", rank as u64),
        ("left_nullity", expected_nullity as u64),
    ])))
}

fn visit_descent(
    curve: &CurveSpec,
    factor_base: &BTreeSet<Point>,
    nodes: &BTreeMap<String, DescentNode>,
    id: &str,
    state: &mut BTreeMap<String, u8>,
    memo: &mut BTreeMap<String, (usize, usize)>,
    reached: &mut BTreeSet<String>,
) -> Result<(usize, usize), ChallengeError> {
    if state.get(id) == Some(&1) {
        return err("descent_tree contains a cycle");
    }
    if let Some(result) = memo.get(id) {
        reached.insert(id.to_string());
        return Ok(*result);
    }
    let node = nodes
        .get(id)
        .ok_or_else(|| ChallengeError(format!("descent_tree references missing node {id}")))?;
    reached.insert(id.to_string());
    state.insert(id.to_string(), 1);
    let result = if node.children.is_empty() {
        if !factor_base.contains(&node.point) {
            return err(format!("descent_tree leaf {id} is outside the factor base"));
        }
        (0, 0)
    } else {
        if node.children.len() != 2 {
            return err(format!(
                "descent_tree node {id} is not a binary decomposition"
            ));
        }
        let left = visit_descent(
            curve,
            factor_base,
            nodes,
            &node.children[0],
            state,
            memo,
            reached,
        )?;
        let right = visit_descent(
            curve,
            factor_base,
            nodes,
            &node.children[1],
            state,
            memo,
            reached,
        )?;
        let expected = add(
            curve,
            &nodes[&node.children[0]].point,
            &nodes[&node.children[1]].point,
        )?;
        if expected != node.point {
            return err(format!("descent_tree node {id} fails its EC decomposition"));
        }
        (1 + left.0 + right.0, 1 + left.1.max(right.1))
    };
    state.insert(id.to_string(), 2);
    memo.insert(id.to_string(), result);
    Ok(result)
}

fn verify_descent(
    fixture: &ChallengeFixture,
    submission: &DescentSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    if submission.nodes.is_empty() || submission.nodes.len() > fixture.descent_tree.max_nodes {
        return err("descent_tree node count is zero or exceeds its budget");
    }
    let mut nodes = BTreeMap::new();
    for node in &submission.nodes {
        if node.id.is_empty() || nodes.insert(node.id.clone(), node.clone()).is_some() {
            return err("descent_tree has an empty or duplicate node id");
        }
        if node.point.is_none() || !on_curve(&fixture.curve, &node.point) {
            return err(format!(
                "descent_tree node {} is infinity or off curve",
                node.id
            ));
        }
    }
    let root = nodes
        .get(&submission.root)
        .ok_or_else(|| ChallengeError("descent_tree root id is missing".to_string()))?;
    if root.point != fixture.descent_tree.root {
        return err("descent_tree root point differs from the challenge root");
    }
    let mut parent_count: BTreeMap<String, usize> =
        nodes.keys().cloned().map(|id| (id, 0usize)).collect();
    for node in nodes.values() {
        for child in &node.children {
            let Some(count) = parent_count.get_mut(child) else {
                return err(format!("descent_tree references missing node {child}"));
            };
            *count += 1;
            if *count > 1 {
                return err(format!("descent_tree node {child} has multiple parents"));
            }
        }
    }
    if parent_count[&submission.root] != 0
        || parent_count
            .iter()
            .any(|(id, count)| id != &submission.root && *count != 1)
    {
        return err("descent_tree parent structure is not one rooted tree");
    }
    let factor_base = fixture.descent_tree.factor_base.iter().cloned().collect();
    let (internal, depth) = visit_descent(
        &fixture.curve,
        &factor_base,
        &nodes,
        &submission.root,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
        &mut BTreeSet::new(),
    )?;
    let mut reached = BTreeSet::new();
    let _ = visit_descent(
        &fixture.curve,
        &factor_base,
        &nodes,
        &submission.root,
        &mut BTreeMap::new(),
        &mut BTreeMap::new(),
        &mut reached,
    )?;
    if reached.len() != nodes.len() {
        return err("descent_tree contains unreachable nodes");
    }
    Ok(verified(metrics(&[
        ("nodes", nodes.len() as u64),
        ("internal_nodes", internal as u64),
        ("depth", depth as u64),
    ])))
}

fn map_point(curve: &CurveSpec, map: &str, point: &Point) -> Result<Point, ChallengeError> {
    match map {
        "negation" => Ok(negate(curve, point)),
        _ => err(format!("unsupported transfer map {map}")),
    }
}

fn verify_transfer(
    fixture: &ChallengeFixture,
    submission: &TransferSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    if submission.map != fixture.transfer_audit.map {
        return err("transfer_audit used a different map from the fixture");
    }
    let obligations = &submission.obligations;
    if obligations.homomorphism != "verified"
        || obligations.subgroup_preservation != "verified"
        || obligations.recovery != "verified"
        || obligations.asymptotic_advantage != "unknown"
    {
        return err("transfer_audit obligation states are not the required scoped states");
    }
    let mut images = BTreeMap::new();
    for image in &submission.images {
        if images.insert(image.source.clone(), image).is_some() {
            return err("transfer_audit repeats a source");
        }
    }
    if images.len() != fixture.transfer_audit.sources.len() {
        return err("transfer_audit does not cover every supplied source exactly once");
    }
    for source in &fixture.transfer_audit.sources {
        let record = images.get(source).ok_or_else(|| {
            ChallengeError("transfer_audit omitted a supplied source".to_string())
        })?;
        let image = map_point(&fixture.curve, &submission.map, source)?;
        if image != record.image {
            return err("transfer_audit image is incorrect");
        }
        let recovered = map_point(&fixture.curve, &submission.map, &image)?;
        if recovered != record.recovered || recovered != *source {
            return err("transfer_audit recovery is incorrect");
        }
        if !on_curve(&fixture.curve, &image)
            || scalar_mul(&fixture.curve, &image, fixture.curve.order)?.is_some()
        {
            return err("transfer_audit image does not preserve the subgroup");
        }
    }
    for left_scalar in 0..fixture.curve.order {
        let left = scalar_mul(&fixture.curve, &fixture.curve.generator, left_scalar)?;
        for right_scalar in 0..fixture.curve.order {
            let right = scalar_mul(&fixture.curve, &fixture.curve.generator, right_scalar)?;
            let sum = add(&fixture.curve, &left, &right)?;
            let mapped_sum = map_point(&fixture.curve, &submission.map, &sum)?;
            let sum_mapped = add(
                &fixture.curve,
                &map_point(&fixture.curve, &submission.map, &left)?,
                &map_point(&fixture.curve, &submission.map, &right)?,
            )?;
            if mapped_sum != sum_mapped {
                return err("transfer_audit map fails exhaustive toy-subgroup homomorphism replay");
            }
        }
    }
    let mut obligation_receipt = BTreeMap::new();
    obligation_receipt.insert("homomorphism".to_string(), "verified".to_string());
    obligation_receipt.insert("subgroup_preservation".to_string(), "verified".to_string());
    obligation_receipt.insert("recovery".to_string(), "verified".to_string());
    obligation_receipt.insert("asymptotic_advantage".to_string(), "unknown".to_string());
    Ok(TrackReceipt {
        status: "verified".to_string(),
        metrics: metrics(&[("recovered_points", images.len() as u64)]),
        obligations: obligation_receipt,
    })
}

fn verify_orbits(
    fixture: &ChallengeFixture,
    submission: &OrbitSubmission,
) -> Result<TrackReceipt, ChallengeError> {
    if submission.representatives.is_empty() {
        return err("orbit_relations has no representatives");
    }
    let universe: BTreeSet<_> = fixture.orbit_relations.universe.iter().cloned().collect();
    let mut representatives = BTreeSet::new();
    for representative in &submission.representatives {
        if !universe.contains(representative) || !representatives.insert(representative.clone()) {
            return err("orbit_relations representative is outside the universe or duplicated");
        }
    }
    let representative_index: BTreeMap<_, _> = submission
        .representatives
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, point)| (point, index))
        .collect();
    let mut reconstruction = BTreeMap::new();
    for record in &submission.reconstruction {
        if !universe.contains(&record.member)
            || !representatives.contains(&record.representative)
            || reconstruction
                .insert(record.member.clone(), record)
                .is_some()
        {
            return err("orbit_relations reconstruction is outside its sets or duplicated");
        }
        let reconstructed = match record.action.as_str() {
            "identity" => record.representative.clone(),
            "negation" => negate(&fixture.curve, &record.representative),
            _ => return err("orbit_relations uses an unsupported reconstruction action"),
        };
        if reconstructed != record.member {
            return err("orbit_relations reconstruction action does not recover its member");
        }
    }
    if reconstruction.len() != universe.len() {
        return err("orbit_relations reconstruction does not cover the universe exactly once");
    }
    if submission.folded_rows.len() != fixture.orbit_relations.relation_rows.len() {
        return err("orbit_relations folded row count differs from the fixture");
    }
    let modulus = fixture.curve.order;
    let mut recomputed = Vec::new();
    for (row_index, original) in fixture.orbit_relations.relation_rows.iter().enumerate() {
        let mut folded = vec![0u64; submission.representatives.len()];
        for (column, coefficient) in original.iter().enumerate() {
            let point = &fixture.orbit_relations.universe[column];
            let record = reconstruction[point];
            let index = representative_index[&record.representative];
            let signed = if record.action == "identity" {
                *coefficient as i128
            } else {
                -(*coefficient as i128)
            };
            folded[index] = add_mod(folded[index], mod_i128(signed, modulus), modulus);
        }
        if submission.folded_rows[row_index].len() != submission.representatives.len() {
            return err("orbit_relations folded row has the wrong width");
        }
        let claimed: Vec<_> = submission.folded_rows[row_index]
            .iter()
            .map(|value| mod_i128(*value as i128, modulus))
            .collect();
        if claimed != folded {
            return err(format!(
                "orbit_relations folded row {row_index} is incorrect"
            ));
        }
        let original_point =
            linear_combination(&fixture.curve, &fixture.orbit_relations.universe, original)?;
        let folded_i64: Vec<_> = folded.iter().map(|value| *value as i64).collect();
        let folded_point =
            linear_combination(&fixture.curve, &submission.representatives, &folded_i64)?;
        if original_point != folded_point {
            return err("orbit_relations folding changed the represented EC sum");
        }
        recomputed.push(folded_i64);
    }
    let original_rank = rank_i64(&fixture.orbit_relations.relation_rows, modulus)?;
    let folded_rank = rank_i64(&recomputed, modulus)?;
    if original_rank != folded_rank {
        return err("orbit_relations folding did not preserve matrix rank");
    }
    Ok(verified(metrics(&[
        (
            "original_columns",
            fixture.orbit_relations.universe.len() as u64,
        ),
        ("folded_columns", submission.representatives.len() as u64),
        ("original_rank", original_rank as u64),
        ("folded_rank", folded_rank as u64),
    ])))
}

pub fn verify(
    fixture: &ChallengeFixture,
    submission: &ChallengeSubmission,
) -> Result<ChallengeReceipt, ChallengeError> {
    validate_fixture(fixture)?;
    if submission.schema != SUBMISSION_SCHEMA {
        return err(format!(
            "unsupported submission schema {}",
            submission.schema
        ));
    }
    if submission.suite_id != fixture.suite_id {
        return err("submission suite_id does not match the fixture");
    }
    if submission.implementation.trim().is_empty() {
        return err("submission implementation label is empty");
    }
    let mut tracks = BTreeMap::new();
    tracks.insert(
        "relation_forge".to_string(),
        verify_relations(fixture, &submission.relation_forge)?,
    );
    tracks.insert(
        "factor_base_lab".to_string(),
        verify_factor_base(fixture, &submission.factor_base_lab)?,
    );
    tracks.insert(
        "matrix_smash".to_string(),
        verify_matrix(fixture, &submission.matrix_smash)?,
    );
    tracks.insert(
        "descent_tree".to_string(),
        verify_descent(fixture, &submission.descent_tree)?,
    );
    tracks.insert(
        "transfer_audit".to_string(),
        verify_transfer(fixture, &submission.transfer_audit)?,
    );
    tracks.insert(
        "orbit_relations".to_string(),
        verify_orbits(fixture, &submission.orbit_relations)?,
    );
    let coordinate = |track: &str, metric: &str, direction: &str, unit: &str| FrontierCoordinate {
        track: track.to_string(),
        metric: metric.to_string(),
        direction: direction.to_string(),
        value: tracks[track].metrics[metric],
        unit: unit.to_string(),
    };
    let frontier = vec![
        coordinate("relation_forge", "independent_rank", "maximize", "rank"),
        coordinate(
            "factor_base_lab",
            "decomposed_targets",
            "maximize",
            "targets",
        ),
        coordinate("matrix_smash", "columns", "maximize", "columns"),
        coordinate("descent_tree", "internal_nodes", "minimize", "nodes"),
        coordinate("transfer_audit", "recovered_points", "maximize", "points"),
        coordinate("orbit_relations", "folded_columns", "minimize", "columns"),
    ];
    Ok(ChallengeReceipt {
        schema: RECEIPT_SCHEMA.to_string(),
        suite_id: fixture.suite_id.clone(),
        fixture_sha256: digest_json(fixture)?,
        submission_sha256: digest_json(submission)?,
        implementation: submission.implementation.clone(),
        status: "verified".to_string(),
        tracks,
        frontier,
        claim_boundary: ClaimBoundary {
            performance: "not_measured".to_string(),
            security: "not_established".to_string(),
            transfer_advantage: "unknown".to_string(),
        },
    })
}

fn discrete_log(curve: &CurveSpec, point: &Point) -> Result<u64, ChallengeError> {
    for scalar in 0..curve.order {
        if scalar_mul(curve, &curve.generator, scalar)? == *point {
            return Ok(scalar);
        }
    }
    err("point has no discrete log in the declared subgroup")
}

fn best_factor_base(fixture: &ChallengeFixture) -> Result<FactorBaseSubmission, ChallengeError> {
    fn combinations(
        start: usize,
        needed: usize,
        current: &mut Vec<usize>,
        output: &mut Vec<Vec<usize>>,
        total: usize,
    ) {
        if needed == 0 {
            output.push(current.clone());
            return;
        }
        for index in start..=total - needed {
            current.push(index);
            combinations(index + 1, needed - 1, current, output, total);
            current.pop();
        }
    }
    let f = &fixture.factor_base_lab;
    let mut choices = Vec::new();
    combinations(
        0,
        f.max_size,
        &mut Vec::new(),
        &mut choices,
        f.universe.len(),
    );
    let mut best: Option<(Vec<usize>, Vec<Decomposition>)> = None;
    for choice in choices {
        let selected: Vec<_> = choice
            .iter()
            .map(|index| f.universe[*index].clone())
            .collect();
        let mut decompositions = Vec::new();
        for (target, expected) in f.targets.iter().enumerate() {
            'pairs: for left in 0..selected.len() {
                for right in left..selected.len() {
                    if add(&fixture.curve, &selected[left], &selected[right])? == *expected {
                        decompositions.push(Decomposition {
                            target,
                            left,
                            right,
                        });
                        break 'pairs;
                    }
                }
            }
        }
        if best.as_ref().is_none_or(|(old, old_decompositions)| {
            decompositions.len() > old_decompositions.len()
                || (decompositions.len() == old_decompositions.len() && choice < *old)
        }) {
            best = Some((choice, decompositions));
        }
    }
    let (choice, decompositions) =
        best.ok_or_else(|| ChallengeError("no factor-base choice".into()))?;
    Ok(FactorBaseSubmission {
        selected: choice
            .iter()
            .map(|index| f.universe[*index].clone())
            .collect(),
        decompositions,
    })
}

fn solve_unique(matrix: &MatrixFixture) -> Result<Vec<i64>, ChallengeError> {
    let columns = rectangular(&matrix.matrix)?;
    let mut augmented = normalize_matrix(&matrix.matrix, matrix.modulus);
    for (row, rhs) in augmented.iter_mut().zip(&matrix.rhs) {
        row.push(mod_i128(*rhs as i128, matrix.modulus));
    }
    let (reduced, pivots) = rref(augmented, matrix.modulus)?;
    if pivots.iter().filter(|pivot| **pivot < columns).count() != columns {
        return err("baseline matrix is not uniquely solvable");
    }
    let mut solution = vec![0i64; columns];
    for (row, pivot) in pivots.into_iter().enumerate() {
        if pivot < columns {
            solution[pivot] = reduced[row][columns] as i64;
        }
    }
    Ok(solution)
}

fn kernel_basis(matrix: &[Vec<i64>], modulus: u64) -> Result<Vec<Vec<i64>>, ChallengeError> {
    if matrix.is_empty() {
        return Ok(Vec::new());
    }
    let columns = rectangular(matrix)?;
    let (reduced, pivots) = rref(normalize_matrix(matrix, modulus), modulus)?;
    let pivot_set: BTreeSet<_> = pivots.iter().copied().collect();
    let mut basis = Vec::new();
    for free in 0..columns {
        if pivot_set.contains(&free) {
            continue;
        }
        let mut vector = vec![0u64; columns];
        vector[free] = 1;
        for (row, pivot) in pivots.iter().copied().enumerate().rev() {
            vector[pivot] = sub_mod(0, reduced[row][free], modulus);
        }
        basis.push(vector.into_iter().map(|value| value as i64).collect());
    }
    Ok(basis)
}

fn baseline_descent(fixture: &ChallengeFixture) -> Result<DescentSubmission, ChallengeError> {
    let order = fixture.curve.order as usize;
    let mut cost = vec![None; order];
    let mut split = vec![None; order];
    for point in &fixture.descent_tree.factor_base {
        cost[discrete_log(&fixture.curve, point)? as usize] = Some(0usize);
    }
    loop {
        let mut changed = false;
        for result in 1..order {
            for left in 1..order {
                let right = (result + order - left) % order;
                if right == 0 {
                    continue;
                }
                if let (Some(left_cost), Some(right_cost)) = (cost[left], cost[right]) {
                    let candidate = 1 + left_cost + right_cost;
                    if cost[result].is_none_or(|old| candidate < old) {
                        cost[result] = Some(candidate);
                        split[result] = Some((left, right));
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    let root_scalar = discrete_log(&fixture.curve, &fixture.descent_tree.root)? as usize;
    if cost[root_scalar].is_none() {
        return err("baseline cannot descend the fixture root");
    }
    fn build(
        fixture: &ChallengeFixture,
        scalar: usize,
        split: &[Option<(usize, usize)>],
        nodes: &mut Vec<DescentNode>,
        next: &mut usize,
    ) -> Result<String, ChallengeError> {
        let id = format!("n{}", *next);
        *next += 1;
        let point = scalar_mul(&fixture.curve, &fixture.curve.generator, scalar as u64)?;
        let children = if let Some((left, right)) = split[scalar] {
            vec![
                build(fixture, left, split, nodes, next)?,
                build(fixture, right, split, nodes, next)?,
            ]
        } else {
            Vec::new()
        };
        nodes.push(DescentNode {
            id: id.clone(),
            point,
            children,
        });
        Ok(id)
    }
    let mut nodes = Vec::new();
    let root = build(fixture, root_scalar, &split, &mut nodes, &mut 0)?;
    Ok(DescentSubmission { root, nodes })
}

fn baseline_orbits(fixture: &ChallengeFixture) -> Result<OrbitSubmission, ChallengeError> {
    let universe: BTreeSet<_> = fixture.orbit_relations.universe.iter().cloned().collect();
    let mut assigned = BTreeSet::new();
    let mut representatives = Vec::new();
    for point in &fixture.orbit_relations.universe {
        if assigned.contains(point) {
            continue;
        }
        let negative = negate(&fixture.curve, point);
        if !universe.contains(&negative) {
            return err("orbit universe is not closed");
        }
        let representative = point.clone().min(negative.clone());
        representatives.push(representative);
        assigned.insert(point.clone());
        assigned.insert(negative);
    }
    representatives.sort();
    let representative_set: BTreeSet<_> = representatives.iter().cloned().collect();
    let reconstruction: Vec<_> = fixture
        .orbit_relations
        .universe
        .iter()
        .map(|member| {
            if representative_set.contains(member) {
                OrbitMember {
                    member: member.clone(),
                    representative: member.clone(),
                    action: "identity".to_string(),
                }
            } else {
                OrbitMember {
                    member: member.clone(),
                    representative: negate(&fixture.curve, member),
                    action: "negation".to_string(),
                }
            }
        })
        .collect();
    let index: BTreeMap<_, _> = representatives
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, point)| (point, index))
        .collect();
    let records: BTreeMap<_, _> = reconstruction
        .iter()
        .map(|record| (record.member.clone(), record))
        .collect();
    let mut folded_rows = Vec::new();
    for row in &fixture.orbit_relations.relation_rows {
        let mut folded = vec![0u64; representatives.len()];
        for (column, coefficient) in row.iter().enumerate() {
            let record = records[&fixture.orbit_relations.universe[column]];
            let signed = if record.action == "identity" {
                *coefficient as i128
            } else {
                -(*coefficient as i128)
            };
            let target = index[&record.representative];
            folded[target] = add_mod(
                folded[target],
                mod_i128(signed, fixture.curve.order),
                fixture.curve.order,
            );
        }
        folded_rows.push(folded.into_iter().map(|value| value as i64).collect());
    }
    Ok(OrbitSubmission {
        representatives,
        reconstruction,
        folded_rows,
    })
}

pub fn baseline(fixture: &ChallengeFixture) -> Result<ChallengeSubmission, ChallengeError> {
    validate_fixture(fixture)?;
    let factor_count = fixture.relation_forge.factor_base.len();
    if fixture.relation_forge.max_rows < factor_count + 1 {
        return err("baseline needs factor-base width plus one relation rows");
    }
    let mut rows = Vec::new();
    for (index, point) in fixture.relation_forge.factor_base.iter().enumerate() {
        let mut coefficients = vec![0i64; factor_count];
        coefficients[index] = 1;
        rows.push(RelationRow {
            factor_coefficients: coefficients,
            a: discrete_log(&fixture.curve, point)? as i64,
            b: 0,
        });
    }
    let target_log = discrete_log(&fixture.curve, &fixture.curve.target)? as i64;
    rows.push(RelationRow {
        factor_coefficients: vec![0; factor_count],
        a: -target_log,
        b: 1,
    });
    let matrix = &fixture.matrix_smash;
    let columns = rectangular(&matrix.matrix)?;
    let transpose: Vec<Vec<i64>> = (0..columns)
        .map(|column| matrix.matrix.iter().map(|row| row[column]).collect())
        .collect();
    let rank = rank_i64(&matrix.matrix, matrix.modulus)?;
    let descent_tree = baseline_descent(fixture)?;
    let transfer_images = fixture
        .transfer_audit
        .sources
        .iter()
        .map(|source| {
            let image = map_point(&fixture.curve, &fixture.transfer_audit.map, source)?;
            let recovered = map_point(&fixture.curve, &fixture.transfer_audit.map, &image)?;
            Ok(TransferImage {
                source: source.clone(),
                image,
                recovered,
            })
        })
        .collect::<Result<Vec<_>, ChallengeError>>()?;
    Ok(ChallengeSubmission {
        schema: SUBMISSION_SCHEMA.to_string(),
        suite_id: fixture.suite_id.clone(),
        implementation: "rust-reference-baseline/v1".to_string(),
        relation_forge: RelationSubmission { rows },
        factor_base_lab: best_factor_base(fixture)?,
        matrix_smash: MatrixSubmission {
            solution: solve_unique(matrix)?,
            rank,
            left_kernel: kernel_basis(&transpose, matrix.modulus)?,
        },
        descent_tree,
        transfer_audit: TransferSubmission {
            map: fixture.transfer_audit.map.clone(),
            images: transfer_images,
            obligations: TransferObligations {
                homomorphism: "verified".to_string(),
                subgroup_preservation: "verified".to_string(),
                recovery: "verified".to_string(),
                asymptotic_advantage: "unknown".to_string(),
            },
        },
        orbit_relations: baseline_orbits(fixture)?,
    })
}

pub fn known_answer_fixture() -> Result<ChallengeFixture, ChallengeError> {
    let mut curve = CurveSpec {
        p: 17,
        a: 2,
        b: 2,
        order: 19,
        generator: Some(AffinePoint { x: 5, y: 1 }),
        target: None,
    };
    curve.target = scalar_mul(&curve, &curve.generator, 7)?;
    let point = |scalar| scalar_mul(&curve, &curve.generator, scalar);
    let relation_factor_base = (1..=4).map(point).collect::<Result<Vec<_>, _>>()?;
    let factor_universe = (1..=9).map(point).collect::<Result<Vec<_>, _>>()?;
    let factor_targets = (2..=10).map(point).collect::<Result<Vec<_>, _>>()?;
    let descent_factor_base = (1..=3).map(point).collect::<Result<Vec<_>, _>>()?;
    let transfer_sources = [1, 3, 7]
        .into_iter()
        .map(point)
        .collect::<Result<Vec<_>, _>>()?;
    let orbit_universe = (1..19).map(point).collect::<Result<Vec<_>, _>>()?;
    let descent_root = point(7)?;
    let mut orbit_rows = Vec::new();
    for index in 0..9 {
        let mut row = vec![0i64; orbit_universe.len()];
        row[index] = 1;
        orbit_rows.push(row);
    }
    let fixture = ChallengeFixture {
        schema: FIXTURE_SCHEMA.to_string(),
        suite_id: "ecdlp-ic-toy-p17-v1".to_string(),
        curve,
        relation_forge: RelationFixture {
            factor_base: relation_factor_base,
            max_rows: 5,
        },
        factor_base_lab: FactorBaseFixture {
            universe: factor_universe,
            targets: factor_targets,
            max_size: 5,
        },
        matrix_smash: MatrixFixture {
            modulus: 19,
            matrix: vec![
                vec![1, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 0],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1],
            ],
            rhs: vec![1, 2, 3, 4, 7, 17],
        },
        descent_tree: DescentFixture {
            root: descent_root,
            factor_base: descent_factor_base,
            max_nodes: 7,
        },
        transfer_audit: TransferFixture {
            map: "negation".to_string(),
            sources: transfer_sources,
        },
        orbit_relations: OrbitFixture {
            action: "negation".to_string(),
            universe: orbit_universe,
            relation_rows: orbit_rows,
        },
    };
    validate_fixture(&fixture)?;
    Ok(fixture)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_pair() -> (ChallengeFixture, ChallengeSubmission) {
        let fixture = known_answer_fixture().unwrap();
        let submission = baseline(&fixture).unwrap();
        (fixture, submission)
    }

    #[test]
    fn reference_baseline_verifies_all_six_tracks() {
        let (fixture, submission) = baseline_pair();
        let receipt = verify(&fixture, &submission).unwrap();
        assert_eq!(receipt.status, "verified");
        assert_eq!(receipt.tracks.len(), 6);
        assert_eq!(receipt.frontier.len(), 6);
        assert_eq!(
            receipt.tracks["relation_forge"].metrics["independent_rank"],
            5
        );
        assert_eq!(
            receipt.tracks["factor_base_lab"].metrics["decomposed_targets"],
            9
        );
        assert_eq!(receipt.tracks["matrix_smash"].metrics["rank"], 5);
        assert_eq!(
            receipt.tracks["transfer_audit"].obligations["asymptotic_advantage"],
            "unknown"
        );
        assert_eq!(receipt.claim_boundary.performance, "not_measured");
    }

    #[test]
    fn each_track_rejects_a_false_certificate() {
        let (fixture, baseline) = baseline_pair();

        let mut submission = baseline.clone();
        submission.relation_forge.rows[0].a += 1;
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("relation_forge"));

        let mut submission = baseline.clone();
        submission.factor_base_lab.decompositions[0].left = 1;
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("factor_base_lab"));

        let mut submission = baseline.clone();
        submission.matrix_smash.solution[0] += 1;
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("matrix_smash"));

        let mut submission = baseline.clone();
        let root = submission.descent_tree.root.clone();
        let root_node = submission
            .descent_tree
            .nodes
            .iter_mut()
            .find(|node| node.id == root)
            .unwrap();
        root_node.point = fixture.curve.generator.clone();
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("descent_tree"));

        let mut submission = baseline.clone();
        submission.transfer_audit.images[0].recovered = fixture.curve.target.clone();
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("transfer_audit"));

        let mut submission = baseline;
        submission.orbit_relations.folded_rows[0][0] += 1;
        assert!(verify(&fixture, &submission)
            .unwrap_err()
            .0
            .contains("orbit_relations"));
    }

    #[test]
    fn fixture_identity_and_claim_boundary_are_bound_into_receipt() {
        let (fixture, submission) = baseline_pair();
        let receipt = verify(&fixture, &submission).unwrap();
        assert_eq!(receipt.fixture_sha256.len(), 64);
        assert_eq!(receipt.submission_sha256.len(), 64);
        assert_ne!(receipt.fixture_sha256, receipt.submission_sha256);
        assert_eq!(receipt.claim_boundary.security, "not_established");
        assert_eq!(receipt.claim_boundary.transfer_advantage, "unknown");
    }
}
