use std::collections::HashMap;
pub const QUARTIC_MIN_P: u64 = 29;
pub const QUARTIC_MAX_P: u64 = 1 << 11;

/// The largest prime the group law itself accepts (a curve whose points
/// are supplied, [`QuarticJacobian::with_points`]): products of two
/// residues fit a `u64`, and a class packs into 32-bit entries.
pub const QUARTIC_LAW_MAX_P: u64 = 1 << 31;

// ── F_p ──────────────────────────────────────────────────────────────

thread_local! {
    static MULS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// `F_p` multiplications this thread has performed in this module (an
/// inversion is its Fermat exponentiation's multiplications).  A transfer
/// reads differences of it to price Jacobian work in the source curve's
/// field-multiplication unit.
pub fn field_muls() -> u64 {
    MULS.with(|m| m.get())
}

#[inline]
pub(crate) fn mulm(a: u64, b: u64, p: u64) -> u64 {
    MULS.with(|m| m.set(m.get() + 1));
    a * b % p
}

fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1 % p;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}

#[inline]
pub(crate) fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}

// ── Linear algebra over F_p ──────────────────────────────────────────

/// Reduced row-echelon form of `rows` (length-`n` vectors), zero rows
/// dropped.
fn rref(mut rows: Vec<Vec<u64>>, n: usize, p: u64) -> Vec<Vec<u64>> {
    let mut r = 0;
    for col in 0..n {
        let Some(piv) = (r..rows.len()).find(|&i| rows[i][col] != 0) else {
            continue;
        };
        rows.swap(r, piv);
        let inv = invm(rows[r][col], p);
        for v in rows[r].iter_mut() {
            *v = mulm(*v, inv, p);
        }
        let pivot_row = rows[r].clone();
        for (i, row) in rows.iter_mut().enumerate() {
            if i != r && row[col] != 0 {
                let c = row[col];
                for (x, &y) in row.iter_mut().zip(&pivot_row) {
                    *x = (*x + p - mulm(c, y, p)) % p;
                }
            }
        }
        r += 1;
        if r == rows.len() {
            break;
        }
    }
    rows.truncate(r);
    rows
}

/// A basis of `{v : M v = 0}` for `M` with rows of length `n`, from the
/// RREF of `M`: one vector per free column.
fn nullspace(m: Vec<Vec<u64>>, n: usize, p: u64) -> Vec<Vec<u64>> {
    let e = rref(m, n, p);
    let pivots: Vec<usize> = e
        .iter()
        .map(|row| row.iter().position(|&v| v != 0).unwrap())
        .collect();
    let mut out = Vec::new();
    for free in (0..n).filter(|c| !pivots.contains(c)) {
        let mut v = vec![0u64; n];
        v[free] = 1;
        for (row, &pc) in e.iter().zip(&pivots) {
            v[pc] = (p - row[free]) % p;
        }
        out.push(v);
    }
    out
}

// ── Monomials and normal forms ───────────────────────────────────────

/// The exponents `(a, b, c)` of `X^a Y^b Z^c`, `a + b + c = n`, in a fixed
/// order: `a` descending, then `b` descending.
pub fn monomials(n: usize) -> Vec<(u8, u8, u8)> {
    let mut v = Vec::new();
    for a in (0..=n).rev() {
        for b in (0..=n - a).rev() {
            v.push((a as u8, b as u8, (n - a - b) as u8));
        }
    }
    v
}

/// The normal-form basis of `S_n`: monomials of degree `n`, `X`-degree ≤ 3.
pub fn basis(n: usize) -> Vec<(u8, u8, u8)> {
    monomials(n).into_iter().filter(|m| m.0 <= 3).collect()
}

pub fn dim(n: usize) -> usize {
    basis(n).len()
}

/// The highest degree any operation of the group law reaches.
const MAX_DEG: usize = 10;

/// Normal forms of every monomial of degree ≤ [`MAX_DEG`], and the index
/// of each normal-form basis monomial.
struct Forms {
    p: u64,
    /// `basis[n]`: the basis of `S_n`.
    basis: Vec<Vec<(u8, u8, u8)>>,
    /// `nf[n][(a, b, c)]`: the normal form of any monomial of degree `n`,
    /// as `(basis index, coefficient)` pairs.
    nf: Vec<HashMap<(u8, u8, u8), Vec<(usize, u64)>>>,
}

