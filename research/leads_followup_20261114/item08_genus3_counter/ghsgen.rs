// Generates GHS instances (unmodified Inst::search of N4) for small q = 7 mod 12; jsonl like instances.jsonl.
// Build: rustc --edition 2021 -O ghsgen.rs
#![allow(dead_code, unused_imports, unused_variables, unused_mut, static_mut_refs)]
#[path = "../../quartic_n4_20261019/common.rs"]
mod common;
#[path = "../../quartic_n4_20261019/ghs.rs"]
mod ghs;
#[path = "../../../src/cryptanalysis/plane_quartic_pic.rs"]
mod pic;
use ghs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let q: u64 = a[1].parse().unwrap();
    for seed in 1..=a[2].parse::<u64>().unwrap() {
        match Inst::search(q, seed, 200000) {
            Some(i) => println!("{{\"q\":{},\"seed\":{},\"ell\":{},\"order\":{},\"tries\":{},\"f\":[{}]}}", q, seed, i.ell, i.order, i.tries, i.f.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),
            None => eprintln!("no instance q={} seed={}", q, seed),
        }
    }
}
