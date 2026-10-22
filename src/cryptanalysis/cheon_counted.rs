//! **Cheon's algorithms for the DLP with auxiliary inputs, counted in group
//! operations** on a prime-field curve of word size (`p < 2^62`).
//!
//! Both variants take the target `[α]G` and the leaked points; neither takes
//! `α`.  Every point addition and doubling goes through the repository's
//! [`CountedGroup`] ([`PrimeCurve`], [`GroupOps`]), so the totals are in the
//! unit of `ecbench` (one affine addition or doubling = 1).  Additions with
//! the identity are not counted.  Work that is not a group operation (hash
//! inserts and lookups, scalar-field and `F_{n²}` multiplications, binomial
//! coefficients) is counted under `*_uncharged` names and never dropped
//! (AGENTS.md section 5a).
//!
//! * [`cheon_pminus1`]: `d | n − 1`, inputs `G, [α]G, [α^d]G`.  Baby-step
//!   giant-step in the exponent of a generator of the order-`(n−1)/d`
//!   subgroup of `F_n^*`, then in the order-`d` subgroup
//!   (Cheon, EUROCRYPT 2006, theorem 1).  `comb = true` uses fixed-base comb
//!   tables (Kozaki–Kutsuma–Matsuo), charged; `comb = false` is a full
//!   double-and-add per step, the algorithm of `cheon_attack.rs`.
//! * [`cheon_pplus1`]: `d | n + 1`, inputs `[α^i]G`, `0 ≤ i ≤ 2d`.  The same
//!   search in the norm-one torus of `F_{n²}` (theorem 2), through
//!   `β = ((α−θ)/(α+θ))^d = (A(α) + B(α)θ)/C(α)`.
//!
//! The legacy Python port source is
//! `research/torsion_auxiliary_inputs_20260918/cheon.py`; this module is the
//! native replacement for the rows this study runs.
//!
//! Also here: Mestre-style point counting in `O(p^{1/4})` and the
//! P-224-shape search used to freeze the curves of
//! `research/cheon_p224_shape_20261022/`.

use crate::cryptanalysis::challenge_ladder::is_prime_u64;
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
use std::collections::{BTreeMap, HashMap};
use std::hash::{BuildHasherDefault, Hasher};

type Pt = PrimePoint;

// ── small helpers ────────────────────────────────────────────────────────

#[inline]
fn mulm(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

#[inline]
fn addm(a: u64, b: u64, m: u64) -> u64 {
    let s = a as u128 + b as u128;
    (if s >= m as u128 { s - m as u128 } else { s }) as u64
}

#[inline]
fn subm(a: u64, b: u64, m: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + m - b
    }
}

fn powm(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, m);
        }
        a = mulm(a, a, m);
        e >>= 1;
    }
    r
}

fn invm(a: u64, m: u64) -> u64 {
    powm(a, m - 2, m)
}

/// Integer square root of a `u64` (floor).
pub fn isqrt_u64(n: u64) -> u64 {
    n.isqrt()
}

/// SplitMix64.
pub struct SplitMix(pub u64);

impl SplitMix {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

/// Distinct prime factors of `m`: trial division to `2^21`, then the
/// remainder must be prime (the study's curves have `n ∓ 1 = d·q`, `q`
/// prime).  `None` when a composite remainder is left.
pub fn distinct_prime_factors(mut m: u64) -> Option<Vec<u64>> {
    let mut out = Vec::new();
    let mut f = 2u64;
    while f < (1 << 21) && f * f <= m {
        if m.is_multiple_of(f) {
            out.push(f);
            while m.is_multiple_of(f) {
                m /= f;
            }
        }
        f += if f == 2 { 1 } else { 2 };
    }
    if m > 1 {
        if is_prime_u64(m) {
            out.push(m);
        } else {
            return None;
        }
    }
    Some(out)
}

/// Fast hasher for table keys that are already well-mixed integers.
#[derive(Default)]
pub struct MixHasher(u64);

impl Hasher for MixHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ b as u64).wrapping_mul(0x100_0000_01B3);
        }
    }
    fn write_u64(&mut self, v: u64) {
        let h = (self.0 ^ v).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        self.0 = h ^ (h >> 29);
    }
}

type Table<K> = HashMap<K, u32, BuildHasherDefault<MixHasher>>;
type Key = (u64, u64);

fn key_of(p: &Pt) -> Key {
    if p.infinity {
        (u64::MAX, 0)
    } else {
        (p.x, p.y)
    }
}

// ── counting context ─────────────────────────────────────────────────────

