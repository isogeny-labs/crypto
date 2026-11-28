//! Supplied-coefficient support, local lifting, and unchanged original-row replay.
use crate::arithmetic::{c, encoded, is_zero, trim, z, Engine, Ring, P};
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use torus_coset_circuit::{
    aggregation::TargetCircuit, index::IndexTable, Circuit, Spec, WitnessInput,
};
use torus_s4::{curve25519_prime, Curve, Field, Point};
use torus_s4_tree::Fp2;

fn real(v: &BigUint) -> Fp2 {
    Fp2 {
        a: v.clone(),
        b: BigUint::zero(),
    }
}
fn validate(e: &Engine, s: &Spec) -> Result<Curve> {
    let toy = e.f.p == BigUint::from(101u32);
    let (a, q, ca): (u64, u64, u64) = if toy {
        (17, 3, 95)
    } else {
        (60824497, 213156431, 486662)
    };
    if s.p != e.f.p
        || s.order_factor != a
        || s.q != q
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
        || s.h.pow(&(BigUint::from(a) * q), &e.f) != Fp2::one()
        || s.h.pow(&a.into(), &e.f) == Fp2::one()
        || s.h.pow(&q.into(), &e.f) == Fp2::one()
    {
        return Err("exact order contract".into());
    }
    Ok(Curve {
        f: e.f.clone(),
        a: ca.into(),
    })
}
fn section_norm(e: &Engine, curve: &Curve, r: &Point, cs: &[BigUint]) -> Result<(P, P, P)> {
    if cs.len() != 6 || cs.iter().any(|v| v >= &e.f.p) {
        return Err("noncanonical section coefficients".into());
    }
    let cc: Vec<_> = cs.iter().map(real).collect();
    let cubic = vec![z(), c(1), real(&curve.a), c(1)];
    let (u, v) = if let Some((x, y)) = r {
        if y.is_zero() {
            return Err("unsupported finite two-torsion target chart".into());
        }
        if cs[4].is_zero() {
            return Err("unsupported finite leading section chart".into());
        }
        let line = [e.neg(&real(x)), c(1)];
        (
            e.padd(
                &e.school(&line, &[cc[0].clone(), cc[1].clone(), cc[3].clone()])?,
                &[e.mul(&cc[5], &real(y))?],
            ),
            e.padd(
                &e.school(&line, &[cc[2].clone(), cc[4].clone()])?,
                &[cc[5].clone()],
            ),
        )
    } else {
        if cs[5].is_zero() {
            return Err("unsupported identity leading section chart".into());
        }
        (
            vec![cc[0].clone(), cc[1].clone(), cc[3].clone(), cc[5].clone()],
            vec![cc[2].clone(), cc[4].clone()],
        )
    };
    let mut norm = e.psub(&e.school(&u, &u)?, &e.school(&cubic, &e.school(&v, &v)?)?);
    if let Some((x, _)) = r {
        let (q, rem) = e.divrem(&norm, &[e.neg(&real(x)), c(1)])?;
        if !rem.is_empty() {
            return Err("norm forced factor did not divide".into());
        }
        norm = q;
    }
    norm = e.monic(&norm)?;
    if norm.len() != 7 {
        return Err("section norm is not degree6".into());
    }
    Ok((norm, u, v))
}
pub(crate) fn lambda(r: &Ring<'_>, a: &BigUint) -> Result<P> {
    let half = r.e.mul(&real(a), &r.e.inv(&c(2))?)?;
    let r0 = r.e.neg(&half);
    let (quotient, remainder) = r.e.divrem(&r.s, &[r.e.neg(&r0), c(1)])?;
    let rem = remainder.first().cloned().unwrap_or_else(z);
    if is_zero(&rem) {
        return Err("nonunit original map chart 2X+A".into());
    }
    r.q(&r
        .e
        .scale(&quotient, &r.e.neg(&r.e.mul(&half, &r.e.inv(&rem)?)?))?)
}
fn sixth(r: &Ring<'_>, a: &[Fp2]) -> Result<P> {
    r.pow(a, &6u32.into())
}
fn zero_poly(a: &[Fp2]) -> bool {
    a.iter().all(is_zero)
}

