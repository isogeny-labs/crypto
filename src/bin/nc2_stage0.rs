//! NC-2 Stage 0 (`research/curve25519_autolab_20261104/ideas/NONCOLLISION-20261108.md`).
//!
//! Zero elliptic-curve group operations: everything is scalar arithmetic mod
//! the prime `r`.  Unknowns are `d` and `C` seed logs `s_1..s_C` (`C` = 0, 1
//! or 2; `C = 0` is the K = 1 cross-check of NC-1).  A design is an oblivious
//! sequence of points with exact scalar `c_t = a_t + b_t d + sum_c e_tc s_c`,
//! generated one addition (one jump vector) per point, stored by the key
//! `min(c, r - c)` so that `c_t` and `-c_t` collide.  A collision `c_t = +-c_u`
//! gives the row `(da, db, de..)` (difference when the scalars are equal, sum
//! when they are opposite) which holds at the true point.  Rows with zero seed
//! part are discarded (the guard).  The run stops when the rows' coefficient
//! columns `(db, de..)` reach rank `K = C + 1` over `Z_r`; the cost is the
//! number of points generated (stored points plus lookup-only probes).
//!
//! # Design table (fixed before any result was read)
//!
//! | id  | what |
//! |-----|------|
//! | D0  | control: 16 uniform random jump vectors in `Z_r^(2+C)`, jump chosen by a stream independent of the point (oblivious), fresh design per sample. Poisson law: E = 1.329 (C=1), 1.662 (C=2), 0.886 (C=0) in sqrt(r) units. |
//! | D0i | diagnostic only: every point a fresh uniform vector (Poisson law without walk correlations). |
//! | D0h | diagnostic only: as D0 but the jump is a hash of the current scalar `c_t` (the literal wording of the task). Trails then merge after a collision. |
//! | D1  | the `wwalk` C = 1 arm (`g=1,grow=1`, `targets=stride-generator`): table walk `E_t = E_{t-1} + J_h`, `J = [alpha]P1 + [gamma]G`, alpha uniform in `[1, round(1.2 sqrt(T))]`, gamma uniform in `[1, r)`, `T = round(x sqrt(r))`, extended up to `2T` until the first free row; then probes `Q + kG`, `k = 0, 1, ..` (lookup only, one addition each). Fresh jumps per sample. |
//! | D2  | control, stride tiling: serpentine enumeration of a (C+2)-dimensional box, one axis (one stride) per step, strides `m^k`, `m = ceil(r^(1/(C+2)))`, seed axes strided like a and b. Deterministic. (A first draft, babies `(0,0,j)` and giants `(0,1,-i m)`, had a = 0 everywhere: every row is then proportional to `(1, -d/s)`, rank 1 forever; replaced before any D2 number was used.) |
//! | D3  | pre-registered search: a genome of at most 16 jump vectors built by per-coordinate rules (zero, one, uniform, small, arithmetic progression, quadratic in the jump index = conic-like), a schedule (random stream, cyclic, ruler, permuted rounds) and a start vector; random-restart hill climbing on Monte Carlo E over a TRAINING sample set; the winner (lowest training E) is re-scored on a disjoint fresh sample set; only that held-out value counts. |
//! | CE  | randomised-e control: the winner of D3 with every seed coefficient of every jump (and of the start) replaced by uniform random values. E must return to the Poisson value. |
//!
//! # SNC-2 Stage 0 pre-registration (`--chained`; written and committed before any chained number was run)
//!
//! Record: `research/curve25519_autolab_20261104/ideas/SYNTHESIS-NC-20261108.md`, SNC-2 [pcc:unverified].
//! Zero group operations, C = 2 only, rank-3 stop rule on `(db, e1, e2)` with the seed-part guard (the engine above,
//! unchanged). Exact chained design `CH`, fixed here:
//!
//! * 16 jumps `J_j = [a_j]P1 + [a'_j]P2 + K_j`, i.e. scalar vector `(K_j.c, K_j.b, a_j, a'_j)` over `(G, Q, P1, P2)`.
//! * `K_0, K_1`: independent uniform in `Z_r^2` (the `[c]G + [b]Q` part). `K_j = K_{j-1} + K_{j-2}` mod r for
//!   `j = 2..15` (one addition per stored jump point; set-up, not counted in E, as in NC-2).
//! * `a_j, a'_j`: independent uniform in `[1, M]`, `M = round(1.2 sqrt(T))`, `T = round(1.75 sqrt(r))` (the wwalk
//!   alpha rule at the record's predicted walk length). Buildable by small scalar multiplications.
//! * Start point: `(c, b)` uniform in `Z_r^2`, `(e1, e2)` uniform in `[1, M]`. Jump choice: a stream independent
//!   of the point (as D0). Jumps, start and stream are fresh per sample (as D0, D1). One addition per stored point.
//! * Stop: rank 3 of the rows' `(db, e1, e2)` columns after the guard; cost = points generated; cap 10 sqrt(r).
//! * Samples: 2000 per curve, `(d, s1, s2)` uniform, the same held-out samples as the NC-2 run (seed ^ 0xAAAA0001).
//! * Curves: the nine NC-2 r values (k = 20: 131203, 131317, 131111; k = 24: 2097349, 2097559, 2097523;
//!   k = 28: 33558163, 33554761, 33558059). Seed 20261108.
//! * Per-k E = mean of the three curve means; its SE = sqrt(sum of the three squared SEs) / 3.
//!
//! Controls (stop and report if any is outside its interval; nothing is tuned):
//!
//! * D0i (iid) at C = 2, n = 20000, first r of each size: E in [1.652, 1.672] (record: 1.662 +- 0.01).
//! * D0 (NC-2 16-jump uniform random walk) at C = 2, n = 2000, all nine curves: E in [1.66, 1.76]
//!   (NC-2 values were 1.684 to 1.740; the walk excess over the Poisson floor is 3.5 to 4.5 percent).
//! * CE (randomised-e control): the CH design with `(a_j, a'_j)` and start seed coordinates replaced by uniform
//!   values in `Z_r`, n = 2000, all nine curves: E in [1.66, 1.77] ("returns to the Poisson value" plus the walk
//!   excess; if CH shows structure, CE removes the seed-axis part of it).
//!
//! Falsification line (record): SNC-2 is dead at Stage 0 if chained E (per-k mean of CH) is above 1.82 at two of
//! the three sizes. The record's prediction is 1.75 [1.70, 1.82]. Only the point value of the per-k mean is compared.

