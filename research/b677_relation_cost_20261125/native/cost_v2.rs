//! Cheaper fully charged relation source for the frozen B677 M31 workload.
//!
//! Same curve, same frozen 677-point support (509 Dickson + 168 Y7), same
//! public target Q, same query seed and the same graph/rank consumer as the
//! frozen baseline `prime31_graph_ecdlp_20261123/native/mitm_four.rs`.
//! Every emitted relation is replayed with the original group law and the
//! recovered scalar is checked by scalar multiplication.
//!
//! Arms (selected by the first argument):
//!   `exact`   - shared-denominator pair table (P_i+P_j and P_i-P_j from one
//!               inverse), shared-denominator complements (R-L and R+L from
//!               one inverse), Montgomery batch inversion. Probes exactly the
//!               same complements as the baseline, so the relation stream,
//!               rank trajectory and recovered scalar must be identical.
//!   `sample:K` - as `exact`, but each query probes only the table pairs whose
//!               keyed hash falls below K/256. A four-term relation has six
//!               (split, side) left pairs, so probing a fraction f finds it
//!               with probability about 1-(1-f)^6 at cost f: up to six times
//!               fewer probes per new relation, paid for with more queries.
//!
//! All field work is counted in the baseline's convention: one Fermat
//! inversion counts as one inversion plus its square-and-multiply products.

use prime31_algebraic_ic::{Arithmetic, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use serde_json::json;
use std::collections::HashSet;
use std::hash::Hasher;
use std::time::Instant;

const Q: Point = Some((976_979_610, 1_228_979_761));
const FB: usize = 509;
const LP: usize = 168;
const TARGET: usize = FB + LP;
const RHS: usize = TARGET + 1;
const WIDTH: usize = RHS + 1;
const P: u64 = PRIME31.p;
const BLOCK: usize = 512;

// ---------------------------------------------------------------- field ---

#[derive(Default, Clone, Copy)]
struct FieldCounts {
    mul: u64,
    inv: u64,
    points: u64,
}

struct Field {
    c: FieldCounts,
}

impl Field {
    #[inline(always)]
    fn reduce(x: u64) -> u64 {
        // x < 2^62; Mersenne reduction for p = 2^31 - 1.
        let x = (x & P) + (x >> 31);
        let x = (x & P) + (x >> 31);
        if x >= P {
            x - P
        } else {
            x
        }
    }
    #[inline(always)]
    fn mul(&mut self, x: u64, y: u64) -> u64 {
        self.c.mul += 1;
        Self::reduce(x * y)
    }
    #[inline(always)]
    fn sub(x: u64, y: u64) -> u64 {
        if x >= y {
            x - y
        } else {
            x + P - y
        }
    }
    #[inline(always)]
    fn add(x: u64, y: u64) -> u64 {
        let s = x + y;
        if s >= P {
            s - P
        } else {
            s
        }
    }
    fn inv(&mut self, x: u64) -> u64 {
        assert!(x != 0 && x < P);
        self.c.inv += 1;
        let (mut out, mut b, mut e) = (1u64, x, P - 2);
        while e > 0 {
            if e & 1 != 0 {
                out = self.mul(out, b);
            }
            b = self.mul(b, b);
            e >>= 1;
        }
        out
    }
    /// Montgomery simultaneous inversion: 3(m-1) products plus one inverse.
    fn batch_inv(&mut self, xs: &mut [u64], scratch: &mut Vec<u64>) {
        if xs.is_empty() {
            return;
        }
        scratch.clear();
        let mut acc = xs[0];
        scratch.push(acc);
        for &x in &xs[1..] {
            acc = self.mul(acc, x);
            scratch.push(acc);
        }
        let mut inv = self.inv(acc);
        for k in (1..xs.len()).rev() {
            let xk = xs[k];
            xs[k] = self.mul(inv, scratch[k - 1]);
            inv = self.mul(inv, xk);
        }
        xs[0] = inv;
    }
    /// Given u=(x,y), v=(z,w) with x != z and inverse d = 1/(z-x), return
    /// (u+v, u-v). Shares the denominator: 2 slopes, 2 squares, 2 products.
    #[inline(always)]
    fn sum_diff(&mut self, u: (u64, u64), v: (u64, u64), d: u64) -> ((u64, u64), (u64, u64)) {
        self.c.points += 2;
        let (x, y) = u;
        let (z, w) = v;
        let lp = self.mul(Self::sub(w, y), d);
        let lm = self.mul(Self::sub(P - w, y) % P, d);
        let xp = Self::sub(Self::sub(self.mul(lp, lp), x), z);
        let yp = Self::sub(self.mul(lp, Self::sub(x, xp)), y);
        let xm = Self::sub(Self::sub(self.mul(lm, lm), x), z);
        let ym = Self::sub(self.mul(lm, Self::sub(x, xm)), y);
        ((xp, yp), (xm, ym))
    }
}

fn neg(p: (u64, u64)) -> (u64, u64) {
    (p.0, (P - p.1) % P)
}

// ------------------------------------------------------------- hashing ---

#[derive(Default)]
struct Fx(u64);
impl Hasher for Fx {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
        }
    }
    fn write_u64(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(5) ^ v).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

