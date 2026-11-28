//! Candidate collection loop for `mitm_four.rs` (same source, same output).
//!
//! `mitm_four.rs` stays the unchanged baseline. This binary runs the same
//! fixture, pair table, queries, graph rank and stopping rule, and differs
//! only in how each query's probe complements `r - left.sum` are computed
//! and how the public pair table is hashed. Three switchable levers, given
//! as the second argument (digits, default `1234`; `0` = none):
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
//!    pair and one inversion for the whole table; same sums, same order.
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

fn mod_mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.n as u128) as u64
}

fn mod_sub(x: u64, y: u64) -> u64 {
    (x + PRIME31.n - y) % PRIME31.n
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
}

impl Levers {
    fn parse(text: &str) -> Self {
        assert!(
            text.chars().all(|c| "01234".contains(c)),
            "levers are digits 0-4"
        );
        Self {
            batch: text.contains('1'),
            mersenne: text.contains('2'),
            fast_hash: text.contains('3'),
            batch_table: text.contains('4'),
        }
    }
    fn label(&self) -> String {
        let mut out = String::new();
        for (on, d) in [
            (self.batch, '1'),
            (self.mersenne, '2'),
            (self.fast_hash, '3'),
            (self.batch_table, '4'),
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

fn main() {
    let max_queries = std::env::args()
        .nth(1)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(40);
    let levers = Levers::parse(&std::env::args().nth(2).unwrap_or_else(|| "1234".into()));
    if levers.fast_hash {
        run::<FastBuild>(max_queries, levers)
    } else {
        run::<RandomState>(max_queries, levers)
    }
}

fn run<S: BuildHasher + Default>(max_queries: usize, levers: Levers) {
    let cold = Instant::now();
    let fixture = Hybrid677::build().expect("frozen public B677 support");
    let fixture_ms = cold.elapsed().as_millis();
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
    let table_start = Instant::now();
    let mut pairs = Vec::with_capacity(4 * points.len() * (points.len() - 1) / 2);
    let mut index: HashMap<Point, Vec<usize>, S> =
        HashMap::with_capacity_and_hasher(pairs.capacity(), S::default());
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
                    index.entry(sum).or_default().push(at);
                }
            }
        }
    }
    let table_ms = table_start.elapsed().as_millis();
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
    let mut rank = GraphRank::new();
    let mut query_records = Vec::new();
    let mut all_rows = Vec::new();
    let mut total_probe_additions = 0u64;
    let mut all_classes = [0usize; 5];
    let mut recovered = None;
    for label in 0..max_queries {
        let a = next_word(&mut seed) % PRIME31.n;
        let b = next_word(&mut seed) % PRIME31.n;
        let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, b, Q);
        let r = ar.add(PRIME31, ag, bq);
        assert!(r.is_some());
        let query_start = Instant::now();
        let mut unique = HashSet::<Vec<(u16, i8)>>::new();
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
            let Some(matches) = index.get(&complement) else {
                continue;
            };
            for &right_id in matches {
                let right = pairs[right_id];
                if left.i == right.i || left.i == right.j || left.j == right.i || left.j == right.j
                {
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
        let source_ms = query_start.elapsed().as_millis();
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
                sum = ar.add(PRIME31, sum, point);
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
            "lp_rank": rank.lp_rank,
            "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
        }));
        eprintln!(
            "query {label}: relations={} lp_rank={} projected={} fb_rank={} source_ms={source_ms}",
            relations.len(),
            rank.lp_rank,
            rank.projected_rows,
            rank.fb_rank
        );
        if let Some(d) = rank.target_scalar() {
            assert_eq!(ar.scalar(PRIME31, d, Some(PRIME31.g)), Q);
            recovered = Some(d);
            break;
        }
    }
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
            "pair_entries": pairs.len(),
            "distinct_pair_sums": index.len(),
            "probe_additions": total_probe_additions,
            "classes_by_base_terms": all_classes,
            "queries": query_records,
            "relation_rows": all_rows,
            "lp_rank": rank.lp_rank,
            "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
            "recovered_scalar": recovered,
            "recovered_scalar_replay": recovered.is_some(),
            "fixture_ms": fixture_ms,
            "fixture_work": fixture.counts(),
            "table_ms": table_ms,
            "cold_wall_ms": cold.elapsed().as_millis(),
            "charged_group_work": ar.counts,
            "total_group_additions": fixture.counts().group_additions + ar.counts.group_additions,
        }))
        .unwrap()
    );
}
