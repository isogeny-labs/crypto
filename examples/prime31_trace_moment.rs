//! Symmetric trace-moment factor-base equations for m-term prime-field
//! Semaev systems, m=2,3,4. Research candidate, not a faster DLP solver.
//!
//! q(T)=prod_(i=1)^m(T-x_i)=T^m-e1*T^(m-1)+...+(-1)^m e_m
//! h(T)=prod_(w in W)(T-w)
//! H_j(e)=sum_i x_i^j*h(x_i) for j=0..m-1.
//! For p>m and squarefree h split over Fp, all H_j=0 iff the multiset
//! of roots of q is drawn from W. The Jacobian is nonzero at every solution.
//!
//! Build / run in the original crate:
//! cargo test --release --locked --example prime31_trace_moment
//! cargo run --release --locked --example prime31_trace_moment
//! No native Rust result is asserted until those commands run.

use crypto_lib::cryptanalysis::semaev_prime_poly::Fp;
use std::collections::{BTreeMap, BTreeSet};

const PRIME31: u64 = 2_147_483_647;
const A31: u64 = 847_514_846;
const B31: u64 = 2_087_444_724;

#[derive(Clone, Debug)]
struct Jet {
    value: u64,
    deriv: Vec<u64>,
}

/// Newton's identities, differentiated with respect to ALL e_k.
/// Recurrence has no division and remains valid even for repeated roots.
fn power_sums_with_derivatives(f: Fp, elementary: &[u64], max_degree: usize) -> Vec<Jet> {
    let m = elementary.len();
    assert!(m > 0 && (m as u64) < f.p);
    assert!(elementary.iter().all(|&x| x < f.p));
    let mut powers = Vec::with_capacity(max_degree + 1);
    powers.push(Jet {
        value: (m as u64) % f.p,
        deriv: vec![0; m],
    });
    for degree in 1..=max_degree {
        let mut jet = Jet {
            value: 0,
            deriv: vec![0; m],
        };
        for i in 1..=m.min(degree.saturating_sub(1)) {
            let coef = if i % 2 == 1 {
                elementary[i - 1]
            } else {
                f.neg(elementary[i - 1])
            };
            let dcoef = if i % 2 == 1 { 1 } else { f.neg(1) };
            let prev = &powers[degree - i];
            jet.value = f.add(jet.value, f.mul(coef, prev.value));
            for k in 0..m {
                jet.deriv[k] = f.add(jet.deriv[k], f.mul(coef, prev.deriv[k]));
            }
            jet.deriv[i - 1] = f.add(jet.deriv[i - 1], f.mul(dcoef, prev.value));
        }
        if degree <= m {
            let signed = if degree % 2 == 1 { 1 } else { f.neg(1) };
            let mult = f.mul(signed, degree as u64);
            jet.value = f.add(jet.value, f.mul(mult, elementary[degree - 1]));
            jet.deriv[degree - 1] = f.add(jet.deriv[degree - 1], mult);
        }
        powers.push(jet);
    }
    powers
}

/// Exactly m symmetric membership equations H_j and their Jacobian.
/// Complexity O(m^2*(degree h+m)) and O(m*(degree h+m)) storage in this
/// unoptimized differential version; fixed m makes it linear in B.
fn trace_moments(f: Fp, h: &[u64], elementary: &[u64]) -> Vec<Jet> {
    assert!(!h.is_empty());
    let m = elementary.len();
    assert!(m > 0);
    let powers = power_sums_with_derivatives(f, elementary, h.len() + m - 2);
    let mut out = Vec::with_capacity(m);
    for j in 0..m {
        let mut jet = Jet {
            value: 0,
            deriv: vec![0; m],
        };
        for (i, &coef) in h.iter().enumerate() {
            let p = &powers[j + i];
            jet.value = f.add(jet.value, f.mul(coef, p.value));
            for k in 0..m {
                jet.deriv[k] = f.add(jet.deriv[k], f.mul(coef, p.deriv[k]));
            }
        }
        out.push(jet);
    }
    out
}

