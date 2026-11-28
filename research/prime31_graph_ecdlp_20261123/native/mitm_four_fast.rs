//! Candidate collection loop for `mitm_four.rs` (same source, same output).
//!
//! `mitm_four.rs` stays the unchanged baseline. This binary runs the same
//! fixture, pair table, queries, graph rank and stopping rule, and differs
//! only in how each query's probe complements `r - left.sum` are computed
//! and how the public pair table is hashed. Three switchable levers, given
//! as the second argument (digits, default `1234567`; `0` = none):
//!
//! 1. batch inversion (Montgomery's trick): one field inversion per query
//!    instead of one per probe; every probe's point is the same value the
//!    baseline's `Arithmetic::add(r, -left.sum)` returns;
//! 2. Mersenne reduction for p = 2^31 - 1 in that path (fold twice instead
//!    of a generic `u128 %`), and branch subtraction instead of `% p`;
//! 3. a non-cryptographic multiply-rotate hasher for the pair table (the
//!    keys are public curve points; nothing adversarial is hashed);
//! 4. batch inversion for the pair-table build: the four sign combinations
//!    of a pair share the denominator `x_j - x_i`, so one prefix slot per
//!    pair and one inversion for the whole table; same sums, same order;
//! 5. streamlined probe loop (requires 1): the backward batch pass visits
//!    each complement directly (no output-point buffer), work buffers are
//!    reused across queries without zeroing, negation is a branch instead of
//!    `% p`, and the passes iterate instead of indexing;
//! 6. lazy Mersenne reduction in that loop (requires 2 and 5): one fold and
//!    one conditional subtraction per product, values kept in [0, p] with p
//!    standing for 0, and only the final coordinates made canonical;
//! 7. chained pair index: one map entry per distinct sum plus a `next`
//!    array over pair ids, instead of a `Vec` allocation per key;
//! 8. couple path (requires 1, 2, 4, 6): the four sign sums of a pair are
//!    (X1, +-Y1) and (X2, +-Y2), so the table needs two additions per pair;
//!    the opposite entries L, -L of a pair form a couple sharing x; probes
//!    r - L and r + L share the denominator x(L) - x(r) and compute only the
//!    complement's x, looked up in a flat x-keyed couple table behind a
//!    bitmap prefilter, with y computed on a hit to tell R from -R. Matches,
//!    relations and counts of group additions are those of the baseline.
//!
//! An optional third argument sets the thread count for lever 8 (default 1):
//! contiguous chunks of the table rows and of the couples, each with its own
//! batch inversion, merged deterministically (sets of relations, summed counts).
//!
//! Charged work counts what this path actually does: one group addition
//! per probe, one inversion per batch, and every field multiplication.

use prime31_algebraic_ic::{Arithmetic, Counts, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use serde_json::json;
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hasher};
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const FB: usize = 509;
const LP: usize = 168;
const TARGET: usize = FB + LP;
const RHS: usize = TARGET + 1;
const WIDTH: usize = RHS + 1;

#[derive(Clone, Copy)]
struct Pair {
    sum: Point,
    i: u16,
    j: u16,
    si: i8,
    sj: i8,
}

// Scalar-field arithmetic for the graph rank. Inputs are reduced (< n < 2^31),
// so products fit in u64 and `% n` by a constant compiles to multiply-shift;
// the values are those of the baseline's `u128` arithmetic.
fn mod_mul(x: u64, y: u64) -> u64 {
    debug_assert!(x < PRIME31.n && y < PRIME31.n);
    (x * y) % PRIME31.n
}

fn mod_sub(x: u64, y: u64) -> u64 {
    debug_assert!(x < PRIME31.n && y < PRIME31.n);
    if x >= y {
        x - y
    } else {
        x + PRIME31.n - y
    }
}

fn mod_inverse(mut x: u64) -> u64 {
    assert_ne!(x, 0);
    let mut result = 1;
    let mut exponent = PRIME31.n - 2;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = mod_mul(result, x);
        }
        x = mod_mul(x, x);
        exponent >>= 1;
    }
    result
}

struct GraphRank {
    lp_pivots: Vec<Option<Vec<u64>>>,
    fb_pivots: Vec<Option<Vec<u64>>>,
    lp_rank: usize,
    fb_rank: usize,
    projected_rows: usize,
}

impl GraphRank {
    fn new() -> Self {
        Self {
            lp_pivots: vec![None; LP],
            fb_pivots: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
            projected_rows: 0,
        }
    }

    fn eliminate(row: &mut [u64], pivot: &[u64], coefficient: u64) {
        for col in 0..WIDTH {
            if pivot[col] != 0 {
                row[col] = mod_sub(row[col], mod_mul(coefficient, pivot[col]));
            }
        }
    }

    fn normalize(row: &mut [u64], coefficient: u64) {
        let inverse = mod_inverse(coefficient);
        for value in row {
            if *value != 0 {
                *value = mod_mul(*value, inverse);
            }
        }
    }

    fn add(&mut self, mut row: Vec<u64>) {
        assert_eq!(row.len(), WIDTH);
        for at in 0..LP {
            let col = FB + at;
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            if let Some(pivot) = &self.lp_pivots[at] {
                Self::eliminate(&mut row, pivot, coefficient);
            } else {
                Self::normalize(&mut row, coefficient);
                self.lp_pivots[at] = Some(row);
                self.lp_rank += 1;
                return;
            }
        }
        self.projected_rows += 1;
        for col in 0..=TARGET {
            if (FB..TARGET).contains(&col) {
                assert_eq!(row[col], 0);
                continue;
            }
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            let at = if col == TARGET { FB } else { col };
            if let Some(pivot) = &self.fb_pivots[at] {
                Self::eliminate(&mut row, pivot, coefficient);
            } else {
                Self::normalize(&mut row, coefficient);
                self.fb_pivots[at] = Some(row);
                self.fb_rank += 1;
                return;
            }
        }
        assert_eq!(row[RHS], 0, "replayed relations cannot contradict");
    }

    fn target_scalar(&self) -> Option<u64> {
        self.fb_pivots[FB].as_ref().map(|row| row[RHS])
    }
}

/// Graph rank with delayed reduction (lever 8): pivots stored as u32 (< n),
/// each elimination adds `(n - c) * pivot` into u64 accumulators (three fit
/// before a reduction), coefficients are reduced when read. Same pivots,
/// ranks and recovered scalar as `GraphRank`, which reduces every operation.
/// Graph rank with delayed reduction (lever 8): pivots stored as u32 (< n),
/// each elimination adds `(n - c) * pivot` into u64 accumulators (three fit
/// before a reduction), coefficients are reduced when read. Same pivots,
/// ranks and recovered scalar as `GraphRank`, which reduces every operation.
struct LazyRank {
    lp_pivots: Vec<Option<Vec<u32>>>,
    fb_pivots: Vec<Option<Vec<u32>>>,
    lp_rank: usize,
    fb_rank: usize,
    projected_rows: usize,
}

impl LazyRank {
    fn new() -> Self {
        Self {
            lp_pivots: vec![None; LP],
            fb_pivots: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
            projected_rows: 0,
        }
    }

