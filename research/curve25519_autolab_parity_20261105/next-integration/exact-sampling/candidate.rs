//! Deterministic full-width hash draws for a new, separately versioned target law.
//! Legacy target laws must keep their original derivation.
use super::canonical::canonical;
use super::exact_scalar::ExactScalar;
use crate::hash::sha256::sha256;
use num_bigint::BigUint;

/// Draw uniformly below `upper` under the ideal independent-digest model.
/// Each attempt hashes its counter along with the domain and decimal inputs.
/// No modulo reduction is applied to a digest: out-of-range draws are rejected.
/// The bound is explicit and exhaustion is returned to the caller.
pub fn derive_below(
    domain: &str,
    parts: &[String],
    upper: ExactScalar,
    max_attempts: u64,
) -> Result<(ExactScalar, u64), String> {
    sample_below(upper, max_attempts, |counter| {
        let input = serde_json::json!({"domain": domain, "parts": parts, "counter": counter});
        sha256(
            canonical(&input)
                .expect("strings and integers only")
                .as_bytes(),
        )
    })
}

fn sample_below(
    upper: ExactScalar,
    max_attempts: u64,
    mut digest: impl FnMut(u64) -> [u8; 32],
) -> Result<(ExactScalar, u64), String> {
    if upper == ExactScalar::default() {
        return Err("sampling bound must be positive".into());
    }
    let limit = upper.to_biguint();
    let bits = (&limit - 1u8).bits();
    let mask = (BigUint::from(1u8) << bits as usize) - 1u8;
    for counter in 0..max_attempts {
        let value = BigUint::from_bytes_be(&digest(counter)) & &mask;
        if value < limit {
            return Ok((
                ExactScalar::from_biguint(&value).expect("masked to at most 256 bits"),
                counter,
            ));
        }
    }
    Err(format!(
        "no scalar accepted in {max_attempts} hash attempts"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejection_keeps_counter_and_does_not_reduce_modulo_bound() {
        let mut seen = Vec::new();
        let (value, counter) = sample_below(5u64.into(), 3, |counter| {
            seen.push(counter);
            let mut d = [0; 32];
            d[31] = [7, 5, 4][counter as usize];
            d
        })
        .unwrap();
        assert_eq!(seen, [0, 1, 2]);
        assert_eq!(value, 4u64.into());
        assert_eq!(counter, 2);
    }
    #[test]
    fn exhaustive_small_draws_have_equal_accepted_multiplicity() {
        for bound in 1u64..=256 {
            let mut counts = vec![0; bound as usize];
            for byte in 0u16..=255 {
                let result = sample_below(bound.into(), 1, |_| {
                    let mut d = [0; 32];
                    d[31] = byte as u8;
                    d
                });
                if let Ok((value, counter)) = result {
                    assert_eq!(counter, 0);
                    counts[value.checked_u64().unwrap() as usize] += 1;
                }
            }
            assert!(counts.iter().all(|n| *n == counts[0] && *n > 0));
        }
    }
    #[test]
    fn invalid_bound_and_exhaustion_are_errors() {
        assert!(sample_below(0u64.into(), 1, |_| panic!("no draw")).is_err());
        assert!(sample_below(5u64.into(), 0, |_| panic!("no draw")).is_err());
        assert!(sample_below(5u64.into(), 2, |_| [255; 32]).is_err());
        assert_eq!(
            sample_below(1u64.into(), 1, |_| [255; 32]).unwrap(),
            (0u64.into(), 0)
        );
    }
    #[test]
    fn full_width_boundaries_keep_high_bits() {
        let max = (BigUint::from(1u8) << 256usize) - 1u8;
        let upper = ExactScalar::from_biguint(&max).unwrap();
        let (value, counter) = sample_below(upper, 2, |counter| {
            let mut d = [255; 32];
            if counter == 1 {
                d[31] = 254;
            }
            d
        })
        .unwrap();
        assert_eq!(value.to_biguint(), max - 1u8);
        assert!(value.checked_u128().is_none());
        assert_eq!(counter, 1);
    }
    #[test]
    fn sha256_draw_replays_exactly_with_domain_and_inputs() {
        let upper: ExactScalar =
            "7237005577332262213973186563042994240857116359379907606001950938285454250989"
                .parse()
                .unwrap();
        let parts = vec![upper.to_string(), "seed=1".into(), "target=0".into()];
        let a = derive_below("ecbench.exact-sampling.test/v1", &parts, upper, 100).unwrap();
        assert_eq!(
            a,
            derive_below("ecbench.exact-sampling.test/v1", &parts, upper, 100).unwrap()
        );
        assert!(a.0 < upper);
        assert!(a.0.checked_u128().is_none());
        assert_ne!(
            a,
            derive_below("ecbench.exact-sampling.test/v2", &parts, upper, 100).unwrap()
        );
        let mut other = parts.clone();
        other[1] = "seed=2".into();
        assert_ne!(
            a,
            derive_below("ecbench.exact-sampling.test/v1", &other, upper, 100).unwrap()
        );
    }
}
