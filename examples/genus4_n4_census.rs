//! Genus-4 non-hyperelliptic curves over `F_q` (q prime) whose Jacobian is
//! isogenous to `Res_{F_{q^4}/F_q} E`, `E/F_{q^4}` of prime order: a sieve
//! census at `q = 3, 5, 7`, the Tian-kernel cross-table at `n = 4`, and the
//! plane-quintic models.
//!
//! Round 5 of the Teske-variant search, lead 3
//! (`research/genus4_n4_census_20261024/PROTOCOL.md`).
//!
//! Parametrisation. A smooth non-hyperelliptic genus-4 curve over `F_q` is its
//! canonical model, a complete intersection `V(Q, F)` in `P^3`, `Q` a quadric
//! (unique, hence defined over `F_q`) of rank 4 or 3, `F` a cubic modulo
//! `Q * (linear forms)` (a 16-dimensional space). Over `F_q` (q odd) quadrics of
//! rank >= 3 have three `PGL_4`-classes: the split `P^1 x P^1` (Witt index 2),
//! the non-split (Witt index 1, `Res_{F_{q^2}/F_q} P^1`) and the cone
//! `P(1,1,2)`. A form is a point of `F_q^16` (split: a `(3,3)` form
//! `sum a_ij x^i y^j`; non-split: the same with `a_ij in F_{q^2}` and
//! `a_ji = conj(a_ij)`; cone: monic `u^3 + a2 u^2 + a4 u + a6`, `a_d` binary of
//! degree `d`, `q^15` forms).
//!
//! Matching. `L_C(T) = 1 - t T^4 + q^4 T^8` (the Weil polynomial of
//! `Res E` is `chi_E(T^4)`) iff `#C(F_{q^k}) = q^k + 1` for `k = 1, 2, 3`
//! and `#C(F_{q^4}) = q^4 + 1 - 4t` (Newton's identities); equal L-polynomials
//! give isogenous Jacobians (Tate). Smoothness is exact: `V(Q,F)` is smooth iff
//! the ideal `(Q, F, 2x2 minors of the Jacobian)` has `J_8 = S_8` (a regular
//! sequence of degrees 2,3,3,3 has socle degree 7).
//!
//! std-only; `rustc --edition 2021 -O examples/genus4_n4_census.rs`.
//! Usage: see `main`.
#![allow(dead_code)]
#![allow(
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_range_loop,
    clippy::unusual_byte_groupings,
    clippy::write_with_newline,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::manual_memcpy,
    clippy::wrong_self_convention
)]

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

// ---------------------------------------------------------------------------
// Small helpers.
// ---------------------------------------------------------------------------

fn splitmix(s: &mut u64) -> u64 {
    *s = s.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *s;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

fn prim_root_p(p: u32) -> u32 {
    for g in 2..p.max(3) {
        let mut x = 1u64;
        let mut ok = true;
        for k in 1..(p - 1) {
            x = x * g as u64 % p as u64;
            if x == 1 && k < p - 1 {
                ok = false;
                break;
            }
        }
        if ok {
            return g;
        }
    }
    1
}

fn nonresidue(p: u32) -> u32 {
    (2..p)
        .find(|&v| powmod(v as u64, (p as u64 - 1) / 2, p as u64) == p as u64 - 1)
        .expect("nonresidue")
}

fn inv_p(a: u32, p: u32) -> u32 {
    powmod(a as u64, p as u64 - 2, p as u64) as u32
}

fn binom(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    let mut r = 1u64;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

fn sha_hex_placeholder() -> &'static str {
    "see env.txt"
}

// ---------------------------------------------------------------------------
// Table fields F_{p^j} in log form; ZERO = n1 = p^j - 1.
// ---------------------------------------------------------------------------

struct Fld {
    p: u32,
    j: u32,
    sz: u32,
    n1: u32,
    exp: Vec<u32>,
    log: Vec<u32>,
    zech: Vec<u32>,
    t2: Vec<u8>,
    t3: Vec<u8>,
    inv3: u32,
    two: u32,
    /// log of a square root of the least non-residue of F_p (j even)
    w: u32,
}

fn mulx(cur: &mut [u32], f: &[u32], p: u32) {
    let j = cur.len();
    let top = cur[j - 1];
    for i in (1..j).rev() {
        cur[i] = cur[i - 1];
    }
    cur[0] = 0;
    if top != 0 {
        for i in 0..j {
            cur[i] = (cur[i] + (p - f[i]) % p * top) % p;
        }
    }
}

fn find_primitive(p: u32, j: u32) -> Vec<u32> {
    let sz = p.pow(j);
    let n1 = sz - 1;
    let mut cand = 1u32;
    loop {
        let mut f = vec![0u32; j as usize];
        let mut c = cand;
        for i in 0..j as usize {
            f[i] = c % p;
            c /= p;
        }
        cand += 1;
        if f[0] == 0 {
            continue;
        }
        let mut one = vec![0u32; j as usize];
        one[0] = 1;
        let mut cur = one.clone();
        let mut ok = false;
        for k in 1..=n1 {
            mulx(&mut cur, &f, p);
            if cur == one {
                ok = k == n1;
                break;
            }
        }
        if ok {
            return f;
        }
    }
}

impl Fld {
    fn new(p: u32, j: u32, tables: bool) -> Fld {
        let sz = p.pow(j);
        let n1 = sz - 1;
        let f = find_primitive(p, j);
        let mut exp = vec![0u32; n1 as usize];
        let mut log = vec![0u32; sz as usize];
        let mut cur = vec![0u32; j as usize];
        cur[0] = 1;
        for k in 0..n1 {
            let mut idx = 0u32;
            for i in (0..j as usize).rev() {
                idx = idx * p + cur[i];
            }
            exp[k as usize] = idx;
            log[idx as usize] = k;
            mulx(&mut cur, &f, p);
        }
        log[0] = n1;
        let mut zech = vec![0u32; n1 as usize];
        for d in 0..n1 as usize {
            let idx = exp[d];
            let nidx = if idx % p == p - 1 {
                idx - (p - 1)
            } else {
                idx + 1
            };
            zech[d] = if nidx == 0 { n1 } else { log[nidx as usize] };
        }
        let mut fl = Fld {
            p,
            j,
            sz,
            n1,
            exp,
            log,
            zech,
            t2: Vec::new(),
            t3: Vec::new(),
            inv3: 0,
            two: 0,
            w: 0,
        };
        fl.two = fl.from_int(2);
        if p != 3 {
            fl.inv3 = fl.inv(fl.from_int(3));
        }
        if j % 2 == 0 {
            let nu = nonresidue(p);
            let ln = fl.from_int(nu as i64);
            debug_assert!(ln % 2 == 0);
            fl.w = ln / 2;
        }
        if tables {
            fl.build_cubic();
        }
        fl
    }
    #[inline]
    fn zero(&self) -> u32 {
        self.n1
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == self.n1 || b == self.n1 {
            self.n1
        } else {
            let s = a + b;
            if s >= self.n1 {
                s - self.n1
            } else {
                s
            }
        }
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        if a == self.n1 {
            return b;
        }
        if b == self.n1 {
            return a;
        }
        let d = if b >= a { b - a } else { b + self.n1 - a };
        let z = self.zech[d as usize];
        if z == self.n1 {
            self.n1
        } else {
            let s = a + z;
            if s >= self.n1 {
                s - self.n1
            } else {
                s
            }
        }
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == self.n1 {
            a
        } else {
            let s = a + self.n1 / 2;
            if s >= self.n1 {
                s - self.n1
            } else {
                s
            }
        }
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.add(a, self.neg(b))
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        if a == 0 {
            0
        } else {
            self.n1 - a
        }
    }
    fn from_int(&self, c: i64) -> u32 {
        let m = c.rem_euclid(self.p as i64) as usize;
        if m == 0 {
            self.n1
        } else {
            self.log[m]
        }
    }
    fn embed2(&self, c0: u32, c1: u32) -> u32 {
        let a = self.from_int(c0 as i64);
        let b = self.mul(self.from_int(c1 as i64), self.w);
        self.add(a, b)
    }
    fn pow_small(&self, a: u32, e: u32) -> u32 {
        let mut r = 0u32;
        for _ in 0..e {
            r = self.mul(r, a);
        }
        r
    }
    fn is_square(&self, a: u32) -> bool {
        a != self.n1 && a % 2 == 0
    }
    fn build_cubic(&mut self) {
        let n = self.sz as usize;
        let z = self.n1;
        if self.p != 3 {
            let mut t = vec![0u8; n * n];
            for pl in 0..=z {
                for zl in 0..=z {
                    let z3 = self.mul(self.mul(zl, zl), zl);
                    let s = self.add(z3, self.mul(pl, zl));
                    let r = self.neg(s);
                    t[pl as usize * n + r as usize] += 1;
                }
            }
            self.t2 = t;
        } else {
            let mut t = vec![0u8; n * n * n];
            for al in 0..=z {
                for bl in 0..=z {
                    for zl in 0..=z {
                        let z2 = self.mul(zl, zl);
                        let z3 = self.mul(z2, zl);
                        let s = self.add(self.add(z3, self.mul(al, z2)), self.mul(bl, zl));
                        let c = self.neg(s);
                        t[(al as usize * n + bl as usize) * n + c as usize] += 1;
                    }
                }
            }
            self.t3 = t;
        }
    }
    /// Distinct roots in `F` of `z^3 + a z^2 + b z + c`.
    #[inline]
    fn monic_roots(&self, a: u32, b: u32, c: u32) -> u8 {
        let n = self.sz as usize;
        if self.p == 3 {
            self.t3[(a as usize * n + b as usize) * n + c as usize]
        } else {
            let a3 = self.mul(a, self.inv3);
            let pp = self.sub(b, self.mul(a, a3));
            let rr = self.add(
                self.sub(c, self.mul(b, a3)),
                self.mul(self.two, self.mul(a3, self.mul(a3, a3))),
            );
            self.t2[pp as usize * n + rr as usize]
        }
    }
    /// Distinct roots in `P^1(F)` of the binary cubic `c3 y^3 + c2 y^2 + c1 y + c0`
    /// (`sz + 1` if the form vanishes identically).
    #[inline]
    fn cubic_form_roots(&self, c0: u32, c1: u32, c2: u32, c3: u32) -> u64 {
        let z = self.n1;
        if c3 != z {
            let ic = self.inv(c3);
            let a = self.mul(c2, ic);
            let b = self.mul(c1, ic);
            let c = self.mul(c0, ic);
            return self.monic_roots(a, b, c) as u64;
        }
        // infinity is a root
        if c2 != z {
            let disc = self.sub(
                self.mul(c1, c1),
                self.mul(self.mul(self.from_int(4), c0), c2),
            );
            if disc == z {
                2
            } else if self.is_square(disc) {
                3
            } else {
                1
            }
        } else if c1 != z {
            2
        } else if c0 != z {
            1
        } else {
            self.sz as u64 + 1
        }
    }
}

// ---------------------------------------------------------------------------
// Curves and point counts.
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum Ty {
    Split,
    Nonsplit,
    Cone,
}

impl Ty {
    fn name(self) -> &'static str {
        match self {
            Ty::Split => "split",
            Ty::Nonsplit => "nonsplit",
            Ty::Cone => "cone",
        }
    }
    fn from_name(s: &str) -> Option<Ty> {
        match s {
            "split" => Some(Ty::Split),
            "nonsplit" => Some(Ty::Nonsplit),
            "cone" => Some(Ty::Cone),
            _ => None,
        }
    }
    fn ndigits(self) -> usize {
        match self {
            Ty::Cone => 15,
            _ => 16,
        }
    }
    fn ncodes(self, p: u32) -> u64 {
        (p as u64).pow(self.ndigits() as u32)
    }
    /// `|G|` of the group acting on forms (without the scalar factor `q - 1`
    /// for split and non-split; the cone forms are monic so no scalar).
    fn group_order(self, q: u64) -> u64 {
        match self {
            Ty::Split => 2 * (q * (q * q - 1)).pow(2),
            Ty::Nonsplit => 2 * q * q * (q.pow(4) - 1),
            Ty::Cone => (q * q - 1) * (q * q - q) * q.pow(3),
        }
    }
    /// Number of `(form, scalar)` pairs per isomorphism-class orbit factor:
    /// mass of a set of forms `S` is `|S| / (scalar_factor * group_order)`.
    fn scalar_factor(self, q: u64) -> u64 {
        match self {
            Ty::Cone => 1,
            _ => q - 1,
        }
    }
    /// Number of forms in the whole space of this type (counting the zero form
    /// for the split and non-split spaces; it is singular and never a hit).
    fn total_forms(self, q: u64) -> u64 {
        q.pow(self.ndigits() as u32)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Curve {
    Split([[u32; 4]; 4]),
    Nonsplit([[(u32, u32); 4]; 4]),
    Cone([u32; 3], [u32; 5], [u32; 7]),
    Hyper([u32; 11]),
}

const PAIRS: [(usize, usize); 6] = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

fn decode(ty: Ty, p: u32, mut code: u64) -> Curve {
    let mut d = [0u32; 16];
    for x in d.iter_mut().take(ty.ndigits()) {
        *x = (code % p as u64) as u32;
        code /= p as u64;
    }
    match ty {
        Ty::Split => {
            let mut a = [[0u32; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    a[i][j] = d[4 * i + j];
                }
            }
            Curve::Split(a)
        }
        Ty::Nonsplit => {
            let mut a = [[(0u32, 0u32); 4]; 4];
            for i in 0..4 {
                a[i][i] = (d[i], 0);
            }
            for (k, &(i, j)) in PAIRS.iter().enumerate() {
                let c0 = d[4 + 2 * k];
                let c1 = d[5 + 2 * k];
                a[i][j] = (c0, c1);
                a[j][i] = (c0, (p - c1) % p);
            }
            Curve::Nonsplit(a)
        }
        Ty::Cone => {
            let mut a2 = [0u32; 3];
            let mut a4 = [0u32; 5];
            let mut a6 = [0u32; 7];
            a2.copy_from_slice(&d[0..3]);
            a4.copy_from_slice(&d[3..8]);
            a6.copy_from_slice(&d[8..15]);
            Curve::Cone(a2, a4, a6)
        }
    }
}

fn encode(p: u32, c: &Curve) -> u64 {
    let mut d: Vec<u32> = Vec::new();
    match c {
        Curve::Split(a) => {
            for i in 0..4 {
                for j in 0..4 {
                    d.push(a[i][j]);
                }
            }
        }
        Curve::Nonsplit(a) => {
            for i in 0..4 {
                d.push(a[i][i].0);
            }
            for &(i, j) in PAIRS.iter() {
                d.push(a[i][j].0);
                d.push(a[i][j].1);
            }
        }
        Curve::Cone(a2, a4, a6) => {
            d.extend_from_slice(a2);
            d.extend_from_slice(a4);
            d.extend_from_slice(a6);
        }
        Curve::Hyper(c) => d.extend_from_slice(c),
    }
    let mut code = 0u64;
    for &x in d.iter().rev() {
        code = code * p as u64 + x as u64;
    }
    code
}

fn split_fibre_count(f: &Fld, a: &[[u32; 4]; 4]) -> u64 {
    let mut total = f.cubic_form_roots(a[3][0], a[3][1], a[3][2], a[3][3]);
    for x in 0..=f.n1 {
        let mut c = [0u32; 4];
        for j in 0..4 {
            let mut v = a[3][j];
            v = f.add(f.mul(v, x), a[2][j]);
            v = f.add(f.mul(v, x), a[1][j]);
            v = f.add(f.mul(v, x), a[0][j]);
            c[j] = v;
        }
        total += f.cubic_form_roots(c[0], c[1], c[2], c[3]);
    }
    total
}

/// Non-split quadric, odd `k`: points `X in P^1(F_{q^{2k}})`, `Y = X^{q^k}`.
/// `f` is `F_{p^{2k}}`, `qp = p^k`.
fn nonsplit_odd_count(f: &Fld, a: &[[u32; 4]; 4], qp: u64) -> u64 {
    let n1 = f.n1 as u64;
    let z = f.n1;
    let mut total = 0u64;
    if a[3][3] == z {
        total += 1;
    }
    if a[0][0] == z {
        total += 1;
    }
    let mut terms: Vec<(usize, usize, u32)> = Vec::with_capacity(16);
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] != z {
                terms.push((i, j, a[i][j]));
            }
        }
    }
    for e in 0..n1 {
        let ye = e * qp % n1;
        let mut xe = [0u64; 4];
        let mut yy = [0u64; 4];
        for i in 1..4 {
            xe[i] = (xe[i - 1] + e) % n1;
            yy[i] = (yy[i - 1] + ye) % n1;
        }
        let mut s = z;
        for &(i, j, l) in &terms {
            let t = ((l as u64 + xe[i] + yy[j]) % n1) as u32;
            s = f.add(s, t);
        }
        if s == z {
            total += 1;
        }
    }
    total
}

