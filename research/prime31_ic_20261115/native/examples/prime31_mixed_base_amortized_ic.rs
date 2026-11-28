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


// EXP-R31-MIXED-AMORT: caller-independent, once-per-curve relation-base setup.
// Only the existing generic 4+4 index-calculus half-table oracle is used.
// All setup and all unsuccessful target descent attempts are charged.
// See MIXED_AMORTIZED_IC_PROTOCOL_20261117.md for frozen parameters.
const AMORT_TARGETS: usize = 1024;
const AMORT_PRECOMP_SEED: u64 = 0x5033_3142_4153_454c;
const AMORT_TARGET_SEED: u64 = 0x414d_4f52_5431_3032;
const AMORT_DESCENT_SEED: u64 = 0x4445_5343_454e_5431;
const AMORT_MAX_PRECOMP: usize = 256;
const AMORT_MAX_DESCENT: usize = 8;

struct ReusableBasis {
    ar: Arithmetic,
    table_a: HalfTable,
    table_d: HalfTable,
    signed_a: Vec<Point>,
    signed_d: Vec<Point>,
    basis: Vec<(u64, u64)>,
    a_indices: Vec<usize>,
    d_indices: Vec<usize>,
    logs: Option<Vec<u64>>,
    rank: usize,
    dependent: usize,
    misses: u64,
    probes: u64,
    setup_ns: u128,
    setup_attempts: Vec<Value>,
    overlap: usize,
}

fn setup_reusable_basis() -> ReusableBasis {
    let started = Instant::now();
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("actual cofactor-one prime31 certificate");
    let first = first_rational_points(&mut ar);
    let dickson = Base::build(PRIME31, 6, &mut ar).expect("actual Dickson base");
    assert_eq!(first.len(), B);
    assert_eq!(dickson.points.len(), B);
    let (basis, ai, di, overlap) = unify_bases(&first, &dickson.points);
    assert_eq!(basis.len(), 61);
    assert_eq!(overlap, 1);
    let signed_a = signed_points(&ar, &first);
    let signed_d = signed_points(&ar, &dickson.points);
    let table_a = HalfTable::build(&mut ar, &signed_a);
    let table_d = HalfTable::build(&mut ar, &signed_d);
    assert_eq!(table_a.distinct_nets, 618_885);
    assert_eq!(table_d.distinct_nets, 618_885);
    assert_eq!(table_a.group_collisions, 0);
    assert_eq!(table_d.group_collisions, 0);

    // No Q is passed into setup. The 61 columns here contain only basis logs.
    let mut rank = Rank::new(basis.len());
    let mut seed = AMORT_PRECOMP_SEED;
    let mut attempts = Vec::new();
    let mut misses = 0u64;
    let mut total_probes = 0u64;
    for attempt in 0..AMORT_MAX_PRECOMP {
        if rank.rank == basis.len() { break; }
        let a = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        let residual = ar.scalar(PRIME31, a, Some(PRIME31.g));
        let (witness, probes) = table_a.find_complement(
            &table_d, &mut ar, residual, &mut seed,
        );
        total_probes += probes as u64;
        if let Some((left, right)) = witness {
            let left_sum = replay(&mut ar, &signed_a, &left);
            let right_sum = replay(&mut ar, &signed_d, &right);
            assert_eq!(ar.add(PRIME31, left_sum, right_sum), residual,
                "precomputed relation fails original curve");
            // Reuse the same native row builder; remove the Q column, which
            // is identically 0 in this target-independent setup system.
            let mut row = signed_row(&left, &right, &ai, &di, basis.len()+1, a, 0);
            // The original single-target helper assumes b>0 and encodes
            // -b as n-b. For target-independent b=0 this returns n, which is
            // zero in F_n but not in canonical [0,n) field representation.
            row[basis.len()] %= PRIME31.n;
            assert_eq!(row[basis.len()], 0);
            row.remove(basis.len());
            assert_eq!(row.len(), basis.len()+1);
            let independent = rank.insert(row.clone());
            attempts.push(json!({
                "attempt": attempt, "a": a, "b": 0,
                "residual": residual, "probes": probes,
                "signed_first": left, "signed_dickson": right,
                "equation": row, "independent": independent,
                "rank_after": rank.rank
            }));
        } else {
            misses += 1;
            attempts.push(json!({
                "attempt": attempt, "a": a, "b": 0,
                "residual": residual, "probes": probes,
                "no_mixed_half_sum": true, "rank_after": rank.rank
            }));
        }
    }
    let logs = rank.solve();
    if let Some(ref known) = logs {
        for (i, &point) in basis.iter().enumerate() {
            assert_eq!(
                ar.scalar(PRIME31, known[i], Some(PRIME31.g)), Some(point),
                "independent basis-log point replay failed at column {i}"
            );
        }
        for attempt in &attempts {
            if let Some(row) = attempt["equation"].as_array() {
                let mut dot = 0u64;
                for (i, x) in row.iter().take(basis.len()).enumerate() {
                    dot = add(dot, mul(x.as_u64().unwrap(), known[i]));
                }
                assert_eq!(dot, row[basis.len()].as_u64().unwrap(),
                    "recovered basis logs violate a recorded precomp relation");
            }
        }
    }
    let setup_ns = started.elapsed().as_nanos();
    ReusableBasis {
        ar, table_a, table_d, signed_a, signed_d, basis,
        a_indices: ai, d_indices: di, logs,
        rank: rank.rank, dependent: rank.dependent,
        misses, probes: total_probes,
        setup_ns, setup_attempts: attempts, overlap
    }
}

