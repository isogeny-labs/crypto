//! Construct the guarded three-abscissa degree-five proof without closure search.
//!
//! This is a native port of the historical constructive_five.py algebra, using
//! arbitrary-precision residues. The emitted elementary DAG is intended for
//! independent sparse replay; the generator does not certify field primality,
//! subgroup order, or the rational-lift interpretation.

use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Zero};
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, value::RawValue, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

type Polynomial = BTreeMap<Vec<u32>, BigUint>;
type Result<T> = std::result::Result<T, ConstructionError>;
const CAP: u32 = 5;
const MAX_MODULUS_BITS: u64 = 4096;
const MAX_INPUT_TERMS: usize = 10_000;
const MAX_NODES: usize = 1_000_000;
const MAX_DOCUMENT_BYTES: usize = 256 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct ConstructionError(String);

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl std::error::Error for ConstructionError {}

fn fail(message: impl Into<String>) -> ConstructionError {
    ConstructionError(message.into())
}
fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(fail(message))
    }
}

#[derive(Clone)]
struct Decimal(BigUint);
impl Serialize for Decimal {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        RawValue::from_string(self.0.to_string())
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }
}

#[derive(Deserialize)]
struct RawCurve {
    a: Box<RawValue>,
    b: Box<RawValue>,
}
#[derive(Deserialize)]
struct RawProblem {
    p: Box<RawValue>,
    n: u32,
    polynomials: Vec<Vec<(Box<RawValue>, Vec<u32>)>>,
    curve: RawCurve,
    factor_base_abscissas: Vec<Box<RawValue>>,
    target: Vec<Box<RawValue>>,
}

fn integer(value: &RawValue, label: &str) -> Result<BigInt> {
    let text = value.get();
    let digits = text.strip_prefix('-').unwrap_or(text);
    require(
        !digits.is_empty()
            && digits.bytes().all(|b| b.is_ascii_digit())
            && !(digits.len() > 1 && digits.starts_with('0'))
            && text != "-0",
        &format!("{label} must be a JSON integer token"),
    )?;
    require(
        text.len() <= 3000,
        "integer token exceeds the construction limit",
    )?;
    text.parse::<BigInt>()
        .map_err(|_| fail(format!("{label} is not a valid integer")))
}

#[derive(Clone)]
struct Field {
    p: BigUint,
}
impl Field {
    fn signed(&self, value: BigInt) -> BigUint {
        value
            .mod_floor(&BigInt::from(self.p.clone()))
            .to_biguint()
            .expect("a nonnegative modular residue")
    }
    fn small(&self, value: i64) -> BigUint {
        self.signed(BigInt::from(value))
    }
    fn neg(&self, value: &BigUint) -> BigUint {
        if value.is_zero() {
            BigUint::zero()
        } else {
            &self.p - value
        }
    }
    fn sub(&self, left: &BigUint, right: &BigUint) -> BigUint {
        (left + &self.p - right) % &self.p
    }
    fn inv(&self, value: &BigUint) -> Result<BigUint> {
        let modulus = BigInt::from(self.p.clone());
        let extended = BigInt::from(value.clone()).extended_gcd(&modulus);
        require(
            extended.gcd.is_one(),
            "a required modular inverse is not a unit",
        )?;
        Ok(extended
            .x
            .mod_floor(&modulus)
            .to_biguint()
            .expect("a nonnegative modular inverse"))
    }
    fn term(&self, exponents: Vec<u32>, value: BigUint) -> Polynomial {
        let value = value % &self.p;
        if value.is_zero() {
            Polynomial::new()
        } else {
            BTreeMap::from([(exponents, value)])
        }
    }
    fn constant(&self, value: BigUint, variables: usize) -> Polynomial {
        self.term(vec![0; variables], value)
    }
    fn constant_small(&self, value: i64, variables: usize) -> Polynomial {
        self.constant(self.small(value), variables)
    }
    fn variable(&self, axis: usize, variables: usize) -> Polynomial {
        let mut exponents = vec![0; variables];
        exponents[axis] = 1;
        self.term(exponents, BigUint::one())
    }
    fn insert(&self, poly: &mut Polynomial, exponents: Vec<u32>, coefficient: BigUint) {
        let value = (poly.get(&exponents).cloned().unwrap_or_default() + coefficient) % &self.p;
        if value.is_zero() {
            poly.remove(&exponents);
        } else {
            poly.insert(exponents, value);
        }
    }
    fn scale(&self, poly: &Polynomial, coefficient: &BigUint) -> Polynomial {
        poly.iter()
            .filter_map(|(exponents, value)| {
                let value = value * coefficient % &self.p;
                (!value.is_zero()).then(|| (exponents.clone(), value))
            })
            .collect()
    }
    fn add_scaled(
        &self,
        left: &Polynomial,
        right: &Polynomial,
        coefficient: &BigUint,
    ) -> Polynomial {
        let mut output = left.clone();
        for (exponents, value) in right {
            self.insert(&mut output, exponents.clone(), value * coefficient);
        }
        output
    }
    fn add(&self, left: &Polynomial, right: &Polynomial) -> Polynomial {
        self.add_scaled(left, right, &BigUint::one())
    }
    fn mul(&self, left: &Polynomial, right: &Polynomial) -> Polynomial {
        let mut output = Polynomial::new();
        for (exponents, value) in left {
            for (powers, coefficient) in right {
                let combined = exponents.iter().zip(powers).map(|(a, b)| a + b).collect();
                self.insert(&mut output, combined, value * coefficient);
            }
        }
        output
    }
    fn evaluate(&self, poly: &Polynomial, values: &[BigUint]) -> BigUint {
        let mut total = BigUint::zero();
        for (powers, coefficient) in poly {
            let mut term = coefficient.clone();
            for (value, exponent) in values.iter().zip(powers) {
                term = term * value.modpow(&BigUint::from(*exponent), &self.p) % &self.p;
            }
            total = (total + term) % &self.p;
        }
        total
    }
    fn lagrange(&self, base: &[BigUint], root: &BigUint, axis: usize) -> Result<Polynomial> {
        let mut output = self.constant_small(1, 3);
        let mut denominator = BigUint::one();
        let variable = self.variable(axis, 3);
        for other in base {
            if other == root {
                continue;
            }
            output = self.mul(
                &output,
                &self.add(&variable, &self.constant(self.neg(other), 3)),
            );
            denominator = denominator * self.sub(root, other) % &self.p;
        }
        Ok(self.scale(&output, &self.inv(&denominator)?))
    }
}