use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};

// ---------------------------------------------------------------- rng

#[derive(Clone)]
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        let mut r = Rng(seed ^ 0x9E37_79B9_7F4A_7C15);
        r.next();
        r
    }
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix(self.0)
    }
    /// Uniform in `[0, n)` (multiply-shift; bias below 2^-30 for n < 2^34).
    fn below(&mut self, n: u64) -> u64 {
        ((self.next() as u128 * n as u128) >> 64) as u64
    }
}

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[derive(Default)]
struct IdHasher(u64);
impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
}
type FastMap = HashMap<u64, u32, BuildHasherDefault<IdHasher>>;

// ------------------------------------------------------------ modular

fn mulmod(a: u64, b: u64, r: u64) -> u64 {
    ((a as u128 * b as u128) % r as u128) as u64
}
fn addmod(a: u64, b: u64, r: u64) -> u64 {
    let s = a + b;
    if s >= r {
        s - r
    } else {
        s
    }
}
fn submod(a: u64, b: u64, r: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + r - b
    }
}
fn powmod(mut a: u64, mut e: u64, r: u64) -> u64 {
    let mut acc = 1u64;
    a %= r;
    while e > 0 {
        if e & 1 == 1 {
            acc = mulmod(acc, a, r);
        }
        a = mulmod(a, a, r);
        e >>= 1;
    }
    acc
}
fn invmod(a: u64, r: u64) -> u64 {
    powmod(a, r - 2, r)
}

// ---------------------------------------------------------- row space

/// Rank of a set of coefficient vectors over `Z_r` (`r` prime), up to 3
/// columns, kept in echelon form.
#[derive(Clone, Debug)]
struct RowSpace {
    r: u64,
    cols: usize,
    basis: Vec<([u64; 3], usize)>,
}

impl RowSpace {
    fn new(r: u64, cols: usize) -> Self {
        RowSpace {
            r,
            cols,
            basis: Vec::new(),
        }
    }
    fn rank(&self) -> usize {
        self.basis.len()
    }
    /// Add a vector; true if it was independent of the rows so far.
    fn add(&mut self, mut v: [u64; 3]) -> bool {
        let r = self.r;
        for x in v.iter_mut() {
            *x %= r;
        }
        for c in self.cols..3 {
            v[c] = 0;
        }
        for (b, p) in &self.basis {
            if v[*p] != 0 {
                let f = v[*p];
                for c in 0..3 {
                    v[c] = submod(v[c], mulmod(f, b[c], r), r);
                }
            }
        }
        match v.iter().position(|&x| x != 0) {
            None => false,
            Some(p) => {
                let inv = invmod(v[p], r);
                for x in v.iter_mut() {
                    *x = mulmod(*x, inv, r);
                }
                self.basis.push((v, p));
                true
            }
        }
    }
}

// ------------------------------------------------------------- engine

type Vec4 = [u64; 4]; // (a, b, e1, e2)

struct Pt {
    v: Vec4,
    /// false: lookup only (a probe), not inserted.
    store: bool,
}

/// An oblivious point generator.  `c_prev` is the previous point's scalar
/// (only the literal-hash control D0h reads it), `rank` the rank so far.
trait Gen {
    fn next(&mut self, c_prev: u64, rank: usize) -> Pt;
}

#[derive(Clone, Copy, Debug, Default)]
struct Outcome {
    points: u64,
    done: bool,
    rows_used: u32,
    guard_dropped: u32,
    dependent: u32,
    collisions: u32,
}

/// One Monte Carlo sample: run `gen` at the true point `(d, s)` until the
/// row space reaches rank `C + 1` or `cap` points were generated.
fn run_sample<G: Gen>(
    gen: &mut G,
    r: u64,
    c: usize,
    d: u64,
    s: [u64; 2],
    cap: u64,
    map: &mut FastMap,
    store: &mut Vec<Vec4>,
) -> Outcome {
    map.clear();
    store.clear();
    let k = c + 1;
    let mut space = RowSpace::new(r, k);
    let mut out = Outcome::default();
    let mut c_prev = 0u64;
    while out.points < cap {
        let pt = gen.next(c_prev, space.rank());
        out.points += 1;
        let v = pt.v;
        let mut cv = addmod(v[0] % r, mulmod(v[1] % r, d, r), r);
        for i in 0..c {
            cv = addmod(cv, mulmod(v[2 + i] % r, s[i], r), r);
        }
        c_prev = cv;
        if cv == 0 {
            continue; // the identity point: no key
        }
        let key = cv.min(r - cv);
        if let Some(&idx) = map.get(&key) {
            out.collisions += 1;
            let u = store[idx as usize];
            // The scalars decide which of the two relations holds.
            let other_c = {
                let mut x = addmod(u[0] % r, mulmod(u[1] % r, d, r), r);
                for i in 0..c {
                    x = addmod(x, mulmod(u[2 + i] % r, s[i], r), r);
                }
                x
            };
            let same = other_c == cv;
            let mut row = [0u64; 4];
            for i in 0..4 {
                row[i] = if same {
                    submod(v[i] % r, u[i] % r, r)
                } else {
                    addmod(v[i] % r, u[i] % r, r)
                };
            }
            if accept_row(&mut space, row, c, &mut out) && space.rank() == k {
                out.done = true;
                return out;
            }
        } else if pt.store {
            map.insert(key, store.len() as u32);
            store.push(v);
        }
    }
    out
}

/// Guard and rank logic for one collision row `(da, db, de1, de2)`.  Rows
/// with zero seed part are dropped (C >= 1); the rest are added to the row
/// space.  Returns true if the row raised the rank.
fn accept_row(space: &mut RowSpace, row: Vec4, c: usize, out: &mut Outcome) -> bool {
    if c >= 1 && row[2..2 + c].iter().all(|&x| x == 0) {
        out.guard_dropped += 1;
        return false;
    }
    if space.add([row[1], row[2], row[3]]) {
        out.rows_used += 1;
        true
    } else {
        out.dependent += 1;
        false
    }
}

