//! Complete mixed-base 4+4 index-calculus-derived ECDLP solver on the existing
//! ordinary PRIME31 curve. Generic half-sum point tables: *not* S9 elimination.
//! Preregistered at MIXED_4P4_IC_PROTOCOL_20261117.md. [pcc:unverified]
//!
//! cargo +1.100.0 test --locked --release --manifest-path \
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_mixed_base_ic
//! cargo +1.100.0 run --locked --release --manifest-path \
//!   research/prime31_ic_20261115/native/Cargo.toml --example prime31_mixed_base_ic
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

const B: usize = 31;
const TARGET_SEED: u64 = 0x4d49_5834_5033_3131;
const RELATION_SEED: u64 = 0x4d49_5834_5245_4c31;
const MAX_RELATIONS: usize = 160;
const TARGETS: usize = 2;
type Sig = [i8; B];

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn first_rational_points(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut out = Vec::with_capacity(B);
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0);
            out.push((x, y.min(PRIME31.p - y)));
            if out.len() == B {
                return out;
            }
        }
    }
    panic!("no 31 rational x points");
}

fn signed_points(ar: &Arithmetic, base: &[(u64, u64)]) -> Vec<Point> {
    let mut pts = Vec::with_capacity(2 * base.len());
    for &p in base {
        pts.push(Some(p));
        pts.push(ar.neg(PRIME31, Some(p)));
    }
    pts
}

fn signature(word: [usize; 4]) -> Sig {
    let mut s = [0_i8; B];
    for i in word {
        s[i / 2] += if i % 2 == 0 { 1 } else { -1 };
    }
    s
}

fn replay(ar: &mut Arithmetic, pts: &[Point], s: &Sig) -> Point {
    let mut result = None;
    let mut weight = 0;
    for (i, &v) in s.iter().enumerate() {
        weight += v.unsigned_abs() as usize;
        if v == 0 {
            continue;
        }
        let token = pts[2 * i + usize::from(v < 0)];
        for _ in 0..v.unsigned_abs() {
            result = ar.add(PRIME31, result, token);
        }
    }
    assert!(weight <= 4 && weight % 2 == 0);
    result
}

struct HalfTable {
    by_point: HashMap<Point, Sig>,
    points: Vec<Point>,
    four_words: u64,
    distinct_nets: usize,
    group_collisions: u64,
}

impl HalfTable {
    fn build(ar: &mut Arithmetic, pts: &[Point]) -> Self {
        assert_eq!(pts.len(), 2 * B);
        let n = pts.len();
        let mut pairs = vec![None; n * n];
        for i in 0..n {
            for j in i..n {
                let q = ar.add(PRIME31, pts[i], pts[j]);
                pairs[i * n + j] = q;
                pairs[j * n + i] = q;
            }
        }
        let mut seen = HashSet::<Sig>::with_capacity(618_885);
        let mut by_point = HashMap::with_capacity(618_885);
        let mut points = Vec::with_capacity(618_885);
        let mut four_words = 0_u64;
        let mut group_collisions = 0_u64;
        for i in 0..n {
            for j in i..n {
                for k in j..n {
                    for l in k..n {
                        four_words += 1;
                        let q = ar.add(PRIME31, pairs[i * n + j], pairs[k * n + l]);
                        let net = signature([i, j, k, l]);
                        if !seen.insert(net) {
                            continue;
                        }
                        // All retained net signatures are checked at build time;
                        // no arbitrary collision is treated as a unique root.
                        assert_eq!(replay(ar, pts, &net), q);
                        match by_point.entry(q) {
                            Entry::Vacant(e) => {
                                e.insert(net);
                                points.push(q);
                            }
                            Entry::Occupied(_) => group_collisions += 1,
                        }
                    }
                }
            }
        }
        assert_eq!(four_words, 677_040);
        assert_eq!(seen.len(), 618_885);
        Self {
            by_point,
            points,
            four_words,
            distinct_nets: seen.len(),
            group_collisions,
        }
    }

    fn find_complement(
        &self,
        other: &HalfTable,
        ar: &mut Arithmetic,
        residual: Point,
        seed: &mut u64,
    ) -> (Option<(Sig, Sig)>, usize) {
        let len = self.points.len();
        let start = (splitmix64(seed) as usize) % len;
        for probe in 0..len {
            let left = self.points[(start + probe) % len];
            let needed = ar.add(PRIME31, residual, ar.neg(PRIME31, left));
            if let Some(&right_sig) = other.by_point.get(&needed) {
                let &left_sig = self.by_point.get(&left).expect("table key missing");
                return (Some((left_sig, right_sig)), probe + 1);
            }
        }
        (None, len)
    }
}

fn mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % PRIME31.n as u128) as u64
}
fn add(x: u64, y: u64) -> u64 {
    (x + y) % PRIME31.n
}
fn sub(x: u64, y: u64) -> u64 {
    (x + PRIME31.n - y) % PRIME31.n
}
fn pow(mut x: u64, mut e: u64) -> u64 {
    let mut out = 1;
    while e > 0 {
        if e & 1 != 0 {
            out = mul(out, x);
        }
        x = mul(x, x);
        e >>= 1;
    }
    out
}
fn signed_mod(v: i8) -> u64 {
    if v < 0 {
        PRIME31.n - v.unsigned_abs() as u64
    } else {
        v as u64
    }
}

