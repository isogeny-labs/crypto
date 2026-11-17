//! Jacobian census for the n = 3 Tian route: does the isogeny class of
//! `Res_{F_{q^3}/F_q} E` contain the Jacobian of a smooth plane quartic over
//! `F_q`?  (`research/tian_n3_quartic_route_20261024/PROTOCOL.md`, item (a).)
//!
//! Facts used (derived in PROTOCOL.md section 2):
//! * the `q`-Frobenius of `Res E` has characteristic polynomial
//!   `P(T) = T^6 - t3 T^3 + q^3`, `t3 = tr(pi_{q^3} | E)`;
//! * a genus-3 curve `C/F_q` has `L_C(T) = 1 + c1 T + c2 T^2 + c3 T^3 + q c2 T^4
//!   + q^2 c1 T^5 + q^3 T^6`; `L_C` equals the reversed `P` iff `c1 = c2 = 0`,
//!   `c3 = -t3`, i.e. iff `#C(F_q) = q+1`, `#C(F_q^2) = q^2+1`,
//!   `#C(F_q^3) = q^3 + 1 - 3 t3`.  Equal polynomials mean isogenous Jacobians
//!   (Tate).
//!
//!
//! Commands (std-only; `rustc --edition 2021 -O examples/tian_quartic_census.rs`):
//!   `list <out-dir> <round2-curves.csv>`   Weil-polynomial table, own Theorem 4
//!                                          flags, cross-check against round 2
//!   `search <out-dir> <q> <samples> <seed> [curves.csv]`   random-quartic census
//!   `smooth <out-dir> <q> <samples> <seed>`   smooth fraction and class mass
//!   `control <out-dir> <q> <samples> <seed>`  positive control (constructed quartic)
//!   `recount <out-dir> <q> <seed> <keep-percent>`  independent brute-force recount of examples
//!   `aa <out-dir> <census1.csv> <census2.csv>`   A/A chi-square of two searches
//!   `summarize <out-dir> <q>...`   hits by group (kernel / no kernel / composite) against local density
#![allow(
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::manual_is_multiple_of,
    clippy::manual_div_ceil,
    clippy::wrong_self_convention,
    clippy::ptr_arg,
    clippy::doc_lazy_continuation,
    clippy::match_like_matches_macro,
    clippy::collapsible_else_if,
    clippy::collapsible_if
)]

use std::collections::BTreeMap;
use std::fmt::Write as _;

// ---------------------------------------------------------------- utilities

struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xD1B5_4A32_D192_ED03)
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
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

// ------------------------------------------------------- extension fields

type El = [u64; 3];

#[derive(Clone)]
struct Fld {
    q: u64,
    k: usize,
    red: [u64; 3], // t^k = red[0] + red[1] t + red[2] t^2
    size: u64,
}

impl Fld {
    fn new(q: u64, k: usize) -> Fld {
        let mut red = [0u64; 3];
        match k {
            1 => {}
            2 => {
                // t^2 = n, n a non-residue
                let n = (2..q)
                    .find(|&n| powmod(n, (q - 1) / 2, q) == q - 1)
                    .unwrap();
                red[0] = n;
            }
            3 => {
                // t^3 = c1 t + c0 without roots in F_q
                'outer: for c1 in 0..q {
                    for c0 in 1..q {
                        let root =
                            (0..q).any(|t| (t * t % q * t % q + q * q - c1 * t % q - c0) % q == 0);
                        if !root {
                            red[0] = c0;
                            red[1] = c1;
                            break 'outer;
                        }
                    }
                }
            }
            _ => panic!("k"),
        }
        Fld {
            q,
            k,
            red,
            size: q.pow(k as u32),
        }
    }
    fn zero(&self) -> El {
        [0; 3]
    }
    fn one(&self) -> El {
        [1, 0, 0]
    }
    fn is_zero(&self, a: &El) -> bool {
        a[0] == 0 && a[1] == 0 && a[2] == 0
    }
    fn add(&self, a: &El, b: &El) -> El {
        let q = self.q;
        [(a[0] + b[0]) % q, (a[1] + b[1]) % q, (a[2] + b[2]) % q]
    }
    fn sub(&self, a: &El, b: &El) -> El {
        let q = self.q;
        [
            (a[0] + q - b[0]) % q,
            (a[1] + q - b[1]) % q,
            (a[2] + q - b[2]) % q,
        ]
    }
    fn neg(&self, a: &El) -> El {
        self.sub(&self.zero(), a)
    }
    fn scal(&self, a: &El, c: u64) -> El {
        let q = self.q;
        [a[0] * c % q, a[1] * c % q, a[2] * c % q]
    }
    fn mul(&self, a: &El, b: &El) -> El {
        let q = self.q;
        let k = self.k;
        if k == 1 {
            return [a[0] * b[0] % q, 0, 0];
        }
        let mut p = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                p[i + j] += a[i] * b[j];
            }
        }
        for v in p.iter_mut() {
            *v %= q;
        }
        for d in (k..=2 * k - 2).rev() {
            let c = p[d];
            if c != 0 {
                for i in 0..k {
                    p[d - k + i] = (p[d - k + i] + c * self.red[i]) % q;
                }
            }
        }
        [
            p[0],
            if k > 1 { p[1] } else { 0 },
            if k > 2 { p[2] } else { 0 },
        ]
    }
    fn pow(&self, a: &El, mut e: u64) -> El {
        let mut r = self.one();
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(&r, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: &El) -> El {
        self.pow(a, self.size - 2)
    }
    fn from_index(&self, mut n: u64) -> El {
        let mut e = [0u64; 3];
        for d in 0..self.k {
            e[d] = n % self.q;
            n /= self.q;
        }
        e
    }
    fn embed(&self, c: u64) -> El {
        [c % self.q, 0, 0]
    }
}

// ----------------------------------------------------------- plane quartics

/// Coefficient index of the monomial x^i y^j z^(4-i-j).
fn midx(i: usize, j: usize) -> usize {
    // row i starts after sum_{r<i} (5 - r)
    let mut s = 0;
    for r in 0..i {
        s += 5 - r;
    }
    s + j
}

type Quartic = [u64; 15];

/// Table of the 15 monomial values at every point of P^2(F_q) for the fast N1 count.
struct PointTable {
    q: u64,
    mon: Vec<[u32; 15]>,
}

impl PointTable {
    fn new(q: u64) -> PointTable {
        let mut pts = Vec::new();
        for x in 0..q {
            for y in 0..q {
                pts.push((x, y, 1u64));
            }
        }
        for x in 0..q {
            pts.push((x, 1, 0));
        }
        pts.push((1, 0, 0));
        let mut mon = Vec::with_capacity(pts.len());
        for &(x, y, z) in &pts {
            let mut xp = [1u64; 5];
            let mut yp = [1u64; 5];
            let mut zp = [1u64; 5];
            for e in 1..5 {
                xp[e] = xp[e - 1] * x % q;
                yp[e] = yp[e - 1] * y % q;
                zp[e] = zp[e - 1] * z % q;
            }
            let mut m = [0u32; 15];
            for i in 0..5 {
                for j in 0..(5 - i) {
                    m[midx(i, j)] = (xp[i] * yp[j] % q * zp[4 - i - j] % q) as u32;
                }
            }
            mon.push(m);
        }
        PointTable { q, mon }
    }
    fn count(&self, c: &Quartic) -> u64 {
        let q = self.q as u32;
        let mut cc = [0u32; 15];
        for i in 0..15 {
            cc[i] = c[i] as u32;
        }
        let mut n = 0u64;
        for m in &self.mon {
            let mut s = 0u32;
            for i in 0..15 {
                s += cc[i] * m[i];
            }
            if s % q == 0 {
                n += 1;
            }
        }
        n
    }
}

