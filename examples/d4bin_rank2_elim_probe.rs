//! Probe (research/leads_20261111/runs/B_d4bin_probes/rank2, preregistered in
//! PREREG.md there): the rank-2 elimination variant of the char-2 D4 (k = 4)
//! Frobenius-form solve, against the current Macaulay/eigen solve
//! (`solve_bin`, as `solve_d4_bin` calls it) on the same targets.
//!
//! Two `system_frobenius` components are linear in (S2, S3) once the
//! (S2^2, S3^2) part (rank 2) is eliminated:
//! `L_i = S2 A_i + S3 B_i + C_i` with `A, B, C` in F_q[s4, P].  Cramer gives
//! `S2 = N2/D`, `S3 = N3/D`; the two remaining components become bivariate
//! `F_j = D^2 Q_j(N2/D, N3/D, s4, P)`; `R(s4) = Res_P(F_1, F_2)` by
//! evaluation/interpolation; rational roots of `R`; back-substitution and a
//! check of all four components.  The line s4 = 0 (where D and N3 vanish
//! identically) is solved directly.  Every F_q multiplication and inversion
//! is counted (one each); Newton divisions count one, as the library does.
//! No library code path is changed.
//!
//! ```sh
//! RAYON_NUM_THREADS=1 cargo run --release --example d4bin_rank2_elim_probe -- 12 300
//! ```
use std::collections::{BTreeMap, HashMap};
use std::env;

use crypto_lib::cryptanalysis::gaudry_quartic_bin::{
    find_bin_curve, roots_with, solve_bin, BinSolve, BinTrace, Gf2q, GfTab, SymmetrisedBin,
    FROBENIUS_WEIGHTS,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

type Fe = u32;
type Comp = HashMap<[u8; 4], Fe>;

/// Bivariate polynomial: `c[i][j]` is the coefficient of s4^i P^j.
#[derive(Clone, Debug, Default)]
struct Bp {
    c: Vec<Vec<Fe>>,
}

impl Bp {
    fn zero() -> Bp {
        Bp { c: Vec::new() }
    }
    fn get(&self, i: usize, j: usize) -> Fe {
        self.c.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0)
    }
    fn set_add(&mut self, i: usize, j: usize, v: Fe) {
        if self.c.len() <= i {
            self.c.resize(i + 1, Vec::new());
        }
        if self.c[i].len() <= j {
            self.c[i].resize(j + 1, 0);
        }
        self.c[i][j] ^= v;
    }
    fn terms(&self) -> Vec<(usize, usize, Fe)> {
        let mut t = Vec::new();
        for (i, r) in self.c.iter().enumerate() {
            for (j, &v) in r.iter().enumerate() {
                if v != 0 {
                    t.push((i, j, v));
                }
            }
        }
        t
    }
    fn is_zero(&self) -> bool {
        self.c.iter().all(|r| r.iter().all(|&v| v == 0))
    }
    fn deg_s(&self) -> usize {
        self.terms().iter().map(|t| t.0).max().unwrap_or(0)
    }
    fn deg_p(&self) -> usize {
        self.terms().iter().map(|t| t.1).max().unwrap_or(0)
    }
    fn deg_tot(&self) -> usize {
        self.terms().iter().map(|t| t.0 + t.1).max().unwrap_or(0)
    }
    fn add(&self, o: &Bp) -> Bp {
        let mut r = self.clone();
        for (i, j, v) in o.terms() {
            r.set_add(i, j, v);
        }
        r
    }
    fn mul(&self, o: &Bp, g: &GfTab, mu: &mut u64) -> Bp {
        let mut r = Bp::zero();
        let (a, b) = (self.terms(), o.terms());
        for &(i, j, x) in &a {
            for &(k, l, y) in &b {
                r.set_add(i + k, j + l, g.mul(x, y));
            }
        }
        *mu += (a.len() * b.len()) as u64;
        r
    }
    fn scale(&self, k: Fe, g: &GfTab, mu: &mut u64) -> Bp {
        let mut r = Bp::zero();
        if k == 0 {
            return r;
        }
        let t = self.terms();
        for &(i, j, v) in &t {
            r.set_add(i, j, g.mul(v, k));
        }
        if k != 1 {
            *mu += t.len() as u64;
        }
        r
    }
    /// Square in characteristic 2: coefficientwise.
    fn square(&self, g: &GfTab, mu: &mut u64) -> Bp {
        let mut r = Bp::zero();
        let t = self.terms();
        for &(i, j, v) in &t {
            r.set_add(2 * i, 2 * j, g.mul(v, v));
        }
        *mu += t.len() as u64;
        r
    }
    /// Divide by s4^k where s4^k divides; returns k.
    fn strip_s(&mut self) -> usize {
        let mut k = 0;
        while !self.is_zero() && self.c.first().is_none_or(|r| r.iter().all(|&v| v == 0)) {
            self.c.remove(0);
            k += 1;
        }
        k
    }
    /// Univariate in P at s4 = x (Horner per P-degree).
    fn at_s(&self, x: Fe, g: &GfTab, mu: &mut u64) -> Vec<Fe> {
        let dp = self.deg_p();
        let ds = self.c.len();
        let mut out = vec![0u32; dp + 1];
        for (j, o) in out.iter_mut().enumerate() {
            let mut acc = 0u32;
            let mut started = false;
            for i in (0..ds).rev() {
                if started {
                    acc = g.mul(acc, x);
                    *mu += 1;
                }
                let v = self.get(i, j);
                if v != 0 || started {
                    acc ^= v;
                    started = true;
                }
            }
            *o = acc;
        }
        ptrim(&mut out);
        out
    }
    fn eval(&self, x: Fe, y: Fe, g: &GfTab, mu: &mut u64) -> Fe {
        let u = self.at_s(x, g, mu);
        horner(&u, y, g, mu)
    }
}

