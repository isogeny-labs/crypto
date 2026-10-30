use divisor_arithmetic::field::PrimeField;
use divisor_arithmetic::g2::ramified::not_char2::{add, double, CurveConstants, DivisorCoords};
use serde_json::json;

type F = PrimeField<31>;

fn coords(d: DivisorCoords<F>) -> [u64; 5] {
    [
        d.u2.value(),
        d.u1.value(),
        d.u0.value(),
        d.v1.value(),
        d.v0.value(),
    ]
}

fn main() {
    // y^2=x^5+2x^3+3x^2+4x+5 over F_31; (0,6) is a non-2-torsion point.
    assert_eq!(6u64 * 6 % 31, 5);
    let cc = CurveConstants {
        f3: F::new(2),
        f2: F::new(3),
        f1: F::new(4),
        f0: F::new(5),
    };
    let p = DivisorCoords::deg1(F::new(0), F::new(6));
    let q = DivisorCoords::deg1(F::new(23), F::new(13)); // (8,13)
    let add_same = add(&p, &p, &cc);
    let doubled = double(&p, &cc);
    let pq = add(&p, &q, &cc);
    let add_pq_same = add(&pq, &pq, &cc);
    let doubled_pq = double(&pq, &cc);
    println!(
        "{}",
        json!({
            "schema": "genus2-external-add-double-gate/v1",
            "upstream_commit": "87e2ebc1b59e5313b67bf81d11e42ec094251518",
            "field": 31,
            "curve_coefficients": [5, 4, 3, 0, 1],
            "point_p": [0, 6],
            "point_q": [8, 13],
            "add_p_p": coords(add_same),
            "double_p": coords(doubled),
            "add_p_p_matches_double_p": add_same == doubled,
            "add_pq_pq": coords(add_pq_same),
            "double_pq": coords(doubled_pq),
            "add_pq_pq_matches_double_pq": add_pq_same == doubled_pq,
        })
    );
}