/// Group-operation ledger with phases and `*_uncharged` counters.
pub struct Ctx<'a> {
    pub g: &'a PrimeCurve,
    pub ops: GroupOps,
    /// Scalar multiplications (full double-and-add or comb), for the
    /// exponentiation count of `cheon_attack.rs`.
    pub smul: u64,
    phase: &'static str,
    mark: u64,
    /// Wall-clock limit; the search loops stop with an error past it.
    pub deadline: Option<std::time::Instant>,
    ticks: u64,
    pub phases: BTreeMap<&'static str, u64>,
    pub uncharged: BTreeMap<&'static str, u64>,
}

impl<'a> Ctx<'a> {
    pub fn new(g: &'a PrimeCurve) -> Self {
        Ctx {
            g,
            ops: GroupOps::default(),
            smul: 0,
            phase: "setup",
            mark: 0,
            deadline: None,
            ticks: 0,
            phases: BTreeMap::new(),
            uncharged: BTreeMap::new(),
        }
    }

    pub fn total(&self) -> u64 {
        self.ops.adds + self.ops.doubles
    }

    /// Close the open phase and open `name`.
    pub fn phase(&mut self, name: &'static str) {
        let t = self.total();
        *self.phases.entry(self.phase).or_insert(0) += t - self.mark;
        self.mark = t;
        self.phase = name;
    }

    /// True once the deadline has passed (checked every 1024 calls).
    pub fn expired(&mut self) -> bool {
        self.ticks += 1;
        self.ticks.is_multiple_of(1024)
            && self
                .deadline
                .is_some_and(|d| std::time::Instant::now() >= d)
    }

    pub fn finish(&mut self) {
        self.phase("done");
    }

    pub fn unc(&mut self, name: &'static str, k: u64) {
        *self.uncharged.entry(name).or_insert(0) += k;
    }

    pub fn add(&mut self, p: Pt, q: Pt) -> Pt {
        if p.infinity {
            q
        } else if q.infinity {
            p
        } else {
            self.g.add(&mut self.ops, p, q)
        }
    }

    pub fn dbl(&mut self, p: Pt) -> Pt {
        if p.infinity {
            p
        } else {
            self.g.double(&mut self.ops, p)
        }
    }

    /// `[k]P` by left-to-right double-and-add.
    pub fn mul(&mut self, k: u64, p: Pt) -> Pt {
        self.smul += 1;
        if k == 0 || p.infinity {
            return PrimePoint::INFINITY;
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.dbl(acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(acc, p);
            }
        }
        acc
    }
}

// ── fixed-base multiplier ────────────────────────────────────────────────

/// Comb width minimising table plus per-step cost for `steps` products of
/// `nbits`-bit scalars (the cost model of the legacy harness).
pub fn comb_width(nbits: u32, steps: u64) -> u32 {
    let mut best = (u64::MAX, 2u32);
    for w in 2..=12u32 {
        let chunks = nbits.div_ceil(w) as u64;
        let cost = chunks * ((1u64 << w) - 1) + steps * (chunks - 1);
        if cost < best.0 {
            best = (cost, w);
        }
    }
    best.1
}

/// Fixed-base multiplier: a comb (table charged at construction) or a plain
/// double-and-add.
pub struct Fixed {
    base: Pt,
    w: u32,
    chunks: u32,
    table: Vec<Pt>, // chunks × (2^w − 1), entry v−1 of chunk i = v·2^{wi}·P
    pub table_points: u64,
}

impl Fixed {
    pub fn new(ctx: &mut Ctx, base: Pt, nbits: u32, steps: u64, comb: bool) -> Self {
        if !comb {
            return Fixed {
                base,
                w: 0,
                chunks: 0,
                table: Vec::new(),
                table_points: 0,
            };
        }
        let w = comb_width(nbits, steps);
        let chunks = nbits.div_ceil(w);
        let row = (1usize << w) - 1;
        let mut table = Vec::with_capacity(chunks as usize * row);
        let mut b = base;
        for i in 0..chunks {
            let mut cur = b;
            table.push(cur);
            for _ in 2..=row {
                cur = ctx.add(cur, b);
                table.push(cur);
            }
            if i + 1 < chunks {
                for _ in 0..w {
                    b = ctx.dbl(b);
                }
            }
        }
        let table_points = table.len() as u64;
        Fixed {
            base,
            w,
            chunks,
            table,
            table_points,
        }
    }

    pub fn mul(&self, ctx: &mut Ctx, k: u64) -> Pt {
        if self.w == 0 {
            return ctx.mul(k, self.base);
        }
        ctx.smul += 1;
        let mask = (1u64 << self.w) - 1;
        let row = (1usize << self.w) - 1;
        let mut r = PrimePoint::INFINITY;
        for i in 0..self.chunks {
            let v = (k >> (self.w * i)) & mask;
            if v != 0 {
                r = ctx.add(r, self.table[i as usize * row + (v as usize - 1)]);
            }
        }
        r
    }
}

