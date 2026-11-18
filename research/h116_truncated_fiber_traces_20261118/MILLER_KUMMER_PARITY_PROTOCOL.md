# H116 Miller-doubling Kummer parity protocol

Protocol ID: `H116-MILLER-KUMMER-PARITY-001`

Filed after the arbitrary horizontal signed-y covers calibrated null, before
evaluating any Miller-function signature. [pcc:unverified]

For anchors `A_j=[j]P`, `1<=j<=8`, use the standard doubling Miller function

```text
g_j(Q) = line_{A_j,A_j}(Q) / vertical_{2A_j}(Q),
div(g_j) = 2[A_j] - [2A_j] - [O].
```

Its quadratic character is public; because a nonzero denominator and its
inverse have the same Legendre symbol, it is computed as the character of the
tangent-line value times the vertical-line value. Zeros and poles abstain.
Test products over all nonempty anchor subsets of size at most three and both
parity polarities: 184 rules total.

Use the same 1,520 targets, exact statistic, deterministic tie-break and 4,096
within-curve family-wise shuffles as both preceding Kummer tests. Retain total
and covered exactness, per-curve values, and byte-identical replay.

This family escapes the x-only negation theorem and has group-law divisor
structure absent from the horizontal y-level family. Only a calibrated result
can nominate a separately frozen OOS test; no abstention, finite excess, or
Miller-function identity is itself an ECDLP algorithm. [pcc:unverified]
