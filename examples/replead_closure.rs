//! Closure (MutantXL) refutation degree of externally built Boolean systems:
//! the instrument binding of `research/representation_leads_20261109/PROTOCOL.md`.
//!
//! The systems come from that campaign's `derive/` crate
//! (`replead-derive export > results/systems.jsonl`), one JSON line per
//! (cell, draw, arm): `nvars`, `polys` (each a list of u64 monomial masks),
//! `satisfiable`, and for satisfiable draws the `planted` assignment.  Each
//! line is scanned with the same routines `examples/dreg_closure.rs` uses
//! ([`closure_refutation_scan`] dense, [`sparse_closure_refutes`] sparse).
//!
//! ```sh
//! cargo run --release --example replead_closure -- SYSTEMS.jsonl \
//!     --cell n11-m3-l3-a0 [--arm e2_aux] [--draw 3] [--from 2] [--to 6] \
//!     [--max-cols 70000] [--sparse D --max-entries N]
//! ```
//!
//! One JSON line per (draw, arm, degree) on stdout.  A degree whose dense
//! matrix would exceed `--max-cols` columns (`C(N,≤D)`) is not built and is
//! reported as unmeasured, never as a refutation.  A satisfiable draw whose
//! planted assignment does not zero every equation aborts the run (exporter
//! fault); a satisfiable draw that refutes is reported with
//! `"instrument_fault":true` and voids its cell (PROTOCOL.md).

use crypto_lib::cryptanalysis::koblitz_bench::{closure_refutation_scan, sparse_closure_refutes};
use crypto_lib::cryptanalysis::pq_groebner_f2::{mask_from64, F2BoolMono, F2BoolPoly};
use serde_json::Value;
use std::io::BufRead;

fn monomials_up_to(n_vars: u64, d: u32) -> f64 {
    let (mut total, mut binom) = (0f64, 1f64);
    for i in 0..=d as u64 {
        if i > n_vars {
            break;
        }
        total += binom;
        binom = binom * (n_vars - i) as f64 / (i + 1) as f64;
    }
    total
}

