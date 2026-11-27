use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point, PRIME31};
use prime31_four_summand::b677_fixture::Hybrid677;
use prime31_four_summand::extractor::extract_hybrid677;
use prime31_four_summand::extractor::{extract, extract_cofactor, Limits, Status};
use serde_json::json;
use std::time::Instant;

const CURVE: Curve = Curve {
    p: 2_147_483_647,
    a: 2_147_483_644,
    b: 21,
    n: 2_147_487_637,
    g: (1_391_443_431, 177_979_044),
};

fn public_points(count: usize) -> Vec<(u64, u64)> {
    let mut ar = Arithmetic::default();
    let mut out = Vec::with_capacity(count);
    for x in 0..CURVE.p {
        let rhs = ar.rhs(CURVE, x);
        if let Some(y) = ar.sqrt(rhs, CURVE.p) {
            if y != 0 {
                out.push((x, y.min(CURVE.p - y)));
            }
        }
        if out.len() == count {
            return out;
        }
    }
    panic!("insufficient public curve points");
}

fn signed_sum(points: &[(u64, u64)], terms: &[(usize, i64)]) -> Point {
    let mut ar = Arithmetic::default();
    let mut sum = None;
    for &(i, sign) in terms {
        let mut point = Some(points[i]);
        if sign == -1 {
            point = ar.neg(CURVE, point);
        }
        sum = ar.add(CURVE, sum, point);
    }
    sum
}

fn bounded_b677(matrix_cap: usize) {
    let fixture = Hybrid677::build().expect("source-bound B677 fixture");
    let points = fixture.points();
    let planted = [(0, 1), (1, 1), (2, -1), (5, 1)];
    let mut ar = Arithmetic::default();
    let mut query = None;
    for (index, sign) in planted {
        let point = Some(points[index]);
        let signed = if sign == 1 {
            point
        } else {
            ar.neg(PRIME31, point)
        };
        query = ar.add(PRIME31, query, signed);
    }
    let started = Instant::now();
    let result = extract_hybrid677(
        &fixture,
        query,
        Limits {
            max_degree: 10,
            seconds: 30,
            max_polynomial_terms: 20_000,
            max_assignments: 4096,
        },
        F4ResourceLimits {
            max_pairs: 20_000,
            max_symbolic_monomials: 8_192,
            max_matrix_entries: matrix_cap,
        },
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "curve": PRIME31,
            "fixture_certificate": fixture.certificate(),
            "query": query,
            "control_planted_terms": planted,
            "status": format!("{:?}", result.status),
            "reason": result.reason,
            "relations": result.relations,
            "diagnostics": result.diagnostics,
            "traversal": result.traversal,
            "elapsed_ms": started.elapsed().as_millis(),
            "limits": {"max_pairs": 20_000, "max_symbolic_monomials": 8_192,
                       "max_matrix_entries": matrix_cap},
        }))
        .unwrap()
    );
    if !matches!(result.status, Status::Complete | Status::Empty) {
        std::process::exit(2);
    }
}

