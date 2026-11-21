# Conditional model and concrete parameter certificates

Build with pinned Lean4.35.0 using `lake build`, then the maintained repository
`tools/lean/proof-gate.sh` for this directory. `PROOF_MAP.md` supplies the
six-field instance gate, inheritance and missing bridges. [pcc:unverified]

`ScalingAudit.lean` contains six named rational numeric-model endpoints.
`ParameterArithmetic.lean` contains four Core natural-number obligations.
The maintained audit counts twelve declarations because two generated
definition-equation theorems are included. All ten named endpoints are bound
by PCC-20261116-76dc04 along with complete source and dependency-manifest
bytes. Source and manifest are frozen; see `FROZEN.md`. [pcc:unverified]

The four Core checks do not certify the earlier transport/cardinality premise
or a point count. The rational proofs do not supply actual solver/yield,
real-power asymptotic, growing-family or full-phase bounds. The report calls
these numeric model values throughout; signed PCC standing is unverified.
