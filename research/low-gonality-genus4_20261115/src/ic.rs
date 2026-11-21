//! Order determination, relation collection (trigonal fibres, plane-quintic
//! lines, general hyperplane sections), Wiedemann/dense linear algebra mod ell,
//! individual logarithms, and matched rho on the same Pic^0 arithmetic.

// Items used only by tests or kept for completeness.
#![allow(dead_code)]

use super::g4::*;
use std::collections::{HashMap, HashSet};

pub fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
pub fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}
pub fn invmod(a: u64, m: u64) -> Option<u64> {
    // extended Euclid
    let (mut t, mut nt) = (0i128, 1i128);
    let (mut r, mut nr) = (m as i128, (a % m) as i128);
    while nr != 0 {
        let q = r / nr;
        (t, nt) = (nt, t - q * nt);
        (r, nr) = (nr, r - q * nr);
    }
    if r != 1 {
        return None;
    }
    Some(((t % m as i128 + m as i128) % m as i128) as u64)
}

pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = vec![];
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            let mut e = 0;
            while n.is_multiple_of(d) {
                n /= d;
                e += 1;
            }
            out.push((d, e));
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if n > 1 {
        out.push((n, 1));
    }
    out
}

pub fn fnv(key: &[u32]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &w in key {
        for b in w.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

/// Random element: the class of D - D0 for four random distinct rational points.
pub fn random_elt(c: &Curve, rng: &mut u64) -> Elt {
    let n = c.pts.len();
    let mut idx = vec![];
    while idx.len() < 4 {
        let i = rnd(rng, n as u64) as usize;
        if !idx.contains(&i) {
            idx.push(i);
        }
    }
    c.elt_of_points(&idx)
}

fn isqrt(n: u128) -> u128 {
    let mut x = (n as f64).sqrt() as u128;
    while x * x > n {
        x -= 1;
    }
    while (x + 1) * (x + 1) <= n {
        x += 1;
    }
    x
}

/// |Pic^0(F_p)| from N_1, N_2 (L-polynomial coefficients a1, a2) and interval
/// baby-step giant-step over the Weil range of the unknown (a3, a4).
pub fn order_bsgs(c: &Curve, rng: &mut u64) -> Result<i128, String> {
    let p = c.p as i128;
    let mut ns = vec![];
    for e in 1..=2 {
        let f = Ext::new(c.p as u64, e, rng);
        ns.push(count_points(&c.g, &f) as i128);
    }
    let s1 = ns[0] - p - 1;
    let s2 = ns[1] - p * p - 1;
    let a1 = s1;
    let a2 = (a1 * s1 + s2) / 2;
    let base = 1 + p.pow(4) + a1 * (1 + p.pow(3)) + a2 * (1 + p * p);
    let a3max = 56 * (isqrt((p as u128).pow(3)) as i128 + 1);
    let a4max = 70 * p * p;
    let mut lo = base - (1 + p) * a3max - a4max;
    let mut hi = base + (1 + p) * a3max + a4max;
    // Weil range of N: ((sqrt p) -+ 1)^8
    let sp = (p as f64).sqrt();
    let wlo = ((sp - 1.0).powi(8)).floor() as i128 - 1;
    let whi = ((sp + 1.0).powi(8)).ceil() as i128 + 1;
    lo = lo.max(wlo).max(1);
    hi = hi.min(whi);
    let width = (hi - lo + 1) as u128;
    let bsz = isqrt(width) + 1;
    for attempt in 0..6 {
        let x = random_elt(c, rng);
        if c.is_zero(&x) {
            continue;
        }
        let u = c.neg(&x);
        let mut table: HashMap<Vec<u32>, u32> = HashMap::new();
        let mut cur = c.zero().clone();
        table.insert(cur.key.clone(), 0);
        for j in 1..=bsz as u32 {
            cur = c.add(&cur, &u);
            table.entry(cur.key.clone()).or_insert(j);
        }
        let stride = c.mul_scalar(&x, bsz);
        let mut z = c.mul_scalar(&x, lo as u128);
        let mut cands: Vec<i128> = vec![];
        let giants = width / bsz + 2;
        for i in 0..giants {
            if let Some(&j) = table.get(&z.key) {
                let m = lo + (i * bsz) as i128 + j as i128;
                if m >= lo && m <= hi {
                    cands.push(m);
                }
            }
            z = c.add(&z, &stride);
        }
        if cands.is_empty() {
            return Err(format!("no candidate in [{lo},{hi}]"));
        }
        if cands.len() > 64 {
            let _ = attempt;
            continue;
        }
        for _ in 0..3 {
            let y = random_elt(c, rng);
            cands.retain(|&m| c.is_zero(&c.mul_scalar(&y, m as u128)));
        }
        if cands.len() == 1 {
            return Ok(cands[0]);
        }
    }
    Err("order not determined".into())
}

// ---------------------------------------------------------------------
// relations
// ---------------------------------------------------------------------

pub type Row = Vec<(u32, i64)>;

/// Unnormalised plane through three points of P^3 (Segre coordinates
/// [tv, tu, sv, su]); zero if collinear.
fn plane3(a: &[u64; 4], b: &[u64; 4], c: &[u64; 4], p: u64) -> [u64; 4] {
    let m = [a, b, c];
    let mut out = [0u64; 4];
    for k in 0..4 {
        let cols: Vec<usize> = (0..4).filter(|&x| x != k).collect();
        let g = |r: usize, j: usize| m[r][cols[j]];
        let det = (g(0, 0) * ((g(1, 1) * g(2, 2) + p * p - g(1, 2) * g(2, 1)) % p)
            + g(0, 1) * ((g(1, 2) * g(2, 0) + p * p - g(1, 0) * g(2, 2)) % p)
            + g(0, 2) * ((g(1, 0) * g(2, 1) + p * p - g(1, 1) * g(2, 0)) % p))
            % p;
        out[k] = if k % 2 == 0 { det } else { (p - det) % p };
    }
    out
}

pub struct RelStats {
    pub pairs: u64,
    pub hits_a: u64,
    pub unique_a: u64,
    pub triples: u64,
    pub hits_c: u64,
    pub unique_c: u64,
    pub fibres_total: u64,
    pub fibres_split3: u64,
    pub fibres_full_in_f: u64,
    pub fibre_hist: [u64; 4],
    pub native: u64,
}

pub struct RelSet {
    pub rows: Vec<Row>,
    pub st: RelStats,
}

/// Column layout: F points other than Pa get columns 0..nf, then kappa, tau1, tau2.
pub struct Layout {
    pub col: Vec<i64>, // per rational point: column or -1
    pub nf: usize,
    pub in_f: Vec<bool>,
    pub fidx: Vec<usize>,
}
impl Layout {
    pub fn kappa(&self) -> u32 {
        self.nf as u32
    }
    pub fn tau(&self, k: usize) -> u32 {
        (self.nf + 1 + k) as u32
    }
    pub fn ncols(&self) -> usize {
        self.nf + 3
    }
}

pub fn make_layout(c: &Curve, fidx: &[usize]) -> Layout {
    let pa = c.d0[0];
    let mut col = vec![-1i64; c.pts.len()];
    let mut in_f = vec![false; c.pts.len()];
    let mut k = 0;
    for &i in fidx {
        in_f[i] = true;
        if i != pa {
            col[i] = k;
            k += 1;
        }
    }
    Layout {
        col,
        nf: k as usize,
        in_f,
        fidx: fidx.to_vec(),
    }
}

fn row_of(lay: &Layout, pts: &[usize], special: Option<u32>) -> Row {
    let mut r: Row = pts
        .iter()
        .filter(|&&i| lay.col[i] >= 0)
        .map(|&i| (lay.col[i] as u32, 1))
        .collect();
    if let Some(s) = special {
        r.push((s, -1));
    }
    r
}

/// Fibre-splitting statistics over all P^1(F_p) for both pencils.
pub fn fibre_stats(c: &Curve) -> ([u64; 4], [u64; 4], u64) {
    let mut h = [[0u64; 4]; 2];
    let mut maps: [HashMap<(u32, u32), u32>; 2] = [HashMap::new(), HashMap::new()];
    for q in &c.pts {
        *maps[0].entry((q[0], q[1])).or_insert(0) += 1;
        *maps[1].entry((q[2], q[3])).or_insert(0) += 1;
    }
    let fibres = c.p as u64 + 1;
    for k in 0..2 {
        let nz: u64 = maps[k].len() as u64;
        h[k][0] = fibres - nz;
        for &cnt in maps[k].values() {
            h[k][(cnt as usize).min(3)] += 1;
        }
    }
    (h[0], h[1], fibres)
}

/// Collect relations from the factor base F (indices into c.pts, must contain D0).
/// `do_a`: plane-quintic lines through Pa and two F points; `do_c_budget`:
/// number of random general-hyperplane triples to test (0 = none).
pub fn collect(
    c: &Curve,
    lay: &Layout,
    ev1: &[[u64; 4]],
    do_a: bool,
    c_budget: u64,
    rng: &mut u64,
) -> RelSet {
    let p = c.pp();
    let pa = c.d0[0];
    let mut rows: Vec<Row> = vec![];
    let mut st = RelStats {
        pairs: 0,
        hits_a: 0,
        unique_a: 0,
        triples: 0,
        hits_c: 0,
        unique_c: 0,
        fibres_total: 0,
        fibres_split3: 0,
        fibres_full_in_f: 0,
        fibre_hist: [0; 4],
        native: 0,
    };
    let fidx = &lay.fidx;
    let fnon: Vec<usize> = fidx.iter().copied().filter(|&i| i != pa).collect();
    let m = fidx.len() as u64;
    // B: fibres of both pencils
    for k in 0..2usize {
        let mut fib: HashMap<(u32, u32), Vec<usize>> = HashMap::new();
        for (i, q) in c.pts.iter().enumerate() {
            let key = if k == 0 { (q[0], q[1]) } else { (q[2], q[3]) };
            fib.entry(key).or_default().push(i);
        }
        let mut keys: Vec<_> = fib.keys().copied().collect();
        keys.sort();
        for key in keys {
            let v = &fib[&key];
            if v.len() == 3 {
                st.fibres_split3 += 1;
                if v.iter().all(|&i| lay.in_f[i]) {
                    st.fibres_full_in_f += 1;
                    rows.push(row_of(lay, v, Some(lay.tau(k))));
                }
            }
        }
        st.fibres_total += c.p as u64 + 1;
    }
    // A: lines (planes) through Pa, P1, P2
    let eval = |l: &[u64; 4], q: &[u64; 4]| -> u64 {
        (l[0] * q[0] + l[1] * q[1] + l[2] * q[2] + l[3] * q[3]) % p
    };
    if do_a {
        let mut seen: HashSet<[u32; 5]> = HashSet::new();
        for a in 0..fnon.len() {
            for b in a + 1..fnon.len() {
                let (i, j) = (fnon[a], fnon[b]);
                st.pairs += 1;
                let l = plane3(&ev1[pa], &ev1[i], &ev1[j], p);
                st.native += 60;
                if l == [0; 4] {
                    continue;
                }
                let mut others = vec![];
                for &q in fidx {
                    if q != pa && q != i && q != j && eval(&l, &ev1[q]) == 0 {
                        others.push(q);
                    }
                }
                st.native += 4 * m;
                if others.len() == 3 {
                    st.hits_a += 1;
                    let mut key = [
                        i as u32,
                        j as u32,
                        others[0] as u32,
                        others[1] as u32,
                        others[2] as u32,
                    ];
                    key.sort();
                    if seen.insert(key) {
                        st.unique_a += 1;
                        let mut pts = vec![i, j];
                        pts.extend(&others);
                        rows.push(row_of(lay, &pts, Some(lay.kappa())));
                    }
                }
            }
        }
    }
    // C: random general triples
    if c_budget > 0 && fidx.len() >= 6 {
        let mut seen: HashSet<[u32; 6]> = HashSet::new();
        for _ in 0..c_budget {
            let mut t = [0usize; 3];
            loop {
                for x in t.iter_mut() {
                    *x = fidx[rnd(rng, fidx.len() as u64) as usize];
                }
                if t[0] != t[1] && t[0] != t[2] && t[1] != t[2] {
                    break;
                }
            }
            st.triples += 1;
            let l = plane3(&ev1[t[0]], &ev1[t[1]], &ev1[t[2]], p);
            st.native += 60;
            if l == [0; 4] {
                continue;
            }
            let mut others = vec![];
            for &q in fidx {
                if q != t[0] && q != t[1] && q != t[2] && eval(&l, &ev1[q]) == 0 {
                    others.push(q);
                }
            }
            st.native += 4 * m;
            if others.len() == 3 {
                st.hits_c += 1;
                let mut key = [
                    t[0] as u32,
                    t[1] as u32,
                    t[2] as u32,
                    others[0] as u32,
                    others[1] as u32,
                    others[2] as u32,
                ];
                key.sort();
                if seen.insert(key) {
                    st.unique_c += 1;
                    let mut pts = t.to_vec();
                    pts.extend(&others);
                    rows.push(row_of(lay, &pts, Some(lay.kappa())));
                }
            }
        }
    }
    // K = T1 + T2
    rows.push(vec![(lay.kappa(), 1), (lay.tau(0), -1), (lay.tau(1), -1)]);
    RelSet { rows, st }
}

// ---------------------------------------------------------------------
// linear algebra mod ell
// ---------------------------------------------------------------------

/// Compact the used columns of a relation set; returns (rows, mapping new->old).
pub fn compact(rows: &[Row], ncols: usize) -> (Vec<Row>, Vec<usize>) {
    let mut used = vec![false; ncols];
    for r in rows {
        for &(c, _) in r {
            used[c as usize] = true;
        }
    }
    let mut newidx = vec![u32::MAX; ncols];
    let mut back = vec![];
    for c in 0..ncols {
        if used[c] {
            newidx[c] = back.len() as u32;
            back.push(c);
        }
    }
    let out = rows
        .iter()
        .map(|r| r.iter().map(|&(c, v)| (newidx[c as usize], v)).collect())
        .collect();
    (out, back)
}

fn signed(v: i64, ell: u64) -> u64 {
    if v >= 0 {
        v as u64 % ell
    } else {
        ell - ((-v) as u64 % ell)
    }
}

pub fn matvec(rows: &[Row], x: &[u64], ell: u64) -> Vec<u64> {
    rows.iter()
        .map(|r| {
            let mut s = 0u64;
            for &(c, v) in r {
                s = (s + mulmod(signed(v, ell), x[c as usize], ell)) % ell;
            }
            s
        })
        .collect()
}
pub fn matvec_t(rows: &[Row], y: &[u64], n: usize, ell: u64) -> Vec<u64> {
    let mut out = vec![0u64; n];
    for (r, row) in rows.iter().enumerate() {
        for &(c, v) in row {
            out[c as usize] = (out[c as usize] + mulmod(signed(v, ell), y[r], ell)) % ell;
        }
    }
    out
}

/// Berlekamp-Massey over F_ell: returns connection polynomial c (c[0]=1) with
/// sum_{i=0}^{L} c[i] s[n-i] = 0.
fn berlekamp_massey(s: &[u64], ell: u64) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let (mut l, mut m, mut bd) = (0usize, 1usize, 1u64);
    for n in 0..s.len() {
        let mut d = 0u64;
        for i in 0..=l.min(c.len() - 1) {
            d = (d + mulmod(c[i], s[n - i], ell)) % ell;
        }
        if d == 0 {
            m += 1;
        } else if 2 * l <= n {
            let t = c.clone();
            let coef = mulmod(d, invmod(bd, ell).unwrap(), ell);
            if c.len() < b.len() + m {
                c.resize(b.len() + m, 0);
            }
            for i in 0..b.len() {
                c[i + m] = (c[i + m] + ell - mulmod(coef, b[i], ell)) % ell;
            }
            l = n + 1 - l;
            b = t;
            bd = d;
            m = 1;
        } else {
            let coef = mulmod(d, invmod(bd, ell).unwrap(), ell);
            if c.len() < b.len() + m {
                c.resize(b.len() + m, 0);
            }
            for i in 0..b.len() {
                c[i + m] = (c[i + m] + ell - mulmod(coef, b[i], ell)) % ell;
            }
            m += 1;
        }
    }
    c.resize(l + 1, 0);
    c
}

pub struct LaOut {
    pub vec: Vec<u64>,
    pub ops: u64,
    pub matvecs: u64,
    pub attempts: u32,
}

/// Wiedemann on B = A^T D A (D random diagonal): a vector of ker A, or None.
pub fn wiedemann_kernel(rows: &[Row], n: usize, ell: u64, rng: &mut u64) -> Option<LaOut> {
    let nnz: u64 = rows.iter().map(|r| r.len() as u64).sum();
    let r = rows.len();
    let mut ops = 0u64;
    let mut matvecs = 0u64;
    for attempt in 1..=4u32 {
        let d: Vec<u64> = (0..r).map(|_| 1 + rnd(rng, ell - 1)).collect();
        let apply = |x: &[u64], ops: &mut u64, mv: &mut u64| -> Vec<u64> {
            let mut y = matvec(rows, x, ell);
            for (yy, &dd) in y.iter_mut().zip(d.iter()) {
                *yy = mulmod(*yy, dd, ell);
            }
            *ops += 2 * nnz + r as u64;
            *mv += 1;
            matvec_t(rows, &y, n, ell)
        };
        let v: Vec<u64> = (0..n).map(|_| rnd(rng, ell)).collect();
        let u: Vec<u64> = (0..n).map(|_| rnd(rng, ell)).collect();
        let mut seq = Vec::with_capacity(2 * n + 2);
        let mut y = v.clone();
        for _ in 0..2 * n + 2 {
            let mut s = 0u64;
            for (a, b) in u.iter().zip(y.iter()) {
                s = (s + mulmod(*a, *b, ell)) % ell;
            }
            ops += n as u64;
            seq.push(s);
            y = apply(&y, &mut ops, &mut matvecs);
        }
        let cpoly = berlekamp_massey(&seq, ell);
        ops += 2 * (seq.len() * seq.len()) as u64;
        // f(lambda) = sum c[i] lambda^(L-i); write f = lambda^e g, g(0) != 0
        let l = cpoly.len() - 1;
        // coefficient of lambda^k in f is c[l-k]
        let mut e = 0usize;
        while e <= l && cpoly[l - e] == 0 {
            e += 1;
        }
        // g(lambda) = sum_{k>=e} c[l-k] lambda^(k-e)
        let mut w = vec![0u64; n];
        let mut pw_v = v.clone();
        for k in e..=l {
            let coef = cpoly[l - k];
            for (wi, pv) in w.iter_mut().zip(pw_v.iter()) {
                *wi = (*wi + mulmod(coef, *pv, ell)) % ell;
            }
            ops += n as u64;
            if k < l {
                pw_v = apply(&pw_v, &mut ops, &mut matvecs);
            }
        }
        // now B^e w = 0; descend to the last nonzero iterate
        let mut z = w;
        let mut found = None;
        for _ in 0..=e + 1 {
            if z.iter().all(|&t| t == 0) {
                break;
            }
            let bz = apply(&z, &mut ops, &mut matvecs);
            if bz.iter().all(|&t| t == 0) {
                found = Some(z.clone());
                break;
            }
            z = bz;
        }
        if let Some(z) = found {
            let az = matvec(rows, &z, ell);
            ops += nnz;
            if az.iter().all(|&t| t == 0) {
                return Some(LaOut {
                    vec: z,
                    ops,
                    matvecs,
                    attempts: attempt,
                });
            }
        }
    }
    None
}

/// Dense kernel dimension of the relation matrix (diagnostic, uncharged).
pub fn dense_kernel(rows: &[Row], n: usize, ell: u64) -> (usize, Vec<u64>) {
    let mut m: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| {
            let mut v = vec![0u64; n];
            for &(c, x) in r {
                v[c as usize] = (v[c as usize] + signed(x, ell)) % ell;
            }
            v
        })
        .collect();
    let mut piv = vec![];
    let mut rk = 0;
    for col in 0..n {
        let Some(pi) = (rk..m.len()).find(|&i| m[i][col] != 0) else {
            continue;
        };
        m.swap(rk, pi);
        let iv = invmod(m[rk][col], ell).unwrap();
        for x in m[rk].iter_mut() {
            *x = mulmod(*x, iv, ell);
        }
        let pr = m[rk].clone();
        for i in 0..m.len() {
            if i != rk && m[i][col] != 0 {
                let a = ell - m[i][col];
                for (x, &y) in m[i].iter_mut().zip(pr.iter()) {
                    *x = (*x + mulmod(a, y, ell)) % ell;
                }
            }
        }
        piv.push(col);
        rk += 1;
    }
    let free: Vec<usize> = (0..n).filter(|c| !piv.contains(c)).collect();
    let mut kv = vec![0u64; n];
    if let Some(&f0) = free.first() {
        kv[f0] = 1;
        for (r, &pc) in piv.iter().enumerate() {
            kv[pc] = (ell - m[r][f0]) % ell;
        }
    }
    (free.len(), kv)
}

