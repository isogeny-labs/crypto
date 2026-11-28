//! Counted field and curve arithmetic over crypto's `Gf2` (unchanged).
//!
//! Every arm, IC and rho alike, does all of its arithmetic through [`CF`],
//! which delegates the value to `semaev_decomp::Gf2` and counts:
//! `m` multiplications, `s` squarings, `i` inversions (a batch of `k`
//! non-zero elements by Montgomery's trick is charged `3(k−1)` m + 1 i),
//! `l` F₂-linear table maps (trace, Artin–Schreier solve, square root,
//! subspace membership), `h` hash-table probes or inserts, `z` products
//! modulo the subgroup order ℓ, `za` additions modulo ℓ.
use crate::cryptanalysis::binary_radical_root::ArtinSchreier;
use crate::cryptanalysis::semaev_decomp::Gf2;
use std::cell::Cell;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Ops {
    pub m: u64,
    pub s: u64,
    pub i: u64,
    pub l: u64,
    pub h: u64,
    pub z: u64,
    pub za: u64,
}

impl Ops {
    pub fn sub(&self, o: &Ops) -> Ops {
        Ops {
            m: self.m - o.m,
            s: self.s - o.s,
            i: self.i - o.i,
            l: self.l - o.l,
            h: self.h - o.h,
            z: self.z - o.z,
            za: self.za - o.za,
        }
    }
    pub fn add(&self, o: &Ops) -> Ops {
        Ops {
            m: self.m + o.m,
            s: self.s + o.s,
            i: self.i + o.i,
            l: self.l + o.l,
            h: self.h + o.h,
            z: self.z + o.z,
            za: self.za + o.za,
        }
    }
    pub fn scale(&self, f: f64) -> [f64; 7] {
        [
            self.m as f64 * f,
            self.s as f64 * f,
            self.i as f64 * f,
            self.l as f64 * f,
            self.h as f64 * f,
            self.z as f64 * f,
            self.za as f64 * f,
        ]
    }
    pub fn json(&self) -> serde_json::Value {
        serde_json::json!({"m":self.m,"s":self.s,"i":self.i,"l":self.l,"h":self.h,"z":self.z,"za":self.za})
    }
    pub fn from_json(v: &serde_json::Value) -> Ops {
        let g = |k: &str| v[k].as_u64().unwrap_or(0);
        Ops { m: g("m"), s: g("s"), i: g("i"), l: g("l"), h: g("h"), z: g("z"), za: g("za") }
    }
}

/// Itoh–Tsujii census of one inversion in `F_{2^n}` (the chain `Gf2::inv`
/// runs): `n − 1` squarings and `⌊log₂(n−1)⌋ + popcount(n−1) − 1` products.
pub fn inv_census(n: u32) -> (f64, f64) {
    let e = n - 1;
    let lg = 31 - e.leading_zeros();
    ((lg + e.count_ones() - 1) as f64, e as f64)
}

/// Pricing of an op vector in multiplication equivalents (Meq).
/// `ws` is the weight of a squaring; `wu` of every non-field native unit
/// (table map, hash probe, mod-ℓ product and addition), the §5a band
/// {0, 0.1, 1}.  An inversion is priced at its Itoh–Tsujii census.
pub fn meq(o: &Ops, n: u32, ws: f64, wu: f64) -> f64 {
    let (im, is) = inv_census(n);
    o.m as f64
        + ws * o.s as f64
        + o.i as f64 * (im + ws * is)
        + wu * (o.l + o.h + o.z + o.za) as f64
}

pub struct CF<'a> {
    pub gf: &'a Gf2,
    pub n: u32,
    pub c: Cell<Ops>,
}

