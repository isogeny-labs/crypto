# Frozen scalar-field endpoint certificate, version 1

`KernelInstance.lean`, its Lake manifest and toolchain are now frozen proof
artifacts. PCC-20261116-0bbe9a pins the complete source/manifest as well as
the declaration statements, so the scalar modulus and contextual hypotheses
cannot drift. Create a new version for a change; do not rewrite this certificate.

The maintained [gate receipt](gate-v1/) passes three endpoints with zero
trusted-native exceptions. Their axioms are only propext, Classical.choice
and Quot.sound. The theorem statements explicitly retain the scalar-primality
hypothesis, checked matrix equations and any nonzero-coefficient contract.
Signed independent PCC standing remains unverified. [pcc:unverified]

The source reuses existing CryptoLean.SingleRelation and Mathlib matrix/group
laws at the pinned revisions. It does not prove the curve/log-coordinate
bridge, uniform sampling/selection of retained rows, the geometric energy
bound, a decomposition algorithm or a new attack exponent. See PROOF_MAP.md.
[pcc:unverified]

Development failures are preserved in `development/`: v1 lacked the new
Mathlib Field.ZMod import, and v3 lacked a Fintype.card_fun simplification.
Neither failed build is a proof. No heartbeat limit was raised; the final
source passes the kernel and axiom gate.


The additive `SharedQueryCounterexample.lean` and
`ROW_CONTRACT_PROOF_MAP.md` supply a concrete F5 same-query witness.
The complete project's new gate-v2 passes four named declarations, with
no failing/trusted-native axioms. PCC-20261116-8547d2 pins the new source,
configuration and unchanged manifest; the old KernelInstance.lean and
historical gate-v1 receipt retain their bytes. Read the versioned
retained-row contract report alongside the original proposal. [pcc:unverified]