fn horner(f: &Fld, c: &[u32], s: u32) -> u32 {
    let mut v = c[0];
    for &ci in &c[1..] {
        v = f.add(f.mul(v, s), ci);
    }
    v
}

fn cone_count(f: &Fld, a2: &[u32], a4: &[u32], a6: &[u32]) -> u64 {
    let mut total = f.monic_roots(a2[0], a4[0], a6[0]) as u64;
    for s in 0..=f.n1 {
        total += f.monic_roots(horner(f, a2, s), horner(f, a4, s), horner(f, a6, s)) as u64;
    }
    total
}

fn hyper_count(f: &Fld, e: &[u32; 11]) -> u64 {
    let mut tot = 0u64;
    for x in 0..=f.n1 {
        let mut v = e[10];
        for i in (0..10).rev() {
            v = f.add(f.mul(v, x), e[i]);
        }
        tot += if v == f.n1 {
            1
        } else if v % 2 == 0 {
            2
        } else {
            0
        };
    }
    tot += if e[10] != f.n1 {
        if e[10] % 2 == 0 {
            2
        } else {
            0
        }
    } else {
        1
    };
    tot
}

struct Engine {
    p: u32,
    f: Vec<Option<Fld>>,
}

impl Engine {
    fn new(p: u32) -> Engine {
        let mut f: Vec<Option<Fld>> = (0..7).map(|_| None).collect();
        for j in [1u32, 2, 3, 4, 6] {
            f[j as usize] = Some(Fld::new(p, j, j <= 4));
        }
        Engine { p, f }
    }
    fn fld(&self, j: u32) -> &Fld {
        self.f[j as usize].as_ref().expect("field")
    }
    fn count(&self, c: &Curve, k: u32) -> u64 {
        match c {
            Curve::Split(a) => {
                let f = self.fld(k);
                let mut e = [[0u32; 4]; 4];
                for i in 0..4 {
                    for j in 0..4 {
                        e[i][j] = f.from_int(a[i][j] as i64);
                    }
                }
                split_fibre_count(f, &e)
            }
            Curve::Nonsplit(a) => {
                let jj = if k % 2 == 0 { k } else { 2 * k };
                let f = self.fld(jj);
                let mut e = [[0u32; 4]; 4];
                for i in 0..4 {
                    for j in 0..4 {
                        e[i][j] = f.embed2(a[i][j].0, a[i][j].1);
                    }
                }
                if k % 2 == 0 {
                    split_fibre_count(f, &e)
                } else {
                    nonsplit_odd_count(f, &e, (self.p as u64).pow(k))
                }
            }
            Curve::Cone(a2, a4, a6) => {
                let f = self.fld(k);
                let e2: Vec<u32> = a2.iter().map(|&c| f.from_int(c as i64)).collect();
                let e4: Vec<u32> = a4.iter().map(|&c| f.from_int(c as i64)).collect();
                let e6: Vec<u32> = a6.iter().map(|&c| f.from_int(c as i64)).collect();
                cone_count(f, &e2, &e4, &e6)
            }
            Curve::Hyper(c) => {
                let f = self.fld(k);
                let mut e = [0u32; 11];
                for i in 0..11 {
                    e[i] = f.from_int(c[i] as i64);
                }
                hyper_count(f, &e)
            }
        }
    }
    /// The sieve: `Some(t)` iff `N_k = q^k + 1` for `k = 1, 2, 3` and
    /// `N_4 = q^4 + 1 - 4t` with `4 | q^4 + 1 - N_4`. With `allowed`
    /// (indexed by `t + 2q^2`), `N_4` is tested before `N_3` and only allowed
    /// traces pass.
    fn sieve(&self, c: &Curve, allowed: Option<&[bool]>) -> Option<i64> {
        let q = self.p as u64;
        if self.count(c, 1) != q + 1 {
            return None;
        }
        if self.count(c, 2) != q * q + 1 {
            return None;
        }
        let mut n4 = None;
        if let Some(al) = allowed {
            let n = self.count(c, 4);
            let d = (q.pow(4) + 1) as i64 - n as i64;
            if d % 4 != 0 {
                return None;
            }
            let t = d / 4;
            let ix = t + 2 * (q * q) as i64;
            if ix < 0 || ix as usize >= al.len() || !al[ix as usize] {
                return None;
            }
            n4 = Some(n);
        }
        if self.count(c, 3) != q.pow(3) + 1 {
            return None;
        }
        let n = match n4 {
            Some(n) => n,
            None => self.count(c, 4),
        };
        let d = (q.pow(4) + 1) as i64 - n as i64;
        if d % 4 != 0 {
            return None;
        }
        Some(d / 4)
    }
}

// ---------------------------------------------------------------------------
// Elliptic curves over F_{q^4}: existence (Waterhouse, even exponent) and
// prime order.
// ---------------------------------------------------------------------------

fn ell_exists(p: u64, q: u64, t: i64) -> bool {
    let sq = (q * q) as i64;
    if t.abs() > 2 * sq {
        return false;
    }
    if t % (p as i64) != 0 {
        return true;
    }
    t.abs() == 2 * sq || (t.abs() == sq && p % 3 != 1) || (t == 0 && p % 4 != 1)
}

fn ell_order(q: u64, t: i64) -> i64 {
    (q.pow(4) + 1) as i64 - t
}

fn allowed_traces(p: u32) -> Vec<bool> {
    let q = p as u64;
    let w = 4 * (q * q) as usize + 1;
    let mut v = vec![false; w];
    for ix in 0..w {
        let t = ix as i64 - 2 * (q * q) as i64;
        let n = ell_order(q, t);
        v[ix] = ell_exists(q, q, t) && n > 1 && is_prime(n as u64);
    }
    v
}

// ---------------------------------------------------------------------------
// Sparse polynomials in x0..x3 over F_p (smoothness, lifts, plane models).
// ---------------------------------------------------------------------------

type Mono = [u8; 4];
type SPoly = Vec<(Mono, u32)>;

fn sp_norm(mut v: Vec<(Mono, u32)>, p: u32) -> SPoly {
    v.sort();
    let mut out: SPoly = Vec::with_capacity(v.len());
    for (m, c) in v {
        if let Some(last) = out.last_mut() {
            if last.0 == m {
                last.1 = (last.1 + c) % p;
                continue;
            }
        }
        out.push((m, c % p));
    }
    out.retain(|x| x.1 != 0);
    out
}

fn sp_mul(a: &SPoly, b: &SPoly, p: u32) -> SPoly {
    let mut v = Vec::with_capacity(a.len() * b.len());
    for &(ma, ca) in a {
        for &(mb, cb) in b {
            let m = [ma[0] + mb[0], ma[1] + mb[1], ma[2] + mb[2], ma[3] + mb[3]];
            v.push((m, ca * cb % p));
        }
    }
    sp_norm(v, p)
}

fn sp_add(a: &SPoly, b: &SPoly, p: u32) -> SPoly {
    let mut v = a.clone();
    v.extend_from_slice(b);
    sp_norm(v, p)
}

fn sp_scale(a: &SPoly, c: u32, p: u32) -> SPoly {
    sp_norm(a.iter().map(|&(m, x)| (m, x * c % p)).collect(), p)
}

fn sp_sub(a: &SPoly, b: &SPoly, p: u32) -> SPoly {
    sp_add(a, &sp_scale(b, p - 1, p), p)
}

fn sp_deriv(a: &SPoly, var: usize, p: u32) -> SPoly {
    let mut v = Vec::new();
    for &(m, c) in a {
        if m[var] > 0 {
            let mut m2 = m;
            m2[var] -= 1;
            v.push((m2, c * (m[var] as u32 % p) % p));
        }
    }
    sp_norm(v, p)
}

fn sp_var(i: usize) -> SPoly {
    let mut m = [0u8; 4];
    m[i] = 1;
    vec![(m, 1)]
}

fn sp_const(c: u32, p: u32) -> SPoly {
    sp_norm(vec![([0; 4], c)], p)
}

/// Evaluate at a point of `F_{p^j}^4` (logs; `zero` = sentinel).
fn sp_eval(f: &Fld, a: &SPoly, x: &[u32; 4]) -> u32 {
    let z = f.zero();
    let n1 = f.n1 as u64;
    let mut s = z;
    'terms: for &(m, c) in a {
        let mut l = f.from_int(c as i64) as u64;
        for i in 0..4 {
            if m[i] > 0 {
                if x[i] == z {
                    continue 'terms;
                }
                l = (l + m[i] as u64 * x[i] as u64) % n1;
            }
        }
        s = f.add(s, l as u32);
    }
    s
}

/// Monomials of degree `d` in 4 variables.
fn monomials(d: usize) -> Vec<Mono> {
    let mut v = Vec::new();
    for a in 0..=d {
        for b in 0..=(d - a) {
            for c in 0..=(d - a - b) {
                v.push([a as u8, b as u8, c as u8, (d - a - b - c) as u8]);
            }
        }
    }
    v
}

fn mono_key(m: &Mono) -> usize {
    ((m[0] as usize * 9 + m[1] as usize) * 9 + m[2] as usize) * 9 + m[3] as usize
}