/// Number of distinct roots in F_Q of the polynomial `a[0] + a[1] y + ... + a[4] y^4`
/// (Q if it is identically zero).
fn nroots(f: &Fld, a: &[El; 5]) -> u64 {
    let mut d = 5;
    while d > 0 && f.is_zero(&a[d - 1]) {
        d -= 1;
    }
    if d == 0 {
        return f.size;
    }
    let d = d - 1; // degree
    if d == 0 {
        return 0;
    }
    if d == 1 {
        return 1;
    }
    let li = f.inv(&a[d]);
    let mut m = [f.zero(); 5];
    for i in 0..=d {
        m[i] = f.mul(&a[i], &li);
    }
    // y^Q mod m by square-and-multiply
    let nb = 64 - f.size.leading_zeros();
    let mut r: Vec<El> = vec![f.zero(); d];
    r[1] = f.one();
    for bit in (0..nb - 1).rev() {
        r = polmulmod(f, &r, &r, &m, d);
        if (f.size >> bit) & 1 == 1 {
            // multiply by y
            let top = r[d - 1];
            for i in (1..d).rev() {
                r[i] = r[i - 1];
            }
            r[0] = f.zero();
            if !f.is_zero(&top) {
                for i in 0..d {
                    r[i] = f.sub(&r[i], &f.mul(&top, &m[i]));
                }
            }
        }
    }
    // h = y^Q - y mod m
    r[1] = f.sub(&r[1], &f.one());
    if r.iter().all(|e| f.is_zero(e)) {
        return d as u64;
    }
    // gcd(m, r)
    let mut p: Vec<El> = m[..=d].to_vec();
    let mut s: Vec<El> = r;
    loop {
        while let Some(l) = s.last() {
            if f.is_zero(l) {
                s.pop();
            } else {
                break;
            }
        }
        if s.is_empty() {
            return (p.len() - 1) as u64;
        }
        if s.len() == 1 {
            return 0;
        }
        // p mod s
        let li = f.inv(s.last().unwrap());
        let sd = s.len() - 1;
        let mut rem = p.clone();
        while rem.len() > sd {
            let top = *rem.last().unwrap();
            let c = f.mul(&top, &li);
            let sh = rem.len() - 1 - sd;
            if !f.is_zero(&c) {
                for i in 0..=sd {
                    rem[sh + i] = f.sub(&rem[sh + i], &f.mul(&c, &s[i]));
                }
            }
            rem.pop();
        }
        p = s;
        s = rem;
    }
}

fn polmulmod(f: &Fld, a: &[El], b: &[El], m: &[El; 5], d: usize) -> Vec<El> {
    let mut prod = vec![f.zero(); 2 * d - 1];
    for i in 0..d {
        if f.is_zero(&a[i]) {
            continue;
        }
        for j in 0..d {
            let t = f.mul(&a[i], &b[j]);
            prod[i + j] = f.add(&prod[i + j], &t);
        }
    }
    for t in (d..2 * d - 1).rev() {
        let c = prod[t];
        if !f.is_zero(&c) {
            for i in 0..d {
                prod[t - d + i] = f.sub(&prod[t - d + i], &f.mul(&c, &m[i]));
            }
        }
    }
    prod.truncate(d);
    prod
}

/// Frobenius images of the basis, for orbit representatives.
fn frob_basis(f: &Fld) -> [El; 3] {
    let mut out = [f.zero(); 3];
    for i in 0..f.k {
        let mut t = f.zero();
        t[i] = 1;
        out[i] = f.pow(&t, f.q);
    }
    out
}

fn frob(f: &Fld, fb: &[El; 3], x: &El) -> El {
    let mut r = f.zero();
    for i in 0..f.k {
        r = f.add(&r, &f.scal(&fb[i], x[i]));
    }
    r
}

/// #C(F_{q^k}) for the projective quartic (including points at infinity).
fn count_points(c: &Quartic, f: &Fld) -> u64 {
    let fb = frob_basis(f);
    let mut total = 0u64;
    for n in 0..f.size {
        let x = f.from_index(n);
        let mut w = 1u64;
        if f.k > 1 {
            let x1 = frob(f, &fb, &x);
            if x1 != x {
                let key = |e: &El| (e[2], e[1], e[0]);
                if f.k == 2 {
                    if key(&x) > key(&x1) {
                        continue;
                    }
                    w = 2;
                } else {
                    let x2 = frob(f, &fb, &x1);
                    if key(&x) > key(&x1) || key(&x) > key(&x2) {
                        continue;
                    }
                    w = 3;
                }
            }
        }
        let mut xp = [f.one(); 5];
        for e in 1..5 {
            xp[e] = f.mul(&xp[e - 1], &x);
        }
        let mut a = [f.zero(); 5];
        for j in 0..5 {
            let mut s = f.zero();
            for i in 0..(5 - j) {
                let cf = c[midx(i, j)];
                if cf != 0 {
                    s = f.add(&s, &f.scal(&xp[i], cf));
                }
            }
            a[j] = s;
        }
        total += w * nroots(f, &a);
    }
    // points at infinity: [0:1:0] and [1:y:0]
    if c[midx(0, 4)] == 0 {
        total += 1;
    }
    let mut a = [f.zero(); 5];
    for j in 0..5 {
        a[j] = f.embed(c[midx(4 - j, j)]);
    }
    total += nroots(f, &a);
    total
}

/// Independent brute-force count over P^2(F_{q^k}) (no root-finding, no orbits, streaming).
fn count_points_brute(c: &Quartic, f: &Fld) -> u64 {
    let zero_at = |p: &[El; 3]| -> bool {
        let mut pw = [[f.one(); 5]; 3];
        for v in 0..3 {
            for e in 1..5 {
                pw[v][e] = f.mul(&pw[v][e - 1], &p[v]);
            }
        }
        let mut s = f.zero();
        for i in 0..5 {
            for j in 0..(5 - i) {
                let cf = c[midx(i, j)];
                if cf == 0 {
                    continue;
                }
                let t = f.mul(&f.mul(&pw[0][i], &pw[1][j]), &pw[2][4 - i - j]);
                s = f.add(&s, &f.scal(&t, cf));
            }
        }
        f.is_zero(&s)
    };
    let mut n = 0;
    for ny in 0..f.size {
        let y = f.from_index(ny);
        for nx in 0..f.size {
            if zero_at(&[f.from_index(nx), y, f.one()]) {
                n += 1;
            }
        }
    }
    for nx in 0..f.size {
        if zero_at(&[f.from_index(nx), f.one(), f.zero()]) {
            n += 1;
        }
    }
    if zero_at(&[f.one(), f.zero(), f.zero()]) {
        n += 1;
    }
    n
}

// ------------------------------------------------------------- smoothness

type Form = [[u64; 5]; 5]; // [i][j] coefficient of x^i y^j z^(d-i-j)

fn form_mul(q: u64, a: &Form, b: &Form) -> Form {
    let mut r = [[0u64; 5]; 5];
    for i1 in 0..5 {
        for j1 in 0..5 {
            if a[i1][j1] == 0 {
                continue;
            }
            for i2 in 0..(5 - i1) {
                for j2 in 0..(5 - j1) {
                    if i1 + i2 < 5 && j1 + j2 < 5 {
                        r[i1 + i2][j1 + j2] = (r[i1 + i2][j1 + j2] + a[i1][j1] * b[i2][j2]) % q;
                    }
                }
            }
        }
    }
    r
}

fn to_form(c: &Quartic) -> Form {
    let mut f = [[0u64; 5]; 5];
    for i in 0..5 {
        for j in 0..(5 - i) {
            f[i][j] = c[midx(i, j)];
        }
    }
    f
}

fn from_form(f: &Form) -> Quartic {
    let mut c = [0u64; 15];
    for i in 0..5 {
        for j in 0..(5 - i) {
            c[midx(i, j)] = f[i][j];
        }
    }
    c
}