    #[inline(always)]
    fn axpy_body(acc: &mut [u64], m: u64, pivot: &[u32]) {
        for (a, &p) in acc.iter_mut().zip(pivot) {
            *a += m * p as u64;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn axpy_avx2(acc: &mut [u64], m: u64, pivot: &[u32]) {
        Self::axpy_body(acc, m, pivot)
    }

    /// acc[lo..hi] += m * pivot[lo..hi] (no reduction; caller bounds terms).
    #[inline]
    fn axpy(acc: &mut [u64], m: u64, pivot: &[u32], lo: usize, hi: usize) {
        let (acc, pivot) = (&mut acc[lo..hi], &pivot[lo..hi]);
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 support was just detected.
            return unsafe { Self::axpy_avx2(acc, m, pivot) };
        }
        Self::axpy_body(acc, m, pivot)
    }

    #[inline(always)]
    fn reduce_body(acc: &mut [u64]) {
        for v in acc {
            *v %= PRIME31.n;
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    unsafe fn reduce_avx2(acc: &mut [u64]) {
        Self::reduce_body(acc)
    }

    #[inline]
    fn reduce_range(acc: &mut [u64], lo: usize, hi: usize) {
        let acc = &mut acc[lo..hi];
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: AVX2 support was just detected.
            return unsafe { Self::reduce_avx2(acc) };
        }
        Self::reduce_body(acc)
    }

    fn reduce(acc: &mut [u64]) {
        let n = acc.len();
        Self::reduce_range(acc, 0, n);
    }

    fn pivot_row(acc: &mut [u64], coefficient: u64) -> Vec<u32> {
        Self::reduce(acc);
        let inverse = mod_inverse(coefficient);
        acc.iter().map(|&v| mod_mul(v, inverse) as u32).collect()
    }

    /// Exact delayed-reduction elimination. Stored pivots are canonical and
    /// vanish on known spans: the LP pivot at `at` on LP columns below `at`
    /// (earlier LP columns were cleared before it was stored), and an FB
    /// pivot at column `c` on [0, c) and on every LP column. Those spans are
    /// skipped in the axpy; residues are identical to the dense loop.
    fn add(&mut self, row: Vec<u64>) {
        assert_eq!(row.len(), WIDTH);
        let mut acc = row;
        let mut pending = 0;
        for at in 0..LP {
            let col = FB + at;
            acc[col] %= PRIME31.n;
            let coefficient = acc[col];
            if coefficient == 0 {
                continue;
            }
            if let Some(pivot) = &self.lp_pivots[at] {
                let m = PRIME31.n - coefficient;
                Self::axpy(&mut acc, m, pivot, 0, FB);
                Self::axpy(&mut acc, m, pivot, col, WIDTH);
                pending += 1;
                if pending == 3 {
                    Self::reduce_range(&mut acc, 0, FB);
                    Self::reduce_range(&mut acc, col, WIDTH);
                    pending = 0;
                }
            } else {
                self.lp_pivots[at] = Some(Self::pivot_row(&mut acc, coefficient));
                self.lp_rank += 1;
                return;
            }
        }
        self.projected_rows += 1;
        // LP columns are all cleared; FB pivots vanish there, so jump from
        // the FB block straight to TARGET.
        let cols = (0..FB).chain(std::iter::once(TARGET));
        for col in cols {
            acc[col] %= PRIME31.n;
            let coefficient = acc[col];
            if coefficient == 0 {
                continue;
            }
            let at = if col == TARGET { FB } else { col };
            if let Some(pivot) = &self.fb_pivots[at] {
                let m = PRIME31.n - coefficient;
                if col < FB {
                    Self::axpy(&mut acc, m, pivot, col, FB);
                }
                Self::axpy(&mut acc, m, pivot, TARGET, WIDTH);
                pending += 1;
                if pending == 3 {
                    if col < FB {
                        Self::reduce_range(&mut acc, col, FB);
                    }
                    Self::reduce_range(&mut acc, TARGET, WIDTH);
                    pending = 0;
                }
            } else {
                self.fb_pivots[at] = Some(Self::pivot_row(&mut acc, coefficient));
                self.fb_rank += 1;
                return;
            }
        }
        for col in FB..TARGET {
            assert_eq!(acc[col] % PRIME31.n, 0);
        }
        Self::reduce(&mut acc);
        assert_eq!(acc[RHS], 0, "replayed relations cannot contradict");
    }

    fn target_scalar(&self) -> Option<u64> {
        self.fb_pivots[FB].as_ref().map(|row| row[RHS] as u64)
    }
}

enum Rank {
    Plain(GraphRank),
    Lazy(LazyRank),
}

impl Rank {
    fn add(&mut self, row: Vec<u64>) {
        match self {
            Rank::Plain(r) => r.add(row),
            Rank::Lazy(r) => r.add(row),
        }
    }
    fn target_scalar(&self) -> Option<u64> {
        match self {
            Rank::Plain(r) => r.target_scalar(),
            Rank::Lazy(r) => r.target_scalar(),
        }
    }
    fn lp_rank(&self) -> usize {
        match self {
            Rank::Plain(r) => r.lp_rank,
            Rank::Lazy(r) => r.lp_rank,
        }
    }
    fn fb_rank(&self) -> usize {
        match self {
            Rank::Plain(r) => r.fb_rank,
            Rank::Lazy(r) => r.fb_rank,
        }
    }
    fn projected_rows(&self) -> usize {
        match self {
            Rank::Plain(r) => r.projected_rows,
            Rank::Lazy(r) => r.projected_rows,
        }
    }
}

fn next_word(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn dickson1024(mut x: u64) -> u64 {
    for _ in 0..10 {
        x = ((x as u128 * x as u128) % PRIME31.p as u128) as u64;
        x = (x + PRIME31.p - 2) % PRIME31.p;
    }
    x
}

#[derive(Clone, Copy, Debug)]
struct Levers {
    batch: bool,
    mersenne: bool,
    fast_hash: bool,
    batch_table: bool,
    streamline: bool,
    lazy: bool,
    chained: bool,
    couple: bool,
}

impl Levers {
    fn parse(text: &str) -> Self {
        assert!(
            text.chars().all(|c| "012345678".contains(c)),
            "levers are digits 0-8"
        );
        Self {
            batch: text.contains('1'),
            mersenne: text.contains('2'),
            fast_hash: text.contains('3'),
            batch_table: text.contains('4'),
            streamline: text.contains('5'),
            lazy: text.contains('6'),
            chained: text.contains('7'),
            couple: text.contains('8'),
        }
    }
    fn check(self) -> Self {
        assert!(!self.streamline || self.batch, "lever 5 needs lever 1");
        assert!(
            !self.lazy || (self.streamline && self.mersenne),
            "lever 6 needs levers 2 and 5"
        );
        assert!(
            !self.couple || (self.batch && self.mersenne && self.batch_table && self.lazy),
            "lever 8 needs levers 1, 2, 4 and 6"
        );
        self
    }
    fn label(&self) -> String {
        let mut out = String::new();
        for (on, d) in [
            (self.batch, '1'),
            (self.mersenne, '2'),
            (self.fast_hash, '3'),
            (self.batch_table, '4'),
            (self.streamline, '5'),
            (self.lazy, '6'),
            (self.chained, '7'),
            (self.couple, '8'),
        ] {
            if on {
                out.push(d);
            }
        }
        if out.is_empty() {
            out.push('0');
        }
        out
    }
}

/// Multiply-rotate hasher (FxHash constant) for public curve-point keys.
#[derive(Default, Clone, Copy)]
struct FastBuild;

struct FastHasher(u64);

impl FastHasher {
    #[inline]
    fn mix(&mut self, word: u64) {
        self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
}

impl Hasher for FastHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for chunk in bytes.chunks(8) {
            let mut word = [0u8; 8];
            word[..chunk.len()].copy_from_slice(chunk);
            self.mix(u64::from_le_bytes(word));
        }
    }
    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.mix(i as u64);
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.mix(i as u64);
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.mix(i);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.mix(i as u64);
    }
    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.mix(i as u64);
    }
}

impl BuildHasher for FastBuild {
    type Hasher = FastHasher;
    #[inline]
    fn build_hasher(&self) -> FastHasher {
        FastHasher(0)
    }
}

const P: u64 = PRIME31.p;

#[inline]
fn fmul(x: u64, y: u64, mersenne: bool) -> u64 {
    if mersenne {
        // x, y < p < 2^31, so x * y < 2^62; 2^31 = 1 (mod p).
        let t = x * y;
        let t = (t & P) + (t >> 31);
        let t = (t & P) + (t >> 31);
        if t >= P {
            t - P
        } else {
            t
        }
    } else {
        ((x as u128 * y as u128) % P as u128) as u64
    }
}

#[inline]
fn fsub(x: u64, y: u64, mersenne: bool) -> u64 {
    if mersenne {
        if x >= y {
            x - y
        } else {
            x + P - y
        }
    } else {
        (x + P - y) % P
    }
}

/// `x^(p-2)` by square-and-multiply, as `Arithmetic::inv` does.
fn finv(mut x: u64, mersenne: bool, mults: &mut u64) -> u64 {
    assert!(x > 0 && x < P);
    let mut out = 1;
    let mut e = P - 2;
    while e > 0 {
        if e & 1 != 0 {
            out = fmul(out, x, mersenne);
            *mults += 1;
        }
        x = fmul(x, x, mersenne);
        *mults += 1;
        e >>= 1;
    }
    out
}

/// `Arithmetic::add(r, -left.sum)` for every pair, with one inversion.
/// Special cases follow `Arithmetic::add` exactly: an identity left gives
/// `r`; `x(left) = x(r)` with cancelling ordinates gives the identity; equal
/// points use the tangent slope.
fn batch_complements(pairs: &[Pair], r: Point, mersenne: bool, counts: &mut Counts) -> Vec<Point> {
    #[derive(Clone, Copy)]
    enum Kind {
        General,
        LeftIdentity,
        Identity,
    }
    let (rx, ry) = r.expect("query point is affine");
    let n = pairs.len();
    let mut kind = Vec::with_capacity(n);
    let mut num = vec![0u64; n];
    let mut den = vec![0u64; n];
    let mut prefix = Vec::with_capacity(n);
    let mut acc = 1u64;
    let mut mults = 0u64;
    let mut general = 0usize;
    for (k, left) in pairs.iter().enumerate() {
        let Some((x, y)) = left.sum else {
            kind.push(Kind::LeftIdentity);
            prefix.push(acc);
            continue;
        };
        let ny = (P - y) % P;
        if rx == x && (ry + ny) % P == 0 {
            kind.push(Kind::Identity);
            prefix.push(acc);
            continue;
        }
        if rx == x {
            num[k] = (3 * fmul(rx, rx, mersenne) + PRIME31.a) % P;
            mults += 1;
            den[k] = 2 * ry % P;
        } else {
            num[k] = fsub(ny, ry, mersenne);
            den[k] = fsub(x, rx, mersenne);
        }
        assert!(den[k] > 0 && den[k] < P);
        acc = fmul(acc, den[k], mersenne);
        mults += 1;
        prefix.push(acc);
        kind.push(Kind::General);
        general += 1;
    }
    let mut out = vec![None; n];
    let mut inv = if general > 0 {
        counts.fp_inversions += 1;
        finv(acc, mersenne, &mut mults)
    } else {
        0
    };
    for k in (0..n).rev() {
        match kind[k] {
            Kind::LeftIdentity => out[k] = r,
            Kind::Identity => out[k] = None,
            Kind::General => {
                let before = if k == 0 { 1 } else { prefix[k - 1] };
                let inv_k = fmul(inv, before, mersenne);
                inv = fmul(inv, den[k], mersenne);
                let (x, _) = pairs[k].sum.expect("general case has an affine left");
                let slope = fmul(num[k], inv_k, mersenne);
                let xx = fsub(
                    fsub(fmul(slope, slope, mersenne), rx, mersenne),
                    x,
                    mersenne,
                );
                let yy = fsub(fmul(slope, fsub(rx, xx, mersenne), mersenne), ry, mersenne);
                mults += 5;
                out[k] = Some((xx, yy));
            }
        }
    }
    counts.group_additions += n as u64;
    counts.fp_multiplications += mults;
    out
}

/// Every `Arithmetic::add(signed[i][si], signed[j][sj])` of the table, in the
/// baseline's loop order (i < j, then si, then sj), with one inversion. For
/// `x_i != x_j` the four sign combinations share the slope denominator
/// `x_j - x_i`; a pair with `x_i == x_j` uses `Arithmetic::add` itself.
fn batch_table_sums(signed: &[[Point; 2]], mersenne: bool, ar: &mut Arithmetic) -> Vec<Point> {
    let n = signed.len();
    let pairs = n * (n - 1) / 2;
    let mut den = Vec::with_capacity(pairs);
    let mut prefix = Vec::with_capacity(pairs);
    let mut acc = 1u64;
    let mut mults = 0u64;
    for i in 0..n {
        for j in i + 1..n {
            let d = match (signed[i][0], signed[j][0]) {
                (Some((x, _)), Some((z, _))) if x != z => fsub(z, x, mersenne),
                _ => 0,
            };
            if d != 0 {
                acc = fmul(acc, d, mersenne);
                mults += 1;
            }
            den.push(d);
            prefix.push(acc);
        }
    }
    let mut inv = if acc != 1 || den.iter().any(|&d| d != 0) {
        ar.counts.fp_inversions += 1;
        finv(acc, mersenne, &mut mults)
    } else {
        0
    };
    let mut inv_den = vec![0u64; pairs];
    for k in (0..pairs).rev() {
        if den[k] == 0 {
            continue;
        }
        let before = if k == 0 { 1 } else { prefix[k - 1] };
        inv_den[k] = fmul(inv, before, mersenne);
        inv = fmul(inv, den[k], mersenne);
        mults += 2;
    }
    let mut out = Vec::with_capacity(4 * pairs);
    let mut k = 0;
    for i in 0..n {
        for j in i + 1..n {
            for si in 0..2 {
                for sj in 0..2 {
                    let (u, v) = (signed[i][si], signed[j][sj]);
                    if den[k] == 0 {
                        out.push(ar.add(PRIME31, u, v));
                        continue;
                    }
                    let ((x, y), (z, w)) = (u.unwrap(), v.unwrap());
                    let slope = fmul(fsub(w, y, mersenne), inv_den[k], mersenne);
                    let xx = fsub(fsub(fmul(slope, slope, mersenne), x, mersenne), z, mersenne);
                    let yy = fsub(fmul(slope, fsub(x, xx, mersenne), mersenne), y, mersenne);
                    mults += 3;
                    ar.counts.group_additions += 1;
                    out.push(Some((xx, yy)));
                }
            }
            k += 1;
        }
    }
    ar.counts.fp_multiplications += mults;
    out
}

