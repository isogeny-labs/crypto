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

// Degree-three quotient arithmetic F_p[X]/(g). This computes an exact
// irreducibility certificate for the cubic factor of the 3-division polynomial.
// It is an integer certificate checker, NOT an alternative EC group law.
fn fmul(a: u64, b: u64) -> u64 {
    ((u128::from(a) * u128::from(b)) % u128::from(PRIME31.p)) as u64
}
fn fsub(a: u64, b: u64) -> u64 { (a + PRIME31.p - b) % PRIME31.p }
fn fadd(a: u64, b: u64) -> u64 { (a + b) % PRIME31.p }

fn cubic_mul_mod(a: [u64; 3], b: [u64; 3], g: [u64; 3]) -> [u64; 3] {
    // g(X) = X^3 + g[2] X^2 + g[1] X + g[0].
    let mut z = [0u64; 5];
    for i in 0..3 {
        for j in 0..3 {
            z[i+j] = fadd(z[i+j], fmul(a[i], b[j]));
        }
    }
    for k in (3..=4).rev() {
        let c = z[k];
        for j in 0..3 {
            z[k-3+j] = fsub(z[k-3+j], fmul(c,g[j]));
        }
        z[k] = 0;
    }
    [z[0],z[1],z[2]]
}
fn x_to_p_mod_cubic(g: [u64;3]) -> [u64;3] {
    let mut e = PRIME31.p;
    let mut a = [0,1,0]; // X
    let mut out = [1,0,0];
    while e>0 {
        if e&1==1 { out=cubic_mul_mod(out,a,g); }
        a=cubic_mul_mod(a,a,g);
        e>>=1;
    }
    out
}
fn certified_three_division_nonsplit() -> ([u64;3], [u64;3]) {
    let e = PRIME31;
    // psi_3(X)=3X^4+6aX^2+12bX-a^2. We certify
    // psi_3=3(X-533591203)*g(X) and g irreducible over Fp.
    let root=533_591_203u64;
    let g=[1_197_836_693,499_402_600,533_591_203]; // monic cubic
    let rhs=[
        fmul(3, fsub(0,fmul(root,g[0]))),
        fmul(3,fsub(g[0],fmul(root,g[1]))),
        fmul(3,fsub(g[1],fmul(root,g[2]))),
        fmul(3,fsub(g[2],root)),
        3
    ];
    let lhs=[fsub(0,fmul(e.a,e.a)),fmul(12,e.b),fmul(6,e.a),0,3];
    assert_eq!(rhs,lhs,"exact 3-division polynomial factor certificate");

    let xp=x_to_p_mod_cubic(g);
    assert_eq!(xp,[58_744_165,1_889_711_167,1_703_332_450]);
    let r=[xp[0],fsub(xp[1],1),xp[2]];
    // If g had a root in Fp then X^p-X would vanish at that root.
    // A unit inverse of (X^p-X) mod g proves there is no root.
    let inverse=[30_145_591,2_045_657_651,1_195_765_287];
    assert_eq!(cubic_mul_mod(r,inverse,g),[1,0,0],
        "Bezout inverse rules out all Fp roots of g");
    (r,inverse)
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
    // If End(E)=Z[pi] (the conductor-three order), b is divisible
    // by three, strengthening the conditional bound by a factor ~9.
    let conditional_conductor_three_bound = (1 - 9 * d_k) / 4;
    assert_eq!(conditional_conductor_three_bound, 1_808_610_775);
    let (_bezout_remainder,_bezout_inverse)=certified_three_division_nonsplit();
    // If End(E)=O_K, pi=18407+3*omega acts as scalar [18407]=[-1]
    // on E[3], so Frobenius fixes all four 3-torsion x-coordinates.
    // The certified irreducible cubic factor contradicts that.
    // Hence End(E)=Z[pi], with conductor exactly 3.
    let actual_endomorphism_conductor=3u64;
    // Sharpness: alpha=pi-[18408] belongs to Z[pi] and attains
    // this norm, despite acting as [-18407] on E(Fp).
    let c=18_408i128;
    let achieved_norm=i128::from(e.p)-c*t+c*c;
    assert_eq!(achieved_norm,conditional_conductor_three_bound);
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
        "actual_endomorphism_conductor":actual_endomorphism_conductor,
        "actual_endomorphism_order":"Z[pi], conductor 3 (3-division cubic certificate)",
        "three_division_rational_x_root":533_591_203,
        "three_division_irreducible_cubic":[1_197_836_693,499_402_600,533_591_203,1],
        "frobenius_remainder_mod_cubic":[58_744_165,1_889_711_166,1_703_332_450],
        "bezout_inverse_mod_cubic":[30_145_591,2_045_657_651,1_195_765_287],
        "proved_prime_factors_DK":[3,13,q],
        "noninteger_endomorphism_degree_lower_bound":minimum_maximal_order_nonscalar_norm.to_string(),
        "no_nonscalar_endomorphism_degree_below":conditional_conductor_three_bound.to_string(),
        "actual_minimal_nonscalar_degree":achieved_norm.to_string(),
        "attaining_endomorphism":"Frobenius-[18408]",
        "weaker_maximal_order_bound":minimum_maximal_order_nonscalar_norm.to_string(),
        "j_special_automorphisms":false,
        "proof":"psi3=(X-r)*irreducible cubic proves End(E)=Z[pi]; b=3k; 4N=(2a+b)^2+|D_K|b^2",
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
        assert_eq!(record["actual_endomorphism_conductor"],3);
        assert_eq!(record["actual_minimal_nonscalar_degree"],"1808610775");
        assert_eq!(record["no_nonscalar_endomorphism_degree_below"],"1808610775");
        assert_eq!(record["performance_measured"], false);
    }
    #[test]
    fn exact_three_division_non_split_certificate() {
        let (r,inverse)=certified_three_division_nonsplit();
        assert_eq!(r,[58_744_165,1_889_711_166,1_703_332_450]);
        assert_eq!(inverse,[30_145_591,2_045_657_651,1_195_765_287]);
    }
    #[test]
    fn distinct_prime_factor_certificates() {
        assert!(prime_by_trial_division(20_610_949));
        assert!(!prime_by_trial_division(20_610_949 * 3));
        assert!(!prime_by_trial_division(1));
    }
}
