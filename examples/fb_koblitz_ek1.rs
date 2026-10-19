//! **E-K1: a Gauss-period window in `s = x + 1/x` on Koblitz analogues.**
//!
//! Runs the design preregistered in Amendment 6 of
//! `research/factor_base_designs_20261015/PREREGISTRATION.md` on the toy
//! Koblitz curve `K_0` over `F_{2^n}`, `n ∈ {11, 23}` (`2n + 1` prime, `2`
//! of order `n` mod `2n + 1`, so the Gauss periods `c_i = ζ^i + ζ^{-i}`,
//! `ζ^{2n+1} = 1`, `1 ≤ i ≤ n`, form a type-II normal basis).
//!
//! **Instrument.** `pq_f4_f2::groebner_basis_f4`, the repository's Boolean
//! F4 over `F_2[v]/(v² − v)`, on the Weil restriction of each decomposition
//! system. `D` is its `solving_degree`, `field_ops` is its `word_xors` (64-bit
//! XORs in the row reductions, the unit the engine counts). The same engine,
//! budget and counters score every arm.
//!
//! Arms (one planted decomposition per target, `m` summands):
//!
//! - `S`: `s(P) ∈ G_ℓ = span(c_1..c_ℓ)`, Semaev's polynomial in the `s_i`.
//! - `X`: `x(P) ∈ G_ℓ`, the same window in `x`.
//! - `XR`: `x(P) ∈ R_ℓ`, a random `ℓ`-dimensional subspace (the repository's
//!   standard `x`-window).
//! - `U`: `u(P) ∈ V`, `u = 1/(x + 1)`, `V ∋ 1` a random `ℓ`-dimensional
//!   subspace (the repository's `u`-frame, `koblitz_symmetrised`).
//! - `NS`: the `S` system with random field coefficients on the same
//!   monomial support and a constant shifted so the planted point is a zero.
//! - `PC`: the `X` window as a chained system (`m = 3` only), the
//!   positive-control presentation.
//!
//! Every set, presentation and planted solution is checked before anything is
//! measured; a failed check panics and is never a measurement.
//!
//! ```text
//! cargo build --release --example fb_koblitz_ek1
//! B=/home/user/cargo-target/release/examples/fb_koblitz_ek1
//! $B check                     # sets, identity, presentations, planted solutions
//! $B one --arm S --n 11 --m 2 --ell 4 --target 0
//! $B run --n 11,23 --m 2,3 --out-dir research/factor_base_designs_20261015/runs
//! $B replay research/factor_base_designs_20261015/runs --out K1-replay.jsonl
//! $B analyze research/factor_base_designs_20261015/runs
//! ```

use std::collections::{BTreeMap, HashMap};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crypto_lib::binary_ecc::curve::BinaryCurve;
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::koblitz_groebner::{
    build_decomposition_system, FieldStructure, SymElement,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::{
    find_irreducible_sparse, koblitz_point_count, points_with_x, KoblitzCurve,
};
use crypto_lib::cryptanalysis::koblitz_symmetrised::{
    artin_schreier, build_symmetrised_system, plain_terms, sym_polynomial, symmetrised_terms,
    SymmetrisedFactorBase,
};
use crypto_lib::cryptanalysis::pq_f4_f2::groebner_basis_f4;
use crypto_lib::cryptanalysis::pq_groebner_f2::F2BoolPoly;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

/// `"EK1"`.
const SEED: u64 = 0x0045_4B31;
const MAX_VARS: usize = 64;

// ── Arms ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Arm {
    S,
    X,
    XR,
    U,
    NS,
    PC,
}

