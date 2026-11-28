//! Topological Fp gates. Simplification uses only public constants and wire IDs.
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::{collections::BTreeMap, io::Write};
use torus_s4::Field;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wire(pub u32);
#[derive(Clone, Copy, Debug)]
pub struct Gate {
    pub op: u32, // 0 constant, 1 semantic input, 2 add, 3 negate, 4 multiply
    pub a: u32,
    pub b: u32,
}
pub struct Builder {
    pub f: Field,
    pub gates: Vec<Gate>,
    pub constants: Vec<BigUint>,
    constant_ids: BTreeMap<BigUint, Wire>,
    pub constraints: Vec<(Wire, u32)>,
    pub inputs: u32,
    pub max_nodes: usize,
    pub refusal: Option<String>,
}
impl Builder {
    pub fn new(f: Field, max_nodes: usize) -> Result<Self> {
        if !(16..=16_000_000).contains(&max_nodes) {
            return Err("node limit outside16..16000000".into());
        }
        let mut b = Self {
            f,
            gates: vec![],
            constants: vec![],
            constant_ids: BTreeMap::new(),
            constraints: vec![],
            inputs: 0,
            max_nodes,
            refusal: None,
        };
        for i in 0u32..5 {
            b.constant(i.into());
        }
        Ok(b)
    }
    fn push(&mut self, gate: Gate) -> Wire {
        if self.refusal.is_some() {
            return Wire(0);
        }
        if self.gates.len() >= self.max_nodes {
            self.refusal = Some("node cap; incomplete compilation".into());
            return Wire(0);
        }
        let id = Wire(self.gates.len() as u32);
        self.gates.push(gate);
        id
    }
    pub fn constant(&mut self, v: BigUint) -> Wire {
        let v = v % &self.f.p;
        if let Some(w) = self.constant_ids.get(&v) {
            return *w;
        }
        let i = self.constants.len() as u32;
        let w = self.push(Gate { op: 0, a: i, b: 0 });
        if self.refusal.is_none() {
            self.constants.push(v.clone());
            self.constant_ids.insert(v, w);
        }
        w
    }
    pub fn input(&mut self) -> Wire {
        let w = self.push(Gate {
            op: 1,
            a: self.inputs,
            b: 0,
        });
        self.inputs += 1;
        w
    }
    pub fn value(&self, w: Wire) -> Option<&BigUint> {
        let g = self.gates.get(w.0 as usize)?;
        (g.op == 0).then(|| &self.constants[g.a as usize])
    }
    pub fn add(&mut self, a: Wire, b: Wire) -> Wire {
        if a.0 == 0 {
            return b;
        }
        if b.0 == 0 {
            return a;
        }
        if let (Some(x), Some(y)) = (self.value(a), self.value(b)) {
            return self.constant(self.f.add(x, y));
        }
        self.push(Gate {
            op: 2,
            a: a.0,
            b: b.0,
        })
    }
    pub fn neg(&mut self, a: Wire) -> Wire {
        if let Some(x) = self.value(a) {
            return self.constant(self.f.sub(&0u32.into(), x));
        }
        if self.gates[a.0 as usize].op == 3 {
            return Wire(self.gates[a.0 as usize].a);
        }
        self.push(Gate {
            op: 3,
            a: a.0,
            b: 0,
        })
    }
    pub fn sub(&mut self, a: Wire, b: Wire) -> Wire {
        if a == b {
            return Wire(0);
        }
        let nb = self.neg(b);
        self.add(a, nb)
    }
    pub fn mul(&mut self, a: Wire, b: Wire) -> Wire {
        if a.0 == 0 || b.0 == 0 {
            return Wire(0);
        }
        if a.0 == 1 {
            return b;
        }
        if b.0 == 1 {
            return a;
        }
        if let (Some(x), Some(y)) = (self.value(a), self.value(b)) {
            return self.constant(self.f.mul(x, y));
        }
        self.push(Gate {
            op: 4,
            a: a.0,
            b: b.0,
        })
    }
    pub fn constrain(&mut self, a: Wire, want: u32) {
        self.constraints.push((a, want));
    }
    pub fn finish(self) -> Result<Program> {
        if let Some(e) = self.refusal {
            return Err(e);
        }
        Ok(Program {
            f: self.f,
            gates: self.gates,
            constants: self.constants,
            constraints: self.constraints,
            inputs: self.inputs,
        })
    }
}
pub struct Program {
    pub f: Field,
    pub gates: Vec<Gate>,
    pub constants: Vec<BigUint>,
    pub constraints: Vec<(Wire, u32)>,
    pub inputs: u32,
}
impl Program {
    pub fn validate(&self) -> Result<()> {
        if self.f.p.bits() > 256
            || self.f.p < BigUint::from(5u32)
            || !self.f.p.bit(0)
            || self.gates.len() > 16_000_000
            || self.inputs > 7
            || self.constants.iter().any(|x| x >= &self.f.p)
        {
            return Err("malformed program limits/constants".into());
        }
        let mut seen = vec![false; self.inputs as usize];
        for (i, g) in self.gates.iter().enumerate() {
            let valid = match g.op {
                0 => (g.a as usize) < self.constants.len() && g.b == 0,
                1 => {
                    let valid = (g.a as usize) < seen.len() && g.b == 0 && !seen[g.a as usize];
                    if valid {
                        seen[g.a as usize] = true;
                    }
                    valid
                }
                2 | 4 => (g.a as usize) < i && (g.b as usize) < i,
                3 => (g.a as usize) < i && g.b == 0,
                _ => false,
            };
            if !valid {
                return Err(format!("malformed gate{i}"));
            }
        }
        if seen.contains(&false)
            || self
                .constraints
                .iter()
                .any(|(w, c)| w.0 as usize >= self.gates.len() || *c > 1)
        {
            return Err("malformed inputs/output constraints".into());
        }
        Ok(())
    }
    fn expected(&self, g: &Gate, values: &[BigUint], inputs: &[BigUint]) -> Result<BigUint> {
        Ok(match g.op {
            0 => self
                .constants
                .get(g.a as usize)
                .ok_or("bad constant")?
                .clone(),
            1 => inputs.get(g.a as usize).ok_or("missing input")?.clone(),
            2 => self.f.add(&values[g.a as usize], &values[g.b as usize]),
            3 => self.f.sub(&0u32.into(), &values[g.a as usize]),
            4 => self.f.mul(&values[g.a as usize], &values[g.b as usize]),
            _ => return Err("bad gate operation".into()),
        })
    }
    pub fn evaluate_prefix(&self, inputs: &[BigUint], end: usize) -> Result<Vec<BigUint>> {
        self.validate()?;
        if inputs.len() != self.inputs as usize
            || inputs.iter().any(|x| x >= &self.f.p)
            || end > self.gates.len()
        {
            return Err("noncanonical/malformed inputs or prefix".into());
        }
        let mut values = Vec::with_capacity(end);
        for g in self.gates.iter().take(end) {
            values.push(self.expected(g, &values, inputs)?);
        }
        Ok(values)
    }
    pub fn check(&self, values: &[BigUint]) -> Result<()> {
        self.validate()?;
        if values.len() != self.gates.len() || values.iter().any(|x| x >= &self.f.p) {
            return Err("truncated/noncanonical assignment".into());
        }
        let mut inputs = vec![BigUint::zero(); self.inputs as usize];
        for (g, v) in self.gates.iter().zip(values) {
            if g.op == 1 {
                inputs[g.a as usize] = v.clone();
            }
        }
        for (i, g) in self.gates.iter().enumerate() {
            if self.expected(g, values, &inputs)? != values[i] {
                return Err(format!("gate{i} mismatch"));
            }
        }
        for (w, want) in &self.constraints {
            if values[w.0 as usize] != BigUint::from(*want) {
                return Err(format!("constraint{} mismatch", w.0));
            }
        }
        Ok(())
    }
    pub fn outputs_hold(&self, values: &[BigUint]) -> bool {
        values.len() == self.gates.len()
            && self
                .constraints
                .iter()
                .all(|(w, c)| values[w.0 as usize] == BigUint::from(*c))
    }
    /// Sparse quadratic rows, including every constant, gate and output guard.
    /// Input gates are free variables. Streaming avoids a second whole system.
    pub fn equations(&self) -> impl Iterator<Item = Vec<(BigUint, Vec<u32>)>> + '_ {
        self.gates
            .iter()
            .enumerate()
            .filter(|(_, g)| g.op != 1)
            .map(|(i, g)| {
                let minus = &self.f.p - 1u32;
                let mut terms = vec![(BigUint::one(), vec![i as u32])];
                match g.op {
                    0 => terms.push((
                        self.f.sub(&0u32.into(), &self.constants[g.a as usize]),
                        vec![],
                    )),
                    2 => {
                        terms.push((minus.clone(), vec![g.a]));
                        terms.push((minus, vec![g.b]));
                    }
                    3 => terms.push((BigUint::one(), vec![g.a])),
                    4 => {
                        let mut m = vec![g.a, g.b];
                        m.sort_unstable();
                        terms.push((minus, m));
                    }
                    _ => unreachable!("validate before lowering"),
                }
                terms
            })
            .chain(self.constraints.iter().map(|(w, c)| {
                vec![
                    (BigUint::one(), vec![w.0]),
                    (self.f.sub(&0u32.into(), &BigUint::from(*c)), vec![]),
                ]
            }))
    }
    pub fn stats(&self) -> Value {
        let mut counts = [0usize; 5];
        for g in &self.gates {
            counts[g.op as usize] += 1;
        }
        json!({"variables":self.gates.len(),"inputs":self.inputs,"constant_gates":counts[0],
            "add_gates":counts[2],"negate_gates":counts[3],"multiply_gates":counts[4],
            "equations":self.gates.len()-counts[1]+self.constraints.len(),"output_constraints":self.constraints.len(),
            "max_equation_degree":2,"scope":"representation counts, not solving degree or complete cold cost"})
    }
    /// Complete canonical little-endian TCG1 gate/constraint export, no omitted gates.
    pub fn write(&self, out: &mut impl Write) -> Result<()> {
        self.validate()?;
        let write = |o: &mut dyn Write, b: &[u8]| o.write_all(b).map_err(|e| e.to_string());
        write(out, b"TCG1")?;
        let mut p = self.f.p.to_bytes_le();
        p.resize(32, 0);
        write(out, &p)?;
        for n in [
            self.inputs as u64,
            self.constants.len() as u64,
            self.gates.len() as u64,
            self.constraints.len() as u64,
        ] {
            write(out, &n.to_le_bytes())?;
        }
        for c in &self.constants {
            let mut v = c.to_bytes_le();
            v.resize(32, 0);
            write(out, &v)?;
        }
        for g in &self.gates {
            for v in [g.op, g.a, g.b] {
                write(out, &v.to_le_bytes())?;
            }
        }
        for (w, c) in &self.constraints {
            write(out, &w.0.to_le_bytes())?;
            write(out, &c.to_le_bytes())?;
        }
        Ok(())
    }
    pub fn write_witness(&self, values: &[BigUint], out: &mut impl Write) -> Result<()> {
        self.check(values)?;
        for v in values {
            let mut b = v.to_bytes_le();
            b.resize(32, 0);
            out.write_all(&b).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_gate_and_corruption_controls() -> Result<()> {
        let mut b = Builder::new(Field::new(101u32.into())?, 100)?;
        let x = b.input();
        let inv = b.input();
        let sq = b.mul(x, x);
        let a = b.add(sq, x);
        let guard = b.mul(x, inv);
        b.constrain(guard, 1);
        b.constrain(a, 0);
        let p = b.finish()?;
        let mut v = p.evaluate_prefix(&[100u32.into(), 100u32.into()], p.gates.len())?;
        p.check(&v)?;
        for i in 0..v.len() {
            let old = v[i].clone();
            v[i] = p.f.add(&old, &BigUint::one());
            assert!(p.check(&v).is_err());
            v[i] = old;
        }
        assert!(p.check(&v[..v.len() - 1]).is_err());
        v[0] = 101u32.into();
        assert!(p.check(&v).is_err());
        assert!(p
            .evaluate_prefix(&[101u32.into(), 0u32.into()], p.gates.len())
            .is_err());
        let mut p = p;
        p.gates[0] = Gate { op: 2, a: 0, b: 0 };
        assert!(p.validate().is_err());
        assert!(p
            .evaluate_prefix(&[100u32.into(), 100u32.into()], p.gates.len())
            .is_err());
        Ok(())
    }
}
