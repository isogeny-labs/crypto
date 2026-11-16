//! Linear two-summand algebraic index calculus on Koblitz curves (the
//! decomposition of `ic_m2_linear.rs`, unchanged) with a single-target
//! cycle stop and optional torsion-randomized anchors.
//!
//! Anchors are `A = [a]G + [b]T` with the target T mixed in, so every
//! offset in the union-find is affine in the unknown t: ℓ_v = σ ℓ_root +
//! α + β t (mod r).  The first relation that closes an equation with no
//! free root (a sign-balanced cycle, a cycle in a component whose root is
//! already fixed, or an edge between two fixed components) gives
//! α + β t = 0 and so t.  A random graph has its first cycles near N/2
//! edges, against the ~1.3 N relations a 90% coverage stop consumes.
//!
//! With `torsion = 1` each walk step also adds a random point of the
//! cofactor part (`[r]Q`), which has ℓ = 0, so relations no longer have to
//! join points whose cofactor components cancel.  This is what the
//! large-cofactor rungs (e.g. a = 0, n = 37, h = 596) need.
//!
//! Usage: `ic_m2_cycle <a> <n> <d|0> <targets> <seed> <torsion 0|1> <out.json>`
//! Targets use the same derivation as `ic_m2_linear` (same seed, same
//! secrets), so rows pair with the frozen ladder.

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

/// `α + β t (mod r)`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Aff(u64, u64);

fn red(k: i64, r: u64) -> u64 {
    let ri = r as i128;
    (((k as i128) % ri + ri) % ri) as u64
}

impl Aff {
    fn add(self, o: Aff, r: u64) -> Aff {
        Aff((self.0 + o.0) % r, (self.1 + o.1) % r)
    }
    fn scale(self, k: i64, r: u64) -> Aff {
        let k = red(k, r);
        Aff(mulmod(self.0, k, r), mulmod(self.1, k, r))
    }
    fn mulu(self, k: u64, r: u64) -> Aff {
        Aff(mulmod(self.0, k, r), mulmod(self.1, k, r))
    }
}

/// Union-find over base columns with `ℓ_v = σ_v ℓ_parent + off_v`, every
/// offset and root value affine in t.
struct Graph {
    r: u64,
    parent: Vec<u32>,
    sigma: Vec<i8>,
    off: Vec<Aff>,
    value: Vec<Option<Aff>>,
    size: Vec<u32>,
    largest: usize,
    fixed_roots: usize,
}

impl Graph {
    fn new(k: usize, r: u64) -> Self {
        Self {
            r,
            parent: (0..k as u32).collect(),
            sigma: vec![1; k],
            off: vec![Aff(0, 0); k],
            value: vec![None; k],
            size: vec![1; k],
            largest: 1,
            fixed_roots: 0,
        }
    }

    /// (root, σ, c) with ℓ_v = σ ℓ_root + c.
    fn find(&mut self, v: u32) -> (u32, i64, Aff) {
        let p = self.parent[v as usize];
        if p == v {
            return (v, 1, Aff(0, 0));
        }
        let (root, sp, cp) = self.find(p);
        let sv = self.sigma[v as usize] as i64;
        let nc = cp.scale(sv, self.r).add(self.off[v as usize], self.r);
        self.parent[v as usize] = root;
        self.sigma[v as usize] = (sv * sp) as i8;
        self.off[v as usize] = nc;
        (root, sv * sp, nc)
    }

