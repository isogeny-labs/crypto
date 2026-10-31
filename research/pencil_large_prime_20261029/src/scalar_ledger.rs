//! Normalize declared scalar events; this is not a whole-program cost model.
//! Callers must supply the verified prime scalar modulus and disjoint counters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "reporting", derive(serde::Serialize))]
pub struct AtomicScalar {
    pub additions_and_negations: u128,
    pub multiplications_outside_inversions: u128,
    pub inversions: u128,
}

/// The pencil pow loop squares once per exponent bit, including the last bit.
/// Remove those expanded multiplications before assigning an atomic-I price.
/// Primality is a caller obligation; this function checks arithmetic consistency.
pub fn pencil_atomic(
    modulus: u64,
    adds: u128,
    negations: u128,
    expanded_muls: u128,
    inversions: u128,
) -> Result<AtomicScalar, &'static str> {
    if modulus <= 2 {
        return Err("scalar model requires odd prime modulus");
    }
    let exponent = modulus - 2;
    let per_inverse = u128::from(64 - exponent.leading_zeros() + exponent.count_ones());
    let internal = inversions
        .checked_mul(per_inverse)
        .ok_or("counter overflow")?;
    Ok(AtomicScalar {
        additions_and_negations: adds.checked_add(negations).ok_or("counter overflow")?,
        multiplications_outside_inversions: expanded_muls
            .checked_sub(internal)
            .ok_or("inversion multiplications missing from pencil ledger")?,
        inversions,
    })
}

/// Ring::inv records one inverse and leaves its modpow internals uncounted.
/// Ring::neg already contributes to the supplied addition counter.
pub fn rho_atomic(adds_including_negations: u128, muls: u128, inversions: u128) -> AtomicScalar {
    AtomicScalar {
        additions_and_negations: adds_including_negations,
        multiplications_outside_inversions: muls,
        inversions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expansion_matches_loop_and_rejects_inconsistent_counts() {
        for modulus in [3u64, 5, 7, 101, 34147, 18446744073709551557] {
            // Independent loop event count, rather than reusing the closed form.
            let mut exponent = modulus - 2;
            let mut multiplications = 0;
            while exponent != 0 {
                if exponent & 1 != 0 {
                    multiplications += 1;
                }
                multiplications += 1;
                exponent >>= 1;
            }
            let candidate = pencil_atomic(modulus, 3, 2, 4 + 2 * multiplications, 2).unwrap();
            assert_eq!(candidate, rho_atomic(5, 4, 2));
            assert!(pencil_atomic(modulus, 0, 0, multiplications - 1, 1).is_err());
        }
        assert!(pencil_atomic(2, 0, 0, 0, 0).is_err());
        assert!(pencil_atomic(7, u128::MAX, 1, 0, 0).is_err());
        assert!(pencil_atomic(7, 0, 0, u128::MAX, u128::MAX).is_err());
    }
}
