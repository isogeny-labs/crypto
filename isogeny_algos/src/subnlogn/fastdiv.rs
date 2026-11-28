//! Exact division through a lower Newton approximation to a reciprocal.
//!
//! This module uses the independent multiplier in [`super::fastint`].  It
//! does not call `Big::divrem` for a growing divisor.  If multiplication of
//! `n`-bit integers costs `M(n)`, the full-width iteration below costs
//! `O(M(n) log n)`.  These fixed logarithmic factors fit the scalar
//! `p^(1 + delta)` allowance of section 7 of the September 2026 manuscript
//! when the multiplication backend is quasi-linear.  This is a correctness
//! and conditional bit-complexity statement, not a timing claim.

use crate::bigint::Big;
use std::fmt;

/// Invalid input to exact unsigned division.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DivisionError {
    ZeroDivisor,
    SizeOverflow,
}

impl fmt::Display for DivisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDivisor => f.write_str("division by zero"),
            Self::SizeOverflow => f.write_str("reciprocal precision exceeds addressable size"),
        }
    }
}

impl std::error::Error for DivisionError {}

/// Return the unique `(q, r)` with `u = q * v + r` and `0 <= r < v`.
///
/// Inputs may have redundant high zero limbs.  Returned values are
/// canonical, including the empty limb vector representing zero.
pub fn divide(u: &Big, v: &Big) -> Result<(Big, Big), DivisionError> {
    divide_counted(u, v).map(|(q, r, _)| (q, r))
}

/// Exact division with the same nonzero-divisor precondition as
/// [`Big::divrem`].  The checked interface is [`divide`].
pub fn divrem(u: &Big, v: &Big) -> (Big, Big) {
    divide(u, v).expect("exact reciprocal division failed")
}

// The count is for correctness tests that must exercise Newton updates,
// rather than only the single-word and small-quotient entry paths.
fn divide_counted(u: &Big, v: &Big) -> Result<(Big, Big, usize), DivisionError> {
    let u = Big::from_limbs(&u.0);
    let v = Big::from_limbs(&v.0);
    if v.is_zero() {
        return Err(DivisionError::ZeroDivisor);
    }
    if u < v {
        return Ok((Big::zero(), u, 0));
    }
    if v.0.len() == 1 {
        // Division by a fixed-width word is a linear scan.  This also
        // handles v = 1 without entering reciprocal iteration.
        let (q, r) = u.divrem_small(v.0[0]);
        return Ok((q, Big::from_u64(r), 0));
    }

    let precision = u.bits().checked_add(1).ok_or(DivisionError::SizeOverflow)?;
    let scale = Big::from_u64(1).shl(precision);
    let twice_divisor = v.shl(1);
    let mut inverse = Big::from_u64(1).shl(precision - v.bits());
    let mut iterations = 0;

    // Write a = scale/v as an exact positive rational.  Initially
    // a/2 <= inverse <= a.  With e = a - inverse, the update is
    // floor(inverse * (2 - inverse/a)), so the same lower bound
    // invariant holds and e_new < e^2/a + 1.
    //
    // At a nonterminal step, residual >= 2v.  Since inverse >= a/2,
    // inverse*residual/scale >= 1, proving strict integer progress.
    // The usual Newton squaring reaches e < 2 in O(log n) updates:
    // for normalized error z=e/a and epsilon=1/a, z_new<z^2+epsilon;
    // while z>=sqrt(epsilon), 2*z_new<=(2*z)^2.  After a constant
    // warmup z<1/2, this requires O(log log a) squarings, followed
    // by a constant number of steps to residual < 2v.  All factors
    // and intermediate products have O(n) bits.
    loop {
        let product = super::fastint::multiply(&v, &inverse);
        let residual = scale.sub(&product);
        if residual < twice_divisor {
            // inverse <= floor(scale/v), and residual < 2v implies
            // that it is short of that floor by at most one.
            if residual >= v {
                inverse = inverse.add_small(1);
            }
            break;
        }
        let increment = super::fastint::multiply(&inverse, &residual).shr(precision);
        assert!(!increment.is_zero(), "Newton reciprocal failed to advance");
        inverse = inverse.add(&increment);
        iterations += 1;
    }

    // inverse = floor(scale/v) and u < scale.  Thus
    // 0 <= u/v - u*inverse/scale < u/scale < 1,
    // and flooring gives either the exact quotient or one less.
    let mut quotient = super::fastint::multiply(&u, &inverse).shr(precision);
    let mut remainder = u.sub(&super::fastint::multiply(&quotient, &v));
    if remainder >= v {
        quotient = quotient.add_small(1);
        remainder = remainder.sub(&v);
    }
    assert!(remainder < v, "exact division correction bound failed");
    Ok((quotient, remainder, iterations))
}

