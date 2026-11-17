//! Common-kernel analysis for declared finite primary modules.
//!
//! TorsionGuard audits a deliberately narrow algebraic question that appears
//! in elliptic-curve circuit reviews: whether supplied linear relations have a
//! nontrivial common kernel on a declared finite abelian cofactor module. It
//! does not infer a curve's group structure or validate circuit wiring. Those
//! facts remain explicit evidence obligations in the input.

use std::collections::{BTreeMap, BTreeSet};

use num_bigint::{BigInt, BigUint};
use num_traits::{One, ToPrimitive, Zero};
use serde_json::{json, Map, Value};

const MAX_COMPONENTS: usize = 32;
const MAX_RELATIONS: usize = 128;
const MAX_REPAIR_CANDIDATES: usize = 20;
const MAX_EXPONENT: u32 = 4096;
const REQUIRED_OBLIGATIONS: [&str; 6] = [
    "group_structure",
    "point_on_curve",
    "input_domain",
    "scalar_binding",
    "coefficient_bounds",
    "map_actions",
];

pub type Vector = Vec<BigInt>;
pub type Matrix = Vec<Vector>;

#[derive(Clone, Debug)]
pub struct Component {
    pub label: String,
    pub prime: BigInt,
    pub exponent: u32,
    pub modulus: BigInt,
    pub prime_proof: Option<String>,
    pub prime_validation: String,
}

#[derive(Clone, Debug)]
pub struct Obligation {
    pub status: String,
    pub evidence: String,
}

#[derive(Clone, Debug)]
pub struct Relation {
    pub name: String,
    pub evidence: String,
    pub matrix: Matrix,
}

#[derive(Clone, Debug)]
pub struct RepairCandidate {
    pub relation: Relation,
    pub cost: u64,
}

#[derive(Clone, Debug)]
pub struct Specification {
    pub name: String,
    pub description: String,
    pub components: Vec<Component>,
    pub obligations: BTreeMap<String, Obligation>,
    pub relations: Vec<Relation>,
    pub repair_candidates: Vec<RepairCandidate>,
}

#[derive(Clone, Debug)]
pub struct RowReduction {
    pub rref: Matrix,
    pub pivots: Vec<usize>,
    pub null_witness: Option<Vector>,
}

#[derive(Clone, Debug)]
pub struct PrimeAnalysis {
    pub prime: BigInt,
    pub component_indices: Vec<usize>,
    pub restricted_matrix: Matrix,
    pub reduction: RowReduction,
    pub lifted_witness: Option<Vector>,
    pub witness_evaluations: Option<BTreeMap<String, Vector>>,
}

impl PrimeAnalysis {
    pub fn sound(&self) -> bool {
        self.reduction.pivots.len() == self.component_indices.len()
    }
}

#[derive(Clone, Debug)]
pub struct Analysis {
    pub specification_name: String,
    pub declared_model: Value,
    pub algebraically_sound: bool,
    pub assumptions_complete: bool,
    pub verdict: String,
    pub missing_obligations: Vec<String>,
    pub conditional_prime_assumptions: Vec<String>,
    pub primes: Vec<PrimeAnalysis>,
}

#[derive(Clone, Debug)]
pub struct RepairAnalysis {
    pub found: bool,
    pub total_cost: u64,
    pub selected_candidates: Vec<String>,
    pub repaired_analysis: Option<Analysis>,
}

fn object<'a>(value: &'a Value, context: &str) -> Result<&'a Map<String, Value>, String> {
    value
        .as_object()
        .ok_or_else(|| format!("{context} must be an object"))
}

fn array<'a>(value: &'a Value, context: &str) -> Result<&'a Vec<Value>, String> {
    value
        .as_array()
        .ok_or_else(|| format!("{context} must be an array"))
}

fn required<'a>(
    object: &'a Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<&'a Value, String> {
    object
        .get(key)
        .ok_or_else(|| format!("{context}.{key} is required"))
}

fn required_string(
    object: &Map<String, Value>,
    key: &str,
    context: &str,
) -> Result<String, String> {
    let value = required(object, key, context)?
        .as_str()
        .ok_or_else(|| format!("{context}.{key} must be a string"))?;
    if value.is_empty() {
        return Err(format!("{context}.{key} must not be empty"));
    }
    Ok(value.to_owned())
}

fn reject_unknown(
    object: &Map<String, Value>,
    allowed: &[&str],
    context: &str,
) -> Result<(), String> {
    for key in object.keys() {
        if !allowed.contains(&key.as_str()) {
            return Err(format!("{context} contains unknown field '{key}'"));
        }
    }
    Ok(())
}

