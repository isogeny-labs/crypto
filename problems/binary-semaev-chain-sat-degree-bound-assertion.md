# Chain-SAT degree-bound assertion disagrees with supported summand range

- **Status:** open
- **Kind:** inherited software-validation assertion failure
- **Last updated:**2026-11-05
- **Affected work:** [PKM continuation](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/README.md)

## Problem and observed evidence

On sourceb022, `source_bound_covers_degree_83_without_constructing_a_model`
fails atline1105: `chain_s3_max_variables(83,10)` is Some190568, while the
assertion expects None. The implementation accepts2..=15 summands; subsequent
assertions explicitly expect m15 support and m16 rejection. This suggests an
outdated m10 assertion, but the chain-SAT owner must establish the intended
contract before changing it. No assertion has been changed or waived here.
The entire source file matches upstream maincc5addeba blob4e2111e29f64c5fece73f2c52dd9aff9c6e7f69f.

[Exact receipt and focused failure log](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/repaired-main-cc5addeba/final-20261105/receipt.json)
bind the actual generated binary. The focused test exits101,0passed/1failed;
the whole library execution later capped without a failure summary. This is
independent of the repaired delimiter and the unchangedJV runtime limit.

## Resolution criteria and next discriminating step

Confirm the intended m10/m15 allocation contract against callers and preflight
bounds; correct the implementation or stale assertion with a justified native
boundary regression. Run the unchanged focused contract and relevant chain-SAT
suite, then complete source-qualified full-library validation. Do not skip or
ignore the failing test. No cost or ECDLP conclusion follows from this failure.

Unresolved; current PKM changes do not modify this source. Recheck on an upstream
contract repair or a qualified maintainer diagnosis. No extra scientific run is
authorized by this problem record.

## Upstream repair integrated2026-11-05

Main d30b9e0db's test now expects Some190568 at m10 and retains m15/m16 plus
invalid-degree boundaries. Local mergebbf241b7e includes it without a local
assertion edit. The new example suites log26passes; the library test itself
still awaits its current-source execution. Status remains open/pending affected
contract qualification; old failure receipts remain unchanged. [pcc:unverified]

## Affected test observed2026-11-05

The current6168-case execution logs source_bound_covers_degree_83_without_constructing_a_model
ok on integrated main d30. This qualifies that test's observed repair only;
original exit criteria also require the broader chain-SAT and full-library
qualification, and the current suite capped with other failures. Preserve the
historical assertion failure; no local assertion edit. [Final receipt](../research/prime_ecdlp_three_leads_20261029/continuation-20261030/validation/main-d30b9e0db-20261105/library-final-20261105T1134Z/receipt.json).
Status remains partial qualification. [pcc:unverified]
