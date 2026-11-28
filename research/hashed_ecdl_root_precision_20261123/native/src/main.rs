//! Unbounded scalar translation, explicitly not the production usize API.
use num_bigint::BigUint;
use num_traits::{One, Zero};

fn pow2(exponent: usize) -> BigUint {
    BigUint::one() << exponent
}

fn precision(p: &BigUint, m: &BigUint, u: usize, a: usize, i: usize) -> BigUint {
    assert!(u > 0 && i <= a);
    let width = (m * 3u32 + BigUint::from(u - 1)) / u;
    p + width * pow2(a - i) * 8u32 + m * BigUint::from(a + 1) * 8u32
}

fn envelope(m: &BigUint, u: usize, i: usize, constant: u32) -> BigUint {
    m * constant * pow2(2 * u - i)
}

#[derive(Clone)]
struct Witness {
    size: BigUint,
    m: BigUint,
    p: BigUint,
    degree: BigUint,
    delta: BigUint,
    lambda: usize,
    sqrt: usize,
    u: usize,
    a: usize,
    length: BigUint,
    array: BigUint,
    t: i64,
}

/// Certificates for this exact-power family; no general nth-root oracle.
fn source_checks(w: &Witness) -> Vec<(&'static str, bool)> {
    let mut c = vec![
        ("size_at_least_two", w.size >= BigUint::from(2u32)),
        ("root_size_edges_equal", w.size == w.m),
        ("even_edges", (&w.m % 2u32).is_zero()),
        ("initial_precision", w.p == &w.m / 2u32 + 1u32),
        ("lambda", (&w.size - 1u32).bits() == w.lambda as u64),
        (
            "sqrt_floor",
            w.sqrt > 0 && w.sqrt * w.sqrt <= w.lambda && w.lambda < (w.sqrt + 1) * (w.sqrt + 1),
        ),
        ("exact_power_delta", w.delta.pow(10) == w.size.pow(9)),
        (
            "nonzero_delta_degree_branch",
            !w.delta.is_zero() && w.degree <= w.delta,
        ),
        ("root_scale", w.u == w.lambda / 131_072 && w.u > 0),
        ("root_exponent", w.a == 2 * w.u),
        ("source_length", w.length == pow2(w.a)),
        (
            "source_array",
            w.array == BigUint::from(64u32).pow((2 * w.a) as u32),
        ),
        ("scale_exponent", w.u <= w.a),
        ("scale_edges", BigUint::from(w.u) <= w.m),
        ("degree_guard", &w.array * &w.array * &w.delta * 2u32 <= w.m),
        ("length_array_guard", &w.length * &w.array <= w.m),
        ("length_delta_guard", &w.length * 2u32 <= w.delta),
        (
            "node_grid",
            if w.t < 0 {
                BigUint::from(w.t.unsigned_abs()) <= w.length
            } else {
                BigUint::from(w.t.unsigned_abs()) < w.length
            },
        ),
    ];
    // A malformed sqrt is rejected above, with no division by zero below.
    c.push(("above_leaf", w.sqrt > 0 && w.m > &w.size / w.sqrt.max(1)));
    let valuation = w.t.unsigned_abs().trailing_zeros() as usize;
    c.push(("active_node", w.t != 0 && valuation < w.u));
    if valuation <= w.a {
        c.push((
            "shift_power",
            (&w.length >> valuation) == pow2(w.a - valuation),
        ));
    } else {
        c.push(("shift_power", false));
    }
    c
}

fn emit_check(id: &str, pass: bool) {
    println!("{{\"kind\":\"check\",\"id\":\"{id}\",\"pass\":{pass}}}");
    assert!(pass, "check failed: {id}");
}

