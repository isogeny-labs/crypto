//! Quaternion algebra B = (-1,-p), maximal order O0 = <1, i, (1+j)/2, (i+k)/2>,
//! left ideals in HNF (order coordinates), LLL and certified enumeration.
//! Conventions follow math-st lib/quaternion.py (commit f631bcc).

use crate::arith::*;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// (c0 + c1 i + c2 j + c3 k) / den, den > 0, in lowest terms.
#[derive(Clone, Debug, PartialEq)]
pub struct Q {
    pub c: [Z; 4],
    pub den: Z,
}

impl Q {
    pub fn new(c: [Z; 4], den: Z) -> Q {
        let (c, den) = if den.is_negative() {
            (c.map(|x| -x), -den)
        } else {
            (c, den)
        };
        let mut g = den.clone();
        for x in c.iter() {
            g = g.gcd(x);
        }
        if g.is_one() || g.is_zero() {
            return Q { c, den };
        }
        Q {
            c: c.map(|x| x / &g),
            den: den / &g,
        }
    }
    pub fn int(c: [i64; 4]) -> Q {
        Q::new(c.map(z), Z::one())
    }
    pub fn conj(&self) -> Q {
        Q {
            c: [
                self.c[0].clone(),
                -&self.c[1],
                -&self.c[2],
                -&self.c[3],
            ],
            den: self.den.clone(),
        }
    }
    pub fn scale(&self, num: &Z, den: &Z) -> Q {
        Q::new(self.c.clone().map(|x| x * num), &self.den * den)
    }
}

pub fn qmul(p: &Z, x: &Q, y: &Q) -> Q {
    let [a, b, c, d] = &x.c;
    let [e, f, g, h] = &y.c;
    let r0 = a * e - b * f - p * c * g - p * d * h;
    let r1 = a * f + b * e + p * (c * h - d * g);
    let r2 = a * g + c * e - b * h + d * f;
    let r3 = a * h + d * e + b * g - c * f;
    Q::new([r0, r1, r2, r3], &x.den * &y.den)
}

pub fn qnrd(p: &Z, x: &Q) -> Rat {
    let [a, b, c, d] = &x.c;
    Rat::new(a * a + b * b + p * (c * c + d * d), &x.den * &x.den)
}

/// Order coordinates z -> quaternion: z0 + z1 i + z2 (1+j)/2 + z3 (i+k)/2.
pub fn from_order(zc: &[Z; 4]) -> Q {
    let [z0, z1, z2, z3] = zc;
    Q::new(
        [
            Z::from(2) * z0 + z2,
            Z::from(2) * z1 + z3,
            z2.clone(),
            z3.clone(),
        ],
        z(2),
    )
}

/// Quaternion -> order coordinates (a-c, b-d, 2c, 2d); None if not in O0.
pub fn to_order(x: &Q) -> Option<[Z; 4]> {
    let [a, b, c, d] = &x.c;
    let nums = [a - c, b - d, Z::from(2) * c, Z::from(2) * d];
    let mut out: [Z; 4] = Default::default();
    for (k, v) in nums.iter().enumerate() {
        let (q, r) = v.div_rem(&x.den);
        if !r.is_zero() {
            return None;
        }
        out[k] = q;
    }
    Some(out)
}

pub fn order_norm(p: &Z, zc: &[Z; 4]) -> Z {
    let [z0, z1, z2, z3] = zc;
    let m = (p + 1u32) / 4u32;
    z0 * z0 + z0 * z2 + &m * z2 * z2 + z1 * z1 + z1 * z3 + &m * z3 * z3
}

/// Doubled norm Gram matrix on order coordinates (upstream doubled_norm_gram).
pub fn g2(p: &Z) -> [[Z; 4]; 4] {
    let h = (p + 1u32) / 2u32;
    [
        [z(2), z(0), z(1), z(0)],
        [z(0), z(2), z(0), z(1)],
        [z(1), z(0), h.clone(), z(0)],
        [z(0), z(1), z(0), h],
    ]
}

pub fn inner(g: &[[Z; 4]; 4], u: &[Z; 4], v: &[Z; 4]) -> Z {
    let mut s = Z::zero();
    for r in 0..4 {
        if u[r].is_zero() {
            continue;
        }
        let mut t = Z::zero();
        for c in 0..4 {
            t += &g[r][c] * &v[c];
        }
        s += &u[r] * t;
    }
    s
}

