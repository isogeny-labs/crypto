//! Algebraic index calculus on Koblitz curves over GF(2^n), n ≤ 63, with a
//! linear two-summand decomposition, beside the strong-rho reference on the
//! same target.
//!
//! Factor base: every point whose x lies in the F2-subspace
//! `V = span{1, z, …, z^{d−1}}` of the polynomial basis (x ≠ 0), one
//! column per x (both signs).  Decomposition of a point R with known
//! logarithm into `R = ε1 P1 + ε2 P2`, `P1, P2` in the base, solves
//! Semaev's third summation polynomial
//!
//!     S3(x1, x2, xR) = (x1 x2 + xR (x1 + x2))² + x1 x2 xR + b
//!                    = p² + xR p + xR² s² + b,     s = x1 + x2, p = x1 x2.
//!
//! Squaring is F2-linear, so S3 = 0 is F2-affine in the coordinates of
//! `(s, p)`.  For x1, x2 in V, s lies in V and p in
//! `W = span{1, …, z^{2d−2}}` with no reduction (2d − 1 ≤ n), so the Weil
//! descent of S3 is a linear system of n equations in 3d − 1 unknowns.
//! Each solution `(s, p)` is split by solving `x1² + s x1 = p` in
//! `F2[z]_{<d}`, again linear (d unknowns).  Every relation found is
//! re-added in the group before it is used.  With two points per relation
//! the linear algebra mod r is a graph: union-find with signed offsets.
//!
//! Cost model: a trial succeeds with probability about `2^{2d−n−2}`, the
//! base has about `2^{d−1}` columns, so a solve costs about `2^{n−d+1}`
//! trials of polynomial cost; it is smallest near `d = (n+1)/3`, about
//! `2^{2n/3}`.
//!
//! Usage: `ic_m2_linear <a> <n> <d|0> <targets> <seed> <out.json>`
//! (`d = 0` selects `(n + 1) / 3`).

use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::koblitz_strong_rho::{
    RawPoint, StrongRho, StrongRhoCharges, StrongRhoParams,
};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;
use num_traits::ToPrimitive;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;
use std::time::Instant;

type Pt = Option<(u64, u64)>;

struct Curve {
    f: Gf2,
    n: u32,
    a: u64,
    b: u64,
    irr: u64,
}

#[derive(Default, Clone, Copy)]
struct Counts {
    group_additions: u64,
    trials: u64,
    consistent: u64,
    split_attempts: u64,
    relations: u64,
    gauss_xors: u64,
    field_muls: u64,
}

impl Curve {
    fn neg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, x ^ y))
    }

    fn add(&self, p: Pt, q: Pt, c: &mut Counts) -> Pt {
        c.group_additions += 1;
        let Some((x1, y1)) = p else { return q };
        let Some((x2, y2)) = q else { return p };
        let f = &self.f;
        if x1 == x2 {
            if y1 ^ y2 == x2 || x1 == 0 {
                return None;
            }
            let l = x1 ^ f.mul(y1, f.inv(x1));
            let x3 = f.sqr(l) ^ l ^ self.a;
            let y3 = f.sqr(x1) ^ f.mul(l ^ 1, x3);
            return Some((x3, y3));
        }
        let l = f.mul(y1 ^ y2, f.inv(x1 ^ x2));
        let x3 = f.sqr(l) ^ l ^ x1 ^ x2 ^ self.a;
        let y3 = f.mul(l, x1 ^ x3) ^ x3 ^ y1;
        Some((x3, y3))
    }

    fn mul(&self, p: Pt, k: u64, c: &mut Counts) -> Pt {
        let mut acc = None;
        for bit in (0..64 - k.leading_zeros()).rev() {
            acc = self.add(acc, acc, c);
            if (k >> bit) & 1 == 1 {
                acc = self.add(acc, p, c);
            }
        }
        acc
    }

    fn frob(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (self.f.sqr(x), self.f.sqr(y)))
    }

    /// A point with this x, if any: `y = x z`, `z² + z = x + a + b/x²`.
    fn lift_x(&self, x: u64) -> Pt {
        if x == 0 {
            return None;
        }
        let f = &self.f;
        let c = x ^ self.a ^ f.mul(self.b, f.sqr(f.inv(x)));
        let (mut t, mut tr) = (c, c);
        for _ in 1..self.n {
            t = f.sqr(t);
            tr ^= t;
        }
        if tr != 0 {
            return None;
        }
        let (mut t, mut z) = (c, c);
        for _ in 0..(self.n - 1) / 2 {
            t = f.sqr(f.sqr(t));
            z ^= t;
        }
        Some((x, f.mul(x, z)))
    }

    /// `v · z` in the field.
    #[inline(always)]
    fn times_z(&self, v: u64) -> u64 {
        let w = v << 1;
        if (w >> self.n) & 1 == 1 {
            w ^ self.irr
        } else {
            w
        }
    }
}

fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn invmod(a: u64, r: u64) -> u64 {
    let (mut e, mut base, mut acc) = (r - 2, a % r, 1u64);
    while e != 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, base, r);
        }
        base = mulmod(base, base, r);
        e >>= 1;
    }
    acc
}

/// Solve `Σ u_k cols[k] = rhs` over F2 (vectors and combinations as u64
/// bit sets); returns a particular solution and a kernel basis, or None.
fn solve_f2(cols: &[u64], rhs: u64, xors: &mut u64) -> Option<(u64, Vec<u64>)> {
    let mut piv: [(u64, u64); 64] = [(0, 0); 64];
    let mut kernel = Vec::new();
    for (k, &col) in cols.iter().enumerate() {
        let (mut v, mut comb) = (col, 1u64 << k);
        while v != 0 {
            let top = 63 - v.leading_zeros() as usize;
            if piv[top].0 == 0 {
                piv[top] = (v, comb);
                break;
            }
            v ^= piv[top].0;
            comb ^= piv[top].1;
            *xors += 1;
        }
        if v == 0 {
            kernel.push(comb);
        }
    }
    let (mut v, mut comb) = (rhs, 0u64);
    while v != 0 {
        let top = 63 - v.leading_zeros() as usize;
        if piv[top].0 == 0 {
            return None;
        }
        v ^= piv[top].0;
        comb ^= piv[top].1;
        *xors += 1;
    }
    Some((comb, kernel))
}

/// Carry-less product of two polynomials over F2 (degrees < 32 each).
fn clmul(a: u64, b: u64) -> u64 {
    let mut acc = 0;
    let mut a = a;
    let mut i = 0;
    while a != 0 {
        if a & 1 == 1 {
            acc ^= b << i;
        }
        a >>= 1;
        i += 1;
    }
    acc
}

struct Decomposer {
    d: usize,
    /// `z^{2j}` in the field, j < 2d − 1.
    z2: Vec<u64>,
    /// Base column of each x in V (u32::MAX when x is not rational).
    col_of_x: Vec<u32>,
    points: Vec<(u64, u64)>,
}

/// A verified two-term relation `R = Σ coef_i P_{col_i}`.
#[derive(Clone, Debug)]
struct Rel {
    terms: Vec<(u32, i64)>,
}