// ----------------------------------------------------------- generators

/// A walk `v_t = v_{t-1} + J_h` over an explicit jump list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sched {
    /// h from a seeded stream (independent of the point).
    Random,
    /// h = t mod n.
    Cyclic,
    /// h = (number of trailing zeros of t+1) mod n.
    Ruler,
    /// rounds of n steps, each round a fresh random permutation.
    Permuted,
    /// literal-hash control (D0h): h from the scalar c_t.
    HashScalar,
}

struct WalkGen {
    r: u64,
    jumps: Vec<Vec4>,
    sched: Sched,
    v: Vec4,
    t: u64,
    rng: Rng,
    perm: Vec<usize>,
    salt: u64,
}

impl WalkGen {
    fn new(r: u64, jumps: Vec<Vec4>, sched: Sched, start: Vec4, stream_seed: u64) -> Self {
        let n = jumps.len();
        WalkGen {
            r,
            jumps,
            sched,
            v: start,
            t: 0,
            rng: Rng::new(stream_seed),
            perm: (0..n).collect(),
            salt: stream_seed,
        }
    }
}

impl Gen for WalkGen {
    fn next(&mut self, c_prev: u64, _rank: usize) -> Pt {
        let n = self.jumps.len();
        if self.t == 0 {
            self.t = 1;
            return Pt {
                v: self.v,
                store: true,
            };
        }
        let step = self.t - 1;
        let h = match self.sched {
            Sched::Random => self.rng.below(n as u64) as usize,
            Sched::Cyclic => (step % n as u64) as usize,
            Sched::Ruler => ((step + 1).trailing_zeros() as usize) % n,
            Sched::Permuted => {
                let pos = (step % n as u64) as usize;
                if pos == 0 {
                    for i in (1..n).rev() {
                        let j = self.rng.below(i as u64 + 1) as usize;
                        self.perm.swap(i, j);
                    }
                }
                self.perm[pos]
            }
            Sched::HashScalar => (mix(c_prev ^ self.salt) % n as u64) as usize,
        };
        self.t += 1;
        let j = self.jumps[h];
        for i in 0..4 {
            self.v[i] = addmod(self.v[i], j[i], self.r);
        }
        Pt {
            v: self.v,
            store: true,
        }
    }
}

/// Diagnostic D0i: every point is a fresh uniform vector (not a walk, hence
/// not one addition per point); isolates the Poisson law from walk
/// correlations.
struct IidGen {
    r: u64,
    c: usize,
    rng: Rng,
}

impl Gen for IidGen {
    fn next(&mut self, _c: u64, _rank: usize) -> Pt {
        let mut v = [0u64; 4];
        for slot in v.iter_mut().take(2 + self.c) {
            *slot = self.rng.below(self.r);
        }
        Pt { v, store: true }
    }
}

/// D1: the `wwalk` C = 1 table-and-probe structure.
struct WwalkGen {
    r: u64,
    t_table: u64,
    t_max: u64,
    jumps: Vec<(u64, u64)>, // (alpha, gamma)
    rng: Rng,
    steps: u64,
    ev: (u64, u64), // (u = coefficient of P1, z = coefficient of G)
    k: u64,
}

impl WwalkGen {
    fn new(r: u64, x: f64, jumps: usize, seed: u64) -> Self {
        let t = (x * (r as f64).sqrt()).round() as u64;
        let w = ((1.2 * (t as f64).sqrt()).round() as u64).max(2);
        let mut rng = Rng::new(seed);
        let js = (0..jumps)
            .map(|_| (1 + rng.below(w), 1 + rng.below(r - 1)))
            .collect();
        WwalkGen {
            r,
            t_table: t,
            t_max: 2 * t,
            jumps: js,
            rng,
            steps: 0,
            ev: (1, 0),
            k: 0,
        }
    }
}

impl Gen for WwalkGen {
    fn next(&mut self, _c: u64, rank: usize) -> Pt {
        let in_table = self.steps < self.t_table || (rank == 0 && self.steps < self.t_max);
        if in_table {
            let (al, ga) = self.jumps[self.rng.below(self.jumps.len() as u64) as usize];
            // u is an integer coefficient in the real table; only u mod r
            // enters the scalar.
            self.ev.0 = (self.ev.0 + al) % self.r;
            self.ev.1 = addmod(self.ev.1, ga, self.r);
            self.steps += 1;
            Pt {
                v: [self.ev.1, 0, self.ev.0, 0],
                store: true,
            }
        } else {
            let k = self.k;
            self.k += 1;
            Pt {
                v: [k % self.r, 1, 0, 0],
                store: false,
            }
        }
    }
}

/// D2: stride tiling.  The points are the serpentine (reflected mixed-radix
/// Gray) enumeration of a `(C+2)`-dimensional box of side `n`: one axis moves
/// by one stride per point (one addition), coordinate `k` of a point is
/// `digit_k * m^k mod r` with `m = ceil(r^(1/(C+2)))`.  The seed axes are
/// strided like the `a`, `b` axes.  Differences of box points repeat heavily
/// (about 3^dims n^dims distinct differences for n^(2 dims) pairs), so this is
/// the redundant-coverage control.
struct StrideGen {
    r: u64,
    dims: usize,
    n: u64,
    scale: [u64; 4],
    digit: [u64; 4],
    dir: [i8; 4],
    v: Vec4,
    started: bool,
}

impl StrideGen {
    fn new(r: u64, c: usize) -> Self {
        let dims = c + 2;
        let m = (r as f64).powf(1.0 / dims as f64).ceil() as u64;
        let mut scale = [0u64; 4];
        let mut p = 1u64;
        for slot in scale.iter_mut().take(dims) {
            *slot = p % r;
            p = p.saturating_mul(m);
        }
        let cap_points = CAP_X * (r as f64).sqrt();
        let n = cap_points.powf(1.0 / dims as f64).ceil() as u64 + 1;
        let mut v = [0u64; 4];
        let mut digit = [0u64; 4];
        for k in 0..dims {
            digit[k] = 1;
            v[k] = scale[k];
        }
        StrideGen {
            r,
            dims,
            n,
            scale,
            digit,
            dir: [1; 4],
            v,
            started: false,
        }
    }
}