pub fn order_basis() -> [Q; 4] {
    [
        from_order(&[z(1), z(0), z(0), z(0)]),
        from_order(&[z(0), z(1), z(0), z(0)]),
        from_order(&[z(0), z(0), z(1), z(0)]),
        from_order(&[z(0), z(0), z(0), z(1)]),
    ]
}

/// Row Hermite normal form of a full-rank lattice in Z^4 (upper triangular,
/// positive pivots, entries above each pivot reduced into [0, pivot)).
pub fn hnf(rows: Vec<[Z; 4]>) -> Option<[[Z; 4]; 4]> {
    let mut a: Vec<[Z; 4]> = rows.into_iter().filter(|r| r.iter().any(|x| !x.is_zero())).collect();
    let mut r = 0usize;
    for col in 0..4 {
        loop {
            let mut piv: Option<usize> = None;
            for i in r..a.len() {
                if !a[i][col].is_zero() {
                    match piv {
                        None => piv = Some(i),
                        Some(j) => {
                            if a[i][col].abs() < a[j][col].abs() {
                                piv = Some(i)
                            }
                        }
                    }
                }
            }
            let piv = piv?;
            a.swap(r, piv);
            let mut done = true;
            for i in (r + 1)..a.len() {
                if !a[i][col].is_zero() {
                    let q = a[i][col].div_floor(&a[r][col]);
                    let pr = a[r].clone();
                    for c in 0..4 {
                        a[i][c] -= &q * &pr[c];
                    }
                    if !a[i][col].is_zero() {
                        done = false;
                    }
                }
            }
            if done {
                break;
            }
        }
        if a[r][col].is_negative() {
            for c in 0..4 {
                a[r][c] = -a[r][c].clone();
            }
        }
        let pr = a[r].clone();
        for i in 0..r {
            let q = a[i][col].div_floor(&pr[col]);
            if !q.is_zero() {
                for c in 0..4 {
                    a[i][c] -= &q * &pr[c];
                }
            }
        }
        r += 1;
    }
    Some([a[0].clone(), a[1].clone(), a[2].clone(), a[3].clone()])
}

#[derive(Clone, Debug)]
pub struct Ideal {
    pub b: [[Z; 4]; 4],
}

impl Ideal {
    pub fn from_gens(gens: Vec<[Z; 4]>) -> Option<Ideal> {
        hnf(gens).map(|b| Ideal { b })
    }
    pub fn index(&self) -> Z {
        &self.b[0][0] * &self.b[1][1] * &self.b[2][2] * &self.b[3][3]
    }
    pub fn norm(&self) -> Option<Z> {
        is_square(&self.index())
    }
    /// Integer coefficients of v in the HNF basis, or None if v is not in the lattice.
    pub fn coeffs(&self, v: &[Z; 4]) -> Option<[Z; 4]> {
        let mut w = v.clone();
        let mut x: [Z; 4] = Default::default();
        for r in 0..4 {
            let (q, rem) = w[r].div_rem(&self.b[r][r]);
            if !rem.is_zero() {
                return None;
            }
            for c in r..4 {
                w[c] -= &q * &self.b[r][c];
            }
            x[r] = q;
        }
        if w.iter().all(|t| t.is_zero()) {
            Some(x)
        } else {
            None
        }
    }
    pub fn contains(&self, v: &[Z; 4]) -> bool {
        self.coeffs(v).is_some()
    }
    pub fn contains_q(&self, x: &Q) -> bool {
        match to_order(x) {
            Some(v) => self.contains(&v),
            None => false,
        }
    }
    /// O0 * I subset I (left O0-module).
    pub fn left_closed(&self, p: &Z) -> bool {
        let ob = order_basis();
        for o in ob.iter() {
            for row in self.b.iter() {
                let prod = qmul(p, o, &from_order(row));
                if !self.contains_q(&prod) {
                    return false;
                }
            }
        }
        true
    }
    /// All HNF entries divisible by m, i.e. I subset m O0.
    pub fn divisible_by(&self, m: &Z) -> bool {
        self.b.iter().all(|r| r.iter().all(|x| (x % m).is_zero()))
    }
}

