// Standalone measurement: cost of one index-calculus relation trial on a
// binary hyperelliptic Jacobian, y^2 + h y = f over F_2, deg f = 2g+1.
// Build: rustc -C opt-level=3 -C target-cpu=native trial_cost.rs -o trial_cost
// Counted unit W = executed F_2[x] u64 word XOR(-with-shift) operations.
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::time::Instant;

static WC: AtomicU64 = AtomicU64::new(0);
static EV: AtomicU64 = AtomicU64::new(0);
static RED: AtomicU64 = AtomicU64::new(0);
static COMPW: AtomicU64 = AtomicU64::new(0);
static COMPE: AtomicU64 = AtomicU64::new(0);
fn cw(n: u64) { WC.fetch_add(n, Relaxed); }
fn ev() { EV.fetch_add(1, Relaxed); }
fn ctr() -> (u64, u64) { (WC.load(Relaxed), EV.load(Relaxed)) }

type P = Vec<u64>;
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, n: u64) -> u64 { self.next() % n }
}
fn splitmix(x: u64) -> u64 { Rng(x).next() }

fn trim(a: &mut P) { while a.last() == Some(&0) { a.pop(); } }
fn deg(a: &P) -> i64 {
    match a.last() { None => -1, Some(w) => (a.len() * 64 - 1 - w.leading_zeros() as usize) as i64 }
}
fn bit(a: &P, i: usize) -> bool { a.get(i / 64).map_or(false, |w| (w >> (i % 64)) & 1 == 1) }
fn flip(a: &mut P, i: usize) { if a.len() <= i / 64 { a.resize(i / 64 + 1, 0); } a[i / 64] ^= 1u64 << (i % 64); trim(a); }
fn one() -> P { vec![1] }
fn xpoly() -> P { vec![2] }
fn from_bits(bits: &[usize]) -> P { let mut p = vec![]; for &b in bits { flip(&mut p, b); } p }
fn xor_shift(r: &mut P, m: &P, s: usize) {
    let (ws, bs) = (s / 64, s % 64);
    let need = m.len() + ws + 1;
    if r.len() < need { r.resize(need, 0); }
    for (k, &w) in m.iter().enumerate() {
        r[k + ws] ^= w << bs;
        if bs > 0 { r[k + ws + 1] ^= w >> (64 - bs); }
    }
}
fn add_raw(a: &P, b: &P) -> P {
    let n = a.len().max(b.len());
    cw(n as u64);
    let mut r = vec![0u64; n];
    for i in 0..n { r[i] = a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0); }
    trim(&mut r); r
}
fn pop(a: &P) -> u64 { a.iter().map(|w| w.count_ones() as u64).sum() }
fn mul_raw(a: &P, b: &P) -> P {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let (c1, c2) = (pop(a) * (b.len() as u64 + 1), pop(b) * (a.len() as u64 + 1));
    let (a, b) = if c1 <= c2 { (a, b) } else { (b, a) };
    cw(c1.min(c2));
    let mut r = vec![0u64; a.len() + b.len() + 1];
    for (i, &w0) in a.iter().enumerate() {
        let mut w = w0;
        while w != 0 { let j = w.trailing_zeros() as usize; w &= w - 1; xor_shift(&mut r, b, i * 64 + j); }
    }
    trim(&mut r); r
}
fn spread(x: u32) -> u64 {
    let mut x = x as u64;
    x = (x | (x << 16)) & 0x0000FFFF0000FFFF;
    x = (x | (x << 8)) & 0x00FF00FF00FF00FF;
    x = (x | (x << 4)) & 0x0F0F0F0F0F0F0F0F;
    x = (x | (x << 2)) & 0x3333333333333333;
    (x | (x << 1)) & 0x5555555555555555
}
fn sqr_raw(a: &P) -> P {
    cw(2 * a.len() as u64);
    let mut r = Vec::with_capacity(2 * a.len());
    for &w in a { r.push(spread(w as u32)); r.push(spread((w >> 32) as u32)); }
    trim(&mut r); r
}
fn divmod_raw(a: &P, m: &P) -> (P, P) {
    let dm = deg(m);
    assert!(dm >= 0, "division by zero");
    let dm = dm as usize;
    let mut r = a.clone();
    let mut q: P = vec![];
    let mut c = 0u64;
    loop {
        trim(&mut r);
        let d = deg(&r);
        if d < dm as i64 { break; }
        let s = d as usize - dm;
        xor_shift(&mut r, m, s);
        if q.len() <= s / 64 { q.resize(s / 64 + 1, 0); }
        q[s / 64] |= 1 << (s % 64);
        c += m.len() as u64 + 1;
    }
    cw(c); trim(&mut r); (q, r)
}
fn gcd_raw(a: &P, b: &P) -> P {
    let (mut a, mut b) = (a.clone(), b.clone());
    while !b.is_empty() { let (_, r) = divmod_raw(&a, &b); a = b; b = r; }
    a
}
// returns (g, s) with s*a = g (mod b)
fn xgcd_raw(a: &P, b: &P) -> (P, P) {
    let (mut r0, mut r1, mut s0, mut s1) = (a.clone(), b.clone(), one(), vec![]);
    while !r1.is_empty() {
        let (q, r) = divmod_raw(&r0, &r1);
        let s2 = add_raw(&s0, &mul_raw(&q, &s1));
        r0 = r1; r1 = r; s0 = s1; s1 = s2;
    }
    (r0, s0)
}
// event-counting public wrappers (one event per primitive call)
fn add(a: &P, b: &P) -> P { add_raw(a, b) }
fn mul(a: &P, b: &P) -> P { ev(); mul_raw(a, b) }
fn sqr(a: &P) -> P { ev(); sqr_raw(a) }
fn modp(a: &P, m: &P) -> P { ev(); divmod_raw(a, m).1 }
fn divexact(a: &P, b: &P) -> P { ev(); let (q, r) = divmod_raw(a, b); assert!(r.is_empty(), "inexact division"); q }
fn gcd(a: &P, b: &P) -> P { ev(); gcd_raw(a, b) }
fn xgcd(a: &P, b: &P) -> (P, P) { ev(); xgcd_raw(a, b) }
fn deriv(a: &P) -> P {
    let mut r: P = a.iter().map(|w| (w & 0xAAAA_AAAA_AAAA_AAAA) >> 1).collect(); trim(&mut r); r
}
fn rand_poly(rng: &mut Rng, d: usize, monic: bool) -> P {
    let mut p: P = (0..d / 64 + 1).map(|_| rng.next()).collect();
    let top = d % 64;
    let last = p.len() - 1;
    if top < 63 { p[last] &= (1u64 << (top + 1)) - 1; }
    if monic { p[last] |= 1u64 << top; }
    trim(&mut p); p
}
fn is_irreducible(p: &P) -> bool {
    let d = deg(p) as usize;
    if d == 0 { return false; }
    if d == 1 { return true; }
    let x = xpoly();
    let mut xp = x.clone();
    for _ in 1..=d / 2 {
        xp = modp(&sqr(&xp), p);
        if deg(&gcd(&add(&xp, &x), p)) > 0 { return false; }
    }
    true
}

