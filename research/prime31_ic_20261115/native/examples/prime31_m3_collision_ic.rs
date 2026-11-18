//! Complete *experimental* collision/anchor/target index-calculus pipeline on
//! the ordinary full-prime-order p=2^31-1 curve.
//! Reuses the checked-in Rust Arithmetic, Base and curve certificate.
//! IMPORTANT: The triple bank is built by generic EC enumeration (not F4).
//! This is a correctness/cost control, not a cheaper algebraic oracle.
//!
//! cargo +1.100.0 run --locked --release \
//!   --manifest-path research/prime31_ic_20261115/native/Cargo.toml \
//!   --example prime31_m3_collision_ic
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::json;
use std::collections::{HashMap, HashSet};

const Q: Point = Some((1_547_953_785, 1_667_575_595));
const VERIFIER_SCALAR_ONLY: u64 = 661_068_045;
const MAX_QUERY_TRIALS: u64 = 100_000;

fn times(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}
fn sub(a: u64, b: u64, n: u64) -> u64 {
    (a + n - b) % n
}
fn inverse(a: u64, n: u64) -> u64 {
    assert_ne!(a, 0);
    let (mut acc, mut b, mut e) = (1_u64, a, n - 2);
    while e != 0 {
        if e & 1 != 0 {
            acc = times(acc, b, n);
        }
        b = times(b, b, n);
        e >>= 1;
    }
    acc
}
fn point_key(q: Point) -> u64 {
    match q {
        None => u64::MAX,
        Some((x, y)) => {
            assert!(x < (1 << 31) && y < (1 << 31));
            (x << 31) | y
        }
    }
}

// 9 bits per index, 3 sign bits: one byte-packed signed distinct-index triple.
// A one bit means negative. The canonical enumeration always has i<j<k.
fn triple_code(i: usize, j: usize, k: usize, signs: u8) -> u32 {
    assert!(i < j && j < k && k < 512 && signs < 8);
    (i as u32) | ((j as u32) << 9) | ((k as u32) << 18) | ((signs as u32) << 27)
}
fn decode(code: u32) -> [(usize, i8); 3] {
    let signs = code >> 27;
    let indices = [
        (code & 511) as usize,
        ((code >> 9) & 511) as usize,
        ((code >> 18) & 511) as usize,
    ];
    [
        (indices[0], if signs & 1 != 0 { -1 } else { 1 }),
        (indices[1], if signs & 2 != 0 { -1 } else { 1 }),
        (indices[2], if signs & 4 != 0 { -1 } else { 1 }),
    ]
}
fn add_coefficient(row: &mut [u64], i: usize, sign: i8, multiplier: i8, n: u64) {
    assert!(i < row.len());
    if sign == multiplier {
        row[i] = (row[i] + 1) % n;
    } else {
        row[i] = (row[i] + n - 1) % n;
    }
}
fn collision_row(previous: u32, current: u32, b: usize, n: u64) -> Vec<u64> {
    let mut row = vec![0_u64; b];
    for (i, sign) in decode(current) {
        add_coefficient(&mut row, i, sign, 1, n);
    }
    for (i, sign) in decode(previous) {
        add_coefficient(&mut row, i, sign, -1, n);
    }
    row
}
/// Canonicalize a nonzero homogeneous relation modulo the prime group order.
/// Scalar multiples span the same row space and must not be counted as
/// independent oracle outputs. Sparse (index, coefficient) pairs save memory.
fn projective_relation(row: &[u64], n: u64) -> Option<Vec<(u16, u64)>> {
    let first = row.iter().copied().find(|&coefficient| coefficient != 0)?;
    let scale = inverse(first, n);
    Some(
        row.iter()
            .enumerate()
            .filter(|(_, &coefficient)| coefficient != 0)
            .map(|(index, &coefficient)| {
                assert!(index <= u16::MAX as usize);
                (index as u16, times(coefficient, scale, n))
            })
            .collect(),
    )
}

fn weighted_sum(code: u32, weights: &[u64], n: u64) -> u64 {
    let mut out = 0_u64;
    for (i, sign) in decode(code) {
        assert!(i < weights.len());
        out = if sign == 1 {
            (out + weights[i]) % n
        } else {
            sub(out, weights[i], n)
        };
    }
    out
}

