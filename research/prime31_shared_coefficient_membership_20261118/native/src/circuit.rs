use crate::ir::RelationSpec;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt::Write as _;

pub const ENCODING_SCHEMA: &str = "prime31.selector-circuit/v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

    const fn signed(self) -> Option<i64> {
        match self {
            Self::Const(_) => None,
            Self::Var { id, negated } => Some(if negated { -(id as i64) } else { id as i64 }),
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

type Word = Vec<Lit>;
type Constraints = (Vec<Vec<i64>>, Vec<(Vec<u32>, bool)>);

#[derive(Clone, Debug)]
struct PointBits {
    x: Word,
    y: Word,
    z: Word,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ArithmeticCounts {
    pub field_additions: u64,
    pub field_negations: u64,
    pub field_multiplications: u64,
    pub field_zero_tests: u64,
    pub word_muxes: u64,
    pub mixed_ec_additions: u64,
    pub jacobian_doublings: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ForbiddenCounters {
    pub secret_scalars_consumed: u64,
    pub witness_indices_or_signs_consumed: u64,
    pub pair_table_entries_consumed: u64,
    pub half_table_entries_consumed: u64,
    pub complete_selector_assignments_enumerated: u64,
    pub grader_records_consumed_after_target_extraction: u64,
}

impl ForbiddenCounters {
    pub fn all_zero(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EncodingStatistics {
    pub schema: String,
    pub case_id: String,
    pub relation_sha256: String,
    pub field_bits: usize,
    pub base_size: usize,
    pub semantic_input_bits: usize,
    pub boolean_variables: u32,
    pub and_gates: usize,
    pub xor_gates: usize,
    pub assertions: usize,
    pub ordinary_cnf_clauses: usize,
    pub cnf_xor_ordinary_clauses: usize,
    pub cnf_xor_rows: usize,
    pub arithmetic: ArithmeticCounts,
    pub forbidden: ForbiddenCounters,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SelectorWitness {
    pub indices: [usize; 8],
    /// `true` means the selected public point is negated.
    pub signs: [bool; 8],
}

#[derive(Default)]
struct CircuitBuilder {
    next_id: u32,
    input_ids: Vec<u32>,
    gates: Vec<Gate>,
    assertions: Vec<Lit>,
    arithmetic: ArithmeticCounts,
}

impl CircuitBuilder {
    fn fresh_id(&mut self) -> Result<u32, String> {
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or_else(|| "Boolean variable ID overflow".to_owned())?;
        Ok(self.next_id)
    }

    fn input(&mut self) -> Result<Lit, String> {
        let id = self.fresh_id()?;
        self.input_ids.push(id);
        Ok(Lit::positive(id))
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
        let both_false = self.and(left.not(), right.not())?;
        Ok(both_false.not())
    }

    fn and_many(&mut self, values: impl IntoIterator<Item = Lit>) -> Result<Lit, String> {
        let mut result = Lit::Const(true);
        for value in values {
            result = self.and(result, value)?;
        }
        Ok(result)
    }

    fn mux(&mut self, select: Lit, when_true: Lit, when_false: Lit) -> Result<Lit, String> {
        if when_true == when_false {
            return Ok(when_true);
        }
        let selected_true = self.and(select, when_true)?;
        let selected_false = self.and(select.not(), when_false)?;
        self.xor(selected_true, selected_false)
    }

    fn mux_word(
        &mut self,
        select: Lit,
        when_true: &[Lit],
        when_false: &[Lit],
    ) -> Result<Word, String> {
        if when_true.len() != when_false.len() {
            return Err("word mux width mismatch".into());
        }
        self.arithmetic.word_muxes += 1;
        when_true
            .iter()
            .copied()
            .zip(when_false.iter().copied())
            .map(|(left, right)| self.mux(select, left, right))
            .collect()
    }

    fn mux_point(
        &mut self,
        select: Lit,
        when_true: &PointBits,
        when_false: &PointBits,
    ) -> Result<PointBits, String> {
        Ok(PointBits {
            x: self.mux_word(select, &when_true.x, &when_false.x)?,
            y: self.mux_word(select, &when_true.y, &when_false.y)?,
            z: self.mux_word(select, &when_true.z, &when_false.z)?,
        })
    }

    fn assert_true(&mut self, value: Lit) {
        if value != Lit::Const(true) {
            self.assertions.push(value);
        }
    }

    fn constant_word(value: u64, width: usize) -> Word {
        (0..width)
            .map(|bit| Lit::Const((value >> bit) & 1 != 0))
            .collect()
    }

    fn full_adder(&mut self, a: Lit, b: Lit, carry: Lit) -> Result<(Lit, Lit), String> {
        let pair_xor = self.xor(a, b)?;
        let sum = self.xor(pair_xor, carry)?;
        let pair_carry = self.and(a, b)?;
        let carry_through = self.and(pair_xor, carry)?;
        let carry_out = self.xor(pair_carry, carry_through)?;
        Ok((sum, carry_out))
    }

    fn ripple_add(&mut self, left: &[Lit], right: &[Lit]) -> Result<Word, String> {
        let width = left.len().max(right.len());
        let mut result = Vec::with_capacity(width + 1);
        let mut carry = Lit::Const(false);
        for bit in 0..width {
            let a = left.get(bit).copied().unwrap_or(Lit::Const(false));
            let b = right.get(bit).copied().unwrap_or(Lit::Const(false));
            let (sum, next_carry) = self.full_adder(a, b, carry)?;
            result.push(sum);
            carry = next_carry;
        }
        result.push(carry);
        Ok(result)
    }

    fn increment_if(&mut self, word: &[Lit], increment: Lit) -> Result<Word, String> {
        let mut output = Vec::with_capacity(word.len());
        let mut carry = increment;
        for bit in word.iter().copied() {
            output.push(self.xor(bit, carry)?);
            carry = self.and(bit, carry)?;
        }
        Ok(output)
    }

    fn canonicalize_mersenne(&mut self, word: &[Lit]) -> Result<Word, String> {
        let all_ones = self.and_many(word.iter().copied())?;
        word.iter()
            .copied()
            .map(|bit| self.and(bit, all_ones.not()))
            .collect()
    }

    fn field_add(&mut self, left: &[Lit], right: &[Lit]) -> Result<Word, String> {
        if left.len() != right.len() || left.is_empty() {
            return Err("field-add width mismatch".into());
        }
        self.arithmetic.field_additions += 1;
        let width = left.len();
        let sum = self.ripple_add(left, right)?;
        let folded = self.increment_if(&sum[..width], sum[width])?;
        self.canonicalize_mersenne(&folded)
    }

    fn field_neg(&mut self, value: &[Lit]) -> Result<Word, String> {
        self.arithmetic.field_negations += 1;
        let complemented = value.iter().copied().map(Lit::not).collect::<Vec<_>>();
        self.canonicalize_mersenne(&complemented)
    }

    fn field_sub(&mut self, left: &[Lit], right: &[Lit]) -> Result<Word, String> {
        let negative = self.field_neg(right)?;
        self.field_add(left, &negative)
    }

    fn field_double(&mut self, value: &[Lit]) -> Result<Word, String> {
        self.field_add(value, value)
    }

    fn field_mul(&mut self, left: &[Lit], right: &[Lit]) -> Result<Word, String> {
        if left.len() != right.len() || left.is_empty() {
            return Err("field-multiply width mismatch".into());
        }
        self.arithmetic.field_multiplications += 1;
        let width = left.len();
        let mut columns = vec![Vec::<Lit>::new(); width * 2 + 2];
        for (i, left_bit) in left.iter().copied().enumerate() {
            for (j, right_bit) in right.iter().copied().enumerate() {
                columns[i + j].push(self.and(left_bit, right_bit)?);
            }
        }
        for column in 0..columns.len() - 1 {
            while columns[column].len() >= 3 {
                let c = columns[column].pop().expect("length checked");
                let b = columns[column].pop().expect("length checked");
                let a = columns[column].pop().expect("length checked");
                let (sum, carry) = self.full_adder(a, b, c)?;
                columns[column].push(sum);
                columns[column + 1].push(carry);
            }
        }
        if columns.last().is_some_and(|column| column.len() > 2) {
            return Err("carry-save product overflowed allocated columns".into());
        }
        let mut row0 = Vec::with_capacity(columns.len());
        let mut row1 = Vec::with_capacity(columns.len());
        for column in columns {
            row0.push(column.first().copied().unwrap_or(Lit::Const(false)));
            row1.push(column.get(1).copied().unwrap_or(Lit::Const(false)));
        }
        let product = self.ripple_add(&row0, &row1)?;
        let mut reduced = vec![Lit::Const(false); width];
        for chunk in product.chunks(width) {
            let mut padded = vec![Lit::Const(false); width];
            padded[..chunk.len()].copy_from_slice(chunk);
            reduced = self.field_add(&reduced, &padded)?;
        }
        Ok(reduced)
    }

    fn field_square(&mut self, value: &[Lit]) -> Result<Word, String> {
        self.field_mul(value, value)
    }

    fn field_zero(&mut self, value: &[Lit]) -> Result<Lit, String> {
        self.arithmetic.field_zero_tests += 1;
        self.and_many(value.iter().copied().map(Lit::not))
    }

    fn assert_word_equal(&mut self, left: &[Lit], right: &[Lit]) -> Result<(), String> {
        if left.len() != right.len() {
            return Err("field equality width mismatch".into());
        }
        for (left, right) in left.iter().copied().zip(right.iter().copied()) {
            let different = self.xor(left, right)?;
            self.assert_true(different.not());
        }
        Ok(())
    }

    fn assert_less_than_constant(&mut self, word: &[Lit], constant: usize) -> Result<(), String> {
        let mut borrow = Lit::Const(false);
        for (bit, input) in word.iter().copied().enumerate() {
            let constant_bit = (constant >> bit) & 1 != 0;
            borrow = if constant_bit {
                self.or(input.not(), borrow)?
            } else {
                self.and(input.not(), borrow)?
            };
        }
        self.assert_true(borrow);
        Ok(())
    }

    fn select_constant_word(
        &mut self,
        selector: &[Lit],
        constants: &[u64],
        width: usize,
    ) -> Result<Word, String> {
        let leaf_count = 1usize
            .checked_shl(selector.len() as u32)
            .ok_or_else(|| "selector width overflow".to_owned())?;
        if constants.is_empty() || constants.len() > leaf_count {
            return Err("selector constant-table shape mismatch".into());
        }
        let mut level = (0..leaf_count)
            .map(|index| Self::constant_word(*constants.get(index).unwrap_or(&constants[0]), width))
            .collect::<Vec<_>>();
        for select in selector.iter().copied() {
            let mut next = Vec::with_capacity(level.len() / 2);
            for pair_index in 0..level.len() / 2 {
                let pair = &level[2 * pair_index..2 * pair_index + 2];
                next.push(self.mux_word(select, &pair[1], &pair[0])?);
            }
            level = next;
        }
        level.pop().ok_or_else(|| "empty selector mux tree".into())
    }

    fn jacobian_double(&mut self, point: &PointBits, curve_a: u64) -> Result<PointBits, String> {
        self.arithmetic.jacobian_doublings += 1;
        let width = point.x.len();
        let xx = self.field_square(&point.x)?;
        let yy = self.field_square(&point.y)?;
        let yyyy = self.field_square(&yy)?;
        let zz = self.field_square(&point.z)?;

        let x_plus_yy = self.field_add(&point.x, &yy)?;
        let x_plus_yy_sq = self.field_square(&x_plus_yy)?;
        let s0 = self.field_sub(&x_plus_yy_sq, &xx)?;
        let s1 = self.field_sub(&s0, &yyyy)?;
        let s = self.field_double(&s1)?;

        let two_xx = self.field_double(&xx)?;
        let three_xx = self.field_add(&two_xx, &xx)?;
        let zzzz = self.field_square(&zz)?;
        let a_word = Self::constant_word(curve_a, width);
        let a_zzzz = self.field_mul(&a_word, &zzzz)?;
        let m = self.field_add(&three_xx, &a_zzzz)?;
        let m_sq = self.field_square(&m)?;
        let two_s = self.field_double(&s)?;
        let x3 = self.field_sub(&m_sq, &two_s)?;

        let s_minus_x3 = self.field_sub(&s, &x3)?;
        let m_times = self.field_mul(&m, &s_minus_x3)?;
        let two_yyyy = self.field_double(&yyyy)?;
        let four_yyyy = self.field_double(&two_yyyy)?;
        let eight_yyyy = self.field_double(&four_yyyy)?;
        let y3 = self.field_sub(&m_times, &eight_yyyy)?;

        let y_plus_z = self.field_add(&point.y, &point.z)?;
        let y_plus_z_sq = self.field_square(&y_plus_z)?;
        let z0 = self.field_sub(&y_plus_z_sq, &yy)?;
        let z3 = self.field_sub(&z0, &zz)?;
        Ok(PointBits {
            x: x3,
            y: y3,
            z: z3,
        })
    }

    fn jacobian_mixed_add(
        &mut self,
        point: &PointBits,
        affine_x: &[Lit],
        affine_y: &[Lit],
        curve_a: u64,
    ) -> Result<PointBits, String> {
        self.arithmetic.mixed_ec_additions += 1;
        let width = point.x.len();
        let z1z1 = self.field_square(&point.z)?;
        let u2 = self.field_mul(affine_x, &z1z1)?;
        let z1_cubed = self.field_mul(&point.z, &z1z1)?;
        let s2 = self.field_mul(affine_y, &z1_cubed)?;
        let h = self.field_sub(&u2, &point.x)?;
        let delta = self.field_sub(&s2, &point.y)?;

        let hh = self.field_square(&h)?;
        let two_hh = self.field_double(&hh)?;
        let i = self.field_double(&two_hh)?;
        let j = self.field_mul(&h, &i)?;
        let r = self.field_double(&delta)?;
        let v = self.field_mul(&point.x, &i)?;
        let r_sq = self.field_square(&r)?;
        let r_sq_minus_j = self.field_sub(&r_sq, &j)?;
        let two_v = self.field_double(&v)?;
        let generic_x = self.field_sub(&r_sq_minus_j, &two_v)?;
        let v_minus_x = self.field_sub(&v, &generic_x)?;
        let r_times = self.field_mul(&r, &v_minus_x)?;
        let y1j = self.field_mul(&point.y, &j)?;
        let two_y1j = self.field_double(&y1j)?;
        let generic_y = self.field_sub(&r_times, &two_y1j)?;
        let z_plus_h = self.field_add(&point.z, &h)?;
        let z_plus_h_sq = self.field_square(&z_plus_h)?;
        let z_minus_z1z1 = self.field_sub(&z_plus_h_sq, &z1z1)?;
        let generic_z = self.field_sub(&z_minus_z1z1, &hh)?;
        let generic = PointBits {
            x: generic_x,
            y: generic_y,
            z: generic_z,
        };

        let doubled = self.jacobian_double(point, curve_a)?;
        let infinity = PointBits {
            x: Self::constant_word(0, width),
            y: Self::constant_word(1, width),
            z: Self::constant_word(0, width),
        };
        let affine = PointBits {
            x: affine_x.to_vec(),
            y: affine_y.to_vec(),
            z: Self::constant_word(1, width),
        };

        let point_infinity = self.field_zero(&point.z)?;
        let h_zero = self.field_zero(&h)?;
        let delta_zero = self.field_zero(&delta)?;
        let y_zero = self.field_zero(&point.y)?;
        let same_or_tangent = self.mux_point(y_zero, &infinity, &doubled)?;
        let exceptional = self.mux_point(delta_zero, &same_or_tangent, &infinity)?;
        let non_infinity = self.mux_point(h_zero, &exceptional, &generic)?;
        self.mux_point(point_infinity, &affine, &non_infinity)
    }
}

#[derive(Clone, Debug)]
pub struct BooleanEncoding {
    case_id: String,
    relation_sha256: String,
    field_bits: usize,
    base_size: usize,
    selector_ids: Vec<Vec<u32>>,
    sign_ids: Vec<u32>,
    max_variable: u32,
    gates: Vec<Gate>,
    assertions: Vec<Lit>,
    arithmetic: ArithmeticCounts,
}

impl BooleanEncoding {
    pub fn from_relation(relation: &RelationSpec) -> Result<Self, String> {
        relation.validate().map_err(|error| error.to_string())?;
        let p_plus_one = relation
            .curve
            .p
            .checked_add(1)
            .ok_or_else(|| "field modulus overflow".to_owned())?;
        if !p_plus_one.is_power_of_two() {
            return Err("selector circuit requires a Mersenne-prime modulus".into());
        }
        let field_bits = p_plus_one.trailing_zeros() as usize;
        if field_bits == 0 || field_bits >= 64 {
            return Err("unsupported Mersenne field width".into());
        }
        let base_size = relation.base_size();
        let selector_width = usize::max(
            1,
            usize::BITS as usize - (base_size - 1).leading_zeros() as usize,
        );
        let relation_bytes = serde_json::to_vec(relation).map_err(|error| error.to_string())?;
        let relation_sha256 = hex_digest(&relation_bytes);

        let mut builder = CircuitBuilder::default();
        let mut selector_lits = Vec::with_capacity(8);
        let mut selector_ids = Vec::with_capacity(8);
        let mut sign_lits = Vec::with_capacity(8);
        let mut sign_ids = Vec::with_capacity(8);
        for _ in 0..8 {
            let selector = (0..selector_width)
                .map(|_| builder.input())
                .collect::<Result<Vec<_>, _>>()?;
            let ids = selector
                .iter()
                .map(|literal| literal.signed().expect("input is a variable") as u32)
                .collect::<Vec<_>>();
            let sign = builder.input()?;
            sign_ids.push(sign.signed().expect("input is a variable") as u32);
            selector_lits.push(selector);
            selector_ids.push(ids);
            sign_lits.push(sign);
        }
        // Allocate every semantic bit before any range-check gate.  This makes
        // the semantic prefix explicit and lets external wrappers return only
        // the first 24--32 model bits without exposing auxiliary assignments.
        if !base_size.is_power_of_two() {
            for selector in &selector_lits {
                builder.assert_less_than_constant(selector, base_size)?;
            }
        }

        let mut selected = Vec::with_capacity(8);
        for slot in 0..8 {
            let xs = relation.slots[slot]
                .points
                .iter()
                .map(|point| point.x)
                .collect::<Vec<_>>();
            let ys = relation.slots[slot]
                .points
                .iter()
                .map(|point| point.y)
                .collect::<Vec<_>>();
            let x = builder.select_constant_word(&selector_lits[slot], &xs, field_bits)?;
            let positive_y = builder.select_constant_word(&selector_lits[slot], &ys, field_bits)?;
            let negative_y = builder.field_neg(&positive_y)?;
            let y = builder.mux_word(sign_lits[slot], &negative_y, &positive_y)?;
            selected.push((x, y));
        }

        // The first selected affine point is already a valid Jacobian point;
        // avoiding an addition from infinity saves one complete EC block.
        let mut sum = PointBits {
            x: selected[0].0.clone(),
            y: selected[0].1.clone(),
            z: CircuitBuilder::constant_word(1, field_bits),
        };
        for (x, y) in selected.iter().skip(1) {
            sum = builder.jacobian_mixed_add(&sum, x, y, relation.curve.a)?;
        }

        let z_zero = builder.field_zero(&sum.z)?;
        builder.assert_true(z_zero.not());
        let z2 = builder.field_square(&sum.z)?;
        let z3 = builder.field_mul(&z2, &sum.z)?;
        let target_x = CircuitBuilder::constant_word(relation.target.x, field_bits);
        let target_y = CircuitBuilder::constant_word(relation.target.y, field_bits);
        let target_x_z2 = builder.field_mul(&target_x, &z2)?;
        let target_y_z3 = builder.field_mul(&target_y, &z3)?;
        builder.assert_word_equal(&sum.x, &target_x_z2)?;
        builder.assert_word_equal(&sum.y, &target_y_z3)?;

        Ok(Self {
            case_id: relation.case_id.clone(),
            relation_sha256,
            field_bits,
            base_size,
            selector_ids,
            sign_ids,
            max_variable: builder.next_id,
            gates: builder.gates,
            assertions: builder.assertions,
            arithmetic: builder.arithmetic,
        })
    }

    pub fn statistics(&self) -> EncodingStatistics {
        let and_gates = self
            .gates
            .iter()
            .filter(|gate| matches!(gate.kind, GateKind::And(..)))
            .count();
        let xor_gates = self.gates.len() - and_gates;
        EncodingStatistics {
            schema: ENCODING_SCHEMA.into(),
            case_id: self.case_id.clone(),
            relation_sha256: self.relation_sha256.clone(),
            field_bits: self.field_bits,
            base_size: self.base_size,
            semantic_input_bits: self.selector_ids.iter().map(Vec::len).sum::<usize>()
                + self.sign_ids.len(),
            boolean_variables: self.max_variable,
            and_gates,
            xor_gates,
            assertions: self.assertions.len(),
            ordinary_cnf_clauses: and_gates * 3 + xor_gates * 4 + self.assertions.len(),
            cnf_xor_ordinary_clauses: and_gates * 3 + self.assertions.len(),
            cnf_xor_rows: xor_gates,
            arithmetic: self.arithmetic.clone(),
            forbidden: ForbiddenCounters::default(),
        }
    }

    pub fn evaluate_witness(&self, witness: &SelectorWitness) -> Result<bool, String> {
        let size = self.max_variable as usize + 1;
        let mut assignment = vec![false; size];
        for slot in 0..8 {
            if witness.indices[slot] >= self.base_size {
                return Err(format!("slot {slot} index is outside the base"));
            }
            for (bit, id) in self.selector_ids[slot].iter().copied().enumerate() {
                assignment[id as usize] = (witness.indices[slot] >> bit) & 1 != 0;
            }
            assignment[self.sign_ids[slot] as usize] = witness.signs[slot];
        }
        evaluate_gates(&self.gates, &mut assignment);
        Ok(self
            .assertions
            .iter()
            .all(|literal| literal.value(&assignment)))
    }

    pub fn replay_witness(
        &self,
        relation: &RelationSpec,
        witness: &SelectorWitness,
    ) -> Result<bool, String> {
        if relation.case_id != self.case_id {
            return Err("relation/circuit case ID mismatch".into());
        }
        if hex_digest(&serde_json::to_vec(relation).map_err(|error| error.to_string())?)
            != self.relation_sha256
        {
            return Err("relation/circuit digest mismatch".into());
        }
        Ok(relation.replay(&witness.indices, &witness.signs) == Some(relation.target))
    }

    /// Decode the semantic model prefix emitted by the pinned external wrapper.
    ///
    /// The circuit allocates all index bits and signs before any auxiliary
    /// gate.  The returned witness is still recomputed through every gate and
    /// independently replayed on the curve; no auxiliary model bit is trusted.
    pub fn decode_semantic_prefix(
        &self,
        relation: &RelationSpec,
        prefix: &str,
    ) -> Result<SelectorWitness, String> {
        let expected = self.selector_ids.iter().map(Vec::len).sum::<usize>() + self.sign_ids.len();
        if prefix.len() != expected {
            return Err(format!(
                "semantic model prefix has {} bits; expected {expected}",
                prefix.len()
            ));
        }
        let bits = prefix
            .bytes()
            .map(|byte| match byte {
                b'0' => Ok(false),
                b'1' => Ok(true),
                _ => Err("semantic model prefix contains a non-binary character".to_owned()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut cursor = 0;
        let mut witness = SelectorWitness {
            indices: [0; 8],
            signs: [false; 8],
        };
        for slot in 0..8 {
            for bit in 0..self.selector_ids[slot].len() {
                witness.indices[slot] |= usize::from(bits[cursor]) << bit;
                cursor += 1;
            }
            witness.signs[slot] = bits[cursor];
            cursor += 1;
            if witness.indices[slot] >= self.base_size {
                return Err(format!("decoded slot {slot} index is outside the base"));
            }
        }
        if !self.evaluate_witness(&witness)? {
            return Err("semantic model prefix fails complete circuit reevaluation".into());
        }
        if !self.replay_witness(relation, &witness)? {
            return Err("semantic model prefix fails independent EC replay".into());
        }
        Ok(witness)
    }

    pub fn decode_solver_model(
        &self,
        relation: &RelationSpec,
        solver_output: &str,
    ) -> Result<SelectorWitness, String> {
        let sat_line = solver_output.lines().any(|line| {
            let line = line.trim();
            line == "s SATISFIABLE" || line == "SAT" || line == "SATISFIABLE"
        });
        if !sat_line {
            return Err("solver output does not declare SAT".into());
        }
        let mut values = vec![None; self.max_variable as usize + 1];
        for line in solver_output.lines().map(str::trim) {
            let body = if let Some(rest) = line.strip_prefix("v ") {
                rest
            } else if line.starts_with('-')
                || line.as_bytes().first().is_some_and(u8::is_ascii_digit)
            {
                line
            } else {
                continue;
            };
            for token in body.split_whitespace() {
                let literal = token
                    .parse::<i64>()
                    .map_err(|_| format!("invalid model literal {token:?}"))?;
                if literal == 0 {
                    continue;
                }
                let id = literal.unsigned_abs() as usize;
                if id == 0 || id > self.max_variable as usize {
                    return Err(format!("model variable {id} is outside the formula"));
                }
                let value = literal > 0;
                if values[id].is_some_and(|old| old != value) {
                    return Err(format!("contradictory model value for variable {id}"));
                }
                values[id] = Some(value);
            }
        }
        let mut assignment = vec![false; values.len()];
        for id in 1..values.len() {
            assignment[id] = values[id].ok_or_else(|| format!("model omits variable {id}"))?;
        }
        for gate in &self.gates {
            let expected = match gate.kind {
                GateKind::And(left, right) => left.value(&assignment) & right.value(&assignment),
                GateKind::Xor(left, right) => left.value(&assignment) ^ right.value(&assignment),
            };
            if assignment[gate.output as usize] != expected {
                return Err(format!(
                    "model violates gate defining variable {}",
                    gate.output
                ));
            }
        }
        if !self
            .assertions
            .iter()
            .all(|literal| literal.value(&assignment))
        {
            return Err("model violates a circuit assertion".into());
        }
        let mut witness = SelectorWitness {
            indices: [0; 8],
            signs: [false; 8],
        };
        for slot in 0..8 {
            for (bit, id) in self.selector_ids[slot].iter().copied().enumerate() {
                witness.indices[slot] |= usize::from(assignment[id as usize]) << bit;
            }
            witness.signs[slot] = assignment[self.sign_ids[slot] as usize];
            if witness.indices[slot] >= self.base_size {
                return Err(format!("decoded slot {slot} index is outside the base"));
            }
        }
        if !self.evaluate_witness(&witness)? {
            return Err("decoded semantic witness fails circuit reevaluation".into());
        }
        if !self.replay_witness(relation, &witness)? {
            return Err("decoded semantic witness fails independent EC replay".into());
        }
        Ok(witness)
    }

    pub fn to_dimacs(&self) -> String {
        self.render(false)
    }

    pub fn to_cnf_xor(&self) -> String {
        self.render(true)
    }

    fn render(&self, native_xor: bool) -> String {
        let (clauses, xors) = self.constraints(native_xor);
        let mut output = String::new();
        writeln!(output, "c schema {ENCODING_SCHEMA}").expect("String write");
        writeln!(output, "c case_id {}", self.case_id).expect("String write");
        writeln!(output, "c relation_sha256 {}", self.relation_sha256).expect("String write");
        writeln!(output, "c field_bits {}", self.field_bits).expect("String write");
        for slot in 0..8 {
            let selector = self.selector_ids[slot]
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",");
            writeln!(
                output,
                "c slot {slot} selector_bits {selector} sign {}",
                self.sign_ids[slot]
            )
            .expect("String write");
        }
        writeln!(
            output,
            "p cnf {} {}",
            self.max_variable,
            clauses.len() + xors.len()
        )
        .expect("String write");
        for clause in clauses {
            for literal in clause {
                write!(output, "{literal} ").expect("String write");
            }
            output.push_str("0\n");
        }
        for (variables, rhs) in xors {
            output.push_str("x ");
            for (index, variable) in variables.iter().copied().enumerate() {
                let literal = if index == 0 && !rhs {
                    -(variable as i64)
                } else {
                    variable as i64
                };
                write!(output, "{literal} ").expect("String write");
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
            Lit::Var { .. } => {
                let signed = literal.signed().expect("variable literal");
                if clause.contains(&-signed) {
                    return;
                }
                clause.insert(signed);
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

fn hex_digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

pub fn validate_dimacs(input: &str, allow_xor: bool) -> Result<(), String> {
    let mut header = None;
    let mut seen = 0usize;
    for (line_index, line) in input.lines().map(str::trim).enumerate() {
        if line.is_empty() || line.starts_with('c') {
            continue;
        }
        if line.starts_with('p') {
            if header.is_some() || seen != 0 {
                return Err(format!(
                    "misplaced DIMACS header on line {}",
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
            header = Some((
                fields[2]
                    .parse::<u32>()
                    .map_err(|_| "invalid variable count")?,
                fields[3]
                    .parse::<usize>()
                    .map_err(|_| "invalid constraint count")?,
            ));
            continue;
        }
        let (variables, expected) = header.ok_or("constraint before DIMACS header")?;
        if seen >= expected {
            return Err("trailing DIMACS constraint".into());
        }
        let mut fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.first() == Some(&"x") {
            if !allow_xor {
                return Err("XOR row in ordinary DIMACS".into());
            }
            fields.remove(0);
        }
        if fields.last() != Some(&"0") {
            return Err(format!(
                "unterminated constraint on line {}",
                line_index + 1
            ));
        }
        for field in &fields[..fields.len() - 1] {
            let literal = field.parse::<i64>().map_err(|_| "invalid DIMACS literal")?;
            if literal == 0 || literal.unsigned_abs() > u64::from(variables) {
                return Err(format!("literal out of range on line {}", line_index + 1));
            }
        }
        seen += 1;
    }
    let (_, expected) = header.ok_or("missing DIMACS header")?;
    if seen != expected {
        return Err(format!(
            "saw {seen} constraints; header declares {expected}"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{AffinePoint, Curve};
    use crate::ir::{RelationSpec, SlotDomain, RELATION_SCHEMA};
    use crate::panel::{planted_word, prime31_weighted_relation, PanelTarget};

    fn eval_field_binary(width: usize, left: u64, right: u64, multiply: bool) -> u64 {
        let mut builder = CircuitBuilder::default();
        let left_bits = (0..width)
            .map(|_| builder.input().unwrap())
            .collect::<Vec<_>>();
        let right_bits = (0..width)
            .map(|_| builder.input().unwrap())
            .collect::<Vec<_>>();
        let output = if multiply {
            builder.field_mul(&left_bits, &right_bits).unwrap()
        } else {
            builder.field_add(&left_bits, &right_bits).unwrap()
        };
        let mut assignment = vec![false; builder.next_id as usize + 1];
        for bit in 0..width {
            assignment[builder.input_ids[bit] as usize] = (left >> bit) & 1 != 0;
            assignment[builder.input_ids[width + bit] as usize] = (right >> bit) & 1 != 0;
        }
        evaluate_gates(&builder.gates, &mut assignment);
        output.iter().enumerate().fold(0, |value, (bit, literal)| {
            value | (u64::from(literal.value(&assignment)) << bit)
        })
    }

    #[test]
    fn mersenne_p31_addition_and_multiplication_are_exhaustive() {
        let modulus = 31;
        for left in 0..modulus {
            for right in 0..modulus {
                assert_eq!(
                    eval_field_binary(5, left, right, false),
                    (left + right) % modulus
                );
                assert_eq!(
                    eval_field_binary(5, left, right, true),
                    (left * right) % modulus
                );
            }
        }
    }

    #[test]
    fn mersenne_p127_deterministic_samples_match_integer_arithmetic() {
        let modulus = 127;
        let mut state = 0x7365_6c65_6374_6f72u64;
        for _ in 0..128 {
            state =
                state.wrapping_add(0x9e37_79b9_7f4a_7c15).rotate_left(17) ^ 0xbf58_476d_1ce4_e5b9;
            let left = state % modulus;
            state = state.wrapping_mul(0x94d0_49bb_1331_11eb).rotate_right(11);
            let right = state % modulus;
            assert_eq!(
                eval_field_binary(7, left, right, false),
                (left + right) % modulus
            );
            assert_eq!(
                eval_field_binary(7, left, right, true),
                (left * right) % modulus
            );
        }
    }

    fn tiny_relation(target: AffinePoint) -> RelationSpec {
        let curve = Curve {
            p: 31,
            a: 2,
            b: 3,
            order: None,
        };
        let base = curve.canonical_base(1).unwrap();
        RelationSpec {
            schema: RELATION_SCHEMA.into(),
            case_id: "tiny-p31-b1".into(),
            curve,
            target,
            slots: (0..8)
                .map(|slot| SlotDomain {
                    label: format!("slot-{slot}"),
                    points: base.clone(),
                })
                .collect(),
        }
    }

    #[test]
    fn exhaustive_tiny_selector_circuit_matches_affine_group_law() {
        let curve = Curve {
            p: 31,
            a: 2,
            b: 3,
            order: None,
        };
        curve.validate().unwrap();
        let seed_relation = tiny_relation(curve.canonical_base(1).unwrap()[0]);
        let mut targets = Vec::new();
        for mask in 0u16..256 {
            let witness = SelectorWitness {
                indices: [0; 8],
                signs: std::array::from_fn(|slot| (mask >> slot) & 1 != 0),
            };
            if let Some(target) = seed_relation.replay(&witness.indices, &witness.signs) {
                targets.push((target, witness));
            }
        }
        targets.sort_by_key(|(point, _)| (point.x, point.y));
        targets.dedup_by_key(|(point, _)| (point.x, point.y));
        for (target, _) in targets {
            let relation = tiny_relation(target);
            let encoding = BooleanEncoding::from_relation(&relation).unwrap();
            for mask in 0u16..256 {
                let witness = SelectorWitness {
                    indices: [0; 8],
                    signs: std::array::from_fn(|slot| (mask >> slot) & 1 != 0),
                };
                let direct = relation.replay(&witness.indices, &witness.signs) == Some(target);
                assert_eq!(encoding.evaluate_witness(&witness).unwrap(), direct);
            }
        }
    }

    #[test]
    fn prime31_planted_witness_passes_circuit_and_replay() {
        let relation = prime31_weighted_relation(4, PanelTarget::Planted(0)).unwrap();
        let encoding = BooleanEncoding::from_relation(&relation).unwrap();
        let (indices, signs) = planted_word(4, 0);
        let witness = SelectorWitness { indices, signs };
        assert!(encoding.evaluate_witness(&witness).unwrap());
        assert!(encoding.replay_witness(&relation, &witness).unwrap());
        assert!(encoding.statistics().forbidden.all_zero());
    }

    #[test]
    fn semantic_prefix_round_trips_for_non_power_of_two_base() {
        let relation = prime31_weighted_relation(6, PanelTarget::Planted(3)).unwrap();
        let encoding = BooleanEncoding::from_relation(&relation).unwrap();
        let (indices, signs) = planted_word(6, 3);
        let mut prefix = String::new();
        for slot in 0..8 {
            for bit in 0..3 {
                prefix.push(if (indices[slot] >> bit) & 1 != 0 {
                    '1'
                } else {
                    '0'
                });
            }
            prefix.push(if signs[slot] { '1' } else { '0' });
        }
        assert_eq!(
            encoding.decode_semantic_prefix(&relation, &prefix).unwrap(),
            SelectorWitness { indices, signs }
        );
    }

    #[test]
    fn dimacs_renderings_are_strict_and_deterministic() {
        let relation = prime31_weighted_relation(4, PanelTarget::Planted(0)).unwrap();
        let encoding = BooleanEncoding::from_relation(&relation).unwrap();
        let cnf = encoding.to_dimacs();
        let xor = encoding.to_cnf_xor();
        validate_dimacs(&cnf, false).unwrap();
        validate_dimacs(&xor, true).unwrap();
        assert_eq!(cnf, encoding.to_dimacs());
        assert_eq!(xor, encoding.to_cnf_xor());
        assert!(validate_dimacs(&xor, false).is_err());
    }
}
