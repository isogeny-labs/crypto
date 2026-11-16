//! Paired L0 versus L0∪L2 quartic twist relation-yield panel.
//! Protocol: research/twist_index_calculus_20261020/QUARTIC_TWIST_LINE_PROTOCOL.md.
use crypto_lib::cryptanalysis::quartic_folds::run_k4_twist24_line;

fn main() {
    const PANEL: [(u64, u64); 7] = [
        (37, 2),
        (73, 5),
        (97, 5),
        (157, 25),
        (229, 6),
        (241, 49),
        (337, 100),
    ];
    let args: Vec<String> = std::env::args().skip(1).collect();
    let primes: Vec<u64> = if args.len() >= 2 && args[0] == "--primes" {
        args[1]
            .split(',')
            .map(|s| s.parse().expect("prime"))
            .collect()
    } else if args.is_empty() {
        PANEL.iter().map(|&(p, _)| p).collect()
    } else {
        panic!("usage: quartic_twist_mixed_lines [--primes p,q,...]")
    };
    for p in primes {
        let b0 = PANEL.iter().find(|&&(q, _)| q == p).expect("panel prime").1;
        for seed in 0..3 {
            for mixed in [false, true] {
                let record = run_k4_twist24_line(p, b0, seed, mixed)
                    .unwrap_or_else(|e| panic!("p={p} seed={seed} mixed={mixed}: {e}"));
                assert!(record.flat.correct && record.cubic.correct && record.twist24.correct);
                println!("{}", serde_json::to_string(&record).expect("serialize"));
                eprintln!(
                    "p={p} seed={seed} mixed={mixed} base={} residuals={} hits={}",
                    record.base_points, record.residuals, record.targets_decomposed
                );
            }
        }
    }
}
