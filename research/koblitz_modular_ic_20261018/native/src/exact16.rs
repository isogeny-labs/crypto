//! Small exact rational and polynomial arithmetic for bounded round16 certificates.
use num_bigint::BigInt as Z;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Q {
    pub n: Z,
    pub d: Z,
}
impl Q {
    pub fn new(n: Z, d: Z) -> Self {
        assert!(!d.is_zero());
        let g = n.gcd(&d) * d.signum();
        let s = Self {
            n: n / &g,
            d: d / &g,
        };
        assert!(s.n.bits() < 20000 && s.d.bits() < 20000);
        s
    }
    pub fn z(n: i64) -> Self {
        Self::new(n.into(), Z::one())
    }
    pub fn add(&self, b: &Self) -> Self {
        Self::new(&self.n * &b.d + &b.n * &self.d, &self.d * &b.d)
    }
    pub fn neg(&self) -> Self {
        Self::new(-&self.n, self.d.clone())
    }
    pub fn sub(&self, b: &Self) -> Self {
        self.add(&b.neg())
    }
    pub fn mul(&self, b: &Self) -> Self {
        Self::new(&self.n * &b.n, &self.d * &b.d)
    }
    pub fn div(&self, b: &Self) -> Self {
        Self::new(&self.n * &b.d, &self.d * &b.n)
    }
    pub fn zero(&self) -> bool {
        self.n.is_zero()
    }
    pub fn integer(&self) -> Z {
        assert!(self.d.is_one());
        self.n.clone()
    }
    pub fn cmp(&self, b: &Self) -> std::cmp::Ordering {
        (&self.n * &b.d).cmp(&(&b.n * &self.d))
    }
    pub fn str(&self) -> String {
        if self.d.is_one() {
            self.n.to_string()
        } else {
            format!("({}/{})", self.n, self.d)
        }
    }
}
pub type P = Vec<Q>;
pub fn trim(mut p: P) -> P {
    while p.last().is_some_and(Q::zero) {
        p.pop();
    }
    p
}
pub fn add(a: &P, b: &P) -> P {
    trim(
        (0..a.len().max(b.len()))
            .map(|i| {
                a.get(i)
                    .unwrap_or(&Q::z(0))
                    .add(b.get(i).unwrap_or(&Q::z(0)))
            })
            .collect(),
    )
}
pub fn neg(a: &P) -> P {
    a.iter().map(Q::neg).collect()
}
pub fn mul(a: &P, b: &P) -> P {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![Q::z(0); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] = c[i + j].add(&x.mul(y));
        }
    }
    trim(c)
}
pub fn div(a: &P, b: &P) -> (P, P) {
    assert!(!b.is_empty());
    let mut r = a.clone();
    let mut q = vec![Q::z(0); a.len().saturating_sub(b.len()) + 1];
    while r.len() >= b.len() {
        let i = r.len() - b.len();
        let t = r.last().unwrap().div(b.last().unwrap());
        q[i] = t.clone();
        for j in 0..b.len() {
            r[i + j] = r[i + j].sub(&t.mul(&b[j]));
        }
        r = trim(r);
    }
    (trim(q), r)
}
pub fn det(mut a: Vec<Vec<Z>>) -> Z {
    let n = a.len();
    let mut last = Z::one();
    let mut sign = Z::one();
    for k in 0..n - 1 {
        let Some(p) = (k..n).find(|i| !a[*i][k].is_zero()) else {
            return Z::zero();
        };
        if p != k {
            a.swap(p, k);
            sign = -sign;
        }
        let pivot = a[k][k].clone();
        for i in k + 1..n {
            for j in k + 1..n {
                let t = &a[i][j] * &pivot - &a[i][k] * &a[k][j];
                let (q, r) = t.div_rem(&last);
                assert!(r.is_zero());
                a[i][j] = q;
                assert!(a[i][j].bits() < 20000);
            }
            a[i][k] = Z::zero();
        }
        last = pivot;
    }
    sign * &a[n - 1][n - 1]
}
