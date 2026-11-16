//! Equal-advice Bernstein–Lange preprocessing-rho arm on the wide field
//! (`u128` words, `64 < n <= 127`), the twin of
//! `ic_next10_09_equal_advice.rs` (`bl` and `targets` modes, n <= 63).
//!
//! Public synthetic Koblitz fixtures only. Every field element, canonical
//! class key and point coordinate is a `u128`. The arithmetic backend is
//! chosen by the field degree: `FastBinaryCurve128` / `Gf2_128` for
//! `n > 63`, and the single-word `FastBinaryCurve` / `Gf2` (widened to `u128`
//! at the interface) for `n <= 63`. The second backend exists for one reason:
//! the instrument check at n = 61 runs the *same wide code* (u128 keys,
//! fingerprints, packing, threads) through the narrow field, so its numbers
//! can be compared with the frozen narrow instrument. It does not exercise
//! `Gf2_128`; that is checked at n = 73 and n = 83 by `selftest` (group law,
//! Frobenius, `[r]G = O`) and by the planted-collision unit tests.
//!
//! Modes:
//!
//! * `targets <n> <a> <count> <label> <points_out> <scalars_out>`: the same
//!   deterministic targets as the narrow example (`d = H(label, i) mod r`),
//!   points written as `[x,y]` with base-10 `u128` coordinates (what
//!   `koblitz_orbit_dlp_fast_online` reads on its wide path).
//! * `bl <n> <a> <table_bytes> <points> <out.jsonl>`: preprocessing rho with
//!   signed-Frobenius classes, 2,048 jumps, 4-step look-ahead, the identical
//!   map online and offline. `T = table_bytes / BL_ENTRY_BYTES` distinguished
//!   points, each stored as an 8-byte fingerprint of the canonical x plus an
//!   8-byte log (16 bytes; `r < 2^64` holds on every admitted rung and is
//!   asserted). The fingerprint is the canonical x itself when `n <= 64` and a
//!   64-bit fold of it above; a fingerprint collision can only cost one
//!   failed `[d]G = Q` check, never a wrong answer.
//! * `selftest <n> <a>`: arithmetic and planted-collision checks.
//!
//! Environment: `BL_SEED`, `BL_ONLINE_MAP_SEED` (negative control),
//! `BL_PREFIXES` (default 3; the run script uses 1), `BL_ENTRY_BYTES`
//! (default 16), `BL_THREADS` (precompute threads, default 1; with 1 the
//! precomputation is the narrow instrument's loop exactly, with more the
//! table is the same distribution from per-thread streams), `BL_LANES`
//! (batch lanes per thread, default 256), `BL_PROGRESS_S` (stderr progress
//! interval, default 60).
//!
//! The online stage is single-threaded and records the three exclusive
//! phases the `vs_rho` claim schema names for the rho arm: `walk`,
//! `collision` and `recovery_check`.

#![recursion_limit = "256"]

use crypto_lib::cryptanalysis::koblitz_fast_arith::{
    FastBinaryCurve, FastBinaryCurve128, FastPoint,
};
use crypto_lib::cryptanalysis::koblitz_index_calculus::KoblitzCurve;
use crypto_lib::cryptanalysis::semaev_decomp::{Gf2, Gf2_128};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rayon::slice::ParallelSliceMut;
use serde_json::{json, Value};
use std::io::Write;
use std::time::Instant;

type Pt = Option<(u128, u128)>;

// ---------------------------------------------------------------- arithmetic

enum Backend {
    Narrow {
        fast: FastBinaryCurve,
        gf: Gf2,
    },
    Wide {
        fast: FastBinaryCurve128,
        gf: Gf2_128,
    },
}

/// Field and group arithmetic on `u128` words over either backend.
struct Arith {
    n: u32,
    mask: u128,
    a: u128,
    backend: Backend,
}

impl Arith {
    fn new(curve: &KoblitzCurve, a: u8) -> Self {
        let n = curve.curve.irreducible.degree;
        if n <= 63 {
            let fast = FastBinaryCurve::new(&curve.curve.irreducible, a as u64).expect("n <= 63");
            let gf = Gf2::new(&curve.curve.irreducible);
            Self {
                n,
                mask: (1u128 << n) - 1,
                a: a as u128,
                backend: Backend::Narrow { fast, gf },
            }
        } else {
            let fast = FastBinaryCurve128::new(&curve.curve.irreducible, a as u128)
                .expect("wide rung must have 64 < n <= 127");
            let gf = Gf2_128::new(&curve.curve.irreducible);
            Self {
                n,
                mask: (1u128 << n) - 1,
                a: a as u128,
                backend: Backend::Wide { fast, gf },
            }
        }
    }

    fn backend_name(&self) -> &'static str {
        match self.backend {
            Backend::Narrow { .. } => "narrow_u64_field_widened",
            Backend::Wide { .. } => "wide_u128_field",
        }
    }

    #[inline(always)]
    fn mul(&self, x: u128, y: u128) -> u128 {
        match &self.backend {
            Backend::Narrow { gf, .. } => gf.mul(x as u64, y as u64) as u128,
            Backend::Wide { gf, .. } => gf.mul(x, y),
        }
    }

    #[inline(always)]
    fn sqr(&self, x: u128) -> u128 {
        match &self.backend {
            Backend::Narrow { gf, .. } => gf.sqr(x as u64) as u128,
            Backend::Wide { gf, .. } => gf.sqr(x),
        }
    }

    /// Montgomery batch inversion (zeros stay zero), in place.
    fn batch_inv(&self, xs: &mut [u128], scratch: &mut Scratch) {
        match &self.backend {
            Backend::Narrow { gf, .. } => {
                scratch.narrow.clear();
                scratch.narrow.extend(xs.iter().map(|&v| v as u64));
                gf.batch_inv(&mut scratch.narrow, &mut scratch.narrow_aux);
                for (slot, &v) in xs.iter_mut().zip(scratch.narrow.iter()) {
                    *slot = v as u128;
                }
            }
            Backend::Wide { gf, .. } => gf.batch_inv(xs, &mut scratch.wide_aux),
        }
    }

    fn add(&self, p: Pt, q: Pt) -> Pt {
        match &self.backend {
            Backend::Narrow { fast, .. } => widen(fast.add(narrow_pt(p), narrow_pt(q))),
            Backend::Wide { fast, .. } => fast.add(p, q),
        }
    }

    fn scalar_mul(&self, p: Pt, k: u64) -> Pt {
        let k = BigUint::from(k);
        match &self.backend {
            Backend::Narrow { fast, .. } => widen(fast.scalar_mul(narrow_pt(p), &k)),
            Backend::Wide { fast, .. } => fast.scalar_mul(p, &k),
        }
    }

    fn word(&self, e: &crypto_lib::binary_ecc::F2mElement) -> u128 {
        match &self.backend {
            Backend::Narrow { fast, .. } => fast.word(e) as u128,
            Backend::Wide { fast, .. } => fast.word(e),
        }
    }

    fn word_of_element(&self, e: &crypto_lib::binary_ecc::F2mElement) -> u128 {
        match &self.backend {
            Backend::Narrow { gf, .. } => gf.from_element(e) as u128,
            Backend::Wide { gf, .. } => gf.from_element(e),
        }
    }

    fn on_curve(&self, p: Pt, b: u128) -> bool {
        match p {
            None => true,
            Some((x, y)) => {
                let lhs = self.sqr(y) ^ self.mul(x, y);
                let x2 = self.sqr(x);
                let rhs = self.mul(x2, x) ^ (if self.a == 1 { x2 } else { 0 }) ^ b;
                lhs == rhs
            }
        }
    }
}