struct HomogeneousRank {
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
}
impl HomogeneousRank {
    fn new(b: usize) -> Self {
        Self { pivots: vec![None; b], rank: 0 }
    }
    fn insert(&mut self, mut row: Vec<u64>, n: u64) {
        let b = self.pivots.len();
        for j in 0..b {
            let coeff = row[j];
            if coeff == 0 {
                continue;
            }
            if let Some(pivot) = self.pivots[j].as_ref() {
                for c in j..b {
                    row[c] = sub(row[c], times(coeff, pivot[c], n), n);
                }
            } else {
                let inv = inverse(coeff, n);
                for value in &mut row[j..] {
                    *value = times(*value, inv, n);
                }
                self.pivots[j] = Some(row);
                self.rank += 1;
                return;
            }
        }
    }
    fn kernel(&self, n: u64) -> Option<Vec<u64>> {
        let b = self.pivots.len();
        if self.rank != b - 1 {
            return None;
        }
        let free = self.pivots.iter().position(Option::is_none)?;
        let mut out = vec![0_u64; b];
        out[free] = 1;
        for j in (0..b).rev() {
            if let Some(pivot) = self.pivots[j].as_ref() {
                let mut accum = 0;
                for c in (j + 1)..b {
                    accum = (accum + times(pivot[c], out[c], n)) % n;
                }
                out[j] = sub(0, accum, n);
            }
        }
        for (j, pivot) in self.pivots.iter().enumerate() {
            if let Some(pivot) = pivot {
                let dot = (j..b).fold(0_u64, |s, c| {
                    (s + times(pivot[c], out[c], n)) % n
                });
                assert_eq!(dot, 0, "kernel back-substitution invalid");
            }
        }
        Some(out)
    }
}
struct TripleBank {
    first: HashMap<u64, u32>,
    hom: HomogeneousRank,
    templates: u64,
    collisions: u64,
    /// Number of same-sum collision rows rejected as projectively duplicate.
    duplicate_projective_rows: u64,
    /// Actual nonzero relation classes modulo F_n^*; NOT the independent rank.
    projective_rows: HashSet<Vec<(u16, u64)>>,
    first_full_rank: Option<(u64, u64)>,
}
impl TripleBank {
    fn new(b: usize) -> Self {
        // A rank-first bank should not reserve the entire O(B^3) exhaustive
        // support. B^2*64 suffices for the frozen B=256 fixture without
        // a rehash before its measured first rank-255 collision.
        let initial_capacity = b * b * 64;
        Self {
            first: HashMap::with_capacity(initial_capacity),
            hom: HomogeneousRank::new(b),
            templates: 0,
            collisions: 0,
            duplicate_projective_rows: 0,
            projective_rows: HashSet::new(),
            first_full_rank: None,
        }
    }
    fn offer(&mut self, point: Point, code: u32, n: u64) {
        self.templates += 1;
        let key = point_key(point);
        if let Some(&previous) = self.first.get(&key) {
            self.collisions += 1;
            if self.hom.rank < self.hom.pivots.len() - 1 {
                let row = collision_row(previous, code, self.hom.pivots.len(), n);
                let normalized = projective_relation(&row, n)
                    .expect("distinct triple codes produced a zero formal row");
                if self.projective_rows.insert(normalized) {
                    self.hom.insert(row, n);
                } else {
                    self.duplicate_projective_rows += 1;
                }
            }
            if self.hom.rank == self.hom.pivots.len() - 1
                && self.first_full_rank.is_none()
            {
                self.first_full_rank = Some((self.collisions, self.templates));
            }
        } else {
            self.first.insert(key, code);
        }
    }
}

fn enumerate_triples(base: &Base, ar: &mut Arithmetic) -> TripleBank {
    let b = base.points.len();
    let mut bank = TripleBank::new(b);
    for i in 0..b {
        let u = Some(base.points[i]);
        for j in (i + 1)..b {
            let v = Some(base.points[j]);
            let uv_plus = ar.add(PRIME31, u, v);
            let neg_v = ar.neg(PRIME31, v);
            let uv_minus = ar.add(PRIME31, u, neg_v);
            for k in (j + 1)..b {
                let w = Some(base.points[k]);
                let neg_w = ar.neg(PRIME31, w);
                let variants: [(Point, u8); 4] = [
                    (ar.add(PRIME31, uv_plus, w), 0),
                    (ar.add(PRIME31, uv_plus, neg_w), 4),
                    (ar.add(PRIME31, uv_minus, w), 2),
                    (ar.add(PRIME31, uv_minus, neg_w), 6),
                ];
                for (sum, signs) in variants {
                    bank.offer(sum, triple_code(i, j, k, signs), PRIME31.n);
                    if bank.hom.rank == b - 1 {
                        return bank;
                    }
                    bank.offer(ar.neg(PRIME31, sum), triple_code(i, j, k, signs ^ 7), PRIME31.n);
                    if bank.hom.rank == b - 1 {
                        return bank;
                    }
                }
            }
        }
    }
    bank
}

