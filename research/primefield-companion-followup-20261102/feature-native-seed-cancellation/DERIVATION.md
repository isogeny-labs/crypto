# Cancellation of projected monomial seed powers

Use the inherited partition h=(r-1)/2=2wk+2s+e, e in{0,1}, and let
N=(r+1)/2=h+1. The normalized division value is T_n=psi_n*c^(n*n).
The full monomial's seed exponent is

    S = delta + alpha*N*N + beta*(2w)*(2w) + gamma*(s+e)*(s+e),

with the tail term omitted when s=0. From the preceding integer factorization,
delta and beta are divisible by4, alpha=4wk+2s+e, gamma=2s.
Also N has parity e+1. Thus modulo4:

- e=0: S = 2s + 2s*s*s = 2s(1+s*s). If s is even,2s is divisible by4;
  if s is odd,1+s*s is even. In both cases S=0 modulo4.
- e=1: N is even, so S = 2s(s+1)^2. If s is even,2s is divisible by4;
  if s is odd,(s+1)^2 is divisible by4. Again S=0 modulo4.

Therefore S is divisible by4 for every supported partition. [pcc:unverified]

Write p-1=q*m with m odd and q=2 or4. Projection raises the monomial to m.
Since q divides S, p-1 divides m*S, so the projected seed factor is1 for
nonzero c. The projected monomial can be initialized directly as a product
of raw division values psi_n raised to projected exponents. This cancellation
is across the product; it does not assert that every individual seed power
is trivial. Larger two-primary images are outside this implementation.
[pcc:unverified]

## Implementation and guard

The constructor checks sum(projected_exponent*n*n)=0 modulo p-1 before making
source state. Accumulation uses u128 modular arithmetic with u64 operands.
A nonzero total is rejected as Domain. Direct initialization now uses d.value(n)
instead of multiplying it by c^(n*n); the twice-base seed c^4*2y is no longer
constructed. Existing zero-exponent omission remains. The phase routine is
still called for subgroup/domain validation, and the public paired wrapper still
requires the original full-value source's sign and ratio agreement.
[pcc:unverified]

This change does not remove derivative-based phase calculation, geometry lanes,
coordinate differences, projected residual exponentiation or the full-value
reference. It removes seed-power work from direct monomial evaluation. No
complete-cost or timing claim follows without accounting for the remaining work.

## Sources and next task

Sources checked: predecessor zero-factor/projected-transport/sign-projection
and multiseed-factor derivations, current phase/source implementation, and the
previously read cyclic-group and division-polynomial primary sources linked in
those reports. KB tools unavailable in this runtime; wrapper previously reported
no executable. This is a direct algebraic continuation, not a novelty survey.

Next: replace the projected path's now-unused phase value computation with a
sufficient subgroup/domain check, while retaining the full validator during
development; then address the projected geometry residual and complete costs.
The original4563-case cohort and prime-field-only scope remain intact.