// ── outcome ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct Outcome {
    pub alpha: Option<u64>,
    /// Group operations (additions plus doublings), every phase.
    pub ops: u64,
    pub adds: u64,
    pub doubles: u64,
    /// Scalar multiplications (exponentiations in `cheon_attack.rs`'s unit).
    pub exponentiations: u64,
    pub table_points: u64,
    pub phases: BTreeMap<String, u64>,
    pub uncharged: BTreeMap<String, u64>,
    pub error: Option<String>,
}

fn outcome(mut ctx: Ctx, alpha: Option<u64>, table_points: u64, error: Option<String>) -> Outcome {
    ctx.finish();
    Outcome {
        alpha,
        ops: ctx.total(),
        adds: ctx.ops.adds,
        doubles: ctx.ops.doubles,
        exponentiations: ctx.smul,
        table_points,
        phases: ctx
            .phases
            .iter()
            .filter(|(_, v)| **v > 0)
            .map(|(k, v)| (k.to_string(), *v))
            .collect(),
        uncharged: ctx
            .uncharged
            .iter()
            .map(|(k, v)| (k.to_string(), *v))
            .collect(),
        error,
    }
}

// ── n − 1 variant ────────────────────────────────────────────────────────

/// Smallest primitive root of the prime `n`, given the prime factors of
/// `n − 1`.
fn primitive_root(n: u64, factors: &[u64]) -> u64 {
    let mut g = 2u64;
    loop {
        if factors.iter().all(|&q| powm(g, (n - 1) / q, n) != 1) {
            return g;
        }
        g += 1;
    }
}

/// `k ∈ [0, order)` with `target = [gen^k]·base`, `gen ∈ F_n^*` of
/// multiplicative order `order`.  Baby steps `[gen^i]base`, giant steps
/// `[gen^{−mj}]target`; every step is one fixed-base multiplication.
#[allow(clippy::too_many_arguments)]
fn bsgs_exponent(
    ctx: &mut Ctx,
    base: &Fixed,
    target: &Fixed,
    gen: u64,
    order: u64,
    n: u64,
    baby: &'static str,
    giant: &'static str,
) -> Option<u64> {
    let m = isqrt_u64(order) + 1;
    ctx.phase(baby);
    let mut table: Table<Key> = Table::default();
    table.reserve(m as usize);
    let mut z = 1u64;
    for i in 0..m {
        if ctx.expired() {
            return None;
        }
        let r = base.mul(ctx, z);
        table.entry(key_of(&r)).or_insert(i as u32);
        ctx.unc("table_insert_uncharged", 1);
        z = mulm(z, gen, n);
        ctx.unc("scalar_mulmod_uncharged", 1);
    }
    ctx.phase(giant);
    let step = powm(invm(gen, n), m, n);
    let mut z = 1u64;
    for j in 0..=m {
        if ctx.expired() {
            return None;
        }
        let r = target.mul(ctx, z);
        ctx.unc("table_lookup_uncharged", 1);
        if let Some(&i) = table.get(&key_of(&r)) {
            return Some((i as u64 + m * j) % order);
        }
        z = mulm(z, step, n);
        ctx.unc("scalar_mulmod_uncharged", 1);
    }
    None
}

