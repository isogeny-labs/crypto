//! p3census: cross-trace covariate census for IDEA-20261107-SEED-NEGATIVE-CONTROL (P3).
mod analyze;
mod cov;
mod curves;
mod ec;
mod poly;
mod rho;
mod selftest;
mod stats;
mod util;
mod yieldcov;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let sub = args.get(1).map(String::as_str).unwrap_or("");
    let rest = &args[2.min(args.len())..];
    match sub {
        "curves" => curves::cmd(rest),
        "covariates" => cov::cmd(rest),
        "rho" => rho::cmd(rest),
        "yield" => yieldcov::cmd(rest),
        "analyze" => analyze::cmd(rest),
        "selftest" => selftest::run(),
        _ => {
            eprintln!("usage: p3census <curves|covariates|rho|yield|analyze|selftest> [options]; see README.md");
            std::process::exit(2);
        }
    }
}
