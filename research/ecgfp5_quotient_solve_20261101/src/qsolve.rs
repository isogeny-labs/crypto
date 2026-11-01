//! EcGFp5-shaped point-decomposition systems for msolve: the raw, S_m-symmetrized and
//! F_p-rational (Z/2)^{m-1} x| S_m quotient arms of KN-OPEN-9b4a2b, built by exact
//! evaluation and interpolation, plus a planted-solution checker and a resource-measuring
//! runner. Standalone: `rustc -O --edition 2021 qsolve.rs` (no crates).
//!
//! Curve: y^2 = x (x^2 + 2x + b) over F_q = F_p[z]/(z^m - c), b = b0*z with b and 4 - 4b
//! non-squares in F_q (the EcGFp5 shape: EcGFp5 itself is b = 263z, m = 5,
//! p = 2^64 - 2^32 + 1, z^5 - 3; Pornin, ePrint 2022/274).
//!
//! Semaev's polynomials by the recursive resultant definition with formal degrees:
//!   S_3(u,v,X) closed form; S_4(x1,x2,x3,X) = Res_V(S_3(x1,x2,V), S_3(x3,X,V));
//!   S_6(x1..x5,X) = Res_Y(S_4(x1,x2,x3,Y), S_4(x4,x5,X,Y)).
//! The target abscissa X0 = x(R) is fixed; F(x) = S_{m+1}(x1..xm, X0) has degree 2^{m-1} in
//! each x_i.
//!
//! Arms (all for the same target, so the planted decomposition is a solution of each u-arm):
//!   raw_x   unknowns x_i in F_p                      F(x) descended            (m = 3 only)
//!   sym_x   unknowns e_k(x)                          G(e), deg <= 2^{m-1}       (m = 3 only)
//!   raw_u   unknowns u_i in F_p, x = lam*u           F(lam u) descended         (m = 3 only)
//!   sym_u   unknowns e_k(u)                          G(e)                       (m = 3 only)
//!   quot    unknowns s_k = e_k(t), t_i = u_i + beta/u_i, and w = prod(u_i - beta/u_i):
//!           L(u) = F(lam u)/prod u_i^h = A(s) + w B(s),  h = 2^{m-2}, deg A <= h, deg B <= h-1,
//!           and w^2 = Q(s) = prod(t_i^2 - 4 beta), deg Q <= 2.
//!           b = lam^2 beta with beta a non-square of F_p (possible because m is odd).
//! Every interpolated polynomial is verified at fresh random points before it is written.
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::time::{Duration, Instant};