impl Arm {
    const ALL: [Arm; 6] = [Arm::S, Arm::X, Arm::XR, Arm::U, Arm::NS, Arm::PC];
    fn name(self) -> &'static str {
        match self {
            Arm::S => "S",
            Arm::X => "X",
            Arm::XR => "XR",
            Arm::U => "U",
            Arm::NS => "NS",
            Arm::PC => "PC",
        }
    }
    fn id(self) -> u64 {
        match self {
            Arm::S => 0,
            Arm::X => 1,
            Arm::XR => 2,
            Arm::U => 3,
            Arm::NS => 4,
            Arm::PC => 5,
        }
    }
    fn parse(s: &str) -> Arm {
        Arm::ALL
            .into_iter()
            .find(|a| a.name() == s)
            .unwrap_or_else(|| panic!("unknown arm {s}"))
    }
    /// The arm whose planted decomposition this arm shares.
    fn plant_arm(self) -> Arm {
        match self {
            Arm::NS => Arm::S,
            Arm::PC => Arm::X,
            a => a,
        }
    }
    /// Which coordinate the window constrains.
    fn frame(self) -> Frame {
        match self {
            Arm::S | Arm::NS => Frame::S,
            Arm::X | Arm::XR | Arm::PC => Frame::X,
            Arm::U => Frame::U,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Frame {
    S,
    X,
    U,
}

fn cell_seed(n: u32, arm: Arm, m: usize, ell: usize) -> u64 {
    SEED ^ ((n as u64) << 48) ^ (arm.id() << 40) ^ ((m as u64) << 32) ^ ((ell as u64) << 16)
}

fn target_seed(cell: u64, k: usize) -> u64 {
    cell ^ ((k as u64 + 1) << 8)
}

// ── F_{2^n} in a machine word ──────────────────────────────────────

#[derive(Clone)]
struct Gf {
    n: u32,
    /// The modulus with its leading bit.
    modp: u64,
}

impl Gf {
    fn mul(&self, a: u64, b: u64) -> u64 {
        let (mut r, mut a, mut b) = (0u64, a, b);
        while b != 0 {
            if b & 1 == 1 {
                r ^= a;
            }
            b >>= 1;
            a <<= 1;
            if (a >> self.n) & 1 == 1 {
                a ^= self.modp;
            }
        }
        r
    }
    fn pow(&self, a: u64, mut e: u64) -> u64 {
        let (mut r, mut b) = (1u64, a);
        while e > 0 {
            if e & 1 == 1 {
                r = self.mul(r, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        r
    }
    fn inv(&self, a: u64) -> u64 {
        assert!(a != 0);
        self.pow(a, (1u64 << self.n) - 2)
    }
    /// Absolute trace, by definition.
    fn trace(&self, a: u64) -> u64 {
        let (mut t, mut acc) = (a, a);
        for _ in 1..self.n {
            t = self.mul(t, t);
            acc ^= t;
        }
        assert!(acc <= 1, "trace outside F_2");
        acc
    }
    /// A root of `t² + t = v` for odd `n` and `Tr(v) = 0`.
    fn half_trace(&self, v: u64) -> u64 {
        assert!(self.n % 2 == 1);
        let (mut t, mut acc) = (v, v);
        for _ in 0..(self.n - 1) / 2 {
            t = self.mul(t, t);
            t = self.mul(t, t);
            acc ^= t;
        }
        acc
    }
}

/// Discrete-log tables of a primitive element, for fast inverses.
struct Logs {
    exp: Vec<u32>,
    log: Vec<u32>,
    order: u64,
}

impl Logs {
    fn new(gf: &Gf) -> Logs {
        let order = (1u64 << gf.n) - 1;
        let mut primes = Vec::new();
        let mut r = order;
        let mut q = 2u64;
        while q * q <= r {
            if r.is_multiple_of(q) {
                primes.push(q);
                while r.is_multiple_of(q) {
                    r /= q;
                }
            }
            q += 1;
        }
        if r > 1 {
            primes.push(r);
        }
        let g = (2u64..)
            .find(|&g| primes.iter().all(|&q| gf.pow(g, order / q) != 1))
            .expect("a primitive element");
        let mut exp = vec![0u32; order as usize];
        let mut log = vec![0u32; (order + 1) as usize];
        let mut v = 1u64;
        for i in 0..order {
            exp[i as usize] = v as u32;
            log[v as usize] = i as u32;
            v = gf.mul(v, g);
        }
        assert_eq!(v, 1, "the generator's order is 2^n − 1");
        Logs { exp, log, order }
    }
    fn inv(&self, a: u64) -> u64 {
        let l = self.log[a as usize] as u64;
        self.exp[((self.order - l) % self.order) as usize] as u64
    }
}

// ── The field, the curve, the Gauss periods ────────────────────────

struct Ctx {
    n: u32,
    kc: KoblitzCurve,
    st: FieldStructure,
    gf: Gf,
    a_tr: u64,
    tmask: u64,
    zeta: u64,
    /// `c_1, …, c_n`.
    gauss: Vec<u64>,
}

fn el(n: u32, v: u64) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(v), n)
}

fn bits(e: &F2mElement) -> u64 {
    e.raw_bits().first().copied().unwrap_or(0)
}

fn rank(vs: &[u64]) -> usize {
    let mut basis: Vec<u64> = Vec::new();
    for &v in vs {
        let mut x = v;
        for &b in &basis {
            x = x.min(x ^ b);
        }
        if x != 0 {
            basis.push(x);
        }
    }
    basis.len()
}

impl Ctx {
    fn new(n: u32) -> Ctx {
        // `KoblitzCurve::new` rejects K_0 over F_2^11 (#E = 4·23², so r² | #E).
        // Neither the systems nor the planted decompositions use the
        // prime-order subgroup, so the curve is built directly.
        let irr = find_irreducible_sparse(n).expect("an irreducible polynomial");
        let kc = KoblitzCurve {
            a: 0,
            n,
            curve: BinaryCurve {
                m: n,
                irreducible: irr.clone(),
                a: F2mElement::zero(n),
                b: F2mElement::one(n),
                generator: BinaryPoint::Infinity,
                order: BigUint::from(1u32),
                cofactor: BigUint::from(1u32),
            },
            trace: -1,
            group_order: koblitz_point_count(0, n),
            subgroup_order: BigUint::from(1u32),
            cofactor: BigUint::from(1u32),
            lambda: BigUint::from(0u32),
            frobenius_is_endomorphism: true,
            k: 1,
            q: 2,
            a_index: 0,
            b_index: 1,
            subfield_basis: Vec::new(),
        };
        let mut modp = 1u64 << n;
        for &k in &irr.low_terms {
            modp |= 1 << k;
        }
        let gf = Gf { n, modp };
        let st = FieldStructure::new(n, &irr);
        // The word field agrees with the repository's.
        let mut rng = StdRng::seed_from_u64(0x6D75_6C00 + n as u64);
        for _ in 0..200 {
            let (a, b) = (rng.gen_range(1..1u64 << n), rng.gen_range(1..1u64 << n));
            let want = bits(&el(n, a).mul(&el(n, b), &irr));
            assert_eq!(
                gf.mul(a, b),
                want,
                "word multiplication differs from F2mElement"
            );
            assert_eq!(
                gf.inv(a),
                bits(&el(n, a).flt_inverse(&irr).expect("invertible")),
                "word inverse differs"
            );
        }
        let a_tr = gf.trace(bits(&kc.curve.a));
        let tmask = (0..n).fold(0u64, |m, k| m | (gf.trace(1 << k) << k));
        for _ in 0..50 {
            let a = rng.gen_range(1..1u64 << n);
            assert_eq!(
                (a & tmask).count_ones() as u64 & 1,
                gf.trace(a),
                "trace functional"
            );
            assert_eq!(
                gf.trace(a) as u8,
                el(n, a).trace(&irr),
                "trace differs from F2mElement"
            );
        }
        assert_eq!(bits(&kc.curve.b), 1, "b = 1 on K_0");
        // ζ of order p = 2n + 1.
        let p = 2 * n as u64 + 1;
        let total = (1u64 << n) - 1;
        assert!(
            p % 2 == 1 && (3..p).step_by(2).all(|d| !p.is_multiple_of(d)),
            "2n+1 prime"
        );
        assert_eq!(total % p, 0, "ζ ∈ F_2^n needs (2n+1) | 2^n − 1");
        let zeta = (2u64..)
            .map(|g| gf.pow(g, total / p))
            .find(|&z| z != 1)
            .expect("ζ");
        assert_eq!(gf.pow(zeta, p), 1);
        let gauss: Vec<u64> = (1..=n as u64)
            .map(|i| gf.pow(zeta, i) ^ gf.pow(zeta, p - i))
            .collect();
        let ctx = Ctx {
            n,
            kc,
            st,
            gf,
            a_tr,
            tmask,
            zeta,
            gauss,
        };
        ctx.check_gauss();
        ctx
    }

    fn tr(&self, a: u64) -> u64 {
        (a & self.tmask).count_ones() as u64 & 1
    }

    /// Type-II normal basis: independent, Frobenius-permuted, Dickson product rule.
    fn check_gauss(&self) {
        let n = self.n as u64;
        let p = 2 * n + 1;
        assert_eq!(
            rank(&self.gauss),
            self.n as usize,
            "c_1..c_n are independent"
        );
        let c = |k: u64| -> u64 {
            let k = k % p;
            let k = if k > n { p - k } else { k };
            if k == 0 {
                0
            } else {
                self.gauss[(k - 1) as usize]
            }
        };
        for i in 1..=n {
            assert_eq!(self.gf.mul(c(i), c(i)), c(2 * i), "c_i² = c_2i");
            for j in 1..=n {
                let d = i.abs_diff(j);
                assert_eq!(
                    self.gf.mul(c(i), c(j)),
                    c(i + j) ^ c(d),
                    "Dickson product rule c_i c_j = c_(i+j) + c_(i-j)"
                );
            }
        }
        assert_eq!(
            self.gauss.iter().fold(0, |a, &b| a ^ b),
            1,
            "the Gauss periods sum to 1"
        );
    }

    /// Whether `x ≠ 0` is the abscissa of curve points.
    fn on_curve_x(&self, x: u64, inv_x: u64) -> bool {
        self.tr(x) ^ self.tr(inv_x) == self.a_tr
    }
}

fn for_window(basis: &[u64], mut f: impl FnMut(u64)) {
    f(0);
    let mut v = 0u64;
    for i in 1u64..(1 << basis.len()) {
        v ^= basis[i.trailing_zeros() as usize];
        f(v);
    }
}

/// Exact number of affine points whose frame coordinate lies in the window
/// spanned by `basis` (`x = 0` is the point `T`).
fn count_points(ctx: &Ctx, logs: &Logs, frame: Frame, basis: &[u64]) -> u64 {
    let mut total = 0u64;
    for_window(basis, |z| match frame {
        Frame::X => {
            if z == 0 {
                total += 1;
            } else if ctx.on_curve_x(z, logs.inv(z)) {
                total += 2;
            }
        }
        Frame::S => {
            if z == 0 {
                // x = 1 only.
                if ctx.on_curve_x(1, 1) {
                    total += 2;
                }
            } else if ctx.tr(logs.inv(z)) == 0 && ctx.tr(z) == ctx.a_tr {
                // two abscissae x, 1/x, two points each.
                total += 4;
            }
        }
        Frame::U => {
            if z != 0 {
                let x = logs.inv(z) ^ 1;
                if x == 0 {
                    total += 1;
                } else if ctx.on_curve_x(x, logs.inv(x)) {
                    total += 2;
                }
            }
        }
    });
    total
}

/// The same count through `points_with_x` on `F2mElement` (small windows).
fn repo_count(ctx: &Ctx, frame: Frame, basis: &[u64]) -> u64 {
    let n = ctx.n;
    let irr = &ctx.kc.curve.irreducible;
    let mut window: std::collections::HashSet<u64> = std::collections::HashSet::new();
    for_window(basis, |z| {
        window.insert(z);
    });
    let pts = |x: u64| points_with_x(&ctx.kc.curve, &el(n, x)).len() as u64;
    let mut total = 0u64;
    match frame {
        Frame::X => {
            for &x in &window {
                total += pts(x);
            }
        }
        Frame::U => {
            for &u in &window {
                if u != 0 {
                    let xe = el(n, u).flt_inverse(irr).expect("inverse").add(&el(n, 1));
                    total += pts(bits(&xe));
                }
            }
        }
        Frame::S => {
            // Every abscissa x ≠ 0 with x + 1/x in the window.
            if n <= 12 {
                for x in 1..1u64 << n {
                    let s = x ^ ctx.gf.inv(x);
                    if window.contains(&s) {
                        total += pts(x);
                    }
                }
            } else {
                for &s in &window {
                    if s == 0 {
                        total += pts(1);
                    } else if ctx.tr(ctx.gf.inv(s)) == 0 {
                        let t = ctx.gf.half_trace(ctx.gf.mul(ctx.gf.inv(s), ctx.gf.inv(s)));
                        let x = ctx.gf.mul(s, t);
                        let xe = el(n, x);
                        // x + 1/x = s, checked in F2mElement.
                        let back = xe.add(&xe.flt_inverse(irr).expect("inverse"));
                        assert_eq!(bits(&back), s, "x solves x + 1/x = s");
                        total += pts(x) + pts(ctx.gf.inv(x));
                    }
                }
            }
        }
    }
    total
}

// ── Windows ────────────────────────────────────────────────────────

/// A random `ℓ`-dimensional subspace basis, optionally starting with `1`.
fn random_basis(ctx: &Ctx, ell: usize, with_one: bool, rng: &mut StdRng) -> Vec<u64> {
    let mut basis: Vec<u64> = Vec::new();
    if with_one {
        basis.push(1);
    }
    while basis.len() < ell {
        let v = rng.gen_range(1..1u64 << ctx.n);
        let mut cand = basis.clone();
        cand.push(v);
        if rank(&cand) == cand.len() {
            basis.push(v);
        }
    }
    basis
}

fn window_basis(ctx: &Ctx, arm: Arm, m: usize, ell: usize) -> Vec<u64> {
    match arm {
        Arm::S | Arm::X | Arm::NS | Arm::PC => ctx.gauss[..ell].to_vec(),
        Arm::XR => {
            let mut rng = StdRng::seed_from_u64(cell_seed(ctx.n, arm, m, ell) ^ 0x5B5_0000_0000);
            random_basis(ctx, ell, false, &mut rng)
        }
        Arm::U => {
            let mut rng = StdRng::seed_from_u64(cell_seed(ctx.n, arm, m, ell) ^ 0x5B5_0000_0000);
            random_basis(ctx, ell, true, &mut rng)
        }
    }
}

// ── Planting ───────────────────────────────────────────────────────

struct Plant {
    points: Vec<BinaryPoint>,
    /// The frame coordinate of each summand.
    coord: Vec<u64>,
    /// The raw coefficient bits of each summand over the window basis.
    bits: Vec<u64>,
    r: BinaryPoint,
    x_r: u64,
    /// Redraws before this one was accepted.
    redraws: usize,
}

fn x_of(p: &BinaryPoint) -> u64 {
    match p {
        BinaryPoint::Affine { x, .. } => bits(x),
        BinaryPoint::Infinity => panic!("the point at infinity has no abscissa"),
    }
}

/// `m` random factor-base points of the arm's window, summed to the target.
fn plant(ctx: &Ctx, frame: Frame, wb: &[u64], m: usize, rng: &mut StdRng) -> Option<Plant> {
    let ell = wb.len();
    let mut redraws = 0usize;
    'attempt: loop {
        if redraws >= 20_000 {
            return None;
        }
        let mut points = Vec::new();
        let mut coord: Vec<u64> = Vec::new();
        let mut bitv: Vec<u64> = Vec::new();
        for _ in 0..m {
            let b = rng.gen_range(0..1u64 << ell);
            let z = (0..ell).fold(0u64, |a, t| if (b >> t) & 1 == 1 { a ^ wb[t] } else { a });
            let coin: u64 = rng.gen();
            // The abscissa this coordinate value stands for.
            let x = match frame {
                Frame::X => {
                    if z == 0 {
                        redraws += 1;
                        continue 'attempt;
                    }
                    z
                }
                Frame::U => {
                    if z == 0 || z == 1 {
                        redraws += 1;
                        continue 'attempt;
                    }
                    ctx.gf.inv(z) ^ 1
                }
                Frame::S => {
                    if z == 0 || ctx.tr(ctx.gf.inv(z)) != 0 {
                        redraws += 1;
                        continue 'attempt;
                    }
                    let zi = ctx.gf.inv(z);
                    let t = ctx.gf.half_trace(ctx.gf.mul(zi, zi));
                    let x = ctx.gf.mul(z, t);
                    assert_eq!(x ^ ctx.gf.inv(x), z, "x + 1/x = s");
                    if coin & 1 == 1 {
                        ctx.gf.inv(x)
                    } else {
                        x
                    }
                }
            };
            let pts = points_with_x(&ctx.kc.curve, &el(ctx.n, x));
            if pts.is_empty() {
                redraws += 1;
                continue 'attempt;
            }
            let p = pts[((coin >> 1) as usize) % pts.len()].clone();
            if coord.contains(&z) {
                redraws += 1;
                continue 'attempt;
            }
            // For the u-frame the AS invariant must differ too.
            if frame == Frame::U {
                let w = ctx.gf.mul(z, z) ^ z;
                if coord.iter().any(|&c| ctx.gf.mul(c, c) ^ c == w) {
                    redraws += 1;
                    continue 'attempt;
                }
            }
            points.push(p);
            coord.push(z);
            bitv.push(b);
        }
        let mut r = BinaryPoint::Infinity;
        for p in &points {
            r = ctx.kc.add(&r, p);
        }
        let BinaryPoint::Affine { x, .. } = &r else {
            redraws += 1;
            continue 'attempt;
        };
        let x_r = bits(x);
        if x_r == 0 || x_r == 1 {
            redraws += 1;
            continue 'attempt;
        }
        return Some(Plant {
            points,
            coord,
            bits: bitv,
            r,
            x_r,
            redraws,
        });
    }
}

/// `x + 1/x`.
fn s_of(ctx: &Ctx, x: u64) -> u64 {
    x ^ ctx.gf.inv(x)
}

/// `1/(x + 1)`.
fn u_of(ctx: &Ctx, x: u64) -> u64 {
    ctx.gf.inv(x ^ 1)
}

// ── Systems ────────────────────────────────────────────────────────

/// `Σ_terms Π vars^e`, in the word field.
fn eval_terms(gf: &Gf, terms: &[Vec<u32>], vals: &[u64]) -> u64 {
    let mut acc = 0u64;
    for t in terms {
        let mut prod = 1u64;
        for (i, &e) in t.iter().enumerate() {
            if e > 0 {
                prod = gf.mul(prod, gf.pow(vals[i], e as u64));
            }
        }
        acc ^= prod;
    }
    acc
}

fn sym_pow(v: &SymElement, e: u32, st: &FieldStructure) -> SymElement {
    let mut result: Option<SymElement> = None;
    let mut base = v.clone();
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result = Some(match result {
                None => base.clone(),
                Some(r) => r.mul(&base, st),
            });
        }
        e >>= 1;
        if e > 0 {
            base = base.square(st);
        }
    }
    result.expect("positive exponent")
}

