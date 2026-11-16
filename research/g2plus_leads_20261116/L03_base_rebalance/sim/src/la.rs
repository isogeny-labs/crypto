use crate::rng::Rng;

// ── counters ─────────────────────────────────────────────────────────────

/// Native operations on residues: modular multiplications and additions or
/// subtractions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nat {
    pub mul: u64,
    pub add: u64,
}

impl Nat {
    pub fn merge(&mut self, o: Nat) {
        self.mul += o.mul;
        self.add += o.add;
    }
}

// ── arithmetic modulo r (prime, < 2^50 for the fast path) ───────────────

#[inline]
pub(crate) fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
#[inline]
pub(crate) fn addm(a: u64, b: u64, r: u64) -> u64 {
    let s = a + b;
    if s >= r {
        s - r
    } else {
        s
    }
}
#[inline]
pub(crate) fn subm(a: u64, b: u64, r: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + r - b
    }
}
pub(crate) fn powm(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut res = 1;
    while e > 0 {
        if e & 1 == 1 {
            res = mulmod(res, a, r);
        }
        a = mulmod(a, a, r);
        e >>= 1;
    }
    res
}
pub(crate) fn invm(a: u64, r: u64) -> u64 {
    powm(a, r - 2, r)
}

/// Factor-base entries one relation can hold.  Genus 3 uses three; the
/// genus ladder (`genus_ladder.rs`, genus up to 5) and the plane-quartic
/// line relations (four points) share the struct.
pub(crate) const REL_FB: usize = 5;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Rel {
    pub(crate) fb: [(u32, i8); REL_FB],
    pub(crate) nfb: u8,
    pub(crate) lp: [(u32, i8); 2],
    pub(crate) nlp: u8,
    /// Coefficient of `t`: `−h·b mod r`.
    pub(crate) tcoef: u64,
    /// `h·a mod r`.
    pub(crate) rhs: u64,
}

impl Rel {
    pub(crate) fn endpoints(&self) -> (u32, i8, u32, i8) {
        match self.nlp {
            0 => (0, 0, 0, 0),
            1 => (0, 0, self.lp[0].0, self.lp[0].1),
            _ => (self.lp[0].0, self.lp[0].1, self.lp[1].0, self.lp[1].1),
        }
    }
}

pub(crate) type Row = (Vec<(u32, u64)>, u64);

/// `sa·a + sb·b` as sorted sparse vectors; `neg*` flips the sign.
pub(crate) fn combine(
    a: &[(u32, u64)],
    neg_a: bool,
    b: &[(u32, u64)],
    neg_b: bool,
    r: u64,
    nat: &mut Nat,
) -> Vec<(u32, u64)> {
    let f = |v: u64, neg: bool| if neg && v != 0 { r - v } else { v };
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    nat.add += (a.len() + b.len()) as u64;
    while i < a.len() || j < b.len() {
        if j == b.len() || (i < a.len() && a[i].0 < b[j].0) {
            out.push((a[i].0, f(a[i].1, neg_a)));
            i += 1;
        } else if i == a.len() || b[j].0 < a[i].0 {
            out.push((b[j].0, f(b[j].1, neg_b)));
            j += 1;
        } else {
            let v = addm(f(a[i].1, neg_a), f(b[j].1, neg_b), r);
            if v != 0 {
                out.push((a[i].0, v));
            }
            i += 1;
            j += 1;
        }
    }
    out
}

pub(crate) struct Dsu {
    pub(crate) parent: Vec<u32>,
    pub(crate) edges: Vec<u32>,
    pub(crate) verts: Vec<u32>,
    pub(crate) ops: u64,
}

impl Dsu {
    pub(crate) fn new() -> Dsu {
        Dsu {
            parent: vec![0],
            edges: vec![0],
            verts: vec![1],
            ops: 0,
        }
    }
    pub(crate) fn add_vertex(&mut self) -> u32 {
        let id = self.parent.len() as u32;
        self.parent.push(id);
        self.edges.push(0);
        self.verts.push(1);
        id
    }
    pub(crate) fn find(&mut self, mut x: u32) -> u32 {
        while self.parent[x as usize] != x {
            let gp = self.parent[self.parent[x as usize] as usize];
            self.parent[x as usize] = gp;
            x = gp;
            self.ops += 1;
        }
        x
    }
    pub(crate) fn edge(&mut self, a: u32, b: u32) {
        let (ra, rb) = (self.find(a), self.find(b));
        if ra == rb {
            self.edges[ra as usize] += 1;
        } else {
            self.parent[rb as usize] = ra;
            self.edges[ra as usize] += self.edges[rb as usize] + 1;
            self.verts[ra as usize] += self.verts[rb as usize];
        }
        self.ops += 1;
    }
    /// Independent cycles in the component of the vertex "1".
    pub(crate) fn cycles0(&mut self) -> i64 {
        let r0 = self.find(0) as usize;
        self.edges[r0] as i64 - self.verts[r0] as i64 + 1
    }
}

