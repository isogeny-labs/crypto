//! pf-validator: the native execution path of the independent validator of
//! XP-20261029-packed-filter (TASK-20261029-654f73). A line-by-line port of
//! validation/code/*.py; it links neither ecbench nor the crypto library.

mod equiv;
mod j1;
mod j2;
mod j3;
mod j4;
mod py;
mod report;
mod s3;
mod scalars;
mod selftest;
mod v;

use std::path::PathBuf;

const USAGE: &str = "usage: pf-validator <command> [args]
  j1 <B> <OUT.json>                   J1 decision statistic (B = 32 or 24)
  xcheck <OUT.json>                   J1 method against the in-pod control-comparison.json
  j2 <OUT.json>                       J2 instrument checks
  j3 <OUT.json>                       J3 charged-work identity and bitop re-pricing
  j4 <OUT.json>                       J4 S' band and the 5a order
  scalars <OUT.json>                  [d]G = Q on every record, own curve arithmetic
  selftest <OUT.json>                 negative controls on the checkers
  s3check <OUT.json>                  S3 copy check (aws CLI, read-only)
  s3requests <S3CHECK.json> <OUT.json> launcher request.json check (aws CLI, read-only)
  c2 <OUT.json>                       RULING-P28 C2 identity facts (aws CLI, read-only)
  report <OUT_DIR> <REPLAY_DIR> <DEST_DIR>  validator-report.json and replay-certificate.json
  equivalence <NATIVE_OUT_DIR> <OUT.jsonl>  native outputs against the committed Python outputs
  warmup-seqs <records.jsonl>         print the seq of every warm-up record (replay.sh helper)
environment: PFV_ROOT overrides the study directory (default: two levels above the crate)";

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let p = |i: usize| -> PathBuf { PathBuf::from(a.get(i).unwrap_or_else(|| usage())) };
    match a.first().map(|s| s.as_str()) {
        Some("j1") => j1::run(a.get(1).unwrap_or_else(|| usage()).parse().unwrap_or_else(|_| usage()), &p(2)),
        Some("xcheck") => j1::run_xcheck(&p(1)),
        Some("j2") => j2::run(&p(1)),
        Some("j3") => j3::run(&p(1)),
        Some("j4") => j4::run(&p(1)),
        Some("scalars") => scalars::run(&p(1)),
        Some("selftest") => selftest::run(&p(1)),
        Some("s3check") => s3::run_s3check(&p(1)),
        Some("s3requests") => s3::run_s3requests(&p(1), &p(2)),
        Some("c2") => s3::run_c2(&p(1)),
        Some("report") => report::run(&p(1), &p(2), &p(3)),
        Some("equivalence") => {
            if !equiv::run(&p(1), &p(2)) {
                eprintln!("equivalence: FAIL");
                std::process::exit(1);
            }
        }
        Some("warmup-seqs") => {
            for line in v::read_text(&p(1)).lines() {
                let r: serde_json::Value = serde_json::from_str(line).unwrap();
                if py::truthy(&r["warmup"]) {
                    println!("{}", r["seq"]);
                }
            }
        }
        Some("lgamma") => {
            // diagnostic: CPython math.lgamma port
            for x in &a[1..] {
                println!("{}", py::float_repr(py::lgamma(x.parse().unwrap())));
            }
        }
        _ => usage(),
    }
}

fn usage() -> ! {
    eprintln!("{USAGE}");
    std::process::exit(64)
}