/// Triangular polynomial row operation:
/// G_j = H_j - e1*H_(j-1) + e2*H_(j-2) - ... + (-1)^j e_j*H0.
/// Same exact (radical) ideal and same Jacobian determinant at every root.
/// Each G_j has total polynomial degree at most deg(h) in e1,...,em,
/// unlike H_j of degree deg(h)+j. This lowers the input degree to the
/// ordered-coordinate degree without introducing permutation copies.
fn reduced_trace_moments(f: Fp, h: &[u64], es: &[u64]) -> Vec<Jet> {
    let raw = trace_moments(f, h, es);
    let m = es.len();
    let mut out = Vec::with_capacity(m);
    for j in 0..m {
        let mut result = Jet {
            value: 0,
            deriv: vec![0; m],
        };
        for a in 0..=j {
            let coeff = if a == 0 {
                1
            } else if a % 2 == 0 {
                es[a - 1]
            } else {
                f.neg(es[a - 1])
            };
            let dcoeff = if a % 2 == 0 { 1 } else { f.neg(1) };
            let term = &raw[j - a];
            result.value = f.add(result.value, f.mul(coeff, term.value));
            for k in 0..m {
                result.deriv[k] = f.add(result.deriv[k], f.mul(coeff, term.deriv[k]));
            }
            if a > 0 {
                result.deriv[a - 1] = f.add(result.deriv[a - 1], f.mul(dcoeff, term.value));
            }
        }
        out.push(result);
    }
    out
}

fn polynomial_from_roots(f: Fp, roots: &[u64]) -> Vec<u64> {
    let mut h = vec![1];
    for &x in roots {
        assert!(x < f.p);
        let mut next = vec![0; h.len() + 1];
        for (i, &v) in h.iter().enumerate() {
            next[i] = f.sub(next[i], f.mul(v, x));
            next[i + 1] = f.add(next[i + 1], v);
        }
        h = next;
    }
    h
}

fn elementary_from_roots(f: Fp, xs: &[u64]) -> Vec<u64> {
    let mut elementary = vec![0; xs.len()];
    for (i, &x) in xs.iter().enumerate() {
        for j in (1..=i).rev() {
            elementary[j] = f.add(elementary[j], f.mul(x, elementary[j - 1]));
        }
        elementary[0] = f.add(elementary[0], x);
    }
    elementary
}

fn derivative_at(f: Fp, h: &[u64], x: u64) -> u64 {
    let mut result = 0;
    for i in (1..h.len()).rev() {
        result = f.add(f.mul(result, x), f.mul(i as u64 % f.p, h[i]));
    }
    result
}

fn evaluate(f: Fp, h: &[u64], x: u64) -> u64 {
    h.iter().rev().fold(0, |acc, &c| f.add(f.mul(acc, x), c))
}

/// det d(H0,...,H_(m-1)) / d(e1,...,em), for exact certificate checks.
fn determinant(f: Fp, mut a: Vec<Vec<u64>>) -> u64 {
    let m = a.len();
    assert!(a.iter().all(|v| v.len() == m));
    let mut product = 1;
    for k in 0..m {
        let Some(pivot) = (k..m).find(|&i| a[i][k] != 0) else {
            return 0;
        };
        if pivot != k {
            a.swap(pivot, k);
            product = f.neg(product);
        }
        let d = a[k][k];
        product = f.mul(product, d);
        let inv = f.inv(d);
        for i in k + 1..m {
            let factor = f.mul(a[i][k], inv);
            for j in k..m {
                a[i][j] = f.sub(a[i][j], f.mul(factor, a[k][j]));
            }
        }
    }
    product
}

fn actual_jacobian(f: Fp, jets: &[Jet]) -> u64 {
    determinant(f, jets.iter().map(|j| j.deriv.clone()).collect())
}

/// Closed-form confluent Vandermonde certificate at a multiset of h-roots:
/// (-1)^(m(m-1)/2) * prod_w multiplicity(w)! * prod_i h'(x_i).
fn expected_jacobian(f: Fp, h: &[u64], xs: &[u64]) -> u64 {
    let m = xs.len();
    let mut ans = if (m * (m - 1) / 2).is_multiple_of(2) {
        1
    } else {
        f.neg(1)
    };
    let mut counts = BTreeMap::new();
    for &x in xs {
        assert_eq!(evaluate(f, h, x), 0);
        *counts.entry(x).or_insert(0usize) += 1;
        ans = f.mul(ans, derivative_at(f, h, x));
    }
    for &count in counts.values() {
        for j in 2..=count {
            ans = f.mul(ans, j as u64);
        }
    }
    ans
}