fn horner(u: &[Fe], y: Fe, g: &GfTab, mu: &mut u64) -> Fe {
    let mut acc = 0u32;
    for (k, &v) in u.iter().enumerate().rev() {
        if k + 1 < u.len() {
            acc = g.mul(acc, y);
            *mu += 1;
        }
        acc ^= v;
    }
    acc
}

fn ptrim(a: &mut Vec<Fe>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn deg(a: &[Fe]) -> Option<usize> {
    a.iter().rposition(|&v| v != 0)
}

fn powc(g: &GfTab, a: Fe, e: usize, mu: &mut u64) -> Fe {
    let mut r = 1u32;
    for _ in 0..e {
        r = g.mul(r, a);
    }
    *mu += e as u64;
    r
}

/// `a mod b` (b nonzero, not necessarily monic): returns the remainder.
fn pmod_gen(g: &GfTab, a: &[Fe], b: &[Fe], mu: &mut u64) -> Vec<Fe> {
    let db = deg(b).expect("b nonzero");
    let lbi = g.inv(b[db]);
    *mu += 1;
    let mut r = a.to_vec();
    ptrim(&mut r);
    while r.len() > db {
        let c = g.mul(*r.last().unwrap(), lbi);
        *mu += 1;
        let shift = r.len() - 1 - db;
        for (i, &bi) in b[..=db].iter().enumerate() {
            if bi != 0 {
                r[shift + i] ^= g.mul(c, bi);
                *mu += 1;
            }
        }
        ptrim(&mut r);
    }
    r
}

/// Resultant with formal degrees (n, m) in characteristic 2 (no signs).
fn res(g: &GfTab, a: &[Fe], n: usize, b: &[Fe], m: usize, mu: &mut u64) -> Fe {
    let (da, db) = (deg(a), deg(b));
    match (da, db) {
        (None, _) | (_, None) => return 0,
        _ => {}
    }
    let (da, db) = (da.unwrap(), db.unwrap());
    if da < n && db < m {
        return 0;
    }
    if db < m {
        let f = powc(g, a[n], m - db, mu);
        let r = res(g, a, n, b, db, mu);
        *mu += 1;
        return g.mul(f, r);
    }
    if da < n {
        let f = powc(g, b[m], n - da, mu);
        let r = res(g, a, da, b, m, mu);
        *mu += 1;
        return g.mul(f, r);
    }
    if m == 0 {
        return powc(g, b[0], n, mu);
    }
    if n == 0 {
        return powc(g, a[0], m, mu);
    }
    if n < m {
        return res(g, b, m, a, n, mu);
    }
    // Res(a, b) = Res(b, a) = lc(b)^(n - k) Res(b, a mod b), k = deg(a mod b).
    let r = pmod_gen(g, a, b, mu);
    let Some(k) = deg(&r) else {
        return 0;
    };
    let f = powc(g, b[m], n - k, mu);
    let s = res(g, b, m, &r, k, mu);
    *mu += 1;
    g.mul(f, s)
}

fn pgcd_gen(g: &GfTab, a: &[Fe], b: &[Fe], mu: &mut u64) -> Vec<Fe> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let r = pmod_gen(g, &a, &b, mu);
        a = b;
        b = r;
    }
    a
}

