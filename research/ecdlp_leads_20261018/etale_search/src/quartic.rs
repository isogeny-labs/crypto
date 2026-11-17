//! Plane quartics (non-hyperelliptic genus 3) over F_2 for the L8 étale search.
//!
//! Points and places of degree 1, 2, 3, 4 and 6 live in F_{2^12}, so every place has one
//! consistent representation. Relations come from two sources: line sections (for a line
//! l of exact degree k' <= 4 the norm of its intersection divisor is linearly equivalent
//! to k' times the hyperplane class, the divisor of an F_2-line; lines meeting C outside
//! the factor base are skipped) and transversal conic and cubic sections over F_2
//! (`form_sections`). The group read off these relations is accepted only if it matches
//! L_C(1).

use crate::{analyse, group_matches, is_prime, lpoly_from_counts, note_primes, Stats};
use std::collections::HashMap;

/// GF(2^12) with log tables; modulus x^12 + x^3 + 1. Start-up checks that one element
/// has 4095 distinct powers, which holds only if the quotient ring is a field.
pub struct F12 {
    exp: Vec<u16>,
    log: Vec<u16>,
}
const M12: u32 = (1 << 12) | (1 << 3) | 1;
const Q12: u32 = 4096;

fn raw_mul(a: u32, b: u32) -> u32 {
    let (mut a, mut b, mut r) = (a, b, 0u32);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a <<= 1;
        b >>= 1;
    }
    while r >= Q12 {
        let s = (31 - r.leading_zeros()) - 12;
        r ^= M12 << s;
    }
    r
}

impl F12 {
    pub fn new() -> F12 {
        // primitive element
        let order = Q12 - 1; // 4095 = 3^2 * 5 * 7 * 13
        let pw = |a: u32, mut e: u32| -> u32 {
            let (mut b, mut r) = (a, 1u32);
            while e > 0 {
                if e & 1 == 1 {
                    r = raw_mul(r, b);
                }
                b = raw_mul(b, b);
                e >>= 1;
            }
            r
        };
        let g = (2..Q12)
            .find(|&z| [3u32, 5, 7, 13].iter().all(|&p| pw(z, order / p) != 1))
            .unwrap();
        let mut exp = vec![0u16; 2 * order as usize];
        let mut log = vec![0u16; Q12 as usize];
        let mut seen = vec![false; Q12 as usize];
        let mut t = 1u32;
        for i in 0..order {
            assert!(
                t != 0 && !seen[t as usize],
                "x^12 + x^3 + 1 is not irreducible"
            );
            seen[t as usize] = true;
            exp[i as usize] = t as u16;
            exp[(i + order) as usize] = t as u16;
            log[t as usize] = i as u16;
            t = raw_mul(t, g);
        }
        assert_eq!(t, 1);
        F12 { exp, log }
    }
    #[inline]
    pub fn mul(&self, a: u16, b: u16) -> u16 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] as usize + self.log[b as usize] as usize]
        }
    }
    #[inline]
    pub fn inv(&self, a: u16) -> u16 {
        assert!(a != 0);
        self.exp[(4095 - self.log[a as usize] as usize) % 4095]
    }
    #[inline]
    pub fn sq(&self, a: u16) -> u16 {
        self.mul(a, a)
    }
    pub fn in_subfield(&self, a: u16, d: u32) -> bool {
        let mut b = a;
        for _ in 0..d {
            b = self.sq(b);
        }
        b == a
    }
    pub fn subfield(&self, d: u32) -> Vec<u16> {
        (0..Q12 as u16)
            .filter(|&a| self.in_subfield(a, d))
            .collect()
    }
}

