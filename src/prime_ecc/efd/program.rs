//! Straight-line EFD programs: parse, cost and evaluate over F_p.
use super::*;
use num_bigint::BigUint;
use num_traits::One;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub(super) enum Expr {
    Constant(BigUint),
    Variable(String),
    Neg(Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, u32),
}

impl Expr {
    pub(super) fn parse(s: &str) -> Result<Self> {
        let mut p = Parser {
            text: s.as_bytes(),
            pos: 0,
        };
        let e = p.sum()?;
        p.space();
        if p.pos != p.text.len() {
            return Err(EfdError::Syntax(s.into()));
        }
        Ok(e)
    }
    pub(super) fn eval(&self, modulus: &BigUint, values: &Bindings) -> Result<FieldElement> {
        Ok(match self {
            Self::Constant(n) => FieldElement::new(n.clone(), modulus.clone()),
            Self::Variable(v) => values
                .get(v)
                .cloned()
                .ok_or_else(|| EfdError::MissingInput(v.clone()))?,
            Self::Neg(a) => a.eval(modulus, values)?.neg(),
            Self::Add(a, b) => a.eval(modulus, values)?.add(&b.eval(modulus, values)?),
            Self::Sub(a, b) => a.eval(modulus, values)?.sub(&b.eval(modulus, values)?),
            Self::Mul(a, b) => a.eval(modulus, values)?.mul(&b.eval(modulus, values)?),
            Self::Div(a, b) => {
                let inverse = b
                    .eval(modulus, values)?
                    .inv()
                    .ok_or(EfdError::ZeroDenominator)?;
                if matches!(**a, Self::Constant(ref n) if n == &BigUint::one()) {
                    inverse
                } else {
                    a.eval(modulus, values)?.mul(&inverse)
                }
            }
            Self::Pow(a, n) => {
                let base = a.eval(modulus, values)?;
                let mut out = FieldElement::one(modulus.clone());
                let mut acc = base;
                let mut n = *n;
                while n > 0 {
                    if n & 1 == 1 {
                        out = out.mul(&acc);
                    }
                    n >>= 1;
                    if n > 0 {
                        acc = acc.mul(&acc);
                    }
                }
                out
            }
        })
    }
    fn variables(&self, out: &mut BTreeSet<String>) {
        match self {
            Self::Variable(v) => {
                out.insert(v.clone());
            }
            Self::Neg(a) | Self::Pow(a, _) => a.variables(out),
            Self::Add(a, b) | Self::Sub(a, b) | Self::Mul(a, b) | Self::Div(a, b) => {
                a.variables(out);
                b.variables(out);
            }
            Self::Constant(_) => {}
        }
    }
    fn renamed(&self, suffix: u32) -> Self {
        match self {
            Self::Variable(v) => Self::Variable(format!("{v}{suffix}")),
            Self::Neg(a) => Self::Neg(Box::new(a.renamed(suffix))),
            Self::Add(a, b) => Self::Add(Box::new(a.renamed(suffix)), Box::new(b.renamed(suffix))),
            Self::Sub(a, b) => Self::Sub(Box::new(a.renamed(suffix)), Box::new(b.renamed(suffix))),
            Self::Mul(a, b) => Self::Mul(Box::new(a.renamed(suffix)), Box::new(b.renamed(suffix))),
            Self::Div(a, b) => Self::Div(Box::new(a.renamed(suffix)), Box::new(b.renamed(suffix))),
            Self::Pow(a, n) => Self::Pow(Box::new(a.renamed(suffix)), *n),
            Self::Constant(n) => Self::Constant(n.clone()),
        }
    }
    fn count(&self, c: &mut OperationCounts) {
        match self {
            Self::Add(a, b) | Self::Sub(a, b) => {
                a.count(c);
                b.count(c);
                c.additions += 1;
            }
            Self::Mul(a, b) => {
                a.count(c);
                b.count(c);
                c.multiplications += 1;
            }
            Self::Div(a, b) => {
                a.count(c);
                b.count(c);
                c.inversions += 1;
                if !matches!(**a, Self::Constant(ref n) if n == &BigUint::one()) {
                    c.multiplications += 1;
                }
            }
            Self::Pow(a, n) => {
                a.count(c);
                if *n > 0 {
                    c.squarings += 31 - n.leading_zeros();
                    c.multiplications += n.count_ones() - 1;
                }
            }
            Self::Neg(a) => a.count(c),
            _ => {}
        }
    }
}