fn next_word(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn public_panel() {
    let points = public_points(8);
    let mut ar = Arithmetic::default();
    let q = Some((291_202_666, 1_848_813_168));
    assert!(ar.on_curve(CURVE, q));
    let mut seed = 0x7033_315f_7272_3501u64;
    let mut rows = Vec::new();
    for label in 0..16 {
        let a = next_word(&mut seed) % CURVE.n;
        let b = next_word(&mut seed) % CURVE.n;
        let ag = ar.scalar(CURVE, a, Some(CURVE.g));
        let bq = ar.scalar(CURVE, b, q);
        let r = ar.add(CURVE, ag, bq);
        assert!(r.is_some());
        assert!(!points.iter().any(|point| Some(*point) == r));
        let started = Instant::now();
        let result = extract(
            CURVE,
            &points,
            r,
            Limits {
                max_degree: 10,
                seconds: 30,
                max_polynomial_terms: 20_000,
                max_assignments: 4096,
            },
        );
        let partials: Vec<_> = result
            .relations
            .iter()
            .filter(|relation| {
                relation.terms.len() == 4
                    && relation.terms.iter().filter(|(i, _)| *i < 7).count() == 3
                    && relation.terms.iter().filter(|(i, _)| *i == 7).count() == 1
            })
            .collect();
        for relation in &partials {
            assert_eq!(signed_sum(&points, &relation.terms), r);
        }
        rows.push(json!({
            "label": label,
            "a": a,
            "b": b,
            "query": r,
            "status": format!("{:?}", result.status),
            "reason": result.reason,
            "elapsed_ms": started.elapsed().as_millis(),
            "one_lp_relations": partials,
            "all_relations": result.relations.len(),
            "diagnostics": result.diagnostics,
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "curve": {"p": CURVE.p, "a": CURVE.a, "b": CURVE.b, "n": CURVE.n, "g": CURVE.g},
            "public_target": q,
            "base": &points[..7],
            "large_prime_pool": &points[7..],
            "seed": "0x7033315f72723501",
            "queries": rows,
        }))
        .unwrap()
    );
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    if mode == "b677" || mode == "b67732" {
        bounded_b677(if mode == "b67732" {
            32_000_000
        } else {
            8_000_000
        });
        return;
    }
    if mode == "panel" {
        public_panel();
        return;
    }
    let smoke = mode == "smoke";
    let wide9 = mode == "wide9";
    let wide = mode == "wide" || mode == "wide2" || wide9;
    assert!(mode.is_empty() || smoke || wide, "unknown control mode");
    let mut check_ar = Arithmetic::default();
    check_curve(CURVE, &mut check_ar).expect("registered curve is valid");
    let base_size = if wide9 {
        8
    } else if wide {
        7
    } else {
        4
    };
    let points = public_points(base_size + 1);
    let planted = if mode == "wide2" {
        [(3, 1), (4, -1), (5, 1), (base_size, 1)]
    } else {
        [(0, 1), (1, -1), (2, 1), (base_size, 1)]
    };
    let query = if smoke {
        Some((291_202_666, 1_848_813_168))
    } else {
        signed_sum(&points, &planted)
    };
    assert!(query.is_some());
    assert!(!points.iter().any(|point| Some(*point) == query));
    let started = Instant::now();
    let limits = Limits {
        max_degree: 10,
        seconds: 30,
        max_polynomial_terms: 20_000,
        max_assignments: 4096,
    };
    let output = if wide9 {
        extract_cofactor(
            CURVE,
            &points,
            query,
            limits,
            F4ResourceLimits {
                max_pairs: 100_000,
                max_symbolic_monomials: 2_000_000,
                max_matrix_entries: 8_000_000,
            },
        )
    } else {
        extract(CURVE, &points, query, limits)
    };
    let elapsed_ms = started.elapsed().as_millis();
    let partials: Vec<_> = output
        .relations
        .iter()
        .filter(|relation| {
            relation.terms.len() == 4
                && relation
                    .terms
                    .iter()
                    .filter(|(i, _)| *i < base_size)
                    .count()
                    == 3
                && relation
                    .terms
                    .iter()
                    .filter(|(i, _)| *i >= base_size)
                    .count()
                    == 1
        })
        .collect();
    for relation in &partials {
        assert_eq!(signed_sum(&points, &relation.terms), query);
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "curve": {"p": CURVE.p, "a": CURVE.a, "b": CURVE.b, "n": CURVE.n, "g": CURVE.g},
            "base": &points[..base_size],
            "base_size": base_size,
            "large_prime_pool": &points[base_size..],
            "public_query": query,
            "control_planted_terms": if smoke { None } else { Some(planted) },
            "status": format!("{:?}", output.status),
            "reason": output.reason,
            "elapsed_ms": elapsed_ms,
            "all_replayed_relations": output.relations.len(),
            "one_lp_replayed_relations": partials.len(),
            "partials": partials,
            "diagnostics": output.diagnostics,
        }))
        .unwrap()
    );
    if output.status != Status::Complete && output.status != Status::Empty {
        std::process::exit(2);
    }
    if !smoke && partials.is_empty() {
        std::process::exit(2);
    }
}
