//! Small construction polynomials, following the frozen pkm_tower_pilot convention.
//! No F4 engine is imported or called. Coefficient/evaluation controls are separate.
use std::collections::BTreeMap;
pub fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn addm(a: u64, b: u64, p: u64) -> u64 {
    (a + b) % p
}
pub fn subm(a: u64, b: u64, p: u64) -> u64 {
    (a + p - b % p) % p
}
pub fn powm(mut a: u64, mut e: u32, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pol {
    pub n: usize,
    pub terms: BTreeMap<Vec<u32>, u64>,
}
impl Pol {
    pub fn zero(n: usize) -> Self {
        Self {
            n,
            terms: BTreeMap::new(),
        }
    }
    pub fn constant(n: usize, c: u64, p: u64) -> Self {
        let mut f = Self::zero(n);
        if c % p != 0 {
            f.terms.insert(vec![0; n], c % p);
        }
        f
    }
    pub fn var(n: usize, i: usize) -> Self {
        assert!(i < n);
        let mut e = vec![0; n];
        e[i] = 1;
        let mut f = Self::zero(n);
        f.terms.insert(e, 1);
        f
    }
    pub fn add(&self, o: &Self, p: u64) -> Self {
        assert_eq!(self.n, o.n);
        let mut f = self.clone();
        for (e, c) in &o.terms {
            let v = f.terms.entry(e.clone()).or_insert(0);
            *v = addm(*v, *c, p);
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
    pub fn scale(&self, k: u64, p: u64) -> Self {
        let mut f = Self::zero(self.n);
        for (e, c) in &self.terms {
            let v = mulm(*c, k, p);
            if v != 0 {
                f.terms.insert(e.clone(), v);
            }
        }
        f
    }
    pub fn sub(&self, o: &Self, p: u64) -> Self {
        self.add(&o.scale(p - 1, p), p)
    }
    pub fn mul(&self, o: &Self, p: u64) -> Self {
        assert_eq!(self.n, o.n);
        let mut f = Self::zero(self.n);
        for (e1, c1) in &self.terms {
            for (e2, c2) in &o.terms {
                let e = e1
                    .iter()
                    .zip(e2)
                    .map(|(a, b)| a.checked_add(*b).expect("exponent overflow"))
                    .collect();
                let v = f.terms.entry(e).or_insert(0);
                *v = addm(*v, mulm(*c1, *c2, p), p);
            }
        }
        f.terms.retain(|_, c| *c != 0);
        f
    }
    pub fn pow(&self, mut e: u32, p: u64) -> Self {
        let mut r = Self::constant(self.n, 1, p);
        let mut a = self.clone();
        while e > 0 {
            if e & 1 == 1 {
                r = r.mul(&a, p);
            }
            a = a.mul(&a, p);
            e >>= 1;
        }
        r
    }
    pub fn eval(&self, x: &[u64], p: u64) -> u64 {
        assert_eq!(x.len(), self.n);
        self.terms.iter().fold(0, |acc, (e, c)| {
            let term = e
                .iter()
                .zip(x)
                .fold(*c, |v, (&k, &xi)| mulm(v, powm(xi, k, p), p));
            addm(acc, term, p)
        })
    }
    pub fn degree(&self) -> u32 {
        self.terms.keys().map(|e| e.iter().sum()).max().unwrap_or(0)
    }
    pub fn substitute(&self, images: &[Self], target_n: usize, p: u64) -> Self {
        assert_eq!(images.len(), self.n);
        assert!(images.iter().all(|f| f.n == target_n));
        let mut out = Self::zero(target_n);
        for (e, c) in &self.terms {
            let mut term = Self::constant(target_n, *c, p);
            for (i, &k) in e.iter().enumerate() {
                if k > 0 {
                    term = term.mul(&images[i].pow(k, p), p);
                }
            }
            out = out.add(&term, p);
        }
        out
    }
    pub fn terms_vec(&self) -> Vec<(Vec<u32>, u64)> {
        self.terms.iter().map(|(e, c)| (e.clone(), *c)).collect()
    }
    pub fn expression(&self, names: &[String]) -> String {
        assert_eq!(names.len(), self.n);
        if self.terms.is_empty() {
            return "0".into();
        }
        self.terms
            .iter()
            .map(|(e, c)| {
                let mut s = c.to_string();
                for (i, &k) in e.iter().enumerate() {
                    if k == 1 {
                        s.push_str(&format!("*{}", names[i]));
                    } else if k > 1 {
                        s.push_str(&format!("*{}^{k}", names[i]));
                    }
                }
                s
            })
            .collect::<Vec<_>>()
            .join("+")
    }
}
pub fn s3(x: &Pol, y: &Pol, z: &Pol, a: u64, b: u64, p: u64) -> Pol {
    let n = x.n;
    let d = x.sub(y, p);
    let s = x.add(y, p);
    let xy = x.mul(y, p);
    let inner = s
        .mul(&xy.add(&Pol::constant(n, a, p), p), p)
        .add(&Pol::constant(n, mulm(2, b, p), p), p);
    let q = xy.sub(&Pol::constant(n, a, p), p);
    d.pow(2, p)
        .mul(&z.pow(2, p), p)
        .add(&inner.mul(z, p).scale(p - 2, p), p)
        .add(&q.pow(2, p).sub(&s.scale(mulm(4, b, p), p), p), p)
}
/// Symmetric scalar formula, independently arranged from the builder expression.
pub fn s3_scalar(x: u64, y: u64, z: u64, a: u64, b: u64, p: u64) -> u64 {
    let s = addm(addm(x, y, p), z, p);
    let q = subm(
        addm(addm(mulm(x, y, p), mulm(x, z, p), p), mulm(y, z, p), p),
        a,
        p,
    );
    subm(
        subm(
            mulm(q, q, p),
            mulm(4, mulm(mulm(mulm(x, y, p), z, p), s, p), p),
            p,
        ),
        mulm(4, mulm(b, s, p), p),
        p,
    )
}