/// Newton interpolation (one multiplication per division, as the library's
/// `newton_interpolate`: the node-difference inverses are fixed constants).
fn newton(g: &GfTab, xs: &[Fe], vals: &[Fe], mu: &mut u64) -> Vec<Fe> {
    let n = xs.len();
    let mut dd = vals.to_vec();
    for lvl in 1..n {
        for i in (lvl..n).rev() {
            let num = dd[i] ^ dd[i - 1];
            dd[i] = g.mul(num, g.inv(xs[i] ^ xs[i - lvl]));
        }
        *mu += (n - lvl) as u64;
    }
    let mut poly = vec![0u32; n];
    for i in (0..n).rev() {
        let mut nxt = vec![0u32; n];
        for d in 0..n - 1 {
            if poly[d] != 0 {
                nxt[d + 1] ^= poly[d];
                nxt[d] ^= g.mul(poly[d], xs[i]);
            }
        }
        *mu += (n - 1 - i) as u64;
        nxt[0] ^= dd[i];
        poly = nxt;
    }
    ptrim(&mut poly);
    poly
}

/// Verification as the library's `eval` charges it (one multiplication per
/// exponent unit per monomial).
fn eval_comp(g: &GfTab, comp: &Comp, e: &[Fe; 4], mu: &mut u64) -> Fe {
    let mut acc = 0u32;
    for (m, &c) in comp {
        let mut t = c;
        for k in 0..4 {
            for _ in 0..m[k] {
                t = g.mul(t, e[k]);
                *mu += 1;
            }
        }
        acc ^= t;
    }
    acc
}

#[derive(Default, Clone, Debug)]
struct ElimCost {
    reduce: u64,
    build: u64,
    resultant: u64,
    interp: u64,
    roots: u64,
    backsub: u64,
    s4zero: u64,
    verify: u64,
}

impl ElimCost {
    fn total(&self) -> u64 {
        self.reduce
            + self.build
            + self.resultant
            + self.interp
            + self.roots
            + self.backsub
            + self.s4zero
            + self.verify
    }
}

#[derive(Default, Debug)]
struct ElimInfo {
    deg_f: [(usize, usize, usize); 2],
    strip_f: [usize; 2],
    npts: usize,
    deg_r: usize,
    strip_r: usize,
    rat_roots: usize,
    flags: Vec<String>,
}

/// Split a component into the (S2, S3) pattern parts.
fn parts(c: &Comp) -> Result<BTreeMap<(u8, u8), Bp>, String> {
    let mut out: BTreeMap<(u8, u8), Bp> = BTreeMap::new();
    for (k, &v) in c {
        let ab = (k[0], k[1]);
        if !matches!(ab, (0, 0) | (1, 0) | (0, 1) | (2, 0) | (0, 2)) {
            return Err(format!("unexpected (S2,S3) pattern {ab:?}"));
        }
        out.entry(ab)
            .or_insert_with(Bp::zero)
            .set_add(k[2] as usize, k[3] as usize, v);
    }
    Ok(out)
}

fn part(p: &BTreeMap<(u8, u8), Bp>, k: (u8, u8)) -> Bp {
    p.get(&k).cloned().unwrap_or_else(Bp::zero)
}

