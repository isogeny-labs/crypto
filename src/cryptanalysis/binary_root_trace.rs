//! Bounded, opt-in quadratic roots over polynomial-basis GF(2^n).
//!
//! The producer follows the existing quadratic/Artin--Schreier method, with
//! explicit inverse, trace, half-trace and binary-linear-system states. Legacy
//! APIs are unchanged. A separate portable checker certifies distinct roots by
//! gcd(f, X^(2^n)+X); it does not invoke the producer or search for roots.
//!
//! Work is selected field calls plus matrix bit tests/word XORs. It excludes
//! validation, serialization, replay and loop/control overhead. Supplied event
//! identities and tallies do not prove complete execution or total machine cost.

use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use serde::{Deserialize, Serialize};

pub const MAX_FIELD_DEGREE: u32 = 61;
pub const MAX_POLYNOMIAL_DEGREE: usize = 2;
pub const MAX_WORK: u64 = 1_000_000;
pub const MAX_EVENTS: u64 = 100_000;
const MAX_SINK_ERROR_BYTES: usize = 4096;
pub const WORK_SCOPE: &str = "field add/mul/square calls plus matrix pivot bit tests and word XORs; inverse calls informational; excludes input validation, serialization, replay, loop/control overhead and adapter setup/lifts";

/// Full monic polynomial bitmask: bit n represents the leading X^n term.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FieldSpec {
    pub degree: u32,
    pub modulus: u64,
}
impl FieldSpec {
    fn validate(self) -> Result<(), String> {
        if !(3..=MAX_FIELD_DEGREE).contains(&self.degree)
            || self.modulus >> self.degree != 1
            || self.modulus & 1 == 0
            || !irreducible(self)
        {
            return Err("invalid or reducible binary field modulus".into());
        }
        Ok(())
    }
    fn order(self) -> u64 {
        1u64 << self.degree
    }
    fn native(self) -> IrreduciblePoly {
        IrreduciblePoly {
            degree: self.degree,
            low_terms: (0..self.degree)
                .filter(|i| self.modulus & (1u64 << i) != 0)
                .collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootLimits {
    pub work: u64,
    pub events: u64,
}
impl RootLimits {
    fn validate(self) -> Result<(), String> {
        if self.work > MAX_WORK || self.events > MAX_EVENTS {
            return Err("binary root limits exceed hard bounds".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootCounters {
    pub additions: u64,
    pub multiplications: u64,
    pub squares: u64,
    pub inverse_calls: u64,
    pub matrix_bit_tests: u64,
    pub matrix_word_xors: u64,
}
impl RootCounters {
    /// Mixed primitive admission units, not normalized field work or instructions.
    pub fn work(&self) -> u64 {
        self.additions
            .saturating_add(self.multiplications)
            .saturating_add(self.squares)
            .saturating_add(self.matrix_bit_tests)
            .saturating_add(self.matrix_word_xors)
    }
    fn observe(&mut self, event: &RootEvent) {
        match event {
            RootEvent::Field { op, .. } => match op {
                FieldOp::Add => self.additions += 1,
                FieldOp::Mul => self.multiplications += 1,
                FieldOp::Square => self.squares += 1,
            },
            RootEvent::InverseBegin { .. } => self.inverse_calls += 1,
            RootEvent::MatrixBitTest { .. } => self.matrix_bit_tests += 1,
            RootEvent::MatrixWordXor { .. } => self.matrix_word_xors += 1,
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldOp {
    Add,
    Mul,
    Square,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Branch {
    ZeroPolynomial,
    Constant,
    Linear,
    PureSquare,
    Quadratic,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixPhase {
    Column,
    Target,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RootEvent {
    Begin {
        field: FieldSpec,
        polynomial: Vec<u64>,
    },
    Branch {
        branch: Branch,
    },
    Field {
        op: FieldOp,
        a: u64,
        b: u64,
        value: u64,
    },
    InverseBegin {
        id: u64,
        value: u64,
    },
    InversePower {
        id: u64,
        exponent: u64,
        value: u64,
    },
    InverseEnd {
        id: u64,
        value: u64,
        inverse: u64,
    },
    Normalized {
        argument: u64,
    },
    TraceBegin {
        argument: u64,
    },
    TraceStep {
        index: u32,
        power: u64,
        sum: u64,
    },
    TraceEnd {
        value: u64,
    },
    HalfTraceBegin {
        argument: u64,
    },
    HalfTraceStep {
        index: u32,
        power: u64,
        sum: u64,
    },
    HalfTraceEnd {
        solution: u64,
    },
    MatrixBegin {
        argument: u64,
    },
    MatrixColumn {
        index: u32,
        basis: u64,
        image: u64,
    },
    MatrixBitTest {
        phase: MatrixPhase,
        column: u32,
        value: u64,
        pivot_image: u64,
        matched: bool,
    },
    MatrixWordXor {
        a: u64,
        b: u64,
        value: u64,
    },
    MatrixReduction {
        phase: MatrixPhase,
        column: u32,
        image_before: u64,
        preimage_before: u64,
        pivot_image: u64,
        pivot_preimage: u64,
        image: u64,
        preimage: u64,
    },
    MatrixPivot {
        column: u32,
        image: u64,
        preimage: u64,
    },
    MatrixEnd {
        rank: u32,
        solution: u64,
        residual: u64,
    },
    SquareRootStep {
        index: u32,
        value: u64,
    },
    Scale {
        value: u64,
    },
    ExtractedRoot {
        root: u64,
    },
    Finish {
        roots: Vec<u64>,
        all_field_elements: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootStop {
    Complete,
    AllFieldElements,
    WorkLimit,
    EventLimit,
    SinkFailure,
    /// Fail closed if a trace-zero linear system unexpectedly has a residual.
    LinearSystemFailure,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootReport {
    pub schema: String,
    pub field: FieldSpec,
    pub polynomial: Vec<u64>,
    pub limits: RootLimits,
    pub stop: RootStop,
    pub counters: RootCounters,
    /// Events acknowledged by the sink, in order.
    pub events: Vec<RootEvent>,
    /// Executed work remains charged even if its event could not be acknowledged.
    pub unacknowledged: Option<RootEvent>,
    pub sink_error: Option<String>,
    /// Populated only after an acknowledged terminal event.
    pub roots: Vec<u64>,
    pub execution_trace_complete_verified: bool,
    pub total_operation_count_verified: bool,
    pub work_scope: String,
}
impl RootReport {
    pub fn complete(&self) -> bool {
        matches!(self.stop, RootStop::Complete | RootStop::AllFieldElements)
    }
}

fn trim(mut polynomial: Vec<u64>) -> Vec<u64> {
    while polynomial.last() == Some(&0) {
        polynomial.pop();
    }
    polynomial
}
fn validate_input(f: &[u64], field: FieldSpec, limits: RootLimits) -> Result<(), String> {
    field.validate()?;
    limits.validate()?;
    if f.len() > MAX_POLYNOMIAL_DEGREE + 1 || f.iter().any(|&x| x >= field.order()) {
        return Err("binary root coefficients are noncanonical or degree exceeds two".into());
    }
    Ok(())
}
fn branch(f: &[u64]) -> Branch {
    match f.len() {
        0 => Branch::ZeroPolynomial,
        1 => Branch::Constant,
        2 => Branch::Linear,
        _ if f[1] == 0 => Branch::PureSquare,
        _ => Branch::Quadratic,
    }
}

struct Engine<'a> {
    report: RootReport,
    irr: IrreduciblePoly,
    sink: &'a mut dyn FnMut(&RootEvent) -> Result<(), String>,
    next_inverse: u64,
}
type Work<T> = Result<T, RootStop>;
impl Engine<'_> {
    fn room(&self) -> Work<()> {
        if self.report.events.len() as u64 >= self.report.limits.events {
            Err(RootStop::EventLimit)
        } else {
            Ok(())
        }
    }
    fn primitive_room(&self) -> Work<()> {
        self.room()?;
        if self.report.counters.work() >= self.report.limits.work {
            Err(RootStop::WorkLimit)
        } else {
            Ok(())
        }
    }
    fn event(&mut self, event: RootEvent) -> Work<()> {
        self.room()?;
        self.report.counters.observe(&event);
        if let Err(mut error) = (self.sink)(&event) {
            let mut end = error.len().min(MAX_SINK_ERROR_BYTES);
            while !error.is_char_boundary(end) {
                end -= 1;
            }
            error.truncate(end);
            self.report.sink_error = Some(error);
            self.report.unacknowledged = Some(event);
            return Err(RootStop::SinkFailure);
        }
        self.report.events.push(event);
        Ok(())
    }
    fn field(&mut self, op: FieldOp, a: u64, b: u64) -> Work<u64> {
        self.primitive_room()?;
        let n = self.report.field.degree;
        let left = F2mElement::from_words(&[a], n);
        let right = F2mElement::from_words(&[b], n);
        let value = match op {
            FieldOp::Add => left.add(&right),
            FieldOp::Mul => left.mul(&right, &self.irr),
            FieldOp::Square => left.square(&self.irr),
        }
        .raw_bits()[0];
        self.event(RootEvent::Field { op, a, b, value })?;
        Ok(value)
    }
    fn square(&mut self, value: u64) -> Work<u64> {
        self.field(FieldOp::Square, value, value)
    }
    fn inverse(&mut self, value: u64) -> Work<u64> {
        let id = self.next_inverse;
        self.next_inverse += 1;
        self.event(RootEvent::InverseBegin { id, value })?;
        let n = self.report.field.degree - 1;
        let mut beta = value;
        let mut length = 1u32;
        self.event(RootEvent::InversePower {
            id,
            exponent: 1,
            value: beta,
        })?;
        for bit_index in (0..(31 - n.leading_zeros())).rev() {
            let mut shifted = beta;
            for _ in 0..length {
                shifted = self.square(shifted)?;
            }
            beta = self.field(FieldOp::Mul, beta, shifted)?;
            length *= 2;
            self.event(RootEvent::InversePower {
                id,
                exponent: (1u64 << length) - 1,
                value: beta,
            })?;
            if n & (1u32 << bit_index) != 0 {
                let squared = self.square(beta)?;
                beta = self.field(FieldOp::Mul, squared, value)?;
                length += 1;
                self.event(RootEvent::InversePower {
                    id,
                    exponent: (1u64 << length) - 1,
                    value: beta,
                })?;
            }
        }
        let inverse = self.square(beta)?;
        self.event(RootEvent::InverseEnd { id, value, inverse })?;
        Ok(inverse)
    }
    fn trace(&mut self, argument: u64) -> Work<u64> {
        self.event(RootEvent::TraceBegin { argument })?;
        let (mut power, mut sum) = (argument, argument);
        self.event(RootEvent::TraceStep {
            index: 0,
            power,
            sum,
        })?;
        for index in 1..self.report.field.degree {
            power = self.square(power)?;
            sum = self.field(FieldOp::Add, sum, power)?;
            self.event(RootEvent::TraceStep { index, power, sum })?;
        }
        self.event(RootEvent::TraceEnd { value: sum })?;
        Ok(sum)
    }
    fn half_trace(&mut self, argument: u64) -> Work<u64> {
        self.event(RootEvent::HalfTraceBegin { argument })?;
        let (mut power, mut sum) = (argument, argument);
        self.event(RootEvent::HalfTraceStep {
            index: 0,
            power,
            sum,
        })?;
        for index in 1..=self.report.field.degree / 2 {
            power = self.square(power)?;
            power = self.square(power)?;
            sum = self.field(FieldOp::Add, sum, power)?;
            self.event(RootEvent::HalfTraceStep { index, power, sum })?;
        }
        self.event(RootEvent::HalfTraceEnd { solution: sum })?;
        Ok(sum)
    }
    fn bit_test(
        &mut self,
        phase: MatrixPhase,
        column: u32,
        value: u64,
        pivot_image: u64,
    ) -> Work<bool> {
        self.primitive_room()?;
        let matched = value & (1u64 << (63 - pivot_image.leading_zeros())) != 0;
        self.event(RootEvent::MatrixBitTest {
            phase,
            column,
            value,
            pivot_image,
            matched,
        })?;
        Ok(matched)
    }
    fn word_xor(&mut self, a: u64, b: u64) -> Work<u64> {
        self.primitive_room()?;
        let value = a ^ b;
        self.event(RootEvent::MatrixWordXor { a, b, value })?;
        Ok(value)
    }
    fn reduce(
        &mut self,
        phase: MatrixPhase,
        column: u32,
        image_before: u64,
        preimage_before: u64,
        pivot: (u64, u64),
    ) -> Work<(u64, u64)> {
        let image = self.word_xor(image_before, pivot.0)?;
        let preimage = self.word_xor(preimage_before, pivot.1)?;
        self.event(RootEvent::MatrixReduction {
            phase,
            column,
            image_before,
            preimage_before,
            pivot_image: pivot.0,
            pivot_preimage: pivot.1,
            image,
            preimage,
        })?;
        Ok((image, preimage))
    }
    fn linear_system(&mut self, argument: u64) -> Work<u64> {
        self.event(RootEvent::MatrixBegin { argument })?;
        let n = self.report.field.degree;
        let mut pivots: Vec<(u64, u64)> = Vec::with_capacity(n as usize);
        for column in 0..n {
            let basis = 1u64 << column;
            let squared = self.square(basis)?;
            let mut image = self.field(FieldOp::Add, squared, basis)?;
            let mut preimage = basis;
            self.event(RootEvent::MatrixColumn {
                index: column,
                basis,
                image,
            })?;
            for &pivot in &pivots {
                if self.bit_test(MatrixPhase::Column, column, image, pivot.0)? {
                    (image, preimage) =
                        self.reduce(MatrixPhase::Column, column, image, preimage, pivot)?;
                }
            }
            if image != 0 {
                self.event(RootEvent::MatrixPivot {
                    column,
                    image,
                    preimage,
                })?;
                pivots.push((image, preimage));
                pivots.sort_by_key(|p| std::cmp::Reverse(p.0));
            }
        }
        let (mut residual, mut solution) = (argument, 0u64);
        for &pivot in &pivots {
            if self.bit_test(MatrixPhase::Target, n, residual, pivot.0)? {
                (residual, solution) =
                    self.reduce(MatrixPhase::Target, n, residual, solution, pivot)?;
            }
        }
        self.event(RootEvent::MatrixEnd {
            rank: pivots.len() as u32,
            solution,
            residual,
        })?;
        if residual != 0 {
            return Err(RootStop::LinearSystemFailure);
        }
        Ok(solution)
    }
    fn finish(&mut self, mut roots: Vec<u64>, all_field_elements: bool) -> Work<RootStop> {
        roots.sort_unstable();
        roots.dedup();
        for &root in &roots {
            self.event(RootEvent::ExtractedRoot { root })?;
        }
        self.event(RootEvent::Finish {
            roots: roots.clone(),
            all_field_elements,
        })?;
        self.report.roots = roots;
        Ok(if all_field_elements {
            RootStop::AllFieldElements
        } else {
            RootStop::Complete
        })
    }
    fn run(&mut self) -> Work<RootStop> {
        self.event(RootEvent::Begin {
            field: self.report.field,
            polynomial: self.report.polynomial.clone(),
        })?;
        let chosen = branch(&self.report.polynomial);
        self.event(RootEvent::Branch { branch: chosen })?;
        let coefficient = |i| self.report.polynomial.get(i).copied().unwrap_or(0);
        let (c, b, a) = (coefficient(0), coefficient(1), coefficient(2));
        match chosen {
            Branch::ZeroPolynomial => self.finish(vec![], true),
            Branch::Constant => self.finish(vec![], false),
            Branch::Linear => {
                let inv = self.inverse(b)?;
                let root = self.field(FieldOp::Mul, c, inv)?;
                self.finish(vec![root], false)
            }
            Branch::PureSquare => {
                let inv = self.inverse(a)?;
                let mut root = self.field(FieldOp::Mul, c, inv)?;
                self.event(RootEvent::SquareRootStep {
                    index: 0,
                    value: root,
                })?;
                for index in 1..self.report.field.degree {
                    root = self.square(root)?;
                    self.event(RootEvent::SquareRootStep { index, value: root })?;
                }
                self.finish(vec![root], false)
            }
            Branch::Quadratic => {
                let b_squared = self.square(b)?;
                let inv_b_squared = self.inverse(b_squared)?;
                let ac = self.field(FieldOp::Mul, a, c)?;
                let argument = self.field(FieldOp::Mul, ac, inv_b_squared)?;
                self.event(RootEvent::Normalized { argument })?;
                if self.trace(argument)? != 0 {
                    return self.finish(vec![], false);
                }
                let z = if self.report.field.degree.is_multiple_of(2) {
                    self.linear_system(argument)?
                } else {
                    self.half_trace(argument)?
                };
                let other = self.field(FieldOp::Add, z, 1)?;
                let inv_a = self.inverse(a)?;
                let scale = self.field(FieldOp::Mul, b, inv_a)?;
                self.event(RootEvent::Scale { value: scale })?;
                let left = self.field(FieldOp::Mul, scale, z)?;
                let right = self.field(FieldOp::Mul, scale, other)?;
                self.finish(vec![left, right], false)
            }
        }
    }
}

/// Solve a bounded quadratic. Input errors are distinct from incomplete reports.
pub fn solve(
    f: &[u64],
    field: FieldSpec,
    limits: RootLimits,
    sink: &mut dyn FnMut(&RootEvent) -> Result<(), String>,
) -> Result<RootReport, String> {
    validate_input(f, field, limits)?;
    let mut engine = Engine {
        report: RootReport {
            schema: "binary-root-trace/v1".into(),
            field,
            polynomial: trim(f.to_vec()),
            limits,
            stop: RootStop::WorkLimit,
            counters: RootCounters::default(),
            events: vec![],
            unacknowledged: None,
            sink_error: None,
            roots: vec![],
            execution_trace_complete_verified: false,
            total_operation_count_verified: false,
            work_scope: WORK_SCOPE.into(),
        },
        irr: field.native(),
        sink,
        next_inverse: 0,
    };
    engine.report.stop = match engine.run() {
        Ok(stop) | Err(stop) => stop,
    };
    Ok(engine.report)
}

// Independent verifier arithmetic: portable bit-polynomial product/reduction,
// simple exponentiation and Euclidean polynomial operations. No native field
// type, producer inverse, root search, or Artin--Schreier solver is invoked.
fn vmul(a: u64, mut b: u64, field: FieldSpec) -> u64 {
    let mut product = 0u128;
    let mut shift = 0u32;
    while b != 0 {
        if b & 1 != 0 {
            product ^= (a as u128) << shift;
        }
        b >>= 1;
        shift += 1;
    }
    while product != 0 && 127 - product.leading_zeros() >= field.degree {
        product ^= (field.modulus as u128) << (127 - product.leading_zeros() - field.degree);
    }
    product as u64
}
fn vpow(mut a: u64, mut exponent: u64, field: FieldSpec) -> u64 {
    let mut value = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = vmul(value, a, field);
        }
        exponent >>= 1;
        if exponent != 0 {
            a = vmul(a, a, field);
        }
    }
    value
}
fn bit_remainder(mut a: u64, b: u64) -> u64 {
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}
fn bit_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, bit_remainder(a, b));
    }
    a
}
fn irreducible(field: FieldSpec) -> bool {
    let mut x = 2u64;
    for i in 1..=field.degree {
        x = vmul(x, x, field);
        if i <= field.degree / 2 && bit_gcd(x ^ 2, field.modulus) != 1 {
            return false;
        }
    }
    x == 2
}
fn vproduct(a: &[u64], b: &[u64], field: FieldSpec) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] ^= vmul(x, y, field);
        }
    }
    trim(out)
}
fn vremainder(a: &[u64], b: &[u64], field: FieldSpec) -> Vec<u64> {
    let mut r = trim(a.to_vec());
    let inverse = vpow(
        *b.last().expect("nonzero polynomial divisor"),
        field.order() - 2,
        field,
    );
    while !r.is_empty() && r.len() >= b.len() {
        let shift = r.len() - b.len();
        let scale = vmul(*r.last().unwrap(), inverse, field);
        for (i, &coefficient) in b.iter().enumerate() {
            r[shift + i] ^= vmul(scale, coefficient, field);
        }
        r = trim(r);
    }
    r
}
fn vgcd(mut a: Vec<u64>, mut b: Vec<u64>, field: FieldSpec) -> Vec<u64> {
    while !b.is_empty() {
        let r = vremainder(&a, &b, field);
        a = b;
        b = r;
    }
    if let Some(&leading) = a.last() {
        let inverse = vpow(leading, field.order() - 2, field);
        for coefficient in &mut a {
            *coefficient = vmul(*coefficient, inverse, field);
        }
    }
    a
}
fn veval(f: &[u64], x: u64, field: FieldSpec) -> u64 {
    f.iter()
        .rev()
        .fold(0, |value, &coefficient| vmul(value, x, field) ^ coefficient)
}
fn canonical_words(words: &[u64], field: FieldSpec) -> Result<(), String> {
    if words.iter().any(|&word| word >= field.order()) {
        return Err("noncanonical binary field word".into());
    }
    Ok(())
}
fn verify_roots(
    f: &[u64],
    roots: &[u64],
    all_field_elements: bool,
    field: FieldSpec,
) -> Result<(), String> {
    canonical_words(roots, field)?;
    if roots.len() > 2 || roots.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("binary root list must be sorted and distinct".into());
    }
    if f.is_empty() {
        return if all_field_elements && roots.is_empty() {
            Ok(())
        } else {
            Err("zero polynomial requires all-field certificate".into())
        };
    }
    if all_field_elements {
        return Err("nonzero polynomial cannot have all-field certificate".into());
    }
    let mut xq = vremainder(&[0, 1], f, field);
    for _ in 0..field.degree {
        xq = vremainder(&vproduct(&xq, &xq, field), f, field);
    }
    xq.resize(xq.len().max(2), 0);
    xq[1] ^= 1;
    let rational_part = vgcd(f.to_vec(), trim(xq), field);
    let mut product = vec![1];
    for &root in roots {
        product = vproduct(&product, &[root, 1], field);
    }
    if rational_part != product {
        return Err("binary distinct-root gcd certificate differs".into());
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct SumState {
    argument: u64,
    next: u32,
    power: u64,
    sum: u64,
}
fn sum_step(
    state: &mut Option<SumState>,
    index: u32,
    power: u64,
    sum: u64,
    squares: u32,
    field: FieldSpec,
) -> Result<(), String> {
    let state = state.as_mut().ok_or("sum step without begin")?;
    if index != state.next {
        return Err("sum step index differs".into());
    }
    let mut expected_power = state.power;
    if index != 0 {
        for _ in 0..squares {
            expected_power = vmul(expected_power, expected_power, field);
        }
    }
    let expected_sum = if index == 0 {
        state.argument
    } else {
        state.sum ^ expected_power
    };
    if power != expected_power || sum != expected_sum {
        return Err("trace/half-trace state identity differs".into());
    }
    state.power = power;
    state.sum = sum;
    state.next += 1;
    Ok(())
}

/// Check supplied event identities/tallies and completed distinct-root sets.
/// This deliberately does not certify completeness of the execution transcript.
pub fn verify(report: &RootReport) -> Result<(), String> {
    validate_input(&report.polynomial, report.field, report.limits)?;
    if report.schema != "binary-root-trace/v1"
        || trim(report.polynomial.clone()) != report.polynomial
        || report.events.len() as u64 + u64::from(report.unacknowledged.is_some())
            > report.limits.events
        || report.roots.len() > 2
        || report.work_scope != WORK_SCOPE
        || report.execution_trace_complete_verified
        || report.total_operation_count_verified
    {
        return Err("invalid binary root report schema, bounds or claim scope".into());
    }
    if (report.stop == RootStop::SinkFailure)
        != (report.unacknowledged.is_some() && report.sink_error.is_some())
        || (report.stop != RootStop::SinkFailure
            && (report.unacknowledged.is_some() || report.sink_error.is_some()))
        || report
            .sink_error
            .as_ref()
            .is_some_and(|error| error.len() > MAX_SINK_ERROR_BYTES)
    {
        return Err("inconsistent binary failed-sink state".into());
    }
    canonical_words(&report.roots, report.field)?;
    let field = report.field;
    let f = &report.polynomial;
    let chosen = branch(f);
    let (c, b, a) = (
        f.first().copied().unwrap_or(0),
        f.get(1).copied().unwrap_or(0),
        f.get(2).copied().unwrap_or(0),
    );
    let argument = if chosen == Branch::Quadratic {
        Some(vmul(
            vmul(a, c, field),
            vpow(vmul(b, b, field), field.order() - 2, field),
            field,
        ))
    } else {
        None
    };
    let mut counts = RootCounters::default();
    let mut inverse: Option<(u64, u64, u64)> = None;
    let mut next_inverse = 0u64;
    let mut seen_branch = false;
    let mut seen_normalized = false;
    let mut trace = None;
    let mut trace_done = None;
    let mut half_trace = None;
    let mut half_done = false;
    let mut matrix_argument = None;
    let mut matrix_columns = 0u32;
    let mut pivots: Vec<(u64, u64)> = vec![];
    let mut matrix_residual = None;
    let mut square_root: Option<(u32, u64)> = None;
    let mut finished: Option<(Vec<u64>, bool, bool)> = None;
    for (index, event) in report
        .events
        .iter()
        .chain(report.unacknowledged.iter())
        .enumerate()
    {
        if finished.is_some() {
            return Err("events after binary root finish".into());
        }
        counts.observe(event);
        if index == 0
            && *event
                != (RootEvent::Begin {
                    field,
                    polynomial: f.clone(),
                })
        {
            return Err("binary root trace header differs".into());
        }
        match event {
            RootEvent::Begin {
                field: event_field,
                polynomial,
            } => {
                if index != 0 || *event_field != field || polynomial != f {
                    return Err("repeated or mismatched root header".into());
                }
            }
            RootEvent::Branch { branch } => {
                if seen_branch || *branch != chosen {
                    return Err("binary root branch differs".into());
                }
                seen_branch = true;
            }
            RootEvent::Field { op, a, b, value } => {
                canonical_words(&[*a, *b, *value], field)?;
                let expected = match op {
                    FieldOp::Add => *a ^ *b,
                    FieldOp::Mul => vmul(*a, *b, field),
                    FieldOp::Square => {
                        if a != b {
                            return Err("square event has unequal operands".into());
                        }
                        vmul(*a, *a, field)
                    }
                };
                if *value != expected {
                    return Err("binary field event identity differs".into());
                }
            }
            RootEvent::InverseBegin { id, value } => {
                canonical_words(&[*value], field)?;
                if inverse.is_some() || *id != next_inverse || *value == 0 {
                    return Err("invalid inverse begin".into());
                }
                inverse = Some((*id, *value, 0));
                next_inverse += 1;
            }
            RootEvent::InversePower {
                id,
                exponent,
                value,
            } => {
                let (active_id, base, previous) =
                    inverse.as_mut().ok_or("inverse power without begin")?;
                if *id != *active_id
                    || *exponent <= *previous
                    || *exponent > field.order() - 2
                    || *value != vpow(*base, *exponent, field)
                {
                    return Err("inverse power identity differs".into());
                }
                *previous = *exponent;
            }
            RootEvent::InverseEnd {
                id,
                value,
                inverse: result,
            } => {
                canonical_words(&[*value, *result], field)?;
                let (active_id, base, _) = inverse.take().ok_or("inverse end without begin")?;
                if *id != active_id || *value != base || vmul(*value, *result, field) != 1 {
                    return Err("inverse identity differs".into());
                }
            }
            RootEvent::Normalized { argument: value } => {
                if seen_normalized || Some(*value) != argument {
                    return Err("quadratic normalization differs".into());
                }
                seen_normalized = true;
            }
            RootEvent::TraceBegin { argument: value } => {
                if trace.is_some() || trace_done.is_some() || Some(*value) != argument {
                    return Err("invalid trace begin".into());
                }
                trace = Some(SumState {
                    argument: *value,
                    next: 0,
                    power: *value,
                    sum: *value,
                });
            }
            RootEvent::TraceStep { index, power, sum } => {
                if *index >= field.degree {
                    return Err("trace index exceeds degree".into());
                }
                sum_step(&mut trace, *index, *power, *sum, 1, field)?;
            }
            RootEvent::TraceEnd { value } => {
                let state = trace.take().ok_or("trace end without begin")?;
                if state.next != field.degree || *value != state.sum || *value > 1 {
                    return Err("trace completion differs".into());
                }
                trace_done = Some(*value);
            }
            RootEvent::HalfTraceBegin { argument: value } => {
                if field.degree.is_multiple_of(2)
                    || half_trace.is_some()
                    || half_done
                    || trace_done != Some(0)
                    || Some(*value) != argument
                {
                    return Err("invalid half-trace begin".into());
                }
                half_trace = Some(SumState {
                    argument: *value,
                    next: 0,
                    power: *value,
                    sum: *value,
                });
            }
            RootEvent::HalfTraceStep { index, power, sum } => {
                if *index > field.degree / 2 {
                    return Err("half-trace index exceeds degree".into());
                }
                sum_step(&mut half_trace, *index, *power, *sum, 2, field)?;
            }
            RootEvent::HalfTraceEnd { solution } => {
                let state = half_trace.take().ok_or("half-trace end without begin")?;
                if state.next != field.degree / 2 + 1
                    || *solution != state.sum
                    || vmul(*solution, *solution, field) ^ *solution != state.argument
                {
                    return Err("half-trace completion differs".into());
                }
                half_done = true;
            }
            RootEvent::MatrixBegin { argument: value } => {
                if !field.degree.is_multiple_of(2)
                    || matrix_argument.is_some()
                    || trace_done != Some(0)
                    || Some(*value) != argument
                {
                    return Err("invalid binary matrix begin".into());
                }
                matrix_argument = Some(*value);
            }
            RootEvent::MatrixColumn {
                index,
                basis,
                image,
            } => {
                if matrix_argument.is_none()
                    || matrix_residual.is_some()
                    || *index != matrix_columns
                    || *index >= field.degree
                    || *basis != 1u64 << *index
                    || *image != vmul(*basis, *basis, field) ^ *basis
                {
                    return Err("binary matrix column differs".into());
                }
                matrix_columns += 1;
            }
            RootEvent::MatrixBitTest {
                phase,
                column,
                value,
                pivot_image,
                matched,
            } => {
                canonical_words(&[*value, *pivot_image], field)?;
                verify_matrix_position(
                    *phase,
                    *column,
                    matrix_columns,
                    matrix_argument,
                    matrix_residual,
                    field,
                )?;
                if *pivot_image == 0
                    || !pivots.iter().any(|p| p.0 == *pivot_image)
                    || *matched != (*value & (1u64 << (63 - pivot_image.leading_zeros())) != 0)
                {
                    return Err("matrix pivot bit-test differs".into());
                }
            }
            RootEvent::MatrixWordXor { a, b, value } => {
                canonical_words(&[*a, *b, *value], field)?;
                if matrix_argument.is_none() || matrix_residual.is_some() || *value != *a ^ *b {
                    return Err("matrix XOR identity differs".into());
                }
            }
            RootEvent::MatrixReduction {
                phase,
                column,
                image_before,
                preimage_before,
                pivot_image,
                pivot_preimage,
                image,
                preimage,
            } => {
                canonical_words(
                    &[
                        *image_before,
                        *preimage_before,
                        *pivot_image,
                        *pivot_preimage,
                        *image,
                        *preimage,
                    ],
                    field,
                )?;
                verify_matrix_position(
                    *phase,
                    *column,
                    matrix_columns,
                    matrix_argument,
                    matrix_residual,
                    field,
                )?;
                let offset = if *phase == MatrixPhase::Target {
                    matrix_argument.unwrap()
                } else {
                    0
                };
                if !pivots.contains(&(*pivot_image, *pivot_preimage))
                    || *image != *image_before ^ *pivot_image
                    || *preimage != *preimage_before ^ *pivot_preimage
                    || vmul(*preimage_before, *preimage_before, field) ^ *preimage_before ^ offset
                        != *image_before
                    || vmul(*preimage, *preimage, field) ^ *preimage ^ offset != *image
                {
                    return Err("binary matrix reduction identity differs".into());
                }
            }
            RootEvent::MatrixPivot {
                column,
                image,
                preimage,
            } => {
                canonical_words(&[*image, *preimage], field)?;
                verify_matrix_position(
                    MatrixPhase::Column,
                    *column,
                    matrix_columns,
                    matrix_argument,
                    matrix_residual,
                    field,
                )?;
                if *image == 0
                    || vmul(*preimage, *preimage, field) ^ *preimage != *image
                    || pivots
                        .iter()
                        .any(|p| p.0.leading_zeros() == image.leading_zeros())
                {
                    return Err("binary matrix pivot differs".into());
                }
                pivots.push((*image, *preimage));
            }
            RootEvent::MatrixEnd {
                rank,
                solution,
                residual,
            } => {
                canonical_words(&[*solution, *residual], field)?;
                let rhs = matrix_argument.ok_or("matrix end without begin")?;
                if matrix_residual.is_some()
                    || matrix_columns != field.degree
                    || *rank != pivots.len() as u32
                    || *rank != field.degree - 1
                    || vmul(*solution, *solution, field) ^ *solution ^ rhs != *residual
                {
                    return Err("binary matrix completion differs".into());
                }
                matrix_residual = Some(*residual);
            }
            RootEvent::SquareRootStep { index, value } => {
                if chosen != Branch::PureSquare || *index >= field.degree {
                    return Err("invalid square-root step".into());
                }
                let expected = if *index == 0 && square_root.is_none() {
                    vmul(c, vpow(a, field.order() - 2, field), field)
                } else {
                    let (previous, prior) =
                        square_root.ok_or("square-root step without initial state")?;
                    if *index != previous + 1 {
                        return Err("square-root index differs".into());
                    }
                    vmul(prior, prior, field)
                };
                if *value != expected {
                    return Err("square-root Frobenius state differs".into());
                }
                square_root = Some((*index, *value));
            }
            RootEvent::Scale { value } => {
                if chosen != Branch::Quadratic
                    || *value != vmul(b, vpow(a, field.order() - 2, field), field)
                {
                    return Err("quadratic root scale differs".into());
                }
            }
            RootEvent::ExtractedRoot { root } => {
                canonical_words(&[*root], field)?;
                if veval(f, *root, field) != 0 {
                    return Err("extracted binary root is not a root".into());
                }
            }
            RootEvent::Finish {
                roots,
                all_field_elements,
            } => {
                if !seen_branch
                    || inverse.is_some()
                    || trace.is_some()
                    || half_trace.is_some()
                    || (matrix_argument.is_some() && matrix_residual.is_none())
                    || square_root.is_some_and(|(index, _)| index + 1 != field.degree)
                {
                    return Err("binary finish interrupts a supplied state sequence".into());
                }
                verify_roots(f, roots, *all_field_elements, field)?;
                finished = Some((
                    roots.clone(),
                    *all_field_elements,
                    index < report.events.len(),
                ));
            }
        }
    }
    if report.counters != counts || counts.work() > report.limits.work {
        return Err("binary root event tally differs or exceeds limit".into());
    }
    if report.complete() {
        let (roots, all, acknowledged) = finished.ok_or("completed binary report lacks finish")?;
        if !acknowledged
            || report.roots != roots
            || (report.stop == RootStop::AllFieldElements) != all
        {
            return Err("binary root completion differs".into());
        }
    } else {
        if !report.roots.is_empty() || finished.as_ref().is_some_and(|f| f.2) {
            return Err("incomplete binary report claims completed roots".into());
        }
        match report.stop {
            RootStop::WorkLimit if counts.work() != report.limits.work => {
                return Err("unjustified binary work stop".into())
            }
            RootStop::EventLimit if report.events.len() as u64 != report.limits.events => {
                return Err("unjustified binary event stop".into())
            }
            RootStop::LinearSystemFailure if matrix_residual.is_none_or(|r| r == 0) => {
                return Err("unjustified binary linear-system stop".into())
            }
            _ => {}
        }
    }
    Ok(())
}

fn verify_matrix_position(
    phase: MatrixPhase,
    column: u32,
    columns: u32,
    argument: Option<u64>,
    residual: Option<u64>,
    field: FieldSpec,
) -> Result<(), String> {
    if argument.is_none()
        || residual.is_some()
        || match phase {
            MatrixPhase::Column => columns == 0 || column >= field.degree || column + 1 != columns,
            MatrixPhase::Target => columns != field.degree || column != field.degree,
        }
    {
        return Err("invalid binary matrix phase or column".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> RootLimits {
        RootLimits {
            work: MAX_WORK,
            events: MAX_EVENTS,
        }
    }
    fn run(f: &[u64], field: FieldSpec) -> RootReport {
        let report = solve(f, field, limits(), &mut |_| Ok(())).unwrap();
        verify(&report).unwrap();
        report
    }
    fn trace(value: u64, field: FieldSpec) -> u64 {
        let (mut power, mut sum) = (value, value);
        for _ in 1..field.degree {
            power = vmul(power, power, field);
            sum ^= power;
        }
        sum
    }

    #[test]
    fn exhaustive_small_odd_even_quadratics_match_enumeration() {
        for field in [
            FieldSpec {
                degree: 3,
                modulus: 0b1011,
            },
            FieldSpec {
                degree: 4,
                modulus: 0b10011,
            },
        ] {
            for a in 0..field.order() {
                for b in 0..field.order() {
                    for c in 0..field.order() {
                        let polynomial = [c, b, a];
                        let report = run(&polynomial, field);
                        assert!(report.complete());
                        if polynomial == [0, 0, 0] {
                            assert_eq!(report.stop, RootStop::AllFieldElements);
                            assert!(report.roots.is_empty());
                        } else {
                            let expected: Vec<_> = (0..field.order())
                                .filter(|&x| veval(&polynomial, x, field) == 0)
                                .collect();
                            assert_eq!(report.stop, RootStop::Complete);
                            assert_eq!(report.roots, expected, "field={field:?}, f={polynomial:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn requested_degrees_have_constructed_and_trace_one_controls() {
        use crate::cryptanalysis::koblitz_index_calculus::find_irreducible_sparse;

        for degree in (23..=31).chain(47..=61) {
            let irr = find_irreducible_sparse(degree).expect("requested field modulus");
            let field = FieldSpec {
                degree,
                modulus: irr
                    .low_terms
                    .iter()
                    .fold(1u64 << degree, |mask, bit| mask | (1u64 << *bit)),
            };
            let (left, right) = ((1u64 << (degree - 1)) | 3, 7u64);
            let polynomial = [vmul(left, right, field), left ^ right, 1];
            let report = run(&polynomial, field);
            assert_eq!(report.roots, vec![right, left]);
            assert_eq!(report.stop, RootStop::Complete);
            assert!(report
                .events
                .iter()
                .any(|event| matches!(event, RootEvent::InversePower { .. })));
            if degree.is_multiple_of(2) {
                assert!(report.events.iter().any(|event| matches!(event, RootEvent::MatrixEnd { rank, residual: 0, .. } if *rank == degree - 1)));
            } else {
                assert!(report
                    .events
                    .iter()
                    .any(|event| matches!(event, RootEvent::HalfTraceEnd { .. })));
            }
            // A nonzero trace functional is one on some polynomial-basis vector.
            let c = (0..degree)
                .map(|bit| 1u64 << bit)
                .find(|&value| trace(value, field) == 1)
                .unwrap();
            let no_roots = run(&[c, 1, 1], field);
            assert_eq!(no_roots.stop, RootStop::Complete);
            assert!(no_roots.roots.is_empty());
            assert!(no_roots
                .events
                .iter()
                .any(|event| matches!(event, RootEvent::TraceEnd { value: 1 })));
        }
    }

    #[test]
    fn linear_pure_square_repeated_and_zero_cases_are_explicit() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        assert_eq!(run(&[7, 1], field).roots, vec![7]);
        let root = 9;
        let square = vmul(root, root, field);
        let report = run(&[square, 0, 1], field);
        assert_eq!(report.roots, vec![root]);
        assert_eq!(
            report
                .events
                .iter()
                .filter(|e| matches!(e, RootEvent::SquareRootStep { .. }))
                .count(),
            field.degree as usize
        );
        assert_eq!(run(&[0, 0, 1], field).roots, vec![0]);
        assert!(run(&[1], field).roots.is_empty());
        let zero = run(&[], field);
        assert_eq!(zero.stop, RootStop::AllFieldElements);
        assert!(
            matches!(zero.events.last(), Some(RootEvent::Finish { all_field_elements: true, roots }) if roots.is_empty())
        );
        let mut bad = zero.clone();
        bad.stop = RootStop::Complete;
        assert!(verify(&bad).is_err());
    }

    #[test]
    fn legacy_quadratic_roots_and_inverse_primitive_counts_match() {
        use crate::cryptanalysis::binary_semaev::solve_quadratic_f2m;

        for field in [
            FieldSpec {
                degree: 3,
                modulus: 0b1011,
            },
            FieldSpec {
                degree: 4,
                modulus: 0b10011,
            },
        ] {
            for polynomial in [[2, 3, 1], [1, 1, 1], [7, 1, 0], [5, 0, 3]] {
                let report = run(&polynomial, field);
                let irr = field.native();
                let element = |value| F2mElement::from_words(&[value], field.degree);
                let mut legacy: Vec<_> = solve_quadratic_f2m(
                    &element(polynomial[2]),
                    &element(polynomial[1]),
                    &element(polynomial[0]),
                    field.degree,
                    &irr,
                )
                .iter()
                .map(|root| root.raw_bits()[0])
                .collect();
                legacy.sort_unstable();
                legacy.dedup();
                assert_eq!(report.roots, legacy);
                let mut inverse_squares = None;
                for event in &report.events {
                    match event {
                        RootEvent::InverseBegin { .. } => inverse_squares = Some(0),
                        RootEvent::Field {
                            op: FieldOp::Square,
                            ..
                        } => {
                            if let Some(count) = inverse_squares.as_mut() {
                                *count += 1;
                            }
                        }
                        RootEvent::InverseEnd { .. } => {
                            assert_eq!(inverse_squares.take(), Some(field.degree - 1))
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    #[test]
    fn exact_work_and_event_caps_preserve_censored_prefixes() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        let f = [2, 3, 1];
        let full = run(&f, field);
        for cap in 0..full.counters.work() {
            let report = solve(
                &f,
                field,
                RootLimits {
                    work: cap,
                    events: MAX_EVENTS,
                },
                &mut |_| Ok(()),
            )
            .unwrap();
            assert_eq!(report.stop, RootStop::WorkLimit);
            assert_eq!(report.counters.work(), cap);
            assert!(report.roots.is_empty());
            assert!(!report.complete());
            verify(&report).unwrap();
            assert_eq!(report.events, full.events[..report.events.len()]);
        }
        let exact_work = solve(
            &f,
            field,
            RootLimits {
                work: full.counters.work(),
                events: MAX_EVENTS,
            },
            &mut |_| Ok(()),
        )
        .unwrap();
        assert_eq!(exact_work.stop, RootStop::Complete);
        verify(&exact_work).unwrap();
        for cap in 0..full.events.len() as u64 {
            let report = solve(
                &f,
                field,
                RootLimits {
                    work: MAX_WORK,
                    events: cap,
                },
                &mut |_| Ok(()),
            )
            .unwrap();
            assert_eq!(report.stop, RootStop::EventLimit);
            assert_eq!(report.events.len() as u64, cap);
            assert_eq!(report.events, full.events[..cap as usize]);
            assert!(report.roots.is_empty());
            verify(&report).unwrap();
        }
        let exact_events = solve(
            &f,
            field,
            RootLimits {
                work: MAX_WORK,
                events: full.events.len() as u64,
            },
            &mut |_| Ok(()),
        )
        .unwrap();
        assert_eq!(exact_events.stop, RootStop::Complete);
        verify(&exact_events).unwrap();
    }

    #[test]
    fn sink_failure_stops_at_each_attempted_event_including_finish() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        let f = [2, 3, 1];
        let full = run(&f, field);
        for failed_at in 0..full.events.len() {
            let mut calls = 0usize;
            let report = solve(&f, field, limits(), &mut |_| {
                let fail = calls == failed_at;
                calls += 1;
                if fail {
                    Err("fixture sink failed".into())
                } else {
                    Ok(())
                }
            })
            .unwrap();
            assert_eq!(report.stop, RootStop::SinkFailure);
            assert_eq!(calls, failed_at + 1);
            assert_eq!(report.events, full.events[..failed_at]);
            assert_eq!(report.unacknowledged, Some(full.events[failed_at].clone()));
            assert!(report.roots.is_empty());
            assert!(!report.complete());
            verify(&report).unwrap();
        }
        let report = solve(&f, field, limits(), &mut |_| {
            Err("é".repeat(MAX_SINK_ERROR_BYTES))
        })
        .unwrap();
        assert_eq!(
            report.sink_error.as_ref().unwrap().len(),
            MAX_SINK_ERROR_BYTES
        );
        verify(&report).unwrap();
    }

    #[test]
    fn rejects_noncanonical_fields_coefficients_limits_and_record_shapes() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        for invalid in [
            FieldSpec {
                degree: 2,
                modulus: 0b111,
            },
            FieldSpec {
                degree: 62,
                modulus: (1u64 << 62) | 3,
            },
            FieldSpec {
                degree: 64,
                modulus: 3,
            },
            FieldSpec {
                degree: 4,
                modulus: 0b10101,
            },
            FieldSpec {
                degree: 4,
                modulus: 0b10010,
            },
            FieldSpec {
                degree: 4,
                modulus: 0b110011,
            },
        ] {
            assert!(solve(&[1, 1], invalid, limits(), &mut |_| Ok(())).is_err());
        }
        assert!(solve(&[16, 1], field, limits(), &mut |_| Ok(())).is_err());
        assert!(solve(&[1, 1, 1, 0], field, limits(), &mut |_| Ok(())).is_err());
        assert!(solve(
            &[1, 1],
            field,
            RootLimits {
                work: MAX_WORK + 1,
                events: 1
            },
            &mut |_| Ok(())
        )
        .is_err());
        assert!(solve(
            &[1, 1],
            field,
            RootLimits {
                work: 1,
                events: MAX_EVENTS + 1
            },
            &mut |_| Ok(())
        )
        .is_err());
        assert!(serde_json::from_value::<FieldSpec>(
            serde_json::json!({"degree":4,"modulus":19,"ignored":true})
        )
        .is_err());
        assert!(serde_json::from_value::<RootLimits>(
            serde_json::json!({"work":1,"events":2,"ignored":true})
        )
        .is_err());
        assert!(serde_json::from_value::<RootEvent>(
            serde_json::json!({"kind":"branch","branch":"linear","ignored":true})
        )
        .is_err());
    }

    #[test]
    fn certificate_and_event_tampering_is_rejected() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        let full = run(&[2, 3, 1], field);
        let mut missing_root = full.clone();
        missing_root.roots.pop();
        if let Some(RootEvent::Finish { roots, .. }) = missing_root.events.last_mut() {
            roots.pop();
        }
        assert!(verify(&missing_root).is_err());
        let mut duplicate = full.clone();
        duplicate.roots = vec![1, 1];
        if let Some(RootEvent::Finish { roots, .. }) = duplicate.events.last_mut() {
            *roots = vec![1, 1];
        }
        assert!(verify(&duplicate).is_err());
        let mut wrong_modulus = full.clone();
        wrong_modulus.field.modulus ^= 2;
        assert!(verify(&wrong_modulus).is_err());
        let mut wrong_coefficients = full.clone();
        wrong_coefficients.polynomial[0] ^= 1;
        assert!(verify(&wrong_coefficients).is_err());
        let mut wrong_counter = full.clone();
        wrong_counter.counters.squares += 1;
        assert!(verify(&wrong_counter).is_err());
        let mut wrong_scope = full.clone();
        wrong_scope.work_scope.push_str(" plus total cost");
        assert!(verify(&wrong_scope).is_err());
        for target in ["field", "trace", "matrix", "reduction", "inverse"] {
            let mut bad = full.clone();
            let mut changed = false;
            for event in &mut bad.events {
                match (target, event) {
                    ("field", RootEvent::Field { value, .. })
                    | ("inverse", RootEvent::InversePower { value, .. }) => {
                        *value ^= 1;
                        changed = true;
                    }
                    ("trace", RootEvent::TraceStep { sum, .. }) => {
                        *sum ^= 1;
                        changed = true;
                    }
                    ("matrix", RootEvent::MatrixColumn { image, .. })
                    | ("reduction", RootEvent::MatrixReduction { image, .. }) => {
                        *image ^= 1;
                        changed = true;
                    }
                    _ => {}
                }
                if changed {
                    break;
                }
            }
            assert!(changed, "missing fixture event: {target}");
            assert!(verify(&bad).is_err(), "accepted {target} tamper");
        }
        let mut extra = full.clone();
        extra.events.push(full.events[0].clone());
        assert!(verify(&extra).is_err());
        let mut censored = solve(
            &[2, 3, 1],
            field,
            RootLimits {
                work: 1,
                events: 1000,
            },
            &mut |_| Ok(()),
        )
        .unwrap();
        censored.stop = RootStop::Complete;
        assert!(verify(&censored).is_err());
    }

    #[test]
    fn root_completeness_does_not_certify_complete_execution_or_total_cost() {
        let field = FieldSpec {
            degree: 4,
            modulus: 0b10011,
        };
        let mut report = run(&[2, 3, 1], field);
        report.events.retain(|event| {
            !matches!(
                event,
                RootEvent::Field { .. }
                    | RootEvent::MatrixBitTest { .. }
                    | RootEvent::MatrixWordXor { .. }
            )
        });
        report.counters = RootCounters::default();
        for event in &report.events {
            report.counters.observe(event);
        }
        verify(&report).unwrap();
        assert!(report.complete());
        assert!(!report.execution_trace_complete_verified);
        assert!(!report.total_operation_count_verified);
        report.execution_trace_complete_verified = true;
        assert!(verify(&report).is_err());
        report.execution_trace_complete_verified = false;
        report.total_operation_count_verified = true;
        assert!(verify(&report).is_err());
    }
}
