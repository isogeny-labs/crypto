//! Round 7, lead R7.7: the trace pair as a label of `E/4E` and `E/8E` for
//! binary curves `y^2 + xy = x^3 + a x^2 + b` over `F_{2^n}`, n odd, beyond
//! the single curve `a = 0, b = 1` of R6.3.
//!
//! ```sh
//! rustc -O --edition 2021 examples/r7_7_coset_label.rs -o r7_7
//! ./r7_7 selftest
//! ./r7_7 exhaustive --nmin 5 --nmax 13 --out exhaustive.json
//! ./r7_7 random --ns 17,19,21,23 --curves 1000 --pts 1500 --seed 77 --out random.json
//! ./r7_7 cost --ns 17,19,21,23 --curves 60 --pts 400 --seed 78 --out cost.json
//! ```
//!
//! The example is self-contained (std only): `F_{2^n}` is a log/antilog table
//! field, so it shares no arithmetic with `crypto_lib` or with the R6.3
//! example.  Its arithmetic is cross-checked by `selftest` against a
//! shift-and-add reference.
//!
//! # Predictions (written before the first measurement)
//!
//! Let `E: y^2 + xy = x^3 + a x^2 + b`, `b != 0`, n odd, `T = (0, sqrt b)` the
//! unique point of order 2, `Tr` the absolute trace, `H` the half-trace.
//! Doubling gives `x(2P) = l^2 + l + a`, so `P in 2E <=> Tr(x) = Tr(a)`.
//! Hence `E[2](F) = <T>` and the 2-primary part of `E(F)` is **cyclic**:
//! `Z/4 x Z/2` cannot occur for these curves.  Let `v = v2(#E)`.
//!
//! * `Tr(a) = 1` (e.g. `a = 1`, n odd): `T notin 2E`, `v = 1`, `#E = 2 mod 4`,
//!   `2E` has odd order, so `E/4E = E/8E = E/2E = Z/2`.  **P1**: the coset of
//!   `P` is `Tr(x) + Tr(a)`; the second trace bit
//!   `b2(P) = Tr(y) + Tr(x H(x+a))` is *not* a coset invariant (so the pair is
//!   not a "label of `E/4E`" in R6.3's sense: it is not constant on cosets).
//!   **P2**: for `P in 2E`, `b2(P)` is the root selector of the halving inside
//!   the odd-order subgroup: with `l = H(x+a) + b2(P)` the half `Q` lies in
//!   `2E`.
//! * `Tr(a) = 0`: `T in 2E`, `v >= 2`, `E/4E = Z/4` and `P in 4E <=> Tr(x) = 0
//!   and b2(P) = 0` for every `b` (the `b = 1` proof of Prop. 14.2 does not use
//!   `b`).  **P3** (the R6.3 lemma, any `b` and `a`): `L(P) = (Tr(x), b2(P))`
//!   is constant on each coset of `4E`; `L = (0,0)` on `4E`, `(0,1)` on `T+4E`
//!   (v = 2) resp. on the class 2, and the two odd cosets carry `(1, beta)` and
//!   `(1, 1+beta)` because `b2(-P) = b2(P) + Tr(x)`.  For `v = 2` the odd
//!   generator is `P4 = (b^(1/4), y4)` with `Tr(y4/x4) = 0` and `beta =
//!   b2(P4)` (a closed form for `beta` is part of the measurement).
//! * `v >= 3` (`Tr(a) = Tr(b) = 0`): `E/8E = Z/8`; the pair has only four
//!   values so it cannot label eight cosets.  **P4**: with `b3(R) = b2(R/2)`
//!   for even `R` (either half, because `T in 4E`) and `b3(P) = b3(P + Q0)`
//!   for odd `P` (`Q0` a fixed odd point with `b2(Q0) = 0`), the triple
//!   `(Tr(x), b2, b3)` is a bijection `E/8E -> F_2^3`:
//!   `k = 0..7 -> 000 100 010 111 001 101 011 110` as `(bit1,b2,b3)` read
//!   `bit1*4 + b2*2 + b3` = `0 4 2 7 1 5 3 6`, `k` taken relative to `Q0`.
//!
//! Falsifier: a point whose label differs from the prediction, a coset on
//! which the label is not constant, or two cosets sharing a label.

use std::cell::Cell;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::Instant;

/// 0 = the real label; 1, 2 = deliberately broken labels (negative controls).
static CONTROL: AtomicU32 = AtomicU32::new(0);

// ---------------------------------------------------------------- field ----

/// `F_{2^n}` (n odd) as `GF(2)[z]/p(z)` with `z` primitive; log/antilog tables.
struct Gf {
    q: u32,
    poly: u32,
    exp: Vec<u32>,
    log: Vec<u32>,
    tmask: u32,
    htab: Vec<[u32; 256]>,
}

fn raw_mulx(a: u32, poly: u32, n: u32) -> u32 {
    let mut e = a << 1;
    if (e >> n) & 1 == 1 {
        e ^= poly;
    }
    e
}

/// Shift-and-add reference product (independent of the tables).
fn ref_mul(a: u32, b: u32, poly: u32, n: u32) -> u32 {
    let (mut a, mut b, mut r) = (a, b, 0u32);
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        a = raw_mulx(a, poly, n);
        b >>= 1;
    }
    r
}

impl Gf {
    fn new(n: u32) -> Gf {
        assert!(n % 2 == 1 && (3..=25).contains(&n), "n must be odd, 3..=25");
        let q = 1u32 << n;
        let order = q - 1;
        let mut exp = vec![0u32; 2 * order as usize];
        let mut poly = 0u32;
        'search: for low in (1..q).step_by(2) {
            let p = q | low;
            let mut e = 1u32;
            for i in 0..order {
                exp[i as usize] = e;
                e = raw_mulx(e, p, n);
                if e == 1 && i + 1 < order {
                    continue 'search;
                }
            }
            if e == 1 {
                poly = p;
                break;
            }
        }
        assert!(poly != 0, "no primitive polynomial found");
        for i in 0..order as usize {
            exp[i + order as usize] = exp[i];
        }
        let mut log = vec![0u32; q as usize];
        for i in 0..order {
            log[exp[i as usize] as usize] = i;
        }
        let mut g = Gf {
            q,
            poly,
            exp,
            log,
            tmask: 0,
            htab: Vec::new(),
        };
        // trace functional and half-trace tables from the basis z^i
        let rsqr = |g: &Gf, a: u32| -> u32 {
            if a == 0 {
                0
            } else {
                g.exp[2 * g.log[a as usize] as usize]
            }
        };
        let mut tmask = 0u32;
        let mut hbasis = vec![0u32; n as usize];
        for i in 0..n {
            let e = 1u32 << i;
            let (mut s, mut acc) = (e, 0u32);
            for _ in 0..n {
                acc ^= s;
                s = rsqr(&g, s);
            }
            assert!(acc <= 1);
            tmask |= acc << i;
            let (mut s, mut h) = (e, 0u32);
            for _ in 0..=(n - 1) / 2 {
                h ^= s;
                s = rsqr(&g, rsqr(&g, s));
            }
            hbasis[i as usize] = h;
        }
        g.tmask = tmask;
        let nbytes = n.div_ceil(8) as usize;
        for j in 0..nbytes {
            let mut t = [0u32; 256];
            for (w, slot) in t.iter_mut().enumerate() {
                let mut h = 0u32;
                for bit in 0..8 {
                    let idx = 8 * j + bit;
                    if idx < n as usize && (w >> bit) & 1 == 1 {
                        h ^= hbasis[idx];
                    }
                }
                *slot = h;
            }
            g.htab.push(t);
        }
        g
    }
}