fn parse_bigint(value: &Value, context: &str) -> Result<BigInt, String> {
    let text = match value {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        _ => {
            return Err(format!(
                "{context} must be an integer string or JSON integer"
            ))
        }
    };
    if text.is_empty()
        || text == "-"
        || !text
            .strip_prefix('-')
            .unwrap_or(&text)
            .bytes()
            .all(|byte| byte.is_ascii_digit())
    {
        return Err(format!("{context} is not a base-10 integer: {text}"));
    }
    BigInt::parse_bytes(text.as_bytes(), 10)
        .ok_or_else(|| format!("{context} is not a base-10 integer: {text}"))
}

fn parse_u64(value: &Value, context: &str) -> Result<u64, String> {
    parse_bigint(value, context)?
        .to_u64()
        .ok_or_else(|| format!("{context} must be an unsigned 64-bit integer"))
}

fn power(base: &BigInt, mut exponent: u32) -> BigInt {
    let mut result = BigInt::one();
    let mut factor = base.clone();
    while exponent != 0 {
        if exponent & 1 == 1 {
            result *= &factor;
        }
        exponent >>= 1;
        if exponent != 0 {
            factor = &factor * &factor;
        }
    }
    result
}

pub fn positive_mod(value: &BigInt, modulus: &BigInt) -> BigInt {
    let result = value % modulus;
    if result < BigInt::zero() {
        result + modulus
    } else {
        result
    }
}

fn mul_mod_u64(lhs: u64, rhs: u64, modulus: u64) -> u64 {
    (u128::from(lhs) * u128::from(rhs) % u128::from(modulus)) as u64
}

fn pow_mod_u64(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1 % modulus;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result = mul_mod_u64(result, base, modulus);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = mul_mod_u64(base, base, modulus);
        }
    }
    result
}

