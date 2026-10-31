//! Symbolic binary Semaev polynomial S_5 and its symmetrisation in the
//! elementary symmetric functions of x1..x4. std only, sparse polynomials
//! over F_2 in x1..x5, b and an auxiliary X (resultant variable).
//!
//! Construction (Semaev 2004, binary curves y^2 + xy = x^3 + a x^2 + b):
//!   S_3(x1,x2,x3) = (x1+x2)^2 x3^2 + x1 x2 x3 + x1^2 x2^2 + b
//!   S_4(x1..x4)   = Res_X(S_3(x1,x2,X), S_3(x3,x4,X))
//!   S_5(x1..x5)   = Res_X(S_3(x1,x2,X), S_4(X,x3,x4,x5))
//! S_3 is checked numerically against point addition in main.rs (`check`).

use std::collections::BTreeMap;

pub const SLOTS: usize = 7; // x1..x5 = 0..4, b = 5, X = 6
const BITS: u32 = 6;

pub type Mono = u64;
pub type Poly = Vec<Mono>; // sorted, unique, coefficients in F_2

pub fn mono(exps: &[(usize, u32)]) -> Mono {
    let mut m = 0u64;
    for &(s, e) in exps {
        m += (e as u64) << (BITS * s as u32);
    }
    m
}
pub fn exp(m: Mono, s: usize) -> u32 {
    ((m >> (BITS * s as u32)) & ((1 << BITS) - 1)) as u32
}
pub fn norm(mut v: Vec<Mono>) -> Poly {
    v.sort_unstable();
    let mut out: Vec<Mono> = Vec::with_capacity(v.len());
    for m in v {
        if out.last() == Some(&m) {
            out.pop();
        } else {
            out.push(m);
        }
    }
    out
}
pub fn add(a: &Poly, b: &Poly) -> Poly {
    let mut v = a.clone();
    v.extend_from_slice(b);
    norm(v)
}
pub fn mul(a: &Poly, b: &Poly) -> Poly {
    let mut v = Vec::with_capacity(a.len() * b.len());
    for &x in a {
        for &y in b {
            v.push(x + y);
        }
    }
    norm(v)
}
pub fn var(s: usize) -> Poly {
    vec![mono(&[(s, 1)])]
}
pub fn one() -> Poly {
    vec![0]
}
/// Relabel variable slots: slot s becomes map[s] (exponents must not collide).
pub fn relabel(p: &Poly, map: &[usize; SLOTS]) -> Poly {
    norm(
        p.iter()
            .map(|&m| {
                let mut r = 0u64;
                for s in 0..SLOTS {
                    r += (exp(m, s) as u64) << (BITS * map[s] as u32);
                }
                r
            })
            .collect(),
    )
}
/// Coefficients of p as a polynomial in slot `s`: result[d] = coefficient of s^d.
pub fn coeffs_in(p: &Poly, s: usize) -> Vec<Poly> {
    let dmax = p.iter().map(|&m| exp(m, s)).max().unwrap_or(0) as usize;
    let mut out = vec![vec![]; dmax + 1];
    for &m in p {
        let d = exp(m, s);
        out[d as usize].push(m - ((d as u64) << (BITS * s as u32)));
    }
    out.into_iter().map(norm).collect()
}

/// Sylvester resultant of f (coeffs f[0..=df] in X) and g (g[0..=dg]).
pub fn resultant(f: &[Poly], g: &[Poly]) -> Poly {
    let (df, dg) = (f.len() - 1, g.len() - 1);
    let n = df + dg;
    let zero: Poly = vec![];
    // rows: dg rows of f (shifted), df rows of g; columns by descending power
    let mut rows: Vec<Vec<&Poly>> = vec![];
    for i in 0..dg {
        let mut r = vec![&zero; n];
        for d in 0..=df {
            r[i + (df - d)] = &f[d];
        }
        rows.push(r);
    }
    for i in 0..df {
        let mut r = vec![&zero; n];
        for d in 0..=dg {
            r[i + (dg - d)] = &g[d];
        }
        rows.push(r);
    }
    // determinant over F_2 (no signs) by subset DP over columns
    let mut cur: BTreeMap<u32, Poly> = BTreeMap::new();
    cur.insert(0, one());
    for r in 0..n {
        let mut next: BTreeMap<u32, Poly> = BTreeMap::new();
        for (&s, minor) in &cur {
            for c in 0..n {
                if s >> c & 1 == 1 || rows[r][c].is_empty() {
                    continue;
                }
                let t = mul(rows[r][c], minor);
                let e = next.entry(s | 1 << c).or_default();
                *e = add(e, &t);
            }
        }
        cur = next;
    }
    cur.remove(&((1u32 << n) - 1)).unwrap_or_default()
}

pub fn s3(a: usize, b: usize, c: usize) -> Poly {
    // (xa+xb)^2 xc^2 + xa xb xc + xa^2 xb^2 + bparam ; slots a,b,c ; b = slot 5
    let t1 = mul(&add(&var(a), &var(b)), &add(&var(a), &var(b)));
    let t1 = mul(&t1, &mul(&var(c), &var(c)));
    let t2 = mul(&mul(&var(a), &var(b)), &var(c));
    let t3 = mul(&mul(&var(a), &var(a)), &mul(&var(b), &var(b)));
    add(&add(&t1, &t2), &add(&t3, &var(5)))
}

pub fn s4_poly() -> Poly {
    // S_4(x1,x2,x3,x4) in slots 0..3, via X = slot 6
    let f = coeffs_in(&s3(0, 1, 6), 6);
    let g = coeffs_in(&s3(2, 3, 6), 6);
    resultant(&f, &g)
}