/// The cycle rows of the relation graph: large-prime columns eliminated
/// along a breadth-first spanning tree of the component of the vertex "1".
pub(crate) fn build_rows(
    rels: &[Rel],
    nverts: usize,
    tcol: u32,
    r: u64,
    nat: &mut Nat,
) -> Vec<Row> {
    let neg = |e: i8| if e > 0 { 1u64 } else { r - 1 }; // coefficient as residue
    let frow = |rel: &Rel| -> Vec<(u32, u64)> {
        let mut v: Vec<(u32, u64)> = rel.fb[..rel.nfb as usize]
            .iter()
            .map(|&(c, e)| (c, neg(e)))
            .collect();
        v.push((tcol, rel.tcoef));
        v.sort_unstable_by_key(|x| x.0);
        v
    };
    // adjacency over non-loop edges
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); nverts];
    for (i, rel) in rels.iter().enumerate() {
        if rel.nlp > 0 {
            let (a, _, b, _) = rel.endpoints();
            adj[a as usize].push(i as u32);
            adj[b as usize].push(i as u32);
            nat.add += 2;
        }
    }
    let mut beta: Vec<Option<(Vec<(u32, u64)>, u64)>> = vec![None; nverts];
    beta[0] = Some((Vec::new(), 0));
    let mut is_tree = vec![false; rels.len()];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0u32);
    while let Some(u) = queue.pop_front() {
        let edges = adj[u as usize].clone();
        for ei in edges {
            let rel = &rels[ei as usize];
            let (a, ea, b, eb) = rel.endpoints();
            let (v, ev, eu) = if a == u { (b, eb, ea) } else { (a, ea, eb) };
            if beta[v as usize].is_some() {
                continue;
            }
            is_tree[ei as usize] = true;
            let (bu, cu) = beta[u as usize].clone().expect("parent reached");
            // ℓ_v = ev·(rhs − F − tcoef t) − ev·eu·ℓ_u
            let fv = frow(rel);
            let neg_f = ev > 0;
            let neg_u = (ev as i32) * (eu as i32) > 0;
            let terms = combine(&fv, neg_f, &bu, neg_u, r, nat);
            let mut c = if ev > 0 { rel.rhs } else { subm(0, rel.rhs, r) };
            let cu_term = if (ev as i32) * (eu as i32) > 0 {
                subm(0, cu, r)
            } else {
                cu
            };
            c = addm(c, cu_term, r);
            nat.add += 2;
            beta[v as usize] = Some((terms, c));
            queue.push_back(v);
        }
    }
    let mut rows = Vec::new();
    for (i, rel) in rels.iter().enumerate() {
        if is_tree[i] {
            continue;
        }
        let (a, ea, b, eb) = rel.endpoints();
        if rel.nlp > 0 && (beta[a as usize].is_none() || beta[b as usize].is_none()) {
            continue;
        }
        let mut terms = frow(rel);
        let mut rhs = rel.rhs;
        if rel.nlp > 0 {
            if a != 0 {
                let (ba, ca) = beta[a as usize].as_ref().expect("reached");
                terms = combine(&terms, false, ba, ea < 0, r, nat);
                rhs = if ea > 0 {
                    subm(rhs, *ca, r)
                } else {
                    addm(rhs, *ca, r)
                };
                nat.add += 1;
            }
            if b != 0 {
                let (bb, cb) = beta[b as usize].as_ref().expect("reached");
                terms = combine(&terms, false, bb, eb < 0, r, nat);
                rhs = if eb > 0 {
                    subm(rhs, *cb, r)
                } else {
                    addm(rhs, *cb, r)
                };
                nat.add += 1;
            }
        }
        rows.push((terms, rhs));
    }
    rows
}

