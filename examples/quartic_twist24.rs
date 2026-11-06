//! Same-base three-arm quartic index-calculus experiment on canonical
//! quadratic twists; frozen protocol in QUARTIC_TWIST_LINE_PROTOCOL.md.
use crypto_lib::cryptanalysis::quartic_folds::run_k4_twist24;

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
    let mut primes: Vec<u64> = PANEL.iter().map(|&(p, _)| p).collect();
    let mut seeds = 3u64;
    let mut first_seed = 0u64;
    let mut rho_runs = 16usize;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--primes" => {
                i += 1;
                primes = args[i]
                    .split(',')
                    .map(|s| s.parse().expect("prime"))
                    .collect();
            }
            "--seeds" => {
                i += 1;
                seeds = args[i].parse().expect("seeds");
            }
            "--first-seed" => {
                i += 1;
                first_seed = args[i].parse().expect("first seed");
            }
            "--rho-runs" => {
                i += 1;
                rho_runs = args[i].parse().expect("rho runs");
            }
            other => panic!("unknown option {other}"),
        }
        i += 1;
    }
    for p in primes {
        let b0 = PANEL
            .iter()
            .find(|&&(q, _)| q == p)
            .expect("unregistered prime")
            .1;
        for seed in first_seed..first_seed + seeds {
            let record = run_k4_twist24(p, b0, seed, rho_runs)
                .unwrap_or_else(|e| panic!("p={p} seed={seed}: {e}"));
            println!(
                "{}",
                serde_json::to_string(&record).expect("serialize result")
            );
            eprintln!(
                "p={} seed={} flat={} cubic={} twist24={} residuals={}",
                p,
                seed,
                record.flat.solved,
                record.cubic.solved,
                record.twist24.solved,
                record.residuals
            );
        }
    }
}
