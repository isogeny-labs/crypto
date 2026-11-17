//! PRIME31 B8 RR9 sign-enumeration comparator: exact generic 4+4 meet-in-the-middle.
//! This is NOT a B31 index-calculus relation extractor and NOT an ECDLP solver.
//! Frozen protocol: RR9_B8_GENERIC_MITM_PROTOCOL_20261117.md.
//! Reuses the original prime31_algebraic_ic::Arithmetic group law throughout.
use prime31_algebraic_ic::{check_curve, Arithmetic, Point, PRIME31};
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[path = "prime31_rr9_public_macaulay.rs"]
mod parent;

const SEED: u64 = 0x4238_4d49_544d_3236;
const BLIND_TARGETS: usize = 1024;

fn first_eight(ar: &mut Arithmetic) -> [(u64, u64); 8] {
    let mut base = Vec::new();
    for x in 0..PRIME31.p {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, PRIME31.p) {
            assert_ne!(y, 0, "cofactor-one prime group has no rational two-torsion");
            base.push((x, y.min(PRIME31.p - y)));
            if base.len() == 8 { break; }
        }
    }
    let out: [(u64,u64);8] = base.try_into().expect("eight rational abscissas");
    assert_eq!(out.map(|p| p.0), [0,1,2,3,5,6,8,9]);
    out
}

fn signed_point(ar: &Arithmetic, base: &[(u64,u64);8], i: usize, bit: bool) -> Point {
    let q = Some(base[i]);
    if bit { ar.neg(PRIME31, q) } else { q }
}

fn replay(ar: &mut Arithmetic, base: &[(u64,u64);8], mask: u8) -> Point {
    let mut q = None;
    for i in 0..8 {
        q = ar.add(PRIME31, q, signed_point(ar, base, i, (mask & (1u8 << i)) != 0));
    }
    q
}

fn four_sum(ar: &mut Arithmetic, base: &[(u64,u64);8], start: usize, mask: u8) -> Point {
    let mut q = None;
    for j in 0..4 {
        q = ar.add(PRIME31, q, signed_point(ar, base, start+j,
            (mask & (1u8 << j)) != 0));
    }
    q
}

struct Table {
    left: Vec<(Point,u8)>,
    right: HashMap<Point,u8>,
}

impl Table {
    fn build(ar: &mut Arithmetic, base: &[(u64,u64);8]) -> Self {
        let mut left = Vec::with_capacity(16);
        let mut right = HashMap::with_capacity(16);
        for mask in 0..16u8 {
            left.push((four_sum(ar, base, 0, mask), mask));
            right.entry(four_sum(ar, base, 4, mask)).or_insert(mask);
        }
        assert_eq!(left.len(), 16);
        // A collision is allowed and represented by the first valid witness.
        Self { left, right }
    }

