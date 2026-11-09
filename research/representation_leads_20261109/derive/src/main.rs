//! Exact derivations for the representation leads of 2026-11-09 (no timings).
//!
//!   replead-derive counts   > ../results/counts.jsonl
//!   replead-derive census   > ../results/census.jsonl
//!   replead-derive powersum > ../results/powersum.jsonl

mod anf;
mod census;
#[cfg_attr(not(test), allow(dead_code))] // test-only helpers (group law checks)
mod curve;
#[cfg_attr(not(test), allow(dead_code))]
mod gf;
#[cfg_attr(not(test), allow(dead_code))]
mod powersum;
mod systems;
#[cfg(test)]
mod tests;

use curve::Curve;
use gf::{field, Rng};
use systems::{build, plant, system_stats, Rep};

const SEED: u64 = 0x2026_1109_0000_0001;

fn counts() {
    for &n in &[11u32, 13, 17, 19] {
        let f = field(n);
        for &(a, b) in &[(0u64, 1u64), (1, 1)] {
            let c = Curve { f, a, b };
            for &m in &[3usize, 4, 5] {
                for &l in &[3usize, 4, 5] {
                    let mut rng =
                        Rng(SEED ^ (n as u64) << 32 ^ (m as u64) << 16 ^ (l as u64) << 8 ^ a);
                    let p = plant(&c, l, m, &mut rng);
                    for rep in Rep::ALL {
                        let built = build(&c, rep, &p);
                        let s = system_stats(&built);
                        let ok = systems::holds(&built);
                        // negative control: flip one variable at a time (first 32), count detections
                        let mut detected = 0;
                        let flips = built.nvars.min(32);
                        for v in 0..flips {
                            let mut bits = built.planted.clone();
                            bits[v] = !bits[v];
                            if built
                                .eqs
                                .iter()
                                .any(|e| e.0.iter().any(|cc| cc.eval(&bits)))
                            {
                                detected += 1;
                            }
                        }
                        let by: Vec<String> = s
                            .terms_by_degree
                            .iter()
                            .map(|(d, k)| format!("\"{d}\":{k}"))
                            .collect();
                        println!(
                            "{{\"n\":{n},\"a\":{a},\"b\":{b},\"m\":{m},\"l\":{l},\"rep\":\"{}\",\"nvars\":{},\"neqs\":{},\"max_degree\":{},\"terms\":{},\"distinct_monomials\":{},\"terms_by_degree\":{{{}}},\"planted_holds\":{ok},\"flip_detected\":{detected},\"flips\":{flips}}}",
                            rep.name(), s.nvars, s.neqs, s.max_degree, s.terms, s.distinct_monomials, by.join(",")
                        );
                    }
                }
            }
        }
    }
}

fn census() {
    for &n in &[11u32, 13] {
        let f = field(n);
        let mut rng = Rng(SEED ^ 0xC0 ^ n as u64);
        let rb = rng.nonzero(&f);
        for &(a, b) in &[(0u64, 1u64), (1, 1), (0, rb), (1, rb)] {
            let c = Curve { f, a, b };
            let r = census::run(&c, 97);
            let cells: Vec<String> = r
                .cells
                .iter()
                .map(|((on, lam), k)| {
                    format!("{{\"on_curve\":{on},\"lambda\":{lam},\"count\":{k}}}")
                })
                .collect();
            println!(
                "{{\"n\":{n},\"a\":{a},\"b\":{b},\"triples\":{},\"cells\":[{}],\"lift_checked\":{},\"lift_ok\":{}}}",
                r.triples, cells.join(","), r.lifted_checked, r.lifted_ok
            );
        }
    }
}

fn powersum() {
    for &n in &[13u32, 17] {
        let f = field(n);
        for draw in 0..3u64 {
            let mut rng = Rng(SEED ^ 0x50 ^ (n as u64) << 8 ^ draw);
            let (b, xr) = (rng.nonzero(&f), rng.nonzero(&f));
            for &k in &[2usize, 3] {
                let r = powersum::R { f: &f, k };
                let s = powersum::summation(&r, b, xr);
                let elem = powersum::in_elem_span(&r, &s);
                let j = powersum::min_j(&r, &s, 8);
                // negative control: x1 is not symmetric, so never in either span
                let neg = powersum::min_j(&r, &r.x(0), 2).is_none()
                    && !powersum::in_elem_span(&r, &r.x(0));
                println!(
                    "{{\"n\":{n},\"draw\":{draw},\"m\":{k},\"b\":{b},\"xr\":{xr},\"deg_S\":{},\"terms_S\":{},\"S_in_elem_span\":{elem},\"min_j_power_sum\":{},\"negative_control_ok\":{neg}}}",
                    r.deg(&s), s.0.len(), j.map(|v| v.to_string()).unwrap_or("null".into())
                );
            }
        }
    }
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("counts") => counts(),
        Some("census") => census(),
        Some("powersum") => powersum(),
        _ => eprintln!("usage: replead-derive counts|census|powersum"),
    }
}
