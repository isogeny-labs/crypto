//! Matched multi-thread Pollard rho (lead: matched rho reference).
//!
//! Parallel distinguished-point rho (van Oorschot-Wiener) on E(K4)[r] with an
//! r-adding walk shared by every thread, optional batched affine walks with
//! Montgomery's simultaneous inversion, optional negation map with fruitless
//! cycle detection and escape-by-doubling, and optional fast (u64 Barrett)
//! K4 arithmetic that returns exactly the same field elements as K4::mul/inv.
//!
//! Activated only by HUMBERT_RHO_THREADS / _BATCH / _NEG / _FASTMUL / _DPBITS / _NB;
//! with all of them unset the crate's scalar `curve::pollard_rho` runs as before.
//! Every walk step and every other group operation is counted.
use crate::curve::{mulr, invr, Curve, Pt, Rng};
use crate::field::{FieldOps, E4, K4};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct ParCfg {
    pub threads: usize,
    pub batch: usize,
    pub neg: bool,
    pub fast: bool,
    /// distinguished-point bits; None = automatic
    pub dp_bits: Option<u32>,
    /// r-adding table size (power of two); None = 256 (2048 with negation)
    pub nb: Option<usize>,
    /// total walk-step cap
    pub cap: u64,
}

impl ParCfg {
    /// None when no rho flag is set (scalar rho, unchanged output).
    pub fn from_env() -> Option<ParCfg> {
        let g = |k: &str| std::env::var(k).ok().filter(|v| !v.is_empty());
        let threads = g("HUMBERT_RHO_THREADS");
        let batch = g("HUMBERT_RHO_BATCH");
        let neg = g("HUMBERT_RHO_NEG").map(|v| v == "1").unwrap_or(false);
        let fast = g("HUMBERT_RHO_FASTMUL").map(|v| v == "1").unwrap_or(false);
        let dp = g("HUMBERT_RHO_DPBITS");
        let nb = g("HUMBERT_RHO_NB");
        if threads.is_none() && batch.is_none() && !neg && !fast && dp.is_none() && nb.is_none() {
            return None;
        }
        let nb = nb.map(|v| v.parse::<usize>().expect("HUMBERT_RHO_NB"));
        if let Some(n) = nb {
            assert!(n.is_power_of_two() && n >= 2, "HUMBERT_RHO_NB must be a power of two");
        }
        Some(ParCfg {
            threads: threads.map(|v| v.parse().expect("HUMBERT_RHO_THREADS")).unwrap_or(1).max(1),
            batch: batch.map(|v| v.parse().expect("HUMBERT_RHO_BATCH")).unwrap_or(1).max(1),
            neg,
            fast,
            dp_bits: dp.map(|v| v.parse().expect("HUMBERT_RHO_DPBITS")),
            nb,
            cap: 2_000_000_000,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParResult {
    pub log: Option<u64>,
    /// walk steps (one point addition each), all threads; includes steps spent
    /// re-walking a detected fruitless cycle
    pub steps: u64,
    pub seconds: f64,
    /// all K4 multiplications (walks + table + starts + escapes + verification), 3 per inversion
    pub k4_muls: u64,
    pub k4_muls_walk: u64,
    /// K4 inversions in the walks (one per batch iteration)
    pub walk_inversions: u64,
    /// group operations outside the walk steps (table, start points, restarts, doublings, verifications)
    pub extra_group_ops: u64,
    pub dp_bits: u32,
    pub nb: usize,
    pub dps: u64,
    pub useless_collisions: u64,
    pub abandoned_walks: u64,
    pub fruitless_cycles: u64,
    pub fruitless_cycle_steps: u64,
    pub degenerate: u64,
    pub steps_per_thread: Vec<u64>,
    pub seconds_per_thread: Vec<f64>,
}

#[inline]
fn pk(x: &E4) -> u64 {
    x[0] | (x[1] << 16) | (x[2] << 32) | (x[3] << 48)
}
#[inline]
fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

/// K4 arithmetic with its own counters. `fast`: u64 products + Barrett reduction
/// (needs p < 2^15); otherwise the crate's K4::mul / K4::inv.
pub struct Ar<'a> {
    pub k: &'a K4,
    p: u64,
    bm: u64,
    fast: bool,
    pub muls: u64,
    pub invs: u64,
}

impl<'a> Ar<'a> {
    pub fn new(k: &'a K4, fast: bool) -> Ar<'a> {
        let fast = fast && k.p < (1 << 15);
        Ar { k, p: k.p, bm: u64::MAX / k.p, fast, muls: 0, invs: 0 }
    }
    #[inline(always)]
    fn red(&self, n: u64) -> u64 {
        // n < 2^34: q >= floor(n/p) - 1
        let q = ((n as u128 * self.bm as u128) >> 64) as u64;
        let r = n - q * self.p;
        if r >= self.p {
            r - self.p
        } else {
            r
        }
    }
    #[inline(always)]
    fn mulf(&self, a: &E4, b: &E4) -> E4 {
        let mut c = [0u64; 7];
        for i in 0..4 {
            for j in 0..4 {
                c[i + j] += a[i] * b[j];
            }
        }
        let mut r = [c[0], c[1], c[2], c[3]];
        for h in 0..3 {
            let ch = self.red(c[4 + h]);
            let rh = &self.k.red[h];
            for i in 0..4 {
                r[i] += ch * rh[i];
            }
        }
        [self.red(r[0]), self.red(r[1]), self.red(r[2]), self.red(r[3])]
    }
    #[inline(always)]
    fn frobf(&self, a: &E4) -> E4 {
        let mut r = [0u64; 4];
        for j in 0..4 {
            let ft = &self.k.frob_t[j];
            for i in 0..4 {
                r[i] += a[j] * ft[i];
            }
        }
        [self.red(r[0]), self.red(r[1]), self.red(r[2]), self.red(r[3])]
    }
    fn inv_fp(&self, a: u64) -> u64 {
        // extended Euclid
        let (mut t, mut nt) = (0i64, 1i64);
        let (mut rr, mut nr) = (self.p as i64, a as i64);
        while nr != 0 {
            let qq = rr / nr;
            (t, nt) = (nt, t - qq * nt);
            (rr, nr) = (nr, rr - qq * nr);
        }
        assert!(rr == 1);
        if t < 0 {
            (t + self.p as i64) as u64
        } else {
            t as u64
        }
    }
    #[inline(always)]
    pub fn mul(&mut self, a: &E4, b: &E4) -> E4 {
        self.muls += 1;
        if self.fast {
            self.mulf(a, b)
        } else {
            self.k.mul(a, b)
        }
    }
    pub fn inv(&mut self, a: &E4) -> E4 {
        self.muls += 3;
        self.invs += 1;
        if self.fast {
            assert!(*a != [0; 4], "K4 inverse of zero");
            let f1 = self.frobf(a);
            let f2 = self.frobf(&f1);
            let f3 = self.frobf(&f2);
            let c = self.mulf(&self.mulf(&f1, &f2), &f3);
            let n = self.mulf(a, &c)[0];
            let ni = self.inv_fp(n);
            [self.red(c[0] * ni), self.red(c[1] * ni), self.red(c[2] * ni), self.red(c[3] * ni)]
        } else {
            self.k.inv(a)
        }
    }
    #[inline(always)]
    pub fn add(&self, a: &E4, b: &E4) -> E4 {
        self.k.add(a, b)
    }
    #[inline(always)]
    pub fn sub(&self, a: &E4, b: &E4) -> E4 {
        self.k.sub(a, b)
    }
    #[inline(always)]
    pub fn neg(&self, a: &E4) -> E4 {
        self.k.neg(a)
    }
}

/// Affine group law over Ar (counted); same formulas as Curve::add.
struct G<'a, 'b> {
    ar: &'b mut Ar<'a>,
    a2: E4,
    a4: E4,
    ops: u64,
}
impl<'a, 'b> G<'a, 'b> {
    fn add(&mut self, p: &Pt<E4>, q: &Pt<E4>) -> Pt<E4> {
        self.ops += 1;
        let (x1, y1) = match p {
            None => return *q,
            Some(v) => *v,
        };
        let (x2, y2) = match q {
            None => return *p,
            Some(v) => *v,
        };
        let lam = if x1 == x2 {
            if self.ar.add(&y1, &y2) == [0; 4] {
                return None;
            }
            let x1s = self.ar.mul(&x1, &x1);
            let t2 = self.ar.mul(&self.a2, &x1);
            let num = self.ar.add(&self.ar.add(&self.ar.add(&x1s, &x1s), &x1s), &self.ar.add(&self.ar.add(&t2, &t2), &self.a4));
            let yy = self.ar.add(&y1, &y1);
            let den = self.ar.inv(&yy);
            self.ar.mul(&num, &den)
        } else {
            let dxx = self.ar.sub(&x2, &x1);
            let den = self.ar.inv(&dxx);
            let dyy = self.ar.sub(&y2, &y1);
            self.ar.mul(&dyy, &den)
        };
        let l2 = self.ar.mul(&lam, &lam);
        let x3 = self.ar.sub(&self.ar.sub(&self.ar.sub(&l2, &self.a2), &x1), &x2);
        let t = self.ar.sub(&x1, &x3);
        let ly = self.ar.mul(&lam, &t);
        let y3 = self.ar.sub(&ly, &y1);
        Some((x3, y3))
    }
    fn mul(&mut self, p: &Pt<E4>, k: u64) -> Pt<E4> {
        let mut r: Pt<E4> = None;
        let mut b = *p;
        let mut k = k;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &b);
            }
            k >>= 1;
            if k > 0 {
                b = self.add(&b, &b);
            }
        }
        r
    }
}

/// canonical representative of {P, -P}: the y with the smaller packed value.
#[inline(always)]
fn canon(ar: &Ar, y: &E4) -> (E4, bool) {
    let ny = ar.neg(y);
    if pk(&ny) < pk(y) {
        (ny, true)
    } else {
        (*y, false)
    }
}

struct Shared {
    done: AtomicBool,
    table: Mutex<HashMap<u64, (u64, u64, u64)>>,
    result: Mutex<Option<u64>>,
}

#[derive(Default)]
struct ThreadStats {
    steps: u64,
    secs: f64,
    muls_walk: u64,
    muls_other: u64,
    invs_walk: u64,
    extra_ops: u64,
    dps: u64,
    useless: u64,
    abandoned: u64,
    fruitless: u64,
    fruitless_steps: u64,
    degenerate: u64,
}

fn dp_bits_auto(r: u64, w: usize, neg: bool) -> u32 {
    let mut n = (std::f64::consts::PI * r as f64 / 2.0).sqrt();
    if neg {
        n /= std::f64::consts::SQRT_2;
    }
    // expected DP overhead ~ W * 2^d steps: keep it near 1/64 of the expected work
    let target = n / (w as f64 * 64.0);
    let d = if target <= 1.0 { 0 } else { target.log2().floor() as u32 };
    d.clamp(2, 26)
}

/// Parallel DP rho for x with q = x * gen, gen of prime order r.
pub fn rho_par(c: &Curve<K4>, gen: &Pt<E4>, q: &Pt<E4>, r: u64, rng: &mut Rng, cfg: &ParCfg) -> ParResult {
    let t0 = std::time::Instant::now();
    let nb = cfg.nb.unwrap_or(if cfg.neg { 2048 } else { 256 });
    let w = cfg.threads * cfg.batch;
    let dp_bits = cfg.dp_bits.unwrap_or_else(|| dp_bits_auto(r, w, cfg.neg));
    let dp_mask: u64 = (1u64 << dp_bits) - 1;
    let maxlen: u64 = 32u64 << dp_bits;
    // shared r-adding table R_j = ma_j gen + mb_j q (the walk function every thread uses)
    let k0 = c.f.clone();
    let mut ar0 = Ar::new(&k0, cfg.fast);
    let mut tab_x = Vec::with_capacity(nb);
    let mut tab_y = Vec::with_capacity(nb);
    let mut tab_a = Vec::with_capacity(nb);
    let mut tab_b = Vec::with_capacity(nb);
    let table_ops;
    {
        let mut g = G { ar: &mut ar0, a2: c.a2, a4: c.a4, ops: 0 };
        while tab_x.len() < nb {
            let (ma, mb) = (rng.below(r), rng.below(r));
            let (pa, pb) = (g.mul(gen, ma), g.mul(q, mb));
            let pt = g.add(&pa, &pb);
            if let Some((x, y)) = pt {
                tab_x.push(x);
                tab_y.push(y);
                tab_a.push(ma);
                tab_b.push(mb);
            }
        }
        table_ops = g.ops;
    }
    let table_muls = ar0.muls;
    let seeds: Vec<u64> = (0..cfg.threads).map(|_| rng.next()).collect();
    let sh = Shared { done: AtomicBool::new(false), table: Mutex::new(HashMap::new()), result: Mutex::new(None) };
    let cap_per_thread = cfg.cap / cfg.threads as u64;
    let stats: Vec<ThreadStats> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..cfg.threads)
            .map(|ti| {
                let kt = c.f.clone();
                let (a2, a4) = (c.a2, c.a4);
                let (tab_x, tab_y, tab_a, tab_b) = (&tab_x, &tab_y, &tab_a, &tab_b);
                let sh = &sh;
                let seed = seeds[ti];
                let cfg = cfg.clone();
                let (gen, q) = (*gen, *q);
                scope.spawn(move || {
                    walker(&kt, a2, a4, &gen, &q, r, seed, &cfg, nb, dp_mask, maxlen, tab_x, tab_y, tab_a, tab_b, sh, cap_per_thread)
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().expect("rho thread")).collect()
    });
    let log = *sh.result.lock().unwrap();
    let dps = sh.table.lock().unwrap().len() as u64;
    let mut res = ParResult {
        log,
        seconds: t0.elapsed().as_secs_f64(),
        dp_bits,
        nb,
        k4_muls: table_muls,
        extra_group_ops: table_ops,
        ..Default::default()
    };
    let mut dp_seen = 0;
    for s in &stats {
        res.steps += s.steps;
        res.k4_muls += s.muls_walk + s.muls_other;
        res.k4_muls_walk += s.muls_walk;
        res.walk_inversions += s.invs_walk;
        res.extra_group_ops += s.extra_ops;
        dp_seen += s.dps;
        res.useless_collisions += s.useless;
        res.abandoned_walks += s.abandoned;
        res.fruitless_cycles += s.fruitless;
        res.fruitless_cycle_steps += s.fruitless_steps;
        res.degenerate += s.degenerate;
        res.steps_per_thread.push(s.steps);
        res.seconds_per_thread.push(s.secs);
    }
    res.dps = dp_seen.max(dps);
    res
}

#[allow(clippy::too_many_arguments)]
fn walker(
    k: &K4,
    a2: E4,
    a4: E4,
    gen: &Pt<E4>,
    q: &Pt<E4>,
    r: u64,
    seed: u64,
    cfg: &ParCfg,
    nb: usize,
    dp_mask: u64,
    maxlen: u64,
    tab_x: &[E4],
    tab_y: &[E4],
    tab_a: &[u64],
    tab_b: &[u64],
    sh: &Shared,
    cap: u64,
) -> ThreadStats {
    let t0 = std::time::Instant::now();
    let mut st = ThreadStats::default();
    let mut rng = Rng(seed);
    let mut ar = Ar::new(k, cfg.fast);
    let m = cfg.batch;
    let neg = cfg.neg;
    let nbm = (nb - 1) as u64;
    // ---- start-point sequence S_k = S_0 + k T (one group addition per restart) ----
    let mut other_muls = 0u64;
    let (mut s_pt, mut s_a, mut s_b, t_pt, t_a, t_b);
    {
        let m0 = ar.muls;
        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
        loop {
            let (a, b) = (rng.below(r), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                s_pt = p;
                s_a = a;
                s_b = b;
                break;
            }
        }
        loop {
            let (a, b) = (1 + rng.below(r - 1), rng.below(r));
            let (pa, pb) = (g.mul(gen, a), g.mul(q, b));
            let p = g.add(&pa, &pb);
            if p.is_some() {
                t_pt = p;
                t_a = a;
                t_b = b;
                break;
            }
        }
        st.extra_ops += g.ops;
        other_muls += ar.muls - m0;
    }
    // next start (x, y, a, b), canonical under negation
    macro_rules! next_start {
        () => {{
            loop {
                let m0 = ar.muls;
                let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                s_pt = g.add(&s_pt, &t_pt);
                st.extra_ops += g.ops;
                other_muls += ar.muls - m0;
                s_a = (s_a + t_a) % r;
                s_b = (s_b + t_b) % r;
                if let Some((x, y)) = s_pt {
                    let (y, fl) = if neg { canon(&ar, &y) } else { (y, false) };
                    let (a, b) = if fl { ((r - s_a) % r, (r - s_b) % r) } else { (s_a, s_b) };
                    break (x, y, a, b);
                }
            }
        }};
    }
    let mut wx = vec![[0u64; 4]; m];
    let mut wy = vec![[0u64; 4]; m];
    let mut wa = vec![0u64; m];
    let mut wb = vec![0u64; m];
    let mut wlen = vec![0u64; m];
    let mut prev1 = vec![u64::MAX; m]; // packed x one step back (negation only)
    let mut prev2 = vec![u64::MAX; m]; // packed x two steps back
    let mut mark = vec![u64::MAX; m]; // cycle marker (negation only)
    let mut wj = vec![0usize; m];
    let mut dx = vec![[0u64; 4]; m];
    let mut pre = vec![[0u64; 4]; m];
    let mut bad = vec![false; m];
    for i in 0..m {
        let (x, y, a, b) = next_start!();
        wx[i] = x;
        wy[i] = y;
        wa[i] = a;
        wb[i] = b;
    }
    let one: E4 = [1, 0, 0, 0];
    const CYC: u64 = 1024;
    let walk_muls0 = ar.muls;
    let other_at_walk0 = other_muls;
    let mut escape_muls = 0u64;
    let mut iter = 0u64;
    'outer: loop {
        if iter & 63 == 0 && sh.done.load(Ordering::Relaxed) {
            break;
        }
        if st.steps >= cap {
            break;
        }
        iter += 1;
        // ---- one step of every walk: Montgomery's simultaneous inversion ----
        for i in 0..m {
            let h = mix(pk(&wx[i]));
            let j = (h & nbm) as usize;
            wj[i] = j;
            let d = ar.sub(&tab_x[j], &wx[i]);
            if d == [0; 4] {
                // W = +-R_j: never in practice; restart this walk
                bad[i] = true;
                dx[i] = one;
            } else {
                bad[i] = false;
                dx[i] = d;
            }
        }
        pre[0] = dx[0];
        for i in 1..m {
            pre[i] = ar.mul(&pre[i - 1], &dx[i]);
        }
        let mut inv = ar.inv(&pre[m - 1]);
        for i in (0..m).rev() {
            let inv_i = if i > 0 {
                let v = ar.mul(&inv, &pre[i - 1]);
                inv = ar.mul(&inv, &dx[i]);
                v
            } else {
                inv
            };
            if bad[i] {
                st.degenerate += 1;
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
                continue;
            }
            let j = wj[i];
            let (x1, y1) = (wx[i], wy[i]);
            let dy = ar.sub(&tab_y[j], &y1);
            let lam = ar.mul(&dy, &inv_i);
            let l2 = ar.mul(&lam, &lam);
            let x3 = ar.sub(&ar.sub(&ar.sub(&l2, &a2), &x1), &tab_x[j]);
            let t = ar.sub(&x1, &x3);
            let ly = ar.mul(&lam, &t);
            let y3 = ar.sub(&ly, &y1);
            let mut na = wa[i] + tab_a[j];
            if na >= r {
                na -= r;
            }
            let mut nb_ = wb[i] + tab_b[j];
            if nb_ >= r {
                nb_ -= r;
            }
            st.steps += 1;
            wlen[i] += 1;
            if neg {
                let (yc, fl) = canon(&ar, &y3);
                wx[i] = x3;
                wy[i] = yc;
                if fl {
                    wa[i] = if na == 0 { 0 } else { r - na };
                    wb[i] = if nb_ == 0 { 0 } else { r - nb_ };
                } else {
                    wa[i] = na;
                    wb[i] = nb_;
                }
                let px = pk(&x3);
                let cyc_len = if px == prev2[i] {
                    2
                } else if px == mark[i] {
                    // cycle of length <= CYC since the marker
                    (wlen[i] - 1) % CYC + 1
                } else {
                    0
                };
                prev2[i] = prev1[i];
                prev1[i] = px;
                if wlen[i] % CYC == 0 {
                    mark[i] = px;
                }
                if cyc_len > 0 {
                    // fruitless cycle: walk it once (scalar, counted), find the point
                    // with the smallest packed x, and escape to canon(2 * min)
                    st.fruitless += 1;
                    let m0 = ar.muls;
                    let (mut cx, mut cy, mut ca, mut cb) = (wx[i], wy[i], wa[i], wb[i]);
                    let (mut bx, mut by, mut ba, mut bb) = (cx, cy, ca, cb);
                    let mut steps_c = 0u64;
                    loop {
                        let jj = (mix(pk(&cx)) & nbm) as usize;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let nx = g.add(&Some((cx, cy)), &Some((tab_x[jj], tab_y[jj])));
                        steps_c += 1;
                        let (xx, yy) = match nx {
                            Some(v) => v,
                            None => break,
                        };
                        let mut aa = (ca + tab_a[jj]) % r;
                        let mut bbb = (cb + tab_b[jj]) % r;
                        let (yc, fl) = canon(&ar, &yy);
                        if fl {
                            aa = (r - aa) % r;
                            bbb = (r - bbb) % r;
                        }
                        cx = xx;
                        cy = yc;
                        ca = aa;
                        cb = bbb;
                        if pk(&cx) < pk(&bx) {
                            bx = cx;
                            by = cy;
                            ba = ca;
                            bb = cb;
                        }
                        if cx == wx[i] || steps_c > 4 * CYC {
                            break;
                        }
                    }
                    st.steps += steps_c;
                    st.fruitless_steps += steps_c;
                    let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                    let d = g.add(&Some((bx, by)), &Some((bx, by)));
                    st.extra_ops += 1;
                    escape_muls += ar.muls - m0;
                    match d {
                        Some((xx, yy)) => {
                            let (yc, fl) = canon(&ar, &yy);
                            let (aa, bbb) = ((2 * ba) % r, (2 * bb) % r);
                            wx[i] = xx;
                            wy[i] = yc;
                            wa[i] = if fl { (r - aa) % r } else { aa };
                            wb[i] = if fl { (r - bbb) % r } else { bbb };
                        }
                        None => {
                            let (x, y, a, b) = next_start!();
                            wx[i] = x;
                            wy[i] = y;
                            wa[i] = a;
                            wb[i] = b;
                            wlen[i] = 0;
                        }
                    }
                    prev1[i] = u64::MAX;
                    prev2[i] = u64::MAX;
                    mark[i] = pk(&wx[i]);
                }
            } else {
                wx[i] = x3;
                wy[i] = y3;
                wa[i] = na;
                wb[i] = nb_;
            }
            // ---- distinguished point? ----
            let h = mix(pk(&wx[i]) ^ 0xD15C_0DE5_A5A5_0F0F);
            if (h >> 20) & dp_mask == 0 {
                st.dps += 1;
                let key = pk(&wx[i]);
                let (a1, b1, y1p) = (wa[i], wb[i], pk(&wy[i]));
                let prev = {
                    let mut t = sh.table.lock().unwrap();
                    match t.get(&key) {
                        Some(v) => Some(*v),
                        None => {
                            t.insert(key, (a1, b1, y1p));
                            None
                        }
                    }
                };
                if let Some((a2c, b2c, y2p)) = prev {
                    // a1 + b1 x = +-(a2 + b2 x)
                    let cand = if y2p == y1p {
                        let den = (b2c + r - b1) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((a1 + r - a2c) % r, invr(den, r), r))
                        }
                    } else {
                        let den = (b1 + b2c) % r;
                        if den == 0 {
                            None
                        } else {
                            Some(mulr((r - (a1 + a2c) % r) % r, invr(den, r), r))
                        }
                    };
                    let mut ok = false;
                    if let Some(x) = cand {
                        let m0 = ar.muls;
                        let mut g = G { ar: &mut ar, a2, a4, ops: 0 };
                        let chk = g.mul(gen, x);
                        st.extra_ops += g.ops;
                        escape_muls += ar.muls - m0;
                        if chk == *q {
                            ok = true;
                            let mut res = sh.result.lock().unwrap();
                            if res.is_none() {
                                *res = Some(x);
                            }
                            sh.done.store(true, Ordering::Relaxed);
                        }
                    }
                    if !ok {
                        st.useless += 1;
                    }
                    if ok {
                        break 'outer;
                    }
                }
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
            } else if wlen[i] > maxlen {
                st.abandoned += 1;
                let (x, y, a, b) = next_start!();
                wx[i] = x;
                wy[i] = y;
                wa[i] = a;
                wb[i] = b;
                wlen[i] = 0;
                prev1[i] = u64::MAX;
                prev2[i] = u64::MAX;
                mark[i] = u64::MAX;
            }
        }
    }
    // walk muls: the batch steps only (escapes, restarts and verifications excluded)
    st.invs_walk = iter;
    st.muls_walk = (ar.muls - walk_muls0) - escape_muls - (other_muls - other_at_walk0);
    st.muls_other = ar.muls - st.muls_walk;
    st.secs = t0.elapsed().as_secs_f64();
    st
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cover::Setup;
    use crate::curve::random_point;

    fn instance(p: u64, tw: i32, seed: u64) -> (Setup, Pt<E4>, Pt<E4>, u64, u64, Rng) {
        let mut rng = Rng(seed);
        let s = Setup::new(p, tw, &mut rng);
        let r = s.r;
        let (gen, q, x) = {
            let ce = s.curve();
            let cof = s.order / r as u128;
            let sq = |a: &E4| s.sqrt_k4(a);
            let gen = loop {
                let pt = ce.mul(&random_point(&ce, &sq, &mut rng), cof);
                if pt.is_some() {
                    break pt;
                }
            };
            let x = 1 + rng.below(r - 1);
            (gen, ce.mul(&gen, x as u128), x)
        };
        (s, gen, q, r, x, rng)
    }

    #[test]
    fn fast_arith_matches_k4() {
        for &p in &[17u64, 16001, 32003] {
            let mut rng = Rng(p);
            let s = Setup::new(p, 1, &mut rng);
            let k = s.k();
            let mut ar = Ar::new(k, true);
            assert!(ar.fast);
            for _ in 0..2000 {
                let a = [rng.below(p), rng.below(p), rng.below(p), rng.below(p)];
                let b = [p - 1, rng.below(p), p - 1, rng.below(p)];
                assert_eq!(ar.mul(&a, &b), k.mul(&a, &b));
                if a != [0; 4] {
                    assert_eq!(ar.inv(&a), k.inv(&a));
                }
            }
        }
    }

    fn solve(p: u64, tw: i32, seed: u64, cfg: ParCfg) -> (ParResult, u64) {
        let (s, gen, q, r, x, mut rng) = instance(p, tw, seed);
        let ce = s.curve();
        let res = rho_par(&ce, &gen, &q, r, &mut rng, &cfg);
        (res, x)
    }

    fn cfg(threads: usize, batch: usize, neg: bool, fast: bool) -> ParCfg {
        ParCfg { threads, batch, neg, fast, dp_bits: None, nb: None, cap: 2_000_000_000 }
    }

    #[test]
    fn par_rho_solves_small_instances() {
        // toy primes: r ~ p^4 / cofactor, sqrt(r) ~ p^2
        for &(p, seed) in &[(101u64, 1u64), (101, 2), (211, 3), (211, 4), (401, 5)] {
            let tw = if seed % 2 == 1 { 1 } else { -1 };
            for c in [cfg(1, 1, false, false), cfg(1, 1, false, true), cfg(4, 1, false, true), cfg(2, 16, false, true), cfg(1, 1, true, true), cfg(4, 8, true, true), cfg(3, 5, true, false)] {
                let (res, x) = solve(p, tw, seed, c.clone());
                assert_eq!(res.log, Some(x), "p={p} seed={seed} cfg={c:?}");
                assert!(res.steps > 0);
            }
        }
    }

    #[test]
    fn par_rho_negation_finds_fruitless_cycles_and_still_solves() {
        // tiny table to force many 2-cycles
        let (res, x) = solve(211, 1, 7, ParCfg { nb: Some(4), ..cfg(2, 4, true, true) });
        assert_eq!(res.log, Some(x));
        assert!(res.fruitless_cycles > 0);
    }
}
