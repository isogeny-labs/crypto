//! Bounded public-input exact reference, using classical representative families.
//! Its explicit half enumeration is cubic for six members, not gamma<2.
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use torus_coset_circuit::aggregation::TargetCircuit;
use torus_coset_circuit::index::{negate, IndexTable};
use torus_coset_circuit::{Circuit, Spec, WitnessInput};
use torus_s4::{Curve, Field, Point};
use torus_s4_tree::Fp2;

pub mod controls;
pub mod determinant;
pub type Result<T> = std::result::Result<T, String>;
const RAW_CAP: usize = 64;
const HALF_CAP: usize = 131072;

#[derive(Clone, Default, Serialize)]
pub struct Events {
    pub half_tuples: usize,
    pub curve_add_calls: usize,
    pub minor_vectors: usize,
    pub basis_insertions: usize,
    pub dictionary_queries: usize,
    pub representative_candidates: usize,
}
#[derive(Clone)]
struct Column {
    raw_id: usize,
    j: u64,
    t: u64,
    input: WitnessInput,
    point: Point,
}
struct Base {
    spec: Spec,
    curve: Curve,
    raw_count: usize,
    columns: Vec<Column>,
    excluded: Vec<(u64, u64)>,
}
#[derive(Clone)]
struct Half {
    columns: Vec<usize>,
    signs: usize,
    mask: u64,
    point: Point,
}
#[derive(Clone)]
struct BasisRow {
    pivot: usize,
    reduced: Vec<BigUint>,
    original: Half,
}
#[derive(Default)]
struct Family {
    rows: Vec<BasisRow>,
}

