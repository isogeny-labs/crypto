# The actual source field behind Joux–Vitse's empty example

This Lean project connects the printed first Joux–Vitse toy example to
its actual elliptic curve and point group. It constructs
K = F101[t]/(t³+t+1), proves this is a field of cardinality 1,030,301,
checks the nonsingular curve and printed nonzero target, and proves
that no two points with base-field x-coordinates sum to that target.
The result confirms this example's stated nondecomposability.
The whole gate passes 59 declarations with zero failures and zero
trusted-native exceptions.
[pcc:unverified]

The source parameters, in the basis (1,alpha,alpha²), are
a=(44,52,60), b=(58,87,74), xR=(85,84,37), yR=(15,3,86).
The field cardinality is distinct from the paper's asserted curve order
1,029,583, which this project does not certify. [pcc:unverified]

| Mathematical step | Actual Lean declaration |
|:--|:--|
| Cubic irreducibility and extension cardinality | `modulus_irreducible`, `field_cardinality` |
| Genuine coefficient extraction, basis uniqueness | `coordinate_representation`, `representation_zero_iff` |
| Smooth curve and actual nonzero target outside the base field | `curve_nonsingular`, `target_nonzero`, `target_x_not_base` |
| Literal S3 source symmetrization | `source_polynomial_identity` |
| Exact printed coefficients and polynomial descent | `source_coefficient_identities`, `coefficient_decomposition`, `actual_descent` |
| No base-field S3 zero | `source_S3_has_no_base_solution` |
| Actual point group obstruction, including both signs | `actual_target_not_decomposable` |
| Closed combined certificate | `literal_jv_source_field_certificate` |

Every declaration above is in namespace `JVSourceFieldAudit`.
The four source files are [SourceField](lean/SourceField.lean),
[ActualCurve](lean/ActualCurve.lean), [SourceDescent](lean/SourceDescent.lean)
and [ActualDecomposition](lean/ActualDecomposition.lean). [pcc:unverified]

The three descended equations are exactly the literal polynomials
already checked in the separate
[39-declaration ideal certificate](../jv_macaulay_certificate_20261113/README.md).
Its affine unit-ideal proof rules out their simultaneous zeros. The
earlier S3 equivalence uses Mathlib's actual point addition, including
repeated-coordinate, tangent and vertical cases. This project joins
those two results through actual coefficient maps; it does not assume
either the desired descent or the desired group obstruction.
[pcc:unverified]

`InFactorBase` includes either sign of every actual affine point whose
x-coordinate belongs to the embedded F101. Thus the conclusion includes
P+Q and P−Q for every pair from that set. No factor-base sampling,
prime-order premise or generic regularity hypothesis is used.
[pcc:unverified]

This result does not certify the source generator/scalar relations,
whole-curve order, the second split example, minimal degree cutoff,
exact Hilbert length8, internal F4 execution or generic Joux–Vitse
Assumption2. Those remain distinct requirements. The earlier actual
Edwards counterexample refutes the literal universal n=2 FGHR reading;
this positive JV example does not extend that refutation.
[pcc:unverified]

```sh
cd research/jv_source_field_audit_20261113/lean
lake build
cd ../../..
tools/lean/proof-gate.sh --out /tmp/jv-source-field-gate \
  research/jv_source_field_audit_20261113/lean
```

Lean4.35.0 and Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679
are pinned. Only propext, Classical.choice and Quot.sound are used.
There are no holes, custom axioms, native_decide, waived checks or
raised elaboration limits. Whole gate, failed attempts, source pins,
proof map, explanatory diagram and PDF report accompany the project.
Local checks remain distinct from independently signed PCC standing.
No new native experiment, paid compute or agent storage upload.
[pcc:unverified]

Sources: original author manuscript
[page7](https://www-fourier.ujf-grenoble.fr/~viva/research/articles/DLFq5.pdf),
the preceding literal ideal proof and the earlier checked S3 equivalence.
Exact hashes/read scopes are in [source-review.json](evidence/source-review.json).
KB unavailable: no MCP service; repository wrapper exits127. Bounded
primary/repository/Mathlib review was used. No novelty claim.
[pcc:unverified]

Exact local replay for **PCC-20261113-6296df** passes at
2026-11-13T03:03:55Z. All17 formal/config pins and11 artifact pins match.
The complete closed declaration binding is
`673e67fcef6a969711fd6dfa83bf82af9ed5530f531c9cf409c397fef09b1752`.
Signed standing remains unverified. [verification.json](verification.json)
records the actual verdict and inspected report hash; [sources.json](sources.json)
records the immutable source/attempt/gate pins. [pcc:unverified]
