//! Twisted-line applicability on the Oakley EC2N curves (RFC 2409 6.3, 6.4)
//! and a c2pnb positive control. std-only, native Rust.
//!
//! E: y^2 + xy = x^3 + a x^2 + b over F_{2^N}, N = d n, q = 2^d, c = sqrt(b),
//! T = Tr_{F_{2^N}/F_q}. If T(u) = T(c/u) = 0 and Tr_N(a) = 0, every nonzero
//! x in u F_q lifts (Lean: twisted_line_lifts in
//! research/lift_dense_factor_base_20261027/lean/LiftDense.lean).
//!
//! Search: u in ker T = {z + z^q}. For u in ker T, T(c/u) = N(u)^-1 G(u),
//! G(u) = T(c prod_{i=1}^{n-1} u^{q^i}). On a line u0 + t u1 (t in F_q)
//! G becomes P(t) = sum_k T(coef_k) t^k, coef_k the coefficients of
//! c prod_i (u0^{q^i} + t u1^{q^i}); roots of P in F_q via
//! gcd(P, t^q - t) and the trace-splitting trick. Every u is re-verified
//! directly in F_{2^N} (T(u), T(c/u) via a field inverse).
//!
//! Usage: oakley_lift <c2pnb-params.txt> <out.json> [samples] [max_lines]

use std::fmt::Write as _;
use std::time::Instant;

type El = Vec<u64>;
type Poly = Vec<El>; // low to high, coefficients in F_{2^N}

const MASTER: u64 = 0x0AC1_2026_1027;

fn bit(a: &[u64], i: usize) -> bool {
    (a[i / 64] >> (i % 64)) & 1 == 1
}
fn degree(a: &[u64]) -> Option<usize> {
    for i in (0..a.len()).rev() {
        if a[i] != 0 {
            return Some(i * 64 + 63 - a[i].leading_zeros() as usize);
        }
    }
    None
}

struct Gf {
    n: usize,
    limbs: usize,
    poly: El,
}

impl Gf {
    fn new(poly: El) -> Gf {
        let n = degree(&poly).expect("zero polynomial");
        let limbs = n / 64 + 1;
        let mut p = poly;
        p.resize(limbs.max(p.len()), 0);
        Gf { n, limbs, poly: p }
    }
    fn zero(&self) -> El {
        vec![0; self.limbs]
    }
    fn one(&self) -> El {
        let mut z = self.zero();
        z[0] = 1;
        z
    }
    fn is_zero(&self, a: &[u64]) -> bool {
        a.iter().all(|&w| w == 0)
    }
    fn add(&self, a: &[u64], b: &[u64]) -> El {
        a.iter().zip(b).map(|(x, y)| x ^ y).collect()
    }
    fn mul(&self, a: &[u64], b: &[u64]) -> El {
        let mut r = vec![0u64; 2 * self.limbs + 1];
        for (i, &aw) in a.iter().enumerate() {
            if aw == 0 {
                continue;
            }
            for sh in 0..64 {
                if (aw >> sh) & 1 == 1 {
                    for (k, &bw) in b.iter().enumerate() {
                        r[i + k] ^= bw << sh;
                        if sh > 0 {
                            r[i + k + 1] ^= bw >> (64 - sh);
                        }
                    }
                }
            }
        }
        self.reduce(r)
    }
    fn reduce(&self, mut r: El) -> El {
        let top = r.len() * 64;
        for i in (self.n..top).rev() {
            if bit(&r, i) {
                let s = i - self.n;
                let (w, b) = (s / 64, s % 64);
                for (k, &pw) in self.poly.iter().enumerate() {
                    if k + w < r.len() {
                        r[k + w] ^= pw << b;
                    }
                    if b > 0 && k + w + 1 < r.len() {
                        r[k + w + 1] ^= pw >> (64 - b);
                    }
                }
            }
        }
        r.truncate(self.limbs);
        r
    }
    fn sq(&self, a: &[u64]) -> El {
        self.mul(a, a)
    }
    fn frob(&self, a: &[u64], k: usize) -> El {
        let mut z = a.to_vec();
        for _ in 0..k {
            z = self.sq(&z);
        }
        z
    }
    fn inv(&self, a: &[u64]) -> El {
        assert!(!self.is_zero(a));
        // a^(2^N - 2) = prod_{i=1}^{N-1} a^(2^i)
        let mut r = self.one();
        let mut z = self.sq(a);
        for _ in 1..self.n {
            r = self.mul(&r, &z);
            z = self.sq(&z);
        }
        r
    }
    /// Tr_{F_{2^N}/F_{2^d}}, d | N
    fn tr_rel(&self, a: &[u64], d: usize) -> El {
        let mut t = self.zero();
        let mut z = a.to_vec();
        for _ in 0..self.n / d {
            t = self.add(&t, &z);
            z = self.frob(&z, d);
        }
        t
    }
    fn tr(&self, a: &[u64]) -> u64 {
        let t = self.tr_rel(a, 1);
        assert!(t[1..].iter().all(|&w| w == 0) && t[0] <= 1);
        t[0]
    }
    fn small(&self, v: u64) -> El {
        let mut z = self.zero();
        z[0] = v;
        z
    }
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn element(&mut self, g: &Gf) -> El {
        let mut e: El = (0..g.limbs).map(|_| self.next()).collect();
        let extra = g.limbs * 64 - g.n;
        let last = g.limbs - 1;
        e[last] &= u64::MAX >> extra;
        e
    }
    /// random element of F_q = image of Tr_{N/d}
    fn fq(&mut self, g: &Gf, d: usize) -> El {
        let e = self.element(g);
        g.tr_rel(&e, d)
    }
}

