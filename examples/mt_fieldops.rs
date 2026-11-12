//! Multi-target ECDLP on a small prime-order curve, counted in FIELD operations (experimental).
//!
//! Four arms share one counted u64 field layer (p < 2^31): every inversion (I), multiplication (M)
//! and squaring (S) is counted, plus hash inserts/lookups (H) and solver edge operations (L).
//! Additions/subtractions in F_p are not counted.  Costs are converted to group-addition
//! equivalents (GAE = one affine addition = I + 2M + S) in the analysis, at several I/M ratios.
//!
//! The lever under test is "P +- Q with one inversion": x(P+Q) and x(P-Q) share the denominator
//! x_Q - x_P, so both x-coordinates cost I + 2M + 2S (one affine addition costs I + 2M + S).
//! Arms:
//!   ic4   index calculus: factor base = m smallest-x points (logs unknown), x-only pair-sum table
//!         {x(F_i +- F_j)} built with shared inversions, factor-base logs from relations
//!         V_j +- W_k = +-(F_i +- F_j), then per target paired trials x(Q_t +- W_k) against the table.
//!         W_k (a shared arithmetic progression of known multiples) is computed once for all targets.
//!   rho   anchored batch rho as in `ic_batch_rho` (r-adding walks, database of visited x, x-only lookups).
//!   rho2  rho with the same lever: every walk step R -> R + A also yields R - A in full (+2M+S, y kept
//!         so a hit needs no recomputation), stored in phase 1 and looked up in phase 2.
//!   rho2x rho2 with an x-only sibling (I + 3M + 2S per step), the sibling recomputed in full on a hit (counted).
//!   rhoseq anchored rho whose phase-1 database is the sequential set {G, 2G, ..., NG} (no walk waste).
//!   bsgs2 multi-target BSGS with a sum-set baby table {x(jU +- iG)} (shared inversions) and paired giant
//!         trials x(Q_t +- kS G), giants shared by all targets.
//! On a hit, the sign is resolved by recomputing the two points in full (counted); every recovered log
//! is verified by a counted double-and-add scalar multiplication.
//!
//! usage: mt_fieldops <arm> <instance.json> <targets> <seed> <param>
//!   ic4: param = mu (m = ceil(mu (nT/2)^(1/4))), optional 6th arg g (relations per fb element, default 1.5)
//!   rho, rho2, rho2x, rhoseq: param = kappa (database = kappa sqrt(nT) x-classes)
//!   bsgs2: param = mf (baby x-classes = mf sqrt(nT)/2)
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cell::Cell;
use std::collections::HashMap;
use std::time::Instant;

type Pt = Option<(u64, u64)>;

struct F {
    p: u64,
    a: u64,
    b: u64,
    ci: Cell<u64>,
    cm: Cell<u64>,
    cs: Cell<u64>,
    ch: Cell<u64>,
    cl: Cell<u64>,
}

impl F {
    fn add(&self, x: u64, y: u64) -> u64 {
        let s = x + y;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    fn sub(&self, x: u64, y: u64) -> u64 {
        if x >= y {
            x - y
        } else {
            x + self.p - y
        }
    }
    fn neg(&self, x: u64) -> u64 {
        if x == 0 {
            0
        } else {
            self.p - x
        }
    }
    fn mul(&self, x: u64, y: u64) -> u64 {
        self.cm.set(self.cm.get() + 1);
        ((x as u128 * y as u128) % self.p as u128) as u64
    }
    fn sqr(&self, x: u64) -> u64 {
        self.cs.set(self.cs.get() + 1);
        ((x as u128 * x as u128) % self.p as u128) as u64
    }
    fn inv(&self, x: u64) -> u64 {
        self.ci.set(self.ci.get() + 1);
        let (mut r0, mut r1) = (self.p as i128, x as i128);
        let (mut t0, mut t1) = (0i128, 1i128);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        assert_eq!(r0, 1, "non-invertible");
        t0.rem_euclid(self.p as i128) as u64
    }
    fn hash(&self) {
        self.ch.set(self.ch.get() + 1);
    }
    fn lin(&self, k: u64) {
        self.cl.set(self.cl.get() + k);
    }
    fn small(&self, k: u64) -> u64 {
        k % self.p
    }
    /// Full affine addition (handles doubling and inverses).
    fn padd(&self, p: Pt, q: Pt) -> Pt {
        let (x1, y1) = match p {
            None => return q,
            Some(v) => v,
        };
        let (x2, y2) = match q {
            None => return p,
            Some(v) => v,
        };
        let lam = if x1 == x2 {
            if self.add(y1, y2) == 0 {
                return None;
            }
            // doubling: (3x^2 + a) / (2y)
            let xx = self.sqr(x1);
            let num = self.add(self.add(self.add(xx, xx), xx), self.a);
            let d = self.inv(self.add(y1, y1));
            self.mul(num, d)
        } else {
            let d = self.inv(self.sub(x2, x1));
            self.mul(self.sub(y2, y1), d)
        };
        let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
        let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
        Some((x3, y3))
    }
    fn pneg(&self, p: Pt) -> Pt {
        p.map(|(x, y)| (x, self.neg(y)))
    }
    /// x(P+Q), x(P-Q) with one shared inversion (I + 2M + 2S).  None if x_P == x_Q or a point is O.
    fn xpair(&self, p: Pt, q: Pt) -> Option<(u64, u64)> {
        let ((x1, y1), (x2, y2)) = (p?, q?);
        if x1 == x2 {
            return None;
        }
        let d = self.inv(self.sub(x2, x1));
        let s = self.add(x1, x2);
        let lp = self.mul(self.sub(y2, y1), d);
        let lm = self.mul(self.neg(self.add(y2, y1)), d);
        Some((self.sub(self.sqr(lp), s), self.sub(self.sqr(lm), s)))
    }
    /// P + Q in full and the sibling P - Q in full, sharing the inversion: (I + 2M + S) + (2M + S).
    fn padd_with_sibling(&self, p: Pt, q: Pt) -> (Pt, Pt) {
        if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
            if x1 != x2 {
                let d = self.inv(self.sub(x2, x1));
                let lam = self.mul(self.sub(y2, y1), d);
                let x3 = self.sub(self.sub(self.sqr(lam), x1), x2);
                let y3 = self.sub(self.mul(lam, self.sub(x1, x3)), y1);
                // P - Q = P + (-Q): slope (-y2 - y1)/(x2 - x1)
                let lm = self.mul(self.neg(self.add(y2, y1)), d);
                let xm = self.sub(self.sub(self.sqr(lm), x1), x2);
                let ym = self.sub(self.mul(lm, self.sub(x1, xm)), y1);
                return (Some((x3, y3)), Some((xm, ym)));
            }
        }
        (self.padd(p, q), None)
    }
    fn smul(&self, p: Pt, k: u64) -> Pt {
        let mut r: Pt = None;
        let mut base = p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.padd(r, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.padd(base, base);
            }
        }
        r
    }
    fn pow(&self, mut b: u64, mut e: u64) -> u64 {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            e >>= 1;
            if e > 0 {
                b = self.sqr(b);
            }
        }
        r
    }
    /// Counted Tonelli-Shanks; None for a non-residue.
    fn sqrt(&self, v: u64) -> Option<u64> {
        if v == 0 {
            return Some(0);
        }
        let p = self.p;
        if self.pow(v, (p - 1) / 2) != 1 {
            return None;
        }
        let (mut q, mut s) = (p - 1, 0u32);
        while q % 2 == 0 {
            q /= 2;
            s += 1;
        }
        let mut z = 2u64;
        while self.pow(z, (p - 1) / 2) != p - 1 {
            z += 1;
        }
        let mut c = self.pow(z, q);
        let mut x = self.pow(v, q.div_ceil(2));
        let mut t = self.pow(v, q);
        let mut m = s;
        while t != 1 {
            let mut i = 0u32;
            let mut tt = t;
            while tt != 1 {
                tt = self.sqr(tt);
                i += 1;
            }
            let mut bb = c;
            for _ in 0..(m - i - 1) {
                bb = self.sqr(bb);
            }
            x = self.mul(x, bb);
            c = self.sqr(bb);
            t = self.mul(t, c);
            m = i;
        }
        Some(x)
    }
}