const NONE: u32 = u32::MAX;

/// Pair-table index: the baseline's `Vec` per key, or a chain over pair ids.
enum PairIndex<S> {
    PerKey(HashMap<Point, Vec<usize>, S>),
    Chained {
        head: HashMap<Point, u32, S>,
        next: Vec<u32>,
    },
}

impl<S: BuildHasher + Default> PairIndex<S> {
    fn new(chained: bool, capacity: usize) -> Self {
        if chained {
            PairIndex::Chained {
                head: HashMap::with_capacity_and_hasher(capacity, S::default()),
                next: Vec::with_capacity(capacity),
            }
        } else {
            PairIndex::PerKey(HashMap::with_capacity_and_hasher(capacity, S::default()))
        }
    }
    /// Insert pair id `at`; ids arrive as 0, 1, 2, ... in table order.
    fn insert(&mut self, sum: Point, at: usize) {
        match self {
            PairIndex::PerKey(map) => map.entry(sum).or_default().push(at),
            PairIndex::Chained { head, next } => {
                debug_assert_eq!(next.len(), at);
                let at = u32::try_from(at).expect("pair ids fit in u32");
                match head.entry(sum) {
                    std::collections::hash_map::Entry::Occupied(mut e) => {
                        next.push(*e.get());
                        *e.get_mut() = at;
                    }
                    std::collections::hash_map::Entry::Vacant(e) => {
                        next.push(NONE);
                        e.insert(at);
                    }
                }
            }
        }
    }
    fn len(&self) -> usize {
        match self {
            PairIndex::PerKey(map) => map.len(),
            PairIndex::Chained { head, .. } => head.len(),
        }
    }
    /// Every pair id with this sum (order differs between layouts; callers
    /// collect into a set).
    #[inline]
    fn for_each(&self, key: &Point, mut f: impl FnMut(usize)) {
        match self {
            PairIndex::PerKey(map) => {
                if let Some(ids) = map.get(key) {
                    ids.iter().for_each(|&id| f(id));
                }
            }
            PairIndex::Chained { head, next } => {
                let mut cur = head.get(key).copied().unwrap_or(NONE);
                while cur != NONE {
                    f(cur as usize);
                    cur = next[cur as usize];
                }
            }
        }
    }
}

/// One probe's matches: the baseline's inner loop, unchanged.
#[inline]
fn record_matches<S: BuildHasher + Default>(
    left: &Pair,
    complement: &Point,
    index: &PairIndex<S>,
    pairs: &[Pair],
    unique: &mut HashSet<Vec<(u16, i8)>>,
) {
    index.for_each(complement, |right_id| {
        let right = pairs[right_id];
        if left.i == right.i || left.i == right.j || left.j == right.i || left.j == right.j {
            return;
        }
        let mut terms = vec![
            (left.i, left.si),
            (left.j, left.sj),
            (right.i, right.si),
            (right.j, right.sj),
        ];
        terms.sort_unstable_by_key(|term| term.0);
        unique.insert(terms);
    });
}

/// Lazy Mersenne product: inputs in [0, p], output in [0, p] (p means 0).
#[inline]
fn fmul_lazy(x: u64, y: u64) -> u64 {
    let t = x * y;
    let t = (t & P) + (t >> 31);
    if t >= P {
        t - P
    } else {
        t
    }
}

/// Lazy subtraction: inputs in [0, p], output in [0, p] (p means 0).
#[inline]
fn fsub_lazy(x: u64, y: u64) -> u64 {
    if x >= y {
        x - y
    } else {
        x + P - y
    }
}

#[inline]
fn canon(x: u64) -> u64 {
    if x == P {
        0
    } else {
        x
    }
}

/// Reused per-query work buffers for the streamlined probe loop.
#[derive(Default)]
struct Scratch {
    num: Vec<u64>,
    den: Vec<u64>,
    prefix: Vec<u64>,
}

/// Same complements as `batch_complements` (special cases encoded as
/// `den = 0`: `num = 1` left identity, `num = 0` identity), visited from the
/// backward pass instead of stored. Counts are charged exactly as there.
fn fused_probe(
    pairs: &[Pair],
    r: Point,
    mersenne: bool,
    lazy: bool,
    s: &mut Scratch,
    counts: &mut Counts,
    mut visit: impl FnMut(&Pair, Point),
) {
    let (rx, ry) = r.expect("query point is affine");
    let mul = |x: u64, y: u64| {
        if lazy {
            fmul_lazy(x, y)
        } else {
            fmul(x, y, mersenne)
        }
    };
    let sub = |x: u64, y: u64| {
        if lazy {
            fsub_lazy(x, y)
        } else {
            fsub(x, y, mersenne)
        }
    };
    s.num.clear();
    s.den.clear();
    s.prefix.clear();
    let mut acc = 1u64;
    let mut mults = 0u64;
    let mut general = false;
    for left in pairs {
        let Some((x, y)) = left.sum else {
            s.num.push(1);
            s.den.push(0);
            s.prefix.push(acc);
            continue;
        };
        let ny = if y == 0 { 0 } else { P - y };
        let (n, d) = if rx == x {
            if (ry + ny) % P == 0 {
                s.num.push(0);
                s.den.push(0);
                s.prefix.push(acc);
                continue;
            }
            mults += 1;
            ((3 * fmul(rx, rx, mersenne) + PRIME31.a) % P, 2 * ry % P)
        } else {
            (fsub(ny, ry, mersenne), fsub(x, rx, mersenne))
        };
        assert!(d > 0 && d < P);
        s.num.push(n);
        s.den.push(d);
        // Product of the general denominators before this one.
        s.prefix.push(acc);
        acc = mul(acc, d);
        mults += 1;
        general = true;
    }
    let mut inv = if general {
        counts.fp_inversions += 1;
        finv(canon(acc), mersenne, &mut mults)
    } else {
        0
    };
    let n = pairs.len();
    for (((left, &nm), &d), &b) in pairs.iter().zip(&s.num).zip(&s.den).zip(&s.prefix).rev() {
        if d == 0 {
            visit(left, if nm == 1 { r } else { None });
            continue;
        }
        let inv_k = mul(inv, b);
        inv = mul(inv, d);
        let (x, _) = left.sum.expect("general case has an affine left");
        let slope = mul(nm, inv_k);
        let xx = sub(sub(mul(slope, slope), rx), x);
        let yy = sub(mul(slope, sub(rx, xx)), ry);
        mults += 5;
        visit(left, Some((canon(xx), canon(yy))));
    }
    counts.group_additions += n as u64;
    counts.fp_multiplications += mults;
}

/// Exact affine addition with the baseline's case analysis and counts, for
/// the rare couples the batched paths do not cover.
fn exact_add(u: Point, v: Point, counts: &mut Counts) -> Point {
    let mut mults = 0u64;
    counts.group_additions += 1;
    let (Some((x, y)), Some((z, w))) = (u, v) else {
        return u.or(v);
    };
    if x == z && (y + w) % P == 0 {
        return None;
    }
    let (num, den) = if x == z {
        mults += 1;
        ((3 * fmul(x, x, true) + PRIME31.a) % P, 2 * y % P)
    } else {
        (fsub(w, y, true), fsub(z, x, true))
    };
    counts.fp_inversions += 1;
    let id = finv(den, true, &mut mults);
    let slope = fmul(num, id, true);
    let xx = fsub(fsub(fmul(slope, slope, true), x, true), z, true);
    let yy = fsub(fmul(slope, fsub(x, xx, true), true), y, true);
    counts.fp_multiplications += mults + 3;
    Some((xx, yy))
}

#[inline]
fn fneg(y: u64) -> u64 {
    if y == 0 {
        0
    } else {
        P - y
    }
}

/// Lever 8 table in compact form. Pair group `m` (the m-th `(i, j)`, i < j,
/// in the baseline's order) has entries `4m + k` with signs
/// (+,+), (+,-), (-,+), (-,-) for k = 0..3, as in the baseline's `pairs`.
/// Couple `q = 2m + c` holds entries `4m + c` (sum L, stored in `xy[q]`) and
/// `4m + 3 - c` (sum -L); both share x. `ij[m]` is the group's indices.
struct CoupleTable {
    xy: Vec<[u32; 2]>,
    ij: Vec<[u16; 2]>,
    /// Per build thread: its couple range and x-index bucket histogram.
    parts: Vec<(usize, usize, Vec<u32>)>,
    /// Couples with y = 0 (points of order two; normally none).
    zero_y: Vec<u32>,
}

const SIGNS: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