fn enumerate_multisets(
    f: Fp,
    roots: &[u64],
    m: usize,
    start: usize,
    current: &mut Vec<u64>,
    results: &mut BTreeSet<Vec<u64>>,
    h: &[u64],
) {
    if current.len() == m {
        let es = elementary_from_roots(f, current);
        let raw = trace_moments(f, h, &es);
        let jets = reduced_trace_moments(f, h, &es);
        assert!(raw.iter().all(|j| j.value == 0));
        assert!(jets.iter().all(|j| j.value == 0));
        let got = actual_jacobian(f, &jets);
        assert_eq!(got, actual_jacobian(f, &raw));
        let predicted = expected_jacobian(f, h, current);
        assert_ne!(got, 0);
        assert_eq!(got, predicted);
        results.insert(es);
        return;
    }
    for i in start..roots.len() {
        current.push(roots[i]);
        enumerate_multisets(f, roots, m, i, current, results, h);
        current.pop();
    }
}

fn positive_images(f: Fp, roots: &[u64], m: usize, h: &[u64]) -> BTreeSet<Vec<u64>> {
    let mut results = BTreeSet::new();
    enumerate_multisets(f, roots, m, 0, &mut Vec::new(), &mut results, h);
    results
}

fn prime31_base() -> Vec<u64> {
    let f = Fp::new(PRIME31);
    let mut out = Vec::new();
    for x in 0..1000u64 {
        let fx = f.add(f.add(f.mul(f.mul(x, x), x), f.mul(A31, x)), B31);
        if fx == 0 || f.pow(fx, (PRIME31 - 1) / 2) == 1 {
            out.push(x);
        }
        if out.len() == 32 {
            return out;
        }
    }
    panic!("existing Prime31 base failed to supply 32 rational abscissae");
}

fn main() {
    let f = Fp::new(PRIME31);
    let base = prime31_base();
    let h = polynomial_from_roots(f, &base);
    // Fully deterministic, fixed-sized positive multiset certificates for
    // m=2,3,4. Not a relation yield measurement on random targets.
    for m in 2..=4 {
        let roots = positive_images(f, &base[..8], m, &h);
        println!(
            "p={PRIME31}, B=32, m={m}, positive_unordered_tests={}, exact_jet_jacobian=pass",
            roots.len()
        );
    }
    println!(
        "No Rust benchmark, F4 relation oracle, complete DLP or asymptotic improvement claimed."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhaustive_f17_m2_m3_m4_with_all_negative_assignments() {
        let f = Fp::new(17);
        let base = [0, 1, 3, 5];
        let h = polynomial_from_roots(f, &base);
        for m in 2..=4 {
            let positives = positive_images(f, &base, m, &h);
            let mut cardinality = 1u64;
            for j in 1..=m {
                cardinality = cardinality * (base.len() + j - 1) as u64 / j as u64;
            }
            assert_eq!(positives.len() as u64, cardinality);
            let count = 17u64.pow(m as u32);
            let mut seen = 0;
            for index in 0..count {
                let mut x = index;
                let mut es = vec![0; m];
                for e in &mut es {
                    *e = x % 17;
                    x /= 17;
                }
                let jets = reduced_trace_moments(f, &h, &es);
                let raw = trace_moments(f, &h, &es);
                let is_solution = jets.iter().all(|j| j.value == 0);
                assert_eq!(is_solution, raw.iter().all(|j| j.value == 0));
                assert_eq!(is_solution, positives.contains(&es));
                if is_solution {
                    seen += 1;
                    assert_ne!(actual_jacobian(f, &jets), 0);
                }
            }
            assert_eq!(seen, positives.len());
        }
    }

    #[test]
    fn prime31_m2_to_m4_repeat_root_and_rational_base() {
        main();
        let f = Fp::new(PRIME31);
        let base = prime31_base();
        let h = polynomial_from_roots(f, &base);
        assert_eq!(positive_images(f, &base[..8], 4, &h).len(), 330);

        // In m=2, the trace moments are literally the earlier trace-skew
        // presentation K = 2 H1 - s H0 (including a repeated root).
        for &x in base.iter().take(8) {
            for &y in base.iter().take(8) {
                let s = f.add(x, y);
                let es = vec![s, f.mul(x, y)];
                let jets = trace_moments(f, &h, &es);
                let hx = evaluate(f, &h, x);
                let hy = evaluate(f, &h, y);
                assert_eq!(jets[0].value, f.add(hx, hy));
                let skew = f.sub(f.mul(2, jets[1].value), f.mul(s, jets[0].value));
                assert_eq!(skew, f.mul(f.sub(x, y), f.sub(hx, hy)));
            }
        }

        // e1 is larger than the maximum sum of four base abscissae.
        for j in 0..1000u64 {
            let e = [1000 + j, 31 + 13 * j, 2 * j, 17 * j];
            let jets = reduced_trace_moments(f, &h, &e);
            assert!(jets.iter().any(|k| k.value != 0));
        }
    }
}