// ---------------------------------------------------------------- Jacobian
struct Curve { g: usize, h: P, f: P }
type Div = (P, P);
fn identity() -> Div { (one(), vec![]) }
fn is_id(d: &Div) -> bool { deg(&d.0) == 0 }
fn nonsingular(h: &P, f: &P) -> bool {
    let hp = deriv(h);
    let t = add_raw(&mul_raw(&sqr_raw(&hp), f), &sqr_raw(&deriv(f)));
    deg(&gcd_raw(h, &t)) == 0
}
fn make_curve(g: usize, rng: &mut Rng) -> Curve {
    loop {
        let h = rand_poly(rng, g, true);
        let f = rand_poly(rng, 2 * g + 1, true);
        if nonsingular(&h, &f) { return Curve { g, h, f }; }
    }
}
fn negate(c: &Curve, d: &Div) -> Div { (d.0.clone(), modp(&add(&d.1, &c.h), &d.0)) }
fn on_curve(c: &Curve, d: &Div) -> bool {
    let t = add_raw(&add_raw(&sqr_raw(&d.1), &mul_raw(&d.1, &c.h)), &c.f);
    let (_, r) = divmod_raw(&t, &d.0);
    let dv = deg(&d.0);
    dv >= 0 && dv <= c.g as i64 && r.is_empty() && (deg(&d.1) < dv || (dv == 0 && d.1.is_empty()))
}
fn cantor_add(c: &Curve, a: &Div, b: &Div) -> Div {
    if is_id(a) { return b.clone(); }
    if is_id(b) { return a.clone(); }
    let (u1, v1) = a; let (u2, v2) = b;
    let (ws0, es0) = ctr();
    let (d1, e1) = xgcd(u1, u2);
    let (mut u, mut v);
    if deg(&d1) == 0 {
        let t = modp(&mul(&add(v1, v2), &e1), u2);
        v = add(v1, &mul(u1, &t));
        u = mul(u1, u2);
        v = modp(&v, &u);
    } else {
        let e2 = divexact(&add(&d1, &mul(&e1, u1)), u2);
        let w = add(&add(v1, v2), &c.h);
        let (d, c1, c2);
        if w.is_empty() { d = d1.clone(); c1 = one(); c2 = vec![]; }
        else {
            let (dd, cc) = xgcd(&d1, &w);
            c2 = divexact(&add(&dd, &mul(&cc, &d1)), &w);
            d = dd; c1 = cc;
        }
        let s1 = mul(&c1, &e1); let s2 = mul(&c1, &e2);
        u = divexact(&mul(u1, u2), &sqr(&d));
        let n1 = mul(&mul(&s1, u1), v2);
        let n2 = mul(&mul(&s2, u2), v1);
        let n3 = mul(&c2, &add(&mul(v1, v2), &c.f));
        v = divexact(&add(&add(&n1, &n2), &n3), &d);
        v = modp(&v, &u);
    }
    COMPW.fetch_add(ctr().0 - ws0, Relaxed);
    COMPE.fetch_add(ctr().1 - es0, Relaxed);
    while deg(&u) > c.g as i64 {
        RED.fetch_add(1, Relaxed);
        let t = add(&c.f, &add(&mul(&v, &c.h), &sqr(&v)));
        let u2 = divexact(&t, &u);
        v = modp(&add(&c.h, &v), &u2);
        u = u2;
    }
    (u, v)
}
fn rand_place(c: &Curve, rng: &mut Rng, d: usize) -> Option<Div> {
    for _ in 0..400 {
        let p = rand_poly(rng, d, true);
        if bit(&p, 0) == false && d > 1 { continue; }
        if !is_irreducible(&p) { continue; }
        let hm = modp(&c.h, &p);
        let fm = modp(&c.f, &p);
        if hm.is_empty() {
            let mut v = fm; for _ in 0..d - 1 { v = modp(&sqr(&v), &p); }
            return Some((p, v));
        }
        let (_, hi) = xgcd(&hm, &p);
        let cc = modp(&mul(&fm, &sqr(&hi)), &p);
        // solve w^2 + w = cc over F_2 (d <= 12 bits)
        let cv = cc.first().copied().unwrap_or(0);
        let mut piv: Vec<Option<(u64, u64)>> = vec![None; 64];
        for i in 0..d {
            let e: P = vec![1u64 << i];
            let img = modp(&sqr(&e), &p).first().copied().unwrap_or(0) ^ (1u64 << i);
            let (mut vec_, mut comb) = (img, 1u64 << i);
            while vec_ != 0 {
                let t = 63 - vec_.leading_zeros() as usize;
                match piv[t] { Some((pv, pc)) => { vec_ ^= pv; comb ^= pc; } None => { piv[t] = Some((vec_, comb)); break; } }
            }
        }
        let (mut r, mut comb) = (cv, 0u64);
        while r != 0 {
            let t = 63 - r.leading_zeros() as usize;
            match piv[t] { Some((pv, pc)) => { r ^= pv; comb ^= pc; } None => break }
        }
        if r != 0 { continue; }
        let mut w = vec![comb]; trim(&mut w);
        let v = modp(&mul(&hm, &w), &p);
        return Some((p, v));
    }
    None
}
fn rand_divisor(c: &Curve, rng: &mut Rng) -> Div {
    let mut d = identity();
    let mut total = 0usize;
    while total < 2 * c.g + 2 {
        let k = 1 + rng.below(c.g.min(12) as u64) as usize;
        if let Some(pl) = rand_place(c, rng, k) { d = cantor_add(c, &d, &pl); total += k; }
    }
    d
}

