# Main strict-lint failures block integrated PKM validation

- **Status:** open
- **Kind:** inherited software lint failures
- **Last updated:**2026-11-05
- **Affected work:** [PR3452](https://github.com/isogeny-labs/crypto/pull/3452)

## Observed blocker

On integrated sourcebbf241b7e, all26release examples pass, then strict scoped
clippy exits101 for two library errors: unused `solve_one_with_trace` at
`research/pencil_large_prime_20261029/src/native_solver.rs:1351`, and
`receipt.completion.roundtrip_verified == false` at `src/ml_dataset/custody.rs:713`.
All library/local-dependency compile-input objects equal main d30b9e0db.
These are producer observations, not independently signed results. [pcc:unverified]

[Failure receipt](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/main-d30b9e0db-20261105/native-clippy-failure/receipt.json)
retains exact source equality, logs and terminal101. Seven later phases were
not reached in that fail-fast recipe. No lint exception, test skip or assertion
weakening was added. Separate checks of those not-reached phases may collect
additional outcomes, but cannot turn the failed clippy gate into a pass.

## Exit criteria and next action

Repair the intended exported trace path/dead-code contract and boolean style
through the responsible main workstream; do not suppress diagnostics locally.
Review actual source changes and rerun affected strict lint with unchanged
warnings-as-errors, then bind all other checks to the final source. Retain this
failure and any later failure/cap. Required exact-head CI/reviews/threads remain
mandatory before merge. Unresolved; no mathematical obstruction is inferred.