fn hex_of(a: &[u64]) -> String {
    let mut s = String::new();
    let mut started = false;
    for &w in a.iter().rev() {
        if started {
            let _ = write!(s, "{:016x}", w);
        } else if w != 0 {
            let _ = write!(s, "{:x}", w);
            started = true;
        }
    }
    if s.is_empty() {
        "0".into()
    } else {
        s
    }
}

fn hex_to_el(hex: &str) -> El {
    let digits: Vec<u8> = hex
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect();
    let mut out = vec![0u64; digits.len() / 16 + 1];
    for (i, &dg) in digits.iter().rev().enumerate() {
        out[i / 16] |= (dg as u64) << (4 * (i % 16));
    }
    out
}

fn field(block: &str, key: &str) -> El {
    let mut acc = String::new();
    let mut on = false;
    for l in block.lines() {
        let t = l.trim_start();
        if on {
            if t.contains(':') && t.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                break;
            }
            acc.push_str(t);
        } else if let Some(rest) = t.strip_prefix(&format!("{key}:")) {
            on = true;
            acc.push_str(rest);
        }
    }
    let acc: String = acc.chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    hex_to_el(&acc)
}

fn cofactor(block: &str) -> u64 {
    for l in block.lines() {
        if let Some(rest) = l.trim_start().strip_prefix("Cofactor:") {
            return rest.split_whitespace().next().unwrap().parse().unwrap();
        }
    }
    panic!("no cofactor")
}

// ---------------- irreducibility over F_2 ----------------