impl Forms {
    fn new(p: u64, f: &[u64; 15]) -> Forms {
        let mons4 = monomials(4);
        let mut basis_v = Vec::new();
        let mut index: Vec<HashMap<(u8, u8, u8), usize>> = Vec::new();
        for n in 0..=MAX_DEG {
            let b = basis(n);
            index.push(b.iter().enumerate().map(|(i, &m)| (m, i)).collect());
            basis_v.push(b);
        }
        let mut nf: Vec<HashMap<(u8, u8, u8), Vec<(usize, u64)>>> = Vec::new();
        for n in 0..=MAX_DEG {
            let mut table: HashMap<(u8, u8, u8), Vec<(usize, u64)>> = HashMap::new();
            // Increasing X-degree: X^a·rest reduces to terms of X-degree < a.
            let mut ms = monomials(n);
            ms.sort_by_key(|m| m.0);
            for m in ms {
                let v = if m.0 <= 3 {
                    vec![(index[n][&m], 1)]
                } else {
                    // X^a Y^b Z^c = X^{a−4} Y^b Z^c · X⁴, X⁴ ≡ −(F − X⁴).
                    let mut acc: HashMap<usize, u64> = HashMap::new();
                    for (k, &mm) in mons4.iter().enumerate() {
                        if mm == (4, 0, 0) || f[k] == 0 {
                            continue;
                        }
                        let prod = (m.0 - 4 + mm.0, m.1 + mm.1, m.2 + mm.2);
                        let c = (p - f[k]) % p;
                        for &(i, v) in &table[&prod] {
                            let e = acc.entry(i).or_insert(0);
                            *e = (*e + mulm(c, v, p)) % p;
                        }
                    }
                    let mut v: Vec<(usize, u64)> = acc.into_iter().filter(|e| e.1 != 0).collect();
                    v.sort_unstable();
                    v
                };
                table.insert(m, v);
            }
            nf.push(table);
        }
        Forms {
            p,
            basis: basis_v,
            nf,
        }
    }

    fn dim(&self, n: usize) -> usize {
        self.basis[n].len()
    }

    /// `u·v` for `u ∈ S_a`, `v ∈ S_b`, in `S_{a+b}`.
    fn mul(&self, u: &[u64], a: usize, v: &[u64], b: usize) -> Vec<u64> {
        let p = self.p;
        let n = a + b;
        let mut out = vec![0u64; self.dim(n)];
        for (i, &ui) in u.iter().enumerate() {
            if ui == 0 {
                continue;
            }
            let mi = self.basis[a][i];
            for (j, &vj) in v.iter().enumerate() {
                if vj == 0 {
                    continue;
                }
                let mj = self.basis[b][j];
                let c = mulm(ui, vj, p);
                for &(k, w) in &self.nf[n][&(mi.0 + mj.0, mi.1 + mj.1, mi.2 + mj.2)] {
                    out[k] = (out[k] + mulm(c, w, p)) % p;
                }
            }
        }
        out
    }

    /// The span of all products, reduced.
    fn product(&self, us: &[Vec<u64>], a: usize, vs: &[Vec<u64>], b: usize) -> Vec<Vec<u64>> {
        let mut rows = Vec::with_capacity(us.len() * vs.len());
        for u in us {
            for v in vs {
                rows.push(self.mul(u, a, v, b));
            }
        }
        rref(rows, self.dim(a + b), self.p)
    }

    /// `{t ∈ S_a : ann·(t·u) = 0 ∀ u ∈ us}`, `ann` the annihilator rows of
    /// a subspace of `S_{a+b}` and `us ⊂ S_b`.
    fn colon(&self, ann: &[Vec<u64>], us: &[Vec<u64>], a: usize, b: usize) -> Vec<Vec<u64>> {
        let p = self.p;
        let da = self.dim(a);
        let mut rows = Vec::with_capacity(ann.len() * us.len());
        let unit = |i: usize| -> Vec<u64> {
            let mut e = vec![0u64; da];
            e[i] = 1;
            e
        };
        for u in us {
            // Column i: the product of the i-th basis monomial with u.
            let cols: Vec<Vec<u64>> = (0..da).map(|i| self.mul(&unit(i), a, u, b)).collect();
            for arow in ann {
                let row: Vec<u64> = cols
                    .iter()
                    .map(|c| {
                        c.iter()
                            .zip(arow)
                            .fold(0u64, |s, (&x, &y)| (s + mulm(x, y, p)) % p)
                    })
                    .collect();
                rows.push(row);
            }
        }
        nullspace(rows, da, p)
    }

    /// The annihilator rows of a subspace of `S_n`.
    fn ann(&self, rows: &[Vec<u64>], n: usize) -> Vec<Vec<u64>> {
        nullspace(rows.to_vec(), self.dim(n), self.p)
    }