fn descend_unknown_q(prep: &mut ReusableBasis, q: Point, index: usize) -> Value {
    let started = Instant::now();
    assert!(q.is_some() && prep.ar.on_curve(PRIME31, q));
    let mut seed = AMORT_DESCENT_SEED
        ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    let mut attempts = Vec::new();
    let mut result = None;
    let logs = prep.logs.as_ref().expect("full basis needed before descent");
    for attempt in 0..AMORT_MAX_DESCENT {
        let a = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        let b = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        let ag = prep.ar.scalar(PRIME31, a, Some(PRIME31.g));
        let bq = prep.ar.scalar(PRIME31, b, q);
        let residual = prep.ar.add(PRIME31, ag, bq);
        let (witness, probes) = prep.table_a.find_complement(
            &prep.table_d, &mut prep.ar, residual, &mut seed
        );
        if let Some((left, right)) = witness {
            let ls = replay(&mut prep.ar, &prep.signed_a, &left);
            let rs = replay(&mut prep.ar, &prep.signed_d, &right);
            assert_eq!(prep.ar.add(PRIME31, ls, rs), residual,
                "target relation fails original-curve replay");
            let row = signed_row(
                &left, &right, &prep.a_indices, &prep.d_indices,
                prep.basis.len()+1, a, b
            );
            assert_eq!(row.len(), prep.basis.len()+2);
            assert_eq!(row[prep.basis.len()], sub(0, b));
            assert_eq!(row[prep.basis.len()+1], a);
            let mut dot = 0u64;
            for (i, &value) in logs.iter().enumerate() {
                dot = add(dot, mul(row[i], value));
            }
            let log_q = mul(sub(dot, a), pow(b, PRIME31.n - 2));
            assert_eq!(prep.ar.scalar(PRIME31, log_q, Some(PRIME31.g)), q,
                "recovered unknown target logarithm fails original curve");
            assert_eq!(sub(dot, mul(b, log_q)), a,
                "signed relation modular replay failed");
            attempts.push(json!({
                "attempt": attempt, "a": a, "b": b,
                "residual": residual, "probes": probes,
                "signed_first": left, "signed_dickson": right,
                "equation": row, "independent_group_replay": true,
                "scalar_point_replay": true
            }));
            result = Some(log_q);
            break;
        } else {
            attempts.push(json!({
                "attempt": attempt, "a": a, "b": b,
                "residual": residual, "probes": probes,
                "no_mixed_half_sum": true
            }));
        }
    }
    let descent_ns = started.elapsed().as_nanos();
    json!({
        "case": index, "public_q": q,
        "status": if result.is_some() {"complete"} else {"capped_incomplete"},
        "target_log": result,
        "descent_ns": descent_ns.to_string(),
        "attempt_count": attempts.len(), "attempts": attempts
    })
}