fn shl(a: &[u64], s: usize, len: usize) -> El {
    let mut r = vec![0u64; len];
    let (w, b) = (s / 64, s % 64);
    for (k, &x) in a.iter().enumerate() {
        if k + w < len {
            r[k + w] ^= x << b;
        }
        if b > 0 && k + w + 1 < len {
            r[k + w + 1] ^= x >> (64 - b);
        }
    }
    r
}
fn bin_gcd(a: &[u64], b: &[u64]) -> El {
    let len = a.len().max(b.len()) + 1;
    let mut x = a.to_vec();
    x.resize(len, 0);
    let mut y = b.to_vec();
    y.resize(len, 0);
    loop {
        let Some(dy) = degree(&y) else { return x };
        while let Some(dx) = degree(&x) {
            if dx < dy {
                break;
            }
            let s = shl(&y, dx - dy, len);
            for k in 0..len {
                x[k] ^= s[k];
            }
        }
        std::mem::swap(&mut x, &mut y);
    }
}
fn prime_factors(mut n: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut p = 2;
    while n > 1 {
        if n % p == 0 {
            v.push(p);
            while n % p == 0 {
                n /= p;
            }
        }
        p += 1;
    }
    v
}
/// Returns (x^(2^N) == x mod f, gcd conditions all trivial) => irreducible.
fn irreducible(g: &Gf) -> (bool, bool, Vec<(usize, bool)>) {
    let x = g.small(2);
    let fro = g.frob(&x, g.n) == x;
    let mut all = true;
    let mut per = vec![];
    for p in prime_factors(g.n) {
        let m = g.n / p;
        let h = g.add(&g.frob(&x, m), &x);
        let gc = bin_gcd(&g.poly, &h);
        let one = degree(&gc) == Some(0);
        per.push((p, one));
        all &= one;
    }
    (fro, all, per)
}

// ---------------- curve arithmetic ----------------

type Pt = Option<(El, El)>;

fn padd(g: &Gf, a: &El, p: &Pt, q: &Pt) -> Pt {
    let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
        return if p.is_none() { q.clone() } else { p.clone() };
    };
    if x1 == x2 {
        if g.add(y1, y2) == *x1 || y1 == y2 {
            // y2 = y1 + x1 means q = -p  (and y1==y2 means doubling)
        }
        if y1 != y2 {
            return None; // q = -p
        }
        if g.is_zero(x1) {
            return None;
        }
        let l = g.add(x1, &g.mul(y1, &g.inv(x1)));
        let x3 = g.add(&g.add(&g.sq(&l), &l), a);
        let y3 = g.add(&g.sq(x1), &g.mul(&g.add(&l, &g.one()), &x3));
        return Some((x3, y3));
    }
    let l = g.mul(&g.add(y1, y2), &g.inv(&g.add(x1, x2)));
    let x3 = g.add(&g.add(&g.add(&g.sq(&l), &l), &g.add(x1, x2)), a);
    let y3 = g.add(&g.add(&g.mul(&l, &g.add(x1, &x3)), &x3), y1);
    Some((x3, y3))
}
fn pmul_scalar(g: &Gf, a: &El, p: &Pt, k: &[u64]) -> Pt {
    let mut r: Pt = None;
    let Some(top) = degree(k) else { return None };
    for i in (0..=top).rev() {
        r = padd(g, a, &r, &r);
        if bit(k, i) {
            r = padd(g, a, &r, p);
        }
    }
    r
}
fn on_curve(g: &Gf, a: &El, b: &El, p: &Pt) -> bool {
    match p {
        None => true,
        Some((x, y)) => {
            let l = g.add(&g.sq(y), &g.mul(x, y));
            let x2 = g.sq(x);
            let r = g.add(&g.add(&g.mul(&x2, x), &g.mul(a, &x2)), b);
            l == r
        }
    }
}
/// solve z^2 + z = w (Tr(w)=0): half-trace for odd N, else F_2-linear
/// elimination on the map z -> z^2 + z
fn solve_quad(g: &Gf, w: &El) -> El {
    if g.n % 2 == 1 {
        let mut z = w.clone();
        let mut t = w.clone();
        for _ in 0..(g.n - 1) / 2 {
            t = g.frob(&t, 2);
            z = g.add(&z, &t);
        }
        return z;
    }
    let mut piv: Vec<(usize, El, El)> = vec![]; // (leading bit, image, combo)
    for i in 0..g.n {
        let mut e = g.zero();
        e[i / 64] |= 1 << (i % 64);
        let mut img = g.add(&g.sq(&e), &e);
        let mut combo = e;
        while let Some(h) = degree(&img) {
            if let Some((_, pi, pc)) = piv.iter().find(|(ph, _, _)| *ph == h) {
                img = g.add(&img, pi);
                combo = g.add(&combo, pc);
            } else {
                piv.push((h, img.clone(), combo.clone()));
                break;
            }
        }
    }
    let mut r = w.clone();
    let mut sol = g.zero();
    while let Some(h) = degree(&r) {
        let (_, pi, pc) = piv.iter().find(|(ph, _, _)| *ph == h).expect("w not in image");
        r = g.add(&r, pi);
        sol = g.add(&sol, pc);
    }
    sol
}
fn lift_ok(g: &Gf, a: &El, b: &El, x: &El) -> bool {
    let w = g.add(&g.add(x, a), &g.mul(b, &g.inv(&g.sq(x))));
    g.tr(&w) == 0
}
fn point_from_x(g: &Gf, a: &El, b: &El, x: &El) -> Pt {
    if !lift_ok(g, a, b, x) {
        return None;
    }
    let w = g.add(&g.add(x, a), &g.mul(b, &g.inv(&g.sq(x))));
    let z = solve_quad(g, &w);
    assert_eq!(g.add(&g.sq(&z), &z), w);
    Some((x.clone(), g.mul(x, &z)))
}

