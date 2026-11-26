//! Graeffe-collapsed RR5 presentations as quadratic F4 inputs (PR #5136 lead).
//!
//! `order = 0`: collapsed presentation. theta=(b,c,d), the RR5 quartic U(theta),
//! ten root-squaring steps x -> x^2 - 2 (so roots become D_1024(x_i)), and the four
//! centered terminal residuals F_j = e_j(D_1024(x_i) - tau).
//! `order = 2`: the same circuit carried as second-order jets in (b,c,d), plus the six
//! second derivatives of F_3 (the six polar equations of PR #5136).
//! Every non-constant intermediate component is lifted to a fresh variable with one
//! quadratic defining equation, so all inputs have total degree at most two.

use std::collections::BTreeMap;

pub const P: u64 = 2_147_483_647;
pub type MP = BTreeMap<Vec<u32>, u64>;

fn mm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}

pub struct Ctx {
    pub n: usize,
    pub cap: usize,
    pub eqs: Vec<MP>,
}

impl Ctx {
    fn konst(&self, c: u64) -> MP {
        let mut m = MP::new();
        if c % P != 0 {
            m.insert(vec![0; self.cap], c % P);
        }
        m
    }
    fn var(&self, i: usize) -> MP {
        let mut e = vec![0; self.cap];
        e[i] = 1;
        let mut m = MP::new();
        m.insert(e, 1);
        m
    }
    fn add(&self, a: &MP, b: &MP) -> MP {
        let mut r = a.clone();
        for (e, c) in b {
            let v = (r.get(e).copied().unwrap_or(0) + c) % P;
            if v == 0 {
                r.remove(e);
            } else {
                r.insert(e.clone(), v);
            }
        }
        r
    }
    fn scale(&self, a: &MP, k: u64) -> MP {
        let k = k % P;
        a.iter().filter_map(|(e, c)| {
            let v = mm(*c, k);
            (v != 0).then(|| (e.clone(), v))
        }).collect()
    }
    fn sub(&self, a: &MP, b: &MP) -> MP {
        self.add(a, &self.scale(b, P - 1))
    }
    fn mul(&self, a: &MP, b: &MP) -> MP {
        let mut r = MP::new();
        for (ea, ca) in a {
            for (eb, cb) in b {
                let e: Vec<u32> = ea.iter().zip(eb).map(|(x, y)| x + y).collect();
                let v = (r.get(&e).copied().unwrap_or(0) + mm(*ca, *cb)) % P;
                if v == 0 {
                    r.remove(&e);
                } else {
                    r.insert(e, v);
                }
            }
        }
        r
    }
    fn is_const(&self, a: &MP) -> bool {
        a.keys().all(|e| e.iter().all(|&x| x == 0))
    }
    /// Replace a non-constant polynomial by a fresh variable `v` with `v - a = 0`.
    fn lift(&mut self, a: &MP) -> MP {
        if self.is_const(a) {
            return a.clone();
        }
        let v = self.n;
        self.n += 1;
        assert!(self.n <= self.cap, "variable capacity");
        let x = self.var(v);
        self.eqs.push(self.sub(&x, a));
        x
    }
}

/// Second-order jet: value, three first and six second derivatives (bb,bc,bd,cc,cd,dd).
#[derive(Clone)]
pub struct Jet {
    pub c: Vec<MP>, // length 1 (order 0) or 10 (order 2)
}

const PAIRS: [(usize, usize); 6] = [(0, 0), (0, 1), (0, 2), (1, 1), (1, 2), (2, 2)];

fn pair_index(i: usize, j: usize) -> usize {
    let (i, j) = if i <= j { (i, j) } else { (j, i) };
    PAIRS.iter().position(|&p| p == (i, j)).unwrap()
}

impl Jet {
    fn constant(ctx: &Ctx, k: u64, order: usize) -> Jet {
        Jet::constant_cap(ctx.cap, k, order)
    }
    fn constant_cap(cap: usize, k: u64, order: usize) -> Jet {
        let mut m = MP::new();
        if k % P != 0 {
            m.insert(vec![0; cap], k % P);
        }
        let mut c = vec![m];
        if order == 2 {
            c.extend((0..9).map(|_| MP::new()));
        }
        Jet { c }
    }
    fn add(&self, o: &Jet, ctx: &Ctx) -> Jet {
        Jet { c: self.c.iter().zip(&o.c).map(|(a, b)| ctx.add(a, b)).collect() }
    }
    fn sub(&self, o: &Jet, ctx: &Ctx) -> Jet {
        Jet { c: self.c.iter().zip(&o.c).map(|(a, b)| ctx.sub(a, b)).collect() }
    }
    fn scale(&self, k: u64, ctx: &Ctx) -> Jet {
        Jet { c: self.c.iter().map(|a| ctx.scale(a, k)).collect() }
    }
    fn mul(&self, o: &Jet, ctx: &Ctx) -> Jet {
        let (x, y) = (&self.c, &o.c);
        let mut c = vec![ctx.mul(&x[0], &y[0])];
        if x.len() == 10 {
            for i in 0..3 {
                c.push(ctx.add(&ctx.mul(&x[1 + i], &y[0]), &ctx.mul(&x[0], &y[1 + i])));
            }
            for &(i, j) in PAIRS.iter() {
                let h = 4 + pair_index(i, j);
                let mut t = ctx.add(&ctx.mul(&x[h], &y[0]), &ctx.mul(&x[0], &y[h]));
                t = ctx.add(&t, &ctx.mul(&x[1 + i], &y[1 + j]));
                t = ctx.add(&t, &ctx.mul(&x[1 + j], &y[1 + i]));
                c.push(t);
            }
        }
        Jet { c }
    }
    fn lift(&self, ctx: &mut Ctx) -> Jet {
        Jet { c: self.c.iter().map(|a| ctx.lift(a)).collect() }
    }
}

