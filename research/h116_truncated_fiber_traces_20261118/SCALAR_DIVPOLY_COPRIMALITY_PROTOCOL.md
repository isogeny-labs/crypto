# H116 scalar division-polynomial coprimality protocol

Protocol `H116-SCALAR-DIVPOLY-COPRIMALITY-001` is frozen before its native
producer, independent verifier, or affected polynomial gcd exists. It targets
the exact remaining premise between displayed division-polynomial degrees and
Mathlib's canonical reduced rational-function degree. [pcc:unverified]

The out-of-sample panel uses three nonsingular short-Weierstrass curves and the
four scalars `2,3,5,7`, for twelve primary cells. Each implementation must
construct `Phi_n` and `Psi_n^2` from its own division-polynomial recurrence,
run Euclid's algorithm over the locked prime field, normalize the gcd to monic,
and record its complete coefficient vector. Primary acceptance requires gcd
one, not merely absence of roots in the base or sampled extension fields.

Two controls are locked before execution.

1. Multiply both numerator and denominator by `X+1`; the monic gcd must have
   degree one and equal `X+1` when the primary gcd is one.
2. On the singular cubic `y^2=x^3` over `F_13` with scalar two, the common gcd
   of the displayed numerator and denominator must have positive degree. This
   prevents nonsingularity from disappearing from the interpretation.

Every cell and control is retained. The experiment checks finite polynomial
identities only; it does not prove generic coprimality, map identification,
separability, an elliptic function-field embedding, ECDLP recovery, or a
subexponential runtime. [pcc:unverified]