    /// The evaluation functional of `S_n` at the projective point `pt`.
    fn eval_row(&self, pt: [u64; 3], n: usize) -> Vec<u64> {
        let p = self.p;
        self.basis[n]
            .iter()
            .map(|&(a, b, c)| {
                mulm(
                    mulm(powm(pt[0], a.into(), p), powm(pt[1], b.into(), p), p),
                    powm(pt[2], c.into(), p),
                    p,
                )
            })
            .collect()
    }

    /// `H⁰(nK − Σ P_i)` for distinct rational points.
    fn vanishing(&self, pts: &[[u64; 3]], n: usize) -> Vec<Vec<u64>> {
        let rows = pts.iter().map(|&q| self.eval_row(q, n)).collect();
        nullspace(rows, self.dim(n), self.p)
    }

    fn full(&self, n: usize) -> Vec<Vec<u64>> {
        (0..self.dim(n))
            .map(|i| {
                let mut e = vec![0u64; self.dim(n)];
                e[i] = 1;
                e
            })
            .collect()
    }
}

// ── The quartic, its points and smoothness ───────────────────────────

/// Evaluate a quartic form (coefficients in [`monomials`]`(4)` order).
fn eval_form(f: &[u64], n: usize, pt: [u64; 3], p: u64) -> u64 {
    monomials(n).iter().zip(f).fold(0, |s, (&(a, b, c), &k)| {
        if k == 0 {
            return s;
        }
        let t = mulm(
            mulm(powm(pt[0], a.into(), p), powm(pt[1], b.into(), p), p),
            powm(pt[2], c.into(), p),
            p,
        );
        (s + mulm(k, t, p)) % p
    })
}

/// The partial derivatives of a quartic form, as cubic forms.
fn partials(f: &[u64; 15], p: u64) -> [Vec<u64>; 3] {
    let m4 = monomials(4);
    let m3 = monomials(3);
    let idx3: HashMap<(u8, u8, u8), usize> = m3.iter().enumerate().map(|(i, &m)| (m, i)).collect();
    let mut out = [vec![0u64; 10], vec![0u64; 10], vec![0u64; 10]];
    for (k, &(a, b, c)) in m4.iter().enumerate() {
        let e = [a, b, c];
        for v in 0..3 {
            if e[v] == 0 {
                continue;
            }
            let mut d = e;
            d[v] -= 1;
            let i = idx3[&(d[0], d[1], d[2])];
            out[v][i] = (out[v][i] + mulm(f[k], u64::from(e[v]), p)) % p;
        }
    }
    out
}

/// Polynomials over F_p, lowest degree first.
fn ptrim(mut a: Vec<u64>) -> Vec<u64> {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}

fn pgcd(mut a: Vec<u64>, mut b: Vec<u64>, p: u64) -> Vec<u64> {
    a = ptrim(a);
    b = ptrim(b);
    while !b.is_empty() {
        let r = prem(&a, &b, p);
        a = b;
        b = r;
    }
    a
}

fn prem(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let inv = invm(b[db], p);
    while r.len() > db && !r.is_empty() {
        let c = mulm(*r.last().unwrap(), inv, p);
        let s = r.len() - 1 - db;
        for (i, &bi) in b.iter().enumerate() {
            r[s + i] = (r[s + i] + p - mulm(c, bi, p)) % p;
        }
        r = ptrim(r);
    }
    r
}

fn det(mut m: Vec<Vec<u64>>, p: u64) -> u64 {
    let n = m.len();
    let mut d = 1u64;
    for col in 0..n {
        let Some(piv) = (col..n).find(|&i| m[i][col] != 0) else {
            return 0;
        };
        if piv != col {
            m.swap(piv, col);
            d = (p - d) % p;
        }
        d = mulm(d, m[col][col], p);
        let inv = invm(m[col][col], p);
        for i in col + 1..n {
            let c = mulm(m[i][col], inv, p);
            if c != 0 {
                for j in col..n {
                    m[i][j] = (m[i][j] + p - mulm(c, m[col][j], p)) % p;
                }
            }
        }
    }
    d
}

/// The Sylvester resultant of two polynomials in `y` (lowest first).
fn resultant(a: &[u64], b: &[u64], p: u64) -> u64 {
    let (m, n) = (a.len() - 1, b.len() - 1);
    let size = m + n;
    let mut s = vec![vec![0u64; size]; size];
    for i in 0..n {
        for (j, &c) in a.iter().rev().enumerate() {
            s[i][i + j] = c;
        }
    }
    for i in 0..m {
        for (j, &c) in b.iter().rev().enumerate() {
            s[n + i][i + j] = c;
        }
    }
    det(s, p)
}