// ---------------------------------------------------------------------
// rho
// ---------------------------------------------------------------------

pub struct Walk {
    pub adders: Vec<(Elt, u64, u64)>, // element, alpha, beta (coefficients of g,h)
}

pub fn make_adders(c: &Curve, g: &Elt, h: &Elt, r: usize, ell: u64, rng: &mut u64) -> Walk {
    let mut adders = vec![];
    for _ in 0..r {
        let a = 1 + rnd(rng, 1 << 14);
        let b = 1 + rnd(rng, 1 << 14);
        let e = c.add(&c.mul_scalar(g, a as u128), &c.mul_scalar(h, b as u128));
        adders.push((e, a % ell, b % ell));
    }
    Walk { adders }
}

pub struct RhoOut {
    pub x: Option<u64>,
    pub adds: u64,
    pub steps: u64,
    pub dps: u64,
    pub native: u64,
    pub restarts: u64,
}

/// Distinguished-point rho in <g> of prime order ell, h = x g.
pub fn rho(
    c: &Curve,
    g: &Elt,
    h: &Elt,
    ell: u64,
    dbits: u32,
    w: &Walk,
    rng: &mut u64,
    max_steps: u64,
) -> RhoOut {
    let n0 = c.n_ops.get();
    let r = w.adders.len() as u64;
    let mask = (1u64 << dbits) - 1;
    let mut table: HashMap<Vec<u32>, (u64, u64)> = HashMap::new();
    let mut out = RhoOut {
        x: None,
        adds: 0,
        steps: 0,
        dps: 0,
        native: 0,
        restarts: 0,
    };
    // start points: s <- s + adder (a,b tracked)
    let (a0, b0) = (1 + rnd(rng, 1 << 14), 1 + rnd(rng, 1 << 14));
    let mut start = c.add(&c.mul_scalar(g, a0 as u128), &c.mul_scalar(h, b0 as u128));
    let (mut sa, mut sb) = (a0 % ell, b0 % ell);
    let trail_cap = 40 * (mask + 1);
    while out.steps < max_steps {
        // fresh start
        let j = rnd(rng, r) as usize;
        start = c.add(&start, &w.adders[j].0);
        sa = (sa + w.adders[j].1) % ell;
        sb = (sb + w.adders[j].2) % ell;
        out.restarts += 1;
        let (mut x, mut a, mut b) = (start.clone(), sa, sb);
        let mut len = 0u64;
        loop {
            let hs = fnv(&x.key);
            out.native += 1;
            if hs & mask == 0 {
                out.dps += 1;
                out.native += 1;
                if let Some(&(pa, pb)) = table.get(&x.key) {
                    if (pa, pb) != (a, b) {
                        // (a - pa) g = (pb - b) h
                        let num = (a + ell - pa) % ell;
                        let den = (pb + ell - b) % ell;
                        if let Some(iv) = invmod(den, ell) {
                            let xx = mulmod(num, iv, ell);
                            out.x = Some(xx);
                            out.adds = c.n_ops.get() - n0;
                            return out;
                        }
                    }
                } else {
                    table.insert(x.key.clone(), (a, b));
                }
                break;
            }
            if len >= trail_cap || out.steps >= max_steps {
                break;
            }
            let j = ((hs >> 40) % r) as usize;
            x = c.add(&x, &w.adders[j].0);
            a = (a + w.adders[j].1) % ell;
            b = (b + w.adders[j].2) % ell;
            len += 1;
            out.steps += 1;
        }
    }
    out.adds = c.n_ops.get() - n0;
    out
}

