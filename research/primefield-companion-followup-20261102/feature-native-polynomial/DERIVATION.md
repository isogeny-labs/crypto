# Native source polynomial engine

The native implementation preserves the historical source polynomial pipeline:
schoolbook multiplication when the shorter input has at most 12 coefficients,
Karatsuba above that cutoff, Newton series inversion, reversed polynomial division,
and product/remainder trees for ordered multipoint evaluation. The source-root
resultant is the product of evaluations at the supplied center points.

Newton updates use v <- v(2-av) modulo increasing powers of X. Reversing the
numerator and divisor reduces quotient extraction to truncated series inversion;
the implementation then checks that the remainder has smaller degree. The tree
retains input order and accepts repeated evaluation points. Common roots yield zero.
Empty trees, zero divisors and noninvertible constants are explicit errors.

Four native unit methods passed once at the source revision in receipt.json.
They check all three historical setup resultants and partial polynomial values,
Karatsuba against independent schoolbook multiplication across the cutoff and
uneven lengths, Newton identities and independent classical long division,
and tree evaluation against Horner including repeats and domain errors.
The checked fixtures have subgroup orders 359, 479 and 683. These are software
contract checks, not a fresh elliptic source or recovery experiment.

The caller certifies that p is prime; the implementation bounds p by u64::MAX/2
and uses u128 products. Unlike the original trim routine, native trimming also
canonicalizes coefficients. Allocation and operation-counter behavior is not
claimed identical. No timing or external-backend performance claim is made.

## Prior work and provenance

The preserved original-polynomial.py.txt is the original
blocked-character-recurrence/check.py, reached through the historical source
aliases. original-inputs.sha256 binds it and the three setup receipts.
The implementation follows that established algorithm structure. The relevant
external reference is FLINT's modular-polynomial documentation:
https://flintlib.org/doc/nmod_poly/index.html . No novelty or FLINT speed claim
is attached to this port.

## Next integration

Connect this engine to the native division/phase and shared-window geometry,
preserving normalized recurrence, partial tails and calibration. The original
4563-case cohort remains unchanged and was not executed here.
