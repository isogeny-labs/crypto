//! Exact fixed-size field rank with invertible row-operation replay.
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use torus_s4::Field;
pub type Matrix = Vec<Vec<BigUint>>;
#[derive(Clone)]
pub enum Op {
    Swap(usize, usize),
    Scale(usize, BigUint),
    Add(usize, usize, BigUint),
}
impl Op {
    fn json(&self) -> Value {
        match self {
            Self::Swap(i, j) => json!(["swap", i, j]),
            Self::Scale(i, a) => json!(["scale", i, a.to_string()]),
            Self::Add(i, j, a) => json!(["add", i, j, a.to_string()]),
        }
    }
}
pub struct Reduction {
    pub rank: usize,
    pub form: Matrix,
    pub left: Matrix,
    pub operations: Vec<Op>,
    pub pivots: Vec<usize>,
}
fn admission(f: &Field, m: &Matrix) -> Result<usize> {
    if f.p != crate::subgroup_order(false)? && f.p != crate::subgroup_order(true)? {
        return Err("only the two declared prime subgroup fields are supported".into());
    }
    let n = m.first().map_or(0, Vec::len);
    if m.len() > 256
        || n > 512
        || m.iter()
            .any(|r| r.len() != n || r.iter().any(|x| x >= &f.p))
    {
        return Err("matrix shape/canonical/resource admission".into());
    }
    Ok(n)
}
fn identity(n: usize) -> Matrix {
    (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        BigUint::one()
                    } else {
                        BigUint::zero()
                    }
                })
                .collect()
        })
        .collect()
}
fn apply(f: &Field, m: &mut Matrix, op: &Op) -> Result<()> {
    let r = m.len();
    match op {
        Op::Swap(i, j) => {
            if *i >= r || *j >= r {
                return Err("swap range".into());
            }
            m.swap(*i, *j);
        }
        Op::Scale(i, a) => {
            if *i >= r || a.is_zero() || a >= &f.p {
                return Err("noninvertible/noncanonical scale".into());
            }
            for x in &mut m[*i] {
                *x = f.mul(x, a);
            }
        }
        Op::Add(i, j, a) => {
            if *i >= r || *j >= r || i == j || a >= &f.p {
                return Err("noninvertible/range add".into());
            }
            let src = m[*j].clone();
            for (x, y) in m[*i].iter_mut().zip(src) {
                *x = f.add(x, &f.mul(a, &y));
            }
        }
    }
    Ok(())
}
pub fn reduce(f: &Field, m: &Matrix) -> Result<Reduction> {
    let n = admission(f, m)?;
    let mut form = m.clone();
    let mut left = identity(m.len());
    let mut operations = vec![];
    let mut pivots = vec![];
    let mut r = 0;
    for c in 0..n {
        let Some(i) = (r..m.len()).find(|i| !form[*i][c].is_zero()) else {
            continue;
        };
        let op = Op::Swap(r, i);
        apply(f, &mut form, &op)?;
        apply(f, &mut left, &op)?;
        operations.push(op);
        let op = Op::Scale(r, f.inv(&form[r][c])?);
        apply(f, &mut form, &op)?;
        apply(f, &mut left, &op)?;
        operations.push(op);
        for i in 0..m.len() {
            if i != r && !form[i][c].is_zero() {
                let op = Op::Add(i, r, f.sub(&BigUint::zero(), &form[i][c]));
                apply(f, &mut form, &op)?;
                apply(f, &mut left, &op)?;
                operations.push(op);
            }
        }
        pivots.push(c);
        r += 1;
        if r == m.len() {
            break;
        }
    }
    let out = Reduction {
        rank: r,
        form,
        left,
        operations,
        pivots,
    };
    verify(f, m, &out)?;
    Ok(out)
}
pub fn verify(f: &Field, m: &Matrix, r: &Reduction) -> Result<()> {
    let n = admission(f, m)?;
    if r.rank > m.len()
        || r.pivots.len() != r.rank
        || r.form.len() != m.len()
        || r.left.len() != m.len()
        || r.operations.len() > 256 * 515
    {
        return Err("rank certificate shape/cap".into());
    }
    let mut a = m.clone();
    let mut e = identity(m.len());
    for op in &r.operations {
        apply(f, &mut a, op)?;
        apply(f, &mut e, op)?;
    }
    if a != r.form || e != r.left {
        return Err("row-operation replay mismatch".into());
    }
    for i in 0..r.rank {
        let c = r.pivots[i];
        if c >= n
            || (i > 0 && r.pivots[i - 1] >= c)
            || a[i][c] != BigUint::one()
            || a[i][..c].iter().any(|x| !x.is_zero())
        {
            return Err("pivot shape".into());
        }
        for (j, row) in a.iter().enumerate() {
            if j != i && !row[c].is_zero() {
                return Err("pivot column".into());
            }
        }
    }
    if a[r.rank..].iter().flatten().any(|x| !x.is_zero()) {
        return Err("nonzero unranked row".into());
    }
    // Separate dense multiplication checks every retained left-kernel vector.
    for v in &e[r.rank..] {
        let mut product = vec![BigUint::zero(); n];
        for (k, row) in v.iter().zip(m) {
            for (acc, x) in product.iter_mut().zip(row) {
                *acc = f.add(acc, &f.mul(k, x));
            }
        }
        if product.iter().any(|x| !x.is_zero()) {
            return Err("kernel multiplication".into());
        }
    }
    Ok(())
}
fn strings(m: &Matrix) -> Vec<Vec<String>> {
    m.iter()
        .map(|r| r.iter().map(ToString::to_string).collect())
        .collect()
}
impl Reduction {
    pub fn json(&self) -> Value {
        json!({"rank":self.rank,"pivots":self.pivots,"rref":strings(&self.form),"left_transform":strings(&self.left),"invertible_row_operations":self.operations.iter().map(Op::json).collect::<Vec<_>>()})
    }
}
pub fn dot(f: &Field, a: &[BigUint], b: &[BigUint]) -> Result<BigUint> {
    if a.len() != b.len() {
        return Err("dot shape".into());
    }
    Ok(a.iter()
        .zip(b)
        .fold(BigUint::zero(), |s, (x, y)| f.add(&s, &f.mul(x, y))))
}
pub struct Analysis {
    pub rank: usize,
    pub augmented_rank: usize,
    pub recovered: Vec<BigUint>,
    pub record: Value,
}
pub fn analyze(f: &Field, m: &Matrix, a: &[BigUint], b: &[BigUint]) -> Result<Analysis> {
    if admission(f, m)? >= 512 {
        return Err("augmented column reservation cap; incomplete".into());
    }
    if m.len() != a.len() || a.len() != b.len() || a.iter().chain(b).any(|x| x >= &f.p) {
        return Err("tag shape/canonical".into());
    }
    let r = reduce(f, m)?;
    let aug = m
        .iter()
        .zip(b)
        .map(|(row, y)| {
            let mut row = row.clone();
            row.push(y.clone());
            row
        })
        .collect();
    let ar = reduce(f, &aug)?;
    let mut recovered = vec![];
    let mut kernels = vec![];
    for v in &r.left[r.rank..] {
        let va = dot(f, v, a)?;
        let vb = dot(f, v, b)?;
        let x = if vb.is_zero() {
            if !va.is_zero() {
                return Err("inconsistent zero-tag kernel".into());
            }
            None
        } else {
            let x = f.mul(&f.sub(&BigUint::zero(), &va), &f.inv(&vb)?);
            recovered.push(x.clone());
            Some(x.to_string())
        };
        kernels.push(json!({"v":v.iter().map(ToString::to_string).collect::<Vec<_>>(),"vTa":va.to_string(),"vTb":vb.to_string(),"candidate_x":x}));
    }
    if (ar.rank > r.rank) != !recovered.is_empty() {
        return Err("augmented-rank/kernel disagreement".into());
    }
    let record = json!({"modulus":f.p.to_string(),"M":strings(m),"a":a.iter().map(ToString::to_string).collect::<Vec<_>>(),"b":b.iter().map(ToString::to_string).collect::<Vec<_>>(),"M_certificate":r.json(),"augmented_certificate":ar.json(),"left_kernel":kernels,"informative_witnesses":recovered.len(),"scope":"exact fixed linear data with invertible-operation and kernel replay; producer custody"});
    Ok(Analysis {
        rank: r.rank,
        augmented_rank: ar.rank,
        recovered,
        record,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn v(a: &[u32]) -> Vec<BigUint> {
        a.iter().copied().map(BigUint::from).collect()
    }
    #[test]
    fn informative_and_degenerate_kernels() {
        let f = Field::new(13u32.into()).unwrap();
        let m = vec![v(&[1, 1, 0]), v(&[0, 1, 1]), v(&[1, 0, 12])];
        let a = v(&[0, 10, 8]);
        let b = v(&[1, 0, 0]);
        let r = analyze(&f, &m, &a, &b).unwrap();
        assert_eq!(r.recovered, vec![5u32.into()]);
        let d = analyze(&f, &vec![v(&[1, 1]), v(&[1, 1])], &v(&[0, 0]), &v(&[1, 1])).unwrap();
        assert!(d.recovered.is_empty());
        let p = analyze(
            &f,
            &vec![v(&[1, 1, 0, 0]), v(&[0, 0, 1, 1])],
            &v(&[0, 0]),
            &v(&[1, 1]),
        )
        .unwrap();
        assert_eq!(p.rank, 2);
    }
    #[test]
    fn certificate_corruption_and_invalid_shapes_refuse() {
        let f = Field::new(13u32.into()).unwrap();
        let m = vec![v(&[1, 1]), v(&[1, 1])];
        let mut r = reduce(&f, &m).unwrap();
        r.operations.push(Op::Scale(0, BigUint::zero()));
        assert!(verify(&f, &m, &r).is_err());
        assert!(reduce(&f, &vec![v(&[1]), v(&[1, 2])]).is_err());
        assert!(reduce(&f, &vec![v(&[13])]).is_err());
        let composite = Field::new(15u32.into()).unwrap();
        assert!(reduce(&composite, &m).is_err());
        assert!(analyze(&f, &vec![vec![BigUint::zero(); 512]], &v(&[0]), &v(&[0])).is_err());
    }
}