/// Cheon's `n − 1` algorithm.  `d` must divide `n − 1`; `n` is the prime
/// order of `g0`; `ag = [α]G`; `adg = [α^d]G`.
#[allow(clippy::too_many_arguments)]
pub fn cheon_pminus1(
    curve: &PrimeCurve,
    n: u64,
    d: u64,
    g0: Pt,
    ag: Pt,
    adg: Pt,
    comb: bool,
    deadline: Option<std::time::Instant>,
) -> Outcome {
    let mut ctx = Ctx::new(curve);
    ctx.deadline = deadline;
    if d == 0 || !(n - 1).is_multiple_of(d) {
        return outcome(ctx, None, 0, Some("d does not divide n - 1".into()));
    }
    let Some(fac) = distinct_prime_factors(n - 1) else {
        return outcome(ctx, None, 0, Some("n - 1 not factored".into()));
    };
    let nbits = 64 - n.leading_zeros();
    let zeta = primitive_root(n, &fac);
    let big_n = (n - 1) / d;
    let zeta_hat = powm(zeta, d, n);
    let mut tables = 0u64;

    // Step 1: α^d = ζ̂^{k0}.
    ctx.phase("p-1:precompute");
    let m1 = isqrt_u64(big_n) + 1;
    let base = Fixed::new(&mut ctx, g0, nbits, m1, comb);
    let targ = Fixed::new(&mut ctx, adg, nbits, m1, comb);
    tables += base.table_points + targ.table_points;
    let Some(k0) = bsgs_exponent(
        &mut ctx,
        &base,
        &targ,
        zeta_hat,
        big_n,
        n,
        "p-1:step1:baby",
        "p-1:step1:giant",
    ) else {
        return outcome(
            ctx,
            None,
            tables,
            Some("step 1 found no match (or deadline)".into()),
        );
    };
    drop((base, targ));

    // Step 2: α ζ^{−k0} = ζ̌^{k1}, ζ̌ = ζ^N of order d.
    ctx.phase("p-1:step2:precompute");
    let m2 = isqrt_u64(d) + 1;
    let shift = powm(invm(zeta, n), k0, n);
    let shifted = ctx.mul(shift, ag);
    let zeta_check = powm(zeta, big_n, n);
    let base2 = Fixed::new(&mut ctx, g0, nbits, m2, comb);
    let targ2 = Fixed::new(&mut ctx, shifted, nbits, m2, comb);
    tables += base2.table_points + targ2.table_points;
    let Some(k1) = bsgs_exponent(
        &mut ctx,
        &base2,
        &targ2,
        zeta_check,
        d,
        n,
        "p-1:step2:baby",
        "p-1:step2:giant",
    ) else {
        return outcome(
            ctx,
            None,
            tables,
            Some("step 2 found no match (or deadline)".into()),
        );
    };
    let alpha = powm(
        zeta,
        (k0 as u128 + k1 as u128 * big_n as u128).rem_euclid(n as u128 - 1) as u64,
        n,
    );
    ctx.phase("p-1:verify");
    let ok = ctx.mul(alpha, g0) == ag;
    if ok {
        outcome(ctx, Some(alpha), tables, None)
    } else {
        outcome(ctx, None, tables, Some("verification failed".into()))
    }
}

// ── n + 1 variant ────────────────────────────────────────────────────────

/// `F_{n²} = F_n[θ]/(θ² − a)`, elements `(s, t) = s + tθ`.
#[derive(Clone, Copy)]
struct Fp2 {
    n: u64,
    a: u64,
}

type E2 = (u64, u64);

impl Fp2 {
    fn mul(&self, x: E2, y: E2) -> E2 {
        let n = self.n;
        (
            addm(mulm(x.0, y.0, n), mulm(self.a, mulm(x.1, y.1, n), n), n),
            addm(mulm(x.0, y.1, n), mulm(x.1, y.0, n), n),
        )
    }
    fn pow(&self, mut x: E2, mut e: u64) -> E2 {
        let mut r = (1, 0);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, x);
            }
            x = self.mul(x, x);
            e >>= 1;
        }
        r
    }
    fn inv(&self, x: E2) -> E2 {
        let n = self.n;
        let nm = subm(mulm(x.0, x.0, n), mulm(self.a, mulm(x.1, x.1, n), n), n);
        let ni = invm(nm, n);
        (mulm(x.0, ni, n), mulm(subm(0, x.1, n), ni, n))
    }
}

/// `Σ [c_i]P_i`, each term by double-and-add, zero coefficients skipped.
fn multi_scalar(ctx: &mut Ctx, coeffs: &[u64], pts: &[Pt]) -> Pt {
    let mut r = PrimePoint::INFINITY;
    for (&c, &p) in coeffs.iter().zip(pts) {
        if c != 0 {
            let t = ctx.mul(c, p);
            r = ctx.add(r, t);
        }
    }
    r
}