/// Peel columns of weight one (and the rows holding them) from `rows`; the
/// column `tcol` is never peeled.  Returns the surviving rows and the number
/// of surviving columns, columns renumbered densely, `t` last.
pub(crate) fn peel(rows: &[Row], ncols: usize, tcol: u32, nat: &mut Nat) -> (Vec<Row>, usize, u32) {
    let mut colrows: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut w = vec![0u32; ncols];
    for (i, (t, _)) in rows.iter().enumerate() {
        for &(c, _) in t {
            colrows[c as usize].push(i as u32);
            w[c as usize] += 1;
            nat.add += 1;
        }
    }
    let mut alive = vec![true; rows.len()];
    let mut stack: Vec<u32> = (0..ncols as u32)
        .filter(|&c| w[c as usize] == 1 && c != tcol)
        .collect();
    while let Some(c) = stack.pop() {
        if w[c as usize] != 1 {
            continue;
        }
        let Some(&ri) = colrows[c as usize].iter().find(|&&i| alive[i as usize]) else {
            continue;
        };
        alive[ri as usize] = false;
        for &(cc, _) in &rows[ri as usize].0 {
            nat.add += 1;
            w[cc as usize] -= 1;
            if w[cc as usize] == 1 && cc != tcol {
                stack.push(cc);
            }
        }
    }
    let mut map = vec![u32::MAX; ncols];
    let mut n = 0u32;
    for c in 0..ncols {
        if c as u32 != tcol && w[c] > 0 {
            map[c] = n;
            n += 1;
        }
    }
    map[tcol as usize] = n;
    let new_t = n;
    let mut out = Vec::new();
    for (i, (t, rhs)) in rows.iter().enumerate() {
        if alive[i] {
            let mut nt: Vec<(u32, u64)> = t.iter().map(|&(c, v)| (map[c as usize], v)).collect();
            nt.sort_unstable_by_key(|x| x.0);
            out.push((nt, *rhs));
        }
    }
    (out, n as usize + 1, new_t)
}

/// `y = A x` (rows) with the operation count.
pub(crate) fn mat_vec(rows: &[Row], x: &[u64], y: &mut [u64], r: u64, nat: &mut Nat) {
    for (i, (t, _)) in rows.iter().enumerate() {
        let mut acc = 0u64;
        for &(c, v) in t {
            let xv = x[c as usize];
            if v == 1 {
                acc = addm(acc, xv, r);
            } else if v == r - 1 {
                acc = subm(acc, xv, r);
            } else {
                nat.mul += 1;
                acc = addm(acc, mulmod(v, xv, r), r);
            }
            nat.add += 1;
        }
        y[i] = acc;
    }
}

/// `z = Aᵀ y`.
pub(crate) fn mat_t_vec(rows: &[Row], y: &[u64], z: &mut [u64], r: u64, nat: &mut Nat) {
    z.iter_mut().for_each(|v| *v = 0);
    for (i, (t, _)) in rows.iter().enumerate() {
        let yv = y[i];
        if yv == 0 {
            continue;
        }
        for &(c, v) in t {
            let term = if v == 1 {
                yv
            } else if v == r - 1 {
                subm(0, yv, r)
            } else {
                nat.mul += 1;
                mulmod(v, yv, r)
            };
            z[c as usize] = addm(z[c as usize], term, r);
            nat.add += 1;
        }
    }
}

/// `out = AᵀDA v`.
pub(crate) fn m_apply(
    rows: &[Row],
    d: &[u64],
    v: &[u64],
    tmp: &mut [u64],
    out: &mut [u64],
    r: u64,
    nat: &mut Nat,
) {
    mat_vec(rows, v, tmp, r, nat);
    for (t, &di) in tmp.iter_mut().zip(d.iter()) {
        nat.mul += 1;
        *t = mulmod(*t, di, r);
    }
    mat_t_vec(rows, tmp, out, r, nat);
}

/// Berlekamp–Massey; returns `c[0..=L]` with `c[0] = 1`,
/// `Σ_i c[i]·s[n−i] = 0` for `n ≥ L`.
pub(crate) fn berlekamp_massey(s: &[u64], r: u64, nat: &mut Nat) -> Vec<u64> {
    let mut c = vec![1u64];
    let mut b = vec![1u64];
    let mut l = 0usize;
    let mut m = 1usize;
    let mut binv = 1u64;
    for n in 0..s.len() {
        let mut d = s[n];
        for i in 1..=l.min(c.len() - 1) {
            nat.mul += 1;
            nat.add += 1;
            d = addm(d, mulmod(c[i], s[n - i], r), r);
        }
        if d == 0 {
            m += 1;
            continue;
        }
        let coef = {
            nat.mul += 1;
            mulmod(d, binv, r)
        };
        let t = c.clone();
        if c.len() < b.len() + m {
            c.resize(b.len() + m, 0);
        }
        for (i, &bv) in b.iter().enumerate() {
            nat.mul += 1;
            nat.add += 1;
            c[i + m] = subm(c[i + m], mulmod(coef, bv, r), r);
        }
        if 2 * l <= n {
            l = n + 1 - l;
            b = t;
            nat.mul += pivot_inversion_steps(r);
            binv = invm(d, r);
            m = 1;
        } else {
            m += 1;
        }
    }
    c.truncate(l + 1);
    c.resize(l + 1, 0);
    c
}