/// `Σ_t c_t Π vars^e + shift`, Weil-restricted: the null's equations.
fn sym_polynomial_coef(
    vars: &[SymElement],
    terms: &[Vec<u32>],
    coefs: &[u64],
    shift: u64,
    st: &FieldStructure,
) -> Vec<F2BoolPoly> {
    let n_vars = vars[0].coords[0].n_vars;
    let mut acc = SymElement::constant(&el(st.n, shift), st.n, n_vars);
    for (term, &c) in terms.iter().zip(coefs) {
        if term.iter().all(|&e| e == 0) {
            continue;
        }
        let mut prod: Option<SymElement> = None;
        for (i, &e) in term.iter().enumerate() {
            if e == 0 {
                continue;
            }
            let p = sym_pow(&vars[i], e, st);
            prod = Some(match prod {
                None => p,
                Some(q) => q.mul(&p, st),
            });
        }
        let t = prod
            .expect("a non-constant term")
            .mul(&SymElement::constant(&el(st.n, c), st.n, n_vars), st);
        acc = acc.add(&t);
    }
    acc.coords
}

struct Built {
    eqs: Vec<F2BoolPoly>,
    n_vars: usize,
    assign: u64,
    row: Value,
}

fn max_degree(eqs: &[F2BoolPoly]) -> u32 {
    eqs.iter()
        .flat_map(|p| p.terms.iter())
        .map(|t| t.mask.count_ones())
        .max()
        .unwrap_or(0)
}

fn pack(summand_bits: &[u64], width: usize) -> u64 {
    summand_bits
        .iter()
        .enumerate()
        .fold(0u64, |a, (i, &b)| a | (b << (i * width)))
}

fn layout_limit(arm: Arm, n: u32, m: usize, ell: usize) -> Option<usize> {
    let nv = match arm {
        Arm::U => m * (ell - 1) + 1,
        Arm::PC => m * ell + (m - 2) * n as usize,
        _ => m * ell,
    };
    (nv > MAX_VARS).then_some(nv)
}