pub struct Query<'a> {
    pub ring: Ring<'a>,
    pub spec: Spec,
    pub r: Point,
    pub target: Point,
    pub u: P,
    pub v: P,
    pub curve: Curve,
    pub rho: Fp2,
    pub coefficients: Vec<BigUint>,
}
impl<'a> Query<'a> {
    pub fn new(
        e: &'a Engine,
        s: &Spec,
        r: Point,
        target: Point,
        coefficients: Vec<BigUint>,
    ) -> Result<Self> {
        let curve = validate(e, s)?;
        if !curve.contains(&r) || !curve.contains(&target) || curve.times(&r, 8)? != target {
            return Err("public [8] pretarget contract".into());
        }
        let (norm, u, v) = section_norm(e, &curve, &r, &coefficients)?;
        let mut ring = Ring::new(e, norm)?;
        let l = lambda(&ring, &curve.a)?;
        let trace = ring.chebyshev(s.q, &l)?;
        ring.tau = ring.sub(
            &ring.scale(&ring.mul(&trace, &trace)?, &c(4))?,
            &ring.constant(c(2)),
        );
        let omega = s.h.pow(&s.q.into(), &e.f);
        let rho = omega.mul(&omega, &e.f);
        Ok(Self {
            ring,
            spec: s.clone(),
            r,
            target,
            u,
            v,
            curve,
            rho,
            coefficients,
        })
    }
    pub fn support(&self) -> Result<Value> {
        let r = &self.ring;
        let e = r.e;
        let x = r.x()?;
        let rational = sixth(r, &r.sub(&r.pow(&x, &e.f.p)?, &x))?;
        let cubic = r.q(&[z(), c(1), real(&self.curve.a), c(1)])?;
        let ordinate = sixth(
            r,
            &r.sub(&r.pow(&cubic, &((&e.f.p - 1u32) >> 1))?, &r.constant(c(1))),
        )?;
        let h = r.h_interval(&self.rho, 1, self.spec.k)?;
        let hs = sixth(r, &h)?;
        if !zero_poly(&rational) || !zero_poly(&ordinate) || !zero_poly(&hs) {
            return Err("supplied section fails support tests; not target absence".into());
        }
        Ok(
            json!({"rational_sixth":encoded(&rational),"ordinate_sixth":encoded(&ordinate),"H_remainder":encoded(&h),"H_sixth":encoded(&hs),"norm":encoded(&r.s)}),
        )
    }
    fn restrict(&self, g: P, start: u64, n: u64, out: &mut Vec<(u64, P)>) -> Result<()> {
        if g.len() <= 1 {
            return Ok(());
        }
        if n == 1 {
            out.push((start, g));
            return Ok(());
        }
        let left = n / 2;
        let h = self.ring.h_interval(&self.rho, start, left)?;
        let h6 = sixth(&self.ring, &h)?;
        let gl = self.ring.e.gcd(&g, &h6)?;
        let (gr, rem) = self.ring.e.divrem(&g, &gl)?;
        if !rem.is_empty() {
            return Err("interval primary partition".into());
        }
        self.restrict(gl, start, left, out)?;
        self.restrict(gr, start + left, n - left, out)
    }
    fn labels(&self) -> Result<Vec<(u64, P)>> {
        let mut out = vec![];
        self.restrict(self.ring.s.clone(), 1, self.spec.k, &mut out)?;
        Ok(out)
    }
    pub fn extract(&self, table: &IndexTable) -> Result<Value> {
        let support = self.support()?;
        let labels = self.labels()?;
        let member = Circuit::compile(&self.spec)?;
        let mut inputs = vec![];
        let mut recovered = vec![];
        let mut raw = BTreeSet::new();
        for (j, g) in &labels {
            for (x, multiplicity) in roots(self.ring.e, g)? {
                let rhs = self.curve.rhs(&x);
                let yy = self
                    .curve
                    .f
                    .sqrt(&rhs)?
                    .ok_or("nonrational ordinate after support test")?;
                if yy.is_zero() {
                    return Err("zero ordinate after support test".into());
                }
                let mut local_sum = 0;
                for y in [yy.clone(), self.curve.f.sub(&0u32.into(), &yy)] {
                    let mut order =
                        local_order(&self.curve.f, &self.curve.a, &self.u, &self.v, &x, &y)?;
                    if let Some((rx, ry)) = &self.r {
                        if *rx == x && self.curve.f.sub(&0u32.into(), ry) == y {
                            order = order
                                .checked_sub(1)
                                .ok_or("missing forced negative-target zero")?;
                        }
                    }
                    local_sum += order;
                    if order == 0 {
                        continue;
                    }
                    let l0 = self.curve.f.mul(
                        &self.curve.a,
                        &self
                            .curve
                            .f
                            .inv(&self.curve.f.add(&self.curve.f.scale(&x, 2), &self.curve.a))?,
                    );
                    let omega_j = self
                        .spec
                        .h
                        .pow(&(BigUint::from(self.spec.q) * j), &self.curve.f);
                    let trace = self.curve.f.chebyshev(self.spec.q.into(), &l0);
                    let (l, branch) = if trace == omega_j.a {
                        (l0, BigUint::zero())
                    } else if self.curve.f.sub(&0u32.into(), &trace) == omega_j.a {
                        (self.curve.f.sub(&0u32.into(), &l0), BigUint::one())
                    } else {
                        return Err("label trace does not replay".into());
                    };
                    let derivative = cheb_derivative(&self.curve.f, self.spec.q, &l);
                    let numerator = self.curve.f.mul(
                        &self.curve.f.sub(&self.curve.f.square(&l), &1u32.into()),
                        &derivative,
                    );
                    let denominator = self.curve.f.mul(
                        &self.curve.f.scale(&omega_j.b, 2),
                        &BigUint::from(self.spec.q),
                    );
                    let imag = self
                        .curve
                        .f
                        .mul(&numerator, &self.curve.f.inv(&denominator)?);
                    let input = WitnessInput {
                        z: Fp2 { a: l, b: imag },
                        selector: j - 1,
                        slack: self.spec.k - j,
                        branch,
                        raw_v: y.clone(),
                    };
                    let values = member.witness(&input)?;
                    member.system.check(&values)?;
                    let decoded = table.decode(&self.spec, &member, &values)?;
                    if order > 1 || !raw.insert((decoded.j, decoded.t)) {
                        return Err("repeated original raw label; not a distinct row".into());
                    }
                    recovered.push(json!({"j":decoded.j,"t":decoded.t,"sign":decoded.coefficient,"x":x.to_string(),"raw_y":y.to_string(),"local_order":order}));
                    inputs.push(input);
                }
                if local_sum != multiplicity {
                    return Err("local/norm multiplicity mismatch".into());
                }
            }
        }
        if inputs.len() != 6 {
            return Err("did not recover six original members".into());
        }
        let aggregate = TargetCircuit::compile(&self.spec, 6, self.target.clone(), true)?;
        let witness = aggregate.witness(&inputs)?;
        aggregate.system.check(&witness)?;
        let row = aggregate.decode_row(&witness, table)?;
        Ok(
            json!({"support":support,"labels":labels.iter().map(|(j,g)|json!({"j":j,"primary":encoded(g)})).collect::<Vec<_>>(),"recovered":recovered,"row":row.iter().map(|t|json!({"j":t.j,"t":t.t,"coefficient":t.coefficient,"point":t.point.as_ref().map(|(x,y)|[x.to_string(),y.to_string()])})).collect::<Vec<_>>(),"original_witness":witness.iter().map(ToString::to_string).collect::<Vec<_>>(),"public_R":point_json(&self.r),"public_Q":point_json(&self.target),"coefficients":self.coefficients.iter().map(ToString::to_string).collect::<Vec<_>>(),"original_replay_checked":true,"index_table":table.stats(),"counts":self.ring.e.stats()}),
        )
    }
}
pub fn point_json(p: &Point) -> Value {
    json!(p.as_ref().map(|(x, y)| [x.to_string(), y.to_string()]))
}