fn degree(poly: &Polynomial) -> u32 {
    poly.keys().map(|e| e.iter().sum()).max().unwrap_or(0)
}

fn divide(
    field: &Field,
    input: &Polynomial,
    divisors: &[Polynomial],
) -> Result<(Vec<Polynomial>, Polynomial)> {
    let mut work = input.clone();
    let mut quotients = vec![Polynomial::new(); divisors.len()];
    let mut remainder = Polynomial::new();
    let leads: Vec<Vec<u32>> = divisors
        .iter()
        .map(|divisor| {
            divisor
                .last_key_value()
                .map(|(e, _)| e.clone())
                .ok_or_else(|| fail("zero divisor in monic division"))
        })
        .collect::<Result<_>>()?;
    for (divisor, lead) in divisors.iter().zip(&leads) {
        require(
            divisor[lead].is_one() && lead.iter().sum::<u32>() == degree(divisor),
            "division requires monic total-degree-preserving divisors",
        )?;
    }
    while let Some((exponents, coefficient)) =
        work.last_key_value().map(|(e, c)| (e.clone(), c.clone()))
    {
        let mut reduced = false;
        for (index, (divisor, lead)) in divisors.iter().zip(&leads).enumerate() {
            if exponents.iter().zip(lead).all(|(a, b)| a >= b) {
                let shift: Vec<u32> = exponents.iter().zip(lead).map(|(a, b)| a - b).collect();
                let term = field.term(shift, coefficient.clone());
                quotients[index] = field.add(&quotients[index], &term);
                work = field.add_scaled(&work, &field.mul(&term, divisor), &field.small(-1));
                reduced = true;
                break;
            }
        }
        if !reduced {
            remainder.insert(exponents.clone(), coefficient);
            work.remove(&exponents);
        }
    }
    Ok((quotients, remainder))
}

fn summation(
    field: &Field,
    a: &BigUint,
    b: &BigUint,
    x: &Polynomial,
    y: &Polynomial,
    z: &Polynomial,
) -> Polynomial {
    let variables = x
        .keys()
        .next()
        .or_else(|| y.keys().next())
        .map_or(5, Vec::len);
    let xy = field.mul(x, y);
    let plus = field.add(x, y);
    let minus = field.add_scaled(x, y, &field.small(-1));
    let middle = field.add(
        &field.mul(
            &plus,
            &field.add(&xy, &field.constant(a.clone(), variables)),
        ),
        &field.constant((b * BigUint::from(2u8)) % &field.p, variables),
    );
    let tail = field.add(&xy, &field.constant(field.neg(a), variables));
    let mut output = field.mul(&field.mul(&minus, &minus), &field.mul(z, z));
    output = field.add_scaled(&output, &field.mul(&middle, z), &field.small(-2));
    output = field.add(&output, &field.mul(&tail, &tail));
    field.add_scaled(
        &output,
        &plus,
        &field.neg(&((b * BigUint::from(4u8)) % &field.p)),
    )
}

