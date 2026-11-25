//! Source-exact bounded 2-Dickson/2-Y7 algebraic branch gate.
//! Positive witness and target scalar are absent from the candidate input.

use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::{Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use prime31_four_summand::extractor::{extract_hybrid677_split_22, Limits};
use serde_json::json;
use std::time::Instant;

fn main() {
    let mut args = std::env::args().skip(1);
    let case = args.next().expect("case: positive|public");
    let matrix_cap: usize = args
        .next()
        .expect("matrix cap")
        .parse()
        .expect("integer cap");
    let pair_batch: usize = args
        .next()
        .expect("pair batch")
        .parse()
        .expect("integer batch");
    let monomial_cap: usize = args
        .next()
        .expect("monomial cap")
        .parse()
        .expect("integer monomial cap");
    assert!(args.next().is_none());
    assert!(matches!(case.as_str(), "positive" | "public"));
    assert!((2_000_000..=64_000_000).contains(&matrix_cap));
    assert!((1..=20_000).contains(&pair_batch));
    assert!((8_192..=65_536).contains(&monomial_cap));
    let q: Point = match case.as_str() {
        "positive" => Some((1_688_097_559, 1_788_262_902)),
        "public" => Some((976_979_610, 1_228_979_761)),
        _ => unreachable!(),
    };
    let cold = Instant::now();
    let fixture = Hybrid677::build().expect("frozen support");
    let fixture_ms = cold.elapsed().as_millis();
    let result = extract_hybrid677_split_22(
        &fixture,
        q,
        Limits {
            max_degree: 10,
            seconds: 30,
            max_polynomial_terms: 20_000,
            max_assignments: 4_096,
        },
        F4ResourceLimits {
            max_pairs: 20_000,
            max_symbolic_monomials: monomial_cap,
            max_matrix_entries: matrix_cap,
        },
        pair_batch,
    );
    println!(
        "{}",
        json!({
            "scope":"bounded 2+2 branch admission; no target solve or speed claim",
            "case":case,"curve":PRIME31,"public_query":q,
            "fixture_certificate":fixture.certificate(),
            "matrix_cap":matrix_cap,"pair_batch":pair_batch,
            "monomial_cap":monomial_cap,"fixture_ms":fixture_ms,
            "cold_wall_ms":cold.elapsed().as_millis(),
            "result":result,
        })
    );
}