// ---------- polynomials over F12 (low -> high) ----------
fn ptrim(p: &mut Vec<u16>) {
    while p.last() == Some(&0) {
        p.pop();
    }
}
fn padd(a: &[u16], b: &[u16]) -> Vec<u16> {
    let mut r = vec![0u16; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        r[i] ^= x;
    }
    for (i, &x) in b.iter().enumerate() {
        r[i] ^= x;
    }
    ptrim(&mut r);
    r
}
fn ppmul(f: &F12, a: &[u16], b: &[u16]) -> Vec<u16> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u16; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] ^= f.mul(x, y);
        }
    }
    ptrim(&mut r);
    r
}
fn pdivrem(f: &F12, a: &[u16], b: &[u16]) -> (Vec<u16>, Vec<u16>) {
    assert!(!b.is_empty());
    let mut r = a.to_vec();
    ptrim(&mut r);
    let db = b.len() - 1;
    let lc_inv = f.inv(b[db]);
    let mut q = vec![0u16; r.len().saturating_sub(db).max(1)];
    while r.len() > db {
        let k = r.len() - 1 - db;
        let c = f.mul(r[r.len() - 1], lc_inv);
        q[k] ^= c;
        for (i, &bi) in b.iter().enumerate() {
            r[k + i] ^= f.mul(c, bi);
        }
        ptrim(&mut r);
    }
    ptrim(&mut q);
    (q, r)
}
fn pgcd12(f: &F12, a: &[u16], b: &[u16]) -> Vec<u16> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    ptrim(&mut a);
    ptrim(&mut b);
    while !b.is_empty() {
        let r = pdivrem(f, &a, &b).1;
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let li = f.inv(lc);
        for x in a.iter_mut() {
            *x = f.mul(*x, li);
        }
    }
    a
}
fn peval(f: &F12, p: &[u16], s: u16) -> u16 {
    p.iter().rev().fold(0u16, |acc, &c| f.mul(acc, s) ^ c)
}
/// Roots in F_{2^12} of a squarefree polynomial that splits there (equal-degree splitting).
fn split_roots(f: &F12, h: &[u16], out: &mut Vec<u16>) {
    let d = h.len() - 1;
    if d == 0 {
        return;
    }
    if d == 1 {
        out.push(f.mul(h[0], f.inv(h[1])));
        return;
    }
    for beta in 1..Q12 as u16 {
        // T = sum_{i<12} (beta s)^(2^i) mod h
        let mut term = pdivrem(f, &[0, beta], h).1;
        let mut t = term.clone();
        for _ in 1..12 {
            term = pdivrem(f, &ppmul(f, &term, &term), h).1;
            t = padd(&t, &term);
        }
        let g = pgcd12(f, h, &t);
        if g.len() > 1 && g.len() < h.len() {
            let (q, r) = pdivrem(f, h, &g);
            assert!(r.is_empty());
            split_roots(f, &g, out);
            split_roots(f, &q, out);
            return;
        }
    }
    panic!("equal-degree splitting failed");
}
/// All roots in F_{2^12} with multiplicity, and whether every root of p lies there.
fn roots_with_mult(f: &F12, p: &[u16]) -> (Vec<(u16, u32)>, bool) {
    let deg = p.len() - 1;
    // s^(2^12) mod p
    let mut t = pdivrem(f, &[0, 1], p).1;
    for _ in 0..12 {
        t = pdivrem(f, &ppmul(f, &t, &t), p).1;
    }
    let t = padd(&t, &[0, 1]); // s^(2^12) - s
    let g = pgcd12(f, p, &t);
    let mut rs = vec![];
    split_roots(f, &g, &mut rs);
    let mut out = vec![];
    let mut total = 0u32;
    for r in rs {
        assert_eq!(peval(f, p, r), 0, "split_roots returned a non-root");
        let mut m = 0u32;
        let mut q = p.to_vec();
        loop {
            let (qq, rem) = pdivrem(f, &q, &[r, 1]);
            if !rem.is_empty() {
                break;
            }
            q = qq;
            m += 1;
        }
        total += m;
        out.push((r, m));
    }
    (out, total as usize == deg)
}

// ---------- quartic forms ----------
type P3 = [u16; 3];