/// The rank-2 elimination solve.  `Err` when a structural assumption fails.
fn solve_elim(
    f: &Gf2q,
    comps: &[Comp],
    frob: bool,
    cost: &mut ElimCost,
    info: &mut ElimInfo,
) -> Result<Vec<[Fe; 4]>, String> {
    let g = &f.tab;
    // 1. Eliminate the (S2^2, S3^2) part: Q1 (S2^2 pivot), Q2 (S3^2 pivot), L1, L2.
    let mut fs: Vec<Comp> = comps.to_vec();
    let mut piv_rows = Vec::new();
    for key in [[2u8, 0, 0, 0], [0, 2, 0, 0]] {
        let Some(p) =
            (0..4).find(|&i| !piv_rows.contains(&i) && fs[i].get(&key).is_some_and(|&v| v != 0))
        else {
            return Err("quadratic part rank < 2".into());
        };
        let pinv = g.inv(fs[p][&key]);
        cost.reduce += 1;
        let pr = fs[p].clone();
        for i in 0..4 {
            if i == p {
                continue;
            }
            let Some(&c) = fs[i].get(&key) else {
                continue;
            };
            if c == 0 {
                continue;
            }
            let k = g.mul(c, pinv);
            cost.reduce += 1 + pr.len() as u64;
            for (&m, &v) in &pr {
                *fs[i].entry(m).or_insert(0) ^= g.mul(k, v);
            }
            fs[i].retain(|_, v| *v != 0);
        }
        piv_rows.push(p);
    }
    let lin: Vec<usize> = (0..4).filter(|i| !piv_rows.contains(i)).collect();
    let q = [parts(&fs[piv_rows[0]])?, parts(&fs[piv_rows[1]])?];
    let l = [parts(&fs[lin[0]])?, parts(&fs[lin[1]])?];
    for li in &l {
        if li.contains_key(&(2, 0)) || li.contains_key(&(0, 2)) {
            return Err("linear row kept a square".into());
        }
    }
    // 2. Cramer pieces.
    let (a1, b1, c1) = (
        part(&l[0], (1, 0)),
        part(&l[0], (0, 1)),
        part(&l[0], (0, 0)),
    );
    let (a2, b2, c2) = (
        part(&l[1], (1, 0)),
        part(&l[1], (0, 1)),
        part(&l[1], (0, 0)),
    );
    let mu = &mut cost.build;
    let d = a1.mul(&b2, g, mu).add(&a2.mul(&b1, g, mu));
    let n2 = b1.mul(&c2, g, mu).add(&b2.mul(&c1, g, mu));
    let n3 = a1.mul(&c2, g, mu).add(&a2.mul(&c1, g, mu));
    let (n2s, n3s, ds) = (n2.square(g, mu), n3.square(g, mu), d.square(g, mu));
    let mut ff: Vec<Bp> = Vec::new();
    for (j, qj) in q.iter().enumerate() {
        let c20 = part(qj, (2, 0)).get(0, 0);
        let c02 = part(qj, (0, 2)).get(0, 0);
        if part(qj, (2, 0)).terms().len() > 1 || part(qj, (0, 2)).terms().len() > 1 {
            return Err("square coefficient not constant".into());
        }
        let lin_part = n2
            .mul(&part(qj, (1, 0)), g, mu)
            .add(&n3.mul(&part(qj, (0, 1)), g, mu));
        let mut fj = n2s
            .scale(c20, g, mu)
            .add(&n3s.scale(c02, g, mu))
            .add(&lin_part.mul(&d, g, mu))
            .add(&ds.mul(&part(qj, (0, 0)), g, mu));
        info.strip_f[j] = fj.strip_s();
        info.deg_f[j] = (fj.deg_s(), fj.deg_p(), fj.deg_tot());
        ff.push(fj);
    }
    if ff.iter().any(|x| x.is_zero()) {
        return Err("F_j vanishes identically".into());
    }
    // 3. R(s4) = Res_P(F1, F2): evaluation at deg bound + 1 points.
    let (n, m) = (ff[0].deg_p(), ff[1].deg_p());
    let bound = ff[0].deg_s() * m + ff[1].deg_s() * n;
    let bound = bound.min(info.deg_f[0].2 * info.deg_f[1].2);
    let npts = bound + 1;
    info.npts = npts;
    if npts as u32 >= f.q {
        return Err("too many evaluation points".into());
    }
    let xs: Vec<Fe> = (0..npts as Fe).collect();
    let mut vals = Vec::with_capacity(npts);
    for &x in &xs {
        let mu = &mut cost.resultant;
        let u = ff[0].at_s(x, g, mu);
        let v = ff[1].at_s(x, g, mu);
        vals.push(res(g, &u, n, &v, m, mu));
    }
    let mut r = newton(g, &xs, &vals, &mut cost.interp);
    if r.is_empty() {
        return Err("resultant vanishes identically (common component)".into());
    }
    let mut k = 0;
    while r.first() == Some(&0) {
        r.remove(0);
        k += 1;
    }
    info.strip_r = k;
    info.deg_r = r.len() - 1;
    let mut s4s = if r.len() >= 2 {
        roots_with(f, &r, &mut cost.roots, frob)
    } else {
        Vec::new()
    };
    s4s.retain(|&x| x != 0);
    info.rat_roots = s4s.len();
    // 4. Back-substitution for s4 != 0.
    let mut cands: Vec<[Fe; 4]> = Vec::new();
    for &x in &s4s {
        let mu = &mut cost.backsub;
        let u = ff[0].at_s(x, g, mu);
        let v = ff[1].at_s(x, g, mu);
        let h = pgcd_gen(g, &u, &v, mu);
        if h.is_empty() {
            info.flags
                .push(format!("F1, F2 vanish on the line s4 = {x}"));
            continue;
        }
        if h.len() < 2 {
            continue;
        }
        for p in roots_with(f, &h, mu, frob) {
            let dv = d.eval(x, p, g, mu);
            if dv != 0 {
                let di = g.inv(dv);
                let s2 = g.mul(n2.eval(x, p, g, mu), di);
                let s3 = g.mul(n3.eval(x, p, g, mu), di);
                *mu += 3;
                cands.push([s2, s3, x, p]);
            } else {
                // D = 0: solve the rank <= 1 linear system and the squares.
                let rows = [
                    [
                        a1.eval(x, p, g, mu),
                        b1.eval(x, p, g, mu),
                        c1.eval(x, p, g, mu),
                    ],
                    [
                        a2.eval(x, p, g, mu),
                        b2.eval(x, p, g, mu),
                        c2.eval(x, p, g, mu),
                    ],
                ];
                match line_solutions(f, &rows, &q, x, p, frob, mu) {
                    Ok(v) => cands.extend(v),
                    Err(e) => info.flags.push(e),
                }
            }
        }
    }
    // 5. The line s4 = 0: L_i = S3 B_i(0,P) + C_i(0,P); Q's S2 enters only as S2^2 there.
    {
        let mu = &mut cost.s4zero;
        let row0 = |bp: &Bp| -> Vec<Fe> {
            let mut v = bp.c.first().cloned().unwrap_or_default();
            ptrim(&mut v);
            v
        };
        let (b10, c10, b20, c20) = (row0(&b1), row0(&c1), row0(&b2), row0(&c2));
        let a0_nonzero = !row0(&a1).is_empty() || !row0(&a2).is_empty();
        if a0_nonzero {
            info.flags.push("A_i(0,P) != 0".into());
        }
        let mut n20 = row0(&n2);
        ptrim(&mut n20);
        let ps: Vec<Fe> = if n20.is_empty() {
            info.flags
                .push("N2(0,P) == 0: exhaustive P on s4 = 0".into());
            (0..f.q).collect()
        } else if n20.len() >= 2 {
            roots_with(f, &n20, mu, frob)
        } else {
            Vec::new()
        };
        for p in ps {
            let rows = [
                [0, horner(&b10, p, g, mu), horner(&c10, p, g, mu)],
                [0, horner(&b20, p, g, mu), horner(&c20, p, g, mu)],
            ];
            match line_solutions(f, &rows, &q, 0, p, frob, mu) {
                Ok(v) => cands.extend(v),
                Err(e) => info.flags.push(e),
            }
        }
    }
    // 6. Verify every candidate on all four original components.
    let mut out = Vec::new();
    for e in cands {
        if comps
            .iter()
            .all(|c| eval_comp(g, c, &e, &mut cost.verify) == 0)
        {
            out.push(e);
        }
    }
    out.sort_unstable();
    out.dedup();
    Ok(out)
}