/// The system of one exported line, and its planted point when satisfiable.
fn parse(v: &Value) -> (Vec<F2BoolPoly>, usize, Option<u64>) {
    let n = v["nvars"].as_u64().expect("nvars") as usize;
    assert!(
        n <= 64,
        "{n} variables: the export format holds u64 masks only"
    );
    let system = v["polys"]
        .as_array()
        .expect("polys")
        .iter()
        .map(|p| {
            let monos = p
                .as_array()
                .expect("poly")
                .iter()
                .map(|m| F2BoolMono::from_mask(mask_from64(m.as_u64().expect("mask"))))
                .collect();
            F2BoolPoly::from_monos(monos, n)
        })
        .collect();
    let planted = v["satisfiable"].as_bool().unwrap_or(false).then(|| {
        let bits = v["planted"].as_str().expect("planted");
        assert_eq!(bits.len(), n, "planted assignment length");
        bits.bytes()
            .enumerate()
            .fold(0u64, |m, (i, b)| m | ((b == b'1') as u64) << i)
    });
    (system, n, planted)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let path = args
        .first()
        .expect("usage: replead_closure SYSTEMS.jsonl --cell CELL [...]");
    let cell = flag("--cell").expect("--cell is required (one cell per process)");
    let arm = flag("--arm");
    let draw: Option<u64> = flag("--draw").map(|v| v.parse().expect("--draw K"));
    let from: u32 = flag("--from").map_or(2, |v| v.parse().expect("--from D"));
    let to: u32 = flag("--to").map_or(6, |v| v.parse().expect("--to D"));
    let max_cols: f64 = flag("--max-cols").map_or(70_000.0, |v| v.parse().expect("--max-cols"));
    let sparse: Option<u32> = flag("--sparse").map(|v| v.parse().expect("--sparse D"));
    let max_entries: usize =
        flag("--max-entries").map_or(1_500_000_000, |v| v.parse().expect("--max-entries"));

    let file = std::fs::File::open(path).expect("open systems file");
    for line in std::io::BufReader::new(file).lines() {
        let v: Value = serde_json::from_str(&line.expect("read line")).expect("json line");
        if v["cell"] != cell.as_str()
            || arm.as_deref().is_some_and(|a| v["arm"] != a)
            || draw.is_some_and(|d| v["draw"].as_u64() != Some(d))
        {
            continue;
        }
        let (system, n, planted) = parse(&v);
        if let Some(pt) = planted {
            let bad = system
                .iter()
                .filter(|p| p.eval(mask_from64(pt)) != 0)
                .count();
            assert_eq!(
                bad, 0,
                "planted assignment fails {bad} equations: exporter fault"
            );
        }
        let head = format!(
            "\"cell\":{},\"draw\":{},\"arm\":{},\"satisfiable\":{},\"nvars\":{n},\"equations\":{}",
            v["cell"],
            v["draw"],
            v["arm"],
            planted.is_some(),
            system.len()
        );
        if let Some(d) = sparse {
            match sparse_closure_refutes(&system, n, d, max_entries) {
                Some(s) => println!(
                    "{{{head},\"engine\":\"sparse\",\"degree\":{},\"closure_refuted\":{},\"saturated\":{},\"rank\":{},\"falls\":{},\"products\":{},\"instrument_fault\":{}}}",
                    s.degree, s.refuted, s.saturated, s.rank, s.falls, s.products,
                    s.refuted && planted.is_some()
                ),
                None => println!("{{{head},\"engine\":\"sparse\",\"degree\":{d},\"measured\":false}}"),
            }
            continue;
        }
        for d in from..=to {
            let cols = monomials_up_to(n as u64, d);
            if cols > max_cols {
                println!("{{{head},\"engine\":\"dense\",\"degree\":{d},\"columns\":{cols},\"measured\":false}}");
                break;
            }
            let Some(r) = closure_refutation_scan(&system, n, d, d).into_iter().next() else {
                println!("{{{head},\"engine\":\"dense\",\"degree\":{d},\"measured\":false}}");
                break;
            };
            println!(
                "{{{head},\"engine\":\"dense\",\"degree\":{},\"measured\":true,\"xl_refuted\":{},\"closure_refuted\":{},\"saturated\":{},\"xl_rank\":{},\"closure_rank\":{},\"columns\":{},\"secs\":{:.3},\"instrument_fault\":{}}}",
                r.degree, r.xl_refuted, r.closure_refuted, r.saturated, r.xl_rank, r.closure_rank,
                r.columns, r.secs, (r.xl_refuted || r.closure_refuted) && planted.is_some()
            );
            if r.closure_refuted {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(polys: &str, n: usize, sat: bool, planted: &str) -> Value {
        serde_json::from_str(&format!(
            "{{\"cell\":\"t\",\"draw\":0,\"arm\":\"a\",\"satisfiable\":{sat},\"nvars\":{n},\"planted\":\"{planted}\",\"polys\":[{polys}]}}"
        ))
        .unwrap()
    }

    #[test]
    fn parses_masks_and_planted_point() {
        // x0 x1 + x2 = 0 and x0 + 1 = 0, satisfied by x0 = 1, x1 = 1, x2 = 1
        let (sys, n, pt) = parse(&line("[3,4],[0,1]", 3, true, "111"));
        assert_eq!((sys.len(), n), (2, 3));
        assert!(sys.iter().all(|p| p.eval(mask_from64(pt.unwrap())) == 0));
        assert!(sys.iter().any(|p| p.eval(mask_from64(0b011)) != 0));
    }

    #[test]
    fn contradictory_system_refutes_and_consistent_one_does_not() {
        // x0 = 0 and x0 = 1: refuted by degree 2
        let (bad, n, _) = parse(&line("[1],[0,1]", 2, false, ""));
        let r = closure_refutation_scan(&bad, n, 2, 2);
        assert!(r.iter().any(|d| d.xl_refuted || d.closure_refuted));
        // x0 x1 = 0 and x0 + x1 + 1 = 0 has solutions (0,1) and (1,0)
        let (ok, n, _) = parse(&line("[3],[0,1,2]", 2, false, ""));
        let r = closure_refutation_scan(&ok, n, 2, 3);
        assert!(r.iter().all(|d| !d.xl_refuted && !d.closure_refuted));
    }
}