// ---------------- polynomials over F_{2^N} ----------------

fn ptrim(g: &Gf, mut p: Poly) -> Poly {
    while p.last().is_some_and(|c| g.is_zero(c)) {
        p.pop();
    }
    p
}
fn pmulp(g: &Gf, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![g.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            let m = g.mul(x, y);
            r[i + j] = g.add(&r[i + j], &m);
        }
    }
    ptrim(g, r)
}
fn paddp(g: &Gf, a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    let mut r = vec![g.zero(); n];
    for (i, x) in a.iter().enumerate() {
        r[i] = g.add(&r[i], x);
    }
    for (i, x) in b.iter().enumerate() {
        r[i] = g.add(&r[i], x);
    }
    ptrim(g, r)
}
fn pmodp(g: &Gf, a: &Poly, m: &Poly) -> Poly {
    let mut r = a.clone();
    let dm = m.len() - 1;
    let il = g.inv(&m[dm]);
    while r.len() > dm {
        let dr = r.len() - 1;
        let f = g.mul(&r[dr], &il);
        for j in 0..=dm {
            let t = g.mul(&f, &m[j]);
            r[dr - dm + j] = g.add(&r[dr - dm + j], &t);
        }
        r = ptrim(g, r);
    }
    r
}
fn pgcd(g: &Gf, a: &Poly, b: &Poly) -> Poly {
    let (mut x, mut y) = (ptrim(g, a.clone()), ptrim(g, b.clone()));
    while !y.is_empty() {
        let r = pmodp(g, &x, &y);
        x = y;
        y = r;
    }
    if let Some(l) = x.last() {
        let il = g.inv(l);
        x = x.iter().map(|c| g.mul(c, &il)).collect();
    }
    x
}
fn psq_mod(g: &Gf, a: &Poly, m: &Poly) -> Poly {
    // char 2: (sum c_k t^k)^2 = sum c_k^2 t^{2k}
    if a.is_empty() {
        return vec![];
    }
    let mut r = vec![g.zero(); 2 * a.len() - 1];
    for (k, c) in a.iter().enumerate() {
        r[2 * k] = g.sq(c);
    }
    pmodp(g, &ptrim(g, r), m)
}
fn peval(g: &Gf, p: &Poly, t: &El) -> El {
    let mut r = g.zero();
    for c in p.iter().rev() {
        r = g.add(&g.mul(&r, t), c);
    }
    r
}