impl Gen for StrideGen {
    fn next(&mut self, _c: u64, _rank: usize) -> Pt {
        if !self.started {
            self.started = true;
            return Pt {
                v: self.v,
                store: true,
            };
        }
        for k in 0..self.dims {
            let nd = self.digit[k] as i64 + self.dir[k] as i64;
            if nd >= 1 && nd <= self.n as i64 {
                self.digit[k] = nd as u64;
                self.v[k] = if self.dir[k] > 0 {
                    addmod(self.v[k], self.scale[k], self.r)
                } else {
                    submod(self.v[k], self.scale[k], self.r)
                };
                break;
            }
            self.dir[k] = -self.dir[k];
        }
        Pt {
            v: self.v,
            store: true,
        }
    }
}

// ----------------------------------------------------------- D3 genome

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rule {
    Zero,
    One,
    Uniform,
    /// signed value in `[-w, w]`, nonzero.
    Small(u64),
    /// `x0 + h delta` over the jump index `h`.
    Ap,
    /// `x0 + h d1 + h^2 d2`: the (h^2, h, 1) conic-like pattern.
    Quad,
}

#[derive(Clone, Debug)]
struct Genome {
    n: usize,
    rules: [Rule; 4],
    sched: Sched,
    jump_seed: u64,
    stream_seed: u64,
    start_uniform: bool,
    start_seed: u64,
}

fn small_rule_widths() -> [u64; 6] {
    [2, 4, 8, 16, 64, 1024]
}

fn rand_rule(rng: &mut Rng) -> Rule {
    match rng.below(6) {
        0 => Rule::Zero,
        1 => Rule::One,
        2 => Rule::Uniform,
        3 => Rule::Small(small_rule_widths()[rng.below(6) as usize]),
        4 => Rule::Ap,
        _ => Rule::Quad,
    }
}

fn rand_sched(rng: &mut Rng) -> Sched {
    match rng.below(4) {
        0 => Sched::Random,
        1 => Sched::Cyclic,
        2 => Sched::Ruler,
        _ => Sched::Permuted,
    }
}

impl Genome {
    fn random(rng: &mut Rng, c: usize) -> Genome {
        let mut rules = [Rule::Zero; 4];
        for (i, slot) in rules.iter_mut().enumerate() {
            if i < 2 + c {
                *slot = rand_rule(rng);
            }
        }
        Genome {
            n: 2 + rng.below(15) as usize,
            rules,
            sched: rand_sched(rng),
            jump_seed: rng.next(),
            stream_seed: rng.next(),
            start_uniform: rng.below(2) == 0,
            start_seed: rng.next(),
        }
    }

    fn mutate(&self, rng: &mut Rng, c: usize) -> Genome {
        let mut g = self.clone();
        match rng.below(7) {
            0 | 1 => {
                let i = rng.below(2 + c as u64) as usize;
                g.rules[i] = rand_rule(rng);
            }
            2 => g.jump_seed = rng.next(),
            3 => g.n = 2 + rng.below(15) as usize,
            4 => g.sched = rand_sched(rng),
            5 => {
                g.stream_seed = rng.next();
                g.start_seed = rng.next();
            }
            _ => g.start_uniform = !g.start_uniform,
        }
        g
    }

    /// Materialise the jump vectors and start vector for modulus `r`.
    fn build(&self, r: u64, c: usize) -> (Vec<Vec4>, Vec4) {
        let mut rng = Rng::new(self.jump_seed);
        let mut jumps = vec![[0u64; 4]; self.n];
        for coord in 0..4 {
            let rule = if coord < 2 + c {
                self.rules[coord]
            } else {
                Rule::Zero
            };
            let (x0, d1, d2) = (rng.below(r), rng.below(r), rng.below(r));
            for (h, j) in jumps.iter_mut().enumerate() {
                let h = h as u64;
                j[coord] = match rule {
                    Rule::Zero => 0,
                    Rule::One => 1 % r,
                    Rule::Uniform => rng.below(r),
                    Rule::Small(w) => {
                        let mag = 1 + rng.below(w.min(r - 1));
                        if rng.below(2) == 0 {
                            mag
                        } else {
                            r - mag
                        }
                    }
                    Rule::Ap => addmod(x0, mulmod(h, d1, r), r),
                    Rule::Quad => addmod(
                        addmod(x0, mulmod(h, d1, r), r),
                        mulmod(mulmod(h, h, r), d2, r),
                        r,
                    ),
                };
            }
        }
        let mut srng = Rng::new(self.start_seed);
        let mut start = [0u64; 4];
        if self.start_uniform {
            for slot in start.iter_mut().take(2 + c) {
                *slot = srng.below(r);
            }
        } else {
            start[2] = 1 % r;
        }
        (jumps, start)
    }

    fn describe(&self, c: usize) -> String {
        format!(
            "n={} sched={:?} rules(a,b,e..)={:?} start={} jump_seed={:#x} stream_seed={:#x} start_seed={:#x}",
            self.n,
            self.sched,
            &self.rules[..2 + c],
            if self.start_uniform { "uniform" } else { "e1=1" },
            self.jump_seed,
            self.stream_seed,
            self.start_seed
        )
    }
}

// ------------------------------------------------------------ sampling

#[derive(Clone, Copy)]
struct Sample {
    d: u64,
    s: [u64; 2],
    /// per-sample seed for designs that are fresh per sample (D0, D1).
    design: u64,
}