struct Problem {
    field: Field,
    polynomials: Vec<Polynomial>,
    base: Vec<BigUint>,
    a: BigUint,
    b: BigUint,
}
impl Problem {
    fn parse(input: &str) -> Result<Self> {
        let raw: RawProblem =
            serde_json::from_str(input).map_err(|e| fail(format!("invalid problem JSON: {e}")))?;
        let modulus = integer(&raw.p, "p")?;
        require(modulus.sign() == Sign::Plus, "p must be positive")?;
        let p = modulus.to_biguint().expect("positive modulus");
        require(p > BigUint::from(3u8), "construction requires p > 3")?;
        require(
            p.bits() <= MAX_MODULUS_BITS,
            "modulus exceeds the construction limit",
        )?;
        require(
            raw.n == 5 && raw.polynomials.len() == 6,
            "expected five variables and six guarded-chain inputs",
        )?;
        let field = Field { p };
        let a = field.signed(integer(&raw.curve.a, "curve a")?);
        let b = field.signed(integer(&raw.curve.b, "curve b")?);
        require(
            raw.factor_base_abscissas.len() == 3,
            "factor base must have three abscissas",
        )?;
        let mut base = raw
            .factor_base_abscissas
            .iter()
            .map(|value| integer(value, "base abscissa").map(|v| field.signed(v)))
            .collect::<Result<Vec<_>>>()?;
        base.sort();
        require(
            base.windows(2).all(|pair| pair[0] != pair[1]),
            "factor-base abscissas must be distinct",
        )?;
        require(
            raw.target.len() == 2,
            "target must contain affine x,y coordinates",
        )?;
        let r = field.signed(integer(&raw.target[0], "target x")?);
        let target_y = field.signed(integer(&raw.target[1], "target y")?);
        require(
            !base.contains(&r),
            "target abscissa lies in the factor base, so this guarded model is satisfiable",
        )?;
        let rhs = ((&r * &r % &field.p) * &r + &a * &r + &b) % &field.p;
        require(
            &target_y * &target_y % &field.p == rhs,
            "target is not on the declared short Weierstrass curve",
        )?;
        let discriminant = (BigUint::from(4u8) * a.modpow(&BigUint::from(3u8), &field.p)
            + BigUint::from(27u8) * &b * &b)
            % &field.p;
        require(
            !discriminant.is_zero(),
            "declared short Weierstrass curve is singular",
        )?;
        let mut polynomials = Vec::new();
        for input in raw.polynomials {
            require(input.len() <= MAX_INPUT_TERMS, "too many input terms")?;
            let mut polynomial = Polynomial::new();
            for (coefficient, exponents) in input {
                require(
                    exponents.len() == 5,
                    "input exponent vector must have length five",
                )?;
                require(
                    exponents.iter().all(|e| *e <= CAP) && exponents.iter().sum::<u32>() <= CAP,
                    "input polynomial exceeds the construction degree limit",
                )?;
                let coefficient = field.signed(integer(&coefficient, "input coefficient")?);
                field.insert(&mut polynomial, exponents, coefficient);
            }
            polynomials.push(polynomial);
        }
        let variables: Vec<Polynomial> = (0..5).map(|axis| field.variable(axis, 5)).collect();
        let mut expected = Vec::new();
        for variable in &variables[..3] {
            let mut membership = field.constant_small(1, 5);
            for root in &base {
                membership = field.mul(
                    &membership,
                    &field.add(variable, &field.constant(field.neg(root), 5)),
                );
            }
            expected.push(membership);
        }
        expected.push(summation(
            &field,
            &a,
            &b,
            &variables[0],
            &variables[1],
            &variables[3],
        ));
        expected.push(summation(
            &field,
            &a,
            &b,
            &variables[3],
            &variables[2],
            &field.constant(r.clone(), 5),
        ));
        expected.push(field.add(
            &field.mul(
                &variables[4],
                &field.add_scaled(&variables[0], &variables[1], &field.small(-1)),
            ),
            &field.constant_small(-1, 5),
        ));
        require(
            polynomials == expected,
            "input polynomials do not match the declared inverse-guarded curve presentation",
        )?;
        Ok(Self {
            field,
            polynomials,
            base,
            a,
            b,
        })
    }
    fn digest(&self) -> Result<String> {
        let inputs: Vec<Vec<(Decimal, &Vec<u32>)>> = self
            .polynomials
            .iter()
            .map(|poly| poly.iter().map(|(e, c)| (Decimal(c.clone()), e)).collect())
            .collect();
        let canonical = format!(
            "{{\"n\":5,\"p\":{},\"polynomials\":{}}}",
            self.field.p,
            serde_json::to_string(&inputs).map_err(|e| fail(e.to_string()))?
        );
        Ok(hex::encode(Sha256::digest(canonical.as_bytes())))
    }
}

