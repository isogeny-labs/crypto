//! Guarded linear coefficient elimination, regular chart only; NOT a solver.
pub mod controls;
pub mod sparse;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use sparse::{det, Limits, E, P};
use torus_coset_circuit::{
    aggregation::{shifted, TargetCircuit},
    Circuit, Spec,
};
use torus_s4::{Curve, Field, Point};
pub type Result<T> = std::result::Result<T, String>;
const FREE: [usize; 5] = [0, 1, 2, 3, 5];

pub struct System {
    pub variables: Vec<String>,
    pub equations: Vec<P>,
    f: Field,
}
impl System {
    pub fn check(&self, v: &[BigUint]) -> Result<()> {
        if v.len() != self.variables.len() || v.iter().any(|x| x >= &self.f.p) {
            return Err("noncanonical/incomplete dynamic assignment".into());
        }
        for (i, p) in self.equations.iter().enumerate() {
            if !p.evaluate(&self.f, v)?.is_zero() {
                return Err(format!("dynamic equation{i} failed"));
            }
        }
        Ok(())
    }
    pub fn stats(&self) -> Value {
        json!({"variables":self.variables.len(),"equations":self.equations.len(),"terms":self.equations.iter().map(|p|p.0.len()).sum::<usize>(),"max_degree":self.equations.iter().map(P::degree).max().unwrap_or(0)})
    }
    pub fn encoded(&self) -> Value {
        json!({"p":self.f.p.to_string(),"variables":self.variables,"equations":self.equations.iter().map(P::encoded).collect::<Vec<_>>()})
    }
}
pub struct Block {
    pub(crate) augmented: System,
    binding: String,
    e: E,
    rows: Vec<Vec<P>>,
    guards: Vec<P>,
    prefix: usize,
    original_n: usize,
    original_rows: usize,
    raw: Vec<(P, P)>,
    r: Point,
    curve: Curve,
    point_query: torus_point_solver::PointQuery,
}
pub struct Chart {
    pub(crate) system: System,
    binding: String,
    pub omit: usize,
    d: P,
    n: Vec<P>,
    pub expansion_pairs: u64,
}
impl Block {
    pub fn compile(s: &Spec, r: Point, q: Point, limits: Limits) -> Result<Self> {
        let binding = format!(
            "{}:{}:{}:{}:{}:{}:{:?}:{r:?}:{q:?}",
            s.p, s.h.a, s.h.b, s.order_factor, s.q, s.k, s.curve_a
        );
        let point_query = torus_point_solver::compile(s, r.clone(), q.clone())?;
        let (rx, ry) = r.as_ref().ok_or("UNSUPPORTED identity raw target")?;
        if ry.is_zero() {
            return Err("UNSUPPORTED finite two-torsion target".into());
        }
        let e = E::new(s.p.clone(), limits)?;
        let original = TargetCircuit::compile(s, 6, q, true)?;
        let member = Circuit::compile(s)?;
        let n = member.system.variables.len();
        let raw0 = member
            .raw_point_polys()
            .ok_or("missing original raw coordinates")?;
        let mut raw = vec![];
        for i in 0..6 {
            raw.push((
                e.original(&shifted(&raw0.0, i * n))?,
                e.original(&shifted(&raw0.1, i * n))?,
            ));
        }
        let original_n = original.system.variables.len();
        let original_rows = original.system.equations.len();
        let mut system = System {
            variables: original.system.variables.clone(),
            equations: original
                .system
                .equations
                .iter()
                .map(|p| e.original(p))
                .collect::<Result<_>>()?,
            f: e.f.clone(),
        };
        let mut rows = vec![];
        let mut guards = vec![];
        for (x, y) in &raw {
            let d = e.sub(x, &e.c(rx.clone()))?;
            guards.push(d.clone());
            rows.push(vec![
                d.clone(),
                e.mul(x, &d)?,
                e.mul(y, &d)?,
                e.mul(&e.mul(x, x)?, &d)?,
                e.mul(&e.mul(x, y)?, &d)?,
                e.add(y, &e.c(ry.clone()))?,
            ]);
        }
        for i in 0..6 {
            for j in i + 1..6 {
                guards.push(e.sub(&raw[i].0, &raw[j].0)?);
            }
        }
        for (i, g) in guards.iter().enumerate() {
            let inv = e.var(system.variables.len())?;
            system.variables.push(format!("regular_guard{i}"));
            system
                .equations
                .push(e.sub(&e.mul(g, &inv)?, &e.c(BigUint::one()))?);
        }
        let prefix = system.variables.len();
        let cs = FREE
            .iter()
            .map(|i| {
                let w = system.variables.len();
                system.variables.push(format!("coefficient{i}"));
                e.var(w)
            })
            .collect::<Result<Vec<_>>>()?;
        for row in &rows {
            let mut p = row[4].clone();
            for (j, col) in FREE.iter().enumerate() {
                p = e.add(&p, &e.mul(&row[*col], &cs[j])?)?;
            }
            system.equations.push(p);
        }
        let curve = Curve {
            f: e.f.clone(),
            a: s.curve_a.clone().ok_or("missing curve A")?,
        };
        Ok(Self {
            binding,
            augmented: system,
            e,
            rows,
            guards,
            prefix,
            original_n,
            original_rows,
            raw,
            r,
            curve,
            point_query,
        })
    }
    pub fn chart(&self, omit: usize) -> Result<Chart> {
        if omit >= 6 {
            return Err("omitted row outside six choices".into());
        }
        let start = self.e.pairs();
        let ids = (0..6).filter(|i| *i != omit).collect::<Vec<_>>();
        let a = ids
            .iter()
            .map(|i| {
                FREE.iter()
                    .map(|j| self.rows[*i][*j].clone())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let b = ids
            .iter()
            .map(|i| self.e.neg(&self.rows[*i][4]))
            .collect::<Vec<_>>();
        let d = det(&self.e, &a)?;
        let mut n = vec![];
        for col in 0..5 {
            let mut m = a.clone();
            for row in 0..5 {
                m[row][col] = b[row].clone();
            }
            n.push(det(&self.e, &m)?);
        }
        // Deterministic sparse coefficient cancellation, not random evaluation.
        // This checks the five selected-row Cramer identities at compilation.
        for row in 0..5 {
            let mut identity = self.e.neg(&self.e.mul(&b[row], &d)?);
            for col in 0..5 {
                identity = self.e.add(&identity, &self.e.mul(&a[row][col], &n[col])?)?;
            }
            if !identity.0.is_empty() {
                return Err("symbolic selected-row identity failed".into());
            }
        }
        let mut system = System {
            variables: self.augmented.variables[..self.prefix].to_vec(),
            equations: self.augmented.equations[..self.original_rows + self.guards.len()].to_vec(),
            f: self.e.f.clone(),
        };
        let t = self.e.var(system.variables.len())?;
        system.variables.push("pivot_minor_inverse".into());
        system.equations.push(
            self.e
                .sub(&self.e.mul(&d, &t)?, &self.e.c(BigUint::one()))?,
        );
        let mut residual = self.e.mul(&self.rows[omit][4], &d)?;
        for (j, col) in FREE.iter().enumerate() {
            residual = self
                .e
                .add(&residual, &self.e.mul(&self.rows[omit][*col], &n[j])?)?;
        }
        system.equations.push(residual);
        Ok(Chart {
            binding: self.binding.clone(),
            system,
            omit,
            d,
            n,
            expansion_pairs: self.e.pairs() - start,
        })
    }
    pub fn extend(&self, original: &[BigUint], cs: &[BigUint]) -> Result<Vec<BigUint>> {
        if original.len() != self.original_n
            || cs.len() != 6
            || cs[4] != BigUint::one()
            || cs.iter().any(|x| x >= &self.e.f.p)
        {
            return Err("coefficient/original shape or normalization".into());
        }
        let mut v = original.to_vec();
        for g in &self.guards {
            v.push(
                self.e
                    .f
                    .inv(&g.evaluate(&self.e.f, original)?)
                    .map_err(|_| "UNSUPPORTED regular-chart boundary")?,
            );
        }
        v.extend(FREE.iter().map(|i| cs[*i].clone()));
        self.check_augmented(&v)?;
        Ok(v)
    }
    pub fn check_augmented(&self, v: &[BigUint]) -> Result<Value> {
        self.augmented.check(v)?;
        let original = self.point_query.check_values(&v[..self.original_n])?;
        let mut sum = None;
        for (x, y) in &self.raw {
            sum = self.curve.add(
                &sum,
                &Some((x.evaluate(&self.e.f, v)?, y.evaluate(&self.e.f, v)?)),
            )?;
        }
        if sum != self.r {
            return Err("raw-R sector replay failed".into());
        }
        Ok(json!({"original":original,"raw_R_replay":true}))
    }
    pub fn lift(&self, c: &Chart, augmented: &[BigUint]) -> Result<Vec<BigUint>> {
        if c.binding != self.binding {
            return Err("chart belongs to a different public problem".into());
        }
        self.augmented.check(augmented)?;
        let mut v = augmented[..self.prefix].to_vec();
        let inverse = self
            .e
            .f
            .inv(&c.d.evaluate(&self.e.f, &v)?)
            .map_err(|_| "zero pivot minor: augmented fallback remains available")?;
        v.push(inverse);
        c.system.check(&v)?;
        Ok(v)
    }
    pub fn project(&self, c: &Chart, reduced: &[BigUint]) -> Result<Vec<BigUint>> {
        if c.binding != self.binding {
            return Err("chart belongs to a different public problem".into());
        }
        c.system.check(reduced)?;
        let t = reduced.last().ok_or("missing pivot inverse")?;
        let mut v = reduced[..self.prefix].to_vec();
        for n in &c.n {
            v.push(self.e.f.mul(t, &n.evaluate(&self.e.f, reduced)?));
        }
        self.check_augmented(&v)?;
        Ok(v)
    }
    pub fn raw_points(&self, v: &[BigUint]) -> Result<Vec<Point>> {
        self.raw
            .iter()
            .map(|(x, y)| Ok(Some((x.evaluate(&self.e.f, v)?, y.evaluate(&self.e.f, v)?))))
            .collect()
    }
    pub fn original_count(&self) -> usize {
        self.original_n
    }
    pub fn augmented_system(&self) -> &System {
        &self.augmented
    }
    pub fn coefficients(&self, v: &[BigUint]) -> Result<Vec<BigUint>> {
        if v.len() != self.augmented.variables.len() {
            return Err("augmented length".into());
        }
        let mut cs = vec![BigUint::zero(); 6];
        cs[4] = BigUint::one();
        for (i, col) in FREE.iter().enumerate() {
            cs[*col] = v[self.prefix + i].clone();
        }
        Ok(cs)
    }
}
impl Chart {
    pub fn system(&self) -> &System {
        &self.system
    }
}