// ---------------------------------------------------------------- F_p
fn pw(mut b: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64 % p;
    b %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % p;
        }
        b = b * b % p;
        e >>= 1;
    }
    r
}
fn inv_p(a: u64, p: u64) -> u64 {
    assert!(a % p != 0, "inverse of zero in F_p");
    pw(a, p - 2, p)
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

// ---------------------------------------------------------------- RNG
struct Rng(u64);
impl Rng {
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

// ---------------------------------------------------------------- F_q = F_p[z]/(z^m - c)
type El = [u64; 5];
const ZERO: El = [0; 5];

struct Fq {
    m: usize,
    p: u64,
    c: u64,
    q: u128,
    frob: Vec<El>, // frob[i][k] = omega^{i k}, omega = c^{(p-1)/m}
}

impl Fq {
    fn new(m: usize, p: u64) -> Fq {
        assert!(m == 3 || m == 5, "m must be 3 or 5 (odd prime degree)");
        assert!(is_prime(p) && p < (1 << 25), "p must be a prime below 2^25");
        assert!(p % (m as u64) == 1, "p must be 1 mod m");
        let e = (p - 1) / m as u64;
        let c = (2..p).find(|&c| pw(c, e, p) != 1).unwrap(); // z^m - c irreducible iff c not an m-th power
        let mut q: u128 = 1;
        for _ in 0..m {
            q = q.checked_mul(p as u128).expect("p^m overflows u128");
        }
        let omega = pw(c, e, p);
        let mut frob = vec![ZERO; m];
        for i in 0..m {
            for k in 0..m {
                frob[i][k] = pw(omega, (i * k) as u64, p);
            }
        }
        Fq { m, p, c, q, frob }
    }
    fn from(&self, a: u64) -> El {
        let mut r = ZERO;
        r[0] = a % self.p;
        r
    }
    fn z(&self) -> El {
        let mut r = ZERO;
        r[1] = 1;
        r
    }
    fn is_zero(&self, a: &El) -> bool {
        a[..self.m].iter().all(|&x| x == 0)
    }
    fn eq(&self, a: &El, b: &El) -> bool {
        a[..self.m] == b[..self.m]
    }
    fn add(&self, a: &El, b: &El) -> El {
        let mut r = ZERO;
        for k in 0..self.m {
            let s = a[k] + b[k];
            r[k] = if s >= self.p { s - self.p } else { s };
        }
        r
    }
    fn sub(&self, a: &El, b: &El) -> El {
        let mut r = ZERO;
        for k in 0..self.m {
            r[k] = if a[k] >= b[k] { a[k] - b[k] } else { a[k] + self.p - b[k] };
        }
        r
    }
    fn neg(&self, a: &El) -> El {
        self.sub(&ZERO, a)
    }
    fn mul(&self, a: &El, b: &El) -> El {
        let m = self.m;
        let p = self.p;
        let mut t = [0u64; 9];
        for i in 0..m {
            if a[i] == 0 {
                continue;
            }
            for j in 0..m {
                t[i + j] += a[i] * b[j]; // < 5 * 2^50
            }
        }
        let mut r = ZERO;
        for k in 0..m {
            let hi = if k + m < 2 * m - 1 { t[k + m] % p } else { 0 };
            r[k] = (t[k] % p + hi * self.c) % p;
        }
        r
    }
    fn scal(&self, a: &El, s: u64) -> El {
        let mut r = ZERO;
        for k in 0..self.m {
            r[k] = a[k] * (s % self.p) % self.p;
        }
        r
    }
    fn frob_i(&self, a: &El, i: usize) -> El {
        let mut r = ZERO;
        for k in 0..self.m {
            r[k] = a[k] * self.frob[i % self.m][k] % self.p;
        }
        r
    }
    fn norm_and_cofactor(&self, a: &El) -> (u64, El) {
        let mut prod = self.from(1);
        for i in 1..self.m {
            prod = self.mul(&prod, &self.frob_i(a, i));
        }
        let n = self.mul(a, &prod);
        debug_assert!(n[1..self.m].iter().all(|&x| x == 0));
        (n[0], prod)
    }
    fn inv(&self, a: &El) -> El {
        let (n, prod) = self.norm_and_cofactor(a);
        assert!(n != 0, "inverse of zero in F_q");
        self.scal(&prod, inv_p(n, self.p))
    }
    fn pow(&self, a: &El, mut e: u128) -> El {
        let mut r = self.from(1);
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
    fn is_square(&self, a: &El) -> bool {
        if self.is_zero(a) {
            return true;
        }
        let (n, _) = self.norm_and_cofactor(a); // chi_q(a) = chi_p(N(a))
        pw(n, (self.p - 1) / 2, self.p) == 1
    }
    fn sqrt(&self, a: &El) -> Option<El> {
        if self.is_zero(a) {
            return Some(ZERO);
        }
        if !self.is_square(a) {
            return None;
        }
        let mut s = 0u32;
        let mut t = self.q - 1;
        while t % 2 == 0 {
            t /= 2;
            s += 1;
        }
        let mut k = 1u64;
        let nonres = loop {
            let mut cand = self.z();
            cand[0] = k % self.p;
            if !self.is_square(&cand) {
                break cand;
            }
            k += 1;
        };
        let one = self.from(1);
        let mut mm = s;
        let mut cc = self.pow(&nonres, t);
        let mut tt = self.pow(a, t);
        let mut r = self.pow(a, (t + 1) / 2);
        loop {
            if self.eq(&tt, &one) {
                assert!(self.eq(&self.mul(&r, &r), a));
                return Some(r);
            }
            let mut i = 0u32;
            let mut x = tt;
            while !self.eq(&x, &one) {
                x = self.mul(&x, &x);
                i += 1;
            }
            let mut b = cc;
            for _ in 0..(mm - i - 1) {
                b = self.mul(&b, &b);
            }
            mm = i;
            cc = self.mul(&b, &b);
            tt = self.mul(&tt, &cc);
            r = self.mul(&r, &b);
        }
    }
    fn random(&self, rng: &mut Rng) -> El {
        let mut r = ZERO;
        for k in 0..self.m {
            r[k] = rng.below(self.p);
        }
        r
    }
    fn random_nonzero(&self, rng: &mut Rng) -> El {
        loop {
            let r = self.random(rng);
            if !self.is_zero(&r) {
                return r;
            }
        }
    }
    fn fmt(&self, a: &El) -> String {
        let v: Vec<String> = a[..self.m].iter().map(|x| x.to_string()).collect();
        format!("[{}]", v.join(","))
    }
}

// ---------------------------------------------------------------- curve
struct Curve {
    a2: El,
    a4: El,
    a6: El,
}
type Pt = Option<(El, El)>;

fn rhs(f: &Fq, cv: &Curve, x: &El) -> El {
    let x2 = f.mul(x, x);
    let x3 = f.mul(&x2, x);
    f.add(&f.add(&x3, &f.mul(&cv.a2, &x2)), &f.add(&f.mul(&cv.a4, x), &cv.a6))
}
fn ec_add(f: &Fq, cv: &Curve, p1: &Pt, p2: &Pt) -> Pt {
    match (p1, p2) {
        (None, _) => *p2,
        (_, None) => *p1,
        (Some((x1, y1)), Some((x2, y2))) => {
            let lam = if f.eq(x1, x2) {
                if f.is_zero(&f.add(y1, y2)) {
                    return None;
                }
                let num = f.add(&f.add(&f.scal(&f.mul(x1, x1), 3), &f.scal(&f.mul(&cv.a2, x1), 2)), &cv.a4);
                f.mul(&num, &f.inv(&f.scal(y1, 2)))
            } else {
                f.mul(&f.sub(y2, y1), &f.inv(&f.sub(x2, x1)))
            };
            let x3 = f.sub(&f.sub(&f.sub(&f.mul(&lam, &lam), &cv.a2), x1), x2);
            let y3 = f.sub(&f.mul(&lam, &f.sub(x1, &x3)), y1);
            Some((x3, y3))
        }
    }
}
fn random_point(f: &Fq, cv: &Curve, rng: &mut Rng) -> (El, El) {
    loop {
        let x = f.random(rng);
        let r = rhs(f, cv, &x);
        if f.is_zero(&r) {
            continue;
        }
        if let Some(y) = f.sqrt(&r) {
            return (x, y);
        }
    }
}

// ---------------------------------------------------------------- Semaev
/// S_3(u, v, W) as coefficients [c0, c1, c2] in W.
fn s3(f: &Fq, cv: &Curve, u: &El, v: &El) -> [El; 3] {
    let fu = rhs(f, cv, u);
    let fv = rhs(f, cv, v);
    let d = f.sub(u, v);
    let d2 = f.mul(&d, &d);
    let uv = f.add(u, v);
    let g = f.add(&f.add(&f.add(&f.mul(u, u), &f.mul(u, v)), &f.mul(v, v)), &f.add(&f.mul(&cv.a2, &uv), &cv.a4));
    let cc = f.add(&cv.a2, &uv);
    let ff = f.add(&fu, &fv);
    let c2 = d2;
    let c1 = f.neg(&f.sub(&f.scal(&ff, 2), &f.scal(&f.mul(&cc, &d2), 2)));
    let c0 = f.add(&f.sub(&f.mul(&g, &g), &f.scal(&f.mul(&cc, &ff), 2)), &f.mul(&f.mul(&cc, &cc), &d2));
    [c0, c1, c2]
}
fn res_quad(f: &Fq, a: &[El; 3], b: &[El; 3]) -> El {
    // Res(a2 V^2 + a1 V + a0, b2 V^2 + b1 V + b0) = (a2b0 - a0b2)^2 - (a2b1 - a1b2)(a1b0 - a0b1)
    let t1 = f.sub(&f.mul(&a[2], &b[0]), &f.mul(&a[0], &b[2]));
    let t2 = f.sub(&f.mul(&a[2], &b[1]), &f.mul(&a[1], &b[2]));
    let t3 = f.sub(&f.mul(&a[1], &b[0]), &f.mul(&a[0], &b[1]));
    f.sub(&f.mul(&t1, &t1), &f.mul(&t2, &t3))
}
/// S_4(a, b, c, Y) as coefficients (low to high, formal degree 4) in Y, by exact interpolation
/// at Y = 0..4 (S_4 has degree 4 in each variable).
fn s4_poly(f: &Fq, cv: &Curve, vinv: &[[u64; 5]; 5], a: &El, b: &El, c: &El) -> [El; 5] {
    let ab = s3(f, cv, a, b);
    let mut vals = [ZERO; 5];
    for (j, val) in vals.iter_mut().enumerate() {
        let y = f.from(j as u64);
        *val = res_quad(f, &ab, &s3(f, cv, c, &y));
    }
    let mut co = [ZERO; 5];
    for (i, coi) in co.iter_mut().enumerate() {
        let mut s = ZERO;
        for j in 0..5 {
            s = f.add(&s, &f.scal(&vals[j], vinv[i][j]));
        }
        *coi = s;
    }
    co
}
fn det(f: &Fq, mut a: Vec<Vec<El>>) -> El {
    let n = a.len();
    let mut d = f.from(1);
    for col in 0..n {
        let piv = (col..n).find(|&r| !f.is_zero(&a[r][col]));
        let piv = match piv {
            None => return ZERO,
            Some(r) => r,
        };
        if piv != col {
            a.swap(piv, col);
            d = f.neg(&d);
        }
        d = f.mul(&d, &a[col][col]);
        let iv = f.inv(&a[col][col]);
        for r in col + 1..n {
            if f.is_zero(&a[r][col]) {
                continue;
            }
            let fac = f.mul(&a[r][col], &iv);
            for k in col..n {
                let t = f.mul(&fac, &a[col][k]);
                a[r][k] = f.sub(&a[r][k], &t);
            }
        }
    }
    d
}
/// Sylvester resultant of two polynomials given low-to-high with formal degrees len-1.
fn sylvester(f: &Fq, p: &[El], q: &[El]) -> El {
    let dp = p.len() - 1;
    let dq = q.len() - 1;
    let n = dp + dq;
    let mut m = vec![vec![ZERO; n]; n];
    for i in 0..dq {
        for (k, c) in p.iter().rev().enumerate() {
            m[i][i + k] = *c;
        }
    }
    for i in 0..dp {
        for (k, c) in q.iter().rev().enumerate() {
            m[dq + i][i + k] = *c;
        }
    }
    det(f, m)
}
struct Semaev<'a> {
    f: &'a Fq,
    cv: &'a Curve,
    vinv: [[u64; 5]; 5],
}
impl<'a> Semaev<'a> {
    fn new(f: &'a Fq, cv: &'a Curve) -> Semaev<'a> {
        // inverse Vandermonde on nodes 0..4 over F_p
        let p = f.p;
        let mut a = [[0u64; 10]; 5];
        for i in 0..5 {
            for k in 0..5 {
                a[i][k] = pw(i as u64, k as u64, p);
            }
            a[i][5 + i] = 1;
        }
        for col in 0..5 {
            let piv = (col..5).find(|&r| a[r][col] != 0).unwrap();
            a.swap(piv, col);
            let iv = inv_p(a[col][col], p);
            for k in 0..10 {
                a[col][k] = a[col][k] * iv % p;
            }
            for r in 0..5 {
                if r != col && a[r][col] != 0 {
                    let fac = a[r][col];
                    for k in 0..10 {
                        a[r][k] = (a[r][k] + p - fac * a[col][k] % p) % p;
                    }
                }
            }
        }
        // a[.][5..] = V^{-1} with V[i][k] = i^k, so coefficient k = sum_j V^{-1}[k][j] val_j
        let mut vinv = [[0u64; 5]; 5];
        for k in 0..5 {
            for j in 0..5 {
                vinv[k][j] = a[k][5 + j];
            }
        }
        Semaev { f, cv, vinv }
    }
    /// S_{m+1}(xs, x0)
    fn eval(&self, xs: &[El], x0: &El) -> El {
        let f = self.f;
        match xs.len() {
            2 => {
                let c = s3(f, self.cv, &xs[0], &xs[1]);
                f.add(&f.add(&c[0], &f.mul(&c[1], x0)), &f.mul(&c[2], &f.mul(x0, x0)))
            }
            3 => res_quad(f, &s3(f, self.cv, &xs[0], &xs[1]), &s3(f, self.cv, &xs[2], x0)),
            5 => {
                let p1 = s4_poly(f, self.cv, &self.vinv, &xs[0], &xs[1], &xs[2]);
                let p2 = s4_poly(f, self.cv, &self.vinv, &xs[3], &xs[4], x0);
                sylvester(f, &p1, &p2)
            }
            _ => panic!("unsupported arity"),
        }
    }
}

// ---------------------------------------------------------------- multivariate interpolation
fn monos_total(nv: usize, d: usize) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    fn rec(nv: usize, left: usize, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        if cur.len() == nv {
            out.push(cur.clone());
            return;
        }
        for e in 0..=left {
            cur.push(e as u8);
            rec(nv, left - e, cur, out);
            cur.pop();
        }
    }
    rec(nv, d, &mut Vec::new(), &mut out);
    out.sort_by(|a, b| {
        let da: u32 = a.iter().map(|&x| x as u32).sum();
        let db: u32 = b.iter().map(|&x| x as u32).sum();
        db.cmp(&da).then(b.cmp(a))
    });
    out
}
fn monos_box(nv: usize, d: usize) -> Vec<Vec<u8>> {
    let mut out = vec![vec![]];
    for _ in 0..nv {
        let mut nxt = Vec::new();
        for v in &out {
            for e in 0..=d {
                let mut w = v.clone();
                w.push(e as u8);
                nxt.push(w);
            }
        }
        out = nxt;
    }
    out
}
fn eval_monos(f: &Fq, pt: &[El], monos: &[Vec<u8>]) -> Vec<El> {
    let maxd = monos.iter().flat_map(|m| m.iter()).copied().max().unwrap_or(0) as usize;
    let pows: Vec<Vec<El>> = pt
        .iter()
        .map(|x| {
            let mut v = vec![f.from(1)];
            for k in 1..=maxd {
                let t = f.mul(&v[k - 1], x);
                v.push(t);
            }
            v
        })
        .collect();
    monos
        .iter()
        .map(|mo| {
            let mut r = f.from(1);
            for (i, &e) in mo.iter().enumerate() {
                if e > 0 {
                    r = f.mul(&r, &pows[i][e as usize]);
                }
            }
            r
        })
        .collect()
}
fn solve_linear(f: &Fq, mut a: Vec<Vec<El>>, mut b: Vec<El>) -> Option<Vec<El>> {
    let n = a.len();
    for col in 0..n {
        let piv = (col..n).find(|&r| !f.is_zero(&a[r][col]))?;
        a.swap(piv, col);
        b.swap(piv, col);
        let iv = f.inv(&a[col][col]);
        for k in col..n {
            a[col][k] = f.mul(&a[col][k], &iv);
        }
        b[col] = f.mul(&b[col], &iv);
        let (top, bot) = a.split_at_mut(col + 1);
        let prow = &top[col];
        for (ri, row) in bot.iter_mut().enumerate() {
            let r = col + 1 + ri;
            if f.is_zero(&row[col]) {
                continue;
            }
            let fac = row[col];
            for k in col..n {
                let t = f.mul(&fac, &prow[k]);
                row[k] = f.sub(&row[k], &t);
            }
            let t = f.mul(&fac, &b[col]);
            b[r] = f.sub(&b[r], &t);
        }
    }
    for col in (0..n).rev() {
        for r in 0..col {
            if !f.is_zero(&a[r][col]) {
                let t = f.mul(&a[r][col], &b[col]);
                b[r] = f.sub(&b[r], &t);
            }
        }
    }
    Some(b)
}
/// Interpolate a polynomial with the given support from samples (point, value); the first
/// monos.len() samples determine it, the rest verify it.
fn interpolate(f: &Fq, monos: &[Vec<u8>], samples: &[(Vec<El>, El)], label: &str) -> Vec<El> {
    let n = monos.len();
    assert!(samples.len() >= n + 16, "{label}: not enough samples");
    let t0 = Instant::now();
    let a: Vec<Vec<El>> = samples[..n].iter().map(|(pt, _)| eval_monos(f, pt, monos)).collect();
    let b: Vec<El> = samples[..n].iter().map(|(_, v)| *v).collect();
    let co = solve_linear(f, a, b).unwrap_or_else(|| panic!("{label}: singular interpolation matrix"));
    for (pt, v) in &samples[n..] {
        let row = eval_monos(f, pt, monos);
        let mut s = ZERO;
        for (c, r) in co.iter().zip(row.iter()) {
            s = f.add(&s, &f.mul(c, r));
        }
        assert!(f.eq(&s, v), "{label}: interpolant fails at a verification point (support too small?)");
    }
    eprintln!("  interpolated {label}: {n} monomials, {} verification points, {:.1} s", samples.len() - n, t0.elapsed().as_secs_f64());
    co
}
fn elem_sym(f: &Fq, xs: &[El]) -> Vec<El> {
    // e_1..e_m
    let mut e = vec![f.from(1)];
    for x in xs {
        let mut ne = vec![ZERO; e.len() + 1];
        for k in 0..e.len() {
            ne[k] = f.add(&ne[k], &e[k]);
            let t = f.mul(&e[k], x);
            ne[k + 1] = f.add(&ne[k + 1], &t);
        }
        e = ne;
    }
    e[1..].to_vec()
}

