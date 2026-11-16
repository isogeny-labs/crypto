//! binwcal: a w_cal instrument for GF(2^n) / binary Koblitz curves.
//!
//! The binary analogue of `ecbench wcal` (`src/cryptanalysis/ecbench/wcal.rs`):
//! the time of one native operation divided by the time of one affine group
//! addition (1 GAE) on the same curve, host and process, measured as paired,
//! interleaved rounds.  The group addition is the one ecbench's binary rows
//! count: `FastCurve::add` (ecbench `Instance::Binary`, n <= 62),
//! `WideStrongRho::add` (ecbench `Instance::Wide`, 64 <= n <= 127) and, past
//! ecbench's reach, `certicom131_symlp::BinaryW::add` on `GF(2^n)` as three
//! words (n = 131, and n = 83 as a cross-implementation check).
//!
//! Subcommands (all output JSON on stdout):
//!   measure --n N --a A --impl small|wide|w3 --rounds R --warmup W --seed S --rep K
//!   linz --n N --m M --d D --targets T --reps K
//!   analyze --raw DIR --out DIR      (prices, controls, L2/L4 re-reads, SVG)
//!
//! Every claim is [pcc:unverified].

mod analyze;
mod l2v;

use std::hint::black_box;
use std::time::Instant;

use crypto_lib::binary_ecc::IrreduciblePoly;
use crypto_lib::cryptanalysis::certicom131_symlp::curves::{BinaryW, Pt, SymCurve};
use crypto_lib::cryptanalysis::certicom131_symlp::gf2w::{self, Gf2W, W3};
use crypto_lib::cryptanalysis::ecbench::workload::{self, CurveSpec, Instance};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;
use crypto_lib::cryptanalysis::koblitz_strong_rho::RawPointG;
use serde_json::{json, Value};

// ---------------------------------------------------------------- rng
pub struct Sm(pub u64);
impl Sm {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    pub fn below(&mut self, k: usize) -> usize {
        (self.next() % k as u64) as usize
    }
}

// ---------------------------------------------------------------- backends
/// One curve and its field, behind the ops the instrument times.  Every op
/// runs `iters` units and returns nothing; the caller times it.
trait Backend {
    fn label(&self) -> String;
    fn ga(&mut self, iters: u64);
    fn ga_aa(&mut self, iters: u64);
    fn ga4(&mut self, iters: u64);
    fn dbl(&mut self, iters: u64);
    fn mul_lat(&mut self, iters: u64);
    fn mul8_lat(&mut self, iters: u64);
    fn mul_tp(&mut self, iters: u64);
    fn sqr_lat(&mut self, iters: u64);
    fn sqr_tp(&mut self, iters: u64);
    fn inv_lat(&mut self, iters: u64);
    fn fadd(&mut self, iters: u64);
    /// A check that the ops really compute what they claim (run once).
    fn selfcheck(&mut self) -> Result<(), String>;
}

macro_rules! ga_loop {
    ($self:ident, $iters:ident, $add:expr, $inf:expr, $k:expr) => {{
        let n = $self.pts.len();
        let mut acc = $self.pts[0];
        for i in 0..$iters as usize {
            for j in 0..$k {
                acc = $add(&*$self, black_box(acc), $self.pts[(i * 7 + j * 13 + 1) % n]);
                if $inf(&acc) {
                    acc = $self.pts[(i + 3) % n];
                }
            }
        }
        black_box(acc);
    }};
}

// --- small: ecbench Instance::Binary (FastCurve, Gf2 one word) ---------------
struct Small {
    inst: Box<crypto_lib::cryptanalysis::ic_boundary::BinaryInstance>,
    pts: Vec<FastPoint>,
    xs: Vec<u64>,
    sink: u64,
}

impl Small {
    fn new(a: u8, n: u32, rng: &mut Sm) -> Result<Self, String> {
        let cs = CurveSpec::Koblitz { a, n };
        let Instance::Binary(inst) = cs.build()? else {
            return Err(format!("K_{a}/2^{n} is not a one-word ecbench instance"));
        };
        let g = inst.generator;
        let mut pts = Vec::new();
        let mut acc = g;
        // 64 distinct multiples of G, stride-jumped by a random scalar.
        let k = 2 + rng.below(1000) as u64;
        let mut step = g;
        for _ in 0..k.trailing_zeros() + 3 {
            step = inst.fast.double(step);
        }
        while pts.len() < 64 {
            acc = inst.fast.add(acc, step);
            if !acc.infinity {
                pts.push(acc);
            }
        }
        let xs = (0..1024).map(|_| (rng.next() & ((1u64 << n) - 1)).max(1)).collect();
        Ok(Self { inst, pts, xs, sink: 0 })
    }
    #[inline(always)]
    fn add(&self, p: FastPoint, q: FastPoint) -> FastPoint {
        self.inst.fast.add(p, q)
    }
}

