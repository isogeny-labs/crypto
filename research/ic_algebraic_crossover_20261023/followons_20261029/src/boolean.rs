//! Exact arithmetic in F2[x]/(x_i^2-x_i), with replayable source multipliers.
use crate::field::Field;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poly(pub BTreeSet<u16>);
impl Poly {
    pub fn one() -> Self {
        Self(BTreeSet::from([0]))
    }
    pub fn var(i: usize) -> Self {
        assert!(i < 16);
        Self(BTreeSet::from([1 << i]))
    }
    pub fn add(&self, b: &Self) -> Self {
        Self(self.0.symmetric_difference(&b.0).copied().collect())
    }
    pub fn mul(&self, b: &Self) -> Self {
        let mut r = Self::default();
        for &x in &self.0 {
            for &y in &b.0 {
                let t = x | y;
                if !r.0.remove(&t) {
                    r.0.insert(t);
                }
            }
        }
        r
    }
    pub fn degree(&self) -> u32 {
        self.0.iter().map(|m| m.count_ones()).max().unwrap_or(0)
    }
    pub fn eval(&self, a: u16) -> bool {
        self.0.iter().filter(|&&m| a & m == m).count() % 2 == 1
    }
    pub fn substitute(&self, images: &[Self]) -> Self {
        self.0.iter().fold(Self::default(), |s, &m| {
            let mut t = Self::one();
            for (i, p) in images.iter().enumerate() {
                if m >> i & 1 != 0 {
                    t = t.mul(p);
                }
            }
            assert_eq!(m >> images.len(), 0);
            s.add(&t)
        })
    }
    pub fn specialize(&self, n: usize, k: usize, a: u16) -> Self {
        assert!(k <= n && n < 16 && a < (1 << k));
        let images: Vec<_> = (0..n)
            .map(|i| {
                if i < k {
                    if a >> i & 1 == 0 {
                        Self::default()
                    } else {
                        Self::one()
                    }
                } else {
                    Self::var(i - k)
                }
            })
            .collect();
        self.substitute(&images)
    }
    pub fn lead(&self) -> Option<u16> {
        self.0.iter().max_by_key(|&&m| (m.count_ones(), m)).copied()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CertifiedRow {
    pub row: Poly,
    pub multipliers: Vec<Poly>,
}
impl CertifiedRow {
    pub fn xor(&self, b: &Self) -> Self {
        Self {
            row: self.row.add(&b.row),
            multipliers: self
                .multipliers
                .iter()
                .zip(&b.multipliers)
                .map(|(a, b)| a.add(b))
                .collect(),
        }
    }
    pub fn replay(&self, generators: &[Poly]) -> bool {
        self.multipliers.len() == generators.len()
            && self.row
                == self
                    .multipliers
                    .iter()
                    .zip(generators)
                    .fold(Poly::default(), |s, (a, b)| s.add(&a.mul(b)))
    }
}
#[derive(Clone, Debug, Default)]
pub struct Space {
    pub rows: BTreeMap<(u32, u16), CertifiedRow>,
}
impl Space {
    pub fn insert(&mut self, mut r: CertifiedRow) -> bool {
        while let Some(p) = r.row.lead() {
            let key = (p.count_ones(), p);
            if let Some(b) = self.rows.get(&key) {
                r = r.xor(b)
            } else {
                self.rows.insert(key, r);
                return true;
            }
        }
        false
    }
    pub fn contains(&self, p: &Poly) -> bool {
        let mut p = p.clone();
        while let Some(m) = p.lead() {
            if let Some(b) = self.rows.get(&(m.count_ones(), m)) {
                p = p.add(&b.row)
            } else {
                return false;
            }
        }
        true
    }
    pub fn certificate(&self, p: &Poly) -> Option<CertifiedRow> {
        let len = self.rows.values().next()?.multipliers.len();
        let mut r = CertifiedRow {
            row: p.clone(),
            multipliers: vec![Poly::default(); len],
        };
        while let Some(m) = r.row.lead() {
            let b = self.rows.get(&(m.count_ones(), m))?;
            r = r.xor(b);
        }
        Some(CertifiedRow {
            row: p.clone(),
            multipliers: r.multipliers,
        })
    }
}
pub fn closure(gens: &[Poly], n: usize, d: u32) -> Space {
    let mut s = Space::default();
    for (i, g) in gens.iter().enumerate() {
        if g.degree() <= d {
            let mut ms = vec![Poly::default(); gens.len()];
            ms[i] = Poly::one();
            s.insert(CertifiedRow {
                row: g.clone(),
                multipliers: ms,
            });
        }
    }
    loop {
        let snapshot: Vec<_> = s.rows.values().cloned().collect();
        let mut changed = false;
        for r in snapshot {
            for i in 0..n {
                let x = Poly::var(i);
                let p = r.row.mul(&x);
                if p.degree() <= d {
                    changed |= s.insert(CertifiedRow {
                        row: p,
                        multipliers: r.multipliers.iter().map(|m| m.mul(&x)).collect(),
                    });
                }
            }
        }
        if !changed {
            break;
        }
    }
    assert!(s.rows.values().all(|r| r.replay(gens)));
    s
}
/// Exact intersection, eliminating concatenated coordinates (v, label).
/// Relations with zero first coordinate yield the intersection, not rank matches.
pub fn intersection(a: &Space, b: &Space) -> Space {
    let ar: Vec<_> = a.rows.values().map(|x| x.row.clone()).collect();
    let br: Vec<_> = b.rows.values().map(|x| x.row.clone()).collect();
    let mut piv: BTreeMap<(u32, u16), (Poly, Poly)> = BTreeMap::new();
    let mut out = Space::default();
    for (is_a, row) in ar
        .iter()
        .map(|x| (true, x))
        .chain(br.iter().map(|x| (false, x)))
    {
        let mut p = row.clone();
        let mut label = if is_a { row.clone() } else { Poly::default() };
        while let Some(m) = p.lead() {
            let key = (m.count_ones(), m);
            if let Some((q, l)) = piv.get(&key) {
                p = p.add(q);
                label = label.add(l)
            } else {
                piv.insert(key, (p.clone(), label.clone()));
                break;
            }
        }
        if p.0.is_empty() && !label.0.is_empty() {
            assert!(a.contains(&label) && b.contains(&label));
            out.insert(CertifiedRow {
                row: label,
                multipliers: vec![],
            });
        }
    }
    assert_eq!(out.rows.len(), a.rows.len() + b.rows.len() - piv.len());
    out
}

#[derive(Clone)]
pub struct Element(pub Vec<Poly>);
impl Element {
    pub fn constant(f: Field, x: u32) -> Self {
        Self(
            (0..f.n)
                .map(|i| {
                    if x >> i & 1 != 0 {
                        Poly::one()
                    } else {
                        Poly::default()
                    }
                })
                .collect(),
        )
    }
    pub fn vars(f: Field, offset: usize, len: usize) -> Self {
        Self(
            (0..f.n as usize)
                .map(|i| {
                    if i < len {
                        Poly::var(i + offset)
                    } else {
                        Poly::default()
                    }
                })
                .collect(),
        )
    }
    pub fn add(&self, b: &Self) -> Self {
        Self(self.0.iter().zip(&b.0).map(|(a, b)| a.add(b)).collect())
    }
    pub fn mul(&self, b: &Self, f: Field) -> Self {
        let mut r = vec![Poly::default(); f.n as usize];
        for (i, a) in self.0.iter().enumerate() {
            for (j, b) in b.0.iter().enumerate() {
                let coeff = f.mul(1 << i, 1 << j);
                let p = a.mul(b);
                for (k, q) in r.iter_mut().enumerate() {
                    if coeff >> k & 1 != 0 {
                        *q = q.add(&p);
                    }
                }
            }
        }
        Self(r)
    }
    pub fn square(&self, f: Field) -> Self {
        let mut r = vec![Poly::default(); f.n as usize];
        for (i, a) in self.0.iter().enumerate() {
            let c = f.mul(1 << i, 1 << i);
            for (k, q) in r.iter_mut().enumerate() {
                if c >> k & 1 != 0 {
                    *q = q.add(a);
                }
            }
        }
        Self(r)
    }
    pub fn eval(&self, a: u16) -> u32 {
        self.0
            .iter()
            .enumerate()
            .fold(0, |s, (i, p)| s | ((p.eval(a) as u32) << i))
    }
}
pub fn f3(x: &Element, y: &Element, z: &Element, f: Field) -> Element {
    x.mul(y, f)
        .add(&x.mul(z, f))
        .add(&y.mul(z, f))
        .square(f)
        .add(&x.mul(y, f).mul(z, f))
        .add(&Element::constant(f, 1))
}
pub fn symmetric_s4(es: &[Element; 3], r: u32, f: Field) -> Element {
    let (e1, e2, e3) = (&es[0], &es[1], &es[2]);
    let a = e1.square(f);
    let b = e2.square(f);
    let c = e3.square(f);
    let t = Element::constant(f, r);
    let t2 = t.square(f);
    let t3 = t2.mul(&t, f);
    let t4 = t2.square(f);
    t4.add(&a.square(f))
        .add(&c.square(f))
        .add(&b.square(f).mul(&t4, f))
        .add(&c.mul(e3, f).mul(&t, f))
        .add(&e3.mul(&b, f).mul(&t3, f))
        .add(&e3.mul(&a, f).mul(&t, f))
        .add(&e3.mul(&t3, f))
        .add(&a.mul(&c, f).mul(&t2, f))
        .add(&c.mul(&t4, f))
        .add(&c)
        .add(&b.mul(&t2, f))
}
pub fn field_variables(f: Field, basis: &[u32], offset: usize) -> Element {
    let mut out = Element::constant(f, 0);
    for (j, &b) in basis.iter().enumerate() {
        for i in 0..f.n as usize {
            if b >> i & 1 != 0 {
                out.0[i] = out.0[i].add(&Poly::var(offset + j));
            }
        }
    }
    out
}
pub fn product_bases(f: Field, g: u32) -> [Vec<u32>; 3] {
    [
        vec![1, g],
        vec![1, g, f.sq(g)],
        vec![1, g, f.sq(g), f.mul(f.sq(g), g)],
    ]
}
pub fn correspondence(f: Field, g: u32) -> ([Element; 3], Vec<Poly>) {
    let bases = product_bases(f, g);
    let x = field_variables(f, &bases[0], 0);
    let y = field_variables(f, &bases[0], 2);
    let z = field_variables(f, &bases[0], 4);
    let es = [
        x.add(&y).add(&z),
        x.mul(&y, f).add(&x.mul(&z, f)).add(&y.mul(&z, f)),
        x.mul(&y, f).mul(&z, f),
    ];
    let mut pull = vec![];
    for (e, basis) in es.iter().zip(&bases) {
        for j in 0..basis.len() {
            let dual = (0..1u32 << f.n)
                .find(|&m| {
                    basis
                        .iter()
                        .enumerate()
                        .all(|(k, &b)| ((m & b).count_ones() % 2 == 1) == (j == k))
                })
                .expect("dependent product basis");
            let mut p = Poly::default();
            for (i, c) in e.0.iter().enumerate() {
                if dual >> i & 1 != 0 {
                    p = p.add(c);
                }
            }
            pull.push(p);
        }
    }
    (es, pull)
}

/// Kernel of projection to terms of degree>d, carrying the complete witness.
/// Filtering displayed basis rows individually would miss cancellations.
pub fn low_degree_image(rows: &[CertifiedRow], d: u32) -> Space {
    let mut piv: BTreeMap<(u32, u16), (Poly, CertifiedRow)> = BTreeMap::new();
    let mut out = Space::default();
    for row in rows {
        let mut full = row.clone();
        let mut high = Poly(
            row.row
                .0
                .iter()
                .copied()
                .filter(|m| m.count_ones() > d)
                .collect(),
        );
        while let Some(m) = high.lead() {
            let key = (m.count_ones(), m);
            if let Some((h, w)) = piv.get(&key) {
                high = high.add(h);
                full = full.xor(w)
            } else {
                piv.insert(key, (high.clone(), full.clone()));
                break;
            }
        }
        if high.0.is_empty() {
            assert!(full.row.degree() <= d);
            out.insert(full);
        }
    }
    out
}

/// Reuse literally identical input rows before independently finishing leaves.
/// Every shared elimination row keeps a certificate for every leaf.
pub fn shared_input_elimination(
    gens: &[Vec<Poly>],
    n: usize,
    d: u32,
) -> (usize, usize, Vec<Space>) {
    assert!(!gens.is_empty() && n <= 12);
    let inputs: Vec<BTreeMap<Vec<u16>, CertifiedRow>> = gens
        .iter()
        .map(|gs| {
            let mut rows = BTreeMap::new();
            for (i, g) in gs.iter().enumerate() {
                for m in 0..1u16 << n {
                    if m.count_ones() > d {
                        continue;
                    }
                    let multiplier = Poly(BTreeSet::from([m]));
                    let p = g.mul(&multiplier);
                    if p.degree() > d || p.0.is_empty() {
                        continue;
                    }
                    let mut ms = vec![Poly::default(); gs.len()];
                    ms[i] = multiplier;
                    rows.entry(p.0.iter().copied().collect())
                        .or_insert(CertifiedRow {
                            row: p,
                            multipliers: ms,
                        });
                }
            }
            rows
        })
        .collect();
    let common: Vec<_> = inputs[0]
        .keys()
        .filter(|key| inputs.iter().all(|rs| rs.contains_key(*key)))
        .cloned()
        .collect();
    let mut shared: BTreeMap<(u32, u16), Vec<CertifiedRow>> = BTreeMap::new();
    for key in &common {
        let mut certs: Vec<_> = inputs.iter().map(|rs| rs[key].clone()).collect();
        while let Some(m) = certs[0].row.lead() {
            let k = (m.count_ones(), m);
            if let Some(basis) = shared.get(&k) {
                for (c, b) in certs.iter_mut().zip(basis) {
                    *c = c.xor(b);
                }
            } else {
                assert!(certs.iter().all(|c| c.row == certs[0].row));
                shared.insert(k, certs.clone());
                break;
            }
        }
    }
    let mut spaces = vec![];
    for (leaf, gs) in gens.iter().enumerate() {
        let mut s = Space::default();
        for certs in shared.values() {
            assert!(certs[leaf].replay(gs));
            s.insert(certs[leaf].clone());
        }
        for c in inputs[leaf].values() {
            s.insert(c.clone());
        }
        assert!(s.rows.values().all(|c| c.replay(gs)));
        spaces.push(s);
    }
    (common.len(), shared.len(), spaces)
}