impl Decomposer {
    /// All decompositions `R = ε1 P1 + ε2 P2` (or `±P1`, `±2P1`) of R with
    /// P1, P2 in the base, each verified in the group.
    fn decompose(&self, cv: &Curve, r: (u64, u64), c: &mut Counts) -> Vec<Rel> {
        let mut out = Vec::new();
        let d = self.d;
        let (xr, _) = r;
        c.trials += 1;
        // Direct hit: x(R) in V.
        if xr < (1u64 << d) && self.col_of_x[xr as usize] != u32::MAX {
            let k = self.col_of_x[xr as usize];
            let p = Some(self.points[k as usize]);
            let e = if p == Some(r) { 1 } else { -1 };
            out.push(Rel {
                terms: vec![(k, e)],
            });
        }
        // Columns: s-part xR² z^{2i} (i < d), p-part z^{2j} + xR z^j (j < 2d−1).
        let f = &cv.f;
        let xr2 = f.sqr(xr);
        c.field_muls += 1;
        let mut cols = Vec::with_capacity(3 * d - 1);
        let mut t = xr2;
        for _ in 0..d {
            cols.push(t);
            t = cv.times_z(cv.times_z(t));
        }
        let mut t = xr;
        for j in 0..2 * d - 1 {
            cols.push(self.z2[j] ^ t);
            t = cv.times_z(t);
        }
        let Some((part, kernel)) = solve_f2(&cols, cv.b, &mut c.gauss_xors) else {
            return out;
        };
        c.consistent += 1;
        let dim = kernel.len().min(12);
        let dmask = (1u64 << d) - 1;
        for sel in 0u64..(1u64 << dim) {
            let mut u = part;
            for (i, &kv) in kernel.iter().take(dim).enumerate() {
                if (sel >> i) & 1 == 1 {
                    u ^= kv;
                }
            }
            let s = u & dmask;
            let p = u >> d;
            c.split_attempts += 1;
            // x1² + s x1 = p in F2[z]_{<d}.
            let mut scol = Vec::with_capacity(d);
            for i in 0..d {
                scol.push((1u64 << (2 * i)) ^ clmul(s, 1u64 << i));
            }
            let Some((x1, _)) = solve_f2(&scol, p, &mut c.gauss_xors) else {
                continue;
            };
            let x2 = x1 ^ s;
            if x1 == 0 || x2 == 0 || x1 > dmask || x2 > dmask {
                continue;
            }
            let (k1, k2) = (self.col_of_x[x1 as usize], self.col_of_x[x2 as usize]);
            if k1 == u32::MAX || k2 == u32::MAX {
                continue;
            }
            let (p1, p2) = (
                Some(self.points[k1 as usize]),
                Some(self.points[k2 as usize]),
            );
            if k1 == k2 {
                let dbl = cv.add(p1, p1, c);
                if dbl == Some(r) {
                    out.push(Rel {
                        terms: vec![(k1, 2)],
                    });
                } else if cv.neg(dbl) == Some(r) {
                    out.push(Rel {
                        terms: vec![(k1, -2)],
                    });
                }
                continue;
            }
            let sum = cv.add(p1, p2, c);
            let rel = if sum == Some(r) {
                Some((1, 1))
            } else if cv.neg(sum) == Some(r) {
                Some((-1, -1))
            } else {
                let diff = cv.add(p1, cv.neg(p2), c);
                if diff == Some(r) {
                    Some((1, -1))
                } else if cv.neg(diff) == Some(r) {
                    Some((-1, 1))
                } else {
                    None
                }
            };
            if let Some((e1, e2)) = rel {
                out.push(Rel {
                    terms: vec![(k1, e1), (k2, e2)],
                });
            }
        }
        out
    }
}

/// Union-find over base columns with `ℓ_v = σ_v ℓ_parent + c_v (mod r)`.
struct Graph {
    r: u64,
    parent: Vec<u32>,
    sigma: Vec<i8>,
    off: Vec<u64>,
    value: Vec<Option<u64>>, // known value of a root
    determined: usize,
    size: Vec<u32>,
    largest: usize,
}

impl Graph {
    fn new(k: usize, r: u64) -> Self {
        Self {
            r,
            parent: (0..k as u32).collect(),
            sigma: vec![1; k],
            off: vec![0; k],
            value: vec![None; k],
            determined: 0,
            size: vec![1; k],
            largest: 1,
        }
    }

    fn sgn(&self, s: i64, v: u64) -> u64 {
        if s >= 0 {
            v % self.r
        } else {
            (self.r - v % self.r) % self.r
        }
    }

    /// (root, σ, c) with ℓ_v = σ ℓ_root + c.
    fn find(&mut self, v: u32) -> (u32, i64, u64) {
        let p = self.parent[v as usize];
        if p == v {
            return (v, 1, 0);
        }
        let (root, sp, cp) = self.find(p);
        // ℓ_v = σ_v ℓ_p + c_v = σ_v (sp ℓ_root + cp) + c_v
        let sv = self.sigma[v as usize] as i64;
        let nc = (self.sgn(sv, cp) + self.off[v as usize]) % self.r;
        self.parent[v as usize] = root;
        self.sigma[v as usize] = (sv * sp) as i8;
        self.off[v as usize] = nc;
        (root, sv * sp, nc)
    }

    fn set_root(&mut self, root: u32, v: u64) {
        if self.value[root as usize].is_none() {
            self.value[root as usize] = Some(v % self.r);
            self.determined += self.size[root as usize] as usize;
        }
    }