/// One system with every check before measuring.
fn build_system(
    ctx: &Ctx,
    logs: Option<&Logs>,
    arm: Arm,
    m: usize,
    ell: usize,
    k: usize,
) -> Result<Built, String> {
    let n = ctx.n;
    let nn = n as usize;
    let cell = cell_seed(n, arm, m, ell);
    let pcell = cell_seed(n, arm.plant_arm(), m, ell);
    let mut rng = StdRng::seed_from_u64(target_seed(pcell, k));
    let wb = window_basis(ctx, arm, m, ell);
    // The set.
    assert_eq!(rank(&wb), ell, "window basis independent");
    if arm == Arm::U {
        assert_eq!(wb[0], 1, "V contains 1");
    }
    let frame = arm.frame();
    let v_size = logs.map(|l| count_points(ctx, l, frame, &wb));
    let Some(pl) = plant(ctx, frame, &wb, m, &mut rng) else {
        return Err(format!(
            "no planted decomposition with {m} distinct summands in this window after 20000 draws"
        ));
    };
    let basis_el: Vec<F2mElement> = wb.iter().map(|&v| el(n, v)).collect();
    // Planted points: on the curve, summing to R, coordinate matches the window.
    let mut acc = BinaryPoint::Infinity;
    for (i, p) in pl.points.iter().enumerate() {
        let BinaryPoint::Affine { x, y } = p else {
            panic!("planted point at infinity")
        };
        let (xb, yb) = (bits(x), bits(y));
        // y² + xy = x³ + a x² + 1.
        let lhs = ctx.gf.mul(yb, yb) ^ ctx.gf.mul(xb, yb);
        let x2 = ctx.gf.mul(xb, xb);
        let rhs = ctx.gf.mul(x2, xb) ^ ctx.gf.mul(x2, bits(&ctx.kc.curve.a)) ^ 1;
        assert_eq!(lhs, rhs, "planted point {i} is on the curve");
        let want = match frame {
            Frame::X => xb,
            Frame::S => s_of(ctx, xb),
            Frame::U => u_of(ctx, xb),
        };
        assert_eq!(
            want, pl.coord[i],
            "planted coordinate {i} matches its point"
        );
        let z = (0..ell).fold(0u64, |a, t| {
            if (pl.bits[i] >> t) & 1 == 1 {
                a ^ wb[t]
            } else {
                a
            }
        });
        assert_eq!(z, pl.coord[i], "planted coordinate {i} lies in the window");
        acc = ctx.kc.add(&acc, p);
    }
    assert_eq!(acc, pl.r, "planted points sum to R");
    assert!(pl.x_r != 0 && pl.x_r != 1);

    let mut row = json!({
        "study": "E-K1",
        "arm": arm.name(),
        "n": n,
        "m": m,
        "ell": ell,
        "target": k,
        "target_kind": "planted",
        "seed_cell": cell,
        "seed_plant": target_seed(pcell, k),
        "frame": format!("{frame:?}"),
        "V_size": v_size,
        "b": v_size.map(|v| (v as f64).log2()),
        "zeta": ctx.zeta,
        "modulus": ctx.gf.modp,
        "curve": "K_0",
        "planting_redraws": pl.redraws,
        "x_R": pl.x_r,
    });
    let st = &ctx.st;
    let (eqs, n_vars, assign): (Vec<F2BoolPoly>, usize, u64) = match arm {
        Arm::S | Arm::X | Arm::XR => {
            let terms = plain_terms(m).expect("m ∈ {2, 3}");
            let n_vars = m * ell;
            assert!(n_vars <= MAX_VARS);
            let target = match frame {
                Frame::S => s_of(ctx, pl.x_r),
                _ => pl.x_r,
            };
            let mut vals = pl.coord.clone();
            vals.push(target);
            assert_eq!(
                eval_terms(&ctx.gf, &terms, &vals),
                0,
                "Semaev's polynomial vanishes at the planted coordinates (word field)"
            );
            let mut vars: Vec<SymElement> = (0..m)
                .map(|i| SymElement::from_subspace_vars(&basis_el, i * ell, n, n_vars))
                .collect();
            vars.push(SymElement::constant(&el(n, target), n, n_vars));
            (
                sym_polynomial(&vars, &terms, st),
                n_vars,
                pack(&pl.bits, ell),
            )
        }
        Arm::NS => {
            let terms = plain_terms(m).expect("m ∈ {2, 3}");
            let n_vars = m * ell;
            let target = s_of(ctx, pl.x_r);
            let mut vals = pl.coord.clone();
            vals.push(target);
            let mut crng = StdRng::seed_from_u64(target_seed(cell, k));
            let coefs: Vec<u64> = terms
                .iter()
                .map(|t| {
                    if t.iter().all(|&e| e == 0) {
                        0
                    } else {
                        crng.gen_range(1..1u64 << n)
                    }
                })
                .collect();
            // The constant that makes the planted point a zero.
            let mut shift = 0u64;
            for (t, &c) in terms.iter().zip(&coefs) {
                if t.iter().all(|&e| e == 0) {
                    continue;
                }
                shift ^= ctx
                    .gf
                    .mul(c, eval_terms(&ctx.gf, std::slice::from_ref(t), &vals));
            }
            let mut vars: Vec<SymElement> = (0..m)
                .map(|i| SymElement::from_subspace_vars(&basis_el, i * ell, n, n_vars))
                .collect();
            vars.push(SymElement::constant(&el(n, target), n, n_vars));
            let eqs = sym_polynomial_coef(&vars, &terms, &coefs, shift, st);
            row["null_shift"] = json!(shift);
            row["null_terms"] = json!(terms.len());
            (eqs, n_vars, pack(&pl.bits, ell))
        }
        Arm::U => {
            let ell_w = ell - 1;
            let n_vars = m * ell_w + 1;
            assert!(n_vars <= MAX_VARS);
            // Field-level check of the symmetrised polynomial.
            let uu = pl.coord.clone();
            let u_r = u_of(ctx, pl.x_r);
            let mut wvals: Vec<u64> = uu.iter().map(|&u| ctx.gf.mul(u, u) ^ u).collect();
            wvals.push(ctx.gf.mul(u_r, u_r) ^ u_r);
            wvals.push(uu.iter().fold(u_r, |a, &u| a ^ u));
            assert_eq!(
                eval_terms(&ctx.gf, &symmetrised_terms(m).expect("m"), &wvals),
                0,
                "the symmetrised polynomial vanishes at the planted u (word field)"
            );
            let fb = SymmetrisedFactorBase {
                w_basis: basis_el[1..]
                    .iter()
                    .map(|b| artin_schreier(b, &ctx.kc))
                    .collect(),
                v_basis: basis_el.clone(),
                ell,
                points: Vec::new(),
                index_of: HashMap::new(),
                two_torsion: BinaryPoint::Infinity,
                two_torsion_index: 0,
            };
            let sys = build_symmetrised_system(&ctx.kc, &fb, &pl.r, m, st)
                .expect("the symmetrised system builds");
            assert_eq!(sys.n_vars, n_vars);
            // Layout: c_{i,t} (coefficient of v_{t+1}) at i·ℓ_w + t, then ε.
            let mut assign = 0u64;
            let mut eps = 0u64;
            for i in 0..m {
                eps ^= pl.bits[i] & 1;
                assign |= (pl.bits[i] >> 1) << (i * ell_w);
            }
            assign |= eps << (m * ell_w);
            (sys.equations, n_vars, assign)
        }
        Arm::PC => {
            assert_eq!(m, 3, "PC is the m = 3 chain");
            let n_vars = m * ell + (m - 2) * nn;
            assert!(n_vars <= MAX_VARS);
            let e_pt = ctx.kc.add(&pl.points[0], &pl.points[1]);
            let e = x_of(&e_pt);
            // Field-level check of both links.
            let s3 = plain_terms(2).expect("S3");
            assert_eq!(
                eval_terms(&ctx.gf, &s3, &[pl.coord[0], pl.coord[1], e]),
                0,
                "first link vanishes (word field)"
            );
            assert_eq!(
                eval_terms(&ctx.gf, &s3, &[e, pl.coord[2], pl.x_r]),
                0,
                "last link vanishes (word field)"
            );
            let sys = build_decomposition_system(&basis_el, &el(n, pl.x_r), &ctx.kc.curve.b, m, st)
                .expect("the chained system builds");
            assert_eq!(sys.n_vars, n_vars);
            let assign = pack(&pl.bits, ell) | (e << (m * ell));
            (sys.equations, n_vars, assign)
        }
    };
    for (j, e) in eqs.iter().enumerate() {
        assert_eq!(
            e.eval(assign),
            0,
            "equation {j} of {} does not vanish at the planted solution",
            arm.name()
        );
    }
    let excess = match arm {
        Arm::U => (m * (ell - 1) + 1) as i64 - nn as i64,
        Arm::PC => (m * ell) as i64 - nn as i64,
        _ => (m * ell) as i64 - nn as i64,
    };
    let nterms: usize = eqs.iter().map(|p| p.terms.len()).sum();
    merge(
        &mut row,
        &json!({
            "n_vars": n_vars,
            "n_eqs": eqs.len(),
            "n_terms": nterms,
            "input_degree": max_degree(&eqs),
            "excess_bits": excess,
        }),
    );
    Ok(Built {
        eqs,
        n_vars,
        assign,
        row,
    })
}

fn merge(into: &mut Value, from: &Value) {
    if let (Some(a), Some(b)) = (into.as_object_mut(), from.as_object()) {
        for (k, v) in b {
            a.insert(k.clone(), v.clone());
        }
    }
}

// ── The instrument ─────────────────────────────────────────────────

fn measure(eqs: &[F2BoolPoly], n_vars: usize, assign: u64, budget: Duration) -> Value {
    for var in ["F4_F2_MAX_ROWS", "F4_F2_MAX_COLS", "F4_F2_BATCH_INSERTS"] {
        assert!(
            std::env::var(var).is_err(),
            "the registered run needs {var} unset"
        );
    }
    let started = Instant::now();
    let (basis, st) = groebner_basis_f4(eqs.to_vec(), n_vars, Some(budget));
    let wall = started.elapsed().as_secs_f64();
    let has_one = basis
        .iter()
        .any(|p| p.terms.len() == 1 && p.terms[0].mask == 0);
    let planted_ok = !has_one && basis.iter().all(|p| p.eval(assign) == 0);
    let (status, reason) = if st.timed_out {
        (
            "timeout",
            Value::from(format!("F4 budget of {} s", budget.as_secs())),
        )
    } else if st.oversize {
        (
            "oversize",
            Value::from("a Macaulay matrix would exceed MAX_MATRIX_WORDS (1 GiB)"),
        )
    } else if !planted_ok {
        (
            "planted_violation",
            Value::from("the planted solution fails F4's basis"),
        )
    } else {
        ("ok", Value::Null)
    };
    json!({
        "D": st.solving_degree,
        "steps": st.steps,
        "max_rows": st.matrix_rows_max,
        "max_cols": st.matrix_cols_max,
        "field_ops": st.word_xors,
        "field_ops_unit": "64-bit XORs (F4Stats::word_xors)",
        "word_xors_performed": st.word_xors_performed,
        "degree_reached": st.degree_reached,
        "max_poly_degree": st.max_poly_degree,
        "new_elements": st.new_elements,
        "basis_len": basis.len(),
        "pairs_reduced": st.pairs_reduced,
        "field_pairs_reduced": st.field_pairs_reduced,
        "matrix_rows_sum": st.matrix_rows_sum,
        "peak_matrix_bytes": st.peak_matrix_bytes,
        "f4_wall_ns": st.wall_ns,
        "time": wall,
        "timed_out": st.timed_out,
        "oversize": st.oversize,
        "pairs_left": st.pairs_left,
        "planted_ok": planted_ok,
        "status": status,
        "failure_reason": reason,
        "engine": "pq_f4_f2::groebner_basis_f4",
    })
}

fn one(arm: Arm, n: u32, m: usize, ell: usize, k: usize, budget: Duration) -> Value {
    if let Some(nv) = layout_limit(arm, n, m, ell) {
        return json!({
            "study": "E-K1", "arm": arm.name(), "n": n, "m": m, "ell": ell, "target": k,
            "target_kind": "planted", "status": "layout_limit",
            "failure_reason": format!("{nv} Boolean unknowns exceed the repository's {MAX_VARS}"),
            "planted_ok": null, "D": null,
        });
    }
    let ctx = Ctx::new(n);
    let logs = Logs::new(&ctx.gf);
    let b = match build_system(&ctx, Some(&logs), arm, m, ell, k) {
        Ok(b) => b,
        Err(why) => {
            return json!({
                "study": "E-K1", "arm": arm.name(), "n": n, "m": m, "ell": ell, "target": k,
                "target_kind": "planted", "status": "no_plant", "failure_reason": why,
                "planted_ok": null, "D": null,
            });
        }
    };
    let mut built = b.row.clone();
    merge(&mut built, &json!({"phase": "built"}));
    println!("{built}");
    std::io::stdout().flush().ok();
    let mut row = b.row;
    let r = measure(&b.eqs, b.n_vars, b.assign, budget);
    merge(&mut row, &r);
    row
}

// ── The checks ─────────────────────────────────────────────────────