impl<'a> CF<'a> {
    pub fn new(gf: &'a Gf2) -> Self {
        CF { gf, n: gf.n, c: Cell::new(Ops::default()) }
    }
    #[inline(always)]
    fn bump(&self, f: impl FnOnce(&mut Ops)) {
        let mut o = self.c.get();
        f(&mut o);
        self.c.set(o);
    }
    pub fn ops(&self) -> Ops {
        self.c.get()
    }
    #[inline(always)]
    pub fn mul(&self, a: u64, b: u64) -> u64 {
        self.bump(|o| o.m += 1);
        self.gf.mul(a, b)
    }
    #[inline(always)]
    pub fn sqr(&self, a: u64) -> u64 {
        self.bump(|o| o.s += 1);
        self.gf.sqr(a)
    }
    #[inline(always)]
    pub fn sqr_k(&self, a: u64, k: u32) -> u64 {
        self.bump(|o| o.s += k as u64);
        self.gf.sqr_k(a, k)
    }
    #[inline(always)]
    pub fn inv(&self, a: u64) -> u64 {
        self.bump(|o| o.i += 1);
        self.gf.inv(a)
    }
    /// Montgomery batch inversion in place; zeros stay zero and are not charged.
    pub fn batch_inv(&self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        let k = xs.iter().filter(|&&x| x != 0).count() as u64;
        if k == 0 {
            return;
        }
        self.bump(|o| {
            o.m += 3 * (k - 1);
            o.i += 1;
        });
        self.gf.batch_inv(xs, scratch);
    }
    #[inline(always)]
    pub fn lin(&self) {
        self.bump(|o| o.l += 1);
    }
    #[inline(always)]
    pub fn hash(&self) {
        self.bump(|o| o.h += 1);
    }
    #[inline(always)]
    pub fn zmul(&self, k: u64) {
        self.bump(|o| o.z += k);
    }
    #[inline(always)]
    pub fn zadd(&self, k: u64) {
        self.bump(|o| o.za += k);
    }
}

/// `y² + xy = x³ + a x² + b` over `F_{2^n}`.
pub struct Curve {
    pub gf: Gf2,
    pub n: u32,
    pub a: u64,
    pub b: u64,
    pub sqrt_b: u64,
    pub asv: ArtinSchreier,
    /// Order of the full group, cofactor and prime subgroup order.
    pub order: u64,
    pub h: u64,
    pub ell: u64,
    pub irr_low: Vec<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    Inf,
    A(u64, u64),
}

impl Pt {
    pub fn x(&self) -> u64 {
        match self {
            Pt::Inf => u64::MAX,
            Pt::A(x, _) => *x,
        }
    }
}

