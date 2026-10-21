//! Count rational Weierstrass points of the exact Scholten quintics.
//! A quintic model has full rational 2-torsion only when its five finite
//! Weierstrass points (and the point at infinity) are rational.
use five_leads_study::three_leads::Scholten;
use serde_json::json;

const PRIMES: [u64; 21] = [
    739, 1151, 1187, 3371, 3527, 3931, 11731, 11887, 12143, 48383, 48823, 49451, 186379, 186451,
    186583, 741967, 748807, 749299, 2967343, 2968367, 2972219,
];

fn evaluate(coefficients: &[u64], x: u64, p: u64) -> u64 {
    coefficients.iter().rev().fold(0, |acc, &c| {
        ((acc as u128 * x as u128 + c as u128) % p as u128) as u64
    })
}

fn main() {
    for p in PRIMES {
        let model = Scholten::new(p);
        let polynomial = &model.j.polynomial;
        assert_eq!(polynomial.len(), 6);
        assert!(polynomial.iter().all(|&c| c < p));
        let derivative: Vec<_> = (1..polynomial.len())
            .map(|i| (i as u64 * polynomial[i]) % p)
            .collect();
        let roots: Vec<_> = (0..p)
            .filter(|&x| evaluate(polynomial, x, p) == 0)
            .collect();
        assert!(roots.iter().all(|&x| evaluate(&derivative, x, p) != 0));
        assert_eq!(roots.len(), 5);
        let mut factored = vec![polynomial[5]];
        for &root in &roots {
            let mut next = vec![0; factored.len() + 1];
            for (i, &coefficient) in factored.iter().enumerate() {
                next[i] = (next[i] + p - coefficient * root % p) % p;
                next[i + 1] = (next[i + 1] + coefficient) % p;
            }
            factored = next;
        }
        assert_eq!(&factored, polynomial);
        let root_fractions: Vec<_> = roots.iter().map(|&x| x as f64 / p as f64).collect();
        println!(
            "{}",
            json!({
                "schema":"four-followups-kummer-model/v1",
                "p":p,
                "quintic_coefficients":polynomial,
                "finite_rational_weierstrass_roots":roots,
                "finite_root_fraction_of_p":root_fractions,
                "finite_root_count":roots.len(),
                "full_rational_two_torsion":roots.len()==5,
                "factorization_verified":true,
                "method":"exhaustive Horner evaluation at every x in F_p; squarefree derivative checked at roots; polynomial reconstructed from roots"
            })
        );
    }
}