// ---------------------------------------------------------------- descent and output
/// A polynomial over F_q in named F_p-unknowns: terms (exponents over vars, coefficient).
struct QPoly {
    terms: Vec<(Vec<u8>, El)>,
}
fn descend(f: &Fq, poly: &QPoly) -> Vec<BTreeMap<Vec<u8>, u64>> {
    let mut comps = vec![BTreeMap::new(); f.m];
    for (mo, c) in &poly.terms {
        for k in 0..f.m {
            if c[k] != 0 {
                let e = comps[k].entry(mo.clone()).or_insert(0u64);
                *e = (*e + c[k]) % f.p;
            }
        }
    }
    for c in comps.iter_mut() {
        c.retain(|_, v| *v != 0);
    }
    comps
}
fn poly_string(vars: &[String], p: &BTreeMap<Vec<u8>, u64>) -> String {
    let mut terms: Vec<(&Vec<u8>, &u64)> = p.iter().collect();
    terms.sort_by(|a, b| {
        let da: u32 = a.0.iter().map(|&x| x as u32).sum();
        let db: u32 = b.0.iter().map(|&x| x as u32).sum();
        db.cmp(&da).then(b.0.cmp(a.0))
    });
    let mut out = Vec::new();
    for (mo, c) in terms {
        let mut s = c.to_string();
        for (i, &e) in mo.iter().enumerate() {
            if e == 1 {
                s.push_str(&format!("*{}", vars[i]));
            } else if e > 1 {
                s.push_str(&format!("*{}^{}", vars[i], e));
            }
        }
        out.push(s);
    }
    if out.is_empty() {
        "0".to_string()
    } else {
        out.join("+")
    }
}
fn eval_fp(p: &BTreeMap<Vec<u8>, u64>, pt: &[u64], pr: u64) -> u64 {
    let mut s = 0u64;
    for (mo, c) in p {
        let mut t = *c;
        for (i, &e) in mo.iter().enumerate() {
            t = t * pw(pt[i], e as u64, pr) % pr;
        }
        s = (s + t) % pr;
    }
    s
}
fn total_degree(p: &BTreeMap<Vec<u8>, u64>) -> u32 {
    p.keys().map(|m| m.iter().map(|&x| x as u32).sum()).max().unwrap_or(0)
}