fn small_inf(p: &FastPoint) -> bool {
    p.infinity
}

impl Backend for Small {
    fn label(&self) -> String {
        format!("ecbench Instance::Binary {} (FastCurve::add, Gf2 {})", self.inst.name, self.inst.gf.kernel_name())
    }
    #[inline(never)]
    fn ga(&mut self, iters: u64) {
        ga_loop!(self, iters, Small::add, small_inf, 1)
    }
    #[inline(never)]
    fn ga_aa(&mut self, iters: u64) {
        ga_loop!(self, iters, Small::add, small_inf, 1)
    }
    #[inline(never)]
    fn ga4(&mut self, iters: u64) {
        ga_loop!(self, iters, Small::add, small_inf, 4)
    }
    #[inline(never)]
    fn dbl(&mut self, iters: u64) {
        let mut acc = self.pts[1];
        for i in 0..iters as usize {
            acc = self.inst.fast.double(black_box(acc));
            if acc.infinity {
                acc = self.pts[(i + 5) % 64];
            }
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul_lat(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let mut acc = self.xs[0];
        for i in 0..iters as usize {
            acc = gf.mul(acc, self.xs[i & 1023]) ^ 1;
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul8_lat(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let mut acc = self.xs[0];
        for i in 0..iters as usize {
            for j in 0..8 {
                acc = gf.mul(acc, self.xs[(i + j * 131) & 1023]) ^ 1;
            }
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul_tp(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
        for i in 0..(iters as usize).div_ceil(4) {
            let x = self.xs[i & 1023];
            a = gf.mul(a, x) ^ 1;
            b = gf.mul(b, x) ^ 2;
            c = gf.mul(c, x) ^ 3;
            d = gf.mul(d, x) ^ 5;
        }
        black_box((a, b, c, d));
    }
    #[inline(never)]
    fn sqr_lat(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let mut acc = self.xs[1];
        for i in 0..iters as usize {
            acc = gf.sqr(acc) ^ self.xs[i & 1023];
        }
        black_box(acc);
    }
    #[inline(never)]
    fn sqr_tp(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
        for i in 0..(iters as usize).div_ceil(4) {
            let x = self.xs[i & 1023];
            a = gf.sqr(a) ^ x;
            b = gf.sqr(b) ^ x;
            c = gf.sqr(c) ^ x;
            d = gf.sqr(d) ^ x;
        }
        black_box((a, b, c, d));
    }
    #[inline(never)]
    fn inv_lat(&mut self, iters: u64) {
        let gf = &self.inst.gf;
        let mut acc = 0u64;
        for i in 0..iters as usize {
            acc ^= gf.inv(self.xs[i & 1023] ^ (acc & 1)).max(1);
        }
        black_box(acc);
    }
    #[inline(never)]
    fn fadd(&mut self, iters: u64) {
        let mut acc = self.xs[2];
        for i in 0..iters as usize {
            acc = black_box(acc ^ self.xs[i & 1023]);
        }
        black_box(acc);
    }
    fn selfcheck(&mut self) -> Result<(), String> {
        let f = &self.inst.fast;
        for p in &self.pts {
            if !f.is_on_curve(*p) {
                return Err("point off curve".into());
            }
        }
        let gf = &self.inst.gf;
        for &x in &self.xs[..32] {
            if gf.mul(x, gf.inv(x)) != 1 || gf.sqr(x) != gf.mul(x, x) {
                return Err("field identity failed".into());
            }
        }
        let s = f.add(self.pts[0], self.pts[1]);
        if !f.is_on_curve(s) || f.add(s, f.neg(self.pts[1])) != self.pts[0] {
            return Err("group law check failed".into());
        }
        Ok(())
    }
}

// --- wide: ecbench Instance::Wide (WideStrongRho::add, WideGf2 u128) --------
struct Wide {
    w: Box<workload::WideInstance>,
    f: crypto_lib::cryptanalysis::wide_gf2m::WideGf2,
    pts: Vec<RawPointG<u128>>,
    xs: Vec<u128>,
    sink: u64,
}

impl Wide {
    fn new(a: u8, n: u32, rng: &mut Sm) -> Result<Self, String> {
        let cs = CurveSpec::Koblitz { a, n };
        let Instance::Wide(w) = cs.build()? else {
            return Err(format!("K_{a}/2^{n} is not a wide ecbench instance"));
        };
        let f = w.kc.field();
        let (gx, gy) = w.kc.generator;
        let g = RawPointG::Affine { x: gx, y: gy };
        let mut step = g;
        for _ in 0..(3 + rng.below(5)) {
            step = w.rho.add(step, step);
        }
        let mut acc = g;
        let mut pts = Vec::new();
        while pts.len() < 64 {
            acc = w.rho.add(acc, step);
            if let RawPointG::Affine { .. } = acc {
                pts.push(acc);
            }
        }
        let mask: u128 = (1u128 << n) - 1;
        let xs = (0..1024)
            .map(|_| ((((rng.next() as u128) << 64) | rng.next() as u128) & mask).max(1))
            .collect();
        Ok(Self { w, f, pts, xs, sink: 0 })
    }
    #[inline(always)]
    fn add(&self, p: RawPointG<u128>, q: RawPointG<u128>) -> RawPointG<u128> {
        self.w.rho.add(p, q)
    }
}

fn wide_inf(p: &RawPointG<u128>) -> bool {
    matches!(p, RawPointG::Infinity)
}

macro_rules! field_ops_by_value {
    ($ty:ty) => {
        #[inline(never)]
        fn mul_lat(&mut self, iters: u64) {
            let mut acc = self.xs[0];
            for i in 0..iters as usize {
                acc = self.f.mul(acc, self.xs[i & 1023]) ^ 1;
            }
            black_box(acc);
        }
        #[inline(never)]
        fn mul8_lat(&mut self, iters: u64) {
            let mut acc = self.xs[0];
            for i in 0..iters as usize {
                for j in 0..8 {
                    acc = self.f.mul(acc, self.xs[(i + j * 131) & 1023]) ^ 1;
                }
            }
            black_box(acc);
        }
        #[inline(never)]
        fn mul_tp(&mut self, iters: u64) {
            let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
            for i in 0..(iters as usize).div_ceil(4) {
                let x = self.xs[i & 1023];
                a = self.f.mul(a, x) ^ 1;
                b = self.f.mul(b, x) ^ 2;
                c = self.f.mul(c, x) ^ 3;
                d = self.f.mul(d, x) ^ 5;
            }
            black_box((a, b, c, d));
        }
        #[inline(never)]
        fn sqr_lat(&mut self, iters: u64) {
            let mut acc = self.xs[1];
            for i in 0..iters as usize {
                acc = self.f.sqr(acc) ^ self.xs[i & 1023];
            }
            black_box(acc);
        }
        #[inline(never)]
        fn sqr_tp(&mut self, iters: u64) {
            let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
            for i in 0..(iters as usize).div_ceil(4) {
                let x = self.xs[i & 1023];
                a = self.f.sqr(a) ^ x;
                b = self.f.sqr(b) ^ x;
                c = self.f.sqr(c) ^ x;
                d = self.f.sqr(d) ^ x;
            }
            black_box((a, b, c, d));
        }
        #[inline(never)]
        fn inv_lat(&mut self, iters: u64) {
            let mut acc: $ty = 0;
            for i in 0..iters as usize {
                acc ^= self.f.inv(self.xs[i & 1023] ^ (acc & 1)).max(1);
            }
            black_box(acc);
        }
        #[inline(never)]
        fn fadd(&mut self, iters: u64) {
            let mut acc = self.xs[2];
            for i in 0..iters as usize {
                acc = black_box(acc ^ self.xs[i & 1023]);
            }
            black_box(acc);
        }
    };
}

impl Backend for Wide {
    fn label(&self) -> String {
        format!("ecbench Instance::Wide K_{}/2^{} (WideStrongRho::add, WideGf2 u128)", self.w.kc.a, self.w.kc.n)
    }
    #[inline(never)]
    fn ga(&mut self, iters: u64) {
        ga_loop!(self, iters, Wide::add, wide_inf, 1)
    }
    #[inline(never)]
    fn ga_aa(&mut self, iters: u64) {
        ga_loop!(self, iters, Wide::add, wide_inf, 1)
    }
    #[inline(never)]
    fn ga4(&mut self, iters: u64) {
        ga_loop!(self, iters, Wide::add, wide_inf, 4)
    }
    #[inline(never)]
    fn dbl(&mut self, iters: u64) {
        let mut acc = self.pts[1];
        for i in 0..iters as usize {
            acc = self.w.rho.add(black_box(acc), acc);
            if wide_inf(&acc) {
                acc = self.pts[(i + 5) % 64];
            }
        }
        black_box(acc);
    }
    field_ops_by_value!(u128);
    fn selfcheck(&mut self) -> Result<(), String> {
        for &x in &self.xs[..32] {
            if self.f.mul(x, self.f.inv(x)) != 1 || self.f.sqr(x) != self.f.mul(x, x) {
                return Err("wide field identity failed".into());
            }
        }
        // P + Q − Q = P, with −Q = (x, x + y).
        let (p, q) = (self.pts[0], self.pts[1]);
        let s = self.add(p, q);
        let nq = match q {
            RawPointG::Affine { x, y } => RawPointG::Affine { x, y: x ^ y },
            RawPointG::Infinity => q,
        };
        if self.add(s, nq) != p {
            return Err("wide group law check failed".into());
        }
        Ok(())
    }
}

// --- w3: certicom131_symlp BinaryW (Gf2W, three words) ----------------------
struct W3c {
    c: BinaryW,
    pts: Vec<Pt<W3>>,
    xs: Vec<W3>,
    sink: u64,
}

fn w3_random(f: &Gf2W, rng: &mut Sm) -> W3 {
    let m = f.mask();
    [rng.next() & m[0], rng.next() & m[1], rng.next() & m[2]]
}

impl W3c {
    fn new(a: u8, n: u32, rng: &mut Sm) -> Result<Self, String> {
        // The registry modulus (docs/curves/registry.json); 131 is
        // z^131 + z^13 + z^2 + z + 1 (registry 0x8…2007), past u128 parsing.
        let irr = if n == 131 {
            IrreduciblePoly { degree: 131, low_terms: vec![0, 1, 2, 13] }
        } else {
            workload::registry_koblitz_modulus(a, n).ok_or("modulus not in registry")?
        };
        let c = BinaryW::new(&format!("K_{a}/2^{n} (Gf2W)"), &irr, gf2w::from_u64(a as u64), gf2w::ONE, None, None);
        let f = c.f.clone();
        let mut pts = Vec::new();
        // Lift random abscissae: y = x z with z^2 + z = x + a + b/x^2.
        while pts.len() < 64 {
            let x = w3_random(&f, rng);
            if gf2w::is_zero(&x) {
                continue;
            }
            let ix = f.inv(&x);
            let rhs = gf2w::xor(gf2w::xor(x, c.a), f.sqr(&ix));
            if let Some(z) = f.solve_quadratic(&rhs) {
                let p = Pt { x, y: f.mul(&x, &z) };
                if c.on_curve(&p) {
                    pts.push(p);
                }
            }
        }
        let xs = (0..1024).map(|_| {
            let mut v = w3_random(&f, rng);
            if gf2w::is_zero(&v) {
                v = gf2w::ONE;
            }
            v
        }).collect();
        Ok(Self { c, pts, xs, sink: 0 })
    }
    #[inline(always)]
    fn add(&self, p: Pt<W3>, q: Pt<W3>) -> Pt<W3> {
        // `None` (infinity) is mapped to a fixed point and counted as an add.
        self.c.add(&p, &q).unwrap_or(self.pts[2])
    }
}

fn w3_never_inf(_: &Pt<W3>) -> bool {
    false
}

impl Backend for W3c {
    fn label(&self) -> String {
        format!("certicom131_symlp BinaryW {} (BinaryW::add, Gf2W {})", self.c.name, self.c.f.kernel_name())
    }
    #[inline(never)]
    fn ga(&mut self, iters: u64) {
        ga_loop!(self, iters, W3c::add, w3_never_inf, 1)
    }
    #[inline(never)]
    fn ga_aa(&mut self, iters: u64) {
        ga_loop!(self, iters, W3c::add, w3_never_inf, 1)
    }
    #[inline(never)]
    fn ga4(&mut self, iters: u64) {
        ga_loop!(self, iters, W3c::add, w3_never_inf, 4)
    }
    #[inline(never)]
    fn dbl(&mut self, iters: u64) {
        let mut acc = self.pts[1];
        for _ in 0..iters as usize {
            acc = self.add(black_box(acc), acc);
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul_lat(&mut self, iters: u64) {
        let f = &self.c.f;
        let mut acc = self.xs[0];
        for i in 0..iters as usize {
            acc = gf2w::xor(f.mul(&acc, &self.xs[i & 1023]), gf2w::ONE);
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul8_lat(&mut self, iters: u64) {
        let f = &self.c.f;
        let mut acc = self.xs[0];
        for i in 0..iters as usize {
            for j in 0..8 {
                acc = gf2w::xor(f.mul(&acc, &self.xs[(i + j * 131) & 1023]), gf2w::ONE);
            }
        }
        black_box(acc);
    }
    #[inline(never)]
    fn mul_tp(&mut self, iters: u64) {
        let f = &self.c.f;
        let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
        for i in 0..(iters as usize).div_ceil(4) {
            let x = &self.xs[i & 1023];
            a = gf2w::xor(f.mul(&a, x), gf2w::ONE);
            b = gf2w::xor(f.mul(&b, x), gf2w::ONE);
            c = gf2w::xor(f.mul(&c, x), gf2w::ONE);
            d = gf2w::xor(f.mul(&d, x), gf2w::ONE);
        }
        black_box((a, b, c, d));
    }
    #[inline(never)]
    fn sqr_lat(&mut self, iters: u64) {
        let f = &self.c.f;
        let mut acc = self.xs[1];
        for i in 0..iters as usize {
            acc = gf2w::xor(f.sqr(&acc), self.xs[i & 1023]);
        }
        black_box(acc);
    }
    #[inline(never)]
    fn sqr_tp(&mut self, iters: u64) {
        let f = &self.c.f;
        let (mut a, mut b, mut c, mut d) = (self.xs[0], self.xs[1], self.xs[2], self.xs[3]);
        for i in 0..(iters as usize).div_ceil(4) {
            let x = self.xs[i & 1023];
            a = gf2w::xor(f.sqr(&a), x);
            b = gf2w::xor(f.sqr(&b), x);
            c = gf2w::xor(f.sqr(&c), x);
            d = gf2w::xor(f.sqr(&d), x);
        }
        black_box((a, b, c, d));
    }
    #[inline(never)]
    fn inv_lat(&mut self, iters: u64) {
        let f = &self.c.f;
        let mut acc = gf2w::ZERO;
        for i in 0..iters as usize {
            let mut v = gf2w::xor(self.xs[i & 1023], [acc[0] & 1, 0, 0]);
            if gf2w::is_zero(&v) {
                v = gf2w::ONE;
            }
            acc = gf2w::xor(acc, f.inv(&v));
        }
        black_box(acc);
    }
    #[inline(never)]
    fn fadd(&mut self, iters: u64) {
        let mut acc = self.xs[2];
        for i in 0..iters as usize {
            acc = black_box(gf2w::xor(acc, self.xs[i & 1023]));
        }
        black_box(acc);
    }
    fn selfcheck(&mut self) -> Result<(), String> {
        let f = &self.c.f;
        for x in &self.xs[..32] {
            if f.mul(x, &f.inv(x)) != gf2w::ONE || f.sqr(x) != f.mul(x, x) {
                return Err("Gf2W identity failed".into());
            }
        }
        let (p, q) = (self.pts[0], self.pts[1]);
        let s = self.c.add(&p, &q).ok_or("P+Q at infinity")?;
        if !self.c.on_curve(&s) || self.c.add(&s, &self.c.neg(&q)) != Some(p) {
            return Err("w3 group law check failed".into());
        }
        Ok(())
    }
}

// ---------------------------------------------------------------- field-free ops
/// Row XOR `a ^= b` of `w` words, as L2's `bxor` writes it, over a pool of
/// rows; `iters` counts words.
struct XorPool {
    w: usize,
    rows: usize,
    data: Vec<u64>,
}

impl XorPool {
    fn new(w: usize, rng: &mut Sm) -> Self {
        let rows = (65536 / w).clamp(2, 64);
        Self { w, rows, data: (0..rows * w).map(|_| rng.next()).collect() }
    }
    #[inline(never)]
    fn run(&mut self, words: u64) {
        let (w, rows) = (self.w, self.rows);
        let reps = (words as usize).div_ceil(w);
        for k in 0..reps {
            let i = k % rows;
            let j = (k * 7 + 1) % rows;
            let j = if j == i { (j + 1) % rows } else { j };
            let (lo, hi) = self.data.split_at_mut(i.max(j) * w);
            let (dst, src) = if i < j {
                (&mut lo[i * w..i * w + w], &hi[..w])
            } else {
                (&mut hi[..w], &lo[j * w..j * w + w])
            };
            for (x, y) in dst.iter_mut().zip(src.iter()) {
                *x ^= *y
            }
            black_box(&dst[0]);
        }
    }
}

/// The negative control: the loop and its `black_box`, no work.
#[inline(never)]
fn null_op(iters: u64) {
    for i in 0..iters {
        black_box(i);
    }
}

// ---------------------------------------------------------------- measure
/// Row widths: L2's Macaulay widths (1,2,3,4,5,7,8,13,16,22,23,64,97 words) plus 256 and 4096.
pub const WORDS: [usize; 15] = [1, 2, 3, 4, 5, 7, 8, 13, 16, 22, 23, 64, 97, 256, 4096];

fn ops_list() -> Vec<String> {
    let mut v: Vec<String> = ["ga", "ga_aa", "ga4", "dbl", "mul_lat", "mul8_lat", "mul_tp", "sqr_lat", "sqr_tp", "inv_lat", "fadd", "null"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for w in WORDS {
        v.push(format!("xor_w{w}"));
    }
    v
}

fn run_op(b: &mut dyn Backend, pools: &mut [XorPool], op: &str, iters: u64) -> f64 {
    let t = Instant::now();
    match op {
        "ga" => b.ga(iters),
        "ga_aa" => b.ga_aa(iters),
        "ga4" => b.ga4(iters),
        "dbl" => b.dbl(iters),
        "mul_lat" => b.mul_lat(iters),
        "mul8_lat" => b.mul8_lat(iters),
        "mul_tp" => b.mul_tp(iters),
        "sqr_lat" => b.sqr_lat(iters),
        "sqr_tp" => b.sqr_tp(iters),
        "inv_lat" => b.inv_lat(iters),
        "fadd" => b.fadd(iters),
        "null" => null_op(iters),
        x if x.starts_with("xor_w") => {
            let w: usize = x[5..].parse().unwrap();
            let p = pools.iter_mut().find(|p| p.w == w).unwrap();
            p.run(iters)
        }
        _ => panic!("unknown op {op}"),
    }
    t.elapsed().as_nanos() as f64
}

fn arg(args: &[String], k: &str) -> Option<String> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1).cloned())
}

fn host() -> Value {
    let cpu = std::fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let model = cpu.lines().find(|l| l.starts_with("model name")).map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string());
    let gov = std::fs::read_to_string("/sys/devices/system/cpu/cpu3/cpufreq/scaling_governor").ok().map(|s| s.trim().to_string());
    let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease").ok().map(|s| s.trim().to_string());
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let allowed = status.lines().find(|l| l.starts_with("Cpus_allowed_list")).map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string());
    let load = std::fs::read_to_string("/proc/loadavg").ok().map(|s| s.trim().to_string());
    json!({"cpu_model": model, "governor_cpu3": gov, "kernel": kernel, "cpus_allowed": allowed, "loadavg": load,
           "isolation": "L0: shared 4-vCPU VM, core 3 reserved by lane convention only (no isolcpus, no perf counters)"})
}

fn cmd_measure(args: &[String]) -> Result<(), String> {
    let n: u32 = arg(args, "--n").ok_or("--n")?.parse().map_err(|e| format!("{e}"))?;
    let a: u8 = arg(args, "--a").unwrap_or("0".into()).parse().map_err(|e| format!("{e}"))?;
    let imp = arg(args, "--impl").unwrap_or_else(|| if n <= 62 { "small".into() } else if n <= 127 { "wide".into() } else { "w3".into() });
    let rounds: usize = arg(args, "--rounds").unwrap_or("101".into()).parse().unwrap();
    let warmup: usize = arg(args, "--warmup").unwrap_or("10".into()).parse().unwrap();
    let seed: u64 = arg(args, "--seed").unwrap_or("20261116".into()).parse().unwrap();
    let rep: u64 = arg(args, "--rep").unwrap_or("0".into()).parse().unwrap();
    let target_ns: f64 = arg(args, "--target-ns").unwrap_or("1000000".into()).parse().unwrap();
    let mut rng = Sm(seed ^ (u64::from(n) << 32) ^ (rep << 48) ^ u64::from(a));
    let mut b: Box<dyn Backend> = match imp.as_str() {
        "small" => Box::new(Small::new(a, n, &mut rng)?),
        "wide" => Box::new(Wide::new(a, n, &mut rng)?),
        "w3" => Box::new(W3c::new(a, n, &mut rng)?),
        _ => return Err("--impl small|wide|w3".into()),
    };
    b.selfcheck()?;
    let mut pools: Vec<XorPool> = WORDS.iter().map(|&w| XorPool::new(w, &mut rng)).collect();
    let ops = ops_list();
    // Size every op so one sample takes about target_ns.
    let mut iters = Vec::new();
    for op in &ops {
        let mut it = 16u64;
        loop {
            let ns = run_op(&mut *b, &mut pools, op, it);
            if ns >= target_ns / 4.0 || it > 1 << 34 {
                let per = ns / it as f64;
                iters.push(((target_ns / per.max(1e-3)) as u64).max(16));
                break;
            }
            it *= 4;
        }
    }
    let mut samples: Vec<Vec<f64>> = vec![Vec::new(); ops.len()];
    let mut orders = Vec::new();
    for r in 0..warmup + rounds {
        // Seeded random order each round (interleaved arms).
        let mut ord: Vec<usize> = (0..ops.len()).collect();
        for i in (1..ord.len()).rev() {
            let j = rng.below(i + 1);
            ord.swap(i, j);
        }
        for &k in &ord {
            let ns = run_op(&mut *b, &mut pools, &ops[k], iters[k]);
            if r >= warmup {
                samples[k].push(ns / iters[k] as f64);
            }
        }
        if r >= warmup && orders.len() < 3 {
            orders.push(ord.iter().map(|&k| ops[k].clone()).collect::<Vec<_>>());
        }
    }
    let mut per_op = serde_json::Map::new();
    for (k, op) in ops.iter().enumerate() {
        per_op.insert(op.clone(), json!({"iters": iters[k], "ns_per_op": samples[k]}));
    }
    let out = json!({
        "schema": "binwcal.measure/v1",
        "n": n, "a": a, "impl": imp, "label": b.label(),
        "rounds": rounds, "warmup": warmup, "seed": seed, "rep": rep, "target_ns": target_ns,
        "host": host(),
        "first_orders": orders,
        "ops": per_op,
    });
    println!("{}", serde_json::to_string(&out).unwrap());
    Ok(())
}

// ---------------------------------------------------------------- linz
fn cmd_linz(args: &[String]) -> Result<(), String> {
    let n: u32 = arg(args, "--n").ok_or("--n")?.parse().unwrap();
    let m: u32 = arg(args, "--m").unwrap_or("2".into()).parse().unwrap();
    let d: u32 = arg(args, "--d").ok_or("--d")?.parse().unwrap();
    let targets: usize = arg(args, "--targets").unwrap_or("40".into()).parse().unwrap();
    let reps: usize = arg(args, "--reps").unwrap_or("3".into()).parse().unwrap();
    // The seed L2's raw file records (its run used 20261115000 + 100 n + 10 m + d,
    // not the 20261114000 of run_linz.sh); 200 samples, a = 0.
    let seed: u64 = arg(args, "--seed").ok_or("--seed (from L2 raw linz file)")?.parse().unwrap();
    let mut rows = Vec::new();
    for rep in 0..reps {
        for t in l2v::time_linz(n, 0, m, d, 200, seed, targets, &[2, 3, 4]) {
            rows.push(json!({"rep": rep, "target": t.target, "xR": t.xr, "sat": t.sat, "D": t.dd, "refuted": t.refuted,
                "word_ops": t.word_ops, "row_xors": t.row_xors, "cols": t.cols, "ns": t.ns as u64}));
        }
    }
    println!("{}", serde_json::to_string(&json!({"schema": "binwcal.linz/v1", "n": n, "m": m, "d": d, "seed": seed,
        "targets": targets, "reps": reps, "host": host(), "rows": rows})).unwrap());
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let r = match args.get(1).map(|s| s.as_str()) {
        Some("measure") => cmd_measure(&args),
        Some("linz") => cmd_linz(&args),
        Some("analyze") => analyze::run(&args),
        _ => Err("usage: binwcal measure|linz|analyze …".into()),
    };
    if let Err(e) = r {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