// ---------------------------------------------------------- smoothness tests
fn is_smooth(u: &P, b: usize) -> bool {
    let mut rem = u.clone();
    let x = xpoly();
    let mut xp = x.clone();
    let mut d = 1usize;
    loop {
        let dr = deg(&rem);
        if dr <= b as i64 { return true; }
        if dr < 2 * d as i64 { return false; }
        if d > b { return false; }
        xp = modp(&sqr(&xp), &rem);
        let gd = gcd(&add(&xp, &x), &rem);
        if deg(&gd) > 0 {
            rem = divexact(&rem, &gd);
            let mut t = gcd(&rem, &gd);
            while deg(&t) > 0 { rem = divexact(&rem, &t); t = gcd(&rem, &t); }
            xp = modp(&xp, &rem);
        }
        d += 1;
    }
}
fn edf(g: &P, d: usize, rng: &mut Rng, out: &mut Vec<usize>) {
    let dg = deg(g) as usize;
    if dg == d { out.push(d); return; }
    loop {
        let a = rand_poly(rng, dg - 1, false);
        let (mut t, mut s) = (a.clone(), a);
        for _ in 1..d { s = modp(&sqr(&s), g); t = add(&t, &s); }
        let gg = gcd(&t, g);
        if deg(&gg) > 0 && deg(&gg) < dg as i64 {
            edf(&gg, d, rng, out); edf(&divexact(g, &gg), d, rng, out); return;
        }
    }
}
fn factor_degrees_sqfree(f: &P, rng: &mut Rng) -> Vec<usize> {
    let mut out = vec![];
    let (mut rem, x) = (f.clone(), xpoly());
    let mut xp = x.clone();
    let mut d = 1usize;
    while deg(&rem) >= 2 * d as i64 {
        xp = modp(&sqr(&xp), &rem);
        let g = gcd(&add(&xp, &x), &rem);
        if deg(&g) > 0 { edf(&g, d, rng, &mut out); rem = divexact(&rem, &g); xp = modp(&xp, &rem); }
        d += 1;
    }
    if deg(&rem) > 0 { out.push(deg(&rem) as usize); }
    out
}
fn from_u64(x: u64) -> P { let mut p = vec![x]; trim(&mut p); p }
fn brute_max_factor_degree(f: &P) -> usize {
    let mut rem = f.clone();
    let mut mx = 0usize;
    for dd in 1..=12usize {
        for low in 0..(1u64 << dd) {
            let p = from_u64((1u64 << dd) | low);
            loop {
                if deg(&rem) < dd as i64 { break; }
                let (q, r) = divmod_raw(&rem, &p);
                if r.is_empty() { rem = q; mx = mx.max(dd); } else { break; }
            }
        }
    }
    if deg(&rem) > 0 { mx = mx.max(deg(&rem) as usize); }
    mx
}

// ------------------------------------------------------------------- model
fn mobius(n: usize) -> i64 {
    let (mut m, mut res, mut p) = (n, 1i64, 2usize);
    while p * p <= m { if m % p == 0 { m /= p; if m % p == 0 { return 0; } res = -res; } p += 1; }
    if m > 1 { -res } else { res }
}
#[derive(Clone, Copy, Debug)]
struct BCell { b: usize, lfb: f64, lprob: f64, rel: f64, lin: f64, tot: f64 }
fn price_all(g: usize, bmax: usize) -> Vec<BCell> {
    let pts: Vec<f64> = std::iter::once(0.0).chain((1..=g).map(|k| 2f64.powi(k as i32) + 1.0)).collect();
    let mut nd = vec![0f64; g + 1];
    for d in 1..=g {
        let mut acc = 0.0;
        for e in 1..=d { if d % e == 0 { acc += pts[e] * mobius(d / e) as f64; } }
        nd[d] = acc / d as f64;
    }
    let mut ser = vec![0f64; g + 1]; ser[0] = 1.0;
    let bm = bmax.min(g);
    let (mut smooth_at, mut fb_at) = (vec![0f64; bm + 1], vec![0f64; bm + 1]);
    let mut fb = 0.0;
    for d in 1..=g {
        if nd[d] > 0.0 {
            let jmax = g / d;
            let mut binom = vec![1.0f64];
            for j in 1..=jmax { let last = binom[j - 1]; binom.push(last * (nd[d] + (j - 1) as f64) / j as f64); }
            for t in (d..=g).rev() {
                let mut add = 0.0; let mut j = 1;
                while j * d <= t { add += ser[t - j * d] * binom[j]; j += 1; }
                ser[t] += add;
            }
        }
        if d <= bm { fb += nd[d]; smooth_at[d] = ser[g]; fb_at[d] = fb; }
    }
    let ltot = ser[g].log2();
    let mut v = vec![];
    for b in 2..=bm {
        if fb_at[b] < 2.0 || smooth_at[b] <= 0.0 { continue; }
        let lfb = fb_at[b].log2(); let lprob = smooth_at[b].log2() - ltot;
        let rel = lfb - lprob; let lin = 2.0 * lfb + (g as f64).log2();
        v.push(BCell { b, lfb, lprob, rel, lin, tot: rel.max(lin) + 1.0 });
    }
    v
}
fn best_cell(v: &[BCell]) -> BCell { *v.iter().min_by(|a, b| a.tot.partial_cmp(&b.tot).unwrap()).unwrap() }
fn log2_rho(n: f64) -> f64 { 0.5 * (std::f64::consts::PI.log2() + n - 2.0 - (4.0 * n).log2()) }

// ---------------------------------------------------------- F_2^131 and EC
fn mpoly() -> P { from_bits(&[131, 8, 3, 2, 0]) }
fn freduce(mut a: P) -> P {
    let mut c = 0u64;
    loop {
        let d = deg(&a);
        if d < 131 { break; }
        let d = d as usize;
        flip(&mut a, d);
        for t in [8usize, 3, 2, 0] { flip(&mut a, d - 131 + t); }
        c += 4;
    }
    cw(c); a
}
fn fmul(a: &P, b: &P) -> P { freduce(mul_raw(a, b)) }
fn fsqr(a: &P) -> P { freduce(sqr_raw(a)) }
fn finv(a: &P) -> P { let (g, s) = xgcd_raw(a, &mpoly()); assert_eq!(g, one()); s }
struct Ec { a: P, b: P }
fn ec_on(e: &Ec, p: &(P, P)) -> bool {
    let (x, y) = p;
    let l = add_raw(&fsqr(y), &fmul(x, y));
    let r = add_raw(&add_raw(&fmul(&fsqr(x), x), &fmul(&e.a, &fsqr(x))), &e.b);
    l == r
}
fn ec_add(e: &Ec, p: &(P, P), q: &(P, P)) -> Option<(P, P)> {
    if p.0 == q.0 { return None; }
    let l = fmul(&add_raw(&p.1, &q.1), &finv(&add_raw(&p.0, &q.0)));
    let x3 = add_raw(&add_raw(&add_raw(&fsqr(&l), &l), &add_raw(&p.0, &q.0)), &e.a);
    let y3 = add_raw(&add_raw(&fmul(&l, &add_raw(&p.0, &x3)), &x3), &p.1);
    Some((x3, y3))
}
fn half_trace_solve(c: &P) -> Option<P> {
    // trace
    let (mut t, mut tr) = (c.clone(), c.clone());
    for _ in 1..131 { t = fsqr(&t); tr = add_raw(&tr, &t); }
    if !tr.is_empty() { return None; }
    let (mut t, mut z) = (c.clone(), c.clone());
    for _ in 1..=65 { t = fsqr(&fsqr(&t)); z = add_raw(&z, &t); }
    Some(z)
}
fn ec_rand_point(e: &Ec, rng: &mut Rng) -> (P, P) {
    loop {
        let x = rand_poly(rng, 130, false);
        if x.is_empty() { continue; }
        let xi = finv(&x);
        let c = add_raw(&add_raw(&x, &e.a), &fmul(&e.b, &fsqr(&xi)));
        if let Some(z) = half_trace_solve(&c) { let y = fmul(&x, &z); return (x, y); }
    }
}