    /// Add `Σ e_i ℓ_{v_i} = rhs`.  Returns `Some(K)` when the relation
    /// leaves no free root, i.e. the equation `K = 0` in t.
    fn add(&mut self, terms: &[(u32, i64)], rhs: Aff) -> Option<Aff> {
        let r = self.r;
        let mut k = rhs;
        let mut groups: Vec<(u32, i64)> = Vec::with_capacity(2);
        for &(v, e) in terms {
            let (root, s, c) = self.find(v);
            k = k.add(c.scale(-e, r), r);
            match groups.iter_mut().find(|g| g.0 == root) {
                Some(g) => g.1 += e * s,
                None => groups.push((root, e * s)),
            }
        }
        let mut free: Vec<(u32, i64)> = Vec::with_capacity(2);
        for (root, co) in groups {
            if co == 0 {
                continue;
            }
            match self.value[root as usize] {
                Some(val) => k = k.add(val.scale(-co, r), r),
                None => free.push((root, co)),
            }
        }
        match free.len() {
            0 => Some(k),
            1 => {
                let (root, co) = free[0];
                self.value[root as usize] = Some(k.mulu(invmod(red(co, r), r), r));
                self.fixed_roots += 1;
                None
            }
            _ => {
                let (mut keep, mut ck) = free[0];
                let (mut gone, mut cg) = free[1];
                if self.size[keep as usize] < self.size[gone as usize] || ck.abs() != 1 {
                    std::mem::swap(&mut keep, &mut gone);
                    std::mem::swap(&mut ck, &mut cg);
                }
                assert!(
                    cg.abs() == 1 && ck.abs() == 1,
                    "two-root relation with |coef| ≠ 1"
                );
                // ℓ_gone = cg (K − ck ℓ_keep)
                self.parent[gone as usize] = keep;
                self.sigma[gone as usize] = (-ck * cg) as i8;
                self.off[gone as usize] = k.scale(cg, r);
                self.size[keep as usize] += self.size[gone as usize];
                self.largest = self.largest.max(self.size[keep as usize] as usize);
                None
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 8 {
        eprintln!("usage: ic_m2_cycle <a> <n> <d|0> <targets> <seed> <torsion 0|1> <out.json>");
        std::process::exit(2);
    }
    let a: u8 = args[1].parse().unwrap();
    let n: u32 = args[2].parse().unwrap();
    let mut d: usize = args[3].parse().unwrap();
    let targets: usize = args[4].parse().unwrap();
    let seed: u64 = args[5].parse().unwrap();
    let torsion = args[6] == "1";
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
    let hr = h % r;

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
        for (i, &p) in dec.points.iter().enumerate() {
            if cv.mul(Some(p), h, &mut c).is_none() {
                graph.add(&[(i as u32, 1)], Aff(0, 0));
            }
        }
        // A random point of the cofactor part ([r]Q, ℓ = 0).
        let torsion_point = |c: &mut Counts, next: &mut dyn FnMut() -> u64| -> Pt {
            if !torsion {
                return None;
            }
            loop {
                let x = next() & ((1u64 << n) - 1);
                if let Some(q) = cv.lift_x(x) {
                    return cv.mul(Some(q), r, c);
                }
            }
        };
        let setup_s = started.elapsed().as_secs_f64();
        // Anchors A = [a]G + [b]T (+ W), walked by 16 known steps; every
        // Frobenius conjugate φ^j(A) is a trial with logarithm λ^j (a + b t).
        let mut steps: Vec<(Pt, Aff)> = Vec::with_capacity(16);
        for _ in 0..16 {
            let (u, v) = (next() % (r - 1) + 1, next() % (r - 1) + 1);
            let s = cv.add(
                cv.mul(Some(g), u, &mut c),
                cv.mul(Some(target), v, &mut c),
                &mut c,
            );
            let w = torsion_point(&mut c, &mut next);
            steps.push((cv.add(s, w, &mut c), Aff(u, v)));
        }
        let fresh = |c: &mut Counts, next: &mut dyn FnMut() -> u64| -> (Pt, Aff) {
            let (u, v) = (next() % (r - 1) + 1, next() % (r - 1) + 1);
            let s = cv.add(cv.mul(Some(g), u, c), cv.mul(Some(target), v, c), c);
            let w = torsion_point(c, next);
            (cv.add(s, w, c), Aff(u, v))
        };
        let (mut anchor, mut alog) = fresh(&mut c, &mut next);
        let (mut equations, mut useless, mut inconsistent, mut bad_t) = (0u64, 0u64, 0u64, 0u64);
        let recovered: u64;
        'outer: loop {
            if let Some(ap) = anchor {
                let mut p = Some(ap);
                let mut lg = alog;
                for _ in 0..n {
                    for rel in dec.decompose(&cv, p.unwrap(), &mut c) {
                        c.relations += 1;
                        // [h]A = Σ e_i [h]P_i  =>  Σ e_i ℓ_i = h (a + b t)
                        if let Some(eq) = graph.add(&rel.terms, lg.mulu(hr, r)) {
                            equations += 1;
                            if eq.1 != 0 {
                                // α + β t = 0
                                let t = mulmod((r - eq.0) % r, invmod(eq.1, r), r);
                                if cv.mul(Some(g), t, &mut scratch) == Some(target) {
                                    recovered = t;
                                    break 'outer;
                                }
                                bad_t += 1;
                            } else if eq.0 != 0 {
                                inconsistent += 1;
                            } else {
                                useless += 1;
                            }
                        }
                    }
                    p = cv.frob(p);
                    lg = lg.mulu(lambda, r);
                }
            }
            let (w, wl) = steps[(next() % 16) as usize];
            match cv.add(anchor, w, &mut c) {
                Some(q) => {
                    anchor = Some(q);
                    alog = alog.add(wl, r);
                }
                None => {
                    let (fa, fl) = fresh(&mut c, &mut next);
                    anchor = fa;
                    alog = fl;
                }
            }
        }
        let ic_s = started.elapsed().as_secs_f64();
        let verified = cv.mul(Some(g), recovered, &mut scratch) == Some(target);

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
            "target": ti, "secret": t_secret.to_string(), "ic_recovered": recovered.to_string(),
            "ic_verified": verified && recovered == t_secret % r,
            "ic_s": ic_s, "ic_setup_s": setup_s,
            "ic_trials": c.trials, "ic_consistent": c.consistent, "ic_split_attempts": c.split_attempts,
            "ic_relations": c.relations, "ic_group_additions": c.group_additions,
            "ic_gauss_xors": c.gauss_xors, "ic_columns": k,
            "ic_largest_component": graph.largest, "ic_fixed_roots": graph.fixed_roots,
            "ic_equations": equations, "ic_useless_cycles": useless,
            "ic_inconsistent": inconsistent, "ic_bad_t": bad_t,
            "rho_s": rho_s, "rho_recovered": rho_scalar.map(|v| v.to_string()),
            "rho_verified": rho_scalar == Some(t_secret % r),
            "rho_walk_steps": rho_steps, "rho_group_additions": rho_adds,
        }));
        eprintln!(
            "n={n} d={d} t={ti}: ic {:.3}s ({} trials, {} rels, K={k}, eqs {equations}, bad {bad_t}, incons {inconsistent}) verified={} | rho {:.4}s",
            ic_s, c.trials, c.relations, verified, rho_s
        );
    }
    let out = json!({
        "schema": "ic-m2-cycle/v1", "a": a, "n": n, "d": d, "seed": seed, "torsion": torsion,
        "curve_id": format!("{:?}", kc.curve_id()), "r": r.to_string(), "cofactor": h.to_string(),
        "log2_r": (r as f64).log2(),
        "factor_base": format!("x in span{{1..z^{}}} (polynomial basis), both signs per x", d - 1),
        "stop": "first relation closing an equation in t (anchors [a]G + [b]T)",
        "rows": rows,
    });
    std::fs::write(&args[7], serde_json::to_string_pretty(&out).unwrap()).unwrap();
}