fn is_prime_u64(candidate: u64) -> bool {
    if candidate < 2 {
        return false;
    }
    for prime in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if candidate.is_multiple_of(prime) {
            return candidate == prime;
        }
    }
    let mut odd_part = candidate - 1;
    let mut twos = 0;
    while odd_part & 1 == 0 {
        odd_part >>= 1;
        twos += 1;
    }
    for raw_base in [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022] {
        let base = raw_base % candidate;
        if base == 0 {
            continue;
        }
        let mut value = pow_mod_u64(base, odd_part, candidate);
        if value == 1 || value == candidate - 1 {
            continue;
        }
        let mut composite = true;
        for _ in 1..twos {
            value = mul_mod_u64(value, value, candidate);
            if value == candidate - 1 {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn is_probable_prime(candidate: &BigUint) -> bool {
    let small_primes = [
        2_u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
    ];
    if candidate < &BigUint::from(2_u32) {
        return false;
    }
    for prime in small_primes {
        let prime = BigUint::from(prime);
        if candidate == &prime {
            return true;
        }
        if candidate % &prime == BigUint::zero() {
            return false;
        }
    }
    let one = BigUint::one();
    let candidate_minus_one = candidate - &one;
    let mut odd_part = candidate_minus_one.clone();
    let mut twos = 0;
    while (&odd_part & &one).is_zero() {
        odd_part >>= 1;
        twos += 1;
    }
    for base in small_primes {
        let base = BigUint::from(base);
        let mut value = base.modpow(&odd_part, candidate);
        if value == one || value == candidate_minus_one {
            continue;
        }
        let mut composite = true;
        for _ in 1..twos {
            value = (&value * &value) % candidate;
            if value == candidate_minus_one {
                composite = false;
                break;
            }
        }
        if composite {
            return false;
        }
    }
    true
}

fn parse_component(value: &Value, index: usize) -> Result<Component, String> {
    let context = format!("torsion_components[{index}]");
    let object = object(value, &context)?;
    reject_unknown(
        object,
        &["label", "prime", "exponent", "prime_proof"],
        &context,
    )?;
    let label = required_string(object, "label", &context)?;
    let prime = parse_bigint(
        required(object, "prime", &context)?,
        &format!("{context}.prime"),
    )?;
    if prime < BigInt::from(2_u32) {
        return Err(format!("{context}.prime must be at least 2"));
    }
    let exponent_u64 = parse_u64(
        required(object, "exponent", &context)?,
        &format!("{context}.exponent"),
    )?;
    let exponent = u32::try_from(exponent_u64)
        .map_err(|_| format!("{context}.exponent must fit in 32 bits"))?;
    if exponent == 0 || exponent > MAX_EXPONENT {
        return Err(format!("{context}.exponent must be in [1, {MAX_EXPONENT}]"));
    }
    let prime_proof = object
        .get("prime_proof")
        .map(|value| {
            value
                .as_str()
                .filter(|text| !text.is_empty())
                .map(str::to_owned)
                .ok_or_else(|| format!("{context}.prime_proof must be a nonempty string"))
        })
        .transpose()?;
    let prime_validation = if let Some(candidate) = prime.to_u64() {
        if !is_prime_u64(candidate) {
            return Err(format!("{context}.prime is composite: {prime}"));
        }
        "deterministic_64_bit_miller_rabin".to_owned()
    } else {
        let unsigned = prime
            .to_biguint()
            .ok_or_else(|| format!("{context}.prime must be positive"))?;
        if !is_probable_prime(&unsigned) {
            return Err(format!(
                "{context}.prime failed probable-prime validation: {prime}"
            ));
        }
        if prime_proof.is_none() {
            return Err(format!(
                "{context}.prime_proof is required outside the deterministic 64-bit range"
            ));
        }
        "external_assumption_with_probable_prime_check".to_owned()
    };
    let modulus = power(&prime, exponent);
    Ok(Component {
        label,
        prime,
        exponent,
        modulus,
        prime_proof,
        prime_validation,
    })
}

fn parse_matrix(value: &Value, dimension: usize, context: &str) -> Result<Matrix, String> {
    let rows = array(value, context)?;
    if rows.len() != dimension {
        return Err(format!("{context} must contain {dimension} rows"));
    }
    rows.iter()
        .enumerate()
        .map(|(row_index, row)| {
            let row_context = format!("{context}[{row_index}]");
            let entries = array(row, &row_context)?;
            if entries.len() != dimension {
                return Err(format!("{row_context} must contain {dimension} entries"));
            }
            entries
                .iter()
                .enumerate()
                .map(|(column, entry)| parse_bigint(entry, &format!("{row_context}[{column}]")))
                .collect()
        })
        .collect()
}

fn parse_relation(
    value: &Value,
    dimension: usize,
    context: &str,
    repair_candidate: bool,
) -> Result<(Relation, Option<u64>), String> {
    let object = object(value, context)?;
    let allowed = if repair_candidate {
        &["name", "evidence", "matrix", "cost"][..]
    } else {
        &["name", "evidence", "matrix"][..]
    };
    reject_unknown(object, allowed, context)?;
    let relation = Relation {
        name: required_string(object, "name", context)?,
        evidence: required_string(object, "evidence", context)?,
        matrix: parse_matrix(
            required(object, "matrix", context)?,
            dimension,
            &format!("{context}.matrix"),
        )?,
    };
    let cost = if repair_candidate {
        Some(parse_u64(
            required(object, "cost", context)?,
            &format!("{context}.cost"),
        )?)
    } else {
        None
    };
    Ok((relation, cost))
}

pub fn is_valid_endomorphism(
    specification: &Specification,
    relation: &Relation,
) -> Result<(), String> {
    let dimension = specification.components.len();
    if relation.matrix.len() != dimension {
        return Err("matrix row count does not match torsion dimension".to_owned());
    }
    for (row, entries) in relation.matrix.iter().enumerate() {
        if entries.len() != dimension {
            return Err("matrix column count does not match torsion dimension".to_owned());
        }
        for (column, coefficient) in entries.iter().enumerate() {
            let transported = coefficient * &specification.components[column].modulus;
            if transported % &specification.components[row].modulus != BigInt::zero() {
                return Err(format!(
                    "entry [{row}][{column}] is not well-defined on the declared quotient groups"
                ));
            }
        }
    }
    Ok(())
}

pub fn parse_specification(value: &Value) -> Result<Specification, String> {
    let root = object(value, "input")?;
    reject_unknown(
        root,
        &[
            "schema_version",
            "name",
            "description",
            "torsion_components",
            "obligations",
            "relations",
            "repair_candidates",
        ],
        "input",
    )?;
    let schema_version = parse_u64(required(root, "schema_version", "input")?, "schema_version")?;
    if schema_version != 1 {
        return Err("only schema_version 1 is supported".to_owned());
    }
    let name = required_string(root, "name", "input")?;
    let description = root
        .get("description")
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| "input.description must be a string".to_owned())
        })
        .transpose()?
        .unwrap_or_default();
    let component_values = array(
        required(root, "torsion_components", "input")?,
        "torsion_components",
    )?;
    if component_values.is_empty() || component_values.len() > MAX_COMPONENTS {
        return Err(format!(
            "torsion_components must contain between 1 and {MAX_COMPONENTS} entries"
        ));
    }
    let components: Vec<Component> = component_values
        .iter()
        .enumerate()
        .map(|(index, component)| parse_component(component, index))
        .collect::<Result<_, _>>()?;
    let obligation_values = object(required(root, "obligations", "input")?, "obligations")?;
    let mut obligations = BTreeMap::new();
    for (name, value) in obligation_values {
        let context = format!("obligations.{name}");
        let entry = object(value, &context)?;
        reject_unknown(entry, &["status", "evidence"], &context)?;
        let status = required_string(entry, "status", &context)?;
        if !matches!(
            status.as_str(),
            "verified" | "not_applicable" | "unverified"
        ) {
            return Err(format!("{context}.status has invalid value '{status}'"));
        }
        obligations.insert(
            name.clone(),
            Obligation {
                status,
                evidence: required_string(entry, "evidence", &context)?,
            },
        );
    }
    let relation_values = array(required(root, "relations", "input")?, "relations")?;
    if relation_values.len() > MAX_RELATIONS {
        return Err(format!("relations exceeds the cap of {MAX_RELATIONS}"));
    }
    let partial = Specification {
        name: name.clone(),
        description: description.clone(),
        components: components.clone(),
        obligations: obligations.clone(),
        relations: Vec::new(),
        repair_candidates: Vec::new(),
    };
    let mut names = BTreeSet::new();
    let mut relations = Vec::with_capacity(relation_values.len());
    for (index, value) in relation_values.iter().enumerate() {
        let context = format!("relations[{index}]");
        let (relation, _) = parse_relation(value, components.len(), &context, false)?;
        if !names.insert(relation.name.clone()) {
            return Err(format!("duplicate relation name '{}'", relation.name));
        }
        is_valid_endomorphism(&partial, &relation)
            .map_err(|reason| format!("relation '{}' is invalid: {reason}", relation.name))?;
        relations.push(relation);
    }
    let candidate_values: &[Value] = match root.get("repair_candidates") {
        Some(value) => array(value, "repair_candidates")?,
        None => &[],
    };
    if candidate_values.len() > MAX_REPAIR_CANDIDATES {
        return Err(format!(
            "repair_candidates exceeds the exhaustive-search cap of {MAX_REPAIR_CANDIDATES}"
        ));
    }
    let mut repair_candidates = Vec::with_capacity(candidate_values.len());
    for (index, value) in candidate_values.iter().enumerate() {
        let context = format!("repair_candidates[{index}]");
        let (relation, cost) = parse_relation(value, components.len(), &context, true)?;
        if !names.insert(relation.name.clone()) {
            return Err(format!(
                "duplicate relation/candidate name '{}'",
                relation.name
            ));
        }
        is_valid_endomorphism(&partial, &relation).map_err(|reason| {
            format!("repair candidate '{}' is invalid: {reason}", relation.name)
        })?;
        repair_candidates.push(RepairCandidate {
            relation,
            cost: cost.expect("repair candidate parser always supplies a cost"),
        });
    }
    Ok(Specification {
        name,
        description,
        components,
        obligations,
        relations,
        repair_candidates,
    })
}