fn choose(n: usize, k: usize) -> usize {
    if n < k {
        return 0;
    }
    (0..k).fold(1, |a, i| a * (n - i) / (i + 1))
}
fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    fn visit(n: usize, k: usize, start: usize, p: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if p.len() == k {
            out.push(p.clone());
            return;
        }
        for i in start..n {
            p.push(i);
            visit(n, k, i + 1, p, out);
            p.pop();
        }
    }
    let mut out = vec![];
    visit(n, k, 0, &mut vec![], &mut out);
    out
}
fn det(matrix: &[Vec<BigUint>], f: &Field) -> BigUint {
    match matrix.len() {
        1 => matrix[0][0].clone(),
        2 => f.sub(
            &f.mul(&matrix[0][0], &matrix[1][1]),
            &f.mul(&matrix[0][1], &matrix[1][0]),
        ),
        3 => {
            let m = matrix;
            let a = f.mul(
                &m[0][0],
                &f.sub(&f.mul(&m[1][1], &m[2][2]), &f.mul(&m[1][2], &m[2][1])),
            );
            let b = f.mul(
                &m[0][1],
                &f.sub(&f.mul(&m[1][0], &m[2][2]), &f.mul(&m[1][2], &m[2][0])),
            );
            let c = f.mul(
                &m[0][2],
                &f.sub(&f.mul(&m[1][0], &m[2][1]), &f.mul(&m[1][1], &m[2][0])),
            );
            f.add(&f.sub(&a, &b), &c)
        }
        _ => unreachable!("fixed half arity <=3"),
    }
}
fn wedge(ids: &[usize], r: usize, f: &Field) -> Vec<BigUint> {
    let powers: Vec<Vec<_>> = ids
        .iter()
        .map(|id| {
            let x = BigUint::from(id + 1);
            let mut v = vec![BigUint::one()];
            for i in 1..2 * r {
                v.push(f.mul(&v[i - 1], &x));
            }
            v
        })
        .collect();
    combinations(2 * r, r)
        .iter()
        .map(|rows| {
            let m: Vec<Vec<_>> = rows
                .iter()
                .map(|row| powers.iter().map(|v| v[*row].clone()).collect())
                .collect();
            det(&m, f)
        })
        .collect()
}
impl Family {
    fn insert(&mut self, mut v: Vec<BigUint>, original: Half, f: &Field) -> Result<bool> {
        for row in &self.rows {
            let c = v[row.pivot].clone();
            if c.is_zero() {
                continue;
            }
            for (a, b) in v.iter_mut().zip(&row.reduced) {
                *a = f.sub(a, &f.mul(&c, b));
            }
        }
        let Some(pivot) = v.iter().position(|x| !x.is_zero()) else {
            return Ok(false);
        };
        let inv = f.inv(&v[pivot])?;
        for x in &mut v {
            *x = f.mul(x, &inv);
        }
        let pos = self.rows.partition_point(|row| row.pivot < pivot);
        self.rows.insert(
            pos,
            BasisRow {
                pivot,
                reduced: v,
                original,
            },
        );
        Ok(true)
    }
    fn disjoint(&self, mask: u64) -> Option<&Half> {
        self.rows
            .iter()
            .map(|r| &r.original)
            .find(|h| h.mask & mask == 0)
    }
}
impl Base {
    fn new(spec: &Spec) -> Result<Self> {
        let raw = spec
            .k
            .checked_mul(spec.q)
            .ok_or("raw cardinality overflow")?;
        if raw > RAW_CAP as u64 {
            return Err("raw-column cap exceeded before enumeration".into());
        }
        if spec.p <= BigUint::from(raw) {
            return Err("Vandermonde labels are not distinct in this field".into());
        }
        Circuit::compile(spec)?;
        let f = Field::new(spec.p.clone())?;
        let curve = Curve {
            f: f.clone(),
            a: spec.curve_a.clone().ok_or("curve missing")?,
        };
        let mut columns = vec![];
        let mut excluded = vec![];
        let half = f.mul(&curve.a, &f.inv(&2u32.into())?);
        let center = f.sub(&0u32.into(), &half);
        let mut raw_id = 0;
        for j in 1..=spec.k {
            for t in 0..spec.q {
                let z = spec.h.pow(
                    &(BigUint::from(j) + BigUint::from(spec.order_factor) * t),
                    &f,
                );
                let delta = f.mul(&half, &f.inv(&z.a)?);
                let mut candidates = vec![];
                for (branch, u) in [f.add(&center, &delta), f.sub(&center, &delta)]
                    .into_iter()
                    .enumerate()
                {
                    if let Some(v) = f.sqrt(&curve.rhs(&u))? {
                        let v = v.clone().min(f.sub(&BigUint::zero(), &v));
                        candidates.push((branch, u, v));
                    }
                }
                if candidates.len() != 1 {
                    return Err("raw map branch is not unique".into());
                }
                let (branch, u, v) = candidates.remove(0);
                let point = curve.times(&Some((u, v.clone())), 8)?;
                if point.is_none() {
                    excluded.push((j, t));
                } else {
                    columns.push(Column {
                        raw_id,
                        j,
                        t,
                        input: WitnessInput {
                            z,
                            selector: j - 1,
                            slack: spec.k - j,
                            branch: branch.into(),
                            raw_v: v,
                        },
                        point,
                    });
                }
                raw_id += 1;
            }
        }
        Ok(Self {
            spec: spec.clone(),
            curve,
            raw_count: raw as usize,
            columns,
            excluded,
        })
    }
    /// Returns false on the visitor's explicit early stop, true on exhaustion.
    fn halves(
        &self,
        r: usize,
        events: &mut Events,
        mut visitor: impl FnMut(Half) -> Result<bool>,
    ) -> Result<bool> {
        for indices in combinations(self.columns.len(), r) {
            let mask = indices
                .iter()
                .fold(0u64, |m, i| m | (1u64 << self.columns[*i].raw_id));
            for signs in 0..1usize << r {
                let mut point = None;
                for (bit, i) in indices.iter().enumerate() {
                    let p = &self.columns[*i].point;
                    let p = if (signs >> bit) & 1 == 1 {
                        negate(p, &self.curve.f)
                    } else {
                        p.clone()
                    };
                    point = self.curve.add(&point, &p)?;
                    events.curve_add_calls += 1;
                }
                events.half_tuples += 1;
                if !visitor(Half {
                    columns: indices.clone(),
                    signs,
                    mask,
                    point,
                })? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    fn inputs(&self, l: &Half, r: &Half) -> Vec<WitnessInput> {
        let mut selected = vec![];
        for half in [l, r] {
            for (bit, i) in half.columns.iter().enumerate() {
                let col = &self.columns[*i];
                let mut wi = col.input.clone();
                if (half.signs >> bit) & 1 == 1 {
                    wi.raw_v = self.curve.f.sub(&BigUint::zero(), &wi.raw_v);
                }
                selected.push((col.raw_id, wi));
            }
        }
        selected.sort_by_key(|p| p.0);
        selected.into_iter().map(|p| p.1).collect()
    }
}

pub struct Setup {
    base: Base,
    arity: usize,
    table: BTreeMap<Point, Family>,
    pub events: Events,
    pub half_tuples: usize,
    pub dimension: usize,
}
pub struct Answer {
    pub inputs: Option<Vec<WitnessInput>>,
    pub witness: Option<Vec<BigUint>>,
    pub events: Events,
    pub exhausted: bool,
}
impl Setup {
    pub fn build(spec: &Spec, arity: usize) -> Result<Self> {
        if ![2, 4, 6].contains(&arity) {
            return Err("unsupported reference arity".into());
        }
        let base = Base::new(spec)?;
        let r = arity / 2;
        let half_tuples = choose(base.columns.len(), r) * (1usize << r);
        if half_tuples > HALF_CAP {
            return Err("signed-half cap exceeded before half enumeration".into());
        }
        let dimension = choose(2 * r, r);
        let mut events = Events::default();
        let mut table = BTreeMap::<Point, Family>::new();
        let mut extra = Events::default();
        base.halves(r, &mut events, |h| {
            let ids: Vec<_> = h.columns.iter().map(|i| base.columns[*i].raw_id).collect();
            let v = wedge(&ids, r, &base.curve.f);
            extra.minor_vectors += 1;
            if table
                .entry(h.point.clone())
                .or_default()
                .insert(v, h, &base.curve.f)?
            {
                extra.basis_insertions += 1;
            }
            Ok(true)
        })?;
        events.minor_vectors = extra.minor_vectors;
        events.basis_insertions = extra.basis_insertions;
        Ok(Self {
            base,
            arity,
            table,
            events,
            half_tuples,
            dimension,
        })
    }
    /// Q is the only query input; no labels, scalar, point list or witness.
    pub fn solve(&self, target: &Point) -> Result<Answer> {
        if !self.base.curve.contains(target) {
            return Err("target is not canonical/on-curve".into());
        }
        let mut events = Events::default();
        let mut query_events = Events::default();
        let mut root = None;
        let exhausted = self.base.halves(self.arity / 2, &mut events, |left| {
            let complement = self
                .base
                .curve
                .add(target, &negate(&left.point, &self.base.curve.f))?;
            query_events.curve_add_calls += 1;
            query_events.dictionary_queries += 1;
            if let Some(family) = self.table.get(&complement) {
                for row in &family.rows {
                    query_events.representative_candidates += 1;
                    if row.original.mask & left.mask == 0 {
                        root = Some(self.base.inputs(&left, &row.original));
                        return Ok(false);
                    }
                }
            }
            Ok(true)
        })?;
        events.curve_add_calls += query_events.curve_add_calls;
        events.dictionary_queries = query_events.dictionary_queries;
        events.representative_candidates = query_events.representative_candidates;
        let witness = if let Some(inputs) = &root {
            let circuit =
                TargetCircuit::compile(&self.base.spec, self.arity, target.clone(), true)?;
            let w = circuit.witness(inputs)?;
            circuit.system.check(&w)?;
            circuit.decode_row(&w, &IndexTable::new(&self.base.spec)?)?;
            Some(w)
        } else {
            None
        };
        Ok(Answer {
            inputs: root,
            witness,
            events,
            exhausted,
        })
    }
    pub fn summary(&self) -> Value {
        json!({"arity":self.arity,"raw_columns":self.base.raw_count,"retained_raw_columns":self.base.columns.len(),
            "identity_columns_excluded":self.base.excluded,"signed_half_tuples":self.half_tuples,
            "sum_keys":self.table.len(),"representative_dimension":self.dimension,
            "retained_half_witnesses":self.table.values().map(|f|f.rows.len()).sum::<usize>(),
            "max_witnesses_per_sum":self.table.values().map(|f|f.rows.len()).max().unwrap_or(0),
            "construction_events":self.events,"raw_column_cap":RAW_CAP,"signed_half_cap":HALF_CAP,
            "accounting_scope":"selected API events, not normalized field work or timing; charge construction in full per target"})
    }
}
pub fn point_json(p: &Point) -> Value {
    match p {
        None => Value::Null,
        Some((u, v)) => json!([u.to_string(), v.to_string()]),
    }
}
