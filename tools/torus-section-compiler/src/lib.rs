//! Public coefficient compiler. Supplied-coefficient extension is a separate API.
pub mod arithmetic;
pub mod controls;
pub mod gates;
use arithmetic::{c, z, Engine, Ring, Scalar, P};
use gates::{Program, Wire};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use torus_coset_circuit::Spec;
use torus_s4::{curve25519_prime, Curve, Point};
use torus_s4_tree::Fp2;
pub type Result<T> = std::result::Result<T, String>;

pub struct Compiled {
    pub program: Program,
    pub spec: Spec,
    pub r: Point,
    pub target: Point,
    pub coefficient_slots: Vec<usize>,
    pub input_wires: Vec<Wire>,
    pub s: Vec<Wire>,
    pub map_remainder: Wire,
    pub torsion_resultant: Wire,
    pub guard_prefix: usize,
    pub h: Vec<Wire>,
}
/// Cached cofactor expansion: no pivot, division or unknown-value branch.
fn determinant(e: &Engine, m: &[Vec<Wire>]) -> Result<Wire> {
    let n = m.len();
    if n == 0 || n > 6 || m.iter().any(|r| r.len() != n) {
        return Err("matrix dimension outside1..6".into());
    }
    let mut dp = vec![Wire(0); 1 << n];
    dp[0] = Wire(1);
    let mut k = e.builder.borrow_mut();
    for mask in 1usize..1 << n {
        let row = mask.count_ones() as usize - 1;
        let mut pos = 0;
        for j in 0..n {
            if mask >> j & 1 == 0 {
                continue;
            }
            let term = k.mul(m[row][j], dp[mask ^ (1 << j)]);
            dp[mask] = if (row + pos).is_multiple_of(2) {
                k.add(dp[mask], term)
            } else {
                k.sub(dp[mask], term)
            };
            pos += 1;
        }
    }
    Ok(dp[(1 << n) - 1])
}
pub fn torsion_polynomial(e: &Engine, a: &BigUint) -> Result<P> {
    let neg4 = e.neg(&c(4));
    let middle = e.neg(&e.add(&c(2), &e.mul(&c(4), &e.real(a.clone()))?));
    e.school(&[e.neg(&c(1)), c(1)], &[c(1), neg4, middle, neg4, c(1)])
}
fn validate(e: &Engine, s: &Spec, r: &Point, q: &Point) -> Result<Curve> {
    let toy = s.p == BigUint::from(101u32);
    let (a, smallq, ca): (u64, u64, u64) = if toy {
        (17, 3, 95)
    } else {
        (60824497, 213156431, 486662)
    };
    if (!toy && s.p != curve25519_prime())
        || s.order_factor != a
        || s.q != smallq
        || s.curve_a != Some(ca.into())
        || s.k == 0
        || s.k > 65536
        || 2 * s.k >= a
        || s.h.a >= s.p
        || s.h.b >= s.p
    {
        return Err("unsupported/noncanonical original parameter contract".into());
    }
    if s.h.norm(&e.f) != BigUint::one()
        || s.h.pow(&(BigUint::from(a) * smallq), &e.f) != Fp2::one()
        || s.h.pow(&a.into(), &e.f) == Fp2::one()
        || s.h.pow(&smallq.into(), &e.f) == Fp2::one()
    {
        return Err("original exact-order contract".into());
    }
    let curve = Curve {
        f: e.f.clone(),
        a: ca.into(),
    };
    if [r, q]
        .iter()
        .filter_map(|x| x.as_ref())
        .any(|(x, y)| x >= &s.p || y >= &s.p)
        || !curve.contains(r)
        || !curve.contains(q)
        || curve.times(r, 8)? != *q
    {
        return Err("malformed public pretarget or [8] contract".into());
    }
    if r.as_ref().is_some_and(|(_, y)| y.is_zero()) {
        return Err("unsupported finite2-torsion target chart".into());
    }
    let plus = e.f.add(&curve.a, &2u32.into());
    let minus = e.f.sub(&curve.a, &2u32.into());
    if plus.is_zero()
        || minus.is_zero()
        || e.f.sqrt(&plus)?.is_none()
        || minus.modpow(&((&s.p - 1u32) >> 1), &s.p) != &s.p - 1u32
    {
        return Err("fixed rational torsion hypotheses not established".into());
    }
    Ok(curve)
}
/// Compile only public data. Unknown coefficients/labels/witnesses are not arguments.
pub fn compile(s: &Spec, r: Point, q: Point, max_nodes: usize) -> Result<Compiled> {
    let e = Engine::new(s.p.clone(), max_nodes)?;
    let curve = validate(&e, s, &r, &q)?;
    let pivot = if r.is_some() { 4 } else { 5 };
    let slots: Vec<_> = (0..6).filter(|i| *i != pivot).collect();
    let mut cs = [z(); 6];
    cs[pivot] = c(1);
    let mut inputs = vec![];
    for slot in &slots {
        cs[*slot] = e.input();
        inputs.push(cs[*slot].a);
    }
    let beta = e.input();
    inputs.push(beta.a);
    let eta = e.input();
    inputs.push(eta.a);
    let cubic = vec![z(), c(1), e.real(curve.a.clone()), c(1)];
    let (u, v) = if let Some((x, y)) = &r {
        let line = [e.neg(&e.real(x.clone())), c(1)];
        (
            e.padd(
                &e.school(&line, &[cs[0], cs[1], cs[3]])?,
                &[e.mul(&cs[5], &e.real(y.clone()))?],
            ),
            e.padd(&e.school(&line, &[cs[2], cs[4]])?, &[cs[5]]),
        )
    } else {
        (vec![cs[0], cs[1], cs[3], cs[5]], vec![cs[2], cs[4]])
    };
    let norm = e.psub(&e.school(&u, &u)?, &e.school(&cubic, &e.school(&v, &v)?)?);
    let norm = if let Some((x, _)) = &r {
        let (v, rem) = e.divrem(&norm, &[e.neg(&e.real(x.clone())), c(1)])?;
        // The forced factor is a polynomial identity. Retain its constraint even
        // when this arithmetic layout does not structurally fold it to zero.
        for node in rem {
            e.builder.borrow_mut().constrain(node.a, 0);
            e.builder.borrow_mut().constrain(node.b, 0);
        }
        e.scale(&v, &e.neg(&c(1)))?
    } else {
        norm
    };
    if norm.len() != 7 || norm[6] != c(1) {
        return Err("nonmonic sextic compiler invariant".into());
    }
    let mut ring = Ring::new(&e, norm)?;
    let half = e.mul(&e.real(curve.a.clone()), &e.inv(&c(2))?)?;
    let r0 = e.neg(&half);
    let (quotient, remainder) = e.divrem(&ring.s, &[e.neg(&r0), c(1)])?;
    let rem = remainder.first().copied().unwrap_or_else(z);
    if rem.b != Wire(0) {
        return Err("nonreal map remainder".into());
    }
    let j = ring.q(&torsion_polynomial(&e, &curve.a)?)?;
    let mut m = vec![vec![Wire(0); 6]; 6];
    for col in 0..6 {
        let mut basis = vec![z(); 6];
        basis[col] = c(1);
        let product = ring.mul(&j, &basis)?;
        for row in 0..6 {
            if product[row].b != Wire(0) {
                return Err("nonreal multiplication matrix".into());
            }
            m[row][col] = product[row].a;
        }
    }
    let delta = determinant(&e, &m)?;
    let prefix = e.builder.borrow().gates.len();
    let map_guard = e.mul(&beta, &rem)?;
    e.builder.borrow_mut().constrain(map_guard.a, 1);
    let unit_guard = e.mul(
        &eta,
        &Scalar {
            a: delta,
            b: Wire(0),
        },
    )?;
    e.builder.borrow_mut().constrain(unit_guard.a, 1);
    let lambda = ring.q(&e.scale(&quotient, &e.neg(&e.mul(&half, &beta)?))?)?;
    let t = ring.chebyshev(s.q, &lambda)?;
    ring.tau = ring.sub(
        &ring.scale(&ring.mul(&t, &t)?, &c(4))?,
        &ring.constant(c(2)),
    );
    let omega = s.h.pow(&s.q.into(), &e.f);
    let rho = omega.mul(&omega, &e.f);
    let h = ring.h_interval(&e.scalar(&rho), 1, s.k)?;
    let output: Vec<_> = h.a.iter().chain(&h.b).flat_map(|v| [v.a, v.b]).collect();
    for w in &output {
        e.builder.borrow_mut().constrain(*w, 0);
    }
    let norm: Vec<_> = ring.s.iter().map(|v| v.a).collect();
    drop(ring);
    e.check()?;
    let program = e.builder.into_inner().finish()?;
    program.validate()?;
    Ok(Compiled {
        program,
        spec: s.clone(),
        r,
        target: q,
        coefficient_slots: slots,
        input_wires: inputs,
        s: norm,
        map_remainder: rem.a,
        torsion_resultant: delta,
        guard_prefix: prefix,
        h: output,
    })
}
impl Compiled {
    /// Fill the unique acyclic extension for a supplied normalized tuple.
    /// This is evaluation of known inputs, not coefficient discovery.
    pub fn extend(&self, cs: &[BigUint]) -> Result<Vec<BigUint>> {
        let pivot = if self.r.is_some() { 4 } else { 5 };
        if cs.len() != 6 || cs[pivot] != BigUint::one() || cs.iter().any(|x| x >= &self.spec.p) {
            return Err("noncanonical or unnormalized section tuple".into());
        }
        let mut inputs: Vec<_> = self
            .coefficient_slots
            .iter()
            .map(|i| cs[*i].clone())
            .collect();
        inputs.resize(7, BigUint::zero());
        let prefix = self.program.evaluate_prefix(&inputs, self.guard_prefix)?;
        inputs[5] = self
            .program
            .f
            .inv(&prefix[self.map_remainder.0 as usize])
            .map_err(|_| "nonunit original map chart")?;
        inputs[6] = self
            .program
            .f
            .inv(&prefix[self.torsion_resultant.0 as usize])
            .map_err(|_| "nonunit rational torsion guard")?;
        self.program
            .evaluate_prefix(&inputs, self.program.gates.len())
    }
    pub fn project(&self, values: &[BigUint]) -> Result<Vec<BigUint>> {
        self.program.check(values)?;
        let pivot = if self.r.is_some() { 4 } else { 5 };
        let mut cs = vec![BigUint::zero(); 6];
        cs[pivot] = BigUint::one();
        for (i, slot) in self.coefficient_slots.iter().enumerate() {
            cs[*slot] = values[self.input_wires[i].0 as usize].clone();
        }
        Ok(cs)
    }
}