/// Cheon's `n + 1` algorithm.  `pows[i] = [α^i]G` for `0 ≤ i ≤ 2d`, `d | n+1`.
/// The target `[α]G` is `pows[1]`.
pub fn cheon_pplus1(
    curve: &PrimeCurve,
    n: u64,
    d: u64,
    pows: &[Pt],
    deadline: Option<std::time::Instant>,
) -> Outcome {
    let mut ctx = Ctx::new(curve);
    ctx.deadline = deadline;
    if d == 0 || !(n + 1).is_multiple_of(d) || (pows.len() as u64) < 2 * d + 1 {
        return outcome(ctx, None, 0, Some("bad d or too few powers".into()));
    }
    let Some(fac) = distinct_prime_factors(n + 1) else {
        return outcome(ctx, None, 0, Some("n + 1 not factored".into()));
    };
    let nbits = 64 - n.leading_zeros();
    let g0 = pows[0];
    let g1 = pows[1];
    // non-residue a
    let mut a = 2u64;
    while powm(a, (n - 1) / 2, n) != n - 1 {
        a += 1;
    }
    let f = Fp2 { n, a };
    // generator ζ of the norm-one torus (order n + 1): x^{n−1}
    let mut rng = SplitMix(n ^ 0x5A5A_1234);
    let zeta = loop {
        let x = (rng.next() % n, 1 + rng.next() % (n - 1));
        let z = f.pow(x, n - 1);
        if fac.iter().all(|&q| f.pow(z, (n + 1) / q) != (1, 0)) {
            break z;
        }
    };
    let big_n = (n + 1) / d;
    let neg_exp = |k: u64, ord: u64| (ord - k % ord) % ord;
    let mut tables = 0u64;

    // Polynomials A, B, C in the indeterminate x = α, mod n.
    ctx.phase("p+1:poly");
    let dd = d as usize;
    let mut fact = vec![1u64; 2 * dd + 1];
    for i in 1..=2 * dd {
        fact[i] = mulm(fact[i - 1], i as u64, n);
    }
    let mut ifact = vec![1u64; 2 * dd + 1];
    ifact[2 * dd] = invm(fact[2 * dd], n);
    for i in (1..=2 * dd).rev() {
        ifact[i - 1] = mulm(ifact[i], i as u64, n);
    }
    let binom = |top: usize, k: usize| mulm(fact[top], mulm(ifact[k], ifact[top - k], n), n);
    let mut apow = vec![1u64; dd + 1]; // a^j
    for j in 1..=dd {
        apow[j] = mulm(apow[j - 1], a, n);
    }
    ctx.unc("scalar_mulmod_uncharged", 8 * (2 * d + 1));
    let mut ca = vec![0u64; 2 * dd + 1];
    let mut cb = vec![0u64; 2 * dd + 1];
    let mut cc = vec![0u64; 2 * dd + 1];
    for i in 0..=2 * dd {
        let e = 2 * dd - i;
        let mut c = mulm(binom(2 * dd, i), apow[e / 2], n);
        if e % 2 == 1 {
            c = subm(0, c, n);
        }
        if e % 2 == 0 {
            ca[i] = c;
        } else {
            cb[i] = c;
        }
    }
    for j in 0..=dd {
        let mut c = mulm(binom(dd, j), apow[dd - j], n);
        if (dd - j) % 2 == 1 {
            c = subm(0, c, n);
        }
        cc[2 * j] = c;
    }
    let pw = &pows[..2 * dd + 1];
    let ga = multi_scalar(&mut ctx, &ca, pw);
    let gb = multi_scalar(&mut ctx, &cb, pw);
    let gc = multi_scalar(&mut ctx, &cc, pw);

    // k with (A + Bθ)/C = ζ̂^k from [A]G, [B]G, [C]G.
    #[allow(clippy::too_many_arguments)]
    fn search(
        ctx: &mut Ctx,
        f: &Fp2,
        nbits: u32,
        zeta_hat: E2,
        order: u64,
        pa: Pt,
        pb: Pt,
        pc: Pt,
        tables: &mut u64,
        names: [&'static str; 3],
    ) -> Option<u64> {
        let m = isqrt_u64(order) + 1;
        let n = f.n;
        ctx.phase(names[0]);
        let mc = Fixed::new(ctx, pc, nbits, 2 * m, true);
        let ma = Fixed::new(ctx, pa, nbits, 2 * m, true);
        let mb = Fixed::new(ctx, pb, nbits, 2 * m, true);
        *tables += mc.table_points + ma.table_points + mb.table_points;
        ctx.phase(names[1]);
        let mut table: Table<(u64, u64)> = Table::default();
        table.reserve(m as usize);
        let mut z: E2 = (1, 0);
        let mix = |p: &Pt| {
            let (x, y) = key_of(p);
            x.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ y.rotate_left(32)
        };
        for u in 0..m {
            if ctx.expired() {
                return None;
            }
            let c0 = mc.mul(ctx, z.0);
            let c1 = mc.mul(ctx, z.1);
            // exact two-point key: fold each point into one word, then the pair
            table.entry((mix(&c0), mix(&c1))).or_insert(u as u32);
            ctx.unc("table_insert_uncharged", 1);
            z = f.mul(z, zeta_hat);
            ctx.unc("fp2_mul_uncharged", 1);
        }
        ctx.phase(names[2]);
        let step = f.pow(zeta_hat, neg_pow(m, order));
        let mut z: E2 = (1, 0);
        for v in 0..=m {
            if ctx.expired() {
                return None;
            }
            let (s, t) = z;
            let l0a = ma.mul(ctx, s);
            let l0b = mb.mul(ctx, mulm(f.a, t, n));
            let l0 = ctx.add(l0a, l0b);
            let l1a = ma.mul(ctx, t);
            let l1b = mb.mul(ctx, s);
            let l1 = ctx.add(l1a, l1b);
            ctx.unc("table_lookup_uncharged", 1);
            if let Some(&k) = table.get(&(mix(&l0), mix(&l1))) {
                return Some((k as u64 + m * v) % order);
            }
            z = f.mul(z, step);
            ctx.unc("fp2_mul_uncharged", 1);
        }
        None
    }
    fn neg_pow(m: u64, order: u64) -> u64 {
        (order - m % order) % order
    }

    let zeta_hat = f.pow(zeta, d);
    let Some(k0) = search(
        &mut ctx,
        &f,
        nbits,
        zeta_hat,
        big_n,
        ga,
        gb,
        gc,
        &mut tables,
        ["p+1:step1:precompute", "p+1:step1:baby", "p+1:step1:giant"],
    ) else {
        return outcome(
            ctx,
            None,
            tables,
            Some("step 1 found no match (or deadline)".into()),
        );
    };

    // Step 2: γ = (α−θ)/(α+θ) = (A'+B'θ)/C' with A' = x²+a, B' = −2x,
    // C' = x²−a (degree 2), then γ ζ^{−k0} = ζ̌^{k1}.
    ctx.phase("p+1:step2:shift");
    let ga1 = multi_scalar(&mut ctx, &[a, 0, 1], &pows[..3]);
    let gb1 = multi_scalar(&mut ctx, &[0, subm(0, 2, n)], &pows[..2]);
    let gc1 = multi_scalar(&mut ctx, &[subm(0, a, n), 0, 1], &pows[..3]);
    let (s, t) = f.pow(zeta, neg_exp(k0, n + 1));
    let ga2a = ctx.mul(s, ga1);
    let ga2b = ctx.mul(mulm(a, t, n), gb1);
    let ga2 = ctx.add(ga2a, ga2b);
    let gb2a = ctx.mul(t, ga1);
    let gb2b = ctx.mul(s, gb1);
    let gb2 = ctx.add(gb2a, gb2b);
    let Some(k1) = search(
        &mut ctx,
        &f,
        nbits,
        f.pow(zeta, big_n),
        d,
        ga2,
        gb2,
        gc1,
        &mut tables,
        ["p+1:step2:precompute", "p+1:step2:baby", "p+1:step2:giant"],
    ) else {
        return outcome(
            ctx,
            None,
            tables,
            Some("step 2 found no match (or deadline)".into()),
        );
    };
    let k = ((k0 as u128 + k1 as u128 * big_n as u128) % (n as u128 + 1)) as u64;
    let gamma = f.pow(zeta, k);
    // α = θ(1 + γ)/(1 − γ) must land in F_n.
    let num = f.mul((0, 1), (addm(1, gamma.0, n), gamma.1));
    let den = (subm(1, gamma.0, n), subm(0, gamma.1, n));
    let mut alpha = None;
    if den != (0, 0) {
        let cand = f.mul(num, f.inv(den));
        if cand.1 == 0 {
            alpha = Some(cand.0);
        }
    }
    ctx.phase("p+1:verify");
    match alpha {
        Some(al) if ctx.mul(al, g0) == g1 => outcome(ctx, Some(al), tables, None),
        _ => outcome(ctx, None, tables, Some("verification failed".into())),
    }
}

/// The leaked points `[α^i]G`, `0 ≤ i ≤ q`, as the problem hands them over
/// (uncharged).
pub fn leaked_powers(curve: &PrimeCurve, n: u64, g0: Pt, alpha: u64, q: u64) -> Vec<Pt> {
    let mut ops = GroupOps::default();
    let mut out = Vec::with_capacity(q as usize + 1);
    let mut e = 1u64;
    for _ in 0..=q {
        out.push(scalar_mul_plain(curve, &mut ops, e, g0));
        e = mulm(e, alpha, n);
    }
    out
}

/// `[k]P` without charging a context (set-up and verification code).
pub fn scalar_mul_plain(curve: &PrimeCurve, ops: &mut GroupOps, k: u64, p: Pt) -> Pt {
    let mut acc = PrimePoint::INFINITY;
    for i in (0..64 - k.leading_zeros()).rev() {
        if !acc.infinity {
            acc = curve.double(ops, acc);
        }
        if (k >> i) & 1 == 1 {
            acc = if acc.infinity {
                p
            } else if p.infinity {
                acc
            } else {
                curve.add(ops, acc, p)
            };
        }
    }
    acc
}

// ── curve search and counting ────────────────────────────────────────────

/// `#E(F_p)` when prime, else `None`: baby-step giant-step over the Hasse
/// interval from a fixed point, `O(p^{1/4})` operations.  A unique hit that
/// is prime is the group order (`2·hit` lies above the interval for
/// `p > 37`).
pub fn prime_order_bsgs(curve: &PrimeCurve) -> Option<u64> {
    let p = curve.p;
    let pt = (1..p).find_map(|x| {
        let y = curve.sqrt(curve.rhs(x))?;
        (y != 0).then(|| PrimePoint::affine(x, y.min(p - y)))
    })?;
    let t = (4 * p).isqrt();
    let lo = p + 1 - t;
    let width = 2 * t + 1;
    let s = width.isqrt() + 1;
    let mut ops = GroupOps::default();
    // baby: jP, 0 ≤ j < s
    let mut table: Table<Key> = Table::default();
    table.reserve(s as usize);
    let mut cur = PrimePoint::INFINITY;
    for j in 0..s {
        if table.insert(key_of(&cur), j as u32).is_some() {
            return None; // ord(P) ≤ s: not the prime-order case
        }
        cur = if cur.infinity {
            pt
        } else {
            curve.add(&mut ops, cur, pt)
        };
    }
    let step = scalar_mul_plain(curve, &mut ops, s, pt);
    let mut g = scalar_mul_plain(curve, &mut ops, lo, pt);
    let mut hit: Option<u64> = None;
    for i in 0..=(width / s) {
        let neg = curve.neg(g);
        if let Some(&j) = table.get(&key_of(&neg)) {
            let m = lo + i * s + j as u64;
            if m <= p + 1 + t {
                if hit.is_some() {
                    return None;
                }
                hit = Some(m);
            }
        }
        g = if g.infinity {
            step
        } else if step.infinity {
            g
        } else {
            curve.add(&mut ops, g, step)
        };
    }
    hit.filter(|&m| is_prime_u64(m))
}

/// Which side of `n` carries the smooth part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Minus,
    Plus,
}