/// I = O0 alpha + O0 ell' (upstream MaximalOrder.prime_ideal).
pub fn prime_ideal(p: &Z, ellp: &Z, alpha: &[Z; 4]) -> Ideal {
    let mut gens = Vec::new();
    for r in 0..4 {
        let mut v: [Z; 4] = Default::default();
        v[r] = ellp.clone();
        gens.push(v);
    }
    let a = from_order(alpha);
    for o in order_basis().iter() {
        gens.push(to_order(&qmul(p, o, &a)).unwrap());
    }
    let id = Ideal::from_gens(gens).expect("rank");
    assert_eq!(id.index(), ellp * ellp, "residue did not define a prime-norm ideal");
    id
}

/// I * conj(x) / N(I); None if not integral. Returns the ideal.
pub fn equivalent(p: &Z, id: &Ideal, nid: &Z, x: &Q) -> Option<Ideal> {
    let xc = x.conj();
    let mut gens = Vec::new();
    for row in id.b.iter() {
        let prod = qmul(p, &from_order(row), &xc).scale(&Z::one(), nid);
        gens.push(to_order(&prod)?);
    }
    Ideal::from_gens(gens)
}

// ---------------- LLL and enumeration for q(x) = nrd(x) ----------------

fn gso(b: &[[Z; 4]], g: &[[Z; 4]; 4]) -> (Vec<Vec<Rat>>, Vec<Rat>) {
    let n = b.len();
    let mut pair = vec![vec![Z::zero(); n]; n];
    for r in 0..n {
        for c in 0..n {
            pair[r][c] = inner(g, &b[r], &b[c]);
        }
    }
    let mut mu = vec![vec![Rat::int(Z::zero()); n]; n];
    let mut bs: Vec<Rat> = Vec::new();
    for r in 0..n {
        let mut norm = Rat::int(pair[r][r].clone());
        for pr in 0..r {
            let mut num = Rat::int(pair[r][pr].clone());
            for e in 0..pr {
                num = num.sub(&mu[r][e].mul(&mu[pr][e]).mul(&bs[e]));
            }
            mu[r][pr] = num.div(&bs[pr]);
            norm = norm.sub(&mu[r][pr].mul(&mu[r][pr]).mul(&bs[pr]));
        }
        assert!(!norm.is_neg() && !norm.n.is_zero(), "not positive definite");
        bs.push(norm);
    }
    (mu, bs)
}

/// Exact LLL (delta = 3/4) for the reduced-norm form, as upstream norm_lll_reduce.
pub fn lll(p: &Z, basis: &[[Z; 4]; 4]) -> [[Z; 4]; 4] {
    let g = g2(p);
    let mut b: Vec<[Z; 4]> = basis.to_vec();
    let delta = Rat::new(z(3), z(4));
    let mut k = 1usize;
    while k < 4 {
        let (mut mu, _) = gso(&b, &g);
        for j in (0..k).rev() {
            let q = mu[k][j].round();
            if !q.is_zero() {
                let bj = b[j].clone();
                for c in 0..4 {
                    b[k][c] -= &q * &bj[c];
                }
                mu = gso(&b, &g).0;
            }
        }
        let (mu, bs) = gso(&b, &g);
        let lhs = &bs[k];
        let rhs = delta.sub(&mu[k][k - 1].mul(&mu[k][k - 1])).mul(&bs[k - 1]);
        if rhs.le(lhs) {
            k += 1;
        } else {
            b.swap(k, k - 1);
            k = std::cmp::max(k - 1, 1);
        }
    }
    [b[0].clone(), b[1].clone(), b[2].clone(), b[3].clone()]
}

pub fn coeff_gram(p: &Z, b: &[[Z; 4]; 4]) -> [[Z; 4]; 4] {
    let g = g2(p);
    let mut m: [[Z; 4]; 4] = Default::default();
    for r in 0..4 {
        for c in 0..4 {
            m[r][c] = inner(&g, &b[r], &b[c]);
        }
    }
    m
}