struct Rank {
    pivots: Vec<Option<Vec<u64>>>,
    rank: usize,
    dependent: usize,
}
impl Rank {
    fn new(cols: usize) -> Self {
        Self {
            pivots: vec![None; cols],
            rank: 0,
            dependent: 0,
        }
    }
    fn insert(&mut self, mut row: Vec<u64>) -> bool {
        let cols = self.pivots.len();
        assert_eq!(row.len(), cols + 1);
        for i in 0..cols {
            if row[i] == 0 {
                continue;
            }
            if let Some(ref pivot) = self.pivots[i] {
                let factor = row[i];
                for j in i..=cols {
                    row[j] = sub(row[j], mul(factor, pivot[j]));
                }
            } else {
                let inv = pow(row[i], PRIME31.n - 2);
                for x in &mut row[i..] {
                    *x = mul(*x, inv);
                }
                self.pivots[i] = Some(row);
                self.rank += 1;
                return true;
            }
        }
        assert_eq!(row[cols], 0, "inconsistent original-curve relation system");
        self.dependent += 1;
        false
    }
    fn solve(&self) -> Option<Vec<u64>> {
        let cols = self.pivots.len();
        if self.rank != cols {
            return None;
        }
        let mut out = vec![0; cols];
        for i in (0..cols).rev() {
            let row = self.pivots[i].as_ref().expect("complete rank missing pivot");
            let mut rhs = row[cols];
            for j in (i + 1)..cols {
                rhs = sub(rhs, mul(row[j], out[j]));
            }
            out[i] = rhs;
        }
        Some(out)
    }
}

fn unify_bases(a: &[(u64, u64)], b: &[(u64, u64)])
    -> (Vec<(u64, u64)>, Vec<usize>, Vec<usize>, usize)
{
    let mut points = Vec::new();
    let mut lookup = HashMap::new();
    let mut aidx = Vec::new();
    let mut bidx = Vec::new();
    for (which, base) in [a, b].into_iter().enumerate() {
        for &point in base {
            let idx = *lookup.entry(point).or_insert_with(|| {
                let i = points.len();
                points.push(point);
                i
            });
            if which == 0 {
                aidx.push(idx);
            } else {
                bidx.push(idx);
            }
        }
    }
    let overlap = a.len() + b.len() - points.len();
    (points, aidx, bidx, overlap)
}

fn signed_row(a: &Sig, b: &Sig, aidx: &[usize], bidx: &[usize],
    nvars: usize, label_a: u64, label_b: u64) -> Vec<u64>
{
    let mut row = vec![0; nvars + 1];
    for (s, ids) in [(a, aidx), (b, bidx)] {
        for (i, &coeff) in s.iter().enumerate() {
            row[ids[i]] = add(row[ids[i]], signed_mod(coeff));
        }
    }
    row[nvars - 1] = PRIME31.n - label_b;
    row[nvars] = label_a;
    row
}