struct Quartic<'a> {
    f: &'a F12,
    mons: &'a [(u32, u32, u32)],
    mask: u32,
}
impl Quartic<'_> {
    fn powers(&self, a: u16) -> [u16; 5] {
        let mut p = [1u16; 5];
        for i in 1..5 {
            p[i] = self.f.mul(p[i - 1], a);
        }
        p
    }
    /// (F, F_x, F_y, F_z) at a point.
    fn eval_all(&self, p: P3) -> (u16, u16, u16, u16) {
        let (px, py, pz) = (self.powers(p[0]), self.powers(p[1]), self.powers(p[2]));
        let (mut v, mut dx, mut dy, mut dz) = (0u16, 0u16, 0u16, 0u16);
        for (b, &(i, j, k)) in self.mons.iter().enumerate() {
            if (self.mask >> b) & 1 == 0 {
                continue;
            }
            let f = self.f;
            v ^= f.mul(f.mul(px[i as usize], py[j as usize]), pz[k as usize]);
            if i % 2 == 1 {
                dx ^= f.mul(f.mul(px[i as usize - 1], py[j as usize]), pz[k as usize]);
            }
            if j % 2 == 1 {
                dy ^= f.mul(f.mul(px[i as usize], py[j as usize - 1]), pz[k as usize]);
            }
            if k % 2 == 1 {
                dz ^= f.mul(f.mul(px[i as usize], py[j as usize]), pz[k as usize - 1]);
            }
        }
        (v, dx, dy, dz)
    }
    fn eval(&self, p: P3) -> u16 {
        self.eval_all(p).0
    }
    /// F(u + s v) as a polynomial in s (degree <= 4).
    fn restrict(&self, u: P3, v: P3) -> Vec<u16> {
        let f = self.f;
        let lin = |c: usize| -> Vec<u16> {
            let mut l = vec![u[c], v[c]];
            ptrim(&mut l);
            l
        };
        let (lx, ly, lz) = (lin(0), lin(1), lin(2));
        let pows = |l: &Vec<u16>| -> Vec<Vec<u16>> {
            let mut out = vec![vec![1u16]];
            for e in 1..5 {
                let nxt = ppmul(f, &out[e - 1], l);
                out.push(nxt);
            }
            out
        };
        let (px, py, pz) = (pows(&lx), pows(&ly), pows(&lz));
        let mut g: Vec<u16> = vec![];
        for (b, &(i, j, k)) in self.mons.iter().enumerate() {
            if (self.mask >> b) & 1 == 0 {
                continue;
            }
            let t = ppmul(
                f,
                &ppmul(f, &px[i as usize], &py[j as usize]),
                &pz[k as usize],
            );
            g = padd(&g, &t);
        }
        g
    }
}

fn normalize(f: &F12, p: P3) -> P3 {
    let lead = if p[0] != 0 {
        p[0]
    } else if p[1] != 0 {
        p[1]
    } else {
        p[2]
    };
    let li = f.inv(lead);
    [f.mul(p[0], li), f.mul(p[1], li), f.mul(p[2], li)]
}
fn frob3(f: &F12, p: P3) -> P3 {
    [f.sq(p[0]), f.sq(p[1]), f.sq(p[2])]
}
fn pdeg(f: &F12, p: P3) -> u32 {
    let mut q = p;
    for d in 1..=12 {
        q = frob3(f, q);
        if q == p {
            return d;
        }
    }
    unreachable!()
}
fn pcanon(f: &F12, p: P3) -> P3 {
    let mut best = p;
    let mut q = p;
    for _ in 0..12 {
        q = frob3(f, q);
        if q < best {
            best = q;
        }
    }
    best
}
/// Normalised projective points of P^2(F_{2^d}).
fn proj_points(sub: &[u16]) -> Vec<P3> {
    let mut v = vec![];
    for &y in sub {
        for &z in sub {
            v.push([1, y, z]);
        }
    }
    for &z in sub {
        v.push([0, 1, z]);
    }
    v.push([0, 0, 1]);
    v
}

/// Place degrees in the factor base: all places of degree 1..=4 and 6 (F_64 lies in F_4096,
/// F_32 does not). Degrees 1..=3 give surjectivity onto Pic (Riemann-Roch, given a
/// degree-1 place); degree 6 feeds the out-of-sample N_6 check.
const DEGS: [usize; 5] = [1, 2, 3, 4, 6];

/// Monomials of degree d in (x, y, z).
fn monomials_deg(d: u32) -> Vec<(u32, u32, u32)> {
    let mut v = vec![];
    for i in (0..=d).rev() {
        for j in (0..=d - i).rev() {
            v.push((i, j, d - i - j));
        }
    }
    v
}