struct System {
    arm: String,
    vars: Vec<String>,
    eqs: Vec<BTreeMap<Vec<u8>, u64>>,
    planted: Option<Vec<u64>>,
}
fn write_system(dir: &str, f: &Fq, s: &System, meta: &mut Vec<String>) {
    let path = format!("{dir}/{}.ms", s.arm);
    let mut fh = fs::File::create(&path).unwrap();
    writeln!(fh, "{}", s.vars.join(",")).unwrap();
    writeln!(fh, "{}", f.p).unwrap();
    let strs: Vec<String> = s.eqs.iter().map(|e| poly_string(&s.vars, e)).collect();
    writeln!(fh, "{}", strs.join(",\n")).unwrap();
    let degs: Vec<String> = s.eqs.iter().map(|e| total_degree(e).to_string()).collect();
    let nts: Vec<String> = s.eqs.iter().map(|e| e.len().to_string()).collect();
    let mut planted_ok = String::from("null");
    if let Some(pt) = &s.planted {
        let ok = s.eqs.iter().all(|e| eval_fp(e, pt, f.p) == 0);
        assert!(ok, "{}: planted point does not satisfy the descended system", s.arm);
        planted_ok = "true".into();
        let ps: Vec<String> = pt.iter().map(|x| x.to_string()).collect();
        fs::write(format!("{dir}/{}.planted", s.arm), format!("{}\n{}\n", s.vars.join(","), ps.join(","))).unwrap();
    }
    meta.push(format!(
        "    \"{}\": {{\"file\": \"{}.ms\", \"vars\": {:?}, \"n_equations\": {}, \"total_degrees\": [{}], \"n_terms\": [{}], \"planted_satisfies\": {}}}",
        s.arm, s.arm, s.vars, s.eqs.len(), degs.join(", "), nts.join(", "), planted_ok
    ));
}

// ---------------------------------------------------------------- generate
fn arg(args: &[String], key: &str, default: &str) -> String {
    args.iter().position(|a| a == key).map(|i| args[i + 1].clone()).unwrap_or_else(|| default.to_string())
}