/// Exact smoothness of `V(Q, F)` in `P^3` over the algebraic closure:
/// `J_8 = S_8` for `J = (Q, F, 2x2 minors of d(Q,F))`.
fn smooth_ci(p: u32, q: &SPoly, f: &SPoly) -> bool {
    let dq: Vec<SPoly> = (0..4).map(|v| sp_deriv(q, v, p)).collect();
    let df: Vec<SPoly> = (0..4).map(|v| sp_deriv(f, v, p)).collect();
    let mut gens: Vec<(SPoly, usize)> = vec![(q.clone(), 2), (f.clone(), 3)];
    for i in 0..4 {
        for j in (i + 1)..4 {
            let m = sp_sub(&sp_mul(&dq[i], &df[j], p), &sp_mul(&dq[j], &df[i], p), p);
            if !m.is_empty() {
                gens.push((m, 3));
            }
        }
    }
    let target = 8usize;
    let m8 = monomials(target);
    let mut key8 = vec![usize::MAX; 9 * 9 * 9 * 9];
    for (i, m) in m8.iter().enumerate() {
        key8[mono_key(m)] = i;
    }
    let ncol = m8.len();
    let invs: Vec<u32> = (0..p)
        .map(|a| if a == 0 { 0 } else { inv_p(a, p) })
        .collect();
    // incremental echelon basis
    let mut basis: Vec<Option<Vec<u8>>> = vec![None; ncol];
    let mut rank = 0usize;
    for (g, dg) in &gens {
        for sh in monomials(target - dg) {
            let mut row = vec![0u8; ncol];
            for &(m, c) in g {
                let mm = [m[0] + sh[0], m[1] + sh[1], m[2] + sh[2], m[3] + sh[3]];
                row[key8[mono_key(&mm)]] = c as u8;
            }
            // reduce
            for col in 0..ncol {
                let v = row[col] as u32;
                if v == 0 {
                    continue;
                }
                if basis[col].is_some() {
                    let fct = v;
                    let b = basis[col].as_ref().unwrap();
                    for k in col..ncol {
                        let nv = (row[k] as u32 + (p - fct) * b[k] as u32) % p;
                        row[k] = nv as u8;
                    }
                } else {
                    let iv = invs[v as usize];
                    for k in col..ncol {
                        row[k] = (row[k] as u32 * iv % p) as u8;
                    }
                    basis[col] = Some(row);
                    rank += 1;
                    break;
                }
            }
            if rank == ncol {
                return true;
            }
        }
    }
    rank == ncol
}

// ---- lifts of the forms to (Q, F) over F_p ----

type WPoly = Vec<(Mono, (u32, u32))>;

fn wp_norm(mut v: WPoly, p: u32) -> WPoly {
    v.sort();
    let mut out: WPoly = Vec::new();
    for (m, c) in v {
        if let Some(last) = out.last_mut() {
            if last.0 == m {
                last.1 = ((last.1 .0 + c.0) % p, (last.1 .1 + c.1) % p);
                continue;
            }
        }
        out.push((m, (c.0 % p, c.1 % p)));
    }
    out.retain(|x| x.1 != (0, 0));
    out
}

fn wp_mul(a: &WPoly, b: &WPoly, p: u32, nu: u32) -> WPoly {
    let mut v = Vec::new();
    for &(ma, ca) in a {
        for &(mb, cb) in b {
            let m = [ma[0] + mb[0], ma[1] + mb[1], ma[2] + mb[2], ma[3] + mb[3]];
            let re = (ca.0 * cb.0 + nu * (ca.1 * cb.1 % p)) % p;
            let im = (ca.0 * cb.1 + ca.1 * cb.0) % p;
            v.push((m, (re, im)));
        }
    }
    wp_norm(v, p)
}

fn quad_poly(p: u32, ty: Ty, nu: u32) -> SPoly {
    let x = |i: usize| sp_var(i);
    match ty {
        Ty::Split => {
            // x0 x3 - x1 x2
            let a = sp_mul(&x(0), &x(3), p);
            let b = sp_mul(&x(1), &x(2), p);
            sp_sub(&a, &b, p)
        }
        Ty::Nonsplit => {
            // x0 x3 - x1^2 + nu x2^2
            let a = sp_mul(&x(0), &x(3), p);
            let b = sp_mul(&x(1), &x(1), p);
            let c = sp_scale(&sp_mul(&x(2), &x(2), p), nu, p);
            sp_add(&sp_sub(&a, &b, p), &c, p)
        }
        Ty::Cone => {
            // x0 x2 - x1^2
            let a = sp_mul(&x(0), &x(2), p);
            let b = sp_mul(&x(1), &x(1), p);
            sp_sub(&a, &b, p)
        }
    }
}

fn lift_cubic(p: u32, c: &Curve) -> (SPoly, SPoly) {
    let nu = nonresidue(p);
    match c {
        Curve::Split(a) => {
            let mut f: SPoly = Vec::new();
            for i in 0..4usize {
                for j in 0..4usize {
                    if a[i][j] == 0 {
                        continue;
                    }
                    let xs: Vec<usize> = (0..3).map(|k| if k < 3 - i { 0 } else { 1 }).collect();
                    let ys: Vec<usize> = (0..3).map(|k| if k < 3 - j { 0 } else { 1 }).collect();
                    let mut m = sp_const(a[i][j], p);
                    for k in 0..3 {
                        m = sp_mul(&m, &sp_var(2 * xs[k] + ys[k]), p);
                    }
                    f = sp_add(&f, &m, p);
                }
            }
            (quad_poly(p, Ty::Split, nu), f)
        }
        Curve::Nonsplit(a) => {
            let mvar = |ab: usize| -> WPoly {
                match ab {
                    0 => vec![([1, 0, 0, 0], (1, 0))],
                    3 => vec![([0, 0, 0, 1], (1, 0))],
                    1 => vec![([0, 1, 0, 0], (1, 0)), ([0, 0, 1, 0], (0, 1))],
                    _ => vec![([0, 1, 0, 0], (1, 0)), ([0, 0, 1, 0], (0, p - 1))],
                }
            };
            let mut f: WPoly = Vec::new();
            for i in 0..4usize {
                for j in 0..4usize {
                    if a[i][j] == (0, 0) {
                        continue;
                    }
                    let xs: Vec<usize> = (0..3).map(|k| if k < 3 - i { 0 } else { 1 }).collect();
                    let ys: Vec<usize> = (0..3).map(|k| if k < 3 - j { 0 } else { 1 }).collect();
                    let mut m: WPoly = vec![([0; 4], a[i][j])];
                    for k in 0..3 {
                        m = wp_mul(&m, &mvar(2 * xs[k] + ys[k]), p, nu);
                    }
                    f.extend(m);
                }
            }
            let f = wp_norm(f, p);
            assert!(f.iter().all(|x| x.1 .1 == 0), "non-split lift not rational");
            let sp: SPoly = f.iter().map(|&(m, c)| (m, c.0)).collect();
            (quad_poly(p, Ty::Nonsplit, nu), sp)
        }
        Curve::Cone(a2, a4, a6) => {
            let x = |i: usize| sp_var(i);
            let mut f = sp_mul(&sp_mul(&x(3), &x(3), p), &x(3), p);
            let x3sq = sp_mul(&x(3), &x(3), p);
            // a2: s^{2-i} t^i -> x0, x1, x2
            let q2 = [x(0), x(1), x(2)];
            for i in 0..3 {
                f = sp_add(&f, &sp_scale(&sp_mul(&x3sq, &q2[i], p), a2[i], p), p);
            }
            // a4: s^{4-i} t^i -> x0^2, x0x1, x0x2, x1x2, x2^2
            let q4 = [
                sp_mul(&x(0), &x(0), p),
                sp_mul(&x(0), &x(1), p),
                sp_mul(&x(0), &x(2), p),
                sp_mul(&x(1), &x(2), p),
                sp_mul(&x(2), &x(2), p),
            ];
            for i in 0..5 {
                f = sp_add(&f, &sp_scale(&sp_mul(&x(3), &q4[i], p), a4[i], p), p);
            }
            // a6: s^{6-i} t^i -> x0^3, x0^2x1, x0^2x2, x0x1x2, x0x2^2, x1x2^2, x2^3
            let q6 = [
                sp_mul(&sp_mul(&x(0), &x(0), p), &x(0), p),
                sp_mul(&sp_mul(&x(0), &x(0), p), &x(1), p),
                sp_mul(&sp_mul(&x(0), &x(0), p), &x(2), p),
                sp_mul(&sp_mul(&x(0), &x(1), p), &x(2), p),
                sp_mul(&sp_mul(&x(0), &x(2), p), &x(2), p),
                sp_mul(&sp_mul(&x(1), &x(2), p), &x(2), p),
                sp_mul(&sp_mul(&x(2), &x(2), p), &x(2), p),
            ];
            for i in 0..7 {
                f = sp_add(&f, &sp_scale(&q6[i], a6[i], p), p);
            }
            (quad_poly(p, Ty::Cone, nu), f)
        }
        Curve::Hyper(_) => panic!("no P^3 model"),
    }
}

fn is_smooth(p: u32, c: &Curve) -> bool {
    let (q, f) = lift_cubic(p, c);
    smooth_ci(p, &q, &f)
}

// ---------------------------------------------------------------------------
// Hyperelliptic squarefree test (control).
// ---------------------------------------------------------------------------