/// The `s`-line polynomial equals Semaev's `S_{m+1}` in `x`, on a grid.
fn check_identity(ctx: &Ctx) {
    let gf = &ctx.gf;
    for m in [2usize, 3] {
        let terms = symmetrised_terms(m).expect("m");
        let plain = plain_terms(m).expect("m");
        let nv = m + 1; // s_1..s_m, s_R
        let k = if m == 2 { 2u64 } else { 4 };
        // 6 distinct non-zero grid values per variable: degree ≤ 4 in each.
        let grid: Vec<u64> = (1..=6u64).map(|i| ctx.gauss[(i - 1) as usize]).collect();
        let total = grid.len().pow(nv as u32);
        let mut checked = 0usize;
        for idx in 0..total {
            let mut v = idx;
            let s: Vec<u64> = (0..nv)
                .map(|_| {
                    let g = grid[v % grid.len()];
                    v /= grid.len();
                    g
                })
                .collect();
            let w: Vec<u64> = s.iter().map(|&x| gf.inv(x)).collect();
            // F = P + σ Q, σ eliminated by σ² + σ = Σ w.
            let (mut p, mut q) = (0u64, 0u64);
            for t in &terms {
                let mut prod = 1u64;
                for (i, &e) in t.iter().take(nv).enumerate() {
                    if e > 0 {
                        prod = gf.mul(prod, gf.pow(w[i], e as u64));
                    }
                }
                if t[nv] == 0 {
                    p ^= prod;
                } else {
                    assert_eq!(t[nv], 1);
                    q ^= prod;
                }
            }
            let omega = w.iter().fold(0u64, |a, &b| a ^ b);
            let g = gf.mul(p, p) ^ gf.mul(p, q) ^ gf.mul(omega, gf.mul(q, q));
            let prod_s = s.iter().fold(1u64, |a, &b| gf.mul(a, b));
            let lhs = gf.mul(g, gf.pow(prod_s, k));
            let rhs = eval_terms(gf, &plain, &s);
            assert_eq!(
                lhs,
                rhs,
                "m = {m}: the s-line polynomial differs from S_{} in x at grid point {idx}",
                m + 1
            );
            checked += 1;
        }
        println!(
            "  identity n={} m={m}: G(1/s)·(Πs)^{k} = S_{} on {checked} grid points (degree ≤ 4 per variable, 6 values per variable: exact)",
            ctx.n,
            m + 1
        );
    }
}

fn check(ns: &[u32], ell_max: usize) {
    for &n in ns {
        let ctx = Ctx::new(n);
        let logs = Logs::new(&ctx.gf);
        println!(
            "n = {n}: ζ = {:#x} (order {}), modulus {:#x}, c_1..c_{n} independent, Frobenius-closed, Dickson rule, Σc_i = 1",
            ctx.zeta,
            2 * n + 1,
            ctx.gf.modp
        );
        // The curve model: the abscissa test counts exactly #E − 1 affine points.
        let full: Vec<u64> = (0..n).map(|k| 1u64 << k).collect();
        assert_eq!(
            BigUint::from(count_points(&ctx, &logs, Frame::X, &full) + 1),
            ctx.kc.group_order,
            "K_0 over F_2^{n}: affine points + O = #E from the Koblitz recurrence"
        );
        println!(
            "  #E(K_0/F_2^{n}) = {} confirmed by counting every abscissa",
            ctx.kc.group_order
        );
        check_identity(&ctx);
        for m in [2usize, 3] {
            for ell in 3..=ell_max.min(n as usize) {
                for arm in Arm::ALL {
                    if arm == Arm::PC && m != 3 {
                        continue;
                    }
                    if layout_limit(arm, n, m, ell).is_some() {
                        continue;
                    }
                    let wb = window_basis(&ctx, arm, m, ell);
                    // The set: exact count against the repository's, where cheap.
                    let ours = count_points(&ctx, &logs, arm.frame(), &wb);
                    if ell <= 11 && (n <= 12 || ell <= 10) {
                        let theirs = repo_count(&ctx, arm.frame(), &wb);
                        assert_eq!(ours, theirs, "{} n={n} ell={ell}: |F| differs", arm.name());
                    }
                    for k in 0..2 {
                        if let Err(why) = build_system(&ctx, Some(&logs), arm, m, ell, k) {
                            println!("  {} n={n} m={m} ℓ={ell}: no cell ({why})", arm.name());
                        }
                    }
                }
            }
            println!(
                "  n={n} m={m}: sets, presentations and planted solutions pass for ℓ ≤ {}",
                ell_max.min(n as usize)
            );
        }
    }
    println!("all checks pass");
}

// ── Running ────────────────────────────────────────────────────────

fn git_commit() -> String {
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();
    let dirty = Command::new("git")
        .args([
            "diff",
            "--quiet",
            "HEAD",
            "--",
            "examples",
            "src",
            "Cargo.toml",
            "Cargo.lock",
        ])
        .status()
        .map(|s| !s.success())
        .unwrap_or(true);
    if dirty {
        format!("{head}-dirty")
    } else {
        head
    }
}

fn host() -> Value {
    let name = std::fs::read_to_string("/proc/sys/kernel/hostname")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    let cpu = std::fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|x| x.trim().to_string())
        })
        .unwrap_or_default();
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(0);
    json!({"hostname": name, "cpu": cpu, "cores": cores})
}

fn utc_now() -> String {
    Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

struct RunArgs {
    arms: Vec<Arm>,
    ns: Vec<u32>,
    ms: Vec<usize>,
    ell_min: usize,
    ell_max: Option<usize>,
    targets: usize,
    budget: u64,
    vmem_kb: u64,
    grace: u64,
    out_dir: String,
    file: String,
}

/// Run one system in a child under `ulimit -v` and a hard wall limit of
/// budget + grace; the row, or a failure row with its reason.
fn run_child(
    budget: u64,
    grace: u64,
    vmem_kb: u64,
    arm: Arm,
    n: u32,
    m: usize,
    ell: usize,
    k: usize,
    log: &mut File,
) -> Value {
    let exe = std::env::current_exe().expect("current exe");
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("ulimit -v {vmem_kb} && exec \"$0\" \"$@\""))
        .arg(&exe)
        .args([
            "one",
            "--arm",
            arm.name(),
            "--n",
            &n.to_string(),
            "--m",
            &m.to_string(),
            "--ell",
            &ell.to_string(),
            "--target",
            &k.to_string(),
            "--budget",
            &budget.to_string(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn a child");
    let mut out = child.stdout.take().expect("stdout");
    let mut err = child.stderr.take().expect("stderr");
    let out_t = std::thread::spawn(move || {
        let mut s = String::new();
        out.read_to_string(&mut s).ok();
        s
    });
    let err_t = std::thread::spawn(move || {
        let mut s = String::new();
        err.read_to_string(&mut s).ok();
        s
    });
    let started = Instant::now();
    let limit = Duration::from_secs(budget + grace);
    let mut killed = false;
    let status = loop {
        if let Some(st) = child.try_wait().expect("wait") {
            break st;
        }
        if started.elapsed() > limit {
            child.kill().ok();
            killed = true;
            break child.wait().expect("wait");
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    let wall = started.elapsed().as_secs_f64();
    let stdout = out_t.join().unwrap_or_default();
    let stderr = err_t.join().unwrap_or_default();
    if !stderr.is_empty() {
        write!(log, "{stderr}").ok();
    }
    let rows: Vec<Value> = stdout
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let last = rows.last().cloned();
    let mut row = match last {
        Some(r) if r.get("phase").is_none() && status.success() => r,
        other => {
            let mut r = other.unwrap_or_else(|| {
                json!({"study": "E-K1", "arm": arm.name(), "n": n, "m": m, "ell": ell,
                       "target": k, "target_kind": "planted"})
            });
            if let Some(o) = r.as_object_mut() {
                o.remove("phase");
            }
            let tail: Vec<&str> = stderr.lines().rev().take(3).collect();
            let (st, why) = if killed {
                (
                    "timeout",
                    format!(
                        "killed at the hard wall limit of {} s (budget {budget} s + grace {grace} s)",
                        budget + grace
                    ),
                )
            } else if stderr.contains("memory allocation") {
                (
                    "aborted",
                    format!("memory allocation failed under ulimit -v {vmem_kb} ({status})"),
                )
            } else {
                (
                    "aborted",
                    format!("child exited: {status}; stderr: {}", tail.join(" | ")),
                )
            };
            merge(
                &mut r,
                &json!({"status": st, "failure_reason": why, "planted_ok": null, "D": null,
                        "time": wall}),
            );
            r
        }
    };
    merge(
        &mut row,
        &json!({"process_wall_s": wall, "budget_s": budget, "ulimit_v_kb": vmem_kb}),
    );
    row
}

fn run(a: &RunArgs) {
    std::fs::create_dir_all(&a.out_dir).expect("create --out-dir");
    let commit = git_commit();
    let host = host();
    eprintln!("commit {commit}, host {host}");
    for &n in &a.ns {
        for &m in &a.ms {
            for &arm in &a.arms {
                if arm == Arm::PC && m != 3 {
                    continue;
                }
                let name = a
                    .file
                    .replace("{arm}", arm.name())
                    .replace("{n}", &n.to_string())
                    .replace("{m}", &m.to_string());
                let base = format!("{}/{name}", a.out_dir);
                let mut out = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{base}.jsonl"))
                    .expect("open rows");
                let mut log = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(format!("{base}.log"))
                    .expect("open log");
                writeln!(
                    log,
                    "== {} n={n} m={m}: commit {commit}, budget {} s, grace {} s, ulimit -v {}, started {}",
                    arm.name(),
                    a.budget,
                    a.grace,
                    a.vmem_kb,
                    utc_now()
                )
                .ok();
                let top = a.ell_max.unwrap_or(n as usize).min(n as usize);
                for ell in a.ell_min..=top {
                    let mut all_ok = true;
                    for k in 0..a.targets {
                        let mut row =
                            run_child(a.budget, a.grace, a.vmem_kb, arm, n, m, ell, k, &mut log);
                        merge(&mut row, &json!({"git_commit": commit, "host": host}));
                        let line = format!(
                            "{:2} n={n} m={m} ell={ell:2} target={k} b={:.3} |F|={} n_vars={} D={} steps={} cols={} {:.1} s {}{}",
                            arm.name(),
                            row["b"].as_f64().unwrap_or(f64::NAN),
                            row["V_size"],
                            row["n_vars"],
                            row["D"],
                            row["steps"],
                            row["max_cols"],
                            row["time"].as_f64().unwrap_or(f64::NAN),
                            row["status"].as_str().unwrap_or("?"),
                            row["failure_reason"]
                                .as_str()
                                .map(|s| format!(" ({s})"))
                                .unwrap_or_default(),
                        );
                        eprintln!("{line}");
                        writeln!(log, "{line}").ok();
                        writeln!(out, "{row}").expect("write a row");
                        out.flush().ok();
                        log.flush().ok();
                        if row["status"] != "ok" && row["status"] != "no_plant" {
                            all_ok = false;
                        }
                    }
                    if !all_ok {
                        let msg = format!(
                            "-- {} n={n} m={m}: a system at ell = {ell} hit a limit or failed; the series stops",
                            arm.name()
                        );
                        eprintln!("{msg}");
                        writeln!(log, "{msg}").ok();
                        break;
                    }
                }
                writeln!(log, "== {} n={n} m={m}: ended {}", arm.name(), utc_now()).ok();
            }
        }
    }
}

// ── Reading rows ───────────────────────────────────────────────────

fn read_rows(dir: &str) -> Vec<(String, Value)> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .expect("read the runs directory")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|x| x == "jsonl")
                && p.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with("K1-") && !s.starts_with("K1-replay"))
        })
        .collect();
    files.sort();
    let mut rows = Vec::new();
    for f in files {
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&f).expect("read rows");
        for l in text.lines().filter(|l| !l.trim().is_empty()) {
            rows.push((name.clone(), serde_json::from_str(l).expect("a JSON row")));
        }
    }
    rows
}

