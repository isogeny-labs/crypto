use prime31_algebraic_ic::{check_curve, Arithmetic, Curve, Point};
use prime31_four_summand::extractor::{extract, Limits, Status};
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

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let smoke = mode == "smoke";
    let wide = mode == "wide" || mode == "wide2";
    assert!(mode.is_empty() || smoke || wide, "unknown control mode");
    let mut check_ar = Arithmetic::default();
    check_curve(CURVE, &mut check_ar).expect("registered curve is valid");
    let base_size = if wide { 7 } else { 4 };
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
    let output = extract(
        CURVE,
        &points,
        query,
        Limits {
            max_degree: 10,
            seconds: 30,
            max_polynomial_terms: 20_000,
            max_assignments: 4096,
        },
    );
    let elapsed_ms = started.elapsed().as_millis();
    let partials: Vec<_> = output
        .relations
        .iter()
        .filter(|relation| {
            relation.terms.len() == 4
                && relation.terms.iter().filter(|(i, _)| *i < base_size).count() == 3
                && relation.terms.iter().filter(|(i, _)| *i >= base_size).count() == 1
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