pub fn s5_poly() -> Poly {
    let s4 = s4_poly();
    // S_4(X, x3, x4, x5): slots 0->6, 1->2, 2->3, 3->4
    let mut map = [0usize; SLOTS];
    map[0] = 6;
    map[1] = 2;
    map[2] = 3;
    map[3] = 4;
    map[4] = 4;
    map[5] = 5;
    map[6] = 6;
    let s4r = relabel(&s4, &map);
    let f = coeffs_in(&s3(0, 1, 6), 6);
    let g = coeffs_in(&s4r, 6);
    resultant(&f, &g)
}

/// A term of S_5 in the symmetric basis: e1^i[0] e2^i[1] e3^i[2] e4^i[3]
/// x5^j b^l, coefficient 1 in F_2.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ETerm {
    pub i: [u8; 4],
    pub j: u8,
    pub l: u8,
}

/// Elementary symmetric polynomials of x1..x4 as polys.
fn elem() -> [Poly; 4] {
    let x: Vec<Poly> = (0..4).map(var).collect();
    let mut e: [Poly; 4] = [vec![], vec![], vec![], vec![]];
    for mask in 1u32..16 {
        let k = mask.count_ones() as usize;
        let mut p = one();
        for s in 0..4 {
            if mask >> s & 1 == 1 {
                p = mul(&p, &x[s]);
            }
        }
        e[k - 1] = add(&e[k - 1], &p);
    }
    e
}

/// Symmetrise: express p (symmetric in slots 0..3, degree <= dmax in each)
/// in e1..e4 with F_2[x5,b] coefficients. Returns None if not symmetric.
pub fn symmetrise(p: &Poly, dmax: u32) -> Option<Vec<ETerm>> {
    let e = elem();
    // partitions lambda = (a1>=a2>=a3>=a4), a1<=dmax; e-monomial exps i
    // with lambda = (i1+i2+i3+i4, i2+i3+i4, i3+i4, i4).
    let mut parts: Vec<[u32; 4]> = vec![];
    for a1 in 0..=dmax {
        for a2 in 0..=a1 {
            for a3 in 0..=a2 {
                for a4 in 0..=a3 {
                    parts.push([a1, a2, a3, a4]);
                }
            }
        }
    }
    let np = parts.len();
    let pidx: BTreeMap<[u32; 4], usize> = parts.iter().cloned().enumerate().map(|(i, p)| (p, i)).collect();
    // powers of e
    let mut epow: Vec<Vec<Poly>> = vec![];
    for k in 0..4 {
        let mut v = vec![one()];
        for d in 1..=dmax as usize {
            let nxt = mul(&v[d - 1], &e[k]);
            v.push(nxt);
        }
        epow.push(v);
    }
    // For each partition (i.e. each e-monomial) its coefficients on sorted
    // monomials x^lambda: bit matrix M[mu][lambda]
    let mut m: Vec<Vec<u64>> = vec![vec![0u64; np.div_ceil(64)]; np];
    for (mi, lam) in parts.iter().enumerate() {
        let i = [lam[0] - lam[1], lam[1] - lam[2], lam[2] - lam[3], lam[3]];
        let mut q = epow[0][i[0] as usize].clone();
        for k in 1..4 {
            if i[k] > 0 {
                q = mul(&q, &epow[k][i[k] as usize]);
            }
        }
        for &mm in &q {
            let ex = [exp(mm, 0), exp(mm, 1), exp(mm, 2), exp(mm, 3)];
            if ex[0] >= ex[1] && ex[1] >= ex[2] && ex[2] >= ex[3] {
                if let Some(&li) = pidx.get(&ex) {
                    m[mi][li / 64] ^= 1 << (li % 64);
                }
            }
        }
    }
    // coefficient of x^lambda in p, grouped by (x5 exp j, b exp l)
    let mut by: BTreeMap<(u32, u32), Vec<u64>> = BTreeMap::new();
    for &mm in p {
        let ex = [exp(mm, 0), exp(mm, 1), exp(mm, 2), exp(mm, 3)];
        let (j, l) = (exp(mm, 4), exp(mm, 5));
        if ex[0] > dmax {
            return None;
        }
        if ex[0] >= ex[1] && ex[1] >= ex[2] && ex[2] >= ex[3] {
            let v = by.entry((j, l)).or_insert_with(|| vec![0u64; np.div_ceil(64)]);
            let li = pidx[&ex];
            v[li / 64] ^= 1 << (li % 64);
        }
    }
    // Unitriangular solve: process partitions in decreasing lex order.
    // (e-monomial mu has leading monomial x^mu in lex order.)
    let mut order: Vec<usize> = (0..np).collect();
    order.sort_by(|&a, &b| parts[b].cmp(&parts[a]));
    let mut out = vec![];
    for ((j, l), mut v) in by {
        for &mi in &order {
            if v[mi / 64] >> (mi % 64) & 1 == 1 {
                let lam = parts[mi];
                out.push(ETerm {
                    i: [
                        (lam[0] - lam[1]) as u8,
                        (lam[1] - lam[2]) as u8,
                        (lam[2] - lam[3]) as u8,
                        lam[3] as u8,
                    ],
                    j: j as u8,
                    l: l as u8,
                });
                for w in 0..v.len() {
                    v[w] ^= m[mi][w];
                }
            }
        }
        if v.iter().any(|&w| w != 0) {
            return None; // not symmetric
        }
    }
    out.sort();
    Some(out)
}
