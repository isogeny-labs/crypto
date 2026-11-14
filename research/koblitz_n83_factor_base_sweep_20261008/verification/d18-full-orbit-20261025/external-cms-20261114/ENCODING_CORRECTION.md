# Pre-solver encoding correction, 2026-11-14

The first source-bound `primary-full-d18-m4-l0-xcnf positive` export completed
before any CryptoMiniSat invocation. Its terminal receipt reports
`ordered_first_pair=false`, `search_policy=phase_zero_first`,
`xor_decision_cadence=1`, 329,726 variables, 2,807,752 clauses and 3,901
XOR rows. The 50,518,392-byte XCNF has SHA-256
`0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c`.
This differs from native attempt 7's ordered-first-pair, cadence-32 formula
with 329,808 variables. The [initial protocol](PROTOCOL.md) incorrectly
described the solver backend as the only independent variable relative to
attempt 7. Preserve that protocol and this export receipt; do not calculate
or imply a backend-only speed ratio to attempt 7.

The bounded external panel may continue as a **distinct full-D18 candidate**:
free first-pair order, shared-root Frobenius phase domain, phase-zero-first
search, and CryptoMiniSat 5.14.7. Its positive and first-public-natural
exports must both use this exact CLI entry point and source binary. Freeze
their hashes in `EXPORT_LOCK.json` before solver timing. The existing caps,
host gate, output retention, positive-before-natural condition, and model
verification requirements remain as written in the initial protocol.
Charge the completed positive export's 51.715-second worker process wall as
setup evidence. A terminal solver status from this candidate cannot be
combined with attempt 7's ordered formula to infer a factor-base or solver
winner. If the positive export or later formula identities differ, stop and
amend again before timing.