/// distinct roots in F_q of squarefree-or-not poly f (roots of
/// gcd(f, t^q - t)); trace-splitting over F_q
fn roots_fq(g: &Gf, f: &Poly, d: usize, rng: &mut Rng) -> Vec<El> {
    let f = ptrim(g, f.clone());
    if f.len() <= 1 {
        return vec![];
    }
    let tpoly: Poly = vec![g.zero(), g.one()];
    // h = t^(2^d) mod f
    let mut h = pmodp(g, &tpoly, &f);
    for _ in 0..d {
        h = psq_mod(g, &h, &f);
    }
    let gg = pgcd(g, &f, &paddp(g, &h, &tpoly));
    let mut out = vec![];
    split(g, &gg, d, rng, &mut out);
    out
}
fn split(g: &Gf, f: &Poly, d: usize, rng: &mut Rng, out: &mut Vec<El>) {
    if f.len() <= 1 {
        return;
    }
    if f.len() == 2 {
        // monic: t + r  -> root r (char 2)
        let il = g.inv(&f[1]);
        out.push(g.mul(&f[0], &il));
        return;
    }
    for _ in 0..200 {
        let beta = rng.fq(g, d);
        if g.is_zero(&beta) {
            continue;
        }
        // S = Tr_{F_q/F_2}(beta t) mod f = sum_{j<d} (beta t)^(2^j)
        let bt: Poly = vec![g.zero(), beta];
        let mut z = pmodp(g, &bt, f);
        let mut s: Poly = z.clone();
        for _ in 1..d {
            z = psq_mod(g, &z, f);
            s = paddp(g, &s, &z);
        }
        let g1 = pgcd(g, f, &s);
        if g1.len() > 1 && g1.len() < f.len() {
            // quotient: f / g1 via polynomial division
            let g2 = pdiv(g, f, &g1);
            split(g, &g1, d, rng, out);
            split(g, &g2, d, rng, out);
            return;
        }
    }
    panic!("splitting failed");
}
fn pdiv(g: &Gf, a: &Poly, m: &Poly) -> Poly {
    let mut r = a.clone();
    let dm = m.len() - 1;
    let il = g.inv(&m[dm]);
    let mut q = vec![g.zero(); a.len().saturating_sub(dm)];
    while r.len() > dm {
        let dr = r.len() - 1;
        let f = g.mul(&r[dr], &il);
        q[dr - dm] = f.clone();
        for j in 0..=dm {
            let t = g.mul(&f, &m[j]);
            r[dr - dm + j] = g.add(&r[dr - dm + j], &t);
        }
        r = ptrim(g, r);
    }
    assert!(r.is_empty());
    ptrim(g, q)
}

// ---------------- the search ----------------

struct Found {
    u: El,
    t_u_zero: bool,
    t_cu_zero: bool,
}

struct Search {
    lines: usize,
    lines_to_first: usize,
    interp_checks_ok: bool,
    roots_total: usize,
    found: Vec<Found>,
    secs_to_first: f64,
    secs_total: f64,
    max_deg_seen: usize,
}

fn direct_g(g: &Gf, c: &El, u: &El, d: usize, n: usize) -> El {
    let mut prod = c.clone();
    for i in 1..n {
        prod = g.mul(&prod, &g.frob(u, d * i));
    }
    g.tr_rel(&prod, d)
}