    /// `Σ e_i ℓ_{v_i}` when it is determined: per component, the root's
    /// coefficient either cancels (balanced signs, as every relation is when
    /// the cofactor forces opposite signs) or the root's value is known.
    fn eval(&mut self, terms: &[(u32, i64)]) -> Option<u64> {
        let r = self.r;
        let mut groups: Vec<(u32, u64, u64)> = Vec::new(); // (root, coef, const)
        for &(v, e) in terms {
            let (root, s, c) = self.find(v);
            let coef = self.sgn(e * s, e.unsigned_abs());
            let cst = self.sgn(e, mulmod(e.unsigned_abs(), c, r));
            match groups.iter_mut().find(|g| g.0 == root) {
                Some(g) => {
                    g.1 = (g.1 + coef) % r;
                    g.2 = (g.2 + cst) % r;
                }
                None => groups.push((root, coef, cst)),
            }
        }
        let mut sum = 0u64;
        for (root, coef, cst) in groups {
            sum = (sum + cst) % r;
            if coef != 0 {
                sum = (sum + mulmod(coef, self.value[root as usize]?, r)) % r;
            }
        }
        Some(sum)
    }

    fn largest(&self) -> usize {
        self.largest
    }

    /// Add `Σ e_i ℓ_{v_i} = k`.
    fn add(&mut self, terms: &[(u32, i64)], k: u64) {
        let r = self.r;
        if terms.len() == 1 {
            let (v, e) = terms[0];
            let (root, s, c) = self.find(v);
            // e (s ℓ_root + c) = k
            let a = e * s;
            let rhs = (k + r - mulmod(self.sgn(e, e.unsigned_abs()), c, r)) % r;
            let val = mulmod(rhs, invmod(self.sgn(a, a.unsigned_abs()), r), r);
            self.set_root(root, val);
            return;
        }
        let (u, e1) = terms[0];
        let (v, e2) = terms[1];
        let (ru, su, cu) = self.find(u);
        let (rv, sv, cv) = self.find(v);
        let a = e1 * su; // coefficient of ℓ_ru
        let bco = e2 * sv; // coefficient of ℓ_rv
        let kp = (k + 2 * r
            - mulmod(self.sgn(e1, e1.unsigned_abs()), cu, r)
            - mulmod(self.sgn(e2, e2.unsigned_abs()), cv, r))
            % r;
        if ru == rv {
            // (a + b) ℓ_root = kp
            if a + bco != 0 {
                let val = mulmod(
                    kp,
                    invmod(self.sgn(a + bco, (a + bco).unsigned_abs()), r),
                    r,
                );
                self.set_root(ru, val);
            }
            return;
        }
        // ℓ_rv = (kp − a ℓ_ru) / b  (b = ±1)
        let (vu, vv) = (self.value[ru as usize], self.value[rv as usize]);
        let (keep, gone, s_new, c_new) = if self.size[ru as usize] >= self.size[rv as usize] {
            // express rv in terms of ru
            (ru, rv, -a * bco, self.sgn(bco, kp))
        } else {
            (rv, ru, -bco * a, self.sgn(a, kp))
        };
        self.parent[gone as usize] = keep;
        self.sigma[gone as usize] = s_new as i8;
        self.off[gone as usize] = c_new;
        self.size[keep as usize] += self.size[gone as usize];
        self.largest = self.largest.max(self.size[keep as usize] as usize);
        let (vk, vg) = if keep == ru { (vu, vv) } else { (vv, vu) };
        match (vk, vg) {
            (Some(_), None) => self.determined += self.size[gone as usize] as usize,
            (None, Some(x)) => {
                // ℓ_gone = s_new ℓ_keep + c_new  =>  ℓ_keep = s_new (x − c_new)
                let val = self.sgn(s_new, (x + r - c_new) % r);
                self.value[keep as usize] = Some(val);
                self.determined += (self.size[keep as usize] - self.size[gone as usize]) as usize;
            }
            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 7 {
        eprintln!("usage: ic_m2_linear <a> <n> <d|0> <targets> <seed> <out.json>");
        std::process::exit(2);
    }
    let a: u8 = args[1].parse().unwrap();
    let n: u32 = args[2].parse().unwrap();
    let mut d: usize = args[3].parse().unwrap();
    let targets: usize = args[4].parse().unwrap();
    let seed: u64 = args[5].parse().unwrap();
    if d == 0 {
        d = (n as usize + 1) / 3;
    }
    assert!(
        2 * d - 1 <= n as usize && 3 * d - 1 <= 63,
        "need 2d − 1 ≤ n and 3d − 1 ≤ 63"
    );

    let kc = KoblitzCurve::new(a, n).expect("Koblitz curve with a usable subgroup");
    let r = kc.subgroup_order.to_u64().expect("r fits u64");
    let h = kc.cofactor.to_u64().expect("cofactor fits u64");
    let f = Gf2::new(&kc.curve.irreducible);
    let irr = f.irr;
    let cv = Curve {
        f,
        n,
        a: a as u64,
        b: 1,
        irr,
    };
    let g: (u64, u64) = match RawPoint::from_binary(kc.generator()) {
        RawPoint::Affine { x, y } => (x, y),
        RawPoint::Infinity => panic!("generator at infinity"),
    };
    let lambda = kc.lambda.to_u64().unwrap_or(0);
    let mut scratch = Counts::default();
    assert_eq!(cv.mul(Some(g), r, &mut scratch), None, "[r]G = O");
    assert_eq!(
        cv.frob(Some(g)),
        cv.mul(Some(g), lambda, &mut scratch),
        "φ(G) = [λ]G"
    );

    let rho = StrongRho::new(&kc);
    let mut rows = Vec::new();
    for ti in 0..targets {
        let mut st = seed ^ (ti as u64).wrapping_mul(0x9e3779b97f4a7c15);
        let mut next = || {
            st = st.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = st;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^ (z >> 31)
        };
        let t_secret = next() % (r - 1) + 1;
        let target = cv.mul(Some(g), t_secret, &mut scratch).unwrap();

        // ---------------- IC (cold, one target) ----------------
        let mut c = Counts::default();
        let started = Instant::now();
        // Factor base.
        let mut col_of_x = vec![u32::MAX; 1usize << d];
        let mut points = Vec::new();
        for x in 1u64..(1u64 << d) {
            if let Some(p) = cv.lift_x(x) {
                col_of_x[x as usize] = points.len() as u32;
                points.push(p);
            }
        }
        let k = points.len();
        let mut z2 = Vec::with_capacity(2 * d);
        for j in 0..2 * d - 1 {
            let zj = if j < n as usize { 1u64 << j } else { 0 };
            z2.push(cv.f.sqr(zj));
        }
        let dec = Decomposer {
            d,
            z2,
            col_of_x,
            points,
        };
        let mut graph = Graph::new(k, r);
        // Columns whose projection [h]P is O have logarithm 0.
        for (i, &p) in dec.points.iter().enumerate() {
            if cv.mul(Some(p), h, &mut c).is_none() {
                graph.add(&[(i as u32, 1)], 0);
            }
        }
        let setup_s = started.elapsed().as_secs_f64();
        // Relations: anchors A = [a]G walked by known steps; every Frobenius
        // conjugate φ^j(A) is a trial with logarithm λ^j a.
        let steps: Vec<((u64, u64), u64)> = (0..16)
            .map(|_| {
                let w = next() % (r - 1) + 1;
                (cv.mul(Some(g), w, &mut c).unwrap(), w)
            })
            .collect();
        let rel_started = Instant::now();
        let mut anchor_log = next() % (r - 1) + 1;
        let mut anchor = cv.mul(Some(g), anchor_log, &mut c).unwrap();
        let need = 0.9_f64;
        let mut rels_used = 0u64;
        while (graph.determined.max(graph.largest()) as f64) < need * k as f64 {
            let mut p = Some(anchor);
            let mut lg = anchor_log;
            for _ in 0..n {
                for rel in dec.decompose(&cv, p.unwrap(), &mut c) {
                    // [h]A = Σ e_i [h]P_i  =>  h·log A = Σ e_i ℓ_i
                    graph.add(
                        &rel.terms.iter().map(|&(k, e)| (k, e)).collect::<Vec<_>>(),
                        mulmod(h % r, lg, r),
                    );
                    c.relations += 1;
                    rels_used += 1;
                }
                p = cv.frob(p);
                lg = mulmod(lg, lambda, r);
            }
            if std::env::var("M2_DEBUG").is_ok()
                && (c.trials < 60 || (c.trials / n as u64).is_multiple_of(20000))
            {
                eprintln!(
                    "  trials {} consistent {} splits {} relations {} determined {}/{} {:.2}s",
                    c.trials,
                    c.consistent,
                    c.split_attempts,
                    rels_used,
                    graph.determined,
                    k,
                    rel_started.elapsed().as_secs_f64()
                );
            }
            let (w, wl) = steps[(next() % 16) as usize];
            match cv.add(Some(anchor), Some(w), &mut c) {
                Some(q) => {
                    anchor = q;
                    anchor_log = (anchor_log + wl) % r;
                }
                None => {
                    anchor_log = next() % (r - 1) + 1;
                    anchor = cv.mul(Some(g), anchor_log, &mut c).unwrap();
                }
            }
        }
        let relations_s = rel_started.elapsed().as_secs_f64();
        let rel_counts = c;
        // Target: B = T + [a]G, conjugates λ^j (t + a); first decomposition
        // whose columns are all determined.
        let tgt_started = Instant::now();
        let mut a_log = next() % (r - 1) + 1;
        let mut base_pt = cv.add(Some(target), cv.mul(Some(g), a_log, &mut c), &mut c);
        let recovered: Option<u64>;
        'outer: loop {
            if let Some(bp) = base_pt {
                let mut p = Some(bp);
                let mut lam_j = 1u64;
                for _ in 0..n {
                    for rel in dec.decompose(&cv, p.unwrap(), &mut c) {
                        let val = graph.eval(&rel.terms);
                        if let Some(sum) = val {
                            // h λ^j (t + a) = sum
                            let ta = mulmod(sum, invmod(mulmod(h % r, lam_j, r), r), r);
                            recovered = Some((ta + r - a_log) % r);
                            break 'outer;
                        }
                    }
                    p = cv.frob(p);
                    lam_j = mulmod(lam_j, lambda, r);
                }
            }
            let (w, wl) = steps[(next() % 16) as usize];
            base_pt = cv.add(base_pt, Some(w), &mut c);
            a_log = (a_log + wl) % r;
        }
        let target_s = tgt_started.elapsed().as_secs_f64();
        let ic_s = started.elapsed().as_secs_f64();
        let rec = recovered.unwrap();
        let verified = cv.mul(Some(g), rec, &mut scratch) == Some(target);

        // ---------------- strong rho, same target ----------------
        let rho_started = Instant::now();
        let mut charges = StrongRhoCharges::default();
        let jumps = rho.jumps(seed ^ 0x6a75 ^ ti as u64, &mut charges);
        let mut rng = StdRng::seed_from_u64(seed ^ 0x7268 ^ ti as u64);
        let params = StrongRhoParams::default();
        let q = RawPoint::Affine {
            x: target.0,
            y: target.1,
        };
        let out = rho.solve(q, &jumps, &mut rng, &params, charges);
        let rho_s = rho_started.elapsed().as_secs_f64();
        let (rho_scalar, rho_steps, rho_adds) = match &out {
            Some(o) => (Some(o.scalar), o.walk_steps, o.charges.group_additions),
            None => (None, 0, 0),
        };
        rows.push(json!({
            "target": ti, "secret": t_secret.to_string(), "ic_recovered": rec.to_string(),
            "ic_verified": verified && rec == t_secret % r,
            "ic_s": ic_s, "ic_setup_s": setup_s, "ic_relations_s": relations_s, "ic_target_s": target_s,
            "ic_trials": c.trials, "ic_trials_relations_phase": rel_counts.trials,
            "ic_consistent": c.consistent, "ic_split_attempts": c.split_attempts,
            "ic_relations": rels_used, "ic_group_additions": c.group_additions,
            "ic_gauss_xors": c.gauss_xors, "ic_columns": k, "ic_determined": graph.determined,
            "rho_s": rho_s, "rho_recovered": rho_scalar.map(|v| v.to_string()),
            "rho_verified": rho_scalar == Some(t_secret % r),
            "rho_walk_steps": rho_steps, "rho_group_additions": rho_adds,
        }));
        eprintln!(
            "n={n} d={d} t={ti}: ic {:.3}s ({} trials, {} rels, K={k}) verified={} | rho {:.4}s",
            ic_s, c.trials, rels_used, verified, rho_s
        );
    }
    let out = json!({
        "schema": "ic-m2-linear/v1", "a": a, "n": n, "d": d, "seed": seed,
        "curve_id": format!("{:?}", kc.curve_id()), "r": r.to_string(), "cofactor": h.to_string(),
        "log2_r": (r as f64).log2(),
        "factor_base": format!("x in span{{1..z^{}}} (polynomial basis), both signs per x", d - 1),
        "rows": rows,
    });
    std::fs::write(&args[6], serde_json::to_string_pretty(&out).unwrap()).unwrap();
}
