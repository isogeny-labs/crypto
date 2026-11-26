//! Separate spectrum-v1 IR; the frozen seven-input compiler stays unchanged.
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{cell::RefCell, collections::BTreeMap, io::Write};
use torus_s4::Field;

pub type W = usize;
#[derive(Clone, Debug)]
pub enum Gate {
    Constant(BigUint),
    Input(usize),
    Add(W, W),
    Neg(W),
    Mul(W, W),
}
pub struct Program {
    pub f: Field,
    pub gates: Vec<Gate>,
    pub inputs: usize,
    pub constraints: Vec<(W, W)>,
    pub bitsums: Vec<(W, Vec<W>)>,
}
pub struct Builder {
    pub program: Program,
    constants: BTreeMap<BigUint, W>,
    cap: usize,
    refusal: bool,
}
pub struct E {
    pub f: Field,
    pub b: RefCell<Builder>,
}
impl E {
    pub fn new(p: BigUint, cap: usize) -> Result<Self> {
        if !(16..=1_000_000).contains(&cap) || p.bits() > 256 {
            return Err("spectrum IR node/modulus admission".into());
        }
        let f = Field::new(p)?;
        let e = Self {
            f: f.clone(),
            b: RefCell::new(Builder {
                program: Program {
                    f,
                    gates: vec![],
                    inputs: 0,
                    constraints: vec![],
                    bitsums: vec![],
                },
                constants: BTreeMap::new(),
                cap,
                refusal: false,
            }),
        };
        e.c(0);
        e.c(1);
        Ok(e)
    }
    fn push(&self, gate: Gate) -> W {
        let mut b = self.b.borrow_mut();
        if b.refusal || b.program.gates.len() >= b.cap {
            b.refusal = true;
            return 0;
        }
        let w = b.program.gates.len();
        b.program.gates.push(gate);
        w
    }
    fn value(&self, w: W) -> Option<BigUint> {
        match self.b.borrow().program.gates.get(w) {
            Some(Gate::Constant(v)) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn constant(&self, n: BigUint) -> W {
        let n = n % &self.f.p;
        if let Some(w) = self.b.borrow().constants.get(&n) {
            return *w;
        }
        let w = self.push(Gate::Constant(n.clone()));
        if !self.b.borrow().refusal {
            self.b.borrow_mut().constants.insert(n, w);
        }
        w
    }
    pub fn c(&self, n: u64) -> W {
        self.constant(n.into())
    }
    pub fn input(&self) -> W {
        let id = self.b.borrow().program.inputs;
        let w = self.push(Gate::Input(id));
        self.b.borrow_mut().program.inputs += 1;
        w
    }
    pub fn add(&self, a: W, b: W) -> W {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        if let (Some(x), Some(y)) = (self.value(a), self.value(b)) {
            return self.constant(self.f.add(&x, &y));
        }
        self.push(Gate::Add(a, b))
    }
    pub fn neg(&self, a: W) -> W {
        if let Some(x) = self.value(a) {
            return self.constant(self.f.sub(&0u32.into(), &x));
        }
        self.push(Gate::Neg(a))
    }
    pub fn sub(&self, a: W, b: W) -> W {
        if a == b {
            return 0;
        }
        self.add(a, self.neg(b))
    }
    pub fn mul(&self, a: W, b: W) -> W {
        if a == 0 || b == 0 {
            return 0;
        }
        if a == 1 {
            return b;
        }
        if b == 1 {
            return a;
        }
        if let (Some(x), Some(y)) = (self.value(a), self.value(b)) {
            return self.constant(self.f.mul(&x, &y));
        }
        self.push(Gate::Mul(a, b))
    }
    pub fn equal(&self, a: W, b: W) {
        self.b.borrow_mut().program.constraints.push((a, b));
    }
    pub fn word(&self, bits: &[W]) -> W {
        let mut w = 0;
        for (i, b) in bits.iter().enumerate() {
            self.equal(self.mul(*b, self.sub(*b, 1)), 0);
            w = self.add(w, self.mul(self.constant(BigUint::one() << i), *b));
        }
        self.b.borrow_mut().program.bitsums.push((w, bits.to_vec()));
        w
    }
    pub fn finish(self) -> Result<Program> {
        let b = self.b.into_inner();
        if b.refusal {
            return Err("node cap; incomplete compilation".into());
        }
        b.program.validate()?;
        Ok(b.program)
    }
}
impl Program {
    pub fn validate(&self) -> Result<()> {
        if self.gates.len() > 1_000_000
            || self.inputs > 256
            || self.f.p.bits() > 256
            || !matches!(self.gates.first(),Some(Gate::Constant(x)) if x.is_zero())
            || !matches!(self.gates.get(1),Some(Gate::Constant(x)) if x.is_one())
        {
            return Err("spectrum program limits".into());
        }
        let mut inputs = vec![false; self.inputs];
        for (i, g) in self.gates.iter().enumerate() {
            let valid = match g {
                Gate::Constant(v) => v < &self.f.p,
                Gate::Input(j) => {
                    let ok = *j < inputs.len() && !inputs[*j];
                    if ok {
                        inputs[*j] = true;
                    }
                    ok
                }
                Gate::Add(a, b) | Gate::Mul(a, b) => *a < i && *b < i,
                Gate::Neg(a) => *a < i,
            };
            if !valid {
                return Err(format!("malformed spectrum gate{i}"));
            }
        }
        if inputs.contains(&false)
            || self
                .constraints
                .iter()
                .any(|(a, b)| *a >= self.gates.len() || *b >= self.gates.len())
            || self.bitsums.iter().any(|(w, bs)| {
                *w >= self.gates.len()
                    || bs.is_empty()
                    || bs.len() > 16
                    || bs.iter().any(|b| *b >= self.gates.len())
            })
        {
            return Err("malformed spectrum inputs/constraints/bitsums".into());
        }
        Ok(())
    }
    pub fn evaluate(&self, inputs: &[BigUint]) -> Result<Vec<BigUint>> {
        self.validate()?;
        if inputs.len() != self.inputs || inputs.iter().any(|x| x >= &self.f.p) {
            return Err("noncanonical/truncated inputs".into());
        }
        let mut vs = Vec::with_capacity(self.gates.len());
        for g in &self.gates {
            vs.push(self.expected(g, &vs, inputs));
        }
        Ok(vs)
    }
    fn expected(&self, g: &Gate, vs: &[BigUint], inputs: &[BigUint]) -> BigUint {
        match g {
            Gate::Constant(x) => x.clone(),
            Gate::Input(i) => inputs[*i].clone(),
            Gate::Add(a, b) => self.f.add(&vs[*a], &vs[*b]),
            Gate::Neg(a) => self.f.sub(&BigUint::zero(), &vs[*a]),
            Gate::Mul(a, b) => self.f.mul(&vs[*a], &vs[*b]),
        }
    }
    pub fn check(&self, vs: &[BigUint]) -> Result<()> {
        self.validate()?;
        if vs.len() != self.gates.len() || vs.iter().any(|v| v >= &self.f.p) {
            return Err("truncated/noncanonical assignment".into());
        }
        let mut inputs = vec![BigUint::zero(); self.inputs];
        for (g, v) in self.gates.iter().zip(vs) {
            if let Gate::Input(i) = g {
                inputs[*i] = v.clone();
            }
        }
        for (i, g) in self.gates.iter().enumerate() {
            if self.expected(g, vs, &inputs) != vs[i] {
                return Err(format!("spectrum gate{i} mismatch"));
            }
        }
        if self.constraints.iter().any(|(a, b)| vs[*a] != vs[*b]) {
            return Err("spectrum output constraint".into());
        }
        Ok(())
    }
    pub fn stats(&self) -> Value {
        let muls = self
            .gates
            .iter()
            .filter(|g| matches!(g, Gate::Mul(..)))
            .count();
        json!({"variables":self.gates.len(),"inputs":self.inputs,"equations":self.gates.len()-self.inputs+self.constraints.len(),"multiply_gates":muls,"constraints":self.constraints.len(),"bitsum_annotations":self.bitsums.len(),"max_equation_degree":2,"scope":"representation only; no solving-degree or cold-work claim"})
    }
    pub fn smt(&self, out: &mut impl Write) -> Result<()> {
        self.validate()?;
        let emit =
            |o: &mut dyn Write, s: String| o.write_all(s.as_bytes()).map_err(|e| e.to_string());
        emit(out,format!("(set-logic QF_FF)\n(set-option :produce-models true)\n(define-sort F () (_ FiniteField {}))\n",self.f.p))?;
        for i in 0..self.gates.len() {
            emit(out, format!("(declare-const w{i} F)\n"))?;
        }
        for (i, g) in self.gates.iter().enumerate() {
            let rhs = match g {
                Gate::Input(_) => continue,
                Gate::Constant(v) => format!("(as ff{v} F)"),
                Gate::Add(a, b) => format!("(ff.add w{a} w{b})"),
                Gate::Neg(a) => format!("(ff.mul (as ff{} F) w{a})", &self.f.p - 1u32),
                Gate::Mul(a, b) => format!("(ff.mul w{a} w{b})"),
            };
            emit(out, format!("(assert (= w{i} {rhs}))\n"))?;
        }
        for (a, b) in &self.constraints {
            emit(out, format!("(assert (= w{a} w{b}))\n"))?;
        }
        // Redundant, exact annotations expose bit sums to the pinned solver.
        for (w, bs) in &self.bitsums {
            let term = if bs.len() == 1 {
                format!("w{}", bs[0])
            } else {
                format!(
                    "(ff.bitsum {})",
                    bs.iter()
                        .map(|b| format!("w{b}"))
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            };
            emit(out, format!("(assert (= w{w} {term}))\n"))?;
        }
        emit(out, "(check-sat)\n".into())
    }
}