#[inline(always)]
fn key(p: (u64, u64)) -> u64 {
    (p.0 << 31) | p.1
}

const NIL: u32 = u32::MAX;

/// Open-addressing point index: key -> first entry id, with an intrusive
/// chain for the (rare) repeated pair sums. Replaces HashMap<_, Vec<_>>.
struct FlatIndex {
    keys: Vec<u64>,
    heads: Vec<u32>,
    next: Vec<u32>,
    mask: usize,
    distinct: usize,
}

impl FlatIndex {
    fn new(entries: usize) -> Self {
        let size = (2 * entries).next_power_of_two();
        Self { keys: vec![u64::MAX; size], heads: vec![NIL; size], next: vec![NIL; entries], mask: size - 1, distinct: 0 }
    }
    #[inline(always)]
    fn slot(&self, k: u64) -> usize {
        (mix(k) as usize) & self.mask
    }
    fn insert(&mut self, k: u64, id: u32) {
        let mut s = self.slot(k);
        loop {
            if self.keys[s] == u64::MAX {
                self.keys[s] = k;
                self.heads[s] = id;
                self.distinct += 1;
                return;
            }
            if self.keys[s] == k {
                // Append to keep ascending id order, as Vec::push did.
                let mut c = self.heads[s];
                while self.next[c as usize] != NIL {
                    c = self.next[c as usize];
                }
                self.next[c as usize] = id;
                return;
            }
            s = (s + 1) & self.mask;
        }
    }
    #[inline(always)]
    fn first(&self, k: u64) -> u32 {
        let mut s = self.slot(k);
        loop {
            let kk = self.keys[s];
            if kk == k {
                return self.heads[s];
            }
            if kk == u64::MAX {
                return NIL;
            }
            s = (s + 1) & self.mask;
        }
    }
}

// --------------------------------------------- consumer (unchanged math) ---

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

/// Verbatim copy of the baseline's LP-then-factor-base elimination.
struct GraphRank {
    lp_pivots: Vec<Option<Vec<u64>>>,
    fb_pivots: Vec<Option<Vec<u64>>>,
    lp_rank: usize,
    fb_rank: usize,
    projected_rows: usize,
    scalar_ops: u64,
}