#[derive(Clone, Serialize)]
#[serde(tag = "op")]
enum Node {
    #[serde(rename = "input")]
    Input { index: usize },
    #[serde(rename = "scale")]
    Scale { parent: usize, coefficient: Decimal },
    #[serde(rename = "add")]
    Add {
        left: usize,
        right: usize,
        coefficient: Decimal,
    },
    #[serde(rename = "multiply")]
    Multiply { parent: usize, variable: usize },
}
impl Node {
    fn parents(&self) -> Vec<usize> {
        match self {
            Self::Input { .. } => Vec::new(),
            Self::Scale { parent, .. } | Self::Multiply { parent, .. } => vec![*parent],
            Self::Add { left, right, .. } => vec![*left, *right],
        }
    }
    fn remap(&self, mapping: &BTreeMap<usize, usize>) -> Self {
        match self {
            Self::Input { index } => Self::Input { index: *index },
            Self::Scale {
                parent,
                coefficient,
            } => Self::Scale {
                parent: mapping[parent],
                coefficient: coefficient.clone(),
            },
            Self::Add {
                left,
                right,
                coefficient,
            } => Self::Add {
                left: mapping[left],
                right: mapping[right],
                coefficient: coefficient.clone(),
            },
            Self::Multiply { parent, variable } => Self::Multiply {
                parent: mapping[parent],
                variable: *variable,
            },
        }
    }
}
#[derive(Serialize)]
struct Certificate {
    schema: &'static str,
    degree: u32,
    problem_sha256: String,
    nodes: Vec<Node>,
    goal: usize,
}
struct Dag {
    field: Field,
    nodes: Vec<Node>,
    values: Vec<Polynomial>,
    inputs: Vec<usize>,
    zero: usize,
    maximum_degree: u32,
}
impl Dag {
    fn new(problem: &Problem) -> Result<Self> {
        let mut output = Self {
            field: problem.field.clone(),
            nodes: Vec::new(),
            values: Vec::new(),
            inputs: Vec::new(),
            zero: 0,
            maximum_degree: 0,
        };
        for (index, input) in problem.polynomials.iter().enumerate() {
            let row = output.emit(Node::Input { index }, input.clone())?;
            output.inputs.push(row);
        }
        output.zero = output.scaled(output.inputs[0], &BigUint::zero())?;
        Ok(output)
    }
    fn emit(&mut self, node: Node, poly: Polynomial) -> Result<usize> {
        require(
            degree(&poly) <= CAP,
            "constructive step exceeds degree five",
        )?;
        require(
            self.nodes.len() < MAX_NODES,
            "constructive DAG exceeds its node limit",
        )?;
        self.maximum_degree = self.maximum_degree.max(degree(&poly));
        self.nodes.push(node);
        self.values.push(poly);
        Ok(self.nodes.len() - 1)
    }
    fn scaled(&mut self, row: usize, coefficient: &BigUint) -> Result<usize> {
        let coefficient = coefficient % &self.field.p;
        if coefficient.is_one() {
            return Ok(row);
        }
        let value = self.field.scale(&self.values[row], &coefficient);
        self.emit(
            Node::Scale {
                parent: row,
                coefficient: Decimal(coefficient),
            },
            value,
        )
    }
    fn plus(&mut self, left: usize, right: usize, coefficient: &BigUint) -> Result<usize> {
        let coefficient = coefficient % &self.field.p;
        if coefficient.is_zero() {
            return Ok(left);
        }
        let value = self
            .field
            .add_scaled(&self.values[left], &self.values[right], &coefficient);
        self.emit(
            Node::Add {
                left,
                right,
                coefficient: Decimal(coefficient),
            },
            value,
        )
    }
    fn times_variable(&mut self, row: usize, variable: usize) -> Result<usize> {
        let value = self
            .field
            .mul(&self.values[row], &self.field.variable(variable, 5));
        self.emit(
            Node::Multiply {
                parent: row,
                variable,
            },
            value,
        )
    }
    fn times_polynomial(&mut self, row: usize, factor: &Polynomial) -> Result<usize> {
        let mut result = self.zero;
        for (powers, coefficient) in factor {
            let mut term = row;
            for (axis, count) in powers.iter().enumerate() {
                for _ in 0..*count {
                    term = self.times_variable(term, axis)?;
                }
            }
            result = self.plus(result, term, coefficient)?;
        }
        Ok(result)
    }
    fn certificate(&self, goal: usize, digest: String) -> Result<Certificate> {
        require(
            self.values[goal] == self.field.constant_small(1, 5),
            "constructive goal is not the unit polynomial",
        )?;
        let mut keep = BTreeSet::new();
        let mut pending = vec![goal];
        while let Some(row) = pending.pop() {
            if keep.insert(row) {
                pending.extend(self.nodes[row].parents());
            }
        }
        let mapping: BTreeMap<usize, usize> = keep
            .iter()
            .enumerate()
            .map(|(new, old)| (*old, new))
            .collect();
        let nodes = keep
            .iter()
            .map(|row| self.nodes[*row].remap(&mapping))
            .collect();
        Ok(Certificate {
            schema: "bounded-degree-refutation-v1",
            degree: CAP,
            problem_sha256: digest,
            nodes,
            goal: mapping[&goal],
        })
    }
}