/// Solutions (S2, S3) at a fixed (s4, P) = (x, p) where the 2 x 2 system
/// `rows` = [A B | C] is singular: the rank-1 line, intersected with Q1, Q2.
fn line_solutions(
    f: &Gf2q,
    rows: &[[Fe; 3]; 2],
    q: &[BTreeMap<(u8, u8), Bp>; 2],
    x: Fe,
    p: Fe,
    frob: bool,
    mu: &mut u64,
) -> Result<Vec<[Fe; 4]>, String> {
    let g = &f.tab;
    // Q_j at (x, p) as a polynomial in (S2, S3): c20 S2^2 + c02 S3^2 + c10 S2 + c01 S3 + c00.
    let qv: Vec<[Fe; 5]> = q
        .iter()
        .map(|qj| {
            let mut e = |k| part(qj, k).eval(x, p, g, mu);
            [e((2, 0)), e((0, 2)), e((1, 0)), e((0, 1)), e((0, 0))]
        })
        .collect();
    let Some(row) = rows.iter().find(|r| r[0] != 0 || r[1] != 0) else {
        if rows.iter().any(|r| r[2] != 0) {
            return Ok(Vec::new());
        }
        return Err(format!("both linear rows vanish at ({x},{p})"));
    };
    // Consistency of the other row is left to the final verification.
    // Parametrise: if row[0] != 0, S2 = (row[2] + row[1] t)/row[0], S3 = t; else S3 = row[2]/row[1], S2 = t.
    let mut out = Vec::new();
    // Univariate in t from Q_j: coefficients [k0, k1, k2].
    let (a, b, c) = (row[0], row[1], row[2]);
    let uni: Vec<[Fe; 3]> = qv
        .iter()
        .map(|v| {
            if a != 0 {
                let ai = g.inv(a);
                let (u0, u1) = (g.mul(c, ai), g.mul(b, ai)); // S2 = u0 + u1 t
                *mu += 3;
                // S2^2 = u0^2 + u1^2 t^2
                let k2 = g.mul(v[0], g.mul(u1, u1)) ^ v[1];
                let k1 = g.mul(v[2], u1) ^ v[3];
                let k0 = g.mul(v[0], g.mul(u0, u0)) ^ g.mul(v[2], u0) ^ v[4];
                *mu += 7;
                [k0, k1, k2]
            } else {
                let s3 = g.mul(c, g.inv(b));
                *mu += 2;
                let k2 = v[0];
                let k1 = v[2];
                let k0 = g.mul(v[1], g.mul(s3, s3)) ^ g.mul(v[3], s3) ^ v[4];
                *mu += 3;
                [k0, k1, k2]
            }
        })
        .collect();
    let mut h = vec![uni[0][0], uni[0][1], uni[0][2]];
    let h2 = vec![uni[1][0], uni[1][1], uni[1][2]];
    h = pgcd_gen(g, &h, &h2, mu);
    let ts: Vec<Fe> = if h.is_empty() {
        return Err(format!("Q1, Q2 vanish on the line at ({x},{p})"));
    } else if h.len() >= 2 {
        roots_with(f, &h, mu, frob)
    } else {
        Vec::new()
    };
    for t in ts {
        if a != 0 {
            let ai = g.inv(a);
            let s2 = g.mul(c ^ g.mul(b, t), ai);
            *mu += 3;
            out.push([s2, t, x, p]);
        } else {
            let s3 = g.mul(c, g.inv(b));
            *mu += 2;
            out.push([t, s3, x, p]);
        }
    }
    Ok(out)
}