fn pl_trim(a: &mut Vec<u32>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn pl_rem(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    pl_trim(&mut r);
    let db = b.len() - 1;
    let iv = inv_p(b[db], p);
    while r.len() > db {
        let c = r[r.len() - 1] * iv % p;
        let sh = r.len() - 1 - db;
        for i in 0..=db {
            r[sh + i] = (r[sh + i] + (p - c) * b[i]) % p;
        }
        pl_trim(&mut r);
    }
    r
}

fn pl_gcd(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    pl_trim(&mut x);
    pl_trim(&mut y);
    while !y.is_empty() {
        let r = pl_rem(&x, &y, p);
        x = y;
        y = r;
    }
    x
}

fn hyper_squarefree(p: u32, c: &[u32; 11]) -> bool {
    // binary form of degree 10
    let mut f = c.to_vec();
    pl_trim(&mut f);
    if f.is_empty() {
        return false;
    }
    if f.len() < 10 {
        // root at infinity of multiplicity 10 - deg >= 2
        return false;
    }
    let d: Vec<u32> = (1..f.len()).map(|i| f[i] * i as u32 % p).collect();
    let g = pl_gcd(&f, &d, p);
    g.len() == 1
}

// ---------------------------------------------------------------------------
// Orbits of the automorphism group of the quadric on forms (positive control).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct F2(u32, u32);

struct P2 {
    p: u32,
    nu: u32,
}

impl P2 {
    fn mul(&self, a: F2, b: F2) -> F2 {
        let p = self.p;
        F2(
            (a.0 * b.0 + self.nu * (a.1 * b.1 % p)) % p,
            (a.0 * b.1 + a.1 * b.0) % p,
        )
    }
    fn add(&self, a: F2, b: F2) -> F2 {
        F2((a.0 + b.0) % self.p, (a.1 + b.1) % self.p)
    }
    fn conj(&self, a: F2) -> F2 {
        F2(a.0, (self.p - a.1) % self.p)
    }
    fn pow(&self, a: F2, e: u32) -> F2 {
        let mut r = F2(1, 0);
        for _ in 0..e {
            r = self.mul(r, a);
        }
        r
    }
    fn inv(&self, a: F2) -> F2 {
        // a^{p^2-2}
        let mut r = F2(1, 0);
        let mut b = a;
        let mut e = (self.p as u64) * (self.p as u64) - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn prim(&self) -> F2 {
        let n = self.p * self.p - 1;
        for c0 in 0..self.p {
            for c1 in 0..self.p {
                let g = F2(c0, c1);
                if g == F2(0, 0) {
                    continue;
                }
                let mut x = F2(1, 0);
                let mut ok = true;
                for k in 1..n {
                    x = self.mul(x, g);
                    if x == F2(1, 0) {
                        ok = false;
                        let _ = k;
                        break;
                    }
                }
                if ok {
                    return g;
                }
            }
        }
        panic!("no primitive element")
    }
}

/// Neighbours of a split form under the generators of
/// `(PGL_2(F_p) x PGL_2(F_p)) : 2` and the scalars.
fn split_neighbours(p: u32, a: &[[u32; 4]; 4]) -> Vec<[[u32; 4]; 4]> {
    let g = prim_root_p(p);
    let mut out = Vec::new();
    // x-moves, y-moves
    for axis in 0..2 {
        let get = |a: &[[u32; 4]; 4], i: usize, j: usize| if axis == 0 { a[i][j] } else { a[j][i] };
        let mk = |f: &dyn Fn(usize, usize) -> u32| {
            let mut n = [[0u32; 4]; 4];
            for i in 0..4 {
                for j in 0..4 {
                    let v = f(i, j);
                    if axis == 0 {
                        n[i][j] = v;
                    } else {
                        n[j][i] = v;
                    }
                }
            }
            n
        };
        // shift
        out.push(mk(&|i, j| {
            let mut s = 0u64;
            for i2 in i..4 {
                s += binom(i2 as u64, i as u64) * get(a, i2, j) as u64;
            }
            (s % p as u64) as u32
        }));
        // scale
        out.push(mk(&|i, j| {
            (powmod(g as u64, i as u64, p as u64) * get(a, i, j) as u64 % p as u64) as u32
        }));
        // inversion
        out.push(mk(&|i, j| get(a, 3 - i, j)));
    }
    // swap
    let mut n = [[0u32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            n[i][j] = a[j][i];
        }
    }
    out.push(n);
    // scalar
    let mut n = *a;
    for row in n.iter_mut() {
        for v in row.iter_mut() {
            *v = *v * g % p;
        }
    }
    out.push(n);
    out
}

fn nonsplit_neighbours(p: u32, a: &[[(u32, u32); 4]; 4]) -> Vec<[[(u32, u32); 4]; 4]> {
    let nu = nonresidue(p);
    let f2 = P2 { p, nu };
    let gp = prim_root_p(p);
    let lam = f2.prim();
    let tof = |x: (u32, u32)| F2(x.0, x.1);
    let fromf = |x: F2| (x.0, x.1);
    let mut out = Vec::new();
    let shift_by = |u: F2| {
        // F(X + u, Y + conj u)
        let ub = f2.conj(u);
        let mut tmp = [[F2(0, 0); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut s = F2(0, 0);
                for i2 in i..4 {
                    let c = f2.mul(
                        F2((binom(i2 as u64, i as u64) % p as u64) as u32, 0),
                        f2.pow(u, (i2 - i) as u32),
                    );
                    s = f2.add(s, f2.mul(c, tof(a[i2][j])));
                }
                tmp[i][j] = s;
            }
        }
        let mut res = [[(0u32, 0u32); 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut s = F2(0, 0);
                for j2 in j..4 {
                    let c = f2.mul(
                        F2((binom(j2 as u64, j as u64) % p as u64) as u32, 0),
                        f2.pow(ub, (j2 - j) as u32),
                    );
                    s = f2.add(s, f2.mul(c, tmp[i][j2]));
                }
                res[i][j] = fromf(s);
            }
        }
        res
    };
    out.push(shift_by(F2(1, 0)));
    out.push(shift_by(F2(0, 1)));
    // scale
    let lb = f2.conj(lam);
    let mut n = [[(0u32, 0u32); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            n[i][j] = fromf(f2.mul(
                tof(a[i][j]),
                f2.mul(f2.pow(lam, i as u32), f2.pow(lb, j as u32)),
            ));
        }
    }
    out.push(n);
    // inversion
    let mut n = [[(0u32, 0u32); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            n[i][j] = a[3 - i][3 - j];
        }
    }
    out.push(n);
    // swap
    let mut n = [[(0u32, 0u32); 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            n[i][j] = a[j][i];
        }
    }
    out.push(n);
    // scalar in F_p^*
    let mut n = *a;
    for row in n.iter_mut() {
        for v in row.iter_mut() {
            *v = (v.0 * gp % p, v.1 * gp % p);
        }
    }
    out.push(n);
    out
}

fn bin_mul(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = vec![0u32; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % p;
        }
    }
    r
}

/// Binary form `sum c_i s^{d-i} t^i`, substitute `s -> m00 s + m01 t`,
/// `t -> m10 s + m11 t`.
fn bin_subst(c: &[u32], m: [[u32; 2]; 2], p: u32) -> Vec<u32> {
    let d = c.len() - 1;
    let mut out = vec![0u32; d + 1];
    for (i, &ci) in c.iter().enumerate() {
        if ci == 0 {
            continue;
        }
        // coefficient list indexed by power of t
        let ls = vec![m[0][0], m[0][1]]; // m00 s + m01 t : [s-coeff, t-coeff]
        let lt = vec![m[1][0], m[1][1]];
        let mut poly = vec![1u32];
        for _ in 0..(d - i) {
            poly = bin_mul(&poly, &ls, p);
        }
        for _ in 0..i {
            poly = bin_mul(&poly, &lt, p);
        }
        for (k, &v) in poly.iter().enumerate() {
            out[k] = (out[k] + ci * v) % p;
        }
    }
    out
}

fn cone_neighbours(
    p: u32,
    a: &(Vec<u32>, Vec<u32>, Vec<u32>),
) -> Vec<(Vec<u32>, Vec<u32>, Vec<u32>)> {
    let g = prim_root_p(p);
    let mut out = Vec::new();
    let mats: [[[u32; 2]; 2]; 5] = [
        [[1, 1], [0, 1]],
        [[1, 0], [1, 1]],
        [[g, 0], [0, 1]],
        [[1, 0], [0, g]],
        [[0, 1], [1, 0]],
    ];
    for m in mats {
        out.push((
            bin_subst(&a.0, m, p),
            bin_subst(&a.1, m, p),
            bin_subst(&a.2, m, p),
        ));
    }
    // u -> u + w
    let ws: [Vec<u32>; 3] = [vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
    for w in ws.iter() {
        let w2 = bin_mul(w, w, p);
        let w3 = bin_mul(&w2, w, p);
        let a2w = bin_mul(&a.0, w, p);
        let a2w2 = bin_mul(&a.0, &w2, p);
        let a4w = bin_mul(&a.1, w, p);
        let n2: Vec<u32> = (0..3).map(|i| (a.0[i] + 3 * w[i]) % p).collect();
        let n4: Vec<u32> = (0..5)
            .map(|i| (a.1[i] + 2 * a2w[i] + 3 * w2[i]) % p)
            .collect();
        let n6: Vec<u32> = (0..7)
            .map(|i| (a.2[i] + a4w[i] + a2w2[i] + w3[i]) % p)
            .collect();
        out.push((n2, n4, n6));
    }
    // u -> g u (divide by g^3)
    let gi = inv_p(g, p);
    let n2: Vec<u32> = a.0.iter().map(|&x| x * gi % p).collect();
    let n4: Vec<u32> = a.1.iter().map(|&x| x * gi % p * gi % p).collect();
    let n6: Vec<u32> = a.2.iter().map(|&x| x * gi % p * gi % p * gi % p).collect();
    out.push((n2, n4, n6));
    out
}

fn curve_neighbours(p: u32, c: &Curve) -> Vec<Curve> {
    match c {
        Curve::Split(a) => split_neighbours(p, a)
            .into_iter()
            .map(Curve::Split)
            .collect(),
        Curve::Nonsplit(a) => nonsplit_neighbours(p, a)
            .into_iter()
            .map(Curve::Nonsplit)
            .collect(),
        Curve::Cone(a2, a4, a6) => {
            let t = (a2.to_vec(), a4.to_vec(), a6.to_vec());
            cone_neighbours(p, &t)
                .into_iter()
                .map(|(x, y, z)| {
                    let mut b2 = [0u32; 3];
                    let mut b4 = [0u32; 5];
                    let mut b6 = [0u32; 7];
                    b2.copy_from_slice(&x);
                    b4.copy_from_slice(&y);
                    b6.copy_from_slice(&z);
                    Curve::Cone(b2, b4, b6)
                })
                .collect()
        }
        Curve::Hyper(_) => Vec::new(),
    }
}

/// Orbit of a form under the group (BFS over generators); returns its size.
fn orbit_codes(p: u32, c: &Curve) -> HashSet<u64> {
    let mut seen: HashSet<u64> = HashSet::new();
    let mut stack = vec![c.clone()];
    seen.insert(encode(p, c));
    while let Some(x) = stack.pop() {
        for n in curve_neighbours(p, &x) {
            let code = encode(p, &n);
            if seen.insert(code) {
                stack.push(n);
            }
        }
    }
    seen
}

// ---------------------------------------------------------------------------
// Tian kernels in (Res E)[l], general n (<= 4).
// ---------------------------------------------------------------------------

const DMAX: usize = 8;
type V8 = [u8; DMAX];

fn fl_inv(l: u8, a: u8) -> u8 {
    (1..l)
        .find(|&b| (a as u16 * b as u16) % l as u16 == 1)
        .expect("inverse")
}
fn fl_mul(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 * b as u16) % l as u16) as u8
}
fn fl_add(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 + b as u16) % l as u16) as u8
}
fn fl_sub(l: u8, a: u8, b: u8) -> u8 {
    ((a as u16 + l as u16 - b as u16) % l as u16) as u8
}

fn apply_pi(l: u8, n: usize, phi: [[u8; 2]; 2], v: &V8) -> V8 {
    let mut out = [0u8; DMAX];
    let (x, y) = (v[2 * (n - 1)], v[2 * (n - 1) + 1]);
    out[0] = fl_add(l, fl_mul(l, phi[0][0], x), fl_mul(l, phi[0][1], y));
    out[1] = fl_add(l, fl_mul(l, phi[1][0], x), fl_mul(l, phi[1][1], y));
    for i in 1..n {
        out[2 * i] = v[2 * (i - 1)];
        out[2 * i + 1] = v[2 * (i - 1) + 1];
    }
    out
}

