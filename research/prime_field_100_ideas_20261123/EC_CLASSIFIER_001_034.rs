// Frozen secondary analysis of exact EC(F_1009) signed-point hit counts.
// Ideas 27/31: cheap bit-feature prerequisite only, no neural model.
use std::{env, fs};

const XS: [u32; 8] = [42, 43, 44, 45, 48, 49, 54, 55];
const HITS: [[u32; 4]; 8] = [
    [6, 4, 6, 8], [0, 0, 6, 0], [4, 10, 4, 6], [0, 0, 4, 5],
    [2, 6, 6, 4], [4, 6, 8, 12], [17, 9, 13, 12], [4, 2, 4, 2],
];
const THRESHOLD: u32 = 6;

fn parse_rows(raw: &str) -> Vec<(u32, [u32; 4])> {
    let mut out = Vec::new();
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("lead29_target index=") {
            let nums: Vec<u32> = rest
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().expect("integer in target row"))
                .collect();
            assert_eq!(nums.len(), 6, "malformed target row");
            assert_eq!(nums[0] as usize, out.len() + 1, "target index order");
            out.push((nums[1], [nums[2], nums[3], nums[4], nums[5]]));
        }
    }
    assert_eq!(out.len(), 8, "exactly eight target rows required");
    for i in 0..8 {
        assert_eq!(out[i].0, XS[i], "target x changed");
        assert_eq!(out[i].1, HITS[i], "point-relation hit table changed");
    }
    out
}

fn best_block(hits: &[u32; 4]) -> usize {
    let mut best = 0;
    for b in 1..4 {
        if hits[b] > hits[best] { best = b; }
    }
    best
}

fn main() {
    let path = env::args().nth(1).expect("fixed raw path argument");
    assert!(env::args().nth(2).is_none(), "one input only");
    let raw = fs::read_to_string(path).expect("read frozen EC raw output");
    let rows = parse_rows(&raw);

    let mut train_pos = 0usize;
    for row in rows.iter().take(4) {
        for &h in &row.1 { train_pos += (h >= THRESHOLD) as usize; }
    }
    let constant_positive = train_pos > 8; // tie resolves negative.
    let mut train_sum = [0u32; 4];
    for row in rows.iter().take(4) {
        for (b, total) in train_sum.iter_mut().enumerate() { *total += row.1[b]; }
    }
    let train_best = best_block(&train_sum);
    let mut confusion = [0u32; 4]; // TP, FP, FN, TN.
    let mut constant_confusion = [0u32; 4];
    let mut false_negative_hits = 0u32;
    let mut nn_total = 0u32;
    let mut trained_block_total = 0u32;
    let mut static_total = 0u32;

    for i in 4..8 {
        let (x, hold_hits) = rows[i];
        let mut nearest = 0usize;
        let mut distance = (x ^ rows[0].0).count_ones();
        for (j, train) in rows.iter().enumerate().take(4).skip(1) {
            let d = (x ^ train.0).count_ones();
            if d < distance { nearest = j; distance = d; }
        }
        let nn_block = best_block(&rows[nearest].1);
        nn_total += hold_hits[nn_block];
        trained_block_total += hold_hits[train_best];
        static_total += hold_hits[0];
        for b in 0..4 {
            let prediction = rows[nearest].1[b] >= THRESHOLD;
            let actual = hold_hits[b] >= THRESHOLD;
            let bucket = match (prediction, actual) {
                (true, true) => 0, (true, false) => 1,
                (false, true) => 2, (false, false) => 3,
            };
            confusion[bucket] += 1;
            if bucket == 2 { false_negative_hits += hold_hits[b]; }
            let baseline_bucket = match (constant_positive, actual) {
                (true, true) => 0, (true, false) => 1,
                (false, true) => 2, (false, false) => 3,
            };
            constant_confusion[baseline_bucket] += 1;
        }
        println!("holdout x={x} nearest_train_x={} hamming={} nn_block={} nn_hits={} train_best_block={} train_best_hits={} static_block=1 static_hits={}",
                 rows[nearest].0, distance, nn_block + 1, hold_hits[nn_block],
                 train_best + 1, hold_hits[train_best], hold_hits[0]);
    }

    assert_eq!(train_sum, [10, 14, 20, 19]);
    assert_eq!(train_best, 2);
    assert_eq!(confusion.iter().sum::<u32>(), 16);
    assert_eq!(constant_confusion.iter().sum::<u32>(), 16);
    println!("summary train_positive={train_pos}/16 threshold={THRESHOLD} constant_positive={constant_positive} train_block_hits={train_sum:?}");
    println!("classifier_nn_confusion_TP_FP_FN_TN={confusion:?} false_negative_exact_hits={false_negative_hits} constant_confusion_TP_FP_FN_TN={constant_confusion:?}");
    println!("one_block_holdout_hits nn={nn_total} train_best={trained_block_total} static_block1={static_total} candidate_patterns_per_arm=32768");
    println!("charged train_label_comparisons=131072 shared_group_sum_patterns=32768 nn_inference_xor_popcount=16 holdout_audit_label_comparisons=131072");
    println!("complete EC heldout bit-feature control");
}
