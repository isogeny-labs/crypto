//! Direct Semaev pair consequences: no instance scout or reference GB.
include!("kernels/cheap.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellations_and_small_prime_parameters_keep_exact_certificates() {
        for p in [5, 7, 101] {
            for a in [0, 2, p - 1] {
                for b in [0, 3 % p, p - 1] {
                    for u in 0..p.min(6) {
                        for v in u + 1..p.min(6) {
                            let f = vec![f3(2, [0, 1], a, b, u, p), f3(2, [0, 1], a, b, v, p)];
                            let c = consequences(&f, 2, p);
                            assert_eq!(c.len(), 1);
                            assert!(verify(&c[0].cubic_syzygy, &f, p));
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn direct_cubic_retains_the_threshold_without_a_scout() {
        for input in [
            include_str!("../fixtures/system-2147483647.json"),
            include_str!("../fixtures/system-9007199254740881.json"),
        ] {
            let system: System = serde_json::from_str(input).unwrap();
            let f = system.decode().unwrap();
            let (augmented, pairs) = augment(&f, 2, system.prime);
            assert_eq!(pairs.len(), 1);
            assert_eq!(degree(&pairs[0].cubic_syzygy.poly), 3);
            let gb = reference(&f, 2, system.prime).unwrap();
            assert_eq!(
                trace(&augmented, 2, system.prime, &gb, 6, 5000)
                    .0
                    .solving_degree,
                Some(4)
            );
            let augmented_gb = reference(&augmented, 2, system.prime).unwrap();
            assert_eq!(
                gb.iter().map(|r| r.poly.clone()).collect::<BTreeSet<_>>(),
                augmented_gb.iter().map(|r| r.poly.clone()).collect()
            );
        }
    }
    #[test]
    fn scale_and_variable_embedding_preserve_certificates() {
        let p = 2147483647;
        let mut f = vec![f3(3, [0, 2], 2, 3, 7, p), f3(3, [0, 2], 2, 3, 11, p)];
        for value in f[1].values_mut() {
            *value = mul(*value, 17, p);
        }
        let c = consequences(&f, 3, p);
        assert_eq!(c.len(), 1);
        assert!(verify(&c[0].cubic_syzygy, &f, p));
        assert!(verify(&c[0].cubic_difference, &f, p));
    }
    #[test]
    fn refuses_invalid_or_degenerate_pairs() {
        let p = 101;
        let f = f3(2, [0, 1], 2, 3, 7, p);
        assert!(consequences(&[f.clone(), f.clone()], 2, p).is_empty());
        let mut invalid = f3(2, [0, 1], 2, 3, 9, p);
        invalid.insert(vec![0, 0], 42);
        assert!(consequences(&[f, invalid], 2, p).is_empty());
        assert!(consequences(&[BTreeMap::from([(vec![1, 0], 1)])], 2, 2).is_empty());
    }
}