// ------------------------------------------------------------------ driver
fn me_test(v: &[f64], e_add: f64) -> f64 { v.iter().sum::<f64>() / v.len() as f64 - e_add }
fn mean_sd(v: &[f64]) -> (f64, f64) {
    let m = v.iter().sum::<f64>() / v.len() as f64;
    let s = if v.len() > 1 { (v.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / (v.len() - 1) as f64).sqrt() } else { 0.0 };
    (m, s)
}
fn fit(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let n = xs.len() as f64; let (sx, sy) = (xs.iter().sum::<f64>(), ys.iter().sum::<f64>());
    let sxx: f64 = xs.iter().map(|x| x * x).sum(); let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| x * y).sum();
    let e = (n * sxy - sx * sy) / (n * sxx - sx * sx); ((sy - e * sx) / n, e)
}
struct Walk { table: Vec<Div>, cur: Div, rng: Rng }
fn make_walk(c: &Curve, seed: u64) -> Walk {
    let mut rng = Rng(seed);
    let table: Vec<Div> = (0..16).map(|_| rand_divisor(c, &mut rng)).collect();
    let cur = rand_divisor(c, &mut rng);
    Walk { table, cur, rng }
}
struct Trial { w_add: u64, w_test: u64, w_comp: u64, e_comp: u64, e_add: u64, ev: u64, smooth: bool, degu: i64, ns: u64 }
fn run_trial(c: &Curve, wk: &mut Walk, b: usize) -> Trial {
    let t0 = Instant::now();
    let r = wk.rng.below(16) as usize;
    let (w0, e0) = ctr();
    let (cw0, ce0) = (COMPW.load(Relaxed), COMPE.load(Relaxed));
    wk.cur = cantor_add(c, &wk.cur, &wk.table[r]);
    let (w1, e1) = ctr();
    let (w_comp, e_comp) = (COMPW.load(Relaxed) - cw0, COMPE.load(Relaxed) - ce0);
    let s = is_smooth(&wk.cur.0, b);
    let (w2, e2) = ctr();
    Trial { w_add: w1 - w0, w_test: w2 - w1, w_comp, e_comp, e_add: e1 - e0, ev: e2 - e0, smooth: s, degu: deg(&wk.cur.0), ns: t0.elapsed().as_nanos() as u64 }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let out_dir = args.get(1).cloned().unwrap_or(".".into());
    let master = 20261024u64;
    let mut js = String::new();
    let mut controls_failed = 0usize;
    let mut log = String::new();
    macro_rules! say { ($($a:tt)*) => {{ let s = format!($($a)*); println!("{}", s); writeln!(log, "{}", s).ok(); }} }

    // ---- 5.5 model port
    say!("== model port (f64 price)");
    let mut model_ok = true;
    for (g, wb, wc) in [(128usize, 17usize, 36.41f64), (255, 27, 55.60), (256, 27, 55.65)] {
        let cell = best_cell(&price_all(g, 200));
        let ok = (cell.tot - wc).abs() < 0.01 && cell.b == wb;
        say!("g={g} b={} tot={:.3} (note b={wb}, {wc}) {}", cell.b, cell.tot, if ok { "OK" } else { "MISMATCH" });
        model_ok &= ok;
    }
    if !model_ok { controls_failed += 1; }
    let _ = write!(js, "{{\n \"model_port_ok\": {model_ok},\n");

    // ---- 5.3 smoothness vs full factorisation
    say!("== control: smoothness test vs full factorisation");
    let mut rng = Rng(splitmix(master ^ 0xC0DE));
    let (mut na, mut a_s, mut a_n, mut a_bad) = (0, 0, 0, 0);
    while na < 300 {
        let d = 8 + rng.below(89) as usize;
        let b = 2 + rng.below(23) as usize;
        let f = rand_poly(&mut rng, d, true);
        if deg(&gcd_raw(&f, &deriv(&f))) != 0 { continue; }
        let fd = factor_degrees_sqfree(&f, &mut rng);
        assert_eq!(fd.iter().sum::<usize>() as i64, deg(&f));
        let truth = fd.iter().all(|&k| k <= b);
        let got = is_smooth(&f, b);
        na += 1; if truth { a_s += 1 } else { a_n += 1 }
        if truth != got { a_bad += 1; }
    }
    let (mut nb, mut b_s, mut b_n, mut b_bad, mut b_rep) = (0, 0, 0, 0, 0);
    while nb < 300 {
        let k = 3 + rng.below(4) as usize;
        let mut f = one();
        for _ in 0..k { let d = 1 + rng.below(6) as usize; f = mul_raw(&f, &rand_poly(&mut rng, d, true)); }
        if deg(&f) > 24 { continue; }
        let b = 2 + rng.below(9) as usize;
        let truth = brute_max_factor_degree(&f) <= b;
        let got = is_smooth(&f, b);
        nb += 1; if truth { b_s += 1 } else { b_n += 1 }
        if deg(&gcd_raw(&f, &deriv(&f))) != 0 { b_rep += 1; }
        if truth != got { b_bad += 1; }
    }
    say!("A (DDF+EDF, squarefree deg 8..96, b 2..24): n={na} smooth={a_s} nonsmooth={a_n} disagreements={a_bad}");
    say!("B (trial division, deg<=24, b 2..10, {b_rep} with repeated factors): n={nb} smooth={b_s} nonsmooth={b_n} disagreements={b_bad}");
    if a_bad + b_bad > 0 || a_s < 20 || a_n < 20 || b_s < 20 || b_n < 20 { controls_failed += 1; }
    let _ = write!(js, " \"smooth_control\": {{\"A_n\":{na},\"A_smooth\":{a_s},\"A_nonsmooth\":{a_n},\"A_disagree\":{a_bad},\"B_n\":{nb},\"B_smooth\":{b_s},\"B_nonsmooth\":{b_n},\"B_repeated\":{b_rep},\"B_disagree\":{b_bad}}},\n");

    // ---- EC over F_2^131
    say!("== F_2^131 and EC addition");
    let m = mpoly();
    let m_irr = is_irreducible(&m);
    say!("modulus irreducible: {m_irr}");
    if !m_irr { controls_failed += 1; }
    let mut rng = Rng(splitmix(master ^ 0xEC));
    let e = Ec { a: one(), b: rand_poly(&mut rng, 130, false) };
    let mut ec_bad = 0;
    let pts: Vec<(P, P)> = (0..60).map(|_| ec_rand_point(&e, &mut rng)).collect();
    for p in &pts { if !ec_on(&e, p) { ec_bad += 1; } }
    let mut assoc = 0;
    for i in 0..50 {
        let (a, b, c) = (&pts[i], &pts[i + 1], &pts[(i + 2) % 60]);
        let l = ec_add(&e, a, b).and_then(|ab| ec_add(&e, &ab, c));
        let r = ec_add(&e, b, c).and_then(|bc| ec_add(&e, a, &bc));
        match (l, r) { (Some(l), Some(r)) => { assoc += 1; if l != r || !ec_on(&e, &l) { ec_bad += 1; } } _ => {} }
    }
    say!("EC: points on curve + associativity: triples checked={assoc}, failures={ec_bad}");
    if ec_bad > 0 || assoc < 40 { controls_failed += 1; }
    // costs
    let nrep = 200;
    let (mut wm, mut ws, mut wa, mut wi, mut wadd) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ecns = 0u128;
    for i in 0..nrep {
        let (a, b) = (&pts[i % 59], &pts[i % 59 + 1]);
        let w0 = ctr().0; let _ = fmul(&a.0, &b.0); let w1 = ctr().0; wm += w1 - w0;
        let _ = fsqr(&a.0); let w2 = ctr().0; ws += w2 - w1;
        let _ = add_raw(&a.0, &b.0); let w3 = ctr().0; wa += w3 - w2;
        let _ = finv(&add_raw(&a.0, &b.0)); let w4 = ctr().0; wi += w4 - w3 - (w3 - w2);
        let t0 = Instant::now();
        let _ = ec_add(&e, a, b); ecns += t0.elapsed().as_nanos();
        let w5 = ctr().0; wadd += w5 - w4;
    }
    let (fm, fs, fa, fi, eca) = (wm as f64 / nrep as f64, ws as f64 / nrep as f64, wa as f64 / nrep as f64, wi as f64 / nrep as f64, wadd as f64 / nrep as f64);
    let eld = 8.0 * fm + 5.0 * fs + 9.0 * fa;
    say!("W per op on F_2^131: mul={fm:.0} sqr={fs:.0} add={fa:.0} inv={fi:.0}; affine EC add={eca:.0}; LD mixed 8M+5S+9A={eld:.0}; affine add wall {} ns", ecns / nrep as u128);
    let ec_cheap = eca.min(eld);
    let _ = write!(js, " \"ec\": {{\"modulus_irreducible\":{m_irr},\"onc_assoc_failures\":{ec_bad},\"assoc_triples\":{assoc},\"w_mul\":{fm:.1},\"w_sqr\":{fs:.1},\"w_add\":{fa:.1},\"w_inv\":{fi:.1},\"w_affine_add\":{eca:.1},\"w_ld_mixed_model\":{eld:.1},\"w_cheap\":{ec_cheap:.1},\"affine_add_ns\":{}}},\n", ecns / nrep as u128);

    // ---- main sweep
    let genera = [7usize, 15, 31, 63, 127, 255, 511];
    let (blocks, per_block) = (8usize, 500usize);
    let mut rows: Vec<(usize, usize, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)> = vec![];
    let mut extra: Vec<(usize, f64, f64, f64, f64)> = vec![];
    let _ = write!(js, " \"genera\": [\n");
    for (gi, &g) in genera.iter().enumerate() {
        let seed = splitmix(master ^ g as u64);
        let mut rng = Rng(seed);
        let curve = make_curve(g, &mut rng);
        let bcell = best_cell(&price_all(g, 200));
        let b = bcell.b;
        // 5.1/5.2 Cantor controls
        let mut bad = 0usize;
        let ctab: Vec<Div> = (0..16).map(|_| rand_divisor(&curve, &mut rng)).collect();
        for d in &ctab { if !on_curve(&curve, d) { bad += 1; } }
        let mut sums = 0usize;
        for i in 0..200 {
            let (a, bb) = (&ctab[i % 16], &ctab[(i * 7 + 3) % 16]);
            let s = cantor_add(&curve, a, bb);
            if !on_curve(&curve, &s) { bad += 1; }
            sums += 1;
        }
        let mut gl = 0usize;
        for i in 0..100 {
            let (a, bb, cc) = (&ctab[i % 16], &ctab[(i * 5 + 1) % 16], &rand_divisor(&curve, &mut rng));
            let l = cantor_add(&curve, &cantor_add(&curve, a, bb), cc);
            let r = cantor_add(&curve, a, &cantor_add(&curve, bb, cc));
            if l != r { bad += 1; }
            if cantor_add(&curve, a, bb) != cantor_add(&curve, bb, a) { bad += 1; }
            if cantor_add(&curve, a, &identity()) != *a { bad += 1; }
            if !is_id(&cantor_add(&curve, a, &negate(&curve, a))) { bad += 1; }
            // (A+A)+B = A+(A+B)
            let aa = cantor_add(&curve, a, a);
            if cantor_add(&curve, &aa, bb) != cantor_add(&curve, a, &cantor_add(&curve, a, bb)) { bad += 1; }
            if !on_curve(&curve, &l) { bad += 1; }
            gl += 1;
        }
        say!("g={g}: model b={b}; Cantor controls: outputs={sums}+16 group-law triples={gl} failures={bad}");
        if bad > 0 { controls_failed += 1; }
        // measurement
        let mut wk = make_walk(&curve, splitmix(seed ^ 1));
        // warm-up
        for _ in 0..20 { let _ = run_trial(&curve, &mut wk, b); }
        let (mut va, mut vt, mut vw, mut vev, mut vns, mut vdeg) = (vec![], vec![], vec![], vec![], vec![], vec![]);
        let (mut vc, mut vec_, mut veco) = (vec![], vec![], vec![]);
        let mut hits = 0usize;
        let red0 = RED.load(Relaxed);
        for _ in 0..blocks {
            let (mut sa, mut st, mut se, mut sn, mut sd, mut sc, mut sea, mut sec) = (0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64, 0f64);
            for _ in 0..per_block {
                let t = run_trial(&curve, &mut wk, b);
                sa += t.w_add as f64; st += t.w_test as f64; se += t.ev as f64; sn += t.ns as f64; sd += t.degu as f64; sc += t.w_comp as f64; sea += t.e_add as f64; sec += t.e_comp as f64;
                if t.smooth { hits += 1; }
            }
            let n = per_block as f64;
            va.push(sa / n); vt.push(st / n); vw.push((sa + st) / n); vev.push(se / n); vns.push(sn / n); vdeg.push(sd / n); vc.push(sc / n); vec_.push(sea / n); veco.push(sec / n);
        }
        let red = (RED.load(Relaxed) - red0) as f64 / (blocks * per_block) as f64;
        let (ma, _) = mean_sd(&va); let (mt, _) = mean_sd(&vt); let (mw, sw) = mean_sd(&vw);
        let (mcomp, _) = mean_sd(&vc); let (m_eadd, _) = mean_sd(&vec_); let (m_ecomp, _) = mean_sd(&veco);
        extra.push((g, mcomp, m_eadd, m_ecomp, me_test(&vev, m_eadd)));
        let (me, _) = mean_sd(&vev); let (mn, _) = mean_sd(&vns); let (md, _) = mean_sd(&vdeg);
        say!("g={g} b={b}: W/trial={mw:.0} (add {ma:.0} [compose {mcomp:.0}] + test {mt:.0}) sd_blocks={sw:.0} events={me:.1} ns={mn:.0} mean deg u={md:.2} reduction steps/trial={red:.2} smooth hits={hits}/{}", blocks * per_block);
        rows.push((g, b, ma, mt, mw, sw, me, mn, md, hits as f64, bcell.rel, bcell.lin));
        let _ = write!(js, "  {{\"g\":{g},\"b\":{b},\"trials\":{},\"w_add\":{ma:.1},\"w_test\":{mt:.1},\"w_trial\":{mw:.1},\"w_trial_block_sd\":{sw:.1},\"events\":{me:.2},\"ns_trial\":{mn:.0},\"w_compose\":{mcomp:.1},\"events_add\":{m_eadd:.2},\"events_compose\":{m_ecomp:.2},\"mean_deg_u\":{md:.3},\"reduction_steps\":{red:.3},\"smooth_hits\":{hits},\"cantor_failures\":{bad},\"log2_P_model\":{:.4},\"rel\":{:.4},\"lin\":{:.4},\"log2_fb\":{:.4}}}{}\n",
            blocks * per_block, bcell.lprob, bcell.rel, bcell.lin, bcell.lfb, if gi + 1 < genera.len() { "," } else { "" });
    }
    let _ = write!(js, " ],\n");

    // ---- smooth-rate fidelity
    say!("== smooth-rate fidelity (walk vs model vs random monic polynomials)");
    let _ = write!(js, " \"smooth_rate\": [\n");
    let sr = [7usize, 11, 15, 23, 31];
    for (i, &g) in sr.iter().enumerate() {
        let mut rng = Rng(splitmix(master ^ 0x5000 ^ g as u64));
        let curve = make_curve(g, &mut rng);
        let bc = best_cell(&price_all(g, 200)); let b = bc.b;
        let mut wk = make_walk(&curve, splitmix(master ^ 0x77 ^ g as u64));
        let n = 200_000usize;
        let (mut hw, mut dg) = (0usize, 0usize);
        for _ in 0..n { let t = run_trial(&curve, &mut wk, b); if t.smooth { hw += 1; } if t.degu == g as i64 { dg += 1; } }
        let mut hr = 0usize;
        for _ in 0..n { let f = rand_poly(&mut rng, g, true); if is_smooth(&f, b) { hr += 1; } }
        let (pw, pr, pm) = (hw as f64 / n as f64, hr as f64 / n as f64, 2f64.powf(bc.lprob));
        say!("g={g} b={b}: walk {pw:.5} ({hw}), random monic {pr:.5} ({hr}), model {pm:.5}; walk deg u == g in {:.3}", dg as f64 / n as f64);
        let _ = write!(js, "  {{\"g\":{g},\"b\":{b},\"n\":{n},\"walk_rate\":{pw:.6},\"random_poly_rate\":{pr:.6},\"model_rate\":{pm:.6},\"walk_degu_eq_g\":{:.4}}}{}\n", dg as f64 / n as f64, if i + 1 < sr.len() { "," } else { "" });
    }
    let _ = write!(js, " ],\n");

    // ---- fit
    let sel = |lo: usize| -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        let r: Vec<_> = rows.iter().filter(|r| r.0 >= lo).collect();
        (r.iter().map(|x| (x.0 as f64).log2()).collect(), r.iter().map(|x| x.4.log2()).collect(), r.iter().map(|x| x.2.log2()).collect(), r.iter().map(|x| x.3.log2()).collect())
    };
    let (x63, y63, ya63, yt63) = sel(63); let (x15, y15, ya15, yt15) = sel(15);
    let (a63, e63) = fit(&x63, &y63); let (a15, e15) = fit(&x15, &y15);
    let (aa63, ea63) = fit(&x63, &ya63); let (at63, et63) = fit(&x63, &yt63);
    let (aa15, ea15) = fit(&x15, &ya15); let (at15, et15) = fit(&x15, &yt15);
    // fit excluding 255 to check extrapolation to 255
    let r_ex: Vec<_> = rows.iter().filter(|r| r.0 >= 31 && r.0 != 255 && r.0 != 511).collect();
    let (a_ex, e_ex) = fit(&r_ex.iter().map(|x| (x.0 as f64).log2()).collect::<Vec<_>>(), &r_ex.iter().map(|x| x.4.log2()).collect::<Vec<_>>());
    let pred255 = a_ex + e_ex * 255f64.log2();
    let meas255 = rows.iter().find(|r| r.0 == 255).unwrap().4.log2();
    say!("fit g>=63 total: log2 W = {a63:.3} + {e63:.3} log2 g; g>=15: {a15:.3} + {e15:.3} log2 g");
    say!("fit g>=63 cantor slope {ea63:.3} test slope {et63:.3}; g>=15 cantor {ea15:.3} test {et15:.3}");
    say!("fit on g=31,63,127 only predicts g=255: {pred255:.3} bits vs measured {meas255:.3}");
    let _ = write!(js, " \"fit\": {{\"g63_total\":[{a63:.4},{e63:.4}],\"g15_total\":[{a15:.4},{e15:.4}],\"g63_cantor\":[{aa63:.4},{ea63:.4}],\"g63_test\":[{at63:.4},{et63:.4}],\"g15_cantor\":[{aa15:.4},{ea15:.4}],\"g15_test\":[{at15:.4},{et15:.4}],\"pred255_from_31_63_127\":{pred255:.4},\"meas255\":{meas255:.4}}},\n");

    // ---- b scan at g=255
    say!("== b scan at g=255");
    let g = 255usize;
    let seed = splitmix(master ^ g as u64);
    let mut rng = Rng(seed);
    let curve = make_curve(g, &mut rng);
    let cells = price_all(g, 200);
    let base = best_cell(&cells);
    let rho = log2_rho(130.0);
    let w255 = rows.iter().find(|r| r.0 == 255).unwrap();
    let (w_add255, ev255) = (w255.2, w255.6);
    let _ = write!(js, " \"bscan255\": [\n");
    let blist = [8usize, 12, 16, 20, 24, 27, 32, 36, 40, 48, 56, 64];
    let mut best_scan: Option<(usize, f64, f64)> = None;
    for (i, &b) in blist.iter().enumerate() {
        let mut wk = make_walk(&curve, splitmix(seed ^ 1));
        let n = 300usize;
        let (mut st, mut se) = (0f64, 0f64);
        for _ in 0..n { let t = run_trial(&curve, &mut wk, b); st += t.w_test as f64; se += t.ev as f64; }
        let (wt, evb) = (st / n as f64, se / n as f64);
        let cell = cells.iter().find(|c| c.b == b).unwrap();
        let wtrial = w_add255 + wt;
        let cae = wtrial / ec_cheap;
        let tot = (cell.rel + cae.log2()).max(cell.lin) + 1.0;
        say!("b={b}: test W={wt:.0} trial W={wtrial:.0} CAE(cheap)={cae:.1} (2^{:.2}) rel={:.2} lin={:.2} total={tot:.2} margin={:.2}", cae.log2(), cell.rel, cell.lin, rho - tot);
        let _ = write!(js, "  {{\"b\":{b},\"w_test\":{wt:.1},\"w_trial\":{wtrial:.1},\"events\":{evb:.1},\"cae_cheap\":{cae:.3},\"rel\":{:.4},\"lin\":{:.4},\"total\":{tot:.4},\"margin\":{:.4}}}{}\n", cell.rel, cell.lin, rho - tot, if i + 1 < blist.len() { "," } else { "" });
        if best_scan.map_or(true, |s| tot < s.1) { best_scan = Some((b, tot, cae.log2())); }
        let _ = ev255;
    }
    let _ = write!(js, " ],\n");
    let bs = best_scan.unwrap();

    // ---- margin
    say!("== N=130 margin");
    let w_trial255 = w255.4; let n_prim = w255.6;
    let cae_cheap = w_trial255 / ec_cheap; let cae_aff = w_trial255 / eca;
    let tot_of = |c: f64| (base.rel + c).max(base.lin) + 1.0;
    let note_tot = tot_of(7.0);
    say!("model g=255: b={} rel={:.3} lin={:.3} note total={:.3} margin={:.3}; rho={rho:.3}", base.b, base.rel, base.lin, base.tot, rho - base.tot);
    say!("note's 2^7: total={note_tot:.3} margin={:.3}", rho - note_tot);
    say!("measured trial W={w_trial255:.0}; CAE vs cheap EC add ({ec_cheap:.0}) = {cae_cheap:.2} = 2^{:.3}; vs affine ({eca:.0}) = {cae_aff:.2} = 2^{:.3}", cae_cheap.log2(), cae_aff.log2());
    let mc = rho - tot_of(cae_cheap.log2()); let ma = rho - tot_of(cae_aff.log2());
    say!("calibrated margin (headline, cheap EC add): {mc:.3}; affine: {ma:.3}; re-optimised b={} margin {:.3}", bs.0, rho - bs.1);
    let _ = write!(js, " \"margin\": {{\"rho\":{rho:.4},\"b\":{},\"rel\":{:.4},\"lin\":{:.4},\"model_total\":{:.4},\"model_margin\":{:.4},\"note_2e7_total\":{note_tot:.4},\"note_2e7_margin\":{:.4},\"w_trial255\":{w_trial255:.1},\"n_prim255\":{n_prim:.2},\"cae_cheap\":{cae_cheap:.3},\"cae_affine\":{cae_aff:.3},\"log2_cae_cheap\":{:.4},\"log2_cae_affine\":{:.4},\"margin_cheap\":{mc:.4},\"margin_affine\":{ma:.4},\"reopt_b\":{},\"reopt_total\":{:.4},\"reopt_margin\":{:.4},\"reopt_log2_cae\":{:.4},\n", base.b, base.rel, base.lin, base.tot, rho - base.tot, rho - note_tot, cae_cheap.log2(), cae_aff.log2(), bs.0, bs.1, rho - bs.1, bs.2);
    // scenarios: A full Cantor (measured), B composition only + test (lower bound for any Cantor-type step), C smoothness test only (floor)
    let ex = extra.iter().find(|x| x.0 == 255).unwrap();
    let (w_comp255, e_add255, e_comp255, e_test255) = (ex.1, ex.2, ex.3, ex.4);
    let w_test255 = w255.3;
    let scen = [("A_full_cantor_plus_test", w_trial255, n_prim), ("B_compose_only_plus_test", w_comp255 + w_test255, e_comp255 + e_test255), ("C_test_only_floor", w_test255, e_test255)];
    let _ = write!(js, "  \"scenarios\": [\n");
    for (i, (nm, wt, ne)) in scen.iter().enumerate() {
        let cae = wt / ec_cheap; let lc = cae.log2();
        let tot = tot_of(lc);
        say!("scenario {nm}: W={wt:.0} events={ne:.1} CAE=2^{lc:.3} total=2^{tot:.3} margin={:.3}", rho - tot);
        let _ = write!(js, "   {{\"name\":\"{nm}\",\"w\":{wt:.1},\"events\":{ne:.2},\"log2_cae\":{lc:.4},\"total\":{tot:.4},\"margin\":{:.4}}}{}\n", rho - tot, if i < 2 { "," } else { "" });
    }
    let _ = write!(js, "  ],\n");
    let _ = (e_add255,);
    // band
    let _ = write!(js, "  \"band\": [\n");
    let ws = [0.0f64, 0.1, 1.0];
    let nrhos = [0.0f64, 3.0, 10.0];
    let mut first = true;
    for &(snm, _, ne) in &scen { for &w in &ws {
        for &nr in &nrhos {
            let t = 1.0 + w * ne; let r = 1.0 + w * nr;
            let tot = (base.rel + t.log2()).max(base.lin) + 1.0;
            let mg = rho + r.log2() - tot;
            say!("band {snm} w={w} n_rho={nr}: IC trial = {t:.1} GAE, rho step = {r:.2} GAE, IC total 2^{tot:.2}, rho 2^{:.2}, margin {mg:.2}", rho + r.log2());
            let _ = write!(js, "{}   {{\"scenario\":\"{snm}\",\"w\":{w},\"n_rho\":{nr},\"ic_trial_gae\":{t:.2},\"rho_step_gae\":{r:.2},\"ic_total\":{tot:.4},\"rho\":{:.4},\"margin\":{mg:.4}}}", if first { "" } else { ",\n" }, rho + r.log2());
            first = false;
        }
    } }
    let _ = write!(js, "\n  ]}},\n \"controls_failed\": {controls_failed}\n}}\n");
    std::fs::write(format!("{out_dir}/results.json"), &js).unwrap();
    std::fs::write(format!("{out_dir}/run.log"), &log).unwrap();

    // ---- SVG
    let svg = plot(&rows, ec_cheap, a63, e63);
    std::fs::write(format!("{out_dir}/cost_per_trial_vs_genus.svg"), svg).unwrap();
    say!("controls_failed = {controls_failed}");
    if controls_failed > 0 { std::process::exit(2); }
}

