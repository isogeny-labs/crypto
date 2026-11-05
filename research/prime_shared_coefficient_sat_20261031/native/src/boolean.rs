//! Deterministic Boolean lowerings for the canonical prime-field IR.
//!
//! The ordinary CNF and CNF-XOR exporters share one functional circuit.  Every
//! auxiliary bit is therefore uniquely determined by the semantic field bits.

use crate::{CanonicalIr, Term};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

pub const BOOLEAN_SCHEMA: &str = "prime-shared-coefficient.boolean/v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Lit {
    Const(bool),
    Var { id: u32, negated: bool },
}

impl Lit {
    const fn positive(id: u32) -> Self {
        Self::Var { id, negated: false }
    }

    const fn not(self) -> Self {
        match self {
            Self::Const(value) => Self::Const(!value),
            Self::Var { id, negated } => Self::Var {
                id,
                negated: !negated,
            },
        }
    }

    const fn value(self, assignment: &[bool]) -> bool {
        match self {
            Self::Const(value) => value,
            Self::Var { id, negated } => assignment[id as usize] ^ negated,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GateKind {
    And(Lit, Lit),
    Xor(Lit, Lit),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Gate {
    output: u32,
    kind: GateKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldBits {
    pub ir_id: u32,
    pub name: String,
    pub bits: Vec<u32>,
    pub projected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BooleanEncoding {
    pub schema: String,
    pub ir_sha256: String,
    pub modulus: u64,
    pub bit_width: usize,
    pub field_variables: Vec<FieldBits>,
    pub max_boolean_variable: u32,
    gates: Vec<Gate>,
    assertions: Vec<Lit>,
}

#[derive(Default)]
struct CircuitBuilder {
    next_id: u32,
    gates: Vec<Gate>,
    assertions: Vec<Lit>,
}

type Word = Vec<Lit>;
type Clause = Vec<i64>;
type XorConstraint = (Vec<u32>, bool);
type Constraints = (Vec<Clause>, Vec<XorConstraint>);

impl CircuitBuilder {
    fn input(&mut self) -> Result<Lit, String> {
        let id = self.fresh_id()?;
        Ok(Lit::positive(id))
    }

    fn fresh_id(&mut self) -> Result<u32, String> {
        let id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| "Boolean variable ID overflow".to_owned())?;
        self.next_id = id;
        Ok(id)
    }

    fn gate(&mut self, kind: GateKind) -> Result<Lit, String> {
        let output = self.fresh_id()?;
        self.gates.push(Gate { output, kind });
        Ok(Lit::positive(output))
    }

    fn and(&mut self, left: Lit, right: Lit) -> Result<Lit, String> {
        match (left, right) {
            (Lit::Const(false), _) | (_, Lit::Const(false)) => Ok(Lit::Const(false)),
            (Lit::Const(true), value) | (value, Lit::Const(true)) => Ok(value),
            (left, right) if left == right => Ok(left),
            (left, right) if left == right.not() => Ok(Lit::Const(false)),
            (left, right) => self.gate(GateKind::And(left, right)),
        }
    }

    fn xor(&mut self, left: Lit, right: Lit) -> Result<Lit, String> {
        match (left, right) {
            (Lit::Const(false), value) | (value, Lit::Const(false)) => Ok(value),
            (Lit::Const(true), value) | (value, Lit::Const(true)) => Ok(value.not()),
            (left, right) if left == right => Ok(Lit::Const(false)),
            (left, right) if left == right.not() => Ok(Lit::Const(true)),
            (left, right) => self.gate(GateKind::Xor(left, right)),
        }
    }

    fn or(&mut self, left: Lit, right: Lit) -> Result<Lit, String> {
        Ok(self.and(left.not(), right.not())?.not())
    }

    fn mux(&mut self, select: Lit, when_true: Lit, when_false: Lit) -> Result<Lit, String> {
        if when_true == when_false {
            return Ok(when_true);
        }
        let selected_true = self.and(select, when_true)?;
        let selected_false = self.and(select.not(), when_false)?;
        self.xor(selected_true, selected_false)
    }

    fn assert_true(&mut self, value: Lit) {
        if value != Lit::Const(true) {
            self.assertions.push(value);
        }
    }

    fn word_inputs(&mut self, width: usize) -> Result<Word, String> {
        (0..width).map(|_| self.input()).collect()
    }

    fn add_words(&mut self, left: &[Lit], right: &[Lit]) -> Result<Word, String> {
        let width = left.len().max(right.len());
        let mut result = Vec::with_capacity(width + 1);
        let mut carry = Lit::Const(false);
        for index in 0..width {
            let left_bit = left.get(index).copied().unwrap_or(Lit::Const(false));
            let right_bit = right.get(index).copied().unwrap_or(Lit::Const(false));
            let pair_xor = self.xor(left_bit, right_bit)?;
            result.push(self.xor(pair_xor, carry)?);
            let pair_carry = self.and(left_bit, right_bit)?;
            let carry_through = self.and(pair_xor, carry)?;
            carry = self.or(pair_carry, carry_through)?;
        }
        result.push(carry);
        Ok(result)
    }

    fn subtract_constant(&mut self, value: &[Lit], constant: u64) -> Result<(Word, Lit), String> {
        let mut difference = Vec::with_capacity(value.len());
        let mut borrow = Lit::Const(false);
        for (index, input) in value.iter().copied().enumerate() {
            let constant_bit = if index < u64::BITS as usize {
                (constant >> index) & 1 == 1
            } else {
                false
            };
            let with_constant = self.xor(input, Lit::Const(constant_bit))?;
            difference.push(self.xor(with_constant, borrow)?);
            borrow = if constant_bit {
                self.or(input.not(), borrow)?
            } else {
                self.and(input.not(), borrow)?
            };
        }
        Ok((difference, borrow))
    }

    fn assert_canonical_field_word(&mut self, word: &[Lit], modulus: u64) -> Result<(), String> {
        let (_, borrow) = self.subtract_constant(word, modulus)?;
        self.assert_true(borrow);
        Ok(())
    }

    fn reduce_bits(&mut self, bits: &[Lit], modulus: u64, width: usize) -> Result<Word, String> {
        let mut remainder = vec![Lit::Const(false); width];
        for bit in bits.iter().rev().copied() {
            let mut doubled = Vec::with_capacity(width + 1);
            doubled.push(bit);
            doubled.extend(remainder.iter().copied());
            let (difference, borrow) = self.subtract_constant(&doubled, modulus)?;
            let subtract = borrow.not();
            remainder = (0..width)
                .map(|index| self.mux(subtract, difference[index], doubled[index]))
                .collect::<Result<Vec<_>, _>>()?;
        }
        Ok(remainder)
    }

    fn field_add(
        &mut self,
        left: &[Lit],
        right: &[Lit],
        modulus: u64,
        width: usize,
    ) -> Result<Word, String> {
        let sum = self.add_words(left, right)?;
        self.reduce_bits(&sum, modulus, width)
    }

    fn field_mul(
        &mut self,
        left: &[Lit],
        right: &[Lit],
        modulus: u64,
        width: usize,
    ) -> Result<Word, String> {
        let product_width = width
            .checked_mul(2)
            .ok_or_else(|| "Boolean word width overflow".to_owned())?;
        let mut product = vec![Lit::Const(false); product_width];
        for (right_index, right_bit) in right.iter().copied().enumerate() {
            let mut partial = vec![Lit::Const(false); product_width];
            for (left_index, left_bit) in left.iter().copied().enumerate() {
                partial[left_index + right_index] = self.and(left_bit, right_bit)?;
            }
            let sum = self.add_words(&product, &partial)?;
            product.copy_from_slice(&sum[..product_width]);
        }
        self.reduce_bits(&product, modulus, width)
    }

    fn constant_word(value: u64, width: usize) -> Word {
        (0..width)
            .map(|index| Lit::Const((value >> index) & 1 == 1))
            .collect()
    }
}

impl BooleanEncoding {
    /// Lower one canonical finite-field IR into a deterministic Boolean circuit.
    ///
    /// # Errors
    ///
    /// Returns an error if the source IR is invalid or circuit dimensions overflow.
    pub fn from_ir(ir: &CanonicalIr) -> Result<Self, String> {
        ir.validate()?;
        let modulus = ir
            .spec
            .modulus
            .parse::<u64>()
            .map_err(|_| "invalid canonical modulus".to_owned())?;
        let bit_width = usize::try_from(u64::BITS - modulus.leading_zeros())
            .map_err(|_| "field bit width does not fit usize".to_owned())?;
        let mut builder = CircuitBuilder::default();
        let projected = ir
            .projected_variables
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        let mut field_variables = Vec::with_capacity(ir.variables.len());
        let mut words = Vec::with_capacity(ir.variables.len());
        for variable in &ir.variables {
            let word = builder.word_inputs(bit_width)?;
            builder.assert_canonical_field_word(&word, modulus)?;
            let bits = word
                .iter()
                .map(|bit| match bit {
                    Lit::Var { id, negated: false } => Ok(*id),
                    _ => Err("semantic input bit was simplified unexpectedly".to_owned()),
                })
                .collect::<Result<Vec<_>, _>>()?;
            field_variables.push(FieldBits {
                ir_id: variable.id,
                name: variable.name.clone(),
                bits,
                projected: projected.contains(&variable.id),
            });
            words.push(word);
        }

        for equation in &ir.equations {
            let mut sum = CircuitBuilder::constant_word(0, bit_width);
            for term in &equation.terms {
                let value = lower_term(&mut builder, term, &words, modulus, bit_width)?;
                sum = builder.field_add(&sum, &value, modulus, bit_width)?;
            }
            for bit in sum {
                builder.assert_true(bit.not());
            }
        }

        Ok(Self {
            schema: BOOLEAN_SCHEMA.to_owned(),
            ir_sha256: ir.sha256()?,
            modulus,
            bit_width,
            field_variables,
            max_boolean_variable: builder.next_id,
            gates: builder.gates,
            assertions: builder.assertions,
        })
    }

    /// Emit ordinary DIMACS CNF, expanding every XOR gate into clauses.
    #[must_use]
    pub fn to_cnf(&self) -> String {
        self.render(false, &[])
    }

    /// Emit CryptoMiniSat-compatible DIMACS with native XOR constraints.
    #[must_use]
    pub fn to_cnf_xor(&self) -> String {
        self.render(true, &[])
    }

    /// Emit either representation with additional ordinary CNF clauses.
    ///
    /// This is used for semantic-field assignments and projected-model
    /// blocking.  Native XOR constraints remain unchanged.
    ///
    /// # Errors
    ///
    /// Rejects zero or out-of-range literals and empty additional clauses.
    pub fn with_additional_clauses(
        &self,
        native_xor: bool,
        additional: &[Vec<i64>],
    ) -> Result<String, String> {
        for clause in additional {
            if clause.is_empty() {
                return Err("additional clause must be nonempty".to_owned());
            }
            for literal in clause {
                if *literal == 0 || literal.unsigned_abs() > u64::from(self.max_boolean_variable) {
                    return Err(format!("additional literal out of range: {literal}"));
                }
            }
        }
        Ok(self.render(native_xor, additional))
    }

    /// Encode one complete semantic field assignment as Boolean unit clauses.
    ///
    /// # Errors
    ///
    /// Rejects missing, extra, or noncanonical field values.
    pub fn assignment_unit_clauses(
        &self,
        assignment: &BTreeMap<String, u64>,
    ) -> Result<Vec<Vec<i64>>, String> {
        if assignment.len() != self.field_variables.len() {
            return Err(format!(
                "field assignment has {} entries; expected {}",
                assignment.len(),
                self.field_variables.len()
            ));
        }
        let expected_names = self
            .field_variables
            .iter()
            .map(|field| field.name.as_str())
            .collect::<BTreeSet<_>>();
        if let Some(extra) = assignment
            .keys()
            .find(|name| !expected_names.contains(name.as_str()))
        {
            return Err(format!("unknown field assignment variable: {extra}"));
        }
        let mut clauses = Vec::with_capacity(self.field_variables.len() * self.bit_width);
        for field in &self.field_variables {
            let value = assignment
                .get(&field.name)
                .copied()
                .ok_or_else(|| format!("missing field assignment variable: {}", field.name))?;
            if value >= self.modulus {
                return Err(format!("noncanonical field value for {}", field.name));
            }
            for (index, variable) in field.bits.iter().copied().enumerate() {
                clauses.push(vec![if (value >> index) & 1 == 1 {
                    i64::from(variable)
                } else {
                    -i64::from(variable)
                }]);
            }
        }
        Ok(clauses)
    }

    /// Return deterministic SHA-256 digests of the CNF and CNF-XOR bytes.
    #[must_use]
    pub fn output_sha256(&self) -> (String, String) {
        let cnf = Sha256::digest(self.to_cnf().as_bytes());
        let xor = Sha256::digest(self.to_cnf_xor().as_bytes());
        (format!("{cnf:x}"), format!("{xor:x}"))
    }

    /// Evaluate the functional circuit for one complete field assignment.
    ///
    /// # Errors
    ///
    /// Returns an error for a wrong-length or noncanonical field assignment.
    pub fn semantic_values_satisfy(&self, values: &[u64]) -> Result<bool, String> {
        if values.len() != self.field_variables.len() {
            return Err(format!(
                "field assignment has {} values; expected {}",
                values.len(),
                self.field_variables.len()
            ));
        }
        let assignment_len = usize::try_from(self.max_boolean_variable)
            .map_err(|_| "Boolean variable count does not fit usize".to_owned())?
            .checked_add(1)
            .ok_or_else(|| "Boolean assignment length overflow".to_owned())?;
        let mut assignment = vec![false; assignment_len];
        for (field, value) in self.field_variables.iter().zip(values) {
            if *value >= self.modulus {
                return Err(format!("noncanonical field value for {}", field.name));
            }
            for (index, bit) in field.bits.iter().copied().enumerate() {
                assignment[bit as usize] = (*value >> index) & 1 == 1;
            }
        }
        evaluate_gates(&self.gates, &mut assignment);
        Ok(self
            .assertions
            .iter()
            .all(|literal| literal.value(&assignment)))
    }

    /// Decode the semantic field values from a SAT solver's textual model.
    ///
    /// # Errors
    ///
    /// Rejects missing, contradictory, out-of-range, or non-SAT models.
    pub fn decode_solver_model(&self, output: &str) -> Result<BTreeMap<String, u64>, String> {
        if !output.lines().any(|line| line.trim() == "s SATISFIABLE") {
            return Err("solver output does not contain s SATISFIABLE".to_owned());
        }
        let count = usize::try_from(self.max_boolean_variable)
            .map_err(|_| "Boolean variable count does not fit usize".to_owned())?
            .checked_add(1)
            .ok_or_else(|| "Boolean model length overflow".to_owned())?;
        let mut bits = vec![None; count];
        for line in output
            .lines()
            .map(str::trim)
            .filter(|line| line.starts_with('v'))
        {
            for token in line[1..].split_whitespace() {
                let literal = token
                    .parse::<i64>()
                    .map_err(|_| format!("invalid model literal: {token}"))?;
                if literal == 0 {
                    continue;
                }
                let variable = usize::try_from(literal.unsigned_abs())
                    .map_err(|_| "model variable does not fit usize".to_owned())?;
                if variable == 0 || variable >= bits.len() {
                    return Err(format!("model variable out of range: {variable}"));
                }
                let value = literal > 0;
                if bits[variable].is_some_and(|previous| previous != value) {
                    return Err(format!("contradictory model value for variable {variable}"));
                }
                bits[variable] = Some(value);
            }
        }
        let mut decoded = BTreeMap::new();
        for field in &self.field_variables {
            let mut value = 0_u64;
            for (index, bit) in field.bits.iter().copied().enumerate() {
                let bit_value =
                    bits[bit as usize].ok_or_else(|| format!("model omits semantic bit {bit}"))?;
                if bit_value {
                    value |= 1_u64
                        .checked_shl(u32::try_from(index).map_err(|_| "bit index overflow")?)
                        .ok_or_else(|| "decoded field value overflow".to_owned())?;
                }
            }
            if value >= self.modulus {
                return Err(format!("model gives noncanonical value for {}", field.name));
            }
            decoded.insert(field.name.clone(), value);
        }
        let ordered = self
            .field_variables
            .iter()
            .map(|field| decoded[&field.name])
            .collect::<Vec<_>>();
        if !self.semantic_values_satisfy(&ordered)? {
            return Err("decoded semantic model fails the Boolean circuit".to_owned());
        }
        Ok(decoded)
    }

    /// Project a decoded field assignment in canonical projected-variable order.
    ///
    /// # Errors
    ///
    /// Returns an error when a projected field is absent.
    pub fn projected_values(&self, assignment: &BTreeMap<String, u64>) -> Result<Vec<u64>, String> {
        self.field_variables
            .iter()
            .filter(|field| field.projected)
            .map(|field| {
                assignment
                    .get(&field.name)
                    .copied()
                    .ok_or_else(|| format!("missing projected field {}", field.name))
            })
            .collect()
    }

    /// Return one clause that blocks the projected values in an assignment.
    ///
    /// # Errors
    ///
    /// Returns an error when a projected value is missing or noncanonical.
    pub fn projected_blocking_clause(
        &self,
        assignment: &BTreeMap<String, u64>,
    ) -> Result<Vec<i64>, String> {
        let mut clause = Vec::new();
        for field in self.field_variables.iter().filter(|field| field.projected) {
            let value = assignment
                .get(&field.name)
                .copied()
                .ok_or_else(|| format!("missing projected field {}", field.name))?;
            if value >= self.modulus {
                return Err(format!("noncanonical projected field {}", field.name));
            }
            for (index, variable) in field.bits.iter().copied().enumerate() {
                let positive = (value >> index) & 1 == 0;
                clause.push(if positive {
                    i64::from(variable)
                } else {
                    -i64::from(variable)
                });
            }
        }
        Ok(clause)
    }

    fn render(&self, native_xor: bool, additional: &[Vec<i64>]) -> String {
        let (mut clauses, xors) = self.constraints(native_xor);
        clauses.extend_from_slice(additional);
        let mut output = String::new();
        writeln!(output, "c schema {}", self.schema).expect("String writes cannot fail");
        writeln!(output, "c ir_sha256 {}", self.ir_sha256).expect("String writes cannot fail");
        writeln!(output, "c modulus {}", self.modulus).expect("String writes cannot fail");
        for field in &self.field_variables {
            let bit_list = field
                .bits
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            writeln!(
                output,
                "c field {} {} {} {}",
                field.ir_id,
                field.name,
                bit_list,
                if field.projected {
                    "projected"
                } else {
                    "auxiliary"
                }
            )
            .expect("String writes cannot fail");
        }
        writeln!(
            output,
            "p cnf {} {}",
            self.max_boolean_variable,
            clauses.len() + xors.len()
        )
        .expect("String writes cannot fail");
        for clause in clauses {
            for literal in clause {
                write!(output, "{literal} ").expect("String writes cannot fail");
            }
            output.push_str("0\n");
        }
        for (variables, rhs) in xors {
            output.push_str("x ");
            for (index, variable) in variables.iter().copied().enumerate() {
                let literal = if index == 0 && !rhs {
                    -i64::from(variable)
                } else {
                    i64::from(variable)
                };
                write!(output, "{literal} ").expect("String writes cannot fail");
            }
            output.push_str("0\n");
        }
        output
    }

    fn constraints(&self, native_xor: bool) -> Constraints {
        let mut clauses = Vec::new();
        let mut xors = Vec::new();
        for gate in &self.gates {
            let output = Lit::positive(gate.output);
            match gate.kind {
                GateKind::And(left, right) => {
                    push_clause(&mut clauses, &[left.not(), right.not(), output]);
                    push_clause(&mut clauses, &[left, output.not()]);
                    push_clause(&mut clauses, &[right, output.not()]);
                }
                GateKind::Xor(left, right) if native_xor => {
                    xors.push(normalize_xor(&[output, left, right]));
                }
                GateKind::Xor(left, right) => {
                    for left_value in [false, true] {
                        for right_value in [false, true] {
                            let invalid_output = !(left_value ^ right_value);
                            push_clause(
                                &mut clauses,
                                &[
                                    forbid_value(left, left_value),
                                    forbid_value(right, right_value),
                                    forbid_value(output, invalid_output),
                                ],
                            );
                        }
                    }
                }
            }
        }
        for assertion in &self.assertions {
            push_clause(&mut clauses, &[*assertion]);
        }
        (clauses, xors)
    }
}

fn lower_term(
    builder: &mut CircuitBuilder,
    term: &Term,
    words: &[Word],
    modulus: u64,
    width: usize,
) -> Result<Word, String> {
    let coefficient = term
        .coefficient
        .parse::<u64>()
        .map_err(|_| "invalid term coefficient".to_owned())?;
    let mut value = CircuitBuilder::constant_word(coefficient, width);
    for power in &term.powers {
        let variable = words
            .get(power.variable as usize)
            .ok_or_else(|| "term references an unknown field variable".to_owned())?;
        for _ in 0..power.exponent {
            value = builder.field_mul(&value, variable, modulus, width)?;
        }
    }
    Ok(value)
}

fn evaluate_gates(gates: &[Gate], assignment: &mut [bool]) {
    for gate in gates {
        assignment[gate.output as usize] = match gate.kind {
            GateKind::And(left, right) => left.value(assignment) & right.value(assignment),
            GateKind::Xor(left, right) => left.value(assignment) ^ right.value(assignment),
        };
    }
}

const fn forbid_value(literal: Lit, value: bool) -> Lit {
    if value {
        literal.not()
    } else {
        literal
    }
}

fn push_clause(clauses: &mut Vec<Vec<i64>>, literals: &[Lit]) {
    let mut clause = BTreeSet::new();
    for literal in literals {
        match literal {
            Lit::Const(true) => return,
            Lit::Const(false) => {}
            Lit::Var { id, negated } => {
                let value = if *negated {
                    -i64::from(*id)
                } else {
                    i64::from(*id)
                };
                if clause.contains(&-value) {
                    return;
                }
                clause.insert(value);
            }
        }
    }
    clauses.push(clause.into_iter().collect());
}

fn normalize_xor(literals: &[Lit]) -> (Vec<u32>, bool) {
    let mut variables = BTreeSet::new();
    let mut rhs = false;
    for literal in literals {
        match literal {
            Lit::Const(value) => rhs ^= value,
            Lit::Var { id, negated } => {
                rhs ^= negated;
                if !variables.insert(*id) {
                    variables.remove(id);
                }
            }
        }
    }
    (variables.into_iter().collect(), rhs)
}

/// Strictly validate ordinary or XOR-extended DIMACS syntax.
///
/// # Errors
///
/// Rejects malformed headers, counts, literals, terminators, and XOR lines in
/// an ordinary-CNF document.
pub fn validate_dimacs(input: &str, allow_xor: bool) -> Result<(), String> {
    let mut header = None;
    let mut seen_constraints = 0_usize;
    for (line_index, raw_line) in input.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        if line.starts_with('p') {
            if header.is_some() || seen_constraints != 0 {
                return Err(format!(
                    "misplaced or duplicate header on line {}",
                    line_index + 1
                ));
            }
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() != 4 || fields[0] != "p" || fields[1] != "cnf" {
                return Err(format!(
                    "malformed DIMACS header on line {}",
                    line_index + 1
                ));
            }
            let variables = fields[2]
                .parse::<u32>()
                .map_err(|_| "invalid DIMACS variable count".to_owned())?;
            let constraints = fields[3]
                .parse::<usize>()
                .map_err(|_| "invalid DIMACS constraint count".to_owned())?;
            header = Some((variables, constraints));
            continue;
        }
        let (variables, expected_constraints) =
            header.ok_or_else(|| "constraint appears before DIMACS header".to_owned())?;
        if seen_constraints >= expected_constraints {
            return Err("trailing DIMACS constraint".to_owned());
        }
        let mut tokens = line.split_whitespace();
        let first = tokens.next().ok_or_else(|| "empty constraint".to_owned())?;
        let mut literal_tokens = Vec::new();
        let is_xor = first == "x";
        if is_xor {
            if !allow_xor {
                return Err("XOR line in ordinary CNF".to_owned());
            }
        } else {
            literal_tokens.push(first);
        }
        literal_tokens.extend(tokens);
        if literal_tokens.last().copied() != Some("0") {
            return Err(format!(
                "unterminated constraint on line {}",
                line_index + 1
            ));
        }
        for token in &literal_tokens[..literal_tokens.len() - 1] {
            let literal = token
                .parse::<i64>()
                .map_err(|_| format!("invalid literal on line {}", line_index + 1))?;
            if literal == 0 || literal.unsigned_abs() > u64::from(variables) {
                return Err(format!("literal out of range on line {}", line_index + 1));
            }
        }
        seen_constraints += 1;
    }
    let (_, expected_constraints) = header.ok_or_else(|| "missing DIMACS header".to_owned())?;
    if seen_constraints != expected_constraints {
        return Err(format!(
            "DIMACS constraint count mismatch: saw {seen_constraints}, expected {expected_constraints}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{build_ir, InstanceSpec};

    fn m3_ir() -> CanonicalIr {
        build_ir(InstanceSpec {
            modulus: 17,
            curve_a: 2,
            curve_b: 2,
            arity: 3,
            depth: 4,
            terminal_c: 1,
            target_x: 0,
            target_y: 6,
        })
        .unwrap()
    }

    fn clause_holds(clause: &[i64], assignment: &[bool]) -> bool {
        clause.iter().any(|literal| {
            let variable =
                usize::try_from(literal.unsigned_abs()).expect("test literal index fits in usize");
            assignment[variable] == (*literal > 0)
        })
    }

    fn xor_holds(constraint: &(Vec<u32>, bool), assignment: &[bool]) -> bool {
        constraint.0.iter().fold(false, |parity, variable| {
            parity ^ assignment[*variable as usize]
        }) == constraint.1
    }

    fn check_gate_encoding(kind: GateKind, operation: impl Fn(bool, bool) -> bool) {
        let gate = Gate { output: 3, kind };
        let encoding = BooleanEncoding {
            schema: BOOLEAN_SCHEMA.to_owned(),
            ir_sha256: "test".to_owned(),
            modulus: 17,
            bit_width: 5,
            field_variables: Vec::new(),
            max_boolean_variable: 3,
            gates: vec![gate],
            assertions: Vec::new(),
        };
        for native_xor in [false, true] {
            let (clauses, xors) = encoding.constraints(native_xor);
            for left in [false, true] {
                for right in [false, true] {
                    for output in [false, true] {
                        let assignment = [false, left, right, output];
                        let accepted = clauses
                            .iter()
                            .all(|clause| clause_holds(clause, &assignment))
                            && xors
                                .iter()
                                .all(|constraint| xor_holds(constraint, &assignment));
                        assert_eq!(accepted, output == operation(left, right));
                    }
                }
            }
        }
    }

    #[test]
    fn cnf_and_native_xor_gate_definitions_are_exact() {
        check_gate_encoding(
            GateKind::And(Lit::positive(1), Lit::positive(2)),
            |left, right| left & right,
        );
        check_gate_encoding(
            GateKind::And(Lit::positive(1).not(), Lit::positive(2)),
            |left, right| !left & right,
        );
        check_gate_encoding(
            GateKind::Xor(Lit::positive(1), Lit::positive(2)),
            |left, right| left ^ right,
        );
        check_gate_encoding(
            GateKind::Xor(Lit::positive(1).not(), Lit::positive(2)),
            |left, right| !left ^ right,
        );
    }

    #[test]
    fn canonical_range_constraint_is_exact_over_all_five_bit_words() {
        let mut builder = CircuitBuilder::default();
        let word = builder.word_inputs(5).unwrap();
        let (_, borrow) = builder.subtract_constant(&word, 17).unwrap();
        for value in 0_u64..32 {
            let mut assignment = vec![false; builder.next_id as usize + 1];
            for (index, bit) in word.iter().copied().enumerate() {
                let Lit::Var { id, negated: false } = bit else {
                    panic!("input literal is positive");
                };
                assignment[id as usize] = (value >> index) & 1 == 1;
            }
            evaluate_gates(&builder.gates, &mut assignment);
            assert_eq!(borrow.value(&assignment), value < 17);
        }
    }

    #[test]
    fn field_arithmetic_circuit_is_exhaustive_over_f17() {
        let modulus = 17;
        let width = 5;
        let mut builder = CircuitBuilder::default();
        let left = builder.word_inputs(width).unwrap();
        let right = builder.word_inputs(width).unwrap();
        let sum = builder.field_add(&left, &right, modulus, width).unwrap();
        let product = builder.field_mul(&left, &right, modulus, width).unwrap();
        for left_value in 0..modulus {
            for right_value in 0..modulus {
                let mut values = vec![false; builder.next_id as usize + 1];
                for (index, literal) in left.iter().chain(&right).copied().enumerate() {
                    let Lit::Var { id, negated: false } = literal else {
                        panic!("input literal is positive");
                    };
                    let source = if index < width {
                        left_value
                    } else {
                        right_value
                    };
                    values[id as usize] = (source >> (index % width)) & 1 == 1;
                }
                evaluate_gates(&builder.gates, &mut values);
                let decode = |word: &[Lit]| {
                    word.iter().enumerate().fold(0_u64, |value, (index, bit)| {
                        value | (u64::from(bit.value(&values)) << index)
                    })
                };
                assert_eq!(decode(&sum), (left_value + right_value) % modulus);
                assert_eq!(decode(&product), (left_value * right_value) % modulus);
            }
        }
    }

    #[test]
    fn exporters_are_deterministic_and_strictly_parse() {
        let encoding = BooleanEncoding::from_ir(&m3_ir()).unwrap();
        let cnf = encoding.to_cnf();
        let xnf = encoding.to_cnf_xor();
        assert_eq!(cnf, encoding.to_cnf());
        assert_eq!(xnf, encoding.to_cnf_xor());
        assert_ne!(cnf, xnf);
        validate_dimacs(&cnf, false).unwrap();
        validate_dimacs(&xnf, true).unwrap();
        assert!(validate_dimacs(&xnf, false).is_err());
        assert!(validate_dimacs(&cnf.replacen(" 0\n", "\n", 1), false).is_err());
        assert!(validate_dimacs(&(cnf + "1 0\n"), false).is_err());
    }

    #[test]
    fn known_assignment_satisfies_and_mutation_fails() {
        let ir = m3_ir();
        let encoding = BooleanEncoding::from_ir(&ir).unwrap();
        let witness: serde_json::Value = serde_json::from_str(include_str!(
            "../../../prime_one_shot_20261029/prototype/exports/m3_t4_remainder.witness.json"
        ))
        .unwrap();
        let assignment = witness["assignment"].as_object().unwrap();
        let mut values = ir
            .variables
            .iter()
            .map(|variable| assignment[&variable.name].as_u64().unwrap())
            .collect::<Vec<_>>();
        assert!(encoding.semantic_values_satisfy(&values).unwrap());
        values[0] = (values[0] + 1) % 17;
        assert!(!encoding.semantic_values_satisfy(&values).unwrap());
    }
}
