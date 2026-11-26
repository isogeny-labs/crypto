//! Public-input compact coset spectrum. Construction is not root discovery.
pub mod backend;
pub mod controls;
pub mod ir;
use ir::{Program, E, W};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use torus_coset_circuit::Spec;
use torus_s4::{curve25519_prime, Curve, Point};
use torus_s4_tree::Fp2;
pub type Result<T> = std::result::Result<T, String>;
type P = Vec<W>;
fn add(e: &E, a: &[W], b: &[W]) -> P {
    (0..a.len().max(b.len()))
        .map(|i| e.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}
fn neg(e: &E, a: &[W]) -> P {
    a.iter().map(|x| e.neg(*x)).collect()
}
fn scale(e: &E, a: &[W], b: W) -> P {
    a.iter().map(|x| e.mul(*x, b)).collect()
}
fn school(e: &E, a: &[W], b: &[W]) -> P {
    let mut p = vec![0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            p[i + j] = e.add(p[i + j], e.mul(*x, *y));
        }
    }
    p
}
fn div_linear(e: &E, a: &[W], root: W) -> (P, W) {
    let mut q = vec![0; a.len() - 1];
    q[a.len() - 2] = a[a.len() - 1];
    for i in (1..a.len() - 1).rev() {
        q[i - 1] = e.add(a[i], e.mul(root, q[i]));
    }
    (q.clone(), e.add(a[0], e.mul(root, q[0])))
}
fn reduce(e: &E, s: &[W], a: &[W]) -> P {
    let mut p = a.to_vec();
    p.resize(p.len().max(6), 0);
    for i in (6..p.len()).rev() {
        for j in 0..6 {
            p[i - 6 + j] = e.sub(p[i - 6 + j], e.mul(p[i], s[j]));
        }
    }
    p.truncate(6);
    p
}
fn mul(e: &E, s: &[W], a: &[W], b: &[W]) -> P {
    reduce(e, s, &school(e, a, b))
}
fn chebyshev(e: &E, s: &[W], x: &[W], n: u64) -> P {
    let one = vec![1, 0, 0, 0, 0, 0];
    let mut a = one.clone();
    let mut b = x.to_vec();
    for i in (0..64 - n.leading_zeros()).rev() {
        let aa = add(e, &scale(e, &mul(e, s, &a, &a), e.c(2)), &neg(e, &one));
        let ab = add(e, &scale(e, &mul(e, s, &a, &b), e.c(2)), &neg(e, x));
        if (n >> i) & 1 == 0 {
            a = aa;
            b = ab;
        } else {
            a = ab;
            b = add(e, &scale(e, &mul(e, s, &b, &b), e.c(2)), &neg(e, &one));
        }
    }
    a
}
/// Constant dimension, cached cofactors; no unknown pivot or division.
fn determinant(e: &E, m: &[P]) -> W {
    let n = m.len();
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;
    for mask in 1usize..1 << n {
        let row = mask.count_ones() as usize - 1;
        let mut pos = 0;
        for j in 0..n {
            if mask >> j & 1 == 0 {
                continue;
            }
            let term = e.mul(m[row][j], dp[mask ^ (1 << j)]);
            dp[mask] = if (row + pos).is_multiple_of(2) {
                e.add(dp[mask], term)
            } else {
                e.sub(dp[mask], term)
            };
            pos += 1;
        }
    }
    dp[(1 << n) - 1]
}
fn resultant(e: &E, s: &[W], g: &[W]) -> W {
    let mut m = vec![vec![0; 6]; 6];
    for col in 0..6 {
        let mut basis = vec![0; 6];
        basis[col] = 1;
        for (row, w) in mul(e, s, g, &basis).into_iter().enumerate() {
            m[row][col] = w;
        }
    }
    determinant(e, &m)
}
/// Newton sums in the candidate quotient, not in the coefficient solution ideal.
fn characteristic(e: &E, s: &[W], g: &[W]) -> P {
    let mut tx = vec![e.c(6)];
    for k in 1..6 {
        let mut sum = e.mul(e.c(k as u64), s[6 - k]);
        for i in 1..k {
            sum = e.add(sum, e.mul(s[6 - i], tx[k - i]));
        }
        tx.push(e.neg(sum));
    }
    let mut powers = vec![0];
    let mut gp = vec![1, 0, 0, 0, 0, 0];
    let mut chi = vec![0; 7];
    chi[6] = 1;
    for k in 1..=6 {
        gp = mul(e, s, &gp, g);
        let mut tr = 0;
        for i in 0..6 {
            tr = e.add(tr, e.mul(gp[i], tx[i]));
        }
        powers.push(tr);
        let mut sum = tr;
        for i in 1..k {
            sum = e.add(sum, e.mul(chi[6 - i], powers[k - i]));
        }
        let inv = e.constant(
            e.f.inv(&BigUint::from(k))
                .expect("fixed prime field has characteristic >6"),
        );
        chi[6 - k] = e.neg(e.mul(inv, sum));
    }
    chi
}
fn fp2mul(e: &E, a: (W, W), b: (W, W)) -> (W, W) {
    // The inherited quadratic extension has u^2=2, not u^2=-1.
    (
        e.add(e.mul(a.0, b.0), e.mul(e.c(2), e.mul(a.1, b.1))),
        e.add(e.mul(a.0, b.1), e.mul(a.1, b.0)),
    )
}
fn validate(e: &E, s: &Spec, r: &Point, q: &Point) -> Result<Curve> {
    let toy = s.p == BigUint::from(101u32);
    let (a, n, ca) = if toy {
        (17, 3, 95)
    } else {
        (60824497, 213156431, 486662)
    };
    if (!toy && s.p != curve25519_prime())
        || s.order_factor != a
        || s.q != n
        || s.curve_a != Some(BigUint::from(ca as u64))
        || s.k == 0
        || s.k > 65536
        || 2 * s.k >= a
        || s.h.a >= s.p
        || s.h.b >= s.p
    {
        return Err("unsupported/noncanonical original parameter contract".into());
    }
    if s.h.norm(&e.f) != BigUint::one()
        || s.h.pow(&(BigUint::from(a) * n), &e.f) != Fp2::one()
        || s.h.pow(&a.into(), &e.f) == Fp2::one()
        || s.h.pow(&n.into(), &e.f) == Fp2::one()
    {
        return Err("original exact-order contract".into());
    }
    let curve = Curve {
        f: e.f.clone(),
        a: (ca as u64).into(),
    };
    if [r, q]
        .iter()
        .filter_map(|p| p.as_ref())
        .any(|(x, y)| x >= &s.p || y >= &s.p)
        || !curve.contains(r)
        || !curve.contains(q)
        || curve.times(r, 8)? != *q
    {
        return Err("public point or [8] contract".into());
    }
    if r.as_ref().is_some_and(|(_, y)| y.is_zero()) {
        return Err("unsupported finite2-torsion chart".into());
    }
    let plus = e.f.add(&curve.a, &2u32.into());
    let minus = e.f.sub(&curve.a, &2u32.into());
    if plus.is_zero()
        || minus.is_zero()
        || e.f.sqrt(&plus)?.is_none()
        || minus.modpow(&((&s.p - 1u32) >> 1), &s.p) != &s.p - 1u32
    {
        return Err("original rational torsion hypotheses".into());
    }
    Ok(curve)
}
pub struct Compiled {
    pub program: Program,
    pub spec: Spec,
    pub r: Point,
    pub target: Point,
    pub coefficient_slots: Vec<usize>,
    pub s: P,
    pub g: P,
    pub characteristic: P,
    pub labels: P,
    pub map_remainder: W,
    pub torsion_resultant: W,
    pub squarefree_resultant: W,
    pub bit_length: usize,
}
/// Only public problem data. No label/section/witness tuple is accepted here.
pub fn compile(spec: &Spec, r: Point, target: Point, cap: usize) -> Result<Compiled> {
    let e = E::new(spec.p.clone(), cap)?;
    let curve = validate(&e, spec, &r, &target)?;
    let pivot = if r.is_some() { 4 } else { 5 };
    let slots: Vec<_> = (0..6).filter(|i| *i != pivot).collect();
    let mut cs = [0; 6];
    cs[pivot] = 1;
    for i in &slots {
        cs[*i] = e.input();
    }
    let beta = e.input();
    let eta = e.input();
    let delta = e.input();
    let cubic = vec![0, 1, e.constant(curve.a.clone()), 1];
    let (u, v) = if let Some((x, y)) = &r {
        let line = vec![e.neg(e.constant(x.clone())), 1];
        (
            add(
                &e,
                &school(&e, &line, &[cs[0], cs[1], cs[3]]),
                &[e.mul(cs[5], e.constant(y.clone()))],
            ),
            add(&e, &school(&e, &line, &[cs[2], cs[4]]), &[cs[5]]),
        )
    } else {
        (vec![cs[0], cs[1], cs[3], cs[5]], vec![cs[2], cs[4]])
    };
    let norm = add(
        &e,
        &school(&e, &u, &u),
        &neg(&e, &school(&e, &cubic, &school(&e, &v, &v))),
    );
    let s = if let Some((x, _)) = &r {
        let (p, rem) = div_linear(&e, &norm, e.constant(x.clone()));
        e.equal(rem, 0);
        neg(&e, &p)
    } else {
        norm
    };
    if s.len() != 7 || s[6] != 1 {
        return Err("monic sextic compiler invariant".into());
    }
    let half = e.f.mul(&curve.a, &e.f.inv(&2u32.into())?);
    let (d, rem) = div_linear(&e, &s, e.constant(e.f.sub(&0u32.into(), &half)));
    e.equal(e.mul(beta, rem), 1);
    let lambda = reduce(&e, &s, &scale(&e, &d, e.neg(e.mul(e.constant(half), beta))));
    let ja = school(
        &e,
        &[e.neg(1), 1],
        &[
            1,
            e.neg(e.c(4)),
            e.neg(e.add(e.c(2), e.mul(e.c(4), e.constant(curve.a.clone())))),
            e.neg(e.c(4)),
            1,
        ],
    );
    let torsion = resultant(&e, &s, &ja);
    e.equal(e.mul(eta, torsion), 1);
    let derivative: Vec<_> = (1..=6).map(|i| e.mul(e.c(i as u64), s[i])).collect();
    let sf = resultant(&e, &s, &derivative);
    e.equal(e.mul(delta, sf), 1);
    let t = chebyshev(&e, &s, &lambda, spec.q);
    let g = mul(&e, &s, &t, &t);
    let chi = characteristic(&e, &s, &g);
    let bits = (64 - (spec.k - 1).leading_zeros()).max(1) as usize;
    if (BigUint::from(2u32) * ((BigUint::one() << bits) - 1u32)) >= spec.p {
        return Err("range no-wrap premise".into());
    }
    let omega = spec.h.pow(&spec.q.into(), &e.f);
    let mut want = vec![1];
    let mut labels = vec![];
    for _ in 0..6 {
        let us: Vec<_> = (0..bits).map(|_| e.input()).collect();
        let vs: Vec<_> = (0..bits).map(|_| e.input()).collect();
        let uw = e.word(&us);
        let vw = e.word(&vs);
        e.equal(e.add(uw, vw), e.c(spec.k - 1));
        labels.push(e.add(1, uw));
        let mut z = (e.constant(omega.a.clone()), e.constant(omega.b.clone()));
        let mut power = omega.clone();
        for b in us {
            let selected = (
                e.add(1, e.mul(b, e.constant(e.f.sub(&power.a, &1u32.into())))),
                e.mul(b, e.constant(power.b.clone())),
            );
            z = fp2mul(&e, z, selected);
            power = power.mul(&power, &e.f);
        }
        want = school(&e, &want, &[e.neg(e.mul(z.0, z.0)), 1]);
    }
    for i in 0..6 {
        e.equal(chi[i], want[i]);
    }
    let program = e.finish()?;
    Ok(Compiled {
        program,
        spec: spec.clone(),
        r,
        target,
        coefficient_slots: slots,
        s,
        g,
        characteristic: chi,
        labels,
        map_remainder: rem,
        torsion_resultant: torsion,
        squarefree_resultant: sf,
        bit_length: bits,
    })
}
impl Compiled {
    /// Extension only: supplied tuples cannot enter public compilation.
    pub fn extend(&self, coefficients: &[BigUint], labels: &[u64]) -> Result<Vec<BigUint>> {
        let pivot = if self.r.is_some() { 4 } else { 5 };
        if coefficients.len() != 6
            || coefficients[pivot] != BigUint::one()
            || coefficients.iter().any(|x| x >= &self.spec.p)
            || labels.len() != 6
            || labels.iter().any(|j| *j == 0 || *j > self.spec.k)
        {
            return Err("invalid extension tuple".into());
        }
        let mut inputs: Vec<_> = self
            .coefficient_slots
            .iter()
            .map(|i| coefficients[*i].clone())
            .collect();
        inputs.extend([BigUint::zero(), BigUint::zero(), BigUint::zero()]);
        for j in labels {
            for v in [j - 1, self.spec.k - j] {
                for b in 0..self.bit_length {
                    inputs.push(BigUint::from((v >> b) & 1));
                }
            }
        }
        let initial = self.program.evaluate(&inputs)?;
        for (i, w) in [
            self.map_remainder,
            self.torsion_resultant,
            self.squarefree_resultant,
        ]
        .into_iter()
        .enumerate()
        {
            inputs[5 + i] = self.program.f.inv(&initial[w])?;
        }
        let values = self.program.evaluate(&inputs)?;
        self.program.check(&values)?;
        Ok(values)
    }
}
