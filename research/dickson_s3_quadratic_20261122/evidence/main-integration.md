# Current-main publication integration

Tracking PR #4940 was independently merged at 2026-11-23T03:22:02Z, before
the substantive implementation was attached. Its head was
`dafe40421ca6dfd333ff822bb7f40f5466f9e738` and merge commit was
`9c3e560c13ae439695a8826625b72fb102437380`. Preserve that history. The actual
implementation is published in draft follow-up PR #4946 on
`lane/dickson-s3-quadratic-implementation-20261122`.

The integration combines qualified core
`69fad1271054225a95d5682f5fdcd11fd2a692d7` with main
`c49f8d009978ded324b6ddc404229af1c91a4730`. Git synthesized tree
`efda967c280e1273ab453cdbf23dcb9141435a66` without source conflicts.
The accompanying `main-integration-audit.json` describes this synthesized
tree before report and administrative additions and before the deliberate
whole-file pin update below. It is a same-team static byte comparison, not a
build, scientific execution, external review, or signed verification.

The merged `ic_boundary.rs` preserves main's TwTable changes and adds exactly
the eleven documented repairs. Its whole-file SHA-256 changes to
`ee70c40d611bea2a86e782e8513d6e18be49ca575e1159a42512afb18dc346ce`.
All nine selected ranges remain byte-identical to the qualified core; the
concatenation remains 86,691 bytes with SHA-256
`ae271ab93860613d321ba894c1d4a754520dc16e510778cd65561872b2ec8c26`.
The qualifier's `build.rs` deliberately updates only this whole-file pin.
The selection anchors, selection logic, and generated production computation
do not change. The new surrounding TwTable code is outside the qualifier and
is not qualified by this study. [pcc:unverified]

Across the synthesized tree, all 23 selected ranges in ten production files,
all four full F4 copies, both path-dependency source inventories, and all 43
whole-file manifest entries match the qualified core. After that audit, the
one build-script digest changes solely because of the explicit pin update.
No original source manifest, build result, run receipt, failed output, or claim
is rewritten to look like a later execution. The recorded tests remain bound
to the frozen core; the static audit explains the identical selected
computation in the integrated tree. [pcc:unverified]

Shared ACTIVE content is updated by replacing only this study's reservation.
Main's session log is preserved and this study's additions follow it. The
publication ledger uses current main as an exact byte prefix and adds the same
unchanged unsigned claim. `pcc-reconciliation.json` retains the original local
draft entry and both prior registration receipts remain unchanged. No published
ledger entry is rewritten.

The full root release gate remains open: the retained attempt timed out after
600 seconds before producing a test executable. No new full-root or comparative
scientific run is implied by this publication integration.