fn plot(rows: &[(usize, usize, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64)], ec: f64, a: f64, e: f64) -> String {
    let (w, h, l, r, t, bm) = (900.0, 560.0, 90.0, 40.0, 70.0, 80.0);
    let (xmin, xmax, ymin, ymax) = (2.5f64, 9.5f64, 5.0f64, 22.0f64);
    let px = |x: f64| l + (x - xmin) / (xmax - xmin) * (w - l - r);
    let py = |y: f64| h - bm - (y - ymin) / (ymax - ymin) * (h - t - bm);
    let mut s = String::new();
    let _ = write!(s, "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w} {h}\" width=\"{w}\" height=\"{h}\" font-family=\"DejaVu Sans, Arial, sans-serif\" font-size=\"13\">\n<rect width=\"{w}\" height=\"{h}\" fill=\"#ffffff\"/>\n");
    let _ = write!(s, "<text x=\"{}\" y=\"26\" font-size=\"16\" font-weight=\"bold\" text-anchor=\"middle\">Cost of one relation trial vs genus (binary hyperelliptic, F_2)</text>\n", w / 2.0);
    let _ = write!(s, "<text x=\"{}\" y=\"46\" text-anchor=\"middle\" fill=\"#444\">measured: 4000 walk trials per genus (8 blocks of 500), b = model optimum; unit W = F_2[x] u64 word ops</text>\n", w / 2.0);
    for yy in (ymin as i32)..=(ymax as i32) {
        let y = py(yy as f64);
        let _ = write!(s, "<line x1=\"{l}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"#e3e3e3\"/><text x=\"{}\" y=\"{}\" text-anchor=\"end\" fill=\"#333\">2^{yy}</text>\n", w - r, l - 8.0, y + 4.0);
    }
    for &g in &[7usize, 15, 31, 63, 127, 255, 511] {
        let x = px((g as f64).log2());
        let _ = write!(s, "<line x1=\"{x}\" y1=\"{t}\" x2=\"{x}\" y2=\"{}\" stroke=\"#e3e3e3\"/><text x=\"{x}\" y=\"{}\" text-anchor=\"middle\" fill=\"#333\">{g}</text>\n", h - bm, h - bm + 18.0);
    }
    let _ = write!(s, "<rect x=\"{l}\" y=\"{t}\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"#666\"/>\n", w - l - r, h - t - bm);
    let _ = write!(s, "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\">genus g (log scale)</text>\n", (l + w - r) / 2.0, h - 30.0);
    let _ = write!(s, "<text transform=\"translate(22,{}) rotate(-90)\" text-anchor=\"middle\">word operations W per trial (log2 scale)</text>\n", (t + h - bm) / 2.0);
    // reference lines
    let lec = ec.log2();
    for (yy, lab, col) in [(lec, "1 curve addition on F_2^131 (affine, measured in the same unit)", "#6a6a6a"), (lec + 7.0, "the note's guess: 2^7 curve additions", "#b03030")] {
        let y = py(yy);
        let _ = write!(s, "<line x1=\"{l}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{col}\" stroke-dasharray=\"6 4\"/><text x=\"{}\" y=\"{}\" fill=\"{col}\" font-size=\"12\" text-anchor=\"end\">{lab}</text>\n", w - r, w - r - 8.0, y - 5.0);
    }
    // fit line
    let (x0, x1) = (5.9f64, 9.3f64);
    let _ = write!(s, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#1f5fa8\" stroke-dasharray=\"2 3\" stroke-width=\"2\"/>\n", px(x0), py(a + e * x0), px(x1), py(a + e * x1));
    let series: [(&str, &str, usize); 3] = [("total trial (Cantor add + smoothness test)", "#1f5fa8", 4), ("smoothness test only", "#d07a00", 3), ("Cantor addition only", "#2a8a4a", 2)];
    for (_, col, idx) in &series {
        let mut pts = String::new();
        for r in rows {
            let v = match idx { 4 => r.4, 3 => r.3, _ => r.2 };
            let _ = write!(pts, "{},{} ", px((r.0 as f64).log2()), py(v.log2()));
        }
        let _ = write!(s, "<polyline points=\"{pts}\" fill=\"none\" stroke=\"{col}\" stroke-width=\"2\"/>\n");
        for r in rows {
            let v = match idx { 4 => r.4, 3 => r.3, _ => r.2 };
            let _ = write!(s, "<circle cx=\"{}\" cy=\"{}\" r=\"4.5\" fill=\"{col}\"/>\n", px((r.0 as f64).log2()), py(v.log2()));
        }
    }
    let last = rows.iter().find(|r| r.0 == 255).unwrap();
    let _ = write!(s, "<text x=\"{}\" y=\"{}\" font-size=\"12\" text-anchor=\"start\">g=255: 2^{:.2} W (b={})</text>\n", px(8.0) + 12.0, py(last.4.log2()) + 34.0, last.4.log2(), last.1);
    // legend
    let mut ly = t + 18.0;
    for (lab, col, _) in &series {
        let _ = write!(s, "<rect x=\"{}\" y=\"{}\" width=\"14\" height=\"4\" fill=\"{col}\"/><text x=\"{}\" y=\"{}\">{lab}</text>\n", l + 14.0, ly, l + 34.0, ly + 6.0);
        ly += 22.0;
    }
    let _ = write!(s, "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#1f5fa8\" stroke-dasharray=\"2 3\" stroke-width=\"2\"/><text x=\"{}\" y=\"{}\">fit (g&gt;=63): W = 2^{:.2} g^{:.2}</text>\n", l + 14.0, ly + 2.0, l + 28.0, ly + 2.0, l + 34.0, ly + 6.0, a, e);
    s.push_str("</svg>\n");
    s
}
