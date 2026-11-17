//! Count duplicate GLS target orbits in the recorded sequential relation
//! collection prefixes. These are exact combinatorial diagnostics, not reruns.
use std::collections::{BTreeMap, HashSet};
use std::fs;

fn number(s: &str, key: &str) -> u64 {
    let pattern = format!("\"{}\":", key);
    let tail = &s[s.find(&pattern).unwrap() + pattern.len()..];
    tail.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("NDJSON path");
    let raw = fs::read_to_string(path).unwrap();
    let mut curves = BTreeMap::new();
    for line in raw.lines().filter(|s| s.contains("\"kind\":\"curve\"")) {
        curves.insert(number(line, "p"), (number(line, "r"), number(line, "lambda")));
    }
    println!("p,b,arm,seed,trials,unique_psi_orbits,duplicate_psi_orbit_queries");
    for line in raw.lines().filter(|s| s.contains("\"kind\":\"solve\"")) {
        let p = number(line, "p");
        let (r, lam) = curves[&p];
        assert_eq!(lam * lam % r, r - 1);
        let trials = number(line, "trials");
        let mut seen = HashSet::new();
        for a in 1..=trials {
            let b = lam * a % r;
            let key = [a % r, b, (r - a % r) % r, (r - b) % r]
                .iter()
                .copied()
                .min()
                .unwrap();
            seen.insert(key);
        }
        let arm = if line.contains("\"arm\":\"signed-flat\"") {
            "flat"
        } else {
            "fold"
        };
        println!(
            "{},{},{},{},{},{},{}",
            p,
            number(line, "b"),
            arm,
            number(line, "seed"),
            trials,
            seen.len(),
            trials - seen.len() as u64
        );
    }
}