// ── The trace-replay check ─────────────────────────────────────────

/// Every completed system is measured once more from its seed. The trace
/// (D, max_cols, max_rows, steps) must equal the committed row's.
fn replay(dir: &str, out: &str, budget: u64, grace: u64, vmem_kb: u64) {
    let rows = read_rows(dir);
    let path = format!("{dir}/{out}");
    let mut outf = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect("open replay rows");
    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path.replace(".jsonl", ".log"))
        .expect("open replay log");
    let commit = git_commit();
    let (mut total, mut matched) = (0usize, 0usize);
    for (_, r) in &rows {
        if r["status"] != "ok" {
            continue;
        }
        let arm = Arm::parse(r["arm"].as_str().expect("arm"));
        let n = r["n"].as_u64().expect("n") as u32;
        let m = r["m"].as_u64().expect("m") as usize;
        let ell = r["ell"].as_u64().expect("ell") as usize;
        let k = r["target"].as_u64().expect("target") as usize;
        let rep = run_child(budget, grace, vmem_kb, arm, n, m, ell, k, &mut log);
        let keys = ["D", "max_cols", "max_rows", "steps"];
        let same = rep["status"] == "ok" && keys.iter().all(|key| rep[*key] == r[*key]);
        let fops_same = rep["field_ops"] == r["field_ops"];
        total += 1;
        if same {
            matched += 1;
        }
        let row = json!({
            "study": "E-K1", "arm": arm.name(), "n": n, "m": m, "ell": ell, "target": k,
            "trace_matches": same,
            "field_ops_match": fops_same,
            "committed": {"D": r["D"], "max_cols": r["max_cols"], "max_rows": r["max_rows"],
                          "steps": r["steps"], "field_ops": r["field_ops"]},
            "replayed": {"D": rep["D"], "max_cols": rep["max_cols"], "max_rows": rep["max_rows"],
                         "steps": rep["steps"], "field_ops": rep["field_ops"],
                         "status": rep["status"], "failure_reason": rep["failure_reason"]},
            "git_commit": commit,
        });
        let line = format!(
            "{:2} n={n} m={m} ell={ell:2} target={k} trace_matches={same} field_ops_match={fops_same}",
            arm.name()
        );
        eprintln!("{line}");
        writeln!(log, "{line}").ok();
        writeln!(outf, "{row}").expect("write replay row");
        outf.flush().ok();
    }
    let msg = format!("replayed {total} completed systems, {matched} reproduced the trace");
    eprintln!("{msg}");
    writeln!(log, "{msg}").ok();
}

// ── Analysis ───────────────────────────────────────────────────────

fn lsq(pts: &[(f64, f64)]) -> Option<(f64, f64)> {
    if pts.len() < 2 {
        return None;
    }
    let n = pts.len() as f64;
    let mb = pts.iter().map(|x| x.0).sum::<f64>() / n;
    let md = pts.iter().map(|x| x.1).sum::<f64>() / n;
    let sxx: f64 = pts.iter().map(|x| (x.0 - mb).powi(2)).sum();
    if sxx == 0.0 {
        return None;
    }
    let sxy: f64 = pts.iter().map(|x| (x.0 - mb) * (x.1 - md)).sum();
    let s = sxy / sxx;
    Some((s, md - s * mb))
}

fn mean(xs: &[u64]) -> f64 {
    xs.iter().sum::<u64>() as f64 / xs.len() as f64
}

fn sigma_verdict(cells: usize, sigma: Option<f64>) -> &'static str {
    match sigma {
        _ if cells < 3 => "inconclusive (fewer than 3 completed cells)",
        Some(s) if s <= 0.25 => "candidate (σ ≤ 0.25 on ≥ 3 cells)",
        Some(s) if s < 0.5 => "weak (0.25 < σ < 0.5)",
        Some(_) => "no middle ground at this arity (σ ≥ 0.5)",
        None => "inconclusive (no slope)",
    }
}

fn kappa_verdict(cells: usize, kappa: Option<f64>, m: usize) -> &'static str {
    match kappa {
        _ if cells < 3 => "inconclusive (fewer than 3 completed cells)",
        Some(k) if k < m as f64 - 1.0 => "cost-favourable (κ < m − 1)",
        Some(_) => "cost-unfavourable (κ ≥ m − 1)",
        None => "inconclusive (no slope)",
    }
}

#[derive(Default, Clone)]
struct Cell {
    ell: usize,
    b: f64,
    excess: i64,
    d: Vec<u64>,
    steps: Vec<u64>,
    cols: Vec<u64>,
    rows: Vec<u64>,
    fops: Vec<u64>,
    time: Vec<f64>,
    status: Vec<String>,
}

impl Cell {
    fn complete(&self, targets: usize) -> bool {
        self.status.len() >= targets && self.status.iter().all(|s| s == "ok")
    }
    fn d(&self) -> f64 {
        mean(&self.d)
    }
    fn lf(&self) -> f64 {
        mean(&self.fops).log2()
    }
}

type Key = (String, u32, usize);

fn series(
    rows: &[(String, Value)],
    replay: &HashMap<(String, u32, usize, usize, usize), bool>,
) -> BTreeMap<Key, Vec<Cell>> {
    let mut by: BTreeMap<Key, BTreeMap<usize, Cell>> = BTreeMap::new();
    for (_, r) in rows {
        let arm = r["arm"].as_str().unwrap_or("?").to_string();
        let n = r["n"].as_u64().unwrap_or(0) as u32;
        let m = r["m"].as_u64().unwrap_or(0) as usize;
        let ell = r["ell"].as_u64().unwrap_or(0) as usize;
        let k = r["target"].as_u64().unwrap_or(0) as usize;
        if r["status"] == "no_plant" {
            continue;
        }
        let c = by
            .entry((arm.clone(), n, m))
            .or_default()
            .entry(ell)
            .or_default();
        c.ell = ell;
        if let Some(b) = r["b"].as_f64() {
            c.b = b;
        }
        if let Some(e) = r["excess_bits"].as_i64() {
            c.excess = e;
        }
        let replayed_ok = replay.get(&(arm, n, m, ell, k)).copied().unwrap_or(true);
        let ok = r["status"] == "ok" && replayed_ok && r["field_ops"].is_u64();
        if ok {
            c.d.push(r["D"].as_u64().unwrap_or(0));
            c.steps.push(r["steps"].as_u64().unwrap_or(0));
            c.cols.push(r["max_cols"].as_u64().unwrap_or(0));
            c.rows.push(r["max_rows"].as_u64().unwrap_or(0));
            c.fops.push(r["field_ops"].as_u64().unwrap_or(0));
            c.time.push(r["time"].as_f64().unwrap_or(f64::NAN));
        }
        c.status.push(if ok {
            "ok".into()
        } else if r["status"] == "ok" {
            "replay_mismatch".into()
        } else {
            r["status"].as_str().unwrap_or("?").to_string()
        });
    }
    by.into_iter()
        .map(|(k, v)| (k, v.into_values().collect()))
        .collect()
}

struct Fit {
    cells: usize,
    kappa: Option<f64>,
    sigma: Option<f64>,
    kappa_regime: Option<f64>,
    cells_regime: usize,
}

fn fit(cells: &[Cell], targets: usize) -> Fit {
    let done: Vec<&Cell> = cells.iter().filter(|c| c.complete(targets)).collect();
    let k = lsq(&done.iter().map(|c| (c.b, c.lf())).collect::<Vec<_>>()).map(|x| x.0);
    let s = lsq(&done.iter().map(|c| (c.b, c.d())).collect::<Vec<_>>()).map(|x| x.0);
    let reg: Vec<&&Cell> = done.iter().filter(|c| c.excess >= 0).collect();
    let kr = lsq(&reg.iter().map(|c| (c.b, c.lf())).collect::<Vec<_>>()).map(|x| x.0);
    Fit {
        cells: done.len(),
        kappa: k,
        sigma: s,
        kappa_regime: kr,
        cells_regime: reg.len(),
    }
}