fn narrow_pt(p: Pt) -> FastPoint {
    p.map(|(x, y)| (x as u64, y as u64))
}

fn widen(p: FastPoint) -> Pt {
    p.map(|(x, y)| (x as u128, y as u128))
}

#[derive(Default)]
struct Scratch {
    narrow: Vec<u64>,
    narrow_aux: Vec<u64>,
    wide_aux: Vec<u128>,
}

// ------------------------------------------------------------- normal basis

struct Linear128 {
    tables: Vec<[u128; 256]>,
}

impl Linear128 {
    fn from_images(images: &[u128]) -> Self {
        let chunks = images.len().div_ceil(8);
        let mut tables = vec![[0u128; 256]; chunks];
        for (chunk, table) in tables.iter_mut().enumerate() {
            for byte in 1usize..256 {
                let low = byte & (byte - 1);
                let bit = chunk * 8 + byte.trailing_zeros() as usize;
                let image = images.get(bit).copied().unwrap_or(0);
                table[byte] = table[low] ^ image;
            }
        }
        Self { tables }
    }

    #[inline(always)]
    fn apply(&self, v: u128) -> u128 {
        let mut acc = 0u128;
        for (chunk, table) in self.tables.iter().enumerate() {
            acc ^= table[((v >> (8 * chunk)) & 0xff) as usize];
        }
        acc
    }
}

struct NormalBasis128 {
    n: u32,
    mask: u128,
    to_normal: Linear128,
    to_poly: Linear128,
}

impl NormalBasis128 {
    fn new(arith: &Arith) -> Self {
        let n = arith.n as usize;
        let mask = arith.mask;
        // Same search order as the narrow instrument (candidate = 2, 3, ...),
        // so the normal basis, and every canonical key, coincide at n <= 63.
        for candidate in 2u128.. {
            let mut conjugates = Vec::with_capacity(n);
            let mut value = candidate & mask;
            for _ in 0..n {
                conjugates.push(value);
                value = arith.sqr(value);
            }
            let Some(inverse_columns) = invert_columns128(&conjugates, n) else {
                continue;
            };
            return Self {
                n: arith.n,
                mask,
                to_normal: Linear128::from_images(&inverse_columns),
                to_poly: Linear128::from_images(&conjugates),
            };
        }
        unreachable!()
    }

    #[inline(always)]
    fn rotate(&self, v: u128, k: u32) -> u128 {
        if k == 0 {
            return v;
        }
        ((v << k) | (v >> (self.n - k))) & self.mask
    }

    #[inline(always)]
    fn canonical(&self, v: u128) -> (u128, u32) {
        let mut best = v;
        let mut best_shift = 0;
        let mut current = v;
        for shift in 1..self.n {
            current = ((current << 1) | (current >> (self.n - 1))) & self.mask;
            if current < best {
                best = current;
                best_shift = shift;
            }
        }
        (best, best_shift)
    }
}

/// Columns `c_i` form matrix M (poly = M * normal). Returns the images of the
/// poly basis vectors under M^{-1}, or None when M is singular.
fn invert_columns128(columns: &[u128], n: usize) -> Option<Vec<u128>> {
    let mut rows: Vec<(u128, u128)> = (0..n)
        .map(|r| {
            let mut m = 0u128;
            for (i, &column) in columns.iter().enumerate() {
                if (column >> r) & 1 == 1 {
                    m |= 1u128 << i;
                }
            }
            (m, 1u128 << r)
        })
        .collect();
    for pivot in 0..n {
        let found = (pivot..n).find(|&r| (rows[r].0 >> pivot) & 1 == 1)?;
        rows.swap(pivot, found);
        for r in 0..n {
            if r != pivot && (rows[r].0 >> pivot) & 1 == 1 {
                rows[r].0 ^= rows[pivot].0;
                rows[r].1 ^= rows[pivot].1;
            }
        }
    }
    let mut images = vec![0u128; n];
    for (i, &(_, ih)) in rows.iter().enumerate() {
        for (r, image) in images.iter_mut().enumerate() {
            if (ih >> r) & 1 == 1 {
                *image |= 1u128 << i;
            }
        }
    }
    Some(images)
}

// -------------------------------------------------------------------- misc

fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut acc = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, a, r);
        }
        a = mulmod(a, a, r);
        e >>= 1;
    }
    acc
}

#[inline(always)]
fn mix(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

/// 64-bit fold of a canonical key: the key itself when it fits one word, so
/// at `n <= 64` every hash and table order equals the narrow instrument's.
#[inline(always)]
fn fold(key: u128) -> u64 {
    let high = (key >> 64) as u64;
    (key as u64)
        ^ if high == 0 {
            0
        } else {
            mix(high ^ 0xA076_1D64_78BD_642F)
        }
}

/// Deterministic scalar stream from a label (identical to the narrow one).
struct Stream {
    state: [u8; 32],
    counter: u64,
}

impl Stream {
    fn new(label: &str) -> Self {
        Self {
            state: *blake3::hash(label.as_bytes()).as_bytes(),
            counter: 0,
        }
    }

    fn next_u128(&mut self) -> u128 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.state);
        hasher.update(&self.counter.to_le_bytes());
        self.counter += 1;
        let digest = hasher.finalize();
        u128::from_le_bytes(digest.as_bytes()[..16].try_into().unwrap())
    }

    fn scalar(&mut self, r: u64) -> u64 {
        loop {
            let v = (self.next_u128() % r as u128) as u64;
            if v != 0 {
                return v;
            }
        }
    }
}

fn peak_rss_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let kb: u64 = rest.trim().trim_end_matches("kB").trim().parse().ok()?;
            return Some(kb * 1024);
        }
    }
    None
}

// -------------------------------------------------------------------- setup

struct Setup {
    n: u32,
    a: u8,
    r: u64,
    r_bits: u32,
    lambda: u64,
    arith: Arith,
    b: u128,
    basis: NormalBasis128,
    generator: Pt,
}

fn setup(n: u32, a: u8) -> Setup {
    let curve = KoblitzCurve::new(a, n).expect("admitted Koblitz rung");
    let r = curve
        .subgroup_order
        .to_u64()
        .expect("subgroup order must fit one word (r < 2^64)");
    let lambda = curve.lambda.to_u64().expect("lambda < 2^64");
    let arith = Arith::new(&curve, a);
    let b = arith.word_of_element(&curve.curve.b);
    let basis = NormalBasis128::new(&arith);
    let generator: Pt = match curve.generator() {
        crypto_lib::binary_ecc::BinaryPoint::Affine { x, y } => {
            Some((arith.word(x), arith.word(y)))
        }
        crypto_lib::binary_ecc::BinaryPoint::Infinity => panic!("generator must be affine"),
    };
    // Frobenius acts as lambda on the subgroup and as a left rotation by one
    // in the normal basis.
    let (gx, gy) = generator.unwrap();
    assert_eq!(
        arith.scalar_mul(generator, lambda),
        Some((arith.sqr(gx), arith.sqr(gy)))
    );
    for probe in 1..512u64 {
        let x =
            (probe as u128).wrapping_mul(0x2545_F491_4F6C_DD1D_9E37_79B9_7F4A_7C15) & arith.mask;
        assert_eq!(
            basis
                .to_poly
                .apply(basis.rotate(basis.to_normal.apply(x), 1)),
            arith.sqr(x),
            "normal-basis rotation must be the Frobenius"
        );
    }
    Setup {
        n,
        a,
        r,
        r_bits: 64 - r.leading_zeros(),
        lambda,
        arith,
        b,
        basis,
        generator,
    }
}