/// Relations from F_2-forms G of degree d whose intersection with C is transversal and
/// inside the factor base: if G vanishes at 4d distinct geometric points of factor-base
/// places then, by Bezout, every intersection multiplicity is 1 and there are no others, so
/// div(G / L0^d) = sum of those places - d (L0 . C). For d = 4 the form F of C itself
/// (`fmask`) is skipped; any other quartic form is not divisible by F.
fn form_sections(
    f: &F12,
    geo: &[(P3, usize)],
    pl_deg: &[u32],
    base: &[i128],
    degs: &[u32],
    fmask: u32,
    rows: &mut Vec<Vec<i128>>,
) {
    let np = pl_deg.len();
    for &d in degs {
        let mons = monomials_deg(d);
        let pw = |a: u16, e: u32| (0..e).fold(1u16, |acc, _| f.mul(acc, a));
        // value of each monomial at each geometric point
        let mv: Vec<Vec<u16>> = geo
            .iter()
            .map(|(p, _)| {
                mons.iter()
                    .map(|&(i, j, k)| f.mul(f.mul(pw(p[0], i), pw(p[1], j)), pw(p[2], k)))
                    .collect()
            })
            .collect();
        let m = mons.len();
        let mut val = vec![0u16; geo.len()];
        // Gray code over all nonzero coefficient vectors
        let mut gmask = 0u32;
        for step in 1u32..(1 << m) {
            let bit = step.trailing_zeros() as usize;
            gmask ^= 1 << bit;
            for (v, row) in val.iter_mut().zip(mv.iter()) {
                *v ^= row[bit];
            }
            if d == 4 && gmask == fmask {
                continue;
            }
            let zeros = val.iter().filter(|&&v| v == 0).count();
            if zeros != 4 * d as usize {
                continue;
            }
            let mut acc = vec![0i128; np];
            for (k, &(_, id)) in geo.iter().enumerate() {
                if val[k] == 0 {
                    acc[id] += 1;
                }
            }
            for (id, x) in acc.iter_mut().enumerate() {
                assert!(*x % pl_deg[id] as i128 == 0, "section not Galois-stable");
                *x /= pl_deg[id] as i128;
            }
            rows.push(
                acc.iter()
                    .zip(base)
                    .map(|(x, y)| x - d as i128 * y)
                    .collect(),
            );
        }
    }
}

