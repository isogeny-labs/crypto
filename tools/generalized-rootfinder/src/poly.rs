//! Exact rational-root extraction, including repeated and inseparable inputs.
use super::field::{Field, Result, N};
use num_traits::{One, ToPrimitive, Zero};
pub type Poly = Vec<N>;
pub fn trim(mut p: Poly) -> Poly {
    while p.last().is_some_and(N::is_zero) {
        p.pop();
    }
    p
}
pub fn sub(f: &Field, a: &[N], b: &[N]) -> Poly {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| {
                f.sub(
                    a.get(i).unwrap_or(&N::zero()),
                    b.get(i).unwrap_or(&N::zero()),
                )
            })
            .collect(),
    )
}
pub fn mul(f: &Field, a: &[N], b: &[N]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![N::zero(); a.len() + b.len() - 1];
    for (i, a) in a.iter().enumerate() {
        for (j, b) in b.iter().enumerate() {
            out[i + j] = f.add(&out[i + j], &f.mul(a, b));
        }
    }
    trim(out)
}
pub fn div(f: &Field, mut a: Poly, b: &[N]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    a = trim(a);
    let mut q = vec![N::zero(); a.len().saturating_sub(b.len()) + 1];
    let inv = f.inv(b.last().unwrap());
    while a.len() >= b.len() && !a.is_empty() {
        let i = a.len() - b.len();
        let c = f.mul(a.last().unwrap(), &inv);
        q[i] = c.clone();
        for (j, b) in b.iter().enumerate() {
            a[i + j] = f.sub(&a[i + j], &f.mul(&c, b));
        }
        a = trim(a);
    }
    (trim(q), a)
}
pub fn rem(f: &Field, a: Poly, b: &[N]) -> Poly {
    div(f, a, b).1
}
pub fn monic(f: &Field, p: Poly) -> Poly {
    let p = trim(p);
    if p.is_empty() {
        return p;
    }
    let inv = f.inv(p.last().unwrap());
    p.iter().map(|a| f.mul(a, &inv)).collect()
}
pub fn gcd(f: &Field, mut a: Poly, mut b: Poly) -> Poly {
    while !b.is_empty() {
        let r = rem(f, a, &b);
        a = b;
        b = r;
    }
    monic(f, a)
}
pub fn powmod(f: &Field, a: &[N], e: &N, m: &[N]) -> Poly {
    let (mut a, mut e, mut out) = (rem(f, a.to_vec(), m), e.clone(), vec![N::one()]);
    while !e.is_zero() {
        if e.bit(0) {
            out = rem(f, mul(f, &out, &a), m);
        }
        e >>= 1;
        if !e.is_zero() {
            a = rem(f, mul(f, &a, &a), m);
        }
    }
    rem(f, out, m)
}
pub fn eval(f: &Field, p: &[N], x: &N) -> N {
    p.iter()
        .rev()
        .fold(N::zero(), |v, a| f.add(&f.mul(&v, x), a))
}
#[derive(Clone, Debug)]
pub struct SplitBudget {
    pub remaining: u64,
    pub state: u64,
}
impl SplitBudget {
    fn random(&mut self, f: &Field) -> N {
        let mut out = N::zero();
        for _ in 0..f.order.bits().div_ceil(64) {
            self.state ^= self.state << 13;
            self.state ^= self.state >> 7;
            self.state ^= self.state << 17;
            out = (out << 64) + N::from(self.state);
        }
        out % &f.order
    }
}
fn split(f: &Field, g: Poly, budget: &mut SplitBudget, roots: &mut Vec<N>) -> Result<()> {
    if g.len() < 2 {
        return Ok(());
    }
    if g.len() == 2 {
        roots.push(f.neg(&f.mul(&g[0], &f.inv(&g[1]))));
        return Ok(());
    }
    while budget.remaining > 0 {
        budget.remaining -= 1;
        let a: Poly = (0..g.len() - 1).map(|_| budget.random(f)).collect();
        let h = if f.p == N::from(2u8) {
            let mut a = rem(f, a, &g);
            let mut trace = vec![];
            for _ in 0..f.degree {
                trace = sub(f, &trace, &a);
                a = rem(f, mul(f, &a, &a), &g);
            }
            trace
        } else {
            sub(
                f,
                &powmod(f, &a, &((&f.order - N::one()) >> 1), &g),
                &[N::one()],
            )
        };
        let left = gcd(f, g.clone(), h);
        if left.len() > 1 && left.len() < g.len() {
            let (right, r) = div(f, g, &left);
            assert!(r.is_empty());
            split(f, left, budget, roots)?;
            return split(f, right, budget, roots);
        }
    }
    Err("root-splitting trial budget exhausted".into())
}
pub fn roots(
    f: &Field,
    p: Poly,
    max_domain: usize,
    max_degree: usize,
    budget: &mut SplitBudget,
) -> Result<Vec<N>> {
    for a in &p {
        f.validate(a)?;
    }
    let p = trim(p);
    if p.len() > max_degree.saturating_add(1) {
        return Err("domain polynomial degree cap".into());
    }
    if p.is_empty() {
        let q = f
            .order
            .to_usize()
            .filter(|&q| q <= max_domain)
            .ok_or("unrestricted fiber exceeds domain cap")?;
        return Ok((0..q).map(N::from).collect());
    }
    if p.len() == 1 {
        return Ok(vec![]);
    }
    if let Some(q) = f.order.to_usize().filter(|&q| q <= max_domain.min(4096)) {
        return Ok((0..q)
            .map(N::from)
            .filter(|x| eval(f, &p, x).is_zero())
            .collect());
    }
    let x = vec![N::zero(), N::one()];
    let g = gcd(f, p.clone(), sub(f, &powmod(f, &x, &f.order, &p), &x));
    let expected = g.len().saturating_sub(1);
    if expected > max_domain {
        return Err("domain root count exceeds cap".into());
    }
    let mut out = Vec::new();
    split(f, g, budget, &mut out)?;
    out.sort();
    out.dedup();
    if out.len() != expected || out.iter().any(|x| !eval(f, &p, x).is_zero()) {
        return Err("root extraction completeness check failed".into());
    }
    Ok(out)
}