/// Counted operations: M product, S squaring or square root (linear), I
/// inversion, H half-trace, T trace.
const LABEL_NAMES: [&str; 6] = ["", "", "label2", "label3", "label4", "label5"];
const HALVING_NAMES: [&str; 6] = ["", "", "halving2", "halving3", "halving4", "halving5"];
const NOPS: usize = 5;
const OPN: [&str; NOPS] = ["M", "S", "I", "H", "T"];

struct Ctx<'a> {
    f: &'a Gf,
    c: Cell<[u64; NOPS]>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: u32,
    y: u32,
    inf: bool,
}
const INF: Pt = Pt {
    x: 0,
    y: 0,
    inf: true,
};

impl<'a> Ctx<'a> {
    fn new(f: &'a Gf) -> Self {
        Ctx {
            f,
            c: Cell::new([0; NOPS]),
        }
    }
    fn bump(&self, i: usize) {
        let mut c = self.c.get();
        c[i] += 1;
        self.c.set(c);
    }
    fn snap(&self) -> [u64; NOPS] {
        self.c.get()
    }
    fn mul(&self, a: u32, b: u32) -> u32 {
        self.bump(0);
        if a == 0 || b == 0 {
            0
        } else {
            self.f.exp[(self.f.log[a as usize] + self.f.log[b as usize]) as usize]
        }
    }
    fn sqr(&self, a: u32) -> u32 {
        self.bump(1);
        if a == 0 {
            0
        } else {
            self.f.exp[2 * self.f.log[a as usize] as usize]
        }
    }
    fn sqrt(&self, a: u32) -> u32 {
        self.bump(1);
        if a == 0 {
            0
        } else {
            let l = self.f.log[a as usize];
            self.f.exp[((l + (l & 1) * (self.f.q - 1)) >> 1) as usize]
        }
    }
    fn inv(&self, a: u32) -> u32 {
        self.bump(2);
        assert!(a != 0);
        self.f.exp[(self.f.q - 1 - self.f.log[a as usize]) as usize]
    }
    fn div(&self, a: u32, b: u32) -> u32 {
        self.bump(2);
        self.bump(0);
        assert!(b != 0);
        if a == 0 {
            0
        } else {
            self.f.exp[(self.f.log[a as usize] + self.f.q - 1 - self.f.log[b as usize]) as usize]
        }
    }
    fn tr(&self, a: u32) -> u32 {
        self.bump(4);
        (a & self.f.tmask).count_ones() & 1
    }
    fn half(&self, a: u32) -> u32 {
        self.bump(3);
        let mut h = 0u32;
        for (j, t) in self.f.htab.iter().enumerate() {
            h ^= t[((a >> (8 * j)) & 0xff) as usize];
        }
        h
    }
    // ---- group law on y^2 + xy = x^3 + a x^2 + b
    fn neg(&self, p: Pt) -> Pt {
        if p.inf {
            p
        } else {
            Pt {
                x: p.x,
                y: p.y ^ p.x,
                inf: false,
            }
        }
    }
    fn dbl(&self, p: Pt, a: u32) -> Pt {
        if p.inf || p.x == 0 {
            return INF;
        }
        let l = p.x ^ self.div(p.y, p.x);
        let x3 = self.sqr(l) ^ l ^ a;
        let y3 = self.sqr(p.x) ^ self.mul(l ^ 1, x3);
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    fn add(&self, p: Pt, q: Pt, a: u32) -> Pt {
        if p.inf {
            return q;
        }
        if q.inf {
            return p;
        }
        if p.x == q.x {
            return if p.y == q.y { self.dbl(p, a) } else { INF };
        }
        let l = self.div(p.y ^ q.y, p.x ^ q.x);
        let x3 = self.sqr(l) ^ l ^ p.x ^ q.x ^ a;
        let y3 = self.mul(l, p.x ^ x3) ^ x3 ^ p.y;
        Pt {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    fn smul(&self, p: Pt, k: u64, a: u32) -> Pt {
        let mut r = INF;
        for i in (0..64).rev() {
            r = self.dbl(r, a);
            if (k >> i) & 1 == 1 {
                r = self.add(r, p, a);
            }
        }
        r
    }
    fn on_curve(&self, p: Pt, a: u32, b: u32) -> bool {
        if p.inf {
            return true;
        }
        let lhs = self.sqr(p.y) ^ self.mul(p.x, p.y);
        let x2 = self.sqr(p.x);
        let rhs = self.mul(x2, p.x) ^ self.mul(a, x2) ^ b;
        lhs == rhs
    }
    // ---- trace-label primitives
    /// 1 iff `P notin 2E` (i.e. `Tr(x) != Tr(a)`).
    fn bit1(&self, p: Pt, atr: u32) -> u32 {
        self.tr(p.x) ^ atr
    }
    /// Second trace bit `Tr(y) + Tr(x H(x + a))`.
    fn b2(&self, p: Pt, a: u32) -> u32 {
        match CONTROL.load(Ordering::Relaxed) {
            // negative controls: the checker must fire on these broken labels
            1 => {
                let h = self.half(p.x); // forgets the shift by a
                self.tr(p.y) ^ self.tr(self.mul(p.x, h))
            }
            2 => self.tr(p.y), // drops the Tr(x H) term
            _ => {
                let h = self.half(p.x ^ a);
                self.tr(p.y) ^ self.tr(self.mul(p.x, h))
            }
        }
    }
    /// A half of `P in 2E`; `c` selects the root `l = H(x+a) + c`.
    fn halve(&self, p: Pt, a: u32, c: u32) -> Pt {
        let lam = self.half(p.x ^ a) ^ c;
        let xq = self.sqrt(p.y ^ self.mul(lam ^ 1, p.x));
        let yq = self.mul(xq, lam ^ xq);
        Pt {
            x: xq,
            y: yq,
            inf: false,
        }
    }
    /// Third bit of an even point `R` (`R in 2E`): `b2` of a half.
    fn b3_even(&self, r: Pt, a: u32, c: u32) -> u32 {
        if r.inf {
            0
        } else {
            let h = self.halve(r, a, c);
            self.b2(h, a)
        }
    }
    /// `(bit1, b2)` packed as `bit1*2 + b2`.
    fn label4(&self, p: Pt, a: u32, atr: u32) -> u32 {
        2 * self.bit1(p, atr) + self.b2(p, a)
    }
    /// J-bit label `(bit1(P), b2(S_1), ..., b2(S_{J-1}))` packed MSB first,
    /// with `S_1 = P` and `S_{i+1} = half(R_i)`, `R_i = S_i + Q0` if `S_i` is
    /// odd and `S_i` otherwise (all later bits are 0 once some `R_i = O`).
    /// The half-trace `H(x+a)` of a level is computed once and shared by the
    /// `b2` bit and the halving of that level (so an even level costs one `H`,
    /// an odd level two plus one point addition).
    fn label_j(&self, p: Pt, a: u32, atr: u32, q0: Pt, jj: u32) -> u32 {
        let t = self.bit1(p, atr);
        let mut out = t;
        let mut s = p;
        let mut odd = t == 1;
        for j in 2..=jj {
            if s.inf {
                out *= 2;
                continue;
            }
            let h = self.half(s.x ^ a);
            out = 2 * out + (self.tr(s.y) ^ self.tr(self.mul(s.x, h)));
            if j < jj {
                let (r, hr) = if odd {
                    let r = self.add(s, q0, a);
                    if r.inf {
                        s = INF;
                        continue;
                    }
                    (r, self.half(r.x ^ a))
                } else {
                    (s, h)
                };
                let xq = self.sqrt(r.y ^ self.mul(hr ^ 1, r.x));
                let yq = self.mul(xq, hr ^ xq);
                s = Pt {
                    x: xq,
                    y: yq,
                    inf: false,
                };
                odd = self.bit1(s, atr) == 1;
            }
        }
        out
    }
    /// `k mod 2^bits` by repeated halving; `k` is relative to `q0` (`k(q0) = 1`).
    fn kmod(&self, p: Pt, bits: u32, a: u32, atr: u32, q0: Pt) -> u32 {
        if bits == 0 || p.inf {
            return 0;
        }
        let t = self.bit1(p, atr);
        if bits == 1 {
            return t;
        }
        let r = if t == 1 {
            self.add(p, self.neg(q0), a)
        } else {
            p
        };
        if r.inf {
            return t;
        }
        let r1 = self.halve(r, a, 0);
        (t + 2 * self.kmod(r1, bits - 1, a, atr, q0)) & ((1 << bits) - 1)
    }
}

// ------------------------------------------------------------------ rng ----

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn below(&mut self, m: u64) -> u64 {
        self.next() % m
    }
}

fn mix(seed: u64, a: u64, b: u64, c: u64) -> u64 {
    let mut r = Rng(seed ^ a.wrapping_mul(0xD1B5_4A32_D192_ED03));
    r.0 ^= b.wrapping_mul(0x8CB9_2BA7_2F3D_8DD7);
    r.next();
    r.0 ^= c.wrapping_mul(0xA24B_AED4_963E_E407);
    r.next()
}

// --------------------------------------------------------------- curves ----

/// All affine points (x-ascending, then y) and the group order.
fn enumerate(cx: &Ctx, a: u32, b: u32) -> Vec<Pt> {
    let f = cx.f;
    let mut v = Vec::with_capacity(f.q as usize + 8);
    v.push(Pt {
        x: 0,
        y: cx.sqrt(b),
        inf: false,
    });
    for x in 1..f.q {
        let c = x ^ a ^ cx.div(b, cx.sqr(x));
        if cx.tr(c) == 0 {
            let y = cx.mul(x, cx.half(c));
            v.push(Pt { x, y, inf: false });
            v.push(Pt {
                x,
                y: y ^ x,
                inf: false,
            });
        }
    }
    v
}

fn random_point(cx: &Ctx, a: u32, b: u32, rng: &mut Rng) -> Pt {
    let q = cx.f.q as u64;
    loop {
        let x = 1 + rng.below(q - 1) as u32;
        let c = x ^ a ^ cx.div(b, cx.sqr(x));
        if cx.tr(c) == 0 {
            let y = cx.mul(x, cx.half(c));
            let y = if rng.next() & 1 == 1 { y ^ x } else { y };
            return Pt { x, y, inf: false };
        }
    }
}

/// Group order by counting (`1 + number of affine points`).
fn count_order(cx: &Ctx, a: u32, b: u32) -> u64 {
    let f = cx.f;
    let mut cnt = 2u64; // O and (0, sqrt b)
    for x in 1..f.q {
        let c = x ^ a ^ cx.div(b, cx.sqr(x));
        if cx.tr(c) == 0 {
            cnt += 2;
        }
    }
    cnt
}

/// Highest quotient level tested: `E/2^J E` for `J <= JMAX`.
const JMAX: u32 = 5;

/// Per-curve ground truth: the 2-primary class map `P -> k in Z/2^e`
/// relative to the reference `q0` (`k(q0) = 1`).
struct Cls {
    r: u64,
    v: u32,
    e: u32,
    sh: u32,
    table: Vec<Pt>,
    q0: Pt,
    beta: u32,
    /// closed form `Tr(b H(b))` for `beta` (used only when `v = 2`)
    beta_cf: u32,
    a: u32,
    atr: u32,
}

impl Cls {
    /// Returns `None` if the setup is inconsistent (reported, never ignored).
    fn setup(cx: &Ctx, a: u32, b: u32, rng: &mut Rng) -> Option<(Cls, u64)> {
        let atr = cx.tr(a);
        let nord = count_order(cx, a, b);
        let v = nord.trailing_zeros();
        let r = nord >> v;
        let q0 = if v == 2 {
            // canonical P4 = (b^(1/4), y4) with Tr(y4/x4) = 0
            let x4 = cx.sqrt(cx.sqrt(b));
            let c = x4 ^ a ^ cx.sqr(x4);
            let mut z = cx.half(c);
            if cx.tr(z) == 1 {
                z ^= 1;
            }
            Pt {
                x: x4,
                y: cx.mul(x4, z),
                inf: false,
            }
        } else {
            loop {
                let p = random_point(cx, a, b, rng);
                if cx.bit1(p, atr) == 1 {
                    // orient so that b2(q0) = 0 when v >= 3
                    break if v >= 3 && cx.b2(p, a) == 1 {
                        cx.neg(p)
                    } else {
                        p
                    };
                }
            }
        };
        if !cx.on_curve(q0, a, b) {
            return None;
        }
        let e = v.min(JMAX);
        let sh = v - e;
        let g = cx.smul(q0, r, a);
        // order of g must be exactly 2^v
        let mut t = g;
        for _ in 0..v - 1 {
            t = cx.dbl(t, a);
        }
        if t.inf || !cx.dbl(t, a).inf {
            return None;
        }
        let mut h = g;
        for _ in 0..sh {
            h = cx.dbl(h, a);
        }
        let mut table = vec![INF];
        for m in 1..(1u32 << e) {
            let last = table[(m - 1) as usize];
            table.push(cx.add(last, h, a));
        }
        // check a random point has [N]P = O
        let pr = random_point(cx, a, b, rng);
        if !cx.smul(pr, nord, a).inf {
            return None;
        }
        let beta = if v == 2 { cx.b2(q0, a) } else { 0 };
        Some((
            Cls {
                r,
                v,
                e,
                sh,
                table,
                q0,
                beta,
                beta_cf: cx.tr(cx.mul(b, cx.half(b))),
                a,
                atr,
            },
            nord,
        ))
    }
    fn kclass(&self, cx: &Ctx, p: Pt) -> Option<u32> {
        let mut s = cx.smul(p, self.r, self.a);
        for _ in 0..self.sh {
            s = cx.dbl(s, self.a);
        }
        self.table.iter().position(|t| *t == s).map(|m| m as u32)
    }
}

/// Second bit as a function of the class `k mod 4` (relative to `q0`):
/// `0 -> 0`, `2 -> 1`, `1 -> beta`, `3 -> 1 - beta`.
fn f2(kk: u32, beta: u32) -> u32 {
    match kk & 3 {
        0 => 0,
        2 => 1,
        1 => beta,
        _ => 1 - beta,
    }
}

/// Predicted bit `b_j` as a function of `k mod 2^j`.
fn pred_bit(k: u32, j: u32, beta: u32) -> u32 {
    let kk = k & ((1 << j) - 1);
    match j {
        1 => kk & 1,
        2 => f2(kk, beta),
        _ => {
            let r = if kk & 1 == 1 { kk + 1 } else { kk };
            pred_bit((r >> 1) & ((1 << (j - 1)) - 1), j - 1, beta)
        }
    }
}

/// Predicted J-bit label (MSB first) of class `k`.
fn pred_label(k: u32, jj: u32, beta: u32) -> u32 {
    (1..=jj).fold(0, |acc, j| 2 * acc + pred_bit(k, j, beta))
}

// ------------------------------------------------------------ aggregates ----

const NJ: usize = JMAX as usize + 1;

#[derive(Default, Clone)]
struct Agg {
    curves: u64,
    points: u64,
    setup_fail: u64,
    size_fail: u64,
    // v = 1
    mis_coset1: u64, // bit1 != k
    pair_nonconst_cosets: u64,
    pair_cosets: u64,
    b2_hist: [[u64; 2]; 2], // [k][b2] for v = 1
    rootsel_pts: u64,
    rootsel_fail: u64,
    // level J = 2..=JMAX (index J): curves with v >= J
    curves_j: [u64; NJ],
    points_j: [u64; NJ],
    mis: [u64; NJ],
    nonconst: [u64; NJ],
    coll: [u64; NJ],
    cosets: [u64; NJ],
    b3root_pts: u64,
    b3root_dep: u64, // b3(c=0) != b3(c=1), v >= 3
    v2_root_pts: u64,
    v2_root_dep: u64, // v = 2: expected always
    beta: [u64; 2],
    halve_fail: u64,
    beta_fail: u64,   // v = 2: beta != Tr(b H(b))
    xonly1_fail: u64, // Tr(b/x^2) != Tr(x) + Tr(a) on a point with x != 0
    xonly2_fail: u64, // even P, Tr(a)=0: b2 != Tr(x H(b/x^2))
}

fn jarr(a: &[u64; NJ]) -> String {
    let v: Vec<String> = a.iter().map(|x| x.to_string()).collect();
    format!("[{}]", v.join(","))
}

impl Agg {
    fn merge(&mut self, o: &Agg) {
        macro_rules! m {
            ($($f:ident),*) => { $( self.$f += o.$f; )* };
        }
        m!(
            curves,
            points,
            setup_fail,
            size_fail,
            mis_coset1,
            pair_nonconst_cosets,
            pair_cosets,
            rootsel_pts,
            rootsel_fail,
            b3root_pts,
            b3root_dep,
            v2_root_pts,
            v2_root_dep,
            halve_fail,
            beta_fail,
            xonly1_fail,
            xonly2_fail
        );
        for i in 0..NJ {
            self.curves_j[i] += o.curves_j[i];
            self.points_j[i] += o.points_j[i];
            self.mis[i] += o.mis[i];
            self.nonconst[i] += o.nonconst[i];
            self.coll[i] += o.coll[i];
            self.cosets[i] += o.cosets[i];
        }
        for i in 0..2 {
            self.beta[i] += o.beta[i];
            for j in 0..2 {
                self.b2_hist[i][j] += o.b2_hist[i][j];
            }
        }
    }
    fn json(&self) -> String {
        format!(
            "{{\"curves\":{},\"points\":{},\"setup_fail\":{},\"size_fail\":{},\"mis_coset1\":{},\"pair_cosets\":{},\"pair_nonconst_cosets\":{},\"b2_hist\":[[{},{}],[{},{}]],\"rootsel_pts\":{},\"rootsel_fail\":{},\"curves_j\":{},\"points_j\":{},\"mis\":{},\"cosets\":{},\"nonconst\":{},\"coll\":{},\"b3root_pts\":{},\"b3root_dep\":{},\"v2_root_pts\":{},\"v2_root_dep\":{},\"beta\":[{},{}],\"halve_fail\":{},\"beta_fail\":{},\"xonly1_fail\":{},\"xonly2_fail\":{}}}",
            self.curves, self.points, self.setup_fail, self.size_fail, self.mis_coset1,
            self.pair_cosets, self.pair_nonconst_cosets, self.b2_hist[0][0], self.b2_hist[0][1],
            self.b2_hist[1][0], self.b2_hist[1][1], self.rootsel_pts, self.rootsel_fail,
            jarr(&self.curves_j), jarr(&self.points_j), jarr(&self.mis), jarr(&self.cosets),
            jarr(&self.nonconst), jarr(&self.coll), self.b3root_pts, self.b3root_dep,
            self.v2_root_pts, self.v2_root_dep, self.beta[0], self.beta[1], self.halve_fail, self.beta_fail,
            self.xonly1_fail, self.xonly2_fail
        )
    }
}

/// Group key: (n, a-kind, v-category 1..=JMAX, JMAX meaning v >= JMAX).
type Key = (u32, &'static str, u32);

/// Analyse one curve over the given points; returns (vcat, aggregate).
/// `full`: the points are the complete affine point set (coset sizes are checked).
fn analyze(cx: &Ctx, a: u32, b: u32, pts: &[Pt], full: bool, rng: &mut Rng) -> (u32, Agg) {
    let mut ag = Agg::default();
    let (cl, nord) = match Cls::setup(cx, a, b, rng) {
        Some(c) => c,
        None => {
            ag.setup_fail = 1;
            return (0, ag);
        }
    };
    let (atr, v) = (cl.atr, cl.v);
    let vcat = v.min(JMAX);
    let emax = cl.e; // classes live in Z/2^emax
    ag.curves = 1;
    // [level J][k mod 2^J][label]
    let mut cnt: Vec<Vec<Vec<u32>>> = (0..NJ).map(|j| vec![vec![0u32; 1 << j]; 1 << j]).collect();
    let mut hist = vec![0u64; 1 << emax];
    hist[0] += 1; // the point at infinity
    for &p in pts {
        let Some(k) = cl.kclass(cx, p) else {
            ag.setup_fail += 1;
            continue;
        };
        hist[k as usize] += 1;
        ag.points += 1;
        let l4 = cx.label4(p, a, atr);
        let (t, b2) = (l4 >> 1, l4 & 1);
        if p.x != 0 {
            // x-only forms: Tr(b/x^2) = Tr(x) + Tr(a) on E, and for even P with
            // Tr(a) = 0 the second bit is Tr(x H(b/x^2)) (no y needed)
            let w = cx.div(b, cx.sqr(p.x));
            if cx.tr(w) != t {
                ag.xonly1_fail += 1;
            }
            if v >= 2 && t == 0 && cx.tr(cx.mul(p.x, cx.half(w))) != b2 {
                ag.xonly2_fail += 1;
            }
        }
        if v == 1 {
            if t != k {
                ag.mis_coset1 += 1;
            }
            ag.b2_hist[k as usize][b2 as usize] += 1;
            cnt[2][k as usize][l4 as usize] += 1;
            if t == 0 {
                // P in 2E: halve inside the odd-order subgroup
                ag.rootsel_pts += 1;
                let q = cx.halve(p, a, b2);
                if !(cx.dbl(q, a) == p && cx.bit1(q, atr) == 0) {
                    ag.rootsel_fail += 1;
                }
            }
            continue;
        }
        // v >= 2: level J = 2 uses the pair
        ag.points_j[2] += 1;
        if l4 != pred_label(k, 2, cl.beta) {
            ag.mis[2] += 1;
        }
        cnt[2][(k & 3) as usize][l4 as usize] += 1;
        if t == 0 {
            let h0 = cx.halve(p, a, 0);
            if cx.dbl(h0, a) != p {
                ag.halve_fail += 1;
            }
            if v == 2 {
                ag.v2_root_pts += 1;
                if cx.b2(h0, a) != cx.b2(cx.halve(p, a, 1), a) {
                    ag.v2_root_dep += 1;
                }
            } else {
                ag.b3root_pts += 1;
                if cx.b3_even(p, a, 0) != cx.b3_even(p, a, 1) {
                    ag.b3root_dep += 1;
                }
            }
        }
        for jj in 3..=emax.min(JMAX) {
            ag.points_j[jj as usize] += 1;
            let lj = cx.label_j(p, a, atr, cl.q0, jj);
            cnt[jj as usize][(k & ((1 << jj) - 1)) as usize][lj as usize] += 1;
            if lj != pred_label(k, jj, 0) {
                ag.mis[jj as usize] += 1;
            }
        }
    }
    if full {
        let per = nord >> emax;
        for &h in &hist {
            if h != per {
                ag.size_fail += 1;
            }
        }
    }
    if v == 1 {
        let (nc, _coll, seen) = const_coll(&cnt[2], 2, 4);
        ag.pair_nonconst_cosets = nc;
        ag.pair_cosets = seen;
        return (vcat, ag);
    }
    if v == 2 {
        ag.beta[cl.beta as usize] += 1;
        ag.beta_fail += (cl.beta != cl.beta_cf) as u64;
    }
    for jj in 2..=emax.min(JMAX) {
        let j = jj as usize;
        let (nc, coll, seen) = const_coll(&cnt[j], 1 << jj, 1 << jj);
        ag.curves_j[j] = 1;
        ag.nonconst[j] = nc;
        ag.coll[j] = coll;
        ag.cosets[j] = seen;
    }
    (vcat, ag)
}

/// (cosets with more than one label, labels shared by more than one coset, cosets seen)
fn const_coll(c: &[Vec<u32>], nk: usize, nl: usize) -> (u64, u64, u64) {
    let (mut nc, mut coll, mut seen) = (0, 0, 0);
    for row in c.iter().take(nk) {
        let d = row.iter().take(nl).filter(|&&x| x > 0).count();
        if d > 0 {
            seen += 1;
        }
        if d > 1 {
            nc += 1;
        }
    }
    for l in 0..nl {
        let d = c.iter().take(nk).filter(|row| row[l] > 0).count();
        if d > 1 {
            coll += 1;
        }
    }
    (nc, coll, seen)
}

// ------------------------------------------------------------- commands ----

fn arg(args: &[String], key: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == key)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn ns_list(s: &str) -> Vec<u32> {
    s.split(',').filter_map(|t| t.trim().parse().ok()).collect()
}

fn selftest() {
    let mut rng = Rng(12345);
    for n in [5u32, 7, 9, 11, 13, 17, 23] {
        let f = Gf::new(n);
        let cx = Ctx::new(&f);
        for _ in 0..20000 {
            let (a, b) = (rng.below(f.q as u64) as u32, rng.below(f.q as u64) as u32);
            assert_eq!(cx.mul(a, b), ref_mul(a, b, f.poly, n));
            assert_eq!(cx.sqr(a), ref_mul(a, a, f.poly, n));
            let s = cx.sqrt(a);
            assert_eq!(cx.sqr(s), a);
            if a != 0 {
                assert_eq!(cx.mul(a, cx.inv(a)), 1);
                assert_eq!(cx.mul(cx.div(b, a), a), b);
            }
            // trace = sum of conjugates; half-trace identity H^2 + H = c + Tr(c)
            let (mut acc, mut s) = (0u32, a);
            for _ in 0..n {
                acc ^= s;
                s = cx.sqr(s);
            }
            assert_eq!(acc, cx.tr(a));
            let h = cx.half(a);
            assert_eq!(cx.sqr(h) ^ h, a ^ cx.tr(a));
        }
        // group law: associativity, order, on-curve, halving and 2E criterion
        for &(a, b) in &[(0u32, 1u32), (1, 1), (0, (5 % f.q) | 1), (3, (7 % f.q) | 1)] {
            let nord = count_order(&cx, a, b);
            let q1 = f.q as f64 + 1.0;
            assert!((nord as f64 - q1).abs() <= 2.0 * (f.q as f64).sqrt() + 1e-9);
            for _ in 0..200 {
                let p = random_point(&cx, a, b, &mut rng);
                let r = random_point(&cx, a, b, &mut rng);
                let s = random_point(&cx, a, b, &mut rng);
                assert!(cx.on_curve(p, a, b));
                assert!(cx.smul(p, nord, a).inf);
                let l = cx.add(cx.add(p, r, a), s, a);
                let rr = cx.add(p, cx.add(r, s, a), a);
                assert_eq!(l, rr);
                assert!(cx.add(p, cx.neg(p), a).inf);
                let atr = cx.tr(a);
                let two_e = cx.bit1(p, atr) == 0;
                if two_e {
                    let h = cx.halve(p, a, 0);
                    assert!(cx.on_curve(h, a, b));
                    assert_eq!(cx.dbl(h, a), p);
                }
            }
            // 2E <=> Tr(x) = Tr(a) against the group: |2E| = N/2 (the kernel is <T>)
            let pts = enumerate(&cx, a, b);
            assert_eq!(pts.len() as u64 + 1, nord);
            let atr = cx.tr(a);
            let even = pts.iter().filter(|p| cx.bit1(**p, atr) == 0).count() as u64 + 1;
            assert_eq!(even, nord / 2);
        }
    }
    println!("selftest ok");
}

fn akind_name(a: u32) -> &'static str {
    match a {
        0 => "a0",
        1 => "a1",
        _ => "arand",
    }
}

fn exhaustive(args: &[String]) {
    let nmin: u32 = arg(args, "--nmin", "5").parse().unwrap();
    let nmax: u32 = arg(args, "--nmax", "13").parse().unwrap();
    let threads: usize = arg(args, "--threads", "3").parse().unwrap();
    let out = arg(args, "--out", "exhaustive.json");
    let seed: u64 = arg(args, "--seed", "7").parse().unwrap();
    let all_a = args.iter().any(|x| x == "--alla");
    let t0 = Instant::now();
    let res: Mutex<BTreeMap<Key, Agg>> = Mutex::new(BTreeMap::new());
    for n in (nmin..=nmax).filter(|n| n % 2 == 1) {
        let f = Gf::new(n);
        let alist: Vec<u32> = if all_a {
            (0..f.q).collect()
        } else {
            vec![0, 1]
        };
        for a in alist {
            let next = AtomicU64::new(1);
            std::thread::scope(|s| {
                for _ in 0..threads {
                    s.spawn(|| {
                        let cx = Ctx::new(&f);
                        let mut local: BTreeMap<Key, Agg> = BTreeMap::new();
                        loop {
                            let b = next.fetch_add(1, Ordering::Relaxed);
                            if b >= f.q as u64 {
                                break;
                            }
                            let b = b as u32;
                            let mut rng = Rng(mix(seed, n as u64, a as u64, b as u64));
                            let pts = enumerate(&cx, a, b);
                            let (vc, ag) = analyze(&cx, a, b, &pts, true, &mut rng);
                            local
                                .entry((n, if all_a { "aall" } else { akind_name(a) }, vc))
                                .or_default()
                                .merge(&ag);
                        }
                        let mut g = res.lock().unwrap();
                        for (k, v) in local {
                            g.entry(k).or_default().merge(&v);
                        }
                    });
                }
            });
            eprintln!("n={} a={} done ({:.0}s)", n, a, t0.elapsed().as_secs_f64());
        }
    }
    write_groups(&out, "exhaustive", &res.into_inner().unwrap(), args);
}

fn random_mode(args: &[String]) {
    let ns = ns_list(&arg(args, "--ns", "17,19,21,23"));
    let curves: u64 = arg(args, "--curves", "1000").parse().unwrap();
    let npts: usize = arg(args, "--pts", "1500").parse().unwrap();
    let threads: usize = arg(args, "--threads", "3").parse().unwrap();
    let seed: u64 = arg(args, "--seed", "77").parse().unwrap();
    let out = arg(args, "--out", "random.json");
    let t0 = Instant::now();
    let res: Mutex<BTreeMap<Key, Agg>> = Mutex::new(BTreeMap::new());
    for &n in &ns {
        let f = Gf::new(n);
        // a = 1 (H1), a = 0, a uniform in F_{2^n}
        for akind in [1u32, 0, 2] {
            let next = AtomicU64::new(0);
            std::thread::scope(|s| {
                for _ in 0..threads {
                    s.spawn(|| {
                        let cx = Ctx::new(&f);
                        let mut local: BTreeMap<Key, Agg> = BTreeMap::new();
                        loop {
                            let idx = next.fetch_add(1, Ordering::Relaxed);
                            if idx >= curves {
                                break;
                            }
                            let mut rng = Rng(mix(seed, n as u64, akind as u64, idx));
                            let a = match akind {
                                2 => rng.below(f.q as u64) as u32,
                                v => v,
                            };
                            let b = 1 + rng.below(f.q as u64 - 1) as u32;
                            let pts: Vec<Pt> = (0..npts)
                                .map(|_| random_point(&cx, a, b, &mut rng))
                                .collect();
                            let (vc, ag) = analyze(&cx, a, b, &pts, false, &mut rng);
                            let name = if akind == 2 {
                                "arand"
                            } else {
                                akind_name(akind)
                            };
                            local.entry((n, name, vc)).or_default().merge(&ag);
                        }
                        let mut g = res.lock().unwrap();
                        for (k, v) in local {
                            g.entry(k).or_default().merge(&v);
                        }
                    });
                }
            });
            eprintln!(
                "n={} akind={} done ({:.0}s)",
                n,
                akind,
                t0.elapsed().as_secs_f64()
            );
        }
    }
    write_groups(&out, "random", &res.into_inner().unwrap(), args);
}

fn write_groups(out: &str, mode: &str, res: &BTreeMap<Key, Agg>, args: &[String]) {
    let mut s = String::new();
    write!(
        s,
        "{{\"mode\":\"{}\",\"args\":\"{}\",\"groups\":[",
        mode,
        args[1..].join(" ")
    )
    .unwrap();
    let mut tot = Agg::default();
    println!(
        "{:>3} {:>5} {:>2} {:>7} {:>10} | {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6}",
        "n", "a", "v", "curves", "points", "mis1", "mis2", "nc2", "col2", "mis3", "nc3", "col3",
        "mis4", "nc4", "col4", "mis5", "nc5", "col5", "fail"
    );
    for (i, ((n, a, vc), ag)) in res.iter().enumerate() {
        println!(
            "{:>3} {:>5} {:>2} {:>7} {:>10} | {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6} {:>5} {:>5} | {:>6}",
            n, a, vc, ag.curves, ag.points, ag.mis_coset1,
            ag.mis[2], ag.nonconst[2], ag.coll[2],
            ag.mis[3], ag.nonconst[3], ag.coll[3],
            ag.mis[4], ag.nonconst[4], ag.coll[4],
            ag.mis[5], ag.nonconst[5], ag.coll[5],
            ag.setup_fail
                + ag.halve_fail
                + ag.rootsel_fail
                + ag.size_fail
                + ag.beta_fail
                + ag.xonly1_fail
                + ag.xonly2_fail
        );
        if i > 0 {
            s.push(',');
        }
        write!(
            s,
            "{{\"n\":{},\"a\":\"{}\",\"vcat\":{},\"agg\":{}}}",
            n,
            a,
            vc,
            ag.json()
        )
        .unwrap();
        tot.merge(ag);
    }
    write!(s, "],\"total\":{}}}", tot.json()).unwrap();
    println!("TOTAL {}", tot.json());
    std::fs::write(out, s).unwrap();
}

// ------------------------------------------------------------------ cost ----

#[derive(Default, Clone, Copy)]
struct CostAcc {
    n: u64,
    ops: [u64; NOPS],
}
impl CostAcc {
    fn add(&mut self, before: [u64; NOPS], after: [u64; NOPS]) {
        self.n += 1;
        for (o, (b, a)) in self.ops.iter_mut().zip(before.iter().zip(after.iter())) {
            *o += a - b;
        }
    }
    fn json(&self) -> String {
        let mut s = format!("{{\"points\":{}", self.n);
        for (name, o) in OPN.iter().zip(self.ops.iter()) {
            write!(s, ",\"{}\":{:.3}", name, *o as f64 / self.n.max(1) as f64).unwrap();
        }
        s.push('}');
        s
    }
}

fn cost_mode(args: &[String]) {
    let ns = ns_list(&arg(args, "--ns", "17,19,21,23"));
    let curves: u64 = arg(args, "--curves", "60").parse().unwrap();
    let npts: usize = arg(args, "--pts", "400").parse().unwrap();
    let seed: u64 = arg(args, "--seed", "78").parse().unwrap();
    let out = arg(args, "--out", "cost.json");
    let mut s = String::from("{\"rows\":[");
    let mut first = true;
    for &n in &ns {
        let f = Gf::new(n);
        let cx = Ctx::new(&f);
        // methods: label, halving, scalar; per vcat and parity
        let mut acc: BTreeMap<(u32, &'static str, &'static str), CostAcc> = BTreeMap::new();
        let mut mism = 0u64;
        for akind in [1u32, 0] {
            let mut got = [0u64; 4];
            let mut idx = 0u64;
            while if akind == 1 {
                got[1] < curves
            } else {
                got[2] < curves || got[3] < curves
            } {
                idx += 1;
                let mut rng = Rng(mix(seed, n as u64, akind as u64, idx));
                let a = akind;
                let b = 1 + rng.below(f.q as u64 - 1) as u32;
                let Some((cl, _)) = Cls::setup(&cx, a, b, &mut rng) else {
                    continue;
                };
                let vc = cl.v.min(3);
                // balance v = 2 and v >= 3 for a = 0
                if got[vc as usize] >= curves {
                    continue;
                }
                got[vc as usize] += 1;
                for _ in 0..npts {
                    let p = random_point(&cx, a, b, &mut rng);
                    let par = if cx.bit1(p, cl.atr) == 1 {
                        "odd"
                    } else {
                        "even"
                    };
                    let truth = cl.kclass(&cx, p).unwrap();
                    // scalar-multiplication ground truth
                    let b0 = cx.snap();
                    let _ = cl.kclass(&cx, p);
                    let b1 = cx.snap();
                    acc.entry((vc, "scalar", par)).or_default().add(b0, b1);
                    if vc == 1 {
                        let b0 = cx.snap();
                        let t = cx.bit1(p, cl.atr);
                        let b1 = cx.snap();
                        acc.entry((vc, "label", par)).or_default().add(b0, b1);
                        if t != truth {
                            mism += 1;
                        }
                        continue;
                    }
                    // label4 and halving4
                    let b0 = cx.snap();
                    let l4 = cx.label4(p, a, cl.atr);
                    let b1 = cx.snap();
                    acc.entry((vc, LABEL_NAMES[2], par))
                        .or_default()
                        .add(b0, b1);
                    if l4 != pred_label(truth, 2, cl.beta) {
                        mism += 1;
                    }
                    let b0 = cx.snap();
                    let k4 = cx.kmod(p, 2, a, cl.atr, cl.q0);
                    let b1 = cx.snap();
                    acc.entry((vc, HALVING_NAMES[2], par))
                        .or_default()
                        .add(b0, b1);
                    if k4 != truth & 3 {
                        mism += 1;
                    }
                    for jj in 3..=cl.e {
                        let b0 = cx.snap();
                        let lj = cx.label_j(p, a, cl.atr, cl.q0, jj);
                        let b1 = cx.snap();
                        acc.entry((vc, LABEL_NAMES[jj as usize], par))
                            .or_default()
                            .add(b0, b1);
                        if lj != pred_label(truth, jj, 0) {
                            mism += 1;
                        }
                        let b0 = cx.snap();
                        let kj = cx.kmod(p, jj, a, cl.atr, cl.q0);
                        let b1 = cx.snap();
                        acc.entry((vc, HALVING_NAMES[jj as usize], par))
                            .or_default()
                            .add(b0, b1);
                        if kj != truth & ((1 << jj) - 1) {
                            mism += 1;
                        }
                    }
                }
            }
        }
        for ((vc, m, par), c) in &acc {
            if !first {
                s.push(',');
            }
            first = false;
            write!(
                s,
                "{{\"n\":{},\"vcat\":{},\"method\":\"{}\",\"parity\":\"{}\",\"mean_ops\":{}}}",
                n,
                vc,
                m,
                par,
                c.json()
            )
            .unwrap();
        }
        eprintln!("n={} cost done; mismatches {}", n, mism);
        assert_eq!(mism, 0, "cost-mode correctness mismatch");
    }
    s.push_str("]}");
    std::fs::write(&out, s).unwrap();
    println!("wrote {}", out);
}

/// Dump beta(a, b) for v = 2 curves, to find a closed form.
fn beta_mode(args: &[String]) {
    let nmin: u32 = arg(args, "--nmin", "5").parse().unwrap();
    let nmax: u32 = arg(args, "--nmax", "13").parse().unwrap();
    let out = arg(args, "--out", "beta.json");
    let mut s = String::from("{\"rows\":[");
    let mut first = true;
    // feature -> (counts of beta=0, beta=1) for each candidate closed form
    for n in (nmin..=nmax).filter(|n| n % 2 == 1) {
        let f = Gf::new(n);
        let cx = Ctx::new(&f);
        let mut rng = Rng(5);
        let mut cnt: BTreeMap<String, [u64; 2]> = BTreeMap::new();
        for a in [0u32, (f.q / 3) | 1] {
            if cx.tr(a) == 1 {
                continue;
            }
            for b in 1..f.q {
                if cx.tr(b) != 1 {
                    continue;
                }
                let Some((cl, _)) = Cls::setup(&cx, a, b, &mut rng) else {
                    continue;
                };
                if cl.v != 2 {
                    continue;
                }
                let x4 = cl.q0.x;
                let feats = [
                    ("const", 0u32),
                    ("Tr(a*?)=0", 0),
                    ("Tr(x4)", cx.tr(x4)),
                    ("Tr(b)", cx.tr(b)),
                    ("Tr(sqrt(x4))", cx.tr(cx.sqrt(x4))),
                    ("Tr(a*x4)", cx.tr(cx.mul(a, x4))),
                    ("Tr(x4*H(x4))", cx.tr(cx.mul(x4, cx.half(x4)))),
                    ("Tr(H(x4+a))", cx.tr(cx.half(x4 ^ a))),
                    ("Tr(x4*x4*x4)", cx.tr(cx.mul(x4, cx.sqr(x4)))),
                    ("Tr(y4)", cx.tr(cl.q0.y)),
                ];
                for (name, val) in feats {
                    cnt.entry(format!("{}={}", name, val)).or_default()[cl.beta as usize] += 1;
                }
                cnt.entry("all".into()).or_default()[cl.beta as usize] += 1;
                let _ = rng.next();
            }
        }
        for (k, c) in &cnt {
            if !first {
                s.push(',');
            }
            first = false;
            write!(
                s,
                "{{\"n\":{},\"key\":\"{}\",\"beta0\":{},\"beta1\":{}}}",
                n, k, c[0], c[1]
            )
            .unwrap();
            println!("n={} {:<22} beta0={:<7} beta1={}", n, k, c[0], c[1]);
        }
    }
    s.push_str("]}");
    std::fs::write(&out, s).unwrap();
}

/// Smallest counterexamples: (i) a = 1, the pair is not constant on a coset;
/// (ii) v = 3, two cosets of E/8E with the same pair label (the pair has only
/// four values for eight cosets).  Prints JSON.
fn witness_mode(args: &[String]) {
    let n: u32 = arg(args, "--n", "5").parse().unwrap();
    let out = arg(args, "--out", "witness.json");
    let f = Gf::new(n);
    let cx = Ctx::new(&f);
    let mut rng = Rng(3);
    let mut s = format!("{{\"n\":{},\"modulus\":\"0x{:x}\",\"curves\":[", n, f.poly);
    let mut first = true;
    for (a, want_v) in [(1u32, 1u32), (0, 3)] {
        for b in 1..f.q {
            let Some((cl, nord)) = Cls::setup(&cx, a, b, &mut rng) else {
                continue;
            };
            if cl.v != want_v {
                continue;
            }
            let pts = enumerate(&cx, a, b);
            if !first {
                s.push(',');
            }
            first = false;
            write!(
                s,
                "{{\"a\":{},\"b\":{},\"order\":{},\"v\":{},\"q0\":[{},{}],\"cosets\":[",
                a, b, nord, cl.v, cl.q0.x, cl.q0.y
            )
            .unwrap();
            let ncos = 1usize << cl.e;
            for k in 0..ncos {
                let mut sample = Vec::new();
                for &p in &pts {
                    if cl.kclass(&cx, p) == Some(k as u32) && sample.len() < 6 {
                        let l = cx.label4(p, a, cl.atr);
                        let l3 = if cl.e >= 3 {
                            cx.label_j(p, a, cl.atr, cl.q0, 3) as i32
                        } else {
                            -1
                        };
                        sample.push(format!(
                            "{{\"x\":{},\"y\":{},\"pair\":{},\"triple\":{}}}",
                            p.x, p.y, l, l3
                        ));
                    }
                }
                if k > 0 {
                    s.push(',');
                }
                write!(s, "{{\"k\":{},\"points\":[{}]}}", k, sample.join(",")).unwrap();
            }
            s.push_str("]}");
            break;
        }
    }
    s.push_str("]}");
    std::fs::write(&out, &s).unwrap();
    println!("{}", s);
}

// ---------------------------------------------------------------- search ----

/// Feature pool for a cheap third bit on odd points: `Tr(m)` for every
/// monomial `m` of degree <= 3 (with repetition) in the base elements
/// `x, y, h = H(x+a), g = H(b/x^2), 1/x, y/x, H(h)`, plus the constant 1.
fn features(cx: &Ctx, p: Pt, a: u32, b: u32) -> u128 {
    let x = p.x;
    let h = cx.half(x ^ a);
    let u = cx.inv(x);
    let g = cx.half(cx.mul(b, cx.sqr(u)));
    let base = [x, p.y, h, g, u, cx.mul(p.y, u), cx.half(h)];
    let mut v: u128 = 1; // constant feature in bit 0
    let mut bit = 1;
    let nb = base.len();
    for i in 0..nb {
        v |= (cx.tr(base[i]) as u128) << bit;
        bit += 1;
        for j in i..nb {
            let m2 = cx.mul(base[i], base[j]);
            v |= (cx.tr(m2) as u128) << bit;
            bit += 1;
            for &bk in base.iter().skip(j) {
                v |= (cx.tr(cx.mul(m2, bk)) as u128) << bit;
                bit += 1;
            }
        }
    }
    assert!(bit <= 128);
    v
}

/// Insert `v` into an echelon basis (pivot = highest set bit); true if new.
fn echelon_insert(basis: &mut Vec<u128>, mut v: u128) -> bool {
    for &r in basis.iter() {
        let piv = 127 - r.leading_zeros();
        if (v >> piv) & 1 == 1 {
            v ^= r;
        }
    }
    if v == 0 {
        return false;
    }
    basis.push(v);
    basis.sort_by(|x, y| y.cmp(x));
    true
}

/// Null space (in the feature space of `nbits` bits) of the span of `rows`.
fn nullspace(rows: &[u128], nbits: u32) -> Vec<u128> {
    // reduce rows to reduced echelon form, then read off the null space
    let mut rs: Vec<u128> = Vec::new();
    for &r in rows {
        echelon_insert(&mut rs, r);
    }
    // full reduction
    let n = rs.len();
    for i in 0..n {
        let piv = 127 - rs[i].leading_zeros();
        for j in 0..n {
            if j != i && (rs[j] >> piv) & 1 == 1 {
                rs[j] ^= rs[i];
            }
        }
    }
    let pivots: Vec<u32> = rs.iter().map(|r| 127 - r.leading_zeros()).collect();
    let mut out = Vec::new();
    for free in 0..nbits {
        if pivots.contains(&free) {
            continue;
        }
        let mut c: u128 = 1 << free;
        for (r, &pv) in rs.iter().zip(pivots.iter()) {
            if (r >> free) & 1 == 1 {
                c |= 1 << pv;
            }
        }
        out.push(c);
    }
    out
}

/// Is there `c` in `null` with `c . s_i = 1` for every `s_i`?  (GF(2) solve.)
fn separable(null: &[u128], seps: &[u128]) -> bool {
    // rows: for each s_i, the vector (c_j . s_i)_j with right-hand side 1
    let m = null.len();
    assert!(m <= 120);
    let mut rows: Vec<u128> = Vec::new();
    for &s in seps {
        let mut r: u128 = 0;
        for (j, &c) in null.iter().enumerate() {
            r |= (((c & s).count_ones() & 1) as u128) << j;
        }
        r |= 1 << 127; // rhs marker in the top bit
        rows.push(r);
    }
    // Gaussian elimination on bits 0..m, rhs in bit 127
    let mut used = vec![false; rows.len()];
    for col in 0..m {
        if let Some(i) = (0..rows.len()).find(|&i| !used[i] && (rows[i] >> col) & 1 == 1) {
            used[i] = true;
            let pr = rows[i];
            for (k, row) in rows.iter_mut().enumerate() {
                if k != i && (*row >> col) & 1 == 1 {
                    *row ^= pr;
                }
            }
        }
    }
    // inconsistent iff a row is 0 on the unknown columns but has rhs = 1
    let mask: u128 = if m == 128 { !0 } else { (1u128 << m) - 1 };
    !rows.iter().any(|r| r & mask == 0 && (r >> 127) & 1 == 1)
}

/// Search the feature pool for a universal cheap third bit on odd points of
/// `v >= 3` curves: a combination that is constant on every coset of `8E`
/// (odd cosets) and separates `k` from `k + 4`.
fn search_mode(args: &[String]) {
    let ns = ns_list(&arg(args, "--ns", "7,9,11"));
    let curves: u64 = arg(args, "--curves", "40").parse().unwrap();
    let npts: usize = arg(args, "--pts", "300").parse().unwrap();
    let seed: u64 = arg(args, "--seed", "81").parse().unwrap();
    let out = arg(args, "--out", "search.json");
    // level 3: cosets of 8E, separate k from k+4 (the open question);
    // level 2: cosets of 4E, separate k=1 from k=3 (positive control: b2 does it)
    let level: u32 = arg(args, "--level", "3").parse().unwrap();
    let kmask: u32 = (1 << level) - 1;
    let half_step = 1usize << (level - 1);
    let nfeat: u32 = {
        let nb = 7u32;
        1 + nb + nb * (nb + 1) / 2 + nb * (nb + 1) * (nb + 2) / 6
    };
    let mut s = format!("{{\"level\":{},\"features\":{},\"rows\":[", level, nfeat);
    let mut diffs_all: Vec<u128> = Vec::new();
    let mut seps_all: Vec<u128> = Vec::new();
    let mut first = true;
    for &n in &ns {
        let f = Gf::new(n);
        let cx = Ctx::new(&f);
        let mut diffs_n: Vec<u128> = Vec::new();
        let mut seps_n: Vec<u128> = Vec::new();
        let mut used = 0u64;
        let mut idx = 0u64;
        while used < curves {
            idx += 1;
            let mut rng = Rng(mix(seed, n as u64, 0, idx));
            let a = if idx.is_multiple_of(2) {
                0
            } else {
                // random a with Tr(a) = 0
                loop {
                    let c = rng.below(f.q as u64) as u32;
                    if cx.tr(c) == 0 {
                        break c;
                    }
                }
            };
            let b = 1 + rng.below(f.q as u64 - 1) as u32;
            let Some((cl, _)) = Cls::setup(&cx, a, b, &mut rng) else {
                continue;
            };
            if cl.v < 3 {
                continue;
            }
            used += 1;
            let mut reps: Vec<Option<u128>> = vec![None; 1 << level];
            for _ in 0..npts {
                let p = random_point(&cx, a, b, &mut rng);
                if cx.bit1(p, cl.atr) == 0 {
                    continue;
                }
                let k = (cl.kclass(&cx, p).unwrap() & kmask) as usize;
                let phi = features(&cx, p, a, b);
                match reps[k] {
                    None => reps[k] = Some(phi),
                    Some(r) => {
                        diffs_n.push(phi ^ r);
                        diffs_all.push(phi ^ r);
                    }
                }
            }
            for k in 1..half_step {
                if k % 2 == 0 {
                    continue;
                }
                if let (Some(r1), Some(r2)) = (reps[k], reps[k + half_step]) {
                    seps_n.push(r1 ^ r2);
                    seps_all.push(r1 ^ r2);
                }
            }
        }
        let null_n = nullspace(&diffs_n, nfeat);
        let sep_n = separable(&null_n, &seps_n);
        if !first {
            s.push(',');
        }
        first = false;
        write!(
            s,
            "{{\"n\":{},\"curves\":{},\"pairs\":{},\"null_dim\":{},\"separable\":{}}}",
            n,
            used,
            seps_n.len(),
            null_n.len(),
            sep_n
        )
        .unwrap();
        println!(
            "n={} curves={} (k,k+{}) pairs={} coset-constant combos (dim)={} separating combo exists={}",
            n,
            used,
            half_step,
            seps_n.len(),
            null_n.len(),
            sep_n
        );
    }
    let null_all = nullspace(&diffs_all, nfeat);
    let sep_all = separable(&null_all, &seps_all);
    write!(
        s,
        "],\"all\":{{\"pairs\":{},\"null_dim\":{},\"separable\":{}}}}}",
        seps_all.len(),
        null_all.len(),
        sep_all
    )
    .unwrap();
    println!(
        "ALL n: pairs={} coset-constant combos (dim)={} separating combo exists={}",
        seps_all.len(),
        null_all.len(),
        sep_all
    );
    std::fs::write(&out, s).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    CONTROL.store(
        arg(&args, "--control", "0").parse().unwrap(),
        Ordering::Relaxed,
    );
    match args.get(1).map(|s| s.as_str()) {
        Some("selftest") => selftest(),
        Some("exhaustive") => exhaustive(&args),
        Some("random") => random_mode(&args),
        Some("cost") => cost_mode(&args),
        Some("beta") => beta_mode(&args),
        Some("witness") => witness_mode(&args),
        Some("search") => search_mode(&args),
        _ => eprintln!("usage: r7_7 selftest|exhaustive|random|cost|beta [...]"),
    }
}