fn roots(e: &Engine, s: &[Fp2]) -> Result<Vec<(BigUint, usize)>> {
    let derivative: Result<P> = s
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| e.mul(x, &c(i as u64)))
        .collect();
    let repeated = e.gcd(s, &derivative?)?;
    let (sf, rem) = e.divrem(s, &repeated)?;
    if !rem.is_empty() {
        return Err("squarefree division".into());
    }
    let mut stack = vec![sf];
    let mut xs = vec![];
    while let Some(g) = stack.pop() {
        if g.len() == 2 {
            let x = e.neg(&e.mul(&g[0], &e.inv(&g[1])?)?);
            if !x.b.is_zero() {
                return Err("nonreal support root".into());
            }
            xs.push(x);
            continue;
        }
        if g.len() <= 1 {
            continue;
        }
        let r = Ring::new(e, e.monic(&g)?)?;
        let mut split = None;
        for shift in 0..64 {
            let lin = vec![c(shift), c(1)];
            let zero = e.gcd(&g, &lin)?;
            if zero.len() > 1 && zero.len() < g.len() {
                split = Some(zero);
                break;
            }
            let p = r.pow(&r.q(&lin)?, &((&e.f.p - 1u32) >> 1))?;
            let h = e.gcd(&g, &r.sub(&p, &r.constant(c(1))))?;
            if h.len() > 1 && h.len() < g.len() {
                split = Some(h);
                break;
            }
        }
        let h = split.ok_or("factor split cap; incomplete, not absence")?;
        let (other, rem) = e.divrem(&g, &h)?;
        if !rem.is_empty() {
            return Err("factor division".into());
        }
        stack.push(h);
        stack.push(other);
    }
    xs.sort_by(|a, b| a.a.cmp(&b.a));
    let mut remaining = s.to_vec();
    let mut out = vec![];
    for x in xs {
        let line = [e.neg(&x), c(1)];
        let mut count = 0;
        loop {
            let (q, r) = e.divrem(&remaining, &line)?;
            if !r.is_empty() {
                break;
            }
            remaining = q;
            count += 1;
            if remaining.len() <= 1 {
                break;
            }
        }
        if count == 0 {
            return Err("zero root multiplicity".into());
        }
        out.push((x.a, count));
    }
    if trim(remaining).len() != 1 || out.iter().map(|(_, m)| m).sum::<usize>() != s.len() - 1 {
        return Err("incomplete support factorization".into());
    }
    Ok(out)
}
fn local_order(
    f: &Field,
    a: &BigUint,
    u: &[Fp2],
    v: &[Fp2],
    x: &BigUint,
    y: &BigUint,
) -> Result<usize> {
    const N: usize = 8;
    let mut ys = vec![BigUint::zero(); N];
    ys[0] = y.clone();
    let rhs = [
        f.add(&f.add(&f.mul(&f.square(x), x), &f.mul(a, &f.square(x))), x),
        f.add(
            &f.add(&f.scale(&f.square(x), 3), &f.scale(&f.mul(a, x), 2)),
            &1u32.into(),
        ),
        f.add(&f.scale(x, 3), a),
        1u32.into(),
    ];
    let inv = f.inv(&f.scale(y, 2))?;
    for n in 1..N {
        let mut cross = BigUint::zero();
        for i in 1..n {
            cross = f.add(&cross, &f.mul(&ys[i], &ys[n - i]));
        }
        ys[n] = f.mul(&f.sub(rhs.get(n).unwrap_or(&BigUint::zero()), &cross), &inv);
    }
    let series = |p: &[Fp2]| {
        let mut out = vec![BigUint::zero(); N];
        for coefficient in p.iter().rev() {
            let mut next = vec![BigUint::zero(); N];
            for i in 0..N {
                next[i] = f.mul(&out[i], x);
                if i > 0 {
                    next[i] = f.add(&next[i], &out[i - 1]);
                }
            }
            next[0] = f.add(&next[0], &coefficient.a);
            out = next;
        }
        out
    };
    let us = series(u);
    let vs = series(v);
    let mut g = us;
    for n in 0..N {
        for i in 0..=n {
            g[n] = f.add(&g[n], &f.mul(&vs[i], &ys[n - i]));
        }
    }
    g.iter()
        .position(|x| !x.is_zero())
        .ok_or("local series exhausted; incomplete".into())
}
fn cheb_derivative(f: &Field, n: u64, x: &BigUint) -> BigUint {
    let (mut a, mut b, mut da, mut db) =
        (BigUint::one(), x.clone(), BigUint::zero(), BigUint::one());
    for bit in (0..64 - n.leading_zeros()).rev() {
        let ab = f.sub(&f.scale(&f.mul(&a, &b), 2), x);
        let dab = f.sub(
            &f.scale(&f.add(&f.mul(&da, &b), &f.mul(&a, &db)), 2),
            &1u32.into(),
        );
        if (n >> bit) & 1 == 0 {
            b = ab;
            db = dab;
            da = f.scale(&f.mul(&a, &da), 4);
            a = f.sub(&f.scale(&f.square(&a), 2), &1u32.into());
        } else {
            a = ab;
            da = dab;
            db = f.scale(&f.mul(&b, &db), 4);
            b = f.sub(&f.scale(&f.square(&b), 2), &1u32.into());
        }
    }
    da
}

pub fn admitted_moduli() -> [BigUint; 2] {
    [101u32.into(), curve25519_prime()]
}