fn modular_inverse(value: &BigInt, prime: &BigInt) -> Result<BigInt, String> {
    let value = positive_mod(value, prime)
        .to_biguint()
        .ok_or_else(|| "internal modular conversion failed".to_owned())?;
    let modulus = prime
        .to_biguint()
        .ok_or_else(|| "internal prime conversion failed".to_owned())?;
    let exponent = &modulus - BigUint::from(2_u32);
    Ok(BigInt::from(value.modpow(&exponent, &modulus)))
}

pub fn reduce_matrix(matrix: &Matrix, prime: &BigInt) -> Result<RowReduction, String> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Err("matrix must contain at least one row and column".to_owned());
    }
    let columns = matrix[0].len();
    if matrix.iter().any(|row| row.len() != columns) {
        return Err("matrix rows have inconsistent lengths".to_owned());
    }
    let mut rref: Matrix = matrix
        .iter()
        .map(|row| row.iter().map(|value| positive_mod(value, prime)).collect())
        .collect();
    let mut pivots = Vec::new();
    let mut pivot_row = 0;
    for column in 0..columns {
        if pivot_row == rref.len() {
            break;
        }
        let Some(selected) = (pivot_row..rref.len()).find(|&row| !rref[row][column].is_zero())
        else {
            continue;
        };
        rref.swap(pivot_row, selected);
        let inverse = modular_inverse(&rref[pivot_row][column], prime)?;
        for entry in &mut rref[pivot_row] {
            *entry = positive_mod(&(&*entry * &inverse), prime);
        }
        let normalized_pivot = rref[pivot_row].clone();
        for (row_index, row) in rref.iter_mut().enumerate() {
            if row_index == pivot_row || row[column].is_zero() {
                continue;
            }
            let factor = row[column].clone();
            for entry_column in 0..columns {
                let difference = &row[entry_column] - &factor * &normalized_pivot[entry_column];
                row[entry_column] = positive_mod(&difference, prime);
            }
        }
        pivots.push(column);
        pivot_row += 1;
    }
    let null_witness = if pivots.len() < columns {
        let pivot_set: BTreeSet<usize> = pivots.iter().copied().collect();
        let free_column = (0..columns)
            .find(|column| !pivot_set.contains(column))
            .expect("rank defect guarantees a free column");
        let mut witness = vec![BigInt::zero(); columns];
        witness[free_column] = BigInt::one();
        for (row, pivot) in pivots.iter().copied().enumerate() {
            witness[pivot] = positive_mod(&(-&rref[row][free_column]), prime);
        }
        Some(witness)
    } else {
        None
    };
    Ok(RowReduction {
        rref,
        pivots,
        null_witness,
    })
}