impl Curve {
    pub fn on_curve(&self, p: Pt) -> bool {
        match p {
            Pt::Inf => true,
            Pt::A(x, y) => {
                let g = &self.gf;
                let lhs = g.sqr(y) ^ g.mul(x, y);
                let x2 = g.sqr(x);
                let rhs = g.mul(x2, x) ^ g.mul(self.a, x2) ^ self.b;
                lhs == rhs
            }
        }
    }
    #[inline]
    pub fn neg(&self, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::A(x, y) => Pt::A(x, x ^ y),
        }
    }
    /// Canonical representative of `{P, −P}`: the smaller `y` as an integer.
    /// Returns the representative and whether it is `−P`.
    #[inline]
    pub fn canon(&self, p: Pt) -> (Pt, bool) {
        match p {
            Pt::Inf => (Pt::Inf, false),
            Pt::A(x, y) => {
                let y2 = x ^ y;
                if y2 < y {
                    (Pt::A(x, y2), true)
                } else {
                    (p, false)
                }
            }
        }
    }
    /// Finish an addition once `1/(x1 + x2)` is known (`x1 ≠ x2`).
    #[inline(always)]
    pub fn add_with_inv(&self, cf: &CF, p: (u64, u64), q: (u64, u64), inv: u64) -> Pt {
        let lam = cf.mul(p.1 ^ q.1, inv);
        let x3 = cf.sqr(lam) ^ lam ^ p.0 ^ q.0 ^ self.a;
        let y3 = cf.mul(lam, p.0 ^ x3) ^ x3 ^ p.1;
        Pt::A(x3, y3)
    }
    /// Finish a doubling once `1/x` is known (`x ≠ 0`).
    #[inline(always)]
    pub fn dbl_with_inv(&self, cf: &CF, p: (u64, u64), inv: u64) -> Pt {
        let lam = p.0 ^ cf.mul(p.1, inv);
        let x3 = cf.sqr(lam) ^ lam ^ self.a;
        let y3 = cf.sqr(p.0) ^ cf.mul(lam ^ 1, x3);
        Pt::A(x3, y3)
    }
    pub fn dbl(&self, cf: &CF, p: Pt) -> Pt {
        match p {
            Pt::Inf => Pt::Inf,
            Pt::A(x, y) => {
                if x == 0 {
                    Pt::Inf
                } else {
                    let iv = cf.inv(x);
                    self.dbl_with_inv(cf, (x, y), iv)
                }
            }
        }
    }
    pub fn add(&self, cf: &CF, p: Pt, q: Pt) -> Pt {
        match (p, q) {
            (Pt::Inf, _) => q,
            (_, Pt::Inf) => p,
            (Pt::A(x1, y1), Pt::A(x2, y2)) => {
                if x1 == x2 {
                    if y1 == y2 {
                        self.dbl(cf, p)
                    } else {
                        Pt::Inf
                    }
                } else {
                    let iv = cf.inv(x1 ^ x2);
                    self.add_with_inv(cf, (x1, y1), (x2, y2), iv)
                }
            }
        }
    }
    pub fn mul(&self, cf: &CF, k: u64, p: Pt) -> Pt {
        let mut r = Pt::Inf;
        if k == 0 {
            return r;
        }
        for bit in (0..64 - k.leading_zeros()).rev() {
            r = self.dbl(cf, r);
            if (k >> bit) & 1 == 1 {
                r = self.add(cf, r, p);
            }
        }
        r
    }
    /// `[a]P + [b]Q` by Shamir's trick (P + Q computed once).
    pub fn joint(&self, cf: &CF, a: u64, p: Pt, b: u64, q: Pt) -> Pt {
        let pq = self.add(cf, p, q);
        let mut r = Pt::Inf;
        let top = 64 - (a | b).leading_zeros();
        for bit in (0..top).rev() {
            r = self.dbl(cf, r);
            let (ab, bb) = ((a >> bit) & 1, (b >> bit) & 1);
            r = match (ab, bb) {
                (1, 1) => self.add(cf, r, pq),
                (1, 0) => self.add(cf, r, p),
                (0, 1) => self.add(cf, r, q),
                _ => r,
            };
        }
        r
    }
    /// Points with abscissa `x` (`x ≠ 0`): `y = x·z`, `z² + z = x + a + b/x²`.
    /// `inv_x2 = 1/x²` supplied by the caller (batched).
    pub fn lift_with(&self, cf: &CF, x: u64, inv_x2: u64) -> Option<Pt> {
        let c = x ^ self.a ^ cf.mul(self.b, inv_x2);
        cf.lin();
        if self.asv.trace(c) != 0 {
            return None;
        }
        cf.lin();
        let z = self.asv.solve(c);
        let y = cf.mul(x, z);
        Some(Pt::A(x, y))
    }
    /// The 2-torsion point `(0, √b)`.
    pub fn t2(&self) -> Pt {
        Pt::A(0, self.sqrt_b)
    }
}

/// Batched affine addition of many independent pairs: `out[k] = p[k] + q[k]`.
/// Pairs with equal abscissae are done one at a time (doubling or O).
pub fn batch_add(c: &Curve, cf: &CF, ps: &[Pt], qs: &[Pt], out: &mut Vec<Pt>, scr: &mut (Vec<u64>, Vec<u64>)) {
    let (den, tmp) = scr;
    den.clear();
    for (p, q) in ps.iter().zip(qs) {
        match (p, q) {
            (Pt::A(x1, _), Pt::A(x2, _)) if x1 != x2 => den.push(x1 ^ x2),
            _ => den.push(0),
        }
    }
    cf.batch_inv(den, tmp);
    out.clear();
    for (k, (p, q)) in ps.iter().zip(qs).enumerate() {
        match (p, q) {
            (Pt::A(x1, y1), Pt::A(x2, y2)) if x1 != x2 => {
                out.push(c.add_with_inv(cf, (*x1, *y1), (*x2, *y2), den[k]))
            }
            _ => out.push(c.add(cf, *p, *q)),
        }
    }
}