fn generate(args: &[String]) {
    let m: usize = arg(args, "--m", "3").parse().unwrap();
    let p: u64 = arg(args, "--p", "331").parse().unwrap();
    let seed: u64 = arg(args, "--seed", "1").parse().unwrap();
    let target = arg(args, "--target", "planted");
    let arms_s = arg(args, "--arms", if m == 3 { "raw_x,sym_x,raw_u,sym_u,quot" } else { "quot" });
    let dir = arg(args, "--out", "out");
    let arms: Vec<&str> = arms_s.split(',').collect();
    fs::create_dir_all(&dir).unwrap();
    let t0 = Instant::now();
    let f = Fq::new(m, p);
    let mut rng = Rng(seed ^ 0x5eed_0000_0000_0000 ^ (p << 8) ^ m as u64);
    let a2 = f.from(2);
    // b = b0 z with b and a2^2 - 4b non-squares in F_q (smallest b0)
    let (b0, b) = (1..p)
        .map(|b0| (b0, f.scal(&f.z(), b0)))
        .find(|(_, b)| !f.is_square(b) && !f.is_square(&f.sub(&f.from(4), &f.scal(b, 4))))
        .unwrap();
    let cv = Curve { a2, a4: b, a6: ZERO };
    let sem = Semaev::new(&f, &cv);
    let beta = (2..p).find(|&v| pw(v, (p - 1) / 2, p) != 1).unwrap();
    let lam = f.sqrt(&f.mul(&b, &f.inv(&f.from(beta)))).expect("b/beta must be a square (m odd)");
    assert!(f.eq(&f.scal(&f.mul(&lam, &lam), beta), &b));
    let mut checks = Vec::new();

    // sanity: S_{m+1} vanishes at genuine sums and is symmetric
    let mut vanish = 0;
    let mut nonvanish_random = 0;
    for _ in 0..4 {
        let pts: Vec<(El, El)> = (0..m).map(|_| random_point(&f, &cv, &mut rng)).collect();
        let mut r: Pt = None;
        for pt in &pts {
            r = ec_add(&f, &cv, &r, &Some(*pt));
        }
        if let Some((xr, _)) = r {
            let xs: Vec<El> = pts.iter().map(|p| p.0).collect();
            if f.is_zero(&sem.eval(&xs, &xr)) {
                vanish += 1;
            }
            let mut xs2 = xs.clone();
            xs2.swap(0, m - 1);
            assert!(f.eq(&sem.eval(&xs, &xr), &sem.eval(&xs2, &xr)));
            let mut xs3 = xs.clone();
            let last = xs3[m - 1];
            xs3[m - 1] = xr;
            assert!(f.is_zero(&sem.eval(&xs3, &last)), "S not symmetric in the target slot");
            let rnd = f.random(&mut rng);
            if !f.is_zero(&sem.eval(&xs, &rnd)) {
                nonvanish_random += 1;
            }
        }
    }
    assert_eq!(vanish, 4, "S_{{m+1}} does not vanish at genuine sums");
    checks.push(format!("\"semaev_vanishes_at_sums\": \"{vanish}/4\", \"semaev_nonzero_at_random_X\": \"{nonvanish_random}/4\""));

    // target
    let (x0, planted_u): (El, Option<Vec<u64>>) = if target == "planted" {
        // u_i in F_p with a point at x = lam u_i; R = sum of +-P_i
        let mut us = Vec::new();
        let mut r: Pt = None;
        while us.len() < m {
            let u = 1 + rng.below(p - 1);
            if us.contains(&u) || pw(u, 2, p) == beta % p {
                continue;
            }
            let x = f.scal(&lam, u);
            let rr = rhs(&f, &cv, &x);
            if let Some(y) = f.sqrt(&rr) {
                let y = if rng.below(2) == 0 { y } else { f.neg(&y) };
                r = ec_add(&f, &cv, &r, &Some((x, y)));
                us.push(u);
            }
        }
        let (xr, _) = r.expect("planted sum is the identity; change seed");
        (xr, Some(us))
    } else {
        (random_point(&f, &cv, &mut rng).0, None)
    };
    let h = 1usize << (m - 2);
    let dfull = 1usize << (m - 1);
    let mut meta_sys = Vec::new();
    let names = |pre: &str| -> Vec<String> { (1..=m).map(|i| format!("{pre}{i}")).collect() };
    let nver = 24;

    for arm in &arms {
        match *arm {
            "raw_x" | "raw_u" | "sym_x" | "sym_u" => {
                assert!(m == 3, "raw and S_m arms are generated at m = 3 only (protocol)");
                let rescale = arm.ends_with("_u");
                let sym = arm.starts_with("sym");
                let monos = if sym { monos_total(m, dfull) } else { monos_box(m, dfull) };
                let need = monos.len() + nver;
                let samples: Vec<(Vec<El>, El)> = (0..need)
                    .map(|_| {
                        let v: Vec<El> = (0..m).map(|_| f.random(&mut rng)).collect();
                        let xs: Vec<El> = if rescale { v.iter().map(|u| f.mul(&lam, u)).collect() } else { v.clone() };
                        let val = sem.eval(&xs, &x0);
                        let pt = if sym { elem_sym(&f, &v) } else { v };
                        (pt, val)
                    })
                    .collect();
                let co = interpolate(&f, &monos, &samples, arm);
                let poly = QPoly { terms: monos.iter().cloned().zip(co).filter(|(_, c)| !f.is_zero(c)).collect() };
                let eqs = descend(&f, &poly);
                let vars = if sym { names("e") } else { names(if rescale { "u" } else { "x" }) };
                let planted = if rescale {
                    planted_u.as_ref().map(|us| {
                        if sym {
                            let ue: Vec<El> = us.iter().map(|&u| f.from(u)).collect();
                            elem_sym(&f, &ue).iter().map(|e| e[0]).collect()
                        } else {
                            us.clone()
                        }
                    })
                } else {
                    None
                };
                let (mut vars, mut eqs, mut planted) = (vars, eqs, planted);
                if !sym {
                    // The raw arms are not in shape position (the S_m x (Z/2)^{m-1} orbits share
                    // every coordinate value), so we add our own separating variable
                    // a = 1*v1 + 3*v2 + 7*v3 (last, lowest in DRL) instead of letting msolve
                    // pick one; its planted value is then known.
                    let coef = [1u64, 3, 7];
                    let mut tie = BTreeMap::new();
                    let mut e = vec![0u8; m + 1];
                    e[m] = 1;
                    tie.insert(e, 1u64);
                    for i in 0..m {
                        let mut e = vec![0u8; m + 1];
                        e[i] = 1;
                        tie.insert(e, (p - coef[i] % p) % p);
                    }
                    for eq in eqs.iter_mut() {
                        *eq = eq.iter().map(|(k, v)| {
                            let mut k2 = k.clone();
                            k2.push(0);
                            (k2, *v)
                        }).collect();
                    }
                    eqs.push(tie);
                    vars.push("a".into());
                    if let Some(pl) = planted.as_mut() {
                        let a = (0..m).fold(0u64, |acc, i| (acc + coef[i] * pl[i]) % p);
                        pl.push(a);
                    }
                }
                write_system(&dir, &f, &System { arm: arm.to_string(), vars, eqs, planted }, &mut meta_sys);
            }
            "quot" => {
                let ma = monos_total(m, h);
                let mb = monos_total(m, h - 1);
                let mq = monos_total(m, 2);
                let need = ma.len() + nver;
                let inv_two = f.from(inv_p(2, p));
                let fbeta = f.from(beta);
                let mut sa = Vec::with_capacity(need);
                let mut sb = Vec::with_capacity(need);
                let mut sq = Vec::with_capacity(need);
                let tq = Instant::now();
                for _ in 0..need {
                    let us: Vec<El> = (0..m).map(|_| f.random_nonzero(&mut rng)).collect();
                    let binv: Vec<El> = us.iter().map(|u| f.mul(&fbeta, &f.inv(u))).collect();
                    let ts: Vec<El> = us.iter().zip(&binv).map(|(u, bu)| f.add(u, bu)).collect();
                    let sig = elem_sym(&f, &ts);
                    let mut w = f.from(1);
                    let mut qv = f.from(1);
                    let mut uprod = f.from(1);
                    for i in 0..m {
                        w = f.mul(&w, &f.sub(&us[i], &binv[i]));
                        qv = f.mul(&qv, &f.sub(&f.mul(&ts[i], &ts[i]), &f.scal(&fbeta, 4)));
                        uprod = f.mul(&uprod, &us[i]);
                    }
                    let xs: Vec<El> = us.iter().map(|u| f.mul(&lam, u)).collect();
                    let mut xs1 = xs.clone();
                    xs1[0] = f.mul(&lam, &binv[0]);
                    let uprod1 = f.mul(&f.mul(&uprod, &binv[0]), &f.inv(&us[0]));
                    let l = f.mul(&sem.eval(&xs, &x0), &f.inv(&f.pow(&uprod, h as u128)));
                    let l1 = f.mul(&sem.eval(&xs1, &x0), &f.inv(&f.pow(&uprod1, h as u128)));
                    let av = f.mul(&f.add(&l, &l1), &inv_two);
                    let bv = f.mul(&f.mul(&f.sub(&l, &l1), &inv_two), &f.inv(&w));
                    sa.push((sig.clone(), av));
                    sb.push((sig.clone(), bv));
                    sq.push((sig, qv));
                }
                eprintln!("  quot: {need} samples in {:.1} s", tq.elapsed().as_secs_f64());
                let ca = interpolate(&f, &ma, &sa, "quot.A");
                let cb = interpolate(&f, &mb, &sb[..mb.len() + nver], "quot.B");
                let cq = interpolate(&f, &mq, &sq[..mq.len() + nver], "quot.Q");
                for c in &cq {
                    assert!(c[1..m].iter().all(|&x| x == 0), "Q has a coefficient outside F_p");
                }
                // A(s) + w B(s): variables s1..sm, w
                let mut terms = Vec::new();
                for (mo, c) in ma.iter().zip(ca) {
                    if !f.is_zero(&c) {
                        let mut e = mo.clone();
                        e.push(0);
                        terms.push((e, c));
                    }
                }
                for (mo, c) in mb.iter().zip(cb) {
                    if !f.is_zero(&c) {
                        let mut e = mo.clone();
                        e.push(1);
                        terms.push((e, c));
                    }
                }
                let mut eqs = descend(&f, &QPoly { terms });
                let mut weq = BTreeMap::new();
                let mut e2 = vec![0u8; m];
                e2.push(2);
                weq.insert(e2, 1u64);
                for (mo, c) in mq.iter().zip(cq) {
                    if c[0] != 0 {
                        let mut e = mo.clone();
                        e.push(0);
                        let v = weq.entry(e).or_insert(0);
                        *v = (*v + p - c[0]) % p;
                    }
                }
                weq.retain(|_, v| *v != 0);
                eqs.push(weq);
                let mut vars = names("s");
                vars.push("w".into());
                let planted = planted_u.as_ref().map(|us| {
                    let ts: Vec<El> = us.iter().map(|&u| f.from((u + beta * inv_p(u, p)) % p)).collect();
                    let mut v: Vec<u64> = elem_sym(&f, &ts).iter().map(|e| e[0]).collect();
                    let mut w = 1u64;
                    for &u in us {
                        w = w * ((u + p - beta * inv_p(u, p) % p) % p) % p;
                    }
                    v.push(w);
                    v
                });
                write_system(&dir, &f, &System { arm: "quot".into(), vars, eqs, planted }, &mut meta_sys);
            }
            other => panic!("unknown arm {other}"),
        }
    }
    let pu = match &planted_u {
        Some(v) => format!("{v:?}"),
        None => "null".into(),
    };
    let meta = format!(
        "{{\n  \"tool\": \"qsolve generate\",\n  \"m\": {m}, \"p\": {p}, \"seed\": {seed}, \"target\": \"{target}\",\n  \"field\": \"F_{p}[z]/(z^{m} - {})\",\n  \"curve\": \"y^2 = x(x^2 + 2x + {b0}*z)\",\n  \"beta\": {beta}, \"lambda\": \"{}\",\n  \"x_R\": \"{}\",\n  \"planted_u\": {pu},\n  {},\n  \"generation_wall_seconds\": {:.2},\n  \"systems\": {{\n{}\n  }}\n}}\n",
        f.c,
        f.fmt(&lam),
        f.fmt(&x0),
        checks.join(", "),
        t0.elapsed().as_secs_f64(),
        meta_sys.join(",\n")
    );
    fs::write(format!("{dir}/meta.json"), &meta).unwrap();
    print!("{meta}");
}