fn restrict_relation(
    specification: &Specification,
    relation: &Relation,
    prime: &BigInt,
    indices: &[usize],
) -> Result<Matrix, String> {
    let mut result = vec![vec![BigInt::zero(); indices.len()]; indices.len()];
    for (local_row, &global_row) in indices.iter().enumerate() {
        let output = &specification.components[global_row];
        for (local_column, &global_column) in indices.iter().enumerate() {
            let input = &specification.components[global_column];
            let mut coefficient = relation.matrix[global_row][global_column].clone();
            if input.exponent >= output.exponent {
                coefficient *= power(prime, input.exponent - output.exponent);
            } else {
                let divisor = power(prime, output.exponent - input.exponent);
                if &coefficient % &divisor != BigInt::zero() {
                    return Err("endomorphism validation failed before restriction".to_owned());
                }
                coefficient /= divisor;
            }
            result[local_row][local_column] = positive_mod(&coefficient, prime);
        }
    }
    Ok(result)
}

fn apply_relation(specification: &Specification, relation: &Relation, input: &[BigInt]) -> Vector {
    relation
        .matrix
        .iter()
        .enumerate()
        .map(|(row, coefficients)| {
            let value: BigInt = coefficients
                .iter()
                .zip(input)
                .map(|(coefficient, coordinate)| coefficient * coordinate)
                .sum();
            positive_mod(&value, &specification.components[row].modulus)
        })
        .collect()
}

fn vector_is_zero(vector: &[BigInt]) -> bool {
    vector.iter().all(Zero::is_zero)
}

fn integer_json(value: &BigInt) -> Value {
    Value::String(value.to_string())
}

fn vector_json(vector: &[BigInt]) -> Value {
    Value::Array(vector.iter().map(integer_json).collect())
}

fn matrix_json(matrix: &[Vector]) -> Value {
    Value::Array(matrix.iter().map(|row| vector_json(row)).collect())
}

fn normalized_model(specification: &Specification) -> Value {
    let obligations: Map<String, Value> = specification
        .obligations
        .iter()
        .map(|(name, obligation)| {
            (
                name.clone(),
                json!({"status": obligation.status, "evidence": obligation.evidence}),
            )
        })
        .collect();
    let components: Vec<Value> = specification
        .components
        .iter()
        .map(|component| {
            json!({
                "label": component.label,
                "prime": component.prime.to_string(),
                "exponent": component.exponent,
                "modulus": component.modulus.to_string(),
                "prime_proof": component.prime_proof,
                "prime_validation": component.prime_validation,
            })
        })
        .collect();
    let relations: Vec<Value> = specification
        .relations
        .iter()
        .map(|relation| {
            json!({
                "name": relation.name,
                "evidence": relation.evidence,
                "matrix": matrix_json(&relation.matrix),
            })
        })
        .collect();
    let repair_candidates: Vec<Value> = specification
        .repair_candidates
        .iter()
        .map(|candidate| {
            json!({
                "name": candidate.relation.name,
                "cost": candidate.cost,
                "evidence": candidate.relation.evidence,
                "matrix": matrix_json(&candidate.relation.matrix),
            })
        })
        .collect();
    json!({
        "name": specification.name,
        "description": specification.description,
        "torsion_components": components,
        "obligations": obligations,
        "relations": relations,
        "repair_candidates": repair_candidates,
    })
}

