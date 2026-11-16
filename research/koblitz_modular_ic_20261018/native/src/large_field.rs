//! Exact binary fields beyond the historical u32 helper. No external CAS.
use crate::binary_ecc::{self, BinaryCurve, BinaryPoint, F2mElement, IrreduciblePoly};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::sync::OnceLock;
pub type E = F2mElement;
pub type P = BinaryPoint;
pub type Poly = Vec<E>;
pub struct F {
    pub irr: IrreduciblePoly,
    qrows: OnceLock<(Vec<Vec<u64>>, Vec<usize>)>,
}
fn rem(mut a: BigUint, b: &BigUint) -> BigUint {
    while !a.is_zero() && a.bits() >= b.bits() {
        let s = a.bits() - b.bits();
        a ^= b << s;
    }
    a
}
fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        let r = rem(a, &b);
        a = b;
        b = r;
    }
    a
}
impl F {
    pub fn new(m: u32, low: u64) -> Self {
        Self {
            irr: IrreduciblePoly {
                degree: m,
                low_terms: (0..64).filter(|i| low & (1u64 << i) != 0).collect(),
            },
            qrows: OnceLock::new(),
        }
    }
    pub fn m(&self) -> u32 {
        self.irr.degree
    }
    pub fn z(&self) -> E {
        E::zero(self.m())
    }
    pub fn o(&self) -> E {
        E::one(self.m())
    }
    pub fn e(&self, n: u64) -> E {
        E::from_biguint(&BigUint::from(n), self.m())
    }
    pub fn mul(&self, a: &E, b: &E) -> E {
        a.mul(b, &self.irr)
    }
    pub fn sq(&self, a: &E) -> E {
        a.square(&self.irr)
    }
    pub fn frob(&self, a: &E, n: u32) -> E {
        a.square_k_times(n, &self.irr)
    }
    pub fn inv(&self, a: &E) -> E {
        if *a == self.o() {
            return a.clone();
        }
        a.flt_inverse(&self.irr).expect("nonzero denominator")
    }
    pub fn pow(&self, a: &E, n: &BigUint) -> E {
        let mut v = self.o();
        for i in (0..n.bits()).rev() {
            v = self.sq(&v);
            if n.bit(i) {
                v = self.mul(&v, a);
            }
        }
        v
    }
    pub fn powu(&self, a: &E, n: u64) -> E {
        self.pow(a, &BigUint::from(n))
    }
    pub fn modulus(&self) -> BigUint {
        self.irr
            .low_terms
            .iter()
            .fold(BigUint::one() << self.m(), |v, i| {
                v | (BigUint::one() << *i)
            })
    }
    pub fn irreducible(&self) -> bool {
        let x = self.e(2);
        let mut v = x.clone();
        let m = self.m();
        for i in 1..=m {
            v = self.sq(&v);
            if i < m && m % i == 0 && gcd(v.add(&x).to_biguint(), self.modulus()) != BigUint::one()
            {
                return false;
            }
        }
        v == x
    }
    pub fn find(m: u32) -> Self {
        for low in (1..4096).step_by(2) {
            let f = Self::new(m, low);
            if f.irreducible() {
                println!("field F2^{m}: low={low}");
                return f;
            }
        }
        panic!("irreducible search budget");
    }
    pub fn quadratic(&self, c: &E) -> Option<E> {
        if c.trace(&self.irr) != 0 {
            return None;
        }
        let n = self.m() as usize;
        let (rows, piv) = self.qrows.get_or_init(|| {
            let mut a = vec![vec![0u64; (2 * n).div_ceil(64)]; n];
            for j in 0..n {
                let x = E::from_bit_positions(&[j as u32], self.m());
                let v = self.sq(&x).add(&x);
                for (i, r) in a.iter_mut().enumerate() {
                    if v.raw_bits()[i / 64] >> (i % 64) & 1 == 1 {
                        r[j / 64] |= 1 << (j % 64);
                    }
                }
            }
            for (i, r) in a.iter_mut().enumerate() {
                r[(n + i) / 64] |= 1 << ((n + i) % 64);
            }
            let mut k = 0;
            let mut piv = vec![];
            for j in 0..n {
                if let Some(i) = (k..n).find(|i| a[*i][j / 64] >> (j % 64) & 1 == 1) {
                    a.swap(k, i);
                    for i in 0..n {
                        if i != k && a[i][j / 64] >> (j % 64) & 1 == 1 {
                            for w in 0..a[i].len() {
                                a[i][w] ^= a[k][w];
                            }
                        }
                    }
                    piv.push(j);
                    k += 1;
                }
            }
            assert_eq!(k, n - 1);
            (a, piv)
        });
        let mut positions = vec![];
        for (i, j) in piv.iter().enumerate() {
            let mut v = 0;
            for k in 0..n {
                v ^= ((rows[i][(n + k) / 64] >> ((n + k) % 64)) & 1)
                    * ((c.raw_bits()[k / 64] >> (k % 64)) & 1);
            }
            if v == 1 {
                positions.push(*j as u32);
            }
        }
        let x = E::from_bit_positions(&positions, self.m());
        assert_eq!(self.sq(&x).add(&x), *c);
        Some(x)
    }
    pub fn eval(&self, p: &[E], x: &E) -> E {
        p.iter().rev().fold(self.z(), |v, c| self.mul(&v, x).add(c))
    }
    pub fn embed_bits(&self, n: u64, root: &E) -> E {
        let mut v = self.z();
        for i in (0..64).rev() {
            v = self.mul(&v, root);
            if n >> i & 1 == 1 {
                v = v.add(&self.o());
            }
        }
        v
    }
    pub fn root(&self, bits: u64) -> E {
        let p = (0..64 - u64::leading_zeros(bits))
            .map(|i| self.e((bits >> i) & 1))
            .collect();
        self.roots(&p)
            .into_iter()
            .min_by_key(|v| v.to_biguint())
            .unwrap()
    }
    pub fn curve(&self, a: E, b: E) -> BinaryCurve {
        BinaryCurve {
            m: self.m(),
            irreducible: self.irr.clone(),
            a,
            b,
            generator: P::Infinity,
            order: BigUint::one(),
            cofactor: BigUint::one(),
        }
    }
    pub fn point(&self, c: &BinaryCurve, x: &E) -> Option<P> {
        if x.is_zero() {
            return Some(P::Affine {
                x: x.clone(),
                y: self.frob(&c.b, self.m() - 1),
            });
        }
        let rhs = x.add(&c.a).add(&self.mul(&c.b, &self.inv(&self.sq(x))));
        let y = self.mul(x, &self.quadratic(&rhs)?);
        let p = P::Affine { x: x.clone(), y };
        assert!(c.is_on_curve(&p));
        Some(p)
    }
    pub fn frobp(&self, p: &P, n: u32) -> P {
        match p {
            P::Infinity => P::Infinity,
            P::Affine { x, y } => P::Affine {
                x: self.frob(x, n),
                y: self.frob(y, n),
            },
        }
    }
    pub fn roots(&self, p: &Poly) -> Vec<E> {
        let p = monic(self, p);
        let mut parts = vec![p.clone()];
        let mut seed = 0x6b6f626c69747a06u64 ^ (self.m() as u64) ^ p.len() as u64;
        fn next(s: &mut u64) -> u64 {
            *s = s.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *s;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        }
        for attempt in 0..96 {
            if parts.iter().all(|g| g.len() == 2) {
                break;
            }
            let mut words = vec![];
            for _ in 0..self.m().div_ceil(64) {
                words.push(next(&mut seed));
            }
            let a = E::from_words(&words, self.m());
            let mut trace = vec![];
            let mut v = vec![self.z(), a];
            for _ in 0..self.m() {
                trace = add(&trace, &v);
                v = div(self, &square(self, &v), &p).1;
            }
            let mut out = vec![];
            for g in parts {
                if g.len() == 2 {
                    out.push(g);
                    continue;
                }
                let d = gcdp(self, &g, &trace);
                if d.len() > 1 && d.len() < g.len() {
                    out.push(div(self, &g, &d).0);
                    out.push(d);
                } else {
                    out.push(g);
                }
            }
            parts = out;
            if attempt == 95 {
                assert!(
                    parts.iter().all(|g| g.len() == 2),
                    "trace partition budget; not all roots in declared field"
                );
            }
        }
        let mut roots: Vec<_> = parts
            .iter()
            .map(|g| self.mul(&g[0], &self.inv(&g[1])))
            .collect();
        roots.sort_by_key(|v| v.to_biguint());
        assert_eq!(roots.len(), p.len() - 1);
        assert!(roots.windows(2).all(|w| w[0] != w[1]));
        for r in &roots {
            assert!(self.eval(&p, r).is_zero());
        }
        assert_eq!(from_roots(self, &roots), p);
        roots
    }
}
pub fn neg(p: &P) -> P {
    match p {
        P::Infinity => p.clone(),
        P::Affine { x, y } => P::Affine {
            x: x.clone(),
            y: y.add(x),
        },
    }
}
pub fn plus(c: &BinaryCurve, p: &P, q: &P) -> P {
    binary_ecc::curve::point_add(c, p, q)
}
pub fn times(c: &BinaryCurve, p: &P, n: u64) -> P {
    binary_ecc::curve::scalar_mul(c, p, &BigUint::from(n))
}
pub fn ej(x: &E) -> Value {
    json!(x.to_biguint().to_str_radix(16))
}
pub fn pj(p: &P) -> Value {
    match p {
        P::Infinity => json!({"infinity":true}),
        P::Affine { x, y } => json!([ej(x), ej(y)]),
    }
}
pub fn polyj(p: &[E]) -> Value {
    json!(p.iter().map(ej).collect::<Vec<_>>())
}
pub fn trim(mut p: Poly) -> Poly {
    while p.last().is_some_and(|v| v.is_zero()) {
        p.pop();
    }
    p
}
pub fn add(a: &[E], b: &[E]) -> Poly {
    let mut p = if a.len() >= b.len() {
        a.to_vec()
    } else {
        b.to_vec()
    };
    for i in 0..a.len().min(b.len()) {
        p[i] = a[i].add(&b[i]);
    }
    trim(p)
}
pub fn scale(f: &F, p: &[E], a: &E) -> Poly {
    trim(p.iter().map(|x| f.mul(x, a)).collect())
}
pub fn mul(f: &F, a: &[E], b: &[E]) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut p = vec![f.z(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if x.is_zero() {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            if !y.is_zero() {
                p[i + j] = p[i + j].add(&f.mul(x, y));
            }
        }
    }
    trim(p)
}
pub fn square(f: &F, a: &[E]) -> Poly {
    if a.is_empty() {
        return vec![];
    }
    let mut p = vec![f.z(); 2 * a.len() - 1];
    for (i, x) in a.iter().enumerate() {
        p[2 * i] = f.sq(x);
    }
    trim(p)
}
pub fn pow(f: &F, a: &[E], n: usize) -> Poly {
    (0..n).fold(vec![f.o()], |v, _| mul(f, &v, a))
}
pub fn monic(f: &F, a: &[E]) -> Poly {
    if a.is_empty() {
        return vec![];
    }
    scale(f, a, &f.inv(a.last().unwrap()))
}
pub fn div(f: &F, a: &[E], b: &[E]) -> (Poly, Poly) {
    assert!(!b.is_empty());
    let mut a = trim(a.to_vec());
    let mut q = vec![f.z(); a.len().saturating_sub(b.len()) + 1];
    let inv = f.inv(b.last().unwrap());
    while !a.is_empty() && a.len() >= b.len() {
        let i = a.len() - b.len();
        let c = f.mul(a.last().unwrap(), &inv);
        q[i] = c.clone();
        for j in 0..b.len() {
            if !b[j].is_zero() {
                a[i + j] = a[i + j].add(&f.mul(&c, &b[j]));
            }
        }
        a = trim(a);
    }
    (trim(q), a)
}
pub fn gcdp(f: &F, a: &[E], b: &[E]) -> Poly {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    while !b.is_empty() {
        let r = div(f, &a, &b).1;
        a = b;
        b = r;
    }
    monic(f, &a)
}
pub fn invmod(f: &F, a: &[E], p: &[E]) -> Poly {
    let (mut r, mut s) = (p.to_vec(), div(f, a, p).1);
    let (mut u, mut v) = (vec![], vec![f.o()]);
    while !s.is_empty() {
        let (q, t) = div(f, &r, &s);
        (r, s) = (s, t);
        let w = div(f, &add(&u, &mul(f, &q, &v)), p).1;
        (u, v) = (v, w);
    }
    assert_eq!(r.len(), 1, "nonunit in etale algebra");
    div(f, &scale(f, &u, &f.inv(&r[0])), p).1
}
pub fn from_roots(f: &F, r: &[E]) -> Poly {
    r.iter()
        .fold(vec![f.o()], |v, x| mul(f, &v, &[x.clone(), f.o()]))
}
pub fn division(f: &F, b: &E, n: usize) -> Poly {
    let mut ds = vec![vec![]; n + 1];
    ds[1] = vec![f.o()];
    ds[2] = vec![f.z(), f.o()];
    ds[3] = vec![b.clone(), f.z(), f.z(), f.o(), f.o()];
    ds[4] = vec![f.z(), f.z(), b.clone(), f.z(), f.z(), f.z(), f.o()];
    for i in 5..=n {
        let m = i / 2;
        ds[i] = if i % 2 == 1 {
            add(
                &mul(f, &ds[m + 2], &pow(f, &ds[m], 3)),
                &mul(f, &ds[m - 1], &pow(f, &ds[m + 1], 3)),
            )
        } else {
            let p = mul(
                f,
                &ds[m],
                &add(
                    &mul(f, &ds[m + 2], &square(f, &ds[m - 1])),
                    &mul(f, &ds[m - 2], &square(f, &ds[m + 1])),
                ),
            );
            assert!(p[0].is_zero());
            p[1..].to_vec()
        };
    }
    ds[n].clone()
}
pub fn kernels7(f: &F, b: &E) -> Vec<Vec<E>> {
    let roots = f.roots(&division(f, b, 7));
    let mut seen = vec![];
    let mut out = vec![];
    for r in roots {
        if seen.contains(&r) {
            continue;
        }
        let mut orbit = vec![];
        let mut x = r.clone();
        loop {
            assert!(!seen.contains(&x));
            seen.push(x.clone());
            orbit.push(x.clone());
            x = f.sq(&x).add(&f.mul(b, &f.inv(&f.sq(&x))));
            if x == r {
                break;
            }
        }
        assert_eq!(orbit.len(), 3);
        out.push(orbit);
    }
    assert_eq!(out.len(), 8);
    out
}
pub fn velu(f: &F, k: &[E], xs: &[E]) -> Vec<E> {
    let p = from_roots(f, k);
    let derivative: Poly = p
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, x)| if i % 2 == 1 { x.clone() } else { f.z() })
        .collect();
    xs.iter()
        .map(|x| {
            let z = f.mul(&f.eval(&derivative, x), &f.inv(&f.eval(&p, x)));
            x.add(&f.mul(x, &z)).add(&f.mul(&f.sq(x), &f.sq(&z)))
        })
        .collect()
}
pub fn qb(f: &F, b: &E, k: &[E]) -> E {
    let t = k.iter().fold(f.z(), |v, x| v.add(x));
    b.add(&t).add(&f.sq(&t))
}
pub fn reconstruct(f: &F, js: &[E; 3]) -> P {
    let param = |a: &E, b: &E| {
        let mut p = vec![f.z(); 9];
        p[0] = f.o();
        p[4] = f.o();
        p[7] = a.clone();
        p[8] = f.o();
        let mut q = vec![f.z(); 9];
        q[0] = f.o();
        q[1] = b.clone();
        q[4] = f.o();
        q[8] = f.o();
        let g = gcdp(f, &p, &q);
        assert_eq!(g.len(), 2, "modular parameter gcd");
        f.mul(&g[0], &f.inv(&g[1]))
    };
    let t = param(&js[0], &js[1]);
    let y = param(&js[1], &js[2]);
    let x = f.frob(&f.mul(&t, &y), f.m() - 2);
    let d = f.sq(&x).add(&x).add(&f.o());
    let yy = f.mul(&y.add(&x).add(&f.o()), &f.inv(&d));
    let p = P::Affine { x, y: yy };
    assert!(f.curve(f.o(), f.o()).is_on_curve(&p));
    p
}
