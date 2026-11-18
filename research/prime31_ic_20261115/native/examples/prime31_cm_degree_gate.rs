//! Exact CM-field and low-degree endomorphism exclusion for the registered
//! ordinary, prime-order E/F_(2^31-1). Research gate, NOT a DLP algorithm.
//! Uses the existing Rust arithmetic/check_curve/curve constants.
//! Never substitutes a toy curve or changes the original solver.
//! [pcc:unverified]
use prime31_algebraic_ic::{check_curve, Arithmetic, PRIME31};
use serde_json::{json, Value};

fn prime_by_trial_division(n: u64) -> bool {
    if n < 2 { return false; }
    let mut d = 2u64;
    while d <= n / d {
        if n % d == 0 { return false; }
        d += 1;
    }
    true
}

fn exact_gate() -> Value {
    let e = PRIME31;
    let mut arithmetic = Arithmetic::default();
    check_curve(e, &mut arithmetic).expect("original curve/subgroup certificate");
    assert_eq!(e.p, 2_147_483_647);
    assert_eq!(e.n, 2_147_446_831);
    assert_eq!(e.a, 847_514_846);
    assert_eq!(e.b, 2_087_444_724);
    assert_eq!(e.g, (625_037_715, 2_100_627_251));

    // t = p+1-#E. Since t is prime to p, the curve is ordinary.
    let t = i128::from(e.p) + 1 - i128::from(e.n);
    let delta = t * t - 4 * i128::from(e.p);
    assert_eq!(t, 36_817);
    assert_eq!(delta, -7_234_443_099);
    assert!(t != 0 && t.abs() < i128::from(e.p));
    assert!(delta < 0);

    // Delta = f_pi^2 D_K; D_K is fundamental because it is 1 (mod 4)
    // and squarefree (3*13*20610949, all distinct proven primes).
    let f_pi: i128 = 3;
    assert_eq!(delta % (f_pi * f_pi), 0);
    let d_k = delta / (f_pi * f_pi);
    assert_eq!(d_k, -803_827_011);
    assert_eq!(d_k.rem_euclid(4), 1);
    let q = 20_610_949u64;
    assert!(prime_by_trial_division(3));
    assert!(prime_by_trial_division(13));
    assert!(prime_by_trial_division(q));
    assert_eq!((-d_k) as u64, 3 * 13 * q);
    assert_eq!((-delta) as u64, 9 * 3 * 13 * q);

    // For every noninteger alpha in End(E) <= O_K,
    // alpha = a + b(1+sqrt(D_K))/2, a,b integers, b != 0,
    // and 4 deg(alpha) = (2a+b)^2 + |D_K| b^2.
    // This is >= |D_K|+1 (b odd) or >= 4|D_K| (b even).
    // The lower bound holds regardless of whether End(E) is O_K
    // (conductor 1) or Z[pi] (conductor 3).
    let minimum_maximal_order_nonscalar_norm = (1 - d_k) / 4;
    assert_eq!(minimum_maximal_order_nonscalar_norm, 200_956_753);
    for b in -6i128..=6 {
        for a in -10i128..=10 {
            if b == 0 { continue; }
            let quadruple_norm = (2*a+b)*(2*a+b) + (-d_k)*b*b;
            assert_eq!(quadruple_norm % 4, 0);
            assert!(quadruple_norm / 4 >= minimum_maximal_order_nonscalar_norm);
        }
    }
    // j is neither 0 (which requires a=0) nor 1728 (b=0) in char>3.
    assert_ne!(e.a, 0);
    assert_ne!(e.b, 0);

    json!({
        "schema":"prime31-cm-degree-gate/v1",
        "standing":"[pcc:unverified]",
        "source":"original Rust PRIME31, Arithmetic, check_curve",
        "p":e.p, "n":e.n, "cofactor":1, "a":e.a, "b":e.b,
        "g":[e.g.0,e.g.1],
        "trace":t.to_string(),
        "frobenius_discriminant":delta.to_string(),
        "fundamental_discriminant":d_k.to_string(),
        "frobenius_order_conductor":f_pi,
        "actual_endomorphism_conductor_candidates":[1,3],
        "proved_prime_factors_DK":[3,13,q],
        "noninteger_endomorphism_degree_lower_bound":minimum_maximal_order_nonscalar_norm.to_string(),
        "no_nonscalar_endomorphism_degree_below":minimum_maximal_order_nonscalar_norm.to_string(),
        "j_special_automorphisms":false,
        "proof":"4*N(a+b*omega)=(2a+b)^2+|D_K|b^2; b != 0",
        "scope":"exact quadratic-order bound, not proof of slow factored isogeny loops or a DLP speed result",
        "performance_measured":false,
        "ecdlog_recovery_measured":false
    })
}

fn main() {
    println!("PRIME31_CM_GATE_JSON={}", exact_gate());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_prime31_exact_cm_bound() {
        let record = exact_gate();
        assert_eq!(record["trace"], "36817");
        assert_eq!(record["fundamental_discriminant"], "-803827011");
        assert_eq!(record["noninteger_endomorphism_degree_lower_bound"], "200956753");
        assert_eq!(record["performance_measured"], false);
    }
    #[test]
    fn distinct_prime_factor_certificates() {
        assert!(prime_by_trial_division(20_610_949));
        assert!(!prime_by_trial_division(20_610_949 * 3));
        assert!(!prime_by_trial_division(1));
    }
}
