//! Exact integer-polynomial certificate for the affine-input generic 1+2 branch.
//! It proves formula identities over every commutative ring by coefficient equality.
//! The divisor-class interpretation is stated separately in PROOF_AUDIT.md.
use serde_json::json;
use std::collections::BTreeMap;

const N: usize = 10;
type Monomial = [u8; N];

#[derive(Clone, Debug, PartialEq, Eq)]
struct P(BTreeMap<Monomial, i128>);
impl P {
    fn c(n: i128) -> Self {
        if n == 0 {
            Self(BTreeMap::new())
        } else {
            Self(BTreeMap::from([([0; N], n)]))
        }
    }
    fn var(i: usize) -> Self {
        let mut m = [0; N];
        m[i] = 1;
        Self(BTreeMap::from([(m, 1)]))
    }
    fn add(&self, rhs: &Self) -> Self {
        let mut out = self.0.clone();
        for (m, c) in &rhs.0 {
            let slot = out.entry(*m).or_default();
            *slot = slot.checked_add(*c).expect("coefficient bound");
        }
        out.retain(|_, c| *c != 0);
        Self(out)
    }
    fn neg(&self) -> Self {
        Self(
            self.0
                .iter()
                .map(|(m, c)| (*m, c.checked_neg().expect("coefficient bound")))
                .collect(),
        )
    }
    fn sub(&self, rhs: &Self) -> Self {
        self.add(&rhs.neg())
    }
    fn mul(&self, rhs: &Self) -> Self {
        let mut out = BTreeMap::new();
        for (a, ca) in &self.0 {
            for (b, cb) in &rhs.0 {
                let mut m = [0; N];
                for i in 0..N {
                    m[i] = a[i].checked_add(b[i]).expect("exponent bound");
                }
                let slot = out.entry(m).or_insert(0i128);
                *slot = slot
                    .checked_add(ca.checked_mul(*cb).expect("coefficient bound"))
                    .expect("coefficient bound");
            }
        }
        out.retain(|_, c| *c != 0);
        Self(out)
    }
    fn sq(&self) -> Self {
        self.mul(self)
    }
    fn terms(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct U(Vec<P>);
impl U {
    fn new(mut v: Vec<P>) -> Self {
        while v.last().is_some_and(|p| p.0.is_empty()) {
            v.pop();
        }
        Self(v)
    }
    fn c(p: P) -> Self {
        Self::new(vec![p])
    }
    fn x() -> Self {
        Self::new(vec![P::c(0), P::c(1)])
    }
    fn coeff(&self, i: usize) -> P {
        self.0.get(i).cloned().unwrap_or_else(|| P::c(0))
    }
    fn add(&self, rhs: &Self) -> Self {
        Self::new(
            (0..self.0.len().max(rhs.0.len()))
                .map(|i| self.coeff(i).add(&rhs.coeff(i)))
                .collect(),
        )
    }
    fn sub(&self, rhs: &Self) -> Self {
        Self::new(
            (0..self.0.len().max(rhs.0.len()))
                .map(|i| self.coeff(i).sub(&rhs.coeff(i)))
                .collect(),
        )
    }
    fn scale(&self, p: &P) -> Self {
        Self::new(self.0.iter().map(|c| c.mul(p)).collect())
    }
    fn mul(&self, rhs: &Self) -> Self {
        if self.0.is_empty() || rhs.0.is_empty() {
            return Self::new(vec![]);
        }
        let mut out = vec![P::c(0); self.0.len() + rhs.0.len() - 1];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in rhs.0.iter().enumerate() {
                out[i + j] = out[i + j].add(&a.mul(b));
            }
        }
        Self::new(out)
    }
    fn eval(&self, x: &P) -> P {
        self.0.iter().rev().fold(P::c(0), |a, c| a.mul(x).add(c))
    }
    fn terms(&self) -> usize {
        self.0.iter().map(P::terms).sum()
    }
}

fn main() {
    // Formal independent variables x,y,q,r,s,t,a0,a1,a2,a3.
    let v: Vec<P> = (0..N).map(P::var).collect();
    let (x, y, q, r, s, t) = (&v[0], &v[1], &v[2], &v[3], &v[4], &v[5]);
    let f = U::new(vec![
        v[6].clone(),
        v[7].clone(),
        v[8].clone(),
        v[9].clone(),
        P::c(0),
        P::c(1),
    ]);
    let u = U::new(vec![r.clone(), q.clone(), P::c(1)]);
    let vv = U::new(vec![t.clone(), s.clone()]);
    let mut rem = f.sub(&vv.mul(&vv)).0;
    rem.resize(6, P::c(0));
    let mut h = vec![P::c(0); 4];
    for degree in (2..=5).rev() {
        let lead = rem[degree].clone();
        h[degree - 2] = lead.clone();
        for i in 0..=2 {
            rem[degree - 2 + i] = rem[degree - 2 + i].sub(&lead.mul(&u.coeff(i)));
        }
    }
    let h = U::new(h);
    let c = U::new(vec![rem[0].clone(), rem[1].clone()]);
    assert_eq!(f.sub(&vv.mul(&vv)), u.mul(&h).add(&c));
    assert_eq!(h.coeff(1), v[9].add(&q.sq()).sub(r));

    let k = u.eval(x);
    let a = y.sub(&vv.eval(x));
    let k2 = k.sq();
    let a2 = a.sq();
    let b = s.mul(&k).add(&a.mul(q));
    let ccoef = t.mul(&k).add(&a.mul(r));
    let n = vv.scale(&k).add(&u.scale(&a));
    let qnum = k2.mul(&x.sub(q)).sub(&a2);
    let rscaled = h
        .coeff(1)
        .mul(&k2)
        .sub(&a.mul(&b.add(&s.mul(&k))))
        .add(&qnum.mul(x));
    let uscaled = U::new(vec![rscaled.clone(), qnum.clone(), k2.clone()]);
    let s3 = a.mul(&qnum).sub(&b.mul(&k2));
    let t3 = a.mul(&rscaled.mul(&k2)).sub(&ccoef.mul(&k2.sq()));
    let body = h
        .scale(&k2)
        .sub(&vv.scale(&P::c(2).mul(&k).mul(&a)))
        .sub(&u.scale(&a2));

    // Each assert compares canonical multivariate Z-polynomial coefficients.
    assert_eq!(
        f.scale(&k2).sub(&n.mul(&n)),
        u.mul(&body).add(&c.scale(&k2))
    );
    assert_eq!(
        body.sub(&U::x().sub(&U::c(x.clone())).mul(&uscaled)),
        U::c(body.eval(x))
    );
    assert_eq!(body.eval(x), k.mul(&f.eval(x).sub(&y.sq()).sub(&c.eval(x))));
    assert_eq!(
        n.scale(&k2)
            .sub(&uscaled.scale(&a))
            .scale(&k2)
            .add(&U::new(vec![t3.clone(), k2.mul(&s3)])),
        U::new(vec![])
    );
    // The literal code has r3 = Rscaled*K² and z = K, hence affine R = Rscaled/K².
    assert_eq!(
        rscaled.mul(&k2),
        h.coeff(1)
            .mul(&k2.sq())
            .sub(&a.mul(&b.add(&s.mul(&k))).mul(&k2))
            .add(&qnum.mul(x).mul(&k2))
    );
    println!(
        "{}",
        json!({"schema":"projective-symbolic-1plus2/v1","status":"pass",
            "coefficient_ring":"Z[x,y,q,r,s,t,a0,a1,a2,a3]",
            "exact_integer_polynomial_equalities":7,
            "input_f_minus_v_squared_terms":f.sub(&vv.mul(&vv)).terms(),
            "body_terms":body.terms(),
            "output_u_scaled_terms":uscaled.terms(),
            "scope":"generic affine-input 1+2 branch with K != 0; divisor-class bridge in PROOF_AUDIT.md"})
    );
}
