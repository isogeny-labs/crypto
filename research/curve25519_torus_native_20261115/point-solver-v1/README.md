# Direct original-point model adapter

The new Rust adapter exports the unchanged six-member TargetCircuit directly,
requests a complete original assignment after SAT and applies the original
equation/index/group checks. Twelve supplied assignments and two asserted
full-assignment backend controls pass. These are extension/replay controls,
not unknown-root discovery. [pcc:PCC-20261123-2033f1]

Exactly one public p101/q3/k8 query,327 variables/357 equations/1139 terms,
ended with the backend timeout diagnostic at its declared5000ms limit. No
status/model or get-value request, no retry. Exit signal6 and the separate
pipe transport error are retained. No absence, hardness, solver rate or
attack exponent follows. [pcc:PCC-20261123-2033f1]

Read [report.md](report.md), [report.pdf](report.pdf), [FIDELITY.md](FIDELITY.md)
and [NEXT.md](NEXT.md). Matching Q does not fix the same raw pretarget sector
as the section formulation. The next proposed operation compiles the existing
section interpolation kernel for unknown coordinates with exact pivot/jet
branches; it is not implemented or claimed novel here. [pcc:unverified]

Source/native scope: [PROTOCOL.md](PROTOCOL.md),
[SOURCE_QUALIFICATION.json](SOURCE_QUALIFICATION.json),
[RESULTS.json](RESULTS.json), [FROZEN.md](FROZEN.md),
[REPRODUCE.md](REPRODUCE.md), [PUBLISHING.md](PUBLISHING.md).
All producer standing remains unsigned/unverified.
