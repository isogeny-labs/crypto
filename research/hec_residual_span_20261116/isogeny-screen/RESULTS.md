# Rational degree-eight screen: exact gate passed

Execution checkpoint 923c5f6c85c, exit 0. All source/protocol/binary hashes
verified before the single invocation. No producer rerun or timing comparison.

All 1395 dimension-three subspaces were enumerated; 135 are isotropic.
Exactly two are invariant under the branch seven-cycle. Both are isotropic
and exactly equal to the kernels of the two Gaussian-period operators.
Integer cyclic multiplication and the two F2 cubic factors pass.

| p | c | (-c)^((p-1)/7) mod p | roots of X7+c | irreducible |
|---|---|---|---|---|
| 113 | 3 | 49 | 0 | yes |
| 421 | 2 | 370 | 0 | yes |

Thus the conditional derivation applies to both retained genus-three models:
their rational polarized (2,2,2) quotients return to the same principally
polarized Jacobian. The argument also repeats after each such step because
the resulting polarized variety is isomorphic over the base field to J.
The fixed infinity/branch action is the actual Frobenius module because
irreducibility forces a seven-cycle; it may be any generator, with exactly
the same invariant subspaces as the shift enumerated by the checker.

The finite module and model eligibility are exhaustively qualified. The
geometric polarization argument in DERIVATION.md remains a written,
unformalized proof using standard theory, not a computed quotient or an
independently reviewed theorem. This is a restricted obstruction and no
novelty or tenfold gain is claimed. Other degrees, extension-defined kernels,
other polarizations and other curves remain outside the conclusion.

Research decision: do not implement the rational degree-eight transport on
these original controls. A higher-degree proposal needs a concrete rational
kernel whose polarized quotient is different, an explicit target-preserving
map and a cost case before implementation. The genus-two and genus-four
objectives remain open; this result cannot qualify either of them.