/// Runs the search; returns how many curves needed quartic sections to validate.
pub fn run(st: &mut Stats) -> u64 {
    let mut fallback = 0u64;
    let mut smooth_masks: Vec<u32> = vec![];
    let mut hit_masks: Vec<u32> = vec![];
    let f = F12::new();
    let mons = monomials_deg(4);
    let subs: Vec<Vec<u16>> = (0..=6)
        .map(|d| {
            if d == 0 || d == 5 {
                vec![]
            } else {
                f.subfield(d)
            }
        })
        .collect();
    let pts: Vec<Vec<P3>> = (0..=6)
        .map(|d| {
            if subs[d].is_empty() {
                vec![]
            } else {
                proj_points(&subs[d])
            }
        })
        .collect();
    // lines of exact degree <= 4, given by normalised coefficient vectors over F_16 and F_8
    let mut lines: Vec<(P3, u32)> = vec![];
    let mut seen = std::collections::HashSet::new();
    for d in [1usize, 2, 3, 4] {
        for &l in &pts[d] {
            let c = pcanon(&f, l);
            if seen.insert(c) {
                lines.push((l, pdeg(&f, l)));
            }
        }
    }
    let g = 3;
    for mask in 1u32..(1 << 15) {
        let qc = Quartic {
            f: &f,
            mons: &mons,
            mask,
        };
        // smoothness over F_{2^d}, d <= 4 (singular points of a quartic have degree <= 4);
        // P^2(F_8) and P^2(F_16) contain the points over F_2 and F_4
        let singular = [3usize, 4].into_iter().any(|d| {
            pts[d].iter().any(|&p| {
                let (v, a, b, c) = qc.eval_all(p);
                v == 0 && a == 0 && b == 0 && c == 0
            })
        });
        if singular {
            continue;
        }
        st.curves += 1;
        smooth_masks.push(mask);
        let counts: Vec<i64> = (1..=3)
            .map(|d| pts[d].iter().filter(|&&p| qc.eval(p) == 0).count() as i64)
            .collect();
        let lp = lpoly_from_counts(g, &counts);
        note_primes(st, &lp);
        let jorder: i64 = lp.iter().sum();
        assert!(jorder > 0);
        let primes: Vec<u64> = (3..=jorder as u64)
            .filter(|&p| is_prime(p) && (jorder as u64).is_multiple_of(p))
            .collect();
        if primes.is_empty() {
            continue;
        }
        // places of degree in DEGS, and their geometric points
        let mut pl_deg: Vec<u32> = vec![];
        let mut index: HashMap<P3, usize> = HashMap::new();
        let mut geo: Vec<(P3, usize)> = vec![];
        for d in DEGS {
            for &p in &pts[d] {
                if qc.eval(p) != 0 || pdeg(&f, p) != d as u32 {
                    continue;
                }
                let c = pcanon(&f, p);
                let next = pl_deg.len();
                let id = *index.entry(c).or_insert(next);
                if id == next {
                    pl_deg.push(d as u32);
                }
                geo.push((p, id));
            }
        }
        let np = pl_deg.len();
        // line sections
        let mut base: Option<Vec<i128>> = None;
        let mut rows: Vec<Vec<i128>> = vec![];
        let mut sections: Vec<(Vec<i128>, u32)> = vec![];
        'lines: for &(l, kd) in &lines {
            let (a, b, c) = (l[0], l[1], l[2]);
            let (u, v): (P3, P3) = if c != 0 {
                let ci = f.inv(c);
                ([1, 0, f.mul(a, ci)], [0, 1, f.mul(b, ci)])
            } else if b != 0 {
                ([1, f.mul(a, f.inv(b)), 0], [0, 0, 1])
            } else {
                ([0, 1, 0], [0, 0, 1])
            };
            let gpoly = qc.restrict(u, v);
            assert!(!gpoly.is_empty(), "line is a component of a smooth quartic");
            let dg = gpoly.len() - 1;
            let mut pts_m: Vec<(P3, u32)> = vec![];
            if dg < 4 {
                pts_m.push((normalize(&f, v), (4 - dg) as u32));
            }
            if dg > 0 {
                let (rs, complete) = roots_with_mult(&f, &gpoly);
                if !complete {
                    continue 'lines;
                }
                for (s, m) in rs {
                    let p = [
                        u[0] ^ f.mul(s, v[0]),
                        u[1] ^ f.mul(s, v[1]),
                        u[2] ^ f.mul(s, v[2]),
                    ];
                    pts_m.push((normalize(&f, p), m));
                }
            }
            let mut acc = vec![0i128; np];
            for (p, m) in pts_m {
                let mut q = p;
                for _ in 0..kd {
                    let Some(&id) = index.get(&pcanon(&f, q)) else {
                        continue 'lines; // a point of degree > 4
                    };
                    acc[id] += m as i128;
                    q = frob3(&f, q);
                }
            }
            for (id, x) in acc.iter_mut().enumerate() {
                assert!(*x % pl_deg[id] as i128 == 0);
                *x /= pl_deg[id] as i128;
            }
            let total: i128 = acc
                .iter()
                .enumerate()
                .map(|(i, x)| x * pl_deg[i] as i128)
                .sum();
            assert_eq!(total, 4 * kd as i128);
            if kd == 1 && base.is_none() {
                base = Some(acc.clone());
            }
            sections.push((acc, kd));
        }
        let base = base.expect("an F_2-line section");
        for (acc, kd) in sections {
            rows.push(
                acc.iter()
                    .zip(base.iter())
                    .map(|(x, y)| x - kd as i128 * y)
                    .collect(),
            );
        }
        form_sections(&f, &geo, &pl_deg, &base, &[2, 3], mask, &mut rows);
        if !group_matches(rows.clone(), pl_deg.len(), jorder) {
            fallback += 1;
            form_sections(&f, &geo, &pl_deg, &base, &[4], mask, &mut rows);
        }
        let label = format!("\"quartic_mask\":\"{:#x}\"", mask);
        let Some(curve_hits) = analyse(g, &label, &pl_deg, rows, jorder, &lp, &primes, &[6], st)
        else {
            continue;
        };
        if !curve_hits.is_empty() {
            hit_masks.push(mask);
            println!(
                "{{\"kind\":\"hit\",\"g\":3,{},\"N\":{:?},\"L\":{:?},\"J\":{},\"hits\":[{}]}}",
                label,
                counts,
                lp,
                jorder,
                curve_hits.join(",")
            );
        }
    }
    classes(&mons, &smooth_masks, &hit_masks);
    fallback
}

// ---------- isomorphism classes: orbits of GL_3(F_2) = PGL_3(F_2) on quartic forms ----------
/// All 168 invertible 3x3 matrices over F_2, rows as 3-bit masks.
fn gl3() -> Vec<[u8; 3]> {
    let mut out = vec![];
    for code in 0u32..512 {
        let r = [
            (code & 7) as u8,
            ((code >> 3) & 7) as u8,
            ((code >> 6) & 7) as u8,
        ];
        let bit = |row: u8, c: u32| (row >> c) & 1;
        let det = (0..3u32).fold(0u8, |acc, c| {
            let (c1, c2) = ((c + 1) % 3, (c + 2) % 3);
            acc ^ (bit(r[0], c) & (bit(r[1], c1) & bit(r[2], c2) ^ bit(r[1], c2) & bit(r[2], c1)))
        });
        if det == 1 {
            out.push(r);
        }
    }
    assert_eq!(out.len(), 168);
    out
}