    /// Only receives public R and precomputed public half sums; never caller k.
    fn query(&self, ar: &mut Arithmetic, r: Point) -> (Option<u8>, usize) {
        for (index, &(l, lm)) in self.left.iter().enumerate() {
            let complement = ar.add(PRIME31, r, ar.neg(PRIME31, l));
            if let Some(&rm) = self.right.get(&complement) {
                return (Some(lm | (rm << 4)), index+1);
            }
        }
        (None, self.left.len())
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn fixed_case(ar: &mut Arithmetic, base: &[(u64,u64);8],
              table: &Table, q: Point, label: &str, expect: bool) -> Value {
    let before = ar.counts.group_additions;
    let start = Instant::now();
    let (mask, probes) = table.query(ar, q);
    let ns = start.elapsed().as_nanos();
    let additions = ar.counts.group_additions - before;
    assert_eq!(mask.is_some(), expect, "fixture generic membership");
    if let Some(m) = mask {
        assert_eq!(replay(ar, base, m), q, "independent original-curve witness");
    }
    json!({"label":label,"r":q,"found":mask.is_some(),"mask":mask,
           "probes":probes,"query_wall_ns":ns.to_string(),
           "query_group_additions":additions,"witness_replayed":mask.is_some()})
}

fn run() -> Value {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("original ordinary prime31 curve");
    let base = first_eight(&mut ar);
    let before_setup = ar.counts.group_additions;
    let setup_start = Instant::now();
    let table = Table::build(&mut ar, &base);
    let setup_ns = setup_start.elapsed().as_nanos();
    let setup_additions = ar.counts.group_additions - before_setup;

    let positive = Some((1_911_579_686,548_820_096));
    let negative1 = Some((625_037_715,2_100_627_251));
    let negative2 = Some((1_923_055_775,307_731_458));
    let controls = vec![
        fixed_case(&mut ar,&base,&table,positive,"fixed_positive",true),
        fixed_case(&mut ar,&base,&table,negative1,"fixed_off_support_1",false),
        fixed_case(&mut ar,&base,&table,negative2,"fixed_off_support_2",false),
    ];

    // Exhaustively verify every one of 256 planted signed eight-leaf cases.
    let mut distinct = HashSet::new();
    let mut planted_probes = 0usize;
    for mask in 0..=255u8 {
        let q = replay(&mut ar, &base, mask);
        distinct.insert(q);
        let (witness, probes) = table.query(&mut ar,q);
        planted_probes += probes;
        let found = witness.expect("all signed supports must be found");
        assert_eq!(replay(&mut ar,&base,found),q,
                   "4+4 returned a false signed witness");
    }

    // Test-only scalar generation is outside query timing and no scalar is
    // supplied to Table::query; the candidate uses only public group points.
    let mut seed = SEED;
    let mut seen = HashSet::with_capacity(BLIND_TARGETS);
    let mut qs = Vec::with_capacity(BLIND_TARGETS);
    while qs.len() < BLIND_TARGETS {
        let k = 1 + splitmix64(&mut seed) % (PRIME31.n - 1);
        if seen.insert(k) {
            qs.push(ar.scalar(PRIME31,k,Some(PRIME31.g)));
        }
    }
    let before_online = ar.counts.group_additions;
    let start_online = Instant::now();
    let mut results = Vec::with_capacity(BLIND_TARGETS);
    let mut probes = 0usize;
    for &q in &qs {
        let result = table.query(&mut ar, q);
        probes += result.1;
        results.push(result.0);
    }
    let online_ns = start_online.elapsed().as_nanos();
    let online_additions = ar.counts.group_additions-before_online;
    let mut hits = 0usize;
    for (&q, mask) in qs.iter().zip(results) {
        if let Some(mask) = mask {
            hits += 1;
            assert_eq!(replay(&mut ar,&base,mask),q,
                       "blinded public-Q generic false positive");
        }
    }

    // Same-run existing RR9 control; its reported per-case times are its
    // own 256-mask sign-enumerative chart phase, NOT full ECDLP costs.
    let charts = parent::public_b8_barycentric_controls();
    assert_eq!(charts["positive_residual"],json!(positive));
    assert_eq!(charts["positive_chart_count"],1);
    assert_eq!(charts["negatives"][0]["charts"],0);
    assert_eq!(charts["negatives"][1]["charts"],0);
    assert_eq!(charts["sign_masks_enumerated_per_residual"],256);

    json!({
        "schema":"prime31.rr9.b8-generic-mitm-control.v1",
        "p":PRIME31.p,"n":PRIME31.n,"curve_a":PRIME31.a,
        "curve_b":PRIME31.b,"base_x":base.map(|p|p.0),
        "algorithm":"exact generic 4+4 signed point-sum MITM, not index calculus",
        "right_table_distinct_points":table.right.len(),
        "left_table_entries":table.left.len(),
        "precompute_wall_ns":setup_ns.to_string(),
        "precompute_group_additions":setup_additions,
        "fixed_cases":controls,
        "all_256_masks_replayed":256,
        "all_mask_distinct_group_points":distinct.len(),
        "all_mask_total_probes":planted_probes,
        "blinded_public_targets":BLIND_TARGETS,
        "blinded_seed":format!("{SEED:#x}"),
        "blinded_distinct_secret_labels_test_only":seen.len(),
        "blinded_hits":hits,
        "blinded_total_probes":probes,
        "blinded_online_wall_ns":online_ns.to_string(),
        "blinded_online_group_additions":online_additions,
        "rr9_chart_fixed_positive":charts["positive_chart_count"],
        "rr9_chart_fixed_negative_1":charts["negatives"][0]["charts"],
        "rr9_chart_fixed_negative_2":charts["negatives"][1]["charts"],
        "rr9_chart_positive_256mask_wall_ns":charts["positive_search_wall_ns"],
        "rr9_chart_negative_1_256mask_wall_ns":charts["negatives"][0]["search_wall_ns"],
        "rr9_chart_negative_2_256mask_wall_ns":charts["negatives"][1]["search_wall_ns"],
        "reuses_original_native_rust":true,
        "public_only_query":true,
        "is_nongeneric_extraction":false,
        "complete_unknown_prime31_logs":0,
        "pcc":"unverified"
    })
}

fn main() { println!("{}",serde_json::to_string_pretty(&run()).unwrap()); }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn b8_exact_generic_and_rr9_control_panel() {
        let report=run();
        assert_eq!(report["schema"],"prime31.rr9.b8-generic-mitm-control.v1");
        assert_eq!(report["p"],2_147_483_647u64);
        assert_eq!(report["n"],2_147_446_831u64);
        assert_eq!(report["left_table_entries"],16);
        assert_eq!(report["all_256_masks_replayed"],256);
        assert_eq!(report["blinded_public_targets"],1024);
        assert_eq!(report["blinded_distinct_secret_labels_test_only"],1024);
        assert_eq!(report["fixed_cases"][0]["found"],true);
        assert_eq!(report["fixed_cases"][1]["found"],false);
        assert_eq!(report["fixed_cases"][2]["found"],false);
        assert_eq!(report["rr9_chart_fixed_positive"],1);
        assert_eq!(report["rr9_chart_fixed_negative_1"],0);
        assert_eq!(report["rr9_chart_fixed_negative_2"],0);
        assert_eq!(report["complete_unknown_prime31_logs"],0);
    }
}