/// Lagrange interpolation through `(x_i, y_i)`.
fn interpolate(xs: &[u64], ys: &[u64], p: u64) -> Vec<u64> {
    let n = xs.len();
    let mut out = vec![0u64; n];
    for i in 0..n {
        let mut num = vec![1u64];
        let mut den = 1u64;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut next = vec![0u64; num.len() + 1];
            for (k, &c) in num.iter().enumerate() {
                next[k + 1] = (next[k + 1] + c) % p;
                next[k] = (next[k] + mulm(c, (p - xs[j]) % p, p)) % p;
            }
            num = next;
            den = mulm(den, (xs[i] + p - xs[j]) % p, p);
        }
        let c = mulm(ys[i], invm(den, p), p);
        for (k, &v) in num.iter().enumerate() {
            out[k] = (out[k] + mulm(c, v, p)) % p;
        }
    }
    ptrim(out)
}

/// A cubic form's restriction to `Z = 1` as a polynomial in `y` whose
/// coefficients are evaluated at `x`.
fn cubic_in_y(g: &[u64], x: u64, p: u64) -> Vec<u64> {
    let mut out = vec![0u64; 4];
    for (k, &(a, b, _)) in monomials(3).iter().enumerate() {
        out[b as usize] = (out[b as usize] + mulm(g[k], powm(x, a.into(), p), p)) % p;
    }
    out
}

/// Whether `F = 0` is smooth over `F̄_p`, conservatively: a singular point
/// is a common zero of the three partials (Euler, `p ∤ 4`).  On `Z = 1`
/// the abscissae of common zeros divide `gcd(Res_y(F_X, F_Y), Res_y(F_X,
/// F_Z))`, which has degree ≤ 9; the `y³` coefficients of the partials are
/// constants and must be nonzero so that the resultants specialise.  On
/// `Z = 0` the binary cubics must have no common zero.  Any common root of
/// the resultants is treated as singular, so a smooth curve may be
/// refused, never a singular one accepted.
pub fn is_smooth(f: &[u64; 15], p: u64) -> bool {
    let g = partials(f, p);
    let y3 = monomials(3).iter().position(|&m| m == (0, 3, 0)).unwrap();
    if g.iter().any(|gi| gi[y3] == 0) {
        return false;
    }
    let xs: Vec<u64> = (0..13u64).collect();
    let r = |a: &[u64], b: &[u64]| -> Vec<u64> {
        let ys: Vec<u64> = xs
            .iter()
            .map(|&x| resultant(&cubic_in_y(a, x, p), &cubic_in_y(b, x, p), p))
            .collect();
        interpolate(&xs, &ys, p)
    };
    let r12 = r(&g[0], &g[1]);
    let r13 = r(&g[0], &g[2]);
    if r12.is_empty() || r13.is_empty() || pgcd(r12, r13, p).len() > 1 {
        return false;
    }
    // Z = 0: (1 : 0 : 0) and (x : 1 : 0).
    let at = |gi: &[u64], pt: [u64; 3]| eval_form(gi, 3, pt, p);
    if g.iter().all(|gi| at(gi, [1, 0, 0]) == 0) {
        return false;
    }
    let inf = |gi: &[u64]| -> Vec<u64> {
        let mut out = vec![0u64; 4];
        for (k, &(a, _, c)) in monomials(3).iter().enumerate() {
            if c == 0 {
                out[a as usize] = (out[a as usize] + gi[k]) % p;
            }
        }
        ptrim(out)
    };
    let common = pgcd(pgcd(inf(&g[0]), inf(&g[1]), p), inf(&g[2]), p);
    common.len() <= 1
}

// ── Point counting over F_p and F_{p²} ───────────────────────────────

/// `F_{p^k}` for `k ≤ 3` as `F_p[t]/(m(t))`, `m` monic of degree `k`
/// without roots in `F_p` (irreducible for `k ≤ 3`).
#[derive(Clone)]
struct Ext {
    p: u64,
    k: usize,
    /// `t^k = Σ red[i] t^i`.
    red: Vec<u64>,
}

type E = [u64; 3];