/// Solve `A x = c` (rows × cols, full column rank) by Wiedemann on
/// `AᵀDA x = AᵀD c`.  `None` if the recurrence is degenerate or the residual
/// is nonzero.
pub(crate) fn wiedemann(
    rows: &[Row],
    ncols: usize,
    r: u64,
    rng: &mut Rng,
    dense_u: bool,
    nat: &mut Nat,
) -> Option<Vec<u64>> {
    let nr = rows.len();
    let d: Vec<u64> = (0..nr).map(|_| rng.range(1, r)).collect();
    // b' = Aᵀ D c
    let mut tmp = vec![0u64; nr];
    for ((t, row), &di) in tmp.iter_mut().zip(rows.iter()).zip(d.iter()) {
        nat.mul += 1;
        *t = mulmod(row.1, di, r);
    }
    let mut bp = vec![0u64; ncols];
    mat_t_vec(rows, &tmp, &mut bp, r, nat);
    // projection u
    let mut proj: Vec<(usize, u64)> = Vec::new();
    if dense_u || ncols <= 16 {
        proj.extend((0..ncols).map(|i| (i, rng.range(1, r))));
    } else {
        for _ in 0..16 {
            proj.push((rng.range(0, ncols as u64) as usize, rng.range(1, r)));
        }
    }
    let len = 2 * ncols + 2;
    let mut seq = Vec::with_capacity(len);
    let mut v = bp.clone();
    let mut w = vec![0u64; ncols];
    for _ in 0..len {
        let mut s = 0u64;
        for &(i, uv) in &proj {
            nat.mul += 1;
            nat.add += 1;
            s = addm(s, mulmod(uv, v[i], r), r);
        }
        seq.push(s);
        m_apply(rows, &d, &v, &mut tmp, &mut w, r, nat);
        std::mem::swap(&mut v, &mut w);
    }
    let c = berlekamp_massey(&seq, r, nat);
    let l = c.len() - 1;
    if l == 0 || c[l] == 0 {
        return None;
    }
    // x = −c_L⁻¹ Σ_{j=0}^{L−1} c_{L−1−j}... via Horner on M:
    // P(M) b' = 0 with P(z) = z^L + c_1 z^{L−1} + … + c_L.
    // x = −c_L⁻¹ (M^{L−1} b' + c_1 M^{L−2} b' + … + c_{L−1} b').
    let mut acc = vec![0u64; ncols];
    let mut v = bp.clone();
    for j in 0..l {
        // term M^j b' has coefficient c_{L−1−j}
        let coef = c[l - 1 - j];
        for (a, &vi) in acc.iter_mut().zip(v.iter()) {
            nat.mul += 1;
            nat.add += 1;
            *a = addm(*a, mulmod(coef, vi, r), r);
        }
        if j + 1 < l {
            m_apply(rows, &d, &v, &mut tmp, &mut w, r, nat);
            std::mem::swap(&mut v, &mut w);
        }
    }
    nat.mul += pivot_inversion_steps(r);
    let sc = subm(0, invm(c[l], r), r);
    for a in acc.iter_mut() {
        nat.mul += 1;
        *a = mulmod(*a, sc, r);
    }
    // residual A x = c
    let mut ax = vec![0u64; nr];
    mat_vec(rows, &acc, &mut ax, r, nat);
    if ax.iter().zip(rows.iter()).all(|(a, row)| *a == row.1) {
        Some(acc)
    } else {
        None
    }
}
/// `2w` steps of the binary extended Euclid for a `w`-bit modulus, each
/// priced as one addition modulo `r` (`docs/units/BITOPS.md`: `field_inv`
/// is `2b · field_add`, so the steps reprice to exactly one `field_inv`
/// at `r`'s width).
pub fn pivot_inversion_steps(m: u64) -> u64 {
    2 * u64::from(64 - m.leading_zeros())
}
