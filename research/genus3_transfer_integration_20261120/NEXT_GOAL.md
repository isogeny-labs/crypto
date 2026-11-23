# Next goal: P-256 full-bitangent Frobenius descent

## Decision question

Does the reconstructed P-256 plane quartic over `F_(p^3)` descend, with its
polarization and quotient data, to a usable genus-3 curve over `F_p`?
[pcc:unverified]

## Acceptance contract

1. Construct equations for all 28 bitangents, not only the seven-bitangent
   Aronhold subset.
2. Compute the Frobenius permutation and recover a projective isomorphism
   `M : C -> C^(p)` from the full configuration.
3. Verify the cocycle `M^(p^2) M^(p) M` is scalar and solve the corresponding
   Hilbert-90 descent equation.
4. Produce a 15-term or otherwise explicit quartic over `F_p`; verify smoothness,
   equality of geometric invariants, and round-trip base extension.
5. Record a negative certificate if the repeated-product specialization or
   cocycle obstructs descent; do not replace a failed descent with a modeled one.

## Immediate follow-on gate

Descent alone is insufficient. A successful result must be followed by a signed
effective degree-three divisor lift for P-224 or P-256 and an executable
`pi^dagger(pi(P))=[3]P` check on nontrivial prime-order subgroup points. Kummer
coordinates are only an intermediate control because they erase sign.
[pcc:unverified]

## Stop rule

Do not run a NIST-scale relation or timing panel until the field, signed-map,
subgroup-preservation, and recovery obligations are executable. A failure at any
construction gate is a reportable result and ends that branch of the experiment.