/// RREF rows over the first `ncols`; returns the rank (zero rows dropped).
fn rref(l: u8, ncols: usize, rows: &mut Vec<V8>) -> usize {
    let mut r = 0usize;
    for c in 0..ncols {
        let Some(piv) = (r..rows.len()).find(|&i| rows[i][c] % l != 0) else {
            continue;
        };
        rows.swap(r, piv);
        let inv = fl_inv(l, rows[r][c]);
        for x in rows[r].iter_mut() {
            *x = fl_mul(l, *x, inv);
        }
        for i in 0..rows.len() {
            if i != r && rows[i][c] != 0 {
                let f = rows[i][c];
                let prow = rows[r];
                for k in 0..ncols {
                    rows[i][k] = fl_sub(l, rows[i][k], fl_mul(l, f, prow[k]));
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    rows.retain(|row| row[..ncols].iter().any(|&x| x != 0));
    rows.len()
}

fn omega(l: u8, n: usize, qbar: u8, v: &V8, w: &V8) -> u8 {
    let mut s = 0u8;
    let mut qi = 1u8;
    for i in 0..n {
        let a = fl_sub(
            l,
            fl_mul(l, v[2 * i], w[2 * i + 1]),
            fl_mul(l, v[2 * i + 1], w[2 * i]),
        );
        s = fl_add(l, s, fl_mul(l, qi, a));
        qi = fl_mul(l, qi, qbar);
    }
    s
}

fn is_isotropic(l: u8, n: usize, qbar: u8, rows: &[V8]) -> bool {
    for i in 0..rows.len() {
        for j in i + 1..rows.len() {
            if omega(l, n, qbar, &rows[i], &rows[j]) != 0 {
                return false;
            }
        }
    }
    true
}

/// `(n_kernels, n_tian)`: pi-stable isotropic subspaces of dimension `n` of
/// `W[l]`, and those meeting `V_0` trivially (hence every `V_i`).
fn kernel_census(l: u8, n: usize, qbar: u8, phi: [[u8; 2]; 2]) -> (usize, usize) {
    let ncols = 2 * n;
    // cyclic submodules of dimension <= n, vectors normalised (first nonzero = 1)
    let mut seen: HashSet<Vec<u8>> = HashSet::new();
    let mut cyc: Vec<Vec<V8>> = Vec::new();
    let mut v = [0u8; DMAX];
    loop {
        let mut i = 0;
        loop {
            if i == ncols {
                break;
            }
            v[i] += 1;
            if v[i] == l {
                v[i] = 0;
                i += 1;
            } else {
                break;
            }
        }
        if i == ncols {
            break;
        }
        // normalised: highest nonzero coordinate is 1 (v is scaled so that the
        // last nonzero entry equals 1)
        let last = (0..ncols).rev().find(|&k| v[k] != 0).unwrap();
        if v[last] != 1 {
            continue;
        }
        // Krylov
        let mut rows: Vec<V8> = Vec::new();
        let mut w = v;
        let mut dim = 0usize;
        let mut ok = true;
        for _ in 0..ncols {
            let mut trial = rows.clone();
            trial.push(w);
            let r = rref(l, ncols, &mut trial);
            if r == dim {
                break;
            }
            dim = r;
            rows = trial;
            if dim > n {
                ok = false;
                break;
            }
            w = apply_pi(l, n, phi, &w);
        }
        if !ok {
            continue;
        }
        let mut key = Vec::with_capacity(1 + dim * ncols);
        key.push(dim as u8);
        for r in &rows {
            key.extend_from_slice(&r[..ncols]);
        }
        if seen.insert(key) {
            if is_isotropic(l, n, qbar, &rows) {
                cyc.push(rows);
            }
        }
    }
    let mut found: HashSet<Vec<u8>> = HashSet::new();
    let mut n_ker = 0usize;
    let mut n_tian = 0usize;
    let mut record = |rows: &Vec<V8>, found: &mut HashSet<Vec<u8>>| {
        let mut key = Vec::new();
        for r in rows {
            key.extend_from_slice(&r[..ncols]);
        }
        if found.insert(key) {
            n_ker += 1;
            // meets V_0 trivially: projection to coordinates 2.. has rank n
            let mut proj: Vec<V8> = rows
                .iter()
                .map(|r| {
                    let mut z = [0u8; DMAX];
                    z[..ncols - 2].copy_from_slice(&r[2..ncols]);
                    z
                })
                .collect();
            let rk = rref(l, ncols - 2, &mut proj);
            if rk == n {
                n_tian += 1;
            }
        }
    };
    for (i, a) in cyc.iter().enumerate() {
        if a.len() == n {
            record(a, &mut found);
        }
        for b in cyc.iter().skip(i) {
            if a.len() + b.len() < n || a.len() == n || b.len() == n {
                continue;
            }
            let mut rows = a.clone();
            rows.extend(b.iter().cloned());
            let r = rref(l, ncols, &mut rows);
            if r == n && is_isotropic(l, n, qbar, &rows) {
                record(&rows, &mut found);
            }
        }
    }
    (n_ker, n_tian)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
enum Cls {
    Split,
    Nonsplit,
    Scalar,
    Jordan,
}

impl Cls {
    fn name(self) -> &'static str {
        match self {
            Cls::Split => "split",
            Cls::Nonsplit => "nonsplit",
            Cls::Scalar => "scalar",
            Cls::Jordan => "jordan",
        }
    }
}

/// Representative matrices of the Frobenius class types for `(tr, det)` mod `l`.
fn class_reps(l: u8, tr: u8, det: u8) -> Vec<(Cls, [[u8; 2]; 2])> {
    let roots: Vec<u8> = (0..l)
        .filter(|&x| {
            (x as u32 * x as u32 + det as u32 + l as u32 * l as u32 - tr as u32 * x as u32)
                % l as u32
                == 0
        })
        .collect();
    match roots.len() {
        2 => vec![(Cls::Split, [[roots[0], 0], [0, roots[1]]])],
        1 => vec![
            (Cls::Scalar, [[roots[0], 0], [0, roots[0]]]),
            (Cls::Jordan, [[roots[0], 1], [0, roots[0]]]),
        ],
        _ => vec![(Cls::Nonsplit, [[0, fl_sub(l, 0, det)], [1, tr]])],
    }
}

// ---------------------------------------------------------------------------
// Census driver.
// ---------------------------------------------------------------------------

#[derive(Default, Clone)]
struct Stats {
    samples: u64,
    res_shaped: u64,
    cand_allowed: u64,
    smooth_checked: u64,
    smooth_yes: u64,
    sm_sub_checked: u64,
    sm_sub_yes: u64,
    secs: f64,
}

struct Hit {
    ty: Ty,
    code: u64,
    t: i64,
    smooth: bool,
}

fn run_census(
    eng: &Engine,
    ty: Ty,
    exhaustive: bool,
    nsamples: u64,
    seed: u64,
    prime_only: bool,
    sm_cap: u64,
    log: &mut dyn Write,
    cand_hist: &mut BTreeMap<i64, u64>,
) -> (Stats, Vec<Hit>) {
    let p = eng.p;
    let q = p as u64;
    let allowed = allowed_traces(p);
    let total = ty.ncodes(p);
    let mut st = Stats::default();
    let mut hits = Vec::new();
    let mut rng = seed ^ (ty as u64 + 1).wrapping_mul(0xA24B_AED4_963E_E407);
    let t0 = Instant::now();
    let n = if exhaustive { total } else { nsamples };
    let mut last_log = Instant::now();
    for i in 0..n {
        let code = if exhaustive {
            i
        } else {
            splitmix(&mut rng) % total
        };
        let curve = decode(ty, p, code);
        st.samples += 1;
        if let Some(t) = eng.sieve(&curve, if prime_only { Some(&allowed) } else { None }) {
            st.res_shaped += 1;
            *cand_hist.entry(t).or_insert(0) += 1;
            let ix = t + 2 * (q * q) as i64;
            let ok = ix >= 0 && (ix as usize) < allowed.len() && allowed[ix as usize];
            if ok {
                st.cand_allowed += 1;
                let sm = is_smooth(p, &curve);
                st.smooth_checked += 1;
                if sm {
                    st.smooth_yes += 1;
                }
                hits.push(Hit {
                    ty,
                    code,
                    t,
                    smooth: sm,
                });
            } else if st.sm_sub_checked < sm_cap {
                let sm = is_smooth(p, &curve);
                st.sm_sub_checked += 1;
                if sm {
                    st.sm_sub_yes += 1;
                }
            }
        }
        if last_log.elapsed().as_secs() >= 120 {
            let _ = writeln!(
                log,
                "{} {}: {}/{} samples, res-shaped {}, allowed-trace cand {}, smooth hits {}, {:.0}s",
                ty.name(),
                p,
                i + 1,
                n,
                st.res_shaped,
                st.cand_allowed,
                st.smooth_yes,
                t0.elapsed().as_secs_f64()
            );
            last_log = Instant::now();
        }
    }
    st.secs = t0.elapsed().as_secs_f64();
    (st, hits)
}

// ---------------------------------------------------------------------------
// Plane quintic models.
// ---------------------------------------------------------------------------

fn lin_subst(p: u32, poly: &SPoly, a: &[[u32; 4]; 4]) -> SPoly {
    // x_i = sum_j a[i][j] y_j
    let lin: Vec<SPoly> = (0..4)
        .map(|i| {
            let mut v = Vec::new();
            for j in 0..4 {
                if a[i][j] != 0 {
                    let mut m = [0u8; 4];
                    m[j] = 1;
                    v.push((m, a[i][j]));
                }
            }
            sp_norm(v, p)
        })
        .collect();
    let mut out: SPoly = Vec::new();
    for &(m, c) in poly {
        let mut t = sp_const(c, p);
        for i in 0..4 {
            for _ in 0..m[i] {
                t = sp_mul(&t, &lin[i], p);
            }
        }
        out = sp_add(&out, &t, p);
    }
    out
}

#[derive(Default, Clone, Debug)]
struct PlaneRes {
    checked: u64,
    mismatches: u64,
    degenerate: u64,
    sing_mismatch: u64,
    node_rational: u64,
    node_split_pairs: u64,
    nodes_by_k: [u64; 4],
}

fn poly_part_by_y3(poly: &SPoly, e3: u8) -> SPoly {
    poly.iter()
        .filter(|x| x.0[3] == e3)
        .map(|&(m, c)| ([m[0], m[1], m[2], 0], c))
        .collect()
}

/// Points of `P^2(F)` as triples of logs.
fn p2_points(f: &Fld) -> Vec<[u32; 3]> {
    let mut v = Vec::new();
    let z = f.zero();
    let all: Vec<u32> = (0..=f.n1).collect();
    for &a in &all {
        for &b in &all {
            v.push([0, a, b]); // (1 : a : b)
        }
    }
    for &b in &all {
        v.push([z, 0, b]); // (0 : 1 : b)
    }
    v.push([z, z, 0]); // (0 : 0 : 1)
    v
}

/// Check the plane-quintic model obtained by projecting from the rational
/// point `pt` of `C = V(Q, F)`; returns the node data and compares the point
/// counts of the model (with the node/tangent corrections) with `N_k(C)`.
fn plane_check(eng: &Engine, curve: &Curve, pt: [u32; 4], ks: &[u32], res: &mut PlaneRes) {
    let p = eng.p;
    let (q, f) = lift_cubic(p, curve);
    // basis change: last column = pt
    let m = (0..4).find(|&i| pt[i] != 0).unwrap();
    let mut a = [[0u32; 4]; 4];
    let mut col = 0;
    for i in 0..4 {
        if i != m {
            a[i][col] = 1;
            col += 1;
        }
    }
    for i in 0..4 {
        a[i][3] = pt[i];
    }
    let q2p = lin_subst(p, &q, &a);
    let f2p = lin_subst(p, &f, &a);
    assert!(q2p.iter().all(|x| x.0[3] <= 1), "Q has a y3^2 term");
    assert!(f2p.iter().all(|x| x.0[3] <= 2), "F has a y3^3 term");
    let ell = poly_part_by_y3(&q2p, 1);
    let qq2 = poly_part_by_y3(&q2p, 0);
    let c1 = poly_part_by_y3(&f2p, 2);
    let c2 = poly_part_by_y3(&f2p, 1);
    let c3 = poly_part_by_y3(&f2p, 0);
    let g = sp_add(
        &sp_sub(
            &sp_mul(&c1, &sp_mul(&qq2, &qq2, p), p),
            &sp_mul(&c2, &sp_mul(&qq2, &ell, p), p),
            p,
        ),
        &sp_mul(&c3, &sp_mul(&ell, &ell, p), p),
        p,
    );
    let gd: Vec<SPoly> = (0..3).map(|v| sp_deriv(&g, v, p)).collect();
    for &k in ks {
        let f_k = eng.fld(k);
        let z = f_k.zero();
        let nc = eng.count(curve, k);
        let mut n_model = 0u64;
        let mut nodes: Vec<[u32; 3]> = Vec::new();
        let mut sing: Vec<[u32; 3]> = Vec::new();
        for pt3 in p2_points(f_k) {
            let x = [pt3[0], pt3[1], pt3[2], z];
            let le = sp_eval(f_k, &ell, &x);
            let qv = sp_eval(f_k, &qq2, &x);
            let gv = sp_eval(f_k, &g, &x);
            if le != z {
                if gv == z {
                    n_model += 1;
                }
            } else if qv != z {
                if sp_eval(f_k, &c1, &x) == z {
                    n_model += 1;
                }
            } else {
                nodes.push(pt3);
                let cc1 = sp_eval(f_k, &c1, &x);
                let cc2 = sp_eval(f_k, &c2, &x);
                let cc3 = sp_eval(f_k, &c3, &x);
                if cc1 == z && cc2 == z && cc3 == z {
                    res.degenerate += 1;
                }
                // finite roots of c1 u^2 + c2 u + c3, plus the point P (u = infinity)
                // when c1 = 0
                let r = if cc1 != z {
                    let disc = f_k.sub(
                        f_k.mul(cc2, cc2),
                        f_k.mul(f_k.mul(f_k.from_int(4), cc1), cc3),
                    );
                    if disc == z {
                        1
                    } else if f_k.is_square(disc) {
                        2
                    } else {
                        0
                    }
                } else if cc2 != z {
                    1
                } else {
                    0
                };
                n_model += r + if cc1 == z { 1 } else { 0 };
            }
            if gv == z && gd.iter().all(|d| sp_eval(f_k, d, &x) == z) {
                sing.push(pt3);
            }
        }
        res.checked += 1;
        if n_model != nc {
            res.mismatches += 1;
        }
        if sing != nodes {
            res.sing_mismatch += 1;
        }
        if k <= 3 {
            res.nodes_by_k[k as usize] += nodes.len() as u64;
        }
    }
}

fn p3_rational_points_on(eng: &Engine, curve: &Curve) -> Vec<[u32; 4]> {
    let p = eng.p;
    let (q, f) = lift_cubic(p, curve);
    let f1 = eng.fld(1);
    let mut out = Vec::new();
    for code in 0..p.pow(4) {
        let mut d = [0u32; 4];
        let mut c = code;
        for x in d.iter_mut() {
            *x = c % p;
            c /= p;
        }
        // normalised: first nonzero = 1
        let first = d.iter().position(|&x| x != 0);
        let Some(fi) = first else { continue };
        if d[fi] != 1 {
            continue;
        }
        let lg: [u32; 4] = [
            f1.from_int(d[0] as i64),
            f1.from_int(d[1] as i64),
            f1.from_int(d[2] as i64),
            f1.from_int(d[3] as i64),
        ];
        if sp_eval(f1, &q, &lg) == f1.zero() && sp_eval(f1, &f, &lg) == f1.zero() {
            out.push(d);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Output helpers.
// ---------------------------------------------------------------------------

fn write_hits(path: &Path, hits: &[Hit], p: u32) {
    let q = p as u64;
    let mut f = File::create(path).expect("hits.csv");
    writeln!(f, "type,code,t,nE,smooth").unwrap();
    for h in hits {
        writeln!(
            f,
            "{},{},{},{},{}",
            h.ty.name(),
            h.code,
            h.t,
            ell_order(q, h.t),
            h.smooth
        )
        .unwrap();
    }
}

fn read_hits(path: &str) -> Vec<(Ty, u64, i64, bool)> {
    let s = path
        .split(',')
        .map(|pp| std::fs::read_to_string(pp).expect("read hits"))
        .collect::<Vec<_>>()
        .join("\n");
    let mut v = Vec::new();
    for line in s.lines().filter(|l| !l.starts_with("type,")) {
        let f: Vec<&str> = line.split(',').collect();
        if f.len() < 5 {
            continue;
        }
        v.push((
            Ty::from_name(f[0]).unwrap(),
            f[1].parse().unwrap(),
            f[2].parse().unwrap(),
            f[4] == "true",
        ));
    }
    v
}

fn gi_arg(args: &[String], i: usize, default: u64) -> u64 {
    args.get(i).and_then(|s| s.parse().ok()).unwrap_or(default)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "usage: genus4_n4_census <out-dir> <PART> ...\n\
             HYPER <p> <exh|N> <seed>\n\
             CENSUS <p> <split|nonsplit|cone|all> <exh|N> <seed> <prime_only 0|1> <sm_cap>\n\
             CLASSES <p> <hits.csv>\n\
             TIAN <p> <hits.csv[,hits.csv...]>\n\
             PLANE <p> <hits.csv> <max_per_t> <kmax>\n\
             TIANTHEORY <l,l,...>\n\
             JOIN <p> <mass_by_t.csv> <tian_by_t.csv>\n\
             BENCH <p> <N>"
        );
        std::process::exit(2);
    }
    let out = Path::new(&args[1]);
    std::fs::create_dir_all(out).unwrap();
    let part = args[2].as_str();
    let mut log = std::io::stderr();
    match part {
        "HYPER" => {
            let p = gi_arg(&args, 3, 3) as u32;
            let exh = args.get(4).map(|s| s == "exh").unwrap_or(true);
            let n = gi_arg(&args, 4, 0);
            let seed = gi_arg(&args, 5, 1);
            part_hyper(out, p, exh, n, seed, &mut log);
        }
        "CENSUS" => {
            let p = gi_arg(&args, 3, 3) as u32;
            let tys: Vec<Ty> = match args[4].as_str() {
                "all" => vec![Ty::Split, Ty::Nonsplit, Ty::Cone],
                s => vec![Ty::from_name(s).expect("type")],
            };
            let exh = args[5] == "exh";
            let n = gi_arg(&args, 5, 0);
            let seed = gi_arg(&args, 6, 1);
            let prime_only = gi_arg(&args, 7, 0) == 1;
            let sm_cap = gi_arg(&args, 8, 2000);
            part_census(out, p, &tys, exh, n, seed, prime_only, sm_cap, &mut log);
        }
        "CLASSES" => {
            let p = gi_arg(&args, 3, 3) as u32;
            part_classes(out, p, &args[4]);
        }
        "TIAN" => {
            let p = gi_arg(&args, 3, 3) as u32;
            part_tian(out, p, &args[4]);
        }
        "PLANE" => {
            let p = gi_arg(&args, 3, 3) as u32;
            let maxt = gi_arg(&args, 5, 3) as usize;
            let kmax = gi_arg(&args, 6, 3) as u32;
            part_plane(out, p, &args[4], maxt, kmax);
        }
        "TIANTHEORY" => {
            let ells: Vec<u8> = args[3].split(',').map(|x| x.parse().unwrap()).collect();
            part_tian_theory(out, &ells);
        }
        "JOIN" => {
            let p = gi_arg(&args, 3, 3) as u32;
            part_join(out, p, &args[4], &args[5]);
        }
        "BENCH" => {
            let p = gi_arg(&args, 3, 5) as u32;
            let n = gi_arg(&args, 4, 100000);
            part_bench(p, n);
        }
        _ => {
            eprintln!("unknown part {part}");
            std::process::exit(2);
        }
    }
}

fn part_bench(p: u32, n: u64) {
    let t0 = Instant::now();
    let eng = Engine::new(p);
    eprintln!("engine build {:.2}s", t0.elapsed().as_secs_f64());
    for ty in [Ty::Split, Ty::Nonsplit, Ty::Cone] {
        for po in [false, true] {
            let mut hist = BTreeMap::new();
            let mut sink = std::io::sink();
            let (st, hits) = run_census(&eng, ty, false, n, 7, po, 50, &mut sink, &mut hist);
            eprintln!(
                "{} prime_only={} {} samples {:.2}s ({:.2} us/sample), res-shaped {}, cand {}, smooth {}, sm_sub {}/{}",
                ty.name(),
                po,
                st.samples,
                st.secs,
                st.secs * 1e6 / st.samples as f64,
                st.res_shaped,
                st.cand_allowed,
                hits.iter().filter(|h| h.smooth).count(),
                st.sm_sub_yes,
                st.sm_sub_checked
            );
        }
    }
}

fn part_hyper(out: &Path, p: u32, exh: bool, n: u64, seed: u64, log: &mut dyn Write) {
    let eng = Engine::new(p);
    let q = p as u64;
    let total = (p as u64).pow(11);
    let mut hist: BTreeMap<i64, u64> = BTreeMap::new();
    let mut sqfree = 0u64;
    let mut res = 0u64;
    let mut rng = seed;
    let t0 = Instant::now();
    let m = if exh { total } else { n };
    let mut odd_hits = 0u64;
    for i in 0..m {
        let mut code = if exh { i } else { splitmix(&mut rng) % total };
        let mut c = [0u32; 11];
        for x in c.iter_mut() {
            *x = (code % p as u64) as u32;
            code /= p as u64;
        }
        if !hyper_squarefree(p, &c) {
            continue;
        }
        sqfree += 1;
        if let Some(t) = eng.sieve(&Curve::Hyper(c), None) {
            res += 1;
            *hist.entry(t).or_insert(0) += 1;
            if t % 2 != 0 {
                odd_hits += 1;
            }
        }
    }
    let mut f = File::create(out.join("hyper_t_hist.csv")).unwrap();
    writeln!(f, "t,count").unwrap();
    for (t, c) in &hist {
        writeln!(f, "{t},{c}").unwrap();
    }
    let mut s = String::new();
    s.push_str(&format!(
        "HYPER q={q} mode={} forms_examined={m}\n",
        if exh { "exhaustive" } else { "sampled" }
    ));
    s.push_str(&format!(
        "squarefree_degree10_forms={sqfree}\nres_shaped={res}\nodd_t_hits={odd_hits}\n"
    ));
    s.push_str(&format!(
        "t_hist={hist:?}\nseconds={:.1}\n",
        t0.elapsed().as_secs_f64()
    ));
    std::fs::write(out.join("summary.txt"), &s).unwrap();
    let _ = write!(log, "{s}");
}

fn part_census(
    out: &Path,
    p: u32,
    tys: &[Ty],
    exh: bool,
    n: u64,
    seed: u64,
    prime_only: bool,
    sm_cap: u64,
    log: &mut dyn Write,
) {
    let eng = Engine::new(p);
    let q = p as u64;
    let mut all_hits: Vec<Hit> = Vec::new();
    let mut summary = String::new();
    let mut hist_rows = String::from("type,t,candidates\n");
    let mut mass_rows = String::from("type,t,nE,smooth_hits,mass_estimate,mass_stderr\n");
    for &ty in tys {
        let mut hist = BTreeMap::new();
        let (st, hits) = run_census(&eng, ty, exh, n, seed, prime_only, sm_cap, log, &mut hist);
        let denom = ty.scalar_factor(q) as f64 * ty.group_order(q) as f64;
        let tot_forms = ty.total_forms(q) as f64;
        let scale = st.samples as f64;
        let mut per_t: BTreeMap<i64, u64> = BTreeMap::new();
        for h in &hits {
            if h.smooth {
                *per_t.entry(h.t).or_insert(0) += 1;
            }
        }
        summary.push_str(&format!(
            "{} q={q}: samples={} ({}) res_shaped={} allowed_trace_candidates={} smooth_prime_order_hits={} \
             smooth_fraction_among_prime_cands={:.4} smooth_fraction_other_res_shaped(sub-sample {})={}/{} secs={:.1}\n",
            ty.name(),
            st.samples,
            if exh { "exhaustive" } else { "random" },
            st.res_shaped,
            st.cand_allowed,
            st.smooth_yes,
            st.smooth_yes as f64 / st.cand_allowed.max(1) as f64,
            sm_cap,
            st.sm_sub_yes,
            st.sm_sub_checked,
            st.secs
        ));
        let tot_hits: u64 = per_t.values().sum();
        let mass = tot_hits as f64 / scale * tot_forms / denom;
        let se = (tot_hits as f64).sqrt() / scale * tot_forms / denom;
        summary.push_str(&format!(
            "{} q={q}: total prime-order mass estimate {:.3} +- {:.3} (hit rate per sampled form {:.3e})\n",
            ty.name(),
            mass,
            se,
            tot_hits as f64 / scale
        ));
        for (t, c) in &hist {
            hist_rows.push_str(&format!("{},{},{}\n", ty.name(), t, c));
        }
        for (t, c) in &per_t {
            let m = *c as f64 / scale * tot_forms / denom;
            let e = (*c as f64).sqrt() / scale * tot_forms / denom;
            mass_rows.push_str(&format!(
                "{},{},{},{},{:.4},{:.4}\n",
                ty.name(),
                t,
                ell_order(q, *t),
                c,
                m,
                e
            ));
        }
        all_hits.extend(hits);
    }
    write_hits(&out.join("hits.csv"), &all_hits, p);
    std::fs::write(out.join("cand_t_hist.csv"), hist_rows).unwrap();
    std::fs::write(out.join("mass_by_t.csv"), mass_rows).unwrap();
    std::fs::write(out.join("summary.txt"), &summary).unwrap();
    let _ = write!(log, "{summary}");
}

fn part_classes(out: &Path, p: u32, hits_path: &str) {
    let q = p as u64;
    let hits = read_hits(hits_path);
    let mut done: HashSet<u64> = HashSet::new();
    let mut rows = String::from("type,t,nE,orbit_size,aut,rep_code\n");
    let mut summary = String::new();
    let mut by: BTreeMap<(Ty, i64), (u64, f64)> = BTreeMap::new();
    for (ty, code, t, smooth) in hits {
        if !smooth || done.contains(&code) {
            continue;
        }
        let c = decode(ty, p, code);
        let orb = orbit_codes(p, &c);
        let osz = orb.len() as u64;
        for x in &orb {
            done.insert(*x);
        }
        let g = ty.group_order(q) * ty.scalar_factor(q);
        let aut = g / osz;
        assert_eq!(g % osz, 0, "orbit size does not divide |G|");
        let rep = *orb.iter().min().unwrap();
        rows.push_str(&format!(
            "{},{},{},{},{},{}\n",
            ty.name(),
            t,
            ell_order(q, t),
            osz,
            aut,
            rep
        ));
        let e = by.entry((ty, t)).or_insert((0, 0.0));
        e.0 += 1;
        e.1 += 1.0 / aut as f64;
    }
    summary.push_str("type,t,nE,classes,mass\n");
    let mut nclasses = 0;
    for ((ty, t), (c, m)) in &by {
        summary.push_str(&format!(
            "{},{},{},{},{:.4}\n",
            ty.name(),
            t,
            ell_order(q, *t),
            c,
            m
        ));
        nclasses += c;
    }
    summary.push_str(&format!("total classes {nclasses}\n"));
    std::fs::write(out.join("classes.csv"), rows).unwrap();
    std::fs::write(out.join("summary.txt"), &summary).unwrap();
    eprint!("{summary}");
}

fn isqrt(n: i64) -> i64 {
    let mut r = (n as f64).sqrt() as i64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn part_tian(out: &Path, p: u32, hits_paths: &str) {
    let q = p as u64;
    let mut seen_t: BTreeMap<i64, u64> = BTreeMap::new();
    for path in hits_paths.split(',') {
        for (_, _, t, smooth) in read_hits(path) {
            if smooth {
                *seen_t.entry(t).or_insert(0) += 1;
            }
        }
    }
    let allowed = allowed_traces(p);
    let mut cache: HashMap<(u8, u8, u8, Cls), (usize, usize)> = HashMap::new();
    let mut rows = String::from("q,t,nE,hits_forms,ordinary,disc,ell,qbar,tr,det,types,n_ker_by_type,n_tian_by_type,tian_exists\n");
    let ells: Vec<u8> = [2u8, 3, 5, 7]
        .into_iter()
        .filter(|&l| l as u32 != p)
        .collect();
    let t0 = Instant::now();
    for (ix, &ok) in allowed.iter().enumerate() {
        if !ok {
            continue;
        }
        let t = ix as i64 - 2 * (q * q) as i64;
        let disc = t * t - 4 * (q as i64).pow(4);
        let ordinary = t % p as i64 != 0;
        let _ = isqrt(0);
        for &l in &ells {
            let lq = l as i64;
            let det = ((q as i64).pow(4)).rem_euclid(lq) as u8;
            let tr = t.rem_euclid(lq) as u8;
            let qbar = (q as i64).rem_euclid(lq) as u8;
            let reps = class_reps(l, tr, det);
            let mut types = Vec::new();
            let mut kers = Vec::new();
            let mut tians = Vec::new();
            for (cls, phi) in reps {
                // scalar needs l^2 | disc (ordinary); supersingular: both possible
                if cls == Cls::Scalar && ordinary && disc % (lq * lq) != 0 {
                    continue;
                }
                let key = (l, qbar, tr, cls);
                let (nk, nt) = *cache
                    .entry(key)
                    .or_insert_with(|| kernel_census(l, 4, qbar, phi));
                types.push(cls.name());
                kers.push(nk.to_string());
                tians.push(nt.to_string());
            }
            let exists = if tians.iter().all(|x| x != "0") {
                "yes"
            } else if tians.iter().all(|x| x == "0") {
                "no"
            } else {
                "depends"
            };
            rows.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                q,
                t,
                ell_order(q, t),
                seen_t.get(&t).copied().unwrap_or(0),
                ordinary,
                disc,
                l,
                qbar,
                tr,
                det,
                types.join("|"),
                kers.join("|"),
                tians.join("|"),
                exists
            ));
        }
        eprintln!("t={t} done ({:.0}s)", t0.elapsed().as_secs_f64());
    }
    std::fs::write(out.join("tian_by_t.csv"), rows).unwrap();
}

fn part_plane(out: &Path, p: u32, hits_path: &str, max_per_t: usize, kmax: u32) {
    let eng = Engine::new(p);
    let hits = read_hits(hits_path);
    let mut per_t: BTreeMap<(Ty, i64), usize> = BTreeMap::new();
    let mut rows = String::from("type,t,code,npoints_rational,models_checked,mismatches,degenerate,sing_mismatch,nodes_k1,nodes_k2,nodes_k3\n");
    let mut tot = PlaneRes::default();
    let mut tot_models = 0u64;
    let ks: Vec<u32> = (1..=kmax).collect();
    for (ty, code, t, smooth) in hits {
        if !smooth {
            continue;
        }
        let e = per_t.entry((ty, t)).or_insert(0);
        if *e >= max_per_t {
            continue;
        }
        *e += 1;
        let c = decode(ty, p, code);
        let pts = p3_rational_points_on(&eng, &c);
        let mut r = PlaneRes::default();
        for pt in &pts {
            plane_check(&eng, &c, *pt, &ks, &mut r);
        }
        rows.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}\n",
            ty.name(),
            t,
            code,
            pts.len(),
            pts.len(),
            r.mismatches,
            r.degenerate,
            r.sing_mismatch,
            r.nodes_by_k[1],
            r.nodes_by_k[2],
            r.nodes_by_k[3]
        ));
        tot_models += pts.len() as u64;
        tot.checked += r.checked;
        tot.mismatches += r.mismatches;
        tot.degenerate += r.degenerate;
        tot.sing_mismatch += r.sing_mismatch;
    }
    let s = format!(
        "plane quintic models checked: curves={} models={} (k,model) checks={} count_mismatches={} degenerate_nodes={} singular_set_mismatches={}\n",
        per_t.values().sum::<usize>(),
        tot_models,
        tot.checked,
        tot.mismatches,
        tot.degenerate,
        tot.sing_mismatch
    );
    std::fs::write(out.join("plane_models.csv"), rows).unwrap();
    std::fs::write(out.join("summary.txt"), &s).unwrap();
    eprint!("{s}");
}

// ---------------------------------------------------------------------------
// Predictors and the join of the census with the Tian table.
// ---------------------------------------------------------------------------

/// Hurwitz class number `H(n)`: reduced positive definite forms of
/// discriminant `-n`, weight 1/2 for `a(x^2+y^2)` and 1/3 for `a(x^2+xy+y^2)`.
fn hurwitz(n: i64) -> f64 {
    if n <= 0 || (n % 4 != 0 && n % 4 != 3) {
        return 0.0;
    }
    let mut h = 0.0;
    let mut a = 1i64;
    while 3 * a * a <= n {
        for b in -(a - 1)..=a {
            let num = b * b + n;
            if num % (4 * a) != 0 {
                continue;
            }
            let c = num / (4 * a);
            if c < a || ((b.abs() == a || a == c) && b < 0) {
                continue;
            }
            h += if a == c && b == 0 {
                0.5
            } else if a == b && b == c {
                1.0 / 3.0
            } else {
                1.0
            };
        }
        a += 1;
    }
    h
}

/// Katz-Sarnak heuristic [L]: density, in the space of Weil-polynomial
/// coefficients, of the Haar measure of `USp(8)` at the polynomial
/// `T^8 - t T^4 + q^4`, whose normalised eigenangles are
/// `phi_k = (theta + 2 pi k)/4`, `cos(theta) = t/(2q^2)`:
/// `prod_{j<k} |cos phi_j - cos phi_k| prod_j sin phi_j`.
fn haar_density(t: i64, q: u64) -> f64 {
    let c = (t as f64 / (2.0 * (q * q) as f64)).clamp(-1.0, 1.0);
    let th = c.acos();
    let ph: Vec<f64> = (0..4)
        .map(|k| (th + 2.0 * std::f64::consts::PI * k as f64) / 4.0)
        .collect();
    let mut w = 1.0;
    for j in 0..4 {
        w *= ph[j].sin().abs();
        for k in (j + 1)..4 {
            w *= (ph[j].cos() - ph[k].cos()).abs();
        }
    }
    w
}

fn ranks(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].partial_cmp(&v[b]).unwrap());
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0 + 1.0;
        for k in i..=j {
            r[idx[k]] = avg;
        }
        i = j + 1;
    }
    r
}