fn md(a: i128, n: u64) -> u64 {
    a.rem_euclid(n as i128) as u64
}

/// x_t from a hit "Q + s_w * W = eps * D" where log W = w and log D = dl, eps unknown; recompute
/// both points to resolve eps (counted), then return the log.
fn resolve(f: &F, q: Pt, w_pt: Pt, s_w: i64, w: u64, d_pt: Pt, dl: u64, n: u64) -> u64 {
    let wp = if s_w > 0 { w_pt } else { f.pneg(w_pt) };
    let r = f.padd(q, wp);
    let eps: i128 = if r == d_pt { 1 } else { -1 };
    md(eps * dl as i128 - s_w as i128 * w as i128, n)
}

struct Out {
    ok: usize,
    extra: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arm = args[1].as_str();
    let d: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&args[2]).unwrap()).unwrap();
    let targets: usize = args[3].parse().unwrap();
    let seed: u64 = args[4].parse().unwrap();
    let param: f64 = args[5].parse().unwrap();
    let num = |v: &serde_json::Value| -> u64 { v.to_string().trim_matches('"').parse().unwrap() };
    let p = num(&d["field"]["modulus"]);
    assert!(p < (1u64 << 60)); // bank sums of three logs must fit in u64
    let f = F {
        p,
        a: num(&d["a"]),
        b: num(&d["b"]),
        ci: Cell::new(0),
        cm: Cell::new(0),
        cs: Cell::new(0),
        ch: Cell::new(0),
        cl: Cell::new(0),
    };
    let n = num(&d["subgroup_order"]);
    let g: Pt = Some((num(&d["generator"]["x"]), num(&d["generator"]["y"])));
    // Planted targets (uncounted): same planting stream for every arm.
    let mut plant = StdRng::seed_from_u64(seed ^ 0x9e37_79b9_7f4a_7c15);
    let xs: Vec<u64> = (0..targets).map(|_| plant.gen_range(1..n)).collect();
    let qs: Vec<Pt> = {
        // computed with an uncounted copy of the field
        let f0 = F {
            p,
            a: f.a,
            b: f.b,
            ci: Cell::new(0),
            cm: Cell::new(0),
            cs: Cell::new(0),
            ch: Cell::new(0),
            cl: Cell::new(0),
        };
        xs.iter().map(|&x| f0.smul(g, x)).collect()
    };
    let mut rng = StdRng::seed_from_u64(seed.wrapping_mul(0x2545_f491_4f6c_dd1d) ^ 0xabcdef);
    let t0 = Instant::now();
    let mut found: Vec<u64> = vec![0; targets];
    let mut snap: Vec<(String, [u64; 5])> = Vec::new();
    let take = |f: &F| [f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get()];

    let out: Out = match arm {
        "ic4" => {
            let mu = param;
            let gfac: f64 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(1.5);
            let m = ((mu * ((n as f64) * targets as f64 / 2.0).powf(0.25)).ceil() as usize).max(4);
            // Factor base: smallest x with x^3 + ax + b a nonzero square (counted).
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // x-only pair-sum table: key x -> (i, j, kind) kind 0: F_i+F_j, 1: F_i-F_j, 2: 2F_i
            let mut table: HashMap<u64, (u32, u32, u8)> = HashMap::with_capacity(m * m + 8);
            for i in 0..m {
                let pi = Some(fb[i]);
                if let Some((x2, _)) = f.padd(pi, pi) {
                    f.hash();
                    table.entry(x2).or_insert((i as u32, i as u32, 2));
                }
                for j in (i + 1)..m {
                    if let Some((xp, xm)) = f.xpair(pi, Some(fb[j])) {
                        f.hash();
                        table.entry(xp).or_insert((i as u32, j as u32, 0));
                        f.hash();
                        table.entry(xm).or_insert((i as u32, j as u32, 1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u32, u8)| -> Pt {
                let (i, j, k) = e;
                let pi = Some(fb[i as usize]);
                match k {
                    2 => f.padd(pi, pi),
                    0 => f.padd(pi, Some(fb[j as usize])),
                    _ => f.padd(pi, f.pneg(Some(fb[j as usize]))),
                }
            };
            // Shared progression W_k = W_0 + k D (known logs), extended on demand.
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, k: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= k {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            // Phase 1: relations V_j +- W_k = eps (F_a +- F_b) on known points.
            // Relation: eps*(s_a y_a + s_b y_b) = v_j +- w_k.  Stored as (a, s_a, b, s_b, rhs) after eps fix.
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            let need = (gfac * m as f64).ceil() as usize;
            let mut rels: Vec<(u32, i8, u32, i8, u64)> = Vec::with_capacity(need);
            let mut seen_pairs: HashMap<(u32, u32, u8), ()> = HashMap::new();
            let mut p1_trials = 0u64;
            'outer: loop {
                for k in 0..kw {
                    p1_trials += 2;
                    if let Some((xp, xm)) = f.xpair(v_pt, w_pts[k]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                f.hash();
                                if seen_pairs.insert(e, ()).is_some() {
                                    continue;
                                }
                                // V + sw W = eps * D
                                let wp = if sw > 0 { w_pts[k] } else { f.pneg(w_pts[k]) };
                                let r = f.padd(v_pt, wp);
                                let dpt2 = entry_pt(&f, e);
                                let eps: i8 = if r == dpt2 { 1 } else { -1 };
                                let rhs = md(v_log as i128 + sw as i128 * w_log[k] as i128, n);
                                let (i, j, kind) = e;
                                let (sa, sb): (i8, i8) = match kind {
                                    2 => (2 * eps, 0),
                                    0 => (eps, eps),
                                    _ => (eps, -eps),
                                };
                                rels.push((i, sa, j, sb, rhs));
                                if rels.len() >= need {
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
                v_pt = f.padd(v_pt, e_pt);
                v_log = (v_log + e_log) % n;
            }
            snap.push(("relations".into(), take(&f)));
            // Solve the signed graph: y_i = c_i * r + d_i on each component (c_i = +-1), fix r by an
            // odd cycle / doubled-edge / self relation.
            let inv_mod = |a: u64| -> u64 {
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let mut adj: Vec<Vec<usize>> = vec![Vec::new(); m];
            for (ri, r) in rels.iter().enumerate() {
                adj[r.0 as usize].push(ri);
                if r.3 != 0 && r.2 != r.0 {
                    adj[r.2 as usize].push(ri);
                }
            }
            let mut ylog: Vec<Option<u64>> = vec![None; m];
            let mut comp: Vec<i64> = vec![-1; m];
            let mut cd: Vec<(i64, u64)> = vec![(0, 0); m];
            for root in 0..m {
                if comp[root] >= 0 || adj[root].is_empty() {
                    continue;
                }
                comp[root] = root as i64;
                cd[root] = (1, 0);
                let mut order = vec![root];
                let mut qi = 0;
                let mut fix: Option<u64> = None;
                while qi < order.len() {
                    let u = order[qi];
                    qi += 1;
                    for &ri in &adj[u] {
                        f.lin(1);
                        let (a, sa, b, sb, rhs) = rels[ri];
                        let (a, b) = (a as usize, b as usize);
                        // sa*y_a + sb*y_b = rhs
                        if sb == 0 || a == b {
                            // (sa+sb) y_a = rhs
                            let coef = sa as i64 + if a == b { sb as i64 } else { 0 };
                            let (c, dv) = cd[a];
                            let k = coef * c;
                            if fix.is_none() && k != 0 {
                                let rest = md(rhs as i128 - coef as i128 * dv as i128, n);
                                fix = Some(
                                    (rest as u128 * inv_mod(md(k as i128, n)) as u128 % n as u128)
                                        as u64,
                                );
                            }
                            continue;
                        }
                        let (o, so, su) = if a == u { (b, sb, sa) } else { (a, sa, sb) };
                        let (cu, du) = cd[u];
                        // su*y_u + so*y_o = rhs -> y_o = (rhs - su*y_u)/so, so = +-1
                        let co = -(su as i64) * cu * so as i64;
                        let dov = md((rhs as i128 - su as i128 * du as i128) * so as i128, n);
                        if comp[o] < 0 {
                            comp[o] = root as i64;
                            cd[o] = (co, dov);
                            order.push(o);
                        } else if fix.is_none() {
                            let (c2, d2) = cd[o];
                            let k = c2 - co;
                            if k != 0 {
                                let rest = md(dov as i128 - d2 as i128, n);
                                fix = Some(
                                    (rest as u128 * inv_mod(md(k as i128, n)) as u128 % n as u128)
                                        as u64,
                                );
                            }
                        }
                    }
                }
                if let Some(r) = fix {
                    for &u in &order {
                        let (c, dv) = cd[u];
                        ylog[u] = Some(md(c as i128 * r as i128 + dv as i128, n));
                    }
                }
            }
            let determined = ylog.iter().filter(|v| v.is_some()).count();
            snap.push(("solve".into(), take(&f)));
            // Phase 2: paired trials x(Q_t +- W_k).
            let mut trials = 0u64;
            let mut wasted_hits = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut k = 0usize;
                let v = 'tgt: loop {
                    ensure_w(&f, k, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[k].unwrap().0 {
                        break 'tgt if q == w_pts[k] {
                            w_log[k]
                        } else {
                            (n - w_log[k]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[k]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let (i, j, kind) = e;
                                let yl = match kind {
                                    2 => ylog[i as usize].map(|y| md(2 * y as i128, n)),
                                    0 => ylog[i as usize]
                                        .zip(ylog[j as usize])
                                        .map(|(a, b)| md(a as i128 + b as i128, n)),
                                    _ => ylog[i as usize]
                                        .zip(ylog[j as usize])
                                        .map(|(a, b)| md(a as i128 - b as i128, n)),
                                };
                                match yl {
                                    Some(dl) => {
                                        let dpt2 = entry_pt(&f, e);
                                        break 'tgt resolve(
                                            &f, q, w_pts[k], sw, w_log[k], dpt2, dl, n,
                                        );
                                    }
                                    None => wasted_hits += 1,
                                }
                            }
                        }
                    }
                    k += 1;
                    if k > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"m\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"determined\":{},\"trials\":{},\"wasted_hits\":{},\"w_len\":{}",
                    m, table.len(), rels.len(), p1_trials, determined, trials, wasted_hits, w_pts.len()
                ),
            }
        }
        "ic5" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity(kstar as usize + 16);
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::new();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "ic5ks" => {
            // k-term index calculus: table of x(F_{i1} + s2 F_{i2} + ... + s_k F_{ik}), i1 < ... < ik,
            // built as x(P +- F_l) from full (k-1)-term points P (shared inversion per pair);
            // factor-base logs by incremental Gaussian elimination mod n (every mod-n product counted
            // as M, every mod-n inverse as I); targets by paired trials x(Q +- W_k) as in ic4.
            let mu = param;
            let k: usize = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(3);
            let gfac: f64 = args.get(7).map(|s| s.parse().unwrap()).unwrap_or(1.2);
            assert!((2..=4).contains(&k));
            let kstar = mu * mu * ((n as f64) * targets as f64 / 2.0).sqrt();
            let classes = |m: usize| -> f64 {
                let mut c = 1.0;
                for i in 0..k {
                    c *= (m - i) as f64 / (i + 1) as f64;
                }
                c * (1u64 << (k - 1)) as f64
            };
            let mut m = k + 1;
            while classes(m) < kstar {
                m += 1;
            }
            let mut fb: Vec<(u64, u64)> = Vec::with_capacity(m);
            let mut x = 0u64;
            while fb.len() < m {
                x += 1;
                let xx = f.sqr(x);
                let rhs = f.add(f.add(f.mul(xx, x), f.mul(f.a, x)), f.b);
                if rhs == 0 {
                    continue;
                }
                if let Some(y) = f.sqrt(rhs) {
                    fb.push((x, y));
                }
            }
            snap.push(("fb".into(), take(&f)));
            // (k-1)-term full points: (point, indices, signs); first sign +1.
            let mut combos: Vec<(Pt, Vec<u16>, Vec<i8>)> = (0..m)
                .map(|i| (Some(fb[i]), vec![i as u16], vec![1i8]))
                .collect();
            for _ in 2..k {
                let mut next = Vec::new();
                for (p, idx, sg) in &combos {
                    let last = *idx.last().unwrap() as usize;
                    for l in (last + 1)..m {
                        let (sum, dif) = f.padd_with_sibling(*p, Some(fb[l]));
                        for (pt, s) in [(sum, 1i8), (dif, -1i8)] {
                            if pt.is_some() {
                                let mut i2 = idx.clone();
                                i2.push(l as u16);
                                let mut s2 = sg.clone();
                                s2.push(s);
                                next.push((pt, i2, s2));
                            }
                        }
                    }
                }
                combos = next;
            }
            snap.push(("combos".into(), take(&f)));
            // table: x -> (combo id, l, sign of F_l)
            let mut table: HashMap<u64, (u32, u16, i8)> =
                HashMap::with_capacity(kstar as usize + 16);
            for (ci_, (p, idx, _)) in combos.iter().enumerate() {
                let last = *idx.last().unwrap() as usize;
                for l in (last + 1)..m {
                    if let Some((xp, xm)) = f.xpair(*p, Some(fb[l])) {
                        f.hash();
                        table.entry(xp).or_insert((ci_ as u32, l as u16, 1));
                        f.hash();
                        table.entry(xm).or_insert((ci_ as u32, l as u16, -1));
                    }
                }
            }
            snap.push(("table".into(), take(&f)));
            let entry_pt = |f: &F, e: (u32, u16, i8)| -> Pt {
                let fl = Some(fb[e.1 as usize]);
                f.padd(
                    combos[e.0 as usize].0,
                    if e.2 > 0 { fl } else { f.pneg(fl) },
                )
            };
            // coefficient vector of an entry: (index, sign) pairs
            let entry_terms = |e: (u32, u16, i8)| -> Vec<(usize, i64)> {
                let (_, idx, sg) = &combos[e.0 as usize];
                let mut v: Vec<(usize, i64)> = idx
                    .iter()
                    .zip(sg)
                    .map(|(&i, &s)| (i as usize, s as i64))
                    .collect();
                v.push((e.1 as usize, e.2 as i64));
                v
            };
            // counted arithmetic mod n
            let mulm = |a: u64, b: u64| -> u64 {
                f.cm.set(f.cm.get() + 1);
                ((a as u128 * b as u128) % n as u128) as u64
            };
            let invm = |a: u64| -> u64 {
                f.ci.set(f.ci.get() + 1);
                let (mut r0, mut r1) = (n as i128, a as i128);
                let (mut t0, mut t1) = (0i128, 1i128);
                while r1 != 0 {
                    let q = r0 / r1;
                    (r0, r1) = (r1, r0 - q * r1);
                    (t0, t1) = (t1, t0 - q * t1);
                }
                t0.rem_euclid(n as i128) as u64
            };
            let w0 = rng.gen_range(1..n);
            let dd = rng.gen_range(1..n);
            let dpt = f.smul(g, dd);
            let mut w_pts: Vec<Pt> = vec![f.smul(g, w0)];
            let mut w_log: Vec<u64> = vec![w0];
            let ensure_w = |f: &F, kk: usize, w_pts: &mut Vec<Pt>, w_log: &mut Vec<u64>| {
                while w_pts.len() <= kk {
                    let last = *w_pts.last().unwrap();
                    w_pts.push(f.padd(last, dpt));
                    w_log.push((w_log.last().unwrap() + dd) % n);
                }
            };
            let kw = 64usize;
            ensure_w(&f, kw, &mut w_pts, &mut w_log);
            let e_log = rng.gen_range(1..n);
            let e_pt = f.smul(g, e_log);
            let v0 = rng.gen_range(1..n);
            let mut v_pt = f.smul(g, v0);
            let mut v_log = v0;
            // incremental echelon form: pivots[col] = Some(row) with row[col] = 1 (row has m+1 entries, last = rhs)
            let mut pivots: Vec<Option<Vec<u64>>> = vec![None; m];
            let mut rank = 0usize;
            let mut seen: HashMap<(u32, u16, i8), ()> = HashMap::new();
            let mut nrel = 0usize;
            let budget_rel = (gfac * m as f64).ceil() as usize;
            let mut p1_trials = 0u64;
            let mut kk = 0usize;
            while rank < m {
                if kk == kw {
                    kk = 0;
                    v_pt = f.padd(v_pt, e_pt);
                    v_log = (v_log + e_log) % n;
                }
                p1_trials += 2;
                if let Some((xp, xm)) = f.xpair(v_pt, w_pts[kk]) {
                    for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                        f.hash();
                        if let Some(&e) = table.get(&xx) {
                            f.hash();
                            if seen.insert(e, ()).is_some() {
                                continue;
                            }
                            let wp = if sw > 0 { w_pts[kk] } else { f.pneg(w_pts[kk]) };
                            let r = f.padd(v_pt, wp);
                            let eps: i64 = if r == entry_pt(&f, e) { 1 } else { -1 };
                            let mut row = vec![0u64; m + 1];
                            for (i, s) in entry_terms(e) {
                                row[i] = md((row[i] as i128) + (eps * s) as i128, n);
                            }
                            row[m] = md(v_log as i128 + sw as i128 * w_log[kk] as i128, n);
                            nrel += 1;
                            // reduce against pivots
                            for c in 0..m {
                                if row[c] == 0 {
                                    continue;
                                }
                                if let Some(pr) = &pivots[c] {
                                    let fct = row[c];
                                    for j in c..=m {
                                        if pr[j] != 0 {
                                            row[j] =
                                                md(row[j] as i128 - mulm(fct, pr[j]) as i128, n);
                                        }
                                    }
                                } else {
                                    let iv = invm(row[c]);
                                    for j in c..=m {
                                        if row[j] != 0 {
                                            row[j] = mulm(row[j], iv);
                                        }
                                    }
                                    pivots[c] = Some(row.clone());
                                    rank += 1;
                                    break;
                                }
                            }
                        }
                    }
                }
                kk += 1;
                if nrel > 50 * budget_rel + 1000 {
                    panic!("relation budget");
                }
            }
            // back substitution
            let mut ylog: Vec<u64> = vec![0; m];
            for c in (0..m).rev() {
                let pr = pivots[c].as_ref().unwrap();
                let mut v = pr[m] as i128;
                for j in (c + 1)..m {
                    if pr[j] != 0 {
                        v -= mulm(pr[j], ylog[j]) as i128;
                    }
                }
                ylog[c] = md(v, n);
            }
            snap.push(("relations".into(), take(&f)));
            let mut trials = 0u64;
            let mut ret: HashMap<u64, (u32, u32, i8)> = HashMap::new();
            for t in 0..targets {
                let q = qs[t];
                let mut kq = 0usize;
                let mut trail: Vec<(u64, u32, i8)> = Vec::new();
                let v = 'tgt: loop {
                    ensure_w(&f, kq, &mut w_pts, &mut w_log);
                    trials += 2;
                    if q.unwrap().0 == w_pts[kq].unwrap().0 {
                        break 'tgt if q == w_pts[kq] {
                            w_log[kq]
                        } else {
                            (n - w_log[kq]) % n
                        };
                    }
                    if let Some((xp, xm)) = f.xpair(q, w_pts[kq]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            trail.push((xx, kq as u32, sw as i8));
                            if let Some(&(t2, k2, s2)) = ret.get(&xx) {
                                // Q + sw W_kq = eps (Q_t2 + s2 W_k2), Q_t2 solved: recompute the stored point (counted)
                                let wp2 = if s2 > 0 {
                                    w_pts[k2 as usize]
                                } else {
                                    f.pneg(w_pts[k2 as usize])
                                };
                                let dpt2 = f.padd(qs[t2 as usize], wp2);
                                let dl = md(
                                    found[t2 as usize] as i128
                                        + s2 as i128 * w_log[k2 as usize] as i128,
                                    n,
                                );
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                            if let Some(&e) = table.get(&xx) {
                                let dl = md(
                                    entry_terms(e)
                                        .iter()
                                        .map(|&(i, s)| s as i128 * ylog[i] as i128)
                                        .sum(),
                                    n,
                                );
                                let dpt2 = entry_pt(&f, e);
                                break 'tgt resolve(&f, q, w_pts[kq], sw, w_log[kq], dpt2, dl, n);
                            }
                        }
                    }
                    kq += 1;
                    if kq > 50_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (xx, kk2, s2) in trail {
                    f.hash();
                    ret.entry(xx).or_insert((t as u32, kk2, s2));
                }
            }
            snap.push(("targets".into(), take(&f)));
            let _ = budget_rel;
            Out {
                ok: 0,
                extra: format!(
                    "\"k\":{},\"m\":{},\"combos\":{},\"table\":{},\"relations\":{},\"p1_trials\":{},\"trials\":{},\"w_len\":{}",
                    k, m, combos.len(), table.len(), nrel, p1_trials, trials, w_pts.len()
                ),
            }
        }
        "rho" | "rho2" => {
            let sib = arm == "rho2";
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            // db: x -> (log, y or u64::MAX for a sibling without y)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64)> = HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = if sib {
                    f.padd_with_sibling(prev, arms[h].0)
                } else {
                    (f.padd(prev, arms[h].0), None)
                };
                cur = nx;
                if let Some((xm, ym)) = sx {
                    f.hash();
                    db.entry(xm).or_insert(((ca + n - arms[h].1) % n, ym));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y));
                }
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let _ = prev;
                    let (nx, sx) = if sib {
                        f.padd_with_sibling(prev, arms[h].0)
                    } else {
                        (f.padd(prev, arms[h].0), None)
                    };
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    // main point: cur = ca G + Q
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&x) {
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some((xm, ym)) = sx {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&xm) {
                            // sibling: prev - A = (pa - a_A) G + Q, known in full
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rho2x" => {
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64, (u64, u64, usize))> =
                HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.entry(xm)
                        .or_insert(((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY {
                    return y2;
                }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0))
                    .unwrap()
                    .1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&x) {
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some(xm) = sx {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&xm) {
                            let ym = f.padd(prev, f.pneg(arms[h].0)).unwrap().1; // target sibling in full (counted)
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rho2xks" => {
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            const NOY: u64 = u64::MAX;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            // full P + A and x(P - A) with one inversion: I + 3M + 2S
            let step_x = |f: &F, p: Pt, q: Pt| -> (Pt, Option<u64>) {
                if let (Some((x1, y1)), Some((x2, y2))) = (p, q) {
                    if x1 != x2 {
                        let d = f.inv(f.sub(x2, x1));
                        let lam = f.mul(f.sub(y2, y1), d);
                        let x3 = f.sub(f.sub(f.sqr(lam), x1), x2);
                        let y3 = f.sub(f.mul(lam, f.sub(x1, x3)), y1);
                        let lm = f.mul(f.neg(f.add(y2, y1)), d);
                        let xm = f.sub(f.sub(f.sqr(lm), x1), x2);
                        return (Some((x3, y3)), Some(xm));
                    }
                }
                (f.padd(p, q), None)
            };
            // db: x -> (log, y or NOY, (prev_x, prev_y, arm)) ; a NOY entry is prev - arms[arm], recomputed on a hit (counted)
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64, (u64, u64, usize))> =
                HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = None;
            let mut ca = 0u64;
            let mut st = RESTART;
            let mut db_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while db.len() < db_target {
                if st >= RESTART || cur.is_none() {
                    // chained starts s <- s + B_k: one addition per restart and unboundedly many distinct
                    // starts (a fixed-bank triple sum capped the walks at C(66, 3) and could not fill large dbs)
                    let k1 = rng.gen_range(0..BANK);
                    sp = f.padd(sp, bank[k1].0);
                    sa = (sa + bank[k1].1) % n;
                    cur = sp;
                    ca = sa;
                    st = 0;
                }
                let h = hidx(cur);
                let prev = cur;
                let (nx, sx) = step_x(&f, prev, arms[h].0);
                cur = nx;
                if let Some(xm) = sx {
                    let (px, py) = prev.unwrap();
                    f.hash();
                    db.entry(xm)
                        .or_insert(((ca + n - arms[h].1) % n, NOY, (px, py, h)));
                }
                ca = (ca + arms[h].1) % n;
                st += 1;
                db_steps += 1;
                if let Some((x, y)) = cur {
                    f.hash();
                    db.entry(x).or_insert((ca, y, (0, 0, 0)));
                }
            }
            snap.push(("db".into(), take(&f)));
            let full_y = |f: &F, y2: u64, aux: (u64, u64, usize)| -> u64 {
                if y2 != NOY {
                    return y2;
                }
                f.padd(Some((aux.0, aux.1)), f.pneg(arms[aux.2].0))
                    .unwrap()
                    .1
            };
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut trail: Vec<(u64, u64, u64, (u64, u64, usize))> = Vec::new();
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    let prev = cur;
                    let pa = ca;
                    let (nx, sx) = step_x(&f, prev, arms[h].0);
                    cur = nx;
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        trail.push((x, ca, y, (0, 0, 0)));
                    }
                    if let (Some(xm), Some((px, py))) = (sx, prev) {
                        trail.push((xm, (pa + n - arms[h].1) % n, NOY, (px, py, h)));
                    }
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&x) {
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if let Some(xm) = sx {
                        f.hash();
                        if let Some(&(a2, y2, aux)) = db.get(&xm) {
                            let ym = f.padd(prev, f.pneg(arms[h].0)).unwrap().1; // target sibling in full (counted)
                            let y2 = full_y(&f, y2, aux);
                            let eps: i128 = if ym == y2 { 1 } else { -1 };
                            break md(
                                eps * a2 as i128 - md(pa as i128 - arms[h].1 as i128, n) as i128,
                                n,
                            );
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, off, y, aux) in trail {
                    f.hash();
                    db.entry(x).or_insert(((off + v) % n, y, aux));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }

        "rhoseq" => {
            // validator control: anchored rho with a sequential phase-1 database {G, 2G, ..., NG} (no walk waste, no restarts);
            // phase 2 identical to "rho".
            const ARMS: usize = 32;
            const RESTART: usize = 32;
            const BANK: usize = 64;
            let kappa = param;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let arms: Vec<(Pt, u64)> = (0..ARMS).map(|_| bank[rng.gen_range(0..BANK)]).collect();
            let hidx = |pt: Pt| -> usize { pt.map(|(x, _)| (x as usize) % ARMS).unwrap_or(0) };
            let db_target = (kappa * ((n as f64) * targets as f64).sqrt()).ceil() as usize;
            let mut db: HashMap<u64, (u64, u64)> = HashMap::with_capacity(db_target * 2);
            let mut cur: Pt = g;
            let mut ca = 1u64;
            let mut db_steps = 0u64;
            while db.len() < db_target {
                let (x, y) = cur.unwrap();
                f.hash();
                db.entry(x).or_insert((ca, y));
                cur = f.padd(cur, g);
                ca += 1;
                db_steps += 1;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            for t in 0..targets {
                let q = qs[t];
                let mut cur: Pt = None;
                let mut ca = 0u64;
                let mut st = RESTART;
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let v = loop {
                    if st >= RESTART || cur.is_none() {
                        // chained starts tp <- tp + B_k from tp = Q (tp = ta G + Q): one addition per restart
                        let k1 = rng.gen_range(0..BANK);
                        tp = f.padd(tp, bank[k1].0);
                        ta = (ta + bank[k1].1) % n;
                        cur = tp;
                        ca = ta;
                        st = 0;
                    }
                    let h = hidx(cur);
                    cur = f.padd(cur, arms[h].0);
                    ca = (ca + arms[h].1) % n;
                    st += 1;
                    steps_total += 1;
                    if let Some((x, y)) = cur {
                        f.hash();
                        if let Some(&(a2, y2)) = db.get(&x) {
                            let eps: i128 = if y == y2 { 1 } else { -1 };
                            break md(eps * a2 as i128 - ca as i128, n);
                        }
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"db_steps\":{},\"target_steps\":{}",
                    db.len(),
                    db_steps,
                    steps_total
                ),
            }
        }
        "rhodp" => {
            // Multi-target distinguished-point rho on {P, -P} classes (Kuhn-Struik with retention of solved
            // walks, optional precomputation). Walk: P -> canon(P + A_h(P)), R = 1024 steps A_i = B_j + B_k from
            // the 64-point bank; if the next point would use the same step index (a fruitless 2-cycle risk),
            // the step A_(h+1) is used instead (the extra addition is counted). A point is distinguished when
            // d hash bits are zero (theta = 2^d steps per walk on average); walks longer than 20 theta are
            // abandoned (their steps stay counted). param = kappa: precomputation walks from known-log starts
            // until kappa sqrt(nT) steps are spent; args[6] = offset of d from d0 = round(log2(n/T)/4).
            // Each target walks from chained starts Q + s G until a distinguished point is found in the
            // database (solved) or in its own earlier walks with the opposite Q sign (solved); otherwise it is
            // kept pending. After a target is solved, its pending distinguished points are inserted with known
            // logs. Every addition is counted; every database or pending lookup and insert is one hash op.
            const BANK: usize = 64;
            const R: usize = 1024;
            let kappa = param;
            let doff: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
            let d0 = (0.25 * ((n as f64) / targets as f64).log2()).round() as i32;
            let d = (d0 + doff).max(1) as u32;
            let mask = (1u64 << d) - 1;
            let theta = 1u64 << d;
            let maxwalk = 20 * theta;
            let half = f.p / 2;
            let mut bank: Vec<(Pt, u64)> = Vec::new();
            for _ in 0..BANK {
                let k = rng.gen_range(1..n);
                bank.push((f.smul(g, k), k));
            }
            let mut arms: Vec<(Pt, u64)> = Vec::with_capacity(R);
            for _ in 0..R {
                let (i, j) = (rng.gen_range(0..BANK), rng.gen_range(0..BANK));
                arms.push((f.padd(bank[i].0, bank[j].0), (bank[i].1 + bank[j].1) % n));
            }
            let mix = |x: u64| -> u64 { x.wrapping_mul(0x9E37_79B9_7F4A_7C15) };
            let idx = |p: Pt| -> usize { p.map(|(x, _)| (mix(x) >> 40) as usize % R).unwrap_or(0) };
            let is_dp =
                |p: Pt| -> bool { p.map(|(x, _)| (mix(x) >> 8) & mask == 0).unwrap_or(false) };
            // canonical class representative (y <= p/2); returns the sign applied
            let canon = |p: Pt| -> (Pt, i64) {
                match p {
                    Some((x, y)) if y > half => (Some((x, f.p - y)), -1),
                    _ => (p, 1),
                }
            };
            // one walk step on (point, a, b) with point = a G + b Q canonical
            let step = |f: &F, p: Pt, a: u64, b: i64| -> (Pt, u64, i64) {
                let h = idx(p);
                let (mut q1, mut s1) = canon(f.padd(p, arms[h].0));
                let mut hh = h;
                if q1.is_some() && idx(q1) == h {
                    hh = (h + 1) % R;
                    let r = canon(f.padd(p, arms[hh].0));
                    q1 = r.0;
                    s1 = r.1;
                }
                let a1 = md(s1 as i128 * ((a + arms[hh].1) % n) as i128, n);
                (q1, a1, s1 * b)
            };
            let mut db: HashMap<u64, u64> = HashMap::new();
            let pre_budget = (kappa * ((n as f64) * targets as f64).sqrt()) as u64;
            let mut pre_steps = 0u64;
            let mut sp: Pt = bank[0].0;
            let mut sa = bank[0].1;
            while pre_steps < pre_budget {
                let k1 = rng.gen_range(0..BANK);
                sp = f.padd(sp, bank[k1].0);
                sa = (sa + bank[k1].1) % n;
                let (mut p, s0) = canon(sp);
                let mut a = md(s0 as i128 * sa as i128, n);
                let mut len = 0u64;
                while p.is_some() && len < maxwalk {
                    let r = step(&f, p, a, 0);
                    p = r.0;
                    a = r.1;
                    len += 1;
                    if is_dp(p) {
                        f.hash();
                        db.entry(p.unwrap().0).or_insert(a);
                        break;
                    }
                }
                pre_steps += len;
            }
            snap.push(("db".into(), take(&f)));
            let mut steps_total = 0u64;
            let mut walks = 0u64;
            let mut abandoned = 0u64;
            let inv2 = n.div_ceil(2);
            for t in 0..targets {
                let q = qs[t];
                let mut tp: Pt = q;
                let mut ta = 0u64;
                let mut pending: HashMap<u64, (u64, i64)> = HashMap::new();
                let v = 'tgt: loop {
                    let k1 = rng.gen_range(0..BANK);
                    tp = f.padd(tp, bank[k1].0);
                    ta = (ta + bank[k1].1) % n;
                    let (mut p, s0) = canon(tp);
                    let mut a = md(s0 as i128 * ta as i128, n);
                    let mut b = s0;
                    let mut len = 0u64;
                    walks += 1;
                    while p.is_some() && len < maxwalk {
                        let r = step(&f, p, a, b);
                        p = r.0;
                        a = r.1;
                        b = r.2;
                        len += 1;
                        if is_dp(p) {
                            let x = p.unwrap().0;
                            f.hash();
                            if let Some(&a2) = db.get(&x) {
                                // a G + b Q = a2 G
                                break 'tgt md(b as i128 * (a2 as i128 - a as i128), n);
                            }
                            f.hash();
                            if let Some(&(a2, b2)) = pending.get(&x) {
                                if b2 != b {
                                    // a + b q = a2 + b2 q, b2 = -b: q = (a2 - a) / (2 b)
                                    let num = md(a2 as i128 - a as i128, n);
                                    let qv = md(
                                        b as i128
                                            * ((num as u128 * inv2 as u128) % n as u128) as i128,
                                        n,
                                    );
                                    f.cm.set(f.cm.get() + 1);
                                    break 'tgt qv;
                                }
                            } else {
                                f.hash();
                                pending.insert(x, (a, b));
                            }
                            break;
                        }
                    }
                    steps_total += len;
                    if len >= maxwalk {
                        abandoned += 1;
                    }
                    if steps_total > 4_000_000_000 {
                        panic!("budget");
                    }
                };
                found[t] = v;
                for (x, (a, b)) in pending {
                    f.hash();
                    db.entry(x)
                        .or_insert(md(a as i128 + b as i128 * v as i128, n));
                }
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"db\":{},\"d\":{},\"pre_steps\":{},\"target_steps\":{},\"walks\":{},\"abandoned\":{}",
                    db.len(),
                    d,
                    pre_steps,
                    steps_total,
                    walks,
                    abandoned
                ),
            }
        }
        "bsgs2" => {
            let mf = param;
            let mx_goal = (mf * ((n as f64) * targets as f64).sqrt() / 2.0).ceil() as u64;
            // baby classes |k| <= Mx, Mx = (2u+1)(v-1) + u
            let u = ((mx_goal as f64 / 2.0).sqrt().round() as u64).max(1);
            let v = ((mx_goal + 2 * u + 1) / (2 * u + 1)).max(1) + 1;
            let mx = (2 * u + 1) * (v - 1) + u;
            assert!(2 * mx + 1 < n, "baby set exceeds group");
            let mut a_pts: Vec<Pt> = vec![g];
            for _ in 1..u {
                let l = *a_pts.last().unwrap();
                a_pts.push(f.padd(l, g));
            }
            let ugen = f.padd(f.padd(a_pts[u as usize - 1], a_pts[u as usize - 1]), g); // (2u+1) G
            let mut b_pts: Vec<Pt> = vec![None];
            for _ in 1..v {
                let l = *b_pts.last().unwrap();
                b_pts.push(f.padd(l, ugen));
            }
            let mut baby: HashMap<u64, u64> = HashMap::with_capacity((2 * mx + 4) as usize);
            for i in 0..u as usize {
                f.hash();
                baby.entry(a_pts[i].unwrap().0).or_insert(i as u64 + 1);
            }
            for j in 1..v as usize {
                let c = (2 * u + 1) * j as u64;
                f.hash();
                baby.entry(b_pts[j].unwrap().0).or_insert(c);
                for i in 0..u as usize {
                    if let Some((xp, xm)) = f.xpair(b_pts[j], a_pts[i]) {
                        f.hash();
                        baby.entry(xp).or_insert(c + i as u64 + 1);
                        f.hash();
                        baby.entry(xm).or_insert(c - i as u64 - 1);
                    }
                }
            }
            snap.push(("baby".into(), take(&f)));
            // recompute the baby point k G (|k| <= Mx) from the lists: one counted addition at most
            let baby_pt = |f: &F, k: u64| -> Pt {
                let w = 2 * u + 1;
                let j = (k + u) / w;
                let r = k as i64 - (j * w) as i64;
                if j == 0 {
                    a_pts[r as usize - 1]
                } else if r == 0 {
                    b_pts[j as usize]
                } else if r > 0 {
                    f.padd(b_pts[j as usize], a_pts[r as usize - 1])
                } else {
                    f.padd(b_pts[j as usize], f.pneg(a_pts[(-r) as usize - 1]))
                }
            };
            let stride = 2 * mx + 1;
            let spt = f.smul(g, stride);
            let mut h_pts: Vec<Pt> = vec![None, spt];
            let mut trials = 0u64;
            for t in 0..targets {
                let q = qs[t];
                trials += 1;
                f.hash();
                let mut v_found: Option<u64> = None;
                if let Some(&k) = baby.get(&q.unwrap().0) {
                    let kp = baby_pt(&f, k);
                    v_found = Some(if kp == q { k % n } else { (n - k % n) % n });
                }
                let mut kk = 1usize;
                while v_found.is_none() {
                    while h_pts.len() <= kk {
                        let l = *h_pts.last().unwrap();
                        h_pts.push(f.padd(l, spt));
                    }
                    trials += 2;
                    let hl = md(kk as i128 * stride as i128, n);
                    if q.unwrap().0 == h_pts[kk].unwrap().0 {
                        v_found = Some(if q == h_pts[kk] { hl } else { (n - hl) % n });
                    } else if let Some((xp, xm)) = f.xpair(q, h_pts[kk]) {
                        for (xx, sw) in [(xp, 1i64), (xm, -1i64)] {
                            f.hash();
                            if let Some(&k) = baby.get(&xx) {
                                let dpt = baby_pt(&f, k);
                                v_found = Some(resolve(&f, q, h_pts[kk], sw, hl, dpt, k % n, n));
                                break;
                            }
                        }
                    }
                    kk += 1;
                    if kk as u64 > n {
                        panic!("budget");
                    }
                }
                found[t] = v_found.unwrap();
            }
            snap.push(("targets".into(), take(&f)));
            Out {
                ok: 0,
                extra: format!(
                    "\"u\":{},\"v\":{},\"mx\":{},\"baby\":{},\"giants\":{},\"trials\":{}",
                    u,
                    v,
                    mx,
                    baby.len(),
                    h_pts.len(),
                    trials
                ),
            }
        }
        _ => panic!("unknown arm"),
    };
    // Verification (counted): every recovered log by double-and-add.
    let mut ok = out.ok;
    for t in 0..targets {
        if f.smul(g, found[t]) == qs[t] && found[t] == xs[t] % n {
            ok += 1;
        }
    }
    snap.push(("verify".into(), take(&f)));
    let _ = f.small(0);
    let phases: Vec<String> = snap
        .iter()
        .map(|(k, v)| format!("\"{}\":[{},{},{},{},{}]", k, v[0], v[1], v[2], v[3], v[4]))
        .collect();
    println!(
        "{{\"arm\":\"{}\",\"n\":{},\"p\":{},\"targets\":{},\"seed\":{},\"param\":{},\"ok\":{},\"I\":{},\"M\":{},\"S\":{},\"H\":{},\"L\":{},\"phases\":{{{}}},{},\"ms\":{:.1}}}",
        arm, n, p, targets, seed, param, ok, f.ci.get(), f.cm.get(), f.cs.get(), f.ch.get(), f.cl.get(),
        phases.join(","), out.extra, t0.elapsed().as_secs_f64() * 1e3
    );
}