pub fn analyze(specification: &Specification) -> Result<Analysis, String> {
    for relation in &specification.relations {
        is_valid_endomorphism(specification, relation)
            .map_err(|reason| format!("relation '{}' is invalid: {reason}", relation.name))?;
    }
    let mut missing_obligations = Vec::new();
    for required in REQUIRED_OBLIGATIONS {
        if !specification
            .obligations
            .get(required)
            .is_some_and(|obligation| {
                matches!(obligation.status.as_str(), "verified" | "not_applicable")
            })
        {
            missing_obligations.push(required.to_owned());
        }
    }
    let assumptions_complete = missing_obligations.is_empty();
    let conditional_prime_assumptions = specification
        .components
        .iter()
        .filter(|component| component.prime_validation != "deterministic_64_bit_miller_rabin")
        .map(|component| {
            format!(
                "{}: {}",
                component.label,
                component
                    .prime_proof
                    .as_deref()
                    .unwrap_or("missing proof reference")
            )
        })
        .collect();
    let distinct_primes: BTreeSet<BigInt> = specification
        .components
        .iter()
        .map(|component| component.prime.clone())
        .collect();
    let mut primes = Vec::new();
    let mut algebraically_sound = true;
    for prime in distinct_primes {
        let component_indices: Vec<usize> = specification
            .components
            .iter()
            .enumerate()
            .filter_map(|(index, component)| (component.prime == prime).then_some(index))
            .collect();
        let mut restricted_matrix = Vec::new();
        for relation in &specification.relations {
            restricted_matrix.extend(restrict_relation(
                specification,
                relation,
                &prime,
                &component_indices,
            )?);
        }
        if restricted_matrix.is_empty() {
            restricted_matrix.push(vec![BigInt::zero(); component_indices.len()]);
        }
        let reduction = reduce_matrix(&restricted_matrix, &prime)?;
        let (lifted_witness, witness_evaluations) = if let Some(local) = &reduction.null_witness {
            algebraically_sound = false;
            let mut lifted = vec![BigInt::zero(); specification.components.len()];
            for (local_index, &global_index) in component_indices.iter().enumerate() {
                let component = &specification.components[global_index];
                let value = &local[local_index] * power(&prime, component.exponent - 1);
                lifted[global_index] = positive_mod(&value, &component.modulus);
            }
            if vector_is_zero(&lifted) {
                return Err("internal error: constructed kernel witness is zero".to_owned());
            }
            let mut evaluations = BTreeMap::new();
            for relation in &specification.relations {
                let evaluation = apply_relation(specification, relation, &lifted);
                if !vector_is_zero(&evaluation) {
                    return Err(
                        "internal error: restricted-kernel witness failed full-module verification"
                            .to_owned(),
                    );
                }
                evaluations.insert(relation.name.clone(), evaluation);
            }
            (Some(lifted), Some(evaluations))
        } else {
            (None, None)
        };
        primes.push(PrimeAnalysis {
            prime,
            component_indices,
            restricted_matrix,
            reduction,
            lifted_witness,
            witness_evaluations,
        });
    }
    let verdict = if !algebraically_sound {
        "unsound_in_declared_model"
    } else if !assumptions_complete {
        "not_established_missing_obligations"
    } else {
        "sound_under_declared_assumptions"
    };
    Ok(Analysis {
        specification_name: specification.name.clone(),
        declared_model: normalized_model(specification),
        algebraically_sound,
        assumptions_complete,
        verdict: verdict.to_owned(),
        missing_obligations,
        conditional_prime_assumptions,
        primes,
    })
}

impl Analysis {
    pub fn to_json(&self) -> Value {
        let prime_components: Vec<Value> = self
            .primes
            .iter()
            .map(|prime| {
                let evaluations = prime.witness_evaluations.as_ref().map(|evaluations| {
                    Value::Object(
                        evaluations
                            .iter()
                            .map(|(name, value)| (name.clone(), vector_json(value)))
                            .collect(),
                    )
                });
                json!({
                    "prime": prime.prime.to_string(),
                    "component_indices": prime.component_indices,
                    "dimension": prime.component_indices.len(),
                    "restricted_matrix": matrix_json(&prime.restricted_matrix),
                    "rref": matrix_json(&prime.reduction.rref),
                    "rank": prime.reduction.pivots.len(),
                    "pivot_columns": prime.reduction.pivots,
                    "sound": prime.sound(),
                    "kernel_witness_mod_prime": prime.reduction.null_witness.as_deref().map(vector_json),
                    "lifted_witness": prime.lifted_witness.as_deref().map(vector_json),
                    "witness_evaluations": evaluations,
                })
            })
            .collect();
        json!({
            "tool": "torsionguard",
            "certificate_version": 1,
            "specification": self.specification_name,
            "declared_model": self.declared_model,
            "algebraically_sound": self.algebraically_sound,
            "assumptions_complete": self.assumptions_complete,
            "missing_obligations": self.missing_obligations,
            "conditional_prime_assumptions": self.conditional_prime_assumptions,
            "prime_components": prime_components,
            "verdict": self.verdict,
        })
    }
}

fn subset_better(
    cost: u64,
    names: &[String],
    best_cost: Option<u64>,
    best_names: &[String],
) -> bool {
    match best_cost {
        None => true,
        Some(best) if cost != best => cost < best,
        Some(_) if names.len() != best_names.len() => names.len() < best_names.len(),
        Some(_) => names < best_names,
    }
}