// ------------------------------------------------------------- targets mode

fn fmt_point(p: Pt) -> String {
    let (x, y) = p.expect("affine target");
    format!("[{x},{y}]")
}

fn make_targets(s: &Setup, count: usize, label: &str) -> Vec<(u64, Pt)> {
    let mut stream = Stream::new(&format!("ic-next10-09-targets|{}|{}|{label}", s.n, s.a));
    (0..count)
        .map(|_| {
            let d = stream.scalar(s.r);
            (d, s.arith.scalar_mul(s.generator, d))
        })
        .collect()
}

fn mode_targets(args: &[String]) {
    let n: u32 = args[2].parse().unwrap();
    let a: u8 = args[3].parse().unwrap();
    let count: usize = args[4].parse().unwrap();
    let label = &args[5];
    let s = setup(n, a);
    let mut points = std::fs::File::create(&args[6]).unwrap();
    let mut scalars = std::fs::File::create(&args[7]).unwrap();
    for (d, q) in make_targets(&s, count, label) {
        writeln!(points, "{}", fmt_point(q)).unwrap();
        writeln!(scalars, "{d}").unwrap();
    }
}

fn read_points(path: &str) -> Vec<(u128, u128)> {
    std::fs::read_to_string(path)
        .expect("read targets")
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let inner = l.trim().trim_start_matches('[').trim_end_matches(']');
            let mut words = inner.split(',').map(|w| {
                w.trim()
                    .trim_matches('"')
                    .parse::<u128>()
                    .expect("coordinate")
            });
            let x = words.next().expect("x");
            let y = words.next().expect("y");
            assert!(words.next().is_none(), "point must have two coordinates");
            (x, y)
        })
        .collect()
}

// --------------------------------------------------------------- BL walking

/// Walk state: canonical point `P = [u]G + [v]Q` (online) or `[u]G` (offline).
#[derive(Clone, Copy)]
struct Lane {
    x: u128,
    y: u128,
    /// Normal-basis canonical x (class key).
    key: u128,
    u: u64,
    v: u64,
    len: u64,
}

const JUMPS: usize = 2048;
/// Look-ahead retries against fruitless 2-cycles.
const LOOKAHEAD: usize = 4;

struct Walk<'a> {
    s: &'a Setup,
    jump_points: Vec<(u128, u128)>,
    jump_logs: Vec<u64>,
    lambda_pow: Vec<u64>,
    dp_mask: u64,
    map_salt: u64,
}

impl<'a> Walk<'a> {
    fn new(s: &'a Setup, map_seed: &str, dp_bits: u32) -> Self {
        let mut stream = Stream::new(&format!("ic-next10-09-jumps|{}|{}|{map_seed}", s.n, s.a));
        let mut jump_points = Vec::with_capacity(JUMPS);
        let mut jump_logs = Vec::with_capacity(JUMPS);
        for _ in 0..JUMPS {
            let m = stream.scalar(s.r);
            jump_points.push(s.arith.scalar_mul(s.generator, m).unwrap());
            jump_logs.push(m);
        }
        let mut lambda_pow = Vec::with_capacity(s.n as usize);
        let mut acc = 1u64;
        for _ in 0..s.n {
            lambda_pow.push(acc);
            acc = mulmod(acc, s.lambda, s.r);
        }
        Self {
            s,
            jump_points,
            jump_logs,
            lambda_pow,
            dp_mask: (1u64 << dp_bits) - 1,
            map_salt: mix(stream.next_u128() as u64),
        }
    }

    #[inline(always)]
    fn partition(&self, key: u128) -> usize {
        (mix(fold(key) ^ self.map_salt) >> 40) as usize % JUMPS
    }

    #[inline(always)]
    fn is_dp(&self, key: u128) -> bool {
        (mix(fold(key).wrapping_add(0x5851_F42D_4C95_7F2D)) & self.dp_mask) == 0
    }

    /// Canonical class representative of `(x, y)` with coefficients `(u, v)`.
    #[inline(always)]
    fn canon(&self, x: u128, y: u128, u: u64, v: u64) -> Lane {
        let b = &self.s.basis;
        let r = self.s.r;
        let xn = b.to_normal.apply(x);
        let (key, k) = b.canonical(xn);
        let (cx, mut cy) = if k == 0 {
            (x, y)
        } else {
            (
                b.to_poly.apply(key),
                b.to_poly.apply(b.rotate(b.to_normal.apply(y), k)),
            )
        };
        let mut m = self.lambda_pow[k as usize];
        let other = cx ^ cy;
        if other < cy {
            cy = other;
            m = (r - m) % r;
        }
        Lane {
            x: cx,
            y: cy,
            key,
            u: mulmod(u, m, r),
            v: mulmod(v, m, r),
            len: 0,
        }
    }

    /// One iteration with look-ahead, scalar path (one inversion).
    fn step_single(&self, lane: &Lane) -> Option<Lane> {
        let mut j = self.partition(lane.key);
        let mut candidate = None;
        for attempt in 0..LOOKAHEAD {
            let (jx, jy) = self.jump_points[j];
            let sum = self.s.arith.add(Some((lane.x, lane.y)), Some((jx, jy)))?;
            let next = self.canon(
                sum.0,
                sum.1,
                (lane.u + self.jump_logs[j]) % self.s.r,
                lane.v,
            );
            let nj = self.partition(next.key);
            candidate = Some(next);
            if nj != j || attempt + 1 == LOOKAHEAD {
                break;
            }
            j = (j + 1) % JUMPS;
        }
        candidate.map(|mut c| {
            c.len = lane.len + 1;
            c
        })
    }
}

/// Batched lanes: one shared inversion per round; look-ahead retries and the
/// rare exceptional sums fall back to `step_single`, so every lane follows
/// exactly the map of `step_single`.
fn step_batch(walk: &Walk, lanes: &mut [Lane], dxs: &mut Vec<u128>, scratch: &mut Scratch) {
    let arith = &walk.s.arith;
    let a = arith.a;
    dxs.clear();
    for lane in lanes.iter() {
        let j = walk.partition(lane.key);
        let (jx, _) = walk.jump_points[j];
        let d = lane.x ^ jx;
        dxs.push(if d == 0 { 1 } else { d });
    }
    arith.batch_inv(dxs, scratch);
    for (lane, &inv) in lanes.iter_mut().zip(dxs.iter()) {
        let j = walk.partition(lane.key);
        let (jx, jy) = walk.jump_points[j];
        if lane.x == jx {
            *lane = walk.step_single(lane).unwrap_or(Lane {
                len: u64::MAX,
                ..*lane
            });
            continue;
        }
        let lambda = arith.mul(lane.y ^ jy, inv);
        let x3 = arith.sqr(lambda) ^ lambda ^ lane.x ^ jx ^ a;
        let y3 = arith.mul(lambda, lane.x ^ x3) ^ x3 ^ lane.y;
        let next = walk.canon(x3, y3, (lane.u + walk.jump_logs[j]) % walk.s.r, lane.v);
        if walk.partition(next.key) == j {
            // Look-ahead branch: recompute this step on the scalar path.
            *lane = walk.step_single(lane).unwrap_or(Lane {
                len: u64::MAX,
                ..*lane
            });
            continue;
        }
        let len = lane.len + 1;
        *lane = Lane { len, ..next };
    }
}