/// `d` with `d | n ∓ 1`, `(n ∓ 1)/d` prime and `|log d / log n − ratio| ≤
/// tol`, the closest to `ratio`; `None` if there is none.
pub fn shape_divisor(n: u64, side: Side, ratio: f64, tol: f64) -> Option<u64> {
    let m = match side {
        Side::Minus => n - 1,
        Side::Plus => n + 1,
    };
    let ln = (n as f64).ln();
    let lo = ((ratio - tol) * ln).exp().floor().max(2.0) as u64;
    let hi = ((ratio + tol) * ln).exp().ceil() as u64;
    let mut best: Option<(f64, u64)> = None;
    for d in lo..=hi {
        if d < 2 || !m.is_multiple_of(d) {
            continue;
        }
        let q = m / d;
        if q < 3 || !is_prime_u64(q) {
            continue;
        }
        let r = (d as f64).ln() / ln;
        if (r - ratio).abs() > tol {
            continue;
        }
        let dist = (r - ratio).abs();
        if best.is_none_or(|(b, _)| dist < b) {
            best = Some((dist, d));
        }
    }
    best.map(|(_, d)| d)
}

/// Least prime `≥ x`.
pub fn next_prime(mut x: u64) -> u64 {
    if x.is_multiple_of(2) {
        x += 1;
    }
    while !is_prime_u64(x) {
        x += 2;
    }
    x
}

