//! pgla-vtool: the validator's tooling for research/postgb_linear_stage_20261026.
//!
//!   vtool sample          --snapshot C --out round2/sample.json
//!   vtool verify-scalars  --snapshot C --out round2/scalar-verification.json
//!   vtool recompute       --snapshot C [--round1 C1] --out validator-report.json
//!   vtool check-replay    --snapshot C --out replay-certificate.json
//!
//! Every committed input is read with `git show C:path`.  The replays
//! themselves are launched by round2/run.sh (thin orchestration).

mod recompute;
mod replay;
mod scalars;
mod util;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("sample") => replay::sample(&args),
        Some("check-replay") => replay::check(&args),
        Some("verify-scalars") => scalars::main(&args),
        Some("recompute") => recompute::main(&args),
        _ => {
            eprintln!("usage: vtool sample|verify-scalars|recompute|check-replay --snapshot C --out FILE");
            std::process::exit(2);
        }
    }
}
