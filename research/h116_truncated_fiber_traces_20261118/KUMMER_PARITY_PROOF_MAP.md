# H116 x-only parity obstruction proof map

Written after the frozen Kummer experiment returned exact half accuracy on
every curve, and before modifying the Lean source. [pcc:unverified]

## Claim boundary

The finite experiment tests 184 quadratic covers whose split/inert bit depends
only on the x-coordinate. The formal claim is stronger but narrower:

> On an odd-order cyclic orbit, any binary prediction obtained from an
> observable invariant under point negation must fail on at least one member of
> each scalar pair `k` and `n-k`, for `0<k<n`.

This rules out a total exact parity oracle in the x-only/negation-invariant
class. It does not rule out observables using signed y-data, nonsymmetric
covers, extension-field choices, or nonbinary recursions.

## Dependency chain

1. Natural parity arithmetic:
   if `n % 2 = 1` and `k<=n`, then `(n-k)%2 != k%2`.
2. Orbit negation:
   in an additive commutative group, if `[n]P=0` and `k<=n`, then
   `[(n-k)]P = -[k]P`.
3. Observable symmetry:
   if `obs(-Q)=obs(Q)`, then the observations at the paired scalars agree.
4. Decoder obstruction:
   applying the same decoder to equal observations gives the same prediction,
   which cannot equal both opposite parity labels.

## Mutation/control

The oddness premise is necessary: for even `n`, `k` and `n-k` have the same
parity. Lean will include the concrete `n=4,k=1` equality as a boundary
control. Negation invariance is also explicit rather than hidden in an
elliptic-coordinate simplifier.

## Expected theorem surface

- `odd_sub_mod_two_ne`
- `orbit_sub_eq_neg`
- `negInvariant_binaryDecoder_pair_failure`
- an even-order parity mutation theorem

The exact auditor must report zero failures and zero trusted/native decisions.