impl CoupleTable {
    fn entries(&self) -> usize {
        4 * self.ij.len()
    }
    /// The baseline's `Pair` for entry `e`.
    #[inline]
    fn pair(&self, e: usize) -> Pair {
        let (m, k) = (e / 4, e % 4);
        let q = 2 * m + if k == 0 || k == 3 { 0 } else { 1 };
        let [x, y] = self.xy[q];
        let y = if k <= 1 { y as u64 } else { fneg(y as u64) };
        let [i, j] = self.ij[m];
        Pair {
            sum: Some((x as u64, y)),
            i,
            j,
            si: SIGNS[k].0,
            sj: SIGNS[k].1,
        }
    }
    /// Two additions per pair sharing one batched inverse; rows split over
    /// threads. None if two fixture points share x (caller uses lever 4).
    fn build(signed: &[[Point; 2]], threads: usize, counts: &mut Counts) -> Option<Self> {
        let n = signed.len();
        let pts: Vec<(u64, u64)> = signed
            .iter()
            .map(|s| s[0].expect("affine fixture"))
            .collect();
        let mut xs: Vec<u64> = pts.iter().map(|p| p.0).collect();
        xs.sort_unstable();
        if xs.windows(2).any(|w| w[0] == w[1]) {
            return None;
        }
        let groups = n * (n - 1) / 2;
        let mut xy = vec![[0u32; 2]; 2 * groups];
        let mut ij = vec![[0u16; 2]; groups];
        let before = |i: usize| i * (2 * n - i - 1) / 2;
        let mut cuts = vec![0usize];
        for t in 1..threads {
            let goal = t * groups / threads;
            let mut i = *cuts.last().unwrap();
            while i < n && before(i) < goal {
                i += 1;
            }
            cuts.push(i);
        }
        cuts.push(n);
        let mut work = Vec::new();
        let (mut xy_rest, mut ij_rest): (&mut [[u32; 2]], &mut [[u16; 2]]) = (&mut xy, &mut ij);
        for w in cuts.windows(2) {
            let g = before(w[1]) - before(w[0]);
            let (xa, xb) = xy_rest.split_at_mut(2 * g);
            let (ia, ib) = ij_rest.split_at_mut(g);
            work.push((w[0], w[1], xa, ia));
            xy_rest = xb;
            ij_rest = ib;
        }
        let pts = &pts;
        let results: Vec<(Counts, usize, usize, Vec<u32>, Vec<u32>)> =
            std::thread::scope(|scope| {
                let handles: Vec<_> = work
                    .into_iter()
                    .map(|(i0, i1, xy, ij)| {
                        scope.spawn(move || {
                            let q0 = 2 * before(i0);
                            let q1 = q0 + xy.len();
                            let mut hist = vec![0u32; 1 << BBITS];
                            let mut zero_y = Vec::new();
                            let mut c = Counts::default();
                            let mut mults = 0u64;
                            let mut prefix: Vec<u32> = Vec::with_capacity(ij.len());
                            let mut acc = 1u64;
                            let mut g = 0;
                            for i in i0..i1 {
                                for j in i + 1..n {
                                    ij[g] = [i as u16, j as u16];
                                    g += 1;
                                    prefix.push(acc as u32);
                                    acc = fmul_lazy(acc, fsub(pts[j].0, pts[i].0, true));
                                    mults += 1;
                                }
                            }
                            if prefix.is_empty() {
                                return (c, q0, q1, hist, zero_y);
                            }
                            c.fp_inversions += 1;
                            let mut inv = finv(canon(acc), true, &mut mults);
                            for g in (0..ij.len()).rev() {
                                let [i, j] = ij[g];
                                let ((x, y), (z, w)) = (pts[i as usize], pts[j as usize]);
                                let d = fsub(z, x, true);
                                let inv_d = fmul_lazy(inv, prefix[g] as u64);
                                inv = fmul_lazy(inv, d);
                                // P_i + P_j and P_i - P_j share the denominator z - x.
                                let s1 = fmul_lazy(fsub_lazy(w, y), inv_d);
                                let x1 = canon(fsub_lazy(fsub_lazy(fmul_lazy(s1, s1), x), z));
                                let y1 = canon(fsub_lazy(fmul_lazy(s1, fsub_lazy(x, x1)), y));
                                let s2 = fmul_lazy(fsub_lazy(fneg(w), y), inv_d);
                                let x2 = canon(fsub_lazy(fsub_lazy(fmul_lazy(s2, s2), x), z));
                                let y2 = canon(fsub_lazy(fmul_lazy(s2, fsub_lazy(x, x2)), y));
                                mults += 8;
                                xy[2 * g] = [x1 as u32, y1 as u32];
                                xy[2 * g + 1] = [x2 as u32, y2 as u32];
                            }
                            c.group_additions += 4 * ij.len() as u64;
                            c.fp_multiplications += mults;
                            for (q, &[x, y]) in xy.iter().enumerate() {
                                hist[XIndex::bucket(XIndex::hash(x as u64))] += 1;
                                if y == 0 {
                                    zero_y.push((q0 + q) as u32);
                                }
                            }
                            (c, q0, q1, hist, zero_y)
                        })
                    })
                    .collect();
                handles.into_iter().map(|h| h.join().unwrap()).collect()
            });
        let mut parts = Vec::new();
        let mut zero_y = Vec::new();
        for (c, q0, q1, hist, z) in results {
            zero_y.extend(z);
            counts.group_additions += c.group_additions;
            counts.fp_inversions += c.fp_inversions;
            counts.fp_multiplications += c.fp_multiplications;
            parts.push((q0, q1, hist));
        }
        Some(CoupleTable {
            xy,
            ij,
            parts,
            zero_y,
        })
    }
}

const EMPTY: u32 = u32::MAX;

/// x-keyed index of couples in compressed-bucket form: entries pack
/// (x, couple) in one u64, grouped by the top `BBITS` hash bits with couples
/// ascending inside a bucket, so equal-x couples are found by scanning one
/// short bucket. A 1 MB blocked Bloom filter prefilters lookups. Threads own
/// disjoint bucket ranges (and the matching filter words), so the build has
/// no shared writes and the layout does not depend on the thread count.
struct XIndex {
    off: Vec<u32>,
    ent: Vec<u64>,
    fshift: u32,
    filter: Vec<u32>,
    distinct: usize,
}

const BBITS: u32 = 16;

impl XIndex {
    /// Blocked Bloom filter over 32-bit words (one `vpgatherdd` lane each):
    /// the word is chosen by the top hash bits, three bits inside it by hash
    /// bits 0..14.
    #[inline]
    fn bloom_mask(h: u32) -> u32 {
        (1u32 << (h & 31)) | (1u32 << ((h >> 5) & 31)) | (1u32 << ((h >> 9) & 31))
    }
    /// 32-bit multiplicative hash (vectorizable with `vpmulld`).
    #[inline]
    fn hash(x: u64) -> u32 {
        (x as u32).wrapping_mul(0x9e37_79b1)
    }
    #[inline]
    fn bucket(h: u32) -> usize {
        (h >> (32 - BBITS)) as usize
    }
    /// Build from the couple table's per-thread bucket histograms: global
    /// offsets, a stable scatter of each thread's own couple range, then
    /// filter bits and distinct counts over thread-owned bucket ranges.
    fn build(t: &CoupleTable, threads: usize) -> Self {
        struct SyncPtr(*mut u64);
        // SAFETY: threads write disjoint indices (see the scatter below).
        unsafe impl Sync for SyncPtr {}
        impl SyncPtr {
            fn get(&self) -> *mut u64 {
                self.0
            }
        }
        let xy = &t.xy;
        let nb = 1usize << BBITS;
        let fbits = 23u32;
        let fwords = (1usize << fbits) / 32;
        // Word index from the top (fbits - 6) hash bits; a bucket range maps
        // to a contiguous, disjoint word range.
        let fwbits = fbits - 5;
        let fshift = 32 - fwbits;
        let wpb = fwords / nb;
        let mut off = vec![0u32; nb + 1];
        let mut starts: Vec<Vec<u32>> = t.parts.iter().map(|_| vec![0u32; nb]).collect();
        let mut acc = 0u32;
        for b in 0..nb {
            off[b] = acc;
            for (k, (_, _, hist)) in t.parts.iter().enumerate() {
                starts[k][b] = acc;
                acc += hist[b];
            }
        }
        off[nb] = acc;
        assert_eq!(acc as usize, xy.len());
        let mut ent = vec![0u64; xy.len()];
        let ptr = SyncPtr(ent.as_mut_ptr());
        std::thread::scope(|scope| {
            for ((q0, q1, _), mut pos) in t.parts.iter().zip(starts) {
                let (q0, q1, ptr) = (*q0, *q1, &ptr);
                scope.spawn(move || {
                    for (q, &[x, _]) in xy.iter().enumerate().take(q1).skip(q0) {
                        let b = Self::bucket(Self::hash(x as u64));
                        // SAFETY: positions of (thread k, bucket b) are
                        // [starts[k][b], starts[k][b] + hist_k[b]), disjoint
                        // across (k, b) and inside `ent`.
                        unsafe { *ptr.get().add(pos[b] as usize) = ((x as u64) << 32) | q as u64 };
                        pos[b] += 1;
                    }
                });
            }
        });
        let mut filter = vec![0u32; fwords];
        let ranges: Vec<(usize, usize)> = (0..threads)
            .map(|k| (k * nb / threads, (k + 1) * nb / threads))
            .collect();
        let distinct: usize = std::thread::scope(|scope| {
            let (off, ent) = (&off, &ent);
            let mut frest: &mut [u32] = &mut filter;
            let mut handles = Vec::new();
            for &(lo, hi) in &ranges {
                let (f, ft) = frest.split_at_mut((hi - lo) * wpb);
                frest = ft;
                handles.push(scope.spawn(move || {
                    let fbase = lo * wpb;
                    let mut distinct = 0usize;
                    for b in lo..hi {
                        let bucket = &ent[off[b] as usize..off[b + 1] as usize];
                        for &v in bucket {
                            let h = Self::hash(v >> 32);
                            f[(h >> fshift) as usize - fbase] |= Self::bloom_mask(h);
                        }
                        distinct += Self::bucket_distinct(bucket, xy);
                    }
                    distinct
                }));
            }
            handles.into_iter().map(|h| h.join().unwrap()).sum()
        });
        let mut index = XIndex {
            off,
            ent,
            fshift,
            filter,
            distinct,
        };
        // A y = 0 couple alone on its x was counted as two points; it is one.
        for &q in &t.zero_y {
            let head = index.head(xy[q as usize][0] as u64);
            if index.chain(head).count() == 1 {
                index.distinct -= 1;
            }
        }
        index
    }
    /// Distinct points (x, +-y) among one bucket's couples, counting two
    /// per x held by a single couple (`build` corrects for y = 0 couples)
    /// and reading y only for x shared by several couples.
    fn bucket_distinct(bucket: &[u64], xy: &[[u32; 2]]) -> usize {
        let mut total = 0;
        for (k, &v) in bucket.iter().enumerate() {
            let x = v >> 32;
            let shared = bucket
                .iter()
                .enumerate()
                .any(|(l, &w)| l != k && w >> 32 == x);
            if !shared {
                total += 2;
                continue;
            }
            if bucket[..k].iter().any(|&w| w >> 32 == x) {
                continue;
            }
            let mut ys = Vec::new();
            for &w in &bucket[k..] {
                if w >> 32 == x {
                    let y = xy[w as u32 as usize][1] as u64;
                    ys.push(y);
                    ys.push(fneg(y));
                }
            }
            ys.sort_unstable();
            ys.dedup();
            total += ys.len();
        }
        total
    }
    /// Position of the first entry with this x, or EMPTY.
    #[inline]
    fn head(&self, x: u64) -> u32 {
        let h = Self::hash(x);
        let m = Self::bloom_mask(h);
        if self.filter[(h >> self.fshift) as usize] & m != m {
            return EMPTY;
        }
        self.head_unfiltered(x)
    }
    /// `head` without the filter test (for callers that already did it).
    #[inline]
    fn head_unfiltered(&self, x: u64) -> u32 {
        let b = Self::bucket(Self::hash(x));
        for k in self.off[b]..self.off[b + 1] {
            if self.ent[k as usize] >> 32 == x {
                return k;
            }
        }
        EMPTY
    }
    /// Couples with the x of entry `pos` (a `head` result), ascending.
    #[inline]
    fn chain(&self, pos: u32) -> impl Iterator<Item = u32> + '_ {
        let x = self.ent[pos as usize] >> 32;
        let end = self.off[Self::bucket(Self::hash(x)) + 1];
        self.ent[pos as usize..end as usize]
            .iter()
            .filter(move |&&w| w >> 32 == x)
            .map(|&w| w as u32)
    }
    /// Distinct table points (the baseline's index length).
    fn distinct_points(&self, _xy: &[[u32; 2]]) -> usize {
        self.distinct
    }
}