fn search(g: &Gf, c: &El, d: usize, n: usize, want: usize, max_lines: usize, rng: &mut Rng) -> Search {
    let t0 = Instant::now();
    let mut s = Search {
        lines: 0,
        lines_to_first: 0,
        interp_checks_ok: true,
        roots_total: 0,
        found: vec![],
        secs_to_first: f64::NAN,
        secs_total: 0.0,
        max_deg_seen: 0,
    };
    let kerel = |rng: &mut Rng| {
        let z = rng.element(g);
        g.add(&z, &g.frob(&z, d))
    };
    while s.found.len() < want && s.lines < max_lines {
        s.lines += 1;
        let (u0, u1) = (kerel(rng), kerel(rng));
        if g.is_zero(&u0) || g.is_zero(&u1) {
            continue;
        }
        let ratio = g.mul(&u0, &g.inv(&u1));
        if g.frob(&ratio, d) == ratio {
            continue; // F_q-dependent: degenerate line
        }
        // coef polynomial of c * prod_{i=1}^{n-1} (u0^{q^i} + t u1^{q^i})
        let mut poly: Poly = vec![c.clone()];
        for i in 1..n {
            let lin: Poly = vec![g.frob(&u0, d * i), g.frob(&u1, d * i)];
            poly = pmulp(g, &poly, &lin);
        }
        while poly.len() < n {
            poly.push(g.zero());
        }
        let pp: Poly = ptrim(g, poly.iter().map(|co| g.tr_rel(co, d)).collect());
        // check against direct evaluation of G at 3 random t in F_q
        for _ in 0..3 {
            let t = rng.fq(g, d);
            let u = g.add(&u0, &g.mul(&t, &u1));
            if direct_g(g, c, &u, d, n) != peval(g, &pp, &t) {
                s.interp_checks_ok = false;
            }
        }
        s.max_deg_seen = s.max_deg_seen.max(pp.len().saturating_sub(1));
        let roots = if pp.is_empty() { vec![rng.fq(g, d)] } else { roots_fq(g, &pp, d, rng) };
        for r in roots {
            let u = g.add(&u0, &g.mul(&r, &u1));
            if g.is_zero(&u) {
                continue;
            }
            // direct verification, not using the derivation
            let tu = g.tr_rel(&u, d);
            let tcu = g.tr_rel(&g.mul(c, &g.inv(&u)), d);
            let f = Found { u, t_u_zero: g.is_zero(&tu), t_cu_zero: g.is_zero(&tcu) };
            s.roots_total += 1;
            if f.t_u_zero && f.t_cu_zero {
                if s.found.is_empty() {
                    s.lines_to_first = s.lines;
                    s.secs_to_first = t0.elapsed().as_secs_f64();
                }
                if s.found.len() < want {
                    s.found.push(f);
                }
            } else {
                // a root that fails verification is recorded as a failure
                s.interp_checks_ok = false;
            }
        }
    }
    s.secs_total = t0.elapsed().as_secs_f64();
    s
}

struct Curve {
    name: String,
    source: String,
    g: Gf,
    a: El,
    b: El,
    d: usize,
    n: usize,
    order: El,
    cofactor: u64,
    gen_x: Option<El>,
}