/// First B liftable abscissae, a target-independent non-Dickson control.
fn plain_x_base(b: usize, ar: &mut Arithmetic) -> Result<Base, String> {
    let mut points = Vec::with_capacity(b);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            if y != 0 {
                points.push((x, y.min(PRIME31.p - y)));
            }
        }
        if points.len() == b {
            return Ok(Base { depth: 0, terminal: 0, values: Vec::new(), points });
        }
    }
    Err("not enough liftable x values in requested prime field".into())
}

/// Optional exact full enumeration controls, kept outside the cold target
/// solve. They compare the same number of points with and without the
/// Dickson constraint. Neither constructs an algebraic f6 relation oracle.
fn compare_b135_controls() -> Result<serde_json::Value, String> {
    let mut ar = Arithmetic::default();
    let dickson = Base::build(PRIME31, 8, &mut ar)?;
    if dickson.points.len() != 135 {
        return Err("changed Dickson control size".into());
    }
    let d = enumerate_triples(&dickson, &mut ar);
    if d.templates != 3_207_960 || d.collisions != 2_370
        || d.first.len() != 3_205_590 || d.projective_rows.len() != 127
        || d.duplicate_projective_rows != 2_243 || d.hom.rank != 126
    {
        return Err("Dickson depth-8 signed collision census mismatch".into());
    }
    let natural = plain_x_base(135, &mut ar)?;
    let c = enumerate_triples(&natural, &mut ar);
    if c.templates != 3_207_960 || c.collisions != 2_244
        || c.first.len() != 3_205_716 || c.projective_rows.len() != 122
        || c.duplicate_projective_rows != 2_122 || c.hom.rank != 122
    {
        return Err("plain-x signed collision census mismatch".into());
    }
    Ok(json!({
        "factor_base_size":135,
        "dickson_depth8":{"collisions":d.collisions,
            "projective_unique":d.projective_rows.len(),"rank":d.hom.rank,
            "templates":d.templates},
        "plain_first_135_x":{"collisions":c.collisions,
            "projective_unique":c.projective_rows.len(),"rank":c.hom.rank,
            "templates":c.templates},
        "standing":"[pcc:unverified]",
        "oracle":"complete signed m3 generic enumeration, NOT F4"
    }))
}

fn signed_point(code: u32, base: &Base, ar: &mut Arithmetic) -> Point {
    let mut out = None;
    for (i, sign) in decode(code) {
        let p = Some(base.points[i]);
        let p = if sign == -1 { ar.neg(PRIME31, p) } else { p };
        out = ar.add(PRIME31, out, p);
    }
    out
}
struct SplitMix(u64);
impl SplitMix {
    fn next(&mut self, n: u64) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        (z ^ (z >> 31)) % n
    }
}