// ---------------------------------------------------------------- msolve output check
#[derive(Debug, Clone)]
enum Tok {
    L(Vec<Tok>),
    N(String),
    S(String),
}
fn parse_list(s: &str) -> Tok {
    let b: Vec<char> = s.chars().collect();
    let mut i = 0;
    fn rec(b: &[char], i: &mut usize) -> Tok {
        while *i < b.len() && (b[*i].is_whitespace() || b[*i] == ',') {
            *i += 1;
        }
        if b[*i] == '[' {
            *i += 1;
            let mut v = Vec::new();
            loop {
                while *i < b.len() && (b[*i].is_whitespace() || b[*i] == ',') {
                    *i += 1;
                }
                if b[*i] == ']' {
                    *i += 1;
                    return Tok::L(v);
                }
                v.push(rec(b, i));
            }
        } else if b[*i] == '\'' {
            *i += 1;
            let st = *i;
            while b[*i] != '\'' {
                *i += 1;
            }
            let s: String = b[st..*i].iter().collect();
            *i += 1;
            Tok::S(s)
        } else {
            let st = *i;
            while *i < b.len() && (b[*i].is_ascii_digit() || b[*i] == '-') {
                *i += 1;
            }
            Tok::N(b[st..*i].iter().collect())
        }
    }
    rec(&b, &mut i)
}
fn as_list(t: &Tok) -> &Vec<Tok> {
    match t {
        Tok::L(v) => v,
        _ => panic!("expected list, got {t:?}"),
    }
}
fn as_num(t: &Tok, p: u64) -> u64 {
    match t {
        Tok::N(s) => {
            let v: i128 = s.parse().unwrap();
            (v.rem_euclid(p as i128)) as u64
        }
        _ => panic!("expected number, got {t:?}"),
    }
}
fn poly_of(t: &Tok, p: u64) -> Vec<u64> {
    // [deg, [c0..cdeg]]
    let l = as_list(t);
    as_list(&l[1]).iter().map(|x| as_num(x, p)).collect()
}
fn horner(c: &[u64], x: u64, p: u64) -> u64 {
    c.iter().rev().fold(0u64, |acc, &ci| (acc * x + ci) % p)
}

