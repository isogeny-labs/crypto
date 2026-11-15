//! Fixed matched-input timing driver. --check never opens a timing window.
//! --measure requires a source-pinned build and an externally admitted L2 host.
use orientation_baseline::{pair_scan, subtract, subtract_batched, Counts, Curve, Point, Relation};
use std::{hint::black_box, time::Instant};

type Arm = fn(Curve, &[Point], Point) -> (Vec<Relation>, Counts);
const ARMS: [(&str, Arm); 4] = [
    ("pair-scan", pair_scan),
    ("scalar", subtract),
    ("batch", subtract_batched),
    ("scalar-aa", subtract),
];
const REPETITIONS: usize = 16;
const ROUNDS: usize = 5;

fn execute(arm: Arm, curve: Curve, base: &[Point], cases: &[(Point, Vec<Relation>)]) -> u64 {
    let mut rows = 0;
    for _ in 0..REPETITIONS {
        for (target, expected) in cases {
            let (actual, counts) = arm(black_box(curve), black_box(base), black_box(*target));
            assert_eq!(&actual, expected, "complete relation output mismatch");
            rows += black_box(actual.len()) as u64;
            black_box(counts);
        }
    }
    black_box(rows)
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let measure = match args.as_slice() {
        [mode] if mode == "--check" => false,
        [mode] if mode == "--measure" => true,
        _ => {
            eprintln!("usage: orientation-baseline-bench --check | --measure");
            std::process::exit(2);
        }
    };
    if measure {
        let source = option_env!("BASELINE_SOURCE_REVISION")
            .expect("build with BASELINE_SOURCE_REVISION set to the frozen source commit");
        assert!(source.len() == 40 && source.bytes().all(|b| b.is_ascii_hexdigit()));
        println!("{{\"schema\":\"orientation-baseline-timing/v1\",\"source_revision\":\"{source}\",\"rounds\":{ROUNDS},\"warmup\":1,\"repetitions\":{REPETITIONS},\"l2_admission\":\"external-required\"}}");
    }
    let curve = Curve::new(8, 0, 1);
    let points = curve.points();
    let mut targets = vec![
        None,
        *points.iter().find(|p| matches!(p, Some((0, _)))).unwrap(),
    ];
    targets.extend(points.iter().skip(2).step_by(37).take(6).copied());
    assert_eq!(targets.len(), 8);
    for size in [8, 16, 32, 64] {
        let base = &points[..size];
        let cases: Vec<_> = targets
            .iter()
            .map(|&t| (t, pair_scan(curve, base, t).0))
            .collect();
        // Exact-output preflight on every arm; no timers in --check mode.
        for (_, arm) in ARMS {
            for (t, expected) in &cases {
                assert_eq!(arm(curve, base, *t).0, *expected);
            }
        }
        if !measure {
            println!("{{\"mode\":\"check\",\"base_size\":{size},\"targets\":8,\"arms\":4,\"complete_outputs_equal\":true}}");
            continue;
        }
        // One complete warmup per arm, followed by five alternating rounds.
        for (_, arm) in ARMS {
            execute(arm, curve, base, &cases);
        }
        for round in 0..ROUNDS {
            let order: Vec<_> = if round % 2 == 0 {
                (0..4).collect()
            } else {
                (0..4).rev().collect()
            };
            for index in order {
                let (name, arm) = ARMS[index];
                let start = Instant::now();
                let rows = execute(arm, curve, base, &cases);
                let elapsed = start.elapsed().as_nanos();
                println!("{{\"base_size\":{size},\"round\":{round},\"arm\":\"{name}\",\"targets\":8,\"repetitions\":{REPETITIONS},\"rows\":{rows},\"elapsed_ns\":{elapsed}}}");
            }
        }
    }
}