fn make_samples(r: u64, base: u64, n: usize) -> Vec<Sample> {
    (0..n as u64)
        .map(|i| {
            let mut rng = Rng::new(mix(base) ^ mix(i.wrapping_add(0x5151)));
            Sample {
                d: rng.below(r),
                s: [rng.below(r), rng.below(r)],
                design: rng.next(),
            }
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Stat {
    mean: f64,
    se: f64,
    n: usize,
    censored: usize,
}

fn stat(points: &[(u64, bool)], r: u64) -> Stat {
    let sq = (r as f64).sqrt();
    let xs: Vec<f64> = points.iter().map(|p| p.0 as f64 / sq).collect();
    let n = xs.len() as f64;
    let mean = xs.iter().sum::<f64>() / n;
    let var = xs.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>() / (n - 1.0).max(1.0);
    Stat {
        mean,
        se: (var / n).sqrt(),
        n: xs.len(),
        censored: points.iter().filter(|p| !p.1).count(),
    }
}

const CAP_X: f64 = 10.0;

/// Evaluate `mk(sample)` generators over `samples` on all cores.
fn evaluate<F, G>(r: u64, c: usize, samples: &[Sample], mk: F) -> Stat
where
    F: Fn(&Sample) -> G + Sync,
    G: Gen,
{
    let cap = (CAP_X * (r as f64).sqrt()) as u64;
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(samples.len().max(1));
    let chunk = samples.len().div_ceil(threads);
    let mut all: Vec<(u64, bool)> = Vec::with_capacity(samples.len());
    std::thread::scope(|sc| {
        let hs: Vec<_> = samples
            .chunks(chunk.max(1))
            .map(|ch| {
                let mk = &mk;
                sc.spawn(move || {
                    let mut map = FastMap::default();
                    let mut store = Vec::new();
                    ch.iter()
                        .map(|smp| {
                            let mut g = mk(smp);
                            let o =
                                run_sample(&mut g, r, c, smp.d, smp.s, cap, &mut map, &mut store);
                            (o.points, o.done)
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for h in hs {
            all.extend(h.join().expect("worker"));
        }
    });
    stat(&all, r)
}

fn eval_d0(r: u64, c: usize, samples: &[Sample], hashed: bool) -> Stat {
    evaluate(r, c, samples, |smp| {
        let mut rng = Rng::new(smp.design);
        let jumps = (0..16)
            .map(|_| {
                let mut j = [0u64; 4];
                for slot in j.iter_mut().take(2 + c) {
                    *slot = rng.below(r);
                }
                j
            })
            .collect();
        let mut start = [0u64; 4];
        for slot in start.iter_mut().take(2 + c) {
            *slot = rng.below(r);
        }
        let sched = if hashed {
            Sched::HashScalar
        } else {
            Sched::Random
        };
        WalkGen::new(r, jumps, sched, start, rng.next())
    })
}

fn eval_d0i(r: u64, c: usize, samples: &[Sample]) -> Stat {
    evaluate(r, c, samples, |smp| IidGen {
        r,
        c,
        rng: Rng::new(smp.design),
    })
}

fn eval_d1(r: u64, samples: &[Sample], x: f64, jumps: usize) -> Stat {
    evaluate(r, 1, samples, |smp| WwalkGen::new(r, x, jumps, smp.design))
}

fn eval_d2(r: u64, c: usize, samples: &[Sample]) -> Stat {
    evaluate(r, c, samples, |_| StrideGen::new(r, c))
}

fn eval_genome(r: u64, c: usize, samples: &[Sample], g: &Genome) -> Stat {
    let (jumps, start) = g.build(r, c);
    evaluate(r, c, samples, |_| {
        WalkGen::new(r, jumps.clone(), g.sched, start, g.stream_seed)
    })
}

/// The randomised-e control: seed coefficients of every jump and of the
/// start replaced by uniform random values, everything else unchanged.
fn eval_randomised_e(r: u64, c: usize, samples: &[Sample], g: &Genome, seed: u64) -> Stat {
    let (mut jumps, mut start) = g.build(r, c);
    let mut rng = Rng::new(seed);
    for j in jumps.iter_mut() {
        for slot in j.iter_mut().take(2 + c).skip(2) {
            *slot = rng.below(r);
        }
    }
    for slot in start.iter_mut().take(2 + c).skip(2) {
        *slot = rng.below(r);
    }
    evaluate(r, c, samples, |_| {
        WalkGen::new(r, jumps.clone(), g.sched, start, g.stream_seed)
    })
}

/// SNC-2 chained design (see the pre-registration in the header): wwalk alpha bound.
fn chain_m(r: u64) -> u64 {
    let t = (1.75 * (r as f64).sqrt()).round();
    ((1.2 * t.sqrt()).round() as u64).max(1)
}

/// The chained C = 2 jump set and start vector.  `K_0, K_1` uniform in
/// `Z_r^2` (coordinates 0 and 1 = G and Q parts), `K_j = K_{j-1} + K_{j-2}`;
/// seed coordinates uniform in `[1, M]` (or in `Z_r` for the randomised-e
/// control `uniform_e`).
fn build_chain(r: u64, rng: &mut Rng, uniform_e: bool) -> (Vec<Vec4>, Vec4) {
    let m = chain_m(r);
    let seed_coord = |rng: &mut Rng| {
        if uniform_e {
            rng.below(r)
        } else {
            1 + rng.below(m)
        }
    };
    let mut k = vec![[rng.below(r), rng.below(r)], [rng.below(r), rng.below(r)]];
    for j in 2..16 {
        let (p, q) = (k[j - 1], k[j - 2]);
        k.push([addmod(p[0], q[0], r), addmod(p[1], q[1], r)]);
    }
    let jumps = k
        .iter()
        .map(|kj| [kj[0], kj[1], seed_coord(rng), seed_coord(rng)])
        .collect();
    let start = [rng.below(r), rng.below(r), seed_coord(rng), seed_coord(rng)];
    (jumps, start)
}

fn eval_chain(r: u64, samples: &[Sample], uniform_e: bool) -> Stat {
    evaluate(r, 2, samples, |smp| {
        let mut rng = Rng::new(smp.design ^ 0xC4A1_0005);
        let (jumps, start) = build_chain(r, &mut rng, uniform_e);
        WalkGen::new(r, jumps, Sched::Random, start, rng.next())
    })
}

/// Aggregate per-curve stats: mean of means, SE = sqrt(sum se^2) / m.
fn aggregate(stats: &[Stat]) -> (f64, f64) {
    let m = stats.len() as f64;
    let mean = stats.iter().map(|s| s.mean).sum::<f64>() / m;
    let se = stats.iter().map(|s| s.se * s.se).sum::<f64>().sqrt() / m;
    (mean, se)
}

struct SearchResult {
    best: Genome,
    train: Stat,
    evals: usize,
}

fn search(
    r: u64,
    c: usize,
    train: &[Sample],
    restarts: usize,
    steps: usize,
    seed: u64,
) -> SearchResult {
    let mut rng = Rng::new(seed);
    let mut best: Option<(Genome, Stat)> = None;
    let mut evals = 0;
    for _ in 0..restarts {
        let mut cur = Genome::random(&mut rng, c);
        let mut cur_s = eval_genome(r, c, train, &cur);
        evals += 1;
        for _ in 0..steps {
            let cand = cur.mutate(&mut rng, c);
            let s = eval_genome(r, c, train, &cand);
            evals += 1;
            if s.mean <= cur_s.mean {
                cur = cand;
                cur_s = s;
            }
        }
        if best.as_ref().is_none_or(|b| cur_s.mean < b.1.mean) {
            best = Some((cur, cur_s));
        }
    }
    let (best, train) = best.expect("at least one restart");
    SearchResult { best, train, evals }
}

// ----------------------------------------------------------------- CLI

struct Args {
    rs: Vec<u64>,
    cs: Vec<usize>,
    n: usize,
    seed: u64,
    d1: Option<(f64, usize)>,
    d3: bool,
    train: usize,
    restarts: usize,
    steps: usize,
    chained: bool,
}

fn parse_args() -> Result<Args, String> {
    let mut a = Args {
        rs: vec![131_203],
        cs: vec![1, 2],
        n: 2000,
        seed: 20_261_108,
        d1: None,
        d3: true,
        train: 300,
        restarts: 8,
        steps: 40,
        chained: false,
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let bad = |s: &str| format!("bad value for {s}");
    while i < argv.len() {
        let key = argv[i].as_str();
        let val = argv.get(i + 1).map(|s| s.as_str());
        match (key, val) {
            ("--r", Some(v)) => {
                a.rs = v
                    .split(',')
                    .map(|x| x.parse().map_err(|_| bad(key)))
                    .collect::<Result<_, _>>()?
            }
            ("--c", Some(v)) => {
                a.cs = v
                    .split(',')
                    .map(|x| x.parse().map_err(|_| bad(key)))
                    .collect::<Result<_, _>>()?
            }
            ("--n", Some(v)) => a.n = v.parse().map_err(|_| bad(key))?,
            ("--seed", Some(v)) => a.seed = v.parse().map_err(|_| bad(key))?,
            ("--train", Some(v)) => a.train = v.parse().map_err(|_| bad(key))?,
            ("--restarts", Some(v)) => a.restarts = v.parse().map_err(|_| bad(key))?,
            ("--steps", Some(v)) => a.steps = v.parse().map_err(|_| bad(key))?,
            ("--d1", Some(v)) => {
                let (x, j) = v.split_once(',').ok_or_else(|| bad(key))?;
                a.d1 = Some((
                    x.parse().map_err(|_| bad(key))?,
                    j.parse().map_err(|_| bad(key))?,
                ));
            }
            ("--chained", _) => {
                a.chained = true;
                i += 1;
                continue;
            }
            ("--no-d3", _) => {
                a.d3 = false;
                i += 1;
                continue;
            }
            _ => return Err(format!("unknown or incomplete argument `{key}`")),
        }
        i += 2;
    }
    if a.cs.iter().any(|&c| c > 2) {
        return Err("--c values are 0, 1 or 2".into());
    }
    Ok(a)
}

fn fmt(s: &Stat) -> String {
    format!(
        "{:.4} +- {:.4} (n={}{})",
        s.mean,
        s.se,
        s.n,
        if s.censored > 0 {
            format!(", {} censored at cap", s.censored)
        } else {
            String::new()
        }
    )
}

fn main() {
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("nc2_stage0: {e}");
            eprintln!("usage: nc2_stage0 --r R1,R2 --c 1,2 [--n 2000] [--seed S] [--d1 x,jumps] [--no-d3] [--train 300] [--restarts 8] [--steps 40]");
            std::process::exit(2);
        }
    };
    println!(
        "# nc2_stage0 args: r={:?} c={:?} n={} seed={} d1={:?} d3={} train={} restarts={} steps={}",
        args.rs,
        args.cs,
        args.n,
        args.seed,
        args.d1,
        args.d3,
        args.train,
        args.restarts,
        args.steps
    );
    if args.chained {
        run_chained(&args);
        return;
    }
    let poisson = |c: usize| match c {
        0 => 0.886,
        1 => 1.329,
        _ => 1.662,
    };
    for &r in &args.rs {
        for &c in &args.cs {
            let held = make_samples(r, args.seed ^ 0xAAAA_0001, args.n);
            let train = make_samples(r, args.seed ^ 0x5555_0002, args.train);
            println!(
                "\n## r={r} C={c} (K={}, sqrt(r)={:.1}) Poisson floor {:.3}",
                c + 1,
                (r as f64).sqrt(),
                poisson(c)
            );
            let d0 = eval_d0(r, c, &held, false);
            println!("D0  random-jump control (16 jumps): E = {}", fmt(&d0));
            let d0i = eval_d0i(r, c, &held);
            println!(
                "D0i i.i.d. uniform points (diagnostic, not a walk): E = {}",
                fmt(&d0i)
            );
            let d0h = eval_d0(r, c, &held, true);
            println!("D0h hash-of-scalar jump (diagnostic): E = {}", fmt(&d0h));
            if c == 1 {
                if let Some((x, j)) = args.d1 {
                    let d1 = eval_d1(r, &held, x, j);
                    println!("D1  wwalk x={x} jumps={j} (table+probe): E = {}", fmt(&d1));
                }
            }
            let d2 = eval_d2(r, c, &held);
            println!(
                "D2  stride tiling (serpentine box; E is a lower bound when censored): E = {}",
                fmt(&d2)
            );
            if args.d3 && c >= 1 {
                let sr = search(
                    r,
                    c,
                    &train,
                    args.restarts,
                    args.steps,
                    args.seed ^ 0x3333_0003,
                );
                let ho = eval_genome(r, c, &held, &sr.best);
                println!(
                    "D3  search: {} evals; winner {}",
                    sr.evals,
                    sr.best.describe(c)
                );
                println!("D3  training E = {}", fmt(&sr.train));
                println!("D3  HELD-OUT E = {}", fmt(&ho));
                println!(
                    "D3  D0/D3 = {:.4}   D3/D0 = {:.4}",
                    d0.mean / ho.mean,
                    ho.mean / d0.mean
                );
                let ce = eval_randomised_e(r, c, &held, &sr.best, args.seed ^ 0x7777_0004);
                println!("CE  randomised-e control of D3 winner: E = {}", fmt(&ce));
            }
        }
    }
}

/// SNC-2 Stage 0: CH, D0, D0i and CE at C = 2 on every `--r`, plus the
/// per-invocation aggregates (one invocation per size).
fn run_chained(args: &Args) {
    let mut agg: [Vec<Stat>; 4] = Default::default();
    for &r in &args.rs {
        let held = make_samples(r, args.seed ^ 0xAAAA_0001, args.n);
        println!(
            "\n## r={r} C=2 (K=3, sqrt(r)={:.1}) Poisson floor 1.662, M={}",
            (r as f64).sqrt(),
            chain_m(r)
        );
        let rows = [
            ("CH ", eval_chain(r, &held, false)),
            ("D0 ", eval_d0(r, 2, &held, false)),
            ("D0i", eval_d0i(r, 2, &held)),
            ("CE ", eval_chain(r, &held, true)),
        ];
        for (i, (name, st)) in rows.iter().enumerate() {
            println!("{name} E = {}", fmt(st));
            agg[i].push(*st);
        }
    }
    println!(
        "\n## aggregate over {} curves (mean of means, SE = sqrt(sum se^2)/m)",
        args.rs.len()
    );
    for (name, v) in ["CH ", "D0 ", "D0i", "CE "].iter().zip(agg.iter()) {
        let (m, se) = aggregate(v);
        println!("AGG {name} E = {m:.4} +- {se:.4}");
    }
}

// ---------------------------------------------------------------- tests

#[cfg(test)]
mod tests {
    use super::*;

    const R: u64 = 131_203;

    #[test]
    fn rowspace_rank_and_dependence() {
        let mut s = RowSpace::new(R, 3);
        assert!(s.add([1, 2, 3]));
        // a multiple (mod r) of the first row is dependent
        assert!(!s.add([2, 4, 6]));
        assert!(!s.add([R + 1, R + 2, 3 * (R + 1)]));
        assert!(s.add([0, 1, 0]));
        assert_eq!(s.rank(), 2);
        assert!(!s.add([1, 3, 3])); // = row1 + row2 - ... check below
        assert!(s.add([0, 0, 1]));
        assert_eq!(s.rank(), 3);
        assert!(!s.add([5, 7, 11]));
        assert!(!s.add([0, 0, 0]));
    }

    #[test]
    fn rowspace_respects_column_count() {
        // with 2 columns, the third coordinate is ignored
        let mut s = RowSpace::new(R, 2);
        assert!(s.add([1, 0, 9]));
        assert!(!s.add([2, 0, 5]));
        assert!(s.add([0, 1, 7]));
        assert_eq!(s.rank(), 2);
        assert!(!s.add([3, 4, 1]));
    }

    #[test]
    fn guard_drops_rows_with_zero_seed_part() {
        let mut space = RowSpace::new(R, 2);
        let mut out = Outcome::default();
        // C = 1: row with e = 0 is dropped even though b != 0
        assert!(!accept_row(&mut space, [5, 7, 0, 0], 1, &mut out));
        assert_eq!(out.guard_dropped, 1);
        assert_eq!(space.rank(), 0);
        // seed part nonzero: used
        assert!(accept_row(&mut space, [5, 0, 3, 0], 1, &mut out));
        assert_eq!(space.rank(), 1);
        // dependent row
        assert!(!accept_row(&mut space, [9, 0, 6, 0], 1, &mut out));
        assert_eq!(out.dependent, 1);
        // C = 0 has no guard: rank logic only
        let mut sp0 = RowSpace::new(R, 1);
        assert!(accept_row(&mut sp0, [1, 4, 0, 0], 0, &mut out));
        assert!(!accept_row(&mut sp0, [1, 0, 0, 0], 0, &mut out));
    }

    /// Records the rows the engine accepts and checks them at the true point.
    #[test]
    fn rows_hold_at_the_true_point_and_sign_logic_is_right() {
        let r = 1009u64;
        for seed in 0..40u64 {
            let mut rng = Rng::new(seed);
            let (d, s) = (rng.below(r), [rng.below(r), rng.below(r)]);
            let jumps: Vec<Vec4> = (0..8)
                .map(|_| [rng.below(r), rng.below(r), rng.below(r), rng.below(r)])
                .collect();
            let mut g = WalkGen::new(r, jumps, Sched::Random, [0, 0, 1, 0], seed);
            // replay by hand: collect every collision row and check it
            let mut seen: HashMap<u64, Vec4> = HashMap::new();
            let mut n_rows = 0;
            for _ in 0..400 {
                let v = g.next(0, 0).v;
                let cv = (v[0] + v[1] * d % r + v[2] * s[0] % r + v[3] * s[1] % r) % r;
                if cv == 0 {
                    continue;
                }
                let key = cv.min(r - cv);
                if let Some(u) = seen.get(&key) {
                    let cu = (u[0] + u[1] * d % r + u[2] * s[0] % r + u[3] * s[1] % r) % r;
                    let same = cu == cv;
                    let row: Vec<u64> = (0..4)
                        .map(|i| {
                            if same {
                                (v[i] + r - u[i]) % r
                            } else {
                                (v[i] + u[i]) % r
                            }
                        })
                        .collect();
                    let val = (row[0] + row[1] * d % r + row[2] * s[0] % r + row[3] * s[1] % r) % r;
                    assert_eq!(val, 0, "row must vanish at the true point");
                    n_rows += 1;
                } else {
                    seen.insert(key, v);
                }
            }
            assert!(n_rows > 0, "400 points mod 1009 must collide");
        }
    }

    #[test]
    fn run_sample_stops_at_rank_k_and_counts_points() {
        // A hand-made generator: points 1..: c = e1 * s with e1 = 1,2,..;
        // then probes with b = 1.  With s = 3, d = 5 mod r the table value
        // 3*e1 collides with the probe d + k when 5 + k = 3 e1.
        struct Fixed(Vec<Vec4>, usize);
        impl Gen for Fixed {
            fn next(&mut self, _c: u64, _r: usize) -> Pt {
                let v = self.0[self.1.min(self.0.len() - 1)];
                let store = v[1] == 0;
                self.1 += 1;
                Pt { v, store }
            }
        }
        let r = 1009u64;
        // table: (z, 0, u): c = z + u s ; two entries equal up to sign
        // s = 3: c1 = 1 + 2*3 = 7 ; c2 = 0 + (-7 mod) ... use e1 so that
        // c2 = -c1: e1 = -2 -> (r-2): c2 = (r-2)*3 + 1 = 3r - 5 = -5 mod r. no.
        // choose z so that c2 = -c1 exactly: z2 = -c1 - u2*s
        let (d, s) = (5u64, [3u64, 0]);
        let c1 = (1 + 2 * 3) % r; // z=1, u=2
        let u2 = 4u64;
        let z2 = (r - c1 + r - u2 * 3 % r) % r; // c2 = z2 + 12 = -7
        let pts = vec![
            [1, 0, 2, 0],   // c = 7
            [z2, 0, u2, 0], // c = -7: collision with the first, row = sum
            [0, 1, 0, 0],   // probe d + 0 = 5, no hit
            [1, 1, 0, 0],   // 6 no hit
            [2, 1, 0, 0],   // 7 = c1 -> hit: row = probe - first = (2-1, 1, -2)
        ];
        let mut g = Fixed(pts, 0);
        let (mut map, mut store) = (FastMap::default(), Vec::new());
        let o = run_sample(&mut g, r, 1, d, s, 100, &mut map, &mut store);
        assert!(o.done);
        assert_eq!(o.points, 5);
        assert_eq!(o.rows_used, 2);
        assert_eq!(o.collisions, 2);
    }

    #[test]
    fn guard_blocks_a_pure_d_row() {
        // two probe-like stored points with equal e (=0): the collision row
        // has zero seed part and must not count, so the run never finishes.
        struct Fixed2(u64);
        impl Gen for Fixed2 {
            fn next(&mut self, _c: u64, _r: usize) -> Pt {
                self.0 += 1;
                // c = d + k for k = 0.. and -(d + k') is hit when k + k' = -2d
                Pt {
                    v: [self.0 - 1, 1, 0, 0],
                    store: true,
                }
            }
        }
        let r = 1009u64;
        let (mut map, mut store) = (FastMap::default(), Vec::new());
        let o = run_sample(
            &mut Fixed2(0),
            r,
            1,
            7,
            [11, 0],
            3 * r,
            &mut map,
            &mut store,
        );
        assert!(!o.done);
        assert!(o.guard_dropped > 0);
        assert_eq!(o.rows_used, 0);
    }

    #[test]
    fn d0_is_a_random_walk_near_the_poisson_floor_at_c1_small_r() {
        let r = 131_203u64;
        let smp = make_samples(r, 1, 400);
        let s = eval_d0(r, 1, &smp, false);
        assert!(s.mean > 1.1 && s.mean < 1.6, "got {}", s.mean);
        assert_eq!(s.censored, 0);
    }

    #[test]
    fn samples_are_disjoint_between_train_and_held_out() {
        let a = make_samples(R, 1, 500);
        let b = make_samples(R, 2, 500);
        let set: std::collections::HashSet<(u64, u64, u64)> =
            a.iter().map(|s| (s.d, s.s[0], s.s[1])).collect();
        assert!(b.iter().all(|s| !set.contains(&(s.d, s.s[0], s.s[1]))));
    }

    #[test]
    fn chain_recurrence_and_ranges() {
        let r = 2_097_349u64;
        let m = chain_m(r);
        assert_eq!(m, 60); // round(1.2 * sqrt(round(1.75 * 1448.2)))
        let mut rng = Rng::new(7);
        let (j, start) = build_chain(r, &mut rng, false);
        assert_eq!(j.len(), 16);
        for t in 2..16 {
            assert_eq!(j[t][0], (j[t - 1][0] + j[t - 2][0]) % r);
            assert_eq!(j[t][1], (j[t - 1][1] + j[t - 2][1]) % r);
        }
        for v in j.iter().chain(std::iter::once(&start)) {
            assert!((1..=m).contains(&v[2]) && (1..=m).contains(&v[3]));
        }
        // randomised-e control: seed coordinates range over Z_r
        let (ju, _) = build_chain(r, &mut Rng::new(7), true);
        assert!(ju.iter().any(|v| v[2] > m || v[3] > m));
    }

    #[test]
    fn aggregate_combines_means_and_ses() {
        let mk = |mean, se| Stat {
            mean,
            se,
            n: 1,
            censored: 0,
        };
        let (m, se) = aggregate(&[mk(1.0, 0.3), mk(2.0, 0.4), mk(3.0, 0.0)]);
        assert!((m - 2.0).abs() < 1e-12);
        assert!((se - 0.5 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn chain_runs_and_is_deterministic_small_r() {
        let r = 131_203u64;
        let samples = make_samples(r, 11, 60);
        let a = eval_chain(r, &samples, false);
        let b = eval_chain(r, &samples, false);
        assert_eq!(a.mean, b.mean);
        assert!(a.mean > 0.5 && a.mean < 10.5);
    }
}