struct Parser<'a> {
    text: &'a [u8],
    pos: usize,
}
impl Parser<'_> {
    fn space(&mut self) {
        while self.text.get(self.pos).is_some_and(u8::is_ascii_whitespace) {
            self.pos += 1;
        }
    }
    fn take(&mut self, c: u8) -> bool {
        self.space();
        if self.text.get(self.pos) == Some(&c) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn sum(&mut self) -> Result<Expr> {
        let mut a = self.product()?;
        loop {
            if self.take(b'+') {
                a = Expr::Add(Box::new(a), Box::new(self.product()?));
            } else if self.take(b'-') {
                a = Expr::Sub(Box::new(a), Box::new(self.product()?));
            } else {
                break;
            }
        }
        Ok(a)
    }
    /// Unary minus binds looser than exponentiation, as in the EFD records:
    /// `-a^2` is `-(a^2)`, not `(-a)^2`.
    fn unary(&mut self) -> Result<Expr> {
        if self.take(b'-') {
            return Ok(Expr::Neg(Box::new(self.unary()?)));
        }
        self.power()
    }
    fn product(&mut self) -> Result<Expr> {
        let mut a = self.unary()?;
        loop {
            if self.take(b'*') {
                a = Expr::Mul(Box::new(a), Box::new(self.unary()?));
            } else if self.take(b'/') {
                a = Expr::Div(Box::new(a), Box::new(self.unary()?));
            } else {
                self.space();
                if self
                    .text
                    .get(self.pos)
                    .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'(' || *c == b'_')
                {
                    a = Expr::Mul(Box::new(a), Box::new(self.unary()?));
                } else {
                    break;
                }
            }
        }
        Ok(a)
    }
    fn power(&mut self) -> Result<Expr> {
        let a = self.atom()?;
        if self.take(b'^') {
            self.space();
            let start = self.pos;
            while self.text.get(self.pos).is_some_and(u8::is_ascii_digit) {
                self.pos += 1;
            }
            let n = std::str::from_utf8(&self.text[start..self.pos])
                .unwrap()
                .parse::<u32>()
                .map_err(|_| EfdError::Syntax("invalid exponent".into()))?;
            if n > 65536 {
                return Err(EfdError::Syntax("exponent limit".into()));
            }
            Ok(Expr::Pow(Box::new(a), n))
        } else {
            Ok(a)
        }
    }
    fn atom(&mut self) -> Result<Expr> {
        self.space();
        if self.take(b'(') {
            let e = self.sum()?;
            if !self.take(b')') {
                return Err(EfdError::Syntax("missing )".into()));
            }
            return Ok(e);
        }
        let start = self.pos;
        while self
            .text
            .get(self.pos)
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            self.pos += 1;
        }
        if self.pos == start {
            return Err(EfdError::Syntax(format!(
                "expected expression at byte {}",
                self.pos
            )));
        }
        let token = std::str::from_utf8(&self.text[start..self.pos]).unwrap();
        // EFD names include a few identifiers that start with a digit, such as
        // `2d` and `3overd`. A run of digits is a constant; anything else is a
        // variable.
        if token.bytes().all(|c| c.is_ascii_digit()) {
            Ok(Expr::Constant(token.parse::<BigUint>().map_err(|_| {
                EfdError::Syntax("integer overflow".into())
            })?))
        } else {
            Ok(Expr::Variable(token.into()))
        }
    }
}

/// Raw field operations in the straight-line program, including parameter
/// multiplications. Excludes parameter preparation, validation, point conversion
/// and walk selection. These counts are not timings or complete rho-step costs.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationCounts {
    pub multiplications: u32,
    pub squarings: u32,
    pub inversions: u32,
    pub additions: u32,
}

pub struct Program {
    source: ProgramSource,
    instructions: Vec<(String, Expr)>,
    assumptions: Vec<(Expr, Expr)>,
    parameters: BTreeSet<String>,
    invariants: Vec<(Expr, Expr)>,
}