fn run_curve(cv: &Curve, samples: usize, max_lines: usize) -> String {
    let g = &cv.g;
    let (d, n) = (cv.d, cv.n);
    assert_eq!(d * n, g.n);
    let mut rng = Rng(MASTER ^ (g.n as u64).wrapping_mul(0x1000_0000_01B3) ^ (d as u64));
    let t_all = Instant::now();
    let (fro, gcd1, per) = irreducible(g);
    let c = g.frob(&cv.b, g.n - 1);
    assert_eq!(g.sq(&c), cv.b);
    let tra = g.tr(&cv.a);
    // group order check: [order * cofactor] P = O for the generator x (if
    // given) and 3 random points; also [order] P for cofactor reporting.
    let mut ord_ok = vec![];
    let mut pts: Vec<Pt> = vec![];
    if let Some(gx) = &cv.gen_x {
        let gp = point_from_x(g, &cv.a, &cv.b, gx); assert!(gp.is_some(), "generator x does not lift"); pts.push(gp);
    }
    while pts.len() < 4 {
        let x = rng.element(g);
        if g.is_zero(&x) {
            continue;
        }
        if let Some(p) = point_from_x(g, &cv.a, &cv.b, &x) {
            pts.push(Some(p));
        }
    }
    let mut hasse_ok = true;
    for p in &pts {
        assert!(on_curve(g, &cv.a, &cv.b, p));
        let op = pmul_scalar(g, &cv.a, p, &cv.order);
        let full = pmul_scalar(g, &cv.a, &op, &[cv.cofactor]);
        ord_ok.push((op.is_none(), full.is_none()));
    }
    // Hasse: |h*order - (2^N+1)| <= 2^(N/2+1)  (checked in top bits only)
    {
        let h = cv.cofactor;
        let hb = degree(&cv.order).unwrap() + 1 + (64 - h.leading_zeros() as usize) - 1;
        if (hb as i64 - g.n as i64).abs() > 1 {
            hasse_ok = false;
        }
    }
    let srch = search(g, &c, d, n, 3, max_lines, &mut rng);
    // lift sampling
    let sample_y = |rng: &mut Rng| loop {
        let y = rng.fq(g, d);
        if !g.is_zero(&y) {
            break y;
        }
    };
    let mut tw: Vec<(String, usize)> = vec![];
    for f in &srch.found {
        let mut h = 0;
        for _ in 0..samples {
            let y = sample_y(&mut rng);
            h += lift_ok(g, &cv.a, &cv.b, &g.mul(&f.u, &y)) as usize;
        }
        tw.push((hex_of(&f.u), h));
    }
    let mut std_hits = 0usize;
    for _ in 0..samples {
        let y = sample_y(&mut rng);
        std_hits += lift_ok(g, &cv.a, &cv.b, &y) as usize;
    }
    // control: random u in ker T that is not a root (T(c/u) != 0)
    let (mut ctl_hits, mut ctl_done, mut ctl_tcu_zero) = (0usize, false, false);
    for _ in 0..50 {
        let z = rng.element(g);
        let u = g.add(&z, &g.frob(&z, d));
        if g.is_zero(&u) {
            continue;
        }
        let tcu = g.tr_rel(&g.mul(&c, &g.inv(&u)), d);
        ctl_tcu_zero = g.is_zero(&tcu);
        for _ in 0..samples {
            let y = sample_y(&mut rng);
            ctl_hits += lift_ok(g, &cv.a, &cv.b, &g.mul(&u, &y)) as usize;
        }
        ctl_done = true;
        break;
    }
    let tw_json: Vec<String> = srch
        .found
        .iter()
        .zip(&tw)
        .map(|(f, (h, c))| {
            format!(
                "{{\"u_hex\":\"{h}\",\"T_u_zero\":{},\"T_c_over_u_zero\":{},\"sampled_lift_twisted\":{c}}}",
                f.t_u_zero, f.t_cu_zero
            )
        })
        .collect();
    let per_j: Vec<String> = per.iter().map(|(p, ok)| format!("{{\"p\":{p},\"gcd_is_1\":{ok}}}")).collect();
    let ord_j: Vec<String> = ord_ok
        .iter()
        .map(|(a, b)| format!("{{\"order_P_is_O\":{a},\"order_times_cofactor_P_is_O\":{b}}}"))
        .collect();
    let row = format!(
        "{{\"curve\":\"{}\",\"source\":\"{}\",\"N\":{},\"d\":{d},\"n\":{n},\"b_hex\":\"{}\",\"a_hex\":\"{}\",\"field_poly_hex\":\"{}\",\"order_hex\":\"{}\",\"cofactor\":{},\"Tr_N_a\":{tra},\"field_x_pow_2N_eq_x\":{fro},\"field_gcd_conditions_all_one\":{gcd1},\"field_gcd_per_prime\":[{}],\"group_order_checks\":[{}],\"order_bitlength_rough_check\":{hasse_ok},\"lines_tried\":{},\"lines_to_first_u\":{},\"max_degree_P_seen\":{},\"roots_found_total\":{},\"polynomial_vs_direct_G_and_root_verification_ok\":{},\"u_found\":{},\"twisted\":[{}],\"samples\":{samples},\"sampled_lift_standard\":{std_hits},\"control_random_kerT_u\":{{\"done\":{ctl_done},\"T_c_over_u_zero\":{ctl_tcu_zero},\"sampled_lift\":{ctl_hits}}},\"secs_search_to_first_u\":{:.3},\"secs_search_total\":{:.3},\"secs_curve_total\":{:.3},\"seed\":\"0x{:X}\"}}",
        cv.name,
        cv.source,
        g.n,
        hex_of(&cv.b),
        hex_of(&cv.a),
        hex_of(&g.poly),
        hex_of(&cv.order),
        cv.cofactor,
        per_j.join(","),
        ord_j.join(","),
        srch.lines,
        srch.lines_to_first,
        srch.max_deg_seen,
        srch.roots_total,
        srch.interp_checks_ok,
        !srch.found.is_empty(),
        tw_json.join(","),
        srch.secs_to_first,
        srch.secs_total,
        t_all.elapsed().as_secs_f64(),
        MASTER
    );
    eprintln!("{row}");
    row
}