impl GraphRank {
    fn new() -> Self {
        Self {
            lp_pivots: vec![None; LP],
            fb_pivots: vec![None; FB + 1],
            lp_rank: 0,
            fb_rank: 0,
            projected_rows: 0,
            scalar_ops: 0,
        }
    }
    fn eliminate(row: &mut [u64], pivot: &[u64], coefficient: u64, ops: &mut u64) {
        for col in 0..WIDTH {
            if pivot[col] != 0 {
                *ops += 1;
                row[col] = mod_sub(row[col], mod_mul(coefficient, pivot[col]));
            }
        }
    }
    fn normalize(row: &mut [u64], coefficient: u64, ops: &mut u64) {
        let inverse = mod_inverse(coefficient);
        *ops += 62;
        for value in row {
            if *value != 0 {
                *ops += 1;
                *value = mod_mul(*value, inverse);
            }
        }
    }
    fn add(&mut self, mut row: Vec<u64>) {
        let ops = &mut self.scalar_ops;
        for at in 0..LP {
            let col = FB + at;
            let coefficient = row[col];
            if coefficient == 0 {
                continue;
            }
            if let Some(pivot) = &self.lp_pivots[at] {
                Self::eliminate(&mut row, pivot, coefficient, ops);
            } else {
                Self::normalize(&mut row, coefficient, ops);
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
                Self::eliminate(&mut row, pivot, coefficient, ops);
            } else {
                Self::normalize(&mut row, coefficient, ops);
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

fn mix(mut z: u64) -> u64 {
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

#[derive(Clone, Copy)]
struct Pair {
    i: u16,
    j: u16,
    si: i8,
    sj: i8,
}

fn main() {
    let arm = std::env::args().nth(1).unwrap_or_else(|| "exact".into());
    let (sample_k, sample_salt) = if arm == "exact" {
        (256u64, 0u64)
    } else if let Some(rest) = arm.strip_prefix("sample:") {
        let mut it = rest.split(':');
        let k: u64 = it.next().unwrap().parse().expect("sample:K");
        let salt: u64 = it.next().map(|s| s.parse().unwrap()).unwrap_or(0);
        assert!((1..=256).contains(&k));
        (k, salt)
    } else {
        panic!("arm must be `exact` or `sample:K[:salt]`");
    };
    let max_queries: usize = std::env::args()
        .nth(2)
        .and_then(|v| v.parse().ok())
        .unwrap_or(if sample_k == 256 { 40 } else { 2048 });

    let cold = Instant::now();
    let fixture = Hybrid677::build().expect("frozen public B677 support");
    let fixture_ms = cold.elapsed().as_secs_f64() * 1e3;
    let points: Vec<(u64, u64)> = fixture.points().to_vec();
    let base: Vec<bool> = points
        .iter()
        .map(|(x, _)| dickson1024(*x) == fixture.terminal())
        .collect();
    assert_eq!(base.iter().filter(|&&f| f).count(), FB);
    assert_eq!(points.len(), FB + LP);
    let mut ar = Arithmetic::default();
    assert!(ar.on_curve(PRIME31, Q));
    let mut f = Field { c: FieldCounts::default() };
    let mut scratch = Vec::with_capacity(BLOCK);

    // ---- pair table: 2 points per shared inverse, batch-inverted ----
    let table_start = Instant::now();
    let n = points.len();
    let npairs = n * (n - 1) / 2;
    let mut pairs: Vec<Pair> = Vec::with_capacity(4 * npairs);
    // rep[k] = (P_i+P_j, P_i-P_j) for the k-th unordered pair.
    let mut rep: Vec<((u64, u64), (u64, u64))> = Vec::with_capacity(npairs);
    let mut index = FlatIndex::new(4 * npairs);
    let all_ij: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .collect();
    let mut dens = Vec::with_capacity(BLOCK);
    for chunk in all_ij.chunks(BLOCK) {
        dens.clear();
        dens.extend(
            chunk
                .iter()
                .map(|&(i, j)| Field::sub(points[j].0, points[i].0)),
        );
        f.batch_inv(&mut dens, &mut scratch);
        for (t, &(i, j)) in chunk.iter().enumerate() {
            let (s, d) = f.sum_diff(points[i], points[j], dens[t]);
            rep.push((s, d));
            // Baseline order: (+,+), (+,-), (-,+), (-,-) = S, D, -D, -S.
            for (sum, si, sj) in [(s, 1, 1), (d, 1, -1), (neg(d), -1, 1), (neg(s), -1, -1)] {
                let at = pairs.len() as u32;
                pairs.push(Pair { i: i as u16, j: j as u16, si, sj });
                index.insert(key(sum), at);
            }
        }
    }
    let table_ms = table_start.elapsed().as_secs_f64() * 1e3;
    let distinct_pair_sums = index.distinct;

    let mut columns = vec![usize::MAX; n];
    let (mut fb_next, mut lp_next) = (0, FB);
    for (i, &is_base) in base.iter().enumerate() {
        if is_base {
            columns[i] = fb_next;
            fb_next += 1;
        } else {
            columns[i] = lp_next;
            lp_next += 1;
        }
    }

    let mut seed = 0x7033_315f_6d34_0101u64;
    let mut rank = GraphRank::new();
    let mut query_records = Vec::new();
    let mut all_rows: Vec<(usize, Vec<(u16, i8)>)> = Vec::new();
    let mut probes = 0u64;
    let mut fallback_adds = 0u64;
    let mut all_classes = [0usize; 5];
    let mut recovered = None;
    let mut source_ms_total = 0.0;
    let mut graph_ms_total = 0.0;
    let mut selected: Vec<usize> = Vec::with_capacity(npairs);
    for label in 0..max_queries {
        let a = next_word(&mut seed) % PRIME31.n;
        let b = next_word(&mut seed) % PRIME31.n;
        let ag = ar.scalar(PRIME31, a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, b, Q);
        let r = ar.add(PRIME31, ag, bq).expect("query is finite");
        let query_start = Instant::now();
        let mut unique = HashSet::<Vec<(u16, i8)>>::new();
        let salt = mix(0x5a17_0000_0000_0000 ^ (sample_salt << 32) ^ label as u64);
        selected.clear();
        for k in 0..npairs {
            if sample_k == 256 || (mix(salt ^ k as u64) & 255) < sample_k {
                selected.push(k);
            }
        }
        // Each representative point L in {S_k, D_k} is probed together with
        // -L: complements R-L and R+L share the inverse 1/(x_L - x_R).
        let mut handle = |c: (u64, u64), left_id: usize, unique: &mut HashSet<Vec<(u16, i8)>>| {
            let mut cursor = index.first(key(c));
            if cursor == NIL {
                return;
            }
            let left = pairs[left_id];
            while cursor != NIL {
                let right_id = cursor;
                cursor = index.next[cursor as usize];
                let right = pairs[right_id as usize];
                if left.i == right.i || left.i == right.j || left.j == right.i || left.j == right.j {
                    continue;
                }
                let mut terms = vec![
                    (left.i, left.si),
                    (left.j, left.sj),
                    (right.i, right.si),
                    (right.j, right.sj),
                ];
                terms.sort_unstable_by_key(|t| t.0);
                unique.insert(terms);
            }
        };
        let mut lefts: Vec<(usize, usize)> = Vec::with_capacity(BLOCK); // (k, 0=S|1=D)
        for chunk in selected.chunks(BLOCK / 2) {
            lefts.clear();
            dens.clear();
            for &k in chunk {
                for which in 0..2 {
                    let l = if which == 0 { rep[k].0 } else { rep[k].1 };
                    if l.0 == r.0 {
                        // L = +-R: complement is O or 2R; use the generic law.
                        fallback_adds += 2;
                        probes += 2;
                        let base_id = 4 * k;
                        let (pos_id, neg_id) =
                            if which == 0 { (base_id, base_id + 3) } else { (base_id + 1, base_id + 2) };
                        for (lid, lp) in [(pos_id, l), (neg_id, neg(l))] {
                            if let Some(c) = ar.add(PRIME31, Some(r), Some(neg(lp))) {
                                handle(c, lid, &mut unique);
                            }
                        }
                        continue;
                    }
                    lefts.push((k, which));
                    dens.push(Field::sub(r.0, l.0));
                }
            }
            f.batch_inv(&mut dens, &mut scratch);
            for (t, &(k, which)) in lefts.iter().enumerate() {
                let l = if which == 0 { rep[k].0 } else { rep[k].1 };
                // sum_diff(L, R) gives (L+R, L-R); we need R-L = -(L-R) and R+L.
                let (lpr, lmr) = f.sum_diff(l, r, dens[t]);
                probes += 2;
                let base_id = 4 * k;
                let (pos_id, neg_id) =
                    if which == 0 { (base_id, base_id + 3) } else { (base_id + 1, base_id + 2) };
                handle(neg(lmr), pos_id, &mut unique); // left = +L, complement R-L
                handle(lpr, neg_id, &mut unique); // left = -L, complement R+L
            }
        }
        let source_ms = query_start.elapsed().as_secs_f64() * 1e3;
        source_ms_total += source_ms;
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
                let pt = if sign == 1 { points[i] } else { neg(points[i]) };
                sum = ar.add(PRIME31, sum, Some(pt));
                row[columns[i]] = if sign == 1 { 1 } else { PRIME31.n - 1 };
            }
            assert_eq!(sum, Some(r), "every emitted equation must replay");
            classes[base_terms] += 1;
            all_rows.push((label, terms.clone()));
            if base_terms >= 2 {
                row[TARGET] = mod_sub(0, b);
                row[RHS] = a;
                rank.add(row);
            }
        }
        let graph_ms = graph_start.elapsed().as_secs_f64() * 1e3;
        graph_ms_total += graph_ms;
        for k in 0..5 {
            all_classes[k] += classes[k];
        }
        query_records.push(json!({
            "label": label, "a": a, "b": b, "probed_pairs": selected.len(),
            "relations": relations.len(), "classes_by_base_terms": classes,
            "source_ms": source_ms, "graph_and_replay_ms": graph_ms,
            "lp_rank": rank.lp_rank, "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
        }));
        eprintln!(
            "query {label}: probed_pairs={} relations={} lp_rank={} projected={} fb_rank={} source_ms={source_ms:.1}",
            selected.len(), relations.len(), rank.lp_rank, rank.projected_rows, rank.fb_rank
        );
        if let Some(d) = rank.target_scalar() {
            assert_eq!(ar.scalar(PRIME31, d, Some(PRIME31.g)), Q);
            recovered = Some(d);
            break;
        }
    }
    let cold_wall_ms = cold.elapsed().as_secs_f64() * 1e3;
    let fx = fixture.counts();
    let total_mul = fx.fp_multiplications + ar.counts.fp_multiplications + f.c.mul;
    let total_inv = fx.fp_inversions + ar.counts.fp_inversions + f.c.inv;
    let total_points = fx.group_additions + ar.counts.group_additions + f.c.points;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scope": "fully charged single-target generic four-sum source on frozen B677; candidate arm",
            "arm": arm,
            "sample_fraction": sample_k as f64 / 256.0,
            "curve": PRIME31,
            "public_target": Q,
            "seed": "0x7033315f6d340101",
            "base_size": FB,
            "lp_pool_size": LP,
            "pair_entries": pairs.len(),
            "distinct_pair_sums": distinct_pair_sums,
            "queries_used": query_records.len(),
            "probe_points": probes,
            "probe_fallback_generic_adds": fallback_adds,
            "relations_emitted": all_rows.len(),
            "classes_by_base_terms": all_classes,
            "lp_rank": rank.lp_rank,
            "projected_rows": rank.projected_rows,
            "factor_base_target_rank": rank.fb_rank,
            "recovered_scalar": recovered,
            "recovered_scalar_replay": recovered.is_some(),
            "fixture_ms": fixture_ms,
            "table_ms": table_ms,
            "source_ms": source_ms_total,
            "graph_and_replay_ms": graph_ms_total,
            "cold_wall_ms": cold_wall_ms,
            "charged": {
                "fixture": fx,
                "generic_group_law": ar.counts,
                "batched_field": {"fp_multiplications": f.c.mul, "fp_inversions": f.c.inv, "points_computed": f.c.points},
                "rank_scalar_ops": rank.scalar_ops,
                "total_fp_multiplications_incl_inversions": total_mul,
                "total_fp_inversions": total_inv,
                "total_points_computed": total_points,
            },
            "queries": query_records,
            "relation_rows": all_rows.iter().map(|(l, t)| json!({"label": l, "terms": t})).collect::<Vec<_>>(),
        }))
        .unwrap()
    );
}