pub fn synthesize_repair(specification: &Specification) -> Result<RepairAnalysis, String> {
    let baseline = analyze(specification)?;
    if baseline.algebraically_sound {
        return Ok(RepairAnalysis {
            found: true,
            total_cost: 0,
            selected_candidates: Vec::new(),
            repaired_analysis: Some(baseline),
        });
    }
    let mut best_cost = None;
    let mut best_names = Vec::new();
    let mut best_indices = Vec::new();
    let subset_count = 1_u64 << specification.repair_candidates.len();
    for mask in 1..subset_count {
        let mut cost = 0_u64;
        let mut indices = Vec::new();
        let mut overflow = false;
        for (index, candidate) in specification.repair_candidates.iter().enumerate() {
            if mask & (1_u64 << index) != 0 {
                let Some(next) = cost.checked_add(candidate.cost) else {
                    overflow = true;
                    break;
                };
                cost = next;
                indices.push(index);
            }
        }
        if overflow || best_cost.is_some_and(|best| cost > best) {
            continue;
        }
        let mut candidate_specification = specification.clone();
        for &index in &indices {
            candidate_specification
                .relations
                .push(specification.repair_candidates[index].relation.clone());
        }
        let candidate_analysis = analyze(&candidate_specification)?;
        if !candidate_analysis.algebraically_sound {
            continue;
        }
        let mut names: Vec<String> = indices
            .iter()
            .map(|&index| specification.repair_candidates[index].relation.name.clone())
            .collect();
        names.sort();
        if subset_better(cost, &names, best_cost, &best_names) {
            best_cost = Some(cost);
            best_names = names;
            best_indices = indices;
        }
    }
    let Some(total_cost) = best_cost else {
        return Ok(RepairAnalysis {
            found: false,
            total_cost: 0,
            selected_candidates: Vec::new(),
            repaired_analysis: None,
        });
    };
    let mut repaired = specification.clone();
    for index in best_indices {
        repaired
            .relations
            .push(specification.repair_candidates[index].relation.clone());
    }
    Ok(RepairAnalysis {
        found: true,
        total_cost,
        selected_candidates: best_names,
        repaired_analysis: Some(analyze(&repaired)?),
    })
}

impl RepairAnalysis {
    pub fn to_json(&self) -> Value {
        json!({
            "tool": "torsionguard",
            "found": self.found,
            "total_cost": self.total_cost,
            "selected_candidates": self.selected_candidates,
            "repaired_analysis": self.repaired_analysis.as_ref().map(Analysis::to_json),
        })
    }
}