fn poly_product(a: &[Jet], b: &[Jet], ctx: &Ctx, order: usize) -> Vec<Jet> {
    let mut out: Vec<Jet> = (0..a.len() + b.len() - 1).map(|_| Jet::constant(ctx, 0, order)).collect();
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = out[i + j].add(&x.mul(y, ctx), ctx);
        }
    }
    out
}

pub struct GraeffeSystem {
    pub variables: usize,
    pub equations: Vec<MP>,
    /// Ascending coefficients a0..a3 of the RR5 quartic as polynomials in b,c,d (order-0 part).
    pub quartic: Vec<MP>,
}

/// Build the presentation for target (r,s) on y^2=x^3+a x+b with Dickson terminal tau.
pub fn build(order: usize, curve_a: u64, target: (u64, u64), tau: u64) -> GraeffeSystem {
    let cap = if order == 2 { 520 } else { 64 };
    let mut ctx = Ctx { n: 3, cap, eqs: Vec::new() };
    let theta: Vec<Jet> = (0..3).map(|i| {
        let mut c = vec![ctx.var(i)];
        if order == 2 {
            for k in 0..3 {
                c.push(ctx.konst(u64::from(k == i)));
            }
            c.extend((0..6).map(|_| MP::new()));
        }
        Jet { c }
    }).collect();
    let (r, s) = target;
    let k = |v: u64| Jet::constant_cap(cap, v, order);
    // U = (X^2+rX+r^2+a) V^2 - (X-r) L^2 - 2 s L V, V = X + b, L = cX + d (PR #5136 rr5_quartic).
    let v = vec![theta[0].clone(), k(1)];
    let l = vec![theta[2].clone(), theta[1].clone()];
    let dr = vec![k((mm(r, r) + curve_a) % P), k(r), k(1)];
    let vv = poly_product(&v, &v, &ctx, order);
    let mut u = poly_product(&dr, &vv, &ctx, order);
    let ll = poly_product(&[k(P - r), k(1)], &poly_product(&l, &l, &ctx, order), &ctx, order);
    let lv = poly_product(&l, &v, &ctx, order);
    for (i, x) in ll.iter().enumerate() {
        u[i] = u[i].sub(x, &ctx);
    }
    for (i, x) in lv.iter().enumerate() {
        u[i] = u[i].sub(&x.scale((2 * s) % P, &ctx), &ctx);
    }
    let quartic: Vec<MP> = u[..4].iter().map(|j| j.c[0].clone()).collect();
    let mut a: Vec<Jet> = u[..4].iter().map(|j| j.lift(&mut ctx)).collect();
    for _ in 0..10 {
        let (a0, a1, a2, a3) = (&a[0], &a[1], &a[2], &a[3]);
        let w3 = a2.scale(2, &ctx).sub(&a3.mul(a3, &ctx), &ctx);
        let w2 = a2.mul(a2, &ctx).add(&a0.scale(2, &ctx), &ctx).sub(&a3.mul(a1, &ctx).scale(2, &ctx), &ctx);
        let w1 = a2.mul(a0, &ctx).scale(2, &ctx).sub(&a1.mul(a1, &ctx), &ctx);
        let w0 = a0.mul(a0, &ctx);
        let g0 = k(16).add(&w3.scale(8, &ctx), &ctx).add(&w2.scale(4, &ctx), &ctx).add(&w1.scale(2, &ctx), &ctx).add(&w0, &ctx);
        let g1 = k(32).add(&w3.scale(12, &ctx), &ctx).add(&w2.scale(4, &ctx), &ctx).add(&w1, &ctx);
        let g2 = k(24).add(&w3.scale(6, &ctx), &ctx).add(&w2, &ctx);
        let g3 = k(8).add(&w3, &ctx);
        a = vec![g0.lift(&mut ctx), g1.lift(&mut ctx), g2.lift(&mut ctx), g3.lift(&mut ctx)];
    }
    let (t2, t3) = (mm(tau, tau), mm(mm(tau, tau), tau));
    let t4 = mm(t3, tau);
    let f1 = a[3].scale(P - 1, &ctx).sub(&k(mm(4, tau)), &ctx);
    let f2 = a[2].add(&a[3].scale(mm(3, tau), &ctx), &ctx).add(&k(mm(6, t2)), &ctx);
    let f3 = a[1].scale(P - 1, &ctx).sub(&a[2].scale(mm(2, tau), &ctx), &ctx).sub(&a[3].scale(mm(3, t2), &ctx), &ctx).sub(&k(mm(4, t3)), &ctx);
    let f4 = a[0].add(&a[1].scale(tau, &ctx), &ctx).add(&a[2].scale(t2, &ctx), &ctx).add(&a[3].scale(t3, &ctx), &ctx).add(&k(t4), &ctx);
    for f in [&f1, &f2, &f3, &f4] {
        ctx.eqs.push(f.c[0].clone());
    }
    if order == 2 {
        for h in 4..10 {
            ctx.eqs.push(f3.c[h].clone());
        }
    }
    let n = ctx.n;
    let trim = |m: &MP| -> MP { m.iter().map(|(e, c)| (e[..n].to_vec(), *c)).collect() };
    GraeffeSystem {
        variables: n,
        equations: ctx.eqs.iter().map(trim).filter(|m| !m.is_empty()).collect(),
        quartic: quartic.iter().map(trim).collect(),
    }
}