fn run_amortized() -> Value {
    let mut prep = setup_reusable_basis();
    let mut target_seed = AMORT_TARGET_SEED;
    let mut unique = HashSet::new();
    let mut target_records = Vec::new();
    let mut descent_total = 0u128;
    let mut rejected_repeats = 0usize;
    let mut prefix_costs = Vec::new();

    if prep.logs.is_some() {
        for i in 0..AMORT_TARGETS {
            let k = loop {
                let v = 1 + splitmix64(&mut target_seed) % (PRIME31.n - 1);
                if unique.insert(v) { break v; }
                rejected_repeats += 1;
            };
            // The caller holds k; never pass it to descend_unknown_q.
            let mut caller = Arithmetic::default();
            let q = caller.scalar(PRIME31, k, Some(PRIME31.g));
            let answer = descend_unknown_q(&mut prep, q, i);
            let answer_k = answer["target_log"].as_u64();
            if let Some(actual) = answer_k {
                assert_eq!(actual, k, "caller-known-answer mismatch at target {i}");
            }
            descent_total += answer["descent_ns"].as_str().unwrap().parse::<u128>().unwrap();
            target_records.push(json!({
                "caller_only_scalar": k,
                "caller_check": answer_k.map(|v| v == k),
                "solver": answer
            }));
            if [1,64,256,512,1024].contains(&(i+1)) {
                prefix_costs.push(json!({
                    "number_of_targets": i+1,
                    "fully_charged_setup_plus_descent_ns":
                        (prep.setup_ns+descent_total).to_string(),
                    "complete_so_far": target_records.iter().filter(
                        |v| v["caller_check"]==true
                    ).count(),
                    "capped_or_failed_so_far": target_records.iter().filter(
                        |v| v["caller_check"]!=true
                    ).count()
                }));
            }
        }
    }
    let completed = target_records.iter().filter(|v| v["caller_check"]==true).count();
    json!({
        "schema": "prime31.mixed_4p4_amortized_complete_ic.v1",
        "method": "reusable index-calculus-derived generic 4+4 mixed-base table and 61-log relation matrix",
        "curve": PRIME31,
        "base_cardinality": prep.basis.len(),
        "common_point_overlap": prep.overlap,
        "setup_rank": prep.rank,
        "setup_rank_required": 61,
        "setup_full": prep.logs.is_some(),
        "base_log_point_replays": if prep.logs.is_some() {61} else {0},
        "setup_attempt_count": prep.setup_attempts.len(),
        "setup_dependent_relations": prep.dependent,
        "setup_missed_relations": prep.misses,
        "setup_probes": prep.probes,
        "setup_ns": prep.setup_ns.to_string(),
        "descent_total_ns": descent_total.to_string(),
        "fully_charged_1024_target_ns": (prep.setup_ns+descent_total).to_string(),
        "target_count": AMORT_TARGETS,
        "completed_unknown_target_logs": completed,
        "rejected_duplicate_caller_scalars": rejected_repeats,
        "target_relation_cap": AMORT_MAX_DESCENT,
        "precomp_relation_cap": AMORT_MAX_PRECOMP,
        "target_seed": format!("{AMORT_TARGET_SEED:#x}"),
        "setup_seed": format!("{AMORT_PRECOMP_SEED:#x}"),
        "descent_seed": format!("{AMORT_DESCENT_SEED:#x}"),
        "basis_logs": prep.logs,
        "setup_relations": prep.setup_attempts,
        "full_cold_prefix_costs": prefix_costs,
        "targets": target_records,
        "source_native_rust_only": true,
        "solver_used_hidden_scalar": false,
        "has_same_run_optimized_rho": false,
        "is_nongeneric_s9_extraction": false,
        "pcc": "unverified"
    })
}
fn main() { println!("{}", run_amortized()); }

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


#[cfg(test)]
mod amortized_tests {
    use super::*;
    #[test]
    fn complete_reusable_base_and_two_blind_unknown_targets() {
        let mut pre = setup_reusable_basis();
        assert!(pre.logs.is_some(), "basis precomputation failed to reach rank 61");
        assert_eq!(pre.rank, 61);
        assert_eq!(pre.basis.len(), 61);
        for (i,k) in [324_123u64, 1_112_233_445u64].iter().enumerate() {
            let mut caller = Arithmetic::default();
            let q = caller.scalar(PRIME31, *k, Some(PRIME31.g));
            let answer = descend_unknown_q(&mut pre, q, i);
            assert_eq!(answer["status"], "complete");
            assert_eq!(answer["target_log"], *k);
            assert_eq!(answer["attempt_count"], 1);
        }
    }
}