/// The baseline's match handling for probe entry `left` with complement
/// `(cx, cy)`, over the couples of the x-index chain starting at `head`.
#[inline]
fn record_x_matches(
    t: &CoupleTable,
    left_entry: usize,
    cy: u64,
    head: u32,
    idx: &XIndex,
    unique: &mut HashSet<Vec<(u16, i8)>>,
) {
    let left = t.pair(left_entry);
    for q in idx.chain(head) {
        let (m, c) = (q as usize / 2, q as usize % 2);
        let y = t.xy[q as usize][1] as u64;
        // The couple's entries 4m + c (y) and 4m + 3 - c (-y).
        for (entry, ey) in [(4 * m + c, y), (4 * m + 3 - c, fneg(y))] {
            if ey != cy {
                continue;
            }
            let right = t.pair(entry);
            if left.i == right.i || left.i == right.j || left.j == right.i || left.j == right.j {
                continue;
            }
            let mut terms = vec![
                (left.i, left.si),
                (left.j, left.sj),
                (right.i, right.si),
                (right.j, right.sj),
            ];
            terms.sort_unstable_by_key(|term| term.0);
            unique.insert(terms);
        }
    }
}

const LANES: usize = 4;

/// Lever 8 probes over couples `q0..q1` for target `r`: one batch slot per
/// couple in one of `LANES` interleaved chains (one inversion per chain),
/// x-only complements, y on a hit. Charges one group addition per probe.
fn couple_probe_chunk(
    t: &CoupleTable,
    idx: &XIndex,
    r: Point,
    q0: usize,
    q1: usize,
    prefix: &mut Vec<u32>,
    counts: &mut Counts,
    unique: &mut HashSet<Vec<(u16, i8)>>,
) {
    let (rx, ry) = r.expect("query point is affine");
    let xy = &t.xy[q0..q1];
    prefix.clear();
    let mut acc = [1u64; LANES];
    let mut mults = 0u64;
    for (k, &[u, _]) in xy.iter().enumerate() {
        let lane = k % LANES;
        prefix.push(acc[lane] as u32);
        let u = u as u64;
        let d = if u == rx { 1 } else { fsub(u, rx, true) };
        acc[lane] = fmul_lazy(acc[lane], d);
    }
    mults += xy.len() as u64;
    let mut inv = [0u64; LANES];
    for lane in 0..LANES.min(xy.len()) {
        counts.fp_inversions += 1;
        inv[lane] = finv(canon(acc[lane]), true, &mut mults);
    }
    for k in (0..xy.len()).rev() {
        let lane = k % LANES;
        let q = q0 + k;
        let (m, c) = (q / 2, q % 2);
        let (e0, e1) = (4 * m + c, 4 * m + 3 - c);
        let [u, yl] = xy[k];
        let (u, yl) = (u as u64, yl as u64);
        if u == rx {
            // Rare: x(L) = x(r); exact general additions for both probes.
            for (left_entry, ly) in [(e0, yl), (e1, fneg(yl))] {
                if let Some((cx, cy)) = exact_add(r, Some((u, fneg(ly))), counts) {
                    let head = idx.head(cx);
                    if head != EMPTY {
                        record_x_matches(t, left_entry, cy, head, idx, unique);
                    }
                }
            }
            continue;
        }
        let d = fsub(u, rx, true);
        let inv_q = fmul_lazy(inv[lane], prefix[k] as u64);
        inv[lane] = fmul_lazy(inv[lane], d);
        // e0 (sum L): complement r - L = r + (u, -yl); e1 (sum -L): r + L.
        let s0 = fmul_lazy(fsub_lazy(fneg(yl), ry), inv_q);
        let x0 = canon(fsub_lazy(fsub_lazy(fmul_lazy(s0, s0), rx), u));
        let s1 = fmul_lazy(fsub_lazy(yl, ry), inv_q);
        let x1 = canon(fsub_lazy(fsub_lazy(fmul_lazy(s1, s1), rx), u));
        mults += 6;
        let h0 = idx.head(x0);
        if h0 != EMPTY {
            let y0 = canon(fsub_lazy(fmul_lazy(s0, fsub_lazy(rx, x0)), ry));
            mults += 1;
            record_x_matches(t, e0, y0, h0, idx, unique);
        }
        let h1 = idx.head(x1);
        if h1 != EMPTY {
            let y1 = canon(fsub_lazy(fmul_lazy(s1, fsub_lazy(rx, x1)), ry));
            mults += 1;
            record_x_matches(t, e1, y1, h1, idx, unique);
        }
    }
    counts.group_additions += 2 * xy.len() as u64;
    counts.fp_multiplications += mults;
}

/// AVX2 kernel for lever 8 probes: eight interleaved batch chains in one
/// 8 x u32 vector. Two 32-byte loads of four `[x, y]` couples each are
/// deinterleaved into X = [x0, x4, x1, x5, x2, x6, x3, x7] and Y alike, so
/// 32-bit lane j holds couple `(j & 1) * 4 + j / 2` of the block. Products
/// use `vpmuludq` on the even and the odd lanes, one Mersenne fold each and
/// a shared conditional subtraction; every other operation covers all eight
/// lanes at once. Values are canonical throughout, so results equal the
/// scalar `fmul_lazy` / `fsub_lazy` / `canon` values; filter hits, y on a
/// hit, the rare x(L) = x(r) couples and match recording stay scalar.
#[cfg(target_arch = "x86_64")]
mod avx2 {
    use super::*;
    use std::arch::x86_64::*;

    /// Lane j of the block's X/Y vectors holds couple `LANE_COUPLE[j]`.
    const LANE_COUPLE: [usize; 8] = [0, 4, 1, 5, 2, 6, 3, 7];

    /// a * b mod p for 8 x u32 lanes in [0, p): even lanes from `vpmuludq`
    /// of the low halves, odd lanes after a 32-bit shift; each product
    /// (< 2^62) folds to t <= 2p - 1, and min(t, t - p) on u32 is the
    /// conditional subtraction (t - p wraps above t when t < p).
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn mul8(a: __m256i, b: __m256i, p64: __m256i, p32: __m256i) -> __m256i {
        let e = _mm256_mul_epu32(a, b);
        let o = _mm256_mul_epu32(_mm256_srli_epi64(a, 32), _mm256_srli_epi64(b, 32));
        let e = _mm256_add_epi64(_mm256_and_si256(e, p64), _mm256_srli_epi64(e, 31));
        let o = _mm256_add_epi64(_mm256_and_si256(o, p64), _mm256_srli_epi64(o, 31));
        let t = _mm256_blend_epi32::<0b1010_1010>(e, _mm256_slli_epi64(o, 32));
        _mm256_min_epu32(t, _mm256_sub_epi32(t, p32))
    }

    /// x - y mod p, x and y in [0, p): x - y wraps above x - y + p exactly
    /// when y > x.
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn sub8(x: __m256i, y: __m256i, p32: __m256i) -> __m256i {
        let d = _mm256_sub_epi32(x, y);
        _mm256_min_epu32(d, _mm256_add_epi32(d, p32))
    }

    /// x + y mod p, x and y in [0, p).
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn add8(x: __m256i, y: __m256i, p32: __m256i) -> __m256i {
        let s = _mm256_add_epi32(x, y);
        _mm256_min_epu32(s, _mm256_sub_epi32(s, p32))
    }

    /// Bloom prefilter for eight x values: bit j set if lane j may be in the
    /// index (exactly `XIndex::head`'s filter test).
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn filter8(idx: &XIndex, x: __m256i) -> u32 {
        let h = _mm256_mullo_epi32(x, _mm256_set1_epi32(0x9e37_79b1_u32 as i32));
        let w = _mm256_srl_epi32(h, _mm_cvtsi32_si128(idx.fshift as i32));
        let words = _mm256_i32gather_epi32::<4>(idx.filter.as_ptr() as *const i32, w);
        let m31 = _mm256_set1_epi32(31);
        let one = _mm256_set1_epi32(1);
        let b0 = _mm256_and_si256(h, m31);
        let b1 = _mm256_and_si256(_mm256_srli_epi32(h, 5), m31);
        let b2 = _mm256_and_si256(_mm256_srli_epi32(h, 9), m31);
        let mask = _mm256_or_si256(
            _mm256_or_si256(_mm256_sllv_epi32(one, b0), _mm256_sllv_epi32(one, b1)),
            _mm256_sllv_epi32(one, b2),
        );
        let all = _mm256_cmpeq_epi32(_mm256_and_si256(words, mask), mask);
        _mm256_movemask_ps(_mm256_castsi256_ps(all)) as u32
    }