#[cfg(test)]
mod tests {
    use super::{divide, divide_counted, divrem, DivisionError};
    use crate::bigint::Big;

    fn random_limbs(count: usize, state: &mut u64) -> Big {
        let mut limbs = Vec::with_capacity(count);
        for _ in 0..count {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            limbs.push(*state);
        }
        if let Some(last) = limbs.last_mut() {
            *last |= 1;
        }
        Big::from_limbs(&limbs)
    }

    #[test]
    fn agrees_with_native_u128_division() {
        let cases = [
            (0, 1),
            (u128::MAX, 1),
            (u128::MAX, u64::MAX as u128),
            (u128::MAX, (1u128 << 64) + 1),
            (u128::MAX, (1u128 << 127) - 1),
            (1u128 << 127, 1u128 << 64),
            ((1u128 << 126) + 1, (1u128 << 126) + 1),
            (1, u128::MAX),
        ];
        for (u, v) in cases {
            let (q, r) = divide(&Big::from_u128(u), &Big::from_u128(v)).unwrap();
            assert_eq!(q.to_u128(), Some(u / v), "u={u}, v={v}");
            assert_eq!(r.to_u128(), Some(u % v), "u={u}, v={v}");
        }
        let mut state = 0x4b5a_2f19_76e3_a10d_u64;
        for _ in 0..96 {
            let u = random_limbs(2, &mut state).to_u128().unwrap();
            let v = random_limbs(2, &mut state).to_u128().unwrap();
            let (q, r) = divrem(&Big::from_u128(u), &Big::from_u128(v));
            assert_eq!(q.to_u128(), Some(u / v));
            assert_eq!(r.to_u128(), Some(u % v));
        }
    }

    #[test]
    fn canonicalizes_inputs_and_rejects_zero_divisors() {
        assert_eq!(
            divide(&Big::zero(), &Big::zero()),
            Err(DivisionError::ZeroDivisor)
        );
        assert_eq!(
            divide(&Big::from_u64(7), &Big(vec![0, 0])),
            Err(DivisionError::ZeroDivisor)
        );
        assert_eq!(
            divrem(&Big(vec![25, 0, 0]), &Big(vec![4, 0])),
            (Big::from_u64(6), Big::from_u64(1))
        );
        assert_eq!(
            divrem(&Big(vec![0, 0]), &Big(vec![4, 0])),
            (Big::zero(), Big::zero())
        );
    }

    #[test]
    fn powers_of_two_and_boundary_corrections_are_exact() {
        // At equality, this divisor makes the initial reciprocal one
        // too small and the approximate quotient zero.  Both bounded
        // corrections must therefore be applied to obtain (1, 0).
        let v = Big::from_u64(1).shl(256).add_small(1);
        assert_eq!(divrem(&v, &v), (Big::from_u64(1), Big::zero()));
        assert_eq!(divrem(&v.sub_small(1), &v), (Big::zero(), v.sub_small(1)));
        assert_eq!(
            divrem(&v.add_small(1), &v),
            (Big::from_u64(1), Big::from_u64(1))
        );
        let power = Big::from_u64(1).shl(257);
        let q = Big::from_u64(1).shl(383).sub_small(1);
        let r = power.sub_small(1);
        let u = q.shl(257).add(&r);
        assert_eq!(divrem(&u, &power), (q, r));
    }

    #[test]
    fn known_quotients_cover_balanced_and_unbalanced_operands() {
        let mut state = 0xe82a_d491_703b_65fc;
        for (divisor_limbs, quotient_limbs) in [(2, 17), (7, 9), (17, 3), (33, 65)] {
            let divisor = random_limbs(divisor_limbs, &mut state);
            let quotient = random_limbs(quotient_limbs, &mut state);
            let remainder = divisor.shr(1);
            // The existing bounded schoolbook multiplier constructs the
            // oracle input, independently of the fast backend under test.
            let dividend = divisor.mul(&quotient).add(&remainder);
            assert_eq!(divrem(&dividend, &divisor), (quotient, remainder));
        }
    }

    #[test]
    fn large_quotient_executes_multiple_newton_updates() {
        let divisor = Big::from_u64(1).shl(64).add_small(1);
        let quotient = Big::from_u64(1).shl(2048).sub_small(1);
        let remainder = divisor.sub_small(1);
        let dividend = divisor.mul(&quotient).add(&remainder);
        let (q, r, iterations) = divide_counted(&dividend, &divisor).unwrap();
        assert_eq!((q, r), (quotient, remainder));
        assert!(iterations >= 2, "Newton updates were bypassed");
        // This is a structural bound on a fixed correctness example,
        // not a benchmark or a bound on all possible input lengths.
        assert!(
            iterations <= 16,
            "unexpected Newton iteration count: {iterations}"
        );
    }
}
