//! Test-only source-data certificate for the proposed compact selector.
//!
//! Frozen scope: the original PRIME31 curve, depth-ten Dickson terminal
//! 1616683971, the eight public (c, y0) pairs from
//! `examples/prime31_coset_xlift_f4.rs`, and the actual RUN66 extra CSV.
//! The historical cubic-factor example has a final-coset transcription error:
//! 1852955275 instead of 1852957275. The former caused the preserved dev07
//! seventh-power assertion failure; the latter is the RUN52 anchor constant.
//! The expected census is 1024 distinct Dickson abscissae, of which 509 have
//! nonzero rational lifts and 515 have nonsquare curve RHS, together with
//! eight disjoint degree-21 fibres and the exact canonical 677-point union.
//!
//! Each listed abscissa is checked against its defining polynomial by scalar
//! evaluation. Distinct roots attaining the polynomial degree certify the
//! complete root set: no field scan, tuple enumeration, polynomial expansion,
//! relation extraction, quotient circuit, solver, or timing is involved.
//! `Base::build` retains its original bounded public-seed selection and reverse
//! square-root fibre construction. This certificate adds no seed search.
//!
//! Original `Arithmetic::pow` and `Arithmetic::mul` are private. Exponent
//! checks therefore reuse the already-linked, unchanged production
//! `residual_walk::pow_mod`; curve RHS, square roots and point checks use the
//! original `Arithmetic`. No new arithmetic backend is introduced.
//!
//! The parent must bind this source and the included CSV in its frozen source
//! inventory before execution. Git blob identifiers in the receipt identify
//! the reviewed sources; they are not independently recomputed by this test.
//! Passing certifies selector setup only, not circuit or extraction success.

use crypto_lib::cryptanalysis::residual_walk::pow_mod;
use prime31_algebraic_ic::{Arithmetic, Base, PRIME31};
use std::collections::BTreeSet;

const DEPTH: usize = 10;
const TERMINAL: u64 = 1_616_683_971;
const DICKSON_DEGREE: usize = 1 << DEPTH;
const COSET_DEGREE: usize = 3 * 7;
const EXTRA_CSV: &str = include_str!("../../prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv");
const COSETS: [(u64, u64); 8] = [
    (1_970_273_222, 391_601_180),
    (1_011_118_577, 1_045_445_284),
    (784_868_900, 1_088_714_879),
    (1_902_095_001, 839_007_678),
    (131_394_216, 1_893_426_316),
    (49_444_078, 1_215_727_561),
    (53_346_144, 1_554_347_565),
    (1_852_957_275, 136_769_428),
];