#[cfg(test)]
mod determinant_tests {
    use super::*;
    fn leibniz(f: &torus_s4::Field, m: &[Vec<BigUint>], cols: &mut Vec<usize>) -> BigUint {
        let row = cols.len();
        if row == m.len() {
            let mut v = BigUint::one();
            let mut inversions = 0;
            for i in 0..cols.len() {
                v = f.mul(&v, &m[i][cols[i]]);
                for j in i + 1..cols.len() {
                    if cols[i] > cols[j] {
                        inversions += 1;
                    }
                }
            }
            return if inversions % 2 == 0 {
                v
            } else {
                f.sub(&0u32.into(), &v)
            };
        }
        let mut sum = BigUint::zero();
        for j in 0..m.len() {
            if !cols.contains(&j) {
                cols.push(j);
                sum = f.add(&sum, &leibniz(f, m, cols));
                cols.pop();
            }
        }
        sum
    }
    #[test]
    fn cached_cofactors_match_independent_permutations() -> Result<()> {
        for n in 1usize..=6 {
            let e = Engine::new(101u32.into(), 10000)?;
            let x = e.input();
            let m: Vec<Vec<_>> = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| {
                            if (i + j) % 4 == 0 {
                                x.a
                            } else {
                                e.real((i * 7 + j * 3 + 2).into()).a
                            }
                        })
                        .collect()
                })
                .collect();
            let d = determinant(&e, &m)?;
            let p = e.builder.into_inner().finish()?;
            for val in [0u32, 1, 7] {
                let values = p.evaluate_prefix(&[val.into()], p.gates.len())?;
                let matrix: Vec<Vec<_>> = m
                    .iter()
                    .map(|r| r.iter().map(|w| values[w.0 as usize].clone()).collect())
                    .collect();
                assert_eq!(values[d.0 as usize], leibniz(&p.f, &matrix, &mut vec![]));
            }
        }
        Ok(())
    }
}