pub fn validation_json(specification: &Specification) -> Value {
    json!({
        "tool": "torsionguard",
        "valid": true,
        "name": specification.name,
        "components": specification.components.len(),
        "relations": specification.relations.len(),
        "repair_candidates": specification.repair_candidates.len(),
        "normalized_model": normalized_model(specification),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn component(prime: u64, exponent: u32, label: &str) -> Component {
        let prime = BigInt::from(prime);
        Component {
            label: label.to_owned(),
            modulus: power(&prime, exponent),
            prime,
            exponent,
            prime_proof: None,
            prime_validation: "deterministic_64_bit_miller_rabin".to_owned(),
        }
    }

    fn complete_obligations() -> BTreeMap<String, Obligation> {
        REQUIRED_OBLIGATIONS
            .iter()
            .map(|name| {
                (
                    (*name).to_owned(),
                    Obligation {
                        status: "verified".to_owned(),
                        evidence: "unit-test fixture".to_owned(),
                    },
                )
            })
            .collect()
    }

    fn specification(components: Vec<Component>, relations: Vec<Relation>) -> Specification {
        Specification {
            name: "unit-test".to_owned(),
            description: String::new(),
            components,
            obligations: complete_obligations(),
            relations,
            repair_candidates: Vec::new(),
        }
    }

    fn relation(name: &str, matrix: &[&[i64]]) -> Relation {
        Relation {
            name: name.to_owned(),
            evidence: "unit-test relation".to_owned(),
            matrix: matrix
                .iter()
                .map(|row| row.iter().copied().map(BigInt::from).collect())
                .collect(),
        }
    }

    #[test]
    fn finds_lifted_kernel_witness_and_minimum_repair() {
        let mut specification = specification(
            vec![component(2, 1, "H2")],
            vec![relation("weak_v2", &[&[2]])],
        );
        specification.repair_candidates = vec![
            RepairCandidate {
                relation: relation("no_effect", &[&[4]]),
                cost: 1,
            },
            RepairCandidate {
                relation: relation("bind_subgroup", &[&[7]]),
                cost: 5,
            },
        ];
        let analysis = analyze(&specification).unwrap();
        assert!(!analysis.algebraically_sound);
        assert_eq!(analysis.verdict, "unsound_in_declared_model");
        assert_eq!(
            analysis.primes[0].lifted_witness.as_ref().unwrap(),
            &vec![BigInt::one()]
        );
        assert_eq!(
            analysis.primes[0].witness_evaluations.as_ref().unwrap()["weak_v2"],
            vec![BigInt::zero()]
        );
        let repair = synthesize_repair(&specification).unwrap();
        assert!(repair.found);
        assert_eq!(repair.total_cost, 5);
        assert_eq!(repair.selected_candidates, vec!["bind_subgroup"]);
        assert!(repair.repaired_analysis.unwrap().algebraically_sound);
    }

    #[test]
    fn handles_prime_powers_and_joint_relations() {
        let prime_power = specification(
            vec![component(3, 2, "H9")],
            vec![relation("times_three", &[&[3]])],
        );
        let analysis = analyze(&prime_power).unwrap();
        assert_eq!(
            analysis.primes[0].lifted_witness.as_ref().unwrap(),
            &vec![BigInt::from(3)]
        );
        let two_dimensional = specification(
            vec![component(2, 1, "first"), component(2, 1, "second")],
            vec![
                relation("bind_first", &[&[1, 0], &[0, 0]]),
                relation("bind_second", &[&[0, 0], &[0, 1]]),
            ],
        );
        let combined = analyze(&two_dimensional).unwrap();
        assert!(combined.algebraically_sound);
        assert_eq!(combined.primes[0].reduction.pivots.len(), 2);
    }

    #[test]
    fn rejects_invalid_quotient_map_and_gates_obligations() {
        let invalid = specification(
            vec![component(2, 2, "Z4"), component(2, 1, "Z2")],
            Vec::new(),
        );
        let error =
            is_valid_endomorphism(&invalid, &relation("bad", &[&[0, 1], &[0, 0]])).unwrap_err();
        assert!(error.contains("not well-defined"));
        let mut missing = specification(
            vec![component(2, 1, "H2")],
            vec![relation("identity", &[&[1]])],
        );
        missing.obligations.remove("point_on_curve");
        let analysis = analyze(&missing).unwrap();
        assert!(analysis.algebraically_sound);
        assert!(!analysis.assumptions_complete);
        assert_eq!(analysis.verdict, "not_established_missing_obligations");
    }

    fn brute_force_sound(specification: &Specification) -> bool {
        let moduli: Vec<u64> = specification
            .components
            .iter()
            .map(|component| component.modulus.to_u64().unwrap())
            .collect();
        let combinations: u64 = moduli.iter().product();
        for encoded in 1..combinations {
            let mut remainder = encoded;
            let input: Vector = moduli
                .iter()
                .map(|modulus| {
                    let coordinate = remainder % modulus;
                    remainder /= modulus;
                    BigInt::from(coordinate)
                })
                .collect();
            if specification
                .relations
                .iter()
                .all(|relation| vector_is_zero(&apply_relation(specification, relation, &input)))
            {
                return false;
            }
        }
        true
    }

    #[test]
    fn rank_criterion_matches_10000_exhaustive_small_modules() {
        const SEED: u64 = 0x0054_4f52_5349_4f4e;
        let mut rng = StdRng::seed_from_u64(SEED);
        let primes = [2_u64, 3, 5, 7];
        for instance in 0..10_000 {
            let prime = primes[rng.gen_range(0..primes.len())];
            let dimension = rng.gen_range(1..=2);
            let components: Vec<Component> = (0..dimension)
                .map(|index| component(prime, rng.gen_range(1..=2), &format!("c{index}")))
                .collect();
            let relation_count = rng.gen_range(1..=3);
            let mut relations = Vec::new();
            for relation_index in 0..relation_count {
                let mut matrix = vec![vec![BigInt::zero(); dimension]; dimension];
                for row in 0..dimension {
                    for column in 0..dimension {
                        let output_exponent = components[row].exponent;
                        let input_exponent = components[column].exponent;
                        let output_modulus = components[row].modulus.to_u64().unwrap();
                        matrix[row][column] = if output_exponent > input_exponent {
                            let divisor = prime.pow(output_exponent - input_exponent);
                            BigInt::from(rng.gen_range(0..output_modulus / divisor) * divisor)
                        } else {
                            BigInt::from(rng.gen_range(0..output_modulus))
                        };
                    }
                }
                relations.push(Relation {
                    name: format!("r{relation_index}"),
                    evidence: "generated valid quotient map".to_owned(),
                    matrix,
                });
            }
            let specification = specification(components, relations);
            for relation in &specification.relations {
                is_valid_endomorphism(&specification, relation).unwrap();
            }
            let expected = brute_force_sound(&specification);
            let observed = analyze(&specification).unwrap().algebraically_sound;
            assert_eq!(observed, expected, "generated instance {instance}");
        }
    }

    #[test]
    fn parses_strict_json_and_rejects_composite_base() {
        let fixture = json!({
            "schema_version": 1,
            "name": "json-fixture",
            "torsion_components": [{"label": "H2", "prime": "2", "exponent": 1}],
            "obligations": {
                "group_structure": {"status": "verified", "evidence": "fixture"},
                "point_on_curve": {"status": "verified", "evidence": "fixture"},
                "input_domain": {"status": "verified", "evidence": "fixture"},
                "scalar_binding": {"status": "verified", "evidence": "fixture"},
                "coefficient_bounds": {"status": "not_applicable", "evidence": "fixed"},
                "map_actions": {"status": "not_applicable", "evidence": "scalar map"}
            },
            "relations": [{"name": "identity", "evidence": "fixture", "matrix": [["1"]]}]
        });
        assert!(
            analyze(&parse_specification(&fixture).unwrap())
                .unwrap()
                .algebraically_sound
        );
        let mut composite = fixture;
        composite["torsion_components"][0]["prime"] = Value::String("15".to_owned());
        assert!(parse_specification(&composite)
            .unwrap_err()
            .contains("composite"));
    }
}