fn check(args: &[String]) {
    let out = arg(args, "--msolve-out", "");
    let planted = arg(args, "--planted", "");
    let mut s = String::new();
    fs::File::open(&out).unwrap().read_to_string(&mut s).unwrap();
    let s = s.trim().trim_end_matches(':');
    let root = parse_list(s);
    let r = as_list(&root);
    let dimflag = as_num(&r[0], u64::MAX);
    if dimflag != 0 {
        println!("{{\"check\": \"fail\", \"reason\": \"msolve did not return a zero-dimensional parametrization\", \"flag\": {dimflag}}}");
        return;
    }
    let body = as_list(&r[1]);
    let p = as_num(&body[0], u64::MAX);
    let nvars = as_num(&body[1], u64::MAX) as usize;
    let deg = as_num(&body[2], u64::MAX);
    let vars: Vec<String> = as_list(&body[3]).iter().map(|t| if let Tok::S(s) = t { s.clone() } else { panic!() }).collect();
    let lf: Vec<u64> = as_list(&body[4]).iter().map(|x| as_num(x, p)).collect();
    let param = as_list(&body[5]);
    let inner = as_list(&param[1]);
    let elim = poly_of(&inner[0], p);
    let denom = poly_of(&inner[1], p);
    let params: Vec<Vec<u64>> = as_list(&inner[2])
        .iter()
        .map(|t| {
            let l = as_list(t);
            if let Tok::L(_) = &l[0] {
                if let Tok::L(ll) = &l[0] {
                    if let Tok::L(_) = &ll[0] {
                        return poly_of(&ll[0], p);
                    }
                }
                poly_of(&l[0], p)
            } else {
                poly_of(t, p)
            }
        })
        .collect();
    let mut summary = format!(
        "{{\"p\": {p}, \"nvars\": {nvars}, \"param_degree\": {deg}, \"elim_degree\": {}, \"linear_form\": {lf:?}, \"n_params\": {}",
        elim.len() - 1,
        params.len()
    );
    if !planted.is_empty() {
        let pl = fs::read_to_string(&planted).unwrap();
        let mut lines = pl.lines();
        let pvars: Vec<&str> = lines.next().unwrap().split(',').collect();
        let pvals: Vec<u64> = lines.next().unwrap().split(',').map(|x| x.parse::<u64>().unwrap() % p).collect();
        // msolve may reorder variables (map by name) and, when the staircase is not generic, add
        // one variable A tied to a linear form whose coefficients are k^J (k = 1..n, the input
        // variable index); J is read from the msolve log and both signs of the tie are tried.
        let extra: Vec<&String> = vars.iter().filter(|v| !pvars.contains(&v.as_str())).collect();
        assert!(extra.len() <= 1, "more than one variable added by msolve");
        let jexp: Option<u64> = if extra.is_empty() {
            None
        } else {
            let log = fs::read_to_string(arg(args, "--msolve-log", "")).expect("an added variable needs --msolve-log");
            log.lines()
                .filter(|l| l.contains("coefficients of linear form are k^"))
                .last()
                .and_then(|l| l.split("k^").nth(1))
                .and_then(|r| r.split_whitespace().next())
                .and_then(|v| v.parse().ok())
        };
        let base: Vec<u64> = pvars.iter().map(|v| pvals[pvars.iter().position(|x| x == v).unwrap()]).collect();
        let tie: u64 = match jexp {
            None => 0,
            Some(j) => base.iter().enumerate().fold(0u64, |a, (k, x)| (a + pw(k as u64 + 1, j, p) * x) % p),
        };
        let mut best = (false, false, 0u64, "none");
        for (sign, label) in [(tie, "+"), ((p - tie) % p, "-")] {
            let pt: Vec<u64> = vars
                .iter()
                .map(|v| match pvars.iter().position(|x| x == v) {
                    Some(i) => pvals[i],
                    None => sign,
                })
                .collect();
            let t = lf.iter().zip(&pt).fold(0u64, |a, (c, x)| (a + c * x) % p);
            let elim_ok = horner(&elim, t, p) == 0;
            let dv = horner(&denom, t, p);
            let mut coords_ok = dv != 0;
            // params give the first nvars-1 variables as x_i = -v_i(t)/denom(t) (msolve's prime-field form)
            if coords_ok {
                for (i, v) in params.iter().enumerate() {
                    let xi = (p - horner(v, t, p)) % p * inv_p(dv, p) % p;
                    if xi != pt[i] {
                        coords_ok = false;
                    }
                }
            }
            if (elim_ok && coords_ok) || best.3 == "none" {
                best = (elim_ok, coords_ok, t, if jexp.is_some() { label } else { "no added variable" });
            }
            if jexp.is_none() || (elim_ok && coords_ok) {
                break;
            }
        }
        if jexp.is_some() && !(best.0 && best.1) && (p as u128) * (elim.len() as u128) <= 20_000_000_000 {
            // Fallback when msolve added a variable whose tie we cannot reconstruct: enumerate
            // the F_p-rational points of the parametrization and look for one that projects
            // onto the planted values.
            let (_, rv, pts) = rur_points(&out);
            let hit = pts.iter().any(|pt| pvars.iter().zip(&pvals).all(|(v, x)| pick(&rv, pt, v) == *x));
            best = (hit, hit, 0, "rational-point enumeration");
        }
        summary.push_str(&format!(
            ", \"added_variable_exponent\": {}, \"added_variable_tie\": \"{}\", \"planted_linear_form_value\": {}, \"planted_root_of_eliminant\": {}, \"planted_coordinates_recovered\": {}",
            jexp.map(|j| j.to_string()).unwrap_or("null".into()),
            best.3,
            best.2,
            best.0,
            best.1
        ));
    }
    summary.push('}');
    println!("{summary}");
}

/// F_p-rational points of a zero-dimensional msolve parametrization (all roots t in F_p of the
/// eliminating polynomial, found by evaluation at every t), as (variable names, points).
fn rur_points(path: &str) -> (u64, Vec<String>, Vec<Vec<u64>>) {
    let s = fs::read_to_string(path).unwrap();
    let root = parse_list(s.trim().trim_end_matches(':'));
    let r = as_list(&root);
    assert_eq!(as_num(&r[0], u64::MAX), 0, "{path}: not a zero-dimensional parametrization");
    let body = as_list(&r[1]);
    let p = as_num(&body[0], u64::MAX);
    let vars: Vec<String> = as_list(&body[3]).iter().map(|t| if let Tok::S(s) = t { s.clone() } else { panic!() }).collect();
    let n = vars.len();
    let lf: Vec<u64> = as_list(&body[4]).iter().map(|x| as_num(x, p)).collect();
    let inner = as_list(&as_list(&body[5])[1]).clone();
    let elim = poly_of(&inner[0], p);
    let denom = poly_of(&inner[1], p);
    let params: Vec<Vec<u64>> = as_list(&inner[2]).iter().map(|t| poly_of(&as_list(t)[0], p)).collect();
    assert_eq!(params.len(), n - 1);
    assert!(lf[n - 1] != 0, "last linear-form coefficient is zero");
    let inv_last = inv_p(lf[n - 1], p);
    let mut pts = Vec::new();
    for t in 0..p {
        if horner(&elim, t, p) != 0 {
            continue;
        }
        let dv = horner(&denom, t, p);
        assert!(dv != 0, "{path}: denominator vanishes at a root (non-radical?)");
        let idv = inv_p(dv, p);
        let mut x: Vec<u64> = params.iter().map(|v| (p - horner(v, t, p)) % p * idv % p).collect();
        let partial = (0..n - 1).fold(0u64, |a, i| (a + lf[i] * x[i]) % p);
        x.push((t + p - partial) % p * inv_last % p);
        pts.push(x);
    }
    (p, vars, pts)
}
fn pick(vars: &[String], pt: &[u64], name: &str) -> u64 {
    pt[vars.iter().position(|v| v == name).unwrap_or_else(|| panic!("no variable {name}"))]
}
fn roots_fp(coeffs_low_high: &[u64], p: u64) -> Vec<u64> {
    (0..p).filter(|&x| horner(coeffs_low_high, x, p) == 0).collect()
}

