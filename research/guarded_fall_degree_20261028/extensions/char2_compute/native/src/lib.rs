//! Exact ordinary-degree polynomial closure over GF(2^m), m<=8.
//!
//! Field elements are binary-polynomial residues, never integers modulo 2^m.
//! No Boolean/field-equation reduction is implicit. This is a reference engine,
//! not a claim about large Boolean Weil-descent instances.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type Row = Vec<u16>;
type Sparse = BTreeMap<Vec<usize>, u16>;
pub type Error = String;

fn remainder(mut a: u16, b: u16) -> u16 {
    while a != 0 && a.leading_zeros() <= b.leading_zeros() {
        a ^= b << (b.leading_zeros() - a.leading_zeros());
    }
    a
}

pub fn irreducible(modulus: u16) -> bool {
    if modulus < 3 || modulus & 1 == 0 {
        return false;
    }
    let m = 15 - modulus.leading_zeros() as usize;
    if m > 8 {
        return false;
    }
    for d in 1..=m / 2 {
        for factor in (((1u16 << d) | 1)..(1u16 << (d + 1))).step_by(2) {
            if remainder(modulus, factor) == 0 {
                return false;
            }
        }
    }
    true
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldDescriptor {
    pub characteristic: usize,
    pub degree: usize,
    pub modulus: u16,
}

#[derive(Clone)]
pub struct Field {
    pub descriptor: FieldDescriptor,
    pub q: usize,
    table: Vec<Vec<u16>>,
    inverse: Vec<u16>,
}

impl Field {
    pub fn new(modulus: u16) -> Result<Self, Error> {
        if !irreducible(modulus) {
            return Err("binary irreducible modulus required (degree <=8)".into());
        }
        let degree = 15 - modulus.leading_zeros() as usize;
        let q = 1usize << degree;
        let mut table = vec![vec![0; q]; q];
        for (a, row) in table.iter_mut().enumerate() {
            for (b, value) in row.iter_mut().enumerate() {
                let (mut x, mut y, mut result) = (a as u16, b as u16, 0u16);
                while y != 0 {
                    if y & 1 != 0 {
                        result ^= x;
                    }
                    y >>= 1;
                    x <<= 1;
                    if x & q as u16 != 0 {
                        x ^= modulus;
                    }
                }
                *value = result;
            }
        }
        let mut field = Self {
            descriptor: FieldDescriptor {
                characteristic: 2,
                degree,
                modulus,
            },
            q,
            table,
            inverse: vec![0; q],
        };
        for a in 1..q {
            field.inverse[a] = field.pow(a as u16, q - 2);
            if field.mul(a as u16, field.inverse[a]) != 1 {
                return Err("invalid field inverse".into());
            }
        }
        Ok(field)
    }
    pub fn mul(&self, a: u16, b: u16) -> u16 {
        self.table[a as usize][b as usize]
    }
    pub fn pow(&self, mut a: u16, mut e: usize) -> u16 {
        let mut result = 1;
        while e != 0 {
            if e & 1 != 0 {
                result = self.mul(result, a);
            }
            a = self.mul(a, a);
            e >>= 1;
        }
        result
    }
    pub fn inv(&self, a: u16) -> Result<u16, Error> {
        if a == 0 || a as usize >= self.q {
            return Err("nonzero field element required".into());
        }
        Ok(self.inverse[a as usize])
    }
    pub fn square(&self, a: u16) -> u16 {
        self.mul(a, a)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Problem {
    pub field: FieldDescriptor,
    pub n: usize,
    pub polynomials: Vec<Vec<(u16, Vec<usize>)>>,
}

impl Problem {
    pub fn normalized(&self) -> Result<(Self, Field), Error> {
        let f = Field::new(self.field.modulus)?;
        if f.descriptor != self.field || self.n == 0 || self.n > 64 {
            return Err("invalid field descriptor or variable count".into());
        }
        let mut polys = Vec::new();
        for poly in &self.polynomials {
            let mut terms = BTreeMap::<Vec<usize>, u16>::new();
            for (c, e) in poly {
                if *c as usize >= f.q || e.len() != self.n || e.iter().any(|x| *x > 1_000_000) {
                    return Err("invalid polynomial term".into());
                }
                *terms.entry(e.clone()).or_default() ^= *c;
            }
            polys.push(
                terms
                    .into_iter()
                    .filter(|(_, c)| *c != 0)
                    .map(|(e, c)| (c, e))
                    .collect(),
            );
        }
        Ok((
            Self {
                field: f.descriptor.clone(),
                n: self.n,
                polynomials: polys,
            },
            f,
        ))
    }
}

fn monomials(n: usize, d: usize) -> Result<Vec<Vec<usize>>, Error> {
    if n == 0 || n > 64 || d > 64 || n + d > 100 {
        return Err("reference degree limit".into());
    }
    let mut count = 1usize;
    for k in 1..=n.min(d) {
        count = count.saturating_mul(n + d + 1 - k) / k;
        if count > 20_000 {
            return Err("reference column limit (20,000)".into());
        }
    }
    fn exact(n: usize, t: usize, prefix: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if n == 0 {
            if t == 0 {
                out.push(prefix.clone());
            }
            return;
        }
        for a in (0..=t).rev() {
            prefix.push(a);
            exact(n - 1, t - a, prefix, out);
            prefix.pop();
        }
    }
    let mut out = Vec::new();
    for t in (0..=d).rev() {
        exact(n, t, &mut Vec::new(), &mut out);
    }
    Ok(out)
}

fn row_degree(row: &[u16], columns: &[Vec<usize>]) -> Option<usize> {
    row.iter()
        .position(|c| *c != 0)
        .map(|j| columns[j].iter().sum())
}

fn eligible_inputs(
    problem: &Problem,
    d: usize,
    index: &BTreeMap<Vec<usize>, usize>,
) -> Vec<(usize, Row)> {
    problem
        .polynomials
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.iter().any(|(_, e)| e.iter().sum::<usize>() > d) {
                return None;
            }
            let mut row = vec![0; index.len()];
            for (c, e) in p {
                row[index[e]] ^= *c;
            }
            Some((i, row))
        })
        .collect()
}

fn variable_product(
    row: &[u16],
    variable: usize,
    columns: &[Vec<usize>],
    index: &BTreeMap<Vec<usize>, usize>,
) -> Row {
    let mut result = vec![0; columns.len()];
    for (c, e) in row.iter().zip(columns) {
        if *c != 0 {
            let mut power = e.clone();
            power[variable] += 1;
            result[index[&power]] = *c;
        }
    }
    result
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum Node {
    Input {
        index: usize,
    },
    Scale {
        parent: usize,
        coefficient: u16,
    },
    Add {
        left: usize,
        right: usize,
        coefficient: u16,
    },
    Multiply {
        parent: usize,
        variable: usize,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Refutation {
    pub schema: String,
    pub problem: Problem,
    pub degree: usize,
    pub nodes: Vec<Node>,
    pub goal: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Closure {
    pub problem: Problem,
    pub degree: usize,
    pub basis: Vec<Row>,
    pub dual: Option<Row>,
    pub contains_one: bool,
    pub rounds: usize,
    pub rank: usize,
    pub column_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refutation: Option<Refutation>,
}

#[derive(Clone)]
struct Entry {
    row: Row,
    source: Option<usize>,
}

struct Recorder {
    enabled: bool,
    nodes: Vec<Node>,
}
impl Recorder {
    fn push(&mut self, node: Node) -> Option<usize> {
        if self.enabled {
            let i = self.nodes.len();
            self.nodes.push(node);
            Some(i)
        } else {
            None
        }
    }
    fn add(
        &mut self,
        left: Option<usize>,
        right: Option<usize>,
        coefficient: u16,
    ) -> Option<usize> {
        if self.enabled {
            self.push(Node::Add {
                left: left.unwrap(),
                right: right.unwrap(),
                coefficient,
            })
        } else {
            None
        }
    }
    fn scale(&mut self, parent: Option<usize>, coefficient: u16) -> Option<usize> {
        if self.enabled {
            self.push(Node::Scale {
                parent: parent.unwrap(),
                coefficient,
            })
        } else {
            None
        }
    }
}

fn insert(
    basis: &mut BTreeMap<usize, Entry>,
    mut row: Row,
    mut source: Option<usize>,
    field: &Field,
    record: &mut Recorder,
) -> Result<bool, Error> {
    for (j, old) in basis.iter() {
        let c = row[*j];
        if c != 0 {
            for (a, b) in row.iter_mut().zip(&old.row) {
                *a ^= field.mul(c, *b);
            }
            source = record.add(source, old.source, c);
        }
    }
    let Some(pivot) = row.iter().position(|c| *c != 0) else {
        return Ok(false);
    };
    let inverse = field.inv(row[pivot])?;
    if inverse != 1 {
        for a in &mut row {
            *a = field.mul(*a, inverse);
        }
        source = record.scale(source, inverse);
    }
    for old in basis.values_mut() {
        let c = old.row[pivot];
        if c != 0 {
            for (a, b) in old.row.iter_mut().zip(&row) {
                *a ^= field.mul(c, *b);
            }
            old.source = record.add(old.source, source, c);
        }
    }
    basis.insert(pivot, Entry { row, source });
    Ok(true)
}

fn compact_refutation(problem: &Problem, d: usize, record: &Recorder, goal: usize) -> Refutation {
    let mut keep = vec![false; record.nodes.len()];
    let mut pending = vec![goal];
    while let Some(i) = pending.pop() {
        if keep[i] {
            continue;
        }
        keep[i] = true;
        match record.nodes[i] {
            Node::Input { .. } => {}
            Node::Scale { parent, .. } | Node::Multiply { parent, .. } => pending.push(parent),
            Node::Add { left, right, .. } => {
                pending.push(left);
                pending.push(right);
            }
        }
    }
    let mut map = vec![0; keep.len()];
    let mut count = 0;
    for (i, k) in keep.iter().enumerate() {
        if *k {
            map[i] = count;
            count += 1;
        }
    }
    let nodes = record
        .nodes
        .iter()
        .enumerate()
        .filter(|(i, _)| keep[*i])
        .map(|(_, node)| match *node {
            Node::Input { index } => Node::Input { index },
            Node::Scale {
                parent,
                coefficient,
            } => Node::Scale {
                parent: map[parent],
                coefficient,
            },
            Node::Add {
                left,
                right,
                coefficient,
            } => Node::Add {
                left: map[left],
                right: map[right],
                coefficient,
            },
            Node::Multiply { parent, variable } => Node::Multiply {
                parent: map[parent],
                variable,
            },
        })
        .collect();
    Refutation {
        schema: "gf2m-bounded-degree-refutation-v1".into(),
        problem: problem.clone(),
        degree: d,
        nodes,
        goal: map[goal],
    }
}

pub fn closure(problem: &Problem, d: usize, trace: bool) -> Result<Closure, Error> {
    let (problem, field) = problem.normalized()?;
    let columns = monomials(problem.n, d)?;
    let index: BTreeMap<_, _> = columns
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let mut basis = BTreeMap::new();
    let mut record = Recorder {
        enabled: trace,
        nodes: Vec::new(),
    };
    for (i, row) in eligible_inputs(&problem, d, &index) {
        let source = record.push(Node::Input { index: i });
        insert(&mut basis, row, source, &field, &mut record)?;
    }
    let mut rounds = 0;
    loop {
        rounds += 1;
        let low: Vec<_> = basis
            .values()
            .filter(|entry| row_degree(&entry.row, &columns).is_some_and(|x| x < d))
            .cloned()
            .collect();
        let mut changed = false;
        for entry in low {
            for variable in 0..problem.n {
                let source = if trace {
                    record.push(Node::Multiply {
                        parent: entry.source.unwrap(),
                        variable,
                    })
                } else {
                    None
                };
                changed |= insert(
                    &mut basis,
                    variable_product(&entry.row, variable, &columns, &index),
                    source,
                    &field,
                    &mut record,
                )?;
            }
        }
        if !changed {
            break;
        }
    }
    let contains_one = basis.contains_key(&(columns.len() - 1));
    let dual = if contains_one {
        None
    } else {
        let mut dual = vec![0; columns.len()];
        dual[columns.len() - 1] = 1;
        for (j, entry) in &basis {
            dual[*j] = entry.row[columns.len() - 1];
        }
        Some(dual)
    };
    let refutation = if trace && contains_one {
        Some(compact_refutation(
            &problem,
            d,
            &record,
            basis[&(columns.len() - 1)].source.unwrap(),
        ))
    } else {
        None
    };
    Ok(Closure {
        problem,
        degree: d,
        basis: basis.values().map(|e| e.row.clone()).collect(),
        dual,
        contains_one,
        rounds,
        rank: basis.len(),
        column_count: columns.len(),
        refutation,
    })
}

fn lower_inner(expected: &Problem, cert: &Closure) -> Result<bool, Error> {
    let (problem, field) = expected.normalized()?;
    if cert.problem.normalized()?.0 != problem {
        return Ok(false);
    }
    let columns = monomials(problem.n, cert.degree)?;
    let index: BTreeMap<_, _> = columns
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect();
    let Some(dual) = &cert.dual else {
        return Ok(false);
    };
    if dual.len() != columns.len()
        || dual.last() != Some(&1)
        || dual.iter().any(|c| *c as usize >= field.q)
    {
        return Ok(false);
    }
    let mut pivots = Vec::new();
    for row in &cert.basis {
        if row.len() != columns.len() || row.iter().any(|c| *c as usize >= field.q) {
            return Ok(false);
        }
        let Some(pivot) = row.iter().position(|c| *c != 0) else {
            return Ok(false);
        };
        if row[pivot] != 1 || pivots.last().is_some_and(|j| *j >= pivot) {
            return Ok(false);
        }
        pivots.push(pivot);
    }
    for (i, row) in cert.basis.iter().enumerate() {
        if pivots
            .iter()
            .enumerate()
            .any(|(k, j)| i != k && row[*j] != 0)
        {
            return Ok(false);
        }
        let dot = row
            .iter()
            .zip(dual)
            .fold(0, |a, (x, y)| a ^ field.mul(*x, *y));
        if dot != 0 {
            return Ok(false);
        }
    }
    let member = |mut query: Row| {
        for (pivot, row) in pivots.iter().zip(&cert.basis) {
            let c = query[*pivot];
            if c != 0 {
                for (a, b) in query.iter_mut().zip(row) {
                    *a ^= field.mul(c, *b);
                }
            }
        }
        query.iter().all(|a| *a == 0)
    };
    for (_, row) in eligible_inputs(&problem, cert.degree, &index) {
        if !member(row) {
            return Ok(false);
        }
    }
    for row in &cert.basis {
        if row_degree(row, &columns).is_some_and(|d| d < cert.degree) {
            for variable in 0..problem.n {
                if !member(variable_product(row, variable, &columns, &index)) {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

pub fn verify_lower(expected: &Problem, cert: &Closure) -> bool {
    lower_inner(expected, cert).unwrap_or(false)
}

fn upper_inner(expected: &Problem, cert: &Refutation) -> Result<bool, Error> {
    let (problem, field) = expected.normalized()?;
    if cert.problem.normalized()?.0 != problem || cert.schema != "gf2m-bounded-degree-refutation-v1"
    {
        return Ok(false);
    }
    let mut values: Vec<Sparse> = Vec::new();
    for node in &cert.nodes {
        let mut value: Sparse = match *node {
            Node::Input { index } => {
                let p = problem.polynomials.get(index).ok_or("input index")?;
                p.iter().map(|(c, e)| (e.clone(), *c)).collect()
            }
            Node::Scale {
                parent,
                coefficient,
            } => {
                if coefficient as usize >= field.q {
                    return Ok(false);
                }
                values
                    .get(parent)
                    .ok_or("DAG reference")?
                    .iter()
                    .map(|(e, c)| (e.clone(), field.mul(*c, coefficient)))
                    .collect()
            }
            Node::Add {
                left,
                right,
                coefficient,
            } => {
                if coefficient as usize >= field.q {
                    return Ok(false);
                }
                let mut value = values.get(left).ok_or("DAG reference")?.clone();
                for (e, c) in values.get(right).ok_or("DAG reference")? {
                    *value.entry(e.clone()).or_default() ^= field.mul(*c, coefficient);
                }
                value
            }
            Node::Multiply { parent, variable } => {
                if variable >= problem.n {
                    return Ok(false);
                }
                values
                    .get(parent)
                    .ok_or("DAG reference")?
                    .iter()
                    .map(|(e, c)| {
                        let mut e = e.clone();
                        e[variable] += 1;
                        (e, *c)
                    })
                    .collect()
            }
        };
        value.retain(|_, c| *c != 0);
        if value.keys().any(|e| e.iter().sum::<usize>() > cert.degree) {
            return Ok(false);
        }
        values.push(value);
    }
    let Some(goal) = values.get(cert.goal) else {
        return Ok(false);
    };
    Ok(goal.len() == 1 && goal.get(&vec![0; problem.n]) == Some(&1))
}

pub fn verify_refutation(expected: &Problem, cert: &Refutation) -> bool {
    upper_inner(expected, cert).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_fields_not_integer_rings() {
        for modulus in [7, 11, 19] {
            let f = Field::new(modulus).unwrap();
            for a in 0..f.q as u16 {
                assert_eq!(f.pow(a, f.q), a);
                for b in 0..f.q as u16 {
                    for c in 0..f.q as u16 {
                        assert_eq!(f.mul(a, b ^ c), f.mul(a, b) ^ f.mul(a, c));
                        assert_eq!(f.mul(f.mul(a, b), c), f.mul(a, f.mul(b, c)));
                    }
                }
            }
        }
        assert_eq!(Field::new(7).unwrap().mul(2, 2), 3);
        assert!(!irreducible(0b10101));
    }
    fn toy() -> Problem {
        // (x+alpha)(y+alpha) never vanishes on {0,1}^2 in GF4, alpha=2.
        Problem {
            field: Field::new(7).unwrap().descriptor,
            n: 2,
            polynomials: vec![
                vec![
                    (1, vec![1, 1]),
                    (2, vec![1, 0]),
                    (2, vec![0, 1]),
                    (3, vec![0, 0]),
                ],
                vec![(1, vec![2, 0]), (1, vec![1, 0])],
                vec![(1, vec![0, 2]), (1, vec![0, 1])],
            ],
        }
    }
    #[test]
    fn bound_certificates_and_sparse_replay() {
        let p = toy();
        let low = closure(&p, 2, false).unwrap();
        assert!(!low.contains_one);
        assert!(verify_lower(&p, &low));
        let high = closure(&p, 3, true).unwrap();
        assert!(high.contains_one);
        let proof = high.refutation.unwrap();
        assert!(verify_refutation(&p, &proof));
        let mut changed = p.clone();
        changed.polynomials[0].push((1, vec![0, 0]));
        assert!(!verify_refutation(&changed, &proof));
        assert!(!verify_lower(&changed, &low));
        let mut bad = proof.clone();
        bad.degree = 2;
        assert!(!verify_refutation(&p, &bad));
        let mut bad = proof.clone();
        bad.nodes[0] = Node::Multiply {
            parent: 0,
            variable: 0,
        };
        assert!(!verify_refutation(&p, &bad));
        let mut bad_low = low.clone();
        *bad_low.dual.as_mut().unwrap().last_mut().unwrap() = 0;
        assert!(!verify_lower(&p, &bad_low));
    }
    #[test]
    fn normalize_cancellation_before_degree_filter() {
        let p = Problem {
            field: Field::new(11).unwrap().descriptor,
            n: 1,
            polynomials: vec![vec![(1, vec![20]), (1, vec![20]), (1, vec![0])]],
        };
        let c = closure(&p, 0, true).unwrap();
        assert!(c.contains_one);
        assert!(verify_refutation(&p, &c.refutation.unwrap()));
    }

    #[test]
    fn proved_gf16_cube_orbit_exact_four_regression() {
        // Pinned consequence of the independently audited cube-orbit theorem,
        // not a search over curves, bases, fields, or targets. GF16 is defined
        // by alpha^4+alpha+1; S={1,6,7}=GF4^*, R=(8,15) is outside E(GF4).
        let problem: Problem =
            serde_json::from_str(include_str!("../gf16_subfield_fixture.json")).unwrap();
        let lower = closure(&problem, 3, false).unwrap();
        assert!(!lower.contains_one);
        assert!(verify_lower(&problem, &lower));

        let upper = closure(&problem, 4, true).unwrap();
        assert!(upper.contains_one);
        let proof = upper.refutation.unwrap();
        assert_eq!(proof.degree, 4);
        assert!(verify_refutation(&problem, &proof));

        // Recheck the serialized provenance, not just the in-memory object.
        let encoded = serde_json::to_string(&proof).unwrap();
        let replay: Refutation = serde_json::from_str(&encoded).unwrap();
        assert!(verify_refutation(&problem, &replay));
    }
}