fn spearman(x: &[f64], y: &[f64]) -> f64 {
    let (rx, ry) = (ranks(x), ranks(y));
    let n = x.len() as f64;
    let mx = rx.iter().sum::<f64>() / n;
    let my = ry.iter().sum::<f64>() / n;
    let (mut sxy, mut sxx, mut syy) = (0.0, 0.0, 0.0);
    for i in 0..x.len() {
        sxy += (rx[i] - mx) * (ry[i] - my);
        sxx += (rx[i] - mx).powi(2);
        syy += (ry[i] - my).powi(2);
    }
    if sxx == 0.0 || syy == 0.0 {
        return f64::NAN;
    }
    sxy / (sxx * syy).sqrt()
}

/// Join `mass_by_t.csv` (census) with `tian_by_t.csv` and the predictors.
fn part_join(out: &Path, p: u32, mass_path: &str, tian_path: &str) {
    let q = p as u64;
    // mass per t summed over quadric types
    let mut mass: BTreeMap<i64, (f64, f64, u64)> = BTreeMap::new();
    let mass_text: String = mass_path
        .split(',')
        .map(|pp| std::fs::read_to_string(pp).expect("mass"))
        .collect::<Vec<_>>()
        .join("\n");
    for line in mass_text
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with("type,"))
    {
        let f: Vec<&str> = line.split(',').collect();
        let t: i64 = f[1].parse().unwrap();
        let hits: u64 = f[3].parse().unwrap();
        let m: f64 = f[4].parse().unwrap();
        let e: f64 = f[5].parse().unwrap();
        let en = mass.entry(t).or_insert((0.0, 0.0, 0));
        en.0 += m;
        en.1 += e * e;
        en.2 += hits;
    }
    // Tian: per t, number of l with yes / depends / no
    let mut tian: BTreeMap<i64, (u32, u32, u32)> = BTreeMap::new();
    for line in std::fs::read_to_string(tian_path)
        .expect("tian")
        .lines()
        .skip(1)
    {
        let f: Vec<&str> = line.split(',').collect();
        let t: i64 = f[1].parse().unwrap();
        let en = tian.entry(t).or_insert((0, 0, 0));
        match f[13] {
            "yes" => en.0 += 1,
            "depends" => en.1 += 1,
            _ => en.2 += 1,
        }
    }
    let allowed = allowed_traces(p);
    let mut rows =
        String::from("t,nE,ordinary,hits,mass,mass_se,H,haar_density,arith_x_haar,tian_yes,tian_depends,tian_no\n");
    let mut ts: Vec<i64> = Vec::new();
    let (mut ym, mut xh, mut xd, mut xc, mut xt) = (vec![], vec![], vec![], vec![], vec![]);
    let mut missing: Vec<i64> = Vec::new();
    for (ix, &ok) in allowed.iter().enumerate() {
        if !ok {
            continue;
        }
        let t = ix as i64 - 2 * (q * q) as i64;
        let disc = 4 * (q as i64).pow(4) - t * t;
        let ordinary = t % p as i64 != 0;
        let h = hurwitz(disc);
        let d = haar_density(t, q);
        let c = if disc > 0 {
            d * h / (disc as f64).sqrt()
        } else {
            0.0
        };
        let (m, e2, hits) = mass.get(&t).copied().unwrap_or((0.0, 0.0, 0));
        let (ty, td, tn) = tian.get(&t).copied().unwrap_or((0, 0, 0));
        rows.push_str(&format!(
            "{},{},{},{},{:.4},{:.4},{:.3},{:.6},{:.6},{},{},{}\n",
            t,
            ell_order(q, t),
            ordinary,
            hits,
            m,
            e2.sqrt(),
            h,
            d,
            c,
            ty,
            td,
            tn
        ));
        if hits == 0 {
            missing.push(t);
        }
        if ordinary {
            ts.push(t);
            ym.push(m);
            xh.push(h);
            xd.push(d);
            xc.push(c);
            xt.push(ty as f64 + 0.5 * td as f64);
        }
    }
    let mut s = String::new();
    s.push_str(&format!(
        "q={q}: prime-order traces with an elliptic curve: {} (ordinary {}); traces with no hit: {:?}\n",
        rows.lines().count() - 1,
        ts.len(),
        missing
    ));
    s.push_str(&format!(
        "Spearman(mass, H) = {:.3}\nSpearman(mass, Haar density) = {:.3}\nSpearman(mass, Haar*H/sqrt(disc)) = {:.3}\nSpearman(mass, #l with Tian kernel (depends = 1/2)) = {:.3}\n",
        spearman(&ym, &xh),
        spearman(&ym, &xd),
        spearman(&ym, &xc),
        spearman(&ym, &xt)
    ));
    // scale fits: mass ~ c * predictor, c by least squares on the ratio of sums
    for (name, x) in [("H", &xh), ("haar", &xd), ("haar*H/sqrt", &xc)] {
        let c = ym.iter().sum::<f64>() / x.iter().sum::<f64>();
        let mut rss = 0.0;
        let mut tss = 0.0;
        let mean = ym.iter().sum::<f64>() / ym.len() as f64;
        for i in 0..ym.len() {
            rss += (ym[i] - c * x[i]).powi(2);
            tss += (ym[i] - mean).powi(2);
        }
        s.push_str(&format!(
            "fit mass = c*{name}: c = {:.6e}, R^2 (through origin, ratio-of-sums c) = {:.3}\n",
            c,
            1.0 - rss / tss
        ));
    }
    std::fs::write(out.join("join_by_t.csv"), rows).unwrap();
    std::fs::write(out.join("join_summary.txt"), &s).unwrap();
    eprint!("{s}");
}