// ------------------------------------------------------------- precompute

struct Worker {
    lanes: Vec<Lane>,
    dxs: Vec<u128>,
    scratch: Scratch,
    stream: Stream,
    steps: u64,
    dp_walks: u64,
    abandoned: u64,
}

impl Worker {
    fn fresh(s: &Setup, walk: &Walk, stream: &mut Stream) -> Lane {
        let u = stream.scalar(s.r);
        let (x, y) = s.arith.scalar_mul(s.generator, u).unwrap();
        walk.canon(x, y, u, 0)
    }

    fn new(s: &Setup, walk: &Walk, n: u32, a: u8, seed: &str, thread: usize, lanes: usize) -> Self {
        // Thread 0 uses the narrow instrument's stream label, so a single
        // thread reproduces it; further threads get their own streams.
        let label = if thread == 0 {
            format!("ic-next10-09-pre|{n}|{a}|{seed}")
        } else {
            format!("ic-next10-09-pre|{n}|{a}|{seed}|thread{thread}")
        };
        let mut stream = Stream::new(&label);
        let lanes = (0..lanes)
            .map(|_| Self::fresh(s, walk, &mut stream))
            .collect();
        Self {
            lanes,
            dxs: Vec::new(),
            scratch: Scratch::default(),
            stream,
            steps: 0,
            dp_walks: 0,
            abandoned: 0,
        }
    }

    /// Step until `chunk` new DPs are pending or the sorted table plus the
    /// pending DPs reaches `entries` (the narrow loop's flush condition).
    fn produce(
        &mut self,
        s: &Setup,
        walk: &Walk,
        sorted_len: usize,
        chunk: usize,
        entries: usize,
        cap: u64,
    ) -> Vec<(u64, u64)> {
        let mut pending: Vec<(u64, u64)> = Vec::with_capacity(chunk + self.lanes.len());
        loop {
            step_batch(walk, &mut self.lanes, &mut self.dxs, &mut self.scratch);
            self.steps += self.lanes.len() as u64;
            for index in 0..self.lanes.len() {
                let lane = self.lanes[index];
                if lane.len == u64::MAX || lane.len > cap {
                    self.abandoned += 1;
                    self.lanes[index] = Self::fresh(s, walk, &mut self.stream);
                    continue;
                }
                if walk.is_dp(lane.key) {
                    pending.push((fold(lane.key), lane.u));
                    self.dp_walks += 1;
                    self.lanes[index] = Self::fresh(s, walk, &mut self.stream);
                }
            }
            if pending.len() >= chunk || sorted_len + pending.len() >= entries {
                return pending;
            }
        }
    }
}

struct Precomputed {
    /// Sorted by `(fingerprint, epoch << shift | log)`.
    table: Vec<(u64, u64)>,
    shift: u32,
    epochs: u64,
    steps: u64,
    dp_walks: u64,
    abandoned: u64,
    duplicates: u64,
    ms: f64,
    threads: usize,
}

fn sort_table(table: &mut [(u64, u64)], parallel: bool) {
    if parallel {
        table.par_sort_unstable();
    } else {
        table.sort_unstable();
    }
}

fn precompute(
    s: &Setup,
    walk: &Walk,
    seed: &str,
    entries: usize,
    cap: u64,
    threads: usize,
    lanes: usize,
    progress_s: u64,
) -> Precomputed {
    let started = Instant::now();
    let shift = s.r_bits;
    let epoch_bits = 64 - shift;
    assert!(
        epoch_bits >= 6,
        "log needs {shift} bits; epoch packing has {epoch_bits}"
    );
    let epoch_capacity = 1u64 << epoch_bits;
    // 64 flushes as in the narrow instrument. The epoch tag saturates at the
    // field's maximum (r near 2^57 leaves 7 bits), so a long run can never
    // overflow it; later entries then share the last tag and prefixes get
    // coarser, nothing else changes.
    let flush_every = (entries / 64).max(1024);
    let chunk = flush_every.div_ceil(threads).max(1);
    let mut workers: Vec<Worker> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads)
            .map(|t| scope.spawn(move || Worker::new(s, walk, s.n, s.a, seed, t, lanes)))
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut table: Vec<(u64, u64)> = Vec::with_capacity(entries + entries / 32 + 1024);
    let mut sorted_len = 0usize;
    let mut epoch = 0u64;
    let mut duplicates = 0u64;
    let mut last_report = Instant::now();
    while sorted_len < entries {
        let tag = epoch.min(epoch_capacity - 1);
        let produced: Vec<Vec<(u64, u64)>> = if threads == 1 {
            vec![workers[0].produce(s, walk, sorted_len, chunk, entries, cap)]
        } else {
            std::thread::scope(|scope| {
                let handles: Vec<_> = workers
                    .iter_mut()
                    .map(|w| {
                        scope.spawn(move || w.produce(s, walk, sorted_len, chunk, entries, cap))
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            })
        };
        for batch in produced {
            table.extend(batch.into_iter().map(|(fp, u)| (fp, (tag << shift) | u)));
        }
        sort_table(&mut table, threads > 1);
        let before = table.len();
        table.dedup_by(|later, earlier| later.0 == earlier.0);
        duplicates += (before - table.len()) as u64;
        sorted_len = table.len();
        epoch += 1;
        if progress_s > 0 && last_report.elapsed().as_secs() >= progress_s {
            last_report = Instant::now();
            let steps: u64 = workers.iter().map(|w| w.steps).sum();
            eprintln!(
                "bl precompute: {sorted_len}/{entries} DPs, {steps} steps, epoch {epoch}, {:.0}s",
                started.elapsed().as_secs_f64()
            );
        }
    }
    // Trim to exactly `entries`, dropping the newest epoch's surplus.
    if table.len() > entries {
        table.sort_unstable_by_key(|e| (e.1 >> shift, e.0));
        table.truncate(entries);
        sort_table(&mut table, threads > 1);
    }
    Precomputed {
        table,
        shift,
        epochs: epoch,
        steps: workers.iter().map(|w| w.steps).sum(),
        dp_walks: workers.iter().map(|w| w.dp_walks).sum(),
        abandoned: workers.iter().map(|w| w.abandoned).sum(),
        duplicates,
        ms: started.elapsed().as_secs_f64() * 1000.0,
        threads,
    }
}

/// First `size` distinct DPs by epoch, without copying the full table.
fn prefix_of(table: &[(u64, u64)], shift: u32, epochs: u64, size: usize) -> Vec<(u64, u64)> {
    let mut histogram = vec![0usize; (epochs + 2) as usize];
    for e in table.iter() {
        histogram[(e.1 >> shift) as usize] += 1;
    }
    let mut cumulative = 0usize;
    let mut boundary = 0u64;
    for (index, &count) in histogram.iter().enumerate() {
        if cumulative + count >= size {
            boundary = index as u64;
            break;
        }
        cumulative += count;
    }
    let mut partial = size - cumulative;
    let mut chosen = Vec::with_capacity(size);
    for &e in table.iter() {
        let ep = e.1 >> shift;
        if ep < boundary {
            chosen.push(e);
        } else if ep == boundary && partial > 0 {
            chosen.push(e);
            partial -= 1;
        }
    }
    chosen
}

// ------------------------------------------------------------------- online

struct OnlineResult {
    steps: u64,
    walks: u64,
    misses: u64,
    abandoned: u64,
    lookups: u64,
    verifications: u64,
    result: Option<u64>,
    walk_ms: f64,
    collision_ms: f64,
    recovery_ms: f64,
}

impl OnlineResult {
    fn total_ms(&self) -> f64 {
        self.walk_ms + self.collision_ms + self.recovery_ms
    }
}

#[allow(clippy::too_many_arguments)]
fn solve_target(
    s: &Setup,
    walk: &Walk,
    table: &[(u64, u64)],
    u_mask: u64,
    cut: Option<(u32, u64)>,
    seed: &str,
    index: usize,
    q: (u128, u128),
    walk_len: u64,
    cap: u64,
) -> OnlineResult {
    let (n, a, r) = (s.n, s.a, s.r);
    let mut tstream = Stream::new(&format!("ic-next10-09-online|{n}|{a}|{seed}|{index}"));
    let mut out = OnlineResult {
        steps: 0,
        walks: 0,
        misses: 0,
        abandoned: 0,
        lookups: 0,
        verifications: 0,
        result: None,
        walk_ms: 0.0,
        collision_ms: 0.0,
        recovery_ms: 0.0,
    };
    let mut last = Instant::now();
    let lap = |last: &mut Instant| -> f64 {
        let now = Instant::now();
        let ms = (now - *last).as_secs_f64() * 1000.0;
        *last = now;
        ms
    };
    let budget = 400 * walk_len.max(1);
    'outer: while out.steps < budget {
        out.walks += 1;
        let c = tstream.scalar(r);
        let cg = s.arith.scalar_mul(s.generator, c);
        let Some((sx, sy)) = s.arith.add(cg, Some(q)) else {
            out.walk_ms += lap(&mut last);
            continue;
        };
        let mut lane = walk.canon(sx, sy, c, 1);
        loop {
            if walk.is_dp(lane.key) {
                out.walk_ms += lap(&mut last);
                out.lookups += 1;
                let fp = fold(lane.key);
                // `cut = (shift, limit)` hides entries of epoch >= limit: a
                // zero-copy prefix table (the first `limit` flushes).
                let hit = table
                    .binary_search_by_key(&fp, |e| e.0)
                    .ok()
                    .filter(|&pos| {
                        cut.is_none_or(|(shift, limit)| (table[pos].1 >> shift) < limit)
                    });
                let mut candidate = None;
                if let Some(pos) = hit {
                    let ut = table[pos].1 & u_mask;
                    // [u]G + [v]Q = [ut]G  =>  d = (ut - u) / v.
                    if lane.v != 0 {
                        candidate =
                            Some(mulmod((ut + r - lane.u) % r, powmod(lane.v, r - 2, r), r));
                    }
                }
                out.collision_ms += lap(&mut last);
                if let Some(d) = candidate {
                    out.verifications += 1;
                    let ok = s.arith.scalar_mul(s.generator, d) == Some(q);
                    out.recovery_ms += lap(&mut last);
                    if ok {
                        out.result = Some(d);
                        break 'outer;
                    }
                }
                out.misses += 1;
                break;
            }
            if lane.len > cap || out.steps >= budget {
                out.abandoned += 1;
                out.walk_ms += lap(&mut last);
                break;
            }
            match walk.step_single(&lane) {
                Some(next) => lane = next,
                None => {
                    out.abandoned += 1;
                    out.walk_ms += lap(&mut last);
                    break;
                }
            }
            out.steps += 1;
        }
    }
    // A budget exit leaves un-lapped walk time.
    out.walk_ms += lap(&mut last);
    out
}