#[derive(Clone)]
struct Item {
    polynomial: Polynomial,
    row: usize,
}
struct Construction {
    problem: Problem,
    trace: Dag,
    missing_root_powers: Vec<Polynomial>,
    grids: Vec<Item>,
    divided_difference: usize,
}
impl Construction {
    fn new(problem: Problem) -> Result<Self> {
        let field = &problem.field;
        let sum = problem
            .base
            .iter()
            .fold(BigUint::zero(), |sum, x| (sum + x) % &field.p);
        let mut missing_root = field.constant(sum, 5);
        missing_root = field.add_scaled(&missing_root, &field.variable(0, 5), &field.small(-1));
        missing_root = field.add_scaled(&missing_root, &field.variable(1, 5), &field.small(-1));
        let mut powers = vec![field.constant_small(1, 5)];
        for _ in 0..8 {
            powers.push(field.mul(powers.last().expect("initial power"), &missing_root));
        }
        let trace = Dag::new(&problem)?;
        Ok(Self {
            problem,
            trace,
            missing_root_powers: powers,
            grids: Vec::new(),
            divided_difference: 0,
        })
    }
    fn mapped(&self, formal: &Polynomial) -> Result<Polynomial> {
        let field = &self.problem.field;
        let mut output = Polynomial::new();
        for (exponents, coefficient) in formal {
            require(
                exponents.len() == 3,
                "formal polynomial must have three coordinates",
            )?;
            let power = self
                .missing_root_powers
                .get(exponents[0] as usize)
                .ok_or_else(|| fail("formal power exceeds the construction limit"))?;
            for (powers, value) in power {
                let mut lifted = powers.clone();
                lifted[2] += exponents[1];
                lifted[3] += exponents[2];
                field.insert(&mut output, lifted, coefficient * value);
            }
        }
        Ok(output)
    }
    fn pair(&self, polynomial: Polynomial, row: usize) -> Result<Item> {
        require(
            self.trace.values[row] == self.mapped(&polynomial)?,
            "formal and five-variable DAG rows disagree",
        )?;
        Ok(Item { polynomial, row })
    }
    fn linear(&mut self, terms: &[(BigUint, Item)]) -> Result<Item> {
        let mut formal = Polynomial::new();
        let mut row = self.trace.zero;
        for (coefficient, item) in terms {
            formal = self
                .problem
                .field
                .add_scaled(&formal, &item.polynomial, coefficient);
            row = self.trace.plus(row, item.row, coefficient)?;
        }
        self.pair(formal, row)
    }
    fn times(&mut self, item: &Item, factor: &Polynomial) -> Result<Item> {
        let formal = self.problem.field.mul(&item.polynomial, factor);
        let mapped = self.mapped(factor)?;
        let row = self.trace.times_polynomial(item.row, &mapped)?;
        self.pair(formal, row)
    }
    fn reduce(&mut self, item: &Item, axes: &[usize]) -> Result<Item> {
        let divisors: Vec<Polynomial> = axes
            .iter()
            .map(|axis| self.grids[*axis].polynomial.clone())
            .collect();
        let (quotients, remainder) = divide(&self.problem.field, &item.polynomial, &divisors)?;
        let mut row = item.row;
        let minus = self.problem.field.small(-1);
        for (quotient, axis) in quotients.iter().zip(axes) {
            let mapped = self.mapped(quotient)?;
            let multiple = self
                .trace
                .times_polynomial(self.grids[*axis].row, &mapped)?;
            row = self.trace.plus(row, multiple, &minus)?;
        }
        self.pair(remainder, row)
    }
    fn derive_from_pair_ideal(
        &mut self,
        formal: &Polynomial,
        start: Option<usize>,
    ) -> Result<usize> {
        let field = &self.problem.field;
        let target = self.mapped(formal)?;
        let input_y = self.trace.inputs[1];
        let difference = start.map_or_else(
            || target.clone(),
            |row| field.add_scaled(&self.trace.values[row], &target, &field.small(-1)),
        );
        let (quotients, remainder) = divide(
            field,
            &difference,
            &[
                self.trace.values[self.divided_difference].clone(),
                self.trace.values[input_y].clone(),
            ],
        )?;
        require(
            remainder.is_empty(),
            "pair-ideal reduction has a nonzero remainder",
        )?;
        let mut row = start.unwrap_or(self.trace.zero);
        let coefficient = self
            .problem
            .field
            .small(if start.is_some() { -1 } else { 1 });
        for (quotient, source) in quotients.iter().zip([self.divided_difference, input_y]) {
            let multiple = self.trace.times_polynomial(source, quotient)?;
            row = self.trace.plus(row, multiple, &coefficient)?;
        }
        require(
            self.trace.values[row] == target,
            "pair-ideal derivation does not match its target",
        )?;
        Ok(row)
    }
    fn initialize(&mut self) -> Result<(Item, Item)> {
        let field = self.problem.field.clone();
        let x = field.variable(0, 5);
        let y = field.variable(1, 5);
        let sum = self
            .problem
            .base
            .iter()
            .fold(BigUint::zero(), |s, x| (s + x) % &field.p);
        let mut pair_sum = BigUint::zero();
        for i in 0..3 {
            for j in i + 1..3 {
                pair_sum = (pair_sum + &self.problem.base[i] * &self.problem.base[j]) % &field.p;
            }
        }
        let mut quotient = field.add(&field.mul(&x, &x), &field.mul(&x, &y));
        quotient = field.add(&quotient, &field.mul(&y, &y));
        quotient = field.add_scaled(&quotient, &field.add(&x, &y), &field.neg(&sum));
        quotient = field.add(&quotient, &field.constant(pair_sum, 5));
        let difference =
            self.trace
                .plus(self.trace.inputs[0], self.trace.inputs[1], &field.small(-1))?;
        let first = self.trace.times_variable(difference, 4)?;
        let second = self
            .trace
            .times_polynomial(self.trace.inputs[5], &quotient)?;
        self.divided_difference = self.trace.plus(first, second, &field.small(-1))?;
        require(
            self.trace.values[self.divided_difference] == quotient,
            "guard divided-difference identity failed",
        )?;
        let mut formal_grids = Vec::new();
        for axis in 0..2 {
            let mut grid = field.constant_small(1, 3);
            for root in &self.problem.base {
                grid = field.mul(
                    &grid,
                    &field.add(
                        &field.variable(axis, 3),
                        &field.constant(field.neg(root), 3),
                    ),
                );
            }
            formal_grids.push(grid);
        }
        let first_grid = self.derive_from_pair_ideal(&formal_grids[0], None)?;
        self.grids
            .push(self.pair(formal_grids[0].clone(), first_grid)?);
        self.grids
            .push(self.pair(formal_grids[1].clone(), self.trace.inputs[2])?);
        let mut q1 = Polynomial::new();
        for missing in &self.problem.base {
            let rest: Vec<&BigUint> = self
                .problem
                .base
                .iter()
                .filter(|root| *root != missing)
                .collect();
            let (a, b) = (rest[0], rest[1]);
            let product = a * b % &field.p;
            let sum = (a + b) % &field.p;
            let tail = field.sub(&product, &self.problem.a);
            let constant = field.sub(
                &(&tail * &tail % &field.p),
                &(BigUint::from(4u8) * &self.problem.b * &sum % &field.p),
            );
            let middle = field.neg(
                &(BigUint::from(2u8)
                    * (&sum * ((&product + &self.problem.a) % &field.p)
                        + BigUint::from(2u8) * &self.problem.b)
                    % &field.p),
            );
            let difference = field.sub(a, b);
            let leading = &difference * &difference % &field.p;
            let mut quadratic = Polynomial::new();
            for (power, coefficient) in [constant, middle, leading].into_iter().enumerate() {
                field.insert(&mut quadratic, vec![0, 0, power as u32], coefficient);
            }
            q1 = field.add(
                &q1,
                &field.mul(&field.lagrange(&self.problem.base, missing, 0)?, &quadratic),
            );
        }
        let q1_row = self.derive_from_pair_ideal(&q1, Some(self.trace.inputs[3]))?;
        let first_item = self.pair(q1, q1_row)?;
        let mut q2 = Polynomial::new();
        for (powers, coefficient) in &self.trace.values[self.trace.inputs[4]] {
            require(
                powers[0] == 0 && powers[1] == 0 && powers[4] == 0,
                "second summation equation uses unexpected variables",
            )?;
            q2.insert(vec![0, powers[2], powers[3]], coefficient.clone());
        }
        let second_item = self.pair(q2, self.trace.inputs[4])?;
        Ok((first_item, second_item))
    }
    fn coefficient(&self, item: &Item, power: u32) -> Polynomial {
        item.polynomial
            .iter()
            .filter(|(powers, _)| powers[2] == power)
            .map(|(powers, coefficient)| (vec![powers[0], powers[1], 0], coefficient.clone()))
            .collect()
    }
    fn normalize(&mut self, item: &Item, axis: usize) -> Result<Item> {
        let field = self.problem.field.clone();
        let leading = self.coefficient(item, 2);
        let mut inverse = Polynomial::new();
        for root in &self.problem.base {
            let mut point = vec![BigUint::zero(); 3];
            point[axis] = root.clone();
            let value = field.evaluate(&leading, &point);
            let projector = field.lagrange(&self.problem.base, root, axis)?;
            inverse = field.add(&inverse, &field.scale(&projector, &field.inv(&value)?));
        }
        let variable = field.variable(axis, 3);
        let mut rows = vec![item.clone()];
        for _ in 0..2 {
            let multiple = self.times(rows.last().expect("initial quadratic"), &variable)?;
            rows.push(self.reduce(&multiple, &[axis])?);
        }
        let terms: Vec<(BigUint, Item)> = inverse
            .iter()
            .map(|(e, c)| (c.clone(), rows[e[axis] as usize].clone()))
            .collect();
        let combined = self.linear(&terms)?;
        let result = self.reduce(&combined, &[axis])?;
        require(
            result
                .polynomial
                .get(&vec![0, 0, 2])
                .is_some_and(BigUint::is_one)
                && result
                    .polynomial
                    .keys()
                    .all(|e| e[2] < 2 || e == &[0, 0, 2])
                && degree(&result.polynomial) <= 3,
            "staged normalization failed to give a monic quadratic",
        )?;
        Ok(result)
    }
    fn run(&mut self) -> Result<(Certificate, Value)> {
        let field = self.problem.field.clone();
        let one = BigUint::one();
        let minus = field.small(-1);
        let (q1, q2) = self.initialize()?;
        let h1 = self.normalize(&q1, 0)?;
        let h2 = self.normalize(&q2, 1)?;
        let a = self.coefficient(&h1, 1);
        let c = self.coefficient(&h2, 1);
        let e = field.add_scaled(&a, &c, &minus);
        let l = self.linear(&[(one.clone(), h1.clone()), (minus.clone(), h2.clone())])?;
        let hsum = self.linear(&[(one.clone(), h1.clone()), (one.clone(), h2.clone())])?;
        let u = field.variable(2, 3);
        let ul = self.times(&l, &u)?;
        let ehsum = self.times(&hsum, &e)?;
        let combination = self.linear(&[(field.small(2), ul), (minus.clone(), ehsum)])?;
        let n = self.reduce(&combination, &[0, 1])?;
        let f = self.coefficient(&n, 1);
        require(
            f.keys().all(|powers| powers[0] == 0 || powers[1] == 0)
                && degree(&f) <= 2
                && degree(&n.polynomial) <= 4,
            "linear remainder coefficient is not separable quadratic",
        )?;
        let mut nrows = BTreeMap::from([((0u32, 0u32), n.clone())]);
        for axis in 0..2 {
            let mut current = n.clone();
            for power in 1..=2 {
                let multiple = self.times(&current, &field.variable(axis, 3))?;
                current = self.reduce(&multiple, &[0, 1])?;
                if power == 1 {
                    require(
                        degree(&current.polynomial) <= 4,
                        "first staged remainder product did not reset to degree four",
                    )?;
                }
                nrows.insert(
                    if axis == 0 { (power, 0) } else { (0, power) },
                    current.clone(),
                );
            }
        }
        let terms = e
            .iter()
            .map(|(powers, coefficient)| {
                nrows
                    .get(&(powers[0], powers[1]))
                    .cloned()
                    .map(|row| (coefficient.clone(), row))
                    .ok_or_else(|| fail("nonseparable multiplier in staged resultant"))
            })
            .collect::<Result<Vec<_>>>()?;
        let en = self.linear(&terms)?;
        let fl_raw = self.times(&l, &f)?;
        let fl = self.reduce(&fl_raw, &[0, 1])?;
        let scalar = self.linear(&[(one.clone(), en.clone()), (minus, fl.clone())])?;
        let inverse_two = field.inv(&field.small(2))?;
        let resultant = self.linear(&[(field.neg(&inverse_two), scalar)])?;
        require(
            degree(&resultant.polynomial) <= 4 && resultant.polynomial.keys().all(|e| e[2] == 0),
            "resultant cancellation did not produce a scalar grid polynomial",
        )?;
        let mut inverse = Polynomial::new();
        for w in &self.problem.base {
            for z in &self.problem.base {
                let value = field.evaluate(
                    &resultant.polynomial,
                    &[w.clone(), z.clone(), BigUint::zero()],
                );
                require(
                    !value.is_zero(),
                    "resultant vanishes on a grid fibre; no constructive refutation is available",
                )?;
                let projector = field.mul(
                    &field.lagrange(&self.problem.base, w, 0)?,
                    &field.lagrange(&self.problem.base, z, 1)?,
                );
                inverse = field.add(&inverse, &field.scale(&projector, &field.inv(&value)?));
            }
        }
        let mut rows = BTreeMap::from([((0u32, 0u32), resultant.clone())]);
        for i in 0..3 {
            if i > 0 {
                let previous = rows[&(i - 1, 0)].clone();
                let multiple = self.times(&previous, &field.variable(0, 3))?;
                rows.insert((i, 0), self.reduce(&multiple, &[0, 1])?);
            }
            for j in 1..3 {
                let previous = rows[&(i, j - 1)].clone();
                let multiple = self.times(&previous, &field.variable(1, 3))?;
                rows.insert((i, j), self.reduce(&multiple, &[0, 1])?);
            }
        }
        let terms: Vec<(BigUint, Item)> = inverse
            .iter()
            .map(|(powers, coefficient)| {
                (coefficient.clone(), rows[&(powers[0], powers[1])].clone())
            })
            .collect();
        let goal = self.linear(&terms)?;
        require(
            goal.polynomial == field.constant_small(1, 3),
            "grid inversion did not give one",
        )?;
        let certificate = self.trace.certificate(goal.row, self.problem.digest()?)?;
        let report = json!({
            "algorithm": "native constructive quadratic-fibre degree-five proof",
            "macaulay_or_rref_search": false,
            "arithmetic": "arbitrary-precision modular integers",
            "modulus_bits": field.p.bits(),
            "maximum_node_degree": self.trace.maximum_degree,
            "certificate_nodes": certificate.nodes.len(),
            "formal_stage_degrees": {
                "H1": degree(&h1.polynomial), "H2": degree(&h2.polynomial),
                "L": degree(&l.polynomial), "N": degree(&n.polynomial),
                "EN": degree(&en.polynomial), "FL": degree(&fl.polynomial),
                "resultant": degree(&resultant.polynomial), "goal": degree(&goal.polynomial)
            },
            "resultant_grid_values_nonzero": true,
            "independent_replay_required": true,
            "scope": "constructed ideal identity; no primality, point-lift, subgroup-order, full degree lower-bound or performance certification"
        });
        Ok((certificate, report))
    }
}