fn poly_from_exps(exps: &[usize]) -> El {
    let top = *exps.iter().max().unwrap();
    let mut p = vec![0u64; top / 64 + 1];
    for &e in exps {
        p[e / 64] |= 1 << (e % 64);
    }
    p
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("c2pnb params file");
    let out = &args[2];
    let samples: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);
    let max_lines: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(2000);
    let mut curves: Vec<Curve> = vec![];
    // RFC 2409 sections 6.3, 6.4 (text fetched from rfc-editor.org; the repo
    // copy is research/ghs_oakley_extended_20261023/REPORT.md)
    for (name, nn, mid, bhex, ohex, gx, modhex) in [
        ("oakley3_gf2_155", 155usize, 62usize, "07338f", "0800000000000000000057db5698537193aef944", "7b", "0800000000000000000000004000000000000001"),
        ("oakley4_gf2_185", 185, 69, "1ee9", "01ffffffffffffffffffffffdbf2f889b73e484175f94ebc", "18", "020000000000000000000000000000200000000000000001"),
    ] {
        let poly = poly_from_exps(&[nn, mid, 0]);
        let mut want = hex_to_el(modhex);
        want.resize(poly.len().max(want.len()), 0);
        let mut pp = poly.clone();
        pp.resize(want.len(), 0);
        assert_eq!(pp, want, "RFC modulus hex disagrees with u^N+u^m+1");
        let g = Gf::new(poly);
        let pad = |mut e: El| {
            e.resize(g.limbs, 0);
            e
        };
        curves.push(Curve {
            name: name.into(),
            source: "RFC 2409 sec 6.3/6.4".into(),
            a: g.zero(),
            b: pad(hex_to_el(bhex)),
            d: nn / 5,
            n: 5,
            order: hex_to_el(ohex),
            cofactor: 1,
            gen_x: Some(pad(hex_to_el(gx))),
            g,
        });
    }
    for name in ["c2pnb176v1", "c2pnb208w1", "c2pnb272w1", "c2pnb304w1", "c2pnb368w1"] {
        let block = text.split(&format!("===== {name} =====")).nth(1).unwrap().split("=====").next().unwrap();
        let g = Gf::new(field(block, "Polynomial"));
        let pad = |mut e: El| {
            e.resize(g.limbs, 0);
            e
        };
        let nn = g.n;
        curves.push(Curve {
            name: name.into(),
            source: "ANSI X9.62 params, research/lift_dense_factor_base_20261027/reports/run-20261027T0356Z/c2pnb_input_binary-curve-params.txt".into(),
            a: pad(field(block, "A")),
            b: pad(field(block, "B")),
            d: nn / 4,
            n: 4,
            order: field(block, "Order"),
            cofactor: cofactor(block),
            gen_x: None,
            g,
        });
    }
    let mut rows = vec![];
    for cv in &curves {
        rows.push(run_curve(cv, samples, max_lines));
    }
    std::fs::write(out, format!("[{}]\n", rows.join(",\n"))).unwrap();
}