pub(super) fn equality(s: &str) -> Result<(Expr, Expr)> {
    let (a, b) = s
        .split_once('=')
        .ok_or_else(|| EfdError::Syntax(s.into()))?;
    Ok((Expr::parse(a)?, Expr::parse(b)?))
}

/// Provenance of the straight-line program being executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgramSource {
    /// The optimized `.op3` program served by EFD.
    Upstream,
    /// A program reconstructed from the pinned `data/` record. Used only when
    /// the upstream `.op3` is incomplete or contradicts its own record; its
    /// operation counts are record costs, not EFD's optimized counts.
    RecordFallback,
}

fn instruction(line: &str) -> Result<(String, Expr)> {
    let (a, b) = line
        .split_once('=')
        .ok_or_else(|| EfdError::Syntax(line.into()))?;
    let name = a.trim();
    if !matches!(Expr::parse(name)?, Expr::Variable(_)) {
        return Err(EfdError::Syntax(line.into()));
    }
    Ok((name.into(), Expr::parse(b)?))
}

impl Program {
    pub(super) fn compile(formula: &Formula) -> Result<Self> {
        let mut instructions = Vec::new();
        for line in formula
            .program
            .text
            .lines()
            .filter(|l| !l.trim().is_empty())
        {
            // A handful of upstream `.op3` files contain the EFD generator's
            // own failure notice instead of code. Keep the record and the
            // source hash pinned, and refuse to execute rather than guess.
            if line.trim_start().starts_with("error:") {
                return Err(EfdError::Unsupported(format!(
                    "{}: upstream program incomplete: {}",
                    formula.id,
                    line.trim()
                )));
            }
            instructions.push(instruction(line)?);
        }
        Self::assemble(formula, instructions, ProgramSource::Upstream)
    }

    /// Reconstruct a straight-line program from the `compute NAME = EXPR`
    /// lines of the pinned record. Every EFD record states its computation, so
    /// this makes the eight unusable upstream programs executable without
    /// changing the pinned bytes.
    pub(super) fn compile_record(formula: &Formula) -> Result<Self> {
        let mut instructions = Vec::new();
        for line in formula.record.text.lines() {
            let Some(body) = line.strip_prefix("compute ") else {
                continue;
            };
            instructions.push(instruction(body)?);
        }
        if instructions.is_empty() {
            return Err(EfdError::Unsupported(format!(
                "{}: record has no explicit computation",
                formula.id
            )));
        }
        Self::assemble(formula, instructions, ProgramSource::RecordFallback)
    }

    fn assemble(
        formula: &Formula,
        instructions: Vec<(String, Expr)>,
        source: ProgramSource,
    ) -> Result<Self> {
        let catalogue = catalogue();
        let rep = catalogue
            .representation(&formula.representation)
            .ok_or_else(|| EfdError::Unsupported(formula.representation.clone()))?;
        let model = catalogue
            .representation_model(&formula.representation)
            .ok_or_else(|| EfdError::Unsupported(formula.representation.clone()))?;
        let parameters = model
            .source
            .text
            .lines()
            .chain(rep.source.text.lines())
            .chain(formula.record.text.lines())
            .filter_map(|l| l.strip_prefix("parameter "))
            .map(|s| s.trim().to_owned())
            .collect();
        let assumptions = rep
            .assumptions
            .iter()
            .chain(&formula.assumptions)
            .map(|s| equality(s))
            .collect::<Result<_>>()?;
        let mut invariants = Vec::new();
        for relation in &rep.relations {
            let pair = equality(relation)?;
            let mut vars = BTreeSet::new();
            pair.0.variables(&mut vars);
            pair.1.variables(&mut vars);
            if vars.iter().all(|v| rep.variables.contains(v)) {
                invariants.push(pair);
            }
        }
        Ok(Self {
            source,
            instructions,
            assumptions,
            parameters,
            invariants,
        })
    }

    pub fn source(&self) -> ProgramSource {
        self.source
    }