fn main() -> Result<(), String> {
    let e = PRIME31;
    let mut ar = Arithmetic::default();
    check_curve(e, &mut ar)?;
    if !ar.on_curve(e, Q) || Q.is_none() {
        return Err("invalid public target".into());
    }
    let base = Base::build(e, 9, &mut ar)?;
    if base.points.len() != 256 || base.terminal != 375_193_997 {
        return Err("the frozen ordinary-curve factor base changed".into());
    }
    let before_bank = ar.counts;
    let bank = enumerate_triples(&base, &mut ar);
    let after_bank = ar.counts;
    if bank.templates != 4_051_363
        || bank.first.len() != 4_047_598
        || bank.collisions != 3_765
        || bank.hom.rank != 255
        || bank.projective_rows.len() != 305
        || bank.duplicate_projective_rows != 3_460
        || bank.first_full_rank != Some((3_765, 4_051_363))
    {
        return Err("native m3 census/rank differs from independent producer fixture".into());
    }
    let weights = bank.hom.kernel(e.n).ok_or("rank B-1 kernel unavailable")?;
    let mut anchor_rng = SplitMix(0x6d3300);
    let mut anchor = None;
    for trial in 1..=MAX_QUERY_TRIALS {
        let a = anchor_rng.next(e.n);
        if a == 0 { continue; }
        let point = ar.scalar(e, a, Some(e.g));
        if let Some(&code) = bank.first.get(&point_key(point)) {
            if signed_point(code, &base, &mut ar) != point {
                return Err("anchor witness replay failed".into());
            }
            anchor = Some((trial, a, code));
            break;
        }
    }
    let (anchor_trials, anchor_a, anchor_code) =
        anchor.ok_or("anchor hit was censored")?;
    let anchor_sum = weighted_sum(anchor_code, &weights, e.n);
    if anchor_sum == 0 {
        return Err("nonzero anchor had zero projective weight".into());
    }
    let lambda = times(anchor_a, inverse(anchor_sum, e.n), e.n);
    // Independent relation-level replay: each basis point must be [lambda*t_i]G.
    for (i, &p) in base.points.iter().enumerate() {
        if ar.scalar(e, times(lambda, weights[i], e.n), Some(e.g)) != Some(p) {
            return Err(format!("basis logarithm validation failed at index {i}"));
        }
    }
    let mut target_rng = SplitMix(0x6d3301);
    let mut target = None;
    for trial in 1..=MAX_QUERY_TRIALS {
        let a = target_rng.next(e.n);
        let b = target_rng.next(e.n);
        if b == 0 { continue; }
        let ag = ar.scalar(e, a, Some(e.g));
        let bq = ar.scalar(e, b, Q);
        let mixed = ar.add(e, ag, bq);
        if let Some(&code) = bank.first.get(&point_key(mixed)) {
            if signed_point(code, &base, &mut ar) != mixed {
                return Err("target witness replay failed".into());
            }
            target = Some((trial, a, b, code));
            break;
        }
    }
    let (target_trials, a, b, target_code) =
        target.ok_or("natural target hit was censored")?;
    let scalar = times(
        sub(times(lambda, weighted_sum(target_code, &weights, e.n), e.n), a, e.n),
        inverse(b, e.n),
        e.n,
    );
    if ar.scalar(e, scalar, Some(e.g)) != Q || scalar != VERIFIER_SCALAR_ONLY {
        return Err("independent full scalar verification failed".into());
    }
    let b135_controls = if std::env::var_os("PRIME31_COMPARE_BASES").is_some() {
        Some(compare_b135_controls()?)
    } else {
        None
    };
    println!("{}", serde_json::to_string_pretty(&json!({
        "standing": "[pcc:unverified]",
        "mode": "signed-distinct-index m3 collision/anchor/target index calculus",
        "oracle": "explicit generic group enumeration, NOT algebraic F4",
        "prime":e.p, "curve":[e.a,e.b], "prime_order":e.n,
        "generator":e.g, "public_target":Q, "factor_base_size":base.points.len(),
        "templates":bank.templates,"unique_sums":bank.first.len(),
        "homogeneous_collision_rows":bank.collisions,
        "unique_projective_collision_rows":bank.projective_rows.len(),
        "duplicate_projective_rows":bank.duplicate_projective_rows,
        "homogeneous_rank":bank.hom.rank,"full_rank_at":bank.first_full_rank,
        "b135_full_enumeration_controls":b135_controls,
        "anchor_trials":anchor_trials,"anchor_scalar":anchor_a,
        "target_trials":target_trials,"target_mix_a":a,"target_mix_b":b,
        "recovered_scalar":scalar,"basis_log_replays":base.points.len(),
        "construction_group_additions":after_bank.group_additions-before_bank.group_additions,
        "all_counted_group_additions":ar.counts.group_additions,
        "arithmetic":ar.counts,
        "limitations":"unpriced symbolics, memory and F4; generic enumeration dominates cold cost; no speedup claim"
    })).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_tuple_encoding_and_relation_difference() {
        let a = triple_code(1, 7, 129, 0);
        let b = triple_code(1, 7, 129, 4);
        assert_eq!(decode(a), [(1, 1), (7, 1), (129, 1)]);
        assert_eq!(decode(b), [(1, 1), (7, 1), (129, -1)]);
        let row = collision_row(a, b, 130, 977);
        assert_eq!(row.iter().filter(|&&c| c != 0).count(), 1);
        assert_eq!(row[129], 975); // -2 modulo 977
        assert_eq!(weighted_sum(a, &[1; 130], 977), 3);
        assert_eq!(weighted_sum(b, &[1; 130], 977), 1);
    }

    #[test]
    fn projective_normalization_removes_scaling_not_distinct_rows() {
        let n = 977;
        let a = vec![0, n - 2, 0, 4];
        let scaled = vec![0, n - 1, 0, 2];
        let opposite = vec![0, 2, 0, n - 4];
        let independent = vec![0, n - 1, 0, 3];
        assert_eq!(projective_relation(&a, n), projective_relation(&scaled, n));
        assert_eq!(projective_relation(&a, n), projective_relation(&opposite, n));
        assert_ne!(projective_relation(&a, n), projective_relation(&independent, n));
        assert_eq!(projective_relation(&[0, 0, 0], n), None);
    }

    #[test]
    fn homogeneous_rank_has_one_dimensional_kernel() {
        let n = 977;
        let mut rank = HomogeneousRank::new(4);
        rank.insert(vec![1, n - 1, 0, 0], n);
        rank.insert(vec![0, 1, n - 1, 0], n);
        rank.insert(vec![0, 0, 1, n - 1], n);
        rank.insert(vec![1, 0, 0, n - 1], n);
        assert_eq!(rank.rank, 3);
        assert_eq!(rank.kernel(n), Some(vec![1, 1, 1, 1]));
    }
}