impl Ext {
    fn new(p: u64, k: usize) -> Ext {
        if k == 1 {
            return Ext { p, k, red: vec![0] };
        }
        for seed in 1u64.. {
            let mut m = vec![0u64; k + 1];
            let mut s = seed;
            for c in m.iter_mut().take(k) {
                s = splitmix64(s);
                *c = s % p;
            }
            m[k] = 1;
            let has_root = (0..p).any(|x| {
                let mut v = 0u64;
                for &c in m.iter().rev() {
                    v = (mulm(v, x, p) + c) % p;
                }
                v == 0
            });
            if !has_root {
                return Ext {
                    p,
                    k,
                    red: m[..k].iter().map(|&c| (p - c) % p).collect(),
                };
            }
        }
        unreachable!()
    }
    fn q(&self) -> u64 {
        self.p.pow(self.k as u32)
    }
    fn element(&self, mut i: u64) -> E {
        let mut e = [0u64; 3];
        for c in e.iter_mut().take(self.k) {
            *c = i % self.p;
            i /= self.p;
        }
        e
    }
    fn add(&self, a: E, b: E) -> E {
        let p = self.p;
        [(a[0] + b[0]) % p, (a[1] + b[1]) % p, (a[2] + b[2]) % p]
    }
    fn mul(&self, a: E, b: E) -> E {
        let p = self.p;
        let k = self.k;
        let mut t = [0u64; 5];
        for i in 0..k {
            for j in 0..k {
                t[i + j] = (t[i + j] + mulm(a[i], b[j], p)) % p;
            }
        }
        for d in (k..2 * k - 1).rev() {
            let c = t[d];
            t[d] = 0;
            for (i, &r) in self.red.iter().enumerate() {
                t[d - k + i] = (t[d - k + i] + mulm(c, r, p)) % p;
            }
        }
        [t[0], t[1], t[2]]
    }
    fn is_zero(&self, a: E) -> bool {
        a == [0, 0, 0]
    }
    fn inv(&self, a: E) -> E {
        // a^(q − 2).
        let mut r = self.one();
        let mut b = a;
        let mut e = self.q() - 2;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn one(&self) -> E {
        [1, 0, 0]
    }
    fn scalar(&self, c: u64) -> E {
        [c % self.p, 0, 0]
    }
    fn neg(&self, a: E) -> E {
        let p = self.p;
        [(p - a[0]) % p, (p - a[1]) % p, (p - a[2]) % p]
    }
}

/// Polynomials over `Ext`, lowest first.
fn ext_trim(e: &Ext, mut a: Vec<E>) -> Vec<E> {
    while a.last().is_some_and(|&c| e.is_zero(c)) {
        a.pop();
    }
    a
}

fn ext_mulmod(e: &Ext, a: &[E], b: &[E], m: &[E]) -> Vec<E> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut t = vec![[0u64; 3]; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            t[i + j] = e.add(t[i + j], e.mul(x, y));
        }
    }
    ext_rem(e, t, m)
}

fn ext_rem(e: &Ext, a: Vec<E>, m: &[E]) -> Vec<E> {
    let mut r = ext_trim(e, a);
    let dm = m.len() - 1;
    let inv = if m[dm] == e.one() {
        e.one()
    } else {
        e.inv(m[dm])
    };
    while r.len() > dm {
        let c = e.mul(*r.last().unwrap(), inv);
        let s = r.len() - 1 - dm;
        for (i, &mi) in m.iter().enumerate() {
            r[s + i] = e.add(r[s + i], e.neg(e.mul(c, mi)));
        }
        r = ext_trim(e, r);
    }
    r
}

/// Number of distinct roots in `F_q` of `g` (lowest first, nonzero):
/// `deg gcd(g, y^q − y)`.
fn ext_root_count(e: &Ext, g: Vec<E>) -> u64 {
    let g = ext_trim(e, g);
    if g.len() <= 1 {
        return 0;
    }
    // Monic, so that every reduction below divides by 1.
    let lead = e.inv(*g.last().unwrap());
    let g: Vec<E> = g.into_iter().map(|c| e.mul(c, lead)).collect();
    // y^q mod g by repeated squaring.
    let mut acc = vec![e.one()];
    let mut base = ext_rem(e, vec![[0, 0, 0], e.one()], &g);
    let mut q = e.q();
    while q > 0 {
        if q & 1 == 1 {
            acc = ext_mulmod(e, &acc, &base, &g);
        }
        base = ext_mulmod(e, &base, &base, &g);
        q >>= 1;
    }
    // acc − y
    let mut h = acc;
    if h.len() < 2 {
        h.resize(2, [0, 0, 0]);
    }
    h[1] = e.add(h[1], e.neg(e.one()));
    let h = ext_trim(e, h);
    if h.is_empty() {
        return (g.len() - 1) as u64;
    }
    // gcd(g, h)
    let mut a = g;
    let mut b = h;
    while !b.is_empty() {
        let r = ext_rem(e, a, &b);
        a = b;
        b = r;
    }
    (a.len() - 1) as u64
}