#[test]
fn compact_selector_source_data_setup_certificate() {
    let e = PRIME31;
    assert_eq!(
        (e.p, e.a, e.b, e.n, e.g),
        (
            2_147_483_647,
            847_514_846,
            2_087_444_724,
            2_147_446_831,
            (625_037_715, 2_100_627_251)
        ),
        "the setup certificate is restricted to the frozen original curve"
    );
    assert_eq!(EXTRA_CSV.len(), 3456, "frozen CSV byte length changed");
    let mut arithmetic = Arithmetic::default();
    let base = Base::build(e, DEPTH, &mut arithmetic).expect("original Dickson base must build");
    assert_eq!(base.depth, DEPTH);
    assert_eq!(base.terminal, TERMINAL);
    assert_eq!(base.values.len(), DICKSON_DEGREE);
    let dickson_x: BTreeSet<_> = base.values.iter().copied().collect();
    assert_eq!(dickson_x.len(), DICKSON_DEGREE, "distinct Dickson roots");

    let mut rational_dickson = BTreeSet::new();
    let mut nonsquare_rhs = 0;
    let mut zero_rhs = 0;
    for &x in &dickson_x {
        assert!(x < e.p, "canonical Dickson abscissa");
        let mut terminal_value = x;
        for _ in 0..DEPTH {
            terminal_value = (pow_mod(terminal_value, 2, e.p) + e.p - 2) % e.p;
        }
        assert_eq!(terminal_value, TERMINAL, "Dickson polynomial root");
        let rhs = arithmetic.rhs(e, x);
        if rhs == 0 {
            zero_rhs += 1;
        } else if let Some(y) = arithmetic.sqrt(rhs, e.p) {
            assert_ne!(y, 0);
            let point = (x, y.min(e.p - y));
            assert!(arithmetic.on_curve(e, Some(point)));
            assert!(rational_dickson.insert(point));
        } else {
            assert_eq!(
                pow_mod(rhs, (e.p - 1) / 2, e.p),
                e.p - 1,
                "Euler certificate for a nonsquare curve RHS"
            );
            nonsquare_rhs += 1;
        }
    }
    assert_eq!(zero_rhs, 0, "no rational two-torsion abscissa in the fibre");
    assert_eq!(nonsquare_rhs, 515);
    assert_eq!(rational_dickson.len(), 509);
    assert_eq!(base.points.len(), 509);
    let mut original_dickson_points = base.points.clone();
    original_dickson_points.sort_unstable();
    assert_eq!(
        original_dickson_points,
        rational_dickson.iter().copied().collect::<Vec<_>>(),
        "independent filtering of all 1024 values equals the original base"
    );

    let squared_c = COSETS.map(|(c, y0)| {
        assert!(c > 0 && c < e.p && y0 > 0 && y0 < e.p);
        assert_eq!(pow_mod(y0, 7, e.p), c, "public seventh-power witness");
        pow_mod(c, 2, e.p)
    });
    assert_eq!(
        squared_c.iter().copied().collect::<BTreeSet<_>>().len(),
        COSETS.len(),
        "distinct squared coset constants"
    );
    let mut extra_x = BTreeSet::new();
    let mut extra_points = BTreeSet::new();
    let mut raw_extra_points = Vec::new();
    let mut coset_roots: [BTreeSet<u64>; 8] = std::array::from_fn(|_| BTreeSet::new());
    for (line_number, line) in EXTRA_CSV.lines().enumerate() {
        let (x_text, y_text) = line.split_once(',').expect("CSV has exactly two integers");
        let x: u64 = x_text.parse().expect("CSV x is an unsigned integer");
        let y: u64 = y_text.parse().expect("CSV y is an unsigned integer");
        assert!(x < e.p && y > 0 && y < e.p, "CSV row {line_number}");
        assert!(
            arithmetic.on_curve(e, Some((x, y))),
            "CSV row {line_number}"
        );
        assert!(extra_x.insert(x), "duplicate CSV abscissa");
        assert!(
            !dickson_x.contains(&x),
            "extra root overlaps the full Dickson fibre"
        );
        let rhs = arithmetic.rhs(e, x);
        assert_ne!(rhs, 0, "no zero curve RHS among the extra roots");
        let rhs_seventh = pow_mod(rhs, 7, e.p);
        let coset_index = squared_c
            .iter()
            .position(|&c_squared| rhs_seventh == c_squared)
            .expect("CSV root belongs to one of the eight frozen cosets");
        // Distinct squared constants make the matching coset unique.
        assert!(coset_roots[coset_index].insert(x));
        assert!(extra_points.insert((x, y.min(e.p - y))));
        raw_extra_points.push((x, y));
    }
    assert_eq!(raw_extra_points.len(), 168);
    assert_eq!(extra_x.len(), 168);
    assert_eq!(extra_points.len(), 168);
    for roots in &coset_roots {
        assert_eq!(roots.len(), COSET_DEGREE, "all 21 distinct roots per coset");
    }

    let selector_x: BTreeSet<_> = dickson_x.union(&extra_x).copied().collect();
    assert_eq!(
        selector_x.len(),
        DICKSON_DEGREE + COSETS.len() * COSET_DEGREE
    );
    let canonical_union: Vec<_> = rational_dickson.union(&extra_points).copied().collect();
    assert_eq!(canonical_union.len(), 677);
    assert_eq!(
        canonical_union
            .iter()
            .map(|&(x, _)| x)
            .collect::<BTreeSet<_>>()
            .len(),
        677,
        "exactly one canonical point per base abscissa"
    );
    // Replay the current original_hybrid recipe directly from its two public
    // inputs, and compare it with the independently classified selector roots.
    let mut original_hybrid = base.points;
    original_hybrid.extend(raw_extra_points);
    for (_, y) in &mut original_hybrid {
        *y = (*y).min(e.p - *y);
    }
    original_hybrid.sort_unstable();
    original_hybrid.dedup();
    assert_eq!(
        original_hybrid, canonical_union,
        "exact canonical B677 union"
    );
    assert_eq!(canonical_union.len() + nonsquare_rhs, selector_x.len());

    let coset_receipts: Vec<_> = COSETS
        .iter()
        .zip(squared_c)
        .zip(&coset_roots)
        .map(|((&(c, y0), c_squared), roots)| {
            serde_json::json!({
                "c": c, "y0": y0, "c_squared": c_squared,
                "distinct_roots": roots.len()
            })
        })
        .collect();
    println!(
        "COMPACT_SELECTOR_SETUP {}",
        serde_json::json!({
            "scope": "source-data root-set setup only",
            "curve": e,
            "csv": {
                "path": "research/prime31_f4_admission_20261121/RUN66_Y7_EXTRA168.csv",
                "embedded_bytes": EXTRA_CSV.len(),
                "reviewed_git_blob": "b135333216ef7e72f4baa5da867167062a5daef9"
            },
            "reviewed_sources": {
                "original_native_git_blob": "42e1e7c1912be43190959c5be8f84cf59c867786",
                "coset_constants_git_blob": "9d52fd068c66e2c9b1d8f2ed7152ae6099cc1c54"
            },
            "dickson": {
                "depth": DEPTH, "terminal": TERMINAL,
                "distinct_roots": dickson_x.len(),
                "nonzero_rational_lifts": rational_dickson.len(),
                "nonsquare_rhs": nonsquare_rhs, "zero_rhs": zero_rhs
            },
            "cosets": coset_receipts,
            "extra_distinct_roots": extra_x.len(),
            "selector_distinct_roots": selector_x.len(),
            "canonical_base_points": canonical_union.len(),
            "extra_dickson_overlap": 0,
            "canonical_union_equals_original_hybrid": true,
            "circuit_evaluated": false,
            "relation_extraction_performed": false,
            "performance_measured": false
        })
    );
}