    /// X and Y of block `s` (couples 8s .. 8s + 8 of `src`).
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn load_xy(src: *const __m256i, s: usize) -> (__m256i, __m256i) {
        let v0 = _mm256_loadu_si256(src.add(2 * s));
        let v1 = _mm256_loadu_si256(src.add(2 * s + 1));
        let x = _mm256_blend_epi32::<0b1010_1010>(v0, _mm256_slli_epi64(v1, 32));
        let y = _mm256_blend_epi32::<0b1010_1010>(_mm256_srli_epi64(v0, 32), v1);
        (x, y)
    }

    /// Couples `q0 .. q0 + 8 * blocks`; same matches and counts as the scalar
    /// kernel, with one inversion per chain (eight).
    #[target_feature(enable = "avx2")]
    #[allow(clippy::too_many_arguments)]
    pub(super) unsafe fn probe_blocks(
        t: &CoupleTable,
        idx: &XIndex,
        r: Point,
        q0: usize,
        blocks: usize,
        prefix: &mut Vec<u32>,
        counts: &mut Counts,
        unique: &mut HashSet<Vec<(u16, i8)>>,
    ) {
        let (rx, ry) = r.expect("query point is affine");
        let len = 8 * blocks;
        let xy = &t.xy[q0..q0 + len];
        // Every slot is written by the forward pass before the backward pass
        // reads it; no zero fill.
        prefix.clear();
        prefix.reserve(len);
        // SAFETY: u32 has no invalid bit patterns or drop glue, capacity is
        // at least `len`, and slots are written before being read.
        unsafe { prefix.set_len(len) };
        let p64 = _mm256_set1_epi64x(P as i64);
        let p32 = _mm256_set1_epi32(P as i32);
        let rxv = _mm256_set1_epi32(rx as i32);
        let ryv = _mm256_set1_epi32(ry as i32);
        let nryv = _mm256_set1_epi32(fneg(ry) as i32);
        let one = _mm256_set1_epi32(1);
        let src = xy.as_ptr() as *const __m256i;
        let pre = prefix.as_mut_ptr() as *mut __m256i;
        // x - rx, with 1 in place of 0 (x = rx lanes are handled exactly in
        // the scalar special path).
        let den = |x: __m256i| {
            let sp = _mm256_cmpeq_epi32(x, rxv);
            _mm256_or_si256(sub8(x, rxv, p32), _mm256_and_si256(sp, one))
        };
        let mut acc = one;
        for s in 0..blocks {
            let (x, _) = load_xy(src, s);
            _mm256_storeu_si256(pre.add(s), acc);
            acc = mul8(acc, den(x), p64, p32);
        }
        let mut mults = len as u64;
        let mut lanes = [0u32; 8];
        _mm256_storeu_si256(lanes.as_mut_ptr() as *mut __m256i, acc);
        for v in &mut lanes {
            counts.fp_inversions += 1;
            *v = finv(canon(*v as u64), true, &mut mults) as u32;
        }
        let mut inv = _mm256_loadu_si256(lanes.as_ptr() as *const __m256i);
        let (mut xs0, mut xs1, mut ss0, mut ss1) = ([0u32; 8], [0u32; 8], [0u32; 8], [0u32; 8]);
        for s in (0..blocks).rev() {
            let (x, y) = load_xy(src, s);
            let inv_q = mul8(inv, _mm256_loadu_si256(pre.add(s)), p64, p32);
            inv = mul8(inv, den(x), p64, p32);
            // All operands canonical, so every result is in [0, p).
            let w = add8(rxv, x, p32);
            let s0 = mul8(sub8(nryv, y, p32), inv_q, p64, p32);
            let x0 = sub8(mul8(s0, s0, p64, p32), w, p32);
            let s1 = mul8(sub8(y, ryv, p32), inv_q, p64, p32);
            let x1 = sub8(mul8(s1, s1, p64, p32), w, p32);
            let hit0 = filter8(idx, x0);
            let hit1 = filter8(idx, x1);
            let sp = _mm256_cmpeq_epi32(x, rxv);
            let special = _mm256_movemask_ps(_mm256_castsi256_ps(sp)) as u32;
            if (hit0 | hit1 | special) == 0 {
                continue;
            }
            _mm256_storeu_si256(xs0.as_mut_ptr() as *mut __m256i, x0);
            _mm256_storeu_si256(xs1.as_mut_ptr() as *mut __m256i, x1);
            _mm256_storeu_si256(ss0.as_mut_ptr() as *mut __m256i, s0);
            _mm256_storeu_si256(ss1.as_mut_ptr() as *mut __m256i, s1);
            let entries = |j: usize| {
                let q = q0 + 8 * s + LANE_COUPLE[j];
                let (m, c) = (q / 2, q % 2);
                (4 * m + c, 4 * m + 3 - c)
            };
            let mut bits = special;
            while bits != 0 {
                let j = bits.trailing_zeros() as usize;
                bits &= bits - 1;
                // Rare: x(L) = x(r); exact general additions for both probes.
                let (e0, e1) = entries(j);
                let [u, yl] = xy[8 * s + LANE_COUPLE[j]];
                let (u, yl) = (u as u64, yl as u64);
                for (left_entry, ly) in [(e0, yl), (e1, fneg(yl))] {
                    if let Some((cx, cy)) = exact_add(r, Some((u, fneg(ly))), counts) {
                        let head = idx.head(cx);
                        if head != EMPTY {
                            record_x_matches(t, left_entry, cy, head, idx, unique);
                        }
                    }
                }
            }
            for (hits, xs, ss, second) in [(hit0, &xs0, &ss0, false), (hit1, &xs1, &ss1, true)] {
                let mut bits = hits & !special;
                while bits != 0 {
                    let j = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    let xj = xs[j] as u64;
                    let head = idx.head_unfiltered(xj);
                    if head != EMPTY {
                        let yj = canon(fsub_lazy(fmul_lazy(ss[j] as u64, fsub_lazy(rx, xj)), ry));
                        mults += 1;
                        let (e0, e1) = entries(j);
                        record_x_matches(t, if second { e1 } else { e0 }, yj, head, idx, unique);
                    }
                }
            }
        }
        // Per couple: forward 1 (above) and backward 2 + 4 multiplications;
        // couples with x(L) = x(r) are charged inside `exact_add` as well.
        mults += 6 * len as u64;
        counts.group_additions += 2 * len as u64;
        counts.fp_multiplications += mults;
    }
}

/// Lever 8 probes for couples `q0..q1`: AVX2 kernel for whole blocks of 8
/// when the CPU has AVX2, the scalar kernel for the rest.
fn couple_probe_range(
    t: &CoupleTable,
    idx: &XIndex,
    r: Point,
    q0: usize,
    q1: usize,
    buf32: &mut Vec<u32>,
    counts: &mut Counts,
    unique: &mut HashSet<Vec<(u16, i8)>>,
) {
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("avx2") {
        let blocks = (q1 - q0) / 8;
        if blocks > 0 {
            // SAFETY: AVX2 is available (checked above).
            unsafe { avx2::probe_blocks(t, idx, r, q0, blocks, buf32, counts, unique) };
        }
        let rest = q0 + 8 * blocks;
        if rest < q1 {
            couple_probe_chunk(t, idx, r, rest, q1, buf32, counts, unique);
        }
        return;
    }
    couple_probe_chunk(t, idx, r, q0, q1, buf32, counts, unique);
}

/// Couples per work chunk: fixed boundaries, so the batch-inversion chains
/// (and the charged counts) do not depend on the thread count, and a
/// chunk's prefix buffer (32 KB) stays in L1.
const CHUNK: usize = 4096;

/// Lever 8 probes for one query over all couples. Threads pull fixed chunks
/// from a shared counter (dynamic balance); worker 0 first runs `prelude`
/// (the previous query's graph stage when pipelined). Relation sets merged,
/// counts summed.
fn couple_probe_with<R: Send>(
    t: &CoupleTable,
    idx: &XIndex,
    r: Point,
    buffers: &mut [Vec<u32>],
    counts: &mut Counts,
    prelude: Option<impl FnOnce() -> R + Send>,
) -> (HashSet<Vec<(u16, i8)>>, Option<R>) {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let couples = t.xy.len();
    let chunks = couples.div_ceil(CHUNK);
    let next = AtomicUsize::new(0);
    let mut prelude = prelude;
    let (results, out) = std::thread::scope(|scope| {
        let next = &next;
        let work = move |buf32: &mut Vec<u32>| {
            let mut c = Counts::default();
            let mut u = HashSet::new();
            loop {
                let k = next.fetch_add(1, Ordering::Relaxed);
                if k >= chunks {
                    break;
                }
                let (q0, q1) = (k * CHUNK, ((k + 1) * CHUNK).min(couples));
                couple_probe_range(t, idx, r, q0, q1, buf32, &mut c, &mut u);
            }
            (c, u)
        };
        let (first, rest) = buffers.split_first_mut().expect("at least one thread");
        let handles: Vec<_> = rest
            .iter_mut()
            .map(|buf32| scope.spawn(move || work(buf32)))
            .collect();
        let out = prelude.take().map(|f| f());
        let mut results = vec![work(first)];
        results.extend(handles.into_iter().map(|h| h.join().unwrap()));
        (results, out)
    });
    let mut unique = HashSet::new();
    for (c, u) in results {
        add_counts(counts, &c);
        unique.extend(u);
    }
    (unique, out)
}

fn couple_probe(
    t: &CoupleTable,
    idx: &XIndex,
    r: Point,
    buffers: &mut [Vec<u32>],
    counts: &mut Counts,
) -> HashSet<Vec<(u16, i8)>> {
    couple_probe_with(t, idx, r, buffers, counts, None::<fn()>).0
}