/// `#C(F_{p^k})`, `k ≤ 3`: for each `x ∈ F_q` the distinct roots `y` of
/// `F(x, y, 1)`, plus the points `(x : 1 : 0)` (`(1 : 0 : 0)` is never on
/// `C`: the `X⁴` coefficient is 1).
pub fn count_points(f: &[u64; 15], p: u64, k: usize) -> u64 {
    let e = Ext::new(p, k);
    let m4 = monomials(4);
    let q = e.q();
    let pow = |x: E, n: u8| -> E {
        let mut r = e.one();
        for _ in 0..n {
            r = e.mul(r, x);
        }
        r
    };
    let mut total = 0u64;
    for xi in 0..q {
        let x = e.element(xi);
        let mut g = vec![[0u64; 3]; 5];
        for (kk, &(a, b, _)) in m4.iter().enumerate() {
            if f[kk] != 0 {
                g[b as usize] = e.add(g[b as usize], e.mul(e.scalar(f[kk]), pow(x, a)));
            }
        }
        let g = ext_trim(&e, g);
        if g.is_empty() {
            total += q;
        } else {
            total += ext_root_count(&e, g);
        }
    }
    // Z = 0, Y = 1: F(x, 1, 0).
    let mut g = vec![[0u64; 3]; 5];
    for (kk, &(a, _, c)) in m4.iter().enumerate() {
        if c == 0 && f[kk] != 0 {
            g[a as usize] = e.add(g[a as usize], e.scalar(f[kk]));
        }
    }
    total + ext_root_count(&e, g)
}

// ── The Jacobian ─────────────────────────────────────────────────────

/// A divisor class, stored as the annihilator of `H⁰(3K − E) ⊂ S_3` in
/// reduced row-echelon form (3 rows of 10): a function of the class.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QClass(pub [u32; 30]);

/// The Jacobian of a smooth plane quartic, with everything the group law
/// precomputes: `D∞`, `P₁`, `H⁰(3K − D∞)`, `H⁰(4K − D∞)`, `H⁰(3K − D∞ −
/// P₁)`, `H⁰(3K − 2D∞)` and the fixed `c ∈ H⁰(6K − 2D∞)`.
pub struct QuarticJacobian {
    pub p: u64,
    pub f: [u64; 15],
    forms: Forms,
    pub d_inf: [[u64; 3]; 3],
    pub p1: [u64; 3],
    pub points: Vec<[u64; 3]>,
    r3: Vec<Vec<u64>>,
    r4: Vec<Vec<u64>>,
    r3_p1: Vec<Vec<u64>>,
    t2: Vec<Vec<u64>>,
    c: Vec<u64>,
    s3: Vec<Vec<u64>>,
    s4: Vec<Vec<u64>>,
    zero: QClass,
}

impl QuarticJacobian {
    /// Build from a smooth quartic with `X⁴` coefficient 1 and at least four
    /// rational points, three of them not collinear.
    pub fn new(p: u64, f: [u64; 15]) -> Result<QuarticJacobian, String> {
        if !(QUARTIC_MIN_P..QUARTIC_MAX_P).contains(&p) || !is_prime_u64(p) {
            return Err(format!(
                "plane quartics take a prime {QUARTIC_MIN_P} ≤ p < {QUARTIC_MAX_P}, not {p}"
            ));
        }
        Self::with_points(p, f, rational_points(&f, p))
    }