fn ldl(g: &[[Z; 4]; 4]) -> (Vec<Vec<Rat>>, Vec<Rat>) {
    let n = 4;
    let mut l = vec![vec![Rat::int(Z::zero()); n]; n];
    for r in 0..n {
        l[r][r] = Rat::int(Z::one());
    }
    let mut d: Vec<Rat> = Vec::new();
    for r in 0..n {
        let mut v = Rat::int(g[r][r].clone());
        for pr in 0..r {
            v = v.sub(&l[r][pr].mul(&l[r][pr]).mul(&d[pr]));
        }
        assert!(!v.is_neg() && !v.n.is_zero());
        d.push(v.clone());
        for later in (r + 1)..n {
            let mut num = Rat::int(g[later][r].clone());
            for pr in 0..r {
                num = num.sub(&l[later][pr].mul(&l[r][pr]).mul(&d[pr]));
            }
            l[later][r] = num.div(&v);
        }
    }
    (l, d)
}

fn enum_rec(
    idx: isize,
    rem: Rat,
    coeffs: &mut [Z; 4],
    l: &Vec<Vec<Rat>>,
    d: &Vec<Rat>,
    out: &mut Vec<[Z; 4]>,
) {
    if idx < 0 {
        out.push(coeffs.clone());
        return;
    }
    let i = idx as usize;
    let mut offset = Rat::int(Z::zero());
    for later in (i + 1)..4 {
        offset = offset.add(&l[later][i].mul(&Rat::int(coeffs[later].clone())));
    }
    let sq = rem.div(&d[i]);
    if sq.is_neg() {
        return;
    }
    let radius = sq.floor().sqrt() + 2u32;
    let center = Rat::int(Z::zero()).sub(&offset);
    let cf = center.floor();
    let mut v = &cf - &radius;
    let hi = &cf + &radius;
    while v <= hi {
        let shifted = Rat::int(v.clone()).add(&offset);
        let cost = d[i].mul(&shifted).mul(&shifted);
        if cost.le(&rem) {
            coeffs[i] = v.clone();
            enum_rec(idx - 1, rem.sub(&cost), coeffs, l, d, out);
        }
        v += 1u32;
    }
}

/// Every coefficient vector c with c^T G c <= bound (exact, certified).
pub fn enumerate(g: &[[Z; 4]; 4], bound: &Z) -> Vec<[Z; 4]> {
    let (l, d) = ldl(g);
    let mut out = Vec::new();
    let mut c: [Z; 4] = Default::default();
    enum_rec(3, Rat::int(bound.clone()), &mut c, &l, &d, &mut out);
    out
}

pub fn combo(c: &[Z; 4], b: &[[Z; 4]; 4]) -> [Z; 4] {
    let mut v: [Z; 4] = Default::default();
    for r in 0..4 {
        for col in 0..4 {
            v[col] += &c[r] * &b[r][col];
        }
    }
    v
}

/// Exact min nrd(x)/N(I) over nonzero x in I: (value, witness, candidates, reduced basis).
pub fn exact_svp(p: &Z, id: &Ideal, nid: &Z) -> (Z, [Z; 4], usize, [[Z; 4]; 4]) {
    let red = lll(p, &id.b);
    let gm = coeff_gram(p, &red);
    let mut bound = gm[0][0].clone();
    for r in 1..4 {
        if gm[r][r] < bound {
            bound = gm[r][r].clone();
        }
    }
    let cands = enumerate(&gm, &bound);
    let mut best: Option<(Z, [Z; 4])> = None;
    for c in cands.iter() {
        if c.iter().all(|x| x.is_zero()) {
            continue;
        }
        let v = combo(c, &red);
        let n2 = Z::from(2) * order_norm(p, &v);
        assert_eq!(n2, {
            let mut s = Z::zero();
            for r in 0..4 {
                for cc in 0..4 {
                    s += &c[r] * &gm[r][cc] * &c[cc];
                }
            }
            s
        });
        let better = match &best {
            None => true,
            Some((bn, _)) => &n2 < bn,
        };
        if better {
            best = Some((n2, v));
        }
    }
    let (n2, v) = best.expect("no nonzero vector within LLL bound");
    let nrd = n2 / 2u32;
    let (q, r) = nrd.div_rem(nid);
    assert!(r.is_zero(), "norm not divisible by N(I)");
    (q, v, cands.len(), red)
}