// ------------------------------------------------------------------ bl mode

struct BlConfig {
    n: u32,
    a: u8,
    table_bytes: u64,
    entry_bytes: u64,
    seed: String,
    online_map_seed: String,
    prefixes: u32,
    /// Explicit nested-prefix budgets in bytes (`BL_PREFIX_BYTES=a,b`), each
    /// below `table_bytes`; level 0 is always the full table.
    prefix_bytes: Vec<u64>,
    threads: usize,
    lanes: usize,
    progress_s: u64,
    /// Online passes over all targets and levels (`BL_PASSES`, default 1).
    passes: u32,
    /// With `BL_BARRIER_DIR`, the process writes `bl_ready` after the
    /// precomputation and runs pass `p` only once `go_<p>` exists, writing
    /// `done_<p>` after it. The session script uses this to interleave the
    /// passes with the index-calculus processes (ABBA).
    barrier_dir: Option<String>,
}

impl BlConfig {
    fn from_env(n: u32, a: u8, table_bytes: u64) -> Self {
        let seed = std::env::var("BL_SEED").unwrap_or_else(|_| "v1".to_owned());
        let get = |k: &str, d: u64| -> u64 {
            std::env::var(k)
                .ok()
                .map(|v| v.parse().unwrap())
                .unwrap_or(d)
        };
        Self {
            n,
            a,
            table_bytes,
            entry_bytes: get("BL_ENTRY_BYTES", 16),
            online_map_seed: std::env::var("BL_ONLINE_MAP_SEED").unwrap_or_else(|_| seed.clone()),
            seed,
            prefixes: get("BL_PREFIXES", 3) as u32,
            prefix_bytes: std::env::var("BL_PREFIX_BYTES")
                .ok()
                .map(|v| {
                    v.split(',')
                        .map(|b| b.trim().parse().expect("BL_PREFIX_BYTES"))
                        .collect()
                })
                .unwrap_or_default(),
            passes: get("BL_PASSES", 1) as u32,
            barrier_dir: std::env::var("BL_BARRIER_DIR").ok(),
            threads: get("BL_THREADS", 1).max(1) as usize,
            lanes: get("BL_LANES", 256) as usize,
            progress_s: get("BL_PROGRESS_S", 60),
        }
    }
}

/// Read by the n = 61 check (`#[cfg(test)]`); the production path only writes the JSONL.
#[cfg_attr(not(test), allow(dead_code))]
struct TargetRow {
    pass: u32,
    level: u32,
    size: usize,
    index: usize,
    res: OnlineResult,
}

/// Read by the n = 61 check (`#[cfg(test)]`); the production path only writes the JSONL.
#[cfg_attr(not(test), allow(dead_code))]
struct BlOutcome {
    summary_pre: Value,
    rows: Vec<TargetRow>,
    process_ms: f64,
}