fn solve_public_q(q: Point, case: usize) -> Value {
    let start = Instant::now();
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("curve + full prime order");
    assert!(q.is_some() && ar.on_curve(PRIME31, q));
    let first = first_rational_points(&mut ar);
    let special = Base::build(PRIME31, 6, &mut ar).expect("Dickson base");
    assert_eq!(special.points.len(), B);
    let (points, aidx, bidx, overlap) = unify_bases(&first, &special.points);
    assert_eq!(overlap, 1, "explicitly detect changing common-point geometry");
    let nvars = points.len() + 1;
    let apt = signed_points(&ar, &first);
    let bpt = signed_points(&ar, &special.points);
    let a_table = HalfTable::build(&mut ar, &apt);
    let b_table = HalfTable::build(&mut ar, &bpt);
    let table_elapsed = start.elapsed().as_nanos();
    let mut seed = RELATION_SEED ^ (case as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let mut rank = Rank::new(nvars);
    let mut attempts = Vec::new();
    let mut total_probes = 0_u64;
    let mut misses = 0_u64;
    for attempt in 0..MAX_RELATIONS {
        if rank.rank == nvars {
            break;
        }
        let label_a = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        let label_b = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        let ag = ar.scalar(PRIME31, label_a, Some(PRIME31.g));
        let bq = ar.scalar(PRIME31, label_b, q);
        let residual = ar.add(PRIME31, ag, bq);
        let (witness, probes) = a_table.find_complement(&b_table, &mut ar, residual, &mut seed);
        total_probes += probes as u64;
        if let Some((left, right)) = witness {
            // Genuine original-curve replay; no use of the caller's known k.
            let a_sum = replay(&mut ar, &apt, &left);
            let b_sum = replay(&mut ar, &bpt, &right);
            let lhs = ar.add(PRIME31, a_sum, b_sum);
            assert_eq!(lhs, residual);
            let row = signed_row(&left, &right, &aidx, &bidx, nvars, label_a, label_b);
            let independent = rank.insert(row.clone());
            attempts.push(json!({
                "attempt":attempt,"a":label_a,"b":label_b,
                "public_residual":residual,"probes":probes,
                "signed_first":left,"signed_dickson":right,
                "relation_row_mod_n":row,"independent":independent,
                "rank_after":rank.rank
            }));
        } else {
            misses += 1;
            attempts.push(json!({
                "attempt":attempt,"a":label_a,"b":label_b,
                "public_residual":residual,"probes":probes,
                "no_mixed_half_sum":true,"rank_after":rank.rank
            }));
        }
    }
    let solved = rank.solve();
    let mut replayed = 0;
    let mut target_log = None;
    if let Some(ref logs) = solved {
        for (i, &point) in points.iter().enumerate() {
            assert_eq!(ar.scalar(PRIME31, logs[i], Some(PRIME31.g)), Some(point),
                "recovered factor-base discrete log mismatch");
            replayed += 1;
        }
        let log_q = logs[nvars - 1];
        assert_eq!(ar.scalar(PRIME31, log_q, Some(PRIME31.g)), q,
            "complete unknown-target log mismatch");
        target_log = Some(log_q);
    }
    let elapsed = start.elapsed().as_nanos();
    json!({
        "case":case,"public_q":q,"status":if solved.is_some() {"complete"} else {"capped_incomplete"},
        "target_log":target_log,"factor_base_logs_independently_replayed":replayed,
        "distinct_basis_points":points.len(),"overlap_deduplicated":overlap,
        "required_rank":nvars,"final_rank":rank.rank,"dependent_rows":rank.dependent,
        "failed_relation_attempts":misses,"attempt_count":attempts.len(),
        "total_mixed_half_probes":total_probes,"cold_table_build_ns":table_elapsed.to_string(),
        "complete_cold_ns":elapsed.to_string(),"group_arithmetic_counters":ar.counts,
        "first_half_words":a_table.four_words,"dickson_half_words":b_table.four_words,
        "first_distinct_nets":a_table.distinct_nets,"dickson_distinct_nets":b_table.distinct_nets,
        "first_group_collisions":a_table.group_collisions,
        "dickson_group_collisions":b_table.group_collisions,
        "first_half_image":a_table.points.len(),"dickson_half_image":b_table.points.len(),
        "attempts":attempts
    })
}

fn main() {
    let mut target_seed = TARGET_SEED;
    let mut panels = Vec::new();
    for case in 0..TARGETS {
        // Caller-known scalar is deliberately outside solve_public_q().
        let k = 1 + splitmix64(&mut target_seed) % (PRIME31.n - 1);
        let mut caller_ar = Arithmetic::default();
        let q = caller_ar.scalar(PRIME31, k, Some(PRIME31.g));
        let panel = solve_public_q(q, case);
        let reported = panel["target_log"].as_u64();
        if let Some(got) = reported {
            assert_eq!(got, k, "caller-only scalar verification failed");
        }
        panels.push(json!({
            "caller_only_known_answer":k,
            "caller_comparison":reported.map(|v|v == k),
            "solver":panel
        }));
    }
    let solved = panels.iter().filter(|p|p["caller_comparison"]==true).count();
    println!("{}",json!({
        "schema":"prime31.mixed_base_4p4_complete_ic.v1",
        "curve":PRIME31,
        "method":"index-calculus-derived mixed-base 4+4 generic MITM; NOT nongeneric algebraic S9 extraction",
        "targets":TARGETS,"completed_unknown_target_logs":solved,
        "maximum_relation_attempts":MAX_RELATIONS,"target_seed":format!("{TARGET_SEED:#x}"),
        "relation_seed":format!("{RELATION_SEED:#x}"),
        "optimized_matched_rho_in_same_run":false,
        "independent_signed_PCC":false,
        "pcc":"unverified","panels":panels
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gaussian_recovery_and_dependent_rows() {
        let mut rank = Rank::new(2);
        assert!(rank.insert(vec![2, 0, 6]));
        assert!(!rank.insert(vec![4, 0, 12]));
        assert!(rank.insert(vec![1, 1, 8]));
        assert_eq!(rank.solve(), Some(vec![3, 5]));
        assert_eq!(rank.rank, 2);
        assert_eq!(rank.dependent, 1);
    }
    #[test]
    fn signed_replay_and_overlap() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        let first = first_rational_points(&mut ar);
        let special = Base::build(PRIME31, 6, &mut ar).unwrap();
        assert_eq!(special.points.len(), B);
        let (all, ai, bi, overlap) = unify_bases(&first, &special.points);
        assert_eq!(overlap, 1);
        assert_eq!(all.len(), 61);
        assert_eq!(ai.len(), B);
        assert_eq!(bi.len(), B);
        let tokens = signed_points(&ar, &first);
        let observed = replay(&mut ar, &tokens, &signature([0, 1, 2, 3]));
        let expected = ar.add(PRIME31, tokens[2], tokens[3]);
        assert_eq!(observed, expected);
    }
}