// ---------------------------------------------------------------------------
// Theorem 6 (regular semisimple Frobenius, l odd): closed form for the number
// of Tian kernels at n = 4, checked against the linear algebra above.
// ---------------------------------------------------------------------------

fn pol_trim8(v: &mut Vec<u8>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}

/// Remainder of `a` modulo the monic `b` over `F_l` (coefficients low to high).
fn pol_rem8(a: &[u8], b: &[u8], l: u8) -> Vec<u8> {
    let mut r = a.to_vec();
    pol_trim8(&mut r);
    let db = b.len() - 1;
    while r.len() > db {
        let c = r[r.len() - 1];
        let sh = r.len() - 1 - db;
        for i in 0..=db {
            r[sh + i] = fl_sub(l, r[sh + i], fl_mul(l, c, b[i]));
        }
        pol_trim8(&mut r);
    }
    r
}

fn pol_div8(a: &[u8], b: &[u8], l: u8) -> Vec<u8> {
    let mut r = a.to_vec();
    pol_trim8(&mut r);
    let db = b.len() - 1;
    let mut q = vec![0u8; r.len().saturating_sub(db)];
    while r.len() > db {
        let c = r[r.len() - 1];
        let sh = r.len() - 1 - db;
        q[sh] = c;
        for i in 0..=db {
            r[sh + i] = fl_sub(l, r[sh + i], fl_mul(l, c, b[i]));
        }
        pol_trim8(&mut r);
    }
    q
}

/// Monic irreducible factors of a squarefree monic `f` of degree <= 8 over
/// `F_l`, or `None` if `f` has a repeated factor.
fn factor_squarefree8(f: &[u8], l: u8) -> Option<Vec<Vec<u8>>> {
    let mut rest = f.to_vec();
    let mut out: Vec<Vec<u8>> = Vec::new();
    for d in 1..=4usize {
        let total = (l as u64).pow(d as u32);
        for code in 0..total {
            let mut g = vec![0u8; d + 1];
            let mut c = code;
            for x in g.iter_mut().take(d) {
                *x = (c % l as u64) as u8;
                c /= l as u64;
            }
            g[d] = 1;
            if rest.len() <= d {
                break;
            }
            if pol_rem8(&rest, &g, l).is_empty() {
                rest = pol_div8(&rest, &g, l);
                if pol_rem8(&rest, &g, l).is_empty() {
                    return None;
                }
                out.push(g);
            }
        }
    }
    if rest.len() > 1 {
        out.push(rest);
    }
    Some(out)
}

/// Monic polynomial with roots `qbar / beta` for the roots `beta` of `g`.
fn recip_twist8(g: &[u8], qbar: u8, l: u8) -> Vec<u8> {
    let d = g.len() - 1;
    let mut h = vec![0u8; d + 1];
    let mut qi = 1u8;
    for i in 0..=d {
        h[d - i] = fl_mul(l, g[i], qi);
        qi = fl_mul(l, qi, qbar);
    }
    let inv = fl_inv(l, h[d]);
    for x in h.iter_mut() {
        *x = fl_mul(l, *x, inv);
    }
    h
}

/// Theorem 6: for odd `l`, `chi(T^4) = T^8 - tr T^4 + qbar^4` squarefree over
/// `F_l`, the number of Tian kernels (`n = 4`) is `0` if some irreducible
/// factor is its own reciprocal twist `T -> qbar/T`, and otherwise
/// `2^(#factors/2) - 2·[chi has both roots in F_l]`. `None` if not squarefree.
fn tian_theory(l: u8, qbar: u8, tr: u8) -> Option<usize> {
    if l == 2 {
        return None;
    }
    let mut det = 1u8;
    for _ in 0..4 {
        det = fl_mul(l, det, qbar);
    }
    let mut f = vec![0u8; 9];
    f[8] = 1;
    f[4] = fl_sub(l, 0, tr);
    f[0] = det;
    let fac = factor_squarefree8(&f, l)?;
    for g in &fac {
        if recip_twist8(g, qbar, l) == *g {
            return Some(0);
        }
    }
    let split = class_reps(l, tr, det).iter().any(|(c, _)| *c == Cls::Split);
    let pairs = fac.len() / 2;
    let excl = if split { 2 } else { 0 };
    Some((1usize << pairs).saturating_sub(excl))
}