fn main() {
    println!("{{\"kind\":\"scope\",\"value\":\"unbounded scalar certificates; no allocated graph or production Setup call\"}}");
    let m = pow2(262_150);
    let w = Witness {
        size: m.clone(),
        p: &m / 2u32 + 1u32,
        m,
        degree: BigUint::from(2u32),
        delta: pow2(235_935),
        lambda: 262_150,
        sqrt: 512,
        u: 2,
        a: 4,
        length: BigUint::from(16u32),
        array: pow2(48),
        t: 2,
    };
    for (id, pass) in source_checks(&w) {
        emit_check(id, pass);
    }
    let e = &w.m / 2u32;
    let q = precision(&w.p, &w.m, 2, 4, 1) * 2u32;
    let floor = &w.m * 3u32 / 2u32;
    let rem = (&w.m * 3u32) % 2u32;
    let ceil_by_remainder = floor + u32::from(!rem.is_zero());
    emit_check(
        "independent_ceiling",
        ceil_by_remainder == (&w.m * 3u32 + 1u32) / 2u32,
    );
    emit_check("normalized_exact_identity", q == &e * 546u32 + 2u32);
    emit_check("root35", q <= envelope(&w.m, 2, 1, 35));
    emit_check("constant34_fails", q > envelope(&w.m, 2, 1, 34));
    emit_check(
        "normalized34_difference",
        &q - envelope(&w.m, 2, 1, 34) == &e * 2u32 + 2u32,
    );
    emit_check("exceeds_usize", w.m.bits() > usize::BITS as u64);

    for id in [
        "lambda",
        "u",
        "delta",
        "sqrt",
        "degree",
        "valuation",
        "array",
    ] {
        let mut bad = w.clone();
        match id {
            "lambda" => bad.lambda += 1,
            "u" => bad.u += 1,
            "delta" => bad.delta += 1u32,
            "sqrt" => bad.sqrt += 1,
            "degree" => bad.degree = &bad.delta + 1u32,
            "valuation" => bad.t = 4,
            "array" => bad.array = BigUint::from(64u32).pow(7),
            _ => unreachable!(),
        }
        let failures: Vec<_> = source_checks(&bad)
            .into_iter()
            .filter(|(_, ok)| !ok)
            .collect();
        assert!(!failures.is_empty(), "bad source binding accepted: {id}");
        let names = failures
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{{\"kind\":\"negative\",\"id\":\"{id}\",\"rejected\":true,\"failed\":\"{names}\"}}"
        );
    }

    let mut census = 0u64;
    for u in 1..=12 {
        let mut maximum = (BigUint::zero(), BigUint::one());
        for m_small in 32 * u..32 * u + 128 {
            let m = BigUint::from(m_small);
            for p_small in [0, m_small / 2 + 1, m_small] {
                for i in 0..u {
                    let left = precision(&BigUint::from(p_small), &m, u, 2 * u, i) * u;
                    let denominator = &m * pow2(2 * u - i);
                    assert!(left <= &denominator * 35u32, "root35 scalar sweep");
                    if &left * &maximum.1 > &maximum.0 * &denominator {
                        maximum = (left, denominator);
                    }
                    census += 1;
                }
            }
        }
        println!(
            "{{\"kind\":\"sweep_maximum\",\"u\":{u},\"numerator\":{},\"denominator\":{}}}",
            maximum.0, maximum.1
        );
    }
    emit_check("census_29952", census == 29_952);
    for (id, p, m, u, i) in [
        ("missing_initial_precision", 1024u32, 32u32, 1usize, 0usize),
        ("missing_margin", 3, 3, 2, 1),
        ("missing_active_valuation", 32, 32, 1, 1),
    ] {
        let remaining_premises = match id {
            "missing_initial_precision" => 32 * u <= m as usize && i < u && p > m,
            "missing_margin" => p <= m && i < u && 32 * u > m as usize,
            "missing_active_valuation" => p <= m && 32 * u <= m as usize && i >= u,
            _ => unreachable!(),
        };
        assert!(
            remaining_premises,
            "incorrect premise-deletion control: {id}"
        );
        let left = precision(&BigUint::from(p), &BigUint::from(m), u, 2 * u, i) * u;
        let right = envelope(&BigUint::from(m), u, i, 35);
        emit_check(id, left > right);
        println!(
            "{{\"kind\":\"deletion_values\",\"id\":\"{id}\",\"left\":{left},\"right\":{right}}}"
        );
    }
    let small = precision(&BigUint::from(33u32), &BigUint::from(64u32), 2, 4, 1) * 2u32;
    emit_check(
        "E32_scalar34_failure",
        small == BigUint::from(17474u32) && small > envelope(&BigUint::from(64u32), 2, 1, 34),
    );
    println!("{{\"kind\":\"result\",\"pass\":true,\"scalar_rows\":{census},\"corruptions\":7,\"huge_witness\":\"M=2^262150,p=M/2+1,delta=2^235935\"}}");
}