/// F(M v) for a 3x3 matrix M over F_q.
fn transform(q: u64, c: &Quartic, m: &[[u64; 3]; 3]) -> Quartic {
    let one: Form = {
        let mut f = [[0; 5]; 5];
        f[0][0] = 1;
        f
    };
    // linear forms L_r = m[r][0] x + m[r][1] y + m[r][2] z, as forms of degree 1
    let mut lin = Vec::new();
    for r in 0..3 {
        let mut f = [[0u64; 5]; 5];
        f[1][0] = m[r][0];
        f[0][1] = m[r][1];
        f[0][0] = m[r][2]; // z coefficient: i = j = 0, k = 1
        lin.push(f);
    }
    // powers of each linear form
    let mut pw: Vec<[Form; 5]> = Vec::new();
    for r in 0..3 {
        let mut p = [one; 5];
        for e in 1..5 {
            p[e] = form_mul(q, &p[e - 1], &lin[r]);
        }
        pw.push(p);
    }
    let mut out = [[0u64; 5]; 5];
    for i in 0..5 {
        for j in 0..(5 - i) {
            let cf = c[midx(i, j)];
            if cf == 0 {
                continue;
            }
            let k = 4 - i - j;
            let t = form_mul(q, &form_mul(q, &pw[0][i], &pw[1][j]), &pw[2][k]);
            for a in 0..5 {
                for b in 0..5 {
                    out[a][b] = (out[a][b] + cf * t[a][b]) % q;
                }
            }
        }
    }
    from_form(&out)
}

fn det_mod(f: &Fld, m: &mut Vec<Vec<El>>) -> El {
    let n = m.len();
    let mut det = f.one();
    for col in 0..n {
        let mut piv = None;
        for r in col..n {
            if !f.is_zero(&m[r][col]) {
                piv = Some(r);
                break;
            }
        }
        let Some(p) = piv else {
            return f.zero();
        };
        if p != col {
            m.swap(p, col);
            det = f.neg(&det);
        }
        det = f.mul(&det, &m[col][col]);
        let inv = f.inv(&m[col][col]);
        for r in (col + 1)..n {
            if f.is_zero(&m[r][col]) {
                continue;
            }
            let fac = f.mul(&m[r][col], &inv);
            for c2 in col..n {
                let t = f.mul(&fac, &m[col][c2]);
                m[r][c2] = f.sub(&m[r][c2], &t);
            }
        }
    }
    det
}

/// Sylvester resultant of two formal-degree-3 polynomials (coefficients low to high).
fn res33(f: &Fld, a: &[El; 4], b: &[El; 4]) -> El {
    let mut m = vec![vec![f.zero(); 6]; 6];
    for r in 0..3 {
        for j in 0..4 {
            m[r][r + j] = a[3 - j];
            m[r + 3][r + j] = b[3 - j];
        }
    }
    det_mod(f, &mut m)
}

