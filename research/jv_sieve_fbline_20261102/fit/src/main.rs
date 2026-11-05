//! Sieve-route factor-base line study: the n-selection rule (`rule`) and the
//! paired analysis (`analyze`).  usage:
//!   sievefb_fit rule <v2_raw_dir>
//!   sievefb_fit analyze <v2_raw_dir> <results_dir> <out_dir>
mod analyze;
mod rule;

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("rule") => rule::run(&a[2]),
        Some("analyze") => analyze::run(&a[2], &a[3], &a[4]),
        _ => eprintln!("usage: sievefb_fit rule <v2_raw> | analyze <v2_raw> <results> <out>"),
    }
}
