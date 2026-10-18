//! L8 experiments (study `research/ecdlp_leads_20261018`).
//!
//! Exhaustive search over hyperelliptic curves `B: y^2 + h(x) y = f(x)` over F_2 of
//! genus 2 and 3, and (module `quartic`, argument `q`) over every smooth plane quartic,
//! so every genus-3 curve over F_2. For every curve and every odd prime `n` dividing
//! `#J_B(F_2)`, the geometric unramified Z/n covers `D_chi -> B` (class field theory:
//! characters `chi: J_B(F_2) -> Z/n`) are tested for the property
//!
//!     L(B, chi, X)  is divisible by  (1 - c*pi*X)(1 - c*pibar*X)
//!
//! for some n-th root of unity `c` and `pi` a Frobenius of a Koblitz curve
//! (trace t = +1 for a = 1, t = -1 for a = 0). A hit means the Prym of `D_chi -> B`,
//! or of its constant-field twist, contains the trace-zero variety `T_n`, so a curve
//! of genus `1 + n(g - 1)` over F_2 would carry `T_n` in its Jacobian.
//!
//! The Jacobian group `J_B(F_2)` is computed independently of any Jacobian
//! arithmetic: relations are divisors of functions `a(x) + b(x) y` whose norm is
//! smooth over places of degree <= 4, and the group is read off a Smith-type
//! diagonalisation. Every curve is checked: free rank 1 and torsion order equal to
//! `L_B(1)` from point counts. Every twisted L-function is checked against the
//! Riemann hypothesis (|top coefficient| = 2^(g-1)) and the full functional equation,
//! and for plane quartics against an out-of-sample `N_6`. The factor test runs in
//! floating point and exactly in `Z[zeta_n]`; the two must agree.
//!
//! Output: one JSON object per line on stdout (curve records with hits, and a final
//! summary); diagnostics on stderr.

use std::collections::{BTreeSet, HashMap};

mod quartic;

// ---------- F_2[x] as u64 bit polynomials ----------
fn deg(p: u64) -> i32 {
    if p == 0 {
        -1
    } else {
        63 - p.leading_zeros() as i32
    }
}
fn pmul(mut a: u64, mut b: u64) -> u64 {
    let mut r = 0u64;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    r
}
fn pdivmod(a: u64, b: u64) -> (u64, u64) {
    assert!(b != 0);
    let db = deg(b);
    let (mut q, mut r) = (0u64, a);
    while deg(r) >= db {
        let s = deg(r) - db;
        q ^= 1 << s;
        r ^= b << s;
    }
    (q, r)
}
fn pgcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = pdivmod(a, b).1;
        a = b;
        b = r;
    }
    a
}
fn pderiv(p: u64) -> u64 {
    let mut r = 0;
    let mut i = 1;
    while i < 64 {
        if (p >> i) & 1 == 1 {
            r |= 1 << (i - 1);
        }
        i += 2;
    }
    r
}
fn coef(p: u64, i: i32) -> u64 {
    if !(0..64).contains(&i) {
        0
    } else {
        (p >> i) & 1
    }
}

/// Irreducible polynomials over F_2 of degree 1..=4.
const IRRED: [u64; 8] = [0b10, 0b11, 0b111, 0b1011, 0b1101, 0b10011, 0b11001, 0b11111];

// ---------- GF(2^k), k = 1..=4 ----------
#[derive(Clone, Copy)]
struct Gf {
    k: u32,
    modulus: u32,
}
const FIELDS: [Gf; 5] = [
    Gf { k: 0, modulus: 0 },
    Gf {
        k: 1,
        modulus: 0b11,
    },
    Gf {
        k: 2,
        modulus: 0b111,
    },
    Gf {
        k: 3,
        modulus: 0b1011,
    },
    Gf {
        k: 4,
        modulus: 0b10011,
    },
];
impl Gf {
    fn q(&self) -> u32 {
        1 << self.k
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        let mut r = 0u32;
        let (mut a, mut b) = (a, b);
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            a <<= 1;
            b >>= 1;
        }
        // reduce
        let dm = 31 - self.modulus.leading_zeros();
        while r != 0 && (31 - r.leading_zeros()) >= dm {
            let s = (31 - r.leading_zeros()) - dm;
            r ^= self.modulus << s;
        }
        r
    }
    fn pow(&self, a: u32, mut e: u32) -> u32 {
        let (mut base, mut r) = (a, 1u32);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, base);
            }
            base = self.mul(base, base);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u32) -> u32 {
        assert!(a != 0);
        self.pow(a, self.q() - 2)
    }
    fn frob(&self, a: u32) -> u32 {
        self.mul(a, a)
    }
    fn eval(&self, p: u64, x: u32) -> u32 {
        let mut r = 0u32;
        let mut i = deg(p);
        while i >= 0 {
            r = self.mul(r, x) ^ (coef(p, i) as u32);
            i -= 1;
        }
        r
    }
    fn roots(&self, p: u64) -> Vec<u32> {
        (0..self.q()).filter(|&x| self.eval(p, x) == 0).collect()
    }
}