fn current_total(b: &BinSolve) -> u64 {
    b.weil_muls
        + b.triangularise_muls
        + b.echelon_muls
        + b.nf_muls
        + b.charpoly_muls
        + b.roots_muls
        + b.eigenvector_muls
        + b.verify_muls
}

/// Exhaustive rational solutions (independent of both solvers; uncounted):
/// for every (S3, s4, P) each component is a quadratic in S2; solve the
/// first nonzero one in closed form (characteristic 2) and check all four.
fn brute(f: &Gf2q, comps: &[Comp]) -> Vec<[Fe; 4]> {
    let g = &f.tab;
    let q = f.q;
    let mut out = Vec::new();
    let pw = |x: Fe, e: u8| -> Fe {
        let mut r = 1;
        for _ in 0..e {
            r = g.mul(r, x);
        }
        r
    };
    for s4 in 0..q {
        for p in 0..q {
            for s3 in 0..q {
                let cf: Vec<[Fe; 3]> = comps
                    .iter()
                    .map(|c| {
                        let mut k = [0u32; 3];
                        for (m, &v) in c {
                            let t = g.mul(v, g.mul(pw(s3, m[1]), g.mul(pw(s4, m[2]), pw(p, m[3]))));
                            k[m[0] as usize] ^= t;
                        }
                        k
                    })
                    .collect();
                let Some(k) = cf.iter().find(|k| k.iter().any(|&v| v != 0)) else {
                    for s2 in 0..q {
                        out.push([s2, s3, s4, p]);
                    }
                    continue;
                };
                let cand: Vec<Fe> = if k[2] == 0 {
                    if k[1] == 0 {
                        Vec::new()
                    } else {
                        vec![g.mul(k[0], g.inv(k[1]))]
                    }
                } else if k[1] == 0 {
                    vec![f.sqrt(g.mul(k[0], g.inv(k[2])))]
                } else {
                    // S2 = (k1/k2) y, y^2 + y = k0 k2 / k1^2.
                    let u = g.mul(k[1], g.inv(k[2]));
                    let w = g.mul(g.mul(k[0], k[2]), g.inv(g.mul(k[1], k[1])));
                    match f.artin_schreier(w) {
                        Some(y) => vec![g.mul(u, y), g.mul(u, y ^ 1)],
                        None => Vec::new(),
                    }
                };
                for s2 in cand {
                    if cf
                        .iter()
                        .all(|k| k[0] ^ g.mul(k[1], s2) ^ g.mul(k[2], g.mul(s2, s2)) == 0)
                    {
                        out.push([s2, s3, s4, p]);
                    }
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

fn main() {
    let a: Vec<String> = env::args().skip(1).collect();
    let m: u32 = a.first().map_or(12, |s| s.parse().expect("m"));
    let trials: u64 = a.get(1).map_or(300, |s| s.parse().expect("trials"));
    // Optional: exhaustively check targets (small m only): `N` = the first N,
    // `list:i,j,...` = those trial indices.
    let brute_set: Vec<u64> = match a.get(2) {
        None => Vec::new(),
        Some(s) if s.starts_with("list:") => s[5..]
            .split(',')
            .map(|v| v.parse().expect("index"))
            .collect(),
        Some(s) => (0..s.parse::<u64>().expect("nbrute")).collect(),
    };
    let (mut b_eq_cur, mut b_eq_elim, mut b_n) = (0u64, 0u64, 0u64);
    let c = find_bin_curve(m, 1, 16).expect("curve");
    let curve = c.curve;
    let mut rng = StdRng::seed_from_u64(1 ^ 0x9e);
    let pre = SymmetrisedBin::precompute(&curve, &mut rng, 8).expect("pre");
    let fq = &curve.f.f;
    let mut traces = [BinTrace::default(), BinTrace::default()];
    let mut rrng = StdRng::seed_from_u64(77);
    // Per-trial rows: weil, cur_base, cur_frob, elim_base, elim_frob.
    let mut rows: Vec<[u64; 5]> = Vec::new();
    let mut ccomp = [[0u64; 9]; 2];
    let mut ecomp = [[0u64; 8]; 2];
    let mut agree_nz = 0u64;
    let (mut agree, mut disagree, mut cur_fail, mut elim_err) = (0u64, 0u64, 0u64, 0u64);
    let mut hist_degr: BTreeMap<usize, u64> = BTreeMap::new();
    let mut hist_degf: BTreeMap<String, u64> = BTreeMap::new();
    let mut hist_strip: BTreeMap<String, u64> = BTreeMap::new();
    let mut hist_npts: BTreeMap<usize, u64> = BTreeMap::new();
    let mut hist_rat: BTreeMap<usize, u64> = BTreeMap::new();
    let mut flags: BTreeMap<String, u64> = BTreeMap::new();
    let mut errs: BTreeMap<String, u64> = BTreeMap::new();
    let (mut nsol, mut nsol_dzero) = (0u64, 0u64);
    for trial in 0..trials {
        let r = loop {
            let x = curve.f.random(&mut rrng);
            if let Some(p) = curve.lift_x(&x, &mut rrng) {
                break p;
            }
        };
        let before = fq.muls();
        let comps = pre.system_frobenius(&curve.f, &curve.t_of_x(&r.x));
        let weil = fq.muls() - before;
        // Current path, both root-finding arms, each with its own shape trace
        // (as d4_char2_profile does through solve_d4_bin).
        let mut cur: Vec<(Option<Vec<[Fe; 4]>>, BinSolve)> = Vec::new();
        for (arm, tr) in traces.iter_mut().enumerate() {
            let mut st = BinSolve {
                frob_roots: arm == 1,
                weil_muls: weil,
                ..Default::default()
            };
            let s = solve_bin::<4>(
                fq,
                &comps,
                &FROBENIUS_WEIGHTS,
                3,
                11,
                true,
                &mut st,
                Some(tr),
            );
            let b = &st;
            for (i, v) in [
                b.weil_muls,
                b.triangularise_muls,
                b.echelon_muls,
                b.nf_muls,
                b.charpoly_muls,
                b.roots_muls,
                b.eigenvector_muls,
                b.verify_muls,
                current_total(b),
            ]
            .iter()
            .enumerate()
            {
                ccomp[arm][i] += v;
            }
            cur.push((s, st));
        }
        let mut el: Vec<(Result<Vec<[Fe; 4]>, String>, ElimCost)> = Vec::new();
        let mut info = ElimInfo::default();
        for arm in 0..2 {
            let mut cost = ElimCost::default();
            let mut inf = ElimInfo::default();
            let s = solve_elim(fq, &comps, arm == 1, &mut cost, &mut inf);
            let v = [
                cost.reduce,
                cost.build,
                cost.resultant,
                cost.interp,
                cost.roots,
                cost.backsub,
                cost.s4zero,
                cost.verify,
            ];
            for (i, x) in v.iter().enumerate() {
                ecomp[arm][i] += x;
            }
            if arm == 0 {
                info = inf;
            }
            el.push((s, cost));
        }
        let (Some(s0), Some(s1)) = (&cur[0].0, &cur[1].0) else {
            cur_fail += 1;
            continue;
        };
        assert_eq!(s0, s1, "current arms disagree");
        for fl in &info.flags {
            *flags.entry(fl.clone()).or_default() += 1;
        }
        match (&el[0].0, &el[1].0) {
            (Ok(e0), Ok(e1)) => {
                assert_eq!(e0, e1, "elimination arms disagree");
                let nz = |v: &Vec<[Fe; 4]>| -> Vec<[Fe; 4]> {
                    v.iter().filter(|s| s[2] != 0).copied().collect()
                };
                if nz(e0) == nz(s0) {
                    agree_nz += 1;
                }
                if e0 == s0 {
                    agree += 1;
                } else {
                    disagree += 1;
                    println!("DISAGREE trial {trial}: current {s0:?} elim {e0:?}");
                }
            }
            (Err(e), _) | (_, Err(e)) => {
                elim_err += 1;
                *errs.entry(e.clone()).or_default() += 1;
                println!("ELIM ERR trial {trial}: {e}");
            }
        }
        nsol += s0.len() as u64;
        for s in s0 {
            // D at the solution: s4 = 0 forces D = 0.
            if s[2] == 0 {
                nsol_dzero += 1;
            }
        }
        *hist_degr.entry(info.deg_r).or_default() += 1;
        *hist_npts.entry(info.npts).or_default() += 1;
        *hist_rat.entry(info.rat_roots).or_default() += 1;
        *hist_degf.entry(format!("{:?}", info.deg_f)).or_default() += 1;
        *hist_strip
            .entry(format!("F {:?} R {}", info.strip_f, info.strip_r))
            .or_default() += 1;
        if brute_set.contains(&trial) {
            let bs = brute(fq, &comps);
            b_n += 1;
            let e0 = el[0].0.as_ref().ok();
            b_eq_cur += u64::from(&bs == s0);
            b_eq_elim += u64::from(e0 == Some(&bs));
            println!(
                "BRUTE trial {trial}: brute {bs:?} | current {s0:?} | elim {:?}",
                e0
            );
        }
        rows.push([
            weil,
            current_total(&cur[0].1),
            current_total(&cur[1].1),
            weil + el[0].1.total(),
            weil + el[1].1.total(),
        ]);
    }
    let n = rows.len() as f64;
    if b_n > 0 {
        println!(
            "BRUTE {b_n} targets: brute == current on {b_eq_cur}, brute == elim on {b_eq_elim}"
        );
    }
    println!(
        "RANK2-ELIM m {m} curve seed 1 trials {trials} (paired rows {})",
        rows.len()
    );
    println!("current fails {cur_fail}; elim errors {elim_err} {errs:?}; agree {agree}; disagree {disagree}");
    println!(
        "agree on the s4 != 0 solutions (the only ones that can split into t_i >= 2): {agree_nz}"
    );
    println!(
        "rational solutions {nsol} ({:.3}/trial); with s4 = 0: {nsol_dzero}",
        nsol as f64 / n
    );
    println!("flags {flags:?}");
    println!("deg F (deg_s4, deg_P, total) after s4-strip: {hist_degf:?}");
    println!("s4 strips (F1, F2), R: {hist_strip:?}");
    println!("evaluation points histogram {hist_npts:?}");
    println!("deg R (after stripping s4^k) histogram {hist_degr:?}");
    println!("rational nonzero roots of R histogram {hist_rat:?}");
    let cn = [
        "weil", "triang", "echelon", "nf", "charpoly", "roots", "eigvec", "verify", "TOTAL",
    ];
    println!("{:10} {:>12} {:>12}", "current", "baseline", "frob_roots");
    for (i, nm) in cn.iter().enumerate() {
        println!(
            "{nm:10} {:12.0} {:12.0}",
            ccomp[0][i] as f64 / n,
            ccomp[1][i] as f64 / n
        );
    }
    let en = [
        "reduce",
        "build",
        "resultant",
        "interp",
        "roots",
        "backsub",
        "s4zero",
        "verify",
    ];
    println!("{:10} {:>12} {:>12}", "elim", "baseline", "frob_roots");
    println!(
        "{:10} {:12.0} {:12.0}",
        "weil",
        ccomp[0][0] as f64 / n,
        ccomp[1][0] as f64 / n
    );
    for (i, nm) in en.iter().enumerate() {
        println!(
            "{nm:10} {:12.0} {:12.0}",
            ecomp[0][i] as f64 / n,
            ecomp[1][i] as f64 / n
        );
    }
    for (arm, (ci, ei), nm) in [(0, (1, 3), "baseline"), (1, (2, 4), "frob_roots")]
        .iter()
        .map(|&(a, p, nm)| (a, p, nm))
    {
        let cs: f64 = rows.iter().map(|r| r[ci] as f64).sum();
        let es: f64 = rows.iter().map(|r| r[ei] as f64).sum();
        let ratio = es / cs;
        // Paired bootstrap of the ratio of means (fixed seed).
        let mut brng = StdRng::seed_from_u64(2026_1112 + arm as u64);
        let mut bs = Vec::with_capacity(2000);
        for _ in 0..2000 {
            let (mut c2, mut e2) = (0f64, 0f64);
            for _ in 0..rows.len() {
                let k = rand::Rng::gen_range(&mut brng, 0..rows.len());
                c2 += rows[k][ci] as f64;
                e2 += rows[k][ei] as f64;
            }
            bs.push(e2 / c2);
        }
        bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let wins = rows.iter().filter(|r| r[ei] < r[ci]).count();
        println!(
            "PAIRED {nm}: current {:.0} elim {:.0} muls/trial; ratio elim/current {:.4} [95% boot {:.4}, {:.4}]; elim cheaper on {wins}/{} trials",
            cs / n,
            es / n,
            ratio,
            bs[50],
            bs[1949],
            rows.len()
        );
    }
}