fn f3(x: Option<f64>) -> String {
    x.map_or("-".into(), |v| format!("{v:.3}"))
}

fn cell_of(cells: &[Cell], ell: usize, targets: usize) -> Option<&Cell> {
    cells.iter().find(|c| c.ell == ell && c.complete(targets))
}

fn analyze(dir: &str, replay_file: Option<&str>) {
    let targets = 2usize;
    let rows = read_rows(dir);
    let mut replay_map: HashMap<(String, u32, usize, usize, usize), bool> = HashMap::new();
    let (mut rep_total, mut rep_ok, mut rep_fops) = (0usize, 0usize, 0usize);
    if let Some(f) = replay_file {
        let text = std::fs::read_to_string(format!("{dir}/{f}")).unwrap_or_default();
        for l in text.lines().filter(|l| !l.trim().is_empty()) {
            let r: Value = serde_json::from_str(l).expect("a replay row");
            rep_total += 1;
            let ok = r["trace_matches"] == true;
            if ok {
                rep_ok += 1;
            }
            if r["field_ops_match"] == true {
                rep_fops += 1;
            }
            replay_map.insert(
                (
                    r["arm"].as_str().unwrap_or("?").to_string(),
                    r["n"].as_u64().unwrap_or(0) as u32,
                    r["m"].as_u64().unwrap_or(0) as usize,
                    r["ell"].as_u64().unwrap_or(0) as usize,
                    r["target"].as_u64().unwrap_or(0) as usize,
                ),
                ok,
            );
        }
    }
    let all = series(&rows, &replay_map);
    println!("E-K1, Amendment 6: rows from {dir} ({} rows)", rows.len());
    println!("Instrument: pq_f4_f2::groebner_basis_f4; field_ops = F4Stats::word_xors (64-bit XORs); D = F4Stats::solving_degree.");
    println!("b = log₂ of the exact number of factor-base points; κ = Δlog₂ field_ops/Δb; σ = ΔD/Δb; cells need both targets ok.");
    println!();
    println!("── Trace-replay check");
    if replay_file.is_some() {
        println!(
            "  {rep_ok} of {rep_total} completed systems reproduced D, max_cols, max_rows and steps from their seed; field_ops equal in {rep_fops}"
        );
        println!(
            "  → {}",
            if rep_total == 0 {
                "not run".to_string()
            } else if rep_ok == rep_total {
                "PASS (every replayed system reproduced its trace)".to_string()
            } else {
                "FAIL (mismatched systems are excluded from every score below)".to_string()
            }
        );
    } else {
        println!("  no replay file given");
    }
    let ns: Vec<u32> = {
        let mut v: Vec<u32> = all.keys().map(|k| k.1).collect();
        v.sort();
        v.dedup();
        v
    };
    let arms = ["S", "X", "XR", "U", "NS", "PC"];
    let mut fits: BTreeMap<Key, Fit> = BTreeMap::new();
    for &n in &ns {
        for m in [2usize, 3] {
            for arm in arms {
                let key = (arm.to_string(), n, m);
                let Some(cells) = all.get(&key) else { continue };
                println!();
                println!("── {arm}, n = {n}, m = {m}");
                println!("  ell b       excess D per system  steps        max_cols        max_rows        field_ops per system          time s        status");
                for c in cells {
                    println!(
                        "  {:<3} {:<7.3} {:<6} {:<15} {:<12} {:<15} {:<15} {:<29} {:<13} {}",
                        c.ell,
                        c.b,
                        c.excess,
                        format!("{:?}", c.d),
                        format!("{:?}", c.steps),
                        format!("{:?}", c.cols),
                        format!("{:?}", c.rows),
                        format!("{:?}", c.fops),
                        format!("{:.1?}", c.time),
                        c.status.join(",")
                    );
                }
                let f = fit(cells, targets);
                println!(
                    "  completed cells {}: κ = {} against m − 1 = {} → {};  σ = {} → {}",
                    f.cells,
                    f3(f.kappa),
                    m - 1,
                    kappa_verdict(f.cells, f.kappa, m),
                    f3(f.sigma),
                    sigma_verdict(f.cells, f.sigma)
                );
                println!(
                    "  secondary, cells with excess ≥ 0 ({}): κ = {}",
                    f.cells_regime,
                    f3(f.kappa_regime)
                );
                fits.insert(key, f);
            }
        }
    }

    println!();
    println!("══ Decision rules (Amendment 6)");
    let get = |arm: &str, n: u32, m: usize| all.get(&(arm.to_string(), n, m));
    let fit_of = |arm: &str, n: u32, m: usize| fits.get(&(arm.to_string(), n, m));
    let mut p2_all = true;
    let mut p2_any = false;
    let mut p3_viol = false;
    let mut p5_viol = false;
    let mut p5_any = false;
    let mut p6: Vec<String> = Vec::new();
    let mut p7: Vec<String> = Vec::new();
    for &n in &ns {
        for m in [2usize, 3] {
            println!();
            println!("── n = {n}, m = {m}");
            let (Some(sc), Some(sf)) = (get("S", n, m), fit_of("S", n, m)) else {
                println!("  no S series");
                continue;
            };
            // R1: κ verdict for every arm.
            for arm in arms {
                if let Some(f) = fit_of(arm, n, m) {
                    println!(
                        "  R1 {arm:<2}: κ = {} on {} cells → {}",
                        f3(f.kappa),
                        f.cells,
                        kappa_verdict(f.cells, f.kappa, m)
                    );
                    if f.cells >= 3 && f.kappa.is_some_and(|k| k < m as f64 - 1.0) {
                        p3_viol = true;
                    }
                }
            }
            // R2: S against X.
            if let (Some(xc), Some(xf)) = (get("X", n, m), fit_of("X", n, m)) {
                let shared: Vec<(usize, f64, f64)> = sc
                    .iter()
                    .filter(|c| c.complete(targets))
                    .filter_map(|c| cell_of(xc, c.ell, targets).map(|x| (c.ell, c.d(), x.d())))
                    .collect();
                let verdict = if shared.len() < 3 {
                    "inconclusive (fewer than 3 shared completed cells)".to_string()
                } else {
                    let dok = shared.iter().all(|s| (s.1 - s.2).abs() <= 1.0);
                    let kok = match (sf.kappa, xf.kappa) {
                        (Some(a), Some(b)) => (a - b).abs() < 0.25,
                        _ => false,
                    };
                    if dok && kok {
                        "tied (|ΔD| ≤ 1 at every shared cell and |Δκ| < 0.25)".to_string()
                    } else {
                        format!(
                            "differs (|ΔD| ≤ 1 at every shared cell: {dok}; |Δκ| < 0.25: {kok})"
                        )
                    }
                };
                println!(
                    "  R2 S vs X: {} shared cells, κ_S = {}, κ_X = {} → {verdict}",
                    shared.len(),
                    f3(sf.kappa),
                    f3(xf.kappa)
                );
                p2_any = true;
                if !verdict.starts_with("tied") {
                    p2_all = false;
                }
            }
            // R3: S against XR and U.
            let mut lower_than_both = true;
            let mut any_unknown = false;
            for other in ["XR", "U"] {
                let (Some(oc), Some(of)) = (get(other, n, m), fit_of(other, n, m)) else {
                    any_unknown = true;
                    continue;
                };
                let verdict = match (sf.kappa, of.kappa) {
                    _ if sf.cells < 3 || of.cells < 3 => {
                        any_unknown = true;
                        lower_than_both = false;
                        "inconclusive (fewer than 3 completed cells)"
                    }
                    (Some(a), Some(b)) if a <= b - 0.25 => "S lower (κ_S ≤ κ − 0.25)",
                    (Some(a), Some(b)) if a >= b + 0.25 => {
                        lower_than_both = false;
                        "S higher (κ_S ≥ κ + 0.25)"
                    }
                    (Some(_), Some(_)) => {
                        lower_than_both = false;
                        "tied (|Δκ| < 0.25)"
                    }
                    _ => {
                        any_unknown = true;
                        lower_than_both = false;
                        "inconclusive (no slope)"
                    }
                };
                let mut cheaper = 0;
                let mut dearer = 0;
                let mut shared = 0;
                for c in sc.iter().filter(|c| c.complete(targets)) {
                    if let Some(o) = cell_of(oc, c.ell, targets) {
                        shared += 1;
                        if o.lf() - c.lf() >= 1.0 {
                            cheaper += 1;
                        }
                        if o.lf() - c.lf() <= -1.0 {
                            dearer += 1;
                        }
                    }
                }
                println!(
                    "  R3 S vs {other}: κ_S = {}, κ_{other} = {} → {verdict}; level at equal ℓ over {shared} shared cells: S cheaper by ≥ 2× in {cheaper}, dearer by ≥ 2× in {dearer}",
                    f3(sf.kappa),
                    f3(of.kappa)
                );
            }
            // R4: the outcome.
            let outcome = if sf.cells >= 3 && sf.kappa.is_some_and(|k| k >= m as f64 - 1.0) {
                "no advance (S is cost-unfavourable)"
            } else if any_unknown || sf.cells < 3 {
                "inconclusive (a needed series has fewer than 3 completed cells)"
            } else if sf.kappa.is_some_and(|k| k < m as f64 - 1.0) && lower_than_both {
                "advances (S cost-favourable and slope lower than XR and U)"
            } else {
                "no advance (S is not slope-lower than both XR and U)"
            };
            println!("  R4 outcome: {outcome}");
            if outcome.starts_with("no advance") || outcome.starts_with("advances") {
                p5_any = true;
            }
            if outcome.starts_with("advances") {
                p5_viol = true;
            }
            // R5: the null.
            if let Some(nc) = get("NS", n, m) {
                let shared: Vec<(usize, f64, f64, f64, f64)> = sc
                    .iter()
                    .filter(|c| c.complete(targets))
                    .filter_map(|c| {
                        cell_of(nc, c.ell, targets).map(|x| (c.ell, c.d(), x.d(), c.lf(), x.lf()))
                    })
                    .collect();
                let verdict = if shared.len() < 3 {
                    "inconclusive (fewer than 3 shared completed cells)"
                } else if shared.iter().all(|s| s.1 == s.2) {
                    "shape, not structure (N's D equals S's in every shared cell)"
                } else if shared.iter().filter(|s| s.2 >= s.1 + 1.0).count() >= 2 {
                    "structure beyond shape (N's D exceeds S's by ≥ 1 in ≥ 2 shared cells)"
                } else {
                    "mixed"
                };
                let dearer = shared.iter().filter(|s| s.4 - s.3 >= 1.0).count();
                let cheaper = shared.iter().filter(|s| s.4 - s.3 <= -1.0).count();
                println!(
                    "  R5 null NS vs S over {} shared cells: {verdict}; descriptive on log₂ field_ops: NS dearer by ≥ 2× in {dearer}, cheaper by ≥ 2× in {cheaper}; D(S) {:?} D(NS) {:?}",
                    shared.len(),
                    shared.iter().map(|s| s.1).collect::<Vec<_>>(),
                    shared.iter().map(|s| s.2).collect::<Vec<_>>()
                );
                p6.push(format!("n={n} m={m}: {verdict}"));
            }
            // R7: Gauss window against random window.
            if let (Some(xc), Some(rc)) = (get("X", n, m), get("XR", n, m)) {
                let mut shared = 0;
                let mut random_dearer = 0;
                let mut random_cheaper = 0;
                for c in xc.iter().filter(|c| c.complete(targets)) {
                    if let Some(r) = cell_of(rc, c.ell, targets) {
                        shared += 1;
                        if r.lf() - c.lf() >= 1.0 {
                            random_dearer += 1;
                        }
                        if r.lf() - c.lf() <= -1.0 {
                            random_cheaper += 1;
                        }
                    }
                }
                println!(
                    "  R7 Gauss x-window vs random x-window over {shared} shared cells: random dearer by ≥ 2× in {random_dearer}, cheaper by ≥ 2× in {random_cheaper} (descriptive)"
                );
                p7.push(format!(
                    "n={n} m={m}: {random_dearer} of {shared} shared cells with the random window ≥ 2× dearer"
                ));
            }
        }
    }

    // R6: positive control.
    println!();
    println!("── R6 positive control (m = 3)");
    let mut shared_pc1 = 0usize;
    let mut detected_pc1 = 0usize;
    let mut xr_dearer_dir = 0usize;
    let mut shared_pc2 = 0usize;
    let mut detected_pc2 = 0usize;
    for &n in &ns {
        if let (Some(rc), Some(uc)) = (get("XR", n, 3), get("U", n, 3)) {
            for c in rc.iter().filter(|c| c.complete(targets)) {
                if let Some(u) = cell_of(uc, c.ell, targets) {
                    shared_pc1 += 1;
                    let d = c.lf() - u.lf();
                    if d >= 1.0 {
                        detected_pc1 += 1;
                    }
                    if d > 0.0 {
                        xr_dearer_dir += 1;
                    }
                    println!(
                        "  PC1 n={n} ℓ={}: log₂ field_ops XR {:.2} vs U {:.2} (ratio {:.2}×); D {:.1} vs {:.1}",
                        c.ell, c.lf(), u.lf(), 2f64.powf(d), c.d(), u.d()
                    );
                }
            }
        }
        if let (Some(pc), Some(xc)) = (get("PC", n, 3), get("X", n, 3)) {
            for c in pc.iter().filter(|c| c.complete(targets)) {
                if let Some(x) = cell_of(xc, c.ell, targets) {
                    shared_pc2 += 1;
                    let d = c.lf() - x.lf();
                    if d.abs() >= 1.0 || (c.d() - x.d()).abs() >= 1.0 {
                        detected_pc2 += 1;
                    }
                    println!(
                        "  PC2 n={n} ℓ={}: log₂ field_ops chained {:.2} vs direct {:.2} (ratio {:.2}×); D {:.1} vs {:.1}",
                        c.ell, c.lf(), x.lf(), 2f64.powf(d), c.d(), x.d()
                    );
                }
            }
        }
    }
    let pc1 = if shared_pc1 < 2 {
        "not evaluable (fewer than 2 shared completed cells): every verdict stands as a reading only"
    } else if detected_pc1 >= 2 {
        "DETECTED (XR ≥ 2× U's field_ops in ≥ 2 shared cells)"
    } else {
        "NOT DETECTED: the instrument is blind to this difference, every S verdict is invalid"
    };
    println!(
        "  PC1 (random x-window direct S₄, degree 6, vs u-frame, degree 4): {detected_pc1} of {shared_pc1} shared cells ≥ 2× → {pc1}"
    );
    println!(
        "  PC2 (X chained vs direct, same window and planted points): {detected_pc2} of {shared_pc2} shared cells differ (|Δlog₂ field_ops| ≥ 1 or |ΔD| ≥ 1) (descriptive)"
    );

    // Predictions.
    println!();
    println!("══ Predictions scorecard");
    let mark = |ok: Option<bool>| match ok {
        Some(true) => "held",
        Some(false) => "FAILED",
        None => "not evaluable",
    };
    println!(
        "  P1 the s-line polynomial equals Semaev's S_(m+1) in x: exact identity, checked by `check` on a degree-exact grid at n = 11 and 23 (runs/K1-check.txt)"
    );
    println!(
        "  P2 S tied with X in every (n, m): {}",
        mark(p2_any.then_some(p2_all))
    );
    println!(
        "  P3 no arm cost-favourable on ≥ 3 completed cells: {}",
        mark(Some(!p3_viol))
    );
    println!(
        "  P4 positive control detected with XR dearer than U: {}",
        mark(if shared_pc1 < 2 {
            None
        } else {
            Some(detected_pc1 >= 2 && xr_dearer_dir == shared_pc1)
        })
    );
    println!(
        "  P5 S does not advance in any (n, m): {}",
        mark(p5_any.then_some(!p5_viol))
    );
    println!("  P6 null reads \"structure beyond shape\" in every (n, m):");
    for l in &p6 {
        println!("       {l}");
    }
    println!("  P7 the Gauss x-window is cheaper than the random x-window at equal ℓ (random ≥ 2× dearer in at least half of the shared cells):");
    for l in &p7 {
        println!("       {l}");
    }
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .map(|i| args.get(i + 1).cloned().expect("a flag value"))
}