    pub fn operation_counts(&self) -> OperationCounts {
        let mut c = OperationCounts::default();
        for (_, e) in &self.instructions {
            e.count(&mut c);
        }
        c
    }
    /// Names read before assignment, including parameters required by assumptions.
    /// Derived cache parameters may instead be prepared from their definitions.
    pub fn required_inputs(&self) -> BTreeSet<String> {
        let mut assigned = BTreeSet::new();
        let mut out = BTreeSet::new();
        for (a, b) in &self.assumptions {
            a.variables(&mut out);
            b.variables(&mut out);
        }
        for (name, e) in &self.instructions {
            let mut vars = BTreeSet::new();
            e.variables(&mut vars);
            out.extend(vars.difference(&assigned).cloned());
            assigned.insert(name.clone());
        }
        out
    }
    fn derive(&self, modulus: &BigUint, values: &mut Bindings) -> Result<()> {
        // Only declared parameters are derived, never missing point coordinates.
        for _ in 0..self.assumptions.len() + 1 {
            let before = values.len();
            for (a, b) in &self.assumptions {
                for (left, right) in [(a, b), (b, a)] {
                    if let Ok(value) = right.eval(modulus, values) {
                        if let Some((name, value)) =
                            solve(left, value, modulus, values, &self.parameters)
                        {
                            values.insert(name, value);
                        }
                    }
                }
            }
            if values.len() == before {
                break;
            }
        }
        Ok(())
    }
    pub fn evaluate(&self, modulus: &BigUint, inputs: &Bindings) -> Result<Bindings> {
        check_field(modulus, inputs)?;
        let mut values = inputs.clone();
        self.derive(modulus, &mut values)?;
        for (a, b) in &self.assumptions {
            if a.eval(modulus, &values)? != b.eval(modulus, &values)? {
                return Err(EfdError::Assumption(format!("{a:?} = {b:?}")));
            }
        }
        for index in 1..=3 {
            for (a, b) in &self.invariants {
                let (a, b) = (a.renamed(index), b.renamed(index));
                let mut vars = BTreeSet::new();
                a.variables(&mut vars);
                b.variables(&mut vars);
                if vars.iter().all(|v| values.contains_key(v))
                    && a.eval(modulus, &values)? != b.eval(modulus, &values)?
                {
                    return Err(EfdError::Assumption(format!(
                        "inconsistent coordinate cache at input {index}"
                    )));
                }
            }
        }
        for (name, e) in &self.instructions {
            let v = e.eval(modulus, &values)?;
            values.insert(name.clone(), v);
        }
        Ok(values)
    }
}

/// Solve `target = value` for one unknown declared parameter. Supports a bare
/// variable, a product with a known factor (recursively), and a square (via a
/// square root).
fn solve(
    target: &Expr,
    value: FieldElement,
    modulus: &BigUint,
    values: &Bindings,
    parameters: &BTreeSet<String>,
) -> Option<(String, FieldElement)> {
    let unknown = |v: &str| parameters.contains(v) && !values.contains_key(v) && !v.is_empty();
    match target {
        Expr::Variable(v) if unknown(v) => Some((v.to_owned(), value)),
        Expr::Pow(inner, 2) => {
            let root = super::sqrt(&value)?;
            match &**inner {
                Expr::Variable(v) if unknown(v) => Some((v.to_owned(), root)),
                other => solve(other, root, modulus, values, parameters),
            }
        }
        Expr::Add(a, b) => {
            if let Ok(term) = b.eval(modulus, values) {
                if let Some(found) = solve(a, value.sub(&term), modulus, values, parameters) {
                    return Some(found);
                }
            }
            if let Ok(term) = a.eval(modulus, values) {
                if let Some(found) = solve(b, value.sub(&term), modulus, values, parameters) {
                    return Some(found);
                }
            }
            None
        }
        Expr::Mul(a, b) => {
            if let Ok(factor) = b.eval(modulus, values) {
                if let Some(inverse) = factor.inv() {
                    return solve(a, value.mul(&inverse), modulus, values, parameters);
                }
            }
            if let Ok(factor) = a.eval(modulus, values) {
                if let Some(inverse) = factor.inv() {
                    return solve(b, value.mul(&inverse), modulus, values, parameters);
                }
            }
            None
        }
        _ => None,
    }
}