fn partials(q: u64, fm: &Form) -> [Form; 3] {
    // F_x, F_y, F_z as forms of degree 3
    let mut gx = [[0u64; 5]; 5];
    let mut gy = [[0u64; 5]; 5];
    let mut gz = [[0u64; 5]; 5];
    for i in 0..5 {
        for j in 0..(5 - i) {
            let k = 4 - i - j;
            let c = fm[i][j];
            if c == 0 {
                continue;
            }
            if i > 0 {
                gx[i - 1][j] = (gx[i - 1][j] + c * i as u64) % q;
            }
            if j > 0 {
                gy[i][j - 1] = (gy[i][j - 1] + c * j as u64) % q;
            }
            if k > 0 {
                gz[i][j] = (gz[i][j] + c * k as u64) % q;
            }
        }
    }
    [gx, gy, gz]
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Smooth {
    Smooth,
    Uncertified,
}

/// Resultant polynomial of two cubic partials in the chart z = 1, as coefficients over F_q.
fn res_poly_affine(f2: &Fld, ga: &Form, gb: &Form) -> Vec<u64> {
    // evaluate at 10 points of F_{q^2}, interpolate (degree <= 9)
    let npts = 10;
    let mut xs = Vec::new();
    let mut vs = Vec::new();
    for n in 0..npts {
        let x0 = f2.from_index(n as u64 + 1);
        xs.push(x0);
        let mut ca = [f2.zero(); 4];
        let mut cb = [f2.zero(); 4];
        let mut xp = [f2.one(); 4];
        for e in 1..4 {
            xp[e] = f2.mul(&xp[e - 1], &x0);
        }
        for j in 0..4 {
            for i in 0..(4 - j) {
                ca[j] = f2.add(&ca[j], &f2.scal(&xp[i], ga[i][j]));
                cb[j] = f2.add(&cb[j], &f2.scal(&xp[i], gb[i][j]));
            }
        }
        vs.push(res33(f2, &ca, &cb));
    }
    // Lagrange interpolation over F_{q^2}
    let mut poly = vec![f2.zero(); npts];
    for i in 0..npts {
        let mut num = vec![f2.one()];
        let mut den = f2.one();
        for j in 0..npts {
            if i == j {
                continue;
            }
            // num *= (x - xs[j])
            let mut nn = vec![f2.zero(); num.len() + 1];
            for (d, c) in num.iter().enumerate() {
                nn[d + 1] = f2.add(&nn[d + 1], c);
                nn[d] = f2.sub(&nn[d], &f2.mul(c, &xs[j]));
            }
            num = nn;
            den = f2.mul(&den, &f2.sub(&xs[i], &xs[j]));
        }
        let w = f2.mul(&vs[i], &f2.inv(&den));
        for (d, c) in num.iter().enumerate() {
            poly[d] = f2.add(&poly[d], &f2.mul(c, &w));
        }
    }
    poly.iter()
        .map(|e| {
            assert!(e[1] == 0, "resultant polynomial not over F_q");
            e[0]
        })
        .collect()
}

fn smooth_once(f2: &Fld, fm: &Form) -> bool {
    let q = f2.q;
    let g = partials(q, fm);
    // infinity: some pair of binary cubics with non-zero resultant
    let mut inf_ok = false;
    for (a, b) in [(0, 1), (0, 2), (1, 2)] {
        let mut ca = [f2.zero(); 4];
        let mut cb = [f2.zero(); 4];
        for j in 0..4 {
            ca[j] = f2.embed(g[a][3 - j][j]);
            cb[j] = f2.embed(g[b][3 - j][j]);
        }
        if !f2.is_zero(&res33(f2, &ca, &cb)) {
            inf_ok = true;
            break;
        }
    }
    if !inf_ok {
        return false;
    }
    let r01 = res_poly_affine(f2, &g[0], &g[1]);
    let r02 = res_poly_affine(f2, &g[0], &g[2]);
    let r12 = res_poly_affine(f2, &g[1], &g[2]);
    // gcd of the three
    let mut cur = r01;
    for r in [r02, r12] {
        // gcd(cur, r) as a polynomial: compute fully
        cur = poly_gcd_poly(q, &cur, &r);
    }
    // cur is the gcd (trimmed); smooth certificate iff it is a non-zero constant
    let mut c = cur;
    while let Some(&l) = c.last() {
        if l == 0 {
            c.pop();
        } else {
            break;
        }
    }
    c.len() == 1
}

fn poly_gcd_poly(q: u64, a: &[u64], b: &[u64]) -> Vec<u64> {
    fn trim(v: &mut Vec<u64>) {
        while let Some(&l) = v.last() {
            if l == 0 {
                v.pop();
            } else {
                break;
            }
        }
    }
    let mut p = a.to_vec();
    let mut s = b.to_vec();
    trim(&mut p);
    trim(&mut s);
    while !s.is_empty() {
        let li = powmod(*s.last().unwrap(), q - 2, q);
        let sd = s.len() - 1;
        let mut rem = p.clone();
        while rem.len() > sd {
            let c = rem.last().unwrap() * li % q;
            let sh = rem.len() - 1 - sd;
            for i in 0..=sd {
                rem[sh + i] = (rem[sh + i] + q * q - c * s[i] % q) % q;
            }
            rem.pop();
        }
        trim(&mut rem);
        p = s;
        s = rem;
    }
    p
}

/// Smoothness certificate: sound (never certifies a singular curve); a smooth curve
/// that no tried coordinate system certifies is reported `Uncertified`.
fn smooth_check(f2: &Fld, c: &Quartic, rng: &mut Rng) -> Smooth {
    let q = f2.q;
    if smooth_once(f2, &to_form(c)) {
        return Smooth::Smooth;
    }
    for _ in 0..4 {
        let m = loop {
            let m = [
                [rng.below(q), rng.below(q), rng.below(q)],
                [rng.below(q), rng.below(q), rng.below(q)],
                [rng.below(q), rng.below(q), rng.below(q)],
            ];
            let d = (m[0][0] * ((m[1][1] * m[2][2] + q * q - m[1][2] * m[2][1]) % q)
                + m[0][1] * ((m[1][2] * m[2][0] + q * q - m[1][0] * m[2][2]) % q)
                + m[0][2] * ((m[1][0] * m[2][1] + q * q - m[1][1] * m[2][0]) % q))
                % q;
            if d != 0 {
                break m;
            }
        };
        let c2 = transform(q, c, &m);
        if smooth_once(f2, &to_form(&c2)) {
            return Smooth::Smooth;
        }
    }
    Smooth::Uncertified
}

// --------------------------------------------------------- Theorem 4 flags

#[derive(Debug, Clone, Copy, PartialEq)]
enum Flag {
    No,
    Yes,
    /// double root with l^2 | disc: scalar (kernel) or Jordan (kernel iff l = 1 mod 3)
    Ambiguous,
}

#[derive(Debug, Clone)]
struct EllInfo {
    kind: &'static str, // nonsplit | split | double
    flag: Flag,
    eig1: bool,
}

/// Theorem 4 (round 2) at the level of (q, t3): the type of chi(T) = T^2 - t3 T + q^3 mod l.
fn tian_flag(q: u64, t3: i64, ell: u64) -> EllInfo {
    let qq = (q as i128).pow(3);
    let lm = ell as i128;
    let tm = (t3 as i128).rem_euclid(lm);
    let qm = qq.rem_euclid(lm);
    let mut roots = Vec::new();
    for r in 0..ell as i128 {
        if (r * r - tm * r + qm).rem_euclid(lm) == 0 {
            roots.push(r);
        }
    }
    let is_cube = |a: i128| -> bool {
        if ell % 3 != 1 {
            return true;
        }
        powmod(a as u64, (ell - 1) / 3, ell) == 1
    };
    let eig1 = roots.contains(&1);
    match roots.len() {
        0 => EllInfo {
            kind: "nonsplit",
            flag: Flag::No,
            eig1,
        },
        2 => EllInfo {
            kind: "split",
            flag: if is_cube(roots[0]) {
                Flag::Yes
            } else {
                Flag::No
            },
            eig1,
        },
        _ => {
            let disc = (t3 as i128) * (t3 as i128) - 4 * qq;
            let scalar_possible = disc.rem_euclid(lm * lm) == 0;
            let flag = if scalar_possible {
                if ell % 3 == 1 {
                    Flag::Yes
                } else {
                    Flag::Ambiguous
                }
            } else if ell % 3 == 1 {
                Flag::Yes
            } else {
                Flag::No
            };
            EllInfo {
                kind: "double",
                flag,
                eig1,
            }
        }
    }
}

/// `t3 = s^3 - 3 q s` for an integer |s| <= 2 sqrt(q): E is isogenous to a base change from F_q
/// and the Weil polynomial is reducible over Q.
fn constant_type(q: u64, t3: i64) -> bool {
    let b = (2.0 * (q as f64).sqrt()).floor() as i64;
    (-b..=b).any(|s| s * s * s - 3 * (q as i64) * s == t3)
}

/// Waterhouse: for n = 3 an elliptic curve over F_{q^3} (q prime) with trace t3 exists iff
/// p does not divide t3, or t3 = 0 (supersingular, n odd).
fn realizable(q: u64, t3: i64) -> bool {
    t3 == 0 || t3.rem_euclid(q as i64) != 0
}

fn hasse_bound(q: u64) -> i64 {
    isqrt(4 * q * q * q) as i64
}

// ------------------------------------------------------------ CSV flags

#[derive(Default, Clone)]
struct CsvInfo {
    prime_curves: u32,
    tian_curves: u32,
}

/// Round-2 `curves.csv`: per (q, t3): number of prime-order curves and how many have
/// a Tian kernel at some odd l; also cross-checks the per-row class and flag against
/// this file's own Theorem 4 implementation.
fn load_round2(path: &str) -> (BTreeMap<(u64, i64), CsvInfo>, (u64, u64, u64)) {
    let txt = std::fs::read_to_string(path).expect("round-2 curves.csv");
    // per curve key -> (tian any odd)
    let mut per_curve: BTreeMap<(u64, i64, String), (bool, bool)> = BTreeMap::new();
    let mut rows = 0u64;
    let mut agree = 0u64;
    let mut checked = 0u64;
    for (n, line) in txt.lines().enumerate() {
        if n == 0 {
            continue;
        }
        let f: Vec<&str> = line.split(',').collect();
        let q: u64 = f[0].parse().unwrap();
        let t: i64 = f[6].parse().unwrap();
        let ell: u64 = f[9].parse().unwrap();
        let class = f[12];
        let tian = f[16] == "1";
        let prime = f[8] == "prime";
        let key = (q, t, f[1..6].join(","));
        let e = per_curve.entry(key).or_insert((prime, false));
        if ell != 2 && tian {
            e.1 = true;
        }
        rows += 1;
        if ell != 2 {
            let mine = tian_flag(q, t, ell);
            let kind_ok = match (class, mine.kind) {
                ("nonsplit", "nonsplit") | ("split", "split") => true,
                ("scalar", "double") | ("jordan", "double") => true,
                _ => false,
            };
            let flag_ok = match (class, mine.flag) {
                ("nonsplit", Flag::No) => !tian,
                ("split", Flag::Yes) => tian,
                ("split", Flag::No) => !tian,
                ("scalar", _) => tian,
                ("jordan", Flag::Yes) => tian,
                ("jordan", Flag::No) => !tian,
                ("jordan", Flag::Ambiguous) => tian == (ell % 3 == 1),
                _ => false,
            };
            checked += 1;
            if kind_ok && flag_ok {
                agree += 1;
            }
        }
    }
    let mut out: BTreeMap<(u64, i64), CsvInfo> = BTreeMap::new();
    for (k, (prime, any)) in per_curve {
        if !prime {
            continue;
        }
        let e = out.entry((k.0, k.1)).or_default();
        e.prime_curves += 1;
        if any {
            e.tian_curves += 1;
        }
    }
    (out, (rows, checked, agree))
}

// ---------------------------------------------------------------- commands

fn sha_note() -> String {
    String::new()
}

fn cmd_list(out: &str, csv: &str) {
    std::fs::create_dir_all(out).unwrap();
    let (info, (rows, checked, agree)) = load_round2(csv);
    let mut s = String::from(
        "q,t3,numE,prime,constant_type,l3,l5,l7,own_class,round2_prime_curves,round2_tian_curves,round2_status\n",
    );
    let mut summ = String::new();
    writeln!(summ, "round-2 rows read: {rows}; odd-l rows cross-checked against own Theorem 4 code: {checked}; agree: {agree}").unwrap();
    for q in [7u64, 11, 13] {
        let h = hasse_bound(q);
        let mut n_prime = 0;
        let mut n_cert = 0;
        let mut n_pos = 0;
        let mut n_amb = 0;
        let mut n_none = 0;
        let mut conflict = 0;
        for t3 in -h..=h {
            let ne = (q * q * q + 1) as i64 - t3;
            let prime = is_prime(ne as u64);
            let mut cells = Vec::new();
            let mut any_yes = false;
            let mut any_amb = false;
            for ell in [3u64, 5, 7] {
                if ell == q {
                    cells.push("-".to_string());
                    continue;
                }
                let e = tian_flag(q, t3, ell);
                any_yes |= e.flag == Flag::Yes && !e.eig1;
                any_amb |= e.flag == Flag::Ambiguous && !e.eig1;
                cells.push(format!(
                    "{}{}:{:?}",
                    e.kind,
                    if e.eig1 { "(eig1)" } else { "" },
                    e.flag
                ));
            }
            let own = if any_yes {
                "kernel"
            } else if any_amb {
                "ambiguous"
            } else {
                "none"
            };
            let ci = info.get(&(q, t3)).cloned().unwrap_or_default();
            if realizable(q, t3) != (ci.prime_curves > 0) && prime {
                conflict += 1000; // realizability disagrees with round 2's curve list
            }
            let status = if ci.prime_curves == 0 {
                "na"
            } else if ci.tian_curves == 0 {
                "none"
            } else if ci.tian_curves == ci.prime_curves {
                "all"
            } else {
                "some"
            };
            if prime {
                n_prime += 1;
                // consistency of own flags with round 2
                let ok = !realizable(q, t3)
                    || match (own, status) {
                        ("kernel", "all") | ("kernel", "some") => true,
                        ("none", "none") => true,
                        ("ambiguous", _) => true,
                        _ => false,
                    };
                // "kernel" by own code must give at least the curves of that class: any >= 1
                if !ok {
                    conflict += 1;
                }
                match own {
                    "kernel" => n_cert += 1,
                    "ambiguous" => n_amb += 1,
                    _ => n_none += 1,
                }
                if status == "all" || status == "some" {
                    n_pos += 1;
                }
            }
            writeln!(
                s,
                "{q},{t3},{ne},{},{},{},{},{},{own},{},{},{status}",
                prime as u8,
                constant_type(q, t3) as u8,
                cells[0],
                cells[1],
                cells[2],
                ci.prime_curves,
                ci.tian_curves
            )
            .unwrap();
        }
        writeln!(
            summ,
            "q={q}: t3 in [-{h},{h}]; prime-order t3 values {n_prime}; own flag kernel/ambiguous/none = {n_cert}/{n_amb}/{n_none}; round-2 says some prime-order curve has a kernel for {n_pos} values; conflicts {conflict}"
        )
        .unwrap();
    }
    std::fs::write(format!("{out}/weil_table.csv"), s).unwrap();
    std::fs::write(format!("{out}/list_summary.txt"), &summ).unwrap();
    print!("{summ}");
    let _ = sha_note();
}

fn random_quartic(rng: &mut Rng, q: u64) -> Quartic {
    let mut c = [0u64; 15];
    for v in c.iter_mut() {
        *v = rng.below(q);
    }
    c
}

fn fmt_coefs(c: &Quartic) -> String {
    c.iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// PGL_3(F_q) order and the mass of F_q-isomorphism classes of smooth quartics.
fn class_mass(q: u64, smooth_fraction: f64) -> f64 {
    let qf = q as f64;
    let forms = (qf.powi(15) - 1.0) / (qf - 1.0);
    let pgl = qf.powi(3) * (qf * qf - 1.0) * (qf.powi(3) - 1.0);
    smooth_fraction * forms / pgl
}

fn cmd_search(out: &str, q: u64, n: u64, seed: u64, csv: Option<&str>) {
    std::fs::create_dir_all(out).unwrap();
    let f2 = Fld::new(q, 2);
    let f3 = Fld::new(q, 3);
    let tab = PointTable::new(q);
    let mut rng = Rng::new(seed);
    let mut srng = Rng::new(seed ^ 0xABCDEF);
    let mut pass1 = 0u64;
    let mut pass2 = 0u64;
    let mut uncert = 0u64;
    let mut smooth_hits = 0u64;
    let mut hist: BTreeMap<i64, (u64, Vec<String>)> = BTreeMap::new();
    let t0 = std::time::Instant::now();
    for _ in 0..n {
        let c = random_quartic(&mut rng, q);
        if tab.count(&c) != q + 1 {
            continue;
        }
        pass1 += 1;
        if count_points(&c, &f2) != q * q + 1 {
            continue;
        }
        pass2 += 1;
        match smooth_check(&f2, &c, &mut srng) {
            Smooth::Uncertified => {
                uncert += 1;
                continue;
            }
            Smooth::Smooth => {}
        }
        smooth_hits += 1;
        let n3 = count_points(&c, &f3);
        let base = (q * q * q + 1) as i64;
        let d = base - n3 as i64;
        assert!(d % 3 == 0, "N3 not congruent: {d}");
        let t3 = d / 3;
        let e = hist.entry(t3).or_insert((0, Vec::new()));
        e.0 += 1;
        if e.1.len() < 2 {
            e.1.push(fmt_coefs(&c));
        }
    }
    let secs = t0.elapsed().as_secs_f64();
    let info = csv.map(load_round2).map(|x| x.0);
    let mut s = String::from(
        "q,t3,numE,prime,constant_type,own_class,round2_status,realizable,samples,hits,hits_per_1e6,mass_estimate,example_coeffs\n",
    );
    let h = hasse_bound(q);
    let mass_total = class_mass(q, 1.0);
    for t3 in -h..=h {
        let ne = (q * q * q + 1) as i64 - t3;
        let prime = is_prime(ne as u64);
        let mut any_yes = false;
        let mut any_amb = false;
        for ell in [3u64, 5, 7] {
            if ell == q {
                continue;
            }
            let e = tian_flag(q, t3, ell);
            any_yes |= e.flag == Flag::Yes && !e.eig1;
            any_amb |= e.flag == Flag::Ambiguous && !e.eig1;
        }
        let own = if any_yes {
            "kernel"
        } else if any_amb {
            "ambiguous"
        } else {
            "none"
        };
        let status = match info.as_ref().and_then(|m| m.get(&(q, t3))) {
            None => "na",
            Some(ci) if ci.prime_curves == 0 => "na",
            Some(ci) if ci.tian_curves == 0 => "none",
            Some(ci) if ci.tian_curves == ci.prime_curves => "all",
            Some(_) => "some",
        };
        let (hits, ex) = hist.get(&t3).cloned().unwrap_or((0, Vec::new()));
        let rate = hits as f64 / n as f64 * 1e6;
        let mass = hits as f64 / n as f64 * mass_total;
        writeln!(
            s,
            "{q},{t3},{ne},{},{},{own},{status},{},{n},{hits},{rate:.4},{mass:.3},\"{}\"",
            prime as u8,
            constant_type(q, t3) as u8,
            realizable(q, t3) as u8,
            ex.join(" | ")
        )
        .unwrap();
    }
    std::fs::write(format!("{out}/census_q{q}.csv"), s).unwrap();
    let mut sm = String::new();
    writeln!(sm, "q={q} samples={n} seed={seed} secs={secs:.1}").unwrap();
    writeln!(sm, "N1==q+1: {pass1}  and N2==q^2+1: {pass2}  uncertified-smooth: {uncert}  smooth hits (c1=c2=0): {smooth_hits}").unwrap();
    writeln!(
        sm,
        "P(N1=q+1)={:.5} P(N2=q^2+1|N1)={:.5} P(c1=c2=0, smooth)={:.3e}",
        pass1 as f64 / n as f64,
        pass2 as f64 / pass1.max(1) as f64,
        smooth_hits as f64 / n as f64
    )
    .unwrap();
    writeln!(
        sm,
        "mass of smooth-quartic classes if every form were smooth: {mass_total:.1}"
    )
    .unwrap();
    std::fs::write(format!("{out}/search_summary_q{q}.txt"), &sm).unwrap();
    print!("{sm}");
}

fn cmd_smooth(out: &str, q: u64, n: u64, seed: u64) {
    std::fs::create_dir_all(out).unwrap();
    let f2 = Fld::new(q, 2);
    let mut rng = Rng::new(seed);
    let mut srng = Rng::new(seed ^ 0x1234);
    let mut sm = 0u64;
    let mut sm_first = 0u64;
    for _ in 0..n {
        let c = random_quartic(&mut rng, q);
        if smooth_once(&f2, &to_form(&c)) {
            sm_first += 1;
            sm += 1;
        } else if smooth_check(&f2, &c, &mut srng) == Smooth::Smooth {
            sm += 1;
        }
    }
    let frac = sm as f64 / n as f64;
    let mut s = String::new();
    writeln!(s, "q={q} samples={n} seed={seed}: certified smooth {sm} ({frac:.5}); on the first (identity) chart {sm_first}").unwrap();
    writeln!(s, "uncertified (singular or unlucky): {:.5}", 1.0 - frac).unwrap();
    writeln!(s, "mass of F_q-isomorphism classes of smooth quartics (sum of 1/|Aut|) ~ {:.1}  (q^6 = {:.1})", class_mass(q, frac), (q as f64).powi(6)).unwrap();
    std::fs::write(format!("{out}/smooth_q{q}.txt"), &s).unwrap();
    print!("{s}");
}

fn cmd_control(out: &str, q: u64, n: u64, seed: u64) {
    std::fs::create_dir_all(out).unwrap();
    let f1 = Fld::new(q, 1);
    let f2 = Fld::new(q, 2);
    let f3 = Fld::new(q, 3);
    let tab = PointTable::new(q);
    // construct C*: a fixed-seed smooth quartic; counts by the independent brute-force counter
    let mut crng = Rng::new(0xC0FFEE ^ q);
    let mut srng = Rng::new(7);
    let (cstar, tri) = loop {
        let c = random_quartic(&mut crng, q);
        if smooth_check(&f2, &c, &mut srng) != Smooth::Smooth {
            continue;
        }
        let n1 = count_points_brute(&c, &f1);
        let n2 = count_points_brute(&c, &f2);
        // q <= 13: brute force over P^2(F_{q^3}) is 4.8M points; fine for a control
        let n3 = count_points_brute(&c, &f3);
        break (c, (n1, n2, n3));
    };
    let mut s = String::new();
    writeln!(
        s,
        "q={q} control quartic C* (coefficient order x^i y^j z^k, i then j): {}",
        fmt_coefs(&cstar)
    )
    .unwrap();
    writeln!(s, "brute-force counts N1,N2,N3 = {:?}", tri).unwrap();
    // fast counters on C* must agree with brute force
    let fast = (
        tab.count(&cstar),
        count_points(&cstar, &f2),
        count_points(&cstar, &f3),
    );
    writeln!(
        s,
        "fast counters N1,N2,N3 = {:?}  agree: {}",
        fast,
        fast == tri
    )
    .unwrap();
    // search: random quartics, look for the same triple
    let mut rng = Rng::new(seed);
    let mut found = 0u64;
    let mut first = 0u64;
    let mut example = String::new();
    for i in 0..n {
        let c = random_quartic(&mut rng, q);
        if tab.count(&c) != tri.0 {
            continue;
        }
        if count_points(&c, &f2) != tri.1 {
            continue;
        }
        if smooth_check(&f2, &c, &mut srng) != Smooth::Smooth {
            continue;
        }
        if count_points(&c, &f3) != tri.2 {
            continue;
        }
        found += 1;
        if first == 0 {
            first = i + 1;
            example = fmt_coefs(&c);
        }
    }
    writeln!(
        s,
        "search over {n} samples (seed {seed}): hits {found}, first at sample {first}"
    )
    .unwrap();
    writeln!(s, "first hit: {example}").unwrap();
    std::fs::write(format!("{out}/control_q{q}.txt"), &s).unwrap();
    print!("{s}");
}

fn cmd_recount(out: &str, q: u64, seed: u64, keep_pct: u64) {
    // independent brute-force recount of the stored example quartics of census_q{q}.csv
    let f1 = Fld::new(q, 1);
    let f2 = Fld::new(q, 2);
    let f3 = Fld::new(q, 3);
    let txt = std::fs::read_to_string(format!("{out}/census_q{q}.csv")).expect("census csv");
    let mut rng = Rng::new(seed);
    let mut s = String::from("t3,coeffs,N1,N2,N3,expected_N3,ok\n");
    let mut total = 0;
    let mut ok = 0;
    for line in txt.lines().skip(1) {
        let a = line.find('"').unwrap();
        let b = line.rfind('"').unwrap();
        let head: Vec<&str> = line[..a].trim_end_matches(',').split(',').collect();
        let t3: i64 = head[1].parse().unwrap();
        let hits: u64 = head[9].parse().unwrap();
        if hits == 0 || rng.below(100) >= keep_pct {
            continue;
        }
        let exs: Vec<&str> = line[a + 1..b].split(" | ").collect();
        let pick = exs[rng.below(exs.len() as u64) as usize];
        let c: Vec<u64> = pick.split(' ').map(|v| v.parse().unwrap()).collect();
        let mut cq = [0u64; 15];
        cq.copy_from_slice(&c);
        let n1 = count_points_brute(&cq, &f1);
        let n2 = count_points_brute(&cq, &f2);
        let n3 = count_points_brute(&cq, &f3);
        let en3 = (q * q * q + 1) as i64 - 3 * t3;
        let good = n1 == q + 1 && n2 == q * q + 1 && n3 as i64 == en3;
        total += 1;
        ok += good as u32;
        writeln!(s, "{t3},\"{}\",{n1},{n2},{n3},{en3},{}", pick, good as u8).unwrap();
    }
    s.push_str(&format!(
        "# {ok}/{total} recounted examples agree with (q+1, q^2+1, q^3+1-3 t3)\n"
    ));
    std::fs::write(format!("{out}/recount_q{q}.csv"), &s).unwrap();
    println!("{ok}/{total} recounted examples agree");
}

fn boot_mean(v: &[f64], reps: usize, seed: u64) -> (f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let mut rng = Rng::new(seed);
    let mut m: Vec<f64> = (0..reps)
        .map(|_| {
            (0..v.len())
                .map(|_| v[rng.below(v.len() as u64) as usize])
                .sum::<f64>()
                / v.len() as f64
        })
        .collect();
    m.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (
        m[(reps as f64 * 0.025) as usize],
        m[(reps as f64 * 0.975) as usize],
    )
}

fn cmd_summarize(out: &str, qs: &[u64]) {
    let mut s = String::from(
        "q,group,n_t3,n_zero_hits,min_hits,median_hits,mean_hits,mean_ratio_to_local,ratio_lo95,ratio_hi95,min_ratio_to_local\n",
    );
    let mut detail = String::from("q,t3,group,hits,local_expected,ratio\n");
    for &q in qs {
        let txt = std::fs::read_to_string(format!("{out}/census_q{q}.csv")).expect("census");
        // (t3, prime, constant, status, realizable, hits)
        let mut rows: Vec<(i64, bool, bool, String, bool, f64)> = Vec::new();
        for line in txt.lines().skip(1) {
            let a = line.find('"').unwrap();
            let h: Vec<&str> = line[..a].trim_end_matches(',').split(',').collect();
            rows.push((
                h[1].parse().unwrap(),
                h[3] == "1",
                h[4] == "1",
                // round 2 has no curves for q > 13: fall back to this file's own flag
                match (h[6], h[5]) {
                    ("na", "kernel") if h[7] == "1" && q > 13 => "all".to_string(),
                    ("na", "none") if h[7] == "1" && q > 13 => "none".to_string(),
                    (st, _) => st.to_string(),
                },
                h[7] == "1",
                h[9].parse().unwrap(),
            ));
        }
        let mut groups: BTreeMap<&str, Vec<(f64, f64)>> = BTreeMap::new();
        for r in &rows {
            if !r.4 || r.2 {
                continue;
            }
            // local expectation: mean hits of realizable, non-constant t3 within 6, excluding itself
            let mut sum = 0.0;
            let mut cnt = 0.0;
            for o in &rows {
                if o.4 && !o.2 && o.0 != r.0 && (o.0 - r.0).abs() <= 6 {
                    sum += o.5;
                    cnt += 1.0;
                }
            }
            let loc = if cnt > 0.0 { sum / cnt } else { f64::NAN };
            let g = if r.1 && (r.3 == "all" || r.3 == "some") {
                "prime_with_kernel"
            } else if r.1 && r.3 == "none" {
                "prime_no_kernel"
            } else if r.1 {
                "prime_other"
            } else {
                "composite"
            };
            groups.entry(g).or_default().push((r.5, loc));
            writeln!(detail, "{q},{},{g},{},{loc:.2},{:.3}", r.0, r.5, r.5 / loc).unwrap();
        }
        for (g, v) in &groups {
            let n = v.len();
            let mut hs: Vec<f64> = v.iter().map(|x| x.0).collect();
            hs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let zero = hs.iter().filter(|&&h| h == 0.0).count();
            let mean = hs.iter().sum::<f64>() / n as f64;
            let ratios: Vec<f64> = v.iter().filter(|x| x.1 > 0.0).map(|x| x.0 / x.1).collect();
            let mr = ratios.iter().sum::<f64>() / ratios.len().max(1) as f64;
            let minr = ratios.iter().cloned().fold(f64::INFINITY, f64::min);
            let (lo, hi) = boot_mean(&ratios, 4000, 17 + q);
            writeln!(
                s,
                "{q},{g},{n},{zero},{},{},{mean:.2},{mr:.3},{lo:.3},{hi:.3},{minr:.3}",
                hs[0],
                hs[n / 2]
            )
            .unwrap();
        }
        // primary contrast H2: prime t3 with kernel against prime t3 without, ratio of mean ratios
        if let (Some(p), Some(nn)) = (
            groups.get("prime_with_kernel"),
            groups.get("prime_no_kernel"),
        ) {
            let rp: Vec<f64> = p.iter().filter(|x| x.1 > 0.0).map(|x| x.0 / x.1).collect();
            let rn: Vec<f64> = nn.iter().filter(|x| x.1 > 0.0).map(|x| x.0 / x.1).collect();
            let mut rng = Rng::new(99 + q);
            let mut v = Vec::new();
            for _ in 0..4000 {
                let a: f64 = (0..rp.len())
                    .map(|_| rp[rng.below(rp.len() as u64) as usize])
                    .sum::<f64>()
                    / rp.len() as f64;
                let b: f64 = (0..rn.len())
                    .map(|_| rn[rng.below(rn.len() as u64) as usize])
                    .sum::<f64>()
                    / rn.len() as f64;
                v.push(a / b);
            }
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mp = rp.iter().sum::<f64>() / rp.len() as f64;
            let mn = rn.iter().sum::<f64>() / rn.len() as f64;
            writeln!(
                s,
                "{q},H2_ratio_with_over_without,{},{},,,,{:.3},{:.3},{:.3},",
                rp.len(),
                rn.len(),
                mp / mn,
                v[(v.len() as f64 * 0.025) as usize],
                v[(v.len() as f64 * 0.975) as usize]
            )
            .unwrap();
        }
    }
    std::fs::write(format!("{out}/summary_groups.csv"), &s).unwrap();
    std::fs::write(format!("{out}/summary_detail.csv"), &detail).unwrap();
    print!("{s}");
}

fn read_hits(path: &str) -> BTreeMap<i64, f64> {
    let txt = std::fs::read_to_string(path).expect("census csv");
    let mut m = BTreeMap::new();
    for line in txt.lines().skip(1) {
        let a = line.find('"').unwrap();
        let h: Vec<&str> = line[..a].trim_end_matches(',').split(',').collect();
        if h[7] == "1" && h[4] == "0" {
            m.insert(h[1].parse().unwrap(), h[9].parse().unwrap());
        }
    }
    m
}

/// A/A control: chi-square of two independent searches with the same instrument.
fn cmd_aa(out: &str, c1: &str, c2: &str) {
    let a = read_hits(c1);
    let b = read_hits(c2);
    let mut chi = 0.0;
    let mut dof = 0;
    let mut tot1 = 0.0;
    let mut tot2 = 0.0;
    for (t, h1) in &a {
        let h2 = b[t];
        tot1 += h1;
        tot2 += h2;
        if h1 + h2 > 0.0 {
            chi += (h1 - h2) * (h1 - h2) / (h1 + h2);
            dof += 1;
        }
    }
    // normal approximation of the chi-square tail: z = (chi - dof) / sqrt(2 dof)
    let z = (chi - dof as f64) / (2.0 * dof as f64).sqrt();
    let s =
        format!("A/A: t3 cells {dof}, total hits {tot1} vs {tot2}, chi2 = {chi:.2}, z = {z:.2}\n");
    std::fs::create_dir_all(out).unwrap();
    std::fs::write(format!("{out}/aa_check.txt"), &s).unwrap();
    print!("{s}");
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 3 {
        eprintln!("usage: tian_quartic_census <list|search|smooth|control|recount> <out-dir> ...");
        std::process::exit(2);
    }
    let p = |i: usize| -> u64 { a[i].parse().expect("number") };
    match a[1].as_str() {
        "list" => cmd_list(&a[2], &a[3]),
        "search" => cmd_search(&a[2], p(3), p(4), p(5), a.get(6).map(|s| s.as_str())),
        "smooth" => cmd_smooth(&a[2], p(3), p(4), p(5)),
        "control" => cmd_control(&a[2], p(3), p(4), p(5)),
        "recount" => cmd_recount(&a[2], p(3), p(4), p(5)),
        "aa" => cmd_aa(&a[2], &a[3], &a[4]),
        "summarize" => {
            let qs: Vec<u64> = a[3..].iter().map(|v| v.parse().unwrap()).collect();
            cmd_summarize(&a[2], &qs)
        }
        _ => {
            eprintln!("unknown command");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quartic_from(terms: &[(usize, usize, u64)]) -> Quartic {
        let mut c = [0u64; 15];
        for &(i, j, v) in terms {
            c[midx(i, j)] = v;
        }
        c
    }

    #[test]
    fn field_inverse() {
        for (q, k) in [(7u64, 1usize), (7, 2), (7, 3), (11, 3), (13, 3)] {
            let f = Fld::new(q, k);
            let mut rng = Rng::new(5);
            for _ in 0..50 {
                let a = f.from_index(rng.below(f.size - 1) + 1);
                let b = f.inv(&a);
                assert_eq!(f.mul(&a, &b), f.one());
            }
            // x^(Q) = x for all x
            let a = f.from_index(f.size - 1);
            assert_eq!(f.pow(&a, f.size), a);
        }
    }

    #[test]
    fn counters_agree_with_brute_force() {
        for q in [5u64, 7] {
            let tab = PointTable::new(q);
            let mut rng = Rng::new(11);
            for k in 1..=3usize {
                let f = Fld::new(q, k);
                for _ in 0..4 {
                    let c = random_quartic(&mut rng, q);
                    let b = count_points_brute(&c, &f);
                    assert_eq!(count_points(&c, &f), b, "q={q} k={k}");
                    if k == 1 {
                        assert_eq!(tab.count(&c), b);
                    }
                }
            }
        }
    }

    #[test]
    fn fermat_quartic_counts() {
        // x^4 + y^4 + z^4 over F_5: x^4 in {0,1}; solutions of a+b+c = 0 with a,b,c in {0,1} mod 5
        // (x,y,z) all non-zero would need 3 = 0; two non-zero: 2 = 0; one: 1 = 0: only (0,0,0).
        let c = quartic_from(&[(4, 0, 1), (0, 4, 1), (0, 0, 1)]);
        let f = Fld::new(5, 1);
        // points: need number of nonzero among x,y,z with sum of 1's = 0 mod 5 -> none over F_5
        assert_eq!(count_points_brute(&c, &f), 0);
        assert_eq!(count_points(&c, &f), 0);
    }

    #[test]
    fn smoothness_known_cases() {
        let q = 13u64;
        let f2 = Fld::new(q, 2);
        let mut rng = Rng::new(3);
        // Klein quartic x^3 y + y^3 z + z^3 x: smooth in characteristic != 7
        let klein = quartic_from(&[(3, 1, 1), (0, 3, 1), (1, 0, 1)]);
        assert_eq!(smooth_check(&f2, &klein, &mut rng), Smooth::Smooth);
        // four concurrent lines x y (x+y)(x-y) = x^3 y - x y^3 : singular
        let lines = quartic_from(&[(3, 1, 1), (1, 3, q - 1)]);
        assert_eq!(smooth_check(&f2, &lines, &mut rng), Smooth::Uncertified);
        // nodal quartic y^2 z^2 = x^3 z + x^4 -> y^2 z^2 - x^3 z - x^4
        let nodal = quartic_from(&[(0, 2, 1), (3, 0, q - 1), (4, 0, q - 1)]);
        assert_eq!(smooth_check(&f2, &nodal, &mut rng), Smooth::Uncertified);
        // conic times conic (reducible): (x^2 + y^2 + z^2)(x^2 + 2 y^2 + 3 z^2)
        let cc = quartic_from(&[
            (4, 0, 1),
            (2, 2, 2),
            (0, 4, 2),
            (2, 0, 4), // x^2 z^2 : 3 + 1 = 4
            (0, 2, 5), // y^2 z^2 : 3 + 2 = 5
            (0, 0, 3),
            (2, 2, 0),
        ]);
        let _ = cc;
        // Fermat x^4+y^4+z^4 smooth for p != 2
        let fer = quartic_from(&[(4, 0, 1), (0, 4, 1), (0, 0, 1)]);
        assert_eq!(smooth_check(&f2, &fer, &mut rng), Smooth::Smooth);
    }

    /// Singular points (all partials vanish) of a quartic over P^2(F_{q^k}), by brute force.
    fn has_singular_point(c: &Quartic, f: &Fld) -> bool {
        let g = partials(f.q, &to_form(c));
        let mut pts: Vec<[El; 3]> = Vec::new();
        for ny in 0..f.size {
            for nx in 0..f.size {
                pts.push([f.from_index(nx), f.from_index(ny), f.one()]);
            }
        }
        for nx in 0..f.size {
            pts.push([f.from_index(nx), f.one(), f.zero()]);
        }
        pts.push([f.one(), f.zero(), f.zero()]);
        for p in &pts {
            let mut all_zero = true;
            for gg in &g {
                let mut s = f.zero();
                for i in 0..4 {
                    for j in 0..(4 - i) {
                        if gg[i][j] == 0 {
                            continue;
                        }
                        let mut t = f.embed(gg[i][j]);
                        t = f.mul(&t, &f.pow(&p[0], i as u64));
                        t = f.mul(&t, &f.pow(&p[1], j as u64));
                        t = f.mul(&t, &f.pow(&p[2], (3 - i - j) as u64));
                        s = f.add(&s, &t);
                    }
                }
                if !f.is_zero(&s) {
                    all_zero = false;
                    break;
                }
            }
            if all_zero {
                return true;
            }
        }
        false
    }

    #[test]
    fn smoothness_soundness_against_brute_force() {
        let q = 5u64;
        let f2 = Fld::new(q, 2);
        let fs = [Fld::new(q, 1), Fld::new(q, 2), Fld::new(q, 3)];
        let mut rng = Rng::new(21);
        let mut srng = Rng::new(22);
        let (mut smooth, mut unc_with_sing, mut unc_without) = (0, 0, 0);
        for _ in 0..120 {
            let c = random_quartic(&mut rng, q);
            let sing = fs.iter().any(|f| has_singular_point(&c, f));
            match smooth_check(&f2, &c, &mut srng) {
                Smooth::Smooth => {
                    assert!(!sing, "certified smooth but singular point found");
                    smooth += 1;
                }
                Smooth::Uncertified => {
                    if sing {
                        unc_with_sing += 1;
                    } else {
                        unc_without += 1;
                    }
                }
            }
        }
        // every curve with a singular point over F_5, F_25 or F_125 is rejected; the remainder
        // (uncertified without such a point) must be rare (singular points over larger fields)
        assert!(smooth > 80, "{smooth}");
        assert!(unc_with_sing > 5, "{unc_with_sing}");
        assert!(unc_without <= 6, "{unc_without}");
    }

    #[test]
    fn smooth_fraction_plausible() {
        // for q = 7 roughly 1 - 1/q of random forms are smooth (the discriminant is one equation)
        let q = 7u64;
        let f2 = Fld::new(q, 2);
        let mut rng = Rng::new(1);
        let mut srng = Rng::new(2);
        let mut s = 0;
        for _ in 0..300 {
            let c = random_quartic(&mut rng, q);
            if smooth_check(&f2, &c, &mut srng) == Smooth::Smooth {
                s += 1;
            }
        }
        assert!(s > 200 && s < 300, "{s}");
    }

    #[test]
    fn weil_polynomial_counts() {
        // for q = 7: the three counts of a curve with c1 = c2 = 0 satisfy N3 = q^3 + 1 - 3 t3,
        // so N3 = q^3 + 1 mod 3
        assert_eq!(hasse_bound(7), 37);
        // t3 = s^3 - 3 q s with s = 0 -> 0 ; s = 1 -> 1 - 21 = -20
        assert!(constant_type(7, 0));
        assert!(constant_type(7, -20));
        assert!(!constant_type(7, 1));
    }

    #[test]
    fn theorem4_cells() {
        // q = 7, l = 3: phi = 2I on E[3] <=> t3 = 2*2 = 4 mod 3 = 1 and 7^3 = 1 mod 3 -> det 1 -> lambda^2 = 1
        let e = tian_flag(7, 1, 3);
        assert_eq!(e.kind, "double");
        // l = 5, q = 7: q^3 = 343 = 3 mod 5; t3 = 1 mod 5 -> roots of T^2 - T + 3 : (T = 2: 4 - 2 + 3 = 5 ok; T = 4: 16 - 4 + 3 = 15 ok)
        let e = tian_flag(7, 1, 5);
        assert_eq!((e.kind, e.flag), ("split", Flag::Yes));
        // nonsplit: l = 5, t3 = 0, q = 7: T^2 + 3 mod 5 has roots? -3 = 2, not a square mod 5
        let e = tian_flag(7, 0, 5);
        assert_eq!((e.kind, e.flag), ("nonsplit", Flag::No));
        // l = 7 split with non-cube eigenvalue: q = 11 (q mod 7 = 4), T^2 - t T + 11^3 with roots 2, 3 (det 6 = 4^3 mod 7 = 1? 64 mod 7 = 1)
        // choose roots 2 and 4: product 8 = 1 = q^3 mod 7 ok, sum 6 -> t3 = 6; neither is a cube mod 7 (cubes are 1, 6)
        let e = tian_flag(11, 6, 7);
        assert_eq!((e.kind, e.flag), ("split", Flag::No));
    }
}