/// Identity control at m = 3: the rescaled raw arm's F_p-rational solutions map onto the
/// quotient arm's (and the S_m arm's) solutions orbit by orbit.
fn orbits(args: &[String]) {
    let m: usize = arg(args, "--m", "3").parse().unwrap();
    let beta: u64 = arg(args, "--beta", "").parse().expect("--beta");
    let (p, rv, raw) = rur_points(&arg(args, "--raw", ""));
    let (_, qv, quot) = rur_points(&arg(args, "--quot", ""));
    let (_, sv, sym) = rur_points(&arg(args, "--sym", ""));
    let qset: std::collections::BTreeSet<Vec<u64>> = quot
        .iter()
        .map(|pt| {
            let mut v: Vec<u64> = (1..=m).map(|k| pick(&qv, pt, &format!("s{k}"))).collect();
            v.push(pick(&qv, pt, "w"));
            v
        })
        .collect();
    let sset: std::collections::BTreeSet<Vec<u64>> =
        sym.iter().map(|pt| (1..=m).map(|k| pick(&sv, pt, &format!("e{k}"))).collect()).collect();
    let mut groups: BTreeMap<Vec<u64>, usize> = BTreeMap::new();
    let mut sgroups: BTreeMap<Vec<u64>, usize> = BTreeMap::new();
    let (mut in_quot, mut in_sym, mut degenerate) = (0usize, 0usize, 0usize);
    for pt in &raw {
        let us: Vec<u64> = (1..=m).map(|k| pick(&rv, pt, &format!("u{k}"))).collect();
        if us.iter().any(|&u| u == 0) {
            degenerate += 1;
            continue;
        }
        let ts: Vec<u64> = us.iter().map(|&u| (u + beta * inv_p(u, p)) % p).collect();
        // elementary symmetric functions over F_p
        let esym = |xs: &[u64]| -> Vec<u64> {
            let mut e = vec![1u64];
            for &x in xs {
                let mut ne = vec![0u64; e.len() + 1];
                for k in 0..e.len() {
                    ne[k] = (ne[k] + e[k]) % p;
                    ne[k + 1] = (ne[k + 1] + e[k] * x) % p;
                }
                e = ne;
            }
            e[1..].to_vec()
        };
        let mut img = esym(&ts);
        let w = us.iter().fold(1u64, |a, &u| a * ((u + p - beta * inv_p(u, p) % p) % p) % p);
        img.push(w);
        if qset.contains(&img) {
            in_quot += 1;
        }
        *groups.entry(img).or_insert(0) += 1;
        let simg = esym(&us);
        if sset.contains(&simg) {
            in_sym += 1;
        }
        *sgroups.entry(simg).or_insert(0) += 1;
    }
    // quotient solutions that lift to F_p-rational u (all t-roots and u-roots in F_p)
    let mut liftable = 0usize;
    let mut liftable_hit = 0usize;
    for q in &qset {
        // T^m - s1 T^{m-1} + s2 T^{m-2} - ... (low to high coefficients)
        let mut c = vec![0u64; m + 1];
        c[m] = 1;
        for k in 1..=m {
            let v = q[k - 1];
            c[m - k] = if k % 2 == 1 { (p - v) % p } else { v };
        }
        let rts = roots_fp(&c, p);
        // count with multiplicity by deflation is unnecessary for the generic case: require m distinct roots
        if rts.len() != m {
            continue;
        }
        let ok = rts.iter().all(|&t| {
            let disc = (t * t % p + p - 4 * beta % p) % p;
            disc == 0 || pw(disc, (p - 1) / 2, p) == 1
        });
        if ok {
            liftable += 1;
            if groups.contains_key(q) {
                liftable_hit += 1;
            }
        }
    }
    let mut hist: BTreeMap<usize, usize> = BTreeMap::new();
    for v in groups.values() {
        *hist.entry(*v).or_insert(0) += 1;
    }
    let mut shist: BTreeMap<usize, usize> = BTreeMap::new();
    for v in sgroups.values() {
        *shist.entry(*v).or_insert(0) += 1;
    }
    println!(
        "{{\"p\": {p}, \"raw_rational\": {}, \"raw_with_zero_coordinate\": {degenerate}, \"raw_images_in_quot\": {in_quot}, \"raw_images_in_sym\": {in_sym}, \"quot_rational\": {}, \"sym_rational\": {}, \"orbit_size_histogram_quot\": {:?}, \"orbit_size_histogram_sym\": {:?}, \"quot_rational_liftable\": {liftable}, \"quot_liftable_hit_by_raw\": {liftable_hit}, \"identity_control_pass\": {}}}",
        raw.len(),
        qset.len(),
        sset.len(),
        hist,
        shist,
        in_quot == raw.len() - degenerate && in_sym == raw.len() - degenerate && liftable == liftable_hit && liftable == groups.len()
    );
}

// ---------------------------------------------------------------- measured runner
#[repr(C)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}
#[repr(C)]
struct Rusage {
    ru_utime: Timeval,
    ru_stime: Timeval,
    ru_maxrss: i64,
    rest: [i64; 13],
}
extern "C" {
    fn getrusage(who: i32, usage: *mut Rusage) -> i32;
}
fn proc_kib(pid: u32, key: &str) -> Option<u64> {
    let s = fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    s.lines().find(|l| l.starts_with(key)).and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok())
}

fn run(args: &[String]) {
    // qsolve run --rss-cap-gib G --record FILE -- cmd args...
    let cap_gib: f64 = arg(args, "--rss-cap-gib", "13").parse().unwrap();
    let wall_cap: f64 = arg(args, "--wall-cap-s", "0").parse().unwrap();
    let rec = arg(args, "--record", "run.json");
    let sep = args.iter().position(|a| a == "--").expect("missing -- before the command");
    let cmd = &args[sep + 1..];
    let cap_kib = (cap_gib * 1024.0 * 1024.0) as u64;
    let t0 = Instant::now();
    let mut child = std::process::Command::new(&cmd[0]).args(&cmd[1..]).spawn().expect("spawn");
    let pid = child.id();
    let mut peak_sampled = 0u64;
    let mut last = 0u64;
    let mut killed = false;
    let mut killed_wall = false;
    let mut samples: Vec<(f64, u64)> = Vec::new();
    let mut next_log = 0.0f64;
    let status = loop {
        if let Some(st) = child.try_wait().unwrap() {
            break st;
        }
        if let Some(rss) = proc_kib(pid, "VmRSS:") {
            last = rss;
            peak_sampled = peak_sampled.max(rss);
            let el = t0.elapsed().as_secs_f64();
            if el >= next_log {
                samples.push((el, rss));
                next_log = el + if el < 60.0 { 1.0 } else { 10.0 };
            }
            if rss > cap_kib && !killed {
                eprintln!("qsolve run: RSS {rss} KiB above cap {cap_kib} KiB; killing");
                child.kill().ok();
                killed = true;
            }
            if wall_cap > 0.0 && el > wall_cap && !killed && !killed_wall {
                eprintln!("qsolve run: wall {el:.0} s above cap {wall_cap} s; killing");
                child.kill().ok();
                killed_wall = true;
            }
        }
        std::thread::sleep(Duration::from_millis(200));
    };
    let wall = t0.elapsed().as_secs_f64();
    let mut ru = Rusage { ru_utime: Timeval { tv_sec: 0, tv_usec: 0 }, ru_stime: Timeval { tv_sec: 0, tv_usec: 0 }, ru_maxrss: 0, rest: [0; 13] };
    unsafe {
        getrusage(-1, &mut ru);
    }
    let ut = ru.ru_utime.tv_sec as f64 + ru.ru_utime.tv_usec as f64 * 1e-6;
    let stt = ru.ru_stime.tv_sec as f64 + ru.ru_stime.tv_usec as f64 * 1e-6;
    use std::os::unix::process::ExitStatusExt;
    let ser: Vec<String> = samples.iter().map(|(t, r)| format!("[{t:.1},{r}]")).collect();
    let js = format!(
        "{{\"cmd\": {:?}, \"exit_code\": {}, \"signal\": {}, \"killed_by_rss_cap\": {killed}, \"rss_cap_gib\": {cap_gib}, \"killed_by_wall_cap\": {killed_wall}, \"wall_cap_seconds\": {wall_cap}, \"wall_seconds\": {wall:.3}, \"user_seconds\": {ut:.3}, \"sys_seconds\": {stt:.3}, \"maxrss_kib\": {}, \"peak_rss_sampled_kib\": {peak_sampled}, \"last_rss_sampled_kib\": {last}, \"rss_samples_s_kib\": [{}]}}\n",
        cmd,
        status.code().map(|c| c.to_string()).unwrap_or("null".into()),
        status.signal().map(|c| c.to_string()).unwrap_or("null".into()),
        ru.ru_maxrss,
        ser.join(",")
    );
    fs::write(&rec, &js).unwrap();
    print!("{js}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("generate") => generate(&args[2..]),
        Some("check") => check(&args[2..]),
        Some("orbits") => orbits(&args[2..]),
        Some("run") => run(&args[2..]),
        _ => {
            eprintln!("usage: qsolve generate --m 3|5 --p P --seed S --target planted|random [--arms a,b] --out DIR\n       qsolve check --msolve-out FILE [--planted FILE]\n       qsolve run [--rss-cap-gib G] [--record FILE] -- cmd ...");
            std::process::exit(2);
        }
    }
}