fn part_tian_theory(out: &Path, ells: &[u8]) {
    let mut rows = String::from("ell,qbar,tr,det,class,theory,linear_algebra,n_ker,agree\n");
    let mut bad = 0;
    let mut cells = 0;
    let mut cells_scalar_jordan = String::new();
    for &l in ells {
        for qbar in 1..l {
            let mut det = 1u8;
            for _ in 0..4 {
                det = fl_mul(l, det, qbar);
            }
            for tr in 0..l {
                for (cls, phi) in class_reps(l, tr, det) {
                    let th = tian_theory(l, qbar, tr);
                    let (nk, nt) = kernel_census(l, 4, qbar, phi);
                    let agree = match th {
                        Some(v) if cls == Cls::Split || cls == Cls::Nonsplit => v == nt,
                        _ => true,
                    };
                    if matches!(cls, Cls::Scalar | Cls::Jordan) || th.is_none() {
                        cells_scalar_jordan.push_str(&format!(
                            "l={l} qbar={qbar} tr={tr} {}: n_ker={nk} n_tian={nt}\n",
                            cls.name()
                        ));
                    } else {
                        cells += 1;
                        if !agree {
                            bad += 1;
                        }
                    }
                    rows.push_str(&format!(
                        "{l},{qbar},{tr},{det},{},{},{nt},{nk},{agree}\n",
                        cls.name(),
                        th.map(|v| v.to_string()).unwrap_or_else(|| "-".into()),
                    ));
                }
            }
        }
        eprintln!("theory check l={l} done");
    }
    std::fs::write(out.join("tian_theory.csv"), rows).unwrap();
    std::fs::write(out.join("non_regular_cells.txt"), &cells_scalar_jordan).unwrap();
    let s = format!("Theorem 6 cells compared: {cells}; disagreements: {bad}\n");
    std::fs::write(out.join("summary.txt"), &s).unwrap();
    eprint!("{s}");
}

// ---------------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn rand_curve(ty: Ty, p: u32, s: &mut u64) -> Curve {
        let code = splitmix(s) % ty.ncodes(p);
        decode(ty, p, code)
    }

    /// Brute-force point count on V(Q,F) in P^3(F_{p^k}).
    fn brute(eng: &Engine, c: &Curve, k: u32) -> u64 {
        let p = eng.p;
        let (q, f) = lift_cubic(p, c);
        let fl = eng.fld(k);
        let z = fl.zero();
        let mut n = 0u64;
        let all: Vec<u32> = (0..=fl.n1).collect();
        // normalised points: first nonzero coordinate = 1 (log 0)
        for lead in 0..4usize {
            let free = 3 - lead;
            let total = (all.len() as u64).pow(free as u32);
            for idx in 0..total {
                let mut x = [z; 4];
                x[lead] = 0;
                let mut r = idx;
                for pos in (lead + 1)..4 {
                    x[pos] = all[(r % all.len() as u64) as usize];
                    r /= all.len() as u64;
                }
                if sp_eval(fl, &q, &x) == z && sp_eval(fl, &f, &x) == z {
                    n += 1;
                }
            }
        }
        n
    }

    #[test]
    fn field_axioms() {
        for (p, j) in [(3u32, 2u32), (3, 4), (5, 2), (5, 3), (7, 2)] {
            let f = Fld::new(p, j, false);
            let mut s = 11u64;
            for _ in 0..500 {
                let a = (splitmix(&mut s) % (f.sz as u64)) as u32;
                let b = (splitmix(&mut s) % (f.sz as u64)) as u32;
                let c = (splitmix(&mut s) % (f.sz as u64)) as u32;
                // elements as logs, ZERO = n1 = sz - 1 ; a may be sz-1 = ZERO
                assert_eq!(f.add(a, b), f.add(b, a));
                assert_eq!(f.add(f.add(a, b), c), f.add(a, f.add(b, c)));
                assert_eq!(f.mul(a, f.add(b, c)), f.add(f.mul(a, b), f.mul(a, c)));
                assert_eq!(f.add(a, f.neg(a)), f.zero());
                // characteristic: p * a = 0
                let mut t = f.zero();
                for _ in 0..p {
                    t = f.add(t, a);
                }
                assert_eq!(t, f.zero());
            }
            // Frobenius is additive
            let a = 5 % f.n1;
            let b = 9 % f.n1;
            let fr = |x: u32| {
                if x == f.n1 {
                    x
                } else {
                    ((x as u64 * p as u64) % f.n1 as u64) as u32
                }
            };
            assert_eq!(fr(f.add(a, b)), f.add(fr(a), fr(b)));
        }
    }

    #[test]
    fn cubic_tables_vs_brute() {
        for (p, j) in [(3u32, 2u32), (3, 3), (5, 2), (7, 1), (5, 3)] {
            let f = Fld::new(p, j, true);
            let z = f.zero();
            let mut s = 3u64;
            for _ in 0..300 {
                let c: Vec<u32> = (0..4)
                    .map(|_| (splitmix(&mut s) % (f.sz as u64)) as u32)
                    .collect();
                // brute-force distinct roots in P^1
                let mut n = 0u64;
                for y in 0..=f.n1 {
                    let mut v = c[3];
                    v = f.add(f.mul(v, y), c[2]);
                    v = f.add(f.mul(v, y), c[1]);
                    v = f.add(f.mul(v, y), c[0]);
                    if v == z {
                        n += 1;
                    }
                }
                if c[3] == z {
                    n += 1;
                }
                if c.iter().all(|&x| x == z) {
                    n = f.sz as u64 + 1;
                }
                assert_eq!(f.cubic_form_roots(c[0], c[1], c[2], c[3]), n);
            }
        }
    }

    #[test]
    fn counts_match_p3_brute_force() {
        for (p, ks) in [(3u32, vec![1u32, 2, 3]), (5, vec![1, 2])] {
            let eng = Engine::new(p);
            let mut s = 99u64;
            for ty in [Ty::Split, Ty::Nonsplit, Ty::Cone] {
                for _ in 0..6 {
                    let c = rand_curve(ty, p, &mut s);
                    for &k in &ks {
                        let a = eng.count(&c, k);
                        let b = brute(&eng, &c, k);
                        assert_eq!(a, b, "{:?} p={p} k={k} {:?}", ty, c);
                    }
                }
            }
        }
    }

    #[test]
    fn hyper_squarefree_count_q3() {
        let p = 3u32;
        let mut n = 0u64;
        for code in 1..3u64.pow(11) {
            let mut c = [0u32; 11];
            let mut x = code;
            for v in c.iter_mut() {
                *v = (x % 3) as u32;
                x /= 3;
            }
            if hyper_squarefree(p, &c) {
                n += 1;
            }
        }
        assert_eq!(n, 104976);
    }

    /// Singular point of V(Q,F) among points over F_{p^m}, m <= mmax.
    fn brute_singular(eng: &Engine, c: &Curve, mmax: u32) -> bool {
        let p = eng.p;
        let (q, f) = lift_cubic(p, c);
        let dq: Vec<SPoly> = (0..4).map(|v| sp_deriv(&q, v, p)).collect();
        let df: Vec<SPoly> = (0..4).map(|v| sp_deriv(&f, v, p)).collect();
        for m in 1..=mmax {
            let fl = eng.fld(m);
            let z = fl.zero();
            let all: Vec<u32> = (0..=fl.n1).collect();
            for lead in 0..4usize {
                let free = 3 - lead;
                let total = (all.len() as u64).pow(free as u32);
                for idx in 0..total {
                    let mut x = [z; 4];
                    x[lead] = 0;
                    let mut r = idx;
                    for pos in (lead + 1)..4 {
                        x[pos] = all[(r % all.len() as u64) as usize];
                        r /= all.len() as u64;
                    }
                    if sp_eval(fl, &q, &x) != z || sp_eval(fl, &f, &x) != z {
                        continue;
                    }
                    // rank of Jacobian < 2 : all 2x2 minors vanish
                    let a: Vec<u32> = dq.iter().map(|d| sp_eval(fl, d, &x)).collect();
                    let b: Vec<u32> = df.iter().map(|d| sp_eval(fl, d, &x)).collect();
                    let mut all_zero = true;
                    for i in 0..4 {
                        for j in (i + 1)..4 {
                            let m1 = fl.mul(a[i], b[j]);
                            let m2 = fl.mul(a[j], b[i]);
                            if fl.sub(m1, m2) != z {
                                all_zero = false;
                            }
                        }
                    }
                    if all_zero {
                        return true;
                    }
                }
            }
        }
        false
    }

    #[test]
    fn smoothness_vs_brute_force() {
        let p = 3u32;
        let eng = Engine::new(p);
        let mut s = 5u64;
        let mut agree_sing = 0;
        let mut agree_smooth = 0;
        let mut unexplained = 0;
        for ty in [Ty::Split, Ty::Nonsplit, Ty::Cone] {
            for _ in 0..40 {
                let c = rand_curve(ty, p, &mut s);
                let sm = is_smooth(p, &c);
                let bs = brute_singular(&eng, &c, 2);
                if sm {
                    assert!(!bs, "smooth by Macaulay but singular point found: {:?}", c);
                    agree_smooth += 1;
                } else if bs {
                    agree_sing += 1;
                } else {
                    unexplained += 1;
                }
            }
        }
        assert!(
            agree_smooth > 10 && agree_sing > 5,
            "{agree_smooth} {agree_sing}"
        );
        assert!(
            unexplained <= 8,
            "too many singular-without-small-point: {unexplained}"
        );
    }

    #[test]
    fn orbit_sizes_divide_group_order() {
        for p in [3u32] {
            let mut s = 21u64;
            for ty in [Ty::Split, Ty::Nonsplit, Ty::Cone] {
                for _ in 0..3 {
                    let c = loop {
                        let c = rand_curve(ty, p, &mut s);
                        if is_smooth(p, &c) {
                            break c;
                        }
                    };
                    let orb = orbit_codes(p, &c);
                    let g = ty.group_order(p as u64) * ty.scalar_factor(p as u64);
                    assert_eq!(
                        g % orb.len() as u64,
                        0,
                        "{:?}: orbit {} g {}",
                        ty,
                        orb.len(),
                        g
                    );
                    // orbit members have the same point counts
                    let eng = Engine::new(p);
                    let n1 = eng.count(&c, 1);
                    let n2 = eng.count(&c, 2);
                    for code in orb.iter().take(20) {
                        let d = decode(ty, p, *code);
                        assert_eq!(eng.count(&d, 1), n1);
                        assert_eq!(eng.count(&d, 2), n2);
                    }
                }
            }
        }
    }

    #[test]
    fn tian_known_cells() {
        // Round 2, check 3: (n, l, qbar) = (3, 3, 1), phi = 2I has 12 Tian kernels.
        let (_k, t) = kernel_census(3, 3, 1, [[2, 0], [0, 2]]);
        assert_eq!(t, 12);
        // Theorem 4: nonsplit has none at n = 3
        let (_k, t) = kernel_census(3, 3, 1, [[0, 2], [1, 1]]);
        assert_eq!(t, 0);
    }

    #[test]
    fn theorem6_matches_linear_algebra_l3_l5() {
        for l in [3u8, 5] {
            for qbar in 1..l {
                let mut det = 1u8;
                for _ in 0..4 {
                    det = fl_mul(l, det, qbar);
                }
                for tr in 0..l {
                    for (cls, phi) in class_reps(l, tr, det) {
                        if let Some(v) = tian_theory(l, qbar, tr) {
                            if cls == Cls::Split || cls == Cls::Nonsplit {
                                let (_k, nt) = kernel_census(l, 4, qbar, phi);
                                assert_eq!(v, nt, "l={l} qbar={qbar} tr={tr} {:?}", cls);
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn hurwitz_small() {
        assert!((hurwitz(3) - 1.0 / 3.0).abs() < 1e-12);
        assert!((hurwitz(4) - 0.5).abs() < 1e-12);
        assert!((hurwitz(7) - 1.0).abs() < 1e-12);
        assert!((hurwitz(12) - 4.0 / 3.0).abs() < 1e-12);
        assert!((hurwitz(23) - 3.0).abs() < 1e-12);
    }

    #[test]
    fn spearman_basic() {
        assert!((spearman(&[1.0, 2.0, 3.0], &[10.0, 20.0, 35.0]) - 1.0).abs() < 1e-12);
        assert!((spearman(&[1.0, 2.0, 3.0], &[3.0, 2.0, 1.0]) + 1.0).abs() < 1e-12);
    }

    #[test]
    fn waterhouse_q3() {
        // q = 3: traces with 3 | t that exist: 0, +-9, +-18
        let ok: Vec<i64> = (-18..=18)
            .filter(|&t| t % 3 == 0 && ell_exists(3, 3, t))
            .collect();
        assert_eq!(ok, vec![-18, -9, 0, 9, 18]);
    }

    #[test]
    fn plane_model_identity_small() {
        let p = 3u32;
        let eng = Engine::new(p);
        let mut s = 77u64;
        let mut checked = 0;
        for ty in [Ty::Split, Ty::Nonsplit, Ty::Cone] {
            let mut done = 0;
            while done < 3 {
                let c = rand_curve(ty, p, &mut s);
                if !is_smooth(p, &c) {
                    continue;
                }
                let pts = p3_rational_points_on(&eng, &c);
                let Some(pt) = pts.first() else { continue };
                let mut r = PlaneRes::default();
                plane_check(&eng, &c, *pt, &[1, 2], &mut r);
                assert_eq!(r.mismatches, 0, "{:?} {:?}", ty, c);
                assert_eq!(r.sing_mismatch, 0, "{:?} {:?}", ty, c);
                done += 1;
                checked += 1;
            }
        }
        assert_eq!(checked, 9);
    }
}