// ---------- curve ----------
#[derive(Clone, Copy)]
struct Curve {
    g: i32,
    h: u64,
    f: u64,
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum InfType {
    Split,
    Inert,
    Ramified,
}
impl Curve {
    fn hg1(&self) -> u64 {
        coef(self.h, self.g + 1)
    }
    fn f2g2(&self) -> u64 {
        coef(self.f, 2 * self.g + 2)
    }
    fn is_smooth_genus(&self) -> bool {
        let g = self.g;
        if self.h == 0 {
            return false;
        }
        let m = (2 * deg(self.h)).max(deg(self.f));
        if m != 2 * g + 1 && m != 2 * g + 2 {
            return false;
        }
        let hp = pderiv(self.h);
        let s = pmul(pmul(hp, hp), self.f) ^ pmul(pderiv(self.f), pderiv(self.f));
        if deg(pgcd(self.h, s)) > 0 {
            return false;
        }
        // infinity
        if self.hg1() == 0 {
            let hg = coef(self.h, g);
            let f2g1 = coef(self.f, 2 * g + 1);
            if (hg & self.f2g2()) == f2g1 {
                return false;
            }
        }
        true
    }
    fn inf_type(&self) -> InfType {
        if self.hg1() == 0 {
            InfType::Ramified
        } else if self.f2g2() == 0 {
            InfType::Split
        } else {
            InfType::Inert
        }
    }
    /// Points over GF(2^k): affine (x,y) and infinity roots s.
    fn points(&self, fld: &Gf) -> (Vec<(u32, u32)>, Vec<u32>) {
        let mut aff = vec![];
        for x in 0..fld.q() {
            let hx = fld.eval(self.h, x);
            let fx = fld.eval(self.f, x);
            for y in 0..fld.q() {
                if fld.mul(y, y) ^ fld.mul(hx, y) ^ fx == 0 {
                    aff.push((x, y));
                }
            }
        }
        let a = self.hg1() as u32;
        let c = self.f2g2() as u32;
        let inf = (0..fld.q())
            .filter(|&s| fld.mul(s, s) ^ fld.mul(a, s) ^ c == 0)
            .collect();
        (aff, inf)
    }
    fn count(&self, k: u32) -> i64 {
        let (a, i) = self.points(&FIELDS[k as usize]);
        (a.len() + i.len()) as i64
    }
}

/// L-polynomial coefficients a_0..a_{2g} from point counts N_1..N_g (Newton).
fn lpoly(c: &Curve) -> Vec<i64> {
    let counts: Vec<i64> = (1..=c.g as u32).map(|k| c.count(k)).collect();
    lpoly_from_counts(c.g, &counts)
}
fn lpoly_from_counts(g: i32, counts: &[i64]) -> Vec<i64> {
    let g = g as usize;
    assert_eq!(counts.len(), g);
    let mut s = vec![0i64; g + 1];
    for k in 1..=g {
        s[k] = (1i64 << k) + 1 - counts[k - 1];
    }
    let mut a = vec![0i64; 2 * g + 1];
    a[0] = 1;
    for k in 1..=g {
        let mut acc = 0i64;
        for i in 1..=k {
            acc -= s[i] * a[k - i];
        }
        assert!(acc % (k as i64) == 0);
        a[k] = acc / (k as i64);
    }
    for i in 0..g {
        a[2 * g - i] = (1i64 << (g - i)) * a[i];
    }
    a
}

// ---------- places of degree <= 4 ----------
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Pt {
    Aff(u32, u32),
    Inf(u32),
}
fn frob_pt(fld: &Gf, p: Pt) -> Pt {
    match p {
        Pt::Aff(x, y) => Pt::Aff(fld.frob(x), fld.frob(y)),
        Pt::Inf(s) => Pt::Inf(fld.frob(s)),
    }
}
fn pt_degree(fld: &Gf, p: Pt) -> u32 {
    let mut q = p;
    for d in 1..=fld.k {
        q = frob_pt(fld, q);
        if q == p {
            return d;
        }
    }
    unreachable!()
}
fn canon(fld: &Gf, p: Pt) -> Pt {
    let mut best = p;
    let mut q = p;
    for _ in 0..fld.k {
        q = frob_pt(fld, q);
        if q < best {
            best = q;
        }
    }
    best
}

struct Places {
    deg: Vec<u32>,
    index: HashMap<(u32, Pt), usize>, // (field degree k == place degree, canonical point) -> id
}
impl Places {
    fn build(c: &Curve, maxdeg: u32) -> Places {
        let mut deg = vec![];
        let mut index = HashMap::new();
        for d in 1..=maxdeg {
            let fld = FIELDS[d as usize];
            let (aff, inf) = c.points(&fld);
            let all: Vec<Pt> = aff
                .into_iter()
                .map(|(x, y)| Pt::Aff(x, y))
                .chain(inf.into_iter().map(Pt::Inf))
                .collect();
            for p in all {
                if pt_degree(&fld, p) != d {
                    continue;
                }
                let key = (d, canon(&fld, p));
                if let std::collections::hash_map::Entry::Vacant(e) = index.entry(key) {
                    e.insert(deg.len());
                    deg.push(d);
                }
            }
        }
        Places { deg, index }
    }
    fn id(&self, d: u32, p: Pt) -> Option<usize> {
        let fld = FIELDS[d as usize];
        self.index.get(&(d, canon(&fld, p))).copied()
    }
}

/// Pole divisor of x as (place, multiplicity), plus helper for infinity places.
fn inf_places(c: &Curve, pl: &Places) -> Vec<(usize, i64)> {
    match c.inf_type() {
        InfType::Split => vec![
            (pl.id(1, Pt::Inf(0)).unwrap(), 1),
            (pl.id(1, Pt::Inf(1)).unwrap(), 1),
        ],
        InfType::Ramified => {
            let s = c.f2g2() as u32; // sqrt over F_2 is identity
            vec![(pl.id(1, Pt::Inf(s)).unwrap(), 2)]
        }
        InfType::Inert => {
            let f4 = FIELDS[2];
            let s = (0..4u32).find(|&s| f4.mul(s, s) ^ s ^ 1 == 0).unwrap();
            vec![(pl.id(2, Pt::Inf(s)).unwrap(), 1)]
        }
    }
}

/// t-adic valuation of t^D * (a(1/t) + b(1/t) Y(t) / t^(g+1)) at the split point at
/// infinity with Y(0) = s; returns ord_{inf_s}(a + b y).
fn ord_split_inf(c: &Curve, a: u64, b: u64, s: u64) -> i64 {
    let g = c.g;
    const PREC: i32 = 60;
    // Y series
    let ht = |i: i32| coef(c.h, g + 1 - i);
    let ft = |i: i32| coef(c.f, 2 * g + 2 - i);
    let mut y = vec![0u64; PREC as usize];
    y[0] = s;
    for k in 1..PREC {
        let mut v = if k % 2 == 0 { y[(k / 2) as usize] } else { 0 };
        for i in 1..=k.min(g + 1) {
            v ^= ht(i) & y[(k - i) as usize];
        }
        v ^= ft(k);
        y[k as usize] = v;
    }
    let ybits: u64 = y
        .iter()
        .enumerate()
        .fold(0u64, |acc, (k, &v)| acc | (v << k));
    let da = deg(a);
    let db = deg(b);
    let dd = da.max(db + g + 1);
    let rev = |p: u64| -> u64 {
        let d = deg(p);
        let mut r = 0;
        for i in 0..=d {
            r |= coef(p, i) << (d - i);
        }
        r
    };
    let mask = (1u64 << PREC) - 1;
    let mut ser = 0u64;
    if a != 0 {
        ser ^= rev(a) << (dd - da);
    }
    if b != 0 {
        let t = pmul(rev(b) & mask, ybits & mask) & mask;
        ser ^= (t << (dd - db - g - 1)) & mask;
    }
    ser &= mask;
    assert!(ser != 0, "series vanished to precision");
    (ser.trailing_zeros() as i64) - dd as i64
}

/// Divisor of a(x) + b(x) y (b != 0, gcd(a,b) = 1) if its norm is smooth over
/// irreducibles of degree <= maxdeg; else None.
fn divisor_ab(c: &Curve, pl: &Places, a: u64, b: u64, maxdeg: u32) -> Option<Vec<i64>> {
    let n = pmul(a, a) ^ pmul(pmul(a, b), c.h) ^ pmul(pmul(b, b), c.f);
    let mut rem = n;
    let mut fac = vec![];
    for &q in IRRED.iter() {
        if deg(q) as u32 > maxdeg {
            continue;
        }
        let mut e = 0;
        loop {
            let (qq, r) = pdivmod(rem, q);
            if r != 0 {
                break;
            }
            rem = qq;
            e += 1;
        }
        if e > 0 {
            fac.push((q, e));
        }
    }
    if rem != 1 {
        return None;
    }
    let mut dv = vec![0i64; pl.deg.len()];
    let mut zero_deg = 0i64;
    for (q, e) in fac {
        let d = deg(q) as u32;
        let fld = FIELDS[d as usize];
        let x0 = fld.roots(q)[0];
        let bx = fld.eval(b, x0);
        assert!(bx != 0);
        let y0 = fld.mul(fld.eval(a, x0), fld.inv(bx));
        // on curve
        let lhs = fld.mul(y0, y0) ^ fld.mul(fld.eval(c.h, x0), y0) ^ fld.eval(c.f, x0);
        assert!(lhs == 0);
        let id = pl
            .id(d, Pt::Aff(x0, y0))
            .expect("zero place outside factor base");
        dv[id] += e as i64;
        zero_deg += (d as i64) * (e as i64);
    }
    assert_eq!(zero_deg, deg(n) as i64);
    let dn = deg(n) as i64;
    match c.inf_type() {
        InfType::Ramified => {
            let (id, _) = inf_places(c, pl)[0];
            dv[id] -= dn;
        }
        InfType::Inert => {
            let (id, _) = inf_places(c, pl)[0];
            assert!(dn % 2 == 0);
            dv[id] -= dn / 2;
        }
        InfType::Split => {
            let o0 = ord_split_inf(c, a, b, 0);
            let o1 = ord_split_inf(c, a, b, 1);
            assert_eq!(o0 + o1, -dn, "split infinity orders inconsistent");
            dv[pl.id(1, Pt::Inf(0)).unwrap()] += o0;
            dv[pl.id(1, Pt::Inf(1)).unwrap()] += o1;
        }
    }
    Some(dv)
}

/// Divisor of an irreducible q(x) (pure function of x), if its fibre lies in the factor base.
fn divisor_q(c: &Curve, pl: &Places, q: u64, maxdeg: u32) -> Option<Vec<i64>> {
    let d = deg(q) as u32;
    let fld = FIELDS[d as usize];
    let x0 = fld.roots(q)[0];
    let hx = fld.eval(c.h, x0);
    let fx = fld.eval(c.f, x0);
    let ys: Vec<u32> = (0..fld.q())
        .filter(|&y| fld.mul(y, y) ^ fld.mul(hx, y) ^ fx == 0)
        .collect();
    let mut dv = vec![0i64; pl.deg.len()];
    if !ys.is_empty() {
        if hx == 0 {
            dv[pl.id(d, Pt::Aff(x0, ys[0]))?] += 2;
        } else {
            for &y in &ys {
                dv[pl.id(d, Pt::Aff(x0, y))?] += 1;
            }
        }
    } else {
        let d2 = 2 * d;
        if d2 > maxdeg {
            return None;
        }
        let f2 = FIELDS[d2 as usize];
        let x1 = f2.roots(q)[0];
        let hx = f2.eval(c.h, x1);
        let fx = f2.eval(c.f, x1);
        let y1 = (0..f2.q())
            .find(|&y| f2.mul(y, y) ^ f2.mul(hx, y) ^ fx == 0)
            .expect("inert fibre must split in the quadratic extension");
        dv[pl.id(d2, Pt::Aff(x1, y1))?] += 1;
    }
    for (id, m) in inf_places(c, pl) {
        dv[id] -= (d as i64) * m;
    }
    Some(dv)
}

// ---------- diagonalisation over Z ----------
/// Returns (diag, V) with rows(R)·V spanning diag(d_i) rows; coordinates of e are e·V.
fn diagonalise(mut a: Vec<Vec<i128>>, ncols: usize) -> (Vec<i128>, Vec<Vec<i128>>) {
    let nrows = a.len();
    let mut v: Vec<Vec<i128>> = (0..ncols)
        .map(|i| (0..ncols).map(|j| (i == j) as i128).collect())
        .collect();
    let mut diag = vec![];
    let mut t = 0usize;
    let colop = |a: &mut Vec<Vec<i128>>, v: &mut Vec<Vec<i128>>, j: usize, i: usize, q: i128| {
        // col_j -= q * col_i
        for row in a.iter_mut() {
            row[j] -= q * row[i];
        }
        for row in v.iter_mut() {
            row[j] -= q * row[i];
        }
    };
    let colswap = |a: &mut Vec<Vec<i128>>, v: &mut Vec<Vec<i128>>, i: usize, j: usize| {
        for row in a.iter_mut() {
            row.swap(i, j);
        }
        for row in v.iter_mut() {
            row.swap(i, j);
        }
    };
    while t < nrows.min(ncols) {
        let mut best: Option<(usize, usize, i128)> = None;
        for (i, row) in a.iter().enumerate().skip(t) {
            for (j, &x) in row.iter().enumerate().skip(t) {
                if x != 0 && best.is_none_or(|(_, _, bv)| x.abs() < bv) {
                    best = Some((i, j, x.abs()));
                }
            }
        }
        let Some((pi, pj, _)) = best else { break };
        a.swap(t, pi);
        colswap(&mut a, &mut v, t, pj);
        loop {
            let p = a[t][t];
            for i in t + 1..nrows {
                if a[i][t] != 0 {
                    let q = a[i][t] / p;
                    let (head, tail) = a.split_at_mut(i);
                    for (x, y) in tail[0].iter_mut().zip(head[t].iter()) {
                        *x -= q * y;
                    }
                }
            }
            for j in t + 1..ncols {
                if a[t][j] != 0 {
                    let q = a[t][j] / p;
                    colop(&mut a, &mut v, j, t, q);
                }
            }
            // find smallest nonzero remainder in row t / column t
            let mut best: Option<(bool, usize, i128)> = None;
            for i in t + 1..nrows {
                if a[i][t] != 0 && best.is_none_or(|(_, _, bv)| a[i][t].abs() < bv) {
                    best = Some((true, i, a[i][t].abs()));
                }
            }
            for j in t + 1..ncols {
                if a[t][j] != 0 && best.is_none_or(|(_, _, bv)| a[t][j].abs() < bv) {
                    best = Some((false, j, a[t][j].abs()));
                }
            }
            match best {
                None => break,
                Some((true, i, _)) => a.swap(t, i),
                Some((false, j, _)) => colswap(&mut a, &mut v, t, j),
            }
        }
        diag.push(a[t][t].abs());
        t += 1;
    }
    (diag, v)
}

// ---------- complex helpers ----------
#[derive(Clone, Copy)]
struct C(f64, f64);
impl C {
    fn add(self, o: C) -> C {
        C(self.0 + o.0, self.1 + o.1)
    }
    fn mul(self, o: C) -> C {
        C(self.0 * o.0 - self.1 * o.1, self.0 * o.1 + self.1 * o.0)
    }
    fn scale(self, s: f64) -> C {
        C(self.0 * s, self.1 * s)
    }
    fn abs(self) -> f64 {
        self.0.hypot(self.1)
    }
    fn inv(self) -> C {
        let d = self.0 * self.0 + self.1 * self.1;
        C(self.0 / d, -self.1 / d)
    }
    fn root(n: u64, k: u64) -> C {
        let th = 2.0 * std::f64::consts::PI * (k % n) as f64 / n as f64;
        C(th.cos(), th.sin())
    }
}

// ---------- exact arithmetic in Z[zeta_n], n prime ----------
/// Coefficient vector modulo x^n - 1. It is zero in Z[zeta_n] iff all entries are equal
/// (the kernel of Z[x]/(x^n - 1) -> Z[zeta_n] is spanned by 1 + x + ... + x^(n-1)).
type Cyc = Vec<i128>;
fn cyc_mul(a: &Cyc, b: &Cyc) -> Cyc {
    let n = a.len();
    let mut r = vec![0i128; n];
    for (i, &x) in a.iter().enumerate() {
        if x != 0 {
            for (j, &y) in b.iter().enumerate() {
                r[(i + j) % n] += x * y;
            }
        }
    }
    r
}
/// a * s * zeta^m.
fn cyc_shift(a: &Cyc, m: u64, s: i128) -> Cyc {
    let n = a.len();
    let mut r = vec![0i128; n];
    for (i, &x) in a.iter().enumerate() {
        r[(i + m as usize) % n] += s * x;
    }
    r
}
fn cyc_add(a: &mut Cyc, b: &Cyc) {
    for (x, y) in a.iter_mut().zip(b) {
        *x += y;
    }
}
fn cyc_is_zero(a: &Cyc) -> bool {
    a.iter().all(|&x| x == a[0])
}
/// a / k in Z[zeta_n]: in the basis 1, zeta, ..., zeta^(n-2) the quotient must be integral.
fn cyc_div(a: &Cyc, k: i128) -> Cyc {
    let last = a[a.len() - 1];
    a.iter()
        .map(|&x| {
            assert!((x - last) % k == 0, "L-coefficient not in Z[zeta_n]");
            (x - last) / k
        })
        .collect()
}
/// Whether 1 - c t X + 2 c^2 X^2 (c = zeta^m) divides sum_k cf[k] X^k in Z[zeta_n][X]:
/// power-series division from the constant term, Q_k = c_k + c t Q_{k-1} - 2 c^2 Q_{k-2}.
fn exact_divides(cf: &[Cyc], t: i128, m: u64) -> bool {
    let ldeg = cf.len() - 1;
    let mut q: Vec<Cyc> = vec![];
    for k in 0..=ldeg {
        let mut v = cf[k].clone();
        if let Some(q1) = k.checked_sub(1).and_then(|i| q.get(i)) {
            cyc_add(&mut v, &cyc_shift(q1, m, t));
        }
        if let Some(q2) = k.checked_sub(2).and_then(|i| q.get(i)) {
            cyc_add(&mut v, &cyc_shift(q2, 2 * m, -2));
        }
        if k + 2 <= ldeg {
            q.push(v);
        } else if !cyc_is_zero(&v) {
            return false;
        }
    }
    true
}

fn is_prime(n: u64) -> bool {
    n >= 2
        && (2..)
            .take_while(|d| d * d <= n)
            .all(|d| !n.is_multiple_of(d))
}

/// Odd prime factors of |x|.
fn odd_primes(x: i64) -> Vec<u64> {
    let mut x = x.unsigned_abs();
    let mut out = vec![];
    let mut p = 3u64;
    while x.is_multiple_of(2) && x > 0 {
        x /= 2;
    }
    while p * p <= x {
        if x.is_multiple_of(p) {
            out.push(p);
            while x.is_multiple_of(p) {
                x /= p;
            }
        }
        p += 2;
    }
    if x > 1 {
        out.push(x);
    }
    out
}

/// Records the odd primes dividing L(1) = #J(F_2) (untwisted covers) and L(-1) (covers with
/// Frobenius acting by -1 on the Galois group, the only twist allowed in genus 3).
fn note_primes(st: &mut Stats, lp: &[i64]) {
    let l1: i64 = lp.iter().sum();
    let lm1: i64 = lp
        .iter()
        .enumerate()
        .map(|(k, &a)| if k % 2 == 0 { a } else { -a })
        .sum();
    st.primes_l1.extend(odd_primes(l1));
    st.primes_lm1.extend(odd_primes(lm1));
    st.max_j = st.max_j.max(l1);
    st.max_jm = st.max_jm.max(lm1.abs());
}

#[derive(Default)]
struct Stats {
    curves: u64,
    validated: u64,
    unvalidated: u64,
    rh_fail: u64,
    fe_fail: u64,
    nk_checked: u64,
    nk_fail: u64,
    exact_mismatch: u64,
    chars_tested: HashMap<u64, u64>,
    curves_per_n: HashMap<u64, u64>,
    hits: u64,
    primes_l1: BTreeSet<u64>,
    primes_lm1: BTreeSet<u64>,
    max_j: i64,
    max_jm: i64,
}

/// Whether the relation lattice gives free rank 1 and torsion of order `jorder`.
fn group_matches(rows: Vec<Vec<i128>>, np: usize, jorder: i64) -> bool {
    let (diag, _) = diagonalise(rows, np);
    let free = np - diag.iter().filter(|&&d| d != 0).count();
    let tors: i128 = diag.iter().filter(|&&d| d > 1).product();
    free == 1 && tors == jorder as i128
}

/// Diagonalise the relation lattice, validate the group against `L_B(1)`, and test every
/// character of `J_B(F_2)/n` for the trace-zero factor. Returns the hit records, or `None`
/// when the curve fails validation or has no degree-1 place.
///
/// Every twisted L-polynomial must satisfy the functional equation
/// `c_{2g-2-k} = eps 2^(g-1-k) conj(c_k)` with `|eps| = 1`. For each `k` in `extra_k`
/// (every place of degree dividing `k` is in the factor base, `k > 2g - 2`) the direct
/// character sum `N_k(chi)` must equal the value the L-polynomial predicts.
#[allow(clippy::too_many_arguments)]
fn analyse(
    g: i32,
    label: &str,
    pl_deg: &[u32],
    rows: Vec<Vec<i128>>,
    jorder: i64,
    lp: &[i64],
    primes: &[u64],
    extra_k: &[u32],
    st: &mut Stats,
) -> Option<Vec<String>> {
    let np = pl_deg.len();
    let ldeg = (2 * g - 2) as u32;
    let untouched: Vec<u32> = (0..np)
        .filter(|&j| rows.iter().all(|r| r[j] == 0))
        .map(|j| pl_deg[j])
        .collect();
    let (diag, v) = diagonalise(rows, np);
    let free = np - diag.iter().filter(|&&d| d != 0).count();
    let tors: i128 = diag.iter().filter(|&&d| d > 1).product();
    if free != 1 || tors != jorder as i128 {
        st.unvalidated += 1;
        eprintln!(
            "unvalidated g={} {} free={} tors={} #J={} untouched_place_degrees={:?}",
            g, label, free, tors, jorder, untouched
        );
        return None;
    }
    st.validated += 1;
    if std::env::var("ETALE_VERBOSE").is_ok() {
        println!(
            "{{\"kind\":\"curve\",\"g\":{},{},\"L\":{:?},\"J\":{},\"diag\":{:?}}}",
            g, label, lp, jorder, diag
        );
    }
    // base point: first degree-1 place
    let Some(p0) = (0..np).find(|&i| pl_deg[i] == 1) else {
        return None;
    };
    // coordinates of [v - deg(v) P0]
    let coords: Vec<Vec<i128>> = (0..np)
        .map(|vid| {
            let mut e = vec![0i128; np];
            e[vid] += 1;
            e[p0] -= pl_deg[vid] as i128;
            (0..np)
                .map(|j| (0..np).map(|i| e[i] * v[i][j]).sum::<i128>())
                .collect()
        })
        .collect();
    let mut curve_hits = vec![];
    for &n in primes {
        let idx: Vec<usize> = (0..diag.len())
            .filter(|&i| diag[i] > 1 && diag[i] % n as i128 == 0)
            .collect();
        assert!(!idx.is_empty());
        *st.curves_per_n.entry(n).or_default() += 1;
        let r = idx.len() as u32;
        let total = (n as u128).pow(r) as u64;
        for code in 1..total {
            let mut js = vec![];
            let mut cc = code;
            for _ in 0..r {
                js.push(cc % n);
                cc /= n;
            }
            let chi = |vid: usize| -> u64 {
                let mut s = 0u64;
                for (k, &i) in idx.iter().enumerate() {
                    let w = coords[vid][i].rem_euclid(diag[i]) as u64 % n;
                    s = (s + js[k] * w) % n;
                }
                s
            };
            *st.chars_tested.entry(n).or_default() += 1;
            // N_k(chi), k = 1..ldeg, in floating point and exactly in Z[zeta_n]
            let mut nk = vec![C(0.0, 0.0); ldeg as usize + 1];
            let mut nkx: Vec<Cyc> = vec![vec![0i128; n as usize]; ldeg as usize + 1];
            let mut cnt1 = vec![0i64; n as usize];
            for vid in 0..np {
                let dvv = pl_deg[vid];
                let x = chi(vid);
                if dvv == 1 {
                    cnt1[x as usize] += 1;
                }
                for k in 1..=ldeg {
                    if k % dvv == 0 {
                        nk[k as usize] =
                            nk[k as usize].add(C::root(n, x * (k / dvv) as u64).scale(dvv as f64));
                        nkx[k as usize][((x * (k / dvv) as u64) % n) as usize] += dvv as i128;
                    }
                }
            }
            // Newton: k c_k = sum_{i=1..k} N_i c_{k-i}
            let mut cf = vec![C(0.0, 0.0); ldeg as usize + 1];
            cf[0] = C(1.0, 0.0);
            for k in 1..=ldeg as usize {
                let mut acc = C(0.0, 0.0);
                for i in 1..=k {
                    acc = acc.add(nk[i].mul(cf[k - i]));
                }
                cf[k] = acc.scale(1.0 / k as f64);
            }
            let top = cf[ldeg as usize].abs();
            if (top - (1u64 << (g - 1)) as f64).abs() > 1e-6 {
                st.rh_fail += 1;
                eprintln!("RH check failed g={} {} n={} |top|={}", g, label, n, top);
                continue;
            }
            let eps = cf[ldeg as usize].scale(1.0 / (1u64 << (g - 1)) as f64);
            let fe_ok = (0..=ldeg as usize).all(|k| {
                let q = 2f64.powi(g - 1 - k as i32);
                let want = eps.mul(C(cf[k].0, -cf[k].1)).scale(q);
                let got = cf[ldeg as usize - k];
                C(got.0 - want.0, got.1 - want.1).abs() < 1e-6
            });
            if !fe_ok {
                st.fe_fail += 1;
                eprintln!("functional equation failed g={} {} n={}", g, label, n);
                continue;
            }
            // out-of-sample prediction: k N_k = k c_k - sum_{i<k} N_i c_{k-i}
            if let Some(&kmax) = extra_k.iter().max() {
                let mut nn = nk.clone();
                for k in ldeg as usize + 1..=kmax as usize {
                    let mut acc = C(0.0, 0.0);
                    for i in 1..k {
                        if k - i <= ldeg as usize {
                            acc = acc.add(nn[i].mul(cf[k - i]));
                        }
                    }
                    nn.push(acc.scale(-1.0));
                }
                for &k in extra_k {
                    let mut direct = C(0.0, 0.0);
                    for vid in 0..np {
                        let dvv = pl_deg[vid];
                        if k % dvv == 0 {
                            direct = direct
                                .add(C::root(n, chi(vid) * (k / dvv) as u64).scale(dvv as f64));
                        }
                    }
                    st.nk_checked += 1;
                    let pred = nn[k as usize];
                    if C(direct.0 - pred.0, direct.1 - pred.1).abs() > 1e-6 {
                        st.nk_fail += 1;
                        eprintln!("N_{} prediction failed g={} {} n={}", k, g, label, n);
                    }
                }
            }
            // exact Newton in Z[zeta_n]; integrality is asserted at every division
            let mut cfx: Vec<Cyc> = vec![vec![0i128; n as usize]];
            cfx[0][0] = 1;
            for k in 1..=ldeg as usize {
                let mut acc = vec![0i128; n as usize];
                for i in 1..=k {
                    cyc_add(&mut acc, &cyc_mul(&nkx[i], &cfx[k - i]));
                }
                cfx.push(cyc_div(&acc, k as i128));
            }
            let evalp = |x: C| -> C {
                let mut acc = C(0.0, 0.0);
                for k in (0..=ldeg as usize).rev() {
                    acc = acc.mul(x).add(cf[k]);
                }
                acc
            };
            for t in [1i64, -1] {
                let s7 = 7f64.sqrt();
                let pis = [C(t as f64 / 2.0, s7 / 2.0), C(t as f64 / 2.0, -s7 / 2.0)];
                for m in 0..n {
                    let cm = C::root(n, m);
                    let ok_float = pis.iter().all(|&p| evalp(cm.mul(p).inv()).abs() < 1e-7);
                    let ok = exact_divides(&cfx, t as i128, m);
                    if ok != ok_float {
                        st.exact_mismatch += 1;
                        eprintln!(
                            "exact/float mismatch g={} {} n={} chi={:?} t={} m={}",
                            g, label, n, js, t, m
                        );
                    }
                    if ok {
                        st.hits += 1;
                        curve_hits.push(format!(
                            "{{\"n\":{},\"chi\":{:?},\"t\":{},\"c_exp\":{},\"cnt_deg1\":{:?}}}",
                            n, js, t, m, cnt1
                        ));
                    }
                }
            }
        }
    }
    Some(curve_hits)
}

fn run_genus(g: i32, st: &mut Stats) {
    let maxdeg = 4u32;
    let hmax = 1u64 << (g + 2);
    let fmax = 1u64 << (2 * g + 3);
    let (adeg, bdeg) = if g == 2 { (5, 2) } else { (6, 2) };
    for h in 1..hmax {
        for f in 0..fmax {
            let c = Curve { g, h, f };
            if !c.is_smooth_genus() {
                continue;
            }
            st.curves += 1;
            let lp = lpoly(&c);
            note_primes(st, &lp);
            let jorder: i64 = lp.iter().sum();
            assert!(jorder > 0);
            let primes: Vec<u64> = (3..=jorder as u64)
                .filter(|&p| is_prime(p) && (jorder as u64).is_multiple_of(p))
                .collect();
            if primes.is_empty() {
                continue;
            }
            let pl = Places::build(&c, maxdeg);
            // relations
            let mut rows: Vec<Vec<i128>> = vec![];
            for &q in IRRED.iter() {
                if let Some(dv) = divisor_q(&c, &pl, q, maxdeg) {
                    rows.push(dv.into_iter().map(|x| x as i128).collect());
                }
            }
            for b in 1..(1u64 << (bdeg + 1)) {
                for a in 0..(1u64 << (adeg + 1)) {
                    if deg(pgcd(a, b)) > 0 {
                        continue;
                    }
                    if let Some(dv) = divisor_ab(&c, &pl, a, b, maxdeg) {
                        rows.push(dv.into_iter().map(|x| x as i128).collect());
                    }
                }
            }
            let label = format!("\"h\":\"{:#b}\",\"f\":\"{:#b}\"", h, f);
            let Some(curve_hits) = analyse(g, &label, &pl.deg, rows, jorder, &lp, &primes, &[], st)
            else {
                continue;
            };
            if !curve_hits.is_empty() {
                println!(
                    "{{\"kind\":\"hit\",\"g\":{},{},\"N\":{:?},\"L\":{:?},\"J\":{},\"inf\":\"{:?}\",\"hits\":[{}]}}",
                    g,
                    label,
                    (1..=g as u32).map(|k| c.count(k)).collect::<Vec<_>>(),
                    lp,
                    jorder,
                    c.inf_type(),
                    curve_hits.join(",")
                );
            }
        }
    }
}

fn json_map(m: &HashMap<u64, u64>) -> String {
    let mut kv: Vec<_> = m.iter().collect();
    kv.sort();
    let body: Vec<String> = kv.iter().map(|(k, v)| format!("\"{}\":{}", k, v)).collect();
    format!("{{{}}}", body.join(","))
}

fn summary(g: i32, family: &str, st: &Stats) {
    println!(
        "{{\"kind\":\"summary\",\"g\":{},{}\"smooth_models\":{},\"validated\":{},\"unvalidated\":{},\"rh_fail\":{},\"fe_fail\":{},\"nk_checked\":{},\"nk_fail\":{},\"exact_mismatch\":{},\"hits\":{},\"curves_per_n\":{},\"chars_per_n\":{},\"max_L1\":{},\"max_abs_Lm1\":{},\"odd_primes_L1\":{:?},\"odd_primes_Lm1\":{:?}}}",
        g,
        family,
        st.curves,
        st.validated,
        st.unvalidated,
        st.rh_fail,
        st.fe_fail,
        st.nk_checked,
        st.nk_fail,
        st.exact_mismatch,
        st.hits,
        json_map(&st.curves_per_n),
        json_map(&st.chars_tested),
        st.max_j,
        st.max_jm,
        st.primes_l1.iter().collect::<Vec<_>>(),
        st.primes_lm1.iter().collect::<Vec<_>>()
    );
}

/// Arguments: genera of the hyperelliptic search (default `2 3`), or `q` for plane quartics.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("q") {
        let mut st = Stats::default();
        let fallback = quartic::run(&mut st);
        let family = format!(
            "\"family\":\"plane_quartic\",\"quartic_sections_used\":{},",
            fallback
        );
        summary(3, &family, &st);
        return;
    }
    let genera: Vec<i32> = if args.len() > 1 {
        args[1..].iter().map(|s| s.parse().unwrap()).collect()
    } else {
        vec![2, 3]
    };
    for g in genera {
        let mut st = Stats::default();
        run_genus(g, &mut st);
        summary(g, "", &st);
    }
}