fn list<T: std::str::FromStr>(s: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    s.split(',').map(|x| x.parse().expect("a number")).collect()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str).unwrap_or("") {
        "check" => {
            let ns = flag(&args, "--n").map_or(vec![11, 23], |s| list(&s));
            let ell_max = flag(&args, "--ell-max").map_or(9, |s| s.parse().unwrap());
            check(&ns, ell_max);
        }
        "one" => {
            let arm = Arm::parse(&flag(&args, "--arm").expect("--arm"));
            let n: u32 = flag(&args, "--n").expect("--n").parse().unwrap();
            let m: usize = flag(&args, "--m").expect("--m").parse().unwrap();
            let ell: usize = flag(&args, "--ell").expect("--ell").parse().unwrap();
            let k: usize = flag(&args, "--target").map_or(0, |s| s.parse().unwrap());
            let budget = flag(&args, "--budget").map_or(300, |s| s.parse().unwrap());
            let row = one(arm, n, m, ell, k, Duration::from_secs(budget));
            println!("{row}");
        }
        "run" => {
            let a = RunArgs {
                arms: flag(&args, "--arms").map_or(Arm::ALL.to_vec(), |s| {
                    s.split(',').map(Arm::parse).collect()
                }),
                ns: flag(&args, "--n").map_or(vec![11, 23], |s| list(&s)),
                ms: flag(&args, "--m").map_or(vec![2, 3], |s| list(&s)),
                ell_min: flag(&args, "--ell-min").map_or(3, |s| s.parse().unwrap()),
                ell_max: flag(&args, "--ell-max").map(|s| s.parse().unwrap()),
                targets: flag(&args, "--targets").map_or(2, |s| s.parse().unwrap()),
                budget: flag(&args, "--budget").map_or(300, |s| s.parse().unwrap()),
                vmem_kb: flag(&args, "--vmem-kb").map_or(13_000_000, |s| s.parse().unwrap()),
                grace: flag(&args, "--grace").map_or(120, |s| s.parse().unwrap()),
                out_dir: flag(&args, "--out-dir").expect("--out-dir"),
                file: flag(&args, "--file").unwrap_or_else(|| "K1-{arm}-n{n}-m{m}".into()),
            };
            assert!(a.ell_min >= 3, "ell starts at 3");
            run(&a);
        }
        "replay" => {
            let dir = args.get(1).expect("a runs directory");
            let out = flag(&args, "--out").expect("--out");
            replay(
                dir,
                &out,
                flag(&args, "--budget").map_or(300, |s| s.parse().unwrap()),
                flag(&args, "--grace").map_or(120, |s| s.parse().unwrap()),
                flag(&args, "--vmem-kb").map_or(13_000_000, |s| s.parse().unwrap()),
            );
        }
        "analyze" => {
            let dir = args.get(1).expect("a runs directory");
            analyze(dir, flag(&args, "--replay").as_deref());
        }
        _ => {
            eprintln!(
                "usage: fb_koblitz_ek1 check|one|run|replay|analyze …  (see the module docs)"
            );
            std::process::exit(2);
        }
    }
}