/// Image of each degree-4 monomial under x_r -> sum_c M[r][c] x_c, as a monomial mask.
fn substitution_table(mons: &[(u32, u32, u32)], m: &[u8; 3]) -> Vec<u32> {
    let pos: HashMap<(u32, u32, u32), usize> =
        mons.iter().enumerate().map(|(b, &e)| (e, b)).collect();
    mons.iter()
        .map(|&(i, j, k)| {
            // expand L0^i L1^j L2^k over F_2, polynomials as sets of exponent triples
            let mut poly: std::collections::HashSet<(u32, u32, u32)> = [(0, 0, 0)].into();
            for (row, e) in [(m[0], i), (m[1], j), (m[2], k)] {
                for _ in 0..e {
                    let mut next = std::collections::HashSet::new();
                    for &(a, b, c) in &poly {
                        for v in 0..3u32 {
                            if (row >> v) & 1 == 1 {
                                let t = [(a + 1, b, c), (a, b + 1, c), (a, b, c + 1)][v as usize];
                                if !next.remove(&t) {
                                    next.insert(t);
                                }
                            }
                        }
                    }
                    poly = next;
                }
            }
            poly.iter().fold(0u32, |acc, e| acc | 1 << pos[e])
        })
        .collect()
}

fn equation(mons: &[(u32, u32, u32)], mask: u32) -> String {
    let var = |v: &str, e: u32| match e {
        0 => String::new(),
        1 => v.to_string(),
        _ => format!("{}^{}", v, e),
    };
    let terms: Vec<String> = mons
        .iter()
        .enumerate()
        .filter(|(b, _)| (mask >> b) & 1 == 1)
        .map(|(_, &(i, j, k))| {
            [var("x", i), var("y", j), var("z", k)]
                .into_iter()
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("*")
        })
        .collect();
    terms.join(" + ")
}

/// Prints the number of isomorphism classes among the smooth models and, for the hit models,
/// each class with a representative of fewest monomials and |Aut_{F_2}| = 168 / orbit size.
fn classes(mons: &[(u32, u32, u32)], smooth: &[u32], hits: &[u32]) {
    let tables: Vec<Vec<u32>> = gl3().iter().map(|m| substitution_table(mons, m)).collect();
    let orbit = |mask: u32| -> std::collections::BTreeSet<u32> {
        tables
            .iter()
            .map(|t| {
                (0..15)
                    .filter(|b| (mask >> b) & 1 == 1)
                    .fold(0u32, |acc, b| acc ^ t[b])
            })
            .collect()
    };
    let smooth_set: std::collections::BTreeSet<u32> = smooth.iter().copied().collect();
    let mut seen = std::collections::BTreeSet::new();
    let mut n_classes = 0u64;
    for &m in smooth {
        if seen.contains(&m) {
            continue;
        }
        let o = orbit(m);
        assert!(
            o.is_subset(&smooth_set),
            "smoothness not invariant under GL_3(F_2)"
        );
        n_classes += 1;
        seen.extend(o);
    }
    let hit_set: std::collections::BTreeSet<u32> = hits.iter().copied().collect();
    let mut done = std::collections::BTreeSet::new();
    let mut recs = vec![];
    for &m in hits {
        if done.contains(&m) {
            continue;
        }
        let o = orbit(m);
        assert!(
            o.is_subset(&hit_set),
            "hit property not invariant under GL_3(F_2)"
        );
        let rep = *o.iter().min_by_key(|&&x| (x.count_ones(), x)).unwrap();
        recs.push(format!(
            "{{\"rep\":\"{:#x}\",\"equation\":\"{}\",\"models\":{},\"aut\":{}}}",
            rep,
            equation(mons, rep),
            o.len(),
            168 / o.len()
        ));
        done.extend(o);
    }
    println!(
        "{{\"kind\":\"classes\",\"smooth_models\":{},\"classes\":{},\"hit_models\":{},\"hit_classes\":[{}]}}",
        smooth.len(),
        n_classes,
        hits.len(),
        recs.join(",")
    );
}