pub fn ev1_table(c: &Curve) -> Vec<[u64; 4]> {
    c.pts
        .iter()
        .map(|q| {
            let v = c.eval_vec(1, q);
            [v[0] as u64, v[1] as u64, v[2] as u64, v[3] as u64]
        })
        .collect()
}

// ---------------------------------------------------------------------
// index-calculus attempt
// ---------------------------------------------------------------------

#[derive(Default, Clone, Debug)]
pub struct IcOut {
    pub ok: bool,
    pub reason: String,
    pub x: Option<u64>,
    pub phi: f64,
    pub m: usize,
    pub rows: usize,
    pub ncols_used: usize,
    pub unique_a: u64,
    pub hits_a: u64,
    pub pairs: u64,
    pub unique_c: u64,
    pub triples: u64,
    pub fibre_rows: usize,
    pub kernel_dim: usize,
    pub la_ops: u64,
    pub la_matvecs: u64,
    pub la_attempts: u32,
    pub group_ops: u64,
    pub native_p: u64,
    pub native_ell: u64,
    pub indiv_steps: u64,
    pub indiv_records: usize,
}

#[allow(clippy::too_many_arguments)]
pub fn ic_attempt(
    c: &Curve,
    ev1: &[[u64; 4]],
    ell: u64,
    g: &Elt,
    h: &Elt,
    phi: f64,
    c_budget_factor: f64,
    rng: &mut u64,
) -> IcOut {
    let mut out = IcOut {
        phi,
        ..Default::default()
    };
    let n = c.pts.len();
    let m = ((phi * n as f64).ceil() as usize).clamp(4, n);
    out.m = m;
    // factor base: D0 plus a random subset
    let mut in_f = vec![false; n];
    for &d in &c.d0 {
        in_f[d] = true;
    }
    let mut cnt = c.d0.len();
    while cnt < m {
        let i = rnd(rng, n as u64) as usize;
        if !in_f[i] {
            in_f[i] = true;
            cnt += 1;
        }
    }
    let fidx: Vec<usize> = (0..n).filter(|&i| in_f[i]).collect();
    let lay = make_layout(c, &fidx);
    let c0 = c.cnt.get() - c.inner.get();
    let ops0 = c.n_ops.get();
    // relations A + B (+ C supplement when asked)
    let mut rs = collect(c, &lay, ev1, true, 0, rng);
    out.pairs = rs.st.pairs;
    out.hits_a = rs.st.hits_a;
    out.unique_a = rs.st.unique_a;
    out.fibre_rows = rs.rows.len() - rs.st.unique_a as usize - 1;
    let mut native_p = rs.st.native;
    // rank monitor (dense, uncharged, diagnostic): add general hyperplane
    // sections (C) in chunks while the kernel dimension exceeds 1
    let budget = (c_budget_factor * (m as f64).powi(3)) as u64 + 1000;
    let mut spent = 0u64;
    let mut seen_rows: HashSet<Vec<(u32, i64)>> = HashSet::new();
    if c_budget_factor > 0.0 {
        for r in &rs.rows {
            let mut key = r.clone();
            key.sort();
            seen_rows.insert(key);
        }
    }
    let (rows, back, kd, dense_v) = loop {
        let (rows, back) = compact(&rs.rows, lay.ncols());
        let nused = back.len();
        let (kd, dense_v) = if rows.len() + 1 >= nused {
            dense_kernel(&rows, nused, ell)
        } else {
            (usize::MAX, vec![])
        };
        if kd == 1 || c_budget_factor <= 0.0 || spent >= budget {
            break (rows, back, kd, dense_v);
        }
        let chunk = 2000u64.min(budget - spent);
        let r2 = collect(c, &lay, ev1, false, chunk, rng);
        spent += chunk;
        native_p += r2.st.native;
        out.triples += r2.st.triples;
        for r in r2.rows {
            let mut key = r.clone();
            key.sort();
            if seen_rows.insert(key) {
                rs.rows.push(r);
                out.unique_c += 1;
            }
        }
    };
    out.rows = rs.rows.len();
    let nused = back.len();
    out.ncols_used = nused;
    out.kernel_dim = if kd == usize::MAX { 0 } else { kd };
    if kd != 1 {
        out.reason = if kd == usize::MAX {
            format!(
                "too few relations: {} rows, {} used columns",
                rows.len(),
                nused
            )
        } else {
            format!("kernel dimension {kd}")
        };
        out.native_p = native_p + (c.cnt.get() - c.inner.get() - c0);
        return out;
    }
    let Some(la) = wiedemann_kernel(&rows, nused, ell, rng) else {
        out.reason = "wiedemann failed".into();
        out.native_p = native_p + (c.cnt.get() - c.inner.get() - c0);
        return out;
    };
    out.la_ops = la.ops;
    out.la_matvecs = la.matvecs;
    out.la_attempts = la.attempts;
    out.native_ell = la.ops;
    // sanity: Wiedemann vector proportional to dense kernel vector
    {
        let i0 = (0..nused).find(|&i| dense_v[i] != 0).unwrap();
        let sc = mulmod(la.vec[i0], invmod(dense_v[i0], ell).unwrap(), ell);
        for i in 0..nused {
            if mulmod(sc, dense_v[i], ell) != la.vec[i] {
                out.reason = "wiedemann/dense kernel mismatch".into();
                return out;
            }
        }
    }
    // lambda per rational point
    let pa = c.d0[0];
    let mut lam = vec![0u64; n];
    let mut usable = vec![false; n];
    usable[pa] = true;
    for (k, &oc) in back.iter().enumerate() {
        if oc < lay.nf {
            // column oc corresponds to the oc-th non-Pa F point
            let pt = fidx.iter().copied().filter(|&i| i != pa).nth(oc).unwrap();
            lam[pt] = la.vec[k];
            usable[pt] = true;
        }
    }
    if c.d0.iter().any(|&d| !usable[d]) {
        out.reason = "D0 point not covered by relations".into();
        return out;
    }
    let d0sum: u64 = c.d0.iter().fold(0u64, |s, &d| (s + lam[d]) % ell);
    // individual logs
    let walk = make_adders(c, g, h, 16, ell, rng);
    let (a0, b0) = (1 + rnd(rng, 1 << 12), 1 + rnd(rng, 1 << 12));
    let mut u = c.add(&c.mul_scalar(g, a0 as u128), &c.mul_scalar(h, b0 as u128));
    let (mut a, mut b) = (a0 % ell, b0 % ell);
    let mut recs: Vec<(u64, u64, u64)> = vec![];
    let max_steps = 400_000u64;
    let mut steps = 0u64;
    while recs.len() < 4 && steps < max_steps {
        let supp = c.rational_support(&u.w);
        if supp.len() == 4 && supp.iter().all(|&i| usable[i]) {
            let s = supp.iter().fold(0u64, |t, &i| (t + lam[i]) % ell);
            let s = (s + ell - d0sum) % ell;
            recs.push((s, a, b));
        }
        let j = rnd(rng, 16) as usize;
        u = c.add(&u, &walk.adders[j].0);
        a = (a + walk.adders[j].1) % ell;
        b = (b + walk.adders[j].2) % ell;
        steps += 1;
    }
    out.indiv_steps = steps;
    out.indiv_records = recs.len();
    out.group_ops = c.n_ops.get() - ops0;
    out.native_p = native_p + (c.cnt.get() - c.inner.get() - c0);
    // solve x from pairs of records, verify
    let mut found = None;
    'outer: for i in 0..recs.len() {
        for j in i + 1..recs.len() {
            let (s1, a1, b1) = recs[i];
            let (s2, a2, b2) = recs[j];
            let coef = (mulmod(s1, b2, ell) + ell - mulmod(s2, b1, ell)) % ell;
            let rhs = (mulmod(s2, a1, ell) + ell - mulmod(s1, a2, ell)) % ell;
            if let Some(iv) = invmod(coef, ell) {
                let x = mulmod(rhs, iv, ell);
                if c.mul_scalar(g, x as u128).key == h.key {
                    found = Some(x);
                    break 'outer;
                }
            }
        }
    }
    match found {
        Some(x) => {
            out.ok = true;
            out.x = Some(x);
        }
        None => out.reason = format!("no verified log from {} records", recs.len()),
    }
    out
}