fn main() {
    let max_queries = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(40);
    let levers =
        Levers::parse(&std::env::args().nth(2).unwrap_or_else(|| "1234567".into())).check();
    let threads = std::env::args()
        .nth(3)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    assert!((1..=64).contains(&threads));
    if levers.fast_hash {
        run::<FastBuild>(max_queries, levers, threads)
    } else {
        run::<RandomState>(max_queries, levers, threads)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(ar: &mut Arithmetic, scalar: u64) -> Point {
        ar.scalar(PRIME31, scalar, Some(PRIME31.g))
    }

    fn pairs(points: &[Point]) -> Vec<Pair> {
        points
            .iter()
            .enumerate()
            .map(|(i, &sum)| Pair {
                sum,
                i: i as u16,
                j: 0,
                si: 1,
                sj: 1,
            })
            .collect()
    }

    #[test]
    fn batched_and_fused_complements_match_group_addition() {
        let mut ar = Arithmetic::default();
        let g = point(&mut ar, 1);
        let neg_g = ar.neg(PRIME31, g);
        let points = [None, g, neg_g, point(&mut ar, 2), point(&mut ar, 7)];
        let cases = [
            vec![],
            pairs(&points[..3]),
            pairs(&points),
            pairs(&[point(&mut ar, 13), point(&mut ar, 19)]),
        ];
        for (case, left) in cases.iter().enumerate() {
            for (query, r) in [g, point(&mut ar, 11)].into_iter().enumerate() {
                let expected: Vec<_> = left
                    .iter()
                    .map(|entry| {
                        let neg = ar.neg(PRIME31, entry.sum);
                        ar.add(PRIME31, r, neg)
                    })
                    .collect();
                for (mersenne, lazy) in [(false, false), (true, false), (true, true)] {
                    let mut counts = Counts::default();
                    let batched = batch_complements(left, r, mersenne, &mut counts);
                    assert_eq!(
                        batched, expected,
                        "case={case}, query={query}, mersenne={mersenne}"
                    );
                    assert_eq!(counts.group_additions, left.len() as u64);

                    let mut actual = vec![None; left.len()];
                    let mut scratch = Scratch::default();
                    let mut counts = Counts::default();
                    fused_probe(
                        left,
                        r,
                        mersenne,
                        lazy,
                        &mut scratch,
                        &mut counts,
                        |entry, result| {
                            actual[entry.i as usize] = result;
                        },
                    );
                    assert_eq!(
                        actual, expected,
                        "case={case}, query={query}, mersenne={mersenne}, lazy={lazy}"
                    );
                    assert_eq!(counts.group_additions, left.len() as u64);
                }
            }
        }
    }

    /// The 8-lane AVX2 kernel finds the same relations and charges the
    /// same counts as the scalar couple kernel, including x(L) = x(r)
    /// couples (r = P_1 + P_2 is itself a couple sum).
    #[cfg(target_arch = "x86_64")]
    #[test]
    fn avx2_couple_kernel_matches_scalar_kernel() {
        if !std::is_x86_feature_detected!("avx2") {
            return;
        }
        let mut ar = Arithmetic::default();
        let signed: Vec<[Point; 2]> = (1..=40)
            .map(|k| {
                let p = point(&mut ar, k);
                [p, ar.neg(PRIME31, p)]
            })
            .collect();
        let mut build = Counts::default();
        let t = CoupleTable::build(&signed, 1, &mut build).expect("distinct x");
        let idx = XIndex::build(&t, 1);
        let couples = t.xy.len();
        assert_eq!(couples % 8, 0);
        for r in [point(&mut ar, 3), point(&mut ar, 100), point(&mut ar, 77)] {
            let (mut c_scalar, mut u_scalar) = (Counts::default(), HashSet::new());
            let mut buf = Vec::new();
            couple_probe_chunk(
                &t,
                &idx,
                r,
                0,
                couples,
                &mut buf,
                &mut c_scalar,
                &mut u_scalar,
            );
            let (mut c_avx, mut u_avx) = (Counts::default(), HashSet::new());
            // SAFETY: AVX2 support was detected above.
            unsafe {
                avx2::probe_blocks(
                    &t,
                    &idx,
                    r,
                    0,
                    couples / 8,
                    &mut buf,
                    &mut c_avx,
                    &mut u_avx,
                )
            };
            assert!(!u_scalar.is_empty(), "the query must produce relations");
            assert_eq!(u_avx, u_scalar);
            assert_eq!(c_avx.group_additions, c_scalar.group_additions);
            // Eight chains (one inversion each) against the scalar kernel's
            // `LANES`; the special-path inversions are the same.
            assert_eq!(
                c_avx.fp_inversions - 8,
                c_scalar.fp_inversions - LANES as u64
            );
        }
    }

    #[test]
    fn batched_pair_table_matches_group_addition() {
        let mut ar = Arithmetic::default();
        let g = point(&mut ar, 1);
        let points = [
            None,
            g,
            g,
            ar.neg(PRIME31, g),
            point(&mut ar, 2),
            point(&mut ar, 7),
        ];
        let signed: Vec<_> = points.iter().map(|&p| [p, ar.neg(PRIME31, p)]).collect();
        let mut expected = Vec::new();
        for i in 0..signed.len() {
            for j in i + 1..signed.len() {
                for si in 0..2 {
                    for sj in 0..2 {
                        expected.push(ar.add(PRIME31, signed[i][si], signed[j][sj]));
                    }
                }
            }
        }
        for mersenne in [false, true] {
            let mut batched_ar = Arithmetic::default();
            let actual = batch_table_sums(&signed, mersenne, &mut batched_ar);
            assert_eq!(actual, expected, "mersenne={mersenne}");
            assert_eq!(batched_ar.counts.group_additions, expected.len() as u64);
        }
    }
}

/// Peak resident set (VmHWM, kB) of this process so far; None off Linux.
fn vm_hwm_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    line.split_whitespace().nth(1)?.parse().ok()
}

fn add_counts(dst: &mut Counts, src: &Counts) {
    dst.group_additions += src.group_additions;
    dst.fp_multiplications += src.fp_multiplications;
    dst.fp_inversions += src.fp_inversions;
    dst.scalar_multiplications += src.scalar_multiplications;
    dst.scalar_inversions += src.scalar_inversions;
    dst.polynomial_evaluations += src.polynomial_evaluations;
    dst.extraction_nodes += src.extraction_nodes;
}

/// Microsecond stage ledger; `lap` charges the time since the last lap.
struct Ledger {
    last: Instant,
    us: std::collections::BTreeMap<&'static str, u128>,
}

impl Ledger {
    fn new(start: Instant) -> Self {
        Self {
            last: start,
            us: Default::default(),
        }
    }
    fn lap(&mut self, stage: &'static str) {
        let now = Instant::now();
        *self.us.entry(stage).or_default() += now.duration_since(self.last).as_micros();
        self.last = now;
    }
}