/// A frozen curve: the output of [`find_shaped_curve`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shaped {
    pub p: u64,
    pub a: u64,
    pub b: u64,
    pub n: u64,
    pub d: u64,
    pub gx: u64,
    pub gy: u64,
    /// Coefficient pairs tried (including the accepted one).
    pub tried: u64,
}

/// First curve, in stream order, with prime order `n ≠ p` and the shape
/// condition of `side` (see `PROTOCOL.md` section 2.2).
pub fn find_shaped_curve(p: u64, seed: u64, side: Side, ratio: f64, tol: f64) -> Shaped {
    let mut rng = SplitMix(seed);
    let mut tried = 0u64;
    loop {
        let a = 1 + rng.next() % (p - 1);
        let b = 1 + rng.next() % (p - 1);
        // non-singular: 4a³ + 27b² ≠ 0
        let disc = addm(
            mulm(4, mulm(a, mulm(a, a, p), p), p),
            mulm(27, mulm(b, b, p), p),
            p,
        );
        if disc == 0 {
            continue;
        }
        tried += 1;
        let curve = PrimeCurve { p, a, b };
        let Some(n) = prime_order_bsgs(&curve) else {
            continue;
        };
        if n == p {
            continue;
        }
        let Some(d) = shape_divisor(n, side, ratio, tol) else {
            continue;
        };
        let (gx, gy) = (1..p)
            .find_map(|x| {
                let y = curve.sqrt(curve.rhs(x))?;
                (y != 0).then(|| (x, y.min(p - y)))
            })
            .expect("a point exists");
        return Shaped {
            p,
            a,
            b,
            n,
            d,
            gx,
            gy,
            tried,
        };
    }
}

// ── tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn small(side: Side, ratio: f64) -> (PrimeCurve, Shaped) {
        let p = next_prime(3 << 17);
        let s = find_shaped_curve(p, 77, side, ratio, 0.06);
        (
            PrimeCurve {
                p: s.p,
                a: s.a,
                b: s.b,
            },
            s,
        )
    }

    #[test]
    fn bsgs_count_matches_brute_force() {
        let c = PrimeCurve {
            p: 100003,
            a: 5,
            b: 7,
        };
        let brute = c.point_count();
        if is_prime_u64(brute) {
            assert_eq!(prime_order_bsgs(&c), Some(brute));
        } else {
            assert_eq!(prime_order_bsgs(&c), None);
        }
        // a prime-order curve is found and agrees with the O(p) count
        let (c, s) = small(Side::Minus, 0.2);
        assert_eq!(c.point_count(), s.n);
        assert!(is_prime_u64(s.n));
        assert!(shape_divisor(s.n, Side::Minus, 0.2, 0.06).is_some());
    }

    #[test]
    fn pminus1_recovers_alpha_and_comb_equals_naive_result() {
        let (c, s) = small(Side::Minus, 0.2);
        let g0 = PrimePoint::affine(s.gx, s.gy);
        let mut ops = GroupOps::default();
        for alpha in [2u64, 3, 12345, s.n - 2, s.n / 2 + 1] {
            let ad = powm(alpha, s.d, s.n);
            let ag = scalar_mul_plain(&c, &mut ops, alpha, g0);
            let adg = scalar_mul_plain(&c, &mut ops, ad, g0);
            for comb in [true, false] {
                let o = cheon_pminus1(&c, s.n, s.d, g0, ag, adg, comb, None);
                assert_eq!(o.alpha, Some(alpha), "comb={comb} {:?}", o.error);
                assert_eq!(o.ops, o.phases.values().sum::<u64>());
            }
        }
    }

    #[test]
    fn pminus1_needs_the_auxiliary_input() {
        let (c, s) = small(Side::Minus, 0.2);
        let g0 = PrimePoint::affine(s.gx, s.gy);
        let mut ops = GroupOps::default();
        let alpha = 777u64;
        let ag = scalar_mul_plain(&c, &mut ops, alpha, g0);
        let bogus = scalar_mul_plain(&c, &mut ops, 5, g0);
        let o = cheon_pminus1(&c, s.n, s.d, g0, ag, bogus, true, None);
        assert_ne!(o.alpha, Some(alpha));
    }

    #[test]
    fn pplus1_recovers_alpha() {
        let p = next_prime(3 << 15);
        let s = find_shaped_curve(p, 91, Side::Plus, 0.2, 0.08);
        let c = PrimeCurve {
            p: s.p,
            a: s.a,
            b: s.b,
        };
        let g0 = PrimePoint::affine(s.gx, s.gy);
        for alpha in [2u64, 99, 31337, s.n - 3] {
            let pows = leaked_powers(&c, s.n, g0, alpha, 2 * s.d);
            let o = cheon_pplus1(&c, s.n, s.d, &pows, None);
            assert_eq!(o.alpha, Some(alpha), "{:?}", o.error);
        }
    }

    #[test]
    fn pplus1_wrong_powers_do_not_recover() {
        let p = next_prime(3 << 15);
        let s = find_shaped_curve(p, 91, Side::Plus, 0.2, 0.08);
        let c = PrimeCurve {
            p: s.p,
            a: s.a,
            b: s.b,
        };
        let g0 = PrimePoint::affine(s.gx, s.gy);
        let mut pows = leaked_powers(&c, s.n, g0, 4242, 2 * s.d);
        let other = leaked_powers(&c, s.n, g0, 4243, 2 * s.d);
        // powers of a different secret, same target slot
        pows[2..].copy_from_slice(&other[2..]);
        let o = cheon_pplus1(&c, s.n, s.d, &pows, None);
        assert_ne!(o.alpha, Some(4242));
    }
}