    /// Build from a smooth quartic and a list of its rational points (at
    /// least four, three of them not collinear), for a field too large to
    /// enumerate.  Every point is checked to lie on the curve.
    pub fn with_points(
        p: u64,
        f: [u64; 15],
        points: Vec<[u64; 3]>,
    ) -> Result<QuarticJacobian, String> {
        if !(QUARTIC_MIN_P..QUARTIC_LAW_MAX_P).contains(&p) || !is_prime_u64(p) {
            return Err(format!(
                "the plane-quartic group law takes a prime {QUARTIC_MIN_P} ≤ p < 2^31, not {p}"
            ));
        }
        if f[0] != 1 || f.iter().any(|&c| c >= p) {
            return Err("the quartic must have X⁴ coefficient 1 and reduced coefficients".into());
        }
        if !is_smooth(&f, p) {
            return Err("the quartic is singular, or the smoothness test cannot certify it".into());
        }
        if points
            .iter()
            .any(|&q| eval_form(&f, 4, q, p) != 0 || q == [0, 0, 0])
        {
            return Err("a supplied point is not on the curve".into());
        }
        let collinear = |a: [u64; 3], b: [u64; 3], c: [u64; 3]| {
            det(vec![a.to_vec(), b.to_vec(), c.to_vec()], p) == 0
        };
        let mut d_inf = None;
        'outer: for i in 0..points.len() {
            for j in i + 1..points.len() {
                for k in j + 1..points.len() {
                    if !collinear(points[i], points[j], points[k]) {
                        d_inf = Some([points[i], points[j], points[k]]);
                        break 'outer;
                    }
                }
            }
        }
        let d_inf = d_inf.ok_or("fewer than three non-collinear rational points")?;
        let p1 = *points
            .iter()
            .find(|q| !d_inf.contains(q))
            .ok_or("fewer than four rational points")?;
        let forms = Forms::new(p, &f);
        let r3 = forms.vanishing(&d_inf, 3);
        let r4 = forms.vanishing(&d_inf, 4);
        let mut d1 = d_inf.to_vec();
        d1.push(p1);
        let r3_p1 = forms.vanishing(&d1, 3);
        // H⁰(6K − 2D∞) = H⁰(3K − D∞)², and H⁰(3K − 2D∞) its colon by S_3.
        let r6x2 = forms.product(&r3, 3, &r3, 3);
        let ann6 = forms.ann(&r6x2, 6);
        let s3 = forms.full(3);
        let t2 = forms.colon(&ann6, &s3, 3, 3);
        let c = r6x2[0].clone();
        let s4 = forms.full(4);
        let zero_ann = rref(d_inf.iter().map(|&q| forms.eval_row(q, 3)).collect(), 10, p);
        let mut jac = QuarticJacobian {
            p,
            f,
            forms,
            d_inf,
            p1,
            points,
            r3,
            r4,
            r3_p1,
            t2,
            c,
            s3,
            s4,
            zero: QClass([0; 30]),
        };
        jac.zero = jac.pack(&zero_ann)?;
        jac.check_dims()?;
        Ok(jac)
    }

    fn check_dims(&self) -> Result<(), String> {
        let want = [
            (self.r3.len(), 7),
            (self.r4.len(), 11),
            (self.r3_p1.len(), 6),
            (self.t2.len(), 4),
        ];
        if want.iter().any(|(a, b)| a != b) {
            return Err(format!(
                "Riemann–Roch dimensions {want:?} are wrong: the curve is not a smooth quartic"
            ));
        }
        Ok(())
    }

    fn pack(&self, ann: &[Vec<u64>]) -> Result<QClass, String> {
        if ann.len() != 3 {
            return Err(format!("a class needs codimension 3, got {}", ann.len()));
        }
        let mut out = [0u32; 30];
        for (i, row) in ann.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                out[10 * i + j] = v as u32;
            }
        }
        Ok(QClass(out))
    }

    fn unpack(&self, x: &QClass) -> Vec<Vec<u64>> {
        (0..3)
            .map(|i| (0..10).map(|j| u64::from(x.0[10 * i + j])).collect())
            .collect()
    }

    /// `H⁰(3K − E)` for the class's divisor `E`.
    fn w3(&self, x: &QClass) -> Vec<Vec<u64>> {
        nullspace(self.unpack(x), 10, self.p)
    }

    pub fn zero(&self) -> QClass {
        self.zero
    }

    /// The evaluation functional of `S_3` at a point (for transfers that
    /// build a divisor's conditions themselves).
    pub fn eval_row3(&self, pt: [u64; 3]) -> Vec<u64> {
        self.forms.eval_row(pt, 3)
    }

    /// The class of `E − D∞` for an effective degree-3 divisor `E` given by
    /// three linear conditions on `S_3` cutting out `H⁰(3K − E)` (for
    /// example evaluations at its points, or the two `F_p`-conditions of a
    /// conjugate pair).  Refused unless the conditions have rank 3.
    pub fn class_from_conditions(&self, rows: Vec<Vec<u64>>) -> Result<QClass, String> {
        let ann = rref(rows, 10, self.p);
        let raw = self.pack(&ann)?;
        Ok(self.add_raw(&raw, &self.zero))
    }

    /// The class of `P + Q + R − D∞` for three distinct rational points.
    pub fn from_points(&self, pts: [[u64; 3]; 3]) -> Result<QClass, String> {
        if pts[0] == pts[1] || pts[0] == pts[2] || pts[1] == pts[2] {
            return Err("three distinct points".into());
        }
        let ann = rref(
            pts.iter().map(|&q| self.forms.eval_row(q, 3)).collect(),
            10,
            self.p,
        );
        let raw = self.pack(&ann)?;
        // Canonicalise: the sum with the identity is the canonical member.
        Ok(self.add_raw(&raw, &self.zero))
    }

    /// `x + y`; see the module documentation for the steps.
    pub fn add_raw(&self, x: &QClass, y: &QClass) -> QClass {
        let fm = &self.forms;
        let p = self.p;
        let wx = self.w3(x);
        let wy = self.w3(y);
        let p6 = fm.product(&wx, 3, &wy, 3);
        assert_eq!(p6.len(), 16, "H⁰(6K − E_x − E_y) has dimension 16");
        let f = p6[0].clone();
        let ann6 = fm.ann(&p6, 6);
        let t = fm.colon(&ann6, &self.s3, 3, 3);
        assert_eq!(t.len(), 4, "H⁰(3K − E_x − E_y) has dimension 4");
        let ann_fr3 = fm.ann(&fm.product(std::slice::from_ref(&f), 6, &self.r3, 3), 9);
        let mut q = fm.colon(&ann_fr3, &t, 6, 3);
        if q.len() >= 2 {
            let ann_fr3p1 = fm.ann(&fm.product(std::slice::from_ref(&f), 6, &self.r3_p1, 3), 9);
            q = fm.colon(&ann_fr3p1, &t, 6, 3);
        }
        assert_eq!(q.len(), 1, "the canonical member is unique");
        let h = q[0].clone();
        let ann_fr4 = fm.ann(&fm.product(&[f], 6, &self.r4, 4), 10);
        let u = fm.colon(&ann_fr4, &t, 7, 3);
        assert_eq!(u.len(), 5, "H⁰(7K − A − D∞) has dimension 5");
        let ann_hs4 = fm.ann(&fm.product(&[h], 6, &self.s4, 4), 10);
        let wg = fm.colon(&ann_hs4, &u, 3, 7);
        assert_eq!(wg.len(), 7, "H⁰(3K − G) has dimension 7");
        let ann = rref(nullspace(wg, 10, p), 10, p);
        self.pack(&ann).expect("codimension 3")
    }

    /// `−x`: one flip against the fixed divisor `B` (`div c = 2D∞ + B`).
    pub fn neg_raw(&self, x: &QClass) -> QClass {
        let fm = &self.forms;
        let p = self.p;
        let wx = self.w3(x);
        // H⁰(6K − E_x − B) = {f ∈ S_6 : f·H⁰(3K − 2D∞) ⊂ c·H⁰(3K − E_x)}.
        let ann_cw = fm.ann(&fm.product(std::slice::from_ref(&self.c), 6, &wx, 3), 9);
        let n = fm.colon(&ann_cw, &self.t2, 6, 3);
        assert!(!n.is_empty(), "H⁰(6K − E_x − B) is not zero");
        let f = n[0].clone();
        // H⁰(4K − E_x) = {t ∈ S_4 : t·S_3 ⊂ H⁰(3K − E_x)·S_4}.
        let w7 = fm.product(&wx, 3, &self.s4, 4);
        let w4 = fm.colon(&fm.ann(&w7, 7), &self.s3, 4, 3);
        // H⁰(7K − E_x − B) = {u ∈ S_7 : u·H⁰(3K − 2D∞) ⊂ c·H⁰(4K − E_x)}.
        let ann_cw4 = fm.ann(&fm.product(std::slice::from_ref(&self.c), 6, &w4, 4), 10);
        let u = fm.colon(&ann_cw4, &self.t2, 7, 3);
        // H⁰(3K − G) = {t ∈ S_3 : t·U ⊂ f·S_4}.
        let ann_fs4 = fm.ann(&fm.product(&[f], 6, &self.s4, 4), 10);
        let wg = fm.colon(&ann_fs4, &u, 3, 7);
        assert_eq!(wg.len(), 7, "H⁰(3K − G) has dimension 7");
        let ann = rref(nullspace(wg, 10, p), 10, p);
        let g = self.pack(&ann).expect("codimension 3");
        if n.len() >= 2 {
            // h⁰ = 2: G is a member of the pencil, not the canonical one.
            self.add_raw(&g, &self.zero)
        } else {
            g
        }
    }

}
pub fn rational_points(f: &[u64; 15], p: u64) -> Vec<[u64; 3]> {
    let mut out = Vec::new();
    for x in 0..p {
        for y in 0..p {
            if eval_form(f, 4, [x, y, 1], p) == 0 {
                out.push([x, y, 1]);
            }
        }
    }
    for x in 0..p {
        if eval_form(f, 4, [x, 1, 0], p) == 0 {
            out.push([x, 1, 0]);
        }
    }
    out
}