fn run<S: BuildHasher + Default>(max_queries: usize, levers: Levers, threads: usize) {
    let cold = Instant::now();
    let mut ledger = Ledger::new(cold);
    let fixture = Hybrid677::build().expect("frozen public B677 support");
    let fixture_ms = cold.elapsed().as_millis();
    ledger.lap("a_fixture");
    let points = fixture.points();
    let base: Vec<_> = points
        .iter()
        .map(|(x, _)| dickson1024(*x) == fixture.terminal())
        .collect();
    assert_eq!(base.iter().filter(|&&flag| flag).count(), 509);
    assert_eq!(points.len(), 677);
    let mut ar = Arithmetic::default();
    assert!(ar.on_curve(PRIME31, Q));
    let signed: Vec<[Point; 2]> = points
        .iter()
        .map(|&point| [Some(point), ar.neg(PRIME31, Some(point))])
        .collect();
    ledger.lap("b_setup_base_signed");
    let table_start = Instant::now();
    let ctab = if levers.couple {
        CoupleTable::build(&signed, threads, &mut ar.counts)
    } else {
        None
    };
    let couple_ok = ctab.is_some();
    let mut index: PairIndex<S> = PairIndex::new(levers.chained, 0);
    let pairs: Vec<Pair> = match couple_ok {
        true => Vec::new(),
        false => {
            let mut pairs = Vec::with_capacity(4 * points.len() * (points.len() - 1) / 2);
            index = PairIndex::new(levers.chained, pairs.capacity());
            let table_sums = if levers.batch_table {
                Some(batch_table_sums(&signed, levers.mersenne, &mut ar))
            } else {
                None
            };
            for i in 0..points.len() {
                for j in i + 1..points.len() {
                    for si in 0..2 {
                        for sj in 0..2 {
                            let at = pairs.len();
                            let sum = match &table_sums {
                                Some(all) => all[at],
                                None => ar.add(PRIME31, signed[i][si], signed[j][sj]),
                            };
                            pairs.push(Pair {
                                sum,
                                i: i as u16,
                                j: j as u16,
                                si: if si == 0 { 1 } else { -1 },
                                sj: if sj == 0 { 1 } else { -1 },
                            });
                            index.insert(sum, at);
                        }
                    }
                }
            }
            pairs
        }
    };
    ledger.lap("c1_table_pairs");
    let xidx = ctab.as_ref().map(|t| XIndex::build(t, threads));
    let pair_entries = ctab.as_ref().map_or(pairs.len(), CoupleTable::entries);
    let mut buffers: Vec<Vec<u32>> = vec![Vec::new(); threads];
    ledger.lap("c2_table_xindex");
    let table_ms = table_start.elapsed().as_millis();
    ledger.lap("c_table");
    let mut columns = vec![usize::MAX; points.len()];
    let mut fb_next = 0;
    let mut lp_next = FB;
    for (i, &is_base) in base.iter().enumerate() {
        if is_base {
            columns[i] = fb_next;
            fb_next += 1;
        } else {
            columns[i] = lp_next;
            lp_next += 1;
        }
    }
    assert_eq!((fb_next, lp_next), (FB, TARGET));
    assert!((1..=128).contains(&max_queries));
    let mut seed = 0x7033_315f_6d34_0101u64;
    let mut rank = if couple_ok {
        Rank::Lazy(LazyRank::new())
    } else {
        Rank::Plain(GraphRank::new())
    };
    let mut query_records = Vec::new();
    let mut all_rows = Vec::new();
    let mut total_probe_additions = 0u64;
    let mut all_classes = [0usize; 5];
    let mut recovered = None;
    let mut scratch = Scratch::default();
    ledger.lap("d_columns_setup");
    // Lever 8 with two or more threads: query k's sort, replay and rank run
    // on their own thread while query k + 1 is collected. Query k + 1 is
    // speculative: if query k recovers the scalar, its setup and collection
    // (and their counts) are discarded, so every output equals the
    // sequential loop's.
    let pipelined = couple_ok && threads >= 2;
    if pipelined {
        let t = ctab.as_ref().expect("couple table");
        let xidx = xidx.as_ref().expect("x-index");
        let (signed, base, columns) = (&signed, &base, &columns);
        let graph = |unique: HashSet<Vec<(u16, i8)>>,
                     label: usize,
                     a: u64,
                     b: u64,
                     r: Point,
                     rank: &mut Rank| {
            let mut gc = Counts::default();
            let mut relations: Vec<_> = unique.into_iter().collect();
            relations.sort();
            let graph_start = Instant::now();
            let mut classes = [0usize; 5];
            let mut rows = Vec::with_capacity(relations.len());
            for terms in &relations {
                let mut sum = None;
                let mut base_terms = 0;
                let mut row = vec![0u64; WIDTH];
                for &(i, sign) in terms {
                    let i = i as usize;
                    base_terms += usize::from(base[i]);
                    let point = if sign == 1 {
                        signed[i][0]
                    } else {
                        signed[i][1]
                    };
                    sum = exact_add(sum, point, &mut gc);
                    row[columns[i]] = if sign == 1 { 1 } else { PRIME31.n - 1 };
                }
                assert_eq!(sum, r, "every emitted equation must replay");
                classes[base_terms] += 1;
                rows.push(json!({"label": label, "terms": terms}));
                if base_terms >= 2 {
                    row[TARGET] = mod_sub(0, b);
                    row[RHS] = a;
                    rank.add(row);
                }
            }
            (
                relations.len(),
                classes,
                rows,
                graph_start.elapsed().as_millis(),
                gc,
            )
        };
        let graph = &graph;
        type Pending = (usize, u64, u64, Point, HashSet<Vec<(u16, i8)>>, u128);
        let mut pending: Option<Pending> = None;
        let mut label = 0usize;
        loop {
            let next = (label < max_queries).then(|| {
                let saved = std::mem::take(&mut ar.counts);
                let a = next_word(&mut seed) % PRIME31.n;
                let b = next_word(&mut seed) % PRIME31.n;
                let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
                let bq = ar.scalar(PRIME31, b, Q);
                let r = ar.add(PRIME31, ag, bq);
                assert!(r.is_some());
                let setup = std::mem::replace(&mut ar.counts, saved);
                (label, a, b, r, setup)
            });
            ledger.lap("e_query_scalar_setup");
            if pending.is_none() && next.is_none() {
                break;
            }
            let rank_ref = &mut rank;
            let prelude = pending.take().map(|(pl, pa, pb, pr, pu, psrc)| {
                move || (pl, pa, pb, pr, psrc, graph(pu, pl, pa, pb, pr, rank_ref))
            });
            let (gout, cout) = match next {
                Some((nl, na, nb, nr, setup)) => {
                    let mut qc = Counts::default();
                    let start = Instant::now();
                    let (u, g) = couple_probe_with(t, xidx, nr, &mut buffers, &mut qc, prelude);
                    (
                        g,
                        Some((nl, na, nb, nr, u, start.elapsed().as_millis(), setup, qc)),
                    )
                }
                None => (prelude.map(|f| f()), None),
            };
            ledger.lap("fg_collection_with_graph");
            if let Some((pl, pa, pb, pr, psrc, (nrel, classes, rows, graph_ms, gc))) = gout {
                add_counts(&mut ar.counts, &gc);
                all_rows.extend(rows);
                for k in 0..5 {
                    all_classes[k] += classes[k];
                }
                query_records.push(json!({
                    "label": pl,
                    "a": pa,
                    "b": pb,
                    "query": pr,
                    "classes_by_base_terms": classes,
                    "source_ms": psrc,
                    "graph_and_replay_ms": graph_ms,
                    "lp_rank": rank.lp_rank(),
                    "projected_rows": rank.projected_rows(),
                    "factor_base_target_rank": rank.fb_rank(),
                }));
                eprintln!(
                    "query {pl}: relations={nrel} lp_rank={} projected={} fb_rank={} source_ms={psrc}",
                    rank.lp_rank(),
                    rank.projected_rows(),
                    rank.fb_rank()
                );
                ledger.lap("h_query_record");
                if let Some(d) = rank.target_scalar() {
                    assert_eq!(ar.scalar(PRIME31, d, Some(PRIME31.g)), Q);
                    recovered = Some(d);
                    ledger.lap("i_recovery_check");
                    break;
                }
            }
            let Some((nl, na, nb, nr, u, src, setup, qc)) = cout else {
                break;
            };
            add_counts(&mut ar.counts, &setup);
            add_counts(&mut ar.counts, &qc);
            total_probe_additions += pair_entries as u64;
            pending = Some((nl, na, nb, nr, u, src));
            label += 1;
        }
    }
    for label in 0..if pipelined { 0 } else { max_queries } {
        let a = next_word(&mut seed) % PRIME31.n;
        let b = next_word(&mut seed) % PRIME31.n;
        let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, b, Q);
        let r = ar.add(PRIME31, ag, bq);
        assert!(r.is_some());
        ledger.lap("e_query_scalar_setup");
        let query_start = Instant::now();
        let mut unique = HashSet::<Vec<(u16, i8)>>::new();
        if let Some(xidx) = &xidx {
            let t = ctab.as_ref().expect("couple table");
            unique = couple_probe(t, xidx, r, &mut buffers, &mut ar.counts);
            total_probe_additions += pair_entries as u64;
        } else if levers.streamline {
            fused_probe(
                &pairs,
                r,
                levers.mersenne,
                levers.lazy,
                &mut scratch,
                &mut ar.counts,
                |left, complement| record_matches(left, &complement, &index, &pairs, &mut unique),
            );
            total_probe_additions += pairs.len() as u64;
        } else {
            let batched = if levers.batch {
                Some(batch_complements(
                    &pairs,
                    r,
                    levers.mersenne,
                    &mut ar.counts,
                ))
            } else {
                None
            };
            for (k, left) in pairs.iter().enumerate() {
                let complement = match &batched {
                    Some(all) => all[k],
                    None => {
                        let neg_left = ar.neg(PRIME31, left.sum);
                        ar.add(PRIME31, r, neg_left)
                    }
                };
                total_probe_additions += 1;
                record_matches(left, &complement, &index, &pairs, &mut unique);
            }
        }
        let source_ms = query_start.elapsed().as_millis();
        ledger.lap("f_collection");
        let mut relations: Vec<_> = unique.into_iter().collect();
        relations.sort();
        let graph_start = Instant::now();
        let mut classes = [0usize; 5];
        for terms in &relations {
            let mut sum = None;
            let mut base_terms = 0;
            let mut row = vec![0u64; WIDTH];
            for &(i, sign) in terms {
                let i = i as usize;
                base_terms += usize::from(base[i]);
                let point = if sign == 1 {
                    signed[i][0]
                } else {
                    signed[i][1]
                };
                sum = if couple_ok {
                    exact_add(sum, point, &mut ar.counts)
                } else {
                    ar.add(PRIME31, sum, point)
                };
                row[columns[i]] = if sign == 1 { 1 } else { PRIME31.n - 1 };
            }
            assert_eq!(sum, r, "every emitted equation must replay");
            classes[base_terms] += 1;
            all_rows.push(json!({"label": label, "terms": terms}));
            if base_terms >= 2 {
                row[TARGET] = mod_sub(0, b);
                row[RHS] = a;
                rank.add(row);
            }
        }
        let graph_ms = graph_start.elapsed().as_millis();
        ledger.lap("g_sort_graph_replay");
        for k in 0..5 {
            all_classes[k] += classes[k];
        }
        query_records.push(json!({
            "label": label,
            "a": a,
            "b": b,
            "query": r,
            "classes_by_base_terms": classes,
            "source_ms": source_ms,
            "graph_and_replay_ms": graph_ms,
            "lp_rank": rank.lp_rank(),
            "projected_rows": rank.projected_rows(),
            "factor_base_target_rank": rank.fb_rank(),
        }));
        eprintln!(
            "query {label}: relations={} lp_rank={} projected={} fb_rank={} source_ms={source_ms}",
            relations.len(),
            rank.lp_rank(),
            rank.projected_rows(),
            rank.fb_rank()
        );
        ledger.lap("h_query_record");
        if let Some(d) = rank.target_scalar() {
            assert_eq!(ar.scalar(PRIME31, d, Some(PRIME31.g)), Q);
            recovered = Some(d);
            ledger.lap("i_recovery_check");
            break;
        }
    }
    ledger.lap("j_after_loop");
    let stage_us = ledger.us.clone();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scope": "single-target generic four-sum graph-control; pair enumeration fully charged",
            "variant": "mitm_four_fast",
            "levers": levers.label(),
            "curve": PRIME31,
            "public_target": Q,
            "seed": "0x7033315f6d340101",
            "fixture_certificate": fixture.certificate(),
            "base_size": FB,
            "lp_pool_size": LP,
            "pair_entries": pair_entries,
            "distinct_pair_sums": match &xidx {
                Some(xidx) => xidx.distinct_points(&ctab.as_ref().unwrap().xy),
                None => index.len(),
            },
            "probe_additions": total_probe_additions,
            "classes_by_base_terms": all_classes,
            "queries": query_records,
            "relation_rows": all_rows,
            "lp_rank": rank.lp_rank(),
            "projected_rows": rank.projected_rows(),
            "factor_base_target_rank": rank.fb_rank(),
            "recovered_scalar": recovered,
            "recovered_scalar_replay": recovered.is_some(),
            "fixture_ms": fixture_ms,
            "fixture_work": fixture.counts(),
            "table_ms": table_ms,
            "cold_wall_ms": cold.elapsed().as_millis(),
            "stage_us": stage_us,
            "vm_hwm_kb": vm_hwm_kb(),
            "charged_group_work": ar.counts,
            "total_group_additions": fixture.counts().group_additions + ar.counts.group_additions,
        }))
        .unwrap()
    );
    // The record is complete; with lever 8, flush and exit without running
    // destructors for the tables (the OS reclaims the pages at once).
    if couple_ok {
        use std::io::Write;
        std::io::stdout().flush().expect("flush stdout");
        std::process::exit(0);
    }
}