fn run_bl(cfg: &BlConfig, targets: &[(u128, u128)], out: &mut dyn Write) -> BlOutcome {
    let process_started = Instant::now();
    let s = setup(cfg.n, cfg.a);
    let (n, a, r) = (cfg.n, cfg.a, s.r);
    let classes = r as f64 / (2.0 * n as f64);
    let entries = (cfg.table_bytes / cfg.entry_bytes) as usize;
    assert!(entries > 0, "table budget holds no entry");
    let dp_bits = ((classes / entries as f64).sqrt().log2()).round().max(0.0) as u32;
    let walk_len = 1u64 << dp_bits;
    let cap = 20 * walk_len;
    let walk = Walk::new(&s, &cfg.seed, dp_bits);
    let setup_ms = process_started.elapsed().as_secs_f64() * 1000.0;
    if std::env::var("BL_PLAN_ONLY").as_deref() == Ok("1") {
        // The plan only: the parameters a table of this budget would use, for
        // comparison with a recorded `bl_precompute` line, without walking.
        let plan = json!({
            "kind":"bl_plan", "n":n, "a":a, "r":r, "classes_N":classes, "entries":entries,
            "bytes_per_entry":cfg.entry_bytes, "dp_bits":dp_bits, "walk_len_2t":walk_len,
            "walk_cap":cap, "backend":s.arith.backend_name(),
            "predicted_precompute_steps": entries as f64 * walk_len as f64,
            "predicted_online_steps": classes / (entries as f64 * walk_len as f64) + walk_len as f64,
        });
        writeln!(out, "{plan}").unwrap();
        println!("{plan}");
        return BlOutcome {
            summary_pre: plan,
            rows: Vec::new(),
            process_ms: 0.0,
        };
    }

    let pre = precompute(
        &s,
        &walk,
        &cfg.seed,
        entries,
        cap,
        cfg.threads,
        cfg.lanes,
        cfg.progress_s,
    );
    let u_mask = if s.r_bits >= 64 {
        u64::MAX
    } else {
        (1u64 << s.r_bits) - 1
    };
    // Spot-verify table logs.
    for &(fp, packed) in pre.table.iter().step_by((entries / 64).max(1)) {
        let p = s.arith.scalar_mul(s.generator, packed & u_mask).unwrap();
        let c = walk.canon(p.0, p.1, 1, 0);
        assert_eq!(
            fold(c.key),
            fp,
            "table entry log does not reproduce its class"
        );
    }
    let summary_pre = json!({
        "kind":"bl_precompute", "n":n, "a":a, "r":r, "classes_N":classes,
        "backend":s.arith.backend_name(),
        "table_bytes_budget":cfg.table_bytes, "entries":entries,
        "bytes_per_entry":cfg.entry_bytes,
        "entry_layout":"8-byte fingerprint of canonical x + 8-byte log (16 bytes in RAM)",
        "table_ram_bytes":entries as u64 * 16,
        "dp_bits":dp_bits, "walk_len_2t":walk_len, "walk_cap":cap,
        "precompute_steps":pre.steps, "precompute_walks":pre.dp_walks,
        "precompute_abandoned":pre.abandoned, "precompute_duplicate_dps":pre.duplicates,
        "precompute_epochs":pre.epochs, "precompute_threads":pre.threads,
        "precompute_ms":pre.ms, "precompute_lanes":cfg.lanes,
        "ns_per_precompute_step":pre.ms * 1e6 / pre.steps.max(1) as f64,
        "setup_ms":setup_ms, "map_seed":cfg.seed, "online_map_seed":cfg.online_map_seed,
        "jumps":JUMPS, "lookahead":LOOKAHEAD,
        "extra_advice_bytes_jump_table": JUMPS * (16 + 16 + 8),
    });
    writeln!(out, "{summary_pre}").unwrap();
    println!("{summary_pre}");

    // ---- online: per pass, per prefix table size, every target, single walk.
    let online_walk = Walk::new(&s, &cfg.online_map_seed, dp_bits);
    // Levels: default nested /4 prefixes (copied tables, the narrow
    // instrument's behaviour), or explicit byte budgets as zero-copy epoch
    // cuts of the full table: the largest whole number of flushes whose
    // distinct DPs fit the budget, so level bytes <= the requested bytes.
    let mut level_sizes: Vec<usize> = Vec::new();
    let mut cuts: Vec<Option<(u32, u64)>> = Vec::new();
    let mut prefixes: Vec<Option<Vec<(u64, u64)>>> = Vec::new();
    if cfg.prefix_bytes.is_empty() {
        for level in 0..cfg.prefixes {
            let size = entries >> (2 * level);
            if size == 0 {
                break;
            }
            level_sizes.push(size);
            cuts.push(None);
            prefixes.push((level > 0).then(|| prefix_of(&pre.table, pre.shift, pre.epochs, size)));
        }
    } else {
        let mut histogram = vec![0usize; (pre.epochs + 2) as usize];
        for e in pre.table.iter() {
            histogram[(e.1 >> pre.shift) as usize] += 1;
        }
        level_sizes.push(entries);
        cuts.push(None);
        prefixes.push(None);
        for &bytes in &cfg.prefix_bytes {
            let budget = (bytes / cfg.entry_bytes) as usize;
            assert!(
                budget > 0 && budget < entries,
                "BL_PREFIX_BYTES must lie below the table"
            );
            let (mut count, mut limit) = (0usize, 0u64);
            for (epoch, &c) in histogram.iter().enumerate() {
                if count + c > budget {
                    break;
                }
                count += c;
                limit = epoch as u64 + 1;
            }
            level_sizes.push(count);
            cuts.push(Some((pre.shift, limit)));
            prefixes.push(None);
        }
    }
    let levels_record = json!({
        "kind":"bl_levels",
        "levels": level_sizes.iter().enumerate().map(|(level, &size)| json!({
            "level":level, "entries":size, "bytes":size as u64 * cfg.entry_bytes,
            "epoch_limit":cuts[level].map(|c| c.1),
            "requested_bytes": if level == 0 { json!(cfg.table_bytes) }
                else { cfg.prefix_bytes.get(level - 1).map_or(Value::Null, |b| json!(b)) },
        })).collect::<Vec<_>>(),
    });
    writeln!(out, "{levels_record}").unwrap();
    println!("{levels_record}");
    let mut rows = Vec::new();
    for pass in 1..=cfg.passes {
        if let Some(dir) = &cfg.barrier_dir {
            if pass == 1 {
                std::fs::write(format!("{dir}/bl_ready"), b"ready\n").unwrap();
            }
            let go = format!("{dir}/go_{pass}");
            while !std::path::Path::new(&go).exists() {
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        for (level, &size) in level_sizes.iter().enumerate() {
            let table: &[(u64, u64)] = prefixes[level].as_deref().unwrap_or(&pre.table);
            let (mut steps_sum, mut ms_sum) = (0u64, 0f64);
            for (index, &q) in targets.iter().enumerate() {
                let res = solve_target(
                    &s,
                    &online_walk,
                    table,
                    u_mask,
                    cuts[level],
                    &cfg.seed,
                    index,
                    q,
                    walk_len,
                    cap,
                );
                let elapsed = res.total_ms();
                steps_sum += res.steps;
                ms_sum += elapsed;
                let record = json!({
                    "kind":"bl_online_target", "n":n, "a":a, "target_index":index,
                    "pass":pass,
                    "target":[q.0.to_string(), q.1.to_string()],
                    "table_entries":size, "table_bytes":size as u64 * cfg.entry_bytes,
                    "prefix_level":level,
                    "dp_bits":dp_bits, "online_steps":res.steps, "walks":res.walks,
                    "dp_misses":res.misses, "abandoned":res.abandoned,
                    "table_lookups":res.lookups, "verifications":res.verifications,
                    "solved":res.result.is_some(), "recovered_scalar":res.result,
                    "online_ms":elapsed,
                    "online_phase_ms":{"walk":res.walk_ms,"collision":res.collision_ms,
                                       "recovery_check":res.recovery_ms},
                    "online_start_event":"target_walk_start",
                    "online_stop_event":"recovery_check_end",
                    "online_map_seed":cfg.online_map_seed,
                    "budget_steps":400 * walk_len.max(1),
                });
                writeln!(out, "{record}").unwrap();
                rows.push(TargetRow {
                    pass,
                    level: level as u32,
                    size,
                    index,
                    res,
                });
            }
            let count = targets.len().max(1) as f64;
            println!(
                "{}",
                json!({"kind":"bl_online_summary","pass":pass,"prefix_level":level,
                       "table_entries":size,"mean_steps":steps_sum as f64 / count,
                       "mean_ms":ms_sum / count,"targets":targets.len()})
            );
        }
        out.flush().unwrap();
        if let Some(dir) = &cfg.barrier_dir {
            std::fs::write(format!("{dir}/done_{pass}"), b"done\n").unwrap();
        }
    }
    // Null control: walk online on a different jump map than the table was
    // built with. The table then helps only through random DP hits, so the
    // steps per target should sit at the random-hit level N/T, far above the
    // matched-map level.
    let null_targets = std::env::var("BL_NULL_TARGETS")
        .ok()
        .map(|v| v.parse::<usize>().unwrap())
        .unwrap_or(0)
        .min(targets.len());
    if null_targets > 0 {
        let null_walk = Walk::new(&s, "null-control-mismatch", dp_bits);
        for (index, &q) in targets.iter().enumerate().take(null_targets) {
            let res = solve_target(
                &s, &null_walk, &pre.table, u_mask, None, &cfg.seed, index, q, walk_len, cap,
            );
            let record = json!({
                "kind":"bl_null_target", "n":n, "a":a, "target_index":index,
                "table_entries":entries, "online_steps":res.steps, "walks":res.walks,
                "dp_misses":res.misses, "solved":res.result.is_some(),
                "recovered_scalar":res.result, "online_ms":res.total_ms(),
                "random_hit_level_N_over_T": classes / entries as f64,
            });
            writeln!(out, "{record}").unwrap();
        }
    }
    let process_ms = process_started.elapsed().as_secs_f64() * 1000.0;
    let summary = json!({
        "kind":"bl_summary", "process_ms":process_ms,
        "peak_rss_bytes":peak_rss_bytes(),
    });
    writeln!(out, "{summary}").unwrap();
    println!("{summary}");
    BlOutcome {
        summary_pre,
        rows,
        process_ms,
    }
}

fn mode_bl(args: &[String]) {
    let n: u32 = args[2].parse().unwrap();
    let a: u8 = args[3].parse().unwrap();
    let table_bytes: u64 = args[4].parse().unwrap();
    let targets = read_points(&args[5]);
    let mut out = std::fs::File::create(&args[6]).unwrap();
    let cfg = BlConfig::from_env(n, a, table_bytes);
    run_bl(&cfg, &targets, &mut out);
}

// ------------------------------------------------------------ selftest mode

/// Planted-collision check: the table holds the offline walk that the online
/// walk of each target joins at its first step, so the target must be
/// recovered. Exercises canonicalisation, the jump map, batching and the
/// recovery algebra on the active backend without a real precomputation.
fn planted_check(s: &Setup, count: usize, dp_bits: u32) -> usize {
    let walk = Walk::new(s, "planted", dp_bits);
    let walk_len = 1u64 << dp_bits;
    let cap = 20 * walk_len;
    let targets = make_targets(s, count, "planted");
    let mut table: Vec<(u64, u64)> = Vec::new();
    let mut stream = Stream::new("planted-streams");
    let _ = &mut stream;
    for (index, &(d, _q)) in targets.iter().enumerate() {
        let mut tstream = Stream::new(&format!(
            "ic-next10-09-online|{}|{}|planted|{index}",
            s.n, s.a
        ));
        let c = tstream.scalar(s.r);
        // Online walk 0 starts at Q + [c]G = [d + c]G with (u, v) = (c, 1).
        let u0 = ((d as u128 + c as u128) % s.r as u128) as u64;
        let (x, y) = s.arith.scalar_mul(s.generator, u0).unwrap();
        let mut lane = walk.canon(x, y, u0, 0);
        let mut guard = 0u64;
        while !walk.is_dp(lane.key) {
            lane = walk.step_single(&lane).expect("walk step");
            guard += 1;
            assert!(guard < cap, "planted offline walk exceeded its cap");
        }
        table.push((fold(lane.key), lane.u));
    }
    table.sort_unstable();
    let u_mask = if s.r_bits >= 64 {
        u64::MAX
    } else {
        (1u64 << s.r_bits) - 1
    };
    let mut solved = 0;
    for (index, &(d, q)) in targets.iter().enumerate() {
        let res = solve_target(
            s,
            &walk,
            &table,
            u_mask,
            None,
            "planted",
            index,
            q.unwrap(),
            walk_len,
            cap,
        );
        assert_eq!(res.result, Some(d), "planted target {index} not recovered");
        solved += 1;
    }
    solved
}

fn arithmetic_check(s: &Setup) {
    let arith = &s.arith;
    let g = s.generator;
    assert!(arith.on_curve(g, s.b), "generator must lie on the curve");
    assert_eq!(arith.scalar_mul(g, s.r), None, "[r]G must be the identity");
    let mut stream = Stream::new(&format!("selftest|{}|{}", s.n, s.a));
    for _ in 0..16 {
        let (k1, k2) = (stream.scalar(s.r), stream.scalar(s.r));
        let p1 = arith.scalar_mul(g, k1);
        let p2 = arith.scalar_mul(g, k2);
        assert!(arith.on_curve(p1, s.b) && arith.on_curve(p2, s.b));
        let sum = arith.add(p1, p2);
        let k3 = ((k1 as u128 + k2 as u128) % s.r as u128) as u64;
        assert_eq!(sum, arith.scalar_mul(g, k3), "[k1]G + [k2]G != [k1+k2]G");
    }
    // Field: x * x^-1 = 1 via batch inversion, and squaring is multiplication.
    let mut xs: Vec<u128> = (1..=64u128)
        .map(|i| (i.wrapping_mul(0x9E37_79B9_7F4A_7C15_F39C_C060_5CED_C835)) & arith.mask | 1)
        .collect();
    let originals = xs.clone();
    let mut scratch = Scratch::default();
    arith.batch_inv(&mut xs, &mut scratch);
    for (x, inv) in originals.iter().zip(xs.iter()) {
        assert_eq!(arith.mul(*x, *inv), 1, "batch inverse");
        assert_eq!(arith.sqr(*x), arith.mul(*x, *x), "sqr != mul");
    }
}

fn mode_selftest(args: &[String]) {
    let n: u32 = args[2].parse().unwrap();
    let a: u8 = args[3].parse().unwrap();
    let started = Instant::now();
    let s = setup(n, a);
    arithmetic_check(&s);
    let solved = planted_check(&s, 6, 3);
    let record = json!({
        "kind":"bl_selftest", "n":n, "a":a, "r":s.r, "r_bits":s.r_bits,
        "backend":s.arith.backend_name(), "arithmetic_checks":"pass",
        "planted_targets_recovered":solved, "ms":started.elapsed().as_secs_f64() * 1000.0,
    });
    println!("{record}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("targets") => mode_targets(&args),
        Some("bl") => mode_bl(&args),
        Some("selftest") => mode_selftest(&args),
        _ => {
            eprintln!("modes: targets | bl | selftest (see the header of this file)");
            std::process::exit(2);
        }
    }
}

// -------------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/research/ic_next10_20261014/09-equal-advice/data"
    );

    /// One target's counters as the narrow instrument
    /// (`ic_next10_09_equal_advice bl`) printed them: steps, walks, DP
    /// misses, abandoned walks, recovered scalar.
    type Row = (u64, u64, u64, u64, u64);

    /// Narrow instrument, `bl 61 0 65536` on the first 8 frozen targets
    /// (seed `v1`, `BL_PREFIXES` 3): (dp_bits, entries, precompute steps,
    /// walks, abandoned, duplicate DPs).
    const N61_MULTI_BYTES: u64 = 65_536;
    const N61_MULTI_PRE: (u32, u64, u64, u64, u64, u64) = (14, 4096, 92_374_272, 5652, 0, 1556);

    fn n61_multi_rows() -> Vec<Vec<Row>> {
        vec![
            vec![
                (29588, 3, 2, 0, 34549989762167),
                (33100, 3, 2, 0, 85532182102172),
                (7841, 1, 0, 0, 159972314695470),
                (4632, 1, 0, 0, 127299469972423),
                (13205, 1, 0, 0, 126000694232380),
                (9721, 1, 0, 0, 74513354092942),
                (96623, 3, 2, 0, 152589611815278),
                (40760, 3, 2, 0, 65196123840149),
            ],
            vec![
                (193513, 10, 9, 0, 34549989762167),
                (38210, 4, 3, 0, 85532182102172),
                (7841, 1, 0, 0, 159972314695470),
                (114398, 9, 8, 0, 127299469972423),
                (31470, 3, 2, 0, 126000694232380),
                (93989, 3, 2, 0, 74513354092942),
                (143266, 4, 3, 0, 152589611815278),
                (40760, 3, 2, 0, 65196123840149),
            ],
            vec![
                (854578, 42, 41, 0, 34549989762167),
                (360412, 26, 25, 0, 85532182102172),
                (7841, 1, 0, 0, 159972314695470),
                (608967, 34, 33, 0, 127299469972423),
                (115957, 8, 7, 0, 126000694232380),
                (93989, 3, 2, 0, 74513354092942),
                (179803, 5, 4, 0, 152589611815278),
                (47986, 4, 3, 0, 65196123840149),
            ],
        ]
    }

    /// Run the wide instrument at n = 61 on the first targets of the frozen
    /// item-9 fixture and compare every counter with the narrow
    /// instrument's, run with the same seeds and the same table budget.
    fn n61_matches_narrow(
        table_bytes: u64,
        prefixes: u32,
        expect_pre: (u32, u64, u64, u64, u64, u64),
        expect_rows: &[Vec<Row>],
    ) {
        let targets: Vec<(u128, u128)> = read_points(&format!("{FIXTURE}/targets_n61.txt"))
            .into_iter()
            .take(expect_rows[0].len())
            .collect();
        let scalars: Vec<u64> = std::fs::read_to_string(format!("{FIXTURE}/scalars_n61.txt"))
            .unwrap()
            .lines()
            .take(targets.len())
            .map(|l| l.parse().unwrap())
            .collect();
        let mut cfg = BlConfig::from_env(61, 0, table_bytes);
        // Defaults, whatever the caller's environment says.
        cfg.seed = "v1".into();
        cfg.online_map_seed = "v1".into();
        cfg.prefixes = prefixes;
        cfg.prefix_bytes = Vec::new();
        cfg.entry_bytes = 16;
        cfg.threads = 1;
        cfg.lanes = 256;
        cfg.progress_s = 0;
        cfg.passes = 1;
        cfg.barrier_dir = None;
        let mut sink = Vec::new();
        let outcome = run_bl(&cfg, &targets, &mut sink);
        let pre = &outcome.summary_pre;
        let (dp_bits, entries, steps, walks, abandoned, duplicates) = expect_pre;
        assert_eq!(pre["backend"], "narrow_u64_field_widened");
        assert_eq!(pre["dp_bits"], dp_bits);
        assert_eq!(pre["entries"], entries);
        assert_eq!(pre["precompute_steps"], steps);
        assert_eq!(pre["precompute_walks"], walks);
        assert_eq!(pre["precompute_abandoned"], abandoned);
        assert_eq!(pre["precompute_duplicate_dps"], duplicates);
        for (level, expected) in expect_rows.iter().enumerate() {
            for (index, &(st, wk, ms, ab, d)) in expected.iter().enumerate() {
                let row = outcome
                    .rows
                    .iter()
                    .find(|r| r.level as usize == level && r.index == index)
                    .unwrap();
                assert_eq!(
                    (
                        row.res.steps,
                        row.res.walks,
                        row.res.misses,
                        row.res.abandoned
                    ),
                    (st, wk, ms, ab),
                    "level {level} target {index}"
                );
                assert_eq!(row.res.result, Some(d), "level {level} target {index}");
                // The held-back scalar of the frozen fixture.
                assert_eq!(Some(scalars[index] % setup(61, 0).r), row.res.result);
            }
        }
    }

    /// Several flushes and three prefix levels; about 15 s in release, far
    /// too slow for a debug build, hence ignored by default: `cargo test --release --example
    /// ic_equal_advice_wide -- --ignored`.
    #[test]
    #[ignore]
    fn n61_wide_instrument_matches_narrow_multi_epoch() {
        n61_matches_narrow(N61_MULTI_BYTES, 3, N61_MULTI_PRE, &n61_multi_rows());
    }

    #[test]
    fn arithmetic_and_planted_collisions_n73_and_n83() {
        for (n, a) in [(73u32, 0u8), (83, 1)] {
            let s = setup(n, a);
            assert_eq!(s.arith.backend_name(), "wide_u128_field");
            arithmetic_check(&s);
            assert_eq!(planted_check(&s, 4, 3), 4);
        }
    }

    #[test]
    fn narrow_backend_planted_n61() {
        let s = setup(61, 0);
        assert_eq!(s.arith.backend_name(), "narrow_u64_field_widened");
        arithmetic_check(&s);
        assert_eq!(planted_check(&s, 4, 3), 4);
    }

    #[test]
    fn rung_constants_are_the_frozen_ones() {
        let s73 = setup(73, 0);
        assert_eq!(s73.r, 86_020_738_150_056_119);
        assert_eq!(s73.r_bits, 57);
        let s83 = setup(83, 1);
        assert_eq!(s83.r, 8_569_786_107_849_059);
        assert_eq!(s83.r_bits, 53);
        // Advice budgets of PROTOCOL.md and the plans they imply.
        assert_eq!(21 * 435_652_020u64 + 5 * (1u64 << 30), 14_517_401_540);
        for (r, n, bytes, dp_bits) in [
            (s73.r, 73u32, 5_630_296_064u64, 10u32),
            (s83.r, 83, 14_954_881_024, 8),
        ] {
            let classes = r as f64 / (2.0 * n as f64);
            let entries = (bytes / 16) as f64;
            assert_eq!(((classes / entries).sqrt().log2()).round() as u32, dp_bits);
        }
    }

    #[test]
    fn fingerprint_is_the_key_when_it_fits_one_word() {
        assert_eq!(fold(0x1234_5678_9abc_def0u128), 0x1234_5678_9abc_def0);
        let wide = (5u128 << 64) | 7;
        assert_ne!(fold(wide), 7);
        assert_eq!(fold(wide), fold(wide));
    }
}