/// Produce an exact numeric-JSON DAG and its algebraic construction receipt.
///
/// The caller must pass the DAG to an independent verifier. Large coefficients
/// remain integer JSON tokens; strings, floats and disguised number objects
/// are never accepted as input integers.
pub fn construct_json(problem_json: &str) -> Result<(String, Value)> {
    require(
        problem_json.len() <= MAX_DOCUMENT_BYTES,
        "problem exceeds the document byte limit",
    )?;
    let problem = Problem::parse(problem_json)?;
    let mut construction = Construction::new(problem)?;
    let (certificate, report) = construction.run()?;
    let json = serde_json::to_string_pretty(&certificate).map_err(|e| fail(e.to_string()))?;
    Ok((json, report))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> &'static str {
        include_str!("../tests/fixtures/p101_problem.json")
    }

    #[test]
    fn historical_prime_101_constructive_dag_passes_independent_u64_replay() {
        let (certificate, report) = construct_json(fixture()).unwrap();
        let large_backend =
            fall_degree_replay::bigprime::verify_json(fixture(), &certificate).unwrap();
        assert_eq!(large_backend["valid_refutation"], true);
        let expected: Value = serde_json::from_str(fixture()).unwrap();
        let certificate: Value = serde_json::from_str(&certificate).unwrap();
        let verified = fall_degree_replay::verify(&expected, &certificate).unwrap();
        assert_eq!(verified["valid_refutation"], true);
        assert_eq!(verified["maximum_intermediate_degree"], 5);
        assert_eq!(report["macaulay_or_rref_search"], false);
        assert_eq!(report["formal_stage_degrees"]["goal"], 0);
    }

    #[test]
    fn characteristic_five_edge_constructive_dag_passes_independent_u64_replay() {
        let input = include_str!("../tests/fixtures/p5_problem.json");
        let (certificate, _) = construct_json(input).unwrap();
        let verified = fall_degree_replay::verify(
            &serde_json::from_str(input).unwrap(),
            &serde_json::from_str(&certificate).unwrap(),
        )
        .unwrap();
        assert_eq!(verified["valid_refutation"], true);
    }

    #[test]
    fn rejects_disguised_integer_tokens_and_wrong_declared_curve() {
        let mut input: Value = serde_json::from_str(fixture()).unwrap();
        input["p"] = json!({"$serde_json::private::Number": "101"});
        assert!(construct_json(&serde_json::to_string(&input).unwrap()).is_err());
        let mut input: Value = serde_json::from_str(fixture()).unwrap();
        input["curve"]["a"] = json!(1);
        assert!(construct_json(&serde_json::to_string(&input).unwrap()).is_err());
    }

    #[test]
    fn rejects_decimal_and_string_moduli() {
        for modulus in ["101.0", "\"101\"", "1e2", "-101"] {
            let input = fixture().replacen("\"p\": 101", &format!("\"p\": {modulus}"), 1);
            assert!(construct_json(&input).is_err(), "{modulus}");
        }
    }

    #[test]
    fn large_coefficients_serialize_as_exact_integer_tokens() {
        let value = (BigUint::one() << 521usize) - BigUint::one();
        let json = serde_json::to_string(&Decimal(value.clone())).unwrap();
        assert_eq!(json, value.to_string());
        let raw: Box<RawValue> = serde_json::from_str(&json).unwrap();
        assert_eq!(integer(&raw, "coefficient").unwrap(), BigInt::from(value));
        for invalid in [
            "-0",
            "0.0",
            "\"0\"",
            "{\"$serde_json::private::Number\":\"0\"}",
        ] {
            let raw: Box<RawValue> = serde_json::from_str(invalid).unwrap();
            assert!(integer(&raw, "coefficient").is_err());
        }
    }
}
